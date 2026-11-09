# Game 290 (galaga 94 (1994)(h. mueller)(de)[cr avantgarde]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00290). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 372 of 372 named (0 author symbols), 372 of 372 described (145 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 192594 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9859 of 9859 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13638 |
| code | 31032 |
| data | 292447 |
| padding | 4387 |
| unused | 559616 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| DESV | 2 | 1837 | 101 |
| Galaga 94 (1994)(H. Mueller)(De)[cr Avantgarde].adf:aCHTUNg | 2 | 214 | 7 |
| DE2-Handler | 2 | 3526 | 92 |
| DE3-Handler | 2 | 2393 | 77 |
| DE1-Handler | 2 | 1114 | 53 |
| xpkNUKE.library | 2 | 765 | 41 |
| Galaga 94 (1994)(H. Mueller)(De)[cr Avantgarde].adf:bootblock | 1 | 10 | 1 |
| Galaga_94 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- galaga 94 (1994)(h. mueller)(de)[cr avantgarde]/Galaga 94 (1994)(H. Mueller)(De)[cr Avantgarde].adf:Galaga_94/Galaga_94: high-entropy data file (7.62 bits/byte), classified as packed data
- galaga 94 (1994)(h. mueller)(de)[cr avantgarde]/Galaga 94 (1994)(H. Mueller)(De)[cr Avantgarde].adf:c/execute: high-entropy data file (7.61 bits/byte), classified as packed data
- galaga 94 (1994)(h. mueller)(de)[cr avantgarde]/Galaga 94 (1994)(H. Mueller)(De)[cr Avantgarde].adf:libs/info.library: high-entropy data file (7.69 bits/byte), classified as packed data
- galaga 94 (1994)(h. mueller)(de)[cr avantgarde]/Galaga 94 (1994)(H. Mueller)(De)[cr Avantgarde].adf:libs/req.library: high-entropy data file (7.65 bits/byte), classified as packed data
- galaga 94 (1994)(h. mueller)(de)[cr avantgarde]/Galaga 94 (1994)(H. Mueller)(De)[cr Avantgarde].adf:Galaga_94/Galaga_94: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- galaga 94 (1994)(h. mueller)(de)[cr avantgarde]/Galaga 94 (1994)(H. Mueller)(De)[cr Avantgarde].adf:aCHTUNg: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 192594 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### DESV

