# Game 868 (amazing spider-man, the (1990)(empire software)[h dragon inc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00868). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 350 of 350 named (0 author symbols), 350 of 350 described (20 by rule, 284 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 138 of 138 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7793 of 7818 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 280428 |
| code | 27462 |
| data | 264363 |
| padding | 5795 |
| unused | 323072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:ReadMe | 1 | 47 | 1 |
| Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:Exit | 2 | 117 | 2 |
| Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:Boot | 1 | 43 | 1 |
| Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:Comic | 5 | 848 | 58 |
| Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:TRIO | 2 | 228 | 11 |
| Music | 2 | 396 | 35 |
| Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:Loader | 1 | 68 | 1 |
| Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:Spidey | 8 | 6061 | 240 |
| Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:bootblock | 1 | 10 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:ReadMe

- `manage_library_resources`: Calls a routine to find a task, then checks a field at offset 172 of a structure; if zero, it calls another routine to update a global variable, performs several library calls to manipulate memory or messages, and finally calls two more routines before returning.

### Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:Exit

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_register_to_register`: Copies the contents of d1 to d0 and then branches to another routine.

### Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:Boot

- `initialize_system_state`: Calls a routine to find a task, then if successful, calls several other routines to manage memory and library state, potentially copying a value from one structure to another, and finally returns a value.

### Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:Comic

- `jump_to_entry`: Performs an unconditional jump to a fixed address.
- `initialize_handler_structures_3`: Checks if d0 equals a specific constant, then copies a pointer from a0 and a1 to a table, calls a routine using a pointer from a global variable, and calls another routine using a pointer from a global variable.
- `release_library_resources_2`: Calls another routine, then if a global variable is non-zero, calls a routine via a pointer; if another global variable is non-zero, calls two library routines and a third routine via a pointer, then returns a value from a routine into a register.
- `initialize_handler_structures_2`: Uses a pointer from a global variable to call a routine, and if the return value is non-zero, sets a field in that returned structure to a maximum value.
- `release_library_resources`: Checks if a global byte is non-zero, then calls a routine to unload a segment, clears that byte, and calls two other routines.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `increment_global_counters`: Checks a global word, calls two routines, and increments two consecutive global words by one before returning zero.
- `clear_and_test_global_word`: Clears a 16-bit word in a global variable and returns immediately if the value is zero; otherwise, it enters an infinite loop.
- `allocate_memory_conditionally`: Calls a routine to allocate memory of size 65538 and returns the result, otherwise it jumps to another location.
- `initialize_attribute_table_and_copy_data`: Calls a routine via a pointer, then if a certain global pointer is non-zero, copies a sequence of 137 words from a source table to a destination, otherwise it copies a single word from that pointer's target to a global variable and calls another routine.
- `rebuild_structure_list`: Calls a routine with three different size parameters, and if a global word is non-zero, it calls a routine to allocate memory, copies a sequence of words, and updates two global pointers.
- `execute_conditional_jump`: If d1 is not zero, it calls a routine via an indirect jump through a library function; otherwise, it returns.
- `swap_global_pointers`: Swaps the values of two global variables, sets a byte at a global variable to zero, and sets a word at another global variable to one.
- `populate_attribute_table`: Checks if a global word is non-zero, then clears it and enters a loop that writes a sequence of 4 word-sized values (derived from a global longword and an incrementing offset) into a table, before updating two pointers in a structure to point to the start of that table and returns.
- `copy_word_sequence`: Copies 16 words from a source pointed to by a0 to a destination starting at a global variable, where each word is also written to an offset of 32 from the destination pointer, using a loop.
- `set_handler_data`: The routine checks a pointer argument and, if non-zero, calls a function obtained from a global variable table.
- `reset_object_flags`: The routine clears a byte at an offset in a structure pointed to by a parameter and sets two 32-bit fields to all ones, then calls another routine via an indirect pointer from a global variable and returns its result, or returns a constant if the parameter is zero.
- `update_file_size_and_seek`: Checks a file or resource size via a function pointer, then if the size is valid, it resets a file pointer or seeks to a new position, potentially calling another routine to check or update a size-related counter or pointer, and finally returns a negative value or a modified register or a constant zero.
- `process_conditional_call`: If a2 is not zero, it calls a routine and returns a value of negative one; otherwise, it calls another routine and returns a value of negative one.
- `call_two_subroutines`: Calls two subroutines and then returns.
- `initialize_global_pointers`: Sets a word at a global address to a constant, calls several routines, and swaps two global pointers before setting a global word to zero and another to one.
- `copy_compressed_data_blocks_3`: Pushes three values onto the stack and calls a routine via a pointer.
- `process_data_sequence`: Initializes a counter to 4 and loads a pointer from a table, then enters a loop that calls two routines, performs several comparisons on a value in d0, calls a third routine, and calls a fourth routine via a pointer before decrementing the counter.
- `apply_masked_updates`: Saves d6 to the stack, calls another routine, and then enters a loop that iterates 18 times, calling several routines with different values in d3, and finally calls another routine before restoring d6 and returning.
- `copy_and_or_data`: Loads a pointer from a global variable, adds an offset, and loops a certain number of times to copy or or a value into a destination structure, then calls a routine and sets d7 before branching.
- `copy_and_or_data_with_words`: Loads a pointer from a global variable, adds an offset, and loops a certain number of times to copy bytes and words between structures while or-ing a value, then calls a routine and sets d7 before branching.
- `copy_and_or_data_limited`: Loads a pointer from a global variable, adds an offset, and loops a certain number of times to copy bytes and or-ing a value into a destination structure, then calls a routine and sets d7 before branching.
- `update_word_sequence`: Uses a loop to copy words from a source pointer and or a value into those words at certain offsets, then calls another routine and performs a loop that calls other routines and checks a condition.
- `execute_and_set_counter`: Calls a routine, sets d7 to two, and branches to another routine.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_32_bit_accumulator`: Adds a 16-bit value from a memory location to a 32-bit value stored across two memory locations, then updates those locations with the result.
- `loop_and_test_condition`: Runs a loop using a counter in d7, calling two different routines and checking a condition based on the zero and negative flags before returning.
- `toggle_status_flags`: Reads two bytes from a memory location, swaps them, and then uses bit testing to return or set various longword constants or clear a global word.
- `load_source_pointer`: Loads a longword from a global variable into a0.
- `copy_data_block`: Copies 2000 words from the source pointed to by a0 to the destination pointed to by a1 using a loop.
- `test_register_bit`: Clears the lower byte of d0 and then tests bit 0 of d0.
- `set_register_bytes`: Sets the lower byte of d0 to any value, then sets the lower byte of d0 to zero three times.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_register_byte`: Clears the lower byte of d0.
- `clear_d0_register`: Clears the lower byte of the d0 register.
- `clear_d0_register_2`: Clears the least significant byte of the d0 register.
- `test_and_call_routines`: Calls two different routines and checks a byte at a global address; if the byte is not negative, it calls a third routine before returning.
- `fetch_and_reset_global_word`: Checks if a global word is set to a specific value; if not, it calls another routine, processes the result by swapping bytes and combining it with the global word, and then resets the global word to a specific value and clears a following word before returning.
- `set_global_values`: Sets a 16-bit value to negative one at a global address and clears a 16-bit value at another global address before returning.
- `initialize_handler_structures`: Updates several global pointers with the return values of two subroutines, and if certain conditions are met, calls a routine via a pointer, modifies a structure at an offset, and calls another routine via a pointer before jumping to a third routine.
- `process_and_free_resources`: Checks if a global pointer is non-null and if a field at offset 20 is non-null, then sets a field at offset 40, calls a routine, and calls another routine.
- `compare_and_branch_on_byte`: Compares a byte from a structure in a0 to a constant, and if it matches, loads a word from a table and checks a bit before returning or clearing d0.
- `tiny_move_rts`: Small routine of 2 instructions that copies a0 to d0, then returns.
- `update_global_values`: Checks if a certain bit is set in a value from a structure, and if a global word is zero, writes a value and a bitmask to a global table before returning.
- `compare_and_store_values`: Compares a byte in d1 against two different constants and, if a match is found, branches to other routines; otherwise, it stores d1 and d0 into global memory locations and then either loads a constant into d1 or clears d0 before returning.
- `set_view_and_return`: The routine writes a constant to a global variable, then calls another routine via a table of function pointers, and returns the value received in register A0.
- `set_structure_pointers`: Clears a 16-bit word at a global variable, then stores the value zero in two different offsets of 50 and 54 of a structure pointed to by a global variable.
- `calculate_offset_address`: Adds two 16-bit values from a structure in a0 to d0 and d1, multiplies d1 by a 1s-bit value from a pointer in a1, and returns a pointer calculated by adding the results to a base address.
- `copy_compressed_data_blocks`: Copies data from a source buffer to a destination buffer using a nested loop structure that handles variable-length data segments.
- `update_button_status`: Reads a 16-bit word from offset 12 of a structure in a0, performs bitwise transformations on it, checks bit 7 of the CIAA Port A register, and sets various bits in a byte before storing that byte and its bitwise OR with itself into two consecutive global byte locations.
- `clear_data_register_d0_2`: Performs a logical OR operation on register d0 with zero, effectively clearing the register's value.
- `allocate_and_initialize_structure`: Calls several functions to allocate memory and signals, then populates a structure with data from a global variable and other sources.
- `free_message_port`: Validates a pointer, invokes cleanup routines, sets specific bytes to zero or maximum values, and frees associated memory and signals.

### Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:TRIO

- `initialize_resource_table`: Verifies the presence of required libraries by checking for specific magic strings in the library headers and parses a resource table.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Music

- `branch_to_offset`: Performs a branch to an offset.
- `set_global_byte_and_call_handler`: Sets a global variable to a constant value, loads a pointer into a register, and calls a routine with a constant value in d0 before returning.
- `set_global_byte`: Stores a byte (set to any value) into a global variable and returns.
- `check_and_return_zero`: Loads a word from a memory location, and if it is zero, branches to a different location; otherwise, it clears d0 and returns.
- `update_instruction_pointer`: Saves all registers to the stack, loads a zeroed pointer into a0, and checks if a byte at an offset from a pointer in a4 is certain values; if the byte is within a certain range, it clears that byte and branches to another location.
- `reset_audio_channels`: Loads several addresses into a6 and a5, then calls a routine that writes zero to certain memory locations and increments a counter in a structure, repeating this for several different audio channel registers.
- `check_value_and_branch`: Clears d0, loads a byte from a structure in a4, divides it by a constant, and branches or masks bits based on the result.
- `load_masked_byte`: Loads a byte from an offset of a pointer in a6, masks the lower four bits, and branches to another location.
- `copy_word_to_structure`: Reads a 16-bit word from offset 10 of a structure in a6 and stores it into a structure at offset 6 using a pointer in a5, then branches to another routine.
- `iterate_and_load_data`: Shifts a word in d0, loads a word from a table using a pointer in a0, and loops a certain number of times while comparing a value from the table against a value in d1.
- `store_word_to_structure`: Stores a 16-bit value from d2 into a structure at offset 6 using a pointer in a5 and returns.
- `process_music_instruction`: Sets up several pointers, extracts an opcode and index from a pointer-based structure, and then calls a routine that updates audio parameters and calls another routine that updates audio parameters for several different channels before branching to another routine.
- `process_music_instruction_opcode`: Extracts an opcode and index from a pointer-based structure, then updates a pointer to a table of instruction data, updates a pointer to a new instruction, and modifies a global pointer or a value in a structure based on the decoded opcode.
- `dispatch_instruction_opcode_logic`: Updates a pointer in a structure, copies a word from a table, and checks a masked byte from a structure to decide whether to call a routine or modify a byte or a pointer in a structure.
- `dispatch_instruction_opcode_2`: Updates a hardware register, copies a pointer and a word, and writes a masked word to a structure before branching to another routine.
- `update_audio_parameters`: Updates several audio registers or pointers with values from a table, increments a counter in a structure, and resets that counter or a phase byte if certain conditions or limits are met.
- `modify_structure_fields`: Masks a word from a structure in a6, writes it to offset 24 of that structure, and if certain conditions regarding a value in d2 and d0 are met, sets a byte at offset 22 of that structure before returning.
- `clear_structure_word`: Clears a 16-bit value at offset 24 of a structure pointed to by a6 and returns.
- `update_structure_value`: If a byte at offset 3 of a structure in a6 is non-zero, copies it to offset 23 and clears offset 3; otherwise, if a word at offset 24 is non-zero, it adds a byte from offset 23 to offset 16 or subtracts it, potentially updating a value in a5 and clearing offset 24.
- `update_structure_pointer`: Subtracts a word from a structure field, and if a certain condition is met, copies a word from one structure field to another and clears a word in a structure field.
- `calculate_scaled_offset`: Calculates a value based on a byte from a structure, a table lookup, and a multiplication, then stores the result in a structure and potentially advances a phase byte.
- `subtract_and_advance_phase`: Subtracts one input value from another and stores the difference in a global variable.
- `copy_structure_word`: Copies a 16-bit value from a structure at offset 16 of a6 to a structure at offset 6 of a5 and returns.
- `process_music_command_type`: Reads a command type or value from a structure in a6 and branches to different subroutines or copies a word to a structure in a5 based on the masked or compared value.
- `clamp_counter_value`: Extracts a nibble from a byte in a structure, and if non-zero, adds it to a 16-bit counter in a structure, clamping the result to a maximum value before copying it to an output field.
- `decrement_counter_clamped_zero`: Subtracts the low nibble of a parameter byte from a 16-bit counter, clamps the counter at zero, and copies it to an output field.
- `decrement_position_clamp_minimum`: Subtracts a byte parameter from a 16-bit position.
- `increment_position_clamp_maximum`: Adds a byte parameter to a 16-bit position.
- `dispatch_instruction_opcode`: Reads a byte from a structure in a6, masks the lower four bits, and branches to different subroutines or performs data updates based on the resulting value; otherwise it returns immediately.
- `update_global_from_ciaa_pra`: Reads a byte from the CIAA_PRA register, masks bit zero, shifts it left by one, and ORs the result into a global variable after clearing bit one of that variable.
- `invert_byte_at_offset`: Inverts a byte at offset 71 of a structure in a4 and returns.
- `decrement_and_invert`: Decrements a byte at offset 3 of a structure in a6, stores the result at offset 109 of a structure in a4, and then inverts a byte at offset 113 of the structure in a4.
- `clamp_parameter_to_maximum`: Clamps a parameter byte to a maximum of 64 and copies the result to an output field.
- `clear_and_store_bits`: If the lower five bits of a byte at offset 3 of a structure in a6 are non-zero, it clears a byte at offset 112 of a structure in a4 and stores those five bits at offset 108 of the structure in a4.
- `process_audio_data`: Iterates through a data buffer to calculate values, writes them to a destination table, and clears several hardware audio registers and CIAA registers.

### Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:Loader

- `load_and_execute_library_functions`: Calls several library functions using pointers from a global variable, including logic to check return values and perform indirect calls or jumps.

### Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:Spidey

- `jump_to_address_2`: Performs an indirect jump to the address stored in a register.
- `initialize_resource_pointers_2`: Sets a pointer from a stack value, clears a global variable, and checks if d0 matches a specific signature.
- `release_system_resources`: Calls another routine, then if a global pointer is non-zero, calls a routine via a pointer from a structure, and if another global variable is non-zero, calls two library routines and a third routine via a pointer before returning a value.
- `initialize_resource_pointers`: Checks if a pointer or a structure field is non-zero, and if so, sets a field in a structure to a maximum value and proceeds to check further pointers in a table.
- `handle_object_cleanup_logic`: Checks a global byte, calls a routine using a pointer from a structure, clears a global byte, and then calls several routines, including a loop that checks pointers and calls library functions or other routines.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `increment_global_counters_2`: Checks if a global word is non-zero, then calls two routines and increments two consecutive global words by one before returning zero.
- `clear_and_test_global_word_2`: Clears a 16-bit word in a global variable and returns immediately if the value is zero; otherwise it enters an infinite loop.
- `allocate_memory_conditionally_2`: Attempts to allocate a block of 65546 bytes using a library call and, if the allocation fails, jumps to another routine; otherwise, it returns the result of the allocation or jumps to another routine.
- `initialize_data_table_2`: Calls a routine via a pointer, then if a global pointer is not null, copies a sequence of words from a source table to a destination table, calls another routine, and finally populates a global table with a sequence of longwords and updates two global pointers.
- `rebuild_structure_list_2`: Calls a routine with three different size parameters, and if a global word is non-zero, it calls a routine to allocate memory, copies a sequence of words, and updates two global pointers.
- `call_exec_free_if_nonzero`: Checks if a longword in d1 is non-zero, and if so, calls the routine pointed to by a global variable minus 534 (which corresponds to the free memory routine) using a pointer in d1; otherwise it returns.
- `swap_and_reset_globals`: Swaps the contents of two global pointers, clears a byte in a global variable, and sets a word in another global variable to one, then returns.
- `populate_data_table`: If a global word is non-zero, clears it, then loops to swap halves of a word from a global and write it to a table, and finally updates two pointers in a structure.
- `copy_word_with_offset`: Copies 16 words from a source pointed to by a0 to a destination starting at a global variable, while simultaneously writing each word to an offset of 32 bytes from that same destination pointer using a loop.
- `set_handler_data_2`: The routine checks a pointer argument and, if non-zero, calls a function obtained from a global variable table.
- `reset_object_flags_2`: The routine clears a byte at an offset in a structure pointed to by a parameter and sets two 32-bit fields to all ones, then calls another routine via an indirect pointer from a global variable and returns its result, or returns a constant if the parameter is zero.
- `manage_file_position`: Uses library functions to seek, read, or manipulate file positions based on comparisons of input values and current file state.
- `verify_state_and_return_flag_2`: Calls a routine through a pointer in a table, compares its result with a register, and otherwise sets a 32-bit register to a constant value before branching.
- `verify_state_and_return_flag`: The routine calls another routine through a pointer stored in a table, compares its result with a register value, sets a 32-bit flag to all ones if they differ and to zero otherwise, calls another routine through a pointer stored in a table, and returns a 64-bit value formed from the flag and the incoming stack value.
- `execute_three_subroutines`: Calls three subroutines and then branches to another routine.
- `initialize_memory_buffers`: Copies several blocks of data to various destinations using a routine that performs structured copies, then executes a loop that performs two structured copies for each of eight iterations, and finally performs a loop that performs two structured copies for each of sixteen iterations.
- `copy_charset_data`: Copies a block of data to a destination and then executes a loop that performs two structured copies for each of four iterations.
- `call_routine_and_clear_pointer`: Calls a routine, checks if the return value is negative, and if so, clears a pointer in a register before returning; otherwise it returns.
- `copy_bytes_with_stride`: Performs a loop 128 times, copying a byte from a source at a0 to a destination at a1, then incrementing a0 by 6 and a1 by 1 in each iteration.
- `copy_structured_data_nested`: Executes a nested loop where the outer loop runs 7 times and the inner loop runs 167 times, copying four 32-bit words from a source structure to a destination structure with specific offsets.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_and_update_entity_state`: Clears a series of memory blocks, initializes several global pointers and counters, and performs a sequence of data copies and bitwise operations to update entity attributes and status.
- `update_global_pointer`: Loads a longword from a global variable, performs a bitwise rotation and an addition, and stores the result back into another global variable.
- `fill_data_loop`: Extends a word, then calls four different routines before executing a loop that writes a constant word to a memory location and decrements a counter.
- `update_string_buffer`: Calls a routine, then checks if a value in d0 is certain characters; if certain conditions are met, it copies a word from a global variable to another global variable, or otherwise copies a word from a global variable to another global variable and flips a bit in a global variable.
- `initialize_memory_block`: Swaps d0, checks a global word, and if certain conditions are met, writes a byte and two words to certain global addresses, loads a pointer into a1, adds an offset to it, loads a new address into a0, and enters a loop that copies 1999 32-bit words from a source pointer to a destination pointer before returning.
- `update_counters_based_on_thresholds`: Runs a loop based on a value from a global variable, performing various arithmetic operations on several global 16-bit and 8-bit counters depending on comparisons between a byte from a structure and specific thresholds.
- `do_nothing`: Performs a bitwise OR with zero on d0.
- `clear_data_register_d0`: Clears the least significant byte of data register d0.
- `update_object_collision_flags`: Iterates through a list of objects to check if their certain coordinate or index-based properties fall within a range defined in a table, and if so, sets or clears a bit in a byte or updates a global counter.
- `copy_and_update_data_blocks`: Decrements a byte at a global address and calls a routine; it then performs a loop to copy words from a source to a destination, and if a certain condition is met, it performs a second loop to copy larger blocks of data using a multiplier.
- `initialize_memory_pointers`: Writes a byte and two words to certain global addresses, loads a pointer into a1, adds an offset to it, loads a new address into a0, and branches to another instruction.
- `update_parameter_state_and_check_bounds`: Checks a global variable and, if certain bits are set, calculates a new coordinate or index, then iterates through a table, checking if any byte exceeds a certain value and updating a global pointer or value.
- `clears_d0_register`: Clears the least significant byte of the d0 register using a zero-valued immediate or-operation.
- `set_parameter_register`: Clears bit 1 of a global variable, and if the bit is set, sets d0 to 9 and calls another routine.
- `update_parameter_state_2`: Sets certain global values to certain constants, clears a bit in a global variable, and if a certain bit is set, loads a pointer from a global variable, reads a byte, and calls two routines.
- `update_parameter_state`: Loads two addresses into a1 and a2, calls another routine, stores d4 and d5 into the locations pointed to by a1, calls another routine, loads a pointer from a global variable into a0, compares d0 to 3, performs a table lookup in d0 if it is not less than 3, calls another routine, loads a pointer from a global variable into a0, calls another routine, and stores d7 into a global variable.
- `calculate_summed_values`: Calculates two values by summing several 16-bit words from different global locations and adding 10 to each, then returns them in d4 and d5.
- `calculate_vector_intersection`: Calculates intersection coordinates by performing arithmetic on two vectors derived from global pointers and input registers.
- `process_object_update_loop`: Clears a series of bytes in a table and iterates through a list of structures, updating 16-bit values at offsets 40, 42, and 44 based on results from other routines.
- `process_structure_list`: Iterates through a list of structures pointed to by a value at offset 46 of a structure in a6, calling another routine and updating values in the structure in a6 based on a table and input registers until a bit is cleared in a byte at offset 12 of a pointer.
- `prepare_blitter_bounds`: Calculates the right and bottom edge coordinates for a drawing area by adding a width and height (from a structure in a6) to a starting position (from a structure in a6), then passes a pointer to a data structure to calculate_blitter_area_and_draw.
- `update_structure_state`: Checks a global variable and a bit in a structure in a5 to determine whether to clear a byte in a6 or perform a series of operations involving fetching nibbles from a table, calculating differences between values at offsets 30, 32, 34, and 36 of the structure in a6, and running a loop that calls another routine and updates values in a6.
- `process_collision_data`: Iterates through a table, performing bitwise rotations and comparisons on values from the table against bytes loaded from a0, and increments a byte in the table if a condition is unable to be met.
- `update_entity_position_and_attributes`: Calculates movement deltas for two 16-bit values based on scaled offsets and iterates through a table of structures to update their coordinate and attribute fields.
- `parse_and_populate_entity_structures`: Iterates through a source data table to populate a list of structures, calculating coordinate offsets and bit-packed attributes from the input bytes.
- `copy_nested_data_blocks`: Performs a triple-nested loop to copy bytes from two source tables into four destination buffers, incrementing pointers by specific offsets in each loop or otherwise returns.
- `copy_memory_block`: Copies a block of 8-byte chunks from a source pointer in a0 to a destination pointer in a1, using a loop that runs 8 times.
- `iterate_and_call_copy_strided_data_blocks`: Iterates through a table of pointers using a count from a global variable, calls copy_strided_data_blocks if the byte at the current pointer is non-negative, and increments the pointer by 210 bytes each iteration before returning.
- `update_entity_attributes`: Iterates through a table to calculate and store new byte values into a destination buffer based on bitwise operations and arithmetic on values from a source table and a certain global structure.
- `update_entity_positions_and_bounds`: Iterates through a table of structures to update entity coordinates and bounding boxes by reading scaled offsets from a table and adding them to existing values.
- `update_structure_bounds_and_flags`: Calculates new pointer or offset values based on a structure's properties, updates a global counter, and writes a sequence of calculated or register-derived bytes into a destination buffer.
- `toggle_structure_bits`: Calls two routines to toggle bit 7 and bit 3 of a byte at offset 12 of a structure in a2, then clears d4 and d3 and returns.
- `calculate_offset_value`: Calculates a 16-bit value by performing arithmetic and bit shifts on two 16-bit words from a table and adds the result to a pointer in a0, then loads a byte into d1 and an extended word into d2.
- `copy_strided_data_blocks`: Calls another routine, then enters a nested loop that copies bytes from a source structure at offset 18 of a2 to a destination pointed to by a3, adjusting the source and destination pointers by a stride of 6 and a constant offset after each inner loop or block.
- `toggle_bit_seven`: Checks bit 7 of a byte at offset 12 of a structure in a2 and toggles it by shifting and XORing the value, then clears d4 and returns.
- `toggle_bit_three`: Checks bit 3 of a byte at offset 12 of a structure in a2 and toggles it by shifting and XORing the value, then clears d3 and returns.
- `setup_blitter_parameters`: Saves registers, stores a 32-bit value from offset 14 of a2 into a global variable, and calls a routine to configure the blitter using values from a1 and a2.
- `update_sprite_data_2`: Calculates an index and bitmask from a source table, conditionally modifies a byte in a target buffer, and then writes a large block of data.
- `write_entity_state_record`: Calculates an index and bitmask from a source table, conditionally modifies a byte in a target buffer, and then writes a large block of data including pointers and bytes to a destination buffer.
- `modify_bit_buffer`: Clears a bit in a structure at a6, calls another routine, and then enters a loop that reads bytes from a2 to perform bitwise operations on a target buffer pointed to by a table, occasionally calling another routine or updating a value at an offset of 984 from a0.
- `update_buffer_from_table`: Checks a condition in a structure, modifies a byte in a target buffer based on a bit test, and then copies seven 4-byte sequences from a table into a destination buffer.
- `update_sprite_data`: Calculates memory offsets based on input parameters and performs nested loops to copy blocks of four bytes from a source table into a destination buffer.
- `test_and_modify_bit_table`: Loads a count from a table and enters a loop that, for each entry, checks if a specific bit is set in a memory location pointed to by a table entry, and if so, calls another routine using a pointer or data source from that same table entry before advancing to the next entry.
- `decompress_data_stream`: Pushes a0 to the stack, loads a pointer into a1, calls a routine to decompress data, and returns.
- `load_pointer_and_set_addresses`: Loads a byte from a global variable into a register, sign-extends it to a word, and loads two constant addresses into registers a0 and a1.
- `decompress_custom_stream`: Reads a compressed data stream from a table pointed to by a pointer in a1 and writes the decompressed bytes to a buffer in a0.
- `execute_instruction_stream_logic`: Processes a buffer of instructions by updating a structure, potentially updating a global 32-bit value and a 16-bit offset, and calls several other routines to handle specific instruction types or update structure fields.
- `process_instruction_stream`: Iterates through a buffer, checking for an end marker, and calls several routines to update structure fields, calculate or shift values, and update a palette or sprite attribute buffer using a lookup table or direct copying.
- `update_structure_values`: Calculates and stores two 16-bit values into a structure at offsets 30, 32, 34, and 36 based on values from a table and input registers, then returns.
- `store_and_scan_buffer`: Stores a pointer from a0 and three values from a6 into a structure, then loops through the memory pointed to by a0 until it encounters a byte with the value negative 128.
- `update_structure_fields`: Reads a byte from a0, uses it to index a table to update a 32-bit value at offset 14 of a6, and conditionally updates a 16-bit value at offset 2 and an 8-bit value at offset 4 of a6 based on bits in a byte at offset 27 of a6.
- `reset_structure_fields`: Checks a bit in a structure at a6, and if clear, clears a word at offset 22 and a byte at offset 1 of that structure, then enters a loop that calls another routine and updates certain offsets in the structure using values from a table.
- `update_structure_fields_2`: Copies a byte from a pointer in a0 to offset 27 of a6, calls two other routines, updates offset 24 of a6 by adding the value at offset 54 of a6 shifted left by 1, updates offset 26 of a6 by adding the value at offset 56 of a6 shifted left by 1, and clears offsets 40, 42, 54, and 56 of a6.
- `load_three_bytes`: Loads a word from a pointer incremented by a0 into d2, then loads a byte from a pointer incremented by a0 into d3, and loads a byte from a pointer incremented by a0 into d5.
- `fetch_table_byte`: Selects one of four tables based on the signs and values of d2 and d3, then returns a byte from that table using an offset calculated from d2 and d3.
- `update_clamped_velocity_and_position`: Calculates clamped velocity values and updates a structure at offset 1 of the structure in a6, then performs a loop to adjust position values by calling another routine and updating offsets 34, 35, 36, and 37 based on the results.
- `negate_if_negative`: Checks the signs of d0 and d1, and if either is negative, negates that register before returning.
- `load_and_clear_d5`: Loads a word from a structure offset 2 of a6 into d4, then clears d5 and loads a byte from a structure offset 4 of a6 into d5.
- `increment_and_shift_values`: Increments d4 and d5 by 10 in a loop until d5 reaches the value in d2, then performs an arithmetic shift right by 3 on both registers and returns.
- `negate_registers_by_bit`: Checks bits 0, 1, and 2 of d1 to determine whether to negate d3, d2, or both, then returns.
- `extract_combined_nibble`: Pushes a0 to the stack, performs a bitwise operation on a value from a byte at offset 7 of a6 and the result of a call to another routine, then returns the combined value in d0.
- `call_pointer_update_handler`: Pushes a0 to the stack, loads a pointer into a0, calls a routine, and restores a0 before returning.
- `fetch_nibble_from_table`: Saves a pointer to the stack, loads the address of a data table into a0, calls a routine to extract a nibble from that table, restores the pointer, and returns.
- `extract_nibble_from_table`: Calculates an index using d0 and d1, fetches a byte from a table at that index, and returns either the high or low nibble of that byte based on a bit in d1.
- `check_bounds_and_call_handler`: Checks if a6 is not a specific address and if bit 7 of offset 12 of a pointer at offset 46 of a6 is set, then verifies if four values from a6 fall within a specific range defined by a table, and if so, adds a value from a5 to a global variable and calls another routine.
- `update_bounding_box_center`: Calculates the midpoint of a coordinate range provided in a structure, checks if this midpoint falls within specific bounds relative to a reference point, and if so, updates a global minimum distance value and several global variables with the new midpoint and the structure pointer.
- `iterate_and_update_structure`: Loads a word from a global address into d0, sets a global word to 32767, and enters a loop that checks bit 7 of a byte at offset 12 of a structure, calls another routine, and increments a pointer, otherwise it returns.
- `set_d0_to_zero`: Performs a bitwise OR on d0 with zero.
- `clears_d0_register_2`: Clears the low byte of the d0 register.
- `process_coordinate_table_loops`: Checks the sign of two 16-bit values in d2 and d3; if positive, it calculates a loop count by subtracting two scaled values from a structure in a6, then executes a loop that calls fetch_table_byte and potentially calls toggle_bit_seven or toggle_bit_three while incrementing a value at offset 58 of the structure in a6, otherwise it returns.
- `copy_data_blocks`: Copies blocks of four bytes from one source to a destination using nested loops, where the source address is determined by either a value from a table or a value from a global variable, and the destination is updated through a series of offsets.
- `update_sprite_attribute_table`: Iterates through a table of bytes, modifies a nibble in each byte to increment a value, and uses that value as an index to copy blocks of 8 bytes from a source buffer into a destination table.
- `update_sprite_palette_data`: Updates a palette or sprite attribute buffer by either copying 32-bit words or using a lookup table to transform bytes before writing them to a destination pointer.
- `set_d0_and_jump`: Sets a bitmask in d0, branches to another routine, and calls another routine.
- `decompress_data_blocks`: Performs a multi-stage decompression or block-copying routine that reads a source buffer and writes to a destination buffer.
- `update_element_list_and_handle_input`: Updates a list of elements by shifting bytes or deleting entries, and processes certain input or control characters by updating a pointer-based table, calling several other routines, and modifying a structure or buffer at a certain offset.
- `validate_and_branch_on_range`: Calls a routine, clears registers, loads a pointer from a global variable, and checks if the difference between a constant and a value added to that pointer is non-negative before calling two more routines and branching to another routine.
- `set_byte_to_max`: Stores a byte with value 255 into a long-word at a certain address.
- `clears_d0_and_branches`: Clears the d0 register and branches to a location in another routine.
- `clear_long_variable`: Clears the lower byte of a long-word and then returns.
- `decrement_two_variables`: Subtracts 8 from a 16-bit variable and then subtracts 8 from another 16-bit variable before returning.
- `increment_two_variables`: Adds 8 to a 16-bit variable and then adds 8 to another 16-bit variable before returning.
- `calculate_and_store_offsets`: Takes a byte from a global variable, sign-extends it, multiplies it by 8, adds 64, and stores the result in one 16-bit global variable and the result plus 7 in another 16-bit global variable.
- `copy_structured_data_blocks`: Calculates an offset based on a global variable, then enters a loop to copy blocks of four bytes from two source tables into a destination structure, while also clearing specific bytes in the destination structure during certain iterations.
- `initialize_copy_parameters`: Calls a routine, calculates a pointer to a table or structure, and sets up a register with a count and a pointer to a source table before calling a routine via a pointer-based handler, then calculates a destination pointer and sets a loop counter.
- `copy_multiple_data_entries`: Uses a byte from a source list to calculate an offset into a table, then copies eight bytes from that table location into a destination structure, repeating this for a number of entries specified by a second source value.
- `update_table_and_copy_data`: Calculates a value, stores a byte at a pointer, and then runs a loop that copies eight bytes from a table location into a destination structure.
- `execute_data_processing_loop`: Calls several routines, pushes values onto the stack, and enters a loop that runs 16 times, calling a routine via a pointer and another routine twice within the loop before returning.
- `reorganize_element_buffer`: Iterates through a list of elements to calculate a new size, updates several global length and offset parameters, and then performs a block copy or shift of element data, potentially clearing or updating a buffer, before calling another routine and returning.
- `calculate_blitter_bounds_and_execute_2`: Decrements a byte in a global variable, masks it, and uses the result to load a jump target from a table, then calls calculate_blitter_bounds_and_execute with a pointer to a structure in a6 and two pointers to calculate a range or area.
- `calculate_offset_and_copy_data`: Calculates a destination address using a value from a global variable, then runs a loop that copies eight bytes from a source table into a destination structure.
- `process_input_and_update_globals`: Calls several routines, then checks if a byte in d0 (after a swap) matches certain characters; if certain conditions are met, it writes a constant word to a global variable and clears another global variable, otherwise it loops back to the start.
- `tiny_moveq`: Small routine of 1 instruction that loads 19 into d3.
- `copy_conditional_data_blocks`: Iterates through a list of bytes; if a byte is non-zero, it uses that byte to calculate a table offset and copies eight bytes into a destination structure, then increments the destination pointer by 1952.
- `prepare_blitter_parameters`: Loads a pointer to a structure into a1, calls a routine, then loads a pointer into a0, adds a constant to d4, calls another routine, calls a third routine twice, and finally loads a global pointer into a2.
- `execute_sequential_copy_blocks`: Performs a series of pointer increments and calls another routine, then runs a loop that copies eight bytes from a source table into a destination structure.
- `copy_single_data_block`: Calculates an offset from a value in a source list, then copies eight bytes from that table location into a destination structure using a loop that runs eight times.
- `update_32_bit_accumulator_2`: Adds a 16-bit value from a memory location to a 32-bit value stored in two memory locations, then swaps the high and low 16-bit words of the result in place and returns.
- `set_d7_register`: Clears the d7 register and branches to another instruction.
- `loop_and_test_condition_2`: Calls a routine, calls another routine, calls a third routine, and if the zero and negative flags match, returns; otherwise it decrements d7 and loops if d7 is not negative.
- `toggle_status_flags_2`: Reads two bytes from a memory location, swaps them, and then uses bit testing to return various longword constants or clear a global word.
- `update_blitter_parameters_2`: Loads a pointer to a structure, calls another routine, and then updates several fields in a structure, including a width or offset value, a height or offset value, and a bitmask or flag, while performing arithmetic or bitwise operations on a pointer or index.
- `call_copy_compressed_data`: Pushes two 16-bit values, a 32-bit zero, and two 32-bit pointers onto the stack before calling a routine to copy compressed data blocks.
- `clear_memory_block`: Loads a pointer from a global variable and uses a loop to write zero into each 32-bit word at that location 4095 times before returning.
- `load_pointers`: Loads a pointer from a global variable into a0 and loads another pointer from a global variable into a1.
- `copy_memory_block_2`: Copies 1999 32-bit words from the source pointed to by a0 to the destination pointed to by a1 using a loop.
- `clear_d0_bit`: Clears any bits in d0 and then tests bit 0 of d0.
- `set_d0_bits`: Sets certain bits in d0 using or instructions.
- `initialize_null_data_byte`: Sets the least significant byte of register d0 to zero.
- `clears_least_significant_bit_of_d0`: Clears the least significant bit of the d0 register.
- `clears_d0_register_3`: Sets the least significant bit of d0 to zero.
- `clear_d0_register_8`: Sets the contents of the d0 register to zero.
- `clear_d0_register_3`: Clears the least significant byte of register d0.
- `execute_conditional_tasks`: Saves registers to the stack, calls another routine, calls another routine, tests a byte in a global variable, and based on the result, calls one or two other routines before restoring registers and returning.
- `fetch_and_reset_global_word_2`: Checks if a global word is equal to negative one; if not, it calls another routine, processes the result by swapping bytes and combining it with the global word, then resets the global word to negative one and clears a following word before returning; otherwise, it calls a routine and returns a value based on the result or zero.
- `set_global_values_2`: Sets a global word to all ones and clears a global word, then returns.
- `initialize_device_and_structure`: Calls several routines and, if certain pointers or return values are non-zero, updates a field in a structure, sets a word in a structure, and calls another routine before jumping to another routine.
- `handle_object_cleanup`: Checks if a pointer in a global variable or a secondary global variable is non-null and checks an offset in a structure, then calls library functions and other routines to update structure fields and clear certain bytes or bytes at an offset.
- `check_structure_type_and_clear_register`: Checks if a byte at offset 4 of a structure in a0 equals 2; if not, it branches to another routine, otherwise it checks a bit in a word from a global variable and clears d0 if that bit is set, otherwise it returns.
- `copy_pointer_to_register`: Copies a longword from a pointer in a0 to d0 and returns.
- `update_global_data_from_structure`: Checks if d0 is one, checks if a value at an offset of a0 is non-negative, and checks if bit 6 of a value at an offset of a0 is clear.
- `update_view_parameters`: Compares d1 against certain values, and if they match, writes d1 and d0 to a global table before checking a condition and returning or clearing d0.
- `set_view_and_return_2`: The routine writes a constant to a global variable, then calls another routine via a table of function pointers, and returns the value received in register A0.
- `process_cheat_confirmation_input`: Clears a 16-bit word in a global variable and sets two pointers in a structure to zero, then calls several routines to reset or update global pointers, clears a byte in a global variable, and checks input characters to update a buffer or modify a global counter or pointer.
- `reset_and_retry_sequence`: Calls two routines, loads a pointer into a0, calls a third routine, sets a global variable to negative one, and then enters a loop that calls two routines and branches back to a previous instruction if the zero flag is not set, otherwise it returns.
- `calculate_table_index_offset`: Calls several routines, then checks if a value (decremented by 49) is between 0 and 4 inclusive; if so, it shifts that value left by 6, adds it to a constant, and stores the resulting long pointer into a global variable, otherwise it returns.
- `initialize_data_buffer`: Stores a pointer into a global variable, clears a byte at a global variable, and fills a 23-byte buffer starting at a specific address with the value 32 using a loop.
- `update_state_and_process_list`: Updates several global bytes and words, then enters a loop that compares a masked value against a global variable.
- `find_non_space_character`: Skips spaces in a buffer pointed to by a0 using a loop, then compares a byte from a1 with a character from the buffer to determine if it falls within the range of lowercase letters, returning a modified byte or a specific byte based on comparisons.
- `calculate_scaled_offset_2`: Adds a 16-bit value from offset 4 of a structure in a0 to d0, then calculates a new address by multiplying a 16-bit value from offset 6 of a pointer at offset 0 of a0 with a 16-bit value from offset 6 of a0, adding this product and a bit-shifted version of d0 to a0.
- `copy_compressed_data_blocks_2`: Performs a multi-stage decompression or block-copying routine that reads a source buffer and writes to a destination buffer.
- `jump_to_address`: Performs an indirect jump to an address stored in a register.
- `calculate_blitter_area_and_draw`: Calculates bitplane offsets and Blitter parameters based on a structure's dimensions and coordinates, then configures the Blitter hardware registers to perform a block transfer.
- `interpolate_parameter_tables`: Compares two 16-bit coordinate pairs against bounds provided in a table to determine their state, then performs linear interpolation or clamping on the values in the second table based on the relationship between the two pairs.
- `calculate_blitter_bounds_and_execute`: Calculates a range or area based on a difference between two pointers, updates a structure at a6 with calculated bounds, certain offsets, and a function pointer, then calls a function from a table or otherwise returns.
- `call_function_table_loop`: The routine repeatedly calls a function pointer taken from a table in A1, decrementing a counter in D3 until it reaches -1, then returns the value originally on the stack.
- `call_function_table_loop_2`: The routine repeatedly calls a function taken from a table via A1, decrementing a counter in D3 until it reaches -1, then returns the value originally passed on the stack.
- `execute_blitter_loop`: Executes a loop that calls a routine via a pointer, performs arithmetic on values from a structure, and populates a structure at a6 with several calculated values before calling another routine multiple times.
- `mask_bytes_in_pattern`: The routine repeatedly masks four bytes spaced by a constant offset using a byte mask, writing the masked results back through a pointer.
- `apply_byte_mask_pattern`: The routine writes a byte from a register into a memory location pointed to by a pointer, OR-ing it with the existing byte, then repeatedly applies a byte mask from a register with AND to three successive locations spaced by a word offset, advancing the pointer by that offset each iteration.
- `mask_and_or_pattern`: The routine iterates a fixed number of times, using a pointer and a stride to access a block of memory.
- `apply_byte_mask_pattern_2`: The routine writes a byte from a register to a memory location pointed to by a pointer, ORs it with the existing byte, then repeats the OR with the same byte at an offset, then ANDs two more locations with a mask byte, advancing the pointer by a stride each time in a loop that runs a fixed number of times.
- `mask_bytes_in_pattern_2`: The routine repeatedly masks bytes at a base address and at offsets using a mask byte and an OR byte, writing the results back through a pointer.
- `apply_mask_pattern_to_bytes`: The routine writes to a byte pointed to by a pointer using a byte from a register and a byte from another register, repeating the operation with a fixed byte offset four times per iteration.
- `mask_and_fill_pattern`: The routine iterates a fixed number of times, using a byte mask from a register and a byte value from a register to AND and OR bytes in a memory area accessed through a pointer and a stride.
- `fill_masked_bytes`: A routine that iterates a fixed number of times, OR-ing a byte from a register into three consecutive memory locations pointed to by a pointer and a stride, then AND-ing a second byte from a register into a fourth location, advancing the pointer by the stride each iteration and decrementing a counter until it reaches -1.
- `mask_and_fill_pattern_2`: The routine repeatedly masks three consecutive bytes at an address derived from a base pointer and a stride using a mask byte, then ORs a fill byte into the next byte in the sequence.
- `apply_masks_to_buffer`: The routine writes to a memory buffer using a pointer and a stride.
- `apply_masked_pattern_to_buffer`: The routine iterates a fixed number of times, masking a byte at a pointer with a byte from a register and setting another byte at an offset with a byte from a register using bitwise OR, repeating the pattern with a stride.
- `apply_bitmask_to_memory`: Iterates through memory locations at fixed intervals, applying bitwise OR and AND operations with specific byte values to modify the data.
- `mask_and_fill_bytes`: The routine repeatedly masks two bytes at an offset from a base pointer using a byte from a register and then ORs the same byte into the next two bytes at the same offset, decrementing a counter until it reaches -1.
- `apply_byte_mask_pattern_3`: The routine writes a byte from a register into a memory location pointed to by a pointer, OR-ing it with the existing byte, then uses a second byte from a register to AND a byte at an offset, and repeats OR operations at further offsets using a decrementing counter until the counter reaches zero.
- `fill_masked_bytes_2`: The routine repeatedly ANDs a byte from a register with bytes in a memory area pointed to by a pointer, then ORs a byte from another register with three subsequent bytes spaced by a word offset, updating the pointer and a counter until the counter reaches -1.
- `xor_fill_pattern`: The routine repeatedly ORs a byte from a register into four consecutive bytes of a buffer, stepping through the buffer by an offset held in a register, and repeats the operation a number of times counted down in another register.
- `mask_words_in_table`: The routine masks four consecutive 16-bit words in a table with a value in a register, using an offset and a pointer derived from the table base.
- `apply_mask_to_table_entries`: The routine ORs a 16-bit value from a register into a global variable and then applies a bitwise AND mask from a register to three consecutive 16-bit entries in a table addressed via a pointer offset by a register value.
- `mask_and_or_word_pair`: The routine masks a word at a pointer with an input word, then ORs a second word at an offset derived from a short offset and the first pointer with another input word, and finally masks two further words at offsets derived from the updated pointer with the input word.
- `apply_bitmask_to_table_entries`: The routine ORs a 16-bit mask from a register into a word pointed to by a pointer, then uses an offset from a register to index a table of words and ORs the same mask into that entry.
- `mask_and_or_word_pair_2`: The routine masks two words pointed to by a pointer derived from an input word and a base pointer, using a bitwise AND with an input word, then ORs a word from a register into a third word.
- `apply_mask_to_word_table`: The routine reads a 16-bit value from a register and a 16-bit mask from a register, then ORs the mask into a word pointed to by a pointer in a register, and performs a series of indexed AND and OR operations on words addressed relative to a base offset in a register using the current pointer value.
- `mask_and_or_word_table`: The routine masks one word at a pointer with a value from D0 and then ORs a value from D4 into two words located by adding an offset from D2 to the pointer and to the result, and finally masks a further word reached by adding D2 again with the D0 value.
- `or_data_table_via_pointer_chain`: The routine uses a pointer held in a global variable as a base and an offset from a register to index a table of 16-bit words.
- `mask_and_or_word_table_2`: The routine masks two words in a table pointed to by a global pointer using a word from D0, with an offset from D2, then ORs a word from D4 into a further offset word in the same table.
- `mask_and_or_word_pair_3`: The routine reads a word from a global variable, ORs it with a word pointed to by another global pointer, then uses an offset from a register to index into that pointer to mask two words with an input word, and finally ORs a word from a global variable into a word located at an offset derived from the previous pointer.
- `mask_and_or_word_pair_4`: The routine masks a word at a pointer with an input word, then ORs a word at an offset computed from an index and the pointer with another input word.
- `other_routine`: This routine modifies memory indirectly via a series of pointer arithmetic operations using a global variable.
- `mask_and_or_word_pair_5`: The routine masks two words pointed to by a pointer and an offset pointer with a word from a register, then ORs a word from a register into two further words derived by adding an offset to the previous pointers.
- `mask_and_or_word_pair_6`: The routine reads a word from a global variable and ORs it with a pointer value, then uses the result as an offset to mask a word at a computed address with an input word, and finally ORs a mask word into two further words derived from the masked result.
- `mask_and_or_two_words`: The routine masks a word at a pointer with a value from a register and then ORs a word from a register into two words located at offsets derived from another register and the pointer, and then ORs the same register value into a third word at a further offset derived from the previous result.
- `set_bits_in_table`: The routine sets bits in a table by OR-ing a mask from a register with four words selected via pointer arithmetic using an offset from a register.
- `setup_blitter_context`: Loads a pointer from a global variable into a6, saves registers to the stack, calls the blitter, restores registers, and loads the custom chip base address into a6.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_disown_blitter`: Library stub: loads its arguments and the graphics library base, calls DisownBlitter and returns its result to the caller.
- `apply_bitmask_rotation`: Calculates a bitmask and a rotation amount based on values in a structure, then enters a loop that performs bitwise operations and bit rotations to update a destination buffer using data from a source buffer.
- `process_blitter_data`: Reads values from a structure, performs calculations involving bitwise operations and shifts, and writes multiple values to a structure, including several updates within loops that check a bit in a byte at offset 2.
- `update_blitter_parameters`: Calculates new coordinates and parameters based on values from a structure, then performs a loop that updates a structure at a6 with several values while checking a bit in a byte at offset 2.
- `initialize_data_table`: Performs a series of or instructions that do not change the value of a data register.
- `call_object_handler`: Checks if a 16-bit value is non-zero, then uses it as an index into a table to fetch a pointer and calls another routine with that pointer.
- `update_object_list`: Iterates through a list of structures starting at a global address, incrementing a byte at offset 1 until it equals 1, then populates the structure with data from a provided pointer and a table based on a value at offset 2.
- `iterate_and_match_table_entries`: Runs a loop three times that checks if a 16-bit value at a pointer in a table matches a parameter and if the lower 4 bits of the following byte are non-zero; if both conditions are met, it clears that byte and calls another routine.
- `process_object_instruction_stream`: Checks a type-identifier in a structure, calls a handler, and if a counter at offset 22 reaches zero, resets it to one and iterates through a list of instructions, updating a pointer and a byte-based counter or position or or otherwise returns.
- `select_table_entry_and_jump`: Compares a longword in a register to a constant, and if it is not less than that constant, shifts the register left by one and uses the resulting value as an index to load a word from a table and jump to that address; otherwise, it jumps to another routine.
- `process_object_type_instructions`: Checks a type-identifier or flag in a structure, and if certain bits are set, pushes a series of values including a counter, a pointer, and a data word onto the stack before calling another routine.
- `process_object_table_loop`: Decrements a counter in d6 in a loop, pushing a zero word, a word from offset 24 of a structure in a3, and the value of d6 onto the stack before calling another routine, then adjusts a3 and the stack pointer.
- `decrement_and_update_bounds`: Decrements a byte at offset 51 of a structure, and if it reaches zero, resets it to a value from offset 50, then updates a 16-bit position or range-mapped value based on certain flags and checks if it falls within a specified range, potentially calling another routine to update a pointer or value.
- `update_structure_bounds_and_flags_2`: Updates a 16-bit value in a structure by adding an offset, checks if that value falls within a certain range, and conditionally updates other structure members or flags based on bitmask checks.
- `copy_structure_word_2`: Copies a word from an offset of a structure in a2 to another offset of the same structure and then branches to another instruction.
- `conditionally_move_structure_word`: Tests a word at an offset in a structure pointed to by a2; if the word is not negative or zero, it copies a word from one offset in that structure to another offset in the same structure, then branches to another routine.
- `modify_structure_bits`: Copies a word within a structure, tests a bit in a structure, and if certain conditions are met, negates a word or clears a bit before branching to another instruction.
- `update_status_flags_logic_2`: Clears a 16-bit word at an offset of a structure in a2, checks a bit in a structure, and if certain conditions or values at offsets 38 or 40 are met, it modifies a byte at offset 40, pushes two bytes onto the stack, calls a routine, and updates a byte at offset 35.
- `update_status_flags_logic`: Adds a byte from an offset of a structure in a2 to another offset of the same structure, then checks if the new value is within a certain range or if a bitmask in a2 matches a certain pattern, potentially updating or negating bytes in the structure.
- `update_status_flags_2`: Copies a byte from offset 24 to offset 23 of a structure, then conditionally negates a byte at offset 26 and modifies bits in a byte at offset 26.
- `update_status_flags`: Copies a byte from offset 24 to offset 23 of a structure, then conditionally negates a byte at offset 26 and modifies bits in a byte at offset 29 based on specific bitmask checks.
- `map_value_into_range`: The routine obtains a short value from another routine and maps it into the interval defined by two short fields of its parameter.
- `perform_32bit_addition`: Performs a 32-bit addition using two 16-bit values stored at a memory location and updates two global variables with the result.
- `set_flag_on_lowercase`: The routine checks a byte at an offset from a pointer in A1.
- `update_button_status_2`: Reads a 16-bit word from offset 12 of a structure in a0, performs bitwise transformations on it, checks bit 7 of the CIAA Port A register, and sets various bits in a byte before storing that byte and its bitsw or a byte into two consecutive global byte locations.
- `clear_data_register`: Performs an or instruction on a data register that does not change its value.
- `set_data_register_zero`: Performs a series of or instructions on a data register that do not change its value.
- `clear_data_register_low_byte`: Clears the lower byte of a data register using a sequence of or-with-immediate instructions.
- `clear_d0_low_byte_repeatedly`: Clears the low byte of the d0 register four times in succession.
- `clear_d0_register_4`: Clears the lower byte of d0 four times.
- `clear_d0_register_5`: Sets the lower byte of d0 to zero four times.
- `clear_d0_register_6`: Performs a sequence of or instructions to zero the lower byte of d0.
- `clear_d0_register_7`: Clears the lower byte of d0 register four times in succession.
- `test_d2_d4_bits`: Tests bit zero of d2 and bit zero of d4.
- `initialize_resource_table_2`: Iterates through a table of pointers, allocating memory and initializing a linked list of structures while setting certain flags and updating pointers.
- `iterate_and_call_handlers`: Runs a loop four times, checking if a pointer at an offset from a base address is non-zero to call various routines.
- `set_structure_fields`: Uses a parameter to index into a table, then writes the constant eleven to offset 28 and zero to offset 30 of the resulting structure, before calling a routine via a pointer at offset 20.
- `unknown_routine_0_0_c18`: It calls another routine to initialize enemy structures, then calls another routine to initialize an object chain.
- `set_object_fields_and_trigger_callback`: The routine indexes into a table using an offset from the first argument, loads a pointer from the table, writes a constant word and a zero byte to fixed offsets in the pointed-to structure, copies a word from the first argument and a word from the second argument to other fixed offsets in the same structure, then calls a function pointer stored in the structure with a fixed negative offset applied.
- `allocate_and_initialize_structure_2`: Calls a function to check for a valid port, then calls a memory allocation routine and a signal allocation routine.
- `cleanup_message_port`: Checks if a pointer is valid and if a certain field or signal exists, then sets two bytes at offsets 8 and 32 of the structure to certain values, calls a routine to clear a signal, and calls a routine to free a memory block and a message port.

### Amazing Spider-Man, The (1990)(Empire Software)[h Dragon Inc].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
