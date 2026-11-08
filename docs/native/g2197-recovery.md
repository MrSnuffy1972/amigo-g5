# Game 2197 (faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02197). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902096 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 443 of 443 named (0 author symbols), 443 of 443 described (67 by rule, 78 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2182 of 2182 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14634 of 14666 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 124196 |
| code | 49780 |
| data | 687987 |
| padding | 5317 |
| unused | 34816 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf:type | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf:fmain | 3 | 13909 | 361 |
| 9 | 1 | 51 | 2 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf:echo | 2 | 117 | 2 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 76 | 74 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EF8) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 52 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 32-136 (bytes 16384-70144) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 149-159 (bytes 76288-81920) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.69)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 164-256 (bytes 83968-131584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.96)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 258-311 (bytes 132096-159744) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.47)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 314-340 (bytes 160768-174592) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 342-342 (bytes 175104-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 344-393 (bytes 176128-201728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.98)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 396-415 (bytes 202752-212992) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.67)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 419-422 (bytes 214528-216576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 424-445 (bytes 217088-228352) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 447-453 (bytes 228864-232448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.20)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 455-461 (bytes 232960-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.34)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 471-471 (bytes 241152-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 473-473 (bytes 242176-242688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 477-477 (bytes 244224-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 479-535 (bytes 245248-274432) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.97)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 538-671 (bytes 275456-344064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.89)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 673-673 (bytes 344576-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 677-695 (bytes 346624-356352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.90)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.11)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 714-718 (bytes 365568-368128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 720-756 (bytes 368640-387584) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.15)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 760-782 (bytes 389120-400896) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.68)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 784-791 (bytes 401408-405504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 800-868 (bytes 409600-444928) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.57)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 896-917 (bytes 458752-470016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.01)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 920-955 (bytes 471040-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 960-1171 (bytes 491520-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 1312-1370 (bytes 671744-701952) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.24)
- faery tale adventure, the (1986)(microillusions)[h transformer][tr sp]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf: blocks 1376-1501 (bytes 704512-769024) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `run_game_setup_and_loop` (fmain): A very large routine that sets up screen buffers and resource tables, draws the page graphics, and then runs a long main loop calling many helpers. Kind: loader/initialization with integrity/environment checks. Pass path: the branch where iVar7 == 0 (the 'if' block is entered).

## Functions


### Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_words_then_call`: Copies sixteen 32-bit words from a fixed block in the program data into a buffer whose address is passed on the stack, then makes an indirect call through a stored pointer.

### Disk-Validator

- `run_disk_validator_main`: Program entry point of the file labelled Disk-Validator.

### Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf:fmain

- `clear_state_block_then_setup`: Zeroes a block of 947 longwords starting at an offset from a base register, stores two values into the state block, then calls the main setup routine and returns.
- `open_libraries_and_init_display`: Opens the graphics and layers libraries and the trackdisk device, loads the Amber 9 font, allocates memory and display structures, and returns a distinct numbered error code at the first step that fails.
- `release_flagged_resources`: Checks a set of status flag bits and, for each flag that is set, calls the matching free routine for its resource.
- `build_six_entry_index_table`: Allocates a block and runs an init routine, then walks a packed sequence of six length-prefixed entries, recording each entry's pointer and size in a six-slot table.
- `toggle_open_closed_map_tile`: Finds a matching tile near a given map position and rewrites the surrounding cells into the open or closed form taken from a 17-entry table.
- `run_game_setup_and_loop`: A very large routine that sets up screen buffers and resource tables, draws the page graphics, and then runs a long main loop calling many helpers.
- `advance_y_until_position_free`: Stores a new X and Y position for an entity after recording how far it moved from its previous position.
- `classify_position_terrain_and_region`: Looks up a terrain class for the current position by scanning a range table and rejects the class when the position falls outside an allowed band.
- `update_state_from_table`: Combines a byte from a table entry with a value fetched through a helper and stores the result.
- `map_entity_to_view_coordinates`: Calls two helpers to read the entity's current coordinates, then passes them through a mapping routine with a fixed offset to produce two view coordinates.
- `find_free_spot_near_center`: Tries up to fifteen random offsets around a reference point to find an unoccupied position.
- `retire_expired_table_entry`: For an entry chosen from a table by a random index, it retires the entry once its counter has run out and its kind is not one of two excluded values.
- `reset_actor_slot_for_kind`: Chooses a slot type from a parameter, then reloads the matching resource if the current kind differs.
- `initialize_actor_defaults`: Sets a long list of fields in a shared state block to fixed starting values, including zeroes and two constant position words, and runs a setup routine when the argument is zero.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d7.
- `compute_rect_bounds_from_offset`: Computes paired rectangle coordinates from a parameter and a random value, with each coordinate formed by subtracting from a constant, and then passes the bounds to a helper that builds them.
- `select_zone_table_entry`: Picks a numbered entry from a table based on whether the current position lies inside a fixed screen band and on a state value, then passes that entry to a playback routine.
- `update_current_zone_index`: Computes a zone index from the current map position, calls a helper when the position is inside a range, and stores the index.
- `swap_display_buffers`: Swaps the front and back buffer pointers stored in program data, copies the new buffer's first two words into a working record, then calls three routines to update the display and its state.
- `open_input_device_handler`: Creates a reply port and opens the input device by name.
- `initialise_input_request_block`: Sets a request type code and a handler pointer in an input request block, submits it through three library calls, and then closes the associated port.
- `collect_flagged_entries_into_slots`: Collects up to twelve entries from a record table whose flag bit is set, storing their indexes and calling a helper for each.
- `draw_indexed_entry_by_mode`: Derives a display type code for an indexed entry from the current mode value, using fixed codes for two modes and a table lookup for the rest.
- `dispatch_mode_event_and_redraw`: Dispatches on the current mode and an event code.
- `poll_position_test_until_match`: Runs a step routine repeatedly, up to twenty-five times, until a position test returns five.
- `signal_position_and_follow_up`: Passes the stored position to a helper with a fixed code of one, then runs the same follow-up step used after a successful position test.
- `set_state_if_not_locked`: If a lock bit in a flag byte is clear, stores the given value as the current state, clears a related byte, and calls a routine to apply the change.
- `load_status_bytes_from_source`: Reads seven pairs of bytes and six further bytes from a source routine into two arrays and one shared field, choosing that field's value from a parameter.
- `wait_until_counter_reaches_ten`: Repeatedly calls a step routine while a 16-bit counter in the state block is below ten, then returns.
- `refresh_changed_entry_fields`: For the entry selected by a counter below ten, compares each of its fields with a cached copy.
- `call_scaled_entry_if_enabled`: Checks if bit zero of a flag byte is set.
- `init_four_buffers_from_state`: Calls a routine three times to initialise three buffers with the same count, loads a pointer from a table selected by a state index, and initialises a fourth buffer.
- `store_argument_in_record_field`: Stores the single stack argument into offset four of the record pointed to by an address register.
- `process_input_record_into_ring`: Processes one record in a loop.
- `queue_flag_change_code`: Writes one code into a 128-entry ring buffer when a flag bit changes between two stored words, then advances the buffer head.
- `queue_grid_position_code`: Maps a screen position inside a fixed rectangle to a code derived from its row, writes that code into a 128-entry ring buffer, and stores the position word.
- `pop_byte_from_ring_buffer`: Returns the next byte from a 128-entry ring buffer and advances its tail, or returns zero when the buffer is empty.
- `advance_pseudo_random_state`: Advances a 32-bit state with a multiply-and-add step and returns a value mixed from its high and low bits.
- `mask_random_with_parameter`: Calls the pseudo-random generator and returns its result ANDed with the parameter.
- `random_bit_flag`: Returns the lowest bit of a fresh pseudo-random value.
- `random_two_bit_value`: Returns a fresh pseudo-random value masked to its two lowest bits.
- `random_three_bit_value`: Returns a fresh pseudo-random value masked to its three lowest bits.
- `random_six_bit_value`: Returns a fresh pseudo-random value masked to its six lowest bits.
- `random_low_byte_value`: Calls the pseudo-random generator and returns only the low byte of its result.
- `combine_random_modulo_parameter`: Calls the pseudo-random generator, takes its low 16 bits modulo the low 16 bits of the parameter, and returns that remainder packed together with the parameter.
- `advance_random_then_library_call`: Advances the pseudo-random generator and then calls a library routine at a fixed table offset, returning the register pair unchanged.
- `format_decimal_number_right_aligned`: Writes a 16-bit value as decimal digits into a ten-position text buffer, filling the unused leading positions with spaces.
- `format_decimal_digits_into_buffer`: Writes a number as decimal digits from the end of a buffer toward its start.
- `clear_text_buffer_with_spaces`: Fills a text buffer with space characters, counting down from a given index to zero.
- `sweep_grid_with_callbacks`: Runs three nested counting loops over a grid of 17 by 16 cells with five inner steps, calling routines through function pointers at every step.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `print_text_string_with_codes`: Walks a null-terminated string.
- `lookup_cell_nibble_for_position`: Converts two position words into a cell index, looks up a map value and a bit mask for it in two tables, and returns the high nibble of the matching entry or zero.
- `lookup_cell_nibble_from_registers`: Performs the same map and mask lookup as its sibling routine, taking the two position words from registers instead of the stack, and returns the matching high nibble or zero.
- `call_and_pop_stack_slot`: Calls a routine from the function table, then decrements a stack-pointer counter in the state block by four.
- `call_and_reset_stack`: Calls the same table routine, then subtracts a stored base value plus four from a stack-pointer counter in the state block.
- `run_nineteen_update_steps`: Calls one routine with a state value, then calls nineteen more routines in sequence and returns its input registers unchanged.
- `run_six_update_steps`: Calls six routines in sequence and returns without doing any other work.
- `copy_source_words_to_buffers`: Copies sixteen rows of 16-bit words from a source pointer derived from a parameter into several destination buffers, reading two source words per iteration.
- `stub_return_only`: Decodes to three instructions that only return, with no work done.
- `run_single_state_update`: Calls one routine with a value from the state block and returns its input registers unchanged.
- `run_nineteen_update_steps_2`: Calls one routine with a state value, then calls eighteen more routines in sequence, and returns its input registers unchanged.
- `forward_call_to_routine`: Calls one routine and returns, doing no data work of its own.
- `iterate_visible_tile_grid`: Converts two position words into tile coordinates, wrapping them within a 128-unit range and clamping them to fixed maxima, then loops over the surrounding grid cells and calls a per-cell routine for each one.
- `convert_bytes_to_five_bitplanes`: Reads packed byte data and shifts one bit at a time from each byte into five separate bit-plane buffers, storing each finished word in its plane.
- `blit_tile_rows_to_bitplane`: Copies eight rows of tile words from a source tile into a destination bit plane at an offset computed from the position parameters.
- `compute_map_cell_index`: Converts two position words into a wrapped map cell offset, combines the cell's byte with the low bits of the position, and returns the result plus a base value.
- `build_viewport_tile_indexes`: Walks the viewport in steps, converting each position into a map cell index with the same wrapping and clamping as its sibling routines, and stores the indexes in a 16-bit table.
- `decode_run_length_stream`: Expands a run-length encoded stream into a destination buffer: a negative count repeats the following byte, and a non-negative count copies that many literal bytes.
- `add_table_scaled_delta_masked`: When the second argument is below eight, adds a table-scaled version of the third argument to the first and masks the result to fifteen bits.
- `add_scaled_delta_keep_sign`: Performs the same table-scaled addition as its sibling routine, but keeps the sign bit of the first argument unchanged in the result.
- `copy_bit14_into_sign_bit`: Sets bit fifteen of the value when bit fourteen is set, and clears bit fifteen otherwise.
- `clear_register_sign_bit`: Returns the value with its sign bit, bit fifteen, cleared.
- `step_position_toward_target`: Compares the stored X and Y position with a target and nudges each coordinate one step toward it.
- `nudge_position_from_registers`: Performs the same one-step nudge toward a target as its sibling routine, taking the target from registers, and snaps the coordinate when the offset is large.
- `nudge_position_to_target`: Moves the stored X coordinate one unit toward a target when it is far enough away, and nudges the Y coordinate in the same way or snaps it to a fixed offset from the target.
- `advance_x_and_adjust_y`: Moves the stored X coordinate one unit forward when the offset is large enough, then adjusts Y toward its target, snapping it to a fixed offset when the offset is very large.
- `adjust_y_toward_target`: Moves the stored Y coordinate one unit toward a target when the offset is large enough, and snaps it to a fixed offset from the target when the offset is very large.
- `step_y_toward_target`: Moves the stored Y coordinate one unit toward a target when the offset from it exceeds ten in either direction.
- `increment_y_when_past_threshold`: Adds one to the stored Y coordinate when the offset from its target is greater than ten.
- `report_error_unless_two`: If the argument is not two, shows an error message through three routine calls with constant arguments and a pointer to the error text.
- `select_threshold_band_index`: Looks up a byte from a table using the short argument, then compares the argument against a series of thresholds to pick an index from three to ten.
- `select_direction_index_from_input`: Chooses a direction index either from the first joystick port's movement bits, through a lookup table, or from a stored horizontal position against threshold bands when a mode flag is set.
- `update_direction_index_from_input`: Makes the same direction index choice as its sibling routine, from joystick bits or position bands, and stores and reports the index only when it changes.
- `read_joystick_direction_index`: Reads the joystick port's movement bits and maps them to a direction index through a lookup table.
- `update_clamped_direction_index`: Subtracts twenty from a stored counter and clamps the result to zero to nine, resetting the counter when it falls outside that range.
- `reset_index_to_nine`: Clears a stored counter, and if the stored index is not nine, stores nine and calls a routine with it.
- `check_two_stage_value_gate`: Calls a value routine and, if its result is neither one nor at least ten, calls it again.
- `combine_bitplane_words_by_rows`: Walks rows and columns of a bitmap in a nested loop, combining source words from several bitplane pointers into each destination word.
- `start_blitter_copy_sequence`: Waits for the blitter to become idle, then for four table entries programs its modulo, pointer, control and size registers and starts each blit, using a mask derived from the argument.
- `configure_blitter_registers_with_args`: Writes a fixed control value to the blitter control register and clears a second control register and a modulo register.
- `program_four_blitter_planes`: Waits for the blitter, then for four table entries programs its modulo, pointer and size registers from stored values and starts each blit.
- `set_blitter_pointers_from_offsets`: Waits for the blitter if it is busy, then sets the destination and source pointers and the modulo registers from stored offsets before starting a blit.
- `blit_four_masked_planes`: For four table entries, programs the blitter with a fixed control value and a full mask word, then starts a blit for each entry.
- `blit_four_planes_with_argument`: Blits four table entries with the same fixed control value and full mask as its sibling routine, while using the argument to select the source for each blit.
- `execute_indirect_pointer`: Loads a pointer from a location relative to a base address and executes it.
- `execute_indirect_function`: Calculates an address by applying a negative offset to a value in a register and then calls the function located at that resulting address.
- `compute_offset_sign_codes`: Computes the horizontal and vertical offsets from an entry's stored position to the given coordinates, unless the fourth argument is six, and derives a sign code for each direction.
- `compute_offset_sign_and_notify`: Performs the same offset and sign computation as its sibling routine, then calls a routine with the result.
- `compute_octant_from_offsets`: From two signed offsets held in registers, derives sign codes and a direction value by comparing the halved offsets, and returns them packed together.
- `compute_direction_sign_pair`: Derives two sign and direction codes from the signed offsets held in the registers, using their halved values, and calls three helper routines to finish the calculation.
- `set_direction_field_from_codes`: Sets a field of a record to the sum of two direction codes masked to three bits, negating one code first when a helper reports a flag.
- `store_direction_difference_field`: Sets a field of a record to the difference of two direction codes masked to three bits, and sets a timer byte to twelve unless the mode argument is five.
- `react_to_nearby_entry_kind`: When a counter is below four and the nearby entry's kind matches one of two codes, it calls a routine with a fixed code.
- `subtract_offset_when_kind_allowed`: When the entry kind is not one of two excluded codes, subtracts a stored value from a per-entry field and calls a helper to update that field.
- `decrement_entry_field_and_clamp`: Subtracts a stored value from a per-entry field and clears the field when it becomes negative.
- `place_entry_at_random_offset`: Chooses one of three base offsets according to the current direction state, adds a random amount to it, and passes the result to a placement routine.
- `place_entry_at_side_offset`: Places an entry at a random offset from one of two base values selected by the direction state, then checks the entry kind before continuing.
- `place_entry_at_fixed_base`: Places an entry at a random offset from a fixed base value, then checks its kind and calls a helper when the direction value is not negative.
- `count_entries_of_kind_two`: Loops over entries from index three up to a limit and counts those of kind two, and among them those whose sub-kind is fifteen.
- `tally_kind_two_subtypes`: Loops over the entries and tallies kind-two entries whose sub-kind is fifteen into one counter and those whose sub-kind field is five into a second counter.
- `tally_subtypes_and_check`: Loops over the entries, counting those with sub-kind fifteen and those with a five-valued field, and then compares a stored value to decide whether to continue.
- `resolve_entry_counters_into_action`: Counts kind-five entries and then walks the remaining entries.
- `show_battle_result_messages`: Shows one of a group of battle result messages chosen by two counters and a threshold, calling a helper with the message text for each one.
- `report_enemy_retreat_kill_counts`: Prints one of two status lines, one for enemies retreating and one for enemies killed in battle, selected by counters and a threshold.
- `test_position_against_entries`: Tests a position first with a helper check and then by scanning the entity table, returning a nonzero code on a hit or zero when nothing matches.
- `find_nearest_eligible_entry`: Scans the entries for one that is not the reference entry, is not kind five and does not have sub-kind fifteen, and returns a code for the first match or zero.
- `scan_entries_for_eligible_target`: Walks the entries for an eligible target, skipping the reference entry, entry index one, kind-five entries and sub-kind fifteen entries, and reports the first one it finds.
- `scan_entries_for_other_targets`: Walks the entries after a given index, skipping the reference entry, entry index one and entries of the excluded kinds, and returns zero when the list ends.
- `scan_kind_one_entries`: Scans entries from index one up to the entry count and, for each kind-one entry that passes a sub-kind check when the flag argument is zero, updates a counter.
- `find_closest_entry_within_range`: Scans the entries for the one nearest a reference entry, keeping the smallest distance seen, and stores its index when that distance is closer than the reference entry's range field.
- `compute_entry_distance`: Computes the absolute horizontal and vertical distances between two entries' stored positions and returns a combined measure of them.
- `call_conditional_routines`: Compares two values and, if a condition is met, executes two different routines via pointers stored in a table.
- `run_two_helper_steps`: Calls two helper routines in sequence and returns.
- `try_move_entry_to_offset`: Computes two coordinates for an entry from table values and the given offsets, tests them with a helper, and stores them when the test passes.
- `set_entry_position_words`: Stores two position words in the table slot of the entry selected by a kind code and returns one.
- `apply_byte_record_to_list`: Reads four bytes from a per-index record, stores a long word in a list node, and passes the values together with fixed arguments to a drawing routine.
- `select_mode_parameter_and_draw`: Chooses a numeric parameter from the current mode value and a flag, computes coordinates for an entity from it, and calls helpers to draw it.
- `fill_random_value_table`: Fills a 32-entry table with random values below 4096 and passes the table to a display routine, repeating this 32 times with a short wait between passes.
- `dispatch_queued_command`: Reads the next code from a 32-entry command queue and advances its read index.
- `read_next_command_and_dispatch`: Reads the next code from a 32-entry command queue, advances its read index, and dispatches the code through a switch whose cases call drawing and message routines with fixed arguments.
- `dispatch_command_code_switch`: Switches on a command code passed in a register.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `push_byte_to_command_queue`: Appends one byte to a 32-entry queue if the queue is not full, then advances the write index.
- `measure_and_show_text_frame`: Measures a null-terminated string and makes three helper calls with fixed coordinates and sizes, passing the measured length to the last one.
- `show_fixed_text_frame`: Calls three helper routines with fixed coordinates and sizes, using a state value for the first, and returns.
- `measure_text_and_print`: Measures a null-terminated string and passes it with its length to a text output routine.
- `forward_call_no_arguments`: Calls one further routine with no arguments and returns, doing no other work of its own.
- `wrap_text_into_line_buffer`: Copies characters from a string into a line buffer of up to 37 positions, breaking at spaces, and stores the start of the buffer in a pointer slot.
- `copy_word_into_line_buffer`: Copies characters one at a time from the input text, recording where each word starts, and resets the line pointer at a space or the end of the string.
- `expand_percent_placeholder_text`: Copies text into a buffer, and when it meets a percent sign it inserts the string chosen by a state index from a table of strings before continuing.
- `advance_wrap_scan_position`: Advances the text-scan column counter and copies the next character, recording a word start at each space.
- `store_char_in_line_buffer`: Stores one character at the current line buffer position, advances the buffer pointer and the column counter, and continues scanning the text.
- `terminate_and_flush_line`: Ends the current line buffer with a zero byte.
- `return_from_subroutine`: The routine performs a single return instruction to exit the current execution flow.
- `skip_strings_in_table_a`: Advances through a string table by skipping the given number of null-terminated strings from a fixed base, then stores the resulting pointer for the caller.
- `skip_strings_in_table_b`: Advances through a second string table by skipping the given number of null-terminated strings from its fixed base, then stores the resulting pointer for the caller.
- `skip_n_strings_from_pointer`: Advances from a given string pointer past the given number of null-terminated strings and stores the resulting pointer in a state slot.
- `print_indexed_message_with_suffix`: Prints the message string selected by a state index, then the argument, then a fixed text fragment.
- `print_indexed_message_with_argument`: Prints the message string selected by a state index, then a fixed text fragment and the argument.
- `call_indexed_routine`: Calls a routine, then uses a value from a table indexed by a value from another memory location to call a second routine.
- `print_indexed_message`: Prints the message string selected by a state index from the string table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `prepare_display_record`: Calls one helper, points a state slot at a display record, sets its fields to fixed values through three library calls, and then makes a further helper call.
- `select_secondary_display_record`: Points a state slot at a second display record, sets one of its fields to a fixed value, calls two helpers, and sets a state value to three.
- `run_countdown_wait_loop`: Counts down from 100 in steps of five, calling a helper with the count and a fixed pointer and then waiting one step on each pass.
- `run_countup_wait_loop`: Counts up from zero to 100 in steps of five, calling a helper with the count and a fixed pointer and then waiting one step on each pass.
- `clear_record_position_fields`: Sets two 16-bit fields of a record to zero and then calls a helper routine.
- `run_startup_setup_sequence`: Stores a state value, then issues a series of setup calls with constant and state-derived arguments, and ends by clearing a state field.
- `fill_record_fields_within_limit`: Calls an indirect routine several times to obtain slot offsets, then stores fields from an eight-byte table record into those slots.
- `initialise_slot_from_template`: If the slot already has state 2, it first calls a library function on it.
- `submit_template_copy_to_library`: Copies a 14-long template into a fixed buffer, clears one long and sets a word to 9, then passes the buffer to a library function.
- `stub_self_call_trampoline`: Contains only a call to itself and a return, with no logic of its own.
- `reset_buffer_then_dispatch_entry`: Calls a library function on a fixed buffer and clears a word in it.
- `skip_when_library_reports_ready`: If the slot is in state 2 and a library function returns zero for it, the routine returns early.
- `load_song_offset_table`: Opens the file named songs through a library call, reads up to 28 length words to build an offset table into memory, and closes the file.
- `setup_display_region_if_enabled`: Returns unless a flag byte is zero.
- `draw_scene_rows_from_tables`: Loops over 22 rows, using byte tables to pick a step and a height for each row.
- `read_space_key_state`: Calls a library function to read a key code, stores whether it equals the space key code in a flag byte, and returns that result.
- `copy_high_buffer_to_block`: If the source address lies above the 2 MB address limit, allocates a new block through a library call and copies the bytes into it.
- `project_and_draw_quad`: Computes the four corner points of a quad from a lookup table, then uses a cross product to decide whether the quad faces the viewer.
- `set_state_and_copy_result`: Sets a word to 2, calls a routine with a fixed record and a table-selected entry, then copies the word into another variable if it is greater than three.
- `set_entry_position_and_flags`: Copies an X value and a Y value plus ten from a table entry chosen by the index.
- `store_marker_byte_at_slot`: Selects a record either from a table or from a fixed block, depending on a flag byte.
- `store_register_byte_into_slot`: Selects a record from a table or fixed block as in its sibling routine, then writes the low byte of a data register into the byte at offset five.
- `store_byte_at_offset_five`: Writes a byte value into the record at address register plus five.
- `populate_random_entry_group`: If a group is empty and its counter exceeds nine, fills ten six-byte entries with random positions and a random lookup-table value, then marks the group as filled.
- `fill_ten_random_entries`: Places ten entries at random positions, each with a random lookup-table value and an active flag.
- `scan_objects_for_region_change`: Walks a list of object records and computes each one's offset from a reference point.
- `step_object_scan_to_next`: Advances the object record pointer by six bytes and increments the scan counter, then computes the record's offset from the reference point.
- `claim_slot_for_matching_object`: For objects inside the window whose type is neither 0 nor 2, takes the next entry in the table.
- `fill_slot_with_object_record`: Writes a kind value of 4 and a lookup-table value into the next entry, then sets its flag and size from the current index.
- `fill_slot_position_and_advance`: Sets the entry's kind to 13 and its offset value, then copies the object's position into it.
- `register_object_slot_by_type`: Takes the next free entry and marks it active with a value from the object's type code.
- `copy_position_with_priority_flag`: Sets a priority byte to 0 for type 5 objects and 1 otherwise, copies the object's position into the entry, then advances to the next record.
- `copy_position_into_entry_default`: Sets a priority byte to 1, copies the object's position into the entry, and then advances to the next record for another window check.
- `stub_empty_return`: Contains only a return instruction, so it performs no work.
- `read_typed_name_into_buffer`: Reads keystrokes into a buffer of up to nine printable characters, echoing each one and honouring backspace.
- `compare_entered_text_to_word`: On return, compares the typed buffer with a stored word and returns zero on any mismatch.
- `read_entry_until_match`: Reads a line of keystrokes into a buffer and compares it with stored words on return.
- `match_word_then_retry_prompt`: Compares the typed buffer with a stored word and returns zero on mismatch.
- `compare_typed_text_and_reprompt`: Compares the typed buffer with a stored word and returns zero on mismatch.
- `clear_matched_slot_and_reprompt`: Clears the slot for the current word, counts the attempt, and returns one once more than two attempts have been made.
- `run_library_teardown_sequence`: Calls a sequence of library functions on the system library base, then repeats a pair of them on a stored library base.
- `store_result_then_release`: Calls a routine with two arguments and stores its result in a global.
- `open_df0_device_and_verify`: Opens the df0: device through a library call and, on success, checks a stored type code against an expected value before passing the handle to another routine.
- `stub_bad_instruction_data`: The code is only a bad instruction, and the disassembler truncated control flow here.
- `wait_for_flag_then_clear`: Polls a flag byte up to 300 times, pausing five ticks through a library call between checks.
- `poll_flag_five_tick_delay`: Loops up to 300 times waiting for a flag byte to be set, with a five-tick library delay on each pass.
- `wait_for_flag_ticks`: Pauses five ticks per pass for up to 300 passes, waiting for a flag byte to be set.
- `open_drive_file_for_letter`: Builds a path ending in the drive letter given by the parameter, then opens it on the df1: device using one of two modes chosen by a flag.
- `wait_for_disk_then_open`: Loops waiting for a disk to be inserted, with two checks that can end the wait.
- `prompt_insert_any_disk`: Shows a Spanish prompt asking for a disk in any drive and waits for one to be inserted.
- `prompt_and_open_save_file`: Opens a save file in a mode chosen by a flag.
- `open_save_and_restore_state`: Opens a drive-letter file and, when the open succeeds, copies saved state blocks of fixed sizes into game data areas.
- `open_file_and_restore_tables`: Opens a file and, on success, reads several tables of fixed sizes into the game's data.
- `save_game_with_retry`: If the file cannot be written, shows a Spanish write-error message and waits for input before retrying.
- `reset_load_state_after_error`: Waits for the user to dismiss a load-error message, then, outside the alternate mode, resets several counters and flags.
- `prompt_game_disk_and_reset`: Loops asking for the game disk until one is seen, then clears several counters and flags unless the alternate mode is active.
- `reset_counters_and_refresh_view`: Clears a few flags and counters when the alternate mode flag is clear, calls two routines, and then refreshes the display through a library call.
- `transfer_block_and_flag_failure`: Transfers a block of data through the open file handle, using a library call when the alternate mode flag is clear and a second routine otherwise.
- `read_block_and_flag_error`: Passes two stack arguments to a library read call on the open handle.
- `set_rectangle_around_center`: Writes a rectangle of four words into an entry chosen by index.
- `run_timed_intro_screens`: Runs an intro sequence: it refreshes the display, shows numbered graphics with timed pauses, and draws a frame region through library calls.
- `prepare_scrolling_title_columns`: Clears a set of column words, fills two 32-byte buffers from a source table, and sets up a 25-step loop of column offsets.
- `animate_title_column_drop`: Repeatedly randomises two column offsets, copies a 32-byte buffer to the display, and waits a short delay each step.
- `finish_title_column_animation`: Sets two random column offsets and steps the animation by copying a 32-byte buffer and waiting nine ticks.
- `select_value_from_flag_table`: Returns 10 when the flag byte at the given table index is nonzero, otherwise returns 8.
- `call_positioned_routine_by_index`: Chooses an offset from a flag, then passes coordinates derived from a stored position to a routine when the index is below eight.
- `call_routine_with_offset_coordinates`: When the index is below eight, passes stored position values minus fixed offsets, with a count of one, to a routine.
- `call_routine_with_fixed_coordinates`: Passes three fixed values of 100, a count of one and a buffer address to another routine, then returns.
- `apply_entry_action_code`: Stores a code byte in an entry, and picks a random subtype when the entry's kind is 2.
- `record_code_and_measure_distance`: Stores a code in an entry and, for code 8, computes absolute horizontal and vertical distances from a reference position before calling a routine.
- `measure_entry_distance_by_code`: For code 8, computes absolute horizontal and vertical distances from the reference position and calls routines based on the results.
- `choose_proximity_action_by_distance`: For code 8, computes absolute horizontal and vertical distances from the reference position.
- `stub_forward_single_call`: Makes one call to another routine and returns, with no other work.
- `apply_action_by_entry_kind`: Branches on an entry's kind value.
- `apply_actions_later_entry_kinds`: Handles entry kinds 3, 5, 2, 6 and 10.
- `kind_actions_2_6_10`: Handles entry kinds 5, 2, 6 and 10.
- `kind_actions_2_and_10`: Handles entry kinds 2, 6 and 10.
- `kind_actions_6_and_10`: Handles entry kinds 6 and 10.
- `call_helper_when_triggered`: When the trigger value is nonzero, looks up a value from the entry, passes an adjusted version to a helper, and then calls a routine.
- `set_kind_ten_marker_byte`: When the entry kind is 10 it calls a routine if the trigger is nonzero, then writes the constant 12 into the entry's kind byte.
- `decrement_counter_trigger_at_zero`: Subtracts the given amount from a counter.
- `forward_yum_text_message`: Passes the 'Yum!' text to another routine and returns.
- `open_resource_and_locate_entry`: Computes an index from a table word and an offset, opens a named file through a library call, and stores the handle.
- `clear_flag_and_advance_position`: Clears a flag byte and adds a word from the record to a position value stored in the same record.
- `copy_stream_byte_in_range`: Compares a position with the structure's thresholds and, when the flags allow, reads the next byte from a stream and stores it in an output field.
- `decrement_timer_set_status`: Writes the value 256 into two words of a structure.
- `start_channel_one_sample_playback`: Loads the sample address, length and period into audio channel 1 and sets its volume to 64.
- `silence_channel_one`: Clears a status bit, sets audio channel 1's volume to zero and its period to 2.
- `store_four_long_parameters`: Stores four 32-bit arguments into four separate slots of the game's data block at fixed offsets.
- `initialise_four_audio_channels`: Stores four 32-bit arguments in paired slots, clears the volume registers of all four audio channels and their DMA enables, then loads several table words from a stored pointer.
- `silence_all_audio_channels`: Sets the volume of all four audio channels to zero, disables their DMA through the DMA control register, and sets a status byte to 255.
- `install_vblank_music_handler`: Stores three arguments as music state, sets a timing value, and installs a vertical-blank interrupt server named music vblank through the exec library.
- `remove_vblank_music_handler`: Calls a routine, removes the music interrupt server through the exec library, and then restores the interrupt vector with a second library call.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_and_reset_slot_table`: Computes a slot offset from a stored index, clears 947 long words of a table, and stores pointers into the slot.
- `stub_no_operation_return`: Does nothing and returns, with no instructions beyond the return.
- `open_dos_library_read_header`: Opens dos.library through the exec library and returns early if the open fails.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `copy_bounded_string_padded`: Copies a null-terminated string into a destination of fixed size, stopping at the terminator or the size limit.
- `append_string_with_limit`: Appends a source string to the end of a destination string, copying at most about 32 thousand characters.
- `append_string_up_to_count`: Finds the end of the destination string and appends the source string, copying at most the given count of characters.
- `copy_string_until_terminator`: Copies a null-terminated string from source to destination, including the terminator, and returns the destination pointer.
- `multiply_longs_via_partial_products`: Multiplies two 32-bit unsigned values held in the D0 and D1 registers using 16-bit partial products.
- `multiply_signed_values`: Works out the sign of the product from two signed inputs, calls an unsigned multiply routine on the magnitudes, and negates the result when the signs differ.
- `negate_result_by_sign`: Calls another routine, then negates its result when the first input is negative.
- `forward_call_returning_pair`: Calls another routine and returns its result alongside the original second register value.
- `forward_single_call_and_return`: Calls another routine and returns with no other work.
- `divide_longs_with_remainder`: Divides a 32-bit unsigned dividend by a 32-bit divisor and returns the quotient and remainder as a pair.
- `measure_null_terminated_string`: Walks a zero-terminated string and returns the number of characters before the terminator.
- `run_optional_hook_then_continue`: Calls a stored hook routine if one has been set, then calls another routine.
- `run_twenty_steps_then_hooks`: Calls a routine twenty times, then runs an optional stored hook and several other routines whose pointers are set.
- `release_indexed_slot_or_fail`: Checks that the index is between 0 and 19 and that the slot is in use.
- `call_stored_library_vector_36`: Loads the library base from a global variable and calls the library function at offset minus 36, returning its result.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_144_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_stored_library_vector_132`: Loads the library base from a global variable and calls the library function at offset minus 132, returning its result.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_stored_library_vector_84`: Loads the library base from a global variable and calls the library function at offset minus 84, returning its result.
- `call_stored_library_vector_30`: Loads the library base from a global variable and calls the library function at offset minus 30, returning its result.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_stored_library_vector_60`: Loads the library base from a global variable and calls the library function at offset minus 60, returning its result.
- `call_stored_library_vector_42`: Loads the library base from a global variable and calls the library function at offset minus 42, returning its result.
- `call_stored_library_vector_66`: Loads the library base from a global variable and calls the library function at offset minus 66, returning its result.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_stored_library_vector_90`: Loads the library base from a global variable and calls the library function at offset minus 90, returning its result.
- `call_stored_library_vector_48`: Loads the library base from a global variable and calls the library function at offset minus 48, returning its result.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_468_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_second_library_vector_414`: Loads a second library base from a global variable and calls the library function at offset minus 414, returning its result.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `create_message_port_with_signal`: Allocates a signal bit through a library call, then allocates a zeroed memory block and fills in its message port fields.
- `release_message_port_block`: Calls a routine when a stored pointer is set, marks the block as released, frees its signal bit and then releases the memory block through the library.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `free_memory_block_48`: Passes a pointer to a library function together with a size of 48 bytes, releasing a fixed-size block.
- `release_pointer_through_library`: Passes a pointer to a library function that releases it, with no size argument.
- `allocate_typed_memory_block`: Allocates a memory block of the requested size through a library call unless the pointer argument is zero.
- `free_typed_memory_block`: If the block pointer is nonzero, marks its type as 255 and both link fields as minus one, then releases it through a library call using its stored size.
- `reset_list_node_call_library`: Sets a type byte to 255 and two link fields to minus one in a node passed in an address register, then calls a library function.
- `call_library_vector_198`: Calls the library function at offset minus 198 from the stored library base and returns.
- `call_library_function`: Accesses a library base address from a global variable and calls a specific function offset within that library.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_294`: Calls the library function at offset minus 294 from the stored library base and returns.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Calls the library function at offset minus 132 from the stored library base and returns.
- `call_library_vector_210`: Calls the library function at offset minus 210 from the stored library base and returns.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `initialise_empty_list_header`: Sets up an empty list header in memory: the head points to the tail, the tail is zero, and the tail predecessor points back to the head.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_552`: Calls the library function at offset minus 552 from the stored library base and returns.
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
- `call_library_vector_30`: Calls the library function at offset minus 30 from the stored library base and returns.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_462`: Calls the library function at offset minus 462 from the stored library base and returns.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_414`: Calls the library function at offset minus 414 from the stored library base and returns.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested structure within a table and executes it.
- `call_library_vector_468`: Calls the library function at offset minus 468 from the stored library base and returns.
- `call_library_vector_360`: Calls the library function at offset minus 360 from the stored library base and returns.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216`: Calls the library function at offset minus 216 from the stored library base and returns.
- `call_library_vector_240`: Calls the library function at offset minus 240 from the stored library base and returns.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested offset within a structure and executes it.
- `call_library_vector_456`: Calls the library function at offset minus 456 from the stored library base and returns.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354`: Calls the library function at offset minus 354 from the stored library base and returns.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Calls the library function at offset minus 60 from the stored library base and returns.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_36`: Calls the library function at offset minus 36 from the stored library base and returns.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_150`: Calls the library function at offset minus 150 from the stored library base and returns.
- `call_library_vector_144`: Calls the library function at offset minus 144 from the stored library base and returns.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `find_max_entry_and_store`: Walks a linked list of entries to find the largest value in a field, then stores that value into a slot when a result pointer is set.

### Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf:echo

- `program_entry_find_maximum`: Program entry point: calls a routine, walks a linked list to find the largest field value, then calls another routine and stores the result when a pointer is non-null.
- `copy_sixteen_longs_then_call`: If the destination pointer is non-null, copies sixteen 32-bit values from a fixed table into it, then calls a stored routine.

### Faery Tale Adventure, The (1986)(MicroIllusions)[h Transformer][tr Sp].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_infinite_idle_loop`: Loops forever without doing any work, so the routine never returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `infinite_loop`: An infinite loop that never exits.
- `empty_routine`: This routine contains no executable instructions.
- `execute_infinite_loop`: Enters an infinite loop that never terminates.
- `null_operation_routine`: The routine is empty and contains no executable instructions.
- `empty_routine_2`: This routine is empty and contains no executable instructions.
- `execute_infinite_loop_2`: This routine enters an infinite loop that prevents further execution of the program flow.
- `empty_routine_3`: This routine contains no executable instructions and performs no operations.
- `execute_infinite_loop_3`: This routine enters an infinite loop that prevents further execution of the subsequent code.
- `nop_padding_sequence_8`: This routine consists of a single instruction that serves as padding within the executable segment.
- `empty_routine_4`: This routine contains no valid instructions and performs no operations.
- `nop_instruction`: This routine consists of a single instruction that does not perform any operation.
- `null_operation`: The routine is empty and contains no valid instructions.
- `execute_invalid_instruction`: Contains an instruction that cannot be decoded.
- `enter_infinite_loop`: Enters a continuous loop that never terminates.
- `execute_invalid_instruction_2`: Contains data that does not form a valid instruction.
- `enter_infinite_loop_2`: Performs an endless loop of execution.
- `enter_infinite_loop_3`: Traps execution in a permanent loop.
- `enter_infinite_loop_4`: Executes a loop that repeats indefinitely.
- `execute_invalid_instruction_6`: The routine contains no valid executable instructions and triggers a control flow error upon entry.
- `execute_invalid_instruction_3`: Contains invalid data at the entry point.
- `enter_infinite_loop_5`: Runs a loop that never exits.
- `nop_padding_sequence_9`: This routine consists of a single instruction that serves as a gap between other code segments.
- `execute_infinite_loop_4`: The routine performs an infinite loop that prevents further execution by continuously repeating a no-operation instruction.
- `execute_infinite_loop_5`: The routine enters an infinite loop that performs no operations.
- `enter_infinite_loop_6`: Executes a continuous loop without exit.
- `enter_infinite_loop_7`: Enters a loop that repeats indefinitely.
- `execute_infinite_loop_6`: The routine performs an infinite loop that prevents further execution of the program flow.
- `enter_infinite_loop_8`: Runs an infinite loop structure.
- `null_operation_routine_2`: The routine is empty and performs no operations.
- `enter_infinite_loop_9`: Enters a loop that never terminates.
- `null_operation_routine_3`: The routine contains no valid executable instructions and performs no operations.
- `nop_padding`: This routine is empty and performs no operations.
- `empty_routine_5`: The routine contains no executable instructions.
- `null_instruction_padding`: This routine is empty and consists of a single byte of data that does not form a valid instruction.
- `execute_infinite_loop_7`: The routine enters an infinite loop that prevents further execution of the current thread.
- `nop_padding_sequence`: This routine is empty and consists of a single non-operational instruction used for alignment or padding within the executable segment.
- `execute_infinite_loop_8`: This routine enters an unconditional loop that prevents further execution of the current thread.
- `nop_padding_sequence_2`: This routine is empty and consists of unused space within the executable.
- `execute_infinite_loop_9`: The routine enters an infinite loop that prevents further execution by repeatedly executing a jump instruction to itself.
- `empty_routine_9`: The routine contains no executable instructions and performs no operations.
- `empty_routine_6`: This routine contains no valid instructions.
- `nop_padding_sequence_3`: This routine is empty and consists of non-instructional data used to pad the executable segment.
- `execute_infinite_loop_10`: This routine enters an infinite loop that performs no operations.
- `undefined_instruction_trap`: The routine contains invalid machine code that prevents successful disassembly and execution flow analysis.
- `empty_routine_7`: The routine contains no valid instructions and performs no operations.
- `empty_routine_10`: This routine contains no valid instructions and performs no actions.
- `execute_infinite_loop_11`: The routine enters an infinite loop that prevents further execution of subsequent instructions.
- `execute_invalid_instruction_5`: The routine contains data that does not correspond to a valid instruction, causing the processor to encounter an illegal opcode.
- `nop_padding_sequence_4`: This routine is empty and consists of a single instruction that serves as padding in the executable memory.
- `enter_infinite_loop_10`: The routine executes an infinite loop that prevents further instruction flow.
- `nop_padding_sequence_7`: This routine is empty and consists of a single instruction that serves as padding between other code segments.
- `execute_infinite_loop_12`: The routine performs an unconditional jump to itself, resulting in an infinite loop.
- `execute_infinite_loop_13`: This routine enters an infinite loop that prevents further execution of subsequent code.
- `stub_bad_instruction_data_2`: The bytes at this address do not decode as valid 68000 instructions, so the routine is a stub with no behaviour of its own.
- `spin_in_empty_loop`: The routine loops unconditionally with an empty body and never returns, so it only idles.
- `halt_execution`: The routine contains no valid instructions and terminates execution immediately.
- `null_instruction_trap`: The routine contains no valid instructions and terminates execution immediately upon entry.
- `execute_invalid_instruction_4`: Encounters data that cannot be decoded as valid instructions.
- `execute_infinite_loop_14`: This routine enters an infinite loop that prevents further execution of the current thread.
- `enter_infinite_loop_11`: Stays in a continuous loop.
- `nop_padding_sequence_5`: This routine is empty and consists of a single instruction that serves as padding within the executable segment.
- `execute_infinite_loop_15`: This routine enters an unconditional infinite loop that prevents further execution of the program flow.
- `nop_padding_segment`: This routine is empty and consists of unused bytes used for alignment or padding within the executable.
- `null_operation_routine_4`: This routine contains no executable instructions and performs no actions.
- `return_from_subroutine_2`: The routine consists of a single instruction that returns control to the calling routine.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `nop_instruction_2`: The routine contains a single instruction that does not perform any operation.
- `empty_routine_8`: This routine is empty and contains no valid instructions.
- `nop_padding_byte`: This routine consists of a single non-operational instruction serving as a data gap in the executable segment.
- `empty_instruction_sequence`: The routine contains no valid executable instructions.
- `nop_padding_sequence_6`: This routine is empty and consists of a single null instruction used as padding between other executable segments.
- `execute_invalid_instruction_7`: The routine consists of a single byte of bad instruction data.
