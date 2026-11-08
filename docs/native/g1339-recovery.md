# Game 1339 (total eclipse (1989)(incentive software)[cr black monks][h acu]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01339). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903024 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 159 of 159 named (0 author symbols), 159 of 159 described (5 by rule, 152 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 502 of 502 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2756 of 2756 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4097 |
| code | 9488 |
| data | 449860 |
| padding | 2843 |
| unused | 436736 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Type | 2 | 117 | 2 |
| WaitReturn | 2 | 1527 | 115 |
| Total Eclipse (1989)(Incentive Software)[cr Black Monks][h ACU].adf:echo | 2 | 117 | 2 |
| Total Eclipse (1989)(Incentive Software)[cr Black Monks][h ACU].adf:hi | 3 | 228 | 7 |
| Total Eclipse (1989)(Incentive Software)[cr Black Monks][h ACU].adf:0.tec | 1 | 296 | 11 |
| Total Eclipse (1989)(Incentive Software)[cr Black Monks][h ACU].adf:bootblock | 1 | 226 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 162 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 82 | 6 |

## Cases handled

- total eclipse (1989)(incentive software)[cr black monks][h acu]/Total Eclipse (1989)(Incentive Software)[cr Black Monks][h ACU].adf: file SOT.POS (header 943 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- total eclipse (1989)(incentive software)[cr black monks][h acu]/Total Eclipse (1989)(Incentive Software)[cr Black Monks][h ACU].adf:1.TEC: high-entropy data file (8.00 bits/byte), classified as packed data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 82 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 27 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- total eclipse (1989)(incentive software)[cr black monks][h acu]/Total Eclipse (1989)(Incentive Software)[cr Black Monks][h ACU].adf: blocks 924-925 (bytes 473088-474112) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.02)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_complex_sequence` (WaitReturn): Calls several routines to load values, update a structure in a4 using a pointer from a6, and perform conditional logic involving a global variable and a structure at offset 12 of a6. Kind: checksum/data integrity trap. Pass path: u1__sub_0_0E54(uVar1,uVar2).

## Functions


### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_and_branch`: Moves a longword from d1 to d0 and then performs a relative branch.

### WaitReturn