- `allocate_and_init_desv_memory`: Saves registers, allocates a memory block via exec.AllocMem, copies a global table into it, zeroes a second area, stores pointers and calls setup routines, returning zero or minus one.
- `shutdown_desv_and_free_resources`: Calls cleanup routines, walks a linked list freeing entries through a library call, restores the stack pointer, and frees the saved memory block before returning.
- `clear_indexed_byte_in_buffer`: Reads a byte index from a table entry, calls a routine with the buffer and entry pointer, then clears the byte at that index in the buffer and returns the index.
- `setup_frame_and_scan_buffer`: Saves a2-a3 on the stack, loads the first argument from the caller's stack into a2, creates a 32-byte a5 frame, points a3 at a local buffer, then branches into the scanning loop.
- `skip_spaces_and_check_colon`: Advances a2 past the current character, then tests the next bytes for space, terminator or colon and branches accordingly without returning.
- `copy_token_until_space_or_colon`: Copies characters from a2 to a3 until a space, terminator or colon is found, appends a colon and terminator, then calls a routine with the local buffer and returns.
- `find_matching_entry_by_id_or_39`: Walks a linked list of entries, comparing each entry's id field against the requested id or 39, recursing through a nested lookup, and returns the matching entry pointer or zero.
- `check_first_field_and_return_zero`: Saves a2-a3/a6, loads a pointer from the stack argument, tests its first longword field, and returns zero in d0 when that field is nonzero.
- `load_second_list_head_into_a3`: Loads the global structure pointer, follows its list head, scales the index by four into a2 and the next field into a3, then branches onward.
- `match_node_by_offset_eight`: Tests the longword at offset four of the node in a3; if zero, compares the longword at offset eight of a3 with that of a0 and returns a3 in d0 when equal, otherwise branches to the shared exit.
- `advance_to_next_node_or_null`: Loads the longword at a3, shifts it left by two to form a pointer, stores it in a3 and d0, and returns zero in d0 when the new pointer is null, restoring a2-a3/a6 from the stack.
- `find_list_entry_by_key`: Saves a2-a3/a6, walks the linked list from the pointer argument comparing each node's offset fourteen longword against the key, returning the matching node in d0 or zero after unlocking the frame.
- `get_node_field_at_offset_eight`: Saves a2-a3, calls the lookup routine with the argument and zero, and returns the longword at offset eight of the found node, or zero if none was found.
- `open_xpk_nuke_decompressor_stream`: Looks up a node, opens an xpkNUKE decompressor stream, allocates buffers, fills a state block with pointers and flags, then polls the decompressor until it signals completion.
- `decode_data_extern_port_handlers`: Opens the decoded data external port, allocates buffers, reads three handler entries, stores them in a state block, then invokes the decompressor four times and returns zero.
- `call_44a_with_two_zero_args`: Pushes two zero longwords onto the stack, calls the routine at offset 44a relative to the program counter, then discards the eight argument bytes and returns.
- `negate_d5_store_d0_and_nop`: Negates the low word of d5, stores the low word of d0 to the address in a0 and decrements a0 by two, then executes a no-operation.
- `free_two_global_pointers_if_set`: Checks two longword pointers at fixed offsets from a4, and for each nonzero one pushes it and calls the routine at offset 1a88 relative to the program counter, then returns.
- `open_graphics_and_intuition_libraries`: Opens graphics.library and intuition.library via the library opener, stores both bases in a4-relative globals, calls the error routine and returns 0 if either open fails, otherwise returns 1.
- `drain_message_port_up_to_twenty_times`: Loops up to twenty times fetching messages from the port at offset 86 of the base in a4-relative global, calls the message handler on each, returns early when a message type equals 512, and calls the delay routine with 20 when the port is empty.
- `draw_text_with_centered_x_offset`: Copies a character and color into a4-relative text buffer, measures the string, computes a horizontal offset to center it within 640 pixels, then calls the text drawing routine with the caller's arguments and the buffer.
- `init_display_list_entries`: Loops d2 from 0 to 16, and for entry 6 stores a pointer into a4-relative memory, then calls two subroutines with each table entry and an offset value.
- `setup_workbench_display_and_lists`: Checks a version field, opens the Workbench screen, adjusts a display height, creates a list, computes a size value, then calls the display-list initializer and cleanup routines.
- `build_screen_from_descriptor`: Reads a screen descriptor, copies bitmap and palette pointers into a4-relative globals, creates three display lists, and builds a string table from descriptor fields before refreshing the display.
- `copy_d0_to_d5`: Copies the longword in d0 into d5, then executes a no-op before falling through to the following code.
- `unsigned_32bit_multiply_d0_d1`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, returns the low longword in d0, and pops d0-d1 from the stack.
- `signed_32bit_divide_d0_by_d1`: Computes signed 32-bit division of d0 by d1, negating operands as needed, calls the unsigned divide routine, and returns quotient in d0 and remainder in d1.
- `unsigned_32bit_divide_with_remainder`: Divides d0 by d1, returning quotient in d0 and remainder in d1, handling zero divisor, 16-bit divisors and negative remainder correction while preserving d2-d4.
- `count_whitespace_separated_tokens`: Scans the string at the first stack argument, skipping spaces and tabs, counting tokens, treating quoted sections as one token, and writing null terminators at token ends.
- `split_string_into_argument_pointers`: Walks the string at the first stack argument, skipping spaces and tabs, storing a pointer to each token start into the array at the second argument for the given count.
- `alloc_and_link_desv_node`: Reads a size from the stack, allocates that many plus eight bytes via a subroutine, links the new block into a list at offset -32754 of a4 and returns the block plus eight, or zero on failure.
- `copy_bytes_forward`: Copies a count of bytes from the source pointer to the destination pointer using a0 and a1, decrementing d0 each iteration, and returns the original destination pointer.
- `fill_bytes_with_value`: Loads a fill byte from the stack into d1 and branches into a byte-fill loop that writes that value to consecutive memory locations.
- `fill_memory_with_d1_byte`: Reads destination pointer and count from the stack, writes the byte in d1 to each of count bytes, then returns the original pointer in d0.
- `init_stream_struct_and_test_open`: Stores parameters and default fields into a structure pointed to by a2, calls the open routine with the stored pointer, and sets a status bit depending on the result.
- `strlen_return_length_in_d0`: Scans the null-terminated string passed on the stack, advancing a0 until a zero byte, and returns the length in d0.
- `copy_string_return_destination`: Copies bytes from the source string at the second stack argument to the destination at the first, including the terminating null, and returns the destination pointer in d0.
- `bounded_string_copy_with_terminator`: Copies up to the given count of bytes from the source string to the destination, stopping at a null byte, then writes a null terminator if room remains and returns the destination in d0.
- `compare_strings_via_case_table`: Compares two strings byte by byte using a lookup table to map each character, returning minus one, zero or one in d0 depending on the mapped ordering.
- `close_device_node_by_handle`: Looks up a device node from the passed handle, then either calls its close callback with reason 9 or invokes a secondary close routine, returning -1, 0 or 1.
- `close_all_open_devices`: Walks the global device list calling each node's close callback, then closes three fixed device entries and drains a pending-close list before final cleanup.
- `close_single_device_node`: Checks the node's open flag, calls its shutdown routine, frees its buffer, unlinks it from the doubly linked list and frees the node, returning the shutdown result.
- `desv_stop_channel_and_free_buffers`: Compares two channel offsets, calls the seek routine, marks the channel stopped, frees the two buffers, releases the channel and clears flag bits in the channel structure.
- `desv_stop_all_channels_loop`: Loops a counter from zero up to the channel count in a4-relative memory, calling the per-channel stop routine for each index and returning after the last.
- `desv_stop_single_channel_by_index`: Looks up a channel by index, frees its buffer, invokes its callback with command five, then clears the channel fields and returns the callback result.
- `lookup_slot_by_index_check_flag`: Reads a word index from the stack, compares it against a limit at offset -32746 of a4, and returns a pointer to a 16-byte slot at -32750 of a4 if its byte 6 has bit 4 set; otherwise stores 3 at -32318 of a4 and returns zero.
- `dispatch_slot_callback_or_fallback`: Saves d2-d4/a2-a3, calls the slot lookup with the word argument, then invokes the slot's callback at offset 8 with arguments 2, a2 and d3, or falls back to the slot's other routine; returns -1 on failure or 4 stored at -32318 of a4.
- `check_input_flag_and_run_script`: Loads the structure at offset 276 of the pointer at -32766 of a4, tests bit 4 of byte 28, and if set clears a 4096-byte block, calls the routine at -32738 of a4, then runs script 2 with callback and finally script 1.
- `call_installed_exit_hook`: Tests the longword hook pointer at offset -32310 from a4; if non-zero, pushes a longword argument, loads the pointer into a0, calls it, then adds 4 to a7 and returns d0 as zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `install_exit_hook_if_absent`: Saves a2, loads the argument from 8(a7) into a0 and the current hook pointer from -32738(a4) into a1, and if a0 is zero stores the address of the exit-hook routine there.
- `store_hook_pointer_return_old`: Stores a0 into the hook pointer at offset -32738 from a4, moves the previous pointer from a1 into d0, restores a2 from the stack and returns.
- `add_seven_to_d3_and_xor_d0`: Adds seven to the low word of d3 and exclusive-ors the low byte of d0 with the immediate value 113, then falls through to the next routine.
- `unlink_node_from_list_and_free`: Walks the list at a4-relative offset to find the node eight bytes before the given pointer, unlinks it, then calls the free routine with the node and its stored value.
- `allocate_and_initialize_desv_node`: Allocates a 34-byte block, stores a handle byte, sets type 4, saves the caller pointer and a new handle, initializes a sub-structure, and frees the block on failure.
- `release_desv_object_and_notify`: Saves a2, loads the object pointer from the stack, and if non-null calls the sub-object release, initializes byte 14 from byte 15 via a call, sets byte 14 to 2, then calls the notify routine with selector 34.
- `init_desv_list_header`: Loads the object pointer from the stack, points its first longword at its own offset 4, zeroes the longword at offset 4, stores the object pointer at offset 8, and returns zero in d0.
- `install_desv_handler_pointers`: Loads four addresses relative to a4 and pc into fixed a4-relative longword slots, storing pointers to three data areas and one code routine as handler vectors.
- `check_dos_stdin_and_open_console`: Reads the dos.library process structure, and if its input handle is a console with no buffered data, flushes and reopens it, storing the new handle; then opens the console device and returns 1 on failure or 0 on success.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `close_console_handle_if_open`: Loads the saved console file handle from the global variable, and if it is nonzero passes it to the dos.library close call, returning the close result or zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_device_proc`: Library stub: loads its arguments and the dos library base, calls DeviceProc and returns its result to the caller.
- `call_delay`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `call_is_interactive`: Library stub: loads its arguments and the dos library base, calls IsInteractive and returns its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_330_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_510`: Library stub: loads a library base from a global variable and calls the library function at offset minus 510 of that base, returning its result to the caller.
- `call_library_vector_516`: Library stub: loads a library base from a global variable and calls the library function at offset minus 516 of that base, returning its result to the caller.

