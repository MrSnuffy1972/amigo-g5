# Game 1632 (james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01632). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (950976 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 97 raw-word lines |
| Functions named and described | pass, 219 of 219 named (0 author symbols), 219 of 219 described (4 by rule, 215 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1770 of 1770 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7122 of 8706 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 30895 |
| code | 31944 |
| data | 767305 |
| unused | 120832 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf:bootblock | 1 | 27 | 1 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf boot-block load | 1 | 0 | 0 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf boot-block load | 1 | 1438 | 5 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5486 | 163 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1361 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 394 | 22 |

## Cases handled

- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf: custom-format (non-DOS) disk, 1522 non-empty blocks: matched against track-loaded segments
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf: the boot block reads 8192 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1348 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 152 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 270 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf: 3 logged loads cover 19456 bytes of the image
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf: 12 track-loaded blocks found in recovered runtime segments and classified through them
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf: blocks 18-1215 (bytes 9216-622592) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.66)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf: blocks 1219-1238 (bytes 624128-634368) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.54)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf: blocks 1242-1261 (bytes 635904-646144) outside the filesystem hold data, not code (opcode density 0.020, entropy 5.71)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf: blocks 1265-1284 (bytes 647680-657920) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.50)
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +5 ache extensors]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf: blocks 1288-1431 (bytes 659456-733184) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.24)
- 1 packed track region(s), 35840 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf:bootblock

- `initialize_boot_parameters`: Sets or modifies several values in a structure and calls external functions or or other routines.

### logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +5 Ache Extensors].adf boot-block load

