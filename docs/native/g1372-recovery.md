# Game 1372 (james pond 2 - codename robocod (1991)(millennium)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01372). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (950976 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 97 raw-word lines |
| Functions named and described | pass, 219 of 219 named (0 author symbols), 219 of 219 described (6 by rule, 213 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1770 of 1770 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7111 of 8695 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28848 |
| code | 31886 |
| data | 733570 |
| unused | 156672 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf:bootblock | 1 | 16 | 1 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf boot-block load | 1 | 1438 | 5 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5486 | 163 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1361 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 394 | 22 |

## Cases handled

- james pond 2 - codename robocod (1991)(millennium)[cr skr]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf: custom-format (non-DOS) disk, 1452 non-empty blocks: matched against track-loaded segments
- james pond 2 - codename robocod (1991)(millennium)[cr skr]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1348 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 152 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 270 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- james pond 2 - codename robocod (1991)(millennium)[cr skr]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf: 2 logged loads cover 12288 bytes of the image
- james pond 2 - codename robocod (1991)(millennium)[cr skr]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf: 12 track-loaded blocks found in recovered runtime segments and classified through them
- james pond 2 - codename robocod (1991)(millennium)[cr skr]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf: blocks 4-1215 (bytes 2048-622592) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.63)
- james pond 2 - codename robocod (1991)(millennium)[cr skr]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf: blocks 1219-1238 (bytes 624128-634368) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.54)
- james pond 2 - codename robocod (1991)(millennium)[cr skr]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf: blocks 1242-1261 (bytes 635904-646144) outside the filesystem hold data, not code (opcode density 0.020, entropy 5.71)
- james pond 2 - codename robocod (1991)(millennium)[cr skr]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf: blocks 1265-1284 (bytes 647680-657920) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.50)
- james pond 2 - codename robocod (1991)(millennium)[cr skr]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf: blocks 1288-1431 (bytes 659456-733184) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.24)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf:bootblock

- `perform_dma_io`: Sets up an I/O request structure in a6 with specific command, buffer, and length values, calls an I/O routine, updates a global variable, performs a second I/O operation with different length values, and jumps to a routine at a fixed address.

### logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR].adf boot-block load

