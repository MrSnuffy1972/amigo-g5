# Game 1653 (shanghai (1986)(activision)[v systemz v4.0]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01653). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909232 top-level bytes in 106 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 47 of 47 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 905 of 905 named (34 author symbols), 905 of 905 described (122 by rule, 146 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 714 of 714 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 24655 of 24655 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 213565 |
| code | 82792 |
| data | 241475 |
| padding | 21192 |
| unused | 350208 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Shanghai (1986)(Activision)[v SystemZ v4.0].adf:Shanghai | 180 | 17140 | 545 |
| Run | 2 | 117 | 2 |
| Fault | 2 | 117 | 2 |
| Install | 3 | 117 | 2 |
| Stack | 2 | 117 | 2 |
| Delete | 2 | 117 | 2 |
| Ask | 7 | 208 | 10 |
| Type | 2 | 117 | 2 |
| Path | 2 | 117 | 2 |
| RunBack | 3 | 163 | 2 |
| Break | 2 | 117 | 2 |
| Info | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Rename | 2 | 117 | 2 |
| NewCLI | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Quit | 2 | 117 | 2 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| List | 2 | 117 | 2 |
| MakeDir | 2 | 117 | 2 |
| Date | 2 | 117 | 2 |
| LoadWb | 11 | 362 | 22 |
| Protect | 2 | 117 | 2 |
| SetClock | 20 | 1079 | 52 |
| Wait | 1 | 134 | 4 |
| DefDisk | 2 | 207 | 8 |
| Shanghai (1986)(Activision)[v SystemZ v4.0].adf:CLI | 9 | 362 | 18 |
| 12 | 1 | 51 | 2 |
| 16 | 1 | 2 | 1 |
| 9 | 1 | 72 | 2 |
| 8 | 1 | 2 | 1 |
| arp.library | 3 | 88 | 7 |
| icon.library | 16 | 1444 | 110 |
| diskfont.library | 13 | 819 | 66 |
| sound1 | 1 | 2 | 1 |
| sound2 | 1 | 2 | 1 |
| sound3 | 1 | 2 | 1 |
| sound4 | 1 | 2 | 1 |
| sound7 | 1 | 2 | 1 |
| sound8 | 1 | 2 | 1 |
| sound9 | 1 | 2 | 1 |
| extra | 13 | 0 | 1 |
| Shanghai (1986)(Activision)[v SystemZ v4.0].adf:bootblock | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 3 |

## Cases handled

- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_state_and_program_chips` (Shanghai): Copies a 20-byte global block into the first argument, updates a global byte, and stores one bit chosen by the second argument with the third argument in the copied block. Kind: loader/state initialization with hardware-specific register setup. Pass path: the routine proceeds to load data into memory after setting up the custom chip/DMA state.

## Functions


### Shanghai (1986)(Activision)[v SystemZ v4.0].adf:Shanghai

- `program_entry_find_task_open`: Main program entry.
- `release_pending_message_ports`: When a saved value is non-zero, it releases up to two saved handles through the library vector at offset minus 36, then makes exec calls for Forbid, Alert and ReplyMsg.
- `call_two_library_vectors`: Calls two library functions through the library base held in the register, at offsets minus 384 and then minus 372, and returns nothing.
- `open_dos_library_or_release`: Opens the dos library through the library-open vector at offset minus 552 and returns the library base if it succeeds.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `parse_quoted_argument_list`: Splits a command-line style string into up to thirty-two entries, storing each quoted argument's start pointer in a global table and skipping whitespace between them.
- `link_allocated_block_node`: Calls a routine with an offset of 12 from its argument.
- `return_result_or_minus_one`: Calls one routine with its argument and returns the result if it is non-zero, otherwise minus one.
- `return_minus_one_stub`: Returns minus one immediately with no other work.
- `free_linked_memory_blocks`: The routine walks a singly linked list whose head is stored in a global variable.
- `append_node_to_list_tail`: Appends a node to the tail of a list whose header holds head and tail pointers.
- `open_required_system_libraries`: Opens the diskfont, graphics, icon, intuition and layers libraries by name through a shared open routine.
- `allocate_fixed_size_buffers`: Allocates several fixed-size blocks through library calls, including blocks of 400, 880, 200 and 220 bytes, and raises alert code 10 for each allocation that fails.
- `allocate_large_display_buffers`: Allocates a block of 8000 bytes and a block of 16080 bytes through library calls with a fixed flag, raising alert codes 21 and 31 when they fail.
- `load_winter_tile_resources`: Loads three named resources from the winter directory, Doc.Paper, green.felt and tile2, through a loader routine.
- `open_title_display_surface`: Calls a display routine with fixed numeric arguments, and the shanghai.title text is referenced by the call.
- `trace_rectangle_outline_calls`: Issues five calls that take the four parameters in turn, tracing the corners of a rectangle and closing back at the first corner.
- `report_open_failure_error`: Looks up a numeric code in a table and, when it is recognised, reports an error through a formatted message.
- `point_inside_rectangle`: Returns one when the point given by the first two arguments lies inside the four-value bounds structure, including its edges, and zero otherwise.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `compute_bit_masks_once`: On first use it calls a routine with a fixed value and sets a flag.
- `poll_request_completion_status`: Reads a request record through a stored pointer and copies its header into a local buffer.
- `run_setup_sequence_until_ready`: Runs a fixed series of setup calls that configure a display or device, repeating one query until it returns a nonzero value.
- `copy_picture_rows_to_buffer`: Copies a counted list of 40-byte picture rows from a source table into a buffer, locating each row from the width and offset arguments.
- `emit_packed_word_fields`: Walks a range of 16-bit words, splitting each into a high byte and two four-bit fields.
- `reset_thirty_two_entries`: Calls one library routine for each index from zero to thirty-one, passing zeros as the remaining arguments.
- `forward_global_handle_to_routine`: Passes a global handle to one further routine and returns.
- `forward_global_handle_to_library`: Passes a global handle to a routine that calls a library vector at a fixed offset from the library base.
- `select_record_and_set_mode`: Reads a 16-byte table entry chosen by the argument and passes its fields to a routine.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `forward_library_call_132`: Loads a library base from a global and calls the routine 132 bytes below that base, then returns.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `forward_library_call_198`: Loads a library base from a global and calls the routine 198 bytes below that base, then returns.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `format_string_to_output`: Walks a printf-style format string, copying literal bytes and collapsing doubled percent signs.
- `open_timer_device_request`: Creates a message port and an I/O request block, then opens timer.device with that request.
- `prepare_timer_request_block`: Fills a timer request block with command code nine, zeroes one field, and stores a value in another.
- `release_request_structure`: For a non-null argument, passes the structure's embedded pointer to a helper if it is set, then calls a library routine on the structure.
- `release_two_global_handles`: Calls one library routine on the handle at offset eight, then on the handle at offset four, then on the first handle again.
- `open_input_device_request`: Creates a message port and an I/O request block, then opens input.device with that request.
- `send_control_request_with_buffers`: Runs a library call with a size of 256 for each of two handles, fills a request block on the stack with a command code and a pointer, and sends it.
- `apply_mode_table_entry`: Applies the 16-byte table entry selected by the argument, clamping any index above one to zero.
- `store_bit_mask_result`: Computes a single-bit mask from a byte held in a structure reached through a global, and passes that mask to a routine.
- `run_winter_intro_animation`: Loads the winter picture files and sets up colour and timing tables, then draws sprite rows in staged loops.
- `draw_winter_frame_sprites`: Draws a set of sprites at positions derived from two timing counters, using one of two layouts depending on a flag.
- `repeat_routine_by_count`: Calls one routine repeatedly, the number of times given by the argument, and returns.
- `fill_global_table_entries`: Calls a routine twenty times and stores each result into successive entries of a global table.
- `build_sprite_entry_table`: Fills a table of 34-byte entries, one per item in four groups whose sizes come from a global array, linking each entry to its position and a pointer.
- `retry_library_until_nonzero`: Calls a library routine repeatedly with a global argument until it returns a nonzero result.
- `mark_list_entry_character`: Follows a linked list from a head pointer for a given number of steps, then writes either a space or an asterisk into that node's text buffer, depending on the flag argument.
- `copy_bytes_forward`: Copies the given number of bytes one at a time from a source buffer to a destination buffer.
- `store_long_in_linked_field`: Writes a 32-bit value into a field of the structure that a pointer at offset twenty-six of the argument refers to.
- `forward_library_call_30`: Calls the library routine 30 bytes below the base stored in a global, then returns.
- `forward_library_call_54`: Calls the library routine 54 bytes below the base stored in a global, then returns.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `forward_library_call_66`: Calls the library routine 66 bytes below the base stored in a global, then returns.
- `forward_library_call_72`: Calls the library routine 72 bytes below the base stored in a global, then returns.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_library_call_228`: Calls the library routine 228 bytes below the base held in a global, then returns.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `forward_library_call_240`: Calls the library routine 240 bytes below the base held in a global, then returns.
- `forward_library_call_246`: Calls the library routine 246 bytes below the base held in a global, then returns.
- `forward_library_call_252`: Calls the library routine 252 bytes below the base held in a global, then returns.
- `call_library_vector_258`: Library stub: loads a library base from a global variable and calls the library function at offset minus 258 of that base, returning its result to the caller.
- `forward_library_call_264`: Calls the library routine 264 bytes below the base held in a global, then returns.
- `forward_library_call_270`: Calls the library routine 270 bytes below the base held in a global, then returns.
- `forward_library_call_282`: Calls the library routine 282 bytes below the base held in a global, then returns.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `forward_library_call_354`: Calls the library routine 354 bytes below the base held in a global, then returns.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `allocate_five_game_buffers`: Allocates five buffers of 2560, 260, 36, 160 and 1024 bytes with memory-type flags, reporting an error code if any allocation fails.
- `call_routine_with_fallback`: Calls a routine with the fixed argument four and two stored values, then subtracts one from its result.
- `request_numbered_resource`: Makes a library call with two stored pointers, the identifier 1830 and a flag of one, and passes any nonzero result to a second routine.
- `call_with_fallback_five`: Calls a routine with the fixed argument five and two stored values, then subtracts one from its result.
- `run_menu_selection_loop`: Prepares two stack buffers from its arguments, then reads input codes in a loop until one of three exit codes arrives.
- `run_slot_selection_loop`: Runs the same row-selection loop as its sibling, using stack-held buffers instead of arguments.
- `refill_selection_list_rows`: Fills six row entries with pointers into the item table, then redraws the list.
- `run_selection_loop_body`: Runs one pass of the selection loop: refreshes the list rows, handles the movement codes, and issues redraw calls.
- `draw_frame_around_structure`: Computes a rectangle from fields of a stored structure, offset by fixed amounts, and passes its four coordinates to a drawing routine.
- `draw_rectangle_border`: When a stored item is present, fills a rectangle inside the given corners and draws its outline with eight line-drawing calls.
- `redraw_stored_item_region`: Runs three setup calls, then calls routines on the stored item's field and on a second global offset to redraw its region.
- `bracket_routine_with_handle_calls`: Brackets a call to another routine between two calls on a stored handle, one passing zero beforehand and one passing minus one afterwards.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `forward_library_call_174`: Passes two global pointers and a global address to a library routine 174 bytes below the library base, then returns.
- `scan_directory_entries_into_table`: Skips leading spaces in a path, locks it, and reads its directory entries into a table of up to seventy-eight names.
- `read_directory_listing_checked`: Locks a path and reports an error code, returning one, if the lock fails.
- `collect_directory_entries`: Reads directory entries in a loop, storing each name and its flag into a table.
- `store_directory_entry_name`: Stores each directory entry name in a table, clamping a length to twenty-two characters when it is too large.
- `read_next_directory_entries`: Repeatedly fetches the next directory entry, storing its name and length into a table of 32-byte rows.
- `read_directory_in_batches`: Repeats a batch of directory-entry reads, resetting a counter at each pass.
- `read_listing_then_unlock`: Repeats a batch read of directory entries while a lock is held, and releases the lock when the listing ends.
- `unlock_listing_and_return`: Unlocks the lock held in the frame and returns three to its caller.
- `restore_saved_task_field`: Writes a saved pointer back into a field of the current task, undoing the earlier replacement made by the buffer allocation routine.
- `build_game_file_path`: Builds a full path from a directory and a file name, adds a separator when needed, then locks the result.
- `read_or_write_data_file`: Opens a file through a library call chosen by the mode argument, then reads or writes a block of data with it.
- `request_resource_in_current_dir`: Locks the current directory, makes the numbered resource request with the given pointer, then unlocks the directory.
- `check_table_sum_after_request`: Sums a global array before and after a numbered resource request, and calls a routine when the two totals differ.
- `check_table_sum_against_stored`: Makes the numbered resource request, then sums a global array and compares the total with a stored value.
- `compare_stored_totals`: Compares two stored totals and calls a routine when they differ, then calls a second routine.
- `apply_stored_option_settings`: Reads several stored option flags and selects matching setup calls for each one, then loops over four slots.
- `invoke_record_for_large_key`: Searches a table of three-field records for a matching key value.
- `lookup_record_and_invoke`: Searches the same record table for a matching key and passes the matching record's stored pointer to a routine.
- `invoke_record_when_within_count`: Passes a field of a stored record to a routine when one counter value is at most another.
- `find_name_in_entry_table`: Searches a table of 32-byte entries for a name, comparing characters without regard to case, and returns one when a match is found.
- `match_entry_name_characters`: Walks table entries from a start index, comparing each entry's characters with a target name without regard to case.
- `return_match_flag`: Returns the match flag stored in the caller's frame, which the name search sets.
- `convert_char_to_uppercase`: Returns the byte unchanged unless it is a lowercase letter, in which case it returns the matching uppercase letter.
- `accumulate_file_info_value`: Locks a named path and reads its information block into a 36-byte buffer.
- `forward_library_call_42`: Calls the library routine 42 bytes below the base stored in a global, then returns.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `forward_library_call_150`: Calls the library routine 150 bytes below the base stored in a global, then returns.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_info`: Library stub: loads its arguments and the dos library base, calls Info and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `open_audio_device_channels`: Allocates a 1496-byte block and a message port, reporting an error code of 400 if either fails.
- `release_stored_pointers`: checks a global flag and, if not set, calls another routine with two specific global values and then iterates through a table of ten pointers to call another routine on each non-null entry.
- `release_audio_handles_and_list`: Runs a teardown call, releases the two stored handles, then releases each of ten stored pointers that is non-null.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `start_sound_voices`: Clamps a level argument to three and picks five or nine voices from it.
- `start_sound_from_table`: Clamps a level argument to eight, releases the channel bits held in the mask, then allocates free channel bits for each entry of the table selected by that level.
- `wait_until_sound_channels_free`: Sets fixed values and flag bits in an audio request, then loops while channel bits remain set.
- `silence_remaining_sound_channels`: Repeatedly issues a command for the channel mask and clears the bits of finished requests until no channel bits remain set.
- `return_without_effect`: Contains no code and returns immediately, so it has no effect on memory or hardware.
- `create_message_port_block`: Allocates a signal bit and a 34-byte port block, returning zero if either fails.
- `delete_message_port_block`: Removes the port from its list when it is linked, marks its status fields as deleted, frees its signal bit and releases the block.
- `dispatch_input_messages`: Waits for an input message in a loop and replies to messages of one particular type code.
- `advance_sequence_on_match`: Checks an incoming message's state against the expected entry in a table and advances the sequence index on a match.
- `update_sequence_state`: Compares a value from a structure against a table to increment a global index or reset it based on specific bit flags.
- `update_sequence_index`: Compares a value from a structure against a table entry to increment a global index or reset it based on a flag.
- `scan_input_table_entries`: Resets the sequence index when a flag is clear, then waits for input messages in a loop.
- `scan_game_file_entries`: Walks a list of file entries, comparing each name with a directory listing built from the examine and parent-directory calls.
- `call_routine_and_return`: Calls one routine with no arguments and returns.
- `append_string_at_end`: Finds the current length of a buffer by calling a helper with the first argument, then copies a null-terminated string from the second argument to the end of the buffer.
- `copy_string_with_limit`: Copies a string into a buffer at the position given by a length helper, stopping at the null terminator or after a given count of bytes.
- `expand_format_into_pointer`: Walks a printf-style format string using a pointer held in the argument, handling doubled percent signs and conversions through a helper.
- `initialise_list_header`: Points a list header at itself: its head is set to the field just after the header, its tail word is cleared, and its tail pointer refers back to the header.
- `count_string_length`: Counts the bytes from a given pointer offset up to the terminating zero byte, and returns that count.
- `transfer_buffer_for_stream`: Checks the state flags of a stream structure, then calls a transfer routine with its buffer and count.
- `transfer_stream_block`: Checks a stream's flags and starts a buffer transfer, returning minus one when the stream is unusable.
- `transfer_stream_with_count`: Checks the stream flags, then passes the buffer, a count and the stream's values to a transfer routine.
- `mark_stream_busy_and_transfer`: Marks a stream as busy, transfers a block through a helper, and sets error and end flags from the result.
- `transfer_stream_without_checks`: Marks a stream as busy and transfers a block through a helper, without first checking the stream's state flags.
- `read_next_stream_byte`: Takes the next byte from a stream buffer and decrements its remaining count.
- `forward_stream_to_helper`: Passes the stream pointer from the frame to a helper routine and returns.
- `write_byte_to_stream`: Writes one byte into a stream buffer and decrements its room count.
- `allocate_stream_buffer`: Returns minus one when the stream is already flagged.
- `parse_format_specifier`: Skips an optional minus sign, digits, a decimal point and an 'l' modifier.
- `convert_field_with_width_cap`: Picks a field width from a state value, defaulting to six and capping it at nineteen.
- `format_number_into_field`: Sets the field width to six when unset and caps it at nineteen.
- `format_number_with_width`: Calls a conversion routine with the field width and buffer chosen by the caller, then advances the output pointer past the result.
- `write_fill_characters`: Writes the fill character repeatedly for a counted number of positions, then stores the output pointer and returns the count plus one.
- `write_padded_field_text`: Writes fill characters for a counted number of positions, then copies a sequence of characters from a source buffer and stores the output pointer.
- `copy_field_characters`: Copies a counted number of characters from a source pointer into the output buffer, then stores the output pointer and returns the count plus one.
- `store_pointer_and_return_count`: Stores a saved pointer into the location referenced by the argument and returns a stored counter plus one.
- `format_number_default_width`: Sets the field width to six when unset and caps it at nineteen.
- `format_signed_number_field`: Formats a number into the field, adjusting its sign and digit count before a helper writes the digits.
- `compute_number_field_length`: Works out how many characters a number needs by counting its sign and digits.
- `pad_number_field_to_width`: Applies the same length adjustments as its sibling, then writes fill characters to pad the field to the requested width before the digits are written.
- `pad_and_write_number_digits`: Adjusts the length of a number for values above ninety-nine and above nine hundred and ninety-nine.
- `write_decimal_fraction_digits`: Writes a minus sign when the sign flag is set, then one integer digit or a zero, a decimal point, and the remaining fraction digits.
- `write_integer_and_fraction_digits`: Writes an integer digit or a zero for each remaining position, then writes fraction digits and pads the rest with zeros.
- `write_zero_padding_digits`: Writes a zero, then repeatedly writes zeros or leftover digits until the requested count is used up.
- `write_decimal_point_and_digits`: Writes integer digits while a counter remains, adds a decimal point when a fraction length is set, then writes the fraction digits.
- `write_zero_then_fraction`: Writes a zero, then the decimal point and fraction digits, stopping when the counters run out.
- `write_fraction_part`: Writes a decimal point when a fraction length is set, then writes fraction digits and pads with zeros or fill characters when the field has room.
- `write_zeros_with_field_limit`: Writes zeros while the counters allow, then writes leftover digits and adds fill characters up to the field width.
- `write_digit_then_decimal_point`: Writes one integer digit or a zero, then a decimal point, then the fraction digits that remain.
- `write_leading_zero_and_point`: Writes a leading zero and a decimal point, then the fraction digits from the value, stopping when the counter is used up.
- `write_scientific_exponent`: Writes a zero, then an exponent marker, and the exponent digits with a minus sign where the exponent is negative.
- `write_exponent_marker_and_digits`: Writes an exponent marker and a plus or minus sign, then converts the exponent into digits held in an eleven-character scratch buffer.
- `write_positive_exponent_digits`: Writes a plus sign, then converts the exponent value into decimal digits in a scratch buffer and copies them to the output.
- `copy_padding_characters`: Copies a counted run of padding characters to the output, then stores the output pointer and returns the count plus one.
- `copy_value_and_increment_counter`: The routine copies a value from a location on the stack to a field in a structure pointed to by the frame pointer and returns the value of another field from that structure plus one.
- `create_io_request_block`: Allocates a memory block for an I/O request when the first argument is nonzero, setting its type code, size field and device pointer.
- `delete_io_request_block`: If the pointer is non-null, marks the request's type and two status fields as cleared, then frees the block using its stored size.
- `create_standard_io_request`: Creates an I/O request of the fixed size forty-eight for the given pointer and returns the result.
- `delete_standard_io_request`: Passes its pointer argument to a deletion routine and returns with no other work.
- `combine_pair_into_value`: Calls a routine that fills two values, then returns the first multiplied by two hundred and fifty-six plus the second shifted right by twelve bits.
- `draw_menu_list_items`: Selects a 30-byte list record and stores a value in its text field.
- `draw_cursor_at_index`: Computes a cursor position from the current index and a stored offset, then passes that position to two drawing routines.
- `draw_two_panel_frames`: Draws a pair of framed panels, each built from two coordinate pairs taken from the arguments and fixed sizes of twenty and twenty-three.
- `draw_four_corner_markers`: Sets a mode value of fifteen, then draws four markers at the corners of a rectangle offset by nineteen and twenty-two units from the given position.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `clear_entry_flag_bit`: Looks up an entry by index, then clears bit sixteen in its flag word.
- `clear_entry_flags_masked`: Looks up an entry by index, then clears the bits given by a mask from its flag word.
- `find_list_node_at_index`: Follows the next pointers of a linked list from the head pointer stored at an offset, for the given number of steps.
- `return_immediately_2`: This routine is empty and performs no operations before returning.
- `store_four_word_fields`: Writes four 16-bit values into consecutive fields of the record at the pointer argument.
- `return_immediately_3`: This routine performs no operations and returns to the caller immediately.
- `next_pseudo_random_word`: Shifts a 32-bit state left by one bit and XORs in a feedback constant when the top bit was set, then returns the low sixteen bits.
- `count_list_entries`: Counts the nodes of the linked list whose head pointer is stored at a fixed offset in the structure argument.
- `draw_entry_when_visible`: Draws the entry at its stored coordinates when its high flag bit is clear.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to the caller.
- `draw_two_screen_bands`: Draws two horizontal bands across a screen width of 320 units at fixed positions, each followed by a second drawing call.
- `set_entry_highlight_state`: Sets or clears the high flag bit of an entry, calling library routines when the state changes.
- `draw_three_screen_bands`: Draws three horizontal bands across a screen width of 320 units at fixed offsets, each followed by a second drawing call.
- `draw_text_string`: Sets drawing mode values, measures the length of a null-terminated string, then draws the string with a library call.
- `draw_text_with_colour`: Like its sibling, measures a null-terminated string and draws it, but takes the drawing mode from its second argument.
- `return_immediately_7`: This routine performs no operations and returns immediately to its caller.
- `forward_two_word_arguments`: Passes its two 16-bit arguments to a routine and returns.
- `copy_eight_bytes_to_local`: Copies eight bytes from the source pointer into a local buffer, then returns without using the copy.
- `measure_cell_boundary_distances`: Measures distances from a position to the neighbouring cell boundaries in a 16-bit table, clamping negative distances to zero.
- `compute_cell_overlap_extents`: Computes the clamped distances from a position to the edges of neighbouring cells in a 16-bit table, then returns the extents derived from them.
- `check_cell_clearance`: Computes two clamped distances from a cell to its neighbours in a 16-bit table and returns a value based on them.
- `test_point_inside_cell`: Measures a clamped distance to a neighbouring cell.
- `test_rectangle_overlap_clearance`: Tests whether a candidate point or rectangle lies inside a stored cell rectangle, returning one on a match.
- `test_clearance_for_sprite_edges`: Tests whether a sprite of the given width fits between the cell's edges, returning one when the clearance is sufficient.
- `test_box_within_cell_bounds`: Returns one when a box with the given offsets lies within the cell's bounds, otherwise zero.
- `test_top_edge_fit`: Returns one when a position lies within the cell's horizontal range and the box fits below its top edge, otherwise zero.
- `return_zero_result`: Returns zero with no other work.
- `lookup_grid_neighbour_value`: Computes a neighbouring cell index on a fifteen-column grid from a base index and offsets.
- `return_immediately_4`: This routine performs no operations and returns immediately to the caller.
- `mark_entry_and_refresh_rectangle`: Sets the high flag bit of a sprite entry, computes a rectangle enlarged by a few units around its position and size, and passes it to a drawing routine after a library call.
- `forward_record_position`: Passes the first two 16-bit fields of a record, together with a further argument, to a routine, then returns.
- `forward_zero_argument`: Calls one routine with the argument zero and returns.
- `advance_record_position`: Increments two fields of a record by one, then passes the record and a value to a routine.
- `draw_bordered_frame_pattern`: Draws a patterned border around a rectangle described by a record, repeating the edge segments once per step of a count derived from a global value.
- `draw_record_rectangle`: Copies the four 16-bit fields of a record and passes them, with two of them as coordinates, to a drawing routine.
- `draw_layered_record_rectangle`: Draws a rectangle described by a record in several passes, each with a different mode value, using the record's position and size.
- `clip_and_draw_rectangle`: Clips a rectangle's width and height to the bounds held in a structure, then passes the clipped values to a drawing routine.
- `store_global_mode_value`: Stores its argument in a global field that later drawing routines read.
- `do_nothing`: This routine is empty and performs no operations.
- `save_four_blocks_to_file`: Opens a file through a library call and, when the open succeeds, writes four blocks of 8000 bytes from a table followed by a 32-byte block.
- `write_blocks_to_open_file`: Writes four blocks of 8000 bytes taken from a table, then a 32-byte block, to an already open file, and closes it.
- `write_header_and_close`: Writes a 32-byte block from a global to an open file and then closes it.
- `draw_fixed_marker_set`: Sets a draw mode of four, then draws a fixed set of eight markers at stored positions offset by fixed amounts.
- `set_global_flag`: compares a passed pointer to a global variable and sets a byte in a global variable to one if they match.
- `divide_signed_values`: Divides two signed 32-bit values by restoring division over thirty-two steps.
- `exchange_value_through_helper`: Reads a value into a local through a helper in mode zero, runs two further calls, then passes the local to the helper in mode one and returns it.
- `store_stack_value_via_helper`: Passes a stack slot to a helper in mode one and returns.
- `transfer_saved_value`: When the flag argument is zero, copies a global value into the target location.
- `allocate_free_table_slot`: Scans a table of 24-byte entries for one whose flag byte is clear, and passes the first free entry with its arguments to a routine.
- `call_routine_for_table_slot`: Returns zero when the slot pointer is at the end of the table.
- `forward_slot_arguments`: Passes two arguments and the slot pointer from the frame to a routine, then returns.
- `open_file_by_mode_string`: Parses a file mode string, choosing binary or append flags, then opens the file with a library call using a fixed permission value of 420.
- `open_file_for_mode_pointer`: Parses a mode character and a following plus sign, then opens the file with the same flags and fixed permission value of 420.
- `open_new_file_into_record`: Calls a routine with create-and-write style flags and a fixed permission value, returning null when it yields minus one.
- `seek_end_and_init_record`: Moves the handle to the end, sets a mode byte from a flag, and fills the caller's record with the handle, the mode and cleared pointer fields.
- `fill_record_with_mode_two`: Sets the mode to two, then fills the caller's record with the handle, the mode and cleared pointer fields.
- `open_file_with_flag_bit`: Calls a routine with a flag-selected bit added to a stored value, returning null when it yields minus one.
- `obtain_handle_and_fill_record`: Calls a routine with the record's argument and a stored value, returning null when it yields minus one.
- `fill_record_with_mode_flag`: Sets the mode to one or 128 according to a flag, then fills the caller's record with the handle, that mode and cleared pointer fields.
- `fill_record_with_mode_one`: Sets the mode to one, then fills the caller's record with the handle, the mode and cleared pointer fields.
- `open_flagged_file_into_record`: Calls a routine with a flag-selected bit, a stored value and a permission value, returning null when it yields minus one.
- `populate_record_with_file_data`: calls another routine to allocate a slot, determines a mode byte based on a flag, and populates a record structure with a handle, mode, and cleared pointer fields.
- `fill_record_with_mode_byte`: Sets the mode to two or 128 according to a flag, then fills the caller's record with the handle, that mode and cleared pointer fields.
- `store_mode_record`: calls another routine and then populates a record structure with a specific mode value and other data from local variables.
- `stub_return_zero`: Returns zero without doing any other work, so the routine is a minimal stub.
- `store_handle_mode_record`: Calls a routine with a flag-selected bit, then stores the handle and mode from the locals into the caller's record.
- `reset_record_with_handle_mode`: Clears a pointer field in the caller's record, then stores the handle and mode from the locals into the record and clears its remaining fields.
- `release_structure_links`: Calls a routine with the structure when a flag bit is set, and when the flag's other bits are clear and a stored pointer is non-null it calls a routine with the two stored fields.
- `check_flag_and_return_pointer`: Calls a routine first when a global flag is non-zero.
- `allocate_table_slot_with_flags`: Scans a global table for a free slot and records a limit error with minus one when none is free.
- `init_table_slot_from_record`: Takes the slot index from a record and returns minus one with a limit error if it is at the table limit.
- `set_slot_status_from_flags`: Derives a slot's status byte from a record's flags and access mode, rejects an invalid mode with an error and minus one, then calls a routine whose value is stored in the slot.
- `set_slot_status_with_mode`: Writes the active status bit into a slot's status byte, adds the mode bits from the record's access flags, and rejects an invalid mode with an error.
- `set_slot_mode_one_flagged`: Adds a mode-one bit plus an optional bit chosen by a flag to the slot's status byte.
- `set_slot_mode_one_value`: Adds the mode-one bit to the slot's status byte, then calls a routine with the first argument and one of two mode constants and stores the result in the slot.
- `set_slot_mode_two_flagged`: Adds a mode-two bit plus an optional bit chosen by a flag to the slot's status byte.
- `set_slot_mode_two_value`: Adds the mode-two bits to the slot's status byte, then calls a routine with the first argument and one of two mode constants and stores the result in the slot.
- `fail_slot_with_invalid_mode`: Clears the slot's status byte, records an invalid-argument error code and returns minus one.
- `store_slot_file_by_flags`: Chooses one of two mode constants from a flag group, then calls a routine with the first argument and that constant and stores the result in the slot.
- `open_slot_file_default_mode`: Uses a fixed mode constant to call a file-open routine with the first argument and stores the handle in the slot.
- `return_stored_value`: Returns the value held in a local without doing any other work.
- `clear_status_flag_and_forward`: Forwards its first argument to another routine with a fixed flag set plus the top bit of the second argument, and passes the second argument with that top bit cleared.
- `forward_transfer_to_handle`: Rejects a negative length with an error code and minus one.
- `read_into_buffer_checked`: Calls a read library routine on the stored handle with the caller's buffer and length, and keeps the byte count.
- `write_handle_with_error_check`: Validates a non-negative length and looks up a handle record.
- `seek_file_with_offset_table`: Looks up a handle record and, for an index below three, seeks the file to an offset taken from a global table, recording an error code on failure.
- `close_handle_if_flag_clear`: Looks up a handle record from the argument, returning minus one when it is missing.
- `delete_file_with_error_check`: Calls a delete-file library routine with the argument.
- `forward_single_argument`: Forwards its single argument to another routine and returns nothing.
- `rename_file_with_error_check`: Calls a rename library routine with two arguments.
- `return_zero`: The routine is empty and simply returns a value of zero to the caller.
- `open_global_file_handle`: Opens the global file record with a fixed mode constant and stores the returned handle in a global variable, then returns.
- `read_next_global_byte`: Calls routines when a global flag is set and when the stored handle is zero.
- `read_one_byte_from_handle`: Reads a single byte from the stored handle through a read library routine into a local, and returns that byte.
- `write_single_byte_to_handle`: Checks two global values and calls a routine when a condition fails, then writes the byte argument to the stored handle with a write library routine.
- `set_global_byte_if_clear`: Returns minus one without changing anything when a global byte is already non-zero.
- `store_low_byte_to_global`: Copies the low byte of a value taken from the caller's frame into a global byte variable and returns.
- `close_active_handles`: Walks a table of entries whose count is held in a global.
- `copy_state_and_program_chips`: Copies a 20-byte global block into the first argument, updates a global byte, and stores one bit chosen by the second argument with the third argument in the copied block.
- `return_fixed_value`: Returns a fixed constant without doing any other work.
- `copy_string_to_destination`: Copies a null-terminated string from the second argument into the first, byte by byte including the terminator, and returns the destination.
- `copy_bounded_string`: Copies up to a counted number of bytes from the source to the destination, stopping after a NUL byte.
- `reset_global_state_fields`: Clears a set of global state variables to zero and fills two global buffers through library calls.
- `toggle_around_paired_calls`: Calls a routine with 31, flips a global toggle between 0 and 1, calls the same routine with 15, and flips the toggle back.
- `update_timed_counter_state`: Compares a time reading against a stored base, and when the difference exceeds a threshold or a flag is set it decrements a global counter and calls a routine with the active table entry.
- `format_time_and_output_text`: Formats a global count as minutes and seconds, or as seconds alone when it is under 60, into a local string using a printf-style routine.
- `call_offset_sequence_at_position`: Issues a fixed sequence of seven routine calls with the two arguments offset by small constants, one of which passes a block of eleven values.
- `store_slot_and_reset_state`: Calls routines with two global references, stores the argument into a global slot after reading its previous value, then calls routines with fixed values and a global reference.
- `run_selection_until_confirmed`: Repeats a selection routine until its result is 1 or 2.
- `wait_for_selection_and_apply`: Repeats a selection routine until its result is 1 or 2, storing results from 3 to 6 and calling routines on each table entry.
- `update_flag_and_call_pair`: Sets a global flag from a bit in a structure, then calls routines with a global reference and a stored value.
- `clear_flag_and_call_pair`: Clears a global flag, then calls a routine with a global reference and a stored value.
- `assign_offsets_to_linked_records`: Walks a linked list of records starting at the first argument.
- `call_library_routine_once`: Calls a library routine through the global library base with no arguments and returns nothing.
- `show_end_of_game_result`: Outputs one of several result messages built from score globals, such as a winner, a tie or a no-more-moves ending.
- `center_entries_in_width`: Walks four records from a linked list in a global structure.
- `draw_indexed_entry_fields`: Uses the record selected by a global index to output a formatted two-digit value and several fixed fields, at positions taken from that record and the argument.
- `find_fitting_text_length`: Measures a null-terminated string and shortens its length one character at a time until a measuring routine reports a width at or below the limit.
- `return_from_subroutine`: performs a return from a subroutine.
- `update_small_index_pair`: For each of two arguments below five, calls a routine twice on the global table with the argument and a flag value, using zero for the first argument and one for the second.
- `initialize_setup_state`: Clears several global state fields and sets others to fixed values, then calls routines to set up three global entries.
- `forward_argument_with_globals`: Forwards its argument with three global buffer references to a routine, then calls another routine with a fixed value and zero.
- `repeat_call_fifteen_times`: Calls a routine fifteen times in a loop and returns.
- `return_nothing_stub`: Returns immediately without doing any work, so the routine is a stub.
- `draw_indexed_element_group`: Returns the computed counter value when a global flag is set.
- `clear_saved_selection_state`: When a mode global is set, it calls routines with the saved entries referenced by two globals and with a fixed value.
- `draw_indexed_panel_item`: Positions a numbered item from a table of coordinate values, using a second layout when a global flag is set and the index is from 84 to 87.
- `enable_panel_pair`: Calls a routine for two panel indices with fixed arguments, sets a global flag to one, then calls a routine with a global value.
- `disable_panel_pair`: Calls a routine for two panel indices with fixed arguments, clears a global flag, then calls a routine with a global value.
- `enable_alternate_panel_pair`: Calls a routine for two other panel indices with fixed arguments, and sets a global flag to one.
- `disable_alternate_panel_pair`: Calls a routine for two other panel indices with fixed arguments, and clears a global flag to zero.
- `call_fixed_argument_pair`: Calls a routine with a fixed value and zero.
- `compare_two_values`: Calls routines to read two values and returns one when they are equal, after calling a routine with a global and zero.
- `call_with_four_return_zero`: Calls a routine with the value four and returns zero.
- `set_flag_and_run_steps`: Sets a global flag to one, then calls three routines in sequence, two of them with global buffer references.
- `read_input_and_clear_flag`: Reads into a 256-byte local buffer through a routine.
- `advance_state_and_store_pair`: When a global mode equals four it increments a per-slot counter and sets two flags.
- `clear_flag_and_run_step`: Clears a global flag to zero, then calls a routine.
- `draw_bounds_for_index_range`: Calls a routine with four computed bounds, then for indices from 84 to 87 it calls routines with values from a table and computed rectangle positions.
- `show_status_and_wait`: Outputs a status message chosen by a state global and a flag, and sets the state to 10, 6 or 5 according to its value.
- `output_pair_values_and_advance`: Looks up the entry at an index and outputs one of its two stored values, chosen by comparing against a key global, then outputs the matching values of the following two entries.
- `output_table_values_and_advance`: Outputs values from table entries at the index, choosing between two stored values by a key comparison, then adds two to a counter and calls a routine with fifteen when the counter reaches 144.
- `output_alternating_table_values`: Outputs values from table entries at the index in alternating order, then adds two to a counter and calls a routine with fifteen when the counter reaches 144.
- `activate_flag_and_set_limit`: Sets a global flag to one and a global value to 88, then calls a routine.
- `advance_index_chain_entry`: Updates a table entry for the given index by following the value stored there, marks the entry it reaches as one, increments the index's counter, and calls a routine with the index.
- `advance_index_chain_from_frame`: Sets a table entry from the value it points to, marks the entry it reaches as one, increments the counter for the index taken from the caller's frame, and calls a routine with that index.
- `check_index_available`: Returns zero when the table entry for the index is empty, or when the index is in the range 84 to 87 while a flag is set.
- `check_index_available_frame`: Returns zero for a chained table entry that is set or for an index in the range 84 to 87 while a flag is set, using an index taken from the caller's frame.
- `check_index_chain_available`: Returns one or zero from the availability conditions without the empty-entry or range checks.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a6, then returns.
- `check_chained_entry_free`: Returns zero when a chained table entry is set for the index, and returns one for indices below 89 otherwise.
- `check_chain_for_high_index`: Returns one for indices below 89.
- `return_one_stub`: Returns one without doing any other work, so the routine is a stub.
- `return_constant_one_2`: The routine returns the integer value one to the caller.
- `draw_remaining_tiles_panel`: Draws a panel from fixed rectangle values, then outputs a text line containing a remaining-count value formatted as three digits, taken from a global.
- `dispatch_layout_by_index`: Returns minus one for a non-positive argument.
- `emit_flagged_argument_list`: Optionally calls a routine with an identifier computed from the second argument.
- `test_state_one_to_four`: Returns one when the state global is between one and four inclusive, and zero otherwise.
- `switch_state_between_pairs`: Changes the state global from six to two or from five to one, and calls routines for each change.
- `call_fixed_default_layout`: Calls a routine with a fixed list of eleven values and returns.
- `set_state_two_and_layout`: Sets the state global to two, calls a routine with two fixed values, then calls a routine with one of two fixed value lists chosen by a flag.
- `call_argument_list`: Calls another routine passing a specific sequence of eleven integer arguments.
- `pop_list_head_and_call`: Calls a routine with the argument, then clears a link entry, moves the head to the next linked value, decrements the head's counter, and calls a routine with the argument and the new value.
- `draw_grid_cell_and_neighbours`: Computes the screen position of a cell from its index in a 15-column grid, choosing offsets for edge and centre cells.
- `run_input_until_result`: Stores an argument and a routine's value in a record, then repeatedly calls a routine until it returns one or two.
- `collect_text_entries_until_result`: Reads text into a 256-byte buffer, splits it into up to four entries separated by NUL bytes, and stores each entry's pointer and measured width in records.
- `wait_for_choice_result`: Calls a routine, then keeps calling a routine until its result is one or two, and returns that result.
- `wait_until_result_is_one`: Initializes a record with an argument and a fixed byte value, calls a routine for a value, then calls a routine repeatedly until it returns one and returns that value.
- `drain_entry_list_to_count`: Repeatedly takes an entry from a list.
- `forward_with_global_table_pair`: Forwards its argument to a routine together with two global table references and a global reference.
- `build_display_item_records`: Walks a linked list of entries for one slot and fills consecutive 44-byte display records from them.
- `fill_display_record_from_item`: Writes a 23-byte record from an item's position and size fields, sets its type byte to 23, and clears its other fields.
- `search_slot_table_for_entry`: Searches up to five global table entries for one whose key matches the record's entry, and calls a routine with the matched value.
- `set_layout_limit_by_state`: Sets two global limit values to 63 when the state global is three, and to 47 otherwise.
- `compare_strings_counted`: Compares up to a counted number of bytes of two strings.
- `scan_unquoted_characters`: Scans a string, skipping quoted sections and backslash escapes, and passes each unquoted character to a routine until that routine returns non-zero.
- `handle_game_event`: The routine is 21 instructions long but ends with an immediate return, performing no observable work on data or registers.
- `compare_strings_full`: Compares two null-terminated strings and returns the byte difference at the first mismatch, or zero when they are equal.
- `call_library_offset_thirty`: Calls a library function through the library base held in the argument, at a fixed negative offset of thirty, and returns nothing.
- `initialize_slot_tables`: Clears seven state globals and sets a count global to 144.
- `return_absolute_value`: Returns the absolute value of a 16-bit argument.
- `return_sum_of_two_calls`: Calls two routines that each return a value and returns their sum.
- `shuffle_index_table`: Fills a 144-entry array with the indices one to 144, then shuffles it by swapping each entry with an earlier entry chosen by a routine.
- `reset_count_and_call_pair`: Sets a count global to 144, calls a routine, then calls a routine with two global references.
- `output_table_value_and_char`: Calls a routine twice with the table value at the first index, then calls a routine with the byte at the second index as a signed value.
- `call_fixed_sequence_for_pair`: Issues a fixed sequence of five routine calls, using fixed indices and the global pair positions offset by fixed amounts.
- `draw_grid_and_report_state`: Clears a global counter and calls a routine.
- `draw_positive_entry_frame`: Calls a routine with the entry value at the index.
- `find_cell_under_position`: Converts a position to a grid column clamped to 0 to 12 and a row clamped to 0 to 6.
- `update_selection_from_table`: When a flag is set it draws a fixed item and checks it with a routine, recording the selection if the check succeeds.
- `copy_selected_entry_value`: Copies a value from a global table at the current selection index into a second global.
- `handle_two_step_selection`: Returns when the selected cell is empty or does not fit the pending picks.
- `advance_two_pick_selection`: Advances a two-pick selection for the selected cell.
- `select_cell_if_available`: Returns when a routine reports the selected cell is unavailable.
- `run_two_pick_state_machine`: Runs the two-pick state machine on the selected cell: state zero starts a pick, state one either finishes a matching pair or records a second cell, and state two resolves the pending pair.
- `update_two_pick_state`: Resolves a pending pair in the two-pick state: a match moves the first pick forward, a repeat of the second cell clears it, and the state and output routines are then called.
- `record_second_pick_candidate`: Increments the pick state, stores the selected cell as the second pick, and calls a routine.
- `flag_invalid_selection_pick`: Calls a routine with the pick state and a fixed value, then sets a global flag to one.
- `clear_second_pick_state`: When the pick state is two, it resets the pending pick, or clears the second pick when the selected cell matches the first pick, then calls output routines.
- `revert_unmatched_second_pick`: When the selected cell equals the second pick, it decrements the pick state and clears the second pick, then calls output routines.
- `toggle_pick_at_cell`: Checks the selected cell's table value and returns when it is in the excluded range.
- `toggle_pick_at_current_cell`: Toggles the pick state of the currently selected cell between zero and one, calling output routines for each change.
- `start_or_end_pick`: Starts a pick when the state is zero, or ends it when the state is one and the selected cell matches the stored pick.
- `toggle_pick_state_plain`: Starts a pick when the state is zero, or ends it when the state is one, calling output routines for each change.
- `cancel_pending_pick`: When the pick state is one, it resets the pick state and two pick globals to zero, calls output routines, and calls a routine with a global reference and a fixed value.
- `finish_selection_and_reset`: Finishes a selection: for a matching stored value it calls one routine, otherwise it outputs the selected cell and, for cells from 84 to 87 when a flag is set, calls another routine.
- `finish_pick_and_reset_state`: Decrements a counter, outputs the status for the selected cell, and resets the pick state and four pick globals.
- `reset_pick_state_and_output`: Calls two routines with fixed values, then clears the pick state and four pick globals.
- `check_cell_placement_allowed`: Returns zero after a routine reports a message code when the cell's chained entries or a range restriction forbid placement.
- `check_cell_placement_from_frame`: Returns zero after a routine reports a message code when a chained entry for the frame's index is set or the index is in the range 84 to 87 while a flag is set.
- `check_restricted_range_flag`: Returns zero after a routine reports a message code when the frame's index is in the range 84 to 87 while a flag is set.
- `return_constant_one`: Returns the integer value one.
- `run_selection_loop_until_final`: Calls a selection routine with a fixed value in a loop, calling output routines for each state until the result is final.
- `run_selection_loop_from_frame`: Runs a selection loop like its neighbours, taking its state from the caller's frame.
- `run_selection_loop_retry_state`: Runs a selection loop that calls output routines for each state and repeats until the result is final.
- `run_selection_loop_resume`: Resumes a selection loop at its entry point, calling output routines for each state until the selection is final.
- `run_selection_loop_restart`: Restarts a selection loop, calling output routines for each state until the selection is final.
- `finish_selection_with_fallback`: Finishes a selection and returns one when the selection stands.
- `finish_selection_if_unchanged`: Finishes a selection when its result is unchanged, resetting the pick state and outputting status through routines.
- `restore_selection_and_refresh`: Restores the pick state from the frame, then calls output routines for the current selection.
- `load_tour_file_state`: Loads a tour file's state by calling routines with a file name and a fixed value, stores the result, and calls output routines depending on whether a result was found.
- `draw_panel_wait_for_choice`: Draws a panel from fixed entries and a record, then waits in a loop until a routine returns one or two.
- `confirm_replace_and_set_limit`: Checks whether a stored entry exists and, if so, asks a routine with a reference text whether to replace it.
- `run_selection_loop_set_limit`: Runs a selection loop that calls routines for each table entry until the result is one or two.
- `run_selection_loop_frame_limit`: Runs a selection loop that takes its state from the caller's frame and calls routines for each table entry until the result is one or two.
- `finish_selection_and_report`: Calls output routines for two values, then runs a routine and returns one when its result is non-zero, after calling output routines to finish.
- `return_zero_stub`: Returns zero without doing any other work, so the routine is a stub.
- `return_constant_one_3`: This routine returns the integer value one to the caller.
- `draw_time_left_label`: Outputs a label at a position offset by six and twelve from two global coordinates, then calls output routines at the same coordinates twice.
- `return_nothing`: An empty routine that returns without performing any action.
- `build_score_entry_records`: When a count global is non-zero, builds that many text records from table values, storing each text pointer, a position and a computed centre offset.
- `clear_score_entry_count`: Calls a routine with two global references, then clears the score entry count.
- `reset_score_display_state`: Calls routines to reset the display and output fixed values, stores the current count in a second global, sets a flag to one, and stores the result of a routine in another global.
- `tick_countdown_timer`: When the elapsed time since the last tick exceeds 256, decrements a remaining-time counter.
- `show_tournament_result_text`: Builds a results message from the placement, the tie wording and the scores, outputs it, and waits in a loop until a routine returns one.
- `insert_score_into_table`: Inserts a score into a table of up to five sorted entries and returns six when the table is full and the score is below the last entry.
- `shift_global_variable_left`: Returns a global variable multiplied by eight.
- `forward_value_twice`: Forwards its argument to a routine twice, passing it as both arguments, and returns.
- `allocate_block_from_free_list`: Returns null for a non-positive size.
- `forward_with_repeated_second`: Forwards its first argument to a routine with the second argument repeated as the last two arguments, and returns.
- `insert_or_merge_block_node`: Inserts a block of a given size at a position into a sorted list, returning minus one when the size is non-positive or the block would overlap.
- `format_scaled_decimal_digits`: Scales a 64-bit value by shifts and writes its decimal digits into a 20-byte buffer with zero padding.
- `format_scaled_digits_alt`: Writes the decimal digits of a scaled 64-bit value into a 20-byte buffer through the same shifting loop, then copies them to the destination and stores the digit count and flag.
- `format_scaled_digits_loop`: Repeatedly shifts a 64-bit value and writes zero-padded decimal digits into a buffer, then copies the digits to the destination and stores the digit count.
- `format_scaled_digits_resume`: Resumes the scaled-digit loop, shifting a 64-bit value and writing zero-padded digits before copying them to the destination and storing the digit count.
- `pad_and_copy_digit_buffer`: Writes zero digits into the buffer from a position up to 19, then copies the digits to a destination buffer, handling a leading flag.
- `write_digit_characters_and_copy`: Writes each digit as a character from a carry value, then copies the digits to the destination buffer and stores the digit count and flag.
- `decrement_loop_until_zero`: Decrements a short value in a loop until it reaches zero, then returns with no other effect.
- `scale_pair_by_ten`: Returns a 64-bit value whose low part is the second argument multiplied by ten, and whose high part is the first argument combined with shifts, carries and a multiplication by five, adjusted by sign tests.
- `shift_bits_into_decimal_digits`: Shifts a 64-bit value left one bit per iteration over 64 iterations, feeding each bit into a running digit accumulator.
- `check_signal_and_dispatch`: Calls a system routine with a fixed mask and returns zero when the masked result is clear.
- `call_fixed_setup_pair`: Calls a routine with two global references and the value twelve, then calls it with a global reference and the value 2560.
- `draw_indexed_segment_outline`: Clears a pending state when one is set, resets the fields of a record, and draws a set of outlined segments around the first two arguments.
- `clamp_coordinates_then_call`: Clamps the first and second arguments to be non-negative and within the width and height stored in a structure, then calls a routine with the clamped values.
- `invoke_library_vector_minus_258`: Clamps the two coordinate arguments so neither is negative and each stays inside limits read from a structure that the first argument points to, then forwards them to a library function at offset minus 258 from a library base.
- `format_string_with_directives`: Copies a format string into an output buffer, turning each doubled percent sign into one percent sign and expanding other percent directives through a called routine that fills a temporary buffer.
- `forward_call_and_return`: Calls one other routine and then returns.
- `dispatch_one_or_five_calls`: When both values in the first operand pair are zero, it calls a routine once with the argument three.
- `check_and_repeat_call`: Calls another routine five times in a row when a flag in one register or the masked value in another register is non-zero.
- `divide_double_words_bitwise`: Performs a 32-step restoring division on pairs of 32-bit words.
- `run_three_calls_when_nonzero`: When both operand pairs contain a non-zero value, it calls a routine twice, runs a countdown loop whose overflow branch the self-comparison test never takes, and then calls the routine a third time.
- `execute_triple_call`: Checks if two pairs of values are non-zero and then calls another routine three times.
- `run_three_calls_unconditionally`: Calls a routine twice, runs a countdown branch that the self-comparison test never enables, then calls the routine once more and returns.
- `delay_then_call_routine`: Runs a 16-bit countdown loop that starts from the low word of the third data register, then calls another routine and returns.
- `delay_if_small_then_call`: If the zero flag is clear and the low word of the third data register is below 64, it runs a countdown loop starting from that value.
- `delay_and_call`: Performs a decrementing loop and then calls another routine.
- `self_invoke_and_return`: The routine calls another routine and then returns.
- `scale_called_result_saturating`: Calls a routine for a value, then shifts that value right by an amount taken from the low word of the input.
- `scale_signed_result_by_count`: Calls a routine for a value and shifts it right by an amount derived from the second input, saturating to the largest positive value when that amount is not negative.
- `shift_and_sign_extend`: Right-shifts the first input by the negated low word of the second input, and negates the result when the second input is negative.
- `invoke_library_vector_minus_84`: Loads a library base from a global variable and calls the library routine at offset minus 84 from it, then returns.
- `find_char_in_string`: Scans a zero-terminated string for the given character and returns a pointer to its first match, or null if it is absent.
- `forward_two_arguments_unused`: Forwards its two arguments to another routine and then returns.
- `divide_unsigned_restoring`: Returns zero when either input is zero.
- `run_selection_loop_over_table`: Makes a fixed sequence of library calls and pointer-based setup, then loops reading a selection value.
- `iterate_table_entries_until_done`: Repeatedly reads a value and, when it lies between two and eight, walks seven table entries and forwards selected fields to another routine.
- `forward_three_offset_calls`: Calls another routine three times with fixed leading arguments, passing the input plus zero, four and eight as the last argument each time.
- `position_entry_and_forward_calls`: Makes one call with constant arguments, then calls another routine with position values built from the two inputs plus fixed offsets, and finishes with a call whose value is read from a table indexed by the third input.
- `update_ui_element_geometry`: Calculates and writes four coordinate values to a record using offsets from a table, then executes a series of conditional drawing or state-update sequences based on an input parameter.
- `return_from_subroutine_3`: The routine is empty and performs no operations before returning.
- `forward_two_values_to_call`: Forwards its two word-sized arguments, together with a fixed base value, to another routine and then returns.
- `run_table_driven_update_loop`: Runs a counted loop that walks a 19-entry table on each pass and forwards each entry's values to another routine.
- `resume_table_update_loop`: A stack-frame variant of the table update loop that resumes at its outer loop, walking the 19-entry table and the pair list with state kept in local frame slots.
- `continue_table_update_pass`: Continues a table update pass from the point after the first step, decrementing a counter and walking the 19-entry table on each iteration.
- `advance_table_update_step`: Advances the table update step by incrementing a frame counter, then runs the 19-entry table pass and the pair list when it is the first step.
- `finish_table_update_cleanup`: Makes two calls with a shared global argument, repeats one of them when a frame value is non-zero, then calls another routine and returns.
- `call_with_fixed_parameter_block`: Calls another routine once with a fixed list of constant numeric parameters, taking two of them from a structure pointed to by a global value.
- `poll_until_ready_then_call`: Calls one routine with a constant, then calls another with a global argument, polls a routine repeatedly until it returns one, and makes a final call with the same global argument.
- `unpack_float_mantissa_pair`: Strips the sign bit from the first input and, when the input is non-zero, shifts the second input's bits into the mantissa position ten steps long and sets the top bit to form a normalised pair.
- `shift_sign_bits_into_word`: Shifts the second input left ten times, collecting each outgoing sign bit into the first input, then sets the top bit of the result and returns it.
- `normalize_mantissa_check_range`: Normalises a 64-bit mantissa pair by shifting it until its leading bit is in place and adjusting an exponent word to match.
- `pack_normalized_double`: Normalises a 64-bit mantissa by shifting until a leading bit is set, then packs the exponent and mantissa into a double-precision pair.
- `signal_one_return_zero`: Calls a routine with the argument one and returns a zero double-precision value.
- `signal_two_return_infinity`: Calls a routine with the argument two and returns a double-precision value with the maximum exponent and no mantissa, which is an infinity.
- `store_param_to_global`: Copies its single argument into a global variable and returns.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_pointer`: When its buffer argument is non-null, copies sixteen 32-bit words from a fixed offset in the data area into that buffer.

