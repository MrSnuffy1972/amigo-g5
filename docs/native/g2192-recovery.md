# Game 2192 (faery tale adventure, the (1986)(microillusions)[a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02192). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902096 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 443 of 443 named (0 author symbols), 443 of 443 described (65 by rule, 92 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2408 of 2408 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14409 of 14441 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 123418 |
| code | 49216 |
| data | 682689 |
| padding | 3765 |
| unused | 43008 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 386 | 1 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf:FMAIN | 3 | 13906 | 353 |
| 9 | 1 | 51 | 2 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 88 | 86 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EF8) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 59 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 32-136 (bytes 16384-70144) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 149-159 (bytes 76288-81920) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.69)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 164-256 (bytes 83968-131584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.96)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 258-311 (bytes 132096-159744) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.47)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 314-340 (bytes 160768-174592) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 342-342 (bytes 175104-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 344-393 (bytes 176128-201728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.98)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 396-415 (bytes 202752-212992) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.67)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 419-422 (bytes 214528-216576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 424-445 (bytes 217088-228352) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 447-453 (bytes 228864-232448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.20)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 455-461 (bytes 232960-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.34)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 471-471 (bytes 241152-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 473-473 (bytes 242176-242688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 477-477 (bytes 244224-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 479-535 (bytes 245248-274432) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.97)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 538-671 (bytes 275456-344064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.89)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 673-673 (bytes 344576-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 677-695 (bytes 346624-356352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.90)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.11)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 714-718 (bytes 365568-368128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 720-756 (bytes 368640-387584) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.15)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 760-782 (bytes 389120-400896) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.68)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 784-791 (bytes 401408-405504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 800-868 (bytes 409600-444928) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.57)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 896-917 (bytes 458752-470016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.01)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 920-955 (bytes 471040-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 960-1171 (bytes 491520-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 1312-1370 (bytes 671744-701952) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.24)
- faery tale adventure, the (1986)(microillusions)[a3]/Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf: blocks 1376-1501 (bytes 704512-769024) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialise_game_screens` (FMAIN): Large setup routine that calls twenty-two other routines and works with the hiscreen and page0 display buffers and the 'No Arrows!' text. Kind: track/disk check. Pass path: iVar7 == 0 branch.

## Functions


### Disk-Validator

- `initialise_program_entry`: Program entry point that sets fields of the state block from its counters and copies values between them.

### Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf:FMAIN

- `clear_state_table_and_chain`: Zeroes a block of 947 words at a base computed from a global index and stores a pointer and a value into the state block.
- `open_system_libraries_and_font`: Opens the graphics and layers libraries and the trackdisk device, then loads the Amber 9 font from FONTS. It returns a failure code when one of the opens fails.
- `release_flagged_memory_blocks`: Tests bits in two flag bytes and makes one library call per set bit, passing a stored pointer and a fixed size.
- `build_six_entry_offset_table`: Makes library calls to set up a 920-byte structure and clears a 16-bit field.
- `try_open_door_at_position`: Tests the map tile at the given position and at neighbours four units to either side, returning zero when none matches.
- `initialise_game_screens`: Large setup routine that calls twenty-two other routines and works with the hiscreen and page0 display buffers and the 'No Arrows!' text.
- `record_position_and_delta`: Takes an x value, a y value and a flag, stores the change from the previous position, and records the new position as the previous one.
- `find_table_range_index`: Masks a stored value to one byte and searches one of two byte tables for the entry at or below it, the table being chosen by a comparison with 8.
- `update_stored_index_on_change`: Reads a byte from a record and passes it to a library call, then compares a lookup-table value with a stored value.
- `compute_offset_coordinate_pair`: Calls two routines to obtain values, then computes two coordinates from a stored position plus those values and 150.
- `derive_record_coordinates`: Looks up a 22-byte record by index.
- `reset_record_if_idle`: Finds a record through a lookup call and, when its counter is not positive and its state is neither 14 nor 15, resets its counter and two fields and sets its state to 14.
- `set_mode_and_offset`: Sets a mode byte to six when the argument is ten and to five otherwise.
- `initialise_record_defaults`: Stores a set of default constants into the state fields, including timing values and a threshold of 15000.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d7.
- `set_scroll_positions_from_arg`: Calls two routines and derives several position fields from its argument and a returned value, storing a fixed offset minus the argument and a second offset minus the returned value.
- `classify_position_against_bounds`: Compares two stored 16-bit coordinates with fixed lower and upper bounds, returning 24 when a flag is zero.
- `update_position_flags_and_call`: When the state is below eight and a count is below ten, it masks two stored coordinates and calls another routine when they fall in a range.
- `swap_buffer_pointers`: Swaps two stored buffer pointers, copies two longwords from the new buffer into state fields, and calls two library routines with addresses inside the state block.
- `open_input_device`: Clears two flag bytes, allocates a request through a system call and then opens input.device.
- `configure_input_request_and_send`: Sets fields of an I/O request structure, including a value of 10 and a pointer, then makes four library calls on it.
- `collect_flagged_table_entries`: Walks a table of 18-byte records up to a stored count, and for each record with bit two set in its flag byte copies a value into an output buffer.
- `map_value_to_state_index`: Reads a byte from a buffer by index and remaps it according to a mode value: 14 for mode 8, 13 for mode 9, 4 for small values, and a table lookup for mode 6 and others.
- `dispatch_by_mode_state`: Switches on a stored mode number and handles each case separately; the first case calls the same routine again with argument 8 for that value.
- `retry_until_status_five`: Calls a routine on two stored values up to twenty-five times, stopping early when it returns five.
- `issue_two_step_position_call`: Passes two stored coordinates to a routine with the value 1, then calls another routine with 5.
- `update_global_value`: Checks a bit in a global variable and, if not set, stores a passed value into a global variable and calls itself recursively.
- `load_bytes_from_lookup_table`: Fills two seven-entry byte tables from a lookup routine, one read from offset nine onward and one from offset zero, then fills a further table from entries starting at 16.
- `loop_until_counter_reaches_ten`: Repeatedly calls one routine while a stored counter is below ten, then returns.
- `update_record_fields_by_index`: For a stored index below ten, compares a field of the selected 18-byte record with a stored value.
- `call_scaled_entry_if_enabled`: When bit zero of a flag byte is set, it calls one routine with a table entry and half of a stored value plus the argument.
- `initialise_three_buffers_and_pointer`: Makes three calls that each act on a 35-unit buffer, stores a pointer chosen from a table by a stored index, and then makes a call on a 60-unit buffer.
- `store_stack_argument_field`: Stores the first stack argument into offset four of an address register and returns.
- `advance_record_state_machine`: Loops over a record whose state byte it reads, and when the state is 1 it clears a field, writes a value into a table entry and calls two other routines.
- `compare_and_copy_record_flags`: Compares a flag word from two records and, when the bit pattern differs, copies a byte between them and calls another routine.
- `map_screen_point_to_letter`: Checks that a point falls inside fixed horizontal and vertical ranges, then converts its vertical offset into a letter code by dividing by nine and doubling.
- `pop_byte_from_ring_buffer`: Compares a read and write index pair for a 128-entry ring buffer and, when they differ, returns the next byte and advances the read index with wrap-around.
- `advance_pseudo_random_state`: Multiplies a stored 16-bit state by a constant, adds one and stores it back, then returns a value built from the new state.
- `and_random_with_argument`: Calls the random generator and returns its result ANDed with the argument.
- `random_flag_bit`: Returns the low bit of a call to the random generator.
- `random_value_below_four`: Returns the random generator output masked to its low two bits, giving a value from zero to three.
- `random_value_below_eight`: Returns the random generator output masked to its low three bits, giving a value from zero to seven.
- `random_value_up_to_63`: Returns the random generator output masked to its low six bits, giving a value from zero to 63.
- `random_byte_value`: Calls the random generator and returns its low byte.
- `random_value_modulo_parameter`: Calls the random generator and returns the low 16 bits of its value taken modulo the low 16 bits of the argument, alongside the argument.
- `call_library_after_random`: Calls the random generator, then calls a library routine through a pointer stored in a state field, and returns the input registers.
- `format_number_right_aligned`: Writes a number into a ten-character field from the right, one decimal digit per position, and fills the leading positions with spaces when the number is shorter than the field.
- `write_number_with_space_padding`: Writes a number as decimal digits from a starting position leftwards, stopping when the value reaches zero and filling the rest with spaces.
- `fill_remaining_with_spaces`: Writes space characters from a starting position down to zero, clearing the rest of a text field.
- `nested_callback_loop`: Runs three nested loops that call a set of routines through function pointers held relative to a stored base, with extra calls when the counter exceeds nine.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `step_through_text_with_library`: Loops over a text pointer, calling two library routines through function pointers and advancing the pointer by a stored amount each time.
- `position_bit_mask_from_stack`: Builds a single-bit byte mask from bit tests on two stack arguments, then compares the scaled coordinates against a stored offset and returns the mask.
- `position_bit_mask_from_registers`: Builds a single-bit byte mask from bit tests on two register arguments, then compares the scaled coordinates against a stored offset and returns the mask.
- `decrement_stack_counter_by_four`: Calls one library routine through a pointer stored in the state block, then decrements a counter field by four.
- `subtract_block_size_from_counter`: Calls the same library routine as its neighbour, then subtracts a stored size plus four from a counter field.
- `run_fixed_call_sequence`: Makes one call with a stored value and then a long run of identical no-argument calls to the same routine.
- `run_six_update_calls`: Makes six calls to the same routine in sequence and then returns.
- `copy_sixteen_word_rows`: Copies rows of sixteen 16-bit words from a source derived from its arguments into destination buffers, using register-offset addresses.
- `stub_empty_return`: A routine containing only a return, with no effect and no recovered callers.
- `forward_stored_value_once`: Calls one routine with a value taken from a stored state field and returns the unchanged input registers.
- `run_stored_value_call_sequence`: Starts with a call using a stored state field, then makes a long sequence of further no-argument calls to the same routine and returns the input registers.
- `forward_single_call`: Calls one routine and returns, with no data manipulation of its own.
- `update_block_offsets_from_state`: Reads two stored state fields and combines them with its two arguments by arithmetic, storing the results before calling one routine.
- `walk_word_records_updating_fields`: Walks a set of 16-bit word records reached through pointers stored in the state block, updating several of their fields and calling one other routine.
- `write_tile_words_at_row`: Computes two destination addresses from its arguments using a 64-byte row stride and two base pointers held in the state block, then writes words to them.
- `compute_wrapped_tile_index`: Converts two screen positions to tile coordinates by shifting them and subtracting stored origin values, wrapping each within its range, and returns a combined index.
- `scan_tile_table_at_position`: Uses the same stored origins as its neighbour to convert its arguments to tile offsets, then scans a word table and updates its entries.
- `expand_run_length_bytes`: When a flag byte is nonzero, reads a byte-coded sequence in which negative values are negated to give run lengths, expanding it into the output pointer.
- `scale_value_by_table_entry`: When the selector is below eight, multiplies the second argument by a table entry chosen by the selector, halves the product and adds it to the first argument, keeping the result to fifteen bits.
- `scale_value_keep_sign_bit`: Applies the same table scaling as its neighbour, but keeps the sign bit of the first argument unchanged while the rest of the result is masked to fifteen bits.
- `copy_bit14_to_sign_bit`: Sets bit fifteen of a 16-bit value when bit fourteen is set and clears it otherwise.
- `clear_sign_bit`: Returns its 16-bit argument with the top bit cleared.
- `adjust_offset_toward_target`: Computes the scaled difference between two arguments and two stored offsets, then steps or clamps a stored horizontal offset toward the target.
- `step_offset_x_by_one`: Adjusts a stored horizontal offset by one step when the difference exceeds a dead zone of twenty, and otherwise sets it from a register value.
- `step_offset_toward_difference`: Steps a stored horizontal offset by one when the difference exceeds twenty in either direction, and steps or sets a vertical offset from another register with a dead zone of 24.
- `step_offsets_toward_targets`: Steps a stored horizontal offset when the difference is above twenty, and steps a stored vertical offset by one inside a dead-zone band or sets it from a register value at the outer bound.
- `step_vertical_offset_toward_target`: Moves a stored vertical offset by one step when the difference lies outside a dead zone of ten, and sets it from a register value when the difference reaches 45 or more.
- `step_vertical_offset_by_one`: Moves a stored vertical offset by one toward the difference when it lies outside a dead zone of ten in either direction.
- `increment_vertical_offset_above_ten`: Adds one to a stored vertical offset when the difference is above ten, and otherwise leaves it unchanged.
- `report_error_if_not_two`: When the argument is not two, it calls three routines with constant coordinates, a global text pointer and the argument.
- `classify_argument_into_band`: Looks up a byte in a table by the argument, then grades the argument into bands with nested range checks and returns the band value with the register value.
- `read_joystick_direction_delta`: Reads the joystick port movement bits and derives horizontal and vertical deltas from them.
- `read_joystick_with_held_state`: Reads the joystick port movement bits as its neighbour does, then calls two routines to combine the movement with a stored state before returning.
- `read_joystick_movement_state`: Reads the joystick port data register bits to derive a movement value.
- `clamp_index_to_zero_nine`: Subtracts twenty from a stored value, clamps it to the range zero to nine, and calls one routine with the new value when it changes.
- `reset_index_to_nine`: Clears a stored index to zero, then sets it to nine and calls one routine with nine when the previous value was not nine.
- `probe_twice_with_range_checks`: Calls one routine and, when its short result is neither one nor at least ten, calls it again and returns zero if that second result is neither one nor at least eight.
- `combine_source_words_into_dest`: Runs a nested loop over a region sized by its arguments, reading source words through pointers and writing a combined result into the destination.
- `run_blitter_copy_for_planes`: Waits for the blitter with a library call, then loops over four plane pointers, programming the blitter source, destination, modulo and size registers for each one.
- `program_blitter_destination_and_size`: Writes fixed control values to the blitter registers, sets the destination pointer and the blitter size from its arguments, and returns zero.
- `blit_all_planes_with_wait`: Loops over four bitplane pointers, waiting for the blitter through a library call before programming its source, destination, modulo, control and size registers for each plane.
- `blit_single_plane_with_offset`: Checks the blitter busy bit and waits through a library call when needed, then programs the blitter pointer, modulo and size registers using a stored offset.
- `blit_planes_from_argument_offset`: Loops over four bitplane pointers, waiting on the blitter when it is busy, and programs its registers for each plane using an offset supplied by the argument.
- `blit_four_planes_by_table`: Loops over four bitplane pointers held in a table, waiting on the blitter when busy, and programs its registers for each plane from stored values.
- `execute_indirect_call`: Performs an indirect call to a function pointer located at an offset from a global address.
- `execute_indirect_routine`: loads a routine address from a memory location offset from a pointer and executes it.
- `lookup_record_flags_for_index`: Finds a 22-byte record by index and, unless the fourth argument is six, takes a value from that record before returning a combined result.
- `lookup_record_value_and_call`: Finds a 22-byte record by index, takes a value from it as its neighbour does, and makes one further call before returning.
- `compute_signed_step_direction`: Negates a value and selects a sign from the second argument to build a signed step, then calls one routine.
- `compute_step_with_clamp`: Computes a signed step from its arguments, clamping it against half of a second value, and calls routines to finish the movement.
- `set_random_direction_field`: Uses a random call to decide whether to negate a stored value, then stores the sum of two values masked to three bits into a record field.
- `store_direction_difference`: Stores the difference of two byte values masked to three bits into a record field, and sets a second field to 12 unless the state value is five.
- `try_activate_actor_slot`: Checks a counter and a table entry, using random values, to decide whether an actor slot is free, then calls two routines with its arguments.
- `check_actor_slot_then_update`: Looks up a slot type byte and returns early unless it matches neither of two reserved values, then calls two routines to update the slot.
- `subtract_record_value_from_counter`: Subtracts a field of a record from a stored counter at the index given by a lookup, then calls routines to check and update the result.
- `place_random_offset_by_case`: Chooses one of two base offsets depending on its state arguments, adds a random value from zero to 511, and calls a routine with the result.
- `place_random_offset_alternate`: Adds a random value from zero to 511 to a base of 800 or 400 depending on its state argument, then calls routines with the result.
- `place_random_offset_then_check`: Makes a call with a random offset added to 400, then checks several table entries and calls further routines depending on the values found.
- `count_type_two_entries`: Loops from index three up to a stored limit, counting table entries whose value is two.
- `count_matching_slot_pairs`: Loops over table entries, counting those of type two whose companion entry equals fifteen, and returns the count through its counter registers.
- `count_entries_of_type_fifteen`: Counts table entries marked fifteen and, for other entries, checks a companion entry against five before counting it.
- `count_entries_of_type_five`: Counts table entries marked five, then runs a loop over a further counter up to the limit, calling three routines along the way.
- `display_bravely_done_message`: Prints the 'Bravely done!' message through a call with the global text pointer when a stored value is low and a flag is set.
- `report_battle_outcome_text`: Chooses between the 'foes fled in retreat.' and 'foes were defeated in battle.' messages from flags and a stored counter, and calls routines to show the chosen text.
- `check_pair_slot_and_compute`: Returns early unless two table entries for the given index are both of type two, then calls a routine to compute a result and tests a stored counter before returning the status.
- `scan_slots_skipping_reserved`: Scans slot indices up to a stored limit, skipping a reserved index, and returns zero when it reaches the limit.
- `scan_slots_for_type_five`: Loops over slots, skipping reserved indices and slots whose type is five, and calls two routines to examine each remaining slot.
- `advance_to_next_eligible_slot`: Advances an index through the slots until it finds one that is neither the current slot nor index one, returning zero when it runs past the limit.
- `scan_slots_for_flagged_entry`: Walks slot indices from one up to a stored limit, clearing a counter and testing each slot type with a routine, and returns the second argument when the scan finishes.
- `count_slots_of_type_one`: Loops over slots, and for each slot of type one checks a field of the associated record and calls a routine to update the count.
- `compare_slot_coordinates`: Looks up two 16-bit values for an index and compares their difference against a threshold, calling a routine when the values are close or far apart.
- `call_pair_when_within_double`: Calls two routines when the second argument is no greater than twice the first, and otherwise returns without calling them.
- `call_pair_of_routines`: Calls two routines in sequence and returns.
- `test_record_against_point`: Looks up a record by index and passes its two coordinates with the other arguments to two routines, returning a boolean result from the comparison.
- `store_pair_into_record`: Writes two 16-bit values into the coordinate fields of the record selected by an index field, then returns one.
- `copy_entry_fields_from_table`: Reads four bytes for an index from a table of four-byte entries, copies them into a second table, and calls routines to apply them.
- `configure_mode_dependent_values`: Sets a series of state fields depending on a mode value, with a different set of constants for each mode, and calls routines with the arguments.
- `fill_table_with_random_values`: Fills word table entries with values from the random generator masked to twelve bits, then calls a routine to finish the table.
- `dequeue_event_from_ring`: Compares the head and tail indices of a 32-entry ring and, when they differ, reads the entry at the tail and advances it.
- `dispatch_queued_event`: Takes the next queued entry, advances the index with wrap-around at 32, and dispatches on its type through a switch.
- `dispatch_event_case_two`: Switches on a numeric argument and, for case two, makes several library calls with fixed values to update two fields.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `enqueue_event_if_not_full`: Advances a write index through a 32-entry ring and, unless the ring is full, stores the argument at the new position.
- `print_string_at_position`: Measures a zero-terminated string, then draws it through routine calls with fixed positional constants and the string length.
- `setup_text_window_layout`: Makes a series of routine calls with fixed positional constants, including 400 and 44, and library calls through graphics vectors.
- `print_string_with_length`: Measures a zero-terminated string and passes it with its length to one routine.
- `copy_text_into_scratch_buffer`: Copies characters from its argument into a fixed text buffer one at a time until the end marker, then stores the buffer position.
- `split_text_at_spaces`: Copies characters from an input text one at a time, marking a word boundary at each space.
- `expand_percent_placeholders`: Scans text for percent signs and, when one is found, substitutes a string from a pointer table indexed by a stored counter, copying the result into an output buffer.
- `copy_chars_until_counter_limit`: Increments a stored counter and copies characters from the source pointer while the counter is below 37, writing them to the output buffer.
- `append_char_to_output_buffer`: Appends a character to a text output buffer by advancing the stored write pointer, then checks for a line or length limit and calls routines.
- `terminate_output_string`: Writes a zero terminator at the current output position and advances the write pointer, then checks the length against 38 and calls routines to flush the text.
- `empty_routine`: The routine is empty and performs no operations before returning.
- `copy_message_text_by_index`: Walks a message table, skipping zero-terminated entries until the requested index is reached, then copies the chosen message into the output.
- `copy_message_from_second_table`: Uses the same string-table indexing as its neighbour, but over a second message table.
- `skip_strings_to_index`: Advances a text pointer past a given number of zero-terminated strings and returns the new position.
- `print_search_result_message`: Prints a result message built from a table entry, the 'containing' and 'found' text, and the argument, using a sequence of routine calls.
- `print_found_message`: Prints a short message built from a table entry, the 'found' text and the argument.
- `print_table_entry_text`: Looks up a text entry in a table by a stored index and passes it to one routine for display.
- `initialise_text_block_and_open`: Initialises a text block pointer and a value taken from a linked structure, then calls library routines to set the block up.
- `configure_block_and_flag`: Points a stored pointer at a block, stores a fixed word value, calls two routines, and stores a byte value of three.
- `loop_down_by_fives_drawing`: Loops from 100 down in steps of five, calling one routine with the step value and a buffer pointer, and calls another routine with one on each pass.
- `loop_up_by_fives_drawing`: Loops from zero up to 100 in steps of five, calling one routine with the step value and a buffer pointer, and calls another routine with one on each pass.
- `clear_fields_then_call`: Clears two 16-bit fields of a structure reached through a global pointer and then calls one routine.
- `run_indexed_call_sequence`: Stores a value from a state field and then makes a fixed sequence of calls with small index arguments, including one based on a count minus one.
- `update_table_from_randoms`: Reads random values, doubles one of them and stores it into a table entry at an index, then continues with further table updates and calls.
- `process_56_byte_record_entry`: Selects a 56-byte record by index and, when its state field is two, makes a library call through the graphics vector.
- `copy_fourteen_longs_to_table`: Copies a block of fourteen 32-bit values from a source pointer into a fixed table in the state block.
- `forward_to_routine_once`: Calls one routine and returns.
- `dispatch_entry_from_table`: Initialises a state block, clears one 16-bit field, then calls a routine with values from an 18-byte table entry selected by the argument.
- `call_unless_state_two_done`: When a record's state is two and a routine returns zero, it returns early.
- `load_song_index_entries`: Opens a file named from a global string, which is referenced as 'songs', and reads 4-byte entries from it in a loop.
- `open_screen_with_fixed_geometry`: When a flag is clear, makes a timing call and then a library call with fixed geometry values, including width 320 and height 200, to open a display.
- `loop_over_twenty_two_slots`: Loops 22 times, reading a value from a global structure on each pass and calling two routines with values derived from it.
- `test_space_key_pressed`: Calls one routine to read a key or input code, stores whether it equals 32, the space character, in a flag, and returns that flag.
- `allocate_or_copy_block`: When the pointer argument is above a fixed high boundary, allocates a new block through a library call and copies the old contents across.
- `initialise_game_display_state`: Large setup routine that sets many state values from a structure and calls five library entries plus two routines.
- `start_pointer_table_draw`: Sets a stored field to two, then calls routines with a pointer and a table entry chosen by a stored state index, passing a fixed value of 128.
- `copy_record_position_into_slot`: Copies the position of a 22-byte record into a slot, adding ten to its vertical coordinate, and stores a flag byte from the argument.
- `check_flag_then_store_state`: Looks up a flag byte for an index and, when its top bit is clear, stores the current state value and calls one routine with the second argument.
- `mark_slot_if_type_fifteen`: Looks up an entry for the current state index and, when its type byte is fifteen, updates a flag field in the same table.
- `store_byte_at_offset_five`: Writes a byte value into offset five of an address register.
- `fill_sprite_record_fields`: Stores three arguments into state fields and builds a small record in a local buffer, then calls one routine to finish the setup.
- `randomise_table_entries_with_flag`: Loops over entries writing values derived from a random number masked to fourteen bits, combined with a flag bit from a stored value, and calls two routines to finish each entry.
- `compute_sprite_bounds_from_position`: Computes two offsets from a stored position with a fixed margin of minus eight, then checks flags on linked entries and calls five routines to apply them.
- `scan_records_register_matches`: Walks a sequence of six-byte records, decrementing a remaining count and advancing a frame counter.
- `poll_records_into_slot_table`: Repeats the same record scan as its neighbours, with a loop entered through the record-advance step.
- `fill_new_slot_from_record`: Claims the next free 22-byte slot, fills its kind, flag and offset fields from the current record, and copies the record's coordinates into it.
- `store_record_and_advance_cursor`: Copies the current record's coordinates into the slot being built and then advances to the next record.
- `copy_record_into_new_slot`: Claims a fresh 22-byte slot for the current record, sets its kind to a fixed value depending on the record's type byte, and copies the record fields into it before continuing the scan.
- `classify_slot_kind_and_copy`: Scans records and, for each one inside the window, either copies it into an existing matching slot or claims a new slot whose kind is chosen from the record's type byte.
- `assign_slot_kind_one_and_copy`: Scans records within the window and claims a new 22-byte slot for each, setting its kind to one for ordinary records and to two or zero for the special type codes.
- `do_nothing_routine`: The routine is empty and performs no operations or data modifications.
- `quiz_user_on_random_words`: Repeatedly picks an entry from a pointer table through a library call and prints it, then reads a typed line with backspace and printable-character handling.
- `refresh_library_state_pair`: Calls library routines to refresh the state of two linked library bases, issuing three vector calls on each.
- `call_and_release_nonzero_handle`: Calls a routine with two arguments, stores its result in a global, and when that result is non-zero passes it to a second routine.
- `lock_df0_path_and_verify_type`: Requests a lock on the device path text 'df0:' through a library call, storing the handle.
- `return_empty_routine`: The routine contains no executable instructions and performs no operations before returning or terminating execution flow.
- `poll_timed_flag_then_clear`: Checks a status byte, then waits in five-unit steps through a timing library call, up to about 300 attempts.
- `poll_flag_with_delay_limit`: Waits in five-unit steps through a timing library call and checks a status byte on each pass, stopping after 300 passes.
- `delay_then_poll_flag_limit`: Waits one five-unit step through a timing library call before each check of a status byte, giving up after 300 steps.
- `load_drive_tables_with_retry`: Opens a file named from a drive-letter text and a fixed name, and on success reads a sequence of structured buffers sized from table counts.
- `load_tables_from_df0_winpic`: Opens the file named from the 'df0:' device text and the winpic name, then reads the same sequence of structured buffers used for other drives.
- `prompt_insert_writable_disk_loop`: Repeatedly tries to lock a disk, and when none can be found it shows the message asking for a writable disk in any drive.
- `load_tables_after_disk_wait`: Waits for a disk lock to be granted, retrying with a fallback prompt when the lock fails.
- `load_tables_single_pass_unlock`: Reads the structured tables once through the lock library call, without a retry loop at the start.
- `open_and_load_tables_path`: Opens a fixed file path through a library call and reads the structured tables sized from count fields in the global block.
- `report_save_error_and_release`: Shows the couldn't-save-game message when a status flag is set, then retries the lock call until it succeeds.
- `report_load_error_and_cleanup`: Retries the lock call on the device path until it succeeds, then runs the shared cleanup that clears status fields and restores the display state.
- `prompt_insert_game_disk_loop`: Shows the message asking for the game disk, then retries the lock call on each pass until the disk is found.
- `reset_status_fields_and_redraw`: When the status flag is clear, it zeroes several status fields in the global block and calls library routines to refresh the display state, setting one counter to a fixed value.
- `check_library_call_result`: Calls a library routine with two arguments through a vector and stores the returned value.
- `check_library_call_with_stack_args`: Calls a library routine using two arguments taken from the caller's stack frame and stores the signed result.
- `set_bounding_box_around_point`: Writes four words into a twelve-byte record at the given index: the point's coordinates offset by 250 left and right, and by 200 above and below.
- `reset_display_and_advance_counter`: Redraws the screen state through library calls, advances a shared counter, and sets several state words to fixed starting values.
- `run_winpic_palette_fade`: Loads the 'winpic' picture through a library call, then runs a loop of 56 steps.
- `continue_palette_fade_steps`: Resumes a stepped fill of a 32-byte buffer from a table offset by the step count, copying it to the display and pausing between steps.
- `run_fade_step_sequence`: Sets two starting words from a random-source library call, then runs the stepped fill of a 32-byte buffer from a table.
- `choose_code_by_flag_byte`: Returns eight, or ten when the table byte at the given index is non-zero.
- `draw_sprite_at_tracked_position`: Chooses a horizontal offset of zero or 200 from a flag word and, when a low-bit mask is clear or a timer is above its limit, draws at a position built from a tracked coordinate.
- `draw_tracked_sprite_no_offset`: Draws at a position built from a tracked coordinate, with no flag-based offset, when a low-bit mask is clear or a timer is above its limit.
- `draw_sprite_at_fixed_position`: Calls the drawing routine with the fixed coordinates of 100, 100 and 100 and the same record and returns.
- `dispatch_slot_action_code`: Stores a code in the slot's byte field, then dispatches on codes 1 to 10.
- `dispatch_slot_code_no_arguments`: Stores the current code in the slot's byte field and dispatches on it without taking arguments.
- `dispatch_codes_eight_four_two_ten`: Handles four slot codes: code 8 compares the slot's distance to a reference and may raise its timer byte, code 4 sets a timer, code 2 flags a match with the reference slot, and code 10 sets a timer.
- `dispatch_slot_code_gated_by_flag`: Dispatches on the slot code, with each branch's library call made only when a passed flag is non-zero.
- `forward_call_and_return`: Calls one other routine with no arguments and returns, passing control on without other work.
- `dispatch_slot_code_at_offset_sixteen`: Dispatches on the code passed in the slot, gating each library call on a flag.
- `dispatch_gated_codes_two_to_ten`: Dispatches on the slot code for codes 2, 3, 5, 6 and 10, making each library call only when a passed flag is non-zero.
- `dispatch_gated_codes_five_to_ten`: Handles slot codes 5, 2, 6 and 10 with the same flag-gated calls as its neighbours.
- `dispatch_gated_codes_two_six_ten`: Handles slot codes 2, 6 and 10, making each library call only when the passed flag is non-zero.
- `dispatch_gated_codes_six_ten`: Handles slot codes 6 and 10, making the library calls only when the passed flag is non-zero.
- `call_helper_if_flag_set`: Calls a helper routine only when the passed flag is non-zero, after a library lookup and a further helper call.
- `dispatch_code_ten_and_mark_slot`: When the slot code is ten and the passed flag is non-zero, it calls a helper routine.
- `decrement_timer_clamp_to_zero`: Subtracts the argument from a 16-bit counter.
- `call_shared_routine_and_return`: Calls one routine with the shared global argument and returns, passing control on without other work.
- `draw_strips_from_matched_resource`: Opens a resource through a library call and walks its linked entries to find one matching the identifier.
- `advance_position_and_step_four`: Clears the byte at offset one, adds the 16-bit step at offset two to the 32-bit position at offset sixteen, and when the first byte is zero calls a helper four times.
- `interpret_channel_bytecode_step`: Reads coded bytes from a per-channel stream: codes 129 and 144 load or store a stored value, and 255 either ends the stream or jumps back to a saved position.
- `decrement_counter_and_update_state`: Writes two constant words to offsets in the first structure, then decrements a byte counter from the second argument if it is non-zero.
- `start_audio_channel_one_playback`: Stores channel-one state bytes, then writes the sample pointer, length and period from its three arguments plus a volume of 64 to the channel-one registers.
- `silence_audio_channel_one`: Clears the low bit of a channel-one state byte, sets that channel's volume register to zero and its period register to two.
- `store_four_channel_values`: Stores its four 32-bit arguments into four state slots in the global block, one argument per slot, with no other processing.
- `initialise_four_audio_channels`: Stores two copies of each of four 32-bit arguments, zeroes the volume registers of all four audio channels and copies four words from a table into state.
- `mute_all_audio_channels`: Zeroes the volume registers of all four audio channels, clears the DMA enable bits for channels zero to three, and sets a status byte to a terminal value.
- `install_vblank_music_server`: Fills in an interrupt server structure and an audio-four entry, adds them through the system interrupt-server and interrupt-vector library calls, and sets a timing value of 150.
- `remove_vblank_music_server`: Calls a routine to stop the music handler, then removes the interrupt server through the system library.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_indexed_block_and_store`: Picks a block from an index held in a global, writes a DMA-style control word for it and clears 947 long words of that block.
- `stub_return_no_work`: Contains only a return and performs no work.
- `parse_text_fields_and_load_segment`: Opens the DOS library and checks a segment identifier from the text 'MANX'. When the identifier is present it either loads a named segment and records its pointers, or splits the text into quoted and unquoted string fields, builds a pointer table of them, and loads the result.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `copy_string_pad_zeros`: Copies bytes from the source to the destination until a count runs out or a null terminator is reached, then fills the rest of the count with zeros.
- `append_string_with_limit`: Moves to the end of the first string and copies the second string after it, stopping at its terminating zero or after about 32 thousand characters, then terminates the result.
- `append_counted_string_bytes`: Moves to the end of the destination string, then copies source bytes up to the count in the third argument.
- `copy_string_to_buffer`: Copies bytes from the source to the destination until a zero byte, including the terminator, and returns the original destination pointer.
- `multiply_32_bit_values`: Computes the low 32 bits of the product of two 32-bit values from their 16-bit halves.
- `negate_result_by_operand_signs`: Reads two signed values and works out whether their signs differ, then calls a routine and negates its result when they do.
- `negate_result_if_negative_divisor`: Calls one routine and negates its result when the first argument is negative.
- `call_and_return_concatenated`: Calls one routine and returns a 64-bit value built from the original second operand and the value the call left behind.
- `call_helper_then_return`: Calls one other routine with no arguments and returns.
- `divide_32bit_quotient_remainder`: Divides a 32-bit dividend by a divisor, using shift-and-subtract when the divisor exceeds 16 bits and 16-bit division otherwise.
- `compute_string_length`: Walks a zero-terminated byte string until its terminator and returns the number of bytes before it.
- `run_optional_hook_then_finish`: Calls a stored hook routine when its pointer is set, then calls a second routine.
- `release_loaded_resources`: Runs twenty library calls, then frees each stored buffer and handle that is set.
- `free_indexed_slot_resource`: Looks up a slot in a table of six-byte entries by index.
- `jump_library_vector_36`: Jumps through the library function at offset 36 below the library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `jump_library_vector_132`: Jumps through the library function at offset 132 below the library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `jump_library_vector_84`: Jumps through the library function at offset 84 below the library base kept in a global, returning directly to the caller.
- `jump_library_vector_30`: Jumps through the library function at offset 30 below the library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `jump_library_vector_60`: Jumps through the library function at offset 60 below the library base kept in a global, so that function returns directly to the caller.
- `jump_library_vector_42`: Jumps through the library function at offset 42 below the library base kept in a global, so that function returns directly to the caller.
- `jump_library_vector_66`: Jumps through the library function at offset 66 below the library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `jump_library_vector_90`: Jumps through the library function at offset 90 below the library base kept in a global, so that function returns directly to the caller.
- `jump_library_vector_48`: Jumps through the library function at offset 48 below the library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `jump_library_vector_414`: Jumps through the library function at offset 414 below a second library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `create_signal_message_port`: Allocates a signal bit, then allocates a cleared block of 34 bytes and fills in its type, the caller's first two fields, the signal number and the current task pointer.
- `delete_signal_message_port`: If the block holds a pointer it calls a helper first.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_with_constant_48`: Calls a library routine with the argument and the constant 48, and returns nothing to its caller.
- `pass_argument_to_library_call`: Forwards its single argument to a library function through the library vector, with no other work.
- `allocate_typed_block_with_size`: When the first argument is non-zero, allocates a block of the requested size through the library memory call and sets its type byte to five.
- `clear_block_and_call_library`: When the block pointer is non-zero, sets its type byte to 255, sets two fields to minus one and calls a library routine with the block and its stored size.
- `invalidate_block_fields_and_free`: Sets the type byte to 255 and two fields to minus one, then calls a library routine on the block.
- `jump_library_vector_198`: Jumps through the library function at offset 198 below a second library base kept in a global, so that function returns directly to the caller.
- `execute_library_call`: Performs an indirect call to a function pointer retrieved from a table or a global pointer.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `jump_library_vector_294`: Jumps through the library function at offset 294 below a second library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `jump_library_vector_132_2`: Calls the library function at offset 132 below the library base kept in a global, and returns nothing to its caller.
- `jump_library_vector_210`: Jumps through the library function at offset 210 below a second library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument in the block, adds four to that stored value, zeroes the next word and stores the original pointer in the word after it.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `jump_library_vector_552`: Jumps through the library function at offset 552 below a second library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_258`: Library stub: loads a library base from a global variable and calls the library function at offset minus 258 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `jump_library_vector_30_2`: Jumps through the library function at offset 30 below a library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `jump_library_vector_462`: Jumps through the library function at offset 462 below a library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `jump_library_vector_414_2`: Jumps through the library function at offset 414 below a library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `jump_library_vector_198_2`: Jumps through the library function at offset 198 below a library base kept in a global, so that function returns directly to the caller.
- `jump_library_vector_468`: Jumps through the library function at offset 468 below a library base kept in a global, so that function returns directly to the caller.
- `jump_library_vector_360`: Jumps through the library function at offset 360 below a library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `jump_library_vector_216`: Jumps through the library function at offset 216 below a library base kept in a global, so that function returns directly to the caller.
- `jump_library_vector_240`: Jumps through the library function at offset 240 below a library base kept in a global, so that function returns directly to the caller.
- `jump_library_vector_210_2`: Jumps through the library function at offset 210 below a library base kept in a global, so that function returns directly to the caller.
- `jump_library_vector_456`: Jumps through the library function at offset 456 below a library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `jump_library_vector_354`: Jumps through the library function at offset 354 below a library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `jump_library_vector_60_2`: Jumps through the library function at offset 60 below a library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `jump_library_vector_36_2`: Jumps through the library function at offset 36 below a library base kept in a global, so that function returns directly to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `jump_library_vector_150`: Jumps through the library function at offset 150 below a library base kept in a global, so that function returns directly to the caller.
- `jump_library_vector_144`: Jumps through the library function at offset 144 below a library base kept in a global, so that function returns directly to the caller.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_routine_2`: The routine contains no valid instructions and performs no operations.

### Faery Tale Adventure, The (1986)(MicroIllusions)[a3].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spin_forever_stub`: Contains only an empty infinite loop with no work and no exit.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `create_infinite_loop`: The routine executes an infinite loop that prevents further execution or traps the program flow.
- `empty_routine_3`: the routine contains no executable instructions.
- `execute_infinite_loop`: The routine implements an infinite loop that repeatedly executes a single empty cycle without any state-changing operations or exits.
- `nop_padding_sequence_2`: The routine is empty and contains no executable instructions.
- `execute_infinite_loop_15`: Enters a continuous loop that never terminates.
- `empty_routine_10`: This routine contains no executable instructions.
- `do_nothing`: the routine is empty and performs no operations.
- `nop_padding`: The routine contains a single non-instruction or invalid byte that acts as a gap in the executable code.
- `contain_invalid_instruction`: Consists of data that does not form a valid instruction.
- `contain_invalid_instruction_2`: Contains an instruction that cannot be decoded.
- `execute_infinite_loop_2`: the routine implements an infinite loop that traps execution without any conditional exit or side effects.
- `empty_routine_4`: The routine contains no executable instructions.
- `execute_infinite_loop_16`: Runs a loop that repeats indefinitely.
- `contain_invalid_instruction_3`: Contains data that results in a bad instruction.
- `null_routine_3`: This routine is empty and contains no valid instructions.
- `contain_invalid_instruction_4`: Consists of invalid opcode data.
- `enter_infinite_loop`: The routine executes an infinite loop that never exits.
- `nop_padding_instruction`: The routine contains a single non-executable instruction acting as padding or a gap between executable code blocks.
- `return_empty_routine_2`: The routine contains no executable instructions and performs no operations.
- `nop_padding_2`: an empty instruction or padding acting as a gap in the executable code.
- `empty_routine_5`: the routine contains no executable instructions and performs no operations.
- `empty_routine_6`: the routine contains no executable instructions or any valid operations.
- `loop_indefinitely`: The routine executes an infinite loop.
- `return_empty_routine_3`: The routine contains no executable instructions and performs no operations before returning or terminating control flow.
- `execute_infinite_loop_3`: This routine executes an infinite loop that never exits.
- `contain_invalid_instruction_5`: Contains data that triggers a bad instruction warning.
- `execute_invalid_instruction`: Contains a single byte that does not form a valid instruction.
- `execute_infinite_loop_4`: the routine executes an infinite loop that never exits.
- `execute_infinite_loop_5`: The routine enters an infinite loop that never exits.
- `null_routine_2`: The routine is empty and contains no valid instructions.
- `abort_execution`: terminates the current execution flow due to an invalid or unrecognized instruction.
- `execute_infinite_loop_6`: the routine performs an unconditional jump to itself, preventing any further instruction execution.
- `nop_instruction`: The routine contains a single non-functional instruction that does not perform any data or register modification.
- `create_infinite_loop_2`: Executes an infinite loop that does not exit.
- `create_infinite_loop_3`: Performs a continuous loop without any side effects or exit condition.
- `create_infinite_loop_4`: Contains an unconditional loop that runs indefinitely.
- `create_infinite_loop_5`: Implements a loop that repeats without any conditional checks.
- `create_infinite_loop_6`: Runs a loop that never terminates.
- `create_infinite_loop_7`: Perpetuates an infinite loop execution.
- `halt_on_bad_data`: Stops execution due to invalid or unrecognized instruction data.
- `create_infinite_loop_8`: Provides a loop that repeats without any way to exit.
- `execute_empty_routine`: the routine contains no valid instructions and performs no operations.
- `create_infinite_loop_9`: Executes a loop that cycles indefinitely without modification of any registers.
- `halt_execution`: terminates execution or represents an invalid or unreachable instruction.
- `null_routine`: the routine is empty and contains no executable instructions.
- `execute_infinite_loop_7`: The routine implements an infinite loop that causes the program execution to hang or wait indefinitely.
- `skip_instruction`: an empty routine that contains no executable instructions or valid or recoverable instruction data.
- `execute_infinite_loop_8`: The routine implements an infinite loop that traps execution without performing any operations or exiting.
- `execute_infinite_loop_9`: The routine performs an infinite loop that never exits.
- `enter_infinite_loop_2`: The routine executes an infinite loop that prevents further instruction flow.
- `empty_routine_7`: this routine contains no executable instructions.
- `return_void`: the routine is empty and performs no operations before returning.
- `nop_padding_3`: the routine contains no executable instructions and serves as a padding or gap in the instruction stream.
- `execute_infinite_loop_10`: The routine enters an infinite loop that repeats a no-operation or a jump-to-self instruction indefinitely.
- `undefined_instruction_trap`: the routine contains invalid or unrecognized instruction data, causing execution to terminate or jump to an error handler.
- `return_empty_routine_4`: The routine contains no executable instructions and performs no operations before returning.
- `undefined_instruction_error`: The routine executes an invalid or unrecognized instruction, causing the control flow to terminate or error out.
- `execute_infinite_loop_11`: the routine enters an infinite loop that never exits.
- `abort_execution_2`: terminates the program execution or triggers an error state due to invalid or missing instruction data.
- `nop_padding_stub`: This routine contains no executable instructions and acts as a padding or gap in the instruction stream.
- `enter_infinite_loop_3`: the routine executes an empty infinite loop that never exits.
- `execute_infinite_loop_12`: the routine performs an infinite loop that never exits.
- `execute_infinite_loop_13`: This routine implements an infinite loop that traps execution without performing any operations or exiting.
- `execute_infinite_loop_14`: The routine enters an infinite loop that executes a single jump or branch back to itself, preventing further code execution.
- `terminate_execution`: stops the processor execution or enters an undefined state.
- `enter_infinite_loop_4`: This routine contains a loop that never terminates.
- `nop_padding_sequence`: This routine is empty and contains no valid executable instructions.
- `execute_halt_instruction`: the routine executes a single instruction that stops or breaks the current execution flow.
- `undefined_nop_equivalent`: the routine contains invalid or non-executable instruction data.
- `run_endless_loop`: The routine enters a loop that runs indefinitely.
- `loop_indefinitely_2`: This routine performs an infinite loop.
- `undefined_nop_padding`: The routine contains no executable instructions and acts as a gap or padding in the instruction stream.
- `perform_infinite_loop`: The routine executes a loop that repeats without end.
- `create_infinite_loop_10`: The routine implements an infinite loop.
- `empty_routine_8`: the routine contains no executable instructions or valid instruction data.
- `halt_execution_alt`: This routine causes the program to stop due to bad instruction data.
- `empty_routine_9`: This routine contains no valid instructions and performs no operations.
- `halt_execution_2`: The routine contains invalid or unrecognized instruction data, causing execution to stop or fail.
- `halt_execution_v4`: This routine stops execution because of bad instruction data.
- `nop_padding_4`: The routine contains no executable instructions, serving as a single-byte padding or gap in the instruction stream.
- `enter_infinite_loop_5`: Performs an endless loop that prevents further execution.
- `halt_execution_3`: The routine contains invalid or incomplete instruction data, causing the processor to stop or fail to proceed with further instructions.
- `nop_padding_instruction_2`: an empty instruction acting as a single-byte or multi-byte padding gap in the instruction stream.
- `undefined_halt_instruction`: the routine contains invalid or non-executable instruction data and terminates execution.
