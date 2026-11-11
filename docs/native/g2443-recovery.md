# Game 2443 (kult - the temple of flying saucers (1989)(exxos)(sp)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02443). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 27 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 542 of 542 named (0 author symbols), 542 of 542 described (40 by rule, 499 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4220 of 4220 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 80682 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10229 of 10229 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 540996 |
| code | 39294 |
| data | 258527 |
| padding | 5983 |
| unused | 56320 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:RUN | 2 | 117 | 2 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:KULT | 2 | 9245 | 495 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:RUNBACK | 3 | 740 | 42 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:ENDCLI | 2 | 117 | 2 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:bootblock | 1 | 10 | 1 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:PUZZL.BIN (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- kult - the temple of flying saucers (1989)(exxos)(sp)/Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf: file 4.BIN (header 18304 bytes, blocks hold 15128): its blocks are judged as raw blocks outside the filesystem
- kult - the temple of flying saucers (1989)(exxos)(sp)/Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:PUZZL.BIN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- kult - the temple of flying saucers (1989)(exxos)(sp)/Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.82)
- kult - the temple of flying saucers (1989)(exxos)(sp)/Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf: blocks 11-32 (bytes 5632-16896) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.25)
- kult - the temple of flying saucers (1989)(exxos)(sp)/Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf: blocks 38-43 (bytes 19456-22528) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.91)
- kult - the temple of flying saucers (1989)(exxos)(sp)/Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf: blocks 976-978 (bytes 499712-501248) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.96)
- 1 never-loaded region(s), 80682 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 2560 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:RUN

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves a longword from d1 to d0 and then branches to another routine.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:KULT