### Fault

- `fill_handler_block_from_list`: Scans a linked list for the largest entry value, then makes calls through a function pointer and fills a new block with a counting pattern of words.
- `copy_stack_data_2`: Transfers a block of sixteen bytes from a pointer-based source to the stack before executing a subroutine.

### Install

- `initialize_task_structure_3`: Calculates a maximum value from a linked list and populates a data structure with a sequence of incrementing values.
- `copy_memory_block_2`: Copies sixteen words from a source address to a destination address and then calls another routine.

### Stack

- `allocate_and_initialize_task_structure_2`: Traverses a linked list to find the maximum value and then allocates a new data structure by calculating offsets.
- `copy_stack_data_4`: Copies sixteen words from a memory location to the stack before calling another routine.

### Delete

- `update_task_list_and_copy_data_7`: Traverses a linked list to find the maximum value and then updates a table with a new sequence of incrementing values.
- `copy_block_and_call_3`: Copies a sixteen-byte block of data from a memory location to a destination on the stack and then executes a function pointer.

### Ask

- `parse_startup_argument_string`: Stores the startup argument string and splits it into words at spaces, honouring quoted strings and turning the star-N and star-E escapes into a newline and an escape character.
- `invoke_two_function_pointers`: Calls two library routines through function pointers held at fixed negative offsets from a base register, then returns.
- `open_dos_library_with_setup`: Opens the DOS library by name through the system library vector and returns the result if the open succeeded.
- `forward_dos_read_call`: Calls the DOS library read routine through the stored library base and returns its result.
- `forward_dos_write_call`: Calls the DOS library write routine through the stored library base and returns its result.
- `forward_dos_input_call`: Calls the DOS library input routine through the stored library base to obtain the standard input handle, and returns it.
- `forward_dos_output_call`: Calls the DOS library output routine through the stored library base to obtain the standard output handle, and returns it.
- `call_exit`: Library stub: loads its arguments and the dos library base, calls Exit and returns its result to the caller.
- `prompt_yes_no_then_finish`: Shows a prompt, reads a line of input and repeats until its first character is y, n, Y, N or a newline.
- `count_string_length_2`: Walks a zero-terminated string one byte at a time, counting bytes until the terminator, and returns the count.

