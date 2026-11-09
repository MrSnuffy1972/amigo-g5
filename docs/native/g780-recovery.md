# Game 780 (battle chess (1988)(interplay)[cr amigos][f]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00780). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (935728 top-level bytes in 36 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 24 of 24 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 389 of 389 named (0 author symbols), 389 of 389 described (76 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1522 of 1522 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7545 of 7545 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3999 |
| code | 23742 |
| data | 886685 |
| padding | 9526 |
| unused | 11776 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Else | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Ask | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Icon.Library | 15 | 1414 | 108 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Path | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:EndCLI | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:NewCLI | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Echo | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Why | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Lab | 1 | 2 | 1 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Skip | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:BattleChess | 2 | 267 | 8 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Failat | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Copy | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Makedir | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Which | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:EndIf | 1 | 2 | 1 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:If | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:IconX | 12 | 806 | 37 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:LoadWB | 11 | 362 | 22 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Wait | 2 | 117 | 2 |
| Battle Chess (1988)(Interplay)[cr Amigos][f].adf:bootblock | 1 | 130 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1026 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 446 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1452 | 143 |

## Cases handled

- battle chess (1988)(interplay)[cr amigos][f]/Battle Chess (1988)(Interplay)[cr Amigos][f].adf:ChessStuff: high-entropy data file (7.94 bits/byte), classified as packed data
- runtime segment an address: 442 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 410 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 420 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- battle chess (1988)(interplay)[cr amigos][f]/Battle Chess (1988)(Interplay)[cr Amigos][f].adf:BattleChess: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Else

- `allocate_and_init_else_block`: Calls library routines through a5 to find the largest node value, allocates a block, fills it with incrementing longwords and copies sixteen longs into the structure at offset 536(a2).
- `return_d1_in_d0_then_branch`: Moves d1 into d0 as the return value and branches forward into another routine instead of returning here.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Ask

