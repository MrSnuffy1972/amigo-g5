# Game 2078 (addams family, the (demo) (1992)(ocean)(m3)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02078). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901248 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 61 raw-word lines |
| Functions named and described | pass, 446 of 446 named (0 author symbols), 446 of 446 described (31 by rule, 75 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1865 of 1877 (99.36%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8339 of 8342 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 115868 |
| code | 33624 |
| data | 751726 |
| padding | 30 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf:bootblock | 1 | 33 | 3 |
| logged disk load from Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf boot-block load | 1 | 8270 | 436 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| logged disk load from Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf | 1 | 37 | 5 |

## Cases handled

- addams family, the (demo) (1992)(ocean)(m3)[a]/Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf: custom-format (non-DOS) disk, 1725 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 488 of the 857 PCs the canonical run logged (1415 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- addams family, the (demo) (1992)(ocean)(m3)[a]/Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf: the boot block reads 156672 bytes from disk offset 17408 to an address (found statically); recovered as a unit
- 11 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- prefix run f100 ignored: it logged 1046 PCs the canonical run never executed (not a prefix of it)
- prefix run f50 ignored: it logged 1046 PCs the canonical run never executed (not a prefix of it)
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- addams family, the (demo) (1992)(ocean)(m3)[a]/Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf: 35 executed PCs whose RAM was overwritten before any snapshot match its bytes 0-240: recovered as a unit run at an address
- addams family, the (demo) (1992)(ocean)(m3)[a]/Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf: 2 logged loads cover 156912 bytes of the image
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
- `check_7c86_state_two_values`: Tests the word at absolute address 7c86 and branches away if it is zero or equal to three, otherwise returns to the caller.
- `return_ff_word_constant`: Returns the constant word value 255 in d0 to the caller, with the surrounding instructions forming the routine body.
- `calculate_offset_value_2`: Calculates an offset value by adding a constant to register D0, then uses this value as an index into a table at address 14418.
- `calculate_offset_value`: Calculates an offset value by adding a constant to register D0, then uses this value as an index into a table at address 14418.
- `adjust_and_copy_word`: Adds a constant value to a register, then copies the modified word value from a table to another register, before returning.
- `initialize_disk_operations_2`: Initializes disk operations by clearing multiple word values in a structure, reading from hardware registers, and setting up DMA transfers with specific calculations and conditional branches.
- `concatenate_d0_and_a6_word`: Calls another routine, then returns a 32-bit value formed by concatenating the D0 register with a word read from an address computed from A6.
- `initialize_disk_operations`: Initializes disk operations by clearing multiple word values in a structure, reading from hardware registers, and setting up DMA transfers with conditional branches based on disk status and button presses.
- `execute_dma_operations`: Executes a series of DMA-related operations including memory transfers, comparisons, and conditional branches.
- `update_counter_and_flags`: Reads four bytes from a specific offset, performs several comparisons, and updates a counter value in memory before returning.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 24 into d0, then returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 27 into d0, then returns.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 25 into d0, then returns.
- `process_data_with_conditions`: Processes data through multiple conditional checks and operations, updating flags and counters based on comparison results before returning.
- `tiny_moveq_rts_4`: Small routine of 2 instructions that loads 255 into d0, then returns.
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
- `control_ciab_ports`: Controls specific bits in a hardware register through a sequence of bit set and clear operations when a counter reaches zero.
- `probe_and_claim_memory_block`: Saves all registers, checks two flags, then walks a pointer list at 18392 writing and verifying test patterns in each block until one passes, storing it at 18388 and marking it with a fixed address.
- `find_and_verify_named_block`: Saves registers, walks the block list from 18388 comparing the caller's name string, then copies the block's words to a1 while computing a rotating checksum and returns the block length in d1 or minus one.
- `queue_sound_sample_with_checksum`: Saves registers, calls three helpers, then if sound flags allow and space remains, copies a name string and word data into a sound queue node, computes a rotating xor checksum, writes terminator and links the node.
- `chain_four_subsystem_calls`: Calls four fixed subroutines in sequence via jsr and a tail jmp, touching no registers or memory itself beyond the stack for return addresses.
- `compare_d0_to_constant_and_return`: Compares d0 against a fixed immediate value, branches away if not equal, otherwise returns immediately; the branch target lies outside this routine.
- `store_byte_pair`: Reads a 16-bit value from a global table, stores its low byte at one location and its high byte at another location in the same table.
- `compute_and_accumulate_deltas`: Calculates differences between byte values stored in global memory locations, performs arithmetic operations on these differences, and accumulates results in other global variables.
- `set_ciaa_serial_and_clear_fire`: Writes 8 to a6 offset 154, stores 24800 at absolute address 96, then clears CIAA port A and sets port B to 136 before writing 49160 to a6 offset 154.
- `ciaa_fire_button_interrupt_service`: Disables and acknowledges the CIAA interrupt, reads port A fire bits, and on a fire press toggles port B, pulses port A lines, calls the key-state routine, then re-enables interrupts and returns from exception.
- `clear_memory_block`: Clears a block of memory by setting bytes to zero using a loop that decrements a counter.
- `initialize_string_processor`: Initializes a string processing loop by advancing a pointer past null-terminated text, setting a global counter to zero, and then entering a processing loop that modifies characters based on the counter's value.
- `clear_and_loop`: Clears a byte in memory, sets a register to zero, then branches unconditionally to another routine.
- `process_input_buffer`: Processes characters in a buffer, updating a global counter and modifying buffer contents based on the counter's value, then calls another routine before looping back to process the next character.
- `handle_input_condition_2`: Compares the value in register D0 with eight and branches to different sections based on equality, then performs conditional operations involving registers D1 and D2 before branching to a common location.
- `handle_input_condition`: Compares the value in register D0 with eight and branches to different sections based on equality.
- `process_input_character`: Processes a character from input, storing it in a buffer and updating a global counter, with conditional logic based on the counter's value and input character range.
- `call_and_return`: Calls a function through an address in a register, restores saved registers from the stack, and returns to the caller.
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
- `copy_negated_indexed_byte_to_a5_offset_36`: Loads the byte at offset 22 from a0 indexed by d0 into d1, negates it, stores it at offset 54 of a5, then branches to another routine.
- `copy_anim_byte_and_clear_flag`: Copies a byte from offset 22 of a0 indexed by d0 into offset 47 of a5, clears the byte at offset 49 of a5, then branches to another routine.
- `advance_script_pointer_and_apply_velocity`: Decrements d0, stores it at offset 44 of a5, copies a byte to offset 2, advances a0 by 158, and steps a script pointer through a0 while updating a3 display offsets and velocity from a5 fields.
- `step_frame_script_and_apply_bob_offset`: Decrements d0 into offset 46 of a5, reads a frame index from a0 into a1 table to set offsets 24 and 6 of a3, then adds a sine-scaled bob offset and pops a3.
- `wait_two_vblank_toggles`: Saves d0 on the stack, then polls the VHPOSR register until its low byte changes or a single-count dbra expires, restoring d0 and returning.
- `apply_next_display_entry`: Loads a pointer from (a1)+ into a5, shifts d0 right by one bit, and if the shifted-out bit was set and the entry has a nonzero length, copies that length and a word into the entry's buffer.
- `enable_dma_and_flush_three_entries`: Reads a word from PC-relative memory, enables DMA by setting bit 15 and writing DMACON, waits for a vblank toggle, then applies three display entries and branches to a fourth.
- `animate_entity_sprites_and_dma`: Steps animation counters for up to four entity slots, updates sprite frame pointers and channel DMA bits in DMACON, and jumps through a per-slot dispatch table.
- `misdecoded_data_table_after_jump_table`: Bytes decoded as instructions that write meaningless immediates to odd addresses; this is data, not executable code, ending in a branch to the animation loop.
- `add_and_load_word`: Adds a register's value to a word in memory, then loads that updated word into another register before branching.
- `select_animation_frame_from_direction`: Uses d4 direction and d3 frame counter to pick a word from the table at offset 66 of the entity, returning it in d5 before branching to the shared exit.
- `clamp_volume_byte_and_branch`: Masks d4 into d1, subtracts d4 from the volume byte at offset 2 of a5, clamps it to zero if negative, stores it back and branches to the shared exit.
- `add_high_nibble_to_volume_cap_64`: Shifts d1 right by four bits, adds it to d0, caps the result at 64, stores it at offset 2 of a5 and branches to the shared exit.
- `compare_and_subtract`: Compares a register with a constant, branches if the value is greater or equal, otherwise subtracts a register's value from another register and branches.
- `update_sound_counter_2`: Performs a word subtraction between two registers, compares the result with another register, clears a global variable if the condition is met, and updates sound channel data through multiple conditional branches.
- `update_sound_counter`: Performs a word subtraction between two registers, compares the result with another register, clears a global variable if the condition is met, then copies the cleared value to multiple global variables and branches to another routine.
- `apply_volume_and_advance_sound_channel`: Writes d5 and the scaled volume byte into the channel structure at 20(a5), increments d7, compares it with the channel limit and either loops back or restores registers and returns.
- `update_sound_parameter`: Reads a byte from a global structure, processes it through a lookup table, scales the result using another byte from the structure, and stores the final value back into the structure.
- `conditional_branches_2`: Compares a register with decimal 241, and if not equal, branches to another location; otherwise compares another register with decimal 3, and if not equal, branches to yet another location.
- `conditional_branches`: Compares a register with two different constants using branch instructions, with conditional jumps based on comparison results.
- `process_sound_sample`: Checks if a data value equals decimal 248, and if so, clears bit 1 of a global variable located at offset an address; otherwise it returns.
- `check_and_clear_flag`: Checks if a data value equals decimal 248, and if so, clears bit 1 of a global variable located at offset an address; otherwise it returns.
- `update_sound_dma_and_channels`: Sets a bit in a hardware register, updates sound channel data, and calls a routine to enable DMA and chain sound setup when conditions are met.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `load_sprite_frame_pointer_and_advance`: Stores a table address into a5 offset 66, reads a word indexed by d1 from that table into d5, loads a long from a0 into d1, advances a0 by six, reads two words from a3 into d2 and d3, then returns.
- `store_sprite_descriptor_by_index`: Masks d0 to two bits and scales it by eight to index a table via a5, stores a0 there, packs bits of d2 and d1 into byte four, d3 and d2 into byte five, and d4 into byte six, then returns.
- `process_sound_command`: Reads sound command data from a table and updates various sound parameters in a structure, including channel settings, sample pointers, and DMA configuration.
- `copy_sprite_rows_to_buffer`: Copies rows of sprite data from the structure pointed to by a0 into a fixed buffer, or clears the buffer when the source is missing, then restores registers and returns.
- `clear_memory_block_2`: Clears 62 words of memory starting at the address pointed to by a2 using a loop.
- `init_level_and_report_success`: Sets up the level with the given mode, calls the level loader and setup routines, and returns zero on success or minus one after the fallback path.
- `conditional_run_two_setup_routines`: Reads a flag byte from pc-relative memory; if nonzero, calls two subroutines in sequence, then returns to the caller.
- `store_object_and_configure_fire_button`: Saves a0 into four globals, calls a setup routine, then stores a byte from the object into memory and sets or clears the CIAA fire-button bit based on a flag.
- `clear_joystick_bit`: Clears bit 1 of the CIAA_PRA register and stores a byte from memory location 508 of structure in a0 to a global table, then sets a global variable to 2 before returning a word value from memory location 764 of that structure.
- `conditional_clear_word_and_call`: Reads a pc-relative flag byte; if nonzero, clears a word in memory and calls a subroutine, then returns.
- `clear_global_70dc_and_return_zero`: Saves d2-d3/a4/a6, loads a6 from address 4, clears the word at memory 28892, sets d0 to zero, restores registers and returns.
- `copy_d2_to_d0_and_branch_back`: Moves the longword in d2 into d0 and branches backward into the preceding routine, continuing its flow.
- `read_and_clear_flag_byte_at_6452`: Saves a6, loads the byte at the pc-relative location 6452 into d0, clears that byte if nonzero, restores a6 and returns.
- `cold_boot_init_and_main_loop`: Sets supervisor mode and stack, fills low memory and chip RAM with fixed values, initializes hardware and game state, then runs the main loop calling many game routines.
- `select_execution_path`: Compares a 16-bit value at a specific memory location with decimal 3, branches to another routine if they are equal, otherwise compares it with decimal 2 and branches to yet another routine if they are equal, otherwise it continues execution.
- `setup_and_execute`: Loads several addresses into registers, calls a function with those addresses as parameters, then branches to another location.
- `setup_and_execute_2_2`: Branches unconditionally to a different location without performing any other operations.
- `setup_and_execute_2`: Loads several addresses into registers, calls a function with those addresses as parameters, then branches to another location.
- `load_and_execute_6`: Branches unconditionally to a different location without performing any other operations.
- `load_and_execute`: Loads three addresses into registers and executes a function using one of them as parameter, then branches to another location.
- `update_global_value`: Clears a global variable, loads a word value from a table using another global variable as index, compares it with the global variable, and branches to different routines based on the comparison result.
- `load_and_execute_2`: Loads addresses from specific locations into register pairs and calls a function, then branches to another location.
- `check_and_branch`: Compares a 16-bit value at a specific memory location with decimal 10 and branches to another routine if they are equal, otherwise continues execution.
- `jump_to_load_and_execute`: Branches unconditionally to another routine that loads addresses from specific locations into register pairs and executes a function, then continues execution at a different location.
- `load_and_execute_3`: Loads addresses from specific locations into register pairs and calls a function, then branches to another location.
- `jump_to_next`: Unconditionally branches to another routine located at a fixed offset from the current location, effectively acting as a jump table entry.
- `load_and_execute_4`: Loads addresses from global locations into registers and executes a function using those addresses as parameters, then branches unconditionally to another location.
- `branch_to_update`: Unconditionally branches to another routine that updates a global variable and performs several function calls, then returns.
- `load_and_execute_5`: Loads three addresses into a0, a1, and a2 registers, then calls a routine using the address in a2, and finally branches to another location.
- `jump_to_function`: Branches unconditionally to another location without performing any other operations.
- `execute_multiple_functions`: Calls five different functions in sequence and increments a global counter by one, with the first function call using a specific address as parameter.
- `wait_for_space_key_press`: Sets two flags, calls a routine with a pointer, then repeatedly polls the key byte until a space is seen, restores a word and clears both flags.
- `set_level_state_four_ten_zero_two`: Writes four to the level word, ten to the next word, clears a third word and stores two in the state word, then returns.
- `initialize_globals`: Writes specific 32-bit values to two memory locations and then branches to another location.
- `init_level_and_run_tile_animation`: Sets up level pointers and counters, copies a block of data, reads five bytes from the current animation record into the position and timer variables, then draws the tile and advances the record until the terminator.
- `draw_tiles_with_bit13_set`: Walks a 14 by 16 tile map, and for each cell whose high byte has bit 13 set, decodes the tile index and draws it into the screen buffer via the tile blitter.
- `draw_tiles_with_bit13_clear`: Walks a 14 by 16 tile map, and for each cell whose high byte has bit 13 clear, decodes the tile index and draws it into the screen buffer via the tile blitter.
- `build_bitplane_tile_rows`: Tests bit 6 of the byte at a0; if clear, programs custom chip registers at offsets 100, 102, 64 and 68 of a6, then for sixteen rows reads words from d6, reverses their bits into d1 and stores them at offsets 0, 38, 76 and 114 of a1, clearing offset 152 and advancing a1 by 190.
- `bonus_room_enter_and_draw`: Clears the room counters, blanks five bytes of a text buffer, draws the room graphics and sprites, then waits on the input flag and returns the room number in d0.
- `validate_and_init`: Checks if specific bytes at a fixed memory location match predetermined values, and if so, initializes a global variable to -1 and performs a series of function calls with hardcoded parameters.
- `call_and_check`: Calls a function, loads a word value from memory into a register, checks if it's negative, and returns immediately if it is.
- `initialize_and_loop`: Initializes several global variables with specific values, calls multiple functions, sets a byte to 1 in memory, and enters an infinite loop that does not return.
- `initialize_and_compare`: Initializes a byte variable to zero, then copies bytes from one memory location to another while comparing each byte with a fixed value, incrementing the initialized variable when matches occur.
- `validate_and_clear`: Processes data through a loop using indexed addressing, compares byte values with specific constants, performs bit operations on a register, and clears a global variable before returning.
- `set_global_word`: Sets a 16-bit value to -1 in a global variable and then returns.
- `update_counters_based_on_bits`: Reads bits from two memory locations, increments various global counters based on which bits are set, and sets several other global variables to specific values.
- `test_and_update_global`: Tests bit 3 of a global variable and branches to another routine if it is set, otherwise tests bit 2 and branches if that is set.
- `update_bit_and_counter`: Tests bit 3 of a global variable, and if it is clear, decrements a counter at offset 8ff2 and updates the global variable with a byte from offset 3ca5.
- `update_bit_and_counter_2`: Tests bit 2 of a global variable, and if it is clear, increments a counter at offset 8ff2 and updates the global variable with a byte from offset 3ca5.
- `dispatch_input_flags_to_handlers`: Tests bits 3,2,1,0 then 4 of the byte at 15525 and branches to separate handlers, otherwise copies that byte to 38950 and returns.
- `decrement_counter_on_bit3_clear`: If bit 3 of 38950 is clear, subtracts one from the word at 38954, masks it to two bits, then copies 15525 into 38950 and returns.
- `increment_counter_on_bit2_clear`: If bit 2 of 38950 is clear, adds one to the word at 38954, masks it to two bits, then copies 15525 into 38950 and returns.
- `toggle_bit1_and_decrement_counter`: Tests bit 1 of the byte at memory location 9826; if clear, decrements the word at 9828, masks it to three bits, then copies the byte at 3ca5 into 9826 and returns.
- `toggle_bit0_and_increment_counter`: Tests bit 0 of the byte at memory location 9826; if clear, increments the word at 9828, masks it to three bits, then copies the byte at 3ca5 into 9826 and returns.
- `copy_tile_byte_and_advance_index`: If bit 4 of the byte at 9826 is clear, copies one byte from the table at 982e indexed by 982a and 9828 into the buffer at 139df indexed by 982c, increments that index, refreshes 9826 from 3ca5 and returns.
- `loop_forever_with_globals`: Performs a sequence of operations involving global variables and function calls, then enters an infinite loop without returning.
- `compare_and_jump`: Compares a word value at a memory location with the number 9, branches to another location if they are not equal, and otherwise jumps to a third location.
- `initialize_and_execute_loop`: Initializes multiple global variables with specific values, calls several functions, executes a loop that decrements a counter until it reaches zero, and then jumps to another routine.
- `initialize_game_state_2`: Sets multiple global variables to specific values, clears several variables, calls another routine, and then enters an infinite loop.
- `execute_loop_sequence`: Calls a function at a fixed address, sets register d7 to 200, calls another function at a fixed address, decrements d7 in a loop until it reaches zero, then writes values 256 and 1 to memory locations 39326 and 39722 respectively before branching to address 35776.
- `loop_and_copy`: Initializes a loop counter with 200, calls a function in each iteration, then performs a second loop that copies data from one memory area to another.
- `copy_3ba8_block_to_3b2e`: Copies four words from the long addresses around 3ba8 into the low-memory words at 3b2e, 3b30, 3b2a and 3b2c, then copies a longword from 12908 into 3b24 and returns.
- `reset_display_and_clear_3ba8`: Calls several graphics and sound routines, writes all-ones to 3c04, clears the longwords at 3ba8 and 3ba4, calls the same routine twice, then calls another routine and returns.
- `call_11cea_then_9dae`: Calls the routine at 11cea, then calls the routine at 9dae, and returns to its caller without touching any registers itself.
- `setup_display_and_wait_forever`: Calls three subroutines, writes the value 496 to two adjacent words at 15146, clears that longword, sets byte 15138 and loops forever while it stays nonzero.
- `clear_chip_ram_68000_to_80000`: Calls a subroutine, then zeroes longwords from address a fixed address up to but not including a fixed address using a0 as the pointer, and returns.
- `rearrange_table_entries`: Uses a global value to index into a table, retrieves a word from that location, and if the value is non-negative, shifts four long values in memory in a specific order.
- `initialize_and_execute_sequence`: Sets up several global variables with initial values, calls multiple functions including one that uses stack parameters, and then enters an infinite loop that does not return.
- `fade_palette_colors_down_and_copy`: Repeatedly decrements each of the 32 color words at a fixed address by one per nonzero nibble, calls a routine, copies 16 longwords to COLOR00, and repeats 16 times before a final call.
- `clear_state_and_copy_palette_buffer`: Clears several long and word state variables, sets counters and pointers, zeroes 1200 bytes at the sprite buffer, then copies 256 bytes from one palette buffer to another.
- `init_display_and_build_sprite_pointers`: Sets a flag and two pointers, calls four system routines, copies 256 palette bytes, then fills 256 longwords with increasing sprite frame addresses.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `set_level_counters_and_jump_to_main_loop`: Calls two library routines, stores 4 and 10 into two word counters, clears a third word counter, then jumps to the main loop entry.
- `initialize_level_counters`: Sets specific global variables to predefined values and performs conditional branching based on another variable's state.
- `save_and_execute`: Saves register values and executes a sequence of functions with loop iterations, then restores the saved values and returns.
- `initialize_game_globals`: Initializes multiple global variables to zero, calls several functions in sequence, waits for a bit to be set, and then performs conditional operations based on global state values.
- `setup_game_globals`: Initializes several global variables with specific values, calls library functions, and conditionally executes further operations based on a global variable's value.
- `setup_global_variables`: Clears several global word variables, sets one to -1, calls initialization routines, and copies color data from a table to hardware registers before jumping to another routine.
- `zero_registers_and_jump_to_init`: Sets d0, d1 and d2 to zero and d3 to 32768, then jumps to the game initialization entry point.
- `fade_palette_toward_target`: Saves all registers, clears 16 longs of a work buffer, then repeatedly calls a routine and steps each of 32 words one nibble closer to a target table before copying 16 longs to COLOR00.
- `toggle_flag_and_copy_256_bytes`: Tests a byte flag, copies it and toggles bit 1, calls a routine, then copies 256 bytes from one buffer to another and returns.
- `clear_game_work_buffers`: Clears several memory regions: 304 bytes, 3840 bytes, 32 words spaced 16 bytes apart, 21 longs and 16 longs, then returns.
- `init_level_and_call_setup_routines`: Copies four word globals into nearby state words, stores a longword, then calls five subroutines and clears a counter before returning.
- `toggle_bits_and_reset`: Toggles specific bits in a data table and resets a counter variable when conditions are met.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `clear_low_bits`: Clears the low bit of each byte in a sequence of memory locations and resets a counter variable to zero.
- `set_flag_and_update`: Updates multiple memory locations with specific values, sets a flag bit, and clears certain fields in a data structure.
- `scan_entity_table_for_collisions`: Iterates a table of entity records, computes screen offsets, updates collision counters and calls one of three handlers based on the entity type byte.
- `dispatch_tile_command_byte`: Clears a word at a1, reads a byte from a tile map indexed by a1 fields, clears it, and branches to handlers for command bytes.
- `append_three_bytes_to_ring_buffer`: Loads the write pointer, builds a two-byte value from two globals, stores three bytes at the pointer, advances it by three, and wraps it back to the buffer start when it passes the end.
- `compare_and_clear`: Compares values from two memory locations and clears a byte at a calculated address when the values match.
- `advance_bcd_counter_and_spawn_entity`: Calls the byte writer, plays a sound, increments two BCD counters, and on wrap or a count of thirty-seven triggers sound and setup calls before initialising an entity block at a1.
- `increment_bcd_byte_and_play_sound`: Clears d0, loads a BCD byte counter, skips if it equals one hundred fifty-three, otherwise adds one with abcd, stores it, calls the sound routine and branches back.
- `call_e0b8_then_jump_9308`: Calls the subroutine at absolute address e0b8, then branches unconditionally to the code at 9308.
- `poll_3d00_and_increment_counter`: Calls a setup routine, invokes the routine at 3d00 with d1 and d2 set, then reads the word at 3bc6, increments it in packed BCD unless it equals 153, stores it back, calls the routine at ff1e and jumps to 9308.
- `check_3cbe_and_decrement_3cbc`: Compares the word at 3cbe with 3, and if not equal increments that word and decrements the word at 3cbc before branching to 9308.
- `set_word_ffff_at_3b8c_then_branch`: Writes the immediate value 65535 to the longword-addressed word at 3b8c, then branches unconditionally to the shared code at 9308.
- `save_regs_call_3d00_set_two_words`: Pushes all data and address registers, loads d1 with 12 and d2 with 18, calls the routine at 3d00, restores the registers, writes 3 to 3bf2 and 40 to 3bce, then branches to 9308.
- `branch_to_shared_code_9308`: Contains only an unconditional branch to the shared code at 9308, transferring control without touching any registers or memory.
- `branch_to_9308`: Branches unconditionally to the code at address 9308, transferring execution there without touching any registers or memory.
- `validate_and_update_variables`: Compares a 16-bit value from memory with decimal 65530, branches if greater or equal, loads a word from memory at an offset, clears two global word variables, and stores another value to a different location.
- `validate_and_jump`: Tests multiple conditions involving global variables and memory locations, performs comparisons and conditional branches, and ultimately jumps to a fixed memory location.
- `update_position`: Updates a global variable with a calculated value based on input parameters and resets another variable to zero or two.
- `check_condition_positive`: Tests a 16-bit value at a specific memory location and branches to another routine if the value is positive, otherwise it returns.
- `validate_condition`: Compares a 16-bit value at a specific memory location with decimal 1 and branches to another routine if they are not equal, otherwise it returns.
- `validate_and_reset`: Tests value at 15450 and branches to 40162 if zero, tests bit 0 of word at offset 12 from a1 and branches to 40162 if negative, clears memory location 15240.
- `set_flags_and_reset`: Calculates a new position value, updates several variables, sets a flag bit, and resets specific global variables to zero.
- `call_and_return_2`: Calls another routine and then returns immediately without performing any other operations.
- `test_and_continue`: Tests a 16-bit value at a specific memory location for zero condition and branches to another routine if it is zero, otherwise returns immediately.
- `check_condition_negative`: Tests a 16-bit value at a specific memory location and branches to another routine if the value is negative, otherwise it continues to a different address.
- `update_game_state`: Updates a global variable based on conditions tested on a byte pointer, and performs a decrement operation on another global variable before returning.
- `set_global_flag`: Stores the decimal value 1 into a 16-bit word at a specific memory location and then returns.
- `check_and_clear`: Tests bit 7 of a byte at an offset from a pointer, branches to another routine if the bit is set, clears a byte at another offset from the same pointer, then jumps to a different routine.
- `bit_test_and_branch`: Tests bit 6 of a byte at a specific memory location and branches to another routine if it is set, then tests bit 3 and branches to yet another routine if it is clear.
- `evaluate_conditions`: Tests a 16-bit value at a specific memory location for negative or zero conditions and branches to another routine if either condition is true, otherwise it branches to yet another routine.
- `check_and_branch_conditions`: Compares a 16-bit value from memory with decimal 65528, branches if less than, compares another value from memory with zero, and branches if not equal.
- `update_animation_state`: Updates an animation state variable based on conditions involving global flags and counters, then stores a byte from a global variable to a structure offset 23, and finally returns.
- `jump_to_handler`: Unconditionally branches to another routine at a different memory location.
- `test_and_return`: Tests bit 3 of a byte at an offset from a pointer and branches to another routine if the bit is clear, otherwise it returns immediately.
- `select_and_assign`: Selects a value based on conditional tests and assigns it to a memory location, then clears another field.
- `reset_and_execute`: Clears a 16-bit word value at a specific memory location, tests if another 16-bit word value at a different memory location is non-zero and branches to a routine if it is, otherwise tests a third 16-bit word value and branches to yet another routine if it is non-zero, then clears a byte at an offset from address a1, calls a library function, and finally branches back to the first routine.
- `check_and_execute`: Tests multiple global variables for non-zero values and branches to different addresses based on the results, then calls a library function and returns.
- `per_frame_game_update_dispatch`: Calls the frame setup routine, checks several state words, decrements a counter, updates the two nibble counters at 3ba4 and 3ba6, then calls the sound, sprite and display routines before returning.
- `process_and_update`: Processes data through nested loops, performs calculations, and updates memory locations with values from a lookup table.
- `clear_sprite_slot_table`: Calls the sprite reset routine, then walks twenty entries at a86e, and for each active entry with valid fields calls the slot handler and clears its four bytes.
- `spawn_entity_from_spawn_table`: Looks up a spawn record by index in d0, checks the spawn coordinates in a0 against screen limits, then finds or fills a 60-byte entity slot with position, type, pointer and zeroed counters.
- `update_all_entity_slots`: Iterates 64 entity slots at a0, advancing positions by velocity, decrementing timers, fetching script words from a1 and jumping through the opcode table when a negative command is read.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a1) to a1, then returns.
- `update_coordinates`: Moves words from a1 to memory locations 36(a0), 38(a0), 16(a0), 50(a0), 48(a0), reads two words from a1 into d2 and d3, tests bit 1 of a0, branches to 47946 if clear, adds d2 to 44(a0) and d3 to 46(a0).
- `add_coordinates`: Adds a 16-bit value from a data register to a 16-bit word at an offset from a pointer, and adds another 16-bit value from a second data register to a 16-bit word at a second offset from the same pointer.
- `copy_and_clear`: Copies a 32-bit value from a pointer to a memory location at an offset from another pointer, then clears a 32-bit word at a second offset from the same pointer.
- `move_word_value`: Copies a 16-bit value from a pointer to a 16-bit word at an offset from another pointer.
- `clear_table_entry`: Clears a 16-bit counter in a global variable, multiplies another value by a constant, adds a third value, uses the result as an offset into a table, and clears the byte at that table location.
- `clear_multiple_words`: Clears three 16-bit words at offsets 0, 6 and 8 of a pointer and returns.
- `load_tile_data`: Loads tile data from a lookup table using coordinates and stores the result in a structure, then sets a bit in that structure based on comparison with a register value.
- `clear_bit_six`: Clears bit 6 of a byte at an offset from a pointer, leaving all other bits unchanged.
- `copy_pointer_and_address`: Copies a pointer from a source location to offset 28 in a destination structure, then copies another pointer from the same source location to offset 24 in the destination structure.
- `load_and_return`: Loads a 32-bit pointer from a global variable at offset 28 of the structure in a0, then returns.
- `read_and_adjust_pointer`: Reads a 16-bit value from the pointer in a1, extends it to 32 bits, adds it to the pointer, adjusts the pointer back by four bytes, and returns the original 16-bit value.
- `test_bit_and_adjust_pointer`: Reads a byte from a memory location, masks it with decimal 1, branches to another routine if the result is non-zero, and adjusts a pointer by two bytes.
- `test_bit_and_skip_pointer`: Reads a byte from a memory location, masks it with decimal 1, branches to another routine if the result is zero, and adjusts a pointer by two bytes.
- `main_game_state_dispatch`: Returns early if a flag is set, clears a word, reads low nibbles of two counters into d6 and d7, calls several subroutines, then branches on the game state word in d0 to different handlers.
- `spawn_entity_at_camera_offset`: Checks the game mode word, throttles a counter, calls the allocator, then fills the new entity block with type 1, a sprite pointer, position from the camera offsets plus 16 and 26, and clears its velocity fields.
- `spawn_scrolling_entity_with_wrap`: Counts down a spawn timer, calls the allocator, fills the entity block with type 1 and a sprite pointer, advances a horizontal offset by 20 wrapping at 360, and stores the offset and height 24.
- `spawn_entity_every_32_frames`: Returns unless a flag is set, increments a frame counter masked to 31, and on wrap calls the allocator to create an entity at the camera position with type 1 and a sprite pointer, shifting it down when the mode is 3.
- `scan_60_byte_records_for_nonzero`: Loads the address of a record table into a1, sets d7 to 63, tests the word at a1 and branches away if nonzero, otherwise clears d0 and returns.
- `scan_backwards_records_then_check_state`: Steps a1 back by 60 bytes per d7 count until a nonzero word is found, returning minus one if none; otherwise checks game state words and jumps through a vector.
- `count_matching_tiles_in_grid`: Computes a grid offset from level coordinates, scans two rows of three bytes, counts bytes within the min/max range, and sets a flag word when four or more match.
- `handle_input_key_and_start_level`: Calls a setup routine, sets display window size, then reads the current input character and updates game state flags, starting a level or jumping to a handler based on the key.
- `advance_animation_frame_on_timer`: Reads the animation frame index, looks up a word from a table, adds it to a running total, and advances the frame index or sets a flag when the total reaches the threshold.
- `check_state_conditions`: Reads state values from a structure and performs conditional checks on them, updating a global variable based on the results.
- `set_global_flags_and_jump`: Checks a bit in a global variable and branches if clear, then performs several conditional operations on global variables before jumping to another location.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `clear_global_flag`: Clears a 16-bit value in a global variable at offset 15296 and returns.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reset_global_variable`: Clears a 16-bit value in a global variable at offset 15228 and returns.
- `handle_state_transition`: Handles different state conditions from a structure, performs conditional operations, and updates global variables with specific values based on those conditions.
- `clear_global_var`: Sets a 16-bit global variable to zero and returns immediately.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `check_global_conditions`: Reads values from three global variables and compares a byte from a structure with two, then branches based on the results.
- `compare_and_increment_counter`: Reads a byte from a memory location, compares it with decimal 2, branches to another routine if they are not equal, and increments a global counter by twelve.
- `increment_global_counter`: Increments a 16-bit value in a global variable at offset 15276 by one and returns.
- `compare_and_jump_2`: Reads a byte from a memory location, compares it with decimal 2, and branches to another routine if they are equal, otherwise continues execution.
- `compare_and_decrement_counter`: Reads a byte from a memory location, compares it with decimal 2, branches to another routine if they are not equal, and decrements a global counter by twelve.
- `decrement_global_counter`: Decrements a 16-bit value in a global variable at offset 15276 by one and returns.
- `update_cached_pointer_if_changed`: Compares d0 with the value at a global long, and if different stores d0 into two global longs, otherwise returns without changing memory.
- `update_platformer_vertical_motion`: Reads input flags and a per-frame table at a fixed address to adjust the player's vertical position and velocity words, calling sound and collision helpers and returning early on certain states.
- `check_adjacent_tile_for_special_object`: Looks up tilemap entries at offsets based on the current level word, checks tile type and object field, and returns a flag or jumps to a collision handler.
- `play_sound_effect_and_return_minus_one`: Saves all registers, calls the sound routine with effect numbers six and twenty-three, restores registers, sets d0 to minus one and returns.
- `poll_input_and_toggle_light_state`: Clears two state words, polls input ports via library calls, checks returned bytes and branches to toggle a light bit, clear bytes or set bytes at a1 and a1+4095, returning a status word in d0.
- `swap_two_40_longword_blocks`: Saves a0 and a1, then swaps 40 longwords between the block at 3ee16 and the block at 3eeb6 using d0 as temporary, restoring a0 and a1 before returning.
- `check_and_set_flag`: Tests a bit in a global variable and branches if set.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `update_global_variables`: Compares a global variable with ten, increments it if less than ten, then performs several conditional operations on other global variables before returning.
- `decrement_and_assign_constants`: Decrements a 16-bit value in a global variable at offset 15278 by four, then assigns the constant 65532 to two other global variables, and finally returns.
- `compute_tilemap_offset_from_d0_d1_d6_d7`: Stores d6 and d7, computes two tile coordinates from d0, d1, d6, d7 with shifts and table bases, stores intermediate words, and returns a0 pointing into the tilemap at 3b4e.
- `accumulate_and_compare`: Moves a 16-bit word value from a memory location to a register, adds the value in another memory location to it, stores the result back to the same memory location, moves a 16-bit word value from yet another memory location to a register, compares it with decimal 8 and branches to a routine if it is greater than or equal to that value, then stores zero in the register and branches unconditionally to a third routine.
- `set_constant_and_branch`: Loads the constant 15 into a register, stores it in a global variable at offset 15170, then branches unconditionally to another location.
- `calculate_and_store_values`: Calculates a value using data from tables and registers, performs comparisons, and stores results in global variables based on the calculation outcome.
- `compare_and_update_variables`: Compares a 16-bit value in a register with the constant 65532, branches if less than, stores the register value to a global variable, updates two more global variables with arithmetic operations, and then returns.
- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `player_input_state_machine`: Reads controller bytes through a0, updates global state words at fixed addresses, calls input helpers, and returns a status word in d0.
- `compute_tilemap_cell_pointer`: Multiplies row and column words, adds the base of the tile map, and returns the cell address in a1; a second entry copies row and column into state words and advances the row counter.
- `decode_level_select_digit`: Reads a character through a0, and if it is a digit between 36 and 39 stores the low nibble as a level index and the high nibble as a previous value, then sets two state words.
- `read_difficulty_from_input_byte`: Reads a byte from the pointer returned by the input routine, and if it is an asterisk or plus sign stores a difficulty index and two nibbles from a table into globals, then sets two state words.
- `process_tile_attributes`: Reads a byte from memory, performs several conditional checks based on its value, extracts nibbles from a table entry, and stores these values to global variables while setting specific flags.
- `update_game_state_flags_and_branch`: Clears three state words, tests game flags and input, then branches to subroutines that set mode words and load pointers based on the button bits and a byte at offset twenty of the input buffer.
- `start_level_if_not_active`: If the active flag is clear, sets mode and counter words, clears several state words, loads a pointer, saves all registers, calls the level setup routine with two parameters, restores registers and returns.
- `init_sound_channel_and_set_mode`: Clears a word at 15450, copies two bytes into a sound chip register block at 52226, bumps the first byte, writes 1 to the third, calls the audio routine at 43296 with d2 set to a large count, then sets mode word 15228 to 2 and conditionally adjusts counters based on bit 4 of 15525.
- `reset_mode_counters_to_default`: Clears the word at 15296, writes 12 to 15284 and clears 15450, then returns; used as the alternate branch when bit 4 of the status word is clear.
- `blit_tile_row_from_map_to_bitmap`: Computes tilemap source offset from d6/d7 and scroll words, stores column and row indices, then copies four bytes per iteration from the map at a fixed address through a palette table at 56908 into the bitmap buffer at 15182, advancing by the row stride in d5.
- `decrement_sound_timer_and_fetch_next_byte`: Decrements the word counter at 15536; when it goes negative it reloads from 15538, reads a byte from the pointer at 15540 (resetting to 15544 if the byte is negative), stores the byte in 15286 and returns.
- `advance_sound_sequence_and_trigger_effect`: Copies the sequence counter at 15308 to 15306, and if it is not five increments the counter, calls the sound routine at 65186 and returns; the remaining code handles effect state, sprite registers and a countdown.
- `handle_game_state_update`: Saves all general-purpose and address registers to the stack, calls a routine with specific parameters, restores registers from the stack, compares two 16-bit values in global variables, increments a counter if they are not equal, calls another routine, and then returns.
- `step_sprite_frame_animation`: Checks the animation flags at 15236, 15360 and 15358, then decrements the frame delay byte in the table at 58088 indexed by the direction word at 15288, advancing the frame and calling the sprite update at 60304.
- `update_player_state_and_input_actions`: Reads the state word at offset 3b7c and, unless it is 15, clears 3b9a and calls movement, jump, attack and sound routines based on the sign and magnitude of the vertical velocity and the joystick bits at 3ca4/3ca5.
- `check_3ba6_low_nibble_and_high_bits`: Loads word at 15270, shifts right four bits, compares with word at 15496, then reloads and masks low nibble, returning via rts if both checks pass.
- `apply_vertical_velocity_and_trigger_sound`: Subtracts the words at 3bba and 3b9e from the word at 3bae, stores the result back, sums 3bba and 3b9e into d1, and if that sum is negative calls the sound routines at e8fe and e39a with all registers saved.
- `compare_and_mask`: Loads a 16-bit value from a global variable at offset 15274 into a register, compares it with a value in another global variable at offset 15506, and if they are not equal, branches to another location.
- `update_and_compare_coordinates`: Performs arithmetic operations on several 16-bit values in global variables, stores the result back to one variable, checks if a sum is negative, calls two routines if it's not negative, compares a shifted value with another global variable, and then returns.
- `validate_and_adjust_value`: Stores a 16-bit value from a register to a global variable, checks if another global variable is zero, reads a byte from memory through an offset, performs arithmetic and bitwise operations on the byte, compares the result with a constant, and returns either zero or negative one.
- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant 0 in d0 to the caller without any other effect.
- `process_input_and_update`: Reads a byte from a structure, tests its sign and a specific bit, and if both conditions are met, calls another routine and checks additional conditions before updating a global variable.
- `validate_and_process_input`: Compares a byte value with several constants and branches based on equality, tests a global word variable for non-zero, compares another global word variable with three, and branches based on equality.
- `validate_and_clamp`: Loads a 16-bit value from a global memory location, adds another 16-bit value from a different location to it, branches if the result is positive, moves the result to another register, compares it with a value from yet another location, and branches based on equality.
- `clamp_and_execute`: Moves a 16-bit word value from a specific memory location to a register, compares it with decimal 120 and branches to another routine if it is greater than that value, then calls two library functions and returns.
- `validate_and_clear_variables`: Tests a 16-bit value from a global memory location and branches to an address if it is zero.
- `clamp_negative_value`: Loads a 16-bit value from a global memory location, checks if it's positive, compares it with a constant, and stores a different constant to the same location if the comparison fails.
- `clamp_and_store_sum`: Adds two 16-bit values from registers, stores the sum to a global variable, checks if the sum is positive, compares it with a constant, and stores the constant if it's greater than that value.
- `process_graphics_coordinates`: Performs coordinate calculations using global variables, calls a graphics routine with calculated values, reads a byte from memory, branches if the value is positive, performs bitwise operations on the byte, and branches based on equality.
- `validate_input_byte`: Checks a byte value from a register against several constants and performs conditional jumps based on the comparison results.
- `validate_and_process_input_2`: Adds two 16-bit values from global variables, checks if the result is negative, performs several comparisons with other global variables, and calls library routines when certain conditions are met.
- `validate_and_process_variables`: Tests a 16-bit value from a global memory location and branches to an address if it is zero.
- `add_to_global_var`: Loads a 16-bit value from a global variable into a data register, adds another value from a different data register to it, and stores the result back into the same global variable location.
- `add_and_shift_coordinates`: Adds a word value from register D1 to a global variable at offset 15238, loads another 16-bit value from a global location, shifts it left by four bits, negates the result, adds the sum to the previous addition, and branches to an address if the result is positive.
- `mask_coordinate`: Masks a 16-bit value in a global variable with 4095 and stores the result back to the same variable, then returns.
- `add_and_compare_coordinates`: Loads a 16-bit value from a global location, shifts it left by four bits, tests a word value in register D1 and branches to an address if it is positive.
- `add_and_mask`: Adds a 16-bit value from a register to a global variable, masks the result with 4095, and stores it back to the same variable before returning.
- `update_animation_frame_and_direction`: Checks game state flags, decrements the frame counter, reads a signed or unsigned byte from the animation table at a0 indexed by the counter, stores it as direction, and calls the animation update routines.
- `advance_animation_when_state_zero`: If the state word is zero it calls the sound routine, then depending on the sign of the direction word calls the frame advance or reverse routine and branches to the common exit.
- `step_animation_by_direction_flags`: Tests the frame counter and direction flags, calling the sound, forward and reverse animation routines conditionally, then returns to the caller.
- `update_and_process_counter`: Sets a 16-bit value in a global memory location to one, loads another 16-bit value from a global location into register D0, and branches to an address if the value is zero.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `compute_scaled_value`: Takes an absolute value of a global variable, adds one to it, shifts the result left by three bits, and conditionally applies additional bit shifts based on other global variable values before storing the final result back to another global variable.
- `set_bit_one`: Performs a bitwise OR operation between a global variable and the value one, storing the result back to the same global variable location.
- `loop_until_positive`: Tests a 16-bit value in a global variable for positivity, enters a loop that continues while the value is non-negative, then copies another global variable's value to the first one before returning.
- `dispatch_menu_selection_text`: Branches on a menu state word, loads text pointers and calls a text routine, appends characters from a3 into a buffer until a null byte, then returns.
- `copy_eight_blocks_to_two_buffers`: Copies eight blocks of fourteen longs from a table selected by the global counter into two alternating destination buffers, clearing the final long of each block.
- `copy_data_blocks`: Copies data blocks from different source locations to two destination locations based on an index counter, with conditional logic determining which source table to use for each iteration.
- `interleave_bytes_and_words_2`: Reads byte values from two memory locations using a 16-bit value as indexed offsets, combines them into words, and writes these words to multiple memory locations in a specific pattern.
- `interleave_bytes_and_words`: Interleaves bytes and words from two different memory locations into a third location by using input word values as indexed offsets to select data elements from a lookup table.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `decode_joystick_bits_to_flags`: Reads the word at offset 12 of a6, derives five flag bits from its joystick direction bits and the CIAA fire button, and stores the result to a global word.
- `draw_two_sprite_frames_wait_blitter`: Computes two sprite frame pointers from the global frame index, programs the blitter registers at offsets 64 to 116 of a6, calls the blit routine, and waits on the blitter-done bit after each frame.
- `set_palette_and_draw_three_frames`: Builds a palette value from the global index into a6 offset 64, calls the palette and blit routines, then draws two sprite frames from the global index and waits on the blitter-done bit.
- `set_palette_and_draw_four_frames`: Builds a palette value from the global index into a6 offset 64, calls the palette and blit routines, then advances the source pointers and blits four frames before waiting on the blitter-done bit.
- `compute_level_row_offset`: Reads the word at 15148, derives a row index by halving it, adding 8 and subtracting its low nibble, wraps it into 0-255, then scales it by 272 into a1 and returns two shifted column offsets in d1 and d2.
- `compute_row_offset`: Computes an offset value based on a loop counter, performs multiplication and addition operations with global variables to calculate memory addresses for row-based data access.
- `queue_two_display_requests`: Stores a1, a0 and d1 into a6 offsets 80, 84 and 88, waits while bit 6 of a6+2 is set, then if d2 is nonzero stores a2 and d2 there and waits again.
- `copy_six_byte_level_record`: Multiplies the word at 15158 by 6, indexes the table at a fixed address, and copies six bytes from that record into the bytes at a fixed address, a fixed address, a fixed address, a fixed address, 15529 and 15531.
- `build_all_five_sprite_lists`: Calls five list-building routines for each of the five sprite pointer arrays, then calls a final routine and branches onward.
- `load_sprite_blocks_into_copper_list`: Writes copper list entries for each sprite block, waiting on the blitter busy bit and advancing through four blocks per entry until a negative terminator.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `blit_four_tile_columns_from_list`: Writes blitter control words to a6 offsets and, for each entry in the a0 list, programs four blits at 38-byte column offsets until a negative terminator is read.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `blit_five_tile_columns_from_list`: Like the four-column blitter setup but programs five blits per a0 list entry, using d4 as an extra source and stopping when the list word is negative.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_next_routine`: A single unconditional branch instruction that jumps forward to the following routine, transferring control without touching any registers or memory.
- `init_display_and_run_three_subs`: Loads three fixed addresses into a0, a1 and a2, calls three subroutines, then loads an address into a0 and calls two more subroutines before returning.
- `blit_five_sprites_forward`: Sets up custom chip registers via a6, reads sprite pointers from a0, waits on bit 6 of the control register and writes blitter source, destination and size registers for five sprites.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `blit_five_sprites_offset_backward`: Configures custom chip registers through a6, reads sprite pointers from a0, adds an offset to each source, waits on bit 6 of the control register and writes blitter registers for five sprites.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `draw_foreground_sprites_by_priority`: Loops over 32 sprite slots at a fixed table, and for each active slot with priority below three, sets a global flag from its flags word and calls one of two draw routines depending on its type word.
- `process_sprite_slots`: Calls another routine, adjusts a stack pointer, decrements a counter in register D7, and branches back to an earlier instruction if the counter is not zero.
- `process_sprite_data`: Loads data from multiple global memory locations, performs bitwise operations, adds values to registers, and calls a routine based on conditional checks of the data.
- `process_sprite_data_2`: Calls a routine at a fixed address, adjusts the stack pointer to point to a location 16 bytes higher, decrements a counter in register D7, and branches back to an earlier instruction if the counter is not zero.
- `decrement_loop`: Performs a decrement loop using a register that starts at a specific value and decrements until it reaches -1, then returns.
- `update_player_sprite_and_position`: Computes a tilemap offset from the player's pixel coordinates, handles a respawn counter, then draws the player sprite at its screen position and toggles a global flip flag.
- `vertical_blank_interrupt_service`: Saves all registers, acknowledges the vertical blank interrupt, updates counters, calls the game update routines, reads a key and stores it, then clears the interrupt request and returns from exception.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `compare_and_mask_value`: Clears a 16-bit value in a global memory location, loads a word value from a pointer plus four into register D0, applies a bitwise AND operation with one to the value, shifts the result left by eight bits, loads a byte from a pointer plus six into register D0, compares it with the cleared global value, and branches to an address if the loaded value is less than the cleared value.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `install_trap_handler_and_set_count`: Saves d0 and a0 on the stack, pushes the current level 1 autovector, stores the address of code just after this block into the level 1 vector, and loads 8 into d0.
- `clear_d0_return_value`: Clears the entire d0 register to zero, leaving all other registers and memory untouched.
- `restore_trap_vector_and_registers`: Pops the saved level 1 autovector value back into the vector location, restores d0 and a0 from the stack, and returns to the caller.
- `increment_return_address_and_rte`: Adds four to the saved return address at offset two on the stack, then returns from the exception with rte.
- `configure_vector_table_and_copy_entries`: Selects interrupt and trap vector addresses based on status words, stores them into the vector table at a5, sets a6 offset 260, then copies eight pairs of words into the table at a0.
- `compare_and_branch`: Reads a word value from a table, masks it with 65280, compares it to 13824, and branches based on the comparison result.
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
- `setup_custom_register`: Loads the address of a hardware register into a register and returns immediately without making any changes to the register's contents.
- `build_two_plane_tile_entries`: Calls the tile lookup, computes two bitplane words from d0/d1/d3, writes them to two tile slots in a0 and stores their pointers in the a1 list, then repeats for a second row.
- `build_single_plane_tile_pair`: Calls the tile lookup, computes one bitplane word pair from d0/d1/d3, writes it to two consecutive tile slots in a0 and records both pointers in the a1 list.
- `store_value_repeatedly`: Writes alternating values from global variables into eight consecutive word locations with specific offsets.
- `copy_bitmap_columns`: Copies bitmap data in chunks of four words for multiple rows, updating pointers and handling row iterations based on a counter loaded from a register.
- `call_three_functions`: Calls three functions in sequence using the standard subroutine call mechanism, preserving all general-purpose and address registers on the stack before the calls and restoring them afterward.
- `copy_and_process_data`: Copies word values from a source to multiple destinations based on a counter, updates pointers, and processes structured data by reading byte values and performing bit shifts to calculate offsets for further operations.
- `clear_and_copy_data`: Clears memory locations and copies word values from a source to multiple destinations based on a counter, then updates pointers and returns the difference between two addresses.
- `process_data_blocks_2`: Processes data blocks by iterating through a loop that calls another routine multiple times per iteration, updating pointers and checking for specific conditions.
- `process_data_blocks`: Processes data blocks by iterating through a loop that calls another routine multiple times per iteration, updating pointers and checking for a specific marker value before continuing or exiting.
- `translate_and_copy_bytes`: Translates bytes from a source buffer through a lookup table and copies the translated values to a destination buffer, processing multiple byte pairs in a loop with indexed table lookups.
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
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `blit_sprite_masked`: Performs a masked blit operation for a sprite using calculated dimensions and offsets, setting up blitter registers and executing multiple blits with varying parameters.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `initialize_render_data`: Sets up rendering parameters including offsets, dimensions, and flags in a data structure pointed to by a6, then performs multiple conditional updates based on a status flag before returning.
- `decrement_byte`: Decrements a byte value pointed to by the address in register A0 by thirty-three.
- `initialize_game_data`: Loads multiple memory addresses into registers, calls several routines with different parameters, clears specific global variables, and performs a loop to copy data between memory locations based on a counter value.
- `initialize_game_data_2`: Calls another routine, loads a memory address into a register, calls a second routine, calls a third routine, and then branches to another location in the code.
- `setup_game_state_part1_2`: Pushes a register onto the stack and then branches unconditionally to another location without performing any other operations.
- `setup_game_state_part1`: Loads a memory address into a register, moves a specific value into another register, loads a third memory address into a register, calls a routine, and then branches back to an earlier location.
- `setup_game_state_part2_2`: Pushes a register onto the stack and then branches unconditionally to another location without performing any other operations.
- `setup_game_state_part2`: Loads a memory address into a register, moves a specific value into another register, loads a third memory address into a register, calls a routine, and then branches back to an earlier location.
- `save_pointer_and_branch`: Saves the pointer from a0 onto the stack and unconditionally branches to another routine.
- `load_game_data`: Loads game state variables including screen coordinates and sprite data pointers into memory locations referenced by a0, a1, and a2, then branches to another routine.
- `initialize_game_data_3`: Moves a register value onto the stack and then branches to another location without returning.
- `initialize_game_state`: Sets up various game state variables including screen coordinates, sprite data pointers, and hardware registers.
- `process_input_flags`: Processes input flag values from memory and stores them in a buffer structure.
- `process_input_character_2`: Reads a byte from memory, branches if the value is negative, compares it with decimal 32, and branches if equal.
- `handle_input_case`: Compares a byte value from a register with several constants and performs unconditional jumps to different addresses based on the comparison results.
- `load_memory_address_2`: Reads four bytes from memory starting at address in a0, combines them into a long value, loads that value into register a0, and then branches.
- `load_memory_address`: Reads four bytes from memory starting at address in a0, combines them into a long value, loads that value into register a0, and then branches to another location.
- `process_input_character_2_2`: Loads a memory address, advances a pointer by eight bytes, and then branches to a different location.
- `handle_input_case_2`: Calls a routine and then branches to a different location without returning.
- `load_memory_address_2_2`: Reads a byte from memory, performs a bitwise AND operation with 255, stores the result in a global variable, and then branches to another location.
- `calculate_address_offset`: Reads two bytes from memory starting at address in a0, masks them with 255, multiplies the second byte by 1520, loads the result into register a1, calculates an offset from registers a1 and d0, and then branches to another location.
- `draw_vertical_sprite_strip_columns`: Calls the blitter setup routine repeatedly to draw columns of a sprite, adjusting a1 by one or 1520 bytes and toggling bits in the control word at the global address.
- `build_masked_tile_bitplanes`: Reads eight words from a tile table indexed by d0, splits each into two bytes, complements their OR into a mask buffer, rotates masks together, then combines tile bytes into five bitplane rows at a2 and advances a2 by 190 per tile.
- `apply_tile_masking`: Applies bit masking operations to tile data during bitmap rendering.
- `dispatch_on_low_nibble_table`: Reads the low nibble of the word at the global state address, shifts it left twice, loads a code pointer from a jump table indexed by that value into a5 and jumps to it.
- `return_zero_constant_stub`: Returns the constant zero in d0 without touching other registers or memory, then returns to the caller.
- `copy_d4_to_d0`: Copies the value in register D4 to register D0 while clearing registers D1, D2, and D3.
- `zero_d0_copy_d4_to_d1_zero_d2_d3`: Clears d0, copies the low byte of d4 into d1, clears d2 and d3, then returns to the caller.
- `copy_d4_to_d0_and_d1_zero_d2_d3`: Copies the low byte of d4 into both d0 and d1, clears d2 and d3, then returns to the caller.
- `copy_d4_to_d2`: Copies the value in register D4 to register D2 while clearing registers D0, D1, and D3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.

### logged disk load from Addams Family, The (demo) (1992)(Ocean)(M3)[a].adf

- `boot_clear_colors_alloc_chip_and_launch`: Disables DMA, clears all color registers, allocates chip memory, sets up an IO request to load the game file, calls SuperState and jumps to the loaded code.
- `save_regs_and_install_trap_vector`: Pushes d0 and a0 onto the stack, pushes the current trap vector at address 16, loads the address of the following code into a0 and stores it at address 16, then sets d0 to 8.
- `clear_d0_longword`: Clears the entire 32-bit d0 register to zero by writing a longword zero, then execution continues into the following code.
- `restore_trap_vector_and_registers_2`: Pops the saved trap vector from the stack into address 16, restores d0 and a0 from the stack, and returns to the caller.
- `interrupt_restore_stack_and_return`: Adds four to the stack pointer at offset two from a7, discarding a saved longword, then returns from the exception with rte.