### Type

- `initialize_task_structure_4`: Traverses a linked list to find the maximum value and then populates a new data structure with a sequence of incrementing values.
- `copy_memory_block_and_call_9`: Copies sixteen bytes from a memory location offset from a pointer to a destination address on the stack, then executes a function via a pointer.

### Path

- `populate_task_structure`: Iterates through a linked list to find a maximum value and then populates a data structure with a sequence of incrementing values.
- `copy_memory_block_3`: Copies sixteen bytes from a source address to a destination address using a loop.

### RunBack

- `run_background_command_line`: Parses a command line for a background run: an optional -S switch with a number of at least 1600 is accepted, along with a digit flag, and then the named command is started in the background with its input and output sent to the null device.
- `write_zero_terminated_string`: Measures a zero-terminated string and writes it to the standard output through the DOS output and write routines.

### Break

- `update_task_list_and_copy_data`: Iterates through a linked list to find a maximum value, then updates a table entry and performs a series of memory block copies between a source buffer and a destination structure.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location to a destination address on the stack and then executes a function pointer.

### Info

- `update_task_list_and_copy_data_2`: Traverses a linked list to find the maximum value and then updates a table with a sequence of incrementing values.
- `copy_memory_block_and_call_2`: Copies sixteen words from a memory location offset from a pointer to a destination address on the stack, then executes a function pointer.

