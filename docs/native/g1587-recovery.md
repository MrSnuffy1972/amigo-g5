# Game 1587 (pang (1990)(ocean)[cr proj inc][t +4 proj inc][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01587). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (938064 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 25 raw-word lines |
| Functions named and described | pass, 95 of 95 named (0 author symbols), 95 of 95 described (4 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1688 of 1688 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6015 of 6015 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12965 |
| code | 22132 |
| data | 890147 |
| padding | 20 |
| unused | 12800 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pang (1990)(Ocean)[cr Proj Inc][t +4 Proj Inc][a].adf:bootblock | 1 | 214 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 972 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4217 | 23 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 209 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 238 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 166 | 3 |

## Cases handled

- pang (1990)(ocean)[cr proj inc][t +4 proj inc][a]/Pang (1990)(Ocean)[cr Proj Inc][t +4 Proj Inc][a].adf: custom-format (non-DOS) disk, 1733 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 207 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1200 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 127 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 148 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- pang (1990)(ocean)[cr proj inc][t +4 proj inc][a]/Pang (1990)(Ocean)[cr Proj Inc][t +4 Proj Inc][a].adf: blocks 1633-1709 (bytes 836096-875520) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.75)
- pang (1990)(ocean)[cr proj inc][t +4 proj inc][a]/Pang (1990)(Ocean)[cr Proj Inc][t +4 Proj Inc][a].adf: blocks 1711-1754 (bytes 876032-898560) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.77)
- 1 packed track region(s), 825344 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Pang (1990)(Ocean)[cr Proj Inc][t +4 Proj Inc][a].adf:bootblock

