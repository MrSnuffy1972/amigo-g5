# Game 1283 (ruffian (1994)(grandslam)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01283). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2728624 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 139 of 139 named (0 author symbols), 139 of 139 described (2 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1555 of 1555 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6578 of 6606 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 143707 |
| code | 23360 |
| data | 2535443 |
| padding | 2 |
| unused | 26112 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ruffian (1994)(Grandslam)(Disk 1 of 3).adf:bootblock | 1 | 81 | 5 |
| logged disk load from Ruffian (1994)(Grandslam)(Disk 1 of 3).adf boot-block load | 1 | 1050 | 57 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5470 | 76 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |

## Cases handled

- ruffian (1994)(grandslam)(disk 1 of 3)/Ruffian (1994)(Grandslam)(Disk 1 of 3).adf: custom-format (non-DOS) disk, 1740 non-empty blocks: matched against track-loaded segments
- ruffian (1994)(grandslam)(disk 2 of 3)/Ruffian (1994)(Grandslam)(Disk 2 of 3).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- ruffian (1994)(grandslam)(disk 3 of 3)/Ruffian (1994)(Grandslam)(Disk 3 of 3).adf: custom-format (non-DOS) disk, 1707 non-empty blocks: matched against track-loaded segments
- ruffian (1994)(grandslam)(disk 1 of 3)/Ruffian (1994)(Grandslam)(Disk 1 of 3).adf: the boot block reads 153088 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1550 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ruffian (1994)(grandslam)(disk 1 of 3)/Ruffian (1994)(Grandslam)(Disk 1 of 3).adf: 1 logged loads cover 153088 bytes of the image
- ruffian (1994)(grandslam)(disk 1 of 3)/Ruffian (1994)(Grandslam)(Disk 1 of 3).adf: blocks 301-1759 (bytes 154112-901120) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.01)
- ruffian (1994)(grandslam)(disk 2 of 3)/Ruffian (1994)(Grandslam)(Disk 2 of 3).adf: blocks 2-1759 (bytes 1024-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.59)
- ruffian (1994)(grandslam)(disk 3 of 3)/Ruffian (1994)(Grandslam)(Disk 3 of 3).adf: blocks 2-1046 (bytes 1024-536064) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.93)
- ruffian (1994)(grandslam)(disk 3 of 3)/Ruffian (1994)(Grandslam)(Disk 3 of 3).adf: blocks 1098-1759 (bytes 562176-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.79)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ruffian (1994)(Grandslam)(Disk 1 of 3).adf:bootblock

- `load_boot_block_program`: Runs as the boot block: it allocates memory, issues disk read requests through exec for the program's blocks, waits for the blitter, writes the background colour register and enables DMA channels before passing control to the loaded code.
- `store_register_to_memory`: Stores a register value into a memory location offset from a pointer and then jumps to an indirect address.
- `copy_global_block_to_memory`: Copies 256 longwords from a global area to a destination address, offset by the base register when the given value is in a low range.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `disable_all_interrupts_and_dma`: Writes the same value with every bit set except the enable bit to three hardware-register globals, which clears the enable bits for interrupts, DMA and requests.

### logged disk load from Ruffian (1994)(Grandslam)(Disk 1 of 3).adf boot-block load