### Galaga 94 (1994)(H. Mueller)(De)[cr Avantgarde].adf:aCHTUNg

- `relocate_and_run_embedded_program`: Program entry that copies the relocated image, finds the current task, allocates memory for the embedded hunk segments, copies segment data, patches the segment list, and jumps to the loaded program's entry point.
- `run_length_decode_into_memory`: Checks d5 flag, increments d6 counter, then loops reading counts and offsets from a3, calling the offset resolver and adding decoded bytes into memory at a2 until the count reaches zero.
- `resolve_offset_chain_from_stack_frame`: Walks a linked list starting at the word before the a5 frame pointer, following each stored pointer doubled, decrementing d1 until it underflows, and returns the resolved pointer in a1.
- `decode_and_copy_tile_bytes`: Probes two bit flags via helper calls, selects a length from a table indexed by d0, compares d1 against table words, then copies bytes from a0 to a1 until a2 is reached.
- `write_audio_byte_to_custom_register`: Shifts the current byte left, refills it from memory when it becomes zero, writes the byte to the custom audio register, then rotates it with extend and returns.
- `decode_and_copy_audio_stream`: Repeatedly calls the audio byte writer to build d1, reads run-length and offset tables via pc, copies bytes from a3 to a1, and loops until a1 reaches a2.
- `expand_run_length_encoded_bytes`: Reads a marker word and bytes from a1, compares each byte to the marker, expands repeated byte runs into a0, and stops when a0 reaches a2.

### DE2-Handler

