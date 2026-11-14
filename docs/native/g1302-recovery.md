# Game 1302 (rick dangerous ii (1990)(micro style)[cr angels - genesis]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01302). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904970 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 318 of 318 named (0 author symbols), 318 of 318 described (10 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4250 of 4250 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 512 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9438 of 9439 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20540 |
| code | 37290 |
| data | 843858 |
| padding | 210 |
| unused | 3072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf:bootblock | 1 | 34 | 2 |
| logged disk load from Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf boot-block load | 1 | 163 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 365 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 599 | 36 |
| logged disk load from Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf | 1 | 8220 | 239 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf bytes 11264-11776 (code never loaded by a reference run) | 1 | 18 | 2 |

## Cases handled

- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: custom-format (non-DOS) disk, 1749 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 3570 of the 4202 PCs the canonical run logged (658 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address (play-rs-m1500): 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 226 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 477 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: 3339 executed PCs whose RAM was overwritten before any snapshot match its bytes 207360-255776: recovered as a unit run at an address
- 22 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 11 first-run image unit(s) in 1 version group(s); 180 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: bytes 11264-11776 look like code no run loaded: recovered position-independently (load address unknown)
- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: 3 logged loads cover 54560 bytes of the image
- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: blocks 26-27 (bytes 13312-14336) outside the filesystem hold data, not code (opcode density 0.053, entropy 5.56)
- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: blocks 29-29 (bytes 14848-15360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.89)
- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: blocks 31-31 (bytes 15872-16384) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.76)
- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: blocks 33-33 (bytes 16896-17408) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.72)
- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: blocks 35-35 (bytes 17920-18432) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.99)
- rick dangerous ii (1990)(micro style)[cr angels - genesis]/Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf: blocks 500-883 (bytes 256000-452608) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.77)
- 3 packed track region(s), 641024 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `start_disk_dma_read_with_retries` (runtime segment an address (play-rs-m1500)): Clears a counter, waits, checks CIAA fire button, programs ADKCON, DSKPTH, DSKLEN and DMACON for a disk DMA read, and retries up to five times. Kind: disk track/sector read retry loop with hardware register manipulation. Pass path: the loop continues (sVar2 > 0) and eventually exits via the break condition when the read succeeds or retries are exhausted, specifically the path where bVar3 is false and sVar2 != -1.
- `start_disk_dma_read_to_a0` (runtime segment an address (play-rs-m1500)): Programs the custom chip registers at a fixed address: sets ADKCON, enables disk and bitplane DMA, sets sync word, disk pointer from a0, length from d0 shifted right, and starts the disk DMA transfer. Kind: disk hardware/sync check. Pass path: bne.b an address (sub_0288) or bne.b an address (sub_0128).
- `acknowledge_disk_interrupt_and_set_length` (runtime segment an address (play-rs-m1500)): Writes 2 to INTREQ to clear the disk interrupt, writes 16384 to DSKLEN, tests d0 and returns. Kind: disk hardware/track length check. Pass path: the instruction immediately following the call (rts).
- `enable_disk_dma_and_test_flag` (runtime segment an address (play-rs-m1500)): Writes 1024 to ADKCON to enable disk DMA, tests a word at offset -30 from a6, and sets d1 to 255 if negative. Kind: disk hardware/DMA status check. Pass path: beq.b an address.
- `set_ciab_prb_bit_from_offset` (runtime segment an address (play-rs-m1500)): Writes d1 to CIAB port B, then clears and sets bit (word at a6 minus 36 plus 3) in d1 and rewrites CIAB port B. Kind: bit manipulation/status flag check. Pass path: the code continues execution normally after the bsr.w an address call.
- `reset_floppy_select_and_delay` (runtime segment an address (play-rs-m1500)): Writes all ones to CIAB port B, clears bit 7 of d1, calls the port bit routine, then delays via the wait routine with 200 and returns. Kind: disk hardware/drive verification. Pass path: the execution continues to u4__sub_07FA() after the subroutines return.
- `set_disk_select_with_motor_pulse` (runtime segment an address (play-rs-m1500)): Saves d0, calls the drive-status routine, clears CIAB port B bits 0 and 1 when d1 is non-negative, writes the value to the disk control register, then sets bit 0 and writes again before restoring d0. Kind: disk/track/hardware check. Pass path: the branch where extraout_D1b >= 0 (the 'if' block).
- `write_disk_control_byte` (runtime segment an address (play-rs-m1500)): Calls the drive-status routine to obtain a byte in d0, then writes that byte directly to the CIAB disk control register and returns to the caller. Kind: disk hardware/track check. Pass path: sub_0706.
- `set_ciab_prb_disk_select_bits` (runtime segment an address (play-rs-m1500)): Saves d1-d2, reads a word from offset -36 of a6, loads CIAB port B, sets bits 0-6, clears one bit indexed by that word plus three, and optionally clears bit 2 based on a table lookup before returning the new port value in d0. Kind: disk hardware bit manipulation/check. Pass path: the branch where the bit is not cleared or the condition is not met.
- `play_sample_with_dma_retry` (Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf +an address run at an address (48416 bytes, matched to executed PCs)): Clears a word at offset -6 from a6, calls two setup routines, then up to five times checks the CIAA fire bit, programs ADKCON, DSKPTH, DMACON and DSKLEN on the custom chip base, and calls a status routine until it succeeds. Kind: disk hardware/track validation check. Pass path: the loop continues (sVar2 > -1) and the function eventually returns via the break condition where bVar3 is false.
- `start_disk_dma_read` (Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf +an address run at an address (48416 bytes, matched to executed PCs)): Points a1 at the custom chip base, clears ADKCON, enables disk and bitplane DMA, sets sync word, stores a0 as disk pointer, sets DSKLEN length from d0 shifted right and enables the transfer. Kind: disk hardware/sync check. Pass path: the branch following the bsr.w an address call (likely the bne.b an address or the subsequent instruction depending on the specific caller's logic).
- `acknowledge_disk_interrupt_and_stop_dma` (Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf +an address run at an address (48416 bytes, matched to executed PCs)): Writes to INTREQ to clear the disk interrupt bit, writes to DSKLEN to disable disk DMA, tests d0 and returns to the caller. Kind: disk hardware/timing check. Pass path: the code continues to the next instruction after the call (the 'after' instructions in the call site list).
- `set_adkcon_bit10_and_test_flag` (Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf +an address run at an address (48416 bytes, matched to executed PCs)): Writes the value 1024 to the ADKCON hardware register, tests the word at offset -30 from a6, and if negative loads 255 into d1. Kind: disk hardware/ADKCON check. Pass path: beq.b an address.
- `toggle_ciab_disk_select_line` (Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf +an address run at an address (48416 bytes, matched to executed PCs)): Writes d1 to CIAB port B, reads the word at offset -36 from a6, adds 3, clears then sets that bit in d1 and rewrites the port. Kind: disk hardware/timing check. Pass path: the code continues execution normally after the call, assuming the disk hardware responds to the CIAB_PRB signal as expected.
- `assert_ciab_reset_then_delay` (Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf +an address run at an address (48416 bytes, matched to executed PCs)): Sets d1 to 255 and writes it to CIAB port B, clears bit 7 of d1, calls the disk-select toggle routine, then delays with 200 in d0 before returning. Kind: disk hardware/CIAB_PRB check. Pass path: execution continues to u5__sub_2562.
- `toggle_disk_motor_select_line` (Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf +an address run at an address (48416 bytes, matched to executed PCs)): Saves d0, calls the drive select routine, clears CIAB_PRB bits 0 and 1 to switch the disk motor, then sets bit 0 again, restores d0 and calls the delay routine before returning. Kind: disk hardware/motor control check. Pass path: the branch following the call that continues normal execution (likely the path where the motor/track state is validated).
- `write_ciab_prb_disk_control` (Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf +an address run at an address (48416 bytes, matched to executed PCs)): Calls the disk control helper, then writes the returned byte in d0 to CIAB port B at the disk control register and returns. Kind: disk hardware/track check. Pass path: the code continues execution following the call sites (e.g., moving to the next instruction after the `bsr` or `lea` in the provided call sites).
- `build_ciab_prb_drive_select_byte` (Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf +an address run at an address (48416 bytes, matched to executed PCs)): Reads CIAB port B, sets bits, clears the drive-select bit indexed by the word at offset minus 36 of a6, optionally clears bit 2 from a table lookup, and returns the byte in d0. Kind: disk hardware/drive select check. Pass path: the branch where the bitmask is not modified by the conditional check (the 'if' condition failing).

## Functions


### Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf:bootblock

- `alloc_chip_mem_and_start_boot`: Saves a1, allocates 2560 bytes of chip memory via exec.AllocMem, stores the pointer in a3, calls the boot setup routine, fills in the IO block fields and jumps to the loaded boot code.
- `configure_io_block_and_call_loader`: Sets the IO block length, data and offset fields, calls exec.DoIO, saves all registers, calls the loader at a fixed address, restores registers and stores a3 back into the IO block.

### logged disk load from Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf boot-block load

- `decompress_boot_data_to_chip_ram`: Copies a block of bytes into chip memory, then decodes a packed bitstream using COLOR16 and several bit-reading subroutines, writing the decompressed bytes backwards into the destination buffer.
- `pop_long_ror_into_d7_and_a5`: Pops a longword from the stack pointed to by a0 into d0, copies its low byte to d7, stores d7 to the word at a5, rotates d0 right through extend by one bit, then returns.
- `bitstream_decode_loop_with_escape`: Shifts d0 right repeatedly, calling the bit-fetch routine when it empties, accumulating bits into d2 for d1 iterations; also handles escape bytes by copying runs to the buffer at a0 and jumping to it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `install_trap0_vector_and_jump`: Loads the address of the following code into a0, stores it at the trap zero vector location, executes trap zero, then branches forward past the handler.
- `set_supervisor_mode_and_return`: Loads the supervisor mode value into the status register via the stack and executes a return from exception, then returns.
- `relocate_loader_to_high_memory_and_jump`: Sets supervisor mode, disables all interrupts and DMA, moves the stack to high memory, copies 2110 bytes from a nearby table to the top of memory, re-enables interrupts and jumps there.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_initializer_then_return_zero`: Calls the nearby initialization routine, clears the longword in d0 to zero as the return value, and returns to the caller.
- `save_all_regs_call_init_sequence`: Pushes every data and address register, loads the base of the jump table into a3 and the custom chip base into a6, calls three routines through a3, restores all registers and returns zero in d0.
- `install_copper_and_audio_vectors`: Copies the current copper list pointer and audio period into the a3 work area, sets both audio period registers to maximum, points the copper interrupt vector at a new list, stores a new level interrupt vector and enables the copper and audio interrupts.
- `run_intro_sequence_and_wait_fire`: Calls a series of routines through the a3 jump table to load two picture blocks and play music, then polls the fire button bit in the CIA port and, if not pressed, calls two more routines before returning.
- `reset_audio_channels_and_read_status`: Writes 32767 to two audio registers at offsets 154 and 150 of a6, copies a longword from pc-relative memory to offset 108, reloads both audio registers from pc-relative words, then returns the word at offset 136 of a6 in d0.
- `save_registers_set_vblank_and_rte`: Pushes all data and address registers onto the stack, pops them back, writes 112 to offset 156 of a6, and returns from the interrupt with rte.
- `scan_slot_table_for_type_three`: Walks up to 17 entries of 41 bytes starting at a pc-relative table, comparing each first byte to 3; on a match stores the entry index in offset 8012 of a3 and returns, otherwise returns without storing.
- `draw_joystick_calibration_bar`: Reads the calibration value at offset 8012 in a3; if not minus one, writes 74 or 255 words into the bar buffer at 30590 stepping six bytes per entry, then returns.
- `handle_weapon_pickup_and_fire`: Delays briefly, then reads the current weapon slot from a3, checks its type byte and updates the weapon inventory byte, sound effect and sprite pointer before returning.
- `convert_bcd_counter_to_decimal`: Converts the value in d0 from packed BCD into a decimal digit sequence, adjusting nibbles and building a four-character display value, then returns it.
- `round_low_nibble_to_next_tens`: Masks d1's low byte to its high nibble, adds sixteen to it, then branches to the shared digit-conversion code.
- `increment_and_pack_digits_to_ascii`: Increments d1, skips nibbles equal to nine, shifts the value right four times while rotating bits into d0, adds ascii digit offsets and returns the packed result in d0.
- `insert_space_byte_and_swap_d1`: Rotates d1 right by eight bits, writes a space byte into d1's low byte, swaps d1's halves and branches back into the digit-conversion code.
- `pop_d1_d2_and_return`: Pops registers d1 and d2 from the stack and returns immediately to the caller.
- `copy_15df_bytes_to_50000`: Copies 5600 bytes from the code area near the program counter to memory starting at 50000, then returns.
- `copy_14_words_with_delay_to_3055c`: For 14 iterations, busy-waits then copies three words from a table near the program counter to memory at 3055c, 30560 and 30564, then delays and returns.
- `copy_14_words_to_custom_registers`: Copies fourteen words backwards from a source table into three custom chip registers with delay loops between each write, then delays and returns.
- `write_11_words_to_17_offsets`: Reads eleven words from a source table and writes each word to seventeen spaced offsets in memory with delay loops between writes, then returns.
- `copy_11_words_to_16_sprite_rows`: Reads 11 words backwards from a table near pc, waits in a delay loop, then writes each word into 16 consecutive sprite row slots spaced 12 bytes apart in chip memory.
- `clear_52000_bitplane_buffer`: Loads address a fixed address into a0, sets d0 to 6144, then clears 6145 longwords via post-increment writes and returns.
- `render_16x40_char_tiles_to_bitmap`: Reads 16 rows of 40 characters from a5, maps each through a 40-entry table, and copies 9 bytes per glyph into the a fixed address bitmap with 80 and 40 strides.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_overlay_and_jump_to_10000`: Calls a loader with d0 zero, d1 405, d2 483, d3 32768, a0 a fixed address and a1 a fixed address, then jumps to a fixed address if it returns zero, else sets COLOR00 to 3584 and loops.
- `track_sector_seek_and_read`: Saves registers, computes track and sector from d0/d1, clamps to the disk limit, calls seek and read helpers, and returns the resulting byte offset in d0.
- `retry_disk_track_read_five_times`: Loops up to five times checking CIAA fire button and INTREQR, sets sync words in the track buffer, and calls read helpers until a track read succeeds.
- `start_disk_dma_read_with_retries`: Clears a counter, waits, checks CIAA fire button, programs ADKCON, DSKPTH, DSKLEN and DMACON for a disk DMA read, and retries up to five times.
- `scan_ten_slots_for_valid_target`: Loops up to eleven times calling collision checks, and on a hit with matching row and small column and row values stores adjusted column and remaining count in the a6 structure, returning zero or an error code.
- `iterate_eleven_entity_slots`: Walks entity slots at a5 offset by 1088 stride, checks flags and collision helpers, updates slot fields and returns zero, minus one or an error code.
- `compute_column_offset_from_d3`: Copies d3 into d1, subtracts the word at offset -16 from a6, multiplies by 512 and returns the result in d1.
- `set_bit_in_a6_flags_and_increment_counter`: Reads the word at offset minus 4 from a6, sets the bit selected by d3, stores it back and increments the word at offset minus 6 from a6.
- `clear_eleven_longwords_at_a5_offset`: Starting at a5 plus one thousand eighty-eight, writes zero to eleven consecutive longwords spaced two hundred seventy-two bytes apart, then returns.
- `read_four_bytes_from_a5_offset_eight`: Calls a subroutine with a5 plus 8 as argument, then splits the returned longword into bytes held in d3, d2, d1 and d0.
- `interleave_two_longwords_masked`: Reads two longwords from the address in a0, masks each with the alternating bit pattern, doubles the first and ORs in the second, returning the merged value in d0.
- `compare_checksum_of_two_buffers`: Calls the checksum routine on the buffer at offset 8 of a5, saves the result, computes the checksum of the buffer at offset 48 of a5, and compares the two values in d0.
- `set_a0_to_a5_offset_eight_and_d1_count`: Loads a0 with the address eight bytes past a5 and sets d1 to forty, preparing a buffer pointer and count for the following code.
- `xor_words_masked_parity_checksum`: Pops d2, shifts d1 right by two and subtracts one for the count, then XORs that many longwords from (a0) into d0 and returns d0 masked with alternating bits.
- `fill_1024_bytes_with_aa_pattern`: Loads a pointer from offset minus 24 of a6, sets a1 one kilobyte higher, fills the block downward with the longword pattern of alternating bits using d0 through d7, and returns when a1 reaches a0.
- `init_sprite_block_pointers_and_control`: Saves a2, calls a size routine, then stores pointers at offsets 76, 80 and 84 of a2, writes control words 64 and 66, and stores a shifted flag word at offset 88 before restoring a2.
- `blit_four_rectangles_and_clear_three_rows`: Saves d1-d3/a2, queries blitter sizes, then programs four blitter rectangle passes into a2 blitter registers and clears three rows via repeated calls.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `init_blitter_dma_and_modulos`: Points a2 at the custom chip base, calls the blitter wait, enables DMA with 32832, sets all blitter data masks, modulo 21845 and clears control registers.
- `halve_d0_and_blit_twice`: Pushes d0, shifts it right by one, calls the blitter routine, restores d0 and calls the blitter routine again.
- `toggle_byte_bit7_by_previous_flag`: Reads a byte from (a0), checks bit 0 of the byte at -1(a0); if clear and bit 6 of the byte is clear, sets bit 7, if set clears bit 7, then writes the byte back to (a0).
- `interleave_d0_bits_into_a0_word`: Masks d0 with alternating bits, derives a shifted copy, optionally clears bit 31 based on bit 0 at -1(a0), merges and stores the longword to (a0)+.
- `start_disk_dma_read_to_a0`: Programs the custom chip registers at a fixed address: sets ADKCON, enables disk and bitplane DMA, sets sync word, disk pointer from a0, length from d0 shifted right, and starts the disk DMA transfer.
- `wait_blitter_and_read_disk_status`: Loads the custom chip base into a1, sets d0 to 6000, calls a delay, tests bit 1 of the disk status register, calls a check, and returns 255 or 0 in d0.
- `acknowledge_disk_interrupt_and_set_length`: Writes 2 to INTREQ to clear the disk interrupt, writes 16384 to DSKLEN, tests d0 and returns.
- `enable_disk_dma_and_test_flag`: Writes 1024 to ADKCON to enable disk DMA, tests a word at offset -30 from a6, and sets d1 to 255 if negative.
- `set_ciab_prb_bit_from_offset`: Writes d1 to CIAB port B, then clears and sets bit (word at a6 minus 36 plus 3) in d1 and rewrites CIAB port B.
- `reset_floppy_select_and_delay`: Writes all ones to CIAB port B, clears bit 7 of d1, calls the port bit routine, then delays via the wait routine with 200 and returns.
- `step_floppy_head_to_target_track`: Saves d2-d3, reads a signed word from the track table indexed by the a6 offset, moves the head toward the target track using the step and port routines, stores the new track and returns zero.
- `poll_fire_button_and_clear_slot`: Saves d2, reads CIAA port A bit 4 up to 86 times, calling the disk routine each pass; if the fire button is released it clears a word in a table indexed by a6 offset minus 36 and returns 0, otherwise returns 30.
- `set_disk_select_with_motor_pulse`: Saves d0, calls the drive-status routine, clears CIAB port B bits 0 and 1 when d1 is non-negative, writes the value to the disk control register, then sets bit 0 and writes again before restoring d0.
- `write_disk_control_byte`: Calls the drive-status routine to obtain a byte in d0, then writes that byte directly to the CIAB disk control register and returns to the caller.
- `set_ciab_prb_disk_select_bits`: Saves d1-d2, reads a word from offset -36 of a6, loads CIAB port B, sets bits 0-6, clears one bit indexed by that word plus three, and optionally clears bit 2 based on a table lookup before returning the new port value in d0.
- `wait_until_ciaa_timer_bit_clear`: Calls a subroutine, then repeatedly tests bit 0 of CIAA control register A and decrements d0 until the bit is clear and d0 reaches zero, returning zero.
- `poll_ciaa_cra_bit_then_decrement`: Tests bit 0 of CIAA control register A and branches if set, otherwise decrements d0 and branches if it became zero, without calling anything.
- `set_cia_timer_a_and_control`: Writes 8 to CIAA control register A, 204 to timer A high byte and 2 to timer A low byte, then returns.

### logged disk load from Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf

- `init_subsystems_and_patch_selfmod_code`: Calls thirteen setup routines, sets a flag word to 1, then computes an address from a longword and stores a constant there before returning.
- `branch_to_shared_code_block`: Unconditionally branches to another code block, transferring execution without touching registers or memory.
- `main_game_loop_dispatch`: Copies a longword from a5 to a1, patches a return opcode into memory, then repeatedly calls dozens of game subroutines and branches on flags at fixed globals to drive the main loop.
- `patch_pointer_table_entry`: Reads a longword from a fixed address, subtracts a constant and one, then writes a constant longword into memory at an offset derived from that value.
- `fall_through_entry`: Entry point part-way through straight-line code: runs three instructions (movep.l, bclr.b, bset.b) and then falls through into the code that follows, with no return of its own.
- `read_two_words_and_call_blit`: Saves all registers, reads two words from a0 into d1 and d2, sets a0 from d0, loads d3 with a bit mask and a1 with a fixed address, then calls a subroutine.
- `read_two_words_and_call_disk_helper`: Saves all registers, reads two words from the address in a0 into d1 and d2, moves d0 into a0, clears d0, sets d3 to 32769 and a1 to a fixed buffer, then calls the disk helper at 7580.
- `compute_track_sector_and_read_disk_block`: Saves registers, derives a track and sector from d0-d3 and a0/a1, loops calling disk read helpers to load a block, and returns the computed block offset in d1 with status in d0.
- `retry_disk_block_reads_with_cia_check`: Loops up to five times checking the CIA fire button, writing sync words into a buffer, and calling disk read helpers until a block is read or the retries run out.
- `play_sample_with_dma_retry`: Clears a word at offset -6 from a6, calls two setup routines, then up to five times checks the CIAA fire bit, programs ADKCON, DSKPTH, DMACON and DSKLEN on the custom chip base, and calls a status routine until it succeeds.
- `scan_eleven_slots_for_target`: Loops up to eleven times calling a slot scanner and a position check, and on a match stores the adjusted column and remaining count at offsets -12 and -10 from a6, returning zero or an error code.
- `advance_through_eleven_room_slots`: Walks the eleven room slots at a5 plus 1024 indexed by the word at -8 from a6, testing each slot's flag and object pointer, copying and comparing buffers, and returns zero, minus one or an error code.
- `scale_d3_by_200_minus_offset`: Copies d3 into d1, subtracts the word at offset -16 from a6, multiplies by 512 in d0, leaving the scaled result in d1 and returning.
- `set_bit_d3_and_increment_counter`: Reads the word at offset -4 from a6, sets the bit indexed by d3, writes it back, then increments the word counter at offset -6 from a6 and returns.
- `clear_eleven_words_at_a5_offset`: Loads a5 into a0, adds 1088 to a0, then stores zero to that longword and each following 1088-byte step for eleven iterations before returning.
- `read_four_bytes_from_a5_offset_8`: Calls the bit-interleave routine on the four bytes at offset 8 from a5, then splits the result into d3 (low byte), d2 (second byte), d1 (third byte) and d0 (high byte shifted right eight), returning with d0 compared against 255.
- `interleave_two_longwords_bits`: Reads two longwords from the address in a0, advancing a0 by eight, masks both with alternating bits, doubles the first and ORs in the second, returning the interleaved value in d0.
- `compare_interleaved_words_at_a5_offsets`: Calls the routine at offset 240, saves its result on the stack, interleaves the longword at offset 48 from a5, then compares the two values and returns the comparison result in d0.
- `load_a5_offset_into_a0_set_d1_count`: Loads the address eight bytes past a5 into a0 and sets d1 to 40, preparing arguments for the following code.
- `xor_words_and_mask_parity_bits`: Saves d2, XORs consecutive longwords from a0 into d0 for d1 shifted-right-two minus one iterations, restores d2, masks d0 with alternating bits and returns.
- `fill_1024_bytes_with_aa_pattern_2`: Reads a pointer from minus 24 of a6, fills 1024 bytes ending there with the repeating byte pattern 170 using d0-d7, then returns.
- `setup_sprite_descriptor_three_planes`: Calls the blitter setup, then stores three plane pointers derived from a0 and a1 with one subtracted, sets control words 7640 and 2, and stores a shifted blit size in a2's structure.
- `draw_four_blits_and_three_clear_lines`: Calls the blitter setup, then issues four blit descriptor writes with different control words and sizes, and finally calls the line routine three times advancing a0 by d1 each time.
- `wait_for_blitter_2`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `init_blitter_and_clear_registers`: Loads the custom chip base into a2, calls the blitter wait, enables DMA with 32832, sets first and last word masks to all ones, fills data with alternating bits, and clears three blitter control words.
- `interleave_d0_halves_via_2384`: Pushes d0 on the stack, shifts d0 right by one bit, calls the bit-interleave routine, restores d0 and calls the interleave routine again.
- `toggle_bit7_by_flag_at_a0_minus1`: Reads byte at a0, checks bit 0 of the byte before it; clears bit 7 if set, otherwise sets bit 7 unless bit 6 is set, then writes back.
- `interleave_d0_bits_into_a0`: Merges the alternating bits of d0 with a shifted copy to interleave them, optionally clears the top bit based on the byte before a0, stores the result at a0 and advances a0.
- `start_disk_dma_read`: Points a1 at the custom chip base, clears ADKCON, enables disk and bitplane DMA, sets sync word, stores a0 as disk pointer, sets DSKLEN length from d0 shifted right and enables the transfer.
- `wait_disk_dma_and_check_status`: Loads the custom chip base into a1, calls two subroutines with a delay value, tests the disk status bit and returns 255 or 0 depending on the result.
- `acknowledge_disk_interrupt_and_stop_dma`: Writes to INTREQ to clear the disk interrupt bit, writes to DSKLEN to disable disk DMA, tests d0 and returns to the caller.
- `set_adkcon_bit10_and_test_flag`: Writes the value 1024 to the ADKCON hardware register, tests the word at offset -30 from a6, and if negative loads 255 into d1.
- `toggle_ciab_disk_select_line`: Writes d1 to CIAB port B, reads the word at offset -36 from a6, adds 3, clears then sets that bit in d1 and rewrites the port.
- `assert_ciab_reset_then_delay`: Sets d1 to 255 and writes it to CIAB port B, clears bit 7 of d1, calls the disk-select toggle routine, then delays with 200 in d0 before returning.
- `step_platform_toward_target`: Saves d2-d3, reads the platform speed table indexed by the object offset at -36(a6), and repeatedly calls the move routine to step the platform toward the target speed in d2, then stores the new speed back into the table and returns zero in d0.
- `wait_for_fire_button_release`: Polls CIAA_PRA bit 4 up to 85 times, calling the disk motor routine while the fire button is held; clears the object's speed table entry and returns zero when released, otherwise returns 30.
- `toggle_disk_motor_select_line`: Saves d0, calls the drive select routine, clears CIAB_PRB bits 0 and 1 to switch the disk motor, then sets bit 0 again, restores d0 and calls the delay routine before returning.
- `write_ciab_prb_disk_control`: Calls the disk control helper, then writes the returned byte in d0 to CIAB port B at the disk control register and returns.
- `build_ciab_prb_drive_select_byte`: Reads CIAB port B, sets bits, clears the drive-select bit indexed by the word at offset minus 36 of a6, optionally clears bit 2 from a table lookup, and returns the byte in d0.
- `poll_ciab_disk_ready_with_retry`: Calls the disk status helper, tests bit 0 of CIAA control register, loops while busy, decrements d0 and retries until d0 reaches zero, then returns.
- `wait_cia_timer_then_decrement_d0`: Tests bit 0 of the CIA-A control register at a fixed address, branches away if set, otherwise decrements d0 and branches away if it becomes zero.
- `set_cia_control_and_timer_latches`: Writes 8 to the CIA-A control register, then writes 204 and 2 to the CIA-A timer high and low bytes, and returns.
- `copy_d0_word_to_125a6_and_call`: Copies the word at address 125a6 into address 125a6 plus two, then calls the routine at 78fe and returns.
- `compare_125a6_with_125a8_and_branch`: Loads the word at 125a6 into d0, compares it with the word at 125a8, and if equal calls the routine at 4674 before returning; otherwise falls through to set d0 to three and call 998c.
- `spawn_effect_from_slot_table`: Calls a setup routine, reads a slot counter, clamps it to four, indexes a pointer table, copies two graphics blocks to screen memory, then writes a value into a computed address.
- `write_rts_word_and_return`: Branches away to another routine, then writes the word value for an rts instruction to the address in a0 and returns.
- `reset_game_state_variables`: Clears many word variables in memory to zero, sets a few flags to one and one to 256, then zeroes four entries spaced 88 bytes apart in a table pointed to by a6.
- `copy_state_words_to_buffer`: Copies eight word values from game state variables into consecutive words starting at the buffer at 2FD0, then returns.
- `call_refresh_and_load_state_registers`: Calls a subroutine, then loads eight words from the 2FD0 buffer into d0 through d7 and returns.
- `player_control_and_physics_update`: Reads joystick bits in d0 and d7, updates player position, velocity and state words in memory, calls movement and sound routines, and writes sprite coordinates.
- `check_fire_button_spawn_projectile`: Tests bit 7 of d0 for the fire button, sets the projectile-active flag, and if no projectile exists and ammo is available calls the spawn routine.
- `spawn_projectile_with_direction`: Plays a sound, marks the projectile active, decrements ammo, sets projectile position from player coordinates and chooses horizontal velocity and sprite based on facing.
- `update_projectile_position_and_collision`: Checks the active flag, computes a screen position from the object's x/y and velocity, calls the collision routine, and updates the stored position or clears the active flag.
- `update_player_movement_and_collision`: Reads joystick input, updates the player's velocity, position and animation, calls the collision routine, and handles death or level transitions.
- `read_next_scripted_byte`: Returns the next byte from a scripted byte stream, refilling from the pointer and count, or returns a fixed byte when the stream is disabled.
- `clear_sound_state_if_active`: Tests the word at a fixed address; if zero returns immediately, otherwise clears that word and resets the sound state words and pointer at a fixed address-a fixed address before returning.
- `start_sound_from_channel_table`: Clears the sound flag, checks the active word, calls the sound setup and stop routines, then loads channel parameters from the table into d0-d7 and runs the sound start and clear routines.
- `call_sub_31fe`: Branches to the subroutine at the fixed target address, pushing the return address onto the stack first.
- `save_registers_and_run_frame_setup`: Loads a base pointer into a6, calls several setup routines, stores d2 through d7 into fixed memory words, runs more routines, sets a flag word to all ones, calls another routine, clears the flag and returns.
- `scan_object_table_and_apply_flags`: Walks a four-byte-entry table, matches type and position bytes against current values, sets flag words and calls a routine with computed offsets, then restores registers and returns.
- `toggle_sound_flag_and_call`: Tests the word at absolute address 16afa; if zero stores 232 there and calls the routine at 8cc8, otherwise stores zero and calls the routine at 8d90, then returns.
- `load_level_pointer_table_entry`: Saves d0-d1/a0, indexes the table at 54c00 by d0 times eight, stores derived pointers and d1 into globals 6600-6608, 44fa and 4662, restores registers, then calls two routines and returns.
- `count_down_scan_table_words`: Saves d0/a0, reads the word at 125a6, subtracts one, doubles it to index the table at 1466a, then repeatedly calls the routine at 469e while decrementing d0 until it reaches zero, restores registers and returns.
- `clear_high_bit_walk_entity_list`: Saves d0/a0, indexes a table at a fixed base by d0 times eight, then walks four-byte records clearing bit seven of each first byte and skipping extra records until a zero byte is found.
- `advance_entity_pointer_past_visible`: Saves d0/a0, loads a pointer from a global, and advances it through four-byte records while the first byte is nonzero and the second byte times eight stays below a global limit, storing the result.
- `scan_entities_and_dispatch_collision`: Checks two global flags, then walks four-byte entity records up to a limit, testing flag bits and calling collision or spawn routines before restoring registers and returning.
- `dispatch_object_byte_to_three_handlers`: Saves d0, reads the byte at a0, and calls one of three routines depending on whether it is 120, 124, or between 1 and 116, then restores d0 and returns.
- `init_entity_from_type_table_entry`: Calls a collision check, then reads a type table entry via a0 to fill a6 fields: sprite pointers, animation, position, and flags, clearing counters and setting the high bit at a0.
- `init_projectile_entity_from_type_byte`: Calls a collision check, then uses the low bits of the byte at a0 to select a type table entry, storing animation, position, velocity and flags into a6 and marking a0.
- `push_d0_set_a6_and_jump_to_4b18`: Pushes d0 onto the stack, loads a6 with the absolute address 93576, sets d0 to 5 and branches to the shared scan loop at 4b18.
- `scan_four_objects_for_zero_state`: Pushes d0, walks up to four 88-byte records from the absolute base 93576, and returns with carry clear if a record word is zero, carry set otherwise.
- `check_a6_object_inside_screen_bounds`: Pushes d0, reads the words at offsets 6 and 2 of a6, and returns carry clear only if both lie within the screen limits, carry set otherwise.
- `init_two_sprite_block_arrays`: Saves d7 and a6, then calls a per-block setup routine four times over one array and six times over a second array, stepping 88 bytes each time, before restoring registers.
- `clear_channel_flag_and_clear_bit7`: If the word at a6 is nonzero, zeroes it, loads the pointer at offset 42 into a0, and if non-null clears bit 7 of the byte it points to.
- `clear_channel_flag_unless_bit6_set`: If the word at a6 is nonzero, zeroes it, loads the pointer at offset 42 into a0, and clears bit 7 of its byte only when bit 6 of byte 3 is clear.
- `process_object_collision_flags`: Iterates over up to three sub-entries of the object at a0, testing flag bits in each entry's byte 3 and calling collision checks against the player and object tables, clearing bit 7 on failure and returning carry set on collision.
- `check_player_against_screen_bounds`: Returns carry clear if the player position in d0/d1 plus d2/d3 extents lies within the screen bounds at the global coordinate words, otherwise returns carry set; skips check when the pause flag is set.
- `check_player_hits_active_enemy`: If the enemy-active flag is set, loads two global words into d6/d7 and calls the overlap test, returning carry set when the player overlaps the enemy, otherwise carry clear.
- `check_two_flags_then_point_in_rect`: Saves d6-d7, tests two global word flags and, if both set, loads two global words into d6/d7 and calls the point-in-rectangle test, returning carry set on hit.
- `check_flag_then_point_in_rect`: Saves d6-d7, tests one global word flag and, if set, loads two global words into d6/d7 and calls the point-in-rectangle test, returning carry set on hit.
- `point_in_rectangle_test`: Saves d6-d7, subtracts d0 from d6 and d1 from d7, compares against d2 and d3 bounds, and returns carry set if the point lies inside the rectangle.
- `check_entity_against_expanded_bounds`: If the word at offset 78 of the a2 structure is zero, tests d0 and d1 against bounds built from words at offsets 2 and 6 plus d2 and d3, returning carry set when inside and clear when outside.
- `check_entity_against_stored_extent_bounds`: Tests d0 and d1 against bounds formed from the words at offsets 2 and 6 of the a2 structure plus the extents at offsets 38 and 40, returning carry set when inside and clear when outside.
- `iterate_six_entity_slots_and_dispatch`: Returns immediately if the global word is nonzero; otherwise walks six 88-byte slots from the entity table, calling the sub-routine for each slot whose first word is nonzero.
- `player_control_state_machine`: Reads the player object at a6, tests input bits and flags, updates position, velocity and animation pointers, and calls collision and sound routines before returning.
- `iterate_four_player_slots`: Loops four times over player slot structures spaced 88 bytes apart starting at a fixed address, calling the per-slot update only when the slot's first word is nonzero.
- `player_physics_and_input_update`: Updates one player slot's velocity, position, collision flags and animation state from joystick bits and global tables, calling several helper routines and returning when done.
- `check_entity_type_and_flag_bits`: Saves d5, reads the word at offset 2 of a6, masks its low nibble, and if it equals 4 calls a routine then tests the low two bits of a global byte, returning carry set when they are zero and clear otherwise.
- `scan_four_slots_for_free_entry`: Saves d7 and a2, walks four 88-byte slots starting at a global table, and for each active slot with two zero words calls a test routine, setting word 76 to minus one when the test fails.
- `check_state_one_and_probe_region`: Returns immediately unless the global word at the state address equals one, then saves d4-d7, loads coordinates and sizes into d4-d7, calls a region test, and sets a global word to minus one if it fails.
- `clear_word_at_1594a_and_return`: Writes the word value zero to the long absolute address 1594a, then returns to the caller.
- `start_timer_and_play_sound_17`: Tests the timer active flag; if zero, sets it to -1, stores 8192 and 25 as timer values, then calls the sound routine with d0=17 and d1=0.
- `stop_timer_and_clear_sound_bit`: If the timer active flag is set, clears it, passes the stored timer value to a conversion routine, calls the sound routine with d0=1, then sets bit 7 at (a0).
- `tick_timer_and_bcd_countdown`: If timer active, decrements the countdown word; when it reaches zero reloads 25, subtracts BCD bytes from the counter via sbcd, and clears the active flag if the timer value is zero.
- `init_five_sprite_slots`: Saves d7/a0/a6, writes 20 to a global, then fills five 88-byte sprite slots from a source table, calling a setup routine per slot, and clears several state words before returning.
- `update_player_and_check_collisions`: Tests two state words, moves five sprite slots by deltas, then runs bounding-box collision checks against obstacles and triggers death or respawn handling before returning.
- `init_shot_velocity_from_player_and_target`: Decrements a cooldown word and returns if nonzero; otherwise calls a sound routine, sets timer and sprite fields, computes velocity deltas from player and target positions, halves them until within four pixels, and stores the result.
- `move_shot_and_check_collision`: Decrements a timer, adds velocity to the shot position, writes the position to a sprite, checks tile collision and a flag bit, and on hit sets a hit marker and clears the active flag.
- `reset_five_sprite_objects_and_report`: Saves d7 and d0-d1, calls the sound routine, then loops five times setting the active word of each 88-byte sprite object to one, decrements the shared counter and returns carry set if it reached zero.
- `scan_tilemap_rows_for_collision_flags`: Saves registers, sets scroll and collision globals, reads tile indices from a1 through the tile table at a0, ORs their attribute bits and stores a collision flag word.
- `scan_tilemap_and_update_collision_state`: Saves registers, sets scroll globals, ORs tile attribute bytes from a1 via the table at a0, stores the combined flags, then walks the object list testing bounding boxes and updating collision fields.
- `rectangles_overlap_test`: Saves all data registers, adds d4 to d6 and d5 to d7, compares d0 and d1 against those sums and d2/d3 against d4/d5, then restores registers and returns carry set if the two rectangles overlap, clear otherwise.
- `scan_objects_for_collision_flag`: Saves registers, builds a lookup table via a call, reads a byte from a1 as an index, tests the mapped byte's bit 1, then walks 44-byte object records checking type bits and calling a collision test, setting bit 1 of a flag word when a hit occurs.
- `scan_terrain_tiles_for_player_collision`: Saves registers, builds a collision bitmask from tile bytes at the level map, then walks the object list checking flagged entries against the player position and sets contact flags and stored coordinates.
- `compute_tilemap_row_pointer`: Loads the tilemap base at 65300, adds the row offset from 160ae shifted left twice and the column offset from 160ac divided by eight, returning the tile pointer in a1.
- `blit_tilemap_column_strip`: Saves registers, computes a tilemap pointer from 16008 and 16000, stores it and a remainder count, then calls the column blit and fill routines ten times to draw a vertical strip.
- `copy_tilemap_block_and_advance_column`: Saves registers, copies 39 rows of 32 bytes from the source tilemap to the destination buffer, then calls the tile blitter for the current column and advances the column index and source pointer.
- `scroll_background_tile_columns`: Saves registers, copies thirty-nine rows of eight longs from one screen buffer backwards to the next, decrements a column counter modulo four and shifts the source pointer when it wraps.
- `blit_tile_column_from_tileset`: Saves registers, selects a tileset plane from the index in d0, then copies eight tile rows of four bytes each from the tileset into the destination buffer, advancing the destination by 32 bytes per row.
- `copy_eight_sprite_blocks_from_table`: For eight entries, reads a byte from (a0), scales it by sixteen, and copies four longwords from the table at a fixed address into (a1), 28(a1), 60(a1) and 92(a1), advancing a1 by one longword each time.
- `reset_counter_and_call_two_routines`: Clears the word at a fixed address, copies the word at a fixed address masked with seven into a fixed address, then calls the routines at a fixed address and a fixed address before returning.
- `check_player_x_bounds_and_step`: Tests the pause flag; if clear, reads the player x position, masks it to 8 pixels, compares against a limit and calls the left or right movement routine.
- `move_player_right_clamped`: Adds the horizontal velocity to the player x position, clamps it to the right limit, stores the new position and calls the scroll and animation routines when the pixel column changes.
- `move_player_left_clamped`: Adds the horizontal velocity to the player x position, clamps it to the left limit, stores the new position and calls the scroll and animation routines when the pixel column changes.
- `advance_byte_counter_mod_256`: Saves d0, loads the word at the counter address, adds the word at the increment address, masks the result to a byte, stores it back to the counter, restores d0 and returns.
- `clear_entity_records_until_negative`: Walks a6 through records of 44 bytes, clearing the first word and several later word and long fields, stopping when the first word is negative, then returns.
- `reset_sound_state_via_pointer`: Saves a1 and a6, loads a1 from the global pointer, points a6 at the sound state area, calls the reset routine, restores a1 and a6 and returns.
- `load_fixed_68000_source_into_a1`: Saves a1 and a6 on the stack, loads the fixed address 68000 into a1 and 16940 into a6, calls the shared copy routine, then restores a1 and a6 and returns.
- `load_pointer_from_192a0_into_a1`: Saves a1 and a6 on the stack, loads the longword stored at 192a0 into a1 and 16940 into a6, calls the shared copy routine, then restores a1 and a6 and returns.
- `run_two_stage_copy_sequence`: Calls the first copy stage at 7272 and then the second stage at 7290 before returning to its caller, performing both steps of the shared copy sequence.
- `activate_idle_objects_in_list`: Walks a linked list of object records at a6, and for each active record whose field at offset 20 is zero, calls the object activation routine, advancing by 44 bytes per record.
- `deactivate_active_objects_in_list`: Walks a linked list of object records at a6, and for each active record whose field at offset 20 is nonzero, clears that field to zero and calls the object update routine, advancing by 44 bytes per record.
- `draw_object_sprite_by_type`: Saves registers, selects a sprite base table from the object type at offset 14, computes the sprite address, then draws the object at its x and y coordinates via one of two draw routines depending on the flag at offset 18.
- `add_delta_to_active_objects`: Walks a table of 44-byte object records starting at a fixed address, skipping records whose first word is negative or zero, and adds the incoming d0 word to the word at offset 6 of each active record.
- `advance_script_stream_pointer`: Steps the script pointer at offset 30 of a6 through command words, handling jump, delay and call opcodes, updating the current command word at offset 14 and returning carry set when a command was consumed.
- `read_next_script_pair`: Reads the next byte pair from the script pointer at offset 22 of a6, decrementing the repeat counter at offset 26, and returns the two sign-extended bytes in d0 and d1 with carry set when data was read.
- `advance_script_stream_pointer_2`: Reads the script pointer at offset 22 of a6, skips zero and marker words, optionally calls a routine for marker entries, loads a count and two signed bytes, advances the pointer and returns with carry set or clear.
- `reset_script_slot_table`: Saves a6, clears the slot count word, stores the table base pointer, then walks fourteen-byte records clearing each first byte until a negative byte ends the loop, restoring a6 and returning.
- `spawn_effect_entry_with_position`: Checks the effect count against forty, then stores a type byte, a random variant, and the a0, a3 and a4 pointers into the next free fourteen-byte effect slot and increments the count.
- `advance_effect_positions_forward`: Walks the fourteen-byte effect slots, adding thirty-two to each active slot's position field and freeing slots whose position passes the limit, decrementing the effect count.
- `retreat_effect_positions_backward`: Walks the fourteen-byte effect slots, subtracting thirty-two from each active slot's position field and freeing slots whose position falls below the limit, decrementing the effect count.
- `reset_sound_channel_state_block`: Writes zero to three bytes and a longword, zero to two words, then sets two words to minus one and one word to six in the sound state block, and returns.
- `reset_two_sound_channel_pairs`: Writes six and minus one to two pairs of words in the sound state block, setting two channel periods and volumes, then returns.
- `increment_score_display_counter`: Pushes d0, loads the word at a fixed address, increments it, clamps it to at most 6, stores it back, sets the word at a fixed address to all ones, then restores d0 and returns.
- `clear_three_score_entry_slots`: Saves registers, optionally clears a four-byte buffer, then for three slots blanks the six-byte text field, copies a digit byte into it, calls the text routine, zeroes the slot and advances by fourteen bytes.
- `bcd_add_score_and_split_digits`: If the flag word is zero, saves registers, stores d0, adds three bytes of d0 into a buffer with decimal adjust, splits three bytes into nibble digits in memory, restores registers and sets the flag word to all ones.
- `run_three_delay_loops_by_counter`: Loads three separate word counters from fixed memory, and for each counter repeatedly calls the delay routine with different immediate values until the counter expires, then returns.
- `toggle_flag_and_run_four_step_sequence`: Clears a word flag, sets it to all ones if a status bit is set, then conditionally runs a four-step call sequence with carry checks and finally stores all ones to a global word.
- `draw_sprite_via_7afa`: Saves a0-a1, calls the setup routine, loads a1 from a global pointer and a0 from a fixed table, then calls the sprite blitter and the wait routine before restoring a0-a1 and returning.
- `decode_rle_sprite_to_buffer`: Saves d0-d3/a0-a3, reads a count from (a0)+, then expands a bit-flag-driven run-length stream from a0 into the byte buffer at a1, decrementing d0 until done and restoring registers.
- `menu_cursor_input_loop`: Initialises menu selection words, checks a flag and a count, draws the menu, then polls joystick bits to move the cursor up or down and stores the chosen index before returning.
- `draw_status_bar_if_active`: Returns immediately when the display flag is zero; otherwise saves registers, loads dimensions twelve by twenty-one and a graphic pointer, selects an alternate pointer when the second flag is set and calls the blitter routine.
- `init_sound_and_set_mode_two`: Returns early if the word at 4bd34 is nonzero; otherwise loads d0 with 9 and d1 with 0, calls the sound setup routine, then calls the mode routine with d0 set to 2 and returns.
- `set_mode_one_if_idle`: Tests the word at 4bd34 and returns immediately when it is nonzero; otherwise loads d0 with 1, calls the mode routine at 8324 and returns to the caller.
- `init_level_and_wait_for_input`: Calls setup routines, sets four word variables to minus one, draws a screen region, then polls the input flag and a hardware bit until a key is pressed before calling the exit routine.
- `poll_input_until_key_or_timeout`: Saves d0, d1 and d7, loops up to 175 times calling the input routine and checking the input flag, clearing a word variable, and returns carry set on keypress or clear on timeout.
- `toggle_screen_buffer_on_key_64`: If the byte at the input location equals 64, calls the setup routine and switches the display pointer between two screen buffers before calling the restore routine.
- `flash_screen_rows_decrement_increment`: Saves d0-d1/a0-a1, calls four subroutines, decrements each byte of eight rows of ten bytes at the screen buffer, calls two more subroutines, then increments the same bytes back and restores registers.
- `high_score_table_entry_and_name_entry`: Compares score against eight high-score slots, shifts entries down, stores the new score, draws the table and lets the player type a name with joystick, then writes the name into the table.
- `redraw_cursor_and_name_buffer`: Converts the grid cursor position d2/d3 and the name-entry index d7 into screen coordinates and calls the draw routine three times to redraw the cursor, the name buffer and the current character.
- `run_level_script_dispatch_loop`: Checks a global flag and returns early if set, otherwise clears state words, initialises several globals, then repeatedly reads script indices and calls the selected routine until a stop flag is set.
- `initialize_record_fields`: Reads two words from a structure pointed to by a0, calls another routine with the data, then writes a series of values including a fixed word value of 1 and several zero values to a structure pointed to by a2, before updating the pointer in a0.
- `set_sprite_zero_and_advance_source`: Reads a word from offset two of a0 into d2, calls the shared setup routine, writes zero to the word at a2, advances a0 past four bytes and returns.
- `init_object_velocity_and_advance_source`: Reads a word from offset two of a0 into d2, calls the shared setup routine, clears the long at offset twenty-two of a2, copies two words from a0 into a2 offsets ten and twelve, then advances a0 by eight.
- `set_object_pointer_from_offset_table`: Reads a word from offset two of a0 into d2, calls the shared setup routine, adds the word at offset four of a0 to the base at 55400, stores the result at offset thirty of a2, clears the word at offset thirty-four and advances a0 by six.
- `advance_a0_by_script_entry_size`: Tests the word at offset 2 of a0; if zero it calls the routine at a fixed address then advances a0 by 4, otherwise it indexes the table at a fixed address by that word and jumps to the routine at 36982 before advancing a0 by 16.
- `scan_ff_terminated_bytes_align_a0`: Saves d0-d1/a0, calls a routine with the words at offsets 2 and 4 of a0 and a0+6, then scans bytes from a0+6 until a byte equal to 255, aligns the resulting pointer to even and returns it in a0.
- `skip_four_bytes_in_a0`: Adds 4 to the a0 pointer and returns immediately to the caller, leaving all other registers unchanged.
- `call_bitmask_update`: Saves d0 and d1, loads a word from offset 2 of the structure in a0 into d0, calls another routine, restores d0 and d1, sets a0 to offset 4 of the structure in a0, and returns.
- `update_entity_positions_and_effects`: Loops over entity records from a0, advances each active record's x and y by velocity or a called routine, then runs effect and sound updates based on the game mode word.
- `set_sprite_pointer_from_offset`: Reads a word from a0, calls a routine, then stores a3 pointing into the sprite base plus the offset from a0 into a2 fields and clears two words.
- `advance_a0_by_sixteen_and_return`: Adds sixteen to address register a0 and returns immediately to the caller without touching any other register or memory.
- `update_global_pointer`: Adds 4 to a global variable, reads a word from the address in a0, and returns a new address in a0 calculated by adding that word to a constant.
- `load_pointer_from_memory`: Loads a long value from a specific memory location into address register a0 and returns immediately without modifying any other state.
- `advance_pointer_by_four`: Increments the pointer in address register a0 by four bytes and returns immediately without performing any other operations.
- `index_58_byte_record_table`: Multiplies d2 by 88, loads the base address of a record table into a2, adds the product to a2 and returns with a2 pointing at the selected record.
- `seed_two_word_prng_state`: Builds two initial pseudo-random words from immediate constants, shifts and mixes them, then stores the results into the two-word state at the fixed global addresses.
- `advance_two_word_prng_state`: Loads the two-word pseudo-random state, swaps the words, rotates and mixes them with a subtract and exclusive-or, stores the new state and restores d6 and d7.
- `copy_three_buffers_twenty_six_times`: Saves d0-d2 and a0-a4, then calls a copy routine on three buffer pairs once, then repeats it twenty-six times using d2 as the counter before restoring registers and returning.
- `blit_tile_column_pair_to_bitplanes`: Reads two tile indices from the byte stream at a0, looks each up in the tile table at a5, and copies 40 bytes per tile into four interleaved bitplanes at a1 and the mask plane at a2, advancing pointers and repeating for sixteen rows.
- `scroll_playfield_rows_by_offset`: Reads the vertical scroll offset, computes wrapped source and destination pointers into the playfield buffer at a1, and calls the blitter copy routine to shift rows of the screen buffer.
- `blit_four_bitplane_block_with_mask`: Returns immediately if d0 is zero; otherwise programs the blitter control and modulo registers and issues four blits copying 8192-byte blocks from a0 to a1, waiting on the blitter busy bit between each.
- `blit_tile_row_offset_8`: Saves registers, computes a tile source offset from the word at the tile index variable, subtracts 8, scales it, sets a0/a1/a2 to tile, bitmap and mask bases, then calls the blitter routine.
- `blit_tile_row_offset_208`: Saves registers, computes a tile source offset from the tile index variable, adds 208, scales it, sets a0/a1/a2 to tile, bitmap and mask bases, then calls the blitter routine.
- `render_level_tilemap_to_bitplanes`: Clears the tile index variable, then copies 24 rows of 32 tile bytes from the tile table into four bitplanes and a mask buffer, honoring the scroll offset in the scroll variable.
- `run_forward_animation_sequence`: Calls setup routines, clears and sets several global words, then loops sixteen times stepping a0 backward by two bytes while calling two subroutines, ending with a final call.
- `draw_two_shifted_sprite_frames`: Saves a0-a2, loads two pointers from globals, offsets each by 324 bytes, calls two drawing routines with a2 advanced by two, then restores registers and returns.
- `run_backward_animation_sequence`: Calls setup routines, clears and sets several global words, then loops sixteen times stepping a0 forward by two bytes while calling two subroutines, ending with a final call.
- `blit_two_playfield_buffers`: Saves a0-a2, loads two pointers from globals, offsets them by 326 and 324, calls the wide blit then the narrow blit, restores registers and returns.
- `blit_four_wide_blocks_8192`: Sets both blit modulo registers to 10, then four times waits for blitter idle and starts a 12303-sized blit advancing source and destination by 8192 bytes each.
- `blit_four_narrow_blocks_8192`: Sets both blit modulo registers to 38, then four times waits for blitter idle and starts a 12289-sized blit advancing source and destination by 8192 bytes each.
- `blit_four_8k_planes_to_screen`: Saves a0/a2, sets blitter control and modulo registers, then four times waits for blitter idle and copies 8192 bytes from chip memory to the screen bitmap, advancing both pointers.
- `draw_tile_block_from_tilemap`: Saves all registers, computes destination bitmap and source tilemap addresses from the descriptor in a0, then for each tile row and column copies 32 bytes of tile graphics into four bitplanes.
- `blit_sprite_tiles_to_bitplanes`: For eight sprite entries, advances the animation frame, reads a 40-byte tile from chip memory and copies it into four interleaved bitplanes plus a mask plane at the pointers stored in each entry.
- `save_level3_and_level2_vectors`: Copies the level 3 and level 2 autovector interrupt vectors from low memory into two longwords in the game's data area, then returns.
- `disable_dma_interrupts_and_clear_buffer`: Writes all bits to DMACON and INTENA to disable DMA and interrupts, then zeroes a large block of memory starting at the buffer address before returning.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `init_bitplane_and_display_window`: Writes the bitplane modulo, control, collision, data-fetch and display window registers to set up a full-screen low-resolution playfield.
- `install_level_interrupt_vector`: Stores the address of the level interrupt code into the level 6 autovector and sets a flag word in memory to two.
- `level_interrupt_acknowledge`: Saves all registers, increments a frame counter, calls two subroutines, restores registers, acknowledges the level 6 interrupt and returns from exception.
- `enable_dma_and_interrupts_set_supervisor`: Writes the DMA and interrupt enable masks to the custom chip registers, sets the supervisor bit in the status register and clears all interrupt requests before returning.
- `enable_bitplane_dma_clear_blitter_control`: Enables bitplane DMA via DMACON, sets both blitter first and last word masks to all ones and clears the blitter control registers before returning.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_9430_with_saved_a0`: Saves a0 on the stack, loads a fixed address into a0, calls the routine at the following address, then restores a0 and returns.
- `copy_16_colors_shifted_to_color_registers`: Saves d0-d1/a1, stores a0 to a global pointer, then copies 16 words from (a0) into COLOR00 registers, each value shifted left by one, and restores registers.
- `fade_palette_up_by_bit_planes`: Copies 16 colors to a buffer, then over 8 passes raises each color's red, green and blue components when above thresholds and writes doubled values to COLOR00.
- `fade_palette_down_by_bit_planes`: Copies 16 colors to a buffer, then over 8 passes lowers each color's red, green and blue components when nonzero and writes doubled values to COLOR00.
- `reset_counter_if_at_end`: Saves d0-d1, loads two counters from absolute memory, and clears the first counter when the second minus one is not greater and the values match.
- `call_reset_when_counter_at_end`: Saves d0, loads the second counter minus one, compares it with the first counter, calls the reset routine when not greater, then restores d0 and returns.
- `toggle_six_playfield_bytes`: Flips bit 7 of six longword-addressed bytes in memory, toggling each byte's high bit, then returns to the caller.
- `run_command_list_until_negative`: Saves d0-d1/a0-a1, walks a word list at a0 reading opcode words, and for each non-negative word reads a parameter and pointer then calls the dispatcher; restores registers and returns.
- `blit_tile_column_to_bitplanes`: Saves registers, computes a playfield destination from d0 and d1, then for each byte in the a0 list until a terminator copies eight tile rows into interleaved bitplane offsets and advances to the next column.
- `blit_tile_column_to_bitplanes_2`: Reads tile indices from the byte stream at a0 until a 255 terminator, and for each index copies 32 bytes of tile data from the tile table into four interleaved bitplanes at a1, advancing a1 by one byte per tile.
- `clear_both_playfield_buffers`: Saves a0, loads the two playfield buffer pointers from fixed memory, calls the clear routine on each, then restores a0 and returns.
- `clear_playfield_buffer_32_rows`: Saves registers, then writes 32 rows of 32 zero bytes starting at a0 using a dbra loop over d7, restoring registers and returning.
- `blit_tile_column_to_two_bitplanes`: Copies a 24-row by 16-word tile from the tile data at offset d0 times 1536 into two destination buffers, writing each word to both the first and second bitplane areas 8192 bytes apart.
- `draw_masked_tile_to_four_bitplanes`: Clips the tile at d0,d1 against the screen, then ORs or masked-ORs tile words from a0 into four bitplanes at a1 spaced 8192 bytes apart, advancing 40 bytes per row.
- `draw_masked_tile_with_terrain_mask`: Like the masked tile blitter but also ANDs each tile word with a mask fetched from the terrain buffer at a5, advancing that buffer 40 bytes per row and wrapping after 256 rows.
- `set_cia_interrupt_and_vector`: Stores the long immediate value at absolute address 104, then writes 127 and 137 to the CIA interrupt control register to acknowledge and enable interrupts, and returns.
- `cia_keyboard_interrupt_handler`: Saves d0, reads the CIA interrupt control register, and on keyboard serial interrupt reads the serial data register, inverts and rotates the byte, stores it as a keymap entry, and acknowledges the interrupt before returning from exception.
- `read_joystick_ports_to_flags`: Reads CIA fire buttons and both joystick ports, sets direction and fire bits in a two-word flag area, and restores d0-d1.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `check_pending_flag_and_return`: Tests the word at memory location 4bd34 and returns immediately if it is zero, otherwise falls through to the next routine.
- `select_and_register_sound_slot`: Saves all registers, indexes a table at 1c2f0 by d0, and either starts a pending sound via d1 or picks the least-used of four sound slots and records d0 there before calling the sound start routine.
- `update_sound_state_and_dispatch`: Saves registers, calls the sound tick routine, then depending on the pending-sound and active-sound flags either starts a queued sound with d0 and d1 or calls the sound cleanup routine.
- `reset_sound_channels_and_counters`: Saves all registers, clears four channel bytes at offsets 22, 90, 158 and 226 from a base table, calls a sound reset routine, then zeroes several word counters and restores registers.
- `jump_to_setup_sound_channel_from_table`: Jump stub: passes control straight on to setup_sound_channel_from_tables.
- `silence_audio_channels`: Clears a word value at a memory location, then sets multiple audio hardware registers to silence all audio channels and reset related variables before branching to another routine.
- `setup_sound_channel_from_tables`: Installs a sound interrupt vector, sets channel period and volume, indexes two tables by d0 and d1 to fetch channel parameters, and jumps to the sound playback routine.
- `init_four_player_sprite_slots`: Saves d0, calls a setup routine, writes CIAA and custom chip registers, then fills four sprite slot structures with frame pointers, flags and zeroed fields before returning.
- `tick_sound_timer_and_dispatch_channels`: Calls the sound tick, decrements a countdown byte and reloads it with six, then checks four channel activity bytes and dispatches each active channel to the channel update routine.
- `run_sound_channel_sequence`: Steps the channel's script pointer through command bytes, handling delays, jumps, volume and note commands, and updates the channel state block at a4 before returning.
- `decode_sound_script_command`: Reads the next command byte from a3, selects a sample pointer from the table or stores volume, envelope and effect parameters into the channel block at a4.
- `check_sound_flag_and_clear_bit7`: Branches away if the condition flags are not equal or if byte at offset 64 of a4 is positive; otherwise masks d0 to six bits, and if zero clears bit 7 of that byte before returning.
- `stream_byte_timing_counter`: Reads a byte from the pointer at offset 58 of a4, splits it into a 3-bit repeat count and 5-bit value, adds the base at offset 23 and returns the doubled result in d0.
- `lookup_pattern_byte_and_set_duration`: Builds a pointer from the low byte at offset 64 of a4 into the table at b058, tests the selected byte, and if it is non-negative and bit 6 of the flag byte is clear sets byte 63 of a4 to one and returns.
- `setup_sound_channel_from_stream`: Clears bit 6 of byte at offset 64 of a4, reads two bytes from a3 to set channel period and volume fields at offsets 62 and 63, stores a3 plus one at offset 58, and returns the byte at offset 23 of a4 doubled.
- `update_sound_channel_frequency_slide`: If the flag byte at offset 35 of a4 is set, recomputes the word at offset 38 of a4 from the frequency at offset 8 of a1 and the divisor at offset 37, clears bit 7, decrements counters at offsets 36 and 37, and adds the result to the word at offset 10 of a1.
- `start_sound_channel_playback`: When the counters at offsets 48 and 51 of a4 are active, decrements the reload counter at offset 50, then programs a custom chip audio channel via a6, sets the sample pointer, period, volume and DMA length from the sound descriptor at a0, and installs the playback interrupt routine.
- `silence_all_audio_channels`: Sets volume to maximum on all four audio channels, clears their period registers, and zeroes the period fields of four sound structures before returning.
- `init_audio_and_three_sound_slots`: Calls an audio setup routine, loads the custom chip base into a6, then initializes three sound structures at successive addresses by calling the same per-slot routine.
- `update_sound_slot_playback`: Checks a sound slot's active flag, decrements its counter, calls a helper and a slot callback, then sets period and volume in the audio hardware or clears the slot.
- `check_timer_and_install_next_state`: Tests the word at offset 6 of a3; if negative and the word at offset 14 of a3 is greater than one, loads the address of the following routine into offset 26 of a3, then returns.
- `toggle_sound_and_advance_sequence`: Reads the word at a3, sets its high bit, writes it to offset 150 of a6, decrements the word at offset 6 of a3 and, when it reaches zero, negates it and stores a computed pointer and duration into a1.
- `reset_counter_and_add_frame_delta`: Writes the value 32768 to offset 6 of a3, then adds the word at offset 14 of a2 to the word at offset 12 of a3 and returns.
- `update_actor_state_and_sound_registers`: Tests word at offset 6 of a3; if negative and offset 14 equals one, sets it to 32768, then masks a3 word with a6 offset 2 and either branches, clears offset 12, or decrements offset 6 and writes sound registers at a6 offsets 150, 154 and 156.
- `advance_sprite_frame_and_flip_direction`: Uses counters at a3 offsets 16, 18, 20, 22 and 24 to advance a frame offset, reloading the period from a2 offset 20 and negating the step when the counter expires.
- `acknowledge_and_disable_interrupts`: Saves d0, reads INTENAR and INTREQR, masks bits 7 to 10, writes them to INTENA and INTREQ to clear them, shifts right by 7 and writes to DMACON, restores d0 and returns from exception.
- `decrement_timers_and_clear_flag`: Reads a word counter, and if nonzero decrements two word counters at fixed memory, resetting the first when it expires and clearing a byte flag before branching away.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_constant_address_and_value`: Loads a fixed long address into address register a0 and a fixed long constant into data register d0, then continues into following code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_immediate_address_and_constant`: Loads the immediate long value into address register a0, then loads the immediate long value into data register d0, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_at_405a_then_return`: Calls the subroutine at offset 405a via bsr, then returns to the caller with rts.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `pop_d0_to_d7_and_a0_a1_then_return`: Restores registers d0 through d7 and a0 through a1 from the stack with movem, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_a0_past_four_bytes`: Adds four to address register a0, advancing the pointer past a four-byte field, then returns to the caller.
- `pop_d6_d7_and_return`: Restores registers d6 and d7 by popping them from the stack, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_then_pop_d0`: Calls a nearby subroutine, then pops a longword from the stack into d0 as the return value and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `pop_a1_and_return`: Pops a longword from the stack into address register a1, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `pop_d0_and_return_from_exception`: Pops a word from the stack into data register d0 and returns from the exception with rte.

### Rick Dangerous II (1990)(Micro Style)[cr Angels - Genesis].adf bytes 11264-11776 (code never loaded by a reference run)

- `install_trap0_vector_and_halt`: Loads a nearby address into a0, stores it at absolute address 128 as the trap 0 vector, issues trap 0, then branches and returns from exception.
- `disable_interrupts_dma_copy_to_fast_ram`: Enters supervisor mode, disables all interrupts and DMA, sets the stack pointer, copies 2015 bytes from a nearby table to address an address, then jumps there.
