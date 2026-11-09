# Game 870 (grid start (1987)(anco)[cr a-ha]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00870). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 44 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 30 raw-word lines |
| Functions named and described | pass, 432 of 432 named (17 author symbols), 432 of 432 described (35 by rule, 4 by the local model) |
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
| Grid Start (1987)(Anco)[cr A-Ha].adf:copy | 2 | 117 | 2 |
| Grid Start (1987)(Anco)[cr A-Ha].adf:Grid Start | 35 | 5935 | 295 |
| Grid Start (1987)(Anco)[cr A-Ha].adf:bootblock | 1 | 180 | 6 |
| Grid Start (1987)(Anco)[cr A-Ha].adf bytes 5632-11264 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- grid start (1987)(anco)[cr a-ha]/Grid Start (1987)(Anco)[cr A-Ha].adf: file            (header 5124 bytes, blocks hold 5368): its blocks are judged as raw blocks outside the filesystem
- grid start (1987)(anco)[cr a-ha]/Grid Start (1987)(Anco)[cr A-Ha].adf: bytes 5632-11264 look like code no run loaded: recovered position-independently (load address unknown)
- grid start (1987)(anco)[cr a-ha]/Grid Start (1987)(Anco)[cr A-Ha].adf: 1 logged loads cover 5632 bytes of the image
- 1 never-loaded region(s), 5632 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_20_bytes_set_cia_and_audio_regs` (Grid Start): Copies 20 bytes from a fixed source to the caller's buffer, clears CIA bits, writes all ones to two CIA ports, stores a shifted bit and a longword in the buffer, then programs audio and disk registers before returning. Kind: disk hardware/timing check. Pass path: the routine continues execution normally after setting the hardware registers.
- `disk_motor_pulse_and_fire_wait` (Grid Start): Calls a routine with the pointer at offset eight of a6, clears word at offset six, pulses the disk select line via CIA-B port B, then polls CIA-A port A fire bits and calls another routine until the button state changes. Kind: disk motor control/timing check. Pass path: the loop completes and the routine proceeds to the delay and subsequent function call.
- `toggle_ciaa_fire_and_wait_loop` (Grid Start): Sets bit 1 of CIAA_PRA, reads CIAA_PRA into d0, ORs with all bits except bit 4, increments d0 and branches back if nonzero, then pushes and calls a routine. Kind: disk hardware/timing check. Pass path: the loop terminates and execution continues to u6__sub_2_02BE.
- `clear_ciab_fire_bit_when_target_zero` (Grid Start): Calls a routine with the frame pointer, compares two frame longwords, and if the first is zero clears bit 2 of CIAB port B, then branches onward. Kind: disk hardware/track verification (CIAB_PRB). Pass path: The routine performs a comparison between two values (likely expected vs. actual sector/track data or timing). If the values match (i.e., the `if` conditions `iVar2 != iVar1` are skipped), the routine proceeds to the final instructions with.
- `set_ciab_fire_bit_and_delay` (Grid Start): Sets bit 2 of CIAB port B, calls a delay routine with 200, then compares two frame longwords and clears bit 1 of CIAB port B when the second is smaller. Kind: disk hardware timing/sync check. Pass path: the branch where iVar2 == iVar1 (skipping the delay loop and bit manipulation).
- `toggle_ciab_fire_and_wait_loop` (Grid Start): Sets bit 1 of CIAB port B, then loops calling a delay routine with the count from the frame offset minus the base offset, decrementing until zero. Kind: disk hardware/track check loop. Pass path: the loop completes and execution proceeds to u6__sub_2_02BE (delay/wait) and subsequent data loading/processing.
- `pulse_ciab_motor_line_low_then_high` (Grid Start): Reads CIAB port B, clears bit 0 to drive the motor line low, then sets bit 0 high again and calls a delay routine with 3000 before returning. Kind: hardware/timing-based check. Pass path: the routine continues execution normally after the delay.
- `set_ciab_disk_select_from_parameter` (Grid Start): Sets bit 7 of CIAB port B, then clears the bit selected by the parameter byte and its inverted second byte, writing the result back to CIAB port B. Kind: disk hardware/status bit check. Pass path: the routine modifies a global status flag (DAT_00bfd100) based on the state of the CIAB_PRB hardware; a successful check (valid disk presence/status) sets specific bits in that flag to allow the loader to proceed..
- `set_ciab_port_b_high_bits` (Grid Start): Reads CIAB_PRB, sets bits 3 through 6 with a logical or, writes the result back to CIAB_PRB, releases the a6 frame and returns. Kind: hardware register manipulation (disk controller). Pass path: not determined.

## Functions


### stack

- `allocate_and_init_stack_node`: Calls library vectors through a5 to find the highest stack size, allocates a stack block, fills it with marker values, copies sixteen longs into the node at offset 536, and returns zero or minus one.
- `return_d1_in_d0_then_branch`: Moves d1 into d0 as the return value and branches forward into the middle of the surrounding stack routine.

### endcli

- `endcli_stack_cleanup_node`: Calls library vectors through a5 to size and allocate a stack node, writes marker longs, copies sixteen longs at offset 536, and returns zero or minus one.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0 and branches to another part of the same routine, without touching memory or the stack.

### echo

- `echo_command_main`: Reads the argument string, looks up a command table, allocates and fills a buffer with command data, calls library routines through a5, and returns zero or minus one.
- `copy_d1_to_d0_and_branch_2`: Moves the longword in d1 into d0, then branches to another routine at a fixed offset without returning here.

### loadWb

- `parse_workbench_startup_args`: Saves a7, d0 and a0, opens the current directory, splits the argument string into words stored in a buffer, then opens and initializes the workbench task and returns zero.
- `cleanup_task_and_reply_exit`: Restores the saved stack pointer, closes the dos library, forbids and replies the startup message, then returns the caller's exit code or raises an alert.
- `flush_and_close_workbench_file`: Loads the address of the workbench structure field at offset 92 into a0 and calls the dos library flush and close functions on it twice, then returns.
- `clear_and_load_workbench_task`: Clears the longword at absolute address 8, loads the address of the workbench task name into a1, calls the dos.library function at offset -552 with d0 set to 30, stores the result at address 8 and returns.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `start_workbench_tool_with_error_report`: Saves d2-d5, checks the argument count, optionally compares a string, loads a resident tag, starts a tool, and on failure prints error messages with a delay before restoring registers and returning.
- `compare_two_strings_lexicographically`: Compares the two string pointers passed on the stack byte by byte, returning zero when equal, minus one when the first is less, and one when greater.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_vector_with_arg_and_cleanup`: Saves d2, loads the argument from the stack, pushes it plus a pointer to the following stack slot, calls the function pointer at absolute address 20, then restores the stack and d2.
- `format_string_via_rawdofmt_and_dispatch`: Saves a2-a4/a6, builds a 140-byte stack buffer, calls exec RawDoFmt with the format string and arguments, measures the resulting string length, and passes it to another routine before restoring registers.
- `store_d0_byte_to_a3_postincrement`: Writes the low byte of d0 to the address in a3 and advances a3 by one byte, then returns to the caller.

