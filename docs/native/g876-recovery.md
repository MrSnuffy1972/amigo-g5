# Game 876 (holiday lemmings 1993 (1993)(dma design)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00876). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 303 of 303 named (0 author symbols), 303 of 303 described (13 by rule, 108 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1546 of 1546 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8709 of 8710 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 164590 |
| code | 31138 |
| data | 245413 |
| padding | 8459 |
| unused | 451584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Holiday Lemmings 1993 (1993)(DMA Design).adf:HolidayLemmings | 4 | 8456 | 293 |
| Holiday Lemmings 1993 (1993)(DMA Design).adf:TRAINER! | 2 | 235 | 8 |
| Holiday Lemmings 1993 (1993)(DMA Design).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:Level000: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (1360 -> 8192 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:Level001: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (1344 -> 8192 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:Level002: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (1428 -> 8192 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:Level003: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (1464 -> 8192 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:Level004: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (1836 -> 8192 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:Level005: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (1116 -> 8192 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:Level006: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (1244 -> 8192 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:Level007: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (1636 -> 8192 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:ground1: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (4708 -> 15088 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:ground3: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (22720 -> 35560 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:leveldata: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (3932 -> 7792 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:kw: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (3596 -> 5439 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:objects1: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (13396 -> 52690 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:rudi: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (2940 -> 4827 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:Objects3: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (10508 -> 40650 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:panel1: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (3064 -> 12800 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:Icons: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (44208 -> 77466 bytes)
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:jb: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (2172 -> 9243 bytes)
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:TRAINER!: ProPack (RNC) (xfd) stream found by ancient scan (9168 -> 41400 bytes) but only 0 of 8 sampled windows are in reference RAM: not used
- holiday lemmings 1993 (1993)(dma design)/Holiday Lemmings 1993 (1993)(DMA Design).adf:TRAINER!: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Holiday Lemmings 1993 (1993)(DMA Design).adf:TRAINER!

## Copy protection found

- `drive_cia_port_pattern` (HolidayLemmings): Writes all ones to a CIA port register, then a fixed byte pattern with another routine call between writes, and finally all ones again. Kind: CIAB printer-port pattern check (authenticates a protected disk by reading a byte pattern from the parallel port). Pass path: The routine writes a byte pattern to DAT_00bfd100 (the CIAB PRB register, an address(a6)) via u0__sub_2_2F82(), then reads it back; a successful check corresponds to the caller taking the branch after the call when the returned value matches the e.

## Functions


### Holiday Lemmings 1993 (1993)(DMA Design).adf:HolidayLemmings

- `setup_hardware_and_run_loop`: Program entry point for the start-up code.
- `save_chip_state_mask_interrupts`: Saves several custom-chip register values into a structure, clears all interrupt enables and requests, and stores two interrupt-handler pointers.
- `restore_chip_state_from_structure`: Clears the interrupt request bits, then writes back saved values for several custom-chip registers from a structure, including the interrupt enables and the DMA control.
- `copy_state_then_call_libraries`: Copies two saved values into a structure and calls a chain of library functions through a stored library base, recording the first result.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `drive_cia_port_pattern`: Writes all ones to a CIA port register, then a fixed byte pattern with another routine call between writes, and finally all ones again.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `load_resources_run_frame_loop`: Opens the DOS and graphics libraries through their stored bases, allocates working memory, and sets up display and interrupt registers.
- `forward_to_routine`: The decompiled body is only a single call to another routine followed by a return, so the rest of the work cannot be seen in this code.
- `call_five_routines`: Calls five other routines in sequence and then returns, with no other work in this body.
- `advance_timed_sequence_step`: Returns early when a flag byte is set.
- `find_matching_table_entry`: Scans a table of fixed-size records and counts those whose flag byte is clear and whose position falls in a small window around a target.
- `find_matching_record_by_tag`: Scans a table of fixed-size records, matching each record's tag byte against a value and checking that its position falls within a window around a target.
- `update_button_state_flags`: Stores a fixed value in a structure, then derives two flags from individual bits of a stored state byte.
- `advance_state_machine_map_check`: Advances a small state machine by one step.
- `step_position_toward_target`: Moves a 16-bit position one step toward a target value.
- `step_position_down_to_zero`: Reduces a stored 16-bit position toward zero, by four normally or by sixteen when a flag is set and the result stays non-negative.
- `do_nothing_stub`: The decompiled body is only a return, with no operations visible.
- `set_state_word_call_routine`: Stores a fixed default value in one field of a structure and then calls a other routine.
- `calculate_record_offset`: Takes a 16-bit index, subtracts one from it and returns the result multiplied by the size of a 44-byte record.
- `pack_position_into_word_pair`: Packs a row and a column into two 16-bit words after adding fixed offsets and shifting the column, and returns the pair.
- `pack_position_into_word_pair_2`: Packs a row and a column into two 16-bit words after adding fixed offsets and shifting the column, and returns the pair.
- `pack_position_into_word_pair_3`: Packs a row and a column into two 16-bit words after adding fixed offsets and shifting the column, and returns the pair.
- `do_nothing_stub_2`: The routine is a bare return with no operations visible in the decompiled code.
- `store_routine_result_with_offset`: Calls another routine and stores its result in one global variable, then stores a second global variable set to that result plus a fixed offset.
- `advance_two_phase_countdown`: Runs a two-phase countdown on a structure: while one flag is clear it counts a timer down, and at zero it toggles the phase and reloads the timer.
- `handle_keypress_command`: Dispatches on a key code: one key resets stored fields, another restores saved values and starts a countdown, and another toggles a flag.
- `set_width_from_mode_flag`: Marks a flag byte as set, then assigns a width-like field either from a stored value or from a fixed default, depending on a mode flag.
- `set_default_dimension_fields`: Writes a fixed default value into two dimension fields of a structure.
- `draw_glyph_string_to_bitplanes`: Walks a byte-coded string and copies each character's glyph rows from a font table into four bitplanes of the target bitmap.
- `acknowledge_cia_interrupt_bits`: Reads a CIA interrupt-control byte, calls another routine when one bit is set and another when a second bit is set, then acknowledges the matching interrupt request in the interrupt request register.
- `decode_keyboard_serial_byte`: Reads one byte from the keyboard serial register, rearranges its bits into a key code and performs a handshake on the control register.
- `split_number_by_ten_thousand`: Divides a 32-bit number by ten thousand, packs the quotient and remainder into a pair of 16-bit words, and passes that pair to another routine.
- `format_high_word_four_digits`: Converts the upper 16-bit half of a value into four decimal digit characters and packs them into one 32-bit result, most significant digit first.
- `set_bitplane_pointers_for_frame`: Toggles a frame flag and picks one of two buffer addresses from it, storing that address in a structure.
- `wait_for_fire_button_press`: Polls the input register in a loop, calling another routine when one particular key code arrives, and keeps looping until the fire-button bit pattern changes.
- `decrement_countdown_timer_with_borrow`: Advances a frame counter and, when the gating conditions allow, decrements a two-level countdown.
- `wait_until_flag_set`: Clears a flag byte and then busy-waits until another code path sets it to a nonzero value.
- `poll_until_routine_returns_zero`: Calls another routine in a loop and keeps repeating until the routine returns zero, which acts as a polling wait.
- `fill_three_rows_of_entries`: When a counter field is nonzero, it decrements the counter and fills three runs of entries, of 32, 8 and 16 items, by calling another routine for each item and storing the result in a structure.
- `fill_grid_cells_from_routine`: Runs nested loops over a block of cells, calling other routines to compute a value for each cell and storing it at a fixed offset in a structure.
- `fill_rows_from_routine_values`: An outer loop of sixteen iterations calls two other routines, then an inner loop of thirty-two iterations stores another routine-supplied value at a fixed offset.
- `step_packed_position_toward_target`: Compares a current packed position with a target one nibble at a time and moves the position one unit closer in each field.
- `reset_two_pointer_fields`: Calls another routine, then points two pointer fields in its structures at a fixed global address.
- `store_pointer_twice`: Stores a pointer to a global variable through two different pointer locations computed from two unmodified address registers.
- `store_global_ptr_twice`: It stores the address of a global variable into two fixed offsets of two different A5/A6-based structures.
- `initialise_table_entries`: Writes a set of fixed address and size words into a table of entries.
- `set_pointer_table_for_planes`: Fills a table of pointer pairs: three entries step by a large stride, then four entries step by a smaller stride.
- `set_fixed_pointer_pairs`: Writes four fixed 32-bit values, each split into a pair of 16-bit words, into four register pairs.
- `write_fixed_value_pairs`: Writes eight fixed 32-bit values into a contiguous block of global memory, each value split into a low word and a high word.
- `fill_pointer_table_with_stride`: Loops four times, writing successive pointers that step by a fixed stride into a table of eight-byte entries.
- `copy_bitmap_planes_and_clear`: Copies planar bitmap data from a source block into interleaved bitplane memory, writing each source word into several destination rows.
- `copy_bitmap_block_into_planes`: Copies a source bitmap block into interleaved bitplane memory, writing each source word into several destination rows at fixed row offsets.
- `clear_bitplane_memory_block`: Zeroes a large block of memory by writing zero longwords in pairs for a fixed number of iterations.
- `call_routine_when_flagged`: Calls a other routine, then calls a second routine before returning only when a flag byte in the structure is set.
- `store_computed_table_entries`: Copies two 16-bit fields into a backup pair, then calls other routines to compute values and stores each result in several table entries.
- `store_computed_table_entry`: Copies two 16-bit fields into a backup pair, calls another routine to compute a value and stores it in a table entry.
- `update_position_with_clamping`: Updates a two-dimensional position by the difference between its stored and new coordinates.
- `check_completion_condition`: Returns the value of another routine, and when a flag is set it runs a sequence of routine steps and sums two counters.
- `compute_percent_and_store_output`: Computes a percentage as one value times one hundred divided by a total and stores it.
- `write_status_label_for_state`: Fills a 12-byte output buffer with a text label chosen from a status value.
- `format_colon_separated_digits`: Writes colon separators into a text buffer and fills in the decimal digits of a value, dropping leading zero digits.
- `format_two_digit_colon_field`: Writes two colons into a text buffer, then a one- or two-digit number with any leading zero left out.
- `format_word_as_four_digits`: Converts a 16-bit value into four decimal digit characters and packs them into one 32-bit result, most significant digit first.
- `draw_two_digit_number`: For a nonzero value, looks up font rows for the tens and ones digits and copies them into bitplane positions.
- `advance_active_entry_counters`: Returns early when a flag byte is set.
- `process_all_active_entries`: When a flag is set it first resets a counter field and calls other routines.
- `process_active_entries_loop`: Loops over thirty-two entries and calls another routine for each active one, returning the value from the last active entry processed.
- `clear_word_table`: Zeroes a large table of 16-bit words by writing zero to a fixed number of consecutive words.
- `fill_rectangles_into_bitmap`: For each of sixteen entries with a nonzero size, fills a rectangle in the bitmap with a value combining the entry index and a stored pattern.
- `fill_marked_rectangles_with_value`: For each non-empty entry in a table of thirty-two, fills its rectangle in the bitmap with a fixed value of nine, stepping rows by a fixed stride.
- `spawn_next_queued_entry`: Counts a timer down and, when it reaches zero, reloads it and fills the next slot of a table of records from a cycling source list, then advances the slot count.
- `refresh_level_info_display`: Redraws level information when the selected level changes.
- `load_colour_palette_from_record`: Searches a data block for a four-character colour map tag, then unpacks seven packed nibble groups into 16-bit palette words and writes them to a buffer.
- `copy_table_block_by_index`: Copies 512 longwords from one of four blocks of a table, chosen by an index value, into a buffer.
- `set_handler_pointer_for_index`: Stores a pointer in a structure, computed as a base routine address plus a stored index multiplied by a fixed stride.
- `initialise_entry_state_fields`: Resets a block of state fields in a structure from stored source values and derives several fields from a computed value.
- `copy_input_record_into_state`: Copies nine values from an input record into fixed fields of a structure, then clears four counter fields and a flag byte.
- `run_entry_updates_active_slots`: Calls two other routines, then either walks a sentinel-terminated list calling another routine for each item or calls one routine.
- `run_sentinel_list_updates`: Calls two other routines, then either walks a sentinel-terminated list calling another routine for each item or calls one routine.
- `copy_active_entry_rows`: For each of thirty-two entries that is active, copies a 33-byte row from a source table into a buffer at that entry's row position.
- `forward_to_routine_2`: Calls one other routine and returns, with no other work visible in the code.
- `run_blit_sequence_with_waits`: Sets up a blit-related structure, then writes a fixed size word and waits for the blitter busy bit to clear.
- `build_row_offset_table`: Fills a table of 184 words with successive multiples of 204, giving row offsets, then clears a further run of 48 words.
- `update_saved_percentage_display`: Computes a saved percentage from two counters and copies its digits into a status message.
- `copy_large_word_block`: Copies a large block of 16-bit words from a source pointer into a buffer, counting a 16-bit counter down until it wraps around.
- `call_two_routines`: Calls two other routines in sequence and returns.
- `call_two_routines_2`: Invokes two other routines in sequence and then returns without touching its own data.
- `build_active_entry_positions`: Scans thirty-two entries and, for each one whose flag is one, stores a packed position from its coordinates plus an offset.
- `copy_first_flagged_entry_position`: Scans thirty-two entries and, at the first one whose flag equals two, copies its two packed position fields into a structure and stops.
- `pack_state_bits_rotate_buffer`: Packs flag bits from several state bytes into a set of bitplane bytes, then sums eight bytes into a nibble.
- `wait_for_register_change`: Repeats a spin that waits for the high byte of a word in a structure to change, using the input value as the repeat count.
- `update_fire_button_latch`: Reads a stored input byte.
- `advance_text_entry_character`: Counts a timer down and, when it reaches zero, reloads it and writes the next character from a source table into an entry buffer.
- `load_file_into_buffer`: Locks, opens and reads a file through the DOS library into a buffer, then stores the loaded address in the copper pointer fields.
- `copy_byte_block`: Copies a count of bytes from a source address to a destination address, one byte at a time.
- `enter_name_from_keyboard`: Reads keyboard codes into a short name buffer, handling backspace, delete and cursor-movement codes and uppercasing letters.
- `reset_state_fields_and_routines`: Clears several state fields of a structure, conditionally calls another routine, clears a further field when a flag is zero, and then calls three other routines.
- `setup_fixed_dimensions_and_routines`: Calls two other routines, sets two dimension fields and a buffer pointer to fixed values, clears a flag, and then calls a sequence of sixteen other routines.
- `select_panel_one`: Writes the character for one into a panel selection byte, clears a flag in the structure, and calls two other routines.
- `select_panel_two`: Writes the character for two into a panel selection byte, sets a flag in the structure, and calls two other routines.
- `cycle_animated_entries`: Walks seven entries, decrementing each one's delay counter.
- `advance_frame_and_decrement_counters`: Advances a four-bit frame counter and reloads a delay, then calls another routine.
- `handle_name_entry_input`: Reads keyboard codes into a name buffer, handling backspace, delete and cursor-movement codes and uppercasing letters.
- `initialise_panel_dimensions`: Sets fixed dimension fields in a structure, points a buffer field at a global address, clears an offset field, and calls another routine whose result is stored in a pointer field.
- `forward_to_routine_3`: Calls one other routine and returns its value unchanged.
- `copy_interleaved_table_rows`: Copies a word table into an interleaved buffer across eighty groups, writing fourteen words per group at a fixed stride.
- `process_name_keyboard_input`: Processes keyboard input into a name buffer, handling backspace, delete, and cursor-movement codes while uppercasing letters, and writes the resulting name to a fixed location.
- `draw_glyph_string_to_bitplanes_2`: Decodes a stream of byte-coded glyph data and copies each character's rows into the four bitplanes of a target bitmap.
- `draw_glyph_string_to_bitplanes_3`: Copies a byte-coded string of characters into four bitplanes of a bitmap.
- `update_entry_mask_bits`: Clears a block of mask memory, then for each active entry moves a bit from one mask plane to another at a position derived from the entry's coordinates.
- `build_row_pointer_table`: Returns early when a flag is set.
- `call_routine_then_stored_handler`: Calls one routine and then calls a routine through a stored pointer before returning.
- `toggle_state_byte_and_call`: When a flag is set, toggles one of two state bytes depending on whether a stored position lies past a threshold.
- `decompress_bitstream_into_buffer`: Decodes a packed bit stream, read backwards, into a buffer.
- `shift_bit_reader_by_count`: Shifts a 32-bit bit buffer right by a count, pulling in a new word from memory whenever the buffer empties.
- `unpack_packbits_body_chunk`: Searches for a body data chunk and decodes its run-length packed rows into a bitmap.
- `combine_three_planes_with_or`: ORs three source bitplanes word by word into a destination buffer over a fixed count.
- `unpack_packbits_body_chunk_2`: Searches for a body data chunk and decodes its run-length packed rows into a bitmap, using a different row width and row count from the other unpacking variant.
- `unpack_packbits_body_chunk_3`: Searches for a body data chunk and decodes its run-length packed rows into a bitmap, advancing rows by a stored stride.
- `wait_raster_line_then_reset`: Polls the vertical beam position up to a fixed number of times, waiting for it to pass a threshold.
- `run_shutdown_sequence`: Calls a long sequence of other routines, then calls two routines through stored pointers and finishes with more other routines.
- `decompress_level_data_stream`: Decodes a packed bit stream of level data, read backwards, into a buffer.
- `move_entry_across_bit_map`: Moves an entry across a bitplane map one step at a time.
- `do_nothing_stub_3`: The decompiled body is a bare return, so no behaviour is visible.
- `advance_entry_when_timer_expires`: Decrements a stored delay counter and, once it reaches zero, sets up the entry's state fields and steps through a list of entries, calling other routines as needed.
- `initialise_entry_motion_state`: Writes a masked flag word with a fixed bit set, sets two offset fields to fixed values, and clears two counters in an entry structure.
- `clear_mask_bits_four_planes`: ANDs a mask word into four words of bitplane memory at fixed row offsets, clearing the masked bits in each.
- `copy_entry_pixel_bytes_rows`: Computes a bitmap offset from a position, copies three bytes of an entry's pixel data into three rows spaced at fixed strides, and stores a flag word.
- `apply_map_code_to_entry`: Reads a map code at the entry's position and applies its effect.
- `initialise_entry_position_limits`: Sets position offset fields and fixed initial values for an entry, selects a horizontal limit depending on a mode flag, and calls other routines to finish the setup.
- `draw_active_entries_with_blitter`: Loops over the active entries that fall within the visible range, computes each one's screen position and frame from its state flags, and writes each sprite to the bitmap through blitter passes, waiting for the blitter between passes.
- `update_bit_planes_scan_entries`: Sets or clears one bit at a position across four bitplanes, then walks a list of entries.
- `process_active_entries_blitter_2`: Clears and sets specific flag bits in a hardware register block, then walks a list of active entries, updating counters and validating their values before either invoking a per-entry routine or computing a table offset to dispatch based on a decoded index.
- `manipulate_bit_planes_and_update`: Clears and sets a bit at a calculated offset within a structure pointed to by a3, then enters a loop that iterates through a table of structures to perform bitwise operations and calls blit_tile_rows_with_masks or update_bit_planes_scan_entries based on specific bit flags and coordinate checks.
- `set_multiple_bits_and_branch`: Sets bit 0 of a byte at offset 8448 from a pointer in a3 three times at different offsets, clears the bit at a fourth offset, and then performs a loop based on the value in d0 before branching to another routine.
- `modify_bit_planes_and_update_scanlines`: Clears a bit at a calculated offset in three separate memory locations relative to a pointer in a3, then enters a loop that iterates through a table to perform conditional updates to bit planes and scanline entries via a jump table.
- `manipulate_bit_planes_and_update_tiles`: Sets and clears a bit at a calculated offset within a structure pointed to by a3, then enters a loop that iterates through a table to perform bitwise operations and calls a routine to update bit planes based on a jump table.
- `toggle_bits_pattern_2`: Clears a bit at a calculated offset within a byte at offset 8448 from a pointer in a3, then sets it, then clears it, then sets it again, and performs a loop based on the value in d.
- `set_double_clear_bit_2`: Sets bit 0 of a byte at offset 8448 from a pointer in a3 twice, then clears it, then sets it again, and performs a loop based on d0 before returning.
- `modify_bitplane_data`: Clears a specific bit in three separate bytes at fixed offsets from a pointer in a3, sets the same bit in a fourth byte at a different offset, and then enters a loop that iterates based on the value in d0 before branching to another routine.
- `manipulate_bit_planes_and_update_tiles_2`: Sets a bit in a byte at an offset from a pointer in a3, clears it twice at two other offsets from the same pointer, and then sets it again.
- `toggle_bits_pattern_3`: Clears a bit at a calculated offset within a byte at offset 8448 from a pointer in a3, then sets and clears that same bit, then sets and clears it again, performing a loop based on a value in d.
- `set_double_clear_bit_3`: Sets bit 0 of a byte at offset 8448 from a pointer in a3 twice, then clears it, then sets it again, and performs a loop based on d0 before returning.
- `modify_bit_planes_and_update_tiles`: Clears a bit at a calculated offset in a byte at a specific offset from a pointer, then sets the same bit at two other offsets from that pointer.
- `process_active_entries_blitter`: Clears and sets specific flag bits in a hardware register block, then walks a list of active entries, updating counters and validating their positions against a range.
- `set_bits_alternating`: Sets bit 0 of a byte at offset 8448 from a pointer in a3, then sets the same bit, then clears it, then clears it again, and performs a loop based on d0 before branching to another routine.
- `process_active_entries_flags`: Clears and sets specific flag bits in a large bit-plane buffer, then walks a list of active entries, updating counters and computing offsets that select a handler from a jump table based on computed position values.
- `process_and_draw_entries`: Sets four flag bits in a large bitmap through pointer offsets, then walks a list of entry records, updating counters and computing offsets from their fields.
- `clear_bits_alternating`: Clears bit 0 of a byte at offset 8448 from a pointer in a3, then sets it, then sets it again, then clears it, and performs a loop based on d0 before branching to another routine.
- `set_multiple_bits`: Sets bit 0 of a byte at offset 8448 from a pointer in a3 three times, then clears it, and performs a loop based on d0 before branching to another routine.
- `clear_multiple_bits`: Clears bit 0 of a byte at offset 8448 from a pointer in a3 three times, then sets it, and performs a loop based on d0 before branching to another routine.
- `set_and_clear_bits`: Sets bit 0 of a byte at offset 8448 from a pointer in a3, then clears it twice, then sets it again, and performs a loop based on d0 before branching to another routine.
- `toggle_bits_pattern`: Clears bit 0 of a byte at offset 8448 from a pointer in a3, then sets it, then clears it, then sets it again, and performs a loop based on d0 before branching to another routine.
- `set_double_clear_bit`: Sets bit 0 of a byte at offset 8448 from a pointer in a3 twice, then clears it, then sets it again, and performs a loop based on d0 before branching to another routine.
- `clear_double_set_bit`: Clears bit 0 of a byte at offset 8448 from a pointer in a3 twice, then sets it twice, and performs a loop based on d0 before branching to another routine.
- `set_mixed_bits`: Sets bit 0 of a byte at offset 8448 from a pointer in a3, then clears it, then sets it twice, and performs a loop based on d0 before branching to another routine.
- `set_triple_bits`: Clears bit 0 of a byte at offset 8448 from a pointer in a3, then sets it three times, and performs a loop based on d0 before branching to another routine.
- `set_multiple_bits_2`: Sets bit 0 of a byte at the address in a3, increments a3 by 8496, sets bit 0 of the new address, increments a3 again, sets bit 0 of the new address, increments a3 a third time, sets bit 0 of the new address, and then loops based on d0 before branching to another routine.
- `queue_blit_for_entry_tile`: Looks up a tile from an entry's position, computes an address in the bitmap, and writes four groups of row pointers into the blitter setup.
- `queue_bitplane_blit_rows`: Rounds a stored offset to a 16-bit boundary to derive a shift, then writes four groups of row pointers into the blitter setup.
- `store_blit_offsets`: Computes a byte offset from a value read from a global structure and stores the adjusted source pointer.
- `draw_character_glyphs_to_screen`: Walks a string until a negative byte, copying the glyph for each non-space character from a font table into a screen buffer.
- `apply_routine_to_24_rows`: When a mode flag is clear, loops over twenty-four rows and calls another routine on each row's data and destination pointer.
- `pack_column_mask_bits`: Advances a row index stored in a structure, then for twenty rows packs eight source bytes into one byte.
- `pack_column_mask_bits_2`: For twenty rows, packs eight source bytes into one byte.
- `clear_four_plane_word_columns`: Walks a run of 408 positions, zeroing a word in each of four bitplane arrays at fixed strides, which clears those columns in every plane.
- `convert_bitplane_words_to_pixels`: Expands each 32-bit bitplane word into byte values by weighting selected bit positions, then writes four results into destination rows at fixed offsets.
- `start_blit_four_times_waiting`: Sets the blitter fields and writes a fixed blit size word four times, waiting for the blitter to finish after each write.
- `paint_edge_masks_for_column`: Computes a column offset from a stored value and ORs an edge mask into two bitplane words.
- `clear_plane_edge_bits_rows`: Loops over 160 rows, clearing and setting edge bits in the first and last words of four bitplanes.
- `advance_scroll_and_blit_rows`: Advances a scroll offset by four units and, until it reaches its limit, copies row data through the blitter.
- `advance_frame_index_call_routines`: Increments a stored index with wraparound at thirty-two, then calls two other routines.
- `store_bitmap_row_geometry`: Stores a pixel width, that width divided by eight as bytes per row, a height value, and the product of bytes per row and height into global variables.
- `store_two_global_words`: Stores its two 16-bit inputs into a pair of global variables.
- `blit_tile_rows_with_masks`: Draws a rectangle of tiles into the bitmap through the blitter.
- `clip_rectangle_to_bounds`: Clips a rectangle against limits stored in a structure.
- `call_six_routines`: Calls six other routines in sequence and returns, with no other work visible in the code.
- `store_half_sizes_and_pointers`: Looks up a table entry by index after calling another routine, stores half of two size fields from an input record into it, and stores two pointers to that record's data at fixed positions.
- `jump_to_walk_linked_list_calling_routi`: Jump stub: passes control straight on to walk_linked_list_calling_routine.
- `jump_to_store_three_bytes_and_pointers`: Jump stub: passes control straight on to store_three_bytes_and_pointers.
- `jump_to_process_pointer_list`: Jump stub: passes control straight on to process_pointer_list.
- `jump_to_call_three_routines`: Jump stub: passes control straight on to call_three_routines.
- `jump_to_call_two_routines_3`: Jump stub: passes control straight on to call_two_routines_3.
- `jump_to_copy_slot_bytes_to_outputs`: Jump stub: passes control straight on to copy_slot_bytes_to_outputs.
- `jump_to_call_five_routines_2`: Jump stub: passes control straight on to call_five_routines_2.
- `setup_audio_channels_from_mask`: For each of four bits in the input mask, clears that audio channel's DMA bit, points the channel at a sample address with a length of one, and records the channel in a state byte.
- `call_four_routines`: Calls four other routines in sequence and returns.
- `branch_to_offset`: Performs a relative branch to a specific offset.
- `branch_to_offset_2`: Performs a relative branch to a specific offset.
- `update_table_bit_mask`: Decrements a value in register d0, masks it to eight bits, and stores a byte with only the highest bit set into a table at an offset determined by the new value in d0.
- `write_zero_to_indexed_memory`: Decrements a value in a register, uses the result as an index into a table to calculate a destination address, and stores a zero byte at that location before returning.
- `clear_four_longwords`: Clears four consecutive 32-bit words in a table starting from a memory address held in a4, then returns.
- `call_routines_when_flag_clear`: Calls three other routines, then calls four more only when a stored flag is zero.
- `disable_audio_dma_channels`: Calls another routine, sets a stored flag to one, then clears the DMA enable bits for the first four audio channels.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `loop_forever_setting_field`: Repeats forever, writing a fixed value to one field of a structure, so the routine never returns.
- `check_checksum_and_patch_code`: Compares the sum of four stored words against a fixed check value and, when they differ, overwrites a code word with a no-operation instruction.
- `start_cia_timer_with_interrupt`: For one of two input codes, sets two pointer fields and programs a CIA timer: it writes the interrupt control bits and starts the timer at its maximum count.
- `reset_channel_state_block`: Stores an input value, clears a block of state bytes in a structure, then sets four counter fields to a fixed value of sixty-three.
- `call_three_routines`: Calls three other routines in sequence and returns its input value unchanged.
- `store_half_sizes_and_pointers_2`: Reads a value from a table using an index and calls another routine to locate a four-byte sequence in memory.
- `find_four_byte_sequence`: Scans forward through memory until it finds a four-byte sequence equal to the input value, and returns when the end address is reached.
- `do_nothing_stub_4`: The routine is a bare return with no operations visible in the decompiled code.
- `walk_linked_list_calling_routine`: Walks a zero-terminated linked list, calling another routine on each node and following its link until a zero link is reached.
- `return_argument_unchanged`: Returns the value passed on the stack without any other operation.
- `store_three_bytes_and_pointers`: Uses an index to look up a table slot, stores three bytes from an input pointer into it, and writes a pointer located nine bytes before that input into four fixed-offset fields.
- `process_pointer_list`: Walks a zero-terminated list by following pointers, invoking a helper routine once for each entry until it reaches the terminating null.
- `return_stack_value`: Returns the value passed to it in the stack return address without performing any other operation.
- `call_two_routines_3`: Invokes two other routines in sequence and then returns without modifying any data.
- `call_four_routines_2`: Calls four other routines in sequence and returns its input value unchanged.
- `reset_entry_slot_fields`: Copies a stored word into the first field of a structure, sets one byte to all ones, and clears two more bytes.
- `copy_slot_bytes_to_outputs`: Looks up a slot by index and calls four other routines.
- `rotate_byte_right_by_one`: When bit zero of the input is set, it stores a pointer, clears a word and a byte in a structure.
- `call_five_routines_2`: Calls five other routines in sequence and returns, with no other work in this body.
- `store_packed_bytes_into_entry`: When bit zero of the flags is set, copies three bytes of a 32-bit input into fields of a structure, copying the top byte only when a sub-flag is clear, and clears one more byte.
- `configure_audio_channels`: Clears the audio DMA enable bits, then for each set bit in the input it enables the matching channel, records the bit in a status byte, copies a shared value into that channel's pointer register, and sets its length to one.
- `call_four_routines_3`: Invokes four other routines in sequence and then returns without modifying any data.
- `initialise_entry_byte_fields`: When bit zero of the flags is set, writes one to one byte of a structure, copies the input byte into two adjacent bytes, and sets a fourth byte to all ones.
- `get_and_clear_byte`: Calculates an offset using a value from a table, reads a byte from that location, sets the sign flag based on that byte, and then clears the byte at that same location.
- `read_table_byte`: Decrements d0, masks it to 8 bits, and reads a byte from a table at an offset determined by the new d0 value.
- `set_table_byte`: Decrements d0, masks it to 8 bits, and stores a byte containing only the high bit set into a table at an offset determined by the new d0 value.
- `check_table_sign`: Decrements d0, masks it to 8 bits, and sets the sign flag based on the byte at the resulting offset in a table.
- `clear_table_words`: Loops four times, clearing a 32-bit word at each increment of a pointer starting at the address in a4.
- `invoke_routines_conditional`: Invokes a helper routine three times, and when a global flag is zero it additionally calls another target routine four times before returning both input values.
- `decrement_counter_or_reload_five`: When the stored counter is zero it reloads it to five.
- `invoke_routine_three_times`: It calls another routine three times in a row and then returns, performing no data manipulation itself.
- `copy_three_table_bytes`: When a flag in a referenced structure is clear, copies three consecutive bytes from a table, starting at an index, into three fields of the current structure.
- `self_recursion_triple`: Performs a self-referential call to itself three times in succession and then returns.
- `advance_frame_when_delay_expires`: Decrements a delay counter in a referenced structure, and once it expires advances a word at the start of that structure by twelve.
- `rotate_table_byte`: Reads a byte from a table using a register index, rotates it left by one bit, and stores the result back into the same table entry.
- `copy_entry_record_and_rotate`: When a stored word is nonzero, copies a record of several fields to another location, writes the input word into a register field, clears the source word and sets a flag byte in another structure.
- `call_target_four_times`: Calls one target routine four times in a row with no arguments and then returns.
- `update_countdown_and_step_index`: Calls one routine first, then counts down a byte timer in a record.
- `copy_or_default_pair_out`: When a flag byte in the record is clear, copies a stored long value and a count word to the output record if the count is nonzero.
- `advance_animation_frame`: Loads a new animation frame record from a table indexed by a counter into the current record when a flag byte is set, then stores the incoming word through a pointer.
- `advance_animation_frame_entry`: If a flag byte is set, it stores the incoming word and loads a new frame record from a table, indexed by a counter, into the current record, then clears the flag.
- `dispatch_via_table`: Reads a selector byte from a record, clears that byte, and uses a second byte as an index into a table of offsets to call the matching handler.
- `dispatch_via_jump_table_entry`: Reads a selector byte from a record, clears it, and uses a second byte as an index into a table of offsets to call the matching handler.
- `empty_stub_routine`: Does nothing and returns immediately, so it is an empty stub with no body.
- `set_global_flag_bit_one`: Sets the second-lowest bit of a shared global variable and returns.
- `clear_fire_bit_2`: Clears bit 1 of the CIAA Port A Data Register and returns.
- `decrement_and_sync_offset_values`: Checks if a byte at offset 31 of a structure in a1 is zero; if so, it subtracts 3 from a 16-bit value in d0 and stores the result into offsets 8 of both the structure in a1 and another structure in a3.
- `update_status_flag_bitmask`: Compares a value in a1 against four specific offsets within a structure in a5 and writes a corresponding bitmask value to a 16-bit word at offset 240 of a structure in a6.
- `modify_audio_config_byte`: Checks a byte at offset 13 of a structure pointed to by a2; if that byte is zero, it subtracts 86 from the value in d0 and stores the result at that same offset, otherwise it returns.
- `update_status_bytes_offset_28`: Subtracts 166 from a byte in register d0, stores zero at offset 28 of the structure in a1, and then stores the modified byte into offsets 29 and 30 of that structure.
- `update_status_bytes_2`: Stores 1 at offset 28 of the structure in a1, then stores the byte from d0 plus 80 into offsets 29 and 30 of that structure.
- `set_table_byte_3`: Stores a byte of 255 at the address calculated by adding the byte from d0 plus 69 to offset 282 of the structure in a5.
- `store_byte_at_offset_2`: Stores a byte of 255 at offset 27 of the address held in a1.
- `clear_byte_at_offset_27`: Clears a byte at offset 27 of the memory location pointed to by a1, then branches to another routine.
- `clear_structure_longwords`: Clears five longwords starting at offset 282 of the structure pointed to by a5 using a loop.
- `process_conditional_update_2`: If the byte at offset 3 of the structure in a2 is 255, it copies a longword from the start of the structure to offset 4 and stores the byte from d0 plus 47 at offset 3.
- `branch_to_next_routine`: Performs a branch to another routine.
- `update_structure_offset_158`: Stores the word 32769 into a structure at offset 158 relative to a pointer in a6, then branches to another routine.
- `update_structure_offset_158_2`: Stores the word 32770 into a structure at offset 158 relative to a6, then branches to another routine.
- `update_structure_value`: Stores the word 32772 at offset 158 of the structure pointed to by a6, then branches to another routine.
- `write_status_32784`: Writes the word 32784 to offset 158 of the structure in a6 and branches to another routine.
- `write_status_32800`: Writes the word 32800 to offset 158 of the structure in a6 and branches to another routine.
- `write_status_32832`: Writes the word 32832 to offset 158 of the structure in a6 and branches to another routine.
- `clear_table_byte_2`: Sets a byte to zero at a calculated index within a table pointed to by a5, using a byte from d0 and an offset of 282, then branches to another routine.
- `write_status_1_2`: Writes the word 1 to offset 158 of the structure in a6 and branches to another routine.
- `write_status_2_2`: Writes the word 2 to offset 158 of the structure in a6 and branches to another routine.
- `write_status_4_2`: Writes the word 4 to offset 158 of the structure in a6 and branches to another routine.
- `write_status_16_2`: Writes the word 16 to offset 158 of the structure in a6 and branches to another routine.
- `write_status_32_2`: Writes the word 32 to offset 158 of the structure in a6 and branches to another routine.
- `set_global_flag_bit_one_2`: Sets the second-lowest bit of a shared global variable and returns.
- `clear_fire_bit`: Clears bit 1 of the CIAA_PRA register and returns.
- `update_offset_values`: Checks a byte at offset 31 of a structure in a1; if it is zero, it subtracts 3 from d0 and stores the resulting word into offsets 8 of both the structure in a1 and another structure in a3.
- `update_status_bytes`: Subtracts 166 from a byte in d0, stores 0 at offset 28 of the structure in a1, and then stores the modified d0 byte into offsets 29 and 30 of that structure.
- `update_status_bytes_alt`: Subtracts 176 from a byte in d0, stores 1 at offset 28 of the structure in a1, and then stores the modified d0 byte into offsets 29 and 30 of that structure.
- `set_table_byte_2`: Subtracts 187 from a byte in d0, masks the result to 16 bits, and stores a byte at the address calculated by adding the result to an offset of 282 in a5.
- `store_byte_at_offset`: Stores a byte at offset 27 of the address held in a1.
- `clear_byte_at_offset`: Sets the byte at offset 27 of the address held in a1 to zero.
- `clear_memory_block`: Clears 5 longwords starting at offset 282 of the structure in a5 using a loop.
- `process_conditional_update`: Compares a byte at offset 12 of the structure in a2 to 255; if it is not 255, it branches to another routine, otherwise it copies a longword from offset 0 to offset 4 of the structure and stores a modified d0 byte at offset 12.
- `check_and_reset_byte`: Tests a byte at offset 12 of the structure in a2; if the byte is non-zero, it branches to another routine, otherwise it sets that byte to 255 and returns.
- `decrement_and_copy_structure`: Decrements a byte at offset 12 of the structure in a2 and copies a longword from offset 4 to offset 0 of the structure.
- `subtract_from_longword`: Subtracts 12 from a longword at offset 0 of the structure in a2 and returns.
- `write_value_8001`: Stores the word 32769 at offset 158 of the structure in a6.
- `write_value_8002`: Stores the word 32770 at offset 158 of the structure in a6.
- `write_value_8004`: Stores the word 32772 at offset 158 of the structure in a6.
- `write_status_8010`: Writes the word 32784 to offset 158 of the structure in a6 and returns.
- `write_status_8020`: Writes the word 32800 to offset 158 of the structure in a6 and returns.
- `write_status_8040`: Writes the word 32832 to offset 158 of the structure in a6 and returns.
- `clear_table_byte`: Subtracts 229 from a byte in d0, masks the result to 8 bits, and sets the byte at that index within a table pointed to by a5 plus an offset of 282 to zero.
- `write_status_1`: Writes the word 1 to offset 158 of the structure in a6 and returns.
- `write_status_2`: Writes the word 2 to offset 158 of the structure in a6 and returns.
- `write_status_4`: Writes the word 4 to offset 158 of the structure in a6 and returns.
- `write_status_16`: Writes the word 16 to offset 158 of the structure in a6 and returns.
- `write_status_32`: Writes the word 32 to offset 158 of the structure in a6 and returns.
- `update_status_flag`: Compares a1 with four different offsets of a structure in a5 and writes a specific word value to offset 240 of a6 depending on which match is found.
- `update_audio_configuration`: If a byte at offset 13 of a structure in a2 is zero, it subtracts 86 from d0 and stores the result at that offset; otherwise it calls another routine with d0 set to 15, writes 1 to a memory location, writes 15 to offset 240 of a pointer to hardware registers, and returns.
- `set_hardware_bits`: Sets bit d1 in a byte at offset 48 of a table, sets bit d1 in a byte at an offset calculated by multiplying d0 by 2, sets bit d1 in a byte at an invalid address, performs an OR operation with 240 on a byte at an invalid address, performs an OR operation with 48 on a byte at an invalid address, and performs an OR operation with 240 on a byte at an offset calculated by multiplying d0 by 2.

### Holiday Lemmings 1993 (1993)(DMA Design).adf:TRAINER!

- `unpack_rnc_block_list`: Walks a linked list of memory blocks looking for one tagged as RNC-compressed, then decodes it with literal copies and back-references into newly allocated memory.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a masked global value matches the second word of a pair.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Ands a word with a mask word, and if a byte value is less than another byte it calls another routine and returns that routine's result instead of the masked word.
- `return_empty_routine_3`: This routine performs no operations; it contains no instructions that modify memory or registers and immediately returns.
- `build_bit_length_tables`: Reads a count and fills a byte buffer on the stack with values from a source routine.
- `process_pending_entry_chain`: When a header value matches a fixed marker, it walks a chain of pending entries, clears them, and writes a computed offset into a table slot for each entry.
- `return_empty_routine`: Contains no operations and returns immediately, so calls to it have no effect.

### Holiday Lemmings 1993 (1993)(DMA Design).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
