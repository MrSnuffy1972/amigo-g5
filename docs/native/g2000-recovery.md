# Game 2000 (operation stealth (1990)(u. s. gold)[cr pdx](disk 1 of 3)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02000). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (934768 top-level bytes in 29 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 541 of 541 named (0 author symbols), 541 of 541 described (56 by rule, 137 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1662 of 1662 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9371 of 9372 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 742701 |
| code | 30142 |
| data | 155146 |
| padding | 5755 |
| unused | 1024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| NARRATOR.DEVICE | 3 | 3782 | 268 |
| MESSAGE | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:KRAYZI | 1 | 99 | 3 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:endcli | 2 | 117 | 2 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:Intro | 2 | 174 | 12 |
| TRANSLATOR.LIBRARY | 2 | 490 | 50 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:PAUSE | 2 | 3 | 1 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:delphine | 2 | 207 | 8 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:add21k | 3 | 637 | 38 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1030 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 413 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2375 | 100 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 407 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 130 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 904 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- operation stealth (1990)(u. s. gold)[cr pdx](disk 1 of 3)[a2]/Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:delphine: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:Intro

## Copy protection found

- none found

## Functions


### NARRATOR.DEVICE

- `create_narrator_device_library`: Entry routine for the narrator device file that builds its library structure, allocates memory and a signal, finds the current task, then adds the device and starts its task.
- `open_device_unit_request`: Device open handler that, when the unit is already open, increments the open count and fills a request block with fixed setting values.
- `open_audio_device_unit`: Opens the audio device for the first unit, allocating a request block and returning a failure code if the open or the allocation fails.
- `create_named_command_port`: Fills in a port structure named command port and registers it through a library call.
- `set_request_defaults_and_count`: Increments the unit open count and a request sequence counter, copies the sequence value into a request block, writes fixed default settings into it, and clears a flag bit in the device block.
- `set_request_status_byte`: Stores a status byte into a request block and marks its completion field as minus one, then returns.
- `queue_request_with_interrupts_masked`: Queues a request block onto the device's pending list, turning interrupts off while the list is changed and back on afterwards.
- `insert_request_into_pending_list`: Places a request block onto the device's pending list with interrupts disabled during the change.
- `update_or_insert_request_entry`: Checks whether a request matches the current head entry and, if so, copies its nibble-split value across.
- `copy_byte_and_split_nibbles`: Copies one byte from a source structure into a destination structure, storing it whole and as low and high nibbles in separate fields.
- `validate_and_apply_request_fields`: Checks each numeric field of a request block against fixed ranges, such as a first range field from 1 to 400 and a second from 0 to 64, and copies the accepted values into the device state.
- `validate_request_ranges_and_apply`: Applies the same range checks as its neighbouring routine, copying accepted range, flag and rate values into the device state.
- `close_unit_when_last_reference`: Searches the pending list for a matching request and reduces the open count.
- `decrement_counter_and_close`: Decrements the unit's open count and, when it reaches zero, frees the memory block and closes the device.
- `dispatch_request_by_command`: Reads a command number from the request block.
- `unlink_request_or_mark_busy`: If a request block has no pending link, it unlinks the block from its list, frees it through the operating system, and returns the result.
- `unlink_node_and_free_block`: Calls two operating system library routines, unlinks a node by joining its neighbours, frees that memory, and returns a value taken from a field of the node.
- `dispatch_narrator_request_command`: Command dispatcher for the narrator device.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `mark_command_seven_pending`: Clears a busy flag in the request block, records command seven in the device block, sets a pending bit there, and makes a library call through the device base.
- `mark_command_six_pending`: Clears a busy flag in the request block, records command six in the device block, sets a pending bit there, and makes a library call through the device base.
- `run_with_busy_flag_set`: Sets bit seven of a status byte in a block, calls two other routines, then clears that bit again.
- `run_busy_then_clear_status`: Sets bit seven of a status byte, calls a routine, clears that bit, then clears a status byte in a second block and calls a further routine.
- `flush_pending_requests_masked`: Calls a routine if the device has a current request, then walks the pending list and calls a routine for each entry, with interrupts turned off during the walk.
- `release_interrupt_lock`: Decrements an interrupt nesting count held in a device byte, and when the count drops below one it turns interrupts back on.
- `abort_pending_request`: Turns interrupts off, then searches the pending list for a request block.
- `remove_request_from_pending_list`: Searches the pending list for a request block and, when it finds it, unlinks it and sets its status to an error value.
- `unlink_request_and_release_lock`: Unlinks a request block from its list, sets its error status, reduces the interrupt nesting count and re-enables interrupts when the count drops below one, then calls one more routine.
- `release_interrupt_lock_2`: Decrements an interrupt nesting count held in the device block and re-enables interrupts when the count drops below one.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `set_error_status_then_call`: Writes a specific error status into a request block and then calls one more routine.
- `set_success_status_then_call`: Clears the status byte of a request block to zero and then calls one more routine.
- `clear_busy_flag_and_call`: Clears a flag bit in the device block and a byte in the request block, then calls a library function through the device base.
- `validate_request_fields_and_finish`: Checks a request block's numeric, flag and rate fields against fixed ranges, copies accepted values into the device state, and runs the completion path.
- `validate_from_first_range_field`: Entry point into the shared request validation chain at its first range field.
- `validate_from_second_range_field`: Entry point into the shared request validation chain at its second range field.
- `validate_from_first_flag_field`: Entry point into the shared request validation chain at a flag field.
- `validate_from_second_flag_field`: Entry point into the shared request validation chain at a second flag field.
- `validate_from_rate_field`: Entry point into the shared request validation chain at the rate field.
- `validate_from_level_field`: Entry point into the shared request validation chain at a field allowed from zero to 64.
- `finish_request_and_release_memory`: Completion path shared by the validation exits.
- `copy_field_and_set_status`: Copies one longword from a source field into a destination field, stores the status byte passed in the data register, and calls one more routine.
- `clear_field_and_set_status`: Clears a longword field in a block, stores the status byte passed in the data register, and calls one more routine.
- `search_text_for_punctuation_break`: Searches backwards through a text buffer for the last comma or hyphen and records that position as the break point.
- `search_text_for_space_break`: Searches backwards from about three quarters of a text buffer for a space and records that position as the break point.
- `record_half_length_break`: Records the half-way point of a text buffer as the break position, then runs the completion path, which releases memory when a flag byte is set.
- `initialise_speech_request_block`: Clears the counters of a speech request block and sets fixed starting values, then enters supervisor mode through the operating system.
- `allocate_speech_block_from_template`: Uses a value from the request block as an index into a handler table, allocates a named message port structure, and copies a thirty-three-word template into a newly allocated block.
- `initialise_speech_port_and_signal`: Initialises a supplied message port as a named speech request port and allocates a signal for it.
- `allocate_and_copy_template_rows`: Stores a count byte in the request block, allocates a memory block, and copies a thirty-three-word template into each of several rows of it.
- `copy_template_into_rows`: Stores a pointer in the request block and copies a thirty-three-word template into each of several rows at that pointer, repeating for a count held in a byte.
- `perform_io_and_release_blocks`: Stores a mode byte in the request block, performs the I/O request synchronously, frees the memory blocks it owns, clears a length field, and returns a fixed combined value.
- `perform_io_then_free_resources`: Stores a mode byte in the request block, performs the I/O request synchronously, then frees a signal and the memory blocks it owns.
- `translate_text_to_phonemes`: Main text-to-phoneme translation loop.
- `translate_text_without_length_check`: Same translation loop as its neighbour but without the initial length check.
- `continue_phoneme_translation_loop`: Re-enters the phoneme translation loop partway through, looking up the current input character in the character-class table and handling special marker codes before storing output.
- `resume_phoneme_translation_scan`: Resumes the phoneme translation scan at its end-of-word checks, where a zero or hash-mark character ends the scan and a numeric marker is converted to its digit character.
- `resume_phoneme_translation_emit`: Resumes the translation loop at the point where it writes the character class and checks the output buffer limit, then either returns or continues scanning the input.
- `emit_phoneme_and_advance`: Writes the phoneme code for the current input character into the output buffer, setting flag bits for two special marker codes, then either continues scanning or returns the output position.
- `set_marker_flag_then_scan`: Re-enters the phoneme translation loop at a marker-six check, setting a flag bit on the previous output entry before scanning the next input character.
- `store_output_character_and_check`: Stores the current output character, advances the output position and checks the buffer limit.
- `scan_next_input_character`: Scans the next input character, looking it up in the character-class table and either emitting a code or converting a digit marker.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends immediately.
- `finish_output_and_record_length`: Trims or extends the last output entry, writes end markers into three output buffers, and records the input length consumed and the output count before returning.
- `append_end_markers_and_count`: Adds an optional separator code after the last output entry when a table flag allows it, writes end markers into three output buffers, and records the consumed length and output count.
- `record_output_count`: Stores the output count passed in the data register into the translation state block.
- `propagate_stress_markers`: Walks the phoneme output list up to its end marker and, for each entry whose class flags allow it, sets an emphasis bit on that entry and on the entries before it.
- `propagate_stress_markers_loop`: Loop body of the stress-propagation pass.
- `propagate_stress_markers_entry`: Alternative entry into the stress-propagation pass that applies a class-flag test to the previous entry before stepping through the phoneme list.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `propagate_phoneme_flags_forward`: Walks the phoneme list and sets a marker bit on each entry, copying stress and emphasis bits from the entry before it into a run of following entries when a class flag applies.
- `propagate_flags_over_span`: Continuation of the phoneme flag pass that fills a span of entries with a value derived from the preceding phoneme, then resumes scanning the list until the end marker.
- `fill_span_flag_values`: Fills the entries of a span with a flag value derived from the phoneme before it, then resumes the scan of the phoneme list until the end marker or a stop code.
- `fill_span_flags_fragment`: Decodes as an unreached stretch that repeats the span-filling step of the flag pass, so its behaviour matches the neighbouring fill routine.
- `empty_return_stub_2`: Routine that contains only a return, so it does nothing and ends immediately.
- `match_text_rules_against_input`: Scans a table of translation entries, each holding a pattern of up to three characters with wildcards and a pair of output codes.
- `continue_rule_pattern_match`: Continues the pattern-matching scan at the characters after the current pattern, checking the neighbouring input character against the entry's wildcard pattern.
- `advance_rule_pattern_scan`: Resumes the pattern-matching loop after an output step, checking for the end-of-table marker and advancing to the next table entry when the pattern does not fit.
- `scan_rule_table_entries`: Re-enters the pattern-matching loop at its scan step, testing each table entry's three-character pattern against the input and, on a match, applying the entry's output codes before scanning on.
- `test_rule_condition_bits`: Loops over a condition byte and a table word indexed by a counter, inverting or masking the word according to two flag bits in the byte.
- `insert_shift_three_buffers`: Increments a word counter while it is below its limit, then shifts the bytes of three parallel buffers one place to the right from a given position.
- `shift_three_buffers_and_clear`: Increments a word counter and shifts bytes backward through three parallel buffers by a computed count.
- `call_seven_routines_return_zero`: Calls seven other routines in turn and returns zero.
- `clear_table_flags_mark_end`: Walks a table of eight-byte entries until the end marker, clearing the last byte of each entry.
- `clear_table_flags_until_match`: Clears the last byte of each eight-byte table entry until it reaches an entry whose first byte matches the value passed in the data register.
- `process_phoneme_entry_values`: Walks the phoneme list and writes per-entry values.
- `process_phoneme_entries_inline`: Inline variant of the phoneme entry pass, with the same special-code, lookup and flag steps, running until the end marker.
- `process_phoneme_entries_alternate`: Alternative entry into the phoneme entry pass.
- `process_phoneme_entries_bit_entry`: Entry into the phoneme entry pass that takes a flag value directly from the data register, applies the lookup step, then loops over the remaining entries until the end marker.
- `set_class_flag_and_continue`: Sets a flag bit on the current output entry according to two class bits in the data register, then continues the phoneme entry pass with the next entry.
- `set_class_flag_then_scan`: Sets one of two class flag bits on the current output entry according to one bit of a flag value, then scans forward through the phoneme list, handling special codes and returning at the end marker.
- `set_class_flag_from_bits`: Sets a default class flag on the current output entry, then chooses one of several class flags from bits in the next entry's flag value.
- `continue_phoneme_entry_scan`: Scans the phoneme entry list from the current position, copying each entry's lookup byte to the output and applying class flags.
- `allocate_phoneme_output_tables`: Adds up the values in a phoneme flag table until its end marker, stores the total and the size needed for the output table, then allocates memory for the output.
- `allocate_output_tables_for_count`: Stores a count and the output size derived from it, allocates a block for the output table, and when a flag byte is set allocates a second one.
- `allocate_secondary_output_table`: Stores a block address passed in the data register, and when a flag byte is set allocates a second memory block for the output.
- `build_phoneme_frame_table`: Builds the output frame table.
- `build_phoneme_frame_table_alternate`: Alternative frame-table builder with the same per-phoneme lookups, stress adjustment and eight-byte frame records.
- `build_phoneme_frames_with_lookbehind`: Frame-table builder that writes eight-byte records per phoneme as the other builders do.
- `interpolate_frame_values`: Interpolates six values across each run of frame records, moving each value toward the next entry's value by a weight taken from the phoneme's table.
- `interpolate_frame_values_loop`: Continues the frame interpolation pass, moving six values toward the next entry's values by a weight taken from the phoneme table.
- `interpolate_frame_values_alternate`: Alternative interpolation pass over the frame list, moving each value toward its neighbour by a weight from the phoneme table.
- `interpolate_frame_values_tail`: Interpolation step that works through a run of frame records, computing each value from its neighbours and a weight, and stops at the end marker.
- `interpolate_frame_values_step`: Single interpolation step that moves one frame value toward its neighbour by the weight in the table, advancing to the next record when the count runs out.
- `write_phoneme_frame_records`: Writes frame records for the phoneme list, using each phoneme's class bits to decide how many frames to emit and what value to store in each.
- `write_frame_records_from_offset`: Alternative frame-record writer that starts from the current frame pointer and writes zeroed records for each phoneme's duration, with fill bytes when a flag is set.
- `write_frame_records_tail`: Tail variant of the frame-record writer that fills zeroed records for a phoneme's span, then loads the next phoneme's class bits.
- `write_frame_records_entry`: Entry into the frame-record writer that starts from the first phoneme's lookup value, writes the span of frame records, and returns zero at the end marker.
- `write_frame_records_with_stress`: Frame-record writer variant that sets a stress-related class value from the following phoneme and writes records for each phoneme's span until the end marker.
- `write_frame_records_with_flags`: Frame-record writer that checks the class flags of the current phoneme and writes records for its span, using the following phoneme's flags to fill the remaining entries.
- `write_frame_records_fill_value`: Writes zeroed frame records for each phoneme's span, using a fill value from the data register for the flag bytes when the entry is not marked.
- `write_frame_records_from_pointer`: Writes a counted run of zeroed frame records, then writes the spans for each following phoneme, returning zero at the end marker.
- `write_frame_records_counted`: Writes a counted run of zeroed frame records, then walks the phoneme list and writes a span for each entry until the end marker.
- `write_frame_records_after_first`: Writes one zeroed frame record, then a counted run of records for the remaining phoneme span, and returns zero at the end marker.
- `write_frame_records_by_count`: Writes frame records for the phoneme list using a count taken from each entry's class bits, and returns zero at the end marker.
- `filter_phoneme_entries`: Copies the phoneme list into three output arrays, skipping any entry whose class word has one particular flag bit set, and terminates the copy with end-marker bytes.
- `write_byte_to_three_pointers`: Writes the same byte from a data register into three memory locations, one each through three address registers.
- `call_two_routines_then_return`: Calls two other routines in turn and returns.
- `mark_phoneme_run_boundaries`: Clears the low nibble of each entry in one output array, then walks the phoneme list marking run boundaries by setting flag bits on the first entry of each run and on the last entry before the next run.
- `mark_phoneme_run_boundaries_alt`: Alternative entry into the run-boundary marking pass, with the same flag steps as its neighbour but without clearing the low nibbles first.
- `mark_run_boundaries_loop`: Loop variant of the run-boundary marking pass.
- `mark_run_boundaries_tail`: Tail of the run-boundary marking pass.
- `mark_run_boundaries_scan`: Scans forward through the phoneme list from the current entry, setting boundary flags on the entries that begin and end each run.
- `mark_run_boundaries_inner`: Inner loop of the run-boundary pass that marks each run's final entry and then resumes scanning the list.
- `mark_run_boundaries_pair`: Sets a pair of boundary flags on the previous entries, then runs the boundary scan until the end marker.
- `compute_phoneme_frame_periods`: Converts a rate value into a period by dividing a fixed constant by it, then walks the phoneme list and writes per-frame period values, interpolating between each phoneme's neighbouring values.
- `write_phoneme_periods_from_stored`: Writes per-frame period values for each phoneme using the period already stored in the block, interpolating between neighbouring entries.
- `write_interpolated_frame_periods`: Writes per-frame values for each phoneme by dividing the stored period by each frame's table value and interpolating between neighbouring frames.
- `interpolate_period_values_loop`: Loop that interpolates per-frame period values between neighbouring frames, stepping through the phoneme list and writing a value at each boundary.
- `interpolate_period_values_tail`: Tail of the period-interpolation pass.
- `interpolate_period_values_run`: Interpolates period values across a run of frames, writing each boundary value from the stored period divided by a per-frame value, and loops until the end marker.
- `write_constant_period_values`: Divides the stored period by a fixed divisor and writes the result into the first byte of each eight-byte record until the end marker.
- `return_immediately`: This routine is empty and performs no operations before returning.
- `apply_frame_adjustments`: Walks the frame records and adjusts each frame's value by offsets chosen from the class bits of the neighbouring phonemes, skipping special codes.
- `adjust_frame_values_by_class`: Applies class-based offsets to each frame value using the neighbouring phonemes' class bits, then caps the stored value at 63.
- `adjust_frame_values_tail`: Tail of the frame-value adjustment pass that applies the same class offsets to the current frame, caps the stored value at 63, and then steps to the next entry.
- `adjust_frame_values_loop`: Loop variant of the frame-value adjustment pass.
- `adjust_frame_values_neighbour_pass`: Frame-value adjustment pass that chooses an offset from the class bits of the phonemes either side, applies it, and caps the stored value at 63.
- `adjust_frame_values_neighbour_step`: Single neighbour-dependent adjustment step for the frame values.
- `adjust_frame_values_neighbour_loop`: Loop variant of the neighbour-dependent adjustment pass.
- `adjust_frame_values_neighbour_tail`: Tail of the neighbour-dependent adjustment pass.
- `adjust_frame_values_neighbour_scan`: Scans forward through the frame list applying neighbour-dependent offsets and the 63 cap to each stored value.
- `adjust_frame_values_pair_flags`: Compares flag bits of the neighbouring phonemes before adjusting the current frame value, caps the result at 63, and then moves to the next entry.
- `adjust_frame_values_final_pass`: Final frame-value adjustment loop.
- `adjust_frame_values_class_loop`: Frame-value adjustment pass that applies offsets from neighbouring class bits, caps the stored values at 63, and loops until the end marker.
- `scale_product_with_rounding`: Multiplies the low word of one register by a word from another, then returns the high word of the product together with the low word after adding sixteen and shifting right by five.
- `initialise_block_pointers_and_clear`: Stores a set of pointer offsets into the block structure from its base, sets several word fields to zero, and clears six blocks of thirty-two longwords each.
- `advance_frame_counter_and_call`: Increments a frame counter word in the block and always calls one routine.
- `collect_marked_frame_entries`: Walks the frame list and writes a compact value for each phoneme frame, marking entries that carry class flags.
- `collect_marked_frame_entries_loop`: Loop variant of the frame-entry collection pass.
- `collect_marked_frame_entries_alt`: Alternative frame-entry collection pass with the same compact values, class marking and 128-entry limit.
- `mark_last_frame_entry`: Sets a boundary flag on the last written frame entry, then records that entry's position and the first marked position in the block.
- `assign_frame_entry_flags`: Walks the marked frame entries from the end and assigns flag values according to their class codes.
- `propagate_frame_entry_flags`: Propagates the assigned frame entry flags back through the preceding entries, copying each value to the nearest slot that carries a particular class bit.
- `propagate_frame_entry_flags_loop`: Loop variant of the flag propagation pass.
- `propagate_frame_entry_flags_tail`: Tail of the flag propagation pass.
- `propagate_frame_flag_values_back`: Walks the marked frame entries backwards, copying a flag value into the nearest qualifying slot and clearing the source slot when the two positions differ.
- `copy_flag_to_nearest_slot`: Searches outwards from the current frame entry for the nearest slot with a particular flag bit and copies the flag value there, clearing the source entry when it moves.
- `swap_and_clear_byte_flags`: Copies a flag byte from a source slot to a destination slot in a table, clearing the source when the slots differ, then walks backwards through the marked entries applying the same copy until the start of the list.
- `set_low_flag_across_range`: Sets the lowest bit of each byte in a block, counting down from a size passed in a word register.
- `set_bit0_over_index_range`: Sets the lowest bit of every byte in a buffer, walking the index downward from a starting offset to zero.
- `or_control_mask_until_flag`: Picks a bit mask from a control byte (4 for a value of 1, 8 for a value of 2, otherwise 0) and ORs it into a buffer walking backwards from a start index.
- `set_tile_mask_and_marker`: When the control byte is neither 1 nor 2, ORs a mask of 48 or 176 into one buffer cell, chosen by whether a neighbouring type byte equals 8.
- `add_word_to_table_entries`: Reads a word from a structure, and when a field is zero runs a fixed sequence of helper calls (four extra when a second flag is set).
- `compute_and_store_clamped_level`: Computes a level from a per-step index and a divided counter, clamps it into the range 125 to 165, and stores it as a byte in a table at the position given by the caller's index.
- `clamp_value_and_store_char`: Clamps an input word into the range 125 to 165 and stores the low byte at a base pointer plus an index.
- `adjust_flagged_entries_by_gaps`: Computes a divided step from a word and a per-entry offset, then adds it to a range of table bytes.
- `adjust_table_values_loop`: Repeatedly adds a scaled absolute difference between two table entries to a destination byte, scanning backwards with flag bits to find matching entries.
- `compute_scaled_output_tables`: For each flagged index walking downward, derives two signed scaled values from a type nibble and the entry, and writes them to two output tables with a clamp at zero on one.
- `scale_flagged_entries_backwards`: Repeatedly scales flagged entries from a start index downward by a signed multiplier and a type nibble, writing results into two output tables.
- `compute_scaled_char_values`: Loops over a range of indices, using flag bits to select entries and computing scaled character values from two tables with multiplications and shifts.
- `smooth_adjacent_flagged_entries`: Walks flagged entries backwards, scaling and adjusting pairs of neighbouring values according to the distance between them.
- `smooth_flagged_pair_range`: Scales and adjusts pairs of flagged neighbouring entries over a descending index range, choosing multipliers from the pair distance.
- `scale_flagged_pairs_with_outputs`: Adjusts neighbouring table entries in pairs with scaled residuals, writing the results into two output tables.
- `scale_adjacent_pairs_ordered_outputs`: Updates neighbouring entry pairs by scaled residuals and writes the derived values into two tables.
- `apply_pair_scaling_with_outputs`: Applies scaled residual updates to neighbouring pairs in a table and stores them to output tables.
- `update_pair_residuals_and_maxima`: Adjusts a pair of neighbouring table entries by a scaled residual, then in a final pass writes output bytes for flagged entries.
- `offset_entry_scale_flagged_pairs`: Adds a constant from a register to one table entry, then performs the same flagged pair scaling and final output pass as the neighbouring routines.
- `scale_pair_by_control_step`: Scales a neighbouring pair of entries by a multiplier chosen from a control value (19, 32 or 38 for the first entry, 13 or 19 for the second), then updates output bytes and runs the flagged output pass.
- `scale_pair_by_stack_multiplier`: Scales a neighbouring pair of table entries by a multiplier taken from a stack argument, then updates output bytes and runs the flagged output pass.
- `scale_pair_by_signed_step`: Scales a neighbouring pair of table entries by a signed multiplier chosen from a stack control value, then runs the flagged output pass that sets output bytes from a running maximum or a constant offset depending on type bits.
- `scale_second_entry_output_pass`: Scales the second entry of a neighbouring pair by a control-dependent multiplier, adjusts a flagged entry, and then runs the flagged output pass with running maximum and constant offset writes.
- `scale_pairs_negative_residual`: Adjusts neighbouring pairs of table entries with negated scaled residuals and writes results to an output table.
- `write_running_maximum_and_offsets`: Computes a running maximum over the table bytes below an index and writes a scaled maximum to an output table.
- `update_running_maxima_with_residuals`: Computes a running maximum over earlier table bytes and writes a scaled result to an output table.
- `scale_residuals_from_running_maximum`: Tracks a running maximum over a byte range and writes scaled results from it into an output table.
- `scale_flagged_bytes_by_sign`: Walks an index downward over a range and, for each byte whose high nibble is nonzero, multiplies a second table entry by 38 or by minus 102 depending on the top bit of the flag byte.
- `scale_table_bytes_by_nibble`: For each index in a range whose flag byte has a nonzero high nibble, scales the table byte by 38 or minus 102 chosen from the flag's top bit.
- `scale_buffer_by_signed_factor`: Walks a byte range backwards, skipping bytes whose high nibble is zero, and adds a shifted accumulator to a destination byte.
- `distribute_pair_residuals_across_range`: Initialises per-entry differences from three tables, then walks pairs of flagged indices and splits each residual between neighbours in halves.
- `copy_range_and_offset_entries`: Copies bytes between two tables across a range while subtracting a shared step, and stores each result offset by 5 in a third table.
- `fill_runs_from_control_index`: Walks a range of flagged entries from a control index, copying values between tables and doubling a low nibble into an output table.
- `copy_bytes_double_nibble_runs`: Copies bytes between two tables for each position in a run, storing a value offset by 5 and computed from a low nibble doubled.
- `split_residuals_over_flagged_runs`: Splits a residual between neighbouring flagged entries and fills the entries in the run with values offset by 5 when a nibble flag is set.
- `split_residuals_and_fill_runs`: Splits residuals between neighbouring flagged entries and fills runs with values offset by 5 when a nibble flag is set.
- `fill_run_with_stepped_values`: Sets three tables to the constant 110 across a range and adds a doubled nibble to one table.
- `fill_tables_split_run_residuals`: Fills three tables with the constant 110 plus a doubled nibble across a range, then finds the start of a flagged run and splits its residual evenly over the run.
- `split_run_residual_set_end`: Finds the start of a flagged run walking backwards, divides its residual across the run, and fills the entries with values offset by 5.
- `step_copy_run_set_end`: Copies a run of bytes between two tables while subtracting a fixed step, storing each result offset by 5 in a third table.
- `write_end_value_from_maximum`: Computes a running maximum over the table bytes below a range end and writes a scaled value there, offset by 5 in a second table.
- `accumulate_bytes_and_write_results`: Accumulates a running maximum over a range of bytes, derives a scaled value from it, and writes that value and a derived offset into several buffers.
- `apply_flag_scaled_offsets_backwards`: Walks flagged entries backwards and, where a flag bit in a lookup word is set, applies scaled offsets to three tables using a multiplier of 26 or 102.
- `apply_flag_offsets_run_search`: Walks backwards over flagged entries using a lookup word from another buffer, applying scaled offsets to three tables.
- `scale_entry_then_search_flags`: Applies a scaled offset to one table entry and writes a derived value to two tables.
- `scale_pair_flag_search_loop`: Applies scaled offsets to a pair of table entries and then loops over the remaining positions.
- `subtract_residual_with_flag_search`: Adds a rounded scaled residual to one table and subtracts it from another across a range.
- `apply_run_scale_flag_walk`: Scales a run of entries from a constant starting value and writes the result to two tables.
- `scale_run_by_constant_86`: Scales a run of table entries by the constant 86 and writes the result to a second table.
- `scale_run_by_constant_26`: Scales a run of table entries by the constant 26 and writes the result to a second table.
- `adjust_flagged_attribute_buffers`: Walks a byte array backwards and updates several attribute buffers, applying signed scaling and sign-bit adjustments.
- `copy_bytes_with_feedback_loop`: Copies bytes between two buffers while updating a feedback value from a table entry shifted right by seven.
- `copy_adjust_bytes_with_feedback`: Copies bytes between buffers, updating a feedback value by multiplying it with a table entry and shifting it.
- `check_narrator_flag_and_dispatch`: Calls a fixed sequence of twelve helper routines, then calls one more when a flag byte in the data area is nonzero.
- `call_routine_three_times`: Calls a shared helper routine three times in sequence and returns.
- `interpolate_and_store_bytes`: Interpolates linearly between two bytes read from a buffer across a number of steps, and stores each interpolated byte at intervals of eight bytes.
- `skip_marked_records_until_end`: Walks a buffer in records of eight bytes, skipping entries equal to minus 2 and calling a helper for records that are not on an eight-byte boundary.
- `advance_records_while_flagged`: Steps through eight-byte records, calling a helper to move past the current record when it is not on an eight-byte boundary.
- `scan_records_after_helper_call`: Calls a helper routine, then scans eight-byte records until it finds a terminating minus 1 entry.
- `scan_zero_records_minus_one`: Scans eight-byte records until one with a nonzero entry is found, then stops at a minus 1 terminator or steps over the record.
- `advance_records_until_sentinel`: Advances a pointer through eight-byte records, calling a helper when the record is not on an eight-byte boundary.
- `scan_records_past_zero_run`: Calls a helper, then scans eight-byte records for the first nonzero entry, stepping past records that are all zero.
- `write_table_patterns_per_code`: Walks paired byte streams and, for each code with a flag bit set, writes five-row patterns from a global table into the output.
- `write_patterns_for_code_stream`: Walks two byte streams and writes five-row patterns for flagged codes into the output buffer.
- `emit_patterns_until_terminator`: Loops over a code stream until a minus 1 terminator, writing five-row patterns for flagged codes and advancing the output by a stride for others.
- `emit_code_patterns_then_scan`: Writes five-row patterns for a fixed code count, then scans the remaining code stream, advancing or writing output depending on flags.
- `emit_pattern_for_code_pairs`: Writes five-row patterns for the codes plus and comma, or a default pattern for other codes, then continues scanning the code stream and advancing the output by flags.
- `emit_pattern_code_pair_two`: Writes a five-row pattern for the comma code or a default pattern for other codes, then scans the code stream and advances the output based on flags.
- `emit_pattern_for_default_code`: Writes a five-row default pattern for each code, then scans the code stream and advances or splits the output by flags until the terminator.
- `copy_previous_or_split_nibbles`: Walks a code table, copying the previous record for codes 1 or 2 and halving low nibbles in several bytes for other flagged codes.
- `split_nibbles_per_code_flags`: Walks a code table, copying the previous record for codes 1 or 2 and splitting low nibbles between neighbouring bytes for other flagged codes.
- `split_nibbles_with_mask_parameters`: Walks a code table using a mask and a count from the caller, splitting low nibbles between neighbouring bytes and copying the previous record for codes 1 or 2.
- `split_nibbles_across_record_stream`: Walks a code stream until the terminator, copying previous records for codes 1 or 2 and splitting nibble values for flagged codes.
- `split_nibbles_until_end_code`: Walks a code stream until the end code, copying the previous record for codes 1 or 2 and splitting nibbles for flagged codes.
- `remap_records_through_lookup_table`: Walks eight-byte records from a fixed position, replacing the first three bytes of each record with values from a global lookup table.
- `copy_bytes_through_index_table`: Copies bytes from a source buffer selected by an index table into a record stride, looping until an end code appears in the record stream.
- `nop_routine`: The routine is empty and performs no operations.
- `sum_records_into_checksum_byte`: For each eight-byte record in a table, adds seven bytes read at the record stride, divides the total by eight, and stores the result at offset 24 of the record.
- `accumulate_weighted_running_sum`: Computes a weighted running sum over record bytes at the eight-byte stride, writing the averaged value into offset 24 of each record.
- `compute_weighted_record_checksum`: For each record, sums bytes at the eight-byte stride with a factor of two, subtracts a base byte and an extra byte, and stores a scaled result at offset 24.
- `compute_running_checksum_per_record`: Accumulates a weighted running checksum across record bytes at the eight-byte stride, storing a scaled value at offset 24 of each record.
- `apply_code_offsets_to_records`: Walks paired code and count streams and adds a per-code offset to a byte at offset seven of each eight-byte record.
- `apply_code_offsets_from_stream`: Reads code and count pairs from a stream and adds offsets to a byte at offset seven of each eight-byte record.
- `add_code_stream_run_offsets`: Writes a run of offsets for each code in a stream, adding 10 for the first code type and minus 26 for another.
- `add_negative_six_run_offsets`: Adds a fixed offset of minus 6 to a run of records at offset seven, using the code stream to decide how far to advance.
- `add_offset_runs_for_codes`: Adds offset runs to records for each code in the stream, using a fixed offset of minus 6 for some codes and the code-specific offset for others.
- `add_offset_runs_then_skip`: Adds offset runs to records for each code in the stream, skipping records with no change and returning at a 255 code.
- `fill_offset_runs_for_codes`: Writes minus 26 into offset seven of each record in a run for one code, and adds a code-derived offset for others.
- `apply_offset_runs_previous_byte`: Adds offsets computed from the previous record's byte to a run of records, using the code stream to choose between runs and single steps.
- `decode_nibble_checksum_bytes`: Computes a checksum from the low nibbles of groups of bytes, then from their high nibbles, and writes each result into the third byte of its group with the matching nibble masked.
- `init_signal_and_wait_loop`: Allocates a signal bit through the Exec library, stores it in the structure with a computed signal mask, and loops calling a helper and waiting on the signal.
- `fetch_messages_until_flag_set`: Loops calling an Exec library function to fetch the next message, storing the message pointer when it is not of type 2 and calling a helper.
- `clear_request_flag_bits`: Clears bits 1 and 0 of a flag byte at offset 34 of the request structure.
- `setup_speech_io_request_send`: Links buffer pointers for seven channels, fills fields of a speech I/O request block from the device data, derives a timing value by dividing a clock constant by a period word, and sends the request with the Exec DoIO call.
- `init_audio_channels_and_dma`: Fills per-channel audio buffer pointers and clears channel state, computes a sample period from a clock constant, and enables audio DMA channels by writing the bits to the custom chip DMA control register.
- `mix_wave_tables_into_samples`: Generates audio samples by summing three lookup-table values per step and writes each sample to two output buffers.
- `render_audio_block_with_pointers`: Advances phase accumulators for the audio channels and writes samples for each channel into the output buffers.
- `mix_wave_tables_with_envelope`: Mixes three lookup-table values per sample with an envelope and writes the result to two output buffers.
- `render_bouncing_sweep_samples`: Generates audio samples while a sweep value moves by a step and reverses direction at two bounds, combining table lookups for each sample.
- `sweep_samples_and_disable_dma`: Generates audio samples while a sweep value bounces between two bounds, then reads channel data and writes the DMA control value to disable the channels when the stream ends.
- `render_samples_with_wave_entries`: Renders audio samples by summing table lookups for each step, writing the sum to two output buffers and advancing a phase accumulator.
- `wait_io_disable_audio_dma`: Calls an Exec library WaitIO routine for each audio channel in a loop, and when a stop flag is set it writes the DMA control value that disables the channels before and after the loop.
- `enable_dma_if_flag_set`: Tests a flag word in a structure and, when it is set, writes the DMA control value that enables the audio channels.
- `update_request_counter_and_relink`: Increments a request counter and, when a stored byte has changed, moves a list node and updates a request field before calling a library routine.
- `decrement_counter_and_enable_interrupts`: Reads a counter byte from a structure, decrements it and writes it back.
- `flag_channel_records_and_wait`: Sets a flag bit on each channel record in a block and calls a helper for each one.
- `flag_records_wait_for_io`: Sets a flag bit on a block of channel records and calls a helper for each.
- `wait_for_channel_io_loop`: Enables DMA channels when a flag is set, then loops calling a helper and an Exec library routine until a completion byte becomes nonzero.
- `set_channel_done_flag`: Sets bit 5 of a flag byte at offset 30 of the structure to mark the channel done, then returns.

