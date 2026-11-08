# Game 1006 (turn it (1989)(kingsoft)(m3)[cr black monks][t +2 black monks]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01006). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (912272 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 54 raw-word lines |
| Functions named and described | pass, 154 of 154 named (0 author symbols), 154 of 154 described (2 by rule, 29 by the local model) |
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
| Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:BMturnit | 2 | 81 | 4 |
| Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 646 | 23 |
| Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:3 (raw code file, load address unknown) | 1 | 3100 | 100 |
| Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:4 (raw code file, load address unknown) | 1 | 543 | 14 |
| Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:2#unpacked (raw code file, load address unknown) | 1 | 693 | 12 |

## Cases handled

- turn it (1989)(kingsoft)(m3)[cr black monks][t +2 black monks]/Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:1: BK: ByteKiller-packed, unpacked with ancient (15520 -> 39280 bytes)
- turn it (1989)(kingsoft)(m3)[cr black monks][t +2 black monks]/Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:2: BK: ByteKiller-packed, unpacked with ancient (89952 -> 150000 bytes)
- runtime segment an address: 509 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- turn it (1989)(kingsoft)(m3)[cr black monks][t +2 black monks]/Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:3: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- turn it (1989)(kingsoft)(m3)[cr black monks][t +2 black monks]/Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:4: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- turn it (1989)(kingsoft)(m3)[cr black monks][t +2 black monks]/Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 792000 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_memory_block_with_loop_control` (runtime segment an address): Copies a block of memory from a source pointer to a destination pointer using a counter-based loop. Kind: checksum/data integrity trap. Pass path: the loop that performs the memory copy (the inner `do-while` loop starting at `puVar4 = extraout_A0`).
- `run_startup_then_main_loop` (3 (raw code file, load address unknown)): Writes setup values to the copper, DMA and interrupt control registers, copies a table block, and starts the copper list. Kind: loader/protection routine. Pass path: The routine performs a series of initialization steps (u3__sub_2162, u3__sub_0A6E, etc.) followed by a conditional branch based on `uRam00000100` (likely a disk/track identifier) and a memory copy loop. The presence of the 'apricot' passwor.
- `stub_bad_instruction_data` (3 (raw code file, load address unknown)): The decompiled body is only a return, so the real behaviour cannot be recovered from this evidence. Kind: instruction trap/sabotage path. Pass path: not determined.
- `start_disk_dma_transfer` (3 (raw code file, load address unknown)): Waits for the disk controller to go idle, programs the disk pointer, sync word and transfer length, then enables disk DMA. It polls the interrupt request bit for a bounded count, then stops the transfer. Kind: disk track/sync validation loop. Pass path: the loop completes (iVar1 reaches 0) and execution continues to the instructions following the call.
- `start_disk_dma_fixed_length` (3 (raw code file, load address unknown)): Starts a disk DMA transfer with a fixed length word instead of a sync word, using the same wait, enable and bounded polling sequence as its neighbour. Kind: disk hardware configuration/DMA setup. Pass path: the branch at an address (after the tst.l d6 / beq.w an address sequence).
- `fill_sync_then_disk_dma` (3 (raw code file, load address unknown)): Fills a buffer with 3,251 longwords of an alternating-bit sync pattern, then starts a disk DMA transfer from it and polls for completion with a bounded count. Kind: disk sync/track check. Pass path: the code continues to the instructions following the call (movea.l (a5), a0).
- `pulse_port_b_bit_count` (3 (raw code file, load address unknown)): Clears bit one of the second CIA port's control byte, then toggles bit zero once for each pass in a counted loop, calling a helper on every toggle. Kind: disk hardware timing/pulse check. Pass path: the loop continues (the branch where sVar1 >= 0 is taken).
- `set_port_b_low_call` (3 (raw code file, load address unknown)): Clears the two low bits of the second CIA port's control byte, sets bit zero, then calls one helper routine. Kind: disk hardware/sync check. Pass path: addq.w #an address, an address(a5).
- `set_port_b_bits` (3 (raw code file, load address unknown)): Sets bits zero and one of the second CIA port's control byte, then calls one helper routine. Kind: hardware register manipulation/disk controller check. Pass path: the instruction following the call (subq.w #an address, an address(a5)).
- `pulse_port_b_while_set` (3 (raw code file, load address unknown)): Sets bit one of the second CIA port's control byte, then, while bit four stays set, toggles bit zero and calls a helper on each pass. Kind: disk hardware timing/pulse check. Pass path: the branch at an address (the loop continues/finishes normally).
- `update_port_b_from_argument` (3 (raw code file, load address unknown)): Clears bit two of the second CIA port's control byte and ORs in the bits supplied in a register. Kind: disk hardware/port status check. Pass path: the branch taken when tst.b an address(a5) results in zero (the instruction following the bne.w an address).
- `draw_crack_credit_line` (3 (raw code file, load address unknown)): Clears a 14,400-longword buffer by copying a block into it, then draws a text line that the facts show reads 'cracked in 1990 by'. It disables interrupts and DMA before drawing, re-enables them afterwards, and calls nine helper routines. Kind: crack credit display routine. Pass path: The routine is a specialized display loop that iterates through a character table (likely converting hex or custom encoded values to ASCII) to draw the 'Cracked by...' text to the screen. The presence of the 'cracked in 1990' string and the.

## Functions


### Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:BMturnit

- `start_depacker_with_background_color`: Program entry point.
- `write_countdown_words_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, starting from minus one, until the counter wraps back to minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_and_hardware_state`: Disables interrupts, initializes a global table by copying a block of code, and enters a loop that repeatedly calls update_sprite_data_table, update_blt_registers, initialize_and_load_tile_data, and update_mouse_coordinates while modifying hardware registers and a global variable with specific constants.
- `initialize_and_load_tile_data`: Clears a block of memory, initializes a global table with a specific bit pattern, and then copies or expands data from a source table into a destination buffer using a stride-based copy or a tiling-like pattern.
- `update_blt_registers`: Iterates through a table to update a global pointer and a counter, while setting certain hardware registers.
- `update_mouse_coordinates`: Calculates the difference between two values, updates several global variables by adding or subtracting a value based on a modulo calculation, and handles boundary clamping.
- `update_sprite_data_table`: Calculates an index from a global variable to access a table, then performs a series of loops to copy blocks of data from a source buffer into a destination table based on specific byte values found in the table.
- `configure_interrupt_and_dma`: Sets up DMA pointers and high-word values in a table, acknowledges an interrupt by writing to the interrupt request register, and copies a memory block using a loop.
- `setup_dma_pointers`: Writes a series of pointers and high-word values into a table and sets certain hardware registers.
- `acknowledge_interrupt`: Writes a value to the interrupt request register and returns from an interrupt.
- `copy_memory_block_with_loop_control`: Copies a block of memory from a source pointer to a destination pointer using a counter-based loop.
- `convert_to_lowercase`: Checks if a byte in d1 is between 61 and 7a inclusive, and if so, subtracts 32 from it before returning.
- `update_frame_index_and_bit`: Calculates a new index based on a register value and updates a global memory location.
- `modify_byte_bits_conditionally`: Clears a 16-bit word at the address in a6, sets bits 0 and 1 of a byte at the address in a4, and then loops while bit 4 of a byte at offset 3841 from the address in a4 is set, calling another routine during each iteration.
- `adjust_counter_via_bits`: Compares a 16-bit word at the address in a6 with d0 and enters a loop that either increments the word at a6 while clearing bit 1 of a byte at a4 and calling another routine, or decrements the word at a6 while setting bit 1 of a byte at a4 and calling another routine.
- `toggle_bit_and_call_routine`: Clears bit 0 of a byte at the address in a4, sets bit 0 of the same byte, and then calls another routine.
- `decrement_counter_loop`: Sets d7 to 48832 and decrements it in a loop until it reaches zero.
- `decrement_counter_loop_nop`: Sets d7 to 3000 and decrements it in a loop, followed by a no-operation instruction.
- `wait_for_bit`: Polls bit 5 of a byte at an offset of 3841 from a pointer in a6 until it is clear.
- `poll_bits_and_write_values`: Writes four 16-bit values to offsets of a structure in a5, then loops until bit 4 of a byte at an offset of 1952 from a pointer in a4 is set, and finally loops until bit 1 of a byte at offset 31 of the structure in a5 is set before returning.
- `unpack_bitstream`: Searches a table for a specific marker, then iterates through a data stream to perform bitwise operations and store the results in a destination buffer.
- `decompress_bitstream`: Processes a bitstream by calling two different routines to read bits and refill a buffer, then writes decoded bytes to a destination memory area.
- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:3 (raw code file, load address unknown)

- `run_startup_then_main_loop`: Writes setup values to the copper, DMA and interrupt control registers, copies a table block, and starts the copper list.
- `swap_hundred_longword_blocks`: Swaps 100 longwords between two buffers one at a time, then calls a helper.
- `select_mode_and_start_copper`: Sets a mode value to one of two fixed values depending on a flag, then calls the routine that starts the copper list.
- `rotate_and_complement_state_byte`: When a status bit is set, rotates a state byte by one bit and clears a flag.
- `scan_grid_for_matching_entry`: Scans a grid of table entries row by row, 20 columns by 180 rows, comparing the high byte of each entry against a reference value.
- `store_coordinates_then_call_helpers`: Stores a pair of coordinate values and a value into a state record and calls several helpers when a sign flag is clear.
- `update_position_when_changed`: If a sign check passes and a masked high byte matches a reference, it compares a coordinate pair against a stored pair.
- `adjust_counter_by_code_byte`: When the coordinate pair matches one of two stored pairs, it reads the high byte of a table word and adjusts a counter.
- `call_helper_then_clear_state`: Calls a helper routine, then clears two state bytes.
- `reset_state_then_call_helper`: Calls one helper routine and then zeroes a state byte twice.
- `repeat_helper_until_counter_zero`: Calls a helper twice per pass while a 32-bit counter, built from two 16-bit memory words, is still nonzero.
- `repeat_helper_until_pair_zero`: Same loop shape as its neighbour, calling a helper twice per pass until a counter read from a different pair of memory words reaches zero.
- `process_three_byte_table_entries`: Walks a table of three-byte entries until it reaches a minus-one terminator, calling two helper routines for each entry.
- `add_ten_to_digit_counter`: Combines four stored digit fields into one number, adds ten, and splits the result back into digit fields.
- `copy_block_and_lookup_bytes`: Copies one hundred 32-bit words from a table row chosen by a stored index into a fixed RAM buffer.
- `adjust_digit_records_by_third`: Walks a list of four-digit records until a terminator and converts each one to a single number.
- `sequence_seven_helpers_then_reset`: Calls seven helper routines in order and then zeroes a state byte.
- `reset_state_around_helper_calls`: Zeroes a state byte, calls a helper, zeroes the byte again, then calls two more helpers.
- `zero_state_between_helper_calls`: Calls a helper, zeroes a state byte several times, then calls two helpers.
- `choose_position_from_mode_flags`: Reads two mode words.
- `copy_grid_data`: Copies eighteen blocks of four rows from a source area to a destination area.
- `copy_indexed_block_grid`: Copies eighteen blocks of four rows, each row eighteen bytes, from a source area into a destination area.
- `dispatch_state_to_helper_calls`: Branches on a stored state value from zero to three and calls helper routines in a fixed pattern for each case.
- `raise_state_to_minimum_bounds`: Compares a stored value against two stored bounds and copies each bound into the state byte when the value is below it.
- `copy_three_byte_column_grid`: Copies a grid of four rows by eight entries, three bytes each, from a source area into a destination area, with a row stride on each side.
- `copy_strided_table_column_block`: Calls three helper routines, then copies eleven rows of five values from a table chosen by a stored index into a fixed destination.
- `copy_column_block_to_offset`: Same copy as its neighbour, calling three helpers and then moving eleven rows of five values from an indexed table into a destination at a different fixed address.
- `copy_column_block_at_base`: Same copy shape as its neighbours, calling three helpers and then writing eleven rows of five values into a fixed destination at another base address.
- `copy_column_block_from_pointer`: Copies eleven rows of five bytes from a source pointer offset by a stored index into a destination pointer, spreading each row across blocks 640 bytes apart.
- `run_helper_four_times`: Calls one helper routine four times in a row and returns.
- `copy_column_block_between_pointers`: Copies eleven rows of five bytes from a source pointer into a destination pointer, placing each row across blocks 640 bytes apart.
- `build_grid_neighbour_path_records`: Returns early when two stored positions are neighbours on a grid forty cells wide, either one or two steps apart horizontally or forty apart vertically.
- `write_step_records_until_limit`: Writes four-byte step records along one axis, calling two helper routines per record.
- `write_step_records_until_twenty`: Writes four-byte step records along one axis, similar to its neighbour but stopping at a counter of minus one or twenty.
- `advance_state_clamped_to_bound`: Adds two to a state byte and calls a helper.
- `classify_pair_against_target`: Copies a first record, then writes a marker byte in front of each following two-byte record in a list ended by minus one.
- `blit_or_combined_tile_rows`: Ors four source words per column from a table keyed by a stored position into a temporary buffer, then, for each entry of a minus-one terminated list, programs the Amiga blitter registers for a copy and waits until the blitter is no longer busy.
- `spin_delay_countdown`: Decrements a counter register in a loop until it reaches zero, giving a fixed busy-wait delay.
- `probe_cell_byte_without_effect`: Reads one byte from a table at a computed index and returns the input registers unchanged in both branches, so the read has no visible result.
- `probe_grid_cell_bounds`: Looks up one byte in a grid forty cells wide from stored row and column values, after a range check on the column.
- `probe_grid_cell_from_registers`: Reads one byte in a grid forty cells wide, using row and column values passed in registers, after a range check on the column.
- `copy_word_column_to_buffer`: Copies nineteen 16-bit words from a source buffer into a destination column, stepping forty bytes between destination words.
- `wait_for_port_bit`: Waits until one bit of a stored port byte becomes set, with a countdown delay before and after the wait.
- `call_helper_grid_then_reset`: Calls a helper routine once for each cell of an eight-row by eighteen-column grid, then zeroes a state byte twice and makes one more helper call.
- `compare_cell_bytes_and_flag`: Compares the low bytes of two words at a computed offset from two base pointers.
- `repeat_helper_for_count`: Calls a helper routine repeatedly, decrementing a count held in a register, until the count drops below zero.
- `repeat_helper_until_result_negative`: Calls a helper and feeds its returned value back into a decrementing loop, repeating while the value stays non-negative.
- `copy_rows_after_button_poll`: Waits on the raster position and the right and middle mouse button bits, then copies four-word columns from a source into a destination stepping forty bytes per row, repeating down a counted list of rows.
- `call_helper_over_grid`: Calls one helper routine for each cell of a grid with ten rows and twenty columns.
- `blit_copy_rows_with_wait`: Programs the Amiga blitter for a copy with fixed control settings, then runs five blits.
- `call_helper_pair_twice`: Calls two helper routines in turn, and repeats that pair a second time.
- `return_from_subroutine`: The routine is empty and performs no operations before returning to its caller.
- `wrap_helper_call_with_return`: Calls one helper routine and returns straight away.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `write_copper_list_words`: Writes a fixed word and then a minus-one terminator into a copper list entry.
- `start_copper_then_run_steps`: Starts the copper list running, then calls four helper routines in sequence.
- `set_mid_stride_and_call`: Stores the same stride value into two memory words, calling a helper after each store.
- `set_wide_stride_and_call`: Stores a wider stride value into two memory words, calling a helper after each store.
- `set_low_stride_and_call`: Stores a mid-sized stride value into a second pair of memory words, calling a helper after each store.
- `call_helper_twice_in_row`: Calls one helper routine twice in a row and returns.
- `call_helper_twice_then_return`: Calls one helper routine twice and returns.
- `clear_dma_flag_when_done`: Checks a DMA channel record's active flag and its bit mask against the DMA control bits.
- `start_dma_channel_from_record`: Writes one channel's pointer, length, period and control words from a record into the custom chip registers, then sets the channel's enable bit in the DMA control register and marks the record active.
- `build_block_pointer_table`: Fills a table with pairs of high and low address words for blocks spaced 640 bytes apart, then for further groups spaced 9,600 bytes apart.
- `draw_text_glyphs_centered`: Measures a zero-terminated string, centres it on a line, then copies each character's glyph block from a font table into the destination.
- `decrement_nibble_fields_until_zero`: Repeatedly subtracts one from each nonzero hundreds, tens and units nibble in a row of packed words, calling a helper between passes, until every nibble is zero.
- `transfer_nibble_units_between_rows`: Copies a row of packed nibble words, then moves one unit out of each nonzero hundreds, tens and units field into a second row.
- `wait_for_raster_line_count`: Counts down a stored number of raster lines, waiting on the vertical position register for each one.
- `track_mouse_motion_within_bounds`: Reads the mouse movement counter and applies the horizontal and vertical differences to two state bytes, clamping each result within stored bounds.
- `write_entry_record_from_coords`: Chooses one of two source pointers by comparing a stored index with nine, then writes four bytes built from stored coordinates and a fixed offset into that record.
- `write_four_byte_record`: Stores four bytes into a record at a pointer: a value from one register, a right-shifted copy of another register, and two further register values.
- `clear_large_buffer_longwords`: Zeroes 14,400 consecutive 32-bit words starting at a fixed buffer address.
- `dispatch_string_entry_by_parity`: Acts only when a stored value is a multiple of ten.
- `copy_table_then_wait_bounded`: Sets one control word, copies twelve thousand longwords from a fixed table into a buffer, runs three helpers and polls a status bit for at most 350,000 passes.
- `copy_alt_table_bounded_wait`: Same sequence as its neighbour, copying twelve thousand longwords from a different fixed table into a buffer, then polling a status bit for a bounded count.
- `stub_bad_instruction_data`: The decompiled body is only a return, so the real behaviour cannot be recovered from this evidence.
- `write_sync_and_sector_blocks`: Fills a buffer with a repeated alternating-bit sync pattern, writes a sync word and a header, then calls three helpers per pass to lay out each block.
- `encode_mfm_clock_longword`: Encodes one longword by interleaving clock bits derived from the inverted data bits, using an alternating-bit mask.
- `adjust_mfm_clock_bit`: Adjusts the top bit of one encoded byte using the low bit of the preceding byte.
- `decode_mfm_longword_block`: After one helper call, checks for a sync word in a buffer.
- `checksum_longword_block_ones_complement`: Adds a block of 32-bit words with end-around carry and returns the one's complement of the sum.
- `checksum_block_at_offset`: Adds a block of 32-bit words that starts at a stored offset from a stored pointer, with end-around carry, and returns the one's complement.
- `start_disk_dma_transfer`: Waits for the disk controller to go idle, programs the disk pointer, sync word and transfer length, then enables disk DMA. It polls the interrupt request bit for a bounded count, then stops the transfer.
- `start_disk_dma_fixed_length`: Starts a disk DMA transfer with a fixed length word instead of a sync word, using the same wait, enable and bounded polling sequence as its neighbour.
- `fill_sync_then_disk_dma`: Fills a buffer with 3,251 longwords of an alternating-bit sync pattern, then starts a disk DMA transfer from it and polls for completion with a bounded count.
- `pulse_port_b_bit_count`: Clears bit one of the second CIA port's control byte, then toggles bit zero once for each pass in a counted loop, calling a helper on every toggle.
- `set_port_b_low_call`: Clears the two low bits of the second CIA port's control byte, sets bit zero, then calls one helper routine.
- `set_port_b_bits`: Sets bits zero and one of the second CIA port's control byte, then calls one helper routine.
- `pulse_port_b_while_set`: Sets bit one of the second CIA port's control byte, then, while bit four stays set, toggles bit zero and calls a helper on each pass.
- `spin_delay_4000_passes`: Counts down from 4,000 in a loop, giving a fixed busy-wait delay.
- `update_port_b_from_argument`: Clears bit two of the second CIA port's control byte and ORs in the bits supplied in a register.
- `build_audio_channel_volume_tables`: Scans a 127-entry byte table for a matching value, then builds a table of thirty period-style values for each of four audio channels and writes the volume registers.
- `advance_audio_channel_sequence`: Counts passes, and on every sixth pass restarts the four audio channels by writing each channel's start address from a stored sample table.
- `step_audio_channel_volume`: Updates one audio channel's volume by a stored step, adding or subtracting it and clamping to fixed limits, then writes the result to the channel's volume register.
- `load_audio_sample_record`: Copies a four-byte record from a source table and, when the high nibble of its first byte is nonzero, copies the matching sample entry's start address, length and period into the channel record.
- `advance_frame_block_copy_cycle`: Advances a cycle counter, and on every fifth call copies a block of seventeen rows of four values from a table selected by a stored index into the destination, wrapping the cycle counter at eight.
- `draw_crack_credit_line`: Clears a 14,400-longword buffer by copying a block into it, then draws a text line that the facts show reads 'cracked in 1990 by'. It disables interrupts and DMA before drawing, re-enables them afterwards, and calls nine helper routines.
- `shift_buffer_rows_by_threshold`: Finds the first entry in a five-entry table that is at least a stored value, then shifts five-word rows one at a time through a buffer, stepping backwards.
- `run_helpers_over_five_rows`: Walks a five-entry table, calling a helper for each entry and running a nineteen-pass inner loop that calls helpers and advances a pointer.
- `copy_four_by_ten_rows`: Copies a block of four rows and ten bytes per row from a table at a stored index times fifty-five into a destination with fixed row strides.

### Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:4 (raw code file, load address unknown)

- `init_hardware_on_region_start`: Runs at the start of the code region.
- `build_second_audio_volume_tables`: Scans a 127-entry byte table for a matching value, then builds a table of thirty period-style values for each of four audio channels and writes the four volume registers.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `update_audio_sample_parameters_2`: Modifies audio playback properties such as sample position, length, or pitch by updating hardware registers based on a control byte and internal state variables.
- `update_audio_sample_parameters`: Updates audio playback parameters such as length or position based on a control byte.
- `load_second_audio_sample_record`: Copies a four-byte record and, when the high nibble of its first byte is nonzero, copies the matching sample entry's start address, length and period into the channel record.
- `clear_four_by_four_cells`: Zeroes a four-by-four arrangement of bytes at twenty-eight byte spacing, repeated across four blocks that are spaced 6,440 bytes apart.
- `blit_four_rows_fixed_size`: Programs the blitter for four blits of fixed size, waiting for each one to finish before starting the next.
- `blit_four_strip_copies`: Runs four blitter copies of a fixed size, waiting for the blitter to be idle before each one and advancing the source and destination pointers between passes.
- `decrement_packed_nibbles_until_zero`: Repeatedly subtracts one from each nonzero hundreds, tens and units nibble in a row of packed words, calling a helper between passes, until every nibble is zero.
- `copy_and_redistribute_data`: Copies a block of memory to a destination and then iteratively moves specific bit-pattern values from the destination to another memory location until no such patterns remain.
- `wait_for_ten_raster_lines`: Waits on the vertical position register for ten raster lines in a row, one wait per line.
- `write_copper_block_pointers`: Writes four address pairs into a copper list, each block 6,440 bytes after the last, then writes a fixed word pair to the copper.
- `advance_pseudo_random_mod`: Advances a 32-bit pseudo-random state by doubling it and XORing in a feedback constant when the shift overflows or the result is zero.

### Turn It (1989)(Kingsoft)(M3)[cr Black Monks][t +2 Black Monks].adf:2#unpacked (raw code file, load address unknown)

- `init_and_poll_keyboard_region`: Entry of a raw code region with 133 instructions.
- `run_helper_four_times_b`: Calls one helper routine four times in a row and returns.
- `copy_fourteen_row_blocks`: Copies four blocks of fourteen rows, eight bytes each, from a source chosen by a stored index times 600 into a destination with 40-byte row strides and blocks 9,600 bytes apart.
- `copy_nineteen_row_blocks`: Copies four blocks of nineteen rows, twenty-seven bytes each, from a source chosen by a stored index times 2,700 into a destination with 40-byte row strides and blocks 9,600 bytes apart.
- `decrement_nibbles_until_zero_b`: Repeatedly subtracts one from each nonzero hundreds, tens and units nibble in a row of packed words, calling a helper between passes, until every nibble is zero.
- `copy_and_redistribute_data_2`: Performs a memory block copy followed by a loop that extracts specific bit values from one table and adds them to another table.
- `wait_for_raster_lines_nine`: Waits on the vertical position register for nine raster lines in a row, one wait per line.
- `write_copper_pointers_and_state`: Writes four address pairs spaced 9,600 bytes apart into a pointer table, then stores two fixed words and two state values.
- `build_third_audio_volume_tables`: Scans a 127-entry byte table for a matching value, then builds a table of thirty period-style values for each of four audio channels and writes the four volume registers.
- `reset_audio_state_fields`: Clears four state fields and sets a fifth to fifteen.
- `update_audio_parameter_state`: Updates internal counters and values based on a control byte to manage audio playback parameters.
- `update_sprite_attribute_table`: Parses a data structure to update sprite attributes and positions by indexing into a table.
