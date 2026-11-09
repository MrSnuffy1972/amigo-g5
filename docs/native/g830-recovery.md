# Game 830 (populous (1989)(electronic arts)[m dave soft]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00830). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 33 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 353 raw-word lines |
| Functions named and described | pass, 527 of 527 named (0 author symbols), 527 of 527 described (110 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 708 of 708 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11356 of 12152 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 619427 |
| code | 39784 |
| data | 202590 |
| padding | 9623 |
| unused | 29696 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| loadwb | 12 | 536 | 30 |
| wait | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Populous (1989)(Electronic Arts)[m Dave Soft].adf:populous | 3 | 4352 | 145 |
| Populous (1989)(Electronic Arts)[m Dave Soft].adf:populous.prg | 3 | 1760 | 9 |
| icon.library | 10 | 1375 | 109 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2597 | 167 |
| version.library | 2 | 41 | 7 |
| Populous (1989)(Electronic Arts)[m Dave Soft].adf:bootblock | 1 | 264 | 3 |

## Cases handled

- populous (1989)(electronic arts)[m dave soft]/Populous (1989)(Electronic Arts)[m Dave Soft].adf: file !ANC!ANC!ANC! (header 28080 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- populous (1989)(electronic arts)[m dave soft]/Populous (1989)(Electronic Arts)[m Dave Soft].adf: blocks 989-1043 (bytes 506368-534528) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.14)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `disable_audio_dma_and_clear_flag` (populous): Loads the base of the sound state area into a3, clears the byte at offset 766, then writes 15 to DMACON and 255 to ADKCON to disable audio DMA and clear interrupt control. Kind: disk hardware/sync check. Pass path: the branch following the call that does not lead to a crash or infinite loop (typically the instruction immediately following the call site).

## Functions


### type

- `allocate_type_node_and_copy_name`: Calls library routines through a5 to find the highest type id, allocate a node, fill its type table with incrementing values, copy sixteen longs into the node and return zero or minus one.
- `return_d1_in_d0_then_branch`: Moves d1 into d0 as the return value and branches to another routine, so it returns the caller's d1 unchanged.

### endcli

- `load_and_install_resource_table`: Reads a resource list via a5 library calls, finds the largest entry, allocates a block, fills it with incrementing values, copies sixteen longs into the structure at offset 536 and returns zero or minus one.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0 and branches to another routine, so d0 carries d1 as the value passed onward.

### loadwb

- `loadwb_startup_and_command_parse`: Program entry that opens dos.library, allocates a task structure, parses the command line into argument pointers, opens the named file and starts the main routine.
- `loadwb_cleanup_and_exit`: Finds the current task, restores its saved stack frame, closes the open file and library, frees memory, replies the startup message and returns the exit code.
- `loadwb_start_workbench_task`: Opens workbench.task, allocates and patches a task structure, looks up the Workbench resident tag, starts the tool and signals the workbench task, returning after cleanup.
- `compare_two_strings_lexically`: Compares the two null-terminated strings passed on the stack byte by byte, returning zero if equal, minus one if the first is less, or one if greater.
- `build_linked_list_from_array`: Walks a chain of array entries, allocating eight-byte nodes, copying each entry's second field via a call, linking nodes through the first field, and returning the head node.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `check_workbench_version_string`: Compares the byte at offset 20 of the structure in a2 against the character '2', then compares the structure bytes against a version string; if the string ends, writes a dot and four spaces at offset 17 of a2.
- `write_dot_and_spaces_to_name`: Writes a dot followed by four space characters into bytes at offsets 17 through 21 of the structure pointed to by a2, then branches onward.
- `compare_name_against_version_string`: Loads a2 into a0 and a program-counter-relative string into a1, then compares bytes from a0 and a1 until a mismatch or the string terminator, branching accordingly.
- `jump_to_pc_relative_target_restore_a0_a1`: Loads a2 with a program-counter-relative address, pops a0 and a1 from the stack, pushes the value at address zero, then returns to the loaded target.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_function_with_three_stack_args`: Saves d2 and d3, loads two arguments from the stack, pushes a pointer to the third stack argument plus both arguments, calls a target, then cleans the stack and restores d2 and d3.
- `format_string_via_rawdofmt_then_dispatch`: Saves a2-a4 and a6, calls exec RawDoFmt into a 140-byte stack buffer, measures the resulting string length, and passes the buffer and length to another routine.
- `store_byte_to_a3_postincrement`: Writes the low byte of d0 to the memory pointed to by a3, then increments a3 by one and returns to the caller.

### wait

- `allocate_and_initialize_wait_structure`: Calls library routines through a5 to find the largest node value, allocate a block, fill it with incrementing longwords, copy sixteen longs into the structure at offset 536, set a callback pointer and return zero or minus one.
- `copy_d1_to_d0_and_branch_into_wait`: Moves d1 into d0 and branches backward into the wait routine's code, transferring control rather than returning to the caller.

### Disk-Validator

- `disk_validator_entry_setup`: Program entry that writes a version digit into a buffer, clears a board row, and calls library routines through a5 to set up the disk validator display.

### Populous (1989)(Electronic Arts)[m Dave Soft].adf:populous

- `jump_to_main_startup`: Single jump instruction that transfers execution to the main startup code located further ahead in the program.
- `game_init_open_screens`: Opens the demo picture and several screens, stores their handles in a4-relative globals, then calls the setup, input and display routines before returning.
- `call_first_io_request_handler`: Opens a stack frame, loads the global IO pointer from a4-relative memory, pushes the longword at offset 38 of that structure and calls the handler, then discards the argument and returns.
- `init_sound_and_enable_interrupts`: Calls two setup routines, passes four volume or period values with their globals to a configuration routine, then writes 1040 to the INTENA hardware register to enable interrupts before returning.
- `create_and_configure_sound_channel`: Allocates a 40-byte structure, opens a file named startup, loads it into memory, fills the structure fields with size, name pointer and zeroed offsets, then passes it to a playback routine.
- `build_eight_entry_pointer_table`: Allocates 1152 bytes, stores the returned pointer at offset -32116 from a4, then fills eight consecutive longword entries with pointers spaced 144 bytes apart, and finally calls a routine with the base pointer and value 129 before storing derived pointers at -32196 and -32192.
- `set_sound_mode_399_and_call`: Stores the word value 399 at offset -32196 from a4 and then calls the routine at pc-relative offset 53ee before returning.
- `free_pointer_table_buffer`: Pushes the longword pointer stored at offset -32116 from a4 and the size 1152, calls the memory free routine, then cleans the stack and returns.
- `close_two_audio_channels`: Pushes two audio channel pointers from offsets in a4 onto the stack and calls the close routine for each, then returns.
- `set_audio_channel_volume`: Reads a 16-bit volume argument from the stack, sign-extends it to a long, and passes it with an audio channel pointer from a4 to a setup routine.
- `invoke_audio_update`: Sets up a stack frame and calls the audio update routine at a fixed relative address, then releases the frame and returns.
- `find_index_and_dispatch_pair`: Scans two longword entries in the a4-relative table for a match with the argument at 12(a5), calls the paired routine with the matching entry, and stores the found index in a4-relative memory.
- `swap_two_globals_and_call_compare`: Swaps the two longwords at a4-relative offsets, then pushes minus one and both swapped values on the stack and calls the routine at 776 before restoring the stack.
- `open_libraries_and_read_screen_flags`: Opens graphics and intuition libraries, stores their bases in a4-relative globals, exits on failure, and saves the screen's flag bits masked with four into a4-relative memory.
- `wait_for_menu_choice_then_show_error`: Calls the menu selection routine repeatedly until it returns 1, 2 or 3, then displays a message using the string and file pointers from the a4 base and returns.
- `load_pic_and_poll_mouse_region`: Loads the load.pic file, sets up a screen, then polls the mouse position up to an address times, returning 1, 2 or 3 depending on which screen region was clicked.
- `init_units_and_run_build_loop`: Initialises unit tables and screen pointers, then loops reading input and drawing two groups of units until the input routine signals exit.
- `update_two_player_cursor_positions`: Decrements the two cursor height counters, redraws each cursor at its new row, and updates the score digits and animation frames for all sixteen player slots.
- `branch_to_earlier_code`: Unconditionally branches backward to an earlier address in the same routine; it is a single branch instruction with no other effect.
- `draw_and_animate_landscape_panel`: Draws the landscape panel rows and columns using the graphics routine, fills the height table with random values, and returns one when the game should continue.
- `fill_terrain_height_and_type_arrays`: Loops over the given count, calls a random routine, and stores each result modulo 13 plus 232 into the height array and modulo 9 into the type array indexed by the loop counter.
- `erode_terrain_arrays_return_first_delta`: Loops over the given count, subtracts one plus the type value from each height, halves and negates the type when negative, stores both back, and returns the first computed type value.
- `run_demo_playback_loop`: Loads the demo picture, initializes graphics and sound, generates terrain, then repeatedly draws and updates the demo until a key or flag ends it, restoring the screen and sound.
- `init_four_player_sprite_offsets`: Loops four times storing computed sprite and land pointers into arrays based on a base pointer plus index times 300 and 32000, then calls a setup routine and copies the first entries into current-player globals.
- `open_and_load_land_file`: Opens a file with mode 1005, loads 32000 bytes into the current land buffer, sets up the display, closes the file and returns zero, or minus one if opening failed.
- `load_font_land_and_sprite_files`: Opens and loads three files into memory buffers, copying data to the font, land and sprite areas, calling a display setup between the first and second loads, and closing each file.
- `open_file_and_load_into_memory`: Opens a file by name with mode 1005, stores the returned handle, then reads 40000 bytes and 64 bytes into two buffers and closes the file.
- `blit_sprite_rows_into_planar_bitmap`: Reads five words per row from a sprite table, rotates them by the low nibble of d0, and masks/merges them into four bitplanes of the destination bitmap for up to 32 rows.
- `blit_sprite_rows_into_screen_bitmap`: Reads five words per row from a sprite table offset by d2, rotates by the low nibble of d0, and masks/merges them into four bitplanes at a0 for up to 16 rows.
- `call_blit_with_three_args`: Saves registers, loads a0, d0, d1 and d2 from the stack frame arguments, calls the blit routine, restores registers and returns.
- `blit_sprite_rows_to_bitplanes`: Uses d0 as pixel shift and d1/d2 as coordinates to mask and OR sprite words from a table into four interleaved bitplane rows at a0.
- `rotate_entity_records_forward`: Loops over 1680 records of 20 bytes, shifting each record's fields forward by one slot and swapping the high and low words of three longwords.
- `scale_heightmap_bytes_by_eight`: Loops 100 times over the heightmap bytes at the fixed table plus 200, shifting each byte left by three bits and storing it back in place.
- `draw_isometric_terrain_grid`: Walks a 10 by 10 grid of heightmap bytes, computes isometric screen coordinates from row, column and height, and calls the plot routine for each tile, optionally drawing a second offset copy.
- `plot_single_tile_from_index`: Takes a tile index and a value from the stack, reads the heightmap byte at that index, converts the index to isometric screen coordinates and calls the plot routine once.
- `set_text_pointer_to_start`: Stores the address of the start of the text buffer into the global text cursor and returns to the caller.
- `render_text_string_to_bitmap`: Clears a bitmap buffer, then draws each character of the current text string into it using a font table, advancing the global text cursor and resetting it at the terminator.
- `zero_filled_data_area`: This single zero word decodes as an empty instruction that does nothing; the bytes are zero-filled data referenced by address, not executable code.
- `populous_data_table_at_1fa2`: Bytes here are data words decoded as code, not executable instructions; other code refers to this table by address and it changes no registers.
- `populous_data_table_at_25e2`: This range holds data words that decode as no-op ori instructions; it is a table referenced by address, not working code.
- `branch_to_2c36_trampoline`: Executes a single unconditional branch instruction that transfers control forward to the code at the target address, touching no registers or memory.
- `zero_filled_data_word_at_2c2e`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_2c32`: Another single zero word decoding as ori.b #0,d0; this is zero-filled data pointed to by other code, not a real routine.
- `open_popsam_load_and_install_interrupts`: Allocates memory, opens dos.library and the popsam file, reads it into memory, then saves interrupt vectors, disables interrupts, installs a new level-3 handler and clears a flag before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_a0_offset_byte_and_return`: Clears the byte at offset 916 from a0, then restores all registers from the stack, releases the a5 frame and returns.
- `reset_interrupts_and_install_vectors`: Disables all interrupts, clears interrupt requests, calls a setup routine, frees memory, copies three longwords and a word into low memory vectors, then re-enables interrupts with bits set.
- `compare_d0_to_ten_then_branch`: Compares the low byte of d0 with ten; if not equal it branches ahead, otherwise it calls the setup routine and jumps to the clear-and-return code.
- `map_byte_one_to_zero_and_call`: Compares the low byte of d0 with 1; if equal, sets d0 to 0, calls the routine at 12188, then branches to 11724, otherwise branches to 11882.
- `map_byte_two_to_one_and_branch`: Compares the low byte of d0 with 2; if equal, sets d0 to 1 and branches to 11724, otherwise branches to 11898.
- `map_byte_three_to_two_and_branch`: Compares the low byte of d0 with 3; if equal, sets d0 to 2 and branches to 11724, otherwise branches to 11914.
- `check_d0_for_four_then_set_three`: Compares the low byte of d0 with 4; if not equal it branches away, otherwise it sets d0 to 3 and jumps to another routine.
- `branch_to_shared_return_path`: Unconditionally branches to the shared routine that clears a byte at offset 884 from a0 and returns.
- `vertical_blank_interrupt_handler`: Saves all registers, reads CIA and interrupt registers to test the vertical blank bit, acknowledges it, sets a flag, samples the parallel port bit into a variable and calls a subroutine before restoring registers.
- `level3_interrupt_return_stub`: Executes the 68000 rte instruction to return from an interrupt, restoring the saved status register and program counter from the supervisor stack.
- `zero_filled_data_area_2`: This single zero word decodes as ori.b with immediate zero to d0; the bytes are zero-filled data referenced by address, not executable code.
- `init_level_tables_and_interrupt_vector`: Builds sprite and level pointer tables from data at a3, computes timing values, sets up four level records, installs the level three interrupt vector and sets a flag.
- `disable_audio_dma_and_clear_flag`: Loads the base of the sound state area into a3, clears the byte at offset 766, then writes 15 to DMACON and 255 to ADKCON to disable audio DMA and clear interrupt control.
- `start_audio_channel_playback`: Checks the sound-enabled flag, selects channel 3, and if the channel is idle programs its AUD pointer and length registers from the sample descriptor before advancing the sequencer.
- `set_channel_pointer_and_restart_sequence`: Points channel 3 AUD registers at the silence buffer, decrements the channel counter, and on the last channel sets the DMA enable bit for that channel in DMACON.
- `parse_sound_command_stream`: Clears the first word of the sound channel block at a0, reads a command byte from (a1)+ and dispatches on values 128 to 132 to set up instrument, volume, pitch or loop data before continuing.
- `select_instrument_from_stream_byte`: Reads a byte from (a1)+, scales it by 32 to index the instrument table at a5, stores that pointer and the derived sample pointer into the channel block at a0.
- `set_channel_volume_and_sample_pointer`: Reads a byte from (a1)+, multiplies it by the master volume byte at offset 768 of a3, stores the result as the channel volume and saves the sample pointer and hardware audio registers.
- `branch_to_shared_continuation`: Branches unconditionally to a shared continuation address, transferring control without touching any registers or memory.
- `store_byte_and_set_bit1_then_branch`: Copies the byte at (a1) into offset 20 of a0, increments a1, sets bit 1 at offset 0 of a0, then branches to the shared continuation.
- `advance_record_pointer_and_load_next`: Reads offset 12 of a0 into d0, adds it to the pointer at offset 8 to form a2, tests the longword there, and on zero reloads a2 and d0, sets a1 from (a2) plus a3, stores d0 to offset 12, then branches to the shared continuation.
- `init_voice_channel_from_template`: Scales d0 by a byte at offset 768 of a3, stores it and a copied byte pointer into the a0 channel block, copies a5 fields into the a6 hardware area, computes a period from a table and writes it to the audio registers, then clears the active flag.
- `update_voice_pitch_and_volume`: Adjusts the a0 channel period by its delta byte and writes it to the a6 audio register, applies a signed multiplier from the sample stream, then steps the volume index in a5 and writes a marker byte into the a5 buffer.
- `clear_volume_step_and_jump_back`: Writes a zero byte at the a5 buffer indexed by d1, stores d1 into the a0 volume field, then branches back to the shared channel update loop.
- `decrement_d1_compare_d2_clear_bit2`: Subtracts one from d1, compares it with d2, and if d1 is still above d2 clears bit 2 of the byte at a0 before branching back.
- `store_tile_byte_and_column_index`: Writes the byte value 60 into the word at a2 plus d1, stores d1 into offset 36 of a0, then branches onward.
- `blit_masked_tile_rows_to_bitplanes`: Copies masked tile words from a source table into four bitplane rows of the destination, advancing a0 by two and skipping 32 bytes per row, looping 64 rows of 4 words.
- `blit_masked_tile_rows_to_bitplanes_2`: Copies 160 words from a source buffer plus a mask table into four bitplane rows of a destination, combining each word with the mask and the source offset to update the screen.
- `store_global_and_dispatch_terrain_update`: Loads six word arguments and a pointer from the stack, stores one word into a global, then calls the terrain update routine before restoring registers.
- `compute_terrain_cell_screen_position`: Converts a cell index into screen coordinates using division and table lookups, adjusts for slope height, selects direction offsets, and calls the drawing routine.
- `mouse_and_button_interrupt_handler`: Reads JOY0DAT mouse counters and POTGOR button bits, updates cursor position and button state variables, calls the cursor update routine, then acknowledges CIA and custom-chip interrupts before returning from the interrupt.
- `reset_mouse_delta_baseline`: Samples JOY0DAT to store the current horizontal and vertical mouse counters as the baseline deltas, then calls the cursor update routine with d0 cleared.
- `install_vertical_blank_interrupt_vector`: Saves the current interrupt enable state and level 3 vector, then points the level 3 autovector at the game's handler and enables vertical blank and port interrupts.
- `disable_interrupts_restore_vector`: If the saved interrupt vector at absolute long 5494 is nonzero, disables all interrupts via INTENA, clears INTREQ, installs that vector at CPU vector 27, then re-enables interrupts with the saved mask from 5490 and clears the saved vector; returns the mask in d0.
- `zero_filled_data_area_3`: Three zero words that decode as ori.b #0,d0; this range is a zero-filled data area referenced by address, not executable code.
- `build_copper_list_display`: Builds a Copper display list in the caller's buffer from the passed bitplane pointers, palette, and sprite data, writing move and wait instructions, then enables Copper DMA via DMACON and returns.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `set_copper_list_and_wait_vblank`: Stores the caller's return address at absolute address 1280, writes that address plus four to the COP1LCH copper pointer, then polls INTREQR bit 5 until the vertical blank interrupt flag is set.
- `acknowledge_vblank_interrupt`: Writes 32 to INTREQ to clear the vertical blank interrupt, then polls INTREQR bit 5 until that interrupt request flag becomes set again.
- `copy_words_after_180_marker`: Scans the first pointer's words for value 384, then copies the given count of words from the second pointer into the following slots, advancing both pointers by four bytes per word.
- `copy_sprite_frame_to_two_buffers`: Indexes a table by the first argument times 160, then copies 16 longwords from that entry into two destination buffers offset by 4 and 72 bytes, writing a leading 128 and trailing zero longwords.
- `fill_sprite_offsets_and_flags`: Looks up a sprite record by index, computes position bytes from the arguments, sets flag bits in the record's status byte and stores the values at offsets 0-3 and 72-75.
- `poll_ciaa_keyboard_serial`: Reads the CIAA serial data register, tracks key press and release state, and stores the decoded key code into two keyboard buffers when a new press is detected.
- `copy_8000_bytes_between_buffers`: Copies 8000 bytes from the source pointer at offset 8 of the frame to the destination pointer at offset 12, advancing both pointers, then restores a0-a1 and returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `decrement_packed_nibbles_with_vblank_wait`: Copies words to a stack buffer, decrements each nibble until all are zero, waits on VHPOSR vertical position and calls the blit routines between passes.
- `get_random_then_call_with_two_args`: Calls a routine that returns a value in d0, stores it in a local, then calls another routine with the two longword arguments from the frame and returns its result.
- `call_6ba2_then_optional_5420`: Pushes the longword argument from the stack frame, calls the routine at 6ba2, drops the argument, and if the global at a4 offset is nonzero calls the routine at 5420 before returning.
- `advance_seeded_random_and_return_15bit`: Multiplies the stored seed by a constant via a called routine, adds 12345, stores the new seed, then shifts it right 16 bits and returns the low 15 bits in d0.
- `store_word_arg_high_and_init_library`: Stores the high word of the stack argument into a global, then clears a buffer, saves the stack pointer, opens dos.library via exec and alerts on failure.
- `set_a4_to_upper_memory_limit`: Loads the immediate long value 32766 into address register a4 and returns to the caller.
- `allocate_and_initialize_manx_structure`: Allocates a block sized from a global count, stores it, fills header fields, writes the MANX tag, opens a resource, and sets up linked structures before returning.
- `copy_d0_to_d5`: Moves the long value in data register d0 into data register d5, then falls through into the following routine.
- `parse_quoted_tokens_into_string_array`: Allocates a buffer, copies a token string, splits it into whitespace-separated tokens honoring double quotes, allocates a pointer array and stores each token's address, returning via a4 globals.
- `load_immediate_7fff_into_d0`: Moves d0 to itself, loads the immediate value 32767 into d0, then branches away to another routine.
- `bounded_string_append_with_terminator`: Finds the end of the destination string in a0, copies up to the given count of bytes from a1, and null-terminates, returning the destination pointer in d0.
- `bounded_strncpy_pad_nul`: Copies bytes from a1 to a0 until a zero byte or d1 count is reached, then fills the remaining count with zero bytes and returns a0 in d0.
- `unsigned_32x32_multiply_d0_d1`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit multiplies, returns the low 32-bit product in d0 and preserves d1-d3.
- `open_window_and_load_icon_resources`: Opens a window via library calls, loads icon data from the window's resource list, stores icon pointers and counts at offsets in a2, then closes the window.
- `copy_d0_to_d5_2`: Copies the longword in d0 into d5, then falls through into the following routine.
- `call_hook_then_dispatch_word_arg`: Opens an a5 frame, calls the function pointer stored at offset -3480 from a4 if nonzero, then pushes the word argument at 8(a5) and calls the next routine.
- `shutdown_cleanup_and_restore_stack`: Saves d4, frees several a4-relative buffers and calls registered hooks, optionally invokes exec Supervisor, then restores a7 from -3522(a4) and returns the saved argument in d0.
- `pop_d4_unlink_and_return`: Pops d4 from the stack, releases the a5 frame and returns to the caller.
- `free_slot_by_index_with_bounds_check`: Looks up a six-byte slot by index, validates it against bounds, frees its pointer via a helper, clears the slot and returns zero or minus one on error.
- `jump_to_slot_free_entry`: Jumps directly to the slot-freeing routine, passing control without setting up a new stack frame.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `jump_via_a4_table_offset_1e`: Loads a pointer from the a4-relative table at offset minus three thousand four hundred eighteen, then jumps to the code pointer at offset minus thirty within that structure.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `jump_via_a4_table_offset_19e`: Loads a pointer from the a4-relative table at offset minus three thousand five hundred eighteen, then jumps to the code pointer at offset minus four hundred fourteen within that structure.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `jump_via_a4_table_offset_c6`: Loads a pointer from the a4-relative table at offset minus three thousand five hundred eighteen, then jumps to the code pointer at offset minus one hundred ninety-eight within that structure.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `jump_via_a4_offset_minus_294`: Loads a pointer from the a4-relative table at offset minus 294 and jumps through the entry at minus 294, transferring control indirectly.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `jump_via_a4_offset_minus_132`: Loads a pointer from the a4-relative table at offset minus 294 and jumps through the entry at minus 132, transferring control indirectly.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `jump_via_a4_offset_minus_210`: Loads a pointer from the a4-relative table at offset minus 294 and jumps through the entry at minus 210, transferring control indirectly.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `jump_to_6c9c_via_pc`: Performs an unconditional jump to the instruction at the given program counter offset, transferring control without saving a return address.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### Populous (1989)(Electronic Arts)[m Dave Soft].adf:populous.prg

- `save_registers_and_install_trap_handler`: Branches forward, then saves all registers to a buffer pointed to by a6, stores the stack pointer and a handler address, and executes an illegal instruction.
- `save_context_and_relocate_vectors`: Stores d0 and the user stack pointer, saves all registers into the a6 buffer, sets a new trap vector, then executes many no-ops.
- `relocate_overlay_blocks_by_offset`: Walks a table of block descriptors, adds a base offset to each stored pointer entry, then copies thirteen words from the routine area to the hardware register block.
- `apply_relocation_table_to_overlay`: Computes the overlay end from the size and base globals, then for each descriptor adds the relocated base to the indexed pointer entries in the target block.
- `branch_to_overlay_copy_tail`: Unconditionally branches forward to the tail of the overlay relocation code, skipping the descriptor loop and continuing at the copy stage.
- `index_pointer_table_by_d0`: Uses d0 as a count to walk a table of longword pointers starting at the address loaded relative to pc, advancing a0 by four bytes per entry, then returns the selected pointer in a0.
- `relocate_vector_table_and_jump`: Copies thirteen words from a pc-relative source into the vector area at a fixed low address, sets up stack and registers, then branches to the relocated code.
- `copy_words_then_clear_remainder_rte`: Copies longwords from a0 to a1 until a1 reaches a2, zero-fills up to a0, saves d0-d7 to low memory and returns from exception with rte.
- `populous_sprite_offset_table`: The bytes are data, not executable code: a table of small offset values decoded as ori.b instructions, referenced by a program counter relative address and used by two callers.

### icon.library

- `icon_library_return_minus_one`: Library code that loads the value minus one into d0 and returns immediately, giving the caller a failure or sentinel result.
- `icon_library_open_increment_and_clear_bit`: Library code that increments the word at offset 32 of the library base in a6, clears bit 3 of the byte at offset 14, and returns the base in d0.
- `decrement_open_count_and_close_if_zero`: Decrements the word at offset 32 of the library base in a6; if it reaches zero and bit 3 of the byte at offset 14 is set, calls the close routine at offset 256, then returns zero in d0.
- `close_library_and_unlink_from_list`: If the word at offset 32 of the a6 base is zero, unlinks the library node from its list, calls the expunge vector at offset 34 with d0 set from offsets 16 and 18, and returns the saved value.
- `set_close_flag_and_return_zero`: Sets bit 3 of the byte at offset 14 of the library base in a6, marking it for delayed close, and returns zero in d0.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_and_store_base`: Saves a2, stores the caller's a0 and a6 into the new structure at offsets 42 and 34, opens dos.library via exec, stores the base at offset 38, and alerts on failure.
- `copy_structure_fields_to_globals`: Copies the longwords at offsets 34 and 38 of the a2 structure into two absolute global locations, returns a2 in d0 and pops a2 from the stack.
- `zero_filled_data_area_4`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word`: Contains a single zero word that decodes as an ori.b immediate instruction; this is zero-filled data referenced by address, not executable code.
- `save_a0_a6_call_vector`: Pushes a0 and a6 onto the stack, calls the routine at address zero, then discards both saved registers and returns to the caller.
- `save_a0_a1_call_vector`: Pushes a0 and a1 plus a6 onto the stack, calls the routine at address a8, then removes the saved registers and returns.
- `save_a0_a2_and_a6_call_library`: Pushes registers a0 through a2 and a6 onto the stack, calls a library routine, then discards the saved registers and returns.
- `save_a0_a1_and_a6_call_library`: Pushes registers a0, a1 and a6 onto the stack, calls a library routine, then discards the saved registers and returns.
- `save_a0_and_a6_call_library`: Pushes register a0 and a6 onto the stack, calls a library routine, then discards the saved registers and returns.
- `icon_library_call_with_a0_a6_saved`: Pushes a0 and a6 onto the stack, calls the library routine at offset 692, then discards both saved registers and returns.
- `icon_library_call_with_a6_saved`: Pushes a6 onto the stack, calls the library routine at offset 6a8, then discards the saved register and returns.
- `icon_library_call_with_a0_a2_a6_saved`: Pushes a0 through a2 and a6 onto the stack, calls the library routine at offset 71c, then releases the saved registers and returns.
- `icon_lib_call_768_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 768, then discards both saved registers and returns.
- `icon_lib_call_7fc_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 7fc, then discards both saved registers and returns.
- `icon_lib_call_818_with_a0_a1`: Pushes a0 and a1 onto the stack, calls the library routine at offset 818, then discards both saved registers and returns.
- `icon_library_call_jsr_864`: Saves a0 and a1 on the stack, calls the library routine at offset 864, then discards the saved registers and returns.
- `icon_library_call_jsr_8d6`: Pushes a0 and a1 onto the stack, invokes the library routine at offset 8d6, cleans the stack by adding eight and returns.
- `icon_library_set_field_94`: Opens the library base from the caller frame, calls its offset minus 294 vector, stores the second argument at offset 148 of the returned pointer, and returns.
- `zero_filled_data_area_5`: The single word decodes as ori.b #0,d0, so this range is zero-filled data that other code refers to by address, not executable code.
- `allocate_and_init_icon_object`: Calls library routines to allocate an icon object for two arguments, then fills its fields from a stack buffer and links it into a parent list, returning the object or zero.
- `copy_stack_buffer_into_icon_object`: Copies eleven longs from the stack frame into the icon object at a2, stores several scalar fields, and links the object into its parent list before returning.
- `icon_library_return_zero_restore_regs`: Library routine that sets d0 to zero as its return value, restores d2-d4 and a2 from the stack frame, releases the a6 frame and returns.
- `icon_library_build_and_dispatch_request`: Library routine copies eleven longwords from offset 96 of the passed structure into a local buffer, fills in fields from the structure, then calls the dispatcher with the two arguments and the buffer.
- `icon_library_open_icon_by_name`: Library routine formats a name into a local buffer, looks up the icon by tag, and on success fills the caller's icon structure from the returned data.
- `icon_parse_header_and_allocate_arrays`: Calls a hook, checks the icon header magic and version, clears two fields in the icon structure, then allocates and stores the image and select-mask arrays via library allocation calls.
- `icon_parse_image_and_mask_arrays`: Resolves the icon's image and select-mask data through a library call, stores them in the icon structure, then allocates and fills the image and mask arrays from the header.
- `icon_copy_image_array_words`: Loops over the icon image array, allocating each word through the library and storing it, then optionally allocates the mask array and calls a cleanup routine.
- `set_d4_zero_and_branch_to_epilogue`: Loads immediate zero into data register d4 and branches unconditionally to the shared epilogue at the start of the following routine.
- `return_d4_as_d0_and_restore_registers`: Moves d4 into d0 as the return value, restores d2-d5 and a2-a5 from the stack frame, releases a6 and returns to the caller.
- `icon_library_open_icon_by_name_2`: Library code that links a frame, saves registers, allocates a lock, opens the icon by name, reads its fields and copies name, tool types and other strings into the lock, then frees resources and returns a status.
- `icon_put_chunk_with_optional_children`: Saves d6, calls library helpers to write a chunk header and size, then conditionally writes child chunks from fields at offsets 26, 66, 22 and 26 of the a2 structure, returning status in d4.
- `icon_put_chunk_children_and_strings`: Calls helpers to write chunks from fields 22 and 26 of a2, then writes a length-prefixed string from field 50 and iterates the pointer list at field 54, returning status in d4.
- `icon_skip_string_list_and_write_chunk`: Advances a3 past the zero-terminated pointer list at field 54, counting four bytes per entry, then calls the a4 writer to emit the list chunk and reloads a3 from field 54.
- `icon_library_scan_name_list`: Walks the null-terminated pointer list at a3, calling the string-length routine and the a4 callback for each entry, then handles the pointer at offset 70 of a2, and finally calls two library routines before storing d6 at offset 26 of a2 and returning d4 in d0.
- `push_a2_load_caller_arg_into_a0`: Pushes a2 onto the stack, loads the longword at offset 12 from the stack into a0, sets a2 to two bytes past that pointer, then branches onward.
- `icon_library_free_chain_until_wrap`: Frees the object in d0, then repeatedly frees the object in a2 and advances a2 until it equals the pointer at offset 8 of a2, freeing the final d0 if nonzero and restoring a2 from the stack.
- `call_icon_open_with_offset`: Loads the first stack argument into d0 and the second into a0, then calls the library routine at offset 1622 with a pointer to a0 plus 140 and d0 pushed, cleaning the stack afterward.
- `allocate_and_init_icon_node`: Saves d2-d3/a2, allocates 65536 bytes, then initializes fields at offsets 142 and 140 of the new block via library calls, freeing the block and returning zero if initialization fails.
- `finish_icon_node_init`: Pushes a2, the field at offset 140 of a2, and d2, calls the library routine at offset 1820, cleans 16 bytes of stack, and returns a2 in d0 on success.
- `icon_lib_return_zero_pop_regs`: Sets d0 to zero as the return value, restores d2 and d3 and a2 from the stack, then returns to the caller in icon.library.
- `icon_lib_decrement_count_and_store_pair`: Decrements the word at a2, and if it reaches zero calls a library routine; on success stores d0 and d2 into the table pointed to by a2's field.
- `icon_lib_store_pair_after_decrement`: Decrements the word count at a2, then writes d2 and the stack argument into the table at a1 indexed by the new count, returning one.
- `allocate_and_init_icon_node_2`: Saves d2-d4/a2, allocates a 94-byte block via library call, initializes the node at offset 78, then links it to the two input pointers and returns the block or zero.
- `branch_to_cleanup_exit`: Executes a single unconditional branch to the shared cleanup exit, transferring control without touching any registers or memory.
- `free_icon_node_and_return_zero`: Pushes the node size and d4, calls the free routine, clears d4 to zero, restores d2-d4/a2 and returns zero in d0.
- `call_icon_helper_with_offset_4e`: Saves d2, loads two arguments from the stack, adds 78 to the second argument, pushes both arguments and calls a library routine, then restores d2 and returns.
- `call_function_and_test_result`: Saves d2-d4 and a2-a3, loads a pointer and a value from the stack, pushes the value, calls a routine, stores the result in d3 and the pointer in d4, then branches based on the result.
- `scan_for_byte_3d_after_call`: Saves d3, d2 and a3, calls a routine, tests its result, and if zero advances a3 by d3 and checks for byte 61, returning the pointer if found.
- `pop_next_a2_pointer_to_d0`: Loads the longword pointed to by a2 into a3 and d0, advances a2 by four, and if the value is zero sets d0 to zero before restoring d2-d4/a2-a3 and returning.
- `call_indirect_with_stack_arg_save_d5`: Saves d2-d5, loads two stack arguments into d2 and d3, pushes d3, calls the routine at address zero, stores the result in d5, discards the argument and branches onward.
- `call_with_sevenc_and_d2_difference`: Pushes the constant at address 7c and d2, calls the routine at address 10, saves the result in d4, discards both arguments, and if nonzero returns d4 minus d2 in d0.
- `find_next_icon_block`: Calls two library routines to compare d0 with d5 and test a block; returns 1 in d0 when the match succeeds, otherwise falls through to the shared exit.
- `advance_d2_past_d4_block`: Tests d4; if nonzero copies d4 into d2 and increments d2 by one, then branches to the shared exit path.
- `return_zero_restore_d2_d5`: Clears d2, tests it, sets d0 to zero and restores d2 through d5 from the stack before returning to the caller.
- `parse_icon_header_and_append`: Saves registers, clears byte 30 of the first argument, calls library helpers to parse a five-byte header and a thirty-byte block from the second argument, then appends the parsed data to the first argument and returns it.
- `append_three_byte_block_to_icon`: Advances a4 by five, calls a library helper with a three-byte block and a twenty-eight-byte block, and on success sets d3 to one and advances a4 by three before branching onward.
- `set_d3_zero_and_branch`: Sets d3 to zero and branches forward to another part of the icon library code, returning no value itself.
- `parse_decimal_number_from_string`: Accumulates digits from the string at a4 into d3 as a decimal value, calls the digit converter for each character, then formats and stores the result via library calls before returning a2.
- `convert_ascii_digit_to_value`: Loads the character argument from the stack into d1, checks it lies between ASCII zero and nine, and returns its numeric value in d0 or falls through otherwise.
- `return_minus_one_constant`: Loads minus one into d0 and returns immediately, serving as the failure or invalid-digit result for the digit converter.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `icon_allocate_and_initialize_object`: Saves d2-d5, reads three arguments from the stack, allocates memory via a library call, then initializes the object through two further library calls, returning the object pointer or zero.
- `icon_create_list_with_ten_entries`: Saves d2 and a2, allocates a small block, stores the value ten at offset fourteen and at the caller's word pointer, then links the block into a list and returns success.
- `icon_return_zero_restore_registers`: Sets d0 to zero as the return value, restores d2 and a2 from the stack and returns to the caller.
- `call_icon_vector_30_compare_result`: Loads two arguments from the stack, pushes a third stack argument plus both onto the stack, calls the library vector at offset 48, compares the returned d0 with the saved argument and returns zero on mismatch.
- `return_d1_in_d0`: Copies d1 into d0 and returns immediately, so the caller receives the value of d1 as the result.
- `call_icon_vector_4c_compare_result`: Loads two arguments from the stack, pushes a third stack argument plus both onto the stack, calls the library vector at offset 76, compares the returned d0 with the saved argument and returns zero on mismatch.
- `return_d1_in_d0_2`: Copies the longword in d1 to d0 as the return value and returns to the caller; library code.
- `unsigned_32bit_divide_d1_by_d2`: Divides the longword in d1 by d2, using a shift-subtract loop when d2 exceeds 65535, and returns quotient in d0 and remainder in d1.
- `shift_subtract_divide_loop`: Runs a 16-iteration shift-subtract division of d1 by d2, leaving the quotient in d1 and remainder in d0, then returns.
- `mul32x16_shifted_product`: Multiplies the 32-bit value in d0 by the 16-bit value in d1 using partial products, returns the shifted 32-bit result in d0 and preserves d2.
- `format_string_via_rawdofmt`: Loads the format string and argument pointer from the stack, then calls exec RawDoFmt through the library base in a6 to format output.
- `store_d0_byte_to_a3_postincrement`: Writes the low byte of d0 to the address in a3 and advances a3 by one byte, then returns to the caller.
- `strlen_count_until_null`: Loads the string pointer from the stack into a0, scans bytes until a null terminator using dbeq with d0 counting down from 255, then returns the inverted count in d0.
- `strchr_find_byte_in_string`: Loads a string pointer and a target byte from the stack, scans bytes until null or a match, and returns the pointer to the matching byte or zero in d0.
- `return_zero_branch_to_shared_exit`: Sets d0 to zero and branches to the shared exit path of the string search routine, returning a null result to the caller.
- `copy_string_until_nul`: Loads two pointers from the stack, copies bytes from the source to the destination one at a time, and returns after writing the terminating zero byte.
- `skip_string_to_terminator`: Loads two pointers from the stack, advances the first pointer past bytes until a zero byte is found, steps back one, then branches to shared code.
- `compare_strings_with_length_limit`: Loads two pointers and a length from the stack, compares bytes one by one, returns zero on a full match or when both end, otherwise nonzero.
- `return_one_or_minus_one_flag`: Loads 1 into d0, then branches conditionally to a shared exit; the fall-through loads -1 into d0 and returns, giving a signed flag in d0.
- `scan_bytes_until_zero_or_count`: Loads a0 and a1 from the stack arguments, loads a count from the stack into d0, then scans bytes at a0 until a zero byte or the count runs out.
- `decrement_a0_and_call_shared_exit`: Subtracts one from address register a0, branches to a shared subroutine, then returns to the caller.
- `copy_bytes_until_nul_or_count`: Decrements d0 and, while it stays non-negative, copies bytes from (a1) to (a0) until a zero byte is copied, then pads remaining count with zero bytes in (a0).
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `init_self_referential_list_header`: Loads the pointer argument from the stack into a0, stores a0 at (a0), adds four to that stored value, clears the next field and points the third field back at a0.
- `draw_icon_with_scaled_offsets`: Saves d2-d3/a2, calls the library twice with the icon pointer and its fields, scales width and height values via another routine, and returns one on success or zero on failure.
- `icon_tail_call_with_saved_regs`: Saves d2, d3 and a2 on the stack, loads three arguments from the caller's stack into d2, d3 and a2, then branches to another library routine.
- `icon_validate_chain_nodes`: Walks a linked list through a2, calling a library check on each node and its offset field, returning 1 if the chain ends cleanly or 0 on a failed check.
- `icon_return_zero_restore_regs`: Sets d0 to zero as the return value, restores d2, d3 and a2 from the stack and returns to the caller.
- `create_icon_from_image_spec`: Saves d2-d5/a2, reads four stack arguments, allocates a 20-byte icon, fills it from the image spec via library calls, and returns the icon pointer or -1.
- `return_minus_one_restore_regs`: Loads -1 into d0 as the failure return value, restores d2-d5 and a2 from the stack, and returns to the caller.
- `load_three_args_and_branch`: Saves d2-d6/a2, loads three stack arguments into d2, d3 and d4, clears d6, then branches into the shared icon creation code.
- `icon_alloc_icon_header_and_chain`: Pushes size 65536 and a small immediate, saves d3 and d2, calls an allocation routine, stores the result at offset 56 of the stack frame, then unwinds and branches based on d6 and a saved value.
- `icon_build_icon_from_definition`: Copies a definition pointer into a2, calls allocation and conversion routines using d2-d4, stores the resulting icon pointer, walks a linked list of definitions and returns d6 or -1 on failure.
- `icon_return_failure_minus_one`: Loads -1 into d0 as the failure return value, restores d2 through d6 and a2 from the stack, and returns to the caller.

### translator.library

- `open_translator_library`: Saves d3/a2/a6, reads exec base from address 4, calls AddLibrary with version 42 and name pointers, then stores exec base and a0 into the new library base and calls MakeLibrary, returning -1 on success or 0 on failure.
- `open_translator_library_gate`: Tests the word at offset 32 of a6; if zero, increments it and clears bit 5 of the byte at offset 14 of a6, then returns a6 in d0.
- `close_translator_library_gate`: Returns zero in d0, decrements the word at offset 32 of a6, and if it reaches zero with bit 5 of the byte at offset 14 set, calls the routine at offset 112 with a1 saved.
- `check_translator_open_flag`: Saves d3, a3 and a6, sets a3 to the library base, loads the global a6, tests the word at offset 32 of the base and sets bit 5 of byte 14 if nonzero, then returns zero in d0.
- `unlink_node_and_free_memory`: Unlinks the node at a3 from its list by patching the next and previous pointers, saves the value at offset 38, then calls the library memory-free vector with the node and its size before returning the saved value in d0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `parse_translator_format_string`: Saves registers, fills a stack buffer with spaces, sets up a parse frame from a0 and a1, then scans the format string for hash and space characters, calling helper routines to process each field.
- `copy_d0_to_d1_and_branch`: Copies the word in d0 into d1, then branches to another routine in the library; it does not return to its caller itself.
- `clear_d1_call_cleanup_and_return_d1`: Sets d1 to zero, calls a library routine, clears the byte before a1, computes a1 minus the value at offset eight of a5 into d0, restores registers d2-d7 and a2-a6, then returns d1 sign-extended in d0.
- `walk_a5_list_and_dispatch`: Loads the pointer at offset zero of a5 into a6, checks it for null, advances a6 through the list, calls two library routines, and branches based on the returned condition codes.
- `translator_library_init_and_dispatch`: Loads the library base from offset four of a5, dereferences it into a6, points a6 at an offset table, calls two library routines, then tests d7 and branches based on its sign.
- `translator_parse_sequence_with_cleanup`: Calls a routine and branches if nonzero, saves a2 and a3 on the stack, calls three more routines checking each result, then restores a2, discards a3 and returns.
- `translator_pop_regs_and_check_delimiter`: Pops a3 and a2 from the stack, reads a byte from a6 into d7, compares it against two delimiter values and branches accordingly, otherwise jumps to another routine.
- `lookup_char_class_and_advance_a6`: Reads the byte at a2, indexes the word table at a4 by twice its value, and if bit 7 is clear and bit 1 set uses class 91, else class 65 plus one; shifts the class by two, loads the longword from a6 plus that offset into a3, adds it to a6 and returns.
- `match_bracket_expression_against_a2`: Scans bytes at a6 for an opening bracket, then compares each following byte against the string at a2 until a closing bracket or a mismatch, returning with a3 advanced past the compared bytes.
- `backward_match_until_class_bit`: Decrements the swapped d6 index, and while non-negative reads the byte at a6 plus d6, looks up its class word at a4, and compares it against bytes read backward from a2 until a class bit is set or a mismatch, then returns.
- `set_condition_codes_and_return`: Library routine that loads the condition code register with the immediate value 4 and returns immediately to its caller.
- `check_translator_flag_via_subroutine`: Library routine that sets d7 to 1, calls a subroutine, branches if the result is nonzero, otherwise clears the condition codes and returns.
- `scan_translation_string_until_match`: Library routine that reads bytes from a6 indexed by d6, advances d6, stops at an equals sign, tests a4 table bits, and compares each byte against the caller's string.
- `set_d7_ff_call_check_and_clear_ccr`: Sets d7 to minus one, calls the routine at offset 398, branches back if its result is nonzero, otherwise clears the condition codes and returns.
- `scan_back_for_backslash_or_backtick`: Copies d6 to d7, decrements then increments d7, reads a byte from a6 indexed by d7 into d0, and compares it against backslash and backtick, returning when d7 reaches zero.
- `compute_offset_from_a1_and_a5_base`: Subtracts the longword at offset 8 of a5 from a1 into d0, adds d7, compares with d2, and if less masks d6 to a word, adds it to a6 and pushes d7 before branching.
- `copy_bytes_and_check_escape_flag`: Copies d7+1 bytes from a6 to a1, then looks up the last copied byte in the a4 table and sets or clears bit 0 of byte 20(a5) depending on the table flag and whether the next source byte is 96.
- `scan_back_for_high_bit_char`: Scans backwards from a1 through bytes, looking up each in the a4 table until one has bit 11 set, then writes a zero byte and computes a0 minus the value at 12(a5) into d0, d1 and d7.
- `dispatch_by_character_index`: Searches the a3 character table for d5, computes an index from the match position, loads a code pointer from the a3 table at that index and jumps to it.
- `test_d7_sign_and_predecrement_a2`: Tests the sign of the word in d7 and branches if negative; otherwise loads the byte before a2 into d1 and decrements a2, then branches onward.
- `fetch_byte_index_table_word`: Loads the byte at a2 into d1 and increments a2, zero-extends it into d4, doubles it, then reads the word from the table at a4 indexed by d4 into d4 and returns.
- `call_fetch_then_test_d4_bit6`: Calls the byte-fetch routine, then tests bit 6 of d4 and returns, leaving the result flags for the caller.
- `wait_until_d4_bit5_clear`: Calls a library subroutine, then repeatedly tests bit 5 of d4 and calls the subroutine again while that bit stays set; finally sets the condition codes to indicate failure and returns.
- `check_d4_bit3_then_bit5`: Calls a library subroutine, tests bit 3 of d4 and returns early if clear, otherwise tests bit 5 of d4 and returns with the resulting condition codes.
- `wait_for_d4_bit5_or_char_49_45`: Calls a library subroutine, loops while bit 5 of d4 is set, then compares the returned d1 byte against the characters 49 and 45, setting condition codes to failure and returning.
- `set_condition_codes_and_return_2`: Loads the condition code register with the immediate value 8 and returns immediately to the caller; it is an empty library stub that only sets flags.
- `parse_translator_escape_sequence`: Repeatedly calls the character fetch routine and compares the returned byte in d1 against letters E, S, D, R, L, Y, N, G, testing bit 7 of d4 to validate an escape sequence in translator library code.
- `match_ngs_escape_sequence`: Calls the character fetch routine three times, checking d1 for N then G then S while testing bit 7 of d4, and sets condition codes to 4 before returning when the sequence matches.
- `set_condition_codes_and_return_3`: Loads the condition code register with the immediate value 8, setting the overflow flag, then returns to the caller.
- `call_helper_then_test_bit4_of_d4`: Calls a shared library helper, then tests bit 4 of register d4, setting the zero flag accordingly, and returns.
- `call_helper_then_test_bit2_of_d4`: Calls a shared library helper, then tests bit 2 of register d4, setting the zero flag accordingly, and returns.
- `test_d4_bit5_after_library_call`: Calls the shared library routine at offset 958, then tests bit 5 of d4 and returns, leaving the zero flag set from that bit test.
- `test_d4_bit8_after_library_call`: Calls the shared library routine at offset 958, then tests bit 8 of d4 and returns, leaving the zero flag set from that bit test.
- `test_d4_bit5_and_add_d7_to_a2`: Calls the shared library routine at offset 958, tests bit 5 of d4, and if set adds d7 to a2 and sets the condition codes to 8 before returning.
- `test_d4_bit1_after_call`: Calls the shared subroutine at offset 958, tests bit 1 of d4, and returns to the caller without branching.
- `loop_call_until_d4_bit1_clear`: Calls the shared subroutine, tests bit 1 of d4, and repeats the call while that bit is set; otherwise adds d7 to a2, sets the condition codes and returns.
- `test_d4_bit7_after_call`: Calls the shared subroutine, tests bit 7 of d4, and if clear sets the condition codes via move to ccr before returning.
- `set_condition_codes_and_return_4`: Loads the condition code register with the value 4 and returns immediately to the caller; it is a tiny library stub that only sets flags.
- `translate_input_to_uppercase_buffer`: Reads bytes from the a0 input pointer, maps lowercase to uppercase and control characters to spaces, stores results into the a5 buffer, tracks a countdown at offset 130, and handles double-hash sequences, returning status in d0 and d1.
- `backup_input_and_pad_buffer`: Steps the a0 input pointer back twice, writes a space just before the a2 buffer position, decrements the d3 counter twice, and returns to the caller.
- `return_minus_three_in_d1`: Loads the immediate value minus three into data register d1 and returns immediately to the caller; library code with no other effect.
- `write_two_spaces_via_a2`: Stores the space character into the two bytes pointed to by address register a2, advancing a2 past both, then returns to the caller.
- `expand_escape_sequences_in_buffer`: Scans the buffer at offset 16 of a5 up to a1, checks character class bits in the a4 table, and inserts a byte after matching two-byte codes from the a2 table, updating the a5 pointer.
- `shift_string_right_insert_byte`: Copies bytes backwards from a1 down to a0, shifting the string right by one, writes byte 52 at the new start, then stores a1+1 into a5 offset 16 and restores registers.

### info.library

- `illegal_instruction_trap_stub`: Executes an illegal instruction opcode, which triggers the 68000 illegal instruction exception; it is a single-instruction trap stub with no return.
- `store_a6_d0_and_a0_globals`: Saves a6 and d0 into absolute longword globals, copies d0 into a1, stores a0 at offset 34 of that pointer and into another global, then returns.
- `info_library_call_vector_zero`: Pushes a6 onto the stack, calls the library jump-table entry at offset zero, then discards the pushed register and returns to the caller.
- `info_library_call_vector_120`: Pushes a6 onto the stack, calls the library jump-table entry at offset 120, then removes the pushed register and returns to the caller.
- `info_library_call_vector_18`: Pushes a6 onto the stack, calls the library jump-table entry at offset 18, then discards the pushed register without an explicit return here.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `info_library_call_5e2_with_saved_regs`: Pushes a1 and a6 onto the stack, calls the library routine at offset 5e2, then discards both saved registers and returns to the caller.
- `clear_bit3_in_flags_and_increment_count`: Loads the caller's pointer from the stack, clears bit 3 of the byte at offset 14, increments the word at offset 32, returns the pointer in d0.
- `info_library_test_count_and_set_flag`: Saves d2 and a2, loads the node pointer from the stack, calls a library routine, tests the word at offset 32 and sets bit 3 of the byte at offset 14 if nonzero, then calls another routine and returns zero in d0.
- `info_library_span_rect_call`: Saves d2 and a2, calls a library routine with a2, then pushes a2 minus its word at offset 16 and that word plus the word at offset 18, calls two more library routines, restores d2/a2 and returns the saved global in d0.
- `info_library_decrement_and_dispatch`: Loads the caller's pointer from the stack, decrements the word at offset 32, and if it reaches zero with bit 3 of offset 14 set, pushes the pointer and calls a routine before branching past.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `info_library_release_resources`: Saves d2/a2, then conditionally calls cleanup routines for the argument in d0, the global at d4 offset 292, the argument in d2, and four globals at d4 offsets 80, 72, 76 and 84, restoring registers and returning.
- `allocate_info_library_structures`: Saves a2-a4, calls the allocator with size and flags, stores the returned block pointer in a global, and returns zero on failure.
- `populate_info_library_struct_pointers`: Uses a2 base pointer to fill d4 globals with offsets into the allocated block, copies template data into those fields, and returns 1 after restoring a2-a4.
- `open_info_library_resources`: Saves a2-a3, opens the library file by name, stores the handle in a global, and returns an error code if any of the four resource opens fail.
- `open_icon_library_store_base`: Pushes a zero longword and the library name pointer, calls the open-library vector in a2, stores the returned base at offset 80 in absolute memory, then pops the arguments and branches based on the result.
- `open_intuition_library_store_base`: Pushes a zero longword and the intuition library name pointer, calls the open-library vector in a2, stores the returned base at offset 72 in absolute memory, pops arguments, and on failure pushes zero arguments and a small word before branching.
- `open_graphics_library_store_base`: Pushes a zero longword and the graphics library name pointer, calls the open-library vector in a2, stores the returned base at offset 76 in absolute memory, and on failure calls the a3 vector with arguments 5, 0 and 0 before returning zero.
- `return_one_pop_a2_a3`: Sets d0 to 1 as the return value, then pops a2 and a3 from the stack and returns to the caller.
- `push_a2_check_library_base_flag`: Pushes a2 onto the stack, loads a1 with the library base offset, tests the longword there and branches away if it is nonzero, otherwise branches elsewhere.
- `load_node_and_check_type_byte`: Loads a0 from the pointer at a1, compares the byte at offset 61 of that structure with 7, and branches away if unequal, otherwise branches elsewhere.
- `load_node_and_check_offset_a4`: Loads the pointer at a1 into a0, loads the longword at offset 56 of that node into a2, tests the longword at offset 164, and if nonzero reloads a0 and returns the longword at offset 164 in d0.
- `check_node_type_3d_and_load_1ba`: Loads the pointer at a1 into a0, compares the byte at offset 61 with one, and if equal follows the pointer at offset 76 and returns the longword at offset 442 in d0.
- `return_a2_or_follow_offset_4c`: Moves a2 into d0, and if a2 is nonzero loads the pointer at offset 76 of a2 into a0 before branching onward; otherwise returns zero.
- `return_zero_library_stub`: Library routine that sets d0 to zero as its return value, pops a2 from the stack and returns to the caller.
- `info_library_open_disk_info`: Library code that calls an internal setup routine, allocates a structure, stores the result at offset 152, checks a flag at offset 61 and branches to the disk.info path.
- `info_library_query_disk_info`: Library code that copies a field into a local buffer, calls two library routines with that buffer, clears a global word and returns a value from offset 442 when the flag matches.
- `allocate_info_library_buffers`: Sets up an a6 stack frame, saves d2-d4/a2-a5, loads two arguments into d3 and d2, loads three global pointers into a3/a4/d4, then calls the allocator with size a fixed address and stores the result in a global.
- `allocate_second_info_buffer`: Pushes size a fixed address and a fixed block size, passes d3 to the allocator, stores the returned pointer through a4, cleans the stack and branches based on the result.
- `check_info_header_and_call`: Loads the pointer stored at d4 into a1, dereferences it into a0, checks byte 61 for value 1, then pushes a global and d2 and calls another routine, testing the result.
- `flush_d2_buffer_and_return`: Tests d2; if zero returns immediately, otherwise pushes the a4 pointer and d2, calls the library routine at offset 48, masks d0 to a word, pops eight bytes and branches based on the result.
- `allocate_info_buffer_and_set_size`: Calls library routines to allocate a buffer, increments a counter at offset 100 of the a3 structure, clamps it to 256, then allocates a second buffer and stores it at offset 96, returning success or zero.
- `copy_info_fields_and_return_one`: Calls a library routine with the buffer at offset 96, copies fields from the d4 and a4 structures into offsets 104 and 112 of the a3 structure, restores registers and returns one.
- `info_library_open_init`: Sets up an a6 stack frame, saves d2 and a2-a4, clears several library globals, stores the caller's argument in a global, then calls the open routine and branches on its result.
- `info_library_alloc_open_buffer`: Pushes size and tag arguments, calls the library allocator with a stack buffer pointer, stores the result in a global, cleans the stack and branches on failure.
- `info_library_free_open_buffer`: Calls the library free routine on the stack buffer, tests the result, cleans the stack and branches to the error path if it failed.
- `allocate_memory_into_a3_and_d2`: Calls a library routine, stores the returned pointer in a3 and copies it to d2, branching if it is non-zero and otherwise jumping to the caller's failure path.
- `allocate_forty_four_bytes_store_in_a4`: Pushes a size of 68 bytes, calls an allocation routine, stores the result at the address in a4, pops the argument, and branches on failure to a cleanup path.
- `dispatch_info_library_message`: Pushes caller arguments, calls comparison and setup routines, checks a global flag, then invokes the callback pointer in a2 with a status and buffer.
- `open_info_library_and_verify`: Calls a library open routine with a name pointer, stores the result in d2, and if non-zero calls a verify routine with that handle and a stack argument, returning the result.
- `close_info_library_and_release`: Checks a global flag and d2 handle, then calls several library cleanup routines with a3 and stack arguments before restoring registers and returning.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_372_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_366_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_462_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_234_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_270_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `build_info_release_request`: Builds a request structure on the stack with version and size fields, calls a library routine with it, and if successful calls another routine with the handle and a negative argument.
- `dispatch_info_node_type_jump_table`: Saves a2-a3, loads the node pointer and a value from the stack, reads the node type word at offset 38, and jumps through a table for types 0 to 17.
- `return_one_restore_a2_a3`: Sets d0 to 1 as the return value, restores a2 and a3 from the stack, and returns to the caller.
- `toggle_info_flag_by_node_type`: Saves a2, loads the library base and node pointer, maps node type 6 to 19 to a bit mask, XORs it into the flag at offset 112, then calls another routine.
- `store_d1_in_library_global_and_call`: Loads the library base from absolute address 100, stores d1 at offset 112 of it, then calls the routine at absolute address 276 with a1's offset 44 value and returns a2 popped from the stack.
- `info_library_open_and_validate`: Library open routine that allocates a stack frame, calls several library helpers to set up and check the info device, validates the result and returns a boolean in d0.
- `check_info_state_and_branch`: Loads the pointer at a3 into a0, compares the byte at offset 61 with 1, and branches to one of two targets depending on the result.
- `check_info_file_and_apply_attributes`: Calls library helpers to open and parse the info file, checks a flag byte at offset 61, compares file attributes, formats a name string, and returns d2 as success or zero.
- `set_four_info_attributes_with_default`: Calls a helper four times with the same file handle and attribute pointers, passing -1 as value, and returns 1 if all succeed or 0 otherwise.
- `set_second_info_attribute_default`: Pushes -1 and the second attribute pointer with the saved file handle in d2, calls the helper via a2, and compares the result against -1.
- `call_info_vector_with_minus_one`: Pushes -1, the longword at absolute address 316 and d2 onto the stack, calls the function pointer in a2, compares the result with -1, discards twelve stack bytes and branches based on the comparison.
- `call_info_vector_second_entry`: Pushes -1, the longword at absolute address 320 and d2 onto the stack, calls the function pointer in a2, compares the result with -1, discards twelve stack bytes and returns zero when unequal.
- `call_absolute_vector_return_one`: Pushes d2, calls the routine at absolute address 276, sets d0 to one, discards four stack bytes, restores d2, d3 and a2 from the stack and returns.
- `draw_info_window_border_and_text`: Pushes many coordinate and color words onto the stack and calls the library draw helper repeatedly to paint the info window border, panels and text lines, then restores a2-a3 and returns.
- `draw_vertical_dashed_line_segments`: Pushes coordinate pairs and a color word for each of seven segments and calls the draw helper to render a vertical dashed line, then releases the stack and returns.
- `draw_horizontal_dashed_line_segments`: Pushes coordinate pairs and color words for seven segments and calls the draw helper to render a horizontal dashed line, then releases the stack and returns.
- `info_library_draw_first_icon_set`: Saves d2/a2, loads the library base from the stack, then calls the drawing routine nine times with different coordinate and color arguments before restoring registers and returning.
- `info_library_draw_second_icon_set`: Saves d2/a2, loads the library base from the stack, then calls the drawing routine nine times with a second set of coordinate and color arguments before restoring registers and returning.
- `info_library_check_version_and_set_error`: Saves d2/a2-a3, loads the library base and a value from the stack, reads a byte at offset 61 of the library structure, and branches based on whether it equals four, setting d1 to 131 otherwise.
- `check_info_list_head_and_report`: Loads the info structure pointer from a3, compares its list head plus four against the head, and on mismatch pushes error arguments and calls the library error reporter.
- `allocate_info_node_and_link`: Computes a size from d1 plus eight, calls the library allocator with tag and size, links the new node into the a2 list, and optionally frees the old node.
- `create_info_list_from_template`: Allocates a block of size 65536 with tag fourteen, stores it at offset 234 of the global info structure, and returns zero if allocation failed.
- `init_info_node_list_and_clear_field`: Reads the node pointer from the structure at a3, advances its field at offset 234 by four, stores that pointer at offsets 0 and 8, clears offset 242, then walks the list at global offset 92 until empty.
- `allocate_info_node_block`: Pushes size 65536, a small immediate and d2, calls the allocator, stores the returned pointer in a2 and d3, pops the arguments and branches depending on the result.
- `build_info_nodes_from_table`: Allocates a 65536-byte block, stores it at offset 10 of a2, calls two library routines with the table entry and block, advances a4 by four, and repeats until the table entry is zero, then stores the head pointer at offset 238.
- `check_head_node_and_notify`: Compares the list head node's first and second fields against the stored tail pointer; if equal it calls the notify routine with the object's fields, otherwise it advances the tail pointer and calls the notify routine.
- `advance_tail_and_notify`: Pushes the global notify values and the tail node's field, calls the notify routine, updates the list tail pointer to the next node, then calls the notify routine again and returns.
- `check_second_node_and_notify`: Compares the list head node's second field and a pointer four bytes past the head against the stored tail pointer; if equal it calls the notify routine with the object's fields, otherwise it advances the tail pointer and calls the notify routine.
- `pop_info_node_and_restore_previous`: Pushes two library globals and the current info node's field onto the stack, calls two library routines, then copies the node's first longword into its parent's field at offset 238 before calling the cleanup routine and restoring a2-a3.
- `push_new_info_node_with_two_allocations`: Sets a flag in the library base, checks the node list, allocates two blocks of size 14 and 128, links the second into the first at offset 10, inserts the new node into the list and calls the notify routine.
- `pop_info_node_and_relink_neighbour`: Sets a flag in the library base, checks the node list, calls two library routines, then chooses the previous or next node depending on whether the current node is the list tail and calls the notify routine with the chosen node.
- `advance_info_list_head_and_notify`: Reads the current list head from the library base, calls the notify routine on it, then if the next pointer equals the head plus four, reloads the head and stores it back before calling the update routine with a2.
- `release_info_node_and_free_resources`: Calls the library cleanup entry, pushes the node field at offset 46 and calls the free entry, then pushes the field at offset 62 and calls the release entry before restoring registers and returning.
- `save_registers_and_test_node_flag`: Saves d2 through d6 and a2 through a5 on the stack, loads the constant 100 into d3 and 88 into a4, then tests the longword at offset 242 of the node pointed to by a1 and branches accordingly.
- `compare_node_list_pointer_to_first_entry`: Loads the node list pointer from d3, follows two pointer fields to reach a list head, compares its address plus four against the first entry pointer, and clears a field in the a4 structure if they differ.
- `push_global_and_node_field_then_call`: Saves the global longword at address 272, pushes a field from the node structure reached via d3, calls a subroutine, sets d4 to one, loads the first list entry into a3 and branches on.
- `count_nodes_alloc_info_table`: Walks the a3 linked list counting nodes in d4, allocates a table of d4 times four bytes via library call, stores the pointer at offset 92 of the a4 structure and returns success or zero.
- `info_lib_alloc_first_entry_buffer`: Pushes d4, calls a library routine with the info base offset 140 and a field at offset 92, clears d4, loads the list head from offset 234 into a3, then branches to the shared allocation loop.
- `info_lib_alloc_entry_buffer_loop`: Calls a routine on the node field at offset 10, defaults the size to 4, allocates that many bytes, stores the pointer in the table indexed by d4, and loops until allocation fails.
- `info_lib_copy_entry_and_advance`: Copies the current table entry and node field via two library calls, increments d4, advances a3 to the next node, and returns 1 when the list end is reached, otherwise jumps back to the allocation loop.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_info_panel_by_mode`: Saves registers, loads the info structure pointer, reads the mode byte at offset 61 and jumps through a table for modes 1 to 6, otherwise falls through to the exit path.
- `restore_registers_and_return`: Pops d2 and a2 through a5 from the stack and returns immediately, restoring the saved registers of the caller.
- `allocate_info_panel_bitmaps`: Calls the graphics allocator four times for panel bitmaps, stores the results in globals, and returns zero if any allocation failed.
- `open_four_info_resources_and_check`: Calls the library open function four times with tag values 2c, 24, 74 and 74, stores each returned handle in globals and a4, then tests all four for zero and branches to failure or success.
- `dispatch_on_info_byte_3d`: Loads the info base pointer from absolute address 58, reads byte at offset 3d into d0, bounds-checks it between 1 and 5, then jumps through a word table indexed by d0.
- `restore_registers_unlink_and_return`: Restores d2, d3, a2, a3 and a4 from the stack frame at offset -2c from a6, releases the a6 frame link and returns to the caller.
- `check_info_flag_and_clear_byte`: Saves d2-d3, loads the argument from the stack into d2, tests a longword flag in the info base; if zero it loads a pointer from a global and clears one byte there, then branches onward.
- `call_info_open_and_compare_result`: Pushes info base fields and a string, calls a library routine, then calls another with -1 and the info pointer, compares the result to -1 and returns zero if equal.
- `restore_d2_d3_and_return`: Pops d2 and d3 from the stack and returns to the caller, restoring the saved registers only.
- `open_four_info_streams`: Saves d2-d3/a2, reads the argument into d2, and unless a status byte equals 4, writes stream mode words 118, 125, 119 and 119 into four library stream structures before calling a routine with each stream and -1.
- `open_second_info_stream`: Pushes -1, the second library stream pointer and d2, calls the routine in a2, then compares the result with -1 and branches to the shared failure or success exit.
- `open_third_info_stream`: Pushes -1, the third library stream pointer and d2, calls the routine in a2, then compares the result with -1 and branches to the shared failure or success exit.
- `call_a2_with_ff_and_compare_result`: Pushes -1, the longword at absolute address 344 and d2 onto the stack, calls the routine in a2, then compares the returned d0 with -1 and returns zero if equal.
- `return_one_restore_d2_d3_a2`: Sets d0 to 1 as the return value, restores d2, d3 and a2 from the stack and returns to the caller.
- `poll_info_library_retry_loop`: Saves d2-d4 and a2, reads a shift count from the library structure, then repeatedly calls the poll and process routines, retrying until the process call succeeds.
- `return_zero_and_restore_registers`: Sets d0 to zero as the return value, then restores d2 through d4 and a2 from the stack and returns to the caller.
- `dispatch_info_library_node_by_type`: Reads the type field at offset 20 of the node in a2, compares it against 4, 64 and 512, and branches to the matching handler or the default path.
- `call_handler_with_d2_and_a2_saved`: Pushes d2 and a2 onto the stack, calls the handler at address a4, then discards the two saved registers and branches onward.
- `push_a2_call_d4_return_zero`: Pushes a2 onto the stack, calls the routine at offset d4, sets d0 to zero as the return value, then branches onward without restoring a2.
- `call_b8_and_d4_return_one`: Pushes d3, calls the routine at offset b8, discards that argument, pushes a2, calls the routine at offset d4, sets d0 to one, then restores d2-d3/a2 and returns.
- `info_library_open_entry`: Saves a2, loads the caller's argument into a2, calls three library routines on it, checks a global flag byte, optionally calls another routine, then calls two more with the object's field and returns.
- `format_info_flags_into_four_fields`: Reads a flags longword from the library base, masks and inverts its low bits, then calls a formatting routine four times with bit values and field indices zero through three.
- `build_info_string_with_flag_offsets`: Sets up a small stack buffer and constants, then calls the formatting routine with two or three arguments depending on whether the flag parameter is zero.
- `format_and_draw_info_text_lines`: Builds a formatted string on the stack from the caller's arguments, then calls two drawing routines to render the text at computed positions.
- `load_first_arg_and_zero_d1_a1`: Loads the longword at offset four from the stack into d0, sets d1 to zero, copies d1 into a1, then branches to another routine.
- `scan_string_for_terminator`: Advances a1 by one byte and tests the byte at a1 plus d0 until a zero byte is found, then returns the resulting a1 in d0.
- `copy_string_until_nul_2`: Copies bytes from the source pointer in a1 to the destination pointer in a0, incrementing both, until a zero byte is copied, then returns.
- `load_two_args_and_zero_d1_tailcall`: Loads a0 from the first stack argument, a1 from the second, d0 from the third, sets d1 to zero, then branches to another routine.
- `copy_bytes_until_count_reached`: Copies bytes from the address in a0 to the address in a1, incrementing both pointers and d1 until d1 reaches d0, then returns.
- `load_two_args_tailcall_copy`: Loads a0 from the first stack argument and a1 from the second, then branches to the byte-copy routine without returning here.
- `compare_strings_until_mismatch`: Advances a0 and a1 past matching bytes, stops at a null terminator, and returns nonzero in d0 when the final bytes differ.
- `strlen_from_stack_argument`: Reads a pointer from the stack, scans bytes until a null terminator, and returns the count including the terminator in d0.
- `format_and_emit_info_string`: Saves registers, computes a length via a helper, then calls three library routines with the object at offset 50 and adjusted arguments to output text.
- `info_library_call_with_stack_buffer`: Builds a 12-byte stack buffer, passes it with a global pointer and the fifth argument to one library routine, then passes the four saved arguments plus that buffer to another routine before restoring registers and returning.
- `info_library_load_indexed_byte`: Saves d2 and a2-a3, loads a pointer and index from the stack, computes a byte address by shifting the index left twice, sign-extends the byte into d1, sets d2 and a2 to one, then branches onward.
- `info_library_shift_bytes_and_terminate`: Copies bytes from a2 plus a3 into a2 minus one plus d0 while incrementing a2 until it exceeds d1, then writes a zero terminator at d1 plus d0 and restores registers.
- `init_info_window_and_open`: Fills two caller-supplied structures with window geometry and defaults, opens the info window via the library open call, and returns true on success or false if opening failed.
- `close_info_window_return_true`: Pushes a zero and the window pointer from offset 62 of a3, calls the close routine, then returns 1 after restoring d2-d3 and a2-a3.
- `set_info_window_pen_and_rect`: Calls the pen-set and clear routines on the window at offset 50 of a2, then draws a rectangle using the passed coordinates reduced by ten.
- `format_string_via_rawdofmt_2`: Saves a2-a4 and a6, loads the format string and argument pointer from the stack, loads a format template from pc-relative memory, calls exec.RawDoFmt through a6, then restores registers and returns.
- `store_byte_to_a3_and_advance`: Writes the low byte of d0 to the memory pointed to by a3 and increments a3 by one, then returns to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.

### version.library

- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `store_a6_and_a0_into_library_base`: Copies d0 into a1, then stores a6 at offset 34 and a0 at offset 38 of that structure, and returns.
- `version_lib_increment_open_count`: Increments the 16-bit open counter at offset 32 of the library base in a6, clears bit 3 of the byte at offset 42, and returns the base pointer in d0.
- `version_lib_decrement_open_count`: Sets d0 to zero, decrements the 16-bit open counter at offset 32 of the a6 library base, and if it reaches zero with bit 3 of offset 42 set, calls the close routine.
- `version_lib_open_begin`: Saves d2 and a5-a6 on the stack, builds an a5 frame, loads the library base from offset 34 into a6, and if the open counter at offset 32 is zero sets bit 3 of offset 42 before returning zero in d0.
- `close_version_library_and_free_base`: Saves the library base from offset 38 of a5, calls the close and free vector routines through a6 with a1 set to the base and size fields, then returns the saved base in d0.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.

### Populous (1989)(Electronic Arts)[m Dave Soft].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `bootblock_data_bytes_not_code`: These three words are data bytes in the bootblock, not executable code; the ori.b instructions are meaningless immediates and no branch or call occurs here.
- `bootblock_data_table_not_code`: These bytes are a data table in the bootblock, not executable code; the ori.b and bit instructions are meaningless immediates with no branch or call.
