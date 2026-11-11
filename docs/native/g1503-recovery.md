# Game 1503 (dyna blaster (1991)(hudson soft)[cr vf][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01503). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911952 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | pass, 134 of 134 named (0 author symbols), 134 of 134 described (9 by rule, 41 by the local model) |
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
| Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:bootblock | 1 | 6 | 1 |
| logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf boot-block load | 1 | 124 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 702 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1253 | 50 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf bytes 1024-1536 (code never loaded by a reference run) | 1 | 147 | 8 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf bytes 4096-4608 (code never loaded by a reference run) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown) | 1 | 783 | 20 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 242-289 (bytes 123904-148480)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 297-341 (bytes 152064-175104)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 352-458 (bytes 180224-235008)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown) | 1 | 1105 | 19 |

## Cases handled

- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: custom-format (non-DOS) disk, 1641 non-empty blocks: matched against track-loaded segments
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: the boot block reads 17408 bytes from disk offset 236544 to an address (found statically); recovered as a unit
- runtime segment an address: 664 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 873 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: bytes 1024-1536 look like code no run loaded: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: bytes 4096-4608 look like code no run loaded: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: 3 logged loads cover 18432 bytes of the image
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 4-4 (bytes 2048-2560) outside the filesystem hold data, not code (opcode density 0.020, entropy 4.42)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 11-235 (bytes 5632-120832) hold a IMP: File Imploder stream (115200 -> 144684 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 11-235 (bytes 5632-120832)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 242-289 (bytes 123904-148480) hold a IMP: File Imploder stream (24576 -> 113460 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 242-289 (bytes 123904-148480)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 297-341 (bytes 152064-175104) hold a IMP: File Imploder stream (23040 -> 209510 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 297-341 (bytes 152064-175104)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 352-458 (bytes 180224-235008) hold a IMP: File Imploder stream (54784 -> 177594 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 352-458 (bytes 180224-235008)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 517-525 (bytes 264704-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.47)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1067-1098 (bytes 546304-562688) outside the filesystem hold data, not code (opcode density 0.013, entropy 7.09)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1100-1372 (bytes 563200-702976) hold a IMP: File Imploder stream (139776 -> 200000 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1375-1405 (bytes 704000-719872) hold a IMP: File Imploder stream (15872 -> 12960 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1408-1480 (bytes 720896-758272) hold a IMP: File Imploder stream (37376 -> 39240 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1485-1535 (bytes 760320-786432) hold a IMP: File Imploder stream (26112 -> 60160 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1540-1557 (bytes 788480-797696) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1562-1580 (bytes 799744-809472) hold a IMP: File Imploder stream (9728 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1584-1604 (bytes 811008-821760) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1606-1626 (bytes 822272-833024) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1628-1647 (bytes 833536-843776) hold a IMP: File Imploder stream (10240 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1650-1667 (bytes 844800-854016) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1672-1686 (bytes 856064-863744) hold a IMP: File Imploder stream (7680 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1694-1710 (bytes 867328-876032) hold a IMP: File Imploder stream (8704 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf: blocks 1716-1741 (bytes 878592-891904) hold a IMP: File Imploder stream (13312 -> 41760 bytes)
- 6 never-loaded region(s), 845760 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 273920 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `pulse_drive_select_lines` (runtime segment an address): Waits through helper calls until a counter reaches zero, pulses a drive control port between two values with helper calls in between, and records a fixed value in a global. Kind: disk hardware/timing check. Pass path: the routine continues execution normally if the hardware pulse/timing matches expected values.
- `pulse_disk_drive_control` (Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf bytes 1024-1536 (code never loaded by a reference run)): Writes and restores the drive control port of the disk interface in sequence, waiting between writes, and then records a global value. Kind: disk hardware/timing check. Pass path: the loop continues and executes the subsequent u4__sub_01C6 calls.

## Functions


### Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:bootblock

- `decode_bit_stream_to_memory`: Reads a block from disk with an operating system I/O request, then runs a bit-driven loop that decodes lengths and distances and copies earlier output bytes backward.

### logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf boot-block load

- `select_half_or_input`: Returns half of one word when a flag equals one, otherwise returns the input word unchanged.
- `mask_table_word_by_index`: Looks up a word in a fixed table using a doubled index register and returns it ANDed with a mask register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_display_and_idle_loop`: Sets up interrupt and DMA control through a sequence of custom register writes and helper calls, then waits on a status value in an endless loop.
- `copy_tables_and_init_record`: Copies two large longword tables to fixed destinations, waits for a status bit to clear, and writes a set of initial values into a record.
- `or_combine_bitplane_rows`: Combines five source bitplane rows with a bitwise OR over two rectangular regions and writes the combined word into five destination bitplanes at fixed row offsets.
- `write_split_pointer_entries`: Writes four pairs of split high and low address words into two pointer tables, advancing each source address by a fixed step.
- `fade_palette_entries_by_step`: Toggles a parity flag and, on every second call, advances a fade counter up to sixteen and writes eight colour words, each nibble multiplied by the counter and divided by sixteen.
- `fade_wide_palette_by_step`: On every second call, scales thirty-two stored colour words by the fade counter and writes the nibble-scaled results to an output table.
- `scale_palette_block_down`: When the fade counter is nonzero it decrements the counter and writes a block of colour words, each nibble multiplied by the counter and divided by sixteen.
- `program_blitter_registers`: Waits while the blitter busy bit is set, writes its control, mask and address registers from fixed values and one argument, and sets the blit size.
- `program_blitter_secondary_setup`: Waits for the blitter to be idle, writes its control, mask and address registers from a different set of fixed constants, and starts a block of a given size.
- `program_blitter_modulo_setup`: Waits for the blitter to be idle, programs its control, mask, modulo and address registers from fixed constants, and starts a block operation.
- `advance_blit_script_entry`: Counts down a delay and, when it expires, reads the next three-part entry from a command table and programs the blitter with its values.
- `advance_blit_script_entry_b`: Runs the same table-driven blit scheduling as its neighbour for a second table, with its own delay counter and entries.
- `animate_offsets_and_blit`: Advances a tick counter, toggles a phase value every sixth tick, steps a second value at intervals, and programs blitter registers from offsets computed off the phase.
- `reprogram_blit_from_offset`: Waits for the blitter to be idle, then reprograms its address, modulo and size registers from a stored offset and fixed constants.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `step_table_driven_blit_pair`: Counts down a delay and, on expiry, loads the next table values for a phase and step.
- `cycle_four_entry_table_index`: Decrements a countdown and, when it expires, advances a four-entry index modulo four and reloads two table words from that entry.
- `advance_blit_script_channel`: Advances one table-driven script channel: when its delay expires it loads the next table entry, updates two accumulators, and programs blitter registers from them.
- `tiny_ori_2`: Small routine of 1 instruction that merges 154 into d0.
- `advance_blit_script_channel_b`: Runs the same script step as its neighbour with a different helper call sequence before programming the blitter.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d1.
- `tiny_ori_4`: Small routine of 1 instruction that merges 0 into d1.
- `program_blit_with_or_mask`: Ors a fixed mask into a word, adds an offset to a neighbouring word, programs the blitter from two accumulators, and sets the blit size.
- `program_masked_blit_b`: Performs the same masked blit setup as its neighbour as a separate copy.
- `advance_quad_index_table_word`: Decrements a countdown and, on expiry, advances a modulo-four index and loads a table word and count from that entry, returning the index.
- `advance_quad_index_table_b`: Performs the same quad-index table step as its neighbour without returning the index.
- `advance_blit_script_channel_c`: Runs the table-driven script step for a third channel, loading the next entry when its delay expires and updating accumulators for blitter programming.
- `advance_blit_script_channel_d`: Runs the table-driven script step for a fourth channel, equivalent to its neighbour with its own table offsets.
- `program_blit_from_accumulators`: Ors a constant into a word, then programs the blitter from two accumulators and sets the blit size.
- `cycle_quad_pointer_word`: Decrements a countdown and, on expiry, steps a modulo-four index and loads a pointer word from a table for that index.
- `advance_quad_pointer_table_b`: Performs the same quad-index pointer step as its neighbour without returning a value.
- `read_debounced_port_nibble`: Reads a CIA port several times until the input is stable, then looks up a six-bit value in a table using the low four bits of the reading and returns it with a flag bit.
- `write_cia_port_bits`: Sets a control bit and a two-bit selector in a CIA port register from its argument, then returns the inverse of the low six bits.
- `acknowledge_pending_interrupt_bits`: Reads the interrupt request register and, when certain pending bits are set, updates a control value and calls a helper, then writes the handled bits back to clear them.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_main_display_loop`: Loops forever, writing custom chip registers for DMA, interrupt and display setup on each pass, calling helper routines between the writes.
- `wait_for_status_then_halt`: Busy-waits until a memory-mapped status byte reads as the letter B, then spins forever with no further work.
- `halt_after_status_wait_b`: The decompiled body is the same wait-for-B loop as its neighbour.
- `step_raster_rows_to_seventeen`: Sets two control values, then loops over rows from a starting counter up to seventeen, calling two helpers on each pass.
- `count_rows_down_to_zero`: Calls two helpers on each step while a row counter counts down to zero, then clears the counter.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `scale_thirty_two_colour_words`: Scales thirty-two colour words by a counter value, multiplying each nibble by the counter and dividing by sixteen, and writes the results to a block.
- `pulse_drive_select_lines`: Waits through helper calls until a counter reaches zero, pulses a drive control port between two values with helper calls in between, and records a fixed value in a global.
- `decode_mfm_track_sectors_b`: Performs the same MFM track decode as the boot track reader: it sets DMA parameters, searches for the sync marker, verifies checksums, and copies sector data up to a limit.
- `step_head_target_track_b`: Steps a track counter toward a target and sets a control bit from the low bit of an argument, calling a helper on each pass.
- `clear_counter_await_status_b`: Waits while a global busy bit is set, zeroes a counter word, sets a control bit, then waits for two successive status bytes to read as expected.
- `step_track_inward_wait_b`: Pulses the step line on and off around a helper call, decrements a track counter word, and then waits for two successive status bytes.
- `step_track_outward_wait_b`: Clears and restores the step and direction control bits around a helper call, increments a track counter word, and then waits for two successive status bytes.
- `pulse_control_byte_bits_b`: Writes one control byte value, waits through a helper call, then writes a second control byte value.
- `wait_for_raster_position_b`: Busy-waits on a byte in a loop whose exit test compares it with its own increment, so the decompiled loop does not finish under normal arithmetic.
- `check_form_tag_and_process`: Compares the first longword at an address with the four-letter tag FORM and, when it matches, calls two helpers.
- `unpack_cmap_palette_words`: Searches a chunk list for the four-letter tag CMAP, then expands its packed three-byte colour entries into twelve-bit colour words in the palette area.
- `unpack_body_chunk_rle`: Finds the BODY chunk and copies its data to the output buffer, either as raw bytes or as run-length data where a negative count repeats one byte and a positive count copies literal bytes, depending on a header flag.
- `decode_imp_packed_stream`: Copies a header block, checks for the IMP! tag, and runs a bit-driven decoder that reads length and distance codes and copies earlier output bytes backward.
- `decode_imp_packed_in_place`: Decodes an IMP! packed block in place using the same bit-driven back-reference scheme as its neighbour, reading header fields from the buffer itself.
- `clear_interrupt_request_bits`: When one interrupt request bit is set, updates a pointer register from a computed value and sets a control global, then writes the interrupt request register back to clear the handled bits.
- `init_sound_channel_tables`: Clears a table of about one hundred longwords, fills its channel fields with fixed values and a per-channel increment derived from two source values, and sets the initial pointers and state before calling a helper.
- `tick_sound_playback_frame`: Runs once per playback tick: advances a three-step counter, and at a pattern boundary resets sequence state and steps a row pointer.
- `init_sound_channel_tables_b`: Performs the same table clearing and channel initialisation as its neighbour, then runs its own additional setup steps.
- `tick_sound_playback_frame_b`: Performs the same per-tick playback update as its neighbour, with a three-step counter, row boundary handling and audio register writes.
- `advance_sample_loop_for_channels`: Walks three sound channels and advances each sample pointer by a step, handling one-shot, forward loop and ping-pong reversal modes.
- `load_channel_pattern_fields`: Loads a channel's pattern entry from a table indexed by a row byte, copying a word pointer and two byte parameters into the channel record and clearing a flag.
- `apply_channel_slide_step`: Adds a stored delta to a channel output word and advances an accumulator by a step each tick while it is nonzero, latching a new value from a global when a reset flag is set.
- `read_next_channel_command_byte`: Clears two channel counters, then either counts down a hold delay or reads the next command byte from a stream and stores its parameter bytes, storing a hold count when the byte is negative.
- `start_channel_note_from_table`: When a channel's trigger field is set, clears its effect state and loads sample header values from a table row, setting the initial volume, period and sample pointer from that row.
- `update_channel_period_with_clamp`: Runs the effect helpers for one tick, adds the slide amount to the channel period, clamps it to a floor of 95 and a ceiling of 5760, and stores the result to the period output.
- `count_repeat_effect_steps`: Counts repeats of a channel effect nibble, advancing a step counter until the stored count matches, then adds the effect value to a slide accumulator.
- `dispatch_sample_effect_command`: Dispatches on the low nibble of an effect word and returns early for small values.
- `slide_period_toward_note_target`: Looks up a target period from a table and moves the channel period toward it by a stored step, snapping to the target when reached, and returns the current period.
- `lookup_period_from_table_index`: Splits a parameter word into two nibbles and uses them to look up a base period from a table, storing the result in a channel field.
- `negate_field_and_subtract`: Stores the negated value of one record field into a second field and subtracts that value from a third field.
- `copy_value_and_accumulate`: Copies one record field into another and adds the same value to a third field.
- `branch_on_byte`: Tests a byte at offset 4 of a0 and branches to either the start or the end of the routine based on the result.
- `ramp_level_from_control`: Adjusts a level counter from a control word: when the low nibble is zero it adds four times the high nibble capped at 255, otherwise it subtracts four times the control value floored at zero, and mirrors the result into a second field.
- `advance_position_by_speed`: Latches a new speed when the control field is nonzero, then moves a 32-bit position by the speed scaled by 256 and reduces a fractional field by the speed scaled by 128.
- `latch_speed_and_clear_flag`: When the control field is nonzero, clears a long field, copies the control value into a speed field, and clears the bits of a flag word selected by a mask field.
- `decrement_byte_at_offset`: Copies a byte from offset 5 of a0 to offset 3 of a0, then decrements the byte at offset 3.
- `scale_and_store_value`: Multiplies a value from a table by four, caps it at 255, stores it in a structure, and then stores a scaled version of that value into a destination pointed to by a6.
- `store_global_offset_if_small`: If a byte field is below a fixed limit, stores the sum of a global byte and that field into a byte of the linked record.
- `step_frame_with_table_delta`: Advances a per-object frame counter; when it reaches the record limit it resets from the record value, steps the frame index, and adds a signed delta from a lookup table to a position field.
- `step_frame_and_lookup_word`: Steps a frame counter like its neighbour, but on a frame advance it loads a 16-bit value from a fixed data table indexed by a table-driven offset and stores it in a field.
- `copy_sequence_entry_to_buffer`: Advances a sequence counter and, on a frame boundary, fetches a 64-byte entry from a table, copies its long and word fields into the output buffer at a running offset, and records the entry index.
- `scale_position_into_buffer`: Calls one helper, copies a position word into a second field, and writes the high half of its product with a scale field into an output buffer at a running offset.
- `run_timed_ramp_state_machine`: Runs a four-state sequencer on a position field: idle, a rate ramp toward a cap, a ramp down by a step, and a counter wait toward a target, moving to the next state when each finishes.
- `clear_global_flag_if_zero`: Writes zero back to a global variable only when it already holds zero, so the routine has no visible effect.

### Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf bytes 1024-1536 (code never loaded by a reference run)

- `pulse_disk_drive_control`: Writes and restores the drive control port of the disk interface in sequence, waiting between writes, and then records a global value.
- `decode_amiga_track_sectors`: Waits for the MFM sync marker in the raw disk DMA buffer, decodes the odd and even bit-interleaved sector data, checks an XOR checksum, and copies each valid sector of 512 bytes to memory up to a limit.
- `step_head_to_target_track`: Sets or clears one drive control bit from the low bit of a value, then steps a track counter word toward a target by calling a helper until the two match.
- `frag_btst_beq_bsr_bra_clr_bset`: Short code fragment that tests bit 4 of a fixed memory address, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then clears the memory at a4, then sets bit 2 of the memory at a5, then jumps to another point in the code.
- `frag_bset_bsr_bclr_subq_bra`: Short code fragment that sets bit 1 of the memory at a5, then calls another routine, then clears bit 0 of the memory at a5, then calls wait_for_raster_position, then sets bit 0 of the memory at a5, then subtracts 1 from the memory at a4, then jumps to another point in the code.
- `frag_bclr_bsr_bset_addq_bra`: Short code fragment that clears bit 1 of the memory at a5, then calls another routine, then clears bit 0 of the memory at a5, then calls wait_for_raster_position, then sets bit 0 of the memory at a5, then adds 1 to the memory at a4, then jumps to another point in the code.
- `frag_move_bsr_rts_cmpi_bne`: Short code fragment that copies 127 to the memory at a5, then calls wait_for_raster_position, then copies 119 to the memory at a5, then returns, then compares the memory at an offset from a6 with 66, then branches if the last result was not zero, then compares the memory at an offset from a6 with 65, then branches if the last result was not zero, then returns.
- `wait_for_raster_position`: Busy-waits on a global byte in a loop whose exit test compares a byte with its own increment, so it does not finish under normal arithmetic.

### Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown)

- `initialize_object_dma_tables`: Clears a block of memory, populates several global variables with specific values, and performs pointer arithmetic to build a table of addresses.
- `update_dma_and_object_states`: Checks several global status bytes and, if conditions are met, updates a series of object structures by calling routines to initialize, load parameters, and clamp values.
- `update_object_states`: Iterates through a list of pointers to update various fields in structures, including bitwise inversion of bytes and managing counters and pointer offsets.
- `load_object_parameters`: Reads a byte from a0, uses it to index into tables to retrieve values, and stores a calculated pointer and several bytes into a structure pointed to by a2.
- `process_buffer_and_counter`: Calls another routine to update a pointer in a structure, then if a specific word is non-zero, updates a 32-bit value and adds a value from offset 14 of the structure to a destination pointed to by a5.
- `process_input_stream`: Decrements a counter in a structure and, if zero, reads bytes from a source buffer to populate specific offsets in a destination structure based on the sign and value of the input bytes.
- `initialize_structure_and_fetch_data`: Clears several fields in a structure pointed to by a2, sets a value at offset 18, and then performs a series of table lookups to update offsets 22, 32, 73, 26, 30, 76, and 10 using data from various tables.
- `clamp_and_update_value`: Calls seven different routines, adds a 16-bit word from offset 58 of a structure in a2 to a 16-bit word at offset 10 of the same structure, and clamps that value between 95 and 5856 before storing it at an offset determined by a value from offset 16 of a2 in a structure pointed to by a6.
- `step_repeat_counter`: While a table count is non-zero, it increments the record's counter until it reaches the table's limit, then adds the table count to the record's offset field.
- `update_entity_state_attributes`: Extracts a command index from a word at offset 50 of the structure in a2 and performs a series of conditional operations based on that index.
- `update_offset_with_limit`: If a value at offset 50 is within a specific range and offset 52 is non-zero, it calculates a new value from a table and updates offset 60; it then adjusts offset 10 based on the difference between offset 60 and the current value.
- `set_pitch_from_command_table`: Computes a table index from a command word and a byte in the caller's data, then loads the pitch value at that index into the record.
- `adjust_structure_offsets`: Negates a 16-bit value from offset 52 of a structure, stores it in offset 58, and adds the result to the value at offset 76.
- `adjust_counter_by_scaling`: If certain conditions are met, it updates offset 13, then either subtracts a scaled value from offset 12 or adds a scaled value to offset 12 based on a bitmask check.
- `step_secondary_envelope_pitch`: Advances the secondary envelope counter: when it reaches the segment length it reloads from length minus step and adds a table value to the pitch, otherwise it increments the counter.
- `step_primary_envelope_pitch`: Advances the primary envelope counter using its own record fields, reloading the counter and adding a lookup-table value to the pitch when the segment ends, otherwise incrementing the counter.
- `update_envelope_and_copy_data`: Increments a counter at offset 35 if a byte at offset 1 of a sub-structure matches a value; if the counter reaches a limit, it fetches data from a table and copies it to a destination pointed to by a6.
- `scale_envelope_value`: Copies a 16-bit value from offset 12 of a structure to offset 80, then calculates a product of that value and a value from offset 96, storing the high word of the result in a table at an offset determined by d4.
- `run_envelope_state_machine`: Advances the record's level envelope one step according to its state: a ramp up, a ramp down, a counted pause, or a timed hold, moving to the next state when each one finishes.
- `write_dma_control_registers`: If a word at a specific address is zero, it fetches a byte from a table and writes it to several hardware registers, including a write to the DMA control register to enable channels.

### Dyna Blaster (1991)(Hudson Soft)[cr VF][a].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown)

- `initialize_system_parameters`: Sets hardware registers, calls three different routines, clears a memory block via a loop, and then processes a data stream to update various global variables based on command bytes.
- `calculate_table_offset`: Calculates two different offsets based on the input in d0 and adds them to a base address in a5, then returns.
- `clear_memory_blocks`: Calls another routine, then uses a loop to fill two memory blocks starting at offsets of a5 and a5 plus 8400 with zeros.
- `update_table_values`: Calculates a value by reading from a table using an offset derived from a byte at a specific offset in a structure, then performs a bitwise OR to store this value into two 32-bit words at offsets 0 and 8400 of a destination structure in a5 through a loop.
- `fill_table_from_source`: Copies bytes from a source table to a destination table, incrementing a byte value for each entry until a byte of all ones is encountered.
- `initialize_hardware_and_tables`: Sets a CIA register, configures several hardware registers, and populates a table with specific values derived from a calculation involving a nibble-based bit manipulation.
- `generate_lookup_tables`: Fills a table starting at offset 192 with values calculated via division, populates a second table starting at offset 8116 with bit-rotated bytes, and populates a third table starting at offset 8772 with incrementing 16-bit values.
- `perform_bit_manipulation_and_scaling`: Performs a loop of bitwise XOR operations and shifts on a value, then returns a result that is either a bitwise mask or a scaled product of the input.
- `reset_audio_and_tables`: Iterates through a table to clear a series of memory locations, then sets several hardware audio registers and a CIA register to specific values.
- `reset_and_update_audio_channels`: Resets the four hardware audio channel registers by calling another routine for each channel, then calculates a pitch value from a table and a global variable before clearing a 16-bit value in a global variable and calling another routine to update all four audio channels again.
- `update_audio_playback_parameters`: Updates an audio structure by loading sample data, pitch, and volume from a table based on a command byte, and modifies hardware audio registers and DMA control.
- `dispatch_command_by_nibble`: Calls another routine, then reads a 16-bit value from a structure to check for zero; if non-zero, it extracts a 4-bit nibble from a byte in the structure and branches to various subroutines based on that nibble's value.
- `copy_offset_to_destination`: Copies a 16-bit word from offset 16 of a structure in a6 to offset 6 of a structure in a5 and returns.
- `update_playback_state`: Performs a table lookup to update offset 12 of a structure, clears offset 18, and sets a flag at offset 22 if a comparison between the looked-up value and offset 8 fails.
- `update_counter_and_lookup`: Manages a counter at offset 24 of a structure by adding or subtracting a byte from offset 23, and performs a table lookup to update offset 6 of a structure based on a value at offset 18.
- `update_structure_parameters`: Performs complex arithmetic on values within a structure, including table lookups and bitwise manipulations, to update various offsets such as 6, 27, 29, and 8 based on input bytes and signedness.
- `dispatch_input_command_parameters`: Reads a command byte from a structure and branches to specific subroutines based on its value.
- `update_control_bits`: Updates various control bits and counters within a structure, including bitwise operations on offset 3 and offset 30, and conditionally performs DMA transfers by writing to the DMACON register.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