- `load_and_install_icon_bank`: Reads the icon bank header, finds the largest icon size, allocates a buffer, fills it with icon records, swaps the current icon data with the new bank and returns zero or minus one on failure.
- `move_d1_to_d0_and_branch`: Copies the longword in d1 into d0 and then branches unconditionally to another routine, transferring control without returning here.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Icon.Library

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `icon_lib_open_increment_and_clear_flag`: Increments the word open counter at offset 32 of the library base in a6, clears bit 3 of the byte at offset 14, and returns the base pointer in d0.
- `icon_lib_close_decrement_and_release`: Returns zero in d0, decrements the word open counter at offset 32 of the a6 library base, and if it reaches zero with bit 3 of the byte at offset 14 set, calls the release routine.
- `close_icon_library_and_unlink_node`: Checks word at offset 32 of the library base; if zero, calls the library close vector at -210 with a1 set to the base, then unlinks the node by patching its predecessor and successor pointers and returns the saved value.
- `set_library_open_flag_return_zero`: Sets bit 3 of the byte at offset 14 of the library base to mark it open, then returns zero in d0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `open_icon_library_and_store_vectors`: Stores a0 and a6 into the library base, calls the open vector at -552 with a name and length 33, saves the result, and on failure calls exec Alert with a fixed alert number.
- `store_a2_offsets_to_library_globals`: Copies the longwords at offsets 34 and 38 of a2 into the two library global slots, returns a2 in d0, then pops a2 and returns.
- `zero_filled_data_slot_first`: This single zero word decodes as ori.b with zero immediate; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_slot_second`: This single zero word decodes as ori.b with zero immediate; it is a second zero-filled data slot referenced by address, not executable code.
- `icon_lib_call_with_a0_a6_saved`: Pushes a0 and a6 onto the stack, calls a library routine through an absolute jump, then discards the eight saved bytes and returns.
- `icon_lib_call_with_a0_a1_a6_saved`: Pushes a0 and a1 plus a6 onto the stack, calls a library routine through an absolute jump, then restores the stack by adding twelve and returns.
- `icon_lib_call_with_a0_a2_a6_saved`: Pushes a0, a1 and a2 plus a6 onto the stack, calls a library routine through an absolute jump, then restores the stack by adding sixteen and returns.
- `icon_library_call_3fe_with_a0_a1_saved`: Saves a0, a1 and a6 on the stack, calls the library routine at offset 1022, then discards the saved registers and returns.
- `icon_library_call_666_with_a0_saved`: Pushes a0 and a6 onto the stack, calls the library routine at offset 1638, adds eight to the stack pointer and returns.
- `icon_library_call_698_with_a0_saved`: Pushes a0 and a6 onto the stack, calls the library routine at offset 1688, adds eight to the stack pointer and returns.
- `icon_library_a6_thunk_call`: Pushes a6 onto the stack, calls the library routine at offset 6b2, then discards the pushed longword and returns.
- `icon_library_save_a0_a2_call`: Saves a0 through a2 and a6 on the stack, calls the library routine at offset 72e, then restores the stack by adding sixteen and returns.
- `icon_library_save_a0_call`: Pushes a0 and a6 onto the stack, calls the library routine at offset 782, then discards eight bytes of stack and returns.
- `icon_library_call_820_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 820, then discards both saved registers and returns.
- `icon_library_call_842_with_a0_a1`: Saves a0 and a1 on the stack, calls the library routine at offset 842, then removes the two saved registers and returns.
- `icon_library_call_88a_with_a0_a1`: Pushes a0 and a1 onto the stack, calls the library routine at offset 88a, then cleans the stack and returns to the caller.
- `icon_library_call_jsr_8fe`: Saves a0 and a1 on the stack, calls the routine at address 8fe, then discards the saved registers and returns to the caller.
- `icon_library_set_field_94`: Opens the library base from the stack argument, calls its offset -294 vector, stores the second argument into offset 148 of the returned pointer, and returns.
- `icon_library_zero_fill_data`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data, not executable code.
- `allocate_and_init_icon_node`: Calls library helpers to allocate an icon node from the first argument, then copies 44 bytes of icon data from the stack into the node and stores several fields, returning the node or zero.
- `copy_icon_fields_into_node`: Copies 44 bytes from the stack frame into the icon node at offset 96, then stores byte and long fields from the frame into the node and links it into its parent list.
- `return_zero_restore_registers`: Sets d0 to zero as the return value, restores d2 through d4 and a2 from the stack, releases the a6 frame and returns to the caller.
- `copy_icon_header_to_stack_and_dispatch`: Copies 44 bytes from offset 96 of the icon structure into a local stack buffer, fills in header fields from the structure, then calls the helper with the buffer and two arguments.
- `open_icon_library_and_load_icon`: Builds a library name on the stack, opens the icon library, allocates an icon structure, and loads the icon data, returning zero on failure.
- `allocate_and_load_icon_data`: Allocates memory for the icon and its image data, verifies the icon header, and loads the image planes, returning success or failure.
- `open_icon_library_extra_resources`: Calls library open routines to allocate and store four resource pointers at offsets 18, 22, 50 and 54 of the icon base, returning success or failure in d0.
- `read_icon_library_extra_array`: Repeatedly calls the read and allocate callbacks to fill a pointer array at a3, then optionally reads one more resource into offset 70 and returns one in d0.
- `return_zero_icon_failure`: Sets d6 to zero and branches to the shared exit path, returning a failure result to the caller.
- `restore_regs_unlink_and_return`: Pops registers d2 through d6 and a2 through a5 from the stack, releases the a6 frame pointer and returns to the caller.
- `open_icon_library_and_validate`: Saves registers, opens the icon library with a version request, and on failure reports an error and returns zero; otherwise it stores the library base and continues.
- `build_icon_from_disk_object`: Saves the icon structure pointer, opens the icon library, checks the icon type flags, and calls library routines to load the icon image and associated data.
- `icon_library_draw_icon_lists`: Calls library helpers to draw the icon's two image lists and optional text, storing the resulting size in the frame and returning status in d5.
- `icon_library_skip_remaining_list_entries`: Advances a4 past the remaining nonzero entries of the icon list, adds four to the frame size, then calls the draw callback and restarts the list.
- `icon_library_draw_text_and_finish`: Draws each text string in the list via the callback, draws the optional title, restores registers, stores the frame size and returns d5.
- `free_icon_list_entries`: Saves d2-d3, takes a list pointer from the stack, walks each entry at offset two, calls the free routine on non-null entries, then restores registers and returns.
- `restore_d2_d3_and_return`: Pops d2 and d3 from the stack and returns to the caller, restoring the registers saved by the list-freeing routine.
- `free_icon_at_offset_140`: Saves d2, loads the list pointer and an icon pointer, adds 140 to the icon pointer, calls the list-freeing routine, then restores d2 and returns.
- `open_icon_library_icon_by_name`: Saves d2-d4/a2, allocates a 64KB icon structure, opens the icon library base, then looks up the icon named by the caller's argument and returns its pointer or zero.
- `retry_icon_lookup_with_second_path`: Pushes the library base, icon structure and name argument, calls the lookup routine, cleans the stack, and returns the icon pointer in d0 or branches to the failure path.
- `return_zero_and_restore_registers`: Sets d0 to zero as the failure return value, restores d2-d4 and a2 from the stack, and returns to the caller.
- `decrement_icon_slot_and_store_pair`: Pops saved registers, decrements the word count at a2, and if it reaches zero calls the library allocator before storing two longword values into the icon slot array.
- `store_pair_into_icon_slot`: Decrements the word count at a2, computes the slot offset, and writes d2 and the stack argument into the icon slot array, returning one.
- `allocate_and_initialize_icon_node`: Allocates a 94-byte icon node, initializes its fields, links it via the library routines, and returns the node pointer or zero on failure.
- `return_d6_in_d0_tail_branch`: Copies the longword in d6 into d0 and branches onward to another library routine, leaving other registers untouched.
- `icon_library_close_wrapper`: Pushes a small constant and d6 onto the stack, calls the library close vector, clears d0, restores the stack and pops d2-d6/a2 before returning.
- `icon_library_open_with_offset`: Saves d2-d4, loads two arguments from the stack, adds seventy-eight to the second, pushes both and calls the library open vector before restoring registers.
- `find_tool_name_after_equals`: Saves d2-d3/a2-a3, scans a null-terminated pointer array for a tool name whose length matches and whose next byte is '=', returning the text after the equals sign or zero.
- `return_zero_and_restore_registers_2`: Sets d0 to zero as the return value, restores d2-d3/a2-a3 from the stack and returns to the caller.
- `check_tool_name_length_match`: Saves d2-d6, computes the length of a name, then scans a pointer array for an entry of equal length that matches the name, returning one on success or zero.
- `check_icon_pair_match`: Calls two library routines to compare d5 with the first result and test the second, returning 1 in d0 on a match, otherwise falling through to the shared exit.
- `advance_d2_from_d4_or_zero`: Tests d4; if zero it branches to the shared exit, otherwise copies d4 into d0, increments d0, stores it in d2 and jumps back into the caller's loop.
- `clear_d2_and_rejoin_loop`: Sets d2 to zero with a quick move and branches back into the shared loop entry of the calling routine.
- `return_zero_pop_d2_d6`: Sets d0 to zero as the return value, restores d2 through d6 from the stack, and returns to the caller.
- `parse_icon_tooltypes_into_buffer`: Copies the tool type string at the second argument into a buffer, terminates it at offset thirty, then parses the copy and stores the parsed fields into the first argument.
- `parse_remaining_tooltype_after_copy`: Advances a2 past five bytes, calls the string compare routine on the next field, and on success sets d5 to one and advances a2 by three more bytes.
- `init_d5_zero_and_branch`: Sets d5 to zero and branches forward into another routine, serving as a small entry stub in the Icon.Library code.
- `parse_decimal_digits_into_value`: Accumulates a decimal number from characters at a2 using d5, calls a digit converter, then formats and stores the result via library calls before returning d3.
- `convert_ascii_digit_to_value`: Reads the argument at offset four on the stack, checks it lies between character zero and nine, and returns its numeric value in d0.
- `return_minus_one_constant`: Loads the immediate value minus one into d0 and returns it to the caller as a constant result.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `allocate_and_verify_icon_memory`: Saves d2-d5, allocates memory via a library call, verifies it with a second call, frees it on failure, and returns the pointer or zero.
- `allocate_icon_node_and_init_counts`: Allocates a small memory block, stores the value ten at offset fourteen and in the caller's word, links the block, and returns success.
- `icon_return_zero_restore_regs`: Sets d0 to zero as the return value, then pops d2 and d3 and a2 from the stack before returning to the caller.
- `icon_compare_three_args_via_subcall`: Saves d2 and d3, loads three arguments from the stack, pushes them and calls a library routine, then compares the result with the fourth argument and returns it or zero.
- `icon_return_d1_restore_regs`: Copies d1 into d0 as the return value, then pops d2 and d3 from the stack and returns to the caller.
- `compare_three_arg_call_result`: Saves d2-d3, loads three arguments from the stack, calls a helper with them, then returns the helper result in d0 only if it equals the fourth stack argument, otherwise zero.
- `return_d1_in_d0_restore_d2_d3`: Moves d1 into d0 as the return value, restores d2 and d3 from the stack and returns to the caller.
- `unsigned_32bit_divide_d1_by_d2`: Divides the 32-bit value in d1 by d2 using word divisions, returning quotient in d0 and remainder in d1, with a shift loop for large divisors.
- `unsigned_divide_32_by_16_bitwise`: Divides the 32-bit value in d1 by the 16-bit divisor in d2 using a 16-step shift-and-subtract loop, leaving the quotient in d0 and remainder in d1.
- `multiply_32_by_32_into_64`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, returning the 64-bit product in d0 with the low word cleared.
- `format_string_via_rawdofmt`: Saves a2-a4 and a6, loads the format string and argument pointer from the stack, then calls exec RawDoFmt through the library base at absolute address 4.
- `store_d0_byte_to_a3_postincrement`: Writes the low byte of d0 to the memory pointed to by a3, then increments a3 by one and returns to the caller.
- `strlen_up_to_255_bytes`: Reads bytes from the string pointed to by the stack argument a0, counting until a zero byte or 255 bytes, and returns the count in d0.
- `find_first_matching_byte_in_string`: Scans the string at the stack argument a0 for the byte in d0, returning a pointer to the match in d0 or zero if none is found.
- `return_zero_in_d0`: Sets d0 to zero and branches onward to another routine, providing a zero return value for its caller.
- `copy_string_until_nul`: Loads two pointers from the stack, copies bytes from the source to the destination until a zero byte is copied, then returns.
- `skip_string_to_terminator`: Loads two pointers from the stack, advances a0 past bytes until a zero byte is found, steps back one, then branches to another routine.
- `compare_counted_strings_ignore_case`: Loads a0 and a1 from the stack, takes a length from the stack into d0, decrements it, then compares bytes from (a1) and (a0) until a mismatch or a zero byte, returning zero in d0 on a full match.
- `return_compare_result_flag`: Sets d0 to one, branches on the condition codes left by the caller's comparison, otherwise sets d0 to minus one, then returns to the caller.
- `skip_leading_zero_bytes_counted`: Loads a0 and a1 from the stack, reads a count from the stack into d0, scans bytes at (a0) until a zero byte or the count runs out, then branches onward.
- `decrement_a0_and_call_copy`: Subtracts one from the longword in a0, then branches to the copy routine at the following address before returning to its caller.
- `copy_bytes_with_zero_fill`: Copies bytes from a1 to a0 while decrementing d0, stopping on a zero byte or count exhaustion, then fills remaining bytes with zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_icon_structure_pointers`: Loads the first stack argument into a0, stores a0 at its own start, adds four to that value, clears the next longword and stores a0 at offset eight.
- `add_icon_with_offsets`: Pushes registers, reads three arguments from the stack, checks the icon pointer, then calls library routines to add an icon using computed width and height offsets and returns a status.
- `add_icon_list_chain`: Pushes registers, reads three arguments, then loops over a linked list of icons, calling library routines to add each icon and its scaled data until the list ends.
- `return_one_success`: Loads the value one into d0 and branches to the shared return path, indicating success to the caller.
- `icon_lib_return_zero_pop_d2_d5_a2`: Library routine that sets d0 to zero as its return value, restores d2 through d5 and a2 from the stack, then returns to the caller.
- `icon_lib_allocate_and_fill_icon_bitmap`: Library routine that allocates two memory blocks via library calls, computes a rounded bitmap width from the icon header, fills the bitmap with two color values, stores the result pointer and returns the block or minus one on failure.
- `icon_lib_return_minus_one_pop_d2_d6_a2`: Library routine that sets d0 to minus one as an error return value, restores d2 through d6 and a2 from the stack, then returns to the caller.
- `load_icon_chunk_chain`: Saves registers, reads three arguments from the stack, allocates a sixteen-byte header and a data block via library calls, links chunks through offset twelve and returns the first chunk or minus one.
- `allocate_icon_data_and_link_next`: Stores the next pointer at offset twelve, allocates a sixteen-byte header and a data block sized from byte seven times four, saves the data pointer at offset eight and continues the chain.
- `return_a3_as_d0_and_branch`: Moves the a3 register into d0 as the return value and branches forward into the following routine's code.
- `icon_library_return_minus_one`: Sets d0 to minus one as the return value, restores registers d2 through d7 and a2 through a3 from the stack, then returns to the caller.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Path

- `path_open_library_and_allocate_node`: Calls the library open vector, scans a linked list for the largest value, allocates a node, fills it with tag words, copies sixteen longs into the library base, and returns zero or minus one.
- `path_copy_d1_to_d0_and_branch`: Moves the value in d1 into d0 and branches forward into another routine, transferring control without returning here.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:EndCLI

- `endcli_allocate_cli_and_run_command`: Looks up the largest resident node size, allocates a CLI structure via the exec library, fills its command buffer with incrementing words, sets up the stack frame and runs the command through the CLI vector, returning zero or minus one.
- `copy_d1_to_d0_and_branch`: Moves d1 into d0 and branches to the code at offset 260, which is the tail of the CLI setup routine.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:NewCLI

- `open_library_and_allocate_seglist`: Opens a library via a5, walks the segment list to find the highest segment number, allocates a new segment list, fills it with segment entries, copies sixteen longs into the task structure and returns zero or minus one.
- `copy_d1_to_d0_then_branch`: Moves the longword in d1 into d0 and then branches unconditionally to another routine's code.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Echo

- `echo_command_allocate_and_dispatch`: Reads a command number, finds the largest table entry, allocates memory via library calls, fills a structure with tag values, copies sixteen longwords, installs a return address and dispatches, returning zero or minus one.
- `echo_copy_d1_to_d0_and_branch`: Moves the value in d1 into d0 and branches forward into the shared code at the Echo entry point, so the caller's argument becomes the return value.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Why

- `load_piece_animation_frames`: Reads a resource list through a5 library calls, finds the largest frame index, allocates and fills a frame table, copies sixteen-longword animation data, and returns zero or minus one.
- `return_d1_in_d0_and_branch`: Moves d1 into d0 as the return value and branches forward into another routine's code.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Lab

- `clear_d0_and_return`: Clears d0 to zero with an exclusive-or and returns immediately to the caller.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Skip

- `load_hunk_and_relocate_segments`: Reads a hunk file through the exec library, allocates memory for each segment, copies segment data, applies relocations and returns zero on success or minus one on failure.
- `return_d1_in_d0_and_branch_2`: Moves d1 into d0 as the return value and branches to a fixed offset inside the following routine.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:BattleChess

- `hunk_loader_relocate_and_allocate`: Loads AmigaDOS hunks, allocates memory with exec.AllocMem, copies and relocates segments, frees memory with exec.FreeMem and returns the loaded image.
- `restore_registers_and_close_library`: Pops saved registers d0-d7 and a0-a6 from the stack, closes the opened library via a6, and returns to the caller.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `decompress_rle_bitstream_to_buffer`: Reads a packed bitstream from memory at a0, expands run-length and literal codes, and writes the decoded bytes backwards into the buffer ending at a3.
- `refill_bit_buffer_from_stream`: Shifts the d5 bit buffer right by d4 bits; if empty, loads the next longword from -(a0) and rotates it in, returning the bits in d5.
- `subtract_d4_word_from_d0`: Subtracts the low word of d4 from the low word of d0 and falls through to the following code without returning.
- `blit_bitplane_columns_to_copper`: Shifts bits from d5 through d4 into d1, reads bytes from a5 indexed by d1, copies bytes backwards into a3, and writes a3 to the COLOR17 hardware register.
- `data_table_decoded_as_movep_trap`: The bytes are data, not executable code: they decode as a movep from a2, an ori immediate, a trap and an or, with no meaningful routine behavior.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Failat

- `allocate_and_init_io_request`: Calls the library open routine, walks a linked list to find the largest value, allocates a request block, fills it with incrementing command words and copies sixteen longs into the request structure, returning zero or minus one.
- `return_d1_in_d0_then_jump`: Moves d1 into d0 as the return value and branches to the code at offset 260, which is the shared exit path of the caller.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Copy

- `allocate_and_init_io_request_copy`: Same code as the Failat routine: opens a library, scans a linked list for the maximum, allocates a request block, writes incrementing command words and copies sixteen longs, returning zero or minus one.
- `copy_d1_to_d0_then_branch_2`: Moves the longword in d1 into d0 and then branches unconditionally to another part of the enclosing routine.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Makedir

- `allocate_and_initialize_memory_block`: Calls library routines through a5 to find the largest free block, allocate a block, fill it with a header and table entries, copy a 16-longword structure, and return zero or minus one.
- `makedir_copy_d1_to_d0_and_branch`: Moves the value in d1 into d0 and branches forward into the shared code at the Makedir entry point, passing the caller's argument through as the result.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Which

- `load_piece_animation_frames_2`: Looks up a piece type in a table, allocates frame memory, fills it with animation opcodes, copies sixteen longs into the piece record and installs a new update vector.
- `copy_d1_to_d0_and_branch_2`: Moves d1 into d0 and branches forward into another routine, serving as a small entry stub.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:EndIf

- `return_zero_constant`: Clears d0 to zero and returns immediately, giving callers a constant zero result.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:If

- `load_and_install_iff_brush`: Reads a brush file through the dos library, finds the largest palette index, builds a color table, copies sixteen palette entries into the icon structure and returns zero or minus one.
- `return_d1_in_d0_then_jump_2`: Moves d1 into d0 as the return value and branches to the shared exit code that restores the stack and returns.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:IconX

- `iconx_run_icon_tool`: Opens the icon's tool with dos.Open, parses its argument string into a token array, allocates memory, loads the tool library and jumps to the tool entry, returning a status code.
- `close_iconx_task_and_free_memory`: Finds the current task, switches to its stack, closes the window and library, frees the message and memory, then replies the startup message and returns the original argument.
- `run_iconx_tooltype_command`: Reads the tooltype arguments, opens the window and icon library, builds the command line, runs the icon tool, waits, then closes the window and frees the allocated buffers.
- `execute_iconx_tooltype_entry`: Builds a command string from the tooltype entry, checks for a dot suffix, runs the command with optional wait, and returns one on success or zero when the file cannot be opened.
- `free_icon_list_and_children`: Walks a linked list of icon nodes, frees each node's name and data via library calls, then frees the node itself, returning the first node's handle.
- `build_icon_path_from_list`: Iterates an icon list, appends each node's name into a buffer separated by colons, converts the first slash to a colon, and returns the resulting path length.
- `append_three_strings_to_buffer`: Concatenates three string arguments into a destination buffer by calling the string append routine three times, then appends the buffer at the offset.
- `strlen_count_chars`: Loads the string pointer from the stack into a0, counts bytes until a null terminator, and returns the length in d0.
- `strcpy_copy_string`: Loads destination and source pointers from the stack, copies bytes from source to destination including the null terminator, and returns.
- `strcat_append_string`: Loads a string pointer and a second pointer from the stack, advances a0 to the first string's null terminator, then calls the copy routine to append the second string.
- `parse_signed_decimal_string`: Reads a string pointer from the stack, skips an optional plus or minus sign, accumulates decimal digits into d0 by repeated multiply-by-ten, negates if a minus was seen, and returns the value in d0.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_function_with_three_args`: Saves d2 and d3, loads two arguments from the stack, pushes a third stack-based argument plus both values, calls the target function, cleans the stack and restores registers.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `format_string_via_rawdofmt_2`: Calls exec RawDoFmt with a format string and argument list into a 140-byte stack buffer, measures the resulting string length, then passes buffer and length to another routine.
- `append_byte_to_a3_buffer`: Stores the low byte of d0 at the address in a3 and advances a3 by one, then returns to the caller.
- `format_string_via_rawdofmt_3`: Saves a2-a4 and a6, loads the format string and argument pointer from the stack, calls exec RawDoFmt through a6, restores registers and returns.
- `append_byte_to_a3_buffer_alt`: Stores the low byte of d0 at the address in a3 and increments a3, then returns; identical code reached from a different caller.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:LoadWB

