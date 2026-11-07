# Game 2290 (Star Wars - Return of the Jedi): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02290). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (914288 top-level bytes in 31 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 1967 raw-word lines |
| Functions named and described | pass, 453 of 453 named (0 author symbols), 453 of 453 described (17 by rule, 52 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1889 of 1938 (97.47%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11632 of 11815 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 342414 |
| code | 37892 |
| data | 86716 |
| padding | 6434 |
| unused | 440832 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Star Wars - Return of the Jedi.adf:run | 2 | 117 | 2 |
| type | 2 | 117 | 2 |
| Star Wars - Return of the Jedi.adf:jedi | 28 | 7178 | 327 |
| Star Wars - Return of the Jedi.adf:add21k | 2 | 26 | 1 |
| Star Wars - Return of the Jedi.adf:wait | 2 | 117 | 2 |
| Star Wars - Return of the Jedi.adf:nofastmem | 9 | 306 | 14 |
| Star Wars - Return of the Jedi.adf:bootblock | 1 | 37 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 868 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 985 | 67 |
| logged disk load from Star Wars - Return of the Jedi.adf | 1 | 1235 | 7 |
| logged disk load from Star Wars - Return of the Jedi.adf | 1 | 42 | 2 |
| logged disk load from Star Wars - Return of the Jedi.adf | 1 | 787 | 2 |

## Cases handled

- reference crashes at an address (HLE illegal instruction an address at an address: through vector 0, as before (vector 4 is 0 again)): 109944 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 49 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 605 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Star Wars - Return of the Jedi/Star Wars - Return of the Jedi.adf: 1120 executed PCs whose RAM was overwritten before any snapshot match its bytes 287776-290480: recovered as a unit run at an address
- Star Wars - Return of the Jedi/Star Wars - Return of the Jedi.adf: 17 executed PCs whose RAM was overwritten before any snapshot match its bytes 466272-466400: recovered as a unit run at an address
- Star Wars - Return of the Jedi/Star Wars - Return of the Jedi.adf: 92 executed PCs whose RAM was overwritten before any snapshot match its bytes 288208-290352: recovered as a unit run at an address
- Star Wars - Return of the Jedi/Star Wars - Return of the Jedi.adf: 3 logged loads cover 0 bytes of the image
- Star Wars - Return of the Jedi/Star Wars - Return of the Jedi.adf: blocks 2-12 (bytes 1024-6656) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.18)
- Star Wars - Return of the Jedi/Star Wars - Return of the Jedi.adf: blocks 19-19 (bytes 9728-10240) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.84)
- Star Wars - Return of the Jedi/Star Wars - Return of the Jedi.adf: blocks 21-21 (bytes 10752-11264) hold ASCII text

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Star Wars - Return of the Jedi.adf:run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: When the destination pointer is nonzero, copies sixteen longwords from a fixed offset in the table base into it.

### type

- `build_sized_buffer_from_list`: Finds the largest value in a linked list of entries, then, when the buffer request returns a nonzero address, writes a header longword and a counting pattern of longwords into it and stores the address.
- `copy_multiple_words`: Copies a sequence of words from a source pointer to a destination pointer.

### Star Wars - Return of the Jedi.adf:jedi

