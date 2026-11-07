# Game 414 (Operation Stealth): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00414). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2735808 top-level bytes in 55 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 21 of 21 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 555 of 555 named (0 author symbols), 555 of 555 described (57 by rule, 146 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1670 of 1670 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9545 of 9546 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2361160 |
| code | 30678 |
| data | 281148 |
| padding | 12134 |
| unused | 50688 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| NARRATOR.DEVICE | 3 | 3782 | 268 |
| Message | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:endcli | 2 | 117 | 2 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:intro | 2 | 174 | 12 |
| TRANSLATOR.LIBRARY | 2 | 490 | 50 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:PAUSE | 2 | 3 | 1 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:delphine | 2 | 207 | 8 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:add21k | 3 | 637 | 38 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 2 of 3).adf:PPMORE | 2 | 209 | 8 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 2 of 3).adf:message | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 3 of 3).adf:paradox | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 2 of 3).adf:bootblock | 1 | 10 | 1 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 3 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1030 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 413 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2375 | 100 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 407 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 130 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 904 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- operation stealth (1990)(u. s. gold)[cr pdx](disk 1 of 3)/Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:delphine: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- operation stealth (1990)(u. s. gold)[cr pdx](disk 2 of 3)/Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 2 of 3).adf:PPMORE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:intro

## Copy protection found

- none found

## Functions


### NARRATOR.DEVICE

