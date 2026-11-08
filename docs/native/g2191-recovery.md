# Game 2191 (faery tale adventure, the (1986)(microillusions)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02191). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902112 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 444 of 444 named (0 author symbols), 444 of 444 described (68 by rule, 90 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2408 of 2408 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14441 of 14473 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 123410 |
| code | 49338 |
| data | 682778 |
| padding | 4090 |
| unused | 42496 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 386 | 1 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf:fmain | 3 | 13938 | 354 |
| 9 | 1 | 51 | 2 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 88 | 86 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EF8) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 59 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 32-136 (bytes 16384-70144) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 149-159 (bytes 76288-81920) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.69)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 164-256 (bytes 83968-131584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.96)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 258-311 (bytes 132096-159744) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.47)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 314-340 (bytes 160768-174592) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 342-342 (bytes 175104-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 344-393 (bytes 176128-201728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.98)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 396-415 (bytes 202752-212992) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.67)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 419-422 (bytes 214528-216576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 424-445 (bytes 217088-228352) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 447-453 (bytes 228864-232448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.20)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 455-461 (bytes 232960-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.34)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 471-471 (bytes 241152-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 473-473 (bytes 242176-242688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 477-477 (bytes 244224-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 479-535 (bytes 245248-274432) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.97)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 538-671 (bytes 275456-344064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.89)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 673-673 (bytes 344576-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 677-695 (bytes 346624-356352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.90)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.11)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 714-718 (bytes 365568-368128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 720-756 (bytes 368640-387584) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.15)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 760-782 (bytes 389120-400896) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.68)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 784-791 (bytes 401408-405504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 800-868 (bytes 409600-444928) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.57)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 896-917 (bytes 458752-470016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.01)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 920-955 (bytes 471040-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 960-1171 (bytes 491520-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 1312-1370 (bytes 671744-701952) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.24)
- faery tale adventure, the (1986)(microillusions)[a2]/Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf: blocks 1376-1501 (bytes 704512-769024) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_display_and_game_state` (fmain): Clears a flag, calls a sequence of indirect library routines to set up display and font state, and initialises global pointers and counters. Kind: loader/protection routine. Pass path: iVar7 == 0 branch.

## Functions


### Disk-Validator

- `format_counter_text_and_init`: Writes two text characters showing a counter from a global state field, a space when the counter is below ten and otherwise a digit from an indirect call.

### Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf:fmain

- `clear_long_table_then_setup`: Zeroes a block of longwords at an offset computed from a global, stores a pointer and a value near its start, then calls a further routine.
- `open_system_libraries`: Opens the libraries named in its text references through indirect calls, stores the returned handles in globals, and returns a failure code when an open fails.
- `release_flagged_buffers`: Checks bits in global flag bytes and, for each set bit, passes a stored pointer and a fixed size constant to an indirect routine.
- `build_six_entry_record_index`: Calls an indirect routine with fixed constants, then walks a length-prefixed record list from a global pointer and stores six pointer and length pairs in global arrays.
- `try_open_adjacent_door`: Probes the given grid cell and its neighbours through an indirect test, converts the chosen cell to tile units and calls an indirect routine with it.
- `setup_display_and_game_state`: Clears a flag, calls a sequence of indirect library routines to set up display and font state, and initialises global pointers and counters.
- `update_position_and_refresh`: Updates the current and previous x and y globals from the parameters, optionally computes a tile index from the coordinates, then calls further routines and loops on an indirect call until it returns zero.
- `find_range_entry_state`: Selects one of two range tables by a global counter, scans it for the entry whose byte range contains a global value, and derives a state byte from that entry's type byte, invoking callbacks when the state changes.
- `sync_frame_index_from_table`: Reads a byte from a global structure through an indirect call and adds it to an offset byte.
- `find_valid_spot_near_origin`: Tries up to fifteen offsets around a global origin, each checked by an indirect test, and falls back to fixed coordinates when no offset passes.
- `reset_slot_state`: Computes a slot index through an indirect call and, when that slot's countdown is not positive and its kind is not fourteen or fifteen, clears the countdown, sets state bytes and calls an indirect routine.
- `set_mode_and_reset_flags`: Stores a mode byte of six when the parameter is ten and five otherwise.
- `reset_game_state_globals`: Writes a fixed set of starting constants and cleared flags into globals.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d7.
- `compute_display_offsets`: Derives several screen coordinate globals from its parameter and an indirect result, such as 160 minus the parameter and 95 minus the other value.
- `select_animation_frame`: Chooses an index from several global values and conditionals, then calls an indirect routine with no arguments or with four pointers from a table, depending on a flag and the parameter.
- `update_position_and_state`: Converts two position words into a small cell index, updates a global state word and calls a routine when that index changes.
- `swap_buffer_pointers`: Swaps two stored pointers, copies the first long and a word from the swapped structure into globals, runs three indirect routines on the working buffer and then calls one more routine.
- `open_input_device`: Clears two flag bytes, creates a message port and I/O request, and opens the input.device named in its strings.
- `configure_io_request_then_cleanup`: Sets a request type and a data pointer on an I/O request, runs three indirect routines on it, then calls the cleanup routine on the stored message port.
- `collect_flagged_slots_and_fill`: Collects up to twelve entries from a byte table whose flag bit is set into an index array, then fills the remaining slots by alternating two constant codes passed to indirect routines.
- `render_slot_text_line`: Picks a value from a table selected by a mode global, with fixed overrides for two modes and a floor of four, and passes it with a parameter to indirect routines.
- `setup_mode_tables`: Switches on a mode word; for mode zero with parameter five it builds a five-entry pointer table and calls indirect routines with fixed sizes, while other cases are handled by further calls.
- `poll_coordinate_then_dispatch`: Polls an indirect routine up to twenty-five times at the global coordinates until it returns five.
- `signal_coordinate_then_dispatch`: Calls an indirect routine with the first argument one and the global coordinates, then calls the shared mode dispatch routine with mode five.
- `set_mode_and_dispatch`: When bit zero of a global flag byte is clear, stores the given mode word, clears a companion byte and calls the shared mode dispatch routine.
- `load_text_metric_tables`: Fills two seven-entry byte tables from an indirect byte reader at offsets nine apart, reads a six-byte run and notes whether it contains a newline.
- `loop_until_counter_reaches_ten`: Repeatedly calls an indirect routine while a global counter is below ten.
- `update_cached_slot_parameters`: For the current record index below ten, compares three stored fields with cached globals.
- `conditionally_call_indirect`: If a flag byte in a global variable is set, the routine calls an indirect function pointer taken from a global variable with three arguments built from two tables indexed by param_1 and param_2.
- `init_buffers_and_select_entry`: Runs three indirect calls on consecutive global buffers with the same constant, copies a table entry selected by a global index into a global pointer, then runs one more call on another buffer.
- `store_record_field_value`: Stores the value passed to it into a fixed field of the record addressed by the caller, with no other effect.
- `process_event_record_type`: Handles one event record by its type byte.
- `queue_region_code_for_point`: Maps a point inside a fixed screen region to a code, using a vertical offset divided by nine, and appends it to a circular byte queue when the stored flag changes.
- `queue_region_code_and_advance`: Maps a point in a screen region to a code, appends it to a 128-entry circular queue, and steps the stored coordinates by offsets from a second structure until the region test ends.
- `dequeue_queued_code`: Returns the next byte from a 128-entry circular queue when it is not empty, advancing the read index modulo 128, and returns zero otherwise.
- `advance_pseudo_random_state`: Advances a 32-bit state held in a global by multiplying its low sixteen bits by a constant and adding one.
- `mask_next_random_value`: Draws the next pseudo-random value and returns it ANDed with the caller's mask.
- `fetch_random_bit`: Draws the next pseudo-random value and returns its lowest bit.
- `fetch_random_two_bits`: Draws the next pseudo-random value and returns its two lowest bits.
- `mask_random_three_bits`: Draws the next pseudo-random value and returns its lowest three bits.
- `mask_random_six_bits`: Draws the next pseudo-random value and returns its lowest six bits.
- `fetch_random_byte`: Draws the next pseudo-random value and returns its low byte.
- `fetch_random_modulo_limit`: Draws the next pseudo-random value and returns its remainder modulo the low sixteen bits of the parameter, paired with the parameter.
- `run_format_then_library_call`: Runs an indirect routine, then makes an indirect library call through a global library base, and returns the data register values left by the calls.
- `format_number_right_aligned`: Writes the decimal digits of a 16-bit value into a fixed buffer from the right, then fills the remaining positions on the left with spaces.
- `format_number_into_buffer`: Writes the decimal digits of a value into a buffer at a given index, working leftwards, and fills the remaining positions with spaces once the value reaches zero.
- `fill_buffer_with_spaces`: Writes space characters into a buffer from the given index down to zero.
- `run_nested_library_loop`: Runs three nested loops of sixteen, fifteen and five passes, calling indirect library routines on each pass and making extra calls when the outer counter exceeds nine.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_60_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `process_string_segments`: Walks a zero-terminated string, invoking a callback when it meets a byte with the high bit set.
- `lookup_map_cell_with_mask`: Builds a bit mask from flag bits in two words, converts the coordinates to map cells clamped to a range using global offsets, and returns the byte from a table at the computed index.
- `lookup_map_cell_from_registers`: Performs the same map cell lookup as its sibling, but takes its coordinates and flags from the data registers and returns the byte from the table.
- `call_then_decrement_counter`: Calls an indirect routine through a global pointer, then subtracts four from a 32-bit global counter.
- `call_then_adjust_counter`: Calls an indirect routine through a global pointer, then subtracts a global value plus four from a 32-bit global counter.
- `run_repeated_routine_sequence`: Calls one routine with a value from a global, then makes many further calls to another routine with no arguments, and returns the data registers.
- `call_routine_six_times`: Calls one routine six times in succession with no arguments.
- `copy_words_to_five_buffers`: Copies 16-bit words from a source block into five destination buffers, writing two words per row at offsets that differ by forty, over a loop of sixteen rows.
- `nop_padding_sequence`: This routine is empty and consists of instructions that perform no operation.
- `call_routine_with_global_arg`: Calls one routine with a value loaded from a global, then returns the data registers unchanged.
- `call_routine_repeatedly_after_global`: Calls one routine once with a value loaded from a global, then calls it repeatedly with no arguments.
- `call_routine_and_return`: Calls one routine and returns without doing any other work.
- `compute_clamped_tile_indices`: Derives two tile coordinates from its parameters offset by global values, wraps them into a range of 128, clamps them to 46 and 23, stores the combined indices in a global, then calls a routine in a nested loop.
- `decode_bitplane_tile_words`: Reads five table offsets from a global structure, then for each source byte extracts bits and packs them into five separate word tables, one bit per table per row.
- `copy_tile_block_words`: Copies eight rows of four words from a source tile table into a destination table, with destination offsets computed from the parameters and a global base.
- `compute_tile_index`: Adjusts two coordinates by global offsets with wrap-around, looks up a table byte at the adjusted position, and combines it with the coordinates' low bits and a global base offset.
- `fill_tile_index_buffer`: Loops over a range of coordinates, computing a clamped index for each from global offsets, and writes table values into a buffer of words.
- `decompress_run_length_bytes`: Decodes a run-length stream into a destination buffer: a negative count repeats the following byte, while a positive count copies that many literal bytes.
- `add_table_scaled_offset`: When the second parameter is below eight, adds half of the third parameter times a table entry to the first parameter, masking the result to fifteen bits.
- `add_scaled_offset_keep_sign`: Adds a table-scaled offset like its sibling, but keeps the sign bit of the first parameter and masks the sum to fifteen bits.
- `copy_bit_fourteen_to_fifteen`: Sets bit fifteen of the parameter when bit fourteen is set and clears it otherwise.
- `clear_bit_fifteen`: Returns the data register word with bit fifteen cleared.
- `step_position_toward_target`: Moves the stored x and y position globals one step toward a target derived from the parameters.
- `step_position_from_registers`: Performs the same position step as its sibling, reading the target from data registers and snapping the stored x or y coordinate when it is far away.
- `nudge_x_and_step_y`: Nudges the stored x position by one toward its target in either direction when the x delta exceeds twenty, then steps or snaps the stored y position.
- `increment_x_then_step_y`: Increments the stored x position when the x delta exceeds twenty, then steps or snaps the stored y position toward its target.
- `step_y_toward_target`: Steps the stored y position by one toward its target when the delta is large, or snaps it to a fixed offset when the delta is very large.
- `nudge_y_when_delta_large`: Moves the stored y position by one when the delta from its target exceeds ten.
- `advance_y_if_above_ten`: Increments the stored y position when the input word exceeds ten.
- `handle_error_message`: The routine checks a parameter and, if it is not equal to two, calls three other routines with constant arguments and a global variable pointer.
- `classify_param_by_range`: Returns a table byte for the parameter, but for values above ten it substitutes a class from three to ten chosen by range thresholds.
- `read_joystick_direction`: Decodes joystick movement from the port's data bits into a direction code taken from a table, and holds a counter that clamps at nine while the stick is centred.
- `read_joystick_and_dispatch`: Performs the same joystick decoding as its sibling, then passes the result to two routine calls.
- `decode_joystick_and_notify`: Decodes the joystick port's movement bits into a direction code as its siblings do.
- `clamp_hold_counter_and_notify`: Subtracts twenty from a hold counter and resets it when the value falls outside zero to nine.
- `reset_and_notify_idle_code`: Clears the hold counter and, when the stored code is not nine, stores nine and calls a routine with it.
- `check_routine_result_twice`: Calls an indirect routine and returns its result, unless that result is below ten and not one.
- `combine_five_planes_inverted`: For each row, writes the inverted AND of five source word planes into a destination plane, looping over the rows and columns given by the parameters.
- `setup_blitter_copy_passes`: Calls library routines, then writes blitter registers for four passes in a loop.
- `set_blitter_regs`: Writes a control value and two parameters into blitter registers, clears several control and modulo registers, and returns zero.
- `configure_blitter_masked_passes`: Loops over four entries, writing blitter control values, first and last word masks derived from a global, and pointers from a table.
- `configure_single_blitter_pass`: Sets the blitter's pointers, control word and masks from global values for one pass.
- `setup_blitter_row_passes`: Writes four blitter passes in a loop, each with a fixed control value and full masks, computing pointer offsets from a parameter and a table.
- `setup_blitter_swapped_passes`: Like its sibling, writes four blitter passes with the same control value, but with the source and destination pointer computations swapped.
- `call_indirect_routine_once`: Calls one indirect routine through a global vector and returns.
- `call_indirect_vector_once`: Calls one indirect routine through a different global vector and returns.
- `compute_relative_direction_flags`: Computes signed offsets between a stored position entry and the given coordinates, unless the kind is six, and derives sign and axis flags from them.
- `compute_direction_and_lookup`: Computes the same signed offsets and direction flags as its sibling, then looks up a value in a table with them and returns the result.
- `select_direction_from_offsets`: Computes direction flags from offsets in registers and looks up a direction code in a table.
- `store_direction_code_in_record`: Computes direction flags from offsets, looks up a direction code, and stores it with a sign adjustment from a routine result into a record field.
- `set_record_direction_byte`: Adds a signed direction offset to a stored byte and writes the masked result into a record field, setting another field to twelve unless the kind is five.
- `store_masked_direction_difference`: Stores the difference of two byte values, masked to three bits, into a record field.
- `step_actor_coordinate_by_param`: For an entry in a table of up to four actors, checks its state byte.
- `step_actor_and_dispatch_kind`: Subtracts a record speed value from an entry's coordinate, clearing it when it becomes negative.
- `step_actor_and_spawn`: Subtracts a record speed value from an entry's coordinate, clearing it when it becomes negative.
- `dispatch_actor_kind_and_check`: Calls a routine with a kind-specific code and offset, then checks an entry's state byte.
- `dispatch_actor_kind_default`: Calls a routine with a code and offset chosen by the kind value, then checks an entry's state byte and calls a further routine when conditions match.
- `dispatch_actor_fixed_kind`: Calls a routine with a fixed code and offset, then performs the same state check and conditional call as its siblings.
- `count_actor_kinds_and_report`: Counts table entries of kind two that match two subtypes, starting from index three, then makes conditional routine calls with a global and the counts.
- `count_actor_kinds_in_registers`: Counts the same kind-two entries as its sibling, using register counters, then makes the same conditional routine calls.
- `count_actors_inner_loop`: Counts matching entries in a loop, then makes the conditional routine calls that its siblings make at the end.
- `count_actor_kinds_nested_loop`: Counts entries of one kind with an inner loop over the remaining entries, then makes the conditional routine calls at the end.
- `report_battle_counts`: Makes the conditional routine calls for the counted entries, then calls a routine when a flag byte is set.
- `report_battle_outcome_text`: Makes the counted foe-related routine calls, then calls a routine when a flag byte is set.
- `scan_entries_for_proximity`: Tests the given entry with a routine call, then scans the other active entries and returns a nonzero code when one lies within a proximity check, or zero if none does.
- `find_nearby_entry_code`: Scans the active entries other than the given one for one whose stored offsets lie within eleven horizontally and nine vertically.
- `find_nearby_entry_from_index`: Scans entries starting from a register index for one within eleven horizontally and nine vertically of the reference.
- `find_adjacent_entry_code`: Scans the active entries other than the reference, returning sixteen for the first one within ten horizontally and eight vertically, and zero when none match.
- `find_nearest_entry_index`: Loops over entries from index one, skipping those of certain types, and records in a global the index with the smallest distance value.
- `find_minimum_distance_entry`: Loops over entries, skipping those of certain types, and updates a record field with the smallest distance value found.
- `measure_entry_distance`: Measures the absolute horizontal and vertical distances between two entries.
- `call_two_indirect_routines_2`: If a comparison between two values holds true, the routine calls two different routines through indirect pointers stored in a global table.
- `call_two_indirect_routines`: Calls two routines through a global vector in succession and returns.
- `try_move_entry_to_point`: Transforms an entry's stored coordinates with two indirect calls.
- `store_entry_coordinates`: Stores the two data register values as the coordinates of an entry chosen by an index in the caller's record, and returns one.
- `draw_frame_from_table`: Loads four bytes from a table indexed by the parameter, then calls an indirect routine with fixed dimensions and an offset.
- `fill_weighted_tile_table`: Chooses a global value from the current map identifier and a flag, clamps the second parameter to a range, then runs thirty-two iterations that fill a table from indirect routine results.
- `randomize_word_table_rows`: Repeats thirty-two times: fills a table of thirty-one words with random values up to 4095 and passes it to an indirect routine along with a fixed argument.
- `process_queued_command`: Dequeues the next code from a 32-entry circular queue when one is pending, and dispatches it through a switch of routine calls.
- `dispatch_next_queued_command`: Dequeues the next code from the same 32-entry circular queue and runs the same switch of routine calls, without checking for an empty queue.
- `dispatch_command_code`: Dispatches on a code in a data register, calling routines with values from display-related globals for each matching case.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `enqueue_command_byte`: Appends a byte to a 32-entry circular queue when the queue is not full, moving the write index on with wrap-around.
- `print_string_with_default_layout`: Measures a zero-terminated string, makes two layout calls with fixed positions and sizes, then calls a routine with the string and its length.
- `setup_default_layout`: Makes two layout calls with fixed positions and sizes, then calls a routine to finish the layout.
- `print_counted_string`: Measures a zero-terminated string and calls a routine with the string and its length.
- `wrap_text_into_line_buffer`: Copies a zero-terminated text into a 37-character buffer word by word, expanding percent signs from a pointer table and stopping at the end of text or a carriage return.
- `copy_text_line_with_expansion`: Copies characters from the text pointer into a line buffer, recording word breaks at spaces and ending the line on zero or a carriage return.
- `append_expanded_text_char`: Appends one text character to the output buffer.
- `continue_text_scan_loop`: Increments the scan count and reads the next character, ending the scan on a carriage return or once the count reaches its limit.
- `store_char_and_continue_scan`: Stores the current character in the output buffer, then continues the text scan loop as its siblings do.
- `finish_text_line_and_scan`: Terminates the current line buffer with zero and, while the count is within its limit, calls a routine and continues scanning the input text.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `wrap_text_from_table_entry`: Skips a number of zero-terminated strings from a text base, then wraps the following text into the line buffer.
- `wrap_text_from_second_table`: Skips a number of zero-terminated strings in a second text block, then wraps the following text into the line buffer.
- `wrap_text_after_skipped_strings`: Skips a count of zero-terminated strings from a given pointer, then wraps the following text into the line buffer.
- `print_search_result_text`: Makes four routine calls with arguments from a text table indexed by a global and the parameter, two of them with a global address.
- `print_three_text_parts`: Makes three routine calls with arguments from a text table, a global and the parameter.
- `print_selected_text_entry`: Calls one routine with the text table entry selected by a global index.
- `setup_display_and_clear_fields`: Points a global at a display structure, calls three indirect routines on it, sets a state word and calls further routines.
- `initialize_tile_entry_flag`: Calls a routine, points a global at a structure, and writes a state word with its top two bits set.
- `fill_tiles_descending`: Repeatedly calls a routine with a counter that starts at one hundred and falls in steps of five, using a fixed address, until the counter is negative.
- `fill_tiles_ascending`: Repeatedly calls a routine with a counter that starts at zero and rises in steps of five until it passes one hundred, calling an indirect routine with one after each pass.
- `clear_two_fields_then_call`: Clears two 16-bit fields in a structure reached through a global pointer, then calls a routine.
- `prepare_structure_pointers`: Copies a pointer between globals, runs a fixed sequence of routine calls with small index arguments, then calls routines for the table entries and two further routines.
- `populate_entry_from_table`: Reads several values from indirect calls, stores a shifted short and pointer copies into global arrays, and loads three bytes from a table indexed by the parameter into three 16-bit globals.
- `copy_entry_into_slot`: Points a global at a slot in an array of fifty-six-byte entries chosen by the parameter.
- `copy_entry_to_scratch`: Copies a fifty-six-byte block from a global source into a scratch area, clears a global long and writes a constant word, then calls a routine with the scratch address.
- `forward_to_routine`: Calls one routine and returns.
- `run_entry_setup_with_fields`: Calls an indirect routine with a fixed address, clears a 16-bit global, then calls a routine with two pointers and three shorts loaded from arrays indexed by the parameter.
- `update_display_unless_state_two`: Calls a routine unless the structure's state field is two and an indirect check returns zero, in which case it returns early.
- `load_songs_index`: Opens the file named 'songs' through an indirect call, reads up to twenty-eight length-prefixed records into a table of offsets, and closes the file.
- `setup_viewport_when_enabled`: When a global flag byte is clear, makes a timed call, sets up a viewport with fixed dimensions, makes two indirect calls with the parameters, and runs two more routines when a check returns zero.
- `draw_table_entries_as_strips`: Loops over twenty-two entries of two byte tables.
- `check_space_key_flag`: Calls an indirect input routine, sets a global flag when its result equals the space character, and returns that result.
- `copy_bytes_to_new_block`: When the first parameter exceeds a fixed threshold, allocates a block of the given size through an indirect call and copies the bytes into it.
- `compute_screen_rectangle_offsets`: Computes several screen-coordinate offsets from global positions and a parameter block, then calls library routines for the drawing steps.
- `init_mode_counter_and_calls`: Sets a counter to two, makes two routine calls with arguments from tables indexed by the current mode, and copies the counter into another global when it exceeds three.
- `store_entry_position_and_flag`: Copies the x coordinate of a table entry into a global, stores its y coordinate plus ten in another global, and stores the parameter byte with a flag byte of one.
- `set_entry_flag_byte`: Looks up an entry by a computed index, either from a table or from the global entry base.
- `store_byte_in_entry_slot`: Looks up an entry through a table indexed by a global, then writes the data byte into its slot.
- `store_data_byte_at_offset`: Stores the low byte of a data register at a fixed offset from the address register.
- `place_entries_at_random_spots`: Resets two state bytes and a position global.
- `pick_random_free_spot`: Repeatedly draws random x and y values, with bits chosen by a global index, until an indirect test passes.
- `collect_nearby_typed_entries`: Checks entries against the current position using a bounding box around it, and collects those of types three or four that lie inside the box into a list.
- `advance_entry_scan`: Advances the entry scan pointer and count.
- `match_entries_into_table`: Scans entries inside a bounding box, skips types zero and two, and for types three or four stores a new entry record while comparing positions with existing ones.
- `initialize_new_entry_record`: Fills a new entry record with a kind byte, a table-derived value and flags.
- `finish_new_entry_record`: Sets the entry's code to thirteen where needed, stores the position and offset values into the record, raises the maximum count when needed, and advances the scan pointer.
- `record_entry_state_from_kind`: Sets a new record's state fields from the source kind, copies a state byte to a global flag when it equals the letter f, and sets kind-dependent codes before advancing the scan.
- `copy_entry_position_and_advance`: Sets a record byte from the source kind, copies the source's two position words and two offset words into the record, then advances the scan pointer and count.
- `record_entry_with_state_one`: Sets a record byte to one, copies two position words and two offset words from the source into the record, then advances the scan pointer and count.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `read_line_with_history`: Reads a line of keyboard input through indirect calls, echoing characters and handling the backspace and return keys.
- `call_library_cleanup_sequence`: Makes library calls through two global library bases in a fixed sequence, repeating two of the calls on the same base.
- `store_and_dispatch_result`: Calls a routine with two parameters, stores the returned value in a global, calls another routine with it when it is nonzero, and returns the stored value.
- `store_result_and_dispatch_table`: Calls a routine with a global pointer and a fixed argument, stores the result, and when it is nonzero calls routines depending on a table lookup.
- `stub_bad_instruction_data`: Bad instruction data decoded from a gap in the executable, with no real behaviour.
- `wait_for_flag_then_clear`: Waits up to 300 iterations of a routine call with argument five for a global flag byte to become nonzero.
- `poll_flag_then_clear`: Repeats a routine call with argument five up to 300 times, checking a global flag.
- `poll_flag_without_clear_first`: Polls a global flag with a delayed routine call up to 300 times and clears the flag when it is set, returning one.
- `open_disk_file_by_letter`: Builds a file name ending with a letter derived from the parameter, chooses one of two identifiers by a flag, and makes an indirect call with them.
- `open_winpic_disk_file`: Repeats a loop that checks for a disk and opens the picture file whose name is built from a drive text and a slot letter, retrying until the open succeeds.
- `handle_disk_error_and_retry`: Displays error messages regarding disk insertion or aborting, then attempts to re-initialize display states and reload data blocks from a disk.
- `transfer_state_blocks_on_open`: Repeats the disk-open attempt.
- `transfer_state_blocks_after_open`: Opens the slot file named from the letter.
- `allocate_buffers_and_wait`: Allocates a memory block through an indirect library call and calls itself again if that allocation fails.
- `show_save_error_and_wait`: Calls a library routine and stores its result, and when that result is non-zero it displays the referenced save-failure message.
- `show_load_error_and_wait`: Displays the referenced load-failure message and then waits in a loop for a signal value.
- `prompt_insert_game_disk_loop`: Repeatedly displays the referenced request to insert the game disk and calls library routines until a non-zero signal value is returned.
- `initialize_display_state`: When a flag byte is zero it clears a byte and a word, calls several routines, sets a counter byte to 99 and calls routines with constants and global addresses.
- `call_library_and_check_result`: Calls a library routine through an indirect pointer, or a direct routine when a flag byte is non-zero, passing a stored value and two parameters.
- `call_library_store_word_result`: Calls an indirect library routine with a stored value and two arguments taken from the caller frame, then stores the returned word.
- `write_bounds_to_table`: Computes the address of a twelve-byte entry from the first argument and writes four words into it: the second argument minus 250, the third minus 200, the second plus 250 and the third plus 200.
- `update_display_state`: Calls several indirect display routines, increments a counter word, sets a few global bytes and words, and calls another routine with a constant argument.
- `run_scroll_window_animation`: Calls indirect display routines, then for each step of a countdown from twenty-five down to minus twenty-nine copies a window of words from a table into a buffer and waits between steps.
- `continue_scroll_window_animation`: Continues the countdown-driven scroll loop from a counter already in progress, writing a window of words from a table, setting two offset words and waiting between steps.
- `scroll_window_with_random_offsets`: Repeatedly reads two random values, shifts each left by eight bits and stores them as offset words.
- `return_conditional_size`: Returns the constant eight, or ten when a byte read from a table at the argument plus a global offset is non-zero.
- `select_and_dispatch_position_check`: Selects an offset from a global flag, then masks a state value.
- `dispatch_position_check_from_globals`: Masks a state value and, when it is zero or a global byte is greater than ninety-seven, calls another routine with coordinates derived from global positions while a counter is below eight, otherwise with fixed values of one hundred.
- `call_routine_with_fixed_coordinates`: Calls another routine with the constants one hundred, one hundred, one hundred and one, plus the address of a global pointer, then returns.
- `dispatch_table_entry_action`: Looks up a table entry from a value returned by an indirect call and stores an action code in it.
- `dispatch_action_from_registers`: Looks up a table entry from an indirect call result and stores the action code held in a register.
- `apply_action_code_state_updates`: Handles action codes other than one: code eight compares an offset and sets a timer byte, code four resets a timer, code two sets a flag byte to four when a register matches a global value, and code ten resets a timer.
- `dispatch_action_when_flagged`: Handles action codes two through ten when a second register flag is non-zero: code eight compares an offset and sets a timer byte, codes two, three, five and six call other routines, and code four stores a masked value.
- `forward_to_nested_routine`: Contains a single call to another routine with no arguments and then returns.
- `dispatch_flagged_actions_from_four`: Branches on action codes four through ten, and performs its work only when a flag register is non-zero for codes other than four.
- `dispatch_flagged_actions_from_three`: Handles action codes three through ten when a flag register is non-zero, with code four storing a masked value and resetting a timer byte to twelve.
- `dispatch_flagged_actions_from_five`: Handles action codes five, two, six and ten, performing work only when a flag register is non-zero.
- `dispatch_flagged_actions_from_two`: Handles action codes two, six and ten.
- `dispatch_flagged_six_or_ten`: Handles action code six by calling routines with coordinates when a flag register is non-zero, and action code ten by calling another routine under the same condition before resetting a timer byte to twelve.
- `execute_recursive_action`: If a register is non-zero, it calls a function via a pointer from a table and then calls itself recursively.
- `handle_action_ten_timer`: When a register equals ten it calls another routine if a second register is non-zero, then stores the byte twelve at offset sixteen of a structure.
- `decrement_timer_and_trigger`: Subtracts the argument from a global word counter.
- `pass_own_address_onward`: Calls another routine, passing that routine's own address as the argument, then returns.
- `adjust_entry_and_store_positions`: Decrements a word in a table entry selected by an index, then calls three indirect routines using a global value and a stored position.
- `update_two_word_state`: Calls three indirect routines, using a stored value and the result of the first call, then stores the two returned words into global variables.
- `process_linked_record_list`: Computes a base index from a word and the arguments, then calls an indirect routine with a constant to obtain a pointer and stores it.
- `advance_counter_and_dispatch`: Clears a flag byte in the input structure, adds a word from that structure to a 32-bit counter, and when the flag was zero calls four other routines.
- `interpret_byte_sequence_stream`: Reads two-byte opcodes from a stream: opcode 129 loads a word from a table indexed by the low nibble, opcode 144 stores the second byte in a field, and opcode 255 with a zero operand ends the stream and sets a flag.
- `decrement_counter_and_update_state`: Writes the constant 256 to two word offsets of the first structure, then decrements a byte counter in the second structure when it is non-zero.
- `initialize_audio_and_dma`: Stores two state bytes, then writes the audio channel one location, length, period and volume registers from the three parameters.
- `clear_audio_channel_flags`: Clears the lowest bit of a flag byte at an offset from a global base, then writes zero to the audio period register and two to the audio volume register.
- `store_four_parameters`: Stores the four input parameters to four distinct offsets relative to a global base and returns.
- `initialize_audio_and_channel_state`: Stores four parameters into globals, clears the four audio volume registers and several state fields, and copies four words from a table pointed to by a global into position fields.
- `reset_audio_volumes_and_flags`: Writes zero to four audio volume registers and writes fifteen to the DMA control register, which without its top bit disables the first four channels.
- `setup_interrupt_handlers`: Stores three parameters into global variables, sets up interrupt server structures with a constant priority and a flag, then calls two library routines to add an interrupt server and set an interrupt vector.
- `remove_interrupt_server_and_reset`: Calls another routine, removes an interrupt server through a library call, and writes the constant 256 to an interrupt enable register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_memory_and_init_globals`: Computes a derived address from a global index and stores it in a global.
- `stub_return_immediately`: A stub that does no work and returns straight away.
- `tokenize_text_into_argument_list`: Opens a library through a library call and stops early when that fails.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `copy_string_pad_zeros`: Copies bytes from a source string into a destination up to a count or until a zero byte is found, then fills any remaining bytes of the count with zeros.
- `append_string_bounded`: Scans to the end of the destination string, then appends bytes from the source up to a limit of 32766 bytes.
- `append_string_with_count`: Scans to the end of the destination string, then copies bytes from the source until a zero byte is copied or a counter from the third argument expires.
- `copy_string_to_buffer`: Copies bytes from a source string to a destination, including the zero terminator, and returns the original destination pointer.
- `multiply_32_bit_values`: Multiplies two 32-bit register values using 16-bit halves and returns the low 32 bits of the product, with the second operand kept in the low word of the result.
- `perform_signed_division`: Determines the sign of the result from the signs of two register inputs and calls another routine.
- `negate_result_if_negative`: Calls another routine, then negates the value it leaves in the second register when the first input is negative.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed from the original second register and the value the call leaves in that register.
- `call_self_recursively`: Calls itself with no other work and then returns, so the routine recurses without doing anything else.
- `perform_32bit_division`: Divides a 32-bit dividend by a 32-bit divisor held in registers.
- `compute_string_length`: Walks a zero-terminated byte string from the given pointer until the terminator and returns the number of bytes before it.
- `invoke_optional_callback_then_run`: Calls a global function pointer if it is non-zero, then calls another routine with no arguments and returns.
- `clear_slots_and_dispatch`: Calls a routine twenty times, then calls the global callback if one is set and dispatches stored pointers through other routines.
- `release_slot_entry_checked`: Looks up one of twenty table entries from an index argument.
- `call_library_vector_36`: Loads a library base from a global variable and calls the library function at offset minus 36 from that base, passing the arguments through.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_132`: Loads a library base from a global variable and calls the library function at offset minus 132 from that base, passing the arguments through.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_84`: Loads a library base from a global variable and calls the library function at offset minus 84 from that base, passing the arguments through.
- `call_library_vector_30`: Loads a library base from a global variable and calls the library function at offset minus 30 from that base, passing the arguments through.
- `call_library_vector_30_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Loads a library base from a global variable and calls the library function at offset minus 60 from that base, passing the arguments through.
- `call_library_vector_42`: Loads a library base from a global variable and calls the library function at offset minus 42 from that base, passing the arguments through.
- `call_library_vector_66`: Loads a library base from a global variable and calls the library function at offset minus 66 from that base, passing the arguments through.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_library_vector_90`: Loads a library base from a global variable and calls the library function at offset minus 90 from that base, passing the arguments through.
- `call_library_vector_48`: Loads a library base from a global variable and calls the library function at offset minus 48 from that base, passing the arguments through.
- `call_library_vector_48_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_468_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_414`: Loads a library base from a global variable and calls the library function at offset minus 414 from that base, passing the arguments through.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_indexed_record`: Asks a routine for a free index and returns zero when none is available.
- `initialize_structure_and_call`: Calls a routine to release the structure when a field is non-zero, then writes the constant 255 to a status byte and minus one to a long field.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `forward_call_with_constant`: Calls a function through a pointer taken from a global variable offset, passing the incoming argument and the constant forty-eight, then returns.
- `forward_call_with_argument`: Calls a function through a pointer taken from a global variable offset, passing the incoming argument unchanged, then returns.
- `allocate_typed_record`: Returns zero when the first argument is zero.
- `release_record_if_present`: When the record pointer is non-zero it sets a status byte to 255, writes minus one to two long fields and calls a routine with the record and a word from it, returning that result.
- `reset_record_status_fields`: Sets a status byte in the record to 255, writes minus one to two long fields, and then calls an indirect routine with no arguments.
- `call_library_vector_198`: Loads a library base from a global variable and calls the library function at offset minus 198 from that base, passing the arguments through.
- `call_library_function`: Performs an indirect call to a function located within an Amiga OS library.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_294`: Loads a library base from a global variable and calls the library function at offset minus 294 from that base, passing the arguments through.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested table structure and executes it.
- `call_library_vector_210`: Loads a library base from a global variable and calls the library function at offset minus 210 from that base, passing the arguments through.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory it points to, adds four to that stored value, clears the next long word, and stores the original pointer in the long word after that.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_552`: Loads a library base from a global variable and calls the library function at offset minus 552 from that base, passing the arguments through.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_258`: Library stub: loads a library base from a global variable and calls the library function at offset minus 258 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Loads a library base from another global variable and calls the library function at offset minus 30 from that base, passing the arguments through.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_462`: Loads a library base from a second global variable and calls the library function at offset minus 462 from that base, passing the arguments through.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Loads a library base from a second global variable and calls the library function at offset minus 414 from that base, passing the arguments through.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Loads a library base from a second global variable and calls the library function at offset minus 198 from that base, passing the arguments through.
- `call_library_vector_468`: Loads a library base from a second global variable and calls the library function at offset minus 468 from that base, passing the arguments through.
- `call_library_vector_360`: Loads a library base from a second global variable and calls the library function at offset minus 360 from that base, passing the arguments through.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216`: Loads a library base from a second global variable and calls the library function at offset minus 216 from that base, passing the arguments through.
- `call_library_vector_240`: Loads a library base from a second global variable and calls the library function at offset minus 240 from that base, passing the arguments through.
- `call_library_vector_210_2`: Loads a library base from a second global variable and calls the library function at offset minus 210 from that base, passing the arguments through.
- `call_library_vector_456`: Loads a library base from a second global variable and calls the library function at offset minus 456 from that base, passing the arguments through.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354`: Loads a library base from a second global variable and calls the library function at offset minus 354 from that base, passing the arguments through.
- `call_library_vector_66_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Loads a library base from a second global variable and calls the library function at offset minus 60 from that base, passing the arguments through.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_table_vector_36`: Loads a function pointer from a global table base and calls the entry at offset minus 36 from that pointer.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_table_vector_150`: Loads a function pointer from a global table base and calls the entry at offset minus 150 from that pointer.
- `call_table_vector_144`: Loads a function pointer from a global table base and calls the entry at offset minus 144 from that pointer.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `stub_bad_instruction_data_2`: Contains only bad instruction data in the decoded font file, so the decompiler truncates control flow at its start.

### Faery Tale Adventure, The (1986)(MicroIllusions)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_forever_stub`: A single empty loop that never exits, with no other instructions in this runtime segment block.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `stub_bad_instruction_data_3`: Decodes as bad instruction data, so the decompiler truncates control flow at its start.
- `trap_infinite_loop`: An infinite loop that never returns.
- `empty_routine`: This routine contains no executable instructions.
- `nop_padding_byte`: This routine consists of a single byte of data that does not form a valid instruction.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `execute_infinite_loop_25`: The routine performs an unconditional jump to itself, resulting in an infinite loop.
- `execute_infinite_loop`: The routine enters an unconditional infinite loop that prevents further execution of the current thread.
- `nop_padding_sequence_2`: This routine is empty and contains no executable instructions.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `nop_padding_segment`: This routine is empty and contains no valid instructions, serving as a padding gap within the runtime segment.
- `execute_infinite_loop_2`: The routine enters an unconditional infinite loop that prevents further execution.
- `null_routine`: The routine is empty and contains no executable instructions.
- `execute_infinite_loop_26`: The routine enters an infinite loop that performs no operations and never returns.
- `nop_padding_sequence_14`: This routine consists of a single non-operational instruction used as padding between executable segments.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `nop_padding_sequence_15`: This routine contains no valid instructions and serves as a gap or padding within the executable segment.
- `execute_infinite_loop_3`: The routine enters an unconditional loop that prevents further execution of subsequent instructions.
- `nop_padding_sequence_13`: This routine consists of a single instruction that acts as a data gap within the executable segment.
- `undefined_instruction_trap`: The routine contains invalid opcode data that prevents successful disassembly and execution flow analysis.
- `nop_padding_sequence_3`: The routine is empty and consists of non-executable data or padding within the runtime segment.
- `empty_routine_2`: The routine contains no valid instructions and performs no operations.
- `empty_routine_3`: This routine contains no valid instructions and performs no operations.
- `execute_infinite_loop_4`: This routine enters an infinite loop that prevents further execution of the program flow.
- `empty_routine_4`: The routine is empty and performs no operations.
- `nop_padding_sequence_4`: This routine is empty and consists of an infinite loop of no-operation instructions.
- `execute_invalid_instruction_6`: This routine contains data that does not form a valid instruction.
- `nop_padding_sequence_16`: The routine consists of a single instruction that serves as a data gap between other executable segments.
- `empty_routine_5`: This routine contains no executable instructions and performs no operations.
- `nop_padding_sequence_5`: This routine is empty and consists of a sequence of non-executable data or padding bytes.
- `execute_infinite_loop_5`: The routine enters an unconditional loop that prevents further execution of the current thread.
- `loop_indefinitely`: Performs an endless loop of execution.
- `empty_routine_6`: The routine contains no executable instructions.
- `execute_infinite_loop_6`: The routine performs an infinite loop that prevents further execution.
- `null_instruction_sequence`: The routine contains no valid executable instructions.
- `nop_placeholder`: This routine contains a single non-operational instruction that performs no action.
- `nop_padding_sequence_6`: This routine is empty and consists of a single non-operational instruction used for data alignment or padding within the executable segment.
- `null_operation`: This routine is empty and contains no valid executable instructions.
- `execute_invalid_instruction_3`: Consists of a single byte of data that does not form a valid instruction.
- `execute_invalid_instruction_4`: The routine contains bad instruction data that prevents disassembly.
- `execute_infinite_loop_7`: This routine performs an infinite loop that prevents further execution of subsequent instructions.
- `null_operation_routine`: This routine contains no valid instructions and performs no actions.
- `execute_infinite_loop_8`: The routine enters an unconditional loop that prevents further instruction execution.
- `nop_padding_sequence_7`: This routine is empty and consists of a single non-operational instruction used to align data or code boundaries.
- `execute_infinite_loop_9`: The routine enters an infinite loop that performs no operations.
- `nop_padding_sequence_8`: This routine is empty and consists of a single instruction that performs no operation, serving as a gap or padding in the executable segment.
- `null_routine_2`: This routine is empty and contains no valid instructions.
- `execute_infinite_loop_10`: This routine enters an infinite loop that prevents further execution.
- `empty_routine_7`: The routine contains no executable instructions and performs no operations.
- `nop_padding_sequence_9`: This routine is empty and consists of non-executable data used for alignment or padding within the runtime segment.
- `null_routine_3`: The routine is empty and contains no valid instructions.
- `enter_infinite_loop`: Enters a continuous loop that never terminates.
- `execute_infinite_loop_11`: The routine enters an endless loop that prevents further execution of subsequent instructions.
- `loop_indefinitely_2`: Contains an infinite loop that is never reached during normal execution.
- `empty_routine_8`: This routine contains no valid instructions.
- `execute_infinite_loop_12`: The routine performs an infinite loop that prevents further execution by repeatedly executing a no-operation instruction.
- `loop_indefinitely_3`: Enters a loop that repeats without end.
- `enter_infinite_loop_2`: Performs a continuous, non-terminating loop.
- `halt_execution`: This routine contains no valid instructions and terminates execution.
- `execute_infinite_loop_13`: This routine enters an infinite loop that prevents further execution of subsequent instructions.
- `nop_padding_sequence_10`: This routine is empty and consists of a single instruction that performs no operation, serving as a gap in the executable code.
- `execute_invalid_instruction_5`: Contains data that cannot be parsed as a valid instruction.
- `nop_padding_sequence_11`: This routine is empty and consists of non-executable data used to fill a gap in the executable segment.
- `empty_routine_9`: This routine is empty and performs no operations.
- `terminate_execution`: This routine contains no valid instructions and terminates the current execution flow.
- `execute_infinite_loop_14`: This routine enters an unconditional infinite loop that prevents further execution of the current thread.
- `return_empty_routine`: The routine is empty and contains no valid executable instructions.
- `execute_infinite_loop_15`: The routine enters an unconditional loop that repeats indefinitely without performing any operations.
- `execute_infinite_loop_16`: This routine enters an infinite loop that prevents further execution of subsequent code.
- `execute_infinite_loop_17`: This routine enters an unconditional infinite loop that prevents further execution of the program flow.
- `null_execution_routine`: The routine contains no valid executable instructions and performs no operations.
- `execute_invalid_instruction`: This routine consists of a single byte of bad instruction data.
- `nop_padding_sequence_12`: This routine is empty and consists of a single instruction that performs no operation.
- `execute_invalid_instruction_2`: The routine is empty due to encountering bad instruction data.
- `nop_padding_gap`: This routine is empty and consists of non-executable data or padding located within an unreached gap in the executable.
- `execute_infinite_loop_18`: This routine enters an infinite loop that prevents further execution of the calling code.
- `execute_infinite_loop_19`: The routine enters an unconditional infinite loop that prevents further execution flow.
- `execute_infinite_loop_20`: The routine enters an unconditional infinite loop that prevents further execution of the program flow.
- `execute_infinite_loop_21`: The routine enters an unconditional loop that prevents further execution of the program flow.
- `execute_infinite_loop_22`: This routine enters an unconditional infinite loop that prevents further execution of the calling code.
- `perform_infinite_loop`: The routine enters an infinite loop that prevents further execution.
- `nop_padding_byte_2`: This routine consists of a single byte of data that does not form a valid instruction.
- `execute_infinite_loop_23`: The routine enters an unconditional loop that prevents further execution of the calling code.
- `terminate_execution_2`: The routine enters an infinite loop that prevents further instruction execution.
- `execute_infinite_loop_24`: This routine enters an endless loop that prevents further execution of subsequent code.
