# Game 2001 (operation stealth (1990)(u. s. gold)[cr pdx](disk 1 of 3)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02001). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (934720 top-level bytes in 28 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 537 of 537 named (0 author symbols), 537 of 537 described (56 by rule, 135 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1662 of 1662 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9271 of 9272 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 743121 |
| code | 29890 |
| data | 154309 |
| padding | 5352 |
| unused | 2048 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| NARRATOR.DEVICE | 3 | 3782 | 268 |
| Message | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:endcli | 2 | 117 | 2 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:intro | 2 | 174 | 12 |
| TRANSLATOR.LIBRARY | 2 | 490 | 50 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:PAUSE | 2 | 3 | 1 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:delphine | 2 | 207 | 8 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:add21k | 3 | 637 | 38 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
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
- operation stealth (1990)(u. s. gold)[cr pdx](disk 1 of 3)[a]/Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:delphine: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:intro

## Copy protection found

- none found

## Functions


### NARRATOR.DEVICE

- `initialize_device_and_ports`: Allocates a device record, initialises its list headers, allocates a message port with a signal, and registers the task and device through exec library calls.
- `start_or_queue_device_request`: Handles a device command: for command zero it either queues a request by filling its fields with default values and a sequence number, or, when no request is pending, allocates a message through the library and records an error code on failure.
- `open_audio_device_request`: Checks a request counter and, when it is zero, opens a request on the audio device through an exec library call, recording an error code if the open or allocation fails.
- `allocate_request_message`: Records a request type and a data pointer, allocates a message through an exec library call, and records an error code on failure.
- `update_device_state_fields`: Stores an input value into a structure, increments counters in it, copies a counter into a second structure, writes fixed words and bytes there, and clears a flag bit in the first structure.
- `set_request_status_and_clear`: Stores the low byte of the data register in a request status field and sets the request's pointer field to minus one.
- `queue_pending_request`: Checks a device flag and whether the request's value matches the pending entry.
- `link_request_into_queue`: Compares a request with the pending entry and either updates its nibble fields from the pending data or links it into a queue.
- `update_or_link_request`: Copies the pending value and nibble fields into the request when they differ, otherwise links the request into a queue and adjusts a counter.
- `copy_byte_and_split_nibbles`: Copies a byte from one structure to another, stores its low and high nibbles in two bytes of the destination, clears a status byte, and calls another routine.
- `validate_and_split_speech_text`: Walks the pending request list, checks the request's rate, pitch, flag and volume fields against fixed ranges, and copies valid values into the device state.
- `validate_narrator_request_fields`: Checks a request's rate, pitch, flag, frequency and volume fields against fixed ranges and copies valid values into the device state; out-of-range values produce an error status.
- `finish_matching_requests`: Visits pending entries and calls a routine for each whose tag matches the caller's.
- `decrement_counter_and_close`: Decrements a counter in a structure and, when it reaches zero, calls the library's free-memory and close-device routines through the library base, and optionally calls another routine.
- `dispatch_request_by_index`: Calls one of eight handlers from a table, selected by an index in the request; an index outside that range records an error status.
- `unlink_and_free_request`: If a counter field in the request is zero, unlinks the request from its list, calls the library routines that release it, and returns a stored value.
- `unlink_and_release_node`: Unlinks a node from its list, calls a library routine to release it, and returns a value stored in the node.
- `handle_request_state_transition`: Sets a request's state field and dispatches on its value: states six and seven update device flags and call library routines, states eight and one run a handler twice, and state two matches the request against a pending entry before queuing it.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `mark_request_state_seven`: Clears a device flag bit, sets the request state to seven, and calls two library routines to signal the change.
- `mark_request_state_six`: Sets a device flag bit, sets the request state to six, calls a library routine, and clears the request's status byte.
- `set_flag_then_clear`: Sets bit seven in a byte of the structure, calls two other routines, then clears that bit again.
- `set_flag_clear_then_call`: Sets bit seven in a byte of the structure, calls another routine, clears the bit, clears a status byte in a second structure, and calls another routine.
- `process_pending_list_entries`: Calls a routine when a device field is set, increments a busy counter, calls a routine for each entry in a linked list, then decrements the counter and writes the interrupt enable register when it reaches zero.
- `decrement_counter_and_set_global`: Decrements a byte counter in the global variable area and, if the previous value was less than one, writes a constant to the interrupt enable register, as recorded in the facts.
- `remove_request_from_list_safely`: Disables interrupts while it searches a request list for the given request.
- `remove_request_from_list`: Searches a linked list for the given request; if it is found, removes it and sets its status to a fixed error value.
- `unlink_request_and_complete`: Unlinks a request from its list, sets its status to a fixed error value, decrements a busy counter, and calls a routine to finish.
- `decrement_counter_and_set_global_2`: Decrements a byte counter held in a structure and, if the previous value was less than one, writes a constant to the interrupt enable register, as recorded in the facts.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `reject_request_with_status`: Stores a fixed error status in the request and calls a routine to report it.
- `complete_request_with_status`: Sets the request's status to zero and calls a routine to finish it.
- `clear_flags_and_call_handler`: Clears a flag bit in a structure, clears a byte in the request, and calls a handler through a library base.
- `validate_request_fields_and_split`: Checks a request's numeric fields against fixed ranges and, when they are valid, copies them into the device state.
- `check_input_range_then_split`: Checks an input value against a fixed upper limit and, when it passes, copies it and the request's numeric fields into the device state before splitting the request text at delimiters.
- `check_input_limit_then_split`: Checks an input value against a second fixed upper limit and, when it passes, copies it and the request's numeric fields into the device state before splitting the request text at delimiters.
- `store_word_then_split_text`: Stores an input value in the request's field, checks two more request fields against fixed ranges, and copies the request's numeric fields into the device state.
- `store_flag_then_split_text`: Stores an input value in a device field, checks a range on another request field and a flag field, copies the request's numeric fields into the device state, and then splits the request text at delimiter characters.
- `check_upper_limit_then_split`: Checks an input value against a fixed upper limit, stores it in the device state, validates a flag field, and then splits the request text at delimiter characters.
- `check_volume_then_split_text`: Rejects an input value above a fixed limit, stores the value as a volume-style byte in the device state, copies the request's text fields and splits the text at delimiter characters.
- `set_task_priority_and_split`: Looks up a task, frees storage and sets the task priority through exec library calls according to the facts, then copies the request's fields and splits its text at delimiter characters.
- `copy_counter_and_set_status`: Copies a count field from one structure slot to another, stores the low byte of the data register as a status byte, and calls a routine.
- `clear_counter_and_set_status`: Clears a count field in a structure, stores the low byte of the data register as a status byte, and calls a routine.
- `split_text_at_delimiters`: Scans backwards through the request text for a comma, hyphen or space and records the split position in the request, then runs a completion path that calls library routines and returns a status.
- `find_space_split_position`: Scans backwards through the request text for a space character to choose a split point, falling back to a computed position when none is found, then runs the completion path that calls library routines and returns a status.
- `split_text_and_complete`: Stores half of a count in a request field, scans the request text for a delimiter, and runs the completion path that calls library routines and returns a status.
- `setup_speech_request_buffers`: Clears per-request fields, stores a mode byte and a pointer taken from the request, and calls a library routine.
- `allocate_speech_buffer_copy`: Stores a mode byte from the request, allocates a buffer through an exec library call, and sets up its message port and signal fields.
- `create_speech_iorb_request`: Stores a request pointer and sets up its message port and signal fields, then allocates a buffer and copies the request template into it when the lookup entry is valid.
- `allocate_request_template_copy`: Stores a mode byte in the request, allocates a buffer through an exec library call, and copies a 34-word template into it, returning an error code when allocation fails.
- `copy_word_table_to_buffer`: Stores a buffer pointer in a global slot, then for a count taken from a byte in the structure copies a table of 34 words into the buffer and again into a position 34 words further along, once per iteration.
- `release_request_resources`: Stores a value in the request, calls a library routine to complete the request, frees the buffers and ports it allocated, clears a word, and returns the combined result.
- `handle_io_and_free_resources`: Stores a value in the request, calls a library routine, and then frees the buffers and the message port and signal according to the facts; it clears a word and returns a combined result.
- `match_character_pairs_in_table`: Returns the input unchanged when a count field is not positive.
- `translate_text_to_speech_codes`: Converts the request text into speech-code buffers: it clears three output buffers, matches each character or character pair against a 112-entry table, writes the matched codes, sets flag bits for two special codes, and ends the buffers with a marker byte when the text runs out.
- `translate_text_pair_step`: Handles one step of the text-to-speech-code translation: it treats digit characters specially, looks up the current character pair in the 112-entry table, writes the matched code or flags into the output buffers, and moves to the next character.
- `translate_text_char_step`: Handles one character of the text-to-speech-code translation: it looks up the character pair in a table, writes the matched code into the output buffer, and ends the buffers with a marker byte when the text is exhausted.
- `advance_text_translation_step`: Advances the text-to-speech-code translation by one character, writing matched codes, flag bits or the end marker to the output buffers, and returns the updated position.
- `apply_special_code_then_lookup`: Sets flag bits in the output buffers for two special codes, then looks up the next character pair in the 112-entry table and writes its code.
- `translate_text_special_step`: Handles a special marker in the text-to-speech-code translation, then looks up the next character pair in the 112-entry table and writes its code into the output buffers.
- `emit_speech_code_byte`: Writes one speech-code byte into the output buffer, advances the output position, and continues the translation until the buffer limit or end of text is reached.
- `translate_text_main_loop`: Runs the main text-to-speech-code loop: it reads the next character pair from the request, matches it against the 112-entry table, and writes the code or flags for each character until the text ends.
- `stub_return_only`: Contains only a return instruction and performs no work.
- `emit_speech_terminator_bytes`: Writes the end marker bytes into the three output buffers at the current position and stores the resulting length.
- `emit_marker_and_length`: Writes a marker code and end bytes into the output buffers at the current position, and stores the final length in the request.
- `store_output_position`: Stores the current output position in a field of the device state.
- `mark_stress_flags_in_codes`: Walks the speech-code buffers and sets flag bits on codes that match patterns in the flag table, using the codes that precede them.
- `mark_stress_flags_forward`: Walks the speech-code buffers forward and sets flag bits on codes whose table flags match the preceding codes, stopping at the marker byte.
- `mark_stress_flags_pass_two`: Walks the speech-code buffers and sets flag bits on codes according to the table flags of the preceding code, as a second pass over the same buffers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `mark_segment_flag_bits`: Walks the code buffers and sets bit 128 on each segment, then fills the segments that follow with flag bits derived from the preceding segment's flags.
- `mark_segment_flag_span`: Sets flag bits across a span of the code buffers, deriving the bit pattern from the segment at the span start and stopping when the span is covered.
- `mark_segment_flag_tail`: Continues the segment flag marking over the code buffers, setting the span flags and returning when the terminator byte is reached.
- `mark_segment_flag_run`: Sets flag bits across a run of segments in the code buffers, deriving them from the preceding segment, and returns when the marker byte is reached.
- `do_nothing_routine`: This routine is empty and performs no operations.
- `apply_matching_speech_rules`: Walks a table of three-character speech rules against the text buffer.
- `match_rule_context_and_apply`: Checks a rule's left and right context characters against the text.
- `scan_rule_chain_for_match`: Scans a chain of speech rules for one whose pattern matches the text at the current position, then checks its context and applies its replacement codes; the scan ends at the marker byte.
- `continue_rule_chain_scan`: Continues scanning the speech-rule chain from the current rule, matching patterns and their context, and applying replacements until the chain or the text ends.
- `test_bit_with_table_lookup`: Reads a byte from a buffer and a word from a table indexed by a word, modifies that word according to flag bits in the byte, and stops when a specific bit is set or the buffer's terminator is reached.
- `insert_byte_shifting_buffers`: When the output count is below its limit, increments it and shifts the bytes at the insertion point one place toward the end of three buffers.
- `shift_bytes_and_clear_fields`: Increments a word counter in a structure and shifts bytes backward in three buffers by a computed count, then writes a byte from a register and two zero bytes at the new ends of the buffers.
- `call_seven_routines_in_order`: Calls seven routines in sequence and returns zero.
- `clear_entry_flags_set_head`: Walks an eight-byte-entry table from its head until the sentinel, clearing the seventh byte of each entry, then sets the head entry's seventh byte to 160.
- `clear_entries_until_match`: Clears the seventh byte of eight-byte table entries until it reaches the entry whose first byte matches the argument, then sets the head entry's seventh byte to 160.
- `adjust_segment_timing_values`: Walks three parallel buffers of speech segments: for a punctuation code it splits a combined timing value between two neighbours, for flagged codes it halves a timing field, and for codes with a table flag it looks up a replacement value.
- `adjust_segment_timing_fields`: Applies the same segment timing adjustments as its sibling routine, splitting values between neighbouring segments and looking up replacement values from a table for flagged codes.
- `apply_timing_table_to_segments`: Walks the speech segments, looking up a replacement timing value for each flagged code, splitting values between neighbours for punctuation codes, and setting a stress-type field from the segment flags.
- `apply_timing_flags_to_segments`: Walks the speech segments using the flag bits of each code to set timing values and stress-type fields, splitting combined values for punctuation codes.
- `set_segment_stress_levels`: Sets stress-type bits in the segment field from the flag bits of each code, then continues through the segments until the terminator byte.
- `set_segment_stress_pattern`: Sets stress-type bits in the segment field from the flag bits of each code, with a different mapping from the sibling routine, until the terminator byte.
- `set_default_segment_stress`: Sets a default stress-type bit in each segment field and then applies the flag-derived stress values to the segments that follow.
- `copy_segments_with_stress_flags`: Copies speech segments into the output buffers, setting stress-type fields from the flag bits of each code and splitting combined values for punctuation codes, until the terminator byte.
- `allocate_frame_buffers_from_total`: Sums the length values of the segment table up to its end marker, stores the total and the derived buffer size in the device state, and allocates the frame buffer through an exec library call, plus a second buffer when a flag is set.
- `allocate_frame_buffers_for_size`: Stores a size derived from the input value in the device state and allocates a frame buffer through an exec library call, plus a second buffer when a flag is set.
- `allocate_secondary_frame_buffer`: Stores the input pointer in the device state, and when a flag is set allocates a secondary buffer through an exec library call and records its pointer.
- `fill_frames_for_segments`: Expands each speech segment into eight-byte frame records, copying the segment's table parameters into the frame and setting pitch and flag bits, then writes a terminator record at the end of the table.
- `fill_frames_from_code_stream`: Reads the speech-code stream and writes eight-byte frame records for each code, copying table parameters and flag bits into the frames and ending the table with a terminator record.
- `fill_frames_from_table_values`: Writes eight-byte frame records from table values for each code, adjusting parameters by the segment flags and ending the record list with a terminator.
- `interpolate_frame_values`: Smooths a parameter across neighbouring frames: for each frame pair whose code flags allow it, it blends the two values with a five-step weighting derived from the frame length.
- `interpolate_frame_pair_values`: Applies the same five-step weighting between neighbouring frame values as its sibling routine, using a table of weights and the frame lengths.
- `interpolate_frame_sequence`: Walks a sequence of frames and blends each frame's values toward its neighbours using five-step weights, moving to the next frame after each step.
- `interpolate_linear_frame_run`: Interpolates a run of frame values linearly from one end to the other, stepping through the frame records and stopping at the terminator code.
- `interpolate_linear_frame_step`: Performs one linear interpolation step across a run of frame values, weighting each value by a table entry and moving to the next frame.
- `build_frame_extents`: Writes frame records for a code with a flag-selected pattern, marks extents with sentinel bytes, and fills the remaining frame slots of the segment.
- `compute_frame_amplitude_values`: Computes per-frame amplitude and pitch-related values from the code tables, writing them into the frame records and marking the frames that carry flags.
- `distribute_frame_durations`: Distributes each segment's frame count across eight-byte frame records, computing the split between neighbouring segments from their table values, then writes the frame data and sentinel bytes.
- `fill_frame_run_from_codes`: Writes a run of eight-byte frame records for each speech code, deriving the run length from the code tables and the neighbouring codes, and ends the output with the terminator byte.
- `fill_frame_run_with_flags`: Writes a run of frame records for each code, choosing the run length and sentinel fields from the code's flag bits and the next code, until the terminator byte is reached.
- `fill_frame_extents_pass`: Writes frame extents for each code according to its flag bits, computing the run lengths from neighbouring code values and marking the record slots with sentinel bytes.
- `fill_trailing_frame_run`: Fills the last frame run of a segment from the neighbouring code values, writing the frame records and sentinel bytes until the terminator byte is reached.
- `fill_frame_run_tail`: Completes the frame records for the final segments, writing the remaining slots and sentinel bytes, and returns when the terminator byte is reached.
- `fill_records_from_stream_pairs`: Reads two parallel byte streams and a lookup table, and for each step writes zero-filled four-byte records into an output buffer, adding a fill byte and word when a flag bit is clear.
- `emit_record_runs_from_stream`: Walks two byte streams against a flag table and writes runs of zero-filled records, with optional fill values, into an output buffer until the end marker of 255 is read.
- `emit_packed_count_record_runs`: Decodes packed run counts from a data stream and writes blocks of zero-filled records with an optional fill word into an output buffer.
- `compact_parallel_table_entries`: Copies three parallel byte tables in place, keeping only entries whose flag bit in the lookup table is clear.
- `write_byte_to_three_pointers`: Stores one byte from a data register into three memory locations, each addressed by its own address register, and returns.
- `call_two_handlers_in_sequence`: Calls two unnamed handler routines one after the other and returns.
- `clear_low_nibbles_mark_flags`: Clears the low four bits of each entry in one table up to its end marker, then walks two linked tables and sets flag bits on the entries that begin and end each run.
- `set_table_flag_bits_walk`: Walks two linked byte tables and sets flag bits on the entries that begin and end each run, using a per-byte attribute table to choose them.
- `mark_run_flags_from_stream`: Reads a byte stream and walks a flag table in step with it, setting flag bits on the boundary entry of each run until the stream reaches its end marker.
- `mark_run_boundary_flags_nested`: Walks two linked tables with nested loops and sets flag bits on the first and last entry of each run, choosing the marked entry with attribute tests.
- `mark_run_end_flags`: Walks a flag table against an attribute table and sets flag bit values on run starts and ends.
- `set_flag_bits_walking_tables`: Walks paired byte tables and sets the first two flag bits on each entry where a run begins, then sets a third bit on the run's closing entry.
- `mark_run_edge_flags`: Sets flag bits on the opening and closing entries of each run in a flag table, using a lookup table to decide which neighbour gets the mark.
- `compute_divided_period_records`: Divides a fixed constant by stored period values to produce output values, then fills eight-byte records from several parallel tables.
- `write_period_bytes_by_block`: Divides a fixed constant by stored parameter values and writes the quotients into eight-byte record fields, looping over blocks that end at the end marker of 255.
- `emit_divided_period_block_records`: Writes divided period values into grouped eight-byte records, with each group's length taken from a length table.
- `fill_divided_values_across_runs`: Computes quotients from a fixed constant and per-entry parameter bytes, and spreads them over runs of records bounded by flag bits.
- `fill_values_with_run_lengths`: Writes divided parameter values into record fields across runs whose lengths come from a flag-marked table.
- `write_values_per_block_pair`: Loops over blocks of records, writing quotients of a fixed constant and stored parameter bytes into record fields, with a sub-loop that counts run lengths.
- `fill_quotient_into_strided_fields`: Divides a fixed constant by a stored value and writes the quotient into every eighth byte of a buffer until a sentinel byte of minus one.
- `stub_no_operation_return`: Returns immediately without reading or writing any data.
- `compute_clamped_level_per_entry`: Walks entries until the end marker of 255, calling unnamed handler routines selected by flag bits, and merges a scaled lookup value into the low six bits of each output byte.
- `compute_clamped_level_entries_loop`: Loops over entries until the end marker of 255, calls unnamed handler routines selected by flag bits, and merges a scaled lookup value into the low six bits of each output byte.
- `compute_clamped_levels_from_table`: Walks entry tables, skipping entries whose flag bit is set, and calls unnamed handlers chosen by neighbouring flag bits before writing a clamped value of at most 63 into each output byte.
- `compute_clamped_levels_with_fallback`: Writes a clamped value of at most 63 into the low six bits of each output byte, using 24 as a fallback for entries that fail a flag test.
- `compute_levels_with_flag_skips`: Walks entry tables until the end marker of 255 and merges a clamped level of at most 63 into the low six bits of each output byte.
- `compute_levels_neighbour_flags`: Merges a clamped level of at most 63 into each output byte, choosing the entry from flag bits of neighbouring entries.
- `compute_clamped_levels_entry_walk`: Walks entries to the end marker of 255, writing a clamped level of at most 63 into the low six bits of each output byte.
- `compute_clamped_levels_tail_loop`: Writes clamped levels of at most 63 into the low six bits of output bytes while walking entry tables to the end marker of 255.
- `compute_clamped_levels_skip_flags`: Walks entry tables and merges a clamped level of at most 63 into each output byte, skipping entries whose flag bit is set.
- `compute_clamped_levels_pair_flags`: Writes clamped levels of at most 63 from a table-pair walk, using the flag bits of the current entry and its neighbours to decide the path.
- `compute_clamped_levels_run_marks`: Calls an unnamed handler, then walks entry tables writing clamped levels of at most 63 into output bytes.
- `compute_clamped_levels_run_loop`: Loops over entry tables and writes clamped levels of at most 63 into output bytes, calling unnamed handler routines when run-mark flag bits match.
- `multiply_and_round_shift16`: Multiplies two 16-bit inputs, adds a rounding offset, shifts the product right by five and returns the result.
- `clear_tables_set_base_pointers`: Stores base addresses for eleven working tables in a state block, then zeroes 32-entry blocks of those tables plus two smaller tables.
- `advance_counter_call_handlers`: Increments a 16-bit counter in the state block and calls an unnamed handler.
- `build_entry_flags_and_count`: Scans a byte table until the end marker of 255, writing packed low nibbles with flag bits into an output table and counting entries.
- `pack_flagged_entry_nibbles`: Packs entries from two byte tables into output bytes by combining low nibbles with flag bits, counting entries up to 128.
- `pack_entries_with_flag_runs`: Writes packed output bytes from table entries, tracking flag runs and counting qualifying entries up to 128.
- `mark_last_entry_position`: Sets the end flag on the last written output byte and records that position and the first qualifying position in the state block.
- `resolve_output_bytes_backwards`: Walks output bytes backwards from the entry count, combining flag tables and moving values into earlier slots while clearing vacated ones.
- `pack_entries_then_resolve_flags`: Packs table entries into output bytes with flag bits until a stop value is read, then walks the packed output backwards to resolve marker flags and shift values into earlier slots.
- `pack_entries_and_resolve_markers`: Packs entries into output bytes with marker flags and then resolves the marker slots by shifting values into earlier positions.
- `resolve_packed_marker_slots`: Walks packed output slots backwards, setting marker bits on special codes and shifting each value into an earlier slot that carries the group flag.
- `resolve_marker_slots_from_index`: Walks output slots backwards from a given index, marking entries with flag bits and shifting values into earlier slots.
- `shift_marker_values_backwards`: Walks packed slots backwards, shifting values into earlier slots whose flag bit is clear and setting marker bits on special codes.
- `shift_values_into_marked_slots`: Copies values into earlier marked slots while walking packed output backwards, clearing vacated positions and setting group flag bits.
- `set_low_flag_bit_countdown`: Sets the lowest flag bit on every entry of a state table, counting down from a given index to zero.
- `set_flag_bit_countdown_inplace`: Sets flag bit 1 on each byte of an input block, counting down from a given index to zero.
- `set_class_flags_until_stop`: Maps the class byte before the current position to a flag value and ORs it into flag bytes, walking backwards until a stop bit is found.
- `apply_class_flags_last_entry`: Writes a class flag byte at the current position, then, when a counter is nonzero, clears the low nibble of the last flagged entry and sets its bit 4.
- `add_offset_to_seven_counters`: Optionally calls unnamed handler routines, then adds a stored offset to seven 32-bit counters in the state block.
- `compute_banded_table_value`: Computes a value from a scaled parameter pair and a fixed offset, clamps it so that results below 126 become 125 and results above 164 become 165, then stores it in a table entry.
- `store_clamped_band_value`: Clamps a value in a register so that results below 126 become 125 and results above 164 become 165, then stores it at an index in the output table.
- `smooth_and_scale_flagged_values`: Adjusts a table of signed values by subtracting a stepped amount after a start index, applies difference-based corrections to flagged entries walking backwards, then scales each flagged entry by a four-bit factor around a fixed offset.
- `apply_neighbour_difference_correction`: Walks backwards through flagged entries, adding a correction that depends on the difference to the neighbouring entry, and then scales the flagged entries by a four-bit factor.
- `compute_paired_scaled_outputs`: For each flagged entry, computes two output values from a four-bit signed nibble and a fixed-offset input, adding a rounded term when a second flag is set.
- `apply_rounded_pair_scaling_backwards`: Walks flagged entries backwards and updates each pair of output values from a four-bit nibble and a fixed-offset input, adding rounded terms.
- `store_scaled_pairs_with_rounding`: Adds rounded scaled terms to a pair of output tables for each flagged entry, walking backwards until the index reaches the start.
- `smooth_flagged_value_pairs_backwards`: Walks flagged entries backwards and adjusts each pair of table values by fixed-point multiply-and-shift steps that depend on the gap between flags.
- `smooth_value_pairs_by_gap`: Adjusts pairs of flagged table values with fixed-point steps chosen by the gap size between flags, walking backwards to the start of the table.
- `rescale_pairs_with_neighbour_correction`: Applies fixed-point adjustments to value pairs and a neighbour correction to the second value, walking flagged entries backwards.
- `rescale_pairs_and_propagate_maximum`: Adjusts flagged value pairs with fixed-point steps, then scales each flagged entry against the largest earlier value or a fixed offset depending on its class nibble.
- `smooth_pairs_with_gap_scaling`: Adjusts pairs of table values with fixed-point steps whose size depends on the gap between flagged entries, walking backwards to the start of the table.
- `smooth_pairs_then_scale_maximum`: Applies fixed-point corrections to pairs of table values walking backwards, then scales each flagged entry against the largest earlier value or a fixed offset.
- `propagate_pair_corrections_backwards`: Propagates a correction value from each flagged pair into the earlier entries while walking backwards, and applies the fixed-point adjustments to both tables.
- `scale_pairs_by_gap_class`: Scales pairs of table values with fixed-point factors chosen by the gap between flags, then rescales flagged entries using a fixed offset for each class nibble.
- `smooth_pairs_stored_gap_factor`: Applies fixed-point corrections to pairs of table values using a gap factor kept from the previous step, walking backwards through flagged entries.
- `smooth_pairs_resume_after_gap`: Resumes the fixed-point pair corrections after a flagged gap, updating both tables and the second entry of each pair.
- `smooth_pairs_resume_pair_end`: Continues the backwards pair corrections at the end of a pair, applying fixed-point steps to the second table and the flagged entries.
- `smooth_pairs_with_final_maximum`: Applies fixed-point corrections to pairs of table values walking backwards, then scales each flagged entry against a running maximum or a fixed offset.
- `rescale_entries_to_running_maximum`: Walks flagged entries backwards and scales each against the largest earlier value in its run, using a fixed offset for class-four entries.
- `rescale_entries_with_carried_maximum`: Scales the current entry by a fixed factor, carries the largest value seen so far into the next scaling step, and writes the reduced value into the second table.
- `rescale_entries_maximum_reference_loop`: Repeats the maximum-referenced rescaling over flagged entries, updating the reference value as each run is consumed.
- `scale_entries_by_class_nibble`: Scales each flagged table entry by a factor chosen from its class nibble, with values of one or two taking special factors, and writes the results to a parallel table.
- `scale_entries_flag_nibble_back`: Walks flagged entries backwards and applies a fixed-point scale to each, with the factor selected by the low flag nibble.
- `scale_entries_flag_nibble_rounded`: Applies a rounded fixed-point scale to flagged entries walking backwards, with the factor chosen by the flag nibble.
- `distribute_corrections_class_reset`: Spreads run corrections across flagged runs and resets trailing values when a class nibble is set.
- `distribute_run_corrections_copy_forward`: Copies table values forward across each flagged run, subtracting a per-run correction from each copy.
- `distribute_run_corrections_run_loop`: Loops over flagged runs, copying values forward and subtracting a share of the run correction, then writes the closing entry from a maximum or fixed marker.
- `spread_corrections_fill_copies`: Spreads run corrections over flagged entries while copying values forward, with fill values written for the entries between flags.
- `spread_corrections_handler_calls`: Spreads run corrections across flagged runs and calls unnamed handler routines when a run has one or two entries.
- `spread_run_corrections_handler_fallback`: Spreads run corrections over flagged entries, calling unnamed handler routines for runs of one or two and using a copy loop for longer runs.
- `fill_run_tail_class_base`: Fills trailing table entries with a base value of 110 plus a class nibble, then spreads the corrections across the last flagged run.
- `fill_tail_base_then_close`: Writes trailing entries as 110 plus a nibble offset, spreads run corrections and writes the closing entry from a maximum-based value or fixed marker.
- `spread_corrections_for_class_mode`: Spreads a correction across the last flagged run using a per-class base of 110 or 75.
- `copy_run_values_close_entry`: Copies a run of table values forward while subtracting a correction from each copy, then writes the closing entry as a maximum-based value or a fixed marker of 75 or 110.
- `write_maximum_based_closing_entry`: Scans the output values for their largest value, scales it by a fixed factor, and writes the result as the closing entry with its paired copies.
- `scale_maximum_into_closing_entry`: Finds the largest value in a run of output bytes, scales it by a fixed factor and writes the result as the closing entry.
- `adjust_tables_flag_walk_backwards`: Walks table entries backwards, applying fixed-point adjustments to three parallel tables where flag bits match, and writes the scaled corrections into a fourth table.
- `adjust_tables_flag_fallback_walk`: Walks flagged entries backwards with fixed-point adjustments, using fallback scale factors when a flag test does not match.
- `adjust_tables_walking_backwards`: Walks table entries backwards, adjusting parallel tables with fixed-point steps selected by flag bits and writing a scaled correction into a fourth table.
- `adjust_tables_with_flag_fallbacks`: Applies fixed-point adjustments to parallel table entries walking backwards, with fallback factors when a flag test fails.
- `adjust_tables_resume_at_entry`: Resumes the backwards adjustment of parallel table entries, applying fixed-point steps to three tables and a flag-selected scale to the output table.
- `adjust_tables_with_class_scaling`: Applies fixed-point scaling to parallel table entries as the walk moves backwards, choosing each factor from the flag bits of the entry.
- `adjust_tables_by_flag_steps`: Walks entries backwards and adjusts parallel tables with fixed-point steps chosen by flag bits, using a fallback factor at the end of a block.
- `adjust_tables_with_fixed_factor`: Applies a fixed-point factor to a table entry, then continues walking entries backwards with flag-selected adjustments until the start of the table.
- `adjust_tables_alternate_factors`: Alternates between two fixed-point factors while walking table entries backwards, adjusting parallel tables when flag bits match.
- `decay_run_values_forward`: Copies table values forward one step at a time, scaling a running value by a table entry each step and subtracting it from the copy.
- `decay_run_values_count_zero`: Repeats the forward copy with running scaling for a counted run, subtracting the scaled value from each copy and mirroring the result into a third table.
- `run_handler_sequence_twelve`: Calls an unnamed handler routine twelve times in a row, then calls it once more when a status byte is nonzero.
- `call_three_handlers_in_sequence`: Calls an unnamed handler routine three times in a row and returns.
- `interpolate_stride_eight_values`: Linearly interpolates between two byte endpoints across a number of steps, writing each interpolated value into every eighth byte of a buffer.
- `walk_byte_records_until_end`: Walks eight-byte records, skipping entries marked with minus two and calling an unnamed handler for records that are not the first.
- `walk_records_with_handler_calls`: Walks eight-byte records and calls an unnamed handler between records whose marker differs from the previous one.
- `call_handler_then_walk_records`: Calls an unnamed handler routine, then walks eight-byte records, skipping those marked minus two and stopping at the end marker of minus one.
- `scan_records_until_sentinel`: Scans eight-byte records for nonzero marker bytes, calling an unnamed handler for records after the first, and returns when the sentinel of minus one is found.
- `scan_records_skip_zero_stride`: Steps through eight-byte records skipping zero marker bytes, calling an unnamed handler between records, and returns when the end marker of minus one is read.
- `scan_records_with_leading_handler`: Walks eight-byte records and calls an unnamed handler before each scan, stepping past records whose marker byte is zero.
- `write_table_driven_pattern_records`: Reads a command table and a count table, and for each entry either copies five-value patterns chosen by the command byte or writes counted eight-byte records with added increments.
- `emit_pattern_records_from_commands`: Writes eight-byte output records from a command byte stream: plain commands copy a count of records, while flagged commands copy five-value patterns chosen by the command.
- `emit_command_pattern_records`: Reads command and count streams, writing five-value patterns chosen by command for flagged commands and counted eight-byte records with additive steps otherwise.
- `emit_command_pattern_records_resume`: Resumes writing command-driven records, copying five-value patterns for flagged commands and counted records with additive steps for the rest.
- `emit_command_pattern_records_split`: Writes counted records for each command byte, using five-value patterns for the matched command codes and additive steps for paired fields.
- `emit_patterns_for_command_codes`: Chooses one of several five-value source patterns from the command code and writes it repeatedly for each counted step, then continues the command stream.
- `emit_command_pattern_records_tail`: Writes five-value patterns for command codes at the end of the command stream, then emits counted eight-byte records with additive steps.
- `update_record_nibbles_from_flags`: Rewrites the low nibbles of paired record fields with halved, rounded values for flagged commands, and clears the high nibble of the affected fields.
- `update_record_nibbles_with_repeat`: Applies masked nibble updates to paired record fields and repeats earlier records for commands one and two, stopping at the end marker of 255.
- `update_record_nibbles_masked`: Walks command bytes and record bytes together, copying repeat records forward and rewriting masked low nibbles in paired record fields for flagged commands.
- `update_record_nibbles_chain`: Continues the chained record update, copying earlier records for repeat commands and rewriting masked nibbles for others, and sets a carry flag on each processed command.
- `update_record_nibbles_tail`: Applies the masked nibble updates to the trailing record fields and copies repeat records forward for the remaining commands.
- `remap_record_bytes_through_table`: Walks eight-byte records and replaces the first three bytes of each with values from a lookup table, skipping records whose marker is 254.
- `lookup_three_bytes_per_record`: Replaces the first three bytes of each eight-byte record with values taken from a lookup table, moving forward until a marker byte matches the end value.
- `stub_return_only_2`: Returns immediately without touching any data.
- `accumulate_record_sums_into_slot`: For each eight-byte record, adds up its six leading byte fields until an end marker of 255 is met, then adds the sum shifted right by three into a fixed byte of the record stream.
- `sum_record_fields_into_slot`: Adds record fields with fixed weights across six-field groups until a stop marker matches, then writes the shifted total into a fixed slot of each record.
- `combine_weighted_record_fields`: For each record, sums fields with doubled weights, subtracts the first and seventh fields, and combines the result with the slot value scaled by four.
- `combine_weighted_fields_by_stride`: Accumulates weighted field values in a strided walk over records and stores a shifted combination in each record's slot byte.
- `apply_command_offsets_to_records`: Walks command and count streams and adds a per-command offset byte to the seventh byte of each counted record, with special offsets for two command codes and a fixed fill for one.
- `add_command_offsets_record_seventh`: Adds command-dependent offsets to the seventh byte of counted records, using a shifted field for one command group and fixed fills for others.
- `add_command_offsets_resume_loop`: Resumes adding command-dependent offsets to the seventh record byte across counted runs, with a fixed offset for two command codes.
- `add_fixed_offsets_per_run`: Walks command and count streams, adding a fixed offset to the seventh byte of each record in a counted run.
- `add_fixed_offsets_command_groups`: Writes fixed offsets to the seventh record byte for counted runs, choosing the offset from the command group and skipping records for fill commands.
- `add_fixed_offsets_until_terminator`: Adds group-dependent offsets to the seventh byte of counted records until the command stream reaches the terminator value of 255.
- `add_fixed_offsets_mixed_groups`: Writes fixed fill offsets into the seventh byte of counted records, mixing fill runs with counted groups from the command stream.
- `add_offsets_with_shifted_field`: Adds offsets derived from a shifted field to the seventh record byte in counted runs, and fixed fills for command group codes.
- `average_nibbles_in_sliding_window`: Slides a window of five bytes across a buffer, summing low nibbles of neighbouring bytes with weights and writing the shifted result into one nibble of a fixed byte.
- `setup_task_signals_wait_loop`: Looks up the current task through an Exec library call and stores it, allocates two signal bits and records their combined mask, then loops calling a handler and waiting for signals.
- `process_task_signals_loop`: It repeatedly sets a bit in a status byte of a state block, then calls a routine pointed to by a table entry.
- `clear_busy_status_flags`: Clears two status bits in the state block and returns.
- `initialise_audio_channels_and_dma`: Fills channel tables and computes a period value from a fixed constant divided by a stored value, calls library routines to set up message and list nodes, and writes the custom chip DMA control register before the sample output loop.
- `copy_thirtythree_byte_blocks`: Copies blocks of 33 bytes from a source buffer to a destination, advancing both pointers, until the block count is used up.
- `mix_sample_stream_channel_records`: Mixes sample bytes from lookup tables into the output buffer while parsing channel records that set envelope and table pointers.
- `mix_wave_samples_position_masks`: Writes combined wave-table samples into the output buffer while stepping sample positions with masks and advancing a block counter.
- `mix_three_table_samples`: Sums three lookup-table sample values per step into an output buffer, updating position and envelope counters and rewriting the DMA control register at block boundaries.
- `mix_channel_with_envelope_updates`: Mixes sample bytes from lookup tables into the output buffer while parsing channel records that set envelope and table pointers and clear parameters when a flag is set.
- `mix_channel_records_into_buffer`: Mixes three table samples per step into the output buffer, parsing channel records as they are reached and wrapping the sample position.
- `mix_block_position_wrap`: Mixes sample bytes from lookup tables into the output buffer for one block, wrapping the sample position and parsing channel records as they are reached.
- `stop_channels_and_reset_dma`: Calls an unnamed routine once per channel and, when a channel's status bit is set, clears it and writes the DMA control register with the channel-enable mask.
- `write_dma_enable_flag`: Writes the custom chip DMA control register with the channel-enable mask when a status flag is set, then returns.
- `relink_entry_on_value_change`: When a byte value differs from the stored one, it updates the stored value and moves a list node: it unlinks the first node, relinks it to a list head, decrements a counter, and calls a library routine.
- `decrement_counter_reset_mask`: Decrements a counter stored in the state block and writes a control mask to a hardware register when the previous value was below one.
- `mark_channel_nodes_start_dma`: Marks each node of a channel list as pending by calling a library routine for it, then clears a status bit and writes the DMA control register to start playback.
- `mark_channel_nodes_from_offset`: Marks channel list nodes from a computed offset as pending, calling a library routine for each, and then restarts DMA or sends the closing request depending on a status bit.
- `wait_channels_idle_disable_dma`: Calls an unnamed routine for each channel until one reports busy, in which case it sets a busy bit and returns.
- `set_busy_status_bit`: Sets the busy bit in a state byte and returns.

