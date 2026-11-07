# Game 375 (Rod-Land): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00375). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (943664 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 217 raw-word lines |
| Functions named and described | pass, 164 of 164 named (0 author symbols), 164 of 164 described (1 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1621 of 1621 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3980 of 3980 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21164 |
| code | 12612 |
| data | 15732 |
| padding | 716 |
| unused | 893440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rod-Land.adf:Type | 2 | 117 | 2 |
| Rod-Land.adf:bootblock | 1 | 35 | 1 |
| logged disk load from Rod-Land.adf boot-block load | 1 | 94 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 403 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3327 | 128 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |

## Cases handled

- Rod-Land/Rod-Land.adf: the boot block reads 11264 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- runtime segment an address: 226 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1391 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Rod-Land/Rod-Land.adf: 1 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rod-Land.adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_and_call`: When a destination pointer is provided, copies sixteen long words from a table at a fixed offset into it.

### Rod-Land.adf:bootblock

- `read_disk_blocks_into_memory`: Sets up disk read requests with fixed lengths and destinations, sends them through the device I/O routine, and allocates memory through a library call to point a further read at it.

### logged disk load from Rod-Land.adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spin_calling_routine`: Calls one routine inside an endless loop, so control never returns from this entry point.
- `sign_extend_input_word`: Returns its input word as a signed long, sign-extended to the full register width.
- `decrement_indexed_word_counter`: Uses a byte from a structure field, doubled, as an offset into a table and decrements the word at that offset.
- `add_xor_constant_to_word`: Stores into a fixed slot the sum of a second slot, one, and a stored word XORed with a constant.
- `wait_for_nonzero_input`: Loops calling a routine until its result is non-zero.
- `allocate_eight_byte_aligned_block`: Rounds a requested size up to a multiple of eight and searches a free list for a block at least that large.
- `free_memory_block_to_list`: Returns a block to a free list.
- `merge_with_following_free_block`: Checks whether the free block that starts right after this block is adjacent, and if so absorbs it by adding its size to this block's size and taking over its link.
- `pass_result_to_routine`: Calls one routine and passes the value it returns as an argument to a second routine.
- `sum_free_block_sizes`: Walks a linked list of blocks, summing their sizes and tracking the largest one, and returns the total.
- `difference_between_fields`: Returns the difference between a word field of a structure and the word that a second pointer in the structure refers to.
- `decrement_structure_counter`: Decrements a 16-bit counter field in the structure it is given.
- `load_word_from_pointer_field`: Loads a 16-bit value from the location a pointer field refers to and stores it in a second field of the same structure.
- `delegate_to_one_routine`: Calls one routine and returns the two registers it produced unchanged, adding no logic of its own.
- `store_register_long`: Stores a register value into a long field of a structure at a fixed offset.
- `store_register_to_field`: stores a register value into a long field of a structure passed as an argument.
- `store_register_into_field`: Stores a register value into a long field of the structure it is given, at a fixed offset, and returns.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `wait_until_counter_advances`: Records a counter value and then repeatedly calls a routine until the counter has advanced by at least the requested amount.
- `repeat_call_count_times`: Calls one routine repeatedly, counting the input word down from its value to zero.
- `init_empty_circular_list`: Calls two routines and then sets up an empty circular list head inside the structure with a zero counter word.
- `init_empty_circular_list_2`: Initializes a circular list head at the structure's base and sets a zero counter word at a fixed offset, linking the list pointer back to the head.
- `bubble_sort_list_by_key`: Makes one pass over a circular linked list, swapping adjacent entries whose 16-bit key field is lower than their predecessor's.
- `accumulate_table_words_with_routine`: Loops a fixed number of times, calling a routine on each pass and adding a word from a table to a running total.
- `repeat_call_while_flag_set`: Calls routines repeatedly while a condition flag from the caller is set.
- `wait_byte_then_copy_word`: Waits while a status byte at a fixed offset is nonzero, calling routines in the loop.
- `update_channel_record_by_flags`: Checks flag bits in a channel record and a global byte, and if they allow it, calls routines that update the record's words.
- `advance_byte_stream_player`: Counts down a timer field and, when it expires, reads the next byte from the stream a record field points to.
- `reset_stream_state`: Clears a stream pointer, a second pointer and two 16-bit counters in the record.
- `start_stream_playback`: Sets the record's stream pointers to the input address, stores the input word as a reload value and starts the countdown timer at one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_channel_record_defaults`: Sets several default fields in a channel record, calls one routine, adjusts a flag byte based on one of its bits, and then enters a loop that calls routines.
- `step_channel_state_machine`: Runs one step of a channel state machine selected by a state word.
- `compute_scaled_countdown_value`: Computes a value from a countdown field, scaled by two.
- `step_field_if_below_limit`: Clears a result byte.
- `reload_field_from_pointer`: Clears a flag byte and calls a routine.
- `return_without_effect`: Contains only a return in the decompiled body, so it changes no memory or registers.
- `do_nothing_routine_2`: this routine is empty and performs no operations before returning.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `set_pursuit_target_and_timer`: Stores the second object pointer as the target, sets the direction word to 1 or minus 1 by comparing the two horizontal positions, moves the vertical position up by eight units, and loads two timers with 50 and 10.
- `step_toward_target_horizontally`: Returns to the pursuit setup when the horizontal gap exceeds 56; otherwise it shifts a saved position word, steps both the object and its target 10 units toward each other horizontally, and calls a subroutine before setting a timer to 4.
- `select_actor_state_from_flags`: Chooses a new actor state from two flags in the game globals: when the first flag is clear it adopts a pending state and polls up to 24 frames, and when it is set it selects state 15 or 20 depending on a second flag.
- `clear_pending_target_on_miss`: When the preceding test fails it clears the byte at the output pointer, runs two subroutines, resets the state word to zero and returns minus one; otherwise it returns the test result unchanged.
- `check_stored_point_proximity`: Returns without action unless the vertical position is zero; when a global flag and a stored point are set, it returns early if the object lies within the tolerance of that point on both axes.
- `test_point_inside_tolerance_box`: Returns early when the supplied coordinate pair lies within the tolerance box of a stored object position, and otherwise leaves the state unchanged.
- `check_alternate_point_proximity`: Like the stored-point check, but gated by two flags and tested against a second stored point, returning early when the object is within tolerance on both axes.
- `test_point_inside_secondary_box`: Returns early when the supplied coordinate pair lies within the tolerance box of a second stored position, which is only tested when two flag bytes are set.
- `scan_hazard_table_for_contact`: When a global flag is set, scans a table of up to four coordinate pairs and returns minus one for the first pair within tolerance on both axes, or zero when none match.
- `scan_pointer_table_for_contact`: Scans a table of up to four coordinate pairs held behind a record pointer, returning minus one for the first pair within the record's tolerances on both axes, or zero when none match.
- `check_fixed_rectangle_contact`: When a flag is set and the timer word is above 16, tests whether the object's horizontal position, offset by 24, lies within 5 units and its vertical position within 10 units of a stored point, returning early on contact.
- `check_rectangle_contact_at_pointer`: Does the same rectangle contact test as the fixed-rectangle check, but reads its flag, timer and stored point from a record pointer supplied by the caller.
- `enter_state_six_clearing_flag`: Unless the current state is 4 or 5, records the pointer, sets the timer to 84 and moves the actor to state 6; in every case it writes minus one to a byte of the record.
- `check_flagged_point_proximity`: When a flag at the object is set, tests whether the object lies within the tolerances of a second stored point, returning early on a match.
- `check_record_point_within_box`: When the flag byte of the record is set, returns early if the supplied point lies within the record's stored box on both axes.
- `run_actor_state_machine_step`: Counts down a delay timer and, once it reaches zero, dispatches on the current action code to branches that set a follow-up timer and move the actor to state 7, returning a value from the chosen branch.
- `dispatch_optional_indirect_handler`: Calls a routine through the pointer in the address register, then calls a second routine through a pointer when the zero flag is set.
- `check_record_ahead_within_range`: Returns minus one when the flagged record has a smaller vertical value than the actor and lies within 33 units horizontally, and zero otherwise.
- `check_record_offset_band_contact`: Returns minus one when the flagged record is at or beyond the actor vertically and its horizontal offset, adjusted by a direction bit, lies between 32 and 64 units; otherwise zero.
- `check_record_level_contact_band`: Returns minus one when the flagged record sits at the same vertical value as the actor and its horizontal offset, adjusted by a direction bit, is under 65 units; otherwise zero.
- `check_record_level_contact_wide`: Runs only while a global pointer is clear, then applies the level contact test with a wider horizontal limit of 129 units and returns minus one on a hit.
- `enter_state_eight_by_action`: In state 13 it checks whether a tracked point lies within 12 units on both axes and updates a facing bit; otherwise, when a counter condition holds, it sets state 8 for a fixed set of action codes.
- `write_map_cell_when_aligned`: Counts down a timer and, once it has expired and the vertical position is aligned to a 16-unit grid, computes the grid cell from the position and stores the actor's code byte into that cell.
- `update_direction_from_neighbour_cells`: Sets a direction byte from the action code and from a vertical comparison with a stored value, then probes neighbouring grid cells in turn and returns the last probed value.
- `apply_requested_direction_state`: Checks a two-bit requested direction against the current and opposite stored directions and, when it is new, sets the matching direction code, state timer and movement bit, returning the probe result.
- `apply_requested_direction_only`: Applies a two-bit requested direction in the same way as the direction-state routine but without comparing against the current direction, setting the direction code, timer and movement bit when the probe succeeds.
- `choose_direction_toward_target`: Compares the target's horizontal position with the actor's, chooses a direction value of 0, 1 or 2 according to the action code, and when that direction is new, probes it and stores it with a timer of 4.
- `choose_vertical_direction_from_record`: Compares the actor's vertical position with a record's and, when the horizontal gap exceeds 15 units, probes a direction and stores it with a timer of 4.
- `check_global_point_proximity_band`: When the vertical position is zero and a global flag is set, returns early if the object lies within 64 units horizontally of a global point and its vertical distance to that point is no more than the horizontal offset.
- `check_record_proximity_band`: Returns early when the record's flag is set and the point lies within 64 units horizontally, with a vertical distance no larger than that horizontal offset.
- `set_facing_bit_from_position`: Clears the low bit of a flags byte, then sets it when the actor's horizontal position is less than the value returned by a subroutine.
- `set_facing_bit_from_sign`: Clears the low bit of a flags byte and sets it again when a called routine returns a negative value.
- `read_global_word_value`: Returns a 16-bit word stored at a fixed offset in the global game record, with no other processing.
- `clamp_horizontal_position`: Clamps the horizontal position to the range 8 to 248 by snapping it to the nearer limit whenever it falls outside that range.
- `collect_grid_cell_bit`: Converts the actor's position to a grid cell and, if that cell's bit is set in a bitmask table, clears it, nudges the actor onto the cell, bumps a capped counter on its record and decrements a global remaining count, calling a routine at zero.
- `collect_grid_bit_from_argument`: Performs the same grid-cell bit clear and remaining-count update as the position-based routine, but takes the cell column from the argument register instead of the position.
- `do_nothing_routine`: The routine is empty and performs no operations.
- `snap_to_fixed_contact_point`: Clears a flag byte and, when the subroutine result is zero and a global contact point exists, snaps the vertical position to that point minus 28 if the actor is inside the contact window, setting the flag and returning minus one.
- `snap_to_record_contact_point`: Performs the same snap-to-contact-point test as the fixed-point routine, but takes the contact point from a record pointer supplied by the caller.
- `copy_position_into_new_record`: When the argument is nonzero, copies the actor's horizontal and vertical positions into a new record, stores the argument in it, adds the argument to a counter in the owner record and returns minus one; otherwise it returns zero.
- `pause_state_and_sync_position`: Saves the current state, sets state 18 and polls up to 24 frames for a linked record's movement value, copying that record's position into the actor when found, then restores the saved state and moves the vertical position up by one.
- `insert_rounded_block_into_list`: Rounds a block size up to a multiple of eight and links the block into a size-ordered list, updating a count before and after the insertion when the size and pointer are nonzero.
- `count_down_return_field_difference`: Counts a loop down from the argument, calling a routine on each pass, then returns the difference between a word in the actor record and a word held at a pointer the record references.
- `update_record_position_toggle_bit`: Copies the actor's position into its record, counts down a record timer, toggles the bit-two movement flag on a pattern chosen from the timer's high bits, and stores the vertical position in a field when a flag is clear.
- `update_actor_movement_and_collision`: Clears a state bit and enters state 18 when a movement-cancel flag is set; otherwise advances the actor horizontally by 32 units when the tile bitmap around its position allows, updating its record and calling collision routines.
- `read_record_word_alignment_check`: Reads a word from the record and returns it, after a row-alignment and horizontal distance check whose outcome does not change the returned value.
- `compare_distance_to_limit`: Compares the absolute difference between two values at a fixed offset in a structure and returns if the difference is less than a certain value.
- `step_actor_horizontally_by_two`: Moves the actor two units left or right according to bit zero of its flags, counts down a record counter when it passes the edge limit, and stores the vertical position in a field when a global flag is clear.
- `run_subroutine_in_state_18`: Saves the actor's state word, sets it to 18, calls two subroutines, then restores the saved state.
- `probe_offset_table_for_path`: On first entry marks the record active and tries up to four offset pairs from a table, placing the record at each candidate position and calling routines until one succeeds, then toggles a direction bit and clears the active flag.
- `probe_offset_table_for_path_2`: Performs the same offset-table probe as the preceding routine; it is a duplicate copy of the same code with identical behaviour.
- `check_flag_without_effect`: Tests a flag byte in the actor record and returns without changing anything in either outcome.
- `apply_direction_bits_to_velocity`: Reads direction bits from the input word and, depending on those bits, adds or subtracts the stored velocity step to the position, sets a movement code in the actor record and calls handlers.
- `advance_vertical_position_by_velocity`: Sets the action codes to 4 and 10, adds the stored velocity step to the vertical position, sets the movement flag, and clears it again when bit three of the adjusted position is set.
- `queue_text_draw_request`: Queues a text-drawing request with two-byte parameters, choosing between two text strings by comparing a score word with 40 and by a mode flag, with the interrupt-enable register written around the update.
- `queue_text_draw_at_offset`: Queues a text-drawing request two units below a stored vertical value, sets its mode to 1 or 0 depending on a flag, and sends it only when the output flag is clear.
- `queue_record_text_draw_request`: Queues a fixed label string for drawing two units below the record's vertical value, sending it only when the record's mode byte is clear.
- `format_value_into_text_buffer`: When the output flag and a second flag are clear, fills a text buffer with spaces, writes up to six copies of a fill character derived from a stored value, and queues the buffer for drawing four units below the stored position.
- `format_record_count_text`: When the output flag and a record flag are clear, fills a text buffer with spaces, repeats a fill character up to six times using a count from the record, and queues the buffer for drawing four units below the record's vertical position.
- `format_bit_flags_into_text`: Converts the four low bits of a flag byte into a four-character text buffer, using a space for each clear bit, then queues the buffer for drawing six units below the stored position when the output flag is clear.
- `format_record_bits_into_text`: Converts four bits from a record byte into a text buffer, with a space for each clear bit, and queues it for drawing six units below the record's vertical position.
- `queue_paused_label_text`: Queues a fixed label string for drawing at a constant vertical position in mode zero, with the output counter and the interrupt-enable register updated around the call.
- `update_global_and_counter`: Increments a global counter, modifies several values in a structure, and updates a global variable or resets it if the counter reaches zero.
- `queue_text_request_offset_two`: Queues a text request in mode zero at a position two units below the stored vertical value, updating the output counter around the call.
- `clear_record_counter_word`: Writes zero to a 16-bit word inside the data block that a pointer field of the actor references.
- `clear_two_record_flag_bytes`: Writes zero to two adjacent flag bytes inside the actor record.
- `arm_record_when_enabled`: When an enabling flag is set, computes a timing value from a global word and a record field, stores it in the record, and sets one of two record flags depending on whether the referenced data's first byte equals 4.
- `set_record_flag_twelve`: Sets the byte at offset 12 of the record to minus one and has no other effect.
- `decode_run_length_stream`: Returns the next symbol from a run-length stream when the first flag is set, appends the current symbol and increments its repeat count when the second flag is set, and otherwise returns the leading word of the referenced data.
- `do_nothing_2`: the routine is empty and performs no operations before returning.
- `lookup_indexed_table_entry`: Derives an index from two five-bit fields packed in the input, selects a table pointer from a global table, and returns the indexed entry, or the packed input when that pointer is zero.
- `call_when_self_linked`: Calls a subroutine only when the record's pointer at offset 12 points back to the record itself.
- `clone_record_and_shift_bitplane`: Copies the source record into a new record and links the two, then fills the new record's bitplane rows by shifting each 16-bit word of the source rows by a bit offset from the record and passing each byte through a lookup table.
- `shift_bitplane_rows_by_offset`: Shifts 16-bit words of the source rows by a bit offset taken from the record, passing each byte through a lookup table and writing the results into the destination rows backward from the last word.
- `read_joystick_direction_fire_bits`: Decodes the two joystick port data registers into direction bits and a fire-button bit, storing each port's bits in its own state word.
- `save_registers_and_toggle_flag`: Copies the processor data and address registers into fixed memory cells, calls two subroutines, sets, inverts or clears a flag byte depending on two conditions, and decrements a counter.
- `stub_trap_zero_gap`: Issues trap number zero and returns; it lies in an unreached region, so it is not game logic.
- `count_frame_after_call`: Calls a subroutine and then increments a 32-bit counter stored in the global record.
- `install_level_three_interrupt_handler`: Writes a routine's address into the level-three interrupt vector slot, sets the global interrupt control word, and stores a long word from the low vector table into the global record.
- `dispatch_audio_command_code`: Handles a pending command word: when a flag and a second condition hold it resets several state fields, and otherwise it dispatches the command code, clearing the four audio volume registers for one code, toggling a byte for another, and updating flags for the rest.
- `process_tile_grid_cells`: When two list pointers differ, walks a grid of 14 rows by 16 columns and calls a per-cell routine with each cell's index and an incrementing counter.
- `process_tile_grid_from_base`: Walks the same 14 by 16 grid starting from the pointer passed in the argument register, calling the per-cell routine for each cell with an incrementing counter.
- `run_four_steps_nesting_counter`: Increments a nesting counter, calls four subroutines in turn, decrements the counter, and restores the global interrupt control word when it reaches zero.
- `call_if_table_bit_set`: Selects a table entry from the low four bits of a counter word and calls a routine only when the bit named by that entry is set in a bitmask table.
- `run_steps_with_optional_pair`: Increments a nesting counter, always calls one routine and calls a second pair when a flag byte is set, then decrements the counter and restores the global control word at zero.
- `append_six_byte_command_record`: Writes a six-byte record made of the constants 1025 and 32, the argument and zero into the output list, then advances the list pointer by six.
- `run_four_subroutines`: Calls four subroutines in sequence with no other work of its own.
- `setup_blitter_registers`: Configures the blitter registers with provided mask, source, destination, and size values after waiting for the blitter to be free.
- `append_clipped_sprite_record`: Clips a rectangle against horizontal limits, computes two edge offsets, appends a sprite entry with the clipped position to the output list, and calls four subroutines.
- `program_blitter_registers`: Waits until the blitter is free, then writes the mask, source and destination registers and the blitter size from the supplied values.
- `flush_linked_entry_list`: Walks a linked list of entries, calling three subroutines for each entry, then empties the list by clearing its first link and pointing the list head at it.
- `emit_text_buffer_glyphs`: Clears a pending-blank flag bit, optionally blanks the first buffer cell, then emits the buffer's paired glyph codes, advancing the column counter by two with wrap-around of the column and row counters.
- `emit_text_glyph_pairs`: Walks a zero-terminated buffer of glyph pairs, emitting each glyph through a subroutine, and advances the text cursor after each pair.
- `advance_text_cursor`: Advances the text column by two and, after column seven, resets the column and moves the row forward, wrapping the row after 27.
- `blend_glyph_into_bitplanes`: Combines eight pairs of 16-bit glyph columns into two bitplanes, using a table indexed by two character codes and a running XOR-blend mask, and writes the results into the destination buffer.
- `emit_ascii_digit_by_division`: Derives one decimal digit by counting repeated subtractions of the divisor, converts it to an ASCII character, and stores it when the sign test passes.
- `emit_digit_blank_leading_zero`: Derives one decimal digit by repeated subtraction, writes a space instead of a leading zero when the suppress flag is set, and stores the character when the width counter is positive.
- `advance_eight_step_counter`: Advances a counter modulo eight while it is non-negative, and calls two subroutines each time it wraps to zero.
- `rotate_six_word_buffer`: Saves the first word, shifts the following words down by one place across a six-word buffer, and stores the saved word in the last place.
- `lookup_and_store_table_value`: Returns the input unchanged when a flag is set; otherwise, for an index below 14, looks up a long word from a table, passes it to a subroutine, and links a non-null result into the record, returning 100 for out-of-range indices.
- `clear_list_entry_record_field`: Zeroes a field in the record the entry points to, clears the entry's first two words, and copies its third word into a global.
- `claim_earliest_channel_slot`: Chooses between two stored timing slots for the requested index, picks the one with the smaller key, and when that key is not after the requested time, claims it by storing the time and value and clearing the linked record.
- `claim_earliest_slot_into_record`: Picks the earlier of two stored timing slots and, when its key is not after the requested time, claims it by copying two words into the caller's record and storing the new time and value.
- `claim_second_pair_slot`: Performs the same claim operation as the preceding routine, applied to a different pair of stored timing slots.
- `update_stored_word_when_changed`: Stores the argument word and runs a subroutine inside a nesting-counter bracket only when it differs from the previously stored word.
- `initialize_channel_state_block`: Sets the initial counters and a set of default parameter words for a channel block, including a table pointer and a starting mode, after calling three setup subroutines.
- `initialize_channel_state_block_2`: Performs the same default-parameter initialization as the preceding routine; it is a duplicate copy of the same code with identical behaviour.
- `initialize_channel_block_from_arguments`: Writes the default parameter words of a channel block at the pointer argument, using the two register arguments as its initial position values and linking the block to the supplied table.
- `advance_timer_step_reload`: Subtracts a step from a timer word and, while the result stays negative, calls a subroutine and reloads the timer by twelve.
- `run_five_subroutines`: Calls five subroutines in sequence with no other work of its own.
- `run_two_subroutines`: Calls two subroutines in sequence with no other work of its own.
- `step_command_stream_on_countdown`: Counts down a delay in a channel block and, when it expires, reads command bytes from a stream: a non-negative byte sets the note and duration fields and calls a subroutine, while a negative byte is dispatched through a jump table.
- `start_channel_note_envelope`: Clears a channel link when it points at the current entry, and when a note value is nonzero links the next stored entry, resets the envelope fields and derives the attack step from the stored durations.
- `update_channel_volume_and_period`: Advances an active channel each tick: it steps a modulation value that reverses direction, looks up a table entry from the combined pitch to set the period, scales the volume by a global level, and then takes the next data word or reuses the current one.
- `write_marker_record_or_stop`: Decrements a countdown; while it stays non-negative it writes a two-byte marker constant, the argument and a pointer into the output list and advances the list, otherwise it clears the active flag and unlinks the channel.
- `validate_marker_and_advance_record`: Checks the marker word at the output pointer; if it does not match the expected constant it deactivates the channel, otherwise it counts down the stored counter and advances the record pointer when that counter reaches zero.
- `advance_output_record_pointer`: Moves the output pointer and its counter forward by eight units with no other work.
- `deactivate_channel_and_unlink`: Clears the channel's active byte and, when it is linked to a waiting entry, calls a routine and clears that link.
- `set_flag_from_zero_condition`: Stores minus one in a flag byte when the preceding comparison did not set the zero condition, and zero otherwise.
- `store_argument_into_record_word`: Writes the argument word into a fixed word field of the record.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_and_restore_registers`: Saves a large set of registers or memory locations to a table, updates a counter, and restores them before potentially modifying a certain memory byte based on conditional checks.
