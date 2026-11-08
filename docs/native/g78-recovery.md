# Game 78 (Budokan v1.2 1215): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00078). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1636862 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 41 of 41 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 239 of 239 named (0 author symbols), 239 of 239 described (8 by rule, 39 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1791 of 1791 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4644 of 4664 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 216733 |
| code | 18884 |
| data | 1397591 |
| padding | 70 |
| unused | 3584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Budokan.Slave | 1 | 67 | 6 |
| logged disk load from Disk.1 | 1 | 2066 | 120 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2361 | 88 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 167 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |

## Cases handled

- Budokan_v1.2_1215/Budokan/Disk.1: raw disk image (794112 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Budokan_v1.2_1215/Budokan/Disk.2: raw disk image (788480 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 37 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 797 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Budokan_v1.2_1215/Budokan/Disk.1: 37 logged loads cover 225280 bytes of the image
- Budokan_v1.2_1215/Budokan/Disk.1: chunks 0-43 (bytes 0-11264) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.69)
- Budokan_v1.2_1215/Budokan/Disk.1: chunks 452-483 (bytes 115712-123904) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.31)
- Budokan_v1.2_1215/Budokan/Disk.1: chunks 946-1759 (bytes 242176-450560) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.94)
- Budokan_v1.2_1215/Budokan/Disk.1: chunks 1782-3101 (bytes 456192-794112) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.05)
- Budokan_v1.2_1215/Budokan/Disk.2: chunks 0-1762 (bytes 0-451328) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.95)
- Budokan_v1.2_1215/Budokan/Disk.2: chunks 1764-2242 (bytes 451584-574208) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.60)
- Budokan_v1.2_1215/Budokan/Disk.2: chunks 2244-3079 (bytes 574464-788480) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.89)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `pulse_drive_control_port` (Disk.1 +an address loaded at an address (22528 bytes)): Sets a control flag to zero, writes a control pattern to the second CIA port B register, and then calls a shared routine 250 times to pause. Kind: disk hardware/timing check. Pass path: the branch following the bsr.w calls (the code continues execution normally if the hardware response matches expected timing/patterns).
- `start_disk_dma_transfer` (Disk.1 +an address loaded at an address (22528 bytes)): Waits for the disk interrupt flag, then programs the disk DMA pointer and length registers with a fixed transfer size, enables the disk DMA and interrupt bits, and calls a shared routine. Kind: disk hardware/track validation. Pass path: u1__sub_175E() returns normally.
- `reset_disk_and_dma_control` (runtime segment an address): Clears a status byte and writes fixed values to the disk-control and DMA registers, finishing by masking one bit off the last value. Kind: disk hardware/DMA reset and state synchronization. Pass path: sub_3B62.

## Functions


### Budokan.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `poll_hardware_status_with_codes`: Writes a sequence of status codes to a work area while calling a routine that polls the hardware, and treats one specific failure code as an exit.
- `clear_interrupt_request_flag`: Clears one bit of a control word and writes a fixed value into a global, then returns the input register.
- `wait_blitter_then_enable_interrupts`: Loops until the blitter busy bit clears, runs a routine, and then writes the interrupt enable register with the enable bit set.
- `call_stored_entry_offset`: Calls the routine whose address is stored in the global entry table through an indirect call, and then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### logged disk load from Disk.1