- `initialize_system_and_execute_main_loop`: Calls several initialization routines, sets a global pointer to a specific address, and enters a loop that conditionally executes a sequence of subroutines and library calls.
- `update_audio_state_and_global_logic`: Executes a sequence of routine calls and jumps, performs conditional checks on various global variables and structure members to update a global pointer and several status bytes, and repeats the entire process in a loop if a specific global word is non-zero.
- `check_global_self_assignment`: Checks if a global variable is not equal to itself and performs a routine call and assignment if so, repeating this for a second global variable.
- `update_opcode_pointer`: Combines two bytes from offsets 1 and 2 of a structure in a6 into a word with the high nibble set to 15, then stores this word into a global variable and branches to another routine.
- `store_stack_pointer`: Stores the value of a7 into a longword at a specific memory location.
- `dispatch_opcode_table`: Reads a word from a global variable and checks its high nibble to determine an action.
- `update_global_values_conditionally`: Calls several subroutines and conditionally updates several global variables or modifies a global variable based on a threshold check.
- `store_pointer_and_decrement`: Stores a 16-bit value from a pointer-based memory location into a register and then calls a routine that decrements a register.
- `increment_and_read`: Adds 2 to a6 and then reads a byte from the address at offset -1 relative to a6 into d0.
- `recurse_and_return_registers`: Saves several registers to the stack, loads a long value from a global variable into d1, calls a subroutine, restores the registers, and returns the contents of d0 and d1.
- `process_char_and_recurse`: Compares a byte at a pointer in d0 to 255, adds 48 to d0, and if it matches the byte at the pointer, updates a global variable and a structure in a5 with d0 and d1, then calls another routine and two subsequent routines if a condition is met; otherwise it restores a pointer from a global variable and returns.
- `traverse_and_update_global_state`: Compares a byte at a pointer in d0 to 255, adds 64 to d0, and if it matches the byte at the pointer, updates a global variable and a structure in a5 with d0, then calls another routine and two subsequent routines if a condition is met; otherwise it restores a pointer from a global variable and returns.
- `load_musiq_pointer`: Loads a pointer from a global variable into a5 and branches to another routine.
- `parse_dials_bin_structure`: Iterates through a data structure in a binary file, checking for a terminator character and calling another routine recursively while storing a pointer to the current element and a value from an offset of 10 into global variables.
- `search_string_for_terminator`: Iterates through a data structure by increments of 14 bytes, checking for a specific character terminator and calling another routine while passing a pointer to the current element and a value from an offset of 10.
- `clear_memory_range`: Zeros out a range of memory starting from a global variable pointer back to itself.
- `traverse_string_with_recursive_calls`: Iterates through a string until a terminator character is found, passing a pointer or a value from an offset or a constant to another routine recursively.
- `negate_and_load`: Negates the 16-bit value in d6 and loads a 16-bit value from a memory location into d0.
- `check_file_info_and_size`: Calls a library function to retrieve information, then checks if a specific value at an offset of a pointer matches a constant and if a calculated size difference meets a threshold.
- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `execute_conditional_dos_calls`: Calls a routine, and if the result is non-zero, it calls two other routines, potentially setting a return value to a maximum unsigned 16-bit value.
- `execute_conditional_dos_calls_2`: Stores three values into global variables, calls a routine, and then calls two other routines based on the result in d0 before returning a maximum value.
- `load_and_branch`: Loads the constant 500 into d2 and then branches to another routine.
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
- `copy_data_from_structure`: If a byte at a pointer in a6 is not equal to 135, it copies several values from a structure into global variables and calls two subroutines.
- `process_structure_data`: Compares a byte at an offset of a structure in a0 to a constant, and if not equal, loads two words from the structure, decrements them, calls another routine, and calls a third routine with a modified byte from the structure.
- `set_global_constants`: The routine assigns specific constant values to certain global variables and stores a pointer to a global variable before calling other routines.
- `process_bit_mask_loop`: Iteratively shifts a value left and updates a mask or bit-manipulation state using a decrementing counter or loop control variable.
- `calculate_pattern_count`: Calls a routine to scan for a pattern, adds the result to a counter, and if a pointer matches a global variable, modifies a global word and calls another routine.
- `decrement_counter_conditionally`: Calls another routine, then checks if d6 is zero; if not, decrements d6.
- `execute_bitmask_loop`: Clears register d0, calls another routine, and then iterates through a loop using a counter in d6 before returning.
- `increment_and_copy_byte`: Increments a pointer in a6, clears d4, loads a byte from the new pointer into d4, loads a constant into a5, calls another routine, and copies a word from one global variable to another.
- `process_buffer_pattern_and_update_table`: Calculates offsets and bitmasks based on input bytes in a structure, then iterates through memory ranges to apply bitwise modifications and pattern searches.
- `store_buffer_values`: Clears d4, increments a6, and calls a subroutine, then stores bytes from a6 into global variables and calls three other subroutines.
- `process_complex_buffer_logic`: Updates a global variable based on a comparison, then executes several loops that call other routines to perform bitwise operations and memory modifications using values from a table and various registers.
- `update_global_pointer_with_offset`: Iterates through a list of pointers, adding a pointer's value to itself, and finally stores a new calculated pointer into a global variable.
- `process_buffer_byte`: Compares register d4 to 4, otherwise it performs a rotate left on d4, moves a byte from offset 1 of the structure in a6 to d4, and increments a6.
- `traverse_buffer_with_jumps`: Updates a global variable with a value and then traverses a buffer, advancing a pointer by a value read from the buffer, until a certain condition is met.
- `tiny_lsl`: Small routine of 1 instruction that shifts d0 left by 2.
- `load_pointer_from_address`: Loads a 32-bit value from a specific memory address into register a0.
- `calculate_scaled_index`: Masks bits from a register, shifts the result, and combines it with a scaled value from another register into a single 32-bit return value.
- `set_a4_and_branch`: Sets a4 to a constant and branches to another routine.
- `initialize_pointer_and_call`: Sets a4 to zero, loads a pointer from a global variable into a1, and calls another routine.
- `fetch_two_bytes_and_call`: Calls another routine, subtracts register a4 from register a0, and then fetches two consecutive bytes from the memory pointed to by register a1 into registers d3 and d2.
- `store_shifted_word`: Clears register d5, moves a word from register d2 into it, shifts it left by one bit, and stores the result in a 32-bit location at offset 9ec2.
- `load_pointer_to_a3`: Loads a 32-bit value from a specific memory address into register a3.
- `tiny_subq_subq`: Small routine of 2 instructions that subtracts 1 from d2, then subtracts 1 from d3.
- `apply_bitmask_to_buffer_complex`: A nested loop routine that performs bitwise rotation and logical masking, updating a destination buffer and a secondary buffer offset by a large amount, while adjusting a pointer or counter.
- `load_and_shift_pointer`: Loads a pointer from a global variable into a1, calls another routine, clears d2 and d3, moves two bytes from the pointer in a1 into d3 and d2, calls another routine, clears d5, moves d2 to d5, shifts d5 left by 1, and stores d5 into a global variable before loading that global variable into a3.
- `permute_bit_patterns`: Performs bitwise masking, shifting, and logical ORing to rearrange specific bits within a 16-bit value while iterating through nested loops.
- `permute_bits_in_word`: Performs bitwise masking, shifting, and logical ORing to rearrange specific bits within a 16-bit value.
- `decrement_and_loop_counter`: Decrements a register or memory-based counter value and calls itself or another routine until the counter reaches a terminal value.
- `decrement_and_loop`: Decrements a 16-bit signed integer and repeats a routine call until the value reaches negative one.
- `modify_pointer_value`: Loads a 32-bit value from a specific memory address into register d7, shifts it left by one bit, and subtracts two from the result.
- `setup_and_apply_mask`: Pushes d7 to the stack, loads a pointer from a global variable into a6, calculates a value using a1 and a3, sets a global variable to 3, clears d4, d5, d6, and d7, performs arithmetic on d5 and d6, calls another routine, stores d7 into a global variable, loads a0 into a2, calls another routine, sets a global variable to 2, and restores d7 from the stack.
- `apply_bitmask_loop`: Performs a loop that executes a routine via a pointer-offset while updating a global variable using a bitwise mask.
- `set_global_word`: Sets register d5 to a 16-bit value of all ones and then branches to another routine.
- `update_table_and_call`: Stores a word in a table, pushes register d7 onto the stack, loads a pointer from a table into register a6, performs several arithmetic operations on registers d4, d5, d6, d7, d2, and d3, calls another routine, stores a word into a table, and restores register d7 from the stack.
- `apply_mask_to_buffer`: Performs a loop that executes a routine via a pointer-offset and updates a global index using a bitwise mask.
- `decrement_shifted_pointer`: Loads a 32-bit value from a specific memory address into register d7, shifts it left by one bit, and subtracts two from the result.
- `execute_masked_sequence`: Pushes d7 to the stack, loads a pointer from a global variable into a6, sets a global variable to 0, clears d4, d5, d6, and d7, performs arithmetic on d5, d6, and d2, calls another routine, stores d7 into a global variable, loads a0 into a2, calls another routine, sets a global variable to 1, and restores d7 from the stack.
- `execute_masked_index_loop`: Executes a loop that calls a routine via a pointer-offset and updates a global variable using a bitwise mask.
- `set_constant_and_branch`: Sets register d5 to negative one and branches to another instruction.
- `calculate_and_store_indices`: Performs arithmetic on several registers to calculate values, stores a word in a global variable, calls another routine, and stores a second word in a global variable.
- `decrement_and_loop_with_masking`: Decrements a local variable and a global variable, resetting the global variable to a certain value if it reaches zero, while calling a routine that applies a bit-rotated mask to a buffer and an infinite loop routine.
- `execute_nested_decrement_loops`: Executes a nested loop structure that repeatedly calls another routine, decrementing a register or a global variable and a local variable, while using an unchecked value or register as a loop counter.
- `set_constant_and_branch_2`: Sets register d4 to one and branches to another instruction.
- `decrement_and_loop_with_mask_params`: Decrements a local variable and a global variable while calling routines that merge or shift data and calculate mask parameters.
- `calculate_buffer_offset`: Checks a global byte, calls two routines, calculates an offset by shifting a value in d3, and calls a third routine using a pointer from a table plus that offset.
- `process_indexed_buffer_data`: Checks a global byte, calls two routines, loads two words from global variables, adds d3 to one, and calls two different routines using an offset calculated from d3 and the loaded words.
- `branch_if_not_zero`: Tests a byte at a global address and branches if it is non-zero; otherwise it calls a subroutine, shifts d0 left by two, saves a6 to the stack, calls another subroutine, restores a6, and returns.
- `calculate_and_call_subroutine`: Tests a byte at a global address and branches if it is non-zero; otherwise it calls a subroutine, saves a6 to the stack, adds d2 to d0, shifts d0 left by two, decrements d0, calls another subroutine, restores a6, and returns.
- `process_sequential_updates`: Checks a byte at a global address; if it is zero, it clears d0, loads a byte from offset 1 of a structure in a6 into d0, calls two routines, performs pointer arithmetic on a0 and a1, calls a third routine, and increments a6 by 2.
- `process_nested_updates`: Checks a byte at a global address; if it is zero, it clears d0, loads a byte from offset 1 of a structure in a6 into d0, calls a routine, performs two separate sequences of adding a word to d1, decrementing d1, and calling a routine, performs pointer arithmetic on a0 and a1, calls a third routine, and increments a6 by 2.
- `execute_conditional_subroutines`: Checks a byte in a global variable and, if it is zero, loads a byte from an offset of 1 relative to the stack pointer into d0, calls two subroutines, and returns; otherwise it branches to another routine.
- `test_byte_and_execute_subroutines`: Tests a byte in a global variable and, if zero, loads a byte from an offset of one relative to a6 into a 16-bit register, calls two subroutines, and returns.
- `update_table_with_values`: Searches a table for a zero value and then writes several register or local values into a table at a calculated offset.
- `branch_with_register_set`: Sets register d1 to two and branches to another location.
- `set_table_entry_zero`: Calculates an offset based on an input value and sets a value in a table to zero.
- `loop_through_table_entries`: Compares a word at a memory address with d0 and, if not equal, enters a loop that increments the address and decrements a counter.
- `decrement_and_loop_with_mask_params_2`: Decrements a local variable and calls a routine that calculates mask parameters and merges or shifts data.
- `decrement_and_loop_with_data_copy`: Decrements a local variable and calls a routine that calculates mask parameters and merges or shifts data, then performs a branch to another routine.
- `tiny_addq`: Small routine of 1 instruction that adds 1 to a6.
- `copy_table_blocks`: Copies large blocks of data from one memory area to another using a loop.
- `swap_table_elements`: Swaps or mirrors certain blocks of data between two memory areas using a loop.
- `clear_global_index`: Clears a global variable to zero and calls other routines.
- `execute_sequential_subroutines`: Loads a byte from an offset of 1 relative to the stack pointer into a 16-bit register, calls two subroutines, and returns.
- `increment_and_branch`: Increments the pointer in a6, clears d5, loads a byte from the address in a6 into d5, and branches to another instruction.
- `iterate_and_set_global_bit`: Checks a pointer and a bitmask, then enters a loop that sets a global variable to one while recursively calling another routine and decrementing a local variable if certain bits remain set.
- `retry_bit_rotation`: Calls a routine to rotate and invert bits, then loops if the result is zero, repeating this twice before returning the bitwise AND of the final result with 127.
- `rotate_bits_and_invert`: Performs a bitwise rotation and inversion on a value from the keyboard serial register and returns it.
- `increment_and_return`: Calls another routine and then increments the value in a6 before returning.
- `update_status_and_call_subroutines`: Clears a byte in a global variable, updates it based on a bitwise AND with a byte from another global variable, calls three subroutines, and conditionally sets a byte in a global variable to 1 and calls a fourth subroutine before returning.
- `compare_and_call`: Compares a byte at an offset of a6 with a constant, branches if equal, otherwise it calls another routine, sets d0 and d1 to specific values, sets d4 to either sixteen or seventeen, and calls another routine before returning.
- `clear_and_recursively_traverse_table`: Clears a global variable and enters a loop that recursively calls itself while checking a boolean flag and processing data passed via registers.
- `search_and_clear_table_entry`: Iterates through a table looking for a matching value and sets that table entry to zero if found.
- `clear_table_entry`: Performs a search in a table to clear an entry if a certain value matches.
- `copy_data_blocks`: Copies a block of words from a source table to a destination buffer using a loop.
- `merge_and_shift_data`: Merges bits from a source buffer into a destination buffer and shifts pointer or data pointers.
- `apply_bit_mask_to_buffer`: Applies a bitwise mask or its inverse to a buffer while copying or updating words from a source buffer.
- `load_mask_and_pointers`: Copies three words from a global table to another global table and loads two pointers into registers a0 and a1.
- `copy_and_mask_buffer`: Copies words from a source buffer to a destination buffer while applying a bitmask and updating a pointer or offset.
- `execute_infinite_loop`: The routine executes an infinite loop without performing any further operations or modifying any registers or memory.
- `load_pointer_a1`: Loads a pointer from a global variable into register a1.
- `extract_buffer_elements`: Clears d2 and d3, then reads two bytes, a longword, a word, and a longword from a source buffer into d2, d3, a0, d7, and a3 respectively.
- `apply_bitmask_and_rotate_to_buffer`: Iterates through a source buffer, applying a bitmask-based bit-rotation or bit-shifting (using a rotating bit-mask pattern) and writes the resulting bit-manipulated or bit-aligned words into a destination buffer.
- `load_pointer_from_address_2`: Loads the longword located at a specific constant address into a1.
- `apply_bit_rotation_to_buffer`: Iterates through a source buffer and writes bit-rotated or bit-shifted versions of specific bit-fields or words into a destination buffer.
- `permute_bits_in_buffer`: Performs bitwise masking and bit-rotation-like operations on a buffer using a calculated bitmask.
- `calculate_mask_parameters`: Calculates bitmask values and updates a global variable used for masking or bit-width calculation.
- `initialize_and_rotate_buffer`: Zeros out a buffer range and then applies a bit-rotated mask to a source buffer, writing the results to a destination buffer.
- `apply_bit_rotation_to_buffer_2`: Iterates through a buffer to apply bitwise masking and bit-rotation-like operations to specific memory offsets.
- `initialize_buffer_and_mask_2`: Clears a long value in a0 and calls two other routines before returning.
- `apply_rotated_bitmask`: Calculates a bit-rotated mask and uses it to bitwise-AND and OR values into a table, or potentially a buffer, using a loop controlled by a global counter.
- `apply_mask_to_offset_buffer`: Applies a bit-rotated mask to a buffer using bitwise operations, where the source buffer is an offset version of the destination buffer.
- `setup_buffer_pointers`: Calls two other routines and then returns.
- `initialize_buffer_and_mask`: Fills a buffer with zeros and updates global variables used for masking or bit-width calculation.
- `search_data_table`: Iterates through a table of 6-byte entries, comparing a byte from a global variable with a value in the entry, and if a match occurs, updates two global variables with values from the entry and returns a byte from the entry.
- `extract_and_store_buffer_data`: Extracts or constructs a 32-bit value from a buffer, then stores parts of that value and other pointer-based bytes into global variables and a table.
- `decrement_register_loop`: Decrements a value in a register until it reaches a certain limit.
- `populate_buffer_from_indexed_data`: Calculates an offset into a data table, retrieves values, and performs a series of nested loops to write these values and others into a target buffer.
- `load_and_call_routines`: Loads a constant into a register and calls two subsequent routines.
- `process_buffer_with_bitmasks`: Iterates through a buffer, performing bitwise rotations, masking, and multiplication to update values in a destination buffer based on bytes read from the source buffer and a global pointer table.
- `iterate_pointer_list_and_set_constants`: Iterates through a list of pointers, checking the high bit of each pointer's value and advancing the pointer by a fixed offset, then writes a sequence of constant values to certain memory locations.
- `apply_bitmask_to_buffer`: Iterates through a buffer, performing bitwise OR, bitwise AND, and bit-rotation or bit-swapping operations before storing the result back to the buffer.
- `mask_and_store_bytes`: Applies a bitmask or its inverse to bytes in a buffer and stores them back into a buffer located at a calculated offset.
- `manipulate_bits_via_rotations`: Performs a series of bitwise rotations and logical AND operations on registers d1 and d7, storing the final result in d6.
- `initialize_buffer_values`: Sets a global variable to 9 or 102 based on a comparison, then performs two sequences of saving registers to memory locations and calling subroutines.
- `update_global_pointer_constants`: Sets a global variable to a constant value based on its current state and then updates a pair of global variables with a new pointer value by calling several subroutines.
- `set_global_variable`: Sets a global variable to 24 or 30 based on a comparison, then calls two subroutines that copy words from memory to a global variable and a table.
- `cycle_state_and_update_buffers`: Increments a state byte in a global variable and branches to different routines based on its value.
- `process_buffer_rotations`: Calls two subroutines to perform bitwise operations on data buffers using specific counts and offsets, then branches to another routine.
- `execute_buffer_operations`: Calls four subroutines with specific values in d0, d1, and d2 to process data, then branches to another routine.
- `set_table_and_constants`: Checks a byte in a global variable and, if it is zero, loads a byte from an offset of 1 relative to the stack pointer into d0, calls a subroutine, writes a byte to a global variable, and updates a global word before returning; otherwise it branches to another routine.
- `update_buffer_with_scaled_index`: Calculates scaled offsets and increments a counter to check against a limit, while updating a global variable.
- `calculate_and_update_counters`: Subtracts four bytes loaded from memory from each other, writes a constant to a global variable, and then enters a loop that calls another routine.
- `update_table_with_rotated_mask`: Uses a bit-rotated mask to perform bitwise AND and OR operations on specific offsets within a table.
- `update_buffer_parameters`: Tests a byte at a global address and branches if non-zero; otherwise it calls a subroutine, shifts d0 left by two, and stores the result in a global variable, shifts d2 left by two and stores it in another global variable, calculates the product of d2 and d3 and stores it in a third global variable, then enters a loop that calls two subroutines and performs arithmetic on global variables until a counter reaches zero.
- `apply_bitmask_to_buffers`: Calculates a bitmask using a bit-rotation-like operation and applies it to several distant memory offsets via a pointer.
- `branch_to_subroutine`: Performs a short branch to another routine.
- `store_and_branch`: Stores the constant value 32000 into a global variable and branches to another routine.
- `modify_buffer_bits`: Iterates through a buffer and modifies specific bits in a series of words based on a bitmask check.
- `modify_bit_patterns_2`: Saves d0-d3 to the stack, swaps d6 and d7, sets a0 to zero, calls a subroutine, sets d0 to d7, adds two global pointers to a0, calculates a bit index using d1 and d3, and then tests a bit in a value from a global address to conditionally set a bit in d0.
- `toggle_bit_and_store_3`: Clears a bit in d0 and stores the result in a memory location at offset 0 of a0, then conditionally sets a bit in d0 based on a bit test of a longword at offset 4992 of a1.
- `toggle_bit_and_store_2`: Clears a bit in d0 and stores the result in a memory location at offset 7520 of a0, then performs conditional bit setting based on a bit test of a value at offset 15600 of a0.
- `toggle_bit_and_store`: Clears a bit in d0, stores the result in a global variable, loads two words from global variables into d0 and d2, checks a bit in d2, and either branches to another routine or sets a bit in d0 and branches to another routine.
- `clear_bit_and_loop`: Clears a bit in d0, stores d0 in a memory location, and enters a loop that decrements d1 and repeats based on the value in d3 before branching to another routine.
- `modify_bit_patterns`: Iterates through a pointer and modifies specific bits in a destination pointer, a second pointer, a third pointer, and a fourth pointer based on a bitmask.
- `write_long_and_branch`: Writes the longword value 32000 to a global variable and performs a short branch to another routine.
- `update_bit_flags`: Iterates through a memory range and updates specific bits in several remote memory locations using a bitmask and a counter-based loop.
- `branch_to_address`: Branches to a specific address.
- `load_and_call_subroutines`: Tests a byte at a global address and branches if non-zero; otherwise it calls two subroutines, loads four words from global variables into d0-d3, saves them and a0 to the stack, calls a subroutine, restores the registers, loads a pointer from a global variable, and calls another subroutine before returning.
- `process_parameter_block_and_dispatch`: Checks a global byte for a non-zero value to decide whether to call two subroutines or skip them, then loads four consecutive words from a memory location into registers, calls a subroutine with those values preserved on the stack, and finally calls another routine using a pointer retrieved from a global table.
- `branch_to_subroutine_2`: Performs a short branch to another routine.
- `execute_sequential_subroutines_short`: If a byte at a specific address is not zero, it clears d0, moves a byte from offset 1 of a6 into d0, calls a subroutine, stores d3 in a global variable, calls another subroutine, and increments a6.
- `execute_sequential_subroutines_long`: If a byte at a specific address is not zero, it clears d0, moves a byte from offset 1 of a6 into d0, calls a subroutine, stores d3 and d2 into two consecutive global variables, calls another subroutine, and increments a6.
- `call_two_subroutines_2`: Calls two subroutines and then returns.
- `copy_memory_block`: Performs a nested loop to copy data from a source pointer to a destination pointer with a fixed stride.
- `get_unaligned_word`: Reads a 16-bit value from a pointer offset by two bytes.
- `update_pointer_and_store_value`: Reads a 16-bit value from a pointer, then updates a global pointer and a global variable with a new address.
- `advance_pointer_with_backup`: Stores the value at a global pointer into another global variable before incrementing that pointer by two bytes.
- `store_constant_to_global`: Stores the constant value 40320 into a global variable.
- `set_global_bounds`: Sets a global variable to a specific value and another global variable to a constant integer.
- `check_register_equality`: Compares the byte at the address stored in a global variable with the contents of d0, then returns if they are not equal.
- `register_interrupt_handler`: Adds a new interrupt handler to the exec interrupt vector table and then calls the previous handler, subsequently resetting a global variable to a constant value.
- `call_library_functions_sequentially`: Calls a function from a library three times using different addresses, storing the returned pointer in a global variable each time if the result is non-zero.
- `allocate_and_initialize_buffers`: Calls several functions via pointers retrieved from a global variable to allocate memory and initialize a series of global pointers and counters with specific offsets and values.
- `execute_task_related_functions`: Calls three different functions via pointers retrieved from a global variable and its offsets, involving task and port management.
- `update_mouse_and_pointers`: Calls a routine via a pointer in a structure, updates several global pointers with offsets, and stores bytes from the mouse hardware registers into a table.
- `update_buffer_pointers`: Updates certain global variables using values derived from a pointer and a source buffer, while storing a byte and a word from that buffer into a destination pointer.
- `unpack_and_rotate_bitstream_2`: Tests a byte at a global address and branches if non-zero; otherwise it calls a subroutine, saves d0-d7 and a0-a1 to the stack, calculates several values based on d2, d3, and d4, stores them in global variables, and enters a loop that calls a subroutine and performs arithmetic on global variables.
- `update_pointer_and_loop`: Swaps the bytes in d0, stores the result in a global variable, and then enters a loop that performs arithmetic on d0 using values from a table and updates a global variable.
- `execute_infinite_loop_3`: Swaps the bytes in d0, stores the result in a global variable, and enters a loop that modifies a global variable via subtraction while checking a condition or calling another routine.
- `copy_table_data`: Checks a global byte, calls two routines, and then iterates through a table to compare and copy bytes at specific offsets between two memory locations, finally updating a value in the table.
- `copy_bytes_to_buffers`: Iterates through a table to copy bytes into four different memory locations at specific offsets, then checks a global variable and calls another routine.
- `populate_table_from_source`: Iteratively copies bytes or related data from a source pointer to a table while calculating offsets and checking for zero values.
- `unpack_and_rotate_bitstream`: Calculates division and modulo values for two counters, then enters a nested loop to extract bitfields from a source table using bitwise rotation and shifts to populate a destination buffer.
- `rotate_and_bit_extract_data`: Performs bitwise rotation on two 32-bit values using a shift amount from a register, then extracts specific bit fields from a table into a destination buffer via a loop.
- `setup_data_block_copy`: Loads a source pointer from the stack, sets the destination address in a0, and calculates a word count from a global variable before calling copy_data_blocks.
- `reorder_and_bit_extract_data`: Calculates division and remainder values for two sets of parameters, then enters a nested loop to extract bits from a source table into four separate 16-bit destination buffers.
- `manipulate_and_shift_bitfield_data`: Extracts two 16-bit values from a structure using a4, shifts them left by the value in d4, and reorders them within the structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_and_apply_bit_mask`: Calculates two 16-bit differences from global values, uses them to determine a bit mask and a loop count, and then enters a loop that applies bitwise operations to four bytes at specific offsets within a destination buffer using a rotated 32-bit mask.
- `branch_on_byte_comparison`: Compares a byte at a pointer in a5 with 1, branches if equal, otherwise it rotates d3 left by 1, branches based on the carry flag, increments a0 and a1, and branches to another location.
- `merge_bit_masks`: Rotates a 32-bit value in d3, then performs bitwise AND and OR operations on four specific byte offsets of a destination buffer using a mask derived from d3 and values from a source buffer.
- `subtract_pointers_and_branch`: Subtracts 40 from a0 and a1, adds d4 to d7, and branches to another location.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_buffer_range_bounds`: Calculates and writes new range boundaries into global variables and executes a loop that modifies these bounds or pointers.
- `update_global_counters`: Calculates and stores new values in several global variables and executes a loop that decrements a value while calling another routine.
- `copy_data_blocks_2`: Loads a byte from a global variable, increments a pointer, and then calls two other routines with specific values in d1.
- `clear_registers_and_increment_pointer`: Clears registers d0 and d1, loads a byte from an offset of a6 into d1, and increments a6 by two.
- `set_pointer_and_loop`: Stores a word into a global variable, then executes a loop that increments a pointer by three until it reaches a previously stored pointer value.
- `scan_and_update_bounds`: Calls a subroutine, copies a byte from a global address to d1 and stores it in another global variable, saves a6 to the stack, iterates through a list of values checking if bytes fall within a specific range, updates several global variables with maximum values or bounds-related constants, and calls several subroutines before returning.
- `process_bit_reordering`: Rearranges bits and checks certain character flags before recursively calling another routine with a calculated word value.
- `increment_and_call_two`: Increments a pointer in a6, calls two different routines, and then increments a6 by three before returning.
- `increment_and_call_one`: Increments a pointer in a6, calls one routine, and then increments a6 by three before returning.
- `load_byte_and_increment_pointer`: Loads a byte from an offset of a6 into d0 and increments a6 by two.
- `update_pointer_metadata`: Calls a routine to populate a pointer table, then executes a loop that decrements a value at an offset in a structure and updates a pointer-based address by adding a scaled value from another offset in that structure.
- `load_byte_and_increment_pointer_2`: Loads a byte from an offset of a6 into d0 and increments a6 by two.
- `modify_pointer_table_values`: Calculates a new value based on a pointer, updates a global variable, and then executes a loop that modifies a value in a table.
- `populate_pointer_table`: Iterates through a list of pointers to calculate and store new pointer values and associated metadata into a table.
- `execute_loop_with_calls`: Loads a value from a pointer and executes a loop a fixed number of times while calling other routines.
- `store_byte_via_callback`: Moves a byte from a global address into d6, calls another routine, and then stores d6 back into that same global address.
- `store_conditional_bytes`: Checks a byte at a global address and, if certain conditions are met, fills a destination buffer with one of three specific byte values using a loop.
- `unpack_and_store_data`: Reads bytes from a source pointer and selectively stores certain bytes into a destination buffer based on bitmask checks.
- `branch_to_subroutine_3`: Calls a subroutine and then branches to another location.
- `process_bitstream_metadata`: Clears a global byte, checks a bitmask in a global variable, and if valid, iterates through a table to either call a subroutine via a pointer or update a counter.
- `traverse_table_and_clear`: Pushes a pointer to the stack, calls three subroutines, clears a global byte, and if a global variable matches 22, performs several more subroutines and sets a global byte to 1.
- `dispatch_on_char`: Performs unconditional calls to another routine if a global variable matches any of several specific character constants.
- `shift_global_values`: Loads two words from a global variable into d0 and d1, then performs a logical shift right on d0 by two bits.
- `check_input_range`: Checks if two input bytes fall within a range defined by a table.
- `set_registers_and_branch`: Loads a constant into a4, a constant into d2, and branches to another location.
- `search_for_valid_index`: Increments a value and checks it against a range, potentially setting a global or updating a pointer if a valid range or value is found.
- `invoke_subroutines`: Loads two bytes from memory, calls one routine, and then checks several conditions based on a global word and a bitmask before calling another routine.
- `find_matching_entry`: Iterates through a list of entries to find one where a byte at an offset of 4 matches a specific value, then updates several globals and pointers.
- `search_for_matching_pattern`: Iterates through a memory list to find an entry where a certain masked bit pattern matches a changing global value.
- `update_pointer_from_list`: Iterates through a list of entries using two different range checks, and if successful, updates a global variable and a pointer offset.
- `load_pointer_and_calculate_offset`: Loads a value from a pointer-based offset into a global, updates a global with a byte from a pointer, and calculates a new pointer offset based on a value in memory.
- `update_state_and_registers`: Checks if a byte at a global address is less than 63, then performs several conditional checks involving global words and bytes to call another routine, clear a byte in a structure pointed to by a6, and update a global byte with a value derived from d0; otherwise it returns.
- `tiny_move`: Small routine of 1 instruction that copies 150 to d1.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_two_subroutines`: Calls two different subroutines and then returns.
- `load_calculated_addresses`: Calculates an offset based on an input, then loads values from that offset and a fixed address into global variables.
- `execute_if_value_changed`: Checks if a global value differs from a previously stored value and calls another routine if a change is detected.
- `process_opcode_logic`: Writes a word to a global variable, calls another routine, and if a global word is non-zero, checks a byte from a structure to determine whether to update a structure pointed to by a1 and a2, write a byte to a structure, or write a word to a global variable and call another routine.
- `write_word_and_branch`: Writes the word 49596 to a global variable and branches to another routine.
- `write_word_and_branch_2`: Writes the word 49599 to a global variable and branches to another routine.
- `update_state_and_branch_conditionally`: Writes bytes to offsets of a1 and a2, moves a byte from a global variable to an offset of a0, writes a word to a global variable, and then branches to different locations based on comparisons of bytes at offsets of a1 and a2.
- `update_table_values`: Iterates through a table, checking if a byte equals a certain value and replacing it with a different value.
- `validate_and_update_entity_state`: Checks if a byte at offset 4 of a structure matches specific character constants, then compares a byte at offset 3 of another structure against the value at offset 3 of the first.
- `update_global_pointer_and_branch`: Writes the word 49607 to a longword at a global variable address and branches to another routine.
- `dispatch_conditional_updates`: Clears a byte in a global variable, calls another routine, and if the byte is non-zero, updates a structure pointed to by a1 and a2 with specific bytes and a value from a6, then calls another routine if certain byte comparisons match; otherwise it returns.
- `decrement_and_jump`: Decrements a byte at a global address and branches to another location.
- `initialize_audio_state_2`: Sets a byte in a global variable, sets d2 and d3 to 32768, writes 255 to a global word, calls another routine, and branches to another routine.
- `initialize_audio_state_alt`: Sets a byte in a global variable, loads a longword from a global address into d3, sets d2 to 63, writes 15 to a global word, calls another routine, and branches to another routine.
- `initialize_global_state`: Clears a word in a global variable, calls two subroutines with specific constants, calls another subroutine, and if a global word is non-zero, enters a loop that calls several subroutines and performs bit tests and assignments before finally checking a character at an offset from a pointer-based address to set a global byte to zero or one.
- `update_state_and_dispatch_event`: Clears a 16-bit value in a global variable, calls three subroutines with specific parameters, and updates a byte in a global variable by adding 110 to it if it is not 100.
- `fetch_opcode_from_table`: Calculates an index by multiplying a byte from a global variable by ten and adding a shifted byte from a6, then fetches a word from a table at a0 using this index and stores it in a global variable, or alternatively fetches a word from a different table at a0 plus the shifted byte and stores it in the same global variable.
- `compare_and_mask_byte`: Copies a byte from a global variable to d2, compares d0 to d2, and if d0 is not less than or equal to d2, it masks d2 with 240, subtracts d0 from d2, writes d2 to a global variable, masks d2 with 240, and if the result is not equal to the masked d1, it returns.
- `invoke_instruction_stream`: Calls a routine that processes a sequence of instructions from a memory buffer.
- `set_global_and_call`: Sets d0 to 2, calls two routines, and writes the value 255 to a global variable.
- `process_opcode_table_and_dispatch`: Loads a value from a global variable to use as an index into a table, then writes the resulting word to another global variable.
- `copy_opcode_to_global`: Copies a word from a global variable to another global variable and then branches to another routine.
- `validate_opcode_range`: Compares a byte from a global variable against three different values and, if it matches specific criteria, writes a word to a global variable and branches to another routine; otherwise it branches to another routine.
- `resolve_and_dispatch_opcode`: Checks a global variable to select an entry from a table using a calculated index, then stores the resulting word into another global variable.
- `dispatch_opcode_table_2`: Copies a word from one global variable to another and branches to a new location.
- `process_command_state_and_dispatch`: Updates a global state variable, modifies a bit in a structure, and uses a word from a table to either execute a subroutine via a pointer or perform a series of arithmetic operations and updates to global variables.
- `clear_and_set_opcode`: Copies a word from a global variable to another global variable, clears a byte in a global variable, and then branches to another routine.
- `write_global_constants`: Writes the values one and two into two different global variables and returns.
- `set_constant_opcode`: Writes a constant word to a global variable and then branches to another routine.
- `clear_and_mask_table_elements`: Iterates through a table of 40-byte-aligned entries, clearing the high bit of the second word in each entry.
- `process_data_table_and_dispatch`: Copies words from a source table to a destination table using two nested loops, then iterates through a list of structures to check if a byte at offset 4 matches a specific bitmask and compares a byte at offset 2 against a value, setting a bit in the structure if both conditions are met.
- `process_data_and_branch`: Sets a bit in a byte at offset 0 of a structure in a0, calls a subroutine, clears d0 and d2, reads bytes from a0 and offset 2 of a0 into d0 and d2, calls another subroutine with d1 set to 26, calls a third subroutine, and branches back to an earlier instruction.
- `update_command_state_and_dispatch`: Updates a global command word by either loading a value from a table based on a calculated index or using a fallback value, then dispatches the command by checking its high nibble to either call a subroutine from a table or execute a direct pointer.
- `update_opcode_state`: Copies a word from one global variable to another and then branches to a different routine.
- `update_event_state_and_dispatch`: Updates a global state variable by fetching a word from a table based on an index, then checks the high nibble of that word to determine the next action.
- `update_opcode_state_2`: Copies a word from one global variable to another and branches to a different routine.
- `validate_and_dispatch_command`: Validates a command byte against specific range constraints and then dispatches a command by calling various subroutines based on the resulting value.
- `move_d4_to_global`: Copies the word in d4 to a global variable and then branches to another routine.
- `iterate_and_store_data`: Iterates through a table, checks a bitmask against a value, and stores certain bytes and pointers into a global location.
- `apply_mask_to_pointer_table`: Iterates through a nested table of pointers and applies a bitwise mask or bit-manipulated value to the memory pointed to by those pointers.
- `execute_scripted_opcode_sequence`: Increments a byte and a word in global variables, then performs a sequence of operations by loading values from a global variable into a pointer and calling another routine.
- `write_opcode_to_global_and_branch`: Writes a 16-bit constant to a global variable and branches to another routine.
- `execute_opcode_sequence`: Calls two subroutines, and if a condition is met, writes the word 37034 to a longword at a global variable before branching.
- `write_opcode_and_branch`: Writes the word 37034 to a longword at a global variable and branches to another routine.
- `write_opcode_and_branch_2`: Writes the word 50013 to a longword at a global variable and branches to another routine.
- `execute_conditional_sequence`: Calls four different routines and performs a conditional branch; if the condition is met, it writes the word 37034 to a longword at a global variable and branches to another routine, otherwise it returns.
- `initialize_and_calculate_parameters`: Sets three global variables to specific constant values, clears a longword, and performs a series of divisions and arithmetic operations on a value retrieved from a global variable to update several other global variables through multiple subroutine calls.
- `update_global_state_and_pointer_table`: Updates a global variable using a clamped or masked value and applies a bitmask-driven pointer-based update to a table of global variables.
- `process_opcode_sequence`: Saves a pointer to a global variable, increments a counter, and enters a loop that processes a sequence of commands.
- `branch_to_scatter_routine`: Loads a pointer from a global variable, sets a byte in another global variable, and branches to another location.
- `scatter_data_to_buffers`: Iterates through a loop, calling a routine that copies or scatters data into a destination, and then executes a secondary loop.
- `scatter_table_data`: Copies bytes or words from a source table to a non-contiguous destination pattern.
- `clear_and_load_registers`: Clears registers d1, d0, and d2, then loads two bytes from a location offset from a global variable into d0 and d2.
- `execute_command_table`: Clears two words in a global structure, then enters a loop that reads a byte from a table and either calls a subroutine via a pointer or performs specific data movements.
- `calculate_offset_address`: Calculates a new address or offset based on specific bits in a register and stores it in a global variable.
- `fetch_and_call_offset`: Loads a byte from a pointer in a6 into d0, calls another routine, clears d0, and then loads another byte from a6 into d0.
- `clear_and_process_buffer`: Clears a block of memory to zero and then iterates through a buffer to perform bitwise operations and store results.
- `write_constants_and_branch`: Writes a 16-bit constant and a byte to specific global memory locations and then branches to another routine.
- `copy_word_and_branch`: Copies a 16-bit word from a global variable to another global variable and then branches to another routine.
- `set_word_and_branch`: Writes a 16-bit constant to a global variable and then branches to another routine.
- `update_pointer_from_hardware`: Decrements a word from a hardware register by one, stores the result in a global pointer, and branches to another routine.
- `decode_bitstream_loop`: Iterates through a buffer, extracting bitfields to update a global variable and a secondary global variable, while using nested loops to process or terminate based on bit-shifted values.
- `traverse_string_with_recursive_calls_2`: Iterates through a string until a terminator character is found, passing a pointer or a value from an offset or a constant to another routine.
- `load_and_process_four_words`: Loads four 16-bit values from consecutive global memory locations into registers, calls two other routines, and then clears a global 16-bit variable.
- `copy_and_update_buffer_range`: Performs a block memory copy or scatter-gather-style write where data from a source pointer is written to a destination pointer using a nested loop, followed by updating a set of global boundary or limit variables.
- `fetch_offset_byte`: Retrieves a byte from a calculated memory address based on bit-masked flags and an offset or pointer lookup.
- `evaluate_register_conditions`: Checks a bitmask in a register and performs several conditional comparisons between register values before returning.
- `execute_conditional_calls`: Calls two routines and then branches to another routine if a byte from a pointer in a6 has bit 7 set, otherwise it calls two routines and branches to a third.
- `store_byte_conditionally`: Increments a pointer in a6, calls two routines, and then checks a bit in d0 to decide whether to store a byte or a word from d6 into the memory location pointed to by a0.
- `conditional_unaligned_read`: Increments a pointer in a6, calls a routine, and if d6 is zero, it decrements a6, branches to another routine, or otherwise increments a6 by two and returns.
- `process_button_input`: Checks certain bits in a register and updates a global variable based on the state of those bits.
- `update_mouse_position_and_transform_table`: Updates a global coordinate based on mouse port movement, clamping the values within a specific range, and then performs a bitwise logical transformation on a block of memory using a temporary value and updates a table or global pointer.
- `increment_and_dereference_global`: Increments a global variable and then replaces its value with the value stored at that new offset in a table.
- `increment_and_load_pointer`: Increments a global variable and returns the value stored at the address or offset pointed to by that incremented value.
- `update_global_constants`: Calls three subroutines, writes 31447 to a global variable, calls another subroutine, writes 0 to the same global variable, and calls a final subroutine.
- `update_table_values_2`: Subtracts two 16-bit values from global variables, and if the result is at least 180, updates one global variable with the other and enters a loop that clears bit 5 of a byte in a table every 6 bytes for 62 iterations.
- `call_copy_words_loop`: Loads a byte from an offset of 1 in a6 into d0, increments a6 by 2, and calls another routine using the value in d0 as an index into a table of function pointers.
- `tiny_addq_2`: Small routine of 1 instruction that adds 1 to a6.
- `load_byte_from_address`: Clears d0 and moves a byte from the address stored in a global variable into d0.
- `copy_words_in_loop`: Performs a loop that copies words from a source table to a destination table, where the number of iterations is determined by multiplying a value in d0 by ten.
- `load_address_into_a1`: Loads the address stored in a global variable into a1.
- `call_load_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls LoadRGB4 and returns its result to the caller.
- `increment_nibble_table_values`: Copies sixteen words from a source table to a destination table, then iterates through four words in a table to increment their nibbles, provided the value is not a specific maximum constant.
- `sync_and_modify_table`: Copies a block of words from a source table to a destination table, then enters a loop that compares values from a source table with a destination table and modifies the source values by adjusting their nibbles if they do not match.
- `clear_buffer_and_check_table`: Zeros out a 16-byte buffer at a global address using a loop, then iterates through a table of pointers, calling another routine if bit 6 of the byte at offset 4 of the current pointer is set.
- `rotate_and_store_buffer`: Stores the value in a1 at the address in a2, then calls another routine, calls a second routine, and returns.
- `compare_byte_and_branch`: Compares a byte from a global variable against 22 and 25, branching to another location if the value is within that range.
- `check_byte_range`: Clears d1, loads a byte from a global variable into d1, and branches to another routine if the byte is not between 22 and 25 inclusive.
- `increment_a6`: Adds one to the value in a6.
- `execute_recursive_loop`: Executes a routine that calls itself recursively while decrementing a register, repeating a fixed number of times.
- `check_pointer_and_recurse`: Checks if a pointer in a2 is non-zero, and if so, extracts several bytes from the pointed-to structure to set up registers before calling another routine.
- `increment_a6_alternative`: Adds one to the value in a6.
- `traverse_and_process_buffer`: Iterates through a sequence of 15 memory locations, calling another routine if the value at the current location is non-zero.
- `push_and_call_subroutine`: Pushes a6 onto the stack, calls a subroutine, restores a6 from the stack, increments a6, and returns.
- `search_and_update_buffer_2`: Iterates through a table to check for a bitmask match, then updates a byte in the table and a byte in another memory area.
- `set_address_and_branch`: Sets a0 to zero and branches to another routine.
- `search_and_update_buffer`: Iterates through a table to check if a byte matches a masked value, then increments a byte at a calculated destination address and updates a global variable.
- `compare_and_branch_2`: Compares a 16-bit value from a global variable with another 16-bit value from a global variable and branches if the first is less than the second.
- `scan_buffer_for_pattern`: Iterates through a buffer and checks if certain bits in a byte match a specific pattern, then recursively calls another routine and updates a pointer.
- `transfer_and_call`: Clears register d0, moves a byte from an offset of a6 into d0, calls another routine, and then moves bytes from offsets of a6 into d2 and d3 before incrementing a6.
- `update_global_status_bytes`: Stores two bytes from registers into a memory location, clears a global byte, and then reloads those bytes from memory before calling several subroutines.
- `update_global_state_logic`: Loads a constant into d0, calls another routine, loads two bytes into d2 and d3, calls another routine, and finally loads a pointer from a global variable into a0.
- `write_configuration_values`: Writes three longwords to specific global memory locations, calls another routine, writes two bytes to specific global memory locations, and branches.
- `execute_recursive_call`: Increments a pointer in a6, moves a byte from the structure at the new pointer into d4, calls another routine, and then calls a second routine before returning.
- `process_buffer_and_call_routines`: Sets two registers, calls a routine, increments a pointer, and then calls four consecutive routines before incrementing the pointer again and returning.
- `modify_byte_and_recursively_call`: Increments a pointer in a6, moves a byte from the structure into d4, calls another routine, performs a bitwise AND on a byte at offset 3 of the structure in a6, and then performs three subsequent calls to other routines.
- `modify_byte_and_recurse`: Copies a byte from one location to another and clears a bit in a byte at an offset from a pointer.
- `check_and_modify_memory`: Compares a byte at the address in a0 with 35, and if it is not equal, writes a byte from an offset of 1 in a0 to an offset of 3 in a6.
- `modify_memory_with_offsets`: Reads a byte from a pointer and writes modified or offset values to adjacent memory locations relative to a register-based address.
- `load_address_and_branch`: Loads the constant value 42540 into a1 and branches to another routine.
- `load_address_and_branch_alt`: Loads the constant value 42500 into a1 and branches to another routine.
- `load_address_and_branch_final`: Loads the constant value 42460 into a1 and branches to another routine.
- `resolve_opcode_from_data`: Checks two global flags and, if both are set, performs a series of subtractions and a bit shift on a byte read from an offset of two in a pointer to calculate an index.
- `loop_with_decrementing_counter`: Sets a byte at a global variable to 1, pushes a6 to the stack, clears d0, calls another routine, loads a pointer from a global variable into a3, loads a pointer from a global variable into a2, and copies a longword from a3 to a2.
- `branch_and_save_registers`: Saves registers a0 through a6 and d0 through d7 to the stack, calls a subroutine, and then branches to another location.
- `execute_nested_loop`: Executes a nested loop structure that decrements registers, effectively performing a fixed number of empty or secondary routine calls.
- `update_masked_buffer`: Performs a bitwise or-not and and-with operation while copying or updating values in a large buffer using a nested loop structure.
- `copy_buffer_segments`: Copies data from a source buffer to a destination buffer using a nested loop structure.
- `update_buffer_with_scaled_index_2`: Performs bitwise operations on a destination buffer using values from a source table within a nested loop structure, then updates several global variables and branches to another routine.
- `conditional_loop_on_byte`: Checks if a byte at a pointer is non-zero and then executes a loop a certain number of times, potentially calling another routine once more if the byte was non-zero.
- `loop_fixed_count`: Executes a loop that decrements a register from a fixed starting value down to a certain limit, calling another routine within the loop.
- `decrement_two_counters`: Executes a loop that decrements two different registers simultaneously, calling another routine with a changing or fixed value.
- `clear_global_in_loop`: Executes a loop that decrements a register and sets a global variable to zero during each iteration.
- `loop_on_bit_test`: Compares a byte from a pointer in a register to itself and, if equal, tests bit 0 of that same pointer; if bit 0 is set, it enters an infinite loop, otherwise it returns.
- `execute_recursive_loop_2`: Recursively calls another routine while decrementing a local variable until it reaches a certain value.
- `execute_descending_loop_sequence`: Calls three different routines within a loop that executes 14 times, decrementing a counter in d7 until it reaches negative 1.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_global_pointer_offset`: Calls two routines, clears d2 and d3, reads two bytes from a pointer in a1, sets a global variable to the first byte left-shifted by 1, and calls a third routine.
- `process_string_with_mask`: Iterates through a buffer, performing bitwise masking and conditional branching based on character or byte values.
- `perform_conditional_sequence_of_calls`: Sets a byte in a global variable and performs a series of conditional checks based on the return values of multiple routines, updating various global pointers and values before clearing two bytes in global variables and returning.
- `reset_global_variable`: Clears a byte in a global variable, sets another global variable to 255, and sets the carry flag.
- `apply_relative_offset`: Iterates through a memory range and adds a value stored at a global pointer to the routine's own address if that value is non-zero.
- `process_data_sequence`: Sets a byte in a global variable, calls a routine, and then performs a series of checks against multiple global pointers and values, calling various routines if conditions are met before clearing two bytes.
- `reset_global_range`: Calls another routine, sets a global variable to 0, and then sets a global variable to 255.
- `search_table_and_set_values`: Checks if a global variable is 8 and bit 7 of another global is set, then iterates through a table to find a specific byte, potentially clearing bytes in a table and updating two 16-bit values and a global bit.
- `set_register_bounds`: Increments a pointer, copies a byte to a global variable, calls three subroutines, and sets two global words to specific values based on a global word plus 5.
- `modify_bitmask_and_pointer_2`: Sets two global words, copies a byte to a destination, calculates a pointer using a bitmask and a table, modifies a byte at that pointer, and performs several conditional branches and clears.
- `modify_bitmask_and_pointer`: Sets two global words, copies a byte to a destination, calculates a pointer using a bitmask and a table, modifies a byte at that pointer, and stores the source pointer in a global variable.
- `clear_memory_locations`: Clears a byte at a location pointed to by a global variable and clears a word in another global variable.
- `match_character_and_update`: Checks if a global variable is 59 and a byte at a global address is zero, then iterates through a table to find a matching byte, potentially updating two 16-bit values and a global pointer.
- `set_memory_registers`: Copies a byte from one location to another, calls two routines, assigns a constant to a global variable, and updates another global variable with a value from a third global variable plus five.
- `search_and_reset_table_entry`: Iterates through a table of bytes to find a match for a value read from memory; if the end of the table is reached without a match, it clears a byte at an offset of 25 from a pointer and clears a 16-bit value in a global variable, otherwise it returns.
- `update_pointer_and_bitmask`: Sets a global variable to a constant, updates another global variable with an offset, copies a byte between two memory locations, and modifies a byte at a calculated offset using a bitwise mask before calling two routines.
- `clear_specific_globals`: Clears a byte in a global variable and clears a word in another global variable.
- `update_state_and_hardware`: Checks a global byte against several constants to either copy values to a table, increment a global counter with a cap, or update a hardware register and a global pointer based on conditional checks and a call to another routine.
- `update_state_and_dispatch_command`: Updates several global variables and calls five subroutines in sequence.
- `compare_and_branch_status`: Compares a byte at a global address to six, otherwise it branches; if equal, it writes a constant to a global variable, clears a byte at a global address, and branches to another routine.
- `clear_and_mask_entry`: Calls two routines, clears a byte at a location pointed to by a global variable, clears a word in another global variable, and if a specific bit is set in the byte at the first location, it masks a byte at a calculated offset and clears a global byte before calling two more routines.
- `check_and_call_routine`: Tests a byte at a global variable, and if it is zero, branches to another location; otherwise it sets d0 to 4, calls another routine, and branches to another location.
- `reset_global_and_branch`: Sets a global variable to zero and executes a sequence of other routines based on certain value checks.
- `tiny_move_2`: Small routine of 1 instruction that copies 63 to d3.
- `search_for_byte_in_list`: Iterates through a list of bytes, comparing a byte from a register against a value at an offset from a pointer, and returns if a match is found.
- `calculate_pointer_offset`: Masks a value in d0, adds it to a base address in a0, stores the resulting pointer in a global variable, and calls another routine after modifying a byte from the table at that address.
- `update_global_pointer`: Stores a pointer in a global variable and sets another global variable to a calculated address based on a byte and a global variable.
- `process_conditional_pointer`: Checks bytes at offsets 1 and 2 of a6; if they are non-zero, it performs a series of calculations and calls three different routines, potentially updating a global pointer if a specific byte is zero.
- `parse_bitstream_instruction`: Retrieves a pointer from a table and calls a subroutine, then checks bit 7 of a byte at a location pointed to by a global variable.
- `update_object_list_and_dispatch`: Decrements a byte in a structure, calculates an offset by multiplying the result by six, and updates a global pointer to point into a table.
- `write_opcode_and_jump`: Writes a constant to a global variable and branches to another routine.
- `update_sprite_attribute_table`: Checks a byte at an offset from a pointer in a6, then calculates an index into a table to fetch three bytes and stores them into a global structure.
- `update_handler_pointer`: Checks a bitmask in a memory location and, if a certain condition is met, modifies a byte and updates a global pointer or increments a global variable.
- `process_object_state_and_update_global`: Reads an object structure from a pointer, performs conditional checks on its byte values, and updates several global variables and a table based on the results.
- `select_value_by_comparison`: Compares d1 and d3 against specific constants and, depending on the results, calls another routine and sets d2 to one of several values including 2, 49, or 50.
- `update_global_based_on_check`: Calls another routine, compares the result to 51, and updates a global variable with either 145 or 146 before storing a value from the stack into a global variable.
- `modify_memory_buffer`: Sets a global variable, modifies a byte in a table, and updates a memory location via a pointer.
- `process_bytecode_stream`: Increments a pointer and, if a specific byte is not found, performs several memory updates and enters a loop that checks multiple global bytes to decide whether to continue or branch.
- `select_status_byte`: Clears a byte in a global variable and, based on a comparison of a byte at an offset of 2 from a pointer in a0, sets d0 to either 47 or 66 before branching.
- `compare_memory_values`: Clears d0, sets two pointers, and enters a loop that compares a byte from one pointer to the value at the other pointer, incrementing the second pointer until it reaches the first.
- `validate_and_set_byte`: Tests d0, and if it is zero, branches to another location; otherwise it writes d0 to a6 offset 1, compares that byte to 24, and if it is not equal, branches to another location; if it is 24 or 28, it sets a global variable to 1, sets d4 to 150, calls another routine, and branches to another location.
- `compare_and_branch_byte`: Compares a byte at offset 1 of the structure in a6 to 55, and if it is not equal, branches to another location; otherwise it loads 151 into d4, calls another routine, and branches to another location.
- `parse_bytecode_sequence`: Checks if a byte at an offset from a pointer in a6 is the character 'C', then calls three different routines and checks a global variable before returning.
- `load_and_dispatch_pointers`: Copies a longword from a global variable to another global variable, loads a constant into a1, copies a byte from a0 offset 1 to another global variable, and checks a byte at a global variable; if that byte is not zero, it clears d0, calls another routine, copies a longword from a global variable to another global variable, calls another routine, and branches to another location.
- `dispatch_pointer_data`: Loads a constant into a1, copies a byte from a pointer at offset 1 of a0 to a global variable, copies a longword from a global variable to another global variable, and branches into another routine.
- `parse_command_string_and_execute`: Iterates through a string to parse command characters, updates several status bytes and a pointer in a global structure, and then executes a sequence of subroutines based on the parsed values.
- `modify_memory_with_offset`: Checks a bitmask in a byte at offset 3 of a pointer, modifies that byte if a condition is met, calculates a new address based on a global variable, stores that address in a global variable, and calls another routine.
- `update_bitmask_table`: Saves a6 to the stack, calls another routine, restores a6, increments it, and returns.
- `modify_list_and_globals`: Loads a pointer from a global variable, clears a byte in a global variable, writes 83 to an offset of that pointer, and clears a word in a global variable.
- `iterate_and_compare_table`: Compares a byte in a global variable to eight, calls two subroutines, and if a condition is met, masks a byte in a structure, calls another routine, and then loops through a table to compare bytes until a match or end of table is found.
- `call_initialization_routine`: Calls another routine and returns.
- `process_data_tables`: Initializes several global words to zero, iterates through a table to perform arithmetic and bitwise operations on values, and calls another routine.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `clear_memory_block_with_loop`: Iterates through a memory block using a pointer, setting the contents of each longword to zero until a certain condition is met.
- `store_values_to_offset_addresses`: Calls another routine three times, storing the first result at offset 4 of a1, the second result at offset 6 of a1, and the third result masked with 4095 at offset 8 of a1.
- `loop_bitmask_application`: Sets d5 to 100 and executes a loop that calls another routine 100 times before returning.
- `set_pointer_and_branch`: Sets a5 to a constant value and branches to another routine.
- `apply_bitmask_to_graphics_buffer`: Calculates a bitmask by rotating a constant value based on a field in a structure, then applies this mask to a target buffer at specific offsets using bitwise AND operations.
- `calculate_and_apply_bitmask`: Calculates a scaling factor using division and multiplication based on values in a structure, then uses it to derive bitmask values that are applied via bitwise OR to specific offsets of a memory block.
- `clear_and_branch`: Clears a 16-bit word at offset 8 of the structure in a1 and branches to another routine.
- `initialize_and_clear_tables`: Clears a byte at a global variable, loads 160 into d4, calls another routine, sets a byte at a global variable to 1, loads a pointer from a global variable into a0, and fills 6063 longwords starting at a0 with -1 using a loop.
- `set_registers_and_branch_2`: Sets d4 to 128, sets d5 to 85, and branches into another routine.
- `write_bytes_to_memory`: Sets d4 to 64 and d5 to 43, increments a6, loads a pointer from a global variable into a0, reads a byte from a0 offset 2 into d3, calls another routine, writes d4 to a0, writes a byte from a global variable to a0 offset 1, pushes d5 to the stack, calls another routine, pops d2 from the stack, sets d0 to 44 and d1 to 41, and calls three routines.
- `calculate_handler_offset`: Decrements a byte from an offset of a pointer, multiplies it by 6, and stores the resulting address into a global variable before branching to another routine.
- `update_char_and_global`: Stores a byte into a memory location and increments or decrements a global variable based on a character comparison and a register value.
- `increment_stack_pointer`: Adds 1 to the value in a6.
- `search_and_mask_table`: Searches a table for a specific byte pattern and then increments a global byte if certain characters are found while iterating through a pointer table.
- `process_structure_data_3`: Loads a pointer from a global variable, reads two bytes from it into d0 and d2, calls another routine, performs arithmetic on d4 based on a global variable, copies a word from one global variable to another, and writes a byte to offset 3 of a structure in a6.
- `set_memory_with_offset`: Subtracts 23 from d0, writes 148 to a6 offset 3, subtracts 40 from d2, writes d2 to a6 offset 2, writes d0 to a6 offset 1, calls another routine, sets d1 to 34, calls another routine, restores d0 and d2 from the stack, sets d1 to 43, and calls another routine.
- `search_for_char_in_list`: Iterates through a list using a pointer, checking if a byte at a certain offset matches a byte found in the list.
- `update_opcode_and_call_subroutines`: Sets d0 to 16, calls a subroutine, writes a word to a global variable, and then calls two more subroutines before returning.
- `execute_self_recursive_calls`: Clears d4, loads a byte from a global variable into d4, masks it, adds a constant, and then performs four calls to other routines.
- `update_status_and_structure`: Compares a 16-bit value at a global address to several constants and, depending on the result, updates a byte at an offset of a structure, clears several bytes at offsets 65 through 90 of another structure, and writes a 16-bit value to a global variable.
- `copy_memory_block_2`: Copies a block of data from one memory location to another using a nested loop structure and offsets, then calls three different routines and increments a6 before returning.
- `set_pointer_and_branch_alt`: Sets a4 to a constant value and branches to another routine.
- `swap_bytes_in_buffer`: Writes a value to a memory location and then performs a byte swap on a word at a specific pointer location.
- `recursively_process_byte_range`: Saves a byte from a global variable, checks if a byte at an offset of a pointer in a6 is between 59 and 62, and if so, sets a global variable to 255 and performs multiple recursive calls before restoring the original byte.
- `execute_conditional_buffer_updates`: Checks a byte in a global variable, and if non-zero, performs several operations including calling subroutines to move bytes between a pointer in a global variable and a destination, and updating a byte in a table using a value from another location.
- `update_string_buffer`: Calls two routines, calculates a value based on a byte from a global variable and a nibble-shifted version of that byte, stores the results in a structure at offset 6 and offset 10 of a6, calls several other routines, and performs a conditional loop based on bit 6 of a global variable.
- `calculate_table_index`: Calls a routine, performs bitwise operations on the result to calculate a new value, and then branches to another routine.
- `loop_until_specific_value`: Repeatedly executes a routine and checks a modified value against a constant, continuing the loop until the returned value equals a specific negative constant.
- `execute_indirect_call`: Performs an indirect call to a function pointer located at an offset from a global pointer.
- `initialize_hunk_relocation_data`: Clears a block of memory pointed to by a value in a data structure using a loop, then calls two other routines to update global variables and configuration values including a counter, a size calculation, and two constant values.
- `process_amiga_hunk_relocations`: Iterates through a list of data structures, calculating memory offsets and performing bitwise extractions to manage relocation data.
- `execute_sequence_of_calls_2`: Loads a pointer into a1, calls a subroutine, writes a word to a global variable, calls two more subroutines, and then branches to another routine.
- `execute_sequence_of_calls`: Calls three consecutive routines and then enters an infinite loop.
- `clear_list_elements`: Iterates through a list and sets any byte equal to a specific value to zero, then resets a global variable to zero.
- `replace_list_bytes_2`: Calls a routine to replace bytes in a list at offset a0 with a value in d0, then clears a byte at offset a6a3 and calls another routine to replace bytes in a different list.
- `replace_list_bytes`: Iterates through a list, and if a certain byte is found, replaces it with another byte and stores a global variable in the following byte.
- `modify_table_indices`: Saves a6 on the stack, calls another routine, decrements two bytes at offsets 50 and 51 of a pointer retrieved from a global variable, calls two other routines, and restores a6 before returning.
- `reverse_list_segment`: Reverses a segment of a list by swapping bytes from a start pointer and an end pointer moving towards each other.
- `load_structure_bytes`: Clears two registers, loads three bytes from offsets of a structure pointed to by a6 into those registers, shifts one byte left, and branches to another routine.
- `increment_and_jump`: Increments a6 by one and jumps to another routine.
- `write_byte_to_global`: Writes the value one to a global variable.
- `tiny_clr`: Small routine of 1 instruction that clears d1.
- `update_audio_and_pointer_structure`: If a byte in a global variable is zero, it sets a hardware register, performs a loop, and if a pointer in a global variable is non-zero, it calls another routine, clears that pointer, and eventually updates several hardware audio registers and a global pointer with values from a table.
- `reset_audio_volume`: Tests a byte in a global variable; if it is zero, it sets a hardware register to two and sets another hardware register to zero, then returns.
- `update_audio_state`: Compares a byte in a global variable against two constants, and if it matches a specific value and another global variable is less than a third, it calls a routine and then updates audio DMA registers.
- `increment_frame_pointer`: Increments the value in register a6 by one.
- `process_structure_data_2`: Checks a pointer, copies a longword, decrements a global byte, and performs several calculations and calls using values extracted from a structure.
- `process_and_increment_pointer`: Copies a byte from a6 offset 1 to d1, calls another routine, increments a6 by 2, and returns.
- `update_audio_parameter_table`: Iterates through a table to select specific audio parameters based on a byte value, then updates a structure with calculated volume and frequency values.
- `set_audio_parameters_2`: Sets the audio period, frequency, and volume by writing values from a table to hardware registers and a global variable.
- `set_audio_parameters`: Writes constant values to a global variable and hardware audio registers.
- `initialize_audio_state`: Opens a resource, clears a table of words, and populates a list with a calculated value before executing several indirect calls.
- `clear_audio_state`: Executes an indirect call to a routine that clears certain audio and DMA related global variables.
- `find_maximum_byte`: Iterates through a table of bytes and returns a value based on the maximum byte found in that table.
- `set_cia_register`: Writes a constant value to a global variable and copies a value from a global variable back to itself.
- `clear_four_word_table`: Clears a table of fourteen 32-bit entries by writing zero to each entry.
- `reset_audio_and_custom_state`: The routine writes zero to four audio volume registers and a custom register, and clears several global variables used for audio state.
- `clear_audio_and_dma_state`: Clears several audio and DMA related global variables and sets a global variable to a constant value.
- `call_subroutine_and_clear_d0`: Calls a subroutine and then clears d0 before returning.
- `increment_and_call_audio`: Increments a word in a global variable, and if it is not six, clears it and calls another routine; otherwise it returns.
- `traverse_and_call_indexed`: Sets d7 to 3, initializes several pointers, and then loops four times, calling a routine at an address calculated from a table and incrementing the pointers and counter in each iteration.
- `compare_and_update_structure`: Loads a word from a structure in a4, and if it is positive, adds it to an offset 2 of the structure, then compares the result with an offset 4 of the structure and updates offsets 2 and 6 of a structure in a5.
- `compare_and_branch`: Adds a word from d0 to an offset 2 of a structure in a4, compares the result with a word at offset 4 of the same structure, and branches based on the comparison.
- `dispatch_nibble_command`: Extracts a nibble from a byte at offset 2 of a structure in a6 and branches to different routines based on its value.
- `set_ciaa_bit`: Sets bit 1 of the CIAA_PRA register and returns.
- `clear_ciaa_bit`: Clears bit 1 of the CIAA_PRA register and returns.
- `fetch_table_word_by_nibble_2`: Calculates an offset using a nibble from a byte at offset 3 of a structure in a6, then searches a table for a matching word to determine a final word to store in a structure in a4.
- `fetch_table_word_by_nibble`: Doubles d2, negates it when d4 is zero, reads the word at a0 plus that offset and stores it at offset 4 of a4 before returning.
- `store_word_to_a4`: Stores the word in d0 at offset 4 of the structure in a4 and returns.
- `store_byte_and_branch`: Stores the byte from d4 into a memory location and then branches to another routine.
- `execute_jump_table_entry`: Clears d0 eight times, loads a value from a table into d0, subtracts 1 from it, shifts it left by 2, uses the result to load a new address into a0 from a table, and jumps to that address.
- `fetch_table_word_indexed`: Calculates an index by shifting d0, reads a word from a table at a0 using that index, and compares it to a word at offset 16 of the structure in a6.
- `shift_and_branch_index`: Clears d0, loads a byte from offset 3 of a structure in a6 into d0, shifts d0 right by 4 bits, and branches to another routine.
- `mask_and_branch_index`: Clears d0, loads a byte from offset 3 of a structure in a6 into d0, performs a bitwise AND with 15, and branches to another routine.
- `copy_word_from_a6`: Copies a word from offset 16 of the structure in a6 to offset 6 of the structure in a5 and returns.
- `update_pointer_from_offset`: Clears d0, loads a byte from offset 3 of the structure in a6 into d0, shifts it right by 4 bits, and if the result is non-zero, adds it to the word at offset 0 of the structure in a6 and stores the word from a6 at offset 6 of the structure in a5.
- `decrement_and_store_value`: Checks a byte from a memory location and, if non-zero, subtracts that value from a pointer and stores the updated pointer to another location.
- `update_audio_registers`: Calculates several addresses, performs a recursive call via a loop, sets a bit in a global variable, writes a value to a hardware register, and then performs a loop to write values from a table into audio hardware registers using DMA enabled writes.
- `update_entity_state_from_table`: Fetches an entity pointer from a table, uses a nibble from the entity's second byte to index into a secondary table, and updates a structure in a6 with values from that table.
- `update_clamped_value`: Adds a longword to a value at offset 4 of a structure in a6, stores the result at offset 10, and conditionally modifies a word based on a nibble in offset 2.
- `clamp_and_clear_value`: Adds a byte from offset 3 of a structure in a6 to d2, clamps the result to a minimum of 64, stores it at offset 8 of a structure in a5, and if a word at a6 is equal to 65533, clears a word at offset 2 of a6.
- `copy_entity_data`: If the word at a6 is non-zero, it clears a4, moves the word at offset 16 of a6 to offset 16 of a6, writes the word at offset 20 of a6 to the DMACON register, runs a loop 300 times, and then either clears offset 8 of a5 if the word at a6 is negative 2, or copies a longword from offset 2 of a6 to a5, a word from offset 4 of a6 to offset 1 of a5, and a word from offset 0 of a6 to offset 6 of a5.
- `copy_entity_fields_to_a5`: Copies a longword and two words from offsets of a6 into the a5 structure, then ORs the word at a6 offset 20 into the longword at address 5254 and returns.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `clear_and_store_pointer`: Clears d0 eight times, stores d2 into the structure in a5, and stores d0 into d3.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:RUNBACK

- `install_trap_handler`: Calls a routine at a longword address.
- `run_background_task`: The routine parses a command line for an optional load delay and a program name with parameters, then starts the named program as a background task, handling errors such as missing command, missing file, directory instead of file, out of memory and usage errors, and prints status messages.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `pop_values_from_stack`: Pops four longwords and one byte from the stack into d0, then pops two more longwords into d0 and performs a move of d0 to d0.
- `pop_and_modify_stack`: Pops 11 longwords from a stack into d0, sets bit 5 of the byte at the new top of the stack, then pops 11 more longwords into d0.
- `clear_stack_frame_and_install_trap`: Derives a base from the caller return address, clears 40 longwords in a loop, saves the stack pointer and a value from a global variable, and then calls another routine.
- `call_ram_test_hook_and_cleanup`: Tests the longword at offset minus 32766 from a4; if nonzero, loads it into a6 and calls it, then pushes the word at offset 8 of the frame and calls a routine before unlinking and returning.
- `initialize_libraries_and_parse_arguments`: Opens dos.library and mathffp.library, storing their bases in global variables, and then parses a command line string into an array of pointers by scanning for space characters and null terminators.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to d0.
- `execute_initialization_sequence`: Runs ten initialization calls, invokes optional callbacks and frees two pointers, then either allocates buffers and calls a routine with the argument or frees a pointer, returning the argument in d0 and restoring a7 from a saved value.
- `set_copy_limit`: Loads the constant 32767 into d0 and branches to another routine.
- `bounded_string_copy_return_dest`: Scans the destination for its terminator, copies up to the given count of bytes from source to destination, writes a final zero byte, and returns the destination pointer in d0.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_string_with_padding`: Copies bytes from source a1 to destination a0 up to a count in d1 or until a zero byte, then pads the remaining count with zero bytes and returns the destination pointer in d0.
- `lookup_and_clear_table_entry`: Calculates an offset into a table based on a value in d4, checks if the entry is within bounds and non-zero, and if the entry's high bit is clear, calls another routine before clearing the entry and returning zero.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_offset_102`: Calls a library function using an offset from a base address stored in a global variable.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `dispatch_library_function`: Restores registers from the stack, loads a library base address from an offset in a structure pointed to by a register, and calls a function from that library using an offset.
- `execute_indirect_jump`: Loads a pointer from a location calculated by adding a negative offset to a value in a register, then performs an indirect jump to the address found at a specific offset within that pointer's target.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `jump_to_indirect_address`: Performs an indirect jump to an address calculated by adding a negative offset to a value retrieved from a pointer located at a negative offset from a value in a register.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `dispatch_indirect_function_pointer`: Loads a longword from the stack into a data register, retrieves a pointer from a global table using an offset, and calls a function at an offset from that retrieved pointer.
- `dispatch_library_function_2`: Restores registers from the stack, loads a library base address from an offset of a pointer in a4 into a6, and calls a function from a table using an offset of 48 from that base address.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `jump_to_library_function`: Performs an indirect jump to a function located at an offset from a library base address stored in a global variable.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `jump_to_library_function_2`: Performs an indirect jump to a function located at a different offset from a library base address stored in a global variable.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:ENDCLI

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_and_branch_2`: Moves the contents of d1 to d0 and then branches to another location.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(Sp).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
