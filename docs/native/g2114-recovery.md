# Game 2114 (walker (1993)(psygnosis)(disk 1 of 3)[cr scx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02114). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 203 of 203 named (0 author symbols), 203 of 203 described (15 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 607 of 607 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 16892 bytes, 89 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5315 of 5326 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 34387 |
| code | 18710 |
| data | 448614 |
| padding | 3121 |
| unused | 396288 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:WALKTRO | 4 | 2483 | 51 |
| startup | 1 | 882 | 62 |
| Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:bootblock | 1 | 10 | 1 |
| code (raw code file, load address unknown) | 1 | 1949 | 89 |

## Cases handled

- walker (1993)(psygnosis)(disk 1 of 3)[cr scx]/Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:game/logodd.ppc: high-entropy data file (7.98 bits/byte), classified as packed data
- walker (1993)(psygnosis)(disk 1 of 3)[cr scx]/Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:game/nwalkedd.ppc: high-entropy data file (7.99 bits/byte), classified as packed data
- walker (1993)(psygnosis)(disk 1 of 3)[cr scx]/Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:game/nwalkedd.ppd: high-entropy data file (7.99 bits/byte), classified as packed data
- walker (1993)(psygnosis)(disk 1 of 3)[cr scx]/Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:game/gmanagdd.ppp: high-entropy data file (7.89 bits/byte), classified as packed data
- walker (1993)(psygnosis)(disk 1 of 3)[cr scx]/Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:game/nwalkedd.ppp: high-entropy data file (7.96 bits/byte), classified as packed data
- walker (1993)(psygnosis)(disk 1 of 3)[cr scx]/Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:game/nwlevdd.ppc: high-entropy data file (7.98 bits/byte), classified as packed data
- walker (1993)(psygnosis)(disk 1 of 3)[cr scx]/Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:game/nwlevdd.ppp: high-entropy data file (7.96 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- walker (1993)(psygnosis)(disk 1 of 3)[cr scx]/Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:s/code: raw code file no run loaded verbatim: recovered position-independently (load address unknown)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:WALKTRO

- `initialize_hardware_at_entry`: Program entry routine that disables multitasking around opening graphics.library through the exec library calls.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into (a6).
- `service_fire_button_interrupt`: Interrupt-style handler reached through a vector stored in data.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `advance_ring_indices_and_accumulate`: Advances two ring-buffer offsets and recomputes two stored values from table entries shifted right by nine bits.
- `tiny_subi`: Small routine of 1 instruction that subtracts 0 from d0.
- `initialize_tables_with_stride`: Fills a 31-entry table of 32-bit values that rise by 680 per step, then makes helper calls.
- `reset_tables_with_stride`: Has the same body as the initialisation routine at the neighbouring address: fills a 31-entry table with a 680 step, makes helper calls, then clears a 98-entry word table with helper calls for each entry.
- `rotate_three_buffer_pointers`: Rotates three stored buffer pointers, updates a pointer pair, then waits while a status bit in one structure is set.
- `advance_position_and_fill_pointers`: Advances a counter and, when the counter reaches seven, sets a flag instead.
- `set_status_and_refresh_entries`: Sets the high bit of a status byte held in an address register, advances a ring index, and refreshes eight entries.
- `refresh_entries_with_status_set`: Has the same body as the neighbouring routine: sets the high bit of a status byte, advances a ring index, and refreshes eight entries from the lookup table.
- `tiny_ori_3`: Small routine of 1 instruction that merges 128 into (a4).
- `refresh_entries_without_status`: Refreshes the same eight entries from the lookup table without setting the status bit.
- `refresh_entries_single_lookup`: Refreshes eight entries from the lookup table, storing address halves and copying three lookup words into each entry's trailing fields.
- `wait_then_program_blitter`: Waits until the blitter-busy bit in the DMA status register is clear, then writes control and mask words and two address fields into the chip register block.
- `program_blitter_and_copy_block`: Waits for the blitter-busy bit to clear, writes control and mask words, then copies a run of table values into a memory region at descending offsets.
- `copy_table_block_unrolled`: A 314-instruction routine whose visible part copies a long run of 32-bit values from one table into another at descending offsets, and the listing reports a custom chip touch.
- `transform_triplets_to_points`: Advances three ring-buffer indices, reads table entries at those indices and builds several fixed-point coefficients from pairs of them.
- `compute_face_shade_entries`: Waits until the blitter-busy bit is clear, then sets fields in the chip register block.
- `store_clamped_face_values`: Waits until the blitter-busy bit clears, sets the same chip register fields as its neighbouring routine, then walks the record list.
- `tiny_ori_4`: Small routine of 1 instruction that merges 31 into d0.
- `clip_and_setup_line_draw`: Orders two endpoints and clips the segment against two horizontal limits using interpolated intersections.
- `clip_and_setup_line_unmasked`: Performs the same clipping and line-term calculation as its neighbouring routine, but takes the endpoint values without the low-bit masking.
- `setup_unclipped_line_draw`: Orders two endpoints without any clipping, computes the octant and step terms of the line, and writes them to blitter-style chip registers.
- `fill_words_from_helper`: Loops over a count held in a data register, calling a helper on each step and storing its word result at the address held in an address register, then returns.
- `store_helper_words_in_loop`: Loops a count from a data register, calling a helper on each pass and storing its word result at the address held in an address register.
- `copy_word_then_fill_loop`: Copies one word from the source address register to the destination, then loops a count, calling a helper on each pass and storing its word result at the destination.
- `fill_word_pairs_from_helper`: Loops a count, calling a helper on each pass and writing its word result into two consecutive words at the address register.
- `duplicate_word_pair_fill`: Has the same body as the neighbouring routine: loops a count, calling a helper and writing its word result into two consecutive words at the address register.
- `step_three_nibbles_toward_target`: Splits a 12-bit value into three four-bit fields and moves each field one step toward its corresponding target value held in other registers.
- `accumulate_pairs_and_set_bits`: On the first call with a zero flag it initialises a pair table.
- `advance_offset_and_reload_stream`: Advances a stored offset by 368 and, when it passes its limit, resets it and reads a fresh byte stream.
- `compute_scaled_pair_differences`: Walks two paired tables of 32-bit values and stores the shifted-right-by-four difference of each pair in a destination table.
- `recompute_scaled_pair_differences`: Has the same body as the neighbouring routine: stores the shifted-right-by-four differences of paired table values into a destination table, with the loop count taken from a data register.
- `decode_byte_stream_into_entries`: Reads a byte stream until a zero byte.
- `clear_init_flag`: Clears a global flag that marks the pair table as initialised, then returns.
- `load_next_text_glyph_columns`: Once every sixteen calls it reads the next text byte, converting values above 96 to uppercase by subtracting 32, and fills three buffers of 14 entries from a character table.
- `reset_pointer_list_after_scan`: Scans a byte table for the first entry above a running threshold.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `advance_audio_tick_state`: Advances a tick counter, and on the wrap runs a short delay and writes eight pointer words into the custom chip audio channel registers.
- `update_audio_channel_effects`: Calls four helpers, then reads a command word and branches on its low nibble.
- `process_tracker_channel_row`: Reads a packed command word for one channel when its state is empty, then loads a sample pointer, length, volume and period from tables for a non-zero instrument.
- `apply_tracker_effect_command`: Applies one effect command to a channel state.
- `copy_channel_word_to_register`: Copies one 16-bit field from a channel state record into a register-block field and returns.
- `find_target_period_entry`: Searches a per-instrument table of 16-bit period values for the first entry above the channel's current period, stepping back one entry when a flag bit is set.
- `slide_channel_value_toward_target`: Moves a stored channel value toward its target by a per-tick increment, clearing the target once it is reached.
- `apply_waveform_offset_to_register`: Computes an offset from a phase counter using one of three waveform modes selected by two bits: a table lookup, a signed ramp, or a constant.
- `interpret_channel_effect_code`: Calls a helper, then reads the low nibble of a channel command byte as an effect code.
- `store_low_nibble_parameter`: Copies the low four bits of a parameter byte into a state byte of the channel record and returns.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### startup

- `run_program_entry_sequence`: Program entry routine that calls a helper and compares its result with 31.
- `mark_data_region`: The routine contains no executable instructions; the bytes at this location are data rather than code, so there is nothing for it to do.
- `return_zero_handler_value`: Returns zero without any other work.
- `return_zero_callback_value`: Returns zero without any other work.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_routine`: This routine is empty; it performs no operations and returns immediately.
- `clear_state_and_run_setup`: Clears a run of 91 bytes in a structure, then runs a chain of helpers that stop at the first failure.
- `open_workbench_when_flagged`: Calls three helpers, then calls the intuition library's OpenWorkBench through a library vector when a flag bit is set.
- `probe_dos_file_handle`: Opens a file through the DOS library and, when the handle is valid, passes it to a second DOS call before closing it.
- `parse_hunk_records_into_buffer`: Walks AmigaDOS hunk records, returning at once unless the first word is the header type.
- `load_hunk_code_records`: Walks AmigaDOS hunk records, skipping relocation records by their stored count and copying code record contents into the destination.
- `return_empty_gap_routine`: Contains only a return with no operations.
- `copy_counted_long_block`: Reads a count from the first long of a source block, then copies that many longs following it into the destination pointer.
- `scan_hunk_and_copy_code`: Walks hunk records by their stored lengths until it reaches a code record, then copies that record's longs into the destination.
- `accumulate_through_pointers`: Reads a marker value and, when it matches a specific constant, adds a base register to a pointed-to value and stores the sum back through the same pointer.
- `return_stack_value_unchanged`: Returns the value taken from the stack without changing it, so it has no effect.
- `return_stack_value`: Returns the value held at the top of the stack to its caller without modifying it.
- `run_three_checks_then_cleanup`: Calls a helper, and if it returns zero calls a second helper and then a third.
- `own_blitter_and_forbid`: Calls the graphics library's OwnBlitter through a vector in the caller's structure, then calls exec's Forbid through the exec base held in a global slot.
- `permit_and_disown_blitter`: Calls exec's Permit through the exec base held in a global slot, then calls the graphics library's DisownBlitter through a vector in the caller's structure.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `reset_registers_and_jump`: Calls two helper routines, stores a routine address into a global variable, triggers a processor trap, and writes reset values into custom chip registers.
- `reset_cia_and_custom_registers`: Writes interrupt and DMA reset values through the custom chip base register, zeroes a block of 32 words, and then calls through a jump table the decompiler could not recover.
- `display_intuition_beep`: Calls the intuition library's DisplayBeep through a vector in the caller's structure and returns the register value unchanged.
- `count_and_invoke_routine`: It counts the characters in a null-terminated string and, when the count is zero, returns a fixed nonzero code.
- `invoke_two_routines_checked`: Checks a stack value and, when nonzero, invokes a routine from a structure twice.
- `write_two_chunks_checked`: Makes two calls through the DOS Write vector, returning 16 if the first fails, 24 if the second fails, and zero when both succeed.
- `write_one_chunk_checked`: Makes one call through the DOS Write vector and returns 24 when it fails, or zero when it succeeds.
- `return_stack_value_callback`: Returns the value taken from the stack without changing it, so it has no effect.
- `store_request_and_autorequest`: Saves a word from the caller's structure and a second argument into global slots, then calls the intuition library's AutoRequest through a vector in that structure.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d1, then merges 13 into d6, then merges 0 into d0.
- `call_autorequest_directly`: Calls the intuition library's AutoRequest through a vector in the caller's structure and returns its result.
- `conditionally_call_via_structure`: Checks a register value and, when nonzero, calls a routine looked up through a structure held in a register.
- `validate_size_arguments`: Returns error code 21 when the second argument is zero or negative, zero when the first argument is below 42, and error code 22 otherwise.
- `check_result_and_store`: Calls a routine through a global structure and inspects a field of its result.
- `invoke_table_routine_conditionally`: Reads a pointer from a global table and calls the routine it references.
- `poll_task_message_slot`: Calls exec's FindTask, and if a field of the returned task is zero it calls FindTask again and then GetMsg.
- `open_three_libraries`: Opens three libraries through exec's OpenLibrary, storing each base in the caller's structure.
- `close_three_libraries`: Calls exec's CloseLibrary on each of up to three stored library bases and clears each stored pointer.
- `open_misc_resource_and_setup`: Opens the misc resource through exec's OpenResource, storing it and returning error 4 when that fails.
- `allocate_misc_resources`: Claims two misc resources through the first entry of the misc resource vector, recording each claim in a flag byte.
- `claim_misc_resource_pair`: Claims two misc resources through the first entry of the misc resource vector, recording each claim in a flag byte.
- `allocate_potgo_resource`: Opens potgo.resource through exec's OpenResource and returns error 7 when it is missing.
- `run_three_setup_helpers`: Calls three helpers in sequence and returns, with no checks on their results.
- `free_misc_resources`: For each misc resource whose claim flag byte is set, calls the resource's release entry at offset minus 12 and clears the flag.
- `free_misc_resource_pair`: Releases two misc resources through the release entry at offset minus 12 for each claim flag byte that is set, clearing each flag as it goes.
- `free_potgo_bits`: When the potgo claim value is nonzero, clears it and calls the potgo resource's release entry at offset minus 12.
- `remove_named_device_if_present`: Disables multitasking, looks up a named node with exec's FindName, removes it with RemDevice when it is found, and then re-enables multitasking.
- `install_interrupt_servers`: Copies three stored values from the caller's structure and removes an existing interrupt server through exec when one is set.
- `remove_interrupt_servers_and_release`: Removes an interrupt server through exec's RemIntServer when its flag is set, then calls exec's entry at offset minus 12 for two further flagged blocks.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `store_result_or_return_value`: Checks a field of a structure and, when nonzero, calls a routine through a global structure and stores its result into another structure field.
- `allocate_memory_then_continue`: Has the same body as the neighbouring allocation routine: allocates a memory block through exec's AllocMem when a size field is nonzero, returns error 32 on a null result, and otherwise passes control to a helper.
- `setup_buffers_with_checks`: Calls helpers to fill pointer fields of the structure when its size fields are nonzero.
- `find_region_with_free_size`: Walks the list of memory regions from a global root and returns the free size of the first region whose attribute flags match and that is large enough for the requested size.
- `record_memory_stats_and_allocate`: Records four values from exec's AvailMem into the caller's structure, then allocates optional blocks through AllocMem.
- `allocate_three_optional_blocks`: Allocates up to three optional blocks through AllocMem, storing each pointer and returning error 12, 13 or 14 for the first allocation that fails.
- `allocate_two_optional_blocks`: Allocates up to two optional blocks through AllocMem, storing each pointer and returning error 13 or 14 when one fails.
- `allocate_final_optional_block`: Allocates one optional block through AllocMem when its size field is nonzero, storing the pointer and returning error 14 when it is null.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `free_five_memory_blocks`: Frees up to five stored memory blocks through exec's FreeMem, clearing each stored pointer after it is released.

### Walker (1993)(Psygnosis)(Disk 1 of 3)[cr SCX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### code (raw code file, load address unknown)

- `run_boot_block_entry`: Start of the boot block code, which calls the boot entry routine that finds the DOS library's resident module.
- `reset_interrupt_and_dma_registers`: The decompiler produced no body, but the listing shows this routine touches the interrupt enable and request registers, the DMA control register and the background colour register, and it calls one other routine.
- `process_listed_game_files`: Works through a loop of helper calls that handles the three game file names referenced in its text, repeating while a helper result is nonzero.
- `clear_memory_and_reset_custom`: Writes DMA, interrupt-enable and interrupt-request control values through the custom chip base, clears a large block of memory and a second block addressed by pointer, then fills table entries.
- `clear_bitplane_rows`: Walks a table of six-pointer groups and, for each pointer, clears a byte at three fixed offsets that look like bitplane spacing.
- `dispatch_table_step_or_reset`: If a counter is nonnegative, calls through a dispatch table entry indexed by that counter and returns.
- `write_plane_pixel_and_advance`: ORs a mask into a plane byte and ANDs a second mask into the same position on two other planes spaced at fixed offsets.
- `update_wave_column_heights`: Toggles a parity flag, selects one of two 64-entry tables, and for each entry computes a height from a table value multiplied by a delta and offset.
- `advance_scroll_velocity`: Advances a phase counter by two within 0 to 255 and calls a helper.
- `store_argument_in_global`: Stores its address-register argument in a global slot and returns.
- `compare_and_update_column_entries`: Walks 64 three-word entries from one of two buffers and compares the low byte of each entry with a stored value.
- `reset_flags_and_wait_counter`: Sets two state flags, calls a helper, then waits in a loop until a counter reaches 2048.
- `service_interrupt_request_bits`: Checks two interrupt request bits in the custom chip registers.
- `probe_numbered_disk_units`: Loops over disk unit numbers, writing a digit into a name template for each attempt and calling helpers to probe the unit.
- `read_joystick_axes_and_button`: Stores the previous two bytes read from a state structure and derives a change for each axis by halving the difference.
- `shift_feedback_register_32_steps`: Runs 32 shift steps on a 32-bit global register.
- `select_mode_and_build_pair`: Calls helpers to fetch a value and, depending on whether a mode value is zero or a small code, fetches further values and returns a 64-bit result built from a high and a low 32-bit part.
- `unpack_word_and_byte_runs`: Copies data from a source table into a destination buffer in runs: whole 32-bit words first, then single bytes, with run lengths taken from control values in a structure.
- `decrement_counter_or_invoke_helper`: If a counter in the structure is zero and a second field is non-zero, it calls a helper and returns that result.
- `build_chunked_transfer_records`: Fills a buffer with records, each starting with a fixed tag, a running index and a payload length capped at 488 bytes, and copies the payload bytes from the source until the remaining total is used up.
- `store_value_into_selected_slot`: When a flag global is zero, it writes a 32-bit value built from two globals into a table slot chosen by a stored offset.
- `poll_until_counter_clears`: Calls a helper repeatedly while a 16-bit value is non-zero, then checks a field in the structure.
- `set_bit_in_word_table`: Takes a 32-bit index from the input registers, subtracts 2 from its low part, and sets the bit at the matching position in a table of 32-bit words held in the structure.
- `decrement_and_poll_until_match`: Decrements a 16-bit register value once, then repeatedly passes the updated value to a helper and stops when the returned value equals the constant 1759.
- `poll_until_value_matches`: Calls a helper once, then keeps calling it and storing its return value in a 16-bit register until that register equals the constant 1759.
- `combine_registers_into_pair`: Returns its two 32-bit register inputs unchanged as one 64-bit value, with the first input as the high half and the second as the low half.
- `clear_bit_in_word_table`: Takes a 32-bit index from the input, subtracts 2 from its low part, and clears the bit at that position in a table of 32-bit words held in the structure.
- `forward_single_helper_result`: Calls one helper routine and returns the value it produces.
- `copy_name_string_capped`: Copies a zero-terminated string into a fixed buffer inside the structure, stopping after 30 characters, and stores the number of bytes copied in a byte just before that buffer.
- `scan_slots_and_copy_blocks`: Scans a table of 72 longword slots in the structure.
- `clear_longword_table`: Writes zero into 1536 consecutive 32-bit slots of the structure, starting at a fixed offset.
- `initialize_disk_transfer_block`: Writes a few fixed values into a request block, waits in a loop until a helper returns the constant 1759, and then stores two helper results into the block.
- `find_matching_path_entry`: Searches a table of entries in the structure for one whose stored name matches the given path string, comparing characters one at a time.
- `parse_drive_letter_digit`: Reads two characters from the input string through a helper.
- `tiny_move`: Small routine of 1 instruction that copies (a0)+ to d0.
- `empty_routine_2`: It performs no observable work, does not access memory, registers, or other routines, and simply returns.
- `copy_segment_and_index`: Copies the leading characters of a string up to the first slash into a buffer and ends it with a zero byte.
- `forward_helper_status`: Calls one helper and returns the value it produces.
- `call_helper_with_status_check`: Calls a helper and, when the condition it leaves is set, calls it again and returns the fixed status 405 if that second condition is clear.
- `return_status_if_flag_clear`: Calls a helper and returns its result, unless the condition left by that call is clear, in which case it returns the fixed status 405.
- `sum_negate_longword_array`: Adds up 128 consecutive 32-bit values starting at the address register and returns the negated total in the high half of the result.
- `clear_and_store_helper_result`: Clears a 32-bit field at offset 20 of the structure addressed by the address register, then calls a helper and stores its result in the same offset of the address register left after that call.
- `clear_and_copy_longword`: It clears a longword at a fixed offset from the address passed in an input register, then copies a longword from that same location into another location and stores it.
- `clear_and_copy_slot_field`: Clears a 32-bit field at offset 20 of the structure pointed to by the incoming address, then copies a 32-bit value from a field at offset 20 of one structure into a field at offset 20 of another structure.
- `clear_and_copy_struct_fields`: Clears the four bytes at offset 20 of the structure pointed to by the incoming pointer, copies a value from a global variable into that same location, then stores a second value from another global variable into a different structure's offset 20.
- `clear_and_copy_struct_offsets`: Clears the four bytes at offset 20 of the structure pointed to by the passed address, copies a value from that same structure into a second structure, and then clears the four bytes at offset 20 of that second structure.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d2.
- `call_one_routine_and_return`: Calls one routine and returns, with no other work of its own.
- `forward_into_following_routine`: A single-instruction entry whose decompiled body repeats the code of the routine that follows it, so it adds no separate behaviour of its own.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 0 into d3.
- `convert_block_position_into_groups`: Divides a 16-bit block number by 11 to obtain a group count and a remainder, doubling the group value when a mode bit is clear.
- `read_disk_sector_with_retries`: Tries up to four times to read a disk sector: it clears the buffer fields, checks a disk-ready bit in the CIA-A port register, and writes the standard MFM sync words into the target buffer before calling helpers that wait for and transfer the data.
- `start_disk_dma_read`: Sets the disk controller mode bits, points the disk DMA address register at the buffer, writes the transfer length and enables the disk DMA channel.
- `write_sync_pattern_for_sectors`: Writes a repeated MFM sync pattern into eleven sector slots of a buffer, spacing the slots 1040 bytes apart.
- `verify_sector_header_with_retry`: Retries up to ten times to read a sector header through helpers.
- `verify_track_sectors_against_reference`: Checks each of the eleven sectors on a track in turn: it waits for data to be ready, compares a value read from the buffer with a stored reference, and returns zero when the sector matches.
- `empty_stub_routine`: It performs no observable work, contains no references to memory, registers, or other routines, and simply returns.
- `set_bit_and_increment_counter`: Sets one bit in a 16-bit word stored four bytes before the base pointer, using the low five bits of the input register as the bit index.
- `clear_strided_longwords`: Writes zero into ten 32-bit slots spaced 272 bytes apart, starting from the address held in the A5 register.
- `extract_low_word_top_byte`: Calls a helper for a 32-bit value and returns a result whose high 16 bits are the low 16 bits of that value and whose low 16 bits are its top byte.
- `interleave_alternating_bit_lanes`: Takes two consecutive 32-bit words from the address register and merges them by bit lanes: the masked lanes of the second word are kept, and the masked lanes of the first word are shifted up by one bit and combined with them.
- `obtain_value_and_pass_on`: Obtains a 32-bit value from a helper and hands it to another routine, then returns.
- `xor_fold_ten_words_masked`: XORs ten consecutive 32-bit words, starting eight bytes past the base pointer in the A5 register, and returns the result masked to every other bit.
- `compute_masked_xor_checksum`: XORs a run of 32-bit words from the address register into an accumulator.
- `fill_buffer_with_alternating_pattern`: Fills a buffer of 32-bit words with a fixed alternating-bit pattern, writing backwards from its end.
- `interleave_two_word_blocks`: Combines two blocks of 128 32-bit words.
- `repeat_helper_pairs_over_count`: Runs a loop whose count is the first input divided by four, calling two helpers on each pass, then calls two more helpers.
- `update_top_bit_after_helpers`: Calls two helpers, then updates a byte at the address register using a flag bit from the byte before it.
- `update_top_bit_from_flag`: Updates a byte at the address register using a flag bit from the byte before it.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `configure_disk_dma_transfer`: Enables the DMA and disk control bits, sets the disk sync word, points the disk address register at the buffer in the address register, and writes the transfer length derived from the input word with the enable bit set.
- `check_disk_status_bit`: Checks bit two of a status byte in the structure.
- `write_control_on_negative`: Writes a fixed control word.
- `set_port_bit_from_index`: Computes a bit position from a 16-bit field in the structure plus 3, sets that bit in the byte passed in the data register, and writes the result to the CIA-B port register.
- `poll_port_busy_bit`: Writes an all-ones value to the CIA-B port register, calls a helper, and then waits while bit 0 of that register is set.
- `step_table_entry_toward_target`: Reads a 16-bit entry from a global table selected by a field in the structure.
- `clear_entry_after_flag`: Polls bit four of a global input-status byte up to 86 times, calling a helper between polls.
- `set_port_bit_wait_ack`: Builds a control byte from a helper result, clearing bit 1 when the input byte is non-negative, and writes it to the CIA-B port register with bit 0 set.
- `write_helper_value_to_port`: Calls a helper and writes the byte it returns to the CIA-B port register.
- `compute_port_mask_byte`: Builds a control byte from a global value by clearing a bit whose position comes from a structure field plus three.
- `poll_until_helper_returns_one`: Calls a helper repeatedly, waiting while bit 0 of a global port register is set between calls, and returns zero once the helper returns 1.
- `write_timer_control_if_idle`: If bit 0 of a global control byte is clear and the input value is not 1, it writes three control values, 8, 204 and 2, in order to the same register.
- `write_timer_control_sequence`: Writes three fixed control values, 8, 204 and 2, in order to the same register.
- `unpack_rnc_compressed_block`: Checks a four-byte header against the ASCII signature RNC followed by version byte one.
- `lookup_bit_entry_in_table`: Scans a table of 16-bit pairs until one matches the masked input value, then reads a count byte from that entry and extracts a single bit from the input.
- `mask_word_or_call_helper`: ANDs two 16-bit inputs together.
- `return_empty_routine`: It contains no instructions that modify memory or registers and immediately returns.
- `read_four_bytes_big_endian`: Reads four consecutive bytes from the address register and combines them into one 32-bit value, with the first byte in the top position, and returns it.
- `build_canonical_code_table`: Reads a symbol count and then one code length per symbol.
