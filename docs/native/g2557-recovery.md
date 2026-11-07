# Game 2557 (king's bounty (1990)(new world computing)[cr csl][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02557). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 94 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 236 of 236 named (0 author symbols), 236 of 236 described (55 by rule, 44 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4013 of 4013 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 653017 |
| code | 12380 |
| data | 181255 |
| padding | 23748 |
| unused | 30720 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Binddrivers | 11 | 652 | 35 |
| EndCLI | 2 | 117 | 2 |
| King's Bounty (1990)(New World Computing)[cr CSL][a].adf:King's Bounty | 2 | 267 | 8 |
| Disk-Validator | 1 | 386 | 1 |
| 8 | 3 | 2 | 1 |
| icon.library | 10 | 1375 | 109 |
| diskfont.library | 15 | 1040 | 76 |
| King's Bounty (1990)(New World Computing)[cr CSL][a].adf:bootblock | 1 | 174 | 4 |

## Cases handled

- king's bounty (1990)(new world computing)[cr csl][a]/King's Bounty (1990)(New World Computing)[cr CSL][a].adf:King's Bounty: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Binddrivers

- `parse_command_line`: Allocates a control block and, when the program was started from the command line, splits the raw argument text into zero-terminated strings stored in a linked list.
- `bind_expansion_drivers`: Opens the expansion and icon libraries, then reads a list of driver entries and for each one derives a name, finds the matching expansion device and binds its driver.
- `find_self_referencing_resident_tag`: Walks forward through a memory block looking for a resident tag, a marker word whose stored address points back to the tag itself.
- `accumulate_pipe_separated_fields`: Walks a string of entries separated by vertical bars, measuring each entry and passing its start and length to another routine that updates an accumulated value.
- `parse_numeric_path_segment`: Splits a name at its first slash and checks whether the leading part is all digits.
- `collect_flagged_nodes_into_list`: Walks a list of nodes and, for each node with a particular flag bit set that is not already in the output list, pushes it onto the front of that list through its link field.
- `follow_parent_chain`: Walks a chain of nodes through the link field at a fixed offset, stopping at a null node or at the target node.
- `check_digit_prefix`: Checks the first given number of bytes of a string for decimal digits.
- `parse_decimal_integer`: Reads consecutive decimal digit characters from a string, accumulating their value, and stops at the first non-digit or the terminator.
- `check_result_within_limit`: Calls another routine with the first argument and a sign-extended byte, then returns zero if the first argument plus the third argument is less than that result.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_alert`: Library stub: loads its arguments and the exec library base, calls Alert and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `compare_strings_unsigned_order`: Compares two zero-terminated strings byte by byte as unsigned values.
- `measure_zero_terminated_length`: Counts the bytes before the zero terminator by counting an accumulator down from negative one and returning the complement of the final value.
- `measure_string_length_in_register`: Counts the bytes before the zero terminator by counting an accumulator down from negative one and returning the complement of the final value.
- `locate_char_in_string`: Scans a zero-terminated string from its start for a given byte and returns a pointer to the first match.
- `copy_c_string`: Copies bytes from a source pointer to a destination pointer until a zero byte is copied.
- `compute_range_adjusted_offset`: Reads a pair of values and checks that the high one does not exceed the low one, then runs a further check.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.

### EndCLI

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_saved_state_and_call`: Copies sixteen longwords from a fixed offset in a base structure into a caller buffer when one is supplied, then calls a routine through a pointer.

### King's Bounty (1990)(New World Computing)[cr CSL][a].adf:King's Bounty

- `load_executable_hunks_and_relocate`: Walks the hunk records of an executable image, allocating memory for each code, data and BSS section and applying 32-bit relocation records that patch stored addresses by the section load base.
- `spin_countdown_delay`: Counts a 16-bit register down through its full range in a loop, which wastes time without doing other work.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_backward_compressed_stream`: Decodes a bit-packed stream read through other routines, writing bytes from the end of a buffer toward its start.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bit_stream_into_accumulator`: Shifts a bit stream held in a register pair one bit at a time, feeding each bit shifted out into an accumulator through the extend flag and refilling from memory when the stream is empty.
- `stub_bad_instruction_data`: Only a short trap fragment decoded here, which copies four longwords into global storage before the decoder hits bad instruction data.

### Disk-Validator

