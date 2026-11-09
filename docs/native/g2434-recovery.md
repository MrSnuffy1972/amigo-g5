# Game 2434 (kult - the temple of flying saucers (1989)(exxos)(m3)[cr band]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02434). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904944 top-level bytes in 45 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 563 of 563 named (0 author symbols), 563 of 563 described (43 by rule, 517 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4848 of 4849 (99.98%) |
| Recovered without load address (counted in proof 1, flagged) | 80682 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10717 of 10719 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 563563 |
| code | 40960 |
| data | 278716 |
| padding | 10441 |
| unused | 11264 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:RUN | 2 | 117 | 2 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:KULT | 2 | 9262 | 496 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:type | 2 | 117 | 2 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:RUNBACK | 3 | 740 | 42 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:ENDCLI | 2 | 117 | 2 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:intro | 1 | 99 | 3 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:bootblock | 1 | 25 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 219 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:PUZZL.BIN (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- kult - the temple of flying saucers (1989)(exxos)(m3)[cr band]/Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf: file 1.BIN (header 23256 bytes, blocks hold 6344): its blocks are judged as raw blocks outside the filesystem
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 00C0) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 209 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- kult - the temple of flying saucers (1989)(exxos)(m3)[cr band]/Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:PUZZL.BIN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- kult - the temple of flying saucers (1989)(exxos)(m3)[cr band]/Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf: blocks 803-805 (bytes 411136-412672) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.06)
- kult - the temple of flying saucers (1989)(exxos)(m3)[cr band]/Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf: blocks 814-857 (bytes 416768-439296) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.12)
- 1 never-loaded region(s), 80682 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 512 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:intro

## Copy protection found

- `initialize_system_and_interrupt_handler` (KULT): Performs a sequence of initialization routines, sets specific byte values into a global structure based on conditional checks, manipulates hardware registers to modify interrupt enable bits and video positioning, and finally registers a new interrupt handler by updating the execution interrupt vector table. Kind: disk hardware/track validation check. Pass path: The branch following the `u1__sub_0_8A42()` call and the subsequent hardware register checks (`_DAT_00dff09a`, `_DAT_00dff09e`) which leads to `code_r0x01000b08`..

## Functions


### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:RUN

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of register d1 to d0 and then branches to another location.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:KULT

