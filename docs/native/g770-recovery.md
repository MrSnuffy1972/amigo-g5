# Game 770 (xenon (1988)(melbourne house)[cr a-ha][h british bulldogs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00770). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (928544 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 114 of 114 named (0 author symbols), 114 of 114 described (4 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1280 of 1280 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3573 of 3573 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10017 |
| code | 12924 |
| data | 8066 |
| padding | 1 |
| unused | 897536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Xenon (1988)(Melbourne House)[cr A-Ha][h British Bulldogs].adf:bootblock | 1 | 168 | 3 |
| logged disk load from Xenon (1988)(Melbourne House)[cr A-Ha][h British Bulldogs].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 553 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2775 | 73 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 245 | 8 |

## Cases handled

- xenon (1988)(melbourne house)[cr a-ha][h british bulldogs]/Xenon (1988)(Melbourne House)[cr A-Ha][h British Bulldogs].adf: file Disk.info (header 606 bytes, blocks hold 488): its blocks are judged as raw blocks outside the filesystem
- xenon (1988)(melbourne house)[cr a-ha][h british bulldogs]/Xenon (1988)(Melbourne House)[cr A-Ha][h British Bulldogs].adf: the boot block reads 32768 bytes from disk offset 799744 to an address (found statically); recovered as a unit
- runtime segment an address: 328 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 761 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 191 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- xenon (1988)(melbourne house)[cr a-ha][h british bulldogs]/Xenon (1988)(Melbourne House)[cr A-Ha][h British Bulldogs].adf: 1 logged loads cover 0 bytes of the image
- xenon (1988)(melbourne house)[cr a-ha][h british bulldogs]/Xenon (1988)(Melbourne House)[cr A-Ha][h British Bulldogs].adf: blocks 883-883 (bytes 452096-452608) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.34)
- xenon (1988)(melbourne house)[cr a-ha][h british bulldogs]/Xenon (1988)(Melbourne House)[cr A-Ha][h British Bulldogs].adf: blocks 1038-1038 (bytes 531456-531968) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `step_shift_register_bits` (runtime segment an address): Advances two shift registers by one bit each, feeding in a bit computed from a masked global value, and returns a value scaled by four. Kind: disk hardware timing/bit-stream check. Pass path: the branch at an address (beq.b an address).

## Functions


### Xenon (1988)(Melbourne House)[cr A-Ha][h British Bulldogs].adf:bootblock

