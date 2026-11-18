# Game 1696 (blues brothers, the (1991)(titus)[cr company][t +3 slipstream]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01696). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908752 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 54 of 54 named (0 author symbols), 54 of 54 described (3 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 880 of 880 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1569 of 1569 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10037 |
| code | 7228 |
| data | 803935 |
| unused | 87552 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf:bootblock | 1 | 63 | 2 |
| logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 155 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 929 | 31 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 421 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: custom-format (non-DOS) disk, 1587 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2207175 of its 2208090 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-joy slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2207175 of its 2208090 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-keys slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2207175 of its 2208090 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-mixed slid through memory (an address: 135 64 KB pages each with >= 29491 distinct PCs executed, 2207175 of its 2208090 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run ref ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 12)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: the boot block reads 8192 bytes from disk offset 855040 to an address (found statically); recovered as a unit
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 143 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 675 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: 1 logged loads cover 8192 bytes of the image
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 2-4 (bytes 1024-2560) outside the filesystem hold data, not code (opcode density 0.018, entropy 6.73)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 6-7 (bytes 3072-4096) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.77)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 9-12 (bytes 4608-6656) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.77)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 16-531 (bytes 8192-272384) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.41)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 538-786 (bytes 275456-402944) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.88)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 790-870 (bytes 404480-445952) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.80)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 874-902 (bytes 447488-462336) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.01)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 904-904 (bytes 462848-463360) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.59)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 906-966 (bytes 463872-495104) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.95)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 968-968 (bytes 495616-496128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.18)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 970-970 (bytes 496640-497152) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.38)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 972-1158 (bytes 497664-593408) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.67)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1160-1162 (bytes 593920-595456) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.94)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1164-1164 (bytes 595968-596480) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.30)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1166-1232 (bytes 596992-631296) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.86)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1234-1238 (bytes 631808-634368) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.77)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1240-1268 (bytes 634880-649728) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.18)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1270-1270 (bytes 650240-650752) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.55)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1272-1272 (bytes 651264-651776) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.57)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1274-1544 (bytes 652288-791040) outside the filesystem hold data, not code (opcode density 0.030, entropy 6.48)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1577-1583 (bytes 807424-811008) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.31)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1585-1592 (bytes 811520-815616) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.33)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1628-1666 (bytes 833536-853504) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.33)
- blues brothers, the (1991)(titus)[cr company][t +3 slipstream]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf: blocks 1668-1668 (bytes 854016-854528) outside the filesystem hold data, not code (opcode density 0.027, entropy 1.86)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `install_copper_interrupt_and_disk_control` (runtime segment an address): Calls exec Forbid, sets up the copper list pointer, saves and replaces the level 3 interrupt vector, enables DMA and interrupts, and toggles the CIAB disk control lines. Kind: disk hardware/timing check. Pass path: bsr.w an address.

## Functions


### Blues Brothers, The (1991)(Titus)[cr Company][t +3 Slipstream].adf:bootblock

