# Game 2078 (addams family, the (demo) (1992)(ocean)(m3)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02078). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911752 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | pass, 251 of 251 named (0 author symbols), 251 of 251 described (8 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3929 of 3951 (99.44%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6737 of 6746 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 122555 |
| code | 25380 |
| data | 763740 |
| padding | 77 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf:bootblock | 1 | 33 | 3 |
| logged disk load from Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf boot-block load | 1 | 6645 | 237 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| logged disk load from Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf | 1 | 37 | 5 |
| logged disk load from Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |

## Cases handled

- addams family, the (demo) (1992)(ocean)(m3)[a]/Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf: custom-format (non-DOS) disk, 1725 non-empty blocks: matched against track-loaded segments
- addams family, the (demo) (1992)(ocean)(m3)[a]/Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf: the boot block reads 156672 bytes from disk offset 17408 to an address (found statically); recovered as a unit
- 18 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- addams family, the (demo) (1992)(ocean)(m3)[a]/Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf: 35 executed PCs whose RAM was overwritten before any snapshot match its bytes 0-240: recovered as a unit run at an address
- addams family, the (demo) (1992)(ocean)(m3)[a]/Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf: 6 executed PCs whose RAM was overwritten before any snapshot match its bytes 17456-17552: recovered as a unit run at an address
- 12 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- addams family, the (demo) (1992)(ocean)(m3)[a]/Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf: 3 logged loads cover 156912 bytes of the image
- addams family, the (demo) (1992)(ocean)(m3)[a]/Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf: blocks 2-33 (bytes 1024-17408) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.09)
- 1 packed track region(s), 727040 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `wait_blitter_done_and_clear_intreq` (Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf boot-block load +an address to an address (156672 bytes)): Loads custom chip base into a1, waits for blitter busy bit to clear via two helper calls, then writes 2 to INTREQ and 16384 to DSKLEN, returning -1 or 0 in d0. Kind: track/disk check (sync/length check). Pass path: the loop continues (bVar2 remains true) and eventually exits via the loop condition, or specifically, the branch where the check does not trigger an immediate return/failure.
- `enable_disk_dma_and_check_signal` (Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf boot-block load +an address to an address (156672 bytes)): Writes 1024 to the ADKCON register to enable disk DMA, tests a word at offset -30 from a6 and sets d1 to 255 if that value is negative. Kind: disk/track check (ADKCON/hardware-dependent check). Pass path: The routine checks a value (likely a disk-related or timing-related flag or checksum) and, if certain conditions are met, modifies a bitmask in memory. A successful check allows the routine to proceed or sets a state that permits the subseq.
- `pulse_ciab_disk_select_line` (Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf boot-block load +an address to an address (156672 bytes)): Writes d1 to CIAB port B, reads a word at offset -36 from a6, adds 3, clears then sets that bit in d1 and rewrites the port each time. Kind: disk hardware/track check (CIAB_PRB). Pass path: the routine modifies a bit in a register/memory (DAT_00bfd100) representing a bitmask or status, where a successful read/sync would result in a specific bit being set or cleared without triggering a failure branch or a jump to an error-hand.
- `assert_ciab_disk_select_and_delay` (Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf boot-block load +an address to an address (156672 bytes)): Loads 255 into d1, writes it to CIAB port B, clears bit 7 of d1, calls a delay routine, then loads 200 into d0 and branches onward. Kind: disk hardware/track check. Pass path: iVar1 == 1.
- `select_drive_and_step_head` (Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf boot-block load +an address to an address (156672 bytes)): Saves d0, calls the CIAB motor/select helper, tests bit 7 of d1, clears bits 0 and 1 of d0, writes d0 to CIAB port B, sets bit 0 and writes again, restores d0 and jumps onward. Kind: disk/track/drive check. Pass path: iVar1 == 1.
- `ciab_motor_select_and_wait` (Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf boot-block load +an address to an address (156672 bytes)): Saves d1-d2, reads CIAB port B and the drive number at offset -36 from a6, builds a select mask, optionally clears bit 2, writes it to CIAB port B, then polls CIAB status until ready or a countdown expires. Kind: disk hardware/track check. Pass path: the branch where the bitmasking/bit-clearing logic (bVar4) proceeds without triggering the conditional bit-clear (the 'if' condition failing or resulting in the default bVar4 state or the non-sabotaged bitmasking logic)..

## Functions


### Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf:bootblock

- `bootblock_init_display_and_io`: Disables DMA, clears all 32 color registers to black, allocates an address bytes of chip memory, fills an IOStdReq with length an address and command 2, calls DoIO twice, then enters supervisor mode and jumps to the boot routine.
- `install_supervisor_trap_vector`: Saves d0 and a0 on the stack, pushes the old level-1 trap vector, points the trap vector at the following routine, and loads d0 with 8 before continuing.
- `trap_return_skip_stack_frame`: Adds 4 to the saved stack pointer at offset 2 of the supervisor stack frame and returns from the exception, discarding the trap's return address.

### logged disk load from Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf boot-block load

