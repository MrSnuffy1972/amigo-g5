# Game 2193 (faery tale adventure, the (1986)(microillusions)[a4]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02193). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902096 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 442 of 442 named (0 author symbols), 442 of 442 described (65 by rule, 99 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2408 of 2408 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 13994 of 14026 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 123816 |
| code | 47940 |
| data | 680501 |
| padding | 3759 |
| unused | 46080 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf:fmain | 3 | 13879 | 353 |
| 9 | 1 | 51 | 2 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 86 | 86 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EF8) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 59 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 32-136 (bytes 16384-70144) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 149-159 (bytes 76288-81920) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.69)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 164-256 (bytes 83968-131584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.96)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 258-311 (bytes 132096-159744) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.47)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 314-340 (bytes 160768-174592) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 342-342 (bytes 175104-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 344-393 (bytes 176128-201728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.98)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 396-415 (bytes 202752-212992) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.67)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 419-422 (bytes 214528-216576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 424-445 (bytes 217088-228352) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 447-453 (bytes 228864-232448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.20)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 455-461 (bytes 232960-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.34)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 471-471 (bytes 241152-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 473-473 (bytes 242176-242688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 477-477 (bytes 244224-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 479-535 (bytes 245248-274432) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.97)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 538-671 (bytes 275456-344064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.89)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 673-673 (bytes 344576-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 677-695 (bytes 346624-356352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.90)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.11)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 714-718 (bytes 365568-368128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 720-756 (bytes 368640-387584) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.15)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 760-782 (bytes 389120-400896) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.68)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 784-791 (bytes 401408-405504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 800-868 (bytes 409600-444928) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.57)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 896-917 (bytes 458752-470016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.01)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 920-955 (bytes 471040-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 960-1171 (bytes 491520-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 1312-1370 (bytes 671744-701952) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.24)
- faery tale adventure, the (1986)(microillusions)[a4]/Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf: blocks 1376-1501 (bytes 704512-769024) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf:fmain

- `clear_workspace_then_start`: Clears a block of 944 longwords starting at a computed offset, stores the system base pointer and a global pointer into the block header, then calls a routine to continue startup.
- `open_resources_and_build_display`: Opens the graphics, layers and trackdisk resources named in the referenced text and loads the Amber font at size 9.
- `free_display_resources`: Releases the buffers and resources the setup allocated, each guarded by its own flag bit and freed at a fixed size.
- `index_six_length_prefixed_records`: Calls a library routine to set up a 920-byte area, then walks a block of six length-prefixed records.
- `open_door_or_report_locked`: Searches the cells around a given position for a door tile and, when one matches, rewrites the neighbouring cells to show the open form.
- `setup_main_display_and_run`: Sets up the main display, positions five image slots, and loads the screen and page data before the long game-state code continues.
- `shift_tracked_position_and_scan`: Records a new position by adding its change since the last one to the saved coordinates, and optionally recomputes an index from the high bits.
- `update_region_from_position`: Looks up which rectangular region the current position falls in, using a table of region bounds, and calls a handler when the region code changes.
- `update_state_when_table_changes`: Reads a value through a lookup routine and adds the result to a saved offset, then compares a per-frame table value with the stored one.
- `store_offset_coordinates`: Reads two values from lookup routines and stores them as offset coordinates, each adjusted by 150 from the tracked position.
- `find_free_spot_near_position`: Tries up to fifteen offsets around a centre point until the check accepts one, then stores that position in a 22-byte record and fills in its type, state and frame fields.
- `reset_slot_and_play_effect`: For a slot whose counter is not positive and whose type is neither fourteen nor fifteen, it resets the slot and gives it a fixed type code.
- `set_actor_state_and_position`: Chooses a state code of six for input ten and five otherwise, reloads the matching data when the stored state changes, then places the slot at an offset from a stored position and clears its counters.
- `reset_state_and_load_level`: Resets a set of state variables to fixed starting values, then either calls a setup routine or loads the next entry from a table of eight-byte records.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d7.
- `set_view_offsets_from_input`: Computes a set of view coordinates from the input value and a second value read from a helper, storing them as offsets.
- `select_table_group_and_apply`: Chooses an index into a table based on the tracked position and a state flag, then calls a routine with four values from that table.
- `update_area_code_from_position`: Recomputes an area code from the tracked position and, when it differs from the stored code, stores the new code and calls a handler.
- `swap_double_buffer_pointers`: Swaps two saved buffer pointers so the frame being drawn and the frame being shown trade places.
- `open_input_device`: Creates a message port and an input request, then opens the input device by name and returns one on success.
- `close_input_device`: Sets up a close request on the input device, sends it, then closes the device and releases the port and request.
- `place_sprite_slots_from_table`: Walks a per-level table of sprite entries and records the flagged ones, then fills the remaining slots of twelve with two alternating image codes.
- `draw_sprite_slot_images`: Chooses an image index for a sprite slot from the current level, then draws it at the slot position, with a second image for the frame pair and a third placed from a per-level offset.
- `dispatch_level_mode_actions`: Switches on the current level and then on a mode code.
- `place_actor_or_retry`: Tries up to twenty-five times to find a valid position for the stored coordinates, and when one is found it calls a routine to place the actor there and then calls a handler with code five.
- `place_actor_and_notify`: Places the actor at the stored coordinates through a routine taking a flag of one, then calls the same handler with code five.
- `set_level_and_reset_counter`: Stores the given value as the current level and clears a counter, then calls a routine to load that level, but only when a lock flag is clear.
- `load_config_bytes_from_lookup`: Fills several small tables by calling a lookup routine for consecutive indices, storing seven pairs of values and six further values.
- `wait_until_counter_reaches_ten`: Repeatedly calls a routine until the stored counter reaches ten, then returns.
- `refresh_changed_slot_images`: For the current slot, compares each stored field with its last value and, wherever a value has changed, loads the matching image block and records the new value.
- `execute_conditional_callback`: Checks a bit in a global variable and, if set, calls a function pointer using two values from a table.
- `clear_buffers_set_table`: Clears three 35-byte buffers, points the working table at the entry for the current level, and clears a 60-byte block.
- `store_value_in_node`: Stores the value passed on the stack into the second longword of the node that the address register points to.
- `process_event_record_list`: Walks a list of event records, and for each one either stores a looked-up code in a circular buffer or copies a changed value into it when a position flag changes.
- `queue_mapped_event_code`: Converts a changed coordinate pair into an event code, using a range check and a division by nine, and stores that code in a circular buffer.
- `clamp_position_and_walk_list`: Clamps a moving object's coordinates into the playfield range, stores them, and calls a handler when the object has one.
- `read_ring_buffer_byte`: Returns the next byte from a 128-byte circular buffer when the read and write indices differ, advancing the read index with a wrap at 127.
- `next_pseudo_random_value`: Advances a 16-bit seed by multiplying it and adding one, then returns a value built from shifted bits of the new seed.
- `mask_random_value`: Takes a fresh random value and returns it ANDed with the parameter, so only the bits set in the mask survive.
- `pick_random_bit`: Returns the lowest bit of a fresh random value, so the result is zero or one.
- `pick_random_two_bits`: Returns the low two bits of a fresh random value, so the result runs from zero to three.
- `pick_random_three_bits`: Returns the low three bits of a fresh random value, so the result runs from zero to seven.
- `pick_random_six_bits`: Returns the low six bits of a fresh random value, so the result runs from zero to sixty-three.
- `return_random_byte`: Returns the low byte of a fresh random value and nothing else.
- `pick_random_below_limit`: Takes a fresh random value modulo the limit passed in, and returns the remainder together with the limit.
- `invoke_library_vector_60`: Calls a routine, then a library function at a fixed offset from a stored library base, and returns the two result registers unchanged.
- `format_number_as_decimal_text`: Writes an unsigned number into a ten-character buffer as decimal digits, filling from the right.
- `format_number_into_field`: Writes an unsigned number as decimal digits into a field, working from a given end position toward the start.
- `fill_space_with_space`: Fills a memory range with space characters by decrementing a counter.
- `run_nested_callback_loop`: Runs three nested countdown loops, calling a library routine on every innermost pass, and makes extra calls to two more routines when the outer counter is above nine.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `draw_text_string_segments`: Walks a zero-terminated string, counting each run of ordinary characters and passing it to a text-output routine.
- `lookup_tile_flag_nibble`: Derives a tile index from two coordinates and a lookup table, then tests a bit chosen from the coordinate flags.
- `calculate_attribute_value`: Performs complex arithmetic on input values and table data to return a single byte.
- `call_library_then_decrement_counter`: Calls a library function at a fixed offset, then decrements a stored counter by four.
- `call_library_reduce_counter`: Calls a library function at a fixed offset, then subtracts a stored value plus four from a stored counter.
- `call_routine_eighteen_times`: Calls one routine with a stored parameter, then calls another routine eighteen times in a row, and returns the input registers unchanged.
- `call_routine_six_times`: Calls one routine six times in a row and returns the input registers unchanged.
- `copy_words_into_five_planes`: Copies pairs of 16-bit words from a source block into five destination buffers, writing each pair as two rows forty bytes apart and advancing eighty bytes per row.
- `return_immediately_stub`: Contains only a return, with no calls, memory access or register work.
- `call_routine_with_stored_parameter`: Calls one routine with a stored parameter and returns the input registers unchanged.
- `execute_sequence_of_callbacks`: Calls a series of function pointers stored in a sequence and returns a combined result.
- `forward_call_and_return`: Calls one other routine and returns straight away, with no other work.
- `process_clamped_tile_grid`: Clamps two coordinates into the map window, then walks a grid of nine rows by eighteen columns.
- `pack_bitplane_bits_into_words`: Reads a table of bit-selection bytes, shifting one bit from each lookup into a set of registers.
- `copy_tile_block_rows`: Copies an eight-row block of four-word tile data into a destination area, with the row offsets set by the parameters.
- `compute_tile_map_address`: Clamps the row and column derived from two coordinates into the map bounds, then combines them with a tile nibble and a table lookup to return the address of the map entry.
- `fill_tile_window_from_map`: Walks a grid of positions around the given coordinates, clamps each to the map bounds, and stores the looked-up tile code for each position in a working buffer.
- `unpack_run_length_block`: When a mode flag is set, expands a run-length packed buffer into an output area.
- `scale_value_by_table`: When the index is below eight, scales a short value by a table entry chosen by that index, halves it, adds it to the first value and keeps the result to fifteen bits.
- `scale_value_with_sign`: When the index is below eight, adds a table-scaled amount to the low fifteen bits of the first value and keeps the sign bit of that value.
- `copy_bit14_into_bit15`: Sets the top bit of the value when bit fourteen is set, and clears the top bit otherwise.
- `clear_top_bit`: Returns the value with its top bit cleared, keeping the low fifteen bits.
- `step_scroll_offsets_toward_target`: Moves two stored offsets toward target values: each snaps to its target when it is far away, and otherwise steps by one when the difference exceeds a set margin.
- `step_scroll_offsets_from_registers`: Steps two stored offsets from register inputs.
- `nudge_scroll_offsets_from_registers`: Nudges the horizontal offset by one when it lies beyond twenty, and nudges or snaps the vertical offset depending on how far it is from its target.
- `nudge_scroll_offsets_forward`: Moves the horizontal offset forward by one when the input is beyond twenty, and nudges or snaps the vertical offset depending on its distance from the target.
- `nudge_vertical_scroll_offset`: Nudges the vertical offset by one toward the input or snaps it to a computed edge when it is far away.
- `nudge_vertical_offset_from_input`: Moves the vertical offset down by one when the input is below minus ten, or up by one when it is above ten, and leaves it alone inside that band.
- `nudge_vertical_offset_forward`: Increases the vertical offset by one when the input is above ten, and otherwise leaves it unchanged.
- `handle_error_message`: When the status code is not two, calls a routine with fixed arguments, then one with a global pointer and six, and finishes by calling again with the status code and two.
- `map_value_to_small_index`: Returns a small index from three to ten by checking the input against a sequence of thresholds, starting from a table byte for the value.
- `read_joystick_direction_code`: When a flag is clear, reads two joystick direction bits, combines them into a code from a lookup table and sets a debounce counter.
- `update_joystick_state`: Reads input from hardware registers to determine a direction code and updates a global state via a callback if the value changes.
- `update_joystick_direction_state`: Reads the joystick port direction bits and maps them to a code from a table, with a counter that resets when the stick is centred.
- `clamp_counter_and_notify`: Subtracts twenty from a stored counter and clamps the result into the range zero to nine, resetting the counter when it goes out of range.
- `reset_counter_to_nine`: Clears a counter, and when the stored value is not nine it stores nine there and calls a handler with nine.
- `resolve_two_step_code`: Calls a routine and, if its short result is neither one nor at least ten, calls a second routine and zeroes the result when that is neither one nor below eight.
- `combine_five_planes_inverted`: ANDs five source words together for each position in a block, inverts the result and writes it to the destination.
- `setup_blitter_for_five_blits`: Waits for the blitter to be free, then programs the blitter registers for five blit slots.
- `set_blitter_regs`: Writes fixed control values to the blitter's control and modulo registers, sets the destination address from the first parameter and the blit size from the second, and returns zero.
- `queue_masked_blit_rows`: For five rows, programs the blitter with first and last word masks computed from a shift value, and sets the source, destination and modulo addresses from stored tables.
- `program_single_blit`: Programs the blitter for one rectangle.
- `program_five_blit_rows`: Programs five blit rows from a position table, setting the control values, the destination and source addresses, and the modulo values for each row.
- `program_five_blit_rows_alt`: Programs five blit rows from a position table with the row and column offsets swapped, setting the control values, addresses and modulo values for each row.
- `call_indirect_pointer`: Executes a function through a pointer retrieved from a global table.
- `call_indirect_pointer_alt`: Executes a function through a different pointer retrieved from a global table.
- `compare_offsets_with_mode_thresholds`: Takes the differences between a stored position and two coordinates, records the sign of each difference, and compares their combined magnitude against a threshold chosen by the mode code.
- `process_movement_logic`: Calculates direction and movement flags by comparing input coordinates against a table of offsets.
- `lookup_direction_code_from_signs`: Works out a direction code from the signs of two movement components, using a lookup table indexed by the combined signs.
- `select_direction_from_signs`: Works out a direction code from the signs of two movement components, using a lookup table, and sets a state code.
- `set_facing_from_components`: Computes a facing value from two components, negating one when a status bit is clear, and masks the sum to three bits.
- `store_masked_byte_difference`: Stores the difference of two components, masked to three bits, into one byte of a record, and sets a state code of twelve unless the mode is five.
- `update_actor_movement_step`: Adjusts a stored value by a step amount, then either sets a stop code or passes the position on to a further routine, depending on a state code and flag.
- `advance_actor_toward_target`: Subtracts a step length from a stored value and clamps negative results to zero.
- `choose_target_offset_by_direction`: Subtracts a step from a stored value, clamps negative results to zero, and picks a target offset based on the direction codes.
- `choose_target_offset_by_direction_2`: Picks one of three target offsets based on the direction codes, calls a routine with the chosen offset, then checks a state value and calls further routines when it is not six or four.
- `choose_target_offset_by_direction_3`: Picks a target offset based on the direction codes and calls a routine with it, then checks a state value and calls further routines when it is not six or four.
- `call_offset_then_check_state`: Calls a routine with a mode of three and an offset of 400 plus a computed value, then checks a state value and calls further routines unless it is six or four.
- `count_defeated_and_fleeing_foes`: Counts the foes that have no health left and those in a fleeing state, then reports each non-zero count through a message routine when the battle state allows it.
- `tally_entity_states`: Iterates through a list of entities to count specific states and calls reporting routines based on the totals.
- `tally_fleeing_foes_and_report`: Counts the foes that are in a fleeing state and reports the total through a message routine when the battle state allows it.
- `report_battle_outcome_message`: Reports the battle counts through message routines, choosing the messages from the battle state, and calls a further routine when the flag is set.
- `report_foes_fled_or_defeated`: Reports how many foes fled or were defeated in battle using the referenced messages, and calls a further routine when the flag is non-zero.
- `check_actor_collision_at_index`: Checks an actor against the other active actors and returns a code when a nearby one is found, skipping slots that are dead or in the excluded state.
- `report_nearby_actor_code`: Scans the other active actors for one within eleven units horizontally and nine vertically of this actor, and returns the code sixteen when one is found.
- `scan_nearby_actors_from_index`: Scans the other active actors from a starting index for one within eleven units horizontally and nine vertically, and returns sixteen when one is found.
- `scan_nearby_actors_skip_first`: Scans the other active actors, skipping its own slot and the slot at index one, for one within ten units horizontally and eight vertically, and returns sixteen when one is found.
- `find_closest_actor_index`: Scans the active actors from index one, skipping some dead or excluded entries, and tracks the one with the smallest value from a distance routine.
- `find_closest_actor_to_record`: Scans the active actors and compares each against a stored record, keeping the smallest value from a distance routine.
- `compute_approximate_distance`: Works out the absolute horizontal and vertical gaps between two positions and returns a distance estimate from them.
- `execute_conditional_pair`: Compares two values and, if a condition is met, calls two function pointers in succession.
- `call_two_routines_in_order`: Calls two routines in turn and returns.
- `try_move_actor_to_offset`: Works out new coordinates for an actor from the given offsets, and stores them only when the collision check finds no conflict.
- `store_actor_position`: Stores two supplied coordinates into the actor record selected by the index, and returns one.
- `draw_actor_image_pair`: Draws one or two images for an entry in a four-byte table at the given position, using fixed sizes and offsets.
- `run_palette_fade_steps`: Chooses a constant from the current level and a mode value, clamps the intensity between zero and a limit, then loops over thirty-two colour entries calling routines to fetch and update each one.
- `randomize_palette_entries`: Fills a 32-entry table with random values below 4096, loads it as a colour table and pauses one step, repeating for thirty-two rounds.
- `process_queued_message_event`: Takes the next entry from a 32-entry event queue when one is waiting, then dispatches on its type code.
- `dispatch_queued_event_by_type`: Advances the event queue index and dispatches the entry it read by type, calling message and display routines for each case.
- `dispatch_event_from_argument`: Dispatches on the event code passed in the data register, calling message and display routines for each case such as types two, three and four.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `enqueue_event_code`: Adds the event code to the 32-entry queue unless the queue is full, and advances the write index when it does.
- `draw_text_frame_and_string`: Measures a zero-terminated text string, draws a fixed frame with two calls, then writes the text through a text routine with its measured length.
- `draw_default_text_frame`: Draws a fixed text frame and caption using three library calls, with fixed positions and sizes.
- `print_null_terminated_text`: Measures a zero-terminated string and passes it, with its length, to a text routine.
- `wrap_message_into_lines`: Walks a message string, breaking lines at carriage returns and spaces, and expands percent escapes by inserting entries from a per-level table of names.
- `copy_message_chars_into_lines`: Copies message characters into a line buffer, starting a new line at each carriage return and ending the buffer with a zero byte.
- `copy_message_char_or_name`: Copies one message character into the line buffer, or when the character is a percent escape it copies the current name from a per-level table of names.
- `wrap_message_line_loop`: Continues the line-wrapping loop by reading message characters up to the line limit, breaking at spaces and carriage returns, and ending the line at a zero byte.
- `store_message_char_and_continue`: Stores the current character in the line buffer, advances the count, and keeps the line-wrapping loop going until a break or the limit is reached.
- `finish_message_line`: Ends the current line with a zero byte, and when the character count is still within the limit it hands the line to an output routine.
- `return_immediately_stub_2`: Contains only a return, with no calls, memory access or register work.
- `select_message_and_wrap`: Skips forward through a list of zero-terminated strings to the one at the given index, then lays that text out into lines of up to thirty-seven characters.
- `select_message_from_second_table`: Skips forward through a second list of zero-terminated strings to the one at the given index, then lays that text out into lines of up to thirty-seven characters.
- `skip_strings_and_wrap_message`: Advances past a given number of zero-terminated strings from the supplied pointer, then lays the text out into lines of up to thirty-seven characters.
- `print_item_message_with_suffix`: Prints the message at the current table index, then the supplied text and a closing message.
- `print_table_message_then_text`: Prints the message at the current table index and then the supplied text, using the referenced connecting phrase ' found ' between them.
- `dispatch_indexed_handler_call`: Reads a 16-bit index from a global, subtracts one, scales it by four to fetch a pointer from a fixed table, and passes that pointer to another routine.
- `configure_display_channel_state`: Stores a pointer and clears a field in a display structure, makes several library calls with it using a size of 24 bytes, sets a 16-bit global to 57344, and sets a state byte to two.
- `set_display_mode_three`: Stores a pointer in a global, sets a 16-bit global to 49152 and a state byte to three, and makes two routine calls around that update.
- `loop_descending_value_steps`: Counts down from 100 to 0 in steps of five.
- `loop_ascending_value_steps`: Counts up from 0 to 100 in steps of five.
- `clear_structure_position_fields`: Clears two 16-bit fields at offsets 10 and 8 in the structure that a global pointer references, then calls a routine.
- `run_fixed_setup_call_sequence`: Copies a 32-bit global into another global, then makes a fixed sequence of routine calls with small constants and one value read from a global.
- `append_table_entry_record`: Fills a record buffer for the given index from a table, writing a scaled word, three bytes and a running position at offsets returned by repeated calls.
- `copy_template_into_slot`: Selects a 56-byte slot by index and releases it through a library call if it is already in state two.
- `copy_template_to_fixed_record`: Copies a 14-long template from a global buffer into a fixed local record, sets a 32-bit field to zero and a 16-bit field to nine, and makes a library call with the record.
- `forward_call_to_handler`: Passes control to one other routine and returns.
- `reset_record_and_forward_fields`: Calls a library function on a fixed local structure, clears a 16-bit field in it, then passes five values read from a per-index table entry to a routine.
- `conditionally_forward_slot_call`: Returns early when the slot is in state two and a library check reports zero.
- `load_song_offset_table`: Opens the file named songs through library calls and reads up to 28 length-prefixed segments.
- `setup_display_when_flag_clear`: When a flag byte is zero, calls a library function with 350 and sets up a 320 by 200 structure.
- `render_rows_from_tables`: Loops over 22 rows, reading a step value and an offset from tables, and draws each row with block transfer library calls as either a column of tiles or one block.
- `poll_input_and_store_match`: Calls a library function and stores in a flag byte whether its result equals 32, then returns that same result.
- `copy_memory_block`: Checks if a pointer is within a valid range and then copies a specified number of bytes from a source to a destination.
- `render_quad_via_temp_bitmap`: Works out the four corners of a quadrilateral from a 16-entry offset table and sets two sign flags from cross products.
- `reset_counter_and_forward_copy`: Sets a 16-bit counter global to two, copies a 128-byte buffer through a routine, and makes an indexed call.
- `copy_record_fields_to_globals`: Copies two 16-bit values from a 22-byte record selected by index into fixed globals, adds ten to the second value, stores the byte argument at a fixed offset, and sets a flag byte to one.
- `update_flagged_slot_byte`: Chooses a record offset with a library call, using one of two bases depending on the high bit of a flag byte.
- `update_indexed_slot_byte`: Picks a record offset from a global index and a library call result.
- `store_byte_at_offset_five`: Writes the byte held in a data register into the field five bytes past the address held in an address register, then returns.
- `spawn_records_and_scan_points`: When a table entry is zero and a counter exceeds nine, creates ten records at random positions that a library test accepts, then marks the entry done.
- `seed_records_and_scan_list`: Creates ten records at random positions when a table entry is zero and a counter exceeds nine, then walks a point list through a stack pointer, updating or appending records for points inside screen bounds.
- `scan_point_list_for_records`: Walks a list of six-byte points while the remaining count is positive, checking each against screen bounds.
- `scan_point_list_mid_entry`: Enters the same point-list walk partway through its body and returns once the remaining count is exhausted.
- `advance_to_onscreen_point`: Skips points that lie outside screen bounds, then updates or appends a matching record for the first point inside them.
- `append_onscreen_point_records`: Walks a point list and, for each point inside screen bounds, writes a record with a value of fifteen for type-four points and thirteen otherwise.
- `append_point_records_when_mismatch`: Walks a point list and appends a record for each point inside bounds whose stored position differs.
- `mark_or_append_point_records`: Walks a point list and marks a matching record as type four when its value is fifteen.
- `append_point_records_by_type`: Appends records for points inside bounds, with type zero for type-five points and one otherwise, and walks the list until the count is exhausted.
- `append_typed_point_records`: Appends records for points inside bounds, setting the type field to two for type-six points, zero for type-five points and one otherwise.
- `return_without_action`: Contains only a return instruction, so it performs no work.
- `read_and_match_typed_line`: Reads printable characters into a nine-byte buffer until a carriage return, handling backspace.
- `call_library_teardown_sequence`: Calls three functions on the system library base, then calls two functions on a stored library base and repeats that pair once more.
- `call_and_forward_nonzero_result`: Calls a routine with two arguments and stores its result in a global.
- `probe_df0_device_value`: Calls a routine with the text df0 and a minus-two argument and stores the returned value.
- `stub_bad_instruction_data`: Contains only a bad-instruction marker, so it is not executable code and performs no work.
- `wait_for_flag_with_timeout`: Polls a flag byte up to 300 times, waiting five units between checks.
- `poll_flag_clear_until_timeout`: Checks a flag byte and clears it, returning one, when it is set.
- `wait_then_poll_flag_timeout`: Waits five units before each check of a flag byte.
- `transfer_game_state_to_file`: Builds a file name from a fixed drive prefix and a letter, opens it in a mode chosen by a flag, then passes each state table and its size to a routine.
- `transfer_state_to_winpic`: Loops until an entry is available, then builds a file name from a letter and the df0 winpic prefix.
- `prompt_writable_disk_and_save`: Loops over prompts asking for a writable disk, then runs the save sequence: it opens a letter-named file, passes the game state blocks and their sizes to a routine, and closes the file.
- `save_state_after_disk_check`: Checks for a disk in a loop, then sets a letter into the file name, opens the file, passes the game state blocks and their sizes to a routine, and closes it.
- `save_letter_named_game_file`: Writes a letter into the file name, opens the file in a mode chosen by a flag, then passes the game state blocks and their sizes to a routine.
- `save_default_game_file`: Opens the save file under its fixed name in mode 1005, passes the game state blocks and their sizes to a routine, and closes it.
- `report_save_failure`: Runs the save step and, when it fails, passes the text ERROR: Couldn't save game.
- `report_load_failure`: Passes the text ERROR: Couldn't load game.
- `retry_winpic_request_until_ready`: Sets a request flag when a related byte is clear, then loops on a routine call with the df0 winpic name until it returns nonzero.
- `prompt_game_disk_until_ready`: Shows the text Please insert GAME disk.
- `forward_single_library_call`: Makes one library call with a stored pointer and a stored value, then returns with no other work.
- `transfer_block_with_error_check`: Calls the library transfer function on the stored handle with two arguments, using a second routine instead when a flag byte is set.
- `transfer_frame_args_check`: Calls the library transfer function on the stored handle with two arguments taken from the caller's frame and stores the result.
- `set_rectangle_bounds_from_center`: Stores a rectangle in a 12-byte slot selected by index, with its left and right edges 250 units from a given center x and its top and bottom edges 200 units from a given center y.
- `run_indexed_display_sequence`: Runs a sequence of library calls keyed to an index counter, with fixed pauses between steps, and updates several state values.
- `animate_winpic_column_steps`: Loads the winpic picture, then loops from 25 down to minus 30, rebuilding a 28-entry set of values from a table on each step with pauses between steps.
- `repeat_column_animation_steps`: Clears a 16-bit slot and rebuilds a 28-entry set of values from a table on each step, pausing between steps.
- `run_column_steps_nested`: Sets two stored values from a random source on each outer pass, then runs the column steps with a rebuilt table and pauses between them.
- `select_value_by_flag_byte`: Returns ten when the byte at the given index in a table is nonzero, otherwise eight.
- `render_tracked_position_if_due`: When a masked flag is zero or a byte exceeds 97, passes a position to a routine.
- `render_tracked_position_plain`: When a masked flag is zero or a byte exceeds 97, passes a position computed from a global to a routine.
- `render_fixed_position_once`: Calls a routine with the fixed values 100, 100, 100 and 1 and a buffer, then returns.
- `dispatch_actor_action_code`: Stores a value in an actor record and branches on an action code from one to ten.
- `dispatch_actor_action_register`: Stores the action code in a record byte and branches on codes one to ten.
- `dispatch_actor_action_tail`: Handles the action branches for codes eight, four, three, five, two, six and ten using preset registers.
- `dispatch_actor_action_flagged`: Dispatches action codes eight, four, three, five, two, six and ten.
- `forward_to_action_routine`: Calls one action routine and returns, with no other work.
- `dispatch_actor_action_short`: Dispatches action codes four, three, five, two, six and ten, running the routine for each only when a flag argument is nonzero.
- `dispatch_actor_codes_from_three`: Handles action codes three, five, two, six and ten.
- `dispatch_actor_codes_from_five`: Handles action codes five, two, six and ten.
- `dispatch_actor_codes_from_two`: Handles action codes two, six and ten.
- `dispatch_actor_codes_six_ten`: Handles action codes six and ten.
- `run_table_action_if_flagged`: When a flag argument is nonzero, takes an index from a library call and passes a table value offset by 20 to a routine.
- `apply_code_ten_action`: When the action code is ten, it calls a routine if a flag argument is nonzero, then sets a byte at offset 16 of the record to 12.
- `decrement_counter_and_notify`: Subtracts the argument from a 16-bit counter global.
- `forward_yum_message_call`: Passes a global buffer associated with the text Yum! to a routine and returns.
- `load_matching_record_and_scan`: Opens a file in mode 1005 and matches a stored key against a list of records, reading the matching data from the file.
- `advance_sequence_position`: Clears a byte at offset one of a structure and adds a 16-bit value from offset two into a 32-bit field at offset 16.
- `step_audio_sequence_channel`: Advances a channel's sequence when its timer has run out, processing command pairs: 129 loads a table value, 144 sets a value, and 255 ends the sequence, looping when its argument is nonzero.
- `decrement_count_and_set_state`: Sets two 16-bit fields in a structure to 256, then decrements a counter byte in a second structure if it is nonzero.
- `start_audio_channel_one`: Writes the pointer, length, period and volume registers for audio channel one, then sets its DMA enable bits in the custom chip.
- `silence_audio_channel_one`: Clears the low bit of a state byte, sets the channel one volume register to zero, and sets its period register to two.
- `store_four_longs_into_globals`: Stores four 32-bit arguments into four fixed global slots, one argument per slot, and returns.
- `reset_all_audio_channels`: Clears the volume registers of all four audio channels and copies the first four words of a sample table into channel state.
- `mute_all_audio_channels`: Zeroes the volume registers of all four audio channels and sets a state byte to 255.
- `install_music_interrupt_servers`: Fills two interrupt structures with a handler and registers them through exec calls for the audio 4 and music vblank entries.
- `remove_music_interrupt_server`: Calls a routine, then removes the music interrupt server through an exec call.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_table_store_slot_data`: Computes a slot offset from a global index, sets a 16-bit field, and zeroes a block of 944 longs in a table.
- `stub_empty_return_routine`: Contains only a return, so it performs no work.
- `run_command_line_with_arguments`: Opens the dos library and calls a routine with a fixed error code if that fails.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `copy_string_pad_zeros`: Copies bytes from a source string into a destination up to a counted limit or a zero terminator, then fills the rest of the counted length with zeros.
- `append_string_bounded`: Appends a source string to the end of a destination string, copying at most 32766 bytes and terminating the result.
- `copy_string_with_limit`: Appends a source string to the end of a destination string, stopping at the zero terminator or when the counter argument runs out.
- `copy_string_to_buffer`: Copies a zero-terminated source string, including its terminator, into the destination buffer.
- `multiply_low_32_bits`: Multiplies two 32-bit register values by splitting each into 16-bit halves and summing the cross products.
- `call_with_sign_correction`: Works out the sign of the result from the signs of two inputs, then calls a routine and negates its result when the signs differ.
- `negate_when_first_negative`: Calls a routine, then negates the value it leaves in the second register when the first input is negative.
- `call_and_return_concatenated`: Calls a routine and returns a 64-bit value made from the second register as left by the call, with the original second register in the low word.
- `forward_call_and_return_2`: Calls one routine and returns, doing nothing else in its body.
- `divide_32_bit_values`: Divides a 32-bit dividend by a 32-bit divisor, using a 16-step shift-and-subtract loop when the divisor's high half is set and 16-bit divide steps otherwise.
- `compute_string_length`: Walks a zero-terminated byte string to find its terminator and returns the number of bytes before it.
- `invoke_optional_hook_then_run`: Calls a stored function pointer from a global slot when that slot is set, then calls a second routine.
- `release_stored_resources`: Runs a 20-step loop calling a routine, then releases three stored resources through library calls when they are set.
- `release_indexed_slot_resource`: Checks that the index is between 0 and 19 and that its slot is set, otherwise it records error code three and returns minus one.
- `forward_library_call_minus_36`: Loads a library base from a stored global and calls the function at offset minus 36 from it, then returns.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `forward_library_call_minus_132`: Loads a library base from a stored global and calls the function at offset minus 132 from it, then returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `forward_library_call_minus_84`: Loads a library base from a stored global and calls the function at offset minus 84 from it, then returns.
- `forward_library_call_minus_30`: Loads a library base from a stored global and calls the function at offset minus 30 from it, then returns.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `forward_library_call_minus_60`: Loads a library base from a stored global and calls the function at offset minus 60 from it, then returns.
- `forward_library_call_minus_42`: Loads a library base from a stored global and calls the function at offset minus 42 from it, then returns.
- `forward_library_call_minus_66`: Loads a library base from a stored global and calls the function at offset minus 66 from it, then returns.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `forward_library_call_minus_90`: Loads a library base from a stored global and calls the function at offset minus 90 from it, then returns.
- `forward_library_call_minus_48`: Loads a library base from a stored global and calls the function at offset minus 48 from it, then returns.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `forward_library_call_minus_414`: Loads a library base from a different stored global and calls the function at offset minus 414 from it, then returns.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_slot_record`: Requests a free index, then allocates a 34-byte record through a library call.
- `release_slot_record`: Releases a record allocated earlier by calling a routine on its linked field when that field is set and marking its state and index as unused.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_with_size_48`: Passes its argument and the value 48 to a library function loaded from a stored base, then returns.
- `forward_library_call_with_argument`: Passes its argument to a library function loaded from a stored base, then returns.
- `allocate_typed_record_block`: When the pointer argument is nonzero, allocates a block of the requested size through a library call.
- `release_typed_record_block`: When the pointer is nonzero, marks the record's state and two link fields as unused and frees its block through a library call.
- `mark_record_unused_and_free`: Marks a record as unused by setting its state byte and two link fields to unused values, then calls a library function to free it.
- `forward_library_call_minus_198`: Loads a library base from a different stored global and calls the function at offset minus 198 from it, then returns.
- `forward_library_call_minus_198_2`: Loads a library base from a stored global and calls the function at offset minus 198 from it, then returns.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `forward_library_call_minus_294`: Loads a library base from a stored global and calls the function at offset minus 294 from it, then returns.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `execute_indirect_call`: Performs an indirect call to a function address retrieved from a table via a global pointer.
- `forward_library_call_minus_210`: Loads a library base from a stored global and calls the function at offset minus 210 from it, then returns.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `initialize_self_linked_node`: Sets the first longword of a block to its own address plus four, clears the second longword, and stores the block's address in the third.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `forward_library_call_minus_552`: Loads a library base from a stored global and calls the function at offset minus 552 from it, then returns.
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
- `forward_library_call_minus_30_2`: Loads a library base from another stored global and calls the function at offset minus 30 from it, then returns.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `forward_library_call_minus_462`: Loads a library base from another stored global and calls the function at offset minus 462 from it, then returns.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `forward_library_call_minus_414_2`: Loads a library base from a stored global and calls the function at offset minus 414 from it, then returns.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_call_2`: Calls a function through a pointer located at an offset within a table accessed via a global variable.
- `forward_library_call_minus_468`: Loads a library base from a stored global and calls the function at offset minus 468 from it, then returns.
- `forward_library_call_minus_360`: Loads a library base from a stored global and calls the function at offset minus 360 from it, then returns.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `forward_library_call_minus_216`: Loads a library base from a stored global and calls the function at offset minus 216 from it, then returns.
- `forward_library_call_minus_240`: Loads a library base from a stored global and calls the function at offset minus 240 from it, then returns.
- `execute_indirect_call_3`: Executes a subroutine by dereferencing a pointer stored in a table at a specific offset.
- `forward_library_call_minus_456`: Loads a library base from a stored global and calls the function at offset minus 456 from it, then returns.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `forward_library_call_minus_354`: Loads a library base from a stored global and calls the function at offset minus 354 from it, then returns.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `execute_indirect_call_4`: Performs an indirect jump to a routine address found within a table.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `forward_library_call_minus_36_2`: Loads a library base from a different stored global and calls the function at offset minus 36 from it, then returns.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `forward_library_call_minus_150`: Loads a library base from a different stored global and calls the function at offset minus 150 from it, then returns.
- `forward_library_call_minus_144`: Loads a library base from a different stored global and calls the function at offset minus 144 from it, then returns.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_routine`: This routine is empty and contains no executable instructions.

### Faery Tale Adventure, The (1986)(MicroIllusions)[a4].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `spin_in_empty_loop`: Contains an empty infinite loop with no work, so it never returns.
- `perform_infinite_loop`: The routine enters an infinite loop that prevents further execution.
- `execute_infinite_loop`: This routine enters an infinite loop that prevents further execution of the program flow.
- `perform_infinite_loop_2`: This routine consists of an infinite loop that prevents further execution.
- `execute_infinite_loop_45`: The routine enters a continuous loop that does not perform any operations.
- `enter_infinite_loop_10`: Executes an unconditional jump to itself, resulting in an endless loop.
- `execute_infinite_loop_46`: The routine performs an infinite loop that prevents further execution of the program flow.
- `execute_infinite_loop_2`: This routine performs an infinite loop that prevents further execution of subsequent instructions.
- `execute_empty_loop`: The routine consists of an infinite loop that performs no operations.
- `enter_infinite_loop_11`: The routine enters an unconditional loop that prevents further execution of the current code path.
- `execute_infinite_loop_47`: The routine enters a continuous loop that prevents further execution.
- `execute_infinite_loop_3`: The routine enters an endless loop that prevents further execution of subsequent instructions.
- `nop_padding_segment`: This routine is empty and consists of an unreachable gap in the executable segment.
- `enter_infinite_loop_12`: Executes an endless loop that prevents further instruction flow.
- `execute_infinite_loop_48`: The routine enters a continuous loop that never terminates.
- `enter_infinite_loop_13`: Executes an unconditional loop that prevents further instruction flow.
- `execute_infinite_loop_49`: The routine enters an endless loop that prevents further execution of the program flow.
- `execute_infinite_loop_4`: This routine enters an infinite loop that prevents further execution of subsequent instructions.
- `execute_empty_loop_2`: The routine enters an infinite loop that performs no operations.
- `execute_infinite_loop_5`: The routine enters an infinite loop that prevents further execution of the program flow.
- `execute_infinite_loop_6`: The routine enters an unconditional infinite loop that prevents further execution.
- `execute_infinite_loop_7`: The routine enters an endless loop that prevents further instruction execution.
- `execute_infinite_loop_8`: This routine enters an infinite loop that prevents further execution of subsequent code.
- `execute_infinite_loop_9`: The routine enters an unconditional infinite loop that prevents further execution of the calling code.
- `execute_infinite_loop_10`: The routine enters a continuous loop that performs no operations and never terminates.
- `nop_padding_sequence`: This routine is empty and consists of an infinite loop of no-operation instructions.
- `execute_infinite_loop_50`: Enters an endless loop that prevents further execution of subsequent code.
- `execute_infinite_loop_51`: Performs an endless loop that prevents further execution.
- `execute_infinite_loop_11`: The routine performs an endless loop that prevents further execution.
- `nop_padding_sequence_2`: The routine is empty and contains no executable instructions.
- `execute_infinite_loop_12`: The routine enters an unconditional loop that prevents further execution of the current thread.
- `execute_infinite_loop_13`: The routine enters an unconditional loop that prevents further execution of the calling code.
- `execute_infinite_loop_14`: This routine enters an unconditional infinite loop that prevents further execution.
- `execute_infinite_loop_15`: The routine consists of an infinite loop that prevents further execution.
- `execute_infinite_loop_16`: The routine enters an unconditional loop that prevents further execution of the subsequent code.
- `execute_infinite_loop_17`: This routine enters an infinite loop that performs no operations and never returns.
- `nop_padding_sequence_3`: The routine is empty and performs no operations.
- `execute_infinite_loop_18`: Enters an endless loop that prevents further execution of the current thread.
- `execute_infinite_loop_19`: The routine enters an unconditional loop that prevents further code execution.
- `execute_infinite_loop_20`: The routine enters an infinite loop that prevents further execution of subsequent instructions.
- `execute_infinite_loop_21`: The routine enters an unconditional loop that repeats indefinitely.
- `execute_infinite_loop_22`: The routine enters an infinite loop that performs no operations and never returns.
- `execute_infinite_loop_23`: This routine enters an unconditional infinite loop that prevents further execution of the calling code.
- `perform_infinite_loop_3`: The routine executes an infinite loop that prevents further instruction execution.
- `execute_empty_loop_3`: This routine enters an infinite loop that performs no operations.
- `nop_padding_sequence_4`: This routine is empty and consists of a single instruction that performs no operation, serving as a gap or padding within the executable segment.
- `execute_infinite_loop_24`: The routine enters an unconditional infinite loop that prevents further execution of the current thread.
- `nop_padding_sequence_5`: This routine is empty and consists of a single instruction that does not perform any operations.
- `execute_infinite_loop_25`: The routine enters an unconditional loop that prevents further execution of the program flow.
- `execute_infinite_loop_26`: This routine enters an infinite loop that prevents further execution.
- `enter_infinite_loop`: Performs an unconditional jump to itself, creating an infinite loop that prevents further execution.
- `execute_infinite_loop_27`: The routine enters an infinite loop that prevents further code execution.
- `execute_infinite_loop_28`: The routine enters an infinite loop that prevents further execution of subsequent code.
- `perform_infinite_loop_4`: The routine enters an endless loop that prevents further execution.
- `execute_infinite_loop_29`: This routine enters an infinite loop that prevents further execution of the calling code.
- `execute_infinite_loop_30`: This routine enters an infinite loop that prevents further execution by repeatedly executing a jump instruction to itself.
- `execute_infinite_loop_31`: The routine enters an unconditional loop that prevents further execution of subsequent code.
- `execute_infinite_loop_32`: The routine enters an infinite loop that continuously executes an empty cycle.
- `enter_infinite_loop_2`: Contains an infinite loop structure.
- `stub_idle_spin_loop`: A one-instruction routine whose decompiled body is an endless empty loop, with no work, no calls and no data access.
- `nop_padding_sequence_6`: This routine is empty and consists of a single instruction that performs no operation.
- `enter_infinite_loop_3`: Traps execution in a loop that never terminates.
- `execute_infinite_loop_33`: The routine performs an infinite loop that prevents further execution of the current thread.
- `enter_infinite_loop_4`: Enters a loop that repeats indefinitely.
- `enter_infinite_loop_5`: Stays in a permanent loop.
- `execute_infinite_loop_34`: The routine enters an infinite loop that prevents further instruction execution.
- `execute_infinite_loop_35`: This routine enters an unconditional loop that prevents further execution of the program flow.
- `execute_infinite_loop_36`: The routine enters an infinite loop that prevents further execution of the current thread.
- `execute_infinite_loop_37`: This routine enters an unconditional infinite loop that prevents further execution of the program flow.
- `execute_infinite_loop_38`: This routine enters an unconditional infinite loop that prevents further execution of the current thread.
- `enter_infinite_loop_6`: Runs a loop that never terminates.
- `execute_infinite_loop_39`: The routine enters an endless loop that prevents further execution of the current thread.
- `enter_infinite_loop_7`: Traps execution in a loop.
- `enter_infinite_loop_8`: This routine executes an endless loop that prevents further instruction flow.
- `execute_infinite_loop_40`: The routine enters an infinite loop that never terminates.
- `execute_infinite_loop_41`: The routine enters an endless loop that prevents further execution of the subsequent code.
- `enter_infinite_loop_9`: Enters a loop that repeats forever.
- `execute_infinite_loop_42`: The routine enters an endless loop that prevents further execution of subsequent code.
- `execute_infinite_loop_43`: Enters a loop that never terminates.
- `run_infinite_loop`: Contains an endless loop structure.
- `execute_infinite_loop_44`: The routine enters an endless loop that repeats a no-operation instruction indefinitely.
- `execute_empty_loop_4`: This routine enters an endless loop without executing any instructions.
- `loop_infinitely`: Enters a state of perpetual looping.
- `maintain_infinite_loop`: Runs a loop that repeats indefinitely.
- `perform_infinite_loop_5`: Stays in a loop that does not exit.