- `extract_stream_data`: Reads a longword from a memory stream in a0, then enters a loop that calls another routine to extract bits and stores bytes into a table at a3 based on the extracted values.
- `test_d5_then_return`: Moves a longword from the address in a0 into d5, rotates d5 right by one, and returns if the resulting value is zero; otherwise it returns immediately.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `extract_bits_from_stream`: Extracts a longword from a memory stream by shifting bits through d5 and d1, then uses the resulting bits to index into a table and perform conditional calls.
- `perform_unnamed_operation`: If a global variable is zero, it performs a call to another routine with d0 bitwise ORed with 70, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `program_blitter_with_busy_wait`: Adjusts two global offsets by a sign-dependent step and a shift of a value, then waits until a busy flag clears.
- `extract_bit_pattern_and_test_bits`: Reads a series of 16-bit words from memory using various address registers and performs a sequence of bit tests on the resulting value in a data register.
- `blit_indexed_glyph_variant`: Calculates a source and destination address from a table and index, then programs the blitter registers to draw a specific graphic area with varying widths and masks.
- `parse_control_string`: Manipulates several pointers and registers to process a string, calls another routine, and performs conditional branches based on a byte value.
- `update_global_accumulator`: Compares d0 against several constants; if d0 is negative and matches specific values, it calls another routine, loads a byte from a table using d0 as an index, and adds it to a global word at offset 7e9c, then adds a word from offset 7eae to that same global word.
- `update_global_offset`: Adds 15 to a global word, then adds a word from another global location to that same global word, updates a third global word, and branches to another routine.
- `store_extended_byte`: Loads a byte from a pointer in a6, extends it to a word, stores it in a global word, and branches to another routine.
- `add_extended_byte`: Loads a byte from a pointer in a6, extends it to a word, adds it to a global word, and branches to another routine.
- `add_signed_byte_to_global`: Loads a byte from a location pointed to by a6, sign-extends it to a word, adds it to a global word at offset 1e9c, and branches to another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `compute_blitter_shift_counts`: Scans six words in turn for the first word with bit 0 set and separately the first with bit 1 set, each defaulting to minus one when none is found.
- `blit_rows_with_pointer_step`: Waits for the blitter to be idle, then for each unit of the input count loads a source and a destination pointer into the blitter and advances both pointers by the same step.
- `set_blitter_full_copy_control`: Writes two all-ones values to the blitter's first and last word masks, zeros the modulo and shift registers, and writes a control value that selects the blitter's channels.
- `start_blit_to_destination_pointer`: Waits for the blitter to be idle, sets full masks and a control value, then writes a destination pointer taken from an address register and a size taken from the data register, starting the blit.
- `start_blit_from_two_pointers`: Waits for the blitter to be idle, sets full masks and a control value, then writes a source pointer, a destination pointer and a size from its inputs, starting the blit.
- `blit_four_bitplane_rows`: If the low nine bits of the input are nonzero, it starts four blits into four bitplanes spaced by a fixed stride, using one of two modulo settings chosen by bit 9.
- `configure_blitter_graphics_parameters`: Calculates source and destination addresses, dimensions, and color modes for a blitter operation using values from a structure in a6, then updates various blitter control and size registers and modifies a global variable via a pointer in a5.
- `wrap_single_routine_call`: Saves the status register, disables interrupts, loads two pointers into a0 and a1, calls another routine, performs a self-assignment on a global word, and restores the status register.
- `execute_complex_sequence`: Calls several routines, clears a word in a global variable, and if a global variable is not zero, performs a loop that writes four 16-bit values into a table based on values from a structure.
- `update_hardware_and_tables`: Calls multiple routines while conditionally writing specific values to the custom chip color register, and if a global variable matches certain values, performs a loop that writes four 16-bit values into a table based on values from a structure.
- `update_wrapped_position_record`: Subtracts offsets from a position record and derives a small step from the low nibble of each speed field, limited to a magnitude of 32.
- `process_conditional_updates`: Loads a pointer from a global variable, clears a word in a global variable, and calls two other routines.
- `lookup_doubled_table_value`: Takes the upper bits of a global value, scales them by two to form a byte offset, and reads a word from a fixed table at that offset.
- `compute_aligned_record_offset`: Multiplies a 16-aligned value by 38 and adds it to a field of an input record, plus a fixed bias of 7904.
- `setup_blitter_loop`: Calculates an address using values from a table, calls another routine, and then executes a loop eighteen times that calls another routine.
- `blit_multiple_rows`: Calculates a blit stride and destination address, then executes a loop that calls a routine to blit rows while updating several global values and a pointer.
- `store_double_and_repeat_calls`: Stores twice a global value into a record field, then calls a routine fifteen times in a counted loop.
- `setup_blit_parameters`: Calculates a value by adding a word from offset ae0 to a word from offset a98 and adding 36, then loads a pointer from offset afa and calls another routine, eventually setting a word at offset 1416 and branching to another routine.
- `calculate_and_blit_multiple_rows`: Calls another routine, writes values to the blitter mask registers, calculates several values using multiplication and bit shifts, and calls another routine twice.
- `initialise_fixed_state_values`: Writes a fixed set of constant values into several absolute memory locations and globals, including zeros and the address of a shared global.
- `build_copper_list_pointer_table`: Builds a four-entry pointer table as its neighbours do, splitting a base value into high and low words and stepping it by a global increment.
- `toggle_between_two_pointer_sets`: Switches a state global between two sets of values.
- `perform_bit_wise_or_sequence`: Performs a sequence of bitwise OR operations on the contents of register d0 using various immediate byte values.
- `execute_indirect_jump_via_table`: Loads a word from a global variable, performs a bitwise AND, uses a shifted version of another global value to index into a table, and jumps to the resulting address.
- `start_conditional_blit_for_column`: When a global flag is set, it derives a column offset and a row size from its inputs, writes blitter control and mask values, and starts a blit.
- `adjust_scrolling_offset_and_bounds`: Compares a 16-bit value in a global variable against a 32-bit value in another global variable, then modifies both values by adding or subtracting constants based on the comparison results and a bitmask check.
- `execute_indirect_jump_sequence`: Performs a sequence of bitwise OR operations on register d0 and then executes a routine at an address calculated from a global variable and d0.
- `call_twice_and_jump_indirect`: Loads two different pointers into a0 and calls another routine twice with the value 3413 in d0, then uses a shifted global value to index into a table and jumps to the resulting address.
- `copy_tile_rows_interleaved`: Copies data for sixteen groups of sixteen rows from a source table into a destination buffer.
- `initialize_hardware_colors`: Loads a pointer from a global variable and the address of the custom chip into a4.
- `clear_matching_table_records`: Walks a table of 26-byte records while counting down a global value.
- `process_records_by_threshold_checks`: Walks a table of 26-byte records while counting down a global value.
- `prepare_blitter_operation_parameters`: Calculates source and destination addresses, dimensions, and color modes based on a structure in a6, then populates a blitter control structure in a4.
- `find_first_zero_slot_entry`: Scans up to 32 entries of 10 bytes each, starting from a pointer held in a structure.
- `clear_active_slot_entries`: Sets full blitter masks and a control value, then walks up to 32 ten-byte entries from a structure pointer.
- `blit_four_planes_with_wrap`: Waits for the blitter, copies a value into two fields of a record, and computes a start address and a row split from a count compared against 224.
- `call_four_times_then_clear`: Sets a global value to a fixed constant, calls one routine four times, then clears three globals to zero.
- `clear_field_in_record_table`: Clears a 16-bit field at offset 14 in each of 32 consecutive 26-byte records, starting from a fixed address, and then zeroes a global.
- `invert_underscore_marker_byte`: When a global byte equals the underscore character, it sets the byte to zero and then inverts it, leaving all bits set.
- `branch_to_marker_inversion`: Loads the address of a color register into a0, loads a pointer into a1, and branches to another routine.
- `stub_infinite_loop_halt`: Contains only an unconditional loop with no exit, so calling it never returns.
- `poll_routine_until_value`: Calls a routine repeatedly until it returns 252, unless a global flag is already set.
- `setup_four_audio_channels`: Points each of four audio channels at a shared buffer and sets each channel's length to one word, then writes an enable value to the DMA control register for those channels.
- `read_swapped_nine_bit_position`: Returns the low nine bits of a global value after swapping its two bytes.
- `install_interrupt_vectors`: Disables all interrupts and clears pending requests, fills ten low-memory vector entries with a fixed value, and points four interrupt vector entries at a routine.
- `call_with_bit_six_set`: Sets bit six of a global, calls a routine, clears bit six again, and then sets the global to 8.
- `handle_interrupt_flag_bits`: Checks a flag bit of a global and, when it is set, calls two routines and may set the global to 255 if a returned value exceeds 251.
- `shift_keyboard_serial_bit`: When bit three of a global is set, it shifts one serial bit through the byte in inverted form, calling routines on the bit edges, and then sets the global to 8.
- `run_flagged_interrupt_callbacks`: Calls one routine when bit four of a global is set.
- `dispatch_pending_request_bits`: Checks bits seven to ten of a global request value, clearing each set bit and calling a routine for it.
- `keep_bits_eleven_to_thirteen`: Keeps only bits eleven to thirteen of a global value, clearing all other bits.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `read_joystick_direction_state`: Derives direction flags from the joystick port movement bits, combining the vertical direction as an exclusive-or of two bit positions.
- `increment_table_index_and_jump`: Compares a byte in a table at an index stored in a global variable with the value in d0.
- `apply_letter_specific_state_changes`: When a global is nonzero, applies a change chosen by the input letter.
- `push_and_branch_f00`: Pushes the word in d0 onto the stack, sets d0 to f00, and branches to another routine.
- `push_and_branch_ff0`: Pushes the word in d0 onto the stack, sets d0 to ff0, and branches to another routine.
- `push_and_branch`: Pushes a 16-bit word from register d0 onto the stack and branches to another routine.
- `write_color_loop`: Writes a sequence of values to the hardware color register by looping 10000 times, where each value is derived from a counter masked with 240.
- `advance_feedback_shift_register`: Shifts a global value left one bit and sets its lowest bit when bit 23 and bit one differ, so the value steps through a feedback shift sequence.
- `step_multiplicative_congruential_value`: Increments a global value while it is below five, returning it to one when it reaches five.
- `format_number_as_digit_string`: Converts a number to decimal digits by repeatedly subtracting successive values from a table until a zero entry, writing each count as a digit character into a fixed buffer.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `set_playfield_scroll_and_dma`: Stores its input multiplied by 448, disables all DMA channels, and sets the playfield control registers to zero or fixed values.
- `build_six_entry_pointer_table`: Stores a base value in a structure and derives a step from the input nibbles, adjusted by fixed offsets.
- `move_and_clear_table_blocks`: Copies 32 words from one fixed table to another and clears the source, then moves 256 words from a global buffer into two other tables while zeroing the source.
- `clear_fixed_word_block`: Zeroes 544 consecutive words from a fixed address and then sets a global to 30.
- `fill_word_tables_from_routine`: Fills 32 words at a fixed address with values returned by a routine, then fills two 256-entry tables from routine results, writing each entry into both.
- `compute_grey_level_color`: Adds the three nibbles of an input value to a signed offset and clamps the total to 47.
- `fill_paired_routine_tables`: Decrements a counter global and fills table entries from routine results.
- `stub_partial_decompilation`: Performs a series of comparisons and subtractions using bitwise masks on two values, modifying a third value based on the results.
- `call_routine_for_nonzero_bytes`: Walks four bytes starting at an offset from a global base.
- `scan_table_for_nonzero_entry`: Scans up to four table entries spaced 148 bytes apart, looking for a nonzero first longword.
- `advance_four_counters_with_wrap`: When the flag global is nonzero, increments four 16-bit counters.
- `process_input_bits`: Checks a global flag and, if zero, compares a byte from a pointer to a value at that pointer, potentially incrementing the pointer and updating a global byte through a series of logical shifts and status register updates.
- `initialise_stream_marker_state`: Writes the byte value 255 to the address in an address register, points the stream pointer one byte past it, zeroes the stream counter and clears a global.
- `update_ciaa_peripheral_registers`: Reads a byte from the CIAA Port A Data register, masks bits 6 and 7, and sets bit 3.
- `disable_interrupts_and_branch`: Saves the status register to the stack, sets the status register to a specific value, saves several registers to the stack, and branches to another routine.
- `call_routine_over_word_sequence`: Loops over a count held in a structure, calling a routine with each value and incrementing a global counter.
- `draw_object_plane_blits`: Waits for the blitter, sets its masks and size, and derives two screen words from record fields and the current value.
- `update_structure_data`: Copies a 16-bit value from a global variable to a location pointed to by a pointer in a global variable, then calls another routine.
- `copy_data_to_structure`: Copies ten bytes from a source buffer into a structure at a6 using a loop, then performs several conditional blitter operations based on values in a table.
- `wait_blitter_then_start_copy`: Waits for the blitter to be idle, then writes two pointer-derived values and a size of 1857 to the blitter registers.
- `clear_global_variable`: Sets a global variable to zero and enters a loop that repeatedly checks if the variable is zero, returning only when it is no longer zero.
- `advance_counter_and_call_routines`: Returns early when a global is nonzero.
- `initialize_data_structures`: Clears a block of memory by writing zeros to a table and then performs a series of sequential writes to various global variables.
- `set_all_ones_sentinels`: Fills four global bytes and six fixed longwords with all-ones values, typically a sentinel for empty entries.
- `execute_multiple_blits`: Calls a routine four times with different source addresses and a constant value in d0.
- `round_position_up_sixteen`: Calls two routines, then sets a group of globals depending on whether a global is zero or equal to another.
- `clear_twenty_one_globals`: Sets twenty-one global variables to zero in sequence.
- `set_three_conditional_triplets`: Zeroes four globals, then in three checks, when a global is nonzero, sets it to 255 and loads two fixed small values.
- `execute_state_driven_logic_table`: Scans a table of ten-byte entries until a negative word is encountered, calling various routines based on the current state and entry data.
- `clamp_fixed_point_velocity`: Nudges a fixed-point velocity global by a small increment, subtracts a constant step under a sign test, and clamps the result to a floor value.
- `update_movement_state_offsets`: Adjusts a movement state byte, then updates a position offset by a constant that depends on the state.
- `subtract_clamp_state_four_2`: Increments a byte in a global, then subtracts a value from another global and clamps the result based on several conditional checks and a routine call.
- `add_clamp_state_eight_2`: Increments a byte in a global, then adds a value to another global and clamps the result based on several conditional checks and a routine call.
- `update_state_with_speed_limit`: Clears a global when a state byte is zero.
- `advance_position_with_clamp_fifteen`: Adds a fixed offset of 40960 to a value, takes its high word and clamps it to 15.
- `call_mode_routines_then_flag`: Calls routines depending on whether bits eleven and twelve of a global match a pattern, calls two more routines, and sets a global to 96 when a flag check passes.
- `step_toward_target_within_eight`: Returns the low bits of a value masked with five.
- `update_mode_state_with_clamp`: Runs a state-driven update that clamps a value to fifteen in each direction, looks up mode-based values from the upper bits, and sets several fixed values depending on the state.
- `update_offset_clamped_three`: Adds a fixed offset of 5462 to a value, clamps its high word to plus three on the positive side and minus fifteen on the negative side, doubles it, and calls routines before and after.
- `update_offset_clamped_two`: Adds a fixed offset of 3280 to a value, clamps its high word to plus two on the positive side and minus fifteen on the negative side, doubles it, and calls routines before and after.
- `call_six_routines_clear_flag`: Calls six routines in sequence with small global assignments between them, and clears a global when a value is nonnegative and a flag is set.
- `adjust_global_value`: Compares a global variable with a value from another global variable and performs arithmetic on the first global variable based on the result and several conditional checks, including bitwise operations and shifts.
- `call_then_store_register_words`: Calls one routine and then writes two input register words into the same global variable in turn, ending with the second.
- `store_two_register_words_variant`: Matches the previous entry: it calls one routine and then writes two input register words into the same global variable in turn.
- `subtract_and_correct_sign`: Subtracts a register value from a global.
- `add_with_correction_two`: Adds a register value to a global.
- `subtract_clamp_state_four`: Subtracts a register value from a global.
- `add_clamp_state_eight`: Adds a register value to a global.
- `test_and_call`: Tests a 16-bit word at a global address and calls another routine if the value is not zero.
- `damp_value_fifteen_step_2`: Checks several globals and then moves a global toward zero by up to fifteen units, choosing the direction from a bit test on the result.
- `damp_value_fifteen_step`: Moves a global toward zero by up to fifteen units, choosing the direction from a bit test on the result.
- `add_clamped_offset_fifteen`: Clamps a 16-bit input to the range minus fifteen to fifteen, then adds it to a global.
- `check_threshold_return_pair`: Checks if two specific global words are zero, then masks a 32-bit global and compares it against a threshold of 12288 depending on its sign, calling another routine if the comparison passes and returning the global with a 16-bit result.
- `move_by_capped_steps`: When the state global equals one, it moves a position global in steps of 8192 toward or away from a zero point.
- `set_d6_and_compare`: Sets register d6 to zero, loads a 16-bit value from a global into d0, and compares d0 against a value from another global.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 1 into d6, then returns.
- `scan_table_for_matching_entry`: Scans a table for an entry whose first word matches a value derived from a global, then updates a counter in that entry, calls two routines, and sets two global values.
- `increment_indexed_counter_entry`: Clears a 16-bit global, sets another to 255, calls another routine, and then increments the byte at a fixed table address indexed by the first global.
- `update_clamped_coordinate_values`: Calculates a clamped difference between two 16-bit values and updates two global variables with the result, applying bounds checks against several constants.
- `calculate_clamped_value`: Performs arithmetic on register d1 using d3 and several global variables, then stores the result in a global variable.
- `advance_table_chain_twice`: Twice, while a global is nonzero, adds an entry from a byte table indexed by the global to the global and then decrements it.
- `execute_state_logic`: Scans a table of 10-byte entries until a negative word is found, calling routines per entry.
- `set_registers_and_branch`: Clears d0, sets d1 to an address, and branches to another routine.
- `load_registers_and_branch`: Sets d0 to an address, sets d1 to -an address, and branches to another routine.
- `update_state_parameters`: Checks and modifies various global word and byte values, including setting a long value to a fixed address and updating counters based on specific conditions.
- `set_state_seven_defaults`: If a global variable is not seven, it sets it to seven, sets a long value to a fixed address, calculates a new value by adding 224 to a value from a global, stores it in another global, clears a third global, and marks the state with 255.
- `update_global_constant`: Tests a byte in a global, and if it is zero, stores a 32-bit constant into another global before branching to another routine; otherwise it branches to another routine.
- `initialize_globals`: Stores a 32-bit constant into a global and clears a 16-bit global.
- `update_state_variable`: Modifies a global variable based on its current value, performing conditional increments or resetting it to a specific constant.
- `update_state_machine_via_recursion`: Evaluates a global variable through a series of conditional checks and calls itself recursively with updated values.
- `resolve_relative_table_entry`: Steps a table index back by one and reads a 16-bit relative entry from a table.
- `lookup_tile_flags_at_position`: Indexes a fixed table with the high bits of a position to find a tile word.
- `lookup_tile_word_with_links`: Finds a tile word from a position using a fixed index table, then follows one or two linked neighbouring words while their top flag bits are set, and stores the result.
- `modify_color_value_2`: Subtracts a 16-bit value from a global variable and performs conditional arithmetic on that global variable based on the result and comparisons with a constant.
- `modify_color_value`: Compares a 16-bit value in d1 with 21 and performs arithmetic on a global variable based on the sign of d1 and comparisons with a value from another global variable.
- `add_to_global_word`: Adds the value in register d2 to a 16-bit global variable and returns.
- `subtract_if_greater`: Compares the values in registers d1 and d2, and if d1 is greater than or equal to d2, subtracts d1 from a 16-bit global variable at offset 22ea.
- `subtract_from_global_word`: Subtracts the value in register d2 from a 16-bit global variable and returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d6, then returns.
- `stub_partial_decompilation_2`: The routine performs several comparisons using values from a structure in a0 and global variables, but the decompiler only provides a return value in d6.
- `check_coordinate_overlap`: Calculates the width and height of two rectangles using values from a structure in a0 and global variables, then compares their bounds to determine an overlap status stored in d6.
- `stub_partial_decompilation_c`: The routine performs arithmetic on registers d0 and d1 and uses them as offsets to read from tables in a2 and a4, but the decompiler only provides a return.
- `return_inverted_masked_input`: Returns the bitwise inverse of an input register, masked to bits two and three.
- `reflect_global_when_flagged`: Calculates a value in d0 from two globals, calls another routine, and if d6 has bits 15, 14, 12, 11, or 10 set and the called routine returns nonzero, sets a global to minus three minus its current value.
- `reflect_global_from_three`: Stores a value from another routine into a global, and if d6 has bits 15, 14, 12, 11, or 10 set and a second routine returns nonzero, sets that global to three minus its value.
- `update_entity_state_and_flags`: Checks various global status and position variables to update a state byte and a set of bitwise flags.
- `check_pattern_and_set_state`: Checks an input word against a mask pattern, with the pattern depending on whether a global is zero.
- `process_active_entries_in_table`: Walks 32 entries of 40 bytes each, counting entries whose code byte lies outside a small range and processing active ones with routine calls.
- `scale_byte_field_by`: Returns the byte at offset one of a structure, multiplied by 64.
- `clear_and_test_globals`: Clears two 16-bit global variables, compares a third global variable to seven, and then tests bits in a structure in a0 and a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_clipping_offsets`: Checks a condition in a global variable and, if met, calculates new sprite dimensions and clipping offsets by subtracting values from a width constant and a pointer-based value, then calls build_sprite_record_variant twice to update a sprite record structure.
- `build_sprite_record_variant`: Calculates sprite dimensions, horizontal mirroring, and clipping offsets based on input data, then stores these values into a record structure and advances a list pointer by 26 bytes.
- `allocate_record_from_table`: Checks a global byte, sets a hardware register, and if a global word is non-zero, iterates through a table of records to find one matching specific range criteria, then calls various routines to initialize it.
- `call_allocate_slot`: Loads a zero value into register a2, calls allocate_slot_with_pointer, and then branches to another routine.
- `branch_to_allocate_slot`: Loads a zero value into register a2 and then branches to allocate_slot_with_pointer.
- `allocate_slot_with_pointer`: Searches a table for a free slot by checking a byte at offset 1, then initializes the record with various values, a 32-bit pointer from a register, and a byte from a calculated offset in another table.
- `offset_record_position_variant`: Subtracts values from offsets 2 and 6 of a structure based on data in another structure, and if a mode field is not one, calls another routine and masks a word into offset 6.
- `process_sprite_record_list`: Iterates through a list of sprite records, updating their animation frames, bounds, and motion via several subroutines.
- `select_record_animation_frame`: Checks a global word, then either adjusts a vertical position in a structure, sets state bytes based on a global sign, or stores a frame code derived from a flag word and a constant.
- `set_record_frame_code`: Chooses a value of 5, 6, or 11 based on bits 7 and 8 of a word, writes a specific word to that location, and then sets four subsequent fields to that value if they are non-zero.
- `set_size_globals_from_field`: Sets two global words to 32, then if a byte at offset 31 is non-zero, sets the first global to 16 and the second to the value of the byte at offset 32 shifted left by four.
- `update_structure_field_and_global`: Checks bit flags in a structure pointed to by a register and performs conditional arithmetic on a global variable based on its current value and values stored at offsets 2 and 6 of the structure.
- `check_record_range_and_store`: Checks if a value derived from a global and a field in a structure falls within a specific range, and if so, decrements a global, sets a field to 10, and stores a calculated value into a global.
- `update_color_and_sprite_type`: Tests a byte in a global variable; if it is not zero, it writes values to the color register and a global variable, then tests bit 3 of a structure in a0 and branches accordingly.
- `update_sprite_record_data`: Processes a sprite record structure by checking its type and updating various memory offsets through a series of conditional checks and calls to free_or_write_record_sprites and calculate_table_offset.
- `update_sprite_record_bounds`: Updates several global variables with values from a structure in a0, decrements a counter at offset 52 of the structure, and adds a value from offset 52 to a global variable at offset 6 of the structure before calling another routine.
- `free_or_write_record_sprites`: Checks if a record's pointer and range values fall within specific memory bounds, otherwise it clears a byte at offset 38 of the structure, decrements a global counter, and either updates the record or copies its sprite coordinate data into a shared buffer based on a layout byte at offset 31.
- `dispatch_record_code_events`: Reads a code byte from a record and updates a global variable or state based on several specific code values.
- `store_byte_after_call`: Calls another routine and stores the resulting byte from register d0 into a field at offset 24 of the structure pointed to by a0.
- `allocate_record_and_copy_parts`: Searches a table for an empty entry, initializes the record with provided values, and then copies up to eight 16-bit values from a buffer into the record, stopping if a value is zero or equals fifty-five, and stores the count of copied parts in a status byte.
- `calculate_scaled_sum`: Shifts two 16-bit values right by four bits, doubles each resulting value, multiplies one of these doubled values by a constant from memory, and returns the sum of the first doubled value and the product.
- `flush_pending_entries_to_targets`: Iterates through a table of 32 records, and for each non-zero entry, copies the value to an offset of 4 in a target structure pointed to by the record, clears the entry, and decrements a global count.
- `step_countdown_position_motion`: Decrements a byte at offset 33 of a structure and adds a value from a table to offset 6, setting the byte to a sentinel if it reaches zero or a specific value, or clearing it if the structure is not the active one.
- `populate_bitmask_table`: Populates a table by performing bitwise OR operations with various byte constants into a 16-bit register.
- `process_sprite_record_and_update_memory`: Checks bit 5 and bit 10 of a word in a structure to determine if it should perform a conditional update or call a function via a table offset.
- `toggle_status_bit`: Clears bit 3 and then sets bit 0 of a byte at offset 30 of the structure pointed to by a0, then returns.
- `calculate_table_offset`: Loads a base address into a1, masks d0 to keep the lower 9 bits, shifts d0 left by 7 bits, and updates a1 with the resulting offset from the base address.
- `append_clamped_object_record`: Checks if a global pointer is below a specific threshold, otherwise it calls another routine; if valid, it calculates clamped horizontal and vertical offsets, sets a state word and a sprite-style flag in a structure pointed to by a6, and advances the global pointer by 26 bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_address`: Performs a long jump to a specific address.
- `service_pending_interrupt_bits`: Repeatedly tests four bits of a global pending mask in order, clearing each one as it goes, and calls a handler for a bit when the matching interrupt request bit is set in the custom chip registers.
- `update_timed_position_or_expire`: Decrements a counter in an object, and while the object's direction flag is negative it writes an absolute position into a list entry, computed from the object's base value and a stored offset.
- `load_address_pointers`: Loads four consecutive memory addresses into register a4 by calling a routine after each load.
- `latch_object_scroll_values`: Writes a value from the object into a buffer entry, computes a clamped difference between two object fields, and stores it in a second entry.
- `load_address_and_call`: Loads four different addresses into a4 and calls another routine after each load.
- `decrement_stage_counter`: Checks a longword at offset 58 of a structure; if it is not zero, it decrements the byte at offset 48 and calls another routine, otherwise it returns.
- `step_level_ramp_stages`: Stage parameters are read through the record pointer.
- `advance_three_record_command_streams`: Iterates over three fixed-size records, each with an active command pointer, reading the next byte-coded command from its stream.
- `store_byte_and_branch`: Copies a byte from a pointer in a2 to an offset of 68 in a4 and then branches.
- `fetch_byte_and_branch`: Copies a byte from a pointer in a2 into d0 and then branches.
- `update_pointer_and_branch`: Loads a pointer from an offset of 54 in a4 into a2, stores that pointer at an offset of 58 in a4, and then branches.
- `multiply_byte_and_branch`: Copies a byte from a pointer in a2 to d0, multiplies d0 by a word in a6, and stores the result in an offset of 64 in a4 before branching.
- `divide_byte_and_branch`: Copies a byte from a pointer in a2 to d0, divides the constant 750 by d0, and stores the result in a memory location pointed to by a6 before branching.
- `loop_multiply_and_branch`: Decrements a byte from a pointer in a2, then enters a loop that adds bytes from the same pointer to a running total for the duration of the decremented value.
- `load_command_stream_data`: Calculates a source address from a value in a register, then copies 14 bytes from that location into a structure.
- `fetch_command_byte`: Moves a byte from the address pointed to by a2 into d0 and then branches to another location.
- `calculate_table_index`: Moves a byte from the address pointed to by a2 into d0, multiplies d0 by 14, adds a constant to d0, and then uses the result to load several words and a longword from a table into offsets of a structure pointed to by a4 before branching to another location.
- `load_indexed_pointer`: Moves a byte from the address pointed to by a2 into d0, multiplies d0 by 4, loads a pointer from a table using d0 as an index into a2, and then branches to another location.
- `copy_word_field_triples`: Copies two word fields to two other offsets in a structure, advancing the base pointer two bytes per pass, and repeats for three passes.
- `copy_word_field_pairs`: Copies two word fields to two other offsets in a structure, advancing the base pointer two bytes per pass, and repeats for two passes.
- `clear_d_registers`: Clears twelve d0 through d3 registers by performing a bitwise OR with zero on each.
