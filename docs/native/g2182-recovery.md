# Game 2182 (megaball (1991)(e. mackey)(sw)[a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02182). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903820 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 260 of 260 named (0 author symbols), 260 of 260 described (30 by rule, 59 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2803 of 2803 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6371 of 6371 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 391836 |
| code | 24726 |
| data | 129901 |
| padding | 3565 |
| unused | 353792 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| MegaBall (1991)(E. Mackey)(SW)[a3].adf:iff.library | 3 | 673 | 40 |
| 8 | 1 | 2 | 1 |
| iff.library | 3 | 673 | 49 |
| diskfont.library | 13 | 830 | 53 |
| MegaBall (1991)(E. Mackey)(SW)[a3].adf:MEGABALL | 3 | 3636 | 85 |
| MegaBall (1991)(E. Mackey)(SW)[a3].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 547 | 31 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `load_board_or_reset_table` (MEGABALL): Selects a 68-byte entry from a table, clearing every entry when the selector is the reset value. Kind: board file/hardware lookup. Pass path: sub_0_0C42 (off F8C) or sub_0_2B88 (off 2BAE).

## Functions


### MegaBall (1991)(E. Mackey)(SW)[a3].adf:iff.library

- `reject_direct_execution`: Entry point of iff.library when it is run directly rather than opened as a library.
- `init_iff_list_header`: Part of iff.library: stores the caller's value in a record, allocates a second block through a library call, and sets up an empty list head that points to itself.
- `allocate_iff_node_entry`: Part of iff.library: allocates a node through a library call when a flag allows it, stamps it with a value from the system and adds it to a list.
- `release_iff_node_countdown`: Part of iff.library: when a flag is clear, frees and unlinks a node through library calls.
- `finish_iff_node_or_pend`: Part of iff.library: when the countdown is zero, releases the node through library calls and returns a value stored in it.
- `free_iff_node_and_return`: Part of iff.library: calls library routines to close, free and unlink a node, then returns a value stored in it.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `open_iff_form_file`: Part of iff.library: opens a file through the DOS library, reads its header and checks for the FORM tag.
- `create_iff_form`: Library code that attempts to write an IFF FORM header by allocating a record and checking a type tag.
- `write_iff_form_header`: Part of iff.library: writes an IFF FORM header with a size and type tag through library calls, allocating the record first.
- `create_iff_chunk_record`: Part of iff.library: allocates a chunk record of a given size and tag, writes its header through a library call and returns the record.
- `read_iff_chunk_header`: Part of iff.library: reads an IFF chunk header through the DOS library and stores its tag and size in the caller's buffer.
- `close_iff_file_handle`: Part of iff.library: stores a tag and two values in a record and closes the file through the DOS library.
- `call_routine_and_return`: Part of iff.library: calls one routine in the same module and returns, passing no values through.
- `free_block_if_allocated`: Part of iff.library: frees a memory block through exec when its pointer is nonzero, then calls another routine in the same module and returns.
- `find_iff_chunk_by_tag`: Part of iff.library: walks the IFF chunk list from a given start, comparing each chunk tag with the requested value.
- `call_and_offset_result`: Part of iff.library: calls another routine and adds eight to its result when a flag is clear.
- `unpack_packed_pixel_words`: Part of iff.library: unpacks the three-byte groups written by the picture writer back into 16-bit words.
- `decode_ilbm_picture_body`: Part of iff.library: checks an IFF FORM for the ILBM type, copies the BMHD header fields and decodes the picture body row by row through a selected decompression routine.
- `tiny_move`: Small routine of 1 instruction that copies (a0)+ to (a1)+.
- `copy_counted_bytes`: Part of iff.library: copies a block of bytes one at a time from a source to a destination for a counted length.
- `unpack_packbits_runs`: Part of iff.library: unpacks a PackBits-style stream: a negative control byte repeats the following byte, and a non-negative control byte copies literal bytes until the output is full.
- `initialize_iff_header`: Prepares the necessary data structures for an ILBM file by setting up the FORM and BMHD headers.
- `write_iff_picture_file`: Part of iff.library: builds a FORM header with the ILBM type and a BMHD header, packs each 16-bit word into three bytes, and writes the chunks through the DOS library.
- `finish_iff_file_write`: Part of iff.library: checks the written size, seeks back to write the final length, and closes the file through library calls.
- `close_iff_file_if_open`: Part of iff.library: if a flag is set, closes the open file through the DOS library, then calls another routine in the same module and returns.
- `accumulate_and_write_rows`: Part of iff.library: builds eight running sums from table words and a scaled multiplier, then writes each row through the DOS library while adding the first word into each sum.
- `accumulate_rows_recursive`: Part of iff.library: fills a 72-entry buffer of running sums from table words, calls itself to total the sub-results, and writes each row through the DOS library.
- `pack_bits_encode_stream`: Part of iff.library: compresses a stream in PackBits style, counting runs of identical bytes and writing a repeat or literal header for each, then returns the number of bytes produced.
- `pack_bits_encode_literals`: Part of iff.library: PackBits-style encoder that writes literal runs and repeat headers into the output buffer and returns the total output length.
- `pack_bits_encode_block`: Part of iff.library: packs a block into PackBits-style repeat and literal runs, writing the headers into the output and returning the total output length.
- `count_identical_run_length`: Part of iff.library: counts how many following bytes equal the first byte, stopping at a run limit of 128 or at the end of the input.
- `find_current_task_entry`: Part of iff.library: gets a value from the system through a library call, then walks a linked list and returns the node whose stored field matches it, or zero when none matches.
- `clear_field_on_flag`: Part of iff.library: calls another routine in the same module and, when its result is nonzero, zeroes a field in the record.
- `store_value_if_nonzero`: Part of iff.library: calls another routine in the same module, then stores the input value in a record field when that value is nonzero.
- `take_field_value_once`: Part of iff.library: calls another routine and, when it returns nonzero, returns a value stored in the record and clears that field.
- `return_minus_one`: Part of iff.library: returns minus one with no other work.
- `read_record_word_field`: Part of iff.library: calls another routine and returns the word stored at a fixed offset in the record.
- `flag_byte_field_over_five`: Part of iff.library: calls another routine and, when a byte field in the record exceeds five, sets the bit worth 2048 in the returned value.
- `compute_status_bits`: Part of iff.library: builds a status word from two record fields, setting one bit when a value is within a limit and another bit when a second field exceeds its limit.

