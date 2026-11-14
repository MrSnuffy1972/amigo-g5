# Game 1607 (rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01607). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909056 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 96 of 96 named (0 author symbols), 96 of 96 described (10 by rule, 32 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 95 of 95 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 92672 bytes, 3 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1332 of 1332 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16085 |
| code | 3984 |
| data | 711322 |
| padding | 1 |
| unused | 177664 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf:bootblock | 1 | 8 | 1 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf boot-block load | 1 | 99 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 227 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 622 | 53 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 10 |
| Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf bytes 45056-97792 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf bytes 100864-130048 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf bytes 265216-275968 (code never loaded by a reference run) | 1 | 365 | 3 |

## Cases handled

- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: custom-format (non-DOS) disk, 1411 non-empty blocks: matched against track-loaded segments
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: the boot block reads 16384 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 62 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 32 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: bytes 45056-97792 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: bytes 100864-130048 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: bytes 265216-275968 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: 4 logged loads cover 109056 bytes of the image
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 34-86 (bytes 17408-44544) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.36)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 260-516 (bytes 133120-264704) outside the filesystem hold data, not code (opcode density 0.012, entropy 4.45)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 540-540 (bytes 276480-276992) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.59)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 542-542 (bytes 277504-278016) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.48)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 544-568 (bytes 278528-291328) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.26)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 570-744 (bytes 291840-381440) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.76)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 746-840 (bytes 381952-430592) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.17)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 842-942 (bytes 431104-482816) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.53)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 944-1054 (bytes 483328-540160) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.90)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 1056-1166 (bytes 540672-597504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.02)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 1168-1290 (bytes 598016-660992) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.38)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 1292-1353 (bytes 661504-693248) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.86)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 1364-1364 (bytes 698368-698880) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 1366-1417 (bytes 699392-726016) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.33)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 1605-1619 (bytes 821760-829440) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.96)
- rainbow islands (1987)(ocean)[cr qtx][t +2 slipstream]/Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf: blocks 1723-1736 (bytes 882176-889344) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.87)
- 2 never-loaded region(s), 81920 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf:bootblock

- `issue_single_io_request`: Boot block entry that issues one exec DoIO request and returns, with no other registers or text recorded.

### logged disk load from Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf boot-block load

