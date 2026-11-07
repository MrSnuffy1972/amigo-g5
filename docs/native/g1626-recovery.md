# Game 1626 (james pond 2 - codename robocod (1991)(millennium)[cr skr][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01626). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (950976 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 97 raw-word lines |
| Functions named and described | pass, 219 of 219 named (0 author symbols), 219 of 219 described (4 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1770 of 1770 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7111 of 8695 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28848 |
| code | 31886 |
| data | 727938 |
| unused | 162304 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf:bootblock | 1 | 16 | 1 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf boot-block load | 1 | 1438 | 5 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5486 | 163 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1361 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 394 | 22 |

## Cases handled

- james pond 2 - codename robocod (1991)(millennium)[cr skr][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf: custom-format (non-DOS) disk, 1441 non-empty blocks: matched against track-loaded segments
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1348 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 152 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 270 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf: 2 logged loads cover 12288 bytes of the image
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf: 12 track-loaded blocks found in recovered runtime segments and classified through them
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf: blocks 4-10 (bytes 2048-5632) hold ASCII text
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf: blocks 22-1215 (bytes 11264-622592) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.65)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf: blocks 1219-1238 (bytes 624128-634368) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.54)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf: blocks 1242-1261 (bytes 635904-646144) outside the filesystem hold data, not code (opcode density 0.020, entropy 5.71)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf: blocks 1265-1284 (bytes 647680-657920) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.50)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][a]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf: blocks 1288-1431 (bytes 659456-733184) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.24)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf:bootblock

- `load_boot_block_data`: Boot-block entry that sets up an Exec I/O request with a load address, transfer length and command and submits it with DoIO, then sends a second 1024-byte request.

### logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][a].adf boot-block load

