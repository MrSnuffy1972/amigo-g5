# Game 1757 (another world (1991)(u. s. gold)(fr)[cr company]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01757). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802640 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 116 of 116 named (0 author symbols), 116 of 116 described (4 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1738 of 1738 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 924354 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3419 of 3419 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 561016 |
| code | 11984 |
| data | 1071634 |
| padding | 5430 |
| unused | 152576 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:intro.comp | 2 | 233 | 8 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:another | 1 | 3087 | 106 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 89 | 1 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:bank0D (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:bank01 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:bank02 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 2 of 2).adf:bank0A (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 2 of 2).adf:bank03 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 74 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- another world (1991)(u. s. gold)(fr)[cr company](disk 1 of 2)/Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:bank0D: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)(fr)[cr company](disk 1 of 2)/Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:bank01: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)(fr)[cr company](disk 1 of 2)/Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:bank02: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)(fr)[cr company](disk 2 of 2)/Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 2 of 2).adf:bank0A: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)(fr)[cr company](disk 2 of 2)/Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 2 of 2).adf:bank03: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)(fr)[cr company](disk 2 of 2)/Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 5 never-loaded region(s), 924354 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:intro.comp

## Copy protection found

- `open_library_and_control_hardware` (runtime segment an address): No pseudo-code was produced because the decompiler timed out, so the behaviour is not recovered. Kind: disk hardware/integrity check. Pass path: The routine likely performs a hardware-level validation (CIAB_PRB) of the disk media; a successful check would proceed to the next instruction in the calling function, whereas a failure would likely trigger a loop, a jump to an error messag.

## Functions


### Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:intro.comp

- `load_executable_hunks`: Walks the AmigaDOS hunk records of an executable, comparing each record type with the hunk type codes.
- `delay_by_count_word`: Counts the incoming word down in a loop until it passes zero, so the loop length scales with the input value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_backward_compressed_stream`: Decodes a bit-packed compressed stream into a buffer by writing from the end toward the start, alternating between literal bytes and back-references that copy a run of already written bytes.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bit_buffer_by_count`: Repeats once per unit of the incoming word count, shifting a bit buffer by the count held in a register and loading the next value from memory when the buffer empties.
- `stub_undecoded_three_instructions`: The decompiler could not finish this three-instruction routine, so its body is not recovered.

### Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:another