### 8

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### iff.library

- `reject_direct_execution_2`: Part of iff.library: entry point when it is run directly rather than opened as a library.
- `initialize_list_node`: Library code that initializes a structure by storing a pointer and setting a linked list pointer to itself plus an offset.
- `update_task_status`: Library code that updates a task entry with a value from another routine and increments a counter in a structure.
- `return_first_argument`: Part of iff.library: returns the first value passed on the stack, with no other work.
- `decrement_node_counter`: Library code that decrements a counter in a structure and calls another routine if the counter reaches zero and a specific bit is set.
- `release_iff_node`: Library code that checks if a counter is zero to release a node through library calls, or sets a bit if the counter is non-zero.
- `free_node_data`: Library code that performs cleanup by calling two routines and returning a value from a structure.
- `return_from_subroutine`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `write_iff_form_header_2`: Library code that attempts to write an IFF FORM header into a memory buffer.
- `initialize_iff_form_header`: Allocates a memory buffer and writes an IFF FORM header containing a size and type tag.
- `create_iff_chunk`: Library code that writes a chunk record by checking a type tag and populating a record with provided parameters.
- `allocate_iff_record`: Library code that allocates memory and populates a record with two parameters and a size value.
- `populate_iff_record`: Library code that populates a record with two parameters and a size value after a failed DOS read operation.
- `store_record_parameters`: Library code that stores two parameters into a record and calls another routine.
- `call_conditional_field_clear`: Calls another routine and, if the return value is non-zero, zeroes a field within a record via a pointer.
- `free_iff_memory`: Library code that calls a memory free routine if a pointer is non-zero.
- `search_iff_chunk_list`: Library code that traverses a linked list of chunks to find an entry matching a specific value.
- `adjust_chunk_offset`: Library code that calls another routine and returns an offset based on a boolean result.
- `unpack_pixel_data`: Library code that unpacks bytes from a buffer into a destination table of words using bitwise shifts and masks.
- `decode_ilbm_data`: Library code that selects and executes a decompression routine to process ILBM image data.
- `decode_ilbm_row_loop`: Part of iff.library: a row decode loop of the ILBM body reader that picks between two decompression routines by a stored compression flag.
- `decode_ilbm_data_2`: Library code that checks a byte value against a threshold and then performs conditional unpacking of data into a buffer.
- `process_iff_chunk`: Library code that locates an IFF chunk by its tag and then proceeds to unpack its contents if a specific flag is set.
- `unpack_iff_data`: Library code that conditionally unpacks a compressed data stream into a destination buffer using a loop.
- `conditionally_unpack_data`: Library code that checks a flag before proceeding to unpack a data stream.
- `unpack_packbits_stream_3`: Library code that unpacks a PackBits-style compressed data stream into a destination buffer.
- `branch_to_subroutine`: Performs a subroutine call to another routine within the library.
- `copy_memory_block`: Library code that copies a specified number of bytes from one memory location to another.
- `copy_byte_stream`: Library code that copies bytes from a source pointer to a destination pointer.
- `unpack_packbits_stream_2`: Library code that decodes a PackBits compressed stream by either repeating bytes or copying literal runs into a destination.
- `unpack_packbits_stream`: Part of iff.library: unpacks a PackBits-style stream in which each control byte selects a repeat or a literal copy, writing into the destination until the input length is used up.
- `prepare_iff_header_data`: Initializes data structures required for an ILBM file by setting up the FORM and BMHD headers.
- `write_iff_header`: Library code that constructs an IFF FORM header and converts bitplane data into a packed byte format for file output.
- `close_iff_file`: Library code that handles closing a file and performing cleanup based on specific status flags.
- `clear_field_on_flag_2`: Library code that calls another routine and zeroes a specific field if the result is nonzero.
- `calculate_row_offsets`: Library code that calculates memory offsets for rows based on image dimensions and bitplane data.
- `encode_packbits_stream`: Library code that calculates row offsets and prepares data for PackBits encoding.
- `pack_bits_encode_literals_2`: Library code that encodes a sequence of non-repeating bytes into a PackBits literal run.
- `pack_bits_encode_block_2`: Library code that processes a block of data to produce a PackBits encoded stream containing both literal and repeat runs.
- `encode_packbits_stream_2`: Library code that implements PackBits compression by identifying runs of identical bytes and encoding them as either repeat or literal sequences.
- `count_identical_bytes`: Library code that counts how many consecutive bytes at a given memory location are identical to the first byte.
- `find_task_node`: Library code that uses a system call to find the current task and traverses a linked list to find a node matching a specific value.
- `clear_task_field`: Library code that finds the current task and sets a value at a specific offset within that task's data to zero.
- `set_task_field`: Library code that finds the current task and stores a provided integer value into a field at a specific offset.
- `get_and_clear_task_field`: Library code that finds the current task, retrieves a value from a specific offset, clears that field, and returns the value or minus one if the task is not found.
- `return_constant_error`: Library code that simply returns a constant value of negative one.
- `evaluate_chunk_field`: Library code that retrieves a value from a chunk and sets a specific bit if a byte at a certain offset exceeds five.
- `check_field_threshold`: Library code that calls another routine and sets a bit in the result if a byte at a specific offset is greater than five.
- `calculate_status_flags`: Library code that checks two different fields in a structure to return a bitmask representing specific threshold conditions.

