# Game 2183 (megaball (1991)(e. mackey)(sw)[a4]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02183). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903820 top-level bytes in 17 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 260 of 260 named (0 author symbols), 260 of 260 described (30 by rule, 66 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2825 of 2825 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6371 of 6371 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 392196 |
| code | 24726 |
| data | 130437 |
| padding | 3693 |
| unused | 352768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| MegaBall (1991)(E. Mackey)(SW)[a4].adf:iff.library | 3 | 673 | 40 |
| 8 | 1 | 2 | 1 |
| iff.library | 3 | 673 | 49 |
| diskfont.library | 13 | 830 | 53 |
| MegaBall (1991)(E. Mackey)(SW)[a4].adf:MEGABALL | 3 | 3636 | 85 |
| MegaBall (1991)(E. Mackey)(SW)[a4].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 547 | 31 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### MegaBall (1991)(E. Mackey)(SW)[a4].adf:iff.library

- `open_use_and_close_library`: Opens a system library by name, calls two of its functions, closes it again and returns the value five.
- `setup_iff_list_structure`: Stores a supplied pointer in a structure, obtains a value from a library call and initializes an empty linked list header inside the structure.
- `allocate_and_queue_iff_entry`: Allocates a memory block, records the calling task in it and links it onto a list, then increments a count held in the structure.
- `release_iff_entry_if_unused`: When a condition flag is clear, removes a node from its list and frees its memory block.
- `free_entry_or_mark_pending`: If a count held in the structure is zero, it removes the node, closes its library, frees the memory and returns a stored value.
- `close_free_and_remove_entry`: Removes a node from its list, closes a library, frees its memory block and returns a value stored in the structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `open_and_verify_iff_form`: Opens a file through a DOS call and reads its first four bytes to check for the IFF FORM identifier.
- `open_iff_file_verify_form`: Opens a file through a DOS call, reads the first four bytes and checks for the IFF FORM identifier, returning an error code when the open, allocation or check fails.
- `read_iff_form_chunk_header`: Reads the first bytes of an IFF chunk from an open file and checks for the FORM identifier.
- `allocate_iff_chunk_record`: Allocates a memory block sized to hold a header plus the chunk data, fills in its size and type fields and reads the chunk data from the open file.
- `read_chunk_into_record`: Stores a value in the first field of a record, reads file data into the following field and stores two parameters after that.
- `store_fields_and_close_file`: Stores three values into a record at the address held in an address register, then closes the open file through a DOS call.
- `forward_call_unchanged`: Calls another routine and returns without adding any work.
- `free_block_if_present`: Frees the memory block passed in an address register when that pointer is non-zero, then calls another routine and returns.
- `search_chunk_list_by_id`: Walks a list of length-prefixed records inside a buffer, stepping over each record's size, and returns the record whose identifier matches the value in a data register.
- `return_result_plus_eight`: Calls another routine and returns its result increased by eight.
- `decode_three_byte_groups`: Reads a packed sequence of three-byte groups from a chunk located by an earlier lookup and writes one 16-bit value per group into the destination.
- `decode_ilbm_bitplane_rows`: Checks for the ILBM form type, reads the bitmap header fields and processes the image rows plane by plane, accumulating values per row.
- `tiny_move`: Small routine of 1 instruction that copies (a0)+ to (a1)+.
- `copy_bytes_counted`: Copies bytes from a source address to a destination address, with the count held in a data register and the copy running one byte past that count.
- `unpack_packbits_run_stream`: Decompresses a PackBits-style run-length stream, where a non-negative control byte copies that many following bytes literally and a negative one repeats the next byte.
- `pass_call_to_next_routine`: Calls another routine and returns without any other work.
- `write_iff_ilbm_file`: Opens a file and writes an IFF ILBM image, starting with the FORM and bitmap header chunks, then the converted pixel data and a BODY chunk.
- `finish_iff_chunk_write`: Pads a written chunk to a four-byte boundary when needed, then seeks and writes the padding and closes the file once the writes succeed.
- `close_file_if_open`: Closes the DOS file whose handle is held in a data register when that handle is valid, then returns.
- `write_rows_accumulating_totals`: Computes eight accumulated values from the header words, then writes data rows to a file one at a time while adding those values to each row.
- `write_encoded_rows_totals`: Writes rows to a file through a DOS call, encoding each row with a separate routine and summing the encoded lengths.
- `encode_byterun1_row`: Compresses a row into PackBits-style records, emitting a literal block or a repeat record followed by the data byte, and returns the number of bytes produced.
- `encode_byterun1_remainder`: Continues the same PackBits-style run encoding for the remaining bytes of a row, emitting literal or repeat records and returning the number of bytes produced.
- `encode_byterun1_literal_blocks`: Encodes the remaining bytes of a row as PackBits-style literal blocks and repeat records, returning the number of bytes produced.
- `measure_repeat_run_length`: Tests how many following bytes match the first byte, stopping at a mismatch, the end of the data or the run limit of 128.
- `find_list_entry_for_task`: Finds the current task's record in a list held in the structure by comparing each node's task field with the task identifier.
- `clear_result_field_if_unset`: Clears a 32-bit field of the structure when a condition flag is not set, then always returns one.
- `store_nonzero_result_field`: Stores the value in a data register into a structure field when that value is non-zero, then returns zero.
- `take_result_field_value`: Returns the value stored in a structure field and clears the field, or returns minus one when the preceding check fails.
- `return_minus_one_stub`: Returns minus one without doing any other work.
- `read_chunk_info_flags`: Returns a 32-bit value read from a fixed offset of a record returned by an earlier call.
- `return_value_with_flag_bit`: Calls another routine to get a value and, when a condition flag is clear and a byte in the structure exceeds five, sets an extra flag bit in that value before returning it.
- `build_threshold_flag_word`: Builds a flag word from two 16-bit values and a byte in a structure, setting one bit when the first value exceeds a threshold while a byte condition holds, and another bit when the second value exceeds its limit.

