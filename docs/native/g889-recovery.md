# Game 889 (InternationalKarate+ v1.5 2063): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00889). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (402675 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 19 raw-word lines |
| Functions named and described | pass, 115 of 115 named (0 author symbols), 115 of 115 described (6 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1075 of 1075 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 362496 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2614 of 2938 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 24381 |
| code | 11514 |
| data | 366754 |
| padding | 26 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| InternationalKarate+.Slave | 1 | 80 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2045 | 83 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 186 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 206 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |
| IK+.Image (raw code file, load address unknown) | 1 | 372 | 1 |

## Cases handled

- runtime segment an address: 722 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 116 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 44 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 158 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- InternationalKarate+_v1.5_2063/InternationalKarate+/IK+.Image: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 362496 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### InternationalKarate+.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `copy_block_on_special_code`: Calls two handlers from a table.
- `call_handler_and_set_flag`: Calls a handler obtained from a table and, when it returns non-zero, calls a second handler and another routine, then sets a RAM flag byte to one.
- `invoke_callbacks_and_return`: Calls one routine, calls a function pointer read from a shared variable, calls another routine, and returns zero.
- `swap_buffer_with_fixed_area`: Calls five helpers, then exchanges 51 bytes between a buffer addressed by a register and a fixed memory area.
- `swap_two_byte_buffers`: Exchanges 51 bytes between two buffers addressed by two pointer registers.
- `check_input_and_set_ram`: Compares an input byte with a shared value.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_struct_table_entries`: The routine copies selected words from a global structure table to a fixed memory table.
- `clear_state_and_loop`: The routine iterates through a range of offsets, clearing specific memory locations by writing zero bytes.
- `decode_and_clear_table`: The routine builds a byte value from a bitstream stored in memory and then uses it to index a table, clearing a set of bytes in a global structure and writing a byte from the table into the structure.
- `update_state_and_match_char`: Returns early when the state is five or seven and a flag is clear.
- `check_fire_button_and_latch`: When a counter exceeds 24 and two latch bytes are zero, it reads the fire button bit from the CIA A port, updates the latch bytes to match and resets the shared state variables.
- `store_long_from_pointer`: The routine reads four consecutive bytes from the memory pointed to by a1 and combines them into a 32-bit value, then writes that value to a global variable.
- `run_three_state_helpers`: Calls three helpers in sequence: one sets a shared value and calls a routine four times, one updates a timer state from a lookup table, and one writes a constant and reloads it from a small table.
- `set_global_and_call_routine`: The routine stores a constant value in a global variable and then invokes another routine four times in succession before returning.
- `update_timer_state`: Checks a global variable against a specific value and updates it using a lookup table or sets it to a default.
- `set_global_and_recurse`: The routine writes the constant 6 to a global variable, then conditionally overwrites that variable with a byte from a small table indexed by bits of the variable, and finally calls another routine twice before returning.
- `draw_four_bob_objects`: Calls the masked object blitter four times in a row with prepared registers, drawing four objects side by side.
- `draw_four_bobs`: Draws four objects in a row by calling the masked bob blitter routine four times with the prepared registers.
- `step_flash_counters_per_slot`: Loops over two slots.
- `draw_flash_pairs_for_slots`: Walks the slots from high to low, decrementing the flash counter of each active slot.
- `draw_flash_pair_sequence`: Calls two helpers in sequence, one of which draws the second flashing image pair.
- `draw_flash_pair_b`: Draws the same two-part object with the second image pair, the other half of the flashing effect.
- `update_global_from_table`: Masks a shared value to three bits and uses it as an index into a byte table to set that value.
- `set_state_and_call_once`: Sets a shared value to three and then to one, calls a library routine and a helper, and increments the shared value once.
- `store_state_and_call_once`: Stores the input byte in a shared variable, calls a library routine and a helper, and increments the variable.
- `clear_memory_and_initialize_globals`: The routine clears a block of memory by writing zero bytes to consecutive locations, then writes constant values to several global variables and to a set of four global pointers.
- `return_obfuscation_checksum_byte`: Calls a checksum helper and returns its byte result.
- `compute_obfuscation_checksum`: The routine increments a global variable by several constants, extracts its high byte, and returns a value derived from the variable combined with bytes read from another routine using the variable as an index.
- `write_sprite_mask_tile`: The routine writes masked bytes from a global variable table into two parallel framebuffer areas for a 7-row tile.
- `update_global_and_call_routine`: The routine reads two bytes from a table indexed by a global variable, updates a counter byte in that table and clamps it, then calls another routine six times while toggling the global variable between the two table bytes based on the counter.
- `update_bcd_counters_and_recurse`: The routine checks a global variable and, if it is zero, reads a pointer from it, then updates two BCD-encoded bytes through indirect pointers derived from that pointer and from a byte at an offset from it, applying BCD adjustment to each.
- `save_and_restore_global_variable`: The routine saves the current value of a global variable, clears it, calls another routine, then restores the original value from the saved copies.
- `decode_global_byte_table`: The routine checks a global variable and, if it is non-zero, it clears it and calls another routine.
- `draw_masked_pair`: Draws a two-part object with two calls of the masked plotting routine.
- `draw_indicator_bob`: Draws one indicator bob at a position taken from the current player's table entry.
- `initialize_slot_state_tables`: Clears a 40-byte region and sets six entries to their index modulo four.
- `normalize_global_byte`: The routine reads a global variable and applies a small conditional transformation to it.
- `tiny_not_swap`: Small routine of 2 instructions that inverts d7, then swaps the halves of d7.
- `restore_global_variable_and_call`: The routine saves three bytes from a global variable, modifies the variable with a character constant and a conditional table lookup, clears it, computes a new value, calls another routine, restores the saved bytes in reverse order, and calls another routine again.
- `decrement_flag_loop`: The routine sets a global variable to seven and then repeatedly decrements it while calling another routine.
- `draw_panel_into_both_buffers`: Runs the drawing routine that follows into both display buffers in turn and restores the current-buffer pointer.
- `decode_script_bytes`: The routine walks a byte stream using an index stored in a global variable, reads bytes from a table pointed to by another global, and dispatches on the byte value.
- `decode_script_byte_sequence`: Processes a byte value by subtracting an offset and applying masks to a table.
- `process_global_byte_table`: Checks if a global variable is within a valid positive range.
- `return_global_variable`: The routine loads a global variable and returns its value.
- `apply_byte_mask_to_table`: The routine iterates over a block of memory using a pointer and a counter.
- `forward_sprite_attribute_copy`: Calls one helper and returns.
- `copy_sprite_attribute_blocks`: The routine copies a fixed number of 40-word attribute blocks from a source table to a destination buffer built from a global variable base.
- `draw_fighter_frame`: Draws the current fighter's animation frame into the drawing buffer: picks the frame from one of two frame tables (selected by a fighter flag), positions it from the fighter's coordinates and copies its runs of image words into the bitplanes.
- `build_draw_command_list`: Selects one of two offset tables from a flag and walks a zero-terminated list of word pairs.
- `store_register_and_call`: Stores the address in a register into a shared buffer pointer, then calls a helper and returns.
- `tiny_ori_2`: Small routine of 1 instruction that merges 32 into d6.
- `run_four_helper_calls`: Calls four helpers in sequence and returns.
- `run_four_helper_calls_2`: Calls four helpers in sequence and returns.
- `run_four_helper_calls_3`: Calls four helpers in sequence and returns.
- `execute_recursive_calls`: Performs a series of nested calls to another routine.
- `four_steps_per_frame`: Calls the following step routine four times in a row, giving four steps in one frame.
- `clear_memory_words`: The routine clears a block of memory by repeatedly zeroing ten consecutive 32-bit words per iteration.
- `call_routine_when_flag_set`: Calls one helper when a shared flag is non-zero and returns otherwise.
- `duplicate_word_table_entries`: The routine copies a 16-bit word from a source table to a destination table and writes the same value to three additional offsets within each destination entry.
- `advance_animation_state_entry`: Advances one animation record.
- `initialize_sprite_structure`: The routine writes a byte from a register into two byte fields of a structure pointed to by a register, copies a byte from a global table indexed by that register into two byte fields of the same structure, and then clears several byte fields of the structure and sets one byte field to one.
- `select_next_frame_value`: Checks an active flag for one record and returns a 16-bit target frame value when a timing byte is zero and a table comparison passes.
- `select_frame_value_from_registers`: Variant of the frame selector that takes its record and table indexes from registers rather than fixed globals.
- `reset_record_state_if_idle`: Clears a state byte and a flag byte in a record when the state is set or the target byte is zero, then calls a helper and returns its result.
- `step_record_animation_frame`: Runs one animation step for a record.
- `clamp_frame_lookup_loop`: Loops while a frame counter stays above a table limit, calling a helper on each pass.
- `advance_frame_state_machine`: Loops over a record's frame state, looking up a value clamped at 87 from a table and shifting the previous state fields.
- `return_counter_and_increment`: Returns a per-record counter, calling a helper first when it is zero.
- `adjust_offset_by_state_bits`: When a state bit is set and a flag is clear, adjusts a record offset by plus or minus three depending on a flag.
- `adjust_offset_then_scan_frames`: Adjusts a record offset by plus or minus three depending on a flag, then loops over frame values, calling helpers and comparing table entries.
- `toggle_state_flags`: Toggles a flag byte when a state bit is set, sets a small global to the record index under a condition, sets a second flag when another bit is set, calls a helper and then stores the next frame value.
- `decrement_frame_and_look_up`: Decrements a frame value in a record.
- `update_sprite_byte_field`: Looks up a byte in a shared table using a record field and a register byte as the index, stores it in a record field, then calls a helper.
- `update_record_position_with_bounds`: Adds or subtracts table values to a record's position byte depending on a direction flag, then clamps the result into a fixed range.
- `update_position_from_index`: Variant that takes its table index from a register.
- `clamp_record_position_bounds`: Adds a table value to a record's position byte, then applies bounds.
- `set_position_from_table_sum`: Sets a record's position byte from a table sum minus 40, floored at zero, and resets any value above 207 to 16.
- `clamp_position_when_high_index`: When a shared index is above 174, sets the record's position byte from a table value or a fixed value depending on a direction flag.
- `set_position_then_clamp`: Sets a record's position byte to a fixed constant and resets any value above 207 to 16.
- `choose_background_colours`: Chooses the two background colour values for the frame: fixed colours in phase 6, otherwise per-player colours from a table when that player is active, with defaults when not.
- `compare_d7_to_table`: The routine loads a short zero into a loop counter and repeatedly compares an input short from D7 with successive entries of a global short table, advancing the counter by two each iteration.
- `reset_game_state`: The routine clears four global variables to zero and then returns.
- `draw_into_both_buffers`: Runs the screen drawing routine once for each of the two display buffers by pointing the current-buffer variable at each in turn, then restores it.
- `copy_sprite_attribute_blocks_2`: The routine copies a block of 40 longwords from a source table to a destination buffer derived from a global variable, repeating the copy for 83 iterations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_clear_sprite_attribute_table`: Calls one helper that zeroes sprite attribute words, then returns.
- `clear_sprite_attribute_table`: It writes zeros to ten consecutive 32-bit words and then advances the pointer by a fixed stride, repeating the clearing eight times.
- `compute_row_offset_times_eighty`: Returns zero when a shared mode value is eight.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `music_frame_all_channels`: Music player frame: runs the channel sequencer for each of the four audio channels.
- `write_dma_control_undecoded`: The decompiler timed out, so the body is not decoded.
- `init_stream_record_from_source`: Copies a count and a source pointer into a record, moves saved fields between offsets, zeroes a field in a second structure and points the record at the shared data base.
- `set_bits_except_lowest`: Returns the input byte with every bit except bit zero set, by ORing it with a constant mask.
- `poll_fire_port_undecoded`: The decompiler timed out, so the body is not decoded.
- `run_undecoded_single_helper`: The decompiler timed out, so the body is not decoded.
- `run_undecoded_two_helpers`: The decompiler timed out, so the body is not decoded.
- `run_undecoded_single_helper_2`: The decompiler timed out, so the body is not decoded.
- `run_undecoded_single_helper_3`: The decompiler timed out, so the body is not decoded.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_title_state_arrays`: Calls one helper, which the facts identify as the title logo frame routine, then zeroes a series of shared state variables and a block of 16 longwords.
- `title_logo_frame`: One frame of the animated title sequence: updates the logo objects, the second effect on even frames only, and the remaining title elements in a fixed order.
- `initialize_title_arrays`: Loops 21 times copying a shared table byte into a byte array, writes 31 into a second byte array and clears two word arrays.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_sprite_tile_offset`: Computes a memory address by scaling an index from a register and adding it to a base pointer, while validating bounds against a global variable.
- `preshift_sprite_rows_for_list`: Writes a three-word entry into a shared list, then loops over rows of source words.
- `process_sprite_masks`: The routine iterates over a set of mask words, builds four rotated masks from four source words per block, combines them with a control word and writes the resulting mask to an optional output table.
- `check_position_range_and_dispatch`: Returns its inputs unchanged.
- `check_input_range_and_dispatch`: The routine checks a masked value from a register against a threshold and, if the condition holds, masks another register value and tests it against several ranges.
- `apply_rotated_bitmask`: Iterates through a series of data structures, rotating a combined mask value based on a shift amount.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_without_effect`: The decompiled body is a bare return with no memory access, register writes or calls, so the routine does nothing when it runs.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `return_immediately_2`: This routine is empty and performs no operations before returning.
- `do_nothing`: the routine is empty and performs no operations before returning.

### IK+.Image (raw code file, load address unknown)

- `toggle_table_bit_copy_block`: Toggles one bit in a table entry chosen by an index register, then copies 120 groups of five longwords from one address to another in a loop.
