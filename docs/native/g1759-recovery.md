# Game 1759 (another world (1991)(u. s. gold)(it)(disk 1 of 2)[cr csl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01759). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 117 of 117 named (0 author symbols), 117 of 117 described (4 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1444 of 1444 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 565294 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3447 of 3447 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 144026 |
| code | 11910 |
| data | 659915 |
| padding | 3861 |
| unused | 81408 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:echo | 2 | 117 | 2 |
| Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:intro.comp | 2 | 233 | 8 |
| Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:another | 1 | 3087 | 106 |
| Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:bootblock | 1 | 10 | 1 |
| Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:bank0D (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:bank01 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:bank02 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- another world (1991)(u. s. gold)(it)(disk 1 of 2)[cr csl]/Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:bank0D: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)(it)(disk 1 of 2)[cr csl]/Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:bank01: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)(it)(disk 1 of 2)[cr csl]/Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:bank02: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 565294 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:intro.comp

## Copy protection found

- none found

## Functions


### Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_header_words_then_call`: If the destination pointer is set, copies sixteen 32-bit words from a fixed record into it, then calls a routine through a function pointer held in a register.

### Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:intro.comp

- `load_amigados_hunk_executable`: Loads an AmigaDOS executable hunk by hunk: it allocates memory for each code and data hunk through the exec library, copies the hunk contents, and applies the 32-bit relocation records until the end-of-file marker.
- `spin_delay_countdown`: Counts a 16-bit register down one step per pass as a fixed-length delay, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_backward_bitstream_block`: Unpacks a compressed block from its end toward its start: it reads a bit stream from the tail of the data and either copies literal bytes or repeats earlier output, writing downward until the output pointer reaches the start of the buffer.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bits_from_stream`: Pulls a counted number of bits one at a time from a bit stream held in a word, refilling the stream from memory when it empties, and shifts them into an accumulator word.
- `store_byte_via_indexed_pointer`: Loads a pointer value from a table using an index and then stores a byte from a register through that pointer.

### Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:another

