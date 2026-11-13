# Game 1608 (rainbow islands (1987)(ocean)[cr scx][t +3 scx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01608). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 85 of 85 named (0 author symbols), 85 of 85 described (10 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 48 of 48 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 94208 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 918 of 918 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 33307 |
| code | 2762 |
| data | 682071 |
| padding | 4 |
| unused | 190976 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf:bootblock | 1 | 15 | 1 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load | 1 | 34 | 1 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load | 1 | 90 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 101 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 667 | 60 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 10 |
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf bytes 34816-88576 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf bytes 91648-120832 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf bytes 256000-267264 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: custom-format (non-DOS) disk, 1359 non-empty blocks: matched against track-loaded segments
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: the boot block reads 22528 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: bytes 34816-88576 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: bytes 91648-120832 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: bytes 256000-267264 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: 5 logged loads cover 128000 bytes of the image
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 22-29 (bytes 11264-15360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.12)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 32-39 (bytes 16384-20480) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.95)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 65-66 (bytes 33280-34304) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.29)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 242-498 (bytes 123904-255488) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.45)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 523-523 (bytes 267776-268288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 525-525 (bytes 268800-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.48)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 527-551 (bytes 269824-282624) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.26)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 553-727 (bytes 283136-372736) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.76)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 729-823 (bytes 373248-421888) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.17)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 825-925 (bytes 422400-474112) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.53)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 927-1037 (bytes 474624-531456) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.90)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1039-1149 (bytes 531968-588800) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.03)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1151-1273 (bytes 589312-652288) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.38)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1275-1336 (bytes 652800-684544) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.86)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1347-1347 (bytes 689664-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1349-1400 (bytes 690688-717312) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.33)
- 3 never-loaded region(s), 94208 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf:bootblock

- `load_boot_data_then_jump`: Disables all interrupts and DMA channels, issues two exec DoIO requests, clears a 16-long global area, traps once, copies 256 longwords into low memory and calls a routine through a pointer.

### logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load

- `scale_palette_per_frame`: Waits for and acknowledges the vertical blank request bit, then scales a table of 16 color values by a level that rises each frame and writes them into the palette buffer.

### logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a decrementing 16-bit value into a global cell in a loop that ends when the counter wraps past zero, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_call`: Calculates an address by adding two values and jumps to that location.
- `increment_counter_and_call_handler`: Increments a global counter, then calls a routine whose address is read from a pointer cell plus a 16-bit offset stored 38 bytes into the structure it points to.
- `call_offset_eight`: Loads a pointer from a table at offset 2338 of a structure in a5, then calls a routine at an offset of 8 from the second pointer in that table.
- `call_offset_hundred`: Loads a pointer from a table at offset 2338 of a structure in a5, then calls a routine at an offset of 100 from the second pointer in that table.
- `compare_and_trap`: Compares a 16-bit value at offset 2344 of a structure in a5 with itself, and if they are not equal, executes a trap and returns.
- `call_pseudo_random_filler`: Loads a 16-bit word from offset 4 of a structure in a5 into d0, calls another routine, and then returns.
- `fill_table_with_pseudo_random`: Fills a table starting at offset 2352 of a structure in a5 with 128 16-bit values calculated using a multiplication and division algorithm, then stores the last value at offset 2608 of the structure in a5.
- `update_structure_with_math`: Increments three bytes at specific offsets in a structure in a5, calculates a new value using a multiplication and division algorithm based on a value at offset 2608, and adds this to two values retrieved from a table to store the result in another table location.
- `increment_and_return_word`: Increments a byte at offset 531 of a structure in a5 and returns a 16-bit word from the address stored at offset 530 of the same structure.
- `dispatch_pending_interrupt_levels`: Reads the interrupt enable and request registers, masks the result to interrupt bits four to six, writes the masked bits to the interrupt request register, and calls up to three level handlers through function cells according to those bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_indirect_jump`: Executes a subroutine call using a destination address derived from the sum of two registers.
- `store_word_triple`: Copies three consecutive 16-bit values from a source pointer into three consecutive 16-bit fields of a structure at fixed offsets, and returns minus one.
- `clear_three_word_fields`: Zeroes three consecutive 16-bit fields of a record at offset 30 and then calls another routine.
- `read_long_field_at_offset`: Returns the 32-bit value stored at offset 30 in the record passed through an address register.
- `process_bitfield_and_update_table`: Checks a bit in a bitfield based on a byte from a pointer in a2, and if the bit is set, updates a 16-bit counter in a structure in a6 and stores a 16-bit value composed of a byte from a5 and the input byte into a table at an offset determined by the counter.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_word_field`: Copies a 16-bit word from offset 48 of a structure in a6 to offset 50 of the same structure.
- `clear_two_word_fields_and_call`: Zeroes two 16-bit fields at offsets 48 and 50 of the structure in a6 and then calls another routine.
- `check_word_equality`: Compares a 16-bit word at offset 48 of a structure in a6 with a 16-bit word at offset 50 of the same structure, returning zero if they are equal and negative one otherwise.
- `execute_and_increment_counter`: Repeatedly calls a function pointer retrieved from a table at offset 54 of a structure in a4 until the return value is zero, then increments a 16-bit value at offset 32 of a structure in a6 by two, masks it with 31, and calls a routine at a fixed address.
- `test_and_set_bits`: Loads a byte from a2, and if it is positive, performs bitwise operations to set or clear bits in a table at offset 48 or 48 plus an offset derived from the byte at offset 48 of a structure in a6.
- `clear_bit_from_byte_table`: The routine clears a single bit in a byte taken from a table.
- `clear_four_long_fields`: Zeroes four consecutive 32-bit fields at offset 48 of the record in a6 and then calls another routine.
- `test_bit_from_scaled_index`: Copies a word from one register to another, shifts the second register right by three bits, and tests a bit in a table at an offset determined by the scaled value relative to a pointer in a6, then returns.
- `save_registers_and_branch`: Saves d2 and d3 to the stack, clears d3, sets d0 to zero, and branches to another routine.
- `set_bits_from_table`: Reads a sequence of 16-bit words from a pointer in a0, using each word to check a bit in a table at an offset from a6 and setting a bit in d0 and incrementing d3 accordingly.
- `check_table_bit`: Loads a byte from a pointer in a2 into d0, and if d0 is not negative, checks a bit in a table at an offset from a6 using an index derived from d0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `store_byte_and_load_table`: Stores a byte from d1 into a structure at offset 30 of a6, and if d1 is not zero, loads a pointer from a table into four offsets of the structure in a6 and pushes d0 to the stack before branching to another routine.
- `modify_bit_in_structure`: Performs a bitwise XOR on a bit within a structure at offset 32 plus a scaled value from d1, then enters a loop that reads 16-bit words from a table and calls a routine via a pointer until a negative word is encountered.
- `set_conditional_pointers`: Tests a byte in d1; if it is non-zero, it branches to another routine, otherwise it stores two 32-bit constants at offsets 20 and 24 of a structure in a6 and branches to another routine.
- `set_structure_pointers`: Stores two 32-bit constants at offsets 20 and 24 of a structure in a6 and then calls another routine.
- `process_conditional_chain`: Checks a 16-bit value at offset 46 of a structure in a6, and if it is one, executes a sequence of three routine calls via pointers in a4, potentially decrementing a 16-bit value at offset 32 of a structure in a6 and triggering a trap if a borrow occurs.
- `decrement_and_call_routine`: Decrements a 16-bit value at offset 4 of a structure pointed to by offset 20 of a6, triggers a trap if a borrow occurs, calls a routine at a fixed address, and decrements a 16-bit value at offset 30 of a6.
- `calculate_division_result`: Loads two pointers from a structure in a6, performs a calculation involving a multiplication and division using values from a table, and then branches based on a comparison of d1 with 1.
- `process_data_loop`: Decrements d0 in a loop while fetching words from a structure in a0 using an offset derived from d1, then calls another routine, and finally executes a loop that calls a function pointer retrieved from a table using an offset stored in a structure.
- `store_word_and_branch`: Stores a 16-bit word from d4 into a structure at offset d2 of a0 and branches to another routine.
- `branch_on_d2_value`: Tests d2 and branches to another routine if it is non-zero; otherwise it sets d5 to zero and branches to another routine.
- `setup_loop_parameters`: Writes a 16-bit value to offset 0 of a structure in a0, calls another routine, and then loads several values from a2 and a6 into registers before branching.
- `loop_and_call_function`: Executes a loop that calls a function pointer retrieved from a table using an offset stored in a structure, incrementing d4 if the return value is not negative, and repeats this d2 times.
- `prepare_jump_target`: Loads a word from offset 6 of a structure into d3, subtracts 2 from d3, loads the address at offset 2 of a0 into a1, sets d4 to 255, and branches to another routine.
- `loop_and_calculate_offset`: Enters a loop that tests 16-bit words from a table at a1, and if a word is non-negative, it stores d4 at offset -2 of the table, calculates a value based on the difference between a1 and a0, and repeats the loop based on d0.
- `call_routine_if_nonnegative`: Calls another routine only when a 16-bit value passed in a data register is nonnegative, and otherwise returns without doing anything.
- `clear_linked_cell_chain`: Follows a chain of 16-bit links starting from a table index, zeroing each visited cell, and stops when it reaches a negative link.
- `call_table_function`: Calls a function pointer retrieved from a table using an offset stored at offset 46 of a structure pointed to by offset 26 of a6.
- `decrement_two_counters`: Decrements two 16-bit values located at offsets 4 and 26 of structures pointed to by offsets 14 and 26 of a structure in a6, calling a trap if a borrow occurs during either subtraction.
- `update_and_call_pointers`: Stores a byte from d0 into a structure at offset 36, stores a pointer from a0 into offset 14, increments a 16-bit value at offset 4 of the pointer in a0, updates a pointer in a6 using a value from a5, increments a 16-bit value at offset 4 of that new pointer, clears a 16-bit value at offset 30, and calls a routine at the address in a0.
- `compare_byte_to_stored_flag`: Compares the byte in a data register with the flag byte stored at offset 36 in the record.
- `process_callback_and_update_counter`: Checks a 16-bit value at offset 46 of a structure in a6; if zero, it executes a callback from a table, and if the result is non-negative, executes a second callback and enters a loop that performs multiplication and calls another routine.
- `execute_callback_loop`: Runs a loop that calls a routine via a table pointer at offset 14 of a structure in a6, and if the loop completes, performs several more routine calls and potentially decrements a 16-bit value at offset 4 of a pointer retrieved from a6.
- `call_and_increment_counters`: Calls a routine via a table pointer found at offset 32 of a6, increments a 16-bit value at offset 4 of a pointer retrieved from a6, increments a 16-bit value at offset 46 of a6, and returns zero.
- `call_routine_with_d6_value`: Pushes d6 to the stack, sets d6 to 88, calls another routine, restores d6 from the stack, and returns.
- `call_via_table_pointer`: Calls a routine using a table pointer and offset found in the structure at offset 14 and offset 34 of a6.
- `call_routine_with_different_d6_value`: Pushes d6 to the stack, sets d6 to 138, calls another routine, restores d6 from the stack, and returns.
- `calculate_iterative_offsets_2`: Performs a series of divisions and subtractions using values from a table and a structure in a6 to calculate a result in d0, then branches to another routine.
- `calculate_iterative_offsets`: Runs a loop that updates d4 using values from a table at a3, and then performs arithmetic using values from a2 to calculate a final value in d2.
- `execute_iterative_callback_sequence`: Iterates through a sequence of data, calculating an offset based on values in a structure, and calls a function pointer retrieved from a table using the calculated offset and a value from a register.
- `decrement_and_call_if_one`: Checks if a 16-bit value at offset 44 of a structure in a6 is equal to 1.
- `execute_callback_sequence`: Calls a routine via a pointer at offset 44 of a structure in a4, then conditionally executes a sequence of further calls using pointers from a0, a2, and a3, potentially updating counters at offsets 1 and 2 of structures in a5 and a0.
- `execute_sequence_of_callbacks`: Performs a sequence of calls to routines via table pointers found at various offsets in structures pointed to by a2 and a3, and decrements 16-bit values at offset 1 of those structures, triggering a trap if an arithmetic overflow occurs.
- `clear_long_word_and_call`: Clears a 32-bit word at offset 36 of the structure in a6 and calls another routine.
- `release_secondary_object_then_call`: Decrements the 16-bit counter of the object referenced at offset 50, trapping on signed overflow, and then calls another routine.
- `decrement_held_object_then_call`: Decrements the 16-bit counter of the object referenced at offset 46, trapping on signed overflow, and then calls another routine.
- `decrement_structure_field`: Decrements a short integer at an offset within a structure and triggers a trap if an arithmetic overflow occurs.
- `update_structure_and_call_2`: Calls another routine and then clears a 16-bit word at offset 28 of a structure in a6.
- `increment_flag_and_call`: The routine checks a short flag at an offset from a pointer in register A6.
- `execute_callback_chain`: Walks a singly linked list starting from a pointer held in a structure, calling the routine at offset 16 in each node until a node's next pointer is zero.
- `call_stored_entry_point`: Copies a6 to a0, loads the address at offset 20 of a6 into a1, and calls the routine at that address.
- `update_structure_and_call`: Stores a byte from d0 at offset 14 of a structure in a6, stores a pointer from a0 at offset 16 of a6, calls a routine using a pointer and offset from a4, and if the result in a0 is not zero, updates the pointer at offset 16 of a6 with the new value.
- `call_stored_entry_at`: Copies a6 to a0, loads the address at offset 24 of a6 into a1, and calls the routine at that address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_once_then_increment_counter`: Calls a routine through a table entry when a 16-bit counter in the structure is zero, then increments the counter.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_structure_callback`: Calls the routine whose address is stored 20 bytes into the structure pointed to by the base register, then returns.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