- `init_narrator_device_base`: Creates the device base with MakeLibrary, allocates a message port with a signal bit and the owning task, then registers the device and its task with exec.
- `open_narrator_unit_request`: Rejects a nonzero unit number with an error code, and for unit zero increments the open count and fills in the request when the unit is already open.
- `open_audio_device_for_unit`: When the unit is idle, opens the audio device and allocates a request block with default speech settings; when the unit is already open it increments the count and fills in the request.
- `allocate_narrator_request_block`: Allocates a request block through exec and, on success, stores it, increments the open count and fills in the default speech settings.
- `store_unit_and_fill_defaults`: Stores the given value in the unit structure, increments its counters, and writes default speech settings into the request: rate 150, pitch 110, volume 64 and a sampling value of 22200.
- `finish_request_with_status`: Stores a status byte in the request and sets its completion marker to minus one.
- `queue_or_complete_device_request`: With interrupts disabled, either completes the request immediately when its unit is idle or inserts it into the unit's pending list.
- `insert_request_into_unit_list`: Inserts a request into one of the unit's queues with interrupts disabled.
- `complete_or_queue_unit_request`: When the unit is not busy and the request's status differs, copies the unit's status into the request and calls a helper.
- `copy_byte_and_split_nibbles`: Copies a status byte from one structure to another, splits it into low and high nibbles stored in two bytes, clears the request's flag byte, and calls a helper.
- `validate_speech_request_fields`: Searches the request list for an entry belonging to the unit and checks its speech settings: rate from 1 to 400, pitch from 65 to 320, sampling value from 5000 to 28000, volume from 0 to 64, and two flags of 0 or 1.
- `validate_speech_params_after_search`: Performs the same speech setting checks and copies as its neighbour, starting from the current request rather than searching the list.
- `release_unit_when_idle`: Scans the request list for entries that belong to the unit and runs a helper on each match.
- `decrement_counter_and_close`: The routine decrements a counter in a structure and, when it reaches zero, calls library functions via a base pointer and optionally calls another routine.
- `dispatch_narrator_command`: Dispatches the request to one of eight command handlers, chosen by the request's command number from a table in the device.
- `unlink_free_request_or_flag`: If the request's counter is zero, unlinks it from its list, frees it and a second block through exec, and returns the value stored in the request's final field.
- `unlink_free_and_return_field`: Unlinks the request from its list, frees it and a second block through exec, and returns the value stored in the request's final field.
- `handle_narrator_command_switch`: Handles a narrator request according to its type: types 6, 7, 8 and 1 set or clear device flags around library calls, and type 2 handles a speech request.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `set_mode_seven_run_library`: Clears a flag bit in the request, sets the unit's mode to seven with a pending flag, calls two library routines, clears the request status and calls a helper.
- `set_mode_six_run_library`: Sets a flag bit in the request, sets the unit's mode to six with a pending flag, calls a library routine, clears the request status and calls a helper.
- `set_flag_clear_around_calls`: Sets bit 7 in a byte of the request structure, calls two helpers, then clears the same bit.
- `set_flag_clear_status_call`: Sets bit 7 in a byte of the first structure, calls a helper, clears that bit, sets the status byte of the second structure to zero, and calls another helper.
- `scan_list_with_interrupts_disabled`: Optionally runs a helper, then with interrupts disabled walks a list in the structure and runs a helper on each entry, re-enabling interrupts afterwards.
- `decrement_counter_and_set_global`: Decrements the interrupt nesting count in the global area and, if the previous value was less than one, writes the enable value to the interrupt control register.
- `remove_request_from_list_disabled`: Removes a request from the unit's list with interrupts disabled: if it is the current entry it is only marked busy, otherwise it is searched for and unlinked with status 254.
- `search_and_unlink_request`: Searches a list for the request and, if found, unlinks it and sets its status to 254.
- `unlink_known_request_status`: Unlinks a request already known to be in its list, sets its status to 254, decrements the interrupt nesting count and calls a helper.
- `decrement_nesting_and_enable`: Decrements the interrupt nesting count and re-enables interrupts when the previous value was less than one.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `complete_request_with_status_249`: Sets the request's status byte to 249 and calls a helper.
- `clear_request_status_and_call`: Clears the request's status byte to zero and calls a helper.
- `clear_flags_and_call_handler`: Clears a flag bit in one structure, clears a byte in another, and calls a handler through the library base.
- `validate_speech_params_and_split`: Checks the speech settings in the request against the rate, pitch, sampling, volume and flag ranges, copying valid values into the unit state, then scans the text for word breaks.
- `validate_speech_rate_and_copy`: Stores the rate from the data register into the unit state when it is below 401, then validates pitch, two flags, sampling value and volume and copies them.
- `validate_speech_pitch_and_copy`: Stores the pitch from the data register when it is below 321, then validates the two flags, sampling value and volume and copies them.
- `store_speech_mode_then_validate`: Stores the first flag from the data register into the unit state, then validates the second flag, sampling value and volume before copying them.
- `store_speech_flag_then_validate`: Stores the second flag from the data register, then validates the sampling value and volume before copying them.
- `validate_speech_sampling_and_copy`: Stores the sampling value from the data register when it is below 28001, then validates the volume, copies the speech values and scans the text for word breaks.
- `store_speech_volume_and_scan`: Stores the volume from the data register when it is 64 or less, then copies the speech values and scans the text for word breaks.
- `set_task_priority_free_buffer`: Changes the task priority through exec around a find-task lookup, frees a buffer through exec when a flag is set, and completes the request with its status when a trailing value is nonzero.
- `copy_counter_and_set_status`: Copies a counter from the request into the unit's counter field, stores the status byte and calls a helper.
- `clear_counter_and_set_status`: Clears the request's counter field, stores the status byte and calls a helper.
- `continue_text_break_scan`: Continues the backward scan for a comma, dash or space break point in the speech text, records the break position, and then either finishes the request or moves on to the next segment.
- `scan_text_for_space_break`: Looks backwards through the speech text for a space to use as the break point, records it in the unit state, and then either finishes the request or continues the segment chain.
- `record_text_break_position`: Records the length-based break position in the unit state, then scans the text for a break point and either continues the segment chain or finishes the request.
- `initialise_speech_port_and_table`: Clears several unit fields, opens a library through the DOS base, allocates a message port through exec, and copies a fixed table of 34 words into a buffer.
- `allocate_speech_port_and_table`: Allocates a message port through exec, stores its pointer, and copies a fixed table into a newly allocated buffer for the speech mode.
- `create_speech_iorb_and_table`: Stores a supplied port in the unit, allocates a signal and task pair for it through exec, and copies a 34-word table for the speech mode into a new buffer.
- `allocate_speech_table_buffer`: Stores the speech mode byte, allocates a buffer through exec, and copies a 34-word table from the unit into it.
- `copy_word_table_to_buffer`: Stores the buffer pointer in the unit, then copies a 34-word table from the unit into the buffer, writing each word to two places per iteration for a count taken from a mode byte.
- `update_speech_iorb`: Stores a byte and a word into a structure, calls three different function pointers retrieved from a global variable based on specific conditions, and clears a word at a fixed offset.
- `send_io_and_free_resources`: Sets the unit's mode to nine and sends the request with DoIO through exec, then frees the table buffer and signal through FreeMem and FreeSignal when their pointers are set.
- `tokenize_phoneme_text`: Walks the speech text matching one- or two-character symbols against a 112-entry phoneme table in the device, recording each matched symbol and its attribute bytes in three arrays in the unit state.
- `scan_phoneme_text_unchecked`: Initialises the three phoneme attribute arrays in the unit state and scans the speech text for phoneme symbols against the 112-entry table, without the early exit a zero-length check would give.
- `continue_phoneme_scan_at_match`: Continues the phoneme scan after a symbol has matched, recording the symbol and applying its attribute bits, then advances through the text until a terminator or a limit is reached.
- `continue_phoneme_scan_limited`: Continues the phoneme scan from the current position and stops early when the text pointer reaches the recorded limit, recording matched symbols the same way as its neighbours.
- `continue_phoneme_scan_at_symbol`: Records the current symbol and its attribute bits, then keeps scanning the text for further phoneme symbols until a terminator or the table limit is reached.
- `apply_phoneme_attribute_and_scan`: Applies a stress or attribute bit to the most recent phoneme entry, then continues scanning the text for phoneme symbols until a terminator is reached.
- `mark_phoneme_flag_and_scan`: Sets bit 4 in the attribute byte of the most recent phoneme entry when the symbol is the sixth type, then continues scanning the phoneme text until a terminator.
- `store_phoneme_symbol_and_scan`: Stores the current symbol in the phoneme list at the current index, advances the index, and continues scanning the text until a terminator or the buffer limit is reached.
- `scan_phoneme_text_from_position`: Checks the remaining text against the recorded limit, then matches the next phoneme symbol against the table and keeps recording symbols until a terminator or the buffer limit is reached.
- `stub_empty_return_2`: Contains only a return instruction, so calling it has no effect.
- `terminate_phoneme_arrays`: Writes end markers into the three phoneme attribute arrays at the current index, stores the scanned length in the unit state, and returns the matched value and text position.
- `pad_and_terminate_phoneme_list`: Pads the phoneme list with a default entry when an attribute check passes, writes end markers into the three arrays, and records the scanned length.
- `record_phoneme_count`: Stores the phoneme count from the data register into the unit state.
- `mark_stressed_phoneme_neighbours`: Walks the phoneme list and, for each symbol whose class-table flags mark it as stressed, sets the flag bit on the one or two entries before it.
- `mark_stress_neighbours_loop`: Performs the same neighbour-flag pass over the phoneme list as its neighbour, returning at the end marker.
- `mark_stress_neighbours_from_index`: Applies the neighbour-flag pass to the phoneme list starting from the index in the data register, and returns at the end marker.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `apply_phoneme_flag_rules`: Walks the phoneme list and applies per-symbol rules, setting bit 7 on the symbols it handles and propagating attribute bits to the entries of the preceding symbol.
- `propagate_phoneme_attribute_bits`: Continues the phoneme rule pass by copying the attribute bits of the preceding symbol onto the current entries and marking group flags, until the end marker.
- `apply_phoneme_group_flags`: Continues the phoneme rule pass for the next symbol group, setting group flags on the entries it covers until the end marker.
- `continue_phoneme_rule_pass`: Continues the phoneme rule pass from the current group, setting attribute bits on the entries it covers until the end marker.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `match_phoneme_rule_pattern`: Scans a table of phoneme rules, matching the three characters at the current position against each rule's pattern where 255 acts as a wildcard.
- `continue_phoneme_rule_match`: Continues the phoneme rule-table matching by testing the next pattern's characters against the text, and applies replacement bytes on a full match, until the end marker is reached.
- `advance_phoneme_rule_match`: Advances to the next table rule and tests its pattern against the text, applying replacement bytes when every character matches, until the end marker.
- `scan_phoneme_rule_table_step`: Steps through the phoneme rule table and applies the rule whose pattern matches at the current position, returning when the end marker is found.
- `test_bit_with_table_lookup`: Repeatedly looks up a word in a table indexed by a value, adjusts it according to bit flags in a byte of the input, and stops when the chosen bit is set in the result.
- `insert_phoneme_entry_bounded`: When the phoneme count is below 512, increments the count and shifts the three phoneme arrays up by one entry from the current position.
- `shift_bytes_and_clear_fields`: Increments the phoneme count, shifts the three arrays up by one entry from the computed position, then writes a byte from a register and two zero bytes at the new ends.
- `call_seven_helpers_return_zero`: Calls seven helpers in sequence and returns zero.
- `clear_output_entry_flags`: Walks an array of eight-byte entries starting from a pointer held in the device state, clearing the flag byte of each entry until a minus-one terminator.
- `clear_entries_until_match`: Steps through eight-byte entries from a pointer register, clearing each entry's flag byte until the entry's first byte equals a value passed in a data register.
- `expand_speech_parameter_stream`: Reads a code stream ending in a minus-one byte.
- `expand_speech_codes_register_entry`: Second entry point of the same code-stream expander, taking its table and output pointers from address registers.
- `emit_speech_code_values`: Variant of the code-stream expander that starts at the table lookup for the current code.
- `emit_table_value_stream`: Writes the table value selected by the current code into the output array and sets flag bits from the following code.
- `apply_nibble_flags_stream`: Sets the low flag nibble of each output entry from bits of the current code, choosing among a few fixed values.
- `apply_flag_bit_stream`: Sets the low flag nibble of each output entry from the code's flag bits and copies smoothed table values across the stream.
- `apply_lower_flag_nibble`: Sets the low flag nibble of the current output entry to one of four small values depending on bits of the next code.
- `copy_smoothed_speech_values`: Copies table values into the output parameter arrays for each code in the stream, averaging neighbouring bytes for code 23 and writing flag nibbles.
- `allocate_speech_output_buffers`: Sums the duration entries of a table up to its terminator, storing the total and a size of eight bytes per unit plus eight.
- `allocate_sized_output_buffer`: Stores the frame count, computes a buffer size of eight bytes per frame plus eight, and requests that memory from the system.
- `store_buffer_then_allocate_extra`: Stores the passed pointer as the primary buffer.
- `render_speech_token_frames`: Iterates over the token list until its terminator, looking up per-token values in several tables and adjusting them when a flag bit is set.
- `write_token_frames_buffer`: For each token, writes eight-byte frame records into the output buffer, filling the fields from lookup tables and flag bits.
- `write_frame_records_terminated`: Expands each token into eight-byte frame records using its duration, copying three lookup values, three computed bytes and a flag byte per frame.
- `interpolate_between_frames`: Walks the frame records and, for tokens carrying the blend flags, moves the stored parameter bytes toward the next token's values in six steps using a weighted average.
- `blend_adjacent_frame_values`: Register-based variant of the frame blending pass.
- `blend_frame_record_transitions`: Blends parameter bytes across token boundaries in the frame records, weighting them with a lookup table.
- `blend_frame_transition_steps`: Performs six-step weighted blending between each pair of adjacent frame parameter values, advancing through the records until the terminator.
- `blend_frame_values_next_token`: Continues the frame blending pass, moving to the next token whenever the current duration runs out.
- `build_token_ramp_records`: For each token, computes a per-frame ramp of values between neighbouring table entries over the token's duration.
- `write_ramp_records_alt`: Register-based variant of the ramp builder.
- `emit_ramp_records_until_end`: Continues writing ramp records for each token, reading the next code and its duration, until the terminator.
- `emit_ramp_records_stepping`: Steps through the token list writing four-byte ramp records and updating the running count for each frame.
- `emit_ramp_records_padded`: Writes ramp records for the current token and then pads the remaining slots of the frame with a fixed constant when the segment flag is clear.
- `emit_ramp_records_branching`: Writes ramp records for each token, choosing between two flag-derived values depending on bits of the current code's table entry before continuing to the next token.
- `fill_ramp_record_segment`: Fills a run of four-byte ramp records with the computed values for the current token segment, using the segment length counter to end each run.
- `fill_zero_ramp_records`: Writes zeroed four-byte records followed by a fixed pad value for the remaining frames of the current token segment, then continues with the next token.
- `fill_ramp_records_counted`: Writes counted runs of four-byte ramp records for each token segment, computing the step values from neighbouring table entries until the terminator is reached.
- `emit_ramp_records_tail`: Finishes writing ramp records for the last token segment, computing the step values from neighbouring table entries and advancing the output pointer until the terminator.
- `fill_ramp_records_with_flags`: Writes runs of four-byte records whose length comes from a count in a data register, filling each record from table values and flag bits before moving to the next segment.
- `filter_token_list_entries`: Copies the token list and its two parallel arrays into compacted positions, dropping tokens whose table entry has a particular flag bit set.
- `write_byte_to_three_pointers`: Stores one byte from a data register into the three memory locations pointed to by three address registers.
- `call_two_routines_in_sequence`: Calls two other routines one after the other and returns.
- `set_token_flag_bits`: Clears the low nibble of each flag entry, then walks the token list setting bit flags on the flag entries for each token and its group boundaries.
- `mark_token_boundary_flags`: Walks the token list and sets marker bits on the flag entries that bracket each token's duration.
- `set_token_boundary_markers`: Register-based variant of the boundary marker pass.
- `set_boundary_markers_midloop`: Continues the boundary marker pass from the middle of the loop.
- `set_boundary_markers_alt`: Alternate form of the boundary marker pass that sets the marker bits one entry after each token boundary and returns at the terminator.
- `set_boundary_markers_entry`: Enters the boundary marker pass by setting a marker bit on the current flag entry, then walks the token list setting bits on the flag entries that bracket each token until the terminator.
- `set_boundary_markers_restart`: Restarts the boundary marker pass at its second marker position, setting the same marker bits on the flag entries around each token until the terminator.
- `compute_period_frame_records`: Computes a period by dividing a fixed constant by a stored rate, then walks the token tables.
- `write_interpolated_period_records`: Register-based variant of the period record writer.
- `write_period_records_loop`: Loops over tokens writing period-divided bytes into the frame records, weighting each step between neighbouring pitch values by the token's flag bits until the terminator.
- `finish_period_records_tail`: Writes the final period-divided byte of the frame records and returns once the terminator is reached, using the last table entry as the divisor.
- `write_period_records_branch`: Writes period-divided bytes for each token segment, choosing between two interpolation paths depending on a flag bit in the current code's table entry.
- `write_period_records_segment`: Writes period-divided bytes for the current token segment and advances to the next token, interpolating neighbouring pitch values until the segment flag clears.
- `write_scaled_period_bytes`: Divides a stored rate by a fixed divisor and writes the result into the seventh byte of each eight-byte record until the terminator.
- `stub_empty_return`: Empty routine whose body only returns to its caller.
- `dispatch_token_rule_handlers`: Walks the token list until its terminator, testing flag bits of each token and its neighbours, and calls the matching handler routines to adjust the current segment.
- `run_token_rule_checks`: Register-based form of the token rule pass.
- `step_token_rule_checks`: Advances through the token list one token at a time, testing the same flag bits and calling handler routines for each matching token until the terminator.
- `apply_token_rule_steps`: Applies the token rule handlers in sequence for the current token, testing flag bits of the token and its neighbours before advancing the output and input pointers.
- `check_neighbour_token_flags`: Compares the flags of the tokens before and after the current one and calls handler routines depending on which carry the relevant bits.
- `check_neighbour_token_flags_alt`: Second entry into the neighbour flag check.
- `check_neighbour_token_pair`: Checks the flag pair formed by the current token and its successor, calling handler routines for each combination of bits that is set.
- `scale_neighbour_parameter_value`: Looks up a parameter for the current token, halving it when the relevant flag bits are clear, and passes control to the neighbour handler routines.
- `apply_neighbour_rule_checks`: Runs the neighbour rule checks for the current token, calling handler routines when the preceding token carries the relevant bits.
- `blend_token_parameter_values`: Computes a weighted blend between a stored parameter and the table value for the current token.
- `blend_parameter_stream_loop`: Loops over a stream of codes, computing blended parameter bytes from the table value and the previous byte.
- `blend_parameter_stream_alt`: Alternate entry into the parameter blend loop, with the same table lookups, clamping to 63 and merging into the output records until the terminator.
- `compute_scaled_shifted_product`: Multiplies the low word of one register by a word input and returns a pair of values: the high half of the product and the low half after adding sixteen and shifting right five bits.
- `initialize_and_clear_buffers`: Stores the offsets of the speech tables and buffers into the device state structure, advances several of those pointers and clears a set of word fields.
- `advance_counter_dispatch_calls`: Increments a 16-bit counter in the device state and calls one unnamed routine.
- `encode_frame_level_nibbles`: Walks the code and parameter streams, scaling each parameter by a fixed fraction and writing it as a low nibble into the frame buffer.
- `write_frame_level_nibbles_loop`: Register-based loop of the frame nibble writer.
- `write_frame_nibbles_tail`: Writes the last scaled nibble of the current frame run into the frame buffer and updates the frame counters, returning once 128 entries have been written.
- `set_flag_and_store_indices`: Sets a flag bit in the frame byte before the current index, then stores the current index and, when no marker was recorded, a fallback index into two fields of the device state.
- `mark_frame_flags_by_code`: Walks the code and table streams, setting flag bits in the frame buffer according to each code's table entry and whether a break code occurs.
- `mark_stream_flags_loop`: Walks the code and parameter streams, adding to a counter on negative parameter bytes and setting marker bits in the flag buffer for code 4 and for flagged positions.
- `mark_stream_flags_alt`: Alternate entry into the stream flag marking loop, with the same counter updates and marker bits for code 4 until the terminator.
- `propagate_marker_flags_backward`: Walks the flag buffer backwards from the current index, OR-ing marker bits.
- `scan_marker_flags_downward`: Scans the flag buffer downward from the current index, OR-ing a stored bit into each entry.
- `step_marker_flags_neighbours`: Steps through the flag buffer one neighbour at a time, applying the marker bit and value copies for codes 2 and 14 until the index passes the end of the buffer.
- `swap_and_clear_byte_flags`: Copies a value between two positions in the table, clearing the source when the positions differ.
- `set_low_bit_flags_backward`: Sets the lowest bit of each byte in a flag buffer, counting down from the current count to zero.
- `set_low_bit_flags_countdown`: Sets bit one in each byte of a buffer, counting down from the given index to zero.
- `or_mask_buffer_early_exit`: Chooses a mask from the previous code, four for code 1 and eight for code 2, and ORs it into the flag buffer going downward.
- `set_marker_mask_and_clear`: Writes a marker mask into the flag entry for the current position, choosing between two masks depending on the previous code.
- `accumulate_table_entries`: Calls a sequence of unnamed routines depending on two state fields, then adds a stored word to each of eight consecutive long entries in the table at the state's table offset.
- `compute_clamped_level_byte`: Computes a scaled level from two inputs, clamps it to a narrow range and stores it as a byte in a table indexed by a stored position.
- `clamp_value_and_store_char`: Clamps a word value into a fixed lower and upper bound and stores the result as a byte at an offset from a table pointer.
- `step_levels_across_segment`: Divides the difference between a base value and a stored input by a count, then steps a running level across the flagged entries of a segment, writing each result into the output table.
- `adjust_table_values_loop`: Computes the absolute difference between two table entries, scales it and adds it to a destination byte.
- `compute_flagged_entry_levels`: For each flagged entry from the current index, computes scaled level and secondary values from the entry's stored parameters and writes them into two output tables.
- `scale_and_back_scan_entries`: Scales values in two output tables for each entry, then scans backwards through the flagged entries writing adjusted values until the start of the range is reached.
- `compute_scaled_char_values`: Loops backwards over the flagged entries, computing scaled values from two tables with multiplications and shifts and writing them to two destination tables.
- `adjust_gap_scaled_values`: Scans backwards for flagged entries and measures the gap to the previous flagged entry.
- `adjust_gap_scaled_values_alt`: Register-based variant of the gap adjustment.
- `adjust_pair_values_rounded`: Applies rounded, scaled updates to two pairs of table entries selected by the current indices, with a difference-based correction feeding the output table.
- `adjust_pair_values_partial`: Applies rounded, scaled updates to one pair of table entries and a correction derived from their difference, then continues the flagged-entry walk with the same arithmetic.
- `adjust_pair_values_backscan`: Applies rounded, scaled updates to a pair of entries, then scans backwards over flagged entries applying further scaled adjustments to those whose mask bits match.
- `adjust_pair_values_masked`: Applies a rounded scaled update to a pair of entries and then runs the masked backward scan over flagged entries, copying values into the output table where the mask allows.
- `scan_running_maximum_values`: Adds a byte offset to one table entry, then scans backwards over the earlier entries keeping a running maximum.
- `scale_pair_by_mode_factor`: Chooses one of three scale factors from a mode value, then applies rounded scaled updates to a pair of entries in one table and a pair in another.
- `scale_pair_by_argument_flag`: Applies rounded scaled updates to a pair of table entries, choosing the scale factor and sign from an argument flag, then continues the flagged-entry scan.
- `apply_flagged_scaled_tail`: Applies rounded scaled updates to a pair of entries selected by the argument, then scans backwards over flagged entries copying values into the output table where the bit mask allows.
- `apply_pair_scaled_then_scan`: Applies a rounded scaled update to a pair of entries, adjusts a third entry when the argument and flag bits allow, and then scans backwards over flagged entries.
- `scan_flagged_pairs_backward`: Scans backwards through the flagged entries, applying rounded scaled updates to pairs of entries and copying mask-selected values between the output tables.
- `scale_entry_by_running_maximum`: Adds a scaled correction to one entry, takes the maximum over the earlier entries of a table, and subtracts a scaled version of that maximum from the output.
- `scale_entry_by_maximum_alt`: Register-based variant of the running-maximum scaling.
- `update_maximum_scaled_entries`: Tracks a running maximum over table entries while walking flagged positions.
- `scale_nibble_flagged_entries`: Walks the entries backwards.
- `scale_nibble_flagged_entries_alt`: Register-based variant of the nibble-flagged scaling.
- `scale_buffer_by_signed_factor`: Walks a byte range backwards, skipping entries whose high nibble is zero.
- `compute_flagged_differences_tables`: Stores the differences between a level table and two reference tables at each flagged position, then walks the flagged positions computing pairwise differences for the next adjustment pass.
- `shift_and_copy_flagged_bytes`: Shifts a run of bytes one position forward between two buffers, copying the start value and adjusting the moved entries by a fixed step.
- `copy_run_with_nibble_scaling`: Dispatches on a run length of one or two to an unnamed routine.
- `copy_run_nibble_scaled_loop`: Copies a counted run of bytes forward between buffers, adding twice the low nibble of a flag entry to each copied output byte, then walks on to the next flagged position.
- `smooth_run_pair_differences`: Calls an unnamed routine, then walks the flagged positions.
- `smooth_run_pair_differences_alt`: Alternate entry into the run smoothing pass.
- `spread_run_quotient_levels`: Fills the output for the current run with a constant level and nibble-scaled values.
- `spread_run_quotient_levels_loop`: Loop form of the run level spreading.
- `spread_run_step_values`: Finds the run start from the flag entries and divides the level difference by the run length.
- `spread_run_maximum_levels`: Copies and steps values across a run by a fixed step.
- `write_run_maximum_level`: Takes the maximum of the flagged values in the run, writes a scaled version of it at the end of the run, and copies the preceding entry into the output.
- `accumulate_bytes_and_write_results`: Accumulates a running maximum over a range of bytes and derives a scaled value from it.
- `adjust_flagged_entries_backward`: Walks flagged entries backwards from the current index.
- `adjust_flagged_entries_stepping`: Steps backwards through the flagged entries applying the same scaled level adjustments and stepped copies, returning once the start of the run is reached.
- `apply_scaled_level_steps`: Applies a rounded scaled step to the level tables at the current index, then repeats scaled adjustments across the following flagged entries until the run ends.
- `apply_scaled_level_steps_alt`: Register-based form of the scaled level step.
- `scan_and_scale_run_levels`: Applies a rounded scaled update to the level tables at the current index, scans forward through the code stream for the next marker, and applies further scaled updates weighted by the marker found.
- `scale_levels_backward_chain`: Walks backwards through flagged entries, applying a rounded scaled update to the level tables at each position.
- `scale_levels_backward_chain_alt`: Alternate entry into the backward level chain.
- `scale_levels_backward_tail`: Finishes the backward level chain with a rounded scaled update at the final position, then returns once the start of the run is reached.
- `apply_scaled_delta_backward_scan`: Applies a rounded scaled delta to two level buffers at the current index, then loops backwards over the flagged entries applying scaled adjustments to matching table entries until the index reaches zero.
- `propagate_feedback_stream_values`: Copies bytes forward between buffers while updating a feedback value by multiplying it with a table entry and shifting.
- `copy_adjust_bytes_feedback`: Copies bytes between buffers, updates a feedback value by multiplying it with a table entry and shifting, subtracts the feedback from each byte, and writes the adjusted bytes to two output areas.
- `check_narrator_flag_and_dispatch`: Calls twelve unnamed routines in sequence, then calls one more unnamed routine when the secondary-buffer flag byte in the device state is nonzero.
- `call_three_routines_in_sequence`: Calls three other routines one after another and returns, holding no logic of its own.
- `interpolate_and_store_bytes`: Linearly interpolates between the first byte of a record and a byte selected by an index, storing each intermediate value at eight-byte intervals in the buffer.
- `walk_record_entries_until_end`: Walks eight-byte entries, skipping those marked with a filler value, and returns at the terminator.
- `walk_record_entries_step`: Steps through eight-byte entries, skipping filler-marked slots and calling an unnamed routine at each record boundary, until the terminator is reached.
- `walk_record_entries_recurse`: Calls an unnamed routine and then walks the eight-byte entries of a record until the terminator, calling another unnamed routine at each record boundary.
- `skip_zero_bytes_in_records`: Scans eight-byte records for the next nonzero byte, advancing across records and calling an unnamed routine at each record boundary, until the terminator.
- `skip_zero_bytes_until_end`: Advances through eight-byte records to the next nonzero byte, calling an unnamed routine at record boundaries, and returns when the terminator is reached.
- `skip_zero_bytes_then_helper`: Calls an unnamed routine, then scans eight-byte records for the next nonzero byte.
- `write_record_offsets_from_codes`: Walks the code and count streams, advancing the output pointer by eight bytes per count.
- `emit_record_codes_by_symbol`: Walks the code and count streams like its neighbour.
- `emit_symbol_runs_to_buffer`: For each code, emits a run of symbol bytes chosen from one of three character codes, writing six bytes per repetition at a fixed stride.
- `emit_symbol_runs_alt`: Register-based form of the symbol run writer.
- `emit_symbol_runs_by_code`: Writes symbol runs whose character is chosen from two codes, then advances through the eight-byte records for the remaining codes until the terminator.
- `emit_symbol_runs_comma`: Writes runs of symbol bytes for the current code, using one character code for the comma code and another for the rest, then walks on to the next records.
- `emit_default_symbol_runs`: Writes runs of the default symbol byte for the current code and advances through eight-byte records, continuing until the terminator.
- `copy_previous_record_for_codes`: For codes one and two, copies the previous eight-byte record into each following record for the count given.
- `copy_previous_record_masked`: Copies the previous eight-byte record for codes one and two.
- `update_record_flag_nibbles`: For flagged codes, updates the flag nibbles of the record using a mask and a computed half value.
- `update_record_flag_nibbles_loop`: Loops over the codes applying flag nibble updates to the current and offset records for flagged codes, advancing through the stream until the terminator.
- `copy_previous_record_tail`: Walks the code stream writing eight-byte records.
- `remap_record_bytes_through_table`: Walks eight-byte records until the terminator, replacing the first three bytes of each record with values from a translation table.
- `fill_records_from_lookup_table`: Writes three bytes into the current eight-byte record from a lookup table keyed by each index, then steps through the records until the matching stop value is found.
- `return_void`: This routine is empty and performs no operations before returning.
- `average_record_column_bytes`: Sums the leading bytes of a group of records, adds a stored byte, divides by eight and stores the result in a marker byte.
- `accumulate_record_column_sums`: Accumulates the leading byte of successive records into a running sum, writing an averaged value into the group's marker byte each time six records have been summed.
- `weight_record_column_values`: Sums weighted leading bytes across a group of records, combines the total with two reference bytes, divides by sixteen and stores the result in the group's marker byte.
- `weight_record_column_values_loop`: Loop form of the weighted record column sum.
- `write_record_pattern_by_code`: Writes a pattern byte into the seventh byte of each record for the given count.
- `write_record_pattern_alt`: Register-based form of the record pattern writer.
- `write_record_pattern_tail`: Writes the pattern byte for each record of the current count, advances both streams, and returns when the terminator is reached.
- `apply_pattern_offset_minus_six`: Adds a fixed negative offset to the seventh byte of each record for the given count, choosing later offsets from the code stream, and returns at the terminator.
- `apply_pattern_offset_tail`: Adds the fixed negative offset to the seventh byte of each record of the current count and returns once the terminator is reached.
- `apply_pattern_offset_loop`: Loop form of the pattern offset writer.
- `fill_pattern_constant_and_offsets`: Writes a fixed negative constant into the seventh byte of each record for the count, then applies code-dependent offsets to the following records until the terminator.
- `apply_scaled_offset_to_records`: Adds a value derived from the first byte of each record, scaled down by a fixed ratio, to the seventh byte of each record for the count, then continues through the code stream.
- `decode_nibble_checksum_bytes`: Processes a byte array in two passes.
- `allocate_signal_and_wait_loop`: Allocates a signal bit from the system, looks up the current task, and stores both in the device state.
- `poll_message_port_loop`: Repeatedly tests a state bit, sets a busy bit and calls a library routine that returns a message.
- `clear_busy_flags`: Clears two status bits in the device state flag byte, marking the port as idle.
- `setup_speech_iorb_and_send`: Fills the speech request block with buffer pointers and a timing value derived from a fixed constant divided by a stored rate.
- `setup_audio_dma_channels`: Copies a block of sample bytes into the output buffer, sets up the audio channel pointers and then enables the audio DMA channels by writing the DMA control register with its set bit.
- `mix_speech_samples_to_buffer`: Computes each output sample by summing three table lookups taken from the sample and index tables.
- `advance_sample_phase`: Updates the sample phase accumulators and index, moving the source pointer when its counter reaches zero.
- `mix_channel_samples_by_volume`: Loops over sample bytes, computing volume-scaled values for each channel from the sample table.
- `mix_samples_with_offsets`: Produces each output sample by adding two table values selected from the sample nibbles.
- `bounce_sample_position_and_mix`: Advances a position counter by a step, reversing the step direction at the ends of the sample range, and writes mixed output bytes until the sample count is exhausted.
- `parse_sample_block_headers`: Reads each sample block header and unpacks its fields into the device state.
- `wait_io_and_disable_dma`: Waits for each outstanding audio request with the system's wait call, then disables the audio DMA channel bits and clears the channel's active flag.
- `enable_dma_if_flag_set`: Checks a flag bit in the device state and, when it is set, writes the DMA control register to enable the audio channels.
- `handle_audio_interrupt_reply`: Interrupt-level handler that updates a counter and compares the current state byte with the stored one.
- `decrement_counter_and_enable_interrupts`: Decrements a signed counter in the device state and, when it was zero or below, writes the interrupt enable register with its set bit so interrupts are re-enabled.
- `stop_channel_records_and_wait`: Marks each audio request record for the channel as pending and calls an unnamed routine for each.
- `queue_channel_requests_and_wait`: Marks the channel's request records as pending and calls an unnamed routine for each.
- `wait_channel_requests_and_reset`: Waits on the channel's outstanding request with the system wait call, disabling the audio DMA when its enable bit is set.
- `set_channel_done_flag`: Sets the done bit of the channel's status byte in the request record.

