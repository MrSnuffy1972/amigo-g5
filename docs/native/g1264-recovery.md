# Game 1264 (revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01264). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (915818 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 22 of 22 units identical (bytes and relocations), 41 raw-word lines |
| Functions named and described | pass, 122 of 122 named (0 author symbols), 122 of 122 described (4 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2903 of 2964 (97.94%) |
| Recovered without load address (counted in proof 1, flagged) | 256 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3651 of 3829 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2820 |
| code | 16976 |
| data | 865206 |
| padding | 96 |
| unused | 30720 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf:bootblock | 1 | 84 | 4 |
| logged disk load from Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf | 1 | 98 | 3 |
| logged disk load from Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf | 1 | 507 | 13 |
| logged disk load from Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf | 1 | 108 | 9 |
| logged disk load from Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf | 1 | 64 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1797 | 52 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 410 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 90 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 374 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 91 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 112 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 2 |
| Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf bytes 16640-16896 beside a logged load (code never loaded by a reference run, load address unknown) | 1 | 0 | 0 |

## Cases handled

- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: custom-format (non-DOS) disk, 1698 non-empty blocks: matched against track-loaded segments
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: the boot block reads 2048 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: 95 executed PCs whose RAM was overwritten before any snapshot match its bytes 720896-721200: recovered as a unit run at an address
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: 377 executed PCs whose RAM was overwritten before any snapshot match its bytes 143360-146000: recovered as a unit run at an address
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: 77 executed PCs whose RAM was overwritten before any snapshot match its bytes 14768-16640: recovered as a unit run at an address
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: 62 executed PCs whose RAM was overwritten before any snapshot match its bytes 112640-113344: recovered as a unit run at an address
- 2270 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 15 first-run image unit(s) in 1 version group(s); 4318 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf bytes 16640-16896 beside a logged load (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 0 of 256 bytes decode)
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: 6 logged loads cover 7568 bytes of the image
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: blocks 6-22 (bytes 3072-11776) hold ASCII text
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: blocks 24-27 (bytes 12288-14336) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.29)
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: blocks 33-219 (bytes 16896-112640) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.25)
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: blocks 222-279 (bytes 113664-143360) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.81)
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: blocks 286-797 (bytes 146432-408576) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.66)
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: blocks 854-871 (bytes 437248-446464) outside the filesystem hold data, not code (opcode density 0.029, entropy 3.46)
- revenge of defender (1988)(epyx)[cr trilogy][t +2 trilogy]/Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf: blocks 875-1407 (bytes 448000-720896) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.54)
- 1 never-loaded region(s), 256 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 179712 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf:bootblock

- `bootblock_doio_then_jump_to_loader`: Loads exec base from absolute address 4 into a6, fills an IO request at a1 with command 2 and buffer and length fields, calls DoIO, saves all registers, calls the loader at pc offset 70, restores registers and jumps to address 7fd00.
- `decompress_packed_data_to_7c000`: Reads a packed stream from address 40000 using a bit reader, expands literal and back-reference tokens, writes the decompressed bytes backwards ending at 7c000, and returns when the output pointer reaches the start.
- `read_bits_into_d2_from_stream`: Shifts d0 right one bit at a time, refilling from the longword before a0 and xoring with d5 when empty, and rotates each bit into d2 for d1 iterations before returning.
- `fetch_bitstream_word_rotate_right`: Pops a longword from the stack pointed to by a0, XORs it into d5, sets the extend flag, rotates the popped value right through extend and returns it in d0.

### logged disk load from Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf

- `decompress_rle_bitstream_to_color_ram`: Copies a byte block, then decodes a bit-packed run-length stream into memory, writing each decoded byte to COLOR00 and the destination buffer until the source pointer passes the end.
- `fetch_bitstream_word_and_expand_run`: Pops a longword from the stack via a0, rotates it right through extend into d0, then expands a run of bytes backwards in the buffer at a2 and returns to the decoder loop.
- `bit_shift_with_callback`: Shifts d0 right by one bit d1 times, calling a subroutine whenever the shifted value becomes zero and rotating the carry into d2.

