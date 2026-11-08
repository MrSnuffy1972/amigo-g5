# Game 2194 (faery tale adventure, the (1986)(microillusions)[a5]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02194). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902096 top-level bytes in 23 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 447 of 447 named (0 author symbols), 447 of 447 described (66 by rule, 95 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2408 of 2408 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14643 of 14675 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 123458 |
| code | 49844 |
| data | 688916 |
| padding | 4550 |
| unused | 35328 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Type | 2 | 117 | 2 |
| Wait | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf:FMAIN | 3 | 13906 | 353 |
| 9 | 1 | 51 | 2 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 88 | 86 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EF8) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 59 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 32-136 (bytes 16384-70144) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 149-159 (bytes 76288-81920) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.69)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 164-256 (bytes 83968-131584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.96)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 258-311 (bytes 132096-159744) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.47)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 314-340 (bytes 160768-174592) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 342-342 (bytes 175104-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 344-393 (bytes 176128-201728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.98)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 396-415 (bytes 202752-212992) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.67)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 419-422 (bytes 214528-216576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 424-445 (bytes 217088-228352) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 447-453 (bytes 228864-232448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.20)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 455-461 (bytes 232960-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.34)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 471-471 (bytes 241152-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 473-473 (bytes 242176-242688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 477-477 (bytes 244224-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 479-535 (bytes 245248-274432) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.97)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 538-671 (bytes 275456-344064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.89)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 673-673 (bytes 344576-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 677-695 (bytes 346624-356352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.90)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.11)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 714-718 (bytes 365568-368128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 720-756 (bytes 368640-387584) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.15)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 760-782 (bytes 389120-400896) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.68)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 784-791 (bytes 401408-405504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 800-868 (bytes 409600-444928) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.57)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 896-917 (bytes 458752-470016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.01)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 920-955 (bytes 471040-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 960-1171 (bytes 491520-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 1312-1370 (bytes 671744-701952) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.24)
- faery tale adventure, the (1986)(microillusions)[a5]/Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf: blocks 1376-1501 (bytes 704512-769024) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_and_call`: When the stack argument is non-null, copies sixteen longwords from a fixed offset in the structure held in address register A2 into that argument, then calls a routine through a function pointer.

### Wait

- `update_linked_list_buffer`: Traverses a linked list to find a maximum value and then updates a structured buffer by writing a header and copying data from a stack-allocated source.
- `copy_memory_block`: Copies a fixed number of words from a source pointer to a destination pointer and then calls another routine.

### Disk-Validator

- `run_disk_validator_entry`: Program entry for the Disk-Validator command.

### Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf:FMAIN

- `clear_block_and_call_setup`: Program entry for FMAIN. It clears a large block of longwords starting at a computed offset, stores a register value and a global value in fixed slots, and then calls one other routine.
- `initialize_graphics_and_devices`: Opens graphics.library and layers.library, uses the Amber font at size nine, and refers to trackdisk.device.
- `release_flagged_resources`: Frees each resource whose flag bit is set in the status bytes by calling a memory-release vector with a fixed size for that resource.
- `index_length_prefixed_blocks`: Allocates a block through a library call, then walks a block of length-prefixed records: each longword gives the size of the record that follows it.
- `update_map_tile_from_table`: Probes the requested position and nearby offsets with a query routine, then searches a 17-entry table for a row matching the tile bytes and an argument flag.
- `setup_display_state`: Large setup routine that initialises the display and graphics state by calling many library vectors and routines, including buffer and screen setup.
- `update_position_and_advance_y`: Updates the stored x and y coordinates by the difference from their previous values and stores the new position.
- `update_region_code_from_position`: Looks up the current position in a table of rectangular regions with twelve-byte entries, and updates a stored region code when it changes.
- `update_code_from_lookup_table`: Adds a value from a helper call to a stored byte and derives a table index from a stored word.
- `compute_offset_target_coordinates`: Computes two target coordinates from the stored position and two helper results, adding a fixed offset of 150 to the second result, then passes them to two routines and stores the results in the stored target fields.
- `place_entry_near_position`: Tries up to fifteen nearby positions, offset by values from a routine, until a query routine reports an acceptable spot.
- `reset_expired_entry_state`: Resets a 22-byte record when its counter is not positive and its kind is not one of two reserved values, setting its state bytes and counter.
- `set_mode_and_reset_state`: Sets a mode byte to six when the argument is ten and to five otherwise.
- `initialize_scene_state`: Initialises a large block of state fields in the game data.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d7.
- `set_layout_fields_from_argument`: Calls two routines for values, then sets several layout fields from the argument and those values, using offsets from 160 and 100 and doubled values.
- `call_group_from_index_table`: Chooses an index into a table of four-longword groups according to a state value and a position range, then calls a routine with the four values of that group.
- `update_state_from_position_bits`: Derives a small state value from masked position bits and, when it changes, stores it and calls a routine.
- `swap_double_buffer_pointers`: Swaps two global buffer pointers, copies the first longword of the new buffer into one field and its second longword into another, then calls three library vectors on the fixed structures and restores the second longword.
- `open_input_device`: Creates a message port and an input request block, then opens input.device with that request.
- `close_input_device`: Sets the input request's command field to ten, calls library vectors to perform the request and close the device, and then releases the message port.
- `render_flagged_entries`: Calls a routine for each flagged entry in a state-specific table, up to twelve entries.
- `draw_entry_cell_for_state`: Chooses a cell value from a lookup that depends on the current state, then draws it at a position derived from the record, using a horizontal offset chosen by a parity bit.
- `handle_state_event_switch`: Dispatches on the current state value with a switch.
- `probe_until_code_five`: Calls another routine and queries a function at the stored coordinates up to twenty-five times, stopping when it returns five.
- `notify_coordinates_and_call_five`: Calls a function with one and the stored coordinates, then calls another routine with the value five.
- `set_state_when_flag_clear`: When bit one of a flag byte is clear, stores its argument into a state word, clears a second flag byte and calls another routine.
- `load_byte_lookup_tables`: Fills several small byte tables by calling a lookup routine with computed indices.
- `wait_until_counter_reaches_ten`: Calls another routine repeatedly while a stored counter is below ten, and returns once the counter reaches ten.
- `refresh_cached_entry_resources`: When a counter is below ten, compares an entry's stored fields with cached values.
- `call_indexed_entry_half_size`: When a flag bit is set, calls a routine with the pointer and half the size of the table entry chosen by the argument, together with the second argument.
- `free_fixed_buffers`: Frees three fixed 35-byte buffers and one 60-byte buffer through a library call, and sets a pointer from a table entry chosen by an index.
- `store_argument_in_block`: Stores its stack argument as a longword at offset four of the block in address register A1, then returns.
- `process_event_list_entries`: Walks a linked list of event entries.
- `walk_event_list_type_two`: Variant entry of the event-list walker that starts at the type-two check.
- `process_event_list_with_callback`: Walks the same kind of event list as its neighbours, clamping positions and writing type-one entries to the ring buffer.
- `read_next_ring_buffer_byte`: Returns the next byte from a 128-entry ring buffer when the read and write indices differ, and advances the read index modulo 128.
- `next_pseudo_random_value`: Advances a 16-bit state word with a multiply-and-add step, stores the new state and returns a value formed from its bits.
- `mask_result_with_argument`: Calls another routine for a value and returns its bitwise AND with the argument.
- `mask_result_low_bit`: Calls another routine for a value and returns only its lowest bit.
- `mask_result_low_two_bits`: Calls another routine for a value and returns its two lowest bits.
- `mask_result_low_three_bits`: Calls another routine for a value and returns its three lowest bits.
- `mask_result_low_six_bits`: Calls another routine for a value and returns its six lowest bits.
- `forward_result_byte`: Calls another routine and returns its result as a byte.
- `modulo_result_by_argument`: Calls another routine and returns the remainder of its low 16 bits divided by the low 16 bits of the argument.
- `call_then_library_vector_60`: Calls another routine, then calls a library vector at a fixed offset from the library base stored in a global slot.
- `format_decimal_right_aligned`: Writes the value in D0 as decimal digits into a text buffer, starting at the right end and moving left until the value reaches zero.
- `format_decimal_fixed_width`: Writes a value as decimal digits into a field whose width is given in D1, filling from the right.
- `fill_spaces_from_index`: Writes space characters into a buffer from the index in D1 down to zero.
- `run_nested_callback_loops`: Counts down through three nested loops, calling a routine from a function-pointer table on each pass.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `print_text_segments`: Walks a zero-terminated string.
- `lookup_tile_attribute_nibble`: Computes a tile position from two coordinate arguments, clamped against stored offsets, and looks up a map byte and then a flag table entry.
- `lookup_tile_attribute_nibble_regs`: Register-argument variant of the tile attribute lookup.
- `call_library_vector_42_decrement`: Calls a library vector at a fixed offset from a library base stored in a global slot, then subtracts four from a stored counter.
- `call_library_vector_42_adjust`: Calls a library vector at a fixed offset from a library base stored in a global slot, then subtracts a value derived from a stored pointer plus four from a stored counter.
- `call_routine_sequence_with_global`: Calls another routine once with a value from a global slot, then calls the routine many more times in succession with no arguments.
- `call_routine_six_times`: Calls another routine six times in succession and returns.
- `copy_block_to_multiple_buffers`: Copies 16-bit words from a source block into five destination buffers, moving two words per buffer on each of sixteen iterations with a fixed stride between destinations.
- `stub_return_only`: Routine consisting only of a return; the listing shows no calls, memory access or register use.
- `call_global_arg_return_regs`: Calls another routine with a value from a global slot and returns the input register values unchanged.
- `call_routine_run_with_global`: Calls another routine with a value from a global slot, then calls the same routine many more times with no arguments.
- `forward_call_once`: Calls another routine once and returns.
- `iterate_map_cell_grid`: Clamps two coordinates to fixed limits, then loops over nine rows and eighteen columns.
- `convert_chunky_pixels_to_bitplanes`: Converts indexed pixel bytes into five planar bit words.
- `copy_word_block_to_bitmap`: Copies a block of eight rows, each holding four 16-bit words, from a source buffer into a destination bitmap.
- `compute_tile_image_address`: Converts a position into map coordinates relative to a stored origin and clamps them.
- `fill_tile_window_buffer`: Fills a buffer of nineteen rows and six columns with values read from a tile map, using position-derived coordinates.
- `decode_run_length_stream`: When a flag byte is set, decodes a run-length stream into the output buffer until a stored count is reached: a negative count repeats the next byte, while a positive count copies that many bytes.
- `scale_value_by_table`: When the second argument's low byte is below eight, multiplies the third argument by a word from a global table indexed by that byte, halves the product, adds it to the first argument and masks the result to fifteen bits.
- `scale_value_with_sign`: When the second argument's low byte is below eight, adds half of the third argument times a global table word to the low fifteen bits of the first argument and keeps its sign bit.
- `copy_bit14_to_sign_bit`: Sets bit fifteen of the argument when bit fourteen is set, otherwise clears bit fifteen, and returns the value.
- `mask_low_fifteen_bits`: Returns the value in D0 masked to its lower fifteen bits.
- `follow_target_position`: Moves a stored x and y position toward target coordinates on each call.
- `step_position_from_deltas`: Adjusts a stored x and y position from delta values in registers D2 and D3.
- `step_position_without_x_snap`: Steps a stored x position by one when delta D2 exceeds the threshold, without any snap.
- `step_x_and_adjust_y`: Increments a stored x position when delta D2 exceeds twenty, and adjusts the stored y position from delta D3 with the same snap-and-step thresholds as its neighbours.
- `adjust_y_position_from_delta`: Adjusts the stored y position from delta D3.
- `step_y_by_delta_threshold`: Steps the stored y position down by one when delta D3 is below minus ten, or up by one when it is above ten.
- `increment_y_when_delta_positive`: Increments the stored y position by one when delta D3 exceeds ten.
- `report_error_when_not_two`: When the argument is not two, calls three routines with constant arguments, a global pointer and the argument.
- `select_value_by_range_checks`: Reads an initial byte from a table indexed by the argument.
- `read_joystick_direction_value`: Decodes the movement bits of the joystick port 1 data register into a direction index.
- `update_joystick_state`: Reads joystick port data to determine a direction index, updates a stored value, and calls another routine if the index has changed.
- `read_joystick_direction_index`: Decodes the movement bits of the joystick port 1 data register into a direction index, and stores it and calls another routine when it changes.
- `clamp_level_and_notify`: Subtracts twenty from a stored value.
- `set_level_nine_and_notify`: Resets a stored counter to zero and, when the stored level is not nine, sets it to nine and calls another routine with nine.
- `check_two_results_thresholds`: Calls another routine and, when its short result is not one and is below ten, calls it again.
- `compute_inverted_plane_and`: Loops over rows of words, writing to the destination the bitwise NOT of the AND of five source words taken from five rows spaced by a stride.
- `program_blitter_five_entries`: Takes ownership of the blitter.
- `set_blitter_regs`: Writes fixed control values to three blitter registers, then stores the first argument in the destination pointer register and the second argument in the size register.
- `program_blitter_five_planes_masked`: For each of five entries, waits for the blitter when it is busy, then sets its control, mask, source, destination and modulo registers from stored values with a shift-derived mask, and stores the computed pointers.
- `program_blitter_single_entry`: Programs the blitter for one copy without a loop.
- `program_blitter_five_steps_dest`: For each of five entries, waits for the blitter when it is busy, then sets its control value, masks and pointers from stored fields.
- `program_blitter_five_steps_src`: For each of five entries, waits for the blitter when it is busy, then sets its control value, masks and pointers from stored fields.
- `call_global_pointer_no_args`: Calls the routine whose address is stored in a fixed global slot, with no arguments, and returns.
- `call_second_global_pointer`: Calls the routine whose address is stored in a second fixed global slot, with no arguments, and returns.
- `compute_direction_toward_entry`: Computes signed x and y differences from a stored entry position and converts them to direction signs.
- `compute_direction_toward_entry_plain`: Computes signed x and y differences from a stored entry position and converts them to direction signs.
- `store_direction_from_signs`: Negates a sign value depending on a status bit from another routine, adds it to a direction value, and stores the sum masked to three bits in the entry.
- `store_direction_signs_mode`: Computes sign flags from two coordinate differences and a mode value, looks up a direction code in a table and stores it in the entry masked to three bits when the code is not special.
- `store_adjusted_direction`: Calls another routine and negates a direction value when a status bit is clear, then stores the sum of two direction values masked to three bits in the entry.
- `store_byte_difference`: Stores the masked difference of two register bytes into the entry's direction byte, and sets the next byte to twelve unless the mode is five.
- `update_entry_timer_and_target`: Subtracts a step from a record's timer and clamps it at zero.
- `update_timer_and_target_alt`: Variant of the entry timer update.
- `decay_actor_value_and_dispatch`: Subtracts a field of a second record from a running counter, clamps it to zero when it goes negative, then selects one of four dispatch calls based on two selector values and runs a follow-up check of table flags.
- `dispatch_actor_action_variant_b`: Selects between two indirect dispatch calls and a fallback call based on two selector values, then runs the same table-flag follow-up check as its neighbours.
- `dispatch_actor_action_fallback`: When the second selector is zero it issues a dispatch with a fixed code and offset, otherwise a second dispatch, then runs the table-flag follow-up check.
- `dispatch_actor_action_default`: Always issues one indirect dispatch with a fixed code, then runs the table-flag follow-up check that conditionally calls a further routine.
- `count_foes_and_announce_outcome`: Loops over the actor table counting entries whose type is two and whose subtype is fifteen or five, then shows the matching outcome text and calls a sound or notify routine with the counts when the group is non-empty and a global value is below fifty.
- `count_foes_by_kind_in_loop`: Counts actor-table entries of type two with subtype fifteen or five in a loop bounded by a stored actor count, then reports the counts through the same message and notify calls, gated on a global threshold.
- `tally_foe_kinds_and_report`: Scans actor-table entries for type two with subtype fifteen or five, counting each kind, and reports non-zero counts with message text and a notify call when a global value is below fifty.
- `tally_foe_kinds_with_flags`: Walks actor entries counting type-two records with subtype fifteen or five, and reports both counts through message text and a notify call once the walk reaches the stored count.
- `report_outcome_with_bravely_text`: Shows the text 'Bravely done!' and a notify message for the counted foes when the health value is under five and a count is non-zero, otherwise only when a global value is below fifty; repeats a final check on a global flag.
- `show_foe_outcome_messages`: Displays the text 'foes fled in retreat.' or 'foes were defeated in battle.' with a notify call for each non-zero count, gated on a global threshold, then runs a final global-flag check.
- `find_actor_near_point`: Checks other live actors from the table, skipping the actor at the given index and index one, and returns sixteen when one lies within ten horizontally and eight vertically of the given point; otherwise returns zero.
- `check_nearby_actor_for_record`: Compares the position of the record's stored actor index and the record fields against every live actor except index one and the record's own index, returning sixteen when one is within ten horizontally and eight vertically.
- `scan_actors_near_record`: Iterates over actor indices from one upward, skipping the record's own index and index one, and returns sixteen when a live actor's position is within ten horizontally and eight vertically of the record; returns zero when the table ends.
- `scan_actors_near_record_skipping`: Walks the actor table with a skip-aware loop comparing each live entry's position to the record and returns sixteen when one is within ten horizontally and eight vertically; returns zero if none qualify.
- `find_closest_actor_index`: Walks actor indices from one, measures each qualifying actor's offset to the record, stores the index of the closest one in a global word, and returns the smallest offset found.
- `find_closest_actor_for_record`: Iterates actor indices from zero, computes each qualifying actor's offset from the record, keeps the smallest value in the record and records the matching index in a global word, returning the final minimum.
- `measure_actor_offset_distance`: Takes the absolute horizontal and vertical differences between two actor positions, returns the horizontal difference, or the result of an indirect library call when the two differences are within a factor of two of each other.
- `execute_library_pair`: Compares two values and, if a condition is met, calls two indirect library routines in sequence.
- `invoke_library_routine_pair`: Calls two indirect library routines in sequence with no arguments and returns, with no condition checks.
- `move_actor_if_unblocked`: Computes candidate coordinates for an actor record through two indirect calls, tests them with the blocking-actor scan, and stores them into the record only when no blocking actor is found; returns true when stored.
- `store_actor_position_fields`: Writes two supplied sixteen-bit values into the position fields of the actor entry selected by the record's index, then returns one.
- `draw_indexed_sprite_pair`: Looks up two coordinate bytes for the given index, draws a fixed-size image at the base position through an indirect draw call, and draws it again at the looked-up offset when the index is below nine.
- `build_packed_table_by_mode`: Chooses a colour constant from the current mode value and a stored flag, clamps the magnitude argument, optionally invokes a library call, then fills thirty-two words with three small values packed per word and passes the table to a library copy routine.
- `fill_table_and_wait_frames`: Repeats thirty-two passes, each filling entries one to thirty-one of a table with values masked to twelve bits from an indirect call, copying the table through a library routine and pausing one frame via another library call.
- `pop_and_dispatch_queued_event`: Takes the next code from a thirty-two-entry circular queue when one is pending, otherwise pauses one frame, then dispatches by code value to text output and indirect calls using stored counters and positions.
- `dispatch_next_queued_event`: Advances the read position of a thirty-two-entry circular queue and dispatches the code it held to the same set of text output and indirect calls, with no empty-queue check.
- `dispatch_event_code_direct`: Switches on a code passed in a data register and runs the matching text output and indirect calls for codes two, three, four, five, seven or ten, with no queue handling.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `enqueue_event_code`: Stores the given byte at the write position of a thirty-two-entry circular queue and advances the write position, unless the queue is already full.
- `print_cstring_at_fixed_position`: Measures a zero-terminated string, sets a fixed text window through an indirect call with constant arguments, positions the cursor at sixteen and forty-two, and writes the string using its measured length.
- `invoke_library_setup_sequence`: Makes one library call with seven constant arguments and a stored handle, another with two constants, then a parameterless routine, using three library vector stubs in that order.
- `measure_and_print_cstring`: Counts the characters of a zero-terminated string and passes the string address and its length to the text output routine.
- `wrap_text_into_lines`: Splits a text string into output lines of at most thirty-seven characters at space boundaries, expands percent tokens from a string table selected by a global index, and emits each line through the line-printing routine until a carriage return, the terminator or the count limit.
- `wrap_text_lines_from_pointer`: Performs the same space-break line wrapping as its sibling routine, working from a pointer supplied through an address register and emitting each completed line through the line-printing routine.
- `wrap_text_resume_scan`: Resumes the line-wrapping scan partway through a string, copying characters into the line buffer, expanding percent tokens and emitting a line whenever the count passes the limit.
- `wrap_text_scan_loop`: Runs the line-wrapping character loop, copying characters, recording the last space and emitting completed lines through the line-printing routine until the scan limit or the terminator is reached.
- `wrap_text_emit_and_scan`: Emits the pending line, resets the count, and then continues the same character scan that breaks text at spaces and expands percent tokens.
- `wrap_text_emit_line_breaks`: Terminates the current line buffer, prints it, and then keeps scanning the remaining text for further line breaks at spaces up to the count limit.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `wrap_text_from_string_index`: Skips the given number of zero-terminated strings from the start of a fixed string table, then wraps the string found there into lines of at most thirty-seven characters and emits each line through the line-printing routine.
- `wrap_text_from_second_table`: Skips the given number of zero-terminated strings from a second fixed string table, then wraps the string found there into lines of at most thirty-seven characters, emitting each line through the line-printing routine.
- `wrap_text_after_skipping_strings`: Skips the given number of zero-terminated strings from a supplied address, then wraps the next string into lines of at most thirty-seven characters and emits each line through the line-printing routine.
- `print_table_entry_with_separators`: Prints the table string selected by a global index, then a separator string from global data, then the supplied string, then a second separator string.
- `print_table_entry_then_text`: Prints the table string selected by a global index, then a separator string from global data, then the supplied string.
- `print_indexed_table_string`: Prints the table string selected by the current global index through the text output routine.
- `init_display_and_palette_state`: Points the block pointer at a working record, copies a field into it, makes two library calls with the record and constant arguments, stores a fixed mask word, runs a helper, copies thirty-two table words through a library routine, and sets a mode byte to two.
- `init_alternate_display_state`: Points the block pointer at a second record, stores a fixed mask word, runs two helper routines, and sets a mode byte to three.
- `step_levels_down_with_pauses`: Calls a routine with the current level in three argument positions, a zero and the table, pauses one frame, and lowers the level by five each pass from one hundred down to zero.
- `step_levels_up_with_pauses`: Calls a routine with the current level in three argument positions, a zero and the table, pauses one frame, and raises the level by five each pass from zero up to one hundred.
- `clear_record_fields_and_run`: Zeroes two fields of the record referenced by a stored pointer and then runs a routine.
- `run_scripted_helper_sequence`: Copies a stored long word to another location, makes ten helper calls with small constant arguments and values read from stored fields, copies a status word, and finishes with two parameterless helper calls.
- `append_segment_record_if_fits`: Reads three byte fields and a size word from an eight-byte table entry into a running record, and when the running offset plus the entry size stays within a fixed upper bound, calls a helper with the entry values and advances the offset.
- `fill_slot_record_from_template`: Selects a fifty-six-byte slot by index, releases it through an indirect call if it is already marked as type two, copies a fifty-six-byte template into it, sets its type to two, stores two shifted positions and a third value, and passes the slot to a routine.
- `load_template_record_and_call`: Copies a fifty-six-byte template into a fixed buffer, clears a long field, sets a word field to nine, and passes the buffer to an indirect call.
- `forward_call_to_helper`: Calls a helper routine with no arguments and returns, doing nothing else.
- `dispatch_slot_record_draw`: Passes a fixed buffer to an indirect call, clears a word, then forwards the stored long words and three words from the eighteen-byte entry selected by its argument to a routine.
- `forward_slot_unless_status_clear`: Returns without action only when the slot is marked as type two and an indirect status check returns zero; otherwise it forwards the slot to a helper routine.
- `load_song_chunks_from_file`: Opens a data file by its numeric id, reads up to twenty-eight length-prefixed chunks into a buffer while recording each chunk's offset in a table, stops before the total would exceed a fixed size limit, and closes the file; the referenced text is songs.
- `setup_view_then_draw_pair`: When a global flag is clear, waits through a library call for three hundred fifty units, sets up a 320 by 200 view through an indirect call, passes two supplied values to a second indirect call, and runs a helper followed, when it returns zero, by two more helpers.
- `draw_bar_rows_from_tables`: Iterates twenty-two rows, reads two byte tables per row, draws a background rectangle through an indirect call, then draws a series of segment rectangles whose heights come from a helper call and the table values, and pauses a frame after each row whose delay byte is non-zero.
- `test_input_code_thirty_two`: Reads a code through an indirect input call, sets a flag to one when the code equals thirty-two, and returns the same test result.
- `copy_block_above_address_bound`: When the supplied address value is above a fixed bound, allocates a block of the requested size through an indirect call, copies that many bytes into it and returns the new block; otherwise returns the original pointer.
- `draw_quad_with_orientation_flags`: Computes two signed cross products from four corner points built from the record and grid offsets, stores a sign flag for each, then when a temporary buffer can be allocated, draws the corner shape into it through indirect calls, restores the saved values and releases the buffer.
- `set_state_and_forward_record`: Sets a state word to two, forwards a fixed-size record and a stored value to a routine, forwards a table-selected pointer and a second value, and copies the state word into a counter word when it exceeds three.
- `store_actor_marker_record`: Copies the x coordinate of an actor entry and its y coordinate plus ten into a record, sets the record's first byte to the supplied value and its second byte to one.
- `update_table_entry_marker`: Locates a record either directly by index or through a per-slot offset table, then sets its marker byte to twenty-nine when that byte held fifteen, otherwise stores the supplied byte in the following field.
- `update_slot_marker_by_offset`: Finds the record through a per-slot offset table, then sets its marker byte to twenty-nine when that byte held fifteen, otherwise stores the supplied byte in the following field.
- `store_byte_to_offset`: Writes the byte held in the data register to the location five bytes past the address register, with no other operations.
- `register_nearby_entries_from_list`: When a group's active flag is clear and a counter exceeds nine, fills ten table slots with entries at random positions retried until a test passes, then walks the supplied list and registers each record inside a screen window into the actor table with a kind code, recording special kinds in global flags.
- `register_nearby_entries_from_stack`: Performs the same spawn-and-register procedure as its sibling, reading the list pointer and count from the stack frame rather than from arguments.
- `register_nearby_entries_loop`: Walks the supplied list and registers each record inside the screen window into the actor table with a kind code, using the same kind test and special-kind flags as its siblings, and stops at the supplied count or when the table is full.
- `scan_entries_for_registration`: Advances to the next record, stops when the remaining count reaches zero or the table is full, and applies the screen-window test and registration steps to each record it reaches.
- `skip_out_of_window_entries`: Advances through the record list past records outside the screen window, then registers the first record found inside the window through the same kind-code and table-insertion steps.
- `finish_registered_record_fields`: Fills the kind-four fields of a newly registered table record with the entry's offset and a kind code of fifteen or thirteen depending on the source kind, copies the position words into it, and advances through the list under the window test.
- `register_or_refresh_record`: Refreshes the position words of a matching existing record or fills a new record with its kind-specific fields and offset, then advances through the list applying the screen-window test to each entry.
- `append_window_entry_records`: Appends a new record for each in-window list entry, filling its kind byte, position offset and a value from the stack frame, and refreshes the matching record instead when one already exists.
- `append_window_entry_flagged`: Appends in-window list entries as records whose kind byte is zero for source kind five and one otherwise, copies their position words, and advances the list until the count is exhausted.
- `append_window_entry_default_kind`: Appends in-window list entries as records with a default kind byte of one, switching to zero for source kind five and two for source kind six, copies their position words, and advances the list.
- `stub_return_only_2`: Contains only a return instruction and does nothing; it is an empty routine.
- `prompt_and_verify_typed_entries`: Shows a prompt for up to three pending text entries picked through an indirect call, reads typed printable characters with backspace handling up to nine characters until carriage return, and compares them with the stored string, returning zero on a mismatch and one after three matches.
- `call_six_library_routines`: Makes six library calls with no arguments in sequence, three through the executive library base and three through a stored library base, and returns the register pair unchanged.
- `store_result_and_release_if_set`: Calls a routine with two arguments, stores the returned value, passes that value to a release routine when it is non-zero, and returns the stored value.
- `open_df0_and_check_type`: Opens the device named by the referenced text df0 with a mode argument of minus two and stores the handle; when the handle is non-zero it checks a type field against two hundred thirty, calls a helper if it differs, passes the handle to a release routine and returns the handle.
- `return_from_disk_check`: This routine is empty and serves as a return point for a procedure that verifies a disk type.
- `wait_for_flag_with_timeout`: Polls a flag byte, pausing five units through a library call between checks for at most three hundred polls; clears the flag and returns one when it is set, or returns zero when the poll limit passes.
- `poll_flag_until_limit`: Repeatedly pauses five units through a library call and checks a flag byte, counting polls in the stack frame; clears the flag and returns one when it is set, or returns zero once the count reaches three hundred.
- `poll_flag_with_counter_loop`: Pauses five units through a library call and checks a flag byte in a loop that stops after three hundred polls; clears the flag and returns one when it is set, otherwise returns zero.
- `exchange_state_blocks_with_file`: Opens a file on the device named by the referenced text df1 with a numeric id chosen by a flag, and when it opens passes a sequence of state blocks with their sizes to a transfer routine before closing it; a failed open is recorded and the routine loops over the status checks and prompts.
- `exchange_state_blocks_from_df0`: Waits for input on the disk prompt, opens a file on the device referenced as df0 with a numeric id chosen by a flag, and passes the same sequence of state blocks with their sizes to a transfer routine before closing it; the status is reported through a display call, and the referenced text includes winpic.
- `exchange_state_blocks_after_prompt`: Shows the referenced prompt asking for a writable disk in any drive and waits for input, then runs the same open, block transfer and close sequence, reporting the aborted text when the user cancels.
- `exchange_state_blocks_with_retry`: Loops waiting for input on the disk prompt, then performs the file open, block transfer and close sequence with the drive letter derived from a stored index, reporting status and restoring the display state afterwards.
- `exchange_state_blocks_direct`: Opens the file for the drive letter derived from a stored index, transfers the same sequence of state blocks with their sizes, closes it, and restores the display state without any prompt.
- `exchange_state_blocks_fixed_file`: Opens the file with a fixed numeric id, transfers the state blocks with their sizes, closes it and reports the status, then restores the display state.
- `report_save_error_and_restore`: Shows the referenced message that the game could not be saved when the status is non-zero, waits for input, and restores the display state without opening any file.
- `report_load_error_and_restore`: Shows the referenced message that the game could not be loaded, loops waiting for input, then restores the display state and flags.
- `wait_for_game_disk_and_restore`: Shows the referenced request to insert the game disk and loops until input arrives, then restores the display state and flags.
- `reset_display_state_and_restore`: When the mode flag is clear, resets two state words, runs helper routines, sets a counter to ninety-nine, runs further helpers with small constant arguments, clears two flags, and restores the display through a library call.
- `transfer_block_and_check_status`: Transfers a block through a file-handle call, or through a direct transfer call when the mode flag is set, and calls the error routine when the returned status is negative.
- `transfer_block_from_frame`: Transfers a block whose handle, address and size come from the stack frame through the file-handle call, and calls the error routine when the returned status is negative.
- `store_rect_around_point`: Writes a four-field rectangle into the table slot chosen by the first argument, spanning two hundred fifty units left and right and two hundred units above and below the given point.
- `advance_counter_and_setup_view`: Refreshes display entries selected by a counter that this routine increments, pauses through a library call, sets a view rectangle with fixed coordinates through a library call, and adds three to a run of table bytes.
- `animate_palette_wave_sweep`: Passes the referenced text winpic to an indirect call, then runs a descending sweep of about fifty steps that builds a thirty-two-word table from a source table offset by the step and two indirect values, copies it through a library routine and pauses between steps.
- `animate_palette_wave_loop`: Continues the palette sweep from inside its step loop, refilling thirty-two table entries from the source table at the current offset, copying them through a library routine and pausing between steps until the sweep ends.
- `animate_palette_wave_steps`: Runs the palette sweep steps with two indirect values per step, refilling the table from the source table, copying it through a library routine and pausing, and ends when the step passes the lower bound.
- `select_value_by_table_flag`: Returns ten when the flag byte at the given index in the table is non-zero, otherwise returns eight.
- `place_marker_at_stored_position`: When the low two bits of a stored counter are zero or a state byte exceeds ninety-seven, calls an indirect routine with coordinates offset from the stored position by a flag-dependent amount, or with fixed coordinates when the index is eight or above, and returns its result.
- `place_marker_without_flag_offset`: Under the same counter and state condition, calls an indirect routine with coordinates offset from the stored position without any flag-dependent shift, or with fixed coordinates when the index is eight or above, and returns its result.
- `call_fixed_position_marker`: Calls an indirect routine unconditionally with fixed coordinates of one hundred, a unit flag and the table, and returns.
- `dispatch_actor_mode_action`: Stores a mode code in the actor record and runs the matching action for codes one through ten, such as a step toward coordinates taken from the record or the stored target, a timer adjustment or a fixed-coordinate step, gated where needed by a random check, then returns the action result.
- `dispatch_actor_mode_register`: Stores the mode code held in a data register in the actor record and runs the same mode-matched action set for codes one through ten, returning the action result.
- `dispatch_actor_mode_partial`: Holds the distance-check and mode-two, mode-four, mode-eight and mode-ten branches of the actor mode dispatcher with unreachable blocks removed, and updates only the record's state byte in those branches.
- `dispatch_actor_mode_with_flag`: Runs the mode-matched action set using a flag argument in place of the random gate for modes three, five, two, six and ten, and sets the record's state byte to twelve for modes four and ten.
- `forward_call_to_routine`: Calls one routine with no arguments and returns, with no other behaviour.
- `dispatch_actor_mode_gated`: Runs the mode-matched actions for modes two through six and ten, gated by a flag register, and sets the record's state byte to twelve for modes four and ten.
- `dispatch_actor_mode_no_distance`: Runs the gated actions for modes three, five, two, six and ten with no distance check and no mode-four branch, setting the record's state byte to twelve for mode ten.
- `dispatch_actor_mode_minimal`: Runs the gated actions for modes five, two, six and ten only, setting the record's state byte to twelve for mode ten.
- `dispatch_actor_mode_two_six_ten`: Runs the gated actions for modes two, six and ten, with a mode-two check that sets the record's state byte to four when the actor matches a stored index, and sets the state byte to twelve for mode ten.
- `dispatch_actor_mode_six_ten`: Runs the gated action for mode six and the gated action for mode ten, setting the record's state byte to twelve for mode ten.
- `lookup_actor_when_flagged`: When the flag register is non-zero, looks up a table entry through an indirect call with argument two, derives an offset from the result, and passes it to a routine.
- `set_sprite_flag`: Checks if a register equals ten and another register is non-zero to call another routine, then stores a constant byte into a table at an offset from a pointer.
- `decrement_counter_clamp_and_notify`: Subtracts the supplied amount from a stored counter and, when the result goes negative, clamps the counter to zero and calls a routine with the code thirteen; otherwise it calls a routine with the global text argument.
- `print_yum_text`: Passes the global text argument, which is the referenced text Yum, to a single output routine and returns.
- `scan_index_and_read_block`: Opens the file with a numeric id and scans its index for an entry matching a stored key; when one matches it reads the data block and passes each cell offset, stepped by the record's stride, to a routine in a grid of rows and columns, returning one on success and zero otherwise.
- `advance_record_timer_and_dispatch`: Clears a flag byte in the record, adds a word-sized step from the record to a long field at offset sixteen, and when the first byte is zero calls a routine four times with no arguments, returning the register pair unchanged.
- `step_audio_channel_sequence`: Advances one audio channel's sequence stream: when its timer is due it reads command byte pairs for instrument select, volume, and loop or end, looks up a value in a table and writes the channel's sample fields, and otherwise fetches the next data byte into the channel's note value.
- `decrement_counter_and_update_state`: Writes two constant words into the first structure, then decrements a byte counter in the second structure when it is non-zero; when the counter reaches zero it writes two words to other offsets of the first structure, and it always writes a constant word to the first structure.
- `start_audio_channel_one`: Sets two state bytes, then programs audio channel one by writing a volume of sixty-four, the sample address from the first argument, the length and period from the other two arguments, clears its pending interrupt request, enables its interrupt and switches on its DMA channel.
- `silence_audio_channel_one`: Clears bit zero of a state byte, sets the channel one volume register to zero and its period register to two, which silences audio channel one.
- `store_per_channel_long_values`: Stores four long-word arguments into four per-channel state slots spaced evenly through the state block, one slot for each audio channel.
- `reset_all_audio_channels`: Stores the four long-word arguments into per-channel slots, zeroes the four channel volume registers and the channel state fields, copies four words from a table into per-channel fields, marks each channel's state byte, and enables DMA for all four audio channels.
- `stop_all_audio_channels`: Zeroes the four channel volume registers, disables DMA for the four audio channels by writing their enable bits without the set flag, and marks a state byte with the all-ones value.
- `install_music_vblank_interrupt`: Stores three arguments and fills two interrupt-server structures referenced as audio four and music vblank, registers them through the exec library's add-server and set-vector calls, and clears and disables one audio interrupt through the interrupt registers.
- `remove_music_vblank_interrupt`: Calls a routine, removes the registered interrupt server through the exec library, writes a single-bit value to an interrupt-related field of the structure, and restores the interrupt vector through the library.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_buffer_and_set_fields`: Computes an offset from a stored index, writes a constant field, zeroes nine hundred forty-seven long words of a buffer at a fixed offset, stores two values into fields, and calls a routine.
- `clear_register_d0`: Sets the data register d0 to zero by calling a small routine that performs a logical operation.
- `open_dos_and_parse_arguments`: Opens the library referenced as dos.library, then either writes the tag MANX into a startup block and loads a file by numeric id, or splits the supplied argument text into quoted or space-separated tokens, stores a pointer for each token in a table, and passes that table to a routine; it reports an error routine when the library cannot be opened.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `copy_string_pad_zeros`: Copies bytes from the source string into the destination until the given count is used up or a terminator is copied, then fills the remaining count with zero bytes, and returns the destination.
- `append_string_bounded`: Moves to the end of the destination string, then copies the source string onto it, stopping at the terminator or at a fixed limit near thirty-two thousand characters, and returns the destination.
- `append_string_with_limit`: Moves to the end of the destination string, copies the source string onto it until the terminator or until the counter runs out, writes a terminator when the copy was cut short, and returns the destination.
- `copy_string_to_buffer`: Copies the source string into the destination byte by byte, including its terminator, and returns the original destination pointer.
- `cross_multiply_16bit_halves`: Splits each of two thirty-two-bit register values into sixteen-bit halves, sums the cross products of those halves, and returns that sum in the high long alongside the second operand in the low long.
- `apply_sign_to_helper_result`: Takes two signed register values, works out whether their signs differ, calls a helper, negates the helper's result when the signs differed, and returns that result together with the second operand.
- `negate_helper_result_if_negative`: Calls a helper, then negates the value it leaves in the second data register when the first operand is negative, and returns that value together with the original second operand.
- `call_and_return_concatenated`: Calls a routine and returns a pair of longs made from the original second data register and the value that the call leaves in that register.
- `forward_call_to_routine_2`: Calls one unnamed routine with no arguments and returns, with no other behaviour.
- `perform_32bit_division`: Divides a thirty-two-bit dividend by a divisor: for divisors above sixteen bits it runs a shift-and-subtract loop of sixteen steps, otherwise it uses two sixteen-bit divide steps, and returns the quotient and remainder as a pair of longs.
- `compute_string_length`: Walks a zero-terminated string to its terminator and returns the number of bytes before it, without modifying memory.
- `call_optional_hook_then_routine`: Calls a stored routine pointer when it is non-zero, then calls another routine with no arguments and returns.
- `release_stored_resources`: Calls one routine twenty times, calls a stored hook when it is set, releases up to three stored resources and two stored buffers through routines when they are held, and returns the result of a final call when no extra resource is held.
- `release_slot_entry`: Checks that the slot index lies within twenty entries and that the entry is set, returning minus one and recording error code three otherwise; when valid it releases the stored handle through a routine unless the top bit of the entry's flag word is set, clears the entry and returns zero.
- `call_library_vector_through_handle`: Calls a library function through a vector at a fixed offset from a library base stored in a handle field, with no arguments, and returns.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_routine_via_handle`: Calls a different library function through a vector at another fixed offset from a library base stored in a handle field, with no arguments, and returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_routine_from_handle`: Calls a library function at a fixed offset from the library base stored in a handle field, with no arguments, and returns.
- `dispatch_library_vector_via_handle`: Calls a library function at another fixed offset from the library base stored in a handle field, with no arguments, and returns.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_os_library_routine`: Calls an operating-system library function at a fixed offset from the library base held in a handle field, with no arguments, and returns.
- `call_os_library_routine_alt`: Calls a different operating-system library function at a fixed offset from the library base held in a handle field, with no arguments, and returns.
- `call_os_library_routine_alt_two`: Calls another operating-system library function at a fixed offset from the library base held in a handle field, with no arguments, and returns.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_os_library_routine_alt_three`: Calls a further operating-system library function at a fixed offset from the library base held in a handle field, with no arguments, and returns.
- `call_library_function`: Calls a library function using an offset from a base address stored in a global variable.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_second_base_library_routine`: Calls a library function at a fixed offset from a second library base stored in a separate handle field, with no arguments, and returns.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_slot_record`: Claims a free slot number through a routine and returns zero when none is free; otherwise it allocates a thirty-four-byte cleared record through a library call, fills its fields from the arguments, the slot number and a tag, stores a library call result, and passes the record to a setup routine, releasing the slot and returning zero if the allocation fails.
- `release_slot_record`: Runs a cleanup routine when the record's stored handle is set, marks the record as released, releases its slot number through a routine, and frees the thirty-four-byte record through a library call.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_with_forty_eight`: Passes the argument and the constant forty-eight to a library function, which operates on a block of that size, and returns.
- `call_library_with_argument`: Passes the single argument to a library function at a fixed offset from a stored library base, and returns.
- `allocate_typed_block`: Returns zero when the first argument is zero; otherwise it allocates a cleared public block of the requested size through a library call, sets its type byte to five, stores the size word and the first argument in it, and returns the block, or zero when allocation fails.
- `release_typed_block`: When the block pointer is non-zero, marks its type byte and two long fields with all-ones values and releases the block through a library call using the size stored in the block; returns zero for a null pointer.
- `mark_record_released_and_call`: Sets the record's type byte and two long fields to all-ones values, then calls a library function with no arguments.
- `call_second_base_routine_alt`: Calls a library function at another fixed offset from the second stored library base, with no arguments, and returns.
- `call_second_base_library_call`: Calls the library function at the same fixed offset from the second stored library base, with no arguments, and returns.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_second_base_routine_third`: Calls a library function at a further fixed offset from the second stored library base, with no arguments, and returns.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `execute_indirect_function_via_offset`: Fetches a pointer from a global variable, applies a negative offset to it, and executes the function located at the resulting address.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `initialize_empty_list_header`: Initialises a two-pointer list header so that its head points at the entry immediately after it, clears the following word, and points the tail back at the header, leaving an empty list.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function_2`: Loads a library base from a global variable and calls a function at a specific offset within that library.
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
- `call_library_routine_from_third_base`: Calls a library function at a fixed offset from a third stored library base, with no arguments, and returns.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_third_base_routine_first`: Calls a library function at a fixed offset from the third stored library base, with no arguments, and returns.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_third_base_routine_second`: Calls a different library function at a fixed offset from the third stored library base, with no arguments, and returns.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_third_base_routine_third`: Calls another library function at a fixed offset from the third stored library base, with no arguments, and returns.
- `call_library_offset`: Calls a function located at a specific offset within a library retrieved from a pointer.
- `call_third_base_routine_fifth`: Calls a further library function at a fixed offset from the third stored library base, with no arguments, and returns.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_third_base_routine_sixth`: Calls a library function at yet another fixed offset from the third stored library base, with no arguments, and returns.
- `call_library_offset_2`: Performs an indirect call to a function using an offset from a library base.
- `call_library_offset_3`: Executes a function call via a pointer found within a library structure.
- `call_library_offset_4`: Uses a pointer to access a library and calls a function at a specific offset.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_third_base_routine_tenth`: Calls a library function at another fixed offset from the third stored library base, with no arguments, and returns.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_third_base_routine_eleventh`: Calls a library function at a further fixed offset from the third stored library base, with no arguments, and returns.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_fourth_base_routine`: Calls a library function at a fixed offset from a fourth stored library base, with no arguments, and returns.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_fourth_base_routine_second`: Calls another library function at a fixed offset from the fourth stored library base, with no arguments, and returns.
- `call_fourth_base_routine_third`: Calls a further library function at a fixed offset from the fourth stored library base, with no arguments, and returns.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `null_routine`: The routine is empty and contains no executable instructions.

### Faery Tale Adventure, The (1986)(MicroIllusions)[a5].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spin_forever_loop`: Contains only an unconditional loop with an empty body, so it never returns and performs no work.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `spin_forever_single_loop`: Is a single empty instruction that loops back to itself forever, so it never exits and performs no work.
- `empty_routine`: This routine is empty and contains no executable instructions.
- `execute_infinite_loop`: An unreached routine that performs an infinite loop.
- `perform_infinite_loop_3`: This routine executes an infinite loop that prevents further instruction execution.
- `execute_infinite_loop_24`: The routine enters an endless loop that prevents further execution of the program flow.
- `nop_padding_sequence_11`: This segment consists of unreached data that does not form valid executable instructions.
- `empty_routine_2`: This routine contains no executable instructions.
- `stub_bad_instruction_data`: The bytes at this entry do not decode to valid instructions, so the routine has no readable logic.
- `execute_invalid_instruction_4`: Contains data that does not form a valid instruction.
- `nop_padding_segment`: The routine is an empty gap consisting of a single non-instruction byte used for data alignment or padding within the runtime segment.
- `loop_forever_in_place`: The routine is a single jump back to itself, with no calls, data access or return, so once entered it never leaves.
- `nop_padding_sequence`: This routine is empty and consists of a single byte of data that does not form a valid instruction.
- `enter_infinite_loop`: Runs a continuous loop without exit.
- `execute_invalid_instruction_5`: Contains an instruction that cannot be decoded.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `empty_padding_gap`: This routine is an empty gap in the executable containing no valid instructions.
- `execute_infinite_loop_2`: This routine enters an infinite loop that prevents further execution of subsequent code.
- `nop_padding_sequence_10`: This routine is empty and consists of a single non-operational instruction used for data alignment or padding within the executable segment.
- `stub_bad_instruction_data_2`: The bytes at this entry do not decode to valid instructions, so the routine has no readable logic.
- `nop_padding_byte`: This routine consists of a single byte of data that does not form a valid instruction.
- `empty_routine_3`: This routine contains no executable instructions and performs no operations.
- `empty_routine_13`: This routine is empty and contains no valid executable instructions.
- `execute_infinite_loop_3`: The routine enters an infinite loop that prevents further execution of subsequent instructions.
- `nop_instruction`: The routine contains a single instruction that performs no operation.
- `loop_forever_in_place_2`: The routine is a single jump back to itself, with no calls, data access or return, so once entered it never leaves.
- `execute_invalid_instruction_6`: Contains data that results in a bad instruction.
- `execute_invalid_instruction_7`: Contains a single byte that does not form a valid instruction.
- `execute_infinite_loop_4`: This routine enters an unconditional infinite loop that prevents further execution of the subsequent code.
- `nop_padding_sequence_2`: This routine is empty and consists of a single instruction that performs no operation.
- `empty_routine_4`: This routine contains no valid instructions.
- `empty_routine_5`: The routine contains no valid instructions and performs no operations.
- `execute_infinite_loop_5`: Enters an infinite loop that never exits.
- `nop_padding_sequence_3`: This routine is empty and consists of non-executable data used as padding within the segment.
- `execute_infinite_loop_6`: The routine enters an infinite loop that performs no operations.
- `execute_infinite_loop_7`: The routine enters an unconditional loop that prevents further execution of the current thread.
- `execute_infinite_loop_8`: The routine enters an infinite loop that prevents further execution.
- `execute_infinite_loop_9`: The routine enters an endless loop that prevents further execution of subsequent instructions.
- `execute_infinite_loop_10`: The routine enters an unconditional loop that prevents further execution of subsequent instructions.
- `execute_infinite_loop_11`: This routine enters an infinite loop that performs no operations.
- `empty_routine_6`: This routine contains no valid instructions and performs no operations.
- `execute_infinite_loop_12`: The routine enters an infinite loop that prevents further execution of the program flow.
- `execute_invalid_instruction`: This routine consists of a single byte of invalid data.
- `nop_padding_sequence_4`: This routine is empty and consists of a single instruction that performs no operation, serving as a gap in the executable code.
- `null_routine_3`: This routine is empty and contains no valid instructions.
- `nop_padding_sequence_5`: This routine is empty and consists of a single byte of non-executable data acting as padding.
- `execute_infinite_loop_13`: This routine performs an infinite loop that prevents further execution of the program flow.
- `nop_padding_sequence_6`: The routine is empty and consists of a single non-instruction byte used as padding within the executable segment.
- `execute_infinite_loop_14`: This routine enters an unconditional loop that repeats indefinitely.
- `nop_padding_gap`: This routine is empty and consists of a single instruction that serves as a gap in the executable code.
- `execute_infinite_loop_15`: The routine enters an infinite loop that performs no operations and never returns.
- `empty_routine_7`: This routine is empty and performs no operations.
- `empty_routine_8`: The routine contains no executable instructions.
- `nop_padding_sequence_7`: This routine is empty and consists of non-executable data used for alignment or padding within the segment.
- `execute_infinite_loop_16`: The routine enters an infinite loop that prevents further execution by repeatedly jumping to itself.
- `abort_execution`: The routine contains no valid instructions and terminates the current execution flow.
- `empty_routine_11`: The routine contains no executable instructions and performs no operations.
- `invalidate_instruction_flow`: The routine contains no valid instructions and terminates execution immediately upon entry.
- `execute_infinite_loop_17`: The routine enters an infinite loop that prevents further execution of subsequent code.
- `empty_routine_9`: The routine is empty and contains no valid instructions.
- `nop_padding_sequence_8`: This routine is an empty padding sequence consisting of a single non-operational instruction used to align code boundaries.
- `execute_infinite_loop_18`: This routine enters an infinite loop that prevents further execution of subsequent instructions.
- `perform_infinite_loop`: The routine executes an infinite loop that prevents further instruction execution.
- `execute_infinite_loop_19`: The routine enters an endless loop that prevents further instruction execution.
- `execute_infinite_loop_20`: The routine enters an endless loop that prevents further execution of the calling code.
- `execute_invalid_instruction_2`: This routine contains a single byte of invalid data.
- `execute_infinite_loop_21`: The routine enters an unconditional loop that prevents further execution of the calling code.
- `nop_instruction_2`: The routine contains a single instruction that does not perform any operation or change the state of the processor.
- `null_routine_2`: The routine is empty and contains no valid executable instructions.
- `empty_routine_12`: This routine contains no valid executable instructions.
- `execute_infinite_loop_22`: This routine enters an unconditional infinite loop that prevents further execution of the current thread.
- `execute_infinite_loop_23`: This routine enters an infinite loop that prevents further execution of the program flow.
- `null_padding_sequence`: This routine is empty and consists of non-executable data filling a gap in the memory segment.
- `perform_infinite_loop_2`: Enters a loop that never terminates.
- `null_operation_loop`: The routine consists of an infinite loop that performs no operations.
- `empty_routine_10`: The routine is empty and performs no operations.
- `execute_invalid_instruction_3`: This routine is composed of a single byte of invalid data.
- `terminate_execution`: The routine contains no valid instructions and causes the processor to stop or the execution flow to break.
- `run_infinite_loop`: Enters a loop that repeats indefinitely.
- `contain_invalid_instruction`: This routine contains no valid executable instructions and terminates control flow immediately upon entry.
- `nop_padding_sequence_9`: This routine is empty and consists of non-executable data used for alignment or as a gap in the executable segment.
- `enter_infinite_loop_2`: Performs an endless loop that prevents further execution.
- `empty_routine_14`: the routine is empty and contains no executable instructions.
- `contain_invalid_data`: This routine consists of a single byte that does not form a valid instruction.
- `process_bad_opcode`: The routine is composed of a single byte of data that cannot be decoded as a valid instruction.
