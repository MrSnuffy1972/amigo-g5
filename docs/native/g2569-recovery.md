# Game 2569 (Qwak): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02569). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905590 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 19 raw-word lines |
| Functions named and described | pass, 78 of 78 named (0 author symbols), 78 of 78 described (11 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1419 of 1422 (99.79%) |
| Recovered without load address (counted in proof 1, flagged) | 4608 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1867 of 1913 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16714 |
| code | 7160 |
| data | 872194 |
| padding | 818 |
| unused | 8704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Qwak.adf:bootblock | 1 | 34 | 3 |
| logged disk load from Qwak.adf boot-block load | 1 | 571 | 37 |
| logged disk load from Qwak.adf boot-block load | 1 | 215 | 10 |
| logged disk load from Qwak.adf | 1 | 7 | 1 |
| logged disk load from Qwak.adf | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 396 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 54 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 591 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Qwak.adf bytes 12288-16896 (code never loaded by a reference run) | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 2 |

## Cases handled

- Qwak/Qwak.adf: custom-format (non-DOS) disk, 1736 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 9 of the 1438 PCs the canonical run logged (1436 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- Qwak/Qwak.adf: the boot block reads 11264 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- Qwak/Qwak.adf: the boot block reads 10752 bytes from disk offset 819200 to an address (found statically); recovered as a unit
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- prefix run f1000 ignored: it logged 7 PCs the canonical run never executed (not a prefix of it)
- prefix run f2000 ignored: it logged 7 PCs the canonical run never executed (not a prefix of it)
- Qwak/Qwak.adf: 4 executed PCs whose RAM was overwritten before any snapshot match its bytes 821248-821312: recovered as a unit run at an address
- Qwak/Qwak.adf: 5 executed PCs whose RAM was overwritten before any snapshot match its bytes 819728-819808: recovered as a unit run at an address
- 882 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 6 first-run image unit(s) in 1 version group(s); 822 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- Qwak/Qwak.adf: bytes 12288-16896 look like code no run loaded: recovered position-independently (load address unknown)
- Qwak/Qwak.adf: 5 logged loads cover 26624 bytes of the image
- Qwak/Qwak.adf: blocks 34-213 (bytes 17408-109568) outside the filesystem hold data, not code (opcode density 0.040, entropy 5.23)
- Qwak/Qwak.adf: blocks 227-679 (bytes 116224-348160) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.75)
- Qwak/Qwak.adf: blocks 682-1200 (bytes 349184-614912) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.64)
- Qwak/Qwak.adf: blocks 1202-1599 (bytes 615424-819200) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.82)
- Qwak/Qwak.adf: blocks 1621-1759 (bytes 829952-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.66)
- EVSNAP: 7 executed PCs no other source explains recovered from 1 first-execution RAM image segment(s) (44 bytes, 5 images read); 7 unmapped before, 0 after
- 1 never-loaded region(s), 4608 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `poll_flag_and_set_timer` (Qwak.adf boot-block load +an address to an address (11264 bytes)): Polls a byte in a structure for a specific bit flag. Kind: disk hardware/sync check. Pass path: bne.b an address.
- `set_adkcon_and_test_a6_flag` (Qwak.adf boot-block load +an address to an address (11264 bytes)): Writes the value 1024 to the ADKCON hardware register, tests the word at offset -30 from a6, branches if non-negative, then loads 255 into d1. Kind: disk hardware/ADKCON check. Pass path: beq.b an address.
- `pulse_ciab_disk_select_line` (Qwak.adf boot-block load +an address to an address (11264 bytes)): Writes d1 to CIAB_PRB, reads the word at offset -36 from a6, adds 3, clears then sets that bit in d1, writing CIAB_PRB between each change. Kind: disk hardware/timing check. Pass path: bra.w an address.
- `assert_ciab_reset_then_delay` (Qwak.adf boot-block load +an address to an address (11264 bytes)): Loads 255 into d1 and writes it to CIAB_PRB, clears bit 7 of d1, calls the disk select pulse routine, loads 200 into d0 and branches onward. Kind: disk hardware/CIAB_PRB check. Pass path: iVar1 == 1.
- `pulse_disk_select_line` (Qwak.adf boot-block load +an address to an address (11264 bytes)): Pushes d0, calls the delay routine, tests d1 and if negative skips the select pulse; otherwise it clears bits zero and one of d0, writes the byte to CIAB port B, sets bit zero again, writes it back, restores d0 and branches to the motor control routine. Kind: disk hardware/timing check. Pass path: sub_0C26 (branch to an address).
- `read_ciab_prb_and_write_disk_select` (Qwak.adf boot-block load +an address to an address (11264 bytes)): Calls the CIAB port B read routine, then stores the returned byte in the disk select register at the CIAB port B address and returns. Kind: disk hardware/CIAB_PRB check. Pass path: the instruction immediately following the call (the code continues execution normally).
- `build_port_byte_from_table` (Qwak.adf boot-block load +an address to an address (11264 bytes)): Builds a port byte from a frame field and a bit index, clearing one further bit when a looked-up table entry has its lowest bit set. Kind: disk hardware/drive selection check. Pass path: sub_0C5A (the branch at an address or the instruction following the call).