- `copy_block_and_clear_memory`: Copies a block of longwords from one table to another, raises a trap, and clears a region of low memory with zeros.
- `copy_startup_block_and_clear`: Raises a trap, clears a low-memory region with zeros, and copies a block of words to a second area as part of the startup sequence.
- `set_four_bitplane_pointers`: Waits on the raster position, then stores the start addresses of four bitplanes as high and low word pairs.
- `start_blitter_copy_request`: Waits for the blitter to finish, loads its source pointer, clears its modulo and control registers, and starts a blit whose size comes from the width argument.
- `start_fixed_size_blit`: Waits for the blitter to finish, sets its source pointer, clears the modulo and control registers, and starts a blit of the fixed size 24000.
- `fill_table_from_global_value`: Copies one global value into each of thirty-three consecutive table entries.
- `swap_and_set_bitplane_pointers`: Swaps two stored buffer pointers depending on which one is current, waits for the raster position, and writes the four bitplane pointer pairs spaced 8000 bytes apart.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_frame_boundary`: Waits for the raster position counter to leave the sentinel value and then return to it, which marks one display frame boundary.
- `wait_for_raster_sentinel`: Loops until the stored raster position reaches the sentinel value, which marks the start of a frame.
- `clear_sixteen_word_table`: Sets one global word to 255 and clears a block of sixteen words that follows it.
- `copy_sixteen_word_table`: Copies sixteen consecutive words from a source table into a destination table.
- `combine_four_bitplanes_or`: Ors four bitplane buffers together row by row into one destination buffer, with the row size taken from the image width and height.
- `copy_header_and_combine_planes`: Copies the size and position fields from the input record, calls three shared routines, and then ors the bitplane rows together into the destination buffer.
- `shift_bitmap_rows_by_offset`: Shifts the rows of a bitmap horizontally by the amount needed to align its width to a 16-pixel word boundary, working through each plane's words.
- `unpack_bitplane_bits_to_words`: Walks a bitplane and shifts out one bit at a time from each source word, rebuilding destination words row by row.
- `queue_blit_request_params`: Waits for the blitter, stores two parameters and control values in a request record, and returns the two inputs offset by eight thousand each.
- `run_five_routines_if_enabled`: When the enable global is nonzero it calls five shared routines in sequence, then returns the register pair unchanged.
- `call_shared_routine_only`: Calls one shared routine and returns without doing any other work.
- `call_blit_flush_routine`: Calls a shared routine to update the blitter state and flush the operation, returning a negative value upon failure.
- `setup_blitter_with_clipping`: Loads the blitter masks, modulo values and control bits from an image record.
- `update_blit_state_and_flush`: Calls a shared routine and returns minus one on failure.
- `store_blit_request_fields`: Waits until the blitter is free, then stores the source, destination, size and modulo values for the next blit request.
- `update_blit_state_and_flush_2`: Calls a shared routine and returns minus one on failure.
- `reset_blit_state_and_flush`: Resets a blit state field when a stored flag is zero, then calls five shared routines and returns zero.
- `link_and_walk_record_list`: Initializes a list head so it points to itself, then walks the records of the list and calls a shared routine on each one until the end is reached.
- `queue_blit_request_with_mask`: Waits for the blitter to be free and then stores the source and destination pointers, the width and height values, and the control words for a blit request.
- `clear_interrupts_and_set_cia`: Clears the interrupt request bits, calls a shared routine, and writes the interrupt enable and CIA control registers to set up the interrupt sources.
- `set_key_state_table_entry`: Writes zero or 255 into one entry of a 128-byte key state table, selected by a bit of the state word, and then loops with no exit.
- `store_serial_key_state`: Reads a keyboard serial value and writes zero or 255 into the matching key state table entry, then loops with no exit.
- `mark_table_entry_pressed`: Sets one byte of a table, indexed by the masked input value, to 255, and then loops with no exit.
- `stub_infinite_loop_handler`: Its decoded body only loops forever with no exit, so it performs no work of its own.
- `spin_after_raster_wait`: Its decoded body is an infinite loop with no exit.
- `enable_serial_interrupt`: Clears one bit of the serial control register and writes the interrupt enable register with the serial interrupt enabled.
- `read_joystick_direction_state`: Reads the mouse and joystick port movement registers and the fire-button port, decodes the movement bits into direction flags, and calls a registered callback when one is set.
- `take_and_clear_global`: Returns the current value of a global word and resets the global to zero.
- `merge_key_state_tables`: Merges 128 bytes of key state into a second table and a third table with bitwise operations, updating the copies as it goes.
- `clear_three_state_tables`: Zeroes 32 longwords in each of three tables that start at fixed addresses.
- `set_interrupt_flag_bit_seven`: Stores the single-bit value for bit seven into the interrupt request global, marking that interrupt as pending.
- `set_interrupt_flag_bit_eight`: Stores the single-bit value for bit eight into the interrupt request global, marking that interrupt as pending.
- `set_interrupt_flag_bit_nine`: Stores the single-bit value for bit nine into the interrupt request global, marking that interrupt as pending.
- `set_interrupt_flag_bit_ten`: Stores the single-bit value for bit ten into the interrupt request global, marking that interrupt as pending.
- `init_audio_channel_three`: Initializes the state fields of audio channel three, setting its period, volume and interrupt request bit, and stores a pointer to its channel data.
- `dispatch_pending_interrupt_bits`: Checks interrupt request bits seven through ten, calls the matching handler for each pending bit, and then writes the interrupt enable register with the enable bit set.
- `initialize_all_audio_channels`: Initializes the state of all four audio channels, setting period, volume and handler fields, waits for a raster line, and then enables the DMA and interrupt bits for the channels.
- `schedule_delayed_callback`: Stores a timed callback request.
- `queue_interrupt_handler_request`: Stores a handler address and its arguments in a request record, then writes the control words that schedule it.
- `start_audio_channel_zero_sample`: Starts playback on audio channel zero by setting its sample pointer, length, period and volume, waiting for a raster line, and then enabling its DMA and interrupt bits.
- `start_audio_channel_one_sample`: Starts playback on audio channel one by setting its sample pointer, length, period and volume, waiting for a raster line, and then enabling its DMA and interrupt bits.
- `start_audio_channel_two_sample`: Starts playback on audio channel two by setting its sample pointer, length, period and volume, waiting for a raster line, and then enabling its DMA and interrupt bits.
- `start_audio_channel_three_sample`: Starts playback on audio channel three by setting its sample pointer, length, period and volume, waiting for a raster line, and then enabling its DMA and interrupt bits.
- `build_free_node_chain`: Links nineteen fixed-size nodes spaced 118 bytes apart into a chain, terminates the last one, and sets the list head to empty.
- `tick_timer_list_entries`: Walks a list of timer entries, decrementing each entry's count and returning the stored data of the first entry whose count reaches zero.
- `store_request_registers_in_record`: Copies seven register-passed values into a request record and marks the record as active.
- `poll_timer_list_for_expiry`: Walks a list of timer entries, decrementing their counts until one reaches zero, and returns that entry's stored value, or the list head value when none expire.
- `find_flagged_list_entry`: Walks a linked list and stops at the first entry whose flag byte is set or that matches the current entry.
- `insert_node_into_list`: Inserts a node into a linked list at the position where the search pointer stops, then decrements a counter kept in the list head.
- `clear_and_release_node`: Clears the contents of a node byte by byte and then returns it to the free list.
- `xor_fold_longword_table`: XOR-accumulates a table of 32-bit words into one running value and returns that value masked with an alternating bit pattern.
- `xor_checksum_table`: The routine iterates over a table of 32-bit words, XOR-accumulating them into a running value.
- `calculate_bitmask_checksum`: Iterates through a memory buffer to compute a cumulative XOR sum of the data.
- `calculate_xor_checksum`: Iterates through a memory block using a pointer, performing a cumulative XOR operation on the data.
- `calculate_checksum_parity`: Iterates through a memory buffer to compute a cumulative XOR sum of the words.
- `calculate_xor_checksum_2`: Iterates through a memory block to compute a cumulative XOR sum of the words.
- `calculate_xor_checksum_masked`: Iterates through a table of 32-bit words, XOR-accumulating them into a running value.
- `call_three_routines_pass_input`: Calls three shared routines in sequence and returns the input register unchanged.
- `store_masked_bitplane_word`: Builds a word from the alternating bit pattern of the input and its shifted neighbours, clears the top bit when a flag byte before the target is set, and stores the result through the pointer.
- `update_sprite_flag_bit`: Clears the top bit of a byte when the byte before it has its low bit set.
- `store_low_seven_bits`: Stores the low seven bits of the input byte through the pointer.
- `write_mfm_sync_marks`: Stores the standard disk sync word pattern twice in a record, then runs a sequence of routine calls that writes the track sync data.
- `call_four_shared_routines`: Calls four shared routines in sequence and returns.
- `start_blitter_fill_rectangle`: Waits for the blitter to be free, programs its first and last word masks, modulo values, pointers and size, and starts a blit that uses a fill pattern value.
- `interleave_two_bitplane_words`: Combines two longwords by taking the alternating bit pattern from each, with the second supplying the odd bits and the first, shifted left one bit, supplying the even bits.
- `call_shared_routine_pass_input`: Calls one shared routine and returns the input register unchanged.
- `start_blitter_copy_rectangle`: Waits for the blitter, programs its masks, modulo values, pointers and size from an image record, and starts a copy blit for the rectangle.
- `set_timing_mode_zero`: Sets the mode global to zero and stores the timing value 48 in a second global.
- `set_timing_mode_one`: Sets the mode global to one and stores the timing value 40 in a second global.
- `set_timing_mode_two`: Sets the mode global to two and stores the timing value 24 in a second global.
- `pulse_drive_control_port`: Sets a control flag to zero, writes a control pattern to the second CIA port B register, and then calls a shared routine 250 times to pause.
- `release_drive_control_port`: Writes the value 255 to the second CIA port B register, which releases the drive control lines.
- `load_cia_timer_values`: Updates a control word, loads a timer value of 100 and sets the interrupt mask on the CIA timer registers.
- `step_drive_head_to_track`: Moves a head position counter one track at a time toward the target track, calling a shared routine for each step, and returns when the positions match.
- `select_drive_and_reset_control`: Sets the drive-select bits on the second CIA port, selects motor state from a flag, and resets the control word and timer values.
- `select_drive_port_and_call`: Sets the drive-select bits on the second CIA port and calls one shared routine.
- `update_control_word`: Stores a value into a global variable and then calls another routine.
- `wait_for_port_bit_release`: Calls a shared routine repeatedly while bit four of the fire-button port stays set, and clears a counter when the bit is released.
- `split_block_to_track_sector`: Divides a block number by eleven, the sector count per track on an Amiga disk, and stores the side bit, cylinder and sector remainder in three globals.
- `fill_gap_and_write_sector`: Fills a block of 1086 bytes with the alternating byte pattern used for track gaps, then builds a sector header and calls a shared routine for the sector data.
- `start_disk_dma_transfer`: Waits for the disk interrupt flag, then programs the disk DMA pointer and length registers with a fixed transfer size, enables the disk DMA and interrupt bits, and calls a shared routine.
- `check_drive_status_bits`: Checks two status bits of the drive port.
- `check_drive_ready_bit`: Stores an error code of minus five when a drive status bit is clear, and otherwise calls a shared routine.
- `call_shared_routine_only_2`: Calls one shared routine and returns the register pair it leaves.
- `prepare_disk_sync_read`: Returns an error code of minus eight when the drive status bit is set.
- `stop_disk_dma_and_reset`: Enables the disk DMA control bit, sets a sector counter to minus one, calls a shared routine twelve times, and then clears the related state fields.
- `scan_track_for_sync_word`: Scans a track buffer for the disk sync word and decodes each sector header it finds through a shared routine.
- `clear_track_buffers`: Fills a 2000-longword buffer with minus one and then clears a second buffer of 2000 longwords with zeros, calling a shared routine when the flag is set.
- `fill_marker_and_decode_track`: Fills 1400 longwords with a fixed four-byte marker, then calls a sequence of shared routines that decode track data until a flag is set.
- `call_two_routines_return_status`: Calls two shared routines and returns the stored status value.
- `concatenate_stack_values`: Returns a value formed by concatenating a 32-bit value from the stack with another 32-bit value.
- `retry_drive_operation`: Stores a step value, then calls shared routines in a series of retry steps, returning early when a step succeeds.
- `return_stack_value`: Returns a 64-bit value stored on the stack.
- `check_retry_flags_set_status`: Runs a sequence of flag checks with shared routine calls between them.
- `retry_track_read_loop`: Starts a retry counter at twenty, then repeatedly sets up a step value of forty and calls shared routines until one succeeds or the status reaches minus eight.
- `seek_and_read_track_sectors`: Copies the start track and destination pointer from the inputs, then loops over shared routines that seek and read sectors until none remain.
- `halt_after_one_call`: Calls one shared routine and then loops forever with no exit.
- `copy_track_block_to_buffer`: Copies 128 longwords from a track buffer selected by a track index into the destination pointer, then sets the status to zero.
- `clear_record_list_flags`: Walks a table of eight-longword records until the sentinel value, zeroing three fields and clearing the top bit of each record's flag byte, then sets a state word.
- `clear_record_flag_bits`: Walks the same eight-longword record table until the sentinel value, clearing the top bit of each flag byte and zeroing one field, then sets a state word.
- `set_slot_flag_clear_word`: Sets the high bit of a status byte in a 32-byte slot record, chosen by the index in the data register, then clears the longword that follows it.
- `reserve_and_fill_sector_buffer`: Reserves a region sized in 512-byte blocks for a slot record, then copies staged data in 512-byte pieces when the source and target sizes match, otherwise calling a transfer subroutine.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a memory location.
- `decode_control_bit_stream`: Expands a packed input in which each control byte supplies eight flags: a set flag emits the byte from a lookup indexed by the previous output byte, and a clear flag copies the next literal input byte.
- `setup_trap_and_run_init`: Installs a trap handler address, clears a low-memory vector area, copies a block of code to a second location, and unpacks length-prefixed byte runs.
- `disable_interrupts_then_init_system`: Writes the interrupt-enable mask with bit 15 clear to disable all interrupts, clears a low-memory vector area, and copies a block of code into place.
- `restart_copper_after_raster_wait`: If a pending flag is set, clears it, runs four subroutines, and waits in a loop until a counter reaches a fixed value.
- `poll_flag_table_then_clear`: Its opening test is always true, so the code shown returns at once and the wait-and-clear logic after it is never reached.
- `run_main_wait_loop`: Calls several subroutines, zeroes a counter, and then loops indefinitely, doubling and subtracting the counter while calling subroutines and waiting on its value.
- `advance_counter_run_six_steps`: Increments a shared counter and then calls six subroutines in sequence.
- `advance_lfsr_state_eight_steps`: Advances a 32-bit shared state eight times, shifting it left and XORing in a fixed feedback constant when the shifted-out bit or a zero result requires it.
- `reset_state_after_eight_steps`: Calls eight subroutines, then clears four state words and stores two self-referencing addresses before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_shared_helper_repeatedly`: Makes a long run of identical calls to one subroutine.
- `copy_words_reset_state`: Calls one subroutine, copies two words from fixed source locations into two state words, then clears one state word and sets another to 255 before returning.
- `clear_two_state_words`: Writes zero to two state words and returns.
- `store_constant_after_helper_call`: Calls one subroutine and then stores a fixed constant before returning.
- `stub_truncated_decompile`: The decompiled source stops after its first declarations, so no behaviour is visible.
- `register_five_spaced_entries`: Makes five calls to one setup subroutine, each with an address spaced evenly after the last and the same fixed size and flag arguments, calling a follow-up subroutine after each.
- `queue_five_entries_mode_b`: Makes five calls to one setup subroutine with evenly spaced addresses and a different fixed pair of arguments from its neighbours, calling a follow-up subroutine after each.
- `register_four_entries_mode_c`: Makes four calls to one setup subroutine with evenly spaced addresses and a fixed argument pair, then calls a follow-up subroutine and a run of further subroutines.
- `stub_truncated_decompile_b`: The decompiled source stops after its first declarations, so no behaviour is visible.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately_stub`: Does nothing and returns to its caller.
- `advance_byte_cursor_until_end`: If one flag is clear, calls a subroutine and returns.
- `tiny_ori_2`: Small routine of 1 instruction that merges 236 into d0.
- `tiny_ori_3`: Small routine of 1 instruction that merges 56 into d0.
- `dispatch_keyed_handler_from_table`: Scans a table of 10-byte entries for one whose key byte matches a value in a register, stopping at a zero-key terminator or the first match.
- `dispatch_keyed_handler_alt_entry`: Scans a table of 10-byte entries for a matching key byte, stopping at a zero-key terminator or the first match, then calls the handler address stored in that entry.
- `dispatch_keyed_handler_variant_c`: Scans a table of 10-byte entries for a matching key byte, stopping at a zero-key terminator or the first match, then calls the handler address stored in that entry.
- `search_and_call_slot`: Iterates through a table in increments of ten bytes to find a matching byte and then calls a function pointer located at an offset within that entry.
- `search_and_call_slot_2`: Scans a data structure for a specific byte value and executes a subroutine found at a fixed offset from the match.
- `dispatch_keyed_handler_variant_f`: Scans a table of 10-byte entries for a matching key byte, stopping at a zero-key terminator or the first match, then calls the handler address stored in that entry.
- `search_and_call_slot_3`: Traverses a list of entries to locate a specific byte and invokes a function pointer stored within the identified entry.
- `dispatch_keyed_handler_variant_h`: Scans a table of 10-byte entries for a matching key byte, stopping at a zero-key terminator or the first match, then calls the handler address stored in that entry.
- `process_128_slot_flags`: Walks 128 slots and, for each whose flag byte is set, clears the flag, marks the slot's entry with 255 and calls a subroutine.
- `reset_slot_table_and_pointers`: Calls a subroutine, clears a 128-entry table, zeroes five handler pointers and two state bytes, then returns.
- `reset_slot_table_with_selector`: Calls a subroutine, selects one of two candidate handler addresses according to an argument, clears five handler pointers and a 128-entry table, then calls a further subroutine.
- `scan_eight_bytes_for_one`: Scans eight bytes of a table and returns early if any byte equals one.
- `scan_bytes_offset_two`: Scans eight bytes starting two bytes into a table and returns early if any byte equals one.
- `acquire_or_release_indexed_slot`: When its argument is nonzero and the slot is clear, increments each nonzero counter in an 8-entry table and marks the slot set.
- `acquire_or_release_byte_slot`: Works like its neighbour on an 8-byte table: nonzero argument increments the nonzero counters when the slot is clear and marks it set, zero argument decrements counters at or above the slot's value and marks it clear.
- `acquire_slot_and_bump_counters`: If an indexed slot is clear, increments each nonzero counter in a run of bytes before marking the slot set.
- `tiny_ori_4`: Small routine of 1 instruction that merges 58 into d1.
- `initialise_timed_records_table`: Reads a status value from the joystick and disk port, then fills twelve records from an offset-encoded table, computing each record's rate by dividing a fixed clock constant by a table value.
- `reset_disk_and_dma_control`: Clears a status byte and writes fixed values to the disk-control and DMA registers, finishing by masking one bit off the last value.
- `step_four_channel_bytecode`: Steps four channel records of 40 bytes each.
- `clear_flag_with_optional_helper`: Clears a state word and, when a register argument is nonzero, calls a subroutine and stores a second register value into the state word.
- `call_helper_if_state_set`: Calls a subroutine only if a state word is nonzero, then stores a routine address in a global and returns.
- `call_helper_then_clear_state`: Calls a subroutine, clears a state word, stores a routine address in a global, and returns a value register.
- `call_helper_with_routine_address`: Calls one subroutine with a routine address and a zero argument, then returns.
- `stub_truncated_decompile_c`: The decompiled text holds only a truncation marker, so the routine's behaviour cannot be recovered from this entry.
- `wait_for_vertical_position_cycle`: Busy-waits until a raster position value leaves a fixed value, then waits until it returns to that value.
- `call_helper_then_wait_position`: Calls one subroutine, then runs the same two-stage wait on the raster position value as its neighbour, returning once the value reaches the fixed value.
- `call_helper_routine_address_b`: Calls one subroutine with a routine address and a zero argument, then returns.
- `execute_stub_function`: This routine acts as a jump table entry or a small wrapper that calls another routine via a pointer or a truncated instruction sequence.
- `stub_truncated_decompile_e`: The decompiled text holds only a truncation marker, so the routine's behaviour cannot be recovered from this entry.
- `call_list_entries_return_last`: Walks a zero-terminated list of two-word entries, calling a subroutine for each one, and returns the last subroutine result.
- `call_list_return_last_b`: Walks a zero-terminated list of two-word entries, calling a subroutine for each one, and returns the last subroutine result.
- `return_word_value`: Returns a word value from a subroutine call within a loop.
- `traverse_pointer_table`: Iterates through a table of pointers stored in a global variable, calling another routine for each non-zero entry found in the list.
- `iterate_pointer_list`: Iterates through a null-terminated list of pointers stored in a global variable and calls another routine for each element.
- `clear_buffer_walk_byte_blocks`: Clears a 320-byte buffer, then walks a zero-terminated byte sequence in blocks of eight, calling a subroutine for each non-zero position in turn and stopping at a zero byte.
- `merge_shifted_record_into_rows`: Loads a 16-byte record into four shared longwords, shifting them right by a count or copying them unchanged when the count is zero.
- `setup_blitter_for_clipped_copy`: Checks that a horizontal position and width fit within a 320-pixel line, clipping the start when it is negative and returning minus one when the region is out of range.
- `setup_entry_after_checked_call`: Calls a subroutine and returns its failure value when that is minus one.
- `queue_blitter_request_fields`: Waits for the blitter to be idle, then stores a source address, either a default value or a supplied value depending on a flag bit, a second value and a size word into the request record.
- `return_byte_argument_unchanged`: Returns its byte argument unchanged.
- `decrement_counter_call_at_zero`: Decrements a counter stored in a record and calls a subroutine when it reaches zero.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `poll_state_and_set_flag`: When a state byte is clear, calls a subroutine and, if it reports a nonzero value, clears a flag and sets the state byte to one.
- `decrement_counter_then_compare_call`: Calls a subroutine and decrements a counter in a record.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `check_byte_below_nine_max`: When a flag is clear, calls a subroutine and returns if the byte argument is below nine.
- `run_entry_update_loop`: Calls an indirect routine from a record, then loops indefinitely over records: for each record whose state is not two it calls subroutines, with an extra subroutine when a flag bit is set.
- `copy_rectangle_between_regions`: Copies a rectangular block of words between two regions of a screen buffer.
- `compute_shape_bounds_from_records`: Initialises four bound values to sentinels, then walks a list of shape records, updating the minimum and maximum horizontal and vertical extents from each record and calling subroutines per record.
- `expand_bytes_through_lookup_table`: Fills a buffer with rows whose width and count come from a record, copying each source byte through a lookup table into the destination, then stores the end pointer back into the record.
- `lookup_scaled_offset_by_index`: For index zero, returns a record field shifted left by four.
- `randomise_offset_and_trigger_effect`: Stores an index and a counter from a record and, when a flag byte is set, draws two five-bit random offsets from a subroutine.
- `or_fixed_mask_into_value`: Returns its argument with a fixed bit pattern ORed in.
- `test_box_overlap_pairs`: Tests pairs of objects from two nested lists for overlap of their bounding boxes.
- `test_box_overlap_pairs_b`: Tests pairs of objects from two nested lists for overlap of their bounding boxes, calling subroutines chosen by type code where they overlap.
- `check_bounding_box_overlap`: Compares the dimensions and coordinates of two sets of bounding boxes to detect spatial intersections.
- `validate_object_collision_bounds`: Checks if a set of bounding box coordinates defined in a global structure are valid and then iterates through a list of objects to perform intersection tests.
- `check_bounding_box_overlap_2`: Compares the coordinates of a bounding box against a list of other bounding boxes to detect spatial intersections.
- `validate_object_collision_bounds_2`: Checks if a set of bounding boxes defined in a global structure overlaps with a target area.
- `check_bounding_box_overlap_3`: Compares a set of rectangular coordinates against a global bounding box to determine intersection.
- `check_bounding_box_overlap_4`: Compares a set of rectangular coordinates against a global bounding box to determine spatial intersection.
- `update_sprite_visibility_extents`: Checks if a bounding box defined in a global structure overlaps with a specified rectangular area.
- `check_bounding_box_overlap_5`: Compares spatial extents stored in a global structure against a list of objects to detect intersections.
- `validate_bounding_box_intersection`: Checks if a set of coordinates defined in a global structure falls within a specified rectangular boundary.
- `test_box_overlap_with_calls`: Tests pairs of objects from two nested lists for overlap of their bounding boxes.
- `set_object_byte_from_record`: Stores a byte in an object, computed from a record value shifted right by two, adding three when a global flag is clear.
- `compute_object_anchor_and_extents`: Derives an anchor address for an object from either a record offset or a table entry, then computes a position and two extents from record fields and stores them in shared globals.
- `compute_extents_call_if_visible`: Derives an anchor and extents for an object in the same way as its neighbour, then calls a subroutine only when the object's horizontal and vertical extents fall inside the supplied bounds.
- `scan_rows_for_bounding_overlap`: Scans a range of rows, testing on each row whether two stored extents overlap, and stops at the first row where they do.
- `walk_record_list_with_callback`: Clears shared state, then walks a list of records.
- `no_op_pointer_check`: Compares a pointer with two addresses, discards the result and returns.
- `call_helper_then_store_argument`: Calls a subroutine and, if a flag stays clear, zeroes a state word, stores a register argument in a global and calls a further subroutine.
- `load_record_list_into_context`: Walks a zero-terminated list of pointers to records, copying each record's fields into a context block, calling a subroutine and then processing that record's sub-list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_absolute_long_target`: Pushes the return address onto the stack and transfers execution to the absolute long subroutine target encoded in the instruction.
- `merge_changed_bits_into_accumulator`: Over a 128-byte block, ORs into an accumulator the bits of a new block that differ from the previously saved block, then saves the new block as the previous one.
- `clear_three_128_byte_tables`: Zeroes three 128-byte tables in turn, one longword at a time.
- `link_nineteen_nodes_in_sequence`: Chains nineteen entries together by storing in each the address 118 bytes beyond it, terminates the last with zero, then sets two head pointers to a fixed address.
- `xor_fold_words_masked`: XORs together a run of longwords from a buffer and returns the result masked to every other bit.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `return_immediately_stub_f`: Does nothing and returns to its caller.
- `set_handler_call_five_steps`: Stores a routine address in a shared global, then calls five subroutines in sequence.
- `step_counter_toward_target`: Steps a shared counter one unit at a time toward a target address, calling subroutines at each step, while a status bit stays set.
- `step_counter_on_status_poll`: Steps a shared counter toward a target address in a loop that re-checks a status bit each pass.
- `set_counter_to_minus_eight`: Sets a shared counter to minus eight and returns.
- `nop_padding_instruction`: This routine is empty and contains only a single no-operation instruction.
- `return_from_subroutine_3`: An empty routine that performs a return.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `return_without_operation`: This routine performs no actions and returns immediately to its caller.
- `start_disk_read_sync_word`: If a flag is set, records a timeout error and returns.
- `start_disk_read_polling_status`: Programs the same disk registers and sync word as its neighbour, then polls a status bit for up to 201 passes.
- `set_control_field_constant`: Writes a fixed constant into one field of a control record and returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `poll_flag_table_clear_b`: Its opening test is always true, so it returns without doing anything and the wait-and-clear logic after it is unreachable as written.
- `initialize_linked_tables`: Clears three 128-byte tables, chains nineteen nodes together in a sequence, and performs arithmetic operations on a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_3`: This routine performs no operations and returns immediately to its caller.
- `return_from_subroutine_4`: An empty routine that immediately returns to its caller.
- `perform_no_operation`: This routine is empty and performs no actions before returning.