### Assign

- `update_task_list_and_copy_data_3`: Traverses a linked list to find the maximum value and updates a table with new entries.
- `copy_memory_block_and_call_3`: Copies sixteen bytes from a memory location offset from a register to a destination address on the stack, then executes a function pointer.

### EndCLI

- `update_task_list_and_copy_data_4`: Iterates through a linked list to find the maximum value and updates a table with new entries.
- `copy_block_and_call`: Copies sixteen words from a memory location to a destination pointed to by a stack value, then executes a function via a pointer.

### Rename

- `update_dynamic_task_list`: Iterates through a linked list to find a maximum value and then updates a table with new entries.
- `copy_memory_block_and_call_4`: Copies a block of sixteen bytes from a memory location to a destination address and then executes a function pointer.

### NewCLI

- `update_task_list_and_copy_data_5`: Traverses a linked list to find the maximum value and then updates a task structure with new data.
- `copy_block_and_call_2`: Copies sixteen words from a memory location offset by a register value to a stack-based destination.

### Echo

- `update_task_list_and_copy_context`: Traverses a linked list to find the maximum value and then updates a task structure with new data.
- `copy_memory_block_and_call_5`: Copies sixteen bytes from a memory location offset from a pointer to a destination address on the stack, then executes a function pointer.

### Quit