- `relocate_boot_block_to_fast_ram`: Copies 63 longwords from the program counter area to address 7ff00, then jumps there with a0 at the source and a1 at 1000 to copy 30d40 bytes.
- `backward_copy_bytes_high_to_low`: If a1 is not below a0, adds d0 to both pointers and copies d0 bytes backwards from a0 to a1, then jumps to 7890.
- `forward_copy_bytes_low_to_high`: Copies d0 bytes forward from a0 to a1 one at a time, decrementing d0 until zero, then jumps to 7890.
- `check_7c86_state_two_values`: Tests the word at absolute address 7c86 and branches away if it is zero or equal to three, otherwise returns to the caller.
- `return_ff_word_constant`: Returns the constant word value 255 in d0 to the caller, with the surrounding instructions forming the routine body.
- `scale_d3_minus_a6_offset_by_512`: Copies d3 into d1, subtracts the word at offset minus sixteen from a6, multiplies the result by 512 and returns it in d1.
- `set_bit_and_increment_counter`: Sets the bit indexed by d3 in the word at offset minus four from a6, then increments the word at offset minus six from a6 and returns.
- `clear_eleven_longwords_spaced_272_bytes`: Starting from a5 plus 1088, writes zero to eleven longwords spaced 272 bytes apart, using d1 as the down counter and d0 as the zero value, then returns.
- `read_four_bytes_from_a5_offset_eight`: Calls a helper with the address eight bytes past a5, then splits the returned long into four byte values in d3, d2, d1 and d0 and returns.
- `interleave_two_words_masked`: Reads two longwords from the address in a0, masks each with the alternating bit pattern, doubles the first and ORs in the second, returning the result in d0.
- `compare_checksum_of_two_buffers`: Calls the setup routine, saves d0, points a0 at a buffer offset from a5, calls the interleave routine, then compares its result with the saved value and returns.
- `set_buffer_pointer_and_count`: Loads a0 with an address offset from a5 and sets d1 to forty, preparing a buffer pointer and count before falling into the following routine.
- `xor_words_masked_parity`: XORs consecutive longwords from the address in a0 into d0 for a count derived from d1 shifted right twice, then masks d0 with alternating bits and returns.
- `interleave_two_buffers_masked`: Reads 128 longwords from a0 and from a0 plus 512, masks each with alternating bits, shifts the first left and ORs in the second, storing results to a1.
- `start_disk_dma_read`: Programs the custom chip registers to enable disk DMA, sets sync word and disk pointer from a0, and starts a read of d0 shifted length.
- `wait_blitter_done_and_clear_intreq`: Loads custom chip base into a1, waits for blitter busy bit to clear via two helper calls, then writes 2 to INTREQ and 16384 to DSKLEN, returning -1 or 0 in d0.
- `enable_disk_dma_and_check_signal`: Writes 1024 to the ADKCON register to enable disk DMA, tests a word at offset -30 from a6 and sets d1 to 255 if that value is negative.
- `pulse_ciab_disk_select_line`: Writes d1 to CIAB port B, reads a word at offset -36 from a6, adds 3, clears then sets that bit in d1 and rewrites the port each time.
- `assert_ciab_disk_select_and_delay`: Loads 255 into d1, writes it to CIAB port B, clears bit 7 of d1, calls a delay routine, then loads 200 into d0 and branches onward.
- `step_vertical_position_toward_target`: Saves d2-d3, reads a signed word from a table indexed by a6 offset, and moves d2 toward it one step at a time, calling a sound routine each step, then stores d3 back.
- `poll_fire_button_with_retry_limit`: Saves d2, tests bit 4 of CIAA port A up to 86 times, calling a delay each pass; if the button is released it clears the table entry and returns zero, else returns 30.
- `select_drive_and_step_head`: Saves d0, calls the CIAB motor/select helper, tests bit 7 of d1, clears bits 0 and 1 of d0, writes d0 to CIAB port B, sets bit 0 and writes again, restores d0 and jumps onward.
- `write_d0_to_ciab_port_b`: Calls the CIAB motor/select helper, then writes the returned d0 byte to the CIAB port B register and returns to its caller.
- `ciab_motor_select_and_wait`: Saves d1-d2, reads CIAB port B and the drive number at offset -36 from a6, builds a select mask, optionally clears bit 2, writes it to CIAB port B, then polls CIAB status until ready or a countdown expires.
- `poll_ciab_flag_decrement_d0`: Tests bit 0 of the CIAB control register at a fixed address and branches if set, otherwise decrements d0 and branches when it reaches zero.
- `init_ciab_timer_and_ports`: Writes 8 to the CIAB control register, 204 to the CIAB timer high byte and 2 to the CIAB timer low byte, then returns.
- `run_loader_chain_and_jump_entry`: Saves registers, stores a0/a1/a2 pointers, calls three loader routines checking d1 sign, verifies a magic longword at a0, then jumps to the loaded entry and restores registers.
- `check_and_store_dire_checksum`: Reads a6 from the global pointer, compares the longword at offset minus eight with the marker value, and if it matches and d7 equals the stored value returns; otherwise calls the disk routine, and on success stores d7 and the marker, returning d1 zero or minus one.
- `call_two_disk_routines_with_params`: Loads d1 with twenty-two and d2 with twelve, calls the routine at the first vector with a0 from the global pointer, then repeats the same parameters and calls the second vector with a0 set to the global pointer address, then returns.
- `sum_1536_longwords_into_d7`: Copies a6 into a5, clears d7, sets d6 to fifteen hundred thirty-five, then adds each longword pointed to by a5 into d7 while decrementing d6 until it wraps, and returns.
- `verify_encrypted_password_table`: Scans a table of 299 entries against an input string, comparing each byte after a nibble swap and optional case flip, then returns two longwords read past the match XORed with a constant.
- `read_and_verify_disk_track`: Reads a 512-byte sector from disk into a buffer, copies it to a track buffer, computes a rolling checksum over the data and compares it with the stored checksum, returning the remaining length or minus one.
- `poll_disk_until_ready_with_color_flash`: Disables DMA, repeatedly calls the disk routine while incrementing the COLOR00 register to flash the background, and returns zero after setting a delay counter when the disk reports ready.
- `disable_dma_and_call_init`: Clears the longword at 18436, sets d0 to zero and d3 to 32768, loads a1 from a fixed address, writes 33296 to DMACON to enable DMA, then calls the routine at 15726 and returns.
- `probe_and_claim_memory_block`: Saves all registers, checks two flags, then walks a pointer list at 18392 writing and verifying test patterns in each block until one passes, storing it at 18388 and marking it with a fixed address.
- `find_and_verify_named_block`: Saves registers, walks the block list from 18388 comparing the caller's name string, then copies the block's words to a1 while computing a rotating checksum and returns the block length in d1 or minus one.
- `queue_sound_sample_with_checksum`: Saves registers, calls three helpers, then if sound flags allow and space remains, copies a name string and word data into a sound queue node, computes a rotating xor checksum, writes terminator and links the node.
- `chain_four_subsystem_calls`: Calls four fixed subroutines in sequence via jsr and a tail jmp, touching no registers or memory itself beyond the stack for return addresses.
- `compare_d0_to_constant_and_return`: Compares d0 against a fixed immediate value, branches away if not equal, otherwise returns immediately; the branch target lies outside this routine.
- `set_ciaa_serial_and_clear_fire`: Writes 8 to a6 offset 154, stores 24800 at absolute address 96, then clears CIAA port A and sets port B to 136 before writing 49160 to a6 offset 154.
- `ciaa_fire_button_interrupt_service`: Disables and acknowledges the CIAA interrupt, reads port A fire bits, and on a fire press toggles port B, pulses port A lines, calls the key-state routine, then re-enables interrupts and returns from exception.
- `update_key_state_tables`: Saves all registers, uses d0 bit 7 to set or clear key-pressed flags in two tables indexed by the key code, updates the current key bytes, and returns.
- `set_three_volume_words`: Saves all registers, loads base address into a0, writes the word in d0 to three offsets within that block, restores registers and returns.
- `set_single_volume_word`: Saves all registers, loads base address into a0, writes the word in d0 to one offset in that block, restores registers and returns.
- `adjust_volume_with_clamp_and_callback`: Saves registers, increments or decrements a word counter based on two flag bytes, clamps it between zero and 256, passes it in d0 to a called routine, then restores and returns.
- `add_offset_to_nonzero_long`: Tests the longword at a0; if nonzero, adds d1 to it, then advances a0 by four bytes and returns.
- `relocate_structure_pointer_lists`: Saves registers, applies the offset fixup to several pointer fields of the structure at a0, relocates a linked list of entries, then processes two more pointer fields and restores registers.
- `disable_dma_channel_by_index`: Uses d0 as an index into a table near pc, clears a field of the selected entry and writes a single-bit mask to DMACON to disable that DMA channel.
- `clear_sound_channels_and_zero_pointer`: Saves all registers, calls the sound routine sixteen times with d2 counting down, then clears the longword at the pc-relative address and restores registers.
- `start_sound_sample_playback`: Saves registers, selects a sample from the module table, sets DMACON bit for the channel, computes period and volume, stores playback state in the channel structure and starts the sound routine.
- `advance_sound_effect_script`: Steps the per-channel effect script counters, applies volume and period deltas, reads script opcodes from the sample data and updates the channel period and volume fields.
- `copy_indexed_byte_to_a5_offset_30`: Copies the byte at offset 22 from a0 indexed by d0 into offset 48 of a5, increments d0, then branches to another routine.
- `copy_indexed_byte_to_a5_offset_36`: Copies the byte at offset 22 from a0 indexed by d0 into offset 54 of a5, then branches to another routine.
- `copy_negated_indexed_byte_to_a5_offset_36`: Loads the byte at offset 22 from a0 indexed by d0 into d1, negates it, stores it at offset 54 of a5, then branches to another routine.
- `copy_anim_byte_and_clear_flag`: Copies a byte from offset 22 of a0 indexed by d0 into offset 47 of a5, clears the byte at offset 49 of a5, then branches to another routine.
- `advance_script_pointer_and_apply_velocity`: Decrements d0, stores it at offset 44 of a5, copies a byte to offset 2, advances a0 by 158, and steps a script pointer through a0 while updating a3 display offsets and velocity from a5 fields.
- `step_frame_script_and_apply_bob_offset`: Decrements d0 into offset 46 of a5, reads a frame index from a0 into a1 table to set offsets 24 and 6 of a3, then adds a sine-scaled bob offset and pops a3.
- `wait_two_vblank_toggles`: Saves d0 on the stack, then polls the VHPOSR register until its low byte changes or a single-count dbra expires, restoring d0 and returning.
- `apply_next_display_entry`: Loads a pointer from (a1)+ into a5, shifts d0 right by one bit, and if the shifted-out bit was set and the entry has a nonzero length, copies that length and a word into the entry's buffer.
- `enable_dma_and_flush_three_entries`: Reads a word from PC-relative memory, enables DMA by setting bit 15 and writing DMACON, waits for a vblank toggle, then applies three display entries and branches to a fourth.
- `animate_entity_sprites_and_dma`: Steps animation counters for up to four entity slots, updates sprite frame pointers and channel DMA bits in DMACON, and jumps through a per-slot dispatch table.
- `misdecoded_data_table_after_jump_table`: Bytes decoded as instructions that write meaningless immediates to odd addresses; this is data, not executable code, ending in a branch to the animation loop.
- `select_animation_frame_from_direction`: Uses d4 direction and d3 frame counter to pick a word from the table at offset 66 of the entity, returning it in d5 before branching to the shared exit.
- `clamp_volume_byte_and_branch`: Masks d4 into d1, subtracts d4 from the volume byte at offset 2 of a5, clamps it to zero if negative, stores it back and branches to the shared exit.
- `add_high_nibble_to_volume_cap_64`: Shifts d1 right by four bits, adds it to d0, caps the result at 64, stores it at offset 2 of a5 and branches to the shared exit.
- `apply_volume_and_advance_sound_channel`: Writes d5 and the scaled volume byte into the channel structure at 20(a5), increments d7, compares it with the channel limit and either loops back or restores registers and returns.
- `zero_filled_data_area`: Two zero words that decode as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `load_sprite_frame_pointer_and_advance`: Stores a table address into a5 offset 66, reads a word indexed by d1 from that table into d5, loads a long from a0 into d1, advances a0 by six, reads two words from a3 into d2 and d3, then returns.
- `store_sprite_descriptor_by_index`: Masks d0 to two bits and scales it by eight to index a table via a5, stores a0 there, packs bits of d2 and d1 into byte four, d3 and d2 into byte five, and d4 into byte six, then returns.
- `decode_and_dispatch_level_command`: Reads a command byte pair from the level script, decodes flags and opcode, updates the actor state at a5, and branches to the matching command action.
- `copy_sprite_rows_to_buffer`: Copies rows of sprite data from the structure pointed to by a0 into a fixed buffer, or clears the buffer when the source is missing, then restores registers and returns.
- `init_level_and_report_success`: Sets up the level with the given mode, calls the level loader and setup routines, and returns zero on success or minus one after the fallback path.
- `conditional_run_two_setup_routines`: Reads a flag byte from pc-relative memory; if nonzero, calls two subroutines in sequence, then returns to the caller.
- `store_object_and_configure_fire_button`: Saves a0 into four globals, calls a setup routine, then stores a byte from the object into memory and sets or clears the CIAA fire-button bit based on a flag.
- `conditional_clear_word_and_call`: Reads a pc-relative flag byte; if nonzero, clears a word in memory and calls a subroutine, then returns.
- `clear_global_70dc_and_return_zero`: Saves d2-d3/a4/a6, loads a6 from address 4, clears the word at memory 28892, sets d0 to zero, restores registers and returns.
- `copy_d2_to_d0_and_branch_back`: Moves the longword in d2 into d0 and branches backward into the preceding routine, continuing its flow.
- `read_and_clear_flag_byte_at_6452`: Saves a6, loads the byte at the pc-relative location 6452 into d0, clears that byte if nonzero, restores a6 and returns.
- `cold_boot_init_and_main_loop`: Sets supervisor mode and stack, fills low memory and chip RAM with fixed values, initializes hardware and game state, then runs the main loop calling many game routines.
- `wait_for_space_key_press`: Sets two flags, calls a routine with a pointer, then repeatedly polls the key byte until a space is seen, restores a word and clears both flags.
- `set_level_state_four_ten_zero_two`: Writes four to the level word, ten to the next word, clears a third word and stores two in the state word, then returns.
- `init_level_and_run_tile_animation`: Sets up level pointers and counters, copies a block of data, reads five bytes from the current animation record into the position and timer variables, then draws the tile and advances the record until the terminator.
- `draw_tiles_with_bit13_set`: Walks a 14 by 16 tile map, and for each cell whose high byte has bit 13 set, decodes the tile index and draws it into the screen buffer via the tile blitter.
- `draw_tiles_with_bit13_clear`: Walks a 14 by 16 tile map, and for each cell whose high byte has bit 13 clear, decodes the tile index and draws it into the screen buffer via the tile blitter.
- `build_bitplane_tile_rows`: Tests bit 6 of the byte at a0; if clear, programs custom chip registers at offsets 100, 102, 64 and 68 of a6, then for sixteen rows reads words from d6, reverses their bits into d1 and stores them at offsets 0, 38, 76 and 114 of a1, clearing offset 152 and advancing a1 by 190.
- `blit_rectangle_of_tiles`: Reads width and height bytes from a0, saves a1 and d4-d6 on the stack, then repeatedly calls the routine at 131f6 to draw one tile, advancing a1 by one and stepping down 1520 bytes per row until the height count reaches zero.
- `run_attract_mode_sequence`: Calls many library routines to clear counters, load graphics, wait on a timer, set display registers, then draws a tile map at computed screen coordinates and loops back to the attract sequence until a key or joystick input ends it.
- `copy_3ba8_block_to_3b2e`: Copies four words from the long addresses around 3ba8 into the low-memory words at 3b2e, 3b30, 3b2a and 3b2c, then copies a longword from 12908 into 3b24 and returns.
- `reset_display_and_clear_3ba8`: Calls several graphics and sound routines, writes all-ones to 3c04, clears the longwords at 3ba8 and 3ba4, calls the same routine twice, then calls another routine and returns.
- `call_11cea_then_9dae`: Calls the routine at 11cea, then calls the routine at 9dae, and returns to its caller without touching any registers itself.
- `setup_display_and_wait_forever`: Calls three subroutines, writes the value 496 to two adjacent words at 15146, clears that longword, sets byte 15138 and loops forever while it stays nonzero.
- `clear_chip_ram_68000_to_80000`: Calls a subroutine, then zeroes longwords from address a fixed address up to but not including a fixed address using a0 as the pointer, and returns.
- `fade_palette_colors_down_and_copy`: Repeatedly decrements each of the 32 color words at a fixed address by one per nonzero nibble, calls a routine, copies 16 longwords to COLOR00, and repeats 16 times before a final call.
- `clear_state_and_copy_palette_buffer`: Clears several long and word state variables, sets counters and pointers, zeroes 1200 bytes at the sprite buffer, then copies 256 bytes from one palette buffer to another.
- `init_display_and_build_sprite_pointers`: Sets a flag and two pointers, calls four system routines, copies 256 palette bytes, then fills 256 longwords with increasing sprite frame addresses.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `zero_registers_and_jump_to_init`: Sets d0, d1 and d2 to zero and d3 to 32768, then jumps to the game initialization entry point.
- `fade_palette_toward_target`: Saves all registers, clears 16 longs of a work buffer, then repeatedly calls a routine and steps each of 32 words one nibble closer to a target table before copying 16 longs to COLOR00.
- `toggle_flag_and_copy_256_bytes`: Tests a byte flag, copies it and toggles bit 1, calls a routine, then copies 256 bytes from one buffer to another and returns.
- `clear_game_work_buffers`: Clears several memory regions: 304 bytes, 3840 bytes, 32 words spaced 16 bytes apart, 21 longs and 16 longs, then returns.
- `init_level_and_call_setup_routines`: Copies four word globals into nearby state words, stores a longword, then calls five subroutines and clears a counter before returning.
- `scan_entity_table_for_collisions`: Iterates a table of entity records, computes screen offsets, updates collision counters and calls one of three handlers based on the entity type byte.
- `dispatch_tile_command_byte`: Clears a word at a1, reads a byte from a tile map indexed by a1 fields, clears it, and branches to handlers for command bytes.
- `append_three_bytes_to_ring_buffer`: Loads the write pointer, builds a two-byte value from two globals, stores three bytes at the pointer, advances it by three, and wraps it back to the buffer start when it passes the end.
- `advance_bcd_counter_and_spawn_entity`: Calls the byte writer, plays a sound, increments two BCD counters, and on wrap or a count of thirty-seven triggers sound and setup calls before initialising an entity block at a1.
- `increment_bcd_byte_and_play_sound`: Clears d0, loads a BCD byte counter, skips if it equals one hundred fifty-three, otherwise adds one with abcd, stores it, calls the sound routine and branches back.
- `call_e0b8_then_jump_9308`: Calls the subroutine at absolute address e0b8, then branches unconditionally to the code at 9308.
- `poll_3d00_and_increment_counter`: Calls a setup routine, invokes the routine at 3d00 with d1 and d2 set, then reads the word at 3bc6, increments it in packed BCD unless it equals 153, stores it back, calls the routine at ff1e and jumps to 9308.
- `check_3cbe_and_decrement_3cbc`: Compares the word at 3cbe with 3, and if not equal increments that word and decrements the word at 3cbc before branching to 9308.
- `set_word_ffff_at_3b8c_then_branch`: Writes the immediate value 65535 to the longword-addressed word at 3b8c, then branches unconditionally to the shared code at 9308.
- `save_regs_call_3d00_set_two_words`: Pushes all data and address registers, loads d1 with 12 and d2 with 18, calls the routine at 3d00, restores the registers, writes 3 to 3bf2 and 40 to 3bce, then branches to 9308.
- `branch_to_shared_code_9308`: Contains only an unconditional branch to the shared code at 9308, transferring control without touching any registers or memory.
- `branch_to_9308`: Branches unconditionally to the code at address 9308, transferring execution there without touching any registers or memory.
- `per_frame_game_update_dispatch`: Calls the frame setup routine, checks several state words, decrements a counter, updates the two nibble counters at 3ba4 and 3ba6, then calls the sound, sprite and display routines before returning.
- `clear_sprite_slot_table`: Calls the sprite reset routine, then walks twenty entries at a86e, and for each active entry with valid fields calls the slot handler and clears its four bytes.
- `spawn_entity_from_spawn_table`: Looks up a spawn record by index in d0, checks the spawn coordinates in a0 against screen limits, then finds or fills a 60-byte entity slot with position, type, pointer and zeroed counters.
- `update_all_entity_slots`: Iterates 64 entity slots at a0, advancing positions by velocity, decrementing timers, fetching script words from a1 and jumping through the opcode table when a negative command is read.
- `main_game_state_dispatch`: Returns early if a flag is set, clears a word, reads low nibbles of two counters into d6 and d7, calls several subroutines, then branches on the game state word in d0 to different handlers.
- `spawn_entity_at_camera_offset`: Checks the game mode word, throttles a counter, calls the allocator, then fills the new entity block with type 1, a sprite pointer, position from the camera offsets plus 16 and 26, and clears its velocity fields.
- `spawn_scrolling_entity_with_wrap`: Counts down a spawn timer, calls the allocator, fills the entity block with type 1 and a sprite pointer, advances a horizontal offset by 20 wrapping at 360, and stores the offset and height 24.
- `spawn_entity_every_32_frames`: Returns unless a flag is set, increments a frame counter masked to 31, and on wrap calls the allocator to create an entity at the camera position with type 1 and a sprite pointer, shifting it down when the mode is 3.
- `scan_60_byte_records_for_nonzero`: Loads the address of a record table into a1, sets d7 to 63, tests the word at a1 and branches away if nonzero, otherwise clears d0 and returns.
- `scan_backwards_records_then_check_state`: Steps a1 back by 60 bytes per d7 count until a nonzero word is found, returning minus one if none; otherwise checks game state words and jumps through a vector.
- `count_matching_tiles_in_grid`: Computes a grid offset from level coordinates, scans two rows of three bytes, counts bytes within the min/max range, and sets a flag word when four or more match.
- `handle_input_key_and_start_level`: Calls a setup routine, sets display window size, then reads the current input character and updates game state flags, starting a level or jumping to a handler based on the key.
- `advance_animation_frame_on_timer`: Reads the animation frame index, looks up a word from a table, adds it to a running total, and advances the frame index or sets a flag when the total reaches the threshold.
- `update_cached_pointer_if_changed`: Compares d0 with the value at a global long, and if different stores d0 into two global longs, otherwise returns without changing memory.
- `update_platformer_vertical_motion`: Reads input flags and a per-frame table at a fixed address to adjust the player's vertical position and velocity words, calling sound and collision helpers and returning early on certain states.
- `check_adjacent_tile_for_special_object`: Looks up tilemap entries at offsets based on the current level word, checks tile type and object field, and returns a flag or jumps to a collision handler.
- `play_sound_effect_and_return_minus_one`: Saves all registers, calls the sound routine with effect numbers six and twenty-three, restores registers, sets d0 to minus one and returns.
- `poll_input_and_toggle_light_state`: Clears two state words, polls input ports via library calls, checks returned bytes and branches to toggle a light bit, clear bytes or set bytes at a1 and a1+4095, returning a status word in d0.
- `swap_two_40_longword_blocks`: Saves a0 and a1, then swaps 40 longwords between the block at 3ee16 and the block at 3eeb6 using d0 as temporary, restoring a0 and a1 before returning.
- `compute_tilemap_offset_from_d0_d1_d6_d7`: Stores d6 and d7, computes two tile coordinates from d0, d1, d6, d7 with shifts and table bases, stores intermediate words, and returns a0 pointing into the tilemap at 3b4e.
- `player_input_state_machine`: Reads controller bytes through a0, updates global state words at fixed addresses, calls input helpers, and returns a status word in d0.
- `compute_tilemap_cell_pointer`: Multiplies row and column words, adds the base of the tile map, and returns the cell address in a1; a second entry copies row and column into state words and advances the row counter.
- `decode_level_select_digit`: Reads a character through a0, and if it is a digit between 36 and 39 stores the low nibble as a level index and the high nibble as a previous value, then sets two state words.
- `read_difficulty_from_input_byte`: Reads a byte from the pointer returned by the input routine, and if it is an asterisk or plus sign stores a difficulty index and two nibbles from a table into globals, then sets two state words.
- `update_game_state_flags_and_branch`: Clears three state words, tests game flags and input, then branches to subroutines that set mode words and load pointers based on the button bits and a byte at offset twenty of the input buffer.
- `start_level_if_not_active`: If the active flag is clear, sets mode and counter words, clears several state words, loads a pointer, saves all registers, calls the level setup routine with two parameters, restores registers and returns.
- `init_sound_channel_and_set_mode`: Clears a word at 15450, copies two bytes into a sound chip register block at 52226, bumps the first byte, writes 1 to the third, calls the audio routine at 43296 with d2 set to a large count, then sets mode word 15228 to 2 and conditionally adjusts counters based on bit 4 of 15525.
- `reset_mode_counters_to_default`: Clears the word at 15296, writes 12 to 15284 and clears 15450, then returns; used as the alternate branch when bit 4 of the status word is clear.
- `blit_tile_row_from_map_to_bitmap`: Computes tilemap source offset from d6/d7 and scroll words, stores column and row indices, then copies four bytes per iteration from the map at a fixed address through a palette table at 56908 into the bitmap buffer at 15182, advancing by the row stride in d5.
- `decrement_sound_timer_and_fetch_next_byte`: Decrements the word counter at 15536; when it goes negative it reloads from 15538, reads a byte from the pointer at 15540 (resetting to 15544 if the byte is negative), stores the byte in 15286 and returns.
- `advance_sound_sequence_and_trigger_effect`: Copies the sequence counter at 15308 to 15306, and if it is not five increments the counter, calls the sound routine at 65186 and returns; the remaining code handles effect state, sprite registers and a countdown.
- `step_sprite_frame_animation`: Checks the animation flags at 15236, 15360 and 15358, then decrements the frame delay byte in the table at 58088 indexed by the direction word at 15288, advancing the frame and calling the sprite update at 60304.
- `update_player_state_and_input_actions`: Reads the state word at offset 3b7c and, unless it is 15, clears 3b9a and calls movement, jump, attack and sound routines based on the sign and magnitude of the vertical velocity and the joystick bits at 3ca4/3ca5.
- `check_3ba6_low_nibble_and_high_bits`: Loads word at 15270, shifts right four bits, compares with word at 15496, then reloads and masks low nibble, returning via rts if both checks pass.
- `apply_vertical_velocity_and_trigger_sound`: Subtracts the words at 3bba and 3b9e from the word at 3bae, stores the result back, sums 3bba and 3b9e into d1, and if that sum is negative calls the sound routines at e8fe and e39a with all registers saved.
- `update_animation_frame_and_direction`: Checks game state flags, decrements the frame counter, reads a signed or unsigned byte from the animation table at a0 indexed by the counter, stores it as direction, and calls the animation update routines.
- `advance_animation_when_state_zero`: If the state word is zero it calls the sound routine, then depending on the sign of the direction word calls the frame advance or reverse routine and branches to the common exit.
- `step_animation_by_direction_flags`: Tests the frame counter and direction flags, calling the sound, forward and reverse animation routines conditionally, then returns to the caller.
- `swap_two_longword_globals`: Loads the longword at one global address into d0, copies a second global into the first, then stores d0 into the second, swapping the two values before returning.
- `load_level_graphics_and_palette`: Selects a level table entry, copies graphics data to two buffers, stores header bytes into globals, copies a byte block and a palette of eight longwords, then returns.
- `dispatch_menu_selection_text`: Branches on a menu state word, loads text pointers and calls a text routine, appends characters from a3 into a buffer until a null byte, then returns.
- `copy_tiles_and_shift_tilemap_back`: Calls two library routines, expands 256 blocks of four words into a 16x16 tile layout at the destination, then copies 256 blocks of 32 longwords backwards to a lower address.
- `expand_tiles_and_or_planes`: Expands 256 blocks of four words into a 16x16 tile layout and stores the bitwise OR of the four plane words into a fifth word per tile.
- `blit_tilemap_from_byte_indices`: Sets two hardware registers, calls a blit routine twice, then reads 14 rows of 16 byte tile indices and copies each referenced 16x16 tile block into the destination bitmap.
- `expand_4bitplane_tiles_to_words`: For each of sixteen tile rows, reads four bitplane words from a table indexed by d0 and interleaves their bits into four output words written to a0, advancing a0 by 272 bytes per row.
- `copy_eight_blocks_to_two_buffers`: Copies eight blocks of fourteen longs from a table selected by the global counter into two alternating destination buffers, clearing the final long of each block.
- `decode_joystick_bits_to_flags`: Reads the word at offset 12 of a6, derives five flag bits from its joystick direction bits and the CIAA fire button, and stores the result to a global word.
- `draw_two_sprite_frames_wait_blitter`: Computes two sprite frame pointers from the global frame index, programs the blitter registers at offsets 64 to 116 of a6, calls the blit routine, and waits on the blitter-done bit after each frame.
- `set_palette_and_draw_three_frames`: Builds a palette value from the global index into a6 offset 64, calls the palette and blit routines, then draws two sprite frames from the global index and waits on the blitter-done bit.
- `set_palette_and_draw_four_frames`: Builds a palette value from the global index into a6 offset 64, calls the palette and blit routines, then advances the source pointers and blits four frames before waiting on the blitter-done bit.
- `compute_level_row_offset`: Reads the word at 15148, derives a row index by halving it, adding 8 and subtracting its low nibble, wraps it into 0-255, then scales it by 272 into a1 and returns two shifted column offsets in d1 and d2.
- `queue_two_display_requests`: Stores a1, a0 and d1 into a6 offsets 80, 84 and 88, waits while bit 6 of a6+2 is set, then if d2 is nonzero stores a2 and d2 there and waits again.
- `copy_six_byte_level_record`: Multiplies the word at 15158 by 6, indexes the table at a fixed address, and copies six bytes from that record into the bytes at a fixed address, a fixed address, a fixed address, a fixed address, 15529 and 15531.
- `build_sprite_pointer_lists_from_map`: Reads a tile map byte stream and appends sprite pointer and offset pairs into five separate lists, then writes end markers and returns.
- `build_all_five_sprite_lists`: Calls five list-building routines for each of the five sprite pointer arrays, then calls a final routine and branches onward.
- `load_sprite_blocks_into_copper_list`: Writes copper list entries for each sprite block, waiting on the blitter busy bit and advancing through four blocks per entry until a negative terminator.
- `blit_four_tile_columns_from_list`: Writes blitter control words to a6 offsets and, for each entry in the a0 list, programs four blits at 38-byte column offsets until a negative terminator is read.
- `blit_five_tile_columns_from_list`: Like the four-column blitter setup but programs five blits per a0 list entry, using d4 as an extra source and stopping when the list word is negative.
- `branch_to_next_routine`: A single unconditional branch instruction that jumps forward to the following routine, transferring control without touching any registers or memory.
- `init_display_and_run_three_subs`: Loads three fixed addresses into a0, a1 and a2, calls three subroutines, then loads an address into a0 and calls two more subroutines before returning.
- `blit_five_sprites_forward`: Sets up custom chip registers via a6, reads sprite pointers from a0, waits on bit 6 of the control register and writes blitter source, destination and size registers for five sprites.
- `blit_five_sprites_offset_backward`: Configures custom chip registers through a6, reads sprite pointers from a0, adds an offset to each source, waits on bit 6 of the control register and writes blitter registers for five sprites.
- `draw_foreground_sprites_by_priority`: Loops over 32 sprite slots at a fixed table, and for each active slot with priority below three, sets a global flag from its flags word and calls one of two draw routines depending on its type word.
- `update_player_sprite_and_position`: Computes a tilemap offset from the player's pixel coordinates, handles a respawn counter, then draws the player sprite at its screen position and toggles a global flip flag.
- `vertical_blank_interrupt_service`: Saves all registers, acknowledges the vertical blank interrupt, updates counters, calls the game update routines, reads a key and stores it, then clears the interrupt request and returns from exception.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `install_trap_handler_and_set_count`: Saves d0 and a0 on the stack, pushes the current level 1 autovector, stores the address of code just after this block into the level 1 vector, and loads 8 into d0.
- `clear_d0_return_value`: Clears the entire d0 register to zero, leaving all other registers and memory untouched.
- `restore_trap_vector_and_registers`: Pops the saved level 1 autovector value back into the vector location, restores d0 and a0 from the stack, and returns to the caller.
- `increment_return_address_and_rte`: Adds four to the saved return address at offset two on the stack, then returns from the exception with rte.
- `configure_vector_table_and_copy_entries`: Selects interrupt and trap vector addresses based on status words, stores them into the vector table at a5, sets a6 offset 260, then copies eight pairs of words into the table at a0.
- `wait_for_high_byte_threshold`: Repeatedly reads the word at offset six from a6 and loops until its high byte reaches the threshold value, then returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `compare_11a6e_word_to_itself`: Loads the word at absolute address 11a6e into d0, compares it with the same word, and returns; the branch is never taken so it always returns.
- `install_seven_interrupt_vectors_then_init`: Writes seven longword vector addresses into low memory at 100 through 124, loads the custom chip base into a6 and calls the init routine before returning.
- `initialize_custom_chip_display_and_audio`: Clears sixteen longwords at offset 384 of a6, programs custom chip registers for bitplanes, colors, sprites and audio, then calls three setup routines and returns.
- `copy_eight_longs_to_sprite_buffer`: Copies eight longwords from the source at a fixed address into the destination at a fixed address using a0 and a1 with dbra count seven, then returns.
- `clear_sprite_registers_and_init_pointers`: Calls a setup routine, writes 32 to offset 150 of a6, clears sixteen longwords at the SPR0POS hardware area, stores the same pointer into eight a6 offsets, then returns.
- `call_setup_and_set_display_mode_word`: Calls the setup routine at a fixed address, writes the value 33024 to offset 150 of a6, and returns to its caller.
- `set_sound_priority_and_call`: Calls the routine at a fixed offset, then writes the immediate value 256 to offset 150 in the a6-relative structure before returning.
- `update_sprite_frame_and_coordinates`: Reads a frame index, advances it, computes a source offset from screen and scroll values, then writes the sprite control word and successive coordinate pairs to the custom chip registers.
- `clear_bitmap_memory_block`: Points a0 at the bitmap area, sets d7 to 24575, then clears longwords sequentially through that region using a decrementing loop before returning.
- `build_two_plane_tile_entries`: Calls the tile lookup, computes two bitplane words from d0/d1/d3, writes them to two tile slots in a0 and stores their pointers in the a1 list, then repeats for a second row.
- `build_single_plane_tile_pair`: Calls the tile lookup, computes one bitplane word pair from d0/d1/d3, writes it to two consecutive tile slots in a0 and records both pointers in the a1 list.
- `bit_reverse_256_byte_table`: If the flag byte is clear, fills 256 bytes at the table with bit-reversed values of the index using rotate-through-extend on d1 and d2, then returns.
- `build_tilemap_and_three_passes`: Saves registers, calls two setup routines, marks byte 11 of the structure, computes a destination pointer from tile counts and stores it at offset 12, then calls three subroutines and returns the pointer difference in d1.
- `build_tilemap_single_pass`: Saves registers, calls two setup routines, marks byte 11 of the structure, computes a destination pointer from tile counts and stores it at offset 12, calls one subroutine and returns the pointer difference in d1.
- `or_merge_tile_bitplanes`: Loops over rows and columns, ORs four bitplane words from a1 into one word written to a2, advancing a1 and a2, then returns after restoring all registers.
- `blit_tilemap_rows_to_bitmap`: Saves all registers, computes source and destination pointers from the a6 structure, then expands each tile row through the byte lookup table at a4 into the destination buffer, repeating for all rows and tiles.
- `blit_tilemap_rows_to_second_buffer`: Saves all registers, derives a destination pointer from the a6 structure's longword field, then expands tile rows through the a4 lookup table into that buffer, looping over all tiles and rows.
- `lookup_tile_offset_and_apply_delta`: Uses d2 as a tile index into the a3 tile table, adds the tile's offset to a0, loads width and height into d2 and d3, and conditionally adds signed deltas from the table to d0 and d1.
- `dispatch_sprite_if_flag_then_redraw`: Tests byte at offset 10 of a0; if set, saves registers, calls two library routines with a2 set to 68000 and d1 advanced by 16, then calls the redraw routine and returns.
- `blit_tilemap_blocks_to_bitmap`: Saves all registers, walks a table of tile descriptors at a0, copies 4x4 word blocks through a scratch buffer into the destination bitmap, then restores registers and returns.
- `setup_copper_split_for_scroll_strip`: Checks d0 and d1 against screen limits, computes source and destination pointers from a0 and d2, programs the copper list registers for a horizontal scroll strip, waits on the beam and returns.
- `pop_all_registers_and_return`: Pops d0 through d7 and a0 through a6 from the stack, then returns to the caller; it restores the full register set and does nothing else.
- `blit_masked_sprite_with_clipping`: Clips a sprite against screen bounds, sets Blitter window and modulo registers, waits for the blitter, and programs pointers, size and control registers to draw the masked sprite.
- `draw_vertical_sprite_strip_columns`: Calls the blitter setup routine repeatedly to draw columns of a sprite, adjusting a1 by one or 1520 bytes and toggling bits in the control word at the global address.
- `build_masked_tile_bitplanes`: Reads eight words from a tile table indexed by d0, splits each into two bytes, complements their OR into a mask buffer, rotates masks together, then combines tile bytes into five bitplane rows at a2 and advances a2 by 190 per tile.
- `dispatch_on_low_nibble_table`: Reads the low nibble of the word at the global state address, shifts it left twice, loads a code pointer from a jump table indexed by that value into a5 and jumps to it.
- `return_zero_constant_stub`: Returns the constant zero in d0 without touching other registers or memory, then returns to the caller.
- `zero_d0_copy_d4_to_d1_zero_d2_d3`: Clears d0, copies the low byte of d4 into d1, clears d2 and d3, then returns to the caller.
- `copy_d4_to_d0_and_d1_zero_d2_d3`: Copies the low byte of d4 into both d0 and d1, clears d2 and d3, then returns to the caller.
- `zero_d0_copy_d4_to_d1_d2_d3`: Clears d0, copies the low byte of d4 into d1, d2 and d3, then returns to the caller.
- `copy_d4_to_d0_through_d3`: Copies the byte in d4 into d0, d1, d2 and d3, then returns to the caller.
- `save_regs_call_26e96_and_two_helpers`: Saves d0 and d2-d7 and a0-a6 on the stack, calls the routine at 26e96, pushes the longword at a0, calls two helpers, pops it into d1 and restores registers.
- `init_state_once_and_snapshot_registers`: Saves registers, checks the init flag, calls the setup helper if unset, stores a0 and a1 values, calls another helper, copies all registers to the state block and sets the flag.
- `toggle_global_flag_and_call_two_routines`: Saves all registers and status register, masks interrupts, tests a global byte flag, clears it if set, then restores registers and calls two subroutines before returning.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `fill_palette_entries_and_init_playfield`: Builds 256 four-byte colour entries at a2 with value bytes, then sets up d6, d7 and function pointers and calls two routines to initialise the display.
- `scanline_colour_register_update_loop`: Repeatedly calls the a6 routine, compares d5 with d7, writes d2 and d4 words into the a2 table at d7 times four, calls a4, and loops until d0 reaches zero.
- `swap_d7_advance_a5_check_254`: Swaps the halves of d7, increments a5 by one, compares the low word of a5 with 254 and branches away if unequal; otherwise loads a6 from a nearby address and sets d1 to 10 before branching.
- `swap_d7_advance_a5_check_766`: Swaps the halves of d7, increments a5 by one, compares the low word of a5 with 766 and branches away if unequal; otherwise loads a6 from a nearby address and sets d1 to 11 before branching.
- `swap_d7_advance_a5_check_1790`: Swaps the halves of d7, increments a5 by one, compares the low word of a5 with 1790 and branches away if unequal; otherwise loads a6 from a nearby address and sets d1 to 12 before branching.
- `advance_a5_and_setup_second_entry`: Swaps d7, increments a5, compares a5 with the immediate value 3838 and, when not equal, loads a6 from the nearby address and sets d1 to 13 before branching.
- `extract_shifted_bits_from_a0`: Swaps d7, increments a5, reads a long from (a0), shifts it right by d6, computes a 16-bit shift from d1, adjusts d6 and a0, then swaps d7 and returns.
- `dispatch_byte_via_a4_or_store`: Doubles d3 twice, indexes a2 to load a word pair into d2 and d3, then either calls a4 and stores d2 to (a1) or stores d2 directly to (a1).
- `copy_run_length_encoded_bytes`: Swaps d7, doubles d3 twice to form a byte offset, loads a3 from a2 plus that offset, reads a length byte into d7, and if nonzero advances a1 and copies bytes from a3 while pushing a1 and loading d3 from a3.
- `copy_one_byte_advance_pointers`: Copies a single byte from the address in a3 to the address in a1, incrementing both pointers, then returns to the caller.
- `save_registers_call_two_subroutines`: Pushes d0, d2 through d7 and a0 through a6 onto the stack, calls two subroutines, restores those registers from the stack and returns.
- `call_init_then_two_subroutines`: Calls the routine at the long address, saves d0/d2-d7/a0-a6 on the stack, calls two nearby subroutines, restores the registers and returns.
- `copy_block_backwards_to_offset`: Reads a length and offset from the structure at a0, then copies that many bytes backwards from a0 plus offset into a0 plus aligned length plus 256, restoring all registers.
- `expand_bits_to_longwords`: Saves all registers, takes a bit count in d0, and for each group of eight bits read from a0 shifts them through carry into d4, writing two longwords per group back to a0.
- `expand_1bpp_to_4_planes`: Saves all registers, reads packed bit data from a0, expands each bit into four bitplanes held in d0-d3 and d4-d5, writes eight bytes per group to a0 and repeats d0/8 times.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `copy_d4_byte_to_d3`: Copies the low byte of d4 into d3 and returns immediately to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.

### logged disk load from Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf

- `boot_clear_colors_alloc_chip_and_launch`: Disables DMA, clears all color registers, allocates chip memory, sets up an IO request to load the game file, calls SuperState and jumps to the loaded code.
- `save_regs_and_install_trap_vector`: Pushes d0 and a0 onto the stack, pushes the current trap vector at address 16, loads the address of the following code into a0 and stores it at address 16, then sets d0 to 8.
- `clear_d0_longword`: Clears the entire 32-bit d0 register to zero by writing a longword zero, then execution continues into the following code.
- `restore_trap_vector_and_registers_2`: Pops the saved trap vector from the stack into address 16, restores d0 and a0 from the stack, and returns to the caller.
- `interrupt_restore_stack_and_return`: Adds four to the stack pointer at offset two from a7, discarding a saved longword, then returns from the exception with rte.

### logged disk load from Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf

- `overlapping_memmove_forward_backward`: Copies d0 bytes between a0 and a1, choosing forward or backward direction by comparing the pointers, then jumps to an address held in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `relocate_rom_to_fast_ram_and_jump`: Checks the load address, copies 64 longwords from a nearby table to fast memory, then sets up a an address-byte copy and jumps to that memory.