- `allocate_and_init_de2_handler_state`: Allocates a memory block via exec.AllocMem, copies a 36-entry table into it, zeroes 201 longs, stores the exec base and stack pointer, then runs setup calls and frees the block on exit.
- `single_nop_padding`: Contains only a nop instruction, so it does nothing and falls through to the following code.
- `copy_string_and_terminate_at_length`: Calls a copy routine with the destination and source-plus-one, then writes a zero byte at the destination offset given by the source's first byte and returns that length.
- `dispatch_de2_by_queue_depth`: Pushes a2, loads the argument pointer, reads the word at offset 20 of the structure pointed to by a4-relative memory, and calls one of two routines depending on whether that word is below 36.
- `allocate_and_link_de2_node`: Allocates a node via a library call, stores it in the caller's structure, links it into a list, initializes fields to zero, calls four setup routines on it, and returns the node pointer.
- `build_de2_entry_from_index`: Computes an index-scaled pointer, calls allocation and lookup routines, compares the result against a table base, and copies 65 longwords into the caller's buffer, returning success or zero.
- `build_and_call_de2_request`: Loads a pointer from a4 offset into a5 locals, calls a routine with value 4, fills a request block with d2 and the result, then calls another routine with a2 offset 12 and the block pointer, returning its result or zero.
- `store_result_and_test_second_value`: Copies the a5 local at offset -32 into a3 offset 148, tests the a5 local at offset -36, and returns zero if it is nonzero, otherwise branches onward.
- `copy_64_longs_and_call_compare`: Copies 65 longwords from the a5 local pointer to the caller's buffer, then calls a routine with both pointers plus 8, sets d0 to 1, restores registers and returns.
- `send_de2_command_87_with_args`: Pushes d2-d3/a2-a3/a6, reads three stack arguments, allocates a local message block with command 87 and the arguments, calls the DE2 send routine, stores the result at offset 148 of the returned structure and returns zero or minus one.
- `send_de2_command_82_with_args`: Pushes d2-d3/a2-a3/a6, reads three stack arguments, builds a local message block with command 82 and the arguments, calls the DE2 send routine, stores the result at offset 148 of the returned structure and returns zero or minus one.
- `send_de2_command_1008_with_args`: Pushes d2-d4/a2-a3, reads three stack arguments, builds a local message block with command 1008 and the arguments, calls the DE2 send routine, stores the result at offset 148 of the returned structure and returns zero or minus one.
- `build_and_send_3ef_message`: Saves registers, allocates a message buffer with type 1007 and a field from the table entry, sends it via the message routine, then frees the entry and returns the result.
- `build_and_send_8_message`: Saves registers, allocates a message buffer with type 8 and three caller-supplied values, sends it, stores a result field into the handler structure and returns the status.
- `send_8_message_and_store_result`: Builds a message buffer with type 8 and the saved d2-d4 values, sends it, stores the reply field into offset 148 of a3 and returns the stored value.
- `return_zero_restore_d2_d4_a2_a3`: Sets d0 to zero as the return value, releases the a5 frame, restores d2 through d4 and a2 through a3 from the stack, then returns.
- `send_command_407_with_parameter`: Allocates a message block, fills it with command 407 and a caller-supplied parameter, sends it via the message routine, and stores the result at offset 148 of the allocated structure.
- `send_command_406_with_parameter`: Allocates a message block, fills it with command 406 and a caller-supplied parameter, sends it via the message routine, and stores the result at offset 148 of the allocated structure.
- `build_and_submit_io_request`: Saves d2-d3/a2-a3, allocates a stack frame, calls an allocator, fills a request structure with a command word, a caller pointer and a zero result, submits it via another routine and returns the result or zero.
- `copy_three_vectors_and_post_message`: Copies three five-longword tables from a4-relative data into local stack buffers, stores the three arguments into them and posts a message with fixed words and pointers through a system call.
- `find_node_by_key_in_list`: Locks a global list, walks the linked list at offset 66 comparing each node's field at offset 14 with the argument, unlocks and returns the matching node pointer or zero.
- `validate_and_apply_de2_block_edit`: Checks a block index against the DE2 handler's block count, verifies adjacent block pointers and sizes, calls the block copy and compare routines, updates the block pointer table and current edit pointers, and returns a status in d0.
- `insert_de2_block_and_shift_pointers`: Sets the handler's insert flag, optionally allocates a new block via the memory routines, copies the block data, shifts the block pointer table entries, and clears the insert flag before returning.
- `swap_de2_edit_pointer_pairs`: Exchanges three pairs of longword pointers in the a4-relative DE2 handler state, swapping the current and saved block pointers, then returns.
- `advance_entity_path_segment`: Walks the entity's path point array, compares segment lengths against the remaining distance, calls the move and lookup helpers, and stores the new segment offset or an error code in the entity structure.
- `extend_path_after_last_segment`: For the final path segment, calls the move helper, checks the entity state words, then writes the extended endpoint into the next path slot and returns.
- `shift_path_points_for_gap`: Computes the gap between adjacent path points, calls the allocation and move helpers, shifts the following points by the delta and updates the entity's path array.
- `branch_to_shared_epilogue`: Branches unconditionally to a shared epilogue elsewhere; the following unlk, movem and rts instructions are unreachable here.
- `xpk_pack_buffer_with_retry`: Saves registers, copies a 17-long parameter block, calls the packer, and on error -7 prints the xpk no-memory message and retries while the callback returns nonzero.
- `finish_pack_check_size`: If the pack result is not -7 it calls the cleanup routine, then compares the packed size in d5 against d3 and returns d5 or -2.
- `xpk_unpack_with_retry`: Saves registers, copies a 17-long parameter block from global memory, fills in the caller's pointers and size, calls the xpk unpacker, and retries after showing an error requester when it returns -7.
- `xpk_unpack_finish_and_return_size`: Checks d4 against -7, calls the cleanup routine with the stack buffer if needed, then returns -1, -2 or the unpacked size in d0 after restoring registers.
- `xpk_unpack_chunk_with_checks`: Computes remaining size, calls the unpacker with optional file check, stores the returned chunk length at the buffer end, updates the offset, and returns the length or -2.
- `de2_read_next_byte_or_advance_pointer`: Reads the word at the current DE2 input pointer, calls the byte-fetch routine to get a length, advances the output pointer by that length, and sets the status word to a busy or done code before returning the count in d0.
- `de2_allocate_buffer_and_start_decode`: Computes two offsets from the DE2 state, allocates a buffer, fills its fields with the current pointers and a type byte, then updates the output and input pointers before returning.
- `de2_decode_bytes_into_caller_buffer`: Repeatedly calls the decode step and copies bytes from the DE2 output pointer into the caller's buffer until the buffer is full or the decoder signals completion, returning the number of bytes written.
- `de2_write_buffer_with_checksum`: Copies bytes from a source buffer to a ring buffer, verifying checksums via a helper and returning the count written or minus one on mismatch.
- `de2_set_stream_offsets_and_flush`: Stores two longword arguments into offsets 12 and 16 of the structure pointed to by a2, then calls a flush routine with the structure pointer.
- `de2_main_command_loop`: Runs the DE2 handler main loop, initializing stream pointers, reading command packets and dispatching them to sub-handlers until the loop exits.
- `galaga_enemy_spawn_and_attack_control`: Reads enemy state fields through a4, checks spawn timers and positions, builds spawn records on the stack and calls subroutines to create and move enemies.
- `galaga_level_and_wave_setup`: Opens dos and intuition libraries, reads level data through a4, allocates memory, sets enemy counters and positions, and initializes wave state for the game.
- `signed_32bit_multiply_via_words`: Multiplies the 32-bit values in d0 and d1 using word-sized multiplies, combines partial products, clears the low word and returns the product in d0.
- `signed_divide_d0_by_d1_quotient`: Negates d0 and d1 as needed to make both positive, calls the unsigned division routine, then negates the quotient in d1 and returns it in d0.
- `signed_divide_d0_by_d1_remainder`: Makes d0 and d1 positive, calls the unsigned division routine, then negates the remainder in d0 and returns it in d0.
- `unsigned_32bit_divide_d0_by_d1`: Divides the unsigned 32-bit value in d0 by d1, returning the quotient in d0 and the remainder in d1, using divu and correction steps.
- `strlen_return_length`: Reads the string pointer from the stack, scans bytes until a null terminator, and returns the byte count in d0.
- `bounded_string_compare`: Compares two strings byte by byte up to a given length, returning zero if equal, minus one if the first is lower, or one if higher.
- `copy_string_return_destination_2`: Copies bytes from the source string to the destination until a null is copied, then returns the original destination pointer in d0.
- `bounded_string_copy_return_dest`: Copies bytes from the source pointer to the destination pointer until a null byte or the count in d0 is exhausted, then null-terminates if room remains and returns the destination in d0.
- `case_insensitive_string_compare_table`: Compares two strings byte by byte through a translation table, returning zero when equal, minus one when the first sorts lower, and one when higher, honoring the length limit in d0.
- `allocate_and_init_message_port`: Allocates a 34-byte structure, opens a message port, stores the reply port and signal fields, initializes the list header, and returns the structure pointer or zero on failure.
- `de2_handler_cleanup_and_dispatch`: Saves a2, loads the object pointer from the stack, and if non-null calls the sub-object cleanup, sets byte 14 to 2 after invoking the byte-15 callback, then calls the object routine with selector 34.
- `init_ring_buffer_pointers`: Loads the structure pointer from the stack, sets its first longword to point just past itself, zeroes the second longword and stores the structure address in the third, returning zero in d0.
- `install_two_handler_pointers`: Stores the address of one local table into a global pointer and the address of a second local table into another global pointer, then returns zero in d0.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_offset_minus_e4`: Saves d2 and a6, loads two longwords from the caller's stack into d1 and d2, fetches the library base from a4 offset and jumps to its offset minus e4 vector.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_330_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.

### DE3-Handler

- `allocate_and_init_handler_memory`: Allocates a memory block via exec, copies a table into it, zeroes a region, stores pointers, calls several setup routines, then frees the block and returns a status.
- `copy_three_tables_and_call_setup`: Copies three five-longword tables from a4 offsets into local stack buffers, stores the three arguments and pointers, then calls a setup routine with eight stack arguments.
- `clear_byte_at_index_and_recurse`: Saves a2-a3, loads a buffer pointer and index pointer from the stack, recursively calls the same routine with the index advanced by one, then zeroes the byte at the buffer offset given by the index and returns that index in d0.
- `allocate_24_or_44_byte_block`: Reads a word at offset 20 of the structure pointed to by the a4-relative global; if below 36 it allocates a 44-byte block with size 65536, otherwise allocates with zero arguments, returning the pointer in d0.
- `free_block_by_size_threshold`: Loads a pointer argument and checks the word at offset 20 of the a4-relative global structure; if below 36 it frees the pointer with a zero size argument, otherwise frees it with size 44.
- `allocate_and_init_de3_node`: Saves a2-a3/a6, allocates a zeroed block via a library call, links it into the list at the caller's pointer, clears two fields of the pointed node, then calls four setup routines on the new block and returns it in d0.
- `lookup_de3_entry_by_index`: Saves registers, computes an index-scaled offset into the table at a4-relative base, calls a lookup routine with size four, compares the result against the table base and stores the chosen table pointer in the frame before continuing.
- `build_de3_request_and_dispatch`: Builds a request structure on the stack with a count of twenty-three and the caller's d2 and table pointer, passes it with a field from a2 to the node allocator, and returns zero or the allocated node in d0.
- `copy_a5_field_to_a3_offset_and_test`: Copies the longword at offset minus 32 from a5 into offset 148 of a3, tests the longword at minus 36 from a5, and sets d0 to zero if it is nonzero before branching.
- `copy_65_longwords_and_call_compare`: Copies 65 longwords from the pointer at minus 72 of a5 to the pointer at offset 32 of a5, then calls a routine with both pointers plus 8 and returns 1.
- `allocate_and_register_de3_entry`: Allocates a structure, fills fields from arguments, calls a registration routine with a stack record, frees temporary memory, stores a result at offset 148 and returns a handle or zero.
- `dispatch_de3_handler_via_vector`: Saves d2-d3/a2-a3/a6, reads three arguments from the stack, opens an a5 frame, allocates a small block, and either forwards the arguments to another routine or calls the fallback with them.
- `build_de3_request_and_call`: Builds a request structure on the a5 frame from d2, a2 and d3, calls a routine with it, and on success stores a result field into a6 offset 148 and returns a value.
- `return_minus_one_restore_regs`: Sets d0 to minus one as the return value, releases the a5 frame, restores d2-d3/a2-a3/a6 from the stack and returns to the caller.
- `build_and_send_de3_record`: Saves registers, allocates a stack frame, calls an allocator, fills a record with type 1007 and a field from the indexed table, sends it via a helper, then frees the record and returns its pointer or zero.
- `send_de3_record_with_three_params`: Saves registers, allocates memory, builds a record of type 8 holding three arguments, sends it through a helper, stores a result field at offset 148 of the allocated block, and returns the stored value or zero.
- `copy_indexed_string_and_clear_source`: Takes a destination buffer and an index, computes the indexed entry, calls a string copy helper with the entry's length and text, then clears the first byte of the source entry and returns the length.
- `find_node_by_type_and_pointer`: Walks a linked list of nodes starting from a table entry, returning the node whose type field equals 2 and whose pointer field matches the argument, or zero if none found.
- `allocate_object_and_store_field`: Calls an allocation routine with zero, stores the passed argument at offset 152 of the returned object, and returns with d2 and a2 restored.
- `open_de3_handler_with_retry`: Allocates a 4000-byte buffer and opens the DE3 handler, retrying on failure, then copies several globals into handler state and returns success or zero.
- `setup_de3_handler_state_and_check_status`: Copies several handler state fields into adjacent slots, builds a parameter block, calls three subroutines, and returns zero if a status field equals 200, otherwise one.
- `swap_node_link_and_wait_for_completion`: Saves the second field of the passed node, replaces it with a global value, calls a subroutine with the node's first field, waits on a global until nonzero, then restores the saved field.
- `forward_two_node_fields_to_subroutine`: Pushes the second and first fields of the passed node onto the stack, calls a subroutine with those two arguments, cleans the stack and returns.
- `find_node_by_two_fields`: Searches the linked list at the global list head, comparing each node's field at offset 4 and offset 16 against the caller's argument table, returning the matching node pointer or zero.
- `parse_and_validate_gate_record`: Reads a 36-byte record via a library call, checks type and range bytes, extracts a 16-bit value or computes level times 1000, formats a string and returns one on success or zero.
- `open_gate2_file_and_wait`: Opens the Gate2 file with a 4000-byte buffer, stores the handle, sets a flag and copies pointers, then polls the file until data arrives and returns a value when the status word equals 123.
- `de3_handler_main_dispatch_loop`: Saves registers, opens a stack frame, then repeatedly fetches a message node, reads its command word at offset 8 and branches to the matching handler for values such as 1004, 5, 7, 23, 24 and 20.
- `branch_to_main_dispatch_tail`: Branches unconditionally back to the main dispatch loop, then contains unreachable code that would unlink a5, restore d2-d5/a2-a3/a6 and return.
- `relink_message_nodes_between_queues`: Allocates a message, initializes a queue, then loops moving nodes from one list to another by swapping their offset-14 links and calling the list insert and remove helpers.
- `open_dos_and_intuition_libs_setup_handler`: Opens dos.library and intuition.library, allocates two message ports, checks a window type field, sets window width and calls game routines to run the handler loop, then closes the opened libraries.
- `signed_divide_d1_into_d0`: Calls the signed division routine with d1 as divisor and d0 as dividend, then moves the quotient from d1 into d0 and returns it.
- `signed_divide_with_sign_normalization`: Normalizes the signs of d0 and d1, negating them as needed, then branches to the signed division core and negates the quotient back before returning.
- `unsigned_32bit_divide_with_remainder_2`: Divides d0 by d1, returning quotient in d0 and remainder in d1; handles zero divisor, 16-bit divisor and full 32-bit divisor paths, preserving d2-d4.
- `strlen_count_bytes_until_nul`: Reads the pointer argument from the stack, scans bytes until a zero byte, and returns in d0 the length excluding the terminator.
- `bounded_byte_string_compare`: Compares two byte strings from stack arguments up to a length in d0, returning zero if equal, minus one if first is less, one if greater.
- `append_string_to_end`: Scans the destination string in a0 for its terminating zero, then copies bytes from the source string in a1 including the terminator, returning the original destination pointer in d0.
- `copy_string_to_destination`: Copies bytes from the source string in a1 to the destination in a0 one at a time until a zero byte is stored, returning the original destination pointer in d0.
- `bounded_string_copy_with_terminator_2`: Copies up to d0 bytes from the source in a1 to the destination in a0, stopping at a zero byte, and writes a terminating zero if space remains, returning the destination pointer.
- `compare_strings_via_translation_table`: Walks two byte strings, mapping each byte through a shared translation table and comparing the mapped values, returning minus one, zero or one for ordering.
- `get_string_length_via_stack_pointer`: Pushes the address of a local and the passed string pointer, calls a length routine, then returns the resulting length in d0 through a0.
- `allocate_and_initialize_message_port`: Allocates a 34-byte structure, stores a signal byte, type, pointer and zeroed fields, creates a message port at offset 20 and optionally queues the structure.
- `release_de3_node_and_notify`: Pops a2, loads the node pointer from the stack argument, and if non-null calls three subroutines to detach it, send its byte value, and post message 34 before returning.
- `init_de3_list_header`: Loads the pointer argument into a0, sets its first longword to point just past itself, zeroes the second longword and stores a0 in the third, then returns zero in d0.
- `install_de3_handler_pointers`: Stores two addresses relative to a4 into two other a4-relative longwords, setting up handler pointers, then returns zero in d0.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `call_library_vector_42_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_132_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_234_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_132_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_246_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_324_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_330_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_366_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_348_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.

