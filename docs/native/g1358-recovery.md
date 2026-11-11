# Game 1358 (dyna blaster (1991)(hudson soft)[cr vf]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01358). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911952 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | pass, 115 of 115 named (0 author symbols), 115 of 115 described (6 by rule, 36 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1537 of 1537 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 646272 bytes, 28 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2996 of 3015 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20293 |
| code | 8656 |
| data | 824167 |
| padding | 980 |
| unused | 57856 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dyna Blaster (1991)(Hudson Soft)[cr VF].adf:bootblock | 1 | 6 | 1 |
| logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF].adf boot-block load | 1 | 124 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 702 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1253 | 50 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF].adf bytes 1024-1536 (code never loaded by a reference run) | 1 | 147 | 8 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF].adf bytes 4096-4608 (code never loaded by a reference run) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF].adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown) | 1 | 783 | 20 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF].adf:blocks 242-289 (bytes 123904-148480)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF].adf:blocks 297-341 (bytes 152064-175104)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF].adf:blocks 352-458 (bytes 180224-235008)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: custom-format (non-DOS) disk, 1641 non-empty blocks: matched against track-loaded segments
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: the boot block reads 17408 bytes from disk offset 236544 to an address (found statically); recovered as a unit
- runtime segment an address: 664 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 873 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: bytes 1024-1536 look like code no run loaded: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: bytes 4096-4608 look like code no run loaded: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: 3 logged loads cover 18432 bytes of the image
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 4-4 (bytes 2048-2560) outside the filesystem hold data, not code (opcode density 0.020, entropy 4.42)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 11-235 (bytes 5632-120832) hold a IMP: File Imploder stream (115200 -> 144684 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf:blocks 11-235 (bytes 5632-120832)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 242-289 (bytes 123904-148480) hold a IMP: File Imploder stream (24576 -> 113460 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf:blocks 242-289 (bytes 123904-148480)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 297-341 (bytes 152064-175104) hold a IMP: File Imploder stream (23040 -> 209510 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf:blocks 297-341 (bytes 152064-175104)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 352-458 (bytes 180224-235008) hold a IMP: File Imploder stream (54784 -> 177594 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf:blocks 352-458 (bytes 180224-235008)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 517-525 (bytes 264704-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.47)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1067-1098 (bytes 546304-562688) outside the filesystem hold data, not code (opcode density 0.013, entropy 7.09)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1375-1405 (bytes 704000-719872) hold a IMP: File Imploder stream (15872 -> 12960 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1408-1480 (bytes 720896-758272) hold a IMP: File Imploder stream (37376 -> 39240 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1485-1535 (bytes 760320-786432) hold a IMP: File Imploder stream (26112 -> 60160 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1540-1557 (bytes 788480-797696) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1562-1580 (bytes 799744-809472) hold a IMP: File Imploder stream (9728 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1584-1604 (bytes 811008-821760) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1606-1626 (bytes 822272-833024) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1628-1647 (bytes 833536-843776) hold a IMP: File Imploder stream (10240 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1650-1667 (bytes 844800-854016) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1672-1686 (bytes 856064-863744) hold a IMP: File Imploder stream (7680 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1694-1710 (bytes 867328-876032) hold a IMP: File Imploder stream (8704 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf]/Dyna Blaster (1991)(Hudson Soft)[cr VF].adf: blocks 1716-1741 (bytes 878592-891904) hold a IMP: File Imploder stream (13312 -> 41760 bytes)
- 5 never-loaded region(s), 645760 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 413696 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `pulse_disk_control_port_line` (runtime segment an address): Waits on a pending condition, then pulses one line of the drive control port register low and back high with waits in between. Kind: disk hardware timing/pulse check. Pass path: the loop continues (extraout_D1w != 0) and proceeds to the subsequent data loading/execution logic.
- `pulse_disk_drive_control` (Dyna Blaster (1991)(Hudson Soft)[cr VF].adf bytes 1024-1536 (code never loaded by a reference run)): Writes and restores the drive control port of the disk interface in sequence, waiting between writes, and then records a global value. Kind: sabotage path / integrity check. Pass path: the loop executes u4__sub_013A and u4__sub_004E until extraout_D1w is 0.

## Functions


### Dyna Blaster (1991)(Hudson Soft)[cr VF].adf:bootblock

- `boot_block_setup_and_load`: Runs as the disk boot entry.

### logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF].adf boot-block load

- `select_half_or_input`: Returns the input word unchanged unless the flag is set to one, in which case it returns half of the second word.
- `mask_word_with_table_entry`: Looks up a word in a fixed table using twice the index held in its register and returns it ANDed with a second word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_interrupts_then_idle`: Stores an interrupt-related value and selects one of two interrupt enable masks according to a shared flag.
- `copy_tables_and_setup_blit`: Copies a block of table words to a second area, then copies two interleaved tables.
- `or_bitplanes_into_five_rows`: Ors five source words together at each position and writes the combined value into five destination rows spaced at a fixed word stride.
- `fill_copper_pointer_table`: Fills two four-entry tables with the high and low words of addresses that advance by a fixed step.
- `fade_palette_step_on_toggle`: Toggles a flag on each call, and on every second call increments a fade counter up to a limit.
- `fade_full_palette_on_toggle`: Toggles the same flag and, on every second call, scales a full table of thirty-two color values by the fade counter.
- `fade_colors_one_step_down`: Decrements a fade counter and, while it is nonzero, scales a run of 12-bit color values by the new counter value.
- `program_blit_from_source_pointer`: Waits for the blitter to be idle, then programs its control, mask, modulo and size registers with a fixed size.
- `program_fixed_blit_constants`: Waits for the blitter to be idle and programs its registers from fixed constants for a fixed-size transfer.
- `program_combined_blit_pair`: Waits for the blitter to be idle, then programs a pair of blit operations from fixed addresses and a fixed size.
- `advance_blit_script_entries`: Counts down a step counter and, when it expires, reads the next entry from a blit script table and stops at a terminator value.
- `advance_second_blit_script`: Advances a second blit script table in the same way as the first, counting down its own step counter.
- `advance_frame_counter_and_blit`: Advances a frame counter and toggles a shared bit every sixth tick, adding a fixed offset when a second counter reaches a fixed value.
- `program_blit_from_stored_address`: Waits for the blitter to be idle, then programs its control, mask, modulo and size registers using an address stored in a shared word.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `program_blitter_after_table_step`: Counts down a timer and, when it expires, reloads step values from a table and advances the table pointer.
- `cycle_four_entry_index_table`: Counts down a timer and, when it expires, advances a two-bit index that wraps after four entries.
- `clear_data_register_d0`: Performs a logical OR operation on register d0 with zero to clear its contents.
- `tiny_ori_2`: Small routine of 1 instruction that merges 154 into d0.
- `step_tables_and_program_blits`: Advances two countdown-driven value tables and calls the routine stored in the step pointer.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d1.
- `tiny_ori_4`: Small routine of 1 instruction that merges 0 into d1.
- `apply_offset_and_program_blit`: Ors a flag mask into a word reached through a pointer register and adds a stored offset to a second word.
- `or_flags_and_program_blit`: Ors a flag mask into a word reached through a pointer register and adds a delta to a second word.
- `advance_index_return_scaled_offset`: Counts down a timer and, when it expires, advances a two-bit index and reloads a table value and the timer.
- `advance_table_index_and_reload`: Counts down a timer and, when it expires, advances a two-bit index and reloads a table value and the timer.
- `clear_data_register_d1`: Clears the least significant byte of data register d1.
- `advance_channel_tables_and_blit`: Advances the channel table steppers and accumulated offsets when their countdowns expire, calls the stored routine, then waits for the blitter and programs two blit operations.
- `set_flag_bits_and_blit`: Sets a flag pattern in a word reached through a pointer register, waits until the blitter is idle, and programs blit registers from the accumulated offset values.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `advance_table_index_and_reload_2`: Decrements a 16-bit counter in a global variable and, if it reaches zero, increments a two-bit index, then loads a new timer value and a new table pointer/value from a table into global variables.
- `sample_cia_input_nibble`: Writes select values to a CIA port register and reads a four-bit input up to four times.
- `set_cia_port_select_bits`: Writes control values to a CIA port register, setting or clearing one select bit from the low bit of its input and writing the next input bit pair into two output bits.
- `acknowledge_audio_channel_interrupts`: Checks the interrupt request flags for two audio channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `disable_interrupts_and_loop_forever`: Writes the interrupt-enable and DMA-control registers to disable all interrupts and the four audio DMA channels, re-enabling the master interrupt and one audio interrupt when a shared flag is set.
- `wait_for_flag_then_spin`: Polls a shared byte until it holds the letter B and then enters an endless loop with no exit.
- `configure_hardware_and_call_subroutines`: Sets two 16-bit values at offsets 150 and 154 of a structure pointed to by a6, checks a byte at offset 6 of a6, and if certain conditions are met, performs several memory writes, calls two subroutines, and jumps to the address in a5.
- `step_counter_with_raster_waits`: Sets a shared byte to fifteen and a shared word to 255, then steps a counter up to seventeen.
- `pace_display_lines_with_countdown`: Repeats two routine calls per pass while decrementing a shared word until it drops below zero, then resets that word to zero.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `scale_color_entries_by_factor`: Multiplies each of thirty-two 12-bit color values by a shared factor, scaling the red, green and blue nibbles separately.
- `pulse_disk_control_port_line`: Waits on a pending condition, then pulses one line of the drive control port register low and back high with waits in between.
- `decode_disk_sector_mfm_data`: Starts a disk read using the MFM sync word and waits for the sync pattern and sector data.
- `seek_disk_head_to_track`: Sets or clears the direction bit in a drive control byte from the low bit of the requested value.
- `wait_for_disk_ready_handshake`: Waits while a status bit is set, clears a shared word and sets a control bit.
- `step_head_one_track_back`: Sets the direction bit of the drive control byte, pulses the step bit low and back high, and decrements a shared track counter.
- `step_head_one_track_forward`: Clears the direction bit of the drive control byte, pulses the step bit low and back high, and increments a shared track counter.
- `pulse_drive_control_pattern`: Writes a fixed control pattern to the drive control port, calls a routine, then writes a second pattern.
- `wait_for_beam_position_change`: Loops comparing a shared byte with its own increment, a test the decompiled form can never satisfy.
- `check_iff_form_then_process`: Compares the four-byte value at its pointer with the chunk identifier FORM. When it matches, it calls two routines to process the chunk; otherwise it returns without doing anything.
- `load_cmap_palette_entries`: Walks the file chunks until it finds the colour map chunk, then converts each three-byte red, green and blue triplet into a 12-bit colour value.
- `unpack_iff_body_chunk`: Finds the body chunk of an image and copies its data to the destination.
- `unpack_imploder_compressed_data`: Copies a fixed block of setup data into a low scratch area, then checks for the IMP! marker.
- `unpack_imploder_in_place`: Decompresses an IMP! packed block in place when its marker matches, reading backwards from the end of the packed data.
- `reload_audio_pointer_on_interrupt`: When the first audio channel's interrupt flag is set, it reloads that channel's location register from the beam position and sets a shared value.
- `initialize_four_channel_player_state`: Clears a block of player state, then sets initial values for four channel control entries.
- `run_four_channel_playback_tick`: Runs once per tick when a sequencing flag is clear.
- `reset_four_channel_player_state`: Clears the shared channel block and sets initial control values for four channels, as the other initializer does.
- `update_audio_channel_parameters`: Updates or resets audio channel parameters, including period, volume, and sample pointers, while handling row-boundary or note-end logic and updating the DMA controller or certain audio registers.
- `update_channel_sample_pointers`: Steps the sample playback position for each of four voices and reverses or wraps it at the end of the sample according to a loop mode.
- `load_voice_step_parameters`: Looks up the pitch period for the current pattern step from a table and copies the volume and an effect value into the voice state.
- `apply_voice_slide_and_volume`: Copies a stored counter into the voice when a pending flag is set, then adds the slide amount to the voice period.
- `parse_voice_pattern_commands`: Reads the next command bytes from a voice's pattern data and stores them as note or effect values.
- `trigger_voice_instrument_note`: When a note is triggered it clears the voice effect state, loads the instrument sample start, length and loop values from a table, and sets the initial volume and period.
- `clamp_and_write_voice_period`: Applies the current pitch slide to the voice period and clamps the result between a low and a high limit.
- `step_voice_slide_counter`: Steps a voice repeat counter toward a target value.
- `dispatch_voice_effect_by_nibble`: Runs the pattern effect for a voice once its tick counter reaches a threshold, selecting the effect from the low nibble of the current command.
- `slide_period_toward_target`: Computes a target pitch period for the voice and moves the current period toward it by the slide speed, stopping on arrival.
- `set_voice_period_from_table`: Looks up the pitch period for the voice current note in a table, using its offset and the note index, and stores the result as the voice period.
- `set_slide_down_amount`: Stores the negated slide speed as the voice downward slide value and adds it to the voice pitch step accumulator.
- `set_slide_up_amount`: Stores the slide speed as the voice upward slide value and adds it to the voice pitch step accumulator.
- `branch_on_byte_test`: Tests a byte at offset 4 of a0 and branches to either offset 1694 or 168a based on the result.
- `update_voice_volume_slide`: Adds a scaled upper nibble of the effect parameter to the voice volume when its low nibble is zero, otherwise subtracts a scaled value.
- `advance_sample_position_by_rate`: Stores a new playback rate when the effect parameter is nonzero, then advances the sample position by the rate.
- `reset_voice_on_effect_parameter`: When its effect parameter is nonzero, clears the voice counter and stores the parameter as a new value.
- `move_and_decrement_byte`: Copies a byte from offset 5 of a0 to offset 3 of a0, then decrements the byte at offset 3 of a0.
- `scale_and_store_value`: Multiplies a value from a table by four, caps it at 255, stores it in a structure, and then stores a scaled version of that value into a destination pointed to by a6.
- `set_voice_offset_from_parameter`: When the effect parameter is below a fixed limit, it stores the sum of a shared value and the parameter into the voice state byte.
- `step_voice_pitch_offset_table`: Advances an index through a per-voice table once a tick counter reaches its count, adding each table value to the voice period.
- `step_second_pitch_offset_table`: Advances a second per-voice table index in the same way as the other pitch table.
- `advance_sample_sequence_index`: Advances a per-voice index through a sample sequence.
- `scale_voice_volume_by_envelope`: Copies the voice volume counter into its output field and writes a volume value to the voice register.
- `run_voice_volume_envelope`: Runs a volume envelope state machine for a voice, stepping the volume through its phases with phase timing taken from the instrument data.
- `clear_shared_byte_if_zero`: Stores zero back into a shared byte only when it is already zero, so the shared value is never changed.

### Dyna Blaster (1991)(Hudson Soft)[cr VF].adf bytes 1024-1536 (code never loaded by a reference run)

- `pulse_disk_drive_control`: Writes and restores the drive control port of the disk interface in sequence, waiting between writes, and then records a global value.
- `decode_disk_sector_mfm_data_2`: Searches for a sync word in a memory buffer, verifies an XOR checksum over a block of data, and de-interleaves the even and odd bitstreams into a destination buffer if the checksum matches.
- `update_direction_and_seek_track`: Sets or clears bit 2 of a byte pointed to by a5 based on the lowest bit of d0, then calls another routine to decrement or increment a track counter stored at a4 until it matches the target track value derived from d0.
- `check_status_and_update_track`: Checks a bit in a register, and if set, calls another routine and branches back to itself; otherwise, it clears a word in a register, sets a bit in a byte, and branches to another instruction.
- `set_bit_and_call_subroutine`: Sets bit 1 of a byte in a register, calls another routine, clears bit 0 of that byte, calls another routine, sets bit 0 of that byte, decrements a register, and branches to another instruction.
- `clear_bit_and_call_subroutine`: Clears bit 1 of a byte in a register, calls another routine, clears bit 0 of that byte, calls another or another routine, sets bit 0 of that byte, increments a register, and branches to another instruction.
- `pulse_drive_control_pattern_2`: Writes a byte to a global variable, calls a routine, then writes a different byte to that same global variable; otherwise, it returns if a certain byte at offset six of a pointer in a6 is not equal to certain values.
- `wait_for_beam_position_change_2`: Saves the address register, then loops until a byte read from the visible or hidden or any part of the beam position register, when incremented by one, no longer equals its original value, then restores the address register and returns.

### Dyna Blaster (1991)(Hudson Soft)[cr VF].adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown)

- `initialize_object_dma_tables`: Clears a block of memory, populates several global variables with specific values, and performs pointer arithmetic to build a table of addresses.
- `update_dma_and_object_states`: Checks several global status bytes and, if conditions are met, updates a series of object structures by calling routines to initialize, load parameters, and clamp values.
- `advance_sample_loop_for_channels`: Walks three sound channels and advances each sample pointer by a step, handling one-shot, forward loop and ping-pong reversal modes.
- `load_channel_pattern_fields`: Loads a channel's pattern entry from a table indexed by a row byte, copying a word pointer and two byte parameters into the channel record and clearing a flag.
- `apply_channel_slide_step`: Adds a stored delta to a channel output word and advances an accumulator by a step each tick while it is nonzero, latching a new value from a global when a reset flag is set.
- `read_next_channel_command_byte`: Clears two channel counters, then either counts down a hold delay or reads the next command byte from a stream and stores its parameter bytes, storing a hold count when the byte is negative.
- `start_channel_note_from_table`: When a channel's trigger field is set, clears its effect state and loads sample header values from a table row, setting the initial volume, period and sample pointer from that row.
- `update_channel_period_with_clamp`: Runs several effect helper routines, adds a value from offset 58 of a structure in a2 to the value at offset 10 of the same structure, and clamps that value between 95 and 5760 before storing it at an offset determined by a value from offset 16 and a base pointer in a6.
- `step_repeat_counter`: If a byte at offset 12 of a structure pointed to by a pointer at offset 22 of a6 is non-zero, it compares a byte at offset 13 of that structure with a byte at offset 55 of a6; if they are equal, it adds the byte to a 16-bit value at offset 58 of a6 and increments the byte at offset 55, otherwise it increments the byte at offset 55.
- `update_entity_state_attributes`: Extracts a command index from a word at offset 50 of a structure in a2 and performs conditional operations based on that index.
- `slide_period_toward_note_target`: If a 16-bit value at offset 50 of a6 is between 1 and 111 and a 16-bit value at offset 52 of a6 is non-zero, it looks up a target period from a table using an index derived from offset 50, offset 71 of a6, and offset 14 of a4, then moves the 16-bit value at offset 10 of a6 toward that target by a step stored at offset 62 of a6.
- `set_pitch_from_command_table`: Stores a nibble from a word at offset 52 of a2 into a local table, uses the remaining nibble and a byte from offset 6 of a0 to index into that table, and writes the resulting value doubled and added to a word at offset 32 of a2 into offset 10 of the structure in a2.
- `negate_field_and_subtract`: Stores the negated value of one record field into a second field and subtracts that value from a third field.
- `adjust_counter_by_scaling`: If a 16-bit value at offset 18 of a6 is zero, it checks if a byte at offset 4 of a0 is zero and if a 16-bit value at offset 48 of a6 is non-zero; if so, it updates offset 13 of a6 using a byte from a structure at offset 22 of a6.
- `step_secondary_envelope_pitch`: Advances the secondary envelope counter: when it reaches the segment length it reloads from length minus step and adds a table value to the pitch, otherwise it increments the counter.
- `step_primary_envelope_pitch`: Advances the primary envelope counter using its own record fields, reloading the counter and adding a lookup-table value to the pitch when the segment ends, otherwise incrementing the counter.
- `copy_sequence_entry_to_buffer`: Advances a sequence counter and, on a frame boundary, fetches a 64-byte entry from a table, copies its long and word fields into the output buffer at a running offset, and records the entry index.
- `scale_position_into_buffer`: Calls a helper, copies a word from offset 12 of a structure in a2 to offset 80 of the same structure, and writes the high word of the product of that word and a word at offset 96 into a buffer at an offset determined by a6 and d4.
- `run_envelope_state_machine`: Advances the record's level envelope one step according to its state: a ramp up, a ramp down, a counted pause, or a timed hold, moving to the next state when each one finishes.
- `write_dma_control_registers`: If a word at a specific address is zero, it fetches a byte from a table and writes it to several hardware registers, including a write to the DMA control register to enable channels.
