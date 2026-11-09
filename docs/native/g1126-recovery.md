# Game 1126 (Wizkid v1.0 1449): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01126). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4901334 top-level bytes in 121 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 38 raw-word lines |
| Functions named and described | pass, 390 of 390 named (0 author symbols), 390 of 390 described (15 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2158 of 2159 (99.95%) |
| Recovered without load address (counted in proof 1, flagged) | 432512 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5701 of 6416 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1643760 |
| code | 27064 |
| data | 3230469 |
| padding | 41 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 301 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3572 | 204 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1639 | 111 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 43 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 797 | 42 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| astcode (raw code file, load address unknown) | 1 | 50 | 1 |
| wizkiddemo5e (raw code file, load address unknown) | 1 | 1 | 1 |
| wizkiddemo80 (raw code file, load address unknown) | 1 | 1 | 1 |
| wizkiddemoc0 (raw code file, load address unknown) | 1 | 1 | 1 |

## Cases handled

- Wizkid_v1.0_1449/Wizkid_v1.0_1449.hdf: hard-disk image (AmigaDOS OFS, 5408 blocks): its filesystem parses and all 113 files in it are byte-identical to the extracted tree Wizkid_v1.0_1449_hdf, which is recovered file by file; no block outside the filesystem holds data
- Wizkid_v1.0_1449/Wizkid_v1.0_1449_hdf/Wizkid/WK_DATA/keydisk: high-entropy data file (7.76 bits/byte), classified as packed data
- Wizkid_v1.0_1449/Wizkid_v1.0_1449_hdf/Wizkid/WK_DATA/ldata10: high-entropy data file (7.80 bits/byte), classified as packed data
- 4 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 932 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 503 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 25 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 490 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Wizkid_v1.0_1449/Wizkid_v1.0_1449_hdf/Wizkid/WK_DATA/astcode: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Wizkid_v1.0_1449/Wizkid_v1.0_1449_hdf/Wizkid/WK_DATA/wizkiddemo5e: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Wizkid_v1.0_1449/Wizkid_v1.0_1449_hdf/Wizkid/WK_DATA/wizkiddemo80: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Wizkid_v1.0_1449/Wizkid_v1.0_1449_hdf/Wizkid/WK_DATA/wizkiddemoc0: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 4 never-loaded region(s), 432512 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `load_sound_samples_and_clear_flag` (runtime segment an address): Calls a status check, conditionally triggers two sound routines, then copies many sample blocks into sound memory and clears a word flag before returning. Kind: disk/track check. Pass path: the branch where (uVar1 & 8) != 0 (skipping the subsequent function calls).
- `start_disk_dma_read_with_sync` (runtime segment an address): Checks the disk-present bit, programs ADKCON, DSKSYNC, DSKPTH, DSKLEN and DMACON, then waits for the disk interrupt and DMA completion before returning the status. Kind: disk track/sync check. Pass path: the branch following the call where the code continues execution (e.g., the instructions following the bsr.w calls in the provided call sites).
- `start_disk_dma_read_and_wait` (runtime segment an address): Calls two setup routines, programs ADKCON, DSKSYNC, DSKLEN, DSKPTH and DMACON for a disk read, polls INTREQR bit 1 until done, then clears DSKLEN and returns 1 or 0. Kind: disk track/sync check. Pass path: the instruction following the call (e.g., bne.b an address or lea.l an address(a4), a0).
- `build_ciab_prb_from_flags` (runtime segment an address): Combines bit 0 of one global, a second global word and a third global word into a byte and writes it to CIAB port B at the fixed hardware address. Kind: disk hardware/flag validation. Pass path: sub_22EA (returns via rts, continuing to subsequent instructions).
- `build_ciab_prb_from_state_bits` (runtime segment an address): Reads three global state words, combines their low bits into d2, subtracts one and writes the low byte to CIAB port B. Kind: disk hardware/CIAB_PRB check. Pass path: the instruction following the call (the code continues execution normally).
- `toggle_ciab_disk_select_lines` (runtime segment an address): Writes control values to CIAB port B to pulse disk select lines, stores a value, clears a bit indexed by a counter, then calls a helper. Kind: disk hardware/CIAB_PRB check. Pass path: the instruction following the call (the code continues execution normally).
- `conditional_floppy_deselect_sequence` (runtime segment an address): Tests a status word and, if zero, writes control bytes to CIAB port B with delay calls between them, then sets a flag byte and returns. Kind: disk hardware/track check. Pass path: the instructions immediately following the call sites (e.g., clr.w d0, rts, or movea.l an address.l).

## Functions


### game.slave

- `return_minus_one_at_entry`: Loads the immediate value minus one into d0 and returns immediately, providing a constant return value to the program entry caller.
- `install_code_and_jump_to_loader`: Stores the caller pointer, copies a block to address an address, patches jump tables and vectors in that block, optionally waits on the vertical position register while writing color zero, then jumps to the patched code.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `relocate_stack_words_to_vector_table`: Saves all registers, writes a constant to the stack and to vector address 96, then subtracts a doubling value from stack words and stores results into the stack area before restoring registers.
- `patch_vectors_and_check_magic`: Saves all registers, copies four bytes from a0 into a nearby buffer, calls a subroutine, and if the copied bytes equal the magic value 'astc' writes jump instructions and a branch into the table at a1 before returning d1 and zero d0.
- `inline_magic_bytes_data`: Two bytes of data decoded as an ori.b immediate and a nop; these bytes are data, not executable code, and are read by the caller as a magic value.
- `allocate_and_fill_96_byte_block`: Saves registers, stores a1 and a2 into a nearby buffer, calls a subroutine with a1 set to the high memory address, and if a2 is zero fills 24 longwords from a source table into a2 before restoring registers and returning.
- `zero_filled_data_words_at_0290`: Two zero words that decode as ori.b immediate zero on d0; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_0298`: A single zero word decoding as ori.b immediate zero on d0; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_029c`: One zero word decoding as ori.b immediate zero on d0; this is zero-filled data referenced by address from two callers, not executable code.
- `decode_ilbm_body_into_bitmap`: Parses an IFF FORM ILBM file, reads BMHD dimensions and CMAP, then RLE-decodes the BODY chunk into the bitmap buffer at a1, tracking row and plane counters in the state block at a6.
- `unpack_planar_bitmap_rows_to_chunky`: Reads a row count from the chunk header, divides by three, then converts each group of three planar bytes into chunky pixels by taking the high nibbles and packing them into words at the buffer pointed to by the saved pointer.
- `zero_filled_data_table`: This range is a data table of zero words that other code references by address; the ori.b instructions are decoded data, not executable code, and nothing is cleared or called.
- `zero_filled_data_table_0460`: Twenty-four zero words decoded as ori.b #0 to d0, a data table referenced by address, not executable code.
- `zero_word_data_slot_04c0`: A single zero word decoded as ori.b #0 to d0, a data slot referenced by address rather than working code.
- `zero_word_data_slot_04c4`: One zero word decoded as ori.b #0 to d0, a data slot pointed to by address, not an executable routine.
- `call_vector_offset_eight_store_result`: Saves d0-d1 and a0-a2, loads a pointer from a nearby longword, calls the routine at offset eight of that pointer, stores the returned d0 into a nearby longword, then restores registers and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_absolute_runtime_entry`: Performs a single jump to an absolute address in the runtime segment, transferring execution to the main startup code without saving registers or returning.
- `call_absolute_sub_and_return_d0_in_d7`: Calls an absolute routine, then moves the low byte of the returned d0 into d7 after clearing d7, restores d0-d1 and a0 from the stack and returns.
- `set_d0_low_byte_bits`: ORs the immediate value 141 into the low byte of data register d0, setting several bits, then falls through to the following code.
- `call_91880_then_branch_on_zero`: Calls the routine at absolute address 91880, then branches to a nearby target if the resulting condition codes indicate not equal.
- `poll_status_and_dispatch_handlers`: Copies a status word to a shadow location, saves registers, calls a test routine, stores d1 into a pointer slot, then loops comparing status words and calling two handler routines until they match.
- `save_counter_call_until_changed`: Copies the word at a fixed address to a fixed address, saves d0-d7/a0-a4, calls the routine at a fixed address, and if it returns zero stores d1 at 34 and returns; otherwise it restores registers, calls a fixed address, compares the two counter words, and calls a fixed address or a fixed address before returning.
- `call_far_routine_then_branch`: Calls the routine at a fixed address, then branches to the code at 256 without returning to this point.
- `save_counter_call_with_a0_a2`: Copies the word at a fixed address to a fixed address, saves a0-a2, calls a fixed address, restores a0-a2, and if the result is zero returns; otherwise it calls a fixed address, compares the counter words, and calls a fixed address or a fixed address before returning.
- `poll_flag_and_dispatch_two_handlers`: Copies the word at a fixed address to a fixed address, saves a0-a2, calls the routine at a fixed address, and if its zero flag is clear calls a fixed address; then compares the two words and calls a fixed address if equal, otherwise a fixed address, before returning.
- `wizkid_data_table_zeros`: The bytes decode as repeated zero-immediate instructions, so this is a data table of zero bytes, not executable code.
- `init_pointer_state_and_call_setup`: Loads a3 with a fixed address and a4 with a fixed address, clears the word at a fixed address, stores a0 at a fixed address, calls 5270, then stores a3 and a4 into a fixed address, a fixed address and a fixed address and copies a fixed address to a fixed address.
- `save_regs_call_three_subs`: Pushes d0-d7 and a0-a4 onto the stack, calls three subroutines in sequence, then restores those registers and returns.
- `call_sub_then_jump_into_1468`: Calls the subroutine at the given target, then branches unconditionally to the code at the later address, continuing execution there.
- `update_cached_pointer_and_saved_regs`: Calls a subroutine, compares a0 with the cached pointer at a fixed longword; if different, stores a0, saves a3 and a4, copies a word, calls another subroutine and stores a3 and a4.
- `parse_and_dispatch_script_commands`: Walks a command stream at a fixed table, storing command words and pointers in globals, matching bytes against a table, calling two dispatch routines, then filling a word array with sequential indices until the terminator.
- `compare_slot_and_flush_pending_entries`: Compares the word at a fixed global with a stored value, and if they differ walks a pointer list at another global, calling a routine for each non-null entry until the list ends.
- `store_slot_and_dispatch_indexed_entry`: Copies the current word into a saved slot, scales it by four to index a pointer table, and if the entry is non-null calls two routines on its first pointer, branching on the result.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `select_sound_or_panel_pointer_by_index`: Reads a word index from a global, subtracts two, and returns in a0 a pointer chosen from three tables depending on whether the index is negative, zero, or positive, with a special case for value 123.
- `set_d4_all_bits_high`: Sets every bit of the low word of d4 to one with an ori immediate, then falls through into the following routine.
- `copy_disk_prompt_and_wave_tiles_to_chip`: Copies prompt and wave data into chip memory, duplicates a small word table with gaps, copies three blocks of tile data, clears a word, and copies a sixteen-long block to another buffer.
- `load_pop1_sample_to_channel`: Loads the pop1 sample data into audio channel memory at 60f00 with length -1 and period 8, then returns.
- `load_sound_samples_and_clear_flag`: Calls a status check, conditionally triggers two sound routines, then copies many sample blocks into sound memory and clears a word flag before returning.
- `load_tile_and_map_graphics`: Copies several graphics blocks into chip memory, then fills two tile buffers with repeated word patterns using a copy loop before returning.
- `copy_tile_and_sprite_graphics_to_chip_ram`: Calls a setup routine, then copies three blocks of longword tile data and three blocks of word sprite data from ROM into chip RAM destinations, advancing source and destination pointers between rows.
- `request_audio_channel_one_playback`: Loads d1 with 1 and d0 with 36, then jumps to the audio routine to start playback on channel one with that sound number.
- `request_audio_channel_zero_playback`: Loads d1 with 0 and d0 with 36, then jumps to the audio routine to start playback on channel zero with that sound number.
- `gather_eight_planar_bytes_to_a1`: Copies eight bytes from a0 at offsets 0, 40, 80, 120, 160, 200, 240 and 280 into consecutive bytes at a1, gathering one byte per plane.
- `init_game_state_and_install_vectors`: Sets supervisor mode, resets the stack, runs startup calls, writes CIA control registers, stores a vector table pointer and fills nine exception vectors with the same address.
- `clear_game_arrays_and_reset_counters`: Calls a setup routine, sets two a6 words to maximum, zeroes two large memory blocks and a table of entries, then resets counters and calls further initialisation routines.
- `clear_sprite_blocks_and_init_display`: Clears a series of word entries spaced 608 bytes apart in memory, then calls several setup routines, invokes two absolute library jumps, zeroes two display flags and calls another setup routine.
- `set_display_flag_and_jump_to_main`: Writes 255 to a display flag, copies a word from one global to another, calls a library routine and jumps to the main loop entry.
- `read_and_toggle_display_mode_word`: Calls two library routines, stores the returned word and zero to globals, copies the word to another global and toggles its low bit before returning.
- `stage_intro_sequence_runner`: Sets screen mode, runs the stage intro text screens, waits for space or fire input, then sets up and runs the main stage loop until completion.
- `keydisk_select_menu_loop`: Draws the keydisk selection screen, polls joystick bits to move the selection between entries, and returns the chosen index in d0.
- `store_pointer_and_advance`: Copies the word at a0 into memory location 83616, increments a0 by two, and stores the advanced pointer at 83618.
- `run_task_list_from_pointer`: Loads the current task list pointer, decrements a count word, and when it reaches zero scans past a zero longword to read a new count and pointer, then calls each non-null function pointer in turn.
- `set_d0_one_and_jump_7ff4`: Sets d0 to 1 and branches unconditionally to the routine at offset 7ff4, transferring control without returning to the caller.
- `scan_structures_for_zero_word`: Walks a1 forward by 44 bytes checking the word at offset 3702 for zero, returning d0 with bit 77 set if found, or 1 if the end address is reached.
- `scan_entity_records_until_end`: Walks a1 through 44-byte records from a5, and for each record whose word at offset 3702 is non-negative calls the update routine, stopping when a1 reaches the end address.
- `advance_entity_position_and_check_bounds`: Adds the velocity words at offsets 3720 and 3724 to the position words at offsets 3706 and 3710, loads them into d3 and d4, and if d4 is below 246 calls the routine at 8dbc8 with d0 set to 737.
- `clear_entity_counter_word`: Sets the word at offset 3702 from a1 to zero and returns to the caller.
- `advance_sprite_frame_and_dispatch`: Reads word at offset 3706 into d3 and 3710 into d4, increments the frame counter at 3716, and if the shifted frame index is below 7 adds the base at 3720 and jumps to the routine at the absolute address when the result is at least 12.
- `clear_sprite_frame_counter`: Clears the word at offset 3716 of the structure pointed to by a1 and returns immediately to the caller.
- `init_three_sprite_slots`: Starting from a5, fills three consecutive 30-byte sprite slots with enable flag 1, type values 1, 3 and 4, image indices 412, 421 and 430, and pointers to three data blocks, then returns.
- `reset_and_count_active_slots`: Clears the word at 83ae8, walks 30-byte records from a5+105a to 831d2, increments the counter for each nonzero word, calls the per-record routine, then clears the word at 83b88.
- `advance_sprite_frame_and_draw`: If the 83b88 flag is set, stores frame constants, sets the animation pointer, tests a random value and picks a frame table; then steps the frame counter, reads the next nonzero frame word, and calls the draw routine with d3 and d4.
- `zero_filled_data_word`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `randomize_sprite_frame_and_position`: Decrements a frame counter; when it expires, calls a random routine to pick a new frame index and a new x position, then calls the draw routine with fixed y values in d3 and d4.
- `demo_timer_and_screen_setup`: If a flag byte is set, counts down a timer and on expiry resets it, calls a routine, and if a word is zero calls another; then clears a screen buffer and sets up two display structures via library calls.
- `copy_54d_bytes_to_low_memory`: Copies 1357 bytes from the address in a0 to the address in a1, incrementing both pointers, then returns.
- `seed_slot_index_from_timer_low_nibble`: Tests the word at a fixed address; if non-negative, reads it, masks the low nibble, subtracts three and stores the result in the slot index at a fixed address, then branches to the shared exit.
- `advance_slot_index_until_free_entry`: Increments the word at a fixed address, wraps it to zero when it reaches nine, then indexes the pointer at a fixed address by twice the index and repeats while that word is negative.
- `reset_slot_index_and_advance_pointer`: If the word at 29502 equals six, writes a digit character to 19729, sets a0 to 19724, copies the pointer at 3048 to 19318, calls the allocator, then advances the pointer at 3048 by an even amount.
- `copy_palette_byte_by_index`: Reads a word index from memory, uses it to fetch a byte from a table via a0, and stores that byte to a fixed memory location before returning.
- `write_date_bytes_to_custom_registers`: Copies a word and five bytes from a date table into consecutive custom chip registers starting at a1416, then returns.
- `jump_to_initialization_sequence`: Unconditionally jumps to the main initialization code at a fixed address, transferring control without saving state.
- `wizkid_level_intro_and_play_loop`: Sets up display and sound registers, clears level state, then runs the main play loop polling input and timers until the level ends.
- `draw_sprite_at_offset_from_base`: Calls the setup routine, and unless d3 equals minus one, loads the sprite base pointer, adds the d5 offset into a2 and calls the draw routine.
- `wait_for_input_or_timeout_then_advance`: Sets a countdown, repeatedly calls update routines and checks input flags, then on input or timeout sets the advance flag and jumps back into the main loop.
- `set_d0_low_byte_bits_2`: ORs the immediate value 9 into the low byte of d0, setting bits 0 and 3, then falls through to the following code.
- `zero_filled_data_area`: The word decodes as ORing zero into d0's low byte; this address is a zero-filled data area referenced by other code, not executable logic.
- `copy_indexed_bytes_and_words`: Pushes byte at offset 51 from a1 plus d3 onto the stack, loads word at offset 55 from a5 plus d3 into d2, and loads d4 from a long absolute address.
- `init_input_buffer_and_clear_count`: Stores d3, d4 and d0 into three consecutive bytes at a fixed address, clears the word at a fixed address, then fills the buffer at a fixed address with spaces for the count in a fixed address and appends two 255 bytes.
- `process_keyboard_input_and_update_cursor`: Reads the key code at 6668(a5), appends printable characters to the input buffer, handles backspace and enter, blinks the cursor via the system call, and runs the main game update loop.
- `run_title_screen_countdown_loop`: Initialises the countdown and screen parameters, repeatedly calls the update routines, checks joystick input and decrements the timer until it expires or fire is pressed.
- `countdown_300_calls_six_update_routines`: Stores 300 in the countdown word at a fixed address, then repeatedly calls six unnamed update routines and decrements that word until it reaches zero, finally clearing the word at a fixed address and returning.
- `zero_filled_data_area_2`: This single zero word decodes as ori.b #0,d0; it is a zero-filled data area referenced by address, not executable code.
- `poll_joystick_bits_adjust_menu_counter`: Checks the word at 23728 and returns if nonzero; otherwise tests bits 2 and 3 of the byte at offset 6657 in a5, setting the 22730 timer to 200 and incrementing or decrementing the counter at a fixed address within limits 0 to 6.
- `wizmap_enter_play_loop`: Checks two flags, sets up the wizmap display via a library call, copies 31 words of map data, then runs the main play loop calling many subroutines until the timer or exit flag ends it and jumps back to the outer loop.
- `count_players_and_insert_highscore`: Counts nonzero words in the player table, adds a pending bonus, scans seven highscore entries comparing score and time, shifts entries down to insert the new one, copies the name and returns the rank or minus one.
- `set_player_name_and_show_entry`: Writes the player index and a character into the name buffer, computes the highscore entry address from the index, calls the entry setup and displays it via a library call.
- `draw_seven_highscore_rows`: Writes plus and digit characters to the score display buffer, then loops seven times calling the row formatter and a display routine while advancing the buffer pointer by thirty-six bytes.
- `format_score_row_from_record`: Selects a digit string based on the record type, copies nine bytes into the score buffer, optionally inserts a space and rank digit, converts the score value and copies nine name bytes.
- `show_selected_highscore_entry`: Copies the selected record's nine words into the display buffer, formats its row, sets the rank digit, draws a panel and refreshes the display.
- `poll_input_flags_and_dispatch`: Calls the input-flag routine, then tests bits of the word at offset 6652 in a5 and branches to other routines based on the low nibble and the word at offset 6656.
- `map_input_nibble_to_byte`: Reads the low nibble of the word at offset 6652 in a5, indexes the byte table at 8585a, and stores the byte at 8586b unless it equals 15.
- `reset_sound_and_set_mode`: If the word at 85cb0 is set, writes 4 to 87632, calls the sound routine, clears the long at 80bf4; then sets 80bdc to 9f40, 86026 to 2, 87632 to 123 and calls another routine.
- `data_table_after_ori_movep`: The bytes decode as an ori immediate, a movep long store and another ori immediate, so this is data rather than executable code.
- `advance_scroll_offset_and_redraw`: Adds a signed step to a long scroll offset, clamps the resulting word against a limit, adjusts the step, then calls the draw routine with width, height and offset values.
- `run_four_subsystems_and_copy_pointer`: Calls four subroutines in sequence, then copies the long value at one fixed memory location to the next location and returns.
- `init_sound_and_copy_pointer`: Calls four setup routines, then copies the longword at address 8227A to address 8227E and returns.
- `reset_sound_state_and_restart`: Loads pointer from 8227E, writes 12345 to it, copies longword 8227A to 82276, calls two routines and a sound routine, then returns.
- `jump_to_sound_driver_entry`: Calls the routine at address 8D404 and returns to its caller.
- `double_pulse_59ec_around_59ac`: Calls the routine at 59ec, then the routine at 59ac, then 59ec again, and returns to its caller.
- `set_and_test_flag_at_860ce`: Writes the value 1 to the longword at 860ce, then tests that same longword and branches back if it is nonzero, otherwise returns.
- `clear_10000_bytes_at_9f40`: Loads address 9f40 into a0, sets d0 to 9999 and d1 to zero, then stores zero longwords through a0 with post-increment until the count expires, then returns.
- `clear_10000_byte_buffer_at_a5_offset`: Loads the buffer pointer from offset 6636 of a5 into a0, then writes zero longwords to 10000 successive positions, clearing that buffer.
- `copy_rom_table_to_a5_buffer`: Copies 10000 longwords from the fixed source at address 40768 into the buffer pointed to by offset 6636 of a5, advancing both pointers.
- `copy_low_memory_to_rom_table`: Copies 10000 longwords from low memory starting at address 768 into the fixed destination at address 40768, advancing both pointers.
- `copy_9999_longs_to_buffer_and_alt`: Copies 9999 longwords from either the alternate source or the a5-referenced pointer into the fixed buffer and the a5 destination, advancing both pointers.
- `copy_9999_longs_from_buffer`: Copies 9999 longwords from the fixed buffer into the destination pointer stored at offset in a5, advancing both pointers.
- `reset_game_state_and_init_subsystems`: Clears several global words and longs, sets sentinel values, then calls a sequence of setup routines and jumps through a stored pointer.
- `zero_filled_data_word_at_5cac`: The single word decodes as ori.b with an immediate zero to d1; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_5cb0`: The single word decodes as ori.b with an immediate zero to d0; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_area_6`: This single zero word decodes as an ori.b instruction but is actually zero-filled data that other code references by address, not executable code.
- `apply_all_sprite_palette_entries`: Walks a table of entries until a terminator, computes each sprite slot from the table's index and base pointer, writes three color bytes into the sprite structure, calls the color update routine, then repeats via a tail call.
- `handle_cursor_input_and_redraw_sprite`: Checks joystick bits in a5+6657 to move the selected sprite index up or down, then adjusts the frame byte of the current sprite entry within its min/max bounds and redraws it via the blitter.
- `apply_selected_sprite_palette_entry`: Uses the selected index to pick one table entry, computes its sprite slot, writes three color bytes into the sprite structure, jumps to the color update routine, then compares the entry pointer against the zero data address.
- `apply_palette_entry_to_screen`: Reads a palette index, scales it by fourteen, loads the entry's colour bytes and writes three bytes into the screen buffer at the offset stored in the base pointer.
- `set_frame_ready_flag`: Stores the immediate value one into the longword at the frame-ready flag address and then falls through into the following routine.
- `advance_animation_frame_and_setup_display`: Waits on a lock, picks a frame pointer from a table by index, updates the display pointer and bitplane base, sets the display mode word, calls setup routines and arms the frame lock.
- `clear_audio_flag_and_reset_channels`: Writes zero to the word at absolute address a fixed address, then calls the channel reset routine and returns to the caller.
- `read_audio_flag_and_call_tick`: Loads the word at absolute address a fixed address into d0; if nonzero, calls the library routine at a fixed address and returns its result, otherwise returns zero.
- `zero_four_voice_periods_set_volume`: Writes zero to four words at offsets 168, 184, 200 and 216 from a6, sets the word at offset 150 to fifteen, then returns.
- `clear_24_words_at_a6_offset`: Loads the address a6 plus 160 into a0, then writes zero to 24 consecutive words there using a dbra countdown, and returns.
- `zero_filled_data_word_at_6024`: This single zero word decodes as ori.b #0,d0 but is data referenced by address, not executable code.
- `zero_filled_data_word_at_6028`: This single zero word decodes as ori.b #0,d0 but is a data byte referenced by address, not executable code.
- `update_copper_list_and_tick_counters`: Selects one of two copper list pointers based on a flag and vertical blank bit, writes it to the copper list high register, then decrements a countdown and calls several subroutines while updating frame counters.
- `frame_counter_word`: A single word of data referenced by address, decoded as ori.b #1,d3; it is a counter variable, not executable code.
- `zero_filled_data_word_2`: A single zero word referenced by address, decoded as ori.b #0,d0; it is zero-filled data, not executable code.
- `per_frame_update_and_dispatch`: Calls two setup routines, clears pending flags, resets four longwords and a word to a fixed address, sets a value when counters match, then jumps through a table indexed by a global word.
- `set_flag_call_wait_clear`: Writes 1 to the word at a fixed address, calls a subroutine, then spins until that word becomes zero and returns.
- `run_six_subsystem_updates`: Calls six unnamed subroutines and two library functions, copies the longword at a fixed address to a fixed address, then calls another library function and returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `advance_sprite_frame_index`: Increments the word at a fixed address, wraps it to zero after 19, then reads the word from the table at a fixed address indexed by that value into d2 and returns.
- `draw_single_sprite_at_offset`: Uses the word at a fixed address times four to index a table at a fixed address, computes screen coordinates in d3 and d4 from the table entries and d2, then calls the draw routine at a fixed address with d0 set to 383.
- `draw_all_active_sprites`: Reads the frame index at a fixed address, adds 10 with wrap at 19, fetches a value from a fixed address, then loops over 10 entries of the tables at a fixed address and a fixed address, calling the draw routine at a fixed address for each nonzero entry.
- `set_d0_low_byte_and_load_a0_table`: ORs the low byte of d0 with zero, leaving d0 unchanged, then loads the absolute address 40768 into a0 before falling through to the next routine.
- `draw_tiled_metatile_columns`: Uses d0 bit to pick a fill byte, then writes columns of tile bytes into four rows of screen memory at a0, stepping by 40 and 8000 bytes, and returns.
- `refresh_screen_and_clear_if_unset`: Calls six subroutines to update display state, copies the pointer at 8826 to 8830, and if the pointer at 8822 is null writes 12345 there and calls another routine.
- `zero_filled_data_word_at_733a`: A single zero word that decodes as ori.b with immediate zero to d0; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_733e`: A single zero word that decodes as ori.b with immediate zero to d1; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_pair_at_7342`: Two consecutive zero words that decode as ori.b with immediate zero to d0; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_area_7`: Contains only a zero word that decodes as an ori.b instruction, so this address points into zero-filled data rather than executable code.
- `check_timer_and_counter_threshold`: Tests the word at 89cd2 and branches on zero or negative, then compares the word at 8b6aa with 206 and branches away if lower, otherwise returns.
- `test_three_state_words_and_load_d0`: Tests words at 87348 and 8734c, branches if the latter is nonzero, tests 8734e and branches if nonzero, then loads the word at 8734a into d0 and returns.
- `reset_sound_and_clear_flag`: Compares the word at a fixed address with zero; if equal, writes 127 to a fixed address, then clears the word at a fixed address and returns.
- `mask_d0_store_and_clear_counters`: Masks the low seven bits of d0, stores the result at a fixed address, clears the word at a fixed address, then branches to shared code that clears further counters.
- `clear_counter_and_copy_index_plus_one`: Clears the word at a fixed address, compares the word at a fixed address with twelve, and if not equal copies it to a fixed address incremented by one before branching to shared code.
- `reset_sound_channels_and_wait`: Copies two sound channel words to shadow locations, clears both shadow words, loads 40 into d0 and calls the delay routine before returning.
- `set_sound_mode_and_copy_channel_words`: Writes 5 to a sound control word, copies one word to its shadow, mirrors three channel words into shadow locations, then branches to another routine.
- `clear_sound_state_and_jump`: Clears two sound state words, tests a status word and if zero sets a mode word to 1, then branches to another routine.
- `init_sound_channels_and_clear_state`: Copies three sound parameter words into adjacent globals, calls the audio setup routine, clears two state words and requests value 40 via a subroutine before returning.
- `clear_three_a5_words_and_global`: Writes zero to three word fields at offsets 6506, 6548 and 6530 from a5, clears one absolute longword global, then returns.
- `zero_filled_data_area_3`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `scan_entities_for_next_smallest_id`: Walks entity records fourteen bytes apart from a5, finds the smallest nonzero id above the current threshold, stores it in the shared minimum, and calls the follow-up routine until none remain.
- `check_entity_flag_and_load_coords`: Reads word at offset 3054 of a1; if nonzero and equal to the global at a fixed address, loads words from offsets 3040, 3042 and 3044 into d3, d4 and d0 then branches away.
- `data_byte_ori_54_table_entry`: A single ori.b immediate byte pair that is data, not executable code; it is referenced as an address by three callers and returns nothing.
- `check_joystick_threshold_and_set_d0`: Compares the word at memory location a fixed address against 30 and 254, sets d0 to -1 or 1 depending on the comparison and the word at offset 6672 of a5, then returns.
- `init_sound_and_voice_registers`: Reads a word from a fixed address, writes several derived byte and word values into sound-related memory locations, optionally clears two words and a longword when a flag is zero, then indexes a table and calls a sound routine.
- `select_indexed_word_and_notify_change`: Builds an index from two global words, loads a word from a table at that index, and if it differs from the stored value updates two globals and calls a routine with 41 in d0.
- `apply_bitplane_masks_to_tilemap`: Reads a mask and shift count indexed by the word at offset 6480 of a5, then writes masked shifted words into the 32-entry table at 31af4 and into indexed slots of the table at 31ba6.
- `clear_pending_sound_channels`: Scans twelve-byte channel records from a5 to a fixed end, clearing each pending flag and zeroing the byte at the stored offset in a0; the leading ori instructions are data decoded as code.
- `start_sound_channel_if_free`: Calls a helper, checks the global flag and per-index byte, then finds a free twelve-byte channel record, marks it active, stores the index, loads a pointer from the table at 92dac and clears its counter.
- `clear_pending_flag_and_dispatch`: Reads the word at 8807A; if zero returns zero, otherwise clears it, calls the routine at 7FF4 and returns its result.
- `process_active_slots_from_input`: Looks up the word at 7632 in a table, optionally calls 7FF4 and 807C, then walks twelve-byte slots from a5 to 2F7A calling 8138 for each nonzero slot and counting them in 80DA.
- `decrement_slot_timer_or_jump`: Tests the word at offset 15E6 of the slot in a1; if nonzero decrements it and returns, else stores a1 at 178A(a5) and jumps through the pointer at 15E2(a1).
- `copy_9999_longs_to_a5_buffer`: Copies 10000 longwords from the fixed source at address 40768 into the buffer pointer stored at offset 6636 of a5, then returns.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `init_display_and_check_magic_ids`: Sets up a display area, writes 16384 to offset 154 of a6, calls library routines, then compares the returned value against two magic identifiers and branches accordingly.
- `clear_byte_range_and_reset_counter`: Clears bytes from address a fixed address up to 6680 bytes later, then sets the word at a fixed address to zero and returns.
- `random_indexed_table_byte_xor`: Increments a byte counter, reads a word from a table indexed by the doubled counter, then XORs its low byte with the CIA-A interrupt control register and returns the result in d0.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `install_cia_a_interrupt_vector`: Stores the address of an interrupt routine into the level-two autovector, then writes to the CIA-A interrupt control register to clear and re-enable its interrupts.
- `set_global_and_audio_volume`: Writes a fixed longword to absolute address 100 and stores 32767 into offset 156 of a6, then returns to the caller.
- `ciaa_keyboard_serial_interrupt_handler`: Reads the CIAA serial keyboard register, shifts and inverts the byte, stores it in a keyboard buffer, clears the CIAA interrupt, and returns from the interrupt.
- `vblank_interrupt_dispatch_counter`: Saves all registers, points a6 at the custom chip base, increments a frame counter, acknowledges interrupt level 4, calls a routine and returns from the interrupt.
- `wait_for_counter_reaching_80_twice`: Calls a routine, then waits until the word at offset from a5 equals 80, repeating the call and wait a second time before returning.
- `update_selected_sound_command`: Reads a byte from a table indexed by the word at offset 6672 in a5, maps value 63 to the index with bit 7 set, and stores the result at offsets 6670 and 6668 in a5, or clears offset 6668 when unchanged.
- `apply_pending_sound_command`: Scans a table of eight-byte entries for the word matching offset 6668 in a5, then writes the entry's second word through the pointer in its fourth byte pair and calls the follow-up routine.
- `commit_sound_register_change`: If bit 7 of the byte at 8892B is set, extracts the low five bits of the word at 8892A, stores it in 8733E when different from the incremented value, sets 8734E to one and clears 8892A.
- `zero_filled_data_area_8`: This single zero word decodes as an ori.b immediate instruction, so the bytes are data, not executable code, and are referenced by address from other code.
- `fill_five_sprite_entries_from_base`: If the word at the global counter is at least two, stores a fixed value into a5-relative memory, then writes five pairs of high and low words from a running base into consecutive eight-byte slots at a0.
- `ori_byte_80_data_word`: A single word that decodes as ori.b with immediate 128 to d1; it is data referenced by address rather than executable code.
- `toggle_playfield_buffer_and_set_pointer`: Saves the current playfield pointer, calls a setup routine, then swaps the pointer between two fixed buffers and updates the display pointer accordingly.
- `set_sprite_pointer_if_not_already`: Compares the sprite pointer at a fixed memory location with a constant and, if different, writes a new sprite table address into it.
- `conditional_clear_flag_and_call_setup`: Writes a flag word, tests a status word at an offset from a5, clears the flag and calls a setup routine before clearing the flag again.
- `render_sprite_list_to_copper`: Selects a sprite list by index in d0, clamps each entry's x and y coordinates to the screen range, writes copper move instructions to the buffer at a1 and stores list data at a2, stopping at the terminator words.
- `emit_sprite_header_and_palette_word`: Saves a1 in a3, calls the copper emit routine with d3, writes a4 and a packed 12-bit colour word built from d0, d1 and d2 to the copper list, then stores the byte count and colour word to a2.
- `emit_copper_colour_change`: Compares d4 with the last emitted colour, and if different writes a copper move pair setting the colour register to the shifted value, optionally emitting a wait instruction first.
- `poll_joystick_and_fire_buttons`: Reads joystick direction bits from a6 offset 12 and the fire button from CIAA_PRA, stores five boolean flags in a5, builds a bitmask of active inputs and signals changes via a5 offsets 6654 and 6656.
- `setup_sprite_descriptors_from_d3_d7`: Saves and swaps a global, calls a helper, then fills a6 sprite fields with d3, d7 shifted values, constants and three descriptor blocks spaced 1480 bytes apart.
- `set_sound_and_clear_counters`: Writes the value 1 to a longword global, then writes zero to two other global words and returns.
- `zero_word_data_table_entry_1`: A single zero word that decodes as ori.b with immediate zero to d0; it is data, not executable code, referenced by address from other routines.
- `zero_word_data_table_entry_2`: One zero word following the previous entry, decoding as ori.b immediate zero to d0; this is zero-filled data addressed by callers, not real code.
- `zero_word_data_table_entry_3`: Two consecutive zero words decoding as ori.b immediate zero to d0 twice; this is a zero-filled data area referenced by address, not executable code.
- `zero_filled_data_area_9`: A single zero word that decodes as an ori.b instruction; this address is a zero-filled data area referenced by other code, not executable logic.
- `zero_filled_data_word_at_9cdc`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data whose address is taken by clear and test operations.
- `update_timer_and_state_flags`: Tests several state words, decrements a countdown, plays sound via a library call, and sets or clears flags and counters in memory before returning.
- `step_sprite_frame_forward_or_back`: Reads the current frame index, compares it with a limit, and either advances or rewinds the index while calling the sprite draw routine with frame codes.
- `step_clamped_frame_index_with_random`: Clamps a frame index to at most six, then advances or rewinds it, calling a random routine and the sprite draw routine with frame codes.
- `animate_sprite_table_entries`: If the animation flag is set, walks a table of sprite entries, compares each entry with its previous value and calls the draw routine for changed nonzero entries, then clears the flag.
- `copy_sprite_planes_to_shadow_buffer`: Copies 34 blocks of four longwords and one word from the source buffer at a fixed address into interleaved offsets of the destination buffer at a fixed address, then clears 22 words at the table and zeroes a flag word.
- `apply_velocity_to_scroll_offset`: Adds the signed velocity longword to the scroll offset, clamps it between zero and 64, clears the velocity when clamped, and if a mode flag equals one writes half the offset to the hardware register.
- `scroll_offset_word_data`: This single instruction is data decoded as code: the word at this address is a scroll offset variable, not executable code, and is read and written by other routines.
- `copy_two_word_tables_to_globals`: Copies 31 words from one table to a spaced destination, then copies 16 words from a second table to another spaced destination, then returns.
- `claim_free_slot_store_four_words`: Scans 20-byte slots from a5 for a zero flag word, stores d3, d4, d0 and d1 into the slot's four following words, sets the flag to one and returns.
- `update_all_slots_with_two_callbacks`: Walks 20-byte slots from a4 to a fixed end, calling one routine when the first flag word is set and another when the second flag word is set, then returns.
- `set_a4_context_and_test_condition`: Stores a4 into the a5-relative context pointer, clears the word at offset 6030 of a4, calls the test routine, and sets the word at 6048 of a4 to 1 if the test returns zero, then returns.
- `clear_flag_load_sprite_offsets_and_jump`: Clears the word at 6048 of a4, loads a long offset from 6040 of a4 into d5, loads words from 6044 and 6046 into d2 and d3, adds d5 to the a2 pointer from 6636 of a5, and branches away.
- `start_audio_sequence_with_saved_state`: Reads two words from the a4 context, calls a helper, then stores a function pointer, d2 word and a word into the a5 context and calls another helper, setting a flag when a counter word is zero.
- `acknowledge_vblank_and_chain_interrupt`: Writes interrupt acknowledge values to the custom chip interrupt request register, tests the vertical blank flag, clears it when set, then jumps to the next interrupt handler or returns from the exception.
- `acknowledge_audio_interrupt_if_pending`: Tests the audio interrupt flag in memory and, when nonzero, writes the audio interrupt acknowledge value to the custom chip interrupt request register before returning.
- `acknowledge_copper_interrupt_and_set_flag`: Writes the copper interrupt acknowledge value to the custom chip interrupt request register, sets a pending flag in memory, then branches onward to the main interrupt service code.
- `save_stack_switch_and_enter_supervisor`: Pushes all data and address registers onto the stack, saves the stack pointer, switches to the alternate stack, sets a flag if clear, enters supervisor mode and jumps to the main code.
- `restore_stack_and_return_from_exception`: Pops all data and address registers, returns from exception; the second entry saves registers, swaps the stack pointer with the saved one, restores registers and returns from exception.
- `set_flag_once_and_return`: Tests the word flag at the given memory location, and if zero stores d0 there; then returns to the caller.
- `call_62f0_then_jump_a94a`: Calls the routine at offset 62f0, then branches unconditionally to the code at offset a94a without returning here.
- `wait_for_8a948_then_clear_8a946`: Loads the word at absolute address 8a948 into d0; if it is zero, clears the word at 8a946 and branches back to recheck the flag.
- `reset_display_and_dispatch_by_index`: Writes one and zero to two status words, compares d0 with 1234, and otherwise indexes a pointer table by d0 to call a display routine with a1 and a2 set.
- `dispatch_five_jsr_then_loop`: Calls five fixed subroutines in sequence, then branches back to an earlier address to repeat the sequence indefinitely.
- `find_doggie_sprite_and_copy_to_plane`: Scans a list of null-terminated strings for the one matching doggie.s, then copies that entry's 32-bit data to the bitplane at a fixed address, saving and restoring all registers.
- `set_countdown_to_1000`: Writes the immediate value 1000 into the longword at memory location a fixed address and returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `append_audio_command_triple`: Reads the command write pointer, checks whether the next slot already holds the terminator word, and if not stores d0, d3 and d4 as three words, advances the pointer and writes the terminator.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_blitter_busy_then_set_control`: Writes the blitter control word, polls bit 6 of the custom chip status register until clear, then writes the alternate control value and returns.
- `reset_audio_state_and_drain_command_list`: Calls the blitter wait, clears and sets audio control words at offsets in a6, then walks the command list reading d3, d2, a2 and d5 and calling the dispatcher until the terminator word is found.
- `init_sound_channel_state`: Calls a subroutine, then clears word at offset 66 of a6, stores 2544 at offset 64, and sets words at offsets 70 and 68 to minus one.
- `fill_five_sound_slots_from_base`: Calls a subroutine, stores d3 at offsets 100 and 102 of a6, then loops five times writing a1, a2 and d2 into offsets 80, 84 and 88 while advancing a1 and a2 by 8000.
- `init_two_sound_buffers_and_slots`: Calls a subroutine, clears word at offset 66 of a6, stores 2544 at offset 64, sets offsets 70 and 68 to minus one, then loads a0 and a1 from d5-offset bases and stores d3 at offsets 100 and 102.
- `copy_plane_pointers_and_width`: Calls a subroutine, stores a0 and a1 into offsets 80 and 84 of a6 and d2 into offset 88, then advances a0, a1 and a2 by 8000 bytes per d4 iteration.
- `scan_table_for_terminator_3039`: Calls a setup routine, reads words from the table pointed to by the global pointer, and if the first word is not the terminator value it reads d3 and d4 and calls the next routine.
- `test_bit_and_move_peripheral_long`: Tests bit d1 of the longword at offset 0 of a3 and moves the longword at offset 32 of a4 to d0 using movep.
- `toggle_bit_in_predecremented_a4`: Flips the bit selected by d0 in the byte at the address a4 points to after a4 is decremented by one, then falls through to the next routine.
- `queue_sprite_descriptor_and_start_display`: Calls the sprite setup routine, then if d3 is not minus one writes d3, d2, a computed sprite pointer and a marker word into the buffer at the display list pointer and updates that pointer.
- `build_sprite_from_tile_and_clip`: Looks up a tile record from d0, computes source and destination pointers from d3 and d4, clips against the screen bounds, stores the sprite control words and returns the height in d3.
- `init_sprite_slot_tables_and_sort_entities`: Clears nine 608-byte sprite slot blocks at the selected base, stores pointers and counts in globals, zeroes an 18-entry index table, then scans entity records to bucket them by screen row and call the per-entity update.
- `fill_sprite_pointer_table_and_toggle_base`: Writes eight pairs of high and low address words into the table at the sprite pointer area, advancing each entry by 608 bytes, then flips the base-select flag word.
- `base_select_flag_data_word`: Two no-op immediate instructions on d1 and d0; the bytes are a data word used as the sprite base-select flag, not executable code.
- `append_sprite_descriptor_to_buffers`: Reads a sprite slot index, checks space in two buffers, writes colour, height, control and pixel bytes to both, advances pointers and returns one on success.
- `zero_filled_data_area_4`: A single zero word decoded as an instruction; this address is a zero-filled data area referenced by other code, not executable code.
- `sprite_data_byte_constant`: A single immediate byte constant referenced by address from another routine; the bytes here are data, not executable code.
- `set_low_nibble_of_d2_to_f`: ORs the immediate value 240 into the low byte of data register d2, setting its low four bits, then falls through to the next routine.
- `or_zero_into_longword_at_a6`: ORs the immediate long value zero into the longword at address register a6, a no-op that leaves memory unchanged, then falls through.
- `set_d0_bit1_and_subtract_d2_from_a0_word`: ORs the immediate value 2 into the low byte of d0, then subtracts the low word of d2 from the word at address register a0.
- `set_d2_low_byte_bits`: ORs the immediate value 176 into the low byte of data register d2, setting those bits and leaving the rest of d2 unchanged.
- `or_zero_into_a6_longword`: ORs the immediate value zero into the longword at address register a6, which leaves that memory longword unchanged.
- `set_d0_bits_and_subtract_from_d5`: ORs the immediate value 2 into the low byte of d0, then subtracts the byte at address a0 from the low byte of d5.
- `set_d2_low_byte_bits_2`: ORs the immediate value 112 into the low byte of data register d2, setting those bits, then falls through to the following code.
- `or_zero_into_a6_target`: ORs the immediate value zero into the longword at the address held in address register a6, leaving it unchanged, then falls through.
- `set_d0_bit_and_adjust_stack`: Sets bit 1 of the low byte of data register d0, then subtracts the word at the address in a0 from the stack pointer a7.
- `set_d2_low_nibble_to_ascii_zero`: ORs the low byte of d2 with the immediate value 48, forcing the low nibble bits to the ASCII digit zero pattern, then falls through to the next routine.
- `data_table_ori_zero_fill`: A block of data bytes decoded as instructions: repeated OR-immediate zero operations on d0 and one on (a6), with a single byte of all ones; these are data, not executable code.
- `init_sound_channel_and_allocate_buffers`: Calls a helper, clears and sets sound channel fields at offsets in a6, then allocates two buffers via another helper, stores their pointers and a channel index, and returns zero in d0.
- `stop_custom_chips_and_read_vpos`: Clears the custom chip register at offset 256, reads VPOSR into d0 masked and shifted, silences both CIA interrupt registers, then disables all interrupt and DMA bits before returning.
- `init_copper_and_clear_sprite_buffers`: Calls two setup routines, writes copper list pointers and control words into the custom chip base, then zeroes fifteen entries spaced 304 bytes apart in memory starting at a fixed address.
- `blit_tile_rows_into_bitmap_and_or_mask`: Saves registers, computes destination offsets from d0 and d2, stores a descriptor to a3, clears the target area, then copies rows of words from d7 into the bitmap while OR-ing them into a mask, advancing by 8000 bytes per row.
- `cycle_sound_slot_and_call_1328`: Increments and masks a byte counter to four values, saves d0-d4/a0-a4, sets a0 and a1 to minus one, d1 from the counter, d3 to minus one and d0 to zero, calls the routine at 1328, stores d0 in a global word, restores registers and returns when that word is zero, otherwise jumps to the sound call.
- `zero_word_data_at_1002`: This single zero word decodes as an ori.b immediate instruction but is really zero-filled data referenced by address, not executable code.
- `zero_word_data_at_1006`: Another zero word in the same zero-filled data area, reached only through an absolute address taken by three callers, not real code.
- `zero_filled_data_area_5`: A single zero word that decodes as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `copy_ram_header_and_name_bytes`: Writes the four bytes RAMD to address 736, then copies 26 bytes from a source table into consecutive memory after it, finishing with rts.
- `reset_ram_buffer_pointers`: Clears three longwords and a word in high memory, stores two buffer bounds, copies the lower bound to a pointer, then offsets the bounds by ten thousand and returns.
- `find_and_remove_matching_entry`: Saves all registers, checks two global flags, then scans a table of 40-byte entries comparing each against the string in a0; on a match it copies data, shifts later entries down, decrements the entry count and adjusts the size globals.
- `advance_to_next_table_entry`: Adds 40 to a5 to point at the next table entry, increments d1, compares it with the entry count at a fixed global and branches back to the scan loop or exits.
- `remove_entry_and_compact_table`: Uses the matched entry in a5 to copy its data block, then shifts all following 40-byte entries down by one, decrements the entry count and adjusts the used and free size globals.
- `restore_all_registers_and_return`: Compares d0 with itself to set flags, then pops d0-d7 and a0-a6 from the stack and returns to the caller.
- `append_string_entry_to_buffer`: Checks the free-space counter and entry limit, allocates a record via the allocator, copies the null-terminated string from a0 into the record, stores the buffer pointer and length, then updates the free counter, entry count and buffer pointer.
- `find_matching_string_entry`: Scans the record table for a string equal to the one at a0, and on a match copies the record's stored buffer pointer and length into the shared result globals.
- `check_91692_flag_then_jump`: Tests the longword at memory location 91692; if it is nonzero, branches to the routine at 126A, otherwise jumps to the routine at 92142.
- `load_a4_d7_from_globals_then_branch`: Loads a4 from the longword at 91692 and d7 from the longword at 91696, then branches unconditionally to the code at 1318.
- `dispatch_d0_command_and_set_color`: Compares d0 against command values 8, 24, 28 and 32, calling subroutines and copying a block via a0/a1 for some, clearing the 1002 longword for another, and writing d0 to COLOR00 before returning.
- `jump_to_shared_dispatch`: Jumps directly to the shared dispatch routine, passing the current d0 word value through unchanged.
- `init_sound_buffer_and_copy_table`: Loads a sound resource, calls setup routines, writes a marker word then copies 252 bytes from a table into the sound buffer, then finalizes.
- `save_stack_pointer_and_init_sound`: Stores the stack pointer to a global, calls four sound routines with argument 160, clears d0 and returns zero.
- `save_stack_and_init_sound_then_dispatch`: Stores a7 in a global, calls four setup routines with d0 set to 160, clears d0 and returns; the trailing code clears globals and jumps through a table entry indexed by d0.
- `call_two_subs_set_stack_loop`: Calls two subroutines, loads the longword at absolute address into a7 as the stack pointer, then branches back to an earlier address.
- `clear_d0_call_2112_and_return`: Sets d0 to zero, calls the routine at 2112, then returns to its caller.
- `return_zero_word`: Loads zero into d0, tests the low word of d0 and returns, giving callers a zero result.
- `init_level_state_from_params`: Stores a0 and derived pointers into globals, saves d1 and d3 when non-negative, sets up display pointers and counters, calls a setup routine, then checks CIAA_PRA fire bit and returns zero or branches.
- `start_level_after_fire_check`: Clears counters, reads CIAA_PRA bit four into a flag, calls several setup routines, checks the fire button again and returns zero after optional cleanup.
- `set_mode_11_and_return_four`: Calls three subroutines, writes the value 11 to the longword at memory location 927fa, then loads 4 into d0 and returns.
- `wait_for_mode_flag_then_call`: Calls a subroutine, then tests the word at 927fa and branches back while it is nonzero, calls another subroutine and returns.
- `load_sprite_base_and_frame_count`: Loads the longword at 92cec into a1, computes a0 as a1 plus 52, and reads the word at offset 46 of a1 into d1.
- `match_name_against_table_entries`: Compares up to fifteen bytes of each 32-byte record at a0 against the string at the table pointer, returning d1 as 1 when the record count at offset 46 equals 180, else 0.
- `return_two_in_d1`: Loads the immediate value two into d1 and returns immediately to the caller.
- `check_script_name_against_table`: Loads the table base from the longword at 92cec, adds 302a to form a0, loads the word at 92c54 into d1, calls the name scan routine, then compares d1 with two and returns.
- `copy_entity_records_and_sum_longwords`: Clamps the requested count to the record limit, copies that many eight-longword records from the entity table to a buffer, sums every copied longword into a global total and returns.
- `return_zero_in_d0`: Loads zero into d0, tests d0 and returns, so the caller receives zero as the result.
- `copy_actor_fields_and_select_pointer`: Calls several setup routines, then copies bytes at offsets 26, 27 and 21 and longs at offsets 16 and 28 from a0 into globals, stores a byte to another global, calls a routine, and returns the long at offset 28 or 16 depending on whether the word at offset 21 is zero.
- `call_init_and_return_zero`: Calls the routine at the given target, then clears d0 to zero as the return value and returns to the caller.
- `advance_sprite_frame_and_reload_state`: Compares and stores a byte in memory, calls a routine with d7 in d0, computes a table offset from a byte, stores it, and loads two bytes from the table into memory.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `run_sequence_and_compare_d1_to_two`: Calls four routines in sequence, loads the immediate value two into d0 before one call, then compares d1 with two and returns.
- `retry_disk_read_until_signature_matches`: Calls setup routines, stores a word at offset 18 of the structure, then repeatedly reads a block and compares its first bytes against a stored signature until they match or ten attempts expire.
- `start_disk_dma_read_with_sync`: Checks the disk-present bit, programs ADKCON, DSKSYNC, DSKPTH, DSKLEN and DMACON, then waits for the disk interrupt and DMA completion before returning the status.
- `verify_disk_block_signature_and_retry`: Sets retry counters, reads a block, checks the signature word and length at offsets 8 and 12, and compares the stored word at offset 18 with the current value, retrying on mismatch.
- `decrement_timer_counters_and_branch`: Decrements the word at memory location 925d4 and branches back if nonzero; otherwise reloads it with 4, decrements the word at 925d6, and if zero branches away, else calls the routine at 1fc6 before branching back.
- `compare_signature_and_branch`: Compares the long at offset 8 of a4 with the value an address and branches to one of two other routines depending on whether they are equal.
- `read_and_clear_927f6_word`: Reads the word at memory location 927f6 into d0, clears that location to zero, calls the routine at 2112, then returns to the caller.
- `start_disk_dma_read_and_wait`: Calls two setup routines, programs ADKCON, DSKSYNC, DSKLEN, DSKPTH and DMACON for a disk read, polls INTREQR bit 1 until done, then clears DSKLEN and returns 1 or 0.
- `call_setup_and_store_4000_at_offset_36`: Calls a setup routine, writes the word 16384 to offset 36 of a1, clears d0 and returns.
- `enable_ciab_interrupts_and_intena`: Writes 40960 to INTENA to enable interrupt bits and writes 17 to the CIAB control register, then returns.
- `clear_ciab_cra_and_return`: Writes zero to the CIAB control register at a fixed address, then returns to the caller.
- `disable_interrupts_set_ciab_timers`: Disables all interrupts via INTENA, clears CIAB control register, loads CIAB timer A high and low bytes with 100 and 8, then branches to another routine.
- `setup_ciab_timer_and_interrupt_vector`: Disables interrupts, clears CIAB control register, sets CIAB timer A high and low bytes, enables CIAB timer A interrupt, installs a level 6 interrupt vector, acknowledges interrupt requests, then re-enables interrupts and returns.
- `init_sound_channel_and_wait`: Masks d0 to a byte, clears two global words, stores the channel value, calls three setup routines, then polls a counter until it reaches 100 or a flag is set, calling another routine if flagged.
- `check_flag_and_jump_vector`: Tests the longword at a fixed global address; if zero it branches past, otherwise it jumps through the longword at address zero.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `build_ciab_prb_from_flags`: Combines bit 0 of one global, a second global word and a third global word into a byte and writes it to CIAB port B at the fixed hardware address.
- `build_ciab_prb_from_state_bits`: Reads three global state words, combines their low bits into d2, subtracts one and writes the low byte to CIAB port B.
- `check_fire_button_and_branch`: Tests bit 2 of CIAA port A (fire button) and branches to another routine when clear, otherwise returns immediately.
- `clear_counter_and_poll_buttons`: Clears a global word, sets d0 from CIAA port A button bits, calls a routine, then branches to another routine if bit 2 is clear.
- `check_ciia_fire_button_bit2`: Tests bit 2 of the CIAA port A register and branches away if it is clear, otherwise returns immediately to the caller.
- `clear_word_at_92c54_then_branch`: Writes zero to the word at memory location 92c54 and then branches unconditionally to another routine.
- `toggle_ciab_disk_select_lines`: Writes control values to CIAB port B to pulse disk select lines, stores a value, clears a bit indexed by a counter, then calls a helper.
- `floppy_select_and_clear_status_bit`: Writes control bytes to CIAB port B to toggle floppy select lines, stores a mask into a status word, then clears a bit indexed by another status word and calls a delay routine.
- `conditional_floppy_deselect_sequence`: Tests a status word and, if zero, writes control bytes to CIAB port B with delay calls between them, then sets a flag byte and returns.
- `short_delay_loop_counter`: Loads the value ten into a delay counter word, decrements it in a loop until it reaches zero, then returns to the caller.
- `check_fire_button_and_clear_ciaa`: Tests the fire button bit in CIAA port A, runs two subroutines while held, waits for release, then sets a flag and clears CIAA port A.
- `seed_checksum_words_and_sum_block`: Writes three constant longwords to a4, sums the third word plus 1536 following longwords into d1, and stores the result at offset 12.
- `fill_1690_bytes_with_aa`: Loads a0 with a4 plus 12320 and writes byte 170 into 1690 consecutive bytes, then returns.
- `interleave_odd_bits_and_sum_words`: Reads words from the table at offset 6164 of a4, merges their odd bits into the buffer at offset 8 of a4, and returns in d0 the sum of the first two buffer words plus the merged words.
- `expand_even_odd_bits_into_two_buffers`: Reads words from the buffer at offset 8 of a4, splits even and odd bits into two expanded streams written back to offset 8 and offset 6164 of a4, setting top bits based on previous bytes.
- `set_d0_to_three_and_branch`: Loads the immediate value three into d0 and branches unconditionally to another routine, transferring control without returning to the caller.
- `set_d0_to_4_and_branch`: Loads the immediate value 4 into the low word of d0, then branches unconditionally to a shared routine.
- `set_d0_to_5_and_branch`: Loads the immediate value 5 into the low word of d0, then branches unconditionally to a shared routine.
- `set_d0_to_6_and_branch`: Loads the immediate value 6 into the low word of d0, then branches unconditionally to a shared routine.
- `set_d0_to_7_and_branch`: Loads the immediate value 7 into the low word of d0, then branches unconditionally to a shared routine.
- `set_d0_to_8_and_branch`: Loads the immediate value 8 into the low word of d0, then branches unconditionally to a shared routine.
- `set_d0_to_9_and_branch`: Loads the immediate value 9 into the low word of d0, then branches unconditionally to a shared routine.
- `select_sound_driver_by_flag`: Saves d0-d1/a0-a1 on the stack, picks a sound driver address depending on the word flag at memory location 92cf2, calls the driver setup routine, then restores registers and jumps onward.
- `scan_string_and_store_digit`: Walks a0 forward until it reads a zero byte, then writes the following byte plus the character zero into a global table indexed by two globals, calls a routine and returns 200.
- `fill_two_coordinate_tables`: Copies d0 and d0 plus 40 into two tables at fixed addresses, then repeats four times adding 1440 to d0 and advancing both table pointers by eight bytes.
- `call_subroutine_at_5ed2`: Calls the subroutine at offset 5ed2 relative to this instruction, pushing the return address onto the stack before branching.
- `update_sprite_position`: Reads a 16-bit value from a table using an index in d0, adds it to a 16-bit coordinate in a structure pointed to by a6, and writes the result back to that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_stack_pointer_from_a5_field`: Loads the pointer at offset 178a of a5 into a1, stores the longword popped from the stack into offset 15e2 of that structure, and returns.
- `store_decremented_d6_to_screen_offset`: Pops return address into a4, advances the stack by two, calls the word-fetch routine, decrements d6, then writes d6 to offset 5606 of the structure pointed to by a5's field at 6026 and branches onward.
- `fetch_signed_word_from_a4_stream`: Reads a word from the stream at a4, advancing a4 by two; if the word is negative it negates d6 and indexes a5 by that offset, returning the resulting word in d6.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `pop_return_address_play_sound_and_return`: Pops the return address into a4, advances the stack by two, calls a nearby routine, then writes d6 minus one to d0 and jumps to a sound routine before returning.
- `check_flag_and_restore_stack_return`: Pops return address into a4, advances stack by two, calls a routine, then tests byte at a5+5826 indexed by d6-1; if nonzero, rewinds stack by six and jumps back to re-run, otherwise returns.
- `clear_flag_byte_at_indexed_offset`: Loads a0 with a5+5826 and a1 with the pointer at a5+6026, reads word at a1+5600 into d0, clears byte at a0+d0, then branches to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_screen_mode_and_poll_input`: Reads a mode word, returns if zero, clears it, then programs display registers and a flag for one of two screen modes, calls a setup routine, and polls input up to fifty times comparing results.
- `wait_for_valid_joystick_direction`: Repeatedly calls the input reader until d0 is between 200 and 249, then reads again and returns 1 if d0 is 100 to 299, otherwise returns 0.
- `read_joystick_port_bits`: Loads the longword at offset 4 from a6, shifts it right by 8 bits, masks to the low 9 bits and returns the result in d0.
- `clear_global_and_jump_ahead`: Writes zero to the longword global at the given absolute address, then branches unconditionally to the code at the following location.
- `set_sound_sequence_index_to_two`: Writes the immediate value 2 into the longword sound state variable at a fixed address, selecting sequence index two.
- `dispatch_sound_sequence_by_index`: Reads the word index at a fixed address, loads the word from the table at a fixed address indexed by it into a fixed address, then calls the routine at a fixed address and returns.
- `process_pending_object_action`: Consumes the pending object index at 6630, looks up its table entry, updates sprite and position fields, tests collision against two candidate positions, and calls the action routine when the entry is valid.
- `compute_sprite_entry_pointer`: Copies d1 to d4, shifts d4 left by two to make a byte offset, then adds it to the base at a fixed address and returns the resulting pointer in a1.
- `decrement_four_countdown_timers`: Tests each of four word counters in memory and decrements each one that is nonzero, leaving zero counters unchanged, then returns.
- `update_object_sprite_and_playfield`: Saves all registers, reads the object's sprite pointer and countdown, then writes sprite position or a default pointer and updates playfield words at offsets 150, 154 and 156 of a0.
- `check_flag_and_call_audio_init`: Tests the longword flag at memory location 9f5d0; if zero, copies a0 into a5 and calls the routine at 201e, then returns zero in d0.
- `dispatch_by_game_state_word`: Compares the word at a06fa with 4 and 3, branching away on each match, otherwise loads a4 with 9f2ac, calls the routine at 1c12, then loads a4 with 9f33e.
- `apply_audio_channel_config`: Copies fields from the a4 structure into the a5 channel block, scales a period by the global multiplier, and programs DMACON, INTENA and INTREQ with the channel enable and interrupt bits.
- `init_three_sprite_anim_blocks`: Loads three consecutive animation block pointers into a4 and calls the same setup routine after each, leaving a4 pointing at the third block.
- `advance_anim_if_active_and_timer_expires`: Checks the active flag at offset 58 of a4, loads the frame pointer at offset 20 into a3, decrements the byte timer at offset 48 and calls the frame stepper when it reaches zero.
- `step_animation_frame_phase`: Uses a4 animation state and a3 frame data to advance or reverse the position at offset 44, update the direction byte at offset 48 and clear the active flag at offset 50 when finished.
- `init_four_audio_channels`: Saves d0, enables audio DMA and interrupts, clears four channel structures, then programs AUD0-AUD3 pointers, lengths, periods, volumes and interrupt bits and clears the busy flag.
- `setup_audio_channel_from_table`: Selects a channel structure by d1, clears its fields, programs AUDx registers from the table entry, loads sample data pointers and clears the busy flag.
- `run_three_audio_channel_sequencers`: Iterates three channel structures, fetches command bytes, updates AUDx registers and pointers, advances envelope counters and calls the two mixer update routines.
- `scale_byte_by_table_and_store_offset`: Clears the low byte of d0, reads a byte from (a2) and increments a2, multiplies it by the word at a fixed long address, shifts the product right by six, then stores the word to offset 68 of a4 and branches away.
- `read_byte_advance_a2_and_branch`: Reads one byte from the address in a2 into d0, increments a2 past it, and branches unconditionally to a shared continuation elsewhere.
- `multiply_byte_by_a6_word_store_offset`: Reads a byte from (a2) and increments a2, loads the word at (a6) into d1, multiplies the two, stores the low word to offset 64 of a4, then branches to a shared continuation.
- `divide_constant_by_stream_byte`: Reads a byte from (a2) into d0, divides the constant 750 by it in d1, stores the quotient word to (a6), then branches to the shared exit.
- `sum_stream_bytes_times_word`: Reads a count byte from (a2), sums that many following bytes into d2, multiplies d2 by the word at (a6) and stores the result to offset 64 of a4.
- `load_sixteen_words_and_clear_state`: Clears low bits of byte at offset 138 of a4, copies sixteen sign-extended bytes from a table into a4 offsets 94 upward, then zeroes several a4 fields and branches to the shared exit.
- `read_byte_from_a2_advance`: Reads one byte from the address in a2 into d0 and advances a2 by one, then branches to the shared continuation at the fixed target.
- `read_signed_byte_add_base_store_a4`: Reads a byte from a2 into d0, sign-extends it to a word, adds the longword at the fixed base address, stores the word at offset 140 from a4, then branches to the shared continuation.
- `read_byte_push_a2_and_word_to_a3`: Reads a byte from a2 into d0 and advances a2; if the byte is zero it branches ahead, otherwise it pushes a2 then the word in d0 onto the stack at a3 and branches to the shared continuation.
- `push_d0_words_to_a3_stack`: Pushes the longword in d0 onto the stack at a3, then pushes the low word of d0, then branches away to another routine.
- `load_record_from_table_into_a4_fields`: Reads a byte from a2, scales it by fourteen, adds the table base at memory location 9f13a, then copies three words and two longwords from that record into offsets of a4.
- `copy_three_word_pairs_into_a4`: Loops three times copying the word at offset 118 of a4 to offset 82 and the word at offset 98 to offset 88, advancing a4 by two each pass, then returns.
- `copy_two_word_pairs_from_a4`: Copies two consecutive word pairs from offsets 114 and 94 of a4 to offsets 74 and 78, advancing a4 by two each iteration, then returns.
- `save_regs_and_test_intreqr_bits`: Pushes all data and address registers onto the stack, reads INTREQR, masks bits 7 to 10, and branches away if any are set.
- `test_d0_bit7_and_call_audio_routine`: Tests bit 7 of d0; if set, loads the custom chip base and a table pointer, saves d0, calls a routine, restores d0 and branches away.
- `dispatch_interrupt_by_d0_flags`: Tests bits 8, 9 and 10 of d0 and, when set, calls the coprocessor routine with the custom chip base and a matching parameter table, then restores registers and returns from interrupt.
- `load_sprite_frame_from_table`: Reads a word index, scales it by four, loads two words from a table into the sprite control registers and jumps onward.
- `clear_sixteen_byte_buffer`: Clears sixteen bytes starting at the buffer address in a0 by writing zero bytes in a decrementing loop, then returns.
- `zero_filled_data_block_a`: Four zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `zero_filled_data_at_3ae6`: Four zero words that decode as ori.b instructions; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_at_3af6`: Another four zero words at a later offset, again zero-filled data reached by address rather than a real routine.
- `init_audio_channels_and_voice_tables`: Scans a byte table for the highest value, builds 31 voice pointer entries from the base pointer, then silences AUD0VOL through AUD3VOL and clears several state words before returning.
- `dispatch_sound_command_via_vector`: Builds a command index from immediate bytes, computes a table entry in a0 and a1, sets d1 to minus one and d0 to eight, then calls the sound driver and jumps to its exit vector.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.

### astcode (raw code file, load address unknown)

- `astcode_command_dispatch_stubs`: Jumps through the vector table to set up a0 and d0-d2, stores them to fixed memory, then runs three guarded command sequences that call the driver and compare the frame counter.

### wizkiddemo5e (raw code file, load address unknown)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### wizkiddemo80 (raw code file, load address unknown)

- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### wizkiddemoc0 (raw code file, load address unknown)

- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