### MESSAGE

- `call_library_entry`: Calls a helper routine, then when the zero flag is clear calls a second helper and a function pointer taken from a global table.
- `open_dos_library`: Calls the Exec OldOpenLibrary routine with the dos.library name and stores the returned library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:KRAYZI

- `unpack_bitstream_into_buffer`: Copies a block of bytes from a source into a working buffer, then unpacks a bit-coded stream into bytes with shift-and-carry steps and helper calls.
- `load_bitstream_word_store_byte`: Reads a 32-bit word from a fixed offset before the stream pointer, stores its low byte through a pointer, and returns the word shifted right by one with its top bit set.
- `consume_bits_from_stream`: Shifts a bit buffer right for a counted number of steps, reloading it through a helper whenever it becomes zero.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_pointer`: Copies sixteen longwords from a fixed offset in a structure into the buffer passed in, when one is given, then calls the routine stored at the pointer register.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:Intro

- `call_single_routine_at_entry`: Program entry point that calls one other routine and references the text IMP!. The decompiled body is unavailable, so only its call structure and that text reference are described.
- `copy_block_and_fill_table`: Copies 4648 bytes from the source pointer to a fixed destination, then fills a 4096-entry table with descending values and returns the value 65535.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `unpack_backward_bitstream_copy`: Copies bytes backwards within a buffer under control of a bit stream, shifting bits from a source byte and copying runs whose lengths come from the stream.
- `decode_variable_length_bit_codes`: Reads variable-length codes from a bit stream by repeatedly shifting the bit buffer and counting carries.
- `decode_bit_stream_length_pairs`: Decodes length and offset pairs from a bit stream by shifting the bit buffer and counting carries, then combines a table value with the bit counts.
- `decode_bitstream_table_lookup_run`: Decodes a run from a bit stream by shifting the bit buffer, looking up values in a global table and repeating the shift count.
- `decode_bitstream_literal_copy_loop`: Decodes a bit-coded count from the stream by shifting bits from a source byte with carry steps, then copies bytes backwards for that many positions.
- `decode_bitstream_short_runs`: Decodes short runs from a bit stream using shift-and-carry steps and a global table lookup, then copies the decoded bytes backwards within the output buffer.
- `decode_bitstream_offset_runs`: Decodes an offset and a run length from a bit stream, looking up base values in a global table, and copies the run backwards from the output position.
- `copy_bitstream_run_with_offsets`: Decodes a run length and a table-based offset from a bit stream, then copies that many bytes backwards from the output pointer.
- `copy_run_from_bitstream_offset`: Decodes a bit-coded run and offset, copies the run backwards through the output buffer, and returns minus 1 when the output pointer has passed the buffer start, otherwise 0.

### TRANSLATOR.LIBRARY

- `create_library_node_or_fail`: As library code, calls an Exec routine to allocate a node, and returns 0 when the allocation fails.
- `increment_count_and_clear_flag`: As library code, increments a short counter at offset 32 of the library node and clears bit 5 of the flag byte at offset 14.
- `decrement_count_dispatch_if_flagged`: As library code, decrements the short counter at offset 32 of the library node.
- `unlink_node_or_mark_pending`: As library code, unlinks a node from its list when its counter at offset 8 is zero and calls a library routine.
- `unlink_node_and_call_library`: As library code, unlinks a node from its list by relinking its neighbours, then calls a library routine at a fixed negative offset and returns its result.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `scan_text_fields_into_buffer`: As library code, fills a space-padded buffer with spaces, then scans an input string for the hash marker and space separators, storing parsed values and calling helpers.
- `return_low_word_register`: As library code, returns the low 16 bits of the data register as an integer.
- `clear_terminator_and_return_word`: As library code, calls a helper, writes a zero byte at the position given by the address register, and returns the word from the data register.
- `scan_text_until_delimiter_sequence`: As library code, loops over an input string checking for the hash marker and space delimiters, calling helpers to read fields and writing a zero terminator at the end of a field.
- `scan_text_fields_hash_marker`: As library code, scans an input string for field boundaries, looking for the hash marker and space separators.
- `split_on_backslash_and_backtick`: As library code, walks a string and stops at backslash or backtick separators, calling helper routines to process each segment and returning when the segment list ends.
- `skip_to_backslash_or_backtick`: As library code, advances through a string until it reaches a backslash or backtick separator, then calls helper routines and returns when the end condition is met.
- `stub_immediate_return`: As library code, the routine contains only a return, so it does no visible work in the library.
- `compare_bracketed_text_prefix`: As library code, skips characters up to an opening bracket in one string, then compares a second string byte by byte against the text until a closing bracket is found.
- `scan_buffer_backwards_by_flags`: As library code, scans a byte buffer backwards from a start index, looking up each byte in a flag table and calling a helper when the flag is set.
- `return_immediately_stub`: As library code, returns immediately without any memory access or helper call.
- `scan_back_matching_table_bytes`: As library code, loops calling a helper, then walks backwards through a buffer while bytes match the previous entries and a flag table marks them as significant.
- `match_text_until_equals`: As library code, compares a text pattern byte by byte against a buffer, stopping at an equals sign or at the first mismatch.
- `match_pattern_bytes_until_equals`: As library code, matches a pattern against input bytes and stops at an equals sign.
- `copy_text_slice_with_terminator`: As library code, copies bytes from the input until a backslash or backtick separator, then writes a zero terminator.
- `copy_counted_text_run`: As library code, copies a counted run of bytes from the input and terminates the output at the end.
- `copy_counted_bytes_flag_check`: As library code, copies a counted run of bytes into the output buffer.
- `terminate_text_before_flagged_byte`: As library code, walks backwards from the current position until a byte with a flag bit set is found, then writes a zero terminator at that position.
- `dispatch_by_character_match`: As library code, searches a table of twelve punctuation characters for the byte in the data register, then calls the routine from a jump table at the matching index.
- `call_helper_routine`: As library code, the routine calls another routine.
- `stub_empty_return_slot`: As library code, the routine contains only a return in the decompiled output, so it is an empty stub with no visible effect on the library.
- `forward_to_helper_routine`: As library code, calls one helper routine and returns.
- `loop_helper_while_flag_set`: As library code, calls a helper routine once and then keeps calling it while bit 5 of a control word is set.
- `forward_to_helper_once`: As library code, calls one helper routine and returns.
- `check_command_letter_flag`: As library code, calls a helper, then when bit 7 of a control word is set it reads a character and checks it against the letters I and E before returning.
- `stub_return_no_effect`: As library code, returns immediately.
- `match_command_keyword_letters`: As library code, matches a sequence of command letters E, S, D, R, L, Y, N or G against the input and calls helpers for the matched keyword.
- `match_ng_suffix_letters`: As library code, checks the next two input letters for N and G and then a final S letter, calling helper routines and returning when a check fails.
- `return_without_effect_b`: As library code, returns immediately with no memory access or helper calls.
- `call_handler_routine`: As library code, the routine calls another routine through a pointer.
- `call_helper_routine_2`: This library code calls another routine.
- `execute_indirect_call`: This routine performs an indirect call to another routine.
- `execute_callback_routine`: This library routine calls another routine via a pointer retrieved from a data structure.
- `loop_helper_while_bit_five`: As library code, calls a helper routine repeatedly while bit 5 of a control word remains set, then returns.
- `execute_callback_routine_2`: This routine calls another routine via a pointer retrieved from data.
- `loop_helper_while_bit_one`: As library code, calls a helper routine repeatedly while bit 1 of a control word remains set, then returns.
- `call_helper_check_bit_seven`: As library code, calls a helper routine and tests bit 7 of a control word.
- `return_no_effect_text`: As library code, returns immediately without any memory access or helper calls.
- `normalize_text_into_field`: As library code, copies up to 100 input characters into a fixed field buffer.
- `write_space_before_pointer`: As library code, writes a space character two bytes before the address register.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d1, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 32 to (a2)+, then copies 32 to (a2)+, then returns.
- `substitute_characters_from_table`: As library code, looks up input characters in a fixed 14-entry table and, on a match, shifts the output buffer and inserts a marker byte.
- `shift_buffer_and_insert_marker`: As library code, shifts the bytes of a buffer up by one position, writes a fixed marker byte at the freed slot, and advances the stored output pointer.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:PAUSE

- `wait_while_flag_bit_six`: Spins in a loop while bit 6 of a global value stays set, then returns.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:delphine

- `load_hunk_table_and_relocate`: Program entry for the game loader section: allocates memory for each hunk through Exec AllocMem, then walks the hunk table and handles the hunk type codes for code, data and relocation records.
- `stub_empty_return_routine`: The decompiled body is empty apart from a return, so this routine has no visible behaviour.
- `spin_down_delay_counter`: Decrements a counter held in a data register until it reaches zero, giving a fixed delay.
- `tiny_ori`: Small routine of 1 instruction that merges 80 into d0.
- `expand_run_length_stream_backwards`: Expands a run-length encoded stream backwards into a buffer, reading control bytes and copying literal or repeated runs until the output pointer reaches the start.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_across_count`: Decrements a byte held in a data register once per step across a 16-bit count, wrapping it to a space character when it reaches zero.
- `stub_bad_instruction_data`: The decoded body is bad instruction data; the decompiler truncated control flow here, so there is no behaviour to describe beyond this stub.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:add21k

