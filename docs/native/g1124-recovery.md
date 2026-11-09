# Game 1124 (lords of the realm (1995)(impressions)[cr pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01124). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703360 top-level bytes in 119 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 407 of 407 named (0 author symbols), 407 of 407 described (16 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 208 of 208 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 8946 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12150 of 12152 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1577407 |
| code | 41266 |
| data | 737695 |
| padding | 27504 |
| unused | 319488 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:HDInstall | 3 | 5224 | 141 |
| icon.library | 10 | 1375 | 109 |
| Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:paradox | 4 | 542 | 13 |
| Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:lords | 9 | 251 | 6 |
| Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:message | 2 | 1472 | 44 |
| Lords of the Realm (1995)(Impressions)[cr PDX](Disk 2 of 3).adf:game.bm5 | 2 | 3270 | 93 |
| Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |
| Lords of the Realm (1995)(Impressions)[cr PDX](Disk 2 of 3).adf:casnames.eng (raw code file, load address unknown) | 1 | 0 | 0 |
| Lords of the Realm (1995)(Impressions)[cr PDX](Disk 2 of 3).adf:casnames.fre (raw code file, load address unknown) | 1 | 0 | 0 |
| Lords of the Realm (1995)(Impressions)[cr PDX](Disk 2 of 3).adf:casnames.ger (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- lords of the realm (1995)(impressions)[cr pdx](disk 1 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:mod.lords: high-entropy data file (7.75 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 1 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:messages.eng: high-entropy data file (7.72 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 1 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:mod.title: high-entropy data file (7.79 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 1 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:conquest.map: high-entropy data file (7.70 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 1 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:textcast.eng: high-entropy data file (7.75 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 3 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 3 of 3).adf:cast.bm5: high-entropy data file (7.63 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 3 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 3 of 3).adf:thewalls.am5: high-entropy data file (7.67 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 3 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 3 of 3).adf:port.bm5: high-entropy data file (7.64 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 3 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 3 of 3).adf:back3a.am5: high-entropy data file (7.68 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 3 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 3 of 3).adf:retr.bm5: high-entropy data file (7.66 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 3 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 3 of 3).adf:back1.am5: high-entropy data file (7.65 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 3 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 3 of 3).adf:sieg.bm5: high-entropy data file (7.65 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 3 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 3 of 3).adf:back3.am5: high-entropy data file (7.64 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)[cr pdx](disk 2 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 2 of 3).adf:casnames.eng: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lords of the realm (1995)(impressions)[cr pdx](disk 2 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 2 of 3).adf:casnames.fre: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lords of the realm (1995)(impressions)[cr pdx](disk 2 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 2 of 3).adf:casnames.ger: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lords of the realm (1995)(impressions)[cr pdx](disk 2 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- lords of the realm (1995)(impressions)[cr pdx](disk 3 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- lords of the realm (1995)(impressions)[cr pdx](disk 1 of 3)/Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:lords: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 8946 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `paradox_entry_init_and_restore_system` (paradox): Saves all registers, clears two memory blocks, reads DMACONR and CIAA, installs interrupt vectors, enables DMA and interrupts, then restores the saved vectors and register set before returning zero. Kind: loader/protection initialization. Pass path: the loop continues until cRam01008123 is set, then proceeds to u2__sub_0_03EC and subsequent system restoration.

## Functions


### Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:HDInstall

- `jump_to_hdinstall_main`: Jumps via a program-counter-relative displacement to the main HDInstall code, transferring execution at program entry.
- `harddrive_installation_menu`: Runs the hard drive installer: creates directories, draws the disk and path selection screen, copies files from DF0: and shows progress and completion messages.
- `copy_d0_into_d5`: Copies the longword in d0 into d5, preserving the value for the caller that continues into the following code.
- `negate_d6_load_word_from_pc`: Negates the low word of d6, then loads the word at the program-counter-relative offset into d0 and falls through to the next routine.
- `copy_counted_string_to_buffer`: Saves registers, reads a length byte from the source string in a3, copies that many following bytes into the destination buffer in a2, appends a null terminator and restores registers.
- `draw_indexed_sprite_with_waits`: Saves d2/a6, stores a value in a global, computes a vertical position from a table entry, draws a sprite, then waits on two polling loops before restoring registers and returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `open_screen_window_and_init_display`: Opens intuition screen and window, allocates two 40000-byte bitplanes plus a 12-byte structure, initializes rastports, viewport, coplist and display dimensions, then sets up game display globals.
- `close_display_and_free_resources`: Closes the window and screen, frees bitplanes, rastport and coplist resources, restores the saved display value, and calls the teardown helper to release the display setup.
- `load_two_32_color_palettes`: Calls graphics LoadRGB4 twice with 32 entries, loading the palette pointed to by the stack argument into the screen's viewport color table and the second rastport.
- `set_two_palette_entries_via_setrgb4`: Splits two packed colour words into nibbles and calls graphics SetRGB4 twice, once on the palette at offset 44 and once on the table at the second global, then restores registers.
- `zero_32_word_buffer_then_fill`: Clears 32 words of a local stack buffer to zero, then passes that buffer and the caller's argument to the fill routine and returns.
- `read_32_palette_colours_and_fill`: Loops 32 times calling graphics GetRGB4 on the screen's colour table, storing each result in one local array and zeroing a second, then passes both arrays to the fill routine.
- `sample_32_palette_entries`: Calls GetRGB4 for each of 32 color indices, stores the returned words into a local buffer, then passes the buffer and the caller's argument to the fade routine.
- `fade_palette_toward_target`: Copies 32 words into a local buffer, repeatedly nudges each nibble toward the target palette, waits for the vertical blank, redraws, and delays between passes.
- `set_beam_position_offset`: If the argument is zero, reads two random values and the current VBeamPos to build a position; stores the resulting long in the global beam position variable.
- `seed_lfsr_from_timer`: Saves d2-d4/a6, loads a fixed high bit mask into d3, reads the low word of the timer at address 848 into d2, sets d4 to 31, and branches into the shift loop based on bit 0 of d2.
- `lfsr_shift_and_store_seed`: Shifts d2 right, conditionally XORs d3 when bit 3 is set, decrements d4, loops back while non-negative, then stores d2 to address 848, returns it in d0 and calls a routine with d1 from the stack.
- `toggle_screen_view_on_vblank`: Waits for a VBeam position between 244 and 255, then swaps two screen view pointers and related display structures depending on a flag, and calls a routine with the word at address 852.
- `read_console_input_event`: Reads a console device event via the library, stores cursor position globals, sets two flag bytes from event bits, closes the device and returns a mapped key code or negative error.
- `queue_console_write_request`: Fills a console write request block with the two word arguments from the stack, links it into the console structure and sends the write request through the library.
- `call_clear_pointer`: Library stub: loads its arguments and the intuition library base, calls ClearPointer and returns its result to the caller.
- `set_console_pointer_sprite`: Calls intuition SetPointer with the console window and pointer data, passing zero offsets and a size of sixteen by one, then restores registers and returns.
- `select_install_source_pointers`: Saves d2 and a6, reads a word argument from the stack into d2, stores it in a global word, then copies one of two sets of three longword pointers into three destination globals depending on whether the argument is zero.
- `poll_install_state_until_ready`: Saves d2 and a6, calls the routine at pc-relative offset 14f6, tests the returned word in d0 and branches back to the caller's loop while it is zero.
- `delay_then_poll_install_state`: Loads one into d1, calls the dos Delay library vector through a6, then calls the routine at pc-relative offset 14f6 and branches back while the returned d0 word is zero.
- `retry_call_until_nonzero_result`: Calls the routine at pc-relative offset, stores its word result in d2, and repeats the call while the result is zero; returns the nonzero result in d0 after popping d2 and a6.
- `clear_rastport_then_fill_rect`: Loads the rastport pointer, calls graphics SetAPen with pen zero, then calls RectFill using the four screen-edge words as min/max coordinates.
- `call_blt_clear`: Library stub: loads its arguments and the graphics library base, calls BltClear and returns its result to the caller.
- `blit_bitmap_swap_source_dest`: Chooses source and destination bitmaps based on the stack argument, then calls graphics BltBitMap with fixed dimensions and zero offsets to copy between them.
- `clamp_window_bounds_from_stack_args`: Saves d2-d5 and a2-a4/a6, loads four word arguments from the stack into d2-d5, points a2 at a global table, and clamps d2 and d4 against the table's first word before restoring registers and returning.
- `clamp_d4_against_table_second_word`: Compares d4 with the second word at a2, raises d4 to that value if lower, decrements d4, then compares d4 with d2 and branches back to the earlier clamping code if still greater.
- `clamp_d3_against_table_third_word`: Compares d3 with the third word at a2, raises d3 to that value if lower, then compares d3 with d5 and branches back to the earlier clamping code if d5 is greater.
- `clamp_d5_between_d3_and_a2_limit`: Compares d5 with the word at offset 6 of a2, lowers d5 to that value minus one if greater, then compares d5 with d3 and branches away if d5 is greater.
- `select_bitmap_pointers_by_flag`: Checks the word at offset 44 of the stack; if it equals one, loads a3 and a4 from two absolute longword pointers, otherwise branches to the alternate path.
- `blit_rect_from_bitmap_pointers`: Loads source and destination bitmaps from absolute pointers, computes width and height from stack words, sets blit registers and calls BltBitMap through the graphics library vector.
- `clip_rect_and_fill_with_apen`: Clamps four word arguments to the screen bounds at the global rectangle limits, then calls graphics SetAPen and RectFill on the bitmap in a2 to draw the clipped rectangle.
- `check_left_edge_against_screen_min`: Saves registers, loads two word arguments into d4 and d5, compares d4 with the global left limit and returns early if it is below it, otherwise restores registers and returns.
- `compare_right_edge_to_screen_max`: Compares d4 with the global right limit and branches back to the earlier clipping code if d4 is less, otherwise falls through to the next check.
- `compare_d5_against_2fa_threshold`: Compares the word in d5 with the longword at absolute address 2fa, branching away if d5 is greater or equal, otherwise jumping to another routine.
- `compare_d5_against_2fe_threshold`: Compares the word in d5 with the longword at absolute address 2fe, branching away if d5 is less, otherwise jumping to another routine.
- `set_bitplane_pixels_from_d4_d5`: Computes a byte offset from d4 and d5 plus a base pointer, builds a bit mask from d4, and ORs it into up to five bitplane bytes 8000 apart selected by bits of the word at 554.
- `set_graphics_cursor_position`: Saves d2-d3 and a6, stores the two word arguments into the cursor x and y globals, then calls the graphics library Move with those coordinates via a6.
- `set_pen_draw_and_cursor_position`: Saves registers, sets the graphics pen from a global via SetAPen, draws using the two word arguments via Draw, then stores those arguments into the cursor x and y globals.
- `move_cursor_then_draw_at_position`: Pushes the two word arguments twice onto the stack, calls the cursor-position routine then the pen-and-draw routine, cleans the stack and restores a6 before returning.
- `draw_rect_edges_between_four_points`: Saves d2-d5, reads four word arguments from the stack, then calls the line routine once and the rectangle routine four times to connect the points, finally restoring registers.
- `store_rect_bounds_and_incremented_size`: Copies two word arguments into the global rectangle at 760, then stores the next two arguments plus one into the following two words and returns.
- `build_bitmap_from_compressed_tile_data`: Allocates a bitmap via graphics library, computes row and plane sizes, decodes packed bit data into the bitmap planes and stores the resulting pointers in the structure.
- `release_bitmap_and_clear_handle`: Pops a2/a6, loads the object pointer from the stack, and if its bitmap handle at offset 22 is set, calls the graphics library release function with the handle and size, then clears the handle and optionally frees a nested object.
- `draw_character_glyph_clipped`: Clips a character cell against the screen bounds, computes the glyph bitmap address from the font, sets up the blit rectangle and calls BltBitMap twice to draw the glyph, returning the glyph width.
- `draw_text_string_with_newlines`: Walks a null-terminated string, drawing each character via the glyph routine and advancing the x position, while carriage return and newline reset x and advance y by the font metrics.
- `format_signed_decimal_into_buffer`: Converts the signed long in d3 to decimal digits into a local buffer, prepends a minus sign when negative, then calls the text output routine with the buffer and the two word arguments from the stack frame.
- `measure_text_and_output_centered`: Measures the string at the local buffer with a zero width limit, subtracts the result from d6, and passes the difference and buffer to the text output routine before restoring registers and returning.
- `measure_text_width_with_font_metrics`: Walks the string in a2 up to d5 characters, accumulating glyph widths from the font tables pointed to by the global font structure, resetting at carriage return and newline, and returns the widest line width in d0.
- `draw_text_box_with_border`: Allocates a graphics buffer, saves and clears the text cursor and colour globals, draws a bordered rectangle via the line and fill routines, then reads keyboard input to edit text inside it.
- `dispatch_install_menu_choice`: Stores the first argument in a global, reads four bytes from the input stream, then branches on the byte value to call the install, abort or quit handlers with the two remaining arguments.
- `decode_rle_install_data`: Reads a run-length encoded byte stream, expanding escape codes into zero, 255 or repeated bytes and writing them into the caller's buffer until the decoded length is exhausted.
- `decode_rle_stream_to_buffer`: Reads a length-prefixed run-length encoded byte stream via the byte reader, expands literal and repeated runs into the destination buffer a2, stopping when the output count d5 or input count d4 is exhausted, then restores registers and returns.
- `xor_adjacent_block_pairs`: For each consecutive pair of blocks in the buffer at a4, XORs the bytes of the first block into the second block using offsets from the block lookup routine, iterating over the block count and block size globals.
- `read_next_byte_from_stream`: Returns the next byte from the current stream pointer stored in the global stream structure, advancing the pointer, or calls the refill routine when the pointer reaches the end, returning the byte in d0.
- `examine_file_get_size`: Locks the named file, examines it, unlocks it, and returns the file size from the examine block, or zero if the lock failed.
- `scan_directory_entries_into_array`: Locks a directory, allocates an entry array, iterates entries with ExNext, filters by name and type flags, copies matching names into fixed-size records and returns success.
- `free_entry_array_buffer`: If the structure holds an allocated buffer, frees it via the memory library using the count times thirty-four byte size, then clears the pointer.
- `save_regs_and_check_list_count`: Pushes d2-d4, a2, a3 and a6 onto the stack, loads the caller's argument pointer from offset 28 into a3, and tests the longword at offset 4 of that structure, branching away if it is zero or if the word at offset 2 is at most one.
- `zero_d4_d2_and_branch`: Sets d4 and d2 to zero and branches to another routine, preparing two index counters before the comparison loop.
- `compare_adjacent_entries_and_swap`: Compares the sort key at offset 32 of entry d2 with the next entry in the 34-byte record array at a2, and if the first is smaller calls the swap routine with the entry pointer.
- `bubble_sort_records_by_field`: Compares the word at offset 32 of adjacent 34-byte records in a2, swaps them via two helper calls when out of order, and advances d2/d3 until the record count in a3 is exhausted.
- `rotate_three_record_buffers`: Copies 34 bytes from a2 into a stack buffer, then a2 into a2+34, then the buffer into a2+34, using three block-copy calls to rotate three adjacent records.
- `wildcard_string_match`: Compares string a2 against pattern a3, treating asterisk as a wildcard that skips characters, returning 1 on match or 0 on mismatch after checking both strings are non-empty.
- `save_d0_to_d5`: Copies the longword in d0 into d5, preserving the value for later use by the caller.
- `examine_and_append_directory_path`: Locks and examines a directory, walks parent directories, builds a path string, and appends entries to a buffer until the parent chain ends, then unlocks the lock.
- `save_d0_word_to_d5`: Copies the low word of d0 into d5, preserving that value for the caller.
- `push_d0_onto_stack`: Pushes the longword in d0 onto the stack by predecrementing a7, then falls through into the following routine.
- `lock_and_unlock_current_dir`: Locks the directory path passed on the stack using dos.Lock, unlocks it with dos.UnLock, and returns 1 if the lock succeeded or 0 otherwise.
- `lock_df0_drive_with_number`: Builds a DF0: device name with a digit from the stack argument, locks it via dos.Lock, unlocks it, and returns 1 on success or 0 on failure.
- `return_zero_and_branch_back`: Loads zero into d0 and branches backward to another routine, providing a zero return value for its caller.
- `set_mode_and_read_field`: Saves a2 and a6, stores the value 14 at offset 28 of the structure in a2, calls a library function through a6, then returns the longword at offset 32 of that structure in d0.
- `open_trackdisk_and_poll_disk_events`: Allocates two structures, opens the trackdisk device, then repeatedly calls the status routine to poll disk events, delays, closes the device and frees both structures.
- `compare_two_paths_via_temp_locks`: Resolves two path strings through helper calls, locks each with a temporary lock, compares the resulting lock values and returns 1 if equal or 0 if different, restoring saved registers.
- `lock_install_dir_and_store_current_dir`: Locks the install directory path using dos.Lock with mode -2, then calls dos.CurrentDir on that lock and stores the returned lock in the global current-dir slot.
- `unlock_stored_current_dir_lock`: Takes the lock stored in the global current-dir slot, calls dos.CurrentDir on it, then releases that lock with dos.UnLock and restores a2 and a6.
- `read_bytes_into_buffer_via_stream`: Pops saved registers, reads up to a count of bytes from a stream structure into a destination buffer, calling a byte-read routine when the buffer is exhausted, and returns the number of rows completed.
- `write_buffer_bytes_to_stream`: Pops saved registers, writes a count of bytes from a source buffer into a stream structure, calling a byte-write routine when the buffer is full, and returns the number of rows completed.
- `store_word_to_global_057a`: Loads the word argument from the stack into d0 and stores it to the global longword at offset 057a, then returns.
- `lcg_random_15bit`: Reads the 32-bit seed at memory location 1402, multiplies it by a constant via a called routine, adds 12345, stores the new seed back and returns bits 16-30 masked to 15 bits in d0.
- `load_args_and_tail_call_copy`: Loads the first two stack arguments into a0 and a1, copies a0 into d0, loads the third stack argument into d1 and branches into the byte copy loop.
- `byte_copy_dbra_outer_loop`: Copies bytes from (a1) to (a0) incrementing both pointers, repeating d1+1 times via dbra, then subtracts 65536 from d1 and repeats while the result stays non-negative.
- `compare_null_terminated_strings`: Compares two byte strings at the stack arguments, advancing both pointers until a differing byte or a zero byte, and returns zero if equal, one if the first is greater, or minus one if less.
- `compare_strings_with_length_limit`: Compares two byte strings at the stack arguments up to a given length, stopping at a differing byte or zero, and returns zero if equal, one if the first is greater, or minus one if less.
- `find_next_token_in_string`: Scans the string at a0 or the stored global pointer for a delimiter, null-terminates the token, advances the global pointer past it, and returns the token start or zero.
- `find_char_in_string`: Scans the first string for a byte matching the current byte of the second string, returning a pointer to the match or zero when the first string ends.
- `compare_strings_and_init_library`: Compares two strings byte by byte, then clears a memory block, saves the stack pointer, opens dos.library and calls Supervisor, alerting on failure.
- `load_a4_with_7ffe`: Loads the immediate value 32766 into address register a4 and returns immediately to the caller.
- `open_manx_file_and_init_stream`: Allocates a file handle, writes the MANX signature, opens the named file, fills the handle fields with current directory and input/output handles, and calls the tokenizer.
- `copy_d0_to_d5`: Copies the longword in d0 into d5 and falls through into the following routine; it does not return on its own.
- `tokenize_manx_line_into_argv`: Copies a line into a buffer, splits it into whitespace-separated tokens honoring double quotes, allocates a pointer array, and stores each token address followed by a null terminator.
- `move_d0_to_d0_noop`: Copies register d0 onto itself, an instruction that changes nothing, then falls through into the following routine.
- `bounded_string_copy_with_count`: Scans a0 for the terminating zero, then copies bytes from a1 to a0 until a zero byte or the 16-bit count in d1 expires, writing a final zero and returning the original a0.
- `open_icon_library_and_load_window_image`: Opens icon.library, gets the window object, obtains its image, allocates two bitmaps, stores them and the image count in the caller's structure, then closes the library and returns.
- `copy_d0_to_d5_2`: Copies the longword in d0 into d5 and falls through into the following routine.
- `read_byte_from_stream_buffer`: Validates the stream structure in a2, refills its buffer via helper calls when exhausted, then returns the next byte in d0 or minus one on error.
- `open_stream_with_default_mode`: Pushes minus one as a mode argument, calls the open routine, then passes the returned handle and two stack arguments to the stream setup routine.
- `parse_amiga_memory_flag_string`: Parses a flag string starting with r, w or a plus optional b and +, builds a mode word in d2 and d3, calls the memory allocator with d6, and stores the result index and flags into the output structure at a3.
- `find_free_entry_in_table`: Scans a table of 22-byte entries starting at a fixed base, returns the address of the first entry whose word at offset 12 is zero after clearing its fields, or returns zero if none is free.
- `append_string_to_a0`: Finds the terminating null of the string at a0, then copies the string at a1 including its null terminator to that position, and returns the original a0 pointer in d0.
- `bounded_memcpy_or_zero_fill`: Copies bytes from the source pointer to the destination until a zero byte or the 32-bit count expires, then zero-fills the remainder; returns the original destination pointer in d0.
- `open_file_with_mode_301`: Pushes the caller's word argument and mode 769 onto the stack, calls the file-open routine with the path pointer, cleans the stack and returns.
- `open_or_create_dos_file_slot`: Finds a free six-byte slot in the file table, opens or creates the named file via dos.library with the given mode flags, stores the handle and flags, and returns the slot index or minus one.
- `read_entry_by_index_with_error_code`: Validates a word index against a table bound, computes the entry address, checks its flags, and calls the dos Read vector on the entry's file handle, storing an error code and returning minus one on failure.
- `multiply_two_32bit_words`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products and returns the low 32 bits of the product in d0, preserving d2 and d3.
- `allocate_and_init_file_handle`: Gets a free file handle from the dos library, allocates a 34-byte structure, fills its fields with the passed name and mode, opens the file and returns the structure pointer or zero.
- `close_io_handle_and_free_request`: Saves a2/a6, loads the request pointer from the stack, and if its field at offset 10 is nonzero calls the library close routine through a6; then marks byte 8 as 255, sets long at 20 to -1, reads byte 15 into d0, calls the library free routine, and calls the reply routine with 34 before restoring registers.
- `allocate_and_init_io_request`: Saves d2/a2-a3/a6, loads the argument pointer and word from the stack, returns zero if the pointer is null, otherwise calls the library allocate routine with size 65537 and d0 from the word, then stores 5 at offset 8, the word at 18 and the pointer at 14, returning the new block.
- `abort_io_request_and_reply`: Saves a2/a6, loads the request pointer from the stack, returns immediately if it is null, otherwise sets byte 8 to 255, writes -1 to offsets 20 and 24, loads the word at 18 into d0, and calls the library reply routine through a6 before restoring registers.
- `signed_multiply_with_sign_fixup`: Pushes d1 and d4, negates d0 and d1 when negative while tracking sign in d4, calls the multiply helper, then negates the result if the combined sign flag is set and restores registers.
- `signed_divide_with_sign_fixup`: Pushes d1 and d4, negates d0 and d1 when negative while tracking sign in d4, calls the divide helper, moves the quotient from d1 to d0 and branches to the shared sign-fixup tail.
- `unsigned_divide_entry_wrapper`: Saves d1 on the stack, calls the division helper, copies the quotient from d1 into d0, restores d1, tests d0 and returns.
- `swap_d1_test_high_word_divide_32bit`: Swaps d1 halves, tests the high word; if zero it performs two unsigned 16-bit divisions on d0 by d1 and returns quotient in d0 and remainder in d1.
- `restoring_division_loop_16_iterations`: Swaps d1, saves d2 and d3, then runs a 16-step shift-subtract division of d0 by d1, leaving quotient in d0 and remainder in d1 before restoring registers.
- `validate_and_clear_record_structure`: Checks a record pointer and its status word, calls subroutines to process flags and a numeric field, formats a decimal string, clears the record fields and returns success or failure in d0.
- `advance_a5_and_d0_by_two_and_eight`: Adds 2 to the word in a5 and adds 8 to the low byte of d0, then falls through to the following code.
- `init_stream_handlers_over_table`: Saves a2, walks a table from its start to end in steps of 22 bytes, calling the setup routine for each entry, then restores a2 and returns.
- `buffered_stream_write_byte`: Writes one byte to the stream described by a2, flushing and refilling its buffer via helper calls, and returns the byte or minus one on error.
- `scan_entries_call_until_minus_one`: Walks a table of 22-byte entries from a base pointer, skipping entries whose word at offset 12 is zero or has bits 8 or 9 set, calling a routine on each remaining entry until it returns minus one, then returns minus one or zero.
- `init_stream_buffer_and_flags`: Sets up a stream structure: tests a byte at offset 14, stores a 1024-byte buffer pointer at offset 8 or falls back to an inline byte, sets flags at offset 12, and copies the pointer to offsets 0 and 4.
- `unlink_node_and_free_memory`: Searches a singly linked list for the node whose data starts 8 bytes in, unlinks it from the list or head pointer, then calls the memory free function with the node pointer plus 8.
- `push_regs_and_load_list_head`: Saves a2, a3 and a6 on the stack, loads the global list head at address 908 into a2, then branches to the list walker.
- `free_list_nodes_via_library`: Walks the linked list from a2, calling the library function at offset -210 on each node, then clears the global list head and restores a2, a3 and a6.
- `allocate_and_prepend_list_node`: If the argument is null returns zero; otherwise allocates a node via library offset -198, stores the old list head and argument in it, prepends it to the global list and returns the node data pointer.
- `copy_string_to_destination`: Loads destination and source pointers from the stack, copies bytes from source to destination until a null byte, and returns the destination pointer in d0.
- `measure_null_terminated_string_length`: Loads a string pointer from the stack, scans bytes until a null terminator, and returns the length excluding the terminator in d0.
- `check_index_and_call_dos_interactive`: Validates a word index against bounds and a table entry, then calls the dos library IsInteractive function through a6, returning one or zero, or minus one on invalid index.
- `seek_file_handle_by_index`: Validates a file index against the table, seeks its handle to the given offset, and returns the new position or minus one with an error code stored.
- `delete_file_by_name`: Calls the DOS delete function on the name pointer, and on failure stores the IoErr result and returns minus one, otherwise returns zero.
- `write_to_file_handle_by_index`: Validates a file index, checks its open mode bits, optionally seeks to the start, then writes the buffer and returns the byte count or minus one.
- `run_exit_handlers_chain`: Walks the linked list at the exit-handler global, calling each node's second longword as code and following the first longword to the next node, then calls the single handler pointer and tail-calls itself.
- `shutdown_install_and_free_resources`: Saves d2-d3/a6, calls a per-index routine for each entry, then closes the dos library, invokes optional callbacks, frees four allocated blocks, releases the library base and returns the saved argument in d0.
- `restore_registers_and_return`: Pops d2, d3 and a6 from the stack and returns immediately to the caller, restoring the registers saved by the matching entry code.
- `jump_to_close_file_entry`: Jumps directly to the close-file routine at the following address, acting as a one-instruction trampoline for the handler pointer.
- `close_file_by_index`: Looks up a file entry by index in the table, closes its dos handle via the library, clears the entry and returns zero, or returns minus one on invalid index.
- `init_list_header_pointers`: Takes a pointer argument, stores it as the list head, advances it by four, clears the next field and points the tail back to the head.

### icon.library

- `return_minus_one_library_stub`: Loads minus one into d0 and returns immediately, giving the library entry point a constant failure result.
- `increment_open_count_clear_flag`: Adds one to the word at offset 32 of the library base in a6, clears bit 3 of the byte at offset 14, and returns the base in d0.
- `decrement_open_count_maybe_close`: Sets d0 to zero, decrements the word at offset 32 of the a6 library base, and if it reaches zero with bit 3 of offset 14 set, calls the close routine before returning.
- `close_icon_library_and_free_base`: Checks the library open count at offset 32 of the base; if zero, unlinks the base from its list, calls the expunge vector at offset 34 with the base size, and returns the expunge result.
- `mark_library_base_in_use`: Sets bit 3 of the flags byte at offset 14 of the library base and returns zero in d0.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_icon_library_and_init_base`: Stores the caller's a0 and a6 into the new base, opens dos.library by name, saves the result at offset 38, and calls exec.Alert with an error code if the open fails.
- `copy_a2_offsets_to_library_globals`: Copies the longwords at offsets 34 and 38 from a2 into the library globals at 402 and 406, returns a2 in d0, then pops a2 and returns.
- `zero_filled_data_word_at_0192`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable library code.
- `zero_filled_data_area_2`: This single zero word decodes as an ori.b instruction but is a zero-filled data area referenced by address, not executable code.
- `icon_library_call_via_a0_save`: Pushes a0 and a6 onto the stack, calls the library routine at the absolute address zero, then adds eight to the stack pointer and returns.
- `icon_library_call_via_a0_a1_save`: Pushes a0 and a1 plus a6 onto the stack, calls the library routine at absolute address a8, then restores the stack pointer by adding twelve and returns.
- `icon_library_call_via_a0_a2_save`: Pushes a0 through a2 plus a6 onto the stack, calls the library routine at absolute address 124, then restores the stack pointer by adding sixteen and returns.
- `icon_library_call_with_a0_a1_saved`: Saves a0 and a1 plus a6 on the stack, calls a library routine, then discards the saved registers and returns to the caller.
- `icon_library_call_with_a0_saved`: Pushes a0 and a6 onto the stack, calls a library routine, adds eight to the stack pointer to drop both saved registers, and returns.
- `icon_library_call_second_entry`: Pushes a0 and a6, calls a different library routine, adjusts the stack by eight to remove the saved registers, and returns to the caller.
- `icon_library_call_a6_stub`: Pushes a6 onto the stack, calls the library routine at offset 1704, then discards the saved a6 and returns to the caller.
- `icon_library_save_a0_a2_call_stub`: Saves a0 through a2 and a6 on the stack, calls the library routine at offset 1820, then restores the stack pointer past all saved registers and returns.
- `icon_library_save_a0_call_stub`: Pushes a0 and a6 onto the stack, calls the library routine at offset 1896, then adds eight to the stack pointer to discard both saved registers and returns.
- `icon_lib_push_a0_a6_call_helper`: Pushes a0 then a6 onto the stack, calls a library helper, discards the eight pushed bytes and returns to the caller.
- `icon_lib_push_a0_a1_call_helper`: Pushes a0 and a1 onto the stack, calls a library helper, removes the eight bytes from the stack and returns.
- `icon_lib_push_a0_a1_call_other_helper`: Saves a0 and a1 on the stack, calls a different library helper, cleans up eight stack bytes and returns to the caller.
- `icon_library_call_jsr_8d6`: Saves a0 and a1 on the stack, calls the library routine at address 8d6, then discards the saved registers and returns.
- `icon_library_set_field_94`: Calls the library function at offset -294 of the a6 base, stores the second argument into offset 148 of the returned pointer, and returns.
- `zero_filled_data_area`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `allocate_and_init_icon_object`: Calls the library allocator for the given size, then fills the new object from a caller-supplied structure; returns the object pointer in d0 or zero on failure.
- `copy_icon_params_into_object`: Copies eleven longs from the stack frame into the icon object at a2, stores extra fields, links the object into its parent at offset 424, and returns a2 in d0.
- `return_zero_restore_regs`: Sets d0 to zero as the return value, restores d2 through d4 and a2 from the stack frame, releases a6 and returns to the caller.
- `build_icon_header_and_dispatch`: Copies eleven longwords from the icon structure at offset 96 into a local header, fills in fields from offsets 61, 72, 76, 84, 88, 92, 156 and 160, then calls the shared dispatch routine with the two arguments and the header.
- `open_icon_file_and_read_header`: Builds a path from the argument, opens it, reads the icon header, checks the magic word and version, then loads the icon's image and tool data through the library's file routines.
- `load_icon_images_and_tools`: Allocates and reads the icon's image and tool structures, validates the magic word and version, clears the two longwords at the caller's pointer, and loads the image and tool data through the library's file routines.
- `load_icon_header_and_optional_images`: Calls library helpers to fetch two icon header values into a4 offsets 18 and 22, then conditionally allocates and fills image buffers at a2 offsets 50, 54 and 70, returning success or failure in d0.
- `fill_icon_image_array_from_reader`: Loops over a count at a6 offset -4, calling the read callback and allocating each image into the array pointed to by a4, then optionally loads a final image into a2 offset 70 and returns success in d4.
- `set_d4_zero_and_branch_to_exit`: Sets d4 to zero as a failure return value and branches to the shared exit path, returning immediately without touching other registers or memory.
- `return_d4_restore_regs_unlk`: Moves d4 into d0 as the return value, restores d2-d5 and a2-a5 from the stack frame, releases the a6 frame and returns to the caller.
- `icon_open_allocate_and_load`: Saves registers, opens the icon library base, allocates a work buffer, then loads icon data from the passed structure into the buffer and returns a status in d4.
- `icon_load_data_into_buffer`: Saves the icon data pointer, opens the library, allocates a buffer, then copies icon fields and name strings from the a2 structure into that buffer, returning status in d4.
- `draw_icon_two_images_and_optional_list`: Calls the image draw routine for the two image pointers at offsets 22 and 26 of a2, then draws the string at offset 50 and the image list at offset 54, returning d4 as status.
- `advance_image_list_and_draw_remaining`: Steps a3 through the null-terminated image pointer list at offset 54 of a2, counting entries in the frame slot, then calls the draw callback and restarts the list.
- `draw_image_list_then_final_image`: Loops over the null-terminated image list at a3 drawing each entry, then draws the image at offset 70 of a2, calls cleanup, stores d6 at offset 26 and returns d4.
- `push_a2_and_advance_to_next_node`: Pushes a2 onto the stack, loads the caller's first argument from offset 12 of the stack into a0, sets a2 to two bytes past that address, then branches to another routine.
- `walk_a2_list_call_until_match`: Pushes d0 and a2, calls two library routines, loops while a2's next pointer differs from a2, and calls the first routine again when d0 is nonzero before restoring a2.
- `call_node_routine_with_offset_argument`: Loads the first stack argument into d0 and the second into a0, pushes the address 140 bytes into a0 and d0, calls a routine, cleans the stack and returns.
- `allocate_and_init_icon_node`: Saves d2-d3/a2, allocates a 65536-byte block via library call, then initializes fields at offsets 140 and 142 of the new block using the caller's argument, freeing the block and returning zero on failure.
- `retry_icon_node_init_with_extra_arg`: Pushes the block pointer, its offset-140 field and the saved argument, calls the init routine again, and returns the block pointer in d0 if it succeeds, otherwise falls through to the zero-return path.
- `return_zero_and_restore_registers`: Sets d0 to zero as the return value, restores d2-d3 and a2 from the stack, and returns to the caller.
- `decrement_count_and_append_entry`: Pops d2 and a2, reads a count word at a2 and decrements it; if it reaches zero calls the grow routine, then stores two arguments into the entry array at offset 16 and 20 indexed by the count, returning 1 or 0.
- `store_entry_after_count_decrement`: Decrements the word count at a2, loads the array pointer from 10(a2), scales the count by 8 and writes d2 and the stack argument into the entry slots at offsets 16 and 20, returning 1.
- `allocate_and_register_icon_node`: Allocates a 94-byte block, initializes its list node, links it into the icon list, appends the entry via the count routine, registers it with the caller's data and returns the block pointer or zero on failure.
- `branch_to_icon_library_offset`: Branches unconditionally to another point in the icon library, transferring execution without touching registers or memory.
- `push_pea_call_jsr_return_zero`: Pushes a small immediate and d4 onto the stack, calls a library routine, clears d4, restores the stack, returns zero in d0 and restores d2-d4/a2.
- `call_with_second_arg_plus_78`: Saves d2, loads two stack arguments, adds 78 to the second, pushes both and calls a library routine, then restores d2 and returns.
- `icon_lookup_first_entry`: Saves d2-d4/a2-a3, loads a2 and d2 from stack arguments, calls a library routine with d2 pushed, stores the result in d3 and a2 in d4, then branches based on the call result.
- `icon_scan_for_separator`: Saves d3, d2 and a3, calls a library routine, restores the stack, and if the call succeeded advances a3 by d3 and checks for a separator byte before returning a3 in d0.
- `icon_next_entry_or_finish`: Loads the next pointer from (a2) into a3 and d0, branches to the scan routine if nonzero, otherwise clears d0, restores d2-d4/a2-a3 and returns.
- `icon_lookup_call_with_two_args`: Saves d2-d5 on the stack, loads two arguments from the caller's stack into d2 and d3, pushes d3, calls a library routine, stores the result in d5 and branches onward.
- `icon_open_with_mode_7c`: Pushes the constant 124 and d2, calls a library routine, saves the result in d4, cleans the stack and returns the difference between d4 and d2 or branches if zero.
- `icon_compare_and_verify_handle`: Calls a library routine with d2, compares the result to d5, then calls another routine with d5, d3 and d2, returning 1 on success or 0 otherwise.
- `copy_d4_to_d2_if_nonzero`: Tests d4; if zero falls through, otherwise copies d4 into d2 and increments d2 by one, then branches onward.
- `zero_d2_and_return_zero_d0`: Clears d2, tests it (always zero so no branch), sets d0 to zero, restores d2-d5 from the stack and returns.
- `parse_icon_copy_name_into_buffer`: Saves registers, clears byte 30 of the first argument, calls library helpers to match the 'copy ' prefix and parse a number, then stores the parsed count and copies the remaining name text.
- `icon_parse_skip_five_bytes_and_check`: Advances a4 by five, pushes three arguments and calls the library routine at offset 72, then tests d0 and adjusts the stack by twelve bytes.
- `icon_parse_set_zero_and_branch`: Sets d3 to zero and branches forward to the shared exit path, returning a zero status for the icon parser.
- `icon_parse_decimal_number`: Accumulates a decimal number from bytes at a4 using d3 times ten plus digit, calls conversion and validation routines, and returns the result in d0.
- `ascii_digit_to_value`: Reads the longword argument from the stack, checks it lies between the characters zero and nine, and returns its numeric value or minus one otherwise.
- `return_minus_one`: Loads minus one into d0 and returns immediately, serving as the failure result for the digit conversion routine.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `open_icon_with_fallback`: Saves d2-d5, calls the open routine with two arguments, and on success calls the draw routine; if drawing fails it closes the icon and returns zero.
- `alloc_icon_node_and_init_size`: Saves d2/a2, allocates 96 bytes with tag 65536, stores 10 in the new node's offset 14 and in the caller's word, then links the node into the caller's list and returns 1 on success.
- `return_zero_restore_d2_a2`: Sets d0 to zero as the return value, restores d2 and a2 from the stack and returns to the caller.
- `compare_three_args_call_and_flag`: Pushes the three stack arguments, calls the routine at offset 48, compares the result with the third argument, cleans the stack and returns 1 if equal or 0 otherwise.
- `return_d1_in_d0`: Copies the longword in d1 into d0 and returns to the caller, acting as a trivial value-returning library stub.
- `compare_three_args_via_library_call`: Pushes the three longword arguments from the stack, calls the library routine at offset 76, moves the result to d1, compares it with the saved argument, cleans the stack and returns zero or the result.
- `return_d1_in_d0_alt_entry`: Copies the longword in d1 into d0 and returns immediately, a second trivial library entry point reached from a data pointer.
- `signed_32bit_divide_by_d2`: Compares d2 with 65535; if larger, performs a 32-bit restoring division of d1 by d2 over 16 steps, otherwise divides the two 16-bit halves of d1 by the low word of d2, returning quotient in d0 and remainder in d1.
- `unsigned_32bit_divide_by_d2`: Shifts the 32-bit value in d1 left through 16 iterations, subtracting d2 whenever the high part exceeds it, leaving the quotient in d1 and remainder in d0.
- `multiply_d0_by_d1_32bit`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, accumulating the high cross terms and low product into a 32-bit result returned in d0, preserving d2.
- `format_icon_string_via_rawdofmt`: Saves a2-a4/a6, loads format string and argument pointer from the stack, calls exec RawDoFmt through a6, then restores registers and returns.
- `store_byte_to_a3_and_advance`: Writes the low byte of d0 to the memory pointed to by a3 and increments a3 by one, then returns.
- `strlen_up_to_255_bytes`: Scans the string at a0 for a zero byte, counting at most 255 characters, and returns the length in d0.
- `find_char_in_string_or_return_pointer`: Loads a0 from the first stack argument and d0 from the second, reads a byte from (a0)+ into d1, and if nonzero compares it with d0; on match it backs a0 up by one and returns that pointer in d0, otherwise it returns.
- `return_zero_and_branch_to_shared_exit`: Sets d0 to zero with a quick move and branches forward into the shared exit path of the surrounding library routine.
- `copy_string_until_nul_byte`: Loads a0 and a1 from the stack arguments, then copies bytes from (a1)+ to (a0)+ until a zero byte is copied, and returns.
- `skip_nul_terminated_string`: Loads a0 and a1 from the caller's stack arguments, then advances a0 past the first non-zero byte of the string it points to, leaving a0 one past that byte.
- `compare_strings_up_to_length`: Loads a0 and a1 from the stack and a length from the stack into d0, then compares bytes of the two strings until a mismatch, a zero byte or the length runs out, returning zero on full match.
- `return_compare_result_flag`: Sets d0 to one when the preceding comparison was greater, otherwise sets d0 to minus one, then returns to the caller.
- `scan_strings_until_count_exhausted`: Restores a0 and a1 from the stack, loads a count from the stack into d0, then advances a0 past bytes until a zero byte or the count runs out, branching to the shared exit.
- `decrement_pointer_and_fill_byte`: Subtracts one from a0 and branches to the byte-fill routine, then returns to its caller.
- `fill_bytes_with_value_until_count`: Decrements d0, copies bytes from a1 to a0 while nonzero, and pads with zero bytes until the count is exhausted, then branches to the exit.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `init_icon_header_self_pointers`: Loads the caller's first stack argument into a0, stores a0 at (a0), adds four to that value, clears the longword at offset four, and stores a0 at offset eight.
- `draw_icon_with_offsets`: Saves d2-d3/a2, loads three stack arguments, calls a library routine with a size of twenty, then computes scaled offsets from the icon structure and calls two more routines before restoring registers.
- `load_icon_args_and_branch`: Saves d2-d3/a2 on the stack, loads the three stack arguments into d2, d3 and a2, then branches forward into another routine's body.
- `walk_icon_list_validate_entries`: Walks a linked list of icon nodes in a2, calling a validation routine on each node and its data, returning 1 if all pass or 0 on failure.
- `return_zero_restore_registers`: Sets d0 to zero as the return value, restores d2, d3 and a2 from the stack and returns to the caller.
- `allocate_and_build_icon_from_spec`: Allocates a 20-byte icon structure, creates its bitmap and mask from the spec fields, stores the mask pointer and returns the icon or minus one.
- `return_minus_one_pop_registers`: Sets d0 to minus one as the return value, restores d2 through d5 and a2 from the stack, then returns to the caller.
- `save_regs_load_three_args_zero_d6`: Pushes d2 through d6 and a2 onto the stack, loads three longword arguments from the caller frame into d2, d3 and d4, clears d6, then branches onward.
- `push_args_call_and_test_d6`: Pushes a longword immediate, a low-memory pointer, d3 and d2, calls a library routine, stores the result in the caller frame, then tests d6 and branches.
- `icon_alloc_two_buffers_and_chain`: Calls library allocation routines to create two buffers for the icon structure, stores the second pointer at offset 8, and walks the linked list until a null next pointer, returning -1 on any allocation failure.
- `return_minus_one_restore_registers`: Loads -1 into d0 as the failure return value, restores d2 through d6 and a2 from the stack, and returns to the caller.

### Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:paradox

- `paradox_entry_init_and_restore_system`: Saves all registers, clears two memory blocks, reads DMACONR and CIAA, installs interrupt vectors, enables DMA and interrupts, then restores the saved vectors and register set before returning zero.
- `build_copper_sky_and_ground_palette`: Writes copper list instructions to display memory for sky and ground gradients, then fills palette entries by interpolating colour values through two helper routines.
- `interpolate_rgb_colour_between_two_values`: Blends the red, green and blue bytes of two packed colours by factor d6, then splits the result into two nibble-packed words via the nibble helper.
- `pack_nibbles_into_byte_word`: Combines the low nibble and high nibble of d0 into one byte, shifts the upper word into place and returns the packed value in d0.
- `interpolate_nibbles_by_delta`: Computes a nibble-wise interpolated word from d0 and d1 scaled by d2, packing the low, middle and high nibble results into d4 and returning it.
- `dispatch_sound_command_by_counter`: Saves a5-a6, pushes a return address, loads a word counter and branches to one of three handlers depending on whether it is zero, one or more.
- `advance_sound_sample_pointer`: Restores a5-a6 and returns, or advances a sample pointer by a table offset, wrapping the index at 96 and calling the sound update routine.
- `clear_and_build_playfield_bitplanes`: Clears 128 blocks of 176 bytes at the playfield base, then reads a level map byte stream and writes character glyph bytes into bitplane rows, updating the countdown and cursor globals.
- `vertical_blank_copper_and_keyboard_isr`: On vertical blank interrupt, saves all registers, programs the copper list and color register, reads the keyboard serial register, calls two subroutines and acknowledges the interrupt before returning.
- `cia_interrupt_acknowledge_handler`: On CIA interrupt, saves registers, reads the CIA interrupt control register, pulses the control register and waits on the vertical position register before acknowledging the interrupt and returning.
- `compute_and_store_span_offsets`: Reads two word indices from the a5 structure, sums table entries into 32(a5), calls a helper, then writes 255 span deltas into the a1 buffer and advances the index bytes.
- `toggle_bitplane_pixels_over_43_rows`: Uses four index words and a coordinate table to XOR single bits in two bitplanes of the a4 bitmap across 43 rows of 8 pixels, then advances the index bytes.
- `swap_pointers_and_set_blitter_registers`: Swaps the two long pointers at 12 and 16 of a5, computes an end address from 32(a5), and writes start/end coordinates into three hardware register blocks.

### Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:lords

- `relocate_hunk_and_free_mem`: Saves all registers, calls a relocation helper repeatedly, frees a memory block via exec FreeMem, then patches a relocated hunk pointer table and clears the entry before restoring registers and returning.
- `compute_relocated_hunk_pointer`: Copies d6 to d7, loads the longword just before the current pc into a1, scales it by four, loops dbra over d7, then advances a1 by four and returns the computed pointer.
- `apply_relocation_offsets_to_hunk`: Calls a relocation helper to fetch hunk data, walks relocation entries adding offsets into the target hunk memory, and calls exec CacheClearU when the OS version is high enough.
- `load_pc_relative_table_and_count`: Loads the address of a nearby data table into a1 and sets d0 to 2, then falls through into the following routine.
- `decompress_backward_bitstream_to_buffer`: Decodes a backward-read bitstream from a0 into a3 using a4-held state, copying bytes and expanding matches until the output reaches a2 or the length in d5 runs out.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:message

- `message_program_open_dos_and_run`: Program entry that clears a global block, opens dos.library, allocates memory, reads the message file and calls the message processing routines before returning a status in d0.
- `release_message_resources_and_reply`: Pushes the incoming message pointer, optionally unlocks a DOS lock, calls a cleanup routine, frees memory, closes a library, replies to the message and restores registers before returning.
- `check_stack_against_limit_then_call`: Computes the current stack pointer minus four, compares it with the saved stack limit at offset 536 of a4, and if below it calls a routine with a4 pushed.
- `save_d7_zero_it_and_branch`: Pushes d7 onto the stack, sets d7 to zero, then branches forward into another routine without returning here.
- `run_message_callback_list`: Walks the pointer list at offset 116 of a4, calling each function pointer until a null entry, then sets the byte at offset 572 of a4 and returns the last result in d0.
- `push_global_and_call_vector`: Pushes the address of a global longword onto the stack, calls the function pointer stored at absolute address 48, then returns 20 in d0 after discarding the pushed argument.
- `push_global_and_call_vector_no_result`: Pushes the address of a global longword onto the stack, calls the function pointer stored at absolute address 48, discards the pushed argument and returns without setting a result.
- `strlen_scan_and_count_chars`: Walks the null-terminated string at the argument, calling a routine for each character, and returns the character count in d0 while preserving d7 and a5.
- `save_regs_load_a5_and_jump`: Pushes a3, a5 and a6 onto the stack, loads a5 from offset 576 of a4, then branches to another routine without returning here.
- `free_message_list_nodes`: Walks the linked list at a5, freeing each node with exec.FreeMem, then clears the pointer at offset 576 of a4 and returns zero in d0.
- `copy_string_a1_to_a0_return_end`: Copies bytes from the string at a1 to a0 including the terminating zero, then returns in d0 the address just past the last copied byte.
- `init_message_output_buffers_and_open`: Sets up message buffer pointers and counters in the a4 data area, calls dos.Open on the configured name, and stores the resulting file handle for output.
- `build_output_path_and_allocate_buffer`: Measures the message path string, appends the directory name when needed, adds the output name length, then calls the allocator and returns its pointer or one on failure.
- `open_message_window_and_allocate_buffers`: Builds window title strings, opens a DOS file and a window, allocates a buffer via exec, stores handles at offsets 588 and 604 of a4, and sets up three display entries.
- `open_message_window_with_fallback`: Opens two DOS files and a window, falling back to NIL: on failure, stores handles at offsets 588, 604 and 620 of a4, then initializes three display entries.
- `copy_d0_to_d5_and_load_return_address`: Moves d0 into d5 and loads the caller's return address from the stack into a0, then falls through into the following code.
- `init_message_node_or_fail`: Saves a5, uses a0 as node pointer, tests bit 3 of byte 27; if clear, calls a routine with the value at offset 228 of a4, stores the result at offsets 4 and 16, and on zero sets offset 564 of a4 to 12 and returns -1; otherwise clears offsets 8, 12 and 24 bits and returns 0.
- `set_message_flag_and_jump`: Pushes d6, d7, a5 and a6, sets d0 to 1, stores it at offset 96 of a4, loads a5 from offset 628 of a4 and branches to another routine.
- `walk_message_list_and_dispatch`: Loops over a linked list via a5, and when flags at offset 24 equal 2 and offsets 4 and 16 differ, calls a routine with swapped d0/d1 and a0 from offset 16; then walks the list at offset 236 of a4 and calls handlers based on flags.
- `check_message_flags_and_call_handler`: Loads the longword at offset 4 of a5 into d6, skips if it is zero or has bit 4 set, otherwise moves d7 into d0, sign-extends it and calls the function pointer at offset 278 of a4.
- `close_message_file_and_advance_list`: If bit 2 of d6 is set, closes the file handle at offset 8 of a5 via dos.Close, increments d7, follows the a5 chain, and returns d0 from the next node or pops registers and returns.
- `write_message_text_to_file`: Opens and validates a message record, optionally writes a prefix and the text at a5 with dos.Write, handles newline splitting and errors, stores the IoErr result and returns minus one.
- `copy_three_args_to_second_slot`: Tests bit 6 of d0, then copies the three longword arguments at offsets 4, 8 and 12 from the stack, writing the third over the first argument slot.
- `open_intuition_and_seek_message_file`: Opens intuition.library, calls Seek and IoErr through the dos library base, stores status values in globals and sets an error code when the seek fails.
- `push_d0_call_sub_and_clean_stack`: Pushes d0 onto the stack, calls the routine at offset 450, then adds 4 to the stack pointer to discard the pushed value and returns.
- `tokenize_message_into_argv`: Scans the input string counting whitespace-separated or quoted tokens, allocates a pointer array with AllocMem, then fills it with token pointers while converting backslash escapes and null-terminating each token.
- `free_token_pointer_array`: Checks the token count and pointer array in the message globals, and if both are nonzero calls FreeMem to release the array sized by count plus one.
- `break_message_and_write_output`: Reads a byte from the message buffer, opens intuition.library, writes a break line via dos.Write, and stores a callback pointer in the message state table.
- `find_nth_message_node`: Clears the message error flag, then walks the linked list at offset 236 for the given index in d0, returning the node pointer or null and setting error codes 9 or 5.
- `poll_signals_and_dispatch_message`: Calls exec SetSignal for signal mask 12288, checks the message callback pointer at offset 96, and either invokes it with argument 4 or clears it and jumps to the message handler.
- `scan_buffer_for_byte`: Scans the buffer at a0 for the byte in d0 over d1 bytes, returning a pointer to the match or null when the count is exhausted.
- `return_a0_in_d0_if_nonzero`: Moves the pointer in a0 into d0, branches past the return when the move sets a nonzero condition, otherwise returns immediately.
- `decrement_d0_save_regs_check_flag`: Subtracts four from d0, saves d2-d5, a2, a3 and a6 on the stack, loads a long from offset 576 of a4 into d1, and if that value is zero stores 22 at offset 564 of a4 before restoring registers and returning.
- `range_check_against_a1_limits`: Copies d1 into a1, compares d0 with d1 and branches when d0 is not greater, otherwise adds the long at offset 4 of a1 plus 20 to d2 and compares d0 with d2, branching on the result.
- `load_first_long_and_branch`: Loads the longword at offset zero of a1 into d1, then branches away if it is zero, otherwise branches to another location.
- `scan_message_list_for_entry`: Walks a linked list of message records starting at d1 plus sixteen bytes, comparing each against d0 and following the next-offset word until a match or the end is reached.
- `unlink_message_and_update_neighbors`: Clears the high bit of the message header at d0, recomputes the size fields of the following and preceding records, adds the size to a1's counter and unlinks the record from the list at offset 576 of a4.
- `unlink_message_node_by_id`: Walks the linked list at the pointer in a0 until a node whose first longword equals d1, replaces that node's link with the longword at a1, then frees the old node via exec FreeMem and restores registers.
- `allocate_message_buffer_block`: Rounds d0 up to a multiple of four, searches the message block list for a free slot of that size, splits it and marks it used, or allocates new memory with exec AllocMem and links it in; returns pointer to the block payload.
- `run_message_callback_list_2`: Saves d7 and a5, walks the callback list at offset 388 of a4 calling each node's function pointer, then passes d7 to another routine and returns.
- `message_stream_put_char`: Writes the character in d0 into the message stream buffer at offset 4 of the structure in a5, advancing the write pointer and count, flushing on newline and handling buffer-full and error flags.
- `message_stream_flush_or_reset`: Resets or flushes the message stream in a5, or when a0 is null walks the list at offset 628 of a4 and flushes each stream, returning nonzero if any flush failed.
- `empty_message_stub_return`: Empty routine that returns immediately without touching any registers or memory.
- `call_jsr_260_with_saved_a0_d0`: Pushes a0 and d0 onto the stack, calls the subroutine at address 260, then discards the eight saved bytes and returns.

### Lords of the Realm (1995)(Impressions)[cr PDX](Disk 2 of 3).adf:game.bm5

- `main_open_message_file_and_play`: Program entry that opens the message file, allocates memory, copies text, sets up a copper list and interrupt server, plays the message, then frees resources and returns zero.
- `message_screen_copper_and_input_setup`: Initializes copper list pointers and bitplane registers, clears sprite colors, copies a screen buffer, sets up the display window, and polls fire button input to page through message screens.
- `find_longest_run_store_offset`: Scans the byte string at a5 counting repeated-byte runs, computes an offset from d0 and d1, stores the best run's offset in a global word and returns it in d4.
- `build_tile_grid_and_blit_rows`: Reads tile indices from the map data at a0, computes tile addresses in a3, and fills 16 rows of 20 blitter records in a6 with source, destination and control words, calling the blit routine for each row.
- `fill_pattern_table`: Performs nested loops to combine specific words from a source table and store the result into a destination table.
- `setup_five_slot_bcd_counter_display`: Adds packed BCD bytes into a counter, clears a global, then fills five copper display slots with sprite pointers and control words before returning.
- `setup_four_slot_bcd_counter_display`: Adds packed BCD bytes into a counter, clears a global, then fills four copper display slots with sprite pointers and control words before returning.
- `setup_single_slot_sprite_display`: Computes a sprite table entry from a global index, calls the sprite setup routine, then writes one copper display slot with pointers and control words.
- `queue_three_tile_blits_from_script`: Reads three byte indices from a script table, computes source offsets into tile data and destination offsets into a bitmap, and fills three blitter queue entries at offsets 64 to 102 of the a6 structure.
- `transform_pseudo_random_seed`: Takes a value from a pseudo-random generator, performs a modulo operation and bitwise concatenation, and stores the resulting bit-shifted value into a global variable.
- `queue_map_tile_blit_from_coordinates`: Writes the map x and y coordinates into the tilemap registers, computes a source tile address from the y and x values, and fills one blitter queue entry at offsets 64 to 88 of the a6 structure.
- `setup_terrain_blit_descriptor`: Reads map tile coordinates from the base pointer, computes source and destination addresses, and fills the a6 blitter descriptor fields with fixed values before returning.
- `draw_three_row_tile_strip`: Advances a scroll counter, computes a tile address, then writes three blitter descriptors to a6 to draw three rows of tiles at increasing offsets.
- `or_five_rows_into_filled_words`: Loops sixteen times, ORs five words spaced by d0 from a0, and writes the combined word five times to a1 before returning.
- `scale_font_rows_by_15`: Writes a busy flag, then for 16 rows reads 32 words from a font table, scales each of the three nibble channels by 15 with rounding, and stores the packed result to the destination buffer at offset 384 of a6.
- `scale_font_rows_by_inverse_15`: Writes a busy flag, then for 16 rows reads 32 words from a font table, scales each nibble channel by 16 minus the row counter with rounding, and stores the packed result to the destination buffer at offset 384 of a6.
- `blit_text_glyphs_to_bitmap`: Computes a bitmap destination from d0 and d1, waits on the blitter busy bit, programs blitter registers, then loops over characters in a0 converting each to a glyph address and issuing blits until a negative byte ends the string.
- `toggle_custom_register_and_call_helpers`: Writes 1 to a fixed custom register, polls bit 6 of a6+2 and calls one helper, stores 255 into a6+52, polls bit 2 of a6+22 and calls another helper, then clears the register and returns.
- `xorshift_prng_next_byte`: Advances a 32-bit seed in memory with repeated conditional XOR and doubling steps, stores the new seed, then rotates it and returns the low byte in d0.
- `read_joystick_and_fire_into_byte`: Reads JOY1DAT direction bits and CIAA fire button, builds a direction/button bitmask in d2, stores it to a fixed memory byte and returns.
- `init_audio_channel_volume_table`: Stores the passed pointer in a global, scans 128 bytes for the highest value, builds 31 channel volume entries at offsets from the base, then clears the four Amiga audio volume registers and several state bytes.
- `silence_audio_and_enable_dma`: Writes zero to the four Amiga audio volume registers and sets DMACON to enable the audio DMA channels, then returns.
- `advance_audio_tick_and_dispatch`: Increments the audio tick byte, compares it with the limit byte and wraps it to zero, then tests a flag and branches to the tick handler or the main dispatch.
- `call_f1f6_then_jump_f4de`: Calls the routine at the next address with bsr, then branches unconditionally to another routine, never returning to this caller.
- `init_four_sound_channels`: Tests a global flag and, if clear, loads channel numbers and pointers for four sound structures and calls the setup routine for each, the last by branch.
- `compute_channel_note_and_setup`: Reads a note index from the object, looks up a byte in a table, shifts it, adds a base value, clears a word, and if a global flag is set configures a sound structure with the result.
- `load_four_sprite_slots_and_chain`: Loads four sprite descriptors at offsets a0, b0, c0 and d0 into a6 by calling the loader with d5 and a4 set, then branches to the next stage.
- `decode_sprite_record_from_stream`: Reads a sprite record from the stream at a0 plus d1, fills a4 fields from tables at a3 and a1, and writes the sprite index into a6 plus d5.
- `apply_sprite_motion_and_flags`: Adds d3 to the a4 position fields, stores the frame from a3 plus d4, then tests the sprite type bits and calls the motion routine for types three, five and nine.
- `dispatch_then_jump_f388`: Calls the routine at fa54, then branches unconditionally to f388, transferring control without returning to the caller.
- `dispatch_then_jump_f95c`: Calls the routine at f648, then branches unconditionally to f95c, transferring control without returning to the caller.
- `lookup_terrain_and_copy_unit_fields`: Saves registers, scans a 37-entry word table for the value masked from (a4), stores the indexed entry into 16(a4), then copies unit fields into a6 and a5 and jumps to f95c.
- `advance_sprite_frame_and_animation`: Calls a helper twice, writes sprite control words and animation pointers into a6 offsets, then advances a5 frame counter, handles pending frame and direction bytes, and wraps the animation index against a table limit.
- `dispatch_sprite_command_by_low_nibble`: Calls a helper, reads the low nibble of a4+2 and branches to different sprite command handlers, defaulting to copying a4+16 into the a6/d5 display slot.
- `copy_sprite_word_to_display_slot`: Copies the word at a4+16 into the a6 display slot indexed by d5, then returns to the caller.
- `select_index_from_a5_byte_mod3`: Loads byte at offset 9 from a5 into d0, divides by 3, swaps to get remainder, tests it and branches to the low-nibble or word path.
- `low_nibble_of_a4_byte3`: Loads byte at offset 3 from a4 into d0, masks with 15 to keep the low nibble, then branches to the shared continuation.
- `copy_a4_word10_to_output`: Loads the word at offset 16 from a4 into d2 and branches to the shared continuation that stores it to the output buffer.
- `find_threshold_entry_in_table`: Doubles d0, indexes a table by the byte at offset 30 of a4 times 72, then scans up to 37 words comparing each against the word at offset 16 of a4 and returns the matching word in d2.
- `store_d2_to_a6_d5_slot`: Writes the word in d2 to the memory at offset 6 of a6 plus the signed word in d5, then returns to the caller.
- `decrement_counter_and_clamp_low_bits`: If the byte at offset 9 of a5 is zero, subtracts a masked byte from the word at offset 16 of a4, clamps its low twelve bits to at least 113, and stores the low bits at offset 6 of a6 plus d5.
- `accumulate_masked_byte_into_word_field`: If byte at offset 9 of a5 is zero, masks byte at offset 3 of a4 with a nibble mask, adds it to the word at offset 16 of a4, clamps that word to 856, and stores the low 12 bits into a6 plus d5.
- `select_threshold_index_from_table`: Reads low 12 bits of word at a4, scans a 74-byte table indexed by byte at offset 30 of a4, adjusts index by flag bit, stores selected word at offset 18 of a4 and sets byte at offset 28 based on comparison with word at offset 16.
- `clear_word_at_a4_offset_18`: Sets the word at offset 18 of a4 to zero and returns immediately to the caller.
- `copy_pending_byte_to_offset_1d`: Loads the byte at offset 3 of a4 into d0; if nonzero, stores it at offset 29 of a4 and clears the byte at offset 3.
- `advance_or_retreat_timer_toward_target`: If the word at offset 18 of a4 is zero it returns; otherwise adds or subtracts the byte at offset 29 from the word at offset 16, clamps it to the target at offset 18, clears that target, and stores the resulting value at offset 6 of a6 plus d5.
- `subtract_d0_from_timer_and_clamp`: Subtracts d0 from the word at offset 16 of a4, clamps it up to the word at offset 18, clears that target, then stores the clamped value at offset 6 of a6 plus d5.
- `merge_volume_nibbles_into_byte`: Reads byte at offset 3 of a4, merges its low and high nibbles into the byte at offset 32 of a4, then returns.
- `compute_signed_volume_from_pan_byte`: Extracts a 5-bit index from byte 33 of a4, looks up a table via a0, scales by the low nibble of byte 32, negates if bit 7 set, and stores the result.
- `negate_volume_if_pan_negative`: Tests bit 7 of byte 33 of a4; if negative, sets d2 to all ones and subtracts d0 from d2, then branches onward.
- `copy_d0_to_d2_then_branch`: Copies the low byte of d0 into d2 and then branches forward into another routine, leaving all other registers untouched.
- `scale_table_byte_by_nibble_and_add_base`: Reads a byte from the table at a0 indexed by d0 into d2, multiplies it by the low nibble of a4+32, shifts right by seven, then adds the word at a4+16 unless the sign bit of a4+33 is set.
- `store_difference_and_advance_counter`: Subtracts d2 from d0 and stores the result at offset 6 of a6 plus d5, then adds a value derived from the byte at offset 32 of a4 into the byte at offset 33 of a4.
- `call_f6b8_then_jump_f8ac`: Calls the routine at f6b8, then branches unconditionally to the routine at f8ac, transferring control without returning here.
- `call_f762_then_jump_f8ac`: Calls the routine at f762, then branches unconditionally to the routine at f8ac, transferring control without returning here.
- `update_volume_and_pan_from_control_bytes`: Reads control bytes at offsets 3, 34, 35 and 36 of a4, merges nibbles into the volume byte, computes a scaled pan value from a table and sign bits, clamps it to 0 through 64, and stores it at a6 plus 8 plus d5.
- `adjust_d2_by_d0_when_flag_negative`: Tests the byte at offset 33 of a4; if its sign bit is clear it branches away, otherwise stores d2 to that byte and subtracts d0 from d2 before branching on.
- `copy_d0_to_d2_then_branch_2`: Moves the low byte of d0 into d2 and branches unconditionally to the shared continuation point.
- `scale_table_byte_by_masked_counter`: Loads a byte from a0 indexed by d0 into d2, multiplies it by the low nibble of the byte at offset 34 of a4, shifts right six bits, then adds the byte at offset 31 of a4 unless the byte at offset 35 is negative.
- `clamp_delta_to_64_store_word`: Subtracts d2 from d0, clamps the result to the range 0 through 64, stores it as a word at offset 8 of a6 indexed by d5, then adds a scaled nibble from a4 offset 34 into byte at a4 offset 35.
- `apply_byte_shift_to_counter`: Loads byte at a4 offset 3 into d0; if nonzero stores it at a4 offset 38, shifts it left by 7, and if below the word at a4 offset 22 subtracts it from that word and adds twice it to the long at a4 offset 8.
- `set_word_counter_to_one`: Writes the immediate word value 1 to offset 22 of the structure pointed to by a4 and returns to the caller.
- `add_high_nibble_to_energy_capped`: Reads the high nibble of byte at offset 3 of a4, adds it to the byte at offset 31 of a4, caps that at 64, then stores the byte into the word at offset 8 of a6 indexed by d5 and returns.
- `subtract_low_nibble_from_energy_clamped`: Reads the low nibble of byte at offset 3 of a4, subtracts it from the byte at offset 31 of a4, clamps that to zero if negative, then stores the byte into the word at offset 8 of a6 indexed by d5 and returns.
- `init_counter_from_byte_minus_one`: Copies the byte at offset 3 of a4 to offset 10 of a5, decrements it, clears the byte at offset 11 of a5 and sets the byte at offset 12 of a5 to all ones, then returns.
- `clamp_a4_byte_to_64_store_a4_and_a6`: Reads the byte at offset 3 of a4, caps it at 64, writes the capped value to offset 31 of a4 and to the word at offset 8 of a6 indexed by d5, then returns.
- `decode_two_bcd_digits_to_a5_byte`: Reads the byte at offset 3 of a4, splits it into high and low nibbles, forms high times 10 plus low, and if the result is at most 63 stores it at offset 11 of a5 and sets the byte at offset 12 of a5.
- `validate_a4_byte_and_store_in_a5`: Reads the byte at offset 3 of a4; if it is zero or greater than 31 it branches away, otherwise it clears the byte at offset 9 of a5 and stores the value at offset 8 of a5 before returning.
- `dispatch_low_nibble_command`: Calls a setup routine, reads the low nibble of byte 2 of the a4 structure and branches to a different handler for each value from 9 to 15, otherwise jumps to a default handler.
- `dispatch_high_nibble_command`: Reads the high nibble of byte 3 of the a4 structure and branches to a different handler for each value from 0 to 15, returning immediately if no value matches.
- `set_ciaa_fire_button_bit`: Reads bit 0 of byte 3 of the a4 structure, shifts it into bit 1, clears that bit in CIAA_PRA and writes the new value back to the port.
- `merge_low_nibble_into_byte_25`: Reads byte at offset 3 of a4, masks it to the low nibble, clears the low nibble of byte at offset 37 of a4, then ORs the nibble in and returns.
- `merge_low_nibble_into_byte_24`: Reads byte at offset 3 of a4, masks it to the low nibble, clears the low nibble of byte at offset 36 of a4, then ORs the nibble in and returns.
- `copy_low_nibble_to_byte_1e`: Reads byte at offset 3 of a4, masks it to the low nibble, stores that value into byte at offset 30 of a4, and returns.
- `check_timer_and_decrement_counter`: Tests byte at offset 9 of a5 and branches away if set; reads low nibble of byte at offset 3 of a4, and if nonzero and byte at offset 40 of a4 is set, decrements that byte and copies bytes at offsets 39 and 13 of a4 into a5.
- `store_d0_to_a4_offset_40`: Writes the byte in d0 to offset 40 of a4, then branches back into the preceding routine's code.
- `store_shifted_a5_word_to_a4_offset_39`: Loads the word at offset 4 of a5 into d0, shifts it right by four bits, stores the low byte at offset 39 of a4, and returns.
- `merge_low_nibble_into_flags_byte`: Reads the low nibble of byte at offset 3 of a4, shifts it left four bits, clears the low nibble of byte at offset 36 of a4 and ORs the shifted value into it.
- `emit_sprite_frame_if_divisible`: Checks nibble at a4 offset 3 and a5 offset 9, divides to test divisibility, then writes two sprite descriptors to a6 offsets 150 and indexed d5, calling the sprite routine between them.
- `branch_on_a5_flag_to_f550_or_f8b8`: Tests byte at offset 9 of a5; if nonzero branches to f550, otherwise loads low nibble of a4 offset 3 masked with 13 into d0 and branches to f8b8.
- `check_a5_flag_then_read_a4_nibble`: Tests the byte at offset 9 from a5; if zero, loads the low nibble of the byte at offset 3 from a4 into d0 and branches onward.
- `compare_a4_nibble_with_a5_byte`: Loads the low nibble of the byte at offset 3 from a4 into d0, compares it with the byte at offset 9 from a5, and on match clears the byte at offset 31 from a4 and the word at offset 8 from a6 indexed by d5.
- `match_nibble_then_test_a4_word`: Compares the low nibble of the byte at offset 3 from a4 with the byte at offset 9 from a5, and on match tests the word at a4, pushing d1 before branching if nonzero.
- `read_low_nibble_to_offset_f`: Tests byte at offset 9 of a5 and branches away if set; otherwise reads byte at offset 3 of a4, masks low nibble, tests byte at offset 16 of a5, increments and stores result at offset 15 of a5.
- `merge_high_nibble_into_offset_25`: Tests byte at offset 9 of a5 and branches away if set; otherwise takes low nibble of byte at offset 3 of a4, shifts it left four bits, clears low nibble of byte at offset 37 of a4 and ORs the shifted value in.
- `advance_pointer_and_invert_byte`: Saves d1 and a0, uses high nibble of byte at offset 37 of a4 to index a table and add to byte at offset 41 of a4, then advances pointer at offset 4 of a4 and inverts the byte it points to.
- `wait_for_byte_change_three_tries`: Saves d1 on the stack, then polls the byte at offset 6 of a6 up to four times waiting for it to change, restores d1 and returns.
- `select_sound_sample_by_mode_byte`: Reads a mode byte and, depending on whether it is one, two or other, loads a sample pointer, sets audio period, volume and length fields at offsets in a6, stores a new mode byte and calls the setup and play routines.
- `setup_audio_channel_zero_playback`: Points a6 at the custom chip base, loads a sample address into a0, calls the sample setup routine, writes the pointer and length two to the audio channel zero registers, then calls the playback start routine and returns.
- `wait_for_beam_counter_change`: Sets word at offset 150 of a6 to 1, then repeatedly reads the longword at offset 4 of a6, shifts it right 8 bits and masks to compare successive beam counter values until they differ.
- `wait_for_two_beam_counter_changes`: Sets word at offset 150 of a6 to 32769, then polls the longword at offset 4 of a6 shifted and masked, waiting for two successive changes of the beam counter value.
- `swap_buffers_and_tick_counters`: Saves all registers, clears and increments global counters, swaps two buffer pointers and fills five entries with addresses, then decrements timers and calls a routine when one reaches zero.
- `store_byte_and_call_if_flag_set`: Writes the low byte of d0 to a fixed memory location, tests a flag byte and calls a subroutine when it is nonzero, then returns zero in d0 after restoring all registers.

### Lords of the Realm (1995)(Impressions)[cr PDX](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