- `extract_bit_stream_logic`: Iteratively processes a bitstream or data stream, performing conditional pointer arithmetic and memory copies or checks based on certain bit patterns.
- `test_register_upper_bits`: Checks if the second bit of a register is set and returns without modifying any registers.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `extract_bit_stream`: Iteratively checks or shifts bits from a register and a pointer-referenced memory location.
- `execute_branch_if_null`: Checks if a global variable is zero and, if so, calls another routine with a modified argument.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `program_blitter_with_busy_wait`: Adjusts global offsets, waits for a hardware busy flag to clear, and writes masking or size values into blitter registers before computing a new address.
- `calculate_table_offset_and_pointer`: Calculates a pointer to a data structure or table using a base address and a scaled or offset-based index.
- `setup_blitter_indexed_glyph`: Computes a table entry from an input register and programs the blitter's source, destination, mask, and size registers to draw a specific glyph or pattern.
- `parse_string_tokens`: Iterates through a string, updating a global variable or pointer based on specific character triggers or values found in a table.
- `parse_custom_string_tokens_2`: Parses a string, updating a global variable or pointer based on specific character triggers or values found in a table.
- `parse_string_with_values`: Parses a string, updating a global variable based on specific character triggers or values found in a table.
- `parse_string_with_offsets`: Iterates through a string and updates a global variable using values or offsets found at certain string positions.
- `parse_string_with_additions`: Processes a string-based command or data stream, adding or setting a global variable based on character-triggered logic or table lookups.
- `parse_custom_string_tokens`: Parses a string or a buffer, updating a global variable or a pointer based on certain character or word values found in the buffer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_blitter_shift_offsets`: Scans a table of six words to find the first entry with bit 0 or bit 1 set, setting a global variable to -1 if no match is found.
- `blit_rows_with_pointer_step`: Waits for the blitter to be idle and then updates a source and destination pointer by a fixed amount for a specified number of iterations.
- `set_blitter_full_copy_control`: Sets blitter mask and control registers to a fixed state for a full-width copy with no modification.
- `start_blit_to_destination_pointer`: Waits for the blitter to be idle, configures blitter masks and control, and writes a destination pointer and a size or enoughs to start a transfer.
- `start_blit_from_two_pointers`: Waits for the blitter to be idle, configures blitter masks and control, and writes a source pointer, a destination pointer, and a size to start a transfer.
- `blit_four_bitplane_rows`: Configures DMA certain bitplane-drawing or masking operations by setting a series or certain number of blits with specific source/destination/control-register values, and returns a value derived from the input or a global variable.
- `stub_partial_decompilation`: A routine that performs several assignments to a global variable, including setting it to zero and a maximum value, while touching various blitter and any DMA registers.
- `wrap_single_routine_call`: Calls one unnamed routine and then performs a self-assignment with no effect, so it acts only as a thin wrapper around that call.
- `build_pointer_table`: Sets a pointer to a structure and populates a series of table entries with a modified, incrementing address or value.
- `populate_split_address_table`: Calculates a sequence of 32-bit addresses and stores them into a table in split high-word and low-word format.
- `update_wrapped_position_record`: Subtracts offsets from a position record, derives a step from a speed field, and updates a new position that wraps certain or certain boundaries.
- `execute_recursive_loops`: Performs a series of recursive or nested calls to another routine while modifying a global variable through a decrementing or incrementing loop.
- `lookup_doubled_table_value`: Takes the upper bits of a global value, scales them by two to form a byte offset, and reads a word from a fixed table at that offset, then doubles that word and stores it back to a global.
- `compute_aligned_record_offset`: Multiplies a 16-aligned value by 38 and adds it to a field of an input record, plus a fixed bias, then aligns the sum down to a 16-unit boundary.
- `update_blitter_addresses`: Calculates and stores new destination addresses or offsets into a register or memory location, then executes a loop that modifies a global variable.
- `process_blitter_address_sequence`: Calculates or modifies memory or register values based on a global variable and executes a loop that updates a pointer or value with an offset.
- `store_double_and_repeat_calls`: Stores twice a global value into a record field, then calls a routine fifteen times in a counted loop.
- `execute_blitter_copy_loop`: Sets a destination address relative to a global value and executes a routine a fixed number of times.
- `update_blitter_pointers_linearly`: Updates a global variable using a calculation, then uses a loop to increment two pointer-like values by a shared offset.
- `initialise_fixed_state_values`: Writes a fixed set of constant values into several absolute memory locations and certain globals, including zeros and a pointer to a shared global.
- `build_copper_list_pointer_table`: Builds a four-entry pointer table by splitting a base value into high and low words and stepping it by a global increment, while also writing to a copper list pointer.
- `toggle_between_two_pointer_sets`: Switches a state global between two different sets of pointer or address values based on its current value.
- `execute_indirect_subroutine`: Calculates a target address by offsetting a base address with a value from a global, then jumps to that address.
- `dispatch_indirect_call`: Calculates a target address by adding a scaled index from a global variable to a base address and jumps to the resulting location.
- `start_conditional_blit_for_column`: When a certain flag is set, it calculates a column offset and row size to configure blitter control, mask, and size registers before starting a blit, potentially triggering a second blit if certain conditions are met.
- `initialize_blitter_control_register`: Sets a global or register-like value to a specific bitmask and waits for a certain bit to clear.
- `execute_indirect_subroutine_with_pre_call`: Executes a routine and then performs an indirect jump to a target address calculated using a global value as an offset.
- `call_twice_and_jump_indirect`: Executes a routine twice and then performs an indirect jump to a target address calculated using a global value as an unable offset.
- `copy_tile_rows_interleaved`: Copies data for sixteen groups of sixteen rows from a source table into a destination buffer.
- `update_blitter_registers`: Modifies several memory locations or hardware registers with a fixed constant value based on the current state of a global variable.
- `clear_matching_table_records`: Walks a table of 26-byte records while counting down a global value.
- `process_records_by_threshold_checks`: Walks a table of 26-byte records while counting down a global value.
- `update_sprite_attribute_table`: Calculates and stores new attribute values, including certain bitmask-based masks and coordinate offsets, into a destination structure.
- `find_first_zero_slot_entry`: Scans up to 32 entries of 10 bytes each, starting from a pointer held in a structure.
- `clear_active_slot_entries`: Sets a global value, then walks up to 32 ten-byte entries from a structure pointer, clearing a specific word if it is non-zero.
- `blit_four_planes_with_wrap`: Waits for the blitter, copies a value into two fields of a record, and computes a start address and a row split from a count compared against 224.
- `call_four_times_then_clear`: Sets a global value to a fixed constant, calls one routine four times, then clears three globals to zero.
- `clear_field_in_record_table`: Clears a 16-bit field at offset 14 in each of 32 consecutive 26-byte records, starting from a fixed address, and then zeroes a global.
- `invert_underscore_marker_byte`: When a global byte equals the underscore character, it sets the byte to zero and then inverts it, leaving all bits set.
- `copy_data_block`: Copies a block of data from a source address to a destination address using a loop that processes several words at a time.
- `stub_infinite_loop_halt`: Contains only an unconditional loop with no exit, so calling it never returns.
- `poll_routine_until_value`: Calls a routine repeatedly until it returns 252, unless a global flag is already set, and clears the flag before returning.
- `setup_four_audio_channels`: Points each of four audio channels at a shared buffer and sets each channel's length to one word, then writes an enable value to the DMA control register for those channels.
- `read_swapped_nine_bit_position`: Returns the low nine bits of a global value after swapping its two bytes.
- `install_interrupt_vectors`: Disables interrupts, fills a sequence of low-memory vector entries with a fixed value, and sets certain interrupt vector entries to point to a routine before writing to interrupt enable and disable masks.
- `call_with_bit_six_set`: Sets bit six of a global, calls a routine, clears bit six again, and then sets the global to 8.
- `handle_interrupt_flag_bits`: Checks a flag bit of a global and, if certain conditions are met, calls other routines and potentially sets a global to a certain value before masking a global against a certain bitmask.
- `shift_keyboard_serial_bit`: When bit three of a global is set, it shifts one serial bit through the byte in inverted form, calling routines on the bit edges, and then sets the global to 8.
- `run_flagged_interrupt_callbacks`: Calls one routine when bit four of a global is set.
- `dispatch_pending_request_bits`: Checks bits seven to ten of a global request value, clearing each set bit and calling a routine for it.
- `mask_global_interrupt_bits`: Clears all bits of a global variable except for bits eleven through thirteen.
- `return_immediately`: The routine is empty and performs no operations before returning.
- `update_joystick_input_state`: Reads the joystick port and certain button or disk status flags to calculate direction flags using bitwise operations and an exclusive-or, then calls a routine if the resulting state is zero or if certain bit conditions are met.
- `apply_letter_specific_state_changes_2`: Checks a character against a table and modifies a global variable or a memory location based on certain character matches.
- `apply_letter_specific_state_changes`: If a global variable is nonzero, certain input characters cause it to be inverted, reset to zero, or set to a fixed value.
- `set_global_masked_loop`: Iteratively writes a masked version of a descending counter to a global variable.
- `set_global_masked_loop_alt`: Iteratively writes a masked version of a descending counter to a global variable using a different mask.
- `decrement_register_loop`: Iterates through a loop that writes a descending sequence of masked values to a register.
- `set_register_to_constant_loop`: Iterates through a loop that writes a descending sequence of values to a register, masking the result.
- `advance_feedback_shift_register`: Shifts a global value left one bit and sets its lowest bit if certain or otherwise certain conditions between bit 23 and bit 1 are met.
- `step_multiplicative_congruential_value`: Increments a global value if it is below five, otherwise resets it to one, and then applies a multiplicative or modulo operation.
- `format_number_as_digit_string`: Converts a number to decimal digits by repeatedly subtracting values from a table, writing characters to a buffer, and replacing leading zeros with spaces.
- `halt_execution`: the routine contains invalid or undefined instruction data that prevents any executable logic or control flow or any instruction or register manipulation or any data access or any return.
- `set_playfield_scroll_and_dma`: Stores an input multiplied by a constant, disables DMA channels, and sets certain hardware control registers to fixed values.
- `build_six_entry_pointer_table`: Calculates a pointer or value and writes a sequence of or split 32-bit entries into a table using a loop.
- `move_and_clear_table_blocks`: Copies 32 words from one fixed table to another and clears the source, then moves 256 words from a global buffer into two other tables while zeroing the source.
- `clear_fixed_word_block`: Writes zero to a sequence of words starting at a fixed address and then sets a global variable to a fixed value.
- `fill_word_tables_from_routine`: Fills 32 words at a fixed address with values returned by a routine, then fills two 256-entry tables from routine results.
- `compute_grey_level_color`: Calculates a value based on the sum of nibbles from an input, clamps it, and returns a masked result.
- `fill_paired_routine_tables`: Decrements a counter global and fills table entries from routine results, choosing between different table sizes based on a bitmask.
- `stub_partial_decompilation_2`: This routine is empty and only performs a return.
- `call_routine_for_nonzero_bytes`: Iterates through a range of bytes starting from a global offset and calls another routine if a byte is nonzero.
- `scan_table_for_nonzero_entry`: Scans a table at fixed intervals for a non-zero longword and returns the original input register.
- `advance_four_counters_with_wrap`: If a certain flag is set, it increments four 16-bit counters and checks if the first counter exceeds a certain value.
- `update_status_from_memory`: Checks a byte from a pointer and updates a global variable or a counter based on specific bit conditions.
- `initialise_stream_marker_state`: Writes the byte value 255 to an address, increments a pointer, and zeroes a counter and a global variable.
- `update_control_status`: Sets a bit in a certain flag and resets a global variable.
- `modify_hardware_register_and_store_data`: Updates a bit in a hardware register and stores a 16-bit value from a register into a memory location.
- `call_routine_over_word_sequence`: Loops over a count held in a structure, calling a routine with each value and incrementing a global counter.
- `setup_blitter_plane_draw`: Configures the blitter control register, calculates screen word buffers, and sets the blitter's source, destination, and masking parameters to prepare for plane-based drawing.
- `update_string_and_buffer`: Compares a source string to a destination buffer, writes any differing characters, and updates a length-limited buffer or pointer.
- `copy_data_with_blitter_params`: Copies a fixed number of words from a source buffer to a destination buffer while updating hardware registers.
- `wait_blitter_then_start_copy`: Waits for the blitter to be idle, then writes two pointer-derived values and a size of 1857 to the blitter registers.
- `reset_and_wait_for_zero`: sets a global variable to zero and then enters a busy-wait loop until that same global variable is confirmed to be zero.
- `advance_counter_and_call_routines`: Returns early if a global is nonzero, otherwise it increments the global, potentially sets it to a large value, and calls several other routines.
- `initialize_data_structures`: Clears a block of memory by writing zeros to a table and then performs a series of sequential writes to various global variables.
- `set_all_ones_sentinels`: Fills four global bytes and six fixed longwords with all-ones values, typically a sentinel for empty entries.
- `clear_global_and_wait`: Sets a global variable to a specific value and then waits for a certain bit in a global variable to clear.
- `round_position_up_sixteen`: Calls two routines, then sets a group of globals depending on whether a global is zero or equal to another.
- `clear_twenty_one_globals`: Sets twenty-one global variables to zero in sequence.
- `set_three_conditional_triplets`: Zeroes four globals, then in three checks, when a global is nonzero, sets it to 255 and loads two fixed small values.
- `update_state_with_complex_branching`: updates a global variable or state byte through a series of conditional checks, pointer-based table scans, and recursive calls to other routines.
- `clamp_fixed_point_velocity`: Nudges a fixed-point velocity global by a small increment, subtracts a constant step under a sign test, and clamps the result to a floor value.
- `update_movement_state_offsets`: Adjusts a movement state byte, then updates a position offset by a constant that depends on the state.
- `update_position_with_clamping`: Updates a global variable by a certain amount and checks if the result or its high bits fall within a certain range, potentially setting or resetting the value.
- `update_position_with_limit_check`: Adds a constant to a global variable and checks if certain conditions or bounds are met, otherwise setting or resetting the value to a fixed amount.
- `update_state_with_speed_limit`: Clears a global when a state byte is zero.
- `advance_position_with_clamp_fifteen`: Adds a fixed offset of 40960 to a value, takes its high word and clamps it to 15.
- `call_mode_routines_then_flag`: Calls routines depending on whether certain bits of a global match a pattern, calls two more routines, and sets a global to a specific value when a flag check passes.
- `step_toward_target_within_eight`: Returns the low bits of a value masked with five.
- `update_mode_state_with_clamp`: Updates a global state variable by checking certain bits, clamping or masking a value to a specific range, and resetting or setting the state to a fixed constant based on the previous value's range or bitmask.
- `update_offset_clamped_three`: Adds a fixed offset to a value, clamps its high word to a specific range, doubles it, and clears a global if a flag is set.
- `update_offset_clamped_two`: Adds a fixed offset to a value, clamps its high word to a specific range, doubles it, and clears a global if a flag isset.
- `call_six_routines_clear_flag`: Calls six routines in sequence with small global assignments between them, and clears a global when a value is nonnegative and a flag is set.
- `apply_approximated_subtraction`: Performs a conditional subtraction or addition to a global variable using an approximation or bit-shifting logic.
- `call_then_store_register_words`: Calls one routine and then writes two input register words into the same global variable in turn, ending with the second.
- `store_two_register_words_variant`: Matches the previous entry: it calls one routine and then writes two input register words into the same global variable in turn.
- `subtract_and_correct_sign`: Subtracts a register value from a global.
- `add_with_correction_two`: Adds a register value to a global.
- `subtract_clamp_state_four`: Subtracts a register value from a global.
- `add_clamp_state_eight`: Adds a register value to a global.
- `clamp_and_nudge_global`: Checks if a global variable is within a certain range and then adjusts it by a clamped amount or resets it to zero.
- `update_clamped_global_magnitude`: Checks if a global variable is zero or within a certain range, then applies a secondary additive or subtractive step to that global variable using a clamped or bit-dependent offset.
- `damp_value_fifteen_step`: Moves a global toward zero by up to fifteen units, choosing the direction or step size based on a bit test on the result or the sign of the global.
- `add_clamped_input_to_global`: Clamps a 16-bit value to a range between a certain negative value and fifteen, then adds the result to a global variable.
- `check_threshold_return_pair`: Checks if a global or a returned value exceeds a certain threshold and returns a 32-bit value composed of a local or or otherwise modified value and the global.
- `move_by_capped_steps`: If a state global equals one, it moves a position global in steps of 8192 toward or away from zero, with the number of steps capped at a certain value.
- `check_register_value`: Checks if a global variable or a temporary subtraction results in a specific range and returns.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 1 into d6, then returns.
- `scan_table_for_matching_entry`: Scans a table for a matching key, updates a counter at a fixed address, and may call other routines or modify a global if certain conditions are met.
- `increment_indexed_counter_entry`: Clears a global, sets it to a fixed value, calls a routine, and increments a byte in a fixed table indexed by that global.
- `update_global_with_limit_checks`: updates a global variable by performing conditional bounds checks and applying offsets or bitwise masks based on its current value.
- `adjust_global_by_limit`: Compares a value against a global variable and updates the global variable based on whether it exceeds certain thresholds.
- `advance_table_chain_twice`: Twice, while a global is nonzero, adds an entry from a byte table indexed by the global to the global and then decrements it.
- `initialize_and_dispatch_table_entries`: Sets a global variable to a fixed value and, if zero, scans a table of 20-byte entries for a negative first word, calling a routine for each entry and returning a value.
- `initialize_global_state_variable`: checks if a global variable is zero and if so, sets it to a specific constant value.
- `reset_global_state_variable`: checks if a global variable is zero and, if so, sets it to a specific constant value or resets it to a new constant value.
- `initialize_global_state`: Checks a global variable and sets it to a specific constant or certain value based on its existing state or if it is zero.
- `set_state_seven_defaults`: If a certain global variable is not zero, it sets it to a certain value.
- `reset_global_to_zero_2`: Checks if a global variable is zero and assigns a new constant value to it, then sets that same global variable to zero.
- `reset_global_to_zero`: Sets a global variable to zero.
- `update_state_variable`: Modifies a global variable based on its current value, performing conditional increments or resetting it to a specific constant.
- `traverse_relative_dispatch_table`: This routine uses a global index to traverse a table of relative offsets, recursively calling itself or another routine while updating a global variable.
- `resolve_relative_table_entry`: Decrements a global index, then reads a 16-bit relative entry from a table.
- `lookup_tile_flags_at_position`: Indexes a fixed table with high bits of a position to find a tile word, potentially checking flags and reading a shift-and-mask value from a second table or checking a neighboring word.
- `lookup_tile_word_with_links`: Finds a tile word from a position using a fixed index table, then follows one or two linked neighbouring words while their top flag bits areset, and stores the result.
- `update_global_half_step`: Modifies a certain global variable by adding or subtracting a value derived from a half-shifted register, depending on whether certain conditions or range checks on the difference between the register and a global variable are met.
- `update_global_with_half_step`: Checks a register against a global variable and applies a subtraction or an addition using a half-shifted register value.
- `add_to_global_var`: Adds a value from a register to a global variable.
- `subtract_register_from_global`: Subtracts a value from a global variable based on a conditional check of a register and a shifted register value.
- `subtract_from_global_var`: Subtracts a value from a register from a global variable.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d6, then returns.
- `return_empty_routine`: The routine performs no operations and immediately returns.
- `stub_partial_decompilation_b`: The routine body is missing from thethe evidence, so its behaviour cannot be confirmed.
- `empty_return_routine`: the routine performs no operations before returning.
- `return_inverted_masked_input`: Returns the bitwise inverse of an input register, masked to bits two and three.
- `reflect_global_when_flagged`: Calls another routine and, if certain bits are set in a register and the called routine returns a non-zero value, sets a global variable to negative three minus its current value.
- `reflect_global_from_three`: Stores a register value into a global variable and, if certain bits are set in that register and another routine returns a non-zero value, sets the global variable to three minus its value.
- `validate_and_reset_global_state`: Checks a global variable against specific bitmasks and ranges, resetting it to a constant value or a zeroed state if certain conditions or invalid bounds are met.
- `check_pattern_and_set_state`: Checks an input word against a mask pattern, with the pattern depending on whether a global is zero.
- `process_active_entries_in_table`: Iterates through a table of entries, increments a global variable if a byte at an offset within an entry falls outside a certain range, and executes a series of routine calls or a jump table based on certain conditions or values in the table.
- `scale_byte_field_by`: Returns the byte at offset one of a structure, multiplied by 64.
- `clear_global_variable`: sets a global variable to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_clipping_and_position_2`: Updates sprite coordinate records by calculating horizontal mirroring, screen-edge clipping, and adjusting sprite width or position or pointer or any other necessary field based on a sprite attribute or a global variable.
- `update_sprite_clipping_and_position`: Calculates horizontal mirroring, screen-edge clipping, and updated sprite record coordinates or width adjustments, then advances the list pointer.
- `find_and_fill_table_slot`: Searches a table for a slot with a valid pointer and range, then initializes the record with a fixed value and copies data.
- `update_record_offsets`: Modifies certain fields in a record or stores a masked value into a record field based on a check of a bit and a comparison.
- `initialize_table_slot`: Searches a table for a valid entry and initializes a new record or slot with multiple cleared or fixed values.
- `allocate_slot_with_pointer`: Searches a table for a free slot and, if found, initializes the record with a 32-bit pointer and several cleared or fixed fields.
- `offset_record_position_variant`: Adjusts record position offsets based on a size field and, if a mode field is not one, calls another routine and stores a masked input word.
- `validate_and_rebuild_sprite_buffer_2`: Iterates through a table of object pointers to check if their memory ranges or size parameters are valid, updating a shared buffer and adjusting record state or visibility flags if certain conditions or pointer bounds checks fail.
- `set_record_animation_frame`: Checks a flag bit in a record, then either adjusts a vertical position, sets a frame code, or updates state bytes based on a global sign.
- `set_record_frame_code`: Sets a frame code or a fixed word into a record based on certain flag bits, and updates any non-zero following fields with that value.
- `set_size_globals_from_field`: Sets two global variables to a fixed value, and if a specific byte field is non-zero, sets a global to a fixed value and a bit-shifted version of a field.
- `update_global_value_on_range_condition`: Checks specific bitflags and value ranges within a pointer-referenced structure to conditionally update a global variable and set a specific byte in the structure to a default constant if it is currently zero.
- `check_record_range_and_store`: Checks if a record or a global value falls within a specific range, and if so, clears a global, potentially sets a field to ten, and updates a global with a pointer and a value from the record.
- `update_sprite_render_bounds`: Iterates through a list of object records to verify if their pointer and range or size parameters fall within valid memory or s.
- `recalculate_sprite_render_bounds`: Iterates through a list of sprite or object records to verify if their pointer and range or size parameters fall within valid memory or or screen-space boundaries, updating a global pointer or shared buffer with adjusted or invalidated values.
- `unpack_sprite_attribute_data`: Extracts and redistributes sprite attribute or offset values from a source object into a destination buffer or table.
- `validate_and_rebuild_sprite_buffer`: Checks if a record's pointer and range are valid, otherwise invalidates the record or writes its sprite component values into a shared buffer.
- `dispatch_record_code_events`: Reads a code byte from a record and updates a set of counters or states according to that code, with a separate branch for each of several specific codes.
- `store_byte_after_call`: Calls another routine and stores the resulting byte from a register into a record field at a fixed offset.
- `allocate_record_and_copy_parts`: Searches a table for an unused record, initializes it, and copies a sequence of values from a source buffer into the record's fields, stopping if a terminator or a zero-value or a specific constant is encountered, and records the number of parts copied in a state byte.
- `return_from_subroutine`: Performs a single return instruction to exit the current execution flow.
- `flush_pending_entries_to_targets`: Walks a table of records and, if an entry is non-zero, copies its value to a target field, clears the entry, and decrements a global count.
- `step_countdown_position_motion`: Decrements a countdown byte in a record, updates a position word using a value from a table, and sets a sentinel value if the countdown reaches a certain threshold.
- `process_pointer_table_entry`: Checks flags in a pointer-based structure and, if certain bit-masks or bounds-checks fail, executes a secondary routine via a jump table.
- `process_sprite_pointer_table`: Validates a pointer-based record and executes a jump or secondary routine if certain flags or bounds-checks fail.
- `modify_byte_with_mask`: Clears the highest bit of a byte at a certain offset from a pointer and then toggles the lowest bit.
- `update_global_structure`: Calculates a new pointer value and certain offsets, then writes these or other calculated values to a global structure or table.
- `append_clamped_object_record`: Calculates clamped horizontal and vertical offsets or sprite-style flags based on a pointer's position relative to a frame boundary and writes a new object record or state word to a global cursor.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `recurse_on_zero`: If a certain global variable is zero, it calls itself recursively.
- `service_pending_interrupt_bits`: Repeatedly tests or checks bits of a global pending mask in order, calling another routine if a matching bit is set in a custom chip register.
- `update_timed_position_or_expire`: Decrements a counter in an object and, if a direction flag is negative, calculates and writes an absolute position and a modified offset into a list entry.
- `latch_object_scroll_values_2`: Writes a value from an object into a buffer, computes a clamped difference between two object fields, and if a trigger field is non-zero, updates a pointer-referenced value.
- `latch_object_scroll_values`: Writes a value from an object into a buffer, computes a clamped difference between two object fields, and if a trigger field is non-zero, resets said trigger and stores a new state or parameter.
- `decrement_stage_counter_conditional`: Checks if a global variable is non-zero and, if so, decrements a byte-sized global variable and checks if it has reached zero to trigger a further routine call.
- `decrement_stage_counter`: Checks if a pointer-referenced value is non-zero, and if so, decrements it and calls another routine if the value reaches zero.
- `update_stage_parameters`: Updates a record-based pointer or counter using values from a data table, or resets or clamps a value in a record based on comparisons against table-provided limits or offsets.
- `advance_three_record_command_streams`: Iterates over three fixed-size records, each with an active command pointer, reading the next byte-coded command from its stream.
- `process_command_stream_bytes`: Iterates through a stream of byte-coded commands, updating or resetting certain state-tracking counters and pointers based on the command opcode or a termination signal.
- `process_command_stream_byte`: Parses a byte-coded command stream, updating command pointers and certain state counters or flags based on the instruction type.
- `process_command_stream_bytecode`: Parses a stream of byte-coded commands, updating a global state or pointer and recursively calling itself to handle nested or subsequent command instructions.
- `process_command_streams`: Iterates through command streams to decode and execute byte-coded instructions, updating pointers and counters or recursively calling itself to handle nested or conditional command logic.
- `process_command_stream_bytecode_2`: Parses a stream of byte-coded commands, updating command pointers and managing internal state or counters.
- `parse_command_stream_and_update_state`: Iterates through a command stream to calculate a total length, then executes a switch-based dispatcher that updates pointers and state-tracking counters or recursively calls itself to process further commands or conditional branches.
- `process_command_stream_bytecode_3`: Parses a stream of byte-coded commands to update or reset internal state or pointers.
- `process_command_stream_bytecode_4`: Iterates through a stream of byte-coded commands, updating state-related or pointer-based records and recursively calling another routine to handle nested or subsequent command instructions.
- `process_command_stream_and_update_state`: Decodes a stream of byte-coded commands and updates a set of active command pointers or state counters.
- `process_command_stream_bytecode_5`: Iterates through a stream of byte-coded commands, updating a global state or table and recursively calling itself to handle certain command types.
- `copy_word_field_triples`: Copies two word fields to two other offsets in a structure, advancing the base pointer two bytes per pass, and repeats for three passes.
- `copy_word_field_pairs`: Copies two word-sized fields from one offset to another while incrementing a base pointer for two iterations.
- `empty_routine`: The routine is empty and contains no executable instructions.