### Message

- `write_message_to_output`: Calls an unnamed routine to obtain the output handle and, when the result is not zero, calls the close-library unnamed routine and then the DOS write routine.
- `open_dos_library`: Opens dos.library through the exec OldOpenLibrary call and stores the returned library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_and_call_entry`: Copies a block of sixteen longwords from a fixed offset into the buffer passed in, when that pointer is not null, then calls the routine whose address is held in a register.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:intro

- `program_entry_references_text`: Program entry point of the intro, eight instructions long, which refers to the text IMP! and makes one call to another routine.
- `write_background_color_register`: Writes the background color hardware register, then passes control through a jump-table entry and one other routine.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `unpack_backward_literal_run`: Reads a header of pointers and a count, copies the stored bytes backward from the end of the block, then enters the bit-driven decoding loop that unpacks further runs.
- `read_bits_from_stream_backward`: Pulls bits from a byte stream that is read backwards.
- `decode_bit_coded_values`: Decodes values by shifting bits out of the backward-read stream and combining them into counts and offsets.
- `decode_length_table_five_bits`: Reads bit-coded values and uses a lookup table indexed by the current count to set the length and offset fields.
- `decode_length_table_four_bits`: Reads bit-coded values and uses a lookup table indexed by the current count to set the length and offset fields.
- `decode_length_table_three_bits`: Reads bit-coded values and uses a lookup table indexed by the current count to set the length and offset fields.
- `decode_length_table_two_bits`: Reads bit-coded values and uses a lookup table indexed by the current count to set the length and offset fields.
- `decode_back_reference_lengths`: Decodes a bit-coded count and offset pair from the backward-read stream, using a lookup table for the count's bit width.
- `copy_back_reference_bytes`: Decodes a bit-coded distance and copies a run of earlier bytes into the output buffer going backwards, copying literal bytes from the stream as well.

### TRANSLATOR.LIBRARY

- `create_and_register_library`: Creates the library structure through the system's MakeLibrary call and, when that succeeds, stores the library base data and registers it with AddLibrary.
- `clear_flag_and_increment_counter`: Increments a 16-bit counter in the structure it is given and clears bit 5 of that structure's flag byte.
- `decrement_counter_call_on_zero`: Decrements a 16-bit counter in the structure it is given.
- `unlink_node_or_mark_pending`: Removes the node from its doubly linked list and calls a library vector to finish release when its status word is zero.
- `unlink_node_and_call_library`: Removes the node from its doubly linked list, then calls a function through the Amiga library base and returns a value stored in the node.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `scan_padded_field_text`: Fills a 26-character stack buffer with spaces, then walks an input string, stopping at a hash character and skipping blanks while calling library routines.
- `pass_through_low_word`: Returns the low 16 bits of the data register it receives with no other work.
- `clear_byte_and_return_word`: Calls one unnamed library routine, clears the byte just before the pointer held in the address register, and returns the low 16-bit value of the data register.
- `scan_list_and_text_pairs`: Loops over a list element and a text buffer together, comparing characters and calling library routines to advance each.
- `resume_text_scan_loop`: Re-enters the same text scanning loop at a later point, comparing characters against the buffer and advancing through the list via library calls.
- `treat_escape_and_backtick_text`: Walks a text string one character at a time, treating a backslash as an escape and a backtick as a terminator.
- `find_escape_or_terminator`: Advances through text one character at a time until it meets a backslash or a backtick.
- `stub_immediate_return`: The decompiled body contains only a return, so the routine does no visible work in the library.
- `match_bracketed_tag_text`: Skips forward to an opening bracket character, then compares the characters that follow against a reference string until a closing bracket.
- `scan_text_backward_with_classes`: Scans a text buffer backwards one character at a time, checking each against a character-class table and comparing it with a second string read from its end.
- `stub_return_only_routine`: The decompiled body is a bare return with no other work, so the routine does nothing visible in the library.
- `scan_text_backward_until_mismatch`: Moves backwards through a text string checking each character against a class table, and calls a library routine before comparing against a reference string.
- `match_parameter_text_until_equals`: Reads characters from a source string and compares each with the text at the parameter pointer, stopping at an equals sign or on a mismatch.
- `compare_text_until_equals`: Reads characters forward from a source string and compares each with a reference string held in the address register, stopping at an equals sign.
- `copy_or_wrap_text_segment`: Finds the next backslash or backtick in a text string.
- `copy_text_segment_with_wrap`: Copies a known-length run of text into the output buffer, or if it would overflow the line width, backs up to a break character and returns that position.
- `copy_counted_bytes_with_flag`: Copies a counted run of bytes from the source to the destination buffer, then sets the continuation flag bit and clears it again if the next source byte is a backtick.
- `truncate_text_at_break`: Walks backwards from the current position through a text buffer until it reaches a break character, then writes a zero byte there to end the string.
- `dispatch_on_special_character`: Searches a short fixed table of special characters for the byte in the data register and calls the handler at the matching position.
- `stub_empty_return_routine`: The decompiled body is empty apart from a return, so this routine has no visible behaviour.
- `stub_empty_return_slot`: Contains only a return in the decompiled output, so it is an empty stub with no visible effect on the library.
- `forward_single_library_call`: Calls one unnamed library routine and returns, with no other logic.
- `wait_while_status_bit_set`: Calls a library routine once, then calls it again while bit 5 of the status value in the data register remains set.
- `library_empty_return`: This library code routine performs no visible actions and simply returns.
- `check_status_and_compare_code`: Calls a library routine and, when bit 5 of the status value is set, calls another and compares the returned character against two codes.
- `stub_no_operation_return`: The decompiled body is a bare return, so the routine does nothing that is visible.
- `match_keyword_letter_sequence`: Compares characters of a text string against the letters of two keyword patterns that begin with different letters.
- `match_ng_then_optional_s`: Checks that the next characters of a text string are the letters N and G, then when bit 7 of the status value is set checks for a following S. It calls library routines for each step and returns early on a mismatch.
- `stub_no_op_entry`: The decompiled body is only a return with no calls or data use, so the routine is an empty entry with no visible behaviour.
- `return_empty_library_routine`: This library routine is empty.
- `return_empty_library_routine_2`: This library routine performs no actions and returns.
- `return_immediately`: This routine is an empty function that performs no operations before returning to its caller.
- `return_from_subroutine_2`: This routine executes a single return instruction.
- `poll_status_bit_until_clear`: Calls a library routine repeatedly while bit 5 of the status value stays set, then returns once that bit is clear.
- `return_empty_library_routine_3`: This is a void library routine.
- `repeat_call_while_bit_one`: Calls a library routine repeatedly while bit 1 of the status value is set, and returns once that bit is clear.
- `call_then_test_flag_bit`: Calls one unnamed library routine, then tests bit 7 of the status value and returns in both outcomes.
- `stub_empty_entry_point`: The decompiled body contains only a return, so the routine has no visible effect.
- `normalize_text_into_field`: Copies characters from an input string into a fixed field buffer, turning control characters into spaces and lowercase letters into uppercase.
- `write_space_before_pointer`: Writes a space character two bytes before the address register and returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d1, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 32 to (a2)+, then copies 32 to (a2)+, then returns.
- `insert_marker_after_matching_letter`: Scans a text buffer for characters from a fixed letter set, and when one matches it inserts a marker character and shifts the rest of the buffer right by one.
- `shift_buffer_insert_marker`: Shifts the bytes of a text buffer right by one place up to a fixed position, writes a marker character into the vacated slot, and advances the buffer pointer.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:PAUSE

- `poll_cia_port_bit_clear`: Spins in a loop while bit 6 of the CIA port A register stays set, then returns.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:delphine

- `load_amiga_hunk_executable`: Loads an AmigaDOS executable by allocating memory for each hunk, then reading the hunk records and copying code and data into it.
- `stub_empty_return_routine_2`: The decompiled body is empty apart from a return, so this routine has no visible behaviour.
- `delay_count_down_loop`: Decrements the count held in the data register in a tight loop until it reaches zero, then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 80 into d0.
- `unpack_backward_run_length`: Decodes a packed byte stream by writing output backwards from an end pointer, copying runs of earlier bytes and handling repeat counts of 3, 4, and 7.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `step_byte_with_space_wrap`: Loops a count-controlled number of times, decrementing a byte register and replacing a zero value with a space character when it wraps.
- `stub_bad_instruction_data`: The decompiled output is only a bad instruction marker, so the bytes here are not valid code and the routine has no behaviour to describe.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:add21k

- `init_global_state_and_dispatch`: Clears a block of 23 longwords of global state and stores the library base and the task pointer.
- `open_system_libraries_and_setup`: Opens the system libraries named in its text (dos, graphics and intuition), raising an alert if an open fails.
- `return_from_subroutine_3`: This routine is an empty function that performs no operations before returning.
- `allocate_tagged_file_request`: Allocates a file request block, stamps it with a four-character tag, and opens the named file through the DOS library wrappers, recording the handle and buffer.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_command_line_arguments`: Splits a command line into words, honouring double-quoted strings and doubled quote characters, and writes a terminator after each word.
- `append_string_fixed_limit`: Appends a null-terminated source string to the end of a destination string, copying at most about 32 thousand bytes and ending the result with a terminator.
- `append_string_counted_limit`: Appends a null-terminated source string to the end of a destination string, with the copy limited by the count passed in the third argument.
- `copy_string_pad_zeros`: Copies bytes from the source string into the destination until the count runs out or a null terminator is reached.
- `open_icon_library_window_resources`: Opens the icon and window resources named in its text through several library calls, storing the resulting pointers in the structure it is given.
- `call_optional_hook_then_return`: Calls a stored function pointer when it is set, then calls one more library routine and returns.
- `execute_cleanup_pointer`: Checks a memory location for a non-zero value and executes the function pointed to by that value.
- `release_program_resources_on_exit`: Releases resources the program allocated: frees the list of entries, runs a stored cleanup pointer if one is set, and frees the remaining blocks.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_indexed_entry`: Closes the entry at the given index in the table if it is in range and in use, releasing it unless bit 7 of its flags is set, then clears its slot.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_dos_output_function`: Calls the DOS Output function through a library base stored in a global variable.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `thunk_library_base_call`: Calls a library function through a base pointer held in a global variable, then returns.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_indirect_library_function`: Loads a pointer from a global variable and calls the function located at a specific offset from that pointer.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_indirect_library_function_2`: Performs an indirect call to a function using an offset from a pointer retrieved from a global variable.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 2 of 3).adf:PPMORE