## Functions


### Qwak.adf:bootblock

- `bootblock_load_and_jump_to_game`: Calls the disk loader, fills the IO request with the game load parameters, runs DoIO and SuperState, resets the stack and interrupt/DMA registers, then jumps to the loaded code at the game start address.
- `flush_caches_if_kickstart_new_enough`: Reads the exec library version and, when it is at least thirty-six, calls CacheControl with zero and two hundred fifty-five to flush and invalidate the caches before returning.
- `load_game_track_then_clear_color_zero`: Saves all registers, sets up an IO request to read the game track into memory, runs DoIO and the loaded code, restores registers, flushes caches and clears the first color register.

### logged disk load from Qwak.adf boot-block load

- `alloc_chip_and_fast_mem_set_vbr`: Calls exec AllocMem twice for 320KB chip and 256KB fast memory, aligns the result to 512KB, stores it in VBR, disables interrupts and sets the level 3 autovector to the next routine.
- `reset_hardware_and_load_game_module`: Disables interrupts, resets the stack, clears DMACON and INTENA, loads the game module named game from disk to chip memory, then jumps through the exec vector or flashes the color register on failure.
- `find_and_load_module_by_name`: Searches a 100-entry module table for the name in d0, allocates memory via the loader, copies the module bytes to the offset in its header, and returns a status word in the error variable.
- `store_zero_to_600a8_after_call`: Saves a0 on the stack, sets up arguments in d0-d4 and a0 from a nearby table, calls a subroutine, then writes the returned d0 word to absolute address 600a8 and restores a0.
- `modify_custom_registers`: Modifies a custom hardware register and calls another routine that modifies three custom hardware registers.
- `retry_disk_track_read`: Loops up to three times checking CIAA_PRA fire bit, writes sync words to track buffers, calls read helpers and returns a status code.
- `validate_input_with_retry`: Repeatedly calls helper functions to check a flag and decrement a counter, returning early if the flag is clear or the counter expires.
- `scan_level_tiles_for_matching_object`: Walks the level tile map at a5, checks each tile against the object's type and position, compares tile data, and returns zero on success or a nonzero error code.
- `compute_tile_offset_from_column`: Subtracts the level's left column from d3, multiplies the result by 512 and returns the byte offset in d1.
- `mark_tile_visited_and_increment_counter`: Sets the bit for column d3 in the visited-tile mask at offset -4 of a6 and increments the word counter at offset -6.
- `clear_eleven_longwords_at_a5_offset`: Loads a5 into a0, adds 1088 to it, then writes zero to eleven consecutive longwords spaced 272 bytes apart and returns.
- `extract_high_byte_pair`: It calls another routine to obtain a 32-bit value and reassembles it so the low 16 bits come from the result and the high 8 bits come from the original high byte.
- `interleave_two_longwords_into_d0`: Reads two longwords from the address in a0, masks both with alternating bits, doubles the first and ORs in the second, returning the merged value in d0.
- `compare_two_checksum_results`: Calls the checksum routine at ad4, saves its result on the stack, calls the routine at ab0 with a0 pointing to offset 48 in a5, then compares the popped first result with d0 and returns.
- `setup_checksum_buffer_and_count`: Loads a0 with the address 8 bytes past a5 and sets d1 to 40, preparing the buffer pointer and word count for the following checksum code.
- `xor_words_masked_checksum`: Saves d2, halves d1 and decrements it, XORs consecutive longwords from a0 into d0 that many times, restores d2, masks d0 with alternating bits and returns.
- `interleave_word_bits`: It interleaves the low and high bits of two 32-bit word streams pointed to by two input pointers.
- `set_audio_dma_control`: The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable.
- `poll_flag_and_set_timer`: Polls a byte in a structure for a specific bit flag.
- `set_adkcon_and_test_a6_flag`: Writes the value 1024 to the ADKCON hardware register, tests the word at offset -30 from a6, branches if non-negative, then loads 255 into d1.
- `pulse_ciab_disk_select_line`: Writes d1 to CIAB_PRB, reads the word at offset -36 from a6, adds 3, clears then sets that bit in d1, writing CIAB_PRB between each change.
- `assert_ciab_reset_then_delay`: Loads 255 into d1 and writes it to CIAB_PRB, clears bit 7 of d1, calls the disk select pulse routine, loads 200 into d0 and branches onward.
- `update_table_entry`: Reads a word from a global table using an index, conditionally calls another routine if the value is negative, computes the absolute difference against a register, and writes the result back to the table.
- `wait_for_input_flag_clear`: Saves d2, loops up to eighty-five times testing bit four of the CIAA port A register; while the bit is set it calls the step routine with d0=3 and d1=-1, and when the bit clears it zeroes the track word in the table indexed by the a6 offset and returns zero, otherwise returns thirty.
- `pulse_disk_select_line`: Pushes d0, calls the delay routine, tests d1 and if negative skips the select pulse; otherwise it clears bits zero and one of d0, writes the byte to CIAB port B, sets bit zero again, writes it back, restores d0 and branches to the motor control routine.
- `read_ciab_prb_and_write_disk_select`: Calls the CIAB port B read routine, then stores the returned byte in the disk select register at the CIAB port B address and returns.
- `build_port_byte_from_table`: Builds a port byte from a frame field and a bit index, clearing one further bit when a looked-up table entry has its lowest bit set.
- `poll_ciab_cra_bit0_decrement_counter`: Tests bit 0 of the CIAB control register A, branches if set, otherwise decrements d0 and branches when it reaches zero.
- `set_ciab_timer_a_and_cra`: Writes 8 to CIAB control register A, 204 to timer A high byte and 2 to timer A low byte, then returns.
- `configure_hardware_interrupts`: Sets the CIAA control register, clears the CIAA timer low byte, writes two values to the CIAA interrupt control register, stores a 16-bit value in a global variable, and updates the custom chip interrupt enable and interrupt request registers.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies an indexed entry from a0 to d0, then returns.
- `update_ciaa_status`: Reads a byte from the CIAA status register, performs bitwise operations on it, stores the result in a global variable, sets bit 6 of a byte at offset 3584 of the CIAA register, and writes a byte to offset 1280 of the CIAA register.
- `frag_bclr_move_bra`: Short code fragment that clears bit 7 of d0, then copies 0 to an indexed entry from a0, then jumps to another point in the code.
- `frag_bclr_move_movea_rte`: Short code fragment that clears bit 6 of the memory at an offset from a0, then copies the top of the stack to d0, then copies the top of the stack to a0, then copies 8 to a fixed memory address, then returns from the exception.
- `lookup_conditional_byte`: Checks a 16-bit global variable; if it is non-zero, it compares the value against 64 and 64, and if it falls within a specific range and another global variable is non-zero, it fetches a byte from either a table at offset 4736 or 4832 using the variable as an index.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `frag_move_lsl_swap_rts`: Short code fragment that copies the memory at a0 (advancing a0) to d0, then shifts d0 left by 8, then copies the memory at a0 (advancing a0) to d0, then swaps the halves of d0, then copies the memory at a0 (advancing a0) to d0, then shifts d0 left by 8, then copies the memory at a0 (advancing a0) to d0, then returns.

