# Game 2195 (faery tale adventure, the (1986)(microillusions)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02195). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902112 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 444 of 444 named (0 author symbols), 444 of 444 described (66 by rule, 95 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2408 of 2408 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14441 of 14473 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 123426 |
| code | 49338 |
| data | 683314 |
| padding | 4562 |
| unused | 41472 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 386 | 1 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf:fmain | 3 | 13938 | 354 |
| 9 | 1 | 51 | 2 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 88 | 86 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EF8) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 59 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 32-136 (bytes 16384-70144) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 149-159 (bytes 76288-81920) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.69)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 164-256 (bytes 83968-131584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.96)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 258-311 (bytes 132096-159744) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.47)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 314-340 (bytes 160768-174592) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 342-342 (bytes 175104-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 344-393 (bytes 176128-201728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.98)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 396-415 (bytes 202752-212992) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.67)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 419-422 (bytes 214528-216576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 424-445 (bytes 217088-228352) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 447-453 (bytes 228864-232448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.20)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 455-461 (bytes 232960-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.34)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 471-471 (bytes 241152-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 473-473 (bytes 242176-242688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 477-477 (bytes 244224-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 479-535 (bytes 245248-274432) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.97)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 538-671 (bytes 275456-344064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.89)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 673-673 (bytes 344576-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 677-695 (bytes 346624-356352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.90)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.11)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 714-718 (bytes 365568-368128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 720-756 (bytes 368640-387584) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.15)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 760-782 (bytes 389120-400896) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.68)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 784-791 (bytes 401408-405504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 800-868 (bytes 409600-444928) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.57)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 896-917 (bytes 458752-470016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.01)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 920-955 (bytes 471040-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 960-1171 (bytes 491520-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 1312-1370 (bytes 671744-701952) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.24)
- faery tale adventure, the (1986)(microillusions)[a]/Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf: blocks 1376-1501 (bytes 704512-769024) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Disk-Validator

- `initialise_validation_state_entry`: Entry point of the disk validation program.

### Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf:fmain

- `clear_block_and_store_pointers`: Clears 951 longwords starting at a computed block address, then stores a pointer and a global value into fixed slots of the same structure and calls a routine.
- `setup_graphics_and_trackdisk`: Opens the libraries and device named in its text and allocates the resources they need, recording a flag bit for each one acquired.
- `release_resources_by_flags`: Checks a set of flag bytes and, for each set bit, releases a stored block through a release routine with its fixed size.
- `build_six_entry_offset_table`: Calls a routine with the constant 920 and a block pointer, then initialises a structure.
- `open_tile_at_position`: Checks the tile at a given position and its neighbours for a lookup value of 15, then searches a 17-entry table for a matching entry.
- `initialize_display_and_input`: Runs a long start-up sequence that initialises several structures and five pointer-plus-offset entries, then sets display and text parameters through indirect calls.
- `update_scroll_and_step_row`: Updates two position offsets by the difference between the new and previous coordinates, then stores the coordinates as the new previous values.
- `find_region_and_update_state`: Repeatedly selects a region table from a global counter and searches it for an entry whose bounds contain the current coordinates.
- `update_animation_state`: Combines the result of a routine with a byte from the entry to produce a new byte, then looks up a table value for the current state.
- `place_entry_at_free_position`: Tries up to fifteen positions near a base point, rejecting any that fail a check routine.
- `update_entity_record_state`: For the entry selected by a global index whose state is idle, sets its timing and state bytes to fixed values and adjusts counters.
- `set_state_and_timing`: Sets a state byte to 6 when the parameter is 10 and to 5 otherwise.
- `initialise_stage_state_display`: Clears a set of state variables, then either makes a single routine call when its parameter is zero or sets up the current stage from a table of starting coordinates.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d7.
- `set_display_offsets_from_param`: Calls two routines, then stores display offsets derived from its parameter and a routine result: several words are set to 160 minus the parameter and others to 100 and 95 minus the result.
- `draw_sprite_from_index`: Chooses a base index from state values and position checks, using 24, 16, 20, 8 or 4.
- `update_zone_index_from_position`: When a state value is below eight, computes a zone index from the two position words.
- `swap_buffer_pointers`: Swaps two stored pointers, copies the first two words of the new current buffer into a global pair, and calls routines with pointers to the buffers.
- `open_input_device_request`: Creates a message port and an I/O request, then opens the input device with that request.
- `close_input_request_and_port`: Sets a field of the I/O request to 10 and a pointer field, makes three routine calls with the request and then one routine call with the port handle.
- `render_twelve_entry_slots`: Walks a table of 18-byte stage entries and records the index of each flagged entry in a 12-slot array, calling a routine for each.
- `render_slot_line`: Looks up a value for a slot from a table, adjusted for the current stage, then sets the drawing parameters and draws two entries at computed offsets.
- `dispatch_stage_event_by_value`: A switch on a stage value that dispatches events.
- `retry_check_then_notify`: Calls a routine up to 25 times, stopping early when a test on the stored coordinates returns 5.
- `notify_position_event`: Calls a routine with the value 1 and the stored coordinates, then calls the notifier with code 5.
- `set_stage_and_refresh`: When a flag bit is clear, stores its parameter as the current stage value, clears a byte and calls a routine.
- `load_font_metric_tables`: Fills two seven-entry arrays by calling a routine with offsets, then fills a six-byte array while noting whether a newline value appears, which sets a line height of 10 instead of 8.
- `wait_until_counter_reaches_ten`: Calls a routine repeatedly while a 16-bit counter is below ten.
- `apply_record_state_for_index`: When the index is below ten, it compares several fields of that record with cached values and calls update routines for any that changed.
- `conditionally_call_indirect`: If a flag byte in a global variable is set, the routine calls an indirect function pointer taken from a global variable with three arguments built from two tables indexed by two parameters.
- `initialize_text_and_stage_pointers`: Calls a routine three times with three buffer addresses and the value 35, then sets a stored pointer from a table indexed by the stage value minus one.
- `store_stack_argument_in_field`: Stores the value passed on the stack into the longword at offset four of the structure held in a register.
- `update_linked_object_list`: Walks a linked list of object records.
- `update_linked_objects_unrolled`: Performs the same linked-list update as its neighbour, with the type-one and type-two branches written as jump-based code.
- `update_linked_objects_with_table`: Walks the same kind of object list, copying byte codes from a table for type-one records and computing byte codes from coordinates for type-two records.
- `dequeue_ring_buffer_byte`: Returns the next queued byte from a 128-entry ring buffer and advances the read index.
- `advance_generator_and_mix`: Multiplies the stored 16-bit state by a constant and adds one, stores the result as the new state, and returns a rotated and masked view of that value.
- `mask_parameter_with_routine`: Calls a routine and returns its result combined with the parameter by bitwise AND.
- `return_routine_low_bit`: Calls a routine and returns only its lowest bit.
- `return_routine_low_two_bits`: Calls a routine and returns only the lowest two bits of its result.
- `return_routine_low_three_bits`: Calls a routine and returns only the lowest three bits of its result.
- `return_routine_low_six_bits`: Calls a routine and returns only the lowest six bits of its result.
- `return_routine_low_byte`: Calls a routine and returns its low byte.
- `modulo_routine_result_by_parameter`: Calls a routine and returns the remainder of its low 16 bits divided by the low 16 bits of the parameter, combined with the parameter unchanged.
- `call_routine_then_library_slot`: Calls a routine, then calls the library function at a fixed offset from a stored library base, and returns its two input registers unchanged.
- `format_decimal_into_buffer`: Writes the value in the first register as decimal digits into a ten-character buffer, right-aligned, and fills the leading positions with spaces.
- `format_decimal_with_width`: Writes the 16-bit value as decimal digits into a buffer ending at the index held in the second register, moving left one character per digit.
- `fill_spaces_to_index`: Writes space characters into a buffer from the index in a register down to position zero inclusive.
- `run_nested_routine_loops`: Runs three nested loops with counts of 17, 16 and 5.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `process_text_segments`: Walks a null-terminated string.
- `sample_tile_nibble_at_position`: Computes a bit mask from the two input values and looks up a byte in a map using coordinates offset by stored values.
- `sample_map_nibble_from_registers`: Performs the same map sampling as its neighbour, taking its two inputs from registers instead of arguments.
- `call_routine_and_decrement_counter`: Calls a routine through a function pointer stored in shared state, then decrements a stored 32-bit counter by four.
- `call_routine_and_subtract_counter`: Calls the same routine through the function pointer, then subtracts from a stored 32-bit counter the sum of another stored value and four.
- `call_sequence_return_registers`: Calls one routine with a parameter loaded from a global variable, then calls the same routine many more times with no arguments.
- `call_routine_six_times`: Calls the same routine six times in a row and returns, without touching any data itself.
- `copy_words_to_five_buffers`: Loops sixteen times, copying pairs of 16-bit words from a source block into five separate destination buffers at fixed offsets.
- `return_immediately`: Contains only a return and performs no work.
- `call_once_and_return`: Calls one routine with an argument read from a global variable and returns the two initial data registers combined as one 64-bit value.
- `call_routine_repeatedly_with_arg`: Calls one routine once with an argument read from a global variable, then calls the same routine many more times with no arguments.
- `call_routine_and_return`: Calls one other routine and returns, performing no data manipulation of its own.
- `compute_tile_indices_and_fill`: Derives two tile coordinates from its parameters, offsets them by two global values and clamps them to fixed limits, then stores the results in globals.
- `decode_tile_bits`: Reads five table pointers from a global, then for each of eight blocks gathers one flag bit per source byte into five 16-bit words.
- `copy_tile_words`: Copies eight groups of four 16-bit words from a source table into a destination table at a stride computed from its parameters.
- `compute_tile_index`: Offsets two coordinate inputs by global origin values, wraps each into a fixed range, and packs their low bits with a table lookup into one 16-bit index.
- `fill_tile_buffer`: Fills a buffer of nineteen rows by six words, each taking a byte from a lookup table indexed by offset coordinates.
- `copy_decompressed_bytes`: Copies bytes from a global source pointer to a destination, expanding negative control bytes into repeated runs when a mode flag is set.
- `scale_value_by_table`: When the table index is below eight, halves a scaled value taken from a global lookup table and adds it to the first argument, keeping the low fifteen bits.
- `scale_value_with_sign`: When the table index is below eight, scales the third argument by a global table value and adds it to the first argument.
- `copy_bit14_to_bit15`: Copies bit 14 of its argument into bit 15, setting bit 15 when bit 14 is set and clearing it otherwise.
- `clear_top_bit_of_word`: Returns the 16-bit value in the data register with its top bit cleared.
- `step_stored_coordinates_toward_target`: Compares two position arguments with stored globals and moves each stored value by one step when the difference is inside a band.
- `step_stored_coordinates_from_registers`: Reads two offset values from registers and moves two stored globals toward them, using the same banded step and snap rules as its stack-argument counterpart.
- `nudge_stored_coordinates_by_one`: Adjusts two stored globals by one step each when their differences from register inputs exceed fixed thresholds.
- `bump_x_and_step_y`: Adds one to a stored value when a register input is above a threshold.
- `step_stored_y_coordinate`: Moves a stored value by one step toward a register input when that input lies outside a small band.
- `nudge_stored_y_coordinate`: Adds or subtracts one from a stored value depending on which side of a small threshold a register input lies.
- `increment_counter_if_large`: The routine checks an input word and, if it is greater than ten, increments a short value stored in a global variable.
- `handle_error_message`: When its argument is not two, makes three calls: two with constant arguments and one passing the argument back with the value two.
- `classify_value_into_range_index`: Looks up a byte in a table indexed by the argument, then overrides it with a small index when the argument falls inside a chain of ranges.
- `decode_joystick_direction_index`: Reads the joystick port direction bits and maps them through a lookup table to a direction index, or derives the index from a stored position when a mode flag is set.
- `select_direction_index_and_notify`: Chooses a direction index from the joystick port bits through a lookup table, or from a stored position when a mode flag is set.
- `read_joystick_direction_index`: Reads the joystick port direction bits, maps them through a lookup table to a direction index, and updates a stored flag and countdown.
- `decrement_index_and_notify_change`: Subtracts twenty from a stored value, forcing it to nine and resetting a flag when the result falls outside zero to nine.
- `reset_index_to_nine`: Clears a stored flag word, then sets a stored index to nine and calls another routine with nine, but only when the index was not already nine.
- `sample_pixel_from_map_2`: Calls another routine and, if its short result is not one and is below ten, calls it again.
- `invert_and_combine_five_planes`: Walks a rectangle of 16-bit words and stores, at each position, the inverted AND of five source planes.
- `setup_blitter_copy`: Waits for the blitter through the system library, then programs four blitter jobs in a loop.
- `set_blitter_regs`: Writes a fixed control word and clears the modulo and shift registers of the blitter.
- `configure_blitter_mask_entries`: Waits for the blitter through the system library, then programs four blitter entries.
- `configure_blitter_single_job`: If the blitter is busy, waits for it through the system library.
- `setup_blitter_loop`: Waits for the blitter through the system library when it is busy, then programs four blitter jobs in a loop.
- `setup_blitter_loop_alt`: Waits for the blitter through the system library when it is busy, then programs four blitter jobs in a loop.
- `call_indirect_from_table`: The routine calls an indirect function pointer retrieved from a table.
- `dispatch_indirect_call_slot`: Makes an indirect call through a code pointer at a fixed offset from the base register, because the jump table could not be recovered.
- `compute_direction_toward_target`: Computes signed x and y distances between a record and a target, with limits that depend on a mode argument.
- `steer_direction_toward_target`: Computes signed x and y distances between a record and a target, applying mode-dependent limits and sign handling.
- `steer_direction_from_offsets`: Takes x and y offsets from registers, negates the x offset, and derives a direction from their signs and magnitudes.
- `steer_direction_with_mode_offsets`: Derives a direction from register offsets using mode-dependent comparisons on their magnitudes.
- `store_adjusted_direction_index`: Calls another routine and negates a direction value when one bit of its result is clear.
- `store_direction_difference`: Stores the difference of two register values, masked to three bits, into a byte of a record.
- `countdown_dispatch_with_fallback`: When a state byte is below four and one of two table entries matches, calls a routine with a fixed code and stops.
- `subtract_offset_and_dispatch`: Checks a state byte, then subtracts a register offset from a stored counter clamped at zero.
- `subtract_offset_and_dispatch_group`: Subtracts a register offset from a stored counter clamped at zero, then calls routines with offset-based arguments chosen by four branches.
- `dispatch_three_way_branch`: Calls one of three routines with offset-based arguments, chosen by two register values.
- `dispatch_two_way_branch`: Calls one of two routines with offset-based arguments, chosen by a register value.
- `dispatch_fixed_branch_and_notify`: Always calls one routine with a fixed code and an offset-based argument.
- `count_matching_records_and_notify`: Counts records of one type whose sub-type is fifteen or five, then reports the counts through a routine when a stored value falls in a range.
- `tally_typed_records_and_report`: Walks all records and counts those of one type with sub-type fifteen and those with sub-type five.
- `tally_subtype_records_and_report`: Walks records counting those whose sub-type byte is fifteen and those whose sub-type byte is five, then reports the counts when a stored value is positive and within a range.
- `tally_mixed_records_and_report`: Counts records whose sub-type is five and, for records of one type, those whose sub-type is fifteen.
- `display_outcome_text_by_counts`: Displays a fixed text message when a stored value is below five and the first count is non-zero.
- `display_battle_result_message`: When a stored value is below fifty, displays one message for each non-zero count, with a numeric display call between them.
- `detect_nearby_record_collision`: Returns a nonzero code when another suitable record lies within a small rectangle around a given point.
- `detect_actor_proximity_collision`: Scans records from index zero, skipping the actor's own index, index one, and records of two excluded kinds.
- `detect_actor_proximity_from_index`: Scans records starting from an index passed in a register, skipping the actor's own index, index one, and records of two excluded kinds.
- `scan_for_nearby_record_match`: Walks the record list from an index, skipping the actor's own index, index one, and records of two excluded kinds.
- `find_closest_record_index`: Scans records from index one, measuring each with a distance routine and keeping the smallest value.
- `track_minimum_distance_in_record`: Scans indices from a start value, measuring each through a distance routine and keeping the smallest result in a field of the given record.
- `measure_offset_with_diagonal_check`: Measures the absolute x and y differences between two positions.
- `call_two_routines_sequentially_2`: If a comparison between two values holds true, the routine calls two different indirect function pointers in succession.
- `call_two_routines_sequentially`: Calls two routines one after the other and returns, passing no arguments of its own.
- `try_move_record_to_position`: Computes two candidate coordinates for a record through two indirect calls.
- `store_record_position_pair`: Writes two register values into the position fields of the record selected by a field of the current record, then returns one.
- `draw_indexed_frame_at_offsets`: Reads four bytes from a table indexed by the argument, then issues a call with fixed size arguments and a position from a global pointer.
- `fill_tile_map_entries`: Chooses a base value from a global mode setting and clamps a level argument to the range zero to one hundred, flooring it or raising it to twenty-five depending on a flag.
- `randomize_tile_map`: Repeats thirty-two times: fills a table of thirty-one words with values from a routine called with an upper bound of four thousand ninety-five, then passes the table to another routine and calls a third routine with one.
- `process_next_queued_event`: When the event queue is not empty, reads the next queued code, advances the queue index circularly, and dispatches on that code.
- `dispatch_next_queued_code`: Reads the next queued code, advances the queue index circularly, and dispatches on that code.
- `dispatch_queued_command`: The routine uses a switch statement based on an input value to execute various sequences of indirect function calls and data manipulations.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `store_input_byte`: Stores the byte argument into a circular queue of thirty-two entries unless the queue is full.
- `print_zero_terminated_string`: Measures a zero-terminated string, passes fixed layout values and a text pointer to one routine, then passes the string with its length to another routine.
- `initialize_display_setup`: Makes three setup calls in order: one with a global pointer and six layout constants, one with two small constants, and one with no arguments.
- `pass_string_and_length`: Measures a zero-terminated string and passes it to one routine together with its length.
- `wrap_text_into_lines`: Copies a text string into a line buffer, wrapping lines at about thirty-eight characters.
- `wrap_text_with_line_flush`: A variant of the text wrapper that copies characters into a line buffer, expands percent signs from a table, and flushes each finished line through another routine once it passes a width near thirty-eight characters.
- `resume_text_wrapping_loop`: Resumes the text wrapping loop partway through a line, handling any pending percent-sign expansion before copying characters into the line buffer.
- `continue_text_wrapping`: Advances the line-wrapping state by one character and copies characters into the line buffer.
- `append_text_char_and_wrap`: Writes the current character into the output buffer, then advances the wrapping loop.
- `terminate_and_flush_text_line`: Ends the current output line with a zero byte, passes it to another routine, and resets the line counter.
- `stub_return_only`: Contains only a return and performs no work.
- `skip_strings_then_wrap_text`: Advances past a count of zero-terminated strings given as an argument, then copies the following text into a line buffer.
- `skip_strings_wrap_second_block`: Like its counterpart, skips a count of zero-terminated strings and wraps the following text, but it reads from a different text block.
- `wrap_text_after_skipping`: Skips a given number of zero-terminated strings from a text pointer argument, then copies the remaining text into a line buffer.
- `call_other_routine_four_times`: Calls one other routine four times in succession.
- `call_other_routine_three_times`: Calls one routine three times, passing in turn a table entry chosen by a global index, a global pointer, and the parameter.
- `call_other_routine_once`: Calls one routine once with an argument read from a table entry chosen by a global index.
- `initialize_buffer_and_state_fields`: Points a global pointer at a buffer and copies a value through an indirect pointer into another global.
- `initialize_tile_and_flag`: Calls one routine, points a global pointer at a fixed buffer, and writes a constant word to a global.
- `loop_decrementing_tile_fill`: Loops over counter values from one hundred down to zero in steps of five.
- `loop_incrementing_tile_fill`: Loops over counter values from zero up to one hundred in steps of five.
- `clear_two_word_fields`: Stores zero into two 16-bit fields of a record reached through a global pointer, then calls another routine.
- `copy_and_prepare_structures`: Copies one global pointer into another and calls a routine several times with constant and table-derived arguments.
- `populate_struct_from_table`: Reads several values through an indirect handler and stores a doubled short and a pointer into global arrays.
- `copy_struct_and_update_state`: Points a global at one entry of a record array selected by an argument, calling a handler first if its state field equals two.
- `copy_block_and_invoke_handler`: Copies fourteen longwords from a source pointed to by a global into a local buffer, clears a longword and sets a word to nine in globals, then calls a handler with the buffer's address.
- `forward_call_to_routine`: Calls one other routine and returns, performing no other work.
- `call_with_indexed_global_args`: Makes an indirect call with a fixed buffer address and clears a 16-bit global.
- `conditional_display_update`: Calls an indirect check on the current record when its state field equals two.
- `read_chunked_data_offsets`: Opens a named file through a handler, then reads four-byte size headers and the data each describes into a buffer, recording each chunk's offset in a table.
- `setup_display_area_and_draw`: When a mode flag is clear, calls a routine with a fixed value and configures a 320 by 200 display area from fixed arguments.
- `loop_over_table_and_blit`: Loops over twenty-two table entries, copying bitmap rectangles through an indirect routine with positions derived from the table values.
- `check_key_and_set_flag`: Calls a handler to obtain a value, stores whether it equals thirty-two into a global flag byte, and returns that boolean.
- `relocate_buffer_above_threshold`: When the pointer argument exceeds a fixed address threshold, allocates a new buffer through a system call and copies the given number of bytes into it.
- `compute_orientation_flags_and_draw`: Computes two signed cross products from four points derived from a parameter record and a global offset table, then sets a flag for each result that is non-negative.
- `set_state_and_call_tables`: Writes a constant two to a global state word, makes two calls with arguments taken from global tables indexed by a counter, and copies the state word into another global when it exceeds three.
- `store_entry_from_table`: Copies the x coordinate of a table entry chosen by a parameter into a global word, and its y coordinate plus ten into another.
- `set_entry_flag_or_mark`: Looks up an index through a handler and checks a flag bit in a table of records to choose an entry.
- `store_or_mark_entry_byte`: Adds a handler-supplied index to a table base chosen by a global index.
- `store_byte_to_offset`: Writes the low byte of a data register into the byte five positions past the address register.
- `spawn_entries_and_scan_list`: Resets two global flags, and on the first pass for a given state spawns ten entries at random positions with types taken from a table.
- `spawn_always_and_scan_list`: Always spawns ten entries at random positions with types taken from a table, then sets a global state word.
- `scan_entries_into_active_table`: Walks a list of entries, checking each against a window around a reference position.
- `advance_entry_and_test_window`: Advances the entry pointer and counter, stopping when the counter runs out or the active table is full.
- `update_matching_entries_in_window`: Checks each entry against a window around the reference position.
- `append_window_entry_and_advance`: Fills a new active-table record for a kind-three or kind-four entry that does not match an existing one, then advances the walk.
- `store_window_entry_and_advance`: Finishes an active-table record for a window entry by setting its kind to a fixed value and copying its position, then advances the walk.
- `append_plain_entry_and_advance`: Fills a new active-table record for an entry of a kind other than three or four, copying its position and flags, then advances the walk.
- `store_entry_kind_and_advance`: Sets the kind byte of a new active-table record to one or zero depending on whether the source kind is five, copies its position, and advances the walk through the entry list.
- `fill_entry_record_and_advance`: Fills an active-table record for an entry whose kind is not three or four, setting its kind from a flag, then advances the walk through the entry list.
- `return_without_action`: Contains only a return and does nothing.
- `read_typed_text_and_match`: Shows a prompt and reads keystrokes into a small buffer, accepting printable characters up to nine and handling backspace until return is pressed.
- `call_library_cleanup_sequence`: Calls a fixed sequence of library routines through the system base and through a pointer loaded from a global, reusing the same two offsets on each pointer.
- `store_and_dispatch_result`: Calls another routine with two arguments and stores the result in a global.
- `store_result_with_table_check`: Calls a routine with a global pointer and a fixed negative argument, storing the result in a global.
- `return_from_subroutine`: This routine is empty and contains no executable instructions.
- `poll_flag_with_timeout`: Polls a flag byte up to 300 times, calling a routine with a delay argument of five between checks.
- `wait_flag_with_delay_loop`: Repeatedly checks a flag byte with a short delay call between checks, up to 300 times.
- `wait_for_flag_then_clear`: Calls a delay routine repeatedly until a flag byte is set, giving up after 300 checks.
- `transfer_state_blocks_with_file`: Builds a file name from a slot letter and opens one of two named files.
- `select_slot_and_transfer_state`: Waits for a slot choice from input, builds a file name from it, and opens the matching file.
- `select_slot_report_disk_errors`: Repeats the slot choice and file transfer sequence, and references text for an aborted operation and a request to insert a writable disk.
- `poll_slot_choice_and_transfer`: Polls for a slot choice from input, and on a choice builds a file name, opens the file, and transfers the state blocks.
- `transfer_slot_blocks_and_close`: Builds a file name from a slot letter, opens the matching file, and transfers the state blocks through it.
- `transfer_state_blocks_fixed_file`: Opens one fixed named file and transfers the state blocks through it.
- `report_save_failure_and_finish`: Shows a message through a routine when the transfer result is non-zero, including text for a failed save.
- `report_load_failure_and_finish`: Displays a load-failure message, waits for an input event, then resets state fields and calls a routine with the display handle.
- `prompt_game_disk_and_wait`: Displays a prompt asking for the game disk, waits until an input event arrives, then resets state fields and calls a routine with the display handle.
- `initialize_display_state`: When a flag byte is clear, resets several globals to zero and calls routines with constant and address arguments.
- `dispatch_flagged_call_with_args`: Calls a routine through a handler pointer or directly, depending on a flag, passing two arguments.
- `call_handler_and_check_result`: Calls a handler through a global pointer with two arguments and stores the returned word.
- `write_bounds_to_table`: Writes four 16-bit values into a table entry selected by the first argument.
- `update_display_state`: Refreshes display state through a series of routine and indirect calls, then updates a counter and a global offset.
- `run_timed_buffer_fill_loop`: Makes setup calls, then runs a timed loop of twenty-five steps.
- `continue_timed_buffer_fill_loop`: Continues the timed loop of buffer fills, refilling a 28-entry buffer from a source table offset by the step number and sending it through a routine at each step.
- `restart_timed_buffer_fill_loop`: Restarts the timed buffer loop from a fresh pair of values, refilling the buffer from the source table at each step and pausing between steps.
- `return_conditional_size`: Returns ten when a byte at the argument's offset in a global block is non-zero, and eight otherwise.
- `compute_and_draw_position`: Computes coordinates from a global position and a flag-dependent offset, then passes them to a routine.
- `draw_default_position_or_fixed`: Returns the low bits of a global flag word unless those bits are zero or a byte exceeds a threshold.
- `call_fill_tile_map`: Calls one routine with three constant values of one hundred, the constant one, and a fixed buffer address, then returns.
- `dispatch_entity_action_by_code`: Looks up an entity record and updates its fields for the requested action code.
- `dispatch_entity_action_from_register`: Handles the same action codes as the entity dispatcher, but takes the code from a register and looks up the entity record without setting its position fields first.
- `apply_entity_action_fragment`: A fragment of an entity action dispatcher with several branches removed as unreachable.
- `dispatch_entity_action_with_flag`: Applies the action-code updates for an entity, with a flag register deciding whether each action fires.
- `execute_entity_action`: The routine calls another routine.
- `dispatch_entity_action_partial`: Applies action codes four, two, six and ten, plus the call-only codes three and five, to an entity whose flag is set.
- `dispatch_entity_action_calls_only`: Handles action codes three, five, two, six and ten for an entity.
- `dispatch_entity_action_minimal`: Handles action codes five, two, six and ten for an entity.
- `dispatch_codes_two_six_ten`: Handles action codes two, six and ten for an entity.
- `dispatch_entity_codes_six_ten`: Handles action codes six and ten.
- `execute_conditional_action`: If a value is non-zero, the routine calls an indirect function, retrieves a value from a table using the result, and then calls another routine.
- `set_sprite_flag`: When a register equals ten, calls another routine if a second register is non-zero, then stores twelve into a byte of the record.
- `decrement_counter_and_notify`: Subtracts an argument from a stored counter.
- `call_self_as_argument`: Calls another routine with its own address as the argument and returns.
- `adjust_entry_and_set_positions`: Decrements a 16-bit field of an array entry by four.
- `update_two_word_state`: Calls two routines to obtain a value and an offset, then stores the two words returned by further routine calls into globals.
- `scan_matched_record_grid`: Opens a named file and, if it succeeds, walks a list of records looking for one whose identifier matches a stored value.
- `process_script_state`: Clears a flag byte in a structure and adds a 16-bit step value to a 32-bit position field.
- `run_music_script_step`: Reads command bytes from a channel's sound script.
- `decrement_counter_and_update_state`: Sets two channel fields to a constant, then decrements a byte counter in a second structure when it is non-zero.
- `initialize_audio_and_dma`: Sets two audio state bytes, then writes the sample pointer, length, period and volume registers of audio channel one.
- `silence_audio_channel_one`: Clears the low bit of a channel flag byte, then sets audio channel one's volume to zero and its period to two.
- `store_four_parameters`: Stores its four arguments into four 32-bit global variables at fixed offsets.
- `initialize_audio_channels_and_state`: Stores four parameters into paired globals, clears the volume registers of four audio channels and copies four words from a table into globals.
- `reset_audio_volumes_and_flags`: Sets the volume of all four audio channels to zero, turns off their DMA, and sets a global flag byte to 255.
- `setup_interrupt_handlers`: Stores three parameters into globals and prepares two interrupt-server structures that point at a handler.
- `remove_interrupt_handler_and_disable`: Calls a routine, then removes an interrupt server through the system library.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_memory_and_init_globals`: Computes a global base offset and stores a derived address into a global.
- `stub_return_gap`: Contains only a return and performs no work.
- `parse_quoted_tokens_from_text`: Opens a system library and, if that succeeds, splits a text argument into tokens using a character-class table, with support for quoted strings.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `copy_string_pad_zeros`: Copies a string into a fixed-size destination, stopping at its terminator or when the count runs out, then fills the remaining space with zeros.
- `append_string_with_cap`: Appends a source string to the end of a destination string, copying at most about thirty-two thousand characters.
- `copy_string_with_limit`: Appends a source string to the end of a destination string, copying at most the given count of characters.
- `copy_string_to_buffer`: Copies a zero-terminated string from a source to a destination, including the terminator, and returns the destination pointer.
- `multiply_32_bit_values`: Computes the low 32 bits of the product of two 32-bit register values from their 16-bit halves.
- `perform_signed_division`: Calls another routine and negates its result when exactly one of the two inputs is negative, so the sign is handled outside that routine.
- `negate_result_when_first_negative`: Calls another routine, then negates the value it leaves in a register when the first argument is negative.
- `call_and_return_concatenated`: Calls another routine and returns the original second register paired with the register value the call leaves behind.
- `call_self_recursively`: Calls itself unconditionally and then returns, with no other work, so every entry leads straight into another entry.
- `divide_unsigned_32bit`: Divides a 32-bit dividend by a 32-bit divisor and returns the quotient and the remainder packed into one 64-bit value.
- `compute_string_length`: Walks a zero-terminated byte string forward until it reaches the terminator and returns the number of bytes before it.
- `invoke_optional_callback`: Calls through a function pointer held in a global slot when that pointer is non-zero, then calls one further unnamed routine with no argument.
- `run_twenty_step_dispatch`: Calls an unnamed routine twenty times in a row, then calls through a global handler slot if it is set.
- `release_indexed_slot`: Checks that the index lies between 0 and 19 and that its table entry is in use.
- `jump_library_vector_36`: Jumps to the library function at offset minus 36 from a library base held in a global variable, with no other logic.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `jump_library_vector_132`: Jumps to the library function at offset minus 132 from a library base held in a global variable, with no other logic.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `jump_library_vector_84`: Jumps to the library function at offset minus 84 from a library base held in a global variable, with no other logic.
- `jump_library_vector_30`: Jumps to the library function at offset minus 30 from a library base held in a global variable, with no other logic.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `jump_library_vector_60`: Jumps to the library function at offset minus 60 from a library base held in a global variable, with no other logic.
- `jump_library_vector_42`: Jumps to the library function at offset minus 42 from a library base held in a global variable, with no other logic.
- `jump_library_vector_66`: Jumps to the library function at offset minus 66 from a library base held in a global variable, with no other logic.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `jump_library_vector_90`: Jumps to the library function at offset minus 90 from a library base held in a global variable, with no other logic.
- `jump_library_vector_48`: Jumps to the library function at offset minus 48 from a library base held in a global variable, with no other logic.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `jump_library_vector_414`: Jumps to the library function at offset minus 414 from a library base held in a global variable, with no other logic.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_indexed_record`: Takes a free index from an unnamed routine and returns zero if none is available.
- `reset_record_state`: If the record's first field is set, passes the record to an unnamed routine.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `forward_call_with_constant`: Calls the function reached through a pointer in a global variable, passing the incoming argument and the constant 48.
- `forward_call_with_argument`: Calls the function reached through a pointer in a global variable, passing its single argument unchanged.
- `allocate_typed_record`: Returns zero when the first argument is zero.
- `reset_record_via_library`: Returns zero for a null pointer.
- `reset_record_header_fields`: Sets the byte at offset 8 of the record to 255 and the longwords at offsets 20 and 24 to minus one, then calls a library function with no arguments.
- `jump_library_vector_198`: Jumps to the library function at offset minus 198 from a library base held in a global variable, with no other logic.
- `call_library_vector_198`: Calls the library function at offset minus 198 from a library base held in a global variable, with no arguments of its own.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `jump_library_vector_294`: Jumps to the library function at offset minus 294 from a library base held in a global variable, with no other logic.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_indirect_via_library`: The routine calls an indirect function pointer located at an offset within a structure pointed to by a global variable.
- `jump_library_vector_210`: Jumps to the library function at offset minus 210 from a library base held in a global variable, with no other logic.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the block's own address in its first longword, adds four to that stored value, clears the following longword, and stores the original address in the one after.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `jump_library_vector_552`: Jumps to the library function at offset minus 552 from a library base held in a global variable, with no other logic.
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
- `call_library_vector_30`: Calls the library function at offset minus 30 from a library base held in a global variable, with no arguments of its own.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `jump_library_vector_462`: Jumps to the library function at offset minus 462 from a library base held in a global variable, with no other logic.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_via_offset`: The routine calls an indirect function pointer located at a specific offset within a structure pointed to by a global variable.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_call`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `jump_library_vector_468`: Jumps to the library function at offset minus 468 from a library base held in a global variable, with no other logic.
- `jump_library_vector_360`: Jumps to the library function at offset minus 360 from a library base held in a global variable, with no other logic.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `jump_library_vector_216`: Jumps to the library function at offset minus 216 from a library base held in a global variable, with no other logic.
- `jump_library_vector_240`: Jumps to the library function at offset minus 240 from a library base held in a global variable, with no other logic.
- `execute_indirect_call_2`: Calls a subroutine by dereferencing a pointer located at a specific offset within a table.
- `jump_library_vector_456`: Jumps to the library function at offset minus 456 from a library base held in a global variable, with no other logic.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `jump_library_vector_354`: Jumps to the library function at offset minus 354 from a library base held in a global variable, with no other logic.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Loads a pointer from a global variable, adds a fixed offset of minus 60 to it, and calls the resulting library function with no arguments.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_36`: Loads a function pointer from a global table, adds an offset of minus 36, and calls the library function it reaches with no arguments.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `jump_library_vector_150`: Jumps to the library function at offset minus 150 from a library base held in a global variable, with no other logic.
- `jump_library_vector_144`: Jumps to the library function at offset minus 144 from a library base held in a global variable, with no other logic.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_routine`: The routine contains no valid instructions.

### Faery Tale Adventure, The (1986)(MicroIllusions)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spin_in_infinite_loop`: Consists only of an empty loop with no exit, so control never leaves once it arrives here.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `empty_routine_2`: The routine is empty and contains no executable instructions.
- `enter_infinite_loop`: Enters an infinite loop that never exits.
- `nop_padding_sequence`: This routine is empty and consists of a gap in the executable containing non-instruction data.
- `nop_padding_sequence_8`: This routine is empty and consists of non-instruction data used as padding between functional code segments.
- `empty_routine_14`: The routine contains no valid instructions and performs no operations.
- `perform_infinite_loop_3`: The routine enters an infinite loop that prevents further execution of the program.
- `enter_infinite_loop_2`: Executes a continuous loop that prevents further code execution.
- `enter_infinite_loop_3`: This routine consists of an infinite loop and is unreached.
- `trap_invalid_instruction_stream`: The routine consists of a single decoded instruction that the disassembler could not translate, so decompilation terminates the flow at that point.
- `nop_padding_sequence_9`: This routine consists of a single instruction that serves as data padding within the executable segment.
- `enter_infinite_loop_4`: Performs a continuous loop that does not terminate.
- `nop_padding_segment`: This segment is empty and contains no executable instructions.
- `execute_infinite_loop_29`: The routine enters an endless loop that performs no operations.
- `nop_padding_segment_2`: This routine is empty and consists of a single non-operational instruction used for memory alignment or padding within the runtime segment.
- `execute_invalid_instruction`: The routine contains invalid opcode data that prevents successful disassembly and execution.
- `nop_padding_sequence_10`: This routine is empty and consists of a sequence of non-instruction data used to pad the executable segment.
- `execute_infinite_loop`: This routine enters an unconditional infinite loop that prevents further execution of the current thread.
- `nop_padding_sequence_2`: This routine is empty and consists of a single non-instruction byte that acts as padding between valid code segments.
- `empty_routine_3`: This routine contains no executable instructions.
- `nop_padding_sequence_3`: This routine is empty and consists of non-executable data used for alignment or padding within the segment.
- `empty_routine_4`: The routine contains no executable instructions.
- `empty_routine_5`: This routine is empty and performs no operations.
- `execute_infinite_loop_2`: The routine enters an infinite loop that prevents further execution of the program flow.
- `empty_routine_6`: This routine contains no valid instructions and performs no operations.
- `nop_padding_sequence_4`: This routine is empty and consists of an infinite loop of no-operation instructions.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `nop_padding_sequence_11`: This routine is empty and consists of non-executable data padding within the runtime segment.
- `null_operation_stub`: This routine is empty and contains no valid instructions.
- `nop_padding_sequence_5`: The routine is empty and consists of non-executable data used for alignment or padding within the executable segment.
- `execute_infinite_loop_3`: The routine enters an unconditional loop that repeats indefinitely without performing any operations or returning.
- `execute_infinite_loop_4`: Performs a continuous loop without exit.
- `contain_invalid_data`: Contains data that does not decode as valid instructions.
- `perform_infinite_loop`: This routine consists of an infinite loop that prevents further execution.
- `nop_instruction`: The routine contains a single instruction that does not perform any action.
- `contain_invalid_data_2`: Contains an instruction that cannot be processed.
- `contain_invalid_data_3`: Contains data that results in a bad instruction.
- `contain_invalid_data_4`: Consists of undecodable instruction data.
- `return_empty_routine_2`: The routine contains no executable instructions and performs no operations before returning.
- `empty_routine_12`: the routine contains no executable instructions.
- `enter_infinite_loop_5`: Performs an infinite loop that never exits.
- `null_instruction_placeholder`: The routine contains no valid executable instructions and terminates immediately.
- `execute_infinite_loop_5`: The routine enters an unconditional loop that prevents further execution of subsequent instructions.
- `empty_routine_9`: The routine contains no valid executable instructions.
- `execute_infinite_loop_6`: The routine enters an endless loop that prevents further execution of the program flow.
- `perform_infinite_loop_2`: The routine enters an infinite loop that prevents further execution.
- `empty_routine_10`: The routine is empty and performs no operations.
- `execute_infinite_loop_7`: The routine enters an infinite loop that repeatedly executes a single jump instruction to itself.
- `empty_routine_11`: This routine contains no valid instructions.
- `null_instruction_padding`: This routine is empty and contains no executable instructions, serving as a gap in the instruction stream.
- `null_routine`: This routine is empty and contains no valid executable instructions.
- `enter_infinite_loop_6`: Performs an endless loop without exiting.
- `execute_infinite_loop_8`: The routine enters an infinite loop that performs no operations.
- `execute_infinite_loop_9`: This routine enters an endless loop that performs no operations.
- `empty_routine_13`: the routine contains no executable instructions or valid operations.
- `execute_infinite_loop_10`: The routine enters an infinite loop that prevents further execution of subsequent instructions.
- `execute_infinite_loop_11`: This routine enters an infinite loop that performs no operations.
- `execute_infinite_loop_12`: The routine enters an unconditional loop that prevents further execution of the calling sequence.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `execute_infinite_loop_13`: This routine enters an infinite loop that prevents further execution of subsequent instructions.
- `execute_infinite_loop_14`: The routine enters an infinite loop that repeats a no-operation instruction indefinitely.
- `halt_execution`: The routine contains no valid instructions and terminates the instruction stream immediately upon entry.
- `nop_padding_sequence_6`: This routine is empty and consists of non-executable data used for alignment or padding within the runtime segment.
- `null_operation_stub_2`: The routine contains no valid executable instructions and performs no operations.
- `empty_routine_7`: The routine contains no executable instructions and performs no operations.
- `execute_infinite_loop_15`: This routine enters an infinite loop that prevents further execution of subsequent code.
- `null_operation_routine`: The routine is empty and contains no valid instructions.
- `execute_infinite_loop_16`: This routine performs an infinite loop that prevents further execution by repeatedly executing a jump instruction to itself.
- `execute_infinite_loop_17`: This routine enters an unconditional infinite loop that prevents further execution of the subsequent code.
- `execute_infinite_loop_18`: This routine enters an infinite loop that prevents further execution of the program flow.
- `return_empty_routine_3`: the routine is empty and contains no executable instructions.
- `terminate_execution`: stops the processor or enters an undefined state because the instruction or control flow is invalid.
- `execute_infinite_loop_19`: The routine enters an infinite loop that prevents further execution of the current thread.
- `empty_routine_8`: This routine contains no executable instructions and performs no operations.
- `nop_padding_sequence_7`: This routine is empty and consists of a single non-functional instruction used as padding within the executable segment.
- `execute_infinite_loop_20`: The routine enters an unconditional loop that repeats indefinitely without performing any operations.
- `execute_infinite_loop_21`: This routine enters an endless loop that prevents further instruction execution.
- `execute_infinite_loop_22`: This routine performs an infinite loop that prevents further execution of the program flow.
- `execute_infinite_loop_23`: The routine enters an endless loop that prevents further execution of the calling code.
- `execute_infinite_loop_24`: This routine enters an infinite loop that prevents further execution of the calling code.
- `execute_infinite_loop_25`: The routine enters an unconditional loop that prevents further execution.
- `nop_padding_gap`: This routine is empty and consists of non-executable data used as padding between functional code segments.
- `execute_infinite_loop_26`: This routine enters an unconditional infinite loop that prevents further execution of the calling code.
- `execute_infinite_loop_27`: This routine enters an infinite loop that never terminates.
- `execute_infinite_loop_28`: The routine enters an unconditional loop that prevents further execution of the calling code.