### Message

- `conditionally_call_library_then_handler`: Calls two unnamed routines, calls a library routine at a negative offset when a condition flag is clear, then calls one more unnamed routine and returns.
- `store_library_call_result`: Calls a library routine through a global base address and stores the returned value in a shared variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_fifteen_longwords_and_call`: Copies fifteen 32-bit words from a fixed table into the destination when that pointer is non-null, then calls a routine through a function pointer.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:intro

- `start_and_reference_imp_text`: Program entry point that references the text IMP! and makes one call to an unnamed routine.
- `set_background_colour_then_jump`: Touches the background colour hardware register, then passes control through a jump-table entry to a fixed address and makes one further unnamed call.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `decrunch_backwards_bitstream_block`: Decompresses data in place from the end of a buffer toward the start, reading a bit stream to choose literal and copied runs.
- `read_bitstream_length_code`: Decodes a variable-length code from a backwards bit stream by counting bit groups, producing a small count and a base value for the decompressor.
- `decode_backref_copy_loop`: Decodes length and distance codes from the backwards bit stream, copies a back-reference and then a literal run into the output buffer, and repeats until the buffer start is reached.
- `stub_single_call_forward`: A three-instruction routine that makes one call to an unnamed routine.
- `stub_single_call_forward_2`: A three-instruction routine that makes one call to an unnamed routine.
- `decode_bitstream_run_length`: Decodes a run-length-coded bitstream by reading bits from a source buffer and using a lookup table to expand each code into a repeat count and a value, then writes the decoded data through a pointer.
- `relay_through_shared_call_site`: A seventeen-instruction routine with five callers that makes one call to an unnamed routine.
- `setup_then_call_unnamed_routine`: A twenty-eight-instruction routine with one caller that makes one call to an unnamed routine.
- `setup_and_call_unnamed_routine`: A seventeen-instruction routine with one caller that makes one call to an unnamed routine.

### TRANSLATOR.LIBRARY

- `create_and_add_translator_library`: Library code in the translator library: calls the Exec MakeLibrary function to create a library base, then AddLibrary to register it with the system, storing the segment pointer in the new base.
- `increment_counter_clear_status_bit`: Library handler that increments a 16-bit counter in the library structure and clears one status bit in a flag byte.
- `decrement_counter_and_notify`: Decrements a 16-bit counter in the library structure and, when it reaches zero with a status bit set, calls an unnamed routine and returns its result.
- `unlink_node_or_flag_pending`: If a counter is zero, unlinks a node from a list and calls a library routine with a stored value.
- `unlink_node_and_call_library`: Unlinks a node from a doubly linked list, then calls a library routine at a fixed offset with a stored value and returns that value.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `parse_text_fields_into_buffer`: Library code that fills a 26-byte buffer with spaces, then scans an input string for space-separated fields until a hash terminator, calling unnamed routines to read each field.
- `return_low_word_register`: Returns the low 16 bits of a data register as its result.
- `clear_flag_return_word`: Calls an unnamed routine, clears a flag byte addressed from an address register, and returns a 16-bit value from a data register.
- `scan_text_fields_space_delimiters`: Scans text fields separated by spaces, calling unnamed routines for each field, and stops at a hash terminator.
- `scan_text_fields_until_hash`: Continues scanning space-delimited text fields until a hash terminator, clears the end byte and returns the scan status.
- `scan_escape_sequences_in_text`: Scans text one character at a time, stopping at a backslash or a backquote, and calls unnamed routines to handle the escape.
- `scan_text_for_escape_markers`: Scans text for backslash and backquote markers, calling unnamed routines to handle each marker and continuing until the text ends.
- `stub_bad_instruction_data_2`: Only a return appears in the listing, although the facts report sixteen instructions, so this is a stub or bad instruction data.
- `copy_bracketed_text_span`: Skips characters until an opening bracket, then copies the following characters into a buffer up to the closing bracket and returns.
- `scan_backward_for_flagged_characters`: Walks a buffer backwards one character at a time and stops on characters whose lookup flag is set.
- `stub_return_only_3`: Returns immediately with no effect.
- `scan_counted_text_chunks`: Repeatedly calls unnamed routines to handle text chunks, counting down a length and returning when the count goes negative or a status flag is set.
- `read_token_until_equals`: Reads characters from a text buffer until an equals sign, classifying each one with a lookup table flag, and returns when the terminator is found.
- `scan_text_chunks_forward`: Scans text chunks forward through a buffer, calling unnamed routines for each chunk and returning when a status flag is set.
- `measure_text_run_to_marker`: Scans forward for a backslash or a backquote and returns the length of the text run up to that marker, packed into a register pair.
- `terminate_text_at_flagged_char`: Walks backwards through a text buffer until a character with a lookup flag is found, then writes a zero terminator at that position.
- `copy_text_run_flag_update`: Copies a counted run of bytes into a destination buffer, sets a status flag, and checks the next byte for a backquote marker before finishing.
- `terminate_text_before_flagged_char`: Walks backwards through a text buffer until a character with a lookup flag is found, then writes a zero terminator after the preceding text and returns.
- `dispatch_by_marker_character`: Searches a twelve-character marker table for the current byte and calls the handler at the matching position of a jump table, then returns.
- `stub_bad_instruction_data_3`: Only a return appears in the listing, although four instructions are reported.
- `stub_bad_instruction_data_4`: Only a return appears in the listing, although six instructions are reported, so this is a stub or bad instruction data.
- `relay_single_unnamed_call`: Calls one unnamed routine and returns.
- `call_routine_while_flag_set`: Calls an unnamed routine once, then keeps calling it while a status bit in a register is set, and returns.
- `relay_unnamed_call_on_handler`: Calls one unnamed routine and returns.
- `check_status_char_after_call`: Calls an unnamed routine, then when a status bit is set calls another and compares the returned character with I and E before returning.
- `empty_return_stub`: Library routine that does nothing but return, so it is an empty stub in TRANSLATOR.LIBRARY with no observable work.
- `match_keyword_character_sequence`: Library routine that fetches characters in turn and compares them against fixed letter sequences (E, S, D, R, L, Y, N, G), returning early whenever a status flag bit is clear.
- `match_ng_then_s_sequence`: Library routine that checks for the letter N followed by G, then optionally a further S, fetching a character between comparisons and testing a flag bit at each step.
- `return_empty_routine`: This routine is an empty stub in the library code, containing only a return with no visible behaviour.
- `delegate_to_single_call`: Library routine that makes one call to another routine and then returns, adding no logic of its own.
- `return_empty_routine_2`: This library routine is an empty stub that only returns, despite four instructions being reported in the listing.
- `execute_and_return`: It executes three instructions and then returns, with no library calls or data accesses visible in the listing.
- `return_to_caller`: Executes three instructions that return control to the caller, with the listing showing a reference to a bad-instruction stub but no actual call being made.
- `poll_bit_five_until_clear`: Library routine that calls another routine in a loop for as long as bit five of a data register stays set, then returns.
- `copy_global_variable`: It loads a value from a global variable into a register, stores that value back into the global variable, and then returns.
- `poll_bit_one_until_clear`: Library routine that calls another routine in a loop for as long as bit one of a data register stays set, then returns.
- `call_once_then_return`: Library routine that calls another routine once, then tests bit seven of a data register, but both outcomes return immediately, so it performs no work beyond that call.
- `return_immediately`: This routine contains no instructions and simply returns without performing any operation.
- `copy_normalized_text_line`: Library routine that copies a text string into a buffer, turning control and delete characters into spaces, converting lowercase letters to uppercase, and mapping the right bracket to a space.
- `write_space_before_address`: Library routine that writes a space character two bytes before an address held in an address register, then returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d1, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 32 to (a2)+, then copies 32 to (a2)+, then returns.
- `insert_vowel_stress_marker`: Library routine that scans phonetic text for any of fifteen two-letter vowel codes from its reference table, and where one matches it inserts a digit 4 marker after the code, shifting the rest of the text along.
- `insert_marker_shifting_buffer`: Library routine that shifts the rest of a text buffer one place to the right, writes a digit 4 into the freed slot, and advances the stored buffer pointer.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:PAUSE

- `wait_while_port_bit_set`: Routine in the PAUSE file that busy-waits while bit six of the CIA port A register (CIAA_PRA) stays set, then returns.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:delphine

- `load_hunks_and_relocate`: Main program startup routine that allocates memory for each code, data and uninitialised section, then reads the section type codes and applies the relocation records that patch addresses.
- `stub_empty_return_routine`: The decompiled body is empty apart from a return, so this routine has no visible behaviour.
- `count_register_down_loop`: Program routine that counts a data register down until it passes through zero, then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 80 into d0.
- `unpack_compressed_stream`: Program routine that decodes a compressed byte stream, alternating between copying literal bytes forward and copying earlier output back along the destination buffer, with lengths read from the stream.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_count_loop`: Program routine that repeatedly decrements a byte held in a data register for a count taken from another register, replacing a zero value with a space as it goes.
- `stub_bad_instruction_data`: Program fragment that the decompiler could not translate, so it holds only bad instruction data and no usable behaviour.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:add21k