- `add_rotated_values_save_state`: Walks a table of index runs and adds a rotated value into the buffer at each listed offset.
- `add_rotated_values_save_state_2`: Walks a table of index runs and adds a rotated value into the buffer at each listed offset.
- `spin_delay_loop_count`: Counts a register down to zero as a timing delay, then returns.
- `copy_block_then_zero_tail`: Copies longwords from the source address register to the destination until the destination reaches the limit held in the next register, then zeroes longwords up to a second limit, and restores the saved registers.
- `run_state_machine_loop`: Loops forever, sequencing helper calls keyed on the state word.
- `run_sequence_state_loop`: Repeats a state-driven sequence of helper calls, restarting the sequence when the state word is not 8 after a test.
- `execute_complex_loop`: Performs a series of conditional checks and repeated calls to other routines while modifying a global variable.
- `init_counter_and_run_helpers`: Sets several state words to 500, 5 and 3, then loops calling helper routines while decrementing a counter that clamps at zero.
- `init_state_call_helpers`: Sets several state words to fixed values, then makes three helper calls.
- `wait_state_threshold_clear`: Spins until the state word is at least 4, then resets it to zero.
- `poll_status_bit_with_timeout`: Polls bit 7 of the state word, counting down an outer count of 3500 and an inner count of 600.
- `wait_while_status_bit_set`: Spins while bit 7 of the state word is set, then returns.
- `run_countdown_step_sequence`: Runs a step loop over a counter starting at 12, making helper calls and adjusting state.
- `run_eighty_step_helper_loop`: If the state word is nonzero, makes four helper calls, then loops from 80 down to 0 making helper calls on each step and an extra call at step 50, and returns minus one.
- `run_helper_steps_until_limit`: Repeats five helper calls and increments the state word until the comparison against minus 385 fails.
- `run_eighty_step_helper_loop_2`: If the state word is nonzero, makes four helper calls, then loops from 80 down to 0 making helper calls on each step and an extra call at step 50.
- `run_eighty_step_helper_loop_3`: Makes four helper calls, then loops from 80 down to 0 making helper calls on each step and an extra call at step 50, then returns.
- `run_countdown_step_sequence_2`: Makes helper calls on each step of a counted sequence, adjusts the counter held in the state word, and returns early when a helper reports a nonzero result or the counter reaches a stop value.
- `process_script_state_loop`: Executes a loop that processes script commands and updates a global variable based on various conditional checks and comparisons.
- `evaluate_conditional_return`: Performs a comparison on input values to determine whether to return or continue a loop.
- `interpret_script_commands`: Processes script commands by advancing a state word, looking up bytes in a table, and dispatching record handlers.
- `cycle_state_logic`: Advances a shared state through a table and calls a single runtime routine.
- `dispatch_state_logic`: Executes a complex sequence of conditional logic, table scanning, and string printing based on a global variable.
- `process_complex_state`: Performs a series of conditional checks and updates a global variable through a loop.
- `execute_complex_state_logic`: performs a large sequence of conditional checks and calls to various routines while modifying a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `advance_state_and_lookup_byte`: Increments the state word, reads a byte from a fixed table indexed by the state shifted right two bits and masked to 31, and sets bit 128 in the state word when the state exceeds 1.
- `print_state_string_with_default`: If the state byte is zero, replaces it with the character code for zero.
- `print_state_digit_string`: Adds the character code for zero to the state byte, then calls a helper for each character of the string at the state address until a zero byte.
- `print_string_at_fixed_address`: Calls a helper, then calls a per-character helper for each byte of the zero-terminated string at fixed address 108 until a zero byte.
- `print_fixed_string_after_helpers`: Makes two helper calls, then prints the zero-terminated string at fixed address 146 one character at a time.
- `run_helpers_by_mode`: Makes five helper calls, makes two more when the state word is 3 or two different calls when it is 5, makes one more call, and sets the state word to 65535.
- `call_helpers_on_mode_five`: When the input byte is 5, makes two helper calls.
- `print_fixed_string_after_helpers_2`: Makes two helper calls, then prints the zero-terminated string at fixed address 184 one character at a time.
- `wait_then_halt_if_busy`: If the state byte is nonzero, waits until it clears and then loops forever.
- `wait_then_halt_forever`: Spins until the state byte is zero, then loops forever.
- `save_state_around_two_helpers`: Stores the input word in the state word and makes a helper call, then saves the state word, makes a second helper call and restores the saved value.
- `call_helpers_around_self_copy`: Makes two helper calls, copies 8000 longwords from the buffer onto itself, then makes five more helper calls.
- `call_three_helpers_then_reset`: Makes three helper calls and sets the state word to 65535.
- `call_helpers_by_state_nine`: Makes two helper calls when the state byte is 9, otherwise one, then makes ten more helper calls.
- `call_eleven_helpers`: Makes eleven helper calls in sequence.
- `call_seven_helpers`: Makes seven helper calls in sequence.
- `call_long_helper_sequence`: Makes a long fixed sequence of twenty-six helper calls in sequence.
- `call_helpers_around_self_copy_2`: Makes ten helper calls, then copies 8000 longwords from the buffer onto itself.
- `preserve_state_across_helpers`: Makes one helper call, saves the state word, makes two more helper calls, then restores the saved state word.
- `print_zero_terminated_string`: Calls a helper, then calls a per-character helper for each byte of the zero-terminated string addressed by the input register until a zero byte.
- `stub_empty_return`: Returns immediately without doing anything.
- `spin_wait_full_counter_cycle`: Makes a helper call, counts a 16-bit value down through a full wrap as a timing delay, then makes two more helper calls.
- `clear_and_blit_planes`: Zeroes 8000 longwords of the buffer, then copies 27 rows of tile data into four bitplanes spaced 8000 bytes apart, making helper calls between the copies.
- `clear_buffer_call_draw_pair`: Zeroes 8000 longwords of the buffer, makes a long sequence of helper calls, and restores the buffer pointer before returning.
- `iterate_null_terminated_string`: traverses a null-terminated string by calling another routine for each character until the terminator is reached.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `call_helpers_around_self_copy_3`: Makes two helper calls, copies 10752 longwords from the buffer onto itself, then makes eleven more helper calls.
- `fill_screen_with_pattern_rows`: Makes helper calls, then fills 200 rows of 160 bytes with a repeating four-longword pattern taken from low memory, ending at the state address plus 32000.
- `blit_masked_bitplane_sprite`: Combines a sprite's bitplanes into a screen buffer through a mask plane.
- `wait_helper_zero_twice`: Calls a helper repeatedly until it returns zero, then does the same again, and returns.
- `execute_iterative_calls`: Calls another routine repeatedly until it returns a non-zero value, then repeats the process once more.
- `copy_four_plane_longword_columns`: Copies longwords from the source to the destination one column at a time, moving three further bitplanes at fixed offsets alongside each copy.
- `pack_chunky_pixels_to_bitplanes`: Writes four consecutive source bytes into four bitplanes of the destination at once, advancing the column with wraparound at 56 and the row by 56 bytes per block, over the height and width given by the input registers.
- `copy_plane_block_then_wait`: Makes two helper calls, copies 192 rows of ten longwords between planes at fixed offsets, then calls a helper repeatedly until it returns zero.
- `draw_tile_list_by_mode`: Makes two helper calls, selects one of three tile index lists by the state word, and copies each listed tile into four bitplanes of the buffer, skipping zero entries, for 672 cells.
- `draw_tile_list_by_flag`: Selects one of two tile index lists by the input word, and copies each listed tile into four bitplanes of the buffer, skipping zero entries, for 672 cells.
- `draw_tile_list_default`: Makes two helper calls, then copies each tile in a fixed tile index list into four bitplanes of the buffer, skipping zero entries, for 672 cells.
- `zero_fill_longword_block`: Zeroes 192 rows of 56 longwords, moving forward from the address register.
- `call_then_wait_helper_twice`: Makes two helper calls, then calls a helper repeatedly until it returns zero, twice over.
- `swap_and_translate_plane_bytes`: Swaps bytes between two regions of four bitplanes row by row, passing each value through a fixed lookup table offset by four first.
- `copy_block_to_four_planes`: Copies a block of 12 bytes by 108 rows from a fixed source address into four bitplanes of the buffer, with each plane spaced by a fixed offset.
- `build_bit_reversal_table`: Fills 256 bytes starting at the state address with each index's bit-reversed value, bit 0 becoming bit 7.
- `clear_flagged_records_and_dispatch`: Loads a value from a table selected by the state word, clears the flag word of each record in a list whose low bits match a mask, then calls handlers from a per-entry table for four cells.
- `dispatch_cell_handlers_for_four`: For four bytes at an address derived from the state word, calls the handler selected by bits 4 to 6 of each byte, skipping zero bytes and bytes with a nonzero low nibble.
- `interpret_script_command_byte`: Reads a command byte from a script table at the state position and dispatches on its command group (16, 32, 48, 64, 80, 96 and 112).
- `interpret_script_command_byte_2`: Reads a command byte from a script table at the state position and dispatches on its command group (16, 32, 48, 64, 80, 96 and 112).
- `interpret_script_command_byte_3`: Reads a command byte from a script table at the state position and dispatches on its command group (16, 32, 48, 64, 80, 96 and 112).
- `apply_command_code_table_entry`: Handles command codes 16, 32 and 48.
- `dispatch_command_group_handlers`: Dispatches on an input command group (16, 32, 48 or 64).
- `step_counter_command_handler`: Handles command groups 16, 48 and 64.
- `handle_command_group_counter`: Handles command groups 16, 32 and 64.
- `set_state_to_one`: Sets the state word to 1 when it equals itself, which is always true, so the state word ends up at 1.
- `set_state_to_one_2`: Sets the state word to 1 and returns.
- `add_offset_then_call_helpers`: Makes a helper call, adds 64 to the state word, sets it to 3, then makes a second helper call.
- `add_offset_then_call_helpers_2`: Makes a helper call, adds 192 to the state word, sets it to 2, then makes a second helper call.
- `copy_entry_into_free_record`: Makes three helper calls with state settings between them, sets the state word to 46, and copies a 19-word entry from a fixed table into the first free record slot.
- `stub_state_self_assign`: Assigns the state word to itself, which has no effect.
- `set_state_to_one_3`: Sets the state word to 1.
- `set_state_after_optional_call`: When the state word is zero, makes a helper call.
- `set_state_call_helper`: When the state word is nonzero, sets it to 1 and then to 256 and makes a helper call.
- `load_state_from_table`: Loads the state word from a 32-bit table entry addressed by the input address plus the state word times four.
- `copy_state_into_free_slot`: Copies 19 words starting at the state buffer into the first free record slot, and returns without copying if no free slot exists.
- `init_fields_copy_entry`: Sets several state and record fields, makes helper calls, then copies a 19-word entry from a fixed table into the first free record slot.
- `copy_entry_if_state_matches`: Makes helper calls and sets the state word to 1.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to the caller.
- `set_state_from_bit_field`: Sets the state word to bits 1 to 5 of itself minus 100 and returns.
- `set_state_from_low_bits`: Sets the state word to its low five bits plus 53 and returns, because the later checks cannot pass.
- `advance_table_step_when_zero`: When the state word is zero, advances an index through a three-entry table of longwords with wraparound, loads the selected entry into the state word, calls a helper and advances the index again.
- `advance_table_step_in_range`: Runs a table step only when the state word is within a small range: advances an index through a three-entry table with wraparound, loads the entry into the state word, calls a helper and advances the index again.
- `advance_table_step_for_input`: Returns when the input word is above 1.
- `advance_table_step`: Advances an index through a three-entry table with wraparound, loads the entry into the state word, calls a helper and advances the index again.
- `stub_unreachable_entry_copy`: Returns without doing anything in practice: a 19-word copy of a fixed table entry into a free record slot sits behind a condition that cannot be true.
- `copy_entry_when_state_zero`: When the state word is zero, copies a 19-word entry from a fixed table into the first free record slot.
- `copy_entry_into_free_record_2`: Copies a 19-word entry from a fixed table into the first free record slot, and returns without copying if no free slot exists.
- `copy_entry_if_state_matches_2`: Sets the state word through several assignments and helper calls, then copies a 19-word entry from a fixed table into the first free record slot when the state word equals 2166.
- `advance_table_copy_entry`: When the state word is nonzero and within range, advances an index through a three-entry table of longwords with wraparound, loads the entry into the state word, then copies a 19-word entry from a fixed table into the first free record slot.
- `advance_table_copy_entry_2`: When the state word is within a small range, advances an index through a three-entry table with wraparound, loads the entry into the state word, and copies a 19-word entry from a fixed table into the first free record slot.
- `advance_table_copy_entry_3`: When the input word is 1 or less, advances an index through a three-entry table, loads the entry into the state word, and copies a 19-word entry from a fixed table into the first free record slot.
- `advance_table_copy_entry_4`: Advances an index through a three-entry table with wraparound, loads the entry into the state word, and copies a 19-word entry from a fixed table into the first free record slot.
- `find_and_copy_record`: Searches a table for an entry with a specific bit cleared and then copies a sequence of words from a source to that entry.
- `copy_entry_if_state_matches_3`: Makes helper calls and sets several state words, then copies a 19-word entry from a fixed table into the first free record slot when the state word equals 2166.
- `initialize_and_copy_record`: Sets several global variables to specific values and then copies a sequence of words into a record found within a table.
- `stub_unreachable_entry_copy_2`: Makes a helper call and sets the state word to zero, then returns, so the 19-word entry copy after it never runs.
- `copy_entry_when_state_nonzero`: When the state word is nonzero, sets it from the input word, makes a helper call, then copies a 19-word entry from a fixed table into the first free record slot.
- `set_state_then_copy_entry`: Sets the state word from the input word, makes a helper call, then copies a 19-word entry from a fixed table into the first free record slot.
- `toggle_state_and_copy_entry`: When the state word is nonzero, toggles its low bit, sets a position value, makes two helper calls with adjusted values, then copies a 19-word entry from a fixed table into the first free record slot.
- `toggle_state_and_copy_entry_2`: Toggles the low bit of the state word, sets a position value, makes two helper calls with adjusted values, then copies a 19-word entry from a fixed table into the first free record slot.
- `set_position_and_copy_entry`: Sets a position value to minus 32, makes two helper calls with adjusted values, then copies a 19-word entry from a fixed table into the first free record slot.
- `set_parameter_pair_and_call`: When the state word is nonzero, makes helper calls and sets a pair of parameter words, then calls a helper for state values 2 and 3.
- `call_helper_if_input_three`: Makes a helper call when the input word equals 3, otherwise returns.
- `advance_state_and_call_helpers`: Increments the state word and makes helper calls chosen by whether the previous state was zero and by the new state, with a range check that resets the state to zero.
- `select_step_sequence_by_state`: Branches on a shared state variable (values 1 and 2 plus a default) and on a flag argument, storing fixed constants into that variable and calling the same routine recursively, and resets the variable to zero when it passes a per-branch limit.
- `run_fixed_step_then_reset`: Stores a fixed constant in the shared state variable and calls an unnamed routine, then resets it to zero and calls again if it exceeds 2.
- `select_step_by_register_pair`: Chooses between two constant sequences based on a second register argument equal to 2 and a flag argument, storing each constant in the shared variable and calling an unnamed routine, with a reset once a limit is passed.
- `run_negative_step_then_reset`: Stores a negative fixed constant in the shared variable and calls an unnamed routine, then resets it to zero and calls again if it exceeds 3.
- `select_step_by_flag`: Stores a negative constant or a positive constant in the shared variable depending on the flag argument, calls an unnamed routine, and in the positive branch resets the variable to zero and calls again once it exceeds 4.
- `run_step_then_reset`: Stores a fixed constant in the shared variable, calls an unnamed routine, and resets the variable to zero with a second call if it exceeds 4.
- `relay_single_routine_call`: Contains only one call to an unnamed routine followed by a return, passing control on with no state change of its own.
- `relay_single_call`: Passes control to another routine and then returns.
- `relay_single_call_2`: Calls an unnamed routine before returning to its caller.
- `relay_single_call_3`: Acts as a wrapper that executes a single subroutine call.
- `relay_single_call_4`: Transfers execution to another routine and immediately returns.
- `store_constant_then_call_twice`: Stores a fixed constant in the shared state variable and then calls an unnamed routine twice in succession before returning.
- `call_routine_twice`: Calls an unnamed routine two times in a row and returns; the decompiled body contains nothing else.
- `mask_value_or_forward`: ORs the incoming register value with 223, which forces every bit except bit 5 on; if the shared state variable is nonzero it returns the result of an unnamed routine instead.
- `call_routine_three_times`: Calls an unnamed routine three times in succession and returns, with no other logic in the decompiled body.
- `advance_counter_and_copy_planes`: Increments a shared counter while it stays below 10, then copies 16 scanlines of four bitplanes into each of 12 rows, taking rows before the counter from one source block and the remaining rows from the alternate block.
- `copy_plane_rows_by_counter`: Copies 16 scanlines of four bitplanes into each of 12 row destinations without changing the counter, taking rows before the shared counter from one source block and the remaining rows from the alternate block.
- `increment_shared_counter`: Adds one to the shared state variable and returns.
- `add_bcd_then_call_six`: Adds 3 to a shared state variable, applies a decimal (BCD) adjustment to it, then calls an unnamed routine six times in succession.
- `call_routine_six_times`: Calls an unnamed routine six times in succession and returns, with no other logic in the decompiled body.
- `execute_six_calls`: Calls another routine six times in succession.
- `call_routine_five_times`: Calls an unnamed routine five times in succession and returns, with no other logic in the decompiled body.
- `call_routine_four_times`: Calls an unnamed routine four times in succession and returns, with no other logic in the decompiled body.
- `execute_three_calls`: Calls another routine three times in succession.
- `call_routine_twice_2`: Calls an unnamed routine two times in succession and returns, with no other logic in the decompiled body.
- `relay_single_call_5`: Executes one subroutine call and returns.
- `add_bcd_bytes_call_seven`: Adds the three low-order bytes of a register argument into a shared variable, applying decimal (BCD) adjustment after each addition, then calls an unnamed routine seven times.
- `execute_repeated_routine`: Calls another routine exactly six times before returning.
- `execute_copy_routine`: Calls another routine six times and then returns.
- `run_copy_sequence`: Calls another routine five times and then returns.
- `perform_bitplane_copy`: Calls another routine four times and then returns.
- `execute_bitplane_copy`: Calls another routine twice before returning.
- `call_two_routines`: calls another routine twice and then returns.
- `relay_single_call_6`: Performs a single call to another routine and returns.
- `call_pair_in_compare_loop`: Loops calling an unnamed routine twice per pass, but the loop condition compares the shared variable with itself, so the decompiled exit condition is degenerate and the true iteration count cannot be read from this code.
- `make_call_pairs_forever`: Executes pairs of calls to another routine within a loop that compares a global variable to itself, preventing the loop from ever exiting.
- `copy_twelve_rows_four_planes`: Copies 12 scanlines of four bitplanes from a source block whose start is a register offset plus a fixed constant, advancing the source and destination by 40 bytes per row and by 8000 bytes between planes.
- `cycle_state_and_place_cells`: Advances a shared state through a four-entry table cycle and, when the state holds a particular value, aligns it and calls an unnamed routine; then steps through grid positions from a table-selected start, storing x and y and calling an unnamed routine for each cell, with x up to 256 and y up to 200.
- `align_cycle_and_place_cells`: Aligns a register argument down to a multiple of 8 and calls an unnamed routine, then advances a four-entry table cycle and steps through grid positions from a table-selected start, storing x and y and calling an unnamed routine for each cell until x reaches 256 or y reaches 200.
- `cycle_and_place_grid_cells`: Advances a shared state through a four-entry table cycle, then steps through grid positions starting from a register argument, storing x and y and calling an unnamed routine for each cell until x reaches 256 or y reaches 200.
- `step_grid_cells_within_bounds`: Walks a grid of cell positions starting from the register arguments, storing the horizontal position in one fixed memory location and the vertical position in another and calling an unnamed routine for each cell; horizontal stops at 256 and vertical at 200, advancing 16 and 8 respectively.
- `stub_immediate_return`: Contains only a return instruction, so calling it does nothing; the decompiled body is empty.
- `dispatch_records_for_all_rows`: Scans a table of 19-word records from its start until a negative first word, and for each record with bit 0 set, bit 1 clear and a third word equal to the current row, calls one of two unnamed routines chosen by bit 10; it repeats this for every row from 16 down to 0.
- `scan_table_until_counter_expires`: Repeatedly scans the same 19-word record table, calling an unnamed routine at each pass and decrementing a stack-held counter at each end-of-table marker; it returns when the counter underflows, and matching records are dispatched to one of two unnamed routines by bit 10.
- `dispatch_records_from_row`: Walks the 19-word record table once for each row index from the register argument down to 0, calling one of two unnamed routines for each record that matches that row with bit 0 set and bit 1 clear, selected by bit 10.
- `step_field_nine_then_restore`: Saves a word at offset 8 of the structure addressed by the address register, then runs nine passes that each call an unnamed routine and add 16 to that word, and finally writes the saved value back.
- `position_clip_and_blit_sprite`: Computes a sprite's screen position from its record fields and a shared offset, or reuses the stored values when bit 6 of the flag word is set, then clips it to the screen and blits four bitplanes with optional masking and a sub-byte shift by calling unnamed routines.
- `position_and_blit_sprite_record`: Computes a sprite's screen position from the fields of the record it is given, clips it to the screen, and blits four bitplanes with optional masking and a sub-byte shift, storing the computed position back into the record.
- `blit_sprite_from_stored_position`: Reads the sprite's previously stored position and clip fields from its record, clips it to the screen, and blits four bitplanes with optional masking and a sub-byte shift without recomputing the position.
- `blit_bitplane_data`: Performs bitwise operations to merge sprite bitplane data into a destination buffer using bit shifts and masks.
- `blit_clipped_sprite_from_registers`: Clips a sprite placed by argument registers against the screen bounds, adjusting its origin for negative coordinates, then blits four bitplanes with optional masking and a sub-byte shift.
- `blit_sprite_with_top_clip`: Returns early when the row offset covers the whole sprite height; otherwise it advances the source by the row offset and blits four bitplanes with optional masking and a sub-byte shift from the argument registers.
- `blit_sprite_with_x_clip`: Rejects a sprite whose horizontal position is past the screen edge unless a flag bit is set, then blits four bitplanes using the argument registers for origin, width and row offset, with optional masking and a sub-byte shift.
- `blit_sprite_with_row_offset`: Returns early when the row offset covers the full sprite height; otherwise it moves the source pointer by the row offset and blits four bitplanes with optional masking and a sub-byte shift.
- `blit_sprite_select_shift_kernel`: Selects and runs a blit kernel for a sprite according to its sub-byte shift and mask flags, calling unnamed routines per row group after computing the screen destination from the argument registers.
- `blit_shifted_plane_words`: Shifts source plane words by a sub-byte amount taken from an argument register, combining neighboring words across four bitplanes, and writes them to the destination with optional masking, row by row.
- `copy_planes_through_mask`: Copies four bitplanes of source words into the destination row by row; where the fourth source plane is fully set it stores the three plane words and writes all ones into the fourth destination plane, otherwise it merges the words into the destination.
- `apply_plane_color_masks`: Merges per-plane constant patterns into four bitplane rows of the destination using AND masks and OR values, advancing the destination by the row stride after each row.
- `blit_bytes_into_aligned_planes`: Writes source bytes into four byte-aligned bitplanes of the destination, combining plane bytes from neighboring offsets with AND and OR masks, looping over rows and advancing by the row stride.
- `blit_shifted_words_with_mask`: Shifts source plane words by a sub-byte amount and merges them into the destination's four bitplanes under masks, running row by row and advancing by the row stride.
- `blit_shift_wrapped_plane_words`: Shifts source plane words by a sub-byte amount using the complementary shift count, carrying bits between neighboring words, and merges them into four bitplanes row by row.
- `copy_four_planes_from_offset`: Copies four bitplane words per row from a source block that starts at a fixed offset, advancing the destination by the row stride, and calls an unnamed routine.
- `copy_planes_with_small_offset`: Copies four bitplane words per row from a source block starting a short distance past its base, advancing by the row stride, and calls an unnamed routine.
- `apply_plane_masks_when_partial`: Skips rows whose mask word is all ones; otherwise it ANDs each of the four bitplanes with the mask and ORs in a per-plane pattern word, advancing by the row stride.
- `blit_bytes_with_plane_flags`: Tests flag bits 4 and 10 to choose a byte-level variant, then for each row ANDs or ORs source bytes into four byte-aligned bitplanes and advances by the row stride, calling unnamed routines in some cases.
- `blit_shifted_words_with_fill`: Shifts source plane words by a sub-byte amount from an argument register, filling vacated bits with ones, and writes them into four bitplanes row by row under masks.
- `blit_shifted_words_wrapped`: Shifts source plane words by the complement of a sub-byte amount, filling vacated bits with ones and carrying bits from the next word, then merges them into four bitplanes row by row.
- `call_library_function_minus_36`: Calls two unnamed routines and then a library function at a fixed offset of 36 bytes before the library base held in a global variable, then returns.
- `retry_library_calls`: Repeatedly calls a library function with an argument and a fixed code while the result is zero, using other library functions for delay and retrieval.
- `call_six_then_library_36`: Calls an unnamed routine six times, then calls a library function at a fixed offset of 36 bytes before the library base held in a global variable.
- `unpack_run_length_stream`: Calls an unnamed routine three times, then decodes a word stream into a shared destination: words with high byte 178 are control words, a set bit 7 ends the block, and a count of 8 stops the stream, while other control words repeat the following word count plus one times.
- `call_five_then_library_36`: Calls an unnamed routine five times, then calls a library function at a fixed offset of 36 bytes before the library base held in a global variable.
- `call_three_then_library_36`: Calls an unnamed routine three times, then calls a library function at a fixed offset of 36 bytes before the library base held in a global variable.
- `retry_library_calls_with_stub`: Repeatedly calls a library function with an argument and a fixed code while the result is zero, eventually executing an empty loop stub.
- `set_state_one_and_unpack`: Sets a shared state variable to 1, calls unnamed routines, then decodes a run-length word stream into the shared destination using the same control-word scheme as the unpacker.
- `set_state_two_and_unpack`: Sets a shared state variable to 2, calls unnamed routines, then decodes a run-length word stream into the shared destination using the same control-word scheme as the unpacker.
- `set_state_three_and_unpack`: Sets a shared state variable to 3, calls unnamed routines, then decodes a run-length word stream into the shared destination using the same control-word scheme as the unpacker.
- `unpack_stream_with_library_calls`: Decodes a run-length word stream into the shared destination with the same control-word scheme, and makes calls through the library base held in a global variable at fixed offsets.
- `fill_then_decode_run_stream`: Fills the destination with a source word repeated a count held in an argument register, then decodes a run-length word stream with the same control-word scheme, stopping when a count of 8 ends it.
- `decode_run_stream_with_seed`: Writes an argument word to the destination, then decodes a run-length word stream from the source with the same control-word scheme, returning when a count of 8 ends it.
- `return_from_subroutine_3`: This routine is empty and performs no operations.
- `init_four_audio_channels`: Programs all four audio channels with a fixed start location, a length of 16 words, a period of 400 and zero volume, then sets a shared state variable and calls an unnamed routine.
- `set_state_one_then_loop`: Sets a shared state variable to 1 and calls an unnamed routine, then enters a loop whose exit condition is not visible in the decompiled output.
- `disable_audio_interrupts`: Sets a shared state variable through the values 5 and 1, writes a mask of four audio interrupt bits to the interrupt enable register with the set bit clear to disable them, and then sets the state to 15.
- `disable_audio_interrupts_if_idle`: When the shared state variable is zero, sets it through 8 and 1, clears the four audio interrupt enable bits through the interrupt enable register, and sets the state to 15; otherwise it returns.
- `return_void_2`: The routine is empty and performs no operations.
- `disable_audio_interrupts_2`: Sets a shared state variable through the values 3 and 1, clears the four audio interrupt enable bits through the interrupt enable register, and sets the state to 15.
- `disable_audio_interrupts_3`: Sets a shared state variable through the values 2 and 1, clears the four audio interrupt enable bits through the interrupt enable register, and sets the state to 15.
- `disable_audio_interrupts_4`: Sets a shared state variable to 1, clears the four audio interrupt enable bits through the interrupt enable register, and sets the state to 15.
- `store_argument_then_call_five`: Stores a register argument in a shared state variable, then calls an unnamed routine five times.
- `allocate_memory_and_open_libraries`: Clears a block of memory, finds the current task and allocates memory through the exec library, marks a field of the new allocation with all ones, opens a library and stores its base in a shared variable, then offsets a shared pointer into the allocation by fixed amounts.
- `install_interrupt_server`: Calls the exec library's AddIntServer through the library base to install an interrupt handler, then enters a loop; the decompiled body shows only that call and the loop start.
- `setup_display_and_copper_list`: Counts down a long delay, then writes display window, data-fetch, bitplane control and modulo registers, points the copper at its list through the copper location register, restarts it through the copper jump register, and updates DMA and interrupt enable bits.
- `build_bitplane_pointer_table`: Writes four bitplane addresses, each 8000 bytes past the previous, as low and high word pairs into a table, then copies sixteen source words shifted left by one bit into a shared table.
- `call_four_then_return`: Calls four unnamed routines in succession, some of which reach library functions through a base pointer, and returns.
- `retry_library_call_with_delay`: Repeatedly calls a library function with an argument and a fixed code while the result is zero; each failed attempt calls another library function, advances a shared value by 256, calls an unnamed routine and busy-waits for a long count.
- `store_argument_in_state`: Stores its register argument in a shared state variable and returns.
- `retry_library_call_or_fallback`: Calls a library function through the library base; a nonzero result is stored in a shared variable before returning, while a zero result calls through a stored code pointer and enters a loop.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_stored_pointer_and_loop`: Calls the code address held in a shared variable and then enters a loop; the decompiled body shows no further logic.
- `advance_state_and_dispatch`: Advances a shared counter, wrapping it to zero when it reaches 1, calls through that variable's value as a code address, then calls three unnamed routines and sets the variable to 1.
- `clear_four_state_fields`: Sets four shared state fields to zero and returns.
- `decode_stick_direction_bits`: Reads shared input state and builds direction bits for the stick position, returning early when a previous flag is set, and calls an unnamed routine.
- `decode_button_state_bits`: Combines shared input flags into a small button-state value through conditional branches, then calls two unnamed routines.
- `combine_input_flag_bits`: Builds a bit pattern from shared input flags by setting bit values 8, 4, 2 and 1 according to conditional branches, then calls an unnamed routine.
- `select_input_bit_pattern`: Chooses the input bit pattern from shared flags by OR-ing bit values 8, 2 and 1 under conditional branches, then calls an unnamed routine.
- `add_flags_with_high_bit`: Adds bit value 2 and a high-order flag bit to the input pattern when the shared flags match, clears the state when the pattern is empty, and calls two unnamed routines.
- `map_first_axis_to_bits`: Maps a signed shared axis value to direction bits: below minus one it sets bit values 4 and 1, above one it sets bit value 8, and it then calls an unnamed routine.
- `map_second_axis_to_bits`: Maps a second shared axis value to direction bits, setting bit value 8 when a register argument exceeds one and bit values 1 or 2 from the shared value, and sets a high-order flag when the shared value is nonzero.
- `store_direction_bits_offset`: Stores the combined direction bits in a shared variable and, when the stored value is negative, adds 10 to it.
- `clamp_stick_offset_ten`: Subtracts 10 from a register argument with clamping at zero, then adds 10 back to negative results and clamps again, storing the outcome in a shared variable.
- `subtract_ten_clamped_at_zero`: Subtracts 10 from a register argument, stores the result in a shared variable and clamps negative results to zero.
- `decode_joystick_port_direction`: Reads the joystick port movement register and decodes its direction bits into a shared state value, combining them with a fire flag, then calls an unnamed routine.
- `decode_movement_register_bits`: Combines bits of a movement register argument into a direction value, setting bit values 2 and 1 from the register pattern, and tests a shared flag before storing the result.
- `store_fire_button_bits`: Sets a fire-button bit from the parallel input register and stores the combined bits in a shared variable.
- `read_fire_button_delta`: Reads the fire-button bit from the parallel input register, computes a signed difference from the previous sample, and calls an unnamed routine.
- `read_pot_button_delta`: Reads the right and middle button bits from the pot register, computes a signed difference from the previous sample, and calls an unnamed routine.
- `read_mouse_axis_delta`: Reads the mouse-port movement register into a signed difference from the previous sample, clamps it against a shared range, and calls an unnamed routine.
- `clamp_and_store_position`: Raises a register value to at least a shared minimum and stores it in a shared variable, then clamps a difference against a second register value.
- `raise_to_shared_minimum`: Raises a register value to a shared minimum, stores it in a shared variable and returns.
- `set_state_minus_one`: Stores minus one in a shared variable and returns.
- `stub_empty_loop_return`: Contains only a loop header followed by a return, so the decompiled body shows no visible work; it appears to be a stub or a wait whose condition is not decoded.
- `store_nibble_and_flag`: Stores the low four bits of a register argument in a shared variable unless it already matches, then sets or clears bit value 64 according to bit 0 and calls an unnamed routine.
- `store_value_with_flag_bit`: Stores a register argument in a shared variable, then sets or clears bit value 64 according to its bit 0.
- `set_or_clear_bitmap_bit`: Clears the bit selected by the low three bits of a register argument in a shared byte, or sets it when bit value 2 of a second argument is set.
- `set_or_clear_bitmap_bit_2`: Clears the bit selected by the low three bits of a register argument in a shared byte, or sets it when bit value 4 of a second argument is set.
- `set_or_clear_bitmap_bit_3`: Clears the bit selected by the low three bits of a register argument in a shared byte, or sets it when bit value 8 of a second argument is set.
- `clear_bitmap_bit`: Clears the bit selected by the low three bits of a register argument in a shared byte and returns.
- `clear_pixel_bit_in_planes`: Computes a byte address from row and column arguments, then clears the selected pixel bit in each bitplane and calls an unnamed routine.
- `clear_bit_across_four_planes`: Clears the bit selected by the low three bits of a register argument in the byte at the address register in each of four bitplanes.
- `fill_rows_from_low_memory`: Calls four unnamed routines, then reads four longwords from low system memory and fills a block of memory with that pattern across 200 passes.
- `fill_block_from_low_memory`: Reads four longwords from low system memory and fills a block of memory with that pattern across 200 passes, without calling any routine first.
- `call_single_routine`: Calls one runtime routine and returns.
- `copy_eight_bytes_to_buffer`: Copies eight entries from the source pointer in the address register into a shared buffer, advancing both pointers on each step.
- `shift_eight_entries_in_buffer`: Copies eight entries from one position in a shared buffer to a second position, advancing both pointers in a single loop.
- `call_single_routine_extended`: Calls one runtime routine and returns, with no other visible work performed.
- `wait_for_blitter_then_call`: Calls unnamed routines and waits in a loop while the blitter busy bit is set in the DMA control read register, then calls another unnamed routine.
- `start_blitter_copy_setup`: Waits until the blitter is idle, then loads source and destination pointers, row modulo values, first and last word masks, control bits chosen from a shift argument, and the blit size to start a copy.
- `call_library_then_forbid`: Calls a library function through the library base, then calls the exec library's Forbid, and returns a register pair.
- `call_library_then_permit_2`: Calls a library function through the library base, then calls the exec library's Permit, and returns a register pair.
- `clear_two_plane_blocks`: Calls an unnamed routine, then zeroes two blocks of entries across four bitplanes, with ten passes per block.
- `clear_rows_across_four_planes`: Clears 24 rows of words across four bitplanes at a fixed offset from the shared base pointer.
- `set_state_fields_to_four`: Sets shared state variables to 1, 0 and 4 in turn and returns.
- `set_state_fields_to_eight`: Sets shared state variables to 0 and 8 in turn and returns.
- `set_fields_to_minus_eight`: Sets shared state variables to 0 and minus 8 in turn and returns.
- `compute_row_offset_mod_192`: Stores a register argument in a shared variable, takes its remainder modulo 192 and the distance to the next multiple of 192, and stores a 16-aligned offset derived from it, then calls an unnamed routine.
- `double_and_flag_state`: Doubles a shared value when it is nonzero and, when its sign bit is set, stores 2, 1 and 0 in sequence, calling two unnamed routines along the way.
- `clear_state_field`: Sets a shared variable to zero and returns.
- `compute_row_offset_wrap`: Stores a register argument, reduces it modulo 192 with a branch for the remainder, derives a 16-aligned offset from the result, and calls an unnamed routine.
- `compute_table_offset_entry`: Stores a fixed value of 256 and a table entry selected by a register index in shared variables, then stores a position computed from the arguments scaled by 56.
- `call_if_flags_set`: Calls an unnamed routine when a shared variable is nonzero, repeats the call under a second check of the same variable, then returns.
- `wait_blitter_then_finish_b`: Calls five runtime routines, waits until the blitter busy flag clears, then calls one more routine.
- `start_blitter_copy_wide`: Waits for the blitter to become idle, then sets pointers, masks, control bits and blit size for a copy that uses different source and destination row modulo values.
- `compute_screen_byte_offset`: Stores two register arguments in shared variables and computes a byte offset from the first argument plus the second multiplied by 320, storing the result in a shared variable.
- `copy_glyph_rows_to_screen`: Copies eight bytes from a table entry selected by a register argument into successive rows of the destination, advancing by the 40-byte row stride.
- `advance_index_scaled_offset`: Calls unnamed routines, then increments a shared index and recomputes it as a scaled byte offset by summing shifted copies of itself, giving a multiplier of 322.
- `update_scaled_index`: Increments a global variable and recomputes it as a scaled offset using bit shifts and additions.
- `map_char_to_glyph_index`: Maps a character code in a register argument to a glyph index by subtracting 32, 64 or 96 according to the 32-code range it falls in, starting from codes above 31, and calls an unnamed routine.
- `map_char_to_glyph_index_2`: Maps a character code in a register argument to a glyph index by subtracting 32, 64 or 96 according to the 32-code range it falls in, and calls two unnamed routines.
- `subtract_64_from_code`: Returns the register argument minus 64.
- `subtract_32_from_code`: Returns the register argument minus 32.
- `copy_eight_rows_four_planes`: Copies eight rows of four bitplane words from the source to the destination, advancing both by the 40-byte row stride.
- `merge_masked_rows_four_planes`: Merges eight rows of four bitplanes from the source into the destination under a mask taken from the fourth source plane, advancing by the row stride.
- `set_palette_entry`: Writes its second argument into the hardware colour register selected by the index in its first argument, starting from COLOR00.
- `clear_low_memory_words`: Zeroes a block of words starting at low memory address zero.
- `find_first_inactive_record`: Walks a table of 19-word records, skipping those with bit 0 set, until it finds one with that bit clear, and returns if the table ends with a negative terminator; otherwise it calls an unnamed routine on the record it found.
- `copy_record_into_free_slot`: Advances a destination pointer through a table of 19-word records until it reaches a negative terminator, then copies the 19-word source record into the slot it found.
- `tiny_movea_rts`: Small routine of 2 instructions that copies 0 to a0, then returns.
- `call_flagged_record_handlers`: Walks the 19-word record table until a negative terminator, calling the code pointer stored in each record that has bit value 32 set, then returns.
- `execute_empty_routine`: An empty routine that performs no operations before returning.
- `tiny_move_rts`: Small routine of 2 instructions that copies 0 to (a0), then returns.
- `initialize_record_from_offsets`: Stores sums of a source record's fields and register offsets into a destination record, clamping one value into the range 0 to 16, and calls an unnamed routine.
- `store_clamped_record_field`: Clamps a register argument to a maximum of 16, stores it in a record field, then walks the 19-word record table until a negative terminator and calls an unnamed routine.
- `store_sprite_fields_with_state`: Sets a shared state value to 1, calls an unnamed routine, stores two register arguments into record fields, then clears the state.
- `store_pair_fields_clear_state`: Calls an unnamed routine, sets a shared state flag to 1, stores two register arguments into record fields, then clears the flag.
- `store_pair_into_record_fields`: Calls an unnamed routine and stores two register arguments into record fields.
- `start_audio_channel`: Selects one of four audio channels from the argument, sets its bit in the ADKCON register, enables its interrupt through INTENA, records its state in a shared table, and sets its period register to 200.
- `call_unless_flag_set`: Returns immediately when a shared variable is nonzero; otherwise it calls an unnamed routine and returns its register pair.
- `return_void`: This is an empty routine that performs no operations before returning.
- `return_pair_if_nonzero_2`: checks a global variable and, if nonzero, returns a pair of values from registers, otherwise calls another routine and returns the resulting pair.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `call_and_forward_result`: Calls an unnamed routine and keeps its returned value, then calls another unnamed routine with it, with a branch on a shared variable.
- `return_pair_if_nonzero`: checks a global variable and returns a pair of values from registers if the variable is nonzero, otherwise calls another routine and returns the resulting pair.
- `call_then_return_pair`: Calls an unnamed routine and returns the register pair it leaves behind.
- `scan_range_table_for_match`: Scans a table of three-byte range entries from a fixed start until a sentinel address, comparing a shared value against each entry's bounds and calling unnamed routines for matches.
- `init_entry_and_copy_block`: Unless the entry pointer is the sentinel, sets three bytes of the entry to a shared value, then copies a block of 24 words from the argument record and calls unnamed routines.
- `walk_range_entries_until_sentinel`: Walks range entries from the argument pointer until the sentinel address, calling unnamed routines for each entry.
- `scan_ranges_clear_on_sentinel`: Compares a shared value against range entries and, at the sentinel address, sets the shared value to zero and returns.
- `clear_global_variable`: stores zero into a global variable.
- `init_entry_and_copy_block_2`: Sets three bytes of the entry to a shared value, then copies a block of 24 words from the argument record and calls an unnamed routine.
- `set_dash_and_call_five`: Stores the character with code 45 at a fixed field of the argument record, points a shared pointer past it, and calls unnamed routines five times.
- `write_digit_character`: Writes the ASCII digit for a register value to the buffer pointer, using the zero digit when the value is zero and a width argument is nonzero, and writes nothing when both are zero.
- `write_zero_character_if_set`: Writes the ASCII zero character to the buffer pointer when the width argument is nonzero, and returns.
- `copy_header_then_block`: Copies three header words from the source record into the destination fields, then copies a block of 26 words from the argument record.
- `fill_name_buffer_with_marks`: Writes an underscore fill mark into a shared text buffer, fills the following 13 positions with dot marks, and calls two unnamed routines.
- `fill_text_buffer_with_dots`: Writes an underscore at position 3 of a shared text buffer, fills 13 following positions with dot marks, and calls four unnamed routines.
- `store_char_and_pad_row`: Stores a character argument at a row offset, advances a shared index and fills the rest of the 15-position row with underscore marks, calling two unnamed routines.
- `replace_fill_marks_with_spaces`: Scans 14 characters of a text buffer, replacing underscore marks with spaces and handling dot marks separately, and calls two unnamed routines.
- `call_library_then_store_result`: Calls a library function through the library base; when it returns a nonzero value it stores that value in a shared variable, calls an unnamed routine, then calls a library function at a fixed offset of 36 bytes before the base.
- `return_nothing`: The routine is empty and performs no operations before returning.
- `call_library_functions`: Calls two different functions from a library using a base address stored in a global variable.
- `poll_candidates_with_limit`: Runs up to 20000 passes over candidate addresses starting from a fixed address, testing each with an unnamed routine and returning the first result that passes.
- `scan_string_for_terminator`: Scans bytes from the address register with a loop that stops on the zero flag, storing the current position in a shared pointer and clearing it when a count argument is zero, and calls two unnamed routines.
- `scan_table_for_status_byte`: Walks a fixed table of entries, advancing a stored cursor past empty slots until it finds a nonzero entry, then returns that entry's status byte clamped to non-negative values.
- `scan_table_wrapping_for_entry`: Works like the other table scan but resets the stored cursor to the start of the table when it reaches the end, so the search wraps around.
- `find_text_in_record_table`: Compares a zero-terminated search text against the text fields of ten 26-byte records and returns one when the text occurs inside a record, restarting the comparison on each mismatch.
- `scan_records_for_text_match`: Walks a counted list of 26-byte records and checks each one for the search text, restarting the match whenever a character differs.
- `search_text_in_later_records`: Steps to the next 26-byte record before each comparison, then searches for the zero-terminated text inside it.

### Star Wars - Return of the Jedi.adf:add21k

- `refresh_display_and_close_libraries`: Opens the graphics and intuition libraries, sets a byte in a display structure, calls the library functions that free raster memory and remake the display, then closes both libraries.

### Star Wars - Return of the Jedi.adf:wait

- `allocate_and_install_handler_buffer`: Finds the largest value stored in a linked list, allocates a buffer sized from it, and fills the buffer with an increasing sequence of tag values.
- `copy_sixteen_longs_to_buffer`: Copies sixteen 32-bit values from a fixed table in memory into the buffer passed on the stack when that buffer pointer is non-null, then calls a library function through a stored pointer.

### Star Wars - Return of the Jedi.adf:nofastmem

- `parse_command_line_arguments`: Allocates a record, copies the command line from the process structure, and splits it on spaces and tabs into zero-terminated arguments in a linked list.
- `steal_memory_or_report_error`: Repeatedly tries to take over a memory allocation vector and checks that the replacement held, and when it cannot it displays an error console window whose message is chosen by the parameter.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `invoke_stored_function_one`: Calls through the function pointer held in a shared variable with no arguments, then returns.
- `invoke_stored_function_two`: Calls through the function pointer held in a second shared variable with no arguments, then returns.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.

### Star Wars - Return of the Jedi.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_state_machine_loop_2`: Runs an endless state loop that resets a shared state byte and calls the helper routines for each step, breaking out to run its finishing helpers when the state reaches its final value.
- `initialize_runtime_state_flags`: Calls setup helpers in the runtime segment, clears a block of shared state variables, sets a countdown to 499, then returns the result of a final helper call with the constant 132 ORed in.
- `wait_until_counter_reaches_four`: Spins until a shared counter reaches four, then resets that counter to zero.
- `countdown_polling_flag_bit`: Counts a shared value down in nested loops, leaving early when its flag bit is set, and finally waits until that flag bit clears.
- `run_counter_driven_step_loop`: Initialises a state value, then repeatedly runs a step of helper routines that update the shared counter and call the timing and table helpers.
- `run_countdown_sequence_steps`: Runs a setup helper, then when the counter is non-zero runs eighty steps of helper calls, including a special call near the end, and returns minus one.
- `run_state_step_loop_from_two`: Like the other step loop, but starts from a state value of two and calls a different setup helper first, then repeats the step helpers until the state goes negative.
- `step_state_until_exit`: Repeats a step body that calls helpers and adjusts a shared state value, returning once the state goes negative or a helper signals completion.
- `advance_table_index_with_flag`: Increments a shared index, looks up a byte in a fixed table using bits of that index, and sets the high bit when the looked-up value is greater than one.
- `print_status_text_by_flag`: Chooses one of two zero-terminated text strings from a shared flag, adding the character for zero to a shared value when that value is zero, and sends the characters one at a time to an output helper.
- `print_fixed_string_message`: Calls a setup helper, then writes a fixed zero-terminated text one character at a time through the output helper.
- `print_message_after_setup`: Runs a preparation call and a setup helper, then writes a second fixed text one character at a time through the output helper.
- `finish_mode_and_reset_state`: Calls a shared helper, then for modes three and five calls a mode-specific helper, and finally marks the shared value as minus one.
- `print_string_after_init_call`: Calls an initialisation helper, then writes a fixed zero-terminated message one character at a time through the output helper.
- `wait_for_state_then_halt`: busy-waits until a state global becomes zero, then loops forever.
- `copy_buffer_in_long_pairs`: Calls a setup helper, then copies a block of four thousand pairs of 32-bit words from one buffer to another, and finishes with two helper calls.
- `run_mode_dependent_setup_steps`: Runs one or two extra helper calls depending on whether a shared mode value equals nine, then runs a fixed series of helper calls and returns.
- `call_six_helpers_in_order`: Calls one shared helper and then five further helper routines in a fixed order, with no data handling of its own.
- `call_long_helper_sequence_2`: Calls a long fixed sequence of helper routines in order, including a shared helper call near the start, and returns.
- `run_helpers_then_copy_pairs`: Runs ten helper calls, then copies four thousand pairs of 32-bit words from one buffer to another and returns.
- `busy_wait_then_call_helpers`: Counts a value down through its full wrap-around range as a delay, then calls two helper routines and returns.
- `clear_buffer_and_copy_blocks`: Clears eight thousand longs of a buffer and runs a series of helper calls.
- `clear_buffer_and_call_helpers`: Clears eight thousand longs of a buffer, then calls a series of helper routines in pairs and restores the saved buffer pointer before returning.
- `print_string_argument`: Calls a setup helper, then sends a zero-terminated text passed in through the address register one character at a time to an output helper, and returns zero.
- `build_bit_reversed_byte_table`: Fills a 256-byte table where each entry is the bit-reversed form of its index.
- `lookup_indexed_value_with_status`: Loads a value from a table indexed by a shared counter and clears that counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_helper_pair_then_vector`: Calls two helper routines, then calls a function through a pointer held in a shared variable at a fixed offset, and returns.
- `call_helper_pair_then_vector_gap`: Matches the routine before it: it calls two helper routines and then calls the function at a fixed offset through a shared pointer.
- `call_six_helpers_then_vector`: Calls six helper routines in sequence, then calls a function through a pointer held in a shared variable at a fixed offset.
- `expand_run_length_words`: Copies 16-bit words from a source buffer to a destination.
- `call_five_helpers_then_vector`: Calls five helper routines in sequence, then calls a function through a pointer held in a shared variable at a fixed offset.
- `call_three_helpers_then_vector`: Calls three helper routines in sequence, then calls a function through a pointer held in a shared variable at a fixed offset.
- `decode_runs_into_fixed_buffer`: Sets a shared mode value to one, then expands run-length encoded words into a fixed destination buffer and stops at a run count of eight.
- `decode_runs_mode_two`: Sets a shared mode value to two, then expands run-length encoded words into the buffer held in a shared variable and stops at a run count of eight.
- `expand_runs_into_given_buffer`: Expands run-length encoded words from the source held in a shared variable into the destination pointer supplied by the caller, stopping at a run count of eight, after calling three helper routines.
- `setup_four_audio_channels`: Points each of the four audio channels at the same sample address with a length of sixteen words, zero volume and period 400.
- `update_audio_volumes_from_table`: When a shared flag is set, runs four helper calls and then copies three table bytes, each doubled, into the volume registers of the first three audio channels.
- `dispatch_pending_state_handler`: Moves a pending state byte into the active slot when one is set, clears the pending slot, then calls the handler selected by that state from a dispatch table.
- `reset_channel_state_fields`: Clears a shared flag and six status bytes in a channel record, then sets a 16-bit counter field to fifteen.
- `select_index_from_threshold`: Sets a shared index to thirty-two or sixty-three depending on whether the input value is below one hundred twenty-eight, calls helper routines, and then advances the index by one.
- `advance_index_with_table_check`: Calls helper routines, runs four more when the input value is even, and advances a shared index by one, or by two when the next table entry matches it.
- `test_mask_table_bit_and_call`: Returns zero unless a bit mask in the input matches a table entry selected by the low four bits of a shared counter.
- `clear_channel_status_bytes`: Clears six status bytes in a channel record and sets a 16-bit counter field to fifteen, the same reset as its neighbouring routine.
- `increment_counter_wrap_at_input`: Increments a shared byte and resets it to zero when it reaches the value passed in the data register.
- `store_table_word_in_record`: Calls two helper routines, then stores a 16-bit value from a fixed table, indexed by the input byte, into a field of the record in the address register.
- `store_table_word_at_offset`: Stores a 16-bit value from a fixed table, indexed by the input byte, into the record at an offset given by the data register.
- `advance_or_reset_sequence_counter`: Increments a shared sequence counter.
- `start_record_from_table_byte`: If a byte from a table at the given position is nonzero, computes a table index from that byte and a base value, stores the resulting word and a zero status in a record, then runs an update step and counts down a companion value.
- `start_record_from_table_byte_b`: Uses the same table-driven start logic as its neighbour, but writes the results into a record with a different field layout.
- `start_record_from_table_byte_c`: Uses the same table-driven start logic as its neighbours, but writes the results into a record with a third field layout.
- `step_record_counter_with_table`: Increments a byte in a record.
- `store_value_then_run_helpers`: Stores the value passed in the address register into a shared variable, then calls five helper routines in sequence.
- `initialize_memory_and_library_base`: Clears a block of shared variables, uses exec library calls to open a library and allocate memory, then stores three offset pointers for the allocated area.
- `install_interrupt_server_then_halt`: Calls an exec library function through a shared library base to add an interrupt server, then loops forever without returning.
- `configure_display_and_copper`: Points a shared pointer at a copper list, waits through a long countdown, then writes the display window, data fetch, bitplane modulo and copper pointer registers, and enables copper interrupts.
- `fill_table_entries_and_copy_words`: Stores the input byte plus a comma character in a shared variable, writes four 32-bit values spaced 8000 apart into a table, then copies sixteen words from the source, doubled, into a second fixed array.
- `call_four_helper_routines`: Calls four helper routines in sequence and returns.
- `retry_library_call_until_success`: Repeatedly calls a library function with a fixed code value until it returns a nonzero result.
- `store_library_result_or_halt`: Calls a library function through a shared library base.
- `call_shared_library_function_a`: Calls a library function through a shared library base at a fixed offset and returns without using its result.
- `call_shared_library_function_b`: Calls a library function through a shared library base at a different fixed offset and returns without using its result.
- `call_shared_library_function_c`: Calls a library function through a shared library base at a third fixed offset and returns without using its result.
- `toggle_slot_and_run_helpers`: Toggles a shared index between zero and one, calls the function pointer stored in the selected slot, runs three helper routines and sets a shared flag to one.
- `run_helpers_when_pending`: When a shared value equals minus one, runs two helper routines around a store and sets the value to one.
- `handle_interrupt_line_countdown`: Checks four interrupt request bits.
- `record_keyboard_key_state`: Decodes a byte from the keyboard serial register by inverting and rotating it.
- `clear_byte_at_offset`: Stores a zero byte at an address formed by the base pointer plus the offset in the data register.
- `combine_direction_state_flags`: Combines direction state into a set of flag bits in a shared variable, then steps two counter values toward zero by ten each.
- `decode_joystick_port_one`: Sets direction and fire flag bits in a shared byte, with the top bit set when it is clear, and references the joystick port one registers.
- `compute_mouse_movement_deltas`: Computes horizontal and vertical movement deltas from shared counters and mouse-port values, then stores the adjusted values back into shared variables.
- `set_shared_value_minus_one`: Sets a shared 16-bit variable to all ones, which reads as minus one.
- `wait_until_shared_value_zero`: Spins until a shared variable reads zero, then returns.
- `map_nibble_bits_to_flag`: Updates a shared flag byte from the low four bits of the input, setting or clearing bit number six for each bit.
- `copy_nibble_bits_to_flag`: Copies the four low bits of the input into a shared flag byte by setting or clearing bit number six for each of them.
- `write_bit_flags_at_position`: Writes bit flags into a shared byte at the bit position given in the data register, using the input value's bits one through three in sequence.
- `write_upper_bit_flags_at_position`: Writes flags for the upper two input bits into a shared byte at the bit position given in the data register.
- `write_high_flag_bit_at_position`: Clears the bit at the position given in the data register within a shared byte, then sets it again when the input has the eight bit set.
- `clear_bit_at_position`: Clears the bit at the position given in the data register within a shared byte.
- `fill_rows_with_four_word_pattern`: Calls four helper routines, then fills 200 rows of a 32000-byte area with a repeating four-value pattern read from a fixed table, writing backwards from the end of the area.
- `fill_rows_with_word_pattern`: Fills 200 rows of a 32000-byte area with a repeating four-value pattern read from a fixed table, writing backwards from the end of the area.
- `call_single_helper_routine`: Calls one helper routine and returns.
- `copy_eight_longs_in_place`: Walks eight 32-bit values from a shared pointer and writes each one back to the same position, so as decoded it has no visible effect.
- `wait_for_blitter_idle`: Calls five helper routines, waits until the blitter busy flag clears, then calls one more helper routine.
- `program_blitter_for_rectangle`: Waits for the blitter to go idle, then programs its source and destination pointers, modulo values, control words and a size computed from the width and height inputs.
- `forbid_multitasking_and_call_library`: Calls a library function through a shared base at a fixed offset, then calls the exec function that disables multitasking, and returns the data registers unchanged.
- `call_library_then_permit`: Calls a library function through a shared base at a fixed offset, then calls the exec function that re-enables multitasking, and returns the data registers unchanged.
- `clear_table_and_set_top_bit`: Clears a block of 532 16-bit words at a fixed address, then writes the value 32768 into the word just after the block.
- `store_two_words`: checks a global variable and, if it is zero, calls another routine and then stores two 16-bit values into a record at an offset from a pointer.
- `store_pair_with_busy_flag`: Sets a shared flag, calls a helper, stores two 16-bit input values into a record, then clears the flag.
- `store_pair_after_flagged_helper`: Calls a helper, then sets a shared flag while it stores two 16-bit input values into a record, and clears the flag afterwards.
- `store_value_after_helper`: Calls a helper routine, then stores a 16-bit input value into a fixed field of the record in the address register.
- `start_audio_channel_sample`: Chooses one of four audio channels from the low bits of the index, points it at a sample address with a period and volume, waits a short countdown, then writes its final volume and length and updates its enable bits.
- `store_library_result_and_release`: Calls a library function through a shared base.

### logged disk load from Star Wars - Return of the Jedi.adf

- `stub_bad_instruction_data`: The decoded block is only undecodable instruction data, so it has no recoverable behaviour of its own.
- `empty_routine`: The routine contains no valid executable instructions.
- `add_rotated_bytes_into_destination`: Walks a list of entries and, for each entry, adds a bit-rotated byte into a destination array at offsets taken from the entry, repeating for the entry's count.
- `busy_wait_for_count`: Counts the data register down to zero as a delay, then returns.
- `copy_table_words_into_frame`: Stores sixteen constants from a fixed table into a stack frame, then copies thirteen words from a second table into a fixed buffer and records the buffer address in the frame.
- `save_registers_to_fixed_memory`: Copies bytes from a source into a destination up to a limit, then saves the data and address register values and the stack pointer into fixed memory locations.
- `store_registers_without_copy`: Stores a zero and then the data and address register values into fixed memory locations, the same save as its neighbour without the byte copy.

### logged disk load from Star Wars - Return of the Jedi.adf

- `save_registers_and_status_flags`: Stores the data and address register values into fixed memory locations and records a status word built from the extend and condition flags.
- `save_frame_of_eight_longs`: Saves the input value and the user stack pointer into a frame of eight 32-bit slots.

### logged disk load from Star Wars - Return of the Jedi.adf

- `empty_routine_2`: This routine is empty and contains no executable instructions.
- `empty_routine_3`: The routine is empty and contains no executable instructions.