### icon.library

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_open_count_clear_flag`: Increments the 16-bit counter at offset 32 of the library base in a6 and clears bit 3 of the byte at offset 14, then returns the base pointer in d0.
- `decrement_count_and_close_if_zero`: Returns zero in d0, decrements the 16-bit counter at offset 32 of a6, and if it reaches zero with bit 3 of byte 14 set, calls the close routine.
- `close_library_if_not_open`: If the 16-bit field at offset 32 of a6 is zero, calls the library close vector at offset -210 of the base in 34(a6) and unlinks the node; otherwise sets bit 3 of byte 14 and returns zero.
- `set_icon_library_flag_bit3`: Sets bit 3 of the byte at offset 14 from the library base in a6, then returns zero in d0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_and_store_base`: Saves a0 and a6 into the new structure at offsets 42 and 34, opens dos.library via exec, stores the result at offset 38, and alerts on failure.
- `copy_structure_fields_to_globals`: Copies the longwords at offsets 34 and 38 of the a2 structure into two global variables, returns a2 in d0 and pops a2.
- `zero_word_data_at_0198`: A single zero word that decodes as ori.b with immediate zero to d0; this is zero-filled data referenced by address, not executable code.
- `zero_word_data_at_019c`: Another zero word decoding as ori.b with immediate zero to d0; a zero-filled data slot pointed to by library code, not real instructions.
- `push_a0_a6_call_target_pop_return`: Pushes a0 then a6 onto the stack, calls the routine at address zero, adds eight to the stack pointer to discard both saved registers, and returns.
- `icon_library_call_aa_with_a0_a1`: Saves a0-a1 and a6 on the stack, calls the library routine at offset aa, then discards the saved registers and returns.
- `icon_library_call_124_with_a0_a2`: Saves a0-a2 and a6 on the stack, calls the library routine at offset 124, then discards the saved registers and returns.
- `icon_library_call_3fe_with_a0_a1`: Saves a0-a1 and a6 on the stack, calls the library routine at offset 3fe, then discards the saved registers and returns.
- `icon_library_push_a0_a6_call_helper`: Pushes a0 and a6 onto the stack, calls a library helper, then discards the eight pushed bytes and returns to the caller.
- `icon_library_call_via_jsr_698`: Pushes a0 and a6 onto the stack, calls the library routine at offset 698, discards the two saved registers and returns.
- `icon_library_push_a6_call_third_helper`: Pushes only a6 onto the stack, calls a library helper, then discards the four pushed bytes and returns to the caller.
- `icon_lib_save_regs_call_helper`: Pushes a0 through a2 and a6 onto the stack, calls a library helper, then discards the saved registers and returns.
- `icon_lib_push_a0_a6_call_helper`: Pushes a0 and a6 onto the stack, calls a library helper, adds eight to the stack pointer to drop them and returns.
- `icon_lib_push_a0_a6_call_second_helper`: Pushes a0 and a6 onto the stack, calls a different library helper, adds eight to the stack pointer to drop them and returns.
- `icon_library_call_842_with_a0_a1`: Saves a0 and a1 on the stack, calls the library routine at offset 842, then discards the saved registers and returns to the caller.
- `icon_library_call_88a_with_a0_a1`: Pushes a0 and a1 onto the stack, invokes the library routine at offset 88a, removes the two saved registers and returns.
- `icon_library_call_8fe_with_a0_a1`: Stores a0 and a1 on the stack, jumps to the library routine at offset 8fe, then adds eight to the stack pointer and returns.
- `set_icon_library_userdata_field`: Saves a6, calls the library's open routine through offset -294, stores the second argument into offset 148 of the returned structure, then restores a6 and returns.
- `zero_filled_data_area`: A single zero word that decodes as ori.b with immediate zero; this is zero-filled data, not executable code.
- `load_icon_from_file_into_structure`: Opens the icon library, reads icon data from a file into a stack buffer, copies 44 bytes and several fields into the icon structure, links it, and returns the structure or zero.
- `copy_icon_fields_to_new_object`: Copies 44 bytes from the library base offset into a new object at a2, then copies several longword fields and links the object into a list via offset 424.
- `return_zero_and_restore_registers`: Sets d0 to zero as the return value, restores d2-d4 and a2 from the stack, releases the a5 frame and returns.
- `copy_icon_to_stack_and_dispatch`: Copies 44 bytes and several fields from the icon structure into a stack buffer, then calls the dispatch routine with the two arguments and buffer pointer.
- `load_icon_header_and_validate`: Builds a temporary name buffer, opens the icon file, reads its header, checks the magic word and version, then branches to the icon loader or returns zero.
- `read_icon_header_and_allocate_buffers`: Reads the icon header, verifies the magic word and version, clears two fields, then allocates and reads the image and palette buffers through the library's file hooks.
- `read_icon_image_and_palette_data`: Reads the icon's image and palette data through the library's file hooks, allocating and filling the buffers and storing the resulting pointers in the icon structure.
- `icon_open_four_images_and_optional_extra`: Calls indirect library hooks to open four image slots stored through a3, decrements a counter at -4(a6), then optionally opens a fifth image into 70(a2) and returns d6 as success flag.
- `set_failure_flag_and_branch`: Loads zero into d6 to signal failure and branches to the shared epilogue that restores registers and returns.
- `restore_registers_unlink_and_return`: Pops d2-d6 and a2-a5 from the stack, releases the a6 frame pointer and returns to the caller.
- `open_icon_library_base`: Opens the icon library base with version 260, saves the caller's icon structure pointer, and returns zero on failure after reporting error 103.
- `write_icon_header_and_attributes`: Writes the icon header and optional attribute blocks to the output stream, saving and restoring the icon's current data pointer around the operation.
- `write_icon_alternate_image_data`: Writes the icon's alternate image data and optional palette or array blocks to the output stream, returning the last write result.
- `scan_icon_list_and_call_hook`: Advances a4 by four and adds four to the counter at -8(a6) while the longword at a4 is nonzero, then calls the a3 hook with d3, d4 and the counter pushed, storing the result in d5 and unwinding sixteen bytes.
- `add_icon_entries_from_list`: Walks a null-terminated list of icon name pointers, converts each name to a BSTR and adds it to the icon list, then adds the fallback name at offset 70 of a2 and stores the result count in a2.
- `remove_icon_entries_by_name`: Takes a name string and an icon list, skips the list header, and repeatedly finds and removes matching entries until none remain.
- `restore_d2_d3_and_return`: Pops registers d2 and d3 from the stack and returns to the caller, restoring the saved register state.
- `call_icon_offset_8c_with_arg`: Saves d2, loads the first argument into d2 and the second into a0, then calls the routine at offset 140 in a0 with d2 pushed, cleans the stack and restores d2.
- `allocate_and_init_icon_block`: Saves d2-d4 and a2, allocates a 65536-byte block, initializes fields at offsets 140 and 142, calls subroutines with the block and argument, and returns the block pointer or zero on failure.
- `open_icon_by_name_with_cleanup`: Pushes a8, a2, an offset from a2 and d2, calls the open routine at offset 1838, tests d0, pops four longs, and returns a2 in d0 if successful or branches to the shared failure exit.
- `return_zero_and_restore_registers_2`: Sets d0 to zero as the return value, restores d2 through d4 and a2 from the stack, and returns to the caller.
- `decrement_icon_refcount_and_store`: Decrements the word reference count at a2, and if it reaches zero calls the release routine; then stores d3 and d2 into the entry at the current count index.
- `icon_remove_last_entry`: Decrements the word count at (a2), then stores d2 and the caller's stack argument into the entry array at offset 16 plus count times 8, returning 1 in d0.
- `icon_allocate_and_init_object`: Allocates 65536 bytes with tag 94, initializes the object at offset 78, then calls four library routines to set up and validate it, returning the object pointer or zero.
- `return_allocated_pointer_in_d0`: Moves the allocated object pointer from d6 into d0 and branches forward to the common exit path.
- `call_hook_and_return_zero`: Pushes a small constant and d6 onto the stack, calls a library hook through a jump table, then returns zero in d0 after restoring d2-d6 and a2.
- `call_hook_with_offset_argument`: Saves d2-d4, loads two stack arguments, adds 78 to the second, pushes both and calls a library routine, then restores registers and returns.
- `find_equals_value_in_name_list`: Scans a null-terminated list of strings, comparing each against a name and checking for an equals sign, returning the pointer after the match or zero.
- `return_zero_pop_d2_d3_a2_a3`: Sets d0 to zero as the return value, then pops d2, d3, a2 and a3 from the stack and returns to the caller.
- `scan_string_for_matching_length`: Saves d2-d6, calls library helpers to measure a string and find a delimiter, then loops comparing lengths until a match is found, returning one or zero.
- `verify_string_length_match`: Calls a helper to measure the string in d2, compares the result with d5, and if equal calls another helper to confirm, returning one on success.
- `advance_d4_into_d0_and_d2`: Tests d4; if nonzero, copies d4 to d0, increments d0 by one, stores the result in d2, then branches to shared library code.
- `zero_d2_then_branch_shared`: Loads zero into d2 and branches to the shared library continuation, providing a null result path for the icon library.
- `return_zero_restore_d2_d6`: Sets d0 to zero as the return value, restores registers d2 through d6 from the stack, and returns to the caller.
- `parse_icon_tooltypes_into_buffer`: Saves registers, clears byte 30 of the first argument buffer, then calls library helpers to parse tooltype strings from the second argument and copy up to 30 bytes into the first buffer, returning that buffer.
- `advance_past_tooltype_and_copy`: Adds 5 to a2, pushes three arguments and calls a library helper, then tests d0 and either sets d5 to 1 and advances a2 by 3 or branches onward.
- `set_d5_zero_and_branch`: Sets d5 to zero and branches to another part of the library code without calling anything.
- `parse_decimal_and_open_icon`: Accumulates decimal digits from the string at a2 into d5, calls the digit converter, then opens the icon file and copies it, returning d3.
- `convert_ascii_digit_to_value`: Loads the argument from the stack into d1, checks it lies between character zero and nine, and returns its numeric value in d0 or minus one.
- `return_minus_one_constant`: Loads minus one into d0 and returns immediately, serving as the failure result for the digit conversion.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `icon_remove_with_cleanup`: Saves d2-d5, loads three arguments from the stack, calls the lookup routine, and if found calls the removal routine and then the release routine, returning zero on success or the found pointer otherwise.
- `icon_allocate_and_init_node`: Saves d2-d3/a2, allocates a 96-byte block of type 65536, stores the value 10 at offset 14 of the block and at the caller's word pointer, then links the block into the caller's list and returns nonzero.
- `icon_alloc_fail_return_zero`: Sets d0 to zero as the failure return value, restores d2-d3 and a2 from the stack, and returns to the caller.
- `icon_lookup_verify_first`: Saves d2-d3, loads three arguments from the stack, calls the icon lookup routine, compares the returned value with the fourth argument and returns it or zero.
- `return_saved_lookup_result`: Moves d1 into d0 as the return value, restores d2-d3 from the stack and returns to the caller.
- `icon_lookup_verify_second`: Saves d2-d3, loads three arguments from the stack, calls a second icon lookup routine, compares the result with the fourth argument and returns it or zero.
- `return_d1_in_d0_pop_d2_d3`: Copies the longword in d1 to d0 as the return value, restores d2 and d3 from the stack, then returns to the caller.
- `divide_32bit_by_16bit_unsigned`: Divides the 32-bit value in d1 by the low word of d2, returning quotient and remainder in d0 and d1, using a shift-subtract loop when the divisor exceeds 65535.
- `divide_32bit_by_32bit_shift_loop`: Divides the 32-bit value in d1 by the 32-bit value in d2 using a 16-iteration shift-subtract loop, leaving quotient in d1 and remainder in d0.
- `mul32x16_shifted_product`: Multiplies the 32-bit value in d0 by the 16-bit value in d1 using partial products, returns the shifted 32-bit product in d0 and restores d2 from the stack.
- `format_string_via_rawdofmt`: Saves a2-a4 and a6, loads the format string and argument pointer from the stack, then calls exec RawDoFmt through the library base in a6.
- `store_d0_byte_to_a3_and_return`: Writes the low byte of d0 to the memory pointed to by a3, then increments a3 by one and returns to the caller.
- `strlen_ff_terminated`: Loads the string pointer from the stack, scans bytes until a zero or 255 bytes are counted, then returns the complemented count in d0.
- `find_char_in_string`: Loads a string pointer and a character from the stack, scans bytes until a match or terminator, and returns the pointer to the match or zero in d0.
- `return_zero_and_branch`: Sets d0 to zero and branches forward into the shared string-search exit path, returning a null result to the caller.
- `copy_string_until_nul`: Loads two pointers from the stack, copies bytes from the source to the destination one at a time, and returns after copying the terminating zero byte.
- `skip_string_to_terminator`: Loads two pointers from the stack, advances the first pointer past bytes until a zero byte is found, steps back one, then branches onward.
- `compare_strings_with_length_limit`: Loads two pointers and a length from the stack, compares bytes one by one up to that count, and returns zero in d0 when the strings match.
- `return_one_or_minus_one_flag`: Loads 1 into d0, then branches conditionally to a nearby address; if the branch is not taken it loads -1 into d0 and returns to the caller.
- `scan_string_until_zero_or_count`: Loads a0 and a1 from the stack, loads a count from the stack into d0, then scans bytes at a0 until a zero byte or the count runs out.
- `decrement_a0_and_call_helper`: Subtracts one from address register a0, calls a nearby library routine, then returns to the caller.
- `copy_bytes_until_zero_or_count`: Decrements d0 and, while it stays non-negative, copies bytes from (a1) to (a0) until a zero byte is copied, then clears the next byte if the count remains positive.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `init_self_referential_list_header`: Loads the pointer from four bytes above the stack pointer into a0, stores a0 at (a0), adds four to that word, clears the next longword and stores a0 at offset eight.
- `draw_icon_with_offsets`: Saves d2-d5/a2, checks the icon pointer, calls the library twice with the icon and computed x/y offsets, and returns one on success or zero on failure.
- `walk_icon_list_validate_entries`: Saves d2-d5/a2, reads three arguments from the stack, then walks a linked list of icon nodes, calling a validation routine on each node and its data, returning 1 on success or 0 on failure.
- `return_one_after_null_list_check`: Loads the value 1 into d0 and branches forward into the shared exit path, signalling success when the icon list pointer was null.
- `return_zero_restore_registers`: Sets d0 to zero as the failure return value, restores d2-d5 and a2 from the stack and returns to the caller.
- `alloc_icon_header_and_image`: Allocates a 20-byte header and 65536-byte image via library calls, computes aligned width from header fields, allocates and copies image data, stores result pointer at offset 10, returns header or -1.
- `return_minus_one_restore_registers`: Sets d0 to -1 as the failure return value, pops d2 through d6 and a2 from the stack, and returns to the caller.
- `allocate_linked_icon_list`: Loops allocating 16-byte icon nodes and their image buffers via library calls, links nodes through offset 12, returns the list head or -1 on allocation failure.
- `icon_alloc_and_link_node`: Copies a stack argument into a2 offset 12, calls library allocation routines to create a node and its data, stores results at offsets 8 and 12, and returns -1 on any failure.
- `return_a3_in_d0`: Moves the value in a3 into d0 and branches forward into the shared epilogue that restores registers and returns.
- `return_minus_one_restore_regs`: Sets d0 to minus one as the failure return value, restores d2 through d7 and a2 through a3 from the stack, then returns to the caller.

