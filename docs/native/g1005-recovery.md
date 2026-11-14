# Game 1005 (turn it (1989)(kingsoft)(m3)[cr best]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01005). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 45 raw-word lines |
| Functions named and described | pass, 130 of 130 named (0 author symbols), 130 of 130 described (1 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 19968 bytes, 119 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4004 of 4005 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16438 |
| code | 15206 |
| data | 791650 |
| padding | 2 |
| unused | 77824 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Turn It (1989)(Kingsoft)(M3)[cr Best].adf:bootblock | 1 | 143 | 9 |
| logged disk load from Turn It (1989)(Kingsoft)(M3)[cr Best].adf boot-block load | 1 | 79 | 2 |
| Turn It (1989)(Kingsoft)(M3)[cr Best].adf bytes 5632-7680 (code never loaded by a reference run) | 1 | 181 | 10 |
| Turn It (1989)(Kingsoft)(M3)[cr Best].adf bytes 123904-138752 (code never loaded by a reference run) | 1 | 3059 | 95 |
| Turn It (1989)(Kingsoft)(M3)[cr Best].adf bytes 585728-588800 (code never loaded by a reference run) | 1 | 543 | 14 |

## Cases handled

- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: custom-format (non-DOS) disk, 1605 non-empty blocks: matched against track-loaded segments
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: the boot block reads 16896 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: bytes 5632-7680 look like code no run loaded: recovered position-independently (load address unknown)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: bytes 123904-138752 look like code no run loaded: recovered position-independently (load address unknown)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: bytes 585728-588800 look like code no run loaded: recovered position-independently (load address unknown)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: 4 logged loads cover 36864 bytes of the image
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 274-766 (bytes 140288-392704) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.45)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 772-980 (bytes 395264-502272) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.56)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 990-1126 (bytes 506880-577024) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.26)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1153-1164 (bytes 590336-596480) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.54)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1167-1178 (bytes 597504-603648) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.48)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1180-1192 (bytes 604160-610816) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.12)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1194-1206 (bytes 611328-617984) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.00)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1208-1220 (bytes 618496-625152) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.91)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1222-1234 (bytes 625664-632320) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.74)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1236-1248 (bytes 632832-639488) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.59)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1250-1262 (bytes 640000-646656) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.38)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1264-1355 (bytes 647168-694272) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.92)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1357-1369 (bytes 694784-701440) outside the filesystem hold data, not code (opcode density 0.003, entropy 2.55)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1371-1383 (bytes 701952-708608) outside the filesystem hold data, not code (opcode density 0.003, entropy 2.45)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1385-1388 (bytes 709120-711168) outside the filesystem hold data, not code (opcode density 0.012, entropy 2.08)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1390-1397 (bytes 711680-715776) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.29)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1399-1402 (bytes 716288-718336) outside the filesystem hold data, not code (opcode density 0.012, entropy 1.99)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1404-1506 (bytes 718848-771584) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.17)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1508-1639 (bytes 772096-839680) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.37)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1651-1655 (bytes 845312-847872) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.07)
- turn it (1989)(kingsoft)(m3)[cr best]/Turn It (1989)(Kingsoft)(M3)[cr Best].adf: blocks 1672-1672 (bytes 856064-856576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- 2 packed track region(s), 111616 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `unpack_payload_after_hardware_setup` (bootblock): Writes boot-time values to the interrupt and DMA control registers, then runs a bit-driven unpacker. Kind: bitstream decompression/unpacking. Pass path: The loop continues until puVar8 <= iVar7 and uVar1 == 0, successfully populating the destination memory with the unpacked payload..
- `wait_until_counter_clears` (Turn It (1989)(Kingsoft)(M3)[cr Best].adf bytes 5632-7680 (code never loaded by a reference run)): Calls helper routines in a loop until a returned counter reaches zero, then calls one more helper and returns the result pair. Kind: disk hardware/sync check. Pass path: the loop continues or the code proceeds to the next instruction after the call.
- `run_startup_then_main_loop` (Turn It (1989)(Kingsoft)(M3)[cr Best].adf bytes 123904-138752 (code never loaded by a reference run)): Writes setup values to the copper, DMA and interrupt control registers, copies a table block, and starts the copper list. Kind: loader/sabotage path. Pass path: u3__sub_0AFC() and subsequent initialization calls.
- `poll_helper_until_zero` (Turn It (1989)(Kingsoft)(M3)[cr Best].adf bytes 123904-138752 (code never loaded by a reference run)): Calls helper routines in a loop, choosing the helper from a flag, until a returned value is zero, then calls one more helper. Kind: disk hardware/sync check. Pass path: the branch where unaff_D2w == 0 (u3__sub_2288 is called).

## Functions


### Turn It (1989)(Kingsoft)(M3)[cr Best].adf:bootblock

- `unpack_payload_after_hardware_setup`: Writes boot-time values to the interrupt and DMA control registers, then runs a bit-driven unpacker.
- `decode_mfm_sector_into_buffer`: Programs the disk controller with a sync pattern and transfer setup, waits for the read to finish, and scans the raw track for the sync word.
- `seek_drive_to_track`: Sets or clears one bit of a drive control byte from the lowest bit of the requested value, then steps the head repeatedly until a stored track counter matches the target, stepping in the direction of the difference.
- `wait_drive_reset_counter`: Waits while a busy flag is set, clears a track counter, sets one drive control bit, and then runs a short delay loop.
- `pulse_step_line_decrement_track`: Pulses the step line in a drive control byte by setting one bit, clearing another, and setting it again.
- `pulse_step_line_increment_track`: Pulses the step line in a drive control byte by clearing one bit, clearing another, and setting a third.
- `write_drive_control_pattern_a`: Writes one control pattern to a drive control byte and then a second pattern to the same byte.
- `write_drive_control_pattern_b`: Writes two fixed control patterns in sequence to a drive control byte.
- `unpack_payload_into_memory`: Sets up a DMA transfer block through an Amiga OS library call, then runs a bit-driven unpacker.

### logged disk load from Turn It (1989)(Kingsoft)(M3)[cr Best].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Turn It (1989)(Kingsoft)(M3)[cr Best].adf bytes 5632-7680 (code never loaded by a reference run)

- `run_poll_loop_with_helpers`: Writes constant values to a state word, then loops calling helper routines and clearing a memory flag, with an inner wait on that flag.
- `wait_until_counter_clears`: Calls helper routines in a loop until a returned counter reaches zero, then calls one more helper and returns the result pair.
- `update_bitmapped_data`: Waits for bit 5 of the CIAA_PRA register to clear, then searches a table for a specific 16-bit constant and performs bitwise operations to copy and transform data from one memory location to another using a loop.
- `adjust_track_value`: Sets or clears bit 2 of a byte at a pointer in a6 based on bit 0 of d0, then compares a 16-bit value from a4 with a bit-shifted version of d0 to decide whether to call another routine to increment or decrement a value.
- `wait_drive_reset_counter_2`: Waits while a busy flag is set, clears a track counter, sets one drive control bit, and then runs a short delay loop.
- `pulse_step_line_decrement_track_2`: Pulses the step line in a drive control byte by setting one bit, clearing another, and setting it again.
- `pulse_step_line_increment_track_2`: Pulses the step line in a drive control byte by clearing one bit, clearing another, and setting a third.
- `write_drive_control_pattern_a_2`: Writes one control pattern to a drive control byte and then a second pattern to the same byte.
- `write_drive_control_pattern_b_2`: Writes two fixed control patterns in sequence to a drive control byte.
- `build_copper_list_pointers`: Fills a table with pointers into a copper list, stepping 408 bytes per entry, then writes two fixed control words for the copper and DMA setup.

### Turn It (1989)(Kingsoft)(M3)[cr Best].adf bytes 123904-138752 (code never loaded by a reference run)

- `run_startup_then_main_loop`: Writes setup values to the copper, DMA and interrupt control registers, copies a table block, and starts the copper list.
- `swap_hundred_longword_blocks`: Swaps 100 longwords between two buffers one at a time, then calls a helper.
- `select_mode_and_start_copper`: Sets a mode value to one of two fixed values depending on a flag, then calls the routine that starts the copper list.
- `rotate_and_complement_state_byte`: When a status bit is set, rotates a state byte by one bit and clears a flag.
- `scan_grid_for_matching_entry`: Scans a grid of table entries row by row, 20 columns by 180 rows, comparing the high byte of each entry against a reference value.
- `store_coordinates_then_call_helpers`: Stores a pair of coordinate values and a value into a state record and calls several helpers when a sign flag is clear.
- `update_position_when_changed`: If a sign check passes and a masked high byte matches a reference, it compares a coordinate pair against a stored pair.
- `adjust_counter_by_code_byte`: When the coordinate pair matches one of two stored pairs, it reads the high byte of a table word and adjusts a counter.
- `call_helper_then_clear_state`: Calls a helper routine, then clears two state bytes.
- `call_then_reset_state_pair`: Calls one unnamed routine and then clears two global variables, with the same pattern shared by four callers.
- `wait_while_timer_nonzero`: Calls two helper routines repeatedly while a 32-bit counter stored in memory stays nonzero, then returns.
- `wait_while_countdown_nonzero`: Calls two helper routines repeatedly while a second 32-bit countdown stored in memory stays nonzero, then returns.
- `call_helpers_until_sentinel`: Walks a list of three-byte records until it reaches a sentinel value, calling two helper routines for each record.
- `advance_mixed_radix_clock`: Adds ten units to a counter split into ones, tens, sixties and six-hundreds, carrying into each higher field and storing the updated fields.
- `copy_longword_row_from_table`: Copies a row of 100 longwords, selected by a row index with 400 bytes per row, into a fixed buffer.
- `scale_time_records_by_third`: Walks a list of four-field time records until a sentinel.
- `call_seven_helpers_clear_state`: Calls seven helper routines in sequence, then clears a state byte.
- `clear_state_and_call_helpers`: Clears several state bytes and calls two helper routines.
- `call_helpers_around_state_clear`: Calls one helper routine, clears three state bytes, then calls two more helper routines.
- `set_offset_value_by_mode`: When a selector byte is zero, sets a state byte from a table value adjusted by a mode value: subtracts 400 with clamping at zero for mode 0, keeps the value for mode 1, and adds 600 for other modes.
- `forward_to_helper_routine`: Calls one helper routine and returns.
- `copy_tile_block_into_buffer`: Copies a block of tile rows from a source image selected by an index into a destination buffer.
- `advance_state_by_ten_steps`: Branches on a state byte with values from zero to three, calling helper routines and adding 10 to the state byte once or twice depending on whether it is nonzero.
- `compare_and_store_larger_entries`: Compares a value against two table entries and stores each entry that is larger than it into a state byte, so the later entry wins.
- `copy_strided_pixel_rows`: Copies strips of eight rows, three bytes per row, from a source to a destination with a different stride, four strips at a time.
- `copy_table_column_into_buffer`: Calls three helper routines, then copies a column of 12 bytes from a table, selected by a scaled index, into five spaced rows of a destination grid.
- `copy_table_column_into_buffer_2`: Calls three helper routines, then copies a column of 12 bytes from a table, selected by a scaled index, into five spaced rows of a destination grid.
- `copy_block_to_third_area`: Calls three unnamed routines, then copies twelve byte values from a table row scaled by a stored index into five destinations spaced sixteen rows apart, advancing 40 bytes per row, at a further target area.
- `copy_indexed_table_column`: Copies a column of 12 bytes from a table, offset by an index from a register, into five spaced rows of a destination buffer.
- `call_four_helpers_in_sequence`: Calls four helper routines in sequence and returns.
- `copy_column_to_rows`: Copies a column of 12 bytes from a source pointer into five spaced rows of a destination grid.
- `build_cell_run_list`: Builds a list of four-byte cell records along a straight run between two coordinate pairs, ending the list with a sentinel.
- `build_cell_path_list`: Walks from one grid cell toward another one step at a time, writing a four-byte record for each step and ending with a sentinel.
- `emit_column_entries_until_bound`: Runs a counter from a start value toward a bound of twenty or minus one, writing byte pairs and a marker word for each step while calling four unnamed routines.
- `advance_index_and_clamp_value`: Advances a state byte by two, then, depending on a mode byte, stores a table value into it when a bound check passes and calls a helper.
- `classify_entry_against_record`: Copies a header and a list of two-byte entries into an output list until a terminator, with a marker byte between entries.
- `blit_cell_rows_from_list`: Combines four rows of 20 cells into a temporary buffer, then programs the blitter control registers, size and pointers and waits for the blitter-busy bit to clear.
- `spin_delay_counter`: Counts a register down to zero as a short delay.
- `read_grid_cell_flag`: Reads one cell byte from a grid and returns without changing any state.
- `check_grid_cell_usable`: Looks up a grid cell by row and column and tests it against a range of marker values.
- `check_cell_marker_values_alt`: Checks whether a column index is below nine and a table byte is non-zero and not one of three marker values, using register parameters, but every branch returns with no recorded effect.
- `copy_twenty_words_to_column`: Copies 20 words from a source into a column of a two-dimensional buffer, stepping forty bytes per word.
- `wait_status_bit_with_delays`: Runs a fixed countdown, waits until a status bit is set, then runs a second countdown.
- `call_helper_over_grid_cells`: Calls a helper once for each cell of an 18 by 8 grid, then clears two state bytes and calls a final helper.
- `compare_grid_cells_between_buffers`: Compares the low byte of a grid cell in two buffers.
- `call_helper_countdown_loop`: Calls a helper repeatedly while a 16-bit counter in a register stays non-negative, counting it down on each pass.
- `call_helper_countdown_loop_2`: Calls a helper in a loop that counts a 16-bit value down until it goes negative.
- `copy_grid_rows_after_wait`: Waits on a state byte for a condition, then copies four-word strips of grid columns from a source into a destination buffer, row by row.
- `call_helper_over_cell_grid`: Calls a helper once for each cell of a 20 by 10 grid.
- `blit_four_rows_per_plane`: Programs the blitter control, size and pointer registers and runs four row copies, waiting for the blitter-busy bit to clear before each one.
- `call_helper_pairs_three_times`: Calls a pair of helper routines three times in a row.
- `calculate_table_index`: Calculates an index by shifting a value in d6 left by one, adding the result of multiplying d7 by forty, and then fetches a word from a table using a pointer in a5 and this calculated offset, finally masking the result to keep only the lowest byte.
- `delegate_to_unnamed_routine`: Passes control to one unnamed routine and returns, with no other work shown in the body.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `set_fixed_state_words`: Writes a fixed word value to a state location, then writes an all-ones word to it.
- `call_five_helpers_in_sequence`: Calls five helper routines in sequence and returns.
- `set_two_words_to_constant`: Writes one constant to two state words, calling a helper routine after each write, and returns.
- `set_paired_locations_to_680`: Writes the value 680 to two memory locations, calling one unnamed routine after each write.
- `set_second_pair_to_342`: Writes the value 342 to a second pair of memory locations, calling one unnamed routine after each write.
- `call_two_helpers_in_sequence`: Calls two helper routines in sequence and returns.
- `call_two_helpers_in_sequence_2`: Calls two helper routines in sequence and returns.
- `latch_masked_value_when_set`: When a field is nonzero and a rotated word overlaps the status byte, copies the word into a state location and clears the field.
- `write_custom_register_block`: Writes a set of values to custom chip registers at a base offset, including a DMA control value, with short delays between writes.
- `fill_pointer_tables_into_records`: Fills several groups of record pointers, each a stepped sequence of addresses with its own spacing, then sets two fixed state words.
- `draw_text_string_glyphs`: Copies the glyph bitmap for each character of a zero-terminated string into a destination plane, mapping space to a fixed glyph.
- `decrement_packed_digit_fields`: Decrements the packed digit fields of each word in a list, one step for each nonzero digit, and repeats the pass until no field changes.
- `normalize_packed_digits_into_list`: Copies a list of packed digit words, then repeatedly takes one unit from each nonzero digit field and adds it to a matching counter list until no digits remain.
- `wait_for_raster_frames`: Waits on the vertical position register for a fixed raster condition, repeating the wait a counted number of times.
- `update_mouse_position_from_counter`: Compares the current mouse counter with the previous sample on each axis, adds or subtracts the change within clamp limits, then stores a scaled row value.
- `write_record_for_position`: Selects one of two four-byte table entries by a threshold test, then writes four bytes built from the input coordinates and a flag into a record.
- `store_four_byte_record`: Writes four bytes taken from register inputs into a record at an address register.
- `clear_long_buffer`: Zeroes 14,400 longwords starting at a fixed buffer address.
- `skip_counted_strings_then_call`: When a counter is a multiple of ten, skips a number of NUL-terminated strings in a buffer equal to the counter divided by ten, then calls a routine.
- `copy_longs_from_table`: Copies 12,000 longwords from a source table into a fixed buffer, then waits for a busy flag to clear with a bounded countdown and sets a state word.
- `copy_memory_block`: Sets a global variable, copies a block of twelve thousand bytes from one memory location to another, and performs a busy-wait loop on a bit in a global variable.
- `poll_helper_until_zero`: Calls helper routines in a loop, choosing the helper from a flag, until a returned value is zero, then calls one more helper.
- `read_and_decode_disk_track`: Selects a track buffer, programs the disk controller with a sync word and DMA values, waits for the read, and decodes the MFM data into longwords.
- `update_track_step`: Modifies bit 2 of a byte via a pointer in a5 based on the state of bit 0 in d0, then calls another routine to increment or decrement the 16-bit value pointed to by a4 until it matches a shifted version of d0.
- `wait_drive_reset_counter_2_2`: Waits while a busy flag is set, clears a track counter, sets one drive control bit, and then runs a short delay loop.
- `pulse_step_line_decrement_track_2_2`: Pulses the step line in a drive control byte by setting one bit, clearing another, and setting it again.
- `pulse_step_line_increment_track_2_2`: Pulses the step line in a drive control byte by clearing one bit, clearing another, and setting a third.
- `write_drive_control_pattern_a_2_2`: Writes one control pattern to a drive control byte and then a second pattern to the same byte.
- `write_drive_control_pattern_b_2_2`: Writes two fixed control patterns in sequence to a drive control byte.
- `encode_and_write_disk_track`: Copies 144 bytes into a track buffer, fills the rest of the track with gap longwords and sync words across 11 sectors, then programs the disk controller for a DMA write and waits for it to finish.
- `encode_mfm_longword`: Computes an MFM-style clock pattern for a longword from its data and the previous bits, masks the top bit according to the low bit of the preceding byte, and stores the result.
- `encode_mfm_longword_2`: Calls a helper routine, then computes the same MFM-style clock pattern for a longword and stores the result.
- `build_audio_sample_pointers`: Scans a byte table for the next entry above a threshold, then fills 30 pointer slots with sample addresses stepped by a per-entry length, and clears a block of channel pointer words.
- `advance_audio_channel_envelopes`: Advances a sample counter and, on every sixth call, selects a sample address from a table and resets channel state.
- `update_audio_channel_state`: Updates one channel's volume or period from a mode nibble, adding or subtracting a step with clamping, or looks up a value in a table.
- `parse_sound_record_into_channel`: Reads a four-byte sound record, sets the sample pointer and period for the channel from tables when the instrument nibble is nonzero, and applies an effect chosen by the low nibble of the second byte.
- `copy_sample_block_fifth_tick`: Counts ticks, and on every fifth tick copies four rows of 17 table entries from an indexed block into a destination with a spaced stride.
- `run_text_entry_loop`: Copies a 12,000-longword block into a buffer, then loops on input state, drawing characters into a text line at a cursor and handling a delete code, until a finish code is read.
- `insert_ranked_entry`: Searches a table of six ranked records for the first entry at or below the new value, shifts the lower records down one slot, and writes the value and a blank 20-character name into the freed slot.
- `draw_six_ranked_rows`: Loops over six records, calling helper routines for each row and for each of 20 character cells in the row.
- `copy_glyph_block_to_plane`: Copies four rows of ten bytes from a source block selected by an index into a destination plane, with a spaced row step on the destination side.

### Turn It (1989)(Kingsoft)(M3)[cr Best].adf bytes 585728-588800 (code never loaded by a reference run)

- `initialise_copper_and_clear_audio`: Sets up copper and interrupt control values and pointers, calls the routine that clears the audio DMA channels, and waits on a control flag.
- `build_audio_entries_second_table`: Scans a byte table for the first entry above a running value, then fills thirty entries of a global table from a second table using the matched index; this copy reads from a different table.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `advance_audio_sequence_second`: Advances a frame counter and, every sixth call, reloads the audio pointer from a different table, waits briefly, and writes per-channel values from stored entries.
- `process_audio_channel_effects_second`: Applies the same per-channel effect steps as its neighbour to four audio channels, using the low nibble of a control byte, clamping stepped period values, and writing the channel's length, pointer and period registers.
- `parse_sound_record_into_channel_2`: Reads a four-byte sound record, sets the sample pointer and period for the channel from tables when the instrument nibble is nonzero, and applies an effect chosen by the low nibble of the second byte.
- `clear_indexed_byte_grid`: Clears a grid of single bytes, four rows by four columns, inside a block chosen by an index.
- `blit_clear_four_rows`: Programs the blitter for four rows at a time, waiting for the busy bit to clear before each one.
- `blit_copy_four_rows`: Programs the blitter for four rows at a time, waiting for the busy bit before each one.
- `decrement_digit_words_until_clear`: Repeatedly subtracts one from the nibble digits of each word in a row, calling an unnamed routine after each pass, until a pass changes no digit.
- `copy_row_then_transfer_digits`: Copies a row of words into a buffer, then repeatedly moves each word's digit nibbles into a parallel array one unit at a time, calling one unnamed routine per pass until no digit moves.
- `wait_raster_ten_passes`: Waits for a vertical beam position field to reach a fixed value ten times in succession, then returns.
- `build_copper_bitplane_pointers`: Writes four pointers into a table, each 6,440 bytes after the last, then writes two fixed control words.
- `next_pseudo_random_mod`: Advances a 32-bit shift-register random state by one step, xoring in a feedback constant when the top bit shifts out or the result is zero.
