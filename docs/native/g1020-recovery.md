# Game 1020 (1000cc turbo (1990)(simulmondo)[cr csl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01020). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909072 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 59 of 59 named (0 author symbols), 59 of 59 described (8 by rule, 49 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 875 of 875 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1473 of 1473 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 434411 |
| code | 5306 |
| data | 51957 |
| padding | 4214 |
| unused | 413184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| 1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:Crystal | 2 | 85 | 3 |
| 1000cc Turbo (1990)(Simulmondo)[cr CSL].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1378 | 55 |

## Cases handled

- runtime segment an address: 795 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware_and_load_samples` (runtime segment an address): Configures custom chipset registers, clears several hardware-related values, and sets up a function pointer in a global variable. Kind: disk hardware/track check. Pass path: The branch following the call that continues normal execution (likely the one not leading to a crash or error message)..
- `validate_and_process_data_stream` (runtime segment an address): Clears a table of 543 entries, configures hardware registers, and validates a data stream by performing arithmetic on bytes and checking against a table. Kind: checksum and data validation trap. Pass path: u2__l0_0714.
- `set_control_bit` (runtime segment an address): Sets bit 2 of a longword at a fixed address and returns. Kind: checksum or hardware status trap. Pass path: the instruction following the call (the routine sets a bit that is subsequently tested by a `btst` or used to validate state).

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
- `call_routine_subtract_then_call`: Calls another routine and, if its result is nonzero, subtracts a stack argument from that result and calls another routine.
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
- `tiny_subi`: Small routine of 1 instruction that subtracts 21 from d0.
- `process_coordinate_data_batches`: Iteratively processes batches of coordinate data by loading values from tables into a structure and calling another routine.
- `initialize_structure_parameters`: Sets a 32-bit value at offset 64 of a structure in a6, copies a pointer to a6 offset 84, clears a 16-bit word at offset 102, and writes 276 and 33248 to offsets 88 and 150 respectively.
- `batch_calculate_coordinate_offsets`: Repeatedly loads four 16-bit values from a source list into registers and calls calculate_coordinate_offset to process them into a destination structure pointed to by a1.
- `calculate_coordinate_offset`: Calculates a value based on the difference between two pairs of words, stores several calculated values and pointers into a structure pointed to by a6, and returns.
- `update_bitfield_from_tables`: Updates a global variable by XORing it with a constant, then enters a loop that reads values from four tables to set bits in a bitfield at a memory location pointed to by a2.
- `tiny_ori`: Small routine of 1 instruction that merges 172 into d7.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_and_ori`: Small routine of 2 instructions that masks d0 with d0, then merges 0 into d0.
- `initialize_audio_and_sample_data`: Sets audio hardware registers to specific period and volume values, configures a DMA-driven structure with sample pointers and metadata, and calls another routine four times to process individual sample data entries.
- `initialize_structure_fields`: Clears multiple fields in a structure pointed to by a0, sets a byte at offset 29 to 63, and populates several other fields using values from a table indexed by a calculated offset.
- `update_dma_and_return`: Modifies a global byte and a counter, calls four routines with different pointer offsets, writes a word to the DMA controller register, and returns via an exception return.
- `update_entity_animation_and_state`: Manages entity state by updating pointers and counters within a structure, handles animation frame progression by reading from a table, and performs an indirect jump to a routine based on a calculated offset.
- `branch_to_offset`: Performs a short branch to another location.
- `branch_to_offset_7006`: Performs a short branch to a location 7006 bytes ahead in the code.
- `update_entity_state_and_timers`: Updates a structure containing multiple counters and state variables by decrementing timers, adjusting position or velocity values based on table lookups, and performing conditional arithmetic on various offsets within the structure.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `update_ciaa_status_bit`: Stores a byte from d1 into a structure at offset 47 of a6 and sets or clears bit 1 of the CIAA_PRA register based on whether d1 is non-zero.
- `negate_and_store_value`: Negates a 16-bit value in d1, stores the result at offset 30 of a0, and otherwise stores the original byte from d1 at offset 29 of a0.
- `update_entity_state_and_counters`: Iterates through a table to find a valid entry, updates a pointer and a counter in a structure, and performs several arithmetic operations on various counters and values within the same structure, including bitwise inversion and clamping values to a maximum of twelve or fifteen.
