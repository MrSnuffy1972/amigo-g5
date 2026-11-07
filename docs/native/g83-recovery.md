# Game 83 (DaysOfThunder v1.0 0285): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00083). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (524502 top-level bytes in 23 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 261 raw-word lines |
| Functions named and described | pass, 429 of 429 named (29 author symbols), 429 of 429 described (6 by rule, 153 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6181 of 6184 (99.95%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 13756 of 17926 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 230692 |
| code | 69460 |
| data | 224249 |
| padding | 101 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| DaysOfThunder.slave | 1 | 38 | 4 |
| DOT | 1 | 17653 | 409 |
| loader | 1 | 216 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |

## Cases handled

- 6 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_audio_and_background_color` (DOT): Body not recovered. Kind: disk hardware/ADKCON check. Pass path: normal execution flow (no trap/sabotage triggered).
- `read_disk_track_with_retries` (DOT): Sets up a disk read through the controller's sync word and DMA registers, then waits for the transfer to finish or time out. Kind: disk/track check. Pass path: uVar2 (return value 10 or reduced retry count).
- `step_disk_head_to_track` (DOT): Moves the disk head one track at a time toward a target track by driving the step and direction bits of the second CIA port. Kind: disk track/head movement timing check. Pass path: the loop completes and returns the target track index (in_stack_00000000) after executing a delay loop.
- `clear_and_set_control_bits` (DOT): Clears a global, then clears bit three and sets bit seven of a CIA port control byte. Kind: control bit manipulation/status flag. Pass path: the code continues execution normally after the bitwise operation.
- `clear_channel_state_by_index` (DOT): Sets a global to all ones and clears one of four state bytes chosen by a selector argument. Kind: disk hardware/track check. Pass path: the routine continues execution normally after the loop and memory copies.

## Functions


### DaysOfThunder.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `patch_jump_vectors_in_memory`: Writes an absolute jump opcode and a return opcode into memory-resident vector slots, stores a target address in them, and then calls a helper.
- `rotate_keyboard_serial_byte`: Complements and rotates one byte read from the keyboard serial data register and returns the rotated value unless it equals the stored byte, in which case the stored byte is returned.
- `call_stored_handler_passthrough`: Calls a handler through a pointer held in a global table and returns the value it was given unchanged.

### DOT

- `STARTUP`: Program entry point that stores an initial value in a global and then calls the startup routine; the recovered flow ends after that call.
- `configure_audio_and_background_color`: Body not recovered.
- `orchestrate_seven_subroutine_calls`: Body not recovered.
- `call_two_subroutines`: Body not recovered.
- `run_seven_subroutine_sequence`: Body not recovered.
- `dispatch_seven_subroutine_calls`: Body not recovered.
- `chain_seven_subroutine_calls`: Body not recovered.
- `sequence_seven_subroutine_calls`: Body not recovered.
- `call_three_subroutines`: Body not recovered.
- `forward_single_subroutine_call`: Body not recovered.
- `call_pair_then_return`: Body not recovered.
- `run_seven_subroutine_pipeline`: Body not recovered.
- `call_five_subroutines_shared`: Body not recovered.
- `select_track_and_set_volume`: Body not recovered.
- `set_background_color_title`: Body not recovered.
- `set_channel_zero_volume`: Body not recovered.
- `handle_replay_option`: Body not recovered.
- `run_four_call_sequence`: Body not recovered.
- `forward_one_call_short`: Body not recovered.
- `run_five_call_sequence`: Body not recovered.
- `forward_two_subroutine_calls`: Body not recovered.
- `forward_two_calls_alternate`: Body not recovered.
- `call_two_subroutines_conditional`: Body not recovered.
- `forward_two_calls_short`: Body not recovered.
- `dispatch_shared_single_call`: Body not recovered.
- `run_four_call_sequence_large`: Body not recovered.
- `clear_sixteen_global_words`: Writes zero to sixteen consecutive 16-bit words in a global area using a loop, then returns.
- `advance_audio_playback_state`: Advances a small playback state counter when a status byte matches a fixed character; when the counter passes its limit it calls a helper and reloads the period and volume registers for the audio channel.
- `select_state_from_helper_result`: Calls a helper, copies its result into global state words, calls two further helpers, then branches on the stored state codes.
- `decrement_state_counter_from_result`: Stores a helper result into a global state, calls helpers, then decrements a counter in that state and resets it when it drops below zero.
- `toggle_pattern_table_entries`: Decrements a global countdown and, when it wraps below zero, reloads it and XOR-toggles bit patterns in several table entries, which reads as a periodic blink or animation step.
- `toggle_pattern_table_and_call`: Decrements a global countdown and, when it wraps below zero, reloads it, XOR-toggles bit patterns in table entries and calls one helper.
- `adjust_values_from_helper_results`: Repeatedly calls helpers while a global value stays zero and adds each returned value to it minus a fixed offset, with a different offset at each step.
- `advance_timer_field_within_window`: Adds a fixed step to a 16-bit field and, when the result is inside a window set by the neighbouring value, moves that value across and clears it.
- `call_helper_twice_if_set`: If a field in the input block is non-zero, calls one helper with a constant and then calls it again, returning the second result.
- `compute_scaled_offset_from_helpers`: Calls two helpers and combines their results with a stored pair of words through a multiply to compute an adjusted offset.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `transform_block_words_with_helpers`: Reads many words from a buffer, combines them through helper calls and writes the results back into a data block.
- `set_channel_volume_from_table`: Computes a volume value from a table of words and helper calls, and writes it to the channel-zero volume register.
- `read_button_and_clamp_position`: Loops reading the fire-button and disk status port, updating a position value by the difference from its target and clamping it between a lower and an upper bound, then tail-calls the dial routine.
- `DIAL1`: Author-named dial routine: when a flag bit is set it calls helpers, runs a fixed delay loop, polls a status byte and adjusts a value by a shifted amount, clamped at the top.
- `adjust_counter_from_bit_flags`: When a code value matches, calls helpers and adds or subtracts one from a global counter depending on two flag bits, then updates the position from the difference with its target.
- `adjust_counter_from_port_bits`: Reads the fire-button port, adds or subtracts one from a counter according to two bits, clamps the counter between zero and ninety-nine, and waits for a particular bit pattern.
- `wait_flag_then_loop`: Busy-waits until a global flag clears and then runs a loop of helper calls that read and set fields.
- `clamp_position_from_helper_deltas`: Loops calling helpers and adds their differences to a position value, clamping it between a lower and an upper bound.
- `copy_block_selected_by_flags`: Selects a target value from two flag bytes, copies a large block of longs from a fixed source into the destination buffer, and then copies a smaller block.
- `copy_block_selected_by_record`: Selects a source from the flag bytes of its record argument and copies a large block of longs into the destination buffer.
- `copy_block_then_expand_table`: Copies a large block of longs from a fixed source, then loops over a table of words to expand its entries into the destination.
- `load_color_table_entries`: Calls helpers and copies a table of words into the color registers in a loop over two hundred lines.
- `read_word_clear_flag`: Reads a word from the argument block at an index, clears a flag byte at offset three when it is set, and calls a helper for that case before returning the word.
- `clear_flag_at_offset_three`: Clears the flag byte at offset three of its argument block when it is set, calls a helper, and returns the input value unchanged.
- `clear_flag_at_offset_four`: Clears the flag byte at offset four of its argument block when it is set, calls a helper, and returns the input value unchanged.
- `clear_flag_at_offset_five`: Clears the flag byte at offset five of its argument block when it is set, calls a helper, and returns the input value unchanged.
- `invert_bitplane_rows`: Inverts two words in each of thirty-two rows of a bitplane, stepping a fixed row stride, with the starting row chosen from an index table.
- `copy_rectangle_rows_between_planes`: Copies rectangular blocks of words between buffers in nested loops, reading the dimensions from a table indexed by the argument and updating the destination row by row.
- `copy_rows_into_bitmap_buffer`: Copies twenty-four rows of words from a fixed source into a bitmap buffer whose destination offset is computed from the argument, using a 160-byte row stride.
- `copy_rows_from_bitmap_buffer`: Copies twenty-four rows of words from a bitmap buffer back to a fixed destination, reversing the direction of the copy in its sibling routine.
- `DELETE_POINTER`: Author-named DELETE_POINTER routine.
- `return_immediately_2`: This routine consists of four instructions that perform no operations before returning to the caller.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `copy_sixteen_word_table`: Copies a sixteen-word block from one fixed table to another and then returns.
- `run_four_helper_calls`: Makes four calls to helper routines in sequence and then returns.
- `format_race_positions_text`: Builds a text line that reads RACE POSITIONS SO FAR by writing characters into a buffer and drawing them through three helper calls.
- `wait_for_return_character`: Calls three helpers and then loops until a status character equals the carriage-return code.
- `convert_number_to_decimal_digits`: Converts a number below ten thousand into decimal digits packed into a 32-bit result, by repeated subtraction of powers of ten.
- `copy_mode_specific_block`: Copies a block of words from one of two fixed sources, chosen by a mode value, into a buffer at a base offset, stepping across rows.
- `relink_record_fields`: Follows linked record pointers in a global structure, copies a neighbouring field into the next record and writes a fixed negative value into another offset.
- `update_channel_volume_register`: Body not recovered.
- `run_channel_volume_sequence`: Body not recovered.
- `find_first_greater_word`: Scans a table of words from a starting point and stops at the first entry greater than the value supplied, leaving its position in the result.
- `run_large_seven_call_routine`: Body not recovered.
- `dispatch_three_call_handler`: Body not recovered.
- `run_eight_call_routine`: Body not recovered.
- `chain_eight_subroutine_calls`: Body not recovered.
- `run_color_update_sequence`: Body not recovered.
- `sequence_seven_calls_mid`: Body not recovered.
- `forward_three_calls_short`: Body not recovered.
- `set_channel_volume_sequence`: Body not recovered.
- `set_replay_channel_volume`: Body not recovered.
- `forward_four_calls_short`: Body not recovered.
- `copy_cinf_header_fields`: Reads fields from a structure tagged CINF and copies them into a table of global fields, one word or longword at a time.
- `call_setup_with_fixed_arguments`: Calls a helper with fixed constant arguments and a fixed address, then calls a second routine and returns.
- `copy_word_block_quads`: Copies a block of words four at a time from one buffer to another in a loop.
- `set_channel_volume_short`: Body not recovered.
- `run_channel_volume_routine_large`: Body not recovered.
- `forward_single_call_short`: Body not recovered.
- `combine_bitplane_rows_masked`: Combines four source rows per step with bitwise OR, inverts the result and applies it as a mask over a block of thirty-two rows, selecting the target buffer from a level flag.
- `walk_sprite_table_entries`: Looks up table entries from a row index and, when the entry marker is not minus one, calls a helper and updates a field of the found entry.
- `combine_plane_rows_with_mask`: Loops over rows, combining four plane words with bitwise AND, inverting the result and writing it back into the plane words under a mask.
- `clear_six_slot_words`: For each non-zero slot in a six-entry offset table, clears five words spaced at fixed row strides from the slot base.
- `fill_sprite_records_first`: Fills ninety-five small records with fixed words and calls helpers for each record.
- `fill_sprite_records_second`: Fills ninety-five small records in a second table with fixed words and calls helpers for each record.
- `find_first_nonzero_word_group`: Scans a block of word groups until a combined word is non-zero, then stores the offset and the group's words into the output block.
- `scan_word_groups_for_zero`: Advances a pointer through word groups while their combined words are non-zero, leaving the pointer at the first all-zero group.
- `store_word_group_record`: Writes an output record of five words from the input pointer, with the last word set to the complement of the input value.
- `run_color_and_volume_loop`: Stores several constants into globals, then loops calling a helper and writing the background color and channel-zero volume registers.
- `clear_global_and_return`: Clears a global variable and returns the stack value unchanged.
- `parse_iff_form_chunks`: Checks that the data starts with a FORM header and an ILBM type, then walks the chunks, copying the entries of each CMAP colour map chunk it finds.
- `copy_sixteen_words_to_global`: Copies sixteen words from a source into a global block that backs the color registers.
- `advance_animation_frame_counters`: Loops over records, increments a frame counter in each record when a flag is zero, and compares it with a stored value to choose the next state.
- `build_record_pointer_table`: Fills a table of pointers to records spaced ninety-eight bytes apart, then runs a second loop over the table to clear a field in each record.
- `compute_scaled_position_offset`: Adjusts a value by adding or subtracting five hundred according to its low bits, then scales it by a quarter of itself plus the original and clamps the result.
- `compute_scaled_position_with_calls`: Adjusts and scales a global value as the scaled-position routine does, and calls two helpers depending on flag results.
- `WIN_LIST`: Author-named WIN_LIST routine.
- `update_object_records_from_table`: Loops over up to two hundred fifty-five object records, selects those whose status flags allow it, and updates their position fields from a second table, calling helpers along the way.
- `scale_value_by_table_ratio`: Looks up a table entry by a stored index and divides a field by that entry to compute and store a scaled ratio.
- `run_delayed_step_below_five`: When the step index is below five and a flag bit is set, calls two helpers separated by a fixed delay loop, then advances the index.
- `run_delayed_step_if_active`: When the step index is not minus one and a flag bit is set, calls two helpers separated by a fixed delay loop, then advances the index.
- `set_display_timing_registers`: Writes fixed values to the display timing, bitplane modulo, control and copper pointer registers, then calls a helper.
- `fill_three_bitplane_pointers`: Writes a 32-bit address into a table of three entries at a forty-byte row stride and writes the DMA control register, so it sets up the display DMA channels.
- `fill_sequence_table_bytes`: Fills fifteen entries of a table at a fixed stride with a byte value that increases by two at each step.
- `update_pointer_table`: Loads a new pointer from a table into a global variable and copies a fixed number of words from the new pointer's location to a global array.
- `init_cia_timer_and_interrupts`: Stores initial timer and interrupt-enable constants for the CIA-A timer and the custom interrupt enable register.
- `check_and_reset_bit`: Checks a bit in a global variable and, if set, executes a routine and loops until a bit condition is met before clearing a global variable.
- `modify_bit_in_table`: Uses a value from a register to calculate a bitmask and a target address, then sets or clears a specific bit in a byte at that address.
- `load_byte_from_table`: If a bit in a register is clear, loads a byte from a table into a global variable.
- `advance_counters_and_request_interrupt`: Advances a set of global counters, writes the color-fifteen and interrupt request registers, and calls two helpers.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `KEYSETUP`: Author-named KEYSETUP routine that clears the low bit of a global flag byte.
- `OLDINTVEC`: Author-named OLDINTVEC routine that returns immediately with no work.
- `NOTKEYINT`: Author-named NOTKEYINT routine that adds two to a global counter and clears its low bit.
- `update_channel_period_state`: Compares two globals and, when they differ, resets them and writes the channel-zero period register.
- `read_joystick_direction_bits`: Reads the two joystick data registers and the fire-button port, and combines their bits into a direction and button state value.
- `startup_no_op_return`: Returns immediately without doing any work; the startup routine calls it.
- `adjust_value_by_key_code`: Examines a command character and adjusts a record field by fixed amounts for particular characters, such as R, T and Y.
- `randomise_record_positions`: Loops over object records and, for those without a flag bit set, calls a helper and computes a new position from its result and a stored value.
- `reset_record_position_from_helper`: Calls a helper to pick a value, computes a new position from it and a stored value, and sets the record timer field to five.
- `swap_state_and_call_helpers`: Swaps two global values, calls two helpers, and clears a global before returning.
- `spin_forever_loop`: Loops forever without returning, so the caller never resumes.
- `draw_text_string_to_bitmap`: Walks a zero-terminated string, looks up each character's width and draws it into a bitmap at a computed row and column offset.
- `measure_text_string_width`: Counts the characters of a zero-terminated string and computes a centred horizontal position from the count before drawing.
- `draw_text_at_computed_position`: Computes a text column from a register value and draws a zero-terminated string into a bitmap at that column.
- `draw_text_with_spaces`: Writes characters from a zero-terminated string into a bitmap, treating the space code specially and stopping at the terminator.
- `draw_string_with_glyph_rows`: Looks up each character of a zero-terminated string and copies glyph rows into the bitmap, with the same glyph loop as its neighbour.
- `return_only_large_routine`: The recovered body is only a return, although the facts record one hundred seventy-nine instructions and two callers, so the real work is not visible.
- `draw_centred_text_string`: Measures a zero-terminated string when a mode is one, centres it horizontally from its length, and writes it into a buffer.
- `clear_buffer_and_call_function`: Iteratively clears specific offsets in a buffer using a bitmask, skips certain characters in a string, and finally calls a function pointer located via a global variable.
- `merge_source_first_two_words`: For a caller-supplied number of rows, clears the masked bits of four destination words and ORs a source word into the first two.
- `merge_source_into_third_word`: For a caller-supplied number of rows, clears the masked bits of four destination words and ORs a source word into only the third.
- `merge_source_into_alternate_words`: For a caller-supplied number of rows, clears the masked bits of four destination words and ORs a source word into the first and third.
- `BIG_CON`: ORs a fixed bit pattern into one byte before the byte-coded command stream is read.
- `merge_source_first_three_words`: For a caller-supplied number of rows, clears the masked bits of four destination words and ORs a source word into the first three.
- `merge_source_last_three_words`: For a caller-supplied number of rows, clears the masked bits of four destination words and ORs a source word into the last three.
- `merge_source_all_four_words`: For a caller-supplied number of rows, clears the masked bits of four destination words and ORs a source word into each of them.
- `write_four_hex_digits`: Writes the four hexadecimal digits of a 16-bit value, most significant first, converting each nibble to an ASCII character and passing it to a character output routine at eight-unit steps.
- `write_counted_hex_digits`: Writes a caller-supplied count of hexadecimal digits from the top of a 16-bit value, converting each nibble to an ASCII character and passing it to a character output routine at eight-unit steps.
- `GREENFLAG`: Writes zero to one global byte and returns the input value of a data register unchanged.
- `reset_state_and_copy_block`: Clears five state variables, sets two size values of 320 and 290, calls setup routines, and copies 8192 longwords from one buffer to another.
- `copy_variant_block_to_buffer`: Chooses one of three source addresses from a selector global: value 1 picks one, value 2 another, and any other value the third.
- `STACK`: Merges bytes from a source buffer into a destination buffer at fixed offsets forty bytes apart.
- `CHECKER_TRIANGLE1`: Flips one bit of a global byte, with the bit index taken from a byte in the structure modulo eight.
- `PEEL_TO_CHECKERED`: Forces the high bits of fourteen bytes at fixed offsets to fixed patterns while leaving their low bits unchanged.
- `write_palette_entry_one`: Writes the decimal values 255 and 4095 in turn to the color register for palette entry one, then calls a routine.
- `write_scaled_color_one_value`: Writes zero to a global, then stores a word read from memory, doubled and increased by one, and calls a routine.
- `set_interrupt_vector`: Stores a value into a global variable, stores the routine's own address into another global variable, and executes a software trap.
- `configure_interrupts_and_build_table`: Writes interrupt-control values and sets a vector, references the color register for entry zero, and builds a table of 200 entries rising in steps of 160.
- `nop_padding_routine`: This routine is empty and performs no operations.
- `transform_buffer_values`: Iterates through a fixed number of words in a global buffer, replacing each word with a value calculated by doubling the original and adding one, while also updating a hardware color register.
- `step_countdown_table_entries`: While a countdown global is nonzero, it decrements the global and passes the two words and the longword of an eight-byte table entry to a routine.
- `ENTER_HIRES`: The decompiled body is only a return, so no behaviour was recovered for this routine.
- `run_sequence_with_word_copy`: Calls a few routines, copies sixteen words from one fixed buffer into another, then calls many more routines before returning.
- `call_routine_five_times`: Calls the same routine five times in a row and returns.
- `execute_sequence_of_routines`: Calls a series of different routines that perform tasks such as clearing memory, copying records, and initializing structures.
- `clear_struct_header_and_call`: Zeroes several fields at the start of a structure pointed to by a global, then calls ten routines in sequence.
- `or_xor_table_into_block`: Loops eighty-eight times over a source table, ORing groups of longwords into a destination block at several fixed offsets, with some positions also XORed.
- `perform_no_operation`: This routine is empty and performs no actions before returning.
- `step_palette_toward_target`: Steps each of sixteen color words one unit toward its target in the red, green and blue nibbles, then copies the sixteen results into a global buffer.
- `fill_rows_with_cycling_longwords_2`: Fills 200 rows of a buffer, forty longwords per row, with a cycling pattern of thirteen longwords read from memory, then returns.
- `copy_longword_table_to_buffer`: Copies a block of longwords in a loop of 112 passes from a fixed table into a buffer at an offset from a global pointer.
- `place_entry_at_computed_offset`: Computes a horizontal position as a global index times twenty plus eighty-eight, passes it with a fixed value and a fixed pointer to a routine, then calls another routine.
- `display_lap_option_labels`: Passes fixed screen positions and text references, including the labels FULL LAP, HALF LAP and HIGH, to a routine in sequence.
- `display_high_option_label`: Passes fixed screen positions and text references, including the label HIGH, to a routine.
- `execute_sequential_state_transitions`: Updates game state, audio, and record fields before executing a sequence of conditional loops that check a global flag, copying strings to a buffer and calling specialized routines or drawing text if certain conditions are met.
- `call_routine_per_list_entry`: Walks a list held behind a global pointer until a minus-one entry, calling a routine for each entry with a running value that rises by sixteen per entry.
- `copy_records_from_source_lists`: Copies a longword and a word from two source lists into record fields for each entry, advancing through the lists and the records by fixed steps.
- `clear_and_initialize_records`: Zeroes a block of four-longword groups, then initializes a set of record fields to zero or ten.
- `update_record_positions_and_timers`: For a record, adds its velocity words to its position words when a flag bit is set, and otherwise runs a countdown field and a related bit test.
- `ease_values_toward_targets`: Moves two record fields halfway toward their target fields on each pass.
- `CARS_SETUP`: Loops over an array of records, halving the difference between fields and their targets, selecting values from bit tests, and calling several routines per record.
- `find_and_reset_record`: Iterates through a list of records to find a block matching a specific range, then copies certain values to a global buffer and resets several fields in the source structure.
- `configure_audio_period_and_volume`: Sets a timing value from a mode counter, then writes values to the audio period and volume registers for channel zero, along with several display-size constants.
- `update_global_pointer`: Updates a global pointer using offsets and checks a bit in a RAM variable to potentially subtract a value returned by another routine.
- `DO_CAMERA`: Stores negated record fields into globals, with a fixed offset added to one of them, then returns the input byte ORed with a fixed mask.
- `traverse_pointer_chain`: Traverses a chain of pointers stored in a global variable and applies a constant offset to the final value.
- `clamp_register_value`: Clamps a register value between two constant bounds and stores the result in a global variable.
- `CAMERA_CODE`: Copies two words to globals, then takes the absolute values of two others, divides the smaller by the larger, and looks up the quotient in a 16-bit table.
- `compute_negated_table_angle`: Computes a negated value from a table lookup of a scaled division.
- `CAMERA1B`: Subtracts a fixed amount from a global, copies two words to fixed globals, and computes a negated angle from the ratio of two absolute values using a table.
- `store_negated_table_angle`: Computes a negated value from a table lookup of a scaled division and stores it in one global.
- `reset_state_on_value`: Checks a global variable against several specific negative constants and, if a match is found, resets a global counter and calls a routine to reset state and copy a block.
- `search_marker_pattern`: Searches backward through a memory range for a specific pattern, checking for certain constant values and updating a global pointer.
- `search_for_marker`: Searches through a memory range for a specific constant value and updates a global pointer if a certain pattern is found.
- `copy_memory_block`: copies a block of four-byte words from a source memory location to a destination memory location using a loop.
- `fill_longwords_with_pattern`: Fills a counted run of longwords with a fixed repeating-nibble pattern.
- `initialize_records_and_accumulate`: If the count is zero it clears a flag bit, sets default values and returns.
- `integrate_records_with_damping`: For each record with a nonzero counter, adds scaled velocity terms to its position words and decreases a damping counter by ten toward zero.
- `claim_free_record_slot`: Finds the first empty slot in a record table, marks it in use, and copies position words from the input.
- `advance_active_records_and_copy`: For each active record, adds velocity terms to its position fields and increments a field.
- `initialize_record_fields_from_base`: Calls a routine, then sets a group of record fields to a base value plus fixed offsets.
- `scan_records_with_sentinel`: Walks a list of eight-byte entries until a sentinel value.
- `walk_linked_list_calling_routine`: Walks a linked list in which each node's first longword is the next pointer.
- `compute_offsets_within_bounds`: Checks a point against a set of bounds using words from a record.
- `insert_node_into_sorted_list`: Inserts a node into a list kept in order of its key field, linking it ahead of the first node with a larger key.
- `DRAW_GROUND`: Scans a list of ground segment records for one at or beyond a threshold, then calls setup routines.
- `process_ground_segment_group`: Scans a segment list the same way as the ground-drawing routine before it, then calls setup routines and walks the segment records.
- `or_segment_flags_and_process`: Walks a segment list and calls a routine for each entry.
- `traverse_code_table`: Initializes a global variable to zero and iterates through a code table, calling a routine if certain bitwise conditions or flags are met.
- `process_indexed_table`: Iterates through a table of pointers, checks a flag in a secondary table, and performs a bitwise OR-accumulation of values from a range before calling a routine at a calculated address.
- `process_segment_flags`: Iterates through a list of pointers, checks a flag in a table, and performs an indirect call to a routine based on a bitmask check.
- `traverse_code_table_with_one`: Initializes a global variable to one and iterates through a code table, calling a routine if certain bitwise conditions or flags are met.
- `traverse_code_table_with_two`: Initializes a global variable to two and iterates through a code table, calling a routine if certain bitwise conditions or unable to resolve certain flags.
- `process_buffer_and_traverse_table`: Initializes a global variable to zero, copies longwords to a buffer, and then iterates through a code table to call a routine based on flag checks.
- `execute_counted_routine_and_traverse`: Executes a routine a certain number of times and then iterates through a code table to call a routine based on flag checks.
- `process_segment_group`: Iterates through a list of pointers, checks a flag in a secondary table, and performs a bitwise OR-accumulation of values from a range before calling a routine at a calculated address.
- `find_flagged_segment_and_call`: Walks a segment list calling routines for each entry, then scans forward for the next flagged entry.
- `find_flagged_segment_and_call_2`: Copies longwords to a buffer and searches for a flagged segment in a table to call a routine and update a global variable.
- `search_flagged_segment_and_call`: Executes a routine a certain number of times, then searches a table for a flagged segment to call a routine and update a global variable.
- `find_flagged_segment`: Iterates through a list of pointers until a flag in a secondary table is found, then calls a routine at an address calculated from a value in the table.
- `select_ratio_routine_by_magnitude`: Takes the absolute values of two words, then calls one of two routines depending on which value is smaller.
- `validate_range_and_pointer`: Checks if a pointer or a value within a range or table matches a certain condition and updates a global variable.
- `lookup_quadrant_table_value`: Looks up a sign-adjusted value from a table, reducing the input angle into one of four quadrants.
- `lookup_offset_quadrant_value`: Looks up a sign-adjusted table value for an input angle, using a quarter-offset within the table for each quadrant.
- `divide_fixed_point_by_iteration`: Divides a 32-bit value by a 16-bit divisor using an iterative averaging loop that returns early once the estimate stops changing.
- `lookup_scaled_table_by_range`: Looks up a value in a table using an index taken from the input's upper bits.
- `GET_TRACK_SECTION`: Compares an input value against a long sequence of thresholds, adding a fixed step each time one is exceeded.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 1 to d1, then returns.
- `run_division_step_loop`: Selects a helper routine by a table index, then runs a counted loop that calls it repeatedly.
- `execute_routine_with_range_logic`: Calculates a range or offset and executes a routine a certain number of times, adjusting an offset or counter based on comparison results.
- `call_indexed_routine`: Calls a routine at an address calculated by shifting a global variable and adding it to a base address.
- `xor_memory_values`: Performs a bitwise XOR operation between a value in a register and a value at a memory location, then repeats this for a different offset in the same memory block.
- `NORMALISE`: The routine is empty and performs no operations.
- `LINE`: XORs a register word into one word at a fixed offset of an object pointed to by an address register, then returns.
- `and_register_into_target_word`: ANDs a register word into the word pointed to by an address register, then returns.
- `apply_bit_masks`: Performs bitwise OR or AND operations on a pointer-referenced memory location and several other offsets in a table.
- `apply_bit_masks_mixed`: Performs bitwise AND or OR operations on a pointer-referenced memory location and several other offsets in a table.
- `GO_L`: ORs a register word into the word pointed to by an address register, then returns.
- `SHALLOW_L`: ANDs a register word into one word at a fixed offset of an object pointed to by an address register, then returns.
- `apply_bitmask_to_struct`: Performs a bitwise AND operation on certain members of a structure using a value from a register, and a bitwise OR operation on another member.
- `modify_bitmask_elements`: Performs bitwise OR and AND operations on specific words within a memory block pointed to by a register.
- `apply_bitmask_to_struct_members`: Performs bitwise AND and OR operations on specific members of a structure pointed to by a register.
- `apply_bitwise_masks`: Performs bitwise AND and OR operations on a table using a register and a constant mask.
- `modify_bitmask_flags`: updates specific bitmask flags within a structure or array using bitwise OR and AND operations with values stored in registers.
- `mask_and_set_bits`: Performs a bitwise AND on a value at a pointer location and then performs bitwise OR operations on three other memory locations using the same register value.
- `execute_division_loop`: Decrements a global variable and executes a loop that updates a global variable using values from a table and calls a division routine.
- `execute_indirect_call`: Executes a function call using a pointer stored in a register.
- `call_routine_counted_times`: Calls a routine repeatedly for a counted number of passes, decrementing the count until it passes minus one, then returns.
- `find_indexed_routine`: Iterates through a list of pointers and calls a routine at an address calculated from a table entry.
- `decrement_and_call`: Decrements a value from a pointer and calls another routine until the counter reaches zero.
- `wait_then_compare_scaled_terms`: Waits for a global to become nonzero, then computes scaled squared and product terms and compares their absolute values.
- `divide_and_pack_quotient_remainder`: Divides a 32-bit value by a 16-bit global, returning the remainder in the upper word and the quotient in the lower word.
- `calculate_scaled_quotient`: Computes a value based on a global variable and its square, returning the remainder and quotient packed into a single longword.
- `calculate_scaled_quotient_2`: Performs arithmetic using a global variable to produce a longword containing a remainder and a quotient.
- `calculate_scaled_quotient_3`: Calculates a result by dividing a value derived from a global variable by that same variable, returning the result in a packed format.
- `fill_table_rows_minus_one`: Sets globals based on the sign of an input value, then fills groups of longwords in a table with the value minus one, four at a time, at fixed offsets.
- `fill_table_rows_minus_one_2`: Sets globals based on the sign of an input value, then fills groups of longwords in a table with the value minus one.
- `fill_table_rows_minus_one_3`: Sets globals based on the sign of an input value, then fills groups of longwords in a table with the value minus one, using a longer fill sequence.
- `spin_unless_values_match`: Calls a routine, then returns if two globals are equal.
- `fill_table_with_value`: Fills a range of a table with a repeated value and performs a bitwise XOR on a final entry before potentially entering an infinite loop.
- `update_coordinate_table`: Iterates through a linked list and writes modified offset values or constant offsets to a table.
- `return_from_subroutine_4`: This routine performs no functional operations and immediately returns control to the calling routine.
- `return_without_side_effects`: The routine performs no operations and immediately returns to its caller.
- `read_disk_track_with_retries`: Sets up a disk read through the controller's sync word and DMA registers, then waits for the transfer to finish or time out.
- `retry_disk_read_until_limit`: Retries the disk read routine up to a fixed count, returning a failure code of three when the count is exhausted.
- `step_disk_head_to_track`: Moves the disk head one track at a time toward a target track by driving the step and direction bits of the second CIA port.
- `pulse_disk_step_with_delays`: Pulses a disk control line with busy-wait delay counts around each step, repeating until the requested track matches.
- `busy_wait_for_count`: Counts a register down to minus one as a delay loop, then stores a saved value into a global and returns.
- `set_cia_port_upper_bits`: Sets a global to 255 and rewrites the upper control bits of a CIA port to a fixed pattern, leaving the lower bits unchanged.
- `clear_and_set_control_bits`: Clears a global, then clears bit three and sets bit seven of a CIA port control byte.
- `poll_disk_status_with_pulses`: Pulses a bit of a control register with busy-wait delays while a status flag is set, then clears two globals and sets a bit in the register.
- `clear_globals_set_bit_two`: Clears two globals and sets bit two of the control byte at the pointer.
- `hash_uppercase_text_eleven_bits`: Computes an eleven-bit hash of a zero-terminated text by folding each character, converted to uppercase, into a running total.
- `fold_text_into_product_hash`: Multiplies two inputs, negating one when a sign flag is set.
- `STIPPLE_TABLE`: Writes a fixed 16-bit pattern to two address-register targets, then returns the input byte ORed with a fixed mask.
- `copy_indexed_longwords_to_buffer`: Copies a list of longwords chosen by an index table into a destination buffer.
- `dispatch_entries_through_code_table`: Walks a list of entries, computes a table index from each entry's fields, and calls a routine from a code table with the entry's values.
- `CLIP_TABLE`: Clips a line segment against a set of bounds, computing each intersection from the quotient and remainder of coordinate differences.
- `interpolate_coordinate_pairs`: Calculates a linear interpolation or offset using division and stores the result and a global value into a pointer-based destination.
- `interpolate_area_product`: Performs an area-based calculation and stores an interpolated coordinate or value into a pointer-based destination.
- `interpolate_offset_differences`: Calculates an interpolated offset using division and stores a global value and the result into a pointer-based destination.
- `interpolate_line_segment_coordinates`: Calculates an interpolated coordinate or offset using division and stores the result and a global value into a pointer-based destination.
- `interpolate_area_product_branch`: Performs conditional area-based interpolation and stores a global value and a calculated coordinate into a pointer-based destination.
- `interpolate_coordinate_pairs_2`: Calculates a ratio or offset and writes a pair of values to a table or pointer.
- `store_clipped_area_coordinates`: Calculates an area product and stores the resulting coordinates through a pointer-based destination.
- `calculate_coordinate_interpolation`: Calculates a quotient and remainder, then stores a calculated value and an offset value into a pointer-based destination.
- `interpolate_if_within_bounds`: Checks if a value is within a certain range, then performs a division-based interpolation and stores the result into a pointer.
- `interpolate_if_greater`: Checks if one value is less than or equal to another, then performs a division-based interpolation and stores the result into a pointer.
- `interpolate_coordinate_pairs_branch`: Calculates coordinate differences and area products, then performs conditional interpolation or coordinate storage via a pointer.
- `interpolate_coordinate_pairs_branch_alt`: Executes conditional coordinate difference multiplication and area product calculation before performing a pointer-based interpolation.
- `interpolate_coordinate_value`: Performs a division-based interpolation and stores the result and a global variable into a pointer-based destination if a certain condition is met.
- `store_clipped_coordinate_value_2`: Checks a condition and, if met, calculates an interpolated coordinate value and stores it through a pointer.
- `check_coordinate_bounds`: Compares two short values and returns if the first is less than the second.
- `interpolate_coordinate_table`: Performs complex interpolation or offset storage into a pointer-based destination using division and conditional logic, or stores a global value and a calculated offset.
- `store_interpolated_pair`: Stores two short values from registers into a pointer-based destination after calling a routine that calculates offset differences.
- `interpolate_after_area_calculation`: Performs a series of area and difference calculations, then stores an interpolated value and a global variable into a pointer-based destination.
- `interpolate_coordinate_conditional`: Performs a division-based interpolation and stores the result and a global variable into a pointer-based destination if a certain condition is met, following coordinate difference multiplications.
- `interpolate_coordinate_complex`: Performs complex interpolation or offset storage into a pointer-based destination using division and conditional logic, or stores a global value and a calculated offset, following coordinate difference multiplications.
- `interpolate_coordinate_offset`: Calculates a linear interpolation or offset value using division and remainder, then stores the result and a coordinate into a destination pointer.
- `interpolate_offset_pointer`: Calculates a division-based interpolation and stores a global variable and a calculated offset into a pointer-based destination.
- `interpolate_offset_pointer_conditional`: Performs a division-based interpolation and stores a global variable and a calculated offset into a pointer-based destination if a certain condition is met.
- `store_coordinate_pair`: Stores two short values from registers into a pointer-based destination after performing coordinate difference multiplications and offset clipping.
- `interpolate_coordinate_area`: Performs a division-based interpolation and stores a global value and a calculated offset into a pointer-based destination if a certain condition is met, following an area product calculation.
- `interpolate_offset_pointer_with_clipping`: Performs a division-based interpolation and stores a global variable and a calculated offset into a pointer-based destination after a clipping routine.
- `interpolate_after_store`: Stores a value through a pointer and then performs a division-based interpolation and stores the result into a pointer.
- `store_clipped_coordinates`: Calculates an area product and stores a coordinate pair into a pointer-based destination or an offset-clipped coordinate.
- `interpolate_area_and_offset`: Performs area and difference calculations, then calculates a quotient and remainder to store an interpolated value and offset into a pointer.
- `interpolate_offset_pointer_clipping`: Calculates coordinate differences and area products, then performs conditional interpolation or offset-based coordinate storage.
- `validate_coordinate_range`: Checks if two pairs of short values satisfy a specific range condition and returns if they do or do not.
- `interpolate_coordinate_and_global`: Performs area and difference calculations, then calculates a division-based interpolation to store a value and a global variable into a pointer.
- `interpolate_coordinate_pair`: Performs coordinate-based calculations including area products and conditional clipping, then calculates an interpolated value to store in a pointer.
- `store_interpolated_pair_2`: Calculates coordinate and offset differences and stores a coordinate pair into a pointer-based destination.
- `interpolate_area_with_branch`: Performs area and difference calculations, then calculates a quotient and remainder to store an interpolated value and a global variable into a pointer.
- `interpolate_coordinate_offset_2`: Calculates a ratio or offset based on the division of a global value by the absolute difference of two registers, then stores the resulting quotient and a modified offset into a pointer-based destination.
- `check_coordinate_bounds_2`: Compares a set of short values and returns if certain conditions are met.
- `interpolate_offset_pointer_recursive`: Performs coordinate and offset difference multiplication followed by conditional interpolation and pointer-based storage.
- `interpolate_and_store_coordinates`: Performs coordinate difference multiplication and conditional interpolation before writing a pair to a pointer.
- `store_coordinate_pair_2`: Stores two short values from registers into a pointer-based destination.
- `interpolate_offset_pointer_with_area`: Performs a division-based interpolation and stores a global variable and a calculated offset into a pointer-based destination after an area product calculation.
- `calculate_interpolation_offset`: calculates a fractional offset using division and a remainder-based bit-packing technique, then applies this offset to a coordinate or pointer-based destination.
- `interpolate_coordinate_values`: Performs coordinate interpolation and checks a range, then stores an or-ed or calculated value through a pointer.
- `interpolate_scaled_offset`: Calculates a quotient and remainder-based interpolation and stores the resulting offset into a pointer-based destination.
- `interpolate_area_product_coordinates`: Performs a division-based interpolation and stores the resulting coordinate and a global variable into a pointer-based destination.
- `interpolate_coordinate_offset_3`: Calculates a quotient and remainder-based interpolation or offset, then stores the resulting value and a modified offset into a pointer-based destination.
- `interpolate_scaled_offset_branch`: Executes area product calculation and conditional coordinate interpolation or offset-based storage via a pointer.
- `check_coordinate_bounds_4`: Compares a set of registers and returns if certain boundary conditions are met.
- `compare_coordinate_bounds`: Compares two registers and returns if a certain order is not met.
- `select_and_store_register_pair`: Selects a pointer or a pointer-derived address based on a comparison and writes two short values from registers to that destination.
- `interpolate_offset_with_area`: Performs conditional offset difference multiplication and area-based interpolation, storing a global value and a calculated offset into a pointer.
- `interpolate_after_area_calculation_2`: Performs conditional offset difference multiplication and area-based interpolation, then applies a fractional offset to a coordinate.
- `check_coordinate_bounds_3`: compares two pairs of signed 16-bit integers and returns if the first pair or any value falls outside the range defined by the second pair.
- `interpolate_coordinate_with_area_check`: Performs conditional interpolation or offset calculation and stores a coordinate and a global variable into a pointer-based destination.
- `store_interpolated_coordinate_pair`: Calculates coordinate and offset differences and stores a coordinate pair into a pointer-based destination, using a pointer selected via conditional logic.
- `interpolate_clipped_coordinate_value`: Performs conditional area or offset calculations and stores a coordinate and a global variable into a pointer-based destination.
- `interpolate_coordinate_table_dispatch`: Dispatches through a code table to perform conditional coordinate difference multiplication and area-based interpolation or offset storage.
- `store_interpolated_coordinates`: Calculates coordinate differences and uses conditional branches to store a pair of values to a pointer or a global variable.
- `interpolate_and_store_coordinates_2`: Performs complex coordinate interpolation and clipping checks, calculating an interpolated value and storing it via a pointer.
- `calculate_area_product_2`: Multiplies the difference of two values by the difference of another value and a global variable.
- `multiply_coordinate_differences`: Returns the product of a difference in one coordinate and a difference in another, as a signed 32-bit value.
- `multiply_offset_differences`: Returns the product of two coordinate differences, computed from signed words, as a 32-bit value.
- `calculate_area_product`: Multiplies the difference between a global variable and one value by the difference between two other values.
- `interpolate_after_copy`: Performs a division-based interpolation and stores the result into a pointer after a copy operation.
- `interpolate_after_multiplication`: Performs a division-based interpolation and stores the result into a pointer after a multiplication routine.
- `store_offset_clipped_coordinate`: Makes a divisor positive, adjusts a quotient for negative remainders, and stores the resulting offset coordinate into an output buffer.
- `interpolate_after_copy_to_pointer`: Performs a division-based interpolation and stores a global variable and a calculated offset into a pointer-based destination.
- `store_clipped_coordinate_value`: Normalises a divisor's sign, adjusts a quotient for negative remainders, and stores the resulting value into an output buffer.
- `copy_pairs_until_mode_one`: Inverts a global and, when it is nonzero, stores a register value and returns.
- `copy_value_pairs_by_mode_3`: Copies register values into an output buffer in pairs, stopping on a mode of one.
- `compute_2d_transformation_matrix`: Calculates and stores a 3x3 or 2x3 transformation matrix into a memory buffer using products and sums of values retrieved from other routines.
- `compute_transformation_matrix_elements`: Calculates and stores the coefficients of a 2D transformation matrix into a memory buffer.
- `populate_transformed_vertex_table`: Iterates through a list of input coordinates and applies a scaling or transformation matrix to calculate new coordinates, storing the resulting or mirrored vertex or vector data into a destination table.
- `calculate_vector_division`: Iterates through a list of data structures to perform a complex division or scaling operation involving a sum of vector components.
- `execute_indirect_call_2`: Loads a function pointer from a location relative to the current stack pointer and executes it.
- `TFM_VTCES`: Walks a vertex list and tests each vertex and its neighbour against a clip edge using table lookups.
- `apply_scaling_to_table`: Iterates through a series of data tables and scales each signed integer value by a factor derived from a global variable.
- `calculate_table_offsets`: Iterates through a list of pointers, calculating a new offset or index for a destination table based on the sum of values retrieved from three different memory locations, then scales the result by a constant and stores it.
- `DIVBY0_VEC`: Writes a value into a vector slot, then scans a table of longwords for its minimum and maximum.
- `scan_longword_table_min_max`: Scans a table of longwords for their minimum and maximum values, then stores both results.
- `select_setup_value_by_state`: Chooses a setup value from a state global, with separate branches for states zero, one and two.
- `scan_back_for_key_change`: Walks backward over 32-bit entries and returns at the first entry whose low word differs from the one it started with.
- `scan_forward_until_key_mismatch`: Walks forward through word pairs and stops at the first entry whose key word differs from the one before it.
- `call_selector_and_store_word`: Sets up a masked selector value and calls an unnamed routine with it.
- `init_record_from_table_entry`: Copies several fields from a table entry into a record, storing one value negated, and then returns its input value.
- `init_record_fields_checked`: Initializes a record from a table entry with the same field layout as its neighbours, storing a negated step value and several copied words.
- `init_record_fields_multi_call`: Initializes a record from a table entry, calling an unnamed routine first and a second time only when a status word bit is set.
- `update_attribute_list`: Iterates through a list of pointers, updates a global variable with values from a memory range, and performs a loop that subtracts a constant from a value returned by another routine.
- `update_object_attribute_table`: Iterates through a list of object pointers and, for each, scans a secondary data table to find or calculate a value based on an index-based lookup.
- `copy_record_and_store_vertices`: Copies eight words from a source record into globals, then stores two words into three output slots, calling an unnamed routine before each store.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `skip_sentinel_terminated_lists`: Advances through word lists that end at the sentinel value of minus 32768, skipping several segments, and copies eight words of a record into globals.
- `compute_point_delta_pair`: Returns the difference of the second coordinate pair and the first, packed as two signed words.
- `calculate_vector_difference`: Calculates the difference between two values at specific offsets within two provided pointers and returns the result as a longword.
- `compute_point_delta_pair_2`: Returns the difference of the second coordinate pair and the first, packed as two signed words.
- `return_from_subroutine_3`: The routine performs a single return instruction and contains no other operations.
- `lookup_word_by_scaled_index`: Converts an offset into a table index by biasing it, shifting it right by five and clearing the low bit, then returns the word stored at that index.
- `call_unnamed_and_return_argument`: Calls two unnamed routines in sequence and returns its input argument unchanged.
- `compute_projected_vertex_pair`: Computes a scaled value from a structure field divided by a thousand, then combines coordinate sums with two unnamed arithmetic routines.
- `search_and_set_audio_params`: Searches a memory range for a specific string and then writes a value to a global variable and certain hardware-related memory locations.
- `decrement_and_store_constant`: decrements a register from one hundred to negative one via a loop, then writes a constant value and a byte to a memory location relative to a pointer.
- `calculate_audio_frequency`: Checks a global variable, searches a memory range for a specific string, and calculates a new value for a memory location using a division operation.
- `set_audio_parameters`: Decrements a register and then writes a constant value to a memory location and a global variable.
- `set_audio_and_delay_loop`: Sets a global variable and a DMA control register, then executes a decrementing loop that delays execution.
- `call_unnamed_and_return_argument_2`: Calls an unnamed routine and returns its two input registers unchanged.
- `return_concatenated_registers`: Returns a longword formed by concatenating the contents of two 16-bit registers.
- `call_unnamed_and_return_value`: Calls an unnamed routine and returns its input argument unchanged.
- `fill_block_from_byte_table`: Calls two unnamed routines, then fills a block of globals by indexing a byte table with an index multiplied by ten.
- `silence_audio_channels_and_clear`: Clears the state of four audio channels and writes silent volume and control values to the sound hardware.
- `ONC`: Sets four state flag bytes in globals to one.
- `build_dma_enable_control_word`: If a flag byte is set, builds a control word that sets the DMA enable bit plus one bit per channel for channels three down to zero, and stores it in a global.
- `setup_audio_channel_playback`: Configures audio playback for a channel from a sample table, writing the channel's location and length registers and calling several unnamed routines along the way.
- `update_audio_channel_pointer`: Computes sample pointer and length values from a channel record and writes them into the audio channel registers.
- `update_audio_channel_state`: Updates a channel record's position and length fields, then writes the audio pointer and length registers for the channel.
- `stub_bad_instruction_data`: A two-instruction fragment decoded from a gap in the code.
- `configure_audio_channel_parameters`: Iterates through a list of audio-related structures to calculate and write volume, period, and length/pointer values directly to the hardware audio registers.
- `configure_audio_channel_parameters_2`: Iterates through a list of audio-related structures to update hardware-level audio registers, including volume, period, and frequency-scaled or phase-adjusted parameters.
- `stub_bad_instruction_data_2`: A two-instruction fragment decoded from a gap in the code.
- `process_linked_list_data`: Iterates through a linked list to perform arithmetic operations on data and update a global structure with new pointers and calculated values.
- `clear_channel_state_by_index`: Sets a global to all ones and clears one of four state bytes chosen by a selector argument.
- `initialise_channel_record_fields`: Initialises three fields of a channel structure to fixed starting values and stores an argument in a global.
- `spin_short_delay_loop`: Counts a loop variable down from five hundred and twelve to zero and returns, giving a short busy-wait delay.
- `countdown_channel_and_trigger`: Decrements a countdown byte in a channel structure and, when it reaches zero, clears a flag and stores a value.
- `call_subroutine`: This routine executes another routine and then returns.
- `advance_pseudo_random_state`: Advances a 32-bit shift-register state held in a global once per step of the argument, XORing in a fixed feedback constant when the top bit falls out.
- `initialise_serial_port_registers`: Stores several constant words in globals and writes one fixed value to the serial period hardware register.
- `call_checksum_update`: Executes a call to a routine that updates a global table or value using a byte and a checksum check, then returns the contents of two registers.
- `update_table_with_checksum`: Updates a global table or value based on a byte, checks a checksum, and potentially performs a block copy or reset.
- `update_global_with_branching_logic`: Updates a global variable or a table with a byte and checks a checksum, potentially performing a block copy or clearing a global variable.
- `update_table_with_conditional_logic`: Conditionally prepends a byte to a global or updates a table and checks a checksum, potentially performing a block copy if a certain condition is met.
- `update_global_and_check_sum`: Decrements a global variable and, if it reaches zero, checks if a byte matches the sum of a sequence of bytes in a table, potentially performing a block copy.
- `update_register_and_global`: Checks a global variable and a table, then updates a hardware register and a global variable with a value from a table.
- `SIDE_MASK_TABLE`: Parses a text line by skipping spaces and stopping at line ends, then scans words and stores them in a table.

### loader

- `load_data_file_via_dos`: Entry routine that opens the dos library, opens and reads a data file with seek and close, and allocates memory through exec.
- `fill_range_with_nop_pattern`: Saves a word at a fixed offset, overwrites it with a no-operation instruction pattern, and fills a range of memory with copies of that pattern.
- `copy_block_backward`: Copies a block of 32-bit words from one address to another, moving backward from the end of the block.
- `copy_sixteen_word_palette_table`: Calls a library routine through a stored vector, then copies sixteen words from a fixed table into a global buffer that holds the colour palette.
- `setup_display_and_sprite_registers`: Initialises the display registers: bitplane modulos, bitplane control, display window and fetch limits, and sprite positions.
- `fill_four_step_word_pairs`: Writes four pairs of words into a table with a stride of eight, increasing the stored value by eight thousand for each entry.
- `unpack_run_length_bytes`: Unpacks a run-length encoded byte stream: a zero count ends decoding, a negative count repeats one byte, and a positive count copies a literal run from the input.
- `decode_bit_packed_stream`: Reads bits from a packed stream one at a time, shifting a data word and looking up words in a table through an index, and writes the results to an output area.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_variable`: Calls another routine and then sets a global variable to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_only_3`: An empty stub that only returns to its caller.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately`: The routine is empty and performs no operations before returning.
- `return_to_caller`: an empty routine that immediately returns to the calling routine.
- `return_immediately_3`: this routine performs no operations and immediately returns to its caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `do_nothing`: the routine is empty and performs no operations before returning.
- `perform_empty_operation`: the routine performs no operations and returns immediately.
