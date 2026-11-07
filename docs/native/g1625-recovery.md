# Game 1625 (james pond 2 - codename robocod (1991)(millennium)[cr skr][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01625). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (950976 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 97 raw-word lines |
| Functions named and described | pass, 219 of 219 named (0 author symbols), 219 of 219 described (4 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1770 of 1770 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7111 of 8695 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28848 |
| code | 31886 |
| data | 733570 |
| unused | 156672 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf:bootblock | 1 | 16 | 1 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf boot-block load | 1 | 1438 | 5 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5486 | 163 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1361 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 394 | 22 |

## Cases handled

- james pond 2 - codename robocod (1991)(millennium)[cr skr][a2]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf: custom-format (non-DOS) disk, 1452 non-empty blocks: matched against track-loaded segments
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a2]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a2]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1348 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 152 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 270 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a2]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf: 2 logged loads cover 12288 bytes of the image
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a2]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf: 12 track-loaded blocks found in recovered runtime segments and classified through them
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a2]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf: blocks 4-1215 (bytes 2048-622592) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.65)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a2]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf: blocks 1219-1238 (bytes 624128-634368) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.54)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a2]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf: blocks 1242-1261 (bytes 635904-646144) outside the filesystem hold data, not code (opcode density 0.020, entropy 5.71)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a2]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf: blocks 1265-1284 (bytes 647680-657920) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.50)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a2]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf: blocks 1288-1431 (bytes 659456-733184) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.24)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf:bootblock

- `load_two_blocks_via_doio`: Fills in an I/O request block twice and runs exec's DoIO for each.

### logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a2].adf boot-block load