### 8

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### iff.library

- `run_iff_library_entry_check`: Library code that opens the DOS library, calls two of its routines, closes it again and returns a fixed error code of five.
- `open_dos_and_init_list`: Library code that stores its argument in a record, opens the DOS library through the exec library and saves the returned base, then sets up an empty list head at a fixed offset in the record.
- `update_task_record_status`: Library code that finds the current task's record in a list, updates a field in that record using a value from another routine, and modifies two fields in a structure.
- `return_first_argument`: Returns the value passed to it on the stack without changing it.
- `decrement_and_release_iff_node`: Library code that decrements a countdown field in a node and, when the field reaches zero with a pending bit set, frees the node's memory and removes it from its list through the exec library.
- `release_iff_entry_if_unused_2`: Library code that either removes a node and closes its library and frees its memory if a countdown is zero, or sets a pending bit in a structure if the countdown is non-zero.
- `close_free_and_remove_entry_2`: Library code that closes a library, frees a memory block, and removes a node from its list.
- `return_without_action`: Returns immediately without doing any work.
- `open_iff_file_verify_form_2`: Library code that opens a file, checks for the IFF FORM identifier, and if valid, stores the file size and identifier into a record before closing the file.
- `read_iff_form_chunk_header_2`: Library code that reads a file, verifies the IFF FORM identifier, and stores the resulting size and identifier into a record.
- `allocate_iff_chunk_record_2`: Library code that reads a file, checks for the IFF FORM identifier, and stores the size and identifier into a record using provided parameters.
- `read_chunk_into_record_2`: Library code that stores a calculated size in a record, reads data from a file into the record, and stores two parameters into the record.
- `read_and_store_iff_record`: Library code that reads a file using a DOS call, stores three values into a record at a specific address, and then closes the file.
- `store_record_and_close_file`: Library code that stores two values into a structure and then calls another routine to close a file.
- `forward_to_helper_and_return`: Library code that calls a single subroutine and returns without other work.
- `free_iff_record_if_present`: Library code that frees the memory of a record through the exec library when a pointer is present, then calls a subroutine.
- `find_chunk_by_tag`: Library code that walks a list of length-prefixed chunks following a header, comparing each chunk's tag with the requested value.
- `return_call_result_plus_eight`: Library code that calls a subroutine and returns its result with eight added to it.
- `unpack_triplet_bytes_to_words`: Library code that converts consecutive three-byte groups into 16-bit words, placing the first byte in the high bits and the upper nibble of the third byte in the low bits.
- `parse_ilbm_chunk_data`: Checks for the ILBM identifier and validates chunk size before iterating through data to accumulate values into a stack-allocated array.
- `run_ilbm_row_accumulation`: Continues the ILBM image decoding for a set of rows without repeating the form-type check, accumulating values per row and calling other routines to copy the decoded data.
- `decode_ilbm_rows_from_header`: Runs the ILBM row decoding using header fields already read into the structure, calling other routines for each row and accumulating values.
- `run_ilbm_plane_rows_step`: Runs the ILBM row accumulation for one plane, calling other routines that copy decoded data, and jumps to a cleanup routine when a condition flag is set.
- `finish_ilbm_plane_rows`: Runs the ILBM row accumulation for one plane using the row count from a data register, calls other routines to copy data, then runs a cleanup routine.
- `run_ilbm_plane_when_flagged`: Runs the ILBM plane row loop only when a data-register flag equals one, calling other routines for each row.
- `run_ilbm_plane_rows_loop`: Runs the ILBM plane row loop unconditionally, calling other routines for each row and accumulating values, then calls a cleanup routine.
- `store_nonzero_register_field`: Checks if a value in a data register is non-zero and, if so, writes it to a specific field within a structure.
- `copy_bytes_loop`: Library code that copies a specified number of bytes from one memory location to another using a loop.
- `decrement_and_copy_byte`: Decrements a counter in a data register and, while the counter is non-zero, copies a byte from one memory location to another.
- `unpack_packbits_stream_2`: Library code that decompresses a PackBits-style stream by either repeating a byte a specified number of times or copying a literal block of bytes.
- `unpack_packbits_fill_stream`: Decompresses a PackBits-style stream in which each repeat record fills the output with its repeated byte, using a count held in a data register.
- `call_single_routine_and_return`: Library code that calls one routine and returns without doing any other work.
- `write_iff_header_and_body`: Library code that constructs IFF chunk headers, processes bitmap data, and writes the resulting chunks to a file.
- `finalize_iff_file_write`: Library code that manages the closing of a file and the cleanup of a structure after a write operation.
- `close_iff_handle_if_open`: Library code that closes the DOS file handle when it is open, then calls a subroutine.
- `emit_weighted_row_sums`: Library code that computes eight weighted sums from a 16-bit header record and a multiplier, then writes each value with repeated DOS write calls and stops early if a call returns an error.
- `emit_weighted_sums_and_total`: Library code that computes eight weighted sums like its neighbour, but also accumulates a running total from a subroutine call and writes the values through the DOS library, stopping on the first error.
- `encode_packbits_data`: Library code that encodes a buffer into PackBits-style literal and repeat records, returning the number of bytes written.
- `encode_packbits_literal_blocks`: Library code that encodes a sequence of bytes into PackBits-style literal blocks and repeat records.
- `encode_packbits_row`: Library code that compresses a data row into PackBits-style records by alternating between literal blocks and repeat runs.
- `count_matching_byte_run`: Library code that counts how many consecutive bytes equal the first byte, stopping at the end of the buffer or at a run length of 128, to decide whether a run should be packed.
- `find_node_by_task_id`: Library code that fetches the current task identifier through the exec library and walks a linked list of records, returning the first record whose stored identifier matches, or null if none does.
- `reset_field_and_return_one`: Library code that clears a field in the target record when the test flag is not set, then returns one.
- `store_nonzero_field_value`: Library code that stores the incoming value in a field of the target record when that value is nonzero, then returns zero.
- `get_and_clear_field`: Library code that retrieves a value from a field in a target record and clears that field, or returns minus one if the target record is not found.
- `return_minus_one`: Library code that returns minus one.
- `get_field_with_flag`: Library code that retrieves a value from a record and sets an extra bit in the result if a specific byte in the record exceeds five.
- `compute_size_flag_word`: Library code that returns a flag word derived from a subroutine's result, adding a bit when a size byte exceeds a limit and skipping that test when the zero flag is set.
- `compute_size_flag_bits`: Library code that builds a flag word from a picture record: one bit is set when the first field is small and the second value is large, and a second bit is set when another value exceeds a limit.

