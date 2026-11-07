# Game 1630 (james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01630). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (950976 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 101 raw-word lines |
| Functions named and described | pass, 221 of 221 named (0 author symbols), 221 of 221 described (4 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1770 of 1770 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7215 of 8799 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 43898 |
| code | 32220 |
| data | 733546 |
| unused | 141312 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf:bootblock | 1 | 28 | 1 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf boot-block load | 1 | 92 | 2 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf boot-block load | 1 | 1438 | 5 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5486 | 163 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1361 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 394 | 22 |

## Cases handled

- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf: custom-format (non-DOS) disk, 1482 non-empty blocks: matched against track-loaded segments
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf: the boot block reads 15360 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1348 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 152 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 270 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf: 3 logged loads cover 27648 bytes of the image
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf: 12 track-loaded blocks found in recovered runtime segments and classified through them
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf: blocks 4-1215 (bytes 2048-622592) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.65)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf: blocks 1219-1238 (bytes 624128-634368) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.54)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf: blocks 1242-1261 (bytes 635904-646144) outside the filesystem hold data, not code (opcode density 0.020, entropy 5.71)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf: blocks 1265-1284 (bytes 647680-657920) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.50)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +4 legend]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf: blocks 1288-1431 (bytes 659456-733184) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.24)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf:bootblock

- `submit_boot_io_requests`: As the boot block entry point, it fills in I/O request structures with command codes, offsets and lengths, submits them through the exec library's I/O call, enables DMA through the custom chip, and then returns.

### logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +4 Legend].adf boot-block load

