# Game 162 (Amiga Karate): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00162). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 48 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 76 raw-word lines |
| Functions named and described | pass, 443 of 443 named (42 author symbols), 443 of 443 described (87 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1173 of 1173 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11805 of 11805 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 535569 |
| code | 40568 |
| data | 308324 |
| padding | 12563 |
| unused | 4096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| endcli | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| loadwb | 11 | 362 | 22 |
| icon.library | 15 | 1414 | 108 |
| diskfont.library | 13 | 830 | 67 |
| Amiga Karate.adf:  AMIGA KARATE | 94 | 8876 | 240 |
| Amiga Karate.adf:spdata | 1 | 71 | 1 |
| Amiga Karate.adf:bootblock | 1 | 18 | 1 |

## Cases handled

- Amiga Karate/Amiga Karate.adf:ata: high-entropy data file (7.84 bits/byte), classified as packed data
- Amiga Karate/Amiga Karate.adf:de1: high-entropy data file (7.68 bits/byte), classified as packed data
- Amiga Karate/Amiga Karate.adf:de3: high-entropy data file (7.71 bits/byte), classified as packed data
- Amiga Karate/Amiga Karate.adf:sr2: high-entropy data file (7.97 bits/byte), classified as packed data
- Amiga Karate/Amiga Karate.adf:sr4: high-entropy data file (7.96 bits/byte), classified as packed data
- Amiga Karate/Amiga Karate.adf:sr5: high-entropy data file (7.86 bits/byte), classified as packed data
- Amiga Karate/Amiga Karate.adf:sr6: high-entropy data file (7.95 bits/byte), classified as packed data
- Amiga Karate/Amiga Karate.adf:part0: high-entropy data file (7.92 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### endcli

- `endcli_find_max_node_and_alloc_seglist`: Walks the resident list at offset 356 of a2 to find the highest node value, allocates a segment list via library calls, fills it with tagged entries and copies 16 longs between the new structure and offset 536 of a2.
- `return_d1_in_d0_then_branch`: Moves d1 into d0 as the return value and branches to the code at offset 260, which is outside this two-instruction fragment.

### echo

- `echo_find_max_node_and_alloc_seglist`: Same resident-list scan and segment-list allocation as the endcli entry, but reached from the echo program, copying 16 longs to and from offset 536 of a2.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0 and branches to another routine, so the value is passed on as the return value.

### loadwb

- `loadwb_startup_and_parse_args`: Saves registers, opens the current directory, splits the command line into whitespace-separated arguments, opens the workbench library and starts the main program.
- `loadwb_cleanup_and_exit`: Restores the saved stack pointer, closes the workbench library, replies to the startup message, forbids the task and returns the exit code.
- `open_and_close_workbench_library`: Loads the workbench library base at offset 92 from a4, calls its open vector, then calls its close vector on the same base and returns.
- `start_workbench_task_via_dos`: Clears the global task pointer, loads the workbench task name into a1 and calls the dos library start function with command 31, storing the result in the global and returning.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_dup_lock`: Library stub: loads its arguments and the dos library base, calls DupLock and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `launch_workbench_tool_with_args`: Opens a workbench library, optionally checks a tool argument, starts the workbench task, computes scaled offsets from the library base, reports errors and closes the library before returning.
- `compare_two_strings_return_order`: Compares the two null-terminated strings passed on the stack byte by byte, returning zero if equal, minus one if the first is lower, or one if higher.
- `build_linked_list_from_array`: Allocates nodes of eight bytes, fills each node's second word from an array entry, links nodes through the first word, and returns the head pointer.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_get_msg`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_global_vector_with_stack_args`: Saves d2, loads the stack argument into d2, pushes a pointer to the caller's argument area and a global longword, then calls through address zero.
- `format_string_into_stack_buffer`: Saves a2-a4/a6, reserves 140 bytes on the stack, calls the exec vector at offset -522 to format into that buffer, scans for the terminating zero, and if non-empty passes the length and buffer to another routine.
- `store_d0_byte_and_return`: Writes the low byte of d0 to the address in a3 and increments a3, then returns; used as the put-character callback for the formatting routine.

### icon.library

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `icon_library_open_increment_and_clear_bit`: Library code that increments the word counter at offset 32 of the library base in a6, clears bit 3 of the byte at offset 14, and returns the base in d0.
- `decrement_open_count_and_close_if_last`: Sets d0 to zero, decrements the word open counter at offset 32 of the library base in a6, and if it reaches zero with bit 3 of the flags byte set, calls the close routine at offset 256.
- `close_library_when_unused`: Checks the word at offset 32 of the a6 library base; if zero, calls the library close vector at offset -210 through the base at offset 34, then unlinks the library node from its list and returns the saved value in d0.
- `set_close_flag_and_return_zero`: Sets bit 3 of the flags byte at offset 14 of the a6 library base to mark the library for closing, then returns zero in d0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_and_store_base`: Saves a2, stores a0 and a6 into the new structure at offsets 42 and 34, opens dos.library version 33 via exec, stores the base at offset 38, and alerts on failure.
- `copy_icon_bases_to_globals`: Copies the library base and dos base from offsets 34 and 38 of the a2 structure into two absolute global longs, returns a2 in d0 and pops a2.
- `zero_filled_data_area`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_area_2`: Contains a single zero word that decodes as an ori.b immediate instruction; this range is zero-filled data referenced by address, not executable code.
- `save_a0_a6_call_library_vector`: Pushes a0 and a6 onto the stack, calls the library vector at address zero, then discards both saved registers and returns.
- `save_a0_a1_a6_call_vector_aa`: Pushes a0, a1 and a6 onto the stack, calls the library vector at offset aa, then releases the saved registers and returns.
- `icon_lib_save_a0_a2_call_helper`: Pushes registers a0 through a2 and a6 onto the stack, calls a library helper, then discards the saved registers and returns.
- `icon_lib_save_a0_a1_call_helper`: Pushes registers a0, a1 and a6 onto the stack, calls a library helper, then removes the saved registers and returns.
- `icon_lib_save_a0_call_helper`: Pushes register a0 and a6 onto the stack, calls a library helper, then discards the saved registers and returns.
- `icon_lib_call_698_with_a0_a6_saved`: Pushes a0 and a6 onto the stack, calls the library routine at offset 698, then adds 8 to the stack pointer and returns.
- `icon_lib_call_6b2_with_a6_saved`: Pushes a6 onto the stack, calls the library routine at offset 6b2, then adds 4 to the stack pointer and returns.
- `icon_lib_call_72e_with_a0_a2_a6_saved`: Pushes a0 through a2 and a6 onto the stack, calls the library routine at offset 72e, then restores the stack pointer by adding 16 and returns.
- `icon_library_call_782_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 782, then discards both saved registers and returns.
- `icon_library_call_820_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 820, then discards both saved registers and returns.
- `icon_library_call_842_with_a0_a1`: Pushes a0 and a1 onto the stack, calls the library routine at offset 842, then discards both saved registers and returns.
- `icon_library_call_88a_via_a0_a1`: Saves a0 and a1 on the stack, calls the library routine at offset 88a, then discards the saved registers and returns to the caller.
- `icon_library_call_8fe_via_a0_a1`: Pushes a0 and a1 onto the stack, invokes the library routine at offset 8fe, cleans the stack by adding eight and returns.
- `icon_library_set_field_94_from_stack`: Opens the library base from a6, calls the vector at offset minus 294, stores the stack argument into offset 148 of the returned pointer, then restores a6 and returns.
- `zero_filled_data_area_3`: Contains only a zero word that decodes as ori.b with immediate zero, so this range is zero-filled data referenced by address, not executable code.
- `open_icon_from_drawer_by_name`: Looks up an icon by name in a drawer, copies its 44-byte structure and fields into the icon block, links it to its parent and returns the icon pointer or zero.
- `copy_icon_fields_and_link_parent`: Copies 44 bytes from the stack frame into the icon block at offset 96, stores several fields, and links the icon into its parent's child pointer.
- `return_zero_restore_regs`: Sets d0 to zero as the return value, restores d2 through d4 and a2 from the stack, releases the a6 frame and returns to the caller.
- `copy_icon_header_and_dispatch`: Copies 44 bytes from offset 96 of the icon structure into a local buffer, fills header fields and copies several longwords, then calls the dispatch routine with the buffer and two arguments.
- `load_icon_from_disk`: Builds a filename, opens the icon file, reads and validates the header, then loads associated image and tool data before returning a status.
- `load_icon_tooltypes_and_images`: Calls a hook, checks the icon header magic and version, clears two fields in the icon structure, then loads tool types and image data via library calls, returning success or failure.
- `load_icon_select_and_image_data`: Loads two image pointers through a library call, then allocates and reads the select and image data blocks into the icon structure, returning success or failure.
- `read_icon_image_array_entries`: Loops over image array entries, allocating and reading each block through library calls, then optionally loads a final block and returns one on success.
- `zero_d6_and_branch_to_shared_exit`: Sets d6 to zero with a quick move and branches to the shared exit path at the following routine, part of icon.library.
- `restore_registers_unlink_and_return`: Pops d2 through d6 and a2 through a5 from the stack, releases the a6 frame with unlk and returns to the caller in icon.library.
- `open_icon_tool_window_and_load_images`: Saves registers, opens a workbench window, loads icon images and their tool types, then closes the window and restores the saved pointer before returning.
- `draw_icon_with_optional_second_image`: Saves a2 offset 26, calls three library helpers to set up drawing, then conditionally draws the main image, an optional second image at offset 66, and images at offsets 22 and 26, returning d5 status.
- `draw_icon_text_and_list_images`: Draws images at offsets 22 and 26 via a helper, then if offset 50 is set draws a length-prefixed string and its text, and if offset 54 is set walks that list drawing each entry.
- `advance_icon_list_and_draw_entries`: Advances a4 and the frame counter by four while entries are nonzero, calls the draw callback, then redraws each entry from the list at offset 54 and returns d5 status.
- `draw_icon_list_with_second_image`: Iterates a null-terminated list of image pointers, converts each to a name string, draws it via the a3 callback, then draws the optional image at offset 70 of a2 and stores the result in a2.
- `free_icon_name_chain`: Takes a name pointer from the stack, advances two bytes, and repeatedly calls the free routine on each chained name until a null result is returned.
- `restore_d2_d3_and_return`: Pops registers d2 and d3 from the stack and returns to the caller, restoring the saved registers only.
- `call_icon_cleanup_with_offset`: Saves d2, loads the second stack argument into d2 and the third into a0, then calls the routine at offset 140 of that structure with d2 pushed, restores the stack and d2, and returns.
- `allocate_and_init_icon_structure`: Saves d2-d4/a2, allocates 65536 bytes of memory, initializes fields at offsets 140 and 142 of the new block, and returns the block pointer or zero on failure.
- `finalize_icon_after_init`: Pushes the structure pointer and its offset 140 field, calls the initialization routine, and on success returns the structure pointer in d0 after cleaning the stack.
- `icon_lib_return_zero_pop_regs`: Library code that sets d0 to zero as the return value, restores d2 through d4 and a2 from the stack, then returns to the caller.
- `icon_lib_decrement_count_and_store_pair`: Library code that decrements the word at a2, calls a helper when it reaches zero, then stores two arguments into the table pointed to by a2's field and returns one.
- `icon_lib_store_pair_after_decrement`: Library code that decrements the word at a2, computes an offset from it, and stores d2 and a stack argument into the table at a1 plus sixteen, returning one.
- `open_icon_by_name_and_attach`: Allocates a 94-byte icon structure, initializes its name field, opens the icon by name, then attaches the caller's data and returns the icon pointer or zero on failure.
- `return_icon_pointer_in_d0`: Moves the icon pointer held in d6 into d0 as the return value and branches to the shared epilogue that restores registers.
- `free_icon_and_return_zero`: Pushes the icon size and pointer, calls the free routine to release the icon memory, sets d0 to zero, restores d2-d6/a2 and returns.
- `icon_call_with_offset_4e`: Saves d2-d4, loads two arguments from the stack, adds 78 to the second argument, pushes both and calls a library routine, then restores registers and returns.
- `icon_find_equals_in_list`: Saves registers, walks a null-terminated pointer list, compares each entry against a string using a library call, and returns the text after the first matching equals sign or zero.
- `icon_return_zero_restore_regs`: Sets d0 to zero as the return value, restores d2-d3 and a2-a3 from the stack and returns to the caller.
- `icon_find_matching_entry_by_size`: Saves d2-d6, calls a lookup on the second argument, then scans entries from the first argument comparing sizes and calling a match test until a match is found or the list ends.
- `icon_verify_entry_size_and_match`: Calls a lookup on d2, compares the result with d5, and if equal calls a match test with d5, d3 and d2, returning 1 on success or 0 otherwise.
- `icon_advance_to_next_entry`: Tests d4; if nonzero, copies d4 plus one into d0 and d2, then branches back into the caller's scan loop to continue searching.
- `zero_d2_and_branch_to_shared_exit`: Sets d2 to zero with a quick move, then branches unconditionally into the shared exit path of the icon library routine.
- `return_zero_and_restore_d2_d6`: Sets d0 to zero as the return value, restores registers d2 through d6 from the stack, and returns to the caller.
- `parse_icon_tooltypes_into_buffer`: Saves d2-d6 and a2, reads two stack arguments, clears a byte at offset 30, then calls library helpers to parse tooltype strings and copy up to 30 bytes into the destination buffer.
- `icon_parse_skip_five_bytes_and_check`: Advances a2 by five, pushes three arguments and calls a library routine, then tests d0 and skips twelve stack bytes; on failure sets d5 to one and advances a2 by three.
- `icon_parse_set_zero_and_branch`: Sets d5 to zero and branches forward into the shared parsing code, indicating a successful parse result.
- `icon_parse_decimal_number_loop`: Accumulates a decimal number in d5 from characters at a2, calling a digit conversion routine, then validates and formats the parsed value via library calls before returning it in d0.
- `ascii_digit_to_value`: Reads a longword argument from the stack, checks it lies between the characters 0 and 9, and returns its numeric value in d0 or minus one otherwise.
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
- `icon_lookup_with_fallback`: Saves d2-d5, calls a lookup with two arguments, and on success calls a second routine with four arguments; if that fails it calls a cleanup routine and returns zero.
- `allocate_icon_header_and_init_size`: Saves d2-d3/a2, allocates 96 bytes of memory type 65536, stores 10 in the new block's offset 14 and in the caller's word, then links the block into the caller's list and returns 1.
- `return_zero_restore_registers`: Sets d0 to zero as the return value, restores d2-d3/a2 from the stack and returns to the caller.
- `compare_three_args_result_to_fourth`: Saves d2-d3, loads three arguments and a fourth value, calls a comparison routine with the first three, and returns its result only if it equals the fourth argument, otherwise zero.
- `return_d1_in_d0_restore_d2_d3`: Copies d1 into d0 as the return value, restores d2 and d3 from the stack, then returns to the caller.
- `compare_three_args_via_inner_call`: Saves d2 and d3, loads three stack arguments, calls an inner routine with them, and returns its result in d0 only if it equals the fourth argument, otherwise zero.
- `return_d1_in_d0_restore_d2_d3_2`: Library code that copies d1 into d0 as the return value, pops d2 and d3 from the stack, and returns to the caller.
- `unsigned_32bit_divide_d1_by_d2`: Compares d2 with 65535; if larger, runs a 16-step shift-subtract division of d1 by d2, otherwise divides the two 16-bit halves of d1 by the low word of d2, returning quotient in d0 and remainder in d1.
- `unsigned_32bit_divide_d1_by_d2_loop`: Shifts d1 left through 16 iterations, subtracting d2 from the high part when it fits and setting quotient bits in d1, leaving quotient in d1 and remainder in d0.
- `multiply_d0_by_d1_into_32bit`: Multiplies the 32-bit value in d0 by the 16-bit value in d1 using partial products, restores d2 from the stack and returns the 32-bit product in d0.
- `icon_library_format_string_via_rawdofmt`: Saves a2-a4/a6, loads format string and argument pointer from the stack, loads a format template from pc-relative memory, calls exec RawDoFmt through a6, then restores registers and returns.
- `icon_library_put_byte_to_a3`: Stores the low byte of d0 at the address in a3 and advances a3 by one, used as a pc-relative output callback for formatting.
- `icon_library_strlen_bounded_255`: Scans the string at the stack argument a0 for a null byte, counting up to 255 characters, and returns the length in d0.
- `find_char_in_string_or_return_end`: Loads a0 from the first stack argument and d0 from the second, reads bytes from (a0) into d1, and if the byte is zero returns the current a0 in d0; otherwise compares d1 with d0 and, on a match, backs a0 up by one and returns that address in d0.
- `return_zero_and_branch_to_shared_exit`: Sets d0 to zero with a quick move and branches forward into the shared exit path of the surrounding string routine, so the caller receives zero.
- `copy_string_until_nul_terminator`: Loads a0 and a1 from the stack arguments, then copies bytes from (a1) to (a0), incrementing both pointers, until a copied byte is zero, and returns.
- `skip_leading_byte_and_branch`: Loads a0 and a1 from the stack, advances a0 past one byte, and if that byte is nonzero branches forward; otherwise steps a0 back and branches elsewhere.
- `compare_counted_bytes_until_mismatch`: Loads a0 and a1 from the stack and a count from the stack into d0, then compares bytes from a1 against a0 one at a time, returning zero in d0 when a terminating zero byte matches.
- `return_one_or_minus_one_by_flags`: Sets d0 to one, branches forward if the condition flags indicate greater, otherwise sets d0 to minus one and returns to the caller.
- `scan_string_until_nul_or_count`: Loads a0 and a1 from the stack, loads a count into d0, then advances a0 past bytes until a zero byte or the count runs out, returning the remaining count in d0.
- `decrement_a0_and_call_copy`: Subtracts one from a0 and calls the copy routine at the following address, then returns to its caller.
- `copy_bytes_until_nul_or_count`: Copies bytes from a1 to a0 while decrementing d0, stopping at a zero byte or when the count is exhausted, then pads with a zero byte if space remains.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `init_icon_linked_list_header`: Loads the caller's pointer argument from the stack into a0, then stores a0 at its own address, adds four to that field, clears the next field and stores a0 in the third field before returning.
- `draw_icon_with_offsets_and_planes`: Saves registers, checks the icon pointer, calls a library helper with a size of twenty, computes rounded width and height offsets, calls a scaling helper twice, then calls the helper again with the icon's offset field.
- `iterate_icon_list_drawing_entries`: Saves registers, walks a linked list of icon entries, calling a library helper with a size of sixteen and a second helper with a byte-derived offset, advancing through the next pointer until a call fails or the list ends.
- `return_one_then_branch_to_shared_exit`: Loads the constant 1 into d0 and branches to the shared exit code of the neighbouring library routine, leaving the stack untouched.
- `return_zero_restore_d2_d5_a2`: Sets d0 to zero as the return value, restores d2 through d5 and a2 from the stack, then returns to the caller.
- `allocate_and_fill_icon_bitmap`: Saves registers, allocates two memory blocks via library calls, computes a rounded even byte width from the icon header, fills the bitmap with two drawing calls and returns the first block or minus one.
- `icon_return_minus_one_restore_regs`: Sets d0 to minus one as the return value, restores d2 through d6 and a2 from the stack, then returns to the caller in icon.library.
- `icon_read_chunk_list_into_nodes`: Saves registers, reads a chunk header and data through library calls, links allocated nodes via offset twelve, and returns the first node or minus one on failure.
- `icon_allocate_and_read_chunk_data`: Stores the next node pointer, allocates a buffer sized from the chunk byte count, reads chunk data into it, and loops back to process the following node.
- `return_a3_in_d0_then_branch`: Moves the longword in a3 into d0 as the return value, then branches forward to another part of the library code.
- `return_minus_one_restore_registers`: Sets d0 to minus one as the return value, restores d2 through d7 and a2 through a3 from the stack, and returns to the caller.

### diskfont.library

- `diskfont_library_entry_branch`: Library entry point that branches forward to the startup code at offset eighty, which then calls library base functions through a6.
- `open_diskfont_assign_and_set_node`: Saves a0/a2, calls the library open routine with a name and version, stores a0 at offset 34 of the returned node, then calls the close routine and returns 1 on success.
- `open_dos_and_graphics_libraries`: Stores the base and a6 in globals, opens dos.library and graphics.library via exec.OpenLibrary, initializes a list header at offset 42, and returns the base or zero on failure.
- `close_libraries_and_free_library_base`: Closes the library stored in the global, computes the base from a6 minus the library structure size, frees that memory via exec.FreeMem, and returns zero.
- `close_diskfont_and_free_resources`: Walks the library's open-resource list, unlinks each entry with no users, calls UnLoadSeg on its segment, unlinks its message port, decrements the open count, then frees the library base and returns its value.
- `expunge_diskfont_library_if_unused`: If the library has no open count and no users, closes its two resource libraries, frees the library base memory, unlinks the base from the library list and returns the saved base pointer; otherwise sets the expunge flag and returns zero.
- `set_expunge_flag_and_return_zero`: Sets bit 7 of the library flags byte at offset 14 from a6 and returns zero in d0, marking the library for expunge.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `clear_lib_byte7_and_bump_open_count`: Clears bit 7 of the byte at offset 14 in the library base a6, increments the word at offset 38, then returns the library base in d0.
- `decrement_open_count_and_maybe_expunge`: Decrements the word at offset 38 in the library base a6; if it reaches zero and bit 7 of the byte at offset 14 is set, jumps to the expunge vector at offset -18 of a6.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `build_font_list_from_library_chain`: Library code that scans the font list at offset 140 of the library base, copies font names and attributes into the caller's buffer, and optionally enumerates disk fonts, returning the count.
- `advance_font_pointer_and_skip_entry`: Subtracts the glyph width in d4 from the running offset at -4(a6) and from d2, then if d2 is still non-negative loads the next font entry pointer from 10(a0) into a1 and branches back to the copy loop.
- `copy_glyph_bytes_into_buffer`: Copies one byte from (a1)+ into the buffer at offset d3 from -4(a6), increments d3, and when d3 exceeds the width in d4 writes a type-1 entry with the buffer pointer, width and height into the a2 record and advances a2 by ten bytes.
- `build_font_list_from_fonts_directory`: When bit 1 of d1 is set, opens the FONTS: directory, scans its entries for .font files, reads each font header, and appends type-1 and type-2 records to the caller's list, returning the remaining space in d0.
- `adjust_bitmap_pointer_and_remaining`: Subtracts the word in d4 from the long at -4(a6) and from d2, then if d2 is non-negative sets a1 to a3+8 and d3 to zero before branching.
- `copy_glyph_bytes_to_bitmap`: Copies bytes from (a1)+ into the bitmap at -4(a6) offset by d3, incrementing d3 until it exceeds the word in d4, then reads four bytes from d6 into -12(a6).
- `read_glyph_record_and_store_entry`: Reads a 260-byte record from d6 into -272(a6), and if successful subtracts 10 from d2 and writes a 10-byte entry at a2, incrementing the word at a4.
- `negate_d2_or_zero_after_two_calls`: Calls two library routines with a6-relative and d5 arguments, calls a third with a3 and a word constant, then returns the absolute value of d2 or zero if d2 is non-negative.
- `return_zero_and_restore_registers`: Sets d0 to zero, restores d2 through d7 and a2 through a5 from the stack, releases the a6 frame and returns to the caller.
- `compute_weighted_font_match_score`: Reads two font structures, compares a word field and flag bytes, accumulates weighted differences into d5 and returns the resulting match score in d0.
- `kerning_pair_bit_test_adjust`: Reads two kerning flag bytes from the font and glyph structures, tests bit one, and subtracts a table entry from d5 when the bits differ.
- `kerning_second_byte_bit_adjust`: Subtracts a kerning table entry from d5, then tests bit one of the second flag byte and subtracts another entry when the bits differ.
- `kerning_loop_tail_advance_bit`: Subtracts a kerning table entry from d5, doubles d3, increments d6, and returns d5 in d0 once d6 exceeds seven.
- `return_error_code_minus_32767`: Loads the constant value minus thirty-two thousand seven hundred sixty-seven into d0 as the return value, restores registers d2 through d6 from the stack and returns to the caller.
- `open_font_file_and_load_font`: Copies eight bytes of a name, opens the file, loads the font, reads its header fields, and allocates a font structure, returning the loaded font pointer or zero.
- `load_font_and_attach_to_list`: Opens the font file, reads the header, allocates and fills a font structure, links it into the font list and returns the new font pointer.
- `reset_font_cache_counters`: Clears the word at offset -14 from a6, sets d3 to 255, zeroes d7, copies d7 into a2 and branches onward to the shared continuation.
- `scan_and_activate_font_entry`: Scans font entries indexed by a2, compares returned scores against the best at -14(a6), then activates the chosen entry by calling library routines and updating its counters and flags.
- `clear_font_entry_flag_word`: Pushes a0, calls a library routine, computes the entry address from d3 times 260 plus d5, clears its word at offset 256 and branches to the shared exit.
- `clear_font_entry_and_release_resources`: Clears the word at offset 256 of the font entry indexed by d3, then for negative d3 releases several library resources and returns the value at a6-4.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
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
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `copy_null_terminated_string`: Copies bytes from the source pointer at stack offset 8 to the destination pointer at stack offset 4 until a zero byte is copied, then returns.
- `scan_string_for_nonzero_byte`: Saves d2, loads the string pointer from the stack, tests its first byte and advances a0 while bytes are nonzero, branching back to recheck.
- `call_pc_relative_then_restore_d2`: Pushes d2 and a0 onto the stack, calls the routine at the current program counter, then discards both saved registers, restores d2 and returns.
- `count_bytes_until_null`: Loads the pointer from the stack into a0, sets d0 to zero, then scans bytes at a0 incrementing d0 until a zero byte is found, returning the count in d0.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `compare_strings_until_mismatch_or_null`: Loads two string pointers from the stack, compares bytes at a0 and a1, advancing both, and returns zero in d0 only if the strings match through the terminating null.
- `scan_backward_for_matching_byte`: Reads a byte from the address in a0, decrements a1 and copies it to a0, compares the new byte with the saved one, and returns 255 in d0 if the earlier byte is greater.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_first_byte_with_argument`: Loads a pointer from the stack into a0 and a byte argument into d0, reads the first byte into d1, and returns the pointer in d0 only if that byte is nonzero and equals the argument.
- `advance_pointer_and_branch_back`: Adds one to the address in a0 and branches back to the earlier comparison code, continuing the byte search in the diskfont library.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `find_last_byte_in_buffer`: Saves d2-d3, takes a buffer pointer and a byte value, calls a length routine, then scans backward from the last byte for a match, returning its address or zero.
- `decrement_pointer_and_continue_scan`: Subtracts one from address register a0 and branches back into the backward byte scan loop to test the previous byte.
- `return_zero_restore_d2_d3`: Sets d0 to zero as the not-found return value, restores d2 and d3 from the stack and returns to the caller.
- `open_font_try_two_tags`: Saves d2-d3, loads the font pointer from the stack, calls the open routine with tag 47 then tag 58, and returns the pointer if both calls fail.
- `return_saved_d3_result`: Moves d3 into d0 as the return value, restores d2-d3 from the stack and returns to the caller.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `library_entry_jump_vector`: Saves d0-d1 and a0 on the stack, jumps through the library vector at address zero, then discards the saved registers and returns.
- `diskfont_call_through_vector_and_return`: Pushes a0 onto the stack, calls the routine at vector offset 224, then discards the pushed longword and returns to the caller.

### Amiga Karate.adf:  AMIGA KARATE

- `karate_program_startup_and_main_loop`: Saves registers, opens the current directory and a file, allocates memory, sets up the display, then runs the main game loop and exits.
- `karate_shutdown_close_and_reply`: Restores the saved stack pointer, closes the opened library, forbids and replies the startup message, then returns the exit code.
- `open_and_close_workbench_library_2`: Loads the workbench library by calling the library open vector with a name at offset 92 of a4, then calls the close vector on the same name and returns.
- `open_dos_library_store_base`: Clears the dos library base at absolute address 322, loads a name pointer into a1, calls the open library vector with d0 set to 31, stores the returned base and returns.
- `load_karate_data_file`: Allocates 10000 bytes, opens a file, reads a name string and count, fills a table of pairs, copies bytes to the caller buffer, then frees memory and returns.
- `read_signed_16bit_value`: Reads two bytes via the byte-read routine, combines them into a 16-bit value in d0, sign-extends bit 15, and returns the result.
- `combine_two_random_bytes`: Calls the random byte routine twice, shifts the second result left by eight bits, ORs it with the first, and returns the combined value in d0.
- `clear_global_at_80c`: Writes zero to the longword at memory address 80c and returns to the caller.
- `store_99_at_804`: Loads the immediate value 99 into d0, stores that longword at memory address 804, and returns.
- `decrement_counter_or_fetch_next_value`: If the countdown at a fixed global is positive, decrements it and returns the stored value; otherwise calls the fetch routine, stores its result, and returns it, resetting the countdown to 30000 when the result is minus one.
- `decode_huffman_bitstream_value`: Advances a bit counter, shifting a stored bit value right and indexing a table by the low bit to walk a tree, refilling from the fetch routine when the counter exceeds seven, and returns the decoded value or minus one.
- `refill_buffer_and_read_next_byte`: When the read pointer equals the end pointer, calls the refill routine with two buffer pointers and a count of 10000, resets the read pointer, then returns the byte at the read pointer and advances it by one.
- `get_slot_pointer_or_set_error`: Checks a global flag and calls a helper, then validates the slot index against a global count and returns a pointer into the slot table, or sets error code 9 and returns null.
- `allocate_slot_and_open_file`: Scans the slot table for an empty entry, builds flag bits from the mode argument, optionally deletes the file, opens it with the chosen access mode and stores the file handle in the slot.
- `open_slot_with_forced_flags`: Combines the caller's mode with forced flag bits, clears the high mode bit, pushes the adjusted arguments and tail-calls the slot allocation and file open routine.
- `read_from_file_handle`: Looks up a file handle from the first argument, calls Read on its descriptor with the buffer and length, and returns the byte count or minus one on error.
- `open_file_handle_and_check_status`: Checks the length argument is non-negative, calls the open helper on the first argument, stores the returned handle, and on failure sets the error code to twenty-two and returns minus one.
- `flush_then_write_to_file_handle`: Checks the file handle flags, optionally flushes with a seek, then calls Write on the descriptor with the buffer and length, returning the byte count or minus one.
- `seek_file_handle_by_slot`: Looks up a file handle from the argument, seeks to an offset chosen by the slot index, and returns the resulting position or minus one on failure.
- `seek_to_indexed_offset_then_rewind`: Reads a table entry indexed by the word at offset 16 of a6, seeks the file handle to that offset, and on failure stores the error code and sets error 22, then seeks back to zero.
- `close_file_if_not_already_closed`: Looks up a file record from the argument, returns minus one if missing, returns zero if its flag bit 2 is set, otherwise closes the handle, records any error with code 5, clears the flag byte and returns the status.
- `delete_file_and_record_error`: Calls the delete-file routine on the path argument, and if it fails stores the error code and sets error 2 before returning minus one, else returns zero.
- `return_zero_after_delete_success`: Sets d0 to zero, releases the a6 stack frame and returns, signalling successful file deletion to its caller.
- `open_file_and_store_handle`: Pushes open mode 1005 and a path pointer, calls Open, then stores the returned file handle in a global and returns.
- `read_byte_or_cached_char`: Checks two global flags, calls setup routines if needed, then returns a pending byte from a global buffer or reads one byte from the file handle into a stack slot.
- `read_one_byte_from_handle`: Calls the Read library function to read one byte from the global file handle into a stack buffer, then returns that byte in d0.
- `write_byte_to_handle`: Checks global flags, calls setup routines if needed, then calls the Write library function to write one byte from the stack argument to the global file handle and returns it.
- `_SetIntVector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `_AddIntServer`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_FindTask`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `_SetTaskPri`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `_SetSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `_AllocSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `_FreeSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `_AddPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `_RemPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `_OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `_OpenDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `_DoIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `game_setup_open_libs_alloc_bitmaps`: Opens intuition library, closes workbench, lowers task priority, allocates a large memory block, initializes five bitmaps and calls many setup routines before returning.
- `show_error_and_cleanup`: Displays an error message twice via the message routine, calls an indirect handler, then frees a null pointer through the memory routine and returns.
- `load_form_and_decompress_sprite_data`: Opens a file, checks the FORM header, reads sprite dimensions, decodes packed nibble data into a buffer, allocates 10000 bytes and unpacks run-length sprite rows into it.
- `refill_buffer_and_fetch_next_byte`: When the read counter reaches 10000, refills the buffer from the file handle and resets the counter, then returns the next byte from the buffer and advances the counter.
- `multiply_d0_by_d1_32bit`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit multiplies, returns the low 32-bit product in d0, and returns zero if d0 is zero or d1 is zero.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `call_zero_arg_handler_via_pea`: Pushes a zero longword onto the stack, calls the routine twenty bytes ahead, then discards the pushed argument and returns.
- `call_zero_arg_handler_far`: Pushes a zero longword onto the stack, calls the routine four hundred and four bytes ahead, then discards the pushed argument and returns.
- `wait_tof_load_colors_twice`: Waits for vertical blank, loads thirty-two colors from the argument, toggles a flag, looks up a table entry and copies a longword into the copper list, repeating the sequence twice.
- `decrement_matching_nibbles_in_table`: Loops a counter from 15 down to 1, scanning 32 words of a global table and subtracting 1, 16 or 256 from each word whose low, middle or high nibble equals the counter, then calls the table refresh and a delay routine.
- `clear_table_and_accumulate_nibble_counts`: Zeroes 32 words of a global table, then for each counter from 15 down to 1 scans 32 words of the passed table and adds 1, 16 or 256 to the global word when its nibble is at least the counter, refreshing between passes.
- `count_nibbles_above_threshold`: Scans 32 words of the passed table and, for each word whose low, middle or high nibble exceeds the passed threshold, adds 1, 16 or 256 to the corresponding global table word, then refreshes the table.
- `wait_for_display_frames`: Loops the given count from the stack frame, calling the vertical blank wait each iteration, then releases the frame and returns.
- `clear_32_word_global_table`: Zeroes 32 consecutive words of the global table by looping an index from 0 to 31, then passes the table address to the table routine and returns.
- `_BltBitMap`: Saves registers, loads bitmap arguments from the stack, fetches the graphics library base and jumps to its blit entry, then restores registers.
- `_LoadRGB4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `_InitRastPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_InitVPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `_MrgCop`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_MakeVPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `_LoadView`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `_WaitBlit`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `_WaitTOF`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `_QBlit`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `_BltClear`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `_InitView`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `_InitBitMap`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `signed_32bit_multiply`: Multiplies d0 by d1 as signed 32-bit values using unsigned partial products, restores d1-d4 from the stack and returns the product in d0.
- `_Open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `_Close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `_Read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `_Write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `_Seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `_DeleteFile`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `_Rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `_IoErr`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `check_signal_bits_and_play_sound_sequence`: Reads and clears the 12288 signal bits, returns zero if none set, otherwise sends sound commands 94, 68 or 67, 10, then each index up to a global count, and finally calls a routine with two zero arguments.
- `build_double_buffer_view_and_bitmaps`: Opens graphics.library, initializes a view, rastport and viewport, creates two bitmaps of five planes each with 320 by 200 size, fills their plane pointers from the argument, merges and loads the view, waits for blit and vertical blank, then returns the advanced pointer.
- `load_view_and_set_play_state_one`: Waits for blit, loads a view, waits for time-of-flight, copies a pointer into the copper list area, sets two pointers to a data block and stores 1 in the play state.
- `load_view_and_clear_play_state`: Waits for blit, loads a view, waits for time-of-flight, copies a pointer into the copper list area, sets two pointers to a data block and clears the play state to zero.
- `clear_all_bitmap_planes_with_bltclear`: Loops over two bitmaps and their five planes, calling the blitter clear function on each plane pointer for 8000 bytes with flag zero, then waits for the blitter to finish and returns.
- `clear_five_bitmap_planes_and_wait`: Loops five times, calling a lookup routine with the byte argument to get a base pointer, then clears 8000 bytes at each of the five plane pointers and waits for the blitter.
- `get_bitmap_base_from_resource`: Reads the longword at absolute address 1070, calls the resource lookup with index 44, adds the fixed offset 970 to the returned pointer and returns it in d0.
- `blit_full_screen_bitmap_and_wait`: Calls the bitmap blit with source and destination pointers and fixed dimensions, reports an error if the result is not five, then waits for the blitter to finish.
- `blit_selected_bitmap_region_and_wait`: Looks up a base pointer from a global long, adds the plane offset, blits a fixed region from a source pointer, reports an error if the result is not five, then waits for the blitter.
- `blit_screen_with_error_check`: Calls an indirect routine to get a value, performs a bitmapped blit with specific dimensions and colors, and if the returned value is not five, calls another routine before waiting for the blit to complete.
- `load_and_show_screen_by_index`: Loads a screen file for the given index, displays it, optionally runs index-specific setup, waits 150 frames if the index is unchanged, then restores the previous screen and toggles a flag.
- `blit_screen_with_inverted_check`: Calls an indirect routine twice, performs a bitmapped blit using the results to determine the source and destination, and then waits for the blit to complete.
- `store_selected_menu_value`: Reads a global byte, subtracts one and shifts right twice to index a word table, then writes the selected word into offset 10 of the caller's structure, or zero if out of range.
- `insert_sorted_highscore_entry`: Scans eight highscore records for the first whose score is below the given value, shifts lower records down by sixteen bytes, stores the new score and name, and returns zero.
- `high_score_name_entry_loop`: Draws the high-score entry screen, reads keyboard scancodes, moves a cursor over the letter grid, appends chosen characters to the name buffer and waits eight frames per keypress.
- `draw_high_score_entry_panels`: Waits for vertical blank, draws the score, grade and name panels plus the cursor character at computed screen positions, then waits again before returning.
- `display_master_warriors_table`: Clears a buffer, draws the master warriors score table with names, grades and scores, waits six hundred frames, then restores the display and returns.
- `draw_grade_belt_congratulation_screen`: Reads the player's belt index from global memory, clamps it, stores the belt word and grade number, then draws congratulation text lines and belt graphics via text routines before saving the grade.
- `store_pointer_and_return_offset_address`: Saves the incoming pointer argument to global memory and returns that pointer advanced by 8200 bytes in d0.
- `set_digit_char_and_init_text_cursor`: Converts the argument to an ASCII digit, stores it, calls the text setup routine, sets two display pointers to the saved pointer plus four, and sets a counter to ten.
- `advance_frame_counter_and_reload_anim`: Compares the frame counter at a long global with 10; if equal it clears the counter and sets the animation pointer to the base pointer plus 4, then calls the animation loader with 4000 and the pointer, stores the result, calls the frame update, and increments the counter.
- `step_animation_frame_and_trigger_sound`: Reads the frame index word; when it reaches 31 it increments the loop counter word, otherwise it calls the frame advance, plays a sound with volume 15 minus the index when below 16, and reloads the animation pointer when the index exceeds 11, then increments the index.
- `silence_audio_channels_and_reset_dma`: Loads the custom chip base into a0, clears the volume registers for audio channels 2 and 3, writes 12 to the DMA control register to enable audio DMA, clears a long global, and returns.
- `setup_audio_double_buffer_playback`: Stores the sample pointer and half-length into the audio channel structure, sets both channel pointers to the same buffer, then jumps to the playback start routine with the parameters.
- `start_audio_channel_zero_or_one`: Checks the mute flag, then programs AUD0 or AUD1 length, pointer, period and volume, enables the matching DMA channel and stores the length in the channel-active word.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `clear_audio_done_flag_and_mark_channel`: Tests the word at a1, and if zero writes one to offset a4 of a0, clears the word at a1 and branches past the skip.
- `set_state_one_and_wide_span`: Writes the value 1 to offset 150 and the value 128 to offset 156 of the structure pointed to by a0, then returns.
- `check_flag_then_set_state_one`: Tests the word at a1; if zero it branches past, otherwise writes 1 to offset 180 of a0, clears the word at a1 and branches on.
- `set_state_two_and_wide_span`: Writes the value 2 to offset 150 and the value 256 to offset 156 of the structure pointed to by a0, then returns.
- `set_animation_frame_and_flags`: Saves a2 in d0, loads a2 with a fixed data pointer, copies a longword and word from that data into offsets 192 and 196 of a0, restores a2, and writes 255 and 512 to offsets 158 and 156 of a0.
- `advance_animation_pointer_and_store_frame`: Swaps a2 with the pointer at a1, compares it to a global, optionally increments a counter, checks a magic value, writes frame values to a0 offsets 216, 200, 208, 212, 158 and 156, updates globals, and restores a2.
- `zero_filled_data_area_4`: This single zero word decodes as an ori.b immediate instruction, but the bytes are a zero-filled data area referenced by address, not executable code.
- `zero_filled_data_area_5`: Four zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `open_and_configure_request_window`: Checks the stack against a limit, opens a resource via a helper, fills a request structure with fixed values, calls two helpers to set it up and release it, returning the result or minus one.
- `set_global_char_and_dispatch_request`: Returns zero if a flag byte is set; otherwise copies a pointer, stores the argument plus thirty as a character, and calls a request routine with the pointer and extra argument.
- `dispatch_animation_frame_by_index`: Reads the animation index argument, loads the animation base pointer from global memory, and calls the frame drawing routine with different width, height and offset values for indices one through six.
- `accumulate_twelve_offsets_and_set_base`: Loops twelve times adding the incoming value to itself and storing each result in a global offset table, then stores the second argument into two global base pointers and returns the accumulated value.
- `build_twelve_entry_letter_table`: Loops twelve times writing sequential letter characters into a global buffer and calling a routine with each table entry and offset, then calls a final routine before returning.
- `stack_check_return_zero`: Compares the stack pointer a7 against the stack limit at address 278; if below it jumps to the trap vector, otherwise sets d0 to zero and returns.
- `clear_sound_channel_and_play_sample`: Clears word at offset six of the sound structure pointed to by the global sound pointer, then calls the sample player with that structure, its word at offset two, and two zero arguments.
- `stack_check_return_zero_alt`: Compares a7 with the stack limit at address 278 and jumps to the trap vector if below, otherwise loads zero into d0 and returns to the caller.
- `stack_limit_check_then_jump`: Compares the stack pointer a7 against the value at absolute address 278 and jumps to address zero if a7 is below it, otherwise falls through.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `stack_check_return_zero_2`: Compares a7 against the stack limit at absolute address 278, jumps to address zero if below, otherwise sets d0 to zero and returns.
- `select_sound_slot_and_play`: Checks the stack limit, picks a sound slot from a random value masked to two bits, optionally offsets it, clears a word in the chosen structure and calls the sound routine with its fields.
- `clear_second_sound_channel_and_play`: Clears word at offset six of the second global sound structure, then calls the sample player with that structure, its word at offset two, and two zero arguments, returning zero.
- `pick_random_sound_slot_and_play`: Gets a random value masked to two bits, adds six or eight depending on a flag byte, indexes the sound pointer table, clears the chosen structure's word at offset six and plays it with a flag of one.
- `return_zero_after_stack_check`: Compares the stack pointer against the global stack limit and jumps to the trap routine if below it, otherwise sets d0 to zero and returns immediately.
- `load_pictures_and_run_animation_loop`: Sets up a stack frame, calls several unnamed routines to load picture data, waits for vertical blank, then loops over four picture slots copying pointers and calling a blit routine with the slot data.
- `read_random_byte_below_90`: Calls a routine, masks the result to a byte, stores it in a local, and repeats while the value is 90 or greater, returning the byte in d0.
- `call_routine_with_66_argument`: Pushes the immediate value 66 onto the stack, calls a routine, removes the argument, and returns to the caller.
- `store_five_offsets_and_draw_digits`: Loops five times storing the incoming value into a global offset table, adds each stored value to the running total, writes a digit character to a global byte, and calls drawing routines for each entry before returning the accumulated total.
- `reset_sprite_and_init_animation`: Loads the global object pointer, clears the word at offset six, then calls the animation setup routine with the object's word at offset two and zeroed arguments.
- `return_pointer_to_picture_data`: Loads the address of a picture data block into a0, copies it into d0 as the return value, and returns to the caller.
- `get_pointer_to_global_3f6`: Loads the address of the longword at memory offset 1014 into a0, copies it to d0 as the return value, and returns.
- `install_four_interrupt_vectors`: Fills four interrupt vector structures with mode bytes, data pointers and handler addresses, then registers each structure for interrupt levels seven through ten.
- `enable_disk_dma_channel`: Loads the custom chip base address into a0 and writes 32 to the DMACON register at offset 150, enabling the disk DMA channel, then returns.
- `install_audio_pointer_vectors`: Loads the address of a data block into a0 and stores it to two low-memory vector slots, then writes a fixed magic longword to another low-memory slot before returning.
- `build_three_channel_sample_headers`: Saves registers, walks three sample descriptors from the caller's table, fills each channel header with length, pointer and next-header link, copies ten bytes of a second table, then programs the audio and DMA registers.
- `program_two_audio_channels_and_dma`: Writes channel two and three period, volume, length and pointer registers from a0, d5, d6 and d7, delays briefly, then enables audio DMA and interrupt bits before restoring registers.
- `setup_two_audio_channels_for_sample`: Configures Paula audio channels 0 and 1 with the sample pointer, half-length word count, period 300/301, volume 64, and enables DMA and audio interrupts.
- `decode_form_iff_bitplanes_to_chunky`: Reads a FORM chunk header, decodes RLE-compressed bitplane data into chunky pixel buffers using the supplied palette and dimensions, and returns zero on success.
- `copy_bytes_from_stream_offset`: Copies a requested count of bytes from the source pointer plus a global stream offset into the destination buffer, advancing the global offset and returning the count.
- `blit_masked_glyph_into_bitplanes`: Allocates two bitplane buffers on first call, then for each of 81 rows reads a byte from the font data, computes source and destination offsets, and combines masked font bits into four destination bitplane pointers.
- `zero_filled_bitplane_buffer_pointer`: This address holds a zero word that decodes as an empty instruction; it is a zero-filled data slot used as a pointer to a bitplane buffer, not executable code.
- `zero_filled_source_buffer_pointer`: This address holds a zero word that decodes as an empty instruction; it is a zero-filled data slot used as a pointer to a source buffer, not executable code.
- `advance_pseudo_random_and_scale`: Reads a 16-bit seed from memory, computes seed times 4 minus seed times 3 plus 1, stores it back, and if nonzero divides 65535 by the span between two arguments to return a scaled value.
- `select_animation_frame_pointer`: Tests flag bits in the first argument and a table byte to choose a frame index, stores a pointer to one of three frame tables at the third argument, and writes the index plus 15 to a byte in memory.
- `_NewList`: Takes a list header pointer from the stack, sets its head to point just past itself, clears the tail pointer and points the end marker back at the header.
- `open_input_device_and_send_request`: Clears two globals, opens the input device, stores the returned IO request pointer, sets its command word to 9 and its data pointer to a global buffer, then sends the request and returns.
- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `_L10`: Allocates a signal and a 34-byte message port, fills its fields with the task, name and signal, then adds it as a port or initializes its list and returns the port pointer.
- `_L16`: Removes the message port if it was added, marks it closed, frees its allocated signal and releases the 34-byte port memory back to the system.
- `_L7_2`: If the passed pointer is nonzero, allocates 48 bytes of chip memory, marks byte 8 as 5, clears byte 9, stores the pointer at offset 14 and returns the new block, else returns zero.
- `_L11_2`: Marks byte 8 of the passed structure as 255, sets the two longwords at offsets 20 and 24 to minus one, frees the 48-byte block and returns.
- `init_fifteen_karate_slots`: Calls a setup routine with the argument, stores it in a global, then loops fifteen times allocating 30-byte entries and writing -1 into two word fields of each.
- `update_sprite_position_and_flip`: Computes a sprite record from the index and writes or adds x, y and frame bytes, toggling the horizontal flip bit when the frame byte is minus three, and returns the frame byte in d0.
- `set_sprite_absolute_and_anim`: Calls the position updater with absolute mode, stores the frame byte, reloads animation data when the animation index changes, resets the frame counter and calls the animation stepper.
- `set_sprite_relative_and_anim`: Calls the position updater in relative mode, stores the returned frame byte, reloads animation data when the index changes or the frame exceeds one hundred twenty-seven, then calls the animation stepper.
- `update_actor_animation_and_position`: Reads five stack arguments, calls a helper, indexes a 30-byte actor record, optionally reloads animation frames from a pointer table, calls two subroutines, adds deltas to two record words and returns.
- `dispatch_actor_sound_by_state`: Rejects actor indices above one, checks the state word against a list of values, then calls either the plain sound routine or the sound routine with channel 66 or 67.
- `clear_actor_animation_fields`: Takes an actor index, computes its 30-byte record offset, and writes minus one into the two words at offsets 20 and 24 before returning.
- `advance_frame_counter_and_blit_background`: Calls a setup routine, increments a frame counter, triggers sound effects on certain key values, queues a blit with a local routine pointer, waits for vertical blank or a timer, then queues the main blit and returns.
- `release_three_slots_then_ten_more`: Loops over three byte slots and then ten more, looks up each object via a table call, and releases it when its status word is zero.
- `release_slots_three_through_twelve`: Loops from slot three up to twelve, looks up each object via a table call, and releases it when its status word is zero.
- `return_zero_with_frame_restore`: Sets d0 to zero as the return value, releases the a6 stack frame and returns to the caller.
- `resolve_two_offsets_and_call_compare`: Converts two arguments through a lookup, adds a fixed offset to each pointer, calls a comparison routine with them, and frees the result when the first argument is zero.
- `relocate_twelve_pointers_and_copy_tables`: Stores the base pointer, adds twelve successive offsets to it, then copies twenty-four and eighteen bytes from source tables into the relocated memory and returns the end pointer.
- `setup_blitter_sprite_for_animation_frame`: Reads sprite dimensions and frame index from the object, looks up width/height and image pointer tables, then programs the blitter registers and waits for blits to finish.
- `check_hit_and_place_impact_sprite`: Computes attacker and defender screen positions, tests horizontal distance against a threshold, calls the collision check, and stores the impact sprite coordinates in the sprite table.
- `play_impact_sound_for_sprite_slot`: If the sound lock is clear, marks the slot as busy, picks one of two sound routines based on the slot table, and calls it with the sprite coordinates and volume.
- `copy_and_or_bitplane_rows`: Reads source and destination pointers plus width and height from the structure pointed to by d0, copies the first row of words to a scratch area, then ORs three further rows into it and returns.
- `blit_sprite_with_clipped_offsets`: Allocates a chip buffer, computes clipped horizontal and vertical offsets from the sprite structure, programs the blitter registers at custom chip base and waits for blits to finish.
- `clip_and_program_blitter_shifted`: Clips the sprite x coordinate against the screen width, computes the shift and modulo values, writes blitter control and data registers and adjusts the source pointer for the negative-offset case.
- `build_sprite_strip_from_actor_table`: Reads actor fields from a1, computes a sprite pointer offset into the global table, writes sprite control words and position values into the a3 structure, then loops over d0 entries waiting on the blitter busy bit.
- `terminate_sprite_list_and_restore`: Writes minus one to the sprite control word at offset 68 of a3, restores registers d1 through d7 and a0 through a4, releases the a6 frame and returns.
- `build_shifted_sprite_strip_entry`: Like the strip builder but offsets the row index by one and subtracts two from the doubled sprite width, writing the shifted sprite pointer and control words into the a3 structure before jumping into the shared blitter wait loop.
- `jump_to_playfield_bitmap_setup`: Jumps through the absolute long address at location 88, transferring execution to the playfield and sprite hardware setup code without returning here.
- `clear_three_input_words`: Sets d0 to zero and writes zero to three consecutive word locations in low memory, then returns to the caller.
- `read_two_joystick_ports_combine`: Reads CIAA_PRA and both JOY0DAT and JOY1DAT, converts each port pair through a shared decode call, stores two words and returns the first plus the second shifted left eight.
- `build_karate_input_flag_word`: Reads two parameter bytes from the stack frame, sets bits in a local word for direction and button flags, and returns that word in d0.
- `run_karate_main_game_loop`: Initializes display and game state, then repeatedly calls input, update and render routines, toggling a frame counter between zero and one.
- `load_four_args_and_jump_to_table`: Saves registers, copies four longword arguments from the stack frame into d1 through d4, loads a fixed address into a0 and branches away.
- `draw_text_with_wrap_offset`: Pushes four arguments, adjusts the y coordinate by adding 10000 when below that threshold and selects one of two string pointers, calls the text drawing routine and then the character renderer, restoring registers and returning.
- `render_string_advance_cursor`: Reads bytes from the string in a0, calls the glyph renderer for each non-zero byte, advances d0 by 8 and wraps to the next line by adding 9 to d1 when d0 exceeds 312, returning when a zero byte is found.
- `draw_character_glyph_at_cell`: Selects a glyph pointer from d2 by comparing against punctuation and letter codes, computes a screen offset from d0 and d1, allocates a bitmap, copies five plane pointers and calls the blit routine with a height from d6.
- `copy_masked_bytes_to_five_planes`: For each of d0+1 rows, reads up to five bytes from a0 according to bits 0-4 of d6, writes them to a1-a5 at offset d7, advances a0 by one and d7 by forty.
- `zero_filled_data_area_6`: Two zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `_CloseWorkBench`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `_AutoRequest`: Saves d2-d3 and a2-a3-a6, copies six longword arguments from the stack into a0-a3 and d0-d3, jumps through the library base at offset minus 348, then restores the registers and returns.
- `close_open_file_handles_and_call_cleanup`: Iterates the global file table, closes each entry whose flag byte is nonzero and lacks bit 2, then calls the cleanup routine with the two stack arguments.
- `process_bit_table_and_pointers`: Loops through a table of 256 bytes, performing bitwise rotations on each byte, and then iterates through a list of pointers to add the base address of the list to the value pointed to by each entry.
- `blit_tile_rows_through_palette_lookup`: For a tile index below 3, selects a 2400-byte buffer, maps each source byte through a 256-byte palette table into rows sized by the structure's width and height fields.
- `zero_filled_data_area_7`: A single zero word that decodes as ori.b #0,d0; the bytes are data, not executable code, referenced by address from other code.
- `blit_sprite_to_custom_chip_ram`: Saves all registers, waits for the blitter, computes a destination pointer, then programs BLTCON0, BLTCON1, BLTDPTH, BLTDMOD and BLTSIZE to blit a sprite and restores registers.
- `masked_sprite_blit_with_dma_setup`: Saves registers, computes source and destination offsets, sets BLTCON0, BLTCON1, BLTAPTH, BLTAMOD, BLTDPTH, BLTDMOD and BLTSIZE, enables DMA, ANDs seven source rows with a mask and returns the combined result.
- `save_regs_call_8a6_and_restore`: Builds an a6 stack frame, saves d1-d7 and a0-a6, loads the longword argument at offset 8 into d0, calls the routine at the next address, then restores the registers and returns.
- `toggle_sound_and_play_twice`: If a flag byte is clear, adds a table value indexed by a byte to a word table entry, calls the sound routine twice with that value while flipping a hardware toggle bit between calls, then restores a0-a1.
- `zero_filled_data_area_8`: Three zero words that decode as ori.b instructions; this is a zero-filled data area referenced by address, not executable code.
- `decompress_four_plane_tile_data`: Calls a routine to get a base pointer, then for four planes copies a run-length encoded byte stream into memory at an offset, expanding escape bytes into word offsets, and returns the advanced source pointer aligned to even.
- `install_level_five_interrupt_server`: Writes interrupt server node fields at absolute addresses 8, 9, 10, 14 and 18, then pushes the node and level 5 and jumps to the server-add vector at address 24.
- `run_title_screen_and_menu_loop`: Initializes display and sound, draws the title text and options, then loops reading joystick input to move the selection, updating graphics until the fire button is pressed or a timeout occurs.
- `branch_to_main_loop_entry`: Branches unconditionally to the main game loop entry point, transferring control without touching any registers or memory.
- `karate_match_main_loop`: Runs the fight loop: increments the frame counter, draws six sprite rows, reads input, updates the selected move index and calls the sprite blit routine, returning one when the match ends.
- `blit_sprite_tile_via_bltbitmap`: Builds a 40 by 32 temporary buffer, selects a tile by the byte argument, computes source and destination offsets, then calls the bitmap blit with 320 by 9 size and minterm 192.
- `init_player_slots_and_clear_state`: Clears the slot counter, then for each slot below the limit sets flag bytes in the player tables, zeroes a word, calls the slot allocator with type 3, and marks the returned record with value 6.
- `run_match_setup_and_input_loop`: Initializes the match state, builds the fighter list, draws each fighter, then polls keyboard and joystick input to update player actions and positions until the round ends.
- `format_word_as_decimal_string`: Converts the 16-bit value in the first argument to decimal digits in the buffer pointed to by the second argument, suppressing leading zeros and writing a terminating null byte.
- `parse_decimal_string_backwards`: Calls a length routine twice on the string argument, then walks backwards from its end accumulating digit characters into a decimal value in d0, returning it as a word.
- `copy_bytes_counted`: Copies a count of bytes from the source pointer at offset 8 to the destination pointer at offset 12, decrementing the count in d0 until it wraps, then restores a0-a1 and returns.
- `append_two_strings`: Copies the first string argument byte by byte into the destination pointer until a null terminator, then appends the second string argument the same way, restoring a0-a2 and returning.
- `fill_memory_with_byte_count`: Copies the byte argument into the buffer pointed to by a0, decrementing the count in d0 and advancing a0 until the count reaches minus one, then restores d1/a0 and returns.
- `strlen_count_until_nul`: Scans bytes from the pointer in a0, incrementing d0 from minus one until a zero byte is found, returning the string length in d0 and restoring a0.
- `init_intuition_request`: Opens a library, stores the handle in a global variable, performs an auto request with fixed dimensions, and then calls another routine with twenty as an argument.
- `increment_a1_long_and_return_zero`: Adds one to the longword pointed to by a1, sets d0 to zero as the return value, and returns to the caller.
- `reset_ten_slots_and_init_display`: Loops ten times calling a routine with the index, then reads a byte from a table, calls a routine with it, and sets up three display areas before clearing a flag byte.
- `update_fighter_state_and_animation`: Checks busy and input flags, advances counters and animation frames for the current fighter, triggers sound and delay calls, and returns minus one when a sequence finishes.
- `update_character_action_timers`: Loops over character slots, decrements active action counters, and when a slot is idle calls two subroutines to pick a new action byte and stores it in the slot's state table.
- `wait_for_input_nonzero`: Calls the input polling routine repeatedly until it returns a nonzero value, then returns zero in d0.
- `run_two_frame_update_steps`: Calls two update subroutines in sequence and returns to the caller, performing no other work itself.
- `init_player_heights_and_offsets`: Loops over eight player slots, computes a height from a modulo of the slot index, stores alternating plus or minus four offsets, clears slot flags and calls the sprite draw routine for each slot.
- `update_player_positions_and_bounce`: Loops over active player slots, draws each sprite with a direction-dependent character, adds its velocity to its position and reverses velocity when outside the vertical bounds, then advances the animation frame counter.
- `spawn_players_by_difficulty`: Chooses a spawn count from the difficulty byte, then scans empty player slots, tests each with a spawn check and marks newly spawned players while updating the active count and animation state.
- `find_max_high_score_entry`: Calls setup, then loops over the score table entries comparing each word against the stored maximum and updating it, finally calls teardown and returns.
- `wait_for_space_key_toggle`: Toggles a flag, prints the continue prompt, then polls the keyboard byte masked with 127 until the space bar is pressed and toggles the flag back.
- `display_round_result_screen`: Clears bitplanes, draws score and result graphics, prints the player's score digits, waits, then shows the continue prompt depending on the round and input state.
- `apply_hit_damage_to_fighter`: Subtracts damage from the selected fighter's health word, marks it defeated at zero, plays a hit sound, and draws the updated health bar via graphics calls.
- `blink_fighter_health_bar_twice`: Loops twice, calling the bar-drawing routine with palette and position values for the current fighter, toggling a global flag each pass.
- `draw_two_digit_number_on_bar`: Formats a value into two ASCII digit bytes and calls the text and bar drawing routines to render it at the given position.
- `update_fighter_state_machine`: Advances a fighter's animation and attack state from its script pointer, updating position, frame counters and state bytes in the fighter structure and returning a status flag.
- `compute_fighter_attack_result`: Compares two fighters' positions and facing flags to pick a hit table entry, then returns the resulting damage or reaction code from the table.
- `signed_32bit_divide`: Divides d0 by d1 using shift-and-subtract over 32 bits, returns quotient in d0 and remainder in d1, with sign handling and zero on zero divisor.
- `draw_karate_animation_frames`: Adjusts the frame counter, then either draws two animation frames via the draw routine while stepping the source and count by eight, or calls the frame helper with count four or three.
- `draw_repeated_animation_frames`: Computes a frame count from the argument, returns zero if the count is zero, otherwise repeatedly calls the draw routine advancing the pointer by five each iteration.
- `lookup_character_sprite_pointer`: Selects a character table based on the player index and flags, scans it for the given character byte, stores the index, and returns the sprite pointer.
- `swap_byte_in_global_table`: Reads a byte from a global table at the given index, finds the entry equal to the given byte, swaps the two bytes in the table, and returns.
- `mask_blit_columns_with_pattern`: Uses a pattern table indexed by color and row to AND-mask five consecutive 40-byte-spaced byte pairs in the destination bitmap pointed to by the first argument.
- `blit_masked_sprite_pair_to_bitmap`: Computes a bitmap offset from x and y, programs the blitter modulo and control registers, then calls the blit helper twice to draw two masked sprites.
- `init_sprite_control_block`: Stores two PC-relative pointers and per-sprite fields into the a3 control block, then waits on bit 6 of the status word while decrementing d3 and advancing a0.
- `restore_stack_and_chain_handlers`: Loads a7 from absolute address 334, jumps to address zero, then pushes immediate 20 and jumps to address 160, chaining to other handlers.
- `scan_string_until_nul_byte`: Sets up an a6 frame, clears a local offset, then reads bytes from the a0 string plus offset into d0, incrementing the offset until a zero byte is found.
- `return_saved_d0_from_frame`: Loads the longword stored just below the a6 frame pointer into d0 as the return value, then releases the a6 frame and returns.

### Amiga Karate.adf:spdata

- `spdata_sprite_data_table`: Bytes in the spdata file decoded as ori instructions; they are sprite data, not executable code, and the routine is a data table.

### Amiga Karate.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