### diskfont.library

- `store_argument_in_font_handle`: Requests a handle from a library call and, when one is returned, writes the argument into a field at a fixed offset of the handle and calls a second library routine.
- `check_and_set_font_handle`: Library code that calls a routine via a table entry using the library base.
- `open_two_libraries_init_list`: Opens two system libraries and, when both succeed, initializes an empty list header inside the structure and returns its address.
- `remove_unused_entries_and_close`: Walks a list and unlinks each entry whose counter is not positive, calling a release routine for each one.
- `close_and_free_when_idle`: Closes two libraries, frees the structure and unlinks it from its list when no flag is set and the counter is not positive.
- `mark_structure_pending_close`: Sets the pending bit in a structure's flag byte and returns zero.
- `return_void`: This routine is empty and performs no operations before returning.
- `clear_flag_and_increment_counter`: Clears the pending bit in a structure's flag byte and increments a 16-bit counter stored in the same structure.
- `decrement_counter_and_dispatch`: Decrements a 16-bit counter in the structure and, if it is still non-zero and the pending bit is set, calls a library routine through a table entry.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_font_match_offset`: Computes a signed score by comparing two 16-bit size values and then walking the bits of two flag bytes, subtracting a lookup-table entry for each bit that differs.
- `scan_fonts_directory_best_match`: Scans the FONTS: directory through DOS calls and selects the font entry with the highest computed score, updating counters and flags in the matching list entries.
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
- `copy_null_terminated_string`: Copies bytes from a source string to a destination up to and including the zero terminator.
- `find_string_end_and_call`: Walks a pointer forward until it reaches the zero terminator of a string, then calls another routine with that end position and the second argument.
- `pass_call_to_next_routine_2`: Calls another routine and returns without any other work.
- `count_string_length`: Counts the bytes before the zero terminator of the string it is given and returns that count.
- `compare_strings_by_byte_order`: Compares two zero-terminated strings using pointers held in address registers, stepping both together.
- `compare_byte_against_previous`: Compares the byte at one pointer with the byte just before a second pointer and returns minus one or one depending on their order.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_character_in_string`: Scans a zero-terminated string for a given byte and returns a pointer to its first occurrence.
- `scan_string_after_first_byte`: Steps past the current byte of a string before testing, then walks forward until a byte equals a value held in a data register, returning that position.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `find_last_character_in_string`: Measures a zero-terminated string and then scans backward from its final byte for a given character.
- `locate_path_separator_pointer`: Looks for a slash in a name string and, if there is none, a colon, and returns a pointer to the separator found.
- `enumerate_font_entries_into_buffer`: Fills a buffer with font entry records, first from the library's internal font list and then from the FONTS: directory when the flags request it.
- `append_font_list_entries`: Appends one font record for each entry of the internal font list into a buffer, then scans the FONTS: directory for further entries when a flag bit is set.
- `copy_name_entries_into_buffer`: Copies the names of the internal font list into a buffer as records, and when a flag bit is set also scans the FONTS: directory.
- `scan_fonts_directory_entries`: Scans the FONTS: directory through DOS calls and writes a record for each font file it accepts into a buffer, returning the space remaining.
- `record_font_directory_entries`: Continues the FONTS: directory scan, writing a record for each matching font file found and returning the space remaining in the buffer.
- `read_font_directory_entries`: Reads the FONTS: directory listing through a DOS read call, writing a record for each font file it accepts, and returns the remaining buffer space.
- `write_font_records_from_listing`: Writes font records for matching entries of the FONTS: directory listing, returning the space remaining in the buffer.
- `release_directory_lock_and_report`: Restores the previous current directory and releases the directory lock through DOS calls, then returns the space remaining, negated when the buffer ran out.
- `return_zero_constant`: Returns zero without doing any other work or accessing memory.
- `call_handler_and_return`: Calls another routine and then returns without changing anything else.
- `call_external_handler`: Calls another routine and returns, so it only forwards the call to an external handler.

