# Game 1760 (another world (1991)(u. s. gold)[cr company][h atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01760). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:intro.comp | 2 | 233 | 8 |
| Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:another | 1 | 3087 | 106 |
| Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 89 | 1 |
| Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:bank0D (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:bank01 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:bank02 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 2 of 2).adf:bank0A (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 2 of 2).adf:bank03 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 74 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- another world (1991)(u. s. gold)[cr company][h atx](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:bank0D: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr company][h atx](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:bank01: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr company][h atx](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:bank02: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr company][h atx](disk 2 of 2)/Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 2 of 2).adf:bank0A: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr company][h atx](disk 2 of 2)/Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 2 of 2).adf:bank03: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr company][h atx](disk 2 of 2)/Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 5 never-loaded region(s), 924354 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:intro.comp

## Copy protection found

- `stub_undecoded_library_copper_setup` (runtime segment an address): The decompiler could not finish this routine, so only its listed facts are known. Kind: loader/integrity check stub. Pass path: The routine likely performs a hardware-level verification (given the name 'undecoded_library_copper_setup' and the disk hardware identifier) and the execution flow following the call would proceed to the actual game/application data loading.

## Functions


### Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:intro.comp

- `load_executable_hunks`: Walks the AmigaDOS hunk records of an executable, comparing each record type with the hunk type codes.
- `delay_by_count_word`: Counts the incoming word down in a loop until it passes zero, so the loop length scales with the input value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_backward_compressed_stream`: Decodes a bit-packed compressed stream into a buffer by writing from the end toward the start, alternating between literal bytes and back-references that copy a run of already written bytes.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bit_buffer_by_count`: Repeats once per unit of the incoming word count, shifting a bit buffer by the count held in a register and loading the next value from memory when the buffer empties.
- `stub_undecoded_three_instructions`: The decompiler could not finish this three-instruction routine, so its body is not recovered.

### Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:another

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
- `format_word_as_text_digits`: Converts a 16-bit value into four text characters, one per nibble from the top, using digit characters for values up to nine and letter characters above that.
- `mark_records_within_budget`: Repeatedly selects the highest-priority type-three record whose side flag matches the input, marks it as type four with that flag, and subtracts its size from a running budget.
- `mark_chained_records_within_budget`: Marks a starting record as type four with the input flag and subtracts its size from a running budget.
- `subtract_marked_record_sizes`: Walks the record table and, for each record of type four, calls a routine for its size and subtracts that size from one of two running budgets, chosen by the record's side flag.
- `walk_marked_records_subtract_sizes`: Repeatedly subtracts an amount from one running value and walks the record table, calling a routine for each type-four record it finds.
- `return_stack_argument_unchanged`: Returns its stack argument unchanged and does nothing else.
- `copy_block_shift_pointers`: Copies a block of words from a source address up to a bound chosen by a flag into a lower destination.
- `copy_run_then_shift_pointers`: Copies a run of words from a source to a destination, then copies further full blocks, and subtracts an input amount from each record pointer above a boundary whose flag matches.
- `copy_long_blocks_shift_pointers`: Copies a number of full blocks of 32768 words from source to destination, then subtracts an input amount from each record pointer above a boundary whose flag matches.
- `return_null_value`: This routine is empty and performs no operations before returning.
- `dispatch_code_with_raster_sync`: Calls a routine first, then matches an incoming code against a set of fixed values and assigns slot indices and record values for a match.
- `clear_out_of_range_records`: Walks the twenty-byte record table and zeroes the first byte of every record whose type byte lies outside the range three to six.
- `clear_all_records_restore_values`: Zeroes the first byte of every twenty-byte record in the table, counting down a stored record count, then restores two stored values from backup copies.
- `count_table_entries_to_sentinel`: Counts twenty-byte table entries until a sentinel value is found and stores the count minus one in a global variable.
- `draw_line_with_blitter`: Sets up the blitter to draw a line between two screen points on a 40-byte-per-row bitmap, swapping the endpoints when a flag is set.
- `clip_line_then_draw`: Computes edge outcode flags for both endpoints of a line and returns without drawing when both endpoints lie outside the same edge.
- `return_without_effect`: Contains only a return, so it changes no memory or registers.
- `subdivide_segment_at_midpoint`: Repeatedly splits a line segment at its midpoint and calls a routine for each piece.
- `walk_scaled_polygon_vertices`: Reads a vertex count from the record, then scales and offsets each vertex by a scale factor and an origin.
- `record_polygon_row_crossings`: Walks polygon edges row by row, scaling each endpoint and storing the crossing positions for each row in a table that ends with a sentinel value.
- `store_blit_request_then_wait`: Stores a set of blitter request values, including two fixed values, into a record addressed by the base register.
- `wait_for_blitter_idle`: Loops reading the DMA status register and tests the blitter busy flag until it clears, then returns.
- `decode_joystick_direction_flags`: Reads a stored joystick input word and derives horizontal and vertical direction flags from its bits, using an exclusive-or of adjacent bits for the vertical direction.
- `copy_four_longwords_to_globals`: Calls a subroutine first, then copies four consecutive 32-bit values from the record at the incoming pointer into four global variables.
- `scale_small_value_by_sixteen`: Returns the input word unchanged when it exceeds three.
- `return_without_effect_2`: Contains only a return, so it reads and writes no memory or hardware registers and acts as a no-op.
- `draw_clipped_sprite_planes`: Checks that a sprite record lies within the visible screen, clipping its top and bottom rows as needed.
- `mask_four_word_targets`: ANDs a 16-bit mask value into each of four words, each reached through its own address register, which clears every bit not set in the mask.
- `mask_three_words_return_code`: ANDs a 16-bit mask value into three words reached through address registers, then returns the incoming byte with a fixed pattern of bits set.
- `combine_mask_bits_into_planes`: Works through up to four stored plane pointers, one per mask bit.
- `allocate_memory_or_fail`: Calls the system library's memory allocation function through its library vector.
- `free_memory_or_fail`: Calls the system library's memory release function through its library vector, then tests the returned value against zero and returns minus one when it is zero.
- `queue_row_draw_commands`: Chooses one of two paths depending on a signed input value.
- `apply_bit_mask_four_words`: Runs four passes over the bits of a mask value.
- `switch_display_mode_copper`: A sentinel code skips the stored-data step, another code swaps four long words, and any other code copies four words from the given record.
- `handle_keyboard_scancode`: Decodes a raw keyboard serial byte by rotating its inverted bits, then branches on the key code.
- `program_display_window_mode_b`: Writes fixed values to the display window start and stop registers, the data fetch start and stop registers and the bitplane control register.
- `update_display_registers`: writes values to several hardware registers to configure display parameters such as bitplanes, vertical scrolling, and horizontal scrolling.
- `clear_four_state_words`: Writes zero to four words in the state record addressed by the base register and clears a stored flag.
- `reset_two_words_set_pair`: Clears two words in the state record, sets two other words to forty, and sets a stored flag to one.
- `reset_input_state_from_table`: Sets one stored word to all ones, loads the low byte of a table entry into a second stored value, and clears a third stored value.
- `wait_for_raster_line`: Loops reading the vertical position register until the raster position reaches a fixed line, then returns its input value unchanged.
- `write_dma_control_register`: The decompiler could not finish this routine, so only its listed facts are known: it writes the DMA control register, where setting the top bit enables the channels named in the write, and it calls one other routine.
- `lookup_record_when_code_matches`: When the first argument is zero, it stores the second argument into a global variable if that argument is non-zero, and otherwise calls a routine.
- `open_ciaa_and_fill_table`: On first use it opens the ciaa resource through the exec library and calls a second library function, saving that result.
- `close_ciaa_resource_if_open`: When the open flag is set, it calls a library close function through a stored pointer and clears the flag.
- `load_global_from_fixed_cell`: Writes a constant into a global variable, then overwrites it with the value read from a fixed memory cell.
- `copy_stored_value_to_global`: Copies a value read from a fixed memory cell into a global variable.
- `clear_field_in_table_records`: Walks a table of fifteen record pointers and, for each non-null pointer, writes zero to the longword at offset eight within that record.
- `silence_audio_channels`: Writes zero to the volume register of each of the four audio channels, then clears two stored values.
- `silence_audio_channels_store_fifteen`: Writes zero to the volume register of each of the four audio channels, then stores the value fifteen in a global variable.
- `call_then_return_zero`: Calls one other routine, then returns zero as its result.
- `cycle_audio_channel_updates`: Increments a stored counter, and until it reaches six it calls a routine once for each of the four audio channel register blocks, passing the channel index.
- `run_routine_per_audio_channel`: Calls a routine once for each of the four audio channel register blocks, counting the channel index down from three to zero, and then returns its stack argument unchanged.
- `advance_channel_effect_state`: If the stored step value is non-zero, it adds the step to a stored position, clamps the result to a limit chosen by the step's sign, and stores it.
- `advance_position_with_lower_limit`: Adds an input word to a stored position and raises the result to a stored minimum when it is at or below that minimum.
- `set_channel_effect_parameters`: For effect code one it calls a routine through a jump table indexed by a stored value, and for codes seven and eight it stores a signed low-nibble parameter, the record's key word, and a value found by searching a fixed table for the key.
- `store_key_with_table_lookup`: Stores a low-nibble parameter from a record and the record's key word, then searches a fixed table for the key.
- `copy_word_with_signed_offset`: Copies one word from a source address offset by a signed index, scaled to word size, into a field of the destination record.
- `store_input_word_in_record`: Writes the incoming word into offset four of the record addressed by the destination base register.
- `store_signed_key_with_lookup`: Stores a low-nibble parameter from a record, negated, together with the record's key word.
- `dispatch_routine_by_index`: Calls a routine chosen from a table of routine pointers by a stored index, which is decremented by one and scaled by four before use.
- `lookup_table_value_by_key`: Finds the record's key word in a fixed table, then writes the entry at the incoming index counted from that match into the output field.
- `lookup_table_high_nibble_offset`: Finds the record's key word in a fixed table, then writes the entry at an offset from that match given by the high nibble of a record byte into the output field.
- `lookup_table_low_nibble_offset`: Finds the record's key word in a fixed table, then writes the entry at an offset from that match given by the low nibble of a record byte into the output field.
- `copy_record_word_to_output`: Copies the word at offset sixteen of the record into the output field.
- `start_audio_dma_channels`: Clears an enable word, calls a routine for each of the four audio channels, then writes the enable word with the DMA enable bit set.
- `process_channel_sequence_entry`: Processes one entry of a channel's sequence record, which supplies a value word, a data pointer and a count.
- `update_channel_volume_and_flags`: Adjusts a channel's volume by the entry's parameter according to its command nibble, adding with an upper clamp or subtracting with a lower clamp.
- `add_entry_volume_and_flags`: Sets the channel volume to a base word plus a record byte, clamped at the upper limit, then applies the same special-value handling and running-mask update as the other entry routines.
- `store_active_channel_entry`: For a non-zero entry, it waits briefly and then either silences the channel for the special silence value or copies the entry's pointer, length and value into the channel's output block.
- `copy_channel_entry_fields`: Copies a longword and two words from a channel record into the output block, then ORs the record's flag word into a running mask.
- `fill_bitplanes_by_mask`: Calls a routine first, then for each of four bitplane pointers fills a block of 8000 longwords with all ones or all zeros according to the matching bit of the input mask.
- `lock_df0_volume_as_current`: Releases any previously held DOS lock, waits for a keyboard code, pauses briefly, then locks the DF0 volume and makes it the current directory.
- `wait_for_disk_status_clear`: Loops reading the status port until its disk status bit clears, then calls one other routine.
- `read_file_toggling_blitter_ownership`: Releases the blitter, permits multitasking, and loops opening a file when no handle is held or otherwise seeking and reading from it, closing and reopening the file when the requested value differs from the stored one.
- `forbid_and_own_blitter`: Calls a routine, forbids multitasking and claims the blitter, then returns minus one.
- `store_word_in_delay_loops`: Stores the incoming word into a global location repeatedly within nested countdown loops, which acts as a timed delay.
- `delay_then_zero_global`: Runs a long countdown loop as a delay, then writes zero to one global location once per step of a second loop of 4096 steps.
- `open_libraries_and_layout_buffers`: Opens the DOS and graphics libraries by name and reads the available memory size.
- `toggle_dma_channel_bits`: Calls a routine, writes a DMA control value that clears one group of channel bits, calls another routine, and then writes a second value that sets a different group with the enable bit.
- `register_interrupt_server_node`: Allocates an interrupt node through a called routine, sets its data pointer to a global, its priority to one hundred and its type to two, and stores a handler routine's address.
- `tiny_addq_clr_rts`: Small routine of 3 instructions that adds 1 to (a1), then clears d0, then returns.
- `forbid_and_own_blitter_2`: Disables multitasking through the exec library, then claims the blitter through the graphics library.
- `permit_and_disown_blitter`: Releases the blitter through the graphics library, then re-enables multitasking through the exec library.
- `start_copper_list_from_struct`: Loads the copper list pointer from a field of a stored structure, strobes the copper jump register, and enables copper DMA through the DMA control register.
- `call_four_routines_return_zero`: Calls four routines in sequence and returns zero.

### Another World (1991)(U. S. Gold)[cr Company][h ATX](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_undecoded_library_copper_setup`: The decompiler could not finish this routine, so only its listed facts are known.