- `process_amigados_hunk_relocations`: Parses a memory block containing AmigaDOS hunk type codes to perform relocation and data copying.
- `return_empty_stub`: This routine consists of five instructions that perform no observable work before returning.
- `decrement_register_loop`: Decrements a value held in a data register in a loop until it reaches zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 220 into d0.
- `reverse_engineer`: The routine repeatedly calls another routine and uses its returned values to move bytes between memory areas pointed to by a pointer argument and a global pointer.
- `step_byte_once_with_wrap`: Steps a byte register down by one, replacing a zero value with a space character on wrap, then returns.
- `decrement_byte_with_space_wrap`: Decrements a byte held in a register once per count step, replacing it with a space character when it underflows to zero.
- `nop_placeholder`: This routine is empty and contains no executable instructions.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 2 of 3).adf:message

- `call_library_pointer_conditionally`: Calls a routine and, when the zero flag from it is clear, calls a second routine and then a function pointer taken from a global library table.
- `open_dos_library_2`: Opens the DOS library by name through the exec library and stores the returned library handle in a global slot.
- `close_library_handle`: Closes a library handle held in a global variable through the exec library's close-library function.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 3 of 3).adf:paradox

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library_3`: Calls the exec library function OldOpenLibrary to load dos.library.
- `close_library`: Calls a library function via a pointer retrieved from a global variable to close a library.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 2 of 3).adf:bootblock

- `convert_library_result_to_flag`: Calls a library function through the base in the address register and converts its result to a flag.

### Operation Stealth (1990)(U. S. Gold)[cr PDX](Disk 3 of 3).adf:bootblock

- `check_library_status`: Calls a function via a pointer and returns zero if the result is non-zero, or all ones if the result is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_minus_one`: Returns a constant of minus one with no other work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_object_properties`: Initializes a structure or object by setting certain flags, calculating a pointer-based offset, and performing a loop to clear a memory-mapped table or array.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_no_operation`: The routine is empty and performs no operations before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_update_pointers`: Zeros a block of memory using a pointer, updates a global variable, and calls other routines or recursively calls itself based on certain conditions.
- `process_device_request`: Checks a global variable and calls a routine if a certain value is non-zero, then recursively calls itself and calls another routine based on a returned value.
- `traverse_and_call_pointers`: Checks a memory location and calls a routine if a certain value is non-zero, then recursively calls itself and calls other routines based on pointer checks.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `update_memory_from_pointer`: Calls another routine and copies a value from a pointer-based address into a destination, while zeroing out other memory locations.
- `execute_conditional_logic_chain`: Performs a series of conditional checks on a global pointer, calling various subroutines, and updating memory or configuration tables based on certain values.
- `pass_through_routine`: This routine calls another routine with the same arguments and then returns.
- `read_input_flags`: The routine returns a byte derived from a global variable, testing bits 6and 4 and combining the results with a logical OR. It references the CIAA_PRA and POTGOR hardware registers.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `set_global_value`: Sets a global variable to a constant value.
- `update_and_scale_value`: Checks a value, multiplies a global variable by two, applies an XOR mask if a carry or zero condition occurs, and returns a value involving a division and remainder calculation.
- `handle_keyboard_serial`: The routine returns a byte computed from the bitwise complement of a global variable when a flag bit is set, performs a decrementing delay loop, and clears a flag bit in the global variable.
- `populate_table_with_offsets`: Copies a block of data from a source table and populates a destination table with a series of calculated values or offsets derived from input parameters.
- `write_config_table`: Stores a series of hardcoded word values into a table located via a pointer in a global variable.
- `write_config_table_2`: Stores a series of hardcoded word and long values into a table located via a pointer in a global variable.
- `load_pointer_from_table`: Returns a value loaded from a memory address calculated from a global variable.
- `update_table_with_param`: Updates a destination table with or split-values from a parameter if certain conditions are met.
- `copy_source_to_dest`: Copies a sequence of words from a source buffer to a destination buffer located in a table.
- `do_nothing_2`: the routine is empty and performs no operations.
- `return_empty_routine`: The routine performs no operations and immediately returns to the caller.
- `update_display_raster_parameters`: Updates a table with new offset and bitmask values by calling a routine that waits for a specific raster line, then performs a word-based memory copy from a source buffer to a destination buffer.
- `execute_nested_calls`: Performs a sequence of calls to another routine using values passed or stored on the stack.
- `get_and_increment_index`: Reads a byte from a table using an index, then increments that index stored in a pointer.
- `initialize_and_fill_table`: Clears a block of memory, then iterates through a table to clear or update entries with values retrieved from a recursive call or another routine.
- `traverse_linked_list_node`: Checks if a pointer or value is zero, then if non-zero, traverses a pointer-based structure or table to retrieve a value from an offset or a linked element.
- `get_value_from_global_pointer`: Retrieves a value from a memory location pointed to by a global variable and executes further routines.
- `traverse_linked_list_and_get_value`: Recursively traverses a linked list and returns a value loaded from a pointer-based memory location.
- `traverse_list_and_read_flags`: Recursively traverses a linked list to return a value from a pointer-based memory location and reads input flags from a global variable.
- `read_input_flags_2`: Returns a byte derived from a global variable while testing certain bits and combining results with a logical OR.
- `initialize_game_state_tables`: Initializes a series of global pointers and clears or populates memory-resident tables, including a loop that zeros out a large block of memory.
- `reset_memory_values`: Sets a 4-byte memory location to zero and a 2-byte memory location to all ones.
- `traverse_and_reset_memory`: Iterates through a linked list, calling a routine for each node, and then sets a 4-byte memory location to zero and a 2-byte memory location to all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_linked_list`: Recursively calls itself using a pointer or value from a table and updates a global variable before setting a certain memory location to a maximum value.
- `fill_buffer_with_random_bytes`: Fills a memory range with bytes from a function call and checks a secondary range or table for a valid termination condition.
- `search_table_for_matching_value`: Iterates through a table to find an entry where a specific part of a value matches, then recursively checks a sub-range or linked structure.
- `search_and_recurse_table`: Iterates through a table of structures to find a matching key, then recursively calls itself or another routine on a calculated range of values if a match is found.
- `recursively_search_table`: Performs a recursive search through a table or linked structure, updating a global variable and potentially setting a memory location to a specific value.
- `execute_recursive_operations`: Calls another routine with a fixed value, and if successful, performs several recursive calls involving calculated or passed parameters.
- `decompress_bitstream_to_memory`: Decodes a bitstream using a bit-by-bit or block-based expansion method and writes the resulting bytes to a destination memory range.
- `extract_bits_to_memory`: Extracts bits from a routine return value and stores them into a memory location via a pointer, while performing conditional checks and updates.
- `expand_bitstream_to_memory`: Reads a bitstream and writes a sequence of decoded or expanded bytes to a memory location using a pointer.
- `fill_memory_and_parse_bits`: Fills a memory range with decreasing values and uses a bit reader to extract bits from a buffer while performing conditional pointer-based memory writes.
- `fill_memory_with_decreasing_values`: Fills a range of memory with decreasingly smaller values starting from a value derived from a register.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `return_empty_stack_value`: The routine is empty and returns an uninitialized value from the stack.
- `search_and_clear_table_entries`: Recursively searches a table, calling another routine on non-zero entries, and clears the pointer and associated data in that table entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `initialize_disk_io_sequence`: Performs a sequence of calls to locate a device, copy strings, and update buffers or graphics parameters before entering an infinite loop.
- `infinite_loop_with_function_call`: Executes a loop that calls itself and another routine repeatedly without a termination condition.
- `update_graphics_parameters`: Performs several routine calls and calls a function with calculated offsets and a fixed value, likely intended for a display or certain hardware parameters.
- `calculate_string_layout_dimensions`: Iterates through a null-terminated string to determine the maximum width of any segment separated by a delimiter, then calculates a centered horizontal offset and padding or border dimensions.
- `parse_string_and_calculate_dimensions`: Iterates through a null-terminated string to calculate a string length and then calculates a new width and height or padding based on that length.
- `parse_string_attributes_2`: Iterates through a string or buffer, skipping spaces and using character-based lookups in a table to calculate or update screen or buffer or or other data.
- `parse_string_attributes`: Iterates through a string or buffer, skipping spaces and using character-based lookups in a table to calculate or update screen or buffer offsets and attribute-related values.
- `parse_tile_attribute_table`: Iterates through a table of bytes to check for specific control or attribute values, updating screen or buffer offsets and calling drawing routines.
- `process_tile_attribute_table`: Iterates through a table of bytes to check for specific control or attribute values, updating screen or buffer offsets and calling drawing routines to render or clear specific tile-based or pixel-based areas.
- `render_table_sprite_tiles`: Iterates through a table of values to calculate screen or buffer offsets and calls routines to draw or update specific pixel or tile-based patterns.
- `traverse_and_draw_table_elements`: Iterates through a table of values, using certain entries to calculate or update screen or buffer offsets and calling other routines to draw or clear rectangular or line-based areas.
- `parse_string_and_update_display`: Iterates through a sequence of characters, checking if they are alphanumeric or special control characters, and updates a string buffer and display parameters or character offsets accordingly.
- `update_string_buffer_and_graphics_2`: Iterates through a sequence of characters, updating a buffer with character-based offsets and calling a routine to adjust display parameters.
- `update_string_buffer_and_graphics`: Iterates through a sequence of characters, updating a buffer with character-based offsets and calling a routine to adjust display parameters or graphics or certain hard.
- `update_position_and_buffer_2`: Calculates a difference between a table value and a global variable, updates a global offset, and copies a sequence of 16 words from a source.
- `update_position_and_buffer`: Calculates a difference between a table value and a global variable, updates a global offset, and copies a sequence of 16 words from a source table to a destination buffer.
- `copy_cstring`: Copies a null-terminated byte string from a source pointer to a destination pointer, byte by certainly, and terminates the destination with a zero byte.
- `search_string_table`: Iterates through a table of pointers to strings and uses string comparison to find a match for a provided string.
- `find_trackdisk_device`: Calls the exec library to locate the trackdisk.device and extracts a value from a device structure, returning it combined with an input register.
- `search_trackdisk_device`: Locates a trackdisk device and uses its properties to decide whether to close a device or execute an indirect function.
- `examine_and_lock_file`: Attempts to examine and lock a file resource, and if successful, retrieves a value from a specific offset within the file and then unlocks it.
- `reinitialize_string_structure`: Resets a string structure's metadata, sets certain flags, and writes a constant string or a new pointer back to a global structure.
- `compare_and_update_string_metadata`: Compares two null-terminated strings and, if they match, traverses a linked list of pointers to calculate a size and updates a metadata structure or table.
- `compare_and_populate_string_table`: Compares a string from a resource or file with a stored string and, if they match, traverses a linked list of pointers to populate a table with or offset-adjusted pointers to those strings.
- `execute_indirect_calls`: Performs indirect calls to a target address retrieved from a table using a negative offset and calls a library function via a global base.
- `copy_string`: Copies a null-terminated string from a source pointer to a destination pointer.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `compare_two_byte_strings`: Compares two byte strings byte by byte and returns a value indicating if the first is larger, smaller, or equal.
- `safe_memory_copy`: Copies a specified number of bytes between two memory regions, handling overlapping buffers by choosing a copy direction.
- `clear_and_update_globals`: Zeroes out a block of memory, updates a table entry with a constant, and copies a global variable to a new memory location.
- `return_empty_routine_2`: the routine performs no operations and immediately returns to its caller.
- `parse_string_tokens`: Parses a string or a block of code, extracting or copying space-delimited or quote-enclosed tokens into a table of pointers.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit_2`: Copies a string from a source to a destination, stopping if a null terminator or a maximum length limit is reached.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `update_resource_pointers`: Stores a value in a global variable and, if non-zero, retrieves a pointer from a structure to execute an indirect function and update a pointer in a structure.
- `store_value_to_global`: Stores a 32-bit value into a global memory location and then clears the first byte of that location to zero.
- `store_and_parse_string`: Stores a passed parameter into a global variable and then calls a routine to parse a format string.
- `increment_and_store_byte`: Loads a pointer from a global variable, increments that global variable, and stores a byte into the memory location pointed to by the loaded pointer.
- `recurse_with_concatenation`: Recursively calls itself while returning a 64-bit value formed by concatenating a register's previous contents with the new return value.
- `parse_printf_format_string`: Parses a format string to extract or format numeric and string arguments, handling width, precision, and type-specific padding or truncation.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls itself without performing any other operation.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using a bit-shifting or modulo-based algorithm.
- `execute_conditional_pointer`: Checks if a value at a certain offset from a pointer is non-zero and, if so, executes the function pointer located at that memory location.
- `execute_conditional_table_logic`: Checks various memory locations or table-based pointers and executes corresponding functions or jumps if the pointers or certain flags are non-null.
- `return_immediately_2`: The routine contains no executable instructions and immediately returns to its caller.
- `close_resource_with_validation`: The routine validates an index against a range, checks a pointer, and either calls a DOS close function or updates a global status and returns an error code.
- `call_dos_close_indirect`: The routine calls a function pointer retrieved from a table offset.
- `call_dos_close`: Calls a function pointer retrieved from a structure, which corresponds to the close function in the DOS library.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_indirect_function_7`: calculates a target address by applying an offset to a pointer retrieved from a table and executes the resulting code.
- `get_current_directory`: Calls a library function to retrieve the current directory.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `get_input_data`: Calls a library function to read input.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function_8`: loads a function pointer from a table or structure and executes it.
- `open_file_resource`: Calls a library function to open a file.
- `output_data_to_device`: Calls a library function to write output.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `seek_file_position`: Wraps a library function that sets a file pointer to a new position.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `unlock_file_resource`: Calls a library function to unlock a file.
- `write_data_to_file`: Wraps a library function that writes data to a file.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_indirect_function`: Loads a function pointer from a table offset and calls it.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `invoke_indirect_function`: calculates a target address by applying a negative offset to a pointer retrieved from a table and executes that address.
- `call_library_function`: Loads a function pointer from a table offset and calls a library function.
- `populate_struct_and_recurse`: Populates a structure or memory block with several passed or calculated values and then calls another routine recursively with an updated pointer.
- `copy_and_recurse_with_offset_2`: Copies a certain number of bytes from a source or destination and then calls another routine or itself recursively with an updated pointer or offset.
- `copy_and_recurse_with_offset`: Copies bytes from one memory location to another and then calls itself either with a zeroed argument or an incremented argument.
- `call_with_offset_argument`: The routine calls itself with an argument incremented by a constant value.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `dispatch_indirect_function`: calculates an offset from a global pointer, retrieves a function pointer from a table, and executes it.
- `execute_indirect_function`: loads a function pointer from a table using an offset from a global pointer and calls it.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: Loads a pointer from a table or structure and performs an indirect call to an address offset from that pointer.
- `call_indirect_function_2`: Performs an indirect call to a function pointer located at an offset from a pointer stored in a global variable.
- `execute_indirect_table_offset`: Loads a function pointer from a table using an offset from a global pointer and executes it.
- `execute_indirect_function_2`: loads a function pointer from a table using an offset from a global pointer and executes it.
- `call_calculated_function_pointer`: Calculates a function pointer by adding an offset to a value retrieved from a table and executes it.
- `execute_indirect_function_3`: calculates a function pointer by adding an offset to a value retrieved from a table and executes it.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `execute_indirect_function_4`: Loads a function pointer from a table using an offset from a global pointer and executes it.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `execute_table_offset_function`: Loads a function pointer from a table offset and executes it.
- `execute_indirect_function_5`: loads a function pointer from a table offset and executes it.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `execute_indirect_function_6`: calculates a function pointer by applying a constant offset to an entry in a table and calls that function.
