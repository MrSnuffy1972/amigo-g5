# Game 1631 (james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01631). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (950976 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 101 raw-word lines |
| Functions named and described | pass, 221 of 221 named (0 author symbols), 221 of 221 described (4 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1770 of 1770 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7215 of 8799 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 43898 |
| code | 32220 |
| data | 733546 |
| unused | 141312 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf:bootblock | 1 | 28 | 1 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf boot-block load | 1 | 92 | 2 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf boot-block load | 1 | 1438 | 5 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5486 | 163 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1361 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 394 | 22 |

## Cases handled

- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf: custom-format (non-DOS) disk, 1482 non-empty blocks: matched against track-loaded segments
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf: the boot block reads 15360 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1348 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 152 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 270 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf: 3 logged loads cover 27648 bytes of the image
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf: 12 track-loaded blocks found in recovered runtime segments and classified through them
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf: blocks 4-1215 (bytes 2048-622592) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.65)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf: blocks 1219-1238 (bytes 624128-634368) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.54)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf: blocks 1242-1261 (bytes 635904-646144) outside the filesystem hold data, not code (opcode density 0.020, entropy 5.71)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf: blocks 1265-1284 (bytes 647680-657920) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.50)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf: blocks 1288-1431 (bytes 659456-733184) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.24)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf:bootblock

- `load_boot_data_from_disk`: Builds disk input/output request blocks that read the game's boot data into low memory and submits them through the operating system's I/O call.

### logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend][a].adf boot-block load