- `store_byte_from_longword`: Reads a 32-bit value from just before the address register, stores a word built from the high byte of a data register and the low byte of that value, and returns the value shifted right by one with its top bit set.
- `shift_bits_reloading_on_empty`: Shifts a data register right by one bit per step for a count, calling an unnamed routine to reload it from memory whenever it empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_forward`: Performs a relative branch to an instruction four bytes ahead.
- `call_table_entry`: Increments a 32-bit value at offset 2348 of a structure in a5, then calls a function from a table using an offset stored at offset 2344 of that table.
- `trap_call_offset_eight`: Triggers a trap, then calls a function from a table using an offset stored at offset 8 of the table pointed to by a pointer at offset 2344 of a structure in a5, before triggering another trap.
- `trap_call_offset_hundred`: Triggers a trap, then calls a function from a table using an offset stored at offset 100 of the table pointed to by a pointer at offset 2344 of a structure in a5, before triggering another trap.
- `compare_and_trap`: Triggers a trap, compares a 16-bit value at offset 2414 of a structure in a5 to itself, and if they are not equal, triggers another trap and returns.
- `fill_math_table`: Loads a 16-bit word from offset 4 of a structure in a5 into d0 and calls another routine.
- `fill_table_with_math`: Sets a pointer in a structure in a5 to point to a table at offset 2454, then fills 128 words in that table by repeatedly multiplying a value by 171, dividing by 29843, and adding 1.
- `update_table_with_math`: Modifies three bytes at offsets 2417, 2419, and 2421 of a structure in a5, calculates a new value by multiplying a value at offset 2454 by 171 and dividing by 29843, then adds this to two values retrieved from a table and stores the result in that table.
- `increment_and_dereference`: Increments a byte at offset 2683 of a structure in a5 and returns the 16-bit word located at the address stored in a pointer at offset 2648 of that structure.
- `toggle_input_bit`: Disables interrupts, reads a byte from the custom chip input register, performs an XOR operation between that byte and a byte in a structure in a5, stores the result back in the structure, and re-enables interrupts.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `process_interrupt_and_dispatch`: Reads a byte from the interrupt control register, updates a global variable, sets a bit in the interrupt request register, and conditionally calls several routines based on bit manipulation of the input byte.
- `call_offset_a6c`: Loads a pointer from offset 2668 of a structure in a5 into a0, calls the address in a0, and then branches to another routine.
- `call_offset_a70`: Loads a pointer from offset 2672 of a structure in a5 into a0, calls the address in a0, and then branches to another routine.
- `call_offset_a74`: Loads a pointer from offset 2676 of a structure in a5 into a0, calls the address in a0, and then branches to another routine.
- `call_offset_a78`: Loads a pointer from offset 2680 of a structure in a5 into a0, calls the address in a0, and then branches to another routine.
- `call_offset_a7c`: Loads a pointer from offset 2684 of a structure in a5 into a0, calls the address in a0, and then branches to another routine.
- `process_interrupt_requests`: Reads the interrupt mask and request registers, writes the intersection of these values masked with bit 11 to the interrupt request register, and then calls a routine from a pointer at offset 488 of a structure in a5 if bit 14 is set, and calls a routine from a pointer at offset 492 of the structure in a5 if bit 13 is set.
- `call_offset_a80`: Loads a pointer from offset 2688 of a structure in a5 into a0, calls the address in a0, and then branches to another routine.
- `call_offset_a84`: Loads a pointer from offset 2692 of a structure in a5 into a0, calls the address in a0, and then branches to another routine.
- `handle_masked_interrupts`: Writes the intersection of the interrupt mask, request registers, and a mask of bits 8 through 10 to the interrupt request register, then performs a series of conditional calls to routines at offsets 488, 492, 496, and a table-based address depending on the value of the processed interrupt bits.
- `call_offset_a94`: Loads a long value from offset 2452 of a structure in a5 into a0, calls the routine at that address, and then branches to another routine.
- `call_offset_a90`: Loads a long value from offset 2448 of a structure in a5 into a0, calls the routine at that address, and then branches to another routine.
- `call_offset_a8c`: Loads a long value from offset 2444 of a structure in a5 into a0, calls the routine at that address, and then branches to another routine.
- `call_offset_a88`: Loads a long value from offset 2440 of a structure in a5 into a0, calls the routine at that address, and then branches to another routine.
- `dispatch_interrupt_requests`: Disables interrupts, calculates a bitmask from the interrupt enable and request registers, writes this mask to the interrupt request register, and calls one of three different routines based on the resulting bitmask before returning via an interrupt return instruction.
- `handle_interrupt_and_dispatch`: Sets the interrupt mask to 8, updates a byte in a table using a value from the CIAA interrupt register, and then performs a series of conditional calls to routines stored at offsets 2728, 2732, 2736, 2740, and 2744 of a structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_call`: Performs an indirect function call using a value from a register and an offset.
- `store_word_triple`: Copies three consecutive 16-bit values from a source pointer into three consecutive 16-bit fields of a structure at fixed offsets, and returns minus one.
- `reset_three_word_fields`: Zeroes three consecutive 16-bit fields of a record at offset 30 and then calls another routine.
- `read_long_field_at_offset`: Swaps the high and low words of a 16-bit value from offset 30 of a structure in a6 and loads 16-bit values from offsets 32 and 34 of the same structure into d0 and d1.
- `test_and_set_bits`: Loads a byte from a2, and if it is positive, performs bitwise operations to set or clear bits in a table at offset 48 or 48 plus an offset derived from the byte at offset 48 of a structure in a6.
- `clear_four_long_fields`: Zeroes four consecutive 32-bit fields at offset 48 of the record in a6 and then calls another routine.
- `test_bit_from_offset_table`: Copies a word from one register to another, shifts the second register right by three bits, and tests a bit in a table located at an offset determined by the second register relative to a pointer in a6, then returns.
- `initialize_stack_and_d3`: Saves d2 and d3 to the stack, clears d0, clears d3, and then branches to another routine.
- `set_bits_from_table`: Reads a sequence of 16-bit words from a pointer in a0, using each word to check a bit in a table at an offset from a6 and setting a bit in d0 and incrementing d3 accordingly.
- `check_table_bit`: Loads a byte from a pointer in a2 into d0, and if d0 is not negative, checks a bit in a table at an offset from a6 using an index derived from d0.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `store_byte_and_load_table`: Stores a byte from d1 into a structure at offset 30 of a6, and if d1 is not zero, loads a pointer from a table into four offsets of the structure in a6 and pushes d0 to the stack before branching to another routine.
- `modify_bit_in_structure`: Performs a bitwise XOR on a bit within a structure at offset 32 plus a scaled value from d1, then enters a loop that reads 16-bit words from a table and calls a routine via a pointer until a negative word is encountered.
- `check_byte_and_set_values`: Tests a byte in d1; if it is non-zero, it branches to another routine, otherwise it stores two 32-bit constants at offsets 20 and 24 of the structure in a6 and branches to another routine.
- `update_values_and_call`: Stores the constant 63290 at offset 20 of the structure in a6, stores 16196 at offset 24 of the structure in a6, and then calls another routine.
- `process_conditional_chain`: Checks a 16-bit value at offset 46 of a structure in a6, and if it is one, executes a sequence of three routine calls via pointers in a4, potentially decrementing a 16-bit value at offset 32 of a structure in a6 and triggering a trap if a borrow occurs.
- `decrement_and_call_routine`: Decrements a 16-bit value at offset 4 of a structure pointed to by offset 20 of a6, triggers a trap if a borrow occurs, calls a routine at a fixed address, and decrements a 16-bit value at offset 30 of a6.
- `calculate_division_result`: Loads two pointers from a structure in a6, performs a calculation involving a multiplication and division using values from a table, and then branches based on a comparison of d1 with 1.
- `process_data_loop`: Decrements d0 in a loop while fetching words from a structure in a0 using an offset derived from d1, then calls another routine, and finally executes a loop that calls a function pointer retrieved from a table using an offset stored in a structure.
- `store_word_and_branch`: Stores a 16-bit value from d4 into a structure at offset d2 of a0 and then branches to another routine.
- `test_d2_and_branch`: Tests the 16-bit value in d2 and branches to another routine if it is non-zero, otherwise it clears d5 and branches to another routine.
- `setup_loop_parameters`: Writes a 16-bit value to offset 0 of a structure in a0, calls another routine, and then loads several values from a2 and a6 into registers before branching.
- `loop_and_call_function`: Executes a loop that calls a function pointer retrieved from a table using an offset stored in a structure, incrementing d4 if the return value is not negative, and repeats this d2 times.
- `setup_pointer_and_d4`: Subtracts 2 from a 16-bit value from offset 6 of a structure in a2, loads the address of offset 2 of a structure in a0 into a1, sets d4 to negative 1, and then branches to another routine.
- `loop_and_calculate_offset`: Enters a loop that tests 16-bit words from a table at a1, and if a word is non-negative, it stores d4 at offset -2 of the table, calculates a value based on the difference between a1 and a0, and repeats the loop based on d0.
- `call_routine_if_nonnegative`: Calls another routine only when a 16-bit value passed in a data register is nonnegative, and otherwise returns without doing anything.
- `clear_linked_cell_chain`: Follows a chain of 16-bit links starting from a table index, zeroing each visited cell, and stops when it reaches a negative link.
- `call_table_function`: Calls a function pointer retrieved from a table using an offset stored at offset 46 of a structure pointed to by offset 26 of a6.
- `decrement_two_counters`: Decrements two 16-bit values located at offsets 4 and 26 of structures pointed to by offsets 14 and 26 of a structure in a6, calling a trap if a borrow occurs during either subtraction.
- `update_counters_and_call`: Stores a byte from d0 at offset 36 of a structure in a6, stores a pointer from a0 at offset 14 of a6, increments a 16-bit value at offset 4 of that pointer, updates a pointer at offset 26 of a6 with a value from a table, increments a 16-bit value at offset 4 of that new pointer, clears a 16-bit value at offset 30 of a6, and calls a routine at address in a0.
- `compare_byte_to_stored_flag`: Compares the byte in a data register with the flag byte stored at offset 36 in the record.
- `process_callback_and_update_counter`: Checks a 16-bit value at offset 46 of a structure in a6; if zero, it executes a callback from a table, and if the result is non-negative, executes a second callback and enters a loop that performs multiplication and calls another routine.
- `process_callbacks_and_counters`: Executes a loop that calls routines via a table, increments a counter in d4, and if a specific 16-bit value is found, performs several more routine calls and updates a 16-bit counter at offset 4 of a pointer, potentially triggering a trap if the counter becomes negative.
- `call_and_increment_counters`: Calls a routine via a table pointer found at offset 32 of a6, increments a 16-bit value at offset 4 of a pointer retrieved from a6, increments a 16-bit value at offset 46 of a6, and returns zero.
- `call_routine_with_d6_value`: Pushes d6 to the stack, sets d6 to 88, calls another routine, restores d6 from the stack, and returns.
- `call_via_table_pointer`: Calls a routine using a table pointer and offset found in the structure at offset 14 and offset 34 of a6.
- `call_routine_with_different_d6_value`: Pushes d6 to the stack, sets d6 to 138, calls another routine, restores d6 from the stack, and returns.
- `calculate_iterative_offsets_2`: Performs a series of divisions and subtractions using values from a table and a structure in a6 to calculate a result in d0, then branches to another routine.
- `calculate_iterative_offsets`: Runs a loop that updates d4 using values from a table at a3, and then performs arithmetic using values from a2 to calculate a final value in d2.
- `execute_iterative_callback_sequence`: Iterates through a sequence of data, calculating an offset based on values in a structure, and calls a function pointer retrieved from a table using the calculated offset and a value from a register.
- `decrement_and_call_if_one`: Checks if a 16-bit value at offset 44 of a structure in a6 is equal to 1.
- `execute_sequence_of_callbacks_2`: Performs a sequence of calls to routines via table pointers found at various offsets in structures pointed to by a2 and a3, and decrements 1.
- `execute_sequence_of_callbacks`: Performs a sequence of calls to routines via table pointers found at various offsets in structures pointed to by a2 and a3, and decrements 16-bit values at offset 1 of those structures, triggering a trap if an arithmetic overflow occurs.
- `clear_long_field_and_call`: Clears a 32-bit long word at offset 36 of the structure in a6 and calls another routine.
- `release_secondary_object_then_call`: Decrements the 16-bit counter of the object referenced at offset 50, trapping on signed overflow, and then calls another routine.
- `decrement_held_object_then_call`: Decrements the 16-bit counter of the object referenced at offset 46, trapping on signed overflow, and then calls another routine.
- `decrement_structure_field`: Decrements a short integer at an offset within a structure and triggers a trap if an arithmetic overflow occurs.
- `update_structure_and_call_2`: Calls another routine, then clears a 16-bit word at offset 28 of the structure in a6.
- `increment_flag_and_call`: The routine checks a short flag at an offset from a pointer in register A6.
- `execute_callback_chain`: Walks a singly linked list starting from a pointer held in a structure, calling the routine at offset 16 in each node until a node's next pointer is zero.
- `call_structure_callback`: Calls the routine whose address is stored 20 bytes into the structure pointed to by the base register, then returns.
- `update_structure_and_call`: Stores a byte from d0 at offset 14 of a structure in a6, stores a pointer from a0 at offset 16 of a6, calls a routine using a pointer and offset from a4, and if the result in a0 is not zero, updates the pointer at offset 16 of a6 with the new value.
- `call_stored_entry_at`: Calls the routine whose entry address is stored at offset 24 in the structure in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_once_then_increment_counter`: Calls a routine through a table entry when a 16-bit counter in the structure is zero, then increments the counter.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_pointer_through_offset`: Calls a function located at a pointer stored at a specific offset from a register.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Rainbow Islands (1987)(Ocean)[cr QTX][t +2 Slipstream].adf bytes 265216-275968 (code never loaded by a reference run)

- `decrement_count_and_dispatch`: Calls a routine through a pointer stored in one structure, decrements a 16-bit count in a second structure with an overflow trap check, then calls another pointer-based routine and returns zero.
- `call_routine_per_character`: Walks a string until it finds a space or the count runs out, calling a routine looked up from a table at offset 8 of the base register for each character.
- `copy_name_component_padded`: Copies characters from a source string into a destination buffer up to a count, stopping at a zero, backslash or dot.