### DE1-Handler

- `allocate_and_init_handler_memory_2`: Saves registers, allocates a memory block via exec.AllocMem, copies a table into it, zeroes a second area, stores the exec base and stack pointer, then runs setup subroutines and frees the block on exit.
- `allocate_two_signals_and_install_callback`: Allocates two signals via a helper, stores their bit masks, saves the old callback pointer at offset 180 of the structure and installs the new one, returning one on success.
- `check_pointer_against_linked_list`: Calls a lock helper, validates the argument, compares a field at offset 16 against a stored value and walks a linked list looking for a matching node, returning one if found.
- `dispatch_de1_message_by_type`: Saves a2-a3/a6, loads the DE1 message pointer from a4-relative memory, reads its type field and branches through a chain of subtractions to per-type handlers, returning 1 or 2 in d0.
- `check_de1_type_16_flag_and_dispatch`: Branches to the shared exit, tests a4-relative word, calls the message check routine with the message field, compares byte 16 of the a4-relative structure to 2, then calls the dispatch routine and returns 1 or 2 in d0.
- `check_de1_type_17_and_pending_flag`: Branches to the shared exit, tests a4-relative word, calls the message check routine, compares byte 17 of the a4-relative structure to 2, tests a4-relative longword, then calls the dispatch routine and returns 1 or 2 in d0.
- `check_flag_and_byte80_then_dispatch`: Tests a word flag in a4-relative memory, validates a pointer via a call, checks byte 80 of a structure against 2, then calls a two-argument routine and returns 1 or 2 in d0.
- `check_flag_and_byte81_then_dispatch`: Tests a word flag in a4-relative memory, validates a pointer via a call, checks byte 81 of a structure against 2, then calls a two-argument routine and returns 1 or 2 in d0.
- `validate_two_pointers_and_compare_result`: Validates a pointer, compares a call result against 1, validates two fields of a second structure, then calls a two-argument routine and returns 1 or 2 in d0.
- `return_two_restore_regs`: Sets d0 to 2 as the return value, pops a2, a3 and a6 from the stack, then returns to the caller.
- `poll_input_and_advance_game_state`: Reads input flags, checks a bit selected by a byte in a structure, then repeatedly fetches and updates objects, setting their field to 123 or 200 and a flag word to 1 until the state pointer matches.
- `branch_back_to_state_loop`: Branches unconditionally back to the earlier state-check loop; the following register restore and return are unreachable.
- `send_two_messages_to_port`: Loads the port structure from the a4-relative pointer, pushes its message field and sends it twice via two library calls, once with an extra argument of 30, then returns.
- `open_and_configure_serial_port`: Allocates a message port, fills a reply structure, opens the serial device with a 256-byte buffer, sets the port field at offset 180, then waits for a reply and delays 200 ticks.
- `open_gate1_dos_file_and_start_handler`: Opens the Gate1 dos.library file, copies its 30-longword header, checks the version fields, allocates a buffer, sets up message ports and calls the handler setup routines before returning.
- `signed_divide_d1_into_d0_2`: Tests d1 and d0 signs, negates operands as needed, calls the division routine, then returns the quotient in d0 with the correct sign applied.
- `unsigned_32bit_divide_with_remainder_3`: Divides d0 by d1 using word-sized divu steps, handling zero divisor by returning minus one, and returns quotient in d0 and remainder in d1.
- `allocate_and_initialize_handler_record`: Allocates a 34-byte record, stores a signal byte, type 4, caller pointer and a zeroed field, initializes a sub-structure, and registers the record if a pointer was supplied.
- `flush_and_reinit_message_port`: Pushes a2, loads the port pointer from the stack argument, and if non-null flushes its pending message, sets its signal byte to 2 and calls the port cleanup with reason 34.
- `init_message_port_header`: Loads the port pointer from the stack, sets its first longword to point just past itself, zeroes the next longword and stores the port pointer in the third longword, returning zero in d0.
- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_port_signal_loop`: Saves registers, opens the exec library, waits on signal bit 256, fetches messages from the port and loops until the port state reaches 1 or 2, returning the message.
- `wait_for_de1_state_and_return_saved_d0`: Stores incoming d0 in a global, clears the state word, calls two library vectors, then waits until the state word is 1 or 2 and returns the saved d0.
- `de1_zero_filled_data_area`: Twelve zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `de1_alloc_copy_or_free_buffer`: On selector 1 allocates 184 bytes of chip memory, copies 184 bytes from a source table into it, stores the caller pointer in a global and returns the buffer; on selector 2 frees the caller pointer.
- `call_library_vector_138_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_132_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_252_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_318_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_324_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_330_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_366_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.