- `jump_to_entry_point`: Performs a long jump to the address stored at the memory location specified by the address in a0.
- `trigger_trap_seven`: Moves d0 to d1, sets d0 to 6, executes trap 7, and returns.
- `manage_stack_state`: Calculates a value based on d0, stores it on the stack, saves the status register and several registers, copies the stack pointer to a0, and if a value on the stack is 7 or 8, performs a jump to another location; otherwise, it updates a stack value and jumps to another location.
- `branch_on_stack_value_2`: Compares a word at offset -2 of the stack in a6 with 15; if they are equal, it copies the stack pointer to a0, moves a longword from a stack offset to a new location, and jumps to another routine.
- `check_stack_value`: Compares a word at offset -2 of the stack in a6 with 16, and if it is not 16, clears two longwords at offsets -6 and -16 of the stack before jumping to another routine; otherwise it jumps to another routine.
- `branch_on_stack_value`: Compares a word from an offset of the stack in a6 with 9, and if it is less than 9, jumps to another routine; otherwise it moves the stack pointer to a0, loads a value from the stack into a0, and jumps to another routine.
- `initialize_structure_and_return`: Copies a value from a stack offset to another stack offset, populates a structure at a pointer in a4 with several longwords and a byte, updates a pointer via d0, and returns from an exception.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `store_and_return_extended`: Sets a7 using a pointer from a global, pushes two longwords from d0 onto the stack, and returns d5 as a longword after extending it from a word.
- `check_stack_and_trap`: Compares a calculated stack address against a value in a structure in a5, and if the condition is met, executes trap 7 before returning.
- `setup_complex_structure`: Executes trap 7, then conditionally executes it again based on a bit test and a comparison; if the conditions pass, it clears several longwords in a structure, sets a return address, modifies a word using bitwise operations, and updates a pointer in a1.
- `execute_trap_six`: Executes trap 6 and then returns.
- `restore_context_from_stack`: Pushes several registers onto the stack, switches the stack pointer to a new location, restores registers from that new stack, and returns from an exception.
- `execute_trap_five`: Executes trap 5 and then returns.
- `restore_context_via_pointer_2`: Saves the current stack pointer and registers to the stack, loads a pointer from a1, restores registers from the location pointed to by a1, and executes an exception return.
- `restore_context_via_pointer`: Saves the current stack pointer, pushes several values onto the stack, restores registers from a location pointed to by an offset in a structure, and returns from an exception.
- `dispatch_conditional_routines`: Disables interrupts, pops a byte from the stack, and performs a series of conditional jumps and calls based on whether the byte matches 39, 38, or 37; otherwise it calls another routine.
- `modify_stack_and_return`: Tests a value on the stack, decrements d0 and branches if it is not zero; otherwise it performs bitwise operations on the stack and returns from an exception.
- `decrement_and_return_word`: Decrements a byte in d0 and branches if it is not zero; otherwise it moves a word from d1 to the stack and returns from an exception.
- `decrement_and_return_exception`: Decrements a byte in d0 and branches if it is not zero; otherwise it sets the status register and returns from an exception.
- `decrement_and_return`: Decrements a byte in d0 and branches if it is not zero; otherwise it performs a no-operation and returns from an exception.
- `decrement_and_branch`: Decrements a byte in d0, branches if it is not one, otherwise moves d1 to d0, calls another routine, and returns from an exception.
- `setup_stack_frame`: Clears three longwords on the stack, loads a longword from a global variable into a1 and pushes it onto the stack, clears another longword, pushes a pointer to another routine onto the stack, and branches to another routine.
- `push_long_value`: Loads a long value from a global variable into d0, pushes it onto the stack, and returns.
- `restore_stack_and_return`: Sets the stack pointer to the value in a0, subtracts a6 from a6, performs a bitwise AND on the word at the top of the stack, and executes an exception return.
- `compare_and_trap`: Compares d5 with 9, and if d5 is not higher than 9, adds 2 to d0 and executes a trap.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `multiply_and_store_results`: Performs several 16-bit multiplications using values from a structure in a6, performs additions with carry, and stores the resulting long values back into the structure at offsets 8 and 12.
- `perform_division_sequence`: Compares a longword from a structure in a6 with 65535, and if it is not higher, performs a series of word-sized divisions and swaps using values from a6 and d1.
- `loop_bit_shifts`: Performs a loop sixteen times that shifts d1 and d3, subtracts a value from d1 if it is not below zero, and stores the resulting long values into a structure at offsets 8 and 12.
- `multiply_and_subtract_results`: Performs multiple 16-bit multiplications and additions, then conditionally subtracts values from a result based on the signs of d6 and d5 before storing the results in a structure at offsets 8 and 12.
- `process_arithmetic_shift_2`: Performs a series of bitwise rotations, subtractions, and shifts on values loaded from a structure in a6, including conditional logic based on the results of these operations.
- `process_arithmetic_shift`: Subtracts d6 from d4, and if the result is positive and less than 65535, performs a series of subtractions, swaps, shifts, and additions on d4 and d5 in a loop, eventually clearing d4 or d5 based on specific conditions.
- `execute_trap_and_shift`: Compares a word in d5 with 255, and if it is not less than 255, calls another routine, performs several bitwise shifts and rotations on d4, stores the result in a structure at offset 12 of a6, and returns.
- `jump_to_address`: Loads a longword into a0, clears bits 4 through 7 of a byte at offset 4 of the stack, and jumps to the address in a0.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `setup_task_and_trap`: Stores several values into a table, calls a routine using a pointer from a6, performs three library calls via a6, executes a trap, calls two routines using pointers from the stack, and jumps to another routine.
- `jump_to_external_address`: Performs an indirect jump to a longword address stored at a specific location.
- `execute_indirect_jump_2`: Performs a long jump to the address stored at the memory location pointed to by the address in a1.
- `call_library_function`: Loads a pointer from a structure in a6 into a1 and calls a function via a pointer at an offset from a library base in a6.
- `call_old_open_library`: Calls a function via a pointer at an offset from a library base in a6 and stores the returned value from d0 into a structure at offset 12.
- `call_library_function_and_store_result`: Calls a function via a pointer located at an offset of 4 from a global variable and stores the returned value into a field at offset 12 of a structure pointed to by a6.
- `jump_to_external_address_2`: Performs an indirect jump to a longword address stored at a specific memory location.
- `jump_to_pointer_a20`: Performs a long jump to the address stored at a pointer in a1.
- `jump_to_indirect_address`: Performs an indirect jump to a longword address stored at the location pointed to by the address in register a0.
- `clear_global_and_jump`: Clears a longword at a specific address and then performs a long jump to a pointer stored at a specific address.
- `calculate_buffer_offset`: Sets up a stack frame, calls another routine, and then either performs a calculation involving adding 48 to a value and masking it, or performs a comparison before storing a byte into a table at an offset derived from a word at offset -14 of the structure in a6.
- `update_buffer_index_and_call`: Calculates a new byte value by adding sixty-five to a value from offset twenty-six of a structure, then writes this byte to a memory location determined by an index at offset fourteen.
- `update_offset_byte`: Tests a byte at offset -6 of the stack in a6; if it is not zero, it increments a value at offset -14, performs a byte write to a memory location using a calculated index, and optionally clears a word at offset -18.
- `store_byte_with_offset`: Calculates a word by subtracting a value at offset -14 from a value at offset 20 of the stack in a6, stores this result at offset -18 of the stack, and if the value at offset -18 is not less than the value at offset -18, stores a byte from a structure at a pointer in a4 at an offset determined by the calculated word.
- `copy_byte_with_offset`: Calculates an offset using a word from the stack, loads a byte from a structure at a calculated address, and stores it into another structure at an offset determined by a word from the stack; it then loops based on a value in the stack.
- `clear_byte_at_offset`: Compares a 16-bit value at offset -16 of a structure with a 16-bit value at offset 12, and if the first is higher, clears a byte at the address stored in a4 at an offset determined by d5.
- `execute_indirect_jump_3`: Performs an indirect jump to a longword address stored at the memory location pointed to by the address in register a0.
- `execute_indirect_jump`: Performs a long jump to the address stored at a specific memory location.
- `call_library_function_with_d1`: Loads a value from offset 8 of the structure in a6 into d1, calls a function at an offset from a library pointer, and returns.
- `call_library_function_and_store_d0`: Calls a function at an offset from a library pointer and stores the result from d0 into offset 14 of the structure in a6.
- `call_library_function_and_store_result_2`: Calls a function from a library via a pointer at a specific offset and stores the returned value into a global variable at offset 8 of a structure pointed to by a6.
- `call_library_function_and_store_result_3`: Calls a function from a library via a pointer stored in a global variable and stores the returned value from d0 into a field at offset 8 of the structure pointed to by the library base.
- `call_library_function_and_store_byte_d0`: Loads a value from offset 8 of the structure in a6 into d1, calls a function at an offset from a library pointer, and stores the byte from d0 into offset 12 of the structure in a6.
- `call_library_function_with_d1_d2`: Loads values from offsets 12 and 8 of the structure in a6 into d1 and d2, calls a function at an offset from a library pointer, and stores the result from d0 into offset 18 of the structure in a6.
- `call_library_function_via_pointer`: Loads two values from a structure into registers d1 and d2, calls a function from a library base found at a global offset, and stores the return value from d0 into a field at offset 18 of that same structure before returning.
- `call_library_function_and_store_result_4`: Calls a function from a library via a pointer in a global variable and stores the returned value from d0 into a structure at offset 8 of the library base.
- `call_library_function_and_store_long_d0`: Loads a value from offset 8 of the structure in a6 into d1, calls a function at an offset from a library pointer, and stores the result from d0 into offset 12 of the structure in a6.
- `call_library_function_with_d1_d2_d3`: Loads values from offsets 16, 12, and 8 of the structure in a6 into d1, d2, and d3, calls a function at an offset from a library pointer, and stores the result from d0 into offset 20 of the structure in a6.
- `call_library_offset_78`: Loads three values from a structure in a6, calls a routine at an offset of 78 from a pointer in a global variable, and stores the byte from d0 into offset 20 of the structure in a6.
- `call_library_offset_66`: Loads three values from a structure in a6, calls a routine at an offset of 66 from a pointer in a global variable, and stores the longword from d0 into offset 20 of the structure in a6.
- `call_library_offset_180`: Loads two values from a structure in a6, calls a routine at an offset of 180 from a pointer in a global variable, and stores the byte from d0 into offset 20 of the structure in a6.
- `call_library_offset_48`: Loads three values from a structure in a6, calls a routine at an offset of 48 from a pointer in a global variable, and stores the longword from d0 into offset 20 of the structure in a6.
- `jump_to_indirect_address_2`: Performs an indirect jump to a longword address located at the memory address stored in register a0.
- `jump_to_handler`: Performs a long jump to a pointer stored at a specific address.
- `process_structure_updates`: Checks a longword in a6, and depending on its value, calls two different routines to update a structure in a4, eventually setting a word at offset 6 and a byte at offset 4 of that structure.
- `clear_structure_fields`: Clears a longword at offset 8 of the structure in a6 and a word at offset 6 of the pointer stored at offset 8 of the structure in a6.
- `copy_bytes_and_call_handler`: Copies 12 bytes from a pointer in a6 to the stack, then if a value at offset 4 of a structure in a4 is not 2, calls another routine and sets a word at offset 6 of the structure in a4 to a specific value.
- `set_status_flags`: Clears a word at offset 6 of the pointer in a4 and sets a byte at offset 4 of the pointer in a4 to 1.
- `copy_bytes_and_set_status`: Copies 12 bytes from a pointer in a6 to the stack, then if a value at offset 4 of a structure in a4 is not 1, calls two different routines and sets a word at offset 6 and a byte at offset 4 of the structure in a4.
- `process_wait_condition`: Copies 12 bytes from a source to a destination, then enters a loop that increments a word at offset -18 of a structure in a6 while checking a byte in a table at offset 8 of a6 until a condition is met, calls a routine with a pointer and a value, and finally calls another routine to set a byte at offset 4 and a word at offset 6 of a structure in a4.
- `iterate_and_clear_bytes`: Copies 12 bytes from a source to a destination, then enters a loop that calls a routine with a value from offset 12 of a6, calls another routine to process bytes from a structure in a4, and finally calls a routine to set a byte at offset 4 and a word at offset 6 of a structure in a4.
- `execute_conditional_cleanup`: Copies 12 bytes from a source to a destination, then calls a routine with a pointer and a value, and if a word at offset -16 of a6 is not equal to 2, it clears a word at offset -6 of a6, clears a byte at offset 0 of a structure in a3, and calls another routine to set a byte at offset 4 and a word at offset 6 of a structure in a4.
- `handle_single_byte_condition`: Copies 12 bytes from a source to a destination, then calls a routine with a pointer and a value, and if a word at offset -16 of a6 is not equal to 1, it clears a byte at offset 0 of a structure in a3, sets a byte at offset 4 of a structure in a4, and calls another routine to set a word at offset 6 of a structure in a4.
- `clear_word_at_offset_6`: Clears a word at offset 6 of the pointer in a4.
- `copy_and_call_routine`: Copies 12 bytes from a pointer in a6 to the stack using a loop, then calls another routine with a pointer to the stack.
- `check_structure_flag`: Copies 12 bytes from a source to a destination, then calls a routine with a pointer and a longword constant, and if the longword at offset 8 of a structure in a4 is not zero, it sets d5 to 1.
- `clear_byte_at_offset_4`: Sets a byte at offset 4 of the pointer in a4 to 0.
- `update_structure_pointer`: Copies 12 bytes from a source to a destination, then calls a routine with a pointer and two zeroed longwords, and stores the result in a longword at offset 8 of a structure in a6.
- `dispatch_indirect_jump`: Performs an indirect jump to a longword address stored at the location pointed to by register a0.
- `initialize_library_data`: Checks a global longword, calls a routine with a specific parameter, and then jumps to another routine.
- `increment_matching_index`: Compares a word at offset -2 of a structure in a3 with a word at offset 18 of a structure in a4, and if the byte at offset 0 of a structure in a4 (indexed by the word from a3) is non-zero and equals 32, it increments a word at offset -2 of the structure in a4.
- `unlock_stack_pointer_and_return`: Unlocks the stack pointer in register a6 and returns immediately.
- `check_character_matches`: Clears a word at offset -2 and a byte at offset -5 of a structure in a6, calls another routine, and then compares bytes from a table at offset 14 of the structure in a6 against specific values; if they match, it sets d5 to 1 and branches.
- `process_index_and_validate`: Increments a word at offset -2 of a structure in a6, calls another routine, clears a longword at offset 8 of the structure in a6, and then checks if a byte at offset 14 plus an index at offset -2 of a table is a space or zero; otherwise it calls another routine.
- `negate_conditional_longword`: If a byte at offset negative 5 of the structure in a6 is non-zero, it negates the longword at offset 8 of the structure in a6, then sets a byte at offset 20 of the structure in a6 to 1.
- `convert_character_to_value_2`: Fetches a byte from a table using an index at offset -2 of the structure in a6, converts lowercase letters to uppercase or digits to their numeric values, and stores the result in offset -4 and offset -6 of the structure.
- `convert_character_to_value`: Checks if a byte at offset -6 of the structure in a6 is between 65 and 90, and if so, subtracts 65 from it and stores the result at offset -4 of the structure; it then compares this value to a word at offset 12 and may clear a byte at offset 20.
- `accumulate_and_increment`: Calls a routine with a pointer, adds the returned longword to a longword at offset 8 of a structure in a6, and increments a word at offset -2 of a structure in a6 before jumping to another routine.
- `negate_and_set_byte`: If a byte at offset -5 of the structure in a6 is non-zero, it negates the longword at offset 8 of the structure in a6, then sets a byte at offset 20 of the structure in a6 to 1.
- `increment_index_on_condition`: Clears a word at offset -2 of a structure in a6, compares it to a word at offset 12, and if it is not greater, checks a byte at offset 8 of a table; if the byte is non-zero, it increments the word at offset -2 and branches.
- `search_for_period`: Clears a word at offset -4 and a byte at offset -9 of a structure in a6, then loops through a table starting at offset 14 of the structure in a6 to find a period character; if found, it sets the byte at offset -9 to 1 and increments the counter.
- `test_and_jump`: Tests a byte at offset -9 of the structure in a6; if the byte is not zero, it branches to the next instruction, otherwise it performs a relative jump.
- `jump_to_next_routine`: Performs a relative jump to another routine.
- `update_buffer_content`: Compares a word at offset -4 of the structure in a6 with a word at offset 18, and if the first is not greater, stores a byte at offset 0 of a table pointed to by a4; it then performs several arithmetic operations and a loop to update values at offsets -6 and -8 of the structure.
- `copy_byte_from_table_loop`: Calculates an index by subtracting two values from a structure and subtracting one, then copies a byte from a table at a pointer in a6 to a destination at a pointer in a3 using an offset from a6.
- `clear_byte_at_offset_2`: Compares a value from a structure at offset -6 with a value at offset 18; if the incremented value is not greater, it clears a byte at an offset determined by that value within a buffer pointed to by a pointer at offset 14 of the structure.
- `execute_complex_sequence`: Calls several routines to load values, update a structure in a4 using a pointer from a6, and perform conditional logic involving a global variable and a structure at offset 12 of a6.
- `set_global_zero`: Sets a global byte to zero and returns.
- `execute_handler_and_set_status`: Pushes a word from offset 12 of the structure in a6 onto the stack, calls another routine with a pointer from offset 8 of the structure, and then stores 1 or 0 in a global variable based on a test of a word at a specific memory location.
- `update_status_byte`: Calls a routine to check a condition, and if a global byte is non-zero, calls another routine to copy a longword from offset -24 of a structure in a6 to a pointer at offset 8 of a6.
- `call_handler_with_byte`: Pushes a byte from offset 8 of the structure in a6 onto the stack and calls another routine.
- `call_handler_with_byte_2`: Pushes the byte value 10 onto the stack, calls another routine, adjusts the stack pointer, and returns.
- `execute_wait_callback`: Calls a subroutine via a pointer provided in a structure, then updates a global byte with a value based on whether a specific global word is zero.
- `prepare_stack_for_callback`: Calculates the absolute value of a word at offset 10 of a structure in a6, pushes it and other values onto the stack, and calls a routine if the original value was non-negative.
- `invoke_callback_routines`: Calls two different routines using pointers provided via the stack and a structure in a6.
- `jump_to_indirect_address_3`: Performs a long jump to the address stored at the location specified by the address in register a0.
- `initialize_and_copy_data`: Checks a global longword, calls two routines to initialize data, performs two loops to copy two longwords from one location to another using a counter, and then calls two more routines and a final routine via a jump.
- `dispatch_indirect_jump_2`: Performs a long jump to the address stored at the memory location pointed to by the address in register a0.
- `execute_handler_sequence`: Calls several routines, including two that push specific words onto the stack and call other routines, before returning.

