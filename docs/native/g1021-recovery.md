# Game 1021 (1000cc turbo (1990)(simulmondo)[cr csl][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01021). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| 1000cc Turbo (1990)(Simulmondo)[cr CSL][a].adf:Crystal | 2 | 85 | 3 |
| 1000cc Turbo (1990)(Simulmondo)[cr CSL][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1378 | 55 |

## Cases handled

- runtime segment an address: 795 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### 1000cc Turbo (1990)(Simulmondo)[cr CSL][a].adf:Crystal

- `decompress_stream_backward`: Loads three longwords from a packed stream table into registers, computes a checksum by XORing the third with the running value, then repeatedly calls the bit-reader refill and multi-bit read routine to extract bits.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### 1000cc Turbo (1990)(Simulmondo)[cr CSL][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_custom_and_audio`: Saves the data and address registers on the stack, then disables the CIAB and interrupt/DMA custom registers while saving their previous values.
- `copy_coordinate_batches`: Reads bytes from a source buffer and, for each non-null character, copies eight values from a table indexed by the character into consecutive offsets of a destination structure, advancing the pointer.
- `reset_custom_and_init_blitter`: Disables interrupts and DMA, then resets the custom chips by writing to COLOR00 and DMACON and clearing INTENA. It sets the stack pointer to the chip RAM area, clears a block of memory, and calls a routine to initialize the Blitter and copper registers before processing a length-prefixed string, then jumps to a vector in a table.
- `initialize_blitter_and_process_string`: Clears a block of memory pointed to by a2, then initializes the Blitter and copper registers and processes a length-prefixed string by converting its bytes to lowercase, computing an index from the total length, and looking up a handler routine in a table.
- `match_buffer_and_store_rotated_bytes`: Compares a byte from a buffer in a4 against a table in a0 and, if it matches, advances the pointer and repeats for a fixed number of bytes.
- `rotate_and_store_byte`: Increments the pointer in a0 by four, rotates the byte in d0 left by eight bits, stores that byte through the pointer in a2 advancing it, then compares a0 with a2 and branches back to an earlier instruction while they differ.
- `copy_pointer_then_branch`: Copies the pointer in a0 into d0, then branches into the middle of another routine.
- `call_routine_subtract_then_call`: Loads zero into d0 and calls a routine, then restores the registers.
- `wait_for_word_then_clear`: Loops sixty-five thousand times decrementing a countdown while polling a word in a structure, returning early when that word becomes nonzero.
- `poll_routine_until_result`: Calls a routine and, while the zero flag is clear and its result is zero, calls two other routines on each pass before returning.
- `decrement_global_counter`: Decrements a global counter by one and returns.
- `store_routine_results_in_block`: Calls two routines and returns early if either reports success.
- `divide_by_eleven_and_reset`: Sets a global counter to five, then returns the input word divided by eleven using the divu instruction.
- `check_bit_and_call`: Calls a routine, then checks bit two of a byte at offset 3841 in the structure in a6 and, when that bit is set, calls another routine before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_moveq`: Small routine of 1 instruction that loads 135 into d1.
- `set_flag_and_store`: Calls a routine, sets all bits of the byte pointed to by a6, then stores the value in d1 through the same pointer without returning.
- `set_control_bit`: Sets bit two of a control byte in a hardware register and returns.
- `clear_indexed_bit_and_wait`: Calls another routine to get a bit index and clears that bit in a byte register, then calls two other routines.
- `clear_bit_and_branch`: Clears bit one of the byte pointed to by a6, then branches to an earlier instruction without returning.
- `set_flag_bits_and_delay`: Sets one flag bit, clears another and sets bit zero of a control byte, then runs a wait loop of sixty-four passes that polls a hardware byte.
- `adjust_and_scan_table`: Compares a bit-shifted value from d7 with a shifted value from d2, adjusting d2 until they match, then clears a bit in the control byte pointed to by a6.
- `decrement_and_branch`: Calls a routine, decrements the byte in d2 by one, then branches to an earlier instruction without returning.
- `restore_and_clear_flag`: Swaps the halves of d7, pops an address from the stack into a6, writes the word four thousand to the structure in a5, and when the long word in a4 is zero clears it and advances the pointer before continuing.
- `scan_table_and_process_entry`: Initializes a structure by writing a control word and storing a pointer, then scans a table of 16-bit entries, clearing each one until it finds the marker value 17545, and calls helper routines to process the matching entry.
- `call_wait_routine`: Loads the address four past the structure in a4 into a6, calls a routine that times out waiting for a word to become nonzero, copies a6 back into a4, checks whether d1 is nonzero, and returns.
- `shift_and_copy_buffer_bytes`: Verifies that the long word at the start of the buffer in a0 holds the value an address, then copies the following 127 long words to the destination in a1.
- `shift_and_branch_if_zero`: Shifts the long word in d5 right by d4 bits through the carry flag, then returns immediately if the result is zero, otherwise it branches to the routine at 2498.
- `rotate_and_return`: Calls a routine, rotates the long word in d5 right through the carry using d4, and returns.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_rotate_and_loop`: Clears d1, shifts d5 right by d4 bits through the carry flag, then returns immediately if the result is zero.
- `rotate_and_branch`: Calls a routine, rotates the long word in d5 right through the carry using d4, then branches to an earlier instruction without returning.
- `copy_table_bytes_by_index`: Loads a byte from a table indexed by a counter in a5 into d0, then compares it against values in d6 and d7 to select a shift amount and index, and finally copies bytes from one table to another by decrementing pointers until both are exhausted.
- `conditional_stack_pop`: Compares the value in a4 with the value in a0 and returns immediately if they differ, otherwise it loads a0 from the address in a1 and stores the value at the top of the stack into d5 before returning.
- `store_byte_and_loop`: Calls a routine, copies d1 into d3, adds d4 to d2, stores a byte from the table in a3 indexed by d3 onto the stack, then loops decrementing d2 until it reaches zero before checking a3 against a2.
- `tiny_subi`: Small routine of 1 instruction that subtracts 21 from d0.
- `process_coordinate_data_batches`: Initializes a structure in a6 by setting several fields and waiting for a status flag, then loads coordinate offsets from a table into a batch structure and calls the offset calculator, advancing a state machine that cycles through four batch types.
- `initialize_structure_parameters`: Initializes a structure pointed to by a6 by storing a 32-bit value at offset 64, a pointer at offset 84, clearing the word at offset 102, and writing 276 and 33248 to offsets 88 and 150.
- `batch_calculate_coordinate_offsets`: Loads four 16-bit values from a source list pointed to by a2 into registers d0 through d3 and calls calculate_coordinate_offset to process them into the destination structure pointed to by a1, repeating this four times before returning.
- `calculate_coordinate_offset`: Calculates absolute differences between two pairs of words, selects a value based on their comparison, and stores several computed values and pointers into a structure pointed to by a6.
- `update_bitfield_from_tables`: Updates a global variable by XORing it with a constant, then enters a loop that reads values from four tables to set bits in a bitfield at a memory location pointed to by a2.
- `tiny_ori`: Small routine of 1 instruction that merges 172 into d7.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_and_ori`: Small routine of 2 instructions that masks d0 with d0, then merges 0 into d0.
- `initialize_audio_and_sample_data`: Disables interrupts and configures the four audio channels by setting their period, volume and pointer registers, then loads sample data and metadata into a DMA-driven structure and calls another routine four times to process each sample entry.
- `initialize_structure_fields`: Clears multiple fields in a structure pointed to by a0, sets a byte at offset 29 to 63, and stores a pointer at offset 6.
- `update_dma_and_return`: Clears a byte and decrements a counter in a structure, flipping a status bit when the counter goes negative.
- `process_entity_timer_and_signal`: Loads a pointer from the structure in a0 and, using a byte at offset 39 of the structure in a2, indexes a table in a6 to store a pointer and a 16-bit value into the structure.
- `branch_and_return`: Consists of a single unconditional branch to a later address within the same routine and then returns without modifying any data.
- `branch_and_store_byte`: Branches into the middle of another routine, which stores the value in d1 through the pointer in a6, then returns.
- `negate_and_store_counter`: Negates a 16-bit value from a register and stores it at offset 30 of the structure in a0, then decrements a counter at offset 42 of that structure and returns.
- `branch_into_routine`: Jumps into the middle of another routine to continue its execution.
- `store_byte_and_toggle_bit`: Branches into the middle of another routine, then stores the value in d1 through the pointer in a6.
- `negate_and_store_word`: Negates the 16-bit value in d1 and stores it at offset 30 of the structure in a0.
- `advance_entity_timer_and_counters`: Advances a 16-bit timer in a structure at offset 20 (masked to a 63 maximum) and decrements its byte counter at offset 22, or loads the next entry from a table via a pointer at offset 39 when the counter is zero.
