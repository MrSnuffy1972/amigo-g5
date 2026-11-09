# Game 831 (populous (1989)(electronic arts)[cr flint firforge]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00831). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 37 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 350 raw-word lines |
| Functions named and described | pass, 525 of 525 named (0 author symbols), 525 of 525 described (110 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 708 of 708 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11131 of 11898 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 650141 |
| code | 38826 |
| data | 195053 |
| padding | 10956 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| loadwb | 12 | 536 | 30 |
| wait | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Populous (1989)(Electronic Arts)[cr Flint Firforge].adf:populous | 3 | 4352 | 145 |
| Populous (1989)(Electronic Arts)[cr Flint Firforge].adf:populous.prg | 3 | 1760 | 9 |
| icon.library | 10 | 1375 | 109 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2597 | 167 |
| version.library | 2 | 41 | 7 |
| Populous (1989)(Electronic Arts)[cr Flint Firforge].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- populous (1989)(electronic arts)[cr flint firforge]/Populous (1989)(Electronic Arts)[cr Flint Firforge].adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.70)
- populous (1989)(electronic arts)[cr flint firforge]/Populous (1989)(Electronic Arts)[cr Flint Firforge].adf: blocks 88-96 (bytes 45056-49664) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.99)
- populous (1989)(electronic arts)[cr flint firforge]/Populous (1989)(Electronic Arts)[cr Flint Firforge].adf: blocks 99-109 (bytes 50688-56320) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.48)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `disable_audio_dma_and_clear_flag` (populous): Loads the base of the sound state area into a3, clears the byte at offset 766, then writes 15 to DMACON and 255 to ADKCON to disable DMA and audio interrupts before returning. Kind: disk hardware/sync check. Pass path: the branch not taken by the check (the code following the call site's logic).

## Functions


### type

- `allocate_type_node_and_copy_name`: Calls library routines through a5 to find the highest type id, allocate a node, fill its id and name fields, copy sixteen longs into the node, and return zero or minus one.
- `return_d1_in_d0_and_branch`: Moves d1 into d0 as the return value and branches forward into another routine instead of returning here.

### endcli

- `endcli_type_node_setup`: Same code as the type routine but entered from the endcli program: allocates a type node, fills its fields, copies sixteen longs, and returns zero or minus one.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0 and branches to another routine, so the caller receives d1 as its return value.

### loadwb

- `loadwb_open_dos_and_parse_args`: Opens dos.library, allocates a task structure, parses the command line into argument pointers, opens the named file and starts the loaded program.
- `loadwb_cleanup_and_reply`: Finds the current task, restores its saved stack frame, closes the opened file and library, frees memory and replies the message before returning.
- `load_workbench_tool_and_start_task`: Opens workbench and icon libraries, allocates and patches a task structure, resolves the tool name, starts the workbench task and releases the libraries before returning.
- `compare_two_strings_lexicographically`: Compares the two null-terminated strings passed on the stack byte by byte, returning zero when equal, minus one when the first is less, and one when greater.
- `build_linked_list_of_allocated_nodes`: Walks a chain of index entries, allocates eight-byte nodes, stores each node's second field from a called routine and links nodes through their first longword, returning the head node.
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
- `match_workbench_version_string`: Compares the byte at offset 20 of the structure in a2 against the character '2', then compares the structure bytes with the Workbench release string; on a full match it copies spaces into the name field.
- `fill_name_field_with_dot_and_spaces`: Writes a dot followed by four spaces into the five bytes starting at offset 17 of the structure pointed to by a2, then branches to the shared exit.
- `compare_structure_bytes_to_version_string`: Loads a2 into a0 and a program-counter-relative string into a1, then compares bytes from the structure against the string, branching on the first mismatch or on an empty string.
- `load_a2_pc_relative_and_jump_via_stack`: Loads a2 with a program-counter-relative address, pops a0 and a1 from the stack, pushes the value at address zero onto the stack and returns through it.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_function_with_three_stack_arguments`: Saves d2 and d3, loads two arguments from the stack, pushes a pointer to the third argument plus both values, calls the target function and cleans up the stack.
- `format_string_via_rawdoffmt_and_dispatch`: Calls exec RawDoFmt to format a string into a 140-byte stack buffer, measures its length, then passes the buffer and length to another routine.
- `store_byte_to_a3_postincrement`: Writes the low byte of d0 to the memory pointed to by a3, then increments a3 by one and returns.

### wait

- `allocate_and_initialize_memory_node`: Calls library routines through a5 to find the largest free block, allocate a node, fill its header and table entries, copy sixteen longs into the node, and return zero or minus one.
- `copy_d1_to_d0_and_branch_2`: Moves d1 into d0 and branches forward to another routine's code, transferring control without returning here.

### Disk-Validator

- `disk_validator_entry`: Program entry that sets up a4-relative pointers, calls library routines through a5, and runs the disk validation loop using a0 and a1 work areas.

### Populous (1989)(Electronic Arts)[cr Flint Firforge].adf:populous

- `jump_to_main_entry`: Single jump instruction that transfers execution to the main program code at a pc-relative offset, acting as the executable entry stub.
- `game_init_open_screens`: Opens the demo picture and several screens via library calls, stores the returned handles in a4-relative globals, then runs setup routines and branches on the argument count.
- `push_global_offset_38_and_call`: Opens a stack frame, loads the pointer stored at offset -32124 from a4, pushes the longword at offset 38 of that structure, calls a subroutine, cleans the stack and returns.
- `init_sound_and_enable_interrupts`: Calls two setup routines, passes four size values with their pointers to a routine, then writes 1040 to the INTENA hardware register to enable interrupts before returning.
- `create_and_configure_task_structure`: Allocates a 40-byte structure, opens a file, creates a task with name and stack, fills fields at offsets 14, 18, 10, 28, 36 and 32, then passes the task to another routine.
- `build_eight_sprite_pointer_table`: Allocates a 1152-byte graphics buffer, stores its pointer in a global, then fills eight consecutive longwords with pointers spaced 144 bytes apart and sets two more globals from the buffer base.
- `set_sound_volume_399_and_play`: Stores the value 399 into a global sound variable at offset -32116 from a4, then calls the sound routine to apply it.
- `free_sprite_buffer_1152_bytes`: Pushes the global graphics buffer pointer and its 1152-byte size onto the stack and calls the memory free routine to release it.
- `flush_two_audio_channels`: Pushes two longword pointers from offsets in a4 onto the stack and calls the same subroutine twice, adding four to the stack after each call, then returns.
- `write_word_to_stream_with_handle`: Sign-extends the word argument at offset eight in the frame, pushes it and a longword from an a4 offset, calls a subroutine, cleans eight stack bytes and returns.
- `call_shared_subroutine_no_args`: Opens an a5 frame, calls the subroutine at the given program counter offset with no arguments, releases the frame and returns to the caller.
- `find_index_and_dispatch_by_value`: Scans two longword entries in the a4-relative table for a match with the argument, calls the paired routine on match, and stores the found index in a4-relative memory.
- `swap_two_globals_and_call_compare`: Swaps the two longwords at a4-relative offsets, then pushes minus one and both values and calls the comparison routine at 776.
- `open_libraries_and_read_screen_flags`: Opens graphics and intuition libraries, stores their bases in a4-relative globals, exits on failure, and saves the screen mode flag bits into a4-relative memory.
- `poll_input_until_menu_choice`: Calls the input routine repeatedly, extending d0 to long, until it returns 1, 2 or 3, then calls the display routine with the a4 globals and returns.
- `wait_for_menu_selection`: Clears the selection variable, draws the load.pic screen, polls input up to an address times, and returns 1, 2 or 3 based on the cursor position in the a4 globals.
- `run_level_select_menu_loop`: Initialises menu tables and callbacks in a4, draws the screen, then loops reading input and drawing two menu rows until the selection flag clears, then returns.
- `update_two_player_panel_counters`: Runs the per-frame panel update: toggles a flag, redraws two player counters, decrements digit counters, and processes sixteen panel entries with digit comparisons and drawing calls.
- `branch_into_previous_routine`: Contains only an unconditional branch that jumps backward into the middle of the preceding routine, so it executes no instructions of its own.
- `draw_and_randomize_panel_digits`: Draws the panel digits at two vertical offsets, fills eighteen digit slots with random values masked to three bits, and returns one or zero based on the flag checks.
- `fill_terrain_height_and_type_arrays`: Loops over the count argument, calls a random generator twice, stores height as remainder modulo 13 plus 232 and type as remainder modulo 9 into two word arrays indexed by the loop counter.
- `adjust_column_offsets_and_return_first`: Loops over the requested number of columns, reads two word arrays indexed by a4, adjusts each entry by half its value when negative, stores results back and returns the first adjusted value in d0.
- `run_demo_playback_loop`: Loads the demo picture, sets up display, generates terrain, then repeatedly draws and updates frames until the demo flag or counter ends, restoring the previous screen state.
- `init_four_player_slot_pointers`: Loops a counter from zero to three, storing computed base-plus-offset pointers into two arrays indexed by the counter, calling a setup routine with both pointers and two fixed buffers, then copies four pointer values into the active player slots.
- `open_and_load_landscape_file`: Opens a file with access mode 1005, loads 32000 bytes into the landscape buffer, closes the file, frees the handle, and returns zero on success or minus one if opening failed.
- `load_font_land_and_sprite_files`: Opens and loads three files in turn into the font, land and sprite buffers, closing and freeing each handle, and calls a setup routine between the first and second loads.
- `open_file_and_read_into_buffer`: Opens the file named by the first argument, reads 40000 bytes into the second argument's buffer, copies 64 bytes from a global table into the buffer, then closes the file.
- `blit_sprite_rows_into_planar_bitmap`: Reads five words per row from a sprite table, rotates them by the low nibble of the x coordinate, and masks/merges them into four bitplanes of the destination bitmap for the given row count.
- `blit_sprite_rows_into_screen_planes`: Reads five words per row from a fixed sprite table, rotates them by the low nibble of d0, and masks/merges them into four screen bitplanes at the offset given by d1 and d2.
- `call_blit_with_stack_args`: Saves registers, loads a0, d0, d1 and d2 from the stack frame arguments, calls the blit routine, then restores registers and returns.
- `blit_sprite_rows_to_bitplanes`: Uses d0, d1 and d2 to locate a sprite in the graphics table and masks or merges its rows into four interleaved bitplane buffers at a0.
- `swap_sprite_record_halves`: Loops over 1680 records at the graphics table, swapping the low and high words of three longwords and rotating the remaining words within each record.
- `scale_map_tile_bytes_by_eight`: Loops 100 times over bytes at the map buffer plus 200, shifting each byte left by three bits and storing it back in place.
- `draw_isometric_terrain_grid`: Walks a 10 by 10 grid of map bytes, computes isometric screen coordinates from row and column, and calls the plot routine for each tile, optionally drawing a second offset copy.
- `plot_single_tile_by_index`: Reads a tile index argument, divides it into row and column, fetches its height byte from the map buffer, computes isometric screen coordinates and calls the plot routine once.
- `set_text_pointer_to_default`: Stores the address of the default text cursor into the global pointer at a fixed longword location, then returns to the caller.
- `render_text_string_to_bitmap`: Clears a bitmap buffer, then draws each character of the current text string as five-byte columns into the buffer, advancing the global text pointer and resetting it at the terminator.
- `zero_filled_data_area`: This single zero word decodes as an ori.b instruction but is a zero-filled data area referenced by address, not executable code.
- `populous_data_table_at_1fa2`: Bytes here are data words decoded as code, not executable instructions; other code refers to this table by address and it changes no registers.
- `populous_data_table_at_25e2`: This range holds data words that decode as no-op ori instructions; it is a table referenced by address, not working code.
- `branch_to_offset_2c36`: Executes a single unconditional branch instruction that transfers control forward to the code at offset 2c36.
- `zero_filled_data_word_at_2c2e`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_2c32`: Another single zero word decoding as ori.b #0,d0; a zero-filled data slot pointed to by a handler pointer, not real code.
- `open_popsam_load_and_install_interrupts`: Opens dos.library and the popsam file, reads it into allocated memory, prints error text on failure, then saves interrupt vectors and enables level interrupts before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_a0_offset_byte_and_restore_all`: Writes zero to the byte at offset 884 from a0, then pops all registers d0-d7 and a0-a6, releases the a5 frame and returns.
- `initialize_dispatch_and_free`: Disables interrupts by writing to the hardware registers, calls another routine, and then populates several global variables with values retrieved from memory before re-enabling interrupts with specific bits set.
- `compare_d0_to_ten_branch_or_clear`: Compares the low byte of d0 with ten; if not equal branches ahead, otherwise calls a subroutine and jumps to the routine that clears a byte and restores registers.
- `check_d0_equals_1_then_call`: Compares the low byte of d0 with 1; if not equal it branches away, otherwise it sets d0 to 0, calls the routine at the given offset and branches onward.
- `check_d0_equals_2_then_set_1`: Compares the low byte of d0 with 2; if not equal it branches away, otherwise it sets d0 to 1 and branches to the shared continuation.
- `check_d0_equals_3_then_set_2`: Compares the low byte of d0 with 3; if not equal it branches away, otherwise it sets d0 to 2 and branches to the shared continuation.
- `check_d0_for_four_then_set_three`: Compares the low byte of d0 with 4; if not equal it branches away, otherwise it loads 3 into d0 and branches to another routine.
- `branch_to_shared_clear_byte`: Unconditionally branches to another routine that clears a byte at offset 884 from a0; it performs no work itself.
- `cia_interrupt_ack_and_start_flag`: Saves all registers, reads the CIA interrupt flags masked by the enable register, acknowledges the vertical-blank interrupt, sets a start flag, samples the parallel port bit into a word, calls a routine and restores registers.
- `level3_interrupt_return_stub`: Executes a return from exception instruction, restoring the saved status register and program counter from the supervisor stack to resume interrupted code.
- `zero_filled_data_area_2`: Contains only zero words that decode as ori.b immediate to d0; this is a zero-filled data area referenced by address, not executable code.
- `build_level_resource_tables`: Relocates level resource pointers into tables, computes sprite dimensions and timing values, fills four object entries with code and data pointers, installs the level three interrupt vector and sets a flag.
- `disable_audio_dma_and_clear_flag`: Loads the base of the sound state area into a3, clears the byte at offset 766, then writes 15 to DMACON and 255 to ADKCON to disable DMA and audio interrupts before returning.
- `update_channel_audio_pointer`: Checks the sound-enabled flag, selects channel 3 state, and if not yet started writes the sample pointer and period into the audio hardware registers for that channel.
- `restart_channel_sample_pointer`: Points a2 at the channel sample table, writes its address and length to the audio registers, decrements the channel counter and enables the matching DMA bit in DMACON.
- `parse_sound_command_stream`: Clears the first word at a0, reads command bytes from a1, and dispatches on values 128 to 132 to set up sound channel fields before continuing at the shared loop.
- `select_instrument_from_byte`: Reads a byte from a1, scales it by 32 to index the instrument table at a5, stores the pointer and its sample offset into the channel structure at a0.
- `set_channel_volume_and_sample`: Reads a volume byte from a1, multiplies it by the master volume at offset 768 of a3, stores the result and sample pointer into the channel at a0.
- `jump_to_shared_continuation`: Branches unconditionally to a shared continuation address, transferring control without touching any registers or memory.
- `store_byte_and_set_bit1_then_continue`: Copies the byte at (a1) into offset 20 of a0, increments a1, sets bit 1 at offset 0 of a0, then branches to the shared continuation.
- `advance_record_pointer_and_load_next`: Reads offset 12 of a0 into d0, adds it to the pointer at offset 8 to form a2, increments d0 by 4, tests the longword at a2, and if zero reloads a2 and a1 from the base pointer plus a3 before storing d0 back and branching to the shared continuation.
- `init_sound_channel_from_script`: Reads a script byte, scales a duration by the byte at offset 768 of a3, stores period and pointer into the channel struct, copies volume and sample pointers from a5, computes a scaled period from a table indexed by the script byte, writes it to the audio hardware and clears the active flag.
- `advance_channel_volume_and_script`: Adds the signed volume delta to the channel volume and writes it to the audio hardware, then divides the period by the a5 step and applies a signed script byte, and advances the script pointer while updating the volume ramp index.
- `clear_script_byte_and_advance_index`: Writes zero to the byte at a2 indexed by d1, stores d1 into offset 36 of the channel struct, then branches to the shared channel loop.
- `decrement_d1_compare_d2_clear_bit2`: Subtracts one from d1, compares it with d2, and if d1 is still above d2 clears bit 2 of the byte at a0 before branching back to the caller's loop.
- `store_tile_and_column_index`: Writes the byte value 60 into the tile buffer at a2 indexed by d1, stores d1 into offset 36 of a0, then branches to another routine.
- `blit_masked_tile_rows_to_bitplanes`: Copies masked tile words from a source table into four bitplane rows of the destination bitmap, stepping through 64 rows of 4 words using d0-d6 and a0-a2.
- `blit_sprite_masked_to_bitplanes`: Copies 160 words from a source buffer, ORs each with a rotating mask from a table, and writes masked pixels into four interleaved bitplane rows of the destination.
- `store_flag_and_dispatch_draw`: Saves the argument registers, stores the last word argument into a global flag, then calls the drawing routine with the prepared registers.
- `compute_tile_screen_position_and_draw`: Converts a tile index into screen coordinates using division by ten, adjusts for terrain height, applies direction offsets, and calls the drawing routine.
- `mouse_and_keyboard_interrupt_handler`: Reads mouse movement from JOY0DAT and POTGOR buttons plus keyboard serial data from CIAA_SDR, updates cursor position and button state, then acknowledges interrupt bits and returns from exception.
- `reset_mouse_delta_baseline`: Reads the mouse port counters, stores the low byte and shifted high byte as previous X/Y deltas, then calls the input update routine with d0 zeroed.
- `install_mouse_interrupt_vector`: Saves the current interrupt enable register and level 2 vector, disables interrupts, points the level 2 vector at the mouse handler and re-enables interrupts.
- `restore_saved_interrupt_vectors`: If the saved interrupt vector pointer is nonzero, disables all interrupts and clears pending requests, restores the vector at address 108, re-enables the saved interrupt mask, and returns that mask in d0.
- `zero_filled_data_area_3`: Three zero words that decode as ori.b instructions; this is a zero-filled data area referenced by address, not executable code.
- `build_copper_list_for_display`: Writes a Copper display list into the buffer at a0 using the bitplane pointers, scroll offsets and palette data passed on the stack, then enables Copper and bitplane DMA via DMACON.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `set_copper_list_and_wait_vblank`: Stores the caller's return address at absolute address 1280, writes that address plus four to the COP1LCH copper pointer, then polls INTREQR bit 5 until the vertical blank interrupt arrives.
- `acknowledge_vertical_blank_interrupt`: Writes 32 to INTREQ to clear the vertical blank interrupt, then polls INTREQR bit 5 until that interrupt bit is set again before returning.
- `copy_words_after_marker_into_buffer`: Scans the first pointer's words for the value 384, then copies the requested count of words from the second pointer into the following slots, advancing four bytes per word.
- `copy_sprite_frames_to_buffers`: Reads a frame index and destination pointer from the stack, indexes a table of 160-byte records, then copies 16 longwords into two destination buffers and writes two zero terminators.
- `set_object_position_and_flags`: Looks up an object pointer from a table by index, computes screen coordinates from the arguments, stores x and y bytes at offsets 0-2 and 72-74, and sets direction flag bits at offsets 3 and 75.
- `decode_ciaa_keyboard_scan`: Reads the CIAA serial data register, clears key state bytes, and when a new key is available converts the scan code into a character from the keymap table, storing it for the caller.
- `copy_8000_bytes_between_buffers`: Copies 8000 bytes from the source pointer at offset 8 of the frame to the destination pointer at offset 12, advancing both pointers, then restores a0-a1 and returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `decrement_packed_nibbles_with_vblank_wait`: Copies a word buffer to a local stack area, decrements each nonzero nibble of every word, waits on VHPOSR until the raster line passes 250, and calls the blit routine until no nibble changes.
- `get_raster_line_then_call_6be6`: Calls the routine at 5450 to read a value into d0, stores it in a local, pushes the two arguments at offsets 8 and 12, calls the routine at 6be6, and returns its result in d0.
- `call_6ba2_then_optional_5420`: Pushes the longword argument from the stack frame, calls the routine at 6ba2, drops the argument, and if the global longword at a4-32230 is nonzero calls the routine at 5420 before returning.
- `advance_seeded_random_and_return_15bit`: Multiplies the stored seed by a constant via a helper, adds 12345, stores the new seed, then returns bits 16 through 30 of it as a 15-bit value in d0.
- `store_high_word_of_argument_global`: Reads the word argument from the stack frame, zero-extends it, shifts it right by sixteen bits and stores the result in the global longword at a4-32246, then returns.
- `set_a4_to_high_memory_and_return`: Loads the immediate value 32766 into address register a4 and returns to the caller, leaving all other registers unchanged.
- `allocate_and_initialize_manx_structure`: Allocates memory, stores the MANX signature, fills fields of the new structure, calls several subroutines to set up resources, and returns after restoring a2 and the stack frame.
- `copy_d0_to_d5`: Copies the longword in data register d0 into data register d5, leaving d0 and all other registers unchanged.
- `tokenize_text_into_string_array`: Allocates a buffer, copies a source string, splits it into whitespace-separated tokens handling quoted strings, stores pointers in a newly allocated array and returns via globals.
- `load_d0_with_max_word`: Moves d0 to itself, loads the immediate value 32767 into d0, then branches into another routine.
- `append_string_with_length_limit`: Finds the end of the destination string, copies up to the given count of bytes from the source, null-terminates if truncated, and returns the destination pointer in d0.
- `bounded_strncpy_pad_zero`: Copies bytes from a1 to a0 until a null byte or d1 count is reached, then fills the remaining count with zero bytes and returns the destination pointer in d0.
- `unsigned_32bit_multiply_d0_d1`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, returns the low 32-bit product in d0 and preserves d1-d3.
- `open_window_and_load_icon_images`: Opens a window via library calls, loads icon images into the window structure at offsets 156, 160 and 164, then closes the window and clears the stored handle.
- `copy_d0_to_d5_2`: Copies the longword in d0 into d5, then falls through into the following routine.
- `call_hook_then_forward_arg`: If the longword at offset -3480 from a4 is nonzero, loads it into a0 and calls it; then pushes the word argument from the stack frame and calls the next routine.
- `shutdown_audio_and_free_buffers`: Restores the saved d4, frees several allocated buffers and audio channels via helper calls, optionally invokes the exec Supervisor vector, then returns the saved argument in d0 and restores a7.
- `pop_d4_unlink_a5_return`: Pops d4 from the stack, releases the a5 frame pointer and returns to the caller; it is not empty, it restores d4 and a5.
- `free_six_byte_entry_by_index`: Looks up a six-byte entry by index in a table at a4-relative base, validates the index and entry, optionally frees the pointed block via a call, clears the entry and returns zero or minus one on error.
- `jump_to_entry_free_routine`: Jumps directly to the entry-freeing routine, passing control without setting up a new stack frame.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `jump_via_a4_offset_table_entry`: Loads a pointer from the table at a4 minus three thousand four hundred and ninety four, then jumps to the code at offset minus thirty within that structure.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `jump_via_a4_second_table_entry`: Loads a pointer from the table at a4 minus three thousand five hundred and eighteen, then jumps to the code at offset minus four hundred and fourteen within that structure.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `jump_via_a4_third_table_entry`: Loads a pointer from the table at a4 minus three thousand five hundred and eighteen, then jumps to the code at offset minus one hundred and ninety eight within that structure.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `jump_via_a4_offset_table_entry_2`: Loads a pointer from the table at a4 minus a fixed offset and jumps indirectly through the entry at that table position, transferring control to the target code.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `jump_via_a4_table_second_entry`: Reads the a4-relative table base, indexes a different fixed entry, and performs an indirect jump to the routine pointer stored there.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `jump_via_a4_table_third_entry`: Uses the a4-relative table pointer to fetch another fixed entry and jumps indirectly to the code address it holds, returning control to that target.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `jump_to_relocated_vector_table`: Jumps indirectly through a pointer stored in the a4-relative table, transferring control to the target routine without returning.
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

### Populous (1989)(Electronic Arts)[cr Flint Firforge].adf:populous.prg

- `save_registers_and_install_trap_vector`: Branches forward, then saves all registers to a6-relative memory, stores the stack pointer and a handler address into the trap vector, and executes an illegal instruction.
- `save_context_and_chain_to_next_handler`: Stores d0 and the user stack pointer into a6-relative memory, copies saved registers, sets the trap vector to the next handler, then executes many nops.
- `apply_relocation_table_entries`: Walks the relocation offset table, adds the load base to each stored address in memory, then copies thirteen words from the code area to the low memory vector block.
- `relocate_offsets_into_base`: Reads the relocation count and base, iterates the offset table adding the base to each referenced longword in memory, then falls through to the vector copy.
- `branch_to_vector_copy_tail`: Unconditionally branches forward to the shared tail that copies thirteen words into the low memory vector block; it performs no work itself.
- `skip_d0_pointer_entries`: Walks a linked list starting from the longword at the address just before the program counter, following each entry and skipping d0 entries, then returns the address of the next entry in a0.
- `copy_vector_table_and_jump`: Copies thirteen words from the code following the routine into the exception vector area, sets up the user stack pointer, loads registers and jumps to the relocated code.
- `copy_words_clear_remainder_and_rte`: Copies longwords from a0 to a1 until a1 reaches a2, zero-fills the rest up to a0, saves d0-d7 to low memory, restores registers and returns from exception.
- `populous_sprite_offset_data_table`: The bytes are data decoded as code, a table of small offset values stored as immediate operands, not executable instructions.

### icon.library

- `icon_library_return_minus_one`: Library code that loads minus one into d0 and returns immediately, giving the caller a failure value.
- `icon_library_increment_open_count`: Library code that increments the word open counter at offset 32 of the library base in a6, clears bit 3 at offset 14, and returns the base in d0.
- `decrement_open_count_and_close_if_zero`: Sets d0 to zero, decrements the word at offset 32 of the library base in a6, and if it reaches zero with bit 3 of byte at offset 14 set, calls the close routine at offset 256.
- `close_library_and_unlink_node`: If the word at offset 32 of a6 is zero, unlinks the node at a6 from its list, computes a negative offset from the word fields at offsets 16 and 18, and calls the library close vector at offset -210 of the base at offset 34.
- `set_close_flag_and_return_zero`: Sets bit 3 of the byte at offset 14 of the library base in a6, then sets d0 to zero and returns to the caller.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_and_store_base`: Saves a2, stores a0 and a6 into the new library base at offsets 42 and 34, opens dos.library via exec, stores the result at offset 38, and alerts on failure.
- `copy_library_bases_to_globals`: Copies the library base and dos base from offsets 34 and 38 of a2 into two absolute globals, returns a2 in d0 and pops a2.
- `zero_filled_data_area_4`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_area_5`: Contains a single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `save_a0_a6_call_library_vector`: Pushes a0 and a6 onto the stack, calls the library vector at address zero, then discards the eight saved bytes and returns.
- `save_a0_a1_a6_call_vector`: Pushes a0, a1 and a6 onto the stack, calls the library vector at address a8, then releases twelve bytes of stack and returns.
- `icon_lib_save_a0_a2_call_helper`: Pushes registers a0 through a2 and a6 onto the stack, calls a library helper routine, then discards the saved registers and returns.
- `icon_lib_save_a0_a1_call_helper`: Pushes registers a0 and a1 plus a6 onto the stack, calls a library helper routine, then removes the saved registers and returns.
- `icon_lib_save_a0_call_helper`: Pushes register a0 and a6 onto the stack, calls a library helper routine, then discards the two saved registers and returns.
- `icon_library_call_692_with_a0`: Pushes a0 then a6 onto the stack, calls the library routine at offset 692, discards both saved registers and returns.
- `icon_library_call_6a8_with_a6`: Pushes a6 onto the stack, calls the library routine at offset 6a8, discards the saved register and returns.
- `icon_library_call_71c_saving_a0_a2`: Saves a0 through a2 and a6 on the stack, calls the library routine at offset 71c, then releases the saved registers and returns.
- `icon_lib_push_a0_a6_call_768`: Pushes a0 and a6 onto the stack, calls the library routine at offset 768, then discards both saved registers and returns.
- `icon_lib_push_a0_a6_call_7fc`: Pushes a0 and a6 onto the stack, calls the library routine at offset 7fc, then discards both saved registers and returns.
- `icon_lib_push_a0_a1_call_818`: Saves a0 and a1 on the stack, calls the library routine at offset 818, then removes the two saved registers and returns.
- `icon_library_call_864_with_saved_regs`: Saves a0 and a1 on the stack, calls the library routine at offset 864, then discards the saved registers and returns to the caller.
- `icon_library_call_8d6_with_saved_regs`: Pushes a0 and a1 onto the stack, invokes the library routine at offset 8d6, cleans the stack and returns to the caller.
- `icon_library_set_field_94_via_jsr`: Saves a6, loads the library base from the stack argument, calls the library vector at offset minus 294, stores the second argument at offset 148 of the result, and returns.
- `zero_filled_data_area_6`: A single zero word that decodes as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `allocate_and_init_icon_object`: Looks up an icon object by two arguments, calls an initializer with a stack buffer and the object's field, and on failure frees the object, returning it or zero.
- `copy_stack_fields_into_icon_object`: Copies eleven longs from the stack frame into the icon object at a2, then stores individual fields and links the object into its parent list.
- `return_zero_restore_d2_d4_a2`: Sets d0 to zero as the return value, restores d2 through d4 and a2 from the stack frame, releases the a6 frame and returns to the caller.
- `copy_icon_header_and_dispatch`: Builds a local icon structure by copying eleven longwords from offset 96 of the passed structure plus selected fields, then calls the dispatch routine with the two arguments and the local block.
- `open_icon_library_and_load_info`: Opens the icon library, reads the .info file into a stack buffer, checks the icon magic and version, then loads associated image and tool data through the library before returning a status.
- `load_icon_header_and_optional_images`: Calls the caller-supplied hook, checks the icon header magic and version, clears two fields in the icon structure, then loads the optional image and selects data pointers, returning zero on failure.
- `load_icon_select_and_alternate_images`: Loads the two select-image pointers through the library hook, then loads the alternate image and its mask into the icon structure, returning zero if any allocation fails.
- `load_icon_image_array_and_mask`: Loops over the image count, allocating each image through the library hook and storing it in the icon array, then optionally loads the mask and returns one on success.
- `set_d4_zero_and_branch_to_shared_exit`: Loads immediate zero into d4 and branches to the shared exit path at the following routine, leaving other registers untouched.
- `return_d4_restore_regs_and_unwind_frame`: Moves d4 into d0 as the return value, restores d2-d5 and a2-a5 from the stack frame, releases a6 and returns to the caller.
- `allocate_icon_and_build_icon_data`: Saves registers, allocates a 260-byte icon structure, fills it from the three arguments and nested data, frees it on failure, and returns the result in d0.
- `icon_render_with_optional_second_image`: Saves the image pointer at offset 26 of the icon structure, calls three library helpers to set up drawing, then conditionally draws the main image and optional second image via the callback in a4, returning the result in d4.
- `icon_draw_second_image_and_lists`: Draws the image at offset 22 and the image at offset 26 through the a4 callback, then draws optional text and pointer lists from offsets 50, 54 and 70, returning success in d4.
- `icon_advance_pointer_list_and_draw`: Advances a3 past the null-terminated pointer list at offset 54, counting four bytes per entry, then calls the a4 callback to draw the list and reloads a3 from offset 54.
- `icon_library_scan_and_attach_lists`: Walks the null-terminated pointer list at a3, calling the a4 callback twice per entry and storing the first result plus one at -4(a6); then processes the pointer at 70(a2) the same way, calls two library helpers, stores d6 at 26(a2) and returns d4 in d0.
- `push_a2_and_enter_second_entry`: Saves a2 on the stack, loads the caller's return address from 12(a7) into a0, sets a2 to two bytes past that address and branches forward into another routine.
- `walk_icon_list_until_wrap`: Calls a library helper with d0, then repeatedly calls another helper on a2 and compares a2 with the pointer at 8(a2), stopping when they match or the result is zero; if nonzero it calls the first helper again and returns.
- `forward_icon_call_with_offset`: Loads the first stack argument into d0 and the second into a0, then calls the library routine at offset 140 of a0 with d0 pushed as an argument, cleaning up the stack and returning.
- `allocate_and_initialize_icon_block`: Saves d2-d3/a2, allocates a 65536-byte block, initializes fields at offsets 140 and 142, and returns the block pointer in d0 or zero on failure.
- `finalize_icon_block_and_return_pointer`: Pushes a2 and d2 with the block offset 140, calls the library routine, cleans the stack, and returns a2 in d0 if the call succeeded.
- `return_zero_pop_d2_d3_a2`: Sets d0 to zero as the return value, restores d2, d3 and a2 from the stack, then returns to the caller.
- `decrement_count_and_store_pair`: Decrements the word count at a2, calls a library routine when it reaches zero, then stores d2 and the fourth argument into the table pointed to by a2's field.
- `store_pair_after_decrement`: Decrements the word at a2, then writes d2 and the stack argument into the table at a1 indexed by the new count, returning one.
- `allocate_and_init_icon_header`: Saves d2-d4/a2, allocates 94 bytes of chip memory, initializes the header at offset 78, then calls three helpers to validate and link the icon; returns the block pointer or zero.
- `branch_to_cleanup_return`: Executes a single branch instruction that jumps forward to the shared cleanup and return path, performing no work itself.
- `free_icon_block_and_return_zero`: Pushes the block size and pointer, calls the memory free routine, clears d4, restores d2-d4/a2 from the stack and returns zero in d0.
- `call_icon_helper_with_offset_4e`: Saves d2, loads two arguments from the stack, adds 78 to the second, pushes both and calls a library helper, then restores d2 and returns.
- `call_indirect_with_saved_registers`: Saves d2 through d4 and a2 through a3, loads a pointer and value from the stack, pushes the value, calls through address zero, stores results in d3 and d4, then branches based on the return.
- `check_byte_after_indirect_call`: Saves d3, d2 and a3, calls a library routine, tests its result, and if nonzero advances a3 past a byte equal to 61 and returns that pointer.
- `pop_next_icon_list_entry`: Loads the next longword from the list pointed to by a2 into a3 and d0, advancing a2; if it is zero, sets d0 to zero, then restores d2-d4/a2-a3 and returns.
- `call_icon_callback_with_arg`: Saves d2-d5, loads two stack arguments into d2 and d3, pushes d3, calls the function pointer at address zero, stores the result in d5, discards the argument and branches onward.
- `allocate_icon_block_and_compute_size`: Pushes a small constant and d2, calls the allocator at offset sixteen, saves the result in d4, discards both stack entries, and if non-null returns d4 minus d2 in d0.
- `find_next_icon_by_handle`: Calls library routines to compare d5 with the returned handle, then calls another routine with d3 and d2; returns 1 in d0 when the second call succeeds, otherwise falls through to the shared exit.
- `advance_d2_past_d4_or_zero`: Tests d4; if nonzero copies d4 into d2 and increments it by one, then branches to the shared exit; if d4 is zero it branches directly to the exit.
- `return_zero_icon_result`: Sets d2 to zero, tests it and branches if nonzero, then sets d0 to zero, restores d2 through d5 from the stack and returns to the caller.
- `parse_icon_tooltype_string`: Saves registers, clears byte 30 of the first argument, then calls library helpers to match and copy a tool-type string into the second argument's buffer, returning the first argument.
- `copy_tooltype_suffix_after_match`: Advances a4 by five, calls the library string routine with length 28 and a three-byte pattern, and on success sets d3 to one and advances a4 by three.
- `set_d3_zero_and_branch`: Sets d3 to zero and branches forward to another part of the library code, returning no value itself.
- `parse_decimal_number_from_string`: Accumulates digits from the string at a4 into d3 as a decimal value, calls a digit-conversion routine for each character, then formats and stores the result via library calls before returning a2 in d0.
- `convert_ascii_digit_to_value`: Loads the longword argument from the stack, checks it lies between the characters zero and nine, and returns its numeric value in d0 or falls through to the failure path.
- `return_minus_one_error`: Loads minus one into d0 and returns immediately, signalling a conversion failure to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `allocate_and_init_icon_pair`: Saves d2-d5, reads three arguments from the stack, allocates a structure via the library allocator, then initializes it with the other two arguments and frees it on failure, returning the handle or zero.
- `create_icon_with_size_field`: Saves d2/a2, allocates a 96-byte block, stores the value 10 at offset 14 and at the caller's word pointer, then links the block into the caller's list and returns success.
- `return_zero_restore_registers`: Sets d0 to zero as the return value, restores d2 and a2 from the stack and returns to the caller.
- `call_icon_vector_30_compare_result`: Loads two arguments from the stack, pushes them plus a third stack argument, calls the library vector at offset 48, then compares the result with a stack value and returns zero on mismatch.
- `return_d1_as_d0`: Copies the value in d1 into d0 and returns immediately, acting as a trivial result-forwarding stub.
- `call_icon_vector_4c_compare_result`: Loads two arguments from the stack, pushes them plus a third stack argument, calls the library vector at offset 76, then compares the result with a stack value and returns zero on mismatch.
- `return_d1_in_d0`: Copies the longword in d1 into d0 and returns immediately, acting as a trivial pass-through library stub.
- `unsigned_32bit_divide_d1_by_d2`: Divides the longword in d1 by d2, using a 16-step shift-subtract loop when d2 exceeds 65535, otherwise two word divisions, returning quotient in d0 and remainder in d1.
- `shift_subtract_divide_loop`: Performs a 16-iteration shift-subtract division of d1 by d2, doubling the partial quotient in d0 and subtracting d2 when it fits, leaving quotient in d1.
- `mul32x32_high_low_unsigned`: Multiplies the 32-bit values in d0 and d1 using 16-bit multiplies, combines partial products into a 64-bit result, and returns the high 32 bits in d0 while preserving d2.
- `format_string_via_rawdofmt`: Saves a2-a4 and a6, loads the format string and argument pointer from the stack, sets a2 to a local format descriptor, and calls exec RawDoFmt through the library base at absolute address 4.
- `store_byte_to_a3_and_advance`: Writes the low byte of d0 to the memory pointed to by a3 and increments a3 by one, returning immediately to the caller.
- `strlen_bounded_255`: Loads the string pointer from the stack, scans up to 255 bytes for a zero terminator, and returns the complemented count in d0 as the string length.
- `find_char_or_return_null`: Reads a character from the string in a0, compares it with the byte in d0, and returns the matching address in d0 or zero when the terminator is reached.
- `return_zero_and_branch`: Sets d0 to zero and branches into the shared exit path of the character search routine, returning a null result to the caller.
- `copy_string_until_nul`: Loads two pointers from the stack, copies bytes from the source to the destination one at a time until a zero byte is copied, then returns.
- `skip_leading_nul_scan`: Loads two pointers from the stack, scans forward past bytes in the first buffer until a nonzero byte is found, backs up one, and branches onward.
- `compare_strings_up_to_length`: Loads two pointers and a length from the stack, compares bytes one by one, returns zero when a matching zero byte ends both strings, otherwise branches onward.
- `return_one_or_negative_one_flag`: Loads 1 into d0, then branches past the next instruction; the fall-through path loads -1 into d0 and returns, so callers get a positive or negative flag.
- `scan_bytes_until_zero_or_count`: Loads a0 and a1 from the stack, loads a count into d0, then advances a0 past bytes until a zero byte or the count runs out, branching onward.
- `decrement_a0_and_call_next`: Subtracts one from address register a0, calls the following library routine with bsr, then returns to its caller.
- `copy_bytes_until_nul_or_count`: Decrements d0 and, while it stays non-negative, copies bytes from (a1) to (a0) until a zero byte is copied, then fills remaining count with zero bytes.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `init_self_referential_list_header`: Loads the pointer from the stack into a0, stores a0 at (a0), adds four to that value, clears the next longword and stores a0 at offset eight.
- `icon_draw_with_scaled_offsets`: Saves d2-d3/a2, calls library routines to draw an icon using its width and height fields scaled by eight, then draws again with an offset and returns a success flag.
- `save_regs_and_load_three_args`: Pushes d2, d3 and a2 onto the stack, then loads the three arguments from offsets 16, 20 and 24 of the stack into d2, d3 and a2 before branching to another routine.
- `walk_icon_list_checking_two_entries`: Saves registers, calls a subroutine twice per list node to test fields at offsets 7 and 8 of a2, follows the next pointer at offset 12, and returns 1 if the list ends or 0 on failure.
- `return_zero_restore_registers_2`: Sets d0 to zero as the return value, pops d2, d3 and a2 from the stack and returns to the caller.
- `alloc_icon_header_and_two_images`: Saves d2-d5/a2, reads four stack arguments, allocates a 20-byte header and two images via library calls, stores the second image pointer at offset 10 of the header, and returns the header or -1.
- `return_minus_one_restore_registers`: Sets d0 to -1 as the failure return value, pops d2-d5 and a2 from the stack, and returns to the caller.
- `begin_icon_remap_with_saved_registers`: Pushes d2-d6 and a2, loads three arguments from the stack into d2, d3 and d4, clears d6, then branches onward to continue the icon remap work.
- `icon_alloc_scratch_and_test_d6`: Pushes a size and word argument, saves d3 and d2, calls a library routine, stores the result at offset 56 of the stack, then tests d6 and branches based on the outcome.
- `icon_build_list_from_node_chain`: Walks a linked list of icon nodes, calling library routines to allocate and fill entries, storing results in node fields, and returns minus one on failure or the saved d6 value.
- `icon_return_minus_one_restore_regs`: Loads minus one into d0 as the failure return value, restores d2 through d6 and a2 from the stack, and returns to the caller.

### translator.library

- `open_translator_library_base`: Saves d3/a2/a6, reads exec base from address 4, calls exec AddLibrary with the library name and version 42, stores exec base and the passed a0 in the new library base, then calls MakeLibrary and returns -1 on success or 0 on failure.
- `increment_open_count_clear_bit5`: Tests the word open counter at offset 32 of the library base in a6; if zero, increments it and clears bit 5 of the byte flags at offset 14, then returns the library base in d0.
- `decrement_open_count_and_close`: Sets d0 to zero, decrements the word open counter at offset 32 of the library base in a6, and if it reaches zero with bit 5 of the flags at offset 14 set, saves a1 and calls the close routine before returning.
- `check_translator_open_flag`: Saves d3, a3 and a6, switches a6 to the library base, tests the word at offset 32 of a3 and sets bit 5 of byte 14 there if nonzero, then returns d0 as zero.
- `unlink_node_and_free_memory`: Unlinks the node at a3 from its list by patching the next and previous pointers, saves the value at offset 38, then calls the library memory free function with the node size and returns the saved value.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `parse_translation_string_into_buffer`: Saves registers, builds a 27-word space-filled buffer on the stack, then scans the input string at a1 for hash and space characters, calling helper routines to fill and translate the buffer.
- `copy_d0_to_d1_and_branch`: Copies the low word of d0 into d1 and branches forward into another routine, leaving all other registers untouched.
- `clear_d1_call_59e_and_return_word`: Sets d1 to zero, calls the routine at offset 59e, clears the byte before a1, computes d0 as a1 minus the longword at offset 8 of a5, restores d2-d7/a2-a6 from the stack, and returns d1 sign-extended.
- `walk_a6_list_and_dispatch`: Loads a6 from offset 0 of a5, returns if it is null, otherwise advances a6 to the next node, calls routines at offsets 23a and 202, and branches based on the result flags.
- `translator_library_init_and_dispatch`: Loads the library base from offset four of a5, dereferences it into a6, points a6 at offset 1902, calls two library subroutines, then tests d7 and branches based on its sign.
- `translator_parse_sequence_with_saved_regs`: Calls a subroutine and branches if nonzero, saves a2 and a3 on the stack, calls three more subroutines checking each result, then restores a2 and returns.
- `translator_pop_regs_and_scan_delimiters`: Pops a3 and a2 from the stack, reads a byte from a6 into d7, compares it against two delimiter values and branches accordingly.
- `lookup_char_class_offset`: Reads the byte at a2, uses it as an index into the word table at a4, and if the high bit is clear computes a class offset from the byte and loads a pointer from the a6 table into a3, adding it to a6.
- `match_bracket_escape_sequence`: Scans the a6 string for an opening bracket, then compares following bytes against the a2 string until a closing bracket or mismatch, advancing a3 and d6 and returning early on mismatch.
- `backward_match_bracket_sequence`: Walks d6 backward through the a6 string, looks up each byte in the a4 table, and compares it against bytes read backward from a2 until a mismatch or the index goes negative, then returns.
- `set_condition_codes_and_return`: Sets the 68000 condition code register to the immediate value 4 and returns immediately to the caller; it is an empty library stub that only sets flags.
- `check_translator_entry_with_d7_flag`: Loads 1 into d7, calls a library routine, branches if its result is nonzero, otherwise clears the condition codes and returns.
- `scan_translation_key_until_equals`: Reads bytes from the table at a6 indexed by d6, advances d6, stops at an equals sign, tests each byte against a4 flags and compares with the caller string at a2.
- `set_d7_ff_call_check_and_clear_ccr`: Loads d7 with minus one, calls the routine at offset 398, branches if its result is nonzero, clears the condition codes and returns.
- `scan_string_for_backslash_or_backtick`: Copies d6 to d7, decrements then increments d7, reads a byte from a6 indexed by d7 into d0, compares it with backslash and backtick, and returns when d7 equals d6.
- `compute_offset_from_a1_and_a5_base`: Subtracts the longword at offset 8 of a5 from a1 into d0, adds d7, compares with d2, masks d6 to a word, adds it to a6, pushes d7 and branches.
- `copy_bytes_and_set_escape_flag`: Copies d7+1 bytes from a6 to a1, then looks up the last copied byte in the a4 table and sets or clears bit 0 of byte 20(a5) depending on the table bit and whether the next source byte is 96.
- `scan_back_for_high_bit_char`: Walks a1 backwards while the a4 table entry for each byte lacks bit 11, writes a zero at the stopping position, then computes a0 minus the longword at 12(a5) into d0, d1 and d7.
- `dispatch_translator_char_table`: Scans the 12-byte table at a3 for byte d5, computes an index from the match position, loads a code pointer from the a3 table and jumps to it.
- `test_d7_sign_and_predecrement_a2`: Tests the sign of the word in d7 and branches away if negative; otherwise loads the byte before a2 into d1 and decrements a2, then branches onward.
- `fetch_byte_index_into_a4_word_table`: Loads the byte at a2 into d1 and increments a2, zero-extends it into d4, doubles it, then reads the word at a4 plus that offset into d4 and returns.
- `call_predecrement_fetch_and_test_bit6`: Calls the routine that tests d7 and fetches a byte into d1, then tests bit 6 of the resulting d4 word and returns to the caller.
- `wait_until_d4_bit5_clear`: Calls a library routine, then repeatedly tests bit 5 of d4 and calls the routine again while that bit is set, finally setting the condition codes to indicate a nonzero result and returning.
- `check_d4_bit3_then_bit5`: Calls a library routine, tests bit 3 of d4 and returns early if clear, otherwise tests bit 5 of d4 and returns with the resulting condition codes.
- `wait_for_d4_bit5_or_char_49_45`: Calls a library routine, loops while bit 5 of d4 is set, then compares d1 against the characters 73 and 69, setting the condition codes and returning.
- `set_condition_codes_and_return_2`: Loads the condition code register with the immediate value 8 and returns immediately to the caller; it is a tiny library stub that only sets flags.
- `parse_escape_sequence_letters`: Repeatedly calls the character fetch routine and compares the returned d1 byte against E, S, D, R, L, Y, N and G, branching to shared exits based on d4 bit 7 and the matched letters.
- `match_ngs_escape_sequence`: Calls the character fetch routine to check for N then G, tests d4 bit 7, then checks for S and another high-bit character, setting condition codes to 4 before returning.
- `set_condition_codes_mask_8`: Loads the immediate value 8 into the condition code register, setting the CCR flags, then returns to the caller.
- `call_helper_then_test_d4_bit4`: Calls a shared library helper, tests bit 4 of d4 updating the condition codes, and returns to the caller.
- `call_helper_then_test_d4_bit2`: Calls a shared library helper, tests bit 2 of d4 updating the condition codes, and returns to the caller.
- `test_d4_bit5_after_call`: Calls the shared library routine at offset 958, then tests bit 5 of d4 and returns the resulting condition codes to the caller.
- `test_d4_bit8_after_call`: Calls the shared library routine at offset 958, then tests bit 8 of d4 and returns the resulting condition codes to the caller.
- `test_d4_bit5_loop_add_a2`: Calls the shared library routine at offset 958, tests bit 5 of d4, and while set repeats the call; otherwise adds d7 to a2 and sets the zero flag before returning.
- `test_d4_bit1_after_library_call`: Calls the shared library routine at offset 958, tests bit 1 of d4, and returns with the result in the condition codes.
- `loop_while_d4_bit1_set_after_call`: Calls the shared library routine at offset 958, then repeatedly tests bit 1 of d4 and loops while it is set, adding d7 to a2 and setting the condition codes before returning.
- `test_d4_bit7_after_library_call`: Calls the shared library routine at offset 958, tests bit 7 of d4, branches if set, otherwise sets the condition codes and returns.
- `set_condition_codes_and_return_3`: Loads the condition code register with the value 4 and returns immediately to the caller; it is a two-instruction library stub that only sets flags.
- `translate_input_to_uppercase_buffer`: Reads bytes from the a0 input pointer, maps lowercase to uppercase and control characters to spaces, stores results into the a5 buffer, counts up to 100 characters and returns a status in d0.
- `backspace_output_buffer_and_return`: Steps the a0 input pointer back, writes a space two bytes before a2, decrements the d3 counter twice and returns to the caller.
- `return_minus_three_in_d1`: Library routine that loads the immediate value minus three into data register d1 and returns immediately to its caller.
- `write_two_spaces_to_a2`: Library routine that writes two space characters to consecutive bytes at the address in a2, advancing a2 past them, then returns.
- `expand_byte_pair_via_translation_table`: Library routine that scans bytes between the buffer pointer and a1, checks a flag table, matches two-byte codes against a table and inserts a byte, updating the stored pointer.
- `shift_bytes_up_and_store_pointer`: Copies bytes backwards from a1 down to a0, shifting each one up by one, writes byte 52 after the range, stores a1 plus one at offset 16 of a5, then restores registers and returns.

### info.library

- `illegal_instruction_trap_stub`: Executes an illegal instruction with the 19196 opcode, which raises an exception; it is a single-instruction stub, not a returning routine.
- `save_a6_d0_and_store_a0_globals`: Stores a6 and d0 into absolute longword globals, copies d0 into a1, writes a0 at offset 34 of a1, stores a0 in another global, and returns.
- `info_library_call_vector_zero`: Pushes a6 onto the stack, calls the library jump vector at address zero, then discards the pushed longword and returns to the caller.
- `info_library_call_vector_120`: Pushes a6 onto the stack, calls the library jump vector at offset 120, then discards the pushed longword and returns to the caller.
- `info_library_call_vector_18`: Pushes a6 onto the stack, calls the library jump vector at offset 18, then discards the pushed longword; it falls through without returning.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `info_library_call_5e2_with_saved_regs`: Pushes a1 and a6 onto the stack, calls the library routine at offset 5e2, then discards the eight saved bytes and returns to the caller.
- `clear_bit3_and_increment_count`: Library code loads the argument pointer from the stack, clears bit 3 of the byte at offset 14, increments the word at offset 32 and returns the pointer in d0.
- `info_library_clear_flag_if_count_positive`: Saves d2 and a2, loads the object pointer from the stack, calls two library routines, and if the word at offset 32 is nonzero sets bit 3 of the byte at offset 14 before returning zero in d0.
- `info_lib_scale_and_offset_rect`: Saves d2 and a2, calls a library routine, computes a2 offset plus width and width minus width from words at offsets 16 and 18, pushes both values, calls two more library routines, then returns the saved d0 value in d0.
- `info_lib_decrement_and_dispatch`: Loads the argument pointer from the stack, decrements the word at offset 32, and if it reaches zero with bit 3 of offset 14 set, calls the routine at offset 18 of the program counter.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `info_lib_release_queued_resources`: Saves d2 and a2, then conditionally calls cleanup routines for the argument pointers and for global pointers at offsets 124, 80, 72, 76 and 84, freeing each non-null resource before returning.
- `allocate_and_init_info_block`: Library code saves a2-a4, calls an allocator for a block, stores the result in a global pointer, and returns zero if allocation failed.
- `fill_info_library_structs_from_base`: Uses a2 base pointer to compute offsets into the allocated block, stores them in d4 globals, copies template data into those structures and returns one after restoring a2-a4.
- `open_info_library_and_store_base`: Saves a2-a3, calls the library open function with a name pointer, stores the returned base pointer in a global, and branches to an error path if opening failed.
- `open_icon_library_store_base`: Pushes a longword and the library name string, calls the open-library vector in a2, stores the returned base at offset 80, pops the arguments and branches based on the result.
- `open_intuition_library_store_base`: Pushes a longword and the intuition library name, calls the open-library vector in a2, stores the returned base at offset 72, pops arguments, and on failure pushes three zero arguments before branching.
- `open_graphics_library_store_base`: Pushes a longword and the graphics library name, calls the open-library vector in a2, stores the returned base at offset 76, pops arguments, and on failure calls the a3 vector with argument 5.
- `return_one_pop_a2_a3`: Sets d0 to 1 as the return value, pops a2 and a3 from the stack, and returns to the caller.
- `check_library_base_word_nonzero`: Saves a2 on the stack, loads a1 with the library base at offset 88, tests the longword there and branches away if it is nonzero, otherwise branches elsewhere.
- `check_node_type_seven`: Loads a0 from the pointer at a1, compares the byte at offset 61 of that structure with 7, and branches away if it differs, otherwise branches elsewhere.
- `load_node_and_check_a4_field`: Loads the pointer at (a1) into a0, reads the long at offset 56 into a2, tests the long at offset 164 of a0 and branches away if it is zero, otherwise reloads a0 and returns that long in d0.
- `check_node_type_3d_and_read_1ba`: Loads the pointer at (a1) into a0, compares the byte at offset 61 with one and branches away if not equal, otherwise follows the pointer at offset 76 and returns the long at offset 442 in d0.
- `return_a2_or_follow_4c_pointer`: Moves a2 into d0 and branches away if it is zero, otherwise loads the pointer at offset 76 of a2 into a0 and branches to the shared exit path.
- `return_zero_and_pop_a2`: Sets d0 to zero as the return value, pops a2 from the stack and returns to the caller; library code.
- `info_library_open_disk_info`: Calls a library helper, stores its result in a new structure at offset 152, then checks a flag byte and passes either a global or a stored pointer to further library calls.
- `info_library_query_disk_info`: Passes a stored pointer and minus two to two library calls, clears a global word, and if the result is zero and a flag is set, returns a value from a nested structure.
- `allocate_info_library_buffers`: Saves registers, allocates two 65536-byte buffers via a library call, stores the first pointer in global memory, and returns nonzero on success or zero on failure.
- `allocate_second_info_buffer`: Pushes size 65536 and type 260, calls the allocator with d3, stores the returned pointer through a4, cleans the stack and branches on the result.
- `check_info_buffer_flag_and_call`: Loads the pointer from d4 into a1 and its target into a0, tests byte 61 of that structure, and if equal calls another routine with d2 and the global pointer.
- `flush_d2_buffer_and_branch`: Tests d2; if zero branches ahead, otherwise pushes the a4 target and d2, calls a library routine, zero-extends the returned word in d0, pops eight bytes and branches based on the result.
- `allocate_info_buffer_and_set_size`: Calls library routines with fields from a3 and a4, increments a counter at offset 100 of the a3 structure, clamps it to 256, allocates a buffer of that size and stores the pointer at offset 96.
- `commit_info_buffer_and_return_one`: Calls a library routine with the buffer at offset 96 of the a3 structure, copies fields from the d4 and a4 structures into offsets 104 and 112, restores registers and returns 1.
- `info_library_open_init`: Sets up an info.library frame, clears several global longs, stores the caller's argument in a global, then calls the open routine and branches on its result.
- `info_library_alloc_open_buffer`: Pushes size and type arguments, calls the library open routine with a stack buffer, stores the result in a global and branches on failure.
- `info_library_check_open_result`: Calls a routine with a stack buffer, tests the returned d0, discards the argument and branches to an error path when the result is zero.
- `call_library_open_and_check_result`: Calls a library routine, stores the returned pointer in a3 and d2, and branches based on whether the result is nonzero.
- `allocate_forty_four_bytes_and_store`: Pushes a size of 68 bytes, calls an allocation routine, stores the returned pointer through a4, cleans the stack, and branches on the result.
- `check_info_library_state_and_dispatch`: Calls several library routines with a3 and a4 arguments, tests a global flag, and calls through a2 with a zero or one argument before branching.
- `open_info_library_and_verify`: Calls a library open routine, saves the result in d2, and if non-zero pushes d2 and a stack pointer to call a verification routine, returning its test result.
- `close_info_library_and_release_resources`: Checks a global flag and d2, then calls several library routines to free a3 and stack resources, finally invoking a2 and restoring d2/a2-a4 before returning.
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
- `create_info_message_port`: Builds a message-port structure on the stack with size and signal fields, calls the port creation function, and on success sends an initial message to it.
- `dispatch_info_node_by_type`: Saves a2-a3, loads the node pointer and a value from the stack, reads the node's type word, and jumps through a table for types 0 to 17.
- `return_one_and_restore_registers`: Sets d0 to 1 as the return value, pops a2-a3 from the stack and returns to the caller.
- `toggle_info_flag_by_node_type`: Saves a2, reads a global library pointer and a node type word, maps types 6 to 19 to bit masks, XORs the mask into a global flag, then calls another routine.
- `store_d1_to_library_global_and_call`: Loads the library base from absolute address 100, stores d1 into offset 112 of it, pushes a1's offset 44 and calls the library jump table, then pops a2 and returns.
- `info_library_open_and_validate`: Library entry that opens the info resource, checks state bytes, allocates and copies buffers, formats a name string and returns a boolean success flag in d0.
- `check_state_byte_and_push_disk_arg`: Loads the pointer at a3, compares byte 61 of that structure with 1, and if equal pushes the immediate value 30 before branching onward.
- `check_info_file_and_append_suffix`: Calls library helpers to open the info file, test its type byte, compare two name strings, format a suffixed name and test the result, returning zero on failure or the status in d0.
- `probe_four_info_entries`: Calls a library function with the passed argument and four global values, each with -1, returning zero if any call returns -1, otherwise calls once more and returns 1.
- `probe_second_info_entry`: Calls the function in a2 with d2 and the global at 312, compares the result with -1, cleans the stack and branches to the failure or success path.
- `call_info_hook_with_negative_one`: Pushes -1, the longword at absolute address 316 and d2 onto the stack, calls the function in a2, compares the result with -1, discards the stacked arguments and branches based on that comparison.
- `call_info_hook_expect_negative_one`: Pushes -1, the longword at absolute address 320 and d2 onto the stack, calls the function in a2, compares the result with -1, discards the stacked arguments and returns zero when they differ.
- `call_absolute_vector_and_return_one`: Pushes d2, calls the routine whose address is stored at absolute address 276, sets d0 to 1, pops the argument, restores d2, d3 and a2, then returns.
- `draw_info_library_icon_grid`: Pushes many coordinate and color arguments and calls the library drawing routine repeatedly to render the info.library icon, then restores a2-a3 and returns.
- `draw_vertical_bar_segments`: Calls the library drawing routine seven times with coordinate pairs to draw vertical bar segments, then releases the stack and restores d2/a2.
- `draw_diagonal_line_segments`: Calls the library drawing routine seven times with coordinate pairs to draw diagonal line segments, then releases the stack and restores d2/a2.
- `info_library_draw_icon_set_a`: Saves d2/a2, loads the library base from the stack, then calls the info library routine nine times with different word arguments to draw or configure icon set A, finally restoring the stack and registers.
- `info_library_draw_icon_set_b`: Saves d2/a2, loads the library base from the stack, then calls the info library routine nine times with a second set of word arguments to draw or configure icon set B, then restores stack and registers.
- `info_library_check_flag_and_branch`: Saves d2/a2/a3, loads two stack arguments into a2 and d2, reads a byte at offset 61 of the structure pointed to by absolute address 88, and branches based on whether it equals 4, setting d1 to 131 otherwise.
- `check_info_list_head_and_report_error`: Loads the info structure pointer from a3, compares its list head plus four with the head pointer, and if unequal pushes error arguments and calls the library error reporter before branching away.
- `report_info_error_and_free_node`: Builds an error report from d1 and a2, calls the library error and cleanup routines, then if d2 is nonzero frees the stored node, restores d2 and a2-a3, and returns.
- `allocate_info_node_and_init_list`: Saves registers, allocates a node of size 65536 with tag 14, stores it at offset 234 of the info structure, and returns zero if allocation failed, otherwise initializes the list head.
- `link_info_entries_into_list`: Advances the entry pointer by four bytes, stores it in the list head, clears a counter, and walks the global list linking each entry through the list structure.
- `allocate_info_node_128`: Pushes size 65536, word 14 and d2, calls the allocator at address 244, moves the returned pointer to a2 and d3, pops 12 bytes and branches depending on the result.
- `build_info_nodes_from_table`: Allocates a 65536-byte block with tag 128, stores the result at offset 10 of a2, then loops over the a4 table calling three library routines per entry until a zero terminator, finally copying the list head into offset 238.
- `check_first_node_and_notify`: Compares the first list node value against the stored current pointer and, if they match, calls three library routines with fields from the passed structure.
- `advance_node_and_notify`: Pushes two global values and the passed structure onto the stack, calls three library routines, then advances the current pointer to the next node.
- `check_second_node_and_notify`: Compares the second list node value against the stored current pointer and, if they match, calls three library routines with fields from the passed structure.
- `pop_info_node_and_notify`: Pops saved registers, reads the current info node's field at offset 238, calls the notify and free helpers with the node's data, then advances the node pointer to its first link and calls the callback with zero.
- `insert_info_node_after_current`: Sets a flag in the library base, checks whether the current node is the last, allocates two nodes of sizes 14 and 128, links them, inserts the new node after the current one and calls the callback with one.
- `remove_current_info_node`: Sets a flag in the library base, checks whether the current node is the last, notifies and frees the current node, then relinks the previous or first node and calls the callback with zero.
- `advance_info_library_node_pointer`: Reads the current node pointer from the library base, calls a routine on it, then if the next node pointer equals the base plus four, resets the pointer to the first node and stores it back.
- `info_library_cleanup_and_restore`: Calls three library routines, passing fields at offsets 46 and 62 of a2, then releases sixteen bytes of stack and restores d2 and a2-a3 before returning.
- `info_library_scan_node_flag`: Saves registers, loads constant 100 into d3 and 88 into a4, then tests the longword at offset 242 of the node pointed to by a1 and branches accordingly.
- `compare_node_list_pointer_to_first`: Loads the node list pointer from d3, follows two pointer fields to reach a structure, computes its address plus four into d0 and a2, then compares that against the first list entry and clears a field in the a4 structure if unequal.
- `push_global_and_call_second_field`: Saves the global longword at address 272 on the stack, follows d3 and a5 pointers to load a field, pushes it, calls the routine at address 22, sets d4 to one and loads a3 from the list head.
- `count_list_nodes_and_allocate_array`: Walks the a3 linked list counting nodes in d4, shifts the count left twice, allocates that many bytes with the library allocator, stores the result at offset 92 of the a4 structure and returns zero on failure.
- `info_lib_start_node_allocation`: Saves d4, calls the library allocation setup with the base structure fields, zeroes d4, loads the node list head into a3 and branches to the shared allocation loop.
- `info_lib_allocate_node_buffer`: Calls the size routine on the node field, defaults the size to four, allocates that many bytes, stores the pointer in the table indexed by d4 and branches back to the loop.
- `info_lib_copy_node_and_advance`: Copies the current node data into the allocated buffer, increments d4, advances a3 to the next node and returns one when the list end is reached, otherwise branches to the allocation routine.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_info_library_by_mode_byte`: Saves registers, reads a mode byte from the info structure, and jumps through a table to one of several layout branches for modes one through six.
- `restore_registers_and_return`: Pops d2 and a2 through a5 from the stack and returns immediately, restoring the caller's saved registers.
- `allocate_info_library_bitmap_buffers`: Calls the graphics allocator four times to create bitmaps and stores the results in library globals, returning zero if any allocation fails.
- `open_four_info_library_resources`: Calls the library open function four times with tag values and stores the returned pointers in globals and at a4, then checks all four for zero and branches to retry or continue.
- `dispatch_info_library_mode_jump`: Loads the library base from address 88, reads byte at offset 61 into d0, bounds-checks it between 1 and 5, then jumps through a table indexed by d0.
- `restore_registers_and_return_from_library`: Restores d2-d3 and a2-a4 from the stack frame at offset -44 from a6, releases the a6 frame and returns to the caller.
- `check_info_flag_and_clear_byte`: Saves d2-d3, loads the argument from the stack into d2, tests a longword at offset 104 in the library base; if zero it clears the byte pointed to by the global at offset 352, then branches to the shared exit.
- `call_info_open_and_compare_result`: Calls two library routines with the library base, a string and the argument, compares the returned longword against minus one, and returns zero if equal, otherwise the result.
- `restore_d2_d3_and_return`: Pops d2 and d3 from the stack and returns to the caller, restoring the saved registers.
- `check_four_info_requests`: Saves d2-d3/a2, loads the request pointer, sets four info request words to 118, 125, 119 and 119 unless a status byte equals 4, then calls the dispatcher for each and returns 1 only if all succeed.
- `request_second_info_entry`: Pushes -1 and the second info request pointer plus d2, calls the dispatcher through a2, compares the result with -1, cleans the stack and branches onward.
- `request_third_info_entry`: Pushes -1 and the third info request pointer plus d2, calls the dispatcher through a2, compares the result with -1, cleans the stack and branches onward.
- `call_a2_with_minus_one_and_cleanup`: Pushes -1, the longword at absolute address 344, and d2 onto the stack, calls the routine in a2, compares the result with -1, then discards the pushed arguments and returns zero if equal.
- `return_one_restore_d2_d3_a2`: Sets d0 to 1 as the return value, restores d2, d3 and a2 from the stack, and returns to the caller.
- `poll_info_library_until_success`: Saves d2-d4 and a2, reads a shift count from the library structure, then repeatedly calls library routines to poll and process, returning zero on failure and looping otherwise.
- `return_zero_and_restore_registers`: Sets d0 to zero as the return value, then pops d2 through d4 and a2 from the stack and returns to the caller.
- `dispatch_info_node_by_type_2`: Saves d2, d3 and a2, reads the node type from offset 20 of a2, and branches to type-specific handling for values 4, 64 and 512, otherwise falling through to a common path.
- `call_library_vector_with_saved_regs`: Pushes d2 and a2 onto the stack, calls the library vector at absolute address a4, then discards the two saved registers and branches onward.
- `push_a2_call_d4_return_zero`: Pushes a2 onto the stack, calls the routine at offset d4, sets d0 to zero and branches onward to offset b0.
- `call_b8_then_d4_return_one`: Pushes d3, calls the routine at offset b8, discards the argument, pushes a2, calls the routine at offset d4, sets d0 to one, restores d2-d3/a2 and returns.
- `info_library_open_entry`: Saves a2, loads the argument from the stack, calls three library routines, checks a byte in the global structure at offset 61, conditionally calls another routine, then calls two more with the object and flag one before restoring a2 and returning.
- `set_four_info_flags_from_bits`: Reads a flags word from the info structure, inverts its low five bits, then calls the flag-setting routine four times with indices 0 to 3 and the masked bit values.
- `build_info_flag_strings_and_draw`: Builds a small stack buffer of character offsets, selects two flag indices based on the argument, and calls the drawing routine to render the flag text at two positions.
- `format_and_draw_info_flag_text`: Formats a string from the caller's arguments into a stack buffer, then calls the text drawing routines to render the flag label and its value at computed screen positions.
- `load_first_arg_and_tail_call`: Loads the longword at offset four from the stack into d0, clears d1, copies d1 into a1, then branches to another routine.
- `scan_forward_for_zero_byte`: Increments a1, tests the byte at a1 plus d0 for zero, loops back while nonzero, then returns a1 in d0.
- `copy_string_until_nul_2`: Loads destination from stack into a0 and source into a1, copies bytes from a1 to a0 incrementing both, stopping after a zero byte, then returns.
- `load_copy_args_and_zero_d1`: Loads the first two stack arguments into a0 and a1, loads the third into d0, sets d1 to zero, then branches to the shared copy loop.
- `byte_copy_until_d1_reaches_d0`: Copies bytes from a0 to a1, incrementing both pointers and d1, until d1 is no longer less than d0, then returns.
- `load_two_args_and_branch_to_copy`: Loads the first two stack arguments into a0 and a1, then branches to the shared byte copy routine without setting d0 or d1.
- `compare_strings_until_null_or_mismatch`: Advances a0 and a1 byte by byte, stopping at a null byte in either string or at a mismatch, then returns -1 in d0 if the final bytes differ and 0 if they match.
- `strlen_count_bytes_from_argument`: Loads the string pointer from the stack argument, scans forward counting bytes until a null terminator, and returns the count plus one in d0.
- `info_library_display_text_sequence`: Saves d2-d4/a2, calls a length routine on the last argument, then invokes three library routines through a2 offsets to output text with adjusted lengths and positions before restoring registers.
- `info_library_format_call`: Builds a stack frame, saves d2-d4, formats a buffer via a library call, then passes four arguments and the buffer to another routine before restoring registers and returning.
- `info_library_setup_byte_scan`: Saves d2 and a2-a3, loads a pointer and index from the stack, computes a byte offset, reads one signed byte into d1, and sets up a2 and d2 before branching.
- `info_library_shift_bytes_terminate`: Copies bytes from a2 plus a3 into a0 plus d0 while incrementing a2 until it exceeds d1, then writes a zero terminator and restores saved registers.
- `init_info_request_and_open`: Fills two caller-supplied structures with request fields and a ten-pixel offset, then calls the library open routine and returns true on success.
- `close_info_handle_return_true`: Pushes a zero and the handle at offset 62 of a3 onto the stack, calls the close routine, cleans the stack and returns one.
- `set_info_position_and_size`: Calls three library routines on the handle at offset 50 of a2 to set position and size, subtracting ten from the height and width arguments.
- `raw_do_fmt_via_exec_vector`: Saves a2-a4 and a6, loads format string and argument pointers from the stack, loads the exec library base from absolute address 4, calls RawDoFmt through its vector, then restores registers and returns.
- `store_byte_to_a3_and_advance_2`: Writes the low byte of d0 to the memory pointed to by a3 and increments a3 by one, then returns to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.

### version.library

- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_bases_in_private_struct`: Copies d0 into a1, then stores the a6 library base at offset 34 and the a0 value at offset 38 of that structure, then returns.
- `increment_refcount_clear_flag_return_self`: Adds one to the word at offset 32 of the library base in a6, clears bit 3 of the byte at offset 42, then returns the base pointer in d0.
- `decrement_refcount_release_when_zero`: Sets d0 to zero, decrements the word at offset 32 of the a6 library base, and if it reaches zero with bit 3 of offset 42 set, calls the release routine before returning.
- `open_library_set_busy_flag`: Saves d2 and a5-a6, builds a frame, loads the library base from offset 34 into a6, and if the count at offset 32 is nonzero sets bit 3 of offset 42 and returns zero in d0.
- `version_library_close_and_free_segment`: Saves the value at offset 38 of the library base, calls the library close and free-vector functions through a6, then returns the saved value in d0 after restoring d2 and a5-a6.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.

### Populous (1989)(Electronic Arts)[cr Flint Firforge].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
