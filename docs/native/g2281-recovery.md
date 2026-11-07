# Game 2281 (star wars - return of the jedi (1988)(domark)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02281). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (914224 top-level bytes in 36 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 1966 raw-word lines |
| Functions named and described | pass, 455 of 455 named (0 author symbols), 455 of 455 described (17 by rule, 108 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1871 of 1919 (97.50%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11749 of 11932 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 343866 |
| code | 38202 |
| data | 93832 |
| padding | 8244 |
| unused | 430080 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:run | 2 | 117 | 2 |
| ex | 2 | 117 | 2 |
| type | 2 | 117 | 2 |
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:jedi | 28 | 7178 | 327 |
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:add21k | 2 | 26 | 1 |
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:wait | 2 | 117 | 2 |
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:nofastmem | 9 | 306 | 14 |
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:bootblock | 1 | 37 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 867 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 985 | 67 |
| logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf | 1 | 1235 | 7 |
| logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf | 1 | 43 | 2 |
| logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf | 1 | 787 | 2 |

## Cases handled

- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 605 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- star wars - return of the jedi (1988)(domark)[cr qtx]/Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf: 1120 executed PCs whose RAM was overwritten before any snapshot match its bytes 287776-290480: recovered as a unit run at an address
- star wars - return of the jedi (1988)(domark)[cr qtx]/Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf: 18 executed PCs whose RAM was overwritten before any snapshot match its bytes 466080-466400: recovered as a unit run at an address
- star wars - return of the jedi (1988)(domark)[cr qtx]/Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf: 92 executed PCs whose RAM was overwritten before any snapshot match its bytes 288208-290352: recovered as a unit run at an address
- star wars - return of the jedi (1988)(domark)[cr qtx]/Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf: 3 logged loads cover 0 bytes of the image
- star wars - return of the jedi (1988)(domark)[cr qtx]/Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `start_audio_channel_playback` (jedi): Selects one of four audio channels, loads its sample pointer, length, period, and volume, and enables its DMA and interrupt bits. Kind: track/sync check (frequency-dependent/timing-based check). Pass path: the loop completes (sVar3 reaching -1) and the routine proceeds to update hardware registers with the valid or dummy-calculated values.
- `configure_dma_and_interrupts` (runtime segment an address): Sets a bitmask in a global variable and configures hardware registers for DMA and interrupts using a loop-based delay or count. Kind: track or sync check (track-dependent DMA/interrupt timing). Pass path: the branch where sRam00dff09e (and associated DMA/interrupt registers) is configured based on the masked d2 value (uVar2) without triggering an invalid state or a trap.

## Functions


### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longwords_then_call`: Copies sixteen longwords from a fixed source block into the buffer when one is supplied, then calls another routine through a jump pointer.

### ex

- `compute_max_and_build_header`: Walks a linked list of entries to find the largest stored field value, starting from a floor of 149.
- `copy_memory_block`: copies a fixed number of words from a source pointer to a destination pointer.

### type

- `find_max_and_initialize_header_buffer`: Traverses a linked list to find the maximum value in a specific field, then writes a header and a sequence of incrementing longwords into a buffer.
- `copy_stack_from_memory`: copies a fixed number of words from a memory location into a stack-based buffer.

### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:jedi

- `accumulate_table_then_save_state`: Walks a table of entries and adds a rotated byte value into memory for each nonzero entry, then saves the register set to the stack and copies thirteen 16-bit words from code into a fixed low-memory address.
- `accumulate_table_then_save_state_2`: Walks a table of entries and adds a rotated byte value into memory for each nonzero entry, then saves the register set to the stack and copies thirteen 16-bit words from code into a fixed low-memory address.
- `count_down_busy_wait`: Counts the input value down to zero in a tight loop and then returns, serving as a busy-wait delay.
- `copy_and_zero_pad_buffer`: Copies longwords from one buffer into another up to a limit, zero-fills the destination up to a second limit, then stores the register values into fixed memory and returns.
- `run_state_machine_loop`: Runs a large endless loop that sets and tests a shared state variable and makes calls to many other routines depending on its value.
- `cycle_state_machine_steps`: Repeatedly tests a shared state counter and makes calls for each state, resetting the counter at the value eight.
- `run_nested_state_loop`: Runs an endless loop that resets a shared state, makes a sequence of calls and steps through a nested state loop that exits when the counter reaches eight.
- `run_timed_countdown_sequence`: Starts a counter at 500 and decrements it in a loop that clamps at zero, making calls on each pass.
- `reset_state_then_call_three`: Clears a group of shared globals, sets one of them to 3, and makes three calls to other routines.
- `wait_until_global_reaches_four`: Spins until a shared global reaches at least four, then resets it to zero and returns.
- `poll_status_bit_with_timeout`: Runs a nested countdown that stops early if the high bit of a status global is set, then waits until that bit is clear before returning.
- `wait_status_bit_clear`: Waits while the high bit of a status global is set, then returns.
- `run_timed_state_sequence`: Sets a shared counter to twelve and runs a state loop that makes several calls on each pass.
- `run_countdown_then_return_status`: Makes several calls, then either runs an eighty-step countdown with calls on each step and returns minus one, or returns a status value taken from a called routine or the shared state.
- `loop_calls_until_limit`: Makes five calls to other routines and increments a shared counter, repeating while the counter stays above negative 385.
- `run_countdown_when_active`: If a shared state is nonzero, makes four calls and then runs an eighty-step countdown with several calls on each step, before returning.
- `run_eighty_step_countdown_2`: Makes four calls, then runs an eighty-step countdown with several calls on each step, ending when the counter passes zero.
- `run_state_sequence_from_two`: Starts a shared state at two and runs a state loop that makes calls on each pass, returning early when a called routine reports a nonzero result or when the state reaches an end value.
- `run_state_sequence_loop`: Repeats a set of calls and steps a shared state counter, returning when the counter goes negative or a called routine reports a nonzero result.
- `run_state_sequence_until_target`: Repeats the state sequence until the shared state matches a target value, which depends on whether the first input equals two, and returns at that point or when the counter goes negative.
- `run_state_sequence_to_target`: Repeats the state sequence until the shared state reaches a value derived from the input, returning at that point or when the counter goes negative.
- `step_state_sequence_with_exit`: Steps a shared state counter with calls on each pass, returning when the counter goes negative, when a called routine reports a nonzero result or when the state reaches an end value.
- `step_state_until_zero`: Makes calls and steps a shared state counter, looping until the state reaches zero.
- `step_state_counter_loop`: Steps a shared state counter with calls and repeats until it reaches zero.
- `run_state_sequence_with_reset`: Runs the shared state sequence with several calls per step while decrementing the counter.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `advance_counter_table_lookup`: Increments a shared counter and replaces it with a byte from a small table indexed by bits of the counter.
- `print_zero_terminated_text`: If the first byte of a shared text buffer is empty, it stores the digit zero there.
- `format_digit_and_print_text`: Adds the digit-zero offset to a shared byte, calls one routine, then passes each character of the zero-terminated text at that global to another routine until the terminator.
- `print_text_at_fixed_address`: Calls one routine, then passes each character of the zero-terminated text at a fixed low memory address to another routine until the terminator.
- `print_text_at_second_address`: Makes two calls, then passes each character of the zero-terminated text at a second fixed low memory address to another routine until the terminator.
- `finish_state_with_conditional_calls`: Makes five calls, then for shared state values three or five makes two more calls each.
- `finish_state_if_five`: When the input byte equals five it makes two calls.
- `print_text_after_two_calls`: Makes two calls, then passes each character of the zero-terminated text at a fixed low memory address to another routine until the terminator.
- `wait_then_spin_forever`: If a shared flag is nonzero, waits until it clears and then enters an endless loop that never returns.
- `spin_forever_after_wait`: Waits until a shared flag reads zero and then spins in an endless loop without returning.
- `call_with_saved_global_value`: Writes its input into a shared global and makes two calls, keeping the global's value from after the first call and restoring it at the end.
- `copy_32000_byte_block`: Makes two calls, copies a 32000-byte block of memory from one buffer to another in 4000 double-longword steps, then makes five more calls.
- `finish_with_three_calls`: Makes three calls to other routines, then sets a shared state to minus one.
- `call_chain_if_state_nine`: If the shared state equals nine it makes two calls, otherwise one, then makes ten more calls.
- `make_eleven_calls`: Makes eleven calls to other routines in sequence and returns.
- `make_seven_calls`: Makes seven calls to other routines in sequence and returns.
- `make_long_call_sequence`: Makes a long fixed sequence of calls to other routines and then returns.
- `copy_block_after_ten_calls`: Makes ten calls, then copies a 32000-byte block of memory from one buffer to another in 4000 double-longword steps.
- `save_global_around_two_calls`: Makes one call, saves a shared global, makes two calls, then restores the saved global value.
- `print_zero_terminated_string_arg`: Makes one call, then passes each character of the zero-terminated text at the address it was given to another routine until the terminator.
- `do_nothing_return`: Does nothing and returns at once.
- `pause_between_calls`: Makes one call, runs a full 16-bit countdown as a delay, then makes two more calls.
- `clear_and_copy_strided_block`: Clears an 8000-longword block of memory, makes one call, then makes a run of calls.
- `clear_block_and_make_calls`: Clears an 8000-longword block of memory, makes one call and then a long series of calls.
- `compute_pixel_offset`: Computes a linear pixel offset from certain arguments and stores it in a global, then increments and rescales a global pointer.
- `do_nothing_routine`: the routine is empty and performs no operations.
- `copy_self_block_then_calls`: Makes two calls, runs a 10752-longword copy loop whose decompiled source and destination are the same global so that, as shown, it changes nothing, then makes eleven more calls.
- `fill_block_pattern_after_calls`: Makes several calls, one of them given a buffer offset of 8192 bytes, then fills a 32000-byte block with a repeating four-longword pattern read from low memory.
- `composite_masked_plane_rows`: Combines source and destination over a block of rows: for each pair of bytes the destination becomes the source OR the destination bits selected by a fourth mask plane.
- `poll_until_zero_twice`: Calls one routine repeatedly until it returns zero, then repeats that polling once more before returning.
- `execute_iterative_calls`: calls another routine repeatedly until it returns a non-zero value, then repeats the process once more.
- `copy_four_buffer_longwords`: Copies a run of longwords into four buffers spaced 10752 bytes apart.
- `deinterleave_four_planes`: Splits each group of four source bytes across four destination buffers spaced 10752 bytes apart.
- `copy_planes_between_strides`: Makes two calls, then copies 192 rows of ten-longword groups across four planes, reading them 8000 bytes apart and writing them 10752 bytes apart.
- `copy_table_selected_tiles`: Makes two calls, picks one of three lookup tables by a shared value, then for each nonzero table entry copies a sixteen-longword group from a source buffer across four plane offsets.
- `copy_tiles_by_table_choice`: Selects one of two lookup tables by whether the input equals one, then for each nonzero table entry copies a sixteen-longword group from a source buffer across four plane offsets.
- `copy_tiles_from_fixed_table`: Uses one fixed lookup table, then for each nonzero entry copies a sixteen-longword group from a source buffer across four plane offsets.
- `clear_rows_of_longwords`: Zeroes 192 rows of 56 longwords each, advancing a buffer pointer continuously through memory.
- `poll_twice_after_two_calls`: Makes two calls, then polls one routine until it returns zero and repeats that poll once more before returning.
- `swap_plane_bytes_via_table`: Exchanges bytes between two regions across four planes, passing each byte through a small lookup table before it is stored.
- `copy_planar_block_rows`: Copies 108 rows of 24 bytes in each of four planes from a fixed source to a destination, reading the planes 8000 bytes apart and writing them 10752 bytes apart.
- `build_bit_reverse_table`: Fills a 256-byte table so that each entry holds the bit-reversed value of its index.
- `clear_flags_then_dispatch_five`: Picks a value from a lookup table by a shared index and clears matching flag words in a fixed list.
- `dispatch_five_table_actions`: For each of five bytes from a table selected by a shared index, calls the routine in a jump table that matches the byte's high bits, skipping bytes whose low nibble is zero.
- `dispatch_command_by_state`: Selects a result from a shared state and a command value taken from a table, updating the state and making calls for several command groups.
- `dispatch_command_for_state`: Like its neighbour, selects a result from a shared state and a table command value, making calls for several command groups.
- `dispatch_command_and_copy_record`: Dispatches on a shared state and a table command value, making calls for several command groups.
- `apply_command_to_state`: Applies one of three command codes to a shared state.
- `handle_command_with_record_copy`: Handles command codes against a shared state.
- `handle_three_command_codes`: Handles three command codes against a shared state.
- `handle_command_with_state_steps`: Handles three command codes against a shared state.
- `set_state_to_one`: Sets a shared state to one.
- `set_state_to_one_plain`: Sets a shared state to one, twice in a row, and returns.
- `call_adjust_state_then_call`: Makes one call, adds sixty-four to a shared state, sets the state to three and makes a second call.
- `call_add_offset_then_call`: Makes one call, adds 192 to a shared state, sets the state to two and makes a second call.
- `set_state_and_copy_record`: Makes calls and sets a shared state to two and then to forty-six.
- `reassign_state_to_itself`: Assigns a shared global to itself and returns, so it has no effect.
- `set_state_one_checked`: Sets a shared state to one, then runs a conditional that cannot succeed in the decompiled logic, so the state stays at one.
- `reset_state_to_two`: If a shared state is zero it makes one call.
- `set_state_one_after_call`: If a shared state is nonzero it sets it to one and to 256 and makes one call.
- `load_state_from_table_entry`: Loads a shared state from a table entry selected by the current state value, with the index scaled by four.
- `copy_state_record_to_slot`: Copies a nineteen-word record starting at the shared variable block into the first free slot of the record table.
- `initialise_record_and_copy`: Clears the shared state, makes several calls, writes two 32-bit values and one 16-bit value into a structure addressed by an address register, then copies a nineteen-word record from a fixed source into the first free slot.
- `copy_record_if_state_matches`: Resets several shared globals and makes calls.
- `do_nothing`: This routine is empty and performs no operations.
- `cycle_three_entries_from_value`: Recomputes a shared index from a shared value, then steps a three-entry cycle when a range check passes, loading the selected entry and making one call.
- `cycle_three_entries_offset`: Sets a shared index from the low five bits of its value plus fifty-three, then steps a three-entry cycle under a range check, loading the chosen entry and making one call.
- `cycle_three_entries_if_zero`: If a shared index is zero and a range check passes, steps a three-entry cycle, loads the selected entry and makes one call.
- `cycle_three_entries_on_range`: When a shared index is in a small range, steps a three-entry cycle, loads the selected entry and makes one call.
- `cycle_three_entries_on_input`: When the input is zero or one, steps a three-entry cycle, loads the selected entry and makes one call.
- `cycle_three_entries_always`: Steps a three-entry cycle, loads the selected entry and makes one call.
- `copy_record_when_state_zero`: If a shared state is zero, copies a nineteen-word record from a fixed source into the first free slot of the record table.
- `copy_to_target_slot`: searches a table for a slot with an even or otherwise valid bit pattern and copies a fixed number of words from a source address to that slot.
- `copy_fixed_record_to_slot`: Copies a nineteen-word record from a fixed source into the first free slot of the record table, leaving the table unchanged when no slot is free.
- `reset_and_copy_data`: updates several global variables and then copies a fixed number of words from a source address to a slot found in a table.
- `cycle_entries_then_copy_record`: When a shared state is nonzero and in a small range, steps a three-entry cycle, loads the selected entry and copies a nineteen-word record from a fixed source into the first free slot.
- `cycle_entries_and_copy_record`: When a shared state is in a small range, steps a three-entry cycle, loads the selected entry and copies a nineteen-word record from a fixed source into the first free slot.
- `cycle_entries_on_input_copy`: When the input is zero or one, steps a three-entry cycle and copies a nineteen-word record from a fixed source into the first free slot.
- `cycle_entries_always_copy_record`: Steps a three-entry cycle, loads the selected entry and copies a nineteen-word record from a fixed source into the first free slot.
- `copy_second_record_to_slot`: Copies a nineteen-word record from a second fixed source into the first free slot of the record table.
- `copy_record_if_state_matches_2`: Resets several shared globals, makes calls, and if the shared state equals 2166 copies a nineteen-word record from a fixed source into the first free slot.
- `copy_record_if_state_matches_3`: Makes calls and, if the shared state equals 2166, copies a nineteen-word record from a fixed source into the first free slot.
- `set_state_then_early_return`: Sets a shared state through several values and makes a call.
- `copy_record_when_state_set`: If a shared state is nonzero, sets it from the input, makes a call and copies a nineteen-word record from a fixed source into the first free slot.
- `set_state_copy_record`: Sets a shared state from the input, makes a call and copies a nineteen-word record from a fixed source into the first free slot.
- `toggle_offsets_and_copy_record`: When a shared state is nonzero, toggles it between zero and one, sets fixed memory values for each of two calls, then copies a nineteen-word record from a fixed source into the first free slot.
- `toggle_offsets_copy_record_plain`: Toggles a shared state between zero and one, sets fixed memory values for each of two calls, then copies a nineteen-word record from a fixed source into the first free slot.
- `set_offsets_and_copy_record`: Sets a fixed memory value for each of two calls, then copies a nineteen-word record from a fixed source into the first free slot.
- `configure_calls_by_state`: Sets a shared state, makes calls and writes fixed memory values depending on the state, with a final call when the state is two or three.
- `call_when_input_is_three`: Makes one call only when its input equals three.
- `advance_state_and_step_values`: Advances a shared state and, depending on its new value and whether the previous value was zero, sets a short sequence of fixed values, making a call for each and wrapping the sequence after a few steps.
- `step_values_by_input_flag`: Uses a shared state and an input flag to choose among a short sequence of fixed values, making a call for each one and wrapping the sequence after a few steps.
- `set_value_then_wrap_call`: Sets a shared value to sixty and makes a call.
- `step_values_by_two_inputs`: Chooses among a short sequence of fixed values using two input values, making a call for each chosen value and wrapping the sequence after a few steps.
- `set_negative_value_wrap_call`: Sets a shared value to minus sixty and makes a call.
- `set_value_by_input_wrap`: If the input is zero, sets a shared value to minus sixty and makes a call.
- `set_forty_then_wrap_call`: Sets a shared value to forty and makes a call.
- `forward_to_one_subroutine`: Does no work of its own and only calls one other routine before returning.
- `forward_to_subroutine`: calls another routine before returning.
- `forward_subroutine_call`: This routine acts as a jump or pass-through, immediately calling another routine before returning.
- `forward_to_subroutine_2`: This routine acts as a jump or pass-through stub that calls another routine and returns.
- `forward_subroutine_call_2`: This routine calls another routine before returning.
- `set_state_then_two_calls`: Sets a shared value to 1088 and makes two calls to other routines.
- `make_two_calls`: Makes two calls to other routines and returns.
- `mask_input_or_call_result`: Combines its input with a fixed mask of 223 and, if a shared value is nonzero, returns the result of a call instead.
- `make_three_calls`: Makes three calls to other routines in sequence and returns.
- `advance_and_draw_tile_rows`: Advances a shared counter up to eleven and makes a call.
- `draw_tile_rows_across_planes`: Copies sixteen tile rows from fixed graphics data into each listed buffer position across four planes spaced 8000 bytes apart, filling the remaining positions with a second tile.
- `increment_state_by_one`: Adds one to a shared value and returns.
- `add_three_bcd_then_calls`: Adds three to a shared value, applies a decimal (BCD) adjustment to it, then makes six calls to other routines.
- `make_six_calls`: Makes six calls to other routines in sequence and returns.
- `execute_five_calls`: Calls five different routines in sequence and then returns.
- `make_five_calls`: Makes five calls to other routines in sequence and returns.
- `make_four_calls`: Makes four calls to other routines in sequence and returns.
- `execute_plane_copy`: Executes a sequence of three routine calls used for copying plane strips.
- `copy_plane_strips`: Performs a plane copy and copies a row strip from a table entry into four or more planes.
- `invoke_plane_copy`: Calls a routine that copies twelve bytes across four or more planes.
- `add_bcd_triple_then_calls`: Adds a three-byte packed decimal input into a shared value, applying a decimal adjustment after each byte, then makes seven calls.
- `execute_unnamed_sequence`: Executes a sequence of routine calls.
- `copy_row_strips`: Executes a sequence of routine calls that copy bitplane row strips from a table entry.
- `execute_row_copy`: Executes a sequence of routine calls that copy bitplane row strips or plane-spaced row strips from a table.
- `execute_row_copy_sequence`: Executes a sequence of two routine calls responsible for copying bitplane row strips or copying plane-spaced row strips from a table entry.
- `execute_plane_row_copy`: Executes a sequence of two routine calls used for copying row strips across bitplanes.
- `copy_plane_strips_2`: copies a strip of memory or a row of data across multiple bitplane-like offsets.
- `trigger_plane_copy`: Executes a routine that copies memory strips between different planes.
- `make_call_pairs_loop`: Makes pairs of calls in a loop whose conditions compare a shared value with itself, so as decompiled the loop never exits.
- `make_call_pairs_forever`: Makes pairs of calls in a loop whose exit test compares a shared value with itself, so as decompiled it never exits.
- `copy_four_plane_row_strip`: Copies twelve bytes in each of four planes spaced 8000 bytes apart, stepping forty bytes at a time, from a table entry chosen by its input offset.
- `step_index_place_entries_row`: Advances a shared index through a four-entry table, then places entries along a row in steps of sixteen.
- `place_entries_row_from_input`: Sets a shared index from its input, advances through a four-entry table, then places entries along a row in steps of sixteen.
- `place_entries_row_from_state`: Advances a shared index through a four-entry table, then places entries along a row from the input start position in steps of sixteen.
- `place_entries_row_from_inputs`: Places entries along a row starting at the input horizontal position in steps of sixteen, with the vertical position stepping by eight.
- `return_nothing`: The routine is empty and performs no operations before returning.
- `process_records_by_row`: Walks the 19-word record table once for each row index from sixteen down to zero.
- `walk_records_by_countdown`: Walks the 19-word record table repeatedly while counting a stack value down.
- `dispatch_matching_table_entries`: Walks a table of 38-byte records from its first record, skipping inactive ones, and calls a routine for each active record whose index field matches the current counter value.
- `step_position_nine_times`: Saves a 16-bit position field of the object, calls a routine nine times while advancing that field by 16 each time, then writes the original value back.
- `draw_masked_sprite_from_object`: Computes the screen position of a sprite object from its coordinate fields, returns if it is off-screen, and clips the top and left edges.
- `draw_masked_sprite_stored_pos`: Recomputes the clipped screen coordinates of a sprite and stores them back into the object record.
- `draw_masked_sprite_cached_pos`: Draws the clipped masked sprite using screen coordinates already stored in the object record.
- `clear_clipped_masked_region`: Clips a rectangle whose position and size are passed in registers, then ANDs the shifted mask plane into four bitplanes to clear the covered pixels.
- `clear_clipped_masked_region_raised`: Clips a rectangle passed in registers with its top edge moved up one row, then ANDs the shifted mask plane into four bitplanes to clear the covered pixels.
- `clear_wrapped_masked_strip`: Clips a one-row-high vertical strip whose horizontal position wraps around the screen edge, then clears the covered pixels across four bitplanes using the shifted mask plane.
- `draw_masked_sprite_block`: Draws a block of sprite data into four bitplanes by ORing the source planes into the screen after clearing the masked pixels with the fourth plane.
- `clear_masked_strip_rows`: Clears a strip of rows in four bitplanes using the shifted mask plane, with the same clipping and row stepping as the other masked blitters but without the wrap-around column.
- `blit_clipped_sprite_rows_shifted`: Clips a sprite rectangle to the screen edges and computes a sub-word pixel shift, then walks its rows.
- `blit_sprite_rows_unclipped`: Draws a sprite block into four bitplanes without edge clipping, applying a pixel shift and a row stride taken from the caller's registers.
- `draw_masked_sprite_rows_aligned`: Draws an unshifted, word-aligned masked sprite block into four bitplanes.
- `fill_rectangle_with_plane_colors`: Fills a rectangle in four bitplanes with constant plane values held in registers, combining each word with the mask so only the selected pixels change.
- `draw_byte_aligned_sprite_rows`: Copies byte-aligned sprite data into four bitplanes, using AND and OR masks drawn from the extra mask bytes of the source, and advances two bytes per column.
- `draw_masked_sprite_rows_shifted`: Draws a masked sprite block into four bitplanes without clipping, shifting each source word right by the bit count held in a register.
- `draw_masked_rows_reverse_shift`: Draws a masked sprite block into four bitplanes with a pixel shift computed as sixteen minus the bit count, so the shift runs in the opposite direction to its sibling.
- `copy_sprite_columns_from_table`: Copies a column-major block of four-plane sprite words from a fixed source offset into the screen.
- `copy_sprite_block_with_mask`: Copies a block of four-plane sprite words into the screen, writing directly where the fourth-plane mask is zero and merging with the destination elsewhere.
- `fill_rows_from_plane_colors`: Writes constant plane values into a rectangle of four bitplanes, masked by a register value, and clears the fourth plane for each row.
- `merge_sprite_rows_bytewise`: Merges sprite rows into four bitplanes one byte at a time, ANDing and ORing against byte masks taken from the extra planes of the source.
- `blit_masked_rows_shift_count`: Draws a masked sprite block into four bitplanes with a sub-word shift set by a register, applying the mask and data words to each plane.
- `blit_masked_rows_reverse_count`: Draws a masked sprite block into four bitplanes with the shift computed as sixteen minus the register value, so it shifts in the opposite direction to its sibling blitter.
- `call_two_routines_then_library`: Calls two runtime routines and then a library function reached through the library base stored in a global variable, at a fixed offset.
- `call_two_routines_then_library_2`: Calls two runtime routines and then a library function reached through the library base stored in a global variable, at a fixed offset.
- `call_six_routines_then_library`: Calls six runtime routines in sequence and then a library function reached through the library base stored in a global variable.
- `unpack_run_length_stream`: Decodes a run-length packed byte stream into a global buffer.
- `call_five_routines_then_library`: Calls five runtime routines in sequence and then a library function reached through the library base stored in a global variable.
- `call_three_routines_then_library`: Calls three runtime routines in sequence and then a library function reached through the library base stored in a global variable.
- `call_three_routines_then_library_2`: Calls three runtime routines in sequence and then a library function reached through the library base stored in a global variable.
- `unpack_run_length_mode_one`: Stores the selector value one in a global, calls a routine, and then decodes a run-length packed stream into the global buffer.
- `unpack_run_length_mode_two`: Stores the selector value two in a global, calls routines, and then decodes a run-length packed stream into the global buffer.
- `unpack_run_length_mode_three`: Stores the selector value three in a global, calls routines, and then decodes a run-length packed stream into the global buffer.
- `unpack_runs_into_buffer`: Decodes a run-length packed stream from the global source pointer into a caller-supplied buffer, using the same marker byte of 178 and the same counts.
- `fill_bytes_then_decode_runs`: Writes a repeated byte value into the output a set number of times, then decodes a run-length packed stream, stopping when a count of eight is reached.
- `decode_runs_after_seed_byte`: Writes a seed byte, then decodes a run-length packed stream from the source, copying literal bytes and repeating marked values until the count of eight is reached.
- `stub_return_only`: Does nothing and returns immediately.
- `init_four_audio_channels`: Writes default period, length, and volume values to the four Paula audio channel registers, then sets the global state fields and the audio enable value.
- `wait_for_global_flag_clear`: Sets a global flag to one, calls a routine, then busy-waits until the flag returns to zero, which an interrupt routine clears when it finishes.
- `set_state_five_audio_irqs`: Sets a state global to five and then to one, writes the audio interrupt enable bits to the interrupt control register, and finally stores fifteen in the state global.
- `enable_audio_interrupts_if_idle`: If the state global is zero, sets it to eight, sets it to one, writes the audio interrupt enable bits to the interrupt control register, and stores fifteen in the state global.
- `return_from_routine`: the routine is empty and performs no operations before returning.
- `set_state_three_audio_irqs`: Sets a state global to three and then to one, writes the audio interrupt enable bits to the interrupt control register, and finally stores fifteen in the state global.
- `set_state_two_audio_irqs`: Sets a state global to two and then to one, writes the audio interrupt enable bits to the interrupt control register, and finally stores fifteen in the state global.
- `set_state_one_audio_irqs`: Sets a state global to one, writes the audio interrupt enable bits to the interrupt control register, and stores fifteen in the state global.
- `store_pointer_then_run_five`: Stores its pointer argument in a global, then calls five runtime routines in sequence.
- `initialise_system_libraries_and_memory`: Zero-fills the global data area, looks up the current task and clears its signal field, opens the library bases it needs, and allocates a memory block.
- `install_interrupt_server_then_halt`: Calls the operating system to install an interrupt server through the library base, then loops forever.
- `setup_playfield_display_registers`: Sets the display window, data fetch, and bitplane modulo registers, then starts the copper list and writes the playfield control register after a delay loop.
- `setup_copper_plane_pointers`: Writes the high and low words of four bitplane addresses into copper list entries spaced 8000 bytes apart.
- `call_four_routines_in_sequence`: Calls four runtime routines in order and returns, with no other work.
- `poll_library_until_ready`: Repeatedly calls a library function until it returns a nonzero result.
- `store_argument_to_global`: Stores its register argument in a global variable.
- `dispatch_global_handler`: Calls a handler stored in a global variable, updates that global variable if a certain value is returned, or loops indefinitely if a certain value is returned.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_global_handler_then_halt`: Calls the handler pointer stored in a global variable and then loops forever.
- `cycle_state_and_run_handlers`: Advances a three-state counter global, resets it when it reaches one, calls the stored handler pointer, runs three runtime routines, and sets the state back to one.
- `clear_four_globals`: Sets four global variables to zero.
- `encode_direction_bits_clamped`: Unreached fragment that decodes as a branch-heavy bit encoder.
- `encode_direction_bits_variant_a`: Unreached fragment that decodes as a variant of the direction bit encoder.
- `encode_direction_bits_register`: Unreached fragment that decodes as the direction bit encoder working on a register value.
- `encode_direction_bits_register_b`: Unreached fragment that decodes as the direction bit encoder working on a register value with a different entry test.
- `encode_direction_bits_partial`: Unreached fragment that decodes as a shorter direction bit encoder.
- `encode_direction_bits_global_only`: Unreached fragment that decodes as a direction bit encoder reading only a global.
- `encode_direction_bits_argument_low`: Unreached fragment that decodes as a direction bit encoder driven by a register argument and a global.
- `encode_direction_bits_argument_high`: Unreached fragment that decodes as a direction bit encoder driven by a register argument and a global.
- `subtract_ten_clamp_at_zero`: Subtracts ten from a register value, clamps the result at zero, and stores it in a global.
- `subtract_ten_store_clamped`: Subtracts ten from a register value, clamps the result at zero, and stores it in a global.
- `decode_joystick_port_direction`: Reads the joystick port one movement register and builds a byte of direction flags, covering the four directions and a flag bit in the upper half, which it stores in a global.
- `decode_joystick_direction_from_argument`: Builds the direction flag bits from a raw joystick argument and stores them in a global, adding the flag bit when the global's state bit is clear.
- `read_fire_button_bits`: Reads the fire button state from the CIA port register bit and sets a flag bit in a register accordingly, then stores the result in a global.
- `compute_pointer_movement_deltas`: Computes horizontal and vertical pointer movement from the previous and current counter values, storing each delta clamped to a limit in a global.
- `read_right_and_middle_buttons`: Reads the right and middle mouse button states from the potentiometer port registers and stores them as bits in a global.
- `compute_mouse_port_movement_deltas`: Reads the mouse port movement register and computes horizontal and vertical deltas from the previous counter values, storing each one clamped to a limit in a global.
- `clamp_floor_and_subtract_step`: Raises a register value to at least the global floor, stores it, then subtracts a step value and stores the clamped difference back in the global.
- `clamp_value_to_floor_global`: Raises a register value to at least the global floor and stores the result in the global.
- `set_global_to_minus_one`: Stores the value minus one in a global variable.
- `wait_until_global_zero`: Busy-waits until a global variable becomes zero, which an interrupt or routine clears when it finishes.
- `update_bit6_from_low_nibble`: If the low four bits of the argument differ from the stored value, updates the stored value and sets or clears bit 64 for each of those four bits.
- `store_bit6_from_low_nibble`: Stores the argument in a global and sets or clears bit 64 according to each of its low four bits.
- `update_indexed_bit_from_flags`: Sets or clears bits in a global selected by the low three bits of a register, based on the flag bits of an argument.
- `update_indexed_bit_flags_b`: Sets or clears bits in a global selected by the low three bits of a register, based on the upper flag bits of an argument.
- `update_indexed_bit_flag_three`: Sets or clears one bit in a global selected by the low three bits of a register, based on the fourth flag bit of an argument.
- `clear_indexed_bit_in_global`: Clears one bit in a global selected by the low three bits of a register.
- `clear_pixel_bit_four_planes`: Works out the byte address for a screen position from x and y registers, then clears the matching bit in each of the four bitplanes.
- `clear_bit_four_plane_bytes`: Clears one bit in four bitplane bytes at an address held in a pointer register, with the bit number taken from a register.
- `fill_bitplane_after_routines`: Calls four runtime routines, then fills one 8000-byte bitplane by repeating a four-long pattern read from fixed low memory locations, writing forty longs per row for 200 rows.
- `fill_bitplane_with_pattern`: Fills one 8000-byte bitplane by repeating a four-long pattern read from fixed low memory locations, writing forty longs per row for 200 rows.
- `call_single_routine`: Calls one runtime routine and returns.
- `copy_eight_bytes_to_buffer`: Copies eight bytes from the argument pointer into a global buffer, one byte at a time.
- `copy_eight_bytes_within_buffer`: Copies eight bytes from the global buffer to itself, one byte at a time.
- `call_single_routine_2`: Calls one runtime routine and returns.
- `wait_for_blitter_then_finish`: Calls five runtime routines, waits until the blitter busy flag clears, then calls one more routine.
- `start_blitter_rectangle_copy`: Waits for the blitter to become free, sets its source and destination pointers, modulo values, first and last word masks, and control registers, then starts a rectangle operation sized from the caller's width and height.
- `forbid_multitasking_then_library_call`: Calls the operating system's Forbid routine, then a second library routine through the library base in a global.
- `permit_multitasking_then_library_call`: Calls the operating system's Permit routine, then a second library routine through the library base in a global.
- `clear_plane_word_columns`: Calls a routine, then zeroes one word in each of four bitplane regions for nine rows, repeating the clear at a second offset in the same regions.
- `clear_strided_plane_words`: Zeroes one word in each of four bitplane regions for twenty-four rows, stepping through the region by a fixed stride.
- `set_state_four_after_reset`: Stores one, then zero, then four in a state global variable.
- `set_state_eight_after_reset`: Stores zero, then zero again, then eight in a state global variable.
- `set_state_minus_eight`: Stores zero in a global, then stores minus eight in it.
- `compute_screen_offset_mask`: Splits a position into a remainder within a 192-row band, works out a bit index, and looks up a table value.
- `compute_offset_with_state_update`: Doubles a state global when it is set, updates two state flags from its sign, then computes a combined screen offset and table value as its sibling does.
- `clear_global_value`: Sets a global variable to zero.
- `compute_offset_mask_alt`: Computes a combined screen offset and table value from a position, with the same band split and row length as its sibling, but without the state update.
- `store_linear_position_offset`: Stores a table value from a register index, then stores a linear offset made from the x and y registers and the row length of 56.
- `call_routine_if_state_set`: Calls one runtime routine when a state global is nonzero, and calls it a second time when a second state global is nonzero.
- `wait_blitter_then_finish_b`: Calls five runtime routines, waits until the blitter busy flag clears, then calls one more routine.
- `start_blitter_copy_with_shift`: Waits for the blitter to become free, sets source and destination pointers and masks, sets the modulo values, and writes the control register using a shift value from a global before starting the operation.
- `compute_linear_pixel_offset`: Stores the x and y arguments and computes a linear pixel offset of x plus y times 320 into a global.
- `copy_glyph_rows_to_screen`: Copies eight bytes from a glyph table entry down a screen column with a 40-byte row stride, then advances the destination pointer by one byte.
- `advance_scaled_pointer_after_routines`: Calls two runtime routines, increments a global pointer, then rescales it by 322 using shifts and adds.
- `advance_scaled_pointer_after_pair`: Calls two runtime routines, increments a global pointer, then rescales it by 322 using shifts and adds.
- `fold_value_into_first_band`: Subtracts 32, 64, or 96 from a register value that lies in one of those 32-wide bands, and returns the result.
- `fold_value_into_band_signed`: Subtracts 32, 64, or 96 from a register value depending on the 32-wide band it lies in, and returns the result as a signed value.
- `subtract_sixty_four`: Returns a register value minus 64.
- `subtract_thirty_two`: Returns a register value minus 32.
- `copy_eight_rows_four_planes`: Copies an eight-row block across four bitplanes, advancing both source and destination by 40 words per row.
- `merge_masked_eight_rows`: Merges eight rows of sprite data into three bitplanes by ORing the source over the destination, with the fourth plane masking which destination bits are kept.
- `set_palette_entry`: Writes a colour value into the palette register selected by an index register.
- `clear_low_memory_block`: Zeroes a block of 532 words starting at the beginning of memory, then stores 32768 in the cell after it.
- `insert_record_first_free`: Scans a table of 19-word records for the first slot whose flag bit is clear, stopping at a terminator record.
- `insert_record_after_current_slot`: Scans forward from the current record in a 19-word table for the next slot whose flag bit is clear, then copies eighteen words from the source into it.
- `tiny_movea_rts`: Small routine of 2 instructions that copies 0 to a0, then returns.
- `call_handlers_for_flagged_records`: Walks a table of 19-word records and calls the handler pointer stored in each record whose flag bit 32 is set, stopping at a terminator record.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `tiny_move_rts`: Small routine of 2 instructions that copies 0 to (a0), then returns.
- `build_and_insert_record`: Fills the position fields of a new record from the source record plus two offsets, clamps a height field to the range zero to sixteen, then copies the record into the first free slot of the table.
- `insert_record_with_clamped_height`: Clamps a height register to the range zero to sixteen, stores it in a new record, then copies the record into the first free slot of the table.
- `store_pair_into_record_fields`: Sets a state global to one, calls a routine, then stores two register values into fields of the record pointed to by a register, and clears the state global.
- `store_pair_after_routine`: Calls a routine, then stores two register values into fields of the record pointed to by a register, and clears a state global.
- `store_pair_after_single_routine`: Calls a routine, then stores two register values into fields of the record pointed to by a register.
- `start_audio_channel_playback`: Selects one of four audio channels, loads its sample pointer, length, period, and volume, and enables its DMA and interrupt bits.
- `call_routine_if_state_clear`: Calls a runtime routine when a state global is zero, and returns without doing anything otherwise.
- `execute_empty_subroutine`: The routine performs no operations and returns immediately.
- `call_routine_if_state_clear_2`: Calls a runtime routine when a state global is zero, and returns without doing anything otherwise.
- `empty_routine_5`: The routine is empty and performs no operations or state changes.
- `call_two_routines_then_conditional`: Calls two runtime routines, then calls a third routine with the result when a state global is zero.
- `conditionally_call_and_store_registers`: Checks if a global variable is non-zero, then calls another routine and stores a pair of registers into memory.
- `store_register_pair`: Calls another routine and then stores two registers into a single memory record.
- `insert_high_score_from_top`: Scans the score table from its top entry, where a sentinel value ends the table, and inserts the score held in a global at the first position where it is larger.
- `format_score_entry_at_pointer`: Writes the three score bytes into a table entry given by a pointer, clears its name field to spaces, sets a marker character, and calls runtime routines to render the line.
- `insert_high_score_from_pointer`: Scans down the score table from an entry pointer, shifting lower records by one and inserting the score held in a global.
- `insert_high_score_scan_pointer`: Scans down the score table from an entry pointer, inserting the score held in a global when it belongs above a lower entry.
- `clear_global_variable`: sets a global variable to zero.
- `format_score_entry_unchecked`: Writes the three score bytes into a table entry given by a register, clears its name field to spaces, sets a marker character, and calls routines to render the line.
- `draw_score_marker_and_render`: Sets a marker character in a table entry, then calls runtime routines to render the line.
- `write_decimal_digit_character`: Writes a digit character derived from a register value by adding the character code for zero.
- `write_zero_character_if_flag`: Writes a zero character to the target byte when a flag argument is nonzero, and returns otherwise.
- `copy_score_record_with_name`: Copies the three score bytes and the name field from the source record into the table entry, then copies the name characters into the record.
- `edit_high_score_name`: Draws an underscore cursor and a row of dots in a fifteen-character name field, then reads key codes in a loop.
- `edit_high_score_name_alt`: Draws an underscore cursor and a row of dots in a fifteen-character name field, then reads key codes in a loop as its sibling does.
- `edit_name_from_key_input`: Stores the first key code at the given name position, then reads further key codes in a loop, with code 8 deleting the previous character, and blanks the cursor and dots when entry ends.
- `blank_name_cursor_and_dots`: Replaces the underscore cursor and dot characters in a name field with spaces.
- `open_library_then_call`: Calls a library function through its base and, if the result is nonzero, saves it, calls a routine, and then calls a second library function.
- `return_empty_routine`: the routine performs no operations and returns immediately.
- `open_library_then_call_2`: Calls a library function through its base and, if the result is nonzero, saves it, calls a routine, and then calls a second library function.
- `find_next_nonempty_text_entry`: Walks a fixed region of text table entries to find the next nonempty entry, and returns the first character of that entry.
- `next_text_table_character`: Advances through a fixed region of text table entries and returns the next character from the current entry, or zero when no entry remains.
- `next_text_table_character_b`: Advances through a fixed region of text table entries and returns the next character, or zero at the end.
- `next_text_table_character_c`: Advances through a fixed region of text table entries and returns the next character, or zero at the end, using a count from a register.
- `find_matching_name_entry`: Compares a stored string against the names held in 26-byte table entries and returns one when a name matches, or zero after the table is exhausted.
- `match_string_against_table`: Compares a stored string against the strings held in 26-byte table entries, moving to the next entry after each mismatch, and returns one on a match.
- `find_matching_name_entry_stepped`: Steps through 26-byte table entries and compares each stored name with a string, returning one when the names match and zero when the table is exhausted.

### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:add21k

- `open_and_release_display_libraries`: Opens the graphics and intuition libraries, sets a flag byte in a structure reached through pointers, rebuilds the display, then frees the raster and closes both libraries.

### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:wait

- `prepare_buffer_largest_entry`: Walks a linked list to find the largest stored value, allocates a buffer sized from it through a system routine, fills the buffer with an ascending word pattern, and copies a sixteen-long header into it.
- `copy_sixteen_longs_to_output`: Copies sixteen longs from a fixed table into the output buffer when one is supplied, then calls a routine.

### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:nofastmem

- `parse_command_line`: Starts the program by reading the command line from the CLI or Workbench startup message, allocates a record, and splits the raw line into null-terminated argument strings in a linked list, honouring quoted strings and star escapes.
- `shutdown_retry_error_window`: Runs the shutdown sequence through exec library calls and retries with a delay while an allocation hook is still in use.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_stored_handler_a`: Calls the handler routine whose address is stored in a data pointer.
- `call_stored_handler_b`: Calls the handler routine whose address is stored in a second data pointer.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.

### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_startup_state_machine`: Runs a loop over a state variable that calls runtime routines and selects the next step, leaving the loop when the final state is reached.
- `initialise_runtime_state_and_return`: Sets several state globals to zero and one to 499, calls runtime routines, and returns the result with bits 132 set.
- `wait_until_state_reaches_four`: Busy-waits until a state global reaches four, then resets it to zero.
- `scan_countdown_for_flag_bit`: Counts a global down from 3500 until its bit 128 is set, then busy-waits until that bit clears.
- `run_state_loop_until_exit`: Loops over runtime routines, stepping a state global through its values and polling a status routine.
- `run_eighty_step_countdown`: Runs a countdown of eighty steps, calling runtime routines at each step, and handles a second loop when the state global is two.
- `run_state_loop_variant`: Runs a loop over a state global that calls runtime routines and steps the state through its values, leaving the loop when a status routine signals completion.
- `run_state_loop_without_setup`: Runs the same state loop as its neighbours without the initial setup, calling runtime routines and stepping the state global until it becomes negative.
- `lookup_state_table_byte`: Steps a counter, looks up a byte from a fixed runtime table using bits of the counter, and sets bit 128 on the result when it is greater than one.
- `print_status_text_string`: Selects one of two fixed text messages depending on a state global, adds a digit to the first character when the state is zero, and outputs each character through a routine until the terminator.
- `print_fixed_text_message_a`: Outputs a fixed text message through a routine, one character at a time, until the terminator.
- `print_fixed_text_message_b`: Calls a routine, then outputs a second fixed text message one character at a time until the terminator.
- `dispatch_state_three_or_five`: Calls routines, then calls one extra routine when the state global is three or five, and finally sets the state to minus one.
- `print_fixed_text_message_c`: Calls a routine, then outputs a third fixed text message one character at a time until the terminator.
- `wait_for_state_then_halt`: Busy-waits until a state global becomes zero, then loops forever.
- `copy_interleaved_bitplane_bytes`: Copies 8000 bytes from the source into the destination in interleaved pairs, writing alternate bytes to the two halves of the destination, then calls routines.
- `run_eleven_routine_sequence`: Calls a routine once or twice depending on a state value, then calls eleven runtime routines in sequence.
- `run_routine_sequence_with_hook`: Calls a routine, then a hook routine, then a fixed sequence of runtime routines.
- `run_long_routine_sequence`: Calls a long fixed sequence of runtime routines with no other work.
- `run_routines_then_interleave_copy`: Calls ten runtime routines, then copies 8000 bytes from the source into the destination in interleaved pairs.
- `delay_loop_between_routines`: Calls a routine, spins a delay loop of about 65000 iterations, then calls two more routines.
- `clear_buffer_copy_tile_rows`: Zeroes an 8000-byte buffer, calls a long series of runtime routines, then copies twenty-seven rows of tile data from two fixed tables into the buffer at interleaved offsets.
- `clear_buffer_run_routines`: Zeroes an 8000-byte buffer, then calls a long series of runtime routines.
- `print_text_from_pointer`: Outputs a null-terminated text string from a pointer through a routine, one character at a time.
- `build_bit_reversal_table`: Fills a 256-byte table so that each entry holds its index with the bit order reversed.
- `update_global_with_table_lookup`: reads a value from a table using an offset derived from a global variable and then sets that global variable to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_two_routines_then_library_3`: Calls two runtime routines and then a library function reached through the library base stored in a global variable.
- `call_two_routines_then_library_4`: Calls two runtime routines and then a library function reached through the library base stored in a global variable.
- `call_six_routines_then_library_2`: Calls six runtime routines in sequence and then a library function reached through the library base stored in a global variable.
- `copy_function_table_data`: Iteratively copies data from a source pointer to a destination pointer, checking for a specific termination or skip pattern.
- `call_five_routines_then_library_2`: Calls five runtime routines in sequence and then a library function reached through the library base stored in a global variable.
- `call_three_routines_then_library_3`: Calls three runtime routines in sequence and then a library function reached through the library base stored in a global variable.
- `initialize_table_and_copy`: Sets a global variable and then copies a range of data from a fixed memory address to a destination pointer, processing specific byte patterns.
- `copy_table_with_offset`: Sets a global variable and copies a range of data from a source pointer to a destination pointer while checking for a specific terminator.
- `process_function_table`: Executes a loop that copies data from a table, resets a global variable, and calls a routine via a pointer or a library function.
- `initialize_audio_registers`: writes several constant values to audio-related registers and certain memory locations, and modifies a global variable.
- `set_audio_volumes`: Sets certain audio volume registers by shifting and masking bytes from a memory range.
- `clear_memory_and_jump`: Clears a range of memory and then performs an indirect jump using a value scaled from a memory location.
- `clear_memory_range_2`: Sets several memory locations and a word in a memory range to zero or a constant value.
- `update_global_state`: Sets a global variable to a specific value based on a register and conditionally increments a global variable if a table lookup matches a certain value.
- `update_global_counter`: Conditionally executes several routines and increments a global variable or checks a table-based condition before incrementing a global variable.
- `check_bit_and_call`: Checks a bit in a memory location and calls another routine if the bit is non-zero.
- `clear_memory_range`: sets several bytes and a word in a memory range to zero or a constant value.
- `increment_and_wrap_global`: increments a global variable and resets it to zero if it matches a value from a register.
- `load_table_to_memory_2`: Loads a word from a table into a memory location calculated using a register and an offset.
- `load_table_to_memory`: loads a word from a table into a memory location calculated using a register and an offset.
- `check_table_and_increment_global`: compares a value in a table or a modified global variable against a register and increments a global variable.
- `update_table_with_calculated_value`: Calculates a value using a table lookup and a register, then updates a series of memory locations using that value and further table lookups.
- `update_table_with_offset_store`: Calculates a value using a table lookup and a register, then updates a series of memory locations at a different offset using that value and further table lookups.
- `update_table_and_registers`: Calculates a bitmask-based value and stores it into a memory range, while also updating several other memory locations based on a table lookup.
- `increment_and_update_table_entry`: increments a byte in a memory range and, if it matches a certain value, updates another memory location using a table lookup.
- `assign_register_to_global`: Stores the value of a register into a global variable.
- `initialize_and_call_functions`: Zeros a memory range, calls several functions via pointers or library calls, and modifies a global pointer with an offset.
- `update_dma_mask`: Checks a global flag and, if set, updates a hardware register with a new bitmask and resets a counter.
- `populate_display_registers`: Writes a sequence of values to a memory range and then copies and shifts data from a source pointer to a destination pointer, while updating certain hardware or custom registers.
- `populate_table_from_registers`: writes a sequence of values to a memory range and then copies and shifts data from a source pointer to a destination pointer.
- `execute_function_chain`: Resets a global variable and then executes a loop that calls a routine via a pointer and a library function, eventually jumping to a routine loaded from a global variable.
- `execute_loop_with_library_call`: Resets a global variable, then executes a loop that calls a routine via a pointer and a library function, decrementing a loop counter.
- `invoke_conditional_pointer`: Loads a function pointer from a global variable and calls it, and if the return value is non-zero, updates a global variable with that return value before returning or calling another pointer.
- `call_indirect_from_global`: Calls a routine using a pointer stored at an offset from a global variable.
- `call_indirect_from_global_offset`: Calls a routine using a pointer stored at a different offset from a global variable.
- `execute_indirect_routine`: Loads a function pointer from a global variable and jumps to it.
- `increment_and_branch_global`: Increments a global variable, resets it if it reaches one, and executes a routine pointer or sets a global to one.
- `copy_conditional_block`: Checks if a global variable equals a specific value and, if so, calls a routine and copies a fixed number of 3deleted-bit words from a source address to a destination address.
- `rotate_interrupt_mask`: Checks specific bits in a global flag and, if set, updates a hardware register with a new bitmask and resets a counter.
- `update_memory_with_bit_mask`: Calculates a value based on a bit-rotated version of a global variable and stores either that value or a zero byte to a memory location calculated from a register and anoffset.
- `store_zero_to_memory`: Stores a zero byte to a memory location calculated from a register and an offset.
- `modify_global_with_logic`: Performs a long sequence of conditional checks and bitwise or-operations on a global variable.
- `modify_global_bit_flags`: Performs a series of conditional checks and bitwise OR operations on a global variable.
- `calculate_value_from_global`: Performs arithmetic operations involving a global variable and its previous value.
- `set_global_to_max`: Sets a global variable to a maximum unsigned integer value.
- `wait_for_global_zero`: Enters a loop that continues as long as a global variable is not zero.
- `update_masked_global`: Checks if a masked register value matches a global variable, then updates that global variable with a masked value and certain bits cleared or set.
- `set_global_bitmask`: Assigns a register value to a global variable and then clears or sets specific bits within that global variable.
- `update_bitmask_flags`: Updates specific bits within a global variable based on the state of certain bits in a register.
- `modify_global_bits`: Updates certain bits in a global variable based on the contents of a register.
- `set_bit_from_register`: Clears or sets a specific bit in a global variable based on a bitmask or a value in a register.
- `clear_bit_in_global`: Clears a specific bit in a global variable based on a bitmask or register value.
- `fill_memory_with_vectors`: Sets a global variable to a maximum value and repeatedly writes a sequence of 32-bit registers into a destination memory area using a loop.
- `fill_memory_with_vector`: Repeatedly writes a sequence of four 32-bit registers into a destination memory area using a loop.
- `wait_for_hardware_status`: Waits for a hardware register bit to clear before executing two functions from a table.
- `copy_memory_block_2`: Copies a fixed number of 32-bit words from a source address to a destination address.
- `wait_for_dma_status`: Waits for a specific bit in a hardware status register to clear before calling two functions from a global table.
- `setup_blt_parameters`: Configures hardware registers for a block transfer, including checking a DMA or status register and setting certain register values.
- `invoke_exec_functions`: Calls two functions stored in a global table, which involve disabling and enabling certain system states.
- `invoke_permit_functions`: Calls two functions located at offsets in a global table, which involve system permission or state management.
- `initialize_memory_with_zeros`: Fills a range of memory with zeros and sets a final word to a specific bit pattern.
- `store_registers_to_memory`: Checks if a global variable is zero and, if so, stores two registers into a memory location calculated from a register and an offset.
- `set_global_and_store_memory`: Sets a global variable to one, writes two registers to a memory location, and then sets the global variable to zero.
- `configure_dma_interrupts`: Sets a bitmask in a global variable and writes certain values to a memory location calculated from a register and an offset, while configuring hardware registers.
- `store_register_to_memory`: Stores a byte from a register into a memory location at an offset from a global variable.
- `configure_dma_and_interrupts`: Sets a bitmask in a global variable and configures hardware registers for DMA and interrupts using a loop-based delay or count.
- `execute_indirect_function`: Loads a function pointer from a global variable and calls it, potentially updating a global pointer if the return value is non-zero.

### logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf

- `empty_routine`: the routine contains no executable instructions and performs no operations.
- `empty_routine_2`: This routine is empty and contains no valid executable instructions before a control flow interruption.
- `update_display_values`: Iterates through a memory range to update display registers by performing bitwise rotations and additions using data from a source pointer.
- `decrement_and_loop`: decrements a register until it reaches zero.
- `store_register_context`: Saves a block of registers and a pointer into a table at a specific memory location and copies a small instruction sequence.
- `copy_and_store_state`: copies a block of memory, null-terminates the end of the block, and stores several registers and a pointer into a global table.
- `initialize_memory_table`: null-terminates a memory location and stores several registers and pointers into a global table.

### logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf

- `save_registers_to_table`: Stores a large set of registers and a stack pointer into a global table or memory area.
- `save_registers_to_stack`: copies a register and several or even a pointer value onto a stack.

### logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX].adf

- `empty_routine_3`: The routine contains no executable instructions and performs no operations.
- `empty_routine_4`: the routine contains no executable instructions.
