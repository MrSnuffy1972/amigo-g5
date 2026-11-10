# Game 2002 (operation stealth (1990)(u. s. gold)[cr pdx][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02002). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2748272 top-level bytes in 74 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 45 of 45 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 666 of 666 named (0 author symbols), 666 of 666 described (54 by rule, 253 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3111 of 3111 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12652 of 12653 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2360880 |
| code | 41310 |
| data | 269728 |
| padding | 9794 |
| unused | 66560 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| NARRATOR.DEVICE | 3 | 3782 | 268 |
| Message | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:endcli | 2 | 117 | 2 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:intro | 2 | 174 | 12 |
| TRANSLATOR.LIBRARY | 2 | 490 | 50 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:PAUSE | 2 | 3 | 1 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:delphine | 2 | 207 | 8 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:add21k | 3 | 637 | 38 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 2 of 3).adf:paradox | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 3 of 3).adf:paradox | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 2 of 3).adf:bootblock | 1 | 10 | 1 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 3 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 907 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 80 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 134 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 415 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 413 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 273 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 147 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1047 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 803 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 247 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1328 | 75 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 211 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 936 | 55 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 394 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 100 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 286 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 130 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 98 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 24 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 240 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 52 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 179 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 664 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 126 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 537 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (19 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (19 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (19 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (33 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- operation stealth (1990)(u. s. gold)[cr pdx][a2](disk 1 of 3)/Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:delphine: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:intro

## Copy protection found

- none found

## Functions


### NARRATOR.DEVICE

- `init_narrator_device_and_task`: Creates the device base with the exec MakeLibrary call, then allocates a message port with its signal and list headers, and finally adds the device and a task to the system.
- `allocate_request_or_fill_reply`: When the request code is zero and the unit is already active, it bumps the request counters, writes fixed default values into the reply block and clears the busy flag.
- `open_audio_device_for_request`: If the unit is idle it opens the audio device by name through the exec base and allocates a request block, recording a failure code if either call fails.
- `allocate_command_port_request`: Stores the command port parameters in a new request block and allocates its memory through the exec base, writing a failure code and returning if the allocation fails.
- `update_device_state_fields`: Stores the value passed in the data register into the unit's state field, bumps the unit's request counters and copies the counter into the request block.
- `set_request_status_and_reply`: Stores the passed-in status byte in the request's result field and sets its reply link to minus one.
- `update_active_request_or_queue`: If the unit is not busy and its active request has a changed status byte, it copies that byte split into low and high nibbles and completes the request.
- `match_request_or_insert_queue`: When the request key matches the unit's current request it either copies a changed status byte and completes the request, or links the request onto the unit's list.
- `copy_status_or_queue_request`: Copies the unit's status byte into the request when the status has changed and the unit is not busy, then completes the request.
- `copy_byte_and_split_nibbles`: Copies a status byte from the source request into the destination, stores its low and high four-bit halves in two more destination bytes and clears the result field.
- `dequeue_and_validate_speech_request`: Removes the first matching request from the unit's pending list and checks its numeric fields against fixed bounds before copying them into the unit state.
- `validate_speech_fields_and_start`: Checks the request's numeric fields against fixed bounds and copies them into the unit state when all pass, writing a failure code otherwise.
- `close_unit_on_last_request`: Walks the unit's request list and calls the matching routine for each entry whose key equals the request's key.
- `decrement_counter_and_close`: Decrements the unit's open counter and, when it reaches zero, frees a buffer and closes the device through the exec base.
- `dispatch_handler_by_state`: Reads a small state number from the request and, when it lies between one and eight, calls the handler stored in the table at that index.
- `free_unit_or_mark_busy`: If the unit's counter is zero, it unlinks the unit from its list, frees its memory through the exec base and returns a stored pointer.
- `unlink_and_free_node`: Unlinks the node from its list, frees its memory through the exec base and returns a value stored in the node.
- `dispatch_command_by_code`: Dispatches on the request's command code: codes one, six, seven and eight set or clear the unit's busy and pending flags around library and cleanup calls, and code two links the request into the unit's queue.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `set_command_seven_and_dispatch`: Clears the done flag in the request, sets the unit's command code to seven with its pending bit, and calls two device library routines, the second with the unit as argument.
- `set_command_six_and_dispatch`: Sets the done flag in the request, sets the unit's command code to six with its pending bit, and calls a device library routine.
- `set_flag_then_clear`: Sets the busy bit in the request's flags, calls two routines in turn, and clears the busy bit again.
- `set_flag_clear_then_call`: Sets the busy bit in the request, calls a routine, clears the bit again, zeroes the status byte of the reply block and calls a second routine.
- `walk_list_under_interrupt_guard`: Calls a routine for the unit's current request when one is present, then raises the interrupt nesting counter and calls a routine for each entry of the unit's list.
- `release_interrupt_guard_count`: Decrements the interrupt nesting counter and, when it falls below one, writes the enable value that unmasks interrupts.
- `abort_or_unlink_request`: Under the interrupt guard, marks the request as aborted when it is the unit's active request.
- `unlink_request_if_queued`: Searches the unit's list for the given request and, when it is found, unlinks it and marks it with a cancelled status before calling the completion routine.
- `unlink_and_complete_request`: Unlinks the request from its list, marks it with a cancelled status, releases the interrupt guard counter and calls the completion routine.
- `decrement_counter_and_set_global`: Decrements the interrupt guard counter held in the unit and, when the previous value was below one, writes the enable value that unmasks interrupts.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `fail_request_with_status`: Stores a failure status code in the request's result byte and calls the completion routine.
- `complete_request_with_success`: Stores a zero success status in the request's result byte and calls the completion routine.
- `clear_flags_and_call_handler`: Clears the unit's busy bit and zeroes a byte in the request, then calls a device library routine through the unit's base.
- `validate_speech_request_fields`: Checks the request's numeric fields against fixed bounds and copies them into the unit state.
- `check_speech_bounds_and_split`: Checks the request's first numeric field against a bound and then the remaining fields, copying each accepted value into the unit state.
- `check_partial_speech_and_split`: Checks a subset of the speech fields against fixed bounds and copies the accepted values into the unit state.
- `store_word_and_check_fields`: Stores a word value from the request into the unit's pitch-related field, then checks the following fields against fixed bounds.
- `check_speech_fields_and_split`: Checks the trailing speech fields against fixed bounds and copies the accepted values into the unit state.
- `check_range_then_split_text`: Checks one numeric field against a fixed upper bound and copies the accepted value into the unit state.
- `store_byte_and_split_text`: Checks a field against a bound of sixty-four and stores it as a byte in the unit state, then chooses a phrase break point in the text.
- `run_speech_phrase_loop`: Finds the current task and adjusts its priority through exec, then loops over the text choosing a break point at a comma, dash or space for each phrase.
- `store_result_and_complete`: Copies a stored value into the request's result field, writes the status byte passed in the data register and calls the completion routine.
- `clear_result_and_set_status`: Zeroes the request's result field, writes the passed status byte and calls the completion routine.
- `finish_speech_phrase_loop`: Chooses a break point in the text for the next phrase and runs the speech stages, freeing the phrase buffer and advancing the text pointer.
- `split_phrase_at_space`: Looks for a space in the text, falling back to a comma or dash scan when none is found, then runs the phrase stages and advances the text pointer.
- `split_phrase_at_midpoint`: Computes a break point at half the text length, runs the phrase stages and advances the text pointer until the text is consumed.
- `allocate_and_copy_speech_tables`: Resets the unit's speech state, copies thirty-four words from the request into newly allocated buffers once per stored count, and stores the buffer in the unit.
- `allocate_speech_table_for_index`: Allocates a buffer for the table chosen by the stored index and copies thirty-four words into it repeatedly.
- `create_speech_request_block`: Stores the passed-in request block, sets up its message header and signal through the exec library, and allocates and fills the speech table buffer for the stored index.
- `allocate_and_copy_table_block`: Stores the passed-in byte as the stored index, allocates a memory block and copies thirty-four words into it repeatedly.
- `copy_word_table_to_buffer`: Stores the buffer pointer passed in the data register in the unit and copies thirty-four words from the request into it, repeating for a count taken from a byte in the unit.
- `perform_io_and_free_buffers`: Stores the request's byte in the unit, sends an I/O request through the device base, then frees the two buffers and the signal when they were allocated.
- `finish_io_and_free_resources`: Stores the request's byte in the unit, sends an I/O request through the device base, then frees the buffers and the signal when they were allocated.
- `translate_text_to_phonemes`: Clears the three output tables and, for text longer than zero, converts the input into phoneme codes by matching two-character keys against a rule table.
- `translate_text_to_phonemes_unchecked`: Clears the three output tables and runs the same rule matching as its neighbour, but starts from the first character without checking the text length.
- `scan_phoneme_rules_for_text`: Continues the rule scan from the current text position, matching two-character keys against the rule table and writing phoneme codes and marker bits.
- `continue_phoneme_scan_with_limit`: Continues the rule scan but stops when the text position reaches the limit stored in the unit, recording the position reached.
- `emit_phonemes_until_limit`: Emits the current phoneme and continues scanning the text until the stored limit is reached, then records the scan position in the unit.
- `emit_phoneme_for_letter`: Looks up the rule for the current letter, writes its phoneme code and skips the marker characters that follow, then scans the next text character.
- `emit_marker_phoneme`: Handles one marker character by setting the matching marker bit in the output tables and advancing the scan.
- `write_phoneme_and_flags`: Writes the phoneme code for the current character to the output table, updates the flag tables and advances the scan position.
- `scan_text_to_phonemes_limit`: Scans the input text against the rule table until the stored end position is reached, handling digit characters and marker characters as it goes.
- `return_without_action`: Returns to the caller at once without doing any work, so it acts as an empty entry in the handler tables.
- `record_phoneme_and_return`: Looks up the phoneme at the current output position, writes the end-of-list markers into the three output tables, and records the scan position in the unit.
- `append_phoneme_end_markers`: Writes end-of-list markers into the three phoneme tables and pads them when the entry count exceeds three, then stores the scan position in the unit.
- `store_word_in_unit_field`: Stores the word passed in the data register into a fixed field of the unit.
- `set_marker_bits_on_phonemes`: Walks the phoneme list up to its end marker and, for each entry flagged in the rule table, sets the marker bit on that entry.
- `mark_phoneme_flags_from_table`: Repeats the marker-bit pass over the phoneme list, setting the marker on preceding entries from the rule table flags and the digit rules.
- `mark_phoneme_flags_indexed`: Variant of the marker-bit pass that looks up the current entry by a computed table index and sets markers on preceding entries when the digit rule matches.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `spread_emphasis_across_phoneme_spans`: Walks the phoneme list in runs, setting the high bit on the first entry of each run and spreading an emphasis value from the rule flags across the entries of the span.
- `propagate_run_flags_indexed`: Variant of the run-flag pass that spreads a span's emphasis value across the phoneme entries using index registers.
- `propagate_run_flags_loop`: Repeats the run-flag pass over the phoneme list, setting the high bit on each span start and spreading the span's emphasis value across its entries.
- `spread_run_flags_unreached`: Unreached code in the gap after the run-flag routines, with the same shape as the emphasis pass.
- `return_immediately`: Consists of a single return and does nothing else.
- `match_phoneme_rules_at_position`: Scans the phoneme string from its start, testing each position against the three-entry context patterns in the rule table.
- `apply_matching_phoneme_rule`: Tests the entries on either side of the current phoneme against a rule record and, when the context fits, writes the replacement phonemes and advances the scan.
- `scan_rules_after_match`: Continues the rule scan from the position after a match, testing each candidate rule's context and applying its replacement when the context fits.
- `scan_phoneme_rules_from_index`: Scans the phoneme list from a given index, testing the context around each entry against the rule records and applying the replacement when all context bytes match.
- `test_bit_with_table_lookup`: Walks a rule record's flag bytes, reading a word from a table and inverting it when a flag says so.
- `insert_phoneme_entry_at_index`: When the entry count is below 512, shifts the three output tables up by one entry from the insertion point.
- `shift_bytes_and_clear_fields`: Increments the entry counter in the unit, shifts bytes backward in three buffers by a computed count, and writes the passed byte into the new end with two zero bytes in the other buffers.
- `run_speech_stages_in_order`: Calls seven processing routines in order and returns zero, chaining the stages of the phoneme pipeline.
- `clear_record_flags_set_last`: Walks the eight-byte entries of a table until the end sentinel, zeroing byte seven of each entry.
- `clear_record_flags_until_match`: Clears byte seven of eight-byte entries until one matches the byte in the data register, then sets byte seven of the first entry to 160.
- `adjust_phoneme_durations_and_pitch`: Walks the phoneme buffers, merging the duration fields at the marker code and halving the duration of entries with the split flag.
- `adjust_phoneme_timing_fields`: Performs the same timing and flag pass as its neighbour in a single loop over the phoneme entries.
- `adjust_phoneme_flags_and_durations`: Walks the phoneme entries in the same timing and flag pass as its neighbours, merging marker entries, halving split durations and setting flag codes from the entry bits.
- `set_phoneme_flags_from_entry`: Starts from a preloaded entry code, copies the phoneme value from the rule table and sets flag codes from the entry bits.
- `set_stress_flags_by_entry`: Sets the phoneme's flag code to four, five or six from the entry bits and walks the following entries, merging marker entries and setting codes until the end marker.
- `set_stress_level_flags`: Sets the phoneme flag code from the entry bits and continues over the following entries, merging marker entries and setting codes until the end marker.
- `set_flags_from_entry_bits`: Chooses a flag code of three, four, five or six from the entry bits, writes it into the phoneme flags and continues over the following entries until the end marker.
- `scan_phoneme_entries_for_flags`: Scans the phoneme entries, merging marker entries and copying phoneme values together with their flag codes until the end marker.
- `compute_total_duration_and_allocate`: Adds the duration fields of the phoneme list up to its end marker and stores the total in the unit.
- `allocate_buffers_for_total`: Stores the passed-in total and allocates a buffer of eight bytes per unit plus eight through the exec library.
- `store_buffer_and_allocate_second`: Stores the passed-in pointer as the unit's primary buffer and, when the unit flag is set, allocates a second buffer through the exec library and stores it too.
- `expand_phonemes_into_frames`: Expands each phoneme in the list into eight-byte frame records, one per duration step.
- `expand_stressed_phonemes_into_frames`: Variant of the frame expansion that takes the phoneme code from the current input position and uses the stress-adjusted table values.
- `expand_phoneme_frames_with_levels`: Expands the phoneme list into eight-byte frame records like its neighbours, but writes three table levels from the phoneme code into the frame bytes.
- `interpolate_frame_values`: Walks the frame records and, for each group of six values, moves each value toward the matching value of the next frame by a weight taken from a table.
- `interpolate_frames_by_weight`: Performs the same six-value interpolation between neighbouring frames, weighted by a table value, over the phoneme list until the end marker.
- `interpolate_frames_nested_loop`: Repeats the six-value interpolation across the frames in a nested loop, advancing by each entry's duration and stopping at the end marker.
- `interpolate_frames_with_weight_register`: Interpolates the six frame values between neighbouring entries using a weight held in a register.
- `interpolate_frames_stepwise`: Steps through the frame records, applying the six-value interpolation with a weight held in a register.
- `build_frame_records_for_phonemes`: Builds eight-byte frame records for each phoneme, sizing each run from its duration and its neighbours' values and filling the record bytes with the default markers.
- `build_frame_records_with_fill`: Builds the frame records phoneme by phoneme like its neighbour, writing the fill value into the marker bytes of each frame.
- `build_frame_records_from_neighbours`: Builds the frame records for each phoneme from its neighbouring values and duration, writing fill values into the record bytes.
- `build_frame_records_by_pitch`: Builds frame records with a pitch level for each entry, derived from the neighbouring table values, and returns zero at the end marker.
- `build_frame_records_with_levels`: Builds frame records for each phoneme from the levels of its neighbouring entries, writing fill values for the marker bytes.
- `build_frame_records_with_transitions`: Builds frame records for each phoneme and applies the transition flags from the rule record, branching on those bits to decide whether to emit a level change.
- `build_frame_records_fill_loop`: Builds frame records like its neighbours and writes a fill value into the frame bytes when the marker bit is clear.
- `build_frame_records_tail`: Writes the fill value into the trailing frame records of the last phoneme, clearing their bytes, then walks the remaining entries.
- `build_frame_records_with_runs`: Builds frame records run by run, writing fill values into the marker bytes of each run.
- `build_frame_records_and_gaps`: Builds frame records for each phoneme and clears the gap bytes between runs.
- `build_frame_records_by_duration`: Builds frame records sized by each phoneme's duration, clearing and filling the bytes of each run.
- `remove_entries_flagged_for_drop`: Copies the three phoneme lists into compacted form, skipping every entry whose drop flag is set in the rule table.
- `write_byte_to_three_pointers`: Writes the byte in the data register to three buffers at the positions held in the pointer registers, then returns.
- `run_two_stages_in_order`: Calls two processing routines in order and returns, chaining two stages of the phoneme pipeline.
- `mark_phoneme_run_boundaries`: Clears the low nibble of each frame flag byte, then walks the phoneme list setting boundary bits on the first and last entries of each run that the rule flags mark.
- `set_boundary_flags_on_runs`: Walks the phoneme list and sets boundary bits on the first and last entries of each run, using the flags from the rule table.
- `set_boundary_flags_by_index`: Repeats the run boundary marking, indexing the rule table by the current phoneme code.
- `set_boundary_flags_on_entries`: Marks the boundary bits on each run's entries while advancing through the list, and returns at the end marker.
- `set_boundary_flags_with_scan`: Scans the list for run boundaries and marks the entries flagged by the rule table.
- `mark_run_end_flags`: Scans the list, setting boundary bits as each run starts and the closing bit on its last entry.
- `set_run_edge_flags`: Sets the boundary bits on the two entries before the current position and the closing bit on the last entry, then walks the list to the end marker.
- `compute_frame_periods_from_pitch`: Divides a fixed constant by the unit's rate value to get a base period, then writes per-frame period bytes from the rule table's pitch and duration fields.
- `compute_period_bytes_per_frame`: Variant of the period computation that keeps the pitch and duration values in registers.
- `compute_frame_periods_loop`: Computes the frame period bytes for each phoneme from its pitch and duration fields, interpolating between neighbouring periods and stretching them across the frame count.
- `fill_frame_periods_stepwise`: Steps through the frame period table, writing a period byte for each frame and interpolating between neighbouring periods.
- `fill_frame_periods_by_segment`: Writes period bytes segment by segment, interpolating between neighbouring periods and reloading the pitch and duration values at each phoneme boundary.
- `fill_frame_periods_nested`: Writes period bytes across each phoneme's frames in a nested loop, interpolating between neighbouring periods.
- `normalize_frame_period_bytes`: Divides the period stored in the unit by 110 and writes the quotient into byte seven of each frame record until the end marker.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `compute_frame_output_values`: Computes the output value for each frame from the phoneme's table entry, adjusting it with the neighbouring phonemes' flags and a weighted interpolation.
- `compute_frame_values_by_flags`: Variant of the frame value computation that keeps its state in registers.
- `compute_frame_values_stepwise`: Walks the frame records, computing each output value from the phoneme table with neighbour flags and a weighted interpolation.
- `compute_frame_values_with_transitions`: Computes frame output values with a transition check on the neighbouring phonemes, applying the weighted adjustment and clamping each result to 63 before it is written into the record.
- `apply_frame_value_adjustments`: Applies the neighbour-flag adjustments to the frame values in a single pass, weighting each value by its phoneme table entry and clamping at 63 until the end marker.
- `adjust_frame_values_by_neighbours`: Walks the frame list adjusting each value by its neighbours' flags and table weights, clamps the results at 63 and returns at the end marker.
- `adjust_frame_values_in_loop`: Repeats the neighbour-based frame value adjustment in a loop, clamping each value to 63 and returning at the end marker.
- `adjust_frame_values_nested`: Performs the neighbour-based frame value adjustment through nested branches, clamping each result to 63 and returning at the end marker.
- `adjust_frame_values_with_branches`: Applies the neighbour-based frame value adjustment with branch-heavy control flow, clamping each value to 63 before it is written into the record.
- `finish_frame_value_adjustments`: Finishes the frame value adjustment for the last entries, applying the neighbour-flag weights, clamping at 63 and returning at the end marker.
- `adjust_frame_values_head`: Calls a routine for the leading frame, then walks the frame records applying the neighbour-flag weights and clamping at 63 until the end marker or the next boundary flag.
- `adjust_frame_values_tail`: Walks the frame records applying the neighbour-flag weights and clamping values at 63, then finishes the last boundary and returns at the end marker.
- `compute_scaled_shifted_product`: Multiplies the low word of the data register by the word passed in the other register, adds 16 to the low half of the product and returns it shifted right by five bits along with the high word.
- `initialize_and_clear_buffers`: Stores the start pointers of the unit's phoneme and frame buffers, clears a set of word fields, and zeroes six buffers in blocks of thirty-two entries.
- `increment_counter_and_run_stages`: Increments the unit's 16-bit counter and, unless the counter wrapped or became zero, calls four processing routines in order.
- `pack_phoneme_codes_into_buffer`: Packs the phoneme codes from the input list into the output buffer, storing a flag nibble for each entry and recording the first and last positions that exceed the threshold.
- `pack_phoneme_codes_from_register`: Variant of the packing pass that keeps the current code in a register, writing packed nibbles and flags into the output and recording boundary positions.
- `pack_phoneme_codes_loop`: Packs the phoneme codes into the output buffer with the same flags and boundary recording as its neighbours.
- `set_flag_and_store_indices`: Sets the group-end bit on the entry before the current index and stores the index in two unit fields, copying the first into the second when the counter is zero.
- `propagate_flags_backward`: Counts the entries in the phoneme and frame buffers, marking special codes and setting flag bits from neighbouring codes.
- `propagate_flags_from_index`: Walks the entries marking special codes and flag bits, saves the buffer positions in the unit, and propagates the flag bits backward from the last index.
- `propagate_flags_from_pointers`: Variant of the flag marking pass that stores the current buffer pointers in the unit before the backward propagation.
- `propagate_flags_backward_pass`: Stores the three buffer pointers in the unit, clears the carry byte and runs the backward pass that propagates flag bits and moves marker values through the buffer.
- `propagate_flags_downward`: Walks the buffer downward from the last index, ORing the carry byte into each entry.
- `shift_marker_values_backward`: Moves the marker value backward through the buffer from the current index, stopping when the index leaves the valid range, and clears the origin entry.
- `swap_and_clear_byte_flags`: Copies the entry at the source index to the destination index and clears the source when the two differ.
- `set_low_bit_over_count`: Sets the low bit of every flag entry below the stored count, walking down from the top entry to index zero.
- `set_low_bit_to_zero`: Sets the low bit of each flag entry from the given index down to zero.
- `or_mask_with_early_exit`: Chooses a mask of four or eight from the control byte at the end of the list and ORs it into the entries, walking backward.
- `set_mask_and_nibble_flag`: ORs a mask of 176 into the entry, or 48 when the entry code is eight.
- `accumulate_table_entries`: Calls a sequence of routines depending on two unit fields, then adds the stored short value to eight consecutive words of the table in the unit.
- `compute_clamped_table_value`: Computes a value from the position and a scaled step, clamps it between 125 and 165, and stores the low byte in the table at the given index.
- `clamp_value_and_store_char`: Clamps the value passed in the data register to a lower bound of 125 and an upper bound of 165, then stores the low byte at the table position given by the index.
- `adjust_pitch_across_segments`: Derives a per-entry step from the segment length and adjusts the marked entries around the peak position.
- `adjust_table_values_loop`: Computes the absolute difference between two table entries, scales it and adds the result to a destination entry.
- `apply_final_pitch_corrections`: Computes scaled values for each flagged entry from its level and writes them into two output tables.
- `apply_pitch_corrections_loop`: Walks the flagged entries backward, scaling their values and writing the corrected pitch and level bytes.
- `compute_scaled_char_values`: Walks the flagged entries backward, computing scaled character values from two tables and writing them into the destination tables.
- `apply_segment_slope_adjustments`: For each flagged segment, scales the two boundary values by a slope factor chosen from the segment length and adjusts the level table.
- `apply_segment_slope_loop`: Applies the same segment slope adjustments to the flagged entries in a backward walk, then runs the peak and offset pass over the marked entries.
- `apply_slope_adjustments_nested`: Performs the segment slope adjustments in nested loops, then runs the peak and offset pass over the entries that carry the marker bits.
- `apply_segment_slope_variant`: Variant of the segment slope pass that holds its boundary indices in registers.
- `apply_pitch_slope_pass`: Applies the segment slope scaling to the boundary entries and writes the corrected level and pitch bytes.
- `apply_pitch_slope_pass_variant`: Variant of the slope pass that updates the boundary level before scaling, then runs the peak and offset pass over the marked entries.
- `apply_pitch_slope_pass_tail`: Finishes the slope pass for the last segment, updating the boundary value and writing the corrected level bytes, then runs the peak and offset pass.
- `apply_pitch_slope_pass_entry`: Applies the slope scaling at the segment boundaries and runs the peak and offset pass over the marked entries.
- `apply_pitch_slope_stack_param`: Applies the slope scaling using a slope value passed on the stack, updates the boundary entries, and runs the peak and offset pass over the marked entries.
- `apply_pitch_slope_stack_variant`: Repeats the stack-parameter slope scaling over the segments, then runs the peak and offset pass over the marked entries.
- `apply_pitch_slope_stack_tail`: Finishes the stack-parameter slope scaling for the last segment and runs the peak and offset pass over the marked entries.
- `apply_pitch_slope_with_boundaries`: Applies the slope scaling and boundary adjustments to the marked entries, then runs the peak and offset pass.
- `apply_peak_offset_pass`: Scales the peak level of the current segment and walks backward through the marked entries.
- `apply_peak_offset_pass_register`: Variant of the peak and offset pass that takes its scale factor from a register and walks the marked entries backward.
- `apply_peak_offset_running_max`: Tracks a running maximum over the frame values and writes a scaled peak correction into the entries.
- `apply_scaled_offsets_by_nibble`: Walks the entries backward and, for each entry whose high nibble is set, adds a scaled correction.
- `apply_scaled_offsets_register`: Repeats the nibble-driven scaled correction with the entry pointer held in a register, walking backward until the first entry.
- `scale_buffer_by_signed_factor`: Walks the buffer backward, skipping entries whose high nibble is zero, and multiplies each entry by a signed factor chosen from the source bit.
- `smooth_pitch_curve_segments`: Computes the differences between the level tables for each marked segment, halves them when the two levels disagree, and interpolates values across the segment's entries.
- `smooth_pitch_curve_loop`: Repeats the segment smoothing with interpolated values across each marked run, then writes the fixed default levels to the remaining entries.
- `smooth_pitch_curve_retry`: Smooths the level differences across marked segments of the pitch curve, interpolating values between segment boundaries.
- `smooth_pitch_curve_stepwise`: Repeats the segment smoothing one entry at a time, copying the interpolated values along each run and writing fixed default levels to the remaining entries.
- `smooth_pitch_curve_entry`: Calls a routine for the current entry and continues smoothing the marked segments, writing fixed default levels to the remaining entries.
- `smooth_pitch_curve_from_entry`: Continues the pitch curve smoothing from the entry reached by the previous pass, writing fixed default levels to the remaining entries.
- `finalize_pitch_defaults`: Writes fixed default levels into the pitch table and then adjusts the last segment's end values using the scale factor from its code.
- `fill_pitch_defaults_backward`: Fills the pitch table backward with a scaled value taken from the frame flags, then adjusts the final segment's end values with fixed defaults.
- `interpolate_segment_levels`: Interpolates the levels across the current segment and writes the segment's closing value.
- `interpolate_segment_levels_tail`: Interpolates the segment levels from a scale value held in a register, then writes the closing value from the largest entry or from fixed defaults.
- `write_segment_closing_value`: Finds the largest level in the table and writes a scaled closing value and a zero flag into the segment's last entry.
- `accumulate_bytes_and_write_results`: Accumulates the bytes of the source area while keeping a running maximum, then writes a scaled value and a derived value into the destination buffers with a conditional copy.
- `apply_segment_slopes_backward`: Walks the segments backward, applying scaled slope corrections to the level tables where the flags match.
- `apply_segment_slopes_loop`: Applies the same scaled slope corrections to the segments in a loop over the entries, then applies the end correction to the final entry.
- `update_buffers_with_scaled_deltas`: Walks an index downward through several byte buffers, adding scaled signed deltas to each one.
- `update_buffers_from_mid_entry`: Enters the same downward delta-update loop as its neighbour at a point where the first delta is computed from a register value rather than a fixed constant.
- `walk_and_update_scaled_buffers`: Repeats the scaled delta update over a downward index, choosing between two delta paths depending on a table-selected flag.
- `accumulate_scaled_bytes_by_flags`: Walks an index downward and, for positions whose flag bits are set, adds a scaled byte delta into one buffer.
- `scale_bytes_with_flag_tests`: Walks an index downward, scaling one byte per step by a fixed factor and subtracting it from a second buffer.
- `update_buffers_with_fixed_scale`: Applies a scaled delta to the buffers at each index step using a fixed initial scale factor.
- `update_attribute_buffers_downward`: Walks an index downward, reading a byte from one buffer and using it to scale a signed delta.
- `copy_bytes_with_feedback_update`: Copies one byte forward in a buffer per step and multiplies a running feedback value by a table entry, shifting the result.
- `copy_adjusted_bytes_with_feedback`: Repeats the same copy-and-feedback step as its neighbour, but runs as a do-while loop over a count.
- `invoke_step_routine_twelve_times`: Calls the same unnamed step routine twelve times in a row, then calls it once more only if a flag byte in the structure pointed to by the base register is non-zero.
- `invoke_step_routine_three_times`: Calls the same unnamed step routine three times in succession and then returns.
- `interpolate_and_store_bytes`: Linearly interpolates between the first byte of a buffer and a second byte, using a division by a count derived from the index.
- `walk_records_until_terminator`: Scans a table of eight-byte records and ends at a terminator byte of minus one.
- `walk_marker_table_calling_step`: Walks a byte table of eight-byte records and calls the step routine when an entry's marker differs from the expected value.
- `scan_record_table_after_step`: Calls the step routine once and then scans forward through a record table, advancing one eight-byte record at a time.
- `process_nonzero_records_until_end`: Scans a table of eight-byte records, skipping zero bytes, and stops at a terminator byte of minus one.
- `advance_records_until_terminator`: Advances through a table of eight-byte records, calling the step routine when the scan position is not in the first slot.
- `call_step_then_skip_zero_runs`: Calls the step routine, then skips runs of zero bytes across eight-byte records.
- `expand_coded_entries_into_buffer`: Expands a sequence of code bytes, terminated by minus one, into an output buffer, using a parallel array of counts.
- `expand_coded_entries_with_stride`: Performs the same coded-entry expansion as its neighbour, but takes the table stride from a register, so the pattern copies use a variable step.
- `expand_coded_rows_with_stride`: Repeats the coded-entry expansion in an outer loop, advancing the output by a register-held stride after each pass.
- `expand_codes_with_inner_pattern`: Writes a five-value table pattern for each count in the code stream and handles the remaining codes by skipping eight-byte steps or applying small additive changes.
- `expand_pattern_codes_subset`: Expands a subset of pattern codes into five-value groups from a table, with the code byte selecting the table offset.
- `expand_pattern_codes_comma_only`: Expands pattern codes into five-value table groups for a code byte that selects one of two offsets.
- `expand_pattern_codes_default_offset`: Runs the coded-entry expansion with a default table offset and switches to alternate offsets for three special code bytes.
- `update_row_pairs_from_codes`: Walks a code stream and an output buffer together.
- `copy_rows_and_merge_nibbles`: Processes a code stream against a parallel mask stream, copying three bytes from the previous row for codes one and two.
- `merge_nibbles_by_code_mask`: Walks a code stream with a parallel mask stream and applies the same row copy and nibble merge as its neighbour, using masks held in registers.
- `merge_nibbles_with_row_copy`: Walks two streams to copy previous-row bytes for codes one and two and to merge split nibble values into the output rows for other flagged codes.
- `merge_row_nibbles_until_stop`: Walks a code stream and a mask stream, merging split nibble values into paired output rows for flagged codes and copying previous-row bytes for codes one and two.
- `remap_row_bytes_through_table`: Walks a buffer of eight-byte rows from a starting offset and replaces the first three bytes of each row with values looked up in a global table.
- `gather_bytes_through_index_table`: Copies bytes from a source table, selected by a running index, into eight-byte output rows, and takes the next index from each row's byte.
- `return_from_subroutine_2`: The routine is empty and performs only a return instruction.
- `sum_column_bytes_per_entry`: For each entry in a table, sums seven bytes at a fixed stride starting from that entry, then stores the total, shifted right by three, at a fixed offset within the entry.
- `accumulate_running_sums_per_row`: Walks a byte table in eight-byte steps, keeping a running total of the bytes it reads.
- `weighted_row_sums_into_entry`: Repeats a weighted-sum update over successive eight-byte rows.
- `accumulate_weighted_sums_until_stop`: Walks a byte table in eight-byte steps, accumulating doubled byte values into a running total.
- `apply_code_offsets_to_row_field`: Processes a code stream paired with a count stream and adds a per-code offset to a field of each eight-byte row.
- `apply_code_offsets_to_rows`: Processes a stream of codes with paired counts, applying a fixed or derived offset to a field of each eight-byte row for each code.
- `apply_ten_offset_to_field_rows`: Enters the shared code-driven row update with a starting offset of ten added to each row field.
- `apply_minus_six_offset_rows`: Enters the shared code-driven row update with a starting offset of minus six added to each row field.
- `apply_minus_six_then_derived_offsets`: Adds a starting offset of minus six to each row field and then applies derived offsets for codes with the matching flag bits.
- `apply_minus_six_with_stop_check`: Applies a starting offset of minus six to each row field and returns when the flag bits of the current code end the update.
- `apply_minus_twenty_six_offsets`: Adds a constant of minus twenty-six to a field of each row for the count given by the current code.
- `apply_derived_offset_row_update`: Enters the shared code-driven row update with a starting offset derived from the row's first byte, then continues processing codes with the same branch logic as its neighbours.
- `write_nibble_checksums_per_group`: Processes groups of bytes in two passes.
- `request_signal_mask_and_loop`: Requests a signal bit from the operating system through library calls, stores it in a structure, and combines it with a second bit into a mask stored in another field.
- `service_flagged_library_request_loop`: Loops over a status flag byte in a structure, setting its low bit and leaving the loop if it was already set.
- `clear_low_status_flag_bits`: Clears the two lowest status bits in a flag byte of a structure and returns.
- `prepare_speech_request_and_send`: Fills in the speech I/O request block with pointers to tables, sample length and channel fields, and a rate computed by dividing a constant by a stored value.
- `prepare_speech_block_and_expand`: Copies a block of input bytes into a work buffer, fills in the speech request block with table pointers and a rate computed by division, and then expands table-coded samples into one of two buffers.
- `expand_table_coded_samples_loop`: Expands table-coded samples into the current output buffer by summing three lookup-table values per sample, with counters held in a structure.
- `synthesize_speech_frames_loop`: Continues the speech sample synthesis loop: reads a control byte for each frame, selects a per-frame table pointer from its control bits, and mixes three table lookups per sample into the active output buffer.
- `synthesize_speech_frames_indexed`: Walks a frame table with per-frame control bytes and synthesizes samples with the same three-lookup mixing and buffer toggling as its neighbour.
- `finish_speech_frame_synthesis`: Completes a speech synthesis pass by mixing the remaining samples of the current frame, then reads frame control bytes until a negative byte ends the stream.
- `step_speech_sample_phase`: Advances a phase counter that bounces between the ends of a range, then synthesizes speech samples into the output buffer with the same frame-driven mixing.
- `resume_speech_frame_stream`: Resumes frame-driven synthesis after a phase update by decrementing the sample counter and, when it passes zero, reading the next frame control bytes and mixing that frame's samples.
- `wait_io_per_channel_then_dma`: Calls the operating system's wait-for-I/O routine once for each entry in a channel list, after writing the audio DMA control register with the enable bit set when a status flag is found.
- `enable_audio_dma_if_flagged`: Writes the audio DMA control register with the enable bit set when a status flag in the structure is set, and returns.
- `update_channel_and_requeue_node`: Disables interrupts while it updates a per-channel byte and, when that byte changes and a matching list node is present, moves the node and queues it with a library call.
- `decrement_counter_and_enable_interrupts`: Decrements a byte in a structure and, if the original value was zero or one, writes a value to a global variable to enable interrupts via the INTENA register.
- `flush_channel_entries_and_dma`: Marks each entry in a channel block with a status bit and calls a library routine on it, then clears the block's active bit.
- `flush_channel_entries_from_offset`: Marks channel entries starting at a register-supplied offset with a status bit and calls a library routine on each, then clears the active bit.
- `wait_channels_then_release_dma`: Calls a library routine once per remaining channel, marking entries with a status bit when the routine reports them.
- `set_channel_status_bit`: Sets a status bit in a channel structure and returns.

### Message

- `run_entry_with_conditional_library_call`: Program entry routine that calls a helper, then, when the zero flag test is false, calls a second helper and a library function pointer loaded from a global table.
- `open_dos_library`: Opens the dos library through the exec library's old-open call and stores the returned handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen longwords from a fixed table in the structure into a caller-supplied buffer when that buffer pointer is non-null.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:intro

- `call_unnamed_intro_routine`: Program entry that makes one call to an unnamed routine and references the text IMP!. The decompiler gave no pseudo-code, so the body is known only from that call and the text reference.
- `touch_background_color_then_call`: Touches the background color register and calls a jump-table entry and an unnamed routine.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `decode_backward_copy_stream`: Decodes a compressed bit stream by repeatedly copying runs of bytes backwards from earlier output, with run lengths and distances read through lookup tables.
- `decode_backward_copy_variant`: Runs the same bit-serial backward-copy decoding as its neighbour, but with its own length and distance tables and a different offset term.
- `continue_backward_copy_decoding`: Continues the bit-serial backward-copy decoder: it reads a length and a distance from the stream through lookup tables, copies that run backwards in the output, and stops when the output pointer reaches the start of the buffer.
- `decode_backward_copy_five_run`: Enters the same backward-copy decoder with a starting run length of five and a distance table lookup.
- `decode_backward_copy_four_run`: Enters the backward-copy decoder with a starting run length of four and a distance table lookup.
- `decode_backward_copy_three_run`: Enters the backward-copy decoder with a starting run length of three.
- `decode_backward_copy_two_run`: Enters the backward-copy decoder with a starting run length of two and an initial distance index of zero.
- `decode_backward_copy_fresh_start`: Starts the backward-copy decoder with a distance base of two and reads run and distance codes from the bit stream before copying backwards.
- `decode_backward_copy_table_entry`: Enters the backward-copy decoder with a run length taken from a table at an index held in a register.

### TRANSLATOR.LIBRARY

- `create_and_register_library_base`: Library shared-code entry that calls the library's MakeLibrary routine and, when it returns a base, stores two values in that base and calls AddLibrary.
- `increment_count_clear_flag_bit`: Increments a 16-bit count field in a library structure and clears one flag bit in another byte of that structure.
- `decrement_count_and_call_when_flagged`: Decrements a 16-bit count in a library structure and, when the count reaches zero and a flag bit is set, calls a helper and returns its result.
- `unlink_node_or_flag_pending`: When a count field in a library structure is zero, removes a node from a doubly linked list and calls a library routine with a stored value.
- `unlink_node_and_call_library`: Removes a node from a doubly linked list by relinking its neighbours, then calls a library routine with a value stored in the node.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `scan_text_buffer_with_markers`: Fills a work area with space characters, then scans an input string, checking for a hash marker and for spaces before calling helpers on each segment.
- `return_low_word_value`: Returns the low word of a data register as an integer and does nothing else.
- `call_helper_clear_byte_return`: Calls a helper, clears one byte at a pointer held in a register, and returns the low word of a second register.
- `scan_until_marker_or_space`: Loops over an input string calling helpers, and stops when the byte before the cursor is a hash marker or a space character.
- `scan_segments_return_last_result`: Scans an input string for hash markers and spaces, calling helpers for each segment.
- `scan_to_backslash_or_backtick`: Reads bytes from a buffer until a backslash or grave-accent character is found, calling helpers at each boundary.
- `scan_delimited_bytes_with_helper`: Reads bytes from a buffer and calls a helper whenever a backslash or grave-accent delimiter is found.
- `stub_return_only_routine`: Contains only a return and performs no work, although the decompiler listed sixteen instructions for it.
- `skip_to_bracket_and_match`: Advances through a buffer to the first left-bracket character, then compares the following bytes with a second string until they differ or a right-bracket is reached.
- `scan_backward_flagged_characters`: Scans a buffer backwards, skipping characters whose table entry has a flag bit set and stopping at the first unflagged character.
- `return_without_action_2`: Returns immediately and performs no work.
- `scan_backward_unflagged_characters`: Scans a buffer backwards for the first character whose table entry has the flag bit set.
- `match_text_skipping_flagged_chars`: Reads characters from a buffer, skipping those whose table entry has the flag bit set, and compares the stream with a second string byte by byte until they differ.
- `compare_text_until_flagged_char`: Reads characters from a buffer until one whose table entry has the flag bit set is found, comparing each character against a second string as it goes.
- `copy_text_segment_with_wrap`: Measures a text segment up to a backslash or grave-accent delimiter and copies it into the output buffer if it fits the remaining width.
- `copy_text_segment_flag_marker`: Copies a text segment into the output and, when the segment does not fit, ends the output at a word boundary.
- `copy_counted_bytes_set_flag`: Copies a counted run of bytes from one buffer to another, then sets a flag bit when the last byte has a table flag and the following byte is a grave-accent character.
- `terminate_text_at_word_boundary`: Walks backwards from the current position until it finds a character whose table entry has the flag bit set, then writes a zero terminator at that point.
- `dispatch_by_symbol_table_match`: Searches a table of symbol characters, including hash, asterisk and punctuation marks, for a given byte.
- `return_from_subroutine_3`: Performs a standard subroutine return to the calling routine.
- `stub_empty_return_slot`: Contains only a return in the decompiled output, so it is an empty stub with no visible effect on the library.
- `forward_call_to_helper`: Calls one unnamed helper routine and returns.
- `call_helper_while_flag_set`: Calls a helper once, then keeps calling it while a flag bit in a register is set.
- `forward_call_to_single_helper`: Calls a single unnamed helper and returns, forwarding through a handler pointer.
- `check_letter_suffix_after_helper`: Calls a helper and, when a flag bit is set, calls it again and checks the returned character against two letter codes.
- `stub_return_plain_entry`: Returns immediately; a trivial entry point with no work.
- `match_letter_codes_after_helper`: Calls helpers that return characters and compares them against a sequence of letter codes, taking a branch for each match.
- `match_n_g_s_letter_sequence`: Calls helpers and checks for the letter pattern N, then G, then S, calling helpers for each match before it returns.
- `stub_return_shared_exit`: Empty shared exit that returns immediately, called by two unnamed routines.
- `call_unnamed_routine`: This library routine calls another unnamed routine before returning.
- `execute_library_subroutine`: This library routine performs a call to another unnamed routine.
- `invoke_library_handler`: This library routine invokes another unnamed routine and then returns.
- `transfer_control_library`: This library routine transfers control to another unnamed routine.
- `call_helper_repeatedly_while_flag`: Calls a helper repeatedly for as long as a flag bit in a register remains set, then returns.
- `dispatch_library_call`: This library routine calls an unnamed routine and returns.
- `call_helper_while_second_flag`: Calls a helper repeatedly while a second flag bit in a register remains set, then returns.
- `call_helper_then_test_flag`: Calls a helper and then tests a flag bit in a register before returning.
- `stub_return_empty_exit`: Empty exit point that returns immediately.
- `normalize_text_into_line_buffer`: Copies characters from an input string into a line buffer, replacing control and delete characters with spaces and folding lowercase letters to uppercase.
- `write_space_before_pointer`: Writes a space character two bytes before a pointer held in a register, then returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d1, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 32 to (a2)+, then copies 32 to (a2)+, then returns.
- `insert_marker_after_table_matches`: Scans a text buffer for two-byte codes that appear in a table, and for each match shifts the rest of the buffer up by one byte and inserts a marker byte.
- `insert_marker_byte_at_cursor`: Shifts the bytes from the cursor to the start of the buffer up by one position, writes the marker value fifty-two at the insertion point, and stores the new cursor pointer.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:PAUSE

- `wait_while_pause_bit_set`: Busy-waits while a bit in a global variable remains set, then returns.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:delphine

- `allocate_and_relocate_hunks`: Program entry that allocates memory, looks up the current task and walks a list of executable hunks, comparing type codes while it sets up each block.
- `stub_empty_return_routine`: The decompiled body is empty apart from a return, so this routine has no visible behaviour.
- `delay_count_down_loop`: Counts a register down to zero in a tight loop and then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 80 into d0.
- `decode_run_length_bytes_with_color`: Decodes a run-length byte stream: a zero byte introduces a counted run whose value is repeated into the output.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_wrap_to_space`: Decrements a byte once per count step, replacing a result of zero with a space character, until the count passes minus one.
- `execute_invalid_instruction`: Contains bad instruction data that prevents normal execution flow.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:add21k

- `initialise_program_state_block`: Program entry that calls a helper, clears twenty-three longwords relative to a base register, and stores the initial register value and a global.
- `open_system_libraries_and_init`: Opens the operating system libraries through library calls, raising an alert when a required open fails.
- `stub_return_empty_init`: Empty routine that returns immediately with no work.
- `allocate_and_open_dos_file`: Allocates a block of memory through a library call, initialises its fields and stores a tag value.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `allocate_and_copy_named_string`: Computes a buffer length from a stored string, requests memory through a library call, and copies a string into the new buffer up to its terminator.
- `append_string_with_fixed_cap`: Finds the end of a destination string and appends a source string to it, copying at most a fixed number of bytes and writing a terminator if the copy is cut short.
- `append_string_with_count_limit`: Finds the end of a destination string and copies a source string onto it, stopping at a terminator or when a count held in a third argument expires.
- `copy_string_pad_zeros`: Copies bytes from a source string up to a count or a null terminator, then fills the rest of the count with zero bytes.
- `open_icon_library_window_entry`: Opens a library by name and, on success, looks up a window-related entry and stores two values in the calling structure.
- `run_optional_callback_then_release`: Calls a stored callback when one is set, then makes one more unnamed call and returns.
- `run_optional_callback_then_finish`: Calls a stored callback when one is set, then makes one more unnamed call and returns.
- `release_program_resource_tables`: Releases resources held in the global structure: when a table is set it calls a helper for each entry and then frees the table.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_indexed_table_entry`: Checks an index against the table size and, when the entry is valid, calls a helper to close it unless a flag bit is set.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_vector_from_struct`: Calls a library routine at a fixed negative offset from a base pointer stored in a structure.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_from_struct`: Calls a library function through a base pointer held in a structure, at a fixed negative offset, and returns no value.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_through_pointer`: Calls a library routine through a base pointer loaded from a global, at a fixed negative offset, and returns no value.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_routine_by_pointer`: Calls a library routine through a pointer in a structure at a fixed negative offset, then returns.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 2 of 3).adf:paradox

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library_2`: Calls the exec library function OldOpenLibrary to load dos.library and returns the resulting library handle.
- `close_exec_library_handle`: Calls the exec library's CloseLibrary routine through the global library base and returns its result.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 3 of 3).adf:paradox

- `initialize_dos_output`: Opens the DOS library, attempts to output data, and then closes the library.
- `open_dos_library_3`: Calls the exec library function OldOpenLibrary to load dos.library.
- `close_library_via_pointer`: Calls the exec library function CloseLibrary using a pointer stored in a global variable.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 2 of 3).adf:bootblock

- `boot_check_dos_library_open`: Boot block entry that calls a library function and returns zero when the call gives a non-zero result, and minus one when it gives zero.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a2](Disk 3 of 3).adf:bootblock

- `check_library_open_status`: Calls a function via a pointer in a structure and returns a specific value based on whether the result is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_empty_runtime_entry`: Empty routine at a runtime address that does nothing when executed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_callbacks`: Checks a value in a table, calls another routine with a value from that table, and then performs several other calls before updating the table entry with a specific constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_no_operation`: This routine is empty and performs no actions before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_call_subroutines`: Clears a block of memory using a loop, updates several global values, and conditionally calls subroutines based on a flag in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_runtime_segment_data`: Executes seven instructions within a runtime segment to modify data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `execute_conditional_logic_chain`: Performs a series of conditional checks on a global pointer, calling various subroutines, and updating memory or configuration tables based on certain values.
- `read_input_flags`: The routine returns a byte derived from a global variable, testing bits 64 and 4 and combining the results with a logical OR. It references the CIAA_PRA and POTGOR hardware registers.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `set_global_byte`: Stores the value zerox4f into a global variable and touches the INTENA register.
- `update_and_scale_value`: Checks a value, multiplies a global variable by two, applies an XOR mask if a carry or zero condition occurs, and returns a value involving a division and remainder calculation.
- `handle_keyboard_serial`: The routine returns a byte computed from the bitwise complement of a global variable when a flag bit is set, performs a decrementing delay loop, and clears a flag bit in the global variable.
- `populate_table_with_offsets`: Copies a block of data from a source table and populates a destination table with a series of calculated values or offsets derived from input parameters.
- `write_config_table`: Stores a series of hardcoded word values into a table located via a pointer in a global variable.
- `write_config_table_2`: Stores hardcoded word and long values into a table located via a pointer in a global variable.
- `load_pointer_from_table`: Returns a value loaded from a memory address calculated from a global variable.
- `store_raster_coordinates_2`: The routine stores a value into a global variable and, if a second parameter is provided, writes several sets of offset coordinate values into a table.
- `store_raster_coordinates`: Waits for a specific raster line and then stores several sets of coordinate values into a table.
- `copy_source_to_dest`: Copies a sequence of words from a source buffer to a destination buffer located in a table.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_from_routine`: The routine performs a return from a subroutine.
- `update_display_raster_parameters`: Updates a table with new offset and bitmask values by calling a routine that waits for a specific raster line and then performs a word-based memory copy.
- `execute_nested_calls`: Performs a sequence of calls to another routine using values passed or stored on the stack.
- `get_and_increment_index`: Reads a byte from a table using an index, then increments that index stored in a pointer.
- `initialize_and_fill_table_2`: Clears a block of memory, then iterates through a table to clear or update entries with values retrieved from a recursive call or another routine.
- `fetch_indirect_short_word_2`: Retrieves a value from a memory location pointed to by an offset within a structure after performing a check.
- `fetch_indirect_short_word`: Retrieves a short word from a memory location pointed to by an offset within a structure.
- `fetch_table_value`: Calls another routine to retrieve a value from a global variable and then extracts a short from a pointer found at an offset within a table.
- `process_input_and_fetch_data`: This routine retrieves a byte from a global variable via bitwise testing and combines it with a short extracted from a pointer within a table.
- `read_input_flags_2`: Returns a byte derived from a global variable while testing certain bits and combining results with a logical OR.
- `update_entity_state_and_initialize_tables`: Updates status flags and scales a value based on bitwise checks of a structure member.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_4`: This routine consists of a single instruction that returns control to the caller.
- `loop_on_error_condition`: Checks a condition via another routine and enters an infinite loop if certain global values are non-zero and the condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_joystick_input`: Reads input from a joystick or controller and stores the result in a memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_status_updates`: Checks processor flags and global values to conditionally execute a sequence of calls and loops that poll for specific character values in memory.
- `scale_global_and_split_quotient`: If the 16-bit argument is positive, it doubles a global 32-bit state word, applies an XOR mask when the doubling carries out or yields zero, stores the result back, and returns the low word divided by the argument packed with the remainder in the high and low halves.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_memory_values`: Sets a 4-byte memory location to zero and a 2-byte memory location to all ones.
- `reset_memory_values_2`: Iterates through a linked list, calling a routine for each node, and then sets a 4-byte memory location to zero and a 2-byte memory location to all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding`: This routine is empty and performs no operations.
- `return_from_subroutine_5`: This routine performs a return from a subroutine.
- `return_from_subroutine_6`: This routine performs a single return instruction to exit the current function.
- `process_nested_function_chain`: Retrieves a value via a function call and uses it as an argument for a sequence of subsequent function calls.
- `fetch_and_increment_index`: Reads a byte from a table using an index provided by a pointer, then increments the value at that pointer.
- `initialize_and_fill_table`: Clears a block of memory, then iterates through a table to clear or update entries with values retrieved from a recursive call or or other routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_null_terminated_list`: Traverses a linked list until a null pointer is found and then calls another routine with a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_table_entries`: Iterates through a table to copy data into a destination buffer using a loop and a specific constant.
- `fetch_indirect_pointer_value`: Attempts to retrieve a value from a pointer located at a specific offset within a global structure.
- `get_value_from_pointer`: Attempts to retrieve a value from a pointer stored within a structure, returning a short integer from a specific offset.
- `fetch_structure_member`: Accesses a structure via a pointer and returns a two-byte value located at a fixed offset within that structure.
- `fetch_structure_member_2`: A routine consisting of 43 instructions that accesses a structure via a pointer and returns a two-byte value from a fixed offset.
- `initialize_game_state_tables`: Initializes a series of global pointers and clears or populates memory-resident tables, including a loop that zeros out a large block of memory.
- `clear_and_set_memory_constants`: Clears a four-byte memory location to zero and sets a two-byte memory location to all ones.
- `traverse_and_process_linked_list`: Iterates through a linked list by following pointers, calling another routine for each node, and then clears a four-byte memory location and sets a two-byte memory location to all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_linked_list`: Recursively calls itself using a pointer or value from a table and updates a global variable before setting a certain memory location to a maximum value.
- `validate_table_entries`: Iterates through a table to populate a buffer with values from another routine and checks if a specific count of entries matches a global limit.
- `search_table_for_matching_value`: Iterates through a table to find an entry where a specific part of a value matches, then recursively checks a sub-range or linked structure.
- `search_and_recurse_table`: Iterates through a table of structures to find a matching key, then recursively calls itself or another routine on a calculated range of values if a match is found.
- `search_and_recurse_table_2`: Iterates through a table of structures to find a matching key, then recursively calls itself or another routine on a calculated range of values.
- `execute_recursive_operations`: Calls another routine with a fixed value, and if successful, performs several recursive calls involving calculated or passed parameters.
- `decompress_bitstream_data_2`: Expands a compressed bitstream into a destination memory buffer using a combination of literal byte copying and bit-by-bit reconstruction.
- `decompress_bitstream_data`: Processes a bitstream to extract values and stores them into a memory buffer using a variable bit length.
- `decompress_variable_bitstream`: Extracts values from a bitstream using variable bit lengths and writes them to a memory buffer.
- `decode_bitstream_data`: Uses a bit reader to extract values and fills memory with values derived from registers and bitstream processing.
- `fill_memory_with_value`: Fills a block of memory with a value derived from a register and returns the value from the top of the stack.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `return_stack_value`: The routine returns the value currently held at the top of the stack.
- `search_and_clear_table`: Iterates through a table of structures to find a matching key, calls another routine, and clears the entries.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_and_terminate_node`: Checks if a value in a table is non-negative, then passes that value and two other derived values to separate routines before setting the original table entry to its maximum value.
- `validate_sequence_data`: Iterates through a buffer to copy bytes from a source while checking if a sequence of values matches a specific length criteria.
- `search_table_with_sub_element_validation`: Iterates through a table to find an entry where a specific field does not match a provided value, then performs a secondary loop to validate the entry by calling another routine against a sequence of sub-elements.
- `search_table_with_sub_validation`: Iterates through a table of structures to find a matching key, then executes a loop that calls another routine on a range of derived values to validate the match.
- `validate_sequence_data_2`: Performs multiple searches and validations within tables and buffers to return a status value.
- `calculate_product_value`: Retrieves two two-byte values from a memory location and returns their product as a three-byte integer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_batch_memory_operations`: Performs multiple sequential memory copies, multiplications, and data transfers involving various offsets and lengths.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_multi_step_math`: Executes a series of memory copies and multiplications to update several memory locations based on product results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_sequence`: Performs multiple routine calls and calculates a value by multiplying two short integers before calling another routine with three arguments.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_extended_sequence`: Executes a series of routine calls, performs a multiplication of two short values, and stores the result in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_short_sequence`: Calls a routine three times, calculates a product of two short values, and then calls two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_minimal_sequence`: Calculates a product of two short values, stores it in a global variable, and calls two other routines.
- `decompress_bitstream_variable_length`: Reads a sequence of bits from a source to reconstruct data using a variable-length encoding scheme.
- `extract_bit_stream`: Processes a stream of bits from a source to populate a destination buffer.
- `decompress_variable_bitstream_2`: Reads a sequence of bits from a source to reconstruct data and writes the resulting bytes into a destination buffer.
- `decompress_bitstream`: Reads a sequence of bits from a buffer to reconstruct data and fills a memory range with a specific two-byte value.
- `fill_memory_with_short`: Fills a memory range with a specific two-byte value using a loop.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `return_empty_stack`: The routine returns an uninitialized value from the stack.
- `clear_table_entries`: Iterates through a table to check for non-zero values, calls a routine to process them, and then clears the entries in the table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `null_operation`: This routine is empty and performs no actions.
- `handle_disk_error`: Attempts to locate a trackdisk device and enters an infinite loop if the device cannot be accessed.
- `infinite_loop_with_function_call`: Executes a loop that calls itself and another routine repeatedly without a termination condition.
- `update_graphics_parameters`: Performs several routine calls and calls a function with calculated offsets and a fixed value, likely intended for a display or certain hardware parameters.
- `calculate_dialogue_box_dimensions_2`: Iterates through a null-terminated string to find the longest line by counting characters and identifying line break symbols.
- `parse_dialogue_text`: Iterates through a null-terminated string to calculate text wrapping and line breaks.
- `render_dialogue_text_box`: Calculates the dimensions and screen position for a text box, then iterates through a string to draw characters and handle word wrapping.
- `calculate_dialogue_box_dimensions`: Calculates the width and height of a text box based on a scaling factor and centers it on the screen.
- `render_dialogue_text_stream_2`: Iterates through a character data table to fetch byte values and updates display parameters.
- `process_dialogue_text_stream`: Iterates through a character data table to fetch byte values and updates display parameters.
- `render_dialogue_text_stream`: Iterates through a character data table to fetch byte values and updates display parameters.
- `render_text_dialogue_box`: Iterates through a data table to fetch character bytes and updates display parameters.
- `parse_and_render_text_string_2`: Iterates through a sequence of character bytes to write them into a buffer, handling specific control characters and updating display parameters.
- `parse_and_render_text_string`: Iterates through a sequence of characters to write them into a buffer, performing recursive calls to handle specific character types and updating display parameters.
- `render_character_tile_sequence`: Iterates through a sequence of bytes to write character data into a buffer and updates display parameters.
- `update_position_and_buffer_2`: Calculates a difference between a table value and a global variable, updates a global offset, and copies a sequence of 16 words from a source.
- `update_position_and_buffer`: Calculates a difference between a table value and a global variable, updates a global offset, and copies a sequence of 16 words from a source table to a destination buffer.
- `copy_cstring`: Copies a null-terminated byte string from a source pointer to a destination pointer, byte by byte, and terminates the destination with a zero byte.
- `search_string_table`: Iterates through a table of pointers to strings and uses string comparison to find a match for a provided string.
- `find_trackdisk_device`: Calls the exec library to locate the trackdisk.device and extracts a value from a device structure, returning it combined with an input register.
- `locate_trackdisk_device`: Locates the trackdisk device and attempts to perform operations on it, returning a failure code if the device cannot be accessed or if a loop limit is exceeded.
- `allocate_and_copy_buffer`: Allocates a memory block of a specific size, attempts to copy data into it, and returns a pointer to the data or a failure value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_linked_list`: Traverses a linked list of pointers until a null pointer is reached and performs bitwise operations on a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_with_parameters`: Calls another routine using three values retrieved from a table and a global pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `display_disk_error_message`: Updates various hardware or graphics parameters and enters an infinite loop while displaying a text string regarding disk insertion.
- `execute_infinite_loop_with_calls`: Executes a loop that calls another routine repeatedly without a termination condition.
- `update_graphics_parameters_2`: Performs several routine calls and calls a function with calculated offsets and a fixed value to set display or hardware parameters.
- `calculate_text_layout_and_render`: Iterates through a null-terminated string to find the maximum line length by searching for a specific delimiter character.
- `parse_and_render_text`: Iterates through a null-terminated string to calculate text dimensions and layout parameters.
- `render_justified_text_block`: Calculates character spacing and line breaks to justify a string of text within a defined bounding box.
- `render_justified_text_string`: Calculates character spacing and line breaks to justify a string of text within a defined bounding box.
- `render_string_with_kerning`: Iterates through a character string to retrieve glyph data from a table and writes it to a display buffer.
- `render_text_string_to_buffer`: Iterates through a character string to retrieve glyph data from a table and writes it to a display buffer.
- `render_text_input_sequence_2`: Iterates through a data table to retrieve character values and updates a display buffer.
- `render_text_input_sequence`: Iterates through a data table to retrieve character values and updates a display buffer.
- `process_text_input_buffer_2`: Validates and stores alphanumeric characters into a character buffer while handling backspaces.
- `process_text_input_buffer`: Validates alphanumeric characters and backspaces to manage a character buffer.
- `process_input_buffer`: Reads character data from an input source and stores it into a buffer while performing recursive updates.
- `execute_recursive_call`: The routine calls itself and then calls another routine before returning.
- `update_position_and_table`: Calculates differences between values in a table and global variables, updates those globals, and copies data from the table into another memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `find_list_tail`: Traverses a linked list to find the last element and performs a bitwise OR operation on a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_state`: Calculates differences for two values, updates global counters, and copies sixteen words from a table into a destination buffer.
- `copy_string_from_scaled_pointer`: Copies a null-terminated byte string to a destination, where the source address is derived by multiplying the first parameter by four.
- `search_string_table_by_index`: Iterates through a table of pointers to structures, where each structure contains a string and a secondary value.
- `get_trackdisk_device_id`: Locates the trackdisk.device using the exec library and retrieves its device number from the device structure.
- `initialize_disk_device`: Retrieves a device ID and attempts to initialize a disk device through a loop that checks for specific status bits.
- `allocate_and_read_buffer`: Allocates a memory buffer, attempts to read data into it from a source, and stores a value from the buffer into a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_mask`: Performs a bitwise OR operation between a 16-bit value and a global variable.
- `reinitialize_string_structure_2`: Resets a string structure's metadata, sets specific flags, and writes a constant string or a new pointer back to a global structure.
- `sync_table_entries`: Compares a string from a table against a global string and, if they match, calculates the size of a linked list of pointers to build a new table structure.
- `sync_dos_file_metadata`: Compares a string from a file structure against a global string and, if they match, traverses a linked list of pointers to calculate a count.
- `execute_indirect_calls`: Performs indirect calls to a target address retrieved from a table using a negative offset and calls a library function via a global base.
- `copy_null_terminated_string`: Copies a null-terminated string from a source address to a destination address.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `clear_and_update_globals`: Zeroes out a block of memory, updates a global or a table entry with a constant, and then copies a global variable to a new memory location.
- `return_immediately_stub`: The routine consists of a single RTS instruction and performs no work.
- `parse_command_string`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `copy_string_with_limit_3`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `resolve_indirect_table_entry`: Fetches a function pointer from a nested structure via an indirect table lookup and executes it.
- `store_and_parse_string`: Stores a passed parameter into a global variable and then calls a routine to parse a format string.
- `increment_and_store_byte`: Loads a pointer from a global variable, increments that global variable, and stores a byte into the memory location pointed to by the loaded pointer.
- `recurse_with_concatenation`: Recursively calls itself while returning a 64-bit value formed by concatenating a register's previous contents with the new return value.
- `parse_format_string`: Parses a formatted string containing specifiers for integers, floating-point numbers, and strings.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `execute_conditional_pointer`: Checks if a value at a certain offset from a pointer is non-zero and, if so, executes the function pointer located at that memory location.
- `execute_conditional_table_logic`: Checks various memory locations or table-based pointers and executes corresponding functions or jumps if the pointers or certain flags are non-null.
- `nop_padding_sequence`: The routine is empty and performs no operations.
- `close_file_entry`: The routine checks if an index is within valid bounds and if a pointer in a table is non-zero, then either calls another routine or clears the pointer and sets a status flag.
- `execute_indirect_dos_function`: The routine fetches a function pointer from a table using an offset and executes it.
- `call_dos_close`: Calls a function pointer located within a system library structure to close a file.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_indirect_function_4`: Retrieves a function pointer from a nested table structure and executes it.
- `call_dos_current_dir`: Calls a function pointer located within a system library structure to retrieve the current directory.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_dos_input`: Performs an indirect call to a function retrieved from a table using an offset.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_nested_function_pointer`: Fetches a function pointer from a nested structure within a table and executes it.
- `execute_indirect_function`: Fetches a function pointer from a nested structure within a table and executes it.
- `execute_indirect_function_pointer`: Retrieves a pointer from a table using an offset and executes the function located at that address.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `execute_indirect_function_pointer_2`: Fetches a pointer from a nested structure within a table and executes the routine located at that address.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `execute_indirect_function_pointer_3`: Retrieves a function pointer from a nested table structure and executes it.
- `execute_indirect_function_2`: Fetches a function pointer from a nested structure within a table and executes it.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_indirect_function`: Loads a function pointer from a table offset and calls it.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_table_pointer`: Fetches a pointer from a table using an offset from a base address and executes the function located at the resulting destination.
- `execute_indirect_function_pointer_4`: Fetches a pointer from a table located at an offset from a base address and executes the function at that location.
- `populate_nested_structure`: Allocates or retrieves a structure and populates its various fields with passed parameters and results from other routine calls.
- `process_nested_data_structure`: Performs a recursive data copy and updates specific bytes and words within a structure based on the result of a nested function call.
- `perform_recursive_structure_copy`: Copies data between memory locations and calls itself recursively with an incremented argument.
- `recursive_pointer_offset`: The routine calls itself with an argument incremented by a constant value.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `execute_nested_function_pointer_2`: Fetches a function pointer from a nested table structure by applying an offset to a value retrieved from a base pointer and executes it.
- `execute_indirect_function_3`: Fetches a function pointer from a nested table structure and executes it.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `execute_indirect_pointer`: Fetches a function pointer from a nested offset within a structure and executes it.
- `execute_indirect_function_pointer_5`: Fetches a function pointer from a nested offset within a structure and executes it.
- `execute_indirect_call_7`: Performs an indirect call to a function pointer retrieved from a table using a specific offset.
- `execute_indirect_call`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `execute_table_pointer`: Calls a function through a pointer located at a specific offset within a table.
- `execute_indirect_call_2`: Calls a function through a pointer located at an offset within a table.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `execute_indirect_call_3`: Retrieves a function address from a table and executes it.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `execute_offset_pointer`: Uses a pointer retrieved from a table to perform an indirect jump or call.
- `execute_indirect_call_4`: Uses a pointer from a table to perform an indirect jump or call.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `execute_indirect_call_5`: Executes a routine by dereferencing a pointer found at a specific offset in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reinitialize_string_structure`: Resets a string structure's metadata, sets certain flags, and writes a constant string or a new pointer back to a global structure.
- `return_from_sequence`: This routine consists of a single instruction and is called by a minimal sequence execution function.
- `execute_sequence`: A routine consisting of 124 instructions called by another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_instruction`: The routine performs no operations and returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_build_linked_list`: Compares two null-terminated strings and, if they match, traverses a linked structure to calculate its depth.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length_2`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `compare_two_byte_strings_2`: Compares two byte strings byte by byte and returns a value indicating their lexicographical relationship.
- `safe_memory_copy_2`: Copies a specified number of bytes from a source to a destination while handling overlapping memory regions.
- `clear_and_update_globals_2`: Zeroes out a block of memory, updates a table entry with a constant, and copies a global variable to a new location.
- `return_void`: This routine performs no operations and immediately returns to the caller.
- `parse_command_string_2`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `copy_string_with_limit_4`: Calculates the length of a source string and copies its bytes to a destination buffer until a null terminator is reached or a maximum length is hit.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros_3`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `update_global_and_parse`: Stores a passed parameter into a global variable and then calls a routine to parse a format string.
- `increment_and_store_byte_2`: Loads a pointer from a global variable, increments that global variable, and stores a byte from a register into the memory location pointed to by the loaded pointer.
- `divide_and_concatenate`: Performs a recursive division and returns a 64-bit value by concatenating the previous register contents with the result of the division.
- `parse_format_string_2`: Parses a formatted string containing specifiers for numeric values, including integers, floating-point numbers, and strings.
- `combine_signed_quotient_and_remainder`: Determines the sign of a result based on two signed inputs, calls a 32-bit division routine, applies the sign to the quotient, and concatenates it with the remainder into a 32-bit value.
- `call_and_concatenate_result`: Calls another routine and returns a 64-bit value by concatenating the original contents of a register with the new value left in that register by the called routine.
- `execute_recursive_division`: Calls a routine that performs a 32-bit integer division and then returns.
- `perform_32bit_division_2`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `call_dos_close_2`: Calls a function pointer located within a structure to execute a DOS close operation.
- `call_dos_createproc`: Calls a function pointer located within a structure to execute a DOS create process operation.
- `call_dos_currentdir`: Calls a function pointer located within a structure to execute a DOS current directory operation.
- `call_dos_duplock`: Calls a function pointer located within a structure to execute a DOS duplicate lock operation.
- `call_dos_examine`: Calls a function pointer located within a structure to execute a DOS examine operation.
- `call_dos_info`: Calls a function pointer located within a structure to execute a DOS info operation.
- `call_dos_input_2`: Loads a function pointer from a table offset and calls it to perform an input operation.
- `call_dos_lock`: Loads a function pointer from a table offset and calls it to perform a lock operation.
- `call_dos_open`: Loads a function pointer from a table offset and calls it to perform an open operation.
- `call_dos_output`: Loads a function pointer from a table offset and calls it to perform an output operation.
- `call_dos_read`: Loads a function pointer from a table offset and calls it to perform a read operation.
- `call_dos_seek_2`: The routine loads a function pointer from a table offset and calls it to perform a seek operation.
- `call_dos_seek`: Loads a function pointer from a table offset and calls it to perform a seek operation.
- `call_dos_unlock`: Loads a function pointer from a table offset and calls it to perform an unlock operation.
- `call_dos_write_2`: The routine loads a function pointer from a table offset and calls it to perform a write operation.
- `call_dos_write`: Loads a function pointer from a table offset and calls it to perform a write operation.
- `execute_indirect_function_call`: Fetches a function pointer from a nested offset within a structure and executes it.
- `call_indirect_via_offset`: Fetches a pointer from a table using an offset and executes the function at that address.
- `call_indirect_function_2`: Performs an indirect call to a function address retrieved from a table using an offset.
- `call_indirect_function_3`: Executes a call to a subroutine located via a pointer within a table.
- `call_indirect_function_4`: Calls a function by calculating an address from a base pointer and a specific offset.
- `populate_nested_structure_2`: Initializes a data structure by assigning multiple values to specific offsets and performing recursive calls based on the input parameters.
- `update_object_property_struct`: Copies data from a source structure into a destination structure and updates specific byte and word offsets.
- `execute_indirect_chain`: Performs an indirect jump using a table pointer and calls a routine to retrieve the current directory.
- `increment_pointer_chain`: The routine calls another routine with an offset address.
- `call_indirect_function_5`: Uses a pointer from a table to perform an indirect jump to another routine.
- `call_indirect_function_6`: Performs an indirect call to a routine based on a calculated offset from a table entry.
- `call_indirect_function_12`: The routine retrieves a function pointer from a table and calls it.
- `call_indirect_function_7`: Retrieves a function pointer from a table and calls it.
- `call_indirect_function_8`: Executes an indirect call to a routine using a value from a table.
- `call_indirect_function_9`: Performs an indirect call to a subroutine using an offset from a pointer in a table.
- `call_indirect_function_alt`: The routine calls a function through an indirect pointer derived from a table.
- `call_indirect_function_10`: Calls a function through an indirect pointer derived from a table.
- `call_indirect_function_11`: Performs an indirect call to a routine using a calculated address from a table.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `execute_indirect_call_6`: Calls a routine through a pointer retrieved from a nested structure.