- `decode_packed_stream_backwards`: Reads control bits from a packed stream and either copies literal bytes or repeats back-references, writing into a buffer that is filled from its end towards its start until the write pointer reaches the beginning.
- `test_bit_buffer_remaining`: Tests whether a shifted bit buffer still holds bits and returns in both cases, so it has no observable effect.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `read_bits_from_buffer`: Shifts bits out of a 32-bit bit buffer one at a time, refilling the buffer from memory when it empties and carrying the shifted-out bit into the top, looping for a count held in a register.
- `call_routine_when_cleared`: Returns zero, and calls an unnamed routine with a fixed flag value first only when a global word is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_blitter_copy_with_masks`: Waits for the blitter to go idle, then sets its control, masking and modulo registers for a copy operation and writes its size register to start it.
- `blit_shifted_tile_strip`: Computes a shifted address from a global value and waits for the blitter to finish before each of several blit operations, writing control and size registers for a strip of tiles.
- `blit_masked_tile_strip`: Computes a shifted address and waits for the blitter between blit operations that use separate first and last word masks and a second source channel, writing the strip's control and size registers.
- `copy_backward_block_with_fixups`: Copies bytes and longwords backwards from decreasing addresses into a destination buffer, adjusting a character pointer as it goes.
- `emit_text_characters`: Reads characters from a string: a carriage-return code advances the output position and loops back, a zero byte ends the routine, and other characters are passed to unnamed routines for output.
- `advance_text_past_spaces`: Handles space characters in a text stream by advancing the text pointer and shifting an accumulator value, then calls an unnamed routine for the next character.
- `load_text_byte_into_state`: Loads the next byte of a text stream into a global state word and advances the text pointer, then continues the character loop.
- `accumulate_text_byte_value`: Adds the next byte of a text stream to a global state word and advances the text pointer, then continues the character loop.
- `parse_text_string`: Parses a string by checking for specific control characters and updating a global pointer or value accordingly.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `find_first_set_bit_offsets`: Scans two six-word tables for the first word with bit zero set and the first with bit one set, then derives shift counts from their positions and builds a shift value in a global word for the blitter.
- `blit_rows_with_pointer_steps`: Waits for the blitter to go idle, then repeatedly sets its source and destination pointers, starts a blit of the given size and advances the pointers by a row step, for a count taken from the input.
- `configure_blitter_copy_defaults`: Writes fixed mask, modulo and control values into the blitter register variables for a copy operation, without starting a blit.
- `start_single_channel_blit`: Waits for the blitter to go idle, writes the mask, control and modulo values for a single-source operation, sets the destination pointer and size from its inputs, and starts the blit.
- `start_blitter_copy_from_pointers`: Waits for the blitter to go idle, writes the mask, control and modulo values for a copy, sets source and destination pointers from its inputs and starts the blit with a size argument.
- `blit_four_bitplane_offsets`: Waits for the blitter, then for a non-zero input issues four blits with the same control value, each to a destination offset separated by a fixed plane stride from a base computed from the input.
- `set_blit_fixed_state`: Stores fixed values into global words and returns.
- `forward_call_then_return`: Calls one unnamed routine routine and then returns; the global assignment it makes is a self-copy with no effect.
- `write_stepped_longs_to_record`: Calls several unnamed routines, then writes four long values into a table, each one stride apart from a base read from an object, with a fixed offset added when a flag is set, and stores a second pair of values in the record.
- `pick_background_colour_from_flags`: Chooses a colour value from four flag tests and writes it to the background colour register, then calls unnamed routines to finish the update.
- `compute_scroll_offsets_from_position`: Subtracts reference values from two position words, derives a sub-pixel nibble and a coarse offset from each difference, and stores them in fields of the object.
- `iterate_rows_calling_routine`: Loops over a grid of rows, calling an unnamed routine fourteen times per row and advancing counters, then calls a final unnamed routine before returning.
- `lookup_nibble_table_value`: Takes the high nibble of a value, scales it, looks up a word from a fixed table and doubles the result into a global word.
- `compute_bitmap_offset_from_position`: Computes a bitmap offset from a position word by taking its aligned row part, multiplying it by a stride of 38, adding a base value from the object and a fixed displacement, then aligning the result.
- `store_position_with_offset`: Adds a fixed offset of 608 to a global value when it is non-zero, stores the result in the object, and then calls three unnamed routines.
- `store_negative_offset_position`: Subtracts a fixed offset of 608 from a global value when it is non-zero, stores the result in the object, and then calls unnamed routines.
- `store_doubled_value_and_loop`: Stores double a global value into the object, calls an unnamed routine, and then calls a second unnamed routine fifteen times in a loop.
- `store_offset_value_and_loop`: Stores a global value plus 36 plus itself into the object, calls an unnamed routine, and then calls a second unnamed routine fifteen times in a loop.
- `compute_blitter_row_modulo`: Computes aligned row and column values from an input and fixed constants, after calling unnamed routines, and writes the results to the blitter's modulo registers.
- `initialise_static_state_table`: Writes a fixed set of constant values and table addresses into a block of global memory words, including a pointer to a global counter.
- `write_copper_list_pointers`: Computes a base address from an object and writes long values stepping by a stride, then updates the high word of the second copper list location register.
- `swap_double_buffer_pointers`: Alternates a global between two buffer address pairs, writing the matching pair of values each time it is called.
- `dispatch_by_index_table`: Jumps through a table of routine addresses, selected by a global index, treating the jump as a call.
- `call_table_entry_by_index`: Calls one routine from a table of routine addresses selected by a global index, then returns.
- `start_shifted_blit_with_masks`: Computes blitter shift and control values from two inputs and, when a global is non-zero, sets masks, pointers and modulos and starts a blit of the computed size.
- `start_shifted_blit_control`: Builds a blitter control word from a shift value and fixed bits, waits for the blitter, then writes the control and pointer registers for a blit.
- `dispatch_after_two_routines`: Calls two unnamed routines, then jumps through a table of routine addresses selected by a global index, treating the jump as a call.
- `dispatch_two_routines`: Executes two consecutive routines and then performs an indirect call to a routine selected from a table using a global index.
- `expand_packed_tile_into_buffer`: Expands a tile from packed source bytes into a destination buffer, writing each source word to two rows of the buffer for a sixteen-row tile.
- `call_routine_for_each_index`: Runs a flag-dependent setup, then calls an unnamed routine for each index from four down to zero, and returns the value from the last call.
- `release_entries_matching_key`: Scans a list of twenty-six-byte entries and, for each active entry whose key equals a global index, calls an unnamed routine and marks the entry inactive.
- `process_matching_list_entries`: Scans the same entry list with several range and status checks, calling an unnamed routine for entries that qualify and clearing their active words.
- `update_entry_positions_from_table`: For active entries, sets a position word to a global offset plus a value from a table indexed by a field, and calls unnamed routines to finish each update.
- `find_free_list_slot`: Searches up to 32 ten-byte entries for one whose word at a fixed offset is zero, and calls an unnamed routine when none is free.
- `set_state_find_free_slot`: Writes fixed values into global words and then searches a ten-byte entry list for a free slot, calling an unnamed routine with the result.
- `copy_record_fields_after_wait`: Waits for the blitter to go idle, copies word fields from a source record into the object, and derives position values from them.
- `reset_entry_table_slots`: Sets a table base global, calls four unnamed routines, and then clears three global words.
- `clear_active_flags_in_table`: Walks thirty-two twenty-six-byte entries from a table base and sets each entry's active word to zero.
- `invert_global_if_underscore`: Checks if a global variable contains a specific character and, if so, sets the variable to a bitwise inverted zero.
- `load_colour_longs_from_table`: Copies a block of long values from a table into global memory words, then calls an unnamed routine.
- `spin_forever_trap`: Loops forever without doing any work, so the routine never returns; it appears to be a trap rather than a real routine.
- `wait_until_flag_set`: Repeatedly calls an unnamed routine until a global flag becomes non-zero, then clears the flag and returns.
- `init_audio_channel_registers`: The facts list the four audio channel location and length registers and the DMA control register.
- `read_vertical_beam_position`: Reads two raw custom-chip position bytes, recombines them into a position value and masks the result to nine bits.
- `disable_interrupts_init_copper`: Writes a value that clears all interrupt enable bits, sets up copper list pointers and then calls six unnamed routines to initialise the system state.
- `toggle_cia_bit_around_call`: Sets bit six of a global control value, calls an unnamed routine, clears the same bit and stores the value eight in a global word.
- `handle_interrupt_request_bits`: Tests bits of a pending interrupt mask in a fixed order, clearing or adjusting the mask and calling unnamed routines for the bits that are set.
- `decode_keyboard_serial_byte`: When a state bit is set, shifts and inverts a keyboard serial byte and calls unnamed routines for its handling, as the listed keyboard serial register indicates.
- `service_interrupt_request_flags`: Tests bits of a global flag word and calls unnamed routines for the set bits, then updates the flag word.
- `mask_interrupt_enable_bits`: When a global word is zero, masks it against a fixed bit set and returns; otherwise it continues into a loop over the enable bits.
- `mask_interrupt_request_high_bits`: Masks a global word against a fixed set of high bits and returns.
- `return_without_effect`: Returns immediately without doing anything.
- `read_joystick_port_state`: Reads the joystick movement register and the fire-button bit of the CIA port register, derives a direction and button state from bit tests, and calls two unnamed routines.
- `match_input_against_string_table`: Compares an input byte with entries of a string table, advancing through the table until a terminator, and calls unnamed routines when a match is found.
- `toggle_state_on_letter_d`: When a state word is set and the input byte equals a fixed letter code, inverts a global flag, clears two counters and updates the display control register.
- `count_down_ten_thousand_steps`: Counts down from 10000 in a loop, writing a masked word on each pass, and then calls an unnamed routine.
- `count_down_masked_steps`: Counts down from 10000 in a loop, writing a different masked word on each pass, and then calls an unnamed routine.
- `count_down_nibble_mask_steps`: Counts down from 10000 in a loop, writing a nibble-masked word on each pass, and then returns.
- `count_down_colour_mask_loop`: Counts down from 10000 in a loop, writing a low-nibble masked value on each pass, and then returns.
- `advance_shift_feedback_register`: Doubles a global word and adds one when a pair of bit tests indicates feedback, advancing a shift-register style pseudo-random state.
- `advance_scrambled_counter_state`: Steps a small counter up to five and then scrambles a global state word by multiplying it and reducing the result modulo a fixed value.
- `format_decimal_digits`: Converts a 32-bit value to decimal digit characters by repeated subtraction of descending powers of ten, writing each digit into a buffer.
- `stub_bad_instruction_data`: Contains only a bad-instruction marker and no meaningful code, so it is decoded data rather than a routine with behaviour.
- `set_playfield_control_registers`: Clears the playfield scroll register, writes a playfield priority value and a DMA control word, then calls an unnamed routine.
- `init_position_record_fields`: Stores a computed value and a constant into record fields, derives an adjusted offset from a register input, and then loops over five steps writing entries.
- `move_table_block_clear_source`: Copies a block of words from one table to another while zeroing the source words, then copies a second, longer block to further tables.
- `clear_word_block_set_value`: Clears a block of words starting from a fixed table, then sets a global value to thirty.
- `fill_tables_from_generator`: Fills a 32-entry table and two 256-entry tables with values returned by an unnamed routine, storing one or two values per entry.
- `scale_level_to_colour_nibble`: Scales a level value and an offset to a capped level of forty-seven, divides it by three and replicates the result across the nibbles, masking with a fixed value.
- `fill_colour_palette_table`: Fills palette table words by calling an unnamed routine per entry, writing each value to one table or to two tables depending on a flag bit.
- `empty_routine`: This routine is empty and performs no operations before returning.
- `process_nonzero_bytes_in_buffer`: Walks a byte buffer starting at an index, and for each non-zero byte calls an unnamed routine and advances the index.
- `find_first_nonzero_record`: Checks four table records spaced thirty-seven longs apart and returns the input value as soon as one has a non-zero first word.
- `advance_four_frame_counters`: When a global flag is non-zero, increments four counter words; when the first counter passes sixty-three it resets all four, sets a status bit and clears the flag.
- `step_marker_stream_pointer`: When a global is zero, reads the next pointer-stream byte; a negative marker sets a global to minus one, while other values advance the stream pointer when a counter matches.
- `init_sentinel_output_buffer`: Writes a sentinel byte at an output pointer, stores the pointer advanced by one, zeroes a counter word, then calls an unnamed routine.
- `write_cia_a_port_bytes`: Updates the control bits of a global flag word and writes two bytes from its argument to the CIA-A port registers.
- `write_cia_b_port_bytes`: Updates a control bit of a global word and writes one value and a fixed value to the CIA-B port registers.
- `call_routine_over_word_range`: Calls an unnamed routine repeatedly over a range of word values stepping by two, starting from the first field of a structure, and increments a global counter on each call.
- `build_sprite_entry_and_blit`: Waits for the blitter, sets its control values, computes position and attribute words from object fields, writes them into paired display-list slots and starts a blit of the object's data.
- `refresh_record_when_pointer_changes`: Compares a global pointer with a field of an object's record and, when it has changed, stores the new value and calls an unnamed routine with the record.
- `copy_byte_groups_strided`: Copies ten groups of four bytes from a source into a destination buffer at fixed strides.
- `start_blit_at_pointers`: Waits for the blitter, then writes source and destination pointers and a size word and starts the blit in a loop.
- `wait_for_global_zero`: Sets a global word to zero and then busy-waits until it reads zero, after calling a series of unnamed routines.
- `update_state_flag_branch`: Returns if a global is non-zero; otherwise it tests the sign of the value and calls unnamed routines, setting a status value of 255 in one branch.
- `clear_large_buffer_block`: Zeroes a block of several thousand words starting from a fixed table address, then calls an unnamed routine and updates a global according to its sign.
- `set_four_longs_minus_one`: Sets four global long words to all ones and returns.
- `start_blit_after_routine_calls`: Calls unnamed routines, waits for the blitter, writes its mask, control and modulo values and a size, then waits again.
- `compute_aligned_position_from_flags`: Derives a position value from flag bits, aligns it to a sixteen-unit boundary, adds a fixed inset and stores several globals as zero.
- `clear_global_block_words`: Sets a long run of global variables to zero in sequence, with no calls.
- `reset_flags_when_active`: When two flag words are set, it writes three global values and then checks a second pair of flags in the same way.
- `process_state_machine_step`: Runs a long sequence of conditional state updates, calling unnamed routines for timing and table work and writing a status value when an unnamed routine result is non-zero.
- `advance_clamped_state_counter`: Increments a state counter when it is active, masks its bits and clamps the result into a small range, applying a set of threshold tests.
- `step_state_toward_target`: Decrements a state word by two while it is non-zero and steps it toward a target value, clearing related globals when the target is reached.
- `update_masked_state_bits`: When a state word is non-zero, adds four to it, forces some bits with a mask and applies range tests that write a fixed value when they pass.
- `update_masked_state_bits_alt`: Follows the same state-word update pattern as its sibling routine, testing ranges after masking the state word.
- `reset_or_saturate_state_word`: Sets a state word to zero when a flag is clear, otherwise sets it to 255 under a sign test, and calls an unnamed routine in either case.
- `bias_and_clamp_high_word`: Adds a fixed bias of 40960 to a global value, takes the high word, caps it at fifteen and writes the result with a sign-dependent adjustment.
- `dispatch_state_by_mask_bits`: Tests bit fields of a state word and calls unnamed routines for the matching pattern, then writes a default value of six when a second input is zero.
- `clamp_masked_difference`: Takes the difference between two masked values, and when it is small enough, clamps it to the nearest allowed step before storing it.
- `step_mode_state_sequence`: Calls unnamed routines, returns early when a state is non-zero, and otherwise walks a mode sequence, clearing a state word and comparing limits.
- `bias_then_clamp_offset_wide`: Adds a fixed bias of 5462 to a global value, takes the high word and limits its magnitude to fourteen, unless it has a special value.
- `bias_then_clamp_offset_narrow`: Adds a fixed bias of 3280 to a global value, takes the high word and limits its magnitude to fifteen before negating it.
- `run_routine_sequence_updates`: Calls a fixed sequence of unnamed routines and copies global values between them.
- `step_value_by_difference_loop`: Computes the difference between two global values and repeatedly calls an unnamed routine while the difference is non-zero, stepping the value toward the target.
- `restore_saved_global_pair`: Calls an unnamed routine, then restores two global words from the values passed in.
- `restore_saved_pair_after_call`: Calls an unnamed routine, then restores two global words from the values passed in, as its sibling routine does.
- `clamp_signed_difference_step`: Subtracts a stored value from a global, and when the result is negative and within a small window, adjusts it by a doubled stored value.
- `clamp_sum_step_toward_two`: Adds a stored value to a global, and when the high word is in range, subtracts twice the stored value and sets the state to two.
- `clamp_negative_difference_to_four`: Subtracts a stored value from a global, and when the high word is in a negative range, sets the result to four.
- `clamp_sum_to_eight`: Adds a stored value to a global and, when the high word is below nine, sets the result to eight.
- `select_level_by_zero_flags`: Calls an unnamed routine, then chooses one of two level constants according to whether two global flags are zero.
- `select_level_mask_high_word`: Chooses one of two level constants from flag tests and masks the high word of a global value.
- `reduce_value_by_capped_step`: Reduces a signed global by a step capped at fifteen and, when the result is non-zero, applies a bit-test adjustment.
- `add_capped_signed_step`: Adds a signed step clamped into the range minus fifteen to fifteen onto a global value and returns.
- `select_range_value_from_flags`: Selects a value from a range according to global flags and a comparison against a fixed constant.
- `adjust_value_by_distance_limit`: Compares a global value with a limit and, when the difference is outside a small band, adjusts the value by a fixed step.
- `return_if_within_band`: Returns without effect when a global value is non-zero, or when a difference lies within a small band.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 1 into d6, then returns.
- `scan_table_for_match`: Steps a global value and calls an unnamed routine, then scans thirty-two table entries for a match against the value.
- `increment_counter_at_offset`: Clears and sets global values, calls an unnamed routine, then increments a byte counter at a fixed offset.
- `compare_distance_and_adjust`: Compares two global values, and when their distance is below a limit, derives a shift from the low bits and applies it.
- `clamp_position_within_range`: Computes the distance between two position words and, when it is within a limit, clamps the position to the nearer bound.
- `accumulate_indexed_bytes`: Accumulates bytes looked up through a global index into a running total, decrementing the count each step, and calls an unnamed routine.
- `update_limited_counter`: Calls an unnamed routine, resets two globals and compares a counter against a limit of one hundred twenty-four, applying a range-dependent update when it is exceeded.
- `select_state_by_threshold`: Tests a state value against a threshold and a set of exact values, then sets flag bits and calls unnamed routines for the matching case.
- `select_state_by_threshold_alt`: Follows the same threshold and exact-value tests as its sibling routine, setting flag bits for the matching case.
- `set_default_constants_by_flag`: Chooses between two constant values according to a global flag and writes a default value when the flag is clear.
- `set_constants_when_not_seven`: Sets a flag value to 255 when a global is non-zero, and assigns the constant seven and a fixed value when the state is not seven.
- `pick_constant_from_flag`: Assigns one of two constant values depending on whether a flag is zero, then clears a global.
- `set_constant_and_clear`: Assigns a fixed constant to a global word, clears another global and returns.
- `update_state_variable`: Modifies a global variable based on its current value, either incrementing it or setting it to a specific constant.
- `select_state_table_entry`: Handles a state value by testing several flags, selecting a table pointer or writing one of several values, and calling unnamed routines on each branch.
- `decrement_wrapped_counter`: Decrements a global counter and wraps it to a small range when it goes negative, then reads a value from a pointer.
- `lookup_indexed_table_word`: Computes a table index from two inputs shifted right by four, looks up a word and tests its top bits before storing the result.
- `walk_linked_index_table`: Walks an index table, following entries whose mask bits are clear, and stores the first matching entry in a global word.
- `clear_high_flag_in_range`: When a global value is non-zero and an input lies in range, clears its high flag bit and returns.
- `adjust_value_by_halving_steps`: Adjusts a global value by halving steps from an input, applying the step only when its sign and magnitude test passes.
- `add_input_to_global`: Adds an input word to a global value and returns.
- `subtract_halved_input`: Subtracts either a halved or a full input word from a global value, depending on a comparison of the two.
- `subtract_input_from_global`: Subtracts an input word from a global value and returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d6, then returns.
- `perform_empty_operation`: The routine executes a sequence of instructions that results in no state change or side effects before returning.
- `nop_padding_sequence`: The routine is empty and contains no executable instructions.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `compute_inverted_bit_mask`: Returns the bitwise inverse of an input word masked to two bits of a fixed pattern.
- `negate_global_when_bits_set`: Calls unnamed routines, and when the status bits of an input match, sets a global to minus three minus its previous value.
- `invert_global_when_bits_set`: Calls unnamed routines, and when the status bits of an input match, sets a global to three minus its previous value.
- `test_composite_status_bits`: Tests several bit combinations of a status word against fixed masks and returns a flag indicating which pattern matched.
- `check_input_pattern`: Compares an input word against a fixed pattern under a mask and returns zero on mismatch, otherwise a value derived from the input.
- `dispatch_unrecovered_jump_table`: Calls unnamed routines and then jumps through a table whose extent could not be recovered by the decompiler, so the destinations are not known.
- `multiply_byte_field`: Returns a byte field from the input structure multiplied by sixty-four.
- `stub_bad_instruction_data_2`: Contains only a bad-instruction marker after clearing two globals, so it is decoded data rather than a routine with behaviour.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `step_mirrored_offset_loop`: Repeatedly mirrors an offset around a fixed centre value, calling an unnamed routine on each step, and toggles a bit between steps.
- `process_object_list_entries`: Walks a list of object entries, copying fields between them and updating status bits according to a flag byte and a sign test.
- `select_colour_by_state_flags`: Chooses a colour value from state flags and sign tests and writes it to the background colour register, then calls an unnamed routine.
- `update_object_position_from_fields`: Calls an unnamed routine, then updates the object's position words from another object's offset and size fields unless a status bit is set.
- `count_flagged_table_entries`: Walks a table from a fixed start, counting each entry whose top bit is set and rewriting its word with an input value.
- `count_flagged_table_entries_2`: Walks a table from a fixed start, counting each entry whose top bit is set and rewriting its word with an input value, as its sibling routine does.
- `offset_position_from_reference_object`: Moves the object's horizontal position by half a size field of the reference object, minus a fixed margin, and its vertical position by another offset of the reference object, unless a flag on the reference object is set.
- `process_active_object_slots`: Walks a fixed table of 32 object slots and, for each occupied slot, dispatches on its type word to collision and movement helpers.
- `select_object_state_from_flags`: Sets an object's state from its status flags.
- `assign_frame_codes_from_flags`: Chooses one of three small frame codes from the object's status bits, writes a fixed marker word into its main field on two of those paths, and copies the chosen code into each of its four sub-entries that are not zero.
- `derive_offset_from_mode_byte`: Sets a working value to a default of 32.
- `resolve_vertical_contact_motion`: Applies vertical motion rules to an object in a given state.
- `latch_object_when_in_window`: Checks whether the working value lies within a small window around the object's stored position.
- `emit_child_sprite_parts`: Writes sprite entries for an object's part list, walking up to seven part pointers and copying each part's frame code and position into a shared sprite table.
- `emit_child_sprite_parts_ptr`: Writes sprite entries for an object's part list in the same way as its sibling emitter, including the visible-window check and the release path for parts outside it.
- `recycle_offscreen_sprite_part`: Counts down a timer field on the object, stepping it on odd horizontal positions.
- `recycle_stale_sprite_part`: Keeps a sprite part while its tracked object is still inside the visible window.
- `dispatch_code_and_check_patterns`: Maps a byte code through a range-based switch and a lookup table, updating a counter and a shared state word.
- `store_byte_in_object_field`: Calls a shared helper, then stores a byte passed in a data register into the object record at offset 24.
- `allocate_object_slot_with_parts`: Claims the first free slot in the 32-entry object table and initialises its position, owner and type fields, calling a failure helper if none is free.
- `stub_immediate_return`: The routine only returns, so it is a stub with no behaviour to recover.
- `clear_all_object_slots`: Walks all 32 object slots and, for each occupied one, copies its type word into the owner record it points to, clears the slot and decrements the live-object counter.
- `step_vertical_offset_countdown`: Moves an object vertically by a value read from a small table, indexed by a countdown byte that is decremented on each call and set to a sentinel at zero or a fixed limit.
- `dispatch_object_state_handler`: Reads an object's status word and skips some flagged states.
- `dispatch_object_state_entry`: Same routine body as its sibling dispatcher, reached by a direct call rather than through a stored address.
- `toggle_object_flag_bit`: Clears the top bit of a status byte in the object and then flips its lowest bit.
- `add_position_record_when_room`: Adds a position record to a list when the list has room, computing its placement from a parameter and a fixed base and adjusting it to fit a visible region.
- `add_position_record_if_aligned`: Like its sibling, it adds a position record to a list when there is room, but it writes the record only when a computed horizontal adjustment is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_zero_after_helper_check`: When a status word is zero it calls one helper, then always returns zero to its caller.
- `service_pending_interrupt_bits`: Walks four bit flags in a shared state word, recording each one set and calling a handler when the matching bit in the interrupt request register is also set.
- `update_timed_position_record`: Counts down a timer on a record and, depending on its mode, copies a computed or stored position into the caller's fields or writes a fixed marker.
- `update_audio_channel_state`: Updates one sound channel's output fields from stored values.
- `update_audio_channel_record`: Performs the same channel update as its sibling, reading the channel record through an address register, clearing a restart flag, reloading the data pointer and length, and picking a loop or stop state.
- `tick_channel_delay_counter`: Calls three helpers, then, while a channel flag is set, counts a delay byte down and calls a helper when the count reaches zero.
- `tick_channel_delay_record`: If a channel record's counter is nonzero, it reads a value from the record and, when the record's delay byte is zero or has counted down to zero, calls a helper for that value.
- `step_channel_envelope_phase`: Steps a channel's volume envelope through its phases, each reading a rate from an envelope table and moving the volume toward a target level.
- `update_sound_channels_tick`: Runs one tick of the sound driver across four channels, counting down timers, applying the envelope and a pitch offset, and handling any pending script command.
- `parse_channel_script_commands`: Reads script bytes for one sound channel, where byte 128 stores the next byte in a channel field.
- `parse_channel_script_copy`: Repeats the script command loop of its neighbouring routine.
- `restart_channel_script_parse`: Starts the script command loop at the channel's saved script start position instead of the current position.
- `parse_channel_script_scaled`: Stores the first script byte multiplied by a scale value in a channel's duration field, then runs the shared script command loop.
- `parse_channel_script_period_divide`: Stores a fixed constant of 750 divided by the first script byte in a sound driver variable, then runs the shared script command loop.
- `sum_script_bytes_scaled`: Adds up a run of script bytes whose count comes from the first byte, stores the total multiplied by a scale value in a channel field, then runs the shared script command loop.
- `load_instrument_then_parse_script`: Copies a 15-byte definition from a table, selected by the first script byte, into the channel's parameter fields and clears its counters.
- `parse_channel_script_unreached`: Repeats the shared script command loop from the position after the opcode.
- `load_envelope_then_parse_script`: Loads a 14-byte envelope entry, selected by the first script byte, into the channel's envelope fields, then runs the shared script command loop.
- `start_script_from_table_entry`: Starts the shared script command loop at a position read from a table, selected by the first script byte.
- `copy_word_pair_three_times`: Copies two word fields from one record to two other offsets within the same record, and repeats that pair of copies three times, moving the record pointer by one word each time.
- `copy_word_pair_twice`: Copies two word fields from one record to two other offsets within the same record, repeating the pair of copies for two consecutive words.
- `unrecovered_decompile_timeout`: The decompiler timed out, so no pseudo-code is available.
