# Game 1357 (dyna blaster (1991)(hudson soft)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01357). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911952 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | pass, 134 of 134 named (0 author symbols), 134 of 134 described (6 by rule, 54 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1537 of 1537 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 846272 bytes, 47 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4044 of 4120 instructions identical in start and length, 0 mismatches |

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
| Dyna Blaster (1991)(Hudson Soft).adf:bootblock | 1 | 6 | 1 |
| logged disk load from Dyna Blaster (1991)(Hudson Soft).adf boot-block load | 1 | 124 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 702 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1253 | 50 |
| Dyna Blaster (1991)(Hudson Soft).adf bytes 1024-1536 (code never loaded by a reference run) | 1 | 147 | 8 |
| Dyna Blaster (1991)(Hudson Soft).adf bytes 4096-4608 (code never loaded by a reference run) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft).adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown) | 1 | 783 | 20 |
| Dyna Blaster (1991)(Hudson Soft).adf:blocks 242-289 (bytes 123904-148480)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft).adf:blocks 297-341 (bytes 152064-175104)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft).adf:blocks 352-458 (bytes 180224-235008)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft).adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown) | 1 | 1105 | 19 |

## Cases handled

- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: custom-format (non-DOS) disk, 1641 non-empty blocks: matched against track-loaded segments
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: the boot block reads 17408 bytes from disk offset 236544 to an address (found statically); recovered as a unit
- runtime segment an address: 664 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 873 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: bytes 1024-1536 look like code no run loaded: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: bytes 4096-4608 look like code no run loaded: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: 3 logged loads cover 18432 bytes of the image
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 4-4 (bytes 2048-2560) outside the filesystem hold data, not code (opcode density 0.020, entropy 4.42)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 11-235 (bytes 5632-120832) hold a IMP: File Imploder stream (115200 -> 144684 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf:blocks 11-235 (bytes 5632-120832)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 242-289 (bytes 123904-148480) hold a IMP: File Imploder stream (24576 -> 113460 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf:blocks 242-289 (bytes 123904-148480)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 297-341 (bytes 152064-175104) hold a IMP: File Imploder stream (23040 -> 209510 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf:blocks 297-341 (bytes 152064-175104)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 352-458 (bytes 180224-235008) hold a IMP: File Imploder stream (54784 -> 177594 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf:blocks 352-458 (bytes 180224-235008)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 517-525 (bytes 264704-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.47)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1067-1098 (bytes 546304-562688) outside the filesystem hold data, not code (opcode density 0.013, entropy 7.09)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1100-1372 (bytes 563200-702976) hold a IMP: File Imploder stream (139776 -> 200000 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf:blocks 1100-1372 (bytes 563200-702976)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1375-1405 (bytes 704000-719872) hold a IMP: File Imploder stream (15872 -> 12960 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1408-1480 (bytes 720896-758272) hold a IMP: File Imploder stream (37376 -> 39240 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1485-1535 (bytes 760320-786432) hold a IMP: File Imploder stream (26112 -> 60160 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1540-1557 (bytes 788480-797696) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1562-1580 (bytes 799744-809472) hold a IMP: File Imploder stream (9728 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1584-1604 (bytes 811008-821760) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1606-1626 (bytes 822272-833024) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1628-1647 (bytes 833536-843776) hold a IMP: File Imploder stream (10240 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1650-1667 (bytes 844800-854016) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1672-1686 (bytes 856064-863744) hold a IMP: File Imploder stream (7680 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1694-1710 (bytes 867328-876032) hold a IMP: File Imploder stream (8704 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)/Dyna Blaster (1991)(Hudson Soft).adf: blocks 1716-1741 (bytes 878592-891904) hold a IMP: File Imploder stream (13312 -> 41760 bytes)
- 6 never-loaded region(s), 845760 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 273920 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Dyna Blaster (1991)(Hudson Soft).adf:bootblock

- `issue_request_and_reset_globals`: Fills in a request block with a command code, an address and two sizes, sends it through a library call, then calls a second routine and clears a series of global words.

### logged disk load from Dyna Blaster (1991)(Hudson Soft).adf boot-block load

- `shift_word_when_flag_set`: Returns the input word shifted right by one when the flag register equals one.
- `mask_word_with_table_entry`: Looks up a word in a fixed table, indexed by twice the index register, and ANDs it with the input word, returning the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_copper_list_and_dma`: Sets up the display: writes the copper list location and the DMA and interrupt control registers, chooses one of two display mode values from a flag, and makes fifteen helper calls.
- `copy_two_longword_blocks`: Copies two longword blocks, of 2560 and 9280 longwords, from fixed source areas to fixed destinations, then calls a helper.
- `or_combine_bitplane_words`: Combines five bitplane words spaced at fixed strides by ORing them together, and writes the result to the matching destination words across a block of rows and columns.
- `fill_copper_address_pairs`: Writes the high and low halves of two addresses into alternating entries of two lists, advancing the addresses by 32 bytes on each of four passes.
- `fade_palette_entries`: Toggles a phase flag and, on alternate phases, increases a fade counter up to 16 and rescales the 4-bit color components of each palette entry by it, storing the results in a palette list.
- `fade_second_palette_block`: Toggles a phase flag and, on alternate phases, rescales the 4-bit color components of 32 palette entries by the fade counter, storing the results in a second palette list.
- `fade_palette_row_by_factor`: When the fade counter is nonzero, decrements it and rescales the 4-bit color components of a run of palette entries by the new counter value, storing the results in a palette list.
- `start_blitter_copy_operation`: Waits until the blitter is idle, then programs its control, mask and size registers with fixed values and a source address, and starts the operation.
- `start_blitter_fill_operation`: Waits for the blitter to be idle, then programs fixed control, mask and size values and a fixed source address before starting the blit.
- `start_blitter_with_three_sources`: Waits for the blitter, then sets its control, mask and size registers together with several fixed source and destination addresses.
- `advance_sequence_table_entry`: Decrements a frame counter.
- `advance_second_sequence_entry`: Decrements a frame counter.
- `update_scroll_and_blit_frame`: Advances a tick counter and, every six ticks, toggles a bit pattern in a register.
- `start_blitter_with_offset_sources`: Waits for the blitter, then programs its control, mask and size registers and two source addresses that are offset from a base value.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `program_blitter_after_table_step`: Counts down a timer and, when it expires, reloads step values from a table and advances the table pointer.
- `cycle_four_entry_index_table`: Counts down a timer and, when it expires, advances a two-bit index that wraps after four entries.
- `discard_byte_value`: Contains a single instruction that reads and discards a byte from a register, and returns immediately.
- `tiny_ori_2`: Small routine of 1 instruction that merges 154 into d0.
- `step_tables_and_program_blits`: Advances two countdown-driven value tables and calls the routine stored in the step pointer.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d1.
- `tiny_ori_4`: Small routine of 1 instruction that merges 0 into d1.
- `apply_offset_and_program_blit`: Ors a flag mask into a word reached through a pointer register and adds a stored offset to a second word.
- `or_flags_and_program_blit`: Ors a flag mask into a word reached through a pointer register and adds a delta to a second word.
- `advance_index_return_scaled_offset`: Counts down a timer and, when it expires, advances a two-bit index and reloads a table value and the timer.
- `advance_table_index_and_reload`: Counts down a timer and, when it expires, advances a two-bit index and reloads a table value and the timer.
- `or_byte_zero_d1`: Performs a bitwise OR of the constant zero onto the byte register d1, which is a no-op that leaves d1 unchanged.
- `advance_channel_tables_and_blit`: Advances the channel table steppers and accumulated offsets when their countdowns expire, calls the stored routine, then waits for the blitter and programs two blit operations.
- `set_flag_bits_and_blit`: Sets a flag pattern in a word reached through a pointer register, waits until the blitter is idle, and programs blit registers from the accumulated offset values.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `increment_and_cycle_table_index`: Decrements a 16-bit global counter and, if it reaches zero, increments and masks a 16-bit index variable with 3, then updates two global variables by reading values from a table at an offset determined by the new index.
- `sample_cia_input_nibble`: Writes select values to a CIA port register and reads a four-bit input up to four times.
- `set_cia_port_select_bits`: Writes control values to a CIA port register, setting or clearing one select bit from the low bit of its input and writing the next input bit pair into two output bits.
- `acknowledge_audio_channel_interrupts`: Checks the interrupt request flags for two audio channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `disable_interrupts_and_loop_forever`: Writes the interrupt-enable and DMA-control registers to disable all interrupts and the four audio DMA channels, re-enabling the master interrupt and one audio interrupt when a shared flag is set.
- `wait_for_flag_then_spin`: Polls a shared byte until it holds the letter B and then enters an endless loop with no exit.
- `wait_for_flag_configure`: Polls a custom chip register and waits for a specific byte value, then configures DMA and interrupt registers, calls two other routines, and jumps to a routine address.
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
- `update_four_channel_audio_playback`: Manages a multi-step sequence of audio updates by iterating through four channels, applying volume and pitch slides, triggering instrument notes, and updating sample pointers.
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
- `test_byte_then_branch`: Tests a byte at a structure offset and branches away if it is nonzero, otherwise falls through to another routine, and does not return on its own.
- `update_voice_volume_slide`: Adds a scaled upper nibble of the effect parameter to the voice volume when its low nibble is zero, otherwise subtracts a scaled value.
- `advance_sample_position_by_rate`: Stores a new playback rate when the effect parameter is nonzero, then advances the sample position by the rate.
- `reset_voice_on_effect_parameter`: When its effect parameter is nonzero, clears the voice counter and stores the parameter as a new value.
- `copy_and_decrement_byte`: Copies a byte from one structure offset to another, decrements the destination byte, and returns.
- `scale_and_store_value`: Multiplies a value from a structure by four, caps it at 255, stores it back into the structure, then stores a scaled product into a destination pointed to by a register, and returns.
- `set_voice_offset_from_parameter`: When the effect parameter is below a fixed limit, it stores the sum of a shared value and the parameter into the voice state byte.
- `step_voice_pitch_offset_table`: Advances an index through a per-voice table once a tick counter reaches its count, adding each table value to the voice period.
- `step_second_pitch_offset_table`: Advances a second per-voice table index in the same way as the other pitch table.
- `advance_sample_sequence_index`: Advances a per-voice index through a sample sequence.
- `scale_voice_volume_by_envelope`: Copies the voice volume counter into its output field and writes a volume value to the voice register.
- `run_voice_volume_envelope`: Runs a volume envelope state machine for a voice, stepping the volume through its phases with phase timing taken from the instrument data.
- `clear_shared_byte_if_zero`: Stores zero back into a shared byte only when it is already zero, so the shared value is never changed.

### Dyna Blaster (1991)(Hudson Soft).adf bytes 1024-1536 (code never loaded by a reference run)

- `pulse_disk_drive_control`: Calls several routines including status and timing checks, pulses a port with values in a loop, and finally stores a constant in a global variable.
- `process_data_table`: Configures several memory locations, calls a timing routine, and then performs bitwise operations and data transfers from a table to a destination pointed to by a global variable.
- `sync_register_value`: Sets or clears bit 2 of a port based on bit 0 of a register, then enters a loop that calls another routine until the value at a pointer in a register matches a shifted version of the input register.
- `poll_disk_status`: Checks bit 4 of a register in a loop, calling another routine if the bit is set, otherwise it clears a 16-bit value in a register, sets bit 2 of a port, and branches to another routine.
- `update_port_bits`: Sets bit 1 of a port, calls another routine, clears bit 0 of the port, calls another routine, sets bit 0, decrements a 16-bit value in a register, and branches to another routine.
- `increment_port_bits`: Clears bit 1 of a port, calls another routine, clears bit 0 of the port, calls another routine, sets bit 0, increments a 16-bit value in a register, and branches to another routine.
- `toggle_port_byte`: Writes a byte to a port, calls another routine, and then writes a different byte to the same port before returning.
- `wait_for_vhpors_change`: Polls the value of the video hardware pointer register in a loop until it increments by one, then returns.

### Dyna Blaster (1991)(Hudson Soft).adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown)

- `initialize_object_dma_tables`: Clears a block of global variables and initializes several global tables and pointers, then iterates through a structure to call update_audio_dma_channels, load_pattern_entry_fields, and write_dma_control_registers for each entry, before performing a final loop to calculate and store cumulative offset values into a memory range.
- `update_audio_dma_channels`: Waits for the vertical blank, then decrements and resets several frame counters and, when their timers expire, loads new DMA channel pointers and writes them to the DMA control registers.
- `advance_sample_pointers_channels`: Loops over three records in a table, decrementing the byte at offset 26 and masking it by 31 when nonzero, otherwise copying a length from offset 14 and inverting a sample byte while advancing the pointer at offset 20 with wraparound for one-shot and ping-pong modes, then clears offset 26 of each record.
- `load_pattern_entry_fields`: Reads a row byte from offset 2 of the record in a2, looks up a word pointer and two byte parameters from tables indexed by that row, and stores them at offsets 64, 71 and 57 of the record, then clears the flag at offset 69.
- `apply_channel_slide_step`: Calls a helper to update a pointer, then when a reset flag is set clears two accumulator words, and otherwise adds a stored delta to a channel output word and advances an accumulator by a step each tick while it is nonzero.
- `read_channel_command_byte`: Clears the counters at offsets 46 and 50 of the record, then decrements the hold counter at offset 69 when set, otherwise reads a command byte from the stream at a1 and stores it with its parameter byte at offsets 51 and 53, or stores a negated byte at offset 69 when the command is negative.
- `start_channel_note_from_table`: Clears the effect state fields of the record when the trigger at offset 46 is set, then loads a sample header from a table row indexed by the trigger, copying the pointer and length into the record and computing the initial period from a lookup table.
- `update_channel_period_with_clamp`: Calls the eight effect helpers for one tick, adds the slide amount read at offset 58 to the channel period at offset 10, clamps the result to a floor of 95 and a ceiling of 5760, and stores the clamped value to the period output at offset 6 of the structure in a6 indexed by the repeat count saved at entry.
- `step_repeat_counter`: Reads a repeat count from offset 12 of the pointed-to record, and when it is nonzero increments the counter at offset 37 unless it already matches the value at offset 13, in which case it adds the count to the counter at offset 38.
- `dispatch_entity_command`: Reads a 16-bit value at offset 50 of the structure in a2 and returns immediately if it is below 112 or if its low nibble is below 5 while the byte at offset 4 of the structure in a0 is zero.
- `slide_period_toward_note_target`: Looks up a target period from a table using a note value at offset 50, then moves the period at offset 10 toward that target by the step stored at offset 62, clearing the step and snapping the period to the target when reached, and returns the current period.
- `load_table_index_value`: Builds a table index from the low nibble and high nibble of the word at offset 52 in a2, combines it with the byte at offset 6 in a0, then loads the value at offset 32 in a2 scaled by two into the word at offset 10 in a2.
- `negate_field_and_subtract`: Stores the negated value of one record field into a second field and subtracts that value from a third field.
- `adjust_counter_by_scaling`: When the field at offset 18 is zero and the byte at offset 4 of the record in a0 is zero, updates offset 13 from another record, then either adds a scaled value to or subtracts a scaled value from the counter at offset 12 depending on a bitmask, clamping the result.
- `step_secondary_envelope_pitch`: Advances the secondary envelope counter at offset 43 by comparing it to a segment length, reloading it from the length minus the step and adding a table value to the pitch at offset 10 when the segment ends, otherwise incrementing the counter.
- `step_primary_envelope_pitch`: Advances the primary envelope counter at offset 39 using its own record fields, reloading it and adding a lookup-table value to the pitch at offset 10 when the segment ends, otherwise incrementing the counter.
- `copy_sequence_entry_to_buffer`: Advances the sequence counter at offset 35, and when the segment ends fetches a byte from a table, negating the counter when the byte is negative, otherwise copying the long and word fields of a 64-byte entry from the table into the output buffer at a running offset and recording the entry index.
- `scale_position_into_buffer`: Calls one helper, copies a position word into a second field, and writes the high half of its product with a scale field into an output buffer at a running offset.
- `advance_envelope_state`: Reads two bytes from a record and, depending on the state value at offset 18, either adds or subtracts a delta from a value at offset 12 while clamping it to a byte from the record, or adjusts a counter at offset 20, then decrements the state counter and returns.
- `write_dma_control_registers`: Checks a flag word and, when clear, clears it, then indexes a table using a scaled value in d0 to fetch a byte that is written to several global variables before setting bit 15 of the DMA control register to enable DMA channels.

### Dyna Blaster (1991)(Hudson Soft).adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown)

- `init_system_and_parse_control_table`: Disables interrupts, sets DMA and interrupt masks, stores two longwords into custom registers, calls three helper routines, then branches based on a global flag before zeroing a block and processing a control table that dispatches to other routines depending on each byte's value.
- `compute_double_offset`: Divides the value in d0 by 264, scales the quotient by 128, and adds it to a base address in a5, then adds a masked and shifted portion of d0 to the same base address, and returns.
- `clear_memory_blocks`: Calls a helper routine, then uses a loop to fill two memory blocks starting at offsets of a5 and a5 plus 8400 with zeros.
- `update_table_values`: Calculates a value by reading from a table using an offset derived from a byte at a specific offset in a structure, then performs a bitwise OR to store this value into two 32-bit words at offsets 0 and 8400 of a destination structure in a5 through a loop.
- `fill_table_incrementing`: Reads bytes from a source table until it encounters a byte of all ones, storing an incrementing byte value at each computed destination address, then returns.
- `setup_cia_and_custom`: Writes a byte to a CIA port register, sets several custom chip registers, and stores a computed word at a global variable, then returns.
- `generate_lookup_tables`: Fills a table starting at offset 192 with values calculated via division, populates a second table starting at offset 8116 with bit-rotated bytes, and populates a third table starting at offset 8772 with incrementing 16-bit values.
- `compute_feedback_and_scale`: Computes a polynomial feedback value by repeatedly doubling and conditionally XORing a global constant, then returns either that value or a scaled product of the input, and later branches into code that updates tables.
- `find_block_index_and_clear_audio`: Finds the smallest byte greater than the running minimum in the table at 952 bytes after the pointer stored at offset a fixed address, then uses that byte as an index to zero-fill and populate a table of longwords, clears several structure fields and custom-chip registers, enables DMA, and returns.
- `init_audio_and_load_block_entry`: Initializes the four audio channels by loading their volume tables and then loads a block entry from a table using an index in the structure in a6, copying its fields into offsets 4 through 40 of that structure.
- `load_and_apply_block_entry`: Loads a block entry from a table using an index in the structure in a6, copies its fields into the structure at offsets 4 through 40, and writes the resulting position values into the structure in a5.
- `dispatch_block_byte_values_2`: Reads the low nibble of the byte at offset 2 of the structure in a6 and dispatches on it, using offset 9 of that structure to index a table of 72-bit entries and offset 8 as a 16-bit value, while writing a 16-bit result to offset 6 of the structure in a5.
- `dispatch_block_byte_values_3`: Copies the 16-bit value at offset 16 of the structure in a6 to offset 6 of the structure in a5 and returns.
- `search_clamp_table`: Searches a table for the first entry greater than the value at offset 0 in a6, adjusts the index by the byte at offset 9, stores the found value at offset 24, and sets the byte at offset 11 to 1 when the value is smaller.
- `clamp_and_search_table`: Clamps the 16-bit value at offset 16 of the structure in a6 toward the value at offset 24, then reads byte 31 and the low nibble of byte 18, multiplies it by 72, and scans a table for the first entry greater than the clamped value, advancing by 2 each step until reaching 72, then stores that index in the structure in a5.
- `apply_signed_delta_value`: Computes a signed value from a byte and a table lookup, applies its sign to a value at offset 16 of a structure in a5, and adds a scaled nibble to a counter at offset 27 before returning.
- `dispatch_block_byte_values`: Reads the low nibble of the byte at offset 2 in the structure in a6 and dispatches on its value, with 9 scaling a value in a5, 12 writing a nibble into a byte in a5, and 13 returning zero or the byte at offset 3.
- `store_low_nibble_byte`: Extracts the low nibble of byte 3 in the structure in a6 and stores it at offset 18, then returns.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