### Grid Start (1987)(Anco)[cr A-Ha].adf:copy

- `allocate_and_init_work_buffer`: Calls library vectors through a5 to find the highest slot index, allocates a buffer, fills it with incrementing tag values, copies 16 longs into a2+536, sets a2+8 to a continuation address, and returns zero or minus one.
- `return_d1_in_d0_then_branch_2`: Moves d1 into d0 as the return value and branches to a fixed offset inside the caller's code, transferring control rather than returning.

### Grid Start (1987)(Anco)[cr A-Ha].adf:Grid Start

- `cli_startup_open_and_run`: Saves d0 and a0, opens the current directory via dos, finds the CLI task, builds a command line on the stack, opens the program file, loads it, and jumps to the loaded code.
- `shutdown_cleanup_and_reply`: Calls the optional callback, frees two exec resources via a6, closes two file handles, then Forbids, replies the message and restores registers before returning.
- `open_and_close_console_window`: Loads the address of the console structure at offset 92 from a4 and calls the open then close library vectors through a6.
- `open_dos_file_store_handle`: Loads a filename pointer, sets d0 to zero, calls the dos open vector through a6 and stores the returned handle, branching if it is null.
- `open_four_libraries_and_store_bases`: Copies the exec library base from address 4 into a global, then calls OpenLibrary four times for the named library strings, storing each returned base and branching away if any is zero.
- `zero_filled_data_area_b0`: This single zero word decodes as ori.b with zero immediate; it is a zero-filled data byte referenced by address, not executable code.
- `zero_word_data_at_00b4`: A single zero word that decodes as ori.b with zero immediate; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_entry`: Contains a single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `zero_word_data_at_00bc`: Another lone zero word decoding as ori.b with zero immediate; zero-filled data area pointed to by handler pointers, not real code.
- `main_game_loop_setup_and_frame_cycle`: Initializes audio and DMA registers, calls many game subsystems, waits on vertical blank and timer, swaps display buffers and polls the fire button each frame.
- `close_screens_and_jump_exit`: Tests two global screen pointers, closes each non-null screen via intuition CloseScreen using the library base, then jumps to the exit vector.
- `blink_and_advance_sprite_frame`: On a timer flag, acknowledges the copper interrupt, toggles a blink bit, advances sprite frame counters and resets the frame index, writing DMACON and INTREQ.
- `center_sprite_from_frame_table`: Loads the sprite frame table entry selected by the frame index, adds the base offset, and stores centered x and y coordinates into the sprite structure.
- `setup_sprite_pointer_and_check_index`: Loads the sprite base pointer, stores base plus 1604 at offset 72, clears word at offset 68, and returns zero unless the word at the index global exceeds 17, in which case it returns minus one.
- `position_sprite_from_table_clear_flag`: Calls the setup routine, returns early if it fails, clears word at offset 70, then reads two words from the table at 28620 indexed by d1 and stores their complements from 200 and 200 at offsets 12 and 20.
- `position_sprite_from_table_set_flag`: Calls the setup routine, returns early if it fails, sets word at offset 70 to one, then reads two words from the table at 28620 indexed by d1 and stores their complements from 320 and 200 at offsets 12 and 20.
- `set_object_type_two_and_center_from_table`: Calls a subroutine, branches if it returns non-zero, sets word 2 at offset 70 of a0, reads a table entry indexed by d1 into d2 and stores it at offset 12, then stores 200 minus the next table word at offset 20.
- `compute_screen_offsets_from_tile_coords`: Calls a subroutine, branches if it returns nonzero, then stores 3 at offset 70 of a0 and computes two values by subtracting indexed words from 320 and 200, storing them at offsets 12 and 20 of a0.
- `init_sprite_from_frame_table_and_center`: Loads a base pointer plus 1424 into offset 72 of a0, clears offset 68, clamps the frame index at offset 70 to half of a6's word, then stores centered x and y from the frame table into offsets 12 and 20 and sets a6 to a0.
- `allocate_two_800_byte_buffers`: Loads the library base from pc-relative data into a6, then twice calls the allocator with size 2048 and type 2, storing each result at two globals and branching away if either allocation returns zero.
- `update_audio_volume_from_speed`: If the speed global is nonzero, calls the sound update, scales speed by a multiplier, clamps the result, writes a volume value to AUD1VOL at offset 184 of the custom chip base, and clamps the position global between limits.
- `advance_track_segment_and_spawn_objects`: Steps the current track segment pointer, updates scroll and speed globals, handles segment commands, and when a spawn command appears allocates an object and fills its position and sprite pointer fields.
- `allocate_and_init_object_with_offsets`: Calls an allocator, stores the returned pointer in a global and fills the object with type 10, a data pointer, zeroed fields, and a list of offsets adjusted by a global base until a zero terminator.
- `spawn_sixteen_objects_with_random_offsets`: Loops 23 times calling an allocator, sets each object's type to 1, copies a data pointer, calls a random routine, and conditionally fills offset lists adjusted by a global base before storing type 3 and the loop counter.
- `offset_table_data_bytes`: These eight ori.b instructions are data bytes, not executable code; they form a table of offset values used by the object setup routines.
- `keyboard_scan_code_table_a`: Writes a fixed sequence of immediate byte values into d0 with ori.b, forming a data table of scan codes rather than executable logic.
- `keyboard_scan_code_table_b`: Writes another fixed sequence of immediate byte values into d0 with ori.b, a second data table of scan codes, not executable code.
- `check_keyboard_serial_and_clear_state`: Tests a flag bit in memory, reads the CIA keyboard serial register, rotates and inverts the byte, compares it to a fixed value, and clears two state variables when it matches.
- `joystick_fire_and_velocity_update`: Checks joystick bits and fire button, calls movement routines, reads JOY1DAT to detect direction, and adjusts the velocity value and sound volume register.
- `fire_button_edge_and_velocity_apply`: Reads the CIA fire button, compares it with the previous state to detect a press, then applies velocity to the position and updates the audio volume register.
- `decelerate_velocity_and_raise_speed_level`: Subtracts a fixed amount from the velocity value, clamps it at zero, and increments the speed level counter up to six while setting a flag bit.
- `ciaa_fire_button_timer_decrement`: Reads CIAA port A, tests the fire button bits, and decrements the countdown words at the two timer globals, reloading the first with five when the button is pressed.
- `set_d7_ff60_d0_ffff0000_jump`: Loads d7 with the value minus one hundred sixty and d0 with the high word all ones, then jumps to the shared drawing routine.
- `set_d7_a0_d0_10000_jump`: Loads d7 with one hundred sixty and d0 with the value sixty-five thousand five hundred thirty-six, then jumps to the shared drawing routine.
- `set_d7_minus_140_and_clear_d0`: Loads the immediate value minus 140 into d7, clears d0 to zero and jumps to the shared routine at address 3552.
- `set_d7_140_and_clear_d0`: Loads the immediate value 140 into d7, clears d0 to zero and jumps to the shared routine at address 3552.
- `update_objects_and_decrement_lives`: Loops over 24 object entries, subtracts a scroll offset, draws four sprites per active entry, and if none remain active clears a flag and decrements the BCD lives counter.
- `adjust_vertical_offset_and_draw_line`: Tests d6 for negative, folds it into d2, computes a row offset from d6 times forty and a scaled slope from d5, then calls the line-drawing routine with d7 as the count.
- `setup_ray_origin_and_toggle_pixels`: Computes a starting bitmap address from d7 and d0, picks a length from a table indexed by d6, then toggles bits along the ray in the screen buffer at a2.
- `draw_steep_line_toggle_bits`: Adds a fixed offset to d1, walks up to one hundred twenty-eight steps using d5 as the increment, and toggles individual bits in the bitmap pointed to by a2.
- `scan_objects_for_collision_flag`: Walks the object list at the global pointer, finds the first object of type ten, then scans all other objects for overlapping bounding boxes and sets bit zero of their flags word when a collision is found.
- `decrement_object_timer_by_speed`: Computes a threshold from the global speed value divided by the object's rate, compares it with the object's timer at offset 28, and if reached subtracts the object's step value, clamping to zero and clearing bit zero of its flags.
- `add_gravity_velocity_to_object_position`: If the global gravity flag is negative, loads the object's position and velocity, substitutes a fixed velocity when the level counter is at least four, calls the velocity routine and adds the result to the position at offset 28.
- `clamp_entity_x_to_target`: Checks word at offset 28 of the object in a0; if at least 70, moves the long at offset 16 toward the long at offset 48 by at most an address, snapping to the target when within range.
- `update_entity_motion_and_velocity`: Advances the object in a0: handles a countdown at offset 52, randomizes direction and speed via RangeRan, updates position at offsets 56 and 60 with wrap, computes vertical speed at offset 20, and sets screen coordinates at offsets 12 and 24.
- `iterate_entity_list_and_update`: Walks the pointer list at the global list base, dispatching each object by its type word to update movement, gravity and screen position, and calls the list cleanup routines when the list ends.
- `build_road_curve_scanline_table`: Reads track segment width and curve offset, computes per-scanline horizontal step values and writes them into the copper list buffer at the table pointer.
- `rebuild_track_copper_list`: Calls the copper setup, writes color and bitplane registers plus track palette entries into the copper list at the pointer, then restores saved registers and returns.
- `draw_hud_panel_copper_entries`: Writes bitplane and color register entries into the copper list, clears the copper control area, and appends HUD sprite and text entries before terminating the list.
- `build_sprite_pair_copper_list`: Writes a copper list of sprite and bitplane pointers into the buffer at the global pointer, clears eight longwords of sprite registers, then appends two sprite entries computed from the saved d0 word and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `build_status_panel_copper_list`: Fills the copper list buffer with bitplane and sprite pointers, clears eight sprite register longwords, appends two sprite entries offset from the global word, then calls the display routine with the buffer pointer swapped in.
- `build_scroll_and_score_copper_list`: Sets the display window registers, clears eight sprite register longwords, writes bitplane and sprite pointers into the copper buffer, calls the display routine, then appends a score sprite entry chosen from a table and returns.
- `build_five_sprite_entries_from_a2`: Saves d0-d2, then for five iterations reads a long from (a2), adds d0, and writes four words (sprite number, high word, next number, low word) to (a5), advancing a5 by eight bytes.
- `copy_thirtytwo_words_with_sprite_numbers`: Writes thirty-two pairs of words to (a5): an incrementing sprite number starting at 384 and the next word from (a3), advancing both pointers each iteration.
- `update_scroll_and_build_sprite_list`: Advances a scroll counter from a global, wraps it modulo thirteen, toggles a flag bit, then fills a sprite list at a1 with alternating color words and computed positions until the position falls below 65536, storing list pointers and length.
- `blitter_dual_buffer_page_flip`: Waits for blitter idle, then programs blitter pointers, modulos, control and size registers to draw two alternating bitplane buffers and restores a saved word.
- `blitter_clear_with_saved_pointer`: Waits for blitter idle, then sets blitter control, modulo, pointer from a stored longword and size to clear a region.
- `blitter_fill_from_d0_pointer`: Waits for blitter idle, then programs blitter control, modulo, pointer from d0 and size to fill a region.
- `install_swap_screen_handler`: Writes a small handler structure at a fixed low memory address, opens the graphics library, clears a flag bit and jumps to the library's swap-screen entry.
- `set_display_window_and_copper`: Waits for vertical blank, programs the display window, DMA and interrupt registers, sets a flag, calls the copper setup routine and returns the copper list pointer and status word.
- `per_frame_copper_and_bcd_update`: Saves registers, updates the copper list scroll offsets, palette and sprite pointers, calls two subroutines and increments a packed BCD counter when the frame flag is set.
- `zero_filled_data_words`: Two zero words that decode as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `single_zero_data_word`: One zero word that decodes as ori.b #0,d0; this is a zero-filled data word referenced by address, not executable code.
- `allocate_and_configure_sound_channel`: Calls a routine to get a channel, stores it, fills channel fields from d1,d2,d3, swaps the audio pointer, sets bit 0 and volume 200, calls WaitTOF, then restores the pointer and volume 128.
- `reset_six_counters_and_mark_selected_slot`: Writes 15 into six word counters, then uses the value at the selection index to look up a table entry and stores 2463 into the indexed word of the slot array.
- `table_of_three_ori_immediates`: Contains three immediate OR bytes that set bits in d3, d1 and d7; these bytes are table data addressed by lea, not executable code.
- `poll_fire_button_and_run_frame_steps`: Calls the frame setup, waits on the CIAA port A fire button bit, conditionally calls four update routines, sets bit 5 of a flag word, and waits again on the button.
- `grid_start_main_loop`: Waits for vertical blank, clears DMA and several flag bits, runs the game update calls, swaps two longwords, clears a word and jumps to the exit routine.
- `joystick_menu_up_down`: Reads JOY1DAT bits to detect up or down movement, adjusts the menu selection word with wrap-around, then redraws and swaps display pointers.
- `joystick_select_and_adjust_slot_counter`: Checks the mode word, reads JOY1DAT to detect left or right movement, then increments or decrements the slot counter within 64 to 90 and calls two update routines.
- `draw_selected_char_at_cursor`: Reads the cursor index and character code, substitutes a period when the code equals 64, writes the character into the screen buffer, then calls WaitTOF and PrintIText to draw it at the cursor position.
- `copy_cursor_char_and_dispatch_state`: Copies the character at the cursor index into a two-byte buffer, clears the second byte, draws it via PrintIText, then dispatches through a state table unless the state equals 4, in which case it returns minus one.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `cycle_slot_swap_buffers_and_wait`: Increments a slot counter wrapping at 5 until a table entry is not minus one, calls two unnamed routines, waits for vertical blank, swaps two longword pointers and clears a word flag.
- `toggle_counter_and_set_d0`: Sets d0 to 1, tests the longword at 28764, sets d0 to 2 if it is zero, increments the word at 26962, and clears that word when it reaches d0.
- `queue_sound_command_with_counter`: Loads d0 with 1216, d1 with 8, d2 with 1 and d3 with the word at 26962, then jumps to the sound command routine.
- `toggle_bit6_at_468e`: Flips bit 6 of the byte at 18062 and then falls through into the following routine.
- `set_sprite_geometry_and_jump`: Loads d0 with 1168, d1 with 8, d2 with 12 and clears d3, then tests bit 6 of the longword at 18062 and jumps to the shared continuation.
- `advance_stage_counter_and_dispatch`: Calls the routine at 10584, compares the word at 28208 with 2, and if not equal increments that word, calls the routine at 11056 and jumps onward.
- `read_level_byte_into_sprite_width`: Loads a1 with the table at 28062, reads the byte indexed by the word at 28208 into d1, and if it equals 46 stores 64 into the word at 28206.
- `reset_counter_and_call_three_helpers`: Clears the word at memory offset 6e30, then calls three nearby subroutines in sequence and branches back to an earlier point.
- `blit_swap_buffers_and_decrement_counter`: Sets a counter to 196, calls a setup routine, waits for beam and copper via graphics library, swaps two longwords at 46AA and 46AE, clears word at 465C, then subtracts 4 from the counter and loops until negative.
- `blit_swap_buffers_and_increment_counter`: Calls a routine with arguments 4, 0 and 1, waits for beam and copper via graphics library, swaps the longwords at 46AA and 46AE, clears word at 465C, then adds 4 to the saved counter and loops while at most 200.
- `cycle_bplcon_scroll_and_swap_buffers`: Waits for vertical blank and time-of-frame, swaps the two longword scroll pointers, clears the scroll counter word, then advances d0 by four and repeats while it stays at or below two hundred.
- `draw_five_row_score_panel`: Optionally calls two helpers, scans a pointer list to pick a player index, stores it in the score table, then loops five times printing three score values and two labels with PrintIText at increasing vertical positions.
- `draw_score_digits_for_each_entry`: Walks twenty-four entries in the object list, and for each type one or ten entry prints its text at the stored coordinates using the graphics text call, then advances the coordinate pointer by four.
- `format_unsigned_decimal_into_buffer`: Divides d0 by d1 repeatedly, writing decimal digit characters plus a trailing space or terminator into the buffer at a0, and restores d2 before returning.
- `clamp_velocity_word_to_range`: Clamps the signed word at the global velocity address between positive and negative 4096, storing the limit when exceeded, then returns.
- `joystick_adjust_velocity_word`: Reads joystick port one fire bits to nudge the global velocity word up or down by 180, otherwise decays it by 180 toward zero, then returns.
- `allocate_screen_bitmaps_and_palettes`: Opens screen and window, allocates rasters and bitmaps, fills bitmap pointer tables and palette arrays, returning zero on failure or the last handle on success.
- `load_32_entry_palette_into_screen`: Reads 32 packed palette bytes from a table, splits each into red, green and blue nibbles and calls SetRGB4 on the screen viewport for each entry.
- `run_game_state_update_sequence`: Calls seven fixed update routines in order and then jumps to a final routine, chaining the per-frame game logic steps together.
- `draw_three_byte_hex_values`: Reads three bytes from the display buffer, splits each into high and low nibbles, and calls the digit drawing routine six times with vertical positions from twenty-nine to sixty-eight.
- `draw_byte_469c_two_nibbles`: Loads the byte at 469c, shifts its high nibble right by four, and calls the glyph blitter at offsets 177 and 184 to draw both nibbles.
- `count_players_and_draw_digit`: If 469c is nonzero, counts active player entries whose field at offset 56 exceeds 66bc, divides the count by ten, and draws quotient and remainder digits at offsets 96 and 103.
- `compute_scaled_vertical_offset`: Reads a table byte indexed by a word counter, scales it by a word factor and a shifted long value, divides, then either stores half the difference or resets the accumulator and stores the new value.
- `draw_three_digit_counter_glyphs`: Divides a long counter by one hundred, ten and one, passing each digit and a screen offset to a glyph drawing routine, ending with a jump to the last draw.
- `load_bitmap_base_and_jump`: Loads the pointer at address 4624, follows its offset 12 field into a0, adds 80 bytes to a0 and branches to the drawing code at 340c.
- `blit_tile_row_into_bitmap`: Reads a tile pointer from the level table indexed by d0, then for each of d0 rows copies shifted tile bytes into the bitmap at a0 using mask d3, advancing a0 by 40 bytes per row.
- `scroll_grid_up_one_tile_row`: Compares vertical scroll counters divided by eight, and if scrolled past, decrements the scroll offset, resets the tile column, updates the object position and calls the tile redraw routine.
- `reset_grid_scroll_and_redraw_column`: Sets the scroll counters and object fields to their starting values, then repeatedly calls the tile redraw routine while advancing the object's vertical position by eight until it passes the limit.
- `step_grid_cursor_toward_target`: Reads the current and target grid positions, divides both by six, and moves the cursor one step toward the target, updating the position word, sprite offset and animation frame until they match.
- `set_sprite_frame_and_anim_base`: Writes frame six to offset twenty of the object, loads the animation base offset into d0 and jumps to the animation setup code.
- `start_animation_with_wait_tof`: Adds the animation base to d0, stores it at offset seventy-two, clears offset sixty-eight, swaps the graphics library pointer, sets the animation flag and delay, calls WaitTOF and the animation routine, then restores the pointer.
- `allocate_bitplane_buffers_loop`: Saves a2/a6, computes a byte size from d2 shifted right three times times d3, calls the memory allocator with type 10002, stores each returned pointer through a2 and repeats d4 times, branching away on failure.
- `wait_blitter_and_set_blit_pointers`: Copies a0 to a2, waits until the blitter busy bit in DMACONR clears, then clears BLTCON0 and BLTCON1, writes 256 to BLTSIZE, a2 to BLTDPTH and 1448 to BLTDMOD.
- `compute_scaled_offset_from_d0_d1_d2`: Subtracts d0 from d1, shifts the difference left five and calls a scaling routine, then derives a value from d2 by adding half to double it, doubling again when d4 is at most an address, returning it in d0.
- `compute_scaled_horizontal_velocity`: Multiplies a shifted long value by a speed factor, adjusts for a flag bit, subtracts the pushed input, then scales the result by a word mode and sign before returning it in d0.
- `interpolate_speed_from_table`: Reads a byte from a table indexed by a global word, divides forty times eleven by it, then interpolates between two table words using the remainder and returns the result shifted left eight in d0.
- `compute_divisor_from_seed`: Stores d1 into a global long, builds a divisor in d2 from its bytes and bits, adjusts it until dividing d1 by it leaves a remainder within one, and returns with d2 holding the divisor.
- `open_examine_read_file_block`: Locks and opens the named file, examines it, reads a block into a buffer at an offset, closes and unlocks it, returning the read length or zero on failure.
- `open_examine_read_close_file`: Opens the named file via dos.Open, examines it into a buffer, reads its contents, closes it, and returns the read handle or zero on failure.
- `loop_load_69_sprites_via_library`: Calls a library function 70 times with d1 set to 65536 and d0 to 156, passing each returned pointer to a routine, then returns.
- `dispatch_pending_object_via_two_helpers`: Saves d1-d6 and a0-a6, loads the pointer at the global slot into d0, and if non-zero calls two helper routines with it in a0 before restoring registers and returning.
- `load_global_slot_pointer_and_branch`: Loads the address of the global pointer slot into a1 and branches forward into the shared helper code, without touching any other register.
- `append_pointer_to_global_list`: Scans the global pointer list starting at the fixed base until a zero longword, stores a0 there, and returns, appending the given pointer to the list.
- `remove_matching_long_from_list`: Saves d0, a1 and a2, scans the longword list at a1 for the value in a0, then shifts the following entries down over the match until a zero terminator, and restores registers.
- `store_score_and_sort_entries`: Writes a value into offset 56 of the structure at the pointer in memory, calls the sort routine, then walks a pointer list updating entry fields and counters.
- `bubble_sort_pointer_list_by_offset`: Repeatedly compares the longword at offset 56 of adjacent pointed-to structures and swaps the pointers when out of order, until no swaps occur.
- `bubble_sort_list_by_word_offset8`: Walks the pointer list at the global table, swapping adjacent entries when the word at offset 8 of the first is less than the second, repeating passes until no swap occurs.
- `bubble_sort_list_by_long_offset40`: Walks the pointer list at the global table, swapping adjacent entries when the long at offset 40 of the first is less than the second, repeating passes until no swap occurs.
- `bubble_sort_list_by_word_offset20`: Walks the pointer list at the global table, swapping adjacent entries when the word at offset 20 of the first is less than the second, repeating passes until no swap occurs.
- `open_top_scores_read_and_close`: Opens the topScores file for reading, reads its contents into a buffer, then closes the file handle and returns.
- `insert_score_into_top_scores_table`: Clears the selected-entry pointer, calls the score reader, then if conditions hold shifts the high-score table entries down and writes the updated topScores file.
- `draw_high_score_table_entries`: Loops over the high-score table, printing each entry's text at screen positions and formatting the score value, highlighting the currently selected entry.
- `open_bobs_file_and_alloc_buffer`: Locks and examines the bobs file, reads its size into a global, then allocates a memory buffer and stores the pointer for later loading.
- `open_bobs_seek_and_read_chunk`: Opens the bobs file, seeks to an offset, reads a chunk into the allocated buffer at the stored address, then closes the file.
- `open_bobs_read_into_buffer_and_close`: Opens the bobs file, reads a fixed-size block into the previously allocated buffer, and closes the file handle.
- `load_anim_table_and_store_result`: Loads the address of animTable into a0, calls the routine at offset 38ba, stores the returned d0 into the global at 6a7c, and returns.
- `run_animation_frame_sequence`: Sets a flag, walks the animation table at 6a80, and for each nonzero entry calls the frame routine and conditionally invokes several subroutines on the objects at 6cb8 and 6cbc.
- `check_object_type_ten`: Moves d0 into a6, compares the word at offset zero of that object with ten, and branches to the shared exit when equal.
- `blit_rotated_sprite_strip_to_screen`: Reads sprite descriptor fields from a4, computes blit offsets and modulo values, writes them to the custom chip blitter registers, then loops four times copying bitplane pointers and starting the blit.
- `reset_level_state_and_place_objects`: Calls two setup routines, computes a scaled value into a global, clears many game state words and longs, then walks the object list assigning random positions and velocity values.
- `spawn_random_timed_object`: Calls the object allocation routine, sets the new object's field at offset 68 to 10, stores its pointer and a countdown of 75, then sets a random delay between 100 and 175 and marks a flag.
- `spawn_effect_with_three_offsets`: Calls the allocator, and if it returns a pointer, stores type 2, three base-plus-offset pointers at offsets 112, 116 and 120, and sets words at offsets 22 and 16.
- `countdown_spawn_and_shrink_effect`: Decrements one of two countdown words; when the first hits zero it spawns the effect, copies a word from a source object and sets a delay, and when the second expires it shrinks the stored object's size word.
- `zero_filled_data_area_2`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `unpack_12bit_pixels_to_bitplanes`: Reads three packed bytes per iteration from a source table, expands them into a 12-bit value, writes it to the destination pointed by a2, advances a0 by two bytes and repeats d2 plus one times.
- `scoreboard_setup_and_jump`: Loads a data pointer and calls a setup routine, clears the longword at offset four of the scoreboard structure, loads a value into d7, calls another routine and jumps onward.
- `show_novice_highscore_screen`: Loads the novice high score screen pointer into a0, calls the screen setup routine, clears the longword at offset 4 of the structure at a2, loads a value into d7, calls the display routine, then jumps to the shared exit code.
- `show_track_highscore_screen`: Loads the track high score screen pointer into a0, calls the screen setup routine, clears the longword at offset 4 of the structure at a2, loads a value into d7, calls the display routine, then jumps to the shared exit code.
- `show_loading_highscore_screen`: Loads the loading high score screen pointer into a0, calls the screen setup routine, clears the longword at offset 4 of the structure at a2, loads a value into d7, calls the display routine, then jumps to the shared exit code.
- `show_champion_high_score_screen`: Loads the champion high score screen text pointer into a0, calls the display setup, clears the score buffer field at offset 4 of a2, loads a value into d7 and calls the score routine before jumping to the main loop.
- `show_scores_high_score_screen`: Loads the scores high score screen text pointer into a0, calls the display setup, clears the score buffer field at offset 4 of a2, loads a value into d7 and calls the score routine before jumping to the main loop.
- `draw_selected_high_score_entries`: Saves four globals, indexes a table by the selected entry times twelve, calls the draw routine on three table pointers and a repeated pointer, restores the globals and jumps to the main loop.
- `init_sound_channel_pointer_table`: Calls two subroutines, stores 50 and 120 into fields of the structure at a2, then adds 40 to each of four consecutive longwords starting at the table pointed to by a3.
- `decompress_rle_rows_to_bitmap`: Checks a flag byte, saves row pointers, then expands run-length encoded bytes from a source buffer into each destination row, advancing pointers by the row stride and restoring the saved pointers.
- `setup_three_audio_channels_and_jump`: Allocates three sample buffers via a subroutine, programs the AUD0, AUD1 and AUD2 hardware registers with pointers, lengths, periods and volumes, then jumps to the sound start routine.
- `set_audio_channel_zero_from_sound_data`: Calls two setup routines, stores the returned pointer, then programs AUD0LCH, AUD0LEN, AUD0PER and AUD0VOL from the sound data block and sets volume to 63.
- `open_audio_channel_two_and_start_playback`: Opens audio channel 2 through the library, sets the channel period and volume, closes the device, then jumps to the playback routine with the custom chip base in a0.
- `zero_filled_data_area_3`: Two zero words that decode as ori.b instructions; this is a zero-filled data area referenced by address, not executable code.
- `zero_filled_data_area_4`: Contains a single zero word that decodes as ori.b #0,d0; this range is zero-filled data, not executable code.
- `set_a0_offset_9c_to_512_return_one`: Writes the immediate value 512 to the word at offset 156 from a0, sets d0 to 1 and returns to the caller.
- `call_38ba_test_d0_branch_if_zero`: Calls the routine at address 38ba, tests the low word of d0 and branches to address 9f8 when the result is zero.
- `parse_8svx_header_and_set_sample_rate`: Saves d0, scans the 8SVX chunk headers for BODY and VHDR, computes a rate from a constant divided by the word at offset 16 of the sample header, stores it, then stores the body pointer and returns.
- `compare_string_until_null_or_mismatch`: Compares the null-terminated string at a1 against the bytes at a0, advancing both pointers; returns with zero flag set on full match or clear on mismatch or end of a1.
- `parse_ilbm_header_and_store_bitmap_fields`: Calls a file loader, then scans the ILBM FORM chunks for BMHD, CMAP and BODY, copies the palette bytes, divides the body length by three, stores the bitmap pointer and returns.
- `_RangeRan`: Advances the 32-bit RangeSeed linear feedback shift register once per bit of the requested range, stores the new seed, then returns a value scaled to that range in d0.
- `_FastRand`: Takes the seed passed on the stack, doubles it, and if the shift carries out or the result is zero, XORs it with the feedback constant before returning it in d0.
- `allocate_memory_with_header`: Adds twelve to a value from a stack parameter, calls a memory allocation routine with that size, and if successful, stores the pointer in a local variable, writes the incremented size to offset eight of the allocated block, and calls another routine; otherwise it returns zero.
- `finish_alloc_after_success`: Aligns the allocated block pointer, writes the requested size plus twelve at offset eight, calls the initialization routine with the block and a global, then returns the block address plus twelve.
- `free_linked_memory_blocks`: Walks a singly linked list whose head is held in a global, freeing each node with the size stored at its offset and following the next pointer until the list ends.
- `append_node_to_doubly_linked_list`: Links the node passed as second argument after the tail of the list whose head is the first argument, updating next and previous pointers and the head when the list was empty.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_FindTask`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `_SetSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `_OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `stack_check_then_jump_zero`: Builds a small a6 frame, compares the stack pointer a7 against the value at address 4, and jumps to address zero if the stack is below it.
- `allocate_sound_buffer_and_store_pointer`: Clears a field at offset -38 of a6, pushes a size and flags, calls the allocator, drops the arguments, stores the returned pointer at offset -42 of a6 and returns 1 if allocation failed.
- `open_and_validate_grid_file`: Pushes fields from the a6 frame, calls several subroutines to open and parse a file, clears a byte at -18, tests the result, and sets -38 to -1 on failure before branching.
- `open_file_and_check_size`: Calls open on the name at offset -20 of a6, reads the word at -22 as a mode, opens the file at -42, seeks to offset 924a, reads 3778 bytes, and stores the result at -26 of a6.
- `allocate_and_validate_memory_block`: Adds 16 to the value at offset -26 of a6, stores it at -34, calls the allocation routine with size 3778 and tag 924a, stores the result at -30, and sets status at -38 to 3 on failure or 4 if the returned size is outside the expected range.
- `check_allocated_block_size_range`: Loads the pointers at offsets -30 and -26 of a6, subtracts them, and clears the status at -26 only if the difference lies between the two immediate bounds; otherwise branches away without changing it.
- `free_block_set_status_four`: Stores 4 in the status at offset -26 of a6, calls the memory free routine with the pointer at -42 and size 6ef0, then returns that status after releasing the a6 frame.
- `stack_check_then_tail_jump`: Opens a small a6 stack frame, compares the stack pointer with the value at absolute address 4, and branches past a jump to address zero when the stack is high enough.
- `copy_20_bytes_set_cia_and_audio_regs`: Copies 20 bytes from a fixed source to the caller's buffer, clears CIA bits, writes all ones to two CIA ports, stores a shifted bit and a longword in the buffer, then programs audio and disk registers before returning.
- `stack_check_then_jump_absolute_2`: Builds a small a6 frame, compares the stack pointer with the value at absolute address 4, branches past the jump if the stack is high enough, otherwise jumps to absolute address 0.
- `set_ciaa_interrupt_mask_7fff`: Stores the CIA base address at offset minus four of a6, writes 32767 to the CIA interrupt mask register, then ORs the word at offset ten of a6 with 49152 and writes it to the same mask register before returning.
- `stack_pointer_guard_check`: Opens an a6 frame, compares the stack pointer a7 against the longword at address four, branches past a jump to address zero if a7 is higher, otherwise jumps to address zero.
- `disk_motor_pulse_and_fire_wait`: Calls a routine with the pointer at offset eight of a6, clears word at offset six, pulses the disk select line via CIA-B port B, then polls CIA-A port A fire bits and calls another routine until the button state changes.
- `toggle_ciaa_fire_and_wait_loop`: Sets bit 1 of CIAA_PRA, reads CIAA_PRA into d0, ORs with all bits except bit 4, increments d0 and branches back if nonzero, then pushes and calls a routine.
- `delay_then_call_with_a6_arg`: Pushes the immediate value 15000, calls a delay routine, then pushes the longword at offset 8 of a6 and calls another routine before unlinking a6 and returning.
- `check_stack_pointer_and_jump_zero`: Compares a7 with the longword at address 4, branches past a jump to address zero if a7 is higher or equal, otherwise jumps to address zero.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `stack_check_and_jump_to_zero`: Links a6 to a zero-sized frame, compares the stack pointer a7 with the value at absolute address 4, branches past the jump if a7 is higher, otherwise jumps to address zero.
- `shift_count_right_twice_and_loop`: Loads the longword at offset 8 from a6, shifts it right by two, stores it back, then decrements it and repeats while nonzero, calling the routine at the branch target each pass.
- `unlink_a6_and_return`: Releases the a6 stack frame with unlk and returns immediately to the caller.
- `stack_check_frame_and_trap_on_overflow`: Opens an a6 stack frame, compares the stack pointer with the value at absolute address 4, branches past a jump to address zero if the stack is still above it, otherwise jumps to address zero.
- `clear_flag_and_call_two_cleanup_routines`: Loads the pointer from offset 8 of the a6 frame, tests byte at offset 1, and if nonzero clears it, then calls two subroutines with that pointer pushed, popping each argument, then unlinks and returns.
- `frame_setup_stack_guard_jump`: Creates an a6 frame with no local space, compares a7 against the longword at absolute address 4, and branches over an absolute jump to address 0 when the stack is valid.
- `start_play_if_not_started`: Reads the object pointer from the stack frame, and if its second byte is zero sets it to one, then calls three subroutines passing the object and a delay value of an address.
- `stack_check_and_jump`: Opens a stack frame of sixteen bytes, compares the stack pointer with the value at address four, and branches past a jump to zero if the stack is high enough.
- `init_copper_and_wait_fire`: Stores the custom chip base, clears a counter, sets a delay, writes the object's copper list pointer, calls a subroutine, then reads the CIA fire button and branches or sets a count of 25.
- `pulse_ciaa_port_a_and_count_down`: Clears then writes the value 46968 twice to offset 36 of the structure pointed to by a6-4, stores it at a6-8, decrements the counter at a6-16 and returns 22 after unlinking the frame.
- `read_joystick_fire_and_clear_counter`: Reads the word at offset 30 of the structure pointed to by a6-4 into a6-8, clears the word at offset 36, tests bit 1 of a6-5, and if set reads CIAA_PRA and returns 25 after calling a helper with the argument at a6+8.
- `stack_check_then_jump_to_zero`: Links an a6 frame, saves d2 and d3 on the stack, compares the stack pointer with the value at address 4 and jumps to address zero if the stack is below it.
- `split_axis_bits_and_compare_target`: Reads two longwords from the a6 frame, splits each into low bit and shifted half, stores four results in frame slots, and compares the word at offset 6 of the pointed structure with the second longword, returning if equal.
- `clear_ciab_fire_bit_when_target_zero`: Calls a routine with the frame pointer, compares two frame longwords, and if the first is zero clears bit 2 of CIAB port B, then branches onward.
- `set_ciab_fire_bit_and_delay`: Sets bit 2 of CIAB port B, calls a delay routine with 200, then compares two frame longwords and clears bit 1 of CIAB port B when the second is smaller.
- `toggle_ciab_fire_and_wait_loop`: Sets bit 1 of CIAB port B, then loops calling a delay routine with the count from the frame offset minus the base offset, decrementing until zero.
- `delay_then_store_word_and_call`: Pushes 15000 as an argument to a delay routine, writes the low word of the caller's value into offset 6 of the object pointer, then calls another routine with that pointer.
- `stack_check_and_jump_zero`: Opens an a6 frame, compares the stack pointer against the value at address 4, branches past a jump to address zero if the stack is high enough.
- `pulse_ciab_motor_line_low_then_high`: Reads CIAB port B, clears bit 0 to drive the motor line low, then sets bit 0 high again and calls a delay routine with 3000 before returning.
- `stack_check_and_jump_to_zero_2`: Creates an a6 frame, saves d2, compares the stack pointer against the value at address 4 and jumps to address zero if the stack is below it.
- `set_ciab_disk_select_from_parameter`: Sets bit 7 of CIAB port B, then clears the bit selected by the parameter byte and its inverted second byte, writing the result back to CIAB port B.
- `stack_check_then_jump_absolute`: Opens a6 stack frame, compares the current stack pointer with the value at absolute address 4, branches past the jump if a7 is above or equal, otherwise jumps to absolute address 0.
- `set_ciab_port_b_high_bits`: Reads CIAB_PRB, sets bits 3 through 6 with a logical or, writes the result back to CIAB_PRB, releases the a6 frame and returns.
- `stack_check_frame_with_saved_regs`: Opens a 24-byte a6 stack frame, pushes d5 through d7 onto the stack, compares a7 with the value at absolute address 4, branches past the jump if a7 is above or equal, otherwise jumps to absolute address 0.
- `decode_bitstream_into_grid_cells`: Reads two bit offsets from the a6 structure, converts them to word pointers in the grid buffer, then decodes a 16-bit bitstream into cell values, returning a value or minus one.
- `check_high_word_matches_and_return_value`: Masks d6 to its high word, compares it with d5, and if equal returns the longword at offset 16 of a6 after restoring d5-d7 and the frame.
- `advance_bitstream_and_loop_back`: Increments the longword at offset 16 of a6, shifts d6 left, decrements d7, and branches back into the bitstream decoding loop.
- `read_word_advance_stream_or_into_d6`: Clears d0, loads the pointer at offset minus twelve from a6 into a0, reads a word from it into d0, advances that pointer by two, ORs the word into d6, sets d7 to sixteen and branches onward.
- `return_minus_one_restore_registers_2`: Sets d0 to minus one as the return value, pops d5 through d7 from the stack, releases the a6 frame and returns to the caller.
- `signed_32bit_divide_d0_by_d1`: Divides d0 by d1 using shift-and-subtract over thirty-two bits, saving d2 through d5, applying signs from both operands, and returning the signed quotient in d0 with remainder in d1.
- `clear_d0_d1_restore_d2_d5`: Clears d0 and d1 to zero, then pops d2 through d5 from the stack and returns to the caller.
- `_Open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `_Close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `_Read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `_Write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `_Seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `_DeleteFile`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `_Rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `_IoErr`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `signed_divide_d0_by_d1_setup`: Saves d1-d4, copies d0 to d4, tests d0 and d1, negates d0 if negative, and branches to the division path or returns zero when d1 is zero.
- `signed_32bit_multiply_d0_by_d1`: Multiplies d0 by d1 using 16-bit mulu steps with swaps, negates the result if d4 is negative, restores d1-d4 and returns the product in d0.
- `_fclose`: Pushes d2, loads the file handle from the stack argument, calls the Close routine on it, discards the argument, restores d2 and returns.
- `validate_handle_and_check_flag`: Calls an init routine if a global flag is set, checks the handle argument is non-negative and below a global limit, then tests a byte flag and returns a pointer or sets an error code.
- `get_handle_pointer_from_argument`: Loads the handle argument from the stack, calls a lookup routine with selector six, adds the returned offset to a base and returns the resulting pointer.
- `find_free_file_slot`: Loops over the file slot count, calls a slot lookup routine, and returns the first index whose byte is zero, or sets error 24 and returns minus one if none are free.
- `check_slot_index_or_set_error`: Loads the local slot index, compares it with zero, and if nonzero returns it; otherwise stores error 24 and returns minus one.
- `toggle_open_flag_and_store_slot`: Looks up the slot pointer, XORs the caller's flag word with a global bit, stores it back, saves the pointer, and branches on bit 15 to choose a mode byte.
- `set_open_mode_flags_and_open_file`: Writes 128 to the byte at the pointer in -12(a6), then masks the long at 12(a6) with 3 and sets bits 64, 32 or 96 plus optional 8 in that byte depending on the value, returning -1 with error 22 for other values.
- `test_bit3_of_flags_word`: Tests bit 3 of the word at offset 15 from a6 and branches to the caller's continuation, loading 8 into d0 when the bit is set and otherwise falling through with d0 unchanged.
- `set_bit5_in_status_byte`: Loads 32 into d0, reads the byte pointed to by -12(a6) into d1, ORs them together and writes the result back to that byte before branching to the caller's continuation.
- `check_bit3_then_set_d0_eight`: Tests bit 3 of the byte at offset 15 from a6; if clear it branches away, otherwise sets d0 to 8 and branches to another routine.
- `set_flags_and_open_file`: Sets bits 5 and 6 in the byte pointed to by the pointer at offset -12 from a6, then branches to code that opens a file and stores the handle.
- `clear_byte_set_d0_and_unlink`: Clears the byte pointed to by the pointer at offset -12 from a6, stores 22 at absolute address zero, sets d0 to -1, releases the a6 frame and returns.
- `check_io_flags_and_set_error`: Tests bit 1 of the word at offset 14 in a6; if set, pushes the pointer at offset 8 in a6 and calls the delete routine, then masks the long at offset 12 in a6 with 768 and, if nonzero, stores 1006 at offset -8 in a6.
- `open_file_and_record_handle`: Stores 1005 at offset -8 in a6, calls _Open with the name at offset 8 in a6 and that mode, saves the returned handle at offset 2 of the structure pointed to by offset -12 in a6, and on failure records _IoErr and returns -1.
- `return_saved_result_from_frame`: Loads the long at offset -4 in a6 into d0, releases the a6 stack frame and returns, passing back the value stored there by the caller.
- `open_file_and_store_handle`: Builds a stack frame, pushes the pointer argument at offset 8 of the frame, calls a subroutine to open it, stores the returned handle at frame offset -8, and returns -1 if the handle is null.
- `check_file_flag_bit_two`: Clears d0, loads the stored handle from frame offset -8 into a0, reads the byte it points to into d1, saves d0 at frame offset -4, and returns after testing bit 2 of that byte.
- `close_file_and_clear_flag`: Loads the stored handle from frame offset -8, passes its field at offset 2 to the Close library call, clears the byte at the handle, and returns the value saved at frame offset -4.
- `delete_file_and_handle_error`: Attempts to delete a file and, if the result is zero, calls an error routine, stores the error in a global variable, sets a second global variable to two, and returns negative one; otherwise it returns zero.
- `return_zero_with_frame_restore`: Sets d0 to zero as the return value, releases the a6 stack frame and returns to the caller.
- `open_file_and_store_handle_2`: Pushes open mode 1005 and a path pointer, calls Open, cleans the stack and stores the returned file handle in a global.
- `read_byte_or_cached_char`: Checks two global flags, calls setup routines if needed, then returns a cached byte from a global and clears it, or reads one byte from the file handle into a stack slot and returns it.
- `read_one_byte_from_handle`: Pushes a count of one and a stack buffer address, calls the Read vector with the global file handle, cleans the stack, and returns the byte read.
- `write_byte_to_handle`: Checks two global flags, calls setup routines if needed, then writes the single byte argument to the global file handle via the Write vector and returns that byte.
- `check_flag_and_return_ff`: Opens an a6 stack frame, tests the byte at absolute address 12, and if it is zero branches to the next routine; otherwise returns 255 in d0.
- `store_long_into_byte_flag`: Loads the longword at offset 8 from the a6 frame, stores its low byte to absolute address 12, then releases the frame and returns.
- `close_marked_entries_in_table`: Loops over a count at a fixed longword, and for each six-byte table entry whose first byte is nonzero and lacks bit 2, calls _Close on the pointer at offset 2.
- `call_other_with_a6_offsets`: Pushes the longwords at offsets 12 and 8 of a6 onto the stack, calls the routine at absolute address 358, discards the eight argument bytes, then restores d5-d7 and a6 and returns.
- `check_signal_mask_3000`: Opens a stack frame, calls the signal function with mask 12288 and no extra argument, masks the result with 12288, stores it in the frame and returns zero if no bits are set.
- `check_flag_and_call_50e`: Tests the longword at absolute address zero, and if nonzero pushes 94 and calls the routine at absolute address 1294, then compares the frame longword with 8192 and loads 68 or 68 into d0.
- `send_two_control_codes_then_index_loop`: Sends codes 67 and 10 through a subroutine, then loops a counter from zero up to a global limit, passing each index to another subroutine before returning.
- `push_two_zero_args_and_call`: Pushes two zero longwords onto the stack, calls the routine at address zero, then adds 8 to a7 and branches to the next block.
- `return_saved_long_from_a6_frame`: Loads the longword stored at offset -4 of a6 into d0, releases the a6 stack frame with unlk and returns to the caller.
- `unlink_a6_frame_and_return`: Unlinks the a6 stack frame and returns immediately to the caller without touching any other register or memory.
- `init_intuition_request`: Opens the intuition library, stores the handle in a global variable, and performs an auto request with fixed dimensions.
- `close_twenty_file_handles_loop`: Loops twenty times, obtains each file handle via a helper and closes it with fclose, then continues into further cleanup code.
- `increment_counter_until_nonnegative`: Clears the longword at offset minus four from a6, copies it to d0, compares d0 with the value at address zero, and if d0 is negative pushes it, calls the routine at 918, cleans the stack, increments the counter and branches back.
- `call_function_with_two_frame_args`: Pushes the longwords at offsets twelve and eight from a6 onto the stack, calls the routine at address zero, removes the eight bytes of arguments, releases the a6 frame and returns.
- `_AutoRequest`: Saves d2-d3 and a2-a3/a6, loads the library base into a6, copies six stack arguments into a0-a3 and d0-d3, calls the library offset minus 348, restores registers and returns.
- `zero_filled_data_word_at_00b6`: A single zero word that decodes as ori.b with immediate zero to d0; it is zero-filled data referenced by a handler pointer, not executable code.
- `zero_filled_data_word_at_00ba`: One zero word in a zero-filled data area, decoding as ori.b immediate zero to d0; referenced by a data handler pointer rather than being real code.
- `zero_filled_data_word_at_00be`: A lone zero word treated as ori.b immediate zero to d0; this is padding data pointed to from hunk data, not a working routine.
- `zero_filled_data_area_5`: Contains only a zero word that decodes as an ori.b immediate instruction; this is zero-filled data, not executable code.
- `allocate_and_build_bob_anim_table`: Allocates two memory blocks via the graphics library, stores the second pointer globally, then fills animation table entries with bob pointers, dimensions and offsets, patching matching entries in the bobs table.
- `fill_remaining_bob_anim_entries`: Copies bob coordinates and dimensions into successive animation table entries, computes row sizes and offsets, then jumps to the table finalizer for the animTable and bobs lists.
- `open_and_duplicate_workbench_library`: Loads the library base from the pc-relative pointer into a6, opens library version 1006, then duplicates and closes the returned base before returning.
- `open_workbench_and_load_icon_data`: Opens the workbench library, reads icon data into a buffer, opens the icon library, loads the icon structure and returns the icon data pointer.
- `restore_registers_and_return_zero`: Pops d2 through d7 and a2 through a6 from the stack, clears d0 to zero as the return value, and returns to the caller.
- `or_blit_masks_into_dest`: Reads the byte at offset 8 of a4 minus two as a count, then for each even destination offset ORs together the words from two source tables and stores the result into the destination table pointed to by the third longword.
- `decompress_rle_planar_bitmap`: Reads a bitmap header from a2 and a4, unpacks run-length encoded planar pixel data into the destination buffer and returns when the row count is exhausted.
- `compute_bitmap_row_byte_width`: Compares a5 with d3 and branches away if a5 is higher; otherwise tests the byte at offset 10 of a2 and computes the row byte width from the word at a2 by rounding to a multiple of sixteen and shifting right three.
- `rle_decode_byte_run_to_a5`: Reads a signed byte from (a3), advances a3, and if positive copies that many following bytes to (a5); if negative and not the end marker, repeats the next byte that many times, then branches back to the caller's loop.
- `copy_single_byte_and_loop`: Copies one byte from (a3) to (a5), advances both pointers, decrements d0 and repeats while d0 is not minus one, then branches back to the shared decode loop.
- `finish_decode_restore_and_call`: Masks a word at -2(a5), adds d2 and a table offset to (a4), reloads a5 from the table, copies the saved d1 onto the a6 stack, restores registers, calls the next routine and returns.
- `unpack_rle_bitmap_to_buffer`: Saves registers, reads a source descriptor from a2, stores a pointer into the buffer at the address held in the global pointer, computes row stride from the width, and expands the first row of run-length data into the destination buffer.
- `expand_rle_rows_into_bitmap`: Walks the run-length stream in a3 writing decoded bytes through a5, decrementing d2 and d5 per byte, refilling d5 from the width in a2 and stepping a5 back by d3 at each row end.
- `copy_literal_run_bytes`: Copies d0 plus one literal bytes from a3 to a5, decrements d2 and d5 for each byte, then jumps back to the run-length decoder to continue the next control byte.
- `restore_registers_and_return`: Pops registers d2 through d7 and a2 through a6 from the stack, then returns to the caller.
- `parse_ilbm_header_chunks`: Saves registers, opens the file, then scans IFF chunks for BMHD, CMAP and body data, storing pointers and chunk sizes in globals.
- `copy_cmap_palette_bytes`: Copies a counted run of palette bytes from the chunk pointer into the color map buffer, then advances to the next chunk.
- `parse_count_and_copy_bytes`: Reads a longword count from (a0), stores count divided by three into memory at 06DA, copies that many bytes from (a0)+ to 06DC, then scans forward calling a routine until a match or dbra expires.
- `advance_a0_store_pointer_restore_regs`: Adds four to a0, stores the resulting pointer into memory at 05F6, restores d2-d7 and a2-a6 from the stack and returns to the caller.
- `jump_to_shared_exit_code`: Unconditionally jumps to the shared exit code at 05CA, transferring control without touching any registers or memory.
- `compare_strings_until_null_or_mismatch`: Compares bytes at a1 and a0, returning when a1 holds a null byte or a byte differs, advancing both pointers as it goes.
- `restore_stack_pointer_from_abs_8f0`: Loads the longword at absolute address 8f0 into a7, resetting the stack pointer, then returns to the caller.
- `reset_stack_and_call_twice_with_arg`: Loads a7 from absolute address 4c, calls the routine at address zero, pushes immediate 20 on the stack and calls address zero again.
- `scan_string_until_null`: Walks a0 through the string at the caller-supplied pointer, incrementing a local counter until a zero byte is found, then returns with d0 holding that zero byte.
- `return_saved_string_offset`: Loads the local counter from the stack frame into d0, releases the a6 frame and returns it to the caller.

### Grid Start (1987)(Anco)[cr A-Ha].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `bootblock_alloc_and_animate_bitplanes`: Allocates chip memory, opens graphics.library, sets up copper and DMA, then loops reading VPOSR/VHPOSR to shift bitplane words and toggle playfield colors until fire is pressed, then frees memory.
- `build_copper_list_and_set_video_registers`: Writes a copper list of 32 sprite entries into the buffer at offset 8 of a5, patches two words, points COP1LCH at it, then programs BPLCON1, BPLCON2, DIWSTRT, DIWSTOP, DDFSTRT, DDFSTOP and clears BPL1MOD and COP1LCL.
- `emit_copper_control_words_and_shift_table`: Appends fixed copper control longwords to the buffer at a0, calls two helpers to emit more words, then copies 32 words backwards from the table at 872 to 808 and stores the saved word there.
- `load_copper_table_pointers`: Loads the address of the table at 808 into a1 and the address of the table at 872 into a2, then falls through to the following code.
- `build_copper_list_from_palette_words`: Loops over palette words at a1, writing each colour index, value and inverted value into the copper list at a0 while incrementing d0 by one million per entry, wrapping a1 to a2 and returning when d0 equals d2.
