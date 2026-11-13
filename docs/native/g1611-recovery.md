# Game 1611 (rainbow islands (1987)(ocean)[cr vf]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01611). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 83 of 83 named (0 author symbols), 83 of 83 described (10 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 48 of 48 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 95744 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 961 of 961 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5513 |
| code | 2840 |
| data | 704261 |
| padding | 922 |
| unused | 195584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rainbow Islands (1987)(Ocean)[cr VF].adf:bootblock | 1 | 15 | 1 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr VF].adf boot-block load | 1 | 167 | 2 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr VF].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 101 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 667 | 60 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 10 |
| Rainbow Islands (1987)(Ocean)[cr VF].adf bytes 34816-88576 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr VF].adf bytes 91648-120832 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr VF].adf bytes 256000-267264 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr VF].adf bytes 889344-890880 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: custom-format (non-DOS) disk, 1373 non-empty blocks: matched against track-loaded segments
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: the boot block reads 5632 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: the boot block reads 1024 bytes from disk offset 884224 to an address (found statically); recovered as a unit
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: bytes 34816-88576 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: bytes 91648-120832 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: bytes 256000-267264 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: bytes 889344-890880 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: 6 logged loads cover 102400 bytes of the image
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 22-29 (bytes 11264-15360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.12)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 32-39 (bytes 16384-20480) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.95)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 65-66 (bytes 33280-34304) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.29)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 242-498 (bytes 123904-255488) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.45)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 523-523 (bytes 267776-268288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 525-525 (bytes 268800-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.48)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 527-551 (bytes 269824-282624) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.26)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 553-668 (bytes 283136-342528) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.77)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 671-727 (bytes 343552-372736) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.83)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 729-823 (bytes 373248-421888) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.17)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 825-925 (bytes 422400-474112) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.59)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 927-1037 (bytes 474624-531456) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.04)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1039-1149 (bytes 531968-588800) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.22)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1151-1273 (bytes 589312-652288) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.59)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1275-1336 (bytes 652800-684544) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.43)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1341-1341 (bytes 686592-687104) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.65)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1347-1347 (bytes 689664-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1349-1400 (bytes 690688-717312) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.77)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1407-1407 (bytes 720384-720896) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1418-1418 (bytes 726016-726528) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.50)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1429-1429 (bytes 731648-732160) hold ASCII text
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1440-1440 (bytes 737280-737792) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.95)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1451-1451 (bytes 742912-743424) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.04)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1462-1473 (bytes 748544-754688) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1484-1484 (bytes 759808-760320) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1495-1495 (bytes 765440-765952) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.92)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1506-1506 (bytes 771072-771584) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.04)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1517-1517 (bytes 776704-777216) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.06)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1528-1528 (bytes 782336-782848) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.58)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1539-1539 (bytes 787968-788480) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.57)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1550-1550 (bytes 793600-794112) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.41)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1561-1561 (bytes 799232-799744) hold ASCII text
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1572-1572 (bytes 804864-805376) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.37)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1583-1583 (bytes 810496-811008) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.65)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1594-1594 (bytes 816128-816640) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.54)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1605-1606 (bytes 821760-822784) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.84)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1616-1616 (bytes 827392-827904) outside the filesystem hold data, not code (opcode density 0.008, entropy 3.54)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1627-1627 (bytes 833024-833536) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.47)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1638-1638 (bytes 838656-839168) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.03)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1649-1649 (bytes 844288-844800) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.40)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1660-1660 (bytes 849920-850432) hold ASCII text
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1671-1671 (bytes 855552-856064) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.58)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1682-1682 (bytes 861184-861696) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.10)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1693-1693 (bytes 866816-867328) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.69)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1704-1704 (bytes 872448-872960) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.97)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1715-1715 (bytes 878080-878592) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1748-1748 (bytes 894976-895488) hold ASCII text
- rainbow islands (1987)(ocean)[cr vf]/Rainbow Islands (1987)(Ocean)[cr VF].adf: blocks 1759-1759 (bytes 900608-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.00)
- 4 never-loaded region(s), 95744 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rainbow Islands (1987)(Ocean)[cr VF].adf:bootblock