- `check_disk_block_bitmap`: Sets up a check state and walks a range of block numbers, testing bits in a bitmap table against values read from the disk structure and recording mismatches.

### 8

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Increments a 16-bit counter stored at one offset in a structure and clears a single flag bit in a byte at another offset.
- `decrement_library_refcount`: Decrements a 16-bit reference count in a structure.
- `unlink_node_or_flag_pending`: If a 16-bit field of the node is zero, it unlinks the node from its doubly linked list and calls a library routine with the node's stored value, returning that result.
- `set_pending_flag_bit`: Sets the pending bit in a flag byte of a structure and returns zero.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `store_library_open_result`: Stores two incoming values into a structure, then calls a library routine to look up a library.
- `copy_two_fields_to_globals`: Copies two longword fields from a structure, at fixed offsets from a register-held base, into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `pass_call_through`: Calls one routine and returns without any other operation.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `call_icon_data_copy`: Calls one helper routine that copies icon data and returns without any other operation.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_list_handler`: This library routine calls a handler function pointer stored in a data structure and then returns.
- `call_library_handler_2`: This library routine calls another routine with a fixed offset applied to the second argument, then returns.
- `call_library_helper_wrapper`: Calls one helper routine and returns without any other operation.
- `invoke_icon_setup`: This library routine calls a single initialization function to set up icon data structures and then returns.
- `invoke_handler_pointer`: This library routine retrieves a function pointer from a data structure and calls it.
- `call_library_handler_3`: This library routine calls another routine and then returns.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_check_bar_list`: The routine calls a routine that checks a list of bar-separated entries for a matching item and then returns.
- `call_other_routine_2`: The routine calls another unable to be named routine and then returns.
- `update_data_field`: Calls a library routine through a function table on the structure, then stores the second argument into a field of the returned block at a fixed offset.
- `attach_or_release_resource`: Obtains a resource from a first routine, passes it to a second routine to attach it, and releases it through a further call if the attach step fails.
- `copy_stack_record_into_object`: Allocates a block, reads a record into a stack buffer, and copies its ten longwords and several fields into the block.
- `copy_frame_fields_into_object`: Copies ten longwords and several fields from a local frame into an object at a fixed offset.
- `return_zero_stub_2`: The routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: Copies ten longwords and several other fields from a structure into a local stack buffer, prefixes it with a constant header word, and then calls another routine with that buffer.
- `load_icon_info_file`: Builds the path of an icon file by appending the info extension, reads its header and checks the magic and version words.
- `read_icon_image_blocks`: Checks an icon header's magic and version words, then reads the image blocks into allocated memory and links them to the icon structure.
- `read_icon_secondary_blocks`: Reads two values into the icon structure, then loads any of three optional blocks that are present, allocating memory and reading longword entries into each.
- `read_longword_array_block`: Reads longword entries into a caller-supplied array until the remaining count drops to four or less, then loads an optional final block.
- `call_routine_return_zero`: Calls one routine and then returns zero.
- `empty_routine_2`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `write_icon_info_file`: Builds the icon file name with the info extension in a path buffer, opens the file for new writing and writes the icon header and its optional blocks.
- `write_icon_info_blocks`: Writes the header and optional blocks of an icon structure to an open file through a write routine.
- `write_icon_optional_blocks`: Writes the optional icon blocks, each preceded by its length, through a write routine.
- `write_pointer_table_blocks`: Writes the entries of a pointer table that is terminated by a zero pointer, each with a length word, then writes one more optional block.
- `write_null_terminated_entry_table`: Walks a null-terminated list of strings, writing each string's length and characters through a write routine, then writes one more optional block.
- `free_linked_list_nodes`: Walks a linked list from a node at a fixed offset, releasing each node through a routine until the list ends or returns to its head.
- `iterate_linked_list_nodes`: Calls a routine on each node and advances to the next node through a routine that returns it.
- `call_with_offset_argument`: Calls another routine with the same first argument and the second argument increased by a fixed offset, then returns.
- `allocate_and_attach_sized_block`: Allocates a cleared block of a fixed size, passes a pointer into it to another routine, and returns the block when that call succeeds.
- `refill_table_entry`: This library routine calls another routine to refill a table and returns zero if the refill fails.
- `return_zero_2`: The routine returns a zero or null value.
- `store_entry_with_refill`: Decrements a 16-bit counter in a structure and, when it runs out, asks another routine to refill the table, returning zero if that fails.
- `decrement_counter_and_store_pair`: Decrements a 16-bit counter through a pointer and stores two register values into the table entry it indexes, returning one.
- `allocate_and_attach_record`: Allocates a cleared record of a fixed size, runs two attach steps on it through other routines, and releases the record and returns zero if either step fails.
- `empty_routine`: Does nothing and returns immediately.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `incremental_recursive_call`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `find_value_after_equals`: Walks a null-terminated array of strings and returns the text following an equals sign in the first entry that matches the given name.
- `find_entry_after_equals`: Walks a null-terminated array of pointers, testing each entry, and returns the position after the equals sign in the first entry that passes.
- `find_value_in_string_array`: Walks a null-terminated array of strings, testing each entry through a routine, and returns the text after the equals sign of the first entry that passes.
- `check_bar_list_for_item`: Walks a list of entries separated by vertical bars and returns one when an entry has the same length as the wanted item and matches it through a compare routine.
- `match_bar_item_from_start`: Walks bar-separated entries from a starting position and returns one when an entry has the expected length and its content matches.
- `match_bar_separated_entry`: Steps through bar-separated entries, comparing each entry's length and content with the wanted item.
- `search_list_entries`: This library routine iterates through a list of entries and checks if certain conditions or lengths match, returning one if a certain condition is met.
- `search_bar_entry_length`: Library code that iterates through a list of entries, checking if an entry has a specific length and if a certain value matches a calculated difference.
- `build_copy_numbered_name`: Builds a copy name for an entry by reading any copy number already in the source, incrementing it and writing the result with its label text into a 30-byte name field.
- `parse_copy_number`: Checks for the copy label in a name and parses the decimal number that follows it.
- `parse_trailing_decimal_number`: Parses consecutive decimal digits from a string into a number and, when that number is nonzero, checks the text that follows it through a routine.
- `parse_copy_count_text`: Parses consecutive decimal digits from a string into a number and checks the text that follows through a routine.
- `decode_decimal_digit`: Returns the value of a character that is a decimal digit, or minus one for any other character.
- `return_minus_one_stub`: Returns minus one immediately and does nothing else.
- `call_alloc_mem_stub`: Calls the exec allocate-memory function through the library base, with its arguments already in the registers, and returns its result.
- `call_free_mem_stub`: Calls the exec free-memory function through the library base, with its arguments already in the registers, and returns.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `allocate_counted_block`: Allocates a cleared block of ninety-six bytes, stores a count of ten in the block and in the caller's word, and calls another routine with the block.
- `return_zero_value`: The library routine returns a constant zero.
- `check_result_matches_argument`: Calls another routine with three arguments and returns its result when it equals the fourth argument, otherwise zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `call_library_vector_stub`: This library routine calls another library function and checks if the returned value matches a passed parameter, returning zero if they do not match.
- `return_empty_stub`: The library routine performs an empty return.
- `divide_unsigned_long_with_remainder`: Divides a 32-bit value by a 32-bit divisor, using a bit-by-bit loop when the divisor is above sixteen bits and direct 16-bit divide steps otherwise.
- `shift_subtract_divide_loop`: Performs sixteen shift-and-subtract steps on a 32-bit value against a divisor held in a register, leaving the partial remainder and quotient in the registers.
- `multiply_32_bit_integers`: Multiplies two 32-bit values from registers using 16-bit partial products and returns the low 32 bits of the product.
- `call_raw_do_format_stub`: Calls the exec formatting routine through the library base, with its arguments already in the registers, and returns.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_char_in_string`: Library code that searches for the first occurrence of a specific byte within a null-terminated string and returns a pointer to that byte.
- `return_zero_stub`: Returns zero immediately and does nothing else.
- `copy_c_string_2`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `concatenate_c_string`: Advances to the zero terminator of the destination string and copies the source string, including its terminator, onto the end of it.
- `strings_differ_within_count`: Compares at most a given number of bytes of two strings and returns one as soon as a byte differs.
- `return_constant_one`: Library code that returns a constant value of one.
- `advance_bounded_string_then_call`: Walks a zero-terminated string for at most a given count of bytes, and when the terminator is reached it calls another routine.
- `copy_string_byte`: copies a single byte from a source pointer to a destination pointer and increments or decrements the pointers.
- `copy_string_with_padding`: Copies bytes from a source string into a destination buffer up to a given count, stopping after the terminator and filling the rest of the count with zero bytes.
- `return_immediately`: This routine is empty and performs no operations before returning.
- `initialize_empty_list_header`: Initializes an empty doubly linked list header: the head points at the tail slot, the tail is cleared, and the predecessor points back at the head.
- `process_icon_request`: Returns one for a null argument.
- `process_linked_entries`: Walks a linked list of entries, calling a routine on each entry and then on its data pointer, and returns zero at the first failure.
- `check_linked_entries_pair`: Walks a linked list of entries, applying two routine checks to each one, and returns one when every entry passes or zero at the first failure.
- `return_zero`: This routine, part of the icon library, returns a constant zero value to the caller.
- `allocate_and_attach_node`: Allocates a cleared twenty-byte block, attaches it through two routine calls, stores a value obtained from a third routine into the block, and returns the block.
- `return_max_long`: The library routine returns a constant value of all ones.
- `build_entry_node_list`: Walks a sequence of entries, allocating a node for each one, linking the nodes together and reading each entry's data through routines.
- `link_entry_nodes_from_stack`: Repeats the node-building step over entries whose parameters are held on the stack, linking each new node after the previous one.
- `link_entry_nodes_loop`: Walks entries from a stack argument, storing each link, reading each entry's data through routines and returning the first node, or minus one on failure.
- `return_all_ones`: This library routine returns a 32-bit value where all bits are set to one.

### diskfont.library

- `create_font_entry_and_add`: Calls a library routine to create a font entry.
- `check_and_set_font_handle`: The routine is library code that calls a routine via a table entry using the library base.
- `open_font_libraries_init_list`: Opens the dos and graphics libraries and, when both open, initializes an empty list header inside the structure and returns it.
- `close_library_and_free`: Closes a library base and frees the memory block that holds it, then returns zero.
- `expunge_unused_font_entries`: Walks the font entry list, removing entries whose use count is not positive, unloading their segments and freeing their memory.
- `release_idle_font_structure`: If the use count is zero and no entries remain, it closes the two libraries, frees the structure and returns its stored value.
- `set_pending_flag_return_zero`: Sets the pending high bit in a flag byte of a structure and returns zero.
- `return_void`: This library routine is empty and performs no operations before returning.
- `clear_flag_and_increment_counter`: Clears the high flag bit in a byte of a structure and increments a 16-bit counter stored in it.
- `decrement_and_release_if_pending`: Decrements a 16-bit counter in a structure and, when it reaches zero with the pending flag set, calls a release routine and returns its result.
- `return_zero_3`: This library routine returns a zero or null value.
- `collect_font_name_records`: Fills a buffer with font name records from the loaded font list when one flag is set, and from matching .font files in a directory when another flag is set.
- `copy_font_metadata_strings`: Iterates through a linked list of font records to calculate string lengths and copy name or metadata strings into a destination buffer.
- `populate_font_metadata_buffer`: Iterates through a linked list of font records and copies name or metadata strings into a destination buffer.
- `collect_fonts_from_fonts_dir`: Walks the loaded font list and, when a flag is set, scans the FONTS: directory for font files, writing a record for each match into a buffer.
- `collect_font_file_records`: Walks a list of font file names and, for each entry whose name ends in the font extension, opens the file, reads its records and writes them into a buffer.
- `collect_font_records_directory_pass`: Makes a directory pass over the font entries, reading font file records for each name that ends in the font extension into a buffer.
- `read_font_records_into_buffer`: Copies font name text into a buffer and reads font file records with a read routine, storing one record per match.
- `scan_font_directory_records`: Walks a directory with examine and next-entry calls, and for each entry that is a font file it reads the records into a buffer, releasing its lock and memory afterward.
- `return_zero_stub_3`: Library code that returns zero and performs no operations on registers, memory, or hardware.
- `score_font_entry_match`: Scores how closely a font entry matches a requested font by comparing its size with a stored size and weighing each differing flag bit against a table of values.
- `sum_flag_mismatch_weights`: Subtracts a table weight from an accumulated score for each flag bit that differs between two entries across seven bit positions, and returns the total.
- `sum_flag_mismatch_weights_loop`: Walks the flag bits of two entries, subtracting a table weight from an accumulated score for each mismatch, and returns the score once every bit position has been checked.
- `calculate_weighted_difference`: Library code that subtracts values from a register using a pointer to a table and checks bitmasks during a loop.
- `return_low_sentinel_value`: Returns a fixed large negative sentinel value and takes no arguments.
- `open_font_from_name`: Opens the requested font through the graphics library and, if that fails, searches the FONTS: directory for the best-scoring matching font file and loads it into the font list.
- `select_best_font_in_directory`: Locks the FONTS: directory and repeatedly selects the highest-scoring font entry, loading matches through the file routines.
- `load_best_matching_font_file`: Opens a font file, reads its records and repeatedly selects the highest-scoring entry, loading matches into the font list.
- `select_highest_scoring_font_entry`: Repeatedly scores the candidate entries, keeping the highest-scoring one, and returns the last entry chosen when no candidate remains.
- `find_best_font_entry`: Iterates through a list of font or glyph entries to find the one with the highest score and updates a pointer or table entry with the result.
- `select_best_font_entry`: Iterates through a list of font or glyph entries to find the one with the highest score and updates a pointer or table entry with the selected or even better candidate.
- `close_font_and_select_entry`: Library code that closes a font's graphics handle and frees its memory, then runs the same highest-score selection loop over the remaining entries.
- `build_font_table_from_path`: Library code that takes a file path, replaces its extension with the text .font, and locks the result.
- `load_disk_font_table`: Library code that counts directory entries until none remain, then allocates a cleared memory block for a font table with a 16-bit count and an initial header field.
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
- `copy_null_terminated_string`: Copies bytes from a source string to a destination, including the zero terminator, stopping after the terminator is copied.
- `append_string_at_terminator`: Advances a pointer to the zero terminator of a string and passes that end position and a second argument to the copy routine.
- `skip_string_end_and_call`: Advances a pointer to the zero terminator of a string and then calls a library function through the library base with that pointer.
- `measure_string_length`: Counts the bytes of a zero-terminated string before its terminator and returns the count.
- `count_null_terminated_bytes`: Adds to a starting count while advancing through bytes until a zero byte is reached.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte.
- `compare_string_prefix_ordering`: Compares two zero-terminated strings byte by byte and stops with zero as soon as the first string ends.
- `compare_strings_register_args`: Compares two zero-terminated strings passed in address registers, returning zero when they match and a negative or positive ordering value at the first differing byte.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_forward_in_string`: Scans forward through a zero-terminated string for a given byte.
- `find_char_forward_register_args`: Scans forward through a zero-terminated string for the byte held in a data register, checking the current byte before advancing.
- `find_next_char_after_position`: Advances past the current byte and scans forward for a byte held in a register.
- `find_last_char_in_string`: Measures a zero-terminated string and scans backward from its last byte for a given byte.
- `find_char_backward_bounded`: Scans backward from a pointer toward a lower bound, checking the current byte before stepping back.
- `find_previous_char_bounded`: Steps a pointer back one byte and then checks it against a lower bound and a byte held in a register.
- `skip_path_prefix_to_name`: Looks for a slash in a path, or failing that a colon, and returns the position just after the first one found.
- `increment_returned_value`: Returns the value passed in a data register increased by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `forward_call_to_handler`: Calls a routine reached through a pointer in the library data and returns without any other work.
- `call_external_handler`: This library routine calls another routine and returns.
- `call_checksum_routine`: This library routine calls a routine that calculates a checksum or a string hash and then returns.
- `call_another_routine`: Thin wrapper in the diskfont library that makes one call to another routine and then returns, with no other work.

### King's Bounty (1990)(New World Computing)[cr CSL][a].adf:bootblock

- `check_boot_vectors_and_report`: Boot block entry that takes the processor into supervisor mode, reads the fire and right-button ports, and checks a list of stored addresses against the expected memory ranges.
- `reset_and_call_stored_vector`: Executes the processor reset instruction and then calls the address held in a stored global variable.
- `format_string_to_buffer`: Formats text through the exec RawDoFmt routine into a 256-byte stack buffer, then copies the resulting null-terminated string to the output buffer.
- `append_byte_to_counted_buffer`: Stores one byte after the current end of a length-prefixed buffer and increments the stored count, which stops increasing once it reaches 255.
