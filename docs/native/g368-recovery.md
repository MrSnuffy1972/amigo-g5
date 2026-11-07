# Game 368 (Stardust v1.4 2102): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00368). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (6772948 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 137 of 137 named (0 author symbols), 137 of 137 described (13 by rule, 14 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1200 of 1200 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3654 of 3654 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 282719 |
| code | 13196 |
| data | 6120789 |
| padding | 4 |
| unused | 356240 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 655 | 29 |
| logged disk load from Disk.1 | 1 | 1074 | 41 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 391 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1526 | 42 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| logged disk load from Disk.1 | 1 | 6 | 1 |

## Cases handled

- Stardust_v1.4_2102/Stardust_v1.4_2102_hdf/Stardust/data/Disk.1: raw disk image (990160 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Stardust_v1.4_2102/Stardust_v1.4_2102_hdf/Stardust/data/Disk.2: raw disk image (1027312 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Stardust_v1.4_2102/Stardust_v1.4_2102_hdf/Stardust/data/Disk.3: raw disk image (990160 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Stardust_v1.4_2102/Stardust_v1.4_2102.hdf: hard-disk image (AmigaDOS OFS, 7264 blocks): its filesystem parses and all 11 files in it are byte-identical to the extracted tree Stardust_v1.4_2102_hdf, which is recovered file by file; no block outside the filesystem holds data
- 5 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 327 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 795 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Stardust_v1.4_2102/Stardust_v1.4_2102_hdf/Stardust/data/Disk.1: 6 executed PCs whose RAM was overwritten before any snapshot match its bytes 272-368: recovered as a unit run at an address
- Stardust_v1.4_2102/Stardust_v1.4_2102_hdf/Stardust/data/Disk.1: 6 logged loads cover 265152 bytes of the image
- Stardust_v1.4_2102/Stardust_v1.4_2102_hdf/Stardust/data/Disk.2: chunks 0-3995 (bytes 0-1022976) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.08)
- Stardust_v1.4_2102/Stardust_v1.4_2102_hdf/Stardust/data/Disk.2: chunks 3997-4007 (bytes 1023232-1026048) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.61)
- Stardust_v1.4_2102/Stardust_v1.4_2102_hdf/Stardust/data/Disk.3: chunks 0-0 (bytes 0-256) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.11)
- 2 packed track region(s), 1360128 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `call_check_and_callback` (game.slave): Calls another routine, then calls a function from a table at offset 36 of its pointer argument. Kind: password/string check. Pass path: the branch where iVar1 != 0 (the code following the check executes).
- `call_table_entry_twice` (game.slave): Calls another routine, then calls the table entry at offset 12 of its pointer argument twice, and returns the first argument unchanged. Kind: checksum trap. Pass path: the branch where the repeated call successfully executes without a crash or deviation.
- `start_disk_read_operation` (Disk.1 +an address loaded at an address (5632 bytes)): Configures the disk controller for a read: it writes the fixed sync word 17545 to the disk sync register, enables disk DMA and interrupt bits, and sets drive select bits through the CIA port. Kind: track/sector-based load-length check/sabotage path. Pass path: u1__l0_09A2.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `install_code_hooks_and_jump`: Stores its incoming base pointer, calls two routines through offsets 52 and 40 from that base, and conditionally writes a fixed byte value.
- `patch_code_and_tables`: Writes a long sequence of fixed constants and jump-to-routine instructions into a table of code locations, including a run of identical word stores and several 16-bit patches.
- `store_constants_then_dispatch`: Writes eight fixed constants to a global location in sequence, then calls two other routines and makes an indirect call through a table pointer.
- `stub_bad_instruction_data`: Stores the value 2 in a global location.
- `write_constants_and_dispatch`: Writes a fixed sequence of constants to a global location, with several calls to other routines in between.
- `write_constants_and_dispatch_3`: Writes a sequence of constants to a global variable and calls other routines.
- `update_global_values`: Modifies a global variable with several constant values and executes an indirect jump.
- `write_constants_and_dispatch_2`: Writes a longer fixed sequence of constants to a global location, with more calls to other routines in between.
- `return_without_action`: Returns immediately without touching any register or memory.
- `replace_words_matching_pattern`: Scans 16-bit words from a start address up to an end address.
- `replace_matching_word_pair`: Scans 16-bit words up to an end address.
- `clear_high_byte_of_operands`: Scans 16-bit words up to an end address.
- `replace_word_three_after_match`: Scans 16-bit words up to an end address.
- `replace_word_two_after_match`: Scans 16-bit words up to an end address.
- `shift_stack_longwords_down`: Copies a longword from the pointer argument into a global location, then shifts a block of thirteen longwords on the stack down by one longword.
- `patch_matching_code_sequences`: Walks a code region in two-byte steps and, wherever it finds one of two fixed instruction sequences, overwrites it with a jump or call instruction that points to another routine.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `read_longword_field_masked`: Reads a longword from offset 4 of its pointer argument, calls a routine through a table entry, and returns that longword masked to its low 20 bits.
- `call_check_and_callback`: Calls another routine, then calls a function from a table at offset 36 of its pointer argument.
- `dispatch_by_global_flag`: When a global flag is zero, calls the table entry at offset 36 of the pointer argument and, if that returns nonzero, calls the entry at offset 8.
- `call_table_entry_twice`: Calls another routine, then calls the table entry at offset 12 of its pointer argument twice, and returns the first argument unchanged.
- `return_empty_subroutine`: The routine is empty and performs no operations before returning.
- `install_jump_hooks_then_start`: Writes jump-to-routine and call-to-routine instruction words at two fixed code locations, then transfers control to the start routine.
- `wait_for_blitter_idle`: Calls the blitter-wait routine and then makes a store to a global location that writes its own value back.
- `wait_for_bit_clear`: sets a global variable to a constant value and polls that variable until a specific bit is cleared.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `set_blitter_mask_after_wait`: Waits for the blitter to finish, then writes a longword with ones in its high word and zeros in its low word to a field 68 bytes into its base structure.
- `set_blitter_word_after_wait`: Waits for the blitter to finish, then stores the word the wait routine left in the data register at offset 66 from its base structure.

