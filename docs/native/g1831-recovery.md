# Game 1831 (eye of the beholder - advanced dungeons& dragons (1990)(ssi)(de)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01831). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2745376 top-level bytes in 191 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 777 of 777 named (0 author symbols), 777 of 777 described (44 by rule, 713 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3761 of 3761 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 15960 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 28410 of 28410 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2016486 |
| code | 83564 |
| data | 489381 |
| padding | 43817 |
| unused | 112128 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 386 | 1 |
| 6 | 1 | 2 | 1 |
| 8 | 1 | 2 | 1 |
| diskfont.library | 15 | 1040 | 76 |
| Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:eob | 6 | 231 | 6 |
| Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:eob1 | 7 | 231 | 6 |
| Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:EOB2 | 2 | 267 | 8 |
| Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:Install EOB | 3 | 5338 | 148 |
| Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 3 of 3).adf:Council.Cps | 3 | 9312 | 299 |
| Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1628 | 39 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9961 | 190 |
| Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 2 of 3).adf:level2.elo (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 2 of 3).adf:level5.elo (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 3 of 3).adf:LEVELSAM11.CPS (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- eye of the beholder - advanced dungeons& dragons (1990)(ssi)(de)(disk 1 of 3)/Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:INTRO4.CPS: high-entropy data file (7.61 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 615 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2687 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- eye of the beholder - advanced dungeons& dragons (1990)(ssi)(de)(disk 2 of 3)/Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 2 of 3).adf:level2.elo: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder - advanced dungeons& dragons (1990)(ssi)(de)(disk 2 of 3)/Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 2 of 3).adf:level5.elo: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder - advanced dungeons& dragons (1990)(ssi)(de)(disk 3 of 3)/Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 3 of 3).adf:LEVELSAM11.CPS: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder - advanced dungeons& dragons (1990)(ssi)(de)(disk 2 of 3)/Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- eye of the beholder - advanced dungeons& dragons (1990)(ssi)(de)(disk 3 of 3)/Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- eye of the beholder - advanced dungeons& dragons (1990)(ssi)(de)(disk 1 of 3)/Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:eob1: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- eye of the beholder - advanced dungeons& dragons (1990)(ssi)(de)(disk 1 of 3)/Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:EOB2: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 15960 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Disk-Validator

- `format_display_string`: Converts a numeric value from a global variable into a character string, stores it in a buffer, and enters a loop that performs bitwise operations and bit-shifting on data within a structure.

### 6

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### 8

- `return_negative_one`: Sets d0 to negative one and returns.

### diskfont.library