- `update_task_structure`: Iterates through a linked list to find a maximum value and then updates a task control block.
- `copy_memory_block_and_call_6`: Copies sixteen words from a memory location offset from a pointer into a destination address on the stack, then executes a function pointer.

### Execute

- `allocate_and_initialize_task_structure`: Traverses a linked list to find the maximum value and then allocates a new data structure by calculating offsets.
- `copy_memory_block_and_call_7`: Copies a sixteen-byte block of data from a memory location to a destination pointed to by the stack, then executes a function via a pointer.

### Copy

- `initialize_task_structure`: Calculates a maximum value from a linked list and populates a task control block with a sequence of incrementing values.
- `copy_stack_data`: Copies sixteen bytes from a memory location offset by a register to the stack and then calls another routine.

### List

- `initialize_task_structure_2`: Traverses a linked list to find the maximum value and then populates a new data structure with a header, a sequence of incrementing values, and a pointer to itself.
- `copy_memory_block_and_call_8`: Copies sixteen bytes from a memory location to a destination address and then executes a function pointer.

### MakeDir

- `update_task_list_and_copy_data_6`: Iterates through a linked list to find a maximum value and updates a table entry.
- `copy_data_and_call_function`: Copies sixteen bytes of data from a memory location to a destination pointed to by the stack, then executes a function via a pointer.

### Date

- `update_task_structure_2`: Traverses a linked list to find the maximum value and then modifies a task control block by updating its size and copying data from a source buffer.
- `copy_memory_block`: Copies sixteen bytes from a memory location to a destination pointed to by a stack value and then calls another routine.

### LoadWb

- `start_main_program`: Starts the main work from either a command-line invocation or a Workbench startup message.
- `finish_startup_with_status`: Records its status argument and makes a call through the exec library vector.
- `invoke_two_function_pointers_2`: The routine invokes two function pointers located at fixed negative offsets from a register and then returns.
- `execute_startup_sequence`: Calls a function through a pointer at a negative offset from a register, performs conditional logic based on a global variable, and executes further function calls through pointers.
- `call_dos_write`: Calls a function through a pointer at a fixed negative offset from a global variable.
- `execute_indirect_function`: Fetches a function pointer from a global variable and executes it.
- `execute_indirect_function_2`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_dup_lock`: Library stub: loads its arguments and the dos library base, calls DupLock and returns its result to the caller.
- `forward_dos_ioerr_call`: Calls the DOS library error-code routine through the stored library base and returns the error value it gives.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `start_resident_tool_process`: Starts a tool by finding its resident tag and creating a process for it, honouring a debug switch.
- `compare_strings_ordered`: Compares two zero-terminated strings character by character.
- `allocate_linked_entry_chain`: Walks a linked list, allocating a cleared memory block for each entry and duplicating a lock taken from the entry into it, then links the blocks together.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_get_msg`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `pass_params_to_subroutine`: Calls another routine with a global variable, its own parameter, and the address of an eight-byte stack slot, then returns.
- `format_buffer_and_forward`: Formats text with the operating system formatting routine into a local buffer, measures the resulting string, and when it is non-empty passes the buffer and its length on to another routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Protect

- `reconstruct_obfuscated_data_structure`: Traverses a linked list to find a maximum value and then populates a data structure with a sequence of incrementing values.
- `copy_stack_data_3`: Copies a block of sixteen bytes from a memory location to a stack address and then calls another routine.

### SetClock

- `startup`: Sets up system library pointers and standard input/output streams.
- `_exit`: Performs an exit sequence by calling a function pointer from a global structure and conditionally calling two other function pointers if a global message pointer is non-zero.
- `waitmsg`: Calls two library routines through function pointers held at fixed negative offsets from a base register, then returns.
- `openDOS`: Attempts to open the DOS library and handles errors by calling an exit routine if the base is not returned.
- `_Write`: Calls a function pointer located at a fixed offset from a DOS base pointer.
- `_Input`: Fetches a function pointer from a global variable and executes it.
- `_Output`: Fetches a function pointer from a location relative to a global base and executes it.
- `_Delay`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `_Execute`: Library stub: loads its arguments and the dos library base, calls Execute and returns its result to the caller.
- `probe_clock_chip_then_dispatch`: Checks for a clock chip at one of two fixed addresses, printing warnings when it is missing or found at the second address, and exits with a set code when no chip answers.
- `print_setclock_usage`: Prints three lines of usage text for the clock tool, covering its load and save options.
- `check_clock_chip_ticking`: Reads a clock chip register, waits seventy-five ticks through a delay call, reads it again, and returns whether the low nibble changed.
- `query_timer_device_value`: Sends a request with command number ten to the timer device through an I/O request block, waits for it to finish, returns the result stored in the request, and releases the block.
- `set_timer_device_value`: Sends a request with command number eleven to the timer device, passing a value from another routine, waits for completion, and releases the request.
- `open_timer_device_request_2`: Creates a message port and an extended request block, then opens the timer device with the given unit number.
- `close_timer_device_request`: If given a request block, deletes its reply port when one exists, closes the device, and deletes the request block.
- `convert_clock_fields_to_seconds`: Reads the clock-chip digit fields and combines them into a total count of seconds, adding days from a month table and yearly adjustments.
- `split_seconds_into_clock_fields`: Divides a total seconds count into calendar fields using divide and remainder routines, walks a month-length table to find the month, and stores each field in the clock buffer.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings after converting characters of the first to lower case.
- `compare_pending_char_order`: Compares the character held in a data register with the byte at the address register.
- `classify_char_order_pair`: Returns minus one when the character in the data register sorts below the byte at the address register, and one otherwise.
- `return_constant_one_4`: The routine moves a constant value of one into the accumulator and returns.
- `convert_char_to_lower`: Returns the character as a full-width value, with the upper-case letters A to Z converted to lower case by adding 32.
- `sign_extend_low_byte`: Returns the low byte of the data register sign-extended to a full value.
- `write_fixed_pattern_sequence`: Writes three fixed 32-bit patterns in sequence to a register at a fixed offset from a base pointer, then returns.
- `set_bits_and_poll_status`: Sets a fixed mask of bits in a control word, then polls a status byte up to about two hundred times and returns as soon as the status bit it tests is clear.
- `clear_control_bits`: Clears a fixed set of bits from a control word at a fixed offset from a base pointer, then returns.
- `_L3`: Writes one formatted message to standard output through the C library formatted-print routine, passing its argument through, then returns.
- `_AllocMem`: Calls the exec library memory allocation routine through the stored library base and returns its result.
- `_FreeMem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `_FindTask`: Calls the exec library find-task routine through the stored library base and returns its result.
- `_AllocSignal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `_FreeSignal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `_AddPort`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `_RemPort`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `_OpenDevice`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `_CloseDevice`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `_DoIO`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `POSDIV`: Divides a 32-bit value by a divisor and returns the quotient and remainder as a pair.
- `ulmult`: Multiplies two 32-bit values by splitting them into 16-bit halves and summing the partial products.
- `ulmodt`: Calls the shared division routine and returns, leaving the remainder in the registers for the caller.
- `uldivt`: Calls the shared division routine and returns the value it leaves in the second data register.
- `lmodt`: Calls the shared division routine and returns its remainder, negated when the sign flag held in an address register is non-zero.
- `ldivt`: Calls the shared division routine and returns its quotient, negated when the sign flag held in an address register is non-zero.
- `_CreatePort`: Allocates a signal bit and a 34-byte memory block, fills the port fields with the given name and the current task, and either adds the port to the public list or initializes an empty message list.
- `_DeletePort`: Removes the port from the public list when it is linked, marks it as deleted, frees its signal bit and releases its 34-byte memory block.
- `_CreateExtIO`: When given a reply port, allocates a memory block of the requested size, marks it as an I/O request, records the size and attaches the port.
- `_DeleteExtIO`: Clears specific fields in an I/O request structure and frees the memory block using the size stored within the structure, performing no operation if the pointer is zero.
- `_fprintf`: Passes its format argument and the address of its variable arguments on to the formatted-output routine, then returns.
- `__doprnt`: Formats text with the operating system formatting routine into a local 140-byte buffer, measures the result, and when it is non-empty writes it out through the write routine.
- `stuffChar`: Stores the low byte of the data register at the address held in the address register, then returns.
- `_NewList`: Initializes an empty list header in the given block so its head points to the slot just after it, clears the count word, and points the tail back at the header.

### Wait

- `run_timed_wait_command`: Opens the DOS library and parses the command arguments for a period in seconds or minutes, or a time of day after the UNTIL keyword.
- `call_and_accumulate_result`: Calls one routine, then calls another for a value and adds that value to the integer stored at the address register, then returns.
- `split_sum_into_quotient`: Adds a value to a stored accumulator, stores the remainder of that sum divided by a 16-bit divisor back into the accumulator, and returns the quotient, both truncated to 16 bits.
- `write_error_and_accumulate`: Writes an error message through library output routines, then adds a value returned by another routine into the integer at the address register.

### DefDisk

- `load_executable_hunks`: Loads an AmigaDOS executable from its hunk list.
- `stub_empty_return_routine`: The decompiled body is empty apart from a return, so this routine has no visible behaviour.
- `spin_delay_loop`: Spins in a loop that decrements the data register until it wraps past zero, with no other work, then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 240 into d0.
- `unpack_compressed_stream_backwards`: Unpacks a compressed byte stream from run-length codes.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_with_space_wrap`: Decrements a byte held in a data register once per pass for a 16-bit count, replacing a zero result with a space character.
- `stub_bad_instruction_data`: The bytes at this entry are not valid code; the decompiler shows only a bad-instruction marker, so the routine has no behaviour of its own.