- `clear_state_block_then_dispatch`: Entry routine for the add21k file that clears a block of twenty-three longwords, saves the start-up register, calls one routine only when a system flag bit is set, then branches on a result to one of two follow-on routines.
- `open_system_libraries_at_start`: Start-up routine for the add21k file that opens the dos, graphics and intuition system libraries, raising an alert if one cannot be opened.
- `empty_return_stub_2`: Routine that does nothing but return, so it is an empty stub in the add21k file with no observable work.
- `build_start_up_argument_list`: Start-up routine that allocates a memory block for the program's data, copies the command line into it, and splits the line into words on spaces and tabs, keeping double-quoted text together.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `split_text_into_word_array`: Routine that copies a program name into a newly allocated buffer, then splits the rest of a text line into words, keeping double-quoted text together.
- `append_string_to_end`: Appends one null-terminated text to the end of another, copying no more than about 32 thousand characters and ending the result with a zero byte.
- `append_string_with_count`: Appends a null-terminated text onto the end of another, stopping once the count passed in the third argument is used up.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining count with zeros.
- `load_window_icon_entry`: Opens the icon library, looks up the entry named WINDOW and stores the two resulting pointers in the structure it is given, then closes the library.
- `call_optional_handler_then_continue`: Calls a stored handler through a pointer if one has been set, then calls another routine.
- `execute_cleanup_pointer`: Checks a memory location for a non-zero value and executes the function pointed to by that value.
- `release_allocated_resources`: Teardown routine that frees each entry in a table of allocated blocks, releases the remaining buffers and resources it holds, and returns a result code.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_and_clear_file_slot`: Closes the file held in a numbered slot of a table unless the slot's flag marks it as not owned, then clears the slot.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `get_dos_output_handle`: Library wrapper that loads the dos library base from a stored pointer and calls its output-handle function, passing back the result.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `forward_stored_library_call`: Tiny wrapper that loads a library base from a stored pointer and jumps to one library function through its vector, passing control and the result straight back.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_offset`: Loads a library base from a global variable and performs an indirect jump through a vector at a negative offset from that base.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `invoke_exec_jumptable`: Looks up a routine in a jump table located in the Exec library and invokes it.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_stub`: Runtime segment routine that returns the value minus one and performs no other work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_callbacks`: Checks a value in a table and calls another routine multiple times using various pointers from a structure before updating that table entry with a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_byte`: It clears a byte in memory by writing zero through a pointer, then returns without touching any other data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_call_function`: Zeros out a block of memory, updates a global variable and a table entry, and then calls a function pointer based on a bit test in a global variable.
- `process_narrator_device_status`: Checks a global variable and calls several routines to update status values and potentially recurse based on the results.
- `traverse_and_call_pointers`: Checks a memory location and calls a routine if a certain value is non-zero, then recursively calls itself and calls other routines based on pointer checks.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `update_memory_from_pointer`: Calls another routine and copies a value from a pointer-based address into a destination, while zeroing out other memory locations.
- `execute_conditional_logic_chain`: Performs a series of conditional checks on a global pointer, calling various subroutines, and updating memory or configuration tables based on certain values.
- `pass_through_routine`: This routine calls another routine with the same arguments and then returns.
- `read_input_flags`: The routine returns a byte derived from a global variable, testing bits 64 and 4 and combining the results with a logical OR. It references the CIAA_PRA and POTGOR hardware registers.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `set_global_constant`: Stores a constant value into a global variable.
- `update_and_scale_value`: Checks a value, multiplies a global variable by two, applies an XOR mask if a carry or zero condition occurs, and returns a value involving a division and remainder calculation.
- `handle_keyboard_serial`: The routine returns a byte computed from the bitwise complement of a global variable when a flag bit is set, performs a decrementing delay loop, and clears a flag bit in the global variable.
- `populate_table_with_offsets`: Copies a block of data from a source table and populates a destination table with a series of calculated values or offsets derived from input parameters.
- `write_config_table`: Stores a series of hardcoded word values into a table located via a pointer in a global variable.
- `write_config_table_2`: Stores a series of hardcoded word and long values into a table located via a pointer in a global variable.
- `load_pointer_from_table`: Returns a value loaded from a memory address calculated from a global variable.
- `update_parameter_tables`: Updates a global pointer and populates a destination table with several derived word values based on two input parameters.
- `copy_source_to_dest`: Copies a sequence of words from a source buffer to a destination buffer located in a table.
- `return_from_subroutine_2`: This routine performs no operations and immediately returns to the caller.
- `return_immediately_4`: The routine performs no operations and returns.
- `update_display_raster_parameters`: Updates a table with new offset and bitmask values by calling a routine that waits for a specific raster line and then performs a word-based memory copy.
- `execute_nested_calls`: Performs a sequence of calls to another routine using values passed or stored on the stack.
- `get_and_increment_index`: Reads a byte from a table using an index, then increments that index stored in a pointer.
- `initialize_and_fill_table`: Clears a block of memory, then iterates through a table to clear or update entries with values retrieved from a recursive call or other routine.
- `retrieve_nested_short_value`: Traverses a pointer-based structure to retrieve a short value from a nested offset if a global variable is non-zero.
- `traverse_linked_list_node`: Checks a global variable and, if non-zero, traverses a pointer-based structure to retrieve a short value from a nested offset.
- `fetch_and_process_value`: Calls another routine with a global variable and a constant, then retrieves a value from a pointer located at an offset from a table entry.
- `process_input_and_values`: A routine consisting of 43 instructions that calls another routine to process a global variable and retrieves a byte derived from a global variable while testing specific bits.
- `read_input_flags_2`: Returns a byte derived from a global variable while testing bits 64 and 4 and combining the results with a logical OR.
- `initialize_game_state_tables`: Initializes a series of global pointers and clears or populates memory-resident tables, including a loop that zeros out a large block of memory.
- `reset_memory_values`: Sets a 4-byte memory location to zero and a 2-byte memory location to all ones.
- `traverse_and_reset_list`: Iterates through a linked list, calling another routine for each node, and then sets a 4-byte memory location to zero and a 2-byte memory location to all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_linked_list`: Recursively calls itself using a pointer from a table and updates a global variable before setting a memory location to a maximum value.
- `validate_table_entries`: Iterates through a range of data, calling another routine to populate a buffer and then checking a table for a specific count of valid entries.
- `search_table_for_matching_value`: Iterates through a table to find an entry where a specific part of a value matches, then recursively checks a sub-range or linked structure.
- `search_and_recurse_table`: Iterates through a table of structures to find a matching key, then recursively calls itself or another routine on a calculated range of values if a match is found.
- `search_and_recurse_table_2`: Iterates through a table of structures to find a matching key, then recursively calls itself or another routine on a calculated range of values.
- `execute_recursive_operations`: Calls another routine with a fixed value, and if successful, performs several recursive calls involving calculated or passed parameters.
- `decompress_bitstream_data`: Decodes a variable-length bitstream to reconstruct and store byte values into a destination memory buffer.
- `decode_bitstream_data`: Processes a bitstream by shifting bits and calling another routine to populate a buffer through a pointer.
- `unpack_bitstream_to_memory`: Extracts variable-length bit sequences from a bitstream to reconstruct and store byte values into a destination memory buffer.
- `unpack_and_fill_memory`: Uses a bit reader to extract values and fills a memory range with values derived from a register within a loop.
- `fill_memory_loop`: Fills a range of memory with a value derived from a register using a loop.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `return_uninitialized_stack_value`: This routine returns an uninitialized value from the stack.
- `search_and_clear_table`: Iterates through a table of structures to find matching keys, calls another routine with the found data, and then clears the entries in the table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `handle_disk_error_and_loop`: Updates a global variable with a passed value and enters an infinite loop that repeatedly calls several subroutines and a function to manage disk-related error states.
- `infinite_loop_with_function_call`: Executes a loop that calls itself and another routine repeatedly without a termination condition.
- `update_graphics_parameters`: Performs several routine calls and calls a function with calculated offsets and a fixed value, likely intended for a display or certain hardware parameters.
- `calculate_text_box_dimensions`: Iterates through a null-terminated string to determine the maximum line length by counting characters between vertical bar delimiters.
- `parse_and_render_text`: Iterates through a null-terminated string to calculate text dimensions and layout.
- `render_wrapped_text_string`: Calculates line breaks and character positioning for a space-delimited string within a defined bounding box.
- `parse_and_render_text_string`: Parses a space-delimited string of characters to calculate text positioning and line wrapping.
- `process_sprite_animation_data`: Iterates through a data buffer to update sprite frame indices and animation counters.
- `update_sprite_animation_frames`: Iterates through a sprite data table to update frame indices and animation counters.
- `update_sprite_animation_state`: Iterates through a sprite data table to update frame indices and animation counters.
- `process_sprite_table_and_render`: Iterates through a table to retrieve sprite data and performs coordinate calculations.
- `process_text_input_buffer`: Validates alphanumeric characters and control keys to populate a character buffer.
- `process_recursive_input_string`: Validates and stores alphanumeric characters into a buffer while recursively handling input through a loop.
- `process_input_buffer`: Reads character values from a source and stores them into a destination buffer while performing recursive updates.
- `execute_recursive_calls`: This routine performs two consecutive calls to itself before returning.
- `update_position_and_buffer`: Calculates a difference between a table value and a global variable, updates a global offset, and copies a sequence of 16 words from a source table to a destination buffer.
- `copy_cstring`: Copies a null-terminated byte string from a source pointer to a destination pointer, byte by byte, and terminates the destination with a zero byte.
- `search_string_table`: Iterates through a table of pointers to strings and uses string comparison to find a match for a provided string.
- `find_trackdisk_device`: Calls the exec library to locate the trackdisk.device and extracts a value from a device structure, returning it combined with an input register.
- `loop_and_call_indirect`: Locates a trackdisk device and enters a loop that performs indirect calls to functions retrieved from a table until a specific bit is cleared or a counter limit is reached.
- `access_file_data`: Attempts to lock a file and examine its contents to retrieve data from a specific offset into a pointer.
- `reinitialize_string_structure`: Resets a string structure's metadata, sets certain flags, and writes a constant string or a new pointer back to a global structure.
- `sync_sprite_data`: Compares a string against a stored value and, if they match, traverses a linked list of pointers to populate a new data structure.
- `compare_and_allocate_string_list`: Compares two null-terminated strings and, if they match, calculates the size of a linked list of pointers.
- `execute_indirect_calls`: Performs indirect calls to a target address retrieved from a table using a negative offset and calls a library function via a global base.
- `copy_null_terminated_string`: Copies bytes from a source memory location to a destination memory location until a null terminator is encountered.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `compare_two_byte_strings`: Compares two byte strings byte by byte.
- `safe_memory_copy`: Copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `clear_and_update_globals`: Zeroes out a block of memory, updates a table entry with a constant, and copies a global variable to a new location.
- `return_immediately_2`: This routine performs no operations and returns to its caller immediately.
- `parse_command_string`: Parses a string into individual tokens by handling whitespace, quoted substrings, and escape sequences.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `load_and_store_window_data`: Opens a file and uses icon.library to retrieve data from a nested structure, subsequently storing several values into a provided buffer.
- `store_parameter_to_memory`: The routine stores a value to a memory location calculated from a register and then returns.
- `store_and_parse_string`: Stores a passed parameter into a global variable and then calls a routine to parse a format string.
- `increment_and_store_byte`: Loads a pointer from a global variable, increments that global variable, and stores a byte into the memory location pointed to by the loaded pointer.
- `recurse_with_concatenation`: Recursively calls itself while returning a 64-bit value formed by concatenating a register's previous contents with the new return value.
- `parse_format_string`: Parses a formatted string containing specifiers for integers, floating-point numbers, and strings.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls itself and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `execute_conditional_pointer`: Checks if a value at a certain offset from a pointer is non-zero and, if so, executes the function pointer located at that memory location.
- `execute_conditional_table_logic`: Checks various memory locations or table-based pointers and executes corresponding functions or jumps if the pointers or certain flags are non-null.
- `return_immediately_3`: This routine performs no operations and immediately returns to its caller.
- `execute_indirect_call_with_validation`: The routine validates an index against a range and checks a pointer in a table; if valid, it performs an indirect call and clears the pointer, otherwise it sets a global status and returns an error code.
- `execute_indirect_call_offset_24`: The routine performs an indirect call to a function address retrieved from a table using a specific offset.
- `execute_indirect_call`: Performs an indirect call to a function address retrieved from a structure via a pointer.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_indirect_pointer`: Retrieves a function pointer from a nested table structure and executes it.
- `get_current_directory`: Calls a function to retrieve the current directory.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `read_input`: Calls a function to perform an input operation.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `open_file`: Calls a function to open a file.
- `write_output`: Calls a function to perform an output operation.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `seek_file`: Calls a function to seek within a file.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `unlock_file`: Calls a function to unlock a file.
- `write_data`: Calls a function to write data.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_indirect_function`: Loads a function pointer from a table offset and calls it.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested table structure and executes it.
- `call_library_function`: Calls a function from a library using a specific offset.
- `process_recursive_data`: Recursively processes a data structure by populating a new structure with parameters and calling another routine to handle the result.
- `process_recursive_data_structure`: Recursively processes data by copying bytes from one location to another and updating a structure based on the result of a recursive call.
- `copy_and_recurse_data`: Copies bytes from one memory location to another, performs a recursive call, and then conditionally calls another routine based on a value in a table.
- `perform_recursive_offset_call`: The routine calls itself with an argument incremented by a constant offset.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_3`: Retrieves a pointer from a table located at an offset from a base address and executes the function at that location.
- `call_library_function_alt`: Calls a function from a library using a different offset.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_indirect_offset_pointer`: Performs an indirect call to a routine by dereferencing a pointer located at a specific negative offset from a table entry.
- `call_indirect_offset_c78_minus_126`: Calls a routine by dereferencing a pointer located at an offset from a base address and adding a negative offset.
- `call_indirect_offset_pointer_2`: Executes an indirect call using a routine address retrieved from a table via a specific offset.
- `call_indirect_offset_c78_minus_84`: Performs an indirect call to a routine using a pointer retrieved from a table at a specific offset.
- `call_indirect_offset_pointer_3`: Calls a routine indirectly by accessing a pointer through a table and applying a negative offset.
- `call_indirect_offset_c78_minus_d2`: Executes an indirect call to a routine based on a pointer stored in a table.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_indirect_offset_c78_minus_174`: Calls a routine through a pointer found at a specific offset within a table structure.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_indirect_offset_pointer_4`: Performs an indirect call to another routine using a pointer found at a specific offset within a table.
- `call_indirect_offset_c78_minus_228`: Performs an indirect call to another routine using a pointer from a table.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_indirect_offset_c78_minus_17a`: Calls a routine indirectly by calculating an address from a table and a negative offset.