- `startup_timing_setup_wait`: Runs a series of setup calls, sets a timing value of 300 or 2400 when a status bit is set, waits until another status bit clears, clears a counter, and then runs more setup calls.
- `dispatch_code_to_slot_tables`: Matches an incoming code against a set of fixed values and, for a match, assigns slot indices and stores values read from a record into slot entries.
- `dispatch_code_after_table_loop`: Enters the same slot-table dispatch as its neighbour, matching the code at the point after the table loop and reusing the same assignment and record-store steps.
- `run_bytecode_script_interpreter`: Interprets a byte-coded script, returning when an end opcode is reached or the read position leaves a bounded range.
- `resume_script_if_equal`: Enters the same bytecode interpreter at a point that continues directly when the zero flag from the previous comparison is set, meaning the two compared values were equal.
- `resume_script_if_not_equal`: Enters the same bytecode interpreter at a point that continues directly when the zero flag from the previous comparison is clear, meaning the two compared values differed.
- `resume_script_if_greater`: Enters the same bytecode interpreter at a point that continues directly when the zero, overflow and negative flags from the previous comparison show a signed greater-than result.
- `resume_script_if_not_less`: Enters the same bytecode interpreter at a point that continues directly when the overflow and negative flags from the previous comparison show a signed greater-or-equal result.
- `resume_script_if_less`: Enters the same bytecode interpreter at a point that continues directly when the overflow and negative flags from the previous comparison show a signed less-than result.
- `resume_script_if_less_equal`: Enters the same bytecode interpreter at a point that continues directly when the zero flag is set or the overflow and negative flags show a signed less-than result.
- `apply_plane_operation_record`: Dispatches a drawing operation record.
- `walk_entries_by_high_bit`: Walks a counted list of entries whose length is chosen by the top bit of two words in each entry, and calls a routine for every entry it visits.
- `compute_record_address_by_index`: Returns the address of a twenty-byte record in a global table, located by the low word of the input index.
- `print_text_entry_for_key`: Scans a table of word-keyed entries for the key held in an input register and, when one matches, passes each character of the zero-terminated text that follows it to a routine, skipping carriage-return bytes.
- `draw_eight_row_tile_bounded`: Checks that a position lies within a 40 by 193 byte area, then for eight rows combines a pattern byte into four bitplane locations.
- `depack_bitstream_backward_copies`: Depacks a bit-packed stream into a buffer working from the end toward the start.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_window_right`: Shifts a 32-bit bit window right by the count held in the input register.
- `select_highest_priority_object`: Scans a table of twenty-byte records for entries whose type byte is two, keeping the one with the highest priority byte and stopping at the end marker.
- `format_word_as_digit_chars`: Turns a 16-bit value into four characters, one for each nibble from the top, using the digits 0-9 or the letters A-F. It returns the last character it produced.
- `mark_heaviest_records_within_budget`: Scans the twenty-byte record table for type-three records whose top flag bit matches the argument, picking the one with the largest byte at offset six each time.
- `mark_records_within_budget`: Marks the record it is given as type four with the flag bit and subtracts its amount at offset sixteen from a running value.
- `process_type4_records_adjust_counters`: Walks the record table until the end marker.
- `process_type4_records_from_pointer`: Subtracts a given value from one global counter, then walks the record table from a starting record, handing each type-four record to a helper.
- `return_without_effect`: Only returns, so it changes no memory or registers.
- `move_block_and_adjust_pointers`: Copies a block of words from a source address to a destination address, in blocks of 32768 words.
- `copy_words_then_adjust_pointers`: Copies a counted run of words from a source to a destination, then subtracts a passed-in distance from the offset-two pointer of every record beyond a threshold whose top flag bit matches the argument.
- `copy_word_blocks_adjust_pointers`: Copies words from a source to a destination in fixed blocks of 32768 words, repeated for a count held in a register.
- `return_empty_stack_value`: This routine is empty and performs no operations before returning.
- `dispatch_timed_codes_to_records`: Loops indefinitely over a pass that clears a 128-entry word array and 32 longs in the global area, then compares the current code against nine fixed values.
- `clear_out_of_range_records`: Walks the twenty-byte record table for the stored record count, zeroing the first byte of each record whose second byte lies outside three to six.
- `clear_all_records_restore_values`: Zeroes the first byte of every twenty-byte record for the stored record count.
- `count_table_entries_to_sentinel`: Counts twenty-byte table entries until one has a first word of minus one, then stores the count of entries before that sentinel, minus one, in a global.
- `draw_line_with_blitter`: Sets up the blitter in line mode for a line between two screen points on a 40-byte-per-row bitmap.
- `clip_line_then_draw`: Computes an outcode for each endpoint of a line and returns without drawing when both endpoints lie outside the same screen edge.
- `return_only_stub`: Contains only a return, so it changes no memory or registers.
- `subdivide_segment_at_midpoint`: Repeatedly halves a segment at its midpoint, passing each piece to a helper.
- `fill_polygon_scanline_spans`: Scales and offsets each byte-coded vertex, then walks consecutive vertex pairs through a helper, recording the left and right crossing x positions for each scanline in a buffer.
- `blit_polygon_span_fill`: Walks the vertex list the way the span builder does, scaling and offsetting each vertex and recording scanline crossings, then passes each span pair on for drawing.
- `fill_draw_record_then_wait`: Writes its arguments into a draw record at offset 64 of a given base pointer, together with fixed values, then waits until the blitter is idle.
- `wait_for_blitter_idle`: Polls the blitter busy bit in the DMA status register until the blitter is idle, then returns.
- `decode_joystick_direction_flags`: Reads a stored input word holding joystick movement and fire-button bits, and derives two direction values and a button flag from it.
- `copy_four_longwords_to_globals`: Calls a helper first, then copies four 32-bit values from the record it points to into four global variables.
- `scale_small_value_by_sixteen`: Returns the input word unchanged when it is above three; otherwise it returns the word multiplied by sixteen.
- `return_without_effect_2`: Only returns, so it changes no memory or registers.
- `draw_clipped_shifted_object`: Draws one object from the object table after checking that it lies within the screen horizontally and vertically.
- `mask_four_word_targets`: ANDs four 16-bit words, each reached through its own pointer, with a mask value, clearing every bit not set in the mask.
- `mask_three_words_return_code`: ANDs three 16-bit words, each reached through a pointer, with a mask value.
- `apply_mask_bits_to_planes`: Walks up to four stored bitplane pointers.
- `allocate_memory_or_fail`: Calls the system memory allocation routine through the exec library vector, and returns minus one when it gives back a null pointer.
- `free_memory_or_fail`: Calls the system memory-free routine through the exec library vector, then returns minus one when the call's result is zero.
- `queue_row_draw_commands`: Writes draw-command records for a span of screen rows, each holding fixed mode and mask words and offsets derived from the vertical position.
- `apply_bit_mask_four_words`: Checks four bits of a mask value in turn.
- `switch_display_mode_copper`: Changes the display for a mode code: a sentinel code swaps two sets of four longs, while other codes copy new values.
- `dispatch_keyboard_scancode`: Decodes the keyboard serial byte into a key code by rotating its bits, then maps certain keys to display window and playfield register settings, a timed event code, or stored flags.
- `set_display_window_registers`: Writes fixed display window, fetch and playfield control values for one display mode, zeroes the modulo registers, and clears a stored flag.
- `set_display_window_zeroing_modulo`: Writes fixed display window, fetch and playfield control values with a different playfield control word, zeroes the modulo registers, and clears a stored flag.
- `clear_playfield_control_words`: Clears the playfield control and modulo words in a register block, relative to a base pointer, and clears a stored flag.
- `set_playfield_modulo_forty`: Clears two playfield control words and sets the two playfield modulo words to 40, the row length used by the other blitter routines, then sets a stored flag to one.
- `reset_input_state_from_table`: Sets one stored word to all ones, copies the low byte of a table entry into another stored word, and clears a third stored word.
- `wait_for_raster_line`: Polls the vertical position register until the raster reaches a fixed line, then returns its input value unchanged.
- `write_dma_control_register`: Writes the DMA control register, where a set top bit enables the channels named in the write, and calls one other routine.
- `lookup_record_when_code_matches`: If its first argument is zero, it stores the second argument into a global, or calls a helper when that is zero.
- `open_ciaa_and_fill_table`: Fills a fifteen-entry table with longwords taken from matching records, zeroing entries that do not match.
- `close_ciaa_resource_if_open`: If the ciaa resource is marked open, calls the exec library's close routine through its vector and clears the open flag.
- `write_ciaa_control_register`: Writes a fixed control value of 129 to the CIA-A control register, then writes a second value read from a stored location into it.
- `copy_stored_value_to_global`: Copies a stored value into a global variable.
- `clear_table_entry_fields`: Walks a fifteen-entry pointer table and zeroes the longword at offset eight of each non-null record.
- `silence_audio_channels`: Sets the volume register of each of the four audio channels to zero, then clears two stored values.
- `silence_audio_channels_set_flag`: Sets the volume register of each of the four audio channels to zero, then sets a stored value to fifteen.
- `call_helper_return_zero`: Calls one other routine and returns zero.
- `advance_audio_channel_counter`: Increments a stored counter.
- `run_routine_per_audio_channel`: Calls a helper once for each of the four audio channel register blocks, counting the channel index down from three, then returns the stack argument unchanged.
- `update_channel_state_or_dispatch`: If a channel's step value is nonzero, adds it to the channel's level and clamps the result between two limits, storing it in the record.
- `step_and_clamp_channel_value`: Adds a step value to a stored word, takes the larger of that result and a lower limit stored at offset four, and stores the result in a record field and back in the word.
- `decode_channel_event_type`: Decodes the type nibble of an event byte.
- `set_event_fields_from_table`: Stores event fields into a record, with a level from the low nibble of a byte, then searches a table of words for the event's value until a sentinel.
- `copy_word_with_signed_offset`: Computes a signed word offset from an input value, with the sign chosen by a flag, and copies a word from a source address offset by that amount to offset four of a record.
- `store_word_into_record_field`: Stores the input word into offset four of the record pointed to by a register.
- `store_signed_event_fields`: Stores an event word into a record, along with a level taken from the low nibble of a byte that is negated.
- `dispatch_via_counter_table`: Calls one entry of a dispatch table, selected by a stored counter minus one.
- `copy_matching_table_entry`: Searches a table of words for the value stored at offset sixteen of a record, then copies the entry at the input index from that position into offset six of the record.
- `copy_high_nibble_table_entry`: Searches the same table for the record's value at offset sixteen, then copies the entry offset by the high nibble of a byte in the record into offset six of the record.
- `copy_low_nibble_table_entry`: Searches the same table for the record's value at offset sixteen, then copies the entry offset by the low nibble of a byte in the record into offset six of the record.
- `copy_word_between_records`: Copies the word at offset sixteen of one record into offset six of another record.
- `start_audio_dma_channels`: Clears a channel-enable word, runs a helper for each of the four audio channels, then writes the enable word with its top bit set to the DMA control register.
- `advance_sound_channel_sequence`: Reads the next step of a channel's sequence, loading a sample pointer, a table value and a volume when the step is a note.
- `finish_channel_sequence_step`: Applies the volume envelope for the channel's current step, raising the volume for type five and lowering it for type six.
- `apply_volume_and_store_step`: Adds an offset to a channel's volume, clamps the result to 64, and stores it in the record.
- `store_channel_step_record`: If the step word is nonzero, it clears a field, copies the step's values into the channel record, waits a short delay, and ORs the channel's flag into a running mask.
- `copy_channel_sample_fields`: Copies a longword and two words from a channel record into the output block, and ORs the record's flag word into a running mask.
- `fill_bitplanes_by_mask`: For each of four bitplanes, fills a block of 8000 longwords with all ones or all zeros, chosen by one bit of an input mask.
- `lock_df0_volume_as_current`: Releases any earlier DOS lock, waits for a keyboard serial code, pauses briefly, then locks the DF0: volume and makes it the current directory.
- `wait_for_disk_status_clear`: Polls the disk-status bit in the CIA port register until it clears, then calls one other routine.
- `read_file_chunk_with_locks`: Opens and seeks within a disk file to read a chunk, keeping the file handle cached between calls and closing it when a different file is requested.
- `own_blitter_return_minus_one`: Calls two library vectors through the exec and graphics bases, then returns minus one unless an ownership flag set in between is true.
- `fill_color_register_delay`: Writes a colour value to the background colour register in a nested countdown loop, so the repeated writes form a timed colour pulse.
- `delay_then_zero_color_register`: Runs a long countdown loop, then writes zero 4096 times to one fixed location.
- `allocate_buffers_open_libs`: Opens the DOS and graphics libraries, reads the free memory size, and lays out sixteen buffers spaced 8000 bytes apart, storing their pointers in globals.
- `toggle_dma_channel_bits`: Writes a DMA control value that clears one group of channel bits, then writes a second value that sets another group together with the enable bit.
- `register_interrupt_server_node`: Allocates an interrupt node through a helper, fills in its data pointer, a priority of 100, a type code of two and a handler routine, then registers the node with the exec library.
- `tiny_addq_clr_rts`: Small routine of 3 instructions that adds 1 to (a1), then clears d0, then returns.
- `forbid_and_own_blitter`: Disables multitasking through the exec library and claims the blitter through the graphics library.
- `permit_and_disown_blitter`: Releases the blitter through the graphics library and re-enables multitasking through the exec library.
- `start_copper_list_from_struct`: Loads the copper list pointer from a field of a stored structure, clears the copper restart register, and enables copper DMA by writing the set bit to the DMA control register.
- `call_four_routines_return_zero`: Calls four other routines in sequence and returns zero.

### Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_library_and_control_hardware`: No pseudo-code was produced because the decompiler timed out, so the behaviour is not recovered.
