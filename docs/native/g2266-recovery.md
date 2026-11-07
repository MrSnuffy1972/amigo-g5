# Game 2266 (worms - levels disk (1996-03-07)(various)[h nfa][addon levels][wb]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02266). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 32 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 187 of 187 named (2 author symbols), 187 of 187 described (14 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2 of 2 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5554 of 5573 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 738318 |
| code | 16892 |
| data | 134589 |
| padding | 8825 |
| unused | 2560 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Fade | 5 | 29 | 2 |
| Assign | 1 | 938 | 7 |
| wormintro.exe | 2 | 28 | 2 |
| noclick | 1 | 34 | 2 |
| diskfont.library | 2 | 4480 | 170 |
| 7 | 1 | 52 | 2 |
| Worms - Levels Disk (1996-03-07)(Various)[h NFA][addon levels][WB].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- worms - levels disk (1996-03-07)(various)[h nfa][addon levels][wb]/Worms - Levels Disk (1996-03-07)(Various)[h NFA][addon levels][WB].adf:c/wormintro.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- wormintro.exe

## Copy protection found

- none found

## Functions


### Fade

- `start_fade_command`: Program entry of the Fade command.
- `busy_wait_word_countdown`: Decrements a 16-bit counter until it wraps around, which gives a short timed delay.

### Assign

- `run_assign_command`: Entry point of the Assign command.
- `lookup_assign_volume_name`: Looks up the volume name for an assignment and copies it into a 260-byte buffer.
- `copy_bytes_to_buffer`: Copies a number of bytes from a source pointer to a destination buffer managed through a pointer in A1.
- `process_assign_entry_list`: Walks a linked list of assignment entries, handling each count in batches of 100 through library calls.
- `validate_assign_target`: Checks whether a name can be used as an assignment target and reports when the assignment would refer to itself or the name cannot be assigned.
- `copy_volume_name_before_colon`: Copies the name that comes before a colon into a buffer of at most 30 characters.
- `compare_strings_ignore_case`: Compares two strings letter by letter after converting lower-case letters to upper case.

### wormintro.exe

- `initialize_program_state`: Performs initial setup by calling a function through a pointer in memory and writing a value to a memory location determined by a calculation involving a value from a structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### noclick

- `_AllPatch`: Calls a drive-patching routine three times, then toggles one bit in a drive structure when a library call returns zero.
- `_PatchDrives`: Opens the trackdisk device through exec message port and request calls, toggles one bit in the drive structure when a library result is zero, then closes the device and frees the request and port.

