# Game 323 (Turn It): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00323). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (912272 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 54 raw-word lines |
| Functions named and described | pass, 154 of 154 named (0 author symbols), 154 of 154 described (2 by rule, 28 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 585 of 585 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 792000 bytes, 126 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4999 of 5073 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 77900 |
| code | 16814 |
| data | 794158 |
| padding | 1896 |
| unused | 21504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Turn It.adf:BMturnit | 2 | 81 | 4 |
| Turn It.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 646 | 23 |
| Turn It.adf:3 (raw code file, load address unknown) | 1 | 3100 | 100 |
| Turn It.adf:4 (raw code file, load address unknown) | 1 | 543 | 14 |
| Turn It.adf:2#unpacked (raw code file, load address unknown) | 1 | 693 | 12 |

## Cases handled

- Turn It/Turn It.adf:1: BK: ByteKiller-packed, unpacked with ancient (15520 -> 39280 bytes)
- Turn It/Turn It.adf:2: BK: ByteKiller-packed, unpacked with ancient (89952 -> 150000 bytes)
- runtime segment an address: 509 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Turn It/Turn It.adf:3: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Turn It/Turn It.adf:4: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Turn It/Turn It.adf:2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 792000 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_compressed_data_structure` (runtime segment an address): Parses a data structure by calculating an index from a bitstream, then iterates through nested loops to compare bytes and copy data from a source buffer to a destination buffer. Kind: checksum/integrity trap. Pass path: The loop that performs the memory copy (puVar4 = puVar5) and the subsequent conditional check on in_D1 (if in_D1 != 0) which triggers a secondary routine (u2__sub_2A86) and modifies a global state (DAT_00bfe001) if the data integrity check .
- `initialise_chipset_and_copper` (3 (raw code file, load address unknown)): Initialises interrupt, DMA and copper registers, copies a block of 100 longwords, selects one of three display buffers and calls a copper-start routine plus many unnamed routines. Kind: loader/checksum trap. Pass path: The routine performs a series of hardware/memory initializations followed by a memory copy loop (the `do-while` block) that appears to be moving data from a calculated offset based on `uRam00000100`. The subsequent calls to subroutines like.
- `stub_empty_body_colour_access` (3 (raw code file, load address unknown)): The decompiled body is empty, although the facts record button port, serial port and background colour accesses across six unnamed calls; no behaviour is recovered from the code. Kind: colour-card lookup. Pass path: the instruction immediately following the call (the code continues execution normally).
- `start_disk_read_sync_wait` (3 (raw code file, load address unknown)): Waits for the disk controller to be idle, programs the disk pointer, sync word and read length, then polls for the sync request within a fixed timeout, resetting the transfer when the sync appears and disabling it otherwise. Kind: disk sync/status check loop. Pass path: the loop completes (iVar1 reaches 0) or the status bit check fails to trigger the return, allowing execution to proceed to the next instruction after the call site.
- `start_disk_read_alternate_sync` (3 (raw code file, load address unknown)): Waits for the disk controller to be idle, programs the disk pointer and a different sync word, then polls for the sync request within a fixed timeout, resetting the transfer when it appears and disabling it otherwise. Kind: disk track/sync check. Pass path: tst.l d6; beq.w an address.
- `clear_pattern_start_disk_read` (3 (raw code file, load address unknown)): Waits for the disk controller to be idle, fills a block of longwords with an alternating pattern, programs the disk pointer and sync word, then polls for the sync request and resets the transfer when it appears. Kind: disk pattern/sync initialization. Pass path: the loop completes (iVar1 reaches 0) and the routine proceeds to set disk controller registers for a read operation.
- `step_drive_select_bits` (3 (raw code file, load address unknown)): Clears one port bit, then loops a given number of times toggling a low bit of a second port value and calling an unnamed routine, returning the remaining count. Kind: disk drive selection/bit manipulation check. Pass path: the loop continues (sVar1 >= 0).
- `clear_port_bits_and_call` (3 (raw code file, load address unknown)): Clears bit one and sets bit zero of a port byte held in a global, then calls one unnamed routine. Kind: disk hardware/port bit manipulation check. Pass path: addq.w #an address, an address(a5).
- `set_port_bits_and_call` (3 (raw code file, load address unknown)): Sets the two low bits of a port byte held in a global, then calls one unnamed routine. Kind: disk hardware/sync check. Pass path: subq.w #an address, an address(a5).
- `pulse_port_bit_while_flag` (3 (raw code file, load address unknown)): Sets bit one of a port byte, then repeatedly pulses bit zero by clearing and setting it while bit four stays set, calling an unnamed routine on each pulse. Kind: disk hardware timing/sync check. Pass path: bmi.b an address.
- `set_port_bit_from_argument` (3 (raw code file, load address unknown)): Clears bit two of a port byte held in a global and ORs in a byte passed in a register. Kind: disk hardware status check. Pass path: the branch at an address (bne.w) is NOT taken, meaning the bit test results in zero/false.
- `display_credit_text_screen` (3 (raw code file, load address unknown)): Clears a display region, draws a 12000-longword block into it, then calls several routines; the referenced text includes a 'cracked in 1990 by' line. Kind: sabotage path. Pass path: the branch following the call to sub_0B94 (the instruction after the call is rts, implying the routine is a sub-component of a larger check or a trap).

## Functions


### Turn It.adf:BMturnit

- `start_entry_set_background`: Program entry point that writes the background colour register and calls the bit-reader refill and bit-read routines; the decompiler timed out, so only those calls and the colour write are recovered.
- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Turn It.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_display_and_setup_mouse`: Calls a library function to permit execution, configures custom chipset registers for display and bitplane settings, calls three subroutines to initialize data, and then uses a library call to retrieve a pointer which is used to set the hardware mouse pointer position and enable the mouse via the hardware register.
- `initialize_and_expand_data_table`: Clears a memory area, populates a table with specific bit patterns, and then parses a source byte stream to expand and copy data segments into the cleared area based on offsets found in a secondary table.
- `setup_blt_chain`: Configures the Blitter registers with specific color and size values, then enters a loop that calculates destination addresses from a table and updates Blitter parameters for 39 iterations.
- `update_mouse_coordinates`: Reads the mouse position from a hardware register, calculates a delta, and updates a table of six byte values by incrementing or decrementing them based on the movement and specific boundary conditions.
- `update_sprite_data_and_counters`: Checks hardware status bits for joystick and interrupt availability, then uses a global variable to index into tables to perform multiple loops that copy blocks of seven bytes from various source offsets to a destination table.
- `configure_dma_and_interrupts`: Writes two 16-bit values from a 32-bit pointer into a table at two different offsets, sets the DMA controller registers, modifies the interrupt enable and request registers, and calls setup_dma_descriptors, acknowledge_interrupt, and process_compressed_data_structure.
- `setup_dma_descriptors`: Writes two 16-bit values derived from a 32-bit pointer into a table at two different offsets, then sets the DMA controller registers with a specific address and control word.
- `acknowledge_interrupt`: Writes the value 112 to the interrupt request register and returns from an interrupt.
- `process_compressed_data_structure`: Parses a data structure by calculating an index from a bitstream, then iterates through nested loops to compare bytes and copy data from a source buffer to a destination buffer.
- `convert_to_lowercase`: Checks if a byte in d1 is between 97 and 122 inclusive, and if so, subtracts 32 from it before returning.
- `process_scaled_value_and_update_structure`: Calculates a value by multiplying d0 by 512, performs conditional calls to other routines based on the sign and division results, and updates a 16-bit value at offset 14 of a structure in a6.
- `set_bits_and_loop_on_condition`: Clears a 16-bit word at a6, sets bits 0 and 1 of a byte at a4, and then calls another routine repeatedly while bit 4 of a byte at offset 3841 from a4 is set.
- `adjust_counter_based_on_comparison`: Compares a 16-bit value in d0 with a 16-bit value at a6, then enters a loop that either increments or decrements the value at a6 while calling another routine and toggling bit 1 of a byte at a4 until the values match.
- `toggle_bit_and_call_routine`: Clears bit 0 of a byte at the address in a4, sets bit 0 of the same byte, and then calls another routine.
- `decrement_counter`: Initializes d7 to 49152 and enters a loop that decrements the register until it reaches zero.
- `decrement_counter_loop`: Initializes d7 to 49128 and enters a loop that decrements the register until it reaches zero.
- `wait_for_bit`: Polls bit 5 of a byte at an offset of 3841 from the address in a4 until that bit is cleared.
- `poll_bits_and_write_values`: Writes four 16-bit values to offsets of a structure in a5, then loops until bit 4 of a byte at an offset of 3072 from a pointer in a4 is set, and finally loops until bit 1 of a byte at offset 31 of the structure in a5 is set before returning.
- `unpack_bitstream`: Searches a table for a specific marker, then iterates through a data stream to perform bitwise operations and store the results in a destination buffer.
- `decompress_bitstream`: Processes a bitstream by calling two different routines to read bits and refill a buffer, then writes decoded bytes to a destination memory area.
- `write_decrementing_global_until_wrap_2`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Turn It.adf:3 (raw code file, load address unknown)

- `initialise_chipset_and_copper`: Initialises interrupt, DMA and copper registers, copies a block of 100 longwords, selects one of three display buffers and calls a copper-start routine plus many unnamed routines.
- `swap_two_longword_buffers`: Swaps 100 longwords between the two buffers passed in address registers, then calls one unnamed routine.
- `choose_mode_and_start_copper`: Chooses one of two values from a global flag, then calls the copper-start routine and eleven unnamed routines, with an interrupt-enable register write recorded in the facts.
- `decode_keyboard_serial_byte`: When a keyboard-serial flag bit is set, rotates the received byte, clears the handshake state and stores the inverted code in a global variable.
- `search_grid_for_matching_word`: Scans a grid of 20 columns by 180 rows of words for an entry whose high byte matches a stored key, and on a match sets state values and clears a flag before returning.
- `copy_or_clear_state_pair`: Branches on a condition: in one case copies two stored values into globals, calls several unnamed routines and writes two small constants; otherwise it calls two unnamed routines and clears two globals.
- `match_key_and_update_position`: Compares the high byte of a stored word with a key value, and when they match and a stored pair of values differs from the reference pair, records the pair and calls unnamed routines; otherwise it clears state values.
- `dispatch_key_code_actions`: When a stored pair matches one of two reference pairs, it calls unnamed routines and dispatches on the low byte of a key word; one key code advances a counter by two and others call routines.
- `call_then_reset_state_flags`: Calls one unnamed routine and then clears two global variables.
- `call_then_reset_state_pair`: Calls one unnamed routine and then clears two global variables, with the same pattern shared by four callers.
- `wait_while_counter_nonzero`: Repeatedly calls two unnamed routines while a longword assembled from two memory words is non-zero, returning once it reaches zero.
- `wait_while_second_counter_nonzero`: Repeatedly calls two unnamed routines while a second longword assembled from two memory words is non-zero, returning once it reaches zero.
- `walk_three_byte_records`: Walks a table of three-byte records from a global start address until a terminator byte, calling two unnamed routines for each record.
- `advance_mixed_radix_counter`: Adds ten units to a counter stored as digit fields with weights of one, ten, sixty and six hundred, carrying between fields, then calls one unnamed routine.
- `copy_row_and_load_fields`: Copies 100 longwords from a table row chosen by a global index into a fixed destination, then loads three byte fields from a second table entry selected by the same index.
- `convert_encoded_values`: Iterates through a list of four-byte records, calculating a value from the bytes and updating the record with a new encoded format based on a global sign flag.
- `call_seven_routines_clear_flag`: Calls seven unnamed routines in sequence and then clears one global variable.
- `reset_state_then_call_two`: Clears one global, calls an unnamed routine, clears three more globals, then calls two further unnamed routines.
- `call_clear_then_call_two`: Calls one unnamed routine, clears three global variables, then calls two more unnamed routines.
- `adjust_stored_value_by_setting`: When a setting flag is zero, sets a global from a stored value adjusted by an offset chosen by a second setting, subtracting 400 with a floor at zero or adding 600 for other values, then calls an unnamed routine; the facts record a button port register access.
- `forward_single_unnamed_call`: Makes one unnamed call and returns; the facts list three callees, but the body shows only the single call, so no further behaviour is recovered.
- `copy_strided_tile_block`: Copies a block of 18 tile rows, each 18 bytes wide, from a source selected by a global index into a destination with a 40-byte row stride, with a larger stride between four sub-blocks.
- `dispatch_small_state_increment`: Dispatches on a small state value and adds ten to a counter, adding another ten when the counter is non-zero, before calling a series of unnamed routines; the gotos are unresolved, so only the adjustment pattern and call counts are recovered.
- `copy_values_above_limit`: If a stored limit is below one stored value, copies that value into a global; repeats the test for a second stored value, then returns.
- `copy_strided_three_byte_rows`: Copies a block of four rows by eight entries from a source with a four-byte stride into a destination with a 40-byte row stride, moving three bytes per entry.
- `copy_block_into_five_lines`: Calls three unnamed routines, then copies twelve byte values from a table row scaled by a stored index into five destinations spaced sixteen rows apart, advancing 40 bytes per row.
- `copy_block_to_second_area`: Calls three unnamed routines, then copies twelve byte values from a table row scaled by a stored index into five destinations spaced sixteen rows apart, advancing 40 bytes per row, at a different target area.
- `copy_block_to_third_area`: Calls three unnamed routines, then copies twelve byte values from a table row scaled by a stored index into five destinations spaced sixteen rows apart, advancing 40 bytes per row, at a further target area.
- `copy_param_block_five_lines`: Copies twelve bytes from a source offset derived from a parameter into five destinations spaced sixteen rows apart at 40-byte row strides, with no preceding calls.
- `call_four_routines_in_sequence`: Calls four unnamed routines in sequence and returns.
- `copy_bytes_into_strided_lines`: Copies twelve bytes from a source into five destinations spaced 1680 bytes apart, advancing the source and each destination by one row step per pass.
- `build_adjacent_cell_entries`: Clears three pending markers, and when a stored position lies two or forty bytes from a reference position it sets a flag and returns; otherwise it calls unnamed routines to build a list of four-byte entries for matching coordinates.
- `emit_row_entries_until_end`: Loops over a row counter, storing a pair of bytes and a marker word into a list for each step while calling an unnamed routine that returns a pair of values, and stops when the counter reaches its end value or goes negative.
- `emit_column_entries_until_bound`: Runs a counter from a start value toward a bound of twenty or minus one, writing byte pairs and a marker word for each step while calling four unnamed routines.
- `advance_and_clamp_by_flag`: Advances a global by two and calls an unnamed routine; when a comparison between two stored values holds, copies one of them into the global and calls another routine.
- `copy_record_and_flag_position`: Copies a header record and then a run of two-byte entries terminated by a marker into a destination, padding each entry with a fixed byte; it sets a state value to two or three depending on how the last entry compares with a reference pair.
- `combine_planes_and_blit_rows`: Ors four interleaved rows together into a temporary buffer for each row of a table, then programs the blitter registers and starts blit operations until a terminator byte is reached.
- `busy_wait_countdown`: A pure countdown loop that decrements a register until it reaches zero, then returns, with no other effects.
- `probe_table_byte_no_effect`: Reads one byte from a table indexed by two registers, but both branches return the same values, so the read has no recovered effect.
- `check_cell_marker_values`: Checks whether a column index is below nine and a table byte is non-zero and not one of three marker values, but every branch returns with no recorded effect.
- `check_cell_marker_values_alt`: Checks whether a column index is below nine and a table byte is non-zero and not one of three marker values, using register parameters, but every branch returns with no recorded effect.
- `copy_twenty_words_down_column`: Copies twenty words from a source into a destination column, advancing the destination by 40 bytes per word, with the column offset taken from two register parameters; eight unnamed routines call it.
- `wait_status_bit_with_delays`: Counts down a fixed delay, waits until bit six of a status global becomes set, then counts down the same delay again; the facts record button port and interrupt register touches.
- `iterate_grid_calling_routine`: Runs nested loops that call an unnamed routine once for each cell of an eight-by-eighteen grid, then clears two globals and calls one more routine.
- `compare_table_entries_flag`: Sets a flag to 255, then compares two table entries at the same offset; if they differ it calls two unnamed routines, and the flag ends at zero either way.
- `repeat_call_for_count`: Calls one unnamed routine repeatedly while a counter taken from a register, decremented on each pass, remains non-negative.
- `repeat_call_until_result_negative`: Calls one unnamed routine repeatedly, using its returned word as a counter, and keeps looping while the decremented counter is non-negative.
- `copy_rows_into_four_planes`: Performs a short status check, then copies rows of twenty-word source data into four destinations spaced 4800 bytes apart, advancing 40 bytes per row until the row counter wraps.
- `iterate_grid_ten_by_twenty`: Runs nested loops that call an unnamed routine once for each cell of a ten-by-twenty grid.
- `start_blitter_copy_rows`: Writes blitter setup values, then runs five passes, waiting for the blitter to go idle before each pass, starting a copy at the current source and destination and advancing both by 9600 bytes.
- `repeat_two_calls_three_times`: Calls two unnamed routines in each of three passes.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `delegate_to_unnamed_routine`: Passes control to one unnamed routine and returns, with no other work shown in the body.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `write_fixed_copper_words`: Writes a fixed word value and the all-ones word into a pair of globals; the facts record copper list pointer and jump register touches.
- `start_copper_then_run_steps`: Calls the copper-start routine and then four unnamed routines in sequence.
- `set_paired_locations_to_342`: Writes the value 342 to two memory locations, calling one unnamed routine after each write.
- `set_paired_locations_to_680`: Writes the value 680 to two memory locations, calling one unnamed routine after each write.
- `set_second_pair_to_342`: Writes the value 342 to a second pair of memory locations, calling one unnamed routine after each write.
- `call_two_unnamed_routines`: Calls two unnamed routines in sequence and returns.
- `update_pending_request_value`: Checks a pending request field and, if a bitwise condition is met involving a rotated word and a global mask, copies the request value into a target location.
- `service_pending_request_flag`: Checks a pending request field: when it is non-zero and a word rotated by seven bits overlaps a global bit mask, copies the request value into a global and clears the pending field.
- `apply_channel_block_with_delays`: Writes a block of channel parameters to DMA-related hardware registers with short delays between writes, sets a bit in a global, and marks the block as pending.
- `fill_pointer_table_words`: Fills a table of longword pointers split into word halves, writing successive groups of four and five entries that step by fixed byte offsets such as 640 and 9600 bytes.
- `draw_centered_text_string`: Measures a zero-terminated string, centres it in a 40-byte row, and copies each character's glyph block from a font table into the destination, mapping a space to a fixed glyph index.
- `decrement_bcd_digit_words`: For each word in a row, subtracts one from its hundreds, tens and units digit nibbles whenever they are non-zero, records whether any digit changed, and repeats the row pass until nothing changes.
- `transfer_bcd_digits_to_array`: Copies a run of words into a buffer, then repeatedly moves each word's hundreds, tens and units digit nibbles one unit at a time into a parallel array, repeating passes until no digit moves; it also calls one unnamed routine.
- `wait_raster_position_count`: Repeatedly waits until a vertical beam position field reaches a fixed value, once for each count passed in a register, then returns.
- `track_mouse_axes_within_limits`: Reads the mouse movement value and moves two tracked values toward the last reading, clamping each to stored minimum and maximum limits; the second axis follows the same pattern.
- `write_four_byte_header_record`: Chooses one of two destination buffers from a stored index, then writes a four-byte record from two stored words, offsetting the third byte by a size constant and deriving the fourth from a flag bit and a threshold.
- `store_four_byte_record`: Stores four byte values into consecutive memory locations, with the second byte taken from a register shifted right by one.
- `clear_large_longword_block`: Zeroes a block of 14400 consecutive longwords, starting from a fixed address.
- `skip_strings_and_dispatch_indirect`: When a counter is a multiple of ten, skips a number of zero-terminated strings in a table and then makes an indirect call to a target that is not resolved.
- `copy_and_wait_block`: Sets a global variable, copies a block of twelve thousand bytes from one memory location to another, and performs a busy-wait loop on a bit within the global variable.
- `copy_memory_block`: Sets a global variable, copies a block of twelve thousand bytes from one memory location to another, and performs a busy-wait loop on a bit in a global variable.
- `stub_empty_body_colour_access`: The decompiled body is empty, although the facts record button port, serial port and background colour accesses across six unnamed calls; no behaviour is recovered from the code.
- `fill_alternating_pattern_then_rows`: Writes an alternating bit pattern into a run of longwords, then loops over rows calling three unnamed routines per row while counting down a line budget.
- `encode_clock_bits_longword`: Merges the data bits of one longword with clock bits computed from its neighbouring bits using an alternating-bit mask, clearing the top bit when a flag in the preceding byte is set.
- `set_clock_bit_in_byte`: Sets or clears a single bit in a byte depending on bit zero of the byte before it, returning early when bit six is already set in the no-flag case.
- `encode_paired_longwords_into_buffer`: When a sync marker longword matches the alternating pattern, combines pairs of longwords into the destination buffer, shifting data bits and counting down a remaining length; otherwise it sets a status byte to minus one.
- `compute_ones_complement_checksum`: Sums a block of longwords with end-around carry, starting at a fixed offset from a pointer and sized by a count, and returns the bitwise complement of the sum.
- `compute_checksum_from_offset`: Computes the same end-around-carry checksum over a block of longwords, with the start taken from a word offset in a register, and returns its complement.
- `start_disk_read_sync_wait`: Waits for the disk controller to be idle, programs the disk pointer, sync word and read length, then polls for the sync request within a fixed timeout, resetting the transfer when the sync appears and disabling it otherwise.
- `start_disk_read_alternate_sync`: Waits for the disk controller to be idle, programs the disk pointer and a different sync word, then polls for the sync request within a fixed timeout, resetting the transfer when it appears and disabling it otherwise.
- `clear_pattern_start_disk_read`: Waits for the disk controller to be idle, fills a block of longwords with an alternating pattern, programs the disk pointer and sync word, then polls for the sync request and resets the transfer when it appears.
- `step_drive_select_bits`: Clears one port bit, then loops a given number of times toggling a low bit of a second port value and calling an unnamed routine, returning the remaining count.
- `clear_port_bits_and_call`: Clears bit one and sets bit zero of a port byte held in a global, then calls one unnamed routine.
- `set_port_bits_and_call`: Sets the two low bits of a port byte held in a global, then calls one unnamed routine.
- `pulse_port_bit_while_flag`: Sets bit one of a port byte, then repeatedly pulses bit zero by clearing and setting it while bit four stays set, calling an unnamed routine on each pulse.
- `delay_fixed_countdown_loop`: A countdown loop of 4000 passes with no other effects.
- `set_port_bit_from_argument`: Clears bit two of a port byte held in a global and ORs in a byte passed in a register.
- `scan_table_build_audio_entries`: Scans a byte table for the first entry above a running value, then fills thirty entries of a global table from a second table using the matched index; the facts record audio volume register touches.
- `advance_audio_frame_sequence`: Advances a frame counter and, every sixth call, reloads the audio pointer from a table, waits briefly, and writes per-channel values from stored entries; the facts record audio location register touches.
- `process_audio_channel_effects`: For each of four audio channels, applies a per-step effect selected by the low nibble of a control byte, such as stepping a period value up or down with clamping, then writes the channel's length, pointer and period registers and updates DMA control.
- `decode_pattern_row_to_channel`: Reads a four-byte pattern row, extracts an instrument index from its nibbles, and loads that instrument's pointer, length and volume values into a channel record; a zero index sets default values.
- `copy_frame_rows_on_timer`: Advances a tick counter and, every fifth call, copies a block of seventeen rows across four columns from a table selected by a stored index into destinations spaced 9600 bytes apart.
- `display_credit_text_screen`: Clears a display region, draws a 12000-longword block into it, then calls several routines; the referenced text includes a 'cracked in 1990 by' line.
- `copy_matching_table_rows`: Walks six stored values and, when one is no greater than a reference value, records its index and copies a block of five-longword rows from a fixed table into a destination.
- `loop_five_entries_with_routine`: Loops over five stored longword entries, passing each to an unnamed routine and calling two more routines per pass.
- `copy_index_selected_rows_grid`: Copies a four-row by ten-column block from a source selected by a parameter index scaled by 55 bytes, writing each value into a 40-byte column stride with rows spaced 9600 bytes apart.

### Turn It.adf:4 (raw code file, load address unknown)

- `initialise_entry_and_await_input`: Entry code that sets the chipset interrupt and DMA state, runs three unnamed routines and the audio clearing routine, then waits on a status bit before continuing.
- `build_audio_entries_second_table`: Scans a byte table for the first entry above a running value, then fills thirty entries of a global table from a second table using the matched index; this copy reads from a different table.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `advance_audio_sequence_second`: Advances a frame counter and, every sixth call, reloads the audio pointer from a different table, waits briefly, and writes per-channel values from stored entries.
- `process_audio_channel_effects_second`: Applies the same per-channel effect steps as its neighbour to four audio channels, using the low nibble of a control byte, clamping stepped period values, and writing the channel's length, pointer and period registers.
- `decode_pattern_row_second`: Reads a four-byte pattern row, extracts an instrument index from its nibbles, and loads that instrument's pointer, length and volume values into a channel record, with a zero index setting defaults; it updates the DMA control register.
- `clear_byte_grid_block`: Zeroes a four-by-four grid of bytes, advancing 28 bytes per cell and 6440 bytes per row from a base offset.
- `start_blitter_rows_from_index`: Waits for the blitter to be idle, then starts four blit operations whose addresses are scaled from a stored index, advancing both source and destination by fixed steps per row.
- `start_indexed_blit_row_set`: Runs four passes, each waiting for the blitter to be idle and starting a blit whose addresses are scaled from a stored index, with a fixed row size and per-pass offsets.
- `decrement_digit_words_until_clear`: Repeatedly subtracts one from the nibble digits of each word in a row, calling an unnamed routine after each pass, until a pass changes no digit.
- `copy_row_then_transfer_digits`: Copies a row of words into a buffer, then repeatedly moves each word's digit nibbles into a parallel array one unit at a time, calling one unnamed routine per pass until no digit moves.
- `wait_raster_ten_passes`: Waits for a vertical beam position field to reach a fixed value ten times in succession, then returns.
- `write_copper_pointer_pairs`: Writes four pointer pairs into a copper-related table at fixed steps of 6440 bytes, then writes two fixed words to globals.
- `generate_pseudo_random_modulo`: Advances a 32-bit seed by doubling it and exclusive-ORing with a fixed constant on overflow or zero, then returns the seed modulo the requested range.

### Turn It.adf:2#unpacked (raw code file, load address unknown)

- `initialise_entry_keyboard_and_display`: Program entry that sets colour and interrupt state, selects one of three values from a mode setting, and calls seven unnamed routines; the facts record keyboard serial and interrupt request register access.
- `initialize_recursive_stack`: The routine performs four consecutive self-calls, resulting in a recursive execution pattern that exhausts the stack.
- `copy_indexed_glyph_block`: Copies five rows of fifteen columns, each column eight bytes wide, from a source offset scaled by 600 bytes into a destination with a 40-byte column stride and rows 9600 bytes apart.
- `copy_wide_indexed_block`: Copies five groups of twenty-one rows, each twenty-seven bytes wide, from a source offset scaled by 2700 bytes into a destination with a 40-byte row stride.
- `decrement_digit_words_second`: Uses the same digit decrement loop as its neighbour: repeats passes over a row of words, subtracting one from nibble digits and calling an unnamed routine each pass, until no digit changes.
- `copy_row_transfer_digits_second`: Copies a row of words and then transfers their digit nibbles into a parallel array one unit at a time across repeated passes, the same procedure as its neighbour.
- `wait_for_bit_pattern`: Enters a loop that repeatedly checks a global variable until a specific bit pattern is detected, repeating this check a fixed number of times.
- `write_copper_pointer_table_pairs`: Writes four pointer pairs into a copper-related table at fixed 9600-byte steps, then sets a few fixed globals to constants.
- `build_audio_entries_third_table`: Scans a byte table for the first entry above a running value and fills thirty global entries from a different table, indexed by the matched value; the facts record audio volume register touches.
- `reset_audio_state_globals`: Clears four global variables and sets a fifth to fifteen; the facts record audio pointer and DMA control register touches.
- `process_audio_channel_effects_third`: Applies the per-channel effect steps for four audio channels, using the low nibble of a control byte and clamping period values, then writes each channel's length, pointer and period registers and updates DMA control.
- `decode_pattern_row_third`: Reads a four-byte pattern row, extracts an instrument index from its nibbles, and loads that instrument's pointer, length and volume values into a channel record, with a zero index setting defaults; it writes the DMA control register.