- `unpack_bitstream_then_jump`: Stores the four bytes of a 32-bit constant into four memory cells, then runs a bit-stream decoder that copies earlier output bytes backwards and finally calls a computed address.

### logged disk load from Rainbow Islands (1987)(Ocean)[cr VF].adf boot-block load

- `shift_out_bits_from_stream`: Shifts a count of bits out of a backward-reading bit buffer, loading the previous byte when the buffer empties and setting its top bit as a marker.
- `consume_bits_until_count_expires`: Reads bytes backward from a buffer, shifting each one right a bit at a time and setting a marker bit, until the count reaches minus one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_call`: Calculates an address by adding two values and jumps to that location to execute code.
- `increment_counter_and_call_handler`: Increments a global counter, then calls a routine whose address is read from a pointer cell plus a 16-bit offset stored 38 bytes into the structure it points to.
- `call_offset_eight`: Loads a pointer from a table at offset 2338 of a structure in a5, then calls a routine at an offset of 8 from the second pointer in that table.
- `call_offset_hundred`: Loads a pointer from a table at offset 2338 of a structure in a5, then calls a routine at an offset of 100 from the second pointer in that table.
- `compare_and_trap`: Compares a 16-bit value at offset 2344 of a structure in a5 with itself, and if they are not equal, executes a trap and returns.
- `load_and_call_random_filler`: Loads a 16-bit word from offset 4 of a structure in a5 into d0 and calls another routine.
- `fill_table_with_pseudo_random`: Fills a table starting at offset 2352 of a structure in a5 with 128 16-bit values calculated using a multiplication and division algorithm, then stores the last value at offset 2608 of the structure in a5.
- `update_structure_with_math`: Increments three bytes at specific offsets in a structure in a5, calculates a new value using a multiplication and division algorithm based on a value at offset 2608, and adds this to two values retrieved from a table to store the result in another table location.
- `increment_and_return_word`: Increments a byte at offset 2613 of a structure in a5 and returns a 16-bit word from the address stored at offset 2610 of the same structure.
- `dispatch_pending_interrupt_levels`: Reads the interrupt enable and request registers, masks the result to interrupt bits four to six, writes the masked bits to the interrupt request register, and calls up to three level handlers through function cells according to those bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_indirect_jump`: Adds a short value to a long value to determine a target address for a function call.
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
- `clear_four_long_fields`: Zeroes four consecutive 32-bit fields at offset 30 of the structure in a6 and then calls another routine.
- `test_bit_from_scaled_index`: Copies a 16-bit value from one register to another, shifts the second value right by three bits, and tests a bit in a table located at an offset of forty-eight plus the shifted value from a pointer in a6 before returning.
- `save_registers_and_branch`: Saves d2 and d3 to the stack, clears d0, and zeroes d3 before branching to another routine.
- `set_bits_from_table`: Reads a sequence of 16-bit words from a pointer in a0, using each word to check a bit in a table at an offset from a6 and setting a bit in d0 and incrementing d3 accordingly.
- `check_table_bit`: Loads a byte from a pointer in a2 into d0, and if d0 is not negative, checks a bit in a table at an offset from a6 using an index derived from d0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `store_byte_and_load_table`: Stores a byte from d1 into a structure at offset 30 of a6, and if d1 is not zero, loads a pointer from a table into four offsets of the structure in a6 and pushes d0 to the stack before branching to another routine.
- `modify_bit_in_structure`: Performs a bitwise XOR on a bit within a structure at offset 32 plus a scaled value from d1, then enters a loop that reads 16-bit words from a table and calls a routine via a pointer until a negative word is encountered.
- `set_structure_pointers_conditionally`: Tests a byte in d1; if it is non-zero, it branches to another routine, otherwise it stores two 32-bit constants at offsets 20 and 24 of a structure in a6 and branches to another routine.
- `set_structure_pointers`: Stores two 32-bit constants at offsets 20 and 24 of a structure in a6 and then calls another routine.
- `process_conditional_chain`: Checks a 16-bit value at offset 46 of a structure in a6, and if it is one, executes a sequence of three routine calls via pointers in a4, potentially decrementing a 16-bit value at offset 32 of a structure in a6 and triggering a trap if a borrow occurs.
- `decrement_and_call_routine`: Decrements a 16-bit value at offset 4 of a structure pointed to by offset 20 of a6, triggers a trap if a borrow occurs, calls a routine at a fixed address, and decrements a 16-bit value at offset 30 of a6.
- `calculate_division_result`: Loads two pointers from a structure in a6, performs a calculation involving a multiplication and division using values from a table, and then branches based on a comparison of d1 with 1.
- `process_data_loop`: Decrements d0 in a loop while fetching words from a structure in a0 using an offset derived from d1, then calls another routine, and finally executes a loop that calls a function pointer retrieved from a table using an offset stored in a structure.
- `store_word_and_branch`: Stores a 16-bit word from d4 into a memory location at offset d2 of a structure in a0 and branches to another routine.
- `conditional_branch_and_move`: Tests d2 and branches to another routine if it is non-zero; otherwise it clears d5 and branches to another routine.
- `setup_loop_parameters`: Writes a 16-bit value to offset 0 of a structure in a0, calls another routine, and then loads several values from a2 and a6 into registers before branching.
- `loop_and_call_function`: Executes a loop that calls a function pointer retrieved from a table using an offset stored in a structure, incrementing d4 if the return value is not negative, and repeats this d2 times.
- `prepare_offset_calculation`: Loads a word from offset 6 of a structure into d3, decrements d3 by 2, loads the address at offset 2 of a0 into a1, sets d4 to 255, and branches to another routine.
- `loop_and_calculate_offset`: Enters a loop that tests a word from a table in a1 and decrements d3, and if the word is negative, performs arithmetic to calculate a value in d4 based on a1 and a0 before branching to another routine.
- `call_routine_if_nonnegative`: Calls another routine only when a 16-bit value passed in a data register is nonnegative, and otherwise returns without doing anything.
- `clear_linked_cell_chain`: Follows a chain of 16-bit links starting from a table index, zeroing each visited cell, and stops when it reaches a negative link.
- `call_table_function`: Calls a function pointer retrieved from a table using an offset stored at offset 46 of a structure pointed to by offset 26 of a6.
- `decrement_two_counters`: Decrements two 16-bit values located at offsets 4 and 26 of structures pointed to by offsets 14 and 26 of a structure in a6, calling a trap if a borrow occurs during either subtraction.
- `update_and_call_pointers`: Stores a byte from d0 into a structure at offset 36, stores a pointer from a0 into offset 14, increments a 16-bit value at offset 4 of the pointer in a0, updates a pointer in a6 using a value from a5, increments a 16-bit value at offset 4 of that new pointer, clears a 16-bit value at offset 30, and calls a routine at the address in a0.
- `compare_byte_to_stored_flag`: Compares the byte in d0 with the byte stored at offset 36 in the structure in a6, returning all ones if they match and zero otherwise.
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
- `decrement_structure_field`: Decrements a 16-bit integer at an offset within a structure and triggers a trap if an arithmetic overflow occurs.
- `update_structure_and_call_2`: Calls another routine, clears a 16-bit word at offset 28 of a structure in a6, and returns.
- `increment_flag_and_call`: The routine checks a short flag at an offset from a pointer in register A6.
- `execute_callback_chain`: Walks a singly linked list starting from a pointer held in a structure, calling the routine at offset 16 in each node until a node's next pointer is zero.
- `call_stored_entry_point`: Copies a pointer from offset 20 of a structure in a6 to a1 and calls the routine at that address.
- `update_structure_and_call`: Stores a byte from d0 at offset 14 of a structure in a6, stores a pointer from a0 at offset 16 of a6, calls a routine using a pointer and offset from a4, and if the result in a0 is not zero, updates the pointer at offset 16 of a6 with the new value.
- `call_stored_entry_at`: Copies a pointer from offset 24 of a structure in a6 to a1 and calls the routine at that address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_once_then_increment_counter`: Calls a routine through a table entry when a 16-bit counter in the structure is zero, then increments the counter.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_structure_callback`: Calls the routine whose address is stored 20 bytes into the structure pointed to by the base register, then returns.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
