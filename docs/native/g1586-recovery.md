# Game 1586 (pang (1990)(ocean)[cr proj inc][t +4 proj inc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01586). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (938064 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 23 raw-word lines |
| Functions named and described | pass, 98 of 98 named (0 author symbols), 98 of 98 described (4 by rule, 20 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1688 of 1688 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6015 of 6015 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12965 |
| code | 22134 |
| data | 899873 |
| padding | 20 |
| unused | 3072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pang (1990)(Ocean)[cr Proj Inc][t +4 Proj Inc].adf:bootblock | 1 | 213 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 972 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4217 | 23 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 209 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 238 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 166 | 3 |

## Cases handled

- pang (1990)(ocean)[cr proj inc][t +4 proj inc]/Pang (1990)(Ocean)[cr Proj Inc][t +4 Proj Inc].adf: custom-format (non-DOS) disk, 1752 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 207 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1200 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 127 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 148 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- pang (1990)(ocean)[cr proj inc][t +4 proj inc]/Pang (1990)(Ocean)[cr Proj Inc][t +4 Proj Inc].adf: blocks 1711-1754 (bytes 876032-898560) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.77)
- 1 packed track region(s), 874496 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `wait_flags_then_start_copper` (runtime segment an address): A 41-instruction routine that writes a fixed longword pattern into a buffer while a flag is clear, then waits on a global flag and a counter. Kind: track/sync check. Pass path: u1__l0_004E.

## Functions


### Pang (1990)(Ocean)[cr Proj Inc][t +4 Proj Inc].adf:bootblock

