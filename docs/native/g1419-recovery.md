# Game 1419 (strikefleet (1987)(lucasarts)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01419). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 91 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 168 raw-word lines |
| Functions named and described | pass, 553 of 553 named (44 author symbols), 553 of 553 described (35 by rule, 42 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 64 of 64 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 17579 of 17615 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 479905 |
| code | 66250 |
| data | 152146 |
| padding | 24131 |
| unused | 178688 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| iconx | 12 | 806 | 37 |
| disk-validator | 1 | 386 | 1 |
| ppp.prg | 2 | 2707 | 76 |
| ppq.prg | 2 | 12605 | 365 |
| strikeiv.prg | 2 | 493 | 45 |
| sf.prg | 1 | 374 | 24 |
| Strikefleet (1987)(LucasArts).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- strikefleet (1987)(lucasarts)/Strikefleet (1987)(LucasArts).adf:StrikeFleet/SF_MAP0.LZW: high-entropy data file (7.72 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen longwords from a fixed offset in the base record into the buffer passed in, when one is provided, then calls a second routine.

### assign

- `run_assign_program_entry`: Program entry for the assign command.
- `copy_longwords_then_call`: Copies five consecutive longwords from a source block into a destination buffer when the destination pointer is non-null, decrementing a counter of five until the copy completes.

### iconx

- `run_iconx_program_entry`: Program entry for the iconx command.
- `release_process_resources_and_return`: Stores the parameter as the task's saved value, closes the open file and library handles when present, forbids and replies to a message when present, frees the record memory, and returns the saved value.
- `set_up_icon_console_window`: Looks up an icon in icon.library, sets the default delay, allocates a 1536-byte buffer for the console window, and reports a shortage of memory when the allocation fails before releasing library resources.
- `run_icon_request_handler`: Builds a command buffer from a handler table entry and the request text, then runs the request.
- `create_library_with_functions`: Creates a library from a descriptor structure, builds its function table, and enables the result before returning it.
- `parse_icon_path_string`: Walks a linked list of name nodes, building a path string from entries longer than a threshold, then replaces the first slash with a colon.
- `copy_strings_then_call_offset`: Copies one string to a second destination and a second string to the first, then calls a routine with an offset computed from the routine's own return value.
- `count_string_length`: Returns the number of bytes before the terminating zero in a zero-terminated string.
- `copy_null_terminated_string`: Copies bytes from a source pointer to a destination pointer until a null terminator has been copied.
- `skip_string_then_call`: Advances a pointer to the zero byte that ends a string, then calls a routine with that pointer and the second argument unchanged.
- `parse_signed_decimal`: Parses a signed decimal integer from a zero-terminated string, accepting an optional plus or minus sign and stopping at the first non-digit.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_with_stack_slot`: Calls a routine with the two parameters it receives, plus a pointer to a fixed stack slot.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `format_text_and_send`: Formats text into a 140-byte stack buffer through the library's RawDoFmt routine, measures the string length, and passes the buffer and length to a routine when the string is not empty.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_raw_format_routine`: Invokes the library's RawDoFmt routine through the global library base and returns.
- `store_byte_at_output_pointer`: Writes the low byte of D0 to the address held in A3.

### disk-validator

- `run_disk_validator_main`: Program entry of the disk-validator tool.

### ppp.prg

- `initialise_system_then_loop`: Program entry for the second loader: stores a plane pointer and a register value in globals, calls setup routines, then loops forever calling two routines.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `fill_table_pairs_and_delay`: Fills two tables of word pairs with fixed values, waits in a nested countdown loop, and when a flag is set writes two fixed pointer-like values into several global slots.
- `store_bitplane_word_pointers`: Splits a base address read from a record into four bitplane pointers spaced 8000 bytes apart, storing each as high and low words in two sets of globals.
- `combine_bitplane_masks`: Copies a block of 480 longwords, then for four rows of 24 words combines three bitplanes with bitwise logic to derive mask words.
- `service_audio_channel_requests`: Checks the request bits for the four audio channels.
- `queue_audio_channel_sample`: Selects a free audio channel record for the requested sample, copies its pointer and length into the record, and sets its start address and volume.
- `start_pending_audio_channels`: Loops over four audio channel records and, for each one flagged active and matching the current id, loads its sample address, length, period and volume into the channel's hardware registers, enables its DMA bit, and clears its flag.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `open_and_read_file_retrying`: Opens a DOS file, repeating the open until it returns a handle, then repeats the read until it succeeds, then closes the file and the library.
- `close_file_and_library`: Closes the DOS file and the library, then returns the value passed in.
- `open_write_and_close_file`: Opens a DOS file and writes to it when the open succeeds, then closes the file and the library.
- `open_dos_library_base`: Opens dos.library through Exec's OldOpenLibrary and stores the returned base in a global.
- `report_io_error_and_close`: Reads the DOS I/O error code, closes the file and library, and returns a failure value.
- `open_and_examine_file`: Opens a file and examines it, storing part of the examined result when both calls succeed.
- `decode_raw_keyboard_code`: Decodes a raw keyboard code from the keyboard serial register into a character through a key table, ignoring released keys and recording two special codes in a flag.
- `clear_special_key_flag`: Clears the special-key flag when the low seven bits of the key code are 96 or 97.
- `install_graphics_handler_state`: Opens graphics.library, saves one of its words and replaces it with a handler address, writes enable values to the interrupt and DMA registers, then closes the library.
- `restore_graphics_interrupt_state`: Writes clear masks to the DMA and interrupt control registers, restores two saved values, then writes the enable value again.
- `handle_channel_interrupt_queue`: Interrupt handler that acknowledges the first audio channel request.
- `handle_input_interrupt_queue`: Variant of the channel interrupt handler that takes its flags from D0 and a record value.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_mouse_position_deltas`: Converts the change in the mouse counter into signed deltas and adds them to two position values.
- `update_mouse_position_deltas_entry`: Same mouse delta update as its neighbour, entered with the counter already loaded from the caller.
- `store_mouse_counter_and_clamp`: Stores a new counter value from D0, adds an adjusted delta from D2 to the position with sign correction, clamps the ranges, and updates the second position.
- `update_mouse_port_position`: Reads the mouse port counter in JOY0DAT, computes the change since the last reading, and updates the horizontal and vertical position values with clamping.
- `clamp_mouse_position_values`: Adds a signed delta to a position word, clamps it to a range, raises it to a minimum from a record field, and caps the vertical value at 304.
- `clamp_mouse_position_limits`: Clamps a position word to the record-derived minimum and to 184, and caps the vertical value at 304.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `forbid_task_switching_passthrough`: Calls Exec's Forbid routine to disable task switching, then returns the register pair unchanged.
- `swap_buffers_then_wait`: Disables interrupt bit 5, swaps two stored pointers and sets a flag word, waits until the interrupt request bit 5 is set, then re-enables bit 5.
- `wait_for_interrupt_then_reenable`: Polls the interrupt request register until bit 5 is set, then writes the value that re-enables that interrupt bit.
- `match_tag_and_decode`: Checks a 16-byte tag in a buffer against a stored pattern.
- `match_tag_decode_alt`: Same tag match and field decoding as its neighbour, writing the decoded values into a different set of globals.
- `match_tag_decode_second`: Same tag match and field decoding as the routine before it, with a separate set of destination globals.
- `match_table_and_decode`: Compares a source string against a table entry byte by byte and, on a full match, decodes several ASCII digit fields into numeric values while storing pointers and clearing two flags before returning.
- `match_tag_and_decode_2`: Compares a 16-byte tag in a buffer against a stored pattern byte by byte, and on a full match decodes several ASCII digit fields into numeric values stored in record fields.
- `dispatch_queued_handler_entry`: When the queued handler index differs from the read index, calls the handler from a 32-entry table and advances the index with wraparound.
- `store_value_in_record_field`: Stores its stacked argument in a fixed field of the record.
- `reset_and_jump_to_restart`: Calls the reset routine and then jumps to a fixed low address, without returning.
- `start_blitter_copy_operation`: Waits for the blitter, programs its masks, modulo values, source and destination pointers and control registers from the record, then starts a blit of the set size.
- `start_simple_blitter_copy`: Waits for the blitter, programs its masks, control registers and pointers from the record, then starts a blit of the set size.
- `build_timed_entry_text`: Looks up a table record, derives a timing value by adding fixed increments for each count, copies up to 16 entries into a buffer, writes decimal digits into a text line, and waits before returning.
- `decompress_lzss_into_record_buffer`: Performs the same LZSS-style decompression as its sibling, filling a 4096-byte ring buffer with spaces and writing the output into a buffer offset from the record.
- `update_timed_slot_entries`: Decrements timers on a record and advances or retires entries in a 12-byte-stride list of up to 16 slots, calling routine routines as entries expire.
- `format_three_digit_number`: Writes a number as three ASCII digits in a buffer, hundreds first, and uses a marker to blank out a leading zero.
- `draw_tile_into_four_bitplanes`: Copies a tile of 16 columns by 21 rows into four bitplane buffers, looking up a per-pixel mask and writing one byte to each plane at each step.
- `update_slot_selection_flags`: Marks entries in a 12-byte-stride list with a selection flag where their id matches the current slot, then clears the flags of entries whose stored values have changed.
- `advance_frame_state_machine`: Handles the per-frame state for the active object: counts down several timers, updates the current selection and displayed status, and calls drawing routines when a condition holds.
- `start_text_sequence_entry`: Selects a text sequence by index, records its pointer and start time, then steps through its entries, calling routines for each one until the terminator byte is reached.
- `start_text_sequence_without_flag`: Starts a text sequence by index with the same stepping loop as its neighbour, without the initial position update.
- `add_digit_pattern_to_string`: Adds a stored pattern of digits to an eight-character decimal string with carry, wrapping each digit modulo ten and writing it back as an ASCII digit.
- `subtract_digit_pattern_from_string`: Subtracts a stored pattern of digits from an eight-character decimal string with borrow, wrapping each digit modulo ten and writing it back as an ASCII digit.
- `subtract_digit_pattern_variant`: Performs the same decimal subtraction with borrow as its neighbour on an eight-character string.
- `step_timed_sequence_until_end`: Waits for each timer tick in turn and calls two routines per step until the sequence pointer reaches a zero terminator.
- `draw_text_glyphs_into_bitplanes`: Reads the next word of a sequence and, for each character of a string, either blanks eight rows for a space or draws an eight-row glyph into four bitplanes using a lookup table.
- `advance_sequence_and_pause`: Reads the next word of a sequence, records a stored value when it hits the end marker, then calls a routine eleven times.
- `call_routine_twenty_one_times`: Calls one routine 21 times in three groups of seven, with no other work.
- `fill_rows_when_state_changes`: When the current and target state values differ, counts the frames spent, calls a routine during the first three frames, and after that fills a 72-row column strip with solid bytes for eight or nine columns.
- `fill_column_strip_with_byte`: Fills a 72-row column strip in the bitplane with the byte passed in D1, for eight or nine columns, starting at a table offset chosen by the index in D0.
- `build_spec_display_screen`: Sets up a display screen by copying image and table data into the bitplane buffers and clearing a sprite-style table.
- `interleave_bitplanes_to_buffer`: Reads four bitplanes at fixed offsets and writes their words interleaved into a single buffer, 84 rows of seven words at a time.
- `deinterleave_buffer_into_bitplanes`: Reads an interleaved buffer and writes its words back out to four separate bitplanes, 84 rows of seven words at a time.
- `run_display_update_loop`: Initialises display buffers and entry lists from tables, then runs a per-frame loop that updates entries and text fields until its frame counter reaches zero.
- `initialise_display_bitplanes`: Sets bitplane pointers from the record, copies an image through a routine, expands a nibble-indexed table into four bitplanes, and fills a 256-entry word table with inverted patterns.
- `draw_text_glyphs_alt`: Draws a zero-terminated string into the bitplanes with one eight-row glyph per character and a blank strip for a space.
- `draw_masked_bitplane_rows`: Combines source rows with per-plane masks selected from the state index and writes the combined words into four bitplanes, four rows of eight words each.
- `initialise_sprite_table_and_enable`: Initialises the sprite records: fills a 16-row mask table with alternating words, copies per-entry sprite data into the buffers, sets default position values, then re-enables interrupt bit 5.
- `read_fire_update_display`: Reads the fire and right-button bits from the CIAA_PRA and POTGOR registers to update a direction value, then updates the displayed position and timers and runs a text update when the state changes.
- `build_sprite_control_pair`: Computes the start and stop control words for a pair of hardware sprite entries from an object's screen X and Y position and the current frame index, then stores them in the sprite buffer.
- `load_object_speed_from_table`: Stores a value taken from a 16-bit lookup table, indexed by a register, into an object field.
- `load_object_offset_from_table`: Stores a value taken from a second 16-bit lookup table, indexed by a register, into an object field.
- `find_object_at_point_animate`: Walks a linked list of nine-word object records to find the first record whose rectangle contains the given point.
- `copy_indexed_longword_pair`: Copies two longwords from a table entry selected by the object's index field into fixed state longwords, then calls a follow-up routine.
- `blend_masked_bitplanes`: Combines four source bitplane words, each shifted right by a count held in a register, with a 32-bit mask from the object state and the two destination planes using AND and OR. It writes the result into four destination planes over sixteen iterations, each stepping through the planes by a fixed stride.

### ppq.prg

- `copy_startup_block_then_loop`: Copies five longwords from a source block into the low memory area, records two pointer values in globals, and calls four setup routines.
- `init_scroll_table_and_delay`: Fills a block of 16-bit table entries with fixed values and busy-waits through a nested counted loop.
- `copy_table_row_to_destination`: Copies twelve words from a row of a table into a destination table.
- `set_bitplane_pointers`: Reads a base address and writes its high and low words into the four bitplane pointer slots, spacing the planes 8000 bytes apart.
- `copy_and_repack_bitplanes`: Copies a block of 768 longwords, then runs eight passes that clear the start of each output row.
- `service_audio_channel_flags`: Checks a shared status word for pending requests on the four audio channels and clears the busy flag.
- `claim_audio_channel_slot`: Picks an unused channel record from a small array, stores the requested sample's start address and length there, and records the audio enable value.
- `start_ready_audio_channels`: For each of the four audio channel records that is marked active and matches the current sequence value, copies its sample address, length and period into the channel's hardware registers.
- `load_dos_file_into_buffer`: Opens a DOS file and retries the open while it fails, then reads in a loop until the read returns a non-negative count.
- `read_dos_file_with_retries`: Opens a DOS file through the library and retries the open while it fails.
- `write_block_to_dos_file`: Opens a DOS file handle, and if that succeeds writes a block of data to it and returns the count written.
- `open_dos_library_base_2`: Opens the DOS library through the exec library's OldOpenLibrary call and stores the returned library base in a global.
- `report_dos_error_and_close`: Reads the DOS error code, closes the file handle, releases the library base, and returns a failure marker.
- `examine_dos_file_info`: Opens a file, examines it through the DOS library, and stores the returned information word in a global when the examine call succeeds.
- `translate_keyboard_scancode`: Decodes a raw keyboard code read from the keyboard serial register.
- `update_shift_key_state`: Resets the shift-state flag when the low seven bits of the key code are 96 or 97, the two shift key codes, and returns its input unchanged.
- `install_custom_copper_list`: Opens the graphics library, saves its current copper list pointer and replaces it with the program's own copper list.
- `restore_system_copper_state`: Disables the DMA channels and interrupt sources by writing clear masks, restores the previously saved copper list pointers, and then turns the DMA channels back on.
- `verify_longword_table_or_halt`: Compares five longwords of one table against a second block of five longwords.
- `verify_longword_table_or_halt_2`: Compares five longwords of a table against a second block of five longwords and loops forever if any pair differs.
- `service_copper_and_vblank_interrupts`: Handles the copper interrupt by recording a status code and clearing its request.
- `service_vertical_blank_step`: Runs the vertical blank part of the interrupt work when the request bit is set.
- `update_mouse_button_state`: Combines two mouse button flags into a two-bit button state value.
- `update_pointer_position_from_mouse`: Turns the change in a mouse counter since the previous call into a signed step.
- `refresh_pointer_from_mouse_counters`: Takes the change in the mouse counters since the previous call as a signed step.
- `apply_mouse_delta_to_pointer`: Turns the change in the mouse counters since the previous call into a signed step.
- `set_pointer_position_with_delta`: Stores a new horizontal position and adds a vertical delta to the vertical position word.
- `update_pointer_from_joy0_counter`: Reads the mouse counter from the joystick zero data register and computes its signed change against the previously stored value.
- `store_pointer_counter_and_clamp`: Stores a counter value as the new horizontal position word and adds a corrected vertical delta.
- `clamp_pointer_screen_bounds`: Clamps a vertical position word between a stored lower bound and 183.
- `wait_for_blitter_done`: Spins in a tight loop until a bit in a global variable clears, polling the hardware status register via a move instruction to wait for the blitter or a related hardware operation to finish.
- `re_enable_task_switching`: Calls the exec Permit routine to re-enable task switching and returns the input registers unchanged.
- `disable_task_switching`: Calls the exec Forbid routine to disable task switching and returns the input registers unchanged.
- `swap_pair_and_wait_interrupt`: Swaps two longwords in the object record and sets a flag word to 1.
- `wait_for_vblank_then_enable`: Busy-waits until the vertical blank request bit is set, then writes the interrupt enable word with its set bit.
- `parse_ascii_header_fields`: Compares a marker in the data with the buffer and, when it matches, decodes the ASCII decimal digits that follow it into numeric globals.
- `parse_header_timing_fields`: Compares a marker in the data with the buffer and, when it matches, decodes the ASCII decimal digits that follow it into numeric globals, storing a subset of the timing and pointer fields.
- `parse_header_pointer_fields`: Compares a marker in the data with the buffer and, when it matches, decodes the ASCII decimal digits that follow it into numeric globals, storing a subset of the pointer fields.
- `parse_header_numeric_fields`: Compares a marker in the data with the buffer and, when it matches, decodes the ASCII decimal digits that follow it into numeric globals, storing a subset of the numeric fields.
- `match_banner_and_parse_fields`: Checks the buffer for the credit banner text, DIGITAL PERSUASION, and when it matches decodes the numeric fields after it into globals.
- `drain_key_event_ring`: Takes the next code from a 32-entry ring buffer when the read index differs from the write index, then advances the index with wrap-around.
- `advance_index_and_check_code`: Reads an index from a state record, looks up a code in a table, and advances that index with wrap-around inside a thirty-two entry range.
- `store_longword_in_state`: Stores the longword passed on the stack into a fixed field of the state record.
- `reset_and_jump_low_memory`: Calls the reset routine and then transfers control to the low memory address 2, which is the program's fixed restart point.
- `dispatch_exec_or_reset`: Compares a stored count with 35.
- `open_library_then_reset`: Calls a library routine at offset -30, then resets and jumps through a dispatch table entry.
- `reset_and_dispatch_entry`: Calls the reset routine and then jumps through a dispatch table entry located from the program's data.
- `load_plane_addresses_from_view`: Opens the graphics library, reads the first bitmap plane pointer from the view structure, and derives the other plane addresses at 8000-byte spacing.
- `reload_plane_base_addresses`: Opens the graphics library, reads the bitmap pointers from the view structure, and derives the plane base addresses at 8000-byte spacing.
- `reload_plane_offset_table`: Opens the graphics library, reads the bitmap pointers from the view structure, and derives the plane offsets at 8000-byte spacing.
- `reload_plane_pointer_pair`: Opens the graphics library, reads the bitmap pointers from the view structure, and derives a pair of plane addresses at 8000-byte spacing.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `reload_plane_stride_offsets`: Opens the graphics library, reads the bitmap pointers from the view structure, and derives the plane stride offsets from the 8000-byte plane size.
- `reload_plane_stride_pair`: Opens the graphics library, reads the bitmap pointers from the view structure, and derives a stride pair from the 8000-byte plane size.
- `reload_plane_end_offsets`: Opens the graphics library, reads the bitmap pointers from the view structure, and derives the end offsets of each plane at 8000-byte spacing.
- `open_graphics_and_setup_planes`: Opens the graphics library through the exec OldOpenLibrary call and derives the bitplane addresses from the view structure's bitmap pointers at 8000-byte spacing.
- `initialise_level_state_tables`: Clears the game state fields to zero or to sentinel values and zeroes the counters of 32 records.
- `decompress_lzss_stream`: Unpacks an LZSS-style compressed stream.
- `copy_block_into_four_planes`: Copies a block of 4000 words into four bitplanes spaced 8000 bytes apart.
- `stub_immediate_return`: Empty routine that returns immediately without any work.
- `draw_object_list_by_type`: Walks an object list, converting each object's screen coordinates into a cell position.
- `pack_pixels_into_bitplanes`: Converts a run of source bytes into bit values using a lookup table and merges them into four bitplanes spaced 8000 bytes apart.
- `pack_row_into_bitplanes`: Converts a row of source bytes into bit values using a lookup table and merges them into four bitplanes spaced 8000 bytes apart.
- `update_timer_accumulators_by_mode`: Resets the timer accumulators and limits when the mode is zero.
- `run_frame_sequence_until_limit`: Runs a per-frame loop of several update and draw calls, marking the state record as active.
- `run_state_update_sequence_2`: Returns early when a decremented state counter is negative, otherwise invokes another routine several times while clearing and setting state flags and copying a value between two state fields before resetting the counter.
- `run_state_update_sequence`: Runs the state update calls, sets the state flags and resets the state counter.
- `compute_grid_cell_from_position`: Derives a grid cell index by dividing an object's position offset by a rate when the position lies within bounds.
- `run_frame_state_machine`: Runs the frame state machine: it processes pending request flags in turn and updates the scroll and position state.
- `copy_object_and_wrap_position`: Copies position and display fields from one object record to another and clears a flag.
- `wrap_object_position_modulo`: Wraps the object's two position words into a 375-unit cycle relative to the supplied offsets.
- `clear_status_text_cells`: Writes space characters into a block of cells at fixed row offsets within the text buffer.
- `compose_hud_status_text`: Builds a status text line in a buffer by copying name strings with a colon separator, then appending the literals Kts and a slash.
- `store_signed_index_pair`: Stores the value in one field.
- `scan_render_list_for_position`: Initialises a list entry from the input offset, then repeatedly reads entries from the list and compares each position with the object's position.
- `store_and_swap_index_pair`: Stores a new value in one field and moves the previously stored value into a second field.
- `scan_render_list_mode_three`: Initialises a list entry from the input offset with mode three, then reads entries from the list and compares each position with the object's position.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 0 into d5, then loads 4 into d7.
- `advance_masked_counter_loop`: Repeats a pair of calls for a count held in a register, advancing a 15-bit counter by one on each pass.
- `forward_single_call`: Calls a single routine and returns, with no other work.
- `store_byte_in_indexed_table`: Stores the low byte of an input value into a table entry chosen by doubling the value and offsetting it from a base table.
- `compose_status_name_line`: Fills two text rows with spaces and an attribute byte, then copies name strings from the object's string table into them.
- `format_three_digit_decimal`: Writes an input number as three ASCII digits, replacing leading zeros with a blank glyph.
- `format_three_digits_zero_padded`: Writes an input number as three zero-padded ASCII digits.
- `format_two_digit_zero_padded`: Writes an input number as two zero-padded ASCII digits.
- `format_four_digit_decimal`: Writes an input number as four ASCII digits, replacing leading zeros with a blank glyph.
- `copy_planes_into_frame_buffer`: Resets two state fields and copies a 4000-word block from four plane buffers into the destination bitplanes spaced 8000 bytes apart.
- `build_spread_and_gray_tables`: Fills a 256-entry table with bit-spread words built from each byte value, and a 512-entry table with Gray-code words.
- `clear_text_block_with_spaces`: Fills a block of longwords with the space pattern, using 64 entries or 51 depending on a mode flag.
- `clear_text_block_from_pointer`: Fills a block of longwords with the space pattern, using 64 entries or 51 depending on a mode flag.
- `write_packed_text_entries`: Reads entries from a packed table until a negative marker.
- `render_text_cells_four_passes`: Runs four passes over a grid of text cells, drawing each cell's glyph into four bitplanes with a colour mask chosen by the pass number.
- `init_timer_pointers_and_accumulate`: Sets two buffer pointers in the state record, then resets the timer accumulators and limits when the mode is zero.
- `render_text_string_to_planes`: Renders each character of a zero-terminated string into four bitplanes using 8-byte glyph bitmaps from the font.
- `spin_wait_on_counter_offset`: Busy-loops while the sum of the input and the stored counter differs from the stored counter, so it returns only when the input is zero.
- `draw_text_rows_to_planes`: Optionally stores a position pair and calls a routine, then renders each character of a text list into the four bitplanes using the font.
- `copy_text_block_to_display`: Sets a scroll position, then copies a 32-longword header block and a 1840-longword block between two buffers.
- `render_indexed_text_string`: Looks up a text string in a table using a row index from the state record, then renders each character into the four bitplanes from the font.
- `run_object_update_frame_loop`: Runs a loop that records the frame counter, copies the pending state into the object's fields and calls a set of update routines on each pass.
- `blit_object_into_planes`: Blits an object's image into four bitplanes, combining masked source words with the destination planes over several rows.
- `merge_planes_under_mask`: Merges three source bitplanes into the destination planes over six rows of ten words.
- `draw_text_masked_to_planes`: Renders a zero-terminated string into the bitplanes using the font, combining each glyph row with the existing plane contents through a colour mask.
- `blit_rotated_object_sprite`: Computes a sprite angle for the object, wrapped at 360, and blits its shape data into the bitplanes at the computed screen position.
- `update_object_draw_list`: Walks the object list and computes a scaled size for each object, clamped to a minimum of 1.
- `draw_object_entries_loop`: Loops over an entry list, calling a drawing routine with the fields of each entry.
- `clip_rectangle_to_screen`: Checks a rectangle against the screen edges and returns a failure value when it lies fully outside.
- `update_visible_object_entries`: Walks the object entries, updating each entry's screen position and rate counters.
- `scale_value_by_fraction`: When the input field is nonzero, scales a stored value by the difference from 64 divided by 64.
- `compute_scaled_sprite_size`: Derives scaled sprite dimensions by dividing a 10-bit fixed-point size by a divisor.
- `compute_scaled_sprite_rect`: Checks a sprite's visibility flags and link index, then derives a scaled size from its table entry and stores the result.
- `compute_flagged_sprite_size`: Derives a scaled sprite size from the table entry when the 1024 flag bit is set, then stores the position and dimensions.
- `draw_sprite_entry_list`: Walks a list of sprite entries, computing each entry's position from its stored offsets.
- `clip_sprite_rectangle_to_view`: Checks a sprite rectangle against the view edges, with a horizontal limit of 320 and a vertical limit of 102.
- `advance_animation_counter_table`: Decrements an animation counter and, when it runs out, reloads it from a frame-timing table and advances the table index.
- `clear_sprite_cell_planes`: Clears a block of bitplane bytes for a sprite cell, writing zero to three planes and 255 to the first plane.
- `run_routine_with_flag_once`: Sets the state record's flags, calls a routine once and marks a byte in the state record before returning.
- `draw_status_panel_planes`: Draws a fixed-layout panel into the four bitplanes.
- `walk_entry_list_and_mark`: Walks a list of entries from the state record and skips entries that do not match the current position.
- `test_tile_within_bounds`: Checks an offset pair against bounds that depend on the row, then computes a tile index and reads its entry from the map table.
- `setup_blitter_masked_copy`: Writes a sequence of blitter-related words into the state record, then calls a wait routine and writes the DMA enable word.
- `setup_blitter_row_copy`: Writes a sequence of blitter-related words into the state record, then calls a wait routine and writes the DMA enable word.
- `program_blitter_row_copy`: Programs the blitter registers for a masked row copy, waits for the blitter to finish, and then re-enables the DMA bit.
- `program_blitter_plane_pairs`: Programs the blitter registers for a masked copy of two plane pairs, advancing the source and destination by 8000 bytes between passes.
- `reset_blitter_state_and_wait`: Clears two state fields, writes the interrupt mask value and calls two routines to wait for the blitter.
- `increment_state_counter`: Adds one to a 16-bit counter in the state record and returns the input registers unchanged.
- `poll_input_and_update_state`: Reads the fire button and the right and middle button inputs and records their state in the state record.
- `update_sprite_state_per_frame`: Records the current animation value and clears a pending state marker when it is set.
- `update_sprite_state_on_zero`: When the zero flag is set, it marks a pending clear or processes the object's table entry.
- `update_sprite_state_negative`: Stores the state value.
- `process_sprite_entry_when_clear`: When the flag is clear, processes the object's table entry by calling routines and comparing a scaled value with the stored limit before updating the state.
- `assign_free_sprite_slots`: Sets the slot counter and walks seven sprite slot entries.
- `compute_object_screen_position`: Derives screen X and Y values for an object by scaling a table entry with a shifted value.
- `claim_free_position_record`: Scans records of seven words until it finds one that is free, marked by the value -1.
- `store_object_screen_size`: Stores the object's scaled screen X with an offset of 80 and its Y with a lower clamp of 0 in the state record.
- `store_scaled_screen_position`: Stores the scaled screen X with an offset of 80 and the Y with a lower clamp of 0 and an offset of 3.
- `write_sprite_control_words`: Writes two input words into a sprite control buffer, choosing the second buffer when the flag is set.
- `select_and_run_wait_routine`: Selects a wait routine based on the state flags, calls it, and clears the state field.
- `advance_sprite_state_machine`: Advances a sprite state machine: the state value selects a branch that updates position fields and calls helper routines.
- `find_rect_containing_point`: Scans a list of rectangle records for the first one whose bounds contain the input X and Y, and stops at that match.
- `test_object_rect_hit`: When the state is 4 and the object is above the limit of 176, tests whether its rectangle overlaps a fixed hit area.
- `dispatch_handler_by_index`: Clears the handler field and calls a routine chosen from a jump table by the input index.
- `draw_sprite_frame_from_state`: Draws the current sprite frame when the sprite is active, choosing its source buffer and draw mode from the state record.
- `run_state_handlers_by_index`: Calls the handler selected by the sprite state from a jump table, running it twice when the state is 5 or higher.
- `run_state_handler_hit_test`: Calls the handler selected by the state from a jump table.
- `advance_state_or_reset`: Resets the state to zero when the input is 8, otherwise advances it by one.
- `increment_sprite_state_and_store`: Advances the sprite state by one, clears a flag and records the input parameters in the state record.
- `test_rect_and_set_state`: When the object is above the limit of 176, tests its rectangle against a hit area.
- `set_hit_state_from_rect`: Tests the object's rectangle against a fixed hit area.
- `mark_flag_and_advance_state`: Sets a flag to one, advances the state by one, clears a flag and records the input parameters in the state record.
- `test_rect_and_set_hit`: Tests the object's rectangle against a fixed hit area, and on overlap records the hit values in the state record.
- `find_entry_containing_rect`: Scans table entries and returns the first one whose bounds contain the input rectangle.
- `store_pointer_value_and_advance`: Copies a word from the input pointer into the state record, sets a flag and advances the state by one.
- `copy_plane_block_rows`: Copies an eight-row block of longwords from the source plane area to the destination.
- `dispatch_index_with_zero_check`: Returns when the input is zero.
- `dispatch_table_with_special_index`: Calls a routine from a jump table, using a separate table entry when the input is 12 or 24.
- `call_fixed_table_slot`: Calls the routine at a fixed slot in a jump table and returns.
- `call_table_entry_by_index`: Calls the routine at the jump table offset given by the input value and returns.
- `call_slot_from_state_table`: Calls a routine from a jump table selected by a field in the state record.
- `call_slot_from_object_pointer`: Calls a routine from a jump table selected by a field in the record that the state record points to.
- `draw_sprite_frame_by_mode`: Draws the sprite frame for the current mode, updating the frame buffer pointers and selecting the source frame from the record.
- `copy_name_until_colon`: Copies characters from an input string into a buffer until a colon or the length limit is reached.
- `step_cycle_index_with_wrap`: When the object is active, adds the step value to a cycling index that wraps to zero above four.
- `adjust_cycling_index_and_shift_table`: Adjusts a cycling index that wraps between zero and four based on a signed value, then, when a flag is negative, sets up a block of data and walks a list comparing values until a terminator is found.
- `toggle_flag_and_store`: When the object is active, toggles a flag byte in the object record.
- `repeat_call_twenty_then_finish`: Clears the active flag and calls a routine twenty times.
- `call_three_routines_return`: Calls three routines in sequence and returns the input value unchanged.
- `call_two_routines_return`: Calls two routines in sequence and returns the input value unchanged.
- `call_routine_then_return`: Calls one routine and returns the input value unchanged.
- `return_routine_result`: Invokes another routine and then returns the value that routine placed in its result location without modifying it.
- `forward_helper_call_2`: Invokes another routine and returns the value that routine placed in its result location without modifying it.
- `forward_helper_call`: It invokes another routine and then returns the value that routine placed in its result location without modifying it.
- `call_routine_keeping_registers`: Calls one routine and returns the D0 and D1 register values unchanged.
- `swap_state_word_around_call`: Saves the state longword, replaces it with the value from the second input register for one call, then restores the saved value.
- `dispatch_jump_table_by_register`: Calls a routine from a jump table selected by the scaled value in the index register.
- `toggle_flag_and_draw_entry`: When the object is active, toggles a flag byte in the object record.
- `select_label_string_by_flag`: Looks up a label string from a table based on an object flag, then copies it into the output buffer character by character.
- `measure_and_copy_label_text`: Measures a zero-terminated string from a fixed table, then copies its characters into the output buffer from the end backwards.
- `copy_reversed_string_suffix`: Skips leading spaces in a string and measures the remainder.
- `copy_label_glyphs_reversed`: Measures a zero-terminated string and writes its characters from the end backwards.
- `step_cycle_index_for_label`: When the object is active, updates a cycling index that wraps to zero above four and stores it in the object record.
- `format_stop_status_text`: Chooses between the STOP label and a numeric status text based on a mode byte, then copies the selected text into the output buffer.
- `format_full_or_marker_text`: Selects the FULL label when the input is 4.
- `write_marker_then_label`: Writes two marker characters and a terminator to the output buffer, then copies the object's label text into it from the end backwards.
- `format_signed_value_text`: Formats an input value as text, writing a dash for zero and otherwise converting the absolute value with a terminator.
- `copy_label_glyphs_terminated`: Writes a terminator, measures the label string, then writes its glyph offsets into the output buffer from the end backwards.
- `format_offset_value_text`: When the object is active it calls a routine first.
- `format_offset_value_with_flag`: When the object is active and its two state values differ, it calls a routine first.
- `format_offset_value_reset`: Clears the state flag, then formats the absolute value of the object's offset field as text, writing a dash when the value is zero.
- `render_signed_number_to_bitmap`: Converts a signed integer into its decimal text representation, writing a dash when the value is zero and otherwise encoding the absolute value digit by digit.
- `test_flag_mask_and_act`: Tests a flag word in the object record against a mask.
- `xor_pixel_bits_into_plane`: Updates a plane byte by OR-ing and XOR-ing bit patterns computed from the input offset.
- `write_bitplane_words_with_clip`: Writes a set of longword and byte updates into the bitplane rows at offsets from the input value.
- `format_decimal_up_to_three`: Writes an input value as decimal digits with leading zeros suppressed.
- `format_one_or_two_digits`: Writes an input value as decimal digits, using two digits when the value is above nine.
- `copy_label_glyphs_offsets`: Measures a string and writes its glyph offsets into the output buffer from the end backwards.
- `sum_label_glyph_offsets`: Writes the glyph offsets of a string into the buffer from the end backwards while accumulating a running total of their values.
- `shift_glyph_rows_into_planes`: Walks a null-terminated list of glyph pointers and shifts each glyph's bytes into the bitplane rows by the computed bit offset.
- `unpack_three_plane_rows`: Reads pixel bytes from a source stream and a zero-terminated list of row counts, then writes each byte into three bitplane areas combined with OR, OR and XOR. It is a row-by-row bitplane unpacker.
- `stub_return_only`: This entry contains only a return and does nothing.
- `update_visible_terrain_grid`: Compares the viewer's current cell with the cached one.
- `compute_scaled_table_entries`: Computes a scaled offset from a table using fixed-point division, then reads two lookup values and fills a 16-entry table with the scaled results.
- `compute_triangle_edge_shading`: Computes screen-space offsets for a triangle vertex, then scales those offsets by per-edge brightness values to fill a small lookup table.
- `read_terrain_bits_into_grid`: Performs the same cell check and projection setup as its neighbouring grid routines, then reads one bit from each of three bitplanes per cell and stores a 0 or 100 value for it.
- `compute_grid_cell_screen_values`: Projects a 15 by 15 grid of cells to screen values using cached per-cell values and lookup tables, clamping the results to a fixed range and marking cells outside it as hidden.
- `fill_handler_grid_with_checks`: Fills a fifteen-by-fifteen grid of cell entries with per-cell screen values, computing a slope and offset for each cell and storing a handler pointer, or marking the cell invalid when the values are out of range.
- `advance_or_fill_grid_columns`: Calls three unnamed routines, then either fills vertical strips of screen memory for cells with non-negative values when a step counter equals 8, or advances a pointer through the grid data, lowers the counter by four and returns the absolute value of the entry it read.
- `flood_fill_grid_cells`: Marks a grid cell as visited, then recurses into neighbouring cells that are still unmarked, using a table of neighbour offsets.
- `draw_sorted_triangle_edges`: Sorts three vertex coordinates and works out the step direction and increments for the edges between them, calling unnamed routines for each span.
- `step_line_pixels_with_error`: Walks a line from a start point using error accumulators on both axes, calling an unnamed routine for each step until it reaches a negative terminator in the point list.
- `fill_column_or_lower_top`: Clamps a vertical span to a fixed range, then either fills that column of screen memory row by row, 40 bytes apart, when a flag is clear, or lowers the stored top value for the column when the flag is set.
- `build_row_offset_table`: Builds a table of 200 screen-row byte offsets in steps of 40 bytes, clears the work arrays and a list of records, then disables and re-enables the vertical blank interrupt through the interrupt enable register.
- `poll_buttons_and_advance_frame`: Advances the frame counter, reads the fire and right buttons from the controller port registers into a button-state word, and steps a phase counter when that state changes.
- `advance_frame_and_test_overlap`: Steps the frame state and places two sprite entries as in the neighbouring routine.
- `finish_frame_after_table_store`: Stores a value from a lookup table into the object record and negates the stored value for one table index.
- `store_entry_and_close_frame`: Stores a lookup-table value into the object record and negates it for one table index.
- `find_overlap_record_and_step`: Walks a list of records until it finds one whose box covers the given point, then updates that record's phase and value from a lookup table.
- `scan_records_and_restore_state`: When the state flag and position test pass, it saves four long words of the object and sets new values, then walks a list of records until one covers the object's position.
- `load_long_pair_from_table`: Scales an index from the record by eight, reads two long words from a table held in the object, and stores them in the object's work fields.
- `blit_masked_sprite_rows`: Blits sixteen rows of a sprite through a mask.
- `load_object_sprite_entries`: Reads an object's setup block and copies its position and attribute fields into the object record.
- `copy_sprite_frames`: Copies a block of sprite data into a per-object table, computing per-frame positions from a base coordinate and scaling factors, then applies the current frame selection.
- `copy_and_adjust_record`: Copies a block of fields from a source record into a destination record, adjusting two coordinate values by offsets scaled by 2000.
- `duplicate_records_from_block`: Copies a block of fields from one record into a series of fresh records, computing derived values from a table and clearing the rest of each copy.
- `set_empty_fields_to_value`: Walks sixteen 96-byte records and writes a supplied value into the field at offset 8 of each record whose field is still zero.
- `search_flagged_record_list`: Walks a list of records linked by relative offsets, testing each record's mask word against the search masks and the flag word.
- `search_record_list_by_mask`: Walks a list of records linked by two-byte relative offsets and tests each record's mask word against the search masks.
- `search_record_list_by_mask_2`: Walks a list of records linked by four-byte relative offsets and tests each record's mask word against the search masks.
- `draw_object_with_blitter`: Waits until the blitter is idle, then programs its masks, minterm control, modulos and source and destination pointers from the object's position.
- `start_blitter_from_object_pointers`: Waits for the blitter to be idle, sets its masks and minterm control, then loads source and destination pointers from two fields of the object and starts a small rectangle copy.
- `emit_sprite_strip_entries`: Checks that an x position lies in the visible range, derives a control word and column selection from it, then writes sprite-style control words for a vertical strip into a list held by the object.
- `emit_character_table_entry`: Clamps a column index to a valid range, computes its screen position and width, then writes a series of character-drawing parameters into a per-character table for a fixed number of rows.
- `build_sprite_strip_descriptors`: Clamps a vertical position into a valid range, then fills a loop with sprite-strip descriptors that reference screen memory and a per-row lookup table.
- `emit_clamped_sprite_strip`: Clamps a vertical coordinate into a valid range, then indexes a lookup table to compute a destination pointer and a row offset.
- `fill_blit_descriptor_tables`: Clamps a vertical coordinate into range and derives a base pointer from it, then fills a block of per-entry descriptor tables with pointer, offset, size and flag fields, looping three times.
- `emit_sprite_strip_and_blit`: Runs the same column-entry setup as its neighbours and then, for the visible strip, waits for the blitter and programs it to copy the strip's source and mask planes.
- `write_three_plane_blit_entries`: Runs three times, each time writing a full-mask copy entry for one bitplane: masks, minterm control, source and destination pointers taken from the caller and the object, and a row word.
- `write_offset_plane_blit_entries`: Chooses a row from an object value, defaulting it when negative, and sets the start address and a constant.
- `fill_blit_descriptor_tables_2`: Initializes blit descriptor fields with fixed mask, pointer and control values, then loops three times filling per-entry descriptor tables with bitplane blit data while advancing the destination pointer.
- `write_pair_plane_blit_entries`: Writes one entry at a fixed start position, then writes three full-mask plane entries at a second position computed from the object's offset.
- `write_height_plane_blit_entries`: Derives two start positions from an object field, writes one entry at the first, then writes three full-mask plane entries at the second.
- `write_three_plane_blit_entries_2`: Runs three times, each time writing a full-mask copy entry for one bitplane: masks, minterm control, source and destination pointers taken from the caller and the object, and a row word.
- `write_offset_indexed_plane_bytes`: Walks rows and columns over a table of offsets, writing a computed byte into each of several plane buffers at the offset the table gives.
- `stub_empty_decompile_body`: The decompiler produced only an empty body for this 324-instruction routine, so no behaviour can be recovered from the pseudo-code.
- `refresh_plane_buffers_with_mask`: Copies four bitplane rows from the object into working buffers, fills the edge rows with solid words and inverted mask words, then swaps two buffer pointers in the object.
- `swap_buffers_and_wait_flag`: Swaps the two buffer pointers held in the object, sets a ready flag, waits until a hardware status bit becomes set, and then re-enables that interrupt bit.
- `copy_working_buffers_to_planes`: Copies a block of long words from four working buffers back into the object's four bitplanes, row run by row run.
- `write_checkerboard_mask_rows`: Clears ten long-word fields of the object, sets one byte per row in a scroll buffer from a table, and then writes alternating checkerboard bit patterns across the rows, combined with a stored mask.
- `compute_viewport_clip_rect`: Chooses one of two view layouts from a mode flag, offsets the object's position by that layout's constants, and clamps the result to the layout's range.
- `scan_records_for_closest_hit`: Walks a list of object records and, for each one that overlaps the search point, compares a scaled distance value against the best found so far.
- `turn_heading_toward_target`: Chooses a target heading from the record or the object's table, then turns the record's stored heading toward it by a rate-limited step, wrapping around a full circle.
- `apply_scaled_velocity_components`: Scales two table values by an input factor, divides them by a power of two taken from a global table, and adds the results to the record's two position fields.
- `compute_cordic_angle_from_deltas`: Runs a sixteen-step CORDIC-style iteration on the difference between the input point and the record's position, using step constants from a global table.
- `accumulate_scaled_differences`: Computes the signed differences between two coordinate pairs and takes their absolute values while recording sign flags.
- `compute_angle_to_target_vector`: Works out the angle from a record's position to a target point with an iterative shift-and-add approximation, applying quadrant correction from sign flags, then combines the results with fixed-point scaling constants.
- `combine_span_masks_into_planes`: Clears a block of mask bytes, then for each span in a list reads a table offset, combines it with a bit mask, and ORs the looked-up plane words into the block.
- `accumulate_span_plane_masks`: For each row it clears the mask state, then ORs plane words selected through a table, combined with per-span masks, into the accumulated state.
- `build_edge_span_table`: Builds a per-row table of span entries for a line between two points, stepping with a fixed-point accumulator in the style of a digital differential analyser.
- `update_target_index_state`: Returns the current target index with a status code.
- `reset_target_index_state`: Clears the stored state words, then returns the current target index with a status code by following the same checks as its sibling routine.
- `read_target_index_status`: Returns the input value when a block flag or stored state is set.
- `read_target_status_direct`: Reads the status of the current target without the block-flag check, returning zero for an empty target and otherwise deriving a code from the record's type and the global state tables.
- `walk_target_state_table`: Looks up the target's kind from its record and walks the matching table of entries, copying each entry's values into the global control words until a sentinel is reached.
- `stub_return_only_2`: Contains only a return and does nothing, so it is an empty stub called by one unnamed routine.
- `begin_target_transition_state`: When the target is valid and no transition is active, it searches a small table for the first entry not equal to six, copies that entry to a control word, and marks a transition as started.
- `begin_target_transition_indexed`: Uses the table entry selected by the index passed in, and otherwise follows its sibling: when a transition is due and not already active, it copies the first table entry not equal to six and marks the transition as started.
- `stub_return_only_3`: Contains only a return and does nothing, so it is an empty stub called by two unnamed routines.
- `reset_transition_phase`: Clears the transition phase word in the object and returns.
- `append_timed_event_entry`: Finds the first free entry in a 32-entry table of timed events and fills it with a pointer, a value taken from the source list and a counter.
- `run_event_phase_state_machine`: Advances a small phase state machine over queued events.
- `draw_text_string_to_planes`: Draws a zero-terminated string into the four bitplanes, one eight-row glyph per character.
- `format_number_as_decimal_digits`: Converts a value into decimal digits by repeated subtraction of powers of ten taken from a table, writing ASCII digits into a buffer and a minus sign for negative entries.
- `format_number_digits_scroll_text`: Converts each value in a list of signed numbers into a single decimal digit, writing the digit characters into a buffer and storing the remainder of each division.
- `draw_text_string_to_planes_2`: Draws a zero-terminated string into the four bitplanes, one eight-row glyph per character.
- `tick_countdown_and_spawn_record`: Lowers a countdown by a power-of-two step, and when it drops below a threshold it resets the threshold and starts a new record at the object's base.
- `decrement_and_reload_state`: Decrements a countdown field by one unit and clamps it at zero when it goes negative.
- `advance_record_chain_until_clear`: Negates the entry before the current position in a list of eight values, then advances through a linked chain of records until one has its end flag clear.
- `decrement_and_reload_state_2`: Decrements a countdown field by one unit and clamps it at zero when it goes negative, then walks a chain of records until finding a clear slot, storing the passed parameters there.
- `restore_record_state_words`: Stores three parameter words and one stacked value back into the object's state fields at fixed offsets and then returns.
- `stub_empty_decompile_body_2`: The decompiler produced only an empty body for this routine, so no behaviour can be recovered from the pseudo-code.
- `copy_twelve_bytes`: Copies twelve bytes from a source table into a destination table, advancing both pointers by one byte per iteration until every byte is transferred.
- `lay_out_text_lines`: Copies a block of 25 records into a text buffer, then scans it line by line, ending each line at a carriage-return or terminator character.
- `copy_background_image_to_planes`: When the setup flag is set and the state word is clear, copies a four-plane background image from the object's image buffer into the bitplanes.
- `add_table_digits_to_string`: Adds a table of eight digits to an eight-character ASCII decimal string with carry, writing each result back as a digit.
- `subtract_table_digits_from_string`: Subtracts a table of eight digits from an eight-character ASCII decimal string with borrow, writing each result back as a digit.
- `expand_packed_image_to_planes`: Expands a packed image from a source stream into four bitplanes, reading interleaved source bytes into each pixel group and optionally inverting the bits.
- `wait_for_button_combination`: Waits until the fire and right button bits from the controller registers are both set, then returns as soon as either one is released.
- `place_record_marker_on_screen`: Advances a frame counter, then for each record in a list converts its world position to a cell through a sine-style table and divides by the cell size.
- `select_record_and_clear_buffer`: When the current record matches the object's position, it marks the state active and fills a 180-byte buffer with ones.
- `claim_free_record_slot`: When the record at the given position is unclaimed, it saves the current search state, scans records for the matching index, and claims the slot by copying the caller's position and attributes into it.
- `scan_record_chain_for_index`: Walks a chain of records, subtracting a stored step from the index at each hop until the index reaches zero.
- `draw_marker_rows_with_blitter`: Chooses a horizontal offset from a table using the record's count value, then writes blitter-style register values for three rows of masked copies at that position.
- `find_record_under_point`: Checks the record's flags and, when no special flag is set, walks a list of records looking for one whose position lies within a 150-unit box around the object.
- `select_record_by_flag_bit`: When the current record carries a special flag bit, it runs the record search and, if the search succeeds, makes the first word of the found record the new current index.
- `animate_marker_buffer_entries`: Advances an animation counter stored in the object and, while it is active, updates each entry of a marker buffer.
- `spawn_pending_records_from_list`: Walks up to eight pending entries and marks each one as used.
- `update_all_record_states`: Walks the object's record chain each frame and updates each record's state, timers and flags.
- `advance_record_timers_in_chain`: Walks a chain of records, advancing each record's small counter and setting its completion flag when the counter overflows.
- `resolve_pending_index_pair`: Compares a record index with two pending index slots and clears a slot when it matches.
- `call_five_routines_in_order`: Calls five unnamed routines in sequence and returns, with no logic of its own.
- `repeat_call_until_zero`: Calls an unnamed routine and, while it returns a non-zero value, calls a second unnamed routine and repeats.
- `scan_region_list_for_point`: Walks a list of region entries until a terminator, testing each entry's flags.
- `advance_pseudo_random_seed`: Multiplies a 16-bit seed by 125 and reduces it modulo 65521, then stores the new seed.
- `append_region_entry_from_points`: Marks the source entry as used, then walks a list of point deltas, accumulating coordinates.
- `terminate_region_entry_list`: Writes a sentinel value into the next entry and moves the free-list pointer on by one 64-byte entry.
- `init_record_from_table_entry`: Stores a new record pointer, then fills the record from a type table: it copies the table's image and attribute words and a block of eight words from the entry's data.
- `init_record_defaults_and_flags`: Copies the entry's position and attribute words into the record, then sets default timer, speed and state values.
- `stub_return_only_4`: Returns its stacked argument unchanged and does nothing else, so it is an empty stub called by one unnamed routine.
- `start_timed_state_sequence`: Initialises the object's state fields for a timed sequence, then runs one unnamed call and returns.
- `set_state_five_and_step`: Sets the object's state mode to five with a fixed set of control words, runs one unnamed call, and returns.
- `call_one_routine_and_return`: Calls one unnamed routine and returns, with the result left unchanged.
- `scan_list_for_clear_bits`: Sets the object's state mode to five and runs an unnamed routine, then walks a list of indices until it finds one whose low two bits are clear.
- `store_state_words`: Stores a fresh set of state words into an object's state fields at fixed offsets and then returns unconditionally.
- `stub_return_only_5`: Contains only a return and does nothing, so it is an empty stub.
- `empty_return_routine`: This routine is empty.
- `store_two_longs_in_record`: Stores two longword values into the record's offsets 56 and 60 fields and returns.
- `increment_global_byte_counter`: Adds one to a byte in a global array at an index passed in a data register, then returns.
- `check_objects_within_proximity`: Walks the active object list and, for each live entry, checks whether its position lies within a threshold distance of a related entry's position using absolute differences.
- `check_objects_within_proximity_2`: Runs the same live-object walk as its neighbour, clearing a byte field on entries marked for it, and performs the proximity check on the others.
- `update_object_targets_and_flags`: Clears a status flag on every object entry, then for each active entry checks its kind and a global shift value, computes a distance to a related entry, and updates the entry's flags and state from the result.
- `mark_dead_and_check_range`: Walks a counted list of entries, marking any whose kind is the excluded value or whose state byte is set as dead.
- `mark_dead_and_check_range_2`: Walks a counted list of entries in the same way as its neighbour, entered at a point that skips the dead-marking step.
- `propagate_status_bits_to_flags`: If a status bit is set in the entry's word, copies it into the low flag bit and sets a second flag bit chosen by two other status bits.
- `do_nothing`: This routine contains no instructions and performs no operations.
- `store_value_in_free_slot`: Scans a list of one hundred entries for the first one that is negative and stores the value from the data register there.
- `tiny_move_movea_rts`: Small routine of 3 instructions that copies (a7)+ to d1, then copies (a7)+ to a0, then returns.
- `find_nearest_candidate_entry`: Sets an entry's state byte from its flags, and when it is still active it scans a list of candidates for the one with the smallest distance.
- `find_nearest_candidate_entry_2`: Scans a list of candidates for the one with the smallest distance, stores it in the entry and clears one flag.
- `compute_scaled_squared_distance`: Computes the squared distance between two points after scaling each axis difference by one thousand, returning the sum of the squared scaled differences.
- `check_range_for_kind`: Skips entries flagged as inactive, then computes the distance between the entry and a candidate.
- `mark_entry_engaged_at_time`: Records the current time in the entry when it is in range, sets its engaged flags, and decrements a counter when the entry was newly engaged.
- `stub_return_only_6`: Compares its input against one value and returns in both cases without changing anything, so it has no effect.
- `select_largest_priority_field`: Scans a list of entries and picks the largest value among four priority fields, skipping entries flagged with a bit.
- `select_largest_priority_field_2`: Performs the same largest-value field selection as its neighbour, starting from the list position already set by the caller.
- `find_nearest_kind_match`: Walks a candidate list, tracking the nearest entry of one special kind by distance.
- `find_nearest_kind_match_2`: Repeats the candidate walk of its neighbour, keeping the nearest entry of the special kind and stopping when a candidate passes the flag checks.
- `walk_linked_list_until_match`: Sets the object's state words, then walks a linked list of entries.
- `empty_routine`: This routine contains no instructions and performs no operations.
- `walk_list_until_offset_match`: Walks a list of entries, subtracting each entry's stored offset from the current value.
- `record_event_in_free_slot`: Finds a free slot in one of two eight-entry tables, chosen by the kind code, and stores the entry's values in it.
- `refresh_links_and_clear_stale`: Re-links each active entry to its target's position and clears slots whose flags show they are finished.
- `update_and_clear_entries`: Walks a fixed-size block of entries, updating linked positions and clearing flags based on target state.
- `relink_entries_and_clear_slots`: Walks the entry list, clearing slots whose flags mark them finished, and resets each linked entry's position from its target.
- `snap_entry_to_cell_edge`: Skips entries flagged with a bit, then compares the entry's grid cell with a neighbouring cell from a table and snaps one coordinate to the cell edge.
- `cell_to_world_position`: Converts two grid cell coordinates into world positions by multiplying each by the 6000-unit cell size.
- `find_nearest_cell_rectangle`: Searches a list of rectangles for the one nearest to a point, measuring the squared distance from the point to each rectangle's edges.
- `find_nearest_cell_rectangle_2`: Walks the same rectangle list from a different starting point and keeps the one with the smallest squared distance to the point.
- `find_nearest_cell_rectangle_3`: Walks the rectangle list again, comparing squared edge distances and keeping the nearest rectangle.
- `lookup_cell_code_from_table`: Returns fixed codes for kinds five, six and seven and otherwise reads a code from a two-dimensional table indexed by kind and a second index.
- `read_table_word_at_index`: Returns the word stored in a global table at the index passed in a data register.
- `update_entry_timer_if_due`: When the entry's time falls within the current window, it runs an unnamed call and updates the entry's pointer.
- `select_timed_table_entry`: Returns early unless enough time has passed since its last run.
- `check_point_inside_rectangle`: Tests whether a point lies inside the rectangle given by four words at the input pointer.
- `start_table_driven_action`: Marks the input flag, and when the object's pending count is negative it sets an active flag, calls an unnamed routine with a table-indexed address for the object's kind, and clears the pending count.
- `start_table_driven_action_2`: Clears the input flag, then does the same table-driven start as its neighbour.
- `start_table_driven_action_3`: Performs the table-driven start with the table address passed in by the caller.
- `start_action_without_table`: When the object's pending count is negative, sets the active flag, runs two unnamed calls and clears the pending count.
- `load_kind_table_scale_value`: Looks up a byte in a five-byte-per-kind table for the object's kind, doubles it, and stores the result as the current scale word.
- `scale_kind_table_value`: Looks up a byte in the five-byte-per-kind table row for the object's kind minus two, multiplies it by a constant factor, and stores the result as the current scale word.
- `accumulate_weighted_kind_scores`: Looks up weight bytes for the object's kind and accumulates weighted values over a list of entries, dividing each by a per-entry byte.
- `accumulate_weighted_kind_scores_2`: Performs the same weighted accumulation as its neighbour, storing a scaled result for the entry it finishes on.
- `compute_level_from_ratio`: Divides a value by a per-entry byte and clamps the quotient to twelve.
- `set_state_and_step`: Sets the object's state to mode one with a fixed control word, runs one unnamed call, and returns.
- `return_helper_pair_values`: Invokes a helper routine that searches a flagged record list, then returns a pair of values taken directly from the incoming condition flags regardless of whether the flag is set.
- `increment_tally_for_flag_bit`: Finds the lowest set bit in a six-bit field of the entry's flags and increments the matching tally byte.
- `increment_tally_for_flag_bit_2`: Performs the same lowest-set-bit tally increment as its neighbour, on the table passed in by the caller.

### strikeiv.prg

- `start_music_playback_program`: Program entry: sets up the task, initialises the music player, installs the timer interrupt, waits for playback to be ended by the player, then restores the interrupt and stops the player.
- `Wait_For_End`: Loops until either the fire button or the right button is released, reading both from the controller port registers, then returns.
- `Set_Irq`: Programs the second CIA's interrupt control and timer registers with a fixed sequence of values, then points the interrupt handler at the new interrupt routine.
- `Return_Irq`: Writes a fixed value to the interrupt control register, restores the saved interrupt word, then writes a fixed value to the DMA control register.
- `Set_Task`: Finds the current task through the system library and sets its scheduling priority, using two library calls.
- `New_Interupt`: Runs on the timer interrupt: it clears the interrupt request and then calls the music routine for the next tick.
- `mt_init`: Initialises the ProTracker music replay: reads the song's pattern order table, sets up the sample pointers and resets the song position.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `mt_music`: ProTracker music replay tick, called once per frame: advances the song and feeds new notes and effects to the four Paula audio channels.
- `mt_wait`: Reads the vertical beam position register in a wait loop.
- `mt_nonew`: Calls a per-voice unnamed routine four times, then resets the speed and steps the song position forward, wrapping at 128 positions.
- `mt_playvoice`: Reads a four-byte note event for one voice from the current pattern and decodes its sample number.
- `mt_noloop`: Updates the voice's sample position and volume, then either sustains the current note, sets a slide target for the two slide effects, or starts the new note on its channel with the sample's start address and length.
- `mt_stopsound`: Applies one voice's effect command.
- `mt_setport`: Sets the voice's slide target from its period and resets the slide state.
- `mt_clrport`: Clears the voice's slide counter word, which stops any running slide.
- `mt_port`: Steps a voice's period up or down by its slide speed toward a target period.
- `mt_port2`: Performs the same slide step toward the target period as its sibling, stepping the voice period up or down and storing the result in the voice state.
- `mt_sub`: Lowers a voice's period by a given amount, stopping at the target period when it is reached, and stores the period in the voice state.
- `mt_vib`: Updates a voice's vibration speed and depth from its effect parameters, looks up a sine-table value for the current phase, and adds the scaled offset to the voice period.
- `mt_vib2`: Applies the same vibration offset as its sibling using the current phase and depth, without changing the stored speed or depth.
- `mt_vibsub`: Stores a period value minus an offset into the voice and advances the vibration phase by the speed part of the vibration state.
- `mt_arp`: Selects a note offset from the effect parameter for the current speed phase, then sets the voice period to the table period that the offset reaches from the current period.
- `mt_arp2`: Applies the arpeggio lookup using the low nibble of the effect parameter as the note offset, and falls back to the stored period when no table entry matches.
- `mt_arpfound`: Stores the period found by the arpeggio search into the voice state, reading it from the table entry at the given offset.
- `mt_normper`: Copies the voice's base period into its current period, cancelling any effect that changed it.
- `mt_com`: Reads a voice's effect command nibble and dispatches to the matching effect routine: arpeggio, slides, vibrato or volume combinations.
- `mt_portup`: Subtracts the effect parameter from the voice period, which raises the pitch, clamps the result at the lowest allowed period and stores it.
- `mt_portdown`: Adds the effect parameter to the voice period, which lowers the pitch, clamps the result at the highest allowed period and stores it.
- `mt_volvib`: Applies the vibration step, then slides the voice volume up by the upper nibble of the parameter or down by the lower nibble.
- `mt_volport`: Applies the slide toward the target period, then adjusts the voice volume by the parameter's upper or lower nibble with the same clamping as its neighbour.
- `mt_vol3`: Lowers the voice volume by the low nibble of the effect parameter, clamping it at zero, and stores the result.
- `mt_com2`: Applies the same effect-command handling as its sibling routines: a jump, a pattern break, a volume set or a speed set.
- `mt_filter`: Sets or clears the audio filter bit in the control register from bit 0 of the effect parameter, leaving the other control bits unchanged.
- `mt_pattbreak`: Sets the song position to one, which ends the current pattern early, and returns.
- `mt_songjmp`: Sets the song position to the effect parameter minus one, so playback moves to that song position after the current pattern.
- `mt_setvol`: Caps the stored volume byte at 64 and copies it into the channel volume field and the audio volume register of the voice.
- `mt_setspeed`: Reads the speed parameter, limits it to 31 and raises zero to 1, then stores it in the global speed variable.
- `mt_speed`: Small routine of 1 instruction that adds 0 to d0.
- `mt_songpos`: This routine is a music routine entry point that stores the value passed in register D0 into a global variable holding the current song position.
- `mt_samplestarts`: It writes the value 15 to a byte at a global variable address held in register D0, then stops.
- `mt_voice1`: It reads a byte from a location pointed to by register D0 and tests whether the low nibble of that byte is zero.
- `mt_voice2`: Reads a byte from the register passed in D0 and, based on its value, branches to one of two code paths.
- `mt_voice3`: It inspects the low bits of the byte passed in D0 and, depending on whether they are set, branches to one of two code paths.
- `mt_voice4`: It takes the byte in D0, ORs it with 8, and if the result is negative it branches to a data region that is not a valid instruction.

### sf.prg

- `start_program_entry_loop`: Program entry that writes a few globals and then loops forever calling the same routine.
- `open_read_close_dos_file`: Calls a helper, then opens a file through DOS; if the open succeeds it reads and closes the file before releasing the exec library.
- `write_dos_file_checked`: Opens a DOS file through a helper and writes to it through the DOS write entry, closing the file on success.
- `open_dos_library_base_3`: Opens dos.library through the exec old-open entry and stores the returned library base in a global variable.
- `fetch_io_error_and_close`: Fetches the DOS I/O error code, closes the file, closes the library base, and returns a failure status.
- `take_over_copper_and_interrupts`: Forbids task switching, opens graphics.library and saves its copper list pointer, then points it at a new list.
- `dispatch_copper_and_vblank_irq`: Interrupt handler that tests the interrupt request register.
- `handle_vblank_counter_tick`: Checks bit five of the passed flags and, when it is set, restores the saved copper list pointer, calls a routine and increments a counter.
- `fill_copper_table_and_delay`: Writes a repeating pair of 16-bit constants into a table of eight entries, then busy-waits through a nested counting loop.
- `wait_vblank_then_enable_interrupt`: Polls the interrupt request register until the vertical blank bit is set, then writes the interrupt enable value with that bit set to re-enable it.
- `split_bitplane_pointers_per_quarter`: Splits a base address into low and high 16-bit words and stores them as four bitplane pointer pairs, spaced 8000 bytes apart, the size of one 320 by 200 bitplane.
- `forward_library_call_once`: Makes a single jump through a library vector at a fixed offset with no arguments of its own, then returns the unchanged input registers.
- `permit_multitasking`: Calls the exec Permit entry to re-enable task switching and returns no value.
- `disable_task_switching_2`: Calls the exec Forbid entry and returns no value, so its only effect here is to stop task switching.
- `dispatch_indexed_handler_or_halt`: Calls an exec routine that returns an index; if the index is zero it forbids task switching and loops forever.
- `restore_saved_list_pointer`: Opens a library through exec, stores a saved pointer into one of its fields and into a global, then calls two DOS-base library routines.
- `copy_planes_from_buffer`: Reads four words per step from a 128-byte source buffer and writes one word to each of four bitplanes, repeating this for 4000 steps, effectively copying four planes of data from the buffer into memory.
- `expand_nibbles_through_lookup`: Splits a 32-bit value into eight 4-bit groups and, for each group, writes the matching entry of a 16-entry lookup table into a sequential buffer.
- `copy_four_planes_with_setup`: Makes several library calls, then copies 4000 words from a source offset 128 bytes in into four bitplanes, each plane 8000 bytes from the next.
- `copy_four_planes_from_buffer`: Reads four words per step from a source buffer 128 bytes in and writes one word to each of four bitplanes, 4000 steps in total.
- `allocate_screen_buffer_pointers`: Allocates a memory block through exec and stores its base address plus several derived pointers, spaced 32000 bytes apart, in global variables.
- `free_screen_buffer_block`: Releases the memory block whose address is held in a global by calling the exec FreeMem entry.
- `release_library_after_setup`: Calls two routines, then a library entry on the base at a fixed offset, closes a library and calls another routine.
- `decompress_lzss_stream_to_buffer`: Decompresses an LZSS-style stream: it first fills a 4096-byte window with spaces, then alternates between literal bytes and back-references copied from that window until the expected output length is reached.

### Strikefleet (1987)(LucasArts).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