- `setup_boot_block_io_dma`: Boot block entry that calls the exec I/O request routine and three library offsets, and accesses the DMA and interrupt enable registers.
- `run_single_boot_step`: Boot block routine of 76 instructions with one caller and one unnamed call.
- `setup_copper_and_dma`: Calls a library function at a fixed offset, then touches the COLOR00 register, the copper list pointer and jump registers, and the DMA control register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_single_routine_and_return`: Calls one other routine and returns the value it passes back on the stack.
- `wait_then_select_status_buffer`: Waits through a short countdown, then sums two status bytes from a structure and uses the result to choose between fixed buffer addresses before further processing.
- `select_status_buffer_and_copy`: Selects one of two buffers from a status byte sum and copies words between a source table and a destination, using counters and pointer updates.
- `copy_bytes_with_counters`: Copies bytes from a source to a destination in a loop, with a mode flag changing how each source byte is read, and advances the pointers and counters on each step.
- `clear_bytes_until_limit`: Clears a run of bytes while advancing a counter, stopping once the counter passes a fixed limit, and advances through a table as it goes.
- `initialise_display_state_fields`: Waits through a short countdown and then sets a flag byte to one, clears several word fields in a structure and writes fixed starting values into others.
- `clear_memory_without_exit`: Writes zero long words in an endless loop, advancing through memory from a code address.
- `initialise_display_state_and_loop`: Sets a flag byte to one, clears word fields in a structure and writes fixed starting values, then loops with a counter while calling two routines.
- `advance_scroll_step`: Advances a position field in steps of eight and, after two steps, wraps it and increments a counter, setting two fields to a fixed value.
- `call_four_routines_in_order`: Calls four other routines and returns.
- `call_three_load_routines`: Calls three other routines and returns.
- `call_four_routines_in_order_2`: Calls four other routines and returns.
- `scan_table_using_cia_registers`: Waits briefly, calls a routine, then reads two CIA chip registers in a loop, selecting one of two pointer tables by a flag bit and walking its entries.
- `scan_table_from_cia_ports`: Scans a table in a loop, reading the port A registers of both CIA chips and choosing one of two table pointers by a flag bit.
- `scan_table_using_cia_registers_2`: Waits briefly, calls routines, then reads CIA registers and walks a table of long-word entries in a loop until a condition is met.
- `clear_memory_without_exit_2`: Writes zero long words in an endless loop, advancing through memory from a code address with no exit.
- `scan_table_from_cia_ports_2`: Reads two CIA port values and scans a table in a loop, choosing one of two table pointers by a flag bit and updating entries from the results.
- `call_routine_twice`: Calls one other routine two times in succession and then returns, with no other work in the decompiled body.
- `mask_bitplane_block_rows`: Ands each word of a mask table into four interleaved bitplane copies of the target block, row by row, clearing every bit outside the mask.
- `wait_and_clear_eight_words`: Counts down a short delay loop and then writes zero into eight consecutive 16-bit words starting at a base address held in the address register.
- `clear_eight_words`: Writes zero into eight consecutive 16-bit words starting at a base address, then passes its parameter back unchanged in the return value.
- `write_column_bits_to_planes`: Sets or clears one bit in each of two bitplane buffers for a column, using the next bit of a shifted pattern on every step through four planes.
- `setup_plane_table_and_copy`: Fills four table entries with offsets that step by one plane stride, stores the destination pointer, then copies a fifty-seven-longword block from a fixed source and nine words after it.
- `toggle_stored_value_state`: Toggles a saved state: when the flag is set it restores one stored value and clears the flag, otherwise it stores a second value, sets the flag and sets a companion byte.
- `init_vertical_interrupt_struct`: Fills a structure that points to itself and to a shared data slot, sets a status byte, then either records a previous handler pointer or calls a library function through a library base vector.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `tiny_ori`: Small routine of 1 instruction that merges 136 into d1.
- `dispatch_interrupt_flag_handler`: Checks a flag bit held in a global value and, when it is clear, calls two routines; when it is set it stores the flag value and calls a handler through a stored pointer.
- `read_joystick_input_state`: Reads the joystick movement register, the fire-button port and the mouse movement register, then tracks direction changes with a repeat counter.
- `return_empty`: The routine performs no operations and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_state_byte_change`: Re-reads a state byte in a loop that has an empty body, so it spins until that byte changes under interrupt or hardware control, then returns the input register value unchanged.
- `call_routine_count_times`: Calls another routine once for each step of a 16-bit countdown started from the argument, looping until the counter wraps past zero, and returns the argument.
- `read_cia_port_pair`: Reads two CIA port registers for the fire buttons and disk status, combines their low bytes into one 16-bit word, and stores that word in a state field.
- `generate_scaled_pseudorandom_value`: Steps a 16-bit state word with a multiply-and-add, then scales the result into a range given by the caller using a division.
- `append_byte_until_buffer_full`: While a flag is set, stores one byte at the buffer pointer, advances the pointer and counts the stored bytes.
- `update_input_state_per_frame`: Runs once per frame to read the button and direction state, advance an idle timer and dispatch through a jump table chosen by the low state nibble, then clamp and store a horizontal and a vertical position.
- `return_empty_2`: This routine is a stub that returns without any effect.
- `set_status_bits_and_return`: Writes a byte back unchanged, sets bits 2, 4 and 5 of a status byte at a fixed offset, and returns the argument with its low six bits set.
- `return_constant`: The routine returns a constant value of four.
- `exchange_word_block_buffers`: Clears a state word and exchanges 512 words between a working buffer and a saved buffer, copying the saved values out and loading the working values in.
- `toggle_and_exchange_buffers`: Flips a state word between zero and one, then exchanges 512 words between two buffers, with the roles of the buffers chosen by the new state value.
- `call_two_routines_in_turn`: Calls two other routines one after the other and then returns, with no other work visible in the decompiled body.
- `run_four_blitter_passes`: Programs the blitter with full masks, a fixed modulo and a fixed width, then starts four blits at successive plane offsets, waiting for the blitter to be free before each start.
- `advance_tile_animation_frames`: Walks a table of animation records and, for each record whose countdown has expired, copies a block of words into four bitplane buffers and advances its frame index.
- `call_routine_five_times`: Calls another routine five times in a row and returns.
- `sync_shadow_state_bytes`: Copies two state bytes into their shadow copies whenever they differ, calling a routine for each copy, and returns.
- `call_routines_with_fixed_pointers`: Calls one routine twice with two fixed address arguments, then calls three other routines in turn and returns.
- `call_routine_six_times`: Calls another routine six times in a row and returns.
- `adjust_clamped_counter_byte`: When two status bytes are clear and a third condition holds, adds the argument to a counter byte and clamps it to the range zero to twenty-four.
- `sync_bytes_and_decay_timers`: Compares nine state bytes against their shadow copies and calls three routines for each one that changed, then counts two 16-bit timers down and clears their flags when they reach zero.
- `emit_decimal_digits`: This routine reads a 32-bit value from a state field and emits it as decimal digits by repeated subtraction of descending powers of ten, calling another routine.
- `emit_stored_decimal_digits`: Reads a 32-bit value from a state field and emits it as decimal digits by repeated subtraction of descending powers of ten, calling a routine for each digit and one final routine at the end.
- `dispatch_digit_character`: Converts a character by subtracting the ASCII zero code, optionally maps it through a lookup table, then calls one of two routines depending on whether the result is ten or eleven.
- `call_two_routines_in_order`: Calls two other routines in sequence and returns, with no other work visible.
- `call_two_routines_sequentially`: Calls two other routines one after the other and returns, with no other work visible.
- `emit_text_without_returns`: Walks a zero-terminated text string and calls a routine for each character except carriage returns.
- `emit_text_skipping_carriage_returns`: Walks a zero-terminated text string and calls a routine for every character other than carriage returns, stopping at the terminator.
- `update_cell_grid_and_objects`: Clears a table of eight records, then fills a twelve-by-twelve grid of cells from a source buffer, scaled by 128.
- `run_blit_per_table_entry`: Programs the blitter with full masks and a fixed modulo, then runs one small blit for each entry of a table, adjusting each target offset by a fixed amount and waiting for the blitter to be free before each start.
- `run_four_blits_unless_flagged`: Unless a state byte equals one, programs the blitter with full masks and runs four blits at plane-stride offsets, waiting for the blitter to be free before each one.
- `run_four_blits_unless_state`: Unless a state byte equals fourteen, programs the blitter with full masks and runs four blits at plane-stride offsets, waiting for the blitter to be free before each one.
- `call_routine_and_return`: Calls one other routine and then returns, with no other work visible.
- `trap_if_flagged_then_run`: Issues a trap instruction when a global value is nonzero, then calls three other routines in turn.
- `copy_four_plane_blocks`: Programs the blitter with full masks and a fixed modulo, then runs four block copies whose source and destination offsets step by one plane stride, waiting for the blitter before each start.
- `call_routine_once`: Calls one other routine and returns, with no other work visible.
- `execute_double_calls`: The routine calls another routine twice and then returns.
- `blit_rows_in_place`: Calls one other routine, then runs a row loop of 192 rows that points the blitter source and destination at the same row, advancing 40 bytes per row and waiting for the blitter before each row.
- `fill_longwords_with_ones`: Writes all-ones to a block of 2240 longwords that starts at a fixed offset from the address register.
- `call_two_routines_again`: Calls two other routines in turn and returns, with no other work visible.
- `blit_fifty_six_rows`: Runs a row loop of 56 rows that points the blitter source and destination at the same row, advancing 40 bytes per row and waiting for the blitter before each row.
- `return_unchanged_value`: Has no visible effect in the decompiled body and returns its input values unchanged.
- `scan_neighbours_and_clear_flags`: Steps through four neighbouring positions around a record, checks each in-range position with two test routines, and calls a handler when both pass.
- `resume_neighbour_scan`: Resumes the neighbour scan partway through its loop, using the caller's offsets, and clears two status bytes of the record when the scan finishes.
- `continue_neighbour_scan`: Enters the same neighbour scan loop with the caller's offset and mask values, and clears two status bytes of the record when the scan finishes.
- `scan_neighbours_with_offsets`: Runs the neighbour scan loop with offsets taken from the caller, calling test routines for each in-range position, and clears two status bytes of the record when finished.
- `clear_record_status_bytes`: Sets two status bytes of a record to zero, at fixed offsets from the record pointer.
- `store_record_status_bytes`: Writes three byte arguments into a record: one at offset twenty and two status bytes at offsets twelve and thirteen.
- `init_state_and_copy_positions`: Sets a counter word to six, a state byte to one and another state byte to twenty-four, then copies two saved position words into their current position fields.
- `clear_state_and_copy_positions`: Clears two state bytes and copies two saved position words into the current position fields.
- `update_object_screen_positions`: Picks a speed value from the state, calls a routine with it, then either writes four screen position words from a lookup table or loops over object records, clamping each vertical position to the range zero to 198 and storing it with a horizontal offset.
- `blit_object_across_planes`: Computes a screen position from the object's coordinates, programs the blitter for an object copy, and runs it across four plane offsets, switching between two blit configurations depending on the position.
- `set_flag_and_call_routine`: Sets bit zero of a status byte and calls one other routine.
- `blit_object_planes_on_change`: Computes a screen position and programs the blitter for an object image, skipping the destination reset when a stored value is unchanged, then runs four blits at plane-stride offsets.
- `jump_to_fixed_address`: The decompiler timed out, so only the listed facts are known: a 147-instruction routine that makes four calls, reached through a single jump entry that passes control on to a fixed address.
- `spawn_object_into_mid_slots`: Finds a free object slot in a middle range of a table, clears its ten words, and fills in its position and state bytes from a lookup table.
- `call_and_mark_table_slot`: Calls one other routine, then writes the value 255 into a table entry selected by a state byte.
- `spawn_object_into_low_slots`: Finds a free object slot in the lower range of a table, clears its ten words, and fills in its values from a lookup table.
- `spawn_object_into_high_slots`: Finds a free object slot in the upper range of a table, clears its ten words, and fills in its values from a lookup table.
- `clear_object_table_bytes`: Zeroes 2200 consecutive bytes starting at a base pointer held in the address register.
- `call_routine_for_inactive_slots`: Walks 60 object records at a fixed stride and calls one routine for each record whose second flag bit is clear.
- `call_routine_for_active_slots`: Walks 60 object records at a fixed stride and calls one routine for each record whose second flag bit is set.
- `poll_routine_until_match`: Calls one routine repeatedly until it returns the character X.
- `poll_routine_until_bracket`: Calls one routine repeatedly until it returns the character left square bracket.
- `tick_timer_and_add_value`: Clears a flag on an indexed record and counts down a per-slot timer.
- `append_record_to_list`: If the record's top bit is clear, it counts the filled entries of a list, appends a new entry from a lookup table, and calls routines for each stored value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `queue_object_plane_blits`: Calls one routine and clears bit zero of a status byte, then fills four blitter command blocks for an object at plane offsets that step by one plane stride, with the mode chosen by a flag.
- `draw_object_when_visible`: When an object's row falls within the visible window, it programs the blitter with the object image, takes row offsets from a table and computes the destination address for the plane.
- `draw_object_four_planes_visible`: When an object is inside the visible window, it sets up a blit from a row offset table and runs four blits at plane-stride offsets, with a shift chosen from the object's horizontal position.
- `draw_object_row_in_window`: When an object's row lies in the window, it programs the blitter for a single row draw using offsets from a row table.
- `blit_object_planes_in_window`: Computes a screen position from the object's coordinates and, when it is visible, programs the blitter for an object image and runs four plane-stride blits, with a shift chosen from the horizontal position.
- `register_pointer_and_blit`: Adds a pointer to a table of up to 31 entries unless it is already present, then programs the blitter for a fill with full masks and runs three blits with a fixed control value, waiting for the blitter before each.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_and_return`: Sets one global word to zero and returns.
- `set_dma_channel_control`: The decompiler timed out, so only the listed facts are known: a 170-instruction routine that writes the DMA control register and calls one other routine, and it is called from four places.
- `step_audio_channel_sequences`: Walks a set of audio channel sequences by reading command bytes that set pitch, volume and jump targets, writing period and volume values to the audio channel registers and setting the matching DMA enable bits.
- `start_audio_channel_record`: Loads a channel record from an input stream, then runs the same command-driven channel loop that writes audio register values and sets DMA enable bits for each channel.
- `step_audio_channel_loop`: Runs the command-driven audio channel loop for each channel, writing audio register values and DMA enable bits, and calls one routine for each flagged channel at the end.
- `start_audio_channel_from_stream`: Marks a channel record as active, copies its initial values from an input stream, then runs the audio channel loop that writes audio register values and DMA enable bits.
- `step_shift_register_bits`: Advances two shift registers by one bit each, feeding in a bit computed from a masked global value, and returns a value scaled by four.
- `stub_bad_instruction_data`: The decoder reached bytes that are not valid instructions after a short counter update that clears two fields, so no clean behaviour can be recovered from this routine.