- `decompress_backward_lz_stream`: Unpacks a compressed block into memory from its end toward its start, reading control bits that choose between literal bytes and back-references.
- `test_register_bit`: Checks if the second bit of a register is set and returns if it is not.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_bits_with_reload`: Shifts a bit buffer by one position for each step of the count in the data register, reloading the buffer from memory when it empties and feeding the carry bit back in.
- `call_flagged_routine_then_zero`: Recovery of this 1360-instruction routine is incomplete.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `program_blitter_control_registers`: Adjusts a state value by stepping its shift bits, polls the blitter-busy bit until it clears, then writes the blitter control, modulo, pointer and size registers.
- `compute_blit_table_sequence`: Looks up an entry in a table using the argument, derives two pointers from state variables, then runs a sequence of steps that each wait for the blitter-busy flag before writing.
- `setup_blitter_pointer_sequence`: Looks up a table entry from the argument and computes two pointers, then programs the blitter's word masks, modulo, pointer and size registers, polling the blitter-busy bit before each start.
- `interpret_text_control_codes`: Walks a character string, handling space codes, carriage-return codes and special control bytes that adjust a position value or end the string.
- `advance_position_per_character`: Reads characters from a string and, for each ordinary character, looks up a width in a table and updates a position value with it.
- `apply_carriage_return_step`: Applies the carriage-return step to the position value, then reads the next character and continues the text loop.
- `load_byte_operand_then_continue`: Loads the next byte of the string into the position value and continues the text loop.
- `add_byte_operand_then_continue`: Adds the next byte of the string to the position value and continues the text loop.
- `add_byte_operand_alternate`: Adds the next byte of the string to the position value by a second control code path and continues the text loop.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `compute_blit_shift_values`: Scans two word values for their lowest set bits within six positions, then shifts a working value left by the matching amounts to produce blitter shift settings.
- `step_blitter_row_pointers`: Waits for the blitter-busy bit to clear, then walks a row count, advancing two pointers by a row stride on each pass and writing them to the blitter pointer registers.
- `set_blitter_default_registers`: Writes the blitter's first and last word masks, modulo and control registers with fixed values, including a control value that selects the copy function.
- `load_blitter_source_data`: Waits for the blitter-busy bit to clear, then writes the blitter's mask, control and modulo registers together with a source data value and a destination pointer, and waits again until the blitter is idle.
- `start_blitter_with_pointers`: Writes fixed blitter mask and control values, loads the A, B and destination pointers and the data registers from its arguments, and writes the transfer size register last.
- `blit_bitplane_rows`: Programs the blitter once per bitplane for a row of image data, offsetting the source and destination pointers by a fixed plane stride.
- `set_palette_and_blit_state`: Recovery of this 149-instruction routine is incomplete; the visible part writes fixed values to a few state variables.
- `forward_call_then_return`: Calls one other routine with no arguments and returns, performing no other work.
- `write_plane_address_words`: Calls a sequence of routines, then writes four 32-bit bitplane addresses into a list as high and low 16-bit words, advancing each address by a stride.
- `select_palette_and_write_addresses`: Calls several routines and sets a palette-related state value from fixed masks, then writes four 32-bit addresses as 16-bit word pairs.
- `update_position_record_fields`: Updates a position record by adjusting two coordinates with offset fields, deriving a four-bit fractional part and a sign-adjusted value, and computing an index from modulo and division by fixed sizes.
- `run_counted_routine_loops`: Runs several counted loops that each call a routine repeatedly, with a branch that calls a fixed routine when a state value is nonzero.
- `look_up_table_scaled_value`: Takes the high bits of a state value, scales them into an index, reads a 16-bit entry from a fixed table and doubles it into the state value.
- `compute_aligned_offset_value`: Combines a field from the structure with the masked state value scaled by 38, adds a fixed bias of 7904, and masks the result to multiples of 16.
- `write_row_address_pairs`: Stores an address in the structure, adjusted by a flag-dependent offset, then makes eighteen calls to a routine.
- `write_mirrored_row_address_pairs`: Stores an address reduced by a flag-dependent offset, makes eighteen calls to a routine, and writes four address pairs with a size value.
- `store_doubled_state_and_call`: Stores twice the state value into the structure, then makes fifteen calls to a routine.
- `store_offset_sum_and_call`: Stores twice the state value plus a fixed 36 into the structure, then makes fifteen calls to a routine.
- `set_blit_modulo_row_pointers`: Computes row offsets from the state value and an argument, waits for the blitter-busy bit when an offset is nonzero, and writes four pointer pairs stepping by a row stride.
- `initialise_state_constant_table`: Writes a fixed set of constant values and pointers into a block of state locations, then clears the remaining fields.
- `write_second_copper_list_pointers`: Writes four 32-bit addresses into a list as high and low 16-bit words, advancing each by a stride, then writes the final pair from the second value.
- `toggle_state_constant_set`: Compares a state value with a fixed constant and writes one of two fixed sets of constants to the state locations, returning after either branch.
- `dispatch_state_handler`: Calls a handler selected from a table by the state value times four.
- `dispatch_from_second_table`: Repeats the table-driven dispatch used by its neighbour, calling a handler chosen by the state value.
- `program_blit_row_copy`: When the state value is nonzero, computes row and column offsets, programs the blitter's control and mask registers, polls the blitter-busy bit, then sets the size and pointer fields for a row copy.
- `program_blit_shifted_row`: When the state value is nonzero, shifts a parameter into a blitter control value, polls the blitter-busy bit, and sets the size and pointer fields.
- `dispatch_after_two_calls`: Makes two calls to routines and then calls a handler chosen by the state value times four from a jump table that could not be recovered, so the handler is not identified.
- `dispatch_handler_after_setup`: Calls two setup routines and then the table-selected handler, as its neighbour does.
- `copy_tile_block_rows`: Copies 16 groups of 16 words from a packed source table into a destination area, writing each source word to two destination positions 16 words apart, then advances to the next block.
- `write_color_word_pairs`: Writes fixed pairs of colour words into a list block selected by the state value and returns the result of its last call, or zero when the state is zero.
- `clear_matching_active_records`: Walks a table of 26-byte records from a fixed base, calling a routine on each active record whose key matches the state value and then marking it inactive.
- `clear_eligible_active_records`: Unless the state value equals a fixed sentinel, walks the same 26-byte record table and clears each active record that meets its range and key conditions, calling a routine for the matching cases.
- `program_blit_for_record`: Computes horizontal and vertical positions and a bit shift from a sprite-like record, waits for the blitter-busy bit, then sets the blitter mask, shift, modulo and control registers and writes four bitplane pointers at fixed plane strides.
- `find_free_ten_byte_slot`: Scans up to thirty-two 10-byte entries from a list pointer and returns as soon as it finds an entry whose flag field is zero.
- `clear_active_ten_byte_slots`: Sets the blitter mask and control values, then walks up to thirty-two 10-byte entries, calling a routine on each active entry and clearing its flag field.
- `write_wrapped_plane_pointers`: Waits for the blitter-busy bit to clear, computes a pair of wrapped column values, and writes four bitplane pointers at a fixed plane stride.
- `call_four_times_then_clear`: Sets a state value to a fixed constant, calls one routine four times, and then clears three state values.
- `clear_record_flags_table`: Walks 32 records of 26 bytes from a fixed base, clearing the flag word at offset 14 in each, then zeroes the state value.
- `invert_global_byte`: If a global variable equals an underscore character, it is replaced with a null byte and then bitwise inverted.
- `copy_sixteen_longword_block`: Copies sixteen longwords from a fixed source table into a destination block in four groups of four.
- `execute_infinite_loop`: An infinite loop that prevents further execution.
- `poll_until_marker_value`: Repeatedly calls a routine and checks its result until it returns a specific marker value or a state flag is set.
- `reset_audio_channel_registers`: Points each of the four audio channel address registers at the same location and sets their lengths to one word.
- `read_raster_position_bits`: Reads the raster position value held in the state, swaps its two bytes and returns the low nine bits, which give the current raster position.
- `install_interrupt_handlers`: Disables interrupts through the interrupt enable register, sets exception vector entries to handler addresses, then enables selected interrupt sources.
- `bracket_cia_call_with_flag`: Sets bit 6 of a state value, calls one routine, clears that bit again and writes a fixed value of eight.
- `manipulate_interrupt_registers`: Performs bitwise operations on a global variable and interacts with interrupt control registers.
- `process_keyboard_serial_byte`: When the flag bit is set, rotates an inverted byte received from the keyboard serial register by one bit, calls routines when a full byte is complete, and then sets the state value to eight.
- `service_pending_interrupt_bits`: Calls one routine when bit 4 of the interrupt state is set, and when bit 5 is set calls another routine repeatedly for the count held in the state.
- `dispatch_pending_interrupt_bits`: Checks interrupt state bits in turn and, for each set bit, clears it and calls a routine, repeating until no bits remain.
- `mask_interrupt_state_bits`: Masks the state value with a fixed bit pattern and returns.
- `return_empty_routine`: Returns immediately with no work.
- `read_joystick_direction_flags`: Derives directional and fire-button masks from the joystick movement register and the CIA-A port register, then calls a routine when the derived mask is zero and calls another routine before returning.
- `match_key_code_commands`: Compares an incoming key code with a table entry and advances a table index when they match.
- `apply_key_command_toggles`: When a state value is nonzero, applies the key commands D, Y, X, U and V, inverting, clearing or setting state values for each letter.
- `run_countdown_masked_loop`: Runs a countdown of ten thousand steps, writing a masked count to a state value on each step, and returns with the final value.
- `run_countdown_wide_mask`: Runs a countdown of ten thousand steps, writing the count masked with a wider fixed bit pattern to a state value on each step, and returns with the final value.
- `run_countdown_split_mask`: Runs a countdown of ten thousand steps, writing the count masked with a split bit pattern to a state value on each step, and returns with the final value.
- `run_countdown_color_mask`: Runs a countdown of ten thousand steps, writing the count masked with a low-nibble pattern to a state value on each step.
- `step_shift_register_state`: Shifts a 32-bit state value left by one and adds one when exactly one of two tested bits is set, which is one step of a shift register with feedback.
- `advance_cycle_and_multiply`: Advances a counter that wraps from five back to one, then replaces the state value with its product with 177 reduced modulo 30269, a multiplicative congruential step.
- `format_decimal_digits_text`: Converts a value into decimal digits by repeated subtraction of powers of ten taken from a table.
- `halt_on_bad_data`: An invalid routine that contains bad instruction data.
- `set_display_control_registers`: Writes a value derived from its argument into the first display scroll register, sets fixed values into the playfield priority register, calls one routine, and then writes a final DMA control value.
- `write_six_address_word_pairs`: Stores an address and a fixed constant in the structure, computes a control value from the argument by a rotation and offset, then writes six 32-bit addresses as high and low word pairs, stepping each by a fixed amount.
- `copy_and_clear_palette_tables`: Copies 32 words from a table into a second area while zeroing the originals, then copies 256 words from two further tables into two areas, zeroing the sources, and sets a state value to 30.
- `clear_shared_word_block`: Zeroes 544 words starting at a fixed address and sets a state value to 30.
- `fill_word_tables_from_routine`: Fills 32 words and then 256 pairs of words in two areas, each with a value returned by a routine, and sets a state value to 30.
- `compute_nibble_sum_mask`: Adds the three nibbles of the argument, adds a state offset, caps the total at 47 and divides by three.
- `fill_table_entries_by_mode`: Fills either 256 table entries in two interleaved areas or 32 and 256 entries from routine results, depending on bit 0 of a state value, then decrements the state.
- `populate_table_entries`: Iterates through a data structure to write specific values into a table based on a provided mode.
- `process_four_table_bytes`: Walks four bytes of the state table starting at the index given in the argument, and for each nonzero byte calls a routine with its value.
- `scan_table_for_nonzero_entry`: Scans up to four records 148 bytes apart from a fixed table for a nonzero first longword.
- `advance_four_timers_on_wrap`: When the state is nonzero, increments four counters and, when the first passes 63, sets a flag bit, resets all four counters and clears the state.
- `decode_packed_bit_flags`: When the state is zero, reads a packed control byte through a pointer.
- `initialise_packed_byte_stream`: Writes 255 to the address held in an address register, moves the pointer on to the next byte, clears a counter and clears the state.
- `write_cia_a_word_registers`: Writes the low and high bytes of the argument to a pair of CIA-A registers, sets a state value to one and keeps the upper bits of the state.
- `write_cia_b_word_registers`: Like its neighbour, writes the low and high bytes of the argument to a pair of CIA-B registers, and sets a control value with a fixed bit.
- `write_stepped_values_loop`: Reads a count and a start value from a structure, then calls a routine with each value in turn, stepping the value by two and incrementing a state counter on each step.
- `draw_sprite_plane_blits`: Waits for the blitter-busy bit to clear, records a value into a per-object table, derives edge flags from two position fields and writes pointer words into the sprite area.
- `update_display_label_text`: When the state differs from the value stored in a record, copies a short block of characters into it and updates the record through routine calls.
- `interleave_plane_bytes`: Copies ten groups of four bytes from a source into a destination, writing each group's bytes to two positions 436 bytes apart so that the data ends up in interleaved positions.
- `wait_blitter_and_set_pointers`: Waits for the blitter-busy bit to clear four times, then writes two pointer values and the blitter size value.
- `wait_for_state_clear_sequence`: Recovery of this 158-instruction routine is incomplete; the visible part clears a state value and waits on it.
- `step_colour_state_machine`: Steps a small state value through a fixed sequence of branches, calling routines at each step and wrapping the value to zero at the end.
- `initialise_game_state_defaults`: Zeroes 3416 words from a fixed address, calls one routine, and writes a series of default constants into the state, including a fixed 32-bit value.
- `clear_status_fields_to_ones`: Sets four byte-sized state values to all ones and six longword fields to all ones at fixed addresses.
- `setup_blit_with_wait`: Calls three routines, then waits for the blitter-busy bit to clear and writes fixed blitter mask, control and modulo values and two address values before waiting again.
- `clear_state_fields_after_calls`: Calls two routines and then branches on a state value, clearing several state fields on the main path before returning.
- `clear_twenty_state_values`: Sets twenty state values to zero and returns.
- `set_triplet_state_if_active`: Clears four state values and then, for each of three conditions tied to the state value, writes a fixed set of three values when that condition holds.
- `run_state_machine_step`: Advances a multi-branch state machine: it checks an active flag, runs a sequence of counter and setup calls, and returns the value it captured at the start.
- `step_and_clamp_motion_value`: Adds a fixed step to a state value and clamps it between limits that depend on its sign and a flag, calling a routine in the large-value branch.
- `step_and_clamp_alternate_range`: Advances a state value by a two-step increment and clamps it at limits such as 150 and 50 in its branches, calling a routine on the large-value path.
- `update_offset_with_clamp_four`: Adds four to a state value, subtracts an offset chosen from a small set of constants, and clamps the result, returning early for certain states.
- `update_offset_with_clamp_eight`: Adds eight to a state value, subtracts an offset chosen from a small set of constants, and clamps the result, returning early for certain states.
- `reset_or_clamp_state_value`: Sets the state to zero or to fixed limit values depending on its sign and a flag, and nudges small values by one before returning.
- `add_offset_and_clamp_high`: Adds a fixed offset of 40960 to a state value, clamps its high word to a maximum of 15, and resets the state for certain values before returning.
- `select_state_from_mask_bits`: Tests bits of a state value and calls routines for matching masks, then sets a state value to 96 when a related state is zero.
- `clamp_delta_to_eight`: Computes the difference between two state values and applies it as a step when its magnitude is at most eight.
- `update_motion_with_range_checks`: Performs a range-checked update of a state value, clamping an adjusted value to plus or minus fifteen and calling routines for certain bit patterns, then sets flags and resets state fields.
- `clamp_scaled_value_to_three`: Adds a fixed offset of 5462 to a state value, clamps the high word to at most three and, when negative, to at least minus fifteen.
- `clamp_scaled_value_to_two`: Adds a fixed offset of 3280 to a state value, clamps the high word to at most two and, when negative, to at least minus fifteen.
- `call_sequence_and_reset_flag`: Calls six routines in sequence, copies a state value to itself, and clears the state when it is non-negative and a second flag is nonzero.
- `approach_target_with_clamped_step`: Adjusts a state value toward a target by a clamped step, with a halving rule for larger differences.
- `store_pair_after_call`: Calls one routine and then copies two values from the data registers into two state values.
- `restore_state_pair_after_call`: Calls one routine and then copies two values from the data registers into two state values, as its neighbour does.
- `subtract_step_and_set_one`: Subtracts a step from a state value, applies a correction when the result is negative and within a small magnitude range, then sets the state to one.
- `add_step_and_set_two`: Adds a step to a state value, applying a correction when the result is non-negative and above the step, then sets the state to two.
- `subtract_step_clamped_four`: Subtracts a step from a state value, adds the step back when the result is negative and not far below zero, and sets the state to four.
- `add_step_clamped_eight`: Adds a step to a state value, subtracts it again when the result is positive and not too large, and sets the state to eight.
- `apply_fixed_point_step`: Applies a fixed-point step to a state value, clamped at fifteen, with a direction bit deciding the sign, and calls two routines on the non-negative path.
- `decay_signed_value_toward_zero`: Moves a signed 16-bit value toward zero in steps of at most 15, applying the step a second time on one branch.
- `step_signed_value_toward_zero`: Moves a signed 16-bit value toward zero in steps of at most 15, using the same two-branch logic as its neighbours but without the threshold check.
- `add_clamped_delta_to_accumulator`: Clamps the input word to the range minus 15 to plus 15 and adds the result to a global accumulator.
- `reset_value_outside_threshold_band`: Zeroes the state value when it lies outside a fixed threshold band, otherwise it calls the step helper and repeats the test.
- `apply_scaled_correction_step`: Only when the state equals one, it takes a clamped difference, limits it to a small range, and adds or subtracts multiples of 8192 depending on sign.
- `check_nonzero_range_flag`: Tests whether a state value is nonzero and falls within a small range, then returns.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 1 into d6, then returns.
- `search_table_for_matching_entry`: Returns at once when the state is nonzero.
- `reset_state_and_increment_counter`: Clears the state, sets it to 255, calls a helper, and increments a byte counter indexed by the state value.
- `adjust_state_by_difference_threshold`: Takes the absolute difference between two state words.
- `compute_clamped_offset_from_difference`: Within a fixed difference range it stores a value clamped against the state.
- `advance_state_through_lookup_table`: When the state is nonzero, it replaces the state with a byte from a lookup table indexed by the state plus its own value, then decrements it.
- `compute_next_masked_state`: Initializes a state word and loops over a table of ten-word entries until a negative end marker, then derives a masked value using bit tests and stores it in a record at offset 16.
- `reset_state_variable`: Sets a global variable to various specific constant values based on its current state.
- `reset_state_variable_unreachable`: An unreachable routine that attempts to set a global variable to specific constant values.
- `reset_state_with_fixed_values`: Sets the state to a fixed sentinel when it is zero.
- `set_default_state_sentinels`: Sets the state to 255 when it is nonzero.
- `select_flag_value_then_clear`: Chooses one of two fixed values depending on whether the flag byte is zero, stores the chosen value, and clears the state.
- `store_constant_then_clear_state`: Stores a fixed constant into a state word and then clears the state.
- `advance_counter_or_set_four`: When the state is zero, it adds a fixed amount and increments the result unless it already equals four.
- `dispatch_state_to_handler`: Selects one of several branches based on a state code, calls a helper in each, and for some cases stores a clamped byte into a record at offset 39.
- `resolve_table_offset_chain`: Steps a state index back by one or two through a table of signed offsets until a non-negative entry is found.
- `lookup_indexed_table_cell`: Computes a table index from shifted coordinate values, reads a signed word from a fixed table, and follows bit-masked pointers through a secondary table to return a cell value.
- `follow_masked_table_pointers`: Indexes a fixed table from shifted inputs and follows a pointer when a high-bit mask is set, then stores the selected value into the state.
- `clamp_state_toward_target`: Clears the high bit of the state.
- `move_state_by_half_difference`: Returns unchanged when the input exceeds 21.
- `add_offset_to_state`: Adds a signed offset held in a register to the state word and returns.
- `subtract_half_step_from_state`: Subtracts a half-step of the input from the state when the input is within range, and otherwise subtracts the full input.
- `subtract_offset_from_state`: Subtracts a signed offset held in a register from the state word and returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d6, then returns.
- `nop_sequence_padding`: The routine consists of a sequence of no-operation instructions that perform no computational work or data manipulation.
- `stub_empty_return_routine`: Contains only a return and has no effect.
- `stub_empty_return_routine_2`: Contains only a return and has no effect.
- `compute_inverted_masked_bits`: Returns the bitwise complement of the input register masked to bits two and three.
- `reflect_state_when_flagged`: Calls a helper, then when a masked flag set in the input is present and a second helper returns nonzero, it sets the state to minus three minus its previous value.
- `reflect_input_into_state`: Copies the input into the state after a helper call, then when a masked flag is present and a second helper returns nonzero it sets the state to three minus its value.
- `evaluate_flag_mask_conditions`: Tests several bit masks against the state and the input and branches among seven helper calls.
- `match_control_word_set_state`: Returns zero when the input word does not match a fixed control pattern, with the pattern chosen by whether the state is zero.
- `count_qualifying_table_entries`: Walks a table of byte records, counting entries that are not the terminator and fall outside a small excluded byte range, and calls helpers for each entry.
- `scale_record_byte_by_64`: Returns the byte stored one place after the input address, multiplied by 64.
- `clear_and_halt`: Sets a global variable to zero before encountering bad instruction data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_position_fields`: Updates the position fields of a record by adding a scaled offset derived from a size field and a flag bit, then calls a helper and recomputes clamped coordinates.
- `update_record_position_with_clamp`: Updates the x and y position fields of a record from a size byte and offset values, clamps the result to a fixed range, and calls helpers for the adjustments.
- `set_background_color_register`: Computes a color value from the state with clamping and writes it to the background color register.
- `offset_record_position_by_size`: When a flag bit in a record is clear, subtracts half of a size field and a fixed inset from the record's x and y fields, then calls helpers.
- `allocate_free_table_slot`: Searches a 32-entry table for the first empty slot, stores three input words and a zero word there, and increments a count.
- `allocate_and_fill_slot_record`: Searches the same 32-entry table for a free slot, fills it with input values and increments the active count.
- `offset_record_from_source_size`: Adjusts the x and y fields of one record by half of a size value and a fixed inset taken from another record, unless a flag bit is set.
- `iterate_records_and_draw_entries`: Sets the background color register, then loops over a list of records and calls thirteen helpers to draw or update each one.
- `adjust_record_position_by_flags`: Sets a default of 60 when the state is zero, then when a flag bit in the record is set it subtracts a size-derived offset from a position field.
- `set_record_type_code`: Chooses a code from one of three values based on flag bits in a record, stores a type constant, and copies the code into the nonzero slots of the record.
- `set_default_size_from_record`: Sets a state to a default of 32, then when a record byte is nonzero it stores a value derived from that byte shifted left by four.
- `update_record_counter_on_flags`: When the input has bit one set and the record's flag bit is set, it updates a counter byte in the record according to the state.
- `check_state_within_record_range`: Compares the state with a field of a record and, when the difference falls within a small range, decrements the state and clears it before continuing the update.
- `read_background_color_entry`: Reads a color entry through a record pointer and returns it, touching the background color register.
- `update_background_color_entry`: Reads and writes a background color entry through record pointers, adjusts a color value using two helper calls, and updates the background color register.
- `copy_record_fields_to_state`: Copies two record fields into state words and sets the state to four, then adjusts the result when a flag is set.
- `match_record_within_table_bounds`: Follows a pointer field of a record and, when the current state falls within the bounds of the table entry plus a window of 304, marks the record and updates it.
- `map_byte_code_to_state`: Maps a byte from a record to a code.
- `store_byte_into_record_field`: Calls a helper, then stores the low byte of the data register into a record field at offset 24.
- `search_stride_table_free_entry`: Scans a table of records spaced twenty words apart until it finds an empty entry, then fills in its fields from the inputs and calls one helper.
- `stub_empty_return_routine_3`: Contains only a return and has no effect.
- `flush_pending_table_entries`: Walks a 32-entry table and, for each nonzero entry, copies its value into a linked record, clears the entry, and decrements the active count.
- `update_record_from_lookup_byte`: Reads a byte field from a record.
- `evaluate_status_word_flag`: Calls a helper, then tests a status bit in a record word together with the state, returning a value when the condition holds.
- `evaluate_status_word_and_update`: Calls three helpers, then tests a status bit of a record word against the state and updates the state accordingly.
- `clear_then_toggle_flag_bit`: Clears the top bit of a record flag byte and then toggles its lowest bit.
- `initialize_indexed_record_fields`: Computes a table address from an input shifted left by seven plus a constant, then initializes record fields with small starting values.
- `initialize_indexed_record_block`: Sets up a block of record fields from a table address and starting values of one and sixteen, clearing the remaining fields.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_helper_when_idle`: When the state is zero it calls a helper, and it always returns zero.
- `acknowledge_pending_interrupt_requests`: Loops over an interrupt request register and, when a pending bit is set, acknowledges it by writing the request register and calling one helper.
- `write_record_position_from_counter`: Decrements a counter field of a record, then writes position words derived from a table record using an offset and a half value.
- `compute_clamped_record_difference`: Calls three helpers, then stores a record word and a clamped difference of two state words into the record.
- `configure_dma_and_interrupt_enables`: Stores a clamped difference of two record fields into a record, then writes the DMA control and interrupt enable registers.
- `tick_counter_then_run_helper`: Calls three helpers, then when a stored flag word is nonzero it decrements a byte counter and calls one more helper once the counter reaches zero.
- `countdown_and_run_helper`: When a record word is nonzero it decrements a countdown byte and calls a helper when the countdown finishes, returning the helper's result.
- `load_record_table_entry_fields`: When a record field is zero it clears a companion field, and when another field is set it loads a byte and a value from a table entry into the record.
- `service_dma_channel_sequence`: Runs a sequence over the DMA and interrupt registers that sets up channels and clears pending requests, calling five helpers.
- `stub_bad_instruction_data`: The decompilation hit its restart limit and the body shows only a jump into an unresolved label, so this is a partial decode.
- `forward_to_helper_routine`: A two-instruction gap that is reached only by decoding and forwards to one helper call.
- `load_record_pointer_and_call`: Loads a pointer from a record field and calls one helper with it.
- `dispatch_record_pointer_to_helper`: Calls one helper with a record pointer taken from a register, after loading a pointer from a record field.
- `forward_to_helper_routine_2`: Five-instruction routine that calls one helper.
- `call_helper_in_unreached_gap`: Unreached gap of twelve instructions that calls one helper.
- `call_helper_after_pointer_setup`: Sets up pointer and state registers from its inputs and then calls one helper.
- `forward_to_helper_routine_3`: Two-instruction unreached gap that calls one helper and has no logic of its own.
- `process_pointer_and_call_helper`: Processes a pointer and a state value and then calls one helper.
- `forward_to_helper_routine_4`: Six-instruction unreached gap that calls one helper.
- `copy_word_pair_twice`: Copies two word fields from offsets in a structure to other offsets, repeating the copy twice while stepping the structure pointer by one word each time.
- `copy_word_pair_once`: Copies two word fields from offsets in a structure to other offsets, repeating the copy once while stepping the structure pointer by one word.
- `stub_pseudocode_unavailable`: The decompiler timed out, so no pseudo-code is available.
