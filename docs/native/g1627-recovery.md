# Game 1627 (james pond 2 - codename robocod (1991)(millennium)[cr skr][t +2 soldiers]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01627). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (950976 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 96 raw-word lines |
| Functions named and described | pass, 222 of 222 named (0 author symbols), 222 of 222 described (4 by rule, 17 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1770 of 1770 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7156 of 7396 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 30383 |
| code | 29696 |
| data | 734224 |
| padding | 1 |
| unused | 156672 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf:bootblock | 1 | 16 | 1 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf boot-block load | 1 | 139 | 8 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5486 | 163 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1361 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 394 | 22 |

## Cases handled

- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +2 soldiers]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf: custom-format (non-DOS) disk, 1436 non-empty blocks: matched against track-loaded segments
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +2 soldiers]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +2 soldiers]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1348 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 152 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 270 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +2 soldiers]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf: 2 logged loads cover 12288 bytes of the image
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +2 soldiers]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf: 12 track-loaded blocks found in recovered runtime segments and classified through them
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +2 soldiers]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf: blocks 4-1215 (bytes 2048-622592) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.65)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +2 soldiers]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf: blocks 1219-1238 (bytes 624128-634368) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.54)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +2 soldiers]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf: blocks 1242-1261 (bytes 635904-646144) outside the filesystem hold data, not code (opcode density 0.020, entropy 5.71)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +2 soldiers]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf: blocks 1265-1284 (bytes 647680-657920) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.50)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +2 soldiers]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf: blocks 1288-1431 (bytes 659456-733184) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.24)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf:bootblock

- `boot_load_and_relocate_block`: Builds two exec I/O request blocks, runs each through the device do-I/O call, and enables DMA on the way.

### logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +2 Soldiers].adf boot-block load

