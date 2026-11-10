# Game 1654 (shanghai (1986)(activision)[v systemz v4.0][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01654). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909232 top-level bytes in 107 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 47 of 47 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 905 of 905 named (34 author symbols), 905 of 905 described (122 by rule, 309 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 714 of 714 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 24655 of 24655 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 213565 |
| code | 82792 |
| data | 242499 |
| padding | 21192 |
| unused | 349184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Shanghai (1986)(Activision)[v SystemZ v4.0][a].adf:Shanghai | 180 | 17140 | 545 |
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
| Shanghai (1986)(Activision)[v SystemZ v4.0][a].adf:CLI | 9 | 362 | 18 |
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
| Shanghai (1986)(Activision)[v SystemZ v4.0][a].adf:bootblock | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 3 |

## Cases handled

- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- shanghai (1986)(activision)[v systemz v4.0][a]/Shanghai (1986)(Activision)[v SystemZ v4.0][a].adf: blocks 198-198 (bytes 101376-101888) outside the filesystem hold data, not code (opcode density 0.059, entropy 4.75)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Shanghai (1986)(Activision)[v SystemZ v4.0][a].adf:Shanghai

- `initialize_filesystem_environment`: Determines the current directory and opens a file or directory structure.
- `execute_callback_chain`: Checks several global variables and, if non-zero, executes a series of function pointers stored within a structure and a linked list.
- `invoke_two_function_pointers`: The routine invokes two function pointers located at fixed negative offsets from a register and then returns.
- `execute_callback_chain_2`: Executes a sequence of function pointers stored within a structure and a linked list.
- `open_required_system_libraries`: Opens the diskfont, graphics, icon, intuition, and layers libraries, raising a specific alert code if any library fails to open.
- `allocate_fixed_size_buffers`: Allocates several fixed-size blocks through library calls and raises alert code 10 for each allocation that fails.
- `allocate_game_resources`: Allocates large display buffers and five smaller game buffers, reporting error codes if any allocation fails.
- `load_winter_tile_resources`: Loads three named resources from the winter directory through a loader routine and stores the results in a global table.
- `load_title_graphic`: Loads a title graphic by passing a pointer to a global variable and several parameters to a routine that copies picture rows into a buffer.
- `trace_rectangle_outline_calls`: Issues five calls that take four parameters in turn, tracing the corners of a rectangle and closing back at the first corner.
- `report_open_failure_error`: Looks up a numeric code in a table and, when it is recognised, reports an error through a formatted message.
- `check_bounds_inclusion`: Checks if two values are contained within a range defined by a four-element table.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `compute_bit_masks_once`: On first use it calls a routine with a fixed value and sets a flag.
- `poll_request_completion_status`: Reads a request record through a stored pointer and copies its header into a local buffer.
- `initialize_hardware_and_buffers`: Executes a sequence of setup calls including resource allocation, bitmask computation, and a loop that queries a device until it returns a non-zero value.
- `copy_picture_rows_to_buffer`: Copies a counted list of 40-byte picture rows from a source table into a buffer and passes the result to a drawing call.
- `emit_packed_word_fields`: Walks a range of 16-bit words, splitting each into a high byte and two four-bit fields.
- `reset_thirty_two_entries`: Calls one library routine for each index from zero to thirty-one, passing zeros as the remaining arguments.
- `forward_global_handle_to_routine_2`: Passes a global handle to another routine and returns.
- `forward_global_handle_to_routine`: Passes a global handle to one further routine and returns.
- `select_record_and_set_mode`: Reads a 16-byte table entry chosen by the argument and passes its fields to a routine.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `forward_library_call_14`: Calls the library routine 132 bytes below the base held in a global, then returns.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `forward_library_call_15`: Calls the library routine 198 bytes below the base held in a global, then returns.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_264_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `format_string_to_output`: Walks a printf-style format string, copying literal bytes and collapsing doubled percent signs, while sending output through a buffer.
- `open_timer_device_request_2`: Creates a message port and an I/O request block, then opens timer.device with that request.
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
- `fill_global_table_entries`: Loops twenty times calling another routine and stores the result into successive entries of a global variable table, then loops a variable number of times calling another routine with an incrementing index, and finally calls another routine with two parameters.
- `build_sprite_entry_table`: Fills a table of 34-byte entries, one per item in four groups whose sizes come from a global array, linking each entry to its position and a pointer.
- `retry_library_until_nonzero`: Calls a library routine repeatedly with a global argument until it returns a nonzero result.
- `mark_list_entry_character`: Follows a linked list from a head pointer for a given number of steps, then writes either a space or an asterisk into that node's text buffer, depending on the flag argument.
- `copy_bytes_forward`: Copies the given number of bytes one at a time from a source buffer to a destination buffer.
- `store_long_in_linked_field`: Writes a 32-bit value into a field of the structure that a pointer at offset twenty-six of the argument refers to.
- `forward_library_call_2`: Calls the library routine 30 bytes below the base stored in a global, then returns.
- `forward_library_call_3`: Calls the library routine 54 bytes below the base stored in a global, then returns.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `forward_library_call_4`: Calls the library routine 66 bytes below the base stored in a global, then returns.
- `forward_library_call_5`: Calls the library routine 72 bytes below the base stored in a global, then returns.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_library_call_6`: Calls the library routine 228 bytes below the base stored in a global, then returns.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `forward_library_call_7`: Calls the library routine 240 bytes below the base stored in a global, then returns.
- `forward_library_call_8`: Calls the library routine 246 bytes below the base stored in a global, then returns.
- `forward_library_call_9`: Calls the library routine 252 bytes below the base held in a global, then returns.
- `call_library_vector_258`: Library stub: loads a library base from a global variable and calls the library function at offset minus 258 of that base, returning its result to the caller.
- `forward_library_call_10`: Calls the library routine 264 bytes below the base held in a global, then returns.
- `forward_library_call_11`: Calls the library routine 270 bytes below the base held in a global, then returns.
- `forward_library_call_12`: Calls the library routine 282 bytes below the base held in a global, then returns.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `forward_library_call_13`: Calls the library routine 354 bytes below the base held in a global, then returns.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
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
- `allocate_five_game_buffers`: Allocates five buffers of 2560, 260, 36, 160 and 1024 bytes with memory-type flags, reporting an error code if any allocation fails.
- `call_routine_with_fallback`: Calls a routine with the fixed argument four and two stored values, then subtracts one from its result.
- `request_numbered_resource`: Makes a library call with two stored pointers, a specific identifier, and a flag, then passes any nonzero result to another routine.
- `call_with_fallback_five`: Calls a routine with the fixed argument five and two stored values, then subtracts one from its result.
- `process_menu_navigation_2`: Updates a menu selection index and manages the display of menu entries based on input values.
- `update_menu_selection_and_display`: Updates a menu selection index based on input values and manages the display of menu entries.
- `traverse_linked_list_and_process_nodes`: Iterates through a linked list of structures, performing conditional logic and calling other routines based on the values found within each node.
- `process_menu_navigation`: Updates a menu selection index based on input values and manages the display of menu entries.
- `draw_frame_around_structure`: Computes a rectangle from fields of a stored structure, offset by fixed amounts, and passes its four coordinates to a drawing routine.
- `draw_rectangle_border`: Fills a rectangle inside the given corners and draws its outline with eight line-drawing calls.
- `redraw_stored_item_region`: Runs three setup calls, then calls routines on a stored item's field and on a second global offset to redraw its region.
- `bracket_call_with_handle`: Brackets a call to another routine between two calls to the same routine using a stored handle, passing zero before the call and negative one after.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `forward_library_call`: Passes two global pointers and a global address to a library routine, then returns.
- `parse_and_populate_record_buffer`: Parses a string to locate a record entry and validates its header.
- `process_record_table_entries_2`: Iterates through a table of records to extract signed integers and update global variables based on conditional checks.
- `process_record_table`: Iterates through a table of records to extract signed integers and update various global variables based on conditional checks.
- `process_record_table_entries`: Iterates through a table of records, performing conditional logic based on field values to either clear entries or extract signed integers into a destination array.
- `populate_signed_value_table_2`: Iterates through a record table to extract short integers and store them in a destination array.
- `populate_signed_value_table`: Iterates through a record table to extract short integers and store them in a destination array.
- `process_file_records`: Iterates through a table of records, performing file operations and updating a global index based on the results.
- `unlock_listing_and_return`: Unlocks the lock held in the frame and returns three to its caller.
- `store_pointer_to_global`: Stores a value from a global variable into another global variable at a specific offset.
- `build_game_file_path`: Builds a full path from a directory and a file name, adds a separator when needed, then locks the result.
- `read_or_write_data_file`: Opens a file through a library call chosen by the mode argument, then reads or writes a block of data with it.
- `request_resource_in_current_dir`: Locks the current directory, makes a resource request using a provided pointer, and handles the returned code by calling another routine if it is non-zero.
- `compare_table_sums`: Calculates the sum of a table of values and compares it against a second sum derived from a different calculation.
- `sum_table_values`: Calculates a sum by iterating through a table of values pointed to by a global variable.
- `compare_stored_totals`: Compares two stored values and calls another routine if they are not equal.
- `apply_stored_option_settings`: Reads several stored option flags and selects matching setup calls for each one, then loops over four slots to apply the configuration.
- `invoke_record_for_large_key`: Searches a table of three-field records for a matching key value.
- `find_matching_record`: Searches through a table of records for an entry matching a specific value and calls another routine with a field from that record.
- `invoke_record_when_within_count`: Passes a field of a stored record to a routine when one counter value is at most another.
- `compare_string_tables`: Compares a provided string against entries in a table by iterating through the table and performing case-insensitive byte comparisons.
- `compare_string_data`: Compares two strings byte by byte within a loop and returns a flag indicating if they are identical.
- `get_offset_value`: Returns a 32-bit value from a specific offset relative to a pointer.
- `convert_char_to_uppercase`: Returns the byte unchanged unless it is a lowercase letter, in which case it returns the matching uppercase letter.
- `accumulate_file_info_value`: Locks a named path and reads its information block into a 36-byte buffer.
- `forward_library_call_16`: Calls the library routine 42 bytes below the base stored in a global, then returns.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `forward_library_call_17`: Calls the library routine 150 bytes below the base stored in a global, then returns.
- `call_library_vector_30_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
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
- `execute_pointer_table`: Checks a flag and then executes a series of specific routines and a loop that calls up to ten pointers from a table.
- `iterate_and_call_table_pointers`: Initializes a counter to zero and iterates through a table of pointers up to ten entries.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `start_sound_voices`: Clamps a level argument to a maximum of three and selects either five or nine voices based on that value.
- `start_sound_from_table`: Clamps a level argument to eight, releases the channel bits held in a mask, then allocates free channel bits for each entry of a table selected by that level.
- `release_audio_channels_2`: Updates several fields in a global structure and iterates through a bitmask to release active audio channels using a table.
- `release_audio_channels`: Checks a bitmask for active channels and calls another routine to release them based on values retrieved from a table.
- `return_without_effect`: Does nothing and returns to its caller.
- `create_port`: Allocates a signal bit and a memory block, initializes fields in the block with parameters and the current task pointer, and either initializes a message list or adds the port to a global list.
- `delete_port`: The routine removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `empty_stub_function`: The routine contains a block of 31 68000 instructions but ends by returning without performing any observable effect on memory, registers, or I/O.
- `execute_indirect_jump_2`: Performs a loop that checks various global flags and conditions before executing an indirect jump through a table of function pointers.
- `validate_index_and_jump`: Compares a value against a table to increment a global index and performs an indirect jump through a table of function pointers.
- `increment_and_validate_index`: Compares a value from a table against a global variable and increments the index if they match.
- `execute_indirect_jump`: Compares a value from a table against a global variable and performs an indirect jump to a code address if they match.
- `scan_game_file_entries`: Iterates through a list of file entries to locate specific game files with .slt or .trn extensions.
- `call_routine_and_return`: Calls another routine with no arguments and returns.
- `copy_string_to_offset`: The routine calls another routine with the first argument and then copies a null-terminated byte string from a pointer argument into memory starting at the returned value plus the first argument, byte by byte, stopping at the null terminator.
- `copy_string_to_buffer`: The routine obtains a destination address by adding the first argument to the result of another routine, then copies bytes from a source pointer into that buffer until a null byte is encountered or a length counter expires, and finally writes a terminating null byte.
- `expand_format_string`: Expands a format string into a destination buffer using a pointer stored in a global variable, handling escape sequences and buffer exhaustion.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `allocate_and_init_block`: The routine allocates a memory block sized to the input plus a constant, stores the size at an offset inside the block, calls another routine with a global variable and the block address, and returns the block address plus a constant offset.
- `allocate_memory_block`: Attempts to allocate a memory block of a specified size and returns negative one if the allocation fails.
- `return_minus_one`: The routine is unreachable and consists of a single instruction that returns a constant value.
- `free_linked_memory_blocks`: The routine walks a singly linked list whose head is stored in a global variable.
- `link_pointer_to_structure`: The routine stores a pointer to the second argument structure in the first argument's second word and in the first word when it is zero, and writes the first word of the first argument into the second word of the second argument structure.
- `parse_string_table`: Parses a null-terminated string into tokens delimited by spaces or double quotes, storing pointers to each token in a table and terminating tokens in place.
- `count_string_bytes`: The routine receives a pointer offset and scans forward byte by byte until it finds a zero byte, counting the number of bytes examined.
- `parse_recursive_data_stream`: Recursively processes a data stream by reading bytes and updating status flags within a control structure.
- `process_data_stream`: Reads bytes from a memory stream and updates status flags within a structure based on the results.
- `read_stream_byte`: Reads a byte from a memory stream and updates status flags within a control structure.
- `consume_stream_byte_2`: Reads a byte from a memory stream, updates status flags based on the result, and manages the stream pointer position.
- `consume_stream_byte`: Reads a byte from a memory stream and updates the stream's internal state and status flags.
- `fetch_stream_byte`: Retrieves a byte from a memory stream and increments the stream pointer, or recursively calls itself if the stream is exhausted.
- `forward_stream_to_helper`: Passes a value from an offset within a structure to another routine and returns.
- `write_byte_with_retry`: Writes a byte from a register to a buffer using a state structure to track progress, retries, and error flags.
- `reset_target_flags`: The routine checks a flag in a structure and a global variable, then calls another routine with a global pointer.
- `parse_format_string`: Parses a string by checking for specific characters and uses a jump table to call various subroutines based on the input.
- `format_numeric_string_4`: Calculates the required buffer length for a numeric value, including sign and digit counts, and writes the formatted characters into a destination buffer.
- `format_score_string`: Calculates the character length required for a numeric value and writes the resulting formatted string into a destination buffer.
- `format_numeric_string_3`: Converts a numeric value into a formatted string by calculating the required character length and appending digits or signs to a destination buffer.
- `decrement_and_copy_bytes`: The routine decrements a counter in a structure, then enters a loop that copies bytes from a source to a destination while incrementing a separate counter, finally storing the counter value into a pointer field.
- `copy_buffer_elements`: Decrements counters and copies bytes from a source buffer to a destination buffer in two separate loops.
- `decrement_and_copy_buffer_elements`: Decrements a value at a specific offset within a structure and enters a loop that continues as long as the value remains non-negative.
- `copy_value_and_increment_counter`: The routine copies a value from a location on the stack to a field in a structure pointed to by the frame pointer and returns the value of another field from that structure plus one.
- `format_floating_point_string`: Converts a numeric value into a formatted string representation, handling decimal precision, sign indicators, and scientific notation components.
- `format_numeric_string_buffer`: Converts a numeric value into a formatted string by calculating decimal digits and scientific notation components.
- `format_scientific_notation_string_6`: Converts a floating-point value into a string representation in scientific notation.
- `format_scientific_notation_string_5`: Converts a floating-point value into a string representation using scientific notation.
- `format_scientific_notation_string_4`: Converts a floating-point value into a string representation using scientific notation.
- `format_scientific_notation_string_3`: Converts a floating-point value into a string representation using scientific notation.
- `process_nested_data`: Executes nested loops to copy bytes from a source to a destination while managing multiple decrementing counters and conditional buffer updates.
- `process_nested_data_loops`: The routine executes multiple nested loops that decrement counters, copy bytes from various sources into a destination buffer, and update multiple counters and pointer fields based on conditional checks.
- `copy_data_via_nested_loops_2`: Performs complex nested loop operations to copy bytes from a source table into a destination buffer.
- `copy_data_via_nested_loops`: This routine performs complex nested loop operations to copy bytes from a source table into a destination buffer.
- `copy_data_with_counters_2`: Copies bytes from a source to a destination using nested loops and updates several counters, including a conditional block for remaining data.
- `copy_data_with_counters`: Copies bytes from a source buffer to a destination buffer using nested loops and multiple decrementing counters.
- `format_scientific_notation_string_2`: Converts a floating-point value into a string representation using scientific notation.
- `format_scientific_notation_string`: Converts a numeric value into a string representation using scientific notation, including a decimal point, an exponent indicator, and a sign character.
- `format_numeric_string`: Converts an integer value into a sequence of ASCII characters and stores them in a destination buffer.
- `format_signed_integer_string`: Converts a signed integer into a character string by prepending a sign character and iteratively appending numeric digits.
- `format_numeric_string_2`: Converts a numeric value into a string of characters by performing division and storing the resulting digits into a buffer.
- `decrement_and_copy_data`: Decrements a value in a structure within a loop, copying a byte from a source to a destination and incrementing several counters until a limit is reached.
- `update_structure_field_and_increment`: Copies a value from the stack into a field within a structure pointed to by a frame pointer, then returns the value of a different field from that same structure incremented by one.
- `create_ext_io`: The routine allocates memory for an I/O request structure when the first argument is non-zero, initializes specific fields with constants and arguments, and returns the pointer to the block or zero.
- `delete_ext_io`: The routine clears specific fields in the I/O request structure and frees the memory block using the size stored within the structure, performing no operation if the pointer is zero.
- `create_std_io`: Calls another routine to create an extended I/O structure for the supplied pointer, passing a constant size argument.
- `delete_std_io`: Receives a pointer argument and forwards it unchanged to another routine that deletes an extended I/O.
- `combine_pair_into_value`: Calls a routine that fills two values, then returns the first multiplied by two hundred and fifty-six plus the second shifted right by twelve bits.
- `draw_menu_list_items`: Selects a 30-byte list record and stores a value in its text field.
- `draw_cursor_at_index`: Computes a cursor position from the current index and a stored offset, then passes that position to two drawing routines.
- `draw_two_panel_frames`: Draws a pair of framed panels, each built from two coordinate pairs taken from the arguments and fixed sizes of twenty and twenty-three.
- `draw_four_corner_markers`: Sets a mode value of fifteen, then draws four markers at the corners of a rectangle offset by nineteen and twenty-two units from the given position.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `clear_entry_flag_bit`: Looks up an entry by index, then clears bit sixteen in its flag word.
- `clear_entry_flags_masked`: Looks up an entry by index, then clears the bits given by a mask from its flag word.
- `find_list_node_at_index`: Follows the next pointers of a linked list from the head pointer stored at an offset, for the given number of steps.
- `return_immediately_5`: This routine performs no operations and returns immediately to its caller.
- `store_four_word_fields`: Writes four 16-bit values into consecutive fields of the record at the pointer argument.
- `return_from_subroutine_2`: The routine executes a single instruction to return control to the calling procedure.
- `next_pseudo_random_word`: Shifts a 32-bit state left by one bit and XORs in a feedback constant when the top bit was set, then returns the low sixteen bits.
- `count_list_entries`: Counts the nodes of the linked list whose head pointer is stored at a fixed offset in the structure argument.
- `draw_entry_when_visible`: Draws the entry at its stored coordinates when its high flag bit is clear.
- `return_immediately_4`: This routine performs no operations and returns immediately to the caller.
- `draw_two_screen_bands`: Draws two horizontal bands across a screen width of 320 units at fixed positions, each followed by a second drawing call.
- `set_entry_highlight_state`: Sets or clears the high flag bit of an entry, calling library routines when the state changes.
- `draw_three_screen_bands`: Draws three horizontal bands across a screen width of 320 units at fixed offsets, each followed by a second drawing call.
- `draw_text_string`: Sets drawing mode values, measures the length of a null-terminated string, then draws the string with a library call.
- `draw_text_with_colour`: Like its sibling, measures a null-terminated string and draws it, but takes the drawing mode from its second argument.
- `nop_function`: The routine is empty and performs no operations before returning.
- `pass_two_word_arguments`: Passes two 16-bit arguments to another routine and returns.
- `copy_eight_bytes_to_local`: Copies eight bytes from the source pointer into a local buffer, then returns without using the copy.
- `draw_clipped_tile_area`: Calculates distances from a position to neighboring cell boundaries using a table and clamps negative results to zero.
- `check_tile_in_rectangle`: Calculates the grid coordinates of a specific tile and determines if it falls within the boundaries of a defined rectangular area.
- `validate_rectangle_containment`: Calculates the width and height of a rectangle by comparing grid values at specific boundary indices and clamps the results to zero.
- `check_rectangle_intersection`: Calculates a value based on a grid lookup and checks if a rectangle defined by a pointer overlaps with a boundary rectangle.
- `verify_rect_intersection`: Checks if a rectangle defined by two coordinate pairs is contained within or overlaps a boundary rectangle defined by a four-element table.
- `validate_range_inclusion_2`: Checks if a range defined by two values is contained within a boundary defined by a four-element table.
- `validate_range_inclusion`: Checks if a range defined by two values is contained within a boundary defined by a four-element table.
- `check_boundary_overlap`: Verifies if a specific value and a range overlap with a boundary defined by a four-element table.
- `return_zero_constant_2`: Returns zero as a result and performs no other operations.
- `lookup_grid_neighbour_value`: Computes a neighbouring cell index on a fifteen-column grid from a base index and offsets.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `mark_entry_and_refresh_rectangle`: Sets the high flag bit of a sprite entry, computes a rectangle enlarged by a few units around its position and size, and passes it to a drawing routine after a library call.
- `forward_record_position`: Passes the first two 16-bit fields of a record, together with a further argument, to another routine.
- `forward_zero_argument`: Calls another routine with the argument zero and returns.
- `advance_record_position`: Increments two fields of a record by one, then passes the record and a value to a routine.
- `draw_bordered_frame_pattern`: Draws a patterned border around a rectangle described by a record, repeating the edge segments once per step of a count derived from a global value.
- `forward_bounds_record_to_handler`: Copies four 16-bit values from the caller's record into local storage and passes them, along with the first two values, to a routine whose pointer is read from a global table.
- `draw_bounded_region_passes`: Derives a width and height from the four-value bounds record and issues four calls to one routine with those dimensions.
- `clip_region_then_draw`: Returns without action when the starting point lies outside the bitmap's width and height.
- `set_global_word_value`: Stores its single 16-bit argument into a fixed global variable and returns.
- `return_without_action`: Contains only a return instruction, so calling it has no effect.
- `save_blocks_to_new_file`: Opens the file named by a global string in new-file mode and, if the open succeeds, writes four 8000-byte blocks from a pointer table followed by a 32-byte global block.
- `write_table_blocks_then_close`: Writes four 8000-byte blocks from a pointer table and then a 32-byte global block to the file handle kept in the frame, and then closes that handle.
- `write_global_block_and_close`: Writes a 32-byte global block to the file handle kept in the frame and then closes it.
- `draw_fixed_position_sequence`: Makes eight calls through a graphics library base: a single call with the value 4, four calls with coordinates computed from global values, a call with the value 0, and two more coordinate calls.
- `set_flag_for_global_match`: Compares its pointer argument with the pointer to one particular global buffer and sets a global flag to 1 when they match.
- `compute_signed_division_quotient`: Divides the value in the first register by the value in the second using a 32-step restoring division on absolute values.
- `read_then_write_back_value`: Calls one routine with selector 0 to fetch a value into a local slot, makes two parameterless calls, then sets the slot to a register result and passes it back with selector 1.
- `pass_frame_slot_selector_one`: Passes selector 1 and a pointer to a frame slot at offset 4 to a routine and then returns.
- `transfer_saved_long_both_ways`: When the first argument is zero, copies a saved 32-bit value into the location given by the pointer argument.
- `find_free_slot_and_call`: Scans a table of 24-byte records for the first one whose byte at offset 16 is zero, stopping at a fixed end marker.
- `call_handler_if_slot_valid`: Returns zero when the slot pointer kept in the frame equals the table end marker.
- `forward_frame_args_to_handler`: Passes two arguments and a slot pointer from the caller's frame to a routine and returns with no result of its own.
- `open_stream_from_mode_string`: Parses a mode string whose first character selects read, write or append and whose optional second character selects binary mode or a plus sign for update.
- `open_stream_from_frame_mode`: Performs the same mode-string parsing and file opening as its sibling, reading the mode and the stream record from the calling frame.
- `open_append_stream_and_init`: Opens a file for appending by combining the supplied flags with write-create bits and a fixed permission value, then seeks to the end.
- `seek_end_and_init_stream`: Seeks the file handle to its end and then stores a mode value of 2, or 128 when the plus-sign flag is present.
- `finish_stream_record_mode_two`: Stores the mode value 2 and fills a stream record with its buffer pointer and mode bits, returning the record.
- `open_read_stream_and_init`: Opens a file for reading, adding update access when a plus sign is present, and returns zero if the open fails.
- `open_existing_file_and_init`: Opens the file named by the first argument with a flags value kept in the frame and returns zero if the open fails.
- `finish_stream_record_plus_mode`: Stores a mode value of 1, or 128 when the plus-sign flag is present, and fills a stream record with its buffer pointer and mode bits.
- `finish_stream_record_mode_one`: Stores a mode value of 1 and fills a stream record with its buffer pointer and mode bits, returning the record.
- `open_write_stream_and_init`: Opens a file for writing with the write flags and a fixed permission value, adding update access when a plus sign is present, and returns zero on failure.
- `open_update_stream_and_init`: Opens a file with the frame's flags combined with write bits and a fixed permission value, returning zero on failure.
- `finish_stream_record_write_mode`: Stores a mode value of 2, or 128 when the plus-sign flag is present, then fills a stream record with its buffer pointer and mode bits and returns it.
- `update_stream_record`: Modifies a record based on a condition, updates a pointer, and clears several fields within the record structure.
- `return_zero_constant`: Returns the value zero and performs no other work.
- `fill_stream_record_fields`: Fills a stream record from the frame.
- `init_stream_record_no_buffer`: Clears the stream record's buffer pointer and counters, then stores the mode byte from the frame.
- `clear_object_links`: Makes a release call when a flag bit is set, then calls a routine with two stored fields if the related flags are clear and the second field is non-zero.
- `check_flag_and_return_pointer`: Runs a setup call when a global is set, then for an in-range index looks up a slot and checks its flag byte.
- `allocate_file_slot_and_open`: Searches the slot table for a free entry and returns -1 with a status code when the table is full.
- `validate_slot_and_open_file`: Rejects an index equal to the table size, then sets the slot's access bits from the mode argument and opens the file in an existing-file or new-file mode.
- `lookup_slot_and_open_file`: Finds the slot for the current index through a lookup, sets its access bits from the mode argument, and opens the file in an existing-file or new-file mode.
- `open_file_into_prepared_slot`: Marks the slot as in use and sets its access bits from the mode argument, rejecting unsupported combinations with a status code.
- `open_file_with_slot_access`: Sets access value 32 in the slot, adds an option bit from the mode argument, and optionally runs a setup call.
- `open_file_with_access_only`: Sets access value 32 in the slot, optionally runs a setup call, and opens the file in an existing-file or new-file mode chosen by the write bits.
- `open_file_with_combined_option`: Sets access value 96 in the slot with an option bit taken from the mode argument, optionally runs a setup call, and opens the file in an existing-file or new-file mode chosen by the write bits.
- `open_file_with_combined_access`: Sets access value 96 in the slot, optionally runs a setup call, and opens the file in an existing-file or new-file mode chosen by the write bits.
- `reject_invalid_slot_access`: Clears the slot's flag byte, sets a global status code of 22 and returns failure.
- `open_file_into_slot_tail`: Optionally runs a setup call when a flag is set, then opens the file in an existing-file or new-file mode chosen by the write bits and stores the handle.
- `open_existing_file_into_slot`: Opens the file in existing-file mode and stores the handle in the slot.
- `return_frame_index_value`: Returns the value stored in the frame and changes nothing else.
- `reissue_with_adjusted_mode_flags`: Keeps the top bit of the second argument, adds write-related bits, and clears the top bit from the remaining value.
- `dispatch_io_on_slot_handle`: Looks up the file record for the slot index and, when one exists, passes its handle with the other arguments to an I/O routine.
- `read_from_slot_handle`: Reads from the file whose handle is held in the slot record into a buffer for the requested length and returns the result of the read.
- `write_to_slot_handle`: Looks up the slot record for a valid index and, when its flag bit is set, first seeks to the end of the file.
- `seek_file_with_offset_table`: Looks up the file record and, for an index below three, seeks the file to an offset taken from a global table, recording status 22 if that seek fails.
- `close_handle_if_flag_clear`: Looks up the file record for the given index and returns failure when none exists.
- `delete_file_with_status`: Deletes the file named by its argument and returns zero on success.
- `forward_single_argument`: Passes its single argument to a routine and returns without using a result.
- `rename_file_with_status`: Renames a file from the first name to the second.
- `return_zero`: This routine is empty and simply returns a value of zero to the caller.
- `open_global_file_handle`: Opens the file named by the global string in existing-file mode and stores the resulting handle in a global variable.
- `read_byte_with_pending_pushback`: Runs an optional setup call and then returns a pending byte held in a global when one is present, clearing it.
- `read_single_byte_from_handle`: Reads one byte from the global file handle into a local slot and returns that byte.
- `write_byte_to_handle`: Runs optional setup calls when two global values are in particular states, then writes its single-byte argument to the global file handle.
- `store_byte_if_empty`: Returns -1 when the global byte is already set.
- `store_argument_byte_global`: Copies the low byte of the first frame argument into a global byte variable.
- `close_active_handles`: Walks a table of entries up to a global count and closes each entry whose first byte is nonzero and whose flag bit is clear.
- `init_drive_unit_and_dma`: Copies a 20-byte template into the caller's record and sets one unit-select bit in it from the second argument.
- `return_fixed_numeric_constant`: Returns the fixed value 6875 and performs no other work.
- `copy_string_until_terminator`: Copies bytes from the source string into the destination, including the terminating zero byte, and returns the destination pointer.
- `copy_string_with_length_limit`: Copies at most a given number of bytes from the source into the destination, stopping after a zero byte.
- `initialise_session_state_fields`: Copies two entries from a text table into a buffer and clears five state fields.
- `flip_flag_around_calls`: Calls a routine with the value 31, flips a state flag, calls it with the value 15 and flips the flag back.
- `update_countdown_on_timer_tick`: Compares a timer reading with a stored value and, once at least 256 units have passed or a pending flag is set, decrements a countdown.
- `format_and_draw_time_text`: Converts a count of 60-unit intervals into minutes and seconds and formats it as text with a leading zero where needed.
- `draw_filled_box_with_outline`: Sets the pen to 31 and fills a block 34 units wide and 11 high at an offset from the given point.
- `switch_stored_mode_value`: Calls a routine with the previously stored value and the new argument, then stores the argument as the current value.
- `prepare_selection_screen_loop`: Initialises a record and its text buffers, then loops reading input values until the value is 1 or 2.
- `poll_selection_and_store_interval`: Repeats until the value read is 1 or 2, storing values from 3 to 6 as the selection and updating the entries through a routine.
- `refresh_selection_display`: Sets a state flag from bit 128 of a record field, then calls two routines with the shared text buffer and the stored interval value.
- `clear_flag_and_refresh_display`: Clears the state flag and then calls two routines with the shared text buffer and the stored interval value.
- `update_linked_entries_by_id`: Walks a linked list of records and, for each record whose flag bit is set and whose second flag bit is clear, stores a base value in its field.
- `invoke_single_routine_once`: Calls one routine with no arguments and returns.
- `show_result_screen_until_input`: Prepares a text buffer and draws a result message chosen from the stored scores, the consecutive-pass count and a tie check.
- `position_four_list_entries`: Walks four consecutive entries of a list, storing a value from a parameter array into each one and positioning it at half the gap between a global width and the measured width.
- `draw_numeric_field_for_slot`: Draws the frame for the current slot and prints its value in a two-character numeric field with a colon prefix.
- `find_text_length_fitting_width`: Measures a string and shortens the character count one step at a time until the measured width fits within the given limit.
- `return_without_action_2`: Contains only a return instruction, so calling it has no effect.
- `refresh_two_indexed_entries`: Refreshes up to two indexed entries by calling the shared routine twice for each one, first with flag 0 and then with flag 1.
- `initialise_name_entry_state`: Clears seven state fields, sets fixed position constants and copies text entries for the name and label prompts.
- `forward_argument_with_global_buffers`: Passes its argument and a pointer to the global buffer to a routine, then calls another routine with the constant 260 and zero.
- `repeat_routine_fifteen_times`: Calls a no-argument routine fifteen times in a row and returns nothing.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `draw_menu_entry_by_index`: Returns its input unchanged while a busy flag is set.
- `clear_selection_state_fields`: When a display flag is set it issues drawing calls with stored values and the constant 255, then zeroes seven state fields.
- `draw_indexed_tile_box`: For indices outside 84 to 87, or when a flag is clear, draws a rectangle at coordinates taken from an index table and calls a routine with the index.
- `enable_mode_and_refresh`: Calls the shared routine with the index pairs (2, 0) and (1, 1), sets a flag to 1, and then calls a routine with a stored value.
- `disable_mode_and_refresh`: Calls the shared routine with the index pairs (1, 0) and (2, 1), clears a flag to 0, and then calls a routine with no arguments.
- `enable_alternate_pair_flag`: Calls the shared routine with the index pairs (5, 0) and (4, 1) and sets a second flag to 1.
- `disable_alternate_pair_flag`: Calls the shared routine with the index pairs (4, 0) and (5, 1) and clears a second flag to 0.
- `call_fixed_index_pair`: Calls a routine with the constant 258 and zero and returns nothing.
- `compare_two_reads_and_report`: Reads two values from a source with a sequence of calls and compares them.
- `report_code_four_and_fail`: Passes the code 4 to a routine and returns zero.
- `set_peek_flag_and_draw`: Sets a state flag to 1 and then issues three calls that draw the Peek label and its surrounding area.
- `check_remove_request_and_draw`: Reads a text value into a 256-byte buffer and, when the read reports a nonzero result, issues two drawing calls.
- `record_tile_move_count`: When the state is 4, advances the active player's count and resets the turn values.
- `clear_state_flag_then_call`: Clears a state flag to 0 and calls a no-argument routine.
- `draw_four_indexed_slot_boxes`: Draws a fixed frame at coordinates from global values, then loops over the indices 84 to 87 drawing each entry's rectangle and issuing an index call.
- `update_stage_then_wait`: Draws the status line and then, depending on the peek state and the stage value, sets the stage to 10, 6 or 5 and redraws the matching element.
- `draw_tile_pair_and_advance`: Looks up the current tile record and draws the matching tile values from the tile table, choosing entries by whether the record matches a stored value.
- `redraw_matched_tile_pair`: Looks up the current tile record and draws two tile table entries, choosing their order by whether the record matches a stored value.
- `redraw_tile_pair_entries`: Draws four tile table entries for the current tile in alternating pairs, then steps the tile counter up by two, issues a special call when the counter reaches 144, and draws the final entries.
- `enable_flag_store_constant`: Sets a state flag to 1, stores the constant 88 in another state field, and calls a no-argument routine.
- `link_tile_slot_and_mark`: Follows the stored link for the given slot index when one exists, marks the target entry with 1, and increments the slot's counter.
- `link_tile_slot_from_frame`: Follows the stored link for the slot index kept in the frame, marks the target entry with 1, and increments the slot's counter.
- `check_tile_selectable_index`: Returns 0 when the tile entry is empty, when one of the indices 84 to 87 is blocked by a flag, or when its linked entries are occupied, with a separate rule for index 2.
- `check_tile_selectable_from_frame`: Reads the index from the frame and applies the blocked-index, index-2 and linked-entry rules without the empty-entry check.
- `check_tile_free_from_frame`: Reads the index from the frame and applies the index-2 rule and the linked-entry rules only.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a6, then returns.
- `check_tile_neighbours_free`: Returns 0 when both linked entries for the index are occupied.
- `check_linked_tile_empty`: Returns 1 for indices below 89.
- `return_constant_one`: Returns the value 1 and performs no other work.
- `return_constant_one_3`: This routine returns the integer value one to the caller.
- `draw_remaining_tiles_label`: Formats the tile count held in a global into a text buffer and draws it at a fixed position, with calls that set the pen and position around it.
- `draw_stage_layout_by_index`: Maps its argument minus one to one of about twenty cases, and each case issues a fixed series of drawing calls with small code arguments.
- `draw_status_row_with_flags`: Selects a pen from the first argument when the mode argument is 0 or 1, then walks up to four entries and draws each one whose flag is 0 or 1 at a position built from its index.
- `check_stage_in_range`: Returns 1 when the stage value is between 1 and 4 inclusive, and 0 otherwise.
- `reset_stage_for_redraw`: When the stage value is 6 it sets the stage to 2, and when it is 5 it sets the stage to 1.
- `draw_default_layout_sequence`: Issues one fixed drawing call with a set of small codes that draws the default layout.
- `set_stage_two_and_draw`: Sets the stage value to 2, issues a call with the codes 2 and 2, and draws one of two fixed layouts depending on a stored flag.
- `call_argument_list`: Calls another routine passing a specific sequence of eleven integer arguments.
- `pop_list_head_for_index`: Clears the link of the current head entry for the index, moves the head to the next entry and decrements the entry count, then calls a routine with the index and the new head.
- `draw_board_cell_with_neighbours`: Computes a cell's row and column from its index on a 15-column grid, chooses label offsets by edge position, and draws the cell's frame.
- `await_input_choice_setup`: Stores its argument in a record, prepares a drawing call, and loops reading input values until one of them is 1 or 2.
- `build_text_list_and_wait`: Copies up to 255 bytes of text into a 256-byte buffer, terminates up to four entries with zero bytes, and records each entry's pointer and position.
- `wait_for_choice_value`: Issues a call with the shared buffer, loops reading input until the value is 1 or 2, issues a closing call and returns that value.
- `wait_for_confirm_after_setup`: Stores its argument in a record, sets a display code of 18 in one field and issues a call.
- `poll_messages_until_count`: Repeatedly fetches messages from a port, discarding those of one special type and copying the rest into a 52-byte buffer before releasing them.
- `forward_index_with_globals`: Passes its argument together with two global values and a pointer to a global buffer to a routine and returns nothing.
- `build_display_entry_list`: Walks a linked list of entries for the given slot and fills an array of 11-word display records, one per entry.
- `fill_display_record_from_entry`: Fills a 20-byte display record from an entry by setting a type byte, clearing the link fields, and copying two position words from the entry.
- `find_matching_entry_and_draw`: Searches up to five stored entries for one that matches the record's first value and identifier.
- `set_stage_field_and_call`: Sets two text-layout fields to 63 when the stage value is 3, or to 47 otherwise, copies the second value into a third field, then calls a routine with its argument and the shared buffer.
- `compare_strings_up_to_length`: Compares two strings for up to a given number of characters and returns the difference of the first differing bytes.
- `scan_unquoted_characters_until_match`: Walks a string while skipping quoted sections and backslash-escaped characters, passing each unquoted character to a routine until that routine returns nonzero.
- `return_without_action_3`: Contains only a return instruction and performs no work.
- `compare_strings_to_terminator`: Compares two zero-terminated strings byte by byte and returns the difference at the first mismatch, or zero when they are identical.
- `invoke_library_vector_30`: Calls the library function at a fixed vector offset through the library base stored in its argument record.
- `initialise_tile_table_values`: Resets seven state fields and sets the tile count to 144.
- `return_absolute_value`: Returns the absolute value of its 16-bit argument.
- `sum_two_call_results`: Calls two routines without arguments and returns the sum of their results.
- `shuffle_tile_index_table`: Fills a 144-entry index table with the values 1 to 144, then shuffles it by swapping each position with an earlier one chosen from a routine's result.
- `reset_count_and_call_pair`: Sets a count global to 144 and then calls two routines, one with no arguments and one with a pointer to the global buffer.
- `draw_table_entry_and_byte`: Passes the table entry at the first index to a routine twice, then passes the byte from the global buffer at the second index to another routine.
- `draw_status_panel_frame`: Sets the pen and draws a framed box using offsets from two global positions, then draws a value at a position derived from a stored index.
- `redraw_tile_board_grid`: Returns early when the tile count is zero.
- `draw_positive_entry_frame`: Checks if a table entry's count is positive, then draws a frame and associated values using the entry's position and coordinates.
- `find_tile_under_cursor`: Converts a point into a grid cell by dividing its coordinates by 21 and 24 and clamping the result to the board.
- `update_selected_tile_value`: When a flag is set, tests a fixed special entry with a hit test and selects it if it matches.
- `refresh_selected_tile_value`: Stores in the selected-value field the table entry at the currently selected tile index.
- `handle_tile_selection_state`: Runs a small state machine for choosing tiles: in state 0 it selects the current tile, in state 1 it either completes a matching pair or records a second selection, and in state 2 it resets selections depending on whether the current tile matches the saved ones.
- `apply_tile_selection_transition`: Applies the same selection transitions as its sibling, but without the initial argument call and the zero-entry test.
- `apply_tile_selection_without_guard`: Applies the selection transitions for the current tile after its hit check, without the guard that ignores tiles already matched.
- `advance_tile_selection_state`: Runs the selection state transitions for the current tile without any hit test or entry check.
- `handle_second_selection_state`: Handles the case where one tile is already selected: when the new tile equals the first one it clears the selection, otherwise it records a second selection and checks the pair.
- `record_second_tile_selection`: Advances the selection state and records the current tile as the second selection.
- `show_pair_check_feedback`: Issues a drawing call for the current state, a call with the code 13, and a call with the global buffer, then sets a flag to 1.
- `resolve_pending_second_selection`: When two selections are pending, it keeps one of them depending on whether the current tile matches a saved value, updating the state and saved values before issuing drawing calls.
- `release_second_selection_state`: If the current tile equals the saved second selection, it steps the state down and clears that selection.
- `toggle_tile_selection`: Ignores the tile when it is empty, or when it is a special index while a flag is set, or when a different tile is already selected.
- `toggle_tile_selection_unchecked`: Applies the same select-or-clear toggle as its sibling, skipping the initial argument call and the empty-entry test.
- `toggle_selection_without_range`: Applies the select-or-clear toggle without the special-index test, but still refuses a different tile while one is selected.
- `toggle_selection_state_only`: Selects the current tile when no selection is active, or clears the selection when one is active, issuing drawing calls for the change.
- `cancel_pending_pick`: Resets three global variables to zero and calls several routines, including one with a global reference and a fixed value.
- `remove_selected_tiles_and_reset`: Redraws the saved tile or its special entry when the saved value differs from the reference, decrements the tile counter, updates the counter display, and resets the selection state fields to zero.
- `remove_selected_tile_and_reset`: Redraws the saved tile or its special entry, decrements the tile counter, updates the counter display, and resets the selection state fields.
- `reset_selection_display_state`: Issues two drawing calls with the codes 14 and 0 and zeroes five selection state fields.
- `check_tile_pick_allowed`: Returns 1 when the tile index may be picked.
- `check_tile_pick_from_frame`: Reads the index from the frame and returns 1 when the tile may be picked.
- `check_special_tile_blocked`: Returns 0 and issues feedback code 5 when the frame's index is one of the four special indices while a flag is set.
- `return_constant_one_2`: Returns the integer value one to the caller.
- `run_input_result_loop`: Draws its opening elements, then loops reading input values: a value of 2 ends the loop returning zero, a pending flag returns a value from a further call, and other values trigger feedback calls.
- `poll_input_result_loop`: Repeats an input-handling loop: when a flag is set it returns a value from a call, otherwise it dispatches feedback calls according to the value read.
- `read_input_until_result`: Reads input values in a loop, issuing feedback calls for each value, and leaves when the read returns 2 or a flag is set.
- `poll_selection_result_loop`: Issues a feedback call depending on a stored selection, then repeatedly reads input until a result is reached.
- `process_input_result_loop`: Runs an input loop that starts from a different entry point, issuing feedback calls for each value read.
- `finish_selection_dispatch`: Handles the final step after input: when the selection code is 1 and a call succeeds, it stores the stage value and forwards the call's result.
- `restore_stage_or_close_display`: Calls a routine and, when it returns nonzero, restores the saved stage value and returns.
- `restore_stage_and_close_display`: Restores the saved stage value and issues two drawing calls, followed by a closing call.
- `check_named_file_in_directory`: Reads the current directory into an 854-byte buffer and passes it with a file name to a routine that returns a status.
- `run_setup_panel_until_choice`: Draws a setup panel at fixed sizes, then reads input values until one of 1 or 2 is read.
- `confirm_replace_and_set_limit`: Checks for an existing record and, when the prompt to replace it is shown, loops reading choices from 3 to 6 and redraws the entries for each choice.
- `run_replace_prompt_set_limit`: Performs the same choice loop and time-limit mapping as its sibling, reading the choice from the frame.
- `poll_choice_and_set_limit`: Repeats the choice loop until the read returns 1 or 2, redrawing entries for each choice from 3 to 6.
- `draw_and_check_result_call`: Draws two fixed elements and makes a call that yields a result.
- `return_zero_result`: Returns zero with no other work.
- `return_integer_one`: returns the integer value one to the caller.
- `draw_time_left_label`: Draws the time-left text at a position offset from the stored display coordinates.
- `nop_padding_routine`: This routine is empty and performs no operations.
- `build_tournament_result_entries`: Builds display records for the stored tournament results, one per entry, copying each entry's name and score text and positioning it from the bottom of the list.
- `clear_tournament_results`: Calls a routine with the shared buffer and then resets the stored result count to zero.
- `show_quit_tournament_panel`: Sets the stage value to 3, sets a state flag to 1, copies the time limit into a counter, and records the result of a timing call.
- `tick_tournament_countdown`: When at least 256 time units have passed, it decrements the countdown and redraws its display.
- `show_tournament_result_screen`: Draws the tournament result screen with lines chosen by the player's placement and score, then waits in a loop until the input routine returns 1.
- `insert_sorted_table_entry`: Searches a sorted table of keys for a computed value and reports through the pointer argument whether it was already present.
- `shift_global_value_left`: Reads a global value and returns it shifted left by three bits, which multiplies it by eight.
- `pass_value_twice`: Takes one 32-bit argument, passes it to another routine as both the first and second argument, and returns.
- `allocate_from_free_list`: Requests a block size and searches a global linked list of free blocks for the first block large enough.
- `update_linked_list_node`: Calls another routine passing a pointer and a duplicate of a second parameter to update a linked list node.
- `return_block_to_free_list`: Returns a block of a given size to a sorted free list kept in a global variable.
- `format_scaled_decimal_text`: Converts a 64-bit value into decimal digit characters in a text buffer by repeatedly shifting it and tracking a decimal exponent.
- `format_padded_decimal_text`: Converts a 64-bit value into decimal digit characters using the same shifting loop as the neighboring formatting routines.
- `emit_decimal_digits_by_doubling`: Writes decimal digit characters into a text buffer by repeatedly doubling a 64-bit value and counting the shifts that carry into a decimal exponent.
- `emit_decimal_digits_stepped`: Writes decimal digit characters by shifting a 64-bit value and adjusting a decimal exponent in fixed steps before each digit.
- `fill_decimal_digit_buffer`: Fills the text buffer with zero characters from a starting index up to a fixed limit, shifting a 64-bit value between writes.
- `write_decimal_digit_characters`: Writes characters built from a running digit value and the zero character into a text buffer, advancing the index until it reaches the limit.
- `spin_until_counter_zero`: Decrements a 16-bit counter in a loop until it reaches zero, then returns.
- `multiply_long_by_ten`: Multiplies a 64-bit value held as two 32-bit halves by ten and returns the 64-bit product.
- `shift_binary_into_decimal_digits`: Runs a fixed 64-step loop that shifts a 64-bit value left one bit at a time while tracking a running decimal digit.
- `report_signal_and_indices`: Tests a signal mask returned by a library call and returns the mask unchanged when it is clear.
- `setup_two_global_buffers`: Calls the library routine twice, first for the global buffer with a size of 12 and then for a second global buffer with a size of 2,560.
- `draw_cell_outline_segments`: Draws the outline of one board cell by issuing a sequence of line and rectangle primitives at positions derived from the cell coordinates and a cell type index.
- `clamp_coordinates_to_bounds`: Clamps two coordinates to be nonnegative and to lie within the width and height stored in a referenced structure, then passes the clamped pair to another routine.
- `clamp_coordinates_before_library_call`: Clamps two coordinates to be nonnegative and within the width and height stored in a referenced structure, then passes them to the library routine at a fixed vector offset.
- `expand_percent_format_string`: Copies a format string into an output buffer, writing a single percent sign for each doubled percent escape.
- `forward_to_single_routine`: Calls one other routine with no arguments and returns.
- `dispatch_by_zero_check`: Tests a 64-bit value and, when it is zero, calls another routine once with the argument 3.
- `call_five_times_if_nonzero`: If a value is nonzero when its sign bit is ignored, or a second high value is nonzero, calls another routine five times in succession.
- `divide_pair_by_shift_loop`: Runs a fixed 32-iteration restoring division loop over a register pair, subtracting a divisor pair from a working remainder and shifting quotient bits in.
- `call_with_conditional_busy_wait`: When two values are both nonzero, calls another routine twice, runs a 16-bit countdown busy-wait if a signed overflow test on a low word holds, and then calls another routine once.
- `call_pair_with_busy_wait`: Under the same two-value nonzero test as its neighbor, calls another routine twice, runs a 16-bit countdown when the signed overflow test holds, then calls another routine once more.
- `call_twice_with_busy_wait`: Calls another routine twice, runs a 16-bit countdown busy-wait when a signed overflow test on a low word holds, then calls another routine once.
- `delay_by_counter_then_call`: Counts a 16-bit register down from a starting value to zero in a busy loop, then calls another routine once.
- `countdown_if_small_then_call`: When a flag is clear and a 16-bit value is below 64, counts that value down to zero in a busy loop.
- `run_countdown_then_call`: Counts a 16-bit value down through a busy loop started one below it, then calls another routine once.
- `self_invoke_and_return`: The routine calls another routine and then returns.
- `shift_value_with_saturation`: Shifts a 32-bit value using a count derived from a second value.
- `scale_value_with_saturation`: Scales a value by a shift count taken from a second register.
- `shift_right_with_sign_flip`: Shifts a value right by the negated low six bits of a count and negates the result when the count is negative.
- `invoke_library_routine_once`: Calls one routine through a library vector loaded from a global base address, with no arguments, and returns nothing.
- `find_character_in_string`: Scans a null-terminated string for a given character and returns a pointer to the first match.
- `forward_two_arguments_once`: Passes its two arguments unchanged to another routine and returns nothing.
- `divide_by_restoring_shift_loop`: Divides one unsigned 32-bit value by another with a 32-iteration shift-and-subtract loop, returning zero when either operand is zero.
- `initialize_state_and_loop`: Makes several library calls and sets up global state, then repeatedly reads a selector value from another routine.
- `loop_until_sentinel_value`: Repeatedly reads a value from another routine.
- `draw_three_offset_items`: Calls another routine three times with fixed first coordinates of 52, 92 and 132 and a fixed second coordinate of 84.
- `draw_rect_and_indexed_marker`: Makes an initialization call, draws a rectangle with fixed offsets from the given position, and then draws a table entry selected by the index argument at a position relative to a stored point.
- `update_display_bounds_and_state`: Calculates four coordinate values from a table and writes them to a record, then executes a sequence of drawing and state-update calls based on a provided integer parameter.
- `return_from_subroutine_4`: Performs a subroutine return to the caller.
- `forward_values_with_global_base`: Forwards two 16-bit arguments, widened to 32 bits, to another routine together with a global base value as the first argument, then returns.
- `render_frame_segments_from_table`: Sets up library drawing state, then repeats a frame loop a given number of times.
- `run_animation_frame_loop`: Runs a loop in which, when a stage flag equals one, it draws a zero-terminated coordinate table as connected segments, then draws 19 table-driven entries per pass.
- `draw_polyline_then_entries_loop`: Runs a loop in which, when a stage flag equals one, it draws a zero-terminated coordinate table as connected segments, then draws 19 table-driven entries per pass and advances a counter.
- `run_frame_loop_trailing_polyline`: Runs a loop of frames that draws 19 table-driven entries per pass and, when a stage flag equals one, connected segments from a zero-terminated coordinate table.
- `finish_drawing_frame`: Finishes a drawing sequence with two library calls on a global block, repeats one of them when a stored value is nonzero, makes one more library call and returns.
- `call_routine_with_fixed_arguments`: Calls another routine with a value from a global structure and fixed numbers: dimensions 20 and 5, values 590, 190 and 255, and flags 1 and 0.
- `poll_until_one_returned`: Issues a call with the argument 13, makes a call with a global value, polls another routine repeatedly until it returns one, then makes the global call again.
- `extract_top_bits_with_marker`: Returns zero when the magnitude of a 32-bit value is zero.
- `shift_bits_set_top_bit`: Shifts the bits of a 32-bit input out one at a time over eleven steps, accumulating them into a result, then returns the result with its top bit set.
- `normalize_and_flag_range`: Normalizes a 64-bit magnitude by shifting until its leading bit reaches a fixed position, adjusting an exponent from a second value.
- `pack_normalized_exponent_value`: Shifts a 64-bit value left until its high bit reaches a fixed position, adjusts the exponent and packs the exponent and mantissa into a result.
- `report_code_one_return_zero`: Reports the code one to another routine and returns zero.
- `report_code_two_return_max`: Reports the code two to another routine and returns a value whose exponent field is all ones, the largest exponent value.
- `store_argument_in_global`: Copies its 32-bit argument into a global variable and returns.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_and_call_handler`: Copies a 16-longword record from a fixed offset in a base structure into the caller's buffer when one is given, then calls a routine through a stored code pointer and returns.

### Fault

- `run_fault_command_main`: Program entry for the Fault command.
- `move_stack_data`: Checks if a stack pointer is non-null and performs a sixteen-word copy from a specific offset to the stack before jumping to another routine.

### Install

- `run_install_command_main`: Program entry for the Install command.
- `shift_memory_to_stack`: Copies sixteen words from a source offset to the stack and then calls another routine if the stack pointer is non-zero.

### Stack

- `run_stack_command_main`: Program entry for the Stack command.
- `push_data_words`: Checks a stack pointer and, if non-zero, copies sixteen words from a memory offset to the stack before calling another routine.

### Delete

- `run_delete_command_main`: Program entry for the Delete command.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location offset from a register to a stack address if the stack pointer is not null.

### Ask

- `parse_command_line_arguments`: Program entry for the Ask command.
- `invoke_two_library_routines`: Calls two library routines through function pointers at fixed negative offsets from a base register, passing no arguments, and then returns.
- `open_library_or_clean_up`: Calls a library routine that tries to open a library and returns its result when it is nonzero.
- `forward_dos_read_call`: Calls the DOS library Read routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `forward_dos_write_call`: Calls the DOS library Write routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `forward_dos_input_call`: Calls the DOS library Input routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `forward_dos_output_call`: Calls the DOS library Output routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `call_exit`: Library stub: loads its arguments and the dos library base, calls Exit and returns its result to the caller.
- `prompt_yes_no_and_exit`: Writes a prompt containing a message, then reads a reply and repeats until its first character is y, n, a newline, or the uppercase forms of y and n.
- `count_string_length`: Counts the bytes in a zero-terminated string before the terminating zero and returns the count.

### Type

- `run_type_command_main`: Program entry for the Type command.
- `transfer_data_chunk_2`: Performs a loop to move a block of sixteen bytes from a source pointer to a destination pointer.

### Path

- `run_path_command_main`: Program entry for the Path command.
- `duplicate_memory_segment`: Checks if a pointer is non-zero and then copies a sixteen-byte sequence from a calculated address to the stack.

### RunBack

- `run_command_in_background`: Program entry for the RunBack command.
- `print_missing_item_message`: Advances to the end of a string and passes it to two library routines that output a message.

### Break

- `run_break_command_main`: Program entry for the Break command.
- `copy_memory_and_call_function`: Copies sixteen bytes from a memory location offset from a pointer into a destination address, then executes a function via a pointer.

### Info

- `build_dynamic_dispatch_table`: Iterates through a linked list to find a maximum value and then populates a structured table with calculated entries.
- `copy_memory_block`: Copies sixteen bytes from a source offset to a destination pointer if the destination is not null, then calls another routine.

### Assign

- `run_assign_command_main`: Program entry for the Assign command.
- `transfer_data_chunk`: Performs a loop to move a block of data from a memory location to a stack-based destination before executing a function pointer.

### EndCLI

- `run_endcli_command_main`: Program entry for the EndCLI command.
- `execute_callback_after_copy`: Copies a sequence of words from a table to a destination and then jumps to a stored code address.

### Rename

- `rebuild_filesystem_directory_structure`: Traverses a linked list of records to determine a maximum value, then allocates and initializes a new table of entries.
- `copy_data_block`: Copies sixteen words from a source offset to a destination pointer if the destination is not null, then calls another routine.

### NewCLI

- `run_newcli_command_main`: Program entry for the NewCLI command.
- `transfer_memory_chunk`: Checks if a pointer is non-zero before performing a loop to move a block of data from a table to a stack location.

### Echo

- `run_echo_command_main`: Program entry for the Echo command.
- `duplicate_data_sequence`: Iterates through a specific number of words to copy data from a memory offset to a destination, followed by a function call.

### Quit

- `run_quit_command_main`: Program entry for the Quit command.
- `move_word_array`: Performs a block copy of sixteen words from a calculated memory address to a destination if the destination is valid.

### Execute

- `populate_dynamic_dispatch_table`: Calculates the maximum value from a linked list of records and populates a table with incrementing values.
- `copy_data_block_2`: If a stack pointer is non-zero, it copies sixteen words from a source offset to the stack before calling another routine.

### Copy

- `run_copy_command_main`: Program entry for the Copy command.
- `transfer_memory_block`: Copies a block of sixteen words from a memory location to the stack if the stack pointer is valid, then executes another routine.

### List

- `run_list_command_main`: Program entry for the List command.
- `populate_stack_words`: Transfers sixteen words from a memory offset to the stack and then calls another routine if the stack pointer is not zero.

### MakeDir

- `run_makedir_command_main`: Program entry for the MakeDir command.
- `write_block_to_stack`: Copies sixteen words from a source address to the stack and then executes another routine if the stack pointer is non-zero.

### Date

- `run_date_command_main`: Program entry for the Date command.
- `duplicate_data_sequence_2`: Performs a sixteen-word copy from a source offset to the stack and then calls another routine if the stack pointer is non-zero.

### LoadWb

- `start_tool_from_arguments`: Program startup for the LoadWb command.
- `finish_with_cleanup_result`: Stores its argument as the result, calls a library routine, and when a stored flag is set calls two more library routines to close a library and reply to a message.
- `invoke_two_indirect_pointers`: The routine retrieves two function pointers from fixed negative offsets relative to a register and executes them sequentially.
- `execute_library_cleanup`: Stores an argument, calls a library routine, and conditionally calls two more library routines to close a library.
- `invoke_dos_write`: The routine invokes a function pointer located at a fixed negative offset from a global variable.
- `execute_indirect_function`: Fetches a function pointer from a global variable and executes it.
- `execute_indirect_function_2`: Fetches a function pointer from a global table using an offset and executes it.
- `call_dup_lock`: Library stub: loads its arguments and the dos library base, calls DupLock and returns its result to the caller.
- `forward_dos_ioerr_call`: Calls the DOS library IoErr routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `start_workbench_tool_process`: Starts a Workbench tool process.
- `compare_strings_ordered`: Compares two zero-terminated strings character by character and returns minus one, one or zero for less, greater or equal.
- `duplicate_locks_into_node_list`: Walks a linked list of entries and, for each, allocates a cleared eight-byte node, duplicates a file lock stored in the entry and links the nodes in order.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_get_msg`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `pass_global_and_stack_slot`: Passes a parameter, a global value and the address of an eight-byte local slot to another routine, then returns.
- `pass_buffer_length_if_nonempty`: Calls a library routine, measures the length of a 140-byte stack buffer up to its terminator, and passes the buffer and that length to another routine when the length is nonzero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Protect

