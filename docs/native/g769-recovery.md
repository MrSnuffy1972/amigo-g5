# Game 769 (xenon (1988)(bitmap brothers)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00769). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (928544 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 113 of 113 named (0 author symbols), 113 of 113 described (4 by rule, 109 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1280 of 1280 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3705 of 3705 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10033 |
| code | 12752 |
| data | 8774 |
| padding | 473 |
| unused | 896512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Xenon (1988)(Bitmap Brothers)[a2].adf:bootblock | 1 | 132 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 553 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2775 | 73 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 245 | 8 |

## Cases handled

- xenon (1988)(bitmap brothers)[a2]/Xenon (1988)(Bitmap Brothers)[a2].adf: file Disk.info (header 606 bytes, blocks hold 488): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 328 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 761 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 191 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- xenon (1988)(bitmap brothers)[a2]/Xenon (1988)(Bitmap Brothers)[a2].adf: blocks 883-883 (bytes 452096-452608) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.34)
- xenon (1988)(bitmap brothers)[a2]/Xenon (1988)(Bitmap Brothers)[a2].adf: blocks 1038-1038 (bytes 531456-531968) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `shift_noise_bit_register` (runtime segment an address): Shifts a bit derived from a status value into a 16-bit shift register and a second shared word, then returns four times the computed value. Kind: bit-stream timing/noise check. Pass path: beq.b an address.

## Functions


### Xenon (1988)(Bitmap Brothers)[a2].adf:bootblock