- `forward_boot_entry_call`: A single-instruction boot block entry, entered from the Kickstart strap, that makes one call to an unnamed routine.
- `manage_cia_ports_and_interrupts`: A 48-instruction boot block routine, called from one unnamed routine, that touches the CIA-A and CIA-B port registers, the interrupt enable register and a custom chip register at offset zero.
- `scan_sync_marked_sector_data`: Scans a buffer for a fixed 16-bit marker word, then merges masked odd and even bit fields from the words that follow into an output buffer.
- `emit_masked_words_with_counter`: Writes masked words from an input buffer to an output area for a fixed count, then increments a counter in a record field.
- `run_sync_scan_pass`: Listed as five instructions and called from one unnamed routine, with two calls to unnamed routines.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `split_value_into_quotient_remainder`: Divides an input value by 5632 and stores the quotient and remainder of division by 512 into two fields of a record.
- `loop_until_field_matches_2`: Calls another routine in a loop until a byte in a record field equals a target value, using different branch paths depending on the comparison result.
- `loop_until_field_matches`: Calls two unnamed routines in a loop until a byte in a record field equals a target value, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_flag_then_clear`: Calls an unnamed routine repeatedly while bit four of a byte at a fixed offset is set, then clears a record field.
- `wait_and_clear_byte`: Waits for a bit in a byte at a fixed offset to be set, then clears a byte in a record field.
- `clear_bit_and_increment_field`: Clears bit one of a control byte, calls an unnamed routine, then adds one to a record field.
- `set_bit_and_decrement_field`: Sets bit one of a control byte, calls an unnamed routine, then subtracts one from a record field.
- `set_low_bit_then_call`: Clears and then sets bit zero of a control byte, which leaves that bit set, and calls an unnamed routine.
- `spin_countdown_loop`: A busy loop with no outgoing calls that counts a 16-bit value down from 5120 until it reaches minus one, then returns.
- `setup_disk_dma_and_wait`: Configures several hardware or memory-mapped registers and loops while checking a bit in a byte or waiting for a certain count to reach zero.
- `enable_disk_length_register`: Writes the disk length register with a value whose enable bit is set, then writes the DMA control register with a small value and returns zero.
- `spin_until_status_bit_clears`: Busy-waits with no outgoing calls while bit five of a byte at a fixed offset is set, then returns.
- `update_interrupt_request_and_value`: Decrements a global variable and writes a specific value to a register that triggers an interrupt request.
- `write_interrupt_request_word`: Writes the value 32, which has only bit five set, to the interrupt request register.
- `wait_status_then_enable_dsklen`: Busy-waits while bit five of a byte at one fixed offset is set, writes the disk length register with its enable bit set, then waits for bit four of a byte at another fixed offset to be set.
- `run_two_routine_calls`: Six-instruction routine called from two unnamed routines that makes calls to two other unnamed routines.
- `configure_interrupts_and_dma_registers`: A 12-instruction routine, called from one unnamed routine, that touches the audio control register, the DMA control register, the interrupt enable register and the interrupt request register, and makes one call to another unnamed routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_flags_then_start_copper`: A 41-instruction routine that writes a fixed longword pattern into a buffer while a flag is clear, then waits on a global flag and a counter.
- `return_from_routine`: the routine is empty and performs no operations before returning.
- `install_handler_and_request_value`: An eight-instruction routine, called from one unnamed routine, that makes one call to another unnamed routine.
- `reset_interrupt_state_and_store`: A 17-instruction routine, called from one unnamed routine, that makes one call to another unnamed routine.
- `dispatch_state_via_table`: A large 139-instruction routine that clears the copper jump strobe register, then branches on several global state values.
- `branch_state_through_table`: A 92-instruction routine found only as gap bytes and called from one unnamed routine.
- `clear_state_and_request_interrupt`: A 48-instruction routine that touches the background color register and calls five other unnamed routines.
- `append_three_entries_when_clear`: A nine-instruction routine that, when bit two of a flag byte is clear, writes two longwords and a byte into a table entry at a global index and advances that index by eight.
- `track_decimal_digit_changes`: Extracts the tens and hundreds digits of a global counter by repeated division by ten, and compares each with a stored digit.
- `fill_control_entries_in_loops`: A 53-instruction routine, called from one unnamed routine, that derives an offset from the upper bits of an input value.
- `append_long_pair_entry`: Appends an eight-byte record to a list: two longword values taken from a global and table, and a one-byte value of 255.
- `append_alternate_pair_entry`: Appends an eight-byte record to a list in the same way as its neighbouring routine, using a different pair of stored values.
- `call_state_table_handler`: An eight-instruction routine that makes an indirect call through a table indexed by a state value, and calls a small routine that merges the constant 246 into register d1.
- `advance_timed_sequence_entries`: A 30-instruction routine that decrements a counter and, when it runs out, loads the next pair of values from a table and advances the table pointer by six.
- `decrement_and_set_global`: Decrements a global variable and, if it or a certain value is not certain, stores a byte containing all bits set into a memory location.
- `set_global_to_255`: A two-instruction routine that stores the value 255 into a global byte and returns.
- `toggles_bits_and_updates_memory`: Toggles bits in a byte and performs XOR operations on a pointer-referenced memory location and a global variable, then updates a short value in a table.
- `mark_bitmap_bit_and_compare`: Sets one bit, selected by the low three bits of a byte value, in a bitmap byte, then XORs a value into a stored word and adds a difference into a second word.
- `forward_two_unnamed_calls`: A three-instruction routine, called from one unnamed routine, that makes two calls to other unnamed routines and then returns.
- `advance_second_timed_sequence`: A 30-instruction routine that decrements a timer and, when it runs out, loads the next pair of values from a table and advances the table pointer by six.
- `decrement_and_clamp_global`: Decrements a global variable and, if it falls below zero, sets a certain memory location to a certain value and resets the global variable to a maximum value.
- `set_global_to_max`: Sets a global variable to a value where all bits are set.
- `check_record_list_overlap`: Walks a list of fixed-size records, skipping inactive ones, and compares each record's position against a bounding box derived from two globals.
- `scan_record_list_for_hits`: Walks a list of records and tests whether each active record overlaps a box built from an input position.
- `append_conditional_list_entry`: Calls an unnamed routine while one counter is nonzero, steps a field every fourth pass, selects a value through a global, and appends an eight-byte entry with a one-byte value of 255.
- `append_adjusted_word_entry`: Appends an entry to a list whose two words come from a longword with its low word reduced by four, plus a selected value and a one-byte value of 255.
- `tiny_ori`: Small routine of 1 instruction that merges 246 into d1.
- `invert_global_and_clear_ptr`: Inverts the bits of a global variable and sets a pointer-like value to zero.
- `invert_state_and_reset_pointer`: Inverts the bits of a global variable if certain conditions are met, then clears a pointer-like value to zero.
- `assign_records_from_table`: Walks a list of records and, for each active record whose type code is not eight and whose status word is zero, marks it as taken and loads the next table entry.
- `update_active_slot_entries`: Steps a list of active slots.
- `reset_field_and_call_routine`: Clears a word field, sets a second field to 3, and calls one unnamed routine.
- `find_overlapping_entry_flag`: Scans a list of entries for a box overlap, then sets a flag byte in the entry to a specific value if a match is found.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_timed_event_table`: Walks a fixed table of event records, decrementing per-record timers and dispatching on a type code.
- `test_point_inside_box`: Checks if a point, adjusted by offsets, falls within the bounds of a box defined by a pointer, returning a coordinate if it does.
- `initialize_structure_state`: resets several global or static memory locations to zero or a fixed value and updates a state-tracking counter.
- `process_structure_state_updates`: A very large 934-instruction routine called from four unnamed routines.
- `update_structure_status_fields`: A 178-instruction routine that tests several flag and counter fields of a structure, sets flag bytes, and stores pointers and values into state words depending on a status code.
- `advance_second_status_structure`: A 185-instruction routine with a test-and-set sequence on a different structure offset, ending by storing a pointer and a value.
- `run_counter_driven_state_step`: A 290-instruction routine that resets a state structure to zero when its counter reaches zero, stores the current word into a global table with a running index, and clears several flag fields.
- `run_timed_record_state_step`: A 300-instruction state routine that tests a counter field of a structure.
- `init_state_and_table_record`: A 34-instruction routine, called from three unnamed routines, that resets state fields and appends an 18-byte table record holding two routine pointers, a status value of 0 or 2, and a one-byte value of 255.
- `init_variant_table_record`: A 440-instruction routine called from eight unnamed routines.
- `apply_background_color_update`: A 319-instruction routine, called from five unnamed routines, that touches the background color register and calls four other unnamed routines.
- `pack_position_into_two_words`: Computes two packed words from an input X value, a Y value and an offset, and stores them at a destination pointer.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `dispatch_typed_list_entries`: Walks a list of nine-word entries and dispatches on each entry's type code.
- `emit_records_via_register_block`: Walks a list of 12-byte records, waiting on a status bit before each one, then writes several fixed-offset words of a register block from each record's fields.
- `copy_nearby_records_with_offsets`: Walks a list of eleven-word records and, for those whose position words lie within a small horizontal window, copies the record into an output area and writes values derived from a vertical offset and a table lookup.
- `combine_masked_values_into_buffers`: A 696-instruction routine, called from three unnamed routines, that touches a custom chip register at offset zero and calls three other unnamed routines.
- `swap_display_buffers`: Swaps between two display buffer offsets held in shared longwords and records which buffer is active in a flag word.
- `call_four_routines_in_order`: A five-instruction routine that makes four calls to other unnamed routines in sequence and then returns.
- `blit_frame_table_entries`: Waits for the blitter to be free and programs its control registers.
- `clear_global_word_leaf`: A two-instruction routine that stores zero into one global word and returns.
- `copy_record_entries_to_globals`: A 197-instruction routine, called from one unnamed routine, that selects one of two tables by a flag word and walks linked entries.
- `emit_threshold_table_entries`: An 85-instruction routine, called from one unnamed routine, that waits on a status bit and then walks one of two tables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_at_once_leaf`: A single-instruction routine that returns immediately.
- `run_per_state_table_passes`: A 43-instruction routine that returns early unless a counter is at most one.
- `fill_bullet_pattern`: Performs a bitwise rotation on a value and uses the result to copy multiple bytes from a table into a destination buffer.
- `copy_block_rows_into_buffer`: A 35-instruction routine, called from two unnamed routines, that touches the background color register.
- `dispatch_conditional_calls_by_state`: A 31-instruction routine that makes conditional calls to unnamed routines, based on a state word and a global counter compared with zero and four.
- `write_two_digit_strided_bytes`: Clamps an input value to 99, then writes its tens digit and units digit as lookup-table bytes into an output buffer at 40-byte strides.
- `copy_clamped_rows_nested_loops`: Clamps an input count to four, then copies sixteen-word rows from a source pointer into a destination buffer in nested loops, writing with fixed strides of 20 and 820 bytes.
- `spawn_entity_data`: Increments a counter and stores specific values into a global table and associated memory locations if the counter is below a certain threshold.
- `clear_bit_then_spin_countdown`: Clears one bit, selected by the low three bits of a register value, in a byte at a fixed offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_sequence_when_counter_zero`: A 63-instruction routine that runs only when a state value equals five.
- `write_four_strided_register_sets`: A 26-instruction routine, called from one unnamed routine, that waits on a status bit and then writes four sets of fixed words into a register block.
- `write_four_blocks_with_waits`: A 113-instruction routine, called from two unnamed routines, that writes four blocks to a register block and waits on a status bit before each one.
- `write_table_values_at_index`: Writes a value from one table, and related values from a second table at fixed offsets, into a buffer at the current index, then advances that index and calls a routine.
- `update_display_parameters`: Modifies a control byte, updates a global variable, and copies several values from a source address to a destination address.
- `refresh_table_selection_flags`: A nine-instruction routine called from three unnamed routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `rotate_table_selection_bytes`: A 38-instruction routine executed in the reference run that touches the DMA control register and makes two calls.
- `interpret_packed_command_stream`: A 70-instruction routine, called from one unnamed routine, that interprets a stream of signed command bytes read through a pointer.
- `parse_command_stream_codes`: A 58-instruction routine, called from one unnamed routine, that reads signed command bytes from a stream.