- `poll_keyboard_switch_display_buffer`: Reads the keyboard serial register and decodes the code.
- `render_text_string_glyphs`: Walks a zero-terminated string and looks up each character in an alphabet of letters, digits and punctuation.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 2 into d3, then returns.
- `copy_block_on_key_code`: Copies a block of words from one of two source buffers into a fixed destination and toggles the buffer selector.
- `copy_block_then_await_key`: Copies a block of words from one of two source buffers and toggles the buffer selector.
- `poll_keyboard_then_copy_block`: Copies a block of words into a fixed destination and toggles the buffer selector.
- `copy_block_on_alternate_key`: Copies a block of words into the destination, choosing the source buffer from a flag.
- `clear_bit_in_byte`: Clears a specific bit within a byte at a memory location determined by a pointer and an offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `program_blitter_shifted_copy`: Computes shift counts from a source position, waits for the blitter to become idle, then programs its first-word mask, modulo, pointer, control and size registers.
- `blit_table_driven_rows`: Looks up a shift and offset from a table indexed by a value, then issues several blitter operations, each after waiting for the blitter to finish.
- `blit_masked_glyph_rows`: Looks up a table entry from a character value, computes a screen address, and draws several rows with the blitter using first and last word masks.
- `decode_text_layout_commands`: Reads a byte stream of text and layout codes.
- `advance_cursor_per_character`: Walks a text string and moves the cursor by a per-character advance taken from a table.
- `scan_text_space_control_codes`: Scans a text string where a space adds a fixed step and a zero byte ends it.
- `read_text_control_parameters`: Reads text bytes and control codes.
- `accumulate_text_byte_offsets`: Handles a text stream where a control code adds the next byte to a running total.
- `apply_text_control_step_bytes`: Handles text bytes where one control code adds a following byte to the running total.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `compute_blitter_bit_shifts`: Finds the first set bit in each of two six-entry masks, then shifts a register by those positions to produce blitter shift values.
- `copy_rows_after_blitter_idle`: Waits for the blitter to finish, then runs a counted loop that copies a block row by row between two pointers.
- `set_blitter_copy_defaults`: Writes fixed values to the blitter mask, modulo and copy-mode control registers and clears the shift registers.
- `start_blitter_destination_write`: Waits for the blitter, sets full first and last word masks, and selects a destination-only control mode.
- `start_blitter_copy_from_source`: Waits for the blitter, sets the masks and a copy-mode control word, then loads source and destination pointers and a blit size from its arguments to start a copy.
- `blit_tile_into_four_planes`: Waits for the blitter, then issues a set of blits to one destination address, advancing it by a fixed plane stride on each of four passes.
- `set_background_colour_register`: Writes a fixed value to the background colour register and clears a register before returning.
- `forward_to_helper_routine`: Calls one unnamed routine routine and then assigns a global variable to itself before returning.
- `build_bitplane_pointer_list`: Calls several unnamed routines, then splits 32-bit plane addresses into high and low words and writes them into a list at a four-word stride.
- `choose_background_colour_value`: Chooses a colour value through a series of conditional checks and writes it to the background colour register.
- `compute_scroll_nibble_from_delta`: Subtracts two coordinate values and takes the low four bits of a third value to build a sign-adjusted shift nibble.
- `run_helper_calls_in_loops`: Calls unnamed routines in nested loops, with a 19-row outer loop and fifteen calls per row when a flag is set.
- `lookup_nibble_table_value`: Takes the high nibble of a value, doubles it to form a table offset, and reads a word from a fixed table.
- `compute_screen_row_address`: Multiplies a masked row value by a row stride of 38 bytes, adds a fixed base offset, and masks the result to a 16-byte boundary.
- `blit_column_pair_with_wrap`: Adds a fixed offset when a flag is set, then calls unnamed routines around a loop of nineteen steps.
- `blit_column_pair_negative_offset`: Subtracts a fixed offset when a flag is set, then calls unnamed routines around a loop of nineteen steps.
- `store_doubled_value_and_loop`: Stores a global value added to itself into a structure field, then calls an unnamed routine fifteen times in a loop.
- `store_offset_sum_and_loop`: Stores a global value plus 36 plus the same global value into a structure field, then calls an unnamed routine fifteen times in a loop.
- `blit_tile_rows_with_offsets`: Computes row and column offsets from a column index and a masked row value, then copies three steps of words between two pointers after waiting for the blitter.
- `initialize_block_state_fields`: Stores a set of fixed constants and table addresses into a block of memory, including several pointer fields and counters.
- `write_copper_list_two_pointers`: Splits 32-bit plane addresses into high and low words and writes them into a second copper list at a four-word stride.
- `toggle_buffer_pointer_pair`: Compares a stored value with a fixed constant and writes one of two pairs of values to a set of globals based on the result.
- `execute_indirect_call`: Performs an indirect call to a routine selected from a table using an index stored in a global variable.
- `call_indirect_via_table`: performs an indirect call to a routine selected from a table using an index stored in a global variable.
- `blit_tile_block_with_masks`: Computes a block size and blitter masks from input coordinates, then programs shift and modulo control values and starts a blit.
- `update_global_state_and_registers`: Checks a global variable, resets it, and then loads a value from a memory offset into it.
- `execute_nested_calls`: Calls two routines sequentially and then executes an indirect call from a table using a global variable index.
- `execute_nested_calls_2`: calls two routines sequentially and then performs an indirect call to a routine selected from a table using a global variable index.
- `copy_sixteen_tiles_into_buffer`: Copies a grid of sixteen tiles, each 32 bytes, from a table selected by a byte index into a destination buffer.
- `write_slot_pattern_by_index`: Calls unnamed routines for indexes from four down to zero, then writes the same two-word pattern into a series of slots at offsets that depend on the index.
- `clear_matching_entries_with_helper`: Walks a table of 26-byte entries for a count held in a global.
- `process_matching_table_entries`: Walks a table of 26-byte entries and, for each active entry at a matching level within a range of positions, calls an unnamed routine and clears its active field.
- `update_record_screen_position`: Calls unnamed routines, then computes a screen position for a record from a table lookup and its stored offset, writing the coordinate pair.
- `find_free_slot_or_call`: Scans up to 32 ten-byte entries in a list for one whose field is zero.
- `run_helper_for_active_slots`: Sets blitter mask and control registers, then walks up to 32 ten-byte entries.
- `blit_clipped_rows_across_wrap`: Copies two coordinate words into a structure, then computes source and destination addresses for a block that may run past the plane boundary.
- `reset_slot_table_call_helpers`: Sets a global to a table base, calls an unnamed routine four times, and then clears three globals.
- `clear_active_flags_in_table`: Sets a global to the start of a table of 26-byte entries, then clears a 16-bit field in each of thirty-two entries.
- `reset_flag_when_marker_matches`: Compares a global byte with a fixed marker character and, when it matches, clears the byte and inverts it.
- `copy_fixed_table_into_globals`: Copies sixteen longwords from a constant table into a run of globals, four per step.
- `stub_infinite_loop`: Contains only an infinite loop, so it never returns.
- `poll_helper_until_value_matches`: Calls an unnamed routine repeatedly until it returns 252, returning early if a flag is set.
- `initialize_audio_channels`: Points the four audio channel locations at a shared address, sets each channel's length to one, and enables audio DMA for those four channels.
- `read_raster_line_position`: Reads the video beam position registers and masks the value to nine bits, returning the current raster line.
- `initialize_interrupt_vectors`: Fills a block of ten longwords with a fixed value and stores handler addresses in two interrupt vector slots and four table slots.
- `run_cia_serial_mode_step`: Sets the serial-mode bit of a CIA control register, calls an unnamed routine, then clears that bit again and writes a fixed value to the interrupt control register.
- `acknowledge_pending_interrupt_bit`: Reads the interrupt request register and, when the bit for one source is set, calls unnamed routines to acknowledge it.
- `handle_keyboard_serial_byte`: Checks a flag bit, then rotates a byte read from the keyboard serial register.
- `service_two_interrupt_source_bits`: Checks two interrupt source bits in a status word.
- `service_interrupt_status_bits`: Checks four interrupt status bits in turn.
- `mask_interrupt_status_word`: Masks a global status word with a fixed bit pattern, leaving only the bits inside that pattern set.
- `stub_empty_return`: Returns immediately without doing anything, so it works as an empty hook for callers.
- `read_joystick_direction_and_fire`: Decodes the joystick movement register into direction flags and checks the fire button in the parallel port register.
- `match_typed_key_sequence`: Compares an incoming key character with the next expected character of a stored sequence and advances through it on a match.
- `apply_key_letter_to_flags`: When a flag is set, maps a key character to changes of several globals.
- `store_bits_8_11_repeatedly`: Runs a loop of 10000 passes, storing the counter ANDed with a fixed mask into a global on each pass.
- `loop_mask_store`: Decrements a counter from ten thousand while storing the result masked with a specific value into a global variable.
- `loop_mask_store_alt`: Decrements a counter from ten thousand while storing the result masked with a different value into a global variable.
- `store_colour_nibble_repeatedly`: Runs a loop of 10000 passes, storing the counter ANDed with a fixed mask into a global on each pass.
- `shift_left_fix_bit_pattern`: Shifts a global value left by one, then adds one when the top bit and bit one differ.
- `advance_cycle_counter_and_seed`: Advances a counter that cycles between one and five, then replaces a global with that value times 177 modulo 29245.
- `format_number_as_decimal_digits`: Converts a number to eight decimal digits by repeated subtraction against a table of powers of ten, writing ASCII characters into a buffer.
- `stub_decimal_power_table_data`: Holds bad instruction bytes, so the disassembler stopped here.
- `reset_display_control_registers`: Clears the DMA control register, sets the playfield scroll and priority registers, calls an unnamed routine, then re-enables a set of DMA channels.
- `write_stepped_address_pairs`: Computes a stride from input nibbles plus a fixed offset, then writes six address pairs at an eight-byte step.
- `swap_and_clear_buffer_tables`: Copies a 32-word table to a second location while zeroing the source, then moves a 256-word table the same way.
- `clear_table_and_reset_counter`: Zeroes 544 words starting at a fixed table address, then sets a global counter to 30.
- `fill_tables_from_helper_values`: Fills 32 words of one table with values from an unnamed routine, then fills two 256-entry tables from unnamed routine values in pairs.
- `compute_palette_shade_value`: Adds the three nibbles of a 12-bit value, offsets the sum by another value, and clamps results above 46 to 47.
- `fill_paired_colour_tables`: Fills 256 table entries from unnamed routine values and writes each value to two tables 512 bytes apart when a flag bit is clear.
- `empty_routine`: This routine is empty and performs no operations before returning.
- `process_nonzero_table_bytes`: Walks four bytes of a table from a given offset and calls an unnamed routine for each nonzero byte.
- `scan_records_return_input`: Scans four records of 148 bytes for a nonzero first field, but returns the input value on every path, so the result does not depend on the scan.
- `advance_four_counters_and_flag`: While a global flag is set, increments four counters.
- `decode_bitmask_byte_stream`: Reads one byte from a stream and expands its low five bits into five flag words, each set to all ones or zero.
- `initialize_bitmask_byte_stream`: Writes an end marker byte at a buffer address, points the stream pointer just past it, resets the read counter, and clears a global flag.
- `start_cia_timer_a`: Writes the low and high bytes of a 16-bit count to the first CIA's timer A registers.
- `start_cia_b_timer_a`: Writes the low and high bytes of a 16-bit count to the second CIA's timer A registers.
- `emit_sequence_of_stepped_values`: Reads a start value from a record and calls an unnamed routine for each step of a sequence that moves by two.
- `draw_record_with_blitter`: Waits for the blitter, stores blit control values, then computes a destination address from a record's coordinates and a lookup table.
- `update_palette_entries_and_blit`: Compares a stored value with a cached one and, when they differ, rewrites byte pairs from a source table and calls unnamed routines.
- `distribute_bytes_across_two_planes`: Distributes eleven groups of four source bytes across two interleaved positions.
- `start_fixed_size_blit`: Waits for the blitter to be idle four times, then loads source and destination pointers from its arguments with fixed offsets and starts a blit of a fixed size.
- `clear_and_wait_for_flag`: Clears a global and busy-waits until it becomes nonzero.
- `advance_state_with_helper_calls`: Updates a state global by checking its sign and value, then increments it and calls unnamed routines three times.
- `clear_memory_and_assign`: Zeros out a block of memory and performs a series of assignments to a global variable.
- `reset_entries_to_all_ones`: Writes 255 into four globals and sets six 32-bit entries to all ones.
- `setup_blit_after_helper_calls`: Calls three unnamed routines, waits for the blitter, then sets its mask and control values with a pointer and size.
- `round_position_to_grid_state`: Calls two unnamed routines, then branches on a state value.
- `clear_twenty_globals`: Sets twenty globals to zero in sequence and returns.
- `set_slot_state_for_conditions`: Clears four globals, then checks three slots.
- `step_countdown_and_reload_value`: Calls unnamed routines and resets a set of globals, then counts down a value.
- `update_fixed_point_state_value`: Steps a 32-bit fixed-point value by one unit, then clamps it against fixed limits and adjusts it by fixed amounts depending on sign and range.
- `step_state_by_two_clamped`: Adds two to a state value, or subtracts two when the value is zero, and clamps it at 150 on one path.
- `step_fixed_point_downward`: Subtracts a step of 12288 or 24576, chosen by the state, from a fixed-point value.
- `step_fixed_point_upward`: Adds a step of 12288 or 24576, chosen by the state, to a fixed-point value.
- `update_state_by_flag_count`: Chooses one of several updates from a state value and a flag.
- `compute_clamped_high_word_step`: Adds 40960 to a fixed-point value and keeps its high word clamped to 15.
- `select_state_from_mask_bits`: Checks bits 10 to 12 of a global and calls an unnamed routine when they match one of two patterns.
- `move_value_toward_target`: Moves a stored value toward a target in steps no larger than eight, clearing it when the target is out of range.
- `update_mode_state_with_clamp`: Calls unnamed routines, then branches on a state value.
- `clamp_high_word_step_three`: Adds 5462 to a value and clamps its high word to plus three on the positive side and minus fifteen on the negative side.
- `clamp_high_word_step_two`: Adds 3280 to a value and clamps its high word to plus two on the positive side and minus fifteen on the negative side.
- `run_helpers_and_clear_flag`: Calls six unnamed routines in sequence and updates a few globals.
- `approach_target_with_halved_step`: Moves a stored value toward a target.
- `store_two_parameters_after_helper`: Calls an unnamed routine, then stores two input values into two globals.
- `store_word_to_global`: Stores a 16-bit value from a register pair into a global variable.
- `reflect_value_at_lower_bound`: Subtracts an input from a value and, when the result is negative within a small range, adds back twice the input to reflect it.
- `reflect_value_at_upper_bound`: Adds an input to a value and, when the result is nonnegative and past a bound, subtracts twice the input to reflect it.
- `bounce_value_at_negative_limit`: Subtracts an input from a value and, when the result goes below a small negative limit, reflects it by adding the input back.
- `bounce_value_at_positive_limit`: Adds an input to a value and, when the result is nonnegative and passes a limit of eight, subtracts the input back and clamps it at eight.
- `damp_velocity_by_capped_step`: Chooses a threshold of 12288 or 8192 from a flag and decays a fixed-point value toward zero in steps capped at 15.
- `decay_value_with_helper_threshold`: Chooses a threshold of 12288 or 8192 from a flag and decays a fixed-point value toward zero in steps capped at 15.
- `nudge_signed_value_toward_zero`: Moves a signed value toward zero in steps of at most 15.
- `add_clamped_step_to_global`: Clamps an input value to a range of minus 15 to 15 and adds it to a global variable.
- `check_threshold_and_call_helper`: When two flags are clear, compares a fixed-point value with a threshold of 12288 or 12287 and calls an unnamed routine when it crosses.
- `adjust_velocity_toward_target`: When a state equals one, it takes the difference between two values and adjusts a fixed-point value by up to four steps of 8192.
- `stub_check_without_effect`: Compares two values and returns without changing anything, so its only visible effect is the comparison itself.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 1 into d6, then returns.
- `lookup_table_entry_for_value`: Searches up to 32 word pairs for one whose first word matches a key, then updates a byte in a second table from the matching pair's flag.
- `increment_indexed_byte_counter`: Sets a global to 255, calls an unnamed routine, and increments the byte in a table at the offset given by the global.
- `steer_state_within_band`: When the distance between two positions is under 305 units, adjusts a state value from its low bits and clamps it against a band offset by 288.
- `update_distance_limited_state`: Computes the distance between two positions.
- `follow_table_chain_twice`: Applies a table lookup to a global value twice in a row.
- `run_entry_list_state_machine`: Calls unnamed routines and steps a small state counter, then walks a list of entries in steps of ten words while each entry's sign bit is clear, calling an unnamed routine for each.
- `select_fixed_point_reset_values`: When a state is zero and negative, selects one of several reset values for a set of globals based on a counter.
- `select_reset_values_with_offset`: When a state is zero and negative, selects one of several reset values for a set of globals based on a counter.
- `reset_counter_with_fixed_value`: Sets a state to 172 when it is zero, then sets a fixed-point value and decrements a counter.
- `set_state_seven_defaults`: Sets a flag value to 255 when a global is nonzero.
- `set_fixed_point_negative_default`: Sets a fixed-point value to one of two negative defaults depending on a flag, and clears a second global.
- `set_fixed_point_minus_six`: Sets a fixed-point value to minus six whole units and clears a second global.
- `update_global_state`: Modifies a global variable based on its current value through a series of conditional checks and increments.
- `select_state_with_helper_calls`: Chooses one of several branches from a state value and calls unnamed routines in each branch.
- `walk_linked_index_table`: Decrements an index and follows a chain of entries in a table while the next value is negative.
- `lookup_tile_attribute_nibble`: Looks up a tile index from a 16-unit grid position using a table, then follows indirect entries while flag bits are set.
- `lookup_adjacent_tile_word`: Looks up a tile index from a grid position using a table, then follows neighbouring entries whose top bits are set.
- `approach_zero_with_unit_floor`: Moves a stored value toward a target in steps.
- `step_value_by_input_magnitude`: Returns when the input is 22 or more.
- `add_input_to_global`: Adds a 16-bit input value to a global variable and returns.
- `subtract_input_with_halved_limit`: Subtracts a value from a global variable, choosing between the input and half of the input depending on a comparison with a second value.
- `subtract_input_from_global`: Subtracts a 16-bit input value from a global variable and returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d6, then returns.
- `return_empty_routine`: The routine is empty and performs no operations before returning.
- `nop_padding_sequence`: The routine is empty and performs no operations.
- `test_bit_and_return_status`: Checks specific bits within a word and returns a non-zero value if the condition is met.
- `invert_input_and_mask_bits`: Inverts a 16-bit input and keeps only bits two and three of the result.
- `mirror_value_when_flag_set`: Calls an unnamed routine, then when an input has any of several flag bits set and a second unnamed routine returns nonzero, replaces a global with minus three minus its value.
- `mirror_value_about_three`: Calls an unnamed routine, then when an input has any of several flag bits set and a second unnamed routine returns nonzero, replaces a global with three minus its value.
- `update_state_from_flag_bits`: Tests flag bits on a status word.
- `check_record_mask_match`: Compares a flag word with a mask pattern and returns one when it matches.
- `scan_records_counting_matches`: Scans up to 32 records and counts those whose type byte falls outside a small excluded range.
- `scale_record_byte_by_64`: Returns the byte at offset one of a record multiplied by 64.
- `clear_global_variable`: sets a global variable to zero before encountering invalid data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compute_mirrored_sprite_span`: Optionally mirrors a horizontal position by flipping a bit and subtracting it from 320, and calls an unnamed routine for the mirrored case.
- `compute_record_span_and_offsets`: Reads two byte fields from a record, multiplies them into a span, and writes a position offset into a structure.
- `find_entries_inside_screen_window`: Scans a list of entries that ends at a negative marker and collects those whose positions fall inside a window.
- `position_record_from_anchor`: Subtracts offsets derived from two fields of a source record from the position of a destination record, using 16 as a base offset.
- `allocate_entry_slot_with_values`: Scans 32 entries for one whose first byte has its sign bit set.
- `allocate_entry_slot_with_pointer`: Scans 32 entries for one whose first byte has its sign bit set, then initializes it from three input values, a table byte and a pointer argument.
- `offset_record_relative_to_source`: Subtracts offsets derived from two fields of a source record from the position of a destination record.
- `update_entries_by_type_codes`: Loops over 32 entries and dispatches each nonzero entry by its type word, using ranges of that word to choose between unnamed routines.
- `initialize_entry_state_by_flags`: Chooses default state bytes for an entry from its flag word and adjusts its position by half of a field.
- `set_entry_type_sub_codes`: Chooses a type word and a sub-code from the entry's flag bits, then writes that sub-code into each of four fields that are already nonzero.
- `set_default_size_from_field`: Sets a global to 32, then when a field is nonzero sets it to 16 and shifts a second byte left by four.
- `update_entry_flags_and_timers`: Calls an unnamed routine, then checks flag bits on an input and on the entry.
- `check_entry_proximity_and_trigger`: Compares an entry's field with a global value and, when it lies within a window, sets a timer byte to 10.
- `write_frame_bytes_for_entry`: For an entry of one kind, clears two table words and writes a set of frame bytes at positions derived from the entry's position.
- `update_frame_table_for_entry`: Clears two table words for an entry of one kind, writes a set of frame bytes at positions taken from its position, then calls two unnamed routines to update a second table.
- `advance_entry_and_check_range`: Advances an entry by its speed field, decrementing the speed on odd positions, and calls an unnamed routine to check its range.
- `release_entry_when_off_screen`: Checks whether an entry's linked record is still inside a window of 304 by 208 units around a reference position.
- `dispatch_entry_by_type_byte`: Dispatches on a type byte of an entry.
- `set_entry_byte_after_helper`: Calls an unnamed routine, then stores the input byte into a fixed byte field of the entry.
- `allocate_entry_with_fields`: Finds the first empty slot in a list of 32 entries, where an empty entry has a zero first word, and fills it from its inputs.
- `nop_placeholder_routine`: This routine is empty and performs no operations.
- `flush_entry_values_to_links`: Scans 32 entries and, for each nonzero first word, copies that word into a field of a linked record and clears the entry while decrementing a count.
- `advance_entry_animation_frame`: Adds a table-driven offset to an entry's position and decrements its frame counter.
- `dispatch_entry_handler_by_flags`: Checks an entry's flag word and, when a set of conditions holds, sets a marker flag.
- `dispatch_entry_handler_alt`: Checks an entry's flag word and, when conditions hold, sets a marker flag.
- `clear_high_bit_toggle_low`: Clears the top bit of a byte field and then toggles its lowest bit.
- `place_entry_within_tile_bounds`: Computes a table address from an input index scaled by 128 plus a base offset, then adjusts an entry's coordinates against 16-unit tile bounds.
- `place_entry_and_adjust_offsets`: Computes an entry's placement relative to a 16-unit tile grid and the input index.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_zero_after_optional_helper`: Calls an unnamed routine when a global is zero, then returns zero.
- `service_interrupt_request_bits`: Checks four bits of an interrupt status word in turn.
- `write_entry_position_record`: Decrements a counter in an entry, then writes a position computed from an offset and a base into a record, copies a field into the output, or stores a fixed constant and sets a flag word to one.
- `copy_draw_state_into_record`: Calls unnamed routines and copies a sprite's position and frame globals into a record, clamping a computed difference at zero.
- `latch_channel_state_to_hardware`: Stores a clamped difference of two channel fields into a record, and when a pending flag is set it clears that flag and writes several channel words to the custom chip registers.
- `run_step_sequence_with_countdown`: Calls a three-step routine in sequence, then, when a global flag is set and a countdown byte reaches zero after decrementing, calls the routine once more.
- `countdown_then_invoke_handler`: Checks a record flag and, when a handler pointer is set, decrements a countdown byte in the record and calls the handler once it reaches zero.
- `advance_three_phase_level`: Steps a 16-bit level through three phases using increments and limits read from a seven-byte table, and it moves to the next phase when the level crosses the phase limit.
- `run_four_channel_sequencer`: Loops over four channel records of 74 words each, reading command bytes from a sequence pointer, where the escape byte sets a stored value and any other byte ends the read.
- `enter_command_interpreter_stub`: A three-instruction entry that jumps into the command interpreter body, which reads sequence bytes and programs channel state.
- `stub_bad_instruction_data`: Two-instruction fragment in a gap that the decompiler decoded as code.
- `start_command_stream_from_pointer`: Loads a sequence pointer from the channel record and runs the same command interpreter body, with the pointer stored back into the record first.
- `scale_byte_then_run_interpreter`: Stores a sequence byte multiplied by a parameter word into a channel field, then enters the command interpreter body.
- `store_divided_period_then_interpret`: Divides a fixed constant by a sequence byte and stores the quotient in a channel word, then enters the command interpreter body.
- `sum_byte_run_into_field`: Sums a counted run of sequence bytes, scales the total by a parameter word and stores it in a channel field, then enters the command interpreter body.
- `copy_fifteen_word_table_entry`: Clears the low two flag bits in a channel record, copies a fifteen-word entry selected by a sequence byte from a fixed table into the record, and zeroes several accumulators.
- `enter_interpreter_from_gap`: A two-instruction fragment in a gap that the decompiler decoded as code.
- `load_fourteen_byte_table_entry`: Selects a fourteen-byte entry from a fixed table by a sequence byte, copies three words and two longs of it into the channel record, and enters the command interpreter body.
- `start_stream_from_pointer_table`: Loads a sequence pointer from a table indexed by a sequence byte and enters the command interpreter at that pointer.
- `copy_three_word_pairs_strided`: Copies two word fields from one record to another and steps the record pointer forward by one word, repeating this three times.
- `copy_two_word_pairs_strided`: Copies two word fields between offsets of a record and steps the pointer by one word, repeating this twice.
- `stub_undecompiled_routine`: The decompiler timed out on this twelve-instruction routine, so no pseudo-code is available.
