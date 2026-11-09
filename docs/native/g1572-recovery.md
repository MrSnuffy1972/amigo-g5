# Game 1572 (no second prize (1990)(thalion)[cr ministry][a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01572). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907992 top-level bytes in 28 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 53 of 53 named (0 author symbols), 53 of 53 described (1 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1838 of 1871 (98.24%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1958 of 2029 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 23254 |
| code | 7420 |
| data | 871144 |
| padding | 30 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| No Second Prize (1990)(Thalion)[cr Ministry][a3].adf:bootblock | 1 | 17 | 1 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a3].adf boot-block load | 1 | 94 | 4 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a3].adf boot-block load | 1 | 56 | 2 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a3].adf boot-block load | 1 | 90 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 975 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 401 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 396 | 8 |

## Cases handled

- no second prize (1990)(thalion)[cr ministry][a3]/No Second Prize (1990)(Thalion)[cr Ministry][a3].adf: custom-format (non-DOS) disk, 1743 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 224 of the 2466 PCs the canonical run logged (2242 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- no second prize (1990)(thalion)[cr ministry][a3]/No Second Prize (1990)(Thalion)[cr Ministry][a3].adf: the boot block reads 16896 bytes from disk offset 805376 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][a3]/No Second Prize (1990)(Thalion)[cr Ministry][a3].adf: the boot block reads 2560 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][a3]/No Second Prize (1990)(Thalion)[cr Ministry][a3].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- 1669 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 3 first-run image unit(s) in 1 version group(s); 690 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- no second prize (1990)(thalion)[cr ministry][a3]/No Second Prize (1990)(Thalion)[cr Ministry][a3].adf: 3 logged loads cover 25088 bytes of the image
- no second prize (1990)(thalion)[cr ministry][a3]/No Second Prize (1990)(Thalion)[cr Ministry][a3].adf: blocks 22-1564 (bytes 11264-801280) hold 24 packed streams (IMP: File Imploder, TPWM: Turbo Packer) found by scanning, 98% of the bytes
- no second prize (1990)(thalion)[cr ministry][a3]/No Second Prize (1990)(Thalion)[cr Ministry][a3].adf: blocks 1606-1759 (bytes 822272-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### No Second Prize (1990)(Thalion)[cr Ministry][a3].adf:bootblock

- `run_bit_test_loop`: Boot block entry of seventeen instructions that makes one library request through a stored pointer, then runs a bit-test loop over a value read from memory, calling a helper routine for each group of bits.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a3].adf boot-block load

- `decompress_rle_bitstream_to_chip_ram`: Reads a bit-packed stream through bk_read_bits and bk_refill_bit_buffer, expanding literal and run-length copies backwards into memory at a fixed address until the write pointer passes the start, then jumps to a fixed address.
- `write_decrementing_global_until_wrap`: Writes a decrementing word to a global variable in a loop that runs until the counter wraps, then returns the value minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a3].adf boot-block load

- `disable_chips_then_idle_loop`: Entry of thirty-five instructions that loops forever if its argument is zero; otherwise it writes masks that clear the top bit, which disable DMA and interrupts, runs a delay countdown, and loops forever writing a fixed value to a global variable.
- `read_capped_entry_field`: Reads a field from a linked structure referenced by a global pointer.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][a3].adf boot-block load