- `decompress_packed_data_backwards`: Decodes a packed block by reading control bits from the end of the data and writing output bytes backwards from a destination pointer.
- `stub_returns_without_effect`: The routine returns in both of its paths with no further work, so it is a stub with no behaviour to recover.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_bit_buffer_by_count`: Shifts a bit buffer right by a count held in a data register.
- `call_helper_when_flag_clear`: If a flag is zero, it calls a helper with the input value combined with a constant, then returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_blitter_shift_and_start`: Derives two shift values from a word, waits for the blitter to be idle, then loads the blitter's control, mask and size registers and starts an operation for each value.
- `blit_shifted_plane_rows`: Looks up a plane offset in a table by an input value, computes a destination address from a row count, and issues a series of blitter operations, each waiting for the blitter to be idle.
- `blit_masked_plane_rows`: Computes its destination address the same way as its sibling routine, but its blitter setup also uses the second source channel and the left-word mask.
- `walk_text_string_advances`: Walks a byte-coded string until its zero terminator.
- `advance_text_stream_position`: Walks the same kind of byte-coded string as its neighbours.
- `accumulate_text_advances`: Reads a byte-coded string to its terminator, adding a table advance for each ordinary character and applying the space, carriage-return and control codes.
- `walk_text_string_reload_code`: Walks a byte-coded string to its terminator.
- `walk_text_string_add_code`: Walks a byte-coded string to its terminator.
- `walk_text_string_jump_code`: Walks a byte-coded string to its terminator.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `compute_blitter_shift_masks`: Searches a six-word table for the first word with the lowest bit set, then again for the next bit, and shifts a value by the two positions it found.
- `step_blitter_row_pointers`: Waits for the blitter to be idle, then loops over a count of rows, setting the source and destination pointer registers and advancing both addresses by a stride after each row.
- `set_blitter_copy_defaults`: Writes default blitter values: the first and last word masks, a copy-mode control value and zero modulo values.
- `run_blitter_single_source_copy`: Waits for the blitter to be idle, sets its masks and control values for a single-source copy, loads the source pointer and a size value, then waits again for the operation to finish.
- `start_blitter_two_source_copy`: Waits for the blitter to be idle, sets its masks and control values, loads the two source pointers and a size value, and starts the operation.
- `blit_four_plane_strips`: Waits for the blitter, then issues four blitter operations for bitplane strips at fixed offsets from a base address.
- `set_blitter_state_partial`: The recovered pseudo-code shows only three register writes, although the routine is 149 instructions and also touches blitter, colour and DMA registers.
- `relay_helper_result`: Calls a helper and then copies its result to a global value, which has no further effect.
- `fill_stepped_long_table`: Fills a table with four 32-bit values, each larger than the last by a step, and then writes a fifth value from a second base.
- `choose_mask_and_fill_table`: Chooses a bit mask from several conditions on a state value, then fills a table of stepped 32-bit values in the same way as its sibling routine.
- `update_scroll_tile_position`: Updates a scroll record by moving its horizontal and vertical positions by stored speeds, splitting each into a fraction and a whole part, and recomputing the record's tile column and row by dividing the position by fixed sizes.
- `run_counted_helper_loops`: Runs a group of conditional counted loops, each calling a helper a number of times set by its count with a different step rule.
- `lookup_doubled_table_value`: Computes a table index from the high bits of a value, reads a 16-bit entry from a fixed table and doubles it.
- `compute_tile_row_address`: Computes the address of a tile row from a record's base address and a row index taken from a value, scaled by a row length of 38 bytes and an offset, then aligned to a 16-unit boundary.
- `store_row_address_and_step`: Stores an address adjusted by an offset in a record, calls helpers for 19 rows, then writes four address values with a fixed step to the output.
- `store_negative_row_address_step`: Stores an address adjusted by a negative offset in a record, calls helpers for 19 rows, then writes four address values with a fixed step to the output, like its sibling routine.
- `store_double_then_call_helper`: Stores twice a working value into a record field, then calls a helper fifteen times.
- `store_sum_then_call_helper`: Stores a value made from the working value doubled and offset by 36 into a record field, then calls a helper fifteen times.
- `blit_column_strips`: Calculates a column offset from a row value and the row length.
- `initialise_fixed_record_constants`: Writes a fixed set of constant values into fixed memory locations, including sizes, addresses and counters that are set to zero.
- `write_stepped_copper_entries`: Writes three stepped 32-bit values, then one more, into a list pointed to by a record.
- `toggle_paired_state_values`: Switches a state word between two fixed values and sets a pair of related markers each time it is called.
- `dispatch_handler_by_index`: Calls one of several handlers chosen from a jump table by a shared index value, then returns.
- `dispatch_indexed_handler`: Executes a subroutine selected from a jump table using an index stored in a global variable.
- `setup_tile_blit_parameters`: When an enable value is nonzero, it computes a tile position from two inputs, waits for the blitter to be idle, then programs its masks, control and size registers and sets the source and destination offsets.
- `setup_blitter_for_row`: When an enable value is nonzero, it sets a source offset from a shift value, waits for the blitter, writes its mask and size registers and stores a fixed base address.
- `dispatch_after_two_helpers`: Calls two helpers, then calls a handler chosen from a jump table by a shared index.
- `execute_sequential_handlers`: Calls two subroutines in succession before dispatching a third subroutine from a jump table via a global index.
- `copy_tile_block_rows`: Copies a 16 by 16 block of tile words from a source table into a destination area, writing each word to two positions 16 words apart.
- `write_indexed_pattern_entries`: Calls a helper for each index from zero to four, then writes fixed pairs of 32-bit values into a set of list entries that depend on the index.
- `clear_matching_records_for_key`: Walks the first given number of 26-byte records.
- `clear_matching_records_in_range`: Walks the 26-byte record table and, for each active record whose key matches the input and whose stored values fall in certain ranges, calls a helper and clears the record's active word.
- `blit_sprite_part_with_masks`: Positions a sprite part from its record fields and a shift value, computes blitter masks from its flags, and programs the blitter with source and destination pointers for row strips.
- `find_free_list_entry`: Searches a list of up to 32 ten-byte entries for one whose status word is zero.
- `clear_active_list_entries`: Sets blitter masks and control values, then walks up to 32 ten-byte entries in a list, calling a helper for each entry whose status word is nonzero and clearing that word.
- `blit_wrapped_plane_strips`: Waits for the blitter, then issues groups of four blitter operations for plane strips at fixed offsets.
- `call_helper_four_times_reset`: Sets a base value, calls a helper four times, then clears three working variables.
- `clear_all_record_active_words`: Walks 32 of the 26-byte records from a fixed table and clears each record's active word.
- `map_underscore_to_all_ones`: If a byte equals the underscore character, it clears the byte and sets it to all ones.
- `copy_fixed_block_four_times`: Copies four 16-byte blocks from a fixed source table to a fixed destination area.
- `enter_infinite_loop`: Enters an infinite loop that prevents further execution.
- `poll_helper_until_code`: If a flag is set, it clears the flag and returns.
- `enable_audio_dma_channels`: Sets the location and length registers of the four audio channels to fixed values and writes a DMA control value with the enable bit set.
- `read_vertical_beam_position`: Reads the video beam's vertical position register pair and returns a nine-bit line value.
- `install_interrupt_vectors_and_copper`: Fills a block of low-memory vectors with a fixed value, installs two interrupt handler addresses and a further set of handler addresses, then configures interrupt enable and copper registers.
- `pulse_serial_mode_bit`: Sets one bit of a control value, calls a helper, then clears that bit and writes a fixed low value.
- `check_interrupt_request_level`: Reads an interrupt request flag word and, when one bit is set and a condition holds, calls a helper and caps its result at 255.
- `decode_keyboard_serial_byte`: Decodes a keyboard serial byte when a flag bit is set, shifting and inverting it and calling helpers, then resets the flag word.
- `service_interrupt_request_bits`: When one bit of the flag word is set it calls a helper, and when another is set it calls a helper a number of times equal to a counter, unless the counter's top bit is set.
- `dispatch_pending_interrupt_bits`: If the flag word is zero, it keeps only one group of bits and returns.
- `mask_flag_word_group`: Keeps only one group of bits in a flag word and returns.
- `return_immediately`: An empty routine that returns to its caller.
- `read_joystick_direction_bits`: Reads the first joystick port's movement register and the fire-button register, decodes the up, down, left and right directions from their bit pattern, and calls helper routines depending on the result.
- `stub_bad_instruction_data`: The decompiler reports bad instruction data here, so only part of the routine is recoverable.
- `set_flags_for_key_codes`: When a state word is nonzero, it inverts or sets it for particular key codes, using a set of letter codes.
- `busy_wait_store_masked_value`: Runs a delay loop of 10,000 iterations, storing a masked value from a counter on each pass.
- `busy_wait_store_wide_mask`: Runs a delay loop of 10,000 iterations, storing a wider masked value from a counter on each pass.
- `busy_wait_store_split_mask`: Runs a delay loop of 10,000 iterations, storing a value masked with two separate bit groups on each pass.
- `busy_wait_colour_register_mask`: Runs a delay loop of 10,000 iterations, storing a masked value on each pass.
- `shift_register_with_feedback`: Shifts a 32-bit register left by one bit, then sets its lowest bit according to two bit tests on the old value.
- `advance_cycle_counter_and_scramble`: Advances a small counter that cycles from one to five, then replaces its value with the product of 177 and the counter, taken modulo 30,269.
- `format_number_as_decimal_text`: Converts a number into decimal characters by repeated subtraction against a table of place values, writing each digit into a text buffer, then replaces leading zeros with spaces.
- `stub_bad_instruction_table`: The decompiler reports bad instruction data here, so this is not a recoverable routine.
- `set_playfield_control_registers`: Sets the playfield's scroll and priority control registers to fixed values, calls a helper, then writes a DMA control value that enables the channels.
- `write_stepped_pointer_table`: Writes six 32-bit pointers into a fixed table, each one stepped from an input address by a computed amount, and stores a value from the input into a record.
- `copy_and_clear_table_blocks`: Copies 32 words from one table to another and clears the source.
- `clear_word_table_block`: Clears a block of 544 words in a table and sets a counter to 30.
- `fill_tables_from_helper_results`: Fills a 32-entry table, and then 256 entries of two tables, with values returned by a helper, and sets a counter to 30.
- `compute_grey_colour_from_nibbles`: Adds together the three four-bit groups of a word and a second value, caps the total at 47, then returns a colour value with equal red, green and blue parts.
- `fill_tables_from_helper`: Decrements a counter, then fills tables with values from a helper.
- `stub_no_operation_return`: The routine only returns, so it is a stub with no behaviour to recover.
- `process_four_nonzero_table_bytes`: Walks four bytes starting at a table offset, and for each nonzero byte calls a helper with the byte's position.
- `scan_four_records_nonzero`: Checks a field in four records of a fixed table, spaced 37 longs apart, for a nonzero value.
- `advance_four_counters_until_limit`: When a flag is set, it increments four counters together.
- `decode_packed_flag_byte`: When a flag is zero, it reads the next byte from a stream and sets five flags from its low bits, advancing the stream pointer when a counter wraps.
- `initialise_flag_byte_stream`: Writes an end marker into a buffer, sets the stream pointer to the byte after it, and clears the counter and the state word.
- `write_cia_a_value_bytes`: Sets a state word from its previous value and writes the low and high bytes of a value to two fixed CIA-A memory locations.
- `write_cia_b_value_bytes`: Sets a control value in the CIA-B register block, then writes the low and high bytes of a value to two fixed CIA-B memory locations.
- `call_helper_over_strided_values`: Calls a helper once for each value in a counted series, starting from a value taken from the input and stepping by two each time, and increments a counter on each call.
- `update_sprite_entry_controls`: Waits for the blitter, stores a table value only when it changes, then writes control words for a sprite-like entry.
- `update_number_display_text`: When a value differs from its stored copy, it copies decimal digits from a text buffer into a display record and calls a helper for each digit that changed.
- `copy_interleaved_plane_words`: Copies ten groups of four words from a source area into a destination, placing two of the words in a second plane at a fixed distance.
- `wait_blitter_then_record_pointers`: Waits for the blitter to be idle four times, then records two pointer values and a size constant for the blitter's pointer and size registers.
- `wait_for_cleared_flag`: Clears a flag and waits until it reads zero.
- `step_counter_with_helpers`: Returns at once when a counter is nonzero, otherwise it increments the counter, calls helpers and resets the counter to zero at the end.
- `reset_game_state_constants`: Clears a block of 3,432 words, calls a helper, then writes a set of starting constants into fixed variables, including counters, limits and a fixed 32-bit constant.
- `fill_sentinel_values`: Writes all-ones sentinel values to four byte variables and to six long variables.
- `setup_blitter_and_wait`: Calls three helpers, waits for the blitter to be idle, sets its control and mask values and two size-related constants, then waits for the blitter again.
- `compute_aligned_offset_state`: Chooses a state value from a few comparisons and, in one case, computes a value rounded to a 16-unit boundary with an offset of 8, then clears several state words.
- `clear_twenty_state_words`: Writes zero to twenty state words.
- `clear_state_and_set_defaults`: Clears four state words, then runs three similar checks that each set three state values when their condition holds.
- `update_movement_state_machine`: Updates a movement state by selecting a timing or velocity constant from a state value, applying a range adjustment, and calling different numbers of helper routines depending on a counter.
- `step_and_clamp_signed_value`: Steps a value by a fixed increment and clamps it to a limit that depends on a state byte.
- `step_state_and_clamp_upper`: Advances a small state value by two and clamps it at fixed limits, with one branch handling a zero state and another adding fixed offsets.
- `adjust_state_by_fixed_step`: Adjusts a value by one of two fixed steps, 12,288 or 24,576, chosen from a state value, and sets a state code of four at the end.
- `adjust_state_clamp_at_eight`: Adjusts a value by one of two fixed steps, 12,288 or 24,576, depending on its sign, and clamps the resulting state to eight.
- `update_state_with_thresholds`: Updates a movement state through several threshold tests, setting it to fixed values and returning early when a magnitude is within a small range.
- `compute_capped_high_word_state`: Adds a constant to a value, takes its high word capped at 15, then sets one of two fixed state codes depending on bits of that word.
- `apply_flag_group_defaults`: Checks a flag group in a word and calls helpers when it matches, then sets two default values to fixed codes, one of them only when a zero test passes.
- `step_toward_target_within_eight`: Compares a value with a target and moves the value toward the target by up to eight units when the two are close.
- `update_motion_with_clamped_step`: Updates a motion value by a clamped step computed from its previous value, with separate handling when a flag is set, and calls helpers around the update.
- `apply_capped_velocity_step`: Sets a default value when two conditions hold, calls helpers, then computes a step from a value offset by 5,462.
- `apply_capped_velocity_offset`: Adds an offset of 3,280 to a value, caps the high word of the result at 2 on the positive side and 15 on the negative side, doubles it, then calls helpers and clears the value when a flag is set.
- `run_helper_sequence_clear`: Calls a sequence of six helpers with register copies between them, then clears the value when a flag is set.
- `move_value_toward_target`: Moves a value toward a target by a step based on the difference, limited to half the difference after its sign bit is masked.
- `copy_pair_after_helper`: Calls a helper, then copies two register values into two global variables.
- `copy_pair_after_helper_twin`: Calls a helper, then copies two register values into two global variables.
- `subtract_and_correct_negative`: Subtracts a register value from a global, and when the result is negative and within a range, adds twice the register value back.
- `add_and_correct_overflow`: Adds a register value to a global, subtracts twice that value when the sum is nonnegative and above a limit, then sets a state value to two.
- `subtract_and_clamp_state_four`: Subtracts a register value from a global and, when the result is negative, adds the register value back in some cases.
- `add_and_clamp_state_eight`: Adds a register value to a global and, when the sum is nonnegative and its high word is below nine, subtracts the register value again.
- `adjust_value_by_capped_steps`: Chooses a limit of 12,288 or 8,192 from a flag, then adjusts a value by up to fifteen steps in the direction of its sign, with a second step that depends on a bit test.
- `adjust_value_capped_steps_copy`: Adjusts a value by up to fifteen steps in the direction of its sign, the same as its sibling routine, with a second step that depends on a bit test.
- `step_value_by_capped_amount`: Moves a value by up to fifteen units in the direction of its sign, with a second step that depends on a bit test.
- `add_clamped_step_to_value`: Clamps an input value to the range from minus fifteen to plus fifteen, then adds it to a global value.
- `conditional_threshold_step_helper`: When a state word is zero, it clears the high word of a value.
- `adjust_by_stepped_multiples`: When a state word is one, it moves a value by up to four steps of 8,192 toward or away from zero, depending on the sign of the difference between two inputs.
- `stub_gap_return_routine`: The routine returns without any effect on every path.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 1 into d6, then returns.
- `search_word_pair_table`: Searches a table of word pairs for an entry that matches a key derived from the input.
- `increment_indexed_counter_entry`: Calls a helper with a fixed index, then increments a counter entry in a table at a fixed offset.
- `update_position_with_clamped_limits`: Updates a position value toward a target, clamping the change within limits and handling a few special state codes.
- `update_position_within_bounds`: Compares a position with an input and, when they are close, moves the position by a bounded difference and clamps it at zero.
- `advance_table_counter_twice`: Twice, when a counter is nonzero, adds a byte looked up at the counter's index to a value and then decrements the counter.
- `select_masked_state_value`: Chooses a state value from a few tests on a counter, calling helpers in some cases, and returns it masked with a bit pattern.
- `set_state_codes_by_counter`: When a state word is zero and negative, it picks a fixed code from a chain of tests and sets state fields to fixed values, such as 172 and 125.
- `set_state_codes_gap`: Uses the same state-code chain as its neighbouring routine, with a different fixed value in one branch.
- `reset_state_code_when_zero`: When a state code is zero it sets a fixed value and clears related fields, handling one value specially.
- `set_state_seven_defaults`: Sets a state to 255 when it is nonzero, then, when the state is not seven, sets it to seven and sets several related fields to fixed values.
- `select_fixed_offset_by_flag`: Sets a state value to one of two fixed offsets depending on a flag, then clears the state.
- `clear_to_fixed_offset`: Sets a state value to a fixed offset, then clears it to zero.
- `advance_state_with_constant`: When a state is zero, it adds a constant of 500,000 and then adds one, using a special value of 52 for a zero result.
- `transition_state_from_table`: Updates a state through a table-driven transition, selecting a starting entry from a series of tests and then following a chain of table offsets until it reaches a non-negative one.
- `follow_offset_table_chain`: Steps a counter back, then follows a chain of table offsets until it reaches a non-negative entry, moving the pointer by that entry.
- `lookup_tile_flag_value`: Looks up a tile entry in a table from two position values and checks its flag bits.
- `lookup_tile_neighbour_value`: Looks up a tile entry in a table by position, then checks neighbouring entries in the same table and keeps one whose flag bits are set.
- `move_value_toward_target_gap`: Moves a value toward a target by a step based on the difference, limited to half the difference after its sign bit is masked.
- `step_value_limited_by_input`: When its input is at most 21, subtracts a step from a global value, halving the step when the difference is positive and large, and adds to it when the input is negative.
- `add_secondary_offset`: Adds a secondary offset from a data register to a global value.
- `subtract_halved_offset_when_larger`: Subtracts an offset from a global value, halving the input first when it is larger than the offset.
- `subtract_secondary_offset`: Subtracts a secondary offset from a data register from a global value.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d6, then returns.
- `copy_memory_block`: copies a block of data from a source address to a destination address using a counter.
- `stub_return_unreached`: The routine only returns, so it is a stub with no behaviour to recover.
- `perform_no_operation`: The routine contains eleven instructions but performs no functional changes to the processor state or memory before returning.
- `invert_mask_register_bits`: Returns the bitwise inverse of an input word masked to bits 2 and 3, so the caller receives a two-bit value.
- `reflect_variable_when_masked`: Calls two helper routines, then tests the high bits of an input word against a fixed mask.
- `store_word_then_reflect`: Calls a helper, copies an input word into a shared variable, and if any masked high bit is set and a helper returns nonzero, replaces that variable with three minus its value.
- `evaluate_status_mask_adjustment`: Tests several status masks on a shared variable and returns a word when one pattern matches.
- `match_mode_bits_return_flag`: Checks a word passed in a register against one of two bit patterns, chosen by whether a shared variable is zero.
- `iterate_object_table_update`: Walks a 32-entry table of 40-byte records and counts active entries whose type code lies outside two excluded ranges.
- `multiply_record_byte_by_64`: Reads the byte at offset one from the record addressed by an address register and returns it multiplied by 64.
- `clear_two_shared_variables`: Writes zero to two shared variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `mirror_and_store_object_slot`: If a flag is set, it temporarily toggles a bit and recomputes the horizontal position around a 320 reference value.
- `clip_and_store_object_slot`: Clips an object's horizontal and vertical offsets against fixed bounds, then fills the next free slot of a 26-byte record table with its clipped position, size and state.
- `release_entries_inside_window`: Computes a clamped window around a point and walks a list of three-word entries until a negative marker.
- `adjust_object_position_by_size`: Subtracts half of one size field, less 16, from the x position and the other size field, less 16, from the y position, unless a flag bit is set.
- `allocate_object_slot_from_table`: Searches a 32-entry object table for the first inactive entry and fills it with the position and type values passed in registers, clearing its other fields.
- `allocate_object_slot_with_pointer`: Searches a 32-entry object table for the first inactive entry and fills it with register values, including an address value stored as a long word.
- `offset_position_from_source_record`: Subtracts half of one source size field, less 16, from the destination x position and the other size field, less 16, from the y position, unless a flag bit in the source is set.
- `process_object_table_entries`: Loops over a 32-entry table and dispatches each active entry by its type code range to helper routines, with one exact code value handled separately.
- `update_object_state_by_flags`: Chooses a state for an object from bits in its status word.
- `set_status_code_from_flags`: Picks a status code from two bits of an object's status word and writes a fixed status pattern with it.
- `compute_scaled_size_from_byte`: Sets a shared variable to 32 by default.
- `update_contact_state_in_window`: Checks an input flag word and, when the right bits are set, updates an object's state fields depending on whether a horizontal offset falls inside a window of 25 or 16 units.
- `apply_window_contact_response`: Tests whether a horizontal offset from the object's position lies inside a window of 25 or 16 units, and when it does, updates a countdown field and a vertical value.
- `emit_object_entries_to_list`: Sets up a few shared values from the object's status flags, then copies up to eight nonzero fields of the object into a list buffer at an offset derived from its position.
- `copy_object_fields_to_list`: Copies an object's fields into a list buffer at an offset derived from its position, with the layout chosen by its type byte.
- `retire_object_past_threshold`: Counts down a timer field on an object, then calls a helper and compares its result with a threshold.
- `remove_object_outside_bounds`: Checks whether an object's stored list entry still lies within fixed horizontal and vertical bounds around a shared position.
- `decode_command_code_byte`: Decodes a one-byte code from an input word through a chain of comparisons, setting a shared counter or state for each recognised value.
- `store_data_byte_into_record`: Calls a helper routine, then stores a byte from a data register into a fixed field of the object record addressed by an address register.
- `spawn_object_into_free_slot`: Finds the first free slot in a 32-entry object table and initialises its fields from register inputs.
- `return_from_subroutine`: The routine performs a single return instruction to exit the current execution flow.
- `release_table_entries_to_records`: Walks the 32-entry object table and, for every entry with a nonzero first word, copies that word into the field of its linked record, clears the entry and decrements the active count.
- `advance_countdown_position_step`: Advances an object's position by a value from a lookup table indexed by its countdown byte, then decrements that countdown.
- `dispatch_object_state_handler`: Reads an object's status word and, when its flag conditions hold, calls a handler selected from a jump table by the word's low five bits.
- `dispatch_object_handler_direct`: Has the same body as the preceding routine, but is reached through a direct call rather than by taking its address.
- `clear_flag_and_toggle_bit`: Clears the top bit of a status byte at offset 30 in an object record, then flips the lowest bit of the same byte.
- `clip_and_store_indexed_slot`: Computes a base address from an index register, then, if the record table has a free slot, clips the object's offsets against fixed bounds and stores it there with a fixed size of 32.
- `clip_and_store_offset_slot`: Clips an object's offsets against fixed bounds, adding an offset from an address register into its position, and stores it in the next free slot of a record table with a fixed size of 32 only when the horizontal clip value is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_helper_when_zero`: Calls a helper routine only when a shared variable is zero, then returns zero in all cases.
- `service_interrupt_request_bits_2`: Loops over four pending-event flags kept in a shared variable.
- `update_countdown_and_place_sprite`: Decrements a countdown in an object record, then places the object at a position computed from its stored offsets when a field is negative.
- `refresh_linked_record_state`: Calls three helper routines, then writes a clamped position difference and a fixed value into a linked record.
- `refresh_linked_record_from_object`: Writes a clamped position difference and a stored value into the record linked from an object, and when a flag is set, clears it and copies stored values into that record.
- `tick_countdown_and_call_helper`: Calls three helper routines, then, if a shared flag and a value are both set, decrements a countdown byte and calls a helper when it reaches zero.
- `return_target_after_timer`: When an object's enable field is set, it returns a target value and calls a helper once a timer byte has counted down to zero.
- `advance_three_phase_level_sequence`: Advances a three-phase level sequence: each phase reads step and target bytes from a data block, moves the stored level toward its target, and moves to the next phase when the target is reached.
- `update_four_sound_channels`: Steps through four channels in turn, each with its own state block, matching the layout of the Amiga's four audio channels.
- `parse_channel_command_stream`: Stores the first byte of a sound channel's sequence stream as a channel offset, then reads the next command.
- `parse_channel_stream_entry`: Alternate entry into the sound command decoder that starts at the stream's current position.
- `start_channel_from_stored_pointer`: Loads the channel's stored sequence pointer and runs the command decoder from it.
- `start_channel_with_scaled_length`: Reads a length byte from the stream, multiplies it by a value taken through a pointer, and stores the product as the channel's duration before running the command decoder.
- `start_channel_with_divided_rate`: Divides a fixed constant of 750 by a value read from the stream and stores the quotient through a pointer, then runs the command decoder.
- `start_channel_with_summed_length`: Adds up a run of bytes from the stream, with the count taken from the first byte, and stores the total scaled by a pointer value as the channel's duration.
- `load_channel_instrument_table`: Clears two mode bits on a channel, then copies a block of fifteen bytes from a table selected by a stream byte into the channel record and zeroes its counters.
- `run_channel_command_loop`: Runs the sound command decoder from a given stream position without any preset parameters.
- `load_channel_preset_and_run`: Loads three preset words and two long words from a fourteen-byte table entry chosen by a stream byte into the channel record.
- `start_channel_from_pointer_table`: Selects a sequence start address from a pointer table using a stream byte and runs the command decoder from there.
- `copy_word_pair_three_times`: Copies two word fields of a structure from one pair of offsets to another, repeating the copy three times while stepping the pointer by two bytes each time.
- `copy_word_pair_twice`: Copies two word fields of a structure from one pair of offsets to another, repeating the copy twice while stepping the pointer by two bytes each time.
- `stub_decompile_timed_out`: The decompiler timed out, so no body was recovered for this routine.
