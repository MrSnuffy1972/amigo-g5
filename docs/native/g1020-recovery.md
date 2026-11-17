# Game 1020 (1000cc turbo (1990)(simulmondo)[cr csl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01020). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909072 top-level bytes in 22 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 146 of 146 named (0 author symbols), 146 of 146 described (24 by rule, 103 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 967 of 967 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 375578 bytes, 88 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6068 of 6080 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 354903 |
| code | 5320 |
| data | 131451 |
| padding | 4214 |
| unused | 413184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| 1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:Crystal | 2 | 85 | 3 |
| 1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1384 | 54 |
| 1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:Crystal#unpacked | 1 | 1114 | 29 |
| 1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:1000cc#unpacked (raw code file, load address unknown) | 1 | 3487 | 59 |

## Cases handled

- 1000cc turbo (1990)(simulmondo)[cr csl]/1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:1000cc: PowerPacker Data (xfd)-packed, unpacked with ancient (79508 -> 352788 bytes)
- runtime segment an address: 887 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 1000cc turbo (1990)(simulmondo)[cr csl]/1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:Crystal: crunched executable unpacked with ancient scan (ByteKiller 1.3 (xfd), 14216 -> 22790 bytes), 9 of 9 windows confirmed in reference RAM, loaded at an address: recovered as u3
- 1000cc turbo (1990)(simulmondo)[cr csl]/1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:1000cc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware_and_load_samples` (runtime segment an address): Configures custom chipset registers, clears several hardware-related values, and sets up a function pointer in a global variable. Kind: disk hardware/track check. Pass path: The branch following the call that continues normal execution (likely the one not leading to a crash or error message)..
- `validate_and_process_data_stream` (runtime segment an address): Clears a table of 543 entries, configures hardware registers, and validates a data stream by performing arithmetic on bytes and checking against a table. Kind: checksum and data validation trap. Pass path: u2__l0_0714.
- `set_control_bit` (runtime segment an address): Sets bit 2 of a longword at a fixed address and returns. Kind: checksum or hardware status trap. Pass path: the instruction following the call (the routine sets a bit that is subsequently tested by a `btst` or used to validate state).
- `initialize_hardware_and_data` (Crystal#unpacked): Sets several hardware registers and global pointers, clears multiple memory locations, and calls another routine and three instances of a routine that copies data from one location to another. Kind: loader-based disk protection. Pass path: the instruction immediately following the call (assuming the decompile failure is due to a control-flow obfuscation or a jump-table used to hide the valid execution path).
- `initialize_and_process_input_buffer` (Crystal#unpacked): Clears a memory block of 544 words, configures custom hardware registers, and processes a buffer of input data by applying a multiplier and bitmask to each byte before storing the results. Kind: disk track/data integrity check. Pass path: the branch following the successful comparison of the buffer against the expected sequence (u3__l0_0714).
- `frag_lea_bset_rts` (Crystal#unpacked): Short code fragment that loads the address of a fixed memory address into a6, then sets bit 2 of the memory at a6, then returns. Kind: bitmask flag setter (likely status/state update). Pass path: the instruction following the call (the code continues execution normally).

## Functions


### 1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:Crystal

- `depack_packed_stream_backward`: Decodes a packed stream by calling a bit-reader refill and a multi-bit read routine, copying literal and back-reference runs backwards into memory.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### 1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_load_samples`: Configures custom chipset registers, clears several hardware-related values, and sets up a function pointer in a global variable.
- `parse_newline_delimited_data`: Iteratively reads bytes from a source pointed to by a1, performs arithmetic on each byte to calculate an offset, and stores eight bytes from a table into a structure at a2, repeating this process for each line until a newline or null byte is encountered.
- `configure_hardware_and_jump`: Sets the color, DMA, and interrupt enable registers, then calls another routine before performing an indirect jump through a value loaded from a table.
- `validate_and_process_data_stream`: Clears a table of 543 entries, configures hardware registers, and validates a data stream by performing arithmetic on bytes and checking against a table.
- `compare_and_copy_data_blocks`: Compares a sequence of bytes from a source buffer against a target table, and if a match is found, performs a series of calculations involving a multiplication by eleven and copies a block of data from one memory location to another using a loop.
- `rotate_and_store_bytes`: Increments a pointer in a0 by 4, rotates d0 left by 8 bits, stores the resulting byte at the address in a2 and increments a2, then loops if a2 does not equal a0.
- `branch_with_data_move`: Moves the contents of a pointer into a data register and then branches to another location.
- `call_then_adjust_result`: Calls another routine and, if its result is nonzero, subtracts a stack argument from that result and then calls another routine before returning the combined value.
- `wait_for_word_then_clear`: Waits until a word in a structure becomes nonzero, giving up when a large countdown runs out.
- `poll_routine_until_result`: Repeatedly calls a routine and tests its result and a flag, calling two other routines on each pass until the flag is set.
- `decrement_global_counter`: Decrements a global counter by one and returns.
- `store_routine_results_in_block`: Repeatedly calls another routine and stores its results in consecutive longwords, up to 128 entries, stopping early when a result is zero.
- `divide_by_eleven_and_reset`: Sets a global counter to five and returns the input word divided by eleven.
- `check_status_and_call`: Calls another routine, tests bit 4 of a byte at offset 240 of the structure in a6, and if set, calls another routine before branching.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_moveq`: Small routine of 1 instruction that loads 135 into d1.
- `store_register_to_structure`: Calls another routine, stores a byte from a6 into memory, and then stores the word in d1 into the address pointed to by a6.
- `set_control_bit`: Sets bit 2 of a longword at a fixed address and returns.
- `clear_indexed_bit_and_wait`: Calls another routine to get a bit index and clears that bit in a byte at a6, then calls two other routines.
- `clear_structure_bit`: Clears bit 1 of a byte at the address in a6 and then branches.
- `set_flag_bits_and_delay`: Sets one flag bit, clears another and sets bit zero of a control byte, then runs a wait loop of sixty-four passes that polls a hardware byte.
- `update_status_and_adjust_counter`: Calls another routine, swaps the high and low words of a register, and checks a bit in the resulting byte to decide whether to clear bit 2 of a byte at the address in a6.
- `decrement_and_branch`: Calls another routine, decrements d2 by 1, and then branches.
- `swap_and_clear_memory`: Swaps the high and low words of d7, loads a pointer from the stack into a6, writes the word 16384 to the address in a5, and if the longword at a4 is not zero, clears the longword at a4 and branches.
- `synchronize_and_verify_data_structure`: Waits for a word in a structure to become non-zero, clears specific words in that structure, and then performs a series of checks involving register swaps and comparisons to verify data integrity.
- `call_wait_routine`: Loads the address at offset 4 of the structure in a4 into a6, calls another routine that waits for a word in a structure to become nonzero before timing out, updates a4 with the value in a6, and tests d1 before returning.
- `process_and_copy_data_block`: Checks a magic constant in a buffer, copies 128 longwords from one location to another, and then performs a complex series of shifts and byte movements based on values fetched from a structure.
- `shift_longword_and_return`: Performs a logical shift right on a 32-bit value in d4 through d5 and returns if the carry flag is not set after the shift.
- `rotate_and_return`: Calls another routine, rotates the longword in d4 through d5, and returns.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `rotate_longword_in_loop`: Performs a logical shift right on a 32-bit value in d4 through d5, and if the carry flag is set, rotates the value through d1 and repeats the process using a counter in d0.
- `rotate_and_branch`: Calls another routine, rotates the longword in d4 through d5, and then branches.
- `process_table_entry_and_copy_data`: Fetches a byte from a table using an index in d1, compares d1 against a value in d6, and if equal, calls another routine and performs a series of conditional calls and data movements including a loop that copies bytes from a source address to a destination address.
- `compare_and_load_pointer`: Compares the value in a0 with a value in a4; if they are not equal, it returns, otherwise it loads the value from the address pointed to by a1 into d5 and returns.
- `copy_and_loop_data`: Calls another routine, moves d1 to d3, adds d4 to d2, copies a byte from the address in a3 plus the offset in d3 to a new location at the end of the table in a3, loops based on the value in d2, and then compares a3 with a2 before returning.
- `process_coordinate_data_batches`: Iteratively processes batches of coordinate data by loading values from tables into a structure and calling another routine.
- `initialize_structure_parameters`: Sets a 32-bit value at offset 64 of a structure in a6, copies a pointer to a6 offset 84, clears a 16-bit word at offset 102, and writes 276 and 33248 to offsets 88 and 150 respectively.
- `batch_calculate_coordinate_offsets`: Repeatedly loads four 16-bit values from a source list into registers and calls calculate_coordinate_offset to process them into a destination structure pointed to by a1.
- `calculate_coordinate_offset`: Calculates a value based on the difference between two pairs of words, stores several calculated values and pointers into a structure pointed to by a6, and returns.
- `update_bitfield_from_tables`: Updates a global variable by XORing it with a constant, then enters a loop that reads values from four tables to set bits in a bitfield at a memory location pointed to by a2.
- `tiny_and_ori`: Small routine of 2 instructions that masks d0 with d0, then merges 0 into d0.
- `initialize_audio_and_sample_data`: Sets audio hardware registers to specific period and volume values, configures a DMA-driven structure with sample pointers and metadata, and calls another routine four times to process individual sample data entries.
- `initialize_structure_fields`: Clears multiple fields in a structure pointed to by a0, sets a byte at offset 29 to 63, and populates several other fields using values from a table indexed by a calculated offset.
- `update_dma_and_return`: Modifies a global byte and a counter, calls four routines with different pointer offsets, writes a word to the DMA controller register, and returns via an exception return.
- `update_entity_animation_and_state`: Manages entity state by updating pointers and counters within a structure, handles animation frame progression by reading from a table, and performs an indirect jump to a routine based on a calculated offset.
- `branch_to_offset`: Performs a short branch to another location.
- `branch_to_offset_7006`: Performs a short branch to a location 7006 bytes ahead in the code.
- `update_entity_state_and_timers`: Updates a structure containing multiple counters and state variables by decrementing timers, adjusting position or velocity values based on table lookups, and performing conditional arithmetic on various offsets within the structure.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `frag_bra_move_tst_bne_bset_bclr`: Short code fragment that jumps to another point in the code, then copies d1 to the memory at an offset from a6, then jumps to another point in the code, then tests d1, then branches if the last result was not zero, then sets bit 1 of a fixed memory address, then jumps to another point in the code, then clears bit 1 of a fixed memory address, then jumps to another point in the code.
- `negate_and_store_value`: Negates a 16-bit value in d1, stores the result at offset 30 of a0, and otherwise stores the original byte from d1 at offset 29 of a0.
- `frag_move_bra`: Short code fragment that copies d1 to the memory at an offset from a0, then jumps to another point in the code, then copies d1 to the memory at an offset from a0, then jumps to another point in the code, then copies d1 to the memory at an offset from a0, then jumps to another point in the code.
- `update_entity_state_and_counters`: Iterates through a table to find a valid entry, updates a pointer and a counter in a structure, and performs several arithmetic operations on various counters and values within the same structure, including bitwise inversion and clamping values to a maximum of twelve or fifteen.

### 1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:Crystal#unpacked

- `initialize_hardware_and_data`: Sets several hardware registers and global pointers, clears multiple memory locations, and calls another routine and three instances of a routine that copies data from one location to another.
- `process_input_and_initialize_hardware`: Reads a byte from a buffer at a1, performs a loop to copy bytes from a table to a destination structure based on a calculated offset, and then updates various fields in a structure in a6 before calling several other routines.
- `initialize_and_process_input_buffer`: Clears a memory block of 544 words, configures custom hardware registers, and processes a buffer of input data by applying a multiplier and bitmask to each byte before storing the results.
- `wait_for_word_then_clear_2`: Waits until a word in a structure becomes nonzero, giving up when a large countdown runs out.
- `poll_routine_until_result_2`: Repeatedly calls a routine and tests its result and a flag, calling two other routines on each pass until the flag is set.
- `frag_lea_subq_bpl_moveq_rts`: Short code fragment that loads the address of a location in the code into a2, then subtracts 1 from the memory at a2, then branches if the last result was not negative, then loads 0 into d1, then returns.
- `store_routine_results_in_block_2`: Repeatedly calls another routine and stores its results in consecutive longwords, up to 128 entries, stopping early when a result is zero or if a specific flag in d6 is set.
- `divide_by_eleven_and_reset_2`: Sets a global counter to five and returns the input word divided by eleven.
- `frag_bsr_btst_beq_bra_rts`: Short code fragment that calls another routine, then tests bit 4 of the memory at an offset from a6, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `frag_bsr_st_move`: Short code fragment that calls another routine, then sets the memory at a6 to all ones, then copies d1 to the memory at a6.
- `frag_lea_bset_rts`: Short code fragment that loads the address of a fixed memory address into a6, then sets bit 2 of the memory at a6, then returns.
- `clear_indexed_bit_and_wait_2`: Calls another routine to get a bit index and clears that bit in a byte at the pointer in a6, then calls two other routines.
- `frag_bclr_bra`: Short code fragment that clears bit 1 of the memory at a6, then jumps to another point in the code.
- `set_memory_bits_and_loop`: Sets bit 1 of the memory at a6, clears bit 0 of the memory at a6, then sets bit 0 of the memory at a6.
- `synchronize_and_validate_data_structure`: Waits for a word in a structure to become non-zero, clears specific words in that structure, and validates a magic constant.
- `frag_lea_bsr_movea_tst_rts`: Short code fragment that loads the address of the memory at an offset from a4 into a6, then calls another routine, then copies a6 to a4, then tests d1, then returns.
- `process_indexed_table_data_2`: Checks a constant at the start of a buffer, copies 128 words from that buffer to another location, and then performs a series of calculations and data movements involving multiple calls to other routines and loops.
- `frag_lsr_beq_rts_bsr_roxr`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then calls another routine, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (subq.w) and then falls through into the code that follows, with no return of its own.
- `shift_and_loop_data`: Performs a logical shift right on d5 by the amount in d4, and enters a loop that uses dbra to repeat a sequence of operations involving bit shifts and calls to another routine.
- `compare_and_copy_data`: Compares a0 with a4; if they are not equal, it sets a0 to a1 and moves a longword from the address in a0 to d5 before returning.
- `process_coordinate_data_batches_2`: Iteratively processes batches of coordinate data by loading values from tables into a structure and calling another routine.
- `initialize_structure_parameters_2`: Sets a 32-bit value at offset 64 of a structure in a6, copies a pointer to a6 offset 84, clears a 16-bit word at offset 102, and writes 276 and 33248 to offsets 88 and 150 respectively.
- `batch_calculate_coordinate_offsets_2`: Repeatedly loads four 16-bit values from a source list into registers and calls calculate_coordinate_offset to process them into a destination structure pointed to by a1.
- `calculate_coordinate_offset_2`: Calculates a value based on the difference between two pairs of words, stores several calculated values and pointers into a structure pointed to by a6, and returns.
- `update_bitfield_from_tables_2`: Updates a global variable by XORing it with a constant, then enters a loop that reads values from four tables to set bits in a bitfield at a memory location pointed to by a2.
- `initialize_audio_and_sample_data_2`: Sets audio hardware registers to specific period and volume values, configures a DMA-driven structure with sample pointers and metadata, and calls another routine four times to process individual sample data entries.
- `initialize_structure_fields_2`: Clears multiple fields in a structure pointed to by a0, sets a byte at offset 29 to 63, and populates several other fields using values from a table indexed by a calculated offset.

### 1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:1000cc#unpacked (raw code file, load address unknown)

- `initialize_game_engine_state`: Performs several memory copy operations to initialize data blocks, clears a large memory area, configures hardware registers for DMA, interrupts, and video synchronization, and populates a series of global variables with a sequence of values and table pointers.
- `process_joystick_input`: Decrements a byte in a global variable and, if it reaches zero, checks bits in the custom chip and CIAA registers before entering a loop that calls another routine to copy data blocks.
- `copy_nested_data_blocks`: Initializes several pointers and counters, then executes a nested loop structure that copies bytes from a source table to a destination table, incrementing the destination pointer by specific offsets after each inner loop completion.
- `copy_nested_data_blocks_2`: Performs a triple-nested loop to copy bytes from a source memory area to a destination memory area, where the destination address is derived from a pointer in a structure.
- `frag_bsr_cmp_bne_rts`: Short code fragment that calls another routine, then compares d0 with 80, then branches if the last result was not zero, then calls another routine, then compares d0 with 80, then branches if the last result was not zero, then calls another routine, then returns.
- `frag_lea_move_clr_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 5251 to d0, then clears the memory at a0 (advancing a0), then clears the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `frag_cmpi_bne_lea_bra`: Short code fragment that compares a fixed memory address with 2, then branches if the last result was not zero, then loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_cmpi_bne_move_rts`: Short code fragment that compares a fixed memory address with 2, then branches if the last result was not zero, then copies a fixed memory address to a fixed memory address, then returns.
- `configure_audio_channels`: Checks two global variables, and if they match specific values, it sets a global variable and then performs a series of operations to load audio parameters from a structure in a0 into hardware registers.
- `wait_for_register_change`: Enters a loop that waits for the value at a specific hardware register to change.
- `frag_move_jsr_clr_rts`: Short code fragment that copies 16383 to a fixed memory address (2 times in a row), then calls another routine, then copies 5874 to a fixed memory address, then copies 32784 to a fixed memory address, then clears a fixed memory address, then returns.
- `reset_audio_and_interrupts`: Calls another routine, sets the volume of both audio channels to maximum, and enables all interrupts by writing to the interrupt enable and interrupt request registers.
- `update_raster_pointers`: Checks if a global variable is non-zero and decrements it; if it reaches zero, it sets the variable to 8, reads several longwords from a table at a0 and a0 plus an offset, and writes them to various memory locations.
- `copy_structured_tables`: Calls another routine four times with different source and destination address pairs to copy data from tables.
- `copy_structured_table_data`: Performs a nested loop where it calculates a table address by multiplying a value from a0 by 35 and adding a constant, then copies a byte and four specific offsets from that table to a destination buffer at a1 with fixed increments.
- `update_state_and_validate_pointers`: Decrements a 16-bit global variable and performs conditional branches based on its value to update three global pointers and a byte, otherwise it calls another routine five times and performs several comparisons using values from a table and global variables.
- `validate_range_and_enable_dma`: Calculates a value by subtracting one global variable from another, then checks if this value falls within a specific range and compares it against other parameters.
- `validate_and_enable_dma`: Compares a global variable against 5, and if it matches, performs several comparisons involving a structure in a0 and other global variables before branching.
- `update_state_table_eight`: Saves a set of values from a global table to a temporary buffer, calls another routine, restores the values from the buffer to the table, and updates several pointers and counters.
- `update_state_table_seven`: Copies a block of data from a global table to a buffer, calls another routine, restores the data to the table, and updates several global pointers and constants.
- `update_state_table_six`: Moves data from a global table into a buffer, calls another routine, restores the data to the table, and updates various global pointers and a 16-bit constant.
- `update_state_table_five`: Stores a block of data from a global table into a buffer, calls another routine, restores the data to the table, and updates several global pointers and constants.
- `update_state_table_four`: Copies data from a global table to a buffer, calls another routine, restores the data to the table, and updates several global pointers and a 16-bit constant.
- `manage_game_state_and_tables`: Updates a state variable through a series of transitions, modifies a pointer to a table by adding or subtracting a constant, and populates a structure with values from a table.
- `update_global_counters_2`: Checks a global variable against 4 or 180, and if it matches, updates several global variables and performs a loop to search for a value in a table before updating a structure in a0.
- `process_table_and_update_offsets`: Checks a global variable against a constant; if it matches, it performs several subtractions and updates various global variables.
- `update_address_table_entries`: Compares a global variable to 1 or 2 to select a base address and sets other global variables accordingly.
- `update_global_counters`: Checks a byte at a global address and, if it is not one, calculates and stores several new long values into various global locations by adding offsets to a base value.
- `copy_memory_blocks`: Performs a triple-nested loop to copy bytes from a source table to a destination table using specific increments for the pointers.
- `map_keyboard_input_to_mode`: Reads a character from a routine and maps numeric characters one through nine to corresponding values in a global variable, or maps the character 'M' to a specific mode.
- `scatter_data_to_table`: Enters a loop that copies a longword and a word from specific offsets of a source structure in a0 to specific offsets of a destination table in a1, while also copying a longword and a word from the current pointer in a0 to the start of the destination table, then increments the pointers and repeats the process sixteen times before returning.
- `manage_audio_state_and_buffers`: Checks a global state variable to either copy 8 words from a table into a destination buffer and increment a pointer, or reset multiple global variables and clear a series of memory locations before calling another routine.
- `read_keyboard_input`: Checks an interrupt flag, enables a specific bit in a control register, and reads a byte from a serial data register, then uses that byte to index into a table and return a value.
- `update_joystick_and_counters`: Decrements or increments several global counters and updates a structure at offset 73ea by adding or subtracting 1020 or 1388 from its first two longwords.
- `decrement_and_check_counters`: Checks three global variables for zero, and if they are, updates a global variable and calls another routine, otherwise it decrements various global variables and returns.
- `update_nested_counters`: Calls another routine, then checks three values in a structure in a0, and if they meet specific conditions, it increments or resets various fields within that structure.
- `decrement_structure_fields`: Checks three fields in a structure in a0, and depending on their values, decrements them by 1, 2, or 4 and updates other fields in the structure.
- `decrement_and_update_counters`: Decrements a byte in a global variable and, if it reaches zero, performs a series of decrements and updates on two separate three-element structures located at specific memory addresses.
- `dispatch_bitstream_mode`: Compares a long value in a global variable against two constants and, if it matches neither, returns; otherwise, it sets a byte at offset 5fe6 and calls either another routine or a different routine based on the value.
- `increment_and_process_table`: Increments a long value in a global variable and iterates through a table of pointers, calling another routine if specific bytes are found, then updates two values at offset 26 and 30 of a structure in a6 and potentially resets them.
- `decrement_and_process_table`: Decrements a long value in a global variable and iterates through a table of pointers, calling another routine if specific bytes are found, then updates two values at offset 26 and 30 of a structure in a6 and potentially resets them.
- `update_pointer_list`: Iterates through a list of pointers, decrementing values at various offsets within the pointed-to structures and calling another routine if specific conditions are met, then restores a value to a global variable.
- `process_bitstream_segments`: Sets a byte at offset 5fe6 to 2, then iterates 21 times through a table of pointers, calling another routine if a byte at a calculated offset equals 1, and finally increments a long value in a global variable.
- `decrement_bitstream_segments`: Sets a byte at offset 5fe6 to 1, decrements a long value in a global variable, and then iterates 21 times through a table of pointers, calling another routine if a byte at a calculated offset equals 1.
- `shift_bitstream_segments`: Checks a global byte to determine the direction of bitwise shifts; it then performs three loops that shift bits left or right within specific segments of a data buffer based on a count retrieved from a pointer in a3.
- `update_sprite_attributes_and_load_data`: Decrements a global counter and, if it reaches zero, updates a sprite structure by reading several longwords from a table, performing bitwise arithmetic to calculate new values, and writing them to specific offsets.
- `frag_move_bra_2`: Short code fragment that copies 144 to d0, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies 146 to d0, then jumps to another point in the code.
- `frag_move_bra_4`: Short code fragment that copies 148 to d0, then jumps to another point in the code.
- `process_bitstream_and_update_graphics`: Iterates through a nested table of data to perform bitwise shifts and bit-packing operations on 16-bit words, while updating specific status bits in a structure.
- `decrement_counters_and_clamp_values`: Decrements a global 32-bit variable by two and subtracts 8192 from two 16-bit words at offsets 38 and 42 of a structure pointed to by a0; if the second word reaches 8192, it sets the first word to 61370 and the second to 57344 before decrementing the global variable again and returning, otherwise it returns.
- `frag_addi_cmpi_bne_move_rts`: Short code fragment that adds 2 to a fixed memory address, then adds 8192 to the memory at an offset from a0 (2 times in a row), then compares the memory at an offset from a0 with 0, then branches if the last result was not zero, then copies 4042 to the memory at an offset from a0, then copies 0 to the memory at an offset from a0, then adds 2 to a fixed memory address, then returns.
- `update_object_counters_and_attributes`: Calls several routines, then iterates through a series of global pointers to add bytes from a table to specific memory locations.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `update_game_state_and_objects`: Calculates a value based on a multiplication and division of two 16-bit values from a table, then updates several hardware-related offsets and a global pointer.
- `update_physics_and_load_object`: Updates a global timer and performs bitwise calculations to modify two values in a structure using a base address from a global variable.
- `update_object_parameters_and_state`: Decrements a global counter and returns if it is non-zero; otherwise, it calculates a new value based on several global pointers and offsets, updates hardware registers, and then iterates through a table to load specific data structures into memory based on a state byte.
- `update_game_state_and_load_object`: Decrements a global counter and returns if it is non-zero, otherwise it calculates a value based on several global pointers and offsets to update hardware registers and a global variable.
- `update_blitter_chain`: Iterates through a table of structures to update Blitter hardware registers with address, mode, and size parameters, performing multiple Blitter transfers per entry by incrementing pointers.