- `run_protect_command_main`: Program entry for the Protect command.
- `load_stack_values`: If the stack pointer is not zero, it copies sixteen words from a memory offset to the stack and calls another routine.

### SetClock

- `startup`: Program startup for the SetClock command.
- `_exit`: Stores its argument as the exit code, calls a library routine, and when a message is pending calls two more library routines to close a library and reply to it.
- `waitmsg`: Invokes two function pointers located at fixed negative offsets from a register and then returns.
- `openDOS`: The routine attempts to open the DOS library and, if unsuccessful, performs a series of operations involving a global variable and stack manipulation before returning a value from the stack.
- `_Write`: The routine invokes a function pointer located at a fixed negative offset from the DOS base address.
- `_Input`: Fetches a function pointer from a location relative to a DOS base pointer and executes it.
- `_Output`: Fetches a function pointer from a location within the DOS library base and executes it.
- `_Delay`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `_Execute`: Library stub: loads its arguments and the dos library base, calls Execute and returns its result to the caller.
- `probe_clock_chip_and_dispatch`: Probes two clock chip addresses.
- `print_usage_message`: Prints three lines of usage text describing the load and save options by calling the message routine three times.
- `wait_for_clock_tick_change`: Reads the low nibble of a clock register, waits a fixed delay, and returns whether that nibble has changed since the first read.
- `send_timer_command_read`: Sends a timer device request with command code ten, waits for it to complete, and returns the value stored in the request.
- `send_timer_command_set`: Stores a time value in a timer device request with command code eleven, sends it and waits for completion.
- `open_timer_device_request`: Creates a message port, an extended I/O request and opens the timer device with them.
- `close_timer_device_request`: If given a request block, deletes its reply port when one exists, closes the device, and deletes the request block.
- `convert_clock_fields_to_seconds`: Reads the clock's decoded date and time fields, adds leap-year and per-month day counts from table lookups, and returns the total as a seconds count.
- `split_seconds_into_clock_fields`: Splits a seconds count into year, month, day, hour, minute and second parts using repeated division against month and year lengths, then stores each part into a clock register array.
- `compare_strings_case_insensitive`: Compares two zero-terminated strings after folding each character to lowercase, and returns minus one, zero or one according to the first difference.
- `compare_character_with_byte`: Compares a character held in a register with the byte at an address and returns minus one, one or zero.
- `compare_character_below_or_above`: Compares a character held in a register with the byte at an address and returns minus one when the register character is smaller, otherwise one.
- `return_one`: Returns the value one with no other operations.
- `fold_char_to_lowercase`: Converts an uppercase letter to lowercase and returns the character as a 32-bit value.
- `sign_extend_char_value`: Returns the low byte of its input register sign-extended to a 32-bit value.
- `write_clock_test_patterns`: Writes three fixed 16-bit patterns to a clock register pair in sequence and returns nothing.
- `set_mask_and_poll_status`: Sets a fixed bit mask in a clock register and polls a status bit for up to about 200 iterations, returning as soon as that bit clears.
- `clear_clock_control_bits`: Clears two fixed bits in a clock register's control word and returns.
- `_L3`: Prints one argument through the formatted-output routine using a fixed format string on the standard output stream.
- `_AllocMem`: Calls the exec library memory allocation routine through the system base in a global and returns its result.
- `_FreeMem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `_FindTask`: Calls the exec library task lookup routine through the system base in a global and returns the task pointer.
- `_AllocSignal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `_FreeSignal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `_AddPort`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `_RemPort`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `_OpenDevice`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `_CloseDevice`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `_DoIO`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `POSDIV`: Divides a 32-bit value by a 16-bit divisor.
- `ulmult`: Multiplies two 32-bit values using 16-bit partial products and returns the combined 64-bit result.
- `ulmodt`: Calls the 32-bit division routine and returns, leaving its results in the registers it sets.
- `uldivt`: Calls the division routine and returns the quotient value it leaves in a register.
- `lmodt`: Calls the division routine and returns its first result, negated when a second result flag is nonzero.
- `ldivt`: Calls the division routine and returns its quotient, negated when a sign flag is nonzero.
- `_CreatePort`: Allocates a signal bit and a cleared memory block for a message port, fills in its name, type and owning task, and links it into the port list or initializes an empty list.
- `_DeletePort`: Removes a linked port from the port list, marks the port deleted, frees its signal bit and releases its memory block.
- `_CreateExtIO`: Allocates a cleared memory block of a given size for an I/O request, marks its type, stores its size and attaches it to a port.
- `_DeleteExtIO`: If the provided pointer is non-zero, this routine overwrites specific fields within the structure with constant values and then releases the memory block using the size stored in the structure.
- `_fprintf`: Formats a message from a format string and an argument through the formatted-output routine, passing the stream and a stack argument, and returns.
- `__doprnt`: Measures a 140-byte formatted text buffer up to its terminating zero and writes that many bytes through the write routine when the length is nonzero.
- `stuffChar`: Stores the low byte of the data register at the address held in the pointer register and returns.
- `_NewList`: Initializes an empty list header by pointing its head and tail at the header itself and clearing its count.

### Wait

- `parse_wait_command`: Parses a wait command string to extract time durations such as seconds or minutes and an optional target time.
- `add_result_to_pointed_value`: Calls two routines and adds the value returned by the second call to the value stored at the address in the pointer register.
- `add_and_store_remainder`: Adds a value to the 16-bit value at an address, stores the remainder of dividing that sum by a 16-bit divisor there, and returns the quotient.
- `add_error_to_count`: Calls two library routines that report wait-command errors such as missing arguments, bad numbers, low memory or a wrong time format, then adds the value returned by a further routine to a stored count.

### DefDisk

- `load_executable_hunks`: Walks a sequence of section headers and allocates memory for each one, comparing the section type codes with executable hunk type values to decide how to load or relocate the data.
- `stub_empty_return_routine`: This routine is an empty stub that performs no visible behavior and simply returns.
- `spin_until_counter_zero_2`: Decrements a register counter in a loop until it reaches zero, then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 240 into d0.
- `decode_run_length_pixels`: Decodes a run-length encoded byte stream into a destination buffer and updates a color register while doing so.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `cycle_characters_downward`: Repeatedly decrements a character value, wrapping it to a space character when it passes zero, for a count taken from a register, then returns.
- `empty_routine`: This routine is empty and contains no executable instructions.

### Shanghai (1986)(Activision)[v SystemZ v4.0][a].adf:CLI

- `start_new_cli_window`: Program entry for the CLI command.
- `invoke_dual_function_pointers`: Invokes two function pointers stored at fixed negative offsets from a register.
- `execute_dos_command`: Calls a function from a library and, if successful, performs several subsequent library calls using values from a global variable and stack-based parameters.
- `forward_dos_open_call`: Calls the DOS library Open routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `forward_dos_close_call`: Calls the DOS library Close routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `call_dos_input_handler`: This routine calls a function pointer located at a fixed negative offset from a global variable.
- `execute_indirect_function_3`: Fetches a function pointer from a location offset from a global variable and executes it.
- `forward_dos_lock_call`: Calls the DOS library Lock routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `forward_dos_unlock_call`: Calls the DOS library UnLock routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `forward_dos_duplock_call`: Calls the DOS library DupLock routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `forward_dos_currentdir_call`: Calls the DOS library CurrentDir routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `forward_dos_execute_call`: Calls the DOS library Execute routine through a library base stored in a global, using the arguments the caller has placed in registers.
- `create_cli_console_window`: Builds the console window description and the system path strings for a new shell, allocates memory for them, creates the shell window through library calls and releases the temporary memory before returning.
- `build_allocated_node_list`: Walks a linked list of entries and, for each, allocates a cleared eight-byte node and stores the entry's value in it, linking the nodes in order.
- `free_linked_allocations`: Walks a linked list and frees each node with the memory-free routine, passing the size stored for that node, then returns.
- `execute_indirect_function_4`: Fetches a function pointer from a location within a global data structure and executes it.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `execute_indirect_function_5`: Fetches a function pointer from a location offset from a global variable and executes it.

### 12

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_stub`: Returns zero with no other operations.