### diskfont.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `find_matching_font_entry`: Searches a list for an entry whose key and low flag bits match the given record.
- `compare_packed_halves_equal`: Compares two 32-bit values by cross-multiplying their 16-bit halves.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `parse_font_parameters_2`: This library routine attempts to retrieve multiple values from a source by calling a function repeatedly.
- `execute_cleanup_callbacks`: This library routine calls a function from a global table and then executes a specific function from another table three times before calling a final function.
- `execute_multiple_callbacks`: This library routine executes a specific function from a table three times followed by a final function call.
- `alert_and_close`: This library routine calls functions to display an alert and close a library.
- `alert_and_close_library`: This library routine displays an alert message and then closes a library.
- `alert_and_free_block`: Calls the Alert function, then frees a memory block with the exec FreeMem function and returns zero.
- `release_unused_font_entries`: Walks a list of entries, unlinks those whose count is not positive, and releases each one through library calls, including an unload of a code segment.
- `release_font_entry_if_unused`: If the entry's count is zero and it has no pending flag, it closes the library three times, unlinks the entry and frees it through library calls.
- `return_without_action`: Returns immediately with no work done.
- `set_pending_free_flag`: Sets bit 7 of the flag byte in the structure and returns zero.
- `clear_flag_and_increment_count`: Clears the top bit of the flag byte in the structure and increments the 16-bit count stored beside it.
- `decrement_count_and_release`: Decrements a 16-bit count in the structure.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_font_metadata`: Allocates a memory block for a font structure and populates it with dimensions, bitplane offsets, and character data.
- `shift_font_bitplane_words`: Scales a pair of 16-bit values down until they fit within a 14-bit range, then builds font glyph words from the shifted values and returns a pointer into the output.
- `allocate_and_initialize_font_structure`: Allocates a new memory block via a function pointer and populates it with metadata, including dimensions, bitplane offsets, and a copied name string.
- `copy_font_name_and_links`: Copies a name string of the given length into a font structure, then stores pointers to the copied name and to a second field at fixed offsets within the structure.
- `search_font_table_for_value`: Scans a table of longword entries for the value in the data register, calling a helper when the table is exhausted, and updates a counter in the structure as it goes.
- `build_entry_offset_table`: Walks a table of 32-bit entries, looks up each entry's position in that table, and records a word offset and width into two arrays.
- `record_matched_entry_offset`: Scans a table for a value matching the current entry, stores a long at a word-indexed slot when it matches, and then resumes the shared scan loop.
- `fill_offset_pairs_and_list`: Continues the table scan: stores matched values, computes running offsets, and when the scan finishes it allocates a block through a library call and copies a record list into it.
- `accumulate_running_offsets`: Computes running totals over a table of entries, writing each total with its width into a per-entry slot.
- `emit_record_list_block`: When a flag is nonzero, writes a header tag of 2 and a pointer, counts records from a library list, allocates a block, and copies the list into it.
- `build_record_list_block`: Writes a header tag of 2 and a pointer, counts records from a library list, allocates a block, and copies the list into it.
- `clear_two_longs_and_close`: Clears two longs at the given pointer, calls a library routine, and calls an unnamed routine.
- `call_helper_return_zero`: Calls one unnamed routine with no arguments and returns zero.
- `free_entry_list_and_block`: Frees a linked list of nodes one by one through a library call.
- `execute_callback_chain_2`: This library routine traverses a linked list of structures and executes function calls through pointers stored within them.
- `execute_callback_chain`: As part of a library, this routine checks a pointer and executes a sequence of function calls through offsets in a structure.
- `negate_library_result_if_negative`: Calls a library routine for a word result and negates that result when the input word is negative.
- `multiply_signed_32bit_to_64bit`: Multiplies two signed 32-bit values by splitting each into 16-bit halves, sums the partial products, and fixes the sign of the 64-bit result.
- `append_name_entry_to_buffer`: Measures a zero-terminated name, optionally searches an existing string list for a match, and writes a fixed-size record with the name and its fields into a buffer.
- `store_name_entry_record`: Searches a list of strings for the name, and when no match is found writes a fixed-size name record and the copied name into a buffer.
- `store_entry_after_name_match`: Compares the name with a string list and, on a match, continues into the shared record-writing code, otherwise it advances to the next string.
- `store_name_entry_after_scan`: Advances through a string list comparing names, and when the scan ends it writes a fixed-size name record into a buffer and optionally a copied list of two-long entries.
- `store_entry_with_list_copy`: Counts the entries in a library-provided list, allocates a block large enough for them, and copies the list into it.
- `store_entry_list_copy`: Scans a list of two-long entries for a match and, when the flag and pointer are set, copies the list into a freshly allocated block.
- `store_entry_list_skip_match`: Walks a list of two-long entries, skipping over equal entries until a mismatch, then writes the name record and copies the remaining list.
- `scan_list_then_store_entry`: Steps through a list of two-long entries comparing each against the current one, and when the scan ends it writes the name record and optionally a copied list.
- `store_entry_record_with_pointer`: Reserves ten bytes in the output buffer and writes the name record, storing either a supplied pointer or a copied list of two-long entries.
- `write_name_record_fields`: Copies the name bytes into the record, fills in the record's fields from the entry data, and either stores a supplied pointer or copies a list.
- `fill_entry_record_fields`: Writes the record's source address, flag and width fields, then either stores a supplied pointer or copies a list.
- `copy_two_long_list_terminated`: Copies each two-long entry from a library-provided list into a destination array and ends it with a zero entry.
- `store_record_pointer_return_one`: Stores a supplied value into one fixed field of a record and returns one.
- `copy_entry_flag_into_record`: Copies the flag byte from the entry into the record with its top bit cleared, then returns one.
- `return_zero`: This library routine returns zero.
- `enumerate_matching_font_entries`: Walks the list of font entries and counts those whose flags match the request.
- `scan_font_directory_entries`: Counts font entries that match the request flags and, when the directory flag is set, scans font files on disk through library calls, checking each name for a period.
- `scan_font_directory_fragment`: Continues the font scan from the directory loop: it counts matching entries, then scans file names for a period and writes the matched records into the output buffer.
- `scan_fonts_directory_for_files`: References the FONTS: directory and counts matching font entries.
- `scan_font_files_for_records`: Walks a directory listing through library calls, splits each file name at a period, and writes matching font records into the output buffer.
- `match_dot_font_file_entries`: Checks each directory entry whose name ends in a period-separated font extension and writes the matching records into a buffer.
- `write_font_entry_loop_tail`: Continues the font file loop by writing each matching entry and advancing to the next one, then closes the scan through library calls.
- `advance_font_entry_records`: Advances through the font record table and writes each matching entry, then closes the directory scan through library calls.
- `read_font_tag_flags`: Reads font records with the font and tag extensions, sets flag bits from their size and style values when limits are exceeded, and writes the matching records into the output buffer.
- `read_font_style_flags`: Sets style flag bits on each font entry from size and tag values read through library calls, then writes the entries into the output buffer and returns a count.
- `enumerate_fonts_directory_records`: References the FONTS: directory and walks its entries through library calls, building a font record for each match and returning a count, negated when negative.
- `return_zero_value`: Returns zero immediately without any other work.
- `parse_font_parameters`: This library routine reads template arguments from a file and stores the resulting values into a structure.
- `read_dpi_template_arguments`: Reads the X and Y DPI, dot pitch and symbol set arguments through library calls, storing the first two values into one pair of fields and the next two into another.
- `load_font_file_entries`: Takes a font filename, rejects names too long to store, and splits off the extension.
- `append_font_entries_from_list`: Walks a list of font records and, for each entry of the matching type, allocates a new record, copies its name and attributes, and appends it.
- `build_font_list_and_release`: Builds a list of font records from the directory scan, copying each entry's name and attributes into fixed-size slots.
- `collect_font_records_from_scan`: Walks the directory scan through library calls, allocates a record for each font entry, copies its name and attribute fields into it, and links the records into a list.
- `finish_font_record_block`: Writes the header of the final font record, copies the stored entries into it in fixed-size slots, then frees the temporary list and returns the record head.
- `copy_font_entries_and_release`: Copies each linked entry of a header plus 64 words into one contiguous array, then frees the temporary list and returns the array head.
- `free_font_list_and_return`: Frees each node of a temporary linked list through library calls, releases the working resources, and returns the stored head pointer.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a7).
- `call_library_vector_690`: Library stub: loads a library base from a global variable and calls the library function at offset minus 690 of that base, returning its result to the caller.
- `load_otag_font_data`: Loads the font data file named in the buffer through library calls and adjusts the offset values flagged with the high bit so they point into the loaded block.
- `open_library_file_with_tags`: Finds a library file named in the buffer and adds the base offset to flagged table entries.
- `close_library_handle_tail`: Calls a tagged library function and releases the handle it returns when the call succeeds, then returns the stored result.
- `return_stored_result_after_release`: Releases a pending resource when the flag is set and returns the stored result value.
- `store_glyph_metrics_for_code`: For a character code, reads its glyph record and computes the left offset and width, rounding them by the style flags.
- `store_glyph_bounds_and_advance`: Computes a glyph's left edge and advance from its record, clamps the running bounds, writes the offsets into the tables, and sets the bitmap pointer for the glyph row.
- `copy_glyph_rows_to_bitmap`: Copies a glyph's bit rows into the bitmap one row at a time, updating the running width and bounds.
- `blit_glyph_rows_into_bitmap`: Merges each row of a glyph's bit data into the destination bitmap by shifting 16-bit words into place.
- `merge_glyph_row_bits`: Merges the remaining glyph row words into the destination bitmap, advances the row pointers, and records the glyph's position when it matches the current one.
- `merge_glyph_bits_and_finish`: Merges glyph row bits into the destination bitmap until the rows run out, then records the glyph's position when it matches the current one and returns the result.
- `store_glyph_slot_metrics`: Writes the saved glyph offsets and widths into the metric tables for the current slot, and clears the saved state when the slot matches.
- `store_saved_glyph_offsets`: Writes the saved offset and width values into the metric tables at the given slot and returns the result.
- `load_font_glyph_tables`: Loads the font named by its file name, clears a 256-entry table, and builds glyph offsets and widths for each character.
- `load_font_file_glyph_data`: Clears a 256-entry glyph table, opens the font file named by the entry, reads its size and style flags, and loads the glyph data for each character.
- `read_font_size_style_flags`: Reads the font's size and style flags through library calls and takes the bounds from the font header or the saved defaults.
- `read_font_bounds_from_header`: Reads the font's style flags and two pairs of header bounds through library calls and stores them as the working bounds.
- `store_font_header_bounds`: Loads the font's bounds from its header, reading the extended bounds when the flag is set, and records them for the glyph loop.
- `compute_font_header_metrics`: Computes the font's baseline, height and width from header values and extended bounds, and stores them for the glyph loop.
- `compute_font_height_bounds`: Scans the glyph table entries for the lowest and highest vertical values, then stores the resulting font height and baseline offset.
- `scan_glyph_height_extremes`: Scans the glyph table entries for the lowest and highest vertical values, then stores the resulting font height and baseline offset.
- `store_font_height_and_metrics`: Stores the computed font height, then reads the header metrics through library calls and records the baseline and width values for the font.
- `store_font_baseline_metrics`: Stores the font baseline and reads the header metrics through library calls.
- `build_font_header_and_name`: References the font extension, computes the font's height and baseline, stores the font name into a buffer, and writes the header fields and glyph count.
- `find_or_add_glyph_entry`: Looks for a glyph entry whose height matches the current font height.
- `find_matching_glyph_entry`: Scans the glyph table for the entry that matches the font height and width, and records the baseline values from the first match.
- `search_glyph_table_for_match`: Walks the glyph table comparing each entry's height and width against the current font.
- `read_font_tag_metrics`: Reads the font's metric tags through library calls and sets the default size and style bits from them.
- `initialise_font_metric_defaults`: Sets the font metric flags from the tag lookups, then initialises the glyph table fields to their default values before the scan begins.
- `collect_glyph_records_by_code`: Loops over character codes, looks up each glyph record through a library call, stores its pointer in the glyph table, and tracks the lowest and highest bounds of the glyphs found.
- `collect_glyph_records_with_ranges`: Looks up glyph records for each character code through library calls.
- `compute_glyph_table_extents`: Walks the glyph table by character code, tracking the lowest and highest values, and stores the font's base offset and extent.
- `allocate_font_block_with_stub`: Allocates a font block sized from the glyph count, writes the font header fields, and places a two-instruction return stub in it.
- `fill_font_block_glyph_data`: Writes the font header into the block, then copies the glyph bitmap data for each character into it and stores their offsets.
- `sum_glyph_widths_for_font`: Adds up the glyph widths across the character range and stores the total as the font width.
- `sum_glyph_widths_into_header`: Totals glyph widths over the character range and writes the total into the font header, with a fallback value for the lowercase range.
- `finalize_font_width_and_release`: Finds the largest combined glyph offset and width, clamps it to that maximum, and writes it into the font header.
- `release_font_resources_tail`: Writes the computed maximum width into the font header, then releases the glyph list and the temporary block through library calls.
- `compute_glyph_origin_offsets`: Computes each glyph's origin by splitting the spare space between the glyph and its cell in half, and stores the two offsets.
- `clip_glyph_offsets_to_cell`: Clamps a glyph's x and y offsets so the glyph stays inside its cell, moving any excess into the other axis.
- `store_glyph_offsets_and_advance`: Writes the adjusted offsets into the output arrays and moves to the next glyph.
- `clip_and_store_glyph_offsets`: Clamps the offset when the cell has room, writes both values into the output arrays, and advances through the glyphs.
- `store_glyph_offsets_and_release`: Writes the glyph offsets into the output arrays, advances the glyph index, and releases the working resources through library calls when the glyphs are finished.
- `finish_font_header_and_release`: Writes the computed font extent into the header, releases the glyph list and temporary block through library calls, and returns the result pointer.
- `initialise_font_block_header`: Sets the self-pointer and the clamped count fields of a font block, clears its high flag, and sets its size.
- `set_font_block_size_fill`: Sets the block size field, locks the library, fills the block through an unnamed routine, and unlocks.
- `return_zero_value_2`: This routine is an empty function that returns zero to the caller.
- `find_font_entry_by_name`: Copies the requested name into a buffer and walks the font list through library calls.
- `find_matching_font_record`: Walks a list through library calls until it finds a record whose key and flag bits match the request.
- `match_font_record_key`: Scans records for one whose key and flags match the request, comparing the flagged values and continuing when they differ.
- `search_font_records_by_key`: Scans the font record table for an entry whose key and flags match the request.
- `resolve_matching_font_record`: Scans the font record table for a key and flag match, resolves each match through an unnamed routine, and returns the first non-zero result.
- `scan_font_records_for_resolution`: Walks the font record table for matching key and flags and resolves each match through an unnamed routine.
- `try_font_style_variants`: Tries the font lookup for several style variants (the base, a doubled size and a halved size), using flag bits to step through the combinations.
- `try_font_size_variants_loop`: Tries a font lookup at the stored size, double size and half size for each style bit set in the flags, and returns the first successful result.
- `try_font_size_variants_fast`: Tries a font lookup at the stored size, double size and half size when the flag bit is set, and returns the first successful result.
- `return_zero_constant`: This library routine returns a zero value to the caller.
- `pick_widest_font_record`: Measures each font record through a library call, keeps the one with the largest measured value, and passes it to an unnamed routine.
- `select_largest_measured_font`: Finds the largest measured value among the font records through a library call, passes the best one to an unnamed routine, and returns the result.
- `select_font_by_measured_width`: Measures the font records through a library call and chooses the widest one, passes it to an unnamed routine, and returns that result or zero.
- `select_font_by_max_measure`: Chooses the font record with the largest measured value through a library call, passes it to an unnamed routine, and returns that result.
- `select_largest_font_fallback`: Measures a font through a library call, keeps the largest measured value, and passes the best candidate to an unnamed routine.
- `select_font_by_measure_loop`: Repeats the measured selection over the font records, choosing the largest measured value each pass.
- `handle_font_request_entry`: Entry handler for font requests that calls library routines for the lookup and reports the result.
- `parse_device_font_name`: Splits a font name at its device colon, copies the name into a request buffer, and checks the font list for an entry of the matching type.
- `open_fonts_directory_font`: Opens a font named in the FONTS: directory by building a request, looking it up through library calls, and releasing any temporary entry.
- `lookup_cached_font_entry`: Looks up a font in the cache and sets the cache flag when the lookup succeeds.
- `lookup_font_entry_and_update`: Looks up a font entry, sets the cache flag when the entry is found, and updates the size fields when they changed.
- `lookup_font_entry_cache_update`: Looks up a font entry and updates its size fields when they differ.
- `release_font_request_and_return`: Runs the lookup unnamed routine, releases the temporary font resource when one was allocated, closes the request through a library call, and returns the unnamed routine's result.
- `convert_char_to_uppercase`: Converts a lowercase letter to uppercase, including the extended Latin-1 letters except the division and small y with diaeresis.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d7 to d0, then copies (a7)+ to d7, then returns.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings one character at a time, uppercasing each first.
- `compare_names_case_insensitive`: Compares two zero-terminated names ignoring case and returns false on the first mismatch.
- `match_string_ignoring_case`: Compares two zero-terminated strings by uppercasing each character as it goes.
- `return_one_value`: Returns one immediately without any other work.
- `copy_null_terminated_string`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compare_strings_ordered`: Compares two zero-terminated strings byte by byte.
- `return_sign_from_flags`: Returns negative one when the comparison flags show less-than, otherwise one.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `append_string_to_buffer`: Finds the terminating zero of the destination buffer, then copies the source string, including its terminator, to that point.
- `find_byte_in_string`: Scans a zero-terminated string for the given byte value.
- `scan_string_for_byte_forward`: Scans a zero-terminated string forward for a byte value and returns a pointer to the first match, or null when the terminator comes first.
- `forward_call_with_two_args`: Forwards its two arguments to an unnamed routine and returns nothing.
- `find_last_byte_in_string`: Scans a zero-terminated string and returns a pointer to the last occurrence of the byte value, or null when it does not appear.
- `return_void`: This library routine performs no actions and returns.
- `fill_bytes_with_value`: Fills a block of bytes with the given value for the given count, and returns the start pointer.
- `fill_bytes_loop_tail`: Stores the byte value into consecutive memory locations, one per loop step, for the count given.
- `compute_fixed_point_product`: Multiplies two 32-bit fixed-point values by combining the cross products of their 16-bit halves, and returns the 32-bit fixed-point product.
- `divide_fixed_point_values`: Divides one 32-bit fixed-point value by another, handling negative operands by negating the result.
- `negate_64bit_result`: Calls an unnamed routine that returns a 64-bit value and returns the negation of its two longs.
- `divide_fixed_point_quotient`: Divides a 32-bit fixed-point value by another using 16-bit halves and a normalising shift, returning the quotient.
- `divide_fixed_point_with_remainder`: Divides one 32-bit value by another and returns the quotient and remainder packed into a 64-bit result.
- `divide_with_normalised_divisor`: Normalises the divisor by shifting it left, then divides the dividend by it, producing a 64-bit result that holds the quotient and remainder.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.

### 7

- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `empty_routine`: This routine is empty and contains no executable instructions.

### Worms - Levels Disk (1996-03-07)(Various)[h NFA][addon levels][WB].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_zero_2`: This routine performs no operations and returns a value of zero to the caller.