### xpkNUKE.library

- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `init_xpk_decompress_context`: Links a stack frame, saves registers, stores the caller's a0 and d0 in the context, sets fields at offsets 8, 10, 14, 20, 22 and 24, copies 256 bytes to offset 58, then branches into the relocation loop.
- `copy_block_and_apply_relocations`: Copies d0 bytes from a0 to a1, advances the context pointer by 256, reads a count, then adds the base offset to each listed longword before jumping to the finish routine.
- `restore_registers_and_return_context`: Loads a6 from the saved context, calls the library close routine, returns the saved d0 value, restores d7/a2/a4/a6, releases the a5 frame and returns to the caller.
- `increment_refcount_clear_flag`: Saves a2/a4/a6, sets a4 and a2 to the library base in a6, increments the word at offset 32, clears bit 3 of the byte at offset 14, and returns the base in d0.
- `decrement_refcount_release_if_idle`: Saves registers, decrements the word at offset 32 of the library base, and if it reaches zero with bit 3 of byte 14 set, calls the release routine and returns its result in d0.
- `mark_busy_free_memory_and_remove`: Sets bit 3 of byte 14, and when the word at offset 32 is zero, calls the internal cleanup, frees the memory block via exec FreeMem and removes the library node via exec Remove, returning the saved value.
- `get_library_base_plus_e0`: Saves a4, loads a4 from offset 58 of the library base in a6, then returns the address a4 plus 166 in d0 and restores a4.
- `free_library_allocated_buffers`: Frees the two memory blocks pointed to by offsets 8 and 12 of the structure at offset 52 of a0 via exec.FreeMem, then frees the structure itself.
- `zero_two_library_buffers`: Clears the buffer at offset 8 of a0 with 16384 longword stores and the buffer at offset 12 with 8192 longword stores, writing zero.
- `fill_buffer_with_d7_words`: Loads a count of 64 shifted left by 7 into d6, then loops storing d7 twice per iteration to the buffer pointer at offset -4 of a5, advancing it and decrementing d6 until zero.
- `restore_registers_and_return`: Pops d6, d7 and a2 from the stack, releases the a5 stack frame and returns to the caller.
- `allocate_and_init_stream_buffers`: Allocates three memory blocks via exec.AllocMem for the structure at a2, stores them at offsets 8 and 12, initializes fields from a0, and returns zero or a negative error code.
- `setup_io_request_from_params`: Copies fields from the a2 parameter block into the buffer at -4(a5), stores the word at 32(a0) and the length minus four at 34(a0), then calls the transfer routine and stores its result at 16(a2), returning zero or -17 if out of range.
- `return_zero_restore_regs_a2_a4_a6`: Sets d0 to zero as the return value, restores a2, a4 and a6 from the stack, releases the a5 frame and returns to the caller.
- `stub_return_zero_with_a4_frame`: Saves a2 and a4, loads a4 with the address 58 bytes past a6, copies a0 into a2, sets d0 to zero, restores a2 and a4 and returns.
- `dispatch_xpk_decompress_block`: Saves a2/a4, builds source and destination pointers from the a0 descriptor, swaps them, loads a4/a5 from the descriptor and calls the decompression routine at offset 0992, then returns zero in d0.
- `stub_set_a4_and_return`: Saves a2/a4 on the stack, loads a4 with the library base plus 58, copies a0 into a2, restores a2/a4 and returns without doing anything else.
- `refill_bit_buffer_and_copy_bytes`: Decrements the bit counter at offset 44 of a4, refills the bit buffer from the stream pointer when empty, then copies d7 plus one bytes from the a5 buffer backwards into the a4 output pointer.
- `emit_two_bitplane_words_and_advance`: Decrements two counters at offsets 44 and 46 in the a4 library state; when each expires it writes the shifted word at 52 or 54 to the pointer at 64 or 68, advances that pointer by two and reloads the counter, then shifts the words left and merges d7 bits.
- `emit_second_bitplane_word_from_d0`: Negates d0, masks it to two bits in d7, decrements the counter at offset 46 in a4; on expiry writes the word at 54 to the pointer at 68, advances it by two, reloads the counter and shifts the word twice before oring in d7.
- `scan_string_table_for_match`: Saves registers, calls the setup routine, reads a two-byte key from the a5 table indexed by d5, looks up a word in the a3 table and walks the a5 string table comparing bytes to find a matching entry, updating counters at offsets 40, 38 and 42 in a4.
- `xpk_match_previous_byte_run`: Compares the byte before the current position with the current byte in the a5 buffer, scans a run of equal bytes, and updates the a4 match state counters and output pointer.
- `xpk_start_new_match_at_position`: Computes the remaining match length from d5 minus the a4 limit, clamps it to zero, and sets up a0, d2, d0, d7, d3 and a2 for the next match search.
- `xpk_extend_match_through_tree`: Walks the a6 binary tree comparing bytes at a2 offsets, tracks the longest match in d2 and d3, and stores the resulting match index into the a6 table.
- `check_d7_sign_then_dispatch`: Reads the word at offset 40 of the a4 context into d7, branches away if it is negative, otherwise calls the routine at pc-relative offset and then branches forward.
- `decode_bitstream_into_output_words`: Decrements counters in the a4 context, flushes accumulated bits to output pointers, walks the code table indexed by d2 and d3, and rotates bit fields into the output stream.
- `emit_extra_bitfield_words`: Clears d7, decrements the a4 context counter, flushes a word to the output pointer, shifts the accumulator, and loops emitting bitfield words while d0 exceeds fourteen.
- `lzw_decode_bitstream_into_output`: Reads a 4-bit code from the packed bit accumulator at offset 56 of the library state, refills it from the input pointer when exhausted, and writes decoded symbols to the output buffer at offset 28.
- `lzw_copy_string_and_advance_output`: Copies a byte from the input string into the low byte of d1, doubles it as a table index, writes the table entry to the output at a6+d7, stores d5 in the table, and advances d7 and d5.
- `reset_lzw_decoder_state`: Clears the decoder counters and table pointers in the a4 state block, sets input and output pointers from the block fields, initializes the code size and bit accumulator, and returns.
- `update_shifted_output_registers`: Compares d5 with the word at offset 32 of a4, updates the counter and stored word at offsets 40 and 38, calls a helper, then writes shifted values from a4 fields to the pointers at offsets 64, 68, 72 and 76.
- `compute_buffer_length_difference`: Loads the long at offset 20 of a4, subtracts the long at offset 16, and if the result is negative sets d0 to minus one before branching onward.
- `pad_and_copy_buffer_bytes`: Computes a byte count from a4 fields, fills bytes with 251 until the destination pointer is aligned to four, then copies the remaining bytes from the source pointer.
- `copy_bytes_and_return_offset`: Copies d2+1 bytes from the address in a2 to the address in a1, advancing both pointers, then returns in d0 the final a1 minus the base pointer stored at offset 4 of a4.
- `init_state_and_call_allocator`: Saves d2-d7 and a3-a6, zeroes d0-d7, sets d2 and d3 to 32768, a3 to 16, loads a table address into a6, calls the allocator, and returns a0 in d0 after restoring registers.
- `decode_bitstream_and_dispatch`: Reads a longword from a5, extracts a table index, shifts bits into d6 using counts from a6, adjusts a1 by a computed offset, then jumps through a table entry at offset 64 of a6.
- `xpk_bitstream_copy_byte`: Shifts d2 left to pull bits from the a5 bitstream, refilling from (a5)+ when empty, then copies one byte from -(a4) to (a0)+ and returns early if a0 reaches a2.
- `xpk_decode_d3_jump_table`: Clears d0, shifts d3 left refilling from (a5)+, accumulates carry bits into d0, then jumps through the pc-relative table indexed by d0.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.

### Galaga 94 (1994)(H. Mueller)(De)[cr Avantgarde].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
