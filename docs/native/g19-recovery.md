# Game 19 (Xenon): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00019). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (928544 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 114 of 114 named (0 author symbols), 114 of 114 described (4 by rule, 12 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1280 of 1280 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3573 of 3573 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10017 |
| code | 12842 |
| data | 18900 |
| padding | 1 |
| unused | 886784 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Xenon.adf:bootblock | 1 | 152 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 553 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2775 | 73 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 245 | 8 |

## Cases handled

- Xenon/Xenon.adf: file Disk.info (header 606 bytes, blocks hold 488): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 328 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 761 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 191 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Xenon/Xenon.adf: blocks 883-883 (bytes 452096-452608) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.34)
- Xenon/Xenon.adf: blocks 1038-1038 (bytes 531456-531968) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `shift_noise_bit_register` (runtime segment an address): Shifts one bit derived from a status value into a 16-bit shift register and a second shared word, then returns four times the computed value. Kind: timing/bit-stream integrity check. Pass path: beq.b an address.

## Functions


### Xenon.adf:bootblock

- `setup_boot_block_exec_calls`: Boot block entry routine of about 50 instructions for which no pseudo-code is available.
- `stub_decompiler_timed_out`: No pseudo-code is available for this 76-instruction boot block routine.
- `stub_decompiler_timed_out_2`: No pseudo-code is available for this five-instruction boot block routine.
- `issue_exec_doio_request`: Calls the exec library I/O request routine through a library offset and writes an interrupt enable register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `pass_through_stack_value`: Calls one unnamed routine and then returns the stack slot it was given unchanged.
- `expand_packed_entry_table`: Waits through a short count-down delay, then rebuilds a table of 289 entries from packed source records.
- `expand_packed_entry_table_2`: Rebuilds a table of 289 entries from packed source records, writing expanded bytes and a running offset for each entry.
- `continue_packed_entry_expansion`: Continues the packed table expansion loop: it writes one expanded entry at a time from the current record, advances the running offset and moves to the next record when the repeat count runs out.
- `expand_packed_entry_table_3`: Entry point of the packed table expansion: it writes a zero entry, then walks source records, writing repeat-expanded entries and running offsets until the end of the 289-entry table.
- `verify_checksum_then_init_state`: XOR-sums 118 longwords from a fixed block and compares the result with a constant.
- `init_state_and_run_sequence`: Initialises several state fields and then runs a sequence of calls to an unnamed routine, with count-down loops and step increments up to a limit.
- `advance_step_and_run_sequence`: Advances a step counter by eight and, when it reaches the boundary, increments a row counter and resets the step positions.
- `call_routine_four_times`: Calls one unnamed routine four times in a row and returns.
- `call_routine_three_times`: Calls one unnamed routine three times in a row and returns.
- `execute_recursive_calls`: Calls another routine four times in succession.
- `fill_table_with_pseudorandom_words`: Waits briefly, then fills a buffer in repeated passes with a generated 16-bit sequence.
- `fill_pseudorandom_table_cia`: Makes a call to an unnamed routine and then performs the same pseudo-random word fill as its neighbour.
- `blend_pseudorandom_mask_into_buffer`: Waits briefly and calls an unnamed routine three times, then repeatedly merges a table of words into a buffer across 40 rows of four words.
- `stub_zero_fill_forever`: Writes zero longwords upward from its own address in an endless loop.
- `blend_pseudorandom_mask_cia`: Makes two unnamed calls and then performs the same pseudo-random masked merge as its neighbour.
- `call_routine_twice`: Calls one unnamed routine twice and returns.
- `mask_buffer_with_word_table`: ANDs each word of a 40-row buffer with the matching word of a source table, four words per row, and applies the same operation across four regions at fixed offsets.
- `clear_eight_words_after_delay`: Waits briefly, then clears eight consecutive words starting at a fixed record offset.
- `clear_eight_words_at_offset`: Clears eight consecutive words starting at a fixed record offset and returns.
- `write_bit_column_to_buffers`: Writes one column of bits into three buffers at a computed offset.
- `build_pointer_table_copy_state`: Fills a four-entry table with addresses stepping by a fixed plane stride and stores a base pointer.
- `toggle_saved_value_flag`: If a counter field is nonzero, copies a saved value back into the working field and clears the flag.
- `setup_vertical_interrupt_vector`: Initialises an interrupt-related record and, when an argument is given, stores pointers into an interrupt vector slot and sets a shared word; without an argument it calls a library function through a vector.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `tiny_ori`: Small routine of 1 instruction that merges 136 into d1.
- `test_flag_and_call_handler`: Tests a flag bit in a shared word and calls a fixed routine plus another unnamed routine when it is clear, or stores the flag word and calls a function pointer from a vector record when it is set.
- `read_joystick_direction_state`: Reads the mouse and joystick port movement registers and the CIA fire-button register, turns the direction bits into a stored state byte with a repeat delay, and writes a copper list pointer register.
- `stub_empty_return`: Body is only a return with no other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_byte_change`: Spins on a comparison of a record byte with itself, so the loop only ends if hardware changes that byte between reads.
- `call_routine_count_times`: Calls one unnamed routine once for each step of a count taken from the low word of its input, counting down to zero, then returns the input value.
- `store_cia_port_reading`: Stores a 16-bit value built from two CIA port registers into a record field and returns the input unchanged.
- `generate_scaled_random_value`: Advances a 16-bit pseudo-random state in a record field with a multiply-and-add step, replacing a zero state with one, then maps the result into a range from two inputs by division.
- `append_byte_until_buffer_full`: When a record flag is set, appends the input byte to a buffer and advances the buffer pointer and a counter.
- `run_main_state_machine_loop`: Main loop over a state byte that reads input, dispatches through a table of handler routines and updates position and timer fields each pass.
- `stub_empty_return_2`: Body is only a return with no other effect, and the routine is called once from one unnamed routine.
- `set_record_flag_bits_return`: Writes a record byte back unchanged, ORs a set of bit flags into another record byte, and returns the input byte with its low six bits set.
- `return_constant_value`: Returns the integer value four.
- `copy_words_into_fixed_area`: Copies 512 words from a buffer into a fixed record area and moves the old contents of that area into a second buffer.
- `toggle_word_buffer_copy_direction`: Flips a parity word in the record, then copies 512 words between a fixed record area and one of two buffers, choosing the copy direction from the parity.
- `call_two_routines_in_sequence`: Calls two unnamed routines in order and returns.
- `setup_blitter_copy_operation`: Sets the blitter's mask, modulo, pointer, control and size registers for a copy-style operation over a block offset from the record, then returns.
- `advance_cell_frames_copy_blocks`: Counts down a timer for each cell in a table and, when it expires, advances the cell's frame index.
- `loop_routine_calls`: Executes another routine within a loop that runs four times.
- `sync_shadow_bytes_and_call`: For each of two byte pairs, copies the source byte into a shadow byte when they differ and calls an unnamed routine after each change.
- `call_routine_sequence_five`: Calls one unnamed routine twice with pairs of constant arguments, then three more times with no arguments, and returns.
- `call_routine_five_times`: Calls one unnamed routine five times in a row and returns.
- `update_clamped_value`: Checks several conditions in a global structure and updates a byte at a specific offset with a value that is clamped to a maximum.
- `sync_nine_bytes_tick_timers`: Copies nine bytes from one record area to a mirror area, calling three unnamed routines for each byte that changes.
- `decompose_integer_by_tens`: Iteratively divides a value by ten to extract its decimal digits.
- `perform_division_loop`: Performs a series of arithmetic operations involving a loop to divide a value from a global structure by powers of ten.
- `convert_char_digit_and_emit`: Subtracts the character code for zero from its input, optionally maps the result through a table, and calls one of two unnamed routines depending on whether the result is the newline or vertical tab code.
- `fetch_dual_register_pair`: Calls another routine twice and concatenates the resulting values from two registers into a single 32-bit return value.
- `call_subroutines_and_return`: Calls another routine twice and returns the combined contents of two registers.
- `emit_text_skipping_carriage_returns`: Walks a zero-terminated text string and passes every character other than the carriage-return code to an unnamed routine.
- `emit_text_skipping_carriage_returns_2`: Same zero-terminated text walk as its neighbour: characters other than carriage return go to an unnamed routine, and the loop ends at the terminator.
- `render_cell_sequence_frames`: Clears a block of record words, then computes plane addresses for each cell from a frame index and copies cell data into them while walking a table of per-cell command records.
- `blit_rows_with_stride`: Programs the blitter with fixed masks and modulo values, then runs 169 blit operations that each copy a block taken from a table, waiting for the blitter between them.
- `blit_four_rows_unless_flagged`: Unless a record flag equals one, programs the blitter with fixed masks and performs four blit operations, each advancing down by one plane stride.
- `blit_four_rows_unless_other`: Same four-row blit as its neighbour, but guarded by a different record byte value.
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
- `scan_neighbour_cells_and_clear`: Scans four neighbouring positions taken from a direction table and tests the grid cell at each one.
- `scan_neighbour_cells_from_input`: Scans four neighbouring grid positions, offset by the input values, and calls unnamed routines for each in-range cell that passes a check.
- `scan_neighbour_cells_stepwise`: Steps through four grid positions from a direction table, calling unnamed routines for in-range cells that pass a check.
- `scan_neighbour_cells_variant`: Steps through grid positions from a direction table in the same way as its neighbours, calling unnamed routines for in-range cells that pass a check.
- `clear_two_record_bytes`: Clears two consecutive bytes in the record and returns.
- `store_three_record_bytes`: Stores three input register values into three record bytes and returns.
- `reset_state_and_copy_positions`: Sets a counter word to six, sets a state flag and a timer byte, then copies two saved position words into their working fields.
- `clear_flag_and_restore_positions`: Clears a flag byte and a timer byte, then copies two saved position words back into their working fields.
- `update_position_list_entries`: Chooses an index value from a record field and calls an unnamed routine with it.
- `blit_sprite_column_when_active`: When a record state byte has one specific value, draws a column of blocks with the blitter using masks, modulo values and a source chosen from record position fields.
- `set_flag_bit_and_call`: Sets the low bit of a record flag byte, calls one unnamed routine and returns.
- `blit_sprite_column_from_input`: Draws a column of blocks with the blitter like its neighbour, using an input pointer for the source and a record flag to choose between two blitter control words.
- `stub_decompiler_timed_out_3`: No pseudo-code is available for this 147-instruction routine.
- `allocate_list_slot_and_fill`: Finds the first free 11-word entry in a list by scanning for a clear flag bit, zeroes it and fills its fields from a table indexed by the input.
- `call_routine_then_mark_slot`: Calls one unnamed routine, then writes the all-ones value into a byte of the record selected by an index byte.
- `allocate_list_slot_alt_base`: Same first-free list slot allocation and fill pattern as its neighbour, but with a different list base and scan limit.
- `allocate_list_slot_third_base`: Same list-slot allocation and fill as its neighbours, but with another list base and a scan limit of 59 entries.
- `clear_record_bytes_block`: Clears 2200 bytes starting at a pointer held in a record field.
- `call_routine_for_inactive_slots`: Walks 60 list slots at a 22-byte stride and, for each slot whose flag bit is clear, calls one unnamed routine.
- `call_routine_for_active_slots`: Walks 60 list slots at a 22-byte stride and, for each slot whose flag bit is set, calls one unnamed routine.
- `call_routine_until_marker`: Calls one unnamed routine repeatedly until it returns the letter X.
- `call_routine_until_bracket`: Calls one unnamed routine repeatedly until it returns an open-bracket character.
- `tick_slot_counter_and_flag`: Clears a slot's flag bit, looks up a step from a table by slot index and decrements the slot's counter.
- `write_slot_table_entries`: If the top bit of a source word is clear, counts the used list entries and writes one table byte per slot from a lookup table, repeating over its length.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `queue_three_blit_rows`: Builds a blitter request at a record base, setting control and mask words from a flag, then waits for the blitter to be free before writing three row requests that each advance by a fixed plane stride.
- `blit_clipped_sprite_row`: Sets a shared flag bit, computes a row and column from the inputs, and when the row falls inside a 15-row window programs the blitter with masks, control words and modulo values to draw a clipped row of blocks at a computed address.
- `blit_clipped_row_with_call`: Calls one unnamed routine to obtain a value, then draws a clipped row of blocks with the blitter in the same way as its neighbour.
- `blit_clipped_sprite_row_unflagged`: Like its neighbours, draws a clipped row of blocks with the blitter at a computed address, but clears the shared flag bit first and writes the source pointer directly.
- `blit_clipped_sprite_block_rows`: Calls one unnamed routine, then draws a clipped block of rows with the blitter, issuing three plane-strided blit requests per row like its neighbours.
- `register_table_slot_and_blit`: Looks up the input value in a table of 31 longword slots, storing it in the first empty slot if it is absent.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_shared_word`: Clears one shared word and returns.
- `stub_decompiler_timed_out_4`: No pseudo-code is available for this 170-instruction routine.
- `step_audio_channel_sequences`: Reads command bytes from a per-channel sequence stream, where some ranges set stream or table pointers or dispatch through a table and others update the channel's period and volume from a lookup table.
- `step_audio_channel_from_input`: Same command-stream sequencer as its neighbour, but the stream and starting values come from an input pointer.
- `run_audio_channel_update_loop`: Runs the per-channel sequencer loop without the initial setup.
- `step_audio_channel_setup_b`: Same sequencer as its neighbours, starting from an input pointer and clearing a flag byte before the first command is read.
- `shift_noise_bit_register`: Shifts one bit derived from a status value into a 16-bit shift register and a second shared word, then returns four times the computed value.
- `stub_bad_instruction_data`: Decoded as bad instruction data that the decompiler truncated, so only a counter decrement and a DMA control write are visible.