### MegaBall (1991)(E. Mackey)(SW)[a4].adf:MEGABALL

- `wait_for_task_message`: Looks up the current task and, if its message port is empty, waits for a message and fetches it.
- `call_wait_t_o_f`: Wrapper that calls the graphics library function WaitTOF and returns its result to the caller.
- `wait_for_frames_count`: Waits for display frames by calling the frame-wait routine in a loop, running for the count held in a data register plus one.
- `start_megaball_main_program`: Startup routine that checks for the required fonts, libraries and devices named in its text, opens the audio and timer devices, loads the IFF images and music, and shows a prompt for the player count and the quit key before entering its main loop.
- `enter_handler_stub`: A six-instruction entry stub with no library calls of its own, reached through a handler pointer and an address reference.
- `reply_pending_port_message`: Replies to a message from an exec port and makes four calls to other routines around that reply.
- `service_messages_and_blit`: Retrieves messages from ports, copies memory blocks, blits bitmaps, moves sprites and clears a raster area, calling thirteen other routines.
- `call_program_handler_stub`: A six-instruction stub that makes one call to another routine.
- `pass_call_to_next_routine_3`: Calls another routine and returns without any other work.
- `call_two_routines_in_turn`: Makes two calls to other routines in three instructions and returns.
- `run_three_calls_then_return`: Makes three calls to other routines within its instructions and then returns.
- `validate_hunk_type`: Checks a value against AmigaDOS hunk type codes to determine if it represents executable or relocation code.
- `call_two_routines_then_return`: Makes two calls to other routines and returns.
- `validate_hunk_type_2`: Checks a value against AmigaDOS hunk type codes to determine if a memory block is an executable or relocation hunk.
- `validate_hunk_type_3`: Compares a value against AmigaDOS hunk type codes to determine if a memory segment is executable or relocation code.
- `validate_hunk_type_4`: Checks a value against AmigaDOS hunk type codes to determine if the memory segment is an executable or relocation hunk.
- `validate_hunk_type_and_blit`: Checks a value against AmigaDOS hunk type codes to validate data integrity before performing a bitmap region blit operation.
- `verify_hunk_and_blit`: Validates AmigaDOS hunk type codes and performs a bitmap region blit operation.
- `validate_hunk_type_5`: Compares a value against AmigaDOS hunk type codes to verify executable loading or relocation data.
- `blit_bitmap_region`: Calls the graphics library's bitmap blit function and two other routines within its 25 instructions.
- `validate_hunk_type_6`: Compares a value against AmigaDOS hunk type codes to verify if a memory block is an executable or relocation hunk.
- `validate_amiga_hunk_type`: Compares a value against specific AmigaDOS hunk type codes to identify executable loading or relocation data.
- `validate_hunk_type_10`: Checks if a value matches specific AmigaDOS hunk type codes for executable loading or relocation data.
- `validate_hunk_type_9`: Checks a value against AmigaDOS hunk type codes to identify executable loading or relocation data.
- `validate_hunk_type_7`: Compares a value against AmigaDOS hunk type codes to identify executable loading or relocation data.
- `validate_hunk_type_8`: Compares a value against AmigaDOS hunk type codes to identify executable loading or relocation data.
- `parse_amiga_hunk_headers`: Iterates through a memory structure to identify and process AmigaDOS hunk types.
- `return_without_action_2`: Returns immediately without doing any work.
- `shut_down_devices_and_ports`: Aborts and waits for pending device requests, closes devices, removes interrupt servers and ports, frees signals and memory, and replies to messages.
- `draw_label_and_move_sprites`: Draws a short text label with the graphics library, moves sprites and clears raster areas, using a drawing mode set beforehand.
- `process_message_loop`: Repeatedly calls a function through a pointer stored in a global variable until that function returns zero.
- `drain_messages_and_move_sprite`: Waits for the frame, replies to port messages until none remain and moves a sprite.
- `compute_position_and_step_values`: Updates position and step values held in global words, comparing them against limits and calling three other routines to fetch values.
- `blit_and_move_sprite_step`: Replies to a port message, blits a bitmap and moves a sprite, calling three other routines.
- `adjust_coordinate_within_bounds`: Adds three to a coordinate held in a data register when a flag is set, and returns the original value when the other coordinate lies outside the range from eight to 154.
- `lookup_cell_from_coordinates`: Checks a coordinate against the bounds from eight to 154, converts two coordinates into cell indices by division, reads a byte from a computed table offset and calls four other routines.
- `start_decimal_text_conversion`: Sets the divisor for a decimal conversion and passes control to the digit loop that writes the number as text characters.
- `convert_number_to_decimal_text`: Converts a number into decimal characters by repeated division by a divisor and writes each digit as an ASCII character into a text buffer.
- `blit_sprite_rows_loop`: Loops over a row counter, blitting a bitmap block at fixed positions and calling one other routine.
- `draw_score_and_board_labels`: Builds the text labels for a score and a board display in memory, then moves the drawing position and draws them with the graphics library.
- `blit_grid_cell_if_valid`: Checks a value by subtracting sixteen, dividing by seven and testing that the division is exact and below twenty-one, and blits a cell when the test passes.
- `blit_after_frame_wait`: Blits a bitmap through the graphics library in a wrapper that waits for the frame first.
- `move_sprite_table_entries`: Walks a table of sprite entries, clamping each position to the 320-wide display area and moving the sprite with the graphics library.
- `fade_palette_entries_step`: Scales the 4-bit red, green and blue components of a palette table by a step value divided by fifteen, adds a fixed offset and stores the results in a second table.
- `fade_palette_entries_partial`: Performs the same palette scaling as its neighbour with a fixed step of twelve out of fifteen, storing the adjusted colour values in the second table.
- `scale_palette_entries_by_step`: Scales each 4-bit colour component of a palette table by the step value divided by fifteen and stores the results in a second table, without adding an offset.
- `load_palette_at_full_level`: Scales palette entries to full level and loads them into the display with the graphics library's palette call, waiting for the frame first.
- `abort_pending_io_requests`: Repeatedly calls a routine through the library table, which aborts pending device requests, until it reports zero.
- `wait_for_pending_io`: Repeatedly calls a routine through the library table, which waits for a device request to finish, until it reports zero.
- `prompt_and_load_board_file`: Prompts for a board file name and opens and reads it through DOS calls, reporting 'File not found' and asking for a full path when the open fails.
- `open_dos_library`: Opens dos.library through the exec library and stores the returned base pointer in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `open_intuition_library`: Opens intuition.library through the exec library and stores the returned base pointer in a global variable.
- `close_library_base`: Closes a library through the exec library call, using a stored base pointer, and returns.
- `open_graphics_library`: Opens graphics.library through the exec open-library call and stores the returned library base in a global.
- `close_library_from_global`: Closes the library whose base is held in the shared global block, using the exec close-library call.
- `open_diskfont_library`: Opens diskfont.library through the exec open-library call and stores the returned library base in a global.
- `close_library_indirect`: Calls a library function through a pointer stored at an offset from a global variable.
- `open_iff_library`: Opens iff.library through the exec open-library call and stores the result in a global.
- `call_library_function`: Fetches a function pointer from a global variable and executes it.
- `load_music_file_into_memory`: Reads the free memory reported by the system, sizes a buffer in tiers from that figure, and allocates it through exec.
- `load_scores_file`: Opens the MegaBall.scores file, copies its contents word by word into a fixed global area up to a set end address, and closes the file.
- `save_scores_file`: Opens the MegaBall.scores file for writing, writes the stored score data to it, and closes the file again.
- `load_ball2_iff_picture`: Loads the BALL2.IFF picture through iff.library and records its result in a global.
- `stub_empty_return`: A single return instruction with no work, reached from two callers.
- `decode_iff_picture`: Calls iff.library to decode a loaded picture.
- `open_custom_screen`: Attempts to open a custom screen through an indirect intuition call and stores a pointer to a global variable if the call fails.
- `call_close_screen`: Wrapper that calls the intuition library function CloseScreen and returns its result to the caller.
- `open_main_window`: Opens a window through an indirect intuition call and stores the returned window handle in a global word.
- `open_main_window_with_error`: Attempts to open a window through an indirect intuition call and stores a pointer to a global variable if the call fails.
- `call_close_window`: Wrapper that calls the intuition library function CloseWindow and returns its result to the caller.
- `next_random_number`: Advances a mixing state held in globals using multiply, add and shift steps, reseeding it after every 200 draws.
- `call_handler_if_set`: Calls the routine whose pointer is held in a global when that pointer is nonzero, then returns the register pair it was given unchanged.
- `check_global_flag`: Checks a global variable and calls another routine if the variable is non-zero.
- `invoke_handler_return_zero`: Calls a routine reached through a pointer stored in data and returns zero.
- `call_routine_pass_registers`: Calls a routine and returns the register pair saved on entry unchanged, so the caller sees its own register values.
- `init_channel_record_table`: Calls two other routines, clears four channel-state globals, then scans a byte table to find a boundary.
- `reset_sound_channel_state`: Clears four channel-state globals and scans a byte table to find a boundary.
- `reset_hardware_state`: Clears a global variable and sets several other global variables to zero or specific bitmask values.
- `advance_audio_sequence_step`: Advances a step counter for the sequence and, when a step ends, runs a short delay loop and copies four channel pointers from a table into the audio location registers.
- `parse_music_track_event`: Decodes one packed track event and copies its pointer, period and volume values into the channel record.
- `copy_word_to_target`: Copies one 16-bit value from a field of a caller-supplied record to the address held in a register.
- `scale_channel_volume`: Multiplies a 16-bit value by a stored scale factor, shifts the product right by four bits, and stores the result in the channel record.
- `clamp_channel_period`: Masks a period value to its low 12 bits and stores it in the channel record.
- `apply_channel_effect`: Applies a per-tick effect to a channel record, such as sliding the period or changing the volume.