- `load_and_execute_hunks`: Parses a custom object format by calling a routine to allocate and initialize memory, then configures hardware interrupts and DMA before jumping to the loaded code.
- `parse_hunk_data`: Parses a custom object format by reading hunk type codes and performing operations such as allocating memory, zeroing blocks, copying data, or applying relocations to a destination buffer.
- `load_table_pointer`: Shifts a value in d5 left by two, uses it as an offset into a table pointed to by a3 to load a new address into a2, and moves that address into d1 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_and_restore_registers`: Calls a routine at a fixed address, saves the registers to the stack, and then returns.
- `expand_compressed_data_table`: Checks a condition based on the sum of two bytes from a structure in a5; if the condition is met, it iterates through a table to decompress data into a destination buffer using a custom bitstream decoding algorithm.
- `process_data_table`: Pushes registers to the stack, reads two bytes from a structure at offset 1341 and 1353 relative to a5, and if bit 0 of their sum is zero, performs a series of table lookups and data copies involving a1, a4, and a0, including a loop that processes bytes based on bit flags.
- `copy_structured_data_block`: If a byte in d7 is one, it copies a byte from a0 to d2 and then copies five bytes from a source table to a1, where the fourth byte is d2 and the fifth byte has bit 1 set if bit 6 of d0 is set; otherwise it performs a different sequence of five byte copies from a source table to a1.
- `increment_and_compare_d1`: Writes zero to the byte at the address in a1 and increments the address, then increments d1 and d3, and returns if d1 is less than or equal to 192.
- `initialize_sequence_data`: Pushes registers to the stack, loads a longword from a table at a0, performs an XOR operation between two values from that table, and if the result matches a specific constant, clears a longword at a specific address and branches to another routine; otherwise it performs a loop using dbra.
- `initialize_sequence_parameters`: Sets several values at offsets 1228, 972, 974, 978, 1343, 984, and 982 of a structure in a5, calls another routine with d1 set to 79, and then compares a value at offset 984 of the structure to 160.
- `advance_step_and_run_sequence`: Advances a step counter by eight and, when it reaches the boundary, increments a row counter and resets the step positions.
- `call_unnamed_routine`: Calls a routine at a fixed address.
- `call_routine_three_times`: Calls three different routines in a row and returns.
- `execute_recursive_calls`: Calls four different routines in succession and returns.
- `fill_pseudorandom_table_cia_2`: Performs a pseudo-random word fill into a table by XORing values derived from a loop and returns a concatenated result.
- `fill_pseudorandom_table_cia`: Makes a call to an unnamed routine and then performs the same pseudo-random word fill as its neighbour.
- `process_masked_data_loop`: Pushes registers to the stack, loads a longword from a table, performs a loop six times to XOR and mask data from a table into a destination pointed to by a structure in a5, and then compares the result to a constant before branching.
- `clear_memory_and_branch`: Sets a pointer in a5 to a specific address, clears a longword at that address, and branches to another routine.
- `blend_pseudorandom_mask_cia`: Makes two unnamed calls and then performs the same pseudo-random masked merge as its neighbour.
- `call_routine_twice`: Calls one unnamed routine twice and returns.
- `mask_buffer_with_word_table`: ANDs each word of a 40-row buffer with the matching word of a source table, four words per row, and applies the same operation across four regions at fixed offsets.
- `checksum_data_table`: Reads two consecutive longwords from a table, performs an XOR operation between them, and compares the result against a constant value to determine the next branch.
- `clear_eight_words_at_offset`: Clears eight consecutive words starting at a fixed record offset and returns.
- `write_bit_column_to_buffers`: Writes one column of bits into three buffers at a computed offset.
- `update_structure_pointers_2`: Checks a byte at offset 132 of a structure; if it is non-zero, it copies a longword from offset 128 to offset 438 and clears offset 132 and then performs three loops to copy data from a source to a destination and another loop to copy nine words.
- `update_structure_pointers`: Checks a byte at offset 132 of a structure; if it is non-zero, it copies a longword from offset 128 to offset 438 and clears offset 132 and offset 434, otherwise it copies from offset 124 to 438 and sets offset 132 and 434 to one.
- `setup_handler_structure`: Writes several values to a structure starting at offset 136 of a5, and if a flag in d5 is set, it updates a global function pointer and returns; otherwise it calls a routine via a library vector.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `tiny_ori`: Small routine of 1 instruction that merges 136 into d1.
- `test_flag_and_call_handler`: Tests a flag bit in a shared word and calls a fixed routine plus another unnamed routine when it is clear, or stores the flag word and calls a function pointer from a vector record when it is set.
- `read_joystick_direction_state`: Pushes registers to the stack, checks global flags, reads joystick and mouse data from custom hardware registers, updates a structure in a1 with direction bits and repeat delays, and calls a routine via a pointer at offset 916 of a1.
- `set_bits_based_on_input`: Tests various bits in d0 and sets corresponding bits in d1 based on those tests before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_byte_change`: Spins on a comparison of a byte at an offset of 1326 from a pointer in a6 with itself, so the loop only ends if hardware changes that byte between reads.
- `call_routine_count_times`: Calls one unnamed routine once for each step of a count taken from the low word of its input, counting down to zero, then returns the input value.
- `store_cia_port_reading`: Stores a 16-bit value built from two CIA port registers into a record field at offset 1072 of the structure in a5 and returns the input unchanged.
- `calculate_scaled_value`: Performs arithmetic on a 16-bit word at offset 1072 of a structure in a5 to update it, then uses the result to perform a series of divisions and additions involving other values from the structure and registers before returning.
- `update_entity_buffer`: Checks a byte at offset 1359 of a structure in a5; if non-zero, it copies a byte from a pointer stored at offset 588 of the structure to the location pointed to by a0, increments that pointer, increments a global counter, and if the counter reaches 618, writes a byte of 255 to the memory location pointed to by a0 and calls another routine.
- `process_entity_list_and_update_state`: Iterates through a list of entities to update their positions and properties, manages a 16-bit counter in a structure, and conditionally calls several routines to handle entity-specific logic and text processing.
- `do_nothing`: Performs several comparisons and conditional branches based on the sum of d1 and d0 and a byte at offset 1252 of the structure in a5, but otherwise it returns without modifying d1.
- `set_record_flag_bits_return`: Writes a byte from a pointer in a4 to that same location, ORs the value 52 into a byte at offset 64 of the structure in a0, and returns the input byte with its low six bits set.
- `return_constant_four`: Loads the value 4 into d0 and a value from an offset of 2 from a1 into d2, then returns.
- `initialize_buffer_pointers`: Loads three addresses from offsets 420, 460, and 424 of a structure into a0, a1, and a2, sets d0 to 511, clears a word at offset 430 of the structure, and branches to another routine.
- `toggle_word_buffer_copy_direction`: Flips a parity word in a record at offset 1070 of the structure in a5, then copies 512 words between a fixed record area and one of two buffers, choosing the copy direction from the parity.
- `call_two_routines_in_sequence`: Calls two unnamed routines in order and returns.
- `setup_blitter_copy_operation`: Sets the blitter's mask, modulo, pointer, control and size registers for a copy-style operation over a block offset from the record, then returns.
- `advance_cell_frames_copy_blocks`: Decrements a timer for each entry in a table and, if it reaches zero, increments a frame index and copies an 8-word image block from a frame table into three buffers at fixed plane offsets.
- `loop_routine_calls`: Executes another routine within a loop that runs four times.
- `sync_shadow_bytes_and_call`: For each of two byte pairs, copies the source byte into a shadow byte when they differ and calls an unnamed routine after each change.
- `call_routine_sequence_five`: Calls one unnamed routine twice with pairs of constant arguments, then three more times with no arguments, and returns.
- `call_routine_five_times`: Calls another routine five times in a row and returns.
- `update_clamped_value`: Checks several conditions in a global structure and updates a byte at a specific offset with a value that is clamped to a maximum.
- `sync_nine_bytes_tick_timers`: Copies nine bytes from one record area to a mirror area, calling three unnamed routines for each byte that changes.
- `adjust_pointer_and_branch`: Saves registers d0 through d7 and a6 onto the stack, subtracts the value in a6 from itself, sets d3 to 1, and branches to a relative offset.
- `perform_division_loop`: Performs a series of arithmetic operations involving a loop to divide a value from a global structure by powers of ten.
- `calculate_blit_offset`: Subtracts 48 from d2, loads a pointer from a table in a5, and if d3 is not 1, checks if d2 is 10 or 11; if not, it multiplies d2 by 48, adds the result to the pointer, and calls another routine; otherwise, it loads a byte from a table at a1 using d2 as an index, adds 2048 to the pointer, and branches to another routine.
- `decrement_and_branch`: Saves registers to the stack, decrements the value in a2, and branches to another routine.
- `calculate_and_call_blit_routines`: Calculates an offset using a value from d1 and d0, then calls another routine twice using two different addresses derived from a table in a5.
- `prepare_branch_with_registers`: Saves registers d0 through d5 onto the stack, sets d5 to 1, and branches to a relative offset.
- `emit_text_skipping_carriage_returns`: Same zero-terminated text walk as its neighbour: characters other than carriage return go to an unnamed routine, and the loop ends at the terminator.
- `update_entity_list_and_process_text`: Clears a block of memory, calculates an offset based on a structure value and a multiplier, and populates a linked list of entities.
- `blit_rows_with_stride`: Programs the blitter with fixed masks and modulo values, then runs 168 blit operations that each copy a block taken from a table, waiting for the blitter between them.
- `branch_on_status_byte`: Compares a byte at offset 1202 of a structure to one and branches accordingly.
- `blit_rows_with_stride_2`: Programs the blitter with fixed masks and modulo values, then runs 168 blit operations that each copy a block taken from a table, waiting for the blitter after each write.
- `call_routine_once`: Calls one unnamed routine and returns.
- `trap_if_flag_call_three`: If a shared word is nonzero, raises a trap instruction.
- `blit_copy_four_rows_pairs`: Programs the blitter for a copy and runs four operations, each taking a pair of source and destination pointers that advance by one plane stride per row.
- `call_subroutine_and_return`: Calls another routine and returns.
- `execute_double_calls`: Calls another routine twice and returns.
- `blit_clear_rows_with_stride`: Calls one unnamed routine, then programs the blitter to write a fixed pattern across 192 rows, advancing 40 bytes per row and waiting for the blitter after each write.
- `fill_longs_with_all_ones`: Fills 2240 longwords starting at a fixed record offset with the all-ones value and returns.
- `nop_padding_sequence`: This routine consists of a sequence of no-operation instructions that perform no functional task.
- `blit_clear_fifty_six_rows`: Programs the blitter to clear 56 rows at a fixed record offset, advancing 40 bytes per row and waiting for the blitter after each write.
- `return_registers`: Returns the combined contents of two registers.
- `process_record_data`: Calculates a pointer using a value from a1, loads two words from a table at that pointer, and if the first word is within a specific range, calls two different routines and then calls a third routine with d2, d3, and d7 set to 16, 16, and 2 respectively.
- `branch_on_register_comparison`: Restores registers from the stack, compares a6 to 1, and if not equal, decrements a6, loads a byte from a structure into d5, and branches to another routine.
- `update_counters_and_clear_bytes`: Updates d5 based on d3 and d4, increments d4, and if d4 reaches 4 and a4 is 1, it clears two bytes at offsets 12 and 13 of the structure in a0 and returns.
- `decrement_and_compare_counters`: Decrements d4 and compares it to 252; if not equal, it compares a4 to 1 and if not equal, it decrements a4 and loops back.
- `store_three_record_bytes_2`: Clears two bytes at offsets 12 and 13 of the structure in a0 and branches to another routine.
- `store_three_record_bytes`: Stores three input register values into three record bytes and returns.
- `reset_state_and_copy_positions`: Sets a counter word to six, sets a state flag and a timer byte, then copies two saved position words into their working fields.
- `clear_flag_and_restore_positions`: Clears a flag byte and a timer byte, then copies two saved position words back into their working fields.
- `update_position_list_entries`: Chooses an index value from a record field and calls an unnamed routine with it.
- `update_blt_sprite_parameters`: Checks specific status bytes in a structure, modifies a scale or size value, calculates a memory address for a bitplane, and updates the Amiga Blitter registers and DMA controller to perform a drawing operation.
- `set_flag_bit_and_call`: Sets the low bit of a record flag byte, calls one unnamed routine and returns.
- `blit_sprite_column_from_input`: Draws a column of blocks with the blitter like its neighbour, using an input pointer for the source and a record flag to choose between two blitter control words.
- `execute_blit_operations`: Calls four different routines to perform blitting, updating parameters, clearing bits, and jumping to a fixed address.
- `initialize_entity_list`: Saves registers to the stack, loads constants into d2 and d4, and branches to another location.
- `call_routine_then_mark_slot`: Calls one unnamed routine, then writes the all-ones value into a byte of the record selected by an index byte.
- `initialize_entity_list_zero`: Saves registers to the stack, loads zero into d2 and 29 into d4, and branches to another location.
- `search_and_initialize_entity`: Iterates through a linked list of structures by following pointers, incrementing a counter for each valid entry found.
- `initialize_entity_structure`: Clears a block of memory pointed to by a pointer stored at offset 544 of a structure in a5, using a loop that runs 2199 times.
- `call_routine_for_inactive_slots`: Walks 60 list slots at a 22-byte stride and, for each slot whose flag bit is clear, calls one unnamed routine.
- `call_routine_for_active_slots`: Walks 60 list slots at a 22-byte stride and, for each slot whose flag bit is set, calls one unnamed routine.
- `call_routine_until_marker`: Calls one unnamed routine repeatedly until it returns the letter X.
- `call_routine_until_bracket`: Calls one unnamed routine repeatedly until it returns an open-bracket character.
- `tick_slot_counter_and_flag`: Clears a slot's flag bit, looks up a step from a table by slot index and decrements the slot's counter.
- `copy_entity_records`: Checks if the high bit of a word at a0 is set; if not, it counts non-zero words in a table, copies a byte from a pointer to a record in a2, and performs a loop to copy data and call another routine; if the high bit is set, it calls another routine and writes a byte of all ones to a record in a2.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_blit_parameters_and_branch`: Pushes registers to the stack, calls a routine to calculate blit parameters, and if the result is not zero, exchanges the value in a2 with the result in d0, sets a pointer in a1 to a4, clears bit 0 of a byte at offset 1338 of a5, and branches to another routine.
- `set_blit_parameter_and_branch`: Pushes registers to the stack, sets a1 to a0, sets a3 to the address of the custom hardware registers, sets a0 to a2, sets bit 0 of a byte at offset 1338 of a5, and branches to another routine.
- `call_blit_and_set_bit`: Pushes registers to the stack, sets a1 to a0, calls a routine, sets a0 to a2, sets bit 0 of a byte at offset 1338 of a5, and branches to another routine.
- `clear_bit_and_branch`: Pushes registers to the stack, sets a1 to a0, sets a3 to the address of the custom hardware registers, clears bit 0 of a byte at offset 1338 of a5, and branches to another routine.
- `calculate_and_update_blit_parameters`: Calculates sprite or tile dimensions and source/destination coordinates based on input values and a table, then updates a structure in a3 with multiple sets of address and size parameters.
- `register_table_slot_and_blit`: Looks up the input value in a table of 31 longword slots, storing it in the first empty slot if it is absent.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_audio_3_volume`: Clears the volume of audio channel 3 by writing zero to its hardware register and then returns.
- `process_audio_and_dma`: Performs complex calculations involving multiplication and bit manipulation to update audio registers and DMA settings through a loop.
- `update_pointer_and_value`: Loads a word from a table, calculates a new address using a second word from the table, checks a value at that address, and if zero, updates a word in the table and branches to another routine.
- `clear_and_set_bytes`: Clears a word at offset 32 of a structure, copies two bytes from a pointer into offsets 20 and 21 of that structure, sets bit 0 of a byte at offset 0, and branches to another routine.
- `branch_to_middle`: Branches to an offset within another routine.
- `update_record_bytes`: Stores a byte from a0 at offset 1 of a structure, copies two bytes from a1 to offsets 34 and 36 of a0, clears offset 35 of a0, and branches to another routine.
- `shift_noise_bit_register`: Shifts a bit derived from a status value into a 16-bit shift register and a second shared word, then returns four times the computed value.
- `update_structure_via_pointer`: Decrements a byte in a structure, and if non-zero, performs a series of moves to update a pointer and several words in a target structure.