### Total Eclipse (1989)(Incentive Software)[cr Black Monks][h ACU].adf:echo

- `process_linked_list_and_update_buffer`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_and_branch`: Moves the contents of d1 to d0 and then performs a relative branch.

### Total Eclipse (1989)(Incentive Software)[cr Black Monks][h ACU].adf:hi

- `execute_relocation_and_jump`: Calls a routine to parse and adjust memory hunk relocations, then performs an absolute jump.
- `process_amiga_hunk_relocations`: Parses a list of memory hunks by comparing hunk type codes and performs relocations by adjusting pointers and values in memory.
- `update_pointer_table`: Increments a value in d6 if d5 is non-zero, then enters a loop that reads a count from a pointer in a3, fetches a table entry based on a value in a5, and performs a series of additions and stores to update a destination buffer.
- `apply_bit_masks`: Performs several bitwise OR operations on d0 and d1 with various constants.
- `unpack_stream_with_pointer_shift`: Decompresses a packed stream by reading bits through a shift routine that takes a 32-bit value from the four bytes before the source pointer, shifts it right and sets its top bit, and writes the resulting bytes to a destination buffer.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bitstream_store_pointer`: Repeats a shift of the bit buffer right by one place for a count in d1, storing the current pointer's low word into a global on each pass.