### MegaBall (1991)(E. Mackey)(SW)[a4].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_infinite_loop`: Decodes as an empty infinite loop in the gap between routines, so it is not a real routine.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `empty_routine`: This routine is empty and contains no executable instructions.
- `stub_empty_infinite_loop`: The recovered body is only an empty infinite loop.
- `release_node_countdown`: Calls exec to free and unlink a record while a flag is clear, then decrements its countdown.
- `finish_node_or_mark_pending`: When the countdown field is zero, calls exec to free and remove the record and returns a value stored in it.
- `free_and_remove_record`: Calls exec to close the library, free the record memory and remove it from its list, then returns a value stored in the record.
- `find_tagged_entry`: Walks a block of variable-length entries, comparing each entry's first word with the requested value.
- `call_and_offset_result`: Calls another routine and adds eight to its result when a flag is clear.
- `copy_counted_bytes`: Copies a block of bytes one at a time from a source to a destination for a counted length.
- `copy_bytes_pretest`: Copies bytes one at a time from a source to a destination, testing the count before each copy.
- `unpack_packbits_runs`: Unpacks a PackBits-style stream: a negative control byte repeats the following byte, and a non-negative control byte copies literal bytes.
- `unpack_packbits_stream`: Unpacks a PackBits-style stream in which each control byte selects a repeat or a literal copy, writing into the destination until the input length is used up.
- `call_routine_and_return`: Calls one other routine and returns with no other work.
- `write_ilbm_picture_file`: Writes an IFF picture file: it builds a FORM header with the ILBM type and a BMHD header, packs each 16-bit word into three bytes, and writes the chunks through the file library.
- `finish_ilbm_file_write`: Completes an IFF write by checking the written size and padding it to a four-byte boundary, then closes the chunk and the file through library calls.
- `close_file_if_open_2`: If a flag is set, calls a file-library routine to close the open file, then calls another routine and returns.
- `accumulate_and_write_rows`: Builds eight running sums from table words and a scaled multiplier, then repeatedly adds the first word into each sum while calling a file-library routine to write each step.
- `accumulate_rows_recursive`: Fills a 72-entry buffer of running sums from table words, calls itself to total the sub-results, and writes each row through a file-library routine.
- `pack_bits_encode_stream`: Compresses a stream in PackBits style: it counts runs of identical bytes, writes a repeat header for each run and a literal header for other data, and returns the number of bytes produced.
- `pack_bits_encode_literals`: PackBits-style encoder that writes literal runs and repeat headers into the output buffer and returns the total output length.
- `pack_bits_encode_block`: Packs a block into PackBits-style repeat and literal runs, writing the headers into the output and returning the total output length.
- `count_identical_run_length`: Counts how many following bytes equal the first byte, stopping at a run limit of 128 or at the end of the input.
- `find_current_task_entry`: Gets a value from the system through a library call, then walks a linked list and returns the node whose stored field matches it, or zero when none matches.
- `clear_field_on_flag`: Calls another routine and, when its result is nonzero, zeroes a field in the record.
- `store_value_if_nonzero`: Calls another routine, then stores the input value in a record field when that value is nonzero.
- `compute_status_bits`: Builds a status word from two record fields: it sets one bit when a value is within a limit and another bit when a second field exceeds its limit.
- `read_ilbm_header_info`: Checks an IFF FORM for the ILBM type and copies the BMHD header fields into a local record.
- `unpack_packed_pixel_words`: Unpacks the three-byte groups written by the picture writer back into 16-bit words.
- `call_and_offset_result_2`: Calls another routine and adds eight to the returned value if a boolean flag is false.
- `allocate_and_queue_node`: Allocates a list node through a library call when a flag allows it, stamps it with a value from the system, and queues it on the list.
