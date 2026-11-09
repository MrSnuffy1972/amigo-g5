# Game 959 (Kick Off 2): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00959). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 308 of 308 named (0 author symbols), 308 of 308 described (9 by rule, 297 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 77 of 77 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4945 of 4945 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 81012 |
| code | 16736 |
| data | 802633 |
| padding | 739 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Kick Off 2.adf:bootblock | 1 | 54 | 3 |
| logged disk load from Kick Off 2.adf boot-block load | 1 | 4767 | 300 |
| logged disk load from Kick Off 2.adf boot-block load | 1 | 124 | 5 |

## Cases handled

- Kick Off 2/Kick Off 2.adf: custom-format (non-DOS) disk, 1726 non-empty blocks: matched against track-loaded segments
- Kick Off 2/Kick Off 2.adf: the boot block reads 110080 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- Kick Off 2/Kick Off 2.adf: the boot block reads 18432 bytes from disk offset 870400 to an address (found statically); recovered as a unit
- Kick Off 2/Kick Off 2.adf: 2 logged loads cover 128512 bytes of the image
- Kick Off 2/Kick Off 2.adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.46)
- Kick Off 2/Kick Off 2.adf: blocks 226-1699 (bytes 115712-870400) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.77)
- Kick Off 2/Kick Off 2.adf: blocks 1736-1759 (bytes 888832-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Kick Off 2.adf:bootblock

- `initialize_boot_sequence`: Pushes a pointer to the stack, calls a routine using a pointer from offset 4, and if d0 is not zero, writes values to hardware registers; otherwise, it uses a structure on the stack to call a routine, writes to hardware registers, and enters a loop that copies 18 longwords from one location to another before performing an indirect jump.
- `copy_boot_data`: Copies a block of longwords from a source at offset 49152 to a destination at offset 21248 using a loop controlled by a counter in d0, then performs a trap.
- `configure_dma_and_jump`: Sets the DMA control register to enable specific channels and then performs an indirect jump to a location stored in memory.

### logged disk load from Kick Off 2.adf boot-block load

- `setup_base_pointers`: Loads a base address into a4 by adding a constant to a value from a label and then jumps to another routine.
- `initialize_structure_fields`: Clears or sets various 16-bit and 32-bit fields at specific offsets within a structure pointed to by a4, populates two lists with sequential values, calls a subroutine twice using a pointer from a structure, and calls another routine before returning.
- `call_subroutine_twice`: Loads two different addresses into registers a2 and a3 using offsets from a4, calls a subroutine at an offset from a4 twice, and then returns.
- `initialize_hardware_and_data_tables`: Calls a routine to initialize a structure, then calls another routine to retrieve a value; if that value is non-negative, it calls a third routine with the value's high word and a fourth routine to process a list.
- `toggle_hardware_bit`: Sets bit 2 of a byte at the address in a0, calls another routine, clears bit 2 of the same byte, calls another routine, and then returns.
- `wait_for_flag_clear`: Polls a byte pointer for a specific bit to clear, using a decrementing counter loop.
- `search_pattern_with_counter`: Performs a pattern search in a stream pointed to by a3, checking for specific byte values and using a loop that decrements a counter in d2, while also calling another routine based on the results.
- `check_pointer_and_set_flag`: Repeatedly checks if a word value at a pointer is greater than -1.
- `loop_on_sign_bit`: Moves a 16-bit word from a pointer in a3 to d1 and loops if the result is positive.
- `compare_pointer_value`: Loops checking a value pointed to by a register against an input byte and other conditions, returning when they match or fail.
- `write_and_call`: Stores a byte from d0 into the memory location pointed to by a0, calls a subroutine, and then stores a byte from d1 into the same location.
- `loop_hardware_configuration`: Loops 128 times, writing specific byte values to offsets of a1 and setting a bit in a hardware register at offset e00 of a1.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_bit_and_decrement`: Sets bit 1 of the byte at the address in a0 and decrements d0.
- `toggle_bit_and_branch`: Calls another routine, clears a bit at a pointer, calls another routine, sets the same bit at the pointer, and then branches to another location.
- `loop_hardware_settings`: Contains a loop that calls a routine and branches back to itself if d0 is less than d1, otherwise it executes a sequence of routine calls and updates a 16-bit value in a4.
- `execute_indexed_function`: Calls a routine, updates a 16-bit value at a negative offset in a4 with the result, and then calls a function from a table using an index derived from that value.
- `process_state_transitions`: Performs several subroutine calls, updates a 16-bit value at offset -14230 of a structure in a4 with the result of a call, and branches to different routines based on whether that value is 10, 7, 8, or 9, or loops back to itself.
- `update_dynamic_array_element`: Calculates an index based on a 16-bit value from a structure, retrieves a value from a table at that index, and updates a specific element in a dynamic array.
- `check_and_call_routine`: Tests a 16-bit word at offset -13778 of a structure in a4 and calls another routine if the value is not zero, otherwise it branches to another location.
- `call_and_test_result`: Moves a pointer from a2 to a0, calls another routine, and then branches to another routine or an earlier location depending on whether the result in d0 is negative.
- `execute_specific_update`: Loads an address into a0 from offset -18334 of a structure in a4 and calls another routine if a 16-bit value at offset -13716 of a4 is non-zero, otherwise it branches to another location.
- `return_constant_value`: Loads the value 10 into a register and returns immediately.
- `jump_to_offset`: Loads d0 with 11 and jumps to an address calculated from an offset of a4.
- `execute_callback_loop`: Pushes two words onto the stack, calls a routine at an offset from a4, stores d0 at an offset of a4, and then enters a loop that calls two other routines and a third routine at a PC-relative offset, repeating the loop based on d7.
- `process_string_and_callbacks`: Iterates through a null-terminated string at offset -31434 of a4, and if the string is not empty, calculates an offset using d7, calls a routine at a pointer from a structure in a4, and then calls two other routines.
- `search_byte_in_memory`: Saves registers to the stack, loads a pointer into a0 from an offset of a4, and checks a byte at the location in a0; if the byte is not zero, it branches to another routine, otherwise it decrements d0 and branches to another routine.
- `call_search_routine`: Calls a routine and restores registers d0 through d7 and a0 through a6 from the stack before returning.
- `initialize_table_entries`: Calculates an address based on a value in d7 and a base address from a4, then writes four 32-bit values and one 16-bit value to that location.
- `copy_table_values`: Calculates a destination address using d7 and a base address from a4, then copies three 16-bit words from a table at an offset of a4 into that destination.
- `calculate_and_call_offset`: Saves registers to the stack, calculates a value by subtracting an offset of a4 from d7 and dividing by 24, calls another routine, and then restores the registers and returns.
- `clear_three_words`: Clears three 16-bit words at offsets -14816, -14818, and -15228 relative to a pointer in a4.
- `increment_and_clamp_counter`: Increments a 16-bit value at a specific offset in a structure and resets it to zero if it reaches 24.
- `load_pointer_from_table`: Loads a word from offset -30432 of a4, shifts it left by 2, and uses it as an index into a table at offset -30432 of a4 to load a new pointer into a2.
- `call_linked_list_modifier`: Saves A6 to the stack, loads an address from an offset of A4 into A6, calls another routine, restores A6, and returns.
- `modify_linked_list_element_value`: Traverses a linked list to find an element matching a 16-bit value, then modifies that element's value based on its type: if the type is 0 or 4, it performs a range check using two 16-bit values in d1 and d3; if the type is 6, it decrements the value; if the type is 2, it increments the value.
- `calculate_and_search_index`: Calculates an index by multiplying d4 by d2 and adding values from a table, then calls a routine to search a linked list and calls another routine if the result is not zero.
- `restore_registers_from_stack`: Loads eight 16-bit words from the stack into registers d0 through d7 and returns.
- `process_table_index`: Increments a counter in d4 and compares it to d3, calls another routine, and then checks a 16-bit value at an offset of 438 from a4; if certain conditions are met, it compares d0 with d5 and branches to one of four different routines based on whether d0 equals 0, 2, 4, or 6.
- `branch_on_decrement`: Decrements d6 by one and branches to an earlier instruction if the result is less than zero, otherwise it branches to a later instruction.
- `branch_on_comparison`: Increments d6 by one and branches to an earlier instruction if d6 is greater than or equal to d3, otherwise it branches to a later instruction.
- `update_table_index`: Shifts d7 left by one, loads a word from a table at an offset of a5 using the new d7, decrements that word, and then branches to an earlier instruction if the result is less than zero, otherwise it branches to a later instruction.
- `update_table_elements`: Shifts D7 left by one, loads a word from a table at offset D7 using A5 into D6, increments D6, loads a word from the same table into D0, calls another routine, stores D6 back into the table, calls another routine, and if D0 is positive, stores D0 into the table and calls another routine before branching back to the start of the routine.
- `execute_nested_calls`: Calls another routine, moves D6 to D4, calls another routine, and branches to another routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies d4 to d0, then returns.
- `search_list_update_coords`: Multiplies d4 by d2, adds d1, and calls a routine to search a linked list.
- `execute_conditional_callbacks`: Calls a routine using a pointer from a structure, stores the result, and if the result is not negative, calls two more routines with specific arguments before returning.
- `calculate_scaled_values`: Fetches a word from a table using a value derived from d0, calls another routine, performs a series of calculations involving division and multiplication to update two 32-bit values at offsets -20006 and -20002 of a4, and conditionally negates them based on a value at offset -18064 of a4.
- `update_table_entry_with_bounds_check`: Increments a 16-bit value at a negative offset of a structure in a4 by 10, then compares the result against specific thresholds to branch to different routines or update the value back into the structure.
- `update_table_entry_via_offset`: Calculates an index into a table using a value from a register, checks if that value is greater than 20, and if so, calls another routine with that value as an argument before updating the table entry with a new value loaded from a global variable.
- `parse_and_store_string_data`: Checks a character at a negative offset of a structure in a4, then calls another routine to retrieve a string.
- `branch_to_subroutine_2`: Calls a subroutine and then branches to an earlier instruction.
- `execute_stack_calls`: Pushes a6 to the stack, calls a subroutine using an offset from a4, adjusts the stack, pushes a word from an offset of a4 to the stack, calls another subroutine using an offset from a4, adjusts the stack, calls a third subroutine, and then branches to an earlier instruction.
- `call_offset_routines`: Calls a subroutine, pushes a word from an offset of a4 to the stack, calls another subroutine using an offset from a4, adjusts the stack, and then branches to an earlier instruction.
- `update_table_entry_via_index`: Executes several routines, calculates an index by shifting a value from a table, and if the value at that index is greater than 19, calls a routine using the value as a parameter before updating the table entry with a pointer from a5.
- `restore_registers_only`: Restores registers d0 through d7 and a0, a3, a5, and a6 from the stack and returns.
- `process_iterative_calls`: Performs several routine calls and updates a byte at a negative offset in a4, then enters two loops that execute a sequence of routine calls for 4 and 8 iterations respectively.
- `update_offset_value_2`: Loads a word from an offset of a4, adds 14 to it, calls another routine, subtracts 14 from the result, and stores the final value back into the same offset of a4.
- `call_two_subroutines`: Saves registers, calls a routine at offset -32488 of a4 with d0 set to 163, calls a routine at offset -32734 of a4 with d0 set to 0, and restores registers before returning.
- `process_player_data_list`: Iterates through a list of data entries, calculating a value for each entry using a multiplication and addition formula, and calls another routine for each processed item.
- `perform_sequence_of_calls`: Pushes several values onto the stack and calls four different routines using pointers from a structure before branching to another routine.
- `branch_after_subroutine_call`: Calls another routine and then performs a relative branch to a previous location.
- `execute_multiple_jumps_2`: Pushes a6 to the stack, calls two routines using offsets from a4, calls another routine, and then branches to another location.
- `set_register_and_branch`: Sets register d0 to 255 and branches to another location.
- `jump_to_offset_2`: Clears d0, restores registers d0 through d7 and a0 through a6, and then jumps to an offset of -32124 relative to a4.
- `store_word_to_offset`: Stores the word in d0 into a memory location at offset 392 of a4.
- `load_and_clear_word`: Loads a longword from offset -926 of a4 into d0, clears a 16-bit word at offset -13940 of a4, and returns.
- `execute_conditional_routines`: Calls a routine at offset -31948 of a4, then calls either a routine at offset -32244 or a routine at offset -32212 of a4 based on a zero check of a longword at offset -19118 of a4, and finally calls a routine at offset -32234 of a4.
- `blit_bitmapped_data`: Performs a bitwise block transfer of data from a source buffer to a destination buffer, supporting bit-shifts and masking.
- `update_offset_value`: Calls another routine, then increments a 16-bit value at offset -900 of a structure in a4 by either 8 or 6 depending on the value at offset -868 of that same structure.
- `call_subroutine_only`: Calls another routine and returns.
- `check_character_range`: Examines a byte in d0 to determine if it falls within specific ASCII ranges, excluding letters and certain symbols, and returns.
- `validate_character_byte`: Calls another routine, and if the result is not zero, checks if d0 is 32 or if d0 is 65535 before returning.
- `tiny_cmp_rts`: Small routine of 2 instructions that compares d0 with d0, then returns.
- `set_structure_values`: Checks a 16-bit value at offset -900 of a structure in a4 and writes two 16-bit values to offsets -14136 and -14138 based on whether that value is 0, 1, or something else.
- `process_string_with_nesting`: Iterates through a source string, copying characters to a destination while handling spaces and nested structures by calling another routine to validate characters.
- `update_and_iterate_values`: Calls a routine to update two 16-bit values in a structure, then iterates through a null-terminated string, calling another routine for each character and a final routine before returning.
- `swap_byte_with_structure_member`: If A2 is not zero, copies a byte from offset 28 of A2 to offset -13626 of A4, moves the value in D2 into offset 28 of A2, calls another routine, and then copies the byte from offset -13626 of A4 back into offset 28 of A2.
- `update_structure_from_data`: Updates a structure in a4 using values from a pointer in a2, including setting a word at offset -894 to 1 and updating values at offsets -900 and 2, then calls two routines.
- `swap_byte_via_buffer`: If a2 is not zero, it copies a byte from offset 28 of a2 to offset -8862 of a4, stores the value of d2 into offset 28 of a2, calls another routine, and then restores the original byte from the buffer back to offset 28 of a2.
- `process_nested_string_data`: Saves registers, checks if A2 is zero, copies a byte from offset 27 of A2 to offset -900 of A4, calls two routines, performs a division using a value from offset -14136 of A4 and a value from offset 12 of A2, calls another routine, and enters a loop that processes a string in A3 while performing arithmetic and calling routines based on the results and values in A4.
- `execute_nested_calls_2`: Saves registers, calls two routines using pointers from a4, calls another routine with d0, and if a2 is not equal to 32532, calls two more routines before returning.
- `call_search_linked_list`: Calls a routine that traverses a linked list starting from a pointer within a structure in a4, comparing a 16-bit value in d0 against the second word of each element.
- `update_global_state`: Calls another routine, writes 2 to a 16-bit value at offset -894 of a structure in a4, stores two 16-bit values from a2 into offsets -906 and -904 of a structure in a4, and then calls two more routines.
- `set_and_call_routine`: Stores the value 2 at the address pointed to by the offset -926 of a structure in a4, then calls another routine.
- `execute_four_updates`: Calls another routine, updates two 16-bit values in a structure in a4 using values from a2, and then calls another routine four times with modified values before calling a final routine.
- `process_list_and_sum`: Calls a routine, compares the result with a value at a negative offset in a4, and if they are not equal, saves registers to the stack, calls three routines, and returns.
- `allocate_and_initialize_object_list`: Validates specific data lengths in a structure, then enters a loop that allocates new memory blocks, populates them with data from a source buffer, and inserts them into a linked list.
- `traverse_linked_list`: Calls a routine via a pointer at an offset from a4, then traverses a linked list starting from a pointer at an offset from a4, calling another routine for each non-zero element, and finally calls a second routine via a pointer from a4.
- `process_list_node`: Loads a pointer from an offset from a4 into a2, calls a routine using a pointer from a4 with a2 on the stack, and if d0 is not negative, updates a2 with a value from the memory pointed to by the original a2.
- `find_matching_list_node`: Traverses a linked list starting from a pointer in a4 and checks if the node's values fall within specific ranges defined in a structure, then stores a result in a global variable.
- `load_and_branch`: Loads the value at offset 4 of a2 into a2 and branches to another routine.
- `store_word_to_offset_2`: Stores a 16-bit word from d0 into a 16-bit location at offset -9212 of a structure in a4 and returns.
- `search_linked_list`: Traverses a linked list starting from a pointer at offset -14176 of a structure in a4, comparing a 16-bit value in d0 against the second word of each element, until a null pointer or a match is found.
- `process_structure_updates`: Saves registers, calls a routine, and if a2 is not zero, it performs a series of conditional checks and updates to values at offsets -894 and -906 of a4 using data from a2 and calls several other routines including a loop.
- `update_structure_data`: Calls another routine, then if a2 is not zero, it stores a byte from a2 into another routine, clears a word at offset -894 of a structure in a4, stores two words from a2 into offsets -906 and -904 of a4, increments those two words, adds values from a2 to them and subtracts 2, calls another routine, and if a byte from a2 is not negative, calls another routine twice and another routine again.
- `process_conditional_logic`: Checks a 16-bit value at offset 440 of a structure in a4, calls another routine if that value is zero, and then enters a loop that calls another routine and potentially a second routine based on the result in d0.
- `save_registers_and_branch`: Saves several data and address registers onto the stack, subtracts the value of a register from itself, and performs a short branch to another routine.
- `parse_and_copy_string`: Copies a byte from a buffer into a structure at a negative offset in a4, executes several routines, and then enters a loop to process and potentially write characters into a string buffer.
- `compare_and_clear_byte`: Calls a routine, compares d0 against decimal 13, 127, and 8, and if d0 is 8 and d6 is zero, clears a byte at a negative offset from a5 and decrements d6 before branching back to an earlier instruction.
- `transfer_bytes_conditionally`: If d0 is less than 32 or d6 is greater than or equal to d7, it branches to another routine; otherwise, it stores d0 at the address in a5, stores 0 at the next address in a5, increments d6, and branches to another routine.
- `call_two_routines`: Calls two separate routines and then restores d1 through d7 and a0 through a6 from the stack before returning.
- `copy_name_up_to_dot`: Copies up to twelve characters from a source buffer in a5 into a destination buffer in a3, stopping if a NUL terminator or a period is encountered, and writes a NUL terminator at the end.
- `execute_loop_and_check_sum`: Saves registers, calls another routine, initializes D6 and D7, loads addresses into A5 and A3, calls two routines, runs a loop 31 times, compares D7 to 6, and if D7 is not less than 6, performs a series of additions and comparisons involving a table at address 21682 before returning.
- `restore_and_branch`: Restores d0, d1, d3, a0 from the stack and branches to another routine.
- `store_word_to_offset_3`: Stores a 16-bit word from register d7 into a memory location at a negative offset from a pointer in a4, then restores registers and returns.
- `compare_strings_with_wildcard`: Compares two strings pointed to by a5 and a6 using a loop that handles a wildcard character, returning zero or negative one based on the match result.
- `compare_and_branch`: Compares a byte at the address in a6 with a byte at the address in a5 and increments both pointers; if they are equal, it checks a byte at offset -1 of a6, and if that byte is not zero, it decrements d0 in a loop until d0 is zero, then sets d0 to zero and branches to another routine.
- `restore_registers_with_negative_one`: Sets register d0 to negative one and restores registers a5 and a6 from the stack before returning.
- `calculate_offset_and_call_2`: Saves registers, calculates an offset by multiplying d6 by 16 and adding it to a base address in a4, then calls three routines.
- `loop_specified_count`: Loads a 16-bit value from an offset from a4, decrements it, and if the result is non-negative, calls another routine within a loop that repeats d6 times before returning.
- `call_pair_twenty_four_times`: Calls two routines in each of 24 passes of a loop and returns.
- `reset_and_parse_string_data`: Clears a byte at a negative offset of a structure in a4 and resets a 16-bit value at a negative offset of the same structure.
- `compare_d0_values`: Compares d0 against 0, 4, and 6, and if none match, swaps the upper and lower bytes of d1, tests the sign of d1, and branches to either an earlier instruction or a later instruction.
- `calculate_offset_and_call`: Loads the address at offset -14580 of a4 into a3, multiplies d6 by 16 and stores the result in d0, calculates an address using a value from a structure in a4 and d0, calls another routine, and branches to another routine.
- `decrement_and_branch_on_sign`: Decrements D2 by one and branches to an earlier instruction if the result is not negative, otherwise it branches to another routine.
- `compare_and_call_incremented_values`: Compares a 16-bit value in d2, which is derived from d6 and incremented by 1, against a 16-bit value at offset -13550 of a structure in a4; if d2 is greater, it branches to another routine, otherwise it calls another routine twice with d0 set to d2 plus 10, then copies d2 to d6 and branches to another routine.
- `process_status_code_and_update_flag`: Calls two routines to retrieve values, then evaluates a returned 16-bit integer through a series of comparisons.
- `call_with_decremented_value`: Compares d5 to 1; if equal, it branches to another routine, otherwise it calls another routine with d0 set to d5, decrements d5 by 1, and calls another routine with d0 set to the new d5 before branching to another routine.
- `call_conditionally_on_d5_and_flag`: Compares d5 to 3; if equal, it branches to another routine.
- `call_and_branch`: Copies d5 to d0, calls another routine, and branches to another routine.
- `branch_on_d5_value`: Compares d5 against decimal 1, 2, and 3, and branches to different locations based on the result, otherwise it branches to another routine.
- `clear_memory_offsets`: Clears a byte at a negative offset from a4 and clears a 16-bit word at another negative offset from a4, then branches to another routine.
- `process_string_and_update_state`: Calls a routine using a pointer from a structure in a4, then clears a byte at a fixed offset from a4.
- `check_and_clear_byte`: Loads the address at offset -14100 of a4 into a2, and if the byte at that address is not zero, calls another routine and then branches to another routine.
- `clear_two_memory_locations`: Clears a byte at a negative offset from a4 and clears a 16-bit word at another negative offset from a4, then restores registers and returns.
- `process_dot_delimited_strings`: Searches for a period character in a string pointed to by a6, then searches for a period in a string pointed to by a2, null-terminates the second string at that period, and calls another routine using a pointer from a4.
- `sum_buffer_bytes`: Calls three routines, then enters a loop that iterates 971 times to sum bytes from a buffer starting at 61568 into d1, and if the resulting sum is not equal to -58, calls two more routines.
- `execute_two_calls`: Sets d0 to 6 and calls another routine, then calls another routine and returns.
- `execute_multiple_jumps`: Calls three routines, then calls another routine twice, and finally performs three jumps to different locations with d0 set to 17, 18, and 19 respectively.
- `loop_infinitely`: Performs an infinite branch to itself.
- `process_sum_and_stream`: Calls a routine to sum values, stores the result in a global variable, and if the result is positive, calls another routine to process a stream and then calls a third routine before returning.
- `sum_array_elements`: Calculates the sum of 18000 32-bit values from a table pointed to by a negative offset from a4, negates the result, and returns it in d0.
- `convert_lowercase_to_uppercase`: Checks if register d4 is between 97 and 122 inclusive; if so, adds 128 to it, otherwise it branches to a routine that calls another routine with d0 set to either 6 or 7.
- `unpack_packed_stream_backward`: Decodes a packed stream from its end toward its start, using a bit reader for flag bits and bit counts.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `initialize_data_tables`: Clears a 16-bit value in a global variable, fills a table with 65535 in a loop, and then processes a string to populate another table with offsets and counts.
- `calculate_offset_address`: Pushes d0 to the stack, shifts it left, and uses it to load a 16-bit value from a table into d0, then calculates an address using that value and returns.
- `jump_with_offset`: Loads a pointer from a negative offset from a4 into a2, restores d0 from the stack, and then calls another routine using a negative offset from a4 with d0 set to 21.
- `execute_sequence_of_routines`: Calls several routines, including one via a pointer stored on the stack, and finally stores the address of a structure offset into a global variable.
- `copy_memory_block`: Calculates a length based on the difference between two offsets from a4, then copies that number of bytes from a source at a negative offset from a4 to a destination at another negative offset from a4 using a loop.
- `copy_memory_block_2`: Calculates a count based on the difference between two offsets from a pointer in a6, then copies bytes from one location to another in a loop.
- `process_data_and_copy_memory`: Stores a pointer from a5 into a structure at offset -5442 of a4, calls several subroutines, updates a 16-bit value at offset -11646 of a4, and if certain conditions are met, performs a memory copy from a location in a2 to a location in a3 using a count derived from a value in a4.
- `branch_to_subroutine`: Loads the address at offset -6776 of a4 into a2, calls another routine, and then branches to an earlier location.
- `call_two_routines_2`: Loads 8 into d0, calls a subroutine, calls another subroutine, and then branches to an earlier instruction.
- `restore_registers`: Restores d0 through d7 and a0 through a6 from the stack and returns immediately.
- `loop_routine_calls`: Loads an address into a3, then enters a loop that runs 6 times, calling two routines in each iteration and incrementing d0.
- `execute_sequence_of_calls`: Saves registers multiple times, pushes an offset of A4 onto the stack, calls three different routines, and if D0 is not negative, calls another routine, then calls a fourth routine, a fifth routine, and a sixth routine before branching to another routine.
- `search_and_match_string_sequence`: Restores registers from the stack, calls another routine, and then enters a loop that compares a sequence of bytes from a source at a2 and a destination at a5; if a match is found and specific byte values are present at certain offsets, it pushes a pointer to the stack, calls another routine, and branches to another routine.
- `search_and_call_routine`: Decrements a counter in a loop, calls two routines to retrieve pointers, and then calls a third routine using a pointer from the stack.
- `clear_memory_and_set_values`: Pushes two values onto the stack and calls another routine with a 16-bit value; stores the result in a 16-bit word at offset -11196 of a structure in a4.
- `clear_and_copy_memory`: Calls another routine, clears a block of memory in a loop, and then copies several 16-bit values from one location to another.
- `update_table_values`: Copies a 16-bit value from one location to another and then performs several 16-bit transfers from a source table to various offsets in a structure.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `initialize_structure_data`: Clears several fields in a structure using zero or specific constants, then loads two 16-bit values from a table into the structure using an index derived from a global variable.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_audio_and_dma`: Sets a bit in a hardware register, updates a pointer in a4, and conditionally calls routines to configure audio structures and DMA settings based on the results of previous calls.
- `execute_repeated_calls`: Calls a routine through a pointer in a structure 50 times in a loop, provided the called routine returns a non-zero value.
- `reset_hardware_and_interrupts`: Loads an address into a2, stores it at a specific location, calls three routines, and returns.
- `execute_hardware_clears`: Calls four different routines in sequence and then returns.
- `check_interrupt_requests`: Tests bits 7, 0, 1, and 2 of the interrupt request register and the interrupt enable register sequentially, returning via an exception return if any bit is set.
- `disable_interrupts_and_dma`: Clears several fields in a structure and sets the DMA and interrupt enable registers to specific values.
- `check_condition_and_return`: Tests a 16-bit value at a negative offset from a4 and returns immediately if the value is zero; otherwise it returns.
- `configure_hardware_and_structure`: Sets specific values in hardware registers and, if a condition is met, populates several fields in a structure with specific 16-bit values and updates hardware registers again.
- `clear_hardware_and_structure`: Sets a hardware register to a specific value and clears three fields in a structure.
- `check_and_call_routine_2`: Tests a word at offset -6006 of a4; if it is not zero, it branches to another location, otherwise it calls another routine and then branches to an earlier location.
- `disable_interrupts_and_dma_2`: Clears a 16-bit value in a global variable, sets specific bits in the DMA and interrupt control registers, and branches to another location.
- `update_audio_sample_index`: Increments a 16-bit counter in a structure and updates a pointer to a sample table, then performs range checks on a value from that table to determine which sample or offset to use for audio playback.
- `store_d0_if_not_ffff`: Compares d0 to 65535; if equal, it branches to another routine, otherwise it stores d0 as a 16-bit word at offset -6044 of a structure in a4 and branches to another routine.
- `subtract_and_branch`: Subtracts 200 from d0, stores the result in a global variable, and branches to another location.
- `subtract_and_store`: Subtracts 130 from d0, stores the result in a global variable, and branches to another location.
- `play_audio_sample`: Performs a loop to calculate a pointer into a table, then configures audio hardware registers with values from a structure and a table before performing another loop.
- `play_audio_sample_indexed`: Calculates a pointer into a table using an index, configures audio hardware registers with values from that table and a structure, and performs two loops.
- `copy_audio_pointer`: Copies a 32-bit value from one location in a structure to another and then branches to another routine.
- `increment_global_short`: Increments a 16-bit value at a specific offset from a pointer in a4 and returns.
- `reset_hardware_parameters`: Clears four values in a structure pointed to by a4, sets a 16-bit value at an offset to 63, and writes to the interrupt and DMA control registers.
- `check_audio_flag`: Tests a 16-bit value at an offset of a structure in a6 and returns if it is zero, otherwise it branches to another routine.
- `initialize_audio_structure`: Sets two hardware registers, checks a global flag, and if zero, populates a structure with several values and updates the interrupt and DMA control registers.
- `clear_audio_settings`: Sets a hardware register to 2, clears two 32-bit values in a structure pointed to by a4, and clears a 16-bit value at an offset.
- `check_and_call_routine_audio`: Tests a word at offset -6010 of a4; if it is not zero, it branches to another location, otherwise it calls another routine and then branches to an earlier location.
- `configure_audio_hardware`: Clears a 16-bit value in a structure, writes values to the DMA control, interrupt enable, and interrupt period hardware registers, and then branches to another routine.
- `manage_audio_sample_index`: Increments a 16-bit counter in a structure and manages a pointer to a sample table, updating the pointer or adjusting the counter based on value ranges.
- `store_d0_if_not_ffff_2`: Compares d0 to 65535; if equal, it branches to another routine, otherwise it stores d0 as a 16-bit word at offset -6056 of a structure in a4 and branches to another routine.
- `update_audio_offset_high`: Subtracts 200 from a 16-bit value in d0, stores the result in a structure, and branches to another routine.
- `update_audio_offset_low`: Subtracts 130 from a 16-bit value in d0, stores the result in a structure, and branches to another routine.
- `load_audio_sample`: Calculates a pointer into a table using a value from a structure, updates audio hardware registers with the address and properties, and performs a loop to set a register.
- `update_audio_sample`: Calculates a pointer into a table using a decremented value, updates audio hardware registers with the address and properties, and copies a 16-bit value from the table into a structure.
- `update_audio_sample_pointer`: Copies a 32-bit value from one location in a structure to another and then calls update_audio_sample.
- `increment_global_word`: Increments a 16-bit value located at a fixed offset from a pointer held in a4 and returns.
- `disable_dma_and_interrupts`: Clears four fields in a structure pointed to by a4 and sets the DMA control and interrupt enable registers to disable all DMA channels and interrupts.
- `test_and_branch_on_global_word`: Tests a 16-bit word at an offset of 5986 from the pointer in a4 and branches to another routine if the value is not zero; otherwise it returns.
- `configure_interrupt_and_reset_state`: Sets the interrupt mask and custom interrupt period, then checks a flag in a global structure; if the flag is zero, it updates several fields in that structure using a register and modifies the interrupt period and custom interrupt register.
- `clear_audio_registers`: Sets a hardware register to 4, clears two 32-bit values in a structure pointed to by a4, and clears a 16-bit value at an offset.
- `branch_if_non_zero`: Tests a 16-bit word at an offset of 5982 from a pointer in a4 and calls another routine if the value is non-zero, otherwise it branches to another location.
- `configure_display_and_interrupts`: Clears a 16-bit word in a global variable, sets the vertical blanking period and the copper interrupt period to 512, sets the interrupt mask to 4, and branches to another routine.
- `update_audio_sample_index_2`: Increments a 16-bit counter in a structure and manages a pointer to a sample table, performing bounds checking on a value in register d0 to select a new sample, update audio hardware registers, and execute a delay loop.
- `store_d0_if_not_ffff_3`: Compares d0 to 65535; if equal, it branches to another routine, otherwise it stores d0 as a 16-bit word at offset -6076 of a structure in a4 and branches to another routine.
- `subtract_and_store_word`: Subtracts 200 from a 16-bit value in register d0, stores the result into a global variable at an offset from a4, and branches to another routine.
- `subtract_and_store_word_2`: Subtracts 130 from a 16-bit value in register d0, stores the result into a global variable at an offset from a4, and branches to another routine.
- `configure_audio_channel_two`: Calculates an offset into a table using a value from a global variable, updates the audio channel two pointer, period, and frame registers, and performs two loops to write to hardware registers.
- `configure_audio_sample_playback`: Performs a delay loop, selects a sample from a table using an index derived from a pointer, and updates the audio hardware with a new DMA address, period, frequency, and volume.
- `copy_audio_pointer_2`: Copies a 32-bit value from an offset of 6956 in a structure pointed to by a6 to an offset of 6952 in the same structure, then branches to another routine.
- `increment_global_counter`: Increments a 16-bit value stored at an offset from a global pointer and returns.
- `clear_audio_parameters`: Clears three 32-bit or 16-bit values at offsets from a global pointer and writes values to two hardware registers before returning.
- `check_audio_status`: Tests a 16-bit value at an offset of 6982 from a structure in a4 and branches to another routine if the value is zero, otherwise it returns.
- `initialize_audio_structure_2`: Checks a 16-bit value at an offset from a global pointer; if it is zero, it copies a pointer to two locations, clears several values, and writes to three hardware registers.
- `reset_audio_state`: Writes a value to a hardware register and clears three values at offsets from a global pointer.
- `test_and_call`: Tests a 16-bit word at an offset of 5954 from a pointer in a4 and calls another routine if the value is non-zero, otherwise it branches to another location.
- `clear_audio_settings_2`: Clears a 16-bit value at offset 6982 of a structure in a4, writes 1024 to the DMA control register, writes 1024 to the interrupt enable register, writes 8 to the interrupt mask register, and branches to another routine.
- `update_audio_sample_index_3`: Increments a 16-bit index in a structure and uses the resulting value to fetch a pointer from a table.
- `branch_on_word_equality`: Compares a word in d0 to 65535; if equal, it branches to another routine, otherwise it stores d0 into a word at offset -6000 of the structure in a4 and branches to another routine.
- `update_audio_offset_high_2`: Subtracts 200 from d0, stores the result in a 16-bit word at offset 6994 of a structure in a4, and branches to another routine.
- `update_audio_offset_low_2`: Subtracts 130 from d0, stores the result in a 16-bit word at offset 6976 of a structure in a4, and branches to another routine.
- `configure_audio_channel_three`: Calculates an offset into a table using a value from a global variable, updates the audio channel three pointer, period, and frame registers, and performs two loops to write to hardware registers.
- `configure_audio_sample_playback_2`: Updates the audio hardware by loading a pointer, period, and volume from a table into the DMA and audio registers, then performs a loop to clear a value before re-enabling DMA and clearing an interrupt request.
- `copy_audio_pointer_alt`: Copies a 32-bit value from an offset of 6956 in a structure pointed to by a4 to an offset of 6952 in the same structure, then branches to another routine.
- `increment_global_word_2`: Increments a 16-bit value stored at a fixed offset relative to a pointer in a4 and returns.
- `initialize_audio_channel_structures`: Checks four consecutive 16-bit fields in a structure pointed to by a4; if a field is zero, it populates a corresponding set of audio parameters including two pointers, several zeroed values, and two constant 16-bit values, then updates a global variable with a specific bitmask and sets the field to one.
- `clear_short_values`: Clears two 16-bit values at offsets from a global pointer and writes a value to another offset from the same pointer.
- `convert_ascii_range`: Compares a word in d2 to 65; if it is less than 65, it branches to another routine, otherwise it checks if d2 is greater than 90; if so, it branches to another routine, otherwise it subtracts 65 from d2 and returns.
- `adjust_word_value`: Checks if a word in d2 is between 48 and 57, and if so, subtracts 22 from it before returning.
- `count_matching_bytes`: Increments a counter in d4 for every byte in a memory block pointed to by a1 that matches the value in d2, then stores the final count in d2 and returns.
- `tiny_move_addi_rts`: Small routine of 3 instructions that copies d4 to d2, then adds 36 to d2, then returns.
- `branch_on_masked_value`: Saves registers to the stack, loads a byte from an offset of negative 6004 from a pointer in a4, masks the lower 4 bits, and then branches to different locations based on whether the result is 0, 3, or 4, or jumps to an offset of negative 32124 from a4 if none match.
- `update_structure_field`: Checks if the lower 4 bits of a 16-bit word at offset 5944 from a pointer in a4 are zero; if not, it calls another routine, then updates a 16-bit word at offset 5948 from the same pointer by adding 6 and returns.
- `execute_conditional_loop`: Calculates a long value using d2, d3, d0, and a pointer in a4, then checks a bit in the result; if the bit is zero, it enters a loop that runs up to three times, calling another routine during each iteration.
- `configure_blitter_registers`: Calculates a long value using d2, d3, d0, and a pointer in a4, then checks a bit in the result; if the bit is zero, it enters a loop checking a bit in a hardware register, then writes several values to the hardware register structure at a fixed address, including bit 15 of a register at offset 154, and sets a global variable.
- `configure_blitter_hardware`: Sets several values in the hardware register structure pointed to by a5, including bit 15 of a register at offset 154, and performs a bitwise transformation on d3 to store a value at offset 64.
- `write_hardware_ports`: Writes the values 514 and 32768 to offsets 88 and 154 of the structure pointed to by a5 and returns.
- `jump_to_another_routine`: Performs a short branch to another routine.
- `restore_registers_2`: Restores d0 through d7 and a0 through a6 from the stack and returns.
- `jump_indirect`: Loads 26 into d0 and performs an indirect jump to an address calculated from a4.
- `store_byte_and_update_pointer`: Stores a byte from d0 at the address in a3, increments a pointer at a specific offset from a4 to point to the new a3 value, restores registers, and returns.
- `loop_and_extend_byte`: Pushes a 16-bit value from d0 to the stack, loads a byte from a positive offset of a6 into d0, and if d0 is zero, extends it to a 16-bit value, calls a routine, and branches back to an earlier instruction.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies (a7)+ to d0, then returns.
- `divide_and_check_threshold`: Performs a 16-bit division of d0 by 10000, then compares the result against a value derived from a byte at a negative offset in a4 to determine if a specific constant should be loaded into d0.
- `calculate_and_divide`: Sets d1 to zero, adds 48 to d0, calls a routine, decrements d1, extends d2 to a 32-bit value, divides d2 by 10, and if the result is not zero, branches to an earlier instruction.
- `fetch_table_values`: Uses a value from a global variable to index into two different tables to load new values into d0 and a6, then restores registers and returns.
- `execute_conditional_subroutines_2`: Stores a pointer from a5 into a global variable, then performs a series of conditional calls to other routines based on the return value of the first call.
- `return_and_jump_indirect`: Returns immediately, otherwise it loads the value 29 into register d0 and performs an indirect jump using an offset from the address in register a4.
- `clear_two_words`: Clears two 16-bit words at offsets -5690 and -5686 relative to a4, then calls another routine.
- `initialize_data_structures`: Clears a word at offset -5450 relative to a4, and if a word at offset -908 is non-zero, calls another routine to set up two structures pointed to by a2 and a3.
- `initialize_structures`: If two specific conditions are met, it clears several fields in two structures pointed to by a2 and a3, sets specific 16-bit values at various offsets, and clears a global variable.
- `setup_blitter_copies`: If a word at offset -5450 relative to a4 is non-zero, it calls another routine twice using pointers derived from offsets -5456 and -5452 of a4.
- `setup_blitter_copy`: If a pointer in a3 is nonzero, it waits for bit 14 of a hardware register to be cleared, then programs the blitter, DMA, and interrupt registers using data from the structure in a3 before storing the pointer in a global variable.
- `process_data_buffer_stream`: Checks a condition in a structure and, if met, performs a series of stack-based calls to manipulate data.
- `toggle_bit_and_loop_2`: Sets bit 2 of a byte pointed to by a0, calls another routine, clears bit 2 of the same byte, calls another routine, and then checks if the difference between d5 and d6 is within a specific range before calling another routine and returning if d1 is negative.
- `wait_for_bit_and_value`: Polls a bit in a byte pointed to by a register using a decrementing loop, then waits for a value at a memory location to become non-negative before polling the same bit again with a second decrementing loop.
- `search_byte_pattern_in_stream`: Waits for the top bit of a word at a pointer to be set, then compares the following bytes against the character D and a second fixed byte value, with a counter that restarts the search.
- `loop_on_positive_word`: Loads a 16-bit word from the address in a3 into d1 and branches back to the same instruction if d1 is positive.
- `compare_pointer_values`: Compares a byte in a register against four consecutive 16-bit values pointed to by a3, returning if any value is negative or if any value does not match the target byte or the preceding value in the sequence.
- `write_and_call_2`: Stores the byte from d0 into the memory location pointed to by a0, calls another routine, and then stores the byte from d1 into the same location.
- `initialize_hardware_loop`: Loops while decrementing d2, writing specific values to offsets of a structure in a1 and checking a bit at offset 3584 of that structure.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `set_bit_and_decrement_2`: Sets bit 0 of the byte at the location pointed to by a0 and decrements d0 by one.
- `toggle_bit_and_loop`: Calls another routine, toggles bit 0 of the byte at a0, calls another routine again, toggles bit 0 of the byte at a0, and then branches back to an earlier instruction in this routine.
- `process_data_loop`: Compares d0 and d1 in a loop, performing various stack operations, calls several routines, and manipulates a byte at the location pointed to by a0 before performing a loop that writes values to a structure in a1.
- `call_function_with_stack`: If d6 is not greater than zero, pushes several values onto the stack including a constant and a value from a global variable, calls another routine, and returns d0.
- `process_conditional_data_update`: Calls a routine and stores the result in a structure, then if the result is non-negative, it calls three other routines, including one with a calculated address derived from a table and a pointer in a5.
- `process_alternative_data_update`: Calls a routine and stores the result in a structure, then if the result is non-negative, it calls three other routines, including one with a calculated address derived from a different table offset and a pointer in a5.
- `process_data_structure`: Calls a routine to get a value, stores it in a global variable, and if positive, performs several calls including one that uses a calculated offset from a structure in a5 to access data and another that copies bytes from a source to a destination using a loop.
- `execute_sequential_updates`: Saves a pointer from a5 into a structure at offset -5442 of a4, performs several subroutine calls with specific values in d0 and d1, and if a 16-bit value at offset -908 of a4 is not zero, calls another subroutine before returning.
- `execute_multiple_calls`: If a value at an offset of a4 is not zero, calls three different routines using values from a global table and a global variable, updating a global variable with the results.
- `update_structure_fields`: Compares a word from an offset of a4 with another word at a different offset in a4; if they differ, it updates the second word with the first, shifts the value left by one, and calls a subroutine.
- `clear_status_fields`: Clears several fields in a structure at offsets -3370, -3354, -3348, -3350, and -3342 relative to a4, and if a word at offset -20254 is non-zero, calls another routine and sets a long value at offset -3342 to one.
- `update_data_list`: If a condition involving a4 and d1 is met, it updates a structure in a4 by copying three 16-bit words from a6 into a new location and decrements a counter, otherwise it calls another routine.
- `dispatch_conditional_subroutines`: Checks a 16-bit value at offset -908 of a structure in a4; if non-zero, it stores a constant in a structure offset -5322, pushes two values onto the stack, calls a routine, and then performs a series of conditional calls and an indirect jump based on the results and register values.
- `update_constant_values`: Loads a pointer from an offset of a4 into a2, increments a word from that pointer in d0, writes two constant words to offsets of a4, loads another pointer into a3, compares a2 and a3, and if a2 is less than a3, writes a third constant word to an offset of a4 before branching to an earlier instruction.
- `write_constant_and_call_subroutines`: Writes a 16-bit constant to an offset of negative 5944 from a pointer in a4, calls a subroutine, sets d0 to 32, calls another subroutine, loads the address of a value at offset 54 of a2 into a6, calls a third subroutine, and returns.
- `clear_global_long`: Clears a 32-bit value at an offset of a4 and returns.
- `modify_address_table`: Loads two 16-bit words from a4, adds d0 to a value at a negative offset in a4, and either clears that offset or branches based on the sign of the result.
- `update_blitter_address_table_2`: Calculates a new base address by scaling a value from a structure, compares it against a stored limit, and updates a table of three 32-bit addresses.
- `recalculate_blitter_address_table`: Calculates a new base address based on a value in a structure, compares it against a stored limit, and updates a table of three 32-bit addresses with derived values.
- `update_blitter_address_table`: Decrements three values in a structure, calls a routine to calculate a base address, and then stores three derived 32-bit addresses into a table.
- `configure_blitter_parameters`: Adjusts a value at an offset of a4, calls another routine, and then performs several calculations involving bitwise operations and multiplication to write multiple 16-bit values to various offsets of the structure in a4.
- `set_structure_values_2`: Writes a byte to offset -20546 of a4, a byte from d2 to offset -20542 of a4, and a byte to offset -20543 of a4; it then compares a word at offset -5240 of a4 to 792 and branches accordingly.
- `validate_and_set_dma`: Checks if two values derived from subtracting global variables from constants are within specific ranges, and if so, writes a value to the DMA control register and calls another routine.
- `disable_dma_and_clear_registers`: Sets bit 5 of the DMA control register and clears six specific custom chip registers, then returns.
- `synchronize_game_state_and_hardware`: Checks if a value in a global structure has changed, and if so, executes a series of routines to update hardware state, manipulate a data structure via several calls, and perform bitwise operations on values stored at specific offsets in that structure.
- `toggle_bit_and_validate_range`: Sets bit 2 of a byte pointed to by a0, calls another routine, clears bit 2, and calls another routine.
- `wait_for_flag_clear_2`: Calls a routine, then clears bit 4 of a byte at a pointer in a2 and enters a loop that checks if that bit is set, repeating this process while decrementing a counter, and then performs a similar check on a word at a pointer in a3.
- `search_byte_pattern_in_stream_2`: Searches a data stream pointed to by a3 for a specific sequence of bytes, specifically looking for the character 'D' followed by the byte value 137, using a counter to limit the search.
- `loop_on_positive_word_2`: Loops while the word at the location pointed to by a3 is positive.
- `compare_multiple_pointer_values`: Compares a byte in d1 against four consecutive signed words pointed to by a3, returning if any value is negative or if any value fails to match its corresponding byte in d1, d2, d3, or d4.
- `transfer_bytes_and_call`: Stores a byte from register d0 to the address in a0, calls another routine, and then stores a byte from register d1 to the address in a0.
- `initialize_hardware_loop_2`: Sets d2 to 120 and enters a loop that writes specific bytes to offsets 224, 1024, and 1280 of the structure in a1, decrementing d2 until it reaches zero.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `set_bit_and_decrement_3`: Sets bit 0 of the byte at the address in a0 and decrements the 16-bit value in d0 by 1.
- `toggle_bit_and_branch_2`: Calls a routine, clears bit 0 of a byte at a pointer in a0, calls the same routine again, sets bit 0 of the byte at a0, and then branches to another routine.
- `update_hardware_state`: Enters a loop comparing d0 and d1, calling several routines that manipulate a structure in a5 and a byte at a0, and performs calculations involving multiplication and addition to update hardware registers.
- `execute_conditional_subroutines`: If a 16-bit value at an offset in a4 is non-zero, it calls three different routines using values from the stack and a pointer in a4, storing the results in a structure at an offset in a4.
- `initialize_and_populate_table`: Sets a 16-bit value to all ones and clears a 16-bit value at a negative offset in a4, then calls two routines, updates several pointers in a4, and enters a nested loop that populates a table starting at a5 with incrementing values.
- `store_split_word`: Calculates an offset by shifting d0 left by 3, then stores the 16-bit value from a0 into two consecutive 16-bit locations at that offset within a structure at a4.
- `store_multiple_pointers`: Calls another routine six times, each time passing a different address relative to a4 in a0 and a different constant in d0.
- `update_data_tables`: Calculates a value from d1 and d2, writes several bytes to a structure at an offset in a4, and then copies these values to five different locations in a structure at a4 at fixed offsets.
- `initialize_graphics_and_cop_tables`: Calculates dimensions for a display area, clears two memory blocks using loops, and populates several tables with values and pointers.
- `setup_blt_dma`: Checks a bit in a hardware register, configures several Blitter registers, and enters a loop that calculates an offset to write Blitter parameters to hardware registers.
- `configure_blt_with_offset`: Configures Blitter registers and enters a loop that calculates a destination address and writes Blitter parameters to hardware registers using a value from a table.
- `process_input_state`: Calls a function through a global pointer, updates that pointer with a value derived from bitwise operations on a memory location, and performs multiple lookups in a table based on bits from hardware registers to determine a final value.

### logged disk load from Kick Off 2.adf boot-block load

- `write_constant_long`: Calculates an address using a constant offset from the program counter and stores the long value a fixed address$ at that location, then calls another routine before returning.
- `apply_bitwise_or`: Performs a series of bitwise OR operations on register d0 using several immediate byte values.
- `process_bit_shifts`: Copies a block of memory, then enters a loop that right-shifts a long value in d0 and calls another routine multiple times based on the remaining bits, eventually storing a byte into a destination pointed to by a2.
- `shift_and_store_word`: Loads a longword from the address in a0, stores its byte component in d7, writes d7 to a word at the location in a5, sets the carry flag, and performs a logical right rotation on d0 before returning.
- `count_leading_zeros`: Enters a loop that right-shifts a long value in d0 and decrements a counter in d1, calling another routine when d0 reaches zero.