- `startup_timing_setup_wait`: Startup routine that runs several setup calls, sets a timing value of 300 that becomes 2400 when a configuration bit is set, waits until a status bit clears, clears a counter, and then runs more setup calls.
- `dispatch_code_to_record_slots`: Dispatches on a 16-bit code value, and for each recognised code marks two to four entries of a 20-byte record table and copies record fields for later use.
- `resume_code_slot_dispatch_loop`: Loop variant of the same code dispatch: it resets the word and long tables, scans 64 entries, and dispatches on a 16-bit code value to mark record slots and copy their fields.
- `decode_byte_code_stream_2`: Decodes a stream of byte-code instructions, tracking a program counter and status flags, and executes operations like conditional jumps, comparisons, and table lookups.
- `branch_script_if_equal`: Two-instruction entry that jumps into the shared script opcode loop when the zero flag is set, meaning the compared values were equal.
- `branch_script_if_not_equal`: Two-instruction entry that jumps into the shared script opcode loop when the zero flag is clear, meaning the compared values differed.
- `branch_script_if_signed_greater`: Two-instruction entry that jumps into the shared script opcode loop when the zero flag is clear and the negative and overflow flags agree, which is the signed greater-than test.
- `branch_script_if_signed_ge`: Two-instruction entry that jumps into the shared script opcode loop when the negative and overflow flags agree, which is the signed greater-or-equal test.
- `branch_script_if_signed_less`: Two-instruction entry that jumps into the shared script opcode loop when the negative and overflow flags differ, which is the signed less-than test.
- `decode_byte_code_stream`: Decodes a stream of byte-code instructions, tracking a program counter and status flags, and executes operations like conditional jumps, comparisons, and variable assignments driven by a lookup table.
- `process_shape_draw_command`: Processes one drawing command from a shape stream: for the lower command range it returns, and otherwise it steps through a list of vertices, scaling each offset by a zoom value from a register and building the coordinates for the next step.
- `walk_entries_by_high_bit`: Walks a counted list of entries with a three-word stride, testing the top bit of two words in each entry, and calls an unnamed routine for every entry that is visited.
- `record_address_by_index`: Returns the address of the twenty-byte record at the index held in a register, computed from the base of the record table.
- `print_text_entry_for_key`: Scans a table of word-keyed entries for a matching key and, when one is found, reads the zero-terminated text that follows it, skipping carriage-return bytes and passing each character to an unnamed routine.
- `draw_eight_row_tile_bounded`: Checks that a screen position lies inside a 40-by-193 area, then combines eight pattern rows into the bitmap using OR or AND-NOT, chosen by a bit of a mode word.
- `decode_bitstream_into_buffer`: Decodes a packed bit stream into a destination buffer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_window_right`: Shifts a 32-bit bit window right by a counted number of positions, refilling its top bit from memory whenever the window empties.
- `select_highest_priority_object`: Scans a table of twenty-byte records for entries of type two and keeps the one with the highest priority byte, stopping at the end marker.
- `format_word_as_text_digits`: Converts a 16-bit value into four text characters, one per nibble from the top, using digit or letter characters, and returns the last character.
- `mark_objects_until_budget_spent`: Repeatedly picks the highest-priority record in a twenty-byte table whose type is three and whose sign bit matches a flag, marks it as type four, and subtracts its extent from a budget, stopping when no candidate is left.
- `mark_chain_within_budget`: Marks a given record as type four and subtracts its extent from a budget, then keeps selecting the next type-three record with the same flag until the budget goes negative or no candidate remains.
- `move_marked_records_by_offset`: Walks the twenty-byte record table and, for each record of type four, calls an unnamed routine and subtracts the returned offset from one of two stored values, chosen by the record's sign bit.
- `step_marked_records_by_delta`: Subtracts a delta from one stored value, then walks the record table for type-four records, subtracting an unnamed routine's result from the other stored value until a sign-bit condition ends the walk.
- `return_stack_value_stub`: Returns an eight-byte value taken from the stack, with no other effect.
- `copy_words_to_gap_count`: Copies words from a source buffer to a destination for a count derived from the gap between a source pointer and one of two stored values.
- `copy_words_counted_loop`: Copies words one at a time from a source to a destination, with the count held in a register and decremented until it passes below zero.
- `copy_large_block_repeatedly`: Copies 32,768 longwords, about 128 KB, from a source to a destination in a loop, repeated for a count taken from a register.
- `pass_stack_value_through`: Returns an eight-byte value taken from the stack, with no other effect.
- `dispatch_code_record_slots_alt`: Second copy of the 16-bit code dispatcher.
- `clear_out_of_range_records`: Walks the twenty-byte record table and zeroes the first byte of every record whose type lies outside the range three to six.
- `clear_all_records_restore_values`: Zeroes the first byte of every twenty-byte record in the table, counting down a stored record count, then restores two stored values from backup copies.
- `count_records_until_sentinel`: Counts twenty-byte records in the table until a sentinel word of minus one is reached, and stores the count minus one in a stored value.
- `draw_line_with_blitter`: Sets up the Amiga blitter to draw a line between two screen points on a 40-byte-per-row bitmap.
- `clip_line_then_draw`: Clips a line segment against the screen bounds using edge outcode flags, stores the clipped endpoints, and then calls the line-drawing routine.
- `return_only_stub`: Contains only a return, so it changes no memory or registers.
- `subdivide_segment_at_midpoint`: Recursively splits a line segment at its midpoint, returning when the midpoint equals an endpoint, and calls an unnamed routine for each piece.
- `draw_polygon_outline_from_vertices`: Reads a byte-coded vertex list, scales each vertex by a zoom value, and steps along the polygon edges.
- `fill_polygon_spans_with_blitter`: Fills the spans of a polygon scanline by scanline.
- `init_object_record_fields`: Fills a block of record fields from its parameters: sets one value and a two-count, clears two fields to the maximum word, copies two longwords, and writes a shared value to four further word fields.
- `wait_for_blitter_idle`: Polls the blitter busy bit in the DMA status register until the blitter is idle, then returns.
- `decode_joystick_direction_flags`: Reads the joystick movement register and the fire-button port, combines them with a stored input word into direction and button flags, and stores the result in a state record.
- `copy_four_longwords_to_globals`: Copies four 32-bit values from the pointed-to record into four global variables and returns.
- `scale_small_value_by_sixteen`: Returns the input word unchanged when it is above three; otherwise it returns the word multiplied by sixteen.
- `empty_routine_no_effect`: Contains only a return, so it changes no memory or registers.
- `draw_clipped_sprite_object`: Checks that a sprite object lies within a 320-by-200 screen, clips its top and bottom rows, and draws its columns through a per-mode routine with a bit shift for horizontal position.
- `mask_four_word_targets`: ANDs four 16-bit words, each reached through its own pointer register, with a mask value, clearing every bit not set in the mask.
- `mask_three_words_return_code`: ANDs three 16-bit words, each reached through a pointer register, with a mask value.
- `or_inverted_word_into_planes`: Walks up to four stored plane pointers, and for each bit of a mask value that is set it ORs an inverted word into the plane word at a fixed offset, looping for the plane count.
- `allocate_memory_or_fail`: Calls the exec library's memory allocation routine through its vector and returns minus one when the result is a null pointer.
- `free_memory_or_fail`: Calls the exec library's memory-free routine through its vector and returns minus one when the call result is zero.
- `queue_row_draw_commands`: Builds draw-command records for a span of screen rows.
- `apply_bit_mask_four_words`: Checks four bits of a mask value in turn and calls an unnamed routine for each, passing a full or zero word, which sets or clears one of four stored words.
- `switch_display_mode_copper`: Switches the display for a mode code.
- `set_display_window_for_mode`: Chooses one of several display window and fetch setting sets from a rotated mode byte, writes the window, fetch, and control registers for that set, and clears three stored values.
- `set_default_display_window`: Writes the default display window, data fetch, and control registers, clears four control words and one stored value, and returns.
- `set_display_window_variant`: Writes the display window, data fetch, and control registers using a control word that differs from the default setup in one bit.
- `clear_display_control_words`: Clears four display-control words in the block, along with one stored value, and returns.
- `set_scroll_extents_forty`: Zeroes two control words, sets two others to forty, and sets a stored flag to one.
- `reset_input_state_from_table`: Sets one stored word to all ones, copies the low byte of a table entry into another stored value, and clears a third.
- `wait_for_raster_line`: Polls the vertical position register until the raster reaches a fixed line, then returns its input value unchanged.
- `control_dma_channels_undecoded`: Writes the DMA control register, where a set top bit enables the channels named in the write.
- `lookup_record_when_code_matches`: If its first argument is zero, stores a second argument into a global and returns.
- `open_ciaa_and_fill_table`: Fills a fifteen-entry table with longwords from matching records, zeroing entries that do not match.
- `close_ciaa_resource_if_open`: If the ciaa resource is marked open, calls the exec library's close routine through its vector and clears the open flag.
- `write_ciaa_control_register`: Writes a fixed control value to the CIA-A control register A, then copies a stored byte into a global variable.
- `copy_stored_timer_value`: Copies a stored value into a global variable.
- `clear_table_entry_fields`: Walks a fifteen-entry pointer table and zeroes the longword at offset eight of each non-null record.
- `silence_audio_channels`: Sets the volume register of each of the four audio channels to zero, then clears two stored values.
- `silence_audio_channels_set_flag`: Sets the volume register of each of the four audio channels to zero, then sets a stored value to fifteen.
- `call_routine_return_zero`: Calls an unnamed routine and returns zero.
- `cycle_audio_channel_setup`: Advances a six-step counter.
- `run_routine_per_audio_channel`: Calls an unnamed routine once for each of the four audio channel register blocks, counting the channel index down, and returns the stack argument unchanged.
- `update_channel_envelope_step`: For one sound channel, adds a signed step to a level word and clamps it against a limit, or follows the complementary path, then passes the result to an unnamed routine.
- `clamp_add_channel_level`: Adds an input delta to a stored level word, raises the result to a floor value when it falls below that floor, and writes the result to two destinations.
- `dispatch_channel_effect_table`: Reads the low nibble of a channel control word and, for values of one or seven, dispatches through a jump table of routines indexed by a counter.
- `init_channel_state_from_table`: Initialises a channel state record from a sound command: clears its first word, stores the command value and a four-bit field, then searches a sentinel-terminated table for the value and stores the matching index.
- `store_signed_table_offset`: Stores one table entry into a fixed record field, choosing the table index as twice an input index or its negative according to a flag.
- `store_word_into_record_field`: Stores a 16-bit input value into the fourth word of a record.
- `init_channel_record_with_sign`: Initialises a channel record: clears its first word, stores a value and a four-bit field with the sign taken from the control word, then looks the value up in a sentinel-terminated table.
- `dispatch_through_handler_table`: Calls one of several handler routines chosen by a counter through a pointer table.
- `lookup_table_entry_by_key`: Finds the position of a record's key in a word table, then copies the entry at a register-supplied index into a record field.
- `lookup_entry_by_high_nibble`: Finds the position of a record's key in a word table, then copies the entry at the index given by the high nibble of a control byte into a record field.
- `lookup_entry_by_low_nibble`: Finds the position of a record's key in a word table, then copies the entry at the index given by the low nibble of a control byte into a record field.
- `copy_record_word_to_field`: Copies a 16-bit word from a fixed offset in a record into another field of the same record.
- `start_audio_dma_channels`: Starts the four audio channels: clears a channel-enable word, runs an unnamed routine for each channel, sets the DMA enable bit, waits a short delay, then loads the audio location registers from the channel table.
- `update_channel_note_entry`: Resets a channel record's first two words and, when a flag is set, reads a sample pointer into it.
- `step_channel_volume_envelope`: Sets a channel's flag and step fields, then adjusts the volume level by the parameter.
- `set_channel_level_with_marker`: Adds a parameter to a channel level and clamps the result at sixty-four before storing it.
- `trigger_channel_sample_start`: For an active channel record, clears its flag, copies its sample fields, waits a short delay, then zeroes or sets the length according to the record's marker value.
- `copy_channel_sample_fields`: Copies a longword and two words from a channel record into the output block, and ORs the record's flag word into a running mask.
- `fill_bitplanes_by_mask`: For each of four bitplanes, fills a block of 8,000 longwords with all ones or all zeros according to one bit of an input mask.
- `lock_df0_volume_as_current`: Releases any previous DOS lock, waits on a serial input code, pauses briefly, then locks the DF0 volume through the DOS library and makes it the current directory, storing the lock or closing it when the lock fails.
- `wait_for_disk_status_clear`: Polls the disk-status bit in the CIA port register until it clears, then calls an unnamed routine.
- `load_dos_file_blitter_owned`: Opens a disk file through the DOS library, seeks and reads its contents while multitasking is forbidden and the blitter is owned, then closes the file, releases the blitter, and re-enables multitasking.
- `call_library_pair_and_compare`: Calls two library routines, then when a flag byte is set and a stack argument differs from the second parameter, calls an unnamed routine.
- `fill_color_register_delay`: Writes a colour value to the background colour register in a nested countdown loop, using the repeated writes as a timed colour pulse.
- `delay_then_zero_block`: Runs one long countdown loop, then writes zero to 4,096 consecutive locations.
- `allocate_buffers_open_libs`: Opens the DOS and graphics libraries, reads the free memory size, and lays out a set of buffers spaced 8,000 bytes apart, storing their pointers in globals.
- `toggle_dma_channel_bits`: Writes a DMA control value that clears one group of channel bits, then writes a second value that sets another group with the enable bit.
- `register_interrupt_server_node`: Allocates an interrupt node through a library vector, fills in its data pointer, a priority of one hundred, a type code of two, and a handler routine, then registers the node through the exec library.
- `tiny_addq_clr_rts`: Small routine of 3 instructions that adds 1 to (a1), then clears d0, then returns.
- `forbid_and_own_blitter`: Disables multitasking through the exec library and claims the blitter through the graphics library.
- `permit_and_disown_blitter`: Releases the blitter through the graphics library and re-enables multitasking through the exec library.
- `start_copper_list_from_struct`: Loads the copper list pointer from a field of a stored structure, clears a related register, and enables copper DMA by writing the set bit to the DMA control register.
- `call_four_routines_return_zero`: Calls four unnamed routines in sequence and returns zero.

### Another World (1991)(U. S. Gold)(It)(Disk 1 of 2)[cr CSL].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
