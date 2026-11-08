# Game 346 (1000cc Turbo): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00346). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909136 top-level bytes in 22 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 93 of 93 named (0 author symbols), 93 of 93 described (16 by rule, 73 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1174 of 1174 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2346 of 2362 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 461923 |
| code | 8888 |
| data | 55491 |
| padding | 4466 |
| unused | 378368 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| 1000cc Turbo.adf:SUPPLEX | 2 | 888 | 33 |
| 1000cc Turbo.adf:Crystal | 2 | 85 | 3 |
| 1000cc Turbo.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1378 | 55 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 9 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 598 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_system_and_jump_to_main` (SUPPLEX): Sets up hardware registers, populates a table with incrementing values and specific constants, and initializes several global pointers and interrupt vectors before performing a series of subroutines and jumping to a calculated address in a jump table. Kind: disk/track check and loader. Pass path: The loop involving `piVar7` (iterating through a list of data/track pointers) and the subsequent calls to `u0__sub_0_1568` and `u0__sub_0_033A` represent the loading of protected data after hardware/track validation..
- `initialize_audio_subsystems` (runtime segment an address): Saves registers to the stack, modifies hardware registers, clears several memory locations, and calls another routine three times with different source and destination pointers. Kind: loader/subsystem initialization with potential integrity check. Pass path: The instruction immediately following the call (the branch taken when the routine returns normally without triggering a crash or reset)..
- `initialize_and_validate_data_structure` (runtime segment an address): Clears a block of memory of 543 words, configures hardware registers, and validates a data stream by performing a checksum-like calculation on a sequence of bytes and comparing it against a stored value in a table. Kind: checksum and data structure validation. Pass path: u3__l0_0714.
- `set_hardware_bit` (runtime segment an address): Sets bit 2 of a hardware register at address a fixed address and returns. Kind: disk hardware/status check. Pass path: the branch not taken by beq.b an address (the bsr.b an address path).

## Functions


### 1000cc Turbo.adf:SUPPLEX

- `initialize_system_and_jump_to_main`: Sets up hardware registers, populates a table with incrementing values and specific constants, and initializes several global pointers and interrupt vectors before performing a series of subroutines and jumping to a calculated address in a jump table.
- `call_old_open_library`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `copy_and_decompress_data`: Copies a byte from a source to a destination at offset 512, then enters a loop that copies words from a source to a destination at offset 768 based on a counter and specific marker values.
- `jump_to_address`: Stores a longword into a memory location and then jumps to a longword address.
- `update_counter_and_pointer`: Increments a 16-bit value in a global variable and, if it reaches 215, bitwise inverts another global variable and clears the first; otherwise, it manages a 16-bit value by adding or subtracting 4096 and calls another routine.
- `initialize_structure_from_pointer`: Loops until a pointer in a global variable is non-negative, then uses that pointer to populate a structure in a6 with various values, including a calculated address and a bit-tested value.
- `copy_data_with_back_references`: Iterates through a source buffer, handling specific byte markers to either clear a destination word and increment a pointer or copy blocks of data from a source to a destination using a loop.
- `adjust_global_offset`: Updates a global variable with a value from offset 10 of a structure in a6, then calculates a difference between the high bytes of two 16-bit values to add to a third global variable, which is then clamped between 0 and 10.
- `tiny_ori`: Small routine of 1 instruction that merges 1 into d1.
- `update_counter_and_calculate_offset`: Decrements a global word and, if it reaches zero, checks a bit in a structure in a6 to update a table of pointers and toggle bytes in a table.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d7.
- `calculate_and_store_offset`: Checks a 16-bit value in a global variable and, depending on its value, clears a longword, calculates a new word using a multiplication and addition, stores it in a global variable, and calls another routine.
- `update_status_and_table`: Compares a byte from a memory location to 70; if it matches, it checks if a word in a global variable is non-zero and if another global variable equals 2, then it writes three bytes from a constant to a table, clears a word at a specific offset in a global variable, clears the original byte, and branches to another routine; otherwise it returns.
- `handle_interrupt_request`: Checks a bit in a hardware register and, if set, writes a value to another hardware register, saves registers, calls another routine, restores registers, and returns from interrupt.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `process_serial_data`: Checks a bit in a hardware register, reads a byte from a serial register, performs bitwise operations on it, stores the result in a global variable, and enters a loop that runs up to 500 times before returning.
- `decrement_buffer_values`: Loops 99 times, checking bytes in a buffer and either replacing them with a constant or subtracting an incrementing value from them.
- `initialize_audio_data`: Populates a table with incrementing values, finds a maximum value in a byte array, and clears several memory locations and audio-related structures.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `mt_music`: ProTracker music replay tick, called once per frame: advances the song and feeds new notes and effects to the four Paula audio channels.
- `configure_audio_hardware`: Checks a byte at a specific address, moves a word from a table to a hardware register, stores a longword into a global variable, moves a word to a hardware register, and returns from an interrupt.
- `set_audio_registers`: Checks a byte in a hardware register and, if non-zero, stores a series of specific values from a table into several custom audio registers and sets a global variable.
- `update_music_parameter_state`: Loads a structure from a table using a combined index from a byte at offset 2 and the high nibble of a byte at offset 0, then updates several fields in that structure using data from a secondary table.
- `update_parameter_state_and_dma`: Updates a 16-bit value in a structure by adding a register value to it, then performs conditional logic based on a control byte to either update a global variable, modify a DMA configuration via a pointer, or adjust specific bytes and words within a structure and a table.
- `update_scaled_value`: Checks a 16-bit value at an offset of 34 from a pointer and, if non-zero, modifies a 16-bit value at an offset of 16 from the same pointer by adding or subtracting a byte from an offset of 21, then stores the result at an offset of 6 from a second pointer.
- `update_value_with_scaled_offset`: Calculates a signed, scaled value using bits from a source pointer and adds it to a target location, then increments a byte in a table.
- `process_scaled_parameter_updates`: Reads a control byte from a structure to determine an operation type, then performs arithmetic on 16-bit or 8-bit values at specific offsets within that structure or a second structure.

### 1000cc Turbo.adf:Crystal

- `depack_packed_stream_backward`: Decodes a packed stream by calling a bit-reader refill and a multi-bit read routine, copying literal and back-reference runs backwards into memory.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### 1000cc Turbo.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_subsystems`: Saves registers to the stack, modifies hardware registers, clears several memory locations, and calls another routine three times with different source and destination pointers.
- `process_input_and_initialize_audio`: Reads bytes from a buffer to update a structure at a0, performs several conditional initializations of a structure in a6, and executes a loop that populates memory from tables based on byte values.
- `configure_hardware_and_jump`: Sets the color, DMA, and interrupt enable registers, updates the stack pointer, and then calls another routine before performing an indirect jump through a table.
- `initialize_and_validate_data_structure`: Clears a block of memory of 543 words, configures hardware registers, and validates a data stream by performing a checksum-like calculation on a sequence of bytes and comparing it against a stored value in a table.
- `compare_and_copy_data_stream`: Compares a sequence of bytes from a source buffer against a target buffer, and if a match is found, performs a series of operations involving multiplying a value by eleven, updating a global counter, and copying a block of data from a source to a destination using a loop.
- `rotate_and_store_bytes`: Adds 4 to a pointer in a0, rotates d0 left by 8 bits, stores the byte from d0 to the destination at a2, and loops until a2 matches a0.
- `move_a0_to_d0_and_branch`: Moves the value from a0 to d0 and then branches to another location in the code.
- `call_routine_subtract_then_call`: Calls another routine and, if its result is nonzero, subtracts a stack argument from that result and calls another routine.
- `wait_for_word_then_clear`: Waits until a word in a structure becomes nonzero, giving up when a large countdown runs out.
- `poll_routine_until_result`: Repeatedly calls a routine and tests its result and a flag, calling two other routines on each pass until the flag is set.
- `decrement_global_counter`: Decrements a global counter by one and returns.
- `store_routine_results_in_block`: Repeatedly calls another routine and stores its results in consecutive longwords, up to 128 entries, stopping early when a result is zero or if a specific flag in d6 is set.
- `divide_by_eleven_and_reset`: Sets a global counter to five and returns the input word divided by eleven.
- `execute_conditional_calls`: Calls another routine, tests bit 4 of a byte at offset 240 of a structure in a6, and if set, calls a third routine before branching back to an earlier instruction.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_moveq`: Small routine of 1 instruction that loads 135 into d1.
- `store_byte_to_structure`: Calls another routine, stores a byte from a pointer in a6, and then stores the byte from d1 into the same location.
- `set_hardware_bit`: Sets bit 2 of a hardware register at address a fixed address and returns.
- `clear_indexed_bit_and_wait`: Calls another routine to get a bit index and clears that bit in a byte at the pointer in a6, then calls two other routines.
- `clear_bit_and_branch`: Clears bit 1 of a byte at an offset of a structure in a6 and branches to another routine.
- `set_flag_bits_and_delay`: Sets one flag bit, clears another and sets bit zero of a control byte, then runs a wait loop of sixty-four passes that polls a hardware byte.
- `process_timer_and_update_flag`: Swaps the contents of a register, extracts a value from the high word, and clears bit 2 of a byte at the address pointed to by a6 if a specific bit is set.
- `decrement_and_branch`: Calls a routine that sets flag bits and runs a wait loop, decrements register d2 by one, and then branches to another location.
- `swap_and_clear_memory`: Swaps the contents of d7, loads a new pointer into a6 from the stack, writes the value 16384 to the address in a5, and if the longword at a4 is not zero, clears the longword at a4 and branches to another location; otherwise it branches to another location.
- `synchronize_and_validate_data_structure`: Waits for a word in a structure to become non-zero, clears specific words in that structure, and validates a magic constant.
- `call_wait_routine`: Loads the address of a word at offset 4 of the structure in a4 into a6, calls another routine that waits for a word in a structure to become nonzero before timing out, updates a4 with the value in a6, and returns if d1 is not zero.
- `process_indexed_table_data_2`: Checks a constant at the start of a buffer, copies 128 words from that buffer to another location, and then performs a series of calculations and data movements involving multiple calls to other routines and loops.
- `shift_and_exit`: Performs a logical right shift on a longword in d4 through d5 and returns if the result is not zero; otherwise it returns.
- `rotate_and_return`: Calls a routine that compares two pointers in a0 and a4 and loads a longword into d5 if they differ, rotates the long value in d4 into d5, and returns.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `loop_rotate_left`: Performs a logical right shift on a longword in d4 through d5 and, if the result is not zero, enters a loop that rotates the longword in d4 through d1 and decrements a counter in d0 until it reaches negative one.
- `rotate_and_branch`: Calls a routine that compares two pointers in a0 and a4 and loads a longword into d5 if they differ, rotates the long value in d4 into d5, and then branches to another location.
- `process_indexed_table_data`: Loads a byte from a table using an index in d1, compares the index to a value in d6, and if equal, performs a series of calculations involving a loop that adds values and calls other routines.
- `compare_and_load_pointer`: Compares two pointers in registers a0 and a4, and if they are not equal, loads a longword from the address pointed to by a1 into register d5 before returning.
- `copy_and_loop`: Calls a routine that subtracts 1 from d0, moves d1 into d3, adds d4 to d2, moves a byte from the address at a3 plus the offset in d3 to the location pointed to by a3 while decrementing a3, loops using d2, and if a3 is greater than or equal to a2, returns; otherwise it branches to another location.
- `zero_word_data`: Performs a bitwise OR of register d0 with zero.
- `process_coordinate_data_batches`: Iteratively processes batches of coordinate data by loading values from tables into a structure and calling another routine.
- `initialize_structure_parameters`: Sets a 32-bit value at offset 64 of a structure in a6, copies a pointer to a6 offset 84, clears a 16-bit word at offset 102, and writes 276 and 33248 to offsets 88 and 150 respectively.
- `batch_calculate_coordinate_offsets`: Repeatedly loads four 16-bit values from a source list into registers and calls calculate_coordinate_offset to process them into a destination structure pointed to by a1.
- `calculate_coordinate_offset`: Calculates a value based on the difference between two pairs of words, stores several calculated values and pointers into a structure pointed to by a6, and returns.
- `update_bitfield_from_tables`: Updates a global variable by XORing it with a constant, then enters a loop that reads values from four tables to set bits in a bitfield at a memory location pointed to by a2.
- `tiny_ori_3`: Small routine of 1 instruction that merges 172 into d7.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `tiny_and_ori`: Small routine of 2 instructions that masks d0 with d0, then merges 0 into d0.
- `initialize_audio_and_sample_data`: Sets audio hardware registers to specific period and volume values, configures a DMA-driven structure with sample pointers and metadata, and calls another routine four times to process individual sample data entries.
- `initialize_structure_fields`: Clears multiple fields in a structure pointed to by a0, sets a byte at offset 29 to 63, and populates several other fields using values from a table indexed by a calculated offset.
- `update_dma_and_return`: Modifies a global byte and a counter, calls four routines with different pointer offsets, writes a word to the DMA controller register, and returns via an exception return.
- `update_entity_state_and_counters_3`: Updates an entity structure by managing its pointer and state, performs table lookups to retrieve data, and executes an indirect jump to another routine based on a calculated offset.
- `branch_to_next_instruction`: Performs a short branch to another location.
- `branch_to_offset_7006`: Performs a short branch to a location 7006 bytes ahead in the code.
- `update_entity_state_and_counters_2`: Updates multiple counters and state variables within a structure in a0 by reading from a data table, including decrementing timers, updating position or velocity values, and performing a loop to find a non-null entry in a linked list or table.
- `branch_to_offset_2`: Performs a short branch to a specific instruction offset.
- `update_ciaa_status_bit`: Stores the value of d1 into a global variable at offset 47 of the structure in a6, sets bit 0 of the CIAA_PRA register, or clears bit 0 of the CIAA_PRA register before branching to other routines.
- `negate_and_store_word`: Negates the word in d1, stores the result into a global variable at offset 30 of the structure in a0, or stores the original value of d1 into offset 29 of the structure in a0 before branching to another routine.
- `update_entity_state_and_counters`: Iterates through a table to find a valid entry, updates a pointer and a counter in a structure, and performs several arithmetic operations on various counters and values within the same structure, including bitwise inversion and clamping values to a maximum of twelve or fifteen.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset`: Performs a short branch to an offset of 152 bytes.