### 16

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.

### 9

- `return_zero_value`: This routine returns a value of zero.
- `execute_font_data_routine`: This routine executes a block of code containing seventy instructions.

### 8

- `return_zero_2`: This routine performs no operations and returns zero to the caller.

### arp.library

- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 39 into d0, then returns.
- `initialize_library_base`: In shared library code, stores an argument into a library base, allocates up to a few memory blocks through the exec library, sets an empty list header inside the base and finishes through library calls.
- `initialize_library_list_setup`: In shared library code, sets an empty list header in a library base, then allocates and links further entries through library calls and returns when no entry is available.
- `set_arp_record_on_success`: Stores an argument plus a fixed offset into a record field, then makes four library calls.
- `wait_and_increment_arp_counter`: Calls a routine and a library vector in a loop until the zero flag set by the vector is clear, then adds one to the 16-bit counter at offset 32 of the record.
- `clear_field_and_call_vector`: Zeroes the 32-bit field at offset 8 of the structure passed in one address register, then calls a library routine through a pointer read from the base structure at offset 56.
- `allocate_block_and_store_link`: If the input value is non-zero, it calls the exec memory allocation vector through the library base; when that returns a non-null block, it stores the input plus four in the block's first longword.

### icon.library

- `return_zero_stub_2`: Returns zero immediately with no other work.
- `increment_counter_clear_bit`: Adds one to the 16-bit field at offset 32 of the structure pointed to by the base, then clears bit 3 of the flag byte at offset 14 of the same structure.
- `decrement_refcount_maybe_release`: Decrements the 16-bit counter at offset 32.
- `unlink_node_or_set_flag`: If the short at offset 8 is zero, it calls a library vector with a value from offset 42 and removes the structure from a doubly linked list by joining its neighbours.
- `set_flag_bit_return_zero`: Sets bit 3 of the flag byte at offset 14 of the base structure and returns zero.
- `return_zero_3`: This library routine returns zero.
- `open_dos_library_or_alert`: Stores the argument and the base pointer in the structure, calls a library vector at offset minus 552 with the dos.library name referenced in the routine, and stores the result at offset 38.
- `copy_two_longs_to_globals`: Copies two 32-bit values from offsets 34 and 38 of the structure in the address register into two global variables.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `forward_to_handler_routine`: Calls another routine unconditionally and returns, with no other work.
- `forward_to_handler_routine_2`: Calls another routine unconditionally and returns, with no other work.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `allocate_icon_buffer`: This library routine calls a helper routine to allocate a 260-byte memory block and then returns.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `call_library_handler_2`: This library routine calls another routine with an incremented second argument and then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `forward_to_handler_routine_3`: Calls another routine unconditionally and returns, with no other work.
- `call_library_handler_3`: This library routine simply calls another routine and then returns.
- `call_library_handler_4`: This library routine calls another routine with an offset to the second argument and then returns.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_other_routine_2`: This library routine calls another routine and then returns.
- `call_other_routine_3`: The routine calls another routine and then returns without performing any other operations.
- `update_returned_record_field`: Calls a library vector through a pointer taken from the first argument, passing that argument along.
- `chain_setup_with_cleanup`: Calls one routine and, if it returns a non-zero value, passes that value to a second routine together with the arguments.
- `fill_record_from_stack_buffer`: Gets a record pointer through a library call, then calls a second routine to fill a stack buffer.
- `copy_stack_fields_to_record`: Copies 44 bytes from a stack area into the record at offset 96, then copies several longword and byte fields into fixed record offsets.
- `return_null_pointer`: This library routine is empty and immediately returns zero.
- `copy_record_data_to_stack`: Writes a fixed two-word header into a stack buffer, copies 44 bytes from the record at offset 96 and several record fields into stack slots, then calls another routine with the first two arguments and a pointer to the header.
- `validate_icon_file_header`: Reads the icon data through library calls and checks that the first word matches the header constant written by the copy routine and that the second word is one.
- `read_icon_header_fields`: Repeats the header check and reads two values from the input into the record at offsets 18 and 22.
- `read_icon_optional_blocks`: Reads two values into the record at offsets 18 and 22, then conditionally reads further blocks guarded by the record fields at offsets 50, 54 and 70.
- `read_icon_long_array_entries`: Loops over a longword array, reading one value per entry through a library call and storing it, and decrements a byte counter by four per entry until it reaches four or less.
- `call_routine_return_zero`: Calls another routine and then returns zero, with no other work.
- `return_empty_stub_3`: This library routine is empty and returns without performing any operations.
- `allocate_icon_buffer_and_read`: Allocates a 260-byte block through a library call.
- `read_icon_optional_extras`: Saves a longword from the record on the stack and performs a sequence of read calls gated by flag bits and a pointer field at offset 66, returning the outcome of the last call.
- `process_icon_pointer_list`: Calls a routine and two library vectors, then handles the pointer at offset 50 through a library call when it is set.
- `walk_icon_pointer_list`: Advances past a null-terminated pointer list and, if a library call succeeds, loops over the list at offset 54 calling a routine and two library vectors for each entry.
- `process_pointer_list_entries`: Loops over a null-terminated pointer list, calling a routine and two library vectors per entry.
- `drain_icon_entry_list`: Repeatedly calls a routine on the structure at offset 2 from the argument and passes each non-zero result to a second routine, stopping when the first call returns zero.
- `return_void_stub`: Returns immediately without doing anything.
- `forward_with_offset_140`: Calls another routine with the same first argument and the second argument increased by 140, then returns.
- `allocate_icon_record_or_free`: Allocates a 168-byte block through a library call.
- `return_zero_library_stub`: Library code that returns a constant value of zero.
- `return_zero_constant_3`: This library routine returns a constant value of zero to the caller.
- `take_slot_and_store_pair`: Decrements a 16-bit slot counter and, when no slot remains, calls a routine to refill the table.
- `decrement_counter_store_pair`: Decrements a 16-bit counter through a pointer, then stores two 32-bit values into consecutive slots of a table at the new counter position.
- `allocate_icon_node_or_free`: Allocates a 94-byte block through a library call and passes it to two routines.
- `return_zero_12`: An empty routine that performs no operations and returns.
- `call_lib_func_and_return_zero`: Invokes a function pointer retrieved from a global variable associated with library index minus 210.
- `forward_with_offset_78`: Calls another routine with the same first argument and the second argument increased by 78, then returns.
- `find_value_after_equals`: Walks a null-terminated list of entries, calling a routine on each one.
- `return_zero_value_2`: This library routine is empty and simply returns zero to the caller.
- `scan_icon_entries_for_match`: Walks a linked list of entries, comparing each entry's computed size with a value from a routine and calling another routine to confirm matches.
- `verify_icon_entry_chain`: Walks the entries with the same traversal as the neighbouring scan, calling routines to test each size and confirm it.
- `check_icon_entry_sizes`: Traverses entries through a routine, comparing each computed size with an expected value and calling a confirmation routine.
- `search_icon_entry_size`: Walks entries in the same way as its neighbours, comparing a computed size with an expected value and confirming matches through a routine.
- `return_zero_constant_4`: This library routine returns a constant zero value to the caller.
- `parse_copy_count_from_text`: Clears the flag byte at offset 30 and compares the text at offset 5 with a fixed word.
- `parse_copy_count_suffix`: Compares the text at offset 5 with a three-character marker and, when it matches, parses decimal digits and then checks a following four-character marker.
- `parse_decimal_then_check_marker`: Parses a run of decimal digits from a string pointer and then checks a four-character marker that follows.
- `parse_count_then_find_resident`: Parses decimal digits from a string pointer, checks a following four-character marker, and finishes with routine calls.
- `convert_char_to_digit`: Returns the numeric value of a character if it is a decimal digit from zero to nine, and minus one for any other character.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `call_library_vector_198`: Calls the library function at offset minus 198 from the library base stored in a global variable.
- `call_library_vector_210`: Calls the library function at offset minus 210 from the library base stored in a global variable.
- `call_library_vector_228`: Calls the library function at offset minus 228 from the library base stored in a global variable.
- `call_library_vector_246`: Calls the library function at offset minus 246 from the library base stored in a global variable.
- `call_library_vector_264`: Calls the library function at offset minus 264 from the library base stored in a global variable.
- `call_library_vector_30`: Calls the library function at offset minus 30 from the library base stored in a global variable.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Calls the library function at offset minus 42 from the library base stored in a global variable.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `complex_library_traversal`: Library code that calls another routine and uses the result to conditionally call a second routine or perform a cleanup call.
- `allocate_record_set_count`: Allocates a 96-byte block through a library call, sets a count of ten in the record at offset 14 and in the short the caller passed, and calls a routine to link the record.
- `return_zero_stub_5`: This library routine is empty and immediately returns zero.
- `verify_call_result_matches`: Calls a routine with the last three arguments and returns that result only when it equals the fourth argument.
- `return_empty_stub`: This library routine performs no operations and simply returns.
- `verify_call_result_matches_2`: Calls a routine with the last three arguments and returns that result only when it equals the fourth argument.
- `return_empty_stub_2`: This library routine is empty and returns without any action.
- `forward_call_result_unchanged`: Calls another routine with its last two arguments and returns that result unchanged.
- `forward_second_arg_to_handler`: Passes its second argument to another routine, which makes a library call at offset minus 36, and then returns with no value.
- `divide_unsigned_32bit_loop`: Divides a 32-bit dividend by a divisor using a sixteen-step shift-and-subtract loop when the divisor is larger than sixteen bits.
- `restoring_divide_sixteen_steps`: Runs a sixteen-step restoring division loop that shifts a 32-bit value left one bit at a time and subtracts the divisor whenever it fits.
- `multiply_32bit_partial_products`: Multiplies two 32-bit register values using 16-bit partial products and combines the cross terms into the high word of the result.
- `measure_string_length`: Counts the characters in a zero-terminated string and returns the count.
- `find_character_in_string_2`: Scans a zero-terminated string for the given character and returns a pointer to its first occurrence.
- `return_zero_4`: This routine is an empty function that returns zero to the caller.
- `copy_zero_terminated_string`: Copies bytes from a source string to a destination buffer, advancing both pointers, until the zero terminator has been copied.
- `append_string_to_destination`: Moves to the end of the destination string and copies the source string, including its terminator, onto that point.
- `compare_bounded_strings`: Compares two strings for up to the given number of characters.
- `return_one_stub`: Returns one immediately with no other work.
- `walk_bounded_string_to_end`: Walks a character buffer for up to the given count.
- `call_and_return`: This library routine calls another routine and returns.
- `copy_string_with_padding`: Copies a source string into a destination buffer for up to the given count, copying the terminator and then filling the rest of the count with zero bytes.
- `return_from_subroutine_3`: This routine is an empty function that performs no operations before returning.
- `initialize_list_header`: Initializes a list header in the block it is given.
- `process_block_with_check`: Returns one for a null block pointer.
- `walk_block_list_check`: Walks a linked list of blocks through the link at offset 12, calling a routine on each block with a fixed value and then with the pointer at offset 8 and four times the byte at offset 7.
- `return_one_stub_2`: Returns one immediately with no other work.
- `return_zero_stub_6`: This library routine returns zero without performing any operations.
- `allocate_record_and_fill`: Allocates a 20-byte record through a library call and, when a second call succeeds, reads a value through another routine and stores it at offset 10 of the record.
- `return_minus_one_stub`: Returns minus one immediately with no other work.
- `allocate_chain_of_blocks`: Loops once per requested block, allocating a 16-byte block through a library call and linking each new block to the previous one at offset 12.
- `build_linked_block_chain`: Links blocks in a chain by storing each block's pointer at offset 12 and reading a value into offset 8 through routine calls.
- `return_empty_library_stub`: Library code that performs no operations and returns.
- `return_negative_one_stub`: This library routine returns a value of all ones and performs no other operations.
- `call_rawdofmt_vector`: Calls the exec RawDoFmt routine through its library vector at offset minus 522, passing no arguments and returning nothing.
- `store_byte_to_pointer`: Writes the low byte of the data register to the memory address held in the pointer register and returns.

### diskfont.library

- `allocate_and_store_argument`: Calls the library vector at offset minus 84 and, when it returns a non-null block, stores the argument at offset 34 of that block and calls the library vector at offset minus 396.
- `check_and_set_font_handle`: The routine is library code that calls a routine via a table entry using the library base.
- `open_dos_and_graphics_libraries`: Opens the dos.library and graphics.library names through the library-open vector at offset minus 552, storing the two library bases in global variables.
- `unload_unused_segment_entries`: Walks a linked list of segment entries and, for each entry whose short counter at offset 84 is below one, unlinks it from the list and unloads its segment through the dos library.
- `close_library_and_free_node`: When a short value at offset 38 is zero and the counter at offset 40 is below one, closes two libraries, unlinks the node from its list and frees the memory it occupies.
- `set_flag_high_bit`: Sets the most significant bit of the flag byte at offset 14 and returns zero.
- `return_void_stub_2`: Returns immediately without doing anything.
- `clear_flag_bit_increment_counter`: Clears the most significant bit of the flag byte at offset 14 and adds one to the 16-bit field at offset 38.
- `decrement_counter_and_call`: Decrements the 16-bit counter at offset 38.
- `return_zero_stub_3`: Returns zero immediately with no other work.
- `build_font_name_buffer`: Copies names from a linked list of font entries at offset 140 into a caller buffer, storing each entry's length, flag and pointer fields alongside the name.
- `copy_font_entry_names`: Walks a linked list of font entries, copying each entry's name string into a buffer and recording the entry's size and attribute bytes beside it, until the list ends.
- `fill_font_entry_buffer`: Copies characters from a source string into a buffer, then walks a linked list of font entries recording each entry's name position and attributes.
- `scan_fonts_directory_entries`: When the flag bit is set, it allocates a 260-byte buffer and examines the FONTS: directory.
- `read_font_file_header`: Reads a four-byte header from the input and checks it against a fixed signature.
- `read_font_entry_blocks`: Copies a name string into the buffer, reads and checks a header count, then reads 260-byte entry blocks into the output list while the remaining size stays non-negative.
- `read_font_blocks_until_end`: Reads 260-byte blocks from the input until the list is exhausted, storing each as a record and tracking the remaining size.
- `finish_font_read_status`: Releases resources through routine calls, then returns the absolute value of the remaining size counter when it is negative, and zero otherwise.
- `return_zero_constant_5`: The routine immediately returns a zero value without performing any other operations or accessing memory.
- `score_font_size_distance`: Compares the requested size with the size field at offset 256 of a candidate font entry, producing a weighted distance and adding a fixed bias.
- `score_font_style_bits`: Compares the style flag bytes of a request and a candidate font, subtracting a weighted table value for each mismatched bit from a size-derived base.
- `score_style_bit_penalty`: Tests bits from two flag words in turn and subtracts a table value for each mismatch, returning the accumulated penalty.
- `score_style_bits_iterate`: Iterates over up to six style bits, subtracting table weights for each mismatch and returning the running total when the bits are exhausted.
- `return_negative_sentinel`: Returns a fixed large negative sentinel value without any other work.
- `load_named_font_entries`: Copies the requested font name into a local buffer and looks up the matching entry.
- `select_best_font_entry`: Reads a font header and entry block, then loops over the candidate entries, scoring each one through a routine and keeping the highest score.
- `mark_best_font_entry_loaded`: Selects the highest scoring entry and, when it has the expected type, links it into the font list.
- `select_best_font_candidate`: Iterates through a list of font entries to find the candidate with the highest score.
- `pick_font_entry_until_done`: Repeatedly clears a flag in a font entry and selects the best remaining candidate by score.
- `close_font_and_release_entries`: Clears entry flags and selects the best remaining candidate by score.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_allocmem_stub`: Calls the exec AllocMem library routine through the library base held in a global variable, at offset minus 198.
- `call_freemem_stub`: Calls the exec FreeMem library routine through the library base held in a global variable, at offset minus 210.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_dos_open_stub`: Calls the dos Open routine through the dos library base held in a global variable, at offset minus 30.
- `call_dos_close_stub`: Calls the dos Close routine through the dos library base held in a global variable, at offset minus 36.
- `call_dos_read_stub`: Calls the dos Read routine through the dos library base held in a global variable, at offset minus 42.
- `call_dos_lock_stub`: Calls the dos Lock routine through the dos library base held in a global variable, at offset minus 84.
- `call_dos_unlock_stub`: Calls the dos UnLock routine through the dos library base held in a global variable, at offset minus 90.
- `call_dos_examine_stub`: Calls the dos Examine routine through the dos library base held in a global variable, at offset minus 102.
- `call_dos_exnext_stub`: Calls the dos ExNext routine through the dos library base held in a global variable, at offset minus 108.
- `call_dos_currentdir_stub`: Calls the dos CurrentDir routine through the dos library base held in a global variable, at offset minus 126.
- `call_dos_loadseg_stub`: Calls the dos LoadSeg routine through the dos library base held in a global variable, at offset minus 150.
- `call_dos_unloadseg_stub`: Calls the dos UnLoadSeg routine through the dos library base held in a global variable, at offset minus 156.
- `copy_null_terminated_string`: Library code that copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `seek_string_end_then_call`: Advances to the zero terminator of a string and then calls another routine with the end pointer and the second argument.
- `forward_to_handler_routine_4`: Calls another routine unconditionally and returns, with no other work.
- `calculate_string_length`: Library code that iterates through a null-terminated string to calculate its length.
- `return_immediately_3`: This library routine is empty and performs no operations before returning.
- `compare_zero_terminated_strings`: Compares two zero-terminated strings byte by byte.
- `compare_previous_character`: Compares the character before one pointer with the character at another pointer.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_character_in_string_3`: Library code that scans a null-terminated string for a specific byte and returns its address or zero if not found.
- `find_character_forward`: Scans forward from the character after the start pointer for the given byte and returns its address.
- `return_zero_stub_4`: Returns zero immediately with no other work, in diskfont.library.
- `find_last_character_in_string`: Finds the length of the string, then scans backwards from its last character for the given byte.
- `scan_backwards_for_character`: Scans backwards from a pointer, stopping at a lower limit, for the given byte.
- `return_zero_5`: This library routine returns a value of zero to the caller.
- `find_path_separator_or_start`: Looks for a slash character in a string and then a colon.
- `return_void`: The routine returns without performing any operations.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### sound1

- `return_zero_program_entry`: Program entry that returns zero immediately, with no calls or library use.

### sound2

- `return_zero_6`: The routine performs no operations and returns a value of zero to the caller.

### sound3

- `return_zero_7`: The routine performs no operations and returns zero to the caller.

### sound4

- `return_zero_8`: This routine performs no operations and returns a value of zero to the caller.

### sound7

- `return_zero_9`: This routine is empty and returns a zero value to the caller.

### sound8

- `return_zero_10`: The routine returns a zero value to the caller.

### sound9

- `return_zero_11`: An empty routine that returns zero.

### extra

- `stub_no_instructions_decoded`: The decompiler timed out, and the facts show zero instructions and a program entry.

### Shanghai (1986)(Activision)[v SystemZ v4.0][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routine_then_dos_open`: Runs a fixed-address routine first, then calls the library function at offset minus 30 through a library base held in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_30_2`: Calls the library function at offset minus 30 through a library base held in a global variable, then returns.
- `call_library_vector_348`: Calls the library function at offset minus 348 through a library base held in a global variable, then returns.
- `modify_memory_and_increment`: Clears the most significant bit of a byte at a specific offset and increments a short value at another offset within a structure.