- `run_paired_call_loop`: Calls two routines alternately in a loop, stepping a counter from zero up to 110 in steps of two, then returns.
- `halt_in_infinite_loop`: Loops forever with no exit, so execution never returns from this routine.
- `fill_palette_entries_bright`: Writes the maximum 12-bit colour value into 30 consecutive 16-bit entries in memory, spaced four bytes apart.
- `repeat_two_calls_count`: Calls two routines in each pass of a counter loop that runs down to zero, then returns.
- `find_highest_set_bit`: Scans the bits of a 16-bit value from the top down and returns as soon as it finds a set bit.
- `draw_bit_pattern_into_rows`: Writes a bit pattern derived from the low three bits of the column into bytes of two rows, extending the fill into neighbouring bytes when the column offset is wide enough.
- `halt_after_initial_call`: Calls a routine once and then enters an endless loop, so it never returns.
- `copy_interleaved_plane_bytes`: Advances a frame counter stored in a global, then copies a block of bytes from the source into the destination, spreading each source group across four offsets 40 bytes apart.
- `shift_longword_rows_forward`: Copies 256 longwords one place forward through several interleaved bit rows, shifting the data across those rows in a single pass.
- `copy_offset_indexed_byte_pairs`: Repeatedly takes an offset from another routine and copies a byte pair from the table base to fixed destination locations, writing a fixed sequence of 16 pairs.
- `compute_character_cell_index`: Maps a character code into a range using a fixed offset and a conditional adjustment, then combines the result with a row value scaled by 18 to produce a table index.
- `fade_palette_down_to_targets`: Steps the red, green and blue components of 32 palette entries down by one whenever the matching target value is lower, repeating until no component changes.
- `fade_palette_entries_to_black`: Repeatedly decrements each non-zero colour component of 32 palette entries, calling another routine between passes, until every entry is zero.
- `wait_for_character_transitions`: Runs a counted loop in which it spins while a character byte at a fixed offset is a space, then spins while it is not a space, repeating for the count.
- `clear_table_then_run_passes`: Calls another routine, fills 24 blocks of 200 bytes with all-ones values, then runs two counted loops of up to 80 and 50 passes, calling a set of routines in each pass.
- `empty_infinite_loop`: This routine is empty and performs an infinite loop.
- `copy_plane_byte_triples`: Copies byte triples from the source into three rows of the destination spaced 40 bytes apart, with 13 groups per row across 105 rows.
- `iterate_with_bit_checks`: Scans for the highest set bit in a longword and draws a bit pattern before decrementing a counter in a loop.
- `find_highest_set_bit_long`: Scans the bits of a 32-bit value from the top down and returns as soon as it finds a set bit.
- `draw_bit_pattern_with_padding`: Writes a bit pattern for the given column into two rows, zeroing the bytes on either side and filling the bytes between with all-ones when the column offset is non-zero.
- `clear_row_pair_bytes`: Writes zero to two bytes in the row above and the row below the current position, using a 200-byte row stride.
- `copy_clipped_plane_triplets`: Copies byte triples from the source into three planes of the destination, 14 positions per row over 24 rows.
- `run_steps_every_third_call`: Increments a counter, and on every third call resets it and runs four routines in turn.
- `restore_saved_colour_group`: If a saved-state flag is non-zero, copies five groups of three stored values from a source table into the destination, spreading each group across three positions 40 bytes apart and advancing 200 bytes per group.
- `compute_row_and_column_offsets`: Derives a value from a global counter by reflecting it into a small range and adding a table-driven adjustment, then returns a row offset of 200 bytes per step together with a column offset in words.
- `merge_shifted_plane_words`: Shifts five groups of three source words by the value in a global, then combines each group into three destination planes using bitwise logic, stepping 50 words per group.
- `save_colour_group`: Sets a saved-state flag and copies five groups of three values from the given buffer into a saved table, stepping 200 bytes per group.
- `clear_pixels_from_coordinate_list`: Reads coordinate pairs from a list until it reaches a negative entry, clearing one pixel bit in the bitplane for each pair.
- `set_record_default_values`: Stores four fixed constants into fields of a structure: one 32-bit value and three 16-bit values, at offsets 160, 164, 166 and 168 bytes.
- `initialise_four_slot_records`: Stores a fixed 32-bit value and a count of one into four records spaced 16 bytes apart in the structure.
- `clear_bitmap_region`: Clears a block of memory by writing zero longwords down 256 rows spaced 200 bytes apart, then advancing 40 bytes across columns for each outer pass.
- `run_calls_when_flag_set`: Calls two routines only when a global value is non-zero.
- `build_lookup_table_from_source`: Builds a lookup table of 31 values from a base address and a per-entry increment taken from a source table, clears a set of 32-bit pointer slots, and resets several state fields.
- `advance_six_step_channel_sequence`: Advances a six-step counter, and on each wrap reloads its value from a table and runs four routines.
- `step_channel_period_and_volume`: Steps a channel's period and volume by amounts taken from a byte, clamping the period between 113 and 1336 and the volume to a maximum of 64, then writes the results to the channel's registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_audio_parameters`: Updates audio channel period or volume by adding or subtracting a value, applying a minimum or maximum clamp, and writing the result to a hardware-related register or memory location.
- `step_channel_period_down`: Decreases a channel's period by a byte-sized amount, clamps it to a minimum of 113, and writes it to the channel's period register.
- `step_channel_period_up`: Increases a channel's period by a byte-sized amount, clamps it to a maximum of 1336, and writes it to the channel's period register.
- `step_channel_volume_by_nibble`: Adjusts a channel's volume by the high or low nibble of a byte, raising it up to a maximum of 64 or lowering it to zero, then writes the result to the channel's volume register.
- `step_channel_volume_down`: Decreases a channel's volume by the low nibble of a byte, floored at zero, and writes it to the channel's volume register.
- `lookup_period_by_mode`: Selects a nibble from a byte according to a mode value, then searches a table of 16-bit entries for a matching key and writes the paired value to the channel output.
- `lookup_entry_by_high_nibble`: Searches a table for an entry whose 12-bit key matches the channel's stored key, then writes the table value at the index given by the high nibble of a byte to the channel output.
- `lookup_entry_by_low_nibble`: Searches a table for an entry whose 12-bit key matches the channel's stored key, then writes the table value at the index given by the low nibble of a byte to the channel output.
- `copy_channel_stored_value`: Copies the channel's stored 16-bit value into its output field without other changes.
- `lookup_table_entry_by_index`: Searches a table for the channel's 12-bit key, then writes the entry at the index passed in a data register to the channel output.
- `store_channel_value_directly`: Writes the value passed in a data register straight into the channel's output field.
- `start_channel_playback`: Looks up a period from a table for the channel index, waits through short delays, enables the channel's DMA bit in the DMA control register, and sets the sound-channel pointers for each flagged channel.
- `return_without_action`: Contains only a return, so calling it does nothing.
- `decode_tracker_note_event`: Reads a four-byte note event from a sequence and extracts the instrument number from its nibbles.
- `apply_tracker_effect_command`: Stores the note's period and sample pointer into the channel state, then runs the effect command in the low nibble of the second byte: a position jump, a volume set, or an extended command.
- `apply_position_jump_effect`: Inverts a global value and stores the channel's byte operand minus one in its place, which the effect handler uses as a jump position.
- `store_channel_volume_byte`: Copies one byte from the channel's parameter into the output volume field.
- `invert_global_flag`: Inverts a global value, flipping all of its bits.
- `set_global_flag_from_channel`: Copies bit zero of the channel's parameter byte into bit one of a global flags word while keeping its other bits.
- `store_nonzero_nibble_global`: Stores the low nibble of the channel's parameter byte into a global, but only when that nibble is non-zero.
- `modify_global_variable_bits`: If a global variable is non-zero, it performs a bitwise NOT and then decrements the value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_main_input_loop`: Repeats a main loop that waits for a timing signal, calls its update routines, and checks the right and middle mouse buttons through the port register.
- `clear_flag_and_set_counter`: Clears bit six of a flag byte at a fixed offset and stores 8 into a counter word, then returns the value passed to it.
- `copy_state_and_process_object`: When a flag byte is set, copies several 32-bit state values into 16-bit globals in turn, then calls a series of other routines that process the object's state and returns a 64-bit result.
- `approach_target_speed`: Steps a speed value toward a target derived from a position offset, using small clamped increments, and uses a different target when a secondary flag is set.
- `compute_position_adjust_result`: Compares a masked position value with a fixed threshold and, when it lies above or below, nudges two coordinate fields by two units and returns plus or minus two.
- `update_object_state_branches`: Reads several status fields in turn and, depending on which are set, clears counters, resets motion values and calls a series of other routines.
- `update_direction_flags_from_codes`: Compares four codes passed in registers with a fixed value and, depending on the sign of a speed field, sets or clears one of two direction flags and resets that speed field when a match is found.
- `set_state_parameters_by_index`: Reads a state index and, for each index, sets a flag byte, clears a counter and assigns preset values to the speed and timer fields before returning.
- `update_counter_from_state_flags`: Checks a state index and, for the matching state, resets several counters and steps a direction counter up or down according to the sign of a speed field.
- `countdown_and_reset_state_timer`: Counts down a timer field and, while it is running, adjusts a second field by fixed steps and sets a state pointer.
- `reset_motion_fields`: Clears two fields and sets a third to 40 in the structure.
- `set_trigger_when_conditions_met`: Sets a trigger flag and an indicator field when a counter is above 83, a masked status value matches, and six related fields are all zero.
- `handle_fire_button_state`: Checks a status byte and an object pointer against two fixed values, then selects a new state pointer and sets flag bytes based on counters and a related field.
- `advance_timer_load_table_values`: Advances a timer field up to 150 and sets a ready flag when it expires.
- `move_and_check_tile_collision`: Advances a position by its velocity, then looks up the map cell under the new position and checks its tile type against a fixed value and a per-tile table.
- `update_timer_and_validate_table_bounds`: Updates a timer-related counter and checks a lookup table to determine if a direction or boundary change is required.
- `step_toward_target_position`: Compares a position sum with a target field and sets a speed field to move toward it in steps of up to eight units, marking a direction flag.
- `copy_frame_table_rows`: Copies four blocks of 18 longwords from a table selected by an index field into a destination buffer, placing each block 21 longwords after the previous one.
- `load_blitter_setup_from_table`: Waits until the blitter is idle, then writes a set of control words and table-indexed values into the destination structure and memory locations, derived from an index field.
- `compute_special_state_value`: When the state index is 14 and a flag is set, computes a value from two position fields, stores the previous value in a backup field and derives a new 4-bit value.
- `update_motion_and_bounds`: When a flag is set, increments a counter up to a limit, clamps a horizontal value against an upper bound, scales a velocity by fixed factors into a position field, then calls other routines.
- `wrap_offset_and_update_limit`: When a horizontal offset is negative, adds 304 to it and adjusts a base position to match.
- `unwrap_offset_and_update_limit`: When a horizontal offset exceeds 303, subtracts 304 from it and adjusts a base position to match.
- `copy_column_into_ring_buffer`: Copies 18 words from a table column spaced 256 words apart into a ring buffer whose write index wraps after 19 entries.
- `copy_buffer_data`: Copies a sequence of words from a source pointer to a destination buffer, incrementing the source pointer by a fixed stride and resetting a counter-based index.
- `update_plane_pointers_for_frame`: Computes a frame base from a counter and a pointer field, copies four saved pointer values into their destinations when a flag is set, and writes new pointer values for a vertical position derived from two fields.
- `check_object_bounds_and_reset`: Checks an object's horizontal and vertical offsets against a window of allowed values.
- `process_timed_event_entries`: Walks a list of timed entries and, when an entry's position falls within a small area around the object, updates that entry's table pointer and triggers a follow-up action.
- `queue_blitter_draws_from_table`: Walks a table of up to 49 draw entries.
- `tally_event_when_in_range`: When an object's position is inside a fixed range and an indexed record matches a fixed type code, marks that record as handled, decrements one counter, increments another and sets a flag.
- `trigger_overlapping_entries`: When a related state field is zero, scans a list of entries whose length is stored in a field.
- `check_overlap_with_entry_type`: Tests whether the object overlaps an entry's rectangle.
- `execute_indirect_call`: Calculates a target address by scaling a value from a global variable and adding an offset, then executes a call to that address.
- `advance_and_wrap_counter`: Adds a per-entry increment to a counter and resets it to zero when it reaches the limit stored in the entry.
- `compute_hitbox_for_entry`: Computes an entry's bounding area from the object's base position, applying offsets that depend on the entry's type and sub-state, then tests overlap and records the result.
- `run_object_state_machine`: Runs a long state machine for the object: it reads a stored state index and, for several index values, sets state pointers, timers and direction flags and calls ten other routines.
- `combine_status_flags_for_entry`: If a timer field is non-zero, sets a flag on the entry and returns the input value.
- `combine_status_fields_and_flag`: ORs three status fields together, clearing one flag when the result is non-zero and setting two flags when it is zero, then returns the combined value.
- `dispatch_on_state_flag`: Calls one of two routines depending on whether a state field equals 1.
- `spawn_type_seven_entry`: Searches up to eight entries in an entry table for an unused one and, if found, fills it with type code 7 and the object's offset, position and speed values, marking it active.
- `spawn_type_ten_entry`: Searches up to eight entries in an entry table for an unused one and, if found, fills it with type code 10, sets a 127 value in a separate field, and stores the object's offset, position and speed values.
- `dispatch_object_code_handlers`: Masks an object code word and, for each of several specific codes, updates a counter or flag in the object's state.
- `initialise_six_slot_entries`: Stores a status byte into six slots, sets matching position and value words from the given inputs, and zeroes six counter fields in the same slot table.
- `initialise_entry_block_positions`: Sets a control word to a fixed value, copies two stored position values into six entries, sets their count fields to 50 and clears six related counters.
- `fill_slot_fields_from_registers`: Writes a byte and two word values held in registers into six slot records at fixed offsets, spreading them across each record's fields.
- `advance_ring_slot_and_store`: Advances a ring index by 22 bytes, wrapping past its end, then sets a flag and stores a position word and a value into the chosen slot.
- `process_active_entry_list`: When either of two active flags is set, walks a list of up to 50 entries and runs handlers for each active entry, advancing 28 bytes per entry.
- `check_entry_hit_and_flag`: Tests whether an entry's rectangle contains the given point after an offset step.
- `steer_velocity_toward_target`: When the object is within a 48-unit range of a target on each axis, nudges that axis's velocity by one step toward the target, clamping the results to a small range.
- `store_position_offsets_with_flag`: Stores the two input positions, each offset by minus 16, into fixed fields and sets a flag byte.
- `spawn_slot_at_position`: Decrements one counter and increments another, sets an update flag, stores a position and a fixed timer into a slot, then selects one of two frame codes depending on whether the object is past a horizontal threshold.
- `run_timed_sequence_step`: When a trigger field is set, counts down a step counter and a repeat counter, resetting them at their limits, and calls two other routines to update the sequence state.
- `copy_thirteen_rows_of_words`: Copies 13 words from a source block into a destination at 13 positions spaced 46 words apart.
- `or_thirteen_rows_of_words`: ORs 13 words from a second source into a destination block spaced 46 words apart, combining the two sources.
- `select_frame_by_sequence_index`: When the first field is set and a set of related fields is clear, selects a frame source from a switch on the low three bits of a counter field.
- `advance_table_pointer_on_counter`: Updates a rotating bit counter from a register and, every 32 steps, selects the next data pointer from a fixed table of pointers, copying the matching entries into the output.
- `copy_block_rows_into_frame`: Copies many longword values from several source areas into a frame buffer in one long sequence of stores, calling six other routines along the way.
- `toggle_frame_pointers_on_timer`: Counts a timer down.
- `step_timed_frame_sequence`: Counts a sequence down and, while it runs, updates two pointer pairs and a value.
- `set_global_flag_bit_one`: Sets bit zero of a global flags word.
- `set_global_flag_bit_two`: Sets bit one of a global flags word.
- `set_global_flag_bit_2`: Sets bit 2 (value 4) in a global variable and returns.
- `set_global_flag_bit_3`: Sets bit 3 (value 8) in a global variable and returns.
- `copy_table_row_strided`: Selects an 80-word row from a lookup table using the low 11 bits of a register, then copies it into a destination array at regular word spacing.
- `clear_listed_bits_then_scan`: Clears one bit in each byte named by a pointer table, stopping at the first null pointer; each entry picks the bit from its low bits, and the same bit is cleared 46 bytes further on.
- `claim_bit_slot_if_free`: Works out a byte address and bit number from a control block and an offset.
- `advance_state_machine_step`: Reads a step number from a control block and runs the action for that step, such as resetting flags, subtracting a table value from a position, or adjusting table entries.
- `run_step_action_and_reset`: Runs the action for a step number stored in a control block, covering the same step set as its sibling dispatcher but returning no value.
- `clear_one_of_two_fields`: Tests bit 0 of a flag byte in a control block.
- `update_position_and_claim_slot`: When its counters are idle, it moves a stored position with wraparound and checks a value in a table at that position.
- `process_active_object_list`: Walks a list of 22-byte entries.
- `register_entry_in_table`: Checks whether an entry lies within a window around the control block position.
- `call_routine_from_byte_table`: Uses the byte at the input pointer as an index into a jump table and calls the routine stored there.
- `move_entry_with_tile_collision`: Moves one entry each frame by a step taken from the tile values sampled at its position.
- `move_entry_toward_target_position`: Moves one entry by a per-frame amount and, while the reference flag is set, nudges it toward a target position when it is close enough.
- `advance_entry_animation_state`: Updates one entry's animation: it counts down a delay, sets a frame code, and steps the position, choosing a direction from nearby values and a timer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spin_wait_on_control_byte`: Repeats a nested loop over a counter, with the inner loop waiting on a byte in a control block.