### logged disk load from Qwak.adf boot-block load

- `copy_block_with_alignment_fixups`: Copies a block of data between buffers, adjusting the source and destination pointers to even addresses before copying runs of longwords.
- `decode_and_decompress_tile_data`: Reads a big-endian word from a3, calls setup and three block routines on a2 offsets, then loops copying bytes between a3 and a5 until a5 reaches a6.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `load_ff_and_16_constants`: Loads the immediate value 255 into d0 and the immediate value 16 into d1, then falls through into the following routine.
- `mask_d0_with_d6_and_conditional_bsr`: ANDs d6 into d0, subtracts d1 from d7, branches past a bsr when d7 is greater or equal, otherwise calls the routine at the next address, then shifts d6 right by d1 and returns.
- `refill_bit_buffer_from_a3_stream`: Adds d1 back into d7, shifts d6 right by d7, swaps d6, advances a3 by four and loads two bytes backwards into d6 with a byte swap, then resets d7 to sixteen minus d1.
- `read_four_bytes_into_d0_from_a0`: Sets d1 to three, shifts d0 left by eight, then copies four bytes from the address in a0 into the low byte of d0 while decrementing d1, and returns.
- `decode_and_fill_table`: The routine reads a count, fills a buffer from another routine, then iterates over a stack buffer and a table, writing bit masks and derived values into table entries based on matching bytes.
- `restore_all_registers_and_return`: Restores the stack pointer, pushes d1, pops all data and address registers from the stack and returns to the caller.
- `clear_bytes_dbra_restore_regs_return`: Clears d0 plus one bytes starting at a6, stores a5 above the return address, restores all registers from the stack, releases the frame and returns.

### logged disk load from Qwak.adf