- `unpack_backward_compressed_data`: Decodes compressed data written from a high end address toward a low start address.
- `test_upper_bits_no_effect`: Tests whether a register has any bits above its lowest bit, then returns in both cases.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `read_backward_bitstream_bits`: Repeats a count held in a data register, and each time shifts the lowest bit out of a bit buffer.
- `stub_partial_decompilation`: The routine is 1360 instructions long, but the pseudo-code shown is only a small check on a global flag that calls another routine and returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `program_blitter_with_busy_wait`: Adjusts two global offsets by a sign-dependent step and a shift of a value, then waits until a busy flag clears.
- `blit_indexed_glyph_at_position`: Subtracts 32 from the input register to get a table index, then computes the address of that entry in a table of 60-byte records.
- `blit_indexed_glyph_variant`: The decompiled body is identical to the previous entry, so this description relies on the register list.
- `parse_text_control_codes`: Walks a null-terminated string one byte at a time.
- `parse_text_control_codes_2`: Iterates through a null-terminated string and modifies a global variable based on specific control characters.
- `walk_text_string_fragment`: A five-instruction fragment of the string-walking loop.
- `set_position_from_operand_byte`: A five-instruction fragment of the string-walking loop that starts by loading one operand byte as the new position value, then continues the loop for further characters.
- `add_operand_byte_to_position`: A five-instruction fragment of the string-walking loop that adds the following operand byte to the current position, then continues the loop.
- `add_operand_byte_and_walk`: A five-instruction fragment of the string-walking loop that adds the following operand byte to the current position, entered from a different negative code than the previous fragment, and then continues the loop.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `compute_blitter_shift_counts`: Scans six words in turn for the first word with bit 0 set and separately the first with bit 1 set, each defaulting to minus one when none is found.
- `blit_rows_with_pointer_step`: Waits for the blitter to be idle, then for each unit of the input count loads a source and a destination pointer into the blitter and advances both pointers by the same step.
- `set_blitter_full_copy_control`: Writes two all-ones values to the blitter's first and last word masks, zeros the modulo and shift registers, and writes a control value that selects the blitter's channels.
- `start_blit_to_destination_pointer`: Waits for the blitter to be idle, sets full masks and a control value, then writes a destination pointer taken from an address register and a size taken from the data register, starting the blit.
- `start_blit_from_two_pointers`: Waits for the blitter to be idle, sets full masks and a control value, then writes a source pointer, a destination pointer and a size from its inputs, starting the blit.
- `blit_four_bitplane_rows`: If the low nine bits of the input are nonzero, it starts four blits into four bitplanes spaced by a fixed stride, using one of two modulo settings chosen by bit 9.
- `stub_partial_decompilation_2`: The pseudo-code shows only three assignments, but the routine is 149 instructions long and touches blitter, colour and DMA registers.
- `wrap_single_routine_call`: Calls one unnamed routine and then performs a self-assignment with no effect, so it acts only as a thin wrapper around that call.
- `build_four_entry_pointer_table`: Writes a table of four entries by splitting a 32-bit base value into high and low words for each one, stepping the value by a global increment between entries.
- `select_masks_build_pointer_table`: Chooses among a few preset mask values by testing globals, waits while a global is zero, then fills a four-entry pointer table the same way as the neighbouring routine and sets a colour register.
- `update_wrapped_position_record`: Subtracts offsets from a position record and derives a small step from the low nibble of each speed field, limited to a magnitude of 32.
- `run_counted_loops_by_flags`: Calls a setup routine, then runs one of several counted loops depending on which of four globals is nonzero.
- `lookup_doubled_table_value`: Takes the upper bits of a global value, scales them by two to form a byte offset, and reads a word from a fixed table at that offset.
- `compute_aligned_record_offset`: Multiplies a 16-aligned value by 38 and adds it to a field of an input record, plus a fixed bias of 7904.
- `start_row_blits_offset_up`: If a global flag is set, it adds 608 to a base position, then stores the base and calls a routine 19 times.
- `start_row_blits_offset_down`: Like its neighbour, but when the global flag is set it subtracts 608 from the base position.
- `store_double_and_repeat_calls`: Stores twice a global value into a record field, then calls a routine fifteen times in a counted loop.
- `store_offset_and_repeat_calls`: Stores twice a global value plus 36 into a record field, then calls a routine fifteen times in a counted loop.
- `start_blits_from_row_index`: Splits a value into a coarse index and a remainder.
- `initialise_fixed_state_values`: Writes a fixed set of constant values into several absolute memory locations and globals, including zeros and the address of a shared global.
- `build_copper_list_pointer_table`: Builds a four-entry pointer table as its neighbours do, splitting a base value into high and low words and stepping it by a global increment.
- `toggle_between_two_pointer_sets`: Switches a state global between two sets of values.
- `execute_jump_table_entry`: Performs an indirect call to a function address retrieved from a table using an index stored in a global variable.
- `dispatch_jump_table_call_variant`: Makes the same indirect call as its neighbour through the same unrecovered jump table, indexed by a global value.
- `start_conditional_blit_for_column`: When a global flag is set, it derives a column offset and a row size from its inputs, writes blitter control and mask values, and starts a blit.
- `start_fixed_blit_when_enabled`: When a global flag is set, it writes an adjusted pointer from a value and a shift, waits for the blitter, sets masks and a control value, and starts one blit to a fixed destination pointer.
- `dispatch_second_jump_table_call`: Calls two routines and then dispatches through a second jump table, indexed by a global value scaled by four, which could not be recovered.
- `dispatch_second_table_variant`: Has the same code as the entry before it: two calls, then an indirect dispatch through the same unrecovered jump table indexed by a global value.
- `copy_tile_rows_interleaved`: Copies data for sixteen groups of sixteen rows from a source table into a destination buffer.
- `write_color_list_entry_pairs`: Writes fixed 32-bit value pairs at offsets in a list indexed by a level number, with extra entries for lower levels.
- `clear_matching_table_records`: Walks a table of 26-byte records while counting down a global value.
- `process_records_by_threshold_checks`: Walks a table of 26-byte records while counting down a global value.
- `setup_masked_sprite_blits`: Reads an object record's fields to compute screen coordinates, and a first and last word mask from the low four bits of a horizontal position.
- `find_first_zero_slot_entry`: Scans up to 32 entries of 10 bytes each, starting from a pointer held in a structure.
- `clear_active_slot_entries`: Sets full blitter masks and a control value, then walks up to 32 ten-byte entries from a structure pointer.
- `blit_four_planes_with_wrap`: Waits for the blitter, copies a value into two fields of a record, and computes a start address and a row split from a count compared against 224.
- `call_four_times_then_clear`: Sets a global value to a fixed constant, calls one routine four times, then clears three globals to zero.
- `clear_field_in_record_table`: Clears a 16-bit field at offset 14 in each of 32 consecutive 26-byte records, starting from a fixed address, and then zeroes a global.
- `invert_underscore_marker_byte`: When a global byte equals the underscore character, it sets the byte to zero and then inverts it, leaving all bits set.
- `copy_sixteen_longwords_from_table`: Copies sixteen consecutive longwords from a fixed source address into a global buffer, four at a time, in a counted loop of four passes.
- `stub_infinite_loop_halt`: Contains only an unconditional loop with no exit, so calling it never returns.
- `poll_routine_until_value`: Calls a routine repeatedly until it returns 252, unless a global flag is already set.
- `setup_four_audio_channels`: Points each of four audio channels at a shared buffer and sets each channel's length to one word, then writes an enable value to the DMA control register for those channels.
- `read_swapped_nine_bit_position`: Returns the low nine bits of a global value after swapping its two bytes.
- `install_interrupt_vectors`: Disables all interrupts and clears pending requests, fills ten low-memory vector entries with a fixed value, and points four interrupt vector entries at a routine.
- `call_with_bit_six_set`: Sets bit six of a global, calls a routine, clears bit six again, and then sets the global to 8.
- `handle_interrupt_flag_bits`: Checks a flag bit of a global and, when it is set, calls two routines and may set the global to 255 if a returned value exceeds 251.
- `shift_keyboard_serial_bit`: When bit three of a global is set, it shifts one serial bit through the byte in inverted form, calling routines on the bit edges, and then sets the global to 8.
- `run_flagged_interrupt_callbacks`: Calls one routine when bit four of a global is set.
- `dispatch_pending_request_bits`: Checks bits seven to ten of a global request value, clearing each set bit and calling a routine for it.
- `keep_bits_eleven_to_thirteen`: Keeps only bits eleven to thirteen of a global value, clearing all other bits.
- `stub_empty_return`: Contains only a return instruction, so calling it does nothing.
- `read_joystick_direction_state`: Derives direction flags from the joystick port movement bits, combining the vertical direction as an exclusive-or of two bit positions.
- `match_key_sequence_entry`: Compares an input character with the next entry of a sequence table and advances an index when it matches, returning early while more entries remain.
- `apply_letter_specific_state_changes`: When a global is nonzero, applies a change chosen by the input letter.
- `busy_delay_store_upper_bits`: Runs a loop of 10000 iterations, each time storing the counter masked to bits eight to eleven, so the final stored value is the last masked count.
- `busy_delay_store_middle_bits`: Runs a loop of 10000 iterations, each time storing the counter masked to bits four to eleven, so the final stored value is the last masked count.
- `busy_delay_store_split_bits`: Runs a loop of 10000 iterations, each time storing the counter masked to bits zero to three and eight to eleven.
- `busy_delay_store_low_bits`: Runs a loop of 10000 iterations, each time storing the counter masked to bits four to seven.
- `advance_feedback_shift_register`: Shifts a global value left one bit and sets its lowest bit when bit 23 and bit one differ, so the value steps through a feedback shift sequence.
- `step_multiplicative_congruential_value`: Increments a global value while it is below five, returning it to one when it reaches five.
- `format_number_as_digit_string`: Converts a number to decimal digits by repeatedly subtracting successive values from a table until a zero entry, writing each count as a digit character into a fixed buffer.
- `stub_bad_instruction_data`: Recorded as a routine, but the decompiled code is only a bad-instruction marker.
- `set_playfield_scroll_and_dma`: Stores its input multiplied by 448, disables all DMA channels, and sets the playfield control registers to zero or fixed values.
- `build_six_entry_pointer_table`: Stores a base value in a structure and derives a step from the input nibbles, adjusted by fixed offsets.
- `move_and_clear_table_blocks`: Copies 32 words from one fixed table to another and clears the source, then moves 256 words from a global buffer into two other tables while zeroing the source.
- `clear_fixed_word_block`: Zeroes 544 consecutive words from a fixed address and then sets a global to 30.
- `fill_word_tables_from_routine`: Fills 32 words at a fixed address with values returned by a routine, then fills two 256-entry tables from routine results, writing each entry into both.
- `compute_grey_level_color`: Adds the three nibbles of an input value to a signed offset and clamps the total to 47.
- `fill_paired_routine_tables`: Decrements a counter global and fills table entries from routine results.
- `stub_partial_decompilation_3`: The pseudo-code shows only a return, although the routine is recorded as 31 instructions.
- `call_routine_for_nonzero_bytes`: Walks four bytes starting at an offset from a global base.
- `scan_table_for_nonzero_entry`: Scans up to four table entries spaced 148 bytes apart, looking for a nonzero first longword.
- `advance_four_counters_with_wrap`: When the flag global is nonzero, increments four 16-bit counters.
- `decode_five_bit_flags`: When the global is zero, reads a byte from a stream pointer.
- `initialise_stream_marker_state`: Writes the byte value 255 to the address in an address register, points the stream pointer one byte past it, zeroes the stream counter and clears a global.
- `load_cia_timer_a_value`: Writes the two bytes of its input value into the first CIA's timer A low and high registers, and sets a control global from a masked value.
- `load_cia_b_timer_value`: Writes the two bytes of its input value into the second CIA's timer A low and high registers, and sets its control register and interrupt control register.
- `call_routine_over_word_sequence`: Loops over a count held in a structure, calling a routine with each value and incrementing a global counter.
- `draw_object_plane_blits`: Waits for the blitter, sets its masks and size, and derives two screen words from record fields and the current value.
- `refresh_changed_display_digits`: Refreshes a display's digit characters when its value changes, calling a routine for each character that differs from the stored copy.
- `copy_interleaved_byte_quads`: Copies eleven groups of four bytes from a source to a destination, writing the first two bytes at offsets zero and two and the other two at offsets 436 and 438.
- `wait_blitter_then_start_copy`: Waits for the blitter to be idle, then writes two pointer-derived values and a size of 1857 to the blitter registers.
- `stub_partial_decompilation_4`: The pseudo-code shows only clearing a global and waiting on it, but the routine is 158 instructions long and calls 18 routines.
- `advance_counter_and_call_routines`: Returns early when a global is nonzero.
- `initialize_data_structures`: Clears a block of memory by writing zeros to a table and then performs a series of sequential writes to various global variables.
- `set_all_ones_sentinels`: Fills four global bytes and six fixed longwords with all-ones values, typically a sentinel for empty entries.
- `program_blitter_after_calls`: Calls three routines, waits for the blitter to be idle, then writes full masks, a control value of 496 and two more values to its registers.
- `round_position_up_sixteen`: Calls two routines, then sets a group of globals depending on whether a global is zero or equal to another.
- `clear_twenty_one_globals`: Sets twenty-one global variables to zero in sequence.
- `set_three_conditional_triplets`: Zeroes four globals, then in three checks, when a global is nonzero, sets it to 255 and loads two fixed small values.
- `run_state_machine_step`: Runs one step of a state machine on a global state value, calling routines depending on the state and returning a 16-bit value taken from the state.
- `clamp_fixed_point_velocity`: Nudges a fixed-point velocity global by a small increment, subtracts a constant step under a sign test, and clamps the result to a floor value.
- `update_movement_state_offsets`: Adjusts a movement state byte, then updates a position offset by a constant that depends on the state.
- `step_position_by_state_offset`: Advances a state byte by four, then subtracts a state-dependent fixed offset from a position value.
- `step_position_offset_up`: Advances a state byte by eight and adds a state-dependent fixed offset to a position value.
- `update_state_with_speed_limit`: Clears a global when a state byte is zero.
- `advance_position_with_clamp_fifteen`: Adds a fixed offset of 40960 to a value, takes its high word and clamps it to 15.
- `call_mode_routines_then_flag`: Calls routines depending on whether bits eleven and twelve of a global match a pattern, calls two more routines, and sets a global to 96 when a flag check passes.
- `step_toward_target_within_eight`: Returns the low bits of a value masked with five.
- `update_mode_state_with_clamp`: Runs a state-driven update that clamps a value to fifteen in each direction, looks up mode-based values from the upper bits, and sets several fixed values depending on the state.
- `update_offset_clamped_three`: Adds a fixed offset of 5462 to a value, clamps its high word to plus three on the positive side and minus fifteen on the negative side, doubles it, and calls routines before and after.
- `update_offset_clamped_two`: Adds a fixed offset of 3280 to a value, clamps its high word to plus two on the positive side and minus fifteen on the negative side, doubles it, and calls routines before and after.
- `call_six_routines_clear_flag`: Calls six routines in sequence with small global assignments between them, and clears a global when a value is nonnegative and a flag is set.
- `step_toward_target_limited`: Moves a global toward a target value.
- `call_then_store_register_words`: Calls one routine and then writes two input register words into the same global variable in turn, ending with the second.
- `store_two_register_words_variant`: Matches the previous entry: it calls one routine and then writes two input register words into the same global variable in turn.
- `subtract_and_correct_sign`: Subtracts a register value from a global.
- `add_with_correction_two`: Adds a register value to a global.
- `subtract_clamp_state_four`: Subtracts a register value from a global.
- `add_clamp_state_eight`: Adds a register value to a global.
- `damp_value_toward_zero`: Applies a damping step to a global velocity.
- `damp_value_toward_zero_variant`: Matches the damping step of its neighbour.
- `damp_value_fifteen_step`: Moves a global toward zero by up to fifteen units, choosing the direction from a bit test on the result.
- `add_clamped_offset_fifteen`: Clamps a 16-bit input to the range minus fifteen to fifteen, then adds it to a global.
- `check_threshold_return_pair`: When a global is zero and a flag is clear, it compares a value against a threshold of 12288 on either sign, calls a routine when the test passes, and returns the global together with a 16-bit result word.
- `move_by_capped_steps`: When the state global equals one, it moves a position global in steps of 8192 toward or away from a zero point.
- `test_small_window_no_effect`: Tests a global against a small window and returns in both branches, so the routine has no effect.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 1 into d6, then returns.
- `scan_table_for_matching_entry`: Scans up to 32 entries of a table for one whose first word matches a derived key, stopping early at a sentinel of minus one.
- `increment_indexed_counter_entry`: Clears a global, sets another to 255, calls a routine, then increments the byte at a fixed table address indexed by the global.
- `update_position_toward_target`: Updates a position and velocity pair.
- `update_position_toward_target_narrow`: Moves a value toward a target when the difference is within 209 units, limiting the step by a bound of 200.
- `advance_table_chain_twice`: Twice, while the global is nonzero, adds an entry from a byte table indexed by the global to the global and then decrements it, which follows a chain through the table.
- `scan_entries_and_return_value`: Sets up a few globals.
- `set_state_constants_when_negative`: When a state global is zero and negative, it sets a few state globals to fixed values, mostly by stage number, and returns.
- `set_state_constants_variant`: Matches the previous entry: when a state global is zero and negative, it sets state globals to fixed values.
- `reset_state_to_defaults`: Sets state globals to fixed defaults such as 172 and 255 and decrements a counter.
- `set_state_seven_defaults`: When a state global is not seven, sets it to seven and loads a fixed offset of 224 plus a constant into a position global.
- `set_position_constant_by_flag`: Sets a position global to one of two fixed values depending on whether a flag is zero, then clears a global.
- `set_negative_constant_then_clear`: Writes a fixed large negative value to a global, then clears it to zero.
- `update_state_variable`: Modifies a global variable based on its current value, performing conditional increments or resetting it to a specific constant.
- `run_state_transition_table`: A large state-transition routine that chooses among several state codes, sets a global from a state, calls routines, and then steps through a table of entries until a negative value.
- `resolve_relative_table_entry`: Steps a table index back by one and reads a 16-bit relative entry from a table.
- `lookup_tile_flags_at_position`: Indexes a fixed table with the high bits of a position to find a tile word.
- `lookup_tile_word_with_links`: Finds a tile word from a position using a fixed index table, then follows one or two linked neighbouring words while their top flag bits are set, and stores the result.
- `step_toward_target_masked`: Masks a global to its low 15 bits, then moves it toward a target value.
- `move_by_half_step_limit`: Returns when the input is above 21.
- `add_register_to_global`: Adds an input register value to a global variable and returns.
- `subtract_half_limited_step`: Subtracts a register value from a global.
- `subtract_register_from_global`: Subtracts an input register value from a global variable and returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d6, then returns.
- `stub_partial_decompilation_5`: The pseudo-code shows only a return, although the routine is recorded as 17 instructions.
- `stub_partial_decompilation_b`: The pseudo-code shows only a return, although the routine is recorded as 24 instructions.
- `stub_partial_decompilation_c`: The pseudo-code shows only a return, although the routine is recorded as 11 instructions.
- `return_inverted_masked_input`: Returns the bitwise inverse of an input register, masked to bits two and three.
- `reflect_global_when_flagged`: Calls a routine, then when the input has any of bits 15, 14, 12, 11 or 10 set and a second routine returns nonzero, sets a global to minus three minus its value.
- `reflect_global_from_three`: Stores an input word in a global, and when any of bits 15, 14, 12, 11 or 10 of the input is set and a second routine returns nonzero, sets the global to three minus its value.
- `compute_status_flags_from_state`: Tests state bits of a global to decide which branch to take, adjusts a position, calls routines to compute flags, and returns a status value.
- `check_pattern_and_set_state`: Checks an input word against a mask pattern, with the pattern depending on whether a global is zero.
- `process_active_entries_in_table`: Walks 32 entries of 40 bytes each, counting entries whose code byte lies outside a small range and processing active ones with routine calls.
- `scale_byte_field_by_64`: Returns the byte at offset one of a structure, multiplied by 64.
- `clear_two_globals_partial`: Clears two globals, then the decompiled flow reaches a bad-instruction marker, so the rest of the routine is not recovered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `build_sprite_record_with_flip`: Builds one entry in a sprite record list.
- `build_sprite_record_variant`: Matches the sprite record builder before it: it optionally mirrors the horizontal position about a width of 320, computes positions and clipping amounts, stores them in the record fields, and advances the list pointer by 26 bytes.
- `process_entries_in_screen_window`: Walks a table of three-word entries until one has a negative first word.
- `offset_record_position_by_size`: Adjusts a record's horizontal and vertical positions by fixed offsets derived from two size fields, unless a flag bit is set.
- `allocate_free_record_slot`: Searches 32 records of 40 bytes each for one whose first byte has its top bit set, marking it free.
- `allocate_slot_with_pointer`: Performs the same free-slot search as the previous entry, and also stores a 32-bit pointer from an address register in the record it initialises.
- `offset_record_position_variant`: Matches the record position update of the earlier entry: it applies the same size-based offsets and, when the mode field is not one, calls a routine and stores a masked input word.
- `update_active_records_loop`: Loops backwards over 32 records and updates each active one according to its type by calling routines and setting state globals.
- `select_record_animation_frame`: Looks at a record's flag word.
- `set_record_frame_code`: Chooses a value from two flag bits of a record: 5 or 11 when bit eight is clear, and 6 when it is set.
- `set_size_globals_from_field`: Sets two globals to 32, then when a byte field is nonzero sets them to 16 and to the field value shifted left by four.
- `update_record_contact_state`: Updates a record's contact state from two of its flag bits.
- `check_record_range_and_store`: Checks whether a record lies within a range of a reference value and, if it does, clears a global, sets a timer-like field to 10 when appropriate, and stores a position-derived value.
- `update_record_sprite_list`: Updates a record's sprite list after a range check against a reference value, writing up to seven attached sprite parts into a shared buffer at a fixed address.
- `update_record_sprite_list_variant`: Matches the sprite list update of the previous entry: it writes parts of a record's sprite list into a shared buffer after a range check, in a loop of up to seven parts.
- `write_record_sprite_parts`: When a record's position value is at least 131, writes up to six sprite part values from the record into a shared buffer, selecting the layout from a type byte.
- `free_or_write_record_sprites`: Checks a record's pointer and a range value.
- `dispatch_record_code_events`: Reads a code byte from a record and updates a set of counters or states according to that code, with a separate branch for each of several specific codes.
- `store_byte_after_call`: Calls one routine, then stores the incoming byte in a record field at offset 24.
- `allocate_record_and_copy_parts`: Finds the first free record in a table of 32 entries and initialises it from its inputs.
- `return_from_subroutine`: This routine performs a single return instruction to exit the current execution flow.
- `flush_pending_entries_to_targets`: Walks a fixed table of 32 twenty-byte records and, for each non-zero entry, copies its value into a field of the object it points to, clears the entry and decrements a global count.
- `step_countdown_position_motion`: Decrements a countdown byte in the object and adds a per-step increment, looked up from a table by the current countdown value, to its position word.
- `dispatch_and_store_object_params`: Checks a status word's activity bit and a global, then calls a handler chosen from a 32-entry jump table by the low five bits of the status word.
- `dispatch_and_store_object_params_2`: Matches the same status-word dispatch as its sibling routine: it calls a handler selected by the low five bits of the status word, then copies parameter words from the object into an indexed table and returns the status word.
- `clear_and_toggle_status_bits`: Clears the top bit of a status byte in the object and then flips its lowest bit.
- `init_object_record_at_cursor`: When the cursor pointer lies below the frame address, it fills the record at the cursor with coordinates derived from the global value, using clamped offset tests for two axes, and then advances the cursor by one record.
- `append_clamped_object_record`: Writes a new object record at the global cursor when the cursor is below the frame address, computing clamped horizontal and vertical offsets and setting a state word and a sprite-style flag.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routine_if_cleared`: Returns zero, and calls an unnamed routine first only when a global word is zero.
- `service_pending_interrupt_bits`: Repeatedly tests four bits of a global pending mask in order, clearing each one as it goes, and calls a handler for a bit when the matching interrupt request bit is set in the custom chip registers.
- `update_timed_position_or_expire`: Decrements a counter in the object, and while the object's direction flag is negative it writes an absolute position into a list entry, computed from the object's base value and a stored offset.
- `latch_clamped_scroll_values`: Calls three unnamed routines, then writes a clamped difference between two stored values into a buffer entry.
- `latch_object_scroll_values`: Writes a value from the object into a buffer entry, computes a clamped difference between two object fields, and stores it in a second entry.
- `tick_countdown_then_invoke_routine`: Calls three unnamed routines, then, if a stored word is non-zero, decrements a counter byte and calls an unnamed routine once that counter reaches zero.
- `tick_object_countdown_handler`: When an object's pointer field is non-zero, decrements a countdown byte at a fixed offset and calls a handler when the countdown reaches zero, provided a linked value is set.
- `step_two_stage_ramp_value`: Advances a two-stage ramp in the object: the first stage steps the current value toward a limit read from a table, the second stage steps it toward a floor and then holds a level, and stage counters are updated along the way.
- `advance_three_record_command_streams`: Iterates over three fixed-size records, each with an active command pointer, reading the next byte-coded command from its stream.
- `parse_command_stream_for_record`: Reads bytes from a record's command stream: the escape value sets a parameter from the next byte and continues, while any other byte initialises the record's state from table entries and resets its counters.
- `parse_command_stream_for_record_2`: Parses the same kind of byte-coded command stream for a record.
- `resume_command_stream_for_record`: Starts parsing a record's byte-coded command stream from the stream pointer saved in the record.
- `resume_scaled_command_stream`: Stores a stream byte multiplied by a scale word into the record's timing field, then parses the record's command stream in the same way as its siblings.
- `set_divisor_and_parse_stream`: Computes a timing value by dividing the constant 750 by a stream byte, stores it in a record word, and then parses the record's command stream.
- `sum_prefixed_bytes_and_parse`: Reads a count byte and sums that many following bytes, scales the total by a stored factor into the record's timing field, then parses the record's command stream.
- `load_table_block_and_parse`: Copies a fixed block of fifteen words, selected by a stream byte, from a table into the record, clears its accumulators and flags, then parses the command stream.
- `parse_command_stream_from_cursor`: Parses a record's byte-coded command stream starting at a cursor passed in as an argument.
- `load_table_entry_and_parse`: Copies a fourteen-byte table entry selected by a stream byte into the record's fields and then parses the record's command stream.
- `start_stream_from_table_pointer`: Looks up a stream start pointer from a table by a stream byte and then parses the command stream from that position into the record.
- `copy_word_field_triples`: Copies two word fields to two other offsets in a structure, moving the base pointer two bytes per pass, and repeats this for three passes.
- `copy_word_field_pairs`: Copies two word fields to two other offsets in a structure, moving the base pointer two bytes per pass, and repeats this for two passes.
- `advance_command_stream`: A twelve-instruction routine that is called by another routine to advance command streams.