- `initialise_block_and_call_library`: Clears a block of 23 longwords, stores a system register value and a global pointer, then calls a library routine.
- `open_system_libraries_and_setup`: Opens the system libraries named in its text, raising an alert if an open fails.
- `empty_return_stub`: Returns immediately with no effect.
- `allocate_tagged_file_request`: Allocates a file request block, stamps it with a four-character tag, and opens the named file through DOS library wrappers, recording the handle and buffer.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_argument_string_into_block`: Copies a name into a newly allocated block and then splits the following text into arguments at whitespace.
- `append_string_to_end`: Appends a null-terminated source string to the end of a destination string.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches minus 1.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `open_window_and_icon_libraries`: Looks up the WINDOW name and the icon.library through library calls, then reads fields from the returned structures to set window and icon pointers in the program block.
- `call_optional_hook_and_helper`: Calls a hook routine stored in the program block when one is set, then calls a helper routine and returns.
- `call_optional_hook_then_return`: Calls a hook routine stored in the program block when one is set, then calls a helper routine and returns.
- `release_resources_and_restore`: Releases program resources in order: it loops over a count of slots calling a helper, runs an optional hook, and calls release helpers for each non-null resource field.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_slot_by_index`: Checks that a slot index is in range and nonzero, closes the file in that slot through a helper when it is not flagged, and clears the slot.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_handler_pointer`: Calls a routine through a function pointer stored in the program block at a fixed offset.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `invoke_library_vector_414`: Loads a library base from the program block and calls the library routine at offset minus 414 of that base.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `jump_library_vector_552`: Loads a library base from the program block and jumps to the library routine at offset minus 552 of that base.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `jump_library_vector_384`: Loads a library base from the program block and jumps to the library routine at offset minus 384 of that base.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_stub`: Returns the value minus one immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_recursive_calls`: Checks if a value in a table is not negative, then calls another routine recursively using a pointer from that table and updates the table entry to a maximum value.
- `execute_recursive_calls_2`: Performs a recursive call using a value from a table and subsequently updates that table entry to a maximum value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_ciaa_register`: The routine reads from the CIAA port A data register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_call_routines`: Clears a range of memory, updates a global variable, and calls a routine or a pointer-based routine depending on certain flag checks.
- `check_and_call_routine`: Checks a global variable and calls a routine if it is non-zero, then potentially calls another routine and updates a global variable with a return value.
- `traverse_and_call_pointers`: Checks a memory location and calls a routine if a certain value is non-zero, then recursively calls itself and calls other routines based on pointer checks.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `update_memory_from_pointer`: Calls another routine and copies a value from a pointer-based address into a destination, while zeroing out other memory locations.
- `execute_conditional_logic_chain`: Performs a series of conditional checks on a global pointer, calling various subroutines, and updating memory or configuration tables based on certain values.
- `pass_through_routine`: This routine calls another routine with the same arguments and then returns.
- `read_input_flags`: The routine returns a byte derived from a global variable, testing bits 64 and 4 and combining the results with a logical OR. It references the CIAA_PRA and POTGOR hardware registers.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `set_global_value`: Sets a global variable to a specific constant value.
- `update_and_scale_value`: Checks a value, multiplies a global variable by two, applies an XOR mask if a carry or zero condition occurs, and returns a value involving a division and remainder calculation.
- `handle_keyboard_serial`: Returns a byte computed from the bitwise complement of a global variable when a flag bit is set, performs a decrementing delay loop, and clears a flag bit in the global variable.
- `populate_table_with_offsets`: Copies a block of data from a source table and populates a destination table with a series of calculated values or offsets derived from input parameters.
- `write_config_table`: Stores a series of hardcoded word values into a table located via a pointer in a global variable.
- `write_config_table_2`: Stores hardcoded word and long values into a table located via a pointer in a global variable.
- `load_pointer_from_table`: Returns a value loaded from a memory address calculated from a global variable.
- `update_table_with_raster_sync`: Updates a destination table with new values and potentially updates a global pointer, while calling a routine that waits for a specific raster line.
- `copy_source_to_dest`: Copies a sequence of words from a source buffer to a destination buffer located in a table.
- `return_empty_routine`: The routine is empty and performs no operations before returning.
- `empty_routine_2`: the routine contains no executable instructions and performs no operations.
- `update_display_raster_parameters`: Updates a table with new offset and bitmask values by calling a routine that waits for a specific raster line, then performs a word-based memory copy from a source buffer to a destination buffer.
- `execute_nested_calls`: Performs a sequence of calls to another routine using values passed or stored on the stack.
- `get_and_increment_index`: Reads a byte from a table using an index, then increments that index stored in a pointer.
- `initialize_and_fill_table`: Clears a block of memory, then iterates through a table to clear or update entries with values retrieved from a recursive call or another routine.
- `traverse_linked_list_2`: Recursively traverses a linked list and returns a value from a pointer-based memory location or a table.
- `traverse_linked_list`: Recursively traverses a linked list and returns a value from a table or a pointer-based memory location.
- `traverse_linked_list_extended`: Performs a recursive traversal of a linked list and retrieves a short value from a table offset.
- `traverse_list_and_read_flags`: Performs a recursive traversal of a linked list and retrieves a byte derived from a global variable while testing certain bits and combining results with a logical OR.
- `read_input_flags_2`: Returns a byte derived from a global variable while testing certain bits and combining results with a logical OR.
- `initialize_game_state_tables`: Initializes a series of global pointers and clears or populates memory-resident tables, including a loop that zeros out a large block of memory.
- `reset_memory_values`: Sets a 4-byte memory location to zero and a 2-byte memory location to all ones.
- `traverse_linked_list_3`: Iterates through a linked list, calling a routine for each node, and then sets a 4-byte memory location to zero and a 2-byte memory location to all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_linked_list`: Recursively calls itself using a pointer or value from a table and updates a global variable before setting a certain memory location to a maximum value.
- `populate_buffer_and_verify`: Populates a buffer with bytes from a function call and then verifies a range of memory using a secondary routine.
- `search_table_for_matching_value`: Iterates through a table to find an entry where a specific part of a value matches, then recursively checks a sub-range or linked structure.
- `search_and_recurse_table`: Iterates through a table of structures to find a matching key, then recursively calls itself or another routine on a calculated range of values if a match is found.
- `recursively_check_table`: Checks a value in a table and calls another routine recursively or calculates an offset into a table based on a returned value.
- `execute_recursive_operations`: Calls another routine with a fixed value, and if successful, performs several recursive calls involving calculated or passed parameters.
- `decompress_bitstream_to_memory`: Reads a stream of bits and writes the resulting bytes or bit-manipulated values into a memory range.
- `extract_bit_stream`: Extracts bits from a routine call and stores a byte or a modified value into a memory location pointed to by a pointer.
- `unpack_bitstream_to_memory`: Reads a stream of bits and writes the resulting bytes or bit-manipulated values into a memory range.
- `fill_memory_and_read_bits`: Fills a range of memory with a value using a decrementing loop and reads a certain number of bits from a bit buffer.
- `fill_memory_with_value`: Fills a range of memory with a value derived from a register or stack value using a decrementing loop.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `return_empty_stack_value`: The routine is empty and returns an uninitialized value from the stack.
- `process_table_entries`: Iterates through a table, calling a routine with values from the table, and clears the table entries or sets them to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `initialize_string_and_loop`: Checks a parameter, updates a global variable, and executes a series of routine calls and a loop that terminates based on a return value.
- `infinite_loop_with_function_call`: Executes a loop that calls itself and another routine repeatedly without a termination condition.
- `update_graphics_parameters`: Performs several routine calls and calls a function with calculated offsets and a fixed value, likely intended for a display or certain hardware parameters.
- `calculate_text_window_bounds`: Calculates the maximum width of a string or string segment delimited by a special character and determines the horizontal centering offset for a text bounding box.
- `parse_string_and_set_window`: Iterates through a null-terminated string to calculate string length and update a global counter, then calculates a window or box size based on a linear scaling or clamping logic.
- `calculate_string_render_bounds_2`: Calculates a centered text bounding box or window, adjusting or clamping a width or height value, and then iterates through a string to detect or process characters.
- `calculate_string_render_bounds`: Calculates a centered text bounding box or window, adjusting or clamping a width or height value, and then iterates through a string to determine character spacing and draw character tiles or empty spaces.
- `render_string_with_padding_logic_2`: Iterates through a character string to calculate display offsets and write character tiles or null-terminators to a buffer.
- `render_string_with_padding_logic`: Iterates through a character string to calculate display offsets and write character tiles or null-terminators to a buffer.
- `render_character_tiles`: Iterates through a character sequence, calculating display offsets and writing characters or null-terminators to a buffer.
- `render_string_with_padding`: Iterates through a character sequence, calculating display offsets and writing characters or null-terminators to a buffer.
- `format_string_display_buffer_2`: Iterates through a sequence of characters, updating display offsets and writing character or null-terminator bytes to a buffer.
- `format_string_display_buffer`: Iterates through a sequence of characters, updating display offsets and writing character or null-terminator bytes to a buffer.
- `update_string_display_buffer`: Iterates through a sequence of characters, calculating or updating display offsets and writing character or null-terminator bytes to a buffer.
- `update_position_and_buffer_2`: Calculates a difference between a table value and a global variable, updates a global offset, and copies a sequence of 16 words from a source.
- `update_position_and_buffer`: Calculates a difference between a table value and a global variable, updates a global offset, and copies a sequence of 16 words from a source table to a destination buffer.
- `copy_cstring`: Copies a null-terminated byte string from a source pointer to a destination pointer, byte by byte, and terminates the destination with a zero byte.
- `search_string_table`: Iterates through a table of pointers to strings and uses string comparison to find a match for a provided string.
- `find_trackdisk_device`: Calls the exec library to locate the trackdisk.device and extracts a value from a device structure, returning it combined with an input register.
- `search_trackdisk_device`: Locates a trackdisk device and uses a loop to check a return value, potentially incrementing a counter or returning an error code.
- `examine_and_lock_file`: Attempts to examine a file or lock a file, and if successful, retrieves a value from a specific offset within the file and then unlocks it.
- `reinitialize_string_structure`: Resets a string structure's metadata, sets certain flags, and writes a constant string or a new pointer back to a global structure.
- `compare_and_update_string_metadata`: Compares two null-terminated strings and, if they are identical, traverses a linked list of pointers to calculate a count and updates a metadata structure or table with the new count and a list of pointer references.
- `compare_and_populate_string_table`: Compares two null-terminated strings and, if they match, traverses a linked list of pointers to populate a new table with those pointers.
- `execute_indirect_calls`: Performs indirect calls to a target address retrieved from a table using a negative offset and calls a library function via a global base.
- `copy_string`: Copies a null-terminated string from a source pointer to a destination pointer.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `compare_two_byte_strings`: Compares two byte strings byte by byte and returns a value indicating if the first is lexicographically smaller, larger, or equal.
- `safe_memory_copy`: Copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `clear_and_update_globals`: Zeroes out a block of memory, updates a table entry with a constant, and copies a global variable to a new memory location.
- `execute_empty_routine`: The routine performs no operations and returns immediately without modifying any state or registers.
- `parse_string_tokens`: Parses a string or a sequence of quoted strings, skipping whitespace and extracting individual tokens into a destination table.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit_3`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches a certain limit, and ensures the destination is null-terminated.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `open_and_store_file_data`: Opens a file and, if successful, retrieves a pointer from a structure to store a value in a table and assign a pointer to a structure member.
- `copy_register_data`: Copies a longword from one data register to another.
- `store_and_parse_string`: Stores a passed parameter into a global variable and then calls a routine to parse a format string.
- `increment_and_store_byte`: Loads a pointer from a global variable, increments that global variable, and stores a byte into the memory location pointed to by the loaded pointer.
- `recurse_with_concatenation`: Recursively calls itself while returning a 64-bit value formed by concatenating a register's previous contents with the new return value.
- `parse_format_string`: Parses a string containing format specifiers and substitutes or appends values from a provided pointer array.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls itself without performing any other operation.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers and returns the result in a concatenated 64-bit value.
- `execute_conditional_pointer`: Checks if a value at a certain offset from a pointer is non-zero and, if so, executes the function pointer located at that memory location.
- `execute_conditional_table_logic`: Checks various memory locations or table-based pointers and executes corresponding functions or jumps if the pointers or certain flags are non-null.
- `empty_routine_3`: the routine is empty and performs no operations.
- `execute_indirect_call_with_bounds_check`: The routine checks if an index is within a certain range and if a pointer or its target is valid, then either calls a routine or updates a global variable and returns a failure value.
- `execute_indirect_call_from_table`: The routine performs an indirect call to a function pointer retrieved from a table.
- `execute_indirect_call_2`: Performs an indirect call to a function pointer retrieved from a table or structure.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_indirect_call_with_offset`: The routine performs an indirect call to a function pointer retrieved from a table using a negative offset.
- `get_current_directory`: Calls a library function to retrieve the current directory.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `get_input_string`: Calls a library function to read input.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function_4`: loads a function pointer from a table offset and executes it.
- `open_file`: Calls a library function to open a file.
- `output_string`: Calls a library function to output data.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `seek_file`: Wraps a library function to seek within a file and returns the result.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `unlock_file`: Calls a library function to unlock a file.
- `write_data`: Wraps a library function to write data and returns the result.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_indirect_function`: Loads a function pointer from a table offset and calls it.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: calculates an offset from a global table, applies a constant subtraction, and calls the resulting function pointer.
- `call_library_function`: Loads a function pointer from a table offset and calls a library function.
- `populate_struct_and_recurse`: Populates a structure or a block of memory with passed parameters and a library-returned value, then recursively calls another routine.
- `copy_and_recurse_pointer_2`: Copies a sequence of bytes from one memory location to another and recursively calls another routine or itself based on the return value and a certain condition.
- `copy_and_recurse_pointer`: Copies bytes from one memory location to another, updates a pointer with a return value, and then calls another routine or itself recursively.
- `increment_pointer_and_recurse`: The routine calls itself with a pointer incremented by a constant value.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `execute_indirect_function_5`: Loads a function pointer from a table using an offset and executes it.
- `call_library_function_alt`: Loads a function pointer from a table offset and calls a library function via an offset.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `dereference_indirect_pointer`: loads a pointer from a global table and calls a routine located at a fixed offset from that pointer.
- `call_indirect_pointer`: Performs an indirect call to a routine located at an offset from a pointer stored in a global variable.
- `execute_indirect_pointer_offset`: Loads a pointer from a global table, applies a fixed offset, and performs an indirect call to the resulting address.
- `call_indirect_pointer_2`: Performs an indirect call to a routine located at a different offset from a pointer stored in a global variable.
- `execute_indirect_function_6`: Loads a function pointer from a table using an offset from a global pointer and executes it.
- `execute_indirect_function`: loads a function pointer from a table using an offset from a global pointer and executes it.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `execute_indirect_function_2`: loads a function pointer from a table using an offset and executes it.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `execute_indirect_function_offset`: Loads a function pointer from a table using an offset and executes it.
- `execute_indirect_function_3`: loads a function pointer from a table using an offset and executes it.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_indirect_pointer_3`: Performs an indirect call to a routine located at a different offset from a pointer stored in a global or register-based pointer.
