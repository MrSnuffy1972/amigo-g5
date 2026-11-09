# Game 937 (BlackCrypt v2.0 0281): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00937). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (5368177 top-level bytes in 44 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 22 raw-word lines |
| Functions named and described | pass, 673 of 673 named (0 author symbols), 673 of 673 described (215 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2019 of 2019 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 144169 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11887 of 11948 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2035370 |
| code | 38438 |
| data | 3294357 |
| padding | 12 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 1175 | 77 |
| BlackCrypt | 3 | 1745 | 48 |
| bcdfp | 3 | 6150 | 184 |
| bcdfq | 2 | 1065 | 44 |
| bcdft | 7 | 29 | 2 |
| bcdfu | 2 | 1490 | 84 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 285 | 225 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| bcdfx (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- BlackCrypt_v2.0_0281/BlackCrypt_v2.0_0281.hdf: hard-disk image (AmigaDOS OFS, 5792 blocks): its filesystem parses and all 38 files in it are byte-identical to the extracted tree BlackCrypt_v2.0_0281_hdf, which is recovered file by file; no block outside the filesystem holds data
- 6 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 203 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- BlackCrypt_v2.0_0281/BlackCrypt_v2.0_0281_hdf/BlackCrypt/data/bcdfx: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BlackCrypt_v2.0_0281/BlackCrypt_v2.0_0281_hdf/BlackCrypt/data/bcdft: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 144169 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `return_minus_one_at_entry`: Loads the immediate value minus one into d0 and returns immediately to the caller, providing a constant return value at program entry.
- `store_base_and_jump_through_vectors`: Saves the incoming a0 base pointer to a global and a5, stores a routine address, calls two offsets through a5, then jumps through a pointer table.
- `set_two_a5_offsets_and_jump`: Loads a base pointer from pc-relative memory, adds a large offset, stores it at two a5 offsets, calls a routine, then jumps to a5.
- `set_a3_to_1mb_and_jump_a5`: Loads the immediate value an address into address register a3, then jumps to the address held in a5 without saving a return address.
- `return_a4_or_a3_minus_8`: Copies a4 into d0; if a4 is zero it copies a3 instead, then subtracts 8 from d0 and returns that value in d0.
- `read_pc_relative_word_mask_ff8f`: Loads the longword stored at the program-counter-relative offset 3970 into d0, clears all bits except the low byte and bits 4-6, and returns.
- `intreq_ack_and_rte`: Tests the interrupt request register at the custom chip base, restores d0-d1, a0-a1 and a5-a6 from the stack, then returns from the exception.
- `ack_intreq_from_stack_and_rts`: Writes the word on top of the stack to the interrupt request acknowledge register, advances the stack pointer by two and returns.
- `build_a3_frame_and_return_zero`: Reserves eight bytes on the stack, stores the caller return address and a3, loads a3 with a nearby data address, sets d0 to zero and d1 to a2, then returns.
- `set_a1_flag_bit1_and_jump_pc`: Pops return address into d1, pushes a pc-relative target, restores d1, sets bit 1 of the byte at offset 14 in a1, then returns.
- `load_custom_chip_base_and_branch`: Loads the custom chip register base address into a0 and branches to a fixed pc-relative target, touching the custom hardware register area.
- `store_d0_to_pc_relative_and_load_ciab`: Stores d0 into a pc-relative memory location, then loads the CIA-B base address into a6 and returns.
- `set_copper_jump_to_pc_relative_target`: Loads a longword from a program-counter-relative location into the copper jump address register at the copper list high address, then jumps through a0.
- `copy_a0_offset6_byte_to_a6_800`: Reads the byte at offset 6 from the structure pointed to by a0 and stores it at offset 800 from a6, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_machine_id_return_four_or_one`: Compares a program-counter-relative longword against two machine identifiers; returns 4 in d0 if it matches either, otherwise returns 1.
- `init_two_minus_two_sentinels_and_pointers`: Writes minus two as a longword sentinel twice around a3, stores the advanced a3 pointer into offsets of a4 and a2, clears a word at a4 offset, skips fourteen bytes of return address and returns.
- `copy_two_object_offsets_into_a3`: If the longword at a1 offset 4 is nonzero, copies the longword at that object's offset 4 into a3 offset 50; likewise copies the second object's offset 4 into a3 offset 54, then skips fourteen bytes and returns.
- `d0_nonzero_to_minus_one_flag`: Decrements d0, sets the byte to all ones if the result is nonzero, sign-extends it to a word and longword, and returns that value in d0.
- `modifies_d0_and_updates_vhp_posr`: Inverts the lower bits of d0 and checks it against a constant, then if certain conditions are met, writes a longword from a global address to the video host position register before returning.
- `push_minus_one_and_branch_to_cleanup`: Pushes the immediate value minus one onto the stack and branches forward into the shared cleanup code.
- `restore_registers_and_disable_intena`: Tests d0, and if nonzero pops saved registers from the stack and writes to INTENA at a fixed address, then rearranges stack words and returns via a computed jump.
- `compute_scaled_time_into_a2_offset`: Loads a pointer from a PC-relative table, reads three longs, multiplies the first by 1440 and adds the second, scales by 60, divides the third by 50, stores the sum at offset 198 of a2, then restores d2/a2-a3 and returns.
- `select_disk_menu_action_by_index`: Reads a byte index from a3, clears a1 offset 31, checks a3 flags, optionally calls a helper, then dispatches through a jump table to set a1 offset 31 and returns that byte in d0.
- `keymap_data_table`: The bytes are data, not executable code: they form a small lookup table of key values referenced by the disk menu routine, ending in a return byte.
- `store_bit_test_result_to_a1_offset`: Tests bit selected by byte at offset 67 of a3 in byte at pc-relative address, stores 0 or 1 to offset 32 of a1, then adds 44 to the stack pointer and returns.
- `push_pc_relative_address_plus_offset`: Pushes the longword at the pc-relative location onto the stack, adds a large immediate offset to that pushed value, then returns to the caller.
- `push_pc_relative_address_plus_offset_2`: Pushes the longword at the program counter offset onto the stack, adds a fixed immediate offset to it, and returns that computed address.
- `toggle_bit_and_branch_on_state_flag`: Clears a bit indexed by byte at offset 67 of a3 in a byte table, and if it was set, sets bit 1 of the byte at offset 64, increments the long at offset 294, calls a routine, clears bit 1 and increments again.
- `load_10001_and_jump_to_f4c`: Loads the immediate value 65537 into d1 and branches unconditionally to the routine at offset f4c.
- `read_byte_at_d2_offset_if_positive`: Tests d2; if negative returns immediately, otherwise loads the byte at a0 plus d2 into d3, copies d3 into d1 and returns.
- `set_d1_0x118_and_jump`: Loads the immediate value 280 into d1 and branches unconditionally to a routine further ahead, transferring control without returning here.
- `clear_a1_offset_c_and_store_a4_pointer`: Clears the longword at offset 12 of a1, loads 12 into d4, then stores a4 into a fixed memory location and returns.
- `zero_filled_data_area`: Contains only a zero word that decodes as ori.b with zero immediate; this is zero-filled data referenced by address, not executable code.
- `compare_a4_and_dispatch_via_a5`: Compares a4 with a stored value; if unequal jumps through a5, otherwise saves all registers, loads d0 from a1, optionally calls a subroutine, restores registers and jumps through a6.
- `install_pc_relative_pointer_into_global`: Loads a pc-relative address into a3, stores that address into memory at a3 minus offset, then loads another pc-relative address into a3 and moves it into d1 before returning.
- `alloc_and_init_io_request`: Counts the filename string in a0, allocates a buffer with AllocMem, copies the name, allocates a 44-byte structure, opens dos.library, looks up the file and links the new request into the exec list.
- `open_whdload_device_and_enqueue`: Allocates a 68-byte structure, sets its flags and name, opens expansion.library, builds a device node with a task entry, allocates a 20-byte message and enqueues it to the device.
- `open_dos_library_and_jump_entry`: Opens dos.library by name, takes the returned base, loads the pointer at offset 22 and jumps to it, passing control to the library entry.
- `whdload_task_message_loop`: Finds the current task, builds a message port, opens dos.library, then repeatedly waits for and gets messages, dispatching each by its command word through a jump table.
- `read_byte_and_word_into_d0_d1`: Calls two subroutines, moves d7 into d0, calls another, moves d7 into d1, loads a long from offset 28 of a4 into d2, calls a routine, shifts d0 right by two and branches based on the result.
- `read_byte_then_call_and_return_ff`: Calls a subroutine, moves d7 into d0, calls another subroutine, sets d0 to minus one, then branches to a common exit.
- `copy_stream_bytes_into_buffer`: Copies bytes from a stream buffer into a destination buffer, refilling via a library allocation call and a read routine, then returns the copied count in d0 and zero in d1.
- `stream_buffer_fill_and_flush`: Advances the stream position by the requested length, grows the high-water mark, allocates a buffer via library call when empty, copies bytes from the source pointer into the buffer, and flushes the buffer through a jump table call before resetting the counters.
- `flush_remaining_stream_bytes`: Moves the remaining byte count and position into d0 and d1, loads the stream base pointer into a0 and source pointer into a1, calls the flush routine through the jump table, then loads a constant and branches away.
- `set_d2_minus_two_and_branch`: Loads the value minus two into data register d2 and branches forward to another routine, setting up a negative return code before continuing.
- `store_allocated_block_at_offset_36`: Sets d2 to minus one, calls three subroutines with d7 as argument, and if the result is nonzero stores it at offset 36 of the d7 structure and returns minus one in d0.
- `free_block_then_chain_to_store`: Calls setup routines with d7, obtains a block pointer in d2, invokes a call through a2 offset 12, frees memory via a6 offset minus 210, then branches to the store routine.
- `flush_pending_audio_channel`: Checks that a5 matches the active channel, and if a pending sample pointer is set, calls the channel callback with its length and data, then frees the channel structure.
- `add_high_limit_to_d2_if_positive`: Loads a0 from offset 20 of a4 and d2/d1 from offsets 24 and 28, then if d1 is positive adds the longword at offset 148 of a0 to d2 before branching.
- `add_and_clamp_d2_to_limit`: Adds the longword at offset 20 of a0 to d2, compares with the limit at offset 148 of a0, and if not above stores d2 there and returns the old value in d0 with d1 zeroed.
- `set_error_code_ff_and_d1_negative`: Sets d0 to 255 and d1 to the bitwise complement of 36, then branches away to a common exit path.
- `get_a4_offset20_shifted_left2`: Loads the longword at offset 20 from a4 into d7, shifts d7 left by two bits and returns it to the caller.
- `get_a4_offset24_shifted_left2`: Loads the longword at offset 24 from a4 into d7, shifts d7 left by two bits and returns it to the caller.
- `get_a4_offset28_shifted_left2`: Loads the longword at offset 28 from a4 into d7, shifts d7 left by two bits and returns it to the caller.
- `shift_string_left_and_store_length`: Copies bytes from a0 forward one position, overwriting the first byte with the next, until a zero byte is found, then writes the count of shifted bytes just before the final position and returns.
- `allocate_and_init_io_request`: Allocates a 264-byte memory block via exec, fills fields at offsets 28 and 148, opens a device with a2, and stores d4, d2, a5 and a3 into the block before returning it in d0.
- `close_device_and_free_request`: Checks d0 for a request block, verifies a5 matches its owner field, clears that field, closes the device via exec, frees the block and jumps to the library free routine.
- `build_allocated_path_from_dir_and_name`: Allocates memory via the dos library, writes the combined length, then copies a directory prefix and a filename separated by a slash into the new buffer, collapsing duplicate slashes and trimming a trailing slash.
- `verify_whdboot_files_present`: Compares the WHDBoot.exe and WHDBoot.ss name strings against expected values using the string compare routine, returning zero only if both match.
- `compare_two_strings_until_mismatch`: Compares bytes at a0 and a1, advancing both pointers, and returns when a byte differs or the a0 string terminates; a0 is preserved on the stack.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `get_pc_relative_f9a_plus_32`: Pushes the longword stored at program-counter-relative offset f9a onto the stack, adds 32 to that stacked value, then returns to the caller.
- `save_regs_open_libs_and_run_game`: Saves registers to a buffer, opens exec and dos libraries, opens the game disks and save file, then jumps to the loaded game entry or exits.
- `copy_bytes_alloc_mem_and_call_target`: Copies d0 bytes from a0 to a buffer, allocates memory via the library, fills the allocation header, then jumps to the target address in a3.
- `xor_d0_with_byte_constant`: Exclusive-ORs the low byte of d0 with the immediate value 113, leaving the upper bytes of d0 unchanged, then falls through to the following code.
- `zero_filled_data_area_twelve_words`: Twelve consecutive zero words that decode as ori.b instructions; this is a zero-filled data area referenced by address, not executable code.
- `zero_word_data_slot`: A single zero word that decodes as ori.b #0,d0; it is a zero-filled data slot referenced by address, not executable code.
- `zero_filled_data_area_12_words`: Twelve zero words that decode as ori.b #0,d0 but are a zero-filled data area referenced by address, not executable code.
- `zero_filled_data_area_52_words`: Fifty-two zero words that decode as ori.b #0,d0 but are a zero-filled data table referenced by address, changing no registers.
- `match_command_name_and_dispatch`: Reads a length-prefixed name from a table indexed by d0, skips separators, compares it case-insensitively against command entries, then calls the library with the matched command and jumps to its handler.
- `patch_two_jsr_sites_to_trampoline`: Saves all registers, computes an address from d1 times four, and if two locations hold the same longword pattern, overwrites each with a jump instruction and the address of the trampoline routine.
- `check_flag_patch_nop_and_flash_color`: Copies a1 to a2, tests a global longword, and if set and a word at offset matches, writes a nop and repeatedly writes color register zero with a fixed value before branching onward.
- `walk_linked_list_and_call_trampoline`: Loads a pointer from a2, follows the list by shifting each node value left twice and adding the previous word, calling the trampoline for each node until a zero terminator is reached.
- `restore_a3_and_all_registers_return`: Stores a3 into the caller's stack frame slot at offset 60, then pops d0-d7 and a0-a6 from the stack and returns.
- `scan_memory_for_color_marker`: Walks a0 forward past words until it reaches a1, and whenever a longword equals the marker value it rewrites it and sets COLOR00 to 64.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### BlackCrypt

- `jump_to_main_entry_point`: Program entry that immediately jumps to the main startup code located relative to the program counter, transferring control without returning.
- `open_workbench_and_run_game_loop`: Opens the Workbench screen, locks the game disk, opens the configuration file, then loops reading commands and dispatching them until exit, finally closing the screen and restoring the old directory.
- `initialize_graphics_and_open_game_screens`: Opens the configuration file, allocates the screen and window, initializes bitmaps and color tables, opens the main screen and window, then calls the setup routine and closes the screen.
- `close_screens_windows_and_free_resources`: Closes the game window and screen, frees the allocated bitmap memory, closes the two opened files, and exits the program with the given return code.
- `set_dos_mode_unload_and_delay`: Sets word 6 at offset 20 of the DOS library base, unloads a segment via the library vector, calls a routine with a stored pointer, then delays 25 ticks and calls another library vector with a stored argument.
- `poll_input_until_exit_selection`: Loops calling a routine that returns a selection code, branching on values 1 to 4 to set a stored word, call a routine, or set the return flag, returning zero or one.
- `uppercase_name_from_index_table`: Reads a length byte from a table entry indexed by the second argument, copies that many characters to the buffer, uppercases lowercase letters, appends a colon and terminator, and returns the length.
- `check_volume_name_in_mounted_list`: Locks the dos library base, walks the mounted volume list comparing each entry's name against the passed string, and returns 1 if a match is found, else 0.
- `prompt_insert_disk_and_wait_for_volume`: Builds a volume name from the passed string, prints the insert-disk prompt, then repeatedly locks and examines the dos device list until the matching volume appears, returning its lock.
- `wait_for_second_library_call_result`: Calls two Amiga library vectors through a6, saving and restoring a6 around each call, and repeats until the second call returns a nonzero value in d0, which is returned.
- `load_game_segment_and_init_state`: Opens the BC Game file, loads the GAMEDISK1 segment into memory, stores the segment pointer and size in globals, clears three state longs and registers the segment with the dos library.
- `reload_segment_and_set_state_six`: Finds the loaded segment, checks its state word against the caller's value, sets the state word to six, unloads and reloads the segment, then stores the new segment pointer.
- `load_segment_and_jump_to_entry`: Loads the named segment via dos.LoadSeg, computes an entry offset from its size, saves all registers, jumps to the entry point, then unloads the segment and returns.
- `draw_string_at_fixed_columns`: Draws a null-terminated string at screen columns 9 and 10, then loops over remaining characters drawing each at successive columns starting at 12, offsetting each character by 32.
- `restore_d2_d4_a2_and_unlink_frame`: Pops registers d2 through d4 and a2 from the stack, releases the a5 stack frame and returns to the caller.
- `draw_centered_string`: Computes a starting column from half the string length subtracted from 20, then loops drawing each character at successive columns until the null terminator.
- `call_two_setup_routines`: Establishes an a5 stack frame, calls two nearby subroutines in sequence, then releases the frame and returns to the caller.
- `clear_bb80_byte_buffer_via_bltclear`: Saves a6, loads the graphics library base, then calls BltClear with size 48000 and flag 1 to clear a buffer, restoring a6 before returning.
- `build_main_menu_entries`: Pushes five menu label pointers with their positions, calls the entry-adding routines for each, then returns 5 as the menu item count.
- `check_input_event`: Polls an input event queue, checking message types and data fields to determine a return value based on specific conditions.
- `clear_bitmap_and_dispatch_by_index`: Calls BltClear on the bitmap at the graphics base, then dispatches on the argument to one of several subroutines before returning the result in d0.
- `store_d0_d1_d2_to_global_block`: Loads the address of a global data block into a0 and stores d0, d1 and d2 into its offsets eight, four and zero respectively, then returns.
- `blit_tile_column_to_bitplanes`: Copies 48 bytes from a tile row table into six bitplane rows spaced 40 bytes apart, repeating for six planes at 8000-byte offsets, using d0/d1 as column and row indices.
- `draw_two_tile_lists`: Walks two byte-pair lists, calling the tile blit routine for each entry, using d2 as a plane offset and skipping negative-terminated runs.
- `clear_and_fill_playfield_buffer`: Loops sixteen times, calling the OS library twice, filling a 32-byte buffer via a helper, then copying it to the playfield buffer with the library copy call.
- `decrement_packed_bcd_words_in_place`: For d0 words at a0, subtracts one from each packed BCD digit group (hundreds, tens, units) that is nonzero, writing results back and advancing a0.
- `repeat_screen_clear_and_blit_32_bytes`: Loops 16 times calling the graphics library twice, copying 32 bytes via a helper, then calling the library blit routine with a5 base and a2 source.
- `increment_packed_bcd_words_toward_target`: For d0 words, compares each a0 word's BCD digit groups against a1 source and increments any differing group, storing back to a0 and advancing both pointers.
- `copy_bytes_with_16bit_count`: Copies bytes from the source pointer in a1 to the destination pointer in a0, using the 32-bit count in d1 with a 16-bit dbra inner loop and high-word decrement, then returns the destination pointer in d0.
- `compare_strings_then_init_libraries`: Compares two byte strings from a0 and a1 returning zero, one or minus one, then clears a global buffer, saves the stack pointer and opens dos.library via exec, alerting on failure.
- `load_a4_global_base_pointer`: Loads the fixed value 32766 into address register a4 as the global data base pointer and returns immediately to the caller.
- `open_manx_file_and_init_stream`: Opens the MANX file via dos.Open with a computed buffer size, stores the file handle, sets stream fields, and initializes the stream through helper calls.
- `copy_d0_to_d5_callback`: Copies the value in d0 into d5 and falls through to the following code; used as a small callback address taken by a caller.
- `tokenize_manx_line_into_argv`: Reads a line from the MANX file, splits it into whitespace-separated tokens honoring quotes, allocates an argument vector, and stores pointers to each token.
- `move_d0_to_d0_passthrough`: Copies register d0 onto itself, a no-operation placeholder that leaves d0 unchanged before falling through to the next routine.
- `append_string_with_length_limit`: Scans a0 for the terminating null, then copies bytes from a1 into a0 while decrementing the 32-bit count in d1, stopping on a copied null or when the count goes negative, and writes a final null.
- `open_window_and_load_icon_images`: Opens the WINDOW dos device, gets the icon library base, opens the icon.library, allocates two icon image buffers, stores their pointers and sizes in the structure at a3, then closes the library and device.
- `append_string_to_string`: Scans a0 forward to its terminating null byte, then copies bytes from a1 including the null onto the end, returning the original a0 in d0.
- `bounded_copy_or_fill_bytes`: Copies bytes from a1 to a0 up to a 32-bit count in d1, stopping at a null byte, or fills a0 with null bytes when the count is zero, returning a0 in d0.
- `allocate_and_init_io_request_2`: Allocates a 34-byte IO request via exec, fills its fields from arguments and a file handle, queues it, and returns the request pointer or zero on failure.
- `close_and_reset_audio_channel`: If the channel's field at offset 10 is nonzero, calls the audio library close function on it, then marks byte 8 as 255, sets long at offset 20 to -1, and calls two more audio library functions with the channel.
- `strlen_return_length_in_d0`: Takes a string pointer from the stack, scans bytes until a zero terminator, and returns the length excluding the terminator in d0.
- `init_list_header_pointers`: Takes a pointer from the stack and stores into it its own address plus four, clears the long at offset four, and stores the pointer at offset eight.
- `copy_string_to_destination`: Loads destination and source pointers from the stack, copies bytes from source to destination until a null byte, and returns the destination pointer in d0.
- `drain_callback_list_and_cleanup`: Walks a linked list at a4-relative memory calling each node's function pointer, then invokes a second callback and a cleanup routine before returning.
- `shutdown_engine_release_resources`: Frees a table, calls per-index cleanup, invokes several optional callbacks, closes libraries and memory, restores the stack pointer from a4-relative storage, and returns the argument in d0.
- `pop_d2_d3_and_return`: Pops registers d2 and d3 from the stack and returns immediately to the caller; it is an empty routine with no other effect.
- `close_dos_file_by_index`: Looks up a file entry by index in a table, validates the index and entry, closes the dos file handle via the library, clears the entry, and returns zero or minus one on error.

### bcdfp

- `bcdfp_entry_jump_to_startup`: Program entry point that jumps to the startup code located relative to the program counter, transferring control to the main initialization.
- `open_libraries_and_init_display`: Opens graphics, intuition and other libraries, stores their bases in globals, sets up a display window structure, calls init routines and opens a screen with the given dimensions.
- `close_screen_and_free_libraries`: Closes the screen opened earlier, calls a cleanup routine with the stored pointer, then closes the graphics and intuition library bases and returns.
- `walk_linked_list_and_copy_entries`: Walks a linked list of 20-byte records starting from an index, copies each record to a buffer advancing the pointer, and recurses for entries of type 19 or 35.
- `init_four_character_slots`: Loops over four character records, sets slot type bytes, clears six-by-two grid words and five item words, sets a flag byte, calls a sub-initializer and fills stats and name fields.
- `init_party_member_stats`: Loops over four party members, sets index and stat words, adds two to two bytes, sums five item values into a long, then clears and sets global party state words.
- `write_character_save_block`: Builds a save buffer by copying character records, item records and party globals into memory, then writes the block to the CHARACTERS file via dos.Open and dos.Write.
- `wait_for_async_io_completion`: Calls the device begin and check library vectors in a loop until the check returns nonzero, then returns that result in d0 with a2 restored.
- `reset_counter_and_start_async_read`: Clears the word counter at a4 offset, sends the async read request via library vector, waits for completion, and aborts if the counter is not six.
- `set_counter_two_and_start_async_read`: Stores two into the word counter at a4 offset, issues the async read request through the library vector, waits for completion, and aborts if the counter is not six.
- `set_state_eight_and_run_dos_command`: Stores the value 8 in the state word, calls the DOS library vector through a6, runs the shared dispatcher, and if the state is not 6 calls the error routine with a zero argument.
- `dup_lock_current_dir_and_unlock`: Stores state 3 and the passed lock pointer, calls the DOS library, duplicates the current directory lock, then unlocks the original and returns the duplicate in d2.
- `set_state_seven_and_return_word`: Stores state 7 and the word argument into the state area, calls the DOS library and shared dispatcher, then returns the stored word in d0 unless the state equals 6.
- `show_message_and_wait_for_input`: Opens a message window with the given text and title, calls the display routine, and if the result code is not six, waits for a key press before returning.
- `load_and_verify_dungeon_file`: Prompts for a disk, opens the dungeon file, reads a header and data block, compares sizes, and returns one on open failure, two on size mismatch, or zero on success.
- `save_game_with_disk_prompt`: Locks the current directory, opens the save file, writes game state, prompts for a disk if needed, and restores the directory lock before returning.
- `process_input_event_dispatch`: Reads event type and key code from globals, dispatches to movement, menu or action routines, and loops until the quit key sets a flag, then closes the event.
- `draw_inventory_grid_and_items`: Clears the cursor, draws the inventory panel and grid, then loops over four rows and six columns calling the item draw routine for each cell.
- `draw_nine_inventory_slots`: Loops nine times, reading x and y coordinates from the slot table and calling the draw routine with the row offset from the stack argument.
- `draw_selected_slot_icon`: Reads the slot index from the stack argument, computes its record offset in the table at a4-32440, loads the word at offset 36 and calls the draw routine with that value and 150.
- `update_slot_and_redraw_twice`: Computes a slot record offset from the counters at a4-29132 and a4-29138, stores the decremented value at offset 36, then calls the redraw routine twice with the current slot index.
- `switch_active_slot_and_repaint`: Draws the old slot highlight, sets the active slot from the argument, redraws the new slot highlight, then formats the slot number into a buffer and calls the string routine.
- `redraw_map_view_after_scroll`: Compares the map scroll offset against the previous value, redraws the viewport via the scroll routine, then dispatches on the returned tile code to call the map update and draw routines before restoring registers.
- `find_map_cell_at_screen_point`: Scans ten map cell rectangles in the table at a4-32480, comparing the passed x and y coordinates against each cell's bounds, and returns the one-based index of the matching cell or zero.
- `increment_slot_counter_decrement_charges`: Reads a slot index from the stack argument, checks a per-slot word against 20 and a charge counter, then increments the slot word, decrements the charge word and calls a helper twice with the slot and value 5.
- `draw_creature_hp_digits`: Reads two word parameters, computes a table offset from a4, divides a stored value by ten, then calls the draw routine to place one or two digit glyphs at computed screen positions.
- `draw_creature_status_icon`: Uses the creature index parameter to locate its record in the a4 table, computes screen coordinates, draws a background box, and optionally draws a status icon and number.
- `check_selected_creature_present`: Looks up the currently selected creature index from a4, calls the record lookup routine, and returns after popping d2 if the result is zero.
- `clear_selected_slot_byte_if_flags_clear`: Calls a status routine twice; if bits 0 and 1 of the result are clear, it clears one byte in the record at offset -32440(a4) indexed by the word at -29132(a4) times 38 plus d2 minus one.
- `clear_slot_byte_and_refresh_display`: Clears a byte in the record at -32440(a4) indexed by the word at -29132(a4) times 38 plus d2 minus one, then pushes arguments, calls three routines and branches back to the caller's loop.
- `read_char_into_slot_and_redraw`: Reads a character via a routine on the record at -32440(a4), stores it at the returned index, clears the next byte, then calls the redraw routines with the word at -29132(a4).
- `find_letter_index_in_table`: Scans a table of 26 words at a4-32556 comparing each to the word argument; returns index plus 65 on match, or zero if none found.
- `bcdfp_ori_data_table`: Ten ori.b immediate words whose bytes are data, not executable code; no branch or call is present, so it is a data table.
- `alloc_mem_1cc50_flags_10002`: Calls a subroutine, then invokes exec AllocMem via a6 with size an address and flags 65538, restoring a5 and a6 before returning.
- `allocate_and_init_graphics_context`: Allocates a 1124-byte memory block, stores the caller's d0 and a hardware base pointer, opens graphics.library, allocates several buffers, fills a table of offsets and returns zero on success or a nonzero error code.
- `run_two_stage_setup_sequence`: Saves a5 and a6, calls the setup routine at offset 3524, loads the address of the data at 2162 into a0 and calls the routine at 2178, then restores a5 and a6 and returns.
- `open_graphics_and_load_bitmaps`: Opens graphics.library through the a5 library base, then for each descriptor in the a0 table loads a bitmap from a5-relative offset and finally closes the library, returning after restoring registers.
- `load_a5_a4_a6_from_pc_relative_globals`: Loads a5 from a pc-relative global pointer, then loads a4 from offset 96 of a5 and a6 from offset 1104 of a5, and returns.
- `close_libraries_and_free_memory_blocks`: Walks two pointer tables, closing each non-null exec library via CloseLibrary, frees the block at offset 112 of a5 with FreeMem, then closes the library at the global pointer.
- `initialize_display_and_interrupt_state`: Saves registers, calls setup routines, copies copper and interrupt vectors into a5 fields, programs CIAA interrupt control, installs a new level handler and sets interrupt enable masks.
- `init_cia_and_copper_display_setup`: Saves registers, calls three setup routines, programs CIAA control and interrupt registers, loads copper list pointers from a5 offsets into a6 fields, and clears CIAA CRA bits before returning.
- `wait_ciaa_flag_bit5_set`: Polls bit 5 of the CIAA control register at offset 30 of a6 until it is set, then writes 32 to the a6 offset 156 field and returns.
- `wait_ciaa_flag_bit5_counted`: Repeats d0 times: waits until bit 5 of the CIAA control register at a6 offset 30 is set, then writes 32 to a6 offset 156, decrementing d0 until it wraps.
- `clear_byte_and_wait_loop`: Loads a pointer from pc-relative data into a0, multiplies d0 by a word at offset ce, then repeatedly clears byte at offset 458 and spins until it reads back nonzero, decrementing the count.
- `save_regs_call_build_display_list`: Pushes d2-d7 and a2-a4 onto the stack, calls the display list builder, then restores those registers and returns to its caller.
- `build_copper_display_list`: Writes a long sequence of copper instructions into the buffer pointed to by a5 offset 20, storing pointers at various a5 offsets and ending the list with a terminator.
- `copy_two_18_long_blocks_to_struct`: Copies 18 longs from base plus offset 41000 into the structure at a5 offset 24, then copies 18 more longs from the following source area into the structure at offset 84.
- `get_base_pointer_plus_96`: Loads the pointer stored at the pc-relative global, reads the long at its start, adds 96 and returns the result in d0.
- `store_word_to_base_offset_1068`: Loads the pointer stored at the pc-relative global and writes the low word of d0 to offset 1068 from that base.
- `toggle_sprite_palette_entries`: Calls a subroutine, then either clears a flag and fills fourteen longwords with a fixed color value, or fills seven longwords at offset four with another fixed value and sets the flag.
- `copy_seven_words_to_sprite_buffer`: Calls a subroutine, stores its result and minus one into two words near a5, then copies seven words from a pc-relative table into the buffer at offset six.
- `dispatch_ten_subroutines_via_stack`: Saves all registers, loads a5 and a6 from globals, toggles a byte and increments a word, pushes ten routine addresses and branches to a dispatcher.
- `pop_a0_and_jump`: Pops a return address from the stack into a0 and jumps to it, acting as a tail-call trampoline for the thirteen callers.
- `vblank_irq_set_intena_bits`: Reads the interrupt enable word at offset 30 of a6, masks it with 112, stores it at offset 156 of a6, sets byte at offset 1112 of a5 to 1, restores registers and returns from interrupt.
- `irq_save_call_set_intena_8`: Saves all registers on the stack, calls the routine at the given offset, writes 8 to the interrupt enable word at offset 156 of a6, restores registers and returns from interrupt.
- `jump_to_shared_exit_272c`: Branches unconditionally to the shared exit routine at the earlier address, transferring control without touching any registers or memory.
- `toggle_cursor_blink_and_redraw`: If the blink enable flag at offset 1070 in a5 is set, decrements the countdown at 1072; when it reaches zero, flips the toggle at 1074 and either blanks seven words at offset 6 of the pointer in 1076 or copies seven words from a table into them, then jumps to the shared exit.
- `restore_cursor_from_table`: Sets the countdown at offset 430 in a5 to twenty-six, then copies seven words from the table at the program counter relative address into offset 6 of the pointer stored at 434, and jumps to the shared exit.
- `copy_palette_and_build_color_ramp`: If the flag at offset 997 is set, clears it, copies twelve words from the a5 buffer to the copper list, then writes seven color entries from a table indexed by the word at 1068 and resets that index.
- `init_display_buffer_and_read_clock`: Sets a display pointer from the base plus 1024, reads the clock byte at absolute address 530 into counters, divides by ten, then calls two setup routines and stores their returned bytes.
- `cycle_color_ramp_every_third_frame`: When the enable word is set and the frame counter is divisible by three, advances the color pointer, reloads the ramp table on zero, and writes the next color word to the display structure.
- `store_cursor_bytes_and_continue`: Calls a routine to read two bytes, stores them at offsets 196 and 197 of a5, calls another routine, then jumps onward.
- `check_cursor_flag_and_compare_byte`: Tests word at offset 192 of a5, branches if zero, calls a read routine, compares result with byte at offset 196, and may call a routine with d0=64.
- `scan_rect_list_for_cursor_position`: Loops over 10-byte records at offset 208 of a5, testing flag bit 32 and comparing cursor coordinates at offsets 130 and 132 against each record's bounds.
- `set_mode_two_and_call_with_byte8`: Loads d0 with 2, reads byte at offset 8 of a0 into d1, sign-extends it to a word, calls the routine at 336a, then branches to 2974.
- `set_mode_32_then_check_c5_flag`: Loads d0 with 32 and d1 with zero, calls 336a and 303e, compares d1 with byte at offset c5 of a5, and if d1 is zero loads d0 with 256 and calls 336a again.
- `scan_entity_table_for_point_in_bounds`: Walks 10-byte records at offset 208 of a5, skipping records without the 128 flag bit, comparing record bounds against words at offsets 130 and 132 of a5 until a negative terminator is found.
- `set_d0_four_read_a0_offset_eight`: Loads d0 with 4, reads the byte at offset 8 of a0 into d1, sign-extends it to a word, calls the routine at 336a, then branches to 29e0.
- `read_keyboard_byte_and_update_counter`: Sends 128 to the keyboard routine, stores returned bytes at offsets 196 and 197 of a5, reads a key, and decrements the byte at offset 200 of a5 if positive before jumping to 272c.
- `poll_ciaa_keyboard_serial_register`: Toggles the CIAA CRA handshake bit, reads CIAA ICR and SDR, decodes a keyboard scan code, clears or sets a bit in the byte at offset 202 of a5, and returns minus one or zero in d0.
- `set_bit_in_a5_offset_ca`: Sets the bit selected by d1 in the byte at offset 202 from a5, then returns -1 in d0.
- `read_byte_at_a0_offset_ca`: Loads a pointer from a program-counter-relative longword into a0, reads the byte at offset 202 from it into d0 and returns.
- `reset_display_state_and_clear_buffer`: Clears several a5 state bytes and words, writes 15 to the byte pointed to by the a5 pointer, then zeroes 336 words starting at the pointer in a5 offset 4.
- `check_flag_and_decrement_timer`: Tests byte at offset 1084 of a5, and if zero writes 4096 to the word pointed to by a5+974, sets byte a5+985 to 1, then tests byte a5+984 and decrements the byte at a0 by 4 if set.
- `advance_cursor_and_copy_column_bytes`: Decrements the byte at a0, and when it underflows resets it to 15, calls a helper, then copies eight pairs of bytes from a0 and a1 into a4 at stride 82, advances d3 by 2 wrapping at 42, and updates the sprite pointer.
- `increment_column_index_and_copy_row`: Reads byte a5+986, returns if it equals 8, otherwise increments it, stores it back, and copies eight words from a table indexed by the value into a1+6 at stride 12.
- `compute_two_keymap_offsets`: Calls the keymap index routine twice, saves the first result on the stack, then adds the base pointer at offset 40488 to both results, returning the second in a0 and the first in d0.
- `fetch_next_keymap_byte_scaled`: Reads the byte at the current keymap index, decrements the index wrapping to 79, stores it back, and returns the byte minus 32 shifted left by 3; if index equals the limit, clears the flag byte and returns zero.
- `clear_keymap_flag_return_zero`: Sets d0 to zero and clears the byte flag at offset 984 of the a5 frame, then returns to the caller.
- `process_string_bytes_until_nonzero`: Saves a2/a5/a6, calls a setup routine, then walks bytes from the returned a0 pointer, calling another routine per byte and setting the byte flag at offset 984 of a5 until a nonzero result or terminator.
- `store_byte_into_ring_buffer`: Writes d0 into the buffer pointed to by a5 offset 100 at index from a5 offset 978, decrements the index wrapping to 79, and returns 1 if the index differs from the limit at a5 offset 980.
- `reset_ring_buffer_index_return_zero`: Stores d1 into the ring buffer index at a5 offset 978 and returns zero in d0.
- `decrement_nine_timers_at_a5_43c`: Walks nine consecutive words starting at offset 1084 from a5, decrementing each non-zero word by one, then jumps to another routine.
- `init_projectile_state_block`: Fills a block of fields relative to a5 with fixed position, velocity and limit values, copies a word from a6 into two bytes, calls a setup routine, replicates a longword four times and clears remaining counters.
- `update_projectile_position_and_clamp`: Depending on the byte at 166, either zeroes velocity fields or adds scaled velocity to subpixel accumulators, clamps the resulting x and y positions to the stored limits and stores them back.
- `clamp_d2_d3_to_window_bounds`: Compares d2 with the word at offset 172 in a5 and d3 with the word at offset 170, replacing d0 or d1 with those bounds when exceeded, then branches onward.
- `store_clamped_coords_and_dispatch`: Clamps d1 to the word at offset 174 in a5, stores d0 and d1 to offsets 130 and 132 in a5, calls two subroutines, then jumps to another routine.
- `offset_stored_coords_by_deltas`: Loads the words at offsets 130 and 132 in a5 into d2 and d3, adds the deltas at offsets 138 and 140, then branches to the clamping code.
- `clear_two_entity_structs_and_jump`: Loads pointers from offsets 24 and 28 of a5, zeroes fields at offsets 0 and 80 of the first and 0, 104, 208 and 312 of the second, then branches to another routine.
- `update_angle_bytes_and_compute_deltas`: Saves d2-d5, stores the low and high bytes of a6 offset 10 into a5 offsets 128 and 129, then computes absolute angle differences in d0 and d1 with sign handling.
- `finish_angle_delta_computation`: Complements d0 against 255, negates it if d5 is set, then computes the absolute signed difference of d1 minus d3 with sign handling and returns both in d0 and d1.
- `negate_d1_if_d5_nonzero`: Loads 255 into d4, subtracts d1 from it, stores the result in d1, and negates d1 when d5 is nonzero before popping d2-d5 and returning.
- `store_cursor_position_ring_buffer`: Saves d0 and d1 into a four-entry ring buffer at offset 142 of a5, advances the index at 158, and writes the position into the structures pointed to by 24(a5).
- `cycle_sprite_frame_and_write_offsets`: Decrements a frame index byte wrapping from 0 to 3, reads two words from a table plus offsets, optionally calls a draw helper, then writes the resulting coordinates into four sprite entries in the structure at offset 28 of a5.
- `encode_tile_coords_to_bitfield`: Adds a5 offsets 134 and 136 to d0 and d1, then packs the low bits of the resulting coordinates and d2 into d5 and d4, returning the packed values in d0 and d1.
- `read_fire_button_and_cia_flag`: Tests bit 6 of CIAA_PRA and bit 2 of the byte at offset 22 in a6, storing each inverted result as a byte in d0 and d1.
- `read_mouse_and_restore_interrupt_mask`: Saves INTENAR, disables interrupts, stores d0 and d1 bytes at a0 offsets 166 and 167, reads JOY0DAT into a0 offsets 128 and 129, then restores the interrupt mask.
- `set_audio_period_and_volume`: Saves d2, reads INTENAR, disables the level 14 interrupt via INTENA, writes d0 and d1 words to offsets 130 and 132 of the audio structure, then re-enables the interrupt if it was previously enabled.
- `store_audio_pointer_pair`: Loads the audio structure pointer from the PC-relative longword and stores the d0 and d1 words at offsets 138 and 140, then returns.
- `build_display_list_from_ring_buffer`: Calls a helper, then depending on flags at offsets 192 and 200 of a5 either emits a fixed list word, copies two longwords from a ring buffer entry into the output at a0 and clears the source, or emits a position list and decrements the counter.
- `reset_audio_state_and_clear_buffer`: Calls a setup routine, saves a status word, sets register 154(a6) to 16384, clears three words at 186-190(a5), zeroes 256 bytes at pointer 92(a5), then sets 154(a6) to 49152 if bit 14 of the saved word is set.
- `set_audio_flag_word_to_one`: Loads the pointer from the PC-relative long at 35ce into a0 and writes the value 1 to word offset 192 of that structure, then returns.
- `clear_audio_flag_word_to_zero`: Loads the pointer from the PC-relative long at 35ce into a0 and writes zero to word offset 192 of that structure, then returns.
- `clear_sound_bit_and_maybe_mute`: Loads a pointer from a PC-relative long, clears bits in d0, stores the masked word at offset 194 of that structure, and if bit 0 is clear writes 255 at offset 200.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `enqueue_event_if_masked_and_room`: Tests d0 against the mask at offset 194 of a5, increments the count at offset 190 if below 128, finds the end of the event list at offset 92, stores the new entry pointer in the ring at offset 182, and writes d0, d1 and two words from a5 into the entry.
- `store_event_at_ring_slot`: Stores the a0 pointer into the ring buffer at offset 182 indexed by the word at offset 186, advances that index wrapping at 128, then writes d0, d1 and two words from a5 into the entry and restores d2.
- `compute_frame_delay_from_speed`: Reads a word from the structure pointed to by the PC-relative pointer, multiplies it by d0, divides by 60, and stores the result (or 1 if zero) at offset 201 of that structure.
- `copy_sprite_list_to_display_buffer`: Calls a setup routine, then copies sprite entries from a0 into the buffer at a5+208, writing x, y, x+width-1, y+height, and a word per entry until a null link, ending with -1 and setting the control word at a6+154.
- `build_font_bitmap_and_blit_columns`: Clears the glyph buffers, expands each character of the string into 8-byte and 48-byte font tables, then fills six blitter descriptors at a6+64 with source, destination and size words and waits for the blitter busy bit.
- `load_a5_base_and_a6_from_offset`: Loads the a5 base pointer from a program-counter-relative longword and then loads a6 from offset 1104 of that base before returning.
- `init_double_buffer_state_and_audio`: Clears buffer flags and pointers in the a5 structure, sets two buffer pointers, writes 33792 to a6 offset 150, initializes counters and a function pointer, then returns.
- `swap_buffers_and_copy_state_block`: Copies six longwords from the current buffer pointer into the a5 state area, toggles the buffer flag, swaps the two buffer pointers, and waits on the busy flag before returning.
- `lookup_1c_byte_record_by_index`: Saves d2/a5-a6, calls the shared setup routine, multiplies d2 by 28, adds the result to a table base near the program counter, calls the second routine with that pointer, then restores registers and returns.
- `store_240_byte_slot_pointer`: Saves d2/a5-a6, calls the shared setup routine, computes d2 times 576 plus 96 as a pointer, stores it at offset 2 of a nearby table, calls the second routine, then restores registers and returns.
- `scan_a2_records_until_negative_word`: Saves a2, copies a0 into a2, calls the routine at the next address, advances a2 by 10 bytes and repeats while the word at a2 is non-negative, then restores a2 and returns.
- `setup_tile_blit_params`: Reads a tile descriptor at a0, computes screen offsets and bit masks, and stores blit parameters into the a6 structure for two rows of five bitplanes.
- `copy_args_and_run_blit`: Copies three longword arguments from the stack into a global parameter block, then calls the setup and blit routines to draw the tile.
- `save_frames_call_blit_pair`: Pushes a5 and a6, calls the parameter setup routine and then the tile blit routine, leaving the saved registers on the stack.
- `setup_blit_pointers_from_rect`: Reads a rectangle record via a2, computes source offsets from its coordinates, stores blitter pointers at 80 and 84 of a6, sets modulo and control words, then waits on blitter busy and jumps to the saved return address.
- `setup_blit_from_object_record`: Stores d0 and d1 into the object record, computes source and destination pointers from its fields, programs blitter control, modulo and size registers at a6, waits for blitter busy and returns.
- `clip_and_setup_blit_region`: Clips the object's rectangle against screen bounds, computes row and column offsets, programs blitter pointers, modulo, size and control words at a6, waits for blitter busy and returns.
- `palette_fade_step_forward_or_backward`: Steps a 16-entry Amiga color palette at offset 1108 in the a5 context, decrementing or incrementing each nibble of the color words and writing the resulting color index bytes to the hardware register at 970(a5).
- `decrement_palette_nibbles_in_place`: Walks d0 color words starting at a0 plus two, subtracting one from each nonzero nibble of every word and storing the modified words back in place.
- `step_palette_toward_target_nibbles`: Walks d0 color words at a0 plus two alongside target words at a1, incrementing each nibble that differs from the target and storing the adjusted words back in place.
- `clear_five_sprite_buffers`: Loads a pointer table from the a5 structure, then for each of six entries clears two thousand longwords in the pointed-to buffer, advancing through the table.
- `init_sprite_slots_from_table`: Loops over sprite slots three down to zero, reads a state byte from a table, and sets up each slot's fields and the a6 status word accordingly.
- `setup_sprite_slot_from_descriptor`: For a slot whose state byte is one, copies descriptor fields into the a4 slot, computes the data pointer via a5, and sets the a6 status bit.
- `store_display_list_and_clear_counter`: Writes 16 to offset 4 of a4, copies the longword at offset 32 of a5 to offset 0 of a4, loads the address of a nearby table into a0, decrements the byte at a0 indexed by d3, loops with dbra d3, then branches away.
- `zero_filled_data_area_2`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_block`: Four consecutive zero words decoding as ori.b #0,d0; this is a zero-filled data block referenced by address, not executable code.
- `zero_filled_data_area_3`: Two zero words that decode as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `register_dispatch_table_entry_setter`: Reads three stack arguments, writes a type byte and a word into two tables indexed by the first argument, and stores a computed code pointer into a third table.
- `format_string_with_two_args`: Saves d2 and a2, pushes two stack arguments and a pc-relative format string, calls a formatting routine, returns its result in d0 and restores registers.
- `copy_bytes_with_64k_borrow`: Copies bytes from a1 to a0 for a 32-bit count in d1, decrementing the high word when the low word wraps, and returns the original destination in d0.
- `load_a4_absolute_base`: Loads the absolute address 32766 into a4 and returns immediately, establishing the small-data base pointer for the caller.
- `open_dos_file_and_init_handle`: Opens a DOS file via the library, fills a handle structure with MANX tag, current directory, input and output streams, and calls setup routines before returning.
- `save_d0_to_d5`: Copies the longword in d0 into d5, preserving the value for the following routine.
- `parse_text_into_token_array`: Allocates a buffer, copies a source string, splits it into whitespace-separated tokens honoring double quotes, and stores pointers to each token in a newly allocated array.
- `copy_d0_to_itself`: Moves the longword in d0 back into d0, an empty no-op routine that returns immediately.
- `bounded_strcpy_into_buffer`: Scans a0 for the terminating null, then copies bytes from a1 into a0 until a null byte or the 32-bit count in d1 expires, writing a final null and returning the original a0 in d0.
- `open_window_and_load_icon_images_2`: Opens a dos window, gets the window's screen, loads an icon image, allocates two memory blocks, stores the image pointer and block handles into the structure at a3, then closes the window and frees the library base.
- `copy_d0_to_d5_and_fall_through`: Moves the value in d0 into d5 and then continues into the following routine, since this single instruction has no return of its own.
- `append_string_to_end`: Loads two pointers from the stack, scans the first string for its terminating zero, then copies bytes from the second string including the null terminator, returning the first pointer in d0.
- `bounded_string_copy_or_fill`: Copies bytes from source to destination up to a 32-bit count, stopping at a null byte, or fills the destination with zeros when the count is zero, returning the destination pointer in d0.
- `allocate_and_init_file_handle`: Gets a file handle via dos library, allocates a 34-byte structure, stores the name pointer, mode byte, flags and handle, then opens or initializes the file and returns the structure pointer.
- `close_stream_and_free_buffer`: Pops a2, loads the stream pointer from the stack, and if its field at offset 10 is nonzero calls the close routine; then sets byte 8 to 255, long at 20 to -1, and calls two more library functions with the stream.
- `format_string_into_buffer`: Formats the format string in a2 into the output buffer tracked by a3, handling percent conversions, width, precision and length flags, calling a character emitter and a string length routine, and returns the number of characters written or -1.
- `strlen_return_length`: Loads the string pointer from the stack, scans forward until a zero byte, then returns in d0 the length excluding the terminator.
- `init_self_referential_struct`: Loads the caller's argument pointer from the stack, stores the pointer at its own start, adds four to that stored value, clears the second longword and stores the pointer in the third longword.
- `iterate_struct_table_call_4ec8`: Saves a2, walks a table from a fixed offset up to an end offset in steps of twenty-two bytes, calling the routine at 4ec8 with each entry pointer, then restores a2 and returns.
- `buffered_stream_write_byte`: Validates a stream structure, flushes pending output, then writes one byte into its buffer, updating pointers, flags and the flush callback, returning the byte or minus one on error.
- `check_object_flag_and_return_status`: Sets up a stack frame, saves d2-d4 and a2, loads the object pointer from the caller's argument, and returns zero if the pointer is null or its word at offset 12 is zero, otherwise returns minus one.
- `process_object_fields_and_clear`: Tests flag bits in the object at a2, calls subroutines to handle fields at offsets 8, 14 and 20, formats a five-digit decimal string from the word at offset 20, then clears the object's first fourteen bytes and returns minus one if any call or the digit test set d2, else zero.
- `return_zero_to_object_processor`: Loads zero into d0 and branches back into the object-processing routine's exit path, providing a zero return value for that caller.
- `advance_a5_and_d0_by_two_and_eight`: Adds 2 to address register a5 and adds 8 to the low byte of data register d0, then falls through to the next routine.
- `scan_entries_for_pending_work`: Walks a table of 22-byte entries starting at a fixed offset from a4, skipping entries with zero or masked status flags, and calls a sub-routine on each candidate, returning minus one on failure or zero otherwise.
- `advance_entry_pointer_and_check_end`: Adds 22 to a2, loads the table end address from a4, and if a2 is still below it branches back into the scanning loop; otherwise sets d0 to zero and branches to the exit.
- `push_minus_one_and_a2_call_4d48`: Pushes the immediate value minus one and register a2 onto the stack, calls the routine at pc-relative offset 4d48, then discards both pushed longs and branches onward.
- `negate_d0_if_d4_set_restore_d1_d4`: Tests d4; if nonzero negates d0, then pops d1 and d4 from the stack, tests the resulting d0 and returns to the caller.
- `signed_divide_d0_by_d1`: Saves d1 and d4, takes absolute values of d0 and d1 while tracking signs in d4, calls the division helper, then returns the quotient in d0 with the correct sign.
- `divide_d1_by_d0_return_quotient`: Saves d1, calls the division routine at the next address, moves the quotient from d1 into d0 as the return value, restores d1 and tests d0 before returning.
- `divide_d1_by_d0_keep_d1`: Pushes d1, calls the division routine, pops d1 back to preserve it, then tests d0 and returns without changing the caller's d1.
- `unsigned_32bit_divide_d1_by_d0`: Divides the 32-bit value in d1 by d0, using a fast word division when the high word of d1 is zero and a 16-step shift-subtract loop otherwise, returning quotient in d0 and remainder in d1.
- `init_stream_buffer_and_flags`: Reads a byte from the object, tests it via a helper, allocates a 1024-byte buffer, stores the pointer and size, sets flag bits in the word at offset 12, and initializes the read pointers.
- `start_list_walk_setup_a6`: Saves a2, a3 and a6, loads the argument from the stack into a6, clears a3, loads a global pointer into a2 and branches into the list-walking code.
- `walk_list_until_sentinel`: Loops through a linked list starting at a2, comparing each node against a6 and advancing a2 through the next pointer until a null or matching node is found, then restores registers and returns.
- `copy_a2_longword_to_a3_or_return`: Moves a3 into d0, and if a3 is zero returns; otherwise copies the longword at (a2) to (a3) and branches onward.
- `write_a2_longword_and_call_library`: Stores the longword at (a2) into a4-relative memory, saves a6, loads a6 from a4-relative memory, adds 8 to the longword at offset 4 of a2, sets a1 to a2, calls library offset -210, restores a6 and branches back.
- `save_a2_a3_and_load_a2_pointer`: Pushes a2 and a3 onto the stack, loads a2 from a4-relative memory, then branches onward to shared code.
- `free_list_nodes_via_exec`: Walks the linked list at a2, freeing each node's payload through the exec library call, then clears the list head pointer and returns.
- `allocate_and_prepend_list_node`: Allocates a node of the given size via exec, links it at the head of the list, stores the size and returns a pointer past the node header, or zero on failure.
- `copy_string_return_destination`: Copies bytes from the source string at a1 to the destination at a0 including the terminating null, then returns the original destination pointer in d0.
- `check_file_entry_interactive`: Validates the file index argument against the table bounds and entry pointer, sets error code 3 and returns -1 if invalid, otherwise calls dos.IsInteractive and returns 1 if interactive else 0.
- `seek_file_entry_with_error_check`: Validates the file index, calls the seek helper with offset and mode, and on failure stores dos.IoErr into the error variable and returns -1, otherwise returns the seek result.
- `delete_file_and_store_ioerr`: Calls dos.DeleteFile with the given name, and if it fails stores dos.IoErr into the error variable and returns -1, otherwise returns 0.
- `return_zero_and_jump_to_5300`: Sets d0 to zero and branches unconditionally to the routine at the preceding address, sharing its return path.
- `write_record_via_dos_seek_write`: Validates a record index, checks its flags, optionally calls a callback, then seeks and writes the record through the dos library, returning the byte count or minus one on error.
- `check_signal_bit_and_clear_flag`: Calls the library function with d1 set to 4096 and d0 zero, tests bit twelve of the result, and returns it or zero after clearing a flag.
- `write_four_bytes_to_output`: Saves d2, d3 and a6, opens dos.Output, writes four bytes from a pc-relative buffer, closes the file, then calls the exit routine with status one.
- `run_exit_handlers_and_terminate`: Walks the linked list at the a4-relative exit-handler pointer, calling each handler and advancing to the next node, then calls the single cleanup pointer if set and finally invokes the termination routine with the caller's argument.
- `shutdown_game_and_release_resources`: Calls per-slot cleanup routines, frees the allocated block via exec memory calls, invokes optional callbacks, closes dos handles, checks the exec task flags, releases remaining library resources and restores the saved stack pointer before returning the argument in d0.
- `pop_d2_d3_and_return_2`: Pops registers d2 and d3 from the stack and returns immediately to the caller; no other work is done.
- `close_dos_file_by_index_2`: Looks up a file entry by index, closes its dos handle via dos.Close if open, clears the entry pointer, and returns zero or minus one on invalid index.

### bcdfq

- `bcdfq_program_entry_setup_and_run`: Program entry that opens dos, graphics and intuition libraries, copies screen pointers, enables DMA, then runs the game loop and restores DMACON before returning.
- `read_ciaa_fire_button_state`: Clears d0, tests bit 6 of the CIA-A port A register at the fire button address, and sets d0 to 1 if the bit is clear, otherwise leaves it zero, then returns.
- `read_copper_control_bit_two`: Loads the custom chip base into a0, clears d0, tests bit 2 of the register at offset 22 from a0, and sets d0 to 1 when that bit is clear before returning.
- `wait_for_two_a5_flags_clear`: Tests the word at offset 34 from a5 and returns if zero, otherwise loops testing the word at offset 36 from a5 until it is nonzero, then returns.
- `fade_palette_entries_over_105_frames`: Loops 105 times, disabling and enabling interrupts and waiting for the vertical blank, each pass decrementing four channel counters and writing scaled colour values to the destination table.
- `open_bcdfr_file_into_slot`: Calls dos.Open on the bcdfr file with mode 1005, stores the returned file handle at offset 28 of the a5 structure and returns 1 if opening failed, 0 otherwise.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `copy_second_screen_buffer_to_first`: Copies 3000 longwords from the buffer at a5 offset 12 plus 48000 bytes back to the buffer at a5 offset 12, then returns.
- `clear_bitmap_and_copy_12000_longwords`: Calls graphics BltClear on the bitmap at offset 12 of a5 with size 48000, then copies 8000 longwords from that bitmap plus 48000 bytes back to its start.
- `read_chunks_into_buffer_at_48000`: Reads successive word-sized lengths from a table, calls dos.Read into the buffer at a5 offset 12 plus 48000, advancing the pointer, and stops when a length is zero or a5 offset 36 is set.
- `read_chunks_into_buffer_second_table`: Reads successive word-sized lengths from a second table, calls dos.Read into the buffer at a5 offset 12 plus 48000, advancing the pointer, and stops when a length is zero or a5 offset 36 is set.
- `read_chunk_blocks_until_zero`: Saves d4/a2-a3, points a2 past a fixed offset in the a5 struct, then reads successive word sizes from a table and calls dos.Read for each nonzero block, advancing a2, stopping when a size word is zero or a flag at offset 36 is set.
- `read_single_block_into_buffer`: Saves d2-d3, computes a destination buffer from the a5 struct plus a fixed offset, sets d3 to the block length, and calls dos.Read once with the file handle and length from the a5 struct.
- `flash_palette_sixteen_times`: Saves d2, loops sixteen times calling WaitTOF twice, filling a color table via a helper, and calling LoadRGB4, then clears the word at offset 32 of the a5 struct and returns.
- `decrement_packed_nibbles_in_place`: For d0 words at a0, subtracts one from each non-zero nibble of the three nibble fields and writes the packed result back, advancing a0.
- `fade_palette_loop_with_wait_tof`: Repeats sixteen times: waits for the display, calls the palette interpolation routine on a fixed table, and loads the result into the color registers via graphics library.
- `step_packed_nibbles_toward_target`: For d0 words, compares each nibble field of the word at a0 with the target word at a1 and increments differing fields, storing the updated word back.
- `blit_dual_playfield_scroll_loop`: Owns the blitter, waits for the display, then programs blitter pointers, mods and control registers to copy 16 rows of 6 blocks from the source buffer into the destination buffer, releasing the blitter at the end.
- `enable_dma_channel_from_table`: Uses d0 as an index into a table of DMA channel bit masks, loads the mask and, when the index is below 4, writes it with bit 15 set to DMACON to enable that DMA channel.
- `disable_all_dma_channels`: Loops d2 from 15 down to 0, calling the DMA enable routine for each channel index, then clears the longword at the table address before restoring d2 and returning.
- `start_audio_channel_sample_playback`: Selects a sample from the channel table, sets the DMA channel bit in DMACON, adjusts the period, calls the pitch routine, and stores playback pointers and lengths in the channel structure.
- `wait_for_copper_vertical_position_change`: Saves d0, polls the custom vertical position register until its value changes or a counter of 121 iterations expires, then restores d0 and returns.
- `commit_channel_buffer_pointer_if_odd`: Advances a1 past one longword, shifts d0 right by one, and if the shifted-out bit was set and the channel has a buffer, copies its pointer and length into the output structure.
- `enable_dma_and_call_three_times`: Reads a word from program memory, if nonzero sets bit 15 and writes it to DMACON to enable DMA, then calls a subroutine three times and branches to it.
- `decode_and_apply_map_object_record`: Saves registers, advances a record index, reads a three-byte object record from a table, decodes flags and type, fills fields in the object structure and dispatches on the low nibble.
- `misaligned_data_table_bytes`: These bytes are data decoded as code, containing meaningless immediate operands and bit operations, not executable instructions; the final branch is unreachable data.
- `select_sound_sample_by_index`: Tests d4; if nonzero divides d3 by 3, uses the remainder or high nibble to adjust d4 with a5 fields, then reads a word from the table at 66(a5) indexed by d4 into d5.
- `decrement_volume_by_d4`: Copies d4 to d1, loads the volume byte at 2(a5) into d0, masks d1 with 240; if zero subtracts d4 from d0, clamping to zero and storing back to 2(a5) and d1.
- `add_high_nibble_to_volume`: Shifts d1 right by four bits, adds it to d0, clamps d0 to 64 if greater, then stores the result to 2(a5) and d1.
- `set_d1_from_a5_offset_if_d3_one`: Compares d3 with 1 and, only when equal, loads the byte at offset 2 of a5 into d1 before branching to the shared continuation.
- `advance_pitch_and_emit_note_event`: Updates the a5 pitch accumulator from a table and multiplier, stores the resulting word and byte into the a1 event record, increments d7 and either loops back or calls the finish routine and restores registers.
- `zero_filled_data_area_4`: This single zero word decodes as an ori.b immediate instruction but is actually zero-filled data referenced by address, not executable code.
- `zero_filled_data_word`: Contains a single zero word that decodes as an ori.b immediate; this is zero-filled data referenced by address, not executable code.
- `write_byte_pair_from_table_or_divide`: For d0 below eleven, looks up a word in a table indexed by d0 minus one; otherwise divides a constant by d0, then writes the low and high bytes to two memory pointers.
- `data_word_ori_f0_d7`: A single word that decodes as ori.b with immediate f0 to d7; it is data referenced by address, not executable code.
- `load_pointer_table_and_first_record`: Stores the address of a nearby pointer table into a5 offset 66, indexes it with d1 to read a word into d5, then loads the first longword at a0 into d1 and two words at a3 into d2 and d3, returning a0 advanced by six.
- `add_offset_to_nonzero_longword`: Tests the longword at a0; if it is nonzero, adds d1 to it, then advances a0 by four and returns.
- `relocate_structure_pointer_lists`: Saves d2-d3/a2-a3, applies the add-offset routine to four longwords at a2 offsets 8 through 20, then loops over records adding the base pointer to each entry in their offset lists and finally fixes two more longwords at a2 offset 32.
- `apply_palette_and_copy_sprite_rows`: Scales sixteen palette bytes by a brightness factor into each palette entry, then copies sprite rows from a source into a 64-byte buffer or clears that buffer when no source exists.
- `load_mmd0_module_or_exit`: Calls the module loader, and if it returns null it reports error 33, loads a resource at offset four, and branches to the exit path; otherwise it calls the cleanup routine and returns minus one.
- `install_module_and_set_cia_fire_bit`: Saves a0, applies the palette routine, clears a flag and stored pointer, copies a byte to a global, sets or clears the CIAA port A bit 1 from a config bit, then queues a callback and sets the flag.
- `disable_audio_and_clear_flag`: Reads the audio-enabled byte, and if set clears bit 0 of the hardware register pointed to by the stored pointer, zeroes the word at absolute 1066 and calls the sound routine.
- `open_audio_resource_and_init_channels`: Opens the ciab resource, stores the base pointer, sets up three channel register pointers, opens the audio device and configures channel pointers, returning the channel count in d0.
- `close_audio_device_and_clear_flag`: If the audio-enabled byte is set, clears it, loads the stored audio device base into a6 and calls its close vector with the channel structure pointer.

### bcdft

- `free_mem_and_restore_stack_frame`: Saves all registers, calls a delay helper three times, calls an indirect routine, frees a memory block via exec.FreeMem, then restores registers and returns.
- `delay_loop_using_d6_count`: Copies d6 into d7, walks a pointer chain from a pc-relative longword, multiplies it by four, and busy-loops dbra d7 times before returning.

### bcdfu

- `blackcrypt_main_init_and_game_loop`: Opens dos, graphics and intuition libraries, saves the old interrupt vector, installs a new one, disables and enables DMA, then runs the game setup and main loop before restoring the vector and DMA.
- `iterate_high_bit_terminated_string`: Saves a2, walks a0 through bytes until one has its high bit set, calling the byte-pair routine for each preceding byte, then restores a2 and returns.
- `draw_string_chars_with_wait_tof`: Copies d0 and d1 to d3 and d4, loops over each nonzero byte of the string at a0, calls the glyph blit routine, increments d3, and calls graphics WaitTOF three times per character.
- `blit_glyph_column_to_bitmap`: Computes source offset from d1 times 40 plus d0 and destination from d2 minus 32 times 48, then copies eight bytes per row across six rows spaced 40 bytes apart into the bitmap at a5 offset 12.
- `init_playfield_and_reset_state`: Saves d3 and a2, stores a1 in a2 and d0 in d3, calls nine subroutines to set up graphics and sound, clears the word at offset 40 of a5, then restores registers and returns.
- `load_two_palette_entries`: Loads two PC-relative data blocks into a0, calls the loader for each, then passes values 2 and 5 to the delay routine before returning.
- `read_ciaa_fire_button`: Returns 1 in d0 if bit 6 of CIAA port A is clear (fire pressed), otherwise returns 0, reading the joystick fire line.
- `read_potgor_right_button_state`: Points a0 at the custom chip base, reads POTGOR bit 2 and returns 1 in d0 when the right/middle button is not pressed, otherwise 0.
- `clear_first_6000_longwords_at_a5_offset`: Loads the pointer from offset 12 of a5 into a0 and clears 6000 longwords starting there, advancing a0, then returns.
- `clear_500_longwords_after_32000_offset`: Loads the pointer from offset 12 of a5, adds 32000 bytes, clears 500 longwords from that address while advancing a0, then returns.
- `multiply_d0_by_a5_offset_2e_store_26`: Multiplies the low word of d0 by the signed word at offset 46 from a5 and stores the low result word at offset 38 from a5, then returns.
- `wait_until_a5_offset_26_is_zero`: Tests the word at offset 38 from a5 and branches back to itself while it is nonzero, so it spins until that word becomes zero, then returns.
- `wait_while_26_set_and_28_clear`: Returns if the word at offset 38 from a5 is zero; otherwise spins while the word at offset 40 from a5 is zero, then returns.
- `fade_four_palette_entries_over_105_frames`: Saves d2-d3/a2-a3, then loops 106 times disabling and enabling interrupts and waiting for the vertical blank while decrementing four channel counters and writing scaled values to four destination words.
- `wait_for_channel_status_byte_equal_15`: Loads a pointer from the table at the pc-relative address, offsets it by four, and spins until the byte there equals 15, then returns.
- `open_bcdfv_file_and_store_handle`: Calls dos.Open on the bcdfv filename with mode 1005, stores the returned handle at offset 28 of a5, and returns 1 if opening failed or 0 on success.
- `call_close_2`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `read_bb80_bytes_into_buffer`: Saves d2 and d3, calls dos.Read for 48000 bytes into the buffer at offset 12 of a5, then passes the buffer start and end to the decode routine.
- `read_second_chunk_into_buffer_offset`: Saves d2-d3, reads 26496 bytes from the file handle at offset 28(a5) into the buffer at 12(a5) plus 48000, then copies that data to a second buffer and returns.
- `read_bb80_bytes_at_offset_678c`: Saves d2-d3, computes buffer address from 12(a5) plus offset 48000, sets length 26508 and file handle from 28(a5), calls dos.Read via a6, then restores registers and returns.
- `read_third_chunk_into_second_buffer`: Saves d2-d3, reads 20583 bytes from the file handle at offset 28(a5) into the buffer at 12(a5) plus 96000, then copies it forward by another 48000 bytes and returns.
- `read_first_file_chunk`: Saves d2-d3, then calls dos.Read to read 2832 bytes from the file handle at 28(a5) into the buffer at 12(a5) plus an address, and restores d2-d3 before returning.
- `copy_96000_bytes_forward`: Copies 96000 bytes from the buffer at 12(a5) plus 48000 to the buffer at 12(a5) using longword moves with a dbra loop, then returns.
- `read_60000_bytes_after_base_into_buffer`: Pushes d2-d3, reads 60000 bytes from the dos file handle at offset 28(a5) into d3, then calls the copy routine with source at 12(a5) plus 48000 and destination 12000 bytes further.
- `read_2689_bytes_after_base_into_buffer`: Pushes d2-d3, reads 2689 bytes from the dos file handle at offset 28(a5) into d3, then calls the copy routine with source at 12(a5) plus 48000 and destination 12000 bytes further.
- `clear_96000_byte_block_at_offset`: Loads the base pointer from 12(a5), adds 96000, then clears 1365 longwords in a dbra loop, zeroing that block and returning.
- `rle_decompress_bytes_to_buffer`: Reads a run-length encoded byte stream from a0, writing expanded bytes to a1 until a zero control byte terminates; even control bytes repeat the next byte, odd ones copy literal bytes.
- `fade_palette_over_sixteen_frames`: Waits for the vertical blank twice per iteration, loads 32 color entries into the palette, waits again, and repeats 16 times before clearing the word at 32(a5).
- `decrement_packed_nibble_color_words`: Processes d0 words at a0, subtracting one from each nonzero nibble of the three color components and writing the packed result back, advancing a0 per word.
- `call_load_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls LoadRGB4 and returns its result to the caller.
- `fade_palette_step_down`: Saves d2 and a2, waits for two vertical blanks, converts 32 color entries from the source palette to the destination palette, writes them to the display, repeats 16 times, then sets the flag at offset 32 of a5 and returns.
- `loop_increment_nibbles`: The routine executes a countdown loop from fifteen to zero.
- `blend_palette_entries_toward_target`: For d0 entries, reads a word from a0 and a1, adjusts each of the three nibbles of the a0 word one step toward the matching nibble of the a1 word, writes the result back to a0 and advances both pointers.
- `adjust_packed_nibbles_toward_source`: For a count in d0, reads words from a0 and a1, compares each nibble group, and decrements mismatching groups in the a0 word before storing it back and advancing.
- `fill_32_words_and_init_display_buffers`: Writes the value 4095 into 32 consecutive words at a fixed table, then calls four subroutines to set up the screen buffer at offset 48000 and related structures.
- `poll_input_and_decrement_timer`: Saves registers, calls two input routines, ORs their results, and if nonzero sets a flag and decrements a countdown word before restoring registers and jumping indirectly.
- `set_second_buffer_offset`: Loads the long at offset 12 of the a5 structure, adds 96000 to it, and stores the result at offset 48 before branching onward.
- `blit_floor_tiles_loop`: Owns the blitter, sets up 16 rows of five blits using copper-style register writes to the custom chip base, waits on the blitter busy bit, then disowns the blitter and returns.
- `play_sound_list_until_zero`: Owns the blitter, then repeatedly calls the audio routine and the sound player for each word in a table until a zero word is found, then disowns the blitter and returns.
- `blit_wall_column_strip`: Sets up five chained blitter copies from a base buffer plus 72000 into a destination offset by the column index, waiting on blitter busy each pass.
- `copy_wall_texture_blocks`: Copies six blocks of 55 rows of eight longwords from source plus 5404 into the buffer at plus 72000, skipping eight bytes per row.
- `blit_floor_or_ceiling_strip`: Computes source and destination offsets from the signed strip index, programs blitter control and pointers, then waits for the blitter to finish.
- `blit_wait_loop_until_flag_or_timeout`: Saves d2/a2, calls DisownBlitter, clears the word at offset 40 of a5, then repeatedly calls WaitTOF up to 199 times while polling that word, and calls OwnBlitter before returning.
- `enable_dma_channel_from_table_2`: Uses d0 as an index into a table near pc to fetch a value into a1, and if the low six bits of d0 are below 4, writes the shifted bit mask to DMACON to enable that DMA channel.
- `disable_all_dma_channels_and_clear_flag`: Saves d2, loops d2 from 15 down to 0 calling the DMA enable routine for each channel, then clears the longword at the address loaded near pc and restores d2.
- `queue_sound_channel_from_table`: Pops saved registers, looks up a sound entry by index in the table at offset 24 of a6, sets the DMACON bit for channel d0, updates channel state bytes in a5 and returns.
- `start_sound_with_pitch_and_volume`: Adjusts note value d1, calls the sound setup routine, stores pitch, volume and sample pointer into the channel structure at 20(a5) and configures playback fields.
- `store_volume_if_nonzero`: Tests word d2 and, when it is not zero, writes it to offset 4 of the channel structure pointed to by a1 before continuing.
- `store_sprite_offset_and_dimensions`: Writes d3 to offset 4 of the structure at a1, computes d0 plus twice d2 into a5 offset 24, stores d3 at a5 offset 28 and d5 at a1 offset 6 and a5 offset 18, then pops d3-d7 and returns.
- `wait_vertical_blank_change`: Saves d0, polls the vertical position register until its value changes, retries up to 122 times, then restores d0 and returns.
- `flush_pending_blit_if_odd_flag`: Loads a5 from the pointer at a1, shifts d0 right by one, and if the shifted-out bit was set and a5 offset 24 is nonzero, copies that value and the word at a5 offset 28 into the buffer pointed to by a5 offset 20.
- `disable_dma_and_play_sound_sequence`: Reads a word from program data, if nonzero sets bit 15 and writes it to DMACON to enable DMA channels, then calls a delay routine and plays a sequence of sound commands via a data pointer.
- `advance_sound_channel_counter`: Saves registers, loads a global pointer, clears a word in memory, reads a byte counter, increments it, compares against a limit at offset 769 of a structure and stores it back if below.
- `decode_and_start_sound_sample`: Clears a byte, indexes a table by a global word, reads sample header bytes into a structure, decodes flag bits, looks up volume and pitch tables and calls a playback routine.
- `store_d5_byte_if_d6_is_f2`: Compares d6 with the byte value 242; if equal, writes the low byte of d5 to the address in a5, clears d5 to zero, then branches away to another routine.
- `clear_flag_and_call_on_d6_fe`: Compares d6 with the byte value 254; if equal, clears the word at absolute address 7096, loads a pointer from pc-relative memory into a0, clears bit 0 at that address, calls a subroutine, pops four bytes from the stack and branches away.
- `select_word_from_table_on_d6_fd`: Compares d6 with the byte value 253; if equal and d7 is below 4, decrements d5, indexes a word table pointed to by a5 offset 66, stores that word at offset 6 of the structure pointed to by a5 offset 20, clears d5 and branches away.
- `check_d6_ff_then_emit_d7`: Compares the low byte of d6 with 255; if not equal it branches away, otherwise it copies d7 into d0, calls the routine at 1284 and branches to 1876.
- `increment_long_counter_at_163b`: Adds one to the byte stored at long address 163b, then branches to 1876 without touching any registers.
- `check_d0_e_store_d6_in_a5_2f`: Compares the low byte of d0 with 14; if equal it stores the low byte of d6 at offset 47 from a5, then branches to 1876.
- `scale_nibble_pair_to_volume_byte`: If d0 equals 12, converts d6's two nibbles into a decimal value, clamps it to 64, scales by the word at offset 16 of a5, shifts right by 8 and stores the byte at offset 2 of a5.
- `set_channel_9_pan_or_mute_byte`: If d0 equals 9, masks d6 with 31, substitutes 32 when the result is zero, and writes that byte to offset 769 of a4.
- `split_d6_nibbles_into_a5_offsets`: If d0 equals 8, copies d6 to d0, shifts d6 right by 4, masks d0 with 15, then stores the shifted d6 at offset 9 of a5 and the masked d0 at offset 8 of a5.
- `set_sound_slot_from_d6`: Compares d0 with 11; if equal, masks d6 to a byte, checks it against the limit at offset 506 of a4, stores it at 7094 and sets the byte flag at 5691.
- `decrement_d5_check_d6_zero`: Compares d0 with 3; if equal, decrements d5 and branches on its sign, then tests d6 and branches if zero, otherwise jumps to the shared exit.
- `advance_animation_frame_and_draw`: Checks frame counter d7 against four, adds animation offsets to d5, looks up a word in the table at 66(a5) into 30(a5), then calls the draw routine and advances the frame index.
- `advance_sound_channel_volume_envelope`: Walks the sound channel list, decrements each channel's envelope counter, updates its volume byte and DMACON channel enable bits, then dispatches through a jump table based on the channel's step value.
- `increment_frame_and_restore_registers`: Adds one to d7, compares it with the word at 1636(pc), branches back if lower, otherwise calls the routine at 13d4, pops d2-d7 and a2-a5, and returns.
- `zero_word_data_area`: A single zero word that decodes as ori.b with zero immediate; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_area_5`: Contains a single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `scale_value_by_index_to_two_bytes`: For d0 below eleven, looks up a word from a table indexed by d0 minus one; otherwise divides a constant by d0, then writes the low and high bytes of the result to two memory locations pointed to by absolute addresses.
- `constant_word_data_referenced_by_address`: Contains a single word that decodes as ori.b #240,d7; it is a constant data value referenced by address rather than executable code.
- `load_sound_table_and_first_entry`: Stores the address of a table at offset 66 of a5, reads a word indexed by d1 into d5, then loads the first long from a0 into d1 and two words from a3 into d2 and d3 before returning.
- `add_d1_to_nonzero_long_at_a0`: Tests the long at a0; if nonzero, adds d1 to it, then advances a0 by four bytes and returns.
- `relocate_structure_pointer_lists_2`: Saves d2-d3/a2-a3, applies the add-d1 fixup to several pointer fields of the structure at a2, then relocates each entry of two pointer arrays by adding their base address, restoring registers and returning.
- `apply_volume_and_copy_sample_data`: Scales sixteen per-instrument volume bytes by a factor from the instrument structure and stores each result into the word at offset 16 of sixteen pointed-to structures, then copies sample bytes into a buffer or clears it.
- `load_mmd0_module_from_offset_four`: Calls a check routine, and if it returns zero loads resource 33, then passes the address four bytes into the module to a setup routine and branches to the shared tail.
- `load_mmd0_module_from_high_address`: Calls a check routine, and if it returns zero loads resource 33, then passes a fixed high memory address to a setup routine and branches to the shared tail.
- `check_mmd0_then_load_module`: Calls a test routine, and if it returns zero loads resource 33, then points a0 at the module buffer and calls the loader before branching onward.
- `stop_and_restart_sound_playback`: Checks the sound-active flag, stops playback, then clears the active bit, stores the module pointer, sets the CIAA port A bit from the module flags, and restarts playback.
- `clear_sound_active_flag_and_stop`: If the sound-active flag is set, clears bit 0 of the pointer at the sound state address, zeroes the word flag, and calls the stop routine.
- `open_ciab_resource_and_init_ports`: Opens the ciab.resource via exec, stores the base pointer, programs three CIA-B port registers and calls the resource open vector, returning zero on success or one on failure.
- `close_ciab_resource_if_open`: Reads the stored open flag, clears it, and if it was set calls the close vector of the saved ciab.resource base with the parameter block, returning the flag value.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_runtime_entry_point`: Unconditionally jumps to the runtime entry point at the fixed address, transferring control without saving registers or returning.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_shared_runtime_entry_one`: Performs an unconditional jump to a shared runtime entry point, transferring execution without saving registers or returning.
- `jump_to_shared_runtime_entry_two`: Performs an unconditional jump to a second shared runtime entry point, transferring execution without saving registers or returning.
- `jump_to_shared_runtime_entry_three`: Performs an unconditional jump to a third shared runtime entry point, transferring execution without saving registers or returning.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_vector_error_one`: Performs an indirect jump to a fixed address.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_alternate_runtime_entry`: Performs an unconditional jump to a different shared runtime entry point, transferring execution without saving registers or returning.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_30`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_31`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_32`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_33`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_34`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_35`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_36`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_37`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_38`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_41`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_42`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_43`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_44`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_45`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_46`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_47`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_48`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_49`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_50`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_51`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_52`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_53`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_error_handler`: Performs an unconditional jump to a fixed address that handles the pushing of five error string pointers and subsequent execution.
- `undecodable_bytes_54`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_55`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_56`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_57`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_58`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_59`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_60`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_61`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_62`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_63`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_64`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_65`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_66`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_38`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_39`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_40`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_41`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_42`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_43`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_46`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_47`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_48`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_49`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_50`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_51`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_52`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_53`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_54`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_55`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_56`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_57`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_58`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_59`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_60`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_61`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_62`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_63`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_64`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_65`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_66`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_67`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_68`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_69`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_70`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_71`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_72`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_73`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_74`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_75`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_76`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_77`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_78`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_79`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_80`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_81`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_82`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_83`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_84`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_85`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_86`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_87`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_88`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_89`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_90`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_91`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_92`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_93`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_94`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_95`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_96`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_97`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_98`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_99`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_100`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_101`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_102`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_103`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_104`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_105`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_106`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_107`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_108`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_109`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_110`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_111`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_112`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_113`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_114`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_115`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_116`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_117`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_118`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_119`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_120`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_121`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_122`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_123`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_124`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_125`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_126`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_127`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_128`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_129`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_130`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `load_ed_into_d0_and_branch`: Loads the immediate value 237 into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `load_two_into_d0_and_branch`: Loads the immediate value 2 into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `load_f0_into_d0_and_branch`: Loads the immediate value 240 into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `load_f1_and_branch_to_common`: Loads the immediate value 241 into d0 and branches unconditionally to the shared routine at the later address.
- `load_f5_and_branch_to_common`: Loads the immediate value 245 into d0 and branches unconditionally to the shared routine at the later address.
- `load_f6_and_branch_to_common`: Loads the immediate value 246 into d0 and branches unconditionally to the shared routine at the later address.
- `load_66_into_d0_and_branch`: Loads the immediate value 66 into data register d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `load_65_into_d0_and_branch`: Loads the immediate value 65 into data register d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `load_negative_six_into_d0_and_branch`: Loads the sign-extended immediate value minus six into data register d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `load_minus_three_and_branch`: Loads the value minus three into d0 and branches unconditionally to a shared routine elsewhere in the segment.
- `load_ninety_three_and_branch`: Loads the value ninety-three into d0 and branches unconditionally to a shared routine elsewhere in the segment.
- `load_minus_one_and_branch`: Loads the value minus one into d0 and branches unconditionally to a shared routine elsewhere in the segment.
- `nop_then_branch_to_4cea`: Executes a no-operation instruction, then branches unconditionally to another location; it does not return to its caller.
- `nop_branch_then_increment_a6_offset`: Executes a no-operation, branches unconditionally away, and the trailing instructions increment a word at offset 32 of a6, move a6 to d0 and return.
- `decrement_counter_and_dispatch_via_a5`: Decrements the word counter at offset 32 of a6, then saves registers, computes a stack-based pointer, indexes a table by d0, and calls through a5 with selector 12.
- `jump_vector_entry_131`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_132`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_133`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_134`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `push_five_error_strings_and_jump`: Jumps through a fixed vector, then for five separate entries pushes a longword string pointer after reserving eight bytes on the stack and branches to a common continuation.
- `push_string_c24918_and_branch`: Reserves eight bytes on the stack, pushes the longword at the given absolute address as an argument, and branches to the shared continuation.
- `push_string_c24912_and_branch`: Reserves eight bytes on the stack, pushes the longword at the given absolute address, then branches to the shared continuation.
- `pop_a5_and_return`: Pops a5 from the stack and returns to the caller, restoring the saved frame pointer.
- `jump_vector_entry_135`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_back_to_earlier_code`: Unconditionally branches backward to an earlier instruction, transferring execution without touching registers or memory.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
