# Game 1761 (another world (1991)(u. s. gold)[cr company][h prx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01761). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802512 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 121 of 121 named (0 author symbols), 121 of 121 described (5 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1592 of 1592 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 924354 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3543 of 3543 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 561016 |
| code | 12250 |
| data | 1089300 |
| padding | 5802 |
| unused | 134144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:intro.compp | 2 | 233 | 8 |
| Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:intro.comp | 1 | 155 | 3 |
| Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:another | 1 | 3087 | 106 |
| Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 58 | 3 |
| Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:bank0D (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:bank01 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:bank02 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 2 of 2).adf:bank0A (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 2 of 2).adf:bank03 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- another world (1991)(u. s. gold)[cr company][h prx](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:bank0D: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr company][h prx](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:bank01: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr company][h prx](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:bank02: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr company][h prx](disk 2 of 2)/Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 2 of 2).adf:bank0A: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr company][h prx](disk 2 of 2)/Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 2 of 2).adf:bank03: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr company][h prx](disk 2 of 2)/Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- another world (1991)(u. s. gold)[cr company][h prx](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:intro.comp: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 5 never-loaded region(s), 924354 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:intro.compp
- Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:intro.comp

## Copy protection found

- none found

## Functions


### Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:intro.compp

- `parse_amigados_hunk_records`: Walks AmigaDOS hunk records by comparing each record type against the hunk type codes, allocating and freeing memory through the exec library for sections, and processing relocation data that follows.
- `spin_countdown_delay`: Counts a 16-bit register value down to zero in a tight loop, creating a short busy delay.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_backward_bit_stream`: Decodes a bit-coded stream backwards into a buffer: it checks a shifted flag, reads a count and a value from the bit reader, and copies runs of bytes until the output reaches its start.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bits_from_buffer`: Shifts bits out of a 32-bit bit buffer one count at a time, refilling from memory when the buffer empties and accumulating the bits into a register.
- `examine_unresolved_routine_body`: Pseudo-code could not be produced for this three-instruction routine, so its behaviour is not established.

### Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:intro.comp

- `load_hunk_stream_sections`: Walks an AmigaDOS hunk stream by comparing record types with hunk type codes and copies section data into memory, calling four routines per step.
- `refill_bit_buffer_longword`: Reads the longword before the input pointer, writes its low byte through an output pointer and returns the longword shifted right by one bit with its top bit set.
- `shift_bit_buffer_by_count`: Shifts a bit buffer right by a count, one bit at a time, and when the buffer empties it calls a routine to refill it.

### Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:another

- `configure_timing_and_wait`: Calls four routines, stores a timing value that is larger when a status flag bit is set, waits until a second status bit clears, clears a value, then calls four more routines.
- `map_event_code_to_slots`: Compares an event code against a series of fixed codes; each match selects three slot numbers, and one code also stores a value in a structure.
- `reset_state_and_dispatch_events`: Clears a 128-entry table to minus one and 32 state globals, then copies pending values between two tables with a scaled correction for active entries, and dispatches event codes through a comparison chain.
- `interpret_command_stream`: Interprets a command stream: opcode bytes set or clear flags and select compare-and-branch operations on table values, and the routine returns when the pointer passes its block limit.
- `enter_interpreter_if_zero`: Two-instruction entry that tests the zero flag and continues into the shared command loop, which reads the next command.
- `enter_interpreter_if_not_zero`: Two-instruction entry that tests whether the zero flag is clear and continues into the shared command loop, which reads the next command.
- `enter_interpreter_if_greater`: Two-instruction entry that continues into the shared command loop only when the zero flag is clear and the overflow and negative flags agree.
- `enter_interpreter_if_at_least`: Two-instruction entry that continues into the shared command loop when the overflow and negative flags agree.
- `enter_interpreter_if_less`: Two-instruction entry that continues into the shared command loop when the overflow and negative flags differ.
- `interpret_commands_with_flags`: Interprets a command stream like its neighbouring routine, with compare-and-branch operations that use the condition flags.
- `draw_scaled_object_record`: Decodes an object record with a six-bit type, computes scaled screen positions for the basic types, and for one record type writes bits into a 40-byte-stride buffer when the position lies within a 320 by 200 area, then loops over a count.
- `walk_flagged_word_pairs`: Loops over a count, reading word pairs and calling routines for each pair, and advances through the data by one word or three depending on whether the top bits of the pair are set.
- `compute_record_address`: Returns the address of an entry in a table of 20-byte records, computed from the index held in the data register.
- `scan_records_and_emit_text`: Searches a zero-terminated table for a matching key, then reads a text string byte by byte, skipping carriage returns and passing each other character to a routine until the terminator.
- `blit_masked_tile_rows`: After checking that the column is under 40 and the row is under 193, applies an eight-row pattern to byte positions in three bitplanes, setting or clearing bits according to the mask.
- `unpack_bit_reader_back_references`: Depacker loop: reads flag bits through the bit reader and, for each flag, either copies a literal byte or a back-reference of a length taken from a short code table, until the output count ends.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bits_refill_with_sentinel`: Shifts a 32-bit bit buffer right by a count, refilling it from the previous longword with its top bit set whenever it empties.
- `process_data_table`: Iterates through a table to find a specific entry based on matching criteria and updates various global values and flags.
- `format_word_as_text_digits`: Converts a 16-bit value into four ASCII characters, one per nibble, using letters for values above nine, and leaves the last character in the result register.
- `allocate_entries_within_budget`: Repeatedly takes the highest-priority matching entry, marks it active and subtracts its size from a budget, until the budget is exhausted, then calls a routine.
- `mark_queued_entries_until_budget`: Marks entries from a starting pointer as active, subtracting their sizes from a budget and choosing the next highest-priority matching entry each time, until the budget runs out.
- `subtract_sizes_from_running_totals`: Walks the entry table and, for each entry of kind four, calls a routine and subtracts its returned size from one of two running totals, chosen by a flag bit in the entry.
- `subtract_budget_for_kind_four`: Subtracts an amount from a budget, then walks the kind-four entries, calling a routine for each and subtracting its returned size from a running total unless a flag bit is set.
- `return_stack_pair_unchanged`: Returns the value passed on the stack without any other work.
- `copy_buffer_up_to_limit`: Copies 16-bit words from an input pointer to an output pointer, with the count taken from the distance to one of two limit globals selected by a flag.
- `copy_words_in_full_blocks`: Copies a first run of words, then copies a series of full blocks of 32,768 words each, repeating for the outer count.
- `copy_blocks_and_adjust_entries`: Copies a number of full blocks of 32,768 words, then walks the entry table and subtracts a value from each counter whose stored position is above a limit and whose kind bits match.
- `return_null_value`: This routine is empty and performs no operations before returning.
- `dispatch_events_with_raster_timing`: Dispatches event codes through a comparison chain, storing timing values in globals and calling routines for each event.
- `clear_unmatched_entry_kinds`: Clears the first byte of each 20-byte entry whose kind byte falls outside the range three to six, then restores two saved timing values.
- `clear_all_entry_flags`: Zeroes the first byte of every 20-byte entry, then restores two timing values from saved globals.
- `count_table_entries_to_sentinel`: Counts 20-byte table entries until a sentinel value is found and stores the count minus one in a global.
- `draw_blitter_line_segment`: Computes the absolute deltas and the octant bits between two points, waits for the blitter to go idle, and programs the blitter line-mode control, modulo and size registers.
- `clip_line_to_region`: Computes region codes for both segment endpoints and returns early when they share a region bit.
- `stub_empty_return`: Contains only a return with no loads, stores or calls, so it does no work.
- `bisect_segment_midpoints`: Repeatedly halves a segment by computing the midpoint of its coordinates, calls a test routine on each midpoint, and keeps the half that still matches until the endpoints converge.
- `trace_polygon_outline_spans`: Walks a byte-encoded polygon outline, scales each vertex by a stored factor and clamps the vertical position to 0 through 199.
- `trace_polygon_outline_fill`: Walks a byte-encoded polygon outline with the same scaling and vertical clamping as its sibling, recording boundary pairs for each scanline.
- `initialise_draw_request_record`: Stores a value, a constant two, and two minus-one sentinels into a fixed-offset record, then stores pointer values and copies one word into four more fields.
- `wait_blitter_idle`: Spins on the DMA status register until the blitter busy bit clears, then returns.
- `read_joystick_direction_state`: Decodes the joystick port movement word into horizontal and vertical direction values of minus one, zero or one, and sets matching direction bits in a state word.
- `copy_four_longs_to_globals`: Calls a routine that returns a pointer, then copies the four consecutive 32-bit values at that pointer into four global variables.
- `shift_small_index_left_four`: Returns the input word unchanged when it is greater than three, and otherwise returns it shifted left by four bits.
- `stub_empty_return_routine`: Contains only a return instruction, with no loads, stores or calls.
- `draw_clipped_sprite_columns`: Reads a record from a table, rejects it when its horizontal position lies outside the screen width or its vertical span lies outside the screen height, and otherwise clips the top and bottom rows.
- `mask_four_words_in_place`: ANDs one mask value into four separate 16-bit memory locations, clearing the masked bits in each.
- `mask_three_words_return_byte`: ANDs one mask value into three 16-bit memory locations and returns the input byte with a fixed bit pattern ORed into it.
- `apply_plane_bit_mask`: For each of up to four bitplane pointers in a table, either sets or clears the same bit in the word at that pointer, depending on the matching bit of a mask value.
- `allocate_memory_block`: Calls the system memory allocator through the exec library base and returns its result.
- `release_memory_block`: Calls the system memory release routine through the exec library base and returns its result.
- `queue_wrapped_draw_requests`: Fills one or more draw request records with fixed type and flag words and with positions taken from tables plus an offset.
- `queue_masked_draw_requests`: Creates four draw request records in a loop, one per low bit of a mask value, each with a fixed type word and a value chosen by that bit.
- `install_copper_list_palette`: Optionally swaps four stored long values, copies a 32-byte palette block for a selected index into the palette buffer, and waits for a raster line.
- `handle_keyboard_scancode`: Decodes the keyboard serial byte by inverting and rotating it, then for a few key codes resets the display registers and clears game state.
- `reset_display_window_mode`: Writes fixed display window, data fetch and bitplane control values, clears the scroll and state variables, and sets a flag.
- `reset_display_window_blank_mode`: Writes the same display window and data fetch values as its sibling routine, but sets the bitplane control to a blank mode.
- `clear_control_state_words`: Clears four 16-bit state words in a control record and sets a flag to zero.
- `set_control_state_defaults`: Clears two 16-bit state words, sets two others to forty, and sets a flag to one.
- `latch_keyboard_byte_state`: Sets a flag to minus one, stores the low byte read through a pointer into a state word, and clears a second state word.
- `wait_for_raster_line`: Spins reading the vertical position register until the beam reaches a fixed scanline, then returns the input value unchanged.
- `service_audio_channel_playback`: When a stored level value has changed, reloads its data and resets the DMA enable bits.
- `select_active_resource_entry`: When the first argument is zero it stores the second argument as a count, or leaves it unchanged.
- `open_cia_resource_and_cache`: Copies up to fourteen 32-bit values from a record list into a global table, keeping only entries of one fixed type code.
- `close_cached_resource_if_open`: When a stored handle is nonzero, calls a library routine at a fixed offset with that handle and then clears the stored handle.
- `write_cia_control_pair`: Writes a fixed byte with its top bit set to a CIA-A register, then writes a stored byte to the same register.
- `write_cia_timer_value_byte`: Writes a stored byte to a CIA timer register, then writes it again.
- `clear_resource_slot_fields`: Walks fifteen table entries and, for each non-null pointer, clears the 32-bit value stored eight bytes into the record it points to.
- `silence_audio_channels`: Zeroes the volume register of each of the four audio channels and clears two sound state words.
- `silence_audio_and_set_mask`: Zeroes the volume register of each of the four audio channels and sets a sound state word to fifteen.
- `call_routine_return_zero`: Calls one unnamed routine and returns zero.
- `advance_audio_tick_counter`: Increments a tick counter, and when it reaches six it resets the counter and runs a follow-up routine.
- `call_routine_per_audio_channel`: Calls a per-channel routine for each of the four audio channel register blocks, from the highest index down to zero.
- `step_audio_envelope_channel`: Advances one audio channel's level by its step and clamps the result to a target.
- `advance_audio_level_step`: Adds the step value to a channel level, clamps the result so it does not fall below a stored floor, and writes it to the channel's volume field.
- `load_audio_note_entry`: Reads the effect code of a sound sequence entry.
- `load_note_step_and_lookup`: Stores a zero, then copies a value from a sound entry, a low nibble of a field as the step, and a second value looked up in a table keyed by the entry's first value.
- `copy_word_with_signed_offset`: Computes a signed word offset from an input value, with the sign chosen by a flag, and copies the word at that offset from a source address into a destination derived from another register.
- `store_word_at_record_offset`: Stores the input word at a fixed offset inside the record pointed to by an address register.
- `load_signed_audio_note_entry`: Takes the step sign from a register parameter, then fills the channel step and target from a sound entry and looks up a second value in a table.
- `dispatch_channel_handler_by_state`: Calls a handler selected from a jump table by a state value minus one, with each table entry four bytes apart.
- `lookup_table_entry_by_key`: Searches a short table for the value stored at a record offset, then copies the element at an index given by a register into another field of the record.
- `lookup_entry_by_high_nibble`: Searches a short table for the key stored in a record, then copies the element selected by the high four bits of a byte field of the same record.
- `lookup_entry_by_low_nibble`: Searches a short table for the key stored in a record, then copies the element selected by the low four bits of a byte field of the same record.
- `copy_record_word_to_slot`: Copies one 16-bit word from a fixed offset in one record into a fixed offset in another record.
- `commit_audio_channel_pointers`: Calls a per-channel routine for each of four channels, then loads the sample pointer and length of each active channel into its audio registers.
- `load_sound_channel_from_entry`: Reads a sound sequence entry, then sets the channel's sample pointer and length from a table of sound data.
- `finish_sound_channel_setup`: Sets the channel's start value and volume using the same effect rules as its sibling routine.
- `apply_sound_volume_and_store`: Adds a volume offset from a register to a sound entry's level, caps the result at sixty-four and stores it.
- `store_sound_channel_record`: When the sequence code of an entry is nonzero, stores the channel fields and marks the channel's bit in an active-channel word.
- `store_channel_sample_fields`: Copies the sample pointer, length and sequence code from a sound entry into a channel record, then sets the channel's bit in an active-channel word.
- `fill_buffers_with_mask_pattern`: For each of four buffers, fills 8000 consecutive 32-bit words with either all ones or zero, chosen by one bit of a parameter.
- `lock_df0_disk_directory`: Releases any lock it holds through DOS, waits briefly, and then locks the DF0 drive through DOS, storing the lock handle.
- `wait_cia_port_bit_clear`: Spins while a status bit of the CIA-A port register is set, then calls one routine.
- `load_file_with_blitter_ownership`: Calls a stored routine in a retry loop until it succeeds, reusing a cached handle when the requested sector matches.
- `forbid_and_own_blitter_checked`: Disables multitasking and takes ownership of the blitter through the library bases, then returns minus one by default.
- `write_background_color_with_delay`: Writes the input word to the background colour register in a nested loop, which spaces out the writes with a fixed delay.
- `blank_background_color_with_delay`: Waits through a long delay loop, then writes zero to the background colour register 4096 times with a delay.
- `initialise_buffers_and_libraries`: Opens the DOS and graphics libraries and reads the free memory size.
- `reset_dma_channel_bits`: Clears one group of DMA enable bits in the DMA control register, then sets a different group, with calls to two routines before and after the writes.
- `install_interrupt_server_node`: Fills an interrupt server node with a priority of one hundred, a type code of two, a data pointer and a code pointer to a routine, then registers it through the exec library.
- `tiny_addq_clr_rts`: Small routine of 3 instructions that adds 1 to (a1), then clears d0, then returns.
- `forbid_and_own_blitter`: Disables multitasking through the exec library, then takes ownership of the blitter through the graphics library.
- `release_blitter_and_permit`: Releases the blitter through the graphics library, then re-enables multitasking through the exec library.
- `start_copper_list`: Loads the copper list address from a stored record into the copper pointer register, strobes the copper restart register, and writes the DMA control word that enables the copper.
- `call_four_routines_return_zero`: Calls four unnamed routines in turn and returns zero.

### Another World (1991)(U. S. Gold)[cr Company][h PRX](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_raster_synced_callback_loop`: Disables multitasking, programs the DMA control word, then loops until a status bit clears.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `examine_undecompiled_routine`: The decompiler timed out, so no pseudo-code exists for this sixteen-instruction routine.