### Shanghai (1986)(Activision)[v SystemZ v4.0].adf:CLI

- `initialize_system_and_parse_command_line`: Initializes system libraries and environment settings before parsing a command-line string to extract arguments, handling quoted strings and specific character escape sequences.
- `invoke_indirect_function_pointers`: The routine calls two functions via pointers stored at fixed negative offsets from a register.
- `execute_conditional_logic`: Calls a function pointer from a register, and if the result is zero, calls another function pointer and potentially executes a sequence of two more function pointers based on a global pointer.
- `forward_dos_open_call`: Calls the DOS library open routine through the stored library base and returns its result.
- `forward_dos_close_call`: Calls the DOS library close routine through the stored library base and returns its result.
- `forward_dos_input_call_2`: Calls the DOS library input routine through the stored library base and returns its result.
- `forward_dos_output_call_2`: Calls the DOS library output routine through the stored library base and returns its result.
- `forward_dos_lock_call`: Calls the DOS library lock routine through the stored library base and returns its result.
- `forward_dos_unlock_call`: Calls the DOS library unlock routine through the stored library base and returns its result.
- `forward_dos_duplock_call`: Calls the DOS library duplicate-lock routine through the stored library base and returns its result.
- `forward_dos_currentdir_call`: Calls the DOS library current-directory routine through the stored library base and returns its result.
- `forward_dos_execute_call`: Calls the DOS library execute routine through the stored library base and returns its result.
- `open_console_window_for_cli`: Opens a console window for a new command interpreter using the window specification and system path text.
- `allocate_bptr_entry_chain`: Walks a linked list, allocating a cleared memory block for each entry, recording a value taken from the entry in the block, and linking the blocks together.
- `free_bptr_entry_chain`: Walks the chain built by the allocation routine, releasing each entry value through another routine and then freeing the 8-byte entry block.
- `execute_indirect_call`: Calls a function through a pointer located at an offset from a global variable.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `invoke_task_function`: Executes a routine by dereferencing a pointer stored at a specific offset within a global variable.