- `initialize_system_and_interrupt_handler`: Performs a sequence of initialization routines, sets specific byte values into a global structure based on conditional checks, manipulates hardware registers to modify interrupt enable bits and video positioning, and finally registers a new interrupt handler by updating the execution interrupt vector table.
- `update_audio_state_and_global_logic`: Increments a global variable and replaces its value with an offset-based value from a table, then executes a sequence of conditional checks that update a global variable and clear or set other global variables or pointers.
- `check_global_self_assignment`: Compares a byte from an offset of a global address to another byte at a nearby global address, calls a routine and performs an assignment if they are equal, then repeats this process for a second pair of global bytes.
- `update_pointer_with_mask`: Combines two bytes from the stack into a 16-bit value, applies a bitwise OR with a constant, and stores the result in a global variable.
- `move_stack_pointer`: Moves the value from the stack pointer register a7 to a memory location at offset a3a2.
- `traverse_string_with_recursive_calls_3`: Checks a 16-bit value from a global variable and branches based on its high nibble; it may call a routine, update a global pointer, or enter a loop that calls a routine from a table based on an index derived from the value.
- `update_global_values_conditionally`: Calls several routines and conditionally updates multiple global variables using arithmetic operations and threshold checks.
- `store_pointer_and_decrement`: Stores a 16-bit value from a pointer-based memory location into a register and then calls a routine that decrements a register.
- `increment_and_read`: Increments a pointer in a6 by 2 and then reads a byte from the memory location at offset -1 relative to the new a6 into d0.
- `recurse_and_return_registers`: Saves registers to the stack, loads a long value from a global address into d1, calls another routine, restores the registers, and returns the contents of d0 and d1.
- `process_char_and_recurse`: Updates a global variable and a memory location with a character derived from an input, then calls another routine recursively if a terminator is not reached.
- `traverse_and_update_global_state`: Iterates through a character buffer while updating a global variable and a secondary memory location with the current character.
- `traverse_string_musiq`: Loads a pointer from a global variable into a5 and calls another routine.
- `iterate_string_with_recursion`: Iterates through a string until a terminator character is found, passing a pointer and a value from an offset to another routine via a recursive call.
- `iterate_string_and_call_recursive`: Iterates through a data structure in memory, checking for a specific character terminator; if not found, it loads a value from an offset of 10 bytes into a register, sets a constant to an address, stores the current pointer in a register, and calls another routine recursively.
- `clear_memory_range_2`: Calls another routine, then zeros out a range of memory starting from a global variable pointer back to itself.
- `traverse_string_with_recursive_calls`: Iterates through a string until a terminator character is found, passing a pointer or a value from an offset or a constant to another routine recursively.
- `negate_and_load`: Negates the value in d6 and then loads a 16-bit word from a memory location into d0.
- `check_file_status`: Calls a library routine to check a status, then accesses a structure via a pointer to compare specific values and perform subtractions before returning.
- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `execute_conditional_dos_calls`: Calls a routine, and if the result is non-zero, it calls two other routines, potentially setting a return value to a maximum unsigned 16-bit value.
- `execute_conditional_dos_calls_2`: Writes three constants to global variables, calls another routine, and then calls two more routines based on the result of the first call, otherwise it returns a maximum value.
- `load_and_branch`: Loads the constant 500 into d2 and then branches to another location.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `apply_bitmask_to_memory_range`: The routine increments a global variable and then iterates through a list of offsets to apply a bitmask to a range of memory via bitwise operations and bit-rotated values.
- `traverse_linked_list_and_apply_mask`: Iterates through a linked list of memory offsets, checking bit patterns and updating a global variable or terminating if a specific end-of-list or control-bit pattern is encountered.
- `scan_for_pattern_and_decrement`: Compares a value from a global variable against a result from another routine, and if certain conditions are met, decrements a counter and calls another routine.
- `scan_variable_for_pattern`: Iterates through a memory range or a global buffer, checking if certain bits of a byte match a specific pattern and returning a calculated or incremented counter.
- `find_maximum_pattern_match`: Iterates through a memory range or buffer using a pattern-searching routine and tracks the highest-value match found.
- `calculate_pattern_distance`: Calculates the unsigned distance between a global variable and a value returned by a pattern-searching routine, updating a global variable with the result.
- `update_buffer_with_constants`: Checks a byte in a global variable and, if not equal to a specific value, copies several values from a table to global variables and calls two routines.
- `process_mask_parameters`: Compares a byte at a pointer in d0 to a constant, then loads two words from offsets of that pointer, decrements them, and calls two other routines.
- `set_global_constants`: Assigns specific constant values to several global variables and stores a pointer to a global variable before calling two other routines.
- `process_bit_mask_loop`: Sets d0 and d1 to specific constants, decrements them, calls another routine, and then calls a second routine after setting d2 and d3.
- `calculate_and_update_buffer_offset`: Calls a routine to scan a memory range, calculates a value based on the difference between a global variable and the scan result, and then updates that global variable by subtracting a bit-shifted portion of that difference before calling another routine.
- `decrement_counter_and_call`: Calls another routine, tests d6, and if d6 is not zero, decrements it by 1.
- `apply_bitmask_to_memory_range_2`: Clears register d0, calls another routine, and then iterates through a loop using d6 as a counter before returning.
- `increment_and_copy_pointer`: Increments a pointer in a6 by one, clears d4, loads a byte from the new address in a6 into d4, loads a constant into a5, calls another routine, and then copies a 32-bit value from one global variable location to another.
- `process_bitmask_control_params`: Extracts bitmasking parameters and pointer offsets from a control structure to initialize a bitmasking operation.
- `write_table_from_memory`: Calls a routine and then writes two bytes from the stack to global variables and a third byte to a table at a calculated offset.
- `initialize_and_fill_buffer`: Sets a global pointer and a counter, then executes a loop that writes a null byte to a global variable while iterating through a memory range using a fixed offset.
- `update_global_pointer_with_offset`: Iterates through a list of pointers, adding a pointer's value to itself, and finally stores a new calculated pointer into a global variable.
- `traverse_buffer_with_jumps_2`: Compares d4 to 4, branches if it is less, otherwise rotates d4 left by 8 bits, loads a byte from an offset of 1 from the address in a6 into d4, and increments a6 by 1.
- `traverse_buffer_with_jumps`: Updates a global variable with a value and then traverses a buffer, advancing a pointer by a value read from the buffer, until a certain condition is met.
- `tiny_lsl`: Small routine of 1 instruction that shifts d0 left by 2.
- `load_address_to_a0`: Loads a 32-bit value from a global variable into register a0.
- `calculate_scaled_index`: Masks bits from a register, shifts the result, and combines it with a scaled value from another register into a single 32-bit return value.
- `branch_with_address`: Loads the constant 32000 into a4 and branches to another routine.
- `update_global_pointer_with_offset_2`: Sets a4 to 0, loads a 32-bit value from a global variable into a1, and calls another routine.
- `pop_bytes_to_registers`: Calls another routine, subtracts a pointer from a0, clears d2 and d3, and then loads two consecutive bytes from the pointer in a1 into d3 and d2.
- `store_shifted_word`: Clears d5, moves d2 into d5, shifts d5 left by one bit, and stores the result in a global variable.
- `load_a3_from_global`: Loads a 32-bit value from a global variable into register a3.
- `tiny_subq_subq`: Small routine of 2 instructions that subtracts 1 from d2, then subtracts 1 from d3.
- `apply_bitmask_to_buffer_complex`: A nested loop routine that performs bitwise rotation and logical masking, updating a destination buffer and a secondary buffer offset by a large amount, while adjusting a pointer or counter.
- `fetch_and_scale_pointer_data`: Loads a pointer from a global variable, calls another routine, reads two consecutive bytes from the pointed-to location into registers, calls another routine, scales the second byte left by one bit, and stores the resulting word into a global variable before loading that new address into a3.
- `permute_bit_patterns`: Performs bitwise masking, shifting, and logical ORing to rearrange specific bits within a 16-bit value while iterating through nested loops.
- `permute_bits_in_word`: Performs bitwise masking, shifting, and logical ORing to rearrange specific bits within a 16-bit value.
- `decrement_and_loop_counter`: Decrements a 16-bit signed integer and repeats a routine call until the value reaches negative one.
- `loop_masked_buffer_copy`: Repeats a masked buffer copy routine a specified number of times, decrementing a counter in register d1 until it reaches negative one, while adjusting a pointer in a0 by 40 bytes each iteration.
- `decrement_scaled_global`: Loads a 32-bit value from a global variable into d7, shifts it left by one bit, and subtracts two from the result.
- `apply_rotated_bitmask_setup`: Saves d7 to the stack, loads a pointer from a global variable into a6, adds a value from a global variable to a3, subtracts 2 from a3, stores 3 in a global variable, clears d4, d5, d6, and d7, loads bytes from d3 into d5 and d6, performs arithmetic on d5 and d6, calls another routine, stores d7 in a global variable, sets a2 to a0, calls another routine, stores 2 in a global variable, and restores d7 from the stack.
- `apply_bitmask_loop`: Calls another routine, loads a pointer from a global variable into a1, sets d0 to all ones, clears d1, calls a library function, calls another routine, increments d4, subtracts 1 from a global variable and masks it with 3, compares the global variable to 3, branches if not equal, subtracts 2 from a3, and iterates using d7.
- `update_table_via_subroutine_2`: Sets d5 to all ones and branches to another routine.
- `update_table_via_subroutine`: Stores a word from d5 into a global variable, pushes d7 onto the stack, loads a pointer from a global variable into a6, performs several arithmetic operations on d4 and d5, calls another routine, stores d7 into a global variable, sets a6 to a0, and restores d7 from the stack.
- `apply_mask_to_buffer`: Calls another routine, loads a pointer from a global variable into a1, sets d0 to all ones, clears d1, calls a library function, calls another routine, tests a word at a global variable, branches if not zero, increments d4 and d2, increments and masks a global variable, branches if not zero, adds 2 to a2, and iterates using d7.
- `calculate_scaled_offset`: Loads a longword from a global variable into a register, shifts it left by one bit, and subtracts two from the result.
- `apply_rotated_bitmask_init`: Saves d7 to the stack, loads a pointer from a global variable into a6, sets a global variable to 0, clears d4, d5, d6, and d7, loads bytes from d3 into d5 and d6, performs arithmetic on d5 and d6, calls another routine, stores d7 in a global variable, sets a2 to a0, calls another routine, sets a global variable to 1, and restores d7 from the stack.
- `execute_masked_index_loop`: Executes a loop that calls a routine via a pointer-offset and updates a global variable using a bitwise mask.
- `set_d5_and_branch`: Sets d5 to a 16-bit value of negative one and branches to another routine.
- `calculate_and_store_indices`: Performs arithmetic on several registers to calculate values, calls a routine, and stores a 16-bit result into a global variable.
- `decrement_and_loop_with_masking`: Decrements a local variable and a global variable, resetting the global variable to a certain value if it reaches zero, while calling a routine that applies a bit-rotated mask to a buffer and an infinite loop routine.
- `execute_nested_decrement_loops`: Executes a nested loop structure that repeatedly calls another routine, decrementing a register or a global variable and a local variable, while using an unchecked value or register as a loop counter.
- `set_d4_and_branch`: Sets d4 to one and branches to another routine.
- `decrement_and_loop_with_mask_params`: Decrements a local variable and a global variable while calling routines that merge or shift data and calculate mask parameters.
- `process_conditional_buffer_offset`: Checks a byte in a global variable; if it is zero, it calls two subroutines, calculates a scaled offset using a 16-bit value from d3, updates a pointer from a global table, and calls another subroutine with a decremented value.
- `process_masked_buffer_indices`: Checks a byte in a global variable; if it is zero, it calls two subroutines, loads two words from a pointer-based structure, adds d3 to the second word, calls a subroutine, updates a pointer from a global table, and calls another subroutine with a decremented value.
- `shift_and_call_subroutine`: Checks a byte in a global variable; if it is zero, it calls a subroutine, shifts d0 left by two, saves a6 to the stack, calls another subroutine, and restores a6 before returning.
- `calculate_and_call_scaled_offset`: Checks a byte in a global variable; if it is zero, it calls a subroutine, adds d2 to d0, shifts d0 left by two, decrements d0, and calls another subroutine.
- `execute_sequential_subroutines_2`: Tests a byte at a global address, and if it is zero, clears d0, moves a byte from offset 1 of a6 into d0, calls two routines, modifies a1 using a0, and calls a third routine before incrementing a6 and returning.
- `execute_conditional_subroutines`: Tests a byte in a global variable and, if zero, loads a byte from the stack, calls three different routines with modified registers, and adjusts the stack pointer before returning.
- `execute_conditional_subroutines_2`: Tests a byte in a global variable and, if zero, loads a byte from the stack, calls a routine, performs arithmetic on registers, and calls another routine before returning.
- `execute_conditional_subroutines_3`: Tests a byte in a global variable and, if zero, loads a byte from the stack, calls a routine, performs a bitwise shift on a register, and calls another routine before returning.
- `update_table_with_values`: Searches a table for a zero value and then writes several register or local values into a table at a calculated offset.
- `branch_with_register_set`: Sets register d1 to two and branches to another routine.
- `set_table_entry_zero`: Calculates an offset based on an input value and sets a value in a table to zero.
- `compare_and_loop_table`: Compares a value at a pointer to register d0 and, if not equal, enters a loop that increments the pointer and decrements d6.
- `decrement_and_loop_with_mask_params_2`: Decrements a local variable and calls a routine that calculates mask parameters and merges or shifts data.
- `process_data_block_sequence`: Calls another routine to initialize parameters, then enters a loop that iterates a specified number of times based on a value in a register.
- `tiny_addq`: Small routine of 1 instruction that adds 1 to a6.
- `copy_table_blocks`: Copies large blocks of data from one memory area to another using a loop.
- `swap_table_elements`: Swaps or mirrors certain blocks of data between two memory areas using a loop.
- `clear_global_index`: Clears d0, loads a byte from an offset of one from a6 into d0, calls three different routines, increments a6 by two, and sets a global variable to zero.
- `execute_sequential_subroutines`: Loads a byte from an offset of 1 from the stack frame into a 16-bit register and calls two subsequent routines before returning.
- `increment_and_clear_d5`: Increments a6 by one, clears d5, loads a byte from the address in a6 into d5, and branches to another routine.
- `iterate_and_set_global_bit`: Checks a pointer and a bitmask, then enters a loop that sets a global variable to one while recursively calling another routine and decrementing a local variable if certain bits remain set.
- `retry_rotate_bits_and_invert`: Calls a routine to rotate bits and invert a value up to three times, returning the result masked with 127.
- `rotate_bits_and_invert`: Performs a bitwise rotation and inversion on a value from the keyboard serial register and returns it.
- `increment_pointer_and_return`: Calls another routine, increments a value in a6, and then returns.
- `iteratively_mask_and_delay_input`: Repeatedly masks a global variable with eight, calls two routines in a loop while a specific bit remains set, then sets the global variable to 255 and calls a final routine.
- `compare_and_call_routines`: Compares a byte at a global address to a constant and branches if equal, calls another routine, sets d0, d1, and d4 to specific values, tests a byte at a global address, and either sets d4 to a new value and calls another routine or returns.
- `clear_and_recursively_traverse_table`: Clears a global variable and enters a loop that recursively calls itself while checking a boolean flag and processing data passed via registers.
- `search_and_clear_table_entry`: Iterates through a table looking for a matching value and sets that table entry to zero if found.
- `clear_table_entry`: Performs a search in a table to clear an entry if a certain value matches.
- `copy_data_blocks`: Copies a block of words from a source table to a destination buffer using a loop.
- `merge_and_shift_data`: Merges bits from a source buffer into a destination buffer and shifts pointer or data pointers.
- `apply_bit_mask_to_buffer`: Applies a bitwise mask or its inverse to a buffer while copying or updating words from a source buffer.
- `load_mask_and_pointers`: Copies three words from a source to a destination and loads two pointers into registers.
- `copy_and_mask_buffer`: Copies words from a source buffer to a destination buffer while applying a bitmask and updating a pointer or offset.
- `call_subroutines_and_return`: Calls two different subroutines using the addresses stored at a global location and then returns.
- `load_pointer_to_a1`: Loads a pointer from a global variable into register a1.
- `extract_buffer_parameters`: Clears d2 and d3, then loads two bytes from a buffer into d2 and d3, followed by loading a long value into a0 and a word into d7, and finally a long value into a3.
- `apply_bitmask_and_rotate_to_buffer`: Iterates through a source buffer, applying a bitmask-based bit-rotation or bit-shifting (using a rotating bit-mask pattern) and writes the resulting bit-manipulated or bit-aligned words into a destination buffer.
- `load_pointer_from_memory`: Loads a long value from a specific memory location into a1.
- `apply_bit_rotation_to_buffer`: Iterates through a source buffer and writes bit-rotated or bit-shifted versions of specific bit-fields or words into a destination buffer.
- `permute_bits_in_buffer`: Performs bitwise masking and bit-rotation-like operations on a buffer using a calculated bitmask.
- `calculate_mask_parameters`: Calculates bitmask values and updates a global variable used for masking or bit-width calculation.
- `initialize_and_rotate_buffer`: Zeros out a buffer range and then applies a bit-rotated mask to a source buffer, writing the results to a destination buffer.
- `apply_bit_rotation_to_buffer_2`: Iterates through a buffer to apply bitwise masking and bit-rotation-like operations to specific memory offsets.
- `initialize_buffer_and_call_subroutines`: Subtracts a constant from a pointer, then calls two different subroutines before returning.
- `apply_rotated_bitmask`: Calculates a bit-rotated mask and uses it to bitwise-AND and OR values into a table, or potentially a buffer, using a loop controlled by a global counter.
- `apply_mask_to_offset_buffer`: Applies a bit-rotated mask to a buffer using bitwise operations, where the source buffer is an offset version of the destination buffer.
- `load_global_and_initialize_buffer`: Calls two different subroutines before returning.
- `initialize_buffer_and_mask`: Fills a buffer with zeros and updates global variables used for masking or bit-width calculation.
- `traverse_string_with_recursive_calls_2`: Iterates through a string until a terminator character is found, passing a pointer or a value from an offset or a constant to another routine.
- `extract_and_store_buffer_data`: Extracts or constructs a 32-bit value from a buffer, then stores parts of that value and other pointer-based bytes into global variables and a table.
- `decrement_register_loop`: Decrements a value in a register until it reaches a certain limit.
- `populate_buffer_from_indexed_data`: Calculates an offset into a data table, retrieves values, and performs a series of nested loops to write these values and others into a target buffer.
- `load_and_call_subroutines`: Loads a constant into a register and calls two subroutines.
- `process_buffer_with_bitmasks`: Iterates through a buffer, performing bitwise operations, rotations, and multiplications to update memory locations and potentially call other routines based on bit tests, until a pointer comparison fails.
- `iterate_pointer_list_and_set_constants`: Iterates through a list of pointers, checking the high bit of each pointer's value and advancing the pointer by a fixed offset, then writes a sequence of constant values to certain registers or memory locations.
- `apply_bitmask_to_buffer`: Iterates through a buffer, performing bitwise OR, bitwise AND, and bit-rotation or bit-swapping operations before storing the result back to the buffer.
- `mask_and_store_bytes`: Applies a bitmask or its inverse to bytes in a buffer and stores them back into a buffer located at a calculated offset.
- `extract_bits_via_rotations`: Performs a series of bitwise rotations and logical AND operations on register d1 to construct a value in d6.
- `initialize_buffer_values`: Sets a global variable to a specific value based on a comparison, then performs two sequences of copying multiple registers from memory locations to a destination structure and updating a global word with concatenated values from memory.
- `update_global_pointer_constants`: Sets a global variable to 93 or 103 based on its current value, calls two routines, updates two global variables with the low and high words of a value from a table, and calls two more routines.
- `set_global_variable`: Assigns a constant value to a global variable based on a condition and copies a word from one memory location to another through two separate sequences of register transfers.
- `cycle_state_and_update_tables`: Increments a state byte through a four-step cycle and executes different sequences of table updates and register saves based on the current state.
- `process_buffer_rotations`: Calls several routines with specific register values and then enters a loop that performs bitwise rotations and masks on data pointed to by registers.
- `execute_sequential_subroutines_3`: Calls a sequence of six subroutines with varying values in registers d0, d1, and d2, ending with a branch to another routine.
- `set_table_and_return`: Tests a byte in a global variable and, if zero, loads a byte from the stack, calls a routine, and writes specific values to two global variables before returning.
- `update_buffer_with_scaled_index`: Calculates scaled offsets and increments a counter to check against a limit, while updating a global variable.
- `update_parameter_offsets`: Clears four 16-bit registers, loads four bytes from a table pointed to by a6, subtracts the first two bytes from the third and fourth bytes respectively, writes a constant to a global variable, sets a byte to seventeen, increments the pointer in a6, and branches to another routine.
- `update_table_with_rotated_mask`: Uses a bit-rotated mask to perform bitwise AND and OR operations on specific offsets within a table.
- `calculate_buffer_scaling_and_loop`: Checks a byte in a global variable; if it is zero, it calls a subroutine, performs several arithmetic operations to store scaled values in global variables, enters a loop that runs 100 times calling a subroutine, and performs comparisons and subtractions on global values.
- `apply_bitmask_to_buffers`: Calculates a bitmask using a bit-rotation-like operation and applies it to several distant memory offsets via a pointer.
- `branch_to_handler`: Performs a relative branch to another routine.
- `store_pointer_and_branch`: Stores a long value into a global variable and branches to another routine.
- `modify_buffer_bits`: Iterates through a buffer and modifies specific bits in a series of words based on a bitmask check.
- `modify_bit_patterns_via_table`: Saves d0-d3, swaps d6 and d7, calls a subroutine, adds two global pointers to a0 and a1, calculates a bit index, and uses a bit test to either set or branch based on a bit in a memory location.
- `modify_bit_patterns_3_2`: Clears a bit in d1, stores d0 in a memory location at offset 7968 from a0, and then either sets a bit in d0 or branches based on a bit test.
- `modify_bit_patterns_3`: Clears a bit in d1, stores d0 in a memory location at offset 7968 from a0, and then either sets a bit in d0 or branches based on a bit test of d2.
- `modify_bit_patterns_2`: Clears a bit in a register, stores the register in a memory location, performs a bit test on a value from a table, and either sets a bit or branches based on the result.
- `modify_bit_patterns_in_loop`: Clears a bit in a register, stores that register into a memory location at an offset from a pointer, and then enters a loop that decrements a register and branches based on a counter.
- `modify_bit_patterns`: Iterates through a pointer and modifies specific bits in a destination pointer, a second pointer, a third pointer, and a fourth pointer based on a bitmask.
- `write_long_and_branch`: Stores a long value into a global variable and then branches to another location.
- `update_bit_flags`: Iterates through a memory range and updates specific bits in several remote memory locations using a bitmask and a counter-based loop.
- `branch_to_handler_2`: Performs a short branch to another routine.
- `load_and_call_buffer_parameters`: Checks a byte in a global variable; if it is zero, it calls two subroutines, loads four words from a global structure into d0-d3, saves d3 and a0 to the stack, calls a subroutine, restores them, loads a pointer from a global table, and calls another subroutine.
- `load_and_call_buffer_parameters_alt`: Checks a byte in a global variable; if it is zero, it calls two subroutines, loads four words from a global structure into d0-d3, saves d3 and a0 to the stack, calls a different subroutine, restores them, loads a pointer from a global table, and calls another subroutine.
- `jump_to_address`: Performs a long jump to an address located at an offset of 1 from the stack frame.
- `execute_sequential_subroutines_4`: Tests a byte in a global variable and, if non-zero, clears d0, loads a byte from the stack, and calls two routines before storing a value in a global variable.
- `update_global_counters_2`: Tests a byte in a global variable and, if non-zero, clears d0, loads a byte from the stack, and calls two routines while storing d3 and d2 into global variables.
- `execute_indirect_subroutines`: Calls two subroutines using addresses retrieved from a global location and then returns.
- `copy_memory_block`: Performs a nested loop to copy data from a source pointer to a destination pointer with a fixed stride.
- `get_unaligned_word`: Reads a 16-bit value from a pointer offset by two bytes.
- `update_pointer_and_store_value`: Reads a 16-bit value from a pointer, then updates a global pointer and a global variable with a new address.
- `advance_pointer_with_backup`: Stores the value at a global pointer into another global variable before incrementing that pointer by two bytes.
- `store_constant_to_global`: Stores the constant value 40810 into a longword at a global variable.
- `set_global_bounds`: Sets a global variable to a specific value and another global variable to a constant integer.
- `execute_infinite_loop`: Compares a byte at a global address to itself and branches to itself if they are equal, creating an infinite loop.
- `register_interrupt_handler`: Adds a new interrupt handler to the exec interrupt vector table and then calls the previous handler.
- `check_linked_list_entries`: Calls a routine via a pointer from a list three times, storing the result in different global variables and returning if any result is zero.
- `allocate_and_initialize_buffers`: Calls several functions via pointers to allocate memory and then updates multiple global variables with the returned values or calculated offsets.
- `execute_task_related_functions`: Calls three different functions via pointers retrieved from a global variable and its offsets, involving task and port management.
- `update_linked_list_pointers`: Calls a routine via a pointer in a table, updates several global pointers with offsets from the returned value, and copies bytes from the mouse port to two global locations before branching to another routine.
- `update_buffer_pointers`: Calls a routine, updates two global variables with calculated values, stores a byte and a word from the stack into a destination buffer, and calls another routine.
- `calculate_interleaved_division_factors`: Checks a byte in a global variable; if it is zero, it calls a subroutine, calculates several division and remainder factors using d3 and d0, stores them in global variables, and enters a loop that calls a subroutine and performs arithmetic on global values.
- `update_swapped_value`: Swaps the bytes of a 32-bit register, stores it in a global variable, and enters a loop that performs arithmetic on global variables and calls another routine.
- `execute_infinite_loop_3`: Updates a global variable with a swapped-byte value and enters a loop that modifies a global variable via subtraction while checking a condition or calling another routine.
- `sync_remote_data`: Checks a global byte, calls two routines, and if a condition is met, iterates through a table to copy bytes from one memory location to another at four specific offsets, then updates a value in a table.
- `copy_bytes_to_remote_locations`: Copies bytes from a source table to four remote memory locations at offsets 0, 8000, 16000, and 24000 relative to a pointer, then enters a loop that checks a global variable and calls another routine.
- `populate_table_from_source`: Iteratively copies bytes or related data from a source pointer to a table while calculating offsets and checking for zero values.
- `interleave_bitstream_data`: Calculates division and multiplication factors to process a source table, then enters a nested loop that extracts bits from 16-bit words and interleaves them into a destination buffer using bitwise shifts and rotations.
- `bit_shift_and_interleave_data`: Performs bitwise shifts and masking on two 32-bit values from a structure to create new 32-bit words, then enters a loop that extracts specific bit fields from a table to interleave them into a destination buffer.
- `setup_data_copy_loop`: Loads a source pointer and a word count from a stack-relative location, decrements both values, and branches back to repeat the process.
- `extract_and_repack_bitfields`: Calculates division and remainder values to determine bitfield offsets and then enters a nested loop structure that reads bits from a source table and packs them into four separate 16-bit word buffers.
- `manipulate_and_shift_bitfields`: Performs bitwise shifts and swaps on two 32-bit values stored across four 16-bit words in a structure, then enters a loop that extracts bits from a table to populate a destination buffer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_and_apply_bitmask_rotation`: Calculates two 16-bit differences from global variables, uses them to determine a bitmask and a rotation amount, and then enters a loop that applies these bitwise operations to four memory buffers at 8000-byte intervals using a rotated 32-bit pattern.
- `check_and_rotate_register`: Compares a byte at the address in a5 to 1; if equal it returns, otherwise it rotates d3 left by 1, potentially increments a0 and a1, and branches to another location.
- `rotate_and_mask_buffers`: Rotates a 32-bit value in d3 left by one, then performs bitwise masking and logical OR operations between two buffers at a0 and a1 using the rotated value, and branches to another routine or increments the pointers based on a byte at offset 1 of a5.
- `subtract_and_branch`: Subtracts 40 from the addresses in a0 and a1, adds d4 to d7, and branches to another location.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_buffer_range_bounds`: Calculates and writes new range boundaries into global variables and executes a loop that modifies these bounds or pointers.
- `update_global_counters`: Calculates and stores new values in several global variables and executes a loop that decrements a value while calling another routine.
- `copy_table_blocks_2`: Copies data from a source pointer to a destination pointer in a loop.
- `clear_registers_and_fetch_byte`: Clears word d0 and long d1, then loads a byte from an offset of a6 into d1 and increments a6 by two.
- `set_pointer_and_loop`: Stores a word from d0 into a global variable, calls another routine, stores a pointer into a global variable, and then executes a loop that increments a pointer by 3 until it reaches the stored pointer value.
- `scan_and_update_bounds`: Iterates through a list of values, checking if bytes fall within a specific range, and updates a global variable with a maximum value or certain bounds-related constants.
- `process_bit_reordering`: Rearranges bits and checks certain character flags before recursively calling another routine with a calculated word value.
- `increment_and_call_two`: Increments a6 by 1, calls another routine, calls a third routine, and increments a6 by 3 before returning.
- `increment_and_call_one`: Increments a6 by 1, calls another routine, increments a6 by 3, and returns.
- `fetch_byte_and_increment_pointer`: Loads a byte from an offset of a6 into d0 and increments a6 by two.
- `update_pointer_metadata`: Calls a routine to populate a pointer table, then executes a loop that decrements a value and modifies a pointer-based structure, specifically decrementing a value at an offset and updating a pointer-based address.
- `advance_frame_pointer`: Reads a byte from an offset of 1 relative to the pointer in a6 into d0 and increments the pointer in a6 by 2.
- `modify_pointer_table_values`: Calculates a new value based on a pointer, updates a global variable, and then executes a loop that modifies a value in a table.
- `populate_pointer_table`: Iterates through a list of pointers to calculate and store new pointer values and associated metadata into a table.
- `execute_loop_with_calls`: Loads a value from a pointer and executes a loop a fixed number of times while calling other routines.
- `swap_byte_with_global`: Reads a byte from a global variable into d6, calls another routine, and then writes d6 back to the same global variable.
- `update_state_and_fill_table`: Reads a byte from an offset in a structure, swaps it with a value from a global variable, and then conditionally fills a seven-byte table with a specific constant based on the value read.
- `unpack_and_store_data`: Reads bytes from a source pointer and selectively stores certain bytes into a destination buffer based on bitmask checks.
- `branch_to_subroutine`: Calls a subroutine at a specific address and then branches to another location.
- `process_bitstream_metadata`: Initializes a global variable and certain pointers, then searches through a table to check for a bitmask match or a specific value, performing various subroutine calls and memory updates based on the results.
- `traverse_table_and_clear`: Recursively calls another routine and checks a global variable before setting a global variable to zero, while also performing conditional subroutine calls and memory writes based on global byte values.
- `dispatch_on_char`: Performs unconditional calls to another routine if a global variable matches any of several specific character constants.
- `shift_global_values`: Loads two words from global variables into d0 and d1, then performs a logical shift right on d0 by two bits.
- `check_input_range`: Checks if two input bytes fall within a range defined by a table.
- `set_pointer_and_branch`: Loads the constant value 967c into a4, the constant value 10 into d2, and branches to another routine.
- `search_for_valid_index`: Increments a value and checks it against a range, potentially setting a global or updating a pointer if a valid range or value is found.
- `invoke_subroutines`: Reads two bytes from memory, calls another routine, and then performs several conditional checks on a global word and a byte to decide whether to call a final routine.
- `find_matching_entry`: Iterates through a list of entries to find one where a byte at an offset matches a specific value, then updates several global pointers and values.
- `search_for_matching_pattern`: Iterates through a memory list to find an entry where a certain masked bit pattern matches a changing global value.
- `validate_and_update_pointers`: Performs two range checks using subroutines; if both pass, it updates a byte in a global variable, sets a word in a global variable, and clears a word in another global variable.
- `load_pointer_and_calculate_offset`: Loads a value from a pointer-based offset into a global, updates a global with a byte from a pointer, and calculates a new pointer offset based on a value in memory.
- `evaluate_state_and_initialize`: Checks a byte in a global variable, calls another routine, and then performs a series of conditional checks on other global variables to update a global variable and a byte in a structure.
- `tiny_move`: Small routine of 1 instruction that copies 150 to d1.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_two_subroutines`: Calls two different subroutines and then returns.
- `load_calculated_addresses`: Calculates an offset based on an input, then loads values from that offset and a fixed address into global variables.
- `execute_if_value_changed`: Checks if a global value differs from a previously stored value and calls another routine if a change is detected.
- `process_string_data`: Checks a global byte against a constant, calls another routine, and if a global word is non-zero, performs calculations based on a byte from a pointer-based structure to update a global table and a global word, then calls another routine.
- `store_and_branch`: Stores the word 49596 at a global address and branches to another location.
- `store_and_branch_alternate`: Stores the word 49599 at a global address and branches to another location.
- `update_state_based_on_bytes`: Writes bytes to the locations in a1 and a2, moves a byte from a global variable to an offset of a0, and updates a global word with one of four values based on comparisons of bytes at offsets of a1 and a2 before branching.
- `update_table_values`: Iterates through a table, checking if a byte equals a certain value and replacing it with a different value.
- `validate_string_entry`: Sets a global word, calls another routine, and if certain global conditions are met, calculates an offset from a byte in a structure to compare values in a table and potentially update a global byte and a global word before calling another routine.
- `update_mouse_pointer_address`: Writes a 16-bit value to a longword at a specific memory location and branches to another routine.
- `decrement_and_jump_2`: Clears a byte in a global variable, calls another routine, and if the byte is not zero, updates several values in memory and calls another routine based on specific byte comparisons.
- `decrement_and_jump`: Decrements a byte at the address in a5 and branches to another location.
- `initialize_state_and_dispatch`: Sets a byte in a global variable, sets a 16-bit value in d2 and d3, writes a value to a global variable, calls another routine, and branches to another routine.
- `initialize_state_and_dispatch_alt`: Sets a byte in a global variable, loads a 16-bit value from a global pointer into d3, writes a value to a global variable, calls another routine, and branches to another routine.
- `initialize_global_state`: Sets a global variable to zero or one and checks a character at an offset from a pointer-based address.
- `update_state_and_process_input`: Clears a 16-bit value in a global variable, calls three subroutines with specific parameters, and then increments a byte in a global variable by 109 if it is not equal to 100.
- `fetch_table_entry`: Calculates an index by multiplying a byte from a global variable by ten and adding a shifted value from a register, then stores a 16-bit word from a table at that index into a global variable, or alternatively stores a 16-bit word from a different table location into the same global variable before branching to another routine.
- `manipulate_and_compare_byte`: Moves a byte from a global variable into d2, compares d2 to d0, and if d2 is not lower than d0, sets d1 to the high nibble of d2, subtracts d0 from d2, writes d2 to a global variable, and if the high nibble of d2 equals the high nibble of d1, returns.
- `invoke_instruction_stream`: Adjusts the stack pointer and calls a routine that processes a sequence of instructions from a memory buffer.
- `initialize_and_store_byte`: Sets d0 to 2, calls two other routines, and stores a byte with value 255 into a global variable.
- `update_state_and_dispatch_event_2`: Updates a global state variable and uses it to index into a table to retrieve a value, which is then used to either call a specific routine via a pointer or execute a loop of subroutine calls based on the value's high nibble.
- `load_word_from_address_2`: Copies a 16-bit word from a global variable to another global variable and branches to another routine.
- `compare_and_set_word`: Compares a byte from a global variable to 63 and branches; if not, it compares a byte from another global variable to 22 and branches; if that is not 1, it stores the word 36869 in a global variable and branches to another routine.
- `resolve_and_dispatch_tagged_pointer`: Updates a global 16-bit value by selecting an entry from a table based on a byte value, then branches to a routine or executes a loop of function calls determined by the high nibble of that value.
- `dispatch_tagged_pointer`: Copies a 16-bit value from a global variable to a pointer and branches.
- `process_state_and_dispatch_event`: Checks a global status byte and branches to various subroutines based on its value.
- `clear_byte_and_load_word`: Copies a 16-bit word from a global variable to another global variable, clears a byte in a global variable, and branches to another routine.
- `write_global_constants`: Writes the values one and two into two different global variables and returns.
- `set_constant_word`: Stores the word 36986 in a global variable and branches to another routine.
- `clear_and_mask_table_elements`: Iterates through a table of 40-byte-aligned entries, clearing the high bit of the second word in each entry.
- `update_entity_state_and_dispatch`: Updates a global pointer, performs a series of memory swaps in a loop, and iterates through a table to check byte values against constants before calling a subroutine.
- `process_data_with_calls`: Sets a bit in a byte at an offset of a pointer, clears registers, loads two bytes from a pointer, calls a subroutine with a value in d1, calls another subroutine, and branches to an earlier instruction.
- `update_state_and_dispatch_event_3`: Updates a global state variable by calling several subroutines and then performs a table lookup to update a pointer.
- `dispatch_tagged_pointer_2`: Copies a 16-bit value from a global variable to a second global variable and branches to a new location.
- `update_menu_selection_and_dispatch`: Updates a selection index by retrieving a value from a table based on a global variable, then checks the high nibble of the resulting word to either execute a function from a pointer table, perform a series of state updates and calculations, or enter a loop that calls functions from a table until a zero or specific byte value is encountered.
- `dispatch_tagged_pointer_3`: Copies a 16-bit value from a global variable to a register and branches to a new location.
- `dispatch_control_structure_command`: Validates a control structure by checking specific byte bounds and then branches to a command handler based on a bitmask in a word.
- `load_word_from_register`: Copies the 16-bit word from register d4 to a global variable and branches to another routine.
- `iterate_and_store_data`: Iterates through a table, checks a bitmask against a value, and stores certain bytes and pointers into a global location.
- `apply_mask_to_pointer_table`: Iterates through a nested table of pointers and applies a bitwise mask or bit-manipulated value to the memory pointed to by those pointers.
- `update_counters_and_dispatch_command`: Increments a byte in a global variable and adds 300 to a 16-bit value in another global variable, then performs two subroutine calls with different values loaded into a shared pointer.
- `set_constant_word_alt`: Stores the word 50012 in a global variable and branches to another routine.
- `execute_conditional_branch`: Calls two routines, branches if a condition is met, writes a word to a specific memory address, and then performs an unconditional branch.
- `write_constant_and_branch`: Writes the word 37022 to a global variable at offset 33696 and branches to another routine.
- `write_constant_and_branch_2`: Writes the word 50013 to a global variable at offset 33696 and branches to another routine.
- `execute_conditional_branch_alt`: Calls three routines, branches if a condition is met, calls a fourth routine, and then writes a word to a specific memory address before performing an unconditional branch.
- `reset_global_parameters`: Sets several global variables to specific constant values and enters a loop that continues while a specific bit in a global variable is set.
- `update_global_state_and_pointer_table`: Updates a global variable using a clamped or masked value and applies a bitmask-driven pointer-based update to a table of global variables.
- `process_state_driven_pointer_table`: Saves a pointer to a global variable, updates a status byte to 255, and enters a loop that checks a 16-bit value for specific high-nibble patterns to either call a routine from a table, update a counter, or execute a sequence of nested calls.
- `load_and_branch_2`: Loads a pointer from a global variable into a3, sets a byte at a global address to 1, and branches to another routine.
- `scatter_data_to_buffers`: Iterates through a loop, calling a routine that copies or scatters data into a destination, and then executes a secondary loop.
- `scatter_table_data`: Copies bytes or words from a source table to a non-contiguous destination pattern.
- `load_registers_from_memory`: Clears register d1, reads a byte from an offset of a6 into d1, increments a6, clears d0 and d2, and then reads two consecutive bytes from a6 into d0 and d2.
- `process_data_table`: Clears two words in memory, stores two words from registers into memory, and then iterates through a table, calling subroutines or an indirect function pointer based on the value of the current byte.
- `calculate_offset_address`: Calculates a new address or offset based on specific bits in a register and stores it in a global variable.
- `fetch_and_call_offset`: Moves a byte from the stack into register d0, calls another routine, clears d0, and moves another byte from the stack into d0.
- `clear_and_process_buffer`: Clears a block of memory to zero and then iterates through a buffer to perform bitwise operations and store results.
- `set_and_branch`: Writes a 16-bit value of 65535 to a global variable, writes a byte of 1 to another global variable, and then branches to another location.
- `copy_and_branch`: Copies a 16-bit word from a global variable to another global variable and then branches to another location.
- `set_and_branch_short`: Writes a 16-bit value of 1 to a global variable and then branches to another location.
- `update_global_pointer_2`: Decrements a word from a global variable by one and stores the result into another global variable, then branches to another routine.
- `decode_bitstream_loop`: Iterates through a buffer, extracting bitfields to update a global variable and a secondary global variable, while using nested loops to process or terminate based on bit-shifted values.
- `update_string_index_counter`: Checks a global byte for a specific value and then modifies a 16-bit counter in a global variable based on the state of three bits in that byte.
- `load_and_process_hardware_parameters`: Loads four 16-bit values from global memory into registers, calls two subroutines to process this data, and clears a 16-bit global variable before returning.
- `copy_and_update_buffer_range`: Performs a scatter-gather style memory copy from a source buffer to a destination buffer using nested loops to move words at specific offsets, then updates several global boundary and limit variables with values from registers and memory.
- `fetch_offset_byte`: Retrieves a byte from a calculated memory address based on bit-masked flags and an offset or pointer lookup.
- `evaluate_register_conditions`: Checks a bitmask in a register and performs several conditional comparisons between register values before returning.
- `execute_conditional_calls`: Calls two routines, moves a word from d0 to d6, and moves a byte from a pointer in a6 to d7, then branches based on bit 7 of d7 or performs two more calls.
- `store_byte_conditionally`: Increments a6 by 1, calls another routine, and then checks bit 7 of a value in d0 to decide whether to store a byte or a word from d6 into the memory location pointed to by a0.
- `branch_on_word_zero`: Increments a6 by 1, calls another routine, and if the result in d6 is zero, it decrements a6 and branches to another routine; otherwise, it increments a6 by 2 and returns.
- `process_button_input`: Checks certain bits in a register and updates a global variable based on the state of those bits.
- `update_mouse_position_and_transform_table`: Updates a global coordinate based on mouse port movement, clamping the values within a specific range, and then performs a bitwise logical transformation on a block of memory using a temporary value and updates a table or global pointer.
- `increment_and_dereference_global`: Increments a global variable and then replaces its value with the value stored at that new offset in a table.
- `increment_and_load_pointer`: Increments a global variable and returns the value stored at the address or offset pointed to by that incremented value.
- `toggle_global_word`: Calls three routines, writes a word to a global variable, calls another routine, writes zero to the same global variable, and calls one final routine.
- `update_list_elements`: Subtracts a value from a global variable and compares it to a constant, then updates a global variable if the result is at least 180 before looping through a table to clear bit 6 of entries if certain conditions are met.
- `call_copy_words_loop`: Loads a byte from an offset of 1 in a structure pointed to by a6 into d0, increments a6 by 2, and calls another routine.
- `increment_a6_register`: Adds one to the value in register a6.
- `load_word_from_address`: Clears register d0 and loads a word from the address stored in a global variable into it.
- `copy_words_in_loop`: Multiplies d0 by ten, then enters a loop that copies words from one memory location to another using two incrementing pointers.
- `load_address_into_a1`: Loads the address stored in a global variable into register a1.
- `call_load_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls LoadRGB4 and returns its result to the caller.
- `update_data_table_values_2`: Copies sixteen words from a source table to a destination table, then iterates through four subsequent words in the source table to increment their nibble values while masking specific bits.
- `update_data_table_values`: Copies a block of words from a source table to a destination table, then iterates through a second table to modify word values by masking and adjusting their nibbles based on comparisons with a target table.
- `clear_buffer_with_check`: Zeros out a buffer of 16 words and then iterates through a table, calling another routine if bit 6 of an offset in the table is set.
- `rotate_and_store_buffer`: Copies a long value from a1 to a destination pointed to by a2, loads four bytes from a5 and performs subtractions, calls another routine, and then calls a second routine before returning.
- `compare_byte_and_branch`: Compares a byte from a memory location to two different constants and branches if the value is between 22 and 25.
- `check_byte_value`: Clears d1, loads a byte from a global variable into d1, and branches to another routine if the byte is less than 22 or between 25 and 24 inclusive, otherwise it branches to another routine.
- `increment_frame_pointer`: Increments the value in the address register a6 by one.
- `execute_recursive_loop`: Executes a routine that calls itself recursively while decrementing a register, repeating a fixed number of times.
- `check_pointer_and_recurse`: Checks if a pointer in a2 is non-zero, and if so, extracts several bytes from the pointed-to structure and calls another routine.
- `increment_frame_pointer_2`: Increments the value in a6 by one.
- `traverse_and_process_buffer`: Iterates through a sequence of 15 memory-aligned words and calls another routine if a non-zero value is encountered.
- `update_bitmask_table`: Increments a pointer in a6, calls a routine to check for a bitmask match, and then returns.
- `search_and_update_buffer_2`: Iterates through a table to check if a byte matches a masked value, then updates a byte in the table and a byte in another memory area.
- `set_address_register`: Sets address register a4 to zero and branches to another routine.
- `search_and_update_buffer`: Iterates through a table to check if a byte matches a masked value, then increments a byte at a calculated destination address and updates a global variable.
- `compare_memory_words`: Compares a word from one memory location to a word from another and branches if the first is less than the second.
- `scan_buffer_for_pattern`: Iterates through a buffer and checks if certain bits in a byte match a specific pattern, then recursively calls another routine and updates a pointer.
- `process_pointer_data`: Clears register d0, moves a byte from an offset of 1 of a pointer in a6 into d0, calls another routine, moves bytes from offsets 2 and 3 of a6 into d2 and d3, and increments a6 by 4.
- `update_global_state`: Stores two bytes from registers into a table, clears a global byte, reloads those bytes into registers, calls three different routines, compares two global bytes, and if they differ, calls three more routines before checking bit 6 of a global byte and potentially setting that global byte to 1.
- `update_global_state_logic`: Loads a constant into register d0, calls two subroutines, sets registers d2 and d3 to specific byte values, calls another subroutine, and loads a pointer from a global variable into a0.
- `write_constant_values`: Writes three long values to specific memory locations, calls another routine, writes two bytes to specific memory locations, and branches to another routine.
- `execute_recursive_call`: Increments a6, calls a routine to process bitmasking parameters, calls another routine to traverse a buffer, and then increments a6 before returning.
- `traverse_memory_recursively`: Executes several routine calls, including a recursive call that increments a register and other routines that process bitmasking or memory offsets.
- `modify_byte_and_recursively_call`: Increments a6, calls a routine to process bitmasking parameters, calls a routine to traverse a buffer, applies a bitmask to a byte at offset 3 of a6, and then calls three other routines before returning.
- `modify_byte_and_recurse`: Sets d4 to 6, compares a byte at an offset of a pointer to 55 and 170, calls a subroutine, copies a word from one location to another, calls another subroutine, clears a bit in a byte at an offset from a pointer, calls three more subroutines, and returns.
- `modify_memory_with_offsets_2`: Loads a pointer from a memory location into a0, compares the byte at a0 to 35, and if it is not equal, writes a byte from an offset of a0 into a memory location at an offset of 3 from a6.
- `modify_memory_with_offsets`: Reads a byte from a pointer and writes modified or offset values to adjacent memory locations relative to a register-based address.
- `load_address_a816`: Loads the constant value 43030 into a1 and branches to another location.
- `load_address_a7ee`: Loads the constant value 42990 into a1 and branches to another location.
- `load_address_a7c6`: Loads the constant value 42950 into a1 and branches to another location.
- `update_command_table_entry`: Calls another routine, writes a constant value to a global variable, and checks two global bytes to decide whether to proceed.
- `initialize_and_loop_data_tables`: Sets a global byte to 1, copies a pointer from a table to another location, and enters a nested loop structure that repeatedly calls three different routines while decrementing counters.
- `branch_after_save`: Saves registers to the stack, calls a subroutine, and then branches to another location.
- `execute_nested_loop`: Executes a nested loop structure that decrements registers, effectively performing a fixed number of empty or secondary routine calls.
- `update_masked_buffer`: Performs a bitwise or-not and and-with operation while copying or updating values in a large buffer using a nested loop structure.
- `copy_buffer_segments`: Copies data from a source buffer to a destination buffer using a nested loop structure.
- `update_buffer_with_scaled_index_2`: Performs bitwise operations on a buffer using values from a table and updates several memory locations through a nested loop structure, then writes values to specific global variables and branches to another routine.
- `conditional_loop_on_byte`: Checks if a byte at a pointer is non-zero and then executes a loop a certain number of times, potentially calling another routine once more if the byte was non-zero.
- `loop_fixed_count`: Executes a loop that decrements a register from a fixed starting value down to a certain limit, calling another routine within the loop.
- `decrement_two_counters`: Executes a loop that decrements two different registers simultaneously, calling another routine with a changing or fixed value.
- `clear_global_in_loop`: Executes a loop that decrements a register and sets a global variable to zero during each iteration.
- `execute_infinite_loop_2`: If bit 0 of a global variable is not set, this routine enters an infinite loop; otherwise it returns.
- `execute_recursive_loop_2`: Recursively calls another routine while decrementing a local variable until it reaches a certain value.
- `execute_descending_loop_sequence`: Calls another routine, then enters a loop that executes two other routines and increments a register fourteen times before returning.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_global_pointer_offset`: Calls two routines, then extracts two bytes from a table, shifts the second byte left by 1, stores the result in a global variable, and calls a third routine.
- `process_string_with_mask`: Calls a subroutine, clears registers, loads three bytes from an offset of a6, performs several conditional checks and subroutine calls including a loop that processes a buffer, and eventually returns.
- `perform_sequential_checks`: Sets a byte in a global variable, calls several routines sequentially, and performs multiple conditional checks based on return values and comparisons of global memory locations before clearing two bytes and returning.
- `reset_global_variable`: Sets a global variable to zero and then to a maximum byte value.
- `apply_relative_offset`: Iterates through a memory range and adds a value stored at a global pointer to the routine's own address if that value is non-zero.
- `process_data_sequence`: Sets a byte in a global variable, calls a routine, and then performs a series of checks against multiple global pointers and values, calling various routines if conditions are met before clearing two bytes.
- `reset_global_range`: Calls another routine, sets a global variable to 0, and then sets it to 255.
- `search_table_and_set_flags`: Iterates through a table to find a byte matching a global variable, then performs several checks to update specific memory locations and bit flags.
- `set_register_bounds`: Sets a register to a specific constant value and sets another register to an offset relative to a global variable, then performs several subroutine calls and updates two global words.
- `modify_bitmask_and_pointer_2`: Increments a pointer, checks a byte at a global address, modifies a global word, sets a bit in a global byte, copies a byte from a pointer to another location, and performs several conditional checks and clears a global word.
- `modify_bitmask_and_pointer`: Sets a global word, updates a global word with an offset, copies a byte between memory locations, and modifies a byte at a calculated address using a bitmask before calling three routines.
- `clear_memory_locations`: Sets two specific memory locations to zero.
- `search_table_and_update_values`: Checks a global variable against a constant, then iterates through a table to find a matching byte, and if found, updates a global pointer and two word values based on specific byte comparisons.
- `set_memory_registers`: Sets a pointer in a1 to an offset of a global pointer, copies a byte from one global location to another, calls two subroutines, writes a constant to a global variable, updates another global variable with an offset, and calls three more subroutines.
- `search_table_and_clear_flags`: Iterates through a table of bytes to find a match for a value read from memory; if a match is found, it clears a byte at an offset from a global pointer and clears a 16-bit word in another global variable, otherwise it clears a 16-bit word in a global variable and returns.
- `update_pointer_and_bitmask`: Writes a constant to a global variable, updates a global variable with an offset, copies a byte from one memory location to another, and modifies a byte at a calculated offset using a bitwise mask before calling two subroutines.
- `clear_two_globals`: Clears a byte at a global variable and a word at another global variable.
- `update_game_state_and_hardware`: Checks a global byte against several constants to update a structure at a1, modifies a global variable by adding five and wrapping it at twenty-five, and calls another routine to update hardware registers and a global pointer.
- `update_state_and_dispatch_event`: Updates several global variables and pointers, then calls five subroutines in sequence.
- `check_and_reset_byte`: Compares a byte at a global address to six; if not equal, it otherwise returns; if equal, it writes a constant to a global pointer, clears a byte at a global address, and branches to another routine.
- `clear_table_entry_and_mask`: Calls two subroutines, clears a byte at an offset of a global pointer, clears a 16-bit global variable, and if a specific bit is not set in the byte at the pointer, modifies a byte at a calculated offset, clears a global byte, and calls two subroutines.
- `test_and_call_routine`: Tests a byte at a global address, and if it is zero, sets d0 to 4, calls another routine, and branches to another location.
- `reset_global_and_branch`: Compares a byte at a global offset to 61, calls a subroutine, clears a byte, performs several conditional checks on global bytes, calls a subroutine with a value in d4, calls another subroutine with values from a pointer, and finally clears a global byte.
- `tiny_move_2`: Small routine of 1 instruction that copies 63 to d3.
- `search_for_byte_in_list`: Iterates through a list of bytes, comparing a byte from a register against a value at an offset from a pointer, and returns if a match is found.
- `calculate_offset_and_call_handler`: Masks a value in d0, calculates an address by adding it to a base address in a0, stores this address in a global variable, and calls another routine with a modified byte in d0.
- `update_global_pointer`: Stores a pointer in a global variable and sets another global variable to a calculated address based on a byte and a global variable.
- `process_conditional_logic`: Loads two bytes from a6, performs arithmetic and bitwise operations on d0, calls three different routines, and if a byte at a global address is non-zero, stores a global pointer into another global location.
- `parse_bitstream_instruction`: Pushes registers to the stack, retrieves a pointer from a table, calls a routine, and if bit 7 of a value at a pointer is set, updates two global words with shifted and direct byte values from a buffer, then conditionally calls another routine and writes three bytes to a specific memory location before calling a final routine.
- `process_object_list_and_dispatch`: Decrements a count in a structure, updates a pointer to a table of entries based on the new count, and increments a global index.
- `store_constant_and_branch`: Stores a 16-bit constant into a global variable and branches to another routine.
- `update_sprite_table_and_render`: Checks a condition to select a base address from a table or calculates an offset into a table based on a value at an offset of 4 from a global pointer, then writes three bytes to a global structure and calls another routine.
- `update_handler_pointer`: Checks a bitmask in a memory location and, if a certain condition is met, modifies a byte and updates a global pointer or increments a global variable.
- `process_input_event_data`: Reads a data structure via a pointer in a global variable, performs several conditional checks on its bytes, and updates various global variables and a table based on the results.
- `select_value_based_on_comparisons`: Compares d1 and d3 against specific values and calls another routine to set d2 to either two, thirty-one, or thirty-two based on the results and the return value in d0.
- `update_global_based_on_check`: Calls another routine, compares the result in d0 to 51, sets a byte in a global variable based on the result, and stores a value from the stack into a global variable.
- `modify_memory_buffer`: Sets a global variable, modifies a byte in a table, and updates a memory location via a pointer.
- `process_bytecode_stream`: Updates a global pointer, modifies bytes in a table, and enters a loop that branches back to itself based on several byte comparisons at specific global offsets.
- `set_conditional_values`: Clears a 32-bit value at a global address and, depending on the value at offset 2 of a0, sets d0 to either 47 or 66, otherwise it clears a 16-bit value at a global address and sets two pointers.
- `compare_memory_values`: Compares a byte from a global variable with a byte at a memory location pointed to by a6 in a loop that continues until a6 reaches a specific address.
- `check_and_set_bytecode`: Tests d0, and if it is non-zero, writes d0 to offset 1 of a6, compares that byte to 24, and if it is 28, sets a byte at a global address to 1, sets d4 to 150, calls another routine, and branches to another location.
- `compare_and_branch_bytecode`: Compares a byte at offset 1 of a pointer in a6 to 55, and if equal, sets d4 to 151, calls another routine, and branches to another location.
- `parse_bytecode_sequence`: Compares a byte at offset 1 of a pointer in a6 to 67, and if equal, calls two routines, checks a global variable, and calls another routine; otherwise it calls one routine and returns.
- `transfer_and_call_routines`: Moves a longword from a global variable to a table, sets a1 to 39491, moves a byte from offset 1 of a0 to offset 1 of a6, and if a byte at a global address is non-zero, clears d0, calls another routine, moves a longword from a global variable to a0, and calls another routine before branching.
- `transfer_byte_and_branch`: Moves a longword from a global variable to a table, moves a byte from offset 1 of a0 to offset 1 of a6, and branches to another routine.
- `parse_command_sequence`: Iterates through a string of commands, updating a pointer and modifying various status flags and control bytes based on the characters encountered.
- `modify_memory_with_offset`: Clears a bit in a byte at a pointer offset and calculates a new address stored in a global variable based on a global variable, then calls another routine.
- `update_bitmask_table_2`: Iterates through a table to check for a bitmask match, then updates a byte in that table and a byte in another memory area.
- `clear_and_set_values`: Loads a pointer from a global variable, clears a byte at another global variable, writes the value 53 to an offset of that pointer, and clears a word at a final global variable.
- `iterate_global_table`: Compares a global byte to a constant, calls two different routines, and if a condition is met, masks a byte in a structure and calls another routine, then enters a loop that compares a global byte against a table of longwords until a match is found or the end of the table is reached.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `process_data_tables`: Fills a table with specific values using a loop, clears several global words, and then performs a series of calculations and bit manipulations on data from multiple tables and global variables.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `clear_memory_block_with_loop`: Iterates through a memory block using a pointer, setting the contents of each longword to zero until a certain condition is met, while calling another routine during each iteration.
- `store_values_to_offset_addresses`: Calls another routine three times and stores the resulting words into a sequence of memory locations at offsets 4, 6, and 8 from a1, applying a mask to the final value.
- `loop_apply_bitmask`: Sets d5 to 100 and executes a loop that calls another routine 100 times before returning.
- `set_address_register_alt`: Sets address register a5 to a constant value and branches to another routine.
- `apply_bitmask_and_scale_sprite_data`: Calculates a bitmask by rotating a constant value based on a field in a structure, then applies this mask to three specific word locations in a target memory block.
- `calculate_and_apply_sprite_mask`: Calculates a scaling factor using division and multiplication, then uses it to scale two values from a structure.
- `clear_and_branch`: Clears a 16-bit word at an offset of 8 from a1 and branches to another routine.
- `process_data_and_loops`: Clears a byte at a global address, sets d4 to 160, calls another routine, sets a byte at offset 1 of a pointer in d0 to 1, fills a memory block pointed to by a pointer in d0 with a fixed address using a loop, fills another memory block pointed to by a pointer in d0 with 0 using two loops, calls three routines, sets d4 to 27, sets d2 to 95 and d1 to 44, calls three routines, and branches to another routine.
- `set_registers_and_branch`: Sets d4 to 128, sets d5 to 85, and branches to another routine.
- `write_bytes_to_memory`: Sets d4 to 64, sets d5 to 43, increments a6, loads a pointer from a global variable into a0, loads a byte from offset 2 of a0 into d3, calls another routine, writes d4 to offset 0 of a0, writes a byte from offset 1 of a global variable to offset 1 of a0, pushes d5 to the stack, calls another routine, loads d2 from the stack, sets d0 to 44 and d1 to 41, calls three routines, and returns.
- `calculate_and_store_pointer`: Calculates an address by multiplying a value derived from a byte at offset 1 of a6 by 6 and adding it to a base address, stores this address in a global variable, and branches to another routine.
- `update_char_and_global`: Stores a byte from d3 into offset 2 of the structure in a5 and increments or decrements a global variable based on a character comparison and the value of d4.
- `increment_frame_pointer_3`: Increments the value in the sixth register by one.
- `search_and_mask_table`: Searches a table for a specific byte pattern and then increments a global byte if certain characters are found while iterating through a pointer table.
- `process_data_and_update_memory`: Loads two bytes from a table into d0 and d2, calls another routine, copies a word from one global variable to another, and conditionally writes a byte to offset 3 of a6.
- `set_memory_with_offset`: Subtracts 23 from d0, writes 148 to offset 3 of a6, subtracts 40 from d2, writes d2 to offset 2 of a6, writes d0 to offset 1 of a6, calls two routines, sets d1 to 34, calls another routine, restores d0 and d2 from the stack, sets d1 to 43, calls another routine, restores d0 and d2 from the stack, sets d1 to 45, and calls another routine before returning.
- `search_for_char_in_list`: Iterates through a list using a pointer, checking if a byte at a certain offset matches a byte found in the list.
- `execute_setup_sequence`: Loads the value 16 into d0, calls another routine, stores 37259 at a global address, and calls two more routines before returning.
- `execute_self_recursive_calls`: Performs a sequence of four calls to other routines.
- `update_status_and_structure`: Compares a 16-bit value at a global address against constants, and depending on the results, updates a byte at an offset of a structure, clears several bytes at offsets 65 through 90 of a structure pointed to by a6, and writes specific values to offsets 95 through 110 of that same structure.
- `copy_memory_block_2`: Calls two routines, copies a block of data from one memory location to another using a nested loop structure and offsets, calls another routine, and then calls two more routines.
- `set_address_register_final`: Sets address register a4 to a constant value and branches to another routine.
- `swap_bytes_in_buffer`: Writes a value to a memory location and then performs a byte swap on a word at a specific pointer location.
- `handle_conditional_logic`: Stores a byte from a global variable, checks if a byte at an offset of a pointer in a6 is between 58 and 62, and if so, sets a global variable to 255 and calls two routines twice, otherwise it returns.
- `process_conditional_buffer_updates`: Checks a byte in a global variable, and if non-zero, performs several operations involving calling two routines, moving bytes between memory locations, and calling a final routine before restoring registers and returning.
- `update_string_buffer`: Calls two routines, calculates two derived byte values based on a byte from a global variable and stores them at offsets 6 and 10 of a structure in a6, calls several other routines, and performs a conditional loop based on bit 6 of a global variable.
- `calculate_offset_and_audio_params`: Calls a routine, then calculates a new value for d4 and d1 based on the result using bitwise operations and additions before branching to another routine.
- `loop_until_specific_value`: Calls a routine, then repeatedly calls another routine twice and checks a modified value in d1 against a constant, continuing the loop until the returned value equals a specific negative constant.
- `execute_indirect_call`: Performs an indirect call to a function pointer located at an offset from a global pointer.
- `initialize_hunk_metadata`: Clears a memory block of 8000 words, calls another routine, and updates several global variables with calculated values including a decremented counter, a bit-shifted value, and fixed constants.
- `process_executable_hunks`: Iterates through a list of data structures, extracting bitfield offsets and sizes to calculate memory addresses and lengths.
- `load_resource_and_branch`: Loads an address into a1, calls another routine, stores 37278 at a global address, and then calls two more routines before branching to another location.
- `execute_three_routines`: Calls three consecutive routines and then enters an infinite loop.
- `clear_list_elements`: Iterates through a list and sets any byte equal to a specific value to zero, then resets a global variable to zero.
- `replace_list_bytes_2`: Increments a value in a6, calls a routine with a pointer in a0 and a count in d0, clears a byte at a global address, calls another routine with a pointer in a0 and a count in d0, and finally loads a pointer into a0.
- `replace_list_bytes`: Iterates through a list, and if a certain byte is found, replaces it with another byte and stores a global variable in the following byte.
- `modify_table_indices`: Calls a routine, decrements bytes at offsets 50 and 51 of a structure pointed to by a0, then calls three other routines before returning.
- `reverse_list_segment`: Reverses a segment of a list by swapping bytes from a start pointer and an end pointer moving towards each other.
- `load_buffer_parameters`: Loads three bytes from an offset of a6 into d4 and d1, performs a left shift on d1, and branches to another routine.
- `increment_pointer_and_jump`: Increments the value in a6 by 1 and branches to another routine.
- `write_byte_to_address`: Stores the byte value one into a memory location pointed to by a global variable.
- `tiny_clr`: Small routine of 1 instruction that clears d1.
- `update_audio_dma_registers`: Updates audio registers and enables a DMA channel.
- `reset_audio_volume`: If a byte at a global address is non-zero, it sets the audio volume to zero and the DMA control register to a specific value, otherwise it returns.
- `update_audio_state`: Increments a global variable and then replaces its value with the value stored at that new offset in a table, followed by updating audio DMA registers.
- `increment_a6`: Increments the value in register a6 by one.
- `process_structure_data`: Checks a pointer, updates a global byte, and if the pointer is non-zero, performs several calculations and calls two different routines before branching to another routine.
- `call_audio_routine`: Moves a byte from offset 1 of a6 to d1, calls another routine, increments a6 by 2, and returns.
- `update_audio_parameter_table`: Updates a table of audio-related parameters, including volume and DMA settings, based on a conditional check of a value.
- `set_audio_parameters_2`: Writes a constant to a hardware register, then if a pointer in a global variable is non-zero, it reads three words from that pointer to update a hardware register, a global variable, and another hardware register before advancing the pointer.
- `set_audio_parameters`: Writes constant values to a global variable and hardware audio registers.
- `initialize_audio_state`: Opens a resource, populates a list of 15 words using a loop, calls a routine to store a pointer, calls another routine to perform an indirect call, and then calls three more routines before returning a boolean result.
- `clear_audio_state`: Executes an indirect call to a routine that clears certain audio and DMA related global variables and then calls another routine.
- `find_maximum_byte`: Iterates through a table of bytes and returns a value based on the maximum byte found in that table.
- `set_cia_register`: Writes a constant value to a global variable and copies a value from a global variable back to itself.
- `clear_four_word_table`: Clears a table of fourteen 32-bit entries by writing zero to each entry.
- `reset_audio_and_custom_state`: The routine writes zero to four audio volume registers and a custom register, and clears several global variables used for audio state.
- `clear_audio_and_dma_state`: Clears several audio and DMA related global variables and sets a global variable to a constant value.
- `call_subroutine_and_clear_d0`: Calls another routine and then clears d0 before returning.
- `increment_and_call_indexed`: Increments a 16-bit value in a global variable, and if it does not equal 6, it clears the variable and calls another routine; otherwise, it clears the variable and returns.
- `traverse_and_call_indexed`: Loops four times, calling a routine at an address calculated from a table, incrementing the address and a counter in each iteration.
- `compare_and_update_structure`: Loads a word from a structure in a4, branches if it is zero or negative, otherwise adds it to an offset of 2 in the same structure, compares the new value at offset 2 with a value at offset 4, and if the new value is not higher, stores the value from offset 4 into offset 6 of a structure in a5 and into offset 2 of the structure in a4 before returning.
- `compare_and_branch`: Adds a word from d0 to offset 2 of a structure in a4, compares the result with the word at offset 4 of the same structure, and branches based on the comparison.
- `dispatch_command_nibble`: Compares a nibble extracted from a byte at offset 2 of a structure in a6 against several constants and branches to different routines based on the result.
- `set_ciaa_fire_bit`: Sets bit 1 of the CIAA Port A register.
- `clear_ciaa_fire_bit`: Clears bit 1 of the CIAA Port A register.
- `fetch_table_word_by_nibble_2`: Clears offset 0 of a structure in a4, stores a word from a6 at offset 2, calculates an offset using a nibble from a6, and then searches a table to store a word at offset 4 of a4.
- `fetch_table_word_by_nibble`: Doubles d2, negates it when d4 is zero, reads the word at a0 plus that offset and stores it at offset 4 of a4 before returning.
- `store_word_to_a4_offset`: Stores the word in d0 at offset 4 of the structure in a4.
- `store_byte_and_branch`: Stores the byte from d4 into a memory location and branches to another routine.
- `dispatch_via_table`: Clears d0 eight times, loads a table address into a0, calculates an index using a value from a global variable, and jumps to the address at that index in the table.
- `copy_word_between_structures_2`: Shifts d0 left by one, copies a word from offset 16 of a structure in a6 into d1, and then uses d0 as an offset to read a word from a table and compares it with d1.
- `shift_and_branch`: Clears d0, moves a byte from offset 3 of a6 into d0, shifts d0 right by 4 bits, and branches to another routine.
- `mask_and_branch`: Clears d0, moves a byte from offset 3 of a6 into d0, performs a bitwise AND on d0 with 15, and branches to another routine.
- `copy_word_between_structures`: Copies a word from offset 16 of the structure in a6 to offset 6 of the structure in a5.
- `decrement_and_store_value_2`: Extracts a nibble from offset 3 of a structure in a6, and if it is non-zero, adds that value to the structure in a6 and stores the resulting word into offset 6 of the structure in a5.
- `decrement_and_store_value`: Checks a byte from a memory location and, if non-zero, subtracts that value from a pointer and stores the updated pointer to another location.
- `update_audio_registers`: Calls a routine to update entity state, loops four times to call another routine, sets a bit in a global variable and writes it to an audio register, loops 300 times, then loops four times to write values from a table into audio hardware registers using DMA, and finally updates a global pointer and index based on a threshold check.
- `update_entity_state_from_table`: Fetches an entity pointer from a table, uses a nibble from the entity's second byte to index into a secondary data table, and updates a structure in a6 with values from that table.
- `update_clamped_value`: Adds a longword to a value at offset 4 of a structure in a6, stores it at offset 10, and then either clamps a value based on a nibble at offset 2 or clears a value if a specific condition is met.
- `clamp_and_clear_value`: Adds a byte from offset 3 of a6 to d3, clamps the value of d3 to a minimum of 64, stores the result at offset 8 of a5, and if a 16-bit value at a6 is equal to a specific constant, clears a 16-bit value at offset 2 of a6.
- `update_dma_and_copy_entity_fields`: Updates the DMA controller with a word from an offset of a6, executes a dummy loop of 300 iterations, and copies a longword and two words from a6 into the structure pointed to by a5, or clears a word at offset 8 of a5 if a6 contains a specific terminator value.
- `copy_entity_fields_to_a5`: Copies a longword and two words from offsets of a6 into the a5 structure, then ORs the word at a6 offset 20 into the longword at address 36760 and returns.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d1.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:type

- `process_buffer_and_install_callback`: Walks a linked list to find a maximum value, writes a header of sequential words into a buffer, and sets up a stack frame.
- `move_and_branch_2`: Moves the longword in d1 to d0 and then branches to a relative offset.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:RUNBACK

- `call_trap_handler`: Calls a routine at a specific address.
- `run_background_task`: Parses a command line for an optional load delay and a program name with parameters, then starts the named program as a background task, handling errors such as missing command, missing file, directory instead of file, out of memory and usage errors, and prints status messages.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `pop_long_words`: Pops six long words from the list pointed to by a0 into d0 and sets bit 5 of the byte at the new a0 location.
- `pop_long_words_extended`: Pops twenty-one long words from the list pointed to by a0 into d0 and sets bit 5 of the byte at the new a0 location.
- `clear_stack_frame_and_install_trap`: Derives a base from the caller return address, clears 39 longwords in a loop, saves the stack pointer and a value from a global variable, and then calls another routine.
- `call_ram_test_hook_and_cleanup`: Tests the longword at offset minus 32766 from a4; if nonzero, loads it into a6 and calls it, then pushes the word at offset 8 of the frame and calls a routine before unlinking and returning.
- `initialize_libraries_and_parse_arguments`: Opens dos.library and mathffp.library, stores their bases in global variables, and parses a command line string into a table of pointers.
- `copies_data_register_to_itself`: Copies the contents of data register zero to data register zero.
- `execute_initialization_and_callbacks`: Runs ten initialization calls, invokes optional callbacks, and calls another routine with an argument derived from a table entry, returning the argument in d0 and restoring a7 from a saved value.
- `load_maximum_word`: Loads the value 32767 into d0 and branches to another routine.
- `bounded_string_copy_return_dest`: Scans the destination for its terminator, copies up to the given count of bytes from source to destination, writes a final zero byte, and returns the destination pointer in d0.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_string_with_padding`: Copies bytes from source a1 to destination a0 up to a count in d1 or until a zero byte, then pads the remaining count with zero bytes and returns the destination pointer in d0.
- `process_table_entry`: Calculates an offset into a table based on a value in a5, checks if the index is within range and the entry is non-zero, and if the entry's high bit is clear, calls another routine before clearing the entry.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_offset_102`: Calls a library function using an offset derived from a global pointer.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `dispatch_library_function`: Restores registers from the stack, loads a library base address from an offset in a structure pointed to by a register, and calls a function at a specific offset within that library.
- `call_indirect_library_function`: Loads a pointer from a global variable, applies an offset of 48, and jumps to the address stored at that location.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `jump_to_indirect_address`: Performs an indirect jump to an address retrieved from a pointer located at an offset of 1810 from a base address stored in a register.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_indirect_via_a6`: Loads a long value from the stack into d1, loads a pointer from an offset of a global variable into a6, and performs an indirect jump using a6 with an offset.
- `call_indirect_with_save`: Saves d1 through d3 and a stack pointer to the stack, loads a pointer from an offset of a global variable into a6, and performs an indirect jump using a6 with an offset.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector`: Performs an indirect jump to a library function using a pointer from a global variable and a negative offset.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_jump`: Loads a function pointer from a location calculated by adding a negative offset to a value found at a specific offset within a structure pointed to by a global variable, then jumps to that address.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:ENDCLI

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header containing sequential words into a buffer.
- `move_d1_to_d0`: Moves the contents of d1 to d0 and then branches to another routine.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:intro

- `unpack_compressed_data_stream`: Copies a block of data from one memory location to another and then decompresses a bitstream into a destination buffer using a series of nested loops and bit shifts.
- `pop_and_rotate_longword`: Moves a longword from the location pointed to by a0 into d0, moves a byte from d0 into d7, moves a word from d3 into a structure at a5, sets the carry flag, and rotates d0 left by 1 bit before returning.
- `process_data_loop`: Decrements a value in d0 and loops while d1 is not negative, calling another routine and performing bit shifts and data transfers from a source pointer to a destination pointer.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band].adf:bootblock

- `check_library_function`: Calls a routine, then calls a function via a pointer at an offset from a value in a6, and returns zero if the result is zero or negative one otherwise.
- `process_byte_loop`: Saves registers to the stack, calls another routine, and then enters a loop that adds a byte from memory to a location, performs an eor operation, and decrements a counter until it reaches zero.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_global_and_hardware`: Calls two routines, modifies several values in a global structure and hardware registers, and then updates a global pointer and two other global values based on values from a table before returning.
- `toggle_bits_and_set_data_2`: Calls four routines, writes the constant 112 to an offset of 156 of a6, and returns.
- `initialize_memory_and_increment`: Loops until bit 6 of a byte at offset 2 of a6 is cleared, then writes several constants to a memory block starting at offset 64 of a6, increments a global variable, and calls two other routines if a comparison matches.
- `increment_global_value`: Checks if a word in a global variable is equal to negative one, then writes a constant to a memory location and increments a value at that location by two.
- `initialize_memory_structure`: Loops until bit 6 of a byte at an offset of a6 is cleared, then writes several constants and a calculated address or value to a memory block.
- `decompress_bitstream_data`: Performs a loop 40 times to decompress a bitstream into destination buffers, updating pointers and values based on a table and a bit test of a global variable.
- `clear_memory_range`: Sets all bytes in a memory range from a specific address to another address to zero using a loop.
- `copy_and_append_data`: Copies a block of data from one location to another and appends three specific values to the end of the destination block.
- `toggle_bits_and_set_data`: Copies a word from a global variable, toggles certain bits in a byte, and then writes several constants and a global variable value to a memory block.
- `parse_library_list`: Uses library calls to open a library, iterates through a list of structures to update global pointers or values based on certain identifiers, and then copies a block of memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_via_relative_address`: Performs a jump to another routine using a relative address calculation and then returns.