### logged disk load from Disk.1

- `call_table_entry_260`: Calls a routine through a table entry 260 bytes from a base pointer held in a data register, then returns.
- `spin_in_infinite_loop`: Does no other work and loops forever without returning.
- `copy_block_then_jump`: Clears a global location, copies a block of code pointer entries from the routine's own area to a second location, then calls into the copied block.
- `call_single_unnamed_routine`: Makes one call to another routine.
- `configure_hardware_then_loop`: Calls another routine and then loops forever.
- `tiny_ori_moveq_move`: Small routine of 3 instructions that merges 244 into d0, then loads 5 into d2, then copies a4 to d0.
- `install_default_interrupt_vectors`: Writes the mask 32767, with the top bit clear, to the interrupt and DMA control registers to disable them, clears the first longword, and fills the low vector table with the address of a shared stub that returns from exceptions.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `write_interrupt_request_value`: Writes the value 7 to the interrupt request register.
- `relocate_loaded_hunks`: Walks a list of AmigaDOS hunk records, copying code and data hunks into their destination buffers and adding base offsets to longwords for each relocation record.
- `process_amiga_hunks`: Iterates through a sequence of AmigaDOS hunk records to copy executable or data hunks and apply relocation offsets.
- `relocate_loaded_hunks_2`: Copies hunk data and applies relocation records for a sequence of AmigaDOS hunks.
- `decompress_backwards_stream`: Unpacks a compressed block by reading its bit stream backwards and writing decoded bytes from the end of the output area toward its start.
- `process_bitstream_literals`: Decodes a bit-stream or compressed data block by processing control-bit-driven instructions, including literal-run expansion and back-referencing or copying previously written data.
- `continue_backwards_decompression`: Continues the backwards bit-stream decoding by handling literal runs and back-references and writing output downward.
- `copy_distance_classed_runs`: Decodes further backwards-stream output for the decompressor, copying runs from earlier output in three distance classes that use bit widths of 9, 5 and 14, and emitting literals in between.
- `copy_distance_classed_runs_2`: Continues the backwards decompressor by copying runs from earlier output in distance classes, writing literal output downward.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a7)+ to d1, then returns.
- `start_disk_read_operation`: Configures the disk controller for a read: it writes the fixed sync word 17545 to the disk sync register, enables disk DMA and interrupt bits, and sets drive select bits through the CIA port.
- `control_disk_drive_lines`: Writes a sequence of bit patterns to a CIA port to select a drive and set its control lines, with a counter that decrements on each pass.
- `pulse_disk_drive_step_lines`: Drives the disk drive control port through a sequence of clear and set writes, separated by calls to other routines, and waits on a status bit before finishing.
- `decode_mfm_sector_data`: Selects a drive through a CIA port and decodes encoded disk data by recombining odd and even bit fields of each longword pair.
- `set_disk_dma_length`: Computes a transfer length from its argument, stores it with a sector count in the disk length register, sets drive control bits through a CIA port, and calls two other routines to prepare the transfer.
- `read_disk_words_into_buffer`: Sets drive control bits through a CIA port, then copies 16-bit words from the disk data register into a buffer while calling three other routines.
- `decode_mfm_buffer_pairs`: Decodes buffered pairs of encoded longwords by combining their odd and even bit fields, and sets a control bit on the CIA port.
- `poll_disk_controller_bounded`: Runs a loop bounded at 1000 iterations over the disk data register, writing two fixed values to the disk controller and setting CIA control bits.
- `decode_mfm_track_sectors`: Decodes a track of encoded data by recombining odd and even bit fields from a buffer and placing each decoded block at an offset scaled by 1032 bytes.
- `process_disk_track_data`: A large block of disk-handling code of 259 instructions that the decompiler only partly recovers.
- `advance_disk_buffer_position`: Adjusts a global offset by a fixed amount and restores it around two calls to other routines, while advancing a read position in the disk buffer.
- `disable_disk_dma_interrupts`: Writes disable masks to the interrupt and DMA control registers and adjusts a global offset around three calls to other routines.
- `reset_disk_controller_state`: Writes fixed values to the disk control and interrupt mask registers, clearing a global flag first when it is set and then writing further mask values.
- `set_drive_select_bits`: Writes a drive select bit pattern to a CIA port, adjusts a counter around two calls to other routines, and then clears the same port bits.
- `call_single_unnamed_routine_2`: Pseudo-code is unavailable because the decompiler timed out.
- `wait_disk_ready_then_read`: Waits for a status bit in the disk port to be set, toggles drive control bits through the CIA port, stores a buffer pointer in the device block, and reads from the disk data register.
- `enter_disk_read_block`: A two-instruction entry point that makes one call to another routine.
- `reset_disk_status_bytes`: Clears the high bit of one status byte and zeroes three other status bytes at fixed offsets from a base pointer, then loops until a 16-bit value built from two of those bytes meets its condition.
- `save_and_disable_interrupts`: Writes the mask 32767 to the interrupt enable register to disable all interrupts, saves the interrupt enable register value into a global, and then writes the mask 32239 with bit 15 clear to clear the listed DMA bits.
- `restore_saved_interrupts_and_dma`: When its saved flag is set, clears the flag, disables the DMA and interrupt bits, restores saved register values and sets their enable bit.
- `build_copper_display_list`: Builds a copper list in memory by writing display control and pointer values, then sets the copper list location from a computed address.
- `build_copper_display_list_2`: Builds an alternate copper list with different display control values, using the same waiting and pointer steps as the other copper list builder.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_copper_and_interrupts`: Pseudo-code is unavailable because the decompiler timed out.
- `install_default_interrupt_vectors_2`: Writes the mask 32767 to the interrupt and DMA control registers to disable them, clears the first longword, and fills the low vector table with a shared routine address.
- `install_default_interrupt_vectors_3`: Repeats the interrupt-disable and vector-table fill steps, then calls seven other routines.
- `return_without_action_2`: A single-instruction routine with no visible work, so it returns immediately.
- `set_interrupt_register`: Stores a constant value into a global variable and modifies the interrupt request register.
- `write_cia_interrupt_control`: Writes the value 8 to a CIA interrupt control location and also touches the interrupt request register.
- `write_interrupt_request_value_2`: Writes the value 112 to the interrupt request register.
- `write_interrupt_request_value_3`: Writes the value 1920 to the interrupt request register.
- `write_interrupt_request_value_4`: Writes the value 6144 to the interrupt request register.
- `write_interrupt_request_value_5`: Writes the value 8192 to the interrupt request register and also touches the second CIA interrupt control location.
- `relocate_loaded_hunks_3`: Walks hunk records, copying hunk data and adding relocation offsets for each relocation record.
- `decompress_backwards_stream_2`: Starts a backwards bit-stream decompression pass by reading the stream position and a table of codes, then dispatches to the decoding steps.
- `emit_literal_bytes_backwards`: Copies literal bytes from the bit stream into the output, writing downward from the end of the buffer.
- `copy_single_or_double_run`: Decodes a one-bit length code from the backwards bit stream and copies a run of one or two bytes from earlier output.
- `copy_short_run_from_stream`: Decodes a two-bit length code and copies a run of three to six bytes from earlier output.
- `copy_medium_run_from_stream`: Decodes a four-bit length code and copies a run of seven to 22 bytes from earlier output.
- `copy_long_run_from_stream`: Decodes an eight-bit length code and copies a run of 23 to 278 bytes from earlier output.
- `copy_distance_classed_runs_3`: Copies runs from earlier output using a fourteen-bit distance class, with branches for several run length classes.
- `copy_nine_bit_distance_run`: Copies a run from earlier output using a nine-bit distance field, writing downward into the output.
- `copy_five_bit_distance_run`: Copies a run from earlier output using a five-bit distance field, writing downward into the output.
- `copy_fourteen_bit_distance_run`: Copies a run from earlier output using a fourteen-bit distance field, with the source offset built from the bit stream.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_audio_playback`: Sets up the sound playback state from a data block, initialises counters, and calls one other routine.
- `silence_audio_channels`: Sets the volume of all four audio channels to zero, writes a fixed value to the audio control state, and clears one bit of a global flag.
- `scale_channel_volumes`: Scales a sample volume by per-channel factors using a multiply and a shift by six, then stores the results in the channel records and sets their counters.
- `install_audio_callback`: Stores a routine address in a callback field of a structure, sets a counter to 25, clears a count, and points the audio channel 0 location register at a buffer.
- `update_channel_period`: Subtracts a per-step value from a channel counter, resets it to 113 when needed, and looks up a period value from a global table indexed by a nibble of the channel state.
- `advance_channel_pointer`: Advances a channel pointer by a fixed offset, decrements a counter, and busy-waits while a status word is zero before updating the channel period.
- `update_channel_playback_state`: Copies pointer and length fields between channel state records and sets the CIA timer control register.
- `lookup_channel_frame_word`: Stores a byte from the stream into a channel record, then uses a masked byte as a table index and stores the looked-up word in the record.
- `lookup_channel_frame_word_2`: Uses a masked byte from the stream as a table index and, when that byte is nonzero, loads a word from the table into the channel record.
- `set_channel_note_fields`: Stores a byte from the stream into two channel fields, one of them decremented by one, and looks up a table word from a masked index.
- `reset_audio_timer_state`: Copies a global value, writes 8192 to a global standing in for the interrupt request register, and sets a counter to 25.
- `reset_audio_interrupt_state`: Writes 8192 to a global standing in for the interrupt request register and touches the audio channel 0 location register and the second CIA interrupt control register.
- `setup_display_and_copper`: Disables interrupts, sets the copper list pointer and display control registers, and loads values from data before calling several other routines, including one at a fixed address.
- `tiny_ori_and`: Small routine of 2 instructions that merges 9 into d0, then masks (a4)+ with d2.
- `stub_bad_instruction_data_2`: Only two instructions decode here, and the store sequence that follows does not match a real routine, so this is bad instruction data overlapping the start of the next routine rather than usable behaviour.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 28 into d7.
- `setup_blitter_and_sprites`: Sets up blitter registers for masks, modulo, pointers, control and size, then sets the pointers for four sprite channels and two colour registers.
- `generate_sprite_attribute_table`: calculates and stores a sequence of sprite or blitter attribute or mask data into a destination buffer, deriving certain bit-fields from a source table or pointer-based data.
- `clear_d0_byte`: Performs a bitwise OR operation on register d0 with zero.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d1.
- `clear_d1_byte`: Performs a bitwise OR operation on register d1 with zero.
- `transform_data_table`: Performs a series of arithmetic and bitwise operations on data from two source pointers to populate a target buffer.
- `transform_data_table_2`: Reads values from a source table and uses them to construct new words in a destination table through bit shifting.
- `build_sprite_control_words`: Continues the sprite control builder, filling further blocks with horizontal positions taken modulo 320 and offset by 128.
- `configure_blitter_registers`: Configures blitter registers for the first-word mask, control and destination modulo, and writes the interrupt enable register.
- `clear_data_d0`: Performs a logical OR operation on register d0 with zero, effectively ensuring the register contains zero.
- `advance_table_index_counter`: Updates the global counters in the same way as its neighbour, computes a table address from a stored value, and runs a 127-step loop that calls one other routine.
- `build_offset_lookup_table`: Fills a table of 1024 entries with running offsets, each one advanced by twice a value read from a second table.
- `call_four_routines_in_sequence`: Calls another routine four times in sequence and returns.
- `mix_global_with_constants`: XORs a global value with several fixed words, rotates the result by a count taken from the global, and stores it through the pointer argument.
- `setup_copper_list_and_sprites`: Waits until a flag is set, clears several global values, and writes the copper list pointer, display control, interrupt and DMA registers, and the sprite registers.
- `start_blit_after_wait`: Waits for the blitter to become idle, then writes its modulo, destination pointer, control and size values, which starts a blit.
- `compute_scaled_division_table`: Fills an eight-entry table, each entry being a global value shifted left by 10 bits and divided by a stored divisor.
- `start_blit_with_control_word`: Waits for the blitter to become idle, then writes a control word built from a shifted value combined with a fixed low part of 3066.
- `setup_full_blit_operation`: Waits for the blitter, then configures its masks, modulos, source and destination pointers, control words and size for a blit, and calls two other routines.
- `scan_table_after_raster_wait`: Waits until a global value reaches a fixed level, then walks a table of 16-bit entries while reading the beam position register.
- `scan_table_after_raster_wait_2`: Variant of the raster-wait table scan that starts from a different initial count, reading the beam position register in the same way.
- `compute_packed_position_word`: Computes a scaled value from a global offset by subtracting 5000 and dividing by 200 against a base of 3200, then packs two derived fields into a word stored in a global.
- `accumulate_with_carry_copy`: Stores a pointer argument, copies a longword between two pointers, and accumulates a value with carry from a global over a loop count of 103.
- `run_blit_accumulation_loop`: Waits for the blitter, configures its full register set, and runs a 128-step loop that adds values with carry.
- `stub_bad_instruction_data_3`: Decodes as bad instruction data, so the pseudo-code stops at once and the bytes do not form a usable routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_without_action_3`: A single-instruction routine in another code segment with no visible work, so it returns immediately.

### logged disk load from Disk.1

- `execute_unnamed_routine`: This routine contains six instructions and is entered from outside the static code flow.
