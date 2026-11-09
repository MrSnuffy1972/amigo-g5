# Game 1243 (obsession pinball (1995)(unique development studios)[cr](disk 1 of 2)[t +2 atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01243). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902336 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 369 of 369 named (0 author symbols), 369 of 369 described (15 by rule, 353 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1363 of 1363 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 644506 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6876 of 6876 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 65250 |
| code | 24298 |
| data | 721849 |
| padding | 4923 |
| unused | 86016 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| MuchMore | 3 | 3727 | 141 |
| Echo | 2 | 117 | 2 |
| Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:Obsession | 5 | 2568 | 207 |
| Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:ATX-TRAINER.EXE | 2 | 235 | 8 |
| Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:HELLFIRE | 1 | 19 | 3 |
| Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 49 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 148 | 1 |
| Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:XILE.BIN (raw code file, load address unknown) | 1 | 0 | 0 |
| Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:DRUN.BIN (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- obsession pinball (1995)(unique development studios)[cr](disk 1 of 2)[t +2 atx]/Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:XILE.BIN: high-entropy data file (7.65 bits/byte), classified as packed data
- obsession pinball (1995)(unique development studios)[cr](disk 1 of 2)[t +2 atx]/Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:DRUN.BIN: high-entropy data file (7.67 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 113 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- obsession pinball (1995)(unique development studios)[cr](disk 1 of 2)[t +2 atx]/Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:XILE.BIN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- obsession pinball (1995)(unique development studios)[cr](disk 1 of 2)[t +2 atx]/Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:DRUN.BIN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- obsession pinball (1995)(unique development studios)[cr](disk 1 of 2)[t +2 atx]/Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:ATX-TRAINER.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 644506 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:ATX-TRAINER.EXE

## Copy protection found

- `process_data_and_jump` (MuchMore): Saves registers to a structure at offset 608 of a5, performs several calls with arguments from a5 and stack-based values, executes two jumps, and enters a loop that sets bytes in the structure at offset 608 based on a counter in a structure at offset 1680 of a5, finally performing an indirect jump. Kind: checksum/integrity trap. Pass path: the indirect jump at the end of the routine.

## Functions


### MuchMore

- `jump_to_address_4`: Performs a jump to an address stored in a long word.
- `initialize_font_structure`: Allocates memory using the exec library, and if successful, populates a structure with a specific size, a pointer to another routine, and a stack pointer before calling another routine.
- `free_allocated_memory`: Releases a previously allocated memory block whose size is in d3 by calling the exec library, then restores the original value of d0 and returns.
- `increment_until_zero`: Increments a counter starting at -1, returning it once it exceeds 255 or once the byte at that offset in the structure in a3 is zero.
- `copy_string_data`: Increments a counter, then copies bytes from a source structure to a destination structure as long as the source byte is non-zero and the counter is less than 256 and less than a value at offset 1392 of a structure in a5.
- `clear_and_jump_table`: If a 16-bit value on the stack is not greater than 256, it clears a byte at an offset in a structure pointed to by a stack pointer, then performs an indirect jump using a pointer from the stack.
- `initialize_system_state`: If a pointer in a3 is zero, it sets a pointer to a global variable, then if a byte at offset 7380 of a structure in a5 is zero, it performs several memory moves and calls three library routines before jumping to a pointer in a structure in a5.
- `increment_and_allocate_block`: Increments a 16-bit counter stored in d7, adds it to a running total at offset 7328 of the structure in a5, and stores the result back when the total stays below 4096.
- `free_list_entries_loop`: While the pointer at offset 5284 of the structure in a5 is not null it saves that pointer, calls a routine to free the pointed-to block, restores the next pointer from the freed entry, and repeats, then writes the value 4096 to the word at offset 5280 of the structure before returning.
- `call_routines_conditional`: Calls three routines from function pointers in the structure in a5, then calls a fourth routine passing the byte at offset 4576 if that byte is not zero.
- `call_twice_clear_words`: Calls the routine at offset -300 twice with a doubled value from offset 4900, then clears the word at offset 1688 and the word at offset 10 in the structure pointed to by offset 5848.
- `process_io_and_jump`: Compares two long values at offsets 1376 and 1380 of a structure in a5, calls a routine via a pointer at offset 7464 of a5, and then performs an indirect jump using a pointer popped from the stack.
- `process_keyboard_input_2`: Builds a sequence of keyboard scan codes from status flags and a numeric string, then calls the keyboard handler for each code.
- `call_subroutine_with_multiplication`: Checks a byte at offset 7320 of a structure in a5, and if non-zero, multiplies a word from offset 1386 of a5 by a word from the stack, then calls a routine with a pointer and several values from a5.
- `update_bitfield_and_call`: Sets a 16-bit value on the stack, performs arithmetic using a pointer from a5, calls two different routines, and then manipulates a byte from the stack based on a comparison with a value from a table.
- `modify_stack_byte_2`: Compares the byte in d7 to 18 and, if equal, clears bit 1 and sets bit 2 of the byte at the top of the stack before branching to another routine.
- `modify_stack_bits`: Compares d7 to 19 and, if equal, sets bit 1 and clears bit 2 of the byte at the top of the stack before branching.
- `modify_stack_byte`: Compares d7 to 20, and if equal, loads a byte from the stack into d7, performs a bitwise OR with 6, stores the result back on the stack, and branches to another routine.
- `compare_and_set_bit`: Compares a byte in d7 to 21 and, if equal, sets bit 0 of a byte on the stack, then branches to another routine.
- `clear_bit_on_match`: Compares a byte in d7 to 22 and, if equal, clears bit 0 of a byte on the stack.
- `update_bitfields`: If a byte in a stack parameter is between 2 and 15, it performs arithmetic on the byte and stores the results in two different offsets of the stack.
- `execute_callback_sequence`: If a byte at an offset in a structure in a4 is at least 32, it calls four library routines using values from a structure in a5 and a stack, then performs arithmetic on a stack value and branches back to the start.
- `jump_via_stack`: Loads a pointer from the stack and jumps to it after incrementing the stack pointer by 6.
- `copy_and_process_memory_blocks`: Calculates a value by adding words from a5, calls a routine, and then performs multiple memory copies between destination addresses derived from a structure in a5 and a pointer in a4.
- `call_routines_conditional_2`: Increments a counter in a structure in a5, then if a bit is set in a stack value, it calls another routine and copies multiple words between two arrays before calling a final routine via a jump table.
- `copy_and_process_stack_data`: Copies 67 words from a pointer in a5 to the stack, calls another routine, updates a 16-bit value at offset 5814 of a5, and if a condition is met, decrements that value and modifies a byte in a stack-based table before calling another routine and performing an indirect jump.
- `guarded_call_return`: Sets a flag at offset 5792, saves a value from offset 5788, calls the routine at offset -792, clears the flag, and returns the inverse of whether the pointer at offset 80 is non-zero.
- `save_register_and_update_pointer`: Saves register d7 on the stack, calls another routine, and if the result in d0 is not zero, updates a pointer at offset 80 of a5, increments a counter at offset 1404, adds a value from offset 8 of the new pointer to offset 1680, and if a comparison between offset 1384 and offset 1404 fails, updates a pointer at offset 76 and calls another routine.
- `pop_linked_list_node`: Updates a pointer in a structure to the next element in a linked list and decrements a counter in a global structure.
- `decrement_and_copy_memory_blocks`: Decrements a word at offset 1688 of a5, calls a routine, and then performs multiple memory copies between destination addresses derived from a structure in a5 and a pointer in a4.
- `execute_function_pointer_table`: Updates four pointers in a structure using a value from a fifth, calls a routine if a byte in a structure is non-zero, and then enters a loop that copies bytes between two memory locations and calls a routine via a jump table.
- `multiply_and_decrement_counter`: Checks a word at offset 7320 of a structure in a5; if it is zero, it multiplies a word from offset 1386 of a5 by a word from offset 1384 of a5, stores the result at offset 10 of a pointer from offset 5848 of a5, and decrements the value at offset 7320 of a5.
- `shift_add_and_copy_blocks`: Pushes the pointer at offset 72 of the structure in a5 on the stack, calls a routine, then adds offset 1400 to the value at offset 8 of the pointer at offset 4 and adds offset 1396 to the value at offset 12 of the same pointer, storing both results back.
- `update_string_buffer`: Calculates a new index by adding a word from the stack to a word at offset 14, then calls another routine to compare values.
- `jump_via_stack_offset`: Loads a pointer from the stack after incrementing the stack pointer by 6 and jumps to it.
- `accumulate_values_in_loop`: Clears two longwords in a structure in a5, then enters a loop that increments a longword at offset 1468 and adds a sign-extended word from a table at offset 8 of a3 to a longword at offset 1648 until a counter in a3 reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `advance_linked_list_pointer`: Iterates through a linked list starting from a pointer in a structure, updating the structure's pointer to the next element until a limit is reached or a null pointer is encountered.
- `process_conditional_pointer_loop`: Compares a word at offset 568 of a structure in a5 with d7, and if d7 is greater than or equal, it branches; otherwise, it checks if a pointer at offset 4 of a3 is null, and if so, branches; if not null, it loads that pointer into a3, increments d7, and loops back to the start.
- `process_structure_data_in_loop`: Stores a pointer from a3 into offset 80 of a structure in a5, calls three different routines, and then enters a loop that runs from 1 up to a value stored at offset 1384 of the structure, performing a series of register saves, calls, and bitwise shifts on a value derived from the structure's offset 80, before setting a byte at offset 5816 to zero.
- `set_status_flag`: Sets the sign flag bit in a byte at offset 5816 of the structure in a5.
- `guarded_jump_with_status`: If a pointer at offset 12 in a5 is non-zero, it sets a byte at offset 5793, calls a routine via a pointer in a structure, and sets the sign flag based on the result before performing an indirect jump.
- `copy_table_to_structure`: Copies 67 words from a source table to a destination structure, updates a pointer in a structure in a5, and then enters a loop to copy bytes from a table into a structure based on values from the stack.
- `update_structure_from_table_2`: Calculates a value by subtracting a byte from 17, stores it in a structure, and then enters a loop that copies bytes from a table into that structure.
- `update_and_dispatch_structure`: Copies bytes from a source table into a destination structure based on values from the stack, updates two pointers within that structure if they are non-zero, and then performs an indirect jump using an address from the stack.
- `update_structure_pointers`: Updates two pointers within a structure by copying values from a source table into specific offsets, then performs an indirect jump using an address from the stack.
- `update_structure_pointer`: Checks a byte at offset 522 of a structure in a4; if it is zero, it calls a routine via a pointer in a5, then updates offset 80 of the structure in a5 with a value from a pointer at offset 76 of a5 or its offset 4.
- `update_structure_and_jump`: Copies a pointer from offset 76 of a structure in a5 to offset 80 of the same structure, calls another routine, then checks a byte at offset 522 of a pointer in a4; if that byte is non-zero, it calls another routine and performs an indirect jump using a pointer from the stack.
- `update_conditional_structure_word`: Clears a word at offset 12 of a structure pointed to by a pointer on the stack, then compares a word at offset 8 of a structure pointed to by a pointer in a5 with a word at offset 1392 of that same structure; if the comparison fails, it copies the word from offset 8 of the structure in a3 to offset 6 of the structure in a4, otherwise it returns.
- `search_and_update_string`: Copies a 16-bit word from offset 1392 of a structure in a5 to offset 6 of a structure pointed to by a4, then searches for a character match in a table pointed to by a5, incrementing counters and calling other routines if specific character conditions or null terminators are met.
- `increment_structure_field`: Loads a pointer from the stack, increments a 16-bit word at offset 12 of that pointer, and branches to another routine.
- `dispatch_indirect_address`: Loads an address from the stack and performs an indirect jump to it.
- `process_data_and_jump`: Saves registers to a structure at offset 608 of a5, performs several calls with arguments from a5 and stack-based values, executes two jumps, and enters a loop that sets bytes in the structure at offset 608 based on a counter in a structure at offset 1680 of a5, finally performing an indirect jump.
- `update_structure_and_call_subroutine`: Increments a 16-bit value in a structure, performs a calculation involving multiplication and bit shifting to update a field, and populates a local data block with several constants and pointers.
- `initialize_string_table_loop`: Calls another routine repeatedly with various pointers from a data table, then calculates a loop count from a value in a structure and executes a loop that calls the same routine with a null pointer until the count reaches zero.
- `loop_message_processing`: Calls two routines using pointers from a structure in a5, stores the result in offset 5808 of a5, and enters a loop that calls another routine if a bit is set in offset 22 of a structure and a word at offset 24 is less than 128.
- `reply_and_advance_pointer`: Calls a routine using a pointer from a global variable, updates offset 80 of a structure in a5 with the value from offset 76 of the same structure, calls another routine, and performs an indirect jump using a pointer from the stack.
- `traverse_linked_list`: Calls another routine, copies a pointer from offset 88 to offset 80 of a structure in a5, and enters a loop that updates offset 80 of a5 with the value from offset 4 of the pointer stored there until a word at offset 12 of a structure in a4 reaches 1.
- `call_and_jump_2`: Calls another routine and performs an indirect jump using a pointer from the stack.
- `store_byte_conditionally`: Stores a byte from offset 66 of a structure in a4 into offset 5785 of a structure in a5, then checks a long value at offset 12 of the structure in a5; if that value is zero, it checks a pointer at offset 80 of the structure in a5, and if that pointer is non-zero, it checks a byte at offset 5 of a structure in a4, otherwise it sets d0 to 255 and branches.
- `store_byte_and_branch_2`: Loads a pointer from the stack into a4, stores a byte from the stack at offset 5 of that pointer, and branches.
- `toggle_state_and_call_handler`: Checks a byte at offset 4 of a structure; if zero, it sets that byte and a byte at offset 5816 of a global structure to 255.
- `set_byte_and_jump`: Stores a byte from a pointer in a stack parameter into offset 4 of a structure in a4 and into offset 6840 of a structure in a5, then performs an indirect jump using a pointer from the stack.
- `store_byte_and_set_flag`: Copies a byte from a structure in a4 to a structure in a5, then checks a byte in a4 and updates a byte in a5 based on the status register.
- `set_byte_and_jump_2`: Stores a byte into offset 6840 of a structure in a5, sets the byte at offset 4 of a structure in a4 to zero using the status register, and then performs an indirect jump using a pointer from the stack.
- `synchronize_status_and_wait`: Clears two bytes in a stack-allocated structure, calls a routine via a pointer in a4, and checks a byte at offset 6856 of a structure in a5 to decide whether to call another routine or branch.
- `store_byte_and_branch_3`: Stores a byte from an offset of 22 in the structure pointed to by a5 and branches to another routine.
- `update_status_and_branch`: Moves a byte from an offset of 25 in the structure pointed to by a5 to the stack, calls another routine, loads a pointer from an offset of 76 in a5 into a4, tests the longword at offset 4 of that pointer, sets bit 7 of a byte at offset 22 in the structure in a5, and branches to another routine.
- `process_message_and_update_status`: Retrieves two messages from a structure, stores the second message pointer in a global variable, and if a specific bit is set in the message, clears a bit in a status byte, calls another routine, and then sets the bit back.
- `copy_data_and_post_message`: Copies thirteen words from a structure in a4 to a stack-based buffer using a loop, calls a library function with a pointer to the structure, and then updates several status flags in a structure based on bitwise tests of values retrieved from the stack and the structure.
- `process_input_state_flags`: Checks a value on the stack and, if it matches a specific constant, updates several status flags in a structure based on bitwise tests of a word at offset 40 of the stack.
- `call_handler_on_bit`: Tests bit 5 of a byte at offset 32 of the stack and, if set, calls another routine before branching.
- `check_bits_and_call_routine`: Tests bit 2 of a value at offset 36 of the stack, and if it is 0, compares a word at offset 38 of the stack to 128; if the word is less than 128, it compares it to 64, and if it is not equal, pushes the stack pointer to the stack and calls a routine; if the result of that call in d0 is not 0, it branches to another routine.
- `compare_value_and_call_routine`: Compares a word in register d7 to 65, and if it is not equal, pushes the stack pointer to the stack, calls a routine, and branches to another routine.
- `process_input_command_id`: Compares a value in d7 against several constants and, depending on the match, updates a structure pointed to by a5 or modifies values on the stack before calling other routines.
- `store_bytes_and_branch`: Stores two bytes from a structure in a5 and a byte from the stack into another location, then branches to another routine.
- `decrement_stack_word_conditionally`: Checks a byte at offset 67 of the stack, and if it is not zero, it compares a word at offset 38 of the stack with 31; if that word is not 31, it branches; otherwise, it loads a word from offset 568 of a structure in a5 into d7, decrements d7, and writes the result to offset 12 of the stack.
- `set_status_and_call`: Writes a word to the stack, checks a byte at offset 66 of the stack, sets the sign flag based on a byte at offset 66, calls another routine, decrements a word on the stack, and if that word is non-zero branches to an earlier instruction; otherwise it sets a sign flag at offset 5816 of a5 and branches.
- `filter_input_command_id`: Compares a value in d7 against several constants and, if it matches specific values, performs conditional updates to a structure pointed to by a5 or a7, including setting a byte at offset 5816 of a5 based on the value at offset 5817 of a5.
- `store_bytes_and_branch_2`: Stores two consecutive bytes into a structure pointed to by a5, sets the carry flag based on the sign of a byte at an offset from a7, and branches to another routine.
- `decrement_stack_word_variant`: Checks a byte at offset 67 of the stack, and if it is not zero, it compares a word at offset 38 of the stack with 63; if that word is not 63, it branches; otherwise, it loads a word from offset 568 of a structure in a5 into d7, decrements d7, and writes the result to offset 12 of the stack.
- `set_status_and_call_variant`: Writes a word to the stack, checks a byte at offset 66 of the stack, sets the sign flag based on a byte at offset 66, calls another routine, sets a sign flag at offset 5816 of a5, decrements a word on the stack, and if that word is non-zero branches to an earlier instruction; otherwise it branches.
- `test_and_set_flag`: Compares d7 to 67 and 68, and if d7 is 67, calls another routine and sets the sign flag based on a value at an offset of a5 before branching to another location.
- `update_structure_and_call`: Compares d7 to 61 and, if not equal, copies a pointer from offset 72 of a5 to offset 80 of a5, calls another routine, and then branches to another location.
- `branch_on_value`: Compares a word in d7 to 29 and, if not equal, pushes the address of a stack frame and calls another routine before branching to another location.
- `update_linked_list_from_table`: Checks if a value in d7 is between 80 and 89, calculates an index by subtracting 80 and shifting left by two, then retrieves a pointer from a table and updates a global structure with the next element in a linked list before calling another routine.
- `write_to_offset_table`: Shifts a 16-bit word from the stack left by two bits, then uses it as an offset to store a 32-bit word from a structure into another structure.
- `compare_and_call_subroutine`: Compares d7 with 95 and branches if not equal, otherwise it pushes the address pointed to by the stack pointer and calls another routine before branching to another routine.
- `test_character_and_byte`: Compares d7 with 69 and branches if equal, otherwise compares d7 with 46 and branches if not equal, then checks if a byte at offset 67 of the stack is non-zero to branch to another routine, otherwise it branches to another routine.
- `validate_table_index_and_update_pointer`: Compares a word at offset 38 of the stack to 64 and, if less than 64, loads a byte from a table using an index from that word and an offset from a5 into offset 523 of the stack.
- `compare_byte_and_call_routine`: Compares the byte in d7 to ninety-eight and, if equal, pushes the address pointed to by the stack pointer and calls another routine before branching to another routine; otherwise it returns.
- `normalize_character_case_in_structure`: Checks if a byte in d7 is one of three specific values and, if so, verifies a character at a specific offset in a stack-based structure.
- `increment_stack_offset_in_loop`: Loads a 16-bit value from the stack into d7, tests a byte at offset 72 of the stack plus d7, and if that byte is zero, increments the 16-bit value at offset 102 of the stack and branches back to the start of the routine.
- `remove_linked_list_node`: Compares a longword at offset 80 of a structure with a longword at offset 6900 of the same structure; if they are equal and the value is not zero, it updates the pointer at offset 80 to the next node in the list and decrements a counter at offset 12 of a stack-based structure.
- `update_linked_list_node`: Compares a longword at offset 80 of a structure with a longword at offset 6900 of the same structure; if they are equal and the value is not zero, it updates the pointer at offset 80 with the value from offset 76, then conditionally calls another routine and updates the pointer again by reading from the address stored at the new offset 80.
- `update_linked_pointer_and_call`: Updates a pointer at offset 80 of a structure with a value from offset 4 of the structure it points to, then calls another routine if the new pointer is non-zero.
- `clear_and_call_subroutine`: Clears a longword at offset 80 of the structure in a5, pushes the current stack pointer onto the stack, calls a subroutine, and then branches to another routine.
- `accumulate_linked_list_values`: Checks if a byte in d7 equals 119 and verifies several conditions in a structure pointed to by a5; if met, it clears two 32-bit values in the structure, traverses two linked lists to sum values from offsets 8 and 4 into the cleared locations, and adds a signed 16-bit value from a third list to a structure offset.
- `add_extended_word_to_global`: Loads a pointer from offset 6880 of a structure in a5 into a4, fetches a pointer from offset 4 of the structure pointed to by a4, and if that pointer is not zero, adds the sign-extended 16-bit word from offset 8 of the second structure to a global variable at offset 6820 of the structure in a5.
- `process_counter_callback_list`: Iterates through a linked list of structures, decrementing a 16-bit counter in each and adding a 32-bit value from the structure to a global accumulator until the counter reaches one or the list ends.
- `decrement_counter_and_execute_callback`: Subtracts a value from a 32-bit counter in a structure, and if the result is positive, it pushes a stack frame, calls two subroutines, updates a pointer in the structure, and executes a function via a pointer in a global table.
- `handle_event_type_o`: Checks if a byte in d7 equals 111; if so, it performs several stack-based parameter setups and calls three subroutines, sets a byte at offset 5795 of the structure in a5, and calls a function via a pointer in a4.
- `process_event_type_conditional`: Checks if a byte in register d7 equals 108; if so, it calls two subroutines, sets a byte at offset 5796 of the structure in a5, and then calls two more subroutines via function pointers stored in the structure before performing a bit test and conditional branch.
- `decrement_linked_list_node`: Checks if a byte in register d7 equals 103, then pushes a value onto the stack and calls a subroutine.
- `branch_to_other_routine`: Calls another routine and then performs a branch to another routine.
- `call_subroutine_on_match`: Compares the byte in register d7 to 104; if they are equal, it pushes the current stack pointer to the stack, calls a subroutine, and then branches to another routine; otherwise, it branches to another routine.
- `handle_input_key_events`: Checks if a byte in register d7 matches specific character values, then updates a value at offset 5794 of a structure in a5, loads a pointer from offset 5800 into a4, and calls two subroutines via a table in a6 using values from offsets 5788 and a4.
- `clean_and_validate_path`: Trims trailing slash or colon characters from a path string in a structure, then copies the cleaned path into two separate buffers.
- `setup_font_from_args`: Increments a 16-bit counter in a structure and, if it is one, opens diskfont.library, processes command-line arguments to configure a font, and parses a string into a buffer while handling quoted text and whitespace.
- `release_resource_management_structure`: Decrements a 16-bit reference counter in a structure and, if it reaches zero, proceeds to release several resources including a library, a task, a signal, and multiple pointers by calling their respective cleanup routines.
- `increment_open_library_dispatch`: Increments a 16-bit counter at offset 7344 of a structure in a5 and, when it equals one, calls four routines in turn, then opens a library and stores its result at offset 7340, clearing offset 7364 before jumping through a pointer at offset 7356.
- `decrement_and_close_library_3`: Decrements a 16-bit word at offset 7344 of a structure in a5, and if the result is zero, checks a pointer at offset 7404 of the structure, then calls four different routines and jumps to another routine.
- `increment_and_call_when_one`: Increments the 16-bit counter at offset 7352 of the structure in a5 and, while that counter equals one, calls a routine whose address is stored in the structure.
- `decrement_and_jump`: Decrements a 16-bit word at offset 7352 of the structure in a5, and if the result is not zero, branches to another routine; otherwise it jumps to another routine.
- `multiply_signed_values`: Takes two signed 32-bit values, makes both non-negative, multiplies them as unsigned 16-bit halves and returns the 32-bit result, or returns zero when either low half is zero.
- `calculate_signed_division`: Performs signed 32-bit division by using a loop to repeatedly double a value while halving another, or uses a sequence of swaps and 16-bit divisions if the divisor is less than 65536.
- `calculate_shifted_product`: Moves a 32-bit value from d1 to d3, copies d0 to d1, clears d0 and d2, adds d3 to d3, increments d2 by 1, and branches based on a comparison between d1 and d3.
- `compute_shifted_product`: Repeatedly doubles a value while halving another until the second value is exhausted, decrementing a counter each pass, then adjusts the sign and magnitude of the accumulated result before returning it.
- `allocate_and_link_structure`: Allocates memory using a library call, and if successful, sets a field in the new structure to a value from a stack parameter and calls another routine before performing an indirect jump.
- `free_and_clear_structure`: If a pointer in a stack parameter is not zero, it calls a library routine to remove an item and another to free memory, then clears the byte at offset 8 of the structure pointed to by the parameter before performing an indirect jump.
- `set_value_call_routine`: Sets the 32-bit value at offset 7364 of the global structure in a5 to all ones, calls the routine whose address is stored at offset 7372 of that structure, and returns.
- `store_function_pointer`: Loads a global structure pointer into a4 and derives a second pointer from offset 114, then a third from offset 2e.
- `init_callback_list`: Increments a reference counter in a global structure and, while the counter equals one, initializes a linked list node, stores a code pointer and callback data into a referenced structure, then invokes two stored function pointers before returning.
- `increment_and_call_when_one_2`: Increments the 16-bit counter at offset 7444 of the structure in a5 and, while that counter equals one, calls a routine whose address is stored in the structure.
- `decrement_and_jump_copy`: Decrements a 16-bit word at offset 7444 of the structure in a5, and if the result is not zero, branches to another routine; otherwise it jumps to another routine.
- `increment_call_and_open_library`: Increments the 16-bit counter at offset 7452 of the structure in a5 and, while that counter equals one, calls a routine whose address is stored in the structure, then loads an address from a global table into a4, calls an operating system library routine, and stores the returned value at offset 7448 of the structure.
- `decrement_and_close_library`: Decrements a 16-bit word at offset 7452 of the structure in a5, and if the result is not zero, continues; otherwise it checks a pointer at offset 7448 of the structure in a5, and if it is not zero, loads it into a1, loads a pointer from offset 4 into a6, calls a routine at an offset from a6, and jumps to another routine.
- `increment_counter_open_graphics_library`: Increments a 16-bit counter at offset 7460 of the structure in a5 and, if the counter equals one, calls a helper routine, opens graphics.library, stores the returned handle at offset 7456, and if the handle is non-zero, clears a pointer at offset 7364 before jumping to a function pointer at offset 7356.
- `decrement_and_close_library_2`: Decrements a 16-bit word at offset 7452 of a structure in a5 and, if the result is not zero, continues; otherwise it checks a pointer at offset 7464 and, if not zero, calls a library routine and jumps to an offset of the structure in a5.
- `manage_library_lifecycle`: Increments a 16-bit counter at offset 7468 of a structure in a5 and, when it equals one, calls two routines, opens a library, and stores the result at offset 7464; otherwise, it decrements the counter and, if it reaches zero, calls a library routine and jumps to another routine.
- `initialize_and_process_data_stream`: Clears two destination memory areas by copying zeroed longwords in a loop based on a stride value from a structure in a1, then reads a byte from a0 to perform a series of conditional jumps and arithmetic operations.
- `calculate_scaled_offset`: Subtracts 17 from a byte in register d0, extends it to a word, and performs a series of arithmetic and shift operations to calculate a new value in register d4 before branching to another routine.
- `calculate_jump_offset`: Extends the sign of a byte in d0, performs arithmetic to multiply it by six, and performs an indirect jump using the result as an offset.
- `load_address_and_branch_2`: Loads the address at a PC-relative offset into a5 and then performs a short branch to an earlier instruction.
- `branch_to_offset_3`: Performs a short branch to an offset of 3124.
- `calculate_and_jump_to_offset`: Subtracts thirty-two from a byte in d0, masks it to eight bits, shifts it left by three, adds d4 to it, and performs an indirect jump to an address calculated by adding an offset of four thousand six hundred eight to a pointer located at an index derived from d0 and a2.
- `branch_to_earlier_instruction`: Performs a short relative branch to an earlier instruction.
- `scatter_bytes_with_stride`: Copies seven bytes from a source pointed to by a3 into a destination pointed to by a4, inserting a word-sized stride defined by d2 between each byte, then applies a bitwise OR to the final byte and branches to another routine.
- `increment_and_branch`: Increments the 32-bit values in a1 and a4 and branches to an earlier instruction.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.
- `expand_compressed_bitplanes`: Expands a compressed bitmap stream into eight planar bitplanes by shifting and OR-ing each source byte, advancing the source pointer by a value in d7 each step, then inverting the 1536 bytes following the destination buffer.

### Echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 into d0 and then branches to another location.

### Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:Obsession

- `jump_to_indirect_address`: Performs an indirect jump to an address stored in a memory location relative to the program counter.
- `jump_to_address_3`: Jumps to the address stored at offset 56 of the current program counter relative address.
- `execute_conditional_sequence_of_calls`: Calls a routine, compares a word in a global variable to negative two, and if not equal calls three more routines and a fourth routine at a relative offset from the program counter.
- `adjust_stack_and_return_2`: Increments the stack pointer by four and returns using a return address pushed onto the stack from a location relative to the program counter.
- `branch_and_call_routine`: Branches to another routine, calls a routine at an offset from a6, and then branches to another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `call_and_test_long_value`: Calls a routine, tests a long value at offset 2 of the stack, and if it is 0, branches to another routine; otherwise, it checks if the value is 0 and branches to another routine or a third routine depending on the result.
- `branch_forward`: Branches to an instruction at a relative offset of 68.
- `call_function_via_pointer`: Pushes the contents of a0 onto the stack, loads the constant 4384 into a0, and calls a routine at an offset of 14 from a0.
- `adjust_stack_and_jump_2`: Increments the stack pointer by 4 and jumps to a relative offset of 420.
- `call_subroutine_via_stack`: Pops a pointer from the stack into a0, calls the routine at the address in a0, and returns.
- `dispatch_indirect_jump_and_restore_stack`: Saves the contents of register a0 onto the stack, jumps to a routine at an offset of 88 from a constant, restores the value from the stack into register d0, loads a pointer from a global variable into register a1, branches to another routine, and otherwise returns after pushing a constant into the stack.
- `jump_to_address`: Jumps to the address stored at offset 56.
- `clear_and_jump_table_2`: Clears a word at a global variable, pushes the value of a0 onto the stack, loads the constant 258 into a0, and jumps to an offset of a0.
- `push_and_return`: Branches to a relative offset of 152, pushes the constant value 206 onto the stack, and returns.
- `update_task_pointer_and_call_handler`: Updates a pointer in a global variable using a value from a structure, sets a field in that structure to a constant, and calls another routine.
- `process_data_and_call`: Moves a word from a global variable to d0, loops using d0 as a counter, branches, moves d0 to a global variable, pops a value into d0, calls a routine, pops a value into a0, and branches to a routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies 74 to -(a7), then returns.
- `store_value_to_structure`: Loads a pointer from the stack into a0, stores the value of a global variable at offset 184 of the structure pointed to by a0, and returns.
- `jump_to_offset_table`: Clears register d7, pushes the current stack pointer to the stack, and performs an indirect jump to an address at offset 46 of a table pointed to by a0.
- `load_and_call_library`: Loads the address at offset 92 of a structure in a2 into a0, calls a routine at an offset from a6 twice, and then branches based on the sign of the result in d0.
- `check_status_and_dispatch`: Calls another routine, compares a word at an offset of a pointer in d1 to specific values, and if certain conditions are met, pushes a0 to the stack and jumps to a routine at an offset from a global variable.
- `call_exec_function`: Increments the stack pointer by 4, loads a pointer from a global variable into a6, calls a routine at an offset of -132 from a6, and branches to a relative offset of 546.
- `resolve_task_pointer`: Adjusts the stack pointer, loads a pointer from the stack into a0, branches to a relative offset of -54, loads a global variable into a6, calls a routine at an offset of -294 from a6, moves the result into a0, pushes a0 onto the stack, loads the constant value 234 into a0, jumps to an address at an offset of 64 from a0, and clears a 16-bit value at a global variable.
- `process_task_and_jump`: Loads a pointer from d0 into a2, checks a value at an offset of a2, and if not zero, performs a loop 44 times to push a value onto the stack, then calls a routine at an offset from a global variable and branches based on the result.
- `push_address_and_return`: Pushes the address at offset 96 of the program counter onto the stack and returns.
- `pop_and_call_routine`: Pops a longword from the stack into a0, pushes the constant 610 of the program counter offset onto the stack, and returns.
- `adjust_stack_and_call_handler`: Increments the stack pointer by 4, clears register a1, loads a pointer from a global variable into a6, calls a routine at an offset from a6, and branches to another routine.
- `dispatch_and_disable_interrupts`: Clears a byte at offset 10 of a global structure, then branches to a shared routine when a status word at offset 400 equals one, three, or five, otherwise it dispatches through a routine table and eventually sets three bytes to two before disabling interrupts and returning.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `resolve_dos_library_entry`: Resolves the dos.library entry point table, calls the routine at offset 120 of that table, and if the result is nonzero calls the routine at offset 138 and clears the byte at offset 142 of the structure in a1, otherwise it sets that byte to all ones and returns.
- `resolve_dos_entry`: Clears a byte at offset 146 of a global structure and stores the address of that global into a second global at offset 12.
- `resolve_dos_entry_2`: Clears a byte at offset 93 of a global structure and stores the address of that global into a second global at offset 12.
- `query_library_store_pointer`: Queries a library base into d0, loads a table address into a1, and if the query result is not negative one it calls another routine and stores the unchanged value from d7 into a global at offset 404.
- `open_and_query_library`: Calls a routine to obtain a library base address, then uses that address to call an offset to check for library existence.
- `execute_conditional_logic`: Calls two routines, compares a byte at a global variable to 255, branches if not equal, calls another routine, calls a third routine, increments a byte at a global variable, compares it to 50, branches if not equal, calls a fourth routine, calls a fifth routine, clears a byte at a global variable, and branches.
- `wait_raster_loop_call`: Loads a pointer into a0, queries a library base into d0, and if the result is nonzero it sets d1 to 100 and calls another routine in a loop that repeats until d1 underflows to zero.
- `close_library_and_cleanup`: Saves registers, calls a library query routine, and if the result is nonzero calls a routine to close the library and another to release a resource, otherwise it returns the result of the query routine.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `xor_global_words`: XORs each of 230 consecutive 32-bit words in a global variable with the constant an address, then returns.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `configure_hardware_and_graphics`: If a value in a table is not negative two, it copies a pointer to a global variable, enables interrupts and DMA, calls a routine to get a graphics structure, and sets the copper list to an offset within that structure before disabling interrupts and DMA.
- `convert_integer_to_string`: Converts a 32-bit value into an ASCII numeric string by extracting nibbles through a loop and writing them to a memory location.
- `search_and_update_table`: Calculates an offset by multiplying a word from a global variable by 237, stores the resulting address in a global variable, and enters a loop that compares bytes from two locations, potentially performing a block move and updating a global variable with a calculated address based on a loop counter.
- `update_structure_array_element`: Calculates an offset by multiplying a value in d0 by 21 and adding it to a pointer from a global variable, then stores this new address in another global variable.
- `copy_ten_bytes_advance_d0`: Copies ten bytes from a source pointer to a destination pointer, then returns the value in d0 incremented by one.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_bytes_with_condition`: Increments a pointer in a1 by 2, sets d0 to 6, and if the byte at the address in a0 is nonzero, copies bytes from a0 to a1 in a loop until d0 reaches zero.
- `fill_memory_with_space`: Fills a memory range of d0 bytes starting at the address in a1 with the byte value 32 using a loop.
- `setup_and_call_struct_init`: Saves registers, loads a table address, calls another routine, then sets up pointer pairs and calls two other routines before restoring the registers and returning.
- `initialize_structure_pointer_sets`: Saves registers, calls another routine, and then sets up three sets of pointer pairs in three different structures by loading values from a table and adding specific offsets to the table pointer, before calling spin_down_counter_d0 and store_pointers_and_set_bytes and returning.
- `select_pointer_compare_two`: Saves registers, calls another routine, adds 2016 to a pointer, compares a word at offset 400 of a structure to two, selects a pointer accordingly, and calls two other routines before restoring the registers and returning.
- `select_pointer_compare_values`: Saves registers, calls another routine, adds 2016 to a pointer, compares a word at offset 400 of a structure to one, three, and five, selecting a pointer for each case, and calls two other routines before restoring the registers and returning.
- `store_pointers_in_structures`: Writes 32-bit pointers from d2, d4, and d6 into the two words at offsets 2 and 6 of structures pointed to by a1, a2, and a3 respectively, advancing each pointer by 8 and incrementing the source registers by 6048 or 992 while looping through counters in d3, d5, and d7.
- `spin_down_counter_d0`: Decrements a 16-bit counter in d0 from 14 until it underflows, doing nothing else, then returns.
- `load_pointers_by_index`: Increments a byte at offset 1 of a global variable and returns if it is not 4, otherwise clears it and stores four 32-bit pointers from a table into a structure, choosing the table start by the value of another byte in the same variable.
- `load_pointers_reverse_index`: Increments a byte at offset 1 of a global variable and returns if it is not 4, otherwise clears it and stores four 32-bit pointers from a table into a structure, choosing the table start in reverse order by the value of another byte in the same variable.
- `store_pointers_and_set_bytes`: Stores four 32-bit pointers from a table into a structure at offsets 2 and 6, advancing the pointer by 8 each time, then writes three bytes to three separate global variables and returns.
- `call_subroutine_at_offset_f2c`: Calls a subroutine located at a fixed offset via a word-sized branch with link.
- `prepare_address_for_jump`: Loads the address of another routine onto the stack and returns.
- `execute_indirect_jump_2`: Branches to an offset, moves d0 to a global variable, pops a value into d0, pops a value into a0, loads a constant into a0, and performs an indirect jump using an offset of 34 from a0.
- `adjust_and_branch`: Increments the stack pointer by 4, adds 200 to d0, sets d1 to 3, and branches to another routine.
- `update_interrupt_and_scale_byte`: Sets a custom interrupt vector, enables interrupts via the hardware register, calls another routine, and then scales a byte from a pointer by dividing it by ten and adding forty-eight before storing the result in a global variable.
- `process_indirect_calls`: Adds d0 to itself, branches to another routine, increments a byte at a global variable, pushes a0 onto the stack, loads a pointer from a global variable into a0, calls that routine, restores a0 from the stack, calls another routine, and branches based on the result.
- `execute_indirect_jump`: Pushes a0 onto the stack and jumps to an address at an offset of 82 from a constant.
- `branch_on_sign_bit`: Branches to one location if the sign bit is not set, otherwise it branches to another location.
- `query_library_and_set_a0`: Calls a routine to query a library base, and if the result in d0 is not negative, sets a0 to zero.
- `decrement_counter_and_transform_byte`: Decrements a 16-bit value in a global variable within a loop, calls another routine, and performs a bitwise shift and addition on a byte read from a structure to store the result in a global variable.
- `branch_and_load_pointer`: Branches if the zero flag is set, otherwise branches if the negative flag is set, loads a pointer from a global variable into a0, calls another routine, and finally loads a value from the stack into a0.
- `call_indirect_via_table`: Pushes the current value of a0 onto the stack, calls a routine at an offset of 118 from a pointer loaded from a table, restores the stack, and jumps to another routine at an offset of 548 from a value loaded into a0.
- `branch_and_dispatch_pointer`: Checks the sign of a word at a specific memory location and, if not negative, pops a longword from the stack into d0, loads a longword from a constant address into a0, and jumps to the address stored in d0.
- `process_jump_table_entry`: Pops a pointer from the stack, performs a series of operations including a loop that decrements a value from a table, and eventually jumps to a routine or performs an indirect jump using a pointer from a table.
- `adjust_stack_and_return`: Increments the stack pointer by 4, pushes the address at offset 1612 of the program counter onto the stack, and returns.
- `jump_to_address_2`: Performs an indirect jump to an address stored at a specific location.
- `process_data_and_dispatch`: Calls another routine, decrements a word from a pointer in d1 using a loop, and then calls another routine if a byte from a global variable is not 'A', 'F', or 'D'.
- `store_swapped_values`: Swaps the bytes in d0, performs arithmetic, and if the result is negative, stores two swapped words into a structure pointed to by a0 and then calls another routine.
- `convert_and_store_digit`: Divides a value in d0 by 10, adds 48 to convert it to an ASCII character, and stores the result in a global variable.
- `move_address_to_a0`: Moves a 32-bit value from an offset of 4 on the stack into a0.
- `call_and_jump`: Increments the stack pointer by 8, calls a routine, and then jumps to an offset of 4122 from the program counter.
- `lookup_and_swap_byte`: Increments the stack pointer by 4, compares d0 with 67, and if equal, branches; otherwise, it loads a pointer from a global variable into a0, loads a byte from that pointer at offset d0 into d0, compares d0 with 63, and branches based on whether d0 is not zero, negative, or positive, eventually writing d0 to offset 6 of the pointer, swapping the words in d0, and branching.
- `update_structure_and_call_handler`: Compares a byte at the address in a0 with 32; if it is not 32 and is not negative, it increments a value in d0 by 80, stores the lower 16 bits of d0 at offset 6 of a structure pointed to by a pointer on the stack, stores the swapped 16-bit halves of d0 at offset 2 of that same structure, and calls a subroutine through a pointer at a global address.
- `update_byte_and_jump`: Swaps the high and low words of d0, adds 48 to d0, stores the resulting byte at an offset of 4604 of a structure in d1, and jumps to another routine.
- `tiny_movea`: Small routine of 1 instruction that copies (a7)+ to a0.
- `call_routine_from_word`: Reads a word from the location pointed to by a0, pushes a0 onto the stack, loads the constant 5680 into a0, calls a routine at an offset of 78 from a0, pushes the address at offset 4168 onto the stack, and returns.
- `pop_pointer_and_push_constant`: Pops a longword from the stack into a0, pushes the constant 4326 onto the stack, and returns.
- `loop_and_store_word`: Adjusts the stack pointer, branches to another routine, or it pushes d0 onto the stack, performs a loop using d0 to write a word to a global variable, or it branches to another routine, or it writes d0 to a global variable.
- `execute_indirect_handler`: Pops a pointer from the stack into d0, loads an address into a1 at an offset of 200 from a0, calls a subroutine through a pointer in d0, calls another subroutine at an offset of 14 from a pointer loaded into a0, and then either increments a 16-bit value in a global variable or pushes a return address onto the stack before returning.
- `loop_and_jump_2`: Decrements d0 in a loop and jumps to a long address.
- `push_d0_and_decrement_loop`: Pushes the contents of d0 onto the stack, loads a word from a global variable into d0, and executes a loop based on the value in d0 before branching to another routine.
- `call_function_via_stack`: Moves a 16-bit value into a global variable, pops a 32-bit value into d0, clears a byte at the location pointed to by a2 and increments a2, pushes a0 onto the stack, calls a routine at an offset of 792 from a constant, and finally pops a0 from the stack.
- `branch_and_execute_indirect_jump`: Checks the status register to branch to different locations, otherwise it pushes a long word onto the stack, returns, clears a word at a location pointed to by a long word, pushes a long word onto the stack, returns, pops a long word into a register, writes the value 2176 to a global variable, and performs an indirect jump.
- `pop_and_call_routine_2`: Pops a pointer from the stack into a0, branches, pops a byte from the stack into d0, stores that byte at a global variable, and calls a routine.
- `divide_and_call`: Increments the stack pointer by four, divides d0 by d1, and calls another routine.
- `push_constant_to_stack_3`: Pushes the constant value 3966 onto the stack and returns immediately.
- `jump_via_stack_pointer`: Loads a pointer from offset 4 of the stack into a0, increments the stack pointer by 8, and performs an indirect jump to the address in a0.
- `increment_and_jump`: Loads a value into a2, branches to another routine, increments a byte at an offset in a global variable, and then performs an indirect jump.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `branch_to_offset_4510`: Performs a short relative branch to another routine.
- `adjust_stack_and_branch_2`: Adds 4 to the stack pointer, then branches to a specific address if the result is negative, otherwise it branches to a different address if the result is positive.
- `branch_to_offset_2`: Branches to an offset of 1544 from the program counter.
- `push_constant_to_stack_2`: Pushes the constant value 2936 onto the stack and returns.
- `execute_indirect_jump_offset`: Pushes a0 onto the stack and jumps to an address at an offset of 60 from a constant.
- `adjust_stack_and_branch`: Increments the stack pointer by 4 and branches to another location.
- `pop_address_to_a0`: Loads a longword from the stack into a0.
- `call_subroutine_at_offset_1064`: Calls a subroutine located at an offset of 2660 bytes from the current program counter.
- `adjust_stack_and_return_offset`: Increments the stack pointer by 4, pushes the address at offset 3450 of the program counter onto the stack, and returns.
- `adjust_stack_and_pop`: Adjusts the stack pointer by adding 4, loads a value from the stack into a0, and branches based on the sign flag.
- `configure_hardware_and_dispatch`: Calls two other routines, writes a pointer to a custom interrupt vector, sets a value in a hardware register, and then increments a word in a global variable before checking if a pointer from a table matches a specific address to decide whether to push it to the stack and call another routine.
- `branch_conditionally_on_zero_flag`: Branches to another routine if the zero flag is set, otherwise branches to another routine.
- `clear_byte_and_call`: Clears a byte at a global variable and calls a routine at a PC-relative offset.
- `loop_and_jump`: Loads a pointer from the stack, performs a loop that decrements d0, increments a byte in a global variable, and then performs an indirect jump.
- `branch_to_offset`: Performs a short branch to a relative offset.
- `clear_memory_and_jump`: Clears a block of memory pointed to by a0 for 2568 bytes using a loop, then calculates a value from d7 and calls a routine at an offset of 78 from a constant.
- `execute_indirect_jump_via_table`: Pushes the current stack pointer onto the stack, loads a constant into a0, and performs an indirect jump to an offset of 1674 from that constant.
- `increment_global_and_branch`: Adjusts the stack pointer, then if the result of a comparison is negative it branches, otherwise if positive it increments a word at a location pointed to by d1, branches, or calls another routine and performs an indirect jump.
- `pop_and_call_routine_3`: Loads a value from an offset of 4 from the stack into a register, increments the stack pointer by 8, loads a constant into that register, clears a register, and calls another routine.
- `call_and_branch`: Calls a routine and then performs a short branch.
- `decrement_and_loop`: Increments the stack pointer by 4, stores d0 on the stack, sets d0 to 70, and loops using d0 as a counter.
- `push_a0_and_call_indirect`: Increments the stack pointer by four, pushes the current value of a0 onto the stack, loads a constant into a0, and calls a routine at an offset of fifty-six from that constant.
- `adjust_stack_and_call`: Loads a 32-bit value from an offset of 4 from the stack pointer into a0, increments the stack pointer by 8, and calls a subroutine at a fixed address.
- `check_input_and_jump`: Performs a loop based on a value from a pointer in d1, then checks bits in the CIAA and POTGOR registers and compares a byte in a structure in d1 against constants before performing an indirect jump to a routine.
- `adjust_stack_and_jump`: Increments the stack pointer by 4 and jumps to an address stored in a long word.
- `swap_and_branch`: Adjusts the stack pointer, swaps the high and low words of d0, and branches based on the sign bit.
- `execute_jump_and_loop`: Performs a jump to an address, then enters a loop using a counter in d0, writes the word negative 65534 to a global variable, pushes a pointer from a0 to the stack, loads a pointer from a global variable into a0, calls the routine at that pointer, and finally restores a0 from the stack.
- `return_constant_value`: Stores the word value 250 into a memory location pointed to by a global variable and returns.
- `update_structure_from_table`: Reads a byte from a table, and if it is not zero, uses it to index into another table to copy eight bytes into a structure at specific offsets, then updates a pointer in a global variable.
- `process_keyboard_input`: Reads a byte from the keyboard serial register, performs bitwise operations to invert and rotate the value, updates two global variables if the value has changed, sets a bit in a hardware register, and then either returns or processes a linked list of data by inserting a byte and calling another routine.
- `accumulate_table_values_into_buffer`: Reads a count and an index from a data block, looks up a value in a table indexed by that value, and adds it into a destination buffer at offsets given by a following series of words.
- `initialize_resource_buffers`: Calculates a pointer to a data structure, calls a routine to validate it, and then performs a series of memory fills and data copies using various offsets from a passed structure.
- `init_exec_library`: Loads a pointer from the exec.OldOpenLibrary entry in the library table, calls it to obtain a library base, stores that base at offset 2646 of a global variable, then calls a routine at offset 301 of the library base and returns.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `close_libraries_and_clear`: Closes two libraries via the exec library, clears a 1024-word block in a loop, fills a table with a constant, expands a byte table, and clears a flag word.
- `set_flag_and_call`: Writes the constant one to a global variable and calls another routine.
- `init_buffer_state_from_source`: Saves three incoming pointers into global variables, zeroes a 56-word block and a 16-word block, then inspects bytes at offsets 5, 6 and 7 of a source buffer to set two selector values and compute a pointer.
- `reset_pointer_state_when_flag_set`: If a flag in a global variable is non-zero, it zeroes four consecutive words in a structure, clears two other flags, and recomputes a pointer from a base value, a multiplier and a stored offset before setting the flag to its maximum value.
- `copy_table_data_into_structures`: Copies a large block of words and longwords from a source table into several destination structures using two looping copies, then spreads the final values into a series of global variables and sets two flags to their maximum value.
- `reload_pinball_handler_table`: When a flag in a global variable is set, it copies a sequence of routine entry points and data from a table into a handler dispatch table, then clears that flag.
- `process_callback_and_update_state`: Checks three conditional flags to decide whether to call three different routines or proceed to a loop.
- `update_callback_list_and_transform_data`: Iterates through a list of pointers, calling each routine and continuing as long as they return zero.
- `execute_callback_loop`: Iterates through a list of pointers, calling a routine for each, and if the routine returns zero, it enters a loop that copies bytes from a structure into another structure before returning.
- `execute_callback_sequence_2`: Stores a word from d0 into a global variable, calls four consecutive routines via pointers from a table, and then enters a loop that copies data from a structure to a destination based on a counter and a comparison with a specific routine address.
- `copy_words_in_loop`: Copies a word from the structure pointed to by a0 to the one pointed to by a1, advances a1 by 4, advances a0 by 56, and repeats until d1 reaches zero.
- `copy_and_loop_data`: Copies a word from offset 28 of the structure in a0 to the address in a1, increments a1 by 4, updates a0 to point to offset 56 of the structure in a0, and loops using d1 until d1 is zero.
- `populate_structure_from_table`: Registers a handler pointer in a list and initializes a structure by reading values from a table.
- `store_table_value_via_mask`: Reads a 16-bit value from a table pointed to by a6 and stores it at offset 20 of the structure in a2.
- `store_table_value_via_rotate`: Reads a 16-bit value from a table pointed to by a6 and stores it at offset 22 of the structure in a2.
- `store_pointer_and_branch`: Stores the constant value 8778 in the address pointed to by a1 and increments a1, then branches to another routine.
- `store_byte_and_branch`: Stores the byte from d1 as a word at offset 6 of the structure in a2, stores the constant value 8780 in the address pointed to by a1 and increments a1, then branches to another routine.
- `store_byte_and_branch_alt`: Stores the byte from d1 as a word at offset 6 of the structure in a2, stores the constant value 8964 in the address pointed to by a1 and increments a1, then branches to another routine.
- `split_and_store_bytes`: Extracts individual bytes from the 32-bit value in d1 and stores them into different offsets of the structure in a2, then calls a routine through a pointer before returning.
- `save_word_and_split_nibbles`: Saves a word at offset 38 of the structure in a2 on the stack, calls a routine, restores that word, and if the byte at offset 36 is non-zero it clears the byte at offset 34, sets the word at offset 38 to 65535, clears the byte at offset 36, stores a routine address at the pointer in a1, and splits the low byte of d1 into two nibbles stored at offsets 35 and 37.
- `store_constant_and_branch`: Stores the constant value 9186 at the address pointed to by a1 and then performs a short branch.
- `store_nibbles_and_call`: Stores the low and high nibbles of d1 into offsets 12 and 14 of the structure in a2, calls a routine while preserving a value on the stack, then clears status bytes when a flag is set.
- `process_nibbles_and_copy`: Calls a routine if the byte at offset 54 of the structure in a2 is not negative, then checks bits in d1 to either clear a byte at offset 50 or copy a byte from offset 162 of the structure in a5 to offset 55 of the structure in a2.
- `clear_and_copy_byte_split_nibbles`: Clears the byte at offset 50 of the structure in a2, calls a routine, copies the byte at offset 258 of the structure in a5 to offset 55, stores a routine address at the pointer in a1, clears the byte at offset 54, and when d1 is non-zero splits its low byte into two nibbles stored at offsets 51 and 52.
- `register_callback_and_branch`: Stores the address of this routine into a memory location pointed to by a1, increments a1, and performs a short branch to another location.
- `combine_shifted_clamp`: Calls another routine, combines a byte from d1 shifted left by eight with a word from the structure in a6, and clamps the result using a value at offset 4 of that structure before storing it at offset 44 of the structure in a2.
- `store_nibbles_and_branch`: Stores the constant value 9186 in the address pointed to by a1, extracts the lower nibble of d1 and stores it as a word at offset 12 of the structure in a2, then extracts the upper nibble of d1 and stores it as a word at offset 14 of the structure in a2 before branching to another routine.
- `calculate_and_store_scaled_value`: Clears a word at a global variable, multiplies the byte in d1 by a word at a global variable, adds 8 and a long value from a global variable to the result, stores the result in a global variable, stores the constant value 8778 in the address pointed to by a1 and increments a1, then branches to another routine.
- `store_byte_and_return`: Calls another routine, stores the byte from d1 at offset 258 of the structure in a5, stores the constant value 8778 in the address pointed to by a1 and increments a1, then returns.
- `branch_on_d1_and_setup`: Checks if d1 is non-zero, otherwise it clears a word at a global address, stores a constant in the pointer at a1, and branches to another routine.
- `generate_bitmapped_data`: Calculates a value from d1 and stores it in a global variable, checks a global pointer, and if it matches a specific value, enters a loop that performs bitwise rotations on bytes from a table to generate a sequence of long values stored in a destination buffer starting at a global variable.
- `dispatch_command_via_table`: Extracts a nibble from a value in register d1, uses it as an index into a table to find a function pointer, and jumps to that address.
- `update_global_and_branch`: Decrements d1, stores the byte result into two consecutive memory locations at a fixed address and a fixed address, writes the constant 8906 to the address in a1, and branches to 8768.
- `clear_word_and_branch`: Clears a 16-bit word at 4934 and branches to 8768.
- `store_pointer_and_branch_2`: Stores the constant 8906 at the address pointed to by a1 and branches to 8768.
- `branch_on_d3_value`: Copies d1 to d3, masks the lower four bits of d3, and branches to another routine if d3 is zero or one; otherwise it loads the constant 67288 into a6 and branches to another routine.
- `load_address_and_branch`: Loads a constant address into a6 and branches to another routine.
- `update_structure_and_branch`: Stores the address in a6 at offset 64 of the structure pointed to by a1, stores the constant 8906 at offset 64 of that same structure, and branches to another routine.
- `update_structure_nibble_and_pointer`: Calls another routine, masks the result to a 4-bit value and stores it at offset 48 of the structure in a2, writes a pointer to itself at the address pointed to by a1, masks the original d1 value to isolate the highest byte, and branches back to the start of the routine.
- `filter_and_branch_on_nibble`: Masks the lower four bits of a longword in a register and branches to different locations based on whether the result is zero, one, or another value; otherwise, it loads a constant into a6 and branches to another routine.
- `load_pointer_and_branch`: Loads a pointer from a global variable into a6 and branches to another routine.
- `initialize_structure_and_branch`: Loads the address 6216 into a6, stores a6 at offset 32 of the address in a1, writes 65535 to offset 38 of the structure in a2, stores the constant 8906 at the address in a1, and branches to 8768.
- `store_d1_and_branch`: Stores a routine address at the pointer in a1, stores d1 at offset 28 of the structure in a2, and branches to another routine.
- `increment_word_capped_at`: Calls the handler routine, adds the low nibble of the input to a word at offset 4 of the structure in a2, caps the result at 907, stores it back, writes a routine address into the pointer at a1, and returns.
- `decrement_word_clamped_at`: Calls the handler routine, subtracts the low nibble of the input from a word at offset 4 of the structure in a2, clamps the result to a minimum of 108, stores it back, writes a routine address into the pointer at a1, and returns.
- `increment_byte_capped_at`: Calls the handler routine, adds the low nibble of the input to a byte at offset 258 of the structure in a5, caps the result at 64, stores it back, writes a routine address into the pointer at a1, and returns.
- `decrement_byte_clamped_at_zero`: Calls the handler routine, subtracts the low nibble of the input from a byte at offset 258 of the structure in a5, clamps the result to a minimum of zero, stores it back, writes a routine address into the pointer at a1, and returns.
- `store_pointer_and_branch_3`: Calls another routine, then stores the low nibble of d1 into a word at offset 12 of the structure in a2, and stores the constant value 9862 at the address pointed to by a1 before returning.
- `store_pointer_and_clear_byte`: Clears a byte at offset 258 of the structure in a5, stores the constant value 8906 in the address pointed to by a1, and returns.
- `store_d1_and_branch_masked`: Stores a routine address at the pointer in a1, stores d1 at offset 28 of the structure in a2, masks d1 to clear the upper 24 bits, and branches to another routine.
- `update_nibble_and_branch`: Stores the constant value 9402 at the address pointed to by a1, then checks a global longword; if the value is positive, it stores the low nibble of d1 into a longword at a global address and branches to another routine.
- `set_pointer_and_branch`: Stores the constant value 9402 at the address pointed to by a1 and branches to another routine.
- `init_from_nibbles`: Clears the byte at offset 54 and the word at offset 56 of the structure in a2, then derives a high nibble and a low nibble from the 32-bit input and branches away when either is zero.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `advance_queue_pointer`: Reads a pointer stored at offset 26 of the structure in a2 and compares it with offset 26 itself, falling back to offset 20 when they match.
- `clamp_difference_minimum`: Subtracts the word at offset 6 of the structure in a2 from the word at offset 4, clamps the result to a minimum of 108, stores the clamped value back at offset 4, and returns.
- `clamp_sum_maximum`: Adds the word at offset 4 to the word at offset 6 of the structure in a2, clamps the sum to a maximum of 907, stores the clamped value at offset 4, and returns.
- `compare_and_store_word_2`: Compares the word at offset 10 of the structure in a2 with the word at offset 4, and if it is less than the value in d2, performs a second comparison after subtracting the word at offset 12 from d2, and if that is also less than the value in d1, stores the modified d2 into the word at offset 4.
- `compare_and_store_word`: Adds the word at offset 18 of the structure in a2 to d2, compares it to d1, and if d2 is less than d1, stores d1 into a word at offset 4 of the structure in a2.
- `calculate_scaled_value`: Loads a byte from a table using an index from offset 52 of the structure in a2, multiplies it by a byte from offset 55 of the structure in a2, shifts the result right by 7, and adds it to a word at offset 4 of the structure in a2 if the byte at offset 52 is less than 31.
- `clamp_and_mask_byte`: Subtracts the word in d2 from the word in d3, clamps the result between 108 and 907, stores it at offset 38 of the structure in a2, then masks the addition of the byte at offset 35 and the byte input with sixty-three and stores the result at offset 34.
- `clamp_byte_offset_2`: Calls another routine and then clamps a value calculated by adding a byte from a structure in a3 to a word at offset 14 of a structure in a2, subtracting a word at offset 12 of the structure in a2, and stores the result back into the byte at offset 258 of the structure in a3.
- `update_clamped_byte_offset`: Calls another routine and then calculates a value by adding a byte from offset 258 of a structure in a3 to a word at offset 14 of a structure in a2, subtracting a word at offset 12 of the structure in a2, and clamps the result between 0 and 64 before storing it back to offset 258 of the structure in a3.
- `calculate_scaled_byte`: Loads a byte from a table using an index from offset 50 of the structure in a2, multiplies it by a byte at offset 52 of the structure in a2, shifts the result right by 6, and adds it to a byte at offset 55 of the structure in a2 if the byte at offset 50 is less than 31.
- `clamp_byte_and_mask`: Subtracts the byte in d2 from the byte in d3, clamps the result between zero and sixty-four, stores it at offset 258 of the structure in a3, then adds the byte at offset 51 to the byte input, masks the sum with sixty-three, and stores it at offset 50.
- `clamp_byte_offset`: Adds the byte at offset 258 of the structure in a3 to the word at offset 14 of the structure in a2, subtracts the word at offset 12, clamps the result between zero and sixty-four, stores it at offset 258 of the structure in a3, and returns.
- `compare_nibble_and_branch`: Compares the low nibble of the byte at offset 43 of the structure in a2 with the difference of two global words and, when they match and the byte at offset 40 is non-zero, branches to a shared routine before returning.
- `check_nibble_and_divide`: Checks that the low nibble of the byte at offset 43 of the structure in a2 is non-zero, then when the difference of two global words is zero and the byte at offset 40 is non-zero it divides and branches to a shared routine when the remainder is zero.
- `decrement_word_clear_byte`: Decrements the word at offset 12 of the structure in a2 and, if it reaches zero, clears the byte at offset 258 of the structure in a3, otherwise returns.
- `calculate_and_store_division_results`: Reads a pair of 16-bit values from a table, performs a series of divisions using a constant and the swapped high word of the first value, and stores the resulting quotients into a destination table via a pointer.
- `loop_then_rotate_store`: Decrements the counter in d4 and loops back until it reaches zero, otherwise rotates a value loaded through the pointer in a1 right by two bits and stores a fill pattern through that pointer before returning.
- `reset_globals_conditional`: When two global words are nonzero it clears four consecutive words in a global table.
- `decompress_bitstream_chunk`: Validates a magic constant in a data stream, calculates an offset, and extracts a byte and a word from a source buffer into a destination buffer based on a bit test.
- `extract_bitstream_data`: Constructs a 32-bit value by reading four bytes from a source pointed to by a0 and performing shifts and swaps, then enters a loop that pops words from a stack pointed to by a5 into a destination pointed to by a6 based on a counter and arithmetic logic.
- `process_bitstream_data`: Performs bitwise expansion and data extraction from a bitstream by shifting and masking values based on a control table.
- `reset_audio_and_pointers`: Clears five-word blocks at offsets 0CAE and 0C6E, stores a pointer at 0BEE and a pointer plus 2 at 0C2E, writes the constant 284 to five words at 0BAE, zeroes the four audio volume registers, sets bit 1 of the CIA port A, and sets DMACON to 15.

### Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:ATX-TRAINER.EXE

- `unpack_and_reconstruct_data_table`: Allocates memory and scans a linked list of structures for a specific magic constant, then performs a complex reconstruction of a data table by copying blocks of memory and applying bitwise masks based on values read from the source structure.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask in d6 is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `calculate_bit_manipulated_difference`: Adds a byte to a register, performs bitwise shifts and swaps, fetches two bytes from a pointer in a3 with a word rotation, and subtracts the modified value from a constant before returning.
- `fill_figure_table`: Reads a count from a routine call, fills a table with values from a stack buffer, and then iterates through a loop to write bit masks and derived values into a table based on a key list.
- `process_memory_list`: If a value at a pointer in a4 is 1004, it iterates through a list, calculating a new address based on a pointer-traversal loop and adding it to a structure at offset 0 in a5.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:HELLFIRE

- `copy_bytes_between_tables`: Copies 64 bytes from a source table at a specific address to a destination table at another address one byte at a time in a loop, then clears d0 and returns.
- `compare_long_values`: Compares a long value at offset 4 of a structure in a0 against four different constant values and returns if any match.
- `write_control_bytes`: Writes the byte 96 to offset 342 of a structure in a1 and the byte 1 to offset 1711 of the same structure.

### Obsession Pinball (1995)(Unique Development Studios)[cr](Disk 1 of 2)[t +2 ATX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.
- `branch_to_offset_4`: Performs a short branch to a specific offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_byte_and_return`: Sets bit 2 of d4, stores a byte from d0 into a memory location at an offset of 6702 from a0, pops a longword from the stack into d0, and returns.
- `execute_trap_with_callback`: Saves registers to the stack, calls another routine, calculates an address in a5, stores d0 at an offset of 24 from a5, uses a pointer from a1 to store values into a5, calls a trap, restores values from a5 into a1 and a pointer, restores registers, and returns.
- `call_supervisor_function`: Checks a bitmask in a memory location at an offset from a global variable and, if a condition is met, calls a function via a pointer.
- `update_memory_locations`: Stores a constant into a memory location, loads a longword from an offset of 24 from a5 into another memory location, calls another routine, and then stores two values into consecutive memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_routine`: A routine consisting of 148 instructions that calls two unnamed routines.