### diskfont.library

- `store_font_entry_argument`: Part of diskfont.library: calls a library routine that returns a font entry.
- `check_and_set_font_handle`: This library code calls a routine via a table entry using the library base.
- `create_font_library_state`: Part of diskfont.library: allocates a state record through exec, then allocates a second block, and sets up an empty list head inside the record.
- `unload_unused_font_entries`: Part of diskfont.library: walks a list of font entries, unlinks each entry whose count is not positive, and unloads its code segment through the DOS library.
- `release_font_entry_or_mark`: Part of diskfont.library: when the entry's usage count is zero and its count is not positive, closes libraries, frees and unlinks the entry and returns a value stored in it.
- `mark_font_entry_pending`: Part of diskfont.library: sets the pending bit on a font entry and returns zero.
- `return_immediately`: This library routine is empty.
- `clear_pending_increment_count`: Part of diskfont.library: clears the top bit of a byte flag in the entry and increments a 16-bit counter stored beside it.
- `decrement_count_and_dispatch`: Part of diskfont.library: decrements a 16-bit counter in the entry and, while it is nonzero and the pending bit is set, calls a routine through the library table.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_font_offset`: Part of diskfont.library: computes a signed offset from the difference between two 16-bit fields.
- `load_font_from_fonts_dir`: Part of diskfont.library: copies an 8-byte name from the caller, locks the FONTS: directory, opens and reads the font file, and registers the font through the graphics library.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `copy_null_terminated_string`: Part of diskfont.library: copies bytes from a source string to a destination until a zero byte has been copied.
- `append_string_to_end`: Part of diskfont.library: advances a pointer to the terminating zero of a string, then copies the other string onto that point.
- `forward_string_copy`: Part of diskfont.library: calls the string copy routine with the arguments already in place and returns.
- `count_string_length`: Part of diskfont.library: counts the bytes of a zero-terminated string before its terminator and returns the count.
- `compare_strings_ordered`: Part of diskfont.library: compares two zero-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they are equal.
- `compare_previous_byte_order`: Part of diskfont.library: compares the byte before one pointer with the byte at another pointer and returns minus one when the first is lower, otherwise one.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_in_string`: Part of diskfont.library: scans a zero-terminated string for a given byte and returns a pointer to its first occurrence, or zero when none is found.
- `find_char_from_pointer`: Part of diskfont.library: scans forward from a pointer for a given byte and returns a pointer to the first match, or zero when the terminator comes first.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `find_last_char_in_string`: Part of diskfont.library: measures a zero-terminated string and scans backwards from its end for a given byte.
- `find_path_separator`: Part of diskfont.library: searches a zero-terminated name for a slash and then for a colon.
- `collect_font_names_into_buffer`: Part of diskfont.library: walks a list of font entries and copies name text from them into a buffer when a flag bit is set.
- `write_font_data_records`: Part of diskfont.library: copies font data bytes into a buffer in a loop, reducing a remaining count and writing a record header for each chunk.
- `write_font_data_chunks`: Part of diskfont.library: copies bytes one at a time from a source into a buffer, and when the buffer reaches its limit writes a record header with the stored size.
- `scan_fonts_dir_for_fonts`: Part of diskfont.library: opens the FONTS: directory and examines its entries for files with the .font extension, allocating memory and locking the directory as it goes.
- `read_font_file_header`: Part of diskfont.library: reads a block of font file data into a buffer and checks that a tag word equals 3840.
- `read_font_data_records`: Part of diskfont.library: reads font data from a file in a loop, copying bytes into a buffer and checking each record's tag.
- `read_font_name_records`: Part of diskfont.library: reads a 260-byte block from the file, stores each record's values into a list entry and counts the entries.
- `restore_directory_and_free`: Part of diskfont.library: restores the current directory, unlocks the directory and frees memory through library calls.
- `return_zero`: Part of diskfont.library: returns zero with no other work.
- `call_routine_and_return_2`: Part of diskfont.library: calls one routine in the same module and returns, passing no values through.
- `call_external_handler`: This library code calls another unnamed routine and returns, acting as a dispatcher for an external function pointer.

