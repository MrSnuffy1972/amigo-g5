# Game 1610 (rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01610). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909216 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 117 of 117 named (0 author symbols), 117 of 117 described (10 by rule, 52 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 117 of 117 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 94208 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1301 of 1301 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 63856 |
| code | 4062 |
| data | 684110 |
| padding | 4 |
| unused | 157184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf:bootblock | 1 | 8 | 1 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf boot-block load | 1 | 384 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 231 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 667 | 58 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 10 |
| Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf bytes 34816-88576 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf bytes 91648-120832 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf bytes 256000-267264 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: custom-format (non-DOS) disk, 1440 non-empty blocks: matched against track-loaded segments
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: the boot block reads 67584 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- runtime segment an address: 62 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 54 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: bytes 34816-88576 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: bytes 91648-120832 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: bytes 256000-267264 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: 4 logged loads cover 161792 bytes of the image
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 22-29 (bytes 11264-15360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.12)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 32-39 (bytes 16384-20480) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.95)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 65-66 (bytes 33280-34304) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.29)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 242-498 (bytes 123904-255488) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.45)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 523-523 (bytes 267776-268288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 525-525 (bytes 268800-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.48)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 527-551 (bytes 269824-282624) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.26)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 553-727 (bytes 283136-372736) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.77)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 729-823 (bytes 373248-421888) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.17)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 825-925 (bytes 422400-474112) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.53)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 927-1037 (bytes 474624-531456) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.90)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 1039-1149 (bytes 531968-588800) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.05)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 1151-1273 (bytes 589312-652288) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.39)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 1275-1336 (bytes 652800-684544) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.86)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 1347-1347 (bytes 689664-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 1349-1400 (bytes 690688-717312) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.33)
- 3 never-loaded region(s), 94208 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_adkcon_via_table` (Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf boot-block load +an address to an address (67584 bytes)): Loads a byte from the address in a1, increments a1, sign-extends it to a word, masks it with a constant, and writes the result to the ADKCON register, then branches to another routine. Kind: custom hardware/data-driven lookup protection. Pass path: The routine processes a stream of bytes from a data source (likely a custom disk format) to populate internal object/hardware registers (CUSTOM_ADKCON, CUSTOM_DMACON). A successful load/check allows the routine to complete its loop and proc.

## Functions


### Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf:bootblock

- `issue_boot_block_io_request`: Fills an exec I/O request block with a command, a length and two addresses, issues it through the DoIO library call, then calls a library vector, three unnamed routines, and clears a global before entering a loop.

### logged disk load from Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf boot-block load

- `wait_for_hardware_status`: Polls the vertical position register until a specific bit is set, calls a routine to update audio DMA and interrupts, and then checks a bit in the CIAA Port A register, returning if the bit is not set.
- `update_audio_dma_and_interrupts`: Updates several status bytes using a mask of 1920, which selects the audio interrupt bits, and adds a value to a stored byte with carry.
- `decrement_timer_step_value`: Decrements a countdown byte in an object; when it reaches zero it reloads it from a table, and a zero table entry stops the update.
- `process_byte_block_with_helpers`: A long routine that works on a block of bytes through several pointers and calls three unnamed routines.
- `reset_state_and_control_words`: Clears five state words in program data, then writes two control values to registers listed in the facts as the disk-and-audio control register and the DMA control register.
- `copy_counted_byte_run`: Reads a count from the input bytes and copies data between pointers in a byte loop.
- `set_bits_and_branch`: Sets bit 5 and bit 0 of the byte at the address in a0, then branches to another routine.
- `set_bit_and_branch`: Sets bit 0 of the byte at the address in a0, then branches to another routine.
- `copy_byte_and_branch`: Copies a byte from the address in a1, incrementing a1, to offset 24 of the structure in a0, then branches to another routine.
- `update_multiple_fields`: Copies bytes from the address in a1 to offsets 8, 9, and 19 of the address in a0, clears a 16-bit word at the address in a0, sets bit 2 of the byte at the address in a0, and then branches to another routine.
- `process_byte_and_set_bit`: Copies a byte from the address in a1 to offset 22 of the address in a0, copies the next byte from a1 to d0 and then to offset 23 of the address in a0, adds d0 to itself, stores the result in offset 21 of the address in a0, sets bit 6 of the byte at the address in a0, and then branches to another routine.
- `copy_byte_to_offset`: Copies a byte from the address in a1, incrementing a1, to offset 16 of the structure in a0, then branches to another routine.
- `set_multiple_bits_and_branch`: Sets bit 2 and bit 1 of the byte at the address in a0, then branches to another routine.
- `clear_byte_and_branch`: Clears the byte at the address in a0, then branches to another routine.
- `update_adkcon_via_table`: Loads a byte from the address in a1, increments a1, sign-extends it to a word, masks it with a constant, and writes the result to the ADKCON register, then branches to another routine.
- `advance_counted_byte_copy`: Decrements a count byte in a structure and, while it stays non-zero, copies bytes between pointers, calling one unnamed routine.
- `reset_object_timer_from_input`: When an object's mode byte is clear, it zeroes a counter, sets the mode value, and stores a byte in a global.
- `lookup_table_into_object_fields`: Looks up a byte in a table indexed by a 16-bit value and stores it, along with a second table byte, into two fields of an object.
- `update_value_from_table_record`: When a mode byte is set, it looks up a ten-byte table record by index, decrements a counter, and stores a difference and an offset in the output fields.
- `copy_word_table_passes`: Runs a loop of fifteen passes over a table in program data, copying 16-bit words between pointers.
- `copy_table_to_memory`: Calculates an offset based on an input value, zeroes a byte at that location, and then copies a series of words from a table to a destination block.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_return`: Calls a routine using a pointer from a4 and an offset in d7, then restores a4, a6, a2, and a3 from the stack before returning.
- `increment_counter_and_call_handler`: Increments a global counter, then calls a routine whose address is read from a pointer cell plus a 16-bit offset stored 38 bytes into the structure it points to.
- `call_offset_eight`: Loads a pointer from a table at offset 2338 of a structure in a5, then calls a routine at an offset of 8 from the second pointer in that table.
- `call_offset_hundred`: Loads a pointer from a table at offset 2338 of a structure in a5, then calls a routine at an offset of 100 from the second pointer in that table.
- `compare_and_trap`: Compares a 16-bit value at offset 2344 of a structure in a5 with itself, and if they are not equal, executes a trap and returns.
- `load_word_and_randomize`: Loads a 16-bit word from offset 4 of a structure in a5 into d0 and calls a pseudo-random routine.
- `fill_table_with_pseudo_random`: Fills a table starting at offset 2352 of a structure in a5 with 128 16-bit values calculated using a multiplication and division algorithm, then stores the last value at offset 2608 of the structure in a5.
- `update_structure_with_math`: Increments three bytes at specific offsets in a structure in a5, calculates a new value using a multiplication and division algorithm based on a value at offset 2608, and adds this to two values retrieved from a table to store the result in another table location.
- `increment_and_return_word`: Increments a byte at offset 531 of a structure in a5 and returns a 16-bit word from the address stored at offset 530 of the same structure.
- `toggle_interrupt_mask_and_bit`: Disables interrupts, reads a byte from the CIA interrupt register, performs a bitwise XOR between that byte and a byte at offset 2621 of a structure in a5, stores the result back to that offset, and re-enables interrupts.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `process_interrupt_and_dispatch`: Saves d0, a0, and a5 to the stack, reads a byte from the CIAB input register, updates a byte at offset 2621 of a structure in a5, and conditionally calls several routines based on bit shifts and sign checks.
- `call_offset_a46`: Loads an address from offset 2602 of a structure in a5 into a0, calls the routine at that address, and branches to another location.
- `call_offset_a4a`: Loads an address from offset 2634 of a structure in a5 into a0, calls the routine at that address, and branches to another location.
- `call_offset_a4e`: Loads an address from offset 2638 of a structure in a5 into a0, calls the routine at that address, and branches to another location.
- `call_offset_a52`: Loads an address from offset 2642 of a structure in a5 into a0, calls the routine at that address, and branches to another location.
- `call_indirect_pointer`: Loads a longword from an offset of a structure in a5 into a0, calls the routine at that address, and then branches to another routine.
- `handle_interrupt_request_bits`: Saves d0, a0, and a5 to the stack, reads the interrupt status and mask from custom registers, updates the interrupt request register, and conditionally calls two routines based on bit tests.
- `call_offset_a5a`: Loads an address from offset 2650 of a structure in a5 into a0, calls the routine at that address, and branches to another location.
- `call_offset_a5e`: Loads an address from offset 2654 of a structure in a5 into a0, calls the routine at that address, and branches to another location.
- `dispatch_interrupt_subsets`: Saves d0, a0, and a5 to the stack, reads the interrupt status and mask from custom registers, updates the interrupt request register, and conditionally calls up to four routines based on bit shifts and sign checks.
- `call_offset_a6e`: Loads an address from offset 2670 of a structure in a5 into a0, calls the routine at that address, and branches to another location.
- `call_offset_a6a`: Loads an address from offset 2666 of a structure in a5 into a0, calls the routine at that address, and branches to another location.
- `call_pointer_from_a5_offset_a66`: Loads a long value from offset 102 of a structure in a5 into a0, calls the routine at that address, and then branches to another location.
- `call_pointer_from_a5_offset_a62`: Loads a long value from offset 98 of a structure in a5 into a0, calls the routine at that address, and then branches to another location.
- `dispatch_interrupt_requests`: Calculates a value based on the intersection of the interrupt enable and interrupt request registers, writes it to the interrupt acknowledge register, and calls one of three routines based on the resulting value.
- `handle_cia_interrupt`: Reads the CIA interrupt register, updates a byte in a structure in a5, and if a specific bitwise condition is met, performs a series of shifts and additions before calling one of five routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_indirect_call_by_offset_2`: Calls a routine at an address formed by adding a 16-bit offset to a base pointer register, then returns.
- `store_word_triple`: Copies three consecutive 16-bit values from a source pointer into three fields of a structure, then returns minus one.
- `clear_three_word_fields`: Clears three consecutive 16-bit fields at offsets 30, 32, and 34 of a structure in a6 and then calls another routine.
- `read_long_field_at_offset`: Returns the 32-bit value stored at offset 30 in the record passed through an address register.
- `process_bitfield_and_update_table`: Checks a bit in a bitfield based on a byte from a pointer in a2, and if the bit is set, updates a 16-bit counter in a structure in a6 and stores a 16-bit value composed of a byte from a5 and the input byte into a table at an offset determined by the counter.
- `copy_word_within_structure`: Copies a 16-bit word from offset 48 of a structure in a6 to offset 50 of the same structure.
- `clear_two_word_fields_and_call`: Zeroes two 16-bit fields at offsets 30 and 32 of the structure in a6, then calls another routine.
- `check_word_equality`: Compares a 16-bit word at offset 48 of a structure in a6 with a 16-bit word at offset 50 of the same structure, returning zero if they are equal and negative one otherwise.
- `execute_and_increment_counter`: Repeatedly calls a function pointer retrieved from a table at offset 54 of a structure in a4 until the return value is zero, then increments a 16-bit value at offset 32 of a structure in a6 by two, masks it with 31, and calls a routine at a fixed address.
- `test_and_set_bits`: Loads a byte from a2, and if it is positive, performs bitwise operations to set or clear bits in a table at offset 48 or 48 plus an offset derived from the byte at offset 48 of a structure in a6.
- `clear_four_long_fields`: Zeroes four consecutive 32-bit fields at offset 48 of the record in a6 and then calls another routine.
- `test_bit_from_offset_table`: Copies a 16-bit value from one register to another, shifts the second value right by 3 bits, and tests a bit in a table located at an offset determined by that shifted value relative to a pointer in a6.
- `initialize_d3_and_branch`: Saves d2 and d3 to the stack, clears d3, and branches to another location.
- `set_bits_from_table`: Reads a sequence of 16-bit words from a pointer in a0, using each word to check a bit in a table at an offset from a6 and setting a bit in d0 and incrementing d3 accordingly.
- `check_table_bit`: Loads a byte from a pointer in a2 into d0, and if d0 is not negative, checks a bit in a table at an offset from a6 using an index derived from d0.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `store_byte_and_load_table`: Stores a byte from d1 into a structure at offset 30 of a6, and if d1 is not zero, loads a pointer from a table into four offsets of the structure in a6 and pushes d0 to the stack before branching to another routine.
- `modify_bit_in_structure`: Performs a bitwise XOR on a bit within a structure at offset 32 plus a scaled value from d1, then enters a loop that reads 16-bit words from a table and calls a routine via a pointer until a negative word is encountered.
- `conditional_set_structure_pointers`: Tests the byte in d1; if it is zero, it stores two 32-bit constants at offsets 20 and 24 of a structure in a6 and then branches to another location.
- `set_structure_pointers`: Stores two 32-bit constants at offsets 20 and 24 of a structure in a6 and then calls another routine.
- `process_conditional_chain`: Checks a 16-bit value at offset 46 of a structure in a6, and if it is one, executes a sequence of three routine calls via pointers in a4, potentially decrementing a 16-bit value at offset 32 of a structure in a6 and triggering a trap if a borrow occurs.
- `decrement_and_call_routine`: Decrements a 16-bit value at offset 4 of a structure pointed to by offset 20 of a6, triggers a trap if a borrow occurs, calls a routine at a fixed address, and decrements a 16-bit value at offset 30 of a6.
- `calculate_division_result`: Loads two pointers from a structure in a6, performs a calculation involving a multiplication and division using values from a table, and then branches based on a comparison of d1 with 1.
- `process_data_loop`: Decrements d0 in a loop while fetching words from a structure in a0 using an offset derived from d1, then calls another routine, and finally executes a loop that calls a function pointer retrieved from a table using an offset stored in a structure.
- `move_and_branch`: Stores a 16-bit word from d4 at an offset determined by d2 in a structure in a0, then branches to another routine.
- `test_and_branch`: Tests the value of d2, and if it is non-zero, branches to another routine; otherwise it clears d5 and branches to another routine.
- `setup_loop_parameters`: Writes a 16-bit value to offset 0 of a structure in a0, calls another routine, and then loads several values from a2 and a6 into registers before branching.
- `loop_and_call_function`: Executes a loop that calls a function pointer retrieved from a table using an offset stored in a structure, incrementing d4 if the return value is not negative, and repeats this d2 times.
- `load_and_branch`: Loads a 16-bit word from offset 6 of a structure in a2 into d3, decrements d3 by 2, sets d4 to 255, and branches to another routine.
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
- `clear_field_and_call`: Clears a 32-bit field at offset 36 of a structure in a6 and then calls another routine.
- `release_secondary_object_then_call`: Decrements the 16-bit counter of the object referenced at offset 50, trapping on signed overflow, and then calls another routine.
- `decrement_held_object_then_call`: Decrements the 16-bit counter of the object referenced at offset 46, trapping on signed overflow, and then calls another routine.
- `decrement_structure_field`: Decrements a short integer at an offset within a structure and triggers a trap if an arithmetic overflow occurs.
- `call_and_clear_field`: Calls another routine, zeroes a 16-bit field at offset 28 of the structure in a6, and then returns.
- `increment_flag_and_call`: The routine checks a short flag at an offset from a pointer in register A6.
- `execute_callback_chain`: Walks a singly linked list whose head pointer is read from a fixed field of a base structure, calling a function pointer stored in each node.
- `call_stored_entry_point`: Copies the address in a6 to a0, loads a pointer from offset 20 of the structure in a6 into a1, and calls the routine at that address.
- `update_structure_and_call`: Stores a byte from d0 at offset 14 of a structure in a6, stores a pointer from a0 at offset 16 of a6, calls a routine using a pointer and offset from a4, and if the result in a0 is not zero, updates the pointer at offset 16 of a6 with the new value.
- `call_stored_entry_at`: Calls the routine whose entry address is stored at offset 24 in the structure in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_bad_instruction_data`: The bytes do not decode to a valid instruction, so there is no real routine.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `clear_bit_range_in_word`: Clears a range of bits in a memory word, with the range computed from a register, then reaches a bad instruction that the decompiler cuts off.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
