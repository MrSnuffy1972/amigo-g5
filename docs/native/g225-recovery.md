# Game 225 (ActionFighter v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00225). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (414831 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 71 raw-word lines |
| Functions named and described | pass, 167 of 167 named (0 author symbols), 167 of 167 described (10 by rule, 17 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1989 of 2003 (99.30%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4878 of 5928 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 370731 |
| code | 24328 |
| data | 19673 |
| padding | 99 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| ActionFighter.slave | 1 | 53 | 6 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5874 | 160 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- ActionFighter_v1.0/ActionFighter/Disk.1: raw disk image (360000 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 2 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 1942 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ActionFighter_v1.0/ActionFighter/Disk.1: 2 logged loads cover 360000 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### ActionFighter.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decode_block_into_low_memory`: Zeroes low memory, then copies 3000 longwords from a source block into low memory, XORing each with a constant.
- `set_low_bit_and_return`: Sets bit zero of the low byte of D0 and returns that value.
- `write_branch_instruction_word`: Writes a branch-always instruction word to a fixed low-memory location and returns the low byte of D0 unchanged.
- `call_stored_routine_passthrough`: Calls a routine whose address is read from a fixed offset in the work area, then returns the two input registers unchanged.
- `tiny_ori_nop`: Small routine of 2 instructions that merges 0 into d0, then does nothing for one instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `reset_entries_when_negative`: When a status value is negative, it clears the first word of thirty-one table entries that are 98 bytes apart.
- `set_state_fields_from_lookup`: When a mode value is not two, it resets a state byte and writes two fixed words, then looks up a value through two indexed tables and stores it if it is positive.
- `advance_state_timer_fields`: Checks a state flag and a counter, and while the counter is still running it decrements it and returns.
- `advance_state_machine_step`: Runs one step of a small state machine: it moves from state zero to one and then to two when a flag and a level check pass, and resets to state zero otherwise.
- `check_slot_table_and_activate`: When the status flags are clear it counts down a timer and reloads it from a table.
- `set_bits_and_update_entry`: Sets a fixed group of bits in a word at an offset from the input pointer.
- `activate_entry_with_value`: Marks the entry at the A0 pointer as active by setting one flag byte, stores the D0 word at a fixed offset, and sets a second flag byte to one.
- `enable_entry_clear_secondary`: Sets a flag byte in the entry at the A0 pointer to one, stores the D0 word at a fixed offset, and clears a second flag byte to zero.
- `check_entry_proximity_match`: When the first entry is in state two, it compares its horizontal position with a second entry's, shifted by a bit-dependent offset.
- `init_state_lookup_from_table`: When the state value is above three it resets several fields and searches a byte table for the current code.
- `start_timed_state_transition`: When a counter is zero, it checks a state value and an elapsed count against thresholds and then sets a status code.
- `advance_position_from_table`: Returns the D0 word with bit zero set.
- `set_mode_six_return_byte`: Sets a mode byte to six and returns the low byte of D0 with every bit set except bit zero.
- `copy_table_entry_when_free`: When a flag byte is clear, it reads a record from a table chosen by a counter.
- `step_timer_and_mark_entry`: While a timer is running it calls a helper, clears entry fields, marks a table slot as used, and increments a step counter that stops at 20.
- `clear_linked_entry_fields`: Walks ten tables of records, each ending at a sentinel word, and zeroes the second word of every record.
- `fill_free_slot_entry`: After a short warm-up count, and when no other entries or flags are active, it checks a counter every eighth step.
- `expire_out_of_range_entries`: Walks a short list of entries and clears those whose flag byte is negative and that meet a position range check.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to its caller.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to the caller.
- `write_new_entry_when_close`: When a flag byte and two other flags are clear, it compares the entry's vertical position with a reference.
- `return_immediately`: This routine is empty and performs no operations before returning.
- `return_gap_stub`: Its listing is a bare return.
- `queue_entry_after_timer`: When no blocking flags are set it counts a timer down and reloads it from a table.
- `advance_entry_after_countdown`: While an entry is active and not in a blocking state, it counts a timer down.
- `step_script_table_state`: Switches on a mode value: mode one clears an entry flag when a range check fails, and mode two clears an activity flag when a position passes a limit.
- `copy_table_entries`: iterates through a table to find matching byte values and copies data from one memory location to another while updating a counter.
- `sort_entries_by_key_field`: Clears a 1568-byte work block, then copies active entries from the table into it in order of a key byte, one slot at a time, and links them with sorted indices.
- `copy_work_entries_back`: Copies the work block of entry records back into the entry table, as the mirror of the copy done before sorting.
- `call_then_return_stub`: Calls one routine and returns.
- `spawn_entries_by_type`: In state two, when a table entry's position is at or below a computed limit, it advances the table cursor.
- `fill_free_slots_with_records`: Clears a counter and reads a value from a bit-selected table.
- `run_callbacks_for_active_entries`: In state two it either advances a counter or clears a flag bit on free entries.
- `run_callbacks_second_table`: In state two it calls a stored routine for each active entry of a second entry table, which starts at a different address from the first.
- `call_three_helpers_undecoded`: The decompiler timed out, so only the listing facts are known: the routine has 39 instructions and calls three other routines.
- `initialise_record_block_from_table`: Sets flag and value fields in a block of nine records, writing a constant into a shared field and copying words from a source table into each record's position fields.
- `initialise_offset_record_block`: Fills nine records with fixed field values and source-table words, the same as its neighbour, but starting at a different offset in the record area.
- `fill_records_from_pair_list`: Walks a list of pointer pairs, writing position fields and fixed field values into consecutive records.
- `set_high_bits_return_byte`: Returns the low byte of D0 with every bit set except bit zero.
- `run_entry_callbacks_then_routine`: When a flag byte is set, it calls a stored routine for each active entry in the entry table.
- `run_eight_helpers_then_clear`: Calls eight other routines in sequence and then zeroes a state byte.
- `step_position_counter_loop`: Clears two state values, then loops over a step counter, calling helper routines and reading a value tied to the position counter.
- `set_flag_run_six_helpers`: Sets a flag byte to one, then calls six other routines in sequence and returns.
- `write_table_records_to_block`: Sets a flag byte to one, then copies three words from each record of a selected table into a fixed block.
- `run_helper_set_by_mode`: Sets a flag byte to one and calls a set of helper routines.
- `run_six_helper_calls`: Calls six other routines in sequence and returns.
- `copy_fixed_block_forward`: Copies a fixed block of 320 records of ten longwords each from one memory area to another, moving forward through the block.
- `clear_fixed_block_records`: Zeroes the same 320-record block that its neighbour copies, writing ten longwords per record.
- `run_four_helper_calls`: Calls four other routines in sequence and returns.
- `run_seven_helper_calls`: Calls seven other routines in sequence and returns.
- `initialise_entry_parameters`: Writes a fixed set of starting values into a parameter block: a mode value of two, several position words, and a table index.
- `copy_tile_list_to_buffer`: Reads a list of tile indices until a terminating value.
- `run_three_count_helper_loops`: Calls one helper fifteen times in a loop, with a five-call loop before and after it.
- `run_five_helper_loops`: Calls one helper in five successive loops of five, four, two, four and ten calls.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `format_decimal_digits`: Writes the decimal digits of the D0 value into a text buffer from the least significant digit upward, after placing a terminator byte at the end.
- `count_down_skipping_odds`: Steps a 16-bit counter down through its values, moving past odd values, until it wraps past zero.
- `count_down_skipping_evens`: Decrements a 16-bit counter repeatedly, moving past even values, until it wraps past zero.
- `fill_buffer_with_dashes`: Fills a twelve-byte buffer with the character code for a dash, and toggles the high bit of a status byte around a helper call.
- `read_table_byte_by_index`: Returns the byte from a table at an index formed by doubling the low byte of D0, read through the work-area base.
- `call_helper_then_loop_eight`: Calls one helper, then calls another helper eight times in a loop.
- `call_helper_after_eight_loop`: Calls one helper, then calls another helper eight times in a loop.
- `adjust_index_from_flag_bits`: Waits until a counter reaches six, then adjusts an index from four flag bits of a status byte and masks it to five bits.
- `advance_counter_with_wrap`: Adds one to a byte counter that wraps at 256 and calls a helper, then increments a second counter.
- `copy_table_block_into_record`: Advances an offset, reads a byte index from a table, and copies a block of longwords from a lookup table at that index into the target record.
- `copy_backward_longword_blocks`: Copies longwords from a fixed source area backward into the target, sixteen per iteration over five iterations.
- `write_blocks_after_status_waits`: Waits on a status bit before each group of word writes to a fixed set of locations.
- `write_rows_after_status_wait`: Uses the same wait-then-write loop as its neighbour, waiting on a status bit before each group of writes to fixed locations.
- `program_blitter_row_copy`: Waits on a status bit and programs the blitter modulo, pointer and size registers in a loop over rows, using the same wait-and-write pattern as its neighbours.
- `clear_strided_buffer_rows`: Zeroes four groups of ten longwords per row across a buffer, with the row count taken from a status byte.
- `disable_dma_and_interrupts`: Writes a value with the top bit clear and all other bits set to two fixed DMA and interrupt control registers.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `set_dma_and_interrupt_control`: Writes three fixed words to the DMA control, interrupt request and interrupt enable registers.
- `write_one_hardware_value`: Writes one value to a fixed hardware address and returns.
- `advance_counter_run_helpers`: Increments a counter byte, calls five helper routines and one further routine in sequence, then writes a fixed value to a status location.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d7.
- `empty_routine`: The routine contains no valid instructions and performs no operations.
- `tiny_ori_2`: Small routine of 1 instruction that merges 192 into d0.
- `clear_fixed_byte_block`: Ors a fixed bit pattern into the word at the input pointer, then zeroes a block of 4314 bytes starting at a fixed address.
- `clear_fixed_byte_block_2`: Zeroes 4314 bytes at its own fixed start address, one byte at a time.
- `clear_memory_block`: fills a specific range of memory with zero values using a loop.
- `clear_memory_block_2`: sets a block of memory to zero through a repetitive loop.
- `clear_fixed_byte_block_3`: Zeroes 4314 bytes at its own fixed start address, one byte at a time.
- `clear_memory_block_3`: performs a large scale zero-fill operation on a memory segment.
- `clear_fixed_byte_block_4`: Zeroes 4314 bytes at its own fixed start address, one byte at a time.
- `clear_memory_block_4`: clears a memory area by writing zero to each word in a loop.
- `clear_fixed_byte_block_5`: Zeroes 4314 bytes at its own fixed start address, one byte at a time.
- `wait_raster_then_set_bits`: Waits until the raster position register reads a fixed value, then clears bit zero of one byte and sets the top bit of a control byte.
- `spin_forever_loop`: Loops forever without any work, so once entered it never returns.
- `wait_raster_then_toggle_bits`: Waits until a counter reaches a reference value, then waits for the raster position register to read a fixed value.
- `toggle_control_top_bits`: Toggles bit zero of one byte and the top bit of a control byte, with the top-bit toggle repeated four times.
- `halt_when_marker_set`: When a status value equals the character c, it enters an endless loop.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `set_display_control_registers`: Writes a fixed sequence of words to the display registers: the bitplane modulos, the control registers, the data fetch start and stop, and the display window start and stop.
- `set_blitter_masks_and_control`: Writes fixed words to the blitter first-word and last-word mask registers and to its control registers.
- `repeat_three_helper_calls`: Repeats a group of three helper calls forty-four times, then returns.
- `compute_entry_draw_offset`: When the status word equals one and the entry's position is inside the allowed range, it works out a horizontal clip and shift.
- `blit_masked_rows_status_two`: For status two it computes clip and shift values like its neighbours, then combines shifted mask and source words into destination rows with bitwise AND and OR.
- `blit_masked_rows_alternate`: Computes clip and shift values for an entry record and combines shifted mask and source words into the destination rows, the same blit pattern as its neighbours, read from different field offsets.
- `blit_masked_rows_status_three`: For status three it computes clip and shift values and combines shifted 32-bit mask and source words into destination rows, inverting the mask.
- `call_helper_forty_four_times`: Calls one helper forty-four times in a loop and returns.
- `clear_entry_plane_columns`: When the status is one, it picks a plane base address and a column count from the entry.
- `clear_entry_plane_columns_alt`: Zeroes the same four-plane column pattern as its neighbour, choosing the plane base and count from the entry by a parity bit, with no status check.
- `clear_high_plane_columns`: Zeroes four-plane column groups from a fixed high plane base, with the count taken from one entry byte.
- `combine_plane_rows_inverted`: Loops over 480 rows of sixteen words, ORs each group of four source words, inverts the result, and stores it in the destination words.
- `combine_plane_rows_or`: Ors each group of four source words into a destination word without inverting, storing 32 words per pass.
- `copy_tile_bytes_to_planes`: Reads a list of tile indices until a terminating value of all ones.
- `composite_tile_words_into_planes`: Reads a list of tile indices until a terminating value.
- `return_stack_value_passthrough`: Returns the value held on the stack without changing registers.
- `copy_tile_words_to_planes`: Reads a list of tile indices until a terminating value.
- `clear_sixteen_word_rows`: Zeroes 512 consecutive groups of sixteen words.
- `copy_sixteen_word_rows`: Copies 512 consecutive groups of sixteen words from the source to the destination.
- `split_source_into_four_planes`: Copies 800 groups of twenty source words into four destination planes, distributing five words to each plane per group at fixed spacing.
- `copy_and_run_patched_routine`: Modifies a couple of bytes near base registers, copies sixteen words from the source into a fixed buffer, and calls that buffer.
- `store_word_indexed_then_call`: Stores the D0 word in a table at an offset set by D1 doubled, then calls another routine.
- `build_three_field_counts`: Loops over seven rows of sixteen words.
- `reduce_three_field_values`: Loops over seven rows of sixteen words and subtracts one from each non-zero three-bit field in every word.
- `clear_sixteen_words_call_helper`: Zeroes sixteen words starting at a fixed pointer, then calls one helper.
- `load_palette_from_words`: Writes sixteen words, each shifted left by one bit, into the colour registers starting at the first one, then returns.
- `enable_cia_interrupt_mask`: Writes a fixed value to the first CIA interrupt control register, which enables the interrupt sources its bits name, then calls two other routines.
- `reset_state_on_code`: Checks a state byte against two codes.
- `pulse_status_byte`: Writes two to a status byte and then zeroes it, leaving the byte cleared.
- `map_status_to_code_value`: Turns a status byte into a small code value.
- `nop_operation`: This routine is empty and performs no operations.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
- `advance_table_every_ten`: When a flag is set it counts a byte up to ten.
- `rotate_status_write_serial`: If bit three of a status byte is set, it rotates the byte, flips its sign bit and writes a fixed sequence of values to the CIA keyboard serial and timer registers.
- `process_joystick_bits`: Performs bitwise manipulations on a global variable based on its current state and bit patterns.
- `store_word_parts_status`: Writes parts of a word to two status locations, then writes a fixed value to a third location.
- `store_word_parts_status_2`: Writes parts of a word to two status locations, then writes a fixed value to a third location.
- `read_mouse_button_deltas`: Computes the negated change between the previous and current mouse movement values and stores both.
- `silence_audio_channels`: Clears the four audio channel volume registers and writes a fixed value to the DMA control register, which silences the channels.
- `update_channel_bit_by_mode`: For a record whose state is one, it sets, clears or rotates a bit in a status byte according to a mode field, then clears a counter.
- `run_conditional_helper_calls`: Calls a helper, then conditionally calls one or two more helpers depending on a flag and a state byte, and returns.
- `write_single_status_value`: Writes one value to a fixed status location and returns.
- `write_status_value_then_return`: Writes one value to a fixed status location and returns.
- `poll_channel_interrupt_flags`: Loops over ten channel records, testing one bit of the interrupt request word for each.
- `initialise_channel_records`: Sets starting values in the channel records and points three fields at table entries chosen by a byte.
- `step_event_stream_sequencer`: When a playing flag is set it counts down a per-step timer and reads command bytes from an event stream.
- `set_byte_to_one`: Writes one to the byte at the input pointer and returns.
- `step_channel_event_stream`: Runs one channel's event-stream step.
- `decode_stream_command_byte`: Decodes one command byte from an event stream.
- `advance_channel_note_timer`: Advances a channel's note timer.
- `initialise_channel_from_source`: Stores a decimal-adjusted value and copies several fields from a source record into the channel record.
- `initialise_channel_record_fields`: Copies several fields from a source record into the channel record, sets two flag bits, and stores a handler pointer, then returns its input register.
- `write_audio_channel_block`: When a status bit is set it writes four channel fields into a block (an address, a length, a period and a volume), then clears a flag bit and sets a mode value when a mask matches.
- `update_audio_channel_hardware`: When bit fourteen of the channel flags is set, it copies the volume, period, address and length fields into the channel's hardware registers and decrements a length counter.
- `step_and_clamp_counter`: Adds a signed step to a byte counter.
- `step_counter_within_bounds`: Adds a step to a byte counter.
- `update_function_pointer`: Checks if a value at a specific offset is zero and, if so, stores a routine address into a pointer location.
- `step_and_reset_channel_block`: Adds a step to a counter.
- `clear_fixed_work_block`: Zeroes a fixed work block of 1658 bytes starting at its fixed address, one byte at a time.
- `clear_bit_and_xor_block`: Clears a specific bit in a byte and then performs a block XOR operation on a range of memory using a key.
- `clear_bit_and_xor_block_2`: Clears a specific bit within a byte at a calculated memory location and then performs a bitwise XOR operation on a block of memory using a global key.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_bit_and_xor_block_3`: Clears a specific bit within a byte at a calculated memory location and then performs a bitwise XOR operation on a block of memory using a key from a global variable.
- `clear_bit_and_xor_buffer`: Clears a specific bit within a byte at a calculated memory location and then performs a block XOR operation on a memory range using a key from a global variable.
- `copy_and_xor_block_words`: Copies a run of words from one buffer to another, XOR-ing each word with a key read from a global variable.
- `xor_memory_block`: Iterates through a memory range and applies a bitwise XOR operation to each word using a global value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_without_effect`: Contains only a return, so it changes no memory or registers and does nothing when called.