### logged disk load from Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf

- `disable_dma_interrupts_and_alloc_chip_ram`: Disables all DMA and interrupt sources, saves the interrupt enable register, allocates chip memory at a1 via exec AllocAbs, then jumps to the next routine.
- `game_init_and_main_loop_setup`: Initializes display, audio, copper, blitter and game state tables, then runs the main loop polling keyboard and timing until exit.
- `fill_two_arrays_with_incrementing_words`: Copies words from two source buffers into two destination buffers while writing an incrementing word value into every other slot, for thirty-one iterations, then returns.
- `clear_chip_ram_and_disable_dma`: Clears longwords from the given start address up to the end of chip memory, then writes fifteen to DMACON to enable only the disk and copper DMA bits.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_vposr_then_shift_sprite_table_words`: Waits until VPOSR matches the expected value, then shifts words between fields of a sprite table for two hundred eighty-nine entries and returns.
- `blit_letter_rows_to_playfield`: Reads 20 characters from a0, clears 24 words per space or copies 24 words per letter from the glyph table into the playfield buffer at a1, then if the flag word is zero writes five rows of d2 minus six times d3 into two sprite column buffers.
- `scroll_playfield_rows_by_delta`: Sets a flag bit in a4, then for 16 rows calls the row-shift routine twice and adds d1 and d2 to the two word fields at offsets 10 and 14 of each of 24 entries, stopping each field at its limit word.
- `clamp_and_store_row_position`: Tests d0 and returns if zero, otherwise adds d1 to d0 and stores it at offset 10 of a1, then loads offset 14, tests d2 and adds it before storing back at offset 14.
- `add_d2_to_d0_store_and_advance_a1`: Tests d0, skips if zero, adds d2 to d0, stores the word at offset 14 from a1, advances a1 by 20, and loops via dbra d5 and d6 before returning.
- `init_pointers_and_call_subsystem_setup`: Writes six longword pointers into fixed memory, sets bit 4 of a status byte, calls twelve setup routines, then conditionally calls two more before returning.
- `vertical_position_beam_sync_and_trigger`: Reads game state and flags to set a word value, optionally calls the setup routine, polls VHPOSR until the beam passes a threshold, acknowledges INTREQ and returns.
- `init_copper_list_and_palette_ramp`: Writes two pairs of display pointers to fixed low memory, then fills a copper list with color register writes and a 24-step brightness ramp, ending with a two-word terminator.

### logged disk load from Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf

- `advance_counters_and_call_on_wrap`: Walks a table of twelve-byte entries, increments each entry's counter, resets it and calls the target when it reaches the limit, then returns.
- `advance_linked_animation_frames`: Follows a linked list of animation nodes, advances each node's frame pointer by two bytes, wraps to the node's start when past the end, writes the frame word to the output pointer and continues.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `zero_filled_data_area`: Contains only a zero word that decodes as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `audio_dma_copy_or_playback_setup`: Enables audio DMA and interrupt, copies d2 bytes from a0 to d1 when the zero flag word is set, otherwise configures the audio IO block and calls exec DoIO.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `audio_stop_and_interrupt_disable`: Configures the audio IO block for command 9, calls exec DoIO, delays with a dbra loop on d0, then writes INTENA to disable interrupts and returns.
- `open_trackdisk_and_init_ports`: Opens trackdisk.device via exec, stores the IO request pointer, adds a port, and opens a message port with zero flags, saving the port pointer.
- `dispatch_sound_entry_by_index`: Multiplies d0 by twelve to index a table, clears the cached pointer, bounds-checks against 120, then loads three longs and jumps to the sound routine.

### logged disk load from Revenge of Defender (1988)(Epyx)[cr Trilogy][t +2 Trilogy].adf

- `init_copper_and_display_setup`: Fills two copper lists with bitplane pointers, programs display and bitplane hardware registers, calls a setup routine, writes palette entries, then jumps to the main loop.
- `build_palette_and_fill_bitplanes`: Writes display control bytes, decodes 32 three-byte entries into two palette tables, then calls a routine 8000 times to fill four bitplane rows at a4.
- `palette_decode_data_byte`: Single instruction ORs immediate value 160 into the low byte of d0; it is data reached by lea, not executable code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_d0_and_clear_global_word`: Loads 10 into d0, clears d1, branches past padding, calls a library routine, then clears the word at the global address and falls through nops.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_blitter_and_game_state_flags`: Swaps the halves of d0, stores the result at a fixed longword, writes all-ones to the blitter control register, and clears or sets several fixed word flags in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_fire_buttons_when_flag_set`: Tests a fixed word flag, and if set checks bits 6 and 7 of the CIA-A port register for the two fire buttons, returning after the checks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `test_potgor_button_bits`: Reads the POTGOR register into d2, tests bits 10 and 14, and returns without changing any other register.
- `zero_fill_data_word_at_0016`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `zero_fill_data_word_at_001a`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by three routines via absolute move, add and clear.
- `read_joy0dat_and_update_axis_state`: Reads JOY0DAT mouse port movement, compares the byte against stored globals, clamps values beyond plus or minus three, accumulates a delta into a state byte and returns.
- `update_vertical_offset_from_byte_delta`: Reads two bytes from fixed memory, subtracts one from the other, stores the difference back, sign-extends it, adds it to a running word offset, clamps the result between 20 and -20, and writes the low byte to a fixed address.
- `compute_joystick_direction_bits`: Reads a word from the address in a0 and the current input byte, masks the high nibble, tests bits 1 and 9 of the word and of its shifted xor copy, and sets or clears bits 2 and 0 of the result byte.
- `expand_nibble_and_offset_sprite_rows`: Reads a word from fixed memory, mirrors its low nibble into both halves and stores it, then if two flags are clear shifts the value right by three, clears bit 0, and adds it to five base words stored to five consecutive addresses.
- `compute_scaled_speed_index`: Reads a word from memory, shifts it right by four, subtracts another word, clamps the result to a minimum, stores it, then divides by five and stores the quotient.
- `main_frame_update_dispatch`: Sets up two pointers, increments a frame counter, calls many subroutines for game logic, and when a flag is set programs copper, audio and DMA registers before returning.
- `blit_queued_sprite_blocks`: Walks a pointer backwards through queued blit records, programs blitter control, pointer, modulo and size registers, calls the blit routine, and stores the updated pointer.
- `zero_filled_data_area_4`: This single zero word decodes as an ori.b immediate instruction, so the range is zero-filled data that other code references by address, not executable code.
- `blitter_sprite_list_renderer`: Pops sprite records from a stack pointer, programs the blitter registers and writes each record into the sprite list, looping until the pointer reaches its limit.
- `zero_filled_data_six_words`: Six zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `draw_four_scaled_sprite_frames`: Reads a word counter from memory, scales it by four, three, two and one over five, then calls the position and draw routines with adjusted offsets and sizes for four frames, ending in a jump.
- `compute_sprite_offset_from_d0`: Subtracts the word at a fixed address from d0, derives a parity bit and a doubled offset from the difference, adds the base offset, wraps the result modulo 72 and returns it in d3.
- `append_sprite_descriptor_to_list`: Loads the pointer at a fixed address, writes d4, constants, d2, d3 twice, a rotated d1 byte and its or-ed copy into the list, then stores the advanced pointer back.
- `update_player_vertical_velocity`: Reads joystick bits from the custom chip register, adjusts the vertical speed and position words in game memory, and returns after clamping the movement.
- `compute_sprite_pointer_from_tile`: Multiplies the tile row in d1 by 112 and the column offset in d0 by 5, stores the resulting sprite pointer in memory and returns the low nibble in d6.
- `queue_player_sprite_descriptor`: Selects a sprite frame from a table using the animation index, computes its address, writes a display list entry to the copper pointer and returns.
- `zero_filled_data_area_2`: Nine zero words that decode as ori.b #0,d0; this range is zero-filled data, not executable code.
- `check_two_flags_then_return`: Tests the word at a fixed long address and a second word, branching away if either is nonzero, otherwise returning immediately; the rest is zero-filled data.
- `scan_seven_word_entries_build_records`: Walks seven-word records from a fixed table, skipping zero entries, stopping at 999, and filling output records with size, position and sprite fields.
- `advance_sprite_pointer_and_emit_record`: Checks a2 against a fixed limit, advances a1 by two and d1 by four wrapping at twelve, then pushes d7, d1, a2, a1 and incremented d0 onto the a3 stack and jumps back into the caller loop.
- `decrement_timer_and_spawn_entity`: Decrements a global countdown, and when it reaches zero clears flags, resets the counter, and writes a new entity record with position and type fields into the entity table.
- `build_sprite_display_list_entries`: Steps a frame counter and slot index, reads sprite coordinates from a table, wraps them into screen range, calls the sprite setup routine, and writes nine display list entries to a6 and a1.
- `call_nonzero_function_pointers_in_table`: Walks a table of longword function pointers from the first entry to the last, calling each non-null pointer via jsr and advancing a6 by 32 bytes per entry.
- `spawn_enemy_from_spawn_table`: Reads spawn timing fields from the a6 structure, advances counters, and when the threshold is exceeded writes a new enemy record with position, sprite pointers and flags into the spawn table.
- `init_score_and_call_active_effects`: Stores a fixed value into a global score slot, then scans a table of ten-byte effect records and calls the effect routine for each entry whose first word is nonzero.
- `spawn_effect_from_table_advance_frame`: Increments the frame counter at offset six of the a6 structure, and when the limit at offset four is reached, writes a display object header and copies a table entry into the global list, advancing the list pointer.
- `advance_and_emit_sprite_record`: Increments a frame counter in the a6 structure, compares it against a limit, and when ready writes a sprite record with position, size and rotated d6 bits into the global buffer.
- `clamp_x_and_add_velocity_to_position`: Adds d0 to the word at offset eight of the a6 structure, clamping it below a maximum, and adds d1 to the word at offset ten, storing both back.
- `plot_pixel_run_in_bitplane`: Uses d0 and d1 coordinates to compute a bitplane address, rotates d2 to pick a bit, and sets that bit in three successive scanlines of screen memory.
- `blit_sprite_to_screen_buffer`: Waits for the blitter to finish, then programs BLTCON0, BLTCON1, source and destination pointers, modulos and BLTSIZE to copy a masked image into the screen buffer.
- `fill_palette_color_ramp`: Writes the constant color value to thirteen palette entries at a0, then writes seven more entries with the color increasing by sixteen each step.
- `draw_hud_and_sprite_tables`: Tests a flag word, calls setup routines, draws a fixed screen element from two coordinate words, then loops over nine four-byte entries and a table of sprite records calling the draw routine for each active entry.
- `cycle_three_frame_ring_buffer`: Increments a frame counter; when it reaches two it resets to zero and copies eighteen words backwards through a ring buffer, saving the last word to a separate location.
- `program_blitter_from_sprite_record`: Calls a setup routine, then loads blitter pointer, modulo, control and size registers from fields of the record at a0 and starts the blit.
- `blit_sprite_from_struct`: Calls a helper, then copies the sprite image pointer, modulo, control word and size from the a0 structure into the blitter registers and returns.
- `blit_and_wait_blitter_busy`: Calls a helper, loads blitter pointers, modulos, control and size from the a0 structure, waits for the blitter, then returns 1 if the blitter busy bit is clear and 0 otherwise.
- `update_active_entity_lists`: Scans the entity tables at fixed memory areas, compares positions against the player, and updates or clears entries in the active lists before resetting the list pointers.
- `convert_index_to_level_address`: Subtracts an address from d0, divides by 560, multiplies the quotient by 112, adds the remainder and an address, returning the resulting address in d0.
- `advance_sound_channel_pointer`: Loads the current channel pointer from memory, compares it with a limit, adds 32 to it and jumps to a sound routine, or stores the pointer back and returns.
- `read_and_mask_channel_status`: Reads a word from the current channel pointer unless it equals the end marker, then masks the low byte and sets bit 19, returning the result in d0.
- `mix_seed_into_global_state`: Loads the global seed, stores d1 to a global slot, XORs the seed with a constant and another global, rotates the result right by one bit and writes it back, then reloads d1 and returns.
- `increment_counter_and_build_status_word`: Increments a global counter, returns early if it wrapped, otherwise calls two routines, fills a caller-supplied structure with d1, d2, d3 and pointer words, and returns a status word.
- `zero_filled_data_area_3`: This single instruction is a zero word decoded as ori.b, so the range is zero-filled data referenced by address, not executable code.
- `advance_and_redraw_scanline_column`: Reads the current column index, increments it, and writes either a blank word or a table word into four spaced screen positions per column until the buffer end.
- `copy_sprite_column_words_to_screen`: Uses d0 as a byte index into a source table at a1 and copies 25 words into a0 at 20-byte strides, filling one screen column.
- `reset_column_state_and_clear_buffer`: Checks two flags, clears the pending word, and when both are set zeroes a block of longwords at a0 while returning a status in d0.
- `clear_sprite_block_if_flag_zero`: Tests the word at a fixed global; if zero, clears a flag, stores one to another global, then zeroes sixteen longwords per entry across twelve entries starting at a fixed address.
- `blit_sprite_and_advance_source`: Calls a helper, programs blitter pointer, modulo, control and size registers from a0 and a1, then returns; later code blits successive source words into a destination buffer.
- `update_animation_frame_and_spawn`: Advances a frame counter, toggles animation state, and when a position check passes writes sprite control words and pointers into a display list entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spawn_enemy_sprite_and_advance_animation`: Reads enemy state fields at offsets 28 and 30 of a6, toggles direction via jsr calls, advances the animation counter at offset 12, and writes sprite descriptors to the display list pointed to by the global at 3edbc.
- `advance_enemy_frame_and_emit_sprite`: Increments the enemy frame counter at offset 6 of a6, wraps it at 44, computes the screen position, calls the sprite setup routine and appends a sprite descriptor to the copper list.
- `zero_filled_data_area_5`: Eleven consecutive zero words that decode as ori.b #0,d0; this range is a zero-filled data area referenced by address, not executable code.
- `poll_audio3_toggle_and_return_d0`: Checks two flag words in memory, copies a word between two globals, and when the new value is zero programs AUD3 pointer, length and volume before returning the saved d0.
- `start_audio2_sample_and_pitch`: Enables DMA channel 2, sets AUD2 pointer, saves d0, calls a routine, then writes a pitch derived from the low byte of the result plus 400 into AUD2PER and sets length and volume.
- `start_audio1_sample_with_pitch`: Enables DMA channel 1, sets AUD1 pointer, period and volume, saves d0, computes a pitch from a memory word shifted left three plus 20, writes it to AUD1PER and returns d0.
- `set_aud0_voice_for_first_sound`: Writes DMACON to enable DMA, then sets AUD0 location, length, period and volume for the first sound sample and returns.
- `set_aud0_voice_two_sound_variants`: Contains two entry points that each enable DMA and program AUD0 location, length, period and volume for a different sound sample, then return.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_sound_block_and_play_second_variant`: Copies a block of longwords from a source pointer into a destination buffer, adds a word offset, then programs AUD0 with the second sound sample and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_d0_five_call_init_and_jump`: Loads immediate 5 into d0, calls two absolute subroutines, then jumps to a fixed address to hand off control.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_joystick_until_fire_or_timeout`: Calls joystick read and status routines, stores d4 to memory, loops up to 800 times checking a status flag, then returns.
- `cia_joy_dma_setup_block`: Reads keyboard serial and mouse port data, writes DMA control and copper list pointers, and calls five subroutines to configure hardware.
- `append_byte_to_pointer_buffer`: Checks a flag word; if clear and the write pointer differs from the buffer end, stores the low byte of d0 at the pointer, increments it and saves it back.
- `clear_flag_and_return_if_set`: Clears the word at the flag address, tests it, and returns immediately if it is zero; otherwise executes four no-op ori instructions and returns.
- `build_playfield_and_clear_screen_buffers`: Sets two flag bytes, stores a pointer, expands 32 packed triples into words, then fills two screen regions with generated bytes and copied rows before saving a word.
- `decode_run_length_encoded_byte`: Uses the global source pointer and repeat counter to fetch the next byte, expanding a run-length encoded stream: it reloads the counter when exhausted, negates a high-bit count, and returns the current byte in d0.
- `main_game_init_and_frame_loop`: Initializes game state and hardware registers including CIAA serial, copper list and DMA control, then runs the main frame loop calling subroutines and polling input.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `timed_delay_calling_sub_at_two_counts`: Loops d3 from 24576 down to zero, calling the routine at 3a4f8 when the counter equals 8192 and again at 16384, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_a3_nonzero_spin`: Tests the longword at address a3 and branches back to the preceding instruction while it is nonzero, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_game_state_and_vectors`: Writes initial constants to many global words and longs, sets two interrupt vector pointers from a long in d0, clears a flag and stores a return address, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_32000_block_and_fill_random_words`: Clears 24577 bytes starting at address 32000, then calls a generator 70 times to store three words per iteration, masking the third with 511, into memory after 7c12e.
- `plot_star_positions_and_set_pixels`: Loops 69 times over a table of star records, divides x and y by depth, offsets them, stores screen coordinates, calls plot helpers and sets a bit in the bitmap at offset d1.
- `set_pixel_bit_in_first_bitmap`: Adds d1 to the first bitmap base, sets the bit selected by d2 in that byte, then returns to the caller.
- `clear_pixel_bits_in_both_bitmaps`: Calls the coordinate helper, then clears the d2-selected bit in the byte at d1 offset in both the first and second bitmap buffers.
- `toggle_word_at_a3_with_vhposr`: Reads the vertical position register into d0, multiplies it by the word pointed to by a3, inverts the low word and stores it back through a3.
- `compute_row_offset_from_d0_d1`: Multiplies d1 by forty, adds d0 shifted right three bits, then leaves d2 as d0 minus eight times that shift minus one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_dma_clear_bitmap_and_call_library`: Enables DMA, clears a block of memory at the bitmap area, calls a library vector, adjusts fields in the pointed structures and increments a counter byte.
- `poll_fire_buttons_and_start_copper`: Reads the CIA-A port A fire button bits, programs the copper list pointers and blitter registers, and starts a copper or blitter operation using the vertical position register.
- `keyboard_serial_scan_and_dispatch`: Saves all registers, calls the keyboard scan helper, reads the CIA-A serial data register, counts calls and jumps to the main game loop at a fixed address.
- `blitter_copper_setup_every_fourth_call`: Increments a counter, and every fourth call clears it, advances a pointer, sets up blitter pointers, control and size registers, then returns.
- `fill_five_by_thirtythree_tile_grid`: Clears d0, then calls a helper thirty-three times per row for five rows, stepping a3 backward by 335 bytes after each row to fill a tile grid.
- `copy_eight_bytes_down_column`: Reads a byte from (a0) as an offset, then copies eight bytes from a2 minus 32 into a3, advancing a3 by 42 bytes each time.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_display_dma_and_copper_lists`: Calls the main image, enables display DMA via DMACON, stores three pointer values into a1 offsets, then calls a library function.
- `check_flag_and_jump_to_vector`: Tests the byte at absolute address a fixed address; if zero it branches ahead, otherwise it executes padding no-ops, copies the word at a fixed address to a fixed address, then jumps to the address stored at a fixed address.
