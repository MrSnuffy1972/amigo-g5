# Game 1376 (storm master (1991)(silmarils)(de)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01376). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901280 top-level bytes in 48 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 339 of 339 named (0 author symbols), 339 of 339 described (41 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1607 of 1615 (99.50%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6835 of 7746 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 38686 |
| code | 31446 |
| data | 768454 |
| padding | 9958 |
| unused | 52736 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Storm Master (1991)(Silmarils)(De).adf:t.x | 4 | 7724 | 337 |
| Storm Master (1991)(Silmarils)(De).adf:bootblock | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:FINISH.CO: high-entropy data file (7.64 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:CONSEIL.CO: high-entropy data file (7.75 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:EVEQUE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:TLEO.CO: high-entropy data file (7.71 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:MAIN.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:AVENT.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:VOL.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:ACCUEIL.CO: high-entropy data file (7.71 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:MASTER.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:SANCHO.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:APROJEC.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:COMMER.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:POLIT.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:PARCH.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:BVENT.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:OFFICIER.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:BOSTROM.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:MC2.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:ANUAGE.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:TEXTE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:MESSE.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:LEO.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:LOGO.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:TEXTE2.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:FINISHG.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:PRES3.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:EXPLOSE.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:CARTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:ASHIP.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- storm master (1991)(silmarils)(de)/Storm Master (1991)(Silmarils)(De).adf:PANNEAU.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- reference crashes at an address (HLE illegal instruction an address at an address: through vector 0, as before (vector 4 is 0 again)): 114654 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 1 packed track region(s), 11264 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `save_and_reset_custom_chips` (t.x): Reads ADKCON, INTENA, DMACON and ADKCONR into globals, then writes interrupt, DMA and disk control values to the custom chips and CIAB port B. Kind: hardware state preservation/reset. Pass path: the execution continues to the next instruction in the caller (bsr.w an address, etc.).
- `floppy_select_and_step_delay` (t.x): Clears bit 7 then bit 3 of CIAB port B to deselect and step the floppy drive, busy-waits 101 iterations, then calls the delay and fire-button wait routines before returning. Kind: disk hardware/timing check. Pass path: the routine continues to the next instruction (bsr.w an address or bsr.w an address).
- `poll_fire_and_pulse_disk_select` (t.x): Tests CIAA_PRA bit 4 and, when clear, sets and clears CIAB_PRB bit 1 and bit 0 with delays, calls the step routine, then branches back to re-test. Kind: disk hardware/timing check. Pass path: the loop terminates when the bitwise condition is no longer met, allowing execution to proceed to the next instruction after the call site.
- `disk_step_loop_79_times` (t.x): Loads 79 into d2, then repeatedly clears CIAB_PRB bits 1 and 0, sets bit 0, calls the step routine and decrements d2 until zero. Kind: disk hardware/sync check. Pass path: the loop completes and execution continues to the delay loop.
- `reset_ciab_and_poll_status_bit` (t.x): Calls two setup routines, clears bit 2 of CIAB port B, writes control words to offsets 150, 32, 36 and 156 of a6, then polls bit 1 of offset 30 until set or timeout. Kind: disk hardware status/sync check. Pass path: the loop continues (iVar1 reaches 0) and the function continues to the next iteration of the do-while loop.

## Functions


### Storm Master (1991)(Silmarils)(De).adf:t.x

- `jump_to_main_entry`: Jumps directly to the main program entry point, transferring execution without saving any registers or returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `open_libraries_and_init_memory`: Opens dos, graphics and intuition libraries, checks their version, allocates memory, computes free memory and calls several initialization routines before returning.
- `open_keyboard_device_and_port`: Finds the current task, adds a message port, opens keyboard.device with the port and returns zero on success or nonzero on failure.
- `call_four_init_subroutines`: Calls four fixed subroutines in sequence via absolute jumps, then returns to the caller; no registers or memory are touched directly here.
- `store_joy0dat_to_mouse_variable`: Reads the JOY0DAT hardware register at the mouse port and copies its word value into the global mouse movement variable, then returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `init_screen_buffers_and_window`: Reads a screen pointer from the structure, derives several buffer pointers with fixed offsets, allocates memory for buffers and a bitmap, opens a window, sets the pointer, and initializes view and clipping globals before returning.
- `copy_overlay_and_start_copper`: Copies the overlay block to the destination pointer, sets up the copper list pointers, calls the setup routine, then Forbids, writes COP1LCH and DMACON to start the copper, Permits and returns.
- `zero_filled_data_area`: Two zero words that decode as ori.b #0,d0; this range is a zero-filled data area referenced by address, not executable code.
- `find_word_index_in_table`: Scans a table of words for the value in d0, returning its index in d0, or minus one in d0 and d1 when not found, and minus two in d1 when the table end is reached.
- `skip_negative_or_zero_index`: Tests the word in d0 and returns immediately if it is negative or zero, otherwise also returns without touching any other register or memory.
- `copy_four_scaled_words_to_sprite`: Saves all registers, looks up a table entry indexed by d0, multiplies a global scale into four words read from a0 and writes high and low halves into sprite memory, then restores registers.
- `build_eight_sprite_descriptors`: Saves registers, walks eight word entries from a table, writes sprite control bytes and fills descriptor records with sprite pointers, positions and values, then calls two setup routines.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `close_graphics_library_via_a6`: Saves d0 and a1, loads the graphics library base from absolute address 248 into a1, clears d0, then calls the graphics library close function through a6 offset -234 and restores registers.
- `wait_for_copper_interrupt_bit4`: Polls the custom chip interrupt request register at offset 29 until bit 4 becomes set, then returns; used as a busy wait for a copper interrupt.
- `wait_for_vertical_blank_bit0`: Saves d0, reads the custom chip vertical position register at offset 4 and loops until bit 0 is set, then restores d0 and returns.
- `enable_audio_interrupt_and_init`: Calls two setup routines, then writes 16 to the INTENA register to enable the audio interrupt, and returns.
- `scale_and_store_vertex_coordinates`: Saves all registers, then for three groups of four entries multiplies a scale word by each coordinate, adds an offset, and stores the high and low words into the destination structure.
- `start_sound_and_install_vblank_handler`: Sets the CIAA fire bit, writes sound parameters, calls sound routines, enables interrupts, swaps in a new vertical blank handler via exec, and restores the old handler.
- `disable_dma_intena_and_swap_superstate`: Calls two setup routines, disables DMA and interrupt enable bits, saves the old supervisor state pointer, stores the new state, then restores the previous supervisor state and returns.
- `clear_flags_install_level6_vector`: Clears three flag bytes, saves the supervisor state, stores the old level 6 interrupt vector and installs a new one, then restores the supervisor state and returns.
- `restore_level6_vector_and_superstate`: Saves the supervisor state, restores the saved level 6 interrupt vector from memory, then restores the previous supervisor state and returns.
- `increment_counter_and_call_three_services`: Saves all registers, increments the word counter at a fixed global, calls a routine three times via dbra, then calls three absolute service routines before restoring registers and returning.
- `decrement_channel_volume_and_disable_dma`: Returns early if a flag byte is set; otherwise scales d3 by sixteen, decrements a channel counter byte, and when it reaches zero disables that audio channel's interrupt and DMA and optionally re-enables interrupt.
- `update_audio_channel_period_and_volume`: Checks a global flag, indexes channel table by d3, reads a3 fields to compute period and volume, writes AUDxPER, AUDxVOL, AUDxLC and AUDxLEN, and sets DMACON bits for the channel.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `return_main_ao_pointer`: Loads the address of the MAIN.AO data area into a0 and returns it to the caller.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `decode_compressed_tile_rows`: Walks a list of tile records, expands each run-length encoded row into the four bitplanes and writes the decoded bytes back into the record.
- `blit_tile_planes_to_screen`: Iterates tile records, copies or ORs their four bitplane words into the screen buffers, and updates the record data in place.
- `return_constant`: Returns the constant 3000 in d0 to the caller without any other effect.
- `return_constant_2`: Returns the constant 100 in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant 0 in d0 to the caller without any other effect.
- `read_ciaa_tod_low_bytes`: Reads the low byte of CIA-A time-of-day from its two registers, shifts the first byte left by eight bits and returns the combined value in d0.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `copy_four_sprite_descriptors_to_buffers`: Saves all registers, then for up to four entries in a table copies sixteen words from each source pointer to the destination pointer, plus sixteen more words for the first entry only.
- `call_write_pixel_via_graphics_library`: Saves registers, calls an internal routine, then loads the graphics library base and jumps to its WritePixel vector before restoring registers and returning.
- `draw_and_move_graphics_primitive`: Saves d0-d3/a1, calls the coordinate clamp routine, then calls graphics Draw with a1 from the base pointer and graphics Move using d2/d3 as coordinates, restoring registers.
- `rectfill_with_clamped_coordinates`: Saves d0-d3/a1, calls the coordinate clamp routine, then calls graphics RectFill through a6 with a1 from the base pointer, restoring registers before returning.
- `clamp_and_order_rectangle_coordinates`: Orders d0/d2 and d1/d3 by swapping when needed, then clamps each coordinate to zero and to the screen limits 319 and 199, returning the adjusted values in d0-d3.
- `set_apen_and_store_pen_byte`: Saves d0 and a1, stores the low byte of d0 at a global pen byte, then calls graphics SetAPen through the library base at a6 and restores registers.
- `set_bpen_and_store_pen_byte`: Saves d0 and a1, stores the low byte of d0 at a global background pen byte, then calls graphics SetBPen via the library base and restores registers.
- `set_drmd_remap_mode_three_to_four`: Saves d0 and a1, changes a draw mode value of three to four, stores the byte globally, then calls graphics SetDrMd and restores registers.
- `wait_for_key_press_then_release`: Calls the key-poll routine repeatedly until it reports a key down, then calls it again until the key is released, preserving d0 across the second wait.
- `draw_typed_character_at_cursor`: Polls keyboard input, handles carriage return and newline by moving the cursor, otherwise stores the character, calls graphics Move and Text to draw it, and advances the cursor position.
- `draw_null_terminated_string`: Walks the string pointed to by a0, calling the character-drawing routine for each byte until a zero terminator is reached, then returns.
- `clamp_and_scale_grid_coordinates`: Clamps d0 to at most 39 and d1 to at most 24, then stores d0 times 9 and d1 times 8 into two longword globals before restoring registers.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `read_mouse_and_update_cursor_position`: Reads JOY0DAT mouse movement, clears JOYTEST, applies deltas to stored cursor x and y, clamps x to 639 and y to 399, and saves the button bits.
- `double_and_store_cursor_coordinates`: Doubles d0 and d1 and writes the results into the two cursor coordinate globals, then returns to the caller.
- `read_fire_buttons_and_scale_axes`: Reads CIA-A port A bit 6 and POTGOR bit 2 to set d2 button flags, then loads two axis words, halves each and returns them in d0 and d1.
- `match_and_shift_input_bitmap`: Calls the input poll, compares 13 bytes against a shifted copy, finds the first differing bit, maps it through a table into the input byte and shifts the bitmap, returning the byte in d0.
- `poll_joystick_and_build_direction_byte`: Calls exec DoIO on the input request, then tests bits of the joystick state at offset 12 and builds a direction byte in d0, storing it to the input byte.
- `read_fire_buttons_into_d0`: Saves d1-d2 on the stack, reads the CIA-A port A byte into d2, tests the low byte of d0 and branches onward to the shared joystick decoding code.
- `read_joy0dat_into_d1_shift_d2`: Reads the JOY0DAT word into d1, shifts the low byte of d2 left by one bit and branches to the shared joystick decoding continuation.
- `decode_joy1dat_directions_into_d0`: Reads the JOY1DAT word into d1, clears d0, then sets direction bits in d0 based on the joystick port one movement bits and branches to the shared continuation.
- `set_d0_bits_from_d1_flags`: Tests bits 0, 9 and 8 of d1 and sets bits 2 and 0 of d0 accordingly, then branches to shared exit code.
- `finish_d0_bits_and_restore_registers`: Tests bit 8 of d1 and the sign of d2, sets bit 7 of d0 if needed, then pops d1-d2 and returns.
- `clear_d0_and_return`: Sets the word in d0 to zero and returns immediately to the caller.
- `open_dos_file_via_library`: Calls a setup routine, then invokes dos.Open through the library base at offset 216 with mode 1004, returning -1 if the open fails and restoring d1-d2/a0-a1.
- `return_zero_constant`: Clears d0 to zero and returns immediately, providing a constant zero result to its caller.
- `lock_and_unlock_dos_file`: Calls dos.Lock with mode -2 through the library base at offset 216; if the lock succeeds it calls dos.UnLock on the returned handle, then clears d0 and restores registers.
- `close_sound_file_and_reset_flag`: Closes the DOS file handle, checks the sound-enabled byte, opens BLANC.SND, closes it again, clears the sound counter word and returns zero.
- `jump_to_sound_init_entry`: Unconditionally jumps to the sound initialization routine, transferring control without saving registers or returning here.
- `open_sound_file_by_name`: Calls the sound setup routine, then opens the file named by a0 with access mode 1006 through dos.Open, returning -1 on failure.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `dos_read_via_a6_vector`: Saves d2-d3/a1, moves d1 to d3, d0 to d1 and a1 to d2, then calls the dos.Read vector at offset -42 from the a6 library base stored at address 216.
- `write_then_set_disk_flag`: Calls the routine at 6416, then invokes the dos.Write vector at offset -48 from the a6 base at address 216, and finally stores byte 1 at address 932.
- `disk_protection_wait_prompt_loop`: Saves all registers, polls status words, and if the disk is not ready shows the please-wait and unprotect-disk messages, waits for Return, then loops until the counter reaches 400.
- `copy_playfield_to_buffer_and_clear_source`: Saves all registers, then copies four 20-word rows from four source pointers into a destination buffer while zeroing each source word, advancing all pointers by 40 bytes per row for 17 rows.
- `restore_playfield_from_buffer`: Saves all registers, then copies four 20-word rows from a buffer back into four destination pointers, advancing each pointer by 40 bytes per row for 17 rows.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `poll_message_for_high_bit_flags`: Calls exec GetMsg on a port, inspects the returned message's 32-bit field for bits 15 and 17, and loops polling until a message without bit 15 is found.
- `pop_all_registers_and_return`: Restores every data and address register from the stack and returns immediately to the caller.
- `lock_current_directory_and_store_lock`: Calls dos.Lock on the current directory, stores the returned lock in global memory, then calls dos.CurrentDir on it; on failure it jumps to error handling.
- `return_zero_constant_2`: Clears d0 to zero and returns, providing a constant zero result to the caller.
- `compute_remaining_time_percent`: Calls a helper with d0, then if a global is not minus one, computes elapsed ticks times a word, divides by 1000, subtracts 2 and clamps negative results to zero in d0.
- `lock_and_scan_dos_device`: Selects a DF0, DF1 or DH0 name string from d0, calls dos.Lock, stores the lock, calls dos.Info with a callback and dos.UnLock, returning minus one on failure.
- `zero_filled_data_area_2`: Two zero words that decode as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_1ce0`: A single zero word that decodes as ori.b with immediate zero to d0; it is data, not executable code, referenced by an address pointer.
- `zero_filled_data_word_at_1ce4`: One zero word stored at the next data slot, decoding as ori.b immediate zero to d0; a data byte pair, not a real instruction.
- `zero_filled_data_pair_at_1ce8`: Two consecutive zero words that decode as ori.b immediate zero to d0 twice; this is zero-filled data referenced by an address, not code.
- `zero_filled_data_words_at_handler_slot`: Two zero words that decode as ori.b #0,d0; this is zero-filled data referenced by an address, not executable code.
- `zero_filled_data_words_second_slot`: Another two zero words decoding as ori.b #0,d0; a zero-filled data area reached through a handler pointer, not code.
- `save_all_registers_check_timer_and_vblank`: Pushes d0-d7 and a0-a6 onto the stack, reads the low memory timer into d0, tests a flag word and compares d0 with the vertical blank counter before branching.
- `swap_screen_buffers_with_blank_palette`: Compares d0 with the stored value; if different, saves five display words, sets blank copper values, calls the display routine, swaps the two screen buffer pointers, then calls the update routines and restores registers.
- `animate_rotating_playfield_columns`: Saves registers, swaps the two playfield buffer pointers, copies five longwords between them, adjusts column offsets and scroll values, then repeatedly calls the blit and scroll routines to rotate the field.
- `apply_vertical_scroll_shift_to_buffer`: Saves registers, computes a shift from the scroll direction word, adds it to five longwords of the buffer at the current buffer pointer, then calls the scroll routine with the stored value.
- `zero_filled_data_area_3`: Five zero words that decode as ori.b instructions but are actually a zero-filled data area referenced by pointers, not executable code.
- `single_zero_word_data_slot`: One zero word that decodes as an ori.b instruction but is a zero-filled data slot whose address is taken by other code.
- `init_display_params_and_call`: Saves all registers, clears two words at fixed memory, stores three display parameters, calls a routine at low memory, then restores registers and returns.
- `allocate_bitmap_and_clear_flag`: Saves all registers, computes a bitmap size from the a2 structure width and the global row range, calls the allocator, and clears a global byte flag if allocation fails.
- `store_bitmap_geometry_and_planes`: Computes row stride and plane sizes from the a2 width and global bounds, stores the bitmap pointer and plane addresses in globals, and writes header fields into the bitmap.
- `zero_filled_data_area_4`: Contains only a zero word that decodes as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `zero_word_data_at_221e`: A single zero word that decodes as ori.b with immediate zero on d0; it is zero-filled data referenced by address, not executable code.
- `zero_word_data_at_2222`: One zero word in a zero-filled data area, decoding as ori.b immediate zero to d0; other code takes its address but it is data.
- `zero_word_data_at_2226`: A lone zero word treated as a handler pointer target; the bytes are zero-filled data, not a real instruction sequence.
- `zero_filled_data_word`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `allocate_aligned_block_from_pool`: Saves d1 and a0, rounds the requested size up to a multiple of four plus header, subtracts it from the pool pointer, and returns the aligned block address or zero if the pool limit is passed.
- `return_zero_restore_d1_a0`: Clears d0 to zero as the return value, pops d1 and a0 from the stack and returns to the caller.
- `check_d0_in_audio_range`: Calls the audio setup routine, then returns zero in d0 if the returned word lies between the two immediate bounds, otherwise returns that word unchanged.
- `return_constant_4`: Returns the constant -1 in d0 to the caller without any other effect.
- `init_audio_and_disk_hardware`: Stores a0 in a global, sets a5 to the custom chip base, calls six setup routines for audio and disk, then returns the word in d6 as a long in d0.
- `save_and_reset_custom_chips`: Reads ADKCON, INTENA, DMACON and ADKCONR into globals, then writes interrupt, DMA and disk control values to the custom chips and CIAB port B.
- `floppy_select_and_step_delay`: Clears bit 7 then bit 3 of CIAB port B to deselect and step the floppy drive, busy-waits 101 iterations, then calls the delay and fire-button wait routines before returning.
- `short_busy_wait_3000`: Loads 3000 into d6 and decrements it in a dbra loop, producing a short busy-wait delay before returning to the caller.
- `wait_for_fire_button_release`: Polls bit 5 of CIAA port A, which reflects the joystick fire button, looping until that bit becomes clear, then returns.
- `poll_fire_and_pulse_disk_select`: Tests CIAA_PRA bit 4 and, when clear, sets and clears CIAB_PRB bit 1 and bit 0 with delays, calls the step routine, then branches back to re-test.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_fire_then_step_disk`: Tests CIAA_PRA bit 4 and, while it is clear, calls the disk step routine and branches back to test the bit again.
- `disk_step_loop_79_times`: Loads 79 into d2, then repeatedly clears CIAB_PRB bits 1 and 0, sets bit 0, calls the step routine and decrements d2 until zero.
- `delay_15000_iterations`: Loads 15000 into d0 and decrements it until it reaches minus one, creating a busy-wait delay, then returns to the caller.
- `reset_ciab_and_poll_status_bit`: Calls two setup routines, clears bit 2 of CIAB port B, writes control words to offsets 150, 32, 36 and 156 of a6, then polls bit 1 of offset 30 until set or timeout.
- `clear_offset24_set_offset96_16`: Writes zero to word offset 36 of a6 and the value 16 to word offset 150 of a6, then returns to the caller.
- `wait_ciab_fire_button_release`: Reads CIAB interrupt control register at a fixed address twice into d6 and loops until bit 4 is set, then returns to the caller.
- `select_drive_and_load_track_params`: Sets CIAB port B bits to select the drive, then copies four track parameter words from fixed memory into offsets 150, 154, 156 and 158 of a6 with bit 15 set.
- `scan_bitmap_for_sync_marker`: Reads words from the buffer pointed to by the global pointer, shifting each left and testing the high word for the sync value, up to sixteen shifts per word and many words; sets a flag byte when not found.
- `scan_audio_stream_for_sync_word`: Clears d6, points a4 at the audio buffer, calls the sync scanner, advances a0 by two, adds d5 to d6, and subtracts d5 again if the found flag is clear.
- `find_sync_word_in_word_stream`: Reads words from a0, shifts each through 16 bit positions looking for the sync pattern, stores the match index through a4 and sets the found flag, or clears the flag when d3 words are exhausted.
- `store_sync_index_and_set_found_flag`: Compares a4 with the buffer end, returns if equal, otherwise writes d5 to the word at a4, advances a4, sets the found flag byte and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_absolute_address_2d8`: Jumps directly to the absolute address 728, transferring execution without saving any registers or return address.
- `install_vectors_and_run_object_loop`: Stores the stack pointer and several handler addresses into globals, calls three setup routines, then walks the object list at the global pointer, updating each object's fields and calling movement and drawing routines.
- `dispatch_byte_opcode_via_a2_table`: Pushes the return address, reads a byte from a3, doubles it, loads a word offset from the a2 table and jumps to that entry.
- `fetch_three_bytes_and_push_return_address`: Reads three bytes from the stream at a3, builds a long value from them, decrements d4 by four, stores the advanced a3 pointer at a6 plus d4, adds the value to a3 and returns.
- `fetch_three_bytes_advance_stream`: Reads three bytes from the stream at a3, assembles them into a long value in d0, adds that value to a3 to skip ahead and returns.
- `check_flag_and_fetch_stream_pointer`: Tests the byte flag at memory location 6d71, branches away if set, compares d4 with the word at 6dae, loads the pointer at a6 plus d4 into a3, clears the word at a6 minus twelve when d4 matches, increments d4 by four and returns.
- `save_a3_and_advance_byte_stream`: Stores a3 to global pointer, increments a3, sets d7 to 255, calls a subroutine; if nonzero returns, else reloads a3 from global, reads a signed byte into d0 and advances a3 by it.
- `read_three_bytes_or_skip_three`: Tests d7; if zero reads three bytes from (a3) into d0 and advances a3 by the third byte, otherwise adds three to a3 and returns.
- `call_subroutine_at_offset`: Branches to subroutine with bsr.w, pushing the return address and transferring control to the called routine.
- `swap_6db0_6db8_and_dispatch_via_884`: Exchanges the longwords at memory locations 6db0 and 6db8, then reads a byte from (a3)+, indexes a word table at 884 and jumps through it.
- `jump_to_4cda`: An empty routine that immediately jumps to the fixed address 4cda, transferring control without touching any registers or memory.
- `swap_6db0_6db8_and_dispatch_via_8c0`: Exchanges the longwords at memory locations 6db0 and 6db8, then reads a byte from (a3)+, indexes a word table at 8c0 and jumps through it.
- `sample_five_input_parameters`: Calls the same sampling routine five times and stores each returned d7 word or byte into five consecutive global input-parameter slots.
- `sample_input_with_default_100`: Calls the sampling routine, and if the result is zero and a flag byte is set, substitutes 100 before storing the word to the input parameter slot.
- `dispatch_key_by_input_mode`: Reads the input mode byte and branches to one of three key-handling paths that update cursor or text position globals based on the key in d0.
- `print_zero_terminated_string_via_a0`: Saves d0 and a0, loads the string pointer from memory location 6db0, calls the print routine, then loops reading bytes from a0 and printing each until a zero byte, restoring registers and returning.
- `format_signed_word_to_decimal_string`: Converts the sign-extended word in d0 into a decimal ASCII string written through a0, prepending a minus sign for negatives, using a power-of-ten table and suppressing leading zeros, then null-terminates.
- `dispatch_script_command_stream_from_a3`: Reads a big-endian word command from a3, optionally calls a routine with a0 pointing at the following bytes, skips a null-terminated argument, and either returns or calls two routines and jumps back into the main loop.
- `spawn_object_from_stream_data`: Reads a word index and thirty-two bytes from the stream at a3, marks the object slot active, copies the bytes, links it into the free list and initializes its position and size fields before calling two setup routines.
- `copy_entity_type_to_pool_slot`: Saves d1 and a1, loads the pool base from memory, reads the slot index at offset 2 of a0 and copies the byte at offset 1 of a0 into that slot's type field, then restores registers and returns.
- `set_flag_if_zero`: Tests the byte at address f7; if it is zero, writes 1 to the byte at address 6d97, then returns.
- `unlink_index_from_free_list`: Clears the word at offset 4 plus d0 in the a0 table, then either stores d0 in the global list head or walks the chain at offset 4 until a zero link and appends d0 there.
- `compute_cross_product_into_offsets`: Reads signed bytes at offsets 32 to 37 of the a0 structure, multiplies and subtracts them, and stores the three cross-product results as bytes at offsets 38, 39 and 40.
- `clear_global_byte_flag`: Clears the single byte at the global address to zero and returns immediately; it is an empty flag-reset routine with no other effect.
- `spawn_entity_from_script_stream`: Parses a script byte stream, saves and restores the global position and flip state, recurses for repeated entries, then fills a new entity record at a1+d1 with type, position, sprite and animation fields.
- `advance_to_next_entity_slot`: Copies d1 into d2, loads the word at offset four from a1+d1 into d1 and branches away if that word is zero.
- `match_entity_type_and_owner`: Compares the word at offset two and the byte at offset one of the entity at a1+d1 against the global type and owner values, returning only when both match.
- `find_matching_list_entry`: Walks a linked list in a1 starting at offset -24(a6), comparing each node's word at offset 2 and byte at offset 1 against -22(a6) and a global byte, returning when a match or list end is found.
- `unlink_sprite_node_and_append`: Reads the head index from the sprite list pointer, unlinks that node, relinks it after the node in d2 or stores it in the frame slot, then fills the node fields from a6 offsets and the sprite table.
- `call_4cdc_and_store_d7_byte`: Calls the routine at 4cdc, then stores the low byte of d7 into -43(a6) and returns to the caller.
- `compute_sprite_screen_position`: Looks up a sprite record via a1 and d1, subtracts its origin from the object position, scales by depth, walks the sprite chain, applies damage scaling, and stores the resulting screen x, y and depth in the globals.
- `apply_rotation_matrix_to_offset`: If the object's flag byte at offset 1 is negative, multiplies the d4, d5 and d6 offsets by the three-by-three matrix bytes at offsets 32 to 40 of a0 and returns the rotated values in d4, d5 and d6.
- `test_global_flag_and_branch`: Tests the byte at the fixed global address 6d6d and branches away when it is non-zero, otherwise falling through to the following code.
- `lookup_indexed_entry_pointer`: Pushes d1, reads a table base from the a6 structure, compares d7 against the table count and returns the indexed entry pointer in d0 and d2, or calls the error routine with code 15.
- `clear_flag_if_no_pending_entry`: Loads the a1 table pointer, clears d2, reads the word at -24(a6) into d1 and, when it is zero, clears the byte flag at the global address and returns.
- `mark_and_relink_table_entry`: Checks the global flag and the byte at a1+d1, marks it, moves the word at a1+4+d1 into the a6 slot or the d2 entry, updates the global head and calls the relink routine.
- `relink_sorted_index_entries`: Reads a key from the table at a1 indexed by d1, follows the global pointer table to compare and swap the linked index entries, returning the updated link in d0.
- `save_a6_pointer_pair_to_globals`: Copies the two longwords at offsets zero and four of a6 into the global variables at the fixed addresses, then returns.
- `apply_position_delta_to_active_objects`: Subtracts the saved origin from a6 coordinates, then for each nonzero object index in the table adds the deltas to its position words and marks it active.
- `advance_object_position_and_mark_cell`: Adds three velocity words from offset 42 of a2 into the position words at offsets 22, 24 and 26, then follows the d3-indexed link in a1 and marks the target cell byte with value 2 if unset, finally clearing bit 7 of the byte at a2.
- `main_frame_update_dispatch`: Checks several global flags, runs the input and sound subroutines, advances the frame counter and object list pointer, then walks the object list calling the per-object update for entries without bit 6 set and swaps the two display buffers.
- `process_object_action_script`: Saves all registers, reads the action record at a2, sets up the bounding box globals, then walks the linked object list applying move, copy and delete actions according to the record type bytes before restoring registers and returning.
- `copy_entity_template_to_slot`: Calls a setup routine, copies a word between two entity slots, then writes four global template values into the slot indexed by d1 and calls two more routines before returning d2 in d1.
- `link_slot_and_store_previous_index`: Calls a setup routine, copies a word between two entity slots, saves the global slot index into the slot at offset four, updates that global to d1 and returns d2 in d1.
- `dispatch_entity_by_type_byte`: Calls a setup routine, walks the entity chain from d1, compares type and flag bytes against d4, and branches to the copy, link or recursive dispatch routines before restoring registers and jumping back.
- `reset_bounds_to_extremes`: Writes the four fixed bound words at the shared bound addresses to their initial maximum and minimum values, then returns.
- `expand_bounds_from_entry`: Reads an entry's flag, pointer and two coordinates, follows the pointed offset table and widens the four shared bound words to include the entry's extents.
- `insert_entry_into_sorted_chain`: Walks the linked chain from d3 comparing depth and two bytes, then relinks the entry at d1 into the chain, clears its flag byte and updates d2 when the head changed.
- `process_entity_list_and_update_state`: Saves all registers, clears a flag byte, walks the entity list at the pointer table using word offsets, calls several subroutines to test and update each entity, then restores registers and returns.
- `append_four_words_to_buffer`: Copies four global words into the buffer pointed to by the buffer pointer, advances that pointer by eight bytes, and if it passed the limit calls a routine with d0 set to 22.
- `copy_table_words_to_globals`: Saves all registers, reads four words from the table at address 1856 into the four global words, calls the routine at address 768, and repeats while the table pointer stays below the buffer pointer.
- `clip_sprite_bounds_to_screen`: Clears the clip-valid flag, reads the sprite rectangle at offsets 12,14,22,24 from a1+d1, rejects it if outside the screen limits, otherwise clamps the four edges and stores them to the clip rectangle globals, setting the valid flag.
- `clip_sprite_against_viewport`: Saves d1-d7, resolves the sprite and viewport pointers, rejects the sprite if flagged or outside the viewport, otherwise clamps its rectangle to the viewport bounds, stores the result to the clip globals and sets or clears the valid flag.
- `intersect_sprite_with_region`: Builds the sprite rectangle from a1+d1, tests it against the region at a1+d3, and if they overlap stores the intersection aligned to a sixteen-pixel grid plus the row count and screen pointer, returning without touching the clip flag.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_rle_bytes_to_a6_buffer`: Reads a length and count from the a3 stream, then copies count bytes into the a6 buffer at descending offsets, writing a word pair per iteration until dbra finishes.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `call_random_then_store_d7_byte`: Calls the random number routine, then stores the low byte of d7 into the global byte at the fixed address and returns.
- `call_random_mask_d7_store_flag`: Calls the random number routine, masks d7 with three, stores the low byte to one global byte and sets the next global byte to one.
- `select_sprite_bank_pointers`: Reads a bank index byte from global memory, scales it by thirty-two, and stores the resulting addresses of two sprite tables into global pointer variables before returning.
- `reset_sprite_bank_state`: Writes byte value 255 to one global byte and clears two other global bytes to zero, then returns to the caller.
- `clamp_nibble_sum_to_fifteen`: Masks d0 and d1 to their low nibbles, adds them into d1, and clamps the result to fifteen or zero before returning.
- `clamp_nibble_difference_to_range`: Masks d0 and d1 to their low nibbles, subtracts d0 from d1, and returns d1 as the difference clamped to zero or fifteen depending on sign and magnitude.
- `merge_sorted_nibble_arrays_into_target`: Walks two word arrays indexed by d7 from sixty-three down, compares each nibble of the source and destination words, calls the nibble ordering helpers, writes the merged word back, then invokes the array routine on the destination.
- `copy_sprite_data_unless_state_matches`: Calls a state check, returns early if the state byte equals one, five, three or seven, otherwise sets d0 to fifteen or thirty-one, points a0 at the sprite table and branches into the copy routine.
- `set_d0_one_call_3c9c`: Loads the immediate value 1 into the low word of d0, calls the routine at offset 3c9c, then returns to its caller.
- `set_d0_one_call_3cbe`: Loads the immediate value 1 into the low word of d0, calls the routine at offset 3cbe, then returns to its caller.
- `parse_a3_bytecode_dispatch`: Reads bytes from the a3 pointer, handling an end marker, a zero-prefixed word, or a nonzero byte, then calls the 4CE8 and 4CDC routines and jumps to 656A with d0 set.
- `branch_to_65fa`: Performs an unconditional branch to another routine, transferring execution without touching any registers or memory.
- `read_two_byte_pair_and_call`: Reads two big-endian byte pairs from the stream at a3, builds a word offset in d0 and d1, computes an address from a6 plus d0 into a1, then calls a routine.
- `init_scroll_bounds_from_globals`: Clears a long at 6d5c, copies two global longs into 6d72/6d76 and 6d8e/6d92, then computes a quarter of the height difference into 6d7a.
- `draw_sprite_sequence_with_offsets`: Saves the current sprite position and flip state, walks a list of offset entries updating the x, y and z globals, recurses for each entry, then writes the sprite pointer and adjusted coordinates into the object table and calls the render routine.
- `store_d7_into_sprite_slot`: Calls the shared setup routine, loads the sprite table base into a0 and stores the word in d7 at offset two of that table before returning.
- `call_setup_preserving_d7_in_d6`: Pushes d7, calls the shared setup routine, copies the resulting d7 into d6, restores the original d7 from the stack and returns.
- `copy_d7_to_d6`: Copies the low word of d7 into the low word of d6, then falls through into the following routine.
- `dispatch_byte_via_a5_table`: Clears d0, loads the next byte from (a3) into d0, reads a word offset from the a5 table at that index and jumps to the resulting address.
- `swap_two_globals_then_dispatch`: Exchanges the longwords at the two global addresses through a0, then branches to the byte dispatch routine using a3 and a5.
- `read_signed_byte_from_a3`: Reads one byte from the address in a3, increments a3, sign-extends the byte into d7 as a word and returns.
- `read_big_endian_word_into_d7`: Reads two bytes from a3, shifts the first left by eight bits, merges the second into d7 as a big-endian word, advances a3 twice and returns.
- `read_indexed_byte_via_a6_table`: Reads a big-endian word from a3 into d0, uses it as an index into the byte table at a6, sign-extends the fetched byte into d7 and returns.
- `fetch_byte_indexed_table_to_d7`: Clears the low word of d0, loads the next byte from the a3 stream into d0, reads the byte at offset d0 from table a6 into d7 and sign-extends it to a word.
- `call_shared_routine_and_mask_d7_with_d6`: Calls the shared routine at the given target, then ANDs d7 with d6 to keep only the bits selected by d6, and returns.
- `call_4cda_and_or_d6_into_d7`: Calls the routine at the fixed target, then ORs the low word of d6 into d7 and returns to the caller.
- `compare_d7_equal_d6_set_flag`: Calls the shared comparison routine, then compares d7 with d6; if equal it clears d7 to zero, otherwise sets d7 to minus one, and returns.
- `compare_d6_less_than_d7_set_flag`: Calls the shared comparison routine, then compares d6 with d7; if d6 is less than d7 it clears d7 to zero, otherwise sets d7 to minus one, and returns.
- `compare_d6_greater_than_d7_set_flag`: Calls the shared comparison routine, then compares d6 with d7; if d6 is greater than d7 it clears d7 to zero, otherwise sets d7 to minus one, and returns.
- `read_byte_via_15b2_if_flag_clear`: Tests the byte flag at absolute address 6e6f; if zero, calls the routine at 15b2 and returns its byte result zero-extended in d7, otherwise returns d0's byte in d7.
- `read_byte_via_1676_if_flag_clear`: Tests the byte flag at absolute address 6e6f; if zero, calls the routine at 1676 and returns its byte result zero-extended in d7, otherwise returns d1's byte in d7.
- `dispatch_d7_to_game_state_value`: Tests d7 against small indices and letter codes, returning a computed value from global longs or words, or a count from a linked list, or a converted letter via a subroutine.
- `call_1152_store_result_in_d7`: Calls the routine at address 1152, then copies the returned word from d0 into d7 and returns to the caller.
- `push_d7_word_onto_a4_stack`: Pre-decrements a4 by two and stores the word in d7 at that address, then returns to the caller.
- `shift_d6_into_d7_and_advance_a4`: Copies d6 into d7, loads the next word from a4 into d6 while advancing a4 by two, then returns; trailing instructions are unreachable.
- `discard_four_bytes_and_return`: Adds four to the stack pointer a7, discarding four bytes of caller data, then returns immediately to the caller.
- `store_d7_byte_at_indexed_a6`: Clears d0, loads the next byte from a3 into d0, advances a3, then writes the low byte of d7 to the a6 base plus that index.
- `add_d7_byte_to_indexed_a6`: Clears d0, loads the next byte from a3 into d0, advances a3, then adds the low byte of d7 to the byte at a6 plus that index.
- `save_regs_call_init_then_clear_d2`: Calls a low-level routine, saves all registers on the stack, calls another routine, restores registers, clears d2 and calls a final routine before returning.
- `copy_path_strings_and_init_level_state`: Copies two strings into buffers, calls setup routines, then reads level parameters to compute and store pointers and counters in the game state block.
- `build_linked_sprite_list_from_base`: Uses the base pointer to lay out sprite entries with size and color words, links them by offset, and terminates the list at the end pointer.
- `load_and_parse_filename_into_buffer`: Copies a null-terminated string from a0 into the buffer at the pointer stored in the global, records the position after the dot, replaces that character with 'A' or 'C' depending on a flag, then branches into setup code based on d2.
- `decode_rle_stream_to_bitmap`: Saves all registers, sets a flag byte, checks the high nibble of the mode byte, and either calls the special decoder or loops reading run-length counts from a3 and writing decoded bytes through a5/a6 into the buffer at a4 plus 8000.
- `fetch_next_rle_byte_or_refill`: Returns the byte at a3 and advances a3, or when a3 has reached a4, resets a3 to the buffer at the global pointer after clearing 8000 bytes via the fill routine.
- `store_byte_at_a6_or_rewind_buffer`: Writes d0 to the address in a6 when it is below d6, otherwise recomputes a6 from a5 plus a stride minus d3 and clears d4 when the count runs out.
- `decode_run_length_encoded_block`: Copies two longs from the source pointer into a save area, then expands run-length encoded bytes into the buffer ending at a2, writing repeated or back-referenced bytes until the buffer is filled.
- `load_run_length_code_three`: Loads the immediate value 3 into d0 and falls through to the following routine, providing the run-length code selector for the decoder.
- `rotate_byte_into_d5_with_wrap`: Subtracts d0 from d7 and, depending on the sign, rotates d0 or d7 into the low word of d5, reading a word from (a0) when a0 is below a4, then returns.
- `save_regs_branch_to_shared_exit`: Pushes d0, d1, a0 and a1 onto the stack, then branches to a shared exit path that pops those registers and returns.
- `binary_search_word_in_pointer_table`: Searches a sorted table of word values pointed to by a0 for the value in d2, halving the index range each pass, and returns the matching index in d1 or minus one.
- `init_six_byte_slot_list`: Walks the slot table at the global pointer in six-byte steps, writing each offset into the slot and clearing the following long, then resets the count and index globals.
- `insert_sorted_slot_entry`: Advances the slot cursor by the given offset, inserts the entry pointer into the sorted slot list, bumps the count, and calls the update and sound routines when the count stays within the limit.
- `allocate_and_init_slot_block`: Carves a new block from the free memory pointer, links it into the slot table, zeroes its payload, fills the header fields from the source record, and returns the block pointer.
- `dispatch_input_bits_to_handlers`: Tests bits 8, 9 and 10 of d0 and calls the matching subroutines, storing the result of the file open call in the global at 6cf8 and returning d0.
- `sign_extend_d0_and_call_17a8`: Saves d0 on the stack, sign-extends the low word of d0 into a long, calls the routine at 17a8, restores d0 and returns.
- `retry_global_6cf8_until_nonnegative`: Loads the long at 6cf8 into d0, calls the routine at 17ec, and while the result is negative calls the routine at 69d8 with d0 set to 14 before retrying.
- `retry_until_positive_d0_via_186e`: Saves d0, sign-extends it, calls the routine at 186e, stores the result in memory at 6cf8, restores d0 and returns if non-negative; otherwise sets d0 to 8, calls 69d8 and retries.
- `retry_until_positive_d0_via_1868`: Saves d0, sign-extends it, calls the routine at 1868, stores the result in memory at 6cf8, restores d0 and returns if non-negative; otherwise sets d0 to 8, calls 69d8 and retries.
- `retry_until_positive_6cf8_via_18be`: Saves all registers, loads the value at 6cf8 into d0, calls the routine at 18be, tests the result, restores all registers and returns if non-negative; otherwise sets d0 to 13, calls 69d8 and retries.
- `update_timer_and_run_active_effects`: Sets a busy flag, advances a countdown timer, and when it expires calls the effect update, then tests four effect slots and runs each active one via the shared effect routine before clearing the busy flag.
- `poll_input_and_update_selection`: Checks a state byte, reads input into d2, stores it with a countdown, and when the countdown expires compares the new selection with the stored one and calls the selection update routines.
- `decrement_effect_timer_and_apply_motion`: Decrements the word timer at offset four of the structure in a3, adds velocity to position with clamping, updates the second coordinate, and calls the refresh routine or clears the fields when finished.
- `clear_four_ui_state_blocks`: Loads four consecutive state block addresses into a3 and calls the same clearing routine for each, then returns to the caller.
- `clear_state_block_fields`: Using a3 as a base, zeroes bytes at offsets zero, one and two and words at offsets six and ten, then returns.
- `update_timed_effect_counters`: Checks three global flags and counters, adjusts a global value by step amounts with clamping, decrements counters, and calls three subroutines to apply byte values or clear a flag.
- `error_dialog_and_mode_switch`: Clears a flag byte, calls text and number formatting helpers, swaps two pointers, indexes a jump table by the doubled result, and if the selection equals one sets display registers and jumps to the main loop.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `save_registers_call_check_and_restore`: Pushes all data and address registers, loads a pointer from absolute memory into a0, calls a subroutine, tests d0 and branches away if nonzero, otherwise pops all registers and returns.
- `advance_pointer_and_counter_restore`: Adds an address to a longword at absolute memory, adds two to another longword, then pops all data and address registers from the stack and returns.
- `blit_rect_from_globals`: Saves all registers, loads a3 from absolute address 16, reads four word globals into d4-d7 with d4 masked and d6 ORed, calls the blitter routine, then restores registers and returns.
- `blit_full_screen_region`: Loads a3 from absolute address 12, sets d4 and d5 to zero and d6/d7 to the screen bounds, calls the blitter routine, then returns.
- `blit_clipped_sprite_region`: Clips a sprite rectangle against globals, computes source and destination offsets, shifts four bitplanes by d5, masks and ORs them into the destination planes, and repeats per row via dbra.
- `blit_sprite_rows_to_five_bitplanes`: Reads sprite row words from a table, computes plane offsets from the row height and width, then copies each row into five bitplane pointers with modulo stepping for fifteen rows.
- `data_byte_ori_immediate_table`: A single ori.b immediate instruction whose bytes are data decoded as code, not an executable routine; it returns nothing.
- `blit_four_bitplane_columns_with_dma_wait`: Sets blitter window, modulo and control registers, computes source and destination offsets, then blits four bitplane columns while polling DMACONR busy bit and restores DMA.
- `blit_planar_columns_to_bitplanes`: Saves all registers, computes source and destination offsets from the global rectangle, programs blitter modulo, control and pointers, then blits each of five bitplanes in turn waiting on the blitter busy bit.
- `copy_planar_columns_to_chunky_buffer`: Checks the rectangle width and either calls the blitter routine or copies five bitplane words per column into a chunky buffer using offsets from the plane base pointers, advancing by the row stride.
- `masked_planar_sprite_blit_with_clipping`: Computes clipped source and destination offsets, sets blitter first and last word masks and modulo, then blits masked planar sprite data into the five bitplanes while waiting on the blitter busy bit.
- `setup_blitter_for_vertical_strip`: Configures blitter pointers, modulo, control and size registers from a0, d2, d4 and d7 to blit a vertical strip, then enables blitter DMA and returns.
- `render_mirrored_texture_columns`: Computes clipped column extents from a4 and a6, then loops copying bytes through a lookup table at a6 to write mirrored texture columns into memory and returns.
- `zero_filled_data_area_5`: This single zero word decodes as an ori.b immediate instruction, but the bytes are data, not executable code.
- `clip_sprite_bounds_and_dispatch`: Reads sprite data pointer from a0, clips its rectangle against the four global screen bounds, stores the clipped edges in globals, then branches on the sprite type byte to one of several handlers.
- `select_slot_by_a3_handler`: Saves all registers, checks the busy flag, then compares a3 against four known handler addresses to set d3 to a slot index and jumps to the shared dispatch.
- `copy_state_and_call_draw`: Copies a byte into the a3 structure, loads a1, d0, d1 and d4 from globals, clamps d2 between one and twenty, calls the draw routine, then restores registers and returns.
- `setup_audio_channel_dma_and_interrupt`: Saves all registers, calls exec SuperState, programs an audio channel's length, period, pointer, volume and interrupt bits in custom chip registers, enables the channel DMA and interrupt, then calls UserState and returns.
- `stop_audio_channel_if_active`: Checks the audio channel control byte for the channel in d3; if active, clears it and the paired byte, then disables that channel's DMA and interrupt bits in DMACON and INTENA.
- `audio_interrupt_dispatch_and_ack`: Reads INTENAR and INTREQR, tests interrupt bits 7 through 10, and for each set bit calls the matching audio channel handler with its channel number, then acknowledges the interrupts and returns from exception.
- `decrement_channel_counter_disable_dma_int`: Shifts d3 left four bits, decrements the word counter at a1 plus that offset, and when it reaches zero clears two flag bytes, then writes DMACON and INTENA to disable the channel's DMA and interrupt.
- `init_four_sound_channels`: Saves d3 and a3, then for channel numbers zero through three loads a3 with the matching channel structure and calls the sound setup routine with that channel index in d3.
- `dispatch_sound_command_by_coordinates`: Saves all registers, masks and shifts d1 and d0 to derive an index, points a1 at the sound table, and branches to the shared sound command routine before restoring registers and returning.
- `call_1936_with_d0_one`: Saves d0 on the stack, sets d0 to 1, calls the routine at 1936, restores d0 and returns to the caller.
- `mask_shift_d0_call_1794`: Saves d0, d1 and a0, masks d0 with 127, shifts it right by one, calls the routine at 1794, then restores the registers and returns.
- `compute_index_store_1fee`: Masks d0 with 127, shifts it left by three, subtracts seven, stores the result to memory at 1fee and returns.
- `preserve_regs_call_audio_tick`: Saves d0 and a0-a1 on the stack, calls the audio tick routine, restores those registers and returns to the caller.
- `clamp_and_set_audio_volume`: Clamps d0 to at most 100, stores it in a global and writes it to the four audio volume registers at offsets 168, 184, 200 and 216 of the custom chip base.
- `start_audio_channel3_playback`: Writes a1, d1, a table byte and d4/2 into the channel 3 audio registers, installs a level-3 interrupt vector, stores d0 and enables DMA and interrupt bits.
- `store_audio_volume_and_branch_back`: Writes the immediate value 1024 into the word at offset 154 of a0, then branches backward to the preceding instruction.
- `save_context_stop_audio_install_vblank`: Saves a1, d0, a2 and d1 to a buffer, calls SuperState, disables DMA and clears the four audio volumes and channel flags, then installs a vertical blank interrupt and CIAB timer handler before restoring UserState.
- `toggle_audio_dma_and_interrupt_state`: Checks the audio state word, then enables or disables audio DMA and interrupts via custom registers and restores the saved level 1 vector depending on the d0 request.
- `install_interrupt_vector_and_enable_dma`: Points the level 1 interrupt vector at the handler, enables copper and bitplane DMA, then sets the interrupt enable and a hardware register bit if two flags are set.
- `level1_interrupt_dispatch_check`: Saves registers, reads the interrupt request and enable registers, and if the vertical blanking interrupt is pending jumps through a stored handler pointer.
- `check_audio_interrupt_and_jump`: Tests bit ten of the interrupt request in d1 and, if set, loads a stored handler pointer and jumps to it.
- `test_d1_bit7_then_jump_vector`: Tests bit 7 of d1; if clear it branches ahead, otherwise loads a1 and a5 from a pc-relative table, pushes a pc-relative address and jumps through a5.
- `test_d1_bit9_then_jump_vector`: Tests bit 9 of d1; if clear it branches ahead, otherwise loads a1 and a5 from a pc-relative table, pushes a pc-relative address and jumps through a5.
- `interrupt_mask_check_and_rte`: Builds a mask of 1920, ANDs it with words at offsets 28 and 30 of a0, branches if nonzero, else restores d0-d2, a0-a1, a5-a6 and returns from interrupt.
- `zero_filled_data_word_handler_slot`: A single zero word that decodes as ori.b with immediate zero on d0; it is zero-filled data referenced by a handler pointer, not executable code.
- `zero_filled_data_word_pcrel_target`: A single zero word that decodes as ori.b with immediate zero on d0; it is zero-filled data whose address is taken by a pc-relative load, not executable code.
- `zero_filled_data_word_handler_entry`: A single zero word that decodes as ori.b with immediate zero on d0; it is zero-filled data reached through a handler pointer in the data hunk, not executable code.
- `zero_filled_data_area_6`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `zero_fill_handler_pointer_target`: One zero word stored where a handler pointer in hunk 2 data points; the bytes are zero-filled data, not executable code.
- `zero_fill_data_pair_at_1ad2`: Two consecutive zero words that decode as ori.b with immediate zero on d0; this is zero-filled data referenced by address, not code.
- `zero_filled_data_word_2`: Contains a single zero word that decodes as an ori.b immediate instruction; this is zero-filled data, not executable code.
- `install_audio_irq_vectors`: Writes audio and interrupt control values to custom chip registers, copies two longword vectors into offsets of a0, then stores a handler address at offset 68 of the structure pointed to by the longword at address 4.
- `reset_audio_interrupt_control`: Writes zero to the audio interrupt control register, writes 2 to the interrupt request register, and sets bits in the interrupt enable register.
- `decrement_timer_and_chain_next`: Pushes d0, reads a word from program counter relative memory, subtracts it from a long counter, and if no borrow or zero result restores d0, advances a frame counter, loads a5 from memory and stores a return address before returning.
- `dispatch_stream_command_byte`: Clears a word flag, saves registers, reads a byte from a stream pointer, and branches on its value to handle command bytes, extended bytes and a jump table dispatch through a5.
- `backup_stream_pointer_and_reload_byte`: Steps the a0 stream pointer back by one, loads a byte from program counter relative memory into d0, then branches back into the command dispatch code.
- `clear_byte_and_jump_via_pointer`: Clears the byte at absolute address 8190, loads the longword at address 28190 into a0, then branches to another routine.
- `store_byte_and_pointer_then_restore`: Subtracts 128 from d0, stores the low byte at address 8188, stores a0 at address 8176, restores d0-d2/a0/a3/a5-a6 from the stack and branches onward.
- `index_table_copy_by_byte_and_word`: Reads a byte from (a0), scales it by eight, shifts d0 right four bits, then copies a word and longword from the table at a3 into buffers at a5 using those indices before branching.
- `setup_audio_channel_from_sample`: Shifts d0 to select a sample, loads AUD0LCH base into a5, writes period and volume to the channel registers, sets the sample pointer and length, then branches to the shared playback loop.
- `restore_volume_and_jump_dispatcher`: Writes the volume word from the table indexed by d1 into the audio channel volume register, then branches to the shared command dispatcher.
- `start_audio_channel_with_volume`: Selects a sample from d0, writes period and volume to the audio channel registers, reads volume and frequency bytes from a0, stores the sample pointer and length, and branches to the shared playback loop.
- `store_table_word_to_a5_offset4`: Loads the address of a table into a6, reads the word indexed by d0 from that table and stores it at offset 4 of the structure pointed to by a5, then branches onward.
- `advance_a0_by_two_and_branch`: Adds 2 to the address register a0 and then branches unconditionally to another routine, continuing execution there.
- `setup_stream_buffer_from_a1_header`: If bit 0 of byte 3 at a0 is set, computes a length from fields before a1, stores it at offset 164, sets offset 160 to a1 plus a field, and writes 128 to offsets 154 and 156; always writes 128 to offset 156.
- `set_handler_and_audio_flag_if_bit0`: Tests bit 0 of byte at offset 3 of the structure pointed to by a0; if set, stores the address of the following routine into the handler slot and writes 128 to word at offset 156 of a0.
- `init_three_audio_words_to_defaults`: Writes 1 to word at offset 150, 128 to word at offset 154 and 128 to word at offset 156 of the structure pointed to by a0, then returns.
- `setup_sample_pointer_and_lengths`: If bit 1 of byte at offset 3 of a0 is set, computes a length from fields before a1, stores it at offset 180 of a0, stores a1 plus a field at offset 176, and sets words at offsets 154 and 156 to 256.
- `set_handler_and_clear_flag`: Tests bit 1 of byte at offset 3 of a0; if set, stores the address of the following routine into a5-relative global and writes 256 to word at offset 156 of a0, then returns.
- `init_three_state_words`: Writes 2 to word at offset 150 of a0 and 256 to words at offsets 154 and 156 of a0, then returns.
- `compute_buffer_offset_and_set_flags`: If bit 2 of byte at offset 3 of a0 is set, computes a halfword from a1-relative longs into offset 196, stores adjusted a1 into offset 192, and writes 512 to offsets 154 and 156; always writes 512 to offset 156.
- `set_handler_and_clear_flag_word`: Tests bit 2 of byte at offset 3 of a0; if set, stores the address of the following routine into the pc-relative handler slot and writes 512 to word at offset 156 of a0.
- `init_three_state_words_2`: Writes 4 to word at offset 150 of a0 and 512 to words at offsets 154 and 156 of a0, then returns.
- `decrement_counter_and_set_state_words`: If bit 3 of byte at offset 3 of a0 is set, decrements the long counter at absolute address 8182 and, when it goes negative, writes 8 and 1024 to words at offsets 150 and 154; always writes 1024 to word at offset 156.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Storm Master (1991)(Silmarils)(De).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_d2_by_four_and_branch_back`: Adds four to the low word of d2 and then branches backward to an earlier address, continuing execution there.