### 12

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_constant`: Returns a constant value of zero to the caller.

### 16

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.

### 9

- `return_constant_zero`: Returns zero and does nothing else in the decompiled body.
- `return_zero_2`: This routine is empty and simply returns zero to the caller.

### 8

- `return_zero_3`: This routine is empty and simply returns a value of zero to the caller.

### arp.library

- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 39 into d0, then returns.
- `initialize_arp_library_base`: Library code from arp.library that stores its argument in the library base, tries to open a dependent library up to a few times, and on success links a new structure into the base.
- `collect_values_then_link_entry`: Library code from arp.library that fills a buffer with values fetched through a library routine until one is zero or the counter runs out, then links a new structure into the base.
- `finish_library_entry_setup`: Library code from arp.library that stores a pointer into the structure, makes three library calls to set up an entry, and on a non-zero result links it into the base.
- `poll_until_flag_then_count`: Library code from arp.library that no known caller reaches.
- `clear_slot_then_call_library`: Library code from arp.library.
- `allocate_block_with_link_word`: Library code from arp.library that, when given a non-zero size, allocates a memory block and writes a link word into its first slot pointing just past that slot.

### icon.library

- `return_zero_4`: This library routine is empty.
- `increment_counter_clear_bit`: Library code from icon.library that increments a 16-bit counter in the structure and clears one flag bit in a byte of the same structure.
- `decrement_library_refcount`: Library code from icon.library that decrements a 16-bit reference counter.
- `unlink_or_mark_pending`: Library code from icon.library that, when a counter field is zero, calls a library routine, unlinks the structure from its list and returns the value it was given.
- `set_pending_flag_return_zero`: Library code from icon.library that sets one flag bit in a byte of the structure and returns zero.
- `return_zero_5`: This routine is an empty function that returns zero to the caller.
- `open_dos_library_into_globals`: Library code from icon.library that stores its argument in the structure and opens the DOS library by name, copying two values into global variables on success and calling the alert routine on failure.
- `copy_two_values_into_globals`: Library code from icon.library that copies two 32-bit values from fixed offsets in a structure, referenced by an address register, into two global variables.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `forward_call_and_return_2`: Library code from icon.library that calls one other routine and then returns, with no processing of its own.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_write_icon_info`: This library routine unconditionally calls a helper routine to write icon information and then returns.
- `call_handler_pointer`: This library routine retrieves a function pointer from a memory location and calls it.
- `call_library_handler_2`: This library routine calls another routine and then returns.
- `forward_single_call`: Icon library routine that makes one call to another routine and then returns, with no other work in the pseudo-code.
- `trigger_icon_setup`: This library routine invokes a single initialization function to populate icon data structures and then returns.
- `dispatch_icon_handler`: This library routine calls a handler function retrieved from a pointer in a data structure and then returns.
- `dispatch_library_handler`: This library routine calls another routine and then returns.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_scan_entry_chain`: This routine calls another routine and then returns.
- `call_build_copy_name`: This library routine calls another routine to perform a name copy operation and then returns.
- `update_field_via_table_call`: Calls a function taken from a table inside the structure passed as the first argument, then writes the second argument into the structure that call returns at a fixed field offset.
- `chain_two_calls_conditionally`: Calls one routine with the last two arguments, and if it returns a non-zero value calls a second routine with the first two arguments and that value.
- `read_and_copy_icon_record`: Reads a record through a helper, then copies a 44-byte block and several fields from a stack buffer into the record and links it into a parent record when one is set.
- `copy_record_fields_and_link`: Copies a 44-byte block and several fields from a source area into the record named by the register inputs, then links the record into the parent it points to when one exists.
- `return_zero_stub_4`: This library routine is empty and immediately returns zero.
- `pack_record_and_forward`: Builds a small two-word header on the stack, copies a 44-byte block and several fields from the record given as the third argument into the stack buffer, then passes that buffer to one other routine.
- `read_info_icon_file`: Reads an icon (.info) file: it builds the file name from a text reference, opens it through a helper, reads a 78-byte header and checks its magic number and version, then reads the optional image and tool-data sections the header describes.
- `read_info_icon_sections`: Continues reading an icon file after its header is in place: it checks the magic number and version, reads the stored fields into the record, and allocates the optional image and tool-data blocks, copying each in from the file.
- `read_icon_optional_blocks`: Reads two stored fields into the icon record, then reads an optional block list and a further optional block, allocating memory for each and copying the data in from the file.
- `read_icon_pointer_list`: Fills a list of 4-byte entries from the file in a loop until the remaining byte count drops to four, then reads an optional extra value into the record.
- `return_zero_after_call`: Calls one other routine, discards its result, and returns zero.
- `return_from_subroutine_5`: This routine is empty and performs only a return instruction.
- `write_info_icon_file`: Writes an icon (.info) file: it allocates a 260-byte work buffer, builds the file name from a text reference, opens the file, then writes the header and the optional sections the record describes.
- `write_icon_record_sections`: Writes the optional sections of an icon record to a file: it saves one stored field, clears it for one flag combination, writes a block followed by the optional image and tool-data sections, then restores the field.
- `write_icon_tool_strings`: Writes the optional string list and extra value of an icon record: each string is written as a length followed by its text, then an optional trailing value is written.
- `write_remaining_string_list`: Writes the rest of a pointer-based string list and the optional extra value of an icon record, returning one on success and zero on failure.
- `write_string_pointer_list`: Walks a zero-terminated list of pointers, writing each entry as a length followed by its text, then writes an optional extra value.
- `drain_linked_list`: Repeatedly takes the next item from a list headed at a fixed offset in the second argument and passes it to another routine, until the list is empty.
- `empty_return_stub`: Does nothing and returns immediately; the pseudo-code is a bare return with no calls or memory access.
- `forward_with_shifted_pointer`: Calls the same routine with the first argument unchanged and the second argument advanced by a fixed offset, then returns.
- `allocate_icon_record`: Allocates a zeroed 168-byte record, initializes one of its embedded list fields, and reads its data through a helper.
- `forward_no_arg_call`: Library routine that makes a single call to an unnamed routine and returns, with no arguments or results of its own.
- `return_null_constant`: This library routine returns a zero value to the caller.
- `store_pair_next_slot`: Decrements a 16-bit index, growing a table through a helper when the index runs out, then stores two 32-bit values from its arguments into the next slot.
- `decrement_counter_and_store_pair`: Decrements a 16-bit counter, then stores two 32-bit values from its arguments into consecutive slots of a table at the counter's position, and returns one.
- `allocate_small_icon_record`: Allocates a zeroed 94-byte record, reads its data through one helper and then a second helper, freeing the record and returning zero if either fails.
- `execute_callback_and_return_zero`: Calls a function pointer retrieved from a global variable via a library base and returns zero.
- `call_library_then_return_zero`: Calls a function pointer taken from a global variable through the library base, with no arguments, then returns zero.
- `incremental_offset_recurse`: Calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `find_tagged_value_in_list`: Walks a zero-terminated list of pointers looking for an entry whose text matches the given key and has an equals sign at the matching position.
- `return_zero_constant_3`: This library routine returns a constant value of zero to the caller.
- `scan_entry_chain_for_length`: Scans a chain of entries using a helper that finds each next link, comparing each entry's length with a value derived from the second argument.
- `scan_entry_chain_for_length_2`: Scans a chain of entries using a helper that finds each next link, comparing each entry's length with a value derived from the second argument.
- `find_matching_length_entry`: Walks a chain of entries using a helper that finds each next link, comparing each entry's length with a value passed in a register.
- `find_first_matching_length_entry`: Searches a chain of entries from a null start: it returns zero at once when the start link is empty, and one when an entry with the expected length passes its check.
- `return_zero_value_2`: This routine is an empty function that returns a zero value to the caller.
- `build_copy_name_from_source`: Builds the name for a copy from its source name.
- `parse_copy_number_in_name`: Parses the copy number that follows a copy prefix in a name.
- `parse_decimal_digits_from_text`: Reads decimal digits from a text pointer into a number, and if any digits were read and the text that follows matches a prefix, it makes two follow-up calls before returning.
- `parse_copy_number_text`: Parses a decimal copy number from a name written in the copy-of form and checks the text that follows it.
- `char_to_decimal_digit`: Converts one character code to its decimal digit value, returning the value for the characters zero to nine and minus one for any other character.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `call_library_function_no_args`: Calls one library function through a library base pointer held in data, passing no arguments and returning nothing.
- `call_and_zero_d0`: This library routine calls a function through a pointer and then clears the d0 register.
- `execute_indirect_function_3`: Fetches a function pointer from a calculated offset within a global table and executes it.
- `execute_indirect_function_4`: Fetches a function pointer from a calculated offset within a global table and jumps to it.
- `execute_indirect_function_5`: Fetches a function pointer from a location within a table and executes it.
- `call_and_return_handler`: This library routine calls a function through a pointer and returns.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `invoke_indirect_function`: Fetches a function pointer from a location within a global table and executes it.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `complex_library_traversal`: Library code that calls another routine and uses the result to conditionally call a second routine or perform a cleanup call.
- `allocate_entry_with_defaults`: Allocates a cleared 96-byte record, sets a default value of ten in one of its fields and in the caller's short, then passes the record on to a helper.
- `return_zero_value`: This library routine returns zero.
- `check_call_result_matches`: Calls one routine with three arguments and returns its result when it equals the fourth argument, otherwise returns zero.
- `return_void`: This library routine returns without performing any operations.
- `compare_and_zero`: This library routine calls a function and then sets the return value to zero if the result does not match a specific parameter.
- `return_from_subroutine_4`: This routine is an empty function that performs a single return instruction.
- `forward_two_arg_result`: Calls one routine with two arguments and returns that result unchanged.
- `dispatch_library_function`: Calls one library function through the library base held in data, passing it the second argument, and returns nothing.
- `divide_unsigned_32_by_16`: Divides an unsigned 32-bit dividend by an unsigned divisor and returns the quotient and remainder together.
- `restoring_divide_loop_16`: Performs sixteen steps of restoring division, shifting bits from the dividend into a partial remainder and subtracting the divisor when it fits.
- `multiply_32_bit_integers`: Multiplies two 32-bit values held in registers using 16-bit partial products, and returns the low 32 bits of the result.
- `get_string_length`: Returns the number of characters before the zero terminator of a string.
- `find_char_in_string_2`: Scans a zero-terminated string for the first occurrence of a given character and returns a pointer to it, or null if the character is absent.
- `return_zero_stub_2`: Library routine that does nothing and returns zero.
- `copy_c_string`: Copies a zero-terminated string from the source pointer to the destination, including the terminator.
- `append_c_string`: Appends a zero-terminated source string to the end of the destination string, including its terminator.
- `strings_differ_up_to_count`: Compares up to a given number of characters of two strings and returns one if they differ, or zero if they match up to that count or the terminator is reached.
- `return_one`: This library routine returns one.
- `scan_string_limited_then_call`: Walks a string for up to a given count of characters and, if it reaches the terminator, makes one follow-up call.
- `return_from_routine`: This library routine performs a return from a subroutine.
- `copy_string_with_fixed_length`: Copies at most a given number of characters from a source string into the destination, stopping at its terminator, then fills the rest of the space with zero bytes.
- `return_from_subroutine_6`: This routine is an empty function that performs no operations before returning.
- `init_self_linked_list_header`: Initializes a list header in a memory block: it stores the block's own address plus four in the first word, clears the second word, and stores the block's address in the third word.
- `conditional_library_branching`: Returns one when the pointer argument is null.
- `walk_entry_list_applying_action`: Walks a linked list of entries, applying one helper to each entry and to the counted block it points to, then following the next link.
- `return_one_stub_2`: Does nothing and returns one; the pseudo-code has no calls or memory access.
- `return_zero_constant_2`: This library routine returns a constant zero value to the caller.
- `allocate_optional_node_with_field`: Allocates a cleared 20-byte node when the flag argument is non-zero, fills one of its fields from a call that uses the other arguments, and returns the node.
- `return_negative_one`: This library routine returns negative one.
- `build_linked_node_list`: Builds a linked list of 16-byte nodes, one per entry of a source description: each entry gets a cleared node, a block sized from the entry's count field, and a link to the previous node.
- `read_entry_node_list`: Reads node entries in a loop, allocating a cleared node for each and a block sized from the entry's count, then links them into a list.
- `return_all_ones_2`: This library routine returns a value consisting of all bits set to one.
- `return_all_ones`: This routine returns a value consisting of all bits set to one.
- `call_raw_do_fmt`: Calls the exec library's formatted-output routine through the library base pointer and returns nothing.
- `store_single_byte`: Writes one byte from a data register to the address held in an address register and returns nothing.

### diskfont.library

- `allocate_handle_store_value`: Calls a library function through the library base; if it returns a non-zero handle, stores the caller's value in a fixed field of that handle and calls a second library function with it.
- `check_and_set_font_handle`: Calls a library function through the library base; if it returns a non-zero handle, stores the argument in a fixed field of that handle and calls a second library function.
- `open_font_libraries_and_init`: Opens the libraries named in its text, and on success initializes a self-linked list header inside the structure and returns it.
- `release_unused_entries_and_close`: Walks a list of entries, removing those with no remaining users and releasing each through the segment-unload call.
- `free_owner_when_unused`: Checks whether the structure has no remaining users; if so it closes the two libraries, frees the structure, unlinks it from its list, and returns its stored value.
- `set_pending_flag_return_zero_2`: Sets the high bit of a flag byte in the structure to mark it as pending and returns zero.
- `return_immediately_5`: This library routine is empty and performs no operations before returning.
- `clear_flag_and_increment_counter`: Clears the high bit of a flag byte in the structure and increments a 16-bit user counter stored beside it.
- `decrement_users_then_maybe_release`: Decrements a 16-bit user counter in the structure; when the counter is still non-zero and the pending flag is set, it calls a library function and returns its result.
- `return_zero_stub_3`: Does nothing and returns zero; the pseudo-code has no calls or memory access.
- `build_font_entry_list`: Fills a buffer with font entries: it first copies the names of the fonts already held in the library's list, then, when a second flag is set, scans the FONTS directory for font files.
- `scan_font_directory_entries`: Scans the FONTS directory for font files and copies each matching entry's name and size fields into the caller's buffer.
- `copy_font_name_entries`: Copies font name entries from a text buffer into the output buffer one at a time, and when the flag is set also scans the FONTS directory.
- `scan_fonts_directory`: Scans the FONTS: directory for .font files and, for each one, reads its name and size into an entry in the output buffer.
- `copy_font_entries_from_list`: Adds font entries to the output buffer by copying each source record's name and fields, then runs the closing calls.
- `copy_name_add_font_entries`: Copies a font name from a text pointer into the work buffer, then adds entries for each matching font, scanning the FONTS directory for more when the flag is set.
- `add_verified_font_entries`: Adds font entries whose header reads back as the expected signature value, copying names and size fields into the output buffer, then continues the directory scan.
- `finish_font_scan_return_shortfall`: Closes the open handles used by the font scan and returns the shortfall: zero when all entries fit into the buffer, and the number of missing bytes otherwise.
- `return_zero_constant_4`: The routine immediately returns a zero value without performing any other operations or accessing memory.
- `score_font_match`: Scores how closely a requested font matches a candidate by comparing their sizes and style bits.
- `penalize_style_bit_differences`: Subtracts a weight from a running score for each of seven style bits that differs between the two style fields, and returns the adjusted score.
- `penalize_masked_style_differences`: Walks the style-bit positions selected by a mask, subtracting a weight at each position where the two fields differ, and returns the adjusted score.
- `penalize_alternate_style_bits`: Walks seven style-bit positions, subtracting a weight at each position the mask selects as different, and returns the adjusted score.
- `return_minimum_font_score`: Returns the lowest score constant, which marks a candidate font as unusable.
- `select_best_font_candidate`: Chooses the best matching font for a request: it copies the requested name, looks up the font-directory entry, then scores each candidate and keeps the highest-scoring one.
- `select_best_font_candidate_2`: Chooses the best matching font for a request: it looks up the font-directory entry, scores each candidate against the requested size and style, and keeps the highest-scoring one.
- `pick_font_entries_by_score`: Repeatedly scores the entries of a font table and places the best-scoring entry with a matching type code in the font list, until no entry scores above the minimum.
- `register_best_font_entries`: Repeatedly picks the best-scoring font entry from a table and registers it in the system font list and the graphics font list, returning the last chosen entry once no candidate remains.
- `pick_font_entries_by_score_2`: Chooses the best-scoring entry in a font table, marks it used, and repeats until no entry scores above the minimum.
- `pick_or_close_font_entries`: Repeatedly picks the best-scoring entry in a font table and either registers it in the font list or, when its type code does not match, closes it through the graphics close-font routine.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `dispatch_exec_alloc_mem_call`: Forwards its arguments to the exec library's memory allocation routine through the library base pointer.
- `dispatch_exec_free_mem_call`: Forwards its arguments to the exec library's memory-free routine through the library base pointer.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `dispatch_dos_open_call`: Forwards its arguments to the DOS library's Open routine through the library base pointer and returns nothing.
- `dispatch_dos_close_call`: Forwards its arguments to the DOS library's Close routine through the library base pointer and returns nothing.
- `dispatch_dos_read_call`: Forwards its arguments to the DOS library's Read routine through the library base pointer and returns nothing.
- `dispatch_dos_lock_call`: Forwards its arguments to the DOS library's Lock routine through the library base pointer and returns nothing.
- `dispatch_dos_unlock_call`: Forwards its arguments to the DOS library's UnLock routine through the library base pointer and returns nothing.
- `dispatch_dos_examine_call`: Forwards its arguments to the DOS library's Examine routine through the library base pointer and returns nothing.
- `dispatch_dos_ex_next_call`: Forwards its arguments to the DOS library's ExNext routine through the library base pointer and returns nothing.
- `dispatch_dos_current_dir_call`: Forwards its arguments to the DOS library's current-directory routine through the library base pointer and returns nothing.
- `dispatch_dos_load_seg_call`: Forwards its arguments to the DOS library's LoadSeg routine through the library base pointer and returns nothing.
- `dispatch_dos_unload_seg_call`: Forwards its arguments to the DOS library's UnLoadSeg routine through the library base pointer and returns nothing.
- `copy_null_terminated_string`: Copies a zero-terminated string from the source into the destination, including the terminator.
- `seek_string_end_then_call`: Advances a pointer to the zero terminator of a string and makes one follow-up call with that position and the second argument.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `get_string_length_2`: Returns the number of characters before the zero terminator of a string.
- `return_immediately_6`: This routine is an empty function that performs no operations and returns immediately to its caller.
- `compare_c_strings`: Compares two zero-terminated strings and returns zero when they are equal, minus one when the first differs with a smaller character, and one when it differs with a larger character.
- `compare_byte_before_pointer`: Compares the byte just before one pointer with the byte at another pointer, returning minus one when the earlier byte is smaller and one otherwise.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_character_in_string`: This library routine iterates through a null-terminated string to find a specific character and returns a pointer to its location.
- `find_char_from_current_position`: Advances from the current position in a string until it finds a preset character, returning a pointer to it, or null if the terminator comes first.
- `return_zero_constant_5`: The routine returns a zero value.
- `find_last_char_in_string`: Scans a zero-terminated string backward from its end for a given character and returns a pointer to its last occurrence, or null if none is found.
- `scan_back_for_preset_char`: Moves backward from the current position toward a limit, looking for a preset character.
- `return_zero_6`: This library routine performs no operations and returns a value of zero to the caller.
- `locate_path_separator`: Returns a pointer to the first slash in a path, or to the first colon when there is no slash, or to the start of the path when it contains neither.
- `return_from_subroutine_7`: This library routine executes a return.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: This library routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### sound1

- `return_zero_entry_stub`: Program entry point that returns zero immediately, with no calls or memory access.

### sound2

- `return_zero_constant_6`: The routine returns zero upon entry.

### sound3

- `return_zero_constant_7`: This routine returns a zero value.

### sound4

- `return_zero_constant_8`: The routine immediately returns zero.

### sound7

- `return_zero_7`: This routine is empty and simply returns a zero value to the caller.

### sound8

- `return_zero_8`: The routine performs no operations and returns a zero value to the caller.

### sound9

- `return_zero_9`: The routine performs no operations and returns zero to the caller.

### extra

- `stub_decompile_timeout`: Program entry in the extra segment with no recorded instructions.

### Shanghai (1986)(Activision)[v SystemZ v4.0].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_fixed_helper_then_library`: Calls a fixed helper routine and then makes a library call through a table entry held in a global variable, returning nothing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_function_no_args_2`: Calls a library function through a table pointer held in a global variable, passing no arguments and returning nothing.
- `call_indirect_function`: Loads a function pointer from a global variable and executes it.
- `modify_memory_values`: This routine clears the highest bit of a byte and increments a 16-bit value at a specific offset from a pointer.