- `frag_bchg_move`: Short code fragment that flips bit d5 of the memory at a0, then copies d4 to the memory at an offset from a6, then copies d2 to the memory at an offset from a6, then copies a0 to the memory at an offset from a6, then copies d0 to the memory at a1 (advancing a1), then copies a0 to the memory at a1 (advancing a1), then copies d3 to the memory at a1.

### logged disk load from Qwak.adf

- `frag_move_movem_rts_clr_dbra_lea`: Short code fragment that copies d1 to the top of the stack, then restores saved registers from the stack, then returns, then clears the memory at a6 (advancing a6), then loops back while counting down d0, then copies a5 to the memory at an offset from a7, then copies several registers to or from memory, then loads the address of the memory at an offset from a7 into a7, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_a0_with_b0_and_pad_d0`: Loads the immediate value 176 into address register a0, then performs two no-op OR operations of zero on the low byte of d0.
- `decrement_d0_long`: Subtracts one from the longword in data register d0 and falls through to the following code without returning.
- `branch_to_offset_132`: Performs an unconditional word-sized branch to a location 132 bytes ahead, transferring control without touching any registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_irq_state_and_disable_dma`: Pushes all registers, computes two copper list offsets from a base value, stores them, disables audio and all DMA and interrupts, saves INTENA, DMACON and ADKCON into memory, then calls exec.Forbid.
- `setup_level_and_wait_for_fire`: Installs a copper interrupt vector, writes copper list pointers, scales a table of 23 words by four, sets up copper instructions, then polls CIA fire button, POTGOR and VHPOSR while copying sprite position words into the copper list.
- `restore_copper_and_release_library`: Writes the saved copper list pointer from offset 38 of a1 into COP1LCH, calls a library close function, then loads 5 into d0 and calls another library function before returning.
- `reset_custom_chip_and_restore_registers`: Writes all-ones to DMACON, INTENA and INTREQ, re-enables saved DMA and interrupt bits from memory, clears COPJMP1, pops all registers and returns zero in d0.
- `scroll_playfield_tile_column`: Decrements a scroll counter, advances a pointer through a tile table, wraps it at the end, copies four words into playfield registers, calls the tile blitter and a library routine, then returns minus one in d0.
- `find_byte_in_table_record`: Searches a table of up to 73 bytes for the current byte, then uses the match position to compute a record address.
- `setup_copper_and_blitter_for_rotation`: Waits for the blitter to finish, programs blitter pointers and control registers, rotates eleven coordinate pairs using two sine values, calls a helper and a blit routine ten times, then writes color and DMA registers.
- `reset_blitter_masks_and_modulo`: Waits until the blitter is idle, then writes full first and last mask words, a control word and two modulo values for the blitter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_palette_with_ascending_values`: Writes 256 words starting at the palette area, each incremented by forty, filling the color table with a linear ramp and returning.
- `draw_line_with_blitter`: Orders two endpoints, computes the step counts and octant of a line, toggles one pixel bit at the start position, and programs the blitter with the modulo, control and size values to draw the line.
- `frag_move_rts`: Short code fragment that copies d0 to the memory at a1 (advancing a1), then copies a0 to the memory at a1 (advancing a1), then copies d3 to the memory at a1, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_and_input_state`: Sets up audio volume registers and initializes a table of pointers and values.
- `silence_all_audio_channels`: Writes zero to the four AUDxVOL registers, then writes 15 to DMACON to disable the audio DMA channels and returns.
- `increment_tick_and_compare_sound_limit`: Increments the byte at the sound tick counter, loads two PC-relative bytes, compares them and branches if below, then clears the counter and tests a flag before calling a routine.
- `init_four_audio_channels_and_voice`: Sets up the four audio channel register blocks with their parameter tables, then computes a voice sample offset from the current voice index and triggers playback on all four channels.
- `update_audio_channel_from_voice_table`: Reads a voice entry from the sample table, fills the channel structure with period, length and volume, writes the AUD registers and steps the volume envelope.
- `object_state_dispatch_by_type_nibble`: Reads the low nibble of the object type byte at offset 3 of the a6 structure and branches to per-type handling, updating the a5 result word and object fields.
- `object_vertical_motion_and_input_update`: Scans a per-object table for the object's vertical position, sets the target and direction bytes, applies input nibbles to the velocity byte and writes the resulting position to the a5 structure.
- `object_vertical_velocity_integrate`: Computes a signed vertical step from the sine table indexed by the object's angle byte, scaled by the low nibble of offset 26, adds it to the position at offset 16 and advances the angle.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_high_nibble_of_a6_offset_31`: Saves d1 and a0, reads the byte at offset 31 from a6, shifts it right by four bits into d0, and returns zero or 223 depending on whether that nibble is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `frag_cmpi_beq_subq_subi_bra`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then subtracts 1 from a fixed memory address, then subtracts 2560 from a fixed memory address (2 times in a row), then jumps to another point in the code.
