# Game 549 (Panza Kick Boxing): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00549). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802384 top-level bytes in 67 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 338 of 338 named (0 author symbols), 338 of 338 described (9 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1325 of 1325 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 327390 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 13686 of 13701 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 990634 |
| code | 42992 |
| data | 510820 |
| padding | 18834 |
| unused | 239104 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Panza Kick Boxing (1990)(Futura)(Disk 1 of 2).adf:kick | 26 | 13555 | 330 |
| Panza Kick Boxing (1990)(Futura)(Disk 1 of 2).adf:paradox | 2 | 83 | 3 |
| Panza Kick Boxing (1990)(Futura)(Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| Panza Kick Boxing (1990)(Futura)(Disk 2 of 2).adf:bootblock | 1 | 37 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| tout.bin (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- panza kick boxing (1990)(futura)(disk 1 of 2)/Panza Kick Boxing (1990)(Futura)(Disk 1 of 2).adf:GRAPH/tout.bin: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- panza kick boxing (1990)(futura)(disk 1 of 2)/Panza Kick Boxing (1990)(Futura)(Disk 1 of 2).adf: blocks 1737-1738 (bytes 889344-890368) outside the filesystem hold data, not code (opcode density 0.021, entropy 4.28)
- panza kick boxing (1990)(futura)(disk 1 of 2)/Panza Kick Boxing (1990)(Futura)(Disk 1 of 2).adf: blocks 1749-1749 (bytes 895488-896000) outside the filesystem hold data, not code (opcode density 0.043, entropy 2.88)
- 1 never-loaded region(s), 327390 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 7168 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Panza Kick Boxing (1990)(Futura)(Disk 1 of 2).adf:kick

- `open_dos_and_init_game_state`: Saves registers, clears a block of game state, opens dos.library, reads the current directory and CLI, loads the game data file, then calls two setup routines and returns zero.
- `shutdown_game_and_close_dos`: Calls an optional cleanup callback, frees game resources, closes the dos.library, replies pending messages, forbids and exits interrupts, then restores registers and returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `bounded_string_append`: Appends the second string to the end of the first string, copying at most the given maximum number of characters and null-terminating the result, returning the first string pointer.
- `lookup_and_call_node_method`: Saves d4-d7/a3, looks up a node from the first argument, then calls a method with the node's field and two more arguments, returning minus one on lookup failure or when the global flag at offset 216 of a4 is set.
- `walk_list_flush_pending_ranges`: Walks the linked list at offset 4 of a4, and for each node with bit 2 clear, bit 1 set and a nonzero size, calls the flush routine with its fields, then calls the final routine with the argument.
- `write_buffer_via_dos_and_record_error`: Optionally calls a setup routine, clears the error slot at offset 216 of a4, writes the buffer through the dos library Write vector, and on failure stores IoErr and sets the status at offset 588 to 5.
- `seek_file_with_mode_and_offset`: Saves d2-d7, reads three stack arguments, optionally calls a helper, clears offset 216(a4), then calls dos.Seek via library offset -66 with mode and offset, storing the result and error code.
- `close_file_handle_and_return_zero`: Saves d7, loads the file handle from the stack, optionally calls a helper, then calls dos.Close through library offset -36 and returns zero in d0.
- `multiply_32bit_unsigned_words`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, combining high and low halves into a full 32-bit result in d0, preserving d2 and d3.
- `signed_32bit_divide_with_sign_fixup`: Takes signed 32-bit dividend in d0 and divisor in d1, negates operands to make them positive, calls the unsigned divide routine, then negates the quotient and remainder according to the original signs.
- `unsigned_32bit_divide_quotient_remainder`: Divides unsigned 32-bit d0 by d1 using shift-and-subtract normalization, returns quotient in d0 and remainder in d1, saving and restoring d2 and d3 on the stack.
- `lookup_indexed_entry_pointer`: Reads an index from the stack, clears the error field at offset 216 of a4, validates it against the count at a4 and the table at offset 268, returning the entry pointer in d0 or setting error 9 and returning zero.
- `free_command_list_nodes`: Walks the linked list at offset 596 of the a4 base, freeing each node's payload via exec FreeMem, then clears the list head and the count at offset 600.
- `lookup_and_run_command_entry`: Looks up a command record from the first argument, optionally sends a reset message, then invokes the record's handler with the other two arguments and returns its result or minus one.
- `parse_command_line_and_open_console`: Tokenizes the argument string into up to 32 quoted or bare words, then opens a console window and sets up input and output file handles and message ports.
- `copy_d0_to_d5`: Copies the longword in d0 into d5, then falls through into the following routine.
- `jump_to_fixed_address`: Jumps unconditionally to the fixed absolute address, transferring execution to the code located there.
- `call_library_offset_348`: Saves d2-d3 and a2-a3/a6, loads a6 and argument registers a0-a3 and d0-d3 from the stack, calls library offset -348, restores registers and returns.
- `flush_active_entries_reverse`: Saves d5-d7, walks the active entry table at offset 268 of a4 backwards, calls the release routine for each flagged entry, then calls the final cleanup routine with the argument from the stack.
- `clamp_d7_to_seventy_nine`: Builds a stack frame, saves registers, reads a byte from the pointer at offset 264 of a4 into d7, clamps it to at most 79, and sets a1 to a local buffer before branching on.
- `copy_name_and_write_break_message`: Copies d0 bytes from a0 to a1, terminates the local buffer, finds the current task, and writes the break message and a newline to the dos output using the task's output stream.
- `open_intuition_library_store_result`: Loads the exec library base into a6, opens intuition.library via jsr into exec, stores the handle at offset -102 in a5, and returns -1 if opening failed.
- `setup_ports_and_call_three_arg_helper`: Stores a pointer to a5-relative buffer at a4 offset 144, pushes several arguments and the stored handle, calls the helper at 0774, then returns 0 on success or -1 otherwise.
- `return_zero_restore_registers`: Sets d0 to zero as the return value, restores d2-d3, d6-d7 and a6 from the stack, releases the a5 frame and returns to the caller.
- `poll_signal_and_clear_callback`: Saves d7, calls exec SetSignal with mask 12288, and if those signals are set and the callback pointer at offset 240 of a4 is non-null, calls it; on zero result clears that pointer and calls the cleanup routine with 20.
- `return_1980_and_clear_306`: Executes nops, loads 6528 into d0 as the return value, clears the longword at absolute address 306 and returns to the caller.
- `zero_filled_data_area`: A single zero word that decodes as ori.b #0,d0; this range is zero-filled data referenced by pointers, not executable code.
- `zero_filled_data_word_at_02fc`: A single zero word that decodes as ori.b with immediate zero on d0; it is zero-filled data referenced by a pointer, not executable code.
- `zero_filled_data_word_at_0300`: One zero word in the same zero-filled data area, decoding as ori.b immediate zero to d0; a data pointer target rather than real code.
- `zero_filled_data_word_at_0304`: Another single zero word of the zero-filled region, decoding as ori.b with zero immediate on d0; referenced by a data pointer, not executable.
- `zero_filled_data_area_2`: Contains only a zero word that decodes as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `install_two_interrupt_servers`: Saves all registers, opens exec.library, builds two interrupt server structures from the base pointer, sets their handlers and priorities, calls AddIntServer for levels 3 and 5, sets a flag, then calls a routine with zero.
- `reset_round_counters_and_return_zero`: Calls two subroutines, clears the word at a fixed long address, increments two long counters, sets d0 to zero and returns.
- `decrement_four_timers_and_reset_slots`: For each of four countdown words, if nonzero decrements it and when it reaches zero clears its flag word and writes a pointer and value ten into the pointed structure.
- `track_joystick_direction_changes`: Reads both joystick direction ports, masks the direction bits, folds in the fire button from CIAA, and increments a change counter whenever the combined value differs from the stored previous value.
- `zero_filled_data_area_3`: This single zero word decodes as an ori.b immediate instruction but is a zero-filled data area referenced by address, not executable code.
- `reset_counter_if_below_threshold`: Loads the longword at offset 4 of the stack into d0, compares it with the global counter at address 3008, and clears that counter to zero if d0 is not greater.
- `increment_counter_and_scan_keyboard`: Increments the longword at 3120, reads the complemented and rotated CIAA serial register into d0, and sets a byte in the table at 1312 indexed by d0, returning zero.
- `clear_keyboard_table_entry`: Masks d0 with 127, clears the byte in the table pointed to by a0 at that index, sets d0 to zero and returns.
- `kick_data_table_not_code`: Thirty-two ori.b zero words that change no register; the bytes are a data table referenced by address, not executable code.
- `test_byte_in_table_at_offset_d0`: Loads the address of a table into a0, sign-extends the low word of d0, tests the byte at that offset and returns the flags.
- `dispatch_sound_command_with_retry`: Reads a command byte from the stack, stores it in globals, calls the sound setup, then loops calling the test and play routines until the byte is accepted.
- `read_joystick_buttons_and_directions`: Reads CIA-A port A and joystick port data, builds a button and direction bitmask in d1, optionally merges it with the low byte of d0, and returns the result in d0.
- `scan_keyboard_matrix_for_pressed_keys`: Polls keyboard rows through a helper, sets bits in d2 for detected keys, writes a flag to memory, and returns the accumulated key bitmask in d0.
- `find_first_active_keyboard_entry`: Scans a keyboard state table and a character table from index one to seventy, returning the first non-space character whose state byte is set, or zero if none is found.
- `load_animation_frame_sequence`: Stores the frame index in a global, reads a pointer table entry, writes 16 doubled words into a display buffer, then fills copper list entries with frame offsets until the total reaches 250, ending with a terminator and setting a flag.
- `stop_animation_and_return_index`: Writes -1 to the word pointed to by the global at 607e, reads the current frame index from the global at bbc, sets that global to -1 and returns the old index in d0.
- `dispatch_kick_frame_with_argument`: Loads the caller's first stack argument into d0, calls the routine at 9e4, then branches to the routine at a2c.
- `wait_for_stable_vposr_beam_value`: Pushes d0-d2, polls the VPOSR beam counter 2000 times keeping the largest masked vertical position in d1, and if it stays below the threshold writes two words to memory before restoring registers and returning.
- `disable_all_dma_channels`: Writes the value 15 to the DMACON register, clearing the bit 15 enable flag so all DMA channels are turned off, then returns.
- `enable_dma_channels_set_and_clear`: Writes the value with bit 15 set and channel bits 0-3 and 13 to DMACON, enabling those DMA channels, then returns.
- `init_four_copper_list_entries`: Enables copper DMA, then for four entries writes a copper list pointer, sets control words and stores a computed vertical delay from a division into each entry and a shared variable, finally enabling bitplane DMA and returning zero in d1.
- `load_channel_divisor_from_table`: Uses d0 as an index into a word table at address 3026, doubles it, reads the word and stores it into the divisor variable at 3020, then returns.
- `update_four_channel_minimums`: Reads flags and a value from the caller's stack frame, and for each of four set bits compares and updates a stored minimum, calling the channel update routine with the channel index and structure pointer.
- `setup_audio_channel_dma_and_sample`: Enables one DMA audio channel via DMACON, waits briefly, stores the channel sample pointer into the output list, copies start and length into the sample header, and computes the playback period into the output word.
- `save_registers_call_packet_handler`: Loads the packet pointer and command word from the stack, saves d2-d7 and a2-a6, calls the packet dispatcher, restores the saved registers and returns.
- `validate_and_dispatch_packet`: Checks packet length, type and checksum, calls the type's handler, verifies the payload checksum, and returns a status code in d0.
- `insert_byte_into_sorted_buffer`: Copies a counted byte string from a6+12 into the buffer at a6+d5+256, then inserts each byte into the sorted output at a6, returning 2 on overflow, 1 on exact fill or 0 otherwise.
- `emit_run_and_compare_sorted_output`: Writes d7 repeated d1 times to the output at a2, then compares the growing output against the sorted table at a0, returning 2 on overflow, 1 on match or 0 on mismatch.
- `copy_aligned_block_downward`: Aligns d5 up to a multiple of 16, computes source at a6+offset and destination below a5, copies that many bytes downward in 16-byte chunks, returning 1 on success or 2 if the ranges overlap.
- `check_status_equals_one`: Calls a subroutine, compares the returned word in d0 with 1, branches away if equal, otherwise returns to the caller.
- `interleave_four_planes_to_bitmap`: Calls a subroutine, returns 2 if it reports 2, else splits d5 into four plane pointers from a4 and copies interleaved words into a6, storing plane pointers in globals and returning 1.
- `reverse_copy_planes_backward`: Copies words backward from the three saved plane pointers and a4 into memory below a4, updating a1 to a3, then branches back into the interleave loop.
- `check_byte_equals_one`: Calls a subroutine, compares the returned word in d0 with one, branches if equal, otherwise returns to the caller.
- `unpack_four_words_bit_rotate`: Repeatedly reads four words from a6, rotates their bits through d0 and d5-d7, writes them back, advances a0 by eight bytes and loops d5>>3 times, returning one in d0.
- `run_length_decode_to_buffer`: Copies bytes backwards from a6+d6 to a6+d5+256, then expands run-length pairs using the marker byte at a6+12 into a2, returning one on success or other codes on mismatch.
- `check_condition_one_then_branch`: Calls a subroutine, compares the returned word in d0 with one, branches away if equal, otherwise returns to the caller.
- `check_condition_one_then_branch_alt`: Calls a subroutine, compares the returned word in d0 with one, branches to a different target if equal, otherwise returns.
- `copy_and_interleave_playfield_buffers`: Copies longwords from a6+12 to a6 until reaching a6+d6, then interleaves four 8000-byte buffers into a0 in 20 rows of 200 words, and finally copies 2000 longwords back.
- `copy_sprite_rows_and_clear_screen`: Copies d6 bytes of sprite data from a6+12 to a6, then if the destination stays below a5, expands 160 rows of 200 bytes into a 32000-byte buffer, clears 2000 longwords, and calls the blit routine.
- `decode_rle_stream_into_buffer`: Reads a run-length header at a6+12, stores the marker and count in globals, copies d7 longwords to a stack buffer, calls the decoder, and expands the compressed stream into a6 while checking against a5.
- `expand_rle_run_to_buffer`: Walks the run-length list at a5, writes the marker byte into the output at a0, refills d3 from a4 when exhausted, and decodes literal or repeated bytes until d5 runs out.
- `lz_byte_copy_loop_with_end_marker`: Rotates d3 left by 8, decrements d7 by 8, compares the low byte of d3 with the byte at memory location 2244; if equal branches out, otherwise writes d3's low byte to (a0)+, copies it to d0, compares a0 with a4 and loops back while below, else calls the flush routine and loops.
- `lz_refill_bit_buffer_and_decode`: Clears d3's low byte, compares d7 with d4 and branches if d7 is at least d4; otherwise rotates d3 by d7, saves d3's byte in d2, checks a4 against a3, reads three bytes from (a4)+ into d3 with word rotations and a swap, restores d2 into d3, computes d2 as d4 minus d7, rotates d3 by d2, adds 24 to d7 and branches.
- `lz_rotate_by_d4_and_emit_byte`: Rotates d3 left by d4, subtracts d4 from d7, compares d3's low byte with d6 and branches if equal or if carry set; otherwise ORs d1 into d3, subtracts d3 from d0, writes d0's low byte to (a0)+, compares a0 with a4 and either loops back or calls the flush routine then loops.
- `pop_regs_subtract_a6_check_d5`: Pops d5, d6 and a5 from the stack, subtracts a6 from a0, compares the result with d5 and branches if d5 is higher, otherwise sets d0 to one and returns.
- `compare_a3_a4_skip_or_restore_stack`: Compares a3 with a4 and branches if a3 is below a4, otherwise adds four to the stack pointer and branches to the register-pop routine.
- `backward_copy_bytes_until_a4`: Copies bytes backward from a3 to a5 one at a time until a3 equals a4, then sets a4 and a3 to the copied region and returns.
- `decompress_bitstream_to_buffer`: Copies a word count and packed bitstream from the a6 structure into a buffer below a5, then decodes bits into bytes at a6, writing a mask byte and calling the bit reader as needed.
- `emit_byte_rotate_and_advance`: Rotates d3 left by eight bits, decrements the d7 bit count by eight, writes the low byte of d3 to (a0)+ and d0, then compares a0 with a4 and branches or calls the refill routine.
- `refill_bits_and_emit_byte`: Clears the low byte of d3, compares d7 with d4, rotates in three bytes from (a4)+ when needed, then writes the resulting byte to (a0)+ and advances the bit counter.
- `rotate_d3_by_d4_and_compare`: Rotates the long in d3 left by the count in d4, subtracts the low word of d4 from d7, compares the low byte of d6 with d3 and branches if carry is clear.
- `decode_bits_into_output_buffer`: Subtracts d3 from d0, stores the byte to (a0)+, refills d5 and d1 from (a5)+ when the bit counter runs out, and branches back into the bit decoder.
- `expand_bitplane_row_to_buffer`: Copies a row of bitplane data from a6 into a temporary buffer at a5, calls the bit expander, then writes expanded bytes into the a6 buffer and returns a status in d0.
- `delta_decode_bytes_forward`: Copies bytes from a6+12 onward to a6, storing each byte minus the previous source byte, until the pointer reaches a6+d6, then returns 1 in d0.
- `delta_decode_bytes_previous_source`: Copies bytes from a6+12 onward to a6, storing each byte minus the preceding source byte, until the pointer reaches a6+d6, then returns 1 in d0.
- `delta_decode_words_with_global_store`: Stores the word at a6+12 into global memory at 2254, then writes bytes from a6+14 onward minus that word's low byte to a6 until a6+d6, returning 1.
- `set_sprite_clip_bounds`: Reads four arguments from the stack, masks the first two to multiples of sixteen, adds fifteen to the second, and stores all four as words at fixed global addresses before returning.
- `push_and_run_interrupt_chain`: Stores a new handler pointer and zero at globals, links it into the interrupt chain, calls the chain installer, restores the old pointer, then saves registers and calls the dispatcher.
- `walk_list_and_dispatch_entries`: Stores the argument as the current list pointer, saves all registers, then walks each non-null entry calling the entry routine and finally the dispatcher before restoring registers.
- `run_six_stage_frame_update`: Saves all registers on the stack, calls six unnamed subroutines in sequence, then restores all registers and returns to the caller.
- `load_fighter_state_from_a1`: Reads flags and fields from the structure pointed to by a1, stores them into global state words, copies four longs to a buffer and sets a pointer, then returns.
- `build_sprite_pointer_table`: When the flag word is set, adjusts d0 and d1 by screen offsets and fills a table of sprite pointers with stride and terminator values.
- `expand_16_byte_tile_column_to_bitmap`: Reads sixteen bytes from (a5), maps each through the table at a4 to a byte, stores them backwards below a3, advances a5 by three, adds d7 to a3 and repeats for d2 and d4 counts.
- `blit_two_playfield_bitmap_strips`: Computes a blit size from two globals, waits for the blitter, then programs blitter pointers, modulo, control and size registers to copy two bitmap strips from a0 into the display.
- `clip_and_scroll_playfield_window`: Compares d0 and d1 against the playfield bounds, adjusts the scroll offsets and window sizes in globals, updates the fine scroll tables and returns early when the shift exceeds the visible width.
- `compute_blit_offsets_and_pitch`: Reads two globals, combines them into a blit source offset stored to memory, then derives destination offset from d0 and d1 and returns a negative doubled pitch in d3.
- `blit_playfield_with_copper_lists`: Reorders four source longs into a buffer based on a mode global, runs the blitter wait, programs blitter pointers, modulos, control and size registers, and loops four times writing copper list entries before restoring DMA.
- `wait_blitter_busy_clear`: Executes two nops, then polls bit 6 of the DMACONR register until the blitter busy flag clears, with two more nops before returning.
- `reset_ring_bounds_and_call_setup`: Saves all registers, calls the setup routine, then writes zero to two longword globals and sets two more globals to 319 and 199 before restoring registers and returning.
- `fill_bit_reversed_byte_table`: Writes 256 bytes to a table, each byte being the bit-reversed value of its index, using shift and rotate instructions on d1 and d2, then returns.
- `store_caller_return_address_global`: Copies the longword return address at offset four from the stack pointer into the global at 1d52, then falls through to the following code.
- `blit_sprite_frames_to_bitplanes`: Walks the sprite list, reads frame dimensions and pixel data, copies rows into the a5 buffer, sets up blitter registers and queues a blit for each visible sprite.
- `expand_rle_byte_run_to_buffer`: Reads run-length encoded bytes from a4, writes zeros or repeated bytes to a5, decrements d3 per output byte and returns when the count is exhausted.
- `decode_bitmask_rows_to_buffer`: Uses a bitmask byte from a4 to select source bytes from a6 or zeros, writing one byte per bit to a5 while decrementing d3 until the row count ends.
- `relocate_pointer_list_by_base`: Walks a null-terminated list of offsets, converts each entry to an absolute pointer by adding the base and 2048, then patches four following longwords with the same base offset.
- `run_frame_update_sequence`: Saves all registers, calls eight subroutines in order, XORs two global longwords into a third, then calls two more subroutines before restoring registers and returning.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `copy_bytes_count_from_stack`: Reads source and destination pointers plus a count from the stack, copies that many bytes forward from a0 to a1 using dbra, then returns.
- `copy_words_or_longs_by_count`: Reads source and destination pointers and a count from the stack, then copies words or longwords from source to destination depending on bit one of the halved count.
- `toggle_copper_list_on_vertical_blank`: Swaps between two copper list buffers based on a toggle byte, waits for the vertical position register to fall in range, copies eight words into the copper list and sets a flag.
- `spin_until_8d4_changes`: Saves d0, loads the longword at address 8d4 into d0 and compares it with itself, branching to an infinite loop when equal; otherwise restores d0 and returns.
- `deinterleave_planar_rows_to_four_buffers`: Saves registers, computes source offset from d0 and d1, calls the setup routine, then copies words from a0 into a1 and from offsets 7998, 15998 and 23998 into a3, a4 and a5, advancing a2 by 40 per row.
- `compute_planar_copy_pointer_setup`: Adds d0 to d1 to offset a0, sets a2 to that source, shifts d2 right by three, computes row stride from d2 and d3, and derives a3, a4 and a5 pointers plus d4 and d3 counters.
- `interleave_four_planes_into_rows`: Saves registers, multiplies d1 by 160, calls the pointer setup routine, then copies four consecutive words from a0 into a1, a3, a4 and a5, advancing a2 by 160 per row.
- `mask_playfield_rect_columns`: Computes a playfield address from row and column arguments and, for each of four bitplanes, ANDs or ORs a column mask across the rectangle, clearing or setting pixels depending on the flag.
- `return_zero_and_branch_to_shared_exit`: Loads zero into d0 as the return value and branches to the shared exit code of the neighbouring routine.
- `blit_playfield_rect_to_screen`: Computes source and destination playfield addresses, programs the blitter registers for four planes and starts four blits copying the rectangle to the screen.
- `draw_upper_body_sprite_grid`: Loads the sprite base pointer, saves registers, then calls the blitter routine repeatedly to draw a 4x4 grid of tiles plus a second 4x4 grid offset downward, restoring registers before returning.
- `draw_lower_body_sprite_grid`: Loads the sprite base pointer, saves registers, then calls the blitter routine to draw two 4x2 grids of tiles at different vertical offsets, restoring registers before returning.
- `draw_two_wide_sprite_tiles`: Loads the sprite base pointer, saves registers, then calls the blitter routine twice with different source offsets to draw two wide tiles, restoring registers before returning.
- `draw_ring_apron_and_corner_blocks`: Loads the graphics base pointer, saves d2-d7/a2-a6, then repeatedly calls the block-drawing routine to place rows of tiles at fixed screen coordinates, adjusting d0 between calls, and restores registers before returning.
- `draw_ring_ropes_and_post_tiles`: Loads the graphics base pointer, saves d2-d7/a2-a6, draws one wide tile then two loops of nine tiles each at fixed coordinates via the block-drawing routine, and restores registers before returning.
- `jump_to_ring_draw_entry`: Branches unconditionally to the following routine, acting as a one-instruction trampoline that transfers control to the ring drawing code.
- `set_a1_a0_d0_then_jump`: Loads a1 from the longword at address 2264, loads a0 with the constant an address, sets d0 to 12820, then branches to another routine.
- `set_dual_plane_ptrs_and_jump`: Loads a1 from address 2264 and a0 with constant an address, adds 2520 to each, sets d0 to 5524 and d1 to 8000, then branches away.
- `blit_4_rows_with_modulo`: Computes source and destination offsets from stack arguments, programs blitter pointers, modulo, control and size registers, blits four rows, and restores saved registers.
- `copy_16_words_shifted_left`: Loads a source pointer from absolute memory, advances it by two, then copies 16 words into a destination pointer while shifting each word left by one bit and stepping the destination by four bytes.
- `dispatch_effect_by_low_two_bits`: Reads a parameter from the stack, saves registers, loads a base pointer, pushes a return address, and branches on the parameter's low two bits to one of four effect paths before returning.
- `restore_registers_and_run_effect_loop`: Pops saved registers d2-d7 and a2-a6, then runs a nested loop using d0 and d2 counters that calls two subroutines with d1 set to nine each iteration.
- `shift_left_four_planes_word_run`: For d1+1 words, shifts each word left by one in four bitplanes spaced 4000 words apart starting at a1, advancing a1 each iteration, then returns.
- `shift_right_four_planes_and_dispatch`: Shifts words right by one across four bitplanes spaced 7998 words apart for d1+1 words, then runs three caller-selected loops that call the left and right shift routines with different counts.
- `allocate_display_buffers_via_allocmem`: Saves all registers, calls exec AllocMem for a large buffer stored in two globals, then conditionally allocates a second buffer with different memory attributes based on two flag words.
- `check_chip_memory_and_set_flag`: Calls exec.AvailMem twice for an address bytes of chip memory, then writes a flag word to one of two globals depending on which allocation fails, or zero if both succeed.
- `open_dos_and_graphics_libraries`: Saves all registers, opens dos.library and graphics.library via exec.OldOpenLibrary with version zero, stores both library bases in globals and restores registers.
- `build_copper_lists_and_init_display`: Fills three pointer tables with incrementing addresses, copies sprite and bitplane pointers, then programs BPLCON0, DIWSTRT, DIWSTOP, DDFSTRT, DDFSTOP, BPL1MOD, BPL2MOD, COP1LCH and DMACON for the display.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `dispatch_io_command_by_stack_arg`: Compares the longword at offset 4 of the stack frame against 17, 7 and 37, branching to handlers; otherwise returns, or writes a doubled word into a table indexed by the second argument.
- `dispatch_dos_file_command`: Reads a command code from the stack frame and branches to dos.Open, dos.Read, dos.Close, exec.Forbid or exec.Permit handlers, restoring saved registers and returning when no code matches.
- `wait_fire_button_then_open_files`: Writes a color value to COLOR00, polls the CIAA fire button bit until released, then calls dos.Open twice and dos.Write using the caller's argument block.
- `seed_random_from_vposr_and_advance`: If the stored seed is zero, builds it from two reads of the VPOSR hardware register, then mixes the seed with a constant via a called routine, increments it, stores it back and returns bits 8 to 31 in d0.
- `seed_storage_data_byte`: A single ori.b immediate byte that is data, not executable code; it is the seed storage location referenced by the random routine and is never meant to run.
- `signed_32bit_multiply_via_16bit_parts`: Multiplies two signed long arguments from the stack by taking absolute values, combining 16-bit partial products with mulu, and negating the result in d0 when the signs differed.
- `install_vertical_blank_interrupt_server`: Loads exec base from address 4, fills the interrupt server node at the stored pointer with data pointer, priority 32, type 3 and code pointer, then calls AddIntServer with interrupt number 4.
- `poll_two_joystick_ports_for_changes`: Reads JOY0DAT and JOY1DAT masked to bits 0-1 and 8-9, folds in fire button bits from CIAA_PRA, and increments change counters when either port value differs from its stored previous value.
- `repeat_call_with_constant_37`: Saves d6 and d7, reads a word argument from the stack into d7, then calls the routine at the fixed address with argument 37 that many times before restoring registers and returning.
- `poll_input_until_ready_then_delay`: Saves d7, repeatedly calls the input poll routine with argument 255 until it returns nonzero, then counts down from -16 calling the same poll and a delay routine with argument 4, restoring d7 and returning.
- `compare_boxer_stats_and_redraw`: Loops over two boxer slots, calls the setup routine, compares stat and name bytes between the two boxer tables, and calls the redraw routine for any slot whose data differs.
- `draw_boxer_name_and_stats`: Reads a boxer index, calls the setup routine, builds a name string from the boxer table plus 65, then calls text and drawing routines with the boxer table pointers and name length.
- `apply_boxer_face_and_palette`: Reads boxer index from stack, scales it by 36 into the boxer table, copies the face byte plus 65 into a global, loads the face file, copies 13 palette bytes when the flag is set, and refreshes the boxer graphics.
- `play_voice_sample_once`: Checks a one-shot flag, loads the voix sample file, plays it at volume 48000 with priority 3, starts channel 1, and sets the flag so it never repeats.
- `reset_voice_flag_and_play_voix2`: Loads the voix2 sample file, plays it at volume 50000, clears the voice flag byte, restores d7 and returns.
- `load_tout_graphics_and_start_display`: Checks the display flag, loads either tout0 or tout1 graphics into d7, passes them with two globals to a blit routine, sets the mode byte, clears the flag and refreshes the screen.
- `load_toutx_graphics_and_set_mode`: Stores the argument byte in a global, loads toutx graphics, blits them with a fixed size and the screen pointer, saves the result and sets the mode and pending flags.
- `load_named_graphics_and_configure_palette`: Loads graphics named by the argument into d6, blits them with a fixed size, sets the palette from d7, calls the setup routine and copies a block of the screen structure.
- `toggle_board_palette_and_sound`: Saves d7, reads a flag byte from the stack, loads board picture palettes with a 256-byte limit, and calls sound routines to start or stop playback depending on the flag.
- `load_room_palette_and_play_sample`: Tests a global flag, loads one of two room picture palettes with a 256-byte limit, then calls the sample player with a 32000-period value and a stored pointer.
- `clear_sixteen_sound_channels`: Saves d7, loops d7 from zero to fifteen, and calls a sound routine for each channel index with a zero argument to clear it.
- `jump_to_2730_then_pop_d7`: Calls the routine at address 2730, then pops d7 from the stack and returns to the caller.
- `compute_fighter_stamina_from_record`: Reads three bytes from a fighter record, divides their sum by 15, looks up a base value, adds 50000 or 25000 depending on flag nibbles, and stores the result at offset 18 of the record.
- `play_sound_effect_sequence`: Pushes sound numbers 2, 1 and 2 while calling the sound routines, then discards the pushed arguments and returns.
- `draw_round_timer_digits`: Reads a word argument, computes remaining time from 480, splits it into tens and units via division helpers, and draws four digit sprites into the sprite buffer with blit calls before returning.
- `draw_four_health_bar_segments`: Reads a byte argument, looks up a table value, loops four times computing segment positions and drawing each bar segment through the blit routine, then restores registers and returns.
- `animate_sequence_frame_dispatch`: Increments a frame counter, dispatches on a mode byte to draw sprite frames from offset tables via blit calls, updates frame indices, and resets counters when limits are reached.
- `apply_palette_brightness_to_players`: Reads a brightness byte from the stack, updates a global palette entry, then loops over player rows and columns calling a routine with four palette bytes per entry.
- `set_player_sprite_heights_from_table`: If a global flag is clear, loops six times reading a table byte indexed by the low bit of the argument and writes each value plus 480 into sprite structure height fields, then calls two routines on a buffer.
- `check_two_objects_horizontal_overlap`: Loads two object pointers, computes each object's left and right extents from position, width and a flip bit, and returns 1 if the horizontal ranges overlap within six pixels, else 0.
- `copy_ring_positions_to_sprites`: Loops over two fighter slots, reads position words from the fighter records and packs x, y, flip and palette bits into the sprite control blocks at the indexed sprite table.
- `run_fight_loop_with_input_and_sprites`: Main fight loop: polls input, updates fighter positions and sprite control blocks, calls the sprite and sound routines, and exits when a quit key is pressed.
- `spawn_knockdown_effect_sprite`: When the counter equals three, builds a random-position sprite descriptor, sets its depth from the counter, queues it and decrements the counter.
- `select_fighter_sprite_pair_by_x_order`: Reads two object pointers from globals, extracts a two-bit field from each, subtracts them, and stores the two sprite pointers into globals in an order depending on the sign of the difference.
- `check_fighter_proximity_and_trigger_hit`: Compares the two active fighter objects' positions and flags, and when they are close enough and facing correctly, sets both animation frames and calls sound and effect routines with a hit code.
- `draw_round_number_and_start_fight`: Converts a round digit to a sprite index, draws round and fighter panels, waits for input, then plays sounds and starts the fight sequence.
- `draw_fighter_name_letters`: Loops over up to 64 or 10 character slots, reads a byte from the fighter name table, converts it to a glyph offset and draws two text strings via the text routines, then advances the slot counter.
- `run_victory_sequence_loop`: Plays the victory animation: sets up screen buffers, loops 30 frames updating two fighter sprites and the winner's stats, then adjusts both fighters' attribute bytes and checks for a rematch.
- `init_fight_round_sprites`: Initialises the round by setting sprite control words, clearing per-fighter flags, positioning both fighters' sprites and randomising their attribute bytes before starting the fight.
- `init_two_player_fighters_and_hud`: Compares player selection bytes, stores a flag, sets up two fighter structures in the object table with sprite pointers and attribute bits, then clears several state bytes and returns.
- `retry_until_positive_result`: Calls a setup routine, then repeatedly calls a status routine while it returns zero, and calls a wait routine while the result is negative, returning the final value in d0.
- `read_letter_and_compare_to_player`: Opens a text source, reads one character into a buffer, subtracts the player index byte and the letter a, and returns zero when the character is c.
- `call_3d_with_param_and_2`: Saves a3, loads the first stack argument into a3, then pushes 2 and that argument and calls the routine at a fixed address with selector 3d, cleaning the stack and restoring a3.
- `call_3f_then_3e_return_first`: Saves d5-d7/a3, loads three stack arguments, calls the shared routine with selector 3f passing two arguments, then calls it with selector 3e, returning the first call's result in d0.
- `retry_until_positive_result_2`: Saves d7/a3, loads the stack argument into a3, calls three subroutines in sequence, retrying the second while results are zero or negative, and returns the final result in d0.
- `call_3c_with_one_arg`: Saves a3, loads the first stack argument into a3, pushes zero and that argument plus the constant 60, calls the shared routine, then cleans the stack and restores a3.
- `call_40_then_f0_with_three_args`: Saves d6, d7 and a3, loads three stack arguments, calls the shared routine with 64 and all three, then calls it again with 240 and the first argument, cleans up and restores registers.
- `disk_prompt_and_palette_setup`: Builds a 16-entry table, fills a large palette buffer with masked random values, shows the insert-disk prompt, shifts the table entries and waits for input before restoring the saved values.
- `call_5206_with_two_globals_and_32000`: Pushes the longword at address 8D8, the longword at address 10786 and the immediate 32000 onto the stack, calls the routine at 5206, then discards the three arguments and returns.
- `dispatch_four_entries_by_two_bits`: Saves all registers, then four times builds an index from the low bits of d1 and d0, loads a code pointer from a table near the program counter and calls it, advancing a0 by one each time.
- `zero_sixty_four_sound_structs`: Clears d7 to zero and loops sixty-four times, writing zero to four byte fields at offsets 0, 4, 8 and 12 of each sixteen-byte record starting at a1, then returns.
- `set_d7_ff_and_branch_forward`: Loads minus one into d7 and branches forward to another routine without touching memory or calling anything.
- `fill_byte_ramp_forward_from_a1`: Writes ascending byte values zero to two hundred fifty-five in steps of four into every fourth byte of a block starting at a1, advancing a1 by sixteen per group until d7 wraps to zero.
- `fill_byte_ramp_backward_from_a1_plus_1020`: Starting at a1 plus one thousand twenty and moving backward, writes ascending byte values in steps of four into every fourth byte until d7 wraps to zero, then returns.
- `blit_8x8_tile_column_to_bitplanes`: Reads eight tile indices from the map at a5+d1, looks each up in the four-byte tile table at a6, and writes the four bitplane bytes to a4 at offsets 0, 8000, 16000 and 24000, then advances a4 and d2.
- `mask_and_or_tile_column_bitplanes`: Loops eight times over tile indices at a1, builds a combined mask from the four table bytes, and masks then ORs each byte into the four bitplane rows at a4, advancing a1 by 256 and a4 by 40.
- `dispatch_script_bytes_to_handlers`: Saves registers, reloads a2, a5, a6, d2 and d3 from globals, calls the setup routine, then reads bytes from a3 and jumps to the handler table entry or a2 until a zero byte, restoring the globals.
- `reset_sound_state_and_store_a0`: Clears the word at 842, stores a0 into 83e, writes the address of this code into 828, and clears the words at 838, 83a and 83c before returning.
- `read_three_bytes_build_word_in_d3`: Clears d2, reads two bytes from a3 into d2 and d3, shifts d3 left eight bits, reads a third byte into d3, then branches onward.
- `read_byte_store_to_83c`: Clears d0, reads one byte from a3 into d0, stores the low word of d0 into memory at 83c and returns.
- `read_byte_add_to_d3`: Loads the word at memory location 2108 into d2, clears d0, reads the next byte from (a3) into d0 and increments a3, then adds that byte value to d3.
- `index_table_entry_by_d3_and_d2`: Loads the pointer at 2092, dereferences it into a4, multiplies d3 by 40 and adds it to a4, then adds d2 to a4 and returns.
- `lookup_or_append_byte_pair_index`: Reads two bytes from (a3), searches a word table for the pair, and either returns index 10 or appends the pair, then computes a table offset and calls a subroutine.
- `clear_sprite_buffer_blocks`: Loads the sprite buffer pointer from the global pointer, then zeroes 1600 longwords in 20-byte steps, clears the word at the global flag, and jumps to the next routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `install_pc_relative_handler_pointer`: Loads the address 172 bytes past the program counter into a2 and stores it in the global at 828, then returns to the caller.
- `load_handler_address_and_branch`: Loads the address 422 bytes past the program counter into a2 and branches unconditionally to the code at offset 908 from this routine.
- `add_byte_to_d2_and_branch`: Reads a byte from the address in a3, increments a3, sign-extends it to a word and adds it to d2 before branching to another routine.
- `set_screen_dimensions_and_call_init`: Loads two arguments from the stack, shifts the second left by three, saves all registers, stores both words to global memory, then calls the setup routine and restores registers.
- `write_byte_to_audio_register_pair`: Loads a byte argument from the stack, saves all registers, points a3 at the audio register base, writes the byte to offset one and then to offset zero, calling a subroutine between the writes.
- `write_two_bytes_to_audio_port`: Reads two longwords from the caller stack, saves all registers, then writes the low bytes of each to the audio hardware at offset 2048 and 2049 before branching onward.
- `read_indexed_byte_pair_from_table`: Points a0 at the table base, reads the word count at 2114, and if nonzero decrements it, doubles it, and returns the two bytes at that index in d1 and d0.
- `clear_d0_d1_bytes_and_return`: Sets the low byte of d0 and the low byte of d1 to zero, then returns to the caller.
- `dispatch_vblank_handler_with_saved_regs`: Loads the caller's argument into d1, saves all registers, loads a5, a6 and a2 from fixed low-memory pointers, reads d2 and d3 from fixed words, calls a setup routine then jumps through a2.
- `call_audio_tick_with_a3_argument`: Takes the caller's argument into d0, saves all registers, moves that value into a3, calls a routine at a fixed offset, restores all registers and returns.
- `store_argument_to_global_pointer`: Copies the longword argument from the stack into the fixed global memory location at 2092 and returns immediately.
- `select_joypad_or_keyboard_input_mode`: Tests the word at offset 6 of the stack; if zero it stores the address of the keyboard input routine at the input vector and returns 1, otherwise stores the joystick routine address and returns 0.
- `edit_name_with_joystick_character_grid`: Sets up a stack frame, saves registers, draws a character grid, then loops reading joystick directions and fire to move a cursor and change letters in the name buffer until fire confirms.
- `map_letter_to_grid_index_upper`: Compares the character in d1 with 65; if it is at least that, subtracts 21 from the byte at offset minus one of a5 to convert the letter to a grid index, then branches on.
- `character_grid_cursor_editor`: Reads a character from the alphabet table indexed by a5 offsets, draws it at the cursor cell, polls input twice, merges direction bits into a5-3, and moves the cursor within the grid until an exit bit is set.
- `set_slash_direction_bits`: Compares d0 with the slash character, and if equal stores the value 16 into the byte at a5-3 before branching to the shared exit.
- `set_dash_cursor_and_bits`: Compares d0 with the dash character, and if equal stores 26 at a5-1 and 8 at a5-3 before branching to the shared exit.
- `map_ascii_to_eb_range`: Compares d0 with 65; if below, sets d2 to 235 plus d0 and stores d2 and 8 into a5-relative bytes, then branches onward.
- `adjust_ascii_after_65`: Subtracts 65 from d0, stores it and 8 into a5-relative bytes, optionally increments the first byte when bit 0 of the second is set, clamps it to 36, then branches.
- `decrement_byte_if_bit1_set`: Tests bit 1 of the byte at a5-1; if set, decrements the byte at a5-1, clamps it to 36, and branches onward.
- `decrement_letter_index_if_flag_set`: Tests bit 2 of the byte at offset minus 3 from a5; if set, decrements the byte at minus 2 from a5, then loads that byte into d0 and clamps it to at most 254 by comparing against 254 and clearing it if greater.
- `draw_and_edit_letter_grid_cursor`: Reads cursor and grid bytes from the a5 frame, copies a character from the alphabet table into the a3 grid, calls sound and drawing routines, and handles keyboard input to move or change the selected letter.
- `draw_letter_at_grid_position`: Saves d6, d7 and a3, reads a grid pointer and two byte coordinates from the stack, then calls the coordinate and character drawing routines to render a letter at that position.
- `format_number_into_buffer`: Formats the long value in d7 as a decimal string into a stack buffer, padding with spaces or zeros, adding a minus sign for negatives, then passes the buffer and two byte arguments to a print routine.
- `dispatch_three_graphics_calls`: Saves d5-d7 and a3, loads a pointer and three byte arguments from the stack, then calls three subroutines in sequence to set up and draw a graphics object.
- `main_game_loop_with_input_poll`: Runs the main game loop: polls input, updates game state counters and flags, calls many subroutines for rendering and sound, and exits when the quit flag is set.
- `reset_fighter_hit_flags_if_idle`: Loops over two fighter slots, and when both global busy flags are clear and each slot's counters exceed thresholds, calls the routine at offset 0A48 with argument 3; then clears two global bytes and restores d7.
- `check_fighter_attack_collision`: Sets up pointers to two fighters' 56-byte records, compares their attack priority bits, then scans each fighter's six-byte attack entries for overlapping hit boxes and calls the collision routine with the resulting coordinates.
- `compute_fighter_damage_and_knockback`: Reads two fighters' 36-byte records and a third index, scales damage by stance and direction, calls multiply and divide helpers, applies random critical bonus, subtracts from the target's health byte and returns a hit result code.
- `decode_joystick_direction_index`: Reads the byte argument from the stack, tests bits 2 through 6 to pick a base value (1, 7, 13, 19 or 25), adds the low two bits, and returns the result in d0.
- `set_action_state_unless_busy`: Reads the byte argument and two global state bytes; if either global equals 2 or the second equals 1 it clears the state byte, otherwise it stores the argument there, returning the tested value in d0.
- `resolve_and_apply_fighter_action`: Compares two fighter direction flags, picks attacker and defender slots, looks up action tables, calls the action-selection and effect routines, updates fighter state bytes and returns a status byte.
- `increment_counter_and_show_round_text`: Increments a global byte counter, passes it to several display routines, prints round text with the counter plus one, then calls cleanup routines and restores d7.
- `update_fighter_positions_and_input`: Reads fighter position words, computes midpoint and distance, checks input bits, updates direction and selection globals, and branches into movement or attack handling based on the current selection.
- `select_animation_frame_table`: Stores the byte argument as the current index, uses it to fetch a pointer from a table, saves that pointer and its offset-eight field to globals, and clears a flag byte.
- `advance_animation_frame_and_apply_sprite`: Reads the current frame index, looks up a six-byte sprite entry, toggles a hardware bit, writes the sprite control word and position into the display structure, and increments the frame counter.
- `select_fighter_animation_frame`: Chooses a fighter's animation frame from tables based on facing, distance and random values, then stores the resulting frame code for the caller.
- `compute_opponent_attack_frame`: Computes an attack animation frame for one fighter from both fighters' state tables, distance and random bits, returning the chosen frame byte.
- `cycle_fighter_pose_by_input`: Reads a byte argument, looks up its current pose in a table, and either returns it unchanged or advances it to the next pose using a random value and a fighter record, storing the new pose back in the table.
- `assign_fighter_pose_and_position`: Takes fighter index, slot and flag bytes, loads the fighter record, sets facing direction, copies the pose pointer into the slot table, computes a vertical position from the record and stores it, adjusting the pose when it exceeds bounds.
- `copy_pose_pointer_and_accumulate_offset`: Stores a pose byte for a slot, copies the pose pointer from a global table into the slot's pointer table, then adds the facing-scaled record offset to the slot's vertical position word.
- `advance_fighter_animation_frame`: Reads fighter index from stack, steps its animation frame counter, writes frame sprite and position into the shared sprite block, and recurses when the frame sequence wraps.
- `reset_fighter_animation_and_sprite`: Saves the fighter's animation timer, calls the animation setup with the fighter's frame count, restores the timer and resets the frame counter to the fighter's initial frame.
- `update_fighter_state_machine`: Advances the fighter's state byte, picks animation frames and sprite positions, toggles sprite flip bits, and returns the new state value.
- `clear_global_word_at_offset_36`: Writes zero to the longword at absolute address 54 and returns immediately to the caller.
- `find_matching_attack_slot_by_three_bytes`: Scans up to eleven attack table entries comparing three parameter bytes against entry fields and a per-index table, then checks a computed range against a status word and returns the matching index or zero.
- `init_six_fighter_slots`: Sets up six fighter slot records of fifty-six bytes each, links them into a pointer table, sets flag bits and copies word and byte fields from data tables, then clears a counter and sets a status bit.
- `init_player_slot_tables`: Clears four player slot pointers, sets up six slot entries with flags and coordinates, then zeroes two small state arrays and two global bytes before returning.
- `allocate_sound_channel_slot`: Scans eight twelve-byte channel slots for a free one, marks it busy and stores the sound id, two pointers and two bytes from the stack arguments.
- `tick_sound_channel_slots`: Walks eight twelve-byte channel slots, decrements active counters, and for expired slots clears the busy flag and calls the sound routine with stored parameters.
- `clear_eight_slot_flags`: Loops over eight records spaced twelve bytes apart and clears the byte at offset one in each, then returns.
- `reset_state_and_run_main_loop`: Clears a byte in global memory, calls several setup routines, pushes arguments for two more calls, then branches forever into the main loop after clearing the stack.
- `build_and_dispatch_sprite_descriptor`: Looks up a 36-byte record by index, finds the highest set bit of its byte, computes a table offset, calls two lookup routines, and passes the result plus constants to a dispatcher.
- `draw_fighter_health_bar_segments`: Reads a fighter index byte, looks up its record, and draws either a low or high health bar using two graphics calls with different coordinates and tile bases.
- `draw_fighter_stamina_bar`: Reads a fighter index byte, computes a value from the fighter record, and draws a five-pixel bar segment at offset five in the record.
- `wait_for_input_then_clear_screen`: Saves d7, calls a setup routine, waits in a loop for a nonzero result from a system call, then calls another routine and restores d7.
- `draw_fighter_stats_panel`: Draws a fighter's force, reflex and resist values as formatted text at screen positions, using the fighter index and row offset to index the fighter table and calling text and blit routines.
- `apply_fighter_style_and_moves`: Copies a fighter's style bytes and move list into its table entry, then sets the current move and merges bit flags into the fighter's state while calling update routines.
- `fighter_select_menu_loop`: Runs the fighter selection screen: loads the menu picture, draws two fighter portraits, polls input to move the cursor and confirm, and dispatches the chosen fighter.
- `init_two_player_input_and_rounds`: Calls setup routines, checks two global flags, then loops twice over two player slots filling empty entries via a helper and adjusting per-player bytes before returning.
- `poll_input_until_break_or_escape`: Increments d6, calls a delay routine, checks two player flags, polls the input port and breaks when bit seven is set, then bumps a per-player byte by three.
- `reset_input_state_and_fill_slots`: Calls two setup routines, invokes a library function, then loops over two player slots calling a helper to fill each empty entry and storing the result.
- `increment_player_stamina_until_input`: Increments d6, calls a routine, then polls input and for each of two player slots whose flag is set adds 5 to a byte in a 36-byte record until it reaches 70.
- `advance_stamina_second_threshold`: For the current player slot, if the record byte at offset 1 is below 98 adds 2 to it, then increments d7 and branches back to the slot scan loop.
- `run_match_loop_until_abort`: Calls two setup routines, then loops up to 300 frames polling input, updating each inactive player slot via a routine, and exits when the high bit of the input result is set.
- `push_pea4_call_f9a_advance_d6`: Pushes the longword 4 onto the stack, calls the routine at f9a, discards the argument by adding 4 to a7, increments the low byte of d6 by one, then branches to the code at 1da.
- `init_six_arg_call_and_scan_two_flags`: Calls a routine with arguments six, a global pointer, zero and zero, then loops over two flag bytes and calls a routine for each nonzero one.
- `call_3de_then_57c2_and_restore_frame`: Calls the routine at 3de, pushes the longword 1 and calls the routine at 57c2, then restores d2, d6 and d7 from the a5 frame, releases the frame and returns.
- `refresh_two_flags_from_entity_bytes`: Calls a setup routine, sets two flag bytes from whether entity records have a byte below eight, then configures eight entity slots and calls a rectangle routine.
- `init_eight_sprite_slots`: Loops over eight sprite slots, stores each slot pointer in a table and global, sets flag bit 7, writes 61 to offset 12, sets or clears bit 29 based on slot index, clears bit 30, and copies two words from tables into the slot.
- `configure_two_player_control_words`: For two slots, sets word 13 of each record, sets or clears the direction bit in the control longword based on slot index, and copies the player flag into the high bit.
- `clear_1c3a_and_call_14d4`: Clears the longword at address 1c3a, then pushes arguments 199, 0, 319 and 0 onto the stack and calls the routine at 14d4 before restoring the stack and popping d7/a2.
- `load_gym1_pi1_and_setup_display`: Calls four routines to load the gym1.pi1 picture, set up a 256-wide display, and pass the global at 8d8 with 32000 to the display routine, then cleans the stack.
- `draw_two_player_gym_slots`: Calls setup routines, then loops over two player slots, and for each active slot formats the name and number fields and draws the gym entry via the text routine.
- `push_sound_args_and_return_d7`: Pushes three arguments and calls two subroutines, then loads a longword from offset minus twelve of a5 into d7, releases the a5 frame and returns.
- `clear_two_globals_after_sound_calls`: Calls a subroutine, sends two argument pairs to another subroutine, cleans the stack, then writes zero bytes to two absolute memory locations and returns.
- `set_channel_periods_and_volume`: Writes period and volume words into two hardware channel structures via absolute pointers, sets both volumes to 24, calls a subroutine, then sends two argument pairs and returns.
- `init_two_player_control_ports`: Writes fixed control words to two hardware port registers, clears their offset-twelve words, calls the port setup routine and clears a status byte.
- `toggle_and_reset_both_players`: Loops over the two player slots, inverts each active flag, calls the per-player reset, clears its state words and flips a bit in its structure, then waits until the poll returns zero.
- `reset_player_slot_state`: For the current player index it clears the active flag, calls the per-player reset, zeroes the score and state bytes, and clears a control bit in that player's structure.
- `poll_until_input_ready`: Pushes a longword immediate, calls a subroutine, discards the argument, tests d0 and branches back to repeat the call while d0 is zero.
- `restore_d2_d7_a2_and_return`: Pops d2, d7 and a2 from the stack with movem and returns to the caller, restoring the saved registers.
- `update_fighter_state_from_input`: Reads a fighter index and input word from the stack frame, checks per-fighter tables, calls several subroutines and stores the new state word in the fighter table.
- `set_d4_ff_if_d6_above_80`: Compares d6 with 80 and branches to shared exit; if d6 is greater it sets d4 to 255, otherwise leaves d4 unchanged.
- `average_byte_into_slot_table`: If bit 0 of d6 is set, calls a routine with d7, stores the result in a local, and averages it into the byte table at offset 30 indexed by d7 when that entry is nonzero.
- `halve_and_store_stat_byte`: Reads the byte at a0 indexed by d7, adds sign-extended d0, halves the sum, stores it back at a1 indexed by d7, then calls a routine with d7.
- `compare_byte_to_slot_value`: Reads the byte table entry indexed by d7, optionally calls a routine with d7, subtracts and takes absolute difference, clamps to 50 when out of range, and returns a direction value.
- `subtract_frame_word_from_itself`: Loads the word at -4(a5) into d0 and d1, sign-extends both, subtracts d0 from d1, stores the low word back at -4(a5), and negates d1 if the result was negative.
- `clamp_fighter_state_and_dispatch`: Reads a word from the a5 frame, clamps it to 50 and stores it back, then indexes a byte table by d7 and calls a helper with d7 when the entry is nonzero.
- `check_byte_at_23_plus_d7_for_7c`: Copies d7 into d0, points a1 at address 35 plus d0, compares the byte there with 124, and sets d4 to 255 if it differs before branching.
- `check_byte_at_a0_plus_d7_for_7b`: Copies d7 into d1, adds it to a0, compares the byte at that address with 123, and sets d4 to 1 if it differs before branching.
- `call_1142_with_d7_if_counter_above_two`: Compares the word at a5 minus 4 with 2; if greater, saves d0 in d5, pushes d7 as a byte argument, calls the routine at 4418, then pops the argument.
- `check_distance_to_opponent`: Uses d7 to index a word table, subtracts that value from d6 and sets d5 to 123 when the difference is at least 300.
- `play_sound_and_animation_for_action`: Loads d5 with 122, then calls two routines passing d5 and d7 as arguments, restores d4-d7, releases the a5 frame and returns d4 in d0.
- `validate_and_update_fighter_action`: Saves registers, reads d7 and d6 from the stack, calls a helper with d7, compares table bytes against 125 and 17, and may call a display routine before returning.
- `check_fighter_slot_and_ai_state`: Reads d7 as an index, tests a word in the fighter table, and if empty calls a routine with a byte from a table plus the slot pointer, returning zero or one based on d6.
- `set_slot_flag_and_jump`: Loads one into d0, stores that word through the pointer at offset sixteen on the stack, then branches away to another routine.
- `check_slot_type_byte_and_dispatch`: Uses d7 to index a byte table, compares it with 126, and if equal tests another byte and a word count before calling a routine with d7.
- `check_opponent_state_and_play_sound`: Reads d7 as an index, calls a routine to get a value, masks it to four bits, and depending on the result writes 1 to a byte table and plays sound 28, 27 or 26, then calls another routine.
- `play_sound_based_on_table_byte`: Uses d7 as an index into a byte table, compares the byte with 8, and plays sound 27 or 26 accordingly before branching to shared tail code.
- `play_sound_26_and_call_1142`: Pushes d7 as an argument, plays sound 26, then pushes d7 again and calls the routine at 1142 before branching to shared tail code.
- `check_energy_and_set_action_flag`: Reads byte d7 as an index into a table, compares the entry with 2, adds 8, and if d4 is at least that value calls a routine with d7 and sets a word in a table to 2 when a status word equals 57.
- `check_byte_for_125_and_call`: Uses byte d7 as an index into a table, and if the entry equals 125 pushes d7 and 126 then calls a routine; otherwise returns 1 or 255 depending on whether the entry is zero.
- `test_table_byte_and_return_flag`: Uses byte d7 as an index into a table at a fixed address, tests the byte there, and returns zero if it is zero or minus one otherwise.
- `check_opponent_state_58`: Loads the current object pointer, reads its word at offset 12, and if it equals 58 calls a display routine with four pushed arguments before returning 1 in d0.
- `toggle_and_clear_fighter_slot`: Negates a global word, clears a word in a table indexed by the byte argument, and conditionally calls subroutines and displays messages based on table flags.
- `lookup_and_apply_fighter_action`: Reads a byte from a table indexed by d7, uses it to index another table, masks the result with 15, and sets d5 to 127 when the low bit is set.
- `test_d4_bit1_set_d5_flag`: Copies d4 into d0, masks with 12, and if the result is zero branches away; otherwise sets d5 to 128 and branches to the shared continuation.
- `update_fighter_state_on_hit`: If bit 1 of d4 is set, sets d5 to -127, calls two subroutines with d7, then checks fighter tables and increments a hit counter and sets a word in memory before restoring registers.
- `advance_animation_frame_counter`: Reads a byte counter, and if between 1 and 10, loads two fighter pointers, writes a table value into both at offset 12, increments the counter and branches to the shared exit.
- `update_opponent_ai_input_state`: Stores d1 in a global byte, and when d7's low five bits equal 31 and d7 is below 280, sets that byte to 1 and loops over two slots, calling a random routine twice per slot to compute a value compared against bytes in two global tables, writing the result into a third table.
- `restore_registers_and_return_to_caller`: Pushes four arguments, calls a library routine, then releases sixteen bytes of stack, restores d2, d5-d7 and a2 from the stack, drops the a5 frame and returns.
- `reset_display_and_set_delay`: Saves d7, loads a byte argument, calls display setup routines, then passes six minus the argument to a delay routine before restoring d7 and returning.
- `pop_animation_frame_and_clear_slot`: Reads a byte index from the stack argument, uses it to look up a frame index in a byte table, fetches a word from a word table, clears that word to zero, and returns the fetched word in d0.
- `draw_three_stat_labels_and_values`: Reads a fighter index from the stack, computes two screen coordinates via a division helper, then prints three stat labels and their byte values from the fighter record using text and number drawing calls.
- `advance_fighter_animation_frame_2`: Reads a fighter index, compares its current frame counter against a limit, fetches the next animation word, updates sprite control registers and position, increments the counter, and resets it when finished.
- `select_sprite_frame_by_index`: Saves d7, loads the index from the stack, scales it by sixteen, adds it to a table base and stores the resulting pointer into a global, then returns negative one unless the index is zero.
- `set_success_flag_and_return`: Loads the value one into d0, writes that word to the global at address 56, restores d7 from the stack and returns to the caller.
- `copy_object_slot_and_clear_flag`: Copies a source object pointer into a slot table, stores its field at offset eight into another table, clears a byte flag for the slot and restores d6, d7 and a2.
- `check_byte_against_table_and_call`: Saves d6-d7, reads a byte argument and a long pointer from the stack, compares the byte with the table byte at offset 24 from the pointer, and if not below calls the routine at the given address before restoring the stack.
- `restore_d6_d7_and_return`: Pops d6 and d7 from the stack and returns immediately to the caller, restoring the saved registers.
- `fill_or_mark_flag_table`: Saves d6-d7, tests the word at the global flag, then either sets all 130 bytes of the table to one or clears them and marks entries indexed by the word list until the terminator value 9999.
- `show_tout_then_load_pres2`: Pushes d7, displays the tout.bin graphic with pres2.pi1 palette, runs setup calls, and if a flag word is zero loads a file and calls a blit with size an address.
- `show_pres1_wait_input_set_flag`: Saves d7, displays pres1.pi1, stores the byte argument into a global, polls input until nonzero, sets a state word to 2, waits 200 ticks and returns.
- `pres0_intro_animation_loop`: Saves registers, sets a state word, displays pres0.pi1, patches sprite offsets, then loops polling input and blitting frames until a key is pressed, restoring state.

### Panza Kick Boxing (1990)(Futura)(Disk 1 of 2).adf:paradox

- `decompress_packed_graphics_to_chip_ram`: Reads a packed stream from a table in program memory, decodes it with bit and run-length codes into chip memory at the start of the custom chip area, flashes the colour register during the copy and jumps to the destination when finished.
- `fetch_next_packed_longword_bit`: Pops the previous longword from the source pointer, folds it into the running checksum in d5, shifts it right through the extend flag and returns the next packed bit in d0.
- `extract_multiple_packed_bits`: Repeats the packed-bit fetch d1 times, shifting each bit into d2, refilling d0 from the source pointer whenever it empties, and returns the collected bits in d2.

### Panza Kick Boxing (1990)(Futura)(Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Panza Kick Boxing (1990)(Futura)(Disk 2 of 2).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `copy_config_and_checksum_vectors`: Copies a block of configuration longs into the base pointed to by a6 offset 58, increments the word at offset 734, stores a pointer at offset 46, sums 24 words into a checksum, then opens a library and stores the result at offset 286.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_words_between_a0_and_a1`: Clears a global, calls several subroutines, then copies a count of words from the area pointed to by a0 into the area pointed to by a1, decrementing the count until it reaches minus one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
