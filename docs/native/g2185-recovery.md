# Game 2185 (megaball (1991)(e. mackey)(sw)[m ale]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02185). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903820 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 287 of 287 named (0 author symbols), 287 of 287 described (38 by rule, 59 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2811 of 2811 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6784 of 6784 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 419742 |
| code | 26076 |
| data | 136952 |
| padding | 4122 |
| unused | 316928 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| MegaBall (1991)(E. Mackey)(SW)[m ALE].adf:iff.library | 3 | 673 | 40 |
| 8 | 1 | 2 | 1 |
| iff.library | 3 | 673 | 49 |
| diskfont.library | 13 | 830 | 53 |
| MegaBall (1991)(E. Mackey)(SW)[m ALE].adf:SHOW | 2 | 206 | 7 |
| MegaBall (1991)(E. Mackey)(SW)[m ALE].adf:MEGABALL | 3 | 3622 | 83 |
| MegaBall (1991)(E. Mackey)(SW)[m ALE].adf:bootblock | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 671 | 52 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- megaball (1991)(e. mackey)(sw)[m ale]/MegaBall (1991)(E. Mackey)(SW)[m ALE].adf:SHOW: PowerPacker 2.3 (xfd) stream found by ancient scan (5076 -> 6112 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- megaball (1991)(e. mackey)(sw)[m ale]/MegaBall (1991)(E. Mackey)(SW)[m ALE].adf:SHOW: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### MegaBall (1991)(E. Mackey)(SW)[m ALE].adf:iff.library

- `open_helper_library_and_return`: Entry code for the iff library that opens a second library through the exec library, calls two of its routines, closes it and returns the value five.
- `initialize_empty_list_header`: Stores its argument in a structure, records the result of a library call in another field, and sets up an empty list header inside the structure.
- `allocate_and_queue_task_record`: When a condition holds, looks up the current task, allocates a memory block, stores the task pointer in it and adds it to a list.
- `remove_node_and_decrement_count`: Removes an item from its list and frees its memory when a condition holds, then decrements a counter.
- `remove_close_library_when_idle`: When its counter is zero, removes an item from its list, closes a library, frees the memory and returns a stored value.
- `remove_close_and_free_record`: Removes an item from its list, closes a library and frees its memory, then returns a value read from the record.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `read_iff_form_record`: Opens a file, reads a chunk header, and when the chunk type is FORM allocates a record and fills it with the size and position read from the file.
- `process_iff_form_record`: Library code that checks if a chunk is a FORM type, allocates a record, and populates it with size and field data before closing a file.
- `create_form_record_from_args`: Checks that a chunk type is FORM, allocates a record for it, and stores the given size and two values in the record.
- `allocate_iff_chunk_record`: Allocates a memory block, stores a size and chunk type in it along with two further values, and returns the block, or zero when allocation fails.
- `read_chunk_header_record`: Stores a value in the first field of a record, reads data from the file into the record, and records two more values in later fields.
- `store_fields_and_close_file`: Stores three values into a record, then closes the file through the dos library and returns.
- `forward_to_iff_helper`: Calls one routine and returns, adding no other work.
- `free_block_if_present`: Frees a memory block when one is passed in, using the exec library's free-memory routine, then calls another routine and returns.
- `find_chunk_record_by_id`: Walks a list of records stored in a block, comparing each record's first field with a given value, and returns the first matching record or zero when it reaches the end.
- `add_eight_to_routine_result`: Calls another routine and returns its result plus eight.
- `pack_byte_groups_into_words`: Combines each group of three source bytes into one 16-bit word using shifts and ORs, writes the words to the destination in order and returns the number of groups.
- `load_ilbm_bitmap_chunks`: Checks that a chunk is an ILBM form, then copies the bitmap header fields.
- `tiny_move`: Small routine of 1 instruction that copies (a0)+ to (a1)+.
- `copy_byte_block`: Copies a block of bytes from a source address to a destination address, using a count held in a register.
- `unpack_packbits_run_data`: Decompresses run-length data: a negative count repeats the following byte that many times, and a non-negative count copies that many literal bytes, until the output count is used up.
- `forward_to_iff_decoder`: Calls another routine and returns without other work, forwarding the caller's request to it.
- `write_ilbm_file_with_header`: Writes an IFF ILBM file header, then for each image word expands it into three bytes and writes them to the file, closing the file when done.
- `finish_iff_chunk_and_close`: Pads the written chunk to a four-byte boundary when needed, seeks back to the chunk size field, writes the final size and closes the file.
- `close_file_if_open`: Closes the file through the dos library when a flag is set, then calls another routine and returns.
- `write_bitplane_rows_with_offsets`: Computes row offsets for each bitplane from the image width and depth, then writes each plane's rows to the file one at a time, adding the offset of each row.
- `write_compressed_plane_rows`: Computes row offsets for each bitplane, encodes each row with the run-length routine, adds the encoded size to a running total and writes the encoded row to the file.
- `pack_bytes_run_length_encode`: Compresses a block with the PackBits run-length method: a run of at least three equal bytes becomes a negative count and the byte, and other bytes become literal runs with a positive count.
- `pack_bytes_run_length_alternate`: Second entry of the same run-length encoder.
- `write_packbits_literal_run`: Part of the run-length encoder that writes a block of literal bytes with a positive count, then checks for the next repeated run, and returns the output size so far.
- `count_repeated_bytes_to_limit`: Counts how many following bytes repeat the first byte, stopping at the end of the data or after 128 bytes, and leaves the count in a register.
- `find_current_task_record`: Looks up the current task, then walks a list of records and returns the first record whose task field matches it, or zero if none does.
- `clear_record_field_when_flagged`: Calls another routine, then clears a field in the record when a condition does not hold, and returns one.
- `store_nonzero_value_in_record`: Calls another routine, then stores the register value in a record field when that value is non-zero, and returns zero.
- `take_record_value_or_fail`: Calls another routine.
- `return_minus_one_failure`: Returns minus one to the caller, signalling failure, with no other work.
- `compute_bitmap_mode_flags`: Returns a mode flags word taken from a header, with an extra flag bit set when the stored depth is greater than five and a second condition holds.
- `add_depth_mode_flag`: Calls routines to obtain a mode flags word and sets an extra flag bit when the stored depth is greater than five, then returns the word.
- `compute_display_mode_flags`: Derives display flags from the width, depth and a second dimension in a header.

### 8

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### iff.library

- `check_iff_library_entry`: Library code that opens the DOS library, calls two of its routines, closes the library, and returns a fixed error code.
- `open_dos_and_init_list`: Library code that stores an argument in a record, opens the DOS library, and initializes an empty list head at a fixed offset in the record.
- `update_task_record_fields`: Library code that finds the current task's record and updates specific fields within it.
- `return_stack_argument`: Returns the first argument pushed on the stack, unchanged.
- `decrement_and_release_iff_node`: Library code that decrements a countdown field in a node and, when it reaches zero with a pending bit set, frees the memory and removes it from its list.
- `manage_record_lifecycle`: Library code that either sets a pending bit on a record if the countdown is non-zero, or removes the record and closes the associated library if the countdown is zero.
- `close_and_free_record`: Library code that closes a library, frees memory, and returns a value stored in a record.
- `return_void`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `process_iff_form`: Library code that identifies a FORM chunk and populates a record with its size and field data.
- `process_iff_form_record_2`: Library code that verifies a chunk type is FORM, allocates a record, and stores size and field values into that record.
- `allocate_iff_chunk_record_2`: Library code that allocates a memory block and populates it with a size, a chunk type, and two provided values.
- `create_iff_record`: Library code that allocates memory for a record, populates its fields with provided parameters, and calls another routine to read data into it.
- `allocate_iff_record`: Library code that populates a record's fields with provided parameters and calls another routine to read data into it.
- `store_record_fields`: Library code that stores two values into a record and then calls another routine.
- `forward_to_helper_and_return`: Library code that calls a single subroutine routine and returns without other work.
- `free_iff_record_if_present`: Library code that frees the memory of a record through the exec library when a pointer is present, then calls a subroutine.
- `find_chunk_by_tag_2`: Library code that walks a list of length-prefixed chunks following a header, comparing each chunk's tag with the requested value.
- `return_call_result_plus_eight_3`: Library code that calls a subroutine and returns its result with eight added to it.
- `unpack_triplet_bytes_to_words_2`: Library code that converts consecutive three-byte groups into 16-bit words, placing the first byte in the high bits and the upper nibble of the third byte in the low bits.
- `decode_iff_bitmap`: Library code that decodes bitmap rows and accumulates the data into a destination buffer.
- `accumulate_bitmap_rows`: Decodes the rows of image data through a decompression routine and adds the first word of each source row to every word of the destination row, repeating over the image rows.
- `accumulate_bitmap_rows_with_header`: Copies header fields from the source record, then decodes each image row through a decompression routine and adds the first word of the row to every word of the destination, until the data end is reached.
- `accumulate_bitmap_rows_from_entry`: Enters the row decoding loop partway through, decoding each image row through a decompression routine and adding the first word of the row to every destination word until the rows are finished.
- `accumulate_bitmap_rows_by_mode`: Checks a mode byte.
- `accumulate_bitmap_rows_if_flag`: When a flag byte equals one, decodes image rows and adds the first word of each row to every destination word.
- `accumulate_bitmap_rows_to_limit`: Decodes image rows through a decompression routine, adding the first word of each row to every destination word, until the position reaches a limit held in a register.
- `execute_conditional_store`: Calls another routine and stores the resulting register value into a record field only if that value is non-zero.
- `copy_byte_block_2`: Library code that copies a block of bytes from one memory location to another using a specified count.
- `copy_byte_increment`: Library code that copies a single byte from one memory location to another and increments both pointers.
- `unpack_packbits_data`: Library code that decompresses data using a PackBits-style algorithm, handling both repeated runs and literal sequences.
- `unpack_packbits_run_data_alternate`: Decompresses PackBits-style run-length data in a second form: a control byte with the high bit set repeats the next byte for the given run, and a control byte without it copies the following literal bytes, until the output count is used up.
- `call_single_routine_and_return_2`: Library code that calls one routine and returns without doing any other work.
- `write_iff_chunk_header`: Library code that writes an IFF chunk header and processes image data by converting bitplane information into a byte buffer.
- `close_iff_file_on_error`: Library code that attempts to finalize a file write and closes the file handle if certain conditions are met or errors occur.
- `close_iff_handle_if_open`: Library code that closes the DOS file handle when it is open, then calls a subroutine.
- `emit_weighted_row_sums_2`: Library code that computes eight weighted sums from a 16-bit header record and a multiplier, then writes each value with repeated DOS write calls and stops early if a call returns an error.
- `emit_weighted_sums_and_total_2`: Library code that computes eight weighted sums like its neighbour, but also accumulates a running total from a subroutine call and writes the values through the DOS library, stopping on the first error.
- `encode_packbits_alternating`: Library code that implements a PackBits compression loop, alternating between writing literal runs and repeated byte runs.
- `encode_packbits_literal_runs`: Library code that processes a data stream to identify and encode literal byte sequences into a compressed format.
- `perform_packbits_compression`: Library code that implements the PackBits run-length encoding algorithm to compress a block of data.
- `count_matching_byte_run_2`: Library code that counts how many consecutive bytes match the first byte, up to a maximum of one hundred and twenty-seven.
- `find_node_by_task_id_2`: Library code that retrieves the current task identifier and traverses a linked list to find a record with a matching identifier.
- `reset_field_and_return_one_2`: Library code that calls another routine and, if the zero flag is not set, clears a longword field in a record before returning one.
- `store_nonzero_field_value_2`: Library code that calls another routine and, if the input value is not zero, stores that value into a specific field of a record.
- `get_and_clear_record_field`: Library code that calls another routine to find a record, then either returns a longword from a field and clears it, or returns a failure value.
- `return_failure_constant`: Library code that returns a constant value of negative one.
- `extract_mode_flags_with_limit`: Library code that retrieves a value from a record and conditionally sets a bit if a byte at a specific offset exceeds five.
- `compute_size_flag_word`: Library code that calls another routine and conditionally sets a bit in the result if a byte in a record exceeds five, provided the zero flag is not set.
- `compute_size_flag_bits_2`: Library code that builds a bitmask based on whether a byte is less than five and a word is greater than a specific value, and whether another word exceeds a limit.

### diskfont.library

- `store_argument_and_register_object`: Calls a library routine to obtain a value.
- `store_argument_and_register_object_2`: Library code that obtains a value from one library routine.
- `open_dos_and_graphics_libraries`: Opens two libraries and, when both open, creates a structure with an empty list header and returns it.
- `release_unused_font_entries`: Walks a list of loaded font entries, removes each entry whose use count is not positive, then closes two libraries, frees the object and returns the stored value once no users remain.
- `close_and_free_when_unused`: When the use count is zero, closes two libraries, removes the object from its list, frees its memory and returns a stored value.
- `mark_object_pending_release`: Sets the high bit of a flag byte in the object to mark it as pending release, then returns zero.
- `return_immediately`: This library routine is empty and returns immediately.
- `clear_flag_and_increment_use`: Clears the pending-release bit in a flag byte of the object and increments its use count.
- `decrement_use_count_checking_pending`: Decrements the object's use count.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `score_font_descriptor_match`: Compares two font descriptors by scoring the difference in their size and the style bits set differently between them, using a table of seven weights.
- `load_named_font_file`: Copies a font name into a local buffer, then searches the FONTS directory for a matching font file using lock, examine and read library calls.
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
- `copy_null_terminated_string`: Copies bytes from a source string to a destination one at a time, including the terminating zero byte, until the zero byte has been copied.
- `append_null_terminated_string`: Advances to the zero byte that ends the destination string, then passes that position and the source argument to another routine, so the source text is appended after the existing text.
- `forward_to_library_handler`: Calls another routine and returns without doing other work, so it acts as a forwarding entry point for the library.
- `measure_null_terminated_string_length`: Counts the bytes before the zero terminator of a string and returns that count.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte, returning zero if they are identical, minus one if the first differing byte of the first string is smaller, and one otherwise.
- `compare_adjacent_bytes`: Compares the byte just before one address with the byte at another address, returning minus one when the first is smaller and one otherwise.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_in_string`: Scans a zero-terminated string for a given byte and returns its address, or zero if the terminating zero byte is reached first.
- `find_char_after_current_position`: Advances past the current position in a zero-terminated string and returns the address of the next byte matching the search byte, or zero if the terminating zero byte is reached first.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `find_last_char_in_string`: Measures a zero-terminated string, then scans it backward from the end for a given byte, returning the address of the last match or zero if there is none.
- `find_path_name_start`: Locates the last slash or colon in a path string and returns the address just after it, where the file name begins.
- `build_available_fonts_list`: Walks a list of font entries and writes a header and an array of records, each holding a name and style fields, into a caller's buffer.
- `append_loaded_font_records`: Walks the list of loaded font entries, copying each name and style fields into the next record of the caller's array and counting the records.
- `scan_font_directory_buffer`: Copies characters from a read buffer into a name buffer, and when a name is complete records its entry in the caller's array.
- `scan_fonts_directory_files`: Locks the FONTS directory and walks its entries with examine and next-entry calls.
- `read_font_file_entries`: Reads a font file's header, checks that its first word matches the expected font file signature, then reads each font record, storing names and sizes in the output list and counting them.
- `parse_font_file_entries`: Iterates through a file to extract font record metadata and character data into a structured table.
- `finish_font_file_read`: Reads the remaining font records from the file, closes it and releases its buffers.
- `release_font_scan_buffers`: Frees the scan buffer and unlocks the directory, then returns a positive error value when the count went below zero, otherwise zero.
- `return_zero`: Returns zero to the caller with no other work.
- `call_unknown_handler`: This library routine invokes another routine and then returns.
- `forward_to_external_handler`: Calls another routine through a function pointer and returns, with no other work.

### MegaBall (1991)(E. Mackey)(SW)[m ALE].adf:SHOW

- `load_executable_hunk_table`: Program entry code for the loader that allocates memory for each executable hunk, records the block addresses in a table, and checks each hunk against AmigaDOS hunk type codes.
- `return_without_work`: Empty stub that returns to the caller without doing any work.
- `busy_wait_loop`: Counts a register down to zero in a loop, using time without any other effect, and returns.
- `decode_back_reference_stream`: Decodes a compressed data stream into a buffer, where a control value selects between repeating a byte and copying earlier output.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_loop_with_space_wrap`: Decrements a byte held in a data register once for each step of a count, and replaces it with a space character when it wraps to zero.
- `stub_bad_instruction_data`: Decoder artefact: the bytes at this address do not decode as valid code, so no behaviour can be recovered from them.

### MegaBall (1991)(E. Mackey)(SW)[m ALE].adf:MEGABALL

- `wait_startup_message_and_reply`: Program entry code that finds its own task and, when the task has no process data, waits for a startup message on its port and takes it.
- `call_wait_t_o_f`: Wrapper that calls the graphics library function WaitTOF and returns its result to the caller.
- `wait_for_frames`: Waits for the given number of display frames, calling the graphics library's vertical-blank wait once per frame.
- `run_main_program_setup`: Main program routine that opens the diskfont, graphics and device resources, sets up the display and an interrupt server, shows a prompt telling the user to press 2 for two or SHIFT-Q to exit, and then runs its main loop.
- `advance_sprite_history_and_blit`: Clears three counters, shifts a set of stored values one slot back in a history buffer, copies memory, blits the bitmap, moves sprites and clears the raster.
- `run_speed_adjusted_frame_loop`: Clears a state value, lowers a speed counter by four with a floor of five, then runs the shared frame loop that calls the library for each frame.
- `advance_timers_and_redraw_frame`: Computes a timer value from a stored count, updates two sequence counters and swaps stored state values when a condition holds, then redraws the frame through library calls.
- `draw_cell_grid_and_update`: Draws a grid of cells by making pairs of library calls for each of three rows, then updates the display and the speed counters in the shared frame loop.
- `reset_counters_and_run_loop`: Increments a counter, clears four state values and a sequence counter, cancels a pending request when one exists, then runs the shared frame loop.
- `enter_frame_loop_mode_one`: Sets one state flag to one, then runs the shared frame loop that draws and updates the bitmaps each frame.
- `scale_speed_and_run_loop`: When a step counter is below two, increments it and doubles a speed value, cancelling a pending request first if one exists.
- `terminate_and_cleanup`: Sets several global flags to zero and calls a routine to release all opened devices, ports, and interrupt servers.
- `terminate_and_cleanup_2`: Sets several global flags to zero and calls a routine to release hardware devices, ports, and the interrupt server.
- `reset_global_state`: Resets several global variables to zero and calls a routine via a pointer stored in a global variable.
- `remap_cells_and_run_loop`: Remaps each cell code in a region to one of a small set of values, then runs the shared frame loop.
- `write_cells_and_run_loop`: Writes cell codes into a buffer, mapping each input value to one of a small set of codes, then runs the shared frame loop.
- `write_pattern_and_run_loop`: Writes a repeating cell pattern into a buffer, calling a library routine for each entry, then runs the shared frame loop.
- `write_cell_pattern_and_blit`: Writes a repeating cell pattern into a buffer and blits it through the graphics library, stepping through the buffer until its limit.
- `enter_frame_loop_mode_five`: Sets two state flags to one, then runs the shared frame loop that draws and updates the bitmaps each frame.
- `cycle_four_step_value`: Advances a four-step counter and selects the matching stored value for the next step, repeating while that value is zero, then runs the shared frame loop.
- `select_three_step_value`: Selects one of three stored values from a counter that advances on each pass, then runs the shared frame loop.
- `select_two_step_value`: Selects one of two or three stored values from a counter that advances on each pass, then runs the shared frame loop.
- `reset_step_value`: Resets a step counter, selects the first stored value, steps through the remaining stored values while they are zero, then runs the shared frame loop.
- `call_twice_then_run_loop`: Makes two calls to other routines, then runs the shared frame loop that draws and updates the bitmaps each frame.
- `halve_speed_and_run_loop`: When a step counter is not minus one, decrements it and halves a speed value, cancelling a pending request first if one exists.
- `enter_frame_loop_reverse_mode`: Sets a mode value to minus one, then runs the shared frame loop that draws and updates the bitmaps each frame.
- `release_devices_and_ports`: Shuts down the devices, ports and interrupt server the program opened: aborts and waits on pending I/O, removes the interrupt server and port, closes the device, and frees the signal and memory before returning.
- `draw_title_text_and_blit`: Draws a text string through the graphics library after setting the drawing mode and cursor position, then blits and clears the rasters around it.
- `drain_port_messages`: Takes and replies to every pending port message until none remain, then returns.
- `wait_for_flagged_message`: Waits for frame timing, then takes each port message, replies to it and records whether one of its flag bits is set, repeating until a flag is found.
- `update_bounce_timing_and_position`: Counts down a timer and, when it expires, reloads it from a speed value.
- `call_graphics_routine_once`: Calls one graphics library routine and returns, with no other work.
- `check_grid_cell_at_position`: Converts a position into a grid cell index and reads the cell value there.
- `clear_grid_cell_and_blit`: Converts a position into a grid cell.
- `format_decimal_digits`: Writes the digits of an unsigned number as decimal characters into a buffer, starting from the ten-thousands place and ending after the last digit, without a terminator.
- `write_decimal_digits_from_divisor`: Writes the digits of an unsigned number as decimal characters into a buffer, using a divisor held in a register that starts at ten thousand and is reduced by ten on each step.
- `check_level_end_show_score`: Draws a row of marks with the blitter, and when the computed value reaches the target it writes the score and board label text, then displays them and returns.
- `show_score_when_target_reached`: Updates a counter from a stored value on each pass, and when that value reaches the target it writes the score and board label text and displays it.
- `blit_cell_if_occupied`: Converts a position into a grid cell and blits that cell's image only when the cell holds a non-zero value.
- `forward_bitmap_blit_call`: Forwards a single argument-free call into the graphics library through its jump vector and returns the two result registers the library leaves behind.
- `blit_sprite_table_entries`: Walks a table of sprite records, clamping each record's horizontal position against a limit and passing its position, size and bitmap fields to the graphics library.
- `fade_palette_in_steps`: Builds thirteen palette levels by scaling each 4-bit red, green and blue value of a source palette by a rising factor and adding a fixed offset.
- `fade_palette_out_steps`: Steps a palette from full strength down to zero across thirteen levels by scaling each colour channel, loading each level and running two routines per step.
- `ramp_palette_brightness_steps`: Ramps a palette from black to full brightness across sixteen levels by scaling each 4-bit colour channel and loading each level.
- `fade_palette_from_full_brightness`: Fades a palette from full brightness down to black over sixteen levels, loading each level into the display and waiting for the frame before the next step.
- `abort_io_until_cleared`: Repeatedly calls the exec library's abort-I/O entry on a stored request block until it returns zero, cancelling any outstanding transfers before the routine returns.
- `wait_io_until_complete`: Repeatedly calls the exec library's wait-for-I/O entry on a stored request block until it returns zero, so the routine blocks until the transfer finishes.
- `handle_board_file_selection`: Ignores input while a flag is set; otherwise it either clears the record table or selects one record by index, sets its state fields and calls library routines.
- `open_dos_library`: Opens the DOS library through the exec library's open entry and stores the returned base for later calls.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `open_intuition_library`: Opens the intuition library through the exec library's open entry and stores the returned base in a global slot.
- `close_library_via_exec`: Closes a previously opened library through the exec library's close entry, using the library base held in a global.
- `open_graphics_library`: Opens the graphics library through the exec library's open entry and stores its base in a global slot for later graphics calls.
- `call_library_function`: Calls a function through a pointer stored at an offset from a global variable.
- `open_diskfont_library`: Opens the diskfont library through the exec library's open entry and stores its base in a global slot.
- `call_library_function_2`: Executes a routine by dereferencing a pointer located within a global variable.
- `open_iff_library_checked`: Opens the iff library and stores its base; if the open fails, it points a data slot at a fixed data area instead.
- `call_library_function_3`: Performs an indirect call to a function address retrieved from a global variable.
- `load_music_by_available_memory`: Compares the free memory reported by the system against a set of size thresholds, and when enough is available it allocates a buffer, opens the music file and reads it in.
- `copy_block_then_load_scores`: Copies a block of longwords up to a fixed end address, then opens the scores file, reads it and calls two library routines when the read succeeds.
- `write_scores_file`: Opens the scores file, writes a block through the DOS library and closes it, calling the write and close entries only when the open succeeds.
- `load_ball_iff_colour_table`: Loads the colour table of the ball image through the iff library and, if the call fails, points a data slot at a fixed area and reports the error.
- `stub_no_operation_return`: Does nothing and returns; the routine is an empty stub reached from two call sites.
- `decode_ball_iff_picture`: Decodes an IFF picture through the iff library and, when decoding fails, points a data slot at a fixed area and reports the failure.
- `open_custom_screen_checked`: Opens a custom intuition screen and stores the result, and on failure points a data slot at a fixed area instead.
- `call_close_screen`: Wrapper that calls the intuition library function CloseScreen and returns its result to the caller.
- `open_intuition_window`: Opens an intuition window and stores its handle in a global slot for later drawing calls.
- `open_main_window_checked`: Opens the main intuition window, storing the handle and redirecting a data slot to a fixed area when the open fails.
- `call_close_window`: Wrapper that calls the intuition library function CloseWindow and returns its result to the caller.
- `generate_random_quotient_remainder`: Advances a 16-bit pseudo-random state by multiplying and mixing it with fixed constants and a call counter, then returns the scrambled value split into a quotient and remainder for a divisor.
- `call_routines_when_flagged`: Calls two routines only when a flag is set, and otherwise returns without doing anything.
- `check_flag_and_call`: Checks a global flag and calls another routine if the flag is non-zero.
- `call_routine_return_zero`: Calls a single routine and returns zero.
- `forward_call_and_return_result`: Calls one routine and passes through the two result registers that routine leaves.
- `initialise_sequence_playback`: Scans the sequence order table for the next entry, builds pattern pointer records for that sequence, clears the playback counters and marks playback as active.
- `reset_channels_and_start_sequence`: Clears the four channel levels, scans the sequence order table to build pattern records, and sets the playback flags.
- `silence_audio_channels_when_flagged`: When a playback flag is set, it clears the flag and silences the four audio channels by zeroing their volumes and clearing the audio DMA bits.
- `advance_playback_and_reload_audio`: Steps the playback position through the order table with wrap-around at 128, jumping back to the loop start at the end marker.
- `decode_pattern_cell_effects`: Decodes one four-byte pattern cell, storing the sample pointer, length and period for its channel, then applies a volume, speed or effect command from the cell's low nibble.
- `copy_record_word_to_target`: Copies a 16-bit field from a state record into a target location passed by pointer.
- `scale_channel_volume_by_master`: Multiplies a channel's sample volume by the master volume and stores the scaled result in the channel record.
- `update_countdown_channel_flag`: Masks a 12-bit field of a state record and compares it with a stored target, clearing the counter on equality and setting a flag when the value is below the target.
- `lookup_table_value_by_threshold`: Selects one of three fields by a global counter taken modulo three, then scans a table of 36 entries for the first value not below the record's threshold and stores the match in the record.

### MegaBall (1991)(E. Mackey)(SW)[m ALE].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `boot_reset_and_jump`: Boot entry point that calls the reset routine and then jumps to the program's main code address held in a global.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `null_routine`: This routine is empty and contains no executable instructions.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `setup_task_node_and_loop`: The decompiled body is only an endless loop, but the library calls it makes show that the entry finds the current task, allocates memory and adds a node to a list before looping.
- `initialize_node_structure`: Sets up a node structure with initial values and calls a library function to allocate memory, storing the result at offset 40 of the structure.
- `decrement_and_release_resources`: Decrements a countdown field in a record and, when the field reaches zero with a pending bit set, runs the release routine again.
- `release_node_or_mark_pending`: Releases the node through the exec library and returns its stored result when the countdown field is zero.
- `free_node_and_return_result`: Removes a node from its list, closes its associated library, frees its memory through the exec library, and returns the result stored in the record.
- `parse_iff_header`: Reads and validates an IFF file header, allocates memory for the file structure, and sets up the file record with specific identifiers when the format is correct.
- `open_file_and_check_format`: Attempts to open a file using a DOS library call and checks if the operation succeeded, setting a flag to indicate success or failure.
- `read_iff_form_chunk_header`: Library code that reads a chunk header from the open IFF file through the DOS library.
- `allocate_iff_chunk_record_3`: Library code that allocates a record for a chunk, stores the supplied tag and sizes in it, and returns a numbered error code if the allocation fails.
- `write_chunk_tag_and_params`: Part of the IFF library: stores a tag value at the start of a buffer and writes it out through the file write call.
- `store_fields_and_close_file_2`: Stores three values into a record at the address held in an address register, then closes the open file through a DOS call.
- `forward_to_helper_and_return_2`: Calls a subroutine to reset a field and then returns without performing any additional operations.
- `free_iff_record_if_present_2`: Frees memory allocated for an IFF record through the exec library when a pointer is provided, then calls a subroutine to reset a field.
- `find_chunk_by_tag`: Walks a list of length-prefixed chunks that follow a header, comparing each chunk's tag with the requested value.
- `return_call_result_plus_eight`: Calls a subroutine and returns its result with eight added to it.
- `copy_block_bytes_by_count`: Copies bytes one at a time from the source to the destination until the count held in the data register expires.
- `copy_block_bytes_checked_first`: Copies bytes from the source to the destination, testing the count before each byte, until the counter wraps past zero.
- `unpack_packbits_run_stream`: Decodes a PackBits-style run-length stream: a negative control byte repeats the next byte, while a non-negative control byte copies that many following bytes literally, until the output count is used up.
- `unpack_rle_repeat_and_literal`: Decodes a run-length stream in which each control byte either repeats a byte a set number of times or copies literal bytes, writing output until the count reaches zero.
- `call_single_routine_and_return`: Calls one routine and returns without doing any other work.
- `write_ilbm_picture_file`: Writes an IFF ILBM picture: it emits the form, picture-type and header chunk tags, converts each source word into three bytes of packed data, and writes the result through the DOS library.
- `finish_file_write_and_close`: Checks the result of the preceding write, calls the file write sequence through the DOS library on success, and closes the file.
- `close_file_if_flagged`: Closes the open file through the DOS library when a flag is set, then calls a subroutine.
- `emit_weighted_row_sums`: Computes eight weighted sums from a 16-bit header record and a multiplier, then writes each value with repeated DOS write calls and stops early if a call returns an error.
- `emit_weighted_sums_and_total`: Computes eight weighted sums like its neighbour, but also accumulates a running total from a routine call and writes the values through the DOS library, stopping on the first error.
- `pack_bytes_packbits_stream`: Packs a source buffer into a PackBits-style stream by choosing repeat or literal runs, writing a control byte before each run and counting the bytes written.
- `pack_literal_bytes_packbits`: Writes a literal run of bytes into a PackBits-style output, emitting a count byte and copying the bytes until a repeat is detected or the limit is reached.
- `emit_packbits_runs`: Walks the source and writes either a repeated-byte run or a literal run into the output with the matching control byte, returning the total count after each run.
- `count_matching_byte_run`: Counts how many consecutive bytes equal the first byte, stopping at the end of the buffer or at a run length of 128.
- `find_node_by_task_id`: Fetches the current task identifier through the exec library and walks a linked list of records, returning the first record whose stored identifier matches, or null if none does.
- `reset_field_and_return_one`: Clears a field in the target record when the test flag is not set, then returns one.
- `store_nonzero_field_value`: Stores the incoming value in a field of the target record when that value is nonzero, then returns zero.
- `get_and_clear_field`: This library code calls another routine to find a record, then either returns a longword from that record and clears it, or returns a failure value.
- `set_failure_flag_and_branch`: Sets a return value to negative one and branches unconditionally to another routine at a fixed offset.
- `read_flags_field_from_helper`: Part of the IFF library: calls one routine and returns the longword at offset eight of the structure it gives back.
- `compute_size_flag_word_2`: Returns a flag word derived from a subroutine's result, adding a bit when a size byte exceeds a limit and skipping that test when the zero flag is set.
- `compute_size_flag_bits`: Builds a flag word from a picture record: one bit is set when the first field is small and the second value is large, and a second bit is set when another value exceeds a limit.
- `load_ilbm_bitmap_header`: Checks the form tag for an ILBM picture and, when the bitmap header matches the expected depth, copies the header fields into a local record and passes them to the decoder.
- `unpack_triplet_bytes_to_words`: Converts consecutive three-byte groups into 16-bit words, placing the first byte in the high bits and the upper nibble of the third byte in the low bits.
- `return_call_result_plus_eight_2`: Calls a subroutine and returns its result with eight added to it.
- `jump_to_find_chunk_by_tag`: Jump stub: passes control straight on to find_chunk_by_tag.
- `jump_to_unknown_location`: Jumps to a fixed memory address without any other operations, likely acting as a placeholder or redirect.
- `jump_to_alternate_parse_routine`: Jumps to another fixed memory address, likely serving as an alternate code path for parsing IFF headers.
- `jump_to_free_node_and_return_result`: Jump stub: passes control straight on to free_node_and_return_result.
- `jump_to_release_node_or_mark_pending`: Jump stub: passes control straight on to release_node_or_mark_pending.
- `jump_to_decrement_and_release_resource`: Jump stub: passes control straight on to decrement_and_release_resources.
- `allocate_task_record_node`: When the lock flag is set and memory is allocated, it stores the current task identifier in the new node, clears its counter and links it into a list.