- `branch_to_font_handler`: As part of library code, this routine branches to an offset of 80.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: As library code, it calls two routines using function pointers from a structure, and if both return non-zero, it initializes a linked list within a structure and returns it.
- `call_library_functions`: As library code, it calls two different routines using function pointers stored in a global structure and returns zero.
- `remove_font_from_list`: As library code, it iterates through a linked list, removes nodes where a specific field is less than one, and if a counter reaches zero, calls two routines and performs a linked list insertion.
- `update_font_status`: As library code, it checks two conditions in a structure; if met, it calls two routines and performs a linked list insertion, otherwise it sets bit 7 of a byte at offset 14 of the structure and returns zero.
- `set_font_bit`: Sets bit 7 of a byte at offset 14 of the structure in a6 and sets d0 to zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_branch`: As part of library code, this routine decrements a word at offset 38 of the structure in a6, branches if the result is not zero, otherwise it checks bit 7 of a byte at offset 14 of the structure and branches if it is zero, otherwise it jumps to an offset of negative 18 from a6.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `realign_font_metadata_entries_2`: Iterates through a linked list of font metadata structures, calculates the length of strings within those structures, and shifts the string data within a buffer to adjust offsets.
- `prepare_list_traversal`: As library code, it loads a pointer from an offset of a4 into a1 and sets d4 to one, then branches to another location.
- `parse_linked_font_list`: Iterates through a linked list of font data structures, copying character strings into a destination buffer and updating a structure with the new string length, offset, and metadata.
- `realign_font_metadata_entries`: Iterates through a linked list of font metadata structures, calculates string lengths, and shifts data within a buffer to adjust offsets.
- `parse_font_metadata_table_2`: Iterates through a font metadata structure to identify font entries by checking specific markers and lengths, then populates a table with font properties.
- `parse_font_metadata_table`: Iterates through a data structure to identify font entries by checking specific markers and lengths, then populates a table with font properties including an offset, a size, and a bitmask.
- `read_font_data_to_buffer`: Reads data from a disk source using a library call and copies it into a destination buffer.
- `parse_font_metadata_entries`: Iterates through a font metadata structure to identify entries, populates a table with font properties including a type flag, length, and character data, and performs string length calculations to adjust remaining buffer offsets.
- `return_zero_stub`: As part of library code, this routine sets d0 to zero, restores registers d2 through d6 and a2 through a4 from the stack, unlocks a6, and returns.
- `calculate_font_offset`: As library code, it calculates a value based on the difference between a 16-bit word at offset 256 of a structure in a2 and a 16-bit word at offset 4 of a structure in a1, then subtracts values from a global table seven times based on bitwise comparisons of bytes at offsets 2 and 6, and 3 and 7 of the structures.
- `subtract_table_values`: As part of library code, it iterates through a loop seven times, checking bits in bytes at offsets 2 of a structure in a1 and 6 of a structure in a0, and subtracts a long value from a global table based on a counter from a global variable.
- `update_font_index`: As part of library code, it iterates through a loop up to seven times, subtracting values from a table at an offset of 72 using an index derived from shifting a 16-bit value in d4 left by two bits, and returns the final result in d2.
- `update_font_character_index`: Shifts a 16-bit value in register d4 left by two bits, subtracts a value from a table at offset 72 using the result as an index to update register d2, increments d4, and loops if d4 is less than 7.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `optimize_font_glyph_table`: Searches a font table to find the glyph with the maximum value, then updates the structure by writing a pointer, incrementing a counter, and modifying bit 1 of a byte at offset 85 of the glyph structure.
- `update_font_metadata_loop`: As a library routine, it calls another routine and then enters a loop that stores the value from d0 into a location at offset negative 12 from a6, continuing the loop if a condition is met.
- `update_font_entry_metadata`: Iterates through a table of font entries to find the one with the maximum value, then updates a structure by writing a pointer, incrementing a counter, and setting a bit in a status byte.
- `clear_registers_and_branch`: Clears registers d4 and d3, sets d2 to 255, and performs a short branch to a different location.
- `update_font_table_entry`: Iterates through a table to find the entry with the maximum value, then updates a structure by writing a pointer and incrementing a counter at offset 92.
- `calculate_and_clear_offset`: As part of library code, it calls a routine to unload a segment, multiplies the 16-bit value in d2 by 260, uses the result to calculate an address in a2, clears a 16-bit word at an offset of 256 from that address, and then branches to another routine.
- `search_and_update_font_table`: Iterates through a table of font entries to find the entry with the maximum value, then updates a structure by writing a pointer, incrementing a counter, and setting a bit in a status byte.
- `parse_disk_font_file`: Parses a font file from disk to build an in-memory font table.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `load_pointers_and_branch`: As part of library code, this routine loads a long pointer from offset 4 of the stack into a0, loads a long value from offset 8 of the stack into d0, and branches to an offset of 28.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `initialize_pointer_and_branch`: As part of library code, this routine loads a long pointer from offset 4 of the stack into a0, sets d0 to zero, and branches to an offset of 54.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `load_pointers_and_branch_2`: As part of library code, it loads a long pointer from offset 8 of the stack into a0, loads a long pointer from offset 4 of the stack into a1, and branches to another location.
- `skip_zero_byte`: Tests a byte at the address in a0 and increments a0; if the byte is not zero, it branches to another routine, otherwise it sets d0 to zero and branches to another routine.
- `compare_bytes_and_set_d0`: As part of library code, it compares a byte from the address in a0 with a byte from the address in a1; if they are equal, it checks if a byte at the address in a1 is greater than or equal to the byte in a0, otherwise it sets d0 to 255 and branches to another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `load_parameters_and_branch`: As part of library code, it loads a pointer from offset 4 of the stack into a0, loads a byte from offset 11 of the stack into d0, and branches to another location.
- `compare_and_load_pointer`: Compares d1 with d0; if they are not equal, it branches to another routine, otherwise it loads the address in a0 into d0 and branches to another routine.
- `increment_and_compare_byte`: As part of library code, it increments the address in a0, compares the byte at that address with d1, and if they are not equal, it returns; otherwise it sets d0 to zero and returns.
- `call_routine_and_adjust_pointer`: As part of library code, it saves registers d2 through d3 to the stack, loads a long value from offset 12 of the stack into d3, loads a byte from offset 19 of the stack into d2, pushes d3 onto the stack, calls another routine, and then modifies a0 by subtracting 1 and adding d3 before restoring the stack and branching to another location.
- `compare_byte_and_load_pointer`: Compares the byte at the address in a0 with d2; if they are not equal, it branches to another routine, otherwise it loads the address in a0 into d0 and branches to another routine.
- `check_address_and_restore`: As part of library code, it decrements the address in a0, compares it to d3, and if it is not greater than or equal to d3, it sets d0 to zero, restores d2 and d3 from the stack, and returns.
- `increment_font_index`: As library code, it calls another routine twice with different second arguments, and if either call returns a non-zero value, it increments that returned value and returns it; otherwise it returns the original input.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:eob

- `execute_init_and_free`: Calls an initial routine and a loop that executes a function pointer from a global variable, then clears a value at an offset from a pointer and returns a concatenated register value.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `execute_nested_loops`: Calls two routines twice, then enters a loop that executes another routine and performs additions on values at an offset from a pointer in a1 based on values returned in d0 and d1.
- `load_address_and_set_d0`: Loads the address at offset 494 from the program counter into a1 and sets d0 to two.
- `copy_memory_block_2`: Copies a block of memory from a source address to a destination address using a loop that decrements a counter.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:eob1

- `initialize_and_free_memory`: The routine serves as the program entry point.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `execute_nested_loops_2`: It calls a routine to load an address and a constant, then enters a loop that runs three times, calling another routine and performing additions to a memory location based on a value returned in d1.
- `load_address_and_constant`: Loads the address at a fixed offset from the program counter into a1 and sets d0 to two.
- `shift_memory_block`: Shifts a block of memory by performing a series of bitwise left shifts on bytes within a specified range.
- `perform_logical_or`: Performs a bitwise logical OR of zero onto register d0.

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:EOB2

- `relocate_amiga_hunks`: Parses a memory block by identifying AmigaDOS hunk type codes to perform relocation and data copying.
- `decrement_and_loop`: Decrements d1 in a loop until it reaches zero.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `process_memory_blocks`: It performs a series of memory operations including copying bytes and moving blocks of data between addresses, using a loop that continues as long as a pointer is less than a specific value.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `perform_bit_rotations`: Performs a series of bitwise shifts and rotations on values in d3 and d5 using a count from d4 within a loop that runs d0 times.
- `execute_trap_routine`: Moves a 16-bit word from an offset of 1791 in a2 to d4, performs a bitwise OR of 60 onto d2, calls a trap, and then performs a bitwise OR of d0 into d1.

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:Install EOB

- `initialize_installer_environment_2`: Clears a memory block using a loop, calls another routine to initialize a structure, and then performs string manipulation to prepare a command or path by copying bytes and appending specific characters.
- `cleanup_installer_libraries`: Runs an optional callback, then releases resources according to a flag: it frees one memory block, or closes a library and performs message cleanup.
- `install_game_files_by_mode`: Checks for existing files and directories with lock calls, then creates directories and renames or opens font and data files according to a mode argument.
- `pass_name_after_last_colon`: Scans a null-terminated string to find the last colon character and calls another routine with the original string and a pointer to the character immediately following that colon.
- `copy_file_blocks_between_handles`: Copies data between two open file handles in blocks until the source ends or an error occurs.
- `check_volume_directory_exists`: Scans a string for a colon or null terminator, replaces the character at that position with zero, and calls a function pointer from a table to attempt to lock the location, returning true if the lock succeeded and restoring the character regardless.
- `call_with_scaled_offset`: It divides a 16-bit argument in d2 by eight, multiplies the result by nine, and passes the new value along with the other two arguments to another routine.
- `forward_three_arguments`: Passes its three arguments unchanged to another routine and returns.
- `find_string_end_and_call`: It iterates through a string in a0 to find a null terminator or a carriage return, then calls five different routines via pointers stored in a table, and if a carriage return was found, it calls itself with an incremented address and an offset of eight.
- `draw_text_segment_and_continue`: Runs the graphics text setup calls and draws one text segment, then calls itself on the following line when the current character is a carriage return.
- `render_text_lines_with_styles`: Scans a string for a carriage return, performs a sequence of graphics setup calls several times, and calls itself recursively on the next line if a carriage return is found.
- `invoke_nested_routines_2`: Executes a sequence of calls to another routine, passing a global variable and certain constant values.
- `allocate_list_node_with_retry`: It copies eight 32-bit words from a global variable to the stack, repeatedly calls a memory allocation routine until a non-zero pointer is returned, and then links this new block into a list at a specific offset while incrementing a counter.
- `scatter_bytes_to_buffers_4`: Copies five consecutive bytes from a source pointer into five destination planes spaced 8000 bytes apart.
- `copy_interleaved_bytes_to_planes`: Copies five consecutive bytes from a source buffer into five destination planes spaced 8000 bytes apart, iterating through a specified number of rows and columns.
- `execute_rect_copy_wrapper`: Saves registers to the stack, loads several 16-bit values from an offset of 32 on the stack into registers, calls copy_bounded_rectangle_planes, and restores the registers before returning.
- `copy_bounded_rectangle_planes`: Clips a rectangle to a 40-column by 200-row boundary and copies the resulting area row by row into five planes spaced 8000 bytes apart.
- `clear_two_words_2`: Clears two 16-bit words at offsets 1126 and 1128 from a pointer in a4.
- `close_indexed_file_handle`: Looks up a file handle in a table using a word index from its argument.
- `copy_fixed_bytes_2`: Copies 64 bytes one at a time from the source pointer in a1 to the destination pointer in a2 using a loop.
- `open_and_close_file_check`: Calls a helper with its argument, then opens a file through the dos library and closes it again when the open succeeds.
- `call_delay_function`: Checks if a parameter is non-zero and, if so, calls a function pointer retrieved from a table at an offset of 198 from a base address in a4.
- `validate_and_reset_object_pointer_2`: It calls another routine and, if the result is non-zero, uses that result to access a pointer at a specific offset within a structure and sets that pointer to zero before returning true; otherwise it retries the call once and returns true if the second attempt is also non-zero.
- `run_optional_callback_then_call`: Calls a callback stored in a state field when that field is set, then calls another routine.
- `iterate_and_execute_callback`: Repeats a helper call until it returns zero, then calls another helper and stores a returned byte in a state field at offset 2152 of a structure.
- `set_pen_move_and_draw`: Calls three different functions retrieved from a table at offsets 240, 246, and 254 from a base address in a4.
- `fade_palette_toward_target`: Steps the red, green and blue nibbles of each of 32 palette entries one unit toward their target values, reloads the palette through a graphics library, and repeats until every entry matches.
- `lock_path_and_report_success`: Calls a callback, then attempts to lock a path through the dos library.
- `fill_rectangle_with_pens`: Sets the foreground and background pens through the graphics library, then fills a rectangle in the drawing area.
- `lock_and_release_handle_check`: Locks a path through the dos library and, when the lock succeeds, unlocks it again.
- `find_string_index_2`: Iterates through a table of pointers to null-terminated strings, calling a case-insensitive comparison routine for each entry against a provided string.
- `unlink_and_free_list_entry`: Searches a linked list for the entry matching a given block, unlinks it and frees its memory through the exec library.
- `release_all_table_entries`: Walks a table of 20-byte entries while each name is non-empty, calling another routine to free attached memory and clearing the pointer, then iterates through ten additional slots to call that same routine if a non-zero value is found.
- `search_string_table_3`: Iterates through a table of pointers and performs a case-insensitive string comparison against a provided parameter, returning the index if a match is found or zero if a null terminator is encountered.
- `sort_flagged_entries_by_attribute`: Clears ten slots, then collects the indexes of table entries whose high flag bit is set together with their attribute bytes.
- `transfer_control_to_handler`: Saves several registers to the stack, pushes the value of a0 onto the stack, loads three constant values into registers a2, a3, and a4, and then branches to another routine.
- `process_input_event_chain_3`: Iterates through a linked list of input event structures to update a bitmask and a circular buffer.
- `process_input_event_chain`: Iterates through a linked list of input event structures, setting bits in a bitmask based on specific event codes and updating a circular buffer with event data.
- `check_and_set_status`: Compares a byte at offset 4 of a structure in a0 to 2; if equal, it clears d1, loads a word from offset 6 of a0 into d0, and if d0 equals 104, sets bit 1 of a byte at offset 8 of a structure in a1 and sets bits 0 and 1 of bytes at offsets 3f6 and 3f7 of a4, otherwise it returns.
- `branch_to_offset`: Branches to a specific offset.
- `set_status_flags`: It compares a word in d0 to 105, and if equal, sets bit 2 of a byte at offset 8 of a structure in a1 and sets bits 1 and 0 of bytes at offsets 1014 and 1015 of a structure in a4, then moves 113 into d0 and branches to another routine.
- `branch_to_instruction`: Performs a short branch to a specific instruction located elsewhere in the executable.
- `update_hardware_status_2`: Compares a word in d0 to 233, and if equal, clears bit 1 of a byte at offset 8 of a structure in a1, clears bit 0 of a byte at offset 1014 of a4, sets bit 0 of a byte at offset 1015 of a4, and branches to another routine.
- `update_hardware_status`: If a word in d0 is equal to 233, it clears bit 2 of a byte at offset 8 of a structure in a1, clears bit 1 and sets bit 1 of a byte at offset 1014 and 1015 of a structure in a4, then checks a long value in a0 and returns it.
- `process_input_event_chain_2`: Iterates through a linked list of input event structures, updating a bitmask based on event codes and appending event data to a circular buffer.
- `call_add_int_server`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `compute_ring_buffer_fill`: Returns the difference between the write and read indices of a ring buffer, which gives the number of queued entries.
- `process_input_event`: Pushes the value in d7 onto the stack, clears d7, and if d7 is not zero, calls another routine; otherwise it pushes d0 to the stack, calls another routine, adjusts the stack pointer, and enters a loop that masks d7 with 255 and repeats.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d7 to d0, then copies (a7)+ to d7, then returns.
- `call_handler_conditionally`: Saves registers d6 and d7 to the stack, sets d7 to zero, and if d7 is not zero it calls another routine; otherwise it moves the value from d0 to d6, and if d6 is not zero it branches to another routine, otherwise it sets d0 to zero and branches to another routine.
- `increment_modulo_counter_2`: Enters a loop that calls another routine, and if the lower byte of the result is zero, increments a 16-bit value at offset 1126 of a structure in a4 and applies a modulo 16 operation, continuing until the result of the called routine has its lower byte non-zero.
- `move_d7_to_d0`: Moves the value from register d7 to d0 and restores the previous values of d6 and d7 from the stack.
- `process_input_event_loop`: Performs a recursive traversal of a structure or linked list, updating a value or pointer with the return value of each recursive call, while calling map_scancode_to_character and dequeue_input_event.
- `peek_input_event`: Pushes d7 to the stack, clears d7, and if d7 is not zero, calls another routine; otherwise it pushes d0 to the stack, calls another routine, adjusts the stack pointer, and moves d0 to d7 before looping back.
- `call_input_handler`: Calls another routine and returns the value from d7.
- `test_pointer_nullity`: Loads a longword from offset 8 of a structure in a5 into d7 and, if it is not zero, branches to another routine; otherwise it clears d0 and returns.
- `map_keyboard_input`: It processes a 32-bit value by either updating a structure in a5 and calling a function pointer if the lower 8 bits are less than 76, or by performing various table lookups and bitwise adjustments to a value based on the input bits and returning the result.
- `map_key_with_modifiers`: Retrieves a byte from a table using a masked value as an index, applies conditional subtractions based on bits in a register, and then adds various bit-weighted offsets to the result based on the state of several bits in the input registers.
- `modify_register_and_branch`: Tests bit 3 of d4, and if set, subtracts 35 from d6 and branches to another routine; otherwise it returns.
- `modify_key_value`: Masks d0 with 48, and if the result is not zero, subtracts 45 from d6 and branches.
- `adjust_key_offset`: Compares d5 to 112 and 113, and if d5 is between these values, adds 86 to d6 and branches.
- `lookup_key_value_with_modifiers`: Maps a key code through a table in one band and an offset in another, then adds modifier bits to any non-zero result and returns it.
- `map_scancode_to_character`: Converts a raw key code through a table to a character value, remaps four specific values, and then adds modifier bits for the qualifier flags.
- `dequeue_input_event`: Under Forbid and Permit protection, removes the next entry from the ring buffer and returns it.
- `peek_next_queued_event`: Returns the next queued entry without removing it from the ring buffer, or zero when the buffer is empty.
- `read_file_into_memory_block`: Checks available memory, allocates a block sized for a file record, and opens and reads the file through the dos library.
- `free_all_list_entries`: Walks a linked list of entries, releasing each entry's contents through a helper, and then clears the list head.
- `set_default_mouse_pointer`: Clears two pointer state globals and calls intuition library routines to set the mouse pointer.
- `clear_mouse_pointer`: Calls one intuition library routine and then ClearPointer to remove the mouse pointer image.
- `call_with_suppressed_field`: Saves a 16-bit value from offset 1032 of a structure, sets that same offset to 0, calls another routine, restores the original 16-bit value to the offset, and returns 1.
- `update_installer_state_and_draw`: Updates a structure with values from a table, performs several drawing operations using calculated coordinates, and enters a loop that calls an indirect function through a pointer in a structure until a specific value is returned.
- `process_installer_state`: Updates a structure based on a value from another structure, enters a loop that calls a function via a pointer and calls another routine repeatedly, and finally writes two 16-bit values to a table at an offset determined by a value in a global variable.
- `manage_installer_state`: Updates a table entry using an index from a structure, calls another routine, and enters a loop that repeatedly calls another routine and updates a value at offset negative 40 from a5 until a specific condition is met.
- `store_negative_byte`: Stores the value negative 255 in register d0 into a memory location at an offset of negative 40 from the address in a5, then branches to another routine.
- `restore_window_field_and_return`: Unlocks a saved handle when one is set, restores a window field from a saved value, and returns one.
- `search_string_table`: Iterates through a table of pointers to compare a string with a provided string, and if a match is found, follows a chain of indices to return a value.
- `calculate_table_offset`: Multiplies the value in d7 by 20, adds this result to an address at offset 792, loads a 16-bit word from the address at that location into d0, and branches to another routine.
- `increment_and_branch`: Increments the 16-bit value in d7 by 1 and branches to another routine.
- `return_negative_one_2`: Returns a 32-bit value of all ones and restores registers d7, a2, and a3 from the stack.
- `shift_entry_position_fields`: Subtracts the argument from one position field of the current entry record and adds scaled multiples of it to three other fields.
- `resolve_string_and_update_table`: Resolves a named entry through a helper and copies its string into a local buffer, locking or opening the named file depending on the mode argument.
- `release_unused_table_entry`: Checks if a table entry is active by iterating through a list of ten entries; if no active slot is found, it calls another routine to free a memory block and clears the pointer at offset 4188 of the structure in a6, returning 1 if the entry was released and 0 otherwise.
- `find_best_index_in_table`: Calculates the number of entries in a table by iterating through a list of structures until a null terminator is found, then iterates through those entries to find the index of the entry that satisfies specific integer and byte comparisons while also having the minimum byte value at offset 42 of the structure.
- `close_installer_screens_and_libraries`: It executes several function pointers stored in a structure, clears specific fields in a structure at offset 786, and conditionally calls multiple routines to release resources such as interrupt servers, windows, screens, and library-related memory.
- `initialize_installer_environment`: Opens the required system libraries, allocates per-frame buffers and bitmap planes, creates a custom screen and window, and adds an interrupt server.
- `call_avail_mem`: Library stub: loads its arguments and the exec library base, calls AvailMem and returns its result to the caller.
- `read_slot_block_from_file`: Checks an entry slot and, when it is empty, first calls a helper to fill it.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `compare_string_list_2`: Iterates through a list of pointers to compare strings against a provided string and returns 1 if a match is found and a specific flag is set in a global variable, otherwise it returns 0.
- `gather_planes_to_interleaved_bytes_2`: Loads three 32-bit values from the stack, uses the third value as a pointer in a0, and calls another routine.
- `gather_planes_to_interleaved_bytes`: Reads bytes from five planes spaced 8000 bytes apart and writes them interleaved into a single buffer, advancing along each row by a fixed stride.
- `update_structure_from_parameter`: If the parameter is not zero, it copies several fields from the structure pointed to by the parameter into a global structure, including a 32-bit value, a 16-bit value, a byte, and a byte with its most significant bit cleared, then calls a series of functions through a table until a null pointer is encountered.
- `restore_registers_and_return_3`: Restores registers d7, a3, and a6 from the stack and returns.
- `execute_indirect_function_2`: Loads a pointer from a global variable and calls the function at that address twice via an offset of negative 192.
- `calculate_scaled_product`: Multiplies the upper and lower halves of a 32-bit value together, masks the product to sixteen bits, and scales the result by five.
- `update_index_and_store_data`: Stores a 32-bit value into a table and increments or wraps a counter if it exceeds a certain limit.
- `process_input_and_update_status`: Decrements bytes at specific offsets in a structure, updates several counters if those bytes reach zero, and processes input from hardware registers to update status flags and a global variable.
- `update_status_flags_2`: Sets bit 1 of a byte at offset 1464 of a4, compares two 16-bit values after shifting them right by 8 bits, and sets bit 0 of the same byte if the first value is greater than the second, then branches to another routine.
- `update_status_flags`: Updates a byte in a global variable by setting specific bits based on a comparison with a value in a different memory location.
- `process_input_and_update_table_2`: Processes input from a 16-bit value by setting bits in a local buffer based on bitwise operations and a comparison with a global variable, then updates a table using a loop that runs eight times.
- `parse_control_string`: Parses a custom-encoded instruction stream by iterating through bytes and handling specific control characters to trigger recursive calls that split or render rectangular buffer areas.
- `invoke_subroutines_on_stack`: Calls another routine twice using a local stack buffer as an argument.
- `store_half_word`: Stores a 16-bit value into a global variable.
- `reset_table_entry_and_call`: Calculates an offset, clears two 16-bit values in a table, and calls an indirect function.
- `write_multiple_words`: Writes several 16-bit or 32-bit values into a table at an offset determined by a parameter.
- `raster_divide_rect_buffer`: Recursively splits a rectangular area into smaller sub-rectangles or strips based on a calculated width or height.
- `execute_conditional_callback_2`: Checks a value in a table and calls an indirect function if certain bit patterns or zero-checks are met.
- `execute_indirect_function_from_stack`: Loads a function pointer from an offset relative to a value stored on the stack and calls it.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `validate_and_clear_pointer`: Checks a pointer or a table entry against a certain condition and, if a pointer in a table or certain memory location is non-null, clears that pointer.
- `delete_and_reset_pointer`: Attempts to delete a file and, if unsuccessful, updates a global variable with a new value and sets a status flag.
- `parse_install_script_tokens`: Parses a string or a quoted string containing escaped characters, storing pointers to the tokens in a table.
- `clear_d0_and_set_constant`: Clears the lower byte of d0 eight times and then sets d0 to 97.
- `traverse_and_call_list_elements`: Iterates through a list of pointers and calls another routine if certain bit flags in the pointed-to data are not set.
- `transform_string_case`: Iterates through a null-terminated string and converts certain characters to a different case based on a global flag bit.
- `copy_string_with_limit_3`: Calculates the length of a null-terminated source string and copies up to a specified maximum number of bytes to a destination buffer, ensuring the result is null-terminated.
- `compare_case_insensitive_strings_2`: Compares two null-terminated byte strings by iterating through them and converting uppercase characters to lowercase for the comparison.
- `copy_string_to_buffer_2`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `find_byte_in_null_terminated_string`: Iterates through a null-terminated string starting at the address in the first parameter, comparing each byte to the value in the second parameter.
- `search_string_for_byte`: Loads a pointer from a stack offset and a target byte from a stack offset, then calls find_byte_in_null_terminated_string to locate the byte within the string.
- `find_last_byte_in_string`: Iterates through a null-terminated string and returns the address of the last occurrence of a specific byte.
- `save_registers_and_move_d0`: Moves the contents of a2 into d0 and saves several registers to the stack before returning.
- `load_word_and_branch`: Clears a longword in d0, loads a word from the stack into d0, and branches to another location.
- `convert_to_string`: Converts a value to a null-terminated string and returns the length of the string.
- `traverse_and_free_list`: Traverses a linked list of pointers and calls a library function to free each element, finally nulling a pointer in a global structure.
- `validate_pointer_with_offset`: Calls a routine to check a pointer and returns either that pointer or a default maximum value, while also checking if a certain global variable or table entry is non-zero.
- `traverse_and_call_list_elements_2`: Iterates through a list of pointers and calls another routine if bit 2 of the byte at offset 19 of the pointed-to data is clear, bit 1 is set, and the long words at offset 2 and offset 6 of the pointed-to data are not equal.
- `check_and_clear_pointer`: Checks a bitmask against a value from a library call and, if a pointer at offset 83c of a structure in a4 is non-null, clears that pointer.
- `update_file_pointer`: Checks a bitmask against a value from a library call and clears a pointer in a table if it is non-null.
- `initialize_structure_bitmasks`: Writes a full-byte bitmask at offset 8 of a structure and writes full-word bitmasks at offsets 20 and 24 of that structure, then calls a routine located at a fixed offset within that structure.
- `reset_display_context`: Checks if a field at offset 10 of the passed structure is non-zero to conditionally call a function pointer, then writes 255 to offset 8 and a four-byte value of all ones to offset 20.
- `close_file_conditionally`: Checks a global variable and calls a file-closing routine if a certain condition is met.
- `compute_fixed_point_product_3`: Multiplies two 32-bit fixed-point values by calculating the product of their high and low 16-bit components and combining the cross terms to return a 32-bit fixed-point result.
- `negate_conditional_values`: Tests d0 and if not positive negates it, then tests d1 and if not positive negates it, calls another routine, and finally negates d1 before returning.
- `negate_and_return`: Calls another routine, negates the resulting values in d0 and d1, and returns.
- `negate_and_call_routine`: Checks the sign of a 32-bit value in d1; if negative, it negates d1, calls another routine, and then negates d0 before returning; otherwise it returns immediately.
- `normalize_and_divide_fixed_point_3`: Normalizes a value by performing a series of bit-shifts based on its magnitude, then calculates a quotient and remainder to perform fixed-point division.
- `normalize_and_divide_fixed_point`: Normalizes a value in a register by performing a series of bit-shifts based on its magnitude, then calculates a quotient and remainder to perform a fixed-point division.
- `compare_and_branch_logic`: Compares a byte loaded from a pointer in a4 to 79 and, if it is not greater than or equal to that value, branches to another location; otherwise it performs a series of register and address manipulations and branches to another location.
- `execute_indirect_logic`: Copies a block of memory, calls an indirect routine via a global variable, and if a specific value is non-zero, calls another routine twice and returns a specific value; otherwise it calls another routine, stores the result in a structure at offset -100 of a6, and calls another routine with several parameters.
- `store_indirect_result`: Calls an indirect function using a pointer from a6, stores the result in a structure at an offset of 100 from a5, and either branches to another routine or sets d0 to all ones and branches elsewhere.
- `execute_indirect_function_from_stack_2`: Stores a value into a structure at an offset, calls an indirect function, and returns either zero or all ones based on the result.
- `return_zero_constant_3`: Clears register d0 to zero and restores multiple registers from the stack before returning.
- `allocate_and_initialize_node`: If the provided parameter is non-zero, it allocates memory for a structure, sets the byte at offset 8 to 5, the byte at offset 9 to 0, and stores the parameter as a 32-bit integer at offset 14; otherwise, it returns zero.
- `create_message_node_3`: Obtains a global variable and calls a routine through a table; if the result is a sentinel, it returns zero.
- `get_table_entry_offset`: Checks a value against a global limit and a table of zero-checks, returning a calculated offset into a table or a fixed value if the check fails.
- `traverse_list_and_clear_pointer`: Calls a routine to get a value, and if certain bits are set and a pointer at offset 83c of a structure is non-null and its value is non-zero, clears that pointer and calls itself recursively.

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 3 of 3).adf:Council.Cps

- `initialize_application_environment`: Clears a memory block using a loop, initializes several global variables, and calls another routine to load libraries.
- `execute_conditional_cleanup`: Checks a pointer in a structure and executes a routine if it is not null, followed by calls to free or close memory-related resources.
- `populate_pointer_table`: Zeros a byte at offset 3440 of a structure in a4, executes several function calls with varying arguments, and then iterates through a table of pointers to pass each one to another routine.
- `populate_string_pointer_table`: Iterates through a newline-terminated sequence to retrieve pointers and store them into a table, while passing these pointers to another routine via a table offset.
- `execute_complex_sequence`: Performs a variety of memory operations including copying a local buffer, updating tables, zeroing memory, and executing a sequence of function pointers or bitwise rotations.
- `update_state_tables`: This routine executes a sequence of loops that call various routines to update state tables, copy memory blocks, and initialize linked list nodes.
- `execute_graphics_sequences`: This routine executes a series of graphics-related routines, including zeroing memory blocks, updating rectangle boundaries, and processing a loop that reads bytes from a buffer to look up and execute function pointers or modify memory.
- `update_graphics_state`: Updates tables, modifies rectangle boundaries, and performs pixel color or bitwise updates while executing a sequence of function pointers.
- `initialize_graphics_buffer_and_sprites`: Initializes a global graphics state and populates a buffer with a sequence of drawing or copying operations.
- `update_state_table`: Iterates through a loop, checks values against a constant, and updates a table or memory offset by incrementing or resetting a byte.
- `process_state_table`: Copies a small block of memory, then either performs a series of swaps and updates a table based on a value in a parameter, or iterates through a loop to check values, update a table, and call other routines.
- `execute_graphics_sequence`: Sets up local data, calls another routine, and then either calls another routine with specific parameters or enters a loop that toggles a boolean and calls a sequence of other routines based on bytes read from a memory range.
- `update_rectangle_bounds`: Iteratively modifies or checks the boundaries of a rectangular area by incrementing or decrementing coordinate-related variables and updating a global memory location.
- `clamp_input_value_2`: Returns a value of negative one, zero, or one based on whether the provided integer argument is less than zero, zero, or otherwise.
- `store_high_bits`: Stores the upper 16 bits of a passed parameter into a global memory location and loops if a certain flag is not set.
- `execute_sequence_of_routines`: Sets a byte in a global structure, calls several subroutines with or without arguments, and executes two functions retrieved via pointers from a table.
- `initialize_display_context`: Allocates memory for a display context structure, sets up a color map, and configures a new screen or window.
- `perform_logical_or_2`: Performs a bitwise logical OR operation on register d0 with zero.
- `check_available_memory_2`: Uses a library call to check for available memory and returns a boolean indicating if the result exceeds 5000.
- `setup_pointer_chain_3`: Allocates memory via a library call, sets a byte at offset 8 of the new memory to 4, and calls another routine with the address incremented by 20.
- `free_memory_if_nonzero_2`: Checks if a passed pointer is non-zero; if so, it loads a function address from a global variable and calls it with the pointer as an argument.
- `free_two_pointers_2`: Retrieves a pointer from a global variable and calls a function at an offset of 210 bytes from that pointer, then repeats this process using the same pointer value to call a function at the same offset.
- `free_pointer_list_2`: Iterates through a list of pointers and calls a routine to free each non-null pointer, then calls a library function to free a global pointer.
- `conditionally_free_memory_2`: Checks if a passed pointer is non-zero and if the value at offset 24 of that pointer is non-zero, then calls a library function to free memory.
- `free_memory_via_global_pointer`: Checks if a passed pointer is non-null; if it is not, it reads a 16-bit value from that pointer, adds 2 to it, and calls a function via a pointer stored in a global variable using that value and the original pointer as arguments.
- `call_free_mem_if_valid_2`: Checks if a pointer passed as an argument is non-null and, if so, calls the memory deallocation library routine using an offset from a global pointer.
- `traverse_and_free_recursive_2`: Recursively traverses a pointer-based structure by checking specific bitmask values in a byte at offset 14, then decrements a 32-bit integer at offset 10 before calling a deallocation routine via a global pointer.
- `process_node_attributes`: Calls another routine, and if the result is non-zero, checks bitmask values in a byte at offset 14 of the result; if certain bits are set, it calls another routine with a pointer from offset 6, then decrements a 32-bit integer at offset 10 and calls another routine.
- `push_and_call_routine_2`: Pushes a 32-bit value from offset 6 of a structure onto the stack, calls another routine, cleans up the stack, and branches to another location.
- `push_pointer_and_call_routine_3`: Pushes a longword from an offset of 6 in a structure pointed to by a2 onto the stack, calls another routine, and then branches to another location.
- `push_pointer_and_call_subroutine`: Pushes a longword from offset 6 of a structure pointed to by a2 onto the stack, calls a subroutine, adjusts the stack pointer, and branches to another routine.
- `push_pointer_and_call_subroutine_2`: Pushes a longword from an offset of 6 in a structure pointed to by a2 onto the stack, calls a subroutine, adjusts the stack pointer by 4, and branches to another routine.
- `decrement_and_free_memory_2`: Recursively calls itself with a pointer from a structure, then decrements a counter in that structure and calls a library function to free memory.
- `decrement_loop_with_delay_2`: Calls another routine, then enters a loop that uses four empty delay loops to decrement a value passed in a parameter, and finally calls another routine.
- `initialize_audio_buffer_and_event`: Copies a null-terminated string to a global buffer and, if a specific flag is unset, allocates memory to initialize a series of audio-related data structures, copies a template buffer into a loop of sixteen buffers, and configures a routine for system event or interrupt handling.
- `traverse_and_free_list_3`: Traverses two separate linked lists by following pointers from offsets 6a0 and 6a4 of a structure pointed to by a value at offset 2ae of a6, calling another routine on each node and updating the list pointers.
- `free_linked_lists`: Traverses two linked lists and calls a routine to free memory if a pointer or a certain global structure member is not null.
- `process_list_elements`: Iterates through a list to copy a dword from a pointer and increment a value at a secondary location, finally setting a bitmask in a global variable.
- `parse_resource_tag_stream`: Parses a stream of tagged data structures, allocating memory or copying string/byte sequences based on specific header tags.
- `parse_resource_buffer_2`: Iterates through a buffer of typed resource headers to allocate memory and copy or modify data.
- `allocate_and_copy_data`: Allocates memory using a library call and copies a range of words from a source buffer to the new memory area, updating a pointer.
- `parse_data_stream`: Iterates through a buffer of tagged data, allocating memory and managing linked list nodes or string buffers based on specific operation codes.
- `reorder_linked_list`: Traverses a linked list and moves or reinserts nodes into a target list or a secondary list based on string comparison results.
- `insert_list_element`: Inserts a new node into a linked list and optionally updates a global pointer or clears attributes in a table.
- `calculate_string_length_recursive_2`: Traverses a null-terminated string using a loop to find its length, then recursively calls itself using the calculated length as a pointer if the string is not empty.
- `parse_resource_stream_3`: Parses a stream of tagged data, allocating memory for resource buffers and updating a string table or linked list.
- `clear_bit_and_recurse_list`: Calls another routine and, if the return value is non-zero, clears bit 5 of a byte at offset 14 of the returned structure and calls another routine recursively with that structure as an argument.
- `clear_list_attributes`: Disables interrupts, then traverses a linked list to clear bit 5 in a byte at offset 14 of each element, clears four 32-bit values from a table, and re-enables interrupts.
- `trigger_system_event_2`: Calls a library function to cause a system event and returns zero.
- `update_pointer_with_offset_2`: Reads a longword from offset 4 of a structure pointed to by a1, adds it to the longword at offset 0 of that same structure, and then calls three other routines before returning.
- `filter_and_update_list`: Iterates through a linked list to remove elements where bit 1 of a byte at offset 13 is set, then finds the element with the smallest value at offset 24 in a separate list to update its status and call another routine.
- `update_pointer_via_register`: Copies a 32-bit value from offset 0 of a structure in a4 to d0, moves d6 to a0, stores d0 at offset 0 of the structure in a0, moves d0 to a4, and branches to another location.
- `update_list_with_minimum`: Iterates through a list of pointers to find the element with the smallest value at offset 24, clears bit 1 of a byte at offset 13 of that element, and updates the list structure.
- `restore_registers_large`: Restores d4 through d7 and a3 through a6 from the stack and returns.
- `test_structure_bit`: Saves registers to the stack, loads a pointer from offset 1672 of a structure in a5 into a4, clears d7, moves a4 to d0, and then branches based on whether d0 is zero or whether bit 0 of the byte at offset 14 of the structure in a4 is set; if bit 0 is set, it calls another routine and then performs an unconditional branch.
- `test_and_branch_bit`: Tests bit 1 of a byte at offset 14 of a structure in a4; if the bit is not set, it calls another routine and then performs an unconditional branch to another location.
- `traverse_pointer_chain_conditionally`: Iterates through a linked list of structures, checking specific bits in a byte at offset 14 of each structure to determine whether to call another routine or update pointers in a global structure.
- `update_linked_list_node`: Writes a longword from a structure into a destination pointed to by a register, updates a pointer within a global structure, and branches to another routine.
- `transfer_pointer_and_branch`: Moves the value from a4 to d7, loads the longword at offset 0 of the structure pointed to by a4 into a4, and branches to another routine.
- `restore_registers_small`: Restores d7 and a4 from the stack and returns.
- `iterate_table_entries`: Loads a pointer from a structure in a4, then enters a loop that accesses an element from a table using an index derived from a comparison, calls another routine, and increments a counter before branching back to the start of the loop.
- `clear_bit_conditionally`: Checks a byte at offset 104 of a structure in a3, and if it is non-zero, clears bit 4 in a byte at offset 14 of the structure in a4.
- `update_status_bit`: Compares a value from a pointer in a2 with a value in a5, and if the comparison passes, clears bit 7 of a byte at offset 104 of a structure in a3.
- `process_data_stream`: Iterates through a data stream by reading a byte and a following byte from a pointer stored at offset 14 of a structure in a2, then performs different actions based on the byte value: updating a value in a structure, loading a long word from a table, writing a byte to offset 2 of the structure, or clearing a bit in a byte at offset 104 of another structure before returning.
- `update_pointer_from_table`: Compares a byte in d6 to 129; if they are equal, it shifts the value in d5 left by 2 bits, loads a long word from a table using a6 and the new d5 value into a0, stores that long word at offset 4 of a structure in a2, and performs an unconditional branch.
- `compare_and_branch_byte`: Compares a byte in d6 to 136 and, if not equal, branches to another location; otherwise it stores a byte from d5 at offset 8 of a2 and branches to another location.
- `compare_and_branch`: Compares a byte in d6 to 136 and branches if not equal, otherwise it branches to another location.
- `test_and_clear_bit`: Compares a byte in d6 to 255, branches if not equal, clears bit 7 of a byte at offset 104 of a structure in a3, and branches to another location.
- `branch_to_address`: Performs a short branch to a specific address.
- `restore_registers_medium`: Restores d4 through d6 from the stack and returns.
- `update_coordinate_pointer`: Saves registers to the stack, loads a pointer from offset 4 of a structure in a2 into a4, checks bit 1 of the byte at offset 14 of that pointer, and if bit 1 is not set, loads a pointer from offset 6 of the structure into a4, stores it at offset 1672 of a structure in a5, and sets a4 to offset 1536 of that same structure before branching.
- `populate_coordinate_structure`: Calculates spatial or grid coordinates by performing bitwise shifts and scaling operations based on structure values, then stores the resulting 16-bit and 32-bit values into a destination structure at offsets 4, 8, 12, 16, 20, 24, 28, 32, 36, and 44.
- `calculate_scaled_offsets`: Loops while d2 is greater than 1 and d1 is not less than 280, adding d3 and d4 to d5, shifting d3 and d4 left by one bit, shifting d1 right by one bit, and decrementing d2.
- `populate_structure_offsets`: Loads a pointer from offset 24 of a structure in a0, adds d5 to it, and stores the result at offset 4 of a structure in a1, then adds d3 to that same pointer and stores the result at offset 8 of a1, offset 12 of a1 with d3, and offset 16 of a1 with d4 before branching.
- `initialize_structure_values_2`: Calculates a 16-bit value based on a division, populates a structure with various 16-bit and 32-bit values from other structures, and sets several bytes to zero.
- `initialize_structure_values`: Stores a 32-bit word from a0 into a structure at offset 28, sets bytes at offsets 32 and 33 to zero, and calculates a 16-bit value into offsets 2c and 2e based on a multiplication of two values.
- `store_values_to_struct`: Stores a 32-bit word from a0 into a structure at offset 40 of a1, then clears a 16-bit word at offset 48 and a byte at offset 50.
- `update_structure_pointer`: Tests bit 1 of a byte at offset 52 of a structure in a0, and if it is set, branches to another routine; otherwise, it writes 7 to that byte, copies a pointer from offset 628 of a structure in a5 to a1, stores a1 at offset 52 of the structure in a0, stores a0 at offset 628 of the structure in a5, and then branches to another routine.
- `decrement_and_loop_2`: Adds 54 to a0, decrements d0, and loops back to the start of the routine if d0 is not zero.
- `modify_structure_fields`: Clears bit 4 of a byte at offset 14 of a structure in a4, calls another routine, and if the result is zero, clears two long words at offsets 28 and 40 of a structure in a2.
- `clear_structure_field`: Loads a pointer from offset 6 of a structure in a4, then loads a pointer from offset 0 of that pointer, and if it is not null, clears a longword at offset 28 of a structure in a2 before branching.
- `initialize_table_data`: Stores a pointer from a0 into offset 28 of a structure in a2, clears the byte at offset 38 and the word at offset 36 of the structure in a2, then loads a pointer from offset 4 of a structure in a4 into a0; if a0 is not zero, it branches to an earlier instruction, otherwise it clears the longword at offset 40 of the structure in a2 and branches to another routine.
- `calculate_table_parameters`: Calculates several values including a sum and bit-shifted versions of two values through a loop, then stores them into a structure at various offsets.
- `set_structure_fields`: Writes several calculated values into a structure at offsets 4, 8, c, 10, 18, 20, 22, 2c, and 2e.
- `set_uniform_values`: Shifts the value in d0 right by two bits and writes the resulting 16-bit word to offsets 44 and 46 of a structure in a2.
- `update_event_list_structures`: Iterates through a table of structures, clearing a bit at offset 52 and checking if a pointer at offset 24 is less than a base address.
- `parse_compressed_data_streams`: Iterates through two data streams within a structure, handling control bytes to perform jumps, scaling, or pointer updates.
- `combine_bytes_to_word`: Reads two bytes from a buffer, shifts the first byte left by 8 bits, combines it with the second byte, and stores the resulting 16-bit word at offset 34 of a structure in a4.
- `parse_scaled_byte`: If d1 is not 254, it loads a byte from a0, increments d1, shifts d1 left by 1, and subtracts the new d1 from a0 before branching back to the start.
- `parse_scaled_data_stream`: Parses a data stream by reading bytes to update a structure, handling scaling factors and jump offsets.
- `process_scaled_byte`: Enters a loop that compares d1 to 254, and if it is not equal, clears d1, loads a byte from the address in a0 into d1, increments d1, shifts d1 left by 1, subtracts d1 from a0, and loops back.
- `calculate_scaled_value`: Clears d0, loads a byte from a pointer in a0 into d0, and if the byte is not negative, calculates a value by multiplying a word from offset 2c of a structure in a4 by d0, shifts the result right by 6 bits, and clamps the result to a maximum of 16384 before returning it in d0.
- `update_offset_value`: Subtracts 192 from a word in d0, shifts it left by 8 bits, adds a word from offset 46 of a structure in a4, and if d1 is zero, stores the result at offset 46 of that same structure.
- `update_status_byte`: Stores a byte from d1 into offset 50 of a structure in a4, and if d1 is equal to 1, stores a word from d0 into offset 46 of a structure in a4 and clears a word at offset 48 of a structure in a4 before branching.
- `update_event_structure_data`: Calculates a value using division and modulo operations, stores it in a structure, and then conditionally calls other routines to populate a new structure with data from the current structure based on specific bit flags.
- `process_event_list_and_update_structures`: Iterates through a list of structures by traversing a table, modifies specific bits in a byte at offset 52 of each structure, and populates several fields in a new structure using data from the current structure's offsets 4, 8, 12, and 14 before calling another routine.
- `invoke_nested_routines_3`: Calls another routine with a global variable and two constant values, then calls that same routine three more times without arguments.
- `allocate_and_link_node_2`: Copies eight 32-bit words from a global variable to a local buffer, then iterates through a linked list via a pointer in a global variable until a null pointer is found.
- `initialize_clipped_bitmap_2`: Checks if a rectangular area defined by a pointer-based structure falls within certain bounds and initializes a bitmap.
- `clip_and_initialize_bitmap`: Checks if a rectangular area defined by a pointer-based structure falls within the bounds of a target bitmap area; if valid, it initializes the bitmap and fills a memory block with a calculated address or value using a loop.
- `call_scatter_bytes_to_buffers`: Loads four 16-bit values from the stack into registers, passes a pointer from the stack to another routine, and then calls scatter_bytes_to_buffers.
- `scatter_bytes_to_buffers`: Copies bytes from a source pointer to five destination buffers offset by 8000, 16000, 24000, and 32000 bytes using a nested loop structure.
- `scatter_bytes_to_buffers_5`: Copies bytes from a source pointer to multiple destination buffers using a single loop structure.
- `copy_bytes_to_buffers`: Copies bytes from a source pointer to five destination buffers offset by 8000, 16000, 24000, and 32000 bytes using a single loop structure.
- `execute_rect_copy_wrapper_2`: Saves registers to the stack, loads sixteen 16-bit values from an offset of 32 on the stack into registers, calls copy_rect_buffer_blocks, and restores the registers before returning.
- `copy_rect_buffer_blocks`: Checks if a source and destination rectangle are within valid bounds, then copies blocks of memory between them using a nested loop with a stride of 40 bytes.
- `clear_status_words`: Clears two 16-bit words at offsets 1898 and 1900 from a structure in a6.
- `zero_memory_block`: Fills a large block of memory with zeros starting from a pointer retrieved from a table based on a value at offset 1804 of a structure in a4.
- `clear_conditional_pointer`: Checks if a value at a calculated offset in a table is non-zero and calls a function if so.
- `copy_fixed_bytes_3`: Copies sixty-three bytes from one pointer to another using a loop.
- `execute_conditional_callback`: Retrieves a pointer from offset f34 of a structure in a4, and if that pointer is non-zero, executes a function pointer located at offset -1e from that pointer, then if the return value is non-zero, executes another function pointer located at offset -24 from that pointer.
- `call_delayed_function`: Checks if a parameter is non-zero, then calls a function pointer retrieved from a structure at offset 3892 of a6.
- `validate_and_reset_object_pointer_3`: Checks if a value returned by another routine is zero to decide whether to retry or proceed, and if non-zero, retrieves a pointer from an offset in a table and sets that pointer to zero.
- `invoke_conditional_cleanup`: Checks if the word at offset 702 of a structure in a4 is non-zero and, if so, calls the function pointed to by that value.
- `get_string_index_result`: Iterates through a series of values, calling a routine for each, and stores a byte into a global variable at offset 3106 of a structure.
- `execute_graphics_setup_2`: Retrieves a pointer from a structure at offset 3880 of a4 and calls three functions located at offsets -342, -240, and -246 from that pointer.
- `retrieve_table_pointer_3`: Pushes registers to the stack, retrieves a word and a longword from the stack to populate d0 and a0, moves two words into d1 and d3, calls another routine, and restores registers before returning.
- `retrieve_table_pointer`: Checks if a register is zero and returns a concatenated 64-bit value from two 32-bit registers; otherwise, it retrieves a 32-bit value from a table using an index in a register, potentially calls another routine if bit 0 of a byte at offset 6 of a structure is set, and returns the concatenated 64-bit value from two registers.
- `update_pixel_colors`: Compares and updates bits in a destination buffer to match a source buffer, and calls a delayed function if a change or a specific bit-mask condition is detected.
- `execute_conditional_logic`: Checks if a value at a specific offset in a structure is zero; if non-zero, it calls a function via a pointer in a table and then calls another function via a different offset in that same table before returning one.
- `execute_graphics_function_sequence`: Retrieves a pointer from a structure at offset 3880 of a4 and calls three functions located at offsets -342, -348, and -306 from that pointer.
- `toggle_resource_lock_2`: Calls a function at offset -84 from a pointer in a structure at offset 3892 of a4, and if the result is non-zero, calls a function at offset -90 from that same pointer.
- `find_string_index_3`: Iterates through a table of pointers to strings, comparing each string to a provided input using a case-insensitive comparison routine.
- `search_and_decrement_counter`: Iterates through a linked list to find a matching pointer and then decrements a global counter.
- `traverse_table_and_recurse`: Iterates through a table of pointers, calling another routine recursively for each non-null pointer found and clearing that pointer, then iterates through a secondary table of ten entries to call another routine for each non-zero value.
- `search_string_table_4`: Iterates through a table of pointers, performing a case-insensitive string comparison for each non-null entry, and returns the index of the match or zero if a null terminator is reached.
- `push_pointer_to_stack`: Saves registers d0, d1, a0, a2, a3, a4, and a5 to the stack, pushes the value from a0 onto the stack, and branches to another routine.
- `process_event_type_and_update_status`: Iterates through a linked list of event structures to modify bitmasks in a status byte based on event type, toggles specific global bytes if certain conditions are met, and updates a circular buffer with event data and status flags.
- `update_event_status_and_log`: Iterates through a linked list of event structures, setting bits in a bitmask based on the event type and updating a circular buffer with event data.
- `update_status_flags_3`: Compares a byte at offset 4 of a structure in a0 to 2; if equal, it clears d1, loads a word from offset 6 of a0 into d0, and if d0 equals 104, sets bit 1 of a byte at offset 8 of a structure in a1 and sets bits 0 and 1 of a byte at offset 1694 of a structure in a4 before branching to the end.
- `branch_to_address_2`: Branches to a specific address.
- `set_status_bits`: Compares a word in d0 to 105, and if equal, sets bit 2 of a byte at offset 8 of a structure in a1, sets bit 1 of a byte at offset 1694 of a structure in a4, sets bit 1 of a byte at offset 1695 of a structure in a4, and moves 113 into d0 before branching.
- `branch_to_status_modifier`: Branches to another routine that modifies specific bits in a structure based on a value in a register.
- `modify_status_bits_2`: Compares a word in d0 to 233, and if equal, clears bit 2 of a byte at offset 8 of a structure in a1, clears bit 0 of a byte at offset 1638 of a4, sets bit 0 of a byte at offset 1639 of a4, then branches to another routine.
- `modify_status_bits`: If a word in d0 is equal to 233, it clears bit 2 of a byte at offset 8 of a structure in a1 and modifies bits 1 and 0 of a byte at offset 1518 of a structure in a4; otherwise, it loads a long value from a pointer in a0 into d0 and returns.
- `process_event_linked_list`: Iterates through a linked list of event structures, modifying bitmasks in a global structure and updating a circular log buffer with event type and data.
- `call_add_int_server_2`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `normalize_shift_register_value`: Calls a routine to rotate bits in bytes, calculates a divisor by subtracting two values from the stack and adding one, then returns a 32-bit value composed of the quotient and the remainder of the division.
- `process_masked_d7_value`: Pushes d7 to the stack, clears d7, and if d7 is non-zero calls another routine; otherwise it pushes d0 to the stack, calls another routine, adds 4 to the stack pointer, moves d0 to d7, masks the lower 8 bits of d7, and loops back to the start of the routine.
- `restore_d0_from_stack`: Moves the value from register d7 into d0, pops the next longword from the stack into d7, and returns.
- `process_conditional_zero_return`: Saves registers d6 and d7 to the stack, calls another routine, and if the return value in d0 is non-zero, moves d0 to d6; otherwise, it clears d0 and branches to another routine.
- `increment_and_wrap_counter`: Calls another routine and, if the lower 8 bits of the return value are zero, increments a 16-bit word at offset 1930 of a structure in a4 and then performs a modulo 16 operation on that same word.
- `restore_registers_and_return_4`: Moves the contents of register d7 to d0 and restores registers d6 and d7 from the stack before returning.
- `recursively_update_structure_pointer`: Performs a recursive traversal of a structure or linked list, updating a value or pointer with the return value of each recursive call.
- `transfer_and_mask_d7`: Pushes d7 to the stack, clears d7, and if d7 is non-zero calls another routine; otherwise it pushes d0 to the stack, calls another routine, adds 4 to the stack pointer, moves d0 to d7, and loops back to the start of the routine.
- `transfer_d7_to_d0`: Calls another routine and then moves the value from d7 into d0 before returning.
- `check_pointer_validity`: Saves registers to the stack, loads a long from offset 8 of a structure in a5 into d7, and if d7 is non-zero, branches to another routine; otherwise, it clears d0 and branches to a different location.
- `transform_table_value_3`: Calculates a value by reading a byte from a table based on an index and applies conditional bitwise or arithmetic offsets to that value.
- `transform_table_value_2`: Calculates a value by reading a byte from a table using an index derived from a register, applying various arithmetic offsets based on bit tests, and adding bit-weighted values to the result.
- `branch_conditionally_on_bit`: Tests bit 3 of d4, and if clear, branches to another routine; otherwise, it subtracts 23 from d6 and branches to another routine.
- `mask_and_branch`: Copies a longword from d4 to d0, masks the lower bits of d0 with 48, and branches to another routine if the result is zero; otherwise it subtracts 45 from d6 and branches to another routine.
- `compare_and_offset`: Compares d5 against 112 and 113, and if d5 is between these values, it copies d5 to d6, adds 86 to d6, and branches to another routine; otherwise it branches to another routine.
- `transform_table_value`: Reads a byte from a table based on an index and applies conditional bitwise or arithmetic offsets to that value.
- `map_input_to_output_2`: Uses an input parameter to look up a byte in a table and applies conditional bitwise or arithmetic offsets to that byte.
- `update_state_from_table`: Checks if a counter range is valid, calls a function, increments a counter, and loads a 32-bit value from a table into two 16-bit global variables.
- `load_table_entry`: Checks if a counter range is valid and loads a 32-bit value from a table into a register.
- `update_counter_with_conditional_skip`: Calls a function, increments a counter, and loads a value from a table, potentially incrementing the counter further if a certain bitmask matches.
- `decompress_compressed_stream_2`: Unpacks a custom-format compressed data stream from a source buffer into a destination buffer using a variable-length instruction-based decoding scheme.
- `traverse_pointer_table`: Retrieves pointers from a table using an offset and calls another routine recursively or with modified arguments.
- `scatter_bytes_to_buffers_6`: Restores d2-d3 and d0-d3 from the stack, loads a pointer from a stack offset into a0, calls another routine, and restores d2-d3 from the stack before returning.
- `copy_interleaved_data`: Copies bytes from several source pointers offset by a fixed stride into a destination buffer using a nested loop.
- `call_allocate_linked_list_node`: Loads three 16-bit values and a pointer from the stack into registers and calls allocate_linked_list_node.
- `allocate_linked_list_node`: Initializes a block of memory with a repeating pattern, sets a size header, and calls a routine to link a new node into a list.
- `traverse_linked_list_2`: Iterates through a linked list and sets a global pointer to zero after processing nodes.
- `execute_two_functions`: Sets two global bytes to zero and calls two function pointers located via an offset from a global pointer.
- `save_restore_global_word`: Saves a word from a global memory location, clears it, calls another routine, and then restores the original value.
- `update_ui_element_and_poll_input`: Updates a structure's field with a value, modifies two 16-bit values in a table based on an index, and enters a loop that calls a function via a pointer in a structure until a specific return value is received.
- `execute_conditional_logic_sequence`: Calls two routines with specific constant arguments, conditionally updates a structure in a5 based on a byte in a4, and then enters a loop that calls a function via a pointer in a4 until a specific value is returned.
- `process_complex_data_structure`: Calculates an offset using a 16-bit value from a structure in a4, saves data from that offset into a structure in a5, and enters a loop that calls a function via a pointer in a4 until a specific value is returned.
- `store_constant_and_branch`: Sets d0 to 255, stores d0 at an offset of 40 bytes below a5, and branches to another routine.
- `update_buffer_and_pointer`: Updates a memory location using an offset or a value from a pointer and performs several calls to other subroutines.
- `search_string_in_list`: Iterates through a list of pointers to find a null-terminated string that matches a target string, returning a concatenated value from a table if found or negative one if not.
- `calculate_offset_value`: Multiplies d7 by 20, adds the result to a base address, loads a word from that address into d0, and branches to another routine.
- `increment_and_branch_2`: Adds 1 to d7 and branches to another routine.
- `return_negative_one_3`: Returns a value of negative one.
- `update_table_values`: Calculates an offset from a global variable and updates four adjacent memory locations with a value and its multiples.
- `manage_string_table_entry`: Searches a string table for a match and updates or clears a corresponding entry or pointer.
- `validate_table_index`: Checks a table of values for a specific matching index and optionally calls another routine while clearing a pointer or decrementing a counter.
- `find_best_match_index`: Counts the number of entries in a table of 20-byte structures by checking for a null terminator, then iterates through the table to find the index of the entry that satisfies specific integer and byte comparisons while selecting the entry with the minimum byte value at offset 10.
- `cleanup_display_context_4`: Clears a display context structure and invokes a series of cleanup callbacks or deallocates display-related resources.
- `rotate_bits_in_bytes`: Performs bitwise shifts and logical OR operations to move bits between three adjacent global bytes.
- `execute_conditional_function_2`: Checks if a 32-bit value at an offset from a structure in a4 is zero; if it is, it calls another routine, otherwise it calls a function pointer retrieved from an offset of a pointer stored at offset 3890 of the structure in a4.
- `apply_bitmask_to_buffer_2`: Iterates through a buffer and uses bit-testing and bit-setting instructions to modify certain bits based on a lookup table.
- `call_rem_int_server_2`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `compare_string_against_list`: Calls another routine to obtain a target string and iterates through a table of pointers to compare it against a list of null-terminated strings.
- `retrieve_table_pointer_4`: Retrieves a pointer from a table using an index.
- `initialize_bitmap_area`: Checks a condition and potentially calls a routine to retrieve a value from a table-based pointer before returning a concatenated value.
- `call_interleave_source_buffers`: Loads four 16-bit values from the stack into registers, moves a pointer from an offset of 20 on the stack into a0, and calls interleave_source_buffers before returning.
- `interleave_source_buffers`: Reorders bytes from a block of parallel source buffers into a single destination buffer by incrementing pointers with a fixed stride.
- `update_structure_and_call_list`: If a parameter is non-zero, it stores that parameter and several values from its offsets into a global structure, then iterates through a table of function pointers to call each one until a null pointer is encountered.
- `restore_registers_and_return_5`: Restores the d7, a3, and a6 registers from the stack and returns immediately.
- `copy_and_run_routine`: Copies a fixed number of bytes to a destination and executes a routine or two via a pointer or library call.
- `shuffle_buffer_elements`: Iterates through a buffer and swaps bytes using a lookup table and a decrementing counter.
- `calculate_scaled_product_2`: Multiplies the upper and lower 16-bit halves of a 32-bit value together, masks the product to sixteen bits, and scales the result by five.
- `update_index_and_store_data_2`: Loads a short from a memory offset, writes a 32-bit register value to a table, and updates a global index if a certain condition or wrap-around occurs.
- `process_string_list_3`: Iterates through a null-terminated string, calling a routine for each character and conditionally updating a value from a structure.
- `process_string_list_2`: Copies a pointer from a register to another, calls a routine, and then conditionally updates a value from a structure at an offset.
- `process_string_list`: Iterates through a list of pointers to null-terminated strings, calling another routine for each, and updates a pointer in a structure based on the results.
- `increment_pointer_conditionally`: Adds a constant to a register, stores the value of a pointer from a register into a global variable, and then increments that global variable if the byte at the pointer's address is zero.
- `restore_registers_and_unlock`: Restores several registers from the stack, unlocks address register a5, and returns.
- `copy_string_with_conditional_logic_2`: Reads a 32-bit value from a source structure to determine the total number of bytes to process, then iterates through the source buffer to copy segments to a destination buffer.
- `call_function_if_not_zero`: Checks if a 32-bit value is non-zero, then executes a function pointer retrieved from an offset relative to a library or a global structure.
- `parse_encoded_length_2`: Reads a 16-bit value from a buffer and recursively calls itself to calculate a length by skipping two bytes if the value is zero or three, or skipping eight bytes if the value minus four equals zero.
- `update_input_counters`: Decrements counters at offsets 21 and 5 of a structure in a1, increments a global counter, and updates several byte-sized counters in a structure if they exceed a threshold, then performs hardware register and CIAA port updates.
- `update_hardware_register_flags_2`: Sets bit 1 of a hardware register, compares two registers after shifting them, and then sets bit 0 of the hardware register if the first register is greater than the second.
- `update_hardware_register_flags`: Modifies certain bits in the CIAA_PRA register and a global byte based on comparisons with a memory-resident value.
- `update_ciaa_port_and_process_bit_changes`: Modifies the CIAA Port A Data Register based on bit patterns from a 16-bit input, then performs a loop that updates a table of 32-bit values by calling another routine if a specific bit is set.
- `call_wait_b_o_v_p`: Library stub: loads its arguments and the graphics library base, calls WaitBOVP and returns its result to the caller.
- `parse_text_with_control_codes`: Iterates through a source buffer byte by byte, handling control characters such as a carriage return, a specific byte value of two, and a by.
- `parse_text_stream_with_control_codes`: Iterates through a source buffer byte by byte, handling control characters such as a carriage return, a specific byte value of two, and a byte value of six.
- `push_arguments_and_call_text_extractor`: Pushes two values onto the stack and calls a routine twice to retrieve data from offsets of fifty-six and one hundred seventy-three from a pointer in a5, then branches to another routine.
- `process_text_control_characters_2`: Iterates through a string to handle control characters such as form feeds, tabs, carriage returns, and spaces.
- `parse_formatted_text_string`: Iterates through a source string to extract characters, handling control codes for line breaks, special characters, and bit-masked values, while checking against buffer limits and calling copy_string_to_buffer_3 to write the processed data.
- `parse_text_string_structure`: Iterates through a source string to extract characters, handling control codes for line breaks, special characters, and bit-masked values.
- `process_text_control_characters`: Iterates through a string to handle control characters such as form feeds, tabs, carriage returns, and spaces.
- `parse_and_format_text_string`: Iterates through a source string to extract characters, handling special control codes for line breaks, carriage returns, and bit-masked characters, while copying the processed data into a destination buffer and calling other routines to manage buffer limits and string termination.
- `parse_text_string_structure_2`: Iterates through a source string to extract characters, handling special control codes for line breaks, carriage returns, and bit-masked character values.
- `process_text_layout_delimiters`: Iterates through a source buffer to handle tab, carriage return, and space characters by calculating line widths and word wrapping.
- `parse_text_buffer_with_formatting`: Iterates through a source buffer to process bytes, handling bit flags and specific delimiter values to write characters to a destination.
- `parse_and_format_text_buffer`: Iterates through a buffer to parse a sequence of bytes, handling specific delimiter values and bit flags, and writes the processed characters into a destination structure.
- `parse_string_with_delimiters`: Iterates through a buffer to parse a sequence of bytes, handling specific delimiter values and bit flags.
- `process_text_wrapping`: Iterates through a string to handle line breaks and word wrapping by checking character values and comparing current line lengths against a maximum width stored in a global structure.
- `copy_string_to_buffer_4`: Clears a byte at an offset from a pointer in a5, pushes that pointer to the stack, calls a routine to copy a string into the buffer, increments a value in d5 by the value in d6, and branches to another routine.
- `check_text_wrap_condition`: Compares d0 to a byte at an offset from a5, and if they are equal, loads a longword from an offset of a4 into d2, then if d2 is not zero, loads a word from offset 18 of the pointer in d2 into d3.
- `wrap_text_to_buffer`: Iterates through a source string to perform word-wrapping by checking character widths against a destination buffer limit, inserting spaces or null terminators as needed, and calls process_formatted_string_buffer and load_word_from_pointer during the process.
- `process_formatted_string_buffer`: Iterates through a source string to handle control characters and bit-masking, copying processed bytes into a destination buffer while checking against length constraints defined in a global structure.
- `load_word_from_pointer`: Tests a longword in d2; if non-zero, it moves that value to a0 and loads a word from offset 24 of the structure pointed to by a0 into d0, then branches to another routine.
- `parse_text_string_with_formatting`: Iterates through a source string to copy characters into a destination buffer, handling special control characters such as carriage returns and bit-flagged bytes.
- `load_word_from_offset_a4`: Tests a longword at offset 1872 of the structure pointed to by a4; if non-zero, it loads that value into a0 and moves a word from offset 24 of the structure pointed to by a0 into d0, then branches to another routine.
- `parse_text_string_with_formatting_2`: Iterates through a source string to copy bytes into a destination buffer, handling special control characters such as carriage returns and bit-masked formatting flags.
- `update_buffer_byte`: Calculates an address by adding d6 to a pointer derived from a5, increments d6, stores a byte from an offset of a5 into that address, and branches to another routine.
- `execute_conditional_text_wrap`: Checks if a register or a byte at an offset from a5 is non-zero, clears a byte at an offset from a5, and then calls another routine with one or two offsets from a5 based on a comparison between a calculated value and a value derived from registers and a structure.
- `calculate_text_wrap_offset`: Calculates a value based on a 16-bit word from a structure at an offset determined by a global variable, compares it to a value in d0, and calls another routine twice with different offsets from a pointer in a5 depending on the comparison result.
- `call_text_wrapping_routines`: Compares d0 to d3, which is d1 minus 8, and if d0 is higher than d3, it pushes a0 onto the stack, calls a routine with a pointer to offset 92 of the structure in a5, clears the stack, calls another routine with the same pointer, and then branches to another routine.
- `prepare_text_wrapping_parameters`: Calls a routine twice to pass two specific offsets derived from a pointer in a5, then restores registers and returns.
- `invoke_subroutines_on_stack_2`: Loads a 16-bit value from an offset of 8 in a structure pointed to by a5 into d7, pushes d7 onto the stack, and calls two different subroutines using a pointer to a location 30 bytes before the start of the structure as the argument.
- `store_short_to_global`: Stores a 16-bit value passed in a register into a global memory location.
- `execute_function_sequence`: Clears two 16-bit values at offsets of a structure, sets a 16-bit value at offset 3880 of a4 to zero, and calls a function pointer located at an offset from a pointer retrieved from offset 3880 of a4.
- `write_multiple_values_to_table`: Calculates an offset based on a parameter and writes several 16-bit values to a table at that offset.
- `extract_text_structure_pointer`: Loads a value from a global variable, shifts it left by four bits, and if non-zero, retrieves a word from an offset of the resulting pointer and moves it into a register; otherwise, it returns.
- `calculate_text_line_wrapping`: Performs a recursive traversal of a text structure to determine line breaks and text metrics.
- `recursively_calculate_text_layout`: Performs a recursive traversal of a tree-like data structure to calculate text metrics and line breaks, updating length and offset values within the structure based on a font width retrieved from a global pointer.
- `calculate_string_metrics_recursively_2`: Performs a recursive calculation on a data structure to determine string lengths and metrics, involving a loop that iterates over a null-terminated string to update length values in a table.
- `calculate_string_metrics_recursively`: Performs a recursive calculation on a data structure to determine string lengths and metrics, involving a loop that iterates through a null-terminated string to update values in a table.
- `recursively_calculate_string_metrics`: Recursively calls itself to process data within a structure, then calculates a string length by iterating through a null-terminated string and updates values at offsets 496 and 498 of the structure in a4 based on the results.
- `calculate_scaled_index`: Divides a 16-bit value from a structure in a4 by 8, and if d1 is not zero, loads a word from offset 20 of a structure pointed to by d1 into d0.
- `calculate_scaled_offset`: Multiplies a value in d3 by 8, stores the result in d6, loads a word from offset 1636 of a structure in a4 into d0, sign-extends d0 to a longword, and shifts it left by 4 bits; if d1 is not zero, it loads a word from offset 20 of the structure pointed to by d1 into d1.
- `update_stat_and_string_metrics`: Calculates a value by dividing a word from a structure by a value retrieved from another structure, then updates a 16-bit field in a table based on the length of a null-terminated string.
- `update_metrics_and_string_length`: Calls another routine, then if a value in d7 is zero, calculates a value by dividing a word from a structure in a4 by a value retrieved from a pointer at offset 750 of a4, updates a table entry, and iterates through a null-terminated string to update a length value in a structure in a4.
- `load_metrics_word`: Extends a word from an offset of a4 into a long, shifts it left by 4, and if a pointer at an offset of a4 is not zero, loads a word from an offset of that pointer into d1.
- `update_structure_metrics`: Calculates a value based on a string length, updates a 16-bit field in a structure, and conditionally resets a counter if a comparison between two derived values is met.
- `reset_table_entry`: Compares a value derived from a structure in a4 against a value in d1, and if the condition is met, clears a 16-bit value at an offset in a4 and increments a 16-bit value at a subsequent offset.
- `reset_table_entry_and_call_function_2`: Compares d3 against d2 minus 8, and if d3 is not lower, clears a 16-bit value at an offset from a table and increments a 16-bit value at another offset in that table using an index derived from a value in a4.
- `process_string_and_update_table`: Calls another routine with parameters derived from a structure in a4, then if d7 is zero, iterates through a null-terminated string to update a length value in a structure in a4 and conditionally resets a table entry and increments a value in another table.
- `calculate_string_length_and_update_table`: Calculates the length of a null-terminated string, adds this length to a 16-bit value at a calculated offset in a structure, and then conditionally clears a 16-bit value and increments a 16-bit value at another offset in the same structure.
- `reset_table_entry_and_call_function_3`: Compares a value at an offset of a4 against a value from a structure, and if the condition is met, clears a 16-bit value and increments a 16-bit value at an offset in a table, then calls a function pointer from an offset of a5.
- `reset_table_entry_and_call_function`: Checks a range, clears a value in a table, increments a value in another table, and calls a function pointer from an offset.
- `call_indirect_via_stack_offset_2`: Loads a function pointer from an offset of 348 bytes relative to a value stored on the stack and executes it.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `get_pointer_with_validation`: Validates a pointer by checking if it is non-zero and if bit 3 of the byte at offset 1 is not set, then recursively calls itself using a pointer from offset 2 of the validated pointer, returning a default maximum value if the initial pointer is zero, if bit 3 is set, or if a specific global variable is non-zero.
- `handle_file_error`: Checks the return value of a function called via a pointer in a structure, and if zero, calls another function to update a 16-bit value at offset 338 and a 16-bit value at offset 368 of that structure, then returns a status.
- `parse_script_token_string`: Parses a string into tokens, handling quoted substrings and escape sequences like newline or carriage return, and stores the resulting pointers into a list within a structure.
- `clear_register_d0`: Copies the contents of d0 to d5, performs eight logical OR operations with zero on d0, and sets d0 to 97.
- `execute_conditional_functions`: Iterates through a table of 6-byte entries, and if a 16-bit value at an offset is non-zero and its bit 4 is not set, calls a routine using a pointer from the table, then calls another routine with a provided parameter.
- `transform_string_bytes`: Iterates through a null-terminated string and subtracts 32 from any byte if a bit in a table-referenced value at offset 2577 of a structure is set.
- `copy_string_with_limit_4`: Calculates the length of a source string, determines the minimum between that length and a provided limit, and copies that number of bytes from the source to a destination buffer before appending a null terminator.
- `compare_case_insensitive_strings_3`: Compares two byte strings character by character, converting uppercase letters to lowercase for comparison, and returns the difference between the first mismatching characters or zero if equal.
- `copy_string_to_buffer_3`: Finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `find_byte_in_string_2`: Iterates through a null-terminated string starting at the address in the first parameter, comparing each byte to the value in the second parameter.
- `search_string_byte_2`: Pushes registers to the stack and calls a routine to search for a specific byte within a null-terminated string.
- `find_last_occurrence_2`: Iterates through a null-terminated string to find the last occurrence of a specific byte.
- `save_and_return_d0`: Moves the contents of a2 to d0, saves several registers to the stack, and returns.
- `clear_d0_and_branch`: Clears register d0 with the value from the stack and branches to another routine.
- `convert_to_string_2`: Converts a numeric value into a null-terminated string by appending character digits and then shifts the resulting characters to the beginning of the buffer.
- `traverse_and_clear_list`: Traverses a linked list and executes a callback for every node, then clears two 32-bit values at offsets 3896 and 3898 of a structure.
- `get_valid_pointer`: Calls another routine and if the result is non-zero, calls a second routine using a pointer from offset 2 of the first result, returning the second result unless a word at offset b2c of a structure in a4 is non-zero, in which case it returns a value of all ones.
- `traverse_linked_list_conditionally`: Traverses a linked list of pointers starting from an offset of 2420 in a4 and calls another routine if bit 2 of a byte at offset 19 of the current pointer is zero, bit 1 of that byte is set, and the 16-bit values at offsets 2 and 6 of the current pointer are not equal.
- `handle_io_error_2`: Checks a pointer-referenced value and, if a certain condition is met, stores a result and updates a status field.
- `update_position_offset`: Updates a parameter or a pointer-referenced value based on the contents of a third parameter and a library-returned value.
- `initialize_structure_fields_2`: Writes a full-byte bitmask at offset 8 of a structure and writes full-word bitmasks at offsets 20 and 24 of that structure, then invokes a callback located at a fixed offset within that structure.
- `cleanup_display_context_3`: Checks if a field at offset 10 of the passed structure is non-zero to invoke a callback, then writes the value 255 to offset 8 and negative one to offset 20 of that structure.
- `call_conditional_function`: Checks if a global value at an offset of a4 is non-zero before calling another routine, then executes a function pointer from an offset of a structure in a6.
- `multiply_fixed_point_values`: Calculates the product of two 32-bit fixed-point values by multiplying their high and low 16-bit components and summing the cross products.
- `negate_values_and_call`: Tests d0 and d1, negates them if they are negative, and calls another routine.
- `negate_registers`: Calls another routine and then negates the values in registers d0 and d1.
- `negate_conditional_d1_2`: If register d1 is negative, it negates d1, calls another routine, and then negates d0.
- `normalize_and_divide_fixed_point_2_2`: Performs fixed-point division by normalizing a value through conditional shifts and then dividing d0 by a value derived from d1.
- `normalize_and_divide_fixed_point_2`: Normalizes a value in a register by performing a series of conditional left shifts and exponent adjustments, then performs a division to compute a quotient and remainder for fixed-point arithmetic.
- `prepare_system_call`: Loads a byte from an offset of a structure in a0, compares it to 79, and if it is greater than 79, moves the value into d0 and d7 before branching.
- `execute_system_calls`: Copies a block of memory, calls a routine via a pointer, and if a specific condition is met, performs further calls and returns a status value.
- `call_library_function_and_check_result`: Calls a routine via a pointer from a stack offset, stores the result in a structure, and if the result is not zero, sets d0 to negative 256 and branches.
- `call_indirect_via_stack_offset_2_2`: Stores a value in a structure, calls a routine with several parameters, and returns zero if the call returns one, otherwise it returns negative one.
- `return_zero_constant_4`: Clears register d0 and restores several registers from the stack before returning.
- `allocate_and_set_node_2`: If the provided parameter is non-zero, it allocates memory for a structure, sets the byte at offset 8 to 5, sets the byte at offset 9 to 0, and stores the parameter as a 32-bit integer at offset 14 of the structure; otherwise it returns zero.
- `create_message_node_4`: Obtains a global variable and calls a routine through a table; if the result is a sentinel, it returns zero.
- `get_table_entry`: Checks if an index is within a certain range and if a certain table entry is non-zero, otherwise returns a pointer or a default value.
- `execute_conditional_functions_2`: Checks a bitmask in a register and, if certain conditions and a pointer-referenced value are met, clears a memory location and calls another routine.

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)(De)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_return`: Restores registers a5 and a6 from the stack and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_script_buffer`: Clears a memory range using a loop, then populates a buffer by copying bytes from a source address and appending specific characters.
- `execute_callback_sequence`: Executes a series of indirect function calls based on the contents of a global variable and a global structure.
- `initialize_stack_buffers`: Sets up several stack-based buffers by loading values from a specific memory range into the stack and calls a routine using a pointer to a global variable and these buffers multiple times.
- `call_indirect_function`: Loads a function pointer from an offset relative to a value stored on the stack and executes it.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `resolve_nested_pointer`: Resolves a pointer from a table and calls another routine, checking a specific bit in a byte, and returning a failure constant if a global value is non-zero.
- `reset_display_context_2`: Iterates through a loop checking certain structure members and executes a sequence of cleanup or deallocation callbacks.
- `parse_script_string_buffer`: Parses a source string by skipping whitespace and extracting tokens into a pointer table, handling quoted strings with escape sequences for characters like newline and carriage return.
- `set_d0_to_constant`: Copies d0 to d5 and performs several bitwise OR operations with zero on d0 before setting d0 to 97.
- `traverse_and_execute_list`: Iterates through a list of pointers, checking a bitmask and recursively calling another routine if a certain condition is met.
- `copy_string_with_limit`: Copies a null-terminated string from a source pointer to a destination pointer, truncating the copy if it exceeds a specified maximum length and ensuring null-termination.
- `insert_into_list`: Attempts to insert a new block of memory into a linked list of memory blocks, updating a global counter and managing pointer redirection or splitting existing blocks.
- `free_linked_list`: Iterates through a linked list, calling a free memory function on each node, and clears the head pointers.
- `search_and_split_memory_block_2`: If a pointer at an offset is not zero, calls another routine and clears that pointer, then either returns zero or calls another routine to allocate a block of a specified size and updates its value.
- `resolve_table_pointer`: Retrieves a pointer from a routine, then checks a global or a pointer-based value to return either a new pointer or a default error value.
- `allocate_and_prepend_list`: Allocates a new block of memory, initializes it with a pointer to a data payload, and inserts it at the head of a linked list.
- `search_and_split_memory_block`: Searches a linked list for a block of a specific size, potentially splitting a block and updating a global size counter or allocating a new block.
- `store_pointer_with_offset`: Checks if a pointer is non-zero and, if so, stores a modified version of that pointer into a global structure.
- `increment_pointer_in_structure`: Sets d0 to 97, loads a pointer from offset 8 of a structure in a5 into a3, and if the value at offset 12 of that structure is non-zero, increments a3 by 4.
- `parse_string_array`: Pushes two longwords from offsets 4 and 8 of a structure in a2 onto the stack, pushes the value of a3, calls another routine, restores a2 and a3 from the stack, and returns.
- `concatenate_string_array_to_buffer`: Iterates through an array of null-terminated strings to calculate the total required length, then allocates a memory block and copies the strings into it with space characters inserted between them and a newline character appended at the end.
- `find_directory_path`: Iterates through a string looking for a directory or file separator and checks for the existence of a task or directory or file using library calls.
- `execute_pointer_list`: Iterates through a linked list of 4-byte pointers, and if a byte at an offset of the pointer has bit 2 cleared and bit 1 set, and the following two 16-bit words are not equal, calls another routine, then calls another routine once more after the loop.
- `handle_io_error_status`: Checks a pointer, executes a routine, and if an error or a specific return value is detected, updates a status or value in a global structure.
- `update_pointer_and_seek`: Checks a pointer, clears a global, and executes a function pointer while potentially updating a pointer or a value based on a parameter.
- `cleanup_display_context`: Conditionally invokes a callback when a field at offset 10 of the structure in a1 is nonzero, then writes reset values to offsets 8 and 14 of that structure.
- `check_and_execute_function`: Checks if a value at offset 480 of the structure in a4 is nonzero, then calls a function at a fixed offset from a pointer stored at offset 1036 of that same structure.
- `compute_fixed_point_product`: Takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value by multiplying high and low 16-bit parts and combining the results.
- `divide_fixed_point`: Performs a normalized fixed-point division using a 32-bit dividend in d0 and a 32-bit divisor in d1, returning a 64-bit result.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method and returns a 64-bit result containing the quotient and remainder.
- `compare_and_branch_status`: Loads a pointer from an offset of 504 in a structure in a4, reads a byte from the address minus one, and branches if that byte is less than or equal to 79; otherwise it sets d0 and d7 to the value of the byte and branches to another location.
- `process_system_status`: Copies a block of memory, calls a routine via a global pointer, and depending on the result and subsequent checks of a structure, either calls another routine twice or calls a routine with several arguments and updates a global variable.
- `update_and_test_status`: Stores a value at offset 288 of a structure in a5, calls another routine, and if the return value is not zero, sets d0 to negative one; otherwise it sets d0 to zero.
- `update_structure_and_test`: Stores a value at an offset of 288 from a pointer, calls another routine with several arguments, and returns zero or negative one based on the result.
- `return_zero_constant`: Clears register d0 and restores multiple registers from the stack before returning.
- `create_message_node`: Obtains a global variable and calls a function through a table; if the result is not a sentinel, it calls another routine to allocate a node, populates it with parameters, and returns the pointer.
- `initialize_context_and_callback`: Sets a value at offset 456 of a structure, calls several routines from a global variable to retrieve data, and assigns a function pointer to a structure field before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_callbacks`: Calls three different routines via pointers stored at offsets of -354, -384, and -372 from a global variable, updates a global variable with a value from a structure, and assigns this routine to a pointer at offset 176 of another structure.
- `update_global_params`: Stores a value into a global variable, clears another global variable, and calls two routines using pointers retrieved from a second parameter.
- `initialize_string_buffer_and_parse`: Clears a memory block using a loop, then checks a global structure to either extract a string from a pointer or construct a new string by copying bytes from a source and appending specific characters.
- `jump_to_sub_c0c62a`: Performs a long jump to the address a fixed address.
- `jump_to_pointer_update`: Performs a long jump to the routine that updates a pointer structure by setting a value from a stack offset to itself, incrementing it by 4, and clearing the longword at offset 0 of the new pointer.
- `traverse_and_free_list_2`: Iterates through a linked list starting at an offset of 4563 of a structure, calling another routine for each node, and then clears the pointers at offsets 4564 and 4568 of that structure.
- `parse_string_tokens`: Parses a string into a list of pointers within a structure, handling whitespace and quoted substrings that support escape sequences for carriage returns and line feeds.
- `reset_resource_status_flags`: Clears two 16-bit values at offsets 3656 and 2012 of a structure and calls another routine; if the value at offset 3656 remains zero, it calls another routine before returning.
- `initialize_resource_tables`: Sets a 16-bit value to 2 at offset 2012 of a structure, calls another routine, and then enters a loop that runs 16 times to populate a table at offset 3608 with values returned from a routine call, followed by another routine call using a bit-shifted index.
- `initialize_graphics_buffer_and_tables`: Initializes several global state flags and populates arrays with values derived from memory-resident tables.
- `update_string_resource_metadata`: Updates string resource metadata and pointer tables, including clearing or setting certain attribute flags and copying or reallocating memory blocks.
- `initialize_graphics_buffer_and_render_ui`: Clears memory blocks and iteratively updates a buffer or attribute list with specific bit-masked values or constants.
- `save_and_restore_word`: Saves a word from an offset of 1980 of a structure, clears that location, calls another routine with two arguments, and then restores the original value to that location.
- `process_table_loop`: Iterates through a memory-resident list, checking for a specific value and updating a certain memory location if a specific constant is encountered.
- `store_high_word`: Stores the high word of a provided parameter into a global memory location and calls another routine if a 16-bit value at an offset of a structure is zero.
- `update_tile_grid_bounds`: Iteratively updates a rectangular boundary or grid of tile-related values, checking or setting certain flags and recalculating or clamping boundary limits.
- `clamp_input_value`: Returns negative one if the input is less than zero, zero if the input is zero, and one otherwise.
- `update_attribute_masks_and_pointers`: Iterates through a table of pointers and associated values to update a set of memory-resident attributes.
- `initialize_graphics_buffer_and_tiles`: Initializes a series of graphics buffers and tile grid boundaries by iterating through memory ranges and updating global flags.
- `update_attribute_buffers`: Iterates through a memory-resident table to update or copy attribute-related data between buffers.
- `initialize_graphics_and_resource_tables`: Sets up graphics parameters and iterates through resource tables to initialize or update memory-resident structures and string pointers.
- `execute_nested_loops_3`: Performs a nested loop structure that writes 1 to a global variable at offset 2008, calls another routine with varying arguments based on loop counters, and calls another routine with four arguments where the first three are zero, two hundred, and zero.
- `call_and_increment_register`: Calls another routine, increments the value in d7 by 1, and then branches to another location.
- `prepare_buffer_copy`: Calls another routine with four arguments, where the first three are zero, two hundred, and zero.
- `update_resource_pointer_table`: Updates a pointer in a table by reading from an offset-based buffer and checks a condition at an offset in a structure.
- `traverse_string_resource_tree_2`: Recursively traverses a structure or table by loading pointers and processing string-related or offset-based data.
- `shuffle_index_tables`: Initializes two different tables with incrementing values and then performs a series of swaps or exchanges on those tables using a calculated index.
- `initialize_resource_and_audio_subsystems`: Allocates memory, initializes audio parameters, and populates a linked list of resource nodes while setting or clearing global status flags and function pointers.
- `trigger_system_event`: Calls a function pointer retrieved from a global variable and returns zero.
- `update_pointer_with_offset`: Reads a longword from an offset of 4 in a structure in a1, adds it to the longword stored at the base of that structure, and calls three separate routines before returning.
- `process_linked_list_nodes`: Iterates through a linked list, clearing bit 1 in a byte at offset 13 of each node and searching a table for a minimum value based on a field at offset 24.
- `traverse_linked_list_3`: Traverses a linked list, checking bit flags in a node's property byte at offset 14 and updating head or certain pointers while removing nodes from the list.
- `iterate_and_modify_bit`: Iterates a loop a certain number of times based on a value in a pointer, calls another routine, and potentially clears a bit in a global byte if a condition is met.
- `process_encoded_instructions`: Decodes a stream of bytes to update a running counter, modify a pointer, or clear a bit in a global register-based bitmask.
- `calculate_line_rendering_parameters`: Calculates line-drawing or primitive-rasterization parameters by computing start/end coordinates and error-correction or stepping-bit offsets.
- `find_and_prepend_to_list`: Iterates through a list of structures up to eight entries long to find an entry where the second bit of a flag byte at offset 13 is clear.
- `update_linked_list_node_properties`: Clears the fifth bit of a flag byte at offset 14 of a structure, then calls find_and_prepend_to_list.
- `process_encoded_string_sequences`: Iterates through a table of pointers to process or skip encoded string and control sequences.
- `check_available_memory`: Calls a library function to check available memory and returns a boolean indicating if the result exceeds 4999.
- `setup_pointer_chain_4`: Allocates memory via a library call, sets a byte at offset 8 of the new memory to 4, and calls another routine with a pointer offset by 20.
- `free_memory_if_nonzero`: Checks if a passed pointer is not null, and if so, calls a library function to free memory using a pointer from a global variable.
- `free_two_pointers`: Calls a library function to free memory using a pointer stored in a global variable and then calls it again using a second pointer.
- `free_pointer_list`: Iterates through a list of four pointers starting at offset 4 of a structure, calling another routine for each non-null pointer, and then calls a library function to free a global pointer.
- `conditionally_free_memory`: Checks if a passed pointer is not null and if a certain offset within that pointer contains a non-zero value, then calls a library function to free memory.
- `free_memory_from_pointer`: Checks if a passed pointer is non-null, and if so, calls a library function to free memory using a pointer stored in a global variable.
- `call_free_mem_if_valid`: Checks if a pointer passed as an argument is non-null and, if so, calls the memory deallocation library routine using an offset from a global pointer.
- `traverse_and_free_recursive`: Recursively traverses a pointer-based structure, checking a bitmask in a byte at an offset, and calls a deallocation routine while decrementing a counter in a structure.
- `process_recursive_node_flags`: Calls another routine, and if the returned pointer is non-zero, checks bit flags in a byte at offset 14 of the structure and calls another routine for specific flag values, then decrements a 32-bit value at offset 10 and calls a routine via a pointer in a global variable.
- `push_and_call_routine`: Pushes a 32-bit value from offset 6 of a structure in a2 onto the stack, calls another routine, and then branches to another location.
- `call_free_memory_pointer`: Pushes a longword from offset 6 of a structure in a2 onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `push_pointer_and_call_routine`: Pushes a longword from offset 6 of a structure pointed to by a2 onto the stack, calls another routine, adjusts the stack pointer, and branches to another location.
- `push_pointer_and_call_routine_2`: Pushes a 32-bit pointer from offset 6 of a structure in a2 onto the stack, calls another routine, adjusts the stack pointer, and branches to another location.
- `decrement_and_free_memory`: Recursively calls itself with a pointer from offset 6 of a structure, then if the longword at offset 10 is non-zero, decrements it and calls a library function to free memory, before calling the library function again.
- `decrement_loop_with_delay`: Calls a library function to disable interrupts, enters a loop that executes four delay loops for every iteration of a counter passed in a parameter, calls another routine, and finally calls a library function to enable interrupts.
- `initialize_audio_subsystem`: Copies a string into a buffer, allocates memory for an audio structure, and initializes several fields including function pointers and configuration constants.
- `iterate_and_restore_pointers`: Traverses two linked lists by following pointers at offsets 0 and 4 of the nodes, calls another routine on each node, and restores the original pointer value to the list.
- `restore_and_free_pointers`: Checks if a short word at offset 1528 in a structure in a4 is non-zero, then calls a routine via a global pointer, calls another routine, and if a pointer at offset 1534 of a structure in a5 plus 144 is non-zero, calls a routine and clears that pointer; it also clears two other pointers at offsets 1692 and 1688 of the structure in a5 and clears the short word at offset 1528.
- `update_table_and_bitmask`: Iterates through a table to copy data between memory locations and increment a value, then sets a bitmask in the table based on the table size.
- `parse_resource_stream`: Parses a stream of tagged data to allocate memory, copy strings, and process data segments.
- `parse_resource_buffer`: Parses a buffer of type-prefixed data blocks to allocate memory and copy or modify data or strings.
- `allocate_and_copy_memory`: Allocates memory and copies a certain amount of data from a source pointer to the new memory, updating a pointer in a table.
- `parse_resource_tags`: Iterates through a buffer of tagged data, allocating memory and building a linked list or copying strings based on specific magic identifiers.
- `search_linked_list`: Traverses a linked list to find a matching string and potentially reorders or moves a node to the head of a list.
- `insert_into_list_2`: Checks a flag in a structure and inserts a new node into either the beginning or the end of a linked list, while potentially updating a table and clearing certain bits or values.
- `calculate_string_length_recursive`: Calculates the length of a null-terminated string and recursively processes a pointer if a non-zero length is found.
- `parse_resource_stream_2`: Parses a stream of tagged data to allocate memory, copy strings, and process data segments.
- `modify_bitmask_and_recurse`: Calls another routine and, if the return value is non-zero, clears a bit in a byte at an offset and calls another routine recursively.
- `clear_linked_list_and_values`: Iterates through a linked list to clear a specific bit in a certain memory offset and clears certain values or pointers in a table.
- `invoke_nested_routines`: Executes a sequence of calls to another routine, passing a global variable and certain constant values, then calls that same routine three more times without arguments.
- `allocate_and_link_node`: Copies a block of data from a global source, allocates memory, and inserts a new node into a linked list while updating a counter.
- `initialize_clipped_bitmap`: Checks if a specified rectangular area or pointer-based structure falls within certain bounds and initializes a bitmap.
- `update_display_clip_bounds`: Checks if a rectangle defined by a pointer intersects with a predefined boundary table; if valid, it updates a table of memory offsets using a loop and calls several subroutines via a function pointer table.
- `check_bit_mismatch`: Compares the lower bits of a value passed in a register and calls another routine if they do not match.
- `scatter_bytes_to_buffers_2`: Copies bytes from a source pointer to multiple destination buffers using a nested loop structure.
- `copy_block_unrolled`: Uses an unrolled loop to copy a block of data from one memory location to another using a calculated destination address.
- `copy_rect_buffer_blocks_2`: Performs a block memory copy or transfer if a source and destination rectangle or certain boundary limits are valid.
- `copy_large_buffer`: Calculates source and destination pointers based on certain bounds and copies a large block of data using a loop with large offsets.
- `clear_two_words`: Clears two 16-bit words at specific offsets within a structure pointed to by a6.
- `clear_large_memory`: Clears a large block of memory by setting a large number of bytes to zero, using a pointer derived from a global variable.
- `clear_subsystem_function_pointer`: Checks if a function pointer at offset 2002 of a global structure is non-zero and, if the 7th bit is not set, executes that function before setting the pointer to zero.
- `copy_fixed_bytes`: Copies sixty-three bytes one at a time from one pointer to another.
- `execute_indirect_function_3`: Retrieves a pointer from a global structure at offset 4640, executes the function at that address, and if the return value is non-zero, executes a second function from an offset of 36 in that same retrieved pointer structure.
- `call_delay_routine`: Checks if a parameter is non-zero and then calls a routine retrieved from a pointer stored in a global structure.
- `validate_and_reset_object_pointer`: Checks if the value returned by another routine is zero to decide whether to retry or proceed, and if non-zero, retrieves a pointer from an offset in a table and sets that pointer to zero.
- `release_subsystem_resources`: Checks if a function pointer at offset 2002 of a global structure is non-zero and executes it if so.
- `search_and_store_string_index`: Iterates through a string and stores a returned value into a global variable, potentially calling a function via a pointer.
- `execute_graphics_setup`: Retrieves a pointer from a global variable and calls a sequence of three different graphics-related routines.
- `retrieve_table_pointer_2`: Retrieves a pointer from a table using an index, then calls another routine.
- `recursively_traverse_table`: Recursively calls itself using a value retrieved from a table or pointer, checking a bitmask in a structure before proceeding.
- `update_attribute_values`: Iterates through a buffer and updates specific bit-masked attributes in another buffer, then calls a delay routine if any changes were detected.
- `validate_string_resource_lock`: Verifies a string index, attempts to lock a resource, and if successful, calls another routine to unlock it and returns one; otherwise it returns zero.
- `execute_indirect_function_sequence`: Retrieves a pointer from a global structure and executes a sequence of three indirect function calls located at fixed offsets from that pointer.
- `toggle_resource_lock`: Calls a routine to lock a resource and then calls a corresponding routine to unlock it if the lock was successful.
- `find_string_index`: Iterates through a table of strings to find a case-insensitive match for a provided string and returns the index or a failure code.
- `search_and_update_list`: Iterates through a linked list to find a matching pointer, updates a node, calls a function, and decrements a global counter.
- `process_list_and_strings`: Iterates through a table to call another routine using pointers, clears a value in a global structure, and then iterates through a list of pointers to call another routine.
- `search_string_table_2`: Iterates through a table of pointers and performs a case-insensitive string comparison against a provided parameter, returning the index if a match is found or a null terminator is encountered.
- `process_string_and_sort_data`: Initializes a table, iterates through a null-terminated string, and if certain bits are set, populates and performs a bubble sort on a temporary buffer containing indices and modified bytes.
- `call_add_int_server_3`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `divide_shift_register_output`: Reads a value from a shift register routine, divides it by a computed range derived from the input, and returns the quotient and remainder combined into a single value.
- `subtract_structure_values`: Subtracts a 16-bit value at offset 38 from a 16-bit value at offset 34 of a structure in a6 and returns the result.
- `process_table_entry_loop`: Saves d7 to the stack, checks if d7 is non-zero, otherwise calls another routine, calls a third routine, and loops by masking the lower byte of d0 into d7 and branching back.
- `return_d7_value`: Moves the contents of d7 into d0 and returns.
- `call_and_check_result`: Clears d7 and, if d7 is non-zero, calls another routine, then checks if the result in d0 is non-zero to decide whether to move d0 to d6 or set d0 to zero.
- `increment_modulo_counter`: Enters a loop where it calls another routine, and if the lower byte of the result is zero, increments a 16-bit counter at offset 2106 of a structure and applies a modulo 16 operation.
- `restore_and_return_d7`: Moves the contents of d7 into d0, restores d6 and d7 from the stack, and returns.
- `traverse_recursive_loop`: Performs a recursive traversal of a structure or linked list, updating a value or pointer with the return value of each recursive call.
- `fetch_table_value_loop`: Saves d7 to the stack, checks if d7 is non-zero, otherwise calls another routine, calls a third routine, and loops by moving the result from d0 to d7 and branching back.
- `process_next_table_entry_2`: Calls another routine and then moves the value from register d7 to d0 before returning.
- `test_table_value`: Loads a 32-bit value from offset 8 of a structure in a5, tests if it is zero, and if it is non-zero, branches to another routine; otherwise, it clears d0 and branches to a different location.
- `decode_table_value_2`: If a 16-bit value in d7 has its lower byte less than 76, it writes several values to a structure in a5 and calls another routine; otherwise, it masks the value to 7 bits and performs a series of range checks to either add a constant or fetch a byte from a table at an offset from a4.
- `resolve_table_index_with_bit_modifiers`: Masks a 16-bit value to a 7-bit range and uses it to fetch a byte from one of several tables via an offset in a pointer.
- `modify_register_value_2`: Tests bit 3 of a register and, if it is clear, subtracts 45 from a register before branching to another routine; otherwise it branches to another routine.
- `modify_register_value`: Copies a long value from d4 to d0, performs a bitwise AND on d0 with 48, and if the result is zero, subtracts 45 from d6; otherwise it branches to another location.
- `compare_and_offset_value`: Compares d5 against 112 and 113, and if d5 is not within that range, copies d5 to d6 and adds 86 to d6 before branching to another location.
- `decode_table_value`: Reads a byte from a table based on an offset and applies several conditional bitwise-based offsets to the result.
- `map_input_to_output`: Reads a byte from a table using a masked index and applies conditional bitwise-based offsets to the result based on certain bits in the input.
- `process_next_table_entry`: Locks a resource, increments a counter, reads a 32-bit value from a table, and potentially stores the lower or upper halves of that value into a global structure before unlocking.
- `fetch_table_value`: Returns a 32-bit value from a table using an index if a certain condition is met.
- `decompress_compressed_stream`: Unpacks a custom-format compressed data stream from a source buffer into a destination buffer using a variable-length instruction-based decoding scheme.
- `traverse_string_resource_tree`: Recursively traverses a data structure by loading pointers from a table and calling itself to process nested elements, string indices, or encoded lengths.
- `scatter_bytes_to_buffers_3`: Copies bytes from a source pointer retrieved from a table to multiple destination buffers using a nested loop structure.
- `copy_memory_block`: Copies a block of memory from a source address, calculated via a table lookup, to a destination address using a nested loop structure.
- `update_node_on_list_match`: Validates a list or table entry and, if certain conditions or pointer matches are met, updates a pointer in a structure and executes a cleanup routine.
- `scatter_bytes_to_buffers_7`: Pushes d2, d3, and the word at offset 12 of the stack to the stack, loads a pointer from offset 20 of the stack into a0, calls another routine, and returns.
- `allocate_and_link_node_3`: Allocates a block of memory, initializes it with a repeated value, and sets a header containing a size and a pointer.
- `traverse_linked_list`: Iterates through a linked list starting from a pointer in a global variable, calling another routine on each node, and finally clears a pointer in a global variable.
- `call_two_functions`: Resets two global short integers to zero and calls two function pointers retrieved from a table.
- `resolve_string_resource_index`: Searches a string table for a specific index and copies the resulting string into a local buffer.
- `update_matching_node`: Checks a table for a specific index and, if a match or certain condition is met, updates a pointer in a structure and clears a global pointer or returns a status.
- `find_max_index`: Iterates through a table of structures to find the index of an entry that satisfies certain conditional checks involving a comparison or a range.
- `cleanup_subsystem_resources`: Invokes several function pointers from a global structure or table to clear or free display or context-related resources.
- `initialize_resource_node_list`: Allocates and initializes a linked list of data structures, populating nodes with pointers or allocated memory blocks.
- `call_avail_mem_2`: Library stub: loads its arguments and the exec library base, calls AvailMem and returns its result to the caller.
- `shift_bits_in_bytes`: Performs bitwise shifts and logical OR operations while updating a sequence of three bytes in memory.
- `execute_cleanup_on_pointer`: Checks if a function pointer at offset 2002 of a global structure is non-zero and, if so, executes a function pointer from a global structure.
- `apply_bitmask_to_buffer`: Iterates through a buffer and uses bit-testing and bit-setting instructions to modify certain bits based on a lookup table.
- `call_rem_int_server_3`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `compare_string_list`: Iterates through a list of pointers to compare strings and returns if a match or a specific flag is found.
- `fetch_table_entry_and_call`: Loads a 16-bit index and a 32-bit pointer from a table, populates registers with three 16-bit words from the table, and calls recurse_table_index.
- `recurse_table_index`: Recursively calls itself or another routine using a value fetched from a table-based offset.
- `copy_and_call_function`: Copies a block of 64 bytes to a specific destination and then calls an external function twice.
- `shuffle_table_data`: Performs a complex, nested loop that swaps values in a table using a temporary register, orp, and updates a flag or pointer.
- `update_index_buffer`: Writes a register value to a table at an offset determined by a global index, and increments or resets that index if it exceeds a certain value.
- `process_string_and_colors`: Calls two routines via pointers in a structure in a4, then iterates through a null-terminated string, calling a routine for each character and updating a 16-bit value based on a pointer in a structure at offset 2048.
- `process_string_pointer_2`: Copies the value from a pointer in a3 to a1, calls another routine, and then checks a global variable at offset 820 of a4 to either load a pointer into a0 or branch.
- `process_string_pointer`: Iterates through a null-terminated string using a pointer from a structure in a5, calls another routine for each character, and updates the pointer in the structure by incrementing it if the character is not zero.
- `increment_pointer_value`: Adds 8 to d6, copies the value in a2 to a0, stores a0 at an offset of 8 from a5, and if the byte at a0 is non-zero, increments the value at the offset of 8 from a5.
- `restore_registers_and_return_2`: Restores several registers from the stack, unlocks the address register a5, and returns.
- `copy_string_with_conditional_logic`: Parses a source buffer to find a length, then copies a certain number of bytes from a source to a destination buffer, handling different length or sign-based logic.
- `execute_indirect_function`: Checks if a 32-bit value is non-zero, then calls a function pointer retrieved from a table offset by a constant.
- `parse_encoded_length`: Checks a value at a pointer and recursively calls itself to calculate a length or skip certain bytes.
- `update_input_state`: Decrements counters in a structure, updates a global variable based on bits in a hardware register, and modifies several values in a structure based on comparison results.
- `process_input_and_update_table`: Modifies a hardware register based on certain bit patterns and updates a table or a global index using a loop.
- `call_indirect_via_stack_offset`: Loads a function pointer from an offset of 348 bytes relative to a value stored on the stack and calls it.
- `update_pointer_structure`: Sets the value at a pointer from a stack offset to itself, increments that value by 4, clears the longword at offset 0 of the new pointer, and stores the new pointer at offset 8 of that same location.
- `traverse_pointer_chain_2`: Recursively traverses a pointer chain and returns a failure constant if a certain global value is non-zero or if a certain bit in a byte is detected.
- `validate_resource_pointer`: Loads a pointer from offset 4592 of a structure in a4, calls a routine at an offset of minus 72 from that pointer, and if the result is zero, calls another routine at an offset of minus 132 from that pointer, stores the result in a 16-bit field at offset 4988 of the structure in a4, sets a 16-bit value at offset 4588 of the structure in a4 to 2, and returns -1; otherwise it returns 0.
- `set_d0_constant`: Moves the value from d0 to d5 and performs five bitwise OR operations with zero on d0 before setting d0 to 97.
- `execute_cleanup_callbacks`: Iterates through a table of function pointers and calls them if certain bit-flags or null-checks allow.
- `jump_to_address`: Performs a long jump to a specific address.
- `convert_string_case`: Iterates through a null-terminated string and modifies characters by subtracting 32 if a bitmask check on a global table passes.
- `copy_string_with_limit_2`: Calculates the length of a source string, determines the minimum between that length and a provided limit, and copies that number of bytes to a destination buffer before appending a null terminator.
- `compare_case_insensitive_strings`: The routine compares two byte strings character by character, converting uppercase letters to lowercase for comparison, and returns the difference between the first mismatching characters or zero if equal.
- `copy_string_to_buffer`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `find_byte_in_string`: Searches a null-terminated string for a specific byte and returns a pointer to its location, or zero when the byte is not present.
- `search_string_byte`: Searches for a specific byte within a null-terminated string.
- `find_last_occurrence`: Iterates through a null-terminated string to find the last occurrence of a specific byte and returns its pointer.
- `free_memory_wrapper`: Calls a routine to free memory.
- `traverse_pointer_chain`: Recursively traverses a pointer chain or updates an offset, returning a failure constant if a certain global value is non-zero or if the initial pointer is null.
- `traverse_pointer_chain_3`: Recursively traverses a pointer chain while checking a bitmask in a byte or detecting a null pointer.
- `handle_io_error`: Checks a flag, clears a 16-bit value in a global variable, and if a function pointer returns -1, calls another routine to update the global variable and sets a value at an offset to 5.
- `seek_and_update_offset`: Executes a function pointer and, depending on a parameter, updates a value or calls a function again to recalculate a pointer.
- `initialize_structure_fields`: It writes a full-byte bitmask at a fixed offset of a structure and writes full-word bitmasks at two other offsets.
- `cleanup_display_context_2`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `execute_conditional_function`: Checks a global pointer, potentially calls another routine, and then executes an indirect function call.
- `compute_fixed_point_product_2`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `negate_registers_and_return_2`: Tests d0 and d1, negates them if they are negative, calls another routine, and then negates d1 again before returning.
- `negate_registers_and_return`: Calls another routine, negates the values in d0 and d1, and then returns.
- `negate_conditional_d1`: Tests d1, and if it is negative, negates it, calls another routine, negates d0, and returns.
- `calculate_reciprocal_fixed_point_2`: Performs a complex calculation involving bit-shifting and division to compute a reciprocal or a related fixed-point value based on input reg.
- `calculate_reciprocal_fixed_point`: Performs a complex calculation involving bit-shifting and division to compute a reciprocal or a related fixed-point value based on input registers.
- `compare_and_branch_status_alt`: Loads a pointer from an offset of 1580 in a structure in a4, reads a byte from the address minus one, and branches if that byte is less than or equal to 79; otherwise it sets d0 and d7 to the value of the byte and branches to another location.
- `execute_indirect_call_2_2`: Copies a block of memory, sets a byte in a structure to zero, calls another routine, and then performs conditional logic involving further indirect calls and structure updates.
- `execute_indirect_call_2`: Calls a routine using a pointer from a6, stores the result in a structure at offset minus 100 from a5, and branches based on the result.
- `execute_indirect_call`: Stores a value at an offset of a structure, calls another routine with seven arguments and a pointer, and returns zero or negative one based on the result.
- `return_zero_constant_2`: The routine returns a constant zero value without performing any other operations or memory accesses.
- `allocate_and_set_node`: It allocates memory for a structure through a library call and, if successful, initializes certain bytes and a dword with a provided parameter.
- `create_message_node_2`: It obtains a global variable and calls another routine through a table.
- `unnamed_routine`: This routine is called by two unnamed routines and consists of 18 instructions.
