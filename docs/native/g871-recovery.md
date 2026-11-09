# Game 871 (grid start (1987)(anco)[cr a-ha][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00871). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 44 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 30 raw-word lines |
| Functions named and described | pass, 432 of 432 named (17 author symbols), 432 of 432 described (36 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 552 of 552 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 5632 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8254 of 8258 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 592339 |
| code | 31936 |
| data | 134546 |
| padding | 11227 |
| unused | 131072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| stack | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| loadWb | 11 | 261 | 15 |
| icon.library | 15 | 1414 | 108 |
| Grid Start (1987)(Anco)[cr A-Ha][a].Adf:copy | 2 | 117 | 2 |
| Grid Start (1987)(Anco)[cr A-Ha][a].Adf:Grid Start | 35 | 5935 | 295 |
| Grid Start (1987)(Anco)[cr A-Ha][a].Adf:bootblock | 1 | 180 | 6 |
| Grid Start (1987)(Anco)[cr A-Ha][a].Adf bytes 5632-11264 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- grid start (1987)(anco)[cr a-ha][a]/Grid Start (1987)(Anco)[cr A-Ha][a].Adf: file            (header 5124 bytes, blocks hold 5368): its blocks are judged as raw blocks outside the filesystem
- grid start (1987)(anco)[cr a-ha][a]/Grid Start (1987)(Anco)[cr A-Ha][a].Adf: bytes 5632-11264 look like code no run loaded: recovered position-independently (load address unknown)
- grid start (1987)(anco)[cr a-ha][a]/Grid Start (1987)(Anco)[cr A-Ha][a].Adf: 1 logged loads cover 5632 bytes of the image
- 1 never-loaded region(s), 5632 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_20_bytes_setup_cia_and_audio` (Grid Start): Copies twenty bytes from a fixed source to the caller's buffer, clears two bits of CIAB port B, sets both disk control ports to all ones, stores a shifted bit and a longword in the buffer, then programs the custom chip audio and disk registers. Kind: track/sync check. Pass path: the routine sets up CIA/Audio registers to check for a specific hardware-dependent signal or timing, likely a track or sync check or a specific disk-related hardware state (CIAB_PRB), where a successful read or presence of a certain signal .
- `set_ciaa_pra_bit1_and_poll_fire` (Grid Start): Reads CIAA port A at a fixed address, sets bit 1, writes it back, then reads CIAA port A at a fixed address, ORs with a fixed address and increments to test the fire button, branching to 658 if nonzero, otherwise pushes a6 offset 8 and calls 1392 before branching to 628. Kind: disk hardware/sync check. Pass path: the loop terminates and execution proceeds to u6__sub_2_02BE(15000).
- `compare_low_bits_and_clear_ciab_bit2` (Grid Start): Calls the step routine, compares the stored low bits, and when the current bit is zero clears bit 2 of CIAB port B before branching onward. Kind: track/sector or checksum-based timing/error check. Pass path: the branch where iVar2 == iVar1 (no subtraction/looping or delay needed).
- `set_ciab_bit2_and_compare_shifted_halves` (Grid Start): Sets bit 2 of CIAB port B, waits 200 units, then compares the shifted halves and clears bit 1 of CIAB port B when the current half is smaller. Kind: checksum/difference trap. Pass path: iVar2 == iVar1 (no branch taken, no modification to DAT_00bfd100 or subtraction logic executed).
- `pulse_ciab_motor_line_and_delay` (Grid Start): Reads CIAB port B, clears bit 0, writes it back, then sets bit 0 and writes again, then calls a delay routine with 3000 and returns. Kind: track/sync check (motor/head delay). Pass path: the routine forces a specific head/track state (bit 0 or certain bitset) and waits for a delay, typical of a check that requires a specific physical delay or head-settle time before reading or proceeding..

## Functions


### stack

- `allocate_stack_node_and_copy_context`: Calls library vectors through a5 to find the highest stack size, allocates a node, fills it with marker words, copies sixteen longs from the caller's context and returns zero or minus one.
- `return_d1_as_d0_then_jump`: Moves d1 into d0 as the return value and branches to the instruction at offset 260, continuing execution there rather than returning.

### endcli

- `endcli_stack_node_setup`: Same stack-node allocation and context copy as the stack variant, using the endcli base offsets, returning zero on success or minus one on failure.
- `copy_d1_to_d0_then_branch`: Moves the longword in d1 into d0 and branches forward to another routine, transferring control without returning here.

### echo

- `echo_command_main`: Reads the argument list, finds the highest argument number, allocates and fills a buffer with argument pointers, copies the parsed argument block, and returns zero or minus one on failure.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0, then branches unconditionally to another routine at a fixed offset.

### loadWb

- `parse_workbench_startup_args`: Saves a7, d0 and a0, opens the current directory, reads the task's startup argument string, splits it into whitespace-separated words stored in a buffer, then opens and initializes the workbench library and returns zero.
- `close_libs_forbid_and_reply_startup`: Restores the saved stack pointer, closes the workbench library, forbids the task, replies the startup message, and on failure raises an alert and returns 100.
- `open_and_close_workbench_library`: Loads the workbench library base from offset 92 of a4, opens it via the exec library vector, then immediately closes it again and returns.
- `clear_workbench_task_slot`: Clears the longword at absolute address 8, loads the address of a resident tag name into a1, calls the dos.library function at offset -552 with d0 set to 30, then stores the returned pointer at address 8.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `start_workbench_tool_from_tag`: Saves d2-d5, checks the argument count, compares a resident tag name, starts a tool via library calls, computes a task structure offset from the count, and reports errors through the debug print and delay routines.
- `compare_two_strings_lexically`: Compares the two string pointers passed on the stack byte by byte, returning zero when equal, minus one when the first sorts lower, and one when it sorts higher.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_vector_with_string_arg`: Saves d2, loads the first stack argument into d2, pushes a pointer to the second argument and the value at absolute address 20, then calls the function pointer at address 0 and restores the stack and d2.
- `format_string_via_rawdofmt`: Saves a2-a4/a6, allocates 140 bytes on the stack, calls exec RawDoFmt to format the string into that buffer, measures its length, and passes the buffer and length to another routine.
- `store_d0_byte_to_a3_postincrement`: Writes the low byte of d0 to the address in a3 and advances a3 by one byte, then returns to the caller.

### icon.library

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_open_count_clear_flag`: Increments the 16-bit counter at offset 32 of the library base in a6 and clears bit 3 of the byte at offset 14, then returns the base pointer in d0.
- `decrement_open_count_maybe_close`: Returns zero in d0, decrements the 16-bit counter at offset 32 of the a6 library base, and if it reaches zero with bit 3 of byte offset 14 set, calls the close routine.
- `close_library_if_not_open`: Checks the 16-bit word at offset 32 of the a6 library base; if zero, calls the library close vector at offset -210 through a6, unlinks the node from its list, and returns the saved value.
- `set_icon_library_flag_bit3`: Sets bit 3 of the byte at offset 14 from the library base in a6, then returns zero in d0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_and_store_base`: Saves a2, stores a0 and a6 into the new structure, opens dos.library via exec, stores the result, and alerts on failure.
- `store_structure_fields_and_return_pointer`: Copies the two longwords at offsets 34 and 38 of a2 into absolute globals, returns a2 in d0 and pops a2.
- `zero_word_data_slot_a`: This single zero word decodes as an ori.b immediate instruction, but it is a zero-filled data slot referenced by address, not executable code.
- `icon_library_zero_fill_data`: A single zero word that decodes as an ori.b instruction; this library range is zero-filled data referenced by address, not executable code.
- `save_a0_a6_call_target_restore`: Pushes a0 and a6 onto the stack, calls the routine at address zero, then adds eight to the stack pointer and returns to the caller.
- `icon_library_call_aa_save_a0_a1`: Saves a0-a1 and a6 on the stack, calls the library routine at offset aa, then discards the saved registers and returns.
- `icon_library_call_124_save_a0_a2`: Saves a0-a2 and a6 on the stack, calls the library routine at offset 124, then discards the saved registers and returns.
- `icon_library_call_3fe_save_a0_a1`: Saves a0-a1 and a6 on the stack, calls the library routine at offset 3fe, then discards the saved registers and returns.
- `icon_library_call_666_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 666, then discards both saved registers and returns.
- `icon_library_call_698_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 698, then discards both saved registers and returns.
- `icon_library_call_6b2_with_a6`: Pushes a6 onto the stack, calls the library routine at offset 6b2, then discards the saved register and returns.
- `icon_library_save_regs_call_helper`: Pushes a0 through a2 and a6 onto the stack, calls a library helper, then discards the saved registers and returns.
- `icon_library_call_with_a0_a6`: Pushes a0 and a6 onto the stack, calls a library helper, adds eight to the stack pointer to drop them, and returns.
- `icon_library_call_second_helper`: Pushes a0 and a6 onto the stack, calls a different library helper, adds eight to the stack pointer to drop them, and returns.
- `icon_library_call_842_with_a0_a1`: Saves a0 and a1 on the stack, calls the library routine at offset 842, then discards the saved registers and returns to the caller.
- `icon_library_call_88a_with_a0_a1`: Pushes a0 and a1 onto the stack, invokes the library routine at offset 88a, removes the two saved registers and returns.
- `icon_library_call_8fe_with_a0_a1`: Stores a0 and a1 on the stack, jumps to the library routine at offset 8fe, then adjusts the stack past the saved registers and returns.
- `set_icon_object_user_data`: Calls the library vector at offset -294 with the library base, stores the second argument into offset 148 of the returned object, and returns.
- `zero_filled_data_area`: A single zero word that decodes as ori.b with immediate zero; this is zero-filled data, not executable code.
- `load_icon_from_disk_into_object`: Looks up an icon object, reads its definition from disk into a stack buffer, copies 44 bytes and several fields into the object, and links it into its parent list.
- `copy_icon_fields_to_new_object`: Copies 44 bytes from the library base offset into a new object at a2, then copies several longword fields and links the object into a list via offset 424.
- `return_zero_restore_registers`: Sets d0 to zero as the return value, restores d2-d4 and a2 from the stack, releases the a6 frame and returns.
- `copy_icon_to_stack_and_dispatch`: Copies 44 bytes and several fields from the icon structure at a0 into a stack buffer, then calls another routine with two arguments and the buffer.
- `icon_library_open_and_parse_header`: Sets up a stack frame, saves registers, calls three library helpers to build a name buffer and open the icon file, then branches to the parse continuation or returns zero.
- `icon_library_validate_and_load_icon`: Calls a helper with the icon structure, checks the magic word and version, clears two longwords, then loads image and mask data via further helper calls.
- `icon_library_load_image_and_mask_data`: Calls helpers to load two image pointers, then conditionally allocates and reads image and mask buffers into the icon structure, returning success or failure.
- `icon_open_four_entries_and_optional_extra`: Calls library hooks through a4 and d4 to open four icon entries stored via a3, decrements a counter at -4(a6), then optionally opens a fifth entry stored at 70(a2) and returns 1 in d0, or 0 on failure.
- `set_d6_zero_and_branch_to_epilogue`: Loads zero into d6 and branches to the shared epilogue that restores registers and returns, signalling failure to the caller.
- `restore_registers_unlink_and_return`: Pops d2 through d6 and a2 through a5 from the stack, releases the a6 frame link and returns to the caller.
- `open_icon_library_and_allocate_buffer`: Saves registers, calls the library open routine with size 260, and on failure reports error 103 with the first argument before returning zero.
- `write_icon_header_and_optional_fields`: Writes the icon header, optional image and tool types, and the default tool string to the opened file, restoring the saved pointer at offset 26 of the icon structure.
- `write_icon_image_and_tooltype_data`: Writes the icon image data and tool type strings to the file, then restores the saved pointer at offset 26 of the icon structure and returns the status.
- `scan_icon_list_and_call_hook`: Advances a4 and the counter at -8(a6) by four while the longword at a4 is nonzero, then calls the a3 hook with d3, d4 and the counter, returning the hook result in d5.
- `process_icon_list_with_hook_calls`: Walks the a4 list calling the a3 hook for each entry, also handles the pointer at 70(a2), then calls two library routines and stores the result in 26(a2) before restoring registers.
- `iterate_icon_entries_via_library_calls`: Loads the argument from 12(a7) into d0 and the pointer from 16(a7) into a0, advances a0 by two, then calls library routines on it until one returns zero.
- `pop_d2_d3_and_return`: Pops registers d2 and d3 from the stack and returns to the caller; an empty library stub that only restores saved registers.
- `call_icon_offset_8c_with_arg`: Saves d2, loads the second argument into d2 and the first into a0, pushes a0 plus offset 140, calls the library routine at the pc-relative target, then cleans the stack and restores d2.
- `allocate_and_init_icon_node`: Saves d2-d4 and a2, allocates 65536 bytes via the memory routine, then initializes the new block by calling routines on offsets 142 and 140, freeing it if initialization fails, and returns the block pointer or zero.
- `open_icon_by_name_or_return_null`: Pushes a2, a name pointer and d2, calls the open routine at offset 1838, cleans the stack, and returns a2 in d0 if opening succeeded, otherwise branches to the shared null return.
- `return_zero_and_restore_registers`: Sets d0 to zero as the failure return value, restores d2 through d4 and a2 from the stack, and returns to the caller.
- `decrement_icon_use_count_and_store_entry`: Decrements the word count at a2, calls the release routine when it reaches zero, then stores d3 and d2 into the icon entry at the indexed offset and returns one.
- `icon_remove_last_entry`: Decrements the word count at (a2), then stores d2 and the caller's stack argument into the two longwords of the indexed entry in the structure pointed to by 10(a2), returning 1 in d0.
- `icon_allocate_and_attach_icon`: Allocates a 94-byte block, initializes fields at offset 78, then calls library helpers to attach the icon to the given object and window, freeing the block and returning zero on any failure.
- `icon_return_allocated_block`: Moves the allocated block pointer from d6 into d0 as the return value and branches forward to the common exit path.
- `icon_library_close_and_return_zero`: Pushes a word and d6 onto the stack, calls a library routine through a jump table, then clears d0 to zero, discards the pushed arguments and restores d2-d6/a2 before returning.
- `icon_library_call_with_offset_argument`: Saves d2-d4, loads two arguments from the stack, adds 78 to the second argument, pushes both and calls a library routine before restoring registers and returning.
- `icon_library_find_equals_in_name_list`: Saves registers, converts the second argument via a library call, then walks a null-terminated pointer list comparing each name's byte at the converted length against an equals sign, returning the character after it or zero.
- `return_zero_and_restore_registers_2`: Sets d0 to zero as the return value, then pops d2, d3, a2 and a3 from the stack and returns to the caller.
- `search_string_for_substring`: Saves d2-d6, calls library helpers to measure the needle and scan the haystack, compares lengths and returns one on a match or zero otherwise.
- `verify_substring_at_position`: Calls a helper to measure the string in d2, compares it with d5, then calls a comparison helper and returns one if equal, otherwise continues.
- `increment_d4_into_d0_and_d2`: Tests d4; if zero branches to the empty return path, otherwise copies d4 to d0, adds one to d0, stores d0 in d2 and branches back to the caller's loop.
- `zero_d2_and_branch_back`: Loads zero into d2 and branches back to the shared caller loop, providing a zero result when d4 was zero.
- `return_zero_restore_d2_d6`: Sets d0 to zero as the return value, pops d2 through d6 from the stack and returns to the caller.
- `parse_icon_tooltypes_into_buffer`: Saves registers, copies two argument pointers, clears byte 30 of the first buffer, then calls library helpers to parse tooltype strings and fill the buffer, returning the first pointer.
- `advance_pointer_and_parse_three_chars`: Adds five to a2, calls a library helper with constants and a2, tests the result, and on success sets d5 to one and advances a2 by three.
- `set_d5_zero_and_branch`: Sets d5 to zero and branches forward into another routine, providing the failure path result.
- `parse_decimal_number_from_string`: Accumulates digits from the string at a2 into d5 as a decimal value, calls a digit-conversion routine per character, then formats and stores the result via library calls before returning d3.
- `convert_ascii_digit_to_value`: Loads the argument from the stack into d1, checks it lies between the characters zero and nine, and returns its numeric value in d0 or minus one otherwise.
- `return_minus_one_constant`: Loads minus one into d0 and returns immediately, serving as the failure result for the digit conversion routine.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `icon_compare_two_objects`: Saves d2-d5, calls the object comparison helper on two argument pairs, and if the first pair matches, calls the second comparison and frees the first object when it fails, returning the result in d0.
- `create_icon_with_ten_planes`: Saves d2-d3/a2, allocates a 96-byte icon with 65536-byte memory, sets word 10 at offset 14 and at the caller's pointer, then stores the icon pointer at offset 2 and returns nonzero.
- `return_zero_restore_registers_2`: Sets d0 to zero as the return value, restores d2, d3 and a2 from the stack, and returns to the caller.
- `icon_compare_object_with_second_helper`: Saves d2-d3, calls the first comparison helper with three arguments, and returns its result in d0 only if it equals the fourth argument, otherwise returns zero.
- `icon_return_saved_result_in_d0`: Moves d1 into d0 as the return value, restores d2-d3 from the stack and returns to the caller.
- `icon_compare_object_with_third_helper`: Saves d2-d3, calls the second comparison helper with three arguments, and returns its result in d0 only if it equals the fourth argument, otherwise returns zero.
- `return_d1_in_d0_restore_d2_d3`: Copies the longword in d1 to d0 as the return value, restores d2 and d3 from the stack, then returns to the caller.
- `divide_32bit_d1_by_d2`: Divides the 32-bit value in d1 by d2, using a 16-step shift-subtract loop when d2 exceeds 65535, and returns quotient and remainder in d0 and d1.
- `divide_32bit_shift_subtract_loop`: Divides the 32-bit value in d1 by d2 using a 16-iteration shift-subtract loop, leaving the quotient in d1 and remainder in d0.
- `mul32x32_high_shift_to_a0`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, combines them into a 64-bit result, shifts right by 16, clears the low word and returns the resulting address in d0 and a0.
- `format_string_via_rawdofmt_2`: Saves a2-a4 and a6, loads the format string and argument pointer from the stack, sets a2 to a local format template, calls exec RawDoFmt through a6, then restores registers and returns.
- `store_byte_to_a3_postincrement`: Writes the low byte of d0 to the memory pointed to by a3 and advances a3 by one byte, then returns to the caller.
- `strlen_count_until_null`: Loads the string pointer from the stack, scans bytes until a null terminator or 256 bytes, and returns the complemented count in d0.
- `find_first_matching_byte`: Reads a string pointer and a target byte from the stack, scans bytes until null or a match, and returns the pointer to the match or zero.
- `return_zero_and_branch_to_shared_exit`: Sets d0 to zero and branches forward into the shared exit path of the string search routine.
- `copy_string_until_nul`: Loads two pointers from the stack, copies bytes from the source to the destination one at a time, and returns after copying the terminating zero byte.
- `skip_string_to_terminator`: Loads two pointers from the stack, advances the first pointer past bytes until a zero byte is found, steps back one, then branches to shared code.
- `compare_strings_with_length_limit`: Loads two pointers and a length from the stack, compares bytes one by one up to that count, returns zero on full match or stops at a mismatch or zero byte.
- `return_one_or_minus_one_flag`: Loads 1 into d0, then a conditional branch skips the next instruction; otherwise loads -1 into d0 and returns to the caller.
- `scan_string_until_nul_or_count`: Loads a0 and a1 from the stack, reads a count from the stack into d0, then advances a0 past bytes until a zero byte or the count runs out.
- `decrement_a0_and_call_next`: Subtracts one from a0, calls the following library routine, then returns to the caller.
- `copy_bytes_until_zero_or_count`: Decrements d0, then copies bytes from (a1) to (a0) until a zero byte or count exhausted, padding with a zero byte if needed.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `init_self_referential_list_header`: Loads the pointer from 4(a7) into a0, stores a0 at (a0), adds 4 to that value, clears 4(a0), and stores a0 at 8(a0).
- `draw_icon_with_offsets`: Saves registers, validates the icon pointer, calls a rectangle fill with computed width and two offset values, then calls a second fill using the icon's offset field.
- `walk_icon_list_validate_entries`: Saves d2-d5/a2, reads three arguments from the stack, then walks a linked list of icon nodes, calling a validation routine on each node header and its data block, returning 1 on success or 0 on failure.
- `return_one_after_null_list_check`: Loads the value 1 into d0 and branches forward into the shared epilogue, returning success when the icon list pointer was null.
- `return_zero_restore_registers_3`: Sets d0 to zero as the failure return value, restores d2-d5 and a2 from the stack, and returns to the caller.
- `allocate_and_load_icon_bitmap`: Pushes registers, allocates two 20-byte structures via library calls, computes a rounded even bitmap size from the icon header, allocates and loads the bitmap, and returns the first structure or -1.
- `return_minus_one_restore_registers`: Sets d0 to -1 as the failure return value, pops d2 through d6 and a2 from the stack, and returns to the caller.
- `allocate_and_chain_icon_list`: Pushes registers, allocates 16-byte nodes via library calls, links them through offset 12, allocates and loads each node's bitmap sized from its header byte, and returns the list head or -1.
- `icon_alloc_and_link_node`: Calls library allocation routines to create a node, stores the returned pointer at offset 8 of the node, copies the caller's pointer into offset 12, and loops to link further nodes until a null link is found.
- `return_a3_in_d0_and_branch`: Moves the value in a3 into d0 as the return value and branches forward to the shared epilogue that restores registers and returns.
- `return_minus_one_restore_registers_2`: Sets d0 to minus one as the failure return value, restores d2 through d7 and a2 through a3 from the stack, and returns to the caller.

### Grid Start (1987)(Anco)[cr A-Ha][a].Adf:copy

- `allocate_and_init_glc_script`: Reads the Glc script header, finds the largest command value, allocates a buffer, fills it with incrementing command words, stores a return address and stack frame, then copies the script into the buffer and returns zero or minus one.
- `return_d1_in_d0_then_jump`: Moves d1 into d0 as the return value and branches to the address at offset 104, continuing execution in the caller's code.

### Grid Start (1987)(Anco)[cr A-Ha][a].Adf:Grid Start

- `grid_start_program_entry`: Saves d0 and a0, opens the current directory, finds the task, builds a command line from the task name and arguments, opens the startup file, loads the program, and jumps to the loaded code.
- `shutdown_and_release_resources`: Calls an optional callback, invokes a cleanup routine, closes two library resources via a6 offsets, frees two more, then restores registers and returns the saved d0.
- `open_and_close_console_device`: Loads the address of a structure at offset 92 from a4 and calls two library functions through a6 to open then close that device.
- `open_dos_library_store_handle`: Loads a name pointer, sets d0 to zero, calls the dos library open function through a6, stores the returned handle in memory and returns if nonzero.
- `open_four_libraries_and_run_game`: Saves the exec base pointer, opens dos, graphics, intuition and layers libraries via OpenLibrary, stores each base, and branches to the main game loop or cleanup if any open fails.
- `zero_filled_data_area_b`: Contains a single zero word that decodes as ori.b with zero immediate; this is zero-filled data referenced by address, not executable code.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `zero_word_data_at_00b8`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `clear_register_d0`: Sets register d0 to zero.
- `main_game_loop_setup_and_frame_cycle`: Initializes audio and DMA registers, calls many game subsystems, waits on vertical blank and timer, swaps double-buffer pointers and loops the frame cycle until exit.
- `close_screens_and_chain_next_handler`: Tests two global screen pointers and closes each non-null screen via intuition.CloseScreen, then jumps to the next handler pointer at low memory.
- `blink_dma_and_advance_frame_counter`: When the frame flag is set, toggles DMACON and INTREQ bits, increments the frame and sub-frame counters, and resets the animation position when the sub-frame exceeds forty-three.
- `center_sprite_from_frame_table`: Reads the current frame index, fetches the frame entry from the table plus base offset, and stores centered x and y coordinates into the sprite structure.
- `setup_sprite_pointer_and_check_index`: Loads a base pointer, adds 1604 to a global offset, stores the result at offset 72 and clears word at offset 68; returns zero if a global word is at most 17, else returns minus one.
- `position_object_from_table_entry`: Calls the setup routine, returns early if it fails, clears word at offset 70, reads a table word indexed by d1 into offset 12 and computes 200 minus the next table word into offset 20.
- `position_object_mirrored_from_table`: Calls the setup routine, returns early if it fails, sets word at offset 70 to one, then stores 320 minus a table word at offset 12 and 200 minus the next table word at offset 20.
- `setup_sprite_frame_from_table`: Calls a subroutine, then on nonzero result stores 2 to offset 70 of a0, reads a word from table at 7014 indexed by d1 into d2, stores it at offset 12, subtracts the next table word from 200 and stores at offset 20.
- `init_velocity_from_track_offset`: Calls a subroutine, and if it returns nonzero writes 3 to offset 70 of a0, then computes two velocities by subtracting table words indexed by d1 from 320 and 200 and stores them at offsets 12 and 20 of a0.
- `init_object_from_sprite_table_entry`: Sets object field 72 to a base plus 1424, clears field 68, clamps field 70 to half of the caller's field 70, then stores centered sprite x and y offsets into fields 12 and 20.
- `allocate_two_800_byte_buffers`: Loads the library base from pc-relative data into a6, then twice calls the allocator with size 2048 and type 2, storing each result at two longword globals and branching away if either allocation returns zero.
- `update_audio_volume_from_speed`: If the speed global is nonzero, calls the sound update, scales speed by a multiplier, clamps the result, writes the volume to AUD1VOL at 184(a0), and clamps the stored volume between the two limits.
- `advance_sound_sequence_and_spawn_voices`: Steps the current sound sequence pointer, updates pitch and volume globals, and when a new note marker is reached allocates a voice structure and fills its channel pointers and parameters.
- `allocate_and_init_entity_from_template`: Calls the allocator, stores the returned pointer globally, sets type 10, links a template, then copies offset-adjusted template longs into the entity's 72-byte area and clears word 70.
- `spawn_23_randomized_entities`: Loops 22 down to zero allocating entities, sets type 1 and a template pointer, calls the random range routine, and on odd results copies offset-adjusted template longs and sets fields 44, 2 and 8.
- `offset_template_data_table`: Contains only immediate ori.b instructions with increasing byte values, so these bytes are a data table of offsets, not executable code.
- `ori_immediate_sequence_data_table`: Eight ori.b immediate instructions that OR fixed byte values into d0 in sequence; the bytes are data decoded as code, not an executable routine.
- `ori_immediate_sequence_data_table_b`: Eight ori.b immediate instructions ORing different fixed byte values into d0; these bytes are data decoded as code, not executable code.
- `check_keyboard_serial_for_escape`: Tests a flag bit, reads the CIAA keyboard serial register, rotates and inverts the byte, and if it equals the escape code clears two memory longs.
- `joystick_fire_steering_update`: Checks flag bits in a global word, calls a routine to read joystick movement, then reads JOY1DAT, mixes its bits and calls fire and steering routines based on the resulting bits.
- `fire_button_edge_and_volume_update`: Reads CIAA_PRA, compares the fire button bit with the previous sample and returns all-ones on a new press, otherwise reads joystick movement and updates AUD0VOL or AUD1VOL from the stored value.
- `decrement_three_timers_or_advance_level`: Subtracts one of three fixed amounts from a long counter, clamps it to zero and returns, or increments a level word up to six and sets a flag bit.
- `poll_fire_and_count_down_timers`: Reads the CIA fire button bits, reloads or decrements two word counters and decrements a level word above one, setting flag bits as it goes.
- `set_d7_ff60_d0_ffff0000_jump`: Loads d7 with the value minus one hundred sixty and d0 with the high word all ones, then jumps to the shared routine at de0.
- `set_d7_a0_d0_10000_jump`: Loads d7 with one hundred sixty and d0 with the value sixty-five thousand five hundred thirty-six, then jumps to the shared routine at de0.
- `set_d7_minus_140_clear_d0_jump`: Loads the immediate value minus 140 into d7, clears d0 to zero, then jumps to the shared routine at a fixed low address.
- `set_d7_140_clear_d0_jump`: Loads the immediate value 140 into d7, clears d0 to zero, then jumps to the shared routine at a fixed low address.
- `update_24_objects_and_decrement_counter`: Loops over 24 object entries, subtracts a scroll delta, computes sprite offsets and calls the draw routine four times per visible entry, then decrements a BCD counter when none remain.
- `project_velocity_into_screen_offset`: Adjusts horizontal and vertical deltas, computes a row offset from a vertical step, scales it by a stored value and calls a routine to apply the result.
- `toggle_playfield_bits_along_line`: Walks a line of playfield bytes using the current x offset and slope, toggling one bit per pixel position and skipping positions outside the visible width.
- `plot_line_with_clipping`: Adds the screen base to d1, walks up to one hundred twenty-eight or d7 pixels using the slope in d5, toggling bits in the bitmap at a2 and skipping pixels outside the three-hundred-twenty-wide screen.
- `scan_objects_for_collision_flag`: Walks the object list at the global pointer, finds the first object of type ten, then scans other objects for overlapping bounding boxes and sets bit zero of their flags word when a collision is detected.
- `decay_object_timer_by_speed`: Computes a threshold from the global speed value divided by the object's speed field, compares it with the object's timer at offset 28, and subtracts the object's decrement value, clearing the timer and flag bit zero when it goes negative.
- `apply_gravity_to_object_position`: If the global gravity flag is negative, adds the object's velocity at offset 28 to its position at offset 28, using a fixed downward value when the level counter is at least four, then returns.
- `clamp_entity_x_to_target`: Checks word at offset 28 of the object at a0; if at least 70, moves x at offset 16 toward target at offset 48 by at most an address, snapping to the target when within range.
- `update_ai_entity_movement_and_animation`: For the object at a0, updates timers, velocity, position and animation frame using globals and RangeRan, recursing for sub-entities and returning a status word.
- `iterate_entity_list_dispatch_by_type`: Walks the pointer list at the global table, dispatching each object by its type word to movement, animation or removal code, and returns when a null pointer is found.
- `build_road_curve_scanline_table`: Reads track segment data and the steering offset, computes per-scanline horizontal offsets by repeated division and accumulation, and writes them into the road table at the fixed buffer.
- `rebuild_copper_list_and_palette`: Calls the setup routine, writes a Copper list to the pointer at the copper buffer, clears the color registers, and fills palette and bitplane entries from the track and sprite data.
- `draw_hud_panels_and_score`: Writes Copper instructions for the display window and bitplanes, clears the color registers, then draws the score and status panels using the digit and text blitters.
- `write_copper_list_for_two_rows`: Fills the copper list pointer with display and color register writes, clears eight audio registers, then emits two row entries using the row index in d1 and d2.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `write_copper_list_and_call_display`: Writes display, color and sprite copper entries, clears eight audio registers, then temporarily swaps the copper pointer and calls the display routine before restoring it.
- `build_copper_list_and_set_display_window`: Writes display window start/stop to the custom chip registers, clears eight copper registers, then appends copper move instructions to the list at the pointer in memory, including bitplane and sprite pointers, and terminates with an end marker.
- `build_five_sprite_entries_from_a2`: Saves d0-d2, then for five iterations reads a long from (a2), adds d0, and writes four words (sprite number and split address halves) to (a5), advancing both pointers.
- `copy_thirtytwo_words_with_sprite_numbers`: Writes thirty-two consecutive sprite numbers starting at 384 into (a5), each followed by a word copied from (a3), incrementing the sprite number by two per entry.
- `update_scroll_speed_and_build_sprite_list`: Computes a speed from the long at 4694, accumulates it into 46A6, rolls over into 6702, toggles bit 1 of 468E, then fills a sprite list at 56B2 or 46B2 and stores its end and length.
- `blit_playfield_and_background_layers`: Waits for the blitter to finish, programs blitter control, modulo, pointers and size registers to copy playfield and background bitmaps, calling blit setup routines between passes.
- `blit_single_bitmap_to_screen`: Waits for the blitter to be idle, then programs blitter control, modulo, source pointer and size registers to copy one bitmap to the screen and returns.
- `blit_from_d0_pointer`: Waits for blitter idle, clears BLTCON1 and BLTDMOD, sets BLTCON0 to 256, writes d0 into BLTDPTH and sets BLTSIZE to a fixed value, then returns.
- `set_copper_list_and_swap_screen`: Writes two bytes and two longwords into low memory to build a copper list, then calls the graphics library to swap screens and jumps to the library exit.
- `wait_bovp_setup_display_window`: Waits for the vertical blank via graphics library, programs the display window, DMA and interrupt registers, toggles flags, calls the copper setup and returns the display pointer.
- `copper_beam_and_bcd_frame_update`: Saves registers, updates the copper beam position and bitplane pointers, optionally calls two subroutines, and increments a BCD frame counter before restoring registers and returning zero.
- `zero_filled_data_words`: Two zero words that decode as ori.b instructions; this range is zero-filled data referenced by address, not executable code.
- `single_zero_data_word`: One zero word that decodes as an ori.b instruction; this is zero-filled data referenced by address, not executable code.
- `allocate_and_configure_sound_channel`: Calls a routine to get a channel, stores it, fills channel fields from d1, d2 and d3, swaps the audio pointer, sets interrupt bits and waits for the timer before restoring state.
- `init_six_speed_slots_and_set_entry`: Writes fifteen into six speed words, then uses the current level index to look up a table entry and stores the value 2463 into the selected speed slot.
- `table_of_ori_immediate_words`: Three ori.b immediate instructions on d3, d1 and d7 that are data bytes read as code, not executable logic.
- `poll_fire_and_run_game_steps`: Calls the main update, waits while the fire button bit in CIAA port A is held, then calls four sub-updates, sets a flag bit and waits again before returning.
- `grid_start_main_loop`: Waits for vertical blank, enables DMA, clears input flags, then runs the menu loop that swaps two longwords, resets the selection index and polls fire and joystick until exit.
- `joystick_menu_navigate`: Reads JOY1DAT bits to detect left or right movement, debounces with a flag, steps the menu index between zero and four, redraws the selection and returns.
- `joystick_slider_adjust`: When the menu index is one, reads JOY1DAT to detect up or down movement, debounces, adjusts the slider value between sixty-four and ninety and calls the redraw routines.
- `draw_selected_char_at_cursor`: Reads the cursor column and character code, substitutes a period when the code is 64, writes the character into the screen buffer, waits for the vertical blank and prints the text at the offset screen position.
- `copy_cursor_char_and_dispatch_state`: Copies the character under the cursor into a buffer, clears the following byte, prints it at the shifted screen position, then jumps through a state table entry or returns minus one when the state equals four.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `cycle_slot_and_swap_buffers`: Increments a slot counter wrapping at five until a table entry is not minus one, calls two subroutines, waits for beam and vertical blank, swaps two longword pointers and clears a word flag.
- `toggle_d0_based_on_flag_and_counter`: Sets d0 to 1, tests the longword at 705c; if zero leaves d0 as 1, otherwise sets d0 to 2, increments the word at 6952, and clears that word when it reaches d0.
- `configure_sound_channel_and_jump`: Loads d0 with 4c0, d1 with 8, d2 with 1 and d3 from the word at 6952, then jumps to a shared sound setup routine.
- `toggle_bit_six_at_468e`: Flips bit 6 of the byte at 468e and then falls through into the following routine.
- `set_sprite_geometry_and_jump`: Loads d0 with 1232, d1 with 8, d2 with 12 and clears d3, then tests bit 6 of the long at 18062 and jumps to the shared continuation, setting d3 to 1 if the bit is set.
- `advance_frame_counter_and_dispatch`: Calls the routine at 10584, compares the long at 28208 with 2, and if not equal increments that long and calls the routine at 11056 before jumping onward.
- `read_byte_from_table_into_6e2e`: Loads a1 with the table at 28062, reads the byte indexed by the word at 28208 into d1, and stores 64 into 28206 when that byte equals 46.
- `clear_6e30_and_chain_three_calls`: Clears the word at memory location 6e30, then calls three nearby subroutines in sequence before branching back to an earlier point.
- `countdown_c4_swap_pointers_wait_tokens`: Sets a counter to 196, calls a setup routine, waits for beam and time-of-flight via graphics library, swaps two longword pointers, clears a word, decrements the counter by 4 and loops while non-negative.
- `loop_d0_4_to_200_swap_pointers`: Calls a routine with d0 starting at 4, waits for beam and time-of-flight, swaps two longword pointers, clears a word, then increments d0 by 4 and repeats while at most 200.
- `toggle_buffers_and_wait_bovp`: Waits for vertical blank via graphics library, swaps the two longword pointers at the two buffer globals, clears the scroll word, and loops adding four to d0 until it exceeds two hundred.
- `draw_five_row_score_panel`: Checks a flag bit, calls two setup routines, finds the player index, then loops five times printing score and level numbers with PrintIText into the graphics rastport at increasing vertical positions.
- `print_player_score_entries`: Walks the entry pointer table, and for each entry of type one or ten sets the two text pen pointers, calls PrintIText at the entry coordinates, then restores the default pen pointers and prints the score value.
- `format_unsigned_decimal_into_buffer`: Converts the 16-bit value in d0 to decimal digits in the base given by d1, writing characters to the buffer at a0 and padding with spaces, then returns.
- `clamp_velocity_word_to_range`: Clamps the signed word at the global velocity variable to between 4096 and negative 4096, storing the limit back when exceeded, then returns.
- `adjust_velocity_from_joystick_buttons`: When the enable flag is set, reads joystick port 1 button bits and adds or subtracts 180 and 60 to the velocity word, or decays it toward zero by 180, then returns.
- `allocate_screen_bitmaps_and_palette`: Opens the workbench screen, allocates raster bitmaps and four sprite buffers, builds bitmap pointer tables, opens two windows and a palette, returning zero on any allocation failure.
- `load_palette_from_table_into_screen`: Reads thirty-two packed colour bytes from the palette table, splits each into red, green and blue nibbles and calls SetRGB4 on the screen viewport for each entry.
- `run_game_state_update_chain`: Calls seven state update routines in sequence then jumps to the final one, chaining the per-frame game logic without returning to the caller.
- `draw_three_digit_score_digits`: Reads three score bytes, shifts each high nibble down, and calls the digit drawing routine at screen offsets 29, 36, 45, 52, 61 and 68.
- `draw_two_digit_counter_at_offset_103`: Reads byte at 469c, draws its high nibble at offset 177 and full byte at offset 184 via the digit drawing routine.
- `draw_lives_remaining_digit`: Counts active entries whose threshold exceeds the score, divides by ten, and draws quotient and remainder digits at offsets 96 and 103.
- `compute_scroll_delta_from_speed`: Scales a table byte by the speed word, divides by the frame time, and either stores half the difference as scroll delta or resets delta and saves the new position.
- `draw_score_digits_via_digit_renderer`: Reads the word at the score variable, divides it by 100 then by 10 to split hundreds, tens and units, and calls the digit renderer at columns 202, 209 and 216 for each digit.
- `advance_to_screen_buffer_offset`: Loads the pointer at address 4624, follows its offset 12 field into a0, adds 80 bytes to reach a screen buffer position, then branches to the drawing routine.
- `blit_tile_row_into_playfield`: Uses d0 as a tile index and d1 as a bit offset to fetch a tile bitmap pointer, then merges shifted tile bytes into the playfield at stride 40 for the tile's height, preserving d2 and d3.
- `scroll_playfield_up_one_tile_row`: Compares vertical and horizontal scroll counters divided by eight, and if scrolling is possible subtracts eight from the vertical counter, clears the object's field, updates its position and tile pointer, then calls the scroll routine.
- `reset_scroll_counters_and_draw_column`: Sets the scroll limits and step counters, positions the object at the top, calls the tile drawing routine, then advances the object's vertical position by eight until it passes the bottom limit.
- `step_grid_cursor_toward_target`: Reads the current and target grid positions, divides both by six, and moves the cursor one step toward the target, updating the position word, the sprite offset at offset 12 and the frame counter at offset 70 of the object pointed to by a0.
- `set_object_frame_and_animation_offset`: Stores frame value six at offset 20 of the object in a0, loads the animation base offset into d0 and jumps to the shared animation setup code.
- `start_object_animation_with_wait`: Adds the animation base to d0 and stores it at offset 72 of the object in a0, clears offset 68, swaps the graphics library base, sets the animation flags and delay, calls the library routine at offset minus 270, then runs the animation step and restores the previous library base.
- `allocate_blit_buffers_loop`: Saves a2/a6, computes a size from d2 shifted right three times multiplied by d3, calls the memory allocator with type 10002 for each of d4+1 buffers, stores each pointer at (a2)+ and exits early on allocation failure.
- `wait_blitter_and_set_blit_pointers`: Copies a0 to a2, waits while DMACONR bit 6 is set, clears BLTCON0 and BLTCON1, writes 256 to BLTSIZE, stores a2 into BLTDPTH and 1448 into BLTDMOD, then returns.
- `scale_d1_difference_by_d2`: Subtracts d0 from d1, returns zero if negative, shifts the difference left five times and calls a scaling routine, then computes d2 times three and, if d4 is at most an address, multiplies that result by three again before returning.
- `compute_scaled_velocity_from_d0_d2`: Calls the speed lookup, scales the global velocity by the horizontal position, applies the direction flag and returns the signed result in d0.
- `lookup_speed_from_position_table`: Reads the speed divisor from the table indexed by the mode, scales the position value, interpolates between table entries and returns the result shifted left by eight in d0.
- `refine_square_root_in_d2`: Stores d1 in the global scratch words, builds an initial estimate in d2 and iterates division to refine it, leaving the square root in d2.
- `open_examine_read_file_into_buffer`: Locks and opens the named file, examines it, reads its contents into the buffer at the loaded module base plus eighty kilobytes, closes and unlocks it, and returns the buffer address or zero on failure.
- `open_read_file_and_alloc_buffer`: Opens and examines the named file, reads its header, asks the library for a 2-type allocation, then opens the file again and reads 1005 bytes into the allocated buffer, returning the buffer pointer or zero on failure.
- `loop_alloc_memory_and_call_3998`: Repeats 70 times: calls the library allocation function with size 156 and type 65536, and when it succeeds passes the returned pointer in a0 to the routine at 3998.
- `dispatch_pending_object_via_two_helpers`: Saves d1-d6 and a0-a6, loads the pointer at the global slot, and if non-zero calls the first helper with it in a0 then the second helper, then restores registers and returns.
- `load_a1_with_table_then_branch`: Loads the address of the data table into a1 and branches unconditionally to the following drawing code, which uses a1 as its source pointer.
- `append_pointer_to_first_free_slot`: Scans the global pointer array from its start for the first zero longword, stores a0 there, and returns; a1 ends just past the written slot.
- `remove_matching_entry_shift_list`: Scans the longword list at a1 for the value in d0, then shifts the following entries down over the match until a zero terminator, preserving d0 and a1-a2.
- `build_sorted_entity_offsets_table`: Stores a global value at offset 56 of the object at the pointer in a0, calls the sort routine, then walks the pointer list filling three word tables with indices and cumulative offsets from each entry's field.
- `bubble_sort_pointer_list_by_offset`: Repeatedly compares offset 56 of adjacent pointers in the list at the global base, swapping out-of-order pairs and setting bit 0 of d0 until a pass makes no swaps.
- `bubble_sort_list_by_word_offset8`: Walks the pointer list at the global table, swapping adjacent entries when the word at offset 8 of the first is less than the second, repeating passes until no swap occurs.
- `bubble_sort_list_by_long_offset40`: Walks the pointer list at the global table, swapping adjacent entries when the long at offset 40 of the first is less than the second, repeating passes until no swap occurs.
- `bubble_sort_list_by_word_offset20`: Walks the pointer list at the global table, swapping adjacent entries when the word at offset 20 of the first is less than the second, repeating passes until no swap occurs.
- `open_top_scores_read_and_close`: Opens the topScores file for reading, reads its contents into a buffer, and closes the file handle, returning after cleanup.
- `insert_score_into_top_scores_table`: Clears the selected-entry pointer, opens topScores, and if conditions match, shifts table entries to insert the new score, then writes and closes the file.
- `draw_top_scores_entries_with_highlight`: Loops over score table entries, sets highlight color for the selected entry, formats each score into a buffer and prints it via PrintIText.
- `open_bobs_file_and_alloc_buffer`: Locks and examines the bobs file, reads its size into a global, then allocates a 65538-byte buffer and stores the pointer; returns after the final rts.
- `open_bobs_seek_and_read_chunk`: Opens the bobs file, seeks to offset an address, reads a chunk into the buffer at the stored pointer plus an address, then closes the file and jumps to the close return.
- `open_bobs_read_into_buffer_and_close`: Opens the bobs file, reads an address bytes into the buffer pointer stored in memory, then closes the file and jumps to the close return.
- `load_anim_table_and_store_result`: Loads the address of animTable into a0, calls the routine at pc-relative offset, then stores the returned d0 into the global at 6a7c and returns.
- `run_animation_frame_sequence`: Sets a flag, walks the animation table at 6a80 calling per-entry routines, then conditionally invokes several subroutines with a6 pointing at the 6cb8 or 6cbc structures.
- `check_anim_entry_type_a`: Moves d0 into a6, compares the word at offset zero of that structure with ten, and branches away if equal.
- `blit_road_segment_to_bitplanes`: Reads track segment parameters from a6 and a4, computes blitter window and modulo values, writes BLTCON, pointer, modulo and size registers, then starts the blit and returns zero in d0.
- `reset_race_state_and_place_objects`: Calls setup routines, clears many game state words and longs, seeds object positions with random values, fills a table with descending words and assigns velocities to type-one objects.
- `spawn_random_object_with_timer`: Calls object allocation, sets its field 68 to ten, stores the object pointer, sets a countdown of 75, picks a random value plus 100 and stores it, then sets a flag to minus one.
- `spawn_effect_with_three_offsets`: Calls the allocator, and if it returns a pointer, stores type 2, three absolute offsets added to a global base into fields 112, 116 and 120, clears word at 22 and sets word at 16 to 1.
- `countdown_spawn_and_shrink_effect`: Decrements a global counter; when it hits zero calls the spawn routine, saves the returned pointer, sets field 68 from a shifted constant, copies word 22 from a global object and arms a second countdown that later shrinks field 68.
- `zero_filled_data_area_2`: A single zero word that decodes as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `unpack_12bit_planar_words_to_a2`: Reads three bytes at a time from a source table, packs their nibbles into 12-bit words, and writes each word to the buffer pointed to by a2 for the count in d2.
- `scoreboard_setup_and_planar_unpack`: Calls setup code, clears the longword at offset four of the scoreboard structure, loads a value into d7, calls another routine, then jumps to the planar unpack loop.
- `show_novice_highscore_screen`: Loads the novice high score screen pointer into a0, calls the screen setup routine, clears the longword at offset four of the structure at a2, loads a value into d7, calls the display routine, then jumps to the tail routine with a2 pointing at the screen data.
- `show_track_highscore_screen`: Loads the track high score screen pointer into a0, calls the screen setup routine, clears the longword at offset four of the structure at a2, loads a value into d7, calls the display routine, then jumps to the tail routine with a2 pointing at the screen data.
- `show_loading_highscore_screen`: Loads the loading high score screen pointer into a0, calls the screen setup routine, clears the longword at offset four of the structure at a2, loads a value into d7, calls the display routine, then jumps to the tail routine with a2 pointing at the screen data.
- `show_champion_high_score_screen`: Loads the champion high score screen text pointer into a0, calls the display setup, clears the field at offset four of the screen structure, loads the champion score into d7, calls the score display, then jumps to the screen loop.
- `show_scores_high_score_screen`: Loads the scores high score screen text pointer into a0, calls the display setup, clears the field at offset four of the screen structure, loads the scores value into d7, calls the score display, then jumps to the screen loop.
- `draw_selected_high_score_entries`: Saves four globals, indexes a twelve-byte table by the selected entry, draws four pointer entries via the draw routine, restores the globals and jumps to the screen loop.
- `init_sound_channel_structs`: Calls two setup routines, writes 50 and 120 into fields of the structure at a2, then adds 40 to each of four longwords starting at the sample buffer address.
- `blit_masked_sprite_rows`: If the flag byte at offset 10 of a2 equals one, copies and run-length expands sprite rows from the source pointer into the destination row pointers, advancing each row by the row stride.
- `configure_three_audio_channels`: Loads three sample structures, stores their pointers, and programs the AUD0, AUD1 and AUD2 hardware registers with sample addresses, lengths, periods and volumes before jumping to the playback start routine.
- `start_audio_channel_zero_playback`: Calls two setup routines, stores the returned sample pointer, then programs AUD0LCH, AUD0LEN, AUD0PER and AUD0VOL with the sample address, half length, period and volume 63.
- `open_audio_channel_two_and_start`: Sets channel 2 flags, installs two callback pointers, opens the audio library, requests channel 2, closes the library and jumps to the installed playback routine with the custom chip base.
- `zero_filled_data_area_3`: Two zero words that decode as ori.b instructions; this is a zero-filled data area referenced by address, not executable code.
- `zero_filled_data_area_4`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `set_offset_9c_to_512_return_one`: Writes the immediate value 512 to the word at offset 156 from a0, sets d0 to 1 and returns to the caller.
- `call_38ba_test_d0_branch_if_zero`: Calls the routine at address 38ba, tests the low word of d0, and branches to address 9f8 when the result is zero.
- `parse_8svx_header_fields`: Scans an IFF 8SVX chunk for NAME, BODY and VHDR tags, computes sample rate from a fixed constant divided by the VHDR word, stores rate and body pointer in globals, then jumps to the error handler on failure.
- `compare_string_until_null`: Compares bytes at a0 against the null-terminated string at a1, advancing both pointers, and returns with zero flag set only if the a1 string ended before a mismatch.
- `parse_ilbm_header_fields`: Opens an IFF ILBM file, verifies FORM, BMHD and CMAP tags, copies palette bytes and body length into globals, divides the body size by three, and stores the image data pointer.
- `_RangeRan`: Advances the global range seed by repeated shift-and-xor steps, stores it back, then returns a value scaled to the word argument, or the low seed word when the argument is zero.
- `_FastRand`: Doubles the longword argument, xors it with the fixed polynomial constant when the shift carries or the result is zero, and returns the new value in d0.
- `allocate_block_with_header`: Adds 12 to the requested size, calls the memory allocator with that size and no flags, stores the size in the block header and returns the pointer past the header, or zero on failure.
- `link_allocated_block_into_list`: Aligns the new block pointer, writes its size into the header, links it into the global list via a call, and returns the pointer past the twelve-byte header.
- `free_linked_memory_blocks`: Walks the linked list at the global pointer, saving each node's next pointer, calling the memory free function on each node and its size field, then returns.
- `append_node_to_doubly_linked_list`: Links the node passed as second argument onto the tail of the list whose head is the first argument, updating next and previous pointers and the head when the list was empty.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_FindTask`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `_SetSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `_OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `stack_check_then_jump_zero`: Opens an a6 frame of forty-two bytes, compares the stack pointer with the value at address four, and jumps to address zero if the stack is below it.
- `allocate_buffer_and_store_pointer`: Clears a field, pushes size and memory attributes onto the stack, calls the allocation routine, stores the returned pointer, and returns 1 if allocation failed.
- `open_and_validate_file_handle`: Calls several helpers to open a file into the structure at -20 from a6, stores the handle at -22, clears a byte at -18, reads and checks the file, and returns a status longword at -38 from a6.
- `close_file_and_allocate_buffer`: Calls helpers to close the file at -20 from a6, passes the handle at -22 to another helper, then allocates a 14200-byte buffer and stores the result at -26 from a6, returning 2 on failure.
- `allocate_and_validate_memory_block`: Adds 16 to the pointer at offset -26 of a6, stores it at -34, calls the allocation routine with size 3778 and tag 924a, then stores the result at -30 and sets status at -38 to 3 if allocation failed, 4 if the size is out of range, or 0 otherwise.
- `check_allocated_block_size_range`: Subtracts the value at -26 of a6 from the value at -30, compares the difference against the lower and upper bounds, and clears the status at -38 when it falls inside the accepted range.
- `free_block_set_error_status`: Sets the status at -26 of a6 to 4, calls the memory free routine with the pointer at -42 and size 6ef0, then returns that status in d0 after releasing the a6 frame.
- `stack_check_frame_then_jump`: Opens a six-byte stack frame on a6, compares the stack pointer with the value at absolute address 4, branches past the jump if a7 is above or equal, otherwise jumps to address zero.
- `copy_20_bytes_setup_cia_and_audio`: Copies twenty bytes from a fixed source to the caller's buffer, clears two bits of CIAB port B, sets both disk control ports to all ones, stores a shifted bit and a longword in the buffer, then programs the custom chip audio and disk registers.
- `stack_check_frame_then_jump_alt`: Opens a six-byte stack frame on a6, compares a7 with the value at absolute address 4, branches past the jump when a7 is above or equal, otherwise jumps to address zero.
- `set_copper_list_pointer_and_enable`: Stores the custom chip base address at offset minus four from a6, writes 32767 then the caller's word at a6 offset ten ORed with 49152 to the copper list pointer register, then releases the frame and returns.
- `stack_check_then_jump_to_zero`: Creates an empty a6 stack frame, compares the stack pointer with the value at absolute address four, branches past a jump to address zero if the stack is above it, otherwise jumps to address zero.
- `disk_motor_pulse_and_fire_wait`: Calls a setup routine with the pointer at a6 offset eight, clears word at offset six, pulses the disk select line via CIA-B port B, then polls CIA-A port A fire bit and calls another routine until the button state changes.
- `set_ciaa_pra_bit1_and_poll_fire`: Reads CIAA port A at a fixed address, sets bit 1, writes it back, then reads CIAA port A at a fixed address, ORs with a fixed address and increments to test the fire button, branching to 658 if nonzero, otherwise pushes a6 offset 8 and calls 1392 before branching to 628.
- `delay_15000_then_call_cleanup`: Pushes the immediate value 15000, calls the delay routine at 702, discards the argument, then pushes the longword at offset 8 of a6, calls the routine at 1540, discards it, unlinks a6 and returns.
- `check_stack_pointer_against_a7_limit`: Compares the current stack pointer a7 with the longword at absolute address 4, branches past the jump if a7 is above or equal, otherwise jumps to absolute address 0.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `stack_limit_check_then_jump`: Creates an a6 stack frame, compares the current stack pointer a7 with the value at absolute address 4, and jumps to address zero if a7 is below that limit.
- `shift_count_right_and_loop_call`: Reads the longword at offset 8 from a6, shifts it right by two, stores it back, then decrements it and calls another routine repeatedly until the counter reaches zero.
- `unlink_frame_and_return`: Releases the a6 stack frame with unlk and returns immediately to the caller.
- `stack_check_frame_setup`: Creates an a6 stack frame, compares the stack pointer against the value at absolute address 4, and jumps to address 0 if the stack is below that limit.
- `clear_flag_and_release_object`: Loads the object pointer from offset 8 of the a6 frame, and if its byte at offset 1 is set, clears it and calls two subroutines with the pointer before returning.
- `stack_check_frame_setup_alt`: Creates an a6 stack frame, compares the stack pointer with the longword at absolute address 4, and jumps to address 0 when the stack pointer is lower.
- `start_grid_sequence_once`: Reads the object pointer from the stack frame, and if its flag byte is clear, sets it, calls three subroutines with the object and a an address delay, then releases the frame and returns.
- `stack_check_and_jump_to_zero`: Builds a stack frame, compares the stack pointer against the value at absolute address 4, branches past a jump to address zero if above, otherwise jumps to address zero.
- `init_copper_and_wait_fire_release`: Stores the custom chip base, clears a counter, sets a timeout, writes the object's copper list pointer to the copper, calls a subroutine, then polls the CIA fire button and returns a status value.
- `pulse_ciaa_port_a_and_decrement_counter`: Clears then writes the value 46968 twice to offset 36 of the object pointed to by a6-4, stores it at a6-8, decrements the long at a6-16 and returns 22 if it stays non-negative.
- `read_joystick_fire_and_delay`: Reads a word from the object, clears its field, tests the fire button bit in CIAA_PRA, sets a delay of 25, calls a wait routine and returns the delay value.
- `link_frame_save_d2_d3_and_stack_check`: Opens an a6 stack frame with 24 bytes of locals, pushes d2 and d3, compares the stack pointer against the value at address 4 and jumps to address 0 if below it.
- `split_target_and_current_into_halves`: Reads the target word from the structure at offset 8 and the current value from offset 12, stores their low bits and shifted halves into the a6 frame, and returns early if the values match.
- `compare_low_bits_and_clear_ciab_bit2`: Calls the step routine, compares the stored low bits, and when the current bit is zero clears bit 2 of CIAB port B before branching onward.
- `set_ciab_bit2_and_compare_shifted_halves`: Sets bit 2 of CIAB port B, waits 200 units, then compares the shifted halves and clears bit 1 of CIAB port B when the current half is smaller.
- `ciab_motor_bit_set_and_delay_loop`: Sets bit 1 of CIAB port B at a fixed address, computes a count from two frame variables, then repeatedly calls a delay routine with the caller's argument until the count expires.
- `delay_then_store_word_and_call`: Pushes 15000 and calls a delay routine, stores the low word of the argument at offset 6 of the pointer, then calls another routine with that pointer and restores d2-d3.
- `stack_check_and_jump_to_zero_2`: Opens an a6 frame, compares the stack pointer against the value at address 4, and jumps to address zero if the stack is below it.
- `pulse_ciab_motor_line_and_delay`: Reads CIAB port B, clears bit 0, writes it back, then sets bit 0 and writes again, then calls a delay routine with 3000 and returns.
- `stack_check_link_frame_and_trap`: Creates an a6 stack frame, saves d2, compares the stack pointer with the value at address 4 and jumps to address 0 if the stack is below it.
- `set_ciab_disk_select_from_parameter`: Sets bit 7 of CIAB port B, then clears the bit selected by the byte pair pointed to by the caller's parameter, writing the result back to CIAB port B.
- `stack_check_then_jump_absolute`: Builds an empty a6 frame, compares the stack pointer with the value at absolute address 4, branches past the trap if a7 is above it, otherwise jumps to absolute address zero.
- `set_ciab_port_b_bits`: Reads the CIAB port B register, sets bits 3 through 6 with an immediate or, writes the result back, then releases the a6 frame and returns.
- `stack_check_frame_with_saved_regs`: Builds an a6 frame of 24 bytes, pushes d5 through d7 onto the stack, compares a7 with the value at absolute address 4, branches past the trap if above, otherwise jumps to absolute address zero.
- `decode_bitmap_row_span`: Reads row and width fields from the a6 structure, computes two bitmap pointers via a shift helper, then scans words comparing against a stored value and returns a match index or minus one.
- `check_high_word_match_and_return`: Masks d6 to its high word, compares it with d5, and if equal returns the longword at offset 16 of a6 after restoring d5-d7 and the frame.
- `advance_bit_counter_and_loop`: Increments the longword at offset 16 of a6, shifts d6 left, decrements d7 and branches back into the surrounding bit-scan loop.
- `read_word_from_stream_and_merge`: Clears d0, loads the stream pointer from offset -12 of a6, reads one word into d0, advances that pointer by two, ORs the word into d6 and sets d7 to 16 before branching onward.
- `return_minus_one_restore_registers_3`: Sets d0 to minus one as the return value, restores d5 through d7 from the stack, releases the a6 frame and returns to the caller.
- `signed_32bit_divide_d0_by_d1`: Divides d0 by d1 using 32 shift-and-subtract steps, handling signs of both operands, and returns the signed quotient in d0 while preserving d2 through d5.
- `clear_d0_d1_restore_d2_d5`: Clears d0 and d1 to zero, then pops d2 through d5 from the stack and returns to the caller.
- `_Open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `_Close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `_Read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `_Write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `_Seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `_DeleteFile`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `_Rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `_IoErr`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `signed_divide_setup_d0_by_d1`: Saves d1-d4, copies d0 to d4, tests d0 and d1, negates d0 if negative, and branches to the shared division code with d2 holding the absolute dividend.
- `unsigned_multiply_32bit_shift_add`: Multiplies d0 by d1 using 16-bit mulu on swapped halves, accumulates partial products into d0, negates the result if d4 is negative, then restores d1-d4 and returns.
- `_fclose`: Saves d2 on the stack, loads the file handle argument from the stack into d2, calls the Close routine with it, discards the argument, restores d2 and returns.
- `validate_index_and_check_flag_byte`: Checks a global flag and calls a setup routine, then validates the index argument against a global limit, reads a byte from a table and tests its high bit, setting error code 9 and returning null on failure.
- `lookup_table_byte_pointer`: Reloads the index argument from the stack, calls a table lookup routine with selector 6, adds the table base to the result and returns that address.
- `find_free_file_slot`: Loops over the file slot count, calls a slot lookup routine, and returns the first index whose byte is zero, or sets an error code and returns minus one when all slots are full.
- `check_slot_index_and_return`: Loads the local slot index, compares it with zero, sets the error code to 24 and returns minus one if it is nonzero, otherwise returns the index in d0.
- `build_file_slot_flags`: Looks up the slot byte for the local index, toggles the high bit of the caller's flags with a global mask, stores the slot pointer, and sets the initial slot byte from the flag bits.
- `set_open_mode_flags_and_open_file`: Clears d0, sets bit 7, stores it to the byte at -12(a6), masks the mode at 12(a6) to two bits and sets bit 6, 5 or 4 in that byte depending on the mode, then deletes and opens the file.
- `test_append_flag_set_bit3`: Tests bit 3 of the byte at offset 15 from a6 and returns 8 in d0 if set, otherwise branches on to the following code with d0 unchanged.
- `set_bit5_in_status_byte`: Loads the byte pointed to by -12(a6), ORs in 32, writes it back, then branches to the shared file-open code.
- `check_bit3_and_set_d0_eight`: Tests bit 3 of the byte at offset 15 from a6; if clear it branches ahead, otherwise sets d0 to 8 and branches to the shared exit.
- `set_flags_and_open_file`: Sets bits 5 and 6 in the byte pointed to by the longword at offset -12 from a6, then branches onward to file open logic.
- `clear_flag_set_dos_error_and_return`: Clears the byte pointed to by the longword at offset -12 from a6, stores 22 in the DOS error longword, sets d0 to -1, releases the a6 frame and returns.
- `check_second_bit_and_delete_file`: Tests bit 1 of the byte at offset 14 in a6; if set, pushes the pointer at offset 8 in a6 and calls the delete-file routine, then checks whether the longword at offset 12 in a6 masked with 768 is zero and stores 1006 at offset -8 in a6 if not.
- `open_file_and_store_handle`: Stores 1005 at offset -8 in a6, calls the open routine with the name at offset 8 in a6, saves the returned handle at offset 2 of the structure pointed to by offset -12 in a6, and on failure records the IO error and returns -1.
- `return_saved_longword_from_frame`: Loads the longword at offset -4 in a6 into d0 as the return value, releases the a6 stack frame and returns to the caller.
- `open_file_and_store_handle_2`: Pushes the pointer argument at offset 8 of the frame, calls the open routine, stores the returned handle in local slot -8, and returns -1 in d0 if the handle is null.
- `check_file_flag_bit_two`: Clears d0, loads the file handle from local slot -8, reads its first byte into d1, stores d0 in local slot -4, and returns after testing bit 2 of that byte.
- `close_file_and_clear_handle`: Loads the handle from local slot -8, calls _Close on the longword at offset 2 of it, clears the first byte of the handle, and returns the value from local slot -4 in d0.
- `delete_file_and_handle_error`: Calls a routine to delete a file, and if the return value is zero, calls another routine to get an error, stores that error in a global variable, sets a global variable to two, and sets another global variable to negative one before returning.
- `return_zero_after_unlink_frame`: Sets d0 to zero as the return value, releases the a6 stack frame and returns to the caller.
- `open_file_and_store_handle_3`: Pushes mode 1005 and a path pointer, calls Open, cleans the stack and stores the returned file handle in a global.
- `read_byte_or_buffered_char`: Checks two globals, calls setup routines if needed, then returns a buffered byte from a global and clears it, or reads one byte from the file handle into a stack slot and returns it.
- `read_one_byte_from_handle`: Pushes a count of one and a stack buffer address, calls the Read library function on the file handle, cleans the stack, and returns the byte read.
- `write_one_byte_to_handle`: Checks two globals and calls setup routines if needed, then calls the Write library function to output one byte from the stack argument to the file handle and returns that byte.
- `check_flag_and_return_ff`: Opens an a6 stack frame, tests the byte at absolute address 12, and if it is zero branches to the routine at the next instruction; otherwise sets d0 to 255, releases the frame and returns.
- `store_byte_from_stack_arg`: Loads the longword at offset 8 from the a6 frame, writes its low byte to absolute address 12, releases the a6 frame and returns to the caller.
- `close_marked_entries_in_table`: Loops over the word count at a fixed global, and for each six-byte table entry whose first byte is nonzero and lacks bit 2, calls _Close on the pointer at offset 2, then falls through to another routine.
- `call_other_with_two_frame_args`: Pushes the two longwords at offsets 8 and 12 of a6 onto the stack, calls the routine at address 358, then discards the arguments, restores d5-d7 and a6, and returns.
- `mask_and_check_pointer`: Clears a temporary stack, calls a routine, masks a pointer in d0, and if the masked pointer is not zero, stores it in a stack-based location before returning.
- `check_flag_then_call_with_94`: Tests the longword at address zero, and if nonzero pushes 94 and calls the routine at address 1294, then compares the frame value with 8192 and may set d0 to 68.
- `init_screen_and_loop_count`: Pushes 67 and 10 as arguments to a subroutine, clears the longword at offset minus four from a6, then loops calling another subroutine with each index until it reaches the global limit.
- `call_zero_args_and_branch`: Pushes two zero longwords onto the stack, calls the routine at address zero, then adds eight to a7 and branches to a later location.
- `return_local_longword`: Loads the longword stored at offset minus four from a6 into d0, releases the a6 frame with unlk and returns that value to the caller.
- `unlink_a6_frame_and_return`: Restores the a6 stack frame with unlk and returns immediately to the caller, doing nothing else.
- `open_library_and_request_intuition`: Opens a library, stores the handle in a global variable, copies a pointer from a global variable to another, and performs an auto request with a fixed size before calling another routine with twenty as an argument.
- `close_twenty_file_handles`: Loops from zero to nineteen, obtains each file handle via a lookup call and closes it with fclose, then continues into further cleanup code.
- `increment_counter_until_limit`: Clears the longword at offset minus four from a6, then repeatedly compares it with the value at address zero, calling the routine at 918 with the counter pushed while it is below the limit, incrementing the counter each pass.
- `call_with_two_frame_args`: Pushes the longwords at offsets twelve and eight from a6 onto the stack, calls the routine at address zero with those two arguments, removes them, releases the a6 frame and returns.
- `_AutoRequest`: Saves d2-d3 and a2-a3/a6, loads the library base into a6, copies six stack arguments into a0-a3 and d0-d3, calls the library offset minus 348, restores registers and returns.
- `zero_filled_data_word_at_00b6`: A single zero word that decodes as ori.b with immediate zero to d0; it is data, not executable code, referenced by a handler pointer.
- `zero_filled_data_word_at_00ba`: One zero word in a zero-filled data area, decoding as ori.b immediate zero to d0; referenced by a data pointer, not real code.
- `zero_filled_data_word_at_00be`: A lone zero word treated as code, ori.b with zero immediate to d0; it is padding data pointed to from hunk data.
- `zero_filled_data_area_5`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data, not executable code.
- `allocate_and_build_bob_table`: Allocates two memory blocks via library calls, stores the second pointer globally, then fills a table of bob entries with computed sizes and offsets, patching matching entries in a lookup list.
- `fill_bob_entries_from_anim_table`: Copies animation table words and dimensions into successive bob entries, computes row bytes and offsets, then calls the patch routine for the table and the global list.
- `open_workbench_and_run_script`: Loads the library base from pc-relative data into a6, opens the workbench library with value 1006 in d2, then calls three library offsets passing a2 and a3 as arguments and returns.
- `open_and_load_body_file`: Opens the file named by a0 with mode -2, stores the handle, reads it into the buffer at the body data area, closes it, then loads the BODY segment and returns the segment pointer in d0.
- `restore_registers_and_return_zero`: Pops d2 through d7 and a2 through a6 from the stack, clears d0 to zero as the return value and returns to the caller.
- `or_plane_rows_into_destination`: Reads the byte at offset 8 of a4 minus two as a row count, then for each even column index ORs words from successive pointer-table entries into a destination word and stores the result.
- `decode_tilemap_into_bitplanes`: Checks the longword at a2 against a threshold, saves registers, then walks the pointer table at a6 and the tile data at a3 to expand packed tile bytes into the bitplane buffer.
- `compute_bitplane_row_offset`: Compares a5 with d3 and branches if not below, tests byte at offset 10 of a2, then computes a row offset from the word at a2 by negating, masking to a multiple of sixteen, negating and shifting right three, minus one.
- `rle_decode_byte_run_to_a5`: Reads a signed byte from (a3)+ into d0; if positive copies that many following bytes to (a5)+, if negative repeats the next byte, and if the value is the end marker it branches away.
- `copy_byte_and_loop_to_a5`: Copies one byte from (a3)+ to (a5)+, decrements d0 and loops back while d0 is not minus one, then branches to the shared decode continuation.
- `finish_decode_restore_regs_and_call`: Masks a word at -2(a5), adds offsets into (a4), walks row pointers, copies a saved long to (a6), restores d2-d7/a2-a6, calls the next routine and returns.
- `unpack_rle_bitmap_to_buffer`: Saves registers, stores a pointer from the source structure into the destination buffer, computes row stride from the width, and expands a run-length encoded bitmap row by row into memory at a5.
- `decode_rle_rows_into_bitmap`: Walks the compressed bitmap stream, handling negative run counts as repeated bytes and positive counts as literal copies, writing bytes to a5 and decrementing d2 and d5 until the row count is exhausted.
- `copy_literal_run_then_continue_decode`: Copies d0 plus one literal bytes from a3 to a5, decrements d2 and d5, then branches back into the run-length decoder to continue the next row.
- `pop_d2_d7_a2_a6_and_return`: Restores registers d2 through d7 and a2 through a6 from the stack, then returns to the caller.
- `parse_ilbm_bmhd_and_body_chunks`: Saves registers, opens the IFF file, reads the BMHD header and body chunk lengths, stores the file pointer and chunk sizes in globals, and scans chunks until the body is found.
- `copy_cmap_palette_bytes`: Copies a counted run of palette bytes from the chunk data into the CMAP buffer, then advances past the chunk and scans for the next chunk header.
- `parse_body_header_and_seek`: Reads a count from (a0), stores count divided by three at the body counter, copies count bytes to the body buffer, then scans forward through the body data until a match is found.
- `advance_a0_store_pointer_restore_regs`: Adds four to a0, stores the resulting pointer into memory at 05F6, restores d2-d7 and a2-a6 from the stack and returns to the caller.
- `jump_to_05ca_entry`: Unconditionally jumps to the code at address 05CA, transferring control without saving registers or returning here.
- `compare_strings_until_null_or_mismatch`: Compares bytes from a1 and a0, returning when a1 holds a null byte or when the bytes differ, advancing both pointers.
- `restore_stack_pointer_from_abs_8f0`: Loads the longword at absolute address 8f0 into a7, replacing the stack pointer, then returns to the caller.
- `reset_stack_call_abs_zero_twice`: Loads a7 from absolute address 4c, calls the routine at address zero, pushes immediate 20, then calls address zero again.
- `scan_string_until_null`: Links a6 frame, clears a local offset, then loops adding the offset to the pointer at 8(a6) and reading a byte into d0 until a zero byte is found, incrementing the offset each pass.
- `return_local_offset_and_unlink`: Loads the local longword at -4(a6) into d0 as the return value, releases the a6 stack frame and returns to the caller.

### Grid Start (1987)(Anco)[cr A-Ha][a].Adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `bootblock_alloc_and_run_demo`: Allocates memory, opens graphics library, sets up copper and DMA, then loops reading VHPOSR to trigger a roxl-based bitmap scroll and advances a string pointer until null before freeing memory.
- `build_copper_list_and_set_video_registers`: Writes a 32-entry copper list of move and wait instructions into the buffer at offset 8 of a5, then programs BPLCON1, BPLCON2, DIWSTRT, DIWSTOP, DDFSTRT, DDFSTOP and BPL1MOD and points COP1LCH at the list.
- `emit_copper_instructions_and_shift_table`: Appends fixed copper move instructions to the buffer in a0, calls two helpers to emit more, then copies 32 words backwards from the table at 872 to 808 and stores the saved word.
- `load_copper_table_pointers`: Loads the address of the table at 808 into a1 and the address of the table at 872 into a2, then falls through to the following code.
- `build_copper_list_from_palette_words`: Loops over palette words at a1, writing colour register move instructions with incrementing register numbers into the copper list at a0 until d0 equals d2, wrapping a1 to a2.