### MegaBall (1991)(E. Mackey)(SW)[a3].adf:MEGABALL

- `handle_workbench_startup`: Program entry point: when the process was started from Workbench rather than the CLI, it takes the startup message with Forbid and GetMsg, runs another routine, then permits multitasking and replies to the message.
- `call_wait_t_o_f`: Wrapper that calls the graphics library function WaitTOF and returns its result to the caller.
- `wait_video_frames`: Waits for the number of video frames given in the register, calling the graphics wait-for-frame routine once per frame.
- `run_main_setup_and_menu`: Main program routine: checks that the diskfont library and the MegaBall font are present, loads the BALL1 and BALL2 pictures and the MegaBall music, opens the graphics and audio resources, then shows a prompt to press 2 for two players or SHIFT-Q to exit.
- `poll_message_loop_and_shutdown`: Polls the exec message port in a loop until a message arrives.
- `poll_message_and_reply`: Polls the exec message port in a loop, replying to each message that arrives.
- `update_playfield_frame`: Per-frame update that copies memory blocks, moves sprites, clears rasters and blits bitmaps, while taking and replying to exec messages.
- `compute_scroll_step_and_draw`: Updates per-frame timing values, computes a scroll step from a width field and calls the drawing routines once per pass.
- `advance_playfield_buffers`: Counts down a row counter and, when it reaches zero, swaps pairs of display buffer pointers and clears their state.
- `draw_scroll_band_rows`: Advances an offset by 40 bytes and draws bands of rows in a loop, computing a step size from a width field and clamping it between 12 and 265.
- `reset_frame_state_and_update`: Increments a frame counter, clears four state fields, then enters the shared per-frame update that draws rows and computes the scroll step.
- `set_pending_flag_then_update`: Sets one status flag word to one and continues into the common per-frame update that draws rows and computes the scroll step.
- `advance_shift_and_update`: Doubles a shift value up to a limit, clears a pending entry, then continues into the common per-frame update that draws rows and computes the scroll step.
- `validate_hunk_type`: Compares a value against AmigaDOS hunk type codes to identify executable or relocation segments.
- `validate_hunk_type_2`: Compares a value against AmigaDOS hunk type codes to identify executable or relocation segments.
- `validate_hunk_types`: Compares data values against AmigaDOS hunk type codes to identify executable or relocation segments.
- `remap_tile_codes_in_table`: Walks a table of byte codes.
- `fill_tile_codes_row`: Writes tile codes along a row, mapping the previous code 7 to 3, 11 to 2 and other values to 10.
- `cycle_tile_codes_row`: Writes tile codes along a row in a repeating pattern starting from two, calling a routine for each cell until the row ends.
- `blit_tile_row_codes`: Writes tile codes along a row starting from three and blits the cell bitmap for each cell through the graphics library.
- `set_two_flags_then_update`: Sets two status flags to one and continues into the common per-frame update that draws rows and computes the scroll step.
- `cycle_buffer_pointer_slots`: When the playback flag is clear and a buffer slot is set, cycles a counter through four buffer slots and loads the next nonzero slot pointer.
- `select_next_buffer_slot`: Selects the next nonzero buffer slot pointer from four slots using a rotating counter, then runs a routine.
- `cycle_stored_state_values`: Steps a small index counter through several stored state values and loads the selected one into a state longword.
- `select_next_state_from_table`: Chooses the next of several stored state values using a wrapping index and loads it into the state longword.
- `run_frame_update_loop`: Runs the per-frame loop, which refreshes a scaled height from the playfield, calls indexed graphics routines through the stored base, and counts frames.
- `decrement_countdown_and_halve_speed`: Decrements a remaining-steps counter while it is active, halves a speed value, and sets a redraw flag.
- `set_state_to_minus_one`: Stores the value minus one in a 16-bit state word and then continues into the shared frame code.
- `setup_audio_sprites_with_cleanup`: Acquires the audio device and three hardware sprites, printing an error message when either cannot be obtained.
- `draw_mega_title_with_sprites`: Draws the title text MEGA with bit-block copies and moves sprites into position.
- `drain_message_port_replies`: Loops over queued messages on a port, replying to each one until the port is empty.
- `poll_input_messages_for_flag`: Waits for the vertical blank, then reads queued messages.
- `update_bounce_timer_and_direction`: Counts down a movement timer and, when it expires, reloads it from a value scaled by a speed setting.
- `forward_graphics_library_call`: Calls one graphics library entry through the stored graphics base and returns, leaving the register results unchanged.
- `lookup_playfield_cell_at_position`: Converts an x and y position into a cell index of a grid with 20 cells per row, using a border offset and cell sizes of 15 by 7.
- `clear_playfield_cell_and_count`: Looks up the playfield cell under a position and, when it holds an object, clears the cell, decrements a remaining count and redraws the cell with a bit-block copy.
- `write_word_as_decimal_digits`: Converts a value to decimal text by dividing by ten thousand and then by successively smaller powers of ten.
- `write_decimal_digits_by_divisor`: Writes a value as decimal text using a divisor passed in a data register.
- `tally_score_until_target_reached`: Redraws a row of score markers while the score divided by 20000 is below the stored target, stepping a counter each time.
- `tally_score_with_status_labels`: Steps the displayed score upward in a nested loop until it reaches the stored target, redrawing the marker row at each step.
- `redraw_occupied_playfield_cell`: Redraws a playfield cell with a bit-block copy when the position falls exactly on a cell boundary and the cell is occupied.
- `forward_graphics_call_with_result`: Calls one graphics library entry through the stored graphics base and returns the two result registers it leaves.
- `draw_sprite_table_entries`: Walks a table of four-word entries and, for each one, computes a horizontal offset clamped to the screen width from a stored value.
- `set_palette_to_offset_black`: Scales each 12-bit colour table entry by a zero factor, adds a fixed bias, and stores the result in the palette buffer.
- `fade_palette_to_partial_level`: Scales each 12-bit colour entry by a fixed level of twelve out of fifteen, then stores the result in the palette buffer and calls a library routine.
- `fade_palette_to_black`: Scales each 12-bit colour entry by a zero factor and stores the result in the palette buffer.
- `fade_palette_to_full_brightness`: Scales each 12-bit colour entry by the full factor of fifteen, stores the result, and loads the palette with the graphics LoadRGB4 call.
- `abort_pending_io_requests`: Calls the library's AbortIO-style entry repeatedly until it returns zero, so no device requests remain outstanding.
- `wait_for_device_requests`: Calls the library's WaitIO-style entry repeatedly until it returns zero, blocking until outstanding device requests complete.
- `load_board_or_reset_table`: Selects a 68-byte entry from a table, clearing every entry when the selector is the reset value.
- `open_dos_library`: Opens dos.library through the exec library table and stores the returned base for later file calls.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `open_intuition_library`: Opens intuition.library through the exec library table and stores the returned base for the window and screen calls.
- `close_stored_library_base`: Closes the library whose base is held in the global library slot through the exec close call.
- `open_graphics_library`: Opens graphics.library through the exec library table and stores its base for the drawing calls.
- `call_function_via_pointer`: Calls a function through a pointer stored at an offset from a global variable.
- `open_diskfont_library`: Opens diskfont.library through the exec library table and stores its base in the font slot.
- `call_function_via_pointer_2`: Executes a subroutine by dereferencing a pointer located within a global structure.
- `open_iff_library_or_fail`: Opens iff.library and, when the open fails, records an error pointer so the caller can report that the library cannot be found.
- `call_function_via_pointer_3`: Uses a pointer from a global variable to jump to another routine.
- `load_music_for_free_memory`: Compares the available memory with a set of thresholds to choose a memory budget.
- `read_scores_file_into_buffer`: Copies a block of longwords into the scores buffer and then reads the MegaBall.scores file if it opens.
- `write_scores_file`: Opens the MegaBall.scores file and writes the score table to it, then closes the file, provided the open succeeded.
- `load_ball_iff_image`: Loads the BALL2 picture through the iff library.
- `return_empty`: An empty routine that performs no actions and returns.
- `decode_iff_picture_checked`: Decodes the picture held by the iff library and, when decoding fails, records an error pointer and releases the iff resources.
- `open_custom_screen_or_fail`: Opens a custom screen through intuition and, when the open fails, records an error pointer for the caller to report.
- `call_close_screen`: Wrapper that calls the intuition library function CloseScreen and returns its result to the caller.
- `open_window_handle`: Opens a window through intuition and stores the returned window handle for later drawing calls.
- `open_main_window_or_fail`: Opens the main window through intuition and, when the open fails, records an error pointer for the caller to report.
- `call_close_window`: Wrapper that calls the intuition library function CloseWindow and returns its result to the caller.
- `next_pseudo_random_value`: Advances a 32-bit pseudo-random state by mixing it with several register values, multiplying and adding a fixed constant.
- `call_if_slot_nonzero`: Calls the shared routine only when a stored slot value is nonzero, and returns the register pair unchanged otherwise.
- `check_global_flag`: Checks a global variable and calls another routine if a specific condition is met.
- `call_then_return_zero`: Calls one shared routine and returns the value zero to its caller.
- `call_shared_routine_pass_through`: Calls one shared routine and returns the register pair unchanged.
- `find_next_larger_table_byte`: Clears four state words and calls the shared routine, then scans a byte table for the next entry larger than the current one.
- `scan_table_set_audio_volumes`: Clears four state words and runs the same table scan as its neighbour.
- `silence_audio_channels_if_active`: When its active flag is set, it clears the flag, zeroes the four audio channel volumes and clears the DMA control bits for the sound channels.
- `advance_audio_playback_step`: Advances a sound playback step counter while playback is active.
- `load_audio_channel_sample_entry`: Copies a four-byte entry into a channel structure and, when the sample index is nonzero, loads the sample pointer and length from a per-sample table, updating the channel volume field.
- `copy_channel_word_to_output`: Copies one 16-bit value from the channel record into the location given by a stored pointer.
- `scale_channel_volume`: Multiplies a stored 16-bit value by a global scale factor, shifts the product right by four bits, and writes the result as the channel volume.
- `compare_channel_value_to_target`: Masks a 12-bit value from the channel record and stores it.
- `update_channel_effect_state`: Reads the effect nibble of a channel record and, when it is nonzero, chooses one of three stored values using a counter modulo three.

