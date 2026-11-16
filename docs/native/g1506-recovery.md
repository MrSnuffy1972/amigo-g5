# Game 1506 (dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01506). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911952 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 24 raw-word lines |
| Functions named and described | pass, 165 of 165 named (0 author symbols), 165 of 165 described (2 by rule, 64 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1537 of 1537 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 846272 bytes, 47 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4377 of 4378 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 39271 |
| code | 9456 |
| data | 817137 |
| padding | 8 |
| unused | 46080 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:bootblock | 1 | 20 | 2 |
| logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf boot-block load | 1 | 196 | 35 |
| logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf boot-block load | 1 | 124 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 691 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1312 | 55 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf bytes 1024-1536 (code never loaded by a reference run) | 1 | 147 | 8 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf bytes 4096-4608 (code never loaded by a reference run) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown) | 1 | 783 | 20 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 242-289 (bytes 123904-148480)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 297-341 (bytes 152064-175104)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 352-458 (bytes 180224-235008)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown) | 1 | 1105 | 19 |

## Cases handled

- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: custom-format (non-DOS) disk, 1664 non-empty blocks: matched against track-loaded segments
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: the boot block reads 19968 bytes from disk offset 253952 to an address (found statically); recovered as a unit
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: the boot block reads 17408 bytes from disk offset 236544 to an address (found statically); recovered as a unit
- runtime segment an address: 664 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 873 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: bytes 1024-1536 look like code no run loaded: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: bytes 4096-4608 look like code no run loaded: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: 4 logged loads cover 38400 bytes of the image
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 4-4 (bytes 2048-2560) outside the filesystem hold data, not code (opcode density 0.020, entropy 4.42)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 11-235 (bytes 5632-120832) hold a IMP: File Imploder stream (115200 -> 144684 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 11-235 (bytes 5632-120832)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 242-289 (bytes 123904-148480) hold a IMP: File Imploder stream (24576 -> 113460 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 242-289 (bytes 123904-148480)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 297-341 (bytes 152064-175104) hold a IMP: File Imploder stream (23040 -> 209510 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 297-341 (bytes 152064-175104)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 352-458 (bytes 180224-235008) hold a IMP: File Imploder stream (54784 -> 177594 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 352-458 (bytes 180224-235008)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1067-1098 (bytes 546304-562688) outside the filesystem hold data, not code (opcode density 0.013, entropy 7.09)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1100-1372 (bytes 563200-702976) hold a IMP: File Imploder stream (139776 -> 200000 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1375-1405 (bytes 704000-719872) hold a IMP: File Imploder stream (15872 -> 12960 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1408-1480 (bytes 720896-758272) hold a IMP: File Imploder stream (37376 -> 39240 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1485-1535 (bytes 760320-786432) hold a IMP: File Imploder stream (26112 -> 60160 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1540-1557 (bytes 788480-797696) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1562-1580 (bytes 799744-809472) hold a IMP: File Imploder stream (9728 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1584-1604 (bytes 811008-821760) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1606-1626 (bytes 822272-833024) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1628-1647 (bytes 833536-843776) hold a IMP: File Imploder stream (10240 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1650-1667 (bytes 844800-854016) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1672-1686 (bytes 856064-863744) hold a IMP: File Imploder stream (7680 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1694-1710 (bytes 867328-876032) hold a IMP: File Imploder stream (8704 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf: blocks 1716-1741 (bytes 878592-891904) hold a IMP: File Imploder stream (13312 -> 41760 bytes)
- 6 never-loaded region(s), 845760 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 270336 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `pulse_disk_control_port_line` (runtime segment an address): Waits on a pending condition, then pulses one line of the drive control port register low and back high with waits in between. Kind: disk hardware timing/pulse check. Pass path: the loop continues or the routine returns normally to allow the next instruction to execute.
- `pulse_disk_drive_control` (Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf bytes 1024-1536 (code never loaded by a reference run)): Iterates through a loop to call a routine that reads and decodes a disk track, then pulses a byte at a specific memory location with values of 255, 247, and 255 while calling another routine between writes, and finally stores a value in a global variable before returning. Kind: sabotage path / integrity check. Pass path: the loop executes normally and the routine returns; the 'protection' is triggered if the loop condition (extraout_D1w) is manipulated or if the sub-calls fail to validate the disk environment..

## Functions


### Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:bootblock

- `boot_unpack_backward_copy_stream`: Boot-block entry point that decodes a packed data stream into memory, copying literal bytes and backward back-references chosen by bit-tested length and distance codes.
- `boot_setup_then_spin`: Writes a set of initial hardware and memory values, selects one of two control words depending on a flag and calls four routines.

### logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf boot-block load

- `unpack_packed_back_references`: Copies a block of longwords, then decodes a packed stream of literal bytes and back-references, writing output from the end of the buffer toward its start.
- `decode_packed_back_reference_block`: Copies a longword block, initialises a bit register and decodes a packed back-reference stream into a buffer, writing from the end toward the start.
- `jump_through_unrecovered_pointer`: Calls through a pointer that the decompiler could not recover, then returns.
- `store_byte_and_dispatch`: Writes the data byte to the location before the address register, then calls one of two unrecovered targets depending on whether the data word is zero.
- `dispatch_on_byte_flag`: Calls one of two unrecovered targets depending on whether the byte in the data register is zero, then returns.
- `load_address_and_jump_small`: Sets register d4 to 3, loads a pointer into a6, and performs an indirect jump using a4.
- `increment_and_branch`: Adds 6 to a 16-bit register and branches to another location.
- `load_address_and_jump`: Sets register d4 to 10, loads a pointer into a6, and performs an indirect jump using a4.
- `set_register_and_branch`: Sets register d4 to 16 and branches to another location.
- `decode_packed_run_lengths`: Reads bits from a packed longword stream, counting leading bits into a value, then chooses a run size of eight or two from that count.
- `dispatch_on_count_threshold`: Calls one of two unrecovered targets depending on whether the count in the data register is below six, then returns.
- `load_pointer_and_jump`: Loads a pointer into a6 and performs an indirect jump using a5.
- `call_after_range_check`: Tests a byte against one and two, then calls an unrecovered target.
- `set_register_and_jump`: Sets register d4 to 2 and performs an indirect jump using a4.
- `set_d3_and_branch`: Sets register d3 to 3 and branches to another routine.
- `test_d6_and_branch`: Sets register d3 to 1, tests the byte in d6, and branches to another routine if d6 is zero or if it is non-zero.
- `call_after_zero_check`: Tests a byte against zero, then calls an unrecovered target.
- `branch_on_d6_zero`: Tests the byte in d6 and branches to another routine if it is zero, otherwise it branches to another routine.
- `dispatch_on_byte_below_three`: Calls one of two unrecovered targets depending on whether the byte in the data register is below three, then returns.
- `dispatch_on_count_above_two`: Calls one of two unrecovered targets depending on whether the count in the data register is above two, then returns.
- `call_after_count_check`: Tests a count against two, then calls an unrecovered target.
- `add_d6_to_d3`: Adds the byte in d6 to d3, loads a pointer into a6, and branches to another routine.
- `test_byte_and_branch`: Tests a byte in register d6 and branches if it is non-zero; otherwise, it sets registers d4 and d2 to 7 and 2 respectively and branches to another location.
- `initialize_and_jump`: Sets register d4 to 8, register d2 to 130, loads a pointer into a6, and performs an indirect jump using a4.
- `copy_bytes_backward_then_call`: Copies a block of bytes backward from a source offset into the output, counting the data word down to minus one, then calls an unrecovered target.
- `set_d3_seven_and_branch`: Sets register d3 to 7 and branches to another routine.
- `set_d3_four_and_branch`: Sets register d3 to 4 and branches to another routine.
- `compare_d6_and_set_d3`: Sets register d3 to 2, compares the byte in d6 to 1, and branches to another routine if d6 is less than 1, equal to 1, or if d3 is set to 3 and it branches to another routine.
- `decode_back_reference_loop`: Decodes a packed stream of literal bytes and back-references from the source into the output, using bit-tested run lengths.
- `decode_back_reference_entry`: Decodes packed back-reference data from a source into an output buffer, reading the next packed word when the bit register empties.
- `decode_back_reference_restart`: Begins a back-reference decode by copying bytes forward from the source, then reads packed words and writes literals and runs into the output.
- `copy_then_decode_back_references`: Copies a block into the output, then decodes back-reference codes and writes runs from the source into the output.
- `enable_dma_interrupts_then_call`: Writes a byte to the output, sets the DMA master enable and interrupt enable bits through the custom registers, then calls an unrecovered target.
- `decode_sentinel_back_reference`: Decodes back-reference data that uses two sentinel byte values to end a run, writing literals and repeats from the source into the output.
- `decode_sentinel_repeat_runs`: Decodes packed bytes with sentinel values, writing repeated runs of a byte and literals into the output.

### logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf boot-block load

- `select_half_or_input_word`: Returns the input word unchanged unless a flag is set, in which case it returns half of another word.
- `mask_word_with_table_entry`: Looks up a word in a table indexed by the data register times two, and returns it ANDed with the input word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_display_and_idle_loop`: Sets up interrupt and DMA control through a sequence of custom register writes and helper calls, then waits on a status value in an endless loop.
- `copy_tables_and_init_record`: Copies two fixed longword tables into buffers, one a block of 2560 longwords and the other a paired table of 9280 entries.
- `or_combine_bitplane_rows`: Combines five source bitplane rows with a bitwise OR over two rectangular regions and writes the combined word into five destination bitplanes at fixed row offsets.
- `write_split_pointer_entries`: Writes four pairs of split high and low address words into two pointer tables, advancing each source address by a fixed step.
- `fade_palette_entries_in`: Toggles a frame flag and, on alternate calls, advances a fade counter and scales eight 12-bit colour entries up by that counter, channel by channel.
- `fade_full_palette_block`: Toggles a frame flag and, on alternate calls, runs the eight-entry fade-in helper and then scales a block of 32 colour entries by the current fade counter, storing the counter back.
- `scale_palette_block_down`: When the fade counter is nonzero it decrements the counter and writes a block of colour words, each nibble multiplied by the counter and divided by sixteen.
- `queue_blitter_fixed_transfer`: Waits until the blitter is idle, then loads its control, mask, size and pointer registers with fixed values and starts a transfer of a constant size.
- `queue_blitter_small_transfer`: Waits for the blitter to be free and starts a small fixed-size transfer using constant source and destination pointers.
- `program_blitter_modulo_setup`: Waits for the blitter to be idle, programs its control, mask, modulo and address registers from fixed constants, and starts a block operation.
- `advance_blit_script_entry`: Counts down a delay and, when it expires, reads the next three-part entry from a command table and programs the blitter with its values.
- `advance_second_blit_script`: Counts down a second frame timer and, on expiry, reads the next script record and queues up to two blitter transfers, stopping at an end marker.
- `animate_offsets_and_blit`: Advances a tick counter, toggles a phase value every sixth tick, steps a second value at intervals, and programs blitter registers from offsets computed off the phase.
- `reprogram_blit_from_offset`: Waits for the blitter to be idle, then reprograms its address, modulo and size registers from a stored offset and fixed constants.
- `step_sprite_position_script`: Steps a timed position script: when its delay counter expires it loads the next delay, speed and table offset, then queues two blitter transfers at the current position.
- `cycle_four_entry_table_index`: Decrements a countdown and, when it expires, advances a four-entry index modulo four and reloads two table words from that entry.
- `advance_scrolling_layer_entry_two`: Performs the same scrolling-layer script step as the other entry point, loading the next record, updating velocity and position, and queuing a blitter transfer.
- `initialize_graphics_data`: Initializes multiple graphics-related data structures with various constants and computed values, then returns.
- `advance_table_index_and_reload_2`: Counts down a timer and, when it expires, advances a two-bit index and reloads a table value and the timer.
- `advance_channel_tables_and_blit`: Advances the channel table steppers and accumulated offsets when their countdowns expire, calls the stored routine, then waits for the blitter and programs two blit operations.
- `advance_table_index_and_reload`: Decrements a 16-bit timer in a global variable and, when it reaches zero, increments a two-bit index in another global variable and reloads two values from a table based on that index.
- `read_debounced_port_nibble`: Reads a CIA port several times until the input is stable, then looks up a six-bit value in a table using the low four bits of the reading and returns it with a flag bit.
- `write_cia_port_bits`: Sets a control bit and a two-bit selector in a CIA port register from its argument, then returns the inverse of the low six bits.
- `acknowledge_pending_interrupt_bits`: Reads the interrupt request register and, when certain pending bits are set, updates a control value and calls a helper, then writes the handled bits back to clear them.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_main_dispatch_loop`: Runs an endless main loop that clears all interrupt and DMA enables, sets a conditional enable mask, and calls the per-frame helpers in sequence.
- `wait_flag_then_spin_forever`: Busy-waits until a shared flag byte equals the character B, then spins forever without returning, so the control flow never leaves this routine.
- `initialize_audio_system_2`: Sets up audio channel state and configures copper list pointers while managing display line pacing and interrupt handling.
- `frag_move_moveq_lea_bsr_jmp`: Short code fragment that copies an address to a fixed memory address, then loads 100 into d0, then loads 23 into d1, then loads the address of a fixed memory address into a0, then calls another routine (4 times in a row), then jumps to another point in the code.
- `wait_flag_then_spin_forever_2`: Busy-waits until a shared flag byte equals the character B, then spins forever without returning; its surrounding interrupt and DMA setup is not reachable in this copy.
- `step_index_through_seventeen_slots`: Sets two control bytes, then steps an index upward through seventeen slots, calling two helpers for each slot and leaving the index at the end of the run.
- `count_index_down_to_zero`: Repeatedly calls two helpers while stepping an index downward until it goes negative, then resets the index to zero.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `scale_thirty_two_colour_words`: Scales thirty-two colour words by a counter value, multiplying each nibble by the counter and dividing by sixteen, and writes the results to a block.
- `pulse_disk_control_port_line`: Waits on a pending condition, then pulses one line of the drive control port register low and back high with waits in between.
- `decode_amiga_track_sectors`: Waits for the MFM sync marker in the raw disk DMA buffer, decodes the odd and even bit-interleaved sector data, checks an XOR checksum, and copies each valid sector of 512 bytes to memory up to a limit.
- `step_head_to_target_track`: Sets or clears one drive control bit from the low bit of a value, then steps a track counter word toward a target by calling a helper until the two match.
- `disk_ready_handshake_reset`: Waits for the disk-ready condition, clears a counter word, sets a control bit on a CIA port, and waits for a two-stage flag handshake of the characters B and A.
- `step_track_counter_down`: Pulses a control bit low and high around helper delays, decrements a track counter word, and waits for the B then A flag handshake.
- `step_track_counter_up`: Pulses a control bit low and high around helper delays, increments a track counter word, and waits for the B then A flag handshake.
- `send_two_byte_control_sequence`: Writes a control byte with its high bit set, waits through a helper delay, then writes a second control byte with a different value.
- `wait_for_video_position_change`: Polls the video position register in a loop until its value is observed to change, so the caller resumes on the next beam position.
- `process_iff_form_chunk`: Checks whether the buffer starts with the four-character FORM header and, if so, calls two helpers to process the chunk; otherwise it returns immediately.
- `unpack_cmap_palette_words`: Searches a chunk list for the four-letter tag CMAP, then expands its packed three-byte colour entries into twelve-bit colour words in the palette area.
- `unpack_iff_body_chunk`: Finds the BODY chunk of an IFF image and copies its bytes to the destination, expanding run-length data when the compression flag is set.
- `decrunch_imp_stream_backwards`: Copies a table block into low memory, then, if the IMP! signature is present, decodes the packed stream from its end using a bit reader that handles literal runs and back-references.
- `decrunch_imp_stream_in_place`: Checks for the IMP! signature and, if present, decodes the packed stream in place from its end using the same bit reader, literal runs and back-references.
- `reload_audio_pointer_on_interrupt`: When the first audio channel's interrupt flag is set, it reloads that channel's location register from the beam position and sets a shared value.
- `init_channel_state_tables`: Clears a 98-longword work area, fills four channel slots with fixed values, and derives start pointers for each channel from a configuration table.
- `update_channel_timers_and_registers`: Advances per-channel countdown timers each frame, writing a stored pointer and length into the chip registers when a timer expires and reloading the next record.
- `update_four_channel_audio`: Processes audio playback for four channels, updating sample pointers, applying volume slides, triggering notes, and managing playback step counters.
- `advance_sample_loop_for_channels_2`: Walks three sound channels and advances each sample pointer by a step, handling one-shot, forward loop and ping-pong reversal modes.
- `load_channel_pattern_fields_2`: Loads a channel's pattern entry from a table indexed by a row byte, copying a word pointer and two byte parameters into the channel record and clearing a flag.
- `apply_channel_slide_step_2`: Calls a helper to update a pointer, then when a reset flag is set clears two accumulator words, and otherwise adds a stored delta to a channel output word and advances an accumulator by a step each tick while it is nonzero.
- `read_next_channel_command_byte_2`: Clears two channel counters, then either counts down a hold delay or reads the next command byte from a stream and stores its parameter bytes, storing a hold count when the byte is negative.
- `start_channel_note_from_table_2`: Clears the effect state fields of the record when the trigger at offset 46 is set, then loads a sample header from a table row indexed by the trigger, copying the pointer and length into the record and computing the initial period from a lookup table.
- `update_channel_period_with_clamp_2`: Calls the eight effect helpers for one tick, adds the slide amount read at offset 58 to the channel period at offset 10, clamps the result to a floor of 95 and a ceiling of 5760, and stores the clamped value to the period output at offset 6 of the structure in a6 indexed by the repeat count saved at entry.
- `step_repeat_counter_2`: While a table count is non-zero, it increments the record's counter until it reaches the table's limit, then adds the table count to the record's offset field.
- `update_entity_state_attributes_2`: Extracts a command index from a word at offset 50 of a structure in a2 and performs conditional operations based on that index.
- `slide_voice_period_toward_target`: Slides a voice period toward a target pitch read from a table, stepping by the slide rate and stopping exactly when the target is reached.
- `set_pitch_from_command_table_2`: Computes a table index from the command word and a byte in the caller's data, then loads the pitch value at that index into the record.
- `frag_move_neg_add_rts`: Short code fragment that copies the memory at an offset from a2 to d0, then negates d0, then copies d0 to the memory at an offset from a2, then adds d0 to the memory at an offset from a2, then returns.
- `apply_pitch_slide_up`: Stores the effect parameter as the slide rate and applies it to the voice pitch accumulator.
- `check_byte_and_branch`: Tests a byte at offset 4 of the structure in a0 and branches to either offset 168a or 1694.
- `ramp_level_from_control`: Adjusts a level counter from a control word: when the low nibble is zero it adds four times the high nibble capped at 255, otherwise it subtracts four times the control value floored at zero, and mirrors the result into a second field.
- `frag_bsr_bra`: Short code fragment that calls ramp_level_from_control, then jumps to another point in the code.
- `calculate_and_update`: Performs a lookup-based calculation using values from a global table, applies a multiplier and bit shift, negates the result if needed, subtracts it from a value in memory, and updates two byte values in the structure.
- `advance_position_by_speed`: Latches a new speed when the control field is nonzero, then moves a 32-bit position by the speed scaled by 256 and reduces a fractional field by the speed scaled by 128.
- `stop_voice_dma_channel`: When a parameter is set, resets the voice offset and clears that voice's bit in the DMA control word, which stops its channel.
- `frag_move_beq_cmp_bne_clr_rts`: Short code fragment that copies the memory at an offset from a2 to d0, then branches if the last result was zero, then compares d0 with the memory at an offset from a0, then branches if the last result was not zero, then clears the memory at an offset from a2 (2 times in a row), then copies 0 to an indexed entry from a6, then returns.
- `frag_move_beq_clr_rts`: Short code fragment that copies the memory at an offset from a2 to d0, then branches if the last result was zero, then clears the memory at an offset from a2, then copies d0 to the memory at an offset from a2, then copies the memory at an offset from a2 to the memory at an offset from a2 (2 times in a row), then returns.
- `decrement_and_move_byte`: Copies a byte from offset 5 of the structure in a0 to offset 3 of the same structure, then decrements the byte at offset 3.
- `scale_and_store_value`: Multiplies a value from a table by four, caps it at 255, stores it in a structure, and then stores a scaled version of that value into a destination pointed to by a6.
- `set_voice_volume_from_global`: When the stored parameter is below a fixed limit, writes the global volume plus that parameter into the voice volume byte.
- `step_secondary_envelope_pitch_2`: Advances the secondary envelope counter: when it reaches the segment length it reloads from length minus step and adds a table value to the pitch, otherwise it increments the counter.
- `step_primary_envelope_pitch_2`: Advances the primary envelope counter using its own record fields, reloading the counter and adding a lookup-table value to the pitch when the segment ends, otherwise incrementing the counter.
- `copy_sequence_entry_to_buffer_2`: Advances the sequence counter at offset 35, and when the segment ends fetches a byte from a table, negating the counter when the byte is negative, otherwise copying the long and word fields of a 64-byte entry from the table into the output buffer at a running offset and recording the entry index.
- `write_scaled_voice_volume`: Saves the voice period, then writes the high word of the voice volume multiplied by a master scale into the chip volume register for that channel.
- `run_volume_envelope_stage`: Runs the voice volume envelope: depending on the current stage it ramps the level up toward a limit or down toward a sustain value, and moves to the next stage when the target is reached.
- `normalise_shared_status_byte`: Rewrites a shared status byte with zero when it is already zero, and otherwise leaves it unchanged.

### Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf bytes 1024-1536 (code never loaded by a reference run)

- `pulse_disk_drive_control`: Iterates through a loop to call a routine that reads and decodes a disk track, then pulses a byte at a specific memory location with values of 255, 247, and 255 while calling another routine between writes, and finally stores a value in a global variable before returning.
- `decode_amiga_track_sectors_2`: Waits for the MFM sync marker in the raw disk DMA buffer, decodes the odd and even bit-interleaved sector data, checks an XOR checksum, and copies each valid sector of 512 bytes to memory up to a limit.
- `seek_disk_head_to_track_2`: Sets or clears bit 2 of a byte at the address in a5 based on the low bit of d0.
- `check_disk_status_bit`: Tests bit 4 of the CIAA_PRA register and, if not set, calls another routine before clearing a 16-bit word at a4 and setting bit 2 of the byte at a5.
- `pulse_drive_control_bits`: Sets bit 1 of the byte at the address in a5, calls another routine, clears bit 0 of that byte, calls another routine, sets bit 0 of that byte, decrements a 16-bit word at the address in a4, and branches to another routine.
- `clear_drive_control_bits`: Clears bit 1 of the byte at the address in a5, calls another routine, clears bit 0 of that byte, calls another routine, sets bit 0 of that byte, increments a 16-bit word at the address in a4, and branches to another routine.
- `pulse_drive_control_pattern`: Writes 127 to the byte at the address in a5, calls another routine, writes 119 to the same address, and returns if a byte at offset 6 of the structure in a6 is 65 or 66.
- `wait_for_beam_position_change`: Loops while comparing the value of the beam position register with its own incremented value, then returns.

### Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown)

- `initialize_object_dma_tables`: Clears a block of memory, populates several global variables with specific values, and performs pointer arithmetic to build a table of addresses.
- `update_dma_and_object_states`: Checks several global status bytes and, if conditions are met, updates a series of object structures by calling routines to initialize, load parameters, and clamp values.
- `advance_sample_loop_for_channels`: Walks three sound channels and advances each sample pointer by a step, handling one-shot, forward loop and ping-pong reversal modes.
- `load_channel_pattern_fields`: Loads a channel's pattern entry from a table indexed by a row byte, copying a word pointer and two byte parameters into the channel record and clearing a flag.
- `apply_channel_slide_step`: Adds a stored delta to a channel output word and advances an accumulator by a step each tick while it is nonzero, latching a new value from a global when a reset flag is set.
- `read_next_channel_command_byte`: Clears two channel counters, then either counts down a hold delay or reads the next command byte from a stream and stores its parameter bytes, storing a hold count when the byte is negative.
- `start_channel_note_from_table`: Clears several fields in a structure, then loads sample header values from a table row, setting the initial volume, period and sample pointer from that row.
- `update_channel_period_with_clamp`: Calls seven different routines, adds a 16-bit value from offset 58 of a structure in a2 to the value at offset 10 of that structure, and clamps the result between 95 and 5760 before storing it at offset 6 of a table indexed by a value from offset 16 of a2.
- `step_repeat_counter`: While a table count is non-zero, it increments the record's counter until it reaches the table's limit, then adds the table count to the record's offset field.
- `update_entity_state_attributes`: Extracts a command index from a word within a data structure and performs a series of conditional operations.
- `slide_period_toward_note_target`: Looks up a target period from a table and moves the channel period toward it by a stored step, snapping to the target when reached, and returns the current period.
- `set_pitch_from_command_table`: Computes a table index from the command word and a byte in the caller's data, then loads the pitch value at that index into the record.
- `negate_field_and_subtract`: Stores the negated value of one record field into a second field and subtracts that value from a third field.
- `adjust_counter_by_scaling`: If certain conditions are met, it updates offset 13, then either subtracts a scaled value from offset 12 or adds a scaled value to offset 12 based on a bitmask check.
- `step_secondary_envelope_pitch`: Advances the secondary envelope counter: when it reaches the segment length it reloads from length minus step and adds a table value to the pitch, otherwise it increments the counter.
- `step_primary_envelope_pitch`: Advances the primary envelope counter using its own record fields, reloading the counter and adding a lookup-table value to the pitch when the segment ends, otherwise incrementing the counter.
- `copy_sequence_entry_to_buffer`: Advances a sequence counter and, on a frame boundary, fetches a 64-byte entry from a table, copies its long and word fields into the output buffer at a running offset, and records the entry index.
- `scale_position_into_buffer`: Calls one helper, copies a position word into a second field, and writes the high half of its product with a scale field into an output buffer at a running offset.
- `run_envelope_state_machine`: Advances the record's level envelope one step according to its state: a ramp up, a ramp down, a counted pause, or a timed hold, moving to the next state when each one finishes.
- `write_dma_control_registers`: If a word at a specific address is zero, it fetches a byte from a table and writes it to several hardware registers, including a write to the DMA control register to enable channels.

### Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT][a].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown)

- `initialize_system_and_tables`: Sets hardware registers, calls three different routines, clears a memory block via a loop, and processes a data table to update various global variables based on specific byte markers.
- `calculate_table_offset`: Calculates two different offsets based on the input in d0 and adds them to a base address in a5, then returns.
- `clear_memory_blocks`: Calls another routine, then uses a loop to fill two memory blocks starting at offsets of a5 and a5 plus 8400 with zeros.
- `update_table_values`: Calculates a value by reading from a table using an offset derived from a byte at a specific offset in a structure, then performs a bitwise OR to store this value into two 32-bit words at offsets 0 and 8400 of a destination structure in a5 through a loop.
- `fill_table_from_source`: Copies bytes from a source table to a destination table, incrementing a byte value for each entry until a byte of all ones is encountered.
- `initialize_hardware_and_tables`: Sets a CIA register, configures several hardware registers, and populates a table with specific values derived from a calculation involving a nibble-based bit manipulation.
- `generate_lookup_tables`: Fills a table starting at offset 192 with values calculated via division, populates a second table starting at offset 8116 with bit-rotated bytes, and populates a third table starting at offset 8772 with incrementing 16-bit values.
- `perform_bit_manipulation_and_scaling`: Performs a loop of bitwise XOR operations and shifts on a value, then returns a result that is either a bitwise mask or a scaled product of the input.
- `process_audio_data_and_hardware`: Searches a table for a specific byte value, then uses a calculated offset to clear a series of memory locations and update several hardware registers including audio volume and DMA control.
- `update_audio_channels_and_parameters`: Updates the four Amiga audio channel registers by calling a routine with specific data pointers, then calculates a value from a table and a global structure to update a 16-bit value in a global variable and calls a routine to process command logic.
- `update_audio_and_hardware_state`: Loads a data structure from a table, updates its internal pointers and counters, and modifies hardware registers including the DMA controller and audio channels.
- `dispatch_parameter_command_logic`: Calls another routine, then extracts a 12-bit value and a 4-bit command nibble from a structure in a6.
- `process_command_parameter_value`: Extracts a 16-bit value from offset 16 of a structure in a6 and stores it at offset 6 of a structure in a5 before returning.
- `resolve_table_index`: Searches a table for a value matching a 12-bit field, then updates offset 12 of a structure in a6 and sets a flag at offset 22 based on a comparison.
- `update_envelope_state`: Updates a counter in a structure, performs table lookups to find a value, and writes that value to offset 6 of a structure in a5.
- `process_parameter_logic`: Performs complex conditional arithmetic and table lookups based on bytes in a structure to update various fields, including a value at offset 6 of a structure in a5 and an offset in a structure in a6.
- `dispatch_input_command_logic`: Reads a command byte from a structure, performs a multi-way branch based on its nibbles, and executes specific data updates or hardware register modifications.
- `manage_hardware_and_counters`: Updates various bytes in a structure and conditionally modifies DMA controller registers and other global memory locations based on bitwise operations on a byte in a6.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