- `loadwb_parse_tooltypes_and_open_libs`: Saves registers, reads the current task's tooltype list, splits the argument string into words, opens the required libraries and stores the resulting base pointers in globals before returning zero.
- `loadwb_cleanup_and_exit`: Restores the saved stack pointer, closes the opened library, forbids and replies to the pending message, then returns the caller's value or triggers an alert and exits.
- `loadwb_open_and_close_console`: Loads the address of the console structure into a0 and calls the open then close console library vectors through a6 before returning.
- `clear_workbench_startup_error`: Clears the longword at absolute address 8, loads the address of the workbench task name into a1, calls the dos.library function at offset -552 with d0 set to 31, then stores the returned value back at address 8 and returns.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_dup_lock`: Library stub: loads its arguments and the dos library base, calls DupLock and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `start_workbench_task_from_resident`: Opens the workbench task structure, optionally checks a resident tag, starts the tool, computes stack and segment sizes, calls several startup routines, and closes the task structure before returning.
- `compare_two_strings_lexically`: Compares the two null-terminated strings passed on the stack byte by byte, returning zero when equal, minus one when the first is less, and one when greater.
- `build_linked_list_from_array`: Walks an array of pointers, allocates a four-byte node for each entry, formats the entry value into the node, links nodes via shifted pointers, and returns the head node or zero.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_get_msg`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_global_hook_with_stack_arg`: Saves d2, loads the first argument into d2, pushes a pointer to the caller's stack argument and a global longword, then jumps through a global vector and restores the stack.
- `format_string_via_rawdofmt_4`: Calls exec RawDoFmt with a format string and arguments into a 140-byte stack buffer, measures the resulting string length, and passes it to another routine.
- `store_d0_byte_to_a3_postincrement_2`: Writes the low byte of d0 to the address in a3, then increments a3 by one and returns to the caller.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:Wait

- `allocate_and_init_library_base`: Queries the library base, scans a list for the largest value, allocates a block, fills it with incrementing longwords, copies sixteen longs to and from offset 536, and returns zero or minus one.
- `copy_d1_to_d0_and_branch_back`: Moves the long in d1 into d0 and branches backward into the preceding routine, so it returns d1 as the result of that caller.

### Battle Chess (1988)(Interplay)[cr Amigos][f].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `bootblock_load_and_run_disk_boot`: Opens graphics.library, allocates a bitmap, copies a copper list to chip memory, builds display copper instructions, waits for vertical blank, plays a short animation loop reading the fire button, then closes the library and returns.
- `fill_copper_line_entries`: Writes fourteen copper instruction pairs into memory at a0, each with a move value from d1, a fixed wait, and a data word from d2, advancing d1 by 512 and d2 by d3 per entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `runtime_startup_init_and_main_loop`: Runs startup subroutines, allocates memory, checks a size limit, initializes game state and pointers in a4-relative memory, then enters an endless main loop calling the game frame routine.
- `run_game_state_update_chain`: Checks two state words and a countdown at offsets from a4, then conditionally calls several library routines and two local routines, finally pushes 10 and calls another library routine before returning.
- `call_cleanup_then_state_update`: Saves a2, passes the caller's first argument to a library routine at offset from a4, then calls the state update routine and restores a2 before returning.
- `show_opponent_quit_message`: Pushes the string "Your Opponent just quit" and length 24 onto the stack, calls a library routine at offset from a4, cleans the stack and returns.
- `build_level_asset_pointer_table`: Walks a byte size table to compute and store sequential asset pointers in a4-relative arrays, then sets up display and sound resources and calls several helpers.
- `init_two_sprite_block_pointers`: Stores base addresses of two a4-relative data blocks and derives sub-block pointers at offsets 264 and 60 into a4-relative pointer slots.
- `xor_decrypt_40000_byte_block`: Selects a graphics asset by index, copies 40000 bytes to the work buffer, then XORs the first 39960 bytes with the following 40 bytes in place.
- `save_state_to_backup_slot`: Copies two longwords, four words and one longword from the current state variables at negative a4 offsets into the backup slot, then returns.
- `restore_state_from_backup_slot`: Copies the backup slot's two longwords, four words and one longword back into the current state variables at negative a4 offsets, then returns.
- `copy_state_to_secondary_slot`: Copies two longwords, four words and one longword from the current state variables into a second saved slot at different negative a4 offsets, then returns.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `xor_blit_columns_advance_cursor`: Copies each source longword and the current bitmap pointer into a table, calls a blit helper with width times height times five, then XORs five rows of bytes into the destination and advances the stored bitmap pointer and index.
- `wait_dmaconr_blitter_busy_clear`: Polls the DMACONR hardware register at the custom chip base and loops while bit 6 (blitter busy) is set, returning once the blitter has finished.
- `compare_sprite_entry_offsets`: Loads an eight-byte entry from a table indexed by a word argument, checks its first two bytes are nonzero, and compares byte offsets against the caller's structure, returning a computed difference.
- `copy_sprite_x_and_height_to_buffer`: Loads the byte at a2 into d0, stores it at the buffer pointed to by the pointer at offset 14 of a5, then copies byte 2 of a2 to byte 2 of that buffer and branches onward.
- `compute_horizontal_overlap_extent`: Reads bytes from a2 and a3, sign-extends them, subtracts the a2 height from the a3 height, adds the a2 width, and compares the sum against the a3 width, returning the larger value in d0.
- `copy_sprite_origin_and_vertical_extent`: Copies byte 0 and byte 2 of a3 into the buffer at offset 14 of a5, then compares byte 3 of a2 with byte 3 of a3 and computes a vertical extent in d0.
- `copy_a2_offsets_to_sprite_struct`: Loads byte at offset 1 of a2 into d0, stores it at offset 1 of the structure pointed to by a5+14, then copies byte at offset 3 of a2 to offset 3 of that structure and branches onward.
- `compute_clamped_sprite_y_from_a2_a3`: Reads signed bytes at offsets 3 and 1 of a2 and a3, computes a clamped vertical value in d4, stores the chosen byte into the sprite structure at a5+14, then branches to shared continuation code.
- `copy_a3_offsets_to_sprite_struct`: Loads byte at offset 1 of a3 into d0, stores it at offset 1 of the structure pointed to by a5+14, then copies byte at offset 3 of a3 to offset 3 of that structure and branches onward.
- `blit_sprite_pair_with_masks`: Copies four bytes from a2 into the sprite structure at a5+14, sets up blitter pointers and modulo from sprite dimensions, then blits the sprite and a second masked sprite at a3 using BLTCON0 values and waits between passes.
- `blit_sprite_with_three_planes`: Reads width and height bytes from a2, computes row bytes and BLTSIZE, sets blitter pointers for three bitplanes from a3, blits the sprite into the destination at a5+12, then disables blitter DMA and returns.
- `blit_sprite_three_planes_fixed_dest`: Computes row bytes and BLTSIZE from a2 dimensions, sets three bitplane pointers from a3, blits to a fixed destination pointer from a4, then disables blitter DMA and returns.
- `open_graphics_and_intuition_libs`: Opens graphics.library and intuition.library via exec, stores the bases in globals, and reports an error and exits if either open fails.
- `open_screen_and_window_setup_rasters`: Opens a screen and window, stores their pointers, sets up two copper or raster structures with palette and scroll values, and advances a scroll offset by 250.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_d4_to_d7_unlink_and_return`: Pops registers d4 through d7 from the stack, releases the a5 frame pointer, and returns to the caller.
- `init_bitplane_clear_and_setup_display`: Clears 512 longwords at the display buffer, allocates 10 bytes on the stack, calls a library routine, stores the returned pointer, reads two pointers from a table, adjusts the source pointer by 512 bytes, conditionally halves the height and width, calls two helper routines, updates a counter and returns.
- `recursive_bit_writer_with_table_store`: Recursively writes bits of d0 into a table at the display buffer indexed by d6, shifting and setting bits based on a helper test, then stores d0 and d1 words into the table and returns.
- `bit_stream_reader_refill_buffer`: Tests the current bit mask in d2 against d3, rotates the mask, and when the source pointer reaches the end, saves registers, resets the pointers from the display buffer, calls a library routine, reloads the next word into d3 and returns.
- `build_huffman_code_bits`: Loops eight times, shifting d6 left and calling a bit-reading routine; sets bit 0 of d6 when the call returns zero, building a byte of code bits.
- `rebuild_huffman_decode_tables`: Scans 256 entries at a fixed table, expands short codes into a stack lookup tree and long codes into a sorted list, then copies the rebuilt tables back over the original.
- `huffman_decode_bytes_to_buffer`: Decodes a requested count of bytes from a bit stream into the caller's buffer using the rebuilt Huffman tables, refilling the bit register from the source and returning the count written.
- `compute_bitmap_buffer_size`: Loads the bitmap base pointer and data pointer from the two longs at the fixed table, adds 512 to the first, subtracts the second and returns the difference in d7.
- `refill_bit_buffer_or_advance_word`: If d7 is nonzero, swaps d0, reads the next word from (a0) into the low half of d0, decrements d7 by two and returns; otherwise saves registers, recomputes the buffer size, calls the refill routine and reloads a0 before returning.
- `decode_huffman_run_into_buffer`: Decodes up to d4 symbols from the bit stream in d0 using the code tables at a3 and a2, writing bytes to (a1) and consuming d6 bytes, saving and restoring the bit state in a4 globals.
- `read_byte_or_word_bitmask`: Pushes d2, clears a word at -20250(a4), calls a library vector, reads a byte or word from the pointer at -82(a4), advances the counter at -94(a4), stores the value at -22306(a4) and a shifted bitmask at -22308(a4), then calls the vector again and returns.
- `advance_stream_offset_by_buffer_gap`: Computes the difference between the buffer end at -86(a4) plus 512 and the current pointer at -82(a4), subtracts d7, adds the result to the offset at -94(a4), calls a library vector and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_two_library_vectors_by_number`: Opens a stack frame, calls a library open routine twice with tag values 7 and 8 and name pointers, stores the returned handles at 11474(a4) and 11478(a4), then releases the frame and returns.
- `init_sound_and_second_call`: Tests a global word at offset from a4; if zero, calls an unnamed routine twice with a zero word and a one word pushed on the stack, then returns.
- `push_arg_call_and_test_flag`: Pushes the word at offset 12 from a5, calls an unnamed routine, then tests a global word and returns without branching if nonzero.
- `start_audio_channel_one_or_two`: Selects audio channel one or two from a5 offset 12, clears a flag, programs AUD1 period, volume, length and pointer from a5 data, and enables DMA and interrupts.
- `start_audio_channel0_playback`: Clears a state word, acknowledges and enables interrupts, computes the period from the sample offset and pitch, writes AUD0LCH/LEN/PER/VOL, then enables DMA and branches to the audio continuation.
- `disable_audio_dma_if_flag_set`: Tests a stack flag; if nonzero it disables audio DMA via DMACON and disables interrupts via INTENA, then branches onward.
- `stop_audio_and_call_cleanup`: Disables audio DMA and interrupts, pushes the value 2 and calls a library routine through a4, then releases the frame and returns.
- `increment_counter_and_set_sound_priority`: Writes 256 to offset 156 of a0, saves a4, sets a4 to a1, reads and increments the word counter at offset 11482 of a4, and if the old value was zero writes 256 and 2 to offsets 154 and 150 of a0.
- `copy_pointer_and_set_timer_word`: Copies the longword at offset minus 12868 of a4 into offset 176 of a0, writes 50 to offset 180 of a0, then pops a4 from the stack and returns.
- `reset_display_and_advance_counter`: Writes 128 to offset 156 of a0, saves a4, copies a1 to a4, reads and increments the word counter at offset 11484 of a4, and if the old value was zero writes 128 and 1 to offsets 154 and 150 of a0.
- `init_disk_port_and_open_trackdisk`: Stores a pointer and word into a caller-supplied structure, then creates a message port, opens trackdisk.device, fills a port structure and validates the drive number, returning after cleanup.
- `compute_scaled_track_offset`: Builds a stack frame, reads two words from the a4 globals, shifts one right by nine bits, adds sixteen times the other, and stores the resulting offset at minus four of a5 before branching away.
- `loop_copy_blocks_advance_track`: Repeatedly calls a copy routine with the offset and buffer from the a5 frame, advances the a4 track position and a5 buffer by five hundred twelve bytes, increments the offset, and loops while the remaining count stays positive.
- `update_scroll_position_and_redraw`: Computes a scroll offset from the a4 globals, compares it with the last stored value, and if changed either calls the blit routine or sets up a hardware scroll and blit before saving the new offset.
- `setup_blit_with_mode_8002`: Stores blit control word 8002, size 512, the given source and a shifted destination into a4 blit registers, then calls the blit routine.
- `setup_blit_with_mode_8003`: Stores blit control word 8003, size 512, the given source and a shifted destination into a4 blit registers, then branches to the blit routine.
- `check_disk_prompt_and_wait_loop`: Calls the disk prompt routine, checks a status byte for value 29, and while it matches calls the disk check and message routines, then waits on a timer until the status byte clears.
- `save_and_restore_disk_state`: Copies 14 longs of disk state to the stack, sets up disk parameter blocks, calls the disk routine, and restores the saved state, returning 1 on success or 0 on failure.
- `reset_disk_params_if_idle`: Tests the long at offset 11542 in a4; if zero, sets the disk parameter word to 9, clears the long at 11522 and calls the disk routine.
- `reset_state_and_return_stored_value`: Writes minus one to a global field at offset minus ninety from a4, then returns the longword stored at offset minus eighty-six from a4 in d0.
- `build_and_submit_audio_request`: Fills a stack buffer via a library call, formats a name string, sets two byte fields to eight and twelve, then submits a request structure with priority thirty-two.
- `init_interrupt_server_and_enable_audio`: Clears a server structure, stores a4 and a callback, registers it, then acknowledges interrupt requests and enables audio and serial interrupts.
- `serial_wait_loop_for_tx_ready`: Sets up a stack frame, saves d4 and a2, loads a2 from the first argument and d4 from the second, then loops testing bit 5 of the SERDATR register until it is set, branching back to the test.
- `serial_send_byte_with_stop_bit`: Copies a2 to a0, increments a2, reads the byte at a0 into d0, sets bit 8 of d0 and writes the word to the SERDAT transmit register, then branches back to the wait loop.
- `restore_d4_a2_and_return`: Restores d4 and a2 from the stack, releases the a5 stack frame and returns to the caller.
- `store_byte_to_ring_buffer`: Writes the word at offset 24 of a0 as a byte into the buffer pointed to by a4's field, advances the pointer, wraps it to the buffer start, and restores a4.
- `uppercase_ascii_to_lowercase`: Reads the byte argument from the stack, and if it is between 'a' and 'z' adds 32 to convert it to uppercase, returning the result in d0.
- `lowercase_ascii_to_uppercase`: Reads the byte argument from the stack, and if it is between 'A' and 'Z' adds 32 to convert it to lowercase, returning the result in d0.
- `clear_bss_open_dos_and_startup`: Calls the a4 setup, clears a block of globals, saves the stack and exec base, opens dos.library, and on failure alerts then runs the startup routine.
- `set_a4_to_runtime_base`: Loads the runtime data base address into a4 and returns immediately to the caller.
- `allocate_and_init_runtime_context`: Allocates a context block, fills its fields, opens a library, sets up message ports and calls several subroutines before returning.
- `copy_d0_to_d5`: Copies the longword in d0 into d5, then falls through into the following routine.
- `tokenize_script_line_into_argv`: Allocates a buffer, copies a script line, splits it into whitespace-separated tokens honoring double quotes, stores pointers in an array and returns via a5 frame.
- `copy_d0_to_d0`: Copies the longword in d0 onto itself, a no-op, then falls through into the following routine.
- `bounded_strncpy_pad_zero`: Copies bytes from source a1 to destination a0 until a zero byte or the word count in d1 runs out, then pads the rest with zero bytes and returns the destination in d0.
- `overlap_safe_memmove_bytes`: Copies a byte count from a0 to a1, choosing forward or backward direction based on address order so overlapping regions are copied safely, then returns.
- `load_7fff_into_d0_and_branch`: Loads the immediate value 32767 into the low word of d0 and branches unconditionally to another routine's code.
- `bounded_strcpy_into_existing_string`: Scans the destination string to its terminator, then copies up to the given count of bytes from the source, null-terminating, and returns the destination pointer in d0.
- `bounded_strcmp_returning_sign`: Compares two strings byte by byte up to 32767 bytes, returning zero if equal, one if the first is greater, or minus one if less in d0.
- `strcpy_return_dest_then_mul32`: Copies the source string including terminator to the destination and returns the destination in d0; the following code multiplies d0 by d1 as a 32-bit product.
- `open_window_and_load_icon_images`: Opens a window via a library call, stores the handle, then fetches an object and its icon images, saving image pointers and indices into the window structure before closing the library.
- `measure_elapsed_timer_ticks`: Opens the timer device, reads the current time, adds a fixed delay, computes elapsed ticks, closes the device and optionally stores the result through the passed pointer.
- `unsigned_32bit_divide_d1_into_d0`: Divides the 32-bit value in d0 by the 32-bit value in d1, using a fast word division when the high word of d1 is zero and a 16-step shift-subtract loop otherwise, returning quotient in d0 and remainder in d1.
- `strlen_return_length_in_d0`: Loads the caller's string pointer from the stack into a0, scans bytes until a null terminator, then returns the length minus one in d0.
- `call_optional_hook_then_tail_call`: Opens an a5 frame, calls the function pointer stored at offset 11754 of a4 if nonzero, then pushes the word argument and jumps to the routine at 9416.
- `store_word_arg_and_test_flag`: Opens an a5 frame, saves d4, sign-extends the word argument from the stack into a local, tests the longword at offset 11724 of a4 and branches accordingly.
- `reset_playfield_and_free_buffers`: Loops calling a per-column routine for each entry up to the count at offset -22320(a4), then frees a buffer sized count times six, calls a stored callback, frees three pointers at 11762, 11766 and 11770(a4), and invokes exec.Supervisor when bit 4 of the exec flags is set.
- `flush_two_buffers_via_copy_routine`: Checks two longwords at offsets 11734 and 11746 of a4; if the first is zero and the second nonzero, calls the copy routine twice with the second buffer and a scaled size, then releases sixteen stack bytes.
- `release_primary_buffer_and_restore_stack`: Calls a routine, frees the buffer pointer at 11734(a4) via another routine, then returns the longword at -4(a5) in d0 after restoring a7 from 11728(a4).
- `free_object_slot_by_index`: Looks up a six-byte object slot from the table at offset 11724 of a4 using the word argument, and if the index is out of range or the slot is empty sets status 2 and returns -1.
- `release_object_and_clear_slot`: Checks bit 7 of byte 4 of the object at a2; if clear it pushes the object pointer and calls the release routine, then clears the longword at a2 and returns zero.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `jump_through_a4_table_entry`: Jumps indirectly to the code pointer stored in the table at offset 11720 of a4, minus 30 bytes, transferring control without returning.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `dos_output_then_read_via_library`: Loads the DOS library base from a4 offset, jumps to its Output vector, then jumps to the Read vector after restoring d1-d3 from the stack.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `jump_to_dos_library_vector_offset`: An unreached single jump that loads the DOS library base from a4 and jumps to a vector at offset minus forty-eight.
- `call_write_2`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `save_regs_call_library_then_read`: Saves d7 and a5, loads arguments from the stack, calls a library vector at minus one hundred eight, restores registers and returns; a second entry jumps to a read vector.
- `jump_through_a4_library_vector`: Loads the return address into a1, fetches the library base from offset 11716 of a4 into a6, then jumps through the vector at offset minus 450 of that base.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_init_message_port`: Gets a signal number, allocates a 34-byte message port, fills its fields from the arguments, attaches a signal or list, and returns the port pointer or zero.
- `free_message_port_and_signal`: If the port has a pending message it detaches it, marks the port closed, clears its list pointer, frees the stored signal number and releases the port.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_neg330_with_stack_arg`: Loads the longword at offset 4 of the stack into d0, fetches the library base from a4 offset 11716 into a6, and jumps through the library vector at -330.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `jump_to_library_vector_neg294`: Jumps indirectly through the library vector at offset -294 of the base stored at a4 offset 11716, passing no arguments.
- `call_library_neg294_with_stack_arg`: Loads the longword at offset 4 of the stack into a1, fetches the library base from a4 offset 11716 into a6, and jumps through the vector at -294.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `jump_through_a4_dispatch_offset`: Jumps indirectly to a code pointer fetched from a table based at the a4-relative offset, transferring control without returning.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `jump_through_a4_dispatch_alt_offset`: Jumps indirectly to a code pointer fetched from a different a4-relative table entry, transferring control without returning.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `init_self_referential_struct`: Loads the first stack argument into a0, stores a0 at (a0), adds four to that word, clears the next long, stores a0 at offset eight, then returns.
- `jump_library_neg1bc_with_stack_args`: Loads return address and arguments from the stack into a0, d0/a1 and d1, fetches the library base from a4 offset, and jumps through the library vector at negative offset.
- `jump_library_neg228_with_a1_d0`: Loads the library base from a4, moves the stack argument into a1 and d0, then jumps through the library vector at negative offset.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `jump_pc_relative_to_next_routine`: Performs a single pc-relative jump to the following instruction address, transferring control without touching any registers or memory.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `jump_through_a4_offset_table`: Loads a pointer from the a4-relative table and jumps indirectly through it, passing no arguments.
- `call_library_neg_384_with_stack_arg`: Loads the first stack argument into a0, fetches the library base from a4 offset and jumps to its negative 384 vector.
- `call_library_neg_246_with_two_args`: Loads a stack pointer into a1 and two stack words into d0-d1, then jumps through the library vector at negative 246.
- `jump_through_a6_offsets_186_and_1d4`: Loads a0 and d0-d2 from the stack, then jumps through the library base at a4 offset to vector offsets -390 and -468, passing a0-a1 and d0.
- `jump_through_a6_offsets_c0_and_1c8`: Loads a0-a1 and d0 from the stack, then jumps through the library base at a4 offset to vector offsets -192 and -456.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `jump_through_a6_offsets_42_4e_54`: Loads a0 or a0-a1 from the stack, then jumps through the library base at a4 offset to vector offsets -66, -78 and -84.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `jump_three_amiga_library_vectors`: Loads a0-a1 and d0-d1 from the stack, then jumps through three library vectors at offsets -114, -198 and -204 from the library base in a4.
- `jump_three_amiga_library_vectors_with_d0`: Loads a0-a1 and d0-d1 from the stack, then jumps through library vectors at offsets -216, -282 and -300, passing a0 and d0 to the second.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_runtime_segment_start`: An unconditional jump to the start of the runtime segment, transferring execution there without touching any registers or memory.
- `jump_to_sound_channel_setup`: Jumps to the routine at the fixed target address, transferring control without touching registers or memory itself.
- `jump_to_second_runtime_entry`: Jumps directly to another routine in the runtime segment, passing control onward without modifying registers or memory.
- `jump_to_store_two_words_and_clear_long`: Jumps to a routine that stores two words and a zeroed long into offsets relative to the a4 base register.
- `store_param_to_a4_minus_5e`: Jumps to the shared routine that writes the incoming parameter to the longword at offset minus 94 from a4 and clears the longword at offset minus 82.
- `scroll_region_by_pixel_count`: Jumps to the shared routine that scrolls a region in 512-pixel chunks, updating the a4-relative offsets minus 94 and minus 82 by the amount actually moved.
- `update_scroll_offset_and_tile`: Jumps to the shared routine that recomputes the a4-relative offset minus 82 from the scroll position and updates the current tile index at minus 90 when it changes.
- `jump_to_clear_state_and_restart`: Jumps to the routine that, when a state word is zero, sets a counter to nine, clears a pointer and re-enters with an adjusted base.
- `jump_to_allocate_and_init_buffers`: Jumps to code that allocates a block, stores the result, sets two pointers to a buffer and writes three hardware registers.
- `jump_to_byte_stream_sender`: Jumps to a routine that waits for a status bit, then writes each byte of a counted buffer to a hardware register.
- `jump_store_high_word_to_a4_offset`: Jumps to a fixed routine that stores the high word of the argument into memory at offset minus 22324 from a4.
- `jump_lowercase_ascii_byte`: Jumps to a fixed routine that converts an uppercase ASCII byte in the second byte of the argument to lowercase and returns it.
- `jump_multiply_d0_d1_words`: Jumps to a fixed routine that multiplies the 16-bit halves of d0 and d1 and returns the combined 32-bit product in d0.
- `jump_to_fixed_runtime_address`: Transfers control unconditionally to a fixed address in the runtime segment, acting as a one-instruction tail jump to another routine.
- `jump_to_shared_runtime_entry`: Performs an unconditional jump to a shared code entry point in the runtime segment, transferring control without saving registers or returning.
- `jump_to_indirect_dispatch_entry`: Performs an unconditional jump to a shared dispatch entry that may call a function pointer stored relative to the a4 base register.
- `jump_via_a4_offset_2dc8_table`: Loads a pointer from the a4-relative table at offset 11720 and jumps to the entry at minus 42 from it, transferring control indirectly.
- `jump_via_a4_offset_2dc4_table`: Loads a pointer from the a4-relative table at offset 11716 and jumps to the entry at minus 216 from it, transferring control indirectly.
- `jump_via_a4_offset_2dc4_far_entry`: Loads a pointer from the a4-relative table at offset 11716 and jumps to the entry at minus 414 from it, transferring control indirectly.
- `jump_via_a4_offset_table_entry`: Loads a pointer from the table at offset 11716 of a4, indexes it by minus 198, and jumps to the resulting code address.
- `jump_via_a4_table_minus_0x84`: Loads a pointer from the table at offset 11716 of a4, indexes it by minus 132, and jumps to the resulting code address.
- `jump_via_a4_table_minus_0xd2`: Loads a pointer from the table at offset 11716 of a4, indexes it by minus 210, and jumps to the resulting code address.
- `jump_via_a4_offset_minus_552`: Loads the pointer at offset 11716 from the a4 base, then jumps through the function pointer at offset minus 552 within that structure.
- `jump_via_a4_offset_minus_138`: Loads the pointer at offset 11716 from the a4 base, then jumps through the function pointer at offset minus 138 within that structure.
- `jump_via_a4_offset_minus_378`: Loads the pointer at offset 11716 from the a4 base, then jumps through the function pointer at offset minus 378 within that structure.
- `jump_via_a4_table_offset_neg_1ce`: Loads a pointer from the a4-relative table at offset minus 1ce and jumps indirectly through it, transferring control to that target.
- `jump_via_a4_table_offset_neg_0f6`: Loads a pointer from the a4-relative table at offset minus f6 and jumps indirectly through it, transferring control to that target.
- `jump_via_a4_table_offset_neg_11a`: Loads a pointer from the a4-relative table at offset minus 11a and jumps indirectly through it, transferring control to that target.
- `jump_to_jumptable_dispatch_entry`: Performs an unconditional jump to a shared entry that dispatches through a jump table indexed via the a4 base register.
- `jump_to_library_vector_via_a4`: Jumps to an indirect code pointer fetched through the a4-relative library base table, transferring control without returning.
- `jump_to_second_library_vector`: Jumps to another indirect code pointer reached through the a4-relative library base table, handing control to that target.
- `jump_to_library_vector_offset_1c8`: Performs an unconditional jump to a fixed code address, which then dispatches through a library vector at offset minus 456 from the base pointer in a4.
- `jump_to_library_vector_offset_132`: Performs an unconditional jump to a fixed code address, which then dispatches through a library vector at offset minus 306 from the base pointer in a4.
- `jump_to_library_vector_offset_4e`: Performs an unconditional jump to a fixed code address, which then dispatches through a library vector at offset minus 78 from the base pointer in a4.
- `jump_via_a4_offset_minus_54`: Loads a pointer from the a4-relative table at offset minus 952, then jumps indirectly through the entry at offset minus 84.
- `jump_via_a4_offset_minus_72`: Loads a pointer from the a4-relative table at offset minus 952, then jumps indirectly through the entry at offset minus 114.
- `jump_via_a4_offset_minus_c6`: Loads a pointer from the a4-relative table at offset minus 952, then jumps indirectly through the entry at offset minus 198.
- `jump_via_a4_offset_minus_cc`: Loads a pointer from the structure at a4 minus nine hundred fifty-two, then jumps to the code at offset minus two hundred four from that pointer.
- `jump_via_a4_offset_minus_d8`: Loads a pointer from the structure at a4 minus nine hundred fifty-two, then jumps to the code at offset minus two hundred sixteen from that pointer.
- `jump_via_a4_offset_minus_11a`: Loads a pointer from the structure at a4 minus nine hundred fifty-two, then jumps to the code at offset minus two hundred eighty-two from that pointer.
- `jump_via_a4_vector_table`: Loads a code pointer from a table indexed by the a4 base register and jumps to it, transferring control to another routine.