- `configure_display_registers_at_boot`: Boot block entry of forty instructions that sets the display window, data fetch and bitplane registers, writes the DMA control register, then calls one library routine and one unnamed routine.
- `swap_buffer_halves_then_call`: Issues three library calls through a stored pointer, then swaps the two halves of each of 256 longwords in a buffer and calls a routine pointer taken from the buffer's fourth longword.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decompress_rle_bitstream_to_playfield`: Copies a block of bytes upward, then decodes a bit-packed stream using repeated bit reads, writing run-length expanded bytes backwards into memory and touching BPLCON0 via a5.
- `read_next_bit_with_carry_flag`: Pops a longword from the stack pointer a0, stores its low byte in d7 and writes d7 to the a5 hardware register, then rotates d0 right through extend and returns the shifted bit in carry.
- `read_bits_into_d2_accumulator`: Shifts d0 right repeatedly, refilling it from the bit reader when empty, and accumulates the extracted bits into d2 for the requested count in d1 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `install_vblank_handler_and_open_graphics`: Saves the level 3 vector, installs a new vertical blank interrupt handler, restores the trace vector, opens the graphics library, stores its base and copies a copper list pointer into the a5 frame before returning zero.
- `vblank_interrupt_service_chain`: Clears two longwords in low chip memory, calls six subroutines to update game state and hardware, then returns from the vertical blank interrupt with rte.
- `patch_eight_sprite_pointer_pairs`: Loads a base address, then for eight iterations writes the high and low halves of that address into the position fields of two sprite pointer tables, advancing both pointers by eight bytes.
- `patch_sprite_pointer_tables`: Writes the address of the sprite data at a fixed address into six pointer slots across three sprite tables, then fills three more slots with addresses stepping by 682 bytes.
- `install_copper_interrupt_and_disk_control`: Calls exec Forbid, sets up the copper list pointer, saves and replaces the level 3 interrupt vector, enables DMA and interrupts, and toggles the CIAB disk control lines.
- `build_sprite_pointer_and_bitmap_tables`: Stores six sprite data addresses into pointer tables, calls a fill routine six times with different patterns, then writes a generated bitmap pattern into the last table.
- `encode_delta_audio_words`: Reads the vertical-blank counter at custom register 7, mixes its low byte into each delta word, writes alternating data and marker longwords to the buffer at a0, and advances d0 by a fixed increment for d1+1 iterations before a terminating zero.
- `clear_audio_buffer_36d24`: Clears 2156 longwords starting at memory address 36d24 to zero using a1 as the write pointer and d7 as the countdown, then returns.
- `clear_audio_buffer_34e34`: Clears 4334 longwords starting at memory address 34e34 to zero using a1 as the write pointer and d7 as the countdown, then returns.
- `blues_bros_title_credits_scroller`: Copies title and credits text blocks into screen buffers, advances a horizontal scroll offset, and triggers the credit scroller when the fire button or timer condition is met.
- `set_a1_to_36682_and_a4_to_7ff00`: Loads the address 36682 into a1 and 7ff00 into a4, then branches to the shared routine at 444.
- `set_a1_to_369f2_and_a4_to_7ff01`: Loads the address 369f2 into a1 and 7ff01 into a4, then branches to the shared routine at 444.
- `load_graphics_and_sound_pointers`: Loads the address of a data block into a1 and a screen or buffer address into a4, then branches to another routine.
- `check_version_tag_and_return`: Loads two data pointers into a1 and a4, compares a longword at a pc-relative address against a tag, branches if unequal, otherwise returns.
- `increment_digit_counter`: Compares a byte at offset 3 of a0 with the character nine, and if equal increments the byte at offset 2 and resets offset 3 to zero, then branches.
- `advance_text_pointer_and_toggle_flag`: Increments the byte at offset 3 of a0 and the byte at a4, advances a0 by one, then either clears a6 or sets or clears a4 based on its value before calling the glyph routine with d1 zero and d2 three.
- `frag_lea_cmpi_bne_rts`: Short code fragment that loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a4, then loads the address of a location in the code into a0, then compares the memory at a0 with an address, then branches if the last result was not zero, then returns.
- `frag_cmpi_bne_subq_move_bra`: Short code fragment that compares the memory at an offset from a0 with 48, then branches if the last result was not zero, then subtracts 1 from the memory at an offset from a0, then copies 57 to the memory at an offset from a0, then jumps to another point in the code.
- `decrement_and_load`: Decrements bytes at two memory locations, increments a pointer, and repeatedly calls another routine until a condition is met.
- `draw_glyph_and_adjust_plane_offsets`: Calls the glyph drawing routine, compares d1 with d2, and if unequal adds 88 to d2 and 792 to a1 before returning.
- `fetch_char_and_copy_seven_plane_rows`: Reads the character at a0 indexed by d1, returns minus one in d4 for end marker, skips zero, and copies seven bytes from the font table into a1 at plane offsets 0, 88, 176, 264, 352, 440 and 528.
- `increment_three_sprite_palette_columns`: Adds 1, 2 and 3 to bytes at three parallel tables, stepping 8 bytes per entry, for 13 entries then 24 entries, then returns.
- `copy_pc_relative_block_to_c0`: Copies bytes from a program-counter-relative source upward into memory starting at address 192 until the source pointer reaches the end address, then returns.
- `patch_trap_vectors_and_jump_to_start`: Writes immediate opcode words into two fixed memory locations, conditionally overwrites two more with a no-op pattern, then jumps to the game start address.
- `conditional_store_d0_and_test_disk_status`: Tests byte at a fixed long address, and if nonzero writes one to another byte, stores d0 to a long address, then tests bit 5 of the custom chip status register before returning.
- `reset_audio_channels_and_clear_dma`: Clears a word at a fixed address, points a5 at the custom chip base, zeroes four audio channel pointer registers, sets audio period register to fifteen, and clears the CIAA fire button bit.
- `init_audio_channels_and_sample_tables`: Enables CIA fire-button bit and audio DMA, builds per-channel sample pointer tables from the module header, and programs four audio channel registers with period, volume and sample start addresses.
- `audio_channel_dma_reload_tick`: Decrements a frame counter and, when it expires, rebuilds four audio sample pointers and lengths, writes them to the four channel registers and enables DMA channels via DMACON.
- `advance_channel_sequence_and_trigger`: Advances the sequence pointer of a channel structure, and when the step byte is non-zero it reloads the instrument and note bytes from the pattern data, updates the channel's sample pointer and period, and writes the channel's DMA enable bit to the custom DMA control register.
- `interpret_sound_command_stream`: Walks a sound channel's command bytes, handling sample, volume, tempo and loop opcodes, updating channel state fields and writing DMA enable bits to DMACON.
- `check_flag_and_call_sound_then_jump`: Tests bit 4 of the word at offset 30 in a5; if set, stores 16 at offset 156 in a5, saves all registers, calls the sound routine, restores them, then saves d0/a0-a2, calls another routine and jumps to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `install_copper_and_interrupt_vectors`: Writes level interrupt and copper list pointers with high bits set, stores the stack pointer, installs a trap handler, then masks interrupt bits and reads the copper and DMA control registers.
- `build_copper_lists_and_init_display`: Clears memory, fills copper list tables with bitplane pointers and color values, sets up blitter and copper registers, then reads VPOSR and joystick bits to drive the game's display and input state.
- `clear_block_and_wait_frames_2`: Clears a block of 456 words, sets two stored addresses and a count, then loops waiting for the beam to reach a line, calling helpers on each pass.
- `advance_sound_channel_and_fill_buffer`: Reads a channel index from the a5 structure, increments it by two, stores it back, then fills 14 entries of a buffer using a6 pointing at custom chip base before jumping to the main loop.
- `set_channel_flag_and_jump_main_loop`: Writes byte 255 to offset 4731 of the a5 structure and branches unconditionally to the main sound driver loop.
- `init_copper_and_blit_scroll_strip`: Loads the custom chip base into a6, reads a scroll offset, stores it in the a5 structure, sets up source and destination pointers and calls the blit routine thirteen times with a fixed blitter control word.
- `reset_scroll_flag_and_restart_loop`: Clears the byte flag at offset 127b of the a5 structure, calls the scroll setup routine, then jumps back into the main loop entry point.
- `advance_colour_step_frame_2`: Counts down a frame counter and, when it reaches zero, moves a step index forward by two, wrapping after 456 entries.
- `reset_pointer_if_end_marker`: Loads a pointer from pc-relative memory, adds 376 to it, checks whether the word there is minus one, and if so reloads the pointer from another pc-relative table and stores it in a5-relative memory before branching away.
- `wait_for_vertical_position_b000`: Reads the VPOSR hardware register, masks it to the vertical position bits, compares against the value b000, and if equal sets a5 from a pc-relative address and calls a subroutine.
- `remap_row_colours_without_wait_2`: Advances the source row pointer and, for each of 160 bytes, searches a small palette list for the matching value, writing the match into a copper list entry with a fixed value of 4 beside it.
- `write_indexed_list_block_entries_2`: When the input value is not the end marker 640, writes two groups of words into a list record, each group taken from pointer tables at that index, together with fixed control words and an offset computed from a parameter.
- `step_sprite_toward_target_nibbles`: Decrements a counter at 2994(a5), then compares low nibbles of a target word with a sprite word and adjusts the sprite word by one.
- `adjust_sprite_high_nibble_toward_target`: Increments the word at (a1), then compares high nibbles of a0 and a1 words, subtracting 16 from (a1) when the target nibble is lower.
- `adjust_palette_entry_by_high_nibble`: Adds 16 to the word at a1, compares the high nibbles of the words at a0 and a1, and subtracts 256 from the a1 word if the a0 nibble is lower.
- `advance_palette_pointer_and_loop`: Adds 256 to the word at a1, advances a0 by two bytes, and decrements d0 looping back to the earlier comparison until d0 expires, then returns.
- `forbid_and_copy_copper_pointer_lists`: Calls exec Forbid, reads two pointer fields from the library base, stores eight pointer and word pairs into tables, and sets two a5 offsets to the table addresses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