- `save_registers_call_depack_then_jump`: Pushes all data and address registers onto the stack, calls the depack_stream_into_buffer routine, restores the registers, then jumps to the entry at 4244.
- `depack_stream_into_buffer`: Depacker loop of about sixty-four instructions that reads packed data through the bit reader routines, decoding each token and writing bytes backward into a buffer from its end.
- `return_without_action`: Routine whose shown body is only a return, so it does no work in the pseudo-code.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_lib_save_hw_state_and_call_init`: Opens a library via OldOpenLibrary, saves the returned pointer, copies six longs from the library base, reads INTENAR and DMACONR, clears eight copper slots, saves registers, calls the init routine, restores registers and copies saved state to low memory.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `init_copper_and_clear_buffers`: Waits for a keyboard character, clears two memory regions, builds copper list entries for bitplanes and sprites, waits for vertical blank, starts copper DMA and enables interrupts.
- `clip_rect_and_set_blitter_pointer`: Reads four words from a1, clamps them against limits at offsets in a5, computes a source pointer from a2 and a3, waits for blitter idle via bit 14 of 2(a6), then writes blitter pointer, control and size words at offsets in a6.
- `rotate_sprite_vectors_and_copy_positions`: Uses sine and cosine words to rotate four sets of nine coordinate pairs into buffers, advances an angle by a step modulo 4096, then copies four position pairs from a table into the a5 fields.
- `init_blitter_and_draw_four_line_groups`: Sets blitter control words and minterms in the custom chip registers, then walks four coordinate pair lists, loading start and end points into d0-d3 and calling the line drawing routine for each pair until a terminator is reached.
- `clip_and_draw_line_with_bounds`: Updates a bounding box in memory from the four input coordinates, clips the line against the viewport limits in the a5 structure, interpolates endpoints, and calls the line rasterizer for the clipped segment.
- `bresenham_line_to_blitter_registers`: Orders the endpoints, computes the Bresenham error terms and octant, then programs the blitter registers with the line address, slope, and minterm to draw one line segment.
- `advance_intro_sequence_timer`: Increments the frame counter at offset 24 of the a5 structure and, based on its value, advances sprite positions at offsets 34 to 40 and writes palette words to fixed memory, returning after each stage.
- `cycle_palette_color_556`: Decrements the counter at offset 30 of a5; when it reaches zero, resets it to 8 and steps the word at the pc-relative palette location toward the value with nibbles 5,5,6 by adding to its high, middle and low nibbles.
- `cycle_eleven_palette_entries`: Decrements the counter at offset 30 of a5; when it reaches zero, resets it to 2 and calls the palette-stepping routine for eleven consecutive word entries starting at the pc-relative table.
- `increment_palette_component_toward_mask`: Reads the word at a0, and if the bits selected by d3, d4 or d5 are not all set, adds 256, 16 or 1 respectively to that word.
- `decrement_palette_word_every_eighth_call`: Counts down the word at offset 30 of a5; on reaching zero reloads it with 8 and subtracts 256, 16 and 1 from the palette word at a0 when those bit fields are set, storing a constant at offset 24 of a5 if all are clear.
- `countdown_then_redraw_eleven_objects`: Decrements the word at offset 30 in a5; when it reaches zero, resets it to two, loads color masks into d3-d5, clears d1 and calls the draw routine eleven times with successive pc-relative addresses.
- `decrement_bcd_digit_on_match`: Reads the word at (a0), tests it against masks in d3, d4 and d5, and subtracts 256, 16 or 1 for each matching bit, setting d1 to 1; if no bit matched it writes 1 to 32(a5).
- `blit_three_text_rows_from_font`: Copies three runs of character glyphs from a font table into screen memory, converting each ASCII byte to a glyph index and writing eight bytes per character at row offsets 0, 140, 280, 420, 560, 700, 840 and 980.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `state_machine_dispatch_by_phase_counter`: Reads the phase word at 66(a5) and branches to per-phase code that advances the counter, updates pointers at 58(a5) and 62(a5), and calls the phase routine until the final phase is reached.
- `expand_21_char_glyph_rows`: Reads 21 characters from a1, subtracts 32 to index a2, and copies eight bytes per glyph into a0 at offsets 0, 140, 280, 420, 560, 700, 840 and 980, advancing a0.
- `set_audio_channel0_pointers_and_period`: Adds the module base from offset 62 of a5 to a program-relative table, stores the sample start and repeat pointers into a6 offsets 80 and 84, then writes period, volume and length values to the audio channel registers.
- `set_audio_channel1_pointers_and_period`: Adds the module base from offset 62 of a5 to a program-relative table plus 7000, stores the resulting start and repeat pointers into a6 offsets 80 and 84, then writes period, volume and length to the second audio channel.
- `init_audio_channel_base_sample`: Waits until bit 14 of the audio hardware status clears, then sets channel pointers, period, volume and length registers from the base sample offset.
- `init_audio_channel_offset_3500`: Waits for audio status bit 14, then programs channel pointers, period, volume and length using the sample base plus 3500 bytes.
- `init_audio_channel_offset_7000`: Waits for audio status bit 14, then programs channel pointers, period, volume and length using the sample base plus 7000 bytes.
- `init_audio_channel_a_forward_buffer`: Computes a sample buffer address from the a5 base plus a fixed offset, waits while bit 14 of the audio status word is set, then stores buffer pointers, period 118, volume 100 and control word into the a6 channel structure.
- `init_audio_channel_b_reverse_buffer`: Builds a buffer address from the a5 base plus a fixed offset, polls bit 14 of the a6 status word, then writes buffer pointers, period 116, volume 100 and control word into the channel structure.
- `init_audio_channel_c_offset_buffer`: Adds a large fixed offset to the a5 base to form a buffer address, waits on bit 14 of the a6 status word, then stores buffer pointers, period 116, volume 100 and control word.
- `init_audio_channel_7000_offset`: Waits while bit 14 of the audio status byte is set, then clears word 112(a6) and loads pointers at offsets 80, 76 and 84(a6) from the base plus 7000, setting period and volume words.
- `init_audio_channel_10500_offset`: Waits while bit 14 of the audio status byte is set, then clears word 112(a6) and loads pointers at offsets 80, 76 and 84(a6) from the base plus 10500, setting period and volume words.
- `init_audio_channel_210_offset`: Waits while bit 14 of the audio status byte is set, then clears word 112(a6) and loads pointers at offsets 80, 76 and 84(a6) from the base plus 210, setting period and volume words.
- `init_audio_channel_first_buffer`: Waits while bit 6 of the audio status byte is set, then clears word 112 and points the channel pointers at the first buffer offset plus 3500, setting period, volume and length registers.
- `init_audio_channel_second_buffer`: Waits while bit 6 of the audio status byte is set, then clears word 112 and points the channel pointers at the second buffer offset plus 7000, setting period, volume and length registers.
- `init_audio_channel_third_buffer`: Waits while bit 6 of the audio status byte is set, then clears word 112 and points the channel pointers at the third buffer offset plus 10500, setting period, volume and length registers.
- `init_audio_channel_pointers`: Computes two sample buffer pointers from the a5 offset, stores them at a6 offsets 80 and 84, sets period and volume words, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `level2_interrupt_ack_and_dispatch`: Saves all registers, acknowledges the level 2 interrupt in INTREQ, reads CIAB_ICR, calls the sound routine, restores registers and returns from interrupt.
- `reset_audio_and_joystick_state`: Scans a table for the highest byte, clears 31 sample slots and their pointers, sets CIAA_PRA bit, silences all four audio volumes and clears state bytes.
- `silence_audio_channels_and_enable_dma`: Clears the volume registers of all four audio channels to zero, then writes fifteen to DMACON to enable the audio and disk DMA channels before returning.
- `advance_audio_frame_and_reload_channels`: Increments a frame counter, and when it reaches the limit clears it, computes sample offsets from the pattern data, programs all four audio channels via the helper, enables DMA, copies channel state to the custom chip registers and advances the pattern position.
- `wait_for_vertical_position_change`: Reads VHPOSR into d2 and polls it until the value changes, retrying a few times with dbra loops, then returns after a short delay.
- `initialize_audio_channel_registers`: Checks a value in a hardware register against a byte in a global variable; if they differ, it iterates through four specific hardware register addresses, calling another routine for each to load data from memory offsets.
- `update_audio_channel_volume_and_period`: Reads a channel structure, looks up sample and period tables, writes AUDx pointers and period, handles volume ramps and vibrato, and updates the DMACON and channel enable word.
- `step_audio_channel_volume_envelope`: Advances a channel's volume or period envelope from its control byte, applying slide, vibrato and table lookups, and writes the resulting period to the audio hardware register.