### MegaBall (1991)(E. Mackey)(SW)[a3].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_infinite_loop_gap`: Only spins forever, so it is a filler gap in the decoded code rather than a real routine with behaviour.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `stub_bad_instruction_data`: A single bad-instruction trap with no real code, so it carries no behaviour.
- `spin_after_task_setup`: Its decoded body only spins in an infinite loop with no exit.
- `release_node_when_count_expires`: Decrements a countdown field in a node, and when the count reaches zero with a flag bit set it runs the removal routine.
- `finish_node_or_defer_release`: When the countdown in a node has reached zero it removes the node, releases its memory and returns a stored result.
- `remove_and_free_node_value`: Removes the node from its list, frees its memory through the exec entries and returns a value stored in the node.
- `find_list_entry_by_key`: Walks a list of variable-length records, each with a key word and a size field, and returns the record whose key equals the value passed in.
- `find_entry_offset_plus_eight`: Looks up a list entry with the shared search routine and returns the address eight bytes past the found value.
- `copy_bytes_counted_inclusive`: Copies bytes from the source to the destination, decrementing a count until it wraps past zero.
- `copy_bytes_until_counter_wraps`: Copies bytes one at a time, decrementing a counter before each copy and stopping when the counter wraps to minus one.
- `decode_run_length_bytes`: Decodes a run-length stream in which a negative control byte repeats the following byte the given number of times, and a positive control byte copies that many literal bytes to the output.
- `decode_packed_byte_runs`: Writes a repeated byte run for each control value and then copies literal bytes from the input, continuing until the output count is used up.
- `call_shared_routine_only`: Calls one shared routine and returns without doing any other work.
- `write_ilbm_picture_file`: Writes an IFF ILBM picture by emitting a FORM header, an ILBM tag and a 20-byte bitmap header chunk.
- `write_sized_chunk_with_padding`: Pads a picture chunk length to a word boundary and writes it through the file library entries, checking each step.
- `close_file_if_open`: Closes the open file handle when the open flag is set, and then calls the shared routine.
- `scale_eight_table_terms`: Multiplies a scale value by an eight-entry table of coefficients and stores the eight results in a local buffer, using 16-bit fixed-point arithmetic.
- `fill_wide_fixed_point_buffer`: Fills a local buffer with eight fixed-point products built from a shared scale value and table coefficients, then continues the calculation over a wider working buffer.
- `count_run_length_until_limit`: Counts successive bytes of an input stream while they stay below a limit, adding each run length to a running total.
- `encode_run_with_length_byte`: Scans input for a run of up to 128 bytes, writes the run length as a single byte, and copies the run to the output.
- `encode_literal_runs_with_header`: Copies literal runs of input bytes to the output, writing a length byte before each run and accumulating the total count.
- `count_identical_byte_run`: Counts how many consecutive input bytes repeat the first byte.
- `find_task_list_node`: Looks up the current task identifier and walks a linked list of nodes until one stores the same identifier, returning that node or a null pointer.
- `clear_field_and_return_true`: Clears a 32-bit field in the record when a condition flag is not set, then returns one.
- `store_nonzero_field_value`: Stores the input value in a record field when the value is nonzero, and returns zero.
- `compute_position_limit_flags`: Builds a status word from a record.
- `read_ilbm_header_fields`: Checks for an ILBM chunk tag and, when it matches, copies four header words from the input record into a local array before handling the picture data.
- `expand_byte_triples_to_words`: Converts groups of three input bytes into one 16-bit value by combining their nibbles, writing one word per group until the picture length is covered.
- `offset_pointer_result`: Adds eight to a value returned by another routine.
- `allocate_task_record`: When the zero condition holds, allocates a record and stores a task identifier in it.