- `write_dma_and_interrupt_registers`: The decompiler timed out, so only the listed facts are available.
- `relay_single_routine_call`: The decompiler timed out, so only the listed facts are available.
- `pulse_drive_control_port`: Writes the drive control port with bit 7 cleared and the other bits set, clears bit 3, makes two calls, then sets bits 3 and 7 again.
- `decode_track_sectors_into_buffer`: Scans the raw track for sync words and matches each sector header against the requested track.
- `find_track_sector_then_decode`: Searches the raw track for the sector whose decoded track value matches the requested track, retrying up to ten times.
- `decode_sector_longs_then_search`: Decodes one sector of alternate-bit longword pairs from the raw track into the output buffer.
- `decode_both_track_sides`: Sets the side bit and decodes the eleven sectors of a track, matching each sector header and retrying up to ten times.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `seek_head_to_byte_offset`: Converts a byte offset into a track, a side and an offset within the track, using 5632 bytes per track.
- `seek_head_to_stored_track`: Sets the side flag in the record to 255 and clears the side bit in the control byte.
- `step_head_until_track_matches`: Steps the head one track at a time, in whichever direction is needed, until the current track equals the target track in the record.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `poll_port_bit_then_clear`: Calls a routine repeatedly while bit 4 of a hardware control byte stays set.
- `wait_disk_ready_clear`: Waits until bit 4 of the disk controller's control register clears, then writes zero to a byte in a global variable.
- `clear_direction_bit_and_step`: Clears bit 1 of the shared control byte, which the stepping routines use as the direction, then calls a routine.
- `step_back_set_direction`: Sets bit 1 of the shared control byte, then calls a routine.
- `set_bit0_then_call_routine`: Clears and then sets bit 0 of the shared control byte, so bit 0 ends up set with the other bits unchanged, and then calls a routine.
- `program_port_registers_and_wait`: Writes fixed values to several hardware port bytes, then waits until bit 0 of another port byte is set.
- `start_disk_read_and_wait`: Sets the DMA and disk control registers, writes the disk sync word, clears the disk-block interrupt request, and starts a disk read of a fixed length.
- `stop_disk_dma`: Writes the disk length register with the DMA-off value, clears the disk DMA enable bit in the control register, and returns 0.
- `wait_while_status_bit_set`: Busy-waits while bit 5 of a hardware status byte at a fixed offset stays set, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restart_copper_list_after_wait`: ORs a fixed longword pattern into successive buffer entries while a flag is clear.
- `stub_return_immediately`: Empty routine that returns to its caller at once without doing anything.
- `install_handler_and_clear_interrupt`: Calls a routine, stores the address of a handler routine in a shared pointer, and writes 2048 to the interrupt request register to clear one interrupt request.
- `install_handler_after_cia_reset`: Calls a routine to finish the previous step and, when a shared value is nonzero, writes 32776 to a control register.
- `run_copper_state_dispatcher`: Strobes the second copper-jump register, then runs a state-dependent branch.
- `run_state_machine_step`: Runs one step of the same state machine as the dispatcher, calling the state routine selected from an indirect table and decrementing a countdown.
- `advance_colour_step_counter`: Clears a flag byte and a state variable, makes a call through a table entry, and tests a combination of status bytes.
- `append_copper_list_entry`: When bit 2 of a status byte is clear, appends two longwords and a flag byte to the list at the current index and advances the index by eight.
- `update_counter_digit_display`: When a state variable is below two and a flag is set, it compares the hundreds digit and then the tens digit of a counter with stored digits.
- `build_plane_transfer_requests`: Computes an offset from its argument and writes two groups of four transfer requests into a control block.
- `append_list_entry_slot_a`: Appends two longwords and a flag byte to a list at the current index.
- `append_list_entry_slot_b`: Appends two longwords and a flag byte to a list at the current index.
- `call_state_routine_from_table`: Calls one routine selected from an indirect table by the current state value, then returns.
- `advance_timed_table_entry`: Counts down a timer and, when it expires, loads the next entry from a table and advances the table pointer by six bytes.
- `decrement_counter_set_flag`: Decrements a shared counter and, while the result stays nonnegative, sets a flag byte in the frame to 255.
- `set_shared_counter_max`: Sets a shared counter to 255 and returns.
- `xor_bitmap_words_and_accumulate`: Sets one bit of a flag byte selected by the low three bits of the index, XORs two mask values into the words at the pointers, and updates a shared value.
- `set_bit_and_accumulate_carry`: Sets one bit of a flag byte selected by the low three bits of the index and XORs a mask into a word and a shared value.
- `chain_two_routine_calls`: Calls two routines in sequence and returns, with no other effect.
- `advance_second_timed_table_entry`: Counts down a second timer and, when it expires, loads the next entry from a second table and advances that table pointer by six bytes.
- `decrement_and_maybe_reset`: Decrements a global variable, and if the result stays above negative one writes 255 to a byte at a fixed offset from a hardware pointer, otherwise resets the global variable to 255.
- `set_global_to_max`: Assigns the value 255 to a global variable and returns.
- `test_entries_against_box`: Returns early when a status byte is set.
- `test_entries_against_box_pointer`: Walks the same list of active entries and runs the same box overlap test as the box check routine.
- `append_entry_when_active`: When one status word is nonzero it makes one call.
- `append_entry_from_pointer`: Appends an entry to the list holding a position value taken from the pointer argument, reduced by four, and a value chosen from a table.
- `tiny_ori`: Small routine of 1 instruction that merges 246 into d1.
- `toggle_shared_flag_return_80`: Inverts a shared flag byte, clears a state pointer, and returns 80 with no other effect.
- `toggle_flag_on_state_25`: When the state value equals 25, inverts the shared flag byte and resets the state value to 0.
- `start_pending_list_entries`: On a tick aligned to eight, walks a list of entries.
- `animate_timed_entries`: Walks a list of timed entries, counting down each entry timer and advancing its animation frame bits while the timer runs.
- `clear_word_and_set_state`: Clears a word at the pointer argument, sets a state word in the work area to 3, and calls a routine.
- `find_overlapping_entry_flag`: Scans the list of entries for the first one whose box overlaps the box in the argument.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `walk_timed_record_table`: Walks a table of timed records, decrementing each active record counter and branching on a per-record type code to update a value or a state table.
- `test_point_inside_box`: Checks whether a point lies inside a box whose bounds come from the pointer argument, after shifting the point by two offsets from the caller.
- `run_object_script_step`: Advances an object script by reading the next command from a script pointer list.
- `advance_object_animation_state`: Advances the animation of one object record.
- `step_entity_motion`: Advances one entity motion record when its active word is negative.
- `step_entity_motion_slot_b`: Advances a second entity motion record in the same way as the first motion routine, using the second record fields.
- `countdown_and_spawn_object`: Counts down a timer field of the object and, when it reaches zero, clears the active word and appends a new 18-byte object record to the list.
- `countdown_spawn_object_slot_b`: Counts down a second timer field of the object and, when it reaches zero, clears its active word and appends a new 18-byte object record to the list.
- `initialise_object_and_append_record`: Clears the state fields of an object record and marks its active word.
- `initialise_object_variant_b`: Clears the state fields of an object record with the second layout and marks its active word.
- `set_colour0_to_white`: Writes 4095, the largest 12-bit colour value, to colour register zero and returns.
- `write_sprite_control_words`: Writes two control words for a hardware sprite at the pointer argument.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `scan_object_list_reset_index`: Walks the object list until it reaches an entry whose active byte is negative.
- `blit_list_entries`: Walks a list of three-word entries until a negative marker.
- `blit_sized_entries`: Walks a list of eleven-word records until a negative marker.
- `write_custom_register_block`: Computes a set of values from its inputs and writes them into the custom chip register block through the base pointer.
- `swap_display_buffers`: Swaps between two display buffer offsets held in shared longwords and records which buffer is active in a flag word.
- `call_four_routines_in_order`: Calls four routines in sequence and returns, with no other effect.
- `blit_frame_table_entries`: Waits for the blitter to be free and programs its control registers.
- `clear_shared_word`: Clears a shared 16-bit word and returns.
- `build_blit_entries_for_mode`: Selects one of two tables by a mode word, then walks a list of four-word entries and builds a blitter entry from each one and the table pointer.
- `blit_table_entries_when_idle`: Waits for the blitter to become idle, then walks a table of entries.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_only`: Returns immediately without any effect.
- `find_table_insert_position`: Returns unless a state value is small.
- `render_nibble_glyphs`: Splits a 32-bit value into four-bit groups, skipping leading zero groups, and writes the glyph bytes for each group into a bitmap.
- `copy_selected_table_rows`: Selects a source table from a one-based index, first running a countdown loop when the index is out of range.
- `conditionally_run_update_steps`: Makes up to four calls, each gated by the state value and by whether a shared counter is above zero or four.
- `render_two_digit_value`: Clamps a value to 99, then writes the glyph bytes for its tens digit and its units digit into the destination using lookup tables.
- `copy_bitmap_block_rows`: Clamps a row count to four and copies sixteen-word runs from the source into the destination one row at a time.
- `append_state_list_entry`: When a state byte is set, increments it and appends a list entry built from the value stored for the current stage.
- `clear_bit_then_delay_loop`: Clears one bit of a flag byte selected by the low three bits of the argument.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_state_five_records`: Runs only when the state value equals five.
- `blit_two_plane_rows`: Waits for the blitter to become idle, then programs its control, mask and size registers.
- `blit_single_row_pair`: Waits for the blitter to become idle and programs its control and size registers.
- `write_table_values_at_index`: Writes a value from one table, and related values from a second table at fixed offsets, into a buffer at the current index, then advances that index and calls a routine.
- `store_table_values`: Loads a value from a table indexed by a running counter and stores four 16-bit values into a table.
- `unavailable_body_routine`: A nine-instruction routine whose decompiled body is unavailable, so only its entry conditions and callers can be described.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_dma_control_from_table`: Reads a word from a table selected by a shared step counter and writes it to the DMA control register, then sets up the next step.
- `decode_byte_runs_into_words`: Walks a table of bytes in a loop, combining bits from each byte into output words and advancing the pointers until the run ends.
- `decode_byte_runs_with_offset`: Walks a table of bytes in a loop with a nine-byte offset, combining bits from each byte into output words until the run ends.