### Total Eclipse (1989)(Incentive Software)[cr Black Monks][h ACU].adf:0.tec

- `allocate_and_initialize_data_buffer`: Allocates memory, populates a series of global pointers with calculated offsets, and performs a series of bitwise transformations and swaps on a data block.
- `transfer_nested_data`: Checks a bit in a value from a hardware register, decrements a counter in a global variable, and if not zero, performs three nested data transfers using a routine that takes a byte and a mode, then updates several pointers and hardware registers.
- `initialize_and_process_data`: Allocates memory via a library call, performs several library-based operations to manage data, and enters a loop that performs bitwise transformations and word-based transfers to a destination buffer.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_zero_or`: Performs a bitwise OR on d0 with zero.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `clears_data_register_zero_2`: Performs a bitwise OR operation on data register zero with zero.
- `clears_data_register_d0`: Performs a logical OR operation on data register d0 with the constant zero.
- `copy_nested_data`: This routine performs nested loops to transfer data between two memory areas using word-based assignments and bit manipulation.
- `double_word_table_entries`: Doubles each of sixteen words from one global table into a second table, then copies four further words into separate globals.
- `copy_global_block_to_buffer`: Copies a block of 8000 longwords, 32000 bytes, from a global area into the buffer held in a global pointer.

### Total Eclipse (1989)(Incentive Software)[cr Black Monks][h ACU].adf:bootblock

- `initialize_boot_memory_tables`: Clears a block of memory starting at a specific address, then populates several memory regions with structured data patterns using loops.
- `initialize_boot_data`: Executes a loop to write values from a routine into two memory areas, calls several library routines to allocate or manipulate memory, and performs bitwise operations on hardware registers and memory tables.
- `modify_and_branch`: Copies the value from d2 to d0, calls another routine, copies d2 to d0 again, and adds 464 to d0.
- `call_library_functions`: Moves a pointer from a4 to a1, moves d3 to d1, calls a routine at an offset from a6, moves a2 to a0, moves d4 to d0, and jumps to another routine at an offset from a6.
- `clear_bits_in_tables`: Calculates an offset using d1 and d2, then clears a specific bit in two different memory locations at offsets 12288 and 24576 from a5.
- `calculate_and_set_bits`: Performs divisions to calculate two values, then enters a loop that uses those values to set specific bits in two different memory tables at offsets 12288 and 24576 from a5.
- `advance_linear_congruential_seed`: Multiplies a value from the video beam position register by a stored seed, adds a fixed constant, and stores the result back into the seed location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_system_state`: Calls a library routine to get a value, updates several global variables, and enters a loop that performs conditional calls to another routine while modifying global 16-bit and 32-bit values based on hardware register status.
- `poll_fire_button`: Checks bit 6 of a hardware register and increments a counter in d2 up to a maximum value if the bit remains set.
- `configure_audio_hardware`: Sets several hardware registers for audio control and enters a loop that runs until d1 reaches a specific value, then sets a hardware register to 1.
- `configure_blt_and_offset`: Sets several Blitter registers, checks a bit in a status register, and calls another routine three times while incrementing and decrementing memory pointers.
- `set_blt_size`: Writes a value to a hardware register and checks bit 14 of another hardware register, branching if it is set.
- `execute_multiple_blt_transfers`: Sets Blitter control and address registers and calls another routine three times with different source and destination addresses.
- `set_blt_size_alternative`: Writes a different value to a hardware register and checks bit 14 of another hardware register, branching if it is set.
- `update_global_tables`: Calls two routines via a6, then updates several global variables with 16-bit values derived from 32-bit constants using a swap operation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block`: Clears a block of memory by writing zero to 111 words starting at a specific address and then updates a global variable with the value of another global variable.
- `process_keyboard_input`: Reads a byte from the keyboard serial register, performs arithmetic and bitwise checks on the value, and stores the result at an offset from a pointer if it meets specific criteria.
- `store_and_restore_registers`: Stores a byte from a0 and d0 into memory, restores registers from the stack, and jumps to a fixed address.
- `process_conditional_bytes`: Iterates through a memory range, performing bitwise operations and copying bytes based on specific conditions and a counter.
- `clear_memory_range`: Fills a range of 111 words starting at a specific memory address with zeros.
- `free_and_clear_memory`: Stores three longwords from registers into global variables, calls two other routines, and if a specific global word is not zero, updates a global longword, clears a global longword, decrements a pointer in a1, and calls another routine using a pointer from a6.
