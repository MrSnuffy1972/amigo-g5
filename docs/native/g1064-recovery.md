# Game 1064 (populous ii - trials of the olympian gods (1991)(electronic arts)[cr dlt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01064). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1831344 top-level bytes in 176 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 34 of 34 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 525 of 525 named (55 author symbols), 525 of 525 described (71 by rule, 93 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1852 of 1852 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14165 of 14166 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 882124 |
| code | 46582 |
| data | 713215 |
| padding | 52207 |
| unused | 137216 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:Install | 2 | 267 | 8 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:loaderprogram123 | 1 | 76 | 2 |
| noprefetch | 1 | 13 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:DELIGHT.EXE | 3 | 27 | 2 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:CREDITS.EXE | 2 | 167 | 7 |
| icon.library | 10 | 1375 | 109 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:populous.ii | 6 | 167 | 7 |
| run | 2 | 117 | 2 |
| ask | 2 | 117 | 2 |
| assign | 1 | 938 | 7 |
| ENDCLI | 2 | 117 | 2 |
| newcli | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| Execute | 1 | 888 | 12 |
| copy | 2 | 1685 | 5 |
| Makedir | 1 | 112 | 1 |
| cd | 2 | 117 | 2 |
| if | 2 | 117 | 2 |
| iconx | 12 | 806 | 37 |
| LOADWB | 12 | 536 | 30 |
| wait | 1 | 238 | 1 |
| DISK-VALIDATOR | 1 | 386 | 1 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 2 of 2).adf:Credits.exe | 2 | 1110 | 62 |
| ICON.LIBRARY | 10 | 1375 | 109 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 2 of 2).adf:Challenge | 7 | 235 | 8 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 2 of 2).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 607 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 290 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 185 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 243 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 483 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 724 | 28 |

## Cases handled

- populous ii - trials of the olympian gods (1991)(electronic arts)[cr dlt](disk 1 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:awards.pak: high-entropy data file (7.70 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr dlt](disk 1 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:hands.pak: high-entropy data file (7.68 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr dlt](disk 1 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:end.pak: high-entropy data file (7.66 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr dlt](disk 1 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:judge.pak: high-entropy data file (7.68 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr dlt](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 2 of 2).adf:NAWARDS.PAK: high-entropy data file (7.70 bits/byte), classified as packed data
- runtime segment an address: 414 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 231 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 180 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 145 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 303 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 346 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr dlt](disk 1 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:DELIGHT.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr dlt](disk 1 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:CREDITS.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr dlt](disk 1 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:populous.ii: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr dlt](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 2 of 2).adf:Challenge: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:Install

- `install_open_dos_and_run_installer`: Program entry that saves exec base, opens dos.library, allocates memory, opens the input and output streams, then runs the installer and returns a status code.
- `close_installer_streams_and_alert`: Closes the two dos file handles at offset 92 of a4, and on other paths raises exec alerts and closes the library before returning.
- `free_installer_memory_and_close_library`: Frees the allocated memory block via a2, closes dos.library, restores the stack from the task structure and returns the saved d2 value.
- `open_console_and_close_after_setup`: Saves all registers, opens the CON: window via dos.Open with mode 1005, stores the file handle in absolute memory, calls the setup routine, then closes the handle via dos.Close and restores registers.
- `prompt_and_create_populous_dir`: Prints a prompt, reads up to 80 bytes into a buffer, strips the newline, calls dos.CreateDir on the entered path, and on failure calls dos.UnLock before returning.
- `copy_file_with_x_to_o_rename`: Builds source and destination paths from the entered name, renames .x to .o when needed, opens both files with dos.Open, copies an address bytes via dos.Read and dos.Write, closes them, and returns OKOK, FAIL or END!.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `write_string_after_scanning_length`: Saves all registers, scans the null-terminated string at a0 to count its bytes, then calls dos.Write with the length in d3, the string in d2 and the file handle from a global, restoring registers before returning.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:loaderprogram123

- `loader_open_libraries_and_jump_to_game`: Program entry that opens dos.library and graphics.library, prints the loading message, opens the game file, patches a jump instruction into memory and jumps to the loaded game code, returning 20 on failure.
- `copy_bytes_and_clear_tail_then_return`: Copies d0 bytes from a0 to a1, writes a null terminator, clears d1 further bytes, patches a longword at offset 13068 of a0, then restores registers from the stack and returns.

### noprefetch

- `ShutDown`: Saves the Exec library base, calls SuperState, writes zero six times to the custom copper control register to stop the copper, restores the saved state with UserState and returns zero in d0.

### Disk-Validator

- `disk_validator_block_scan`: Scans disk blocks using the a0 buffer and a1/a2 state, calling jump-table routines through a5 to read and compare block data and update counters in the a2 structure.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:DELIGHT.EXE

- `delight_entry_free_and_exit`: Saves all registers, calls a local setup routine and the passed a1 code, frees the memory block pointed to by a1 via exec.FreeMem, clears the stack slot and restores registers before returning.
- `skip_dbra_scale_a1_by_four`: Reads a long from the address just before the current pc into a1, doubles it twice to multiply by four, then adds four after a dbra loop over d6.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:CREDITS.EXE

- `hunk_relocator_and_entry`: Saves all registers, reads hunk headers via a byte reader, checks the hunk magic, relocates hunks by adding base offsets, then jumps to the loaded code.
- `read_big_endian_long_from_a0`: Builds a 32-bit value in d0 by shifting left eight bits and appending four bytes read from (a0)+, advancing a0 by four.
- `decode_huffman_symbol_to_d5`: Reads bits via the bit-reader, walks a Huffman code table in program memory using d1 as index, accumulates the decoded value in d5, then copies d5 bytes from a6 to a3 backwards and returns.
- `read_one_bit_into_d3_carry`: Shifts d3 left by one; if it becomes zero, refills d3 from the byte at -(a6), then rotates the fetched bit into d3 and returns with carry set from that bit.
- `decode_huffman_symbol_to_d6`: Reads up to four bits via the bit-reader to pick a table entry, then reads that many more bits into d6 and adds the table offset, returning the decoded value in d6.
- `decode_compressed_credits_stream`: Reads bits from the stream via the bit-reader routine, building a word in d7 from run-length and offset tables, then walks tagged chunks in a0 and patches offsets relative to a1.
- `resolve_relative_offset_pointer`: Walks back through a linked list of a1-relative pointers, scales the entry by four, decrements d0 to pick an entry, and returns the resulting address in d2.

### icon.library

- `icon_library_return_minus_one`: Library entry code that loads minus one into d0 and returns immediately, giving the caller a failure result.
- `increment_counter_clear_flag_bit3`: Adds one to the 16-bit counter at offset 32 of the library base in a6, clears bit 3 of the byte at offset 14, and returns the base pointer in d0.
- `decrement_counter_and_maybe_call_cleanup`: Returns zero in d0, decrements the 16-bit counter at offset 32 of a6, and if it reached zero with bit 3 of byte 14 set, calls the cleanup routine at offset 256.
- `unlink_node_and_free_via_library`: If the word at offset 32 of a6 is zero, unlinks the node from its list, calls the library vector at offset minus 210 with size in d0, and returns the saved pointer.
- `set_icon_bit3_and_return_zero`: Sets bit 3 of the byte at offset 14 from a6, then returns zero in d0 to the caller.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_and_store_handle`: Saves a0 and a6 into the structure pointed to by d0, opens dos.library via exec, stores the result, and alerts on failure.
- `copy_icon_globals_and_return_a2`: Copies the two longwords at offsets 34 and 38 from a2 into absolute globals, returns a2 in d0 and pops a2.
- `zero_word_data_slot_one`: This single zero word decodes as an ori.b immediate instruction but is a zero-filled data slot referenced by address, not executable code.
- `zero_word_data_slot_two`: This one zero word is another zero-filled data area entry that code points at, decoding as an ori.b immediate rather than real instructions.
- `push_a0_a6_call_vector_and_return`: Pushes a0 then a6 onto the stack, calls the routine at address zero, adds eight to the stack pointer to discard both saved registers, and returns.
- `icon_lib_call_a8_save_a0_a1_a6`: Saves a0, a1 and a6 on the stack, calls the library vector at offset a8, then discards the saved registers and returns.
- `icon_lib_call_124_save_a0_a2_a6`: Saves a0, a1, a2 and a6 on the stack, calls the library vector at offset 124, then discards the saved registers and returns.
- `icon_lib_call_3fe_save_a0_a1_a6`: Saves a0, a1 and a6 on the stack, calls the library vector at offset 3fe, then discards the saved registers and returns.
- `icon_lib_push_a0_a6_call_656`: Pushes a0 and a6 onto the stack, calls the library routine at offset 656, then discards both saved registers and returns.
- `icon_lib_push_a0_a6_call_692`: Pushes a0 and a6 onto the stack, calls the library routine at offset 692, then discards both saved registers and returns.
- `icon_lib_push_a6_call_6a8`: Pushes a6 onto the stack, calls the library routine at offset 6a8, then discards the saved register and returns.
- `icon_library_save_a0_a2_and_call`: Pushes registers a0 through a2 and a6 onto the stack, calls a library routine, then discards the saved registers and returns.
- `icon_library_save_a0_and_call`: Pushes a0 and a6 onto the stack, calls a library routine, adds eight to the stack pointer to drop them, and returns.
- `icon_library_save_a0_and_call_alt`: Pushes a0 and a6 onto the stack, calls a different library routine, adds eight to the stack pointer to drop them, and returns.
- `icon_library_call_818_preserve_a0_a1`: Pushes a0 and a1 onto the stack, calls the library routine at offset 818, then discards the saved registers and returns to the caller.
- `icon_library_call_864_preserve_a0_a1`: Saves a0 and a1 on the stack, invokes the library routine at offset 864, removes the saved registers from the stack and returns.
- `icon_library_call_8d6_preserve_a0_a1`: Stores a0 and a1 on the stack, jumps into the library routine at offset 8d6, then restores the stack pointer and returns to its caller.
- `set_icon_library_userdata_field`: Saves a6, loads the library base from the stack argument, calls the library's own vector at offset -294 to get a structure, then stores the second argument at offset 148 of that structure and returns.
- `zero_filled_data_area`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `allocate_and_initialize_icon_structure`: Allocates a structure via a library call, fills it from a local buffer and the second argument, links it into a list, and returns the structure pointer or zero on failure.
- `copy_icon_fields_to_new_object`: Copies eleven longs from the library base at offset minus seventy-six into the new object at offset ninety-six, then copies individual byte and long fields and links the object back through its owner pointer.
- `return_zero_restore_regs_and_frame`: Sets d0 to zero as the return value, restores d2 through d4 and a2 from the stack, releases the a6 frame and returns to the caller.
- `build_icon_message_and_dispatch`: Builds a message on the stack from the icon object's fields, copies eleven longs and several fields, then calls the dispatch routine with the two arguments and the message.
- `icon_load_info_header`: Builds a temporary path buffer, opens the icon file, reads its header, and returns zero or branches to the shared failure path when the read fails.
- `icon_read_immediate_tooltypes`: Reads the icon header, clears two longwords in the caller's structure, then loads tooltype and image data through the library's read callbacks, returning zero on any failure.
- `icon_read_deferred_tooltypes`: Reads two longword values into the caller's structure, then conditionally loads tooltype and image blocks through the read callbacks, returning zero on failure.
- `load_four_icon_entries_and_optional_extra`: Calls the a3 callback and a library routine to allocate and store four icon entries in the a4 array, then optionally allocates one more entry at offset 70 of a2, returning success in d4.
- `set_failure_flag_and_branch`: Sets d4 to zero to signal failure and branches to the shared exit path that restores registers and returns.
- `return_d4_restore_registers`: Moves d4 into d0 as the return value, restores d2-d5 and a2-a5 from the stack frame, releases the a6 frame and returns to the caller.
- `icon_open_alloc_and_dispatch`: Library routine that saves registers, allocates a 260-byte buffer via a library call, and on failure reports error 103 with the first argument before returning zero.
- `icon_write_info_chunk_fields`: Library routine that writes the .info chunk header and serializes the icon's flags, gadget, and optional string and array fields through the a4 callback, returning zero on failure.
- `icon_write_icon_strings_and_arrays`: Library routine that writes the icon's two image pointers, then serializes optional string and array fields with their lengths through the a4 callback, returning zero on failure.
- `scan_icon_list_and_verify`: Advances a3 by four bytes and increments the counter at -4(a6) until a zero longword is found, then calls the function in a4 and returns its result in d4.
- `walk_icon_entries_and_draw`: Iterates over longwords at a3, calling a length routine and the a4 callback for each, then handles the entry at 70(a2) and restores d2-d6/a2-a4 before returning d4 in d0.
- `push_a2_load_string_pointer`: Pushes a2 onto the stack, loads the longword at 12(a7) into a0, sets a2 to two bytes past a0 and branches to another routine.
- `icon_library_find_next_node`: Walks a linked list starting at a2, calling two library routines per node, comparing each node's next pointer at offset 8, and freeing the found node before returning.
- `icon_library_call_with_offset_pointer`: Loads the first stack argument into d0 and the second into a0, then calls a library routine with a pointer to 140 bytes past a0 and d0 pushed.
- `icon_library_allocate_and_init_node`: Allocates 65536 bytes, initializes fields at offsets 140 and 142, calls a routine with the argument, and frees the block if initialization fails.
- `open_icon_by_name_and_return_handle`: Pushes a2, a name pointer and d2 onto the stack, calls the library open routine, cleans the stack, and returns a2 in d0 if the call succeeded, otherwise falls through to the shared exit.
- `return_zero_and_restore_registers`: Sets d0 to zero as the return value, restores d2, d3 and a2 from the stack and returns to the caller.
- `decrement_count_and_store_two_values`: Decrements the word count at a2, calls the library routine when it reaches zero, then stores the two arguments into the entry array at the count index and returns one.
- `icon_remove_last_entry_store_fields`: Decrements the word count at a2, indexes the table at offset 10(a2) by that count times eight, stores d2 and the stack argument into the entry's two longword fields, returns 1 in d0.
- `icon_allocate_and_init_new_icon`: Allocates a 94-byte block, initializes its header, links it to the given icon and data pointers, calls setup and validation routines, and returns the block pointer or zero on failure.
- `branch_to_cleanup_return_zero`: Unconditionally branches to the shared cleanup path that returns zero, serving as the failure exit for the icon allocation routine.
- `icon_library_call_and_return_zero`: Pushes the immediate value 94 and d4 onto the stack, calls a library routine through a fixed vector, clears d4, discards the pushed arguments, returns d4 as zero in d0 and restores d2-d4/a2.
- `icon_library_dispatch_offset_4e`: Saves d2, loads two arguments from the stack, adds 78 to the second, pushes both and jumps to a library vector, then restores d2 and returns.
- `icon_library_save_regs_call_and_branch`: Saves d2-d4/a2-a3, loads a pointer and a value from the stack, pushes the value, calls a library routine, stores the result in d3 and the pointer in d4, then branches based on the result.
- `scan_string_for_equals_after_jsr`: Saves d3, d2 and a3 on the stack, calls a subroutine, and if it returns nonzero adds d3 to a3 and scans forward for a byte equal to 61, returning the following address in d0.
- `fetch_next_pointer_or_zero`: Loads the longword at a2 into a3 and d0, advancing a2 by four; if the value is zero it clears d0, then restores d2-d4 and a2-a3 from the stack and returns.
- `call_with_two_stack_args`: Saves d2-d5, loads two longword arguments from the caller's stack into d2 and d3, pushes d3 and calls a subroutine, storing its result in d5 before continuing.
- `allocate_icon_memory_and_compute_size`: Pushes a small size and d2 onto the stack, calls an allocation routine, stores the returned pointer in d4, cleans the stack, and returns the difference between the new pointer and d2 in d0.
- `find_matching_icon_entry`: Calls a lookup routine with d2, compares the result to d5, then calls another routine with d5, d3 and d2 to verify a match, returning 1 on success or 0 otherwise.
- `advance_icon_pointer_past_entry`: Tests d4 and, if nonzero, copies it to d2 and increments d2 by one before branching onward; if d4 is zero it branches without changing d2.
- `return_zero_after_d2_test`: Sets d2 to zero, tests it (always zero so the branch is not taken), sets d0 to zero, restores d2-d5 from the stack and returns.
- `parse_icon_copy_specification`: Library code that clears a byte at offset 30 of the first argument, then calls parsing helpers on the second argument to read a copy specification and fill the first structure.
- `parse_copy_count_after_keyword`: Advances a4 past five bytes, calls a helper with length 28 and the advanced pointer, and on success sets d3 to one and advances a4 by three more bytes.
- `init_digit_accumulator_zero`: Loads zero into d3 and branches to the shared digit-parsing continuation, setting up an empty accumulator before the loop.
- `parse_decimal_number_from_string`: Reads characters through a4, accumulates digits into d3 using d2, calls the digit converter, then formats and stores the parsed value via library calls before returning a2.
- `convert_ascii_digit_to_value`: Loads the argument from the stack into d1, checks it lies between character zero and nine, and returns its numeric value in d0, or branches away otherwise.
- `return_minus_one_constant`: Loads the immediate value minus one into d0 and returns, providing a constant failure result to the library caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `allocate_and_register_icon_object`: Saves d2-d5, allocates an object via a library call, then registers it with two further calls, returning the object pointer or zero on failure.
- `create_icon_with_size_ten`: Allocates a small structure, stores the value ten in its word fields and in the caller's output, links it through a library call and returns success.
- `return_zero_and_restore_d2_a2`: Sets d0 to zero as the return value, then pops d2 and a2 from the stack and returns to the caller.
- `compare_three_args_via_library_call`: Loads two stack arguments into d0 and d1, pushes them plus a third argument, calls the library vector at offset 48, then compares the result with a saved argument and returns zero or the result.
- `copy_d1_to_d0_and_return`: Moves the value in d1 into d0 as the return value and returns immediately to the caller.
- `compare_icon_result_with_stack_arg`: Pushes three stack arguments, calls the routine at offset 76, compares the returned d0 with the value at stack offset 28, cleans up twelve bytes and returns zero on mismatch or the result on match.
- `return_d1_as_d0`: Copies the longword in d1 into d0 and returns immediately, serving as a small result-passing stub in the icon library.
- `unsigned_32bit_divide_d1_by_d2`: Divides the longword in d1 by d2, using word divisions when d2 fits in sixteen bits and a shift-subtract loop otherwise, returning quotient in d0 and remainder in d1.
- `unsigned_divide_32_by_16_bit`: Divides the 32-bit value in d1 by the 16-bit divisor in d2 using shift-and-subtract over 16 bits, leaving the quotient in d1 and remainder in d0.
- `multiply_32_by_32_into_64`: Multiplies the 32-bit values in d0 and d1 by combining partial 16-bit products, returning the 64-bit result in d0 with d2 preserved.
- `format_string_via_rawdofmt`: Loads the format string and argument pointer from the stack, calls exec RawDoFmt through the library base in a6, then restores registers and returns.
- `append_byte_to_a3_pointer`: Stores the low byte of d0 at the address in a3, then advances a3 by one byte and returns to the caller.
- `strlen_up_to_255_bytes`: Reads bytes from the string pointer passed on the stack, counting until a zero byte or 255 bytes, and returns the length in d0.
- `find_first_matching_byte`: Scans the string at the first stack argument for the byte in d0, returning a pointer to the match or zero when the string ends.
- `return_zero_and_branch_to_shared_tail`: Loads immediate zero into d0 and branches unconditionally to a shared tail elsewhere in the library, leaving other registers untouched.
- `copy_string_until_nul_terminator`: Loads a0 and a1 from the caller's stack arguments, then copies bytes from a1 to a0 one at a time until a zero byte is copied, returning after the terminator.
- `scan_string_for_nul_then_branch`: Loads a0 and a1 from the stack, advances a0 past each nonzero byte, steps back one on the zero byte, then branches to a shared tail.
- `compare_strings_counted`: Loads two string pointers and a length from the stack, decrements the length, then compares bytes from the two strings until a mismatch or a zero byte, returning zero in d0 on a full match.
- `return_compare_result_sign`: Sets d0 to one when the previous comparison was greater, otherwise sets d0 to minus one, then returns to the caller.
- `skip_to_string_end_or_count`: Loads a string pointer and a count from the stack, scans bytes until a zero terminator or the count is exhausted, then branches to the shared comparison exit.
- `decrement_a0_and_call_copy`: Subtracts one from the longword in a0, then calls the byte-copy routine at the following address before returning to its caller.
- `copy_bytes_with_zero_fill`: Copies bytes from a1 to a0 while decrementing d0, stopping on a zero byte or when d0 runs out, then pads the rest with zero bytes.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_self_referential_list_header`: Loads the first stack argument into a0, stores a0 at its own address, adds four to that value, clears the next longword and stores a0 in the third.
- `icon_draw_frame_with_border`: Saves d2-d3/a2, reads three arguments from the stack, calls the library helper to draw a rectangle, then computes scaled coordinates and draws the inner content before restoring registers.
- `icon_setup_args_and_jump`: Saves d2-d3/a2 on the stack, loads the three stack arguments into d2, d3 and a2, then branches into the shared drawing code.
- `icon_draw_list_until_null`: Loops over a linked list in a2, calling the library helper to draw each node's rectangle and scaled data, advancing via the next pointer until null, returning success or failure.
- `return_zero_restore_d2_d3_a2`: Sets d0 to zero as the return value, then pops d2, d3 and a2 from the stack and returns to the caller.
- `build_icon_from_two_bitmaps`: Allocates a 20-byte icon structure, creates two bitmaps from the passed dimensions and bitmaps, stores the second bitmap pointer at offset 10, and returns the icon or minus one on failure.
- `return_minus_one_restore_d2_d5_a2`: Sets d0 to minus one as the failure return value, then pops d2 through d5 and a2 from the stack and returns to the caller.
- `save_regs_load_three_args_and_clear_d6`: Pushes d2 through d6 and a2 onto the stack, loads the three longword arguments from offsets 28, 32 and 36 of the stack into d2, d3 and d4, clears d6, then branches onward.
- `push_icon_args_and_call_allocator`: Pushes the constant 65536, a word from address 16, d3 and d2 onto the stack, calls a library routine, stores its result at offset 56 of the stack, discards the arguments and branches based on the result.
- `allocate_and_chain_icon_nodes`: Stores an argument in a2, calls library routines to allocate and fill icon nodes, links each new node through offset 12, and returns d6 or minus one on failure.
- `icon_library_failure_return`: Library code that sets d0 to minus one as a failure result, restores d2 through d6 and a2 from the stack, and returns to its caller.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:populous.ii

- `hunk_executable_relocator`: Reads AmigaDOS hunk records through a byte reader, checks the executable magic, then relocates and copies hunks using a0, a2, a3, a4, a6 and d0-d7 before returning a length or zero.
- `read_big_endian_long`: Reads four bytes from the address in a0 into d0 in big-endian order, advancing a0 by four, and returns the assembled long value.
- `decode_bitstream_into_a3_buffer`: Reads a variable-length bitstream via the bit-fetch routine, decodes a run of bits into d5, then copies that many bytes backwards from a6 to a3 and returns.
- `fetch_next_bit_from_a6_stream`: Shifts d3 left by one, refilling the low bit from the byte at a6 when it empties, and returns the fetched bit in the carry flag.
- `decode_short_bitfield_into_d6`: Fetches up to four bits via the bit-fetch routine, then reads a length byte from the table at pc and accumulates that many more bits into d6 before returning.
- `read_bit_encoded_value`: Reads bits from a hardware or memory source via a helper, assembling them into d7 using rotate-through-extend, with two paths depending on d6, and returns the value in d7.
- `compute_relative_address_and_jump`: Walks a linked list from a1 to find an entry, computes an offset from a0 and a1, then pushes crafted instruction words onto the stack and jumps to them.

### run

- `allocate_and_initialize_memory_block`: Calls library routines through a5 to find the largest free block, allocates a block, fills it with incrementing values, sets up a stack frame and copies sixteen longs, returning zero or minus one.
- `copy_d1_to_d0_then_branch`: Moves the longword in d1 into d0 and branches to another part of the same routine, leaving the stack untouched.

### ask

- `allocate_and_init_ask_structure`: Calls library routines through a5 to find a free slot, allocate memory, fill a structure with tag values, copy sixteen longwords, install a callback and return zero or minus one.
- `copy_d1_to_d0_then_branch_2`: Moves the value in d1 into d0 and then branches to another routine, transferring control with d0 holding the copied value.

### assign

- `assign_command_main`: Program entry that parses the assign command line, opens dos.library, validates ADD/SUB/PATH/DEFER options and device names, then performs the volume assignment and prints errors.
- `format_volume_entry_text`: Takes a volume entry in d0 and output buffer in a1, writes the entry's name or a Volume: prefix plus the volume name via the text helper.
- `append_bytes_to_buffer_chain`: Copies d0 bytes from the buffer at a0 into the current buffer node, allocating a new node via the library vector at offset -198 when full, then updates the node's used count and returns 0, or 1 if the node is marked full.
- `flush_assign_list_buffers`: Walks a linked list of nodes, and for each node with a positive count calls the write and flush library vectors in chunks of up to 100 bytes, then clears the count and returns 0, 103 or 304.
- `parse_assign_arguments_into_list`: Opens the assign library, splits each argument at a colon, looks up the device, rejects duplicates and overflow, stores device pointers in the caller's array and returns an error string or null.
- `split_device_name_at_colon`: Scans the source string for a colon, copies the part before it into the destination buffer when shorter than 30 characters, terminates it and returns null or an error string.
- `case_insensitive_string_compare`: Reads bytes from a0 and a1, uppercases lowercase ASCII letters, and returns the difference of the first unequal pair in d0, stopping at a null byte.

### ENDCLI

- `endcli_command_line_parser`: Reads the command line, finds the largest argument number, allocates and fills an argument table, copies sixteen longs into the saved area, and returns zero or minus one.
- `copy_d1_to_d0_and_branch`: Moves d1 into d0 and branches to another routine, passing the copied value as the return value.

### newcli

- `allocate_cli_structure_and_copy_table`: Calls library vectors through a5 to find the highest table index, allocates a CLI structure, fills its command table with incrementing entries, copies 16 longs into and out of offset 536 of the structure, and returns zero or minus one.
- `return_d1_in_d0_then_jump`: Moves d1 into d0 as the return value and branches to the code at offset 260, which is the shared exit path of the caller.

### echo

- `build_and_install_echo_command`: Finds the highest command number, allocates a command entry, fills it with a marker and argument words, saves and restores a sixteen-longword block, installs a handler pointer and returns 0 or -1 on failure.
- `copy_d1_to_d0_and_branch_2`: Moves the longword in d1 into d0 and branches to another routine, so the caller receives d1 as its return value.

### Execute

- `execute_command_file_main`: Program entry that opens the command file, allocates and initialises a command structure, parses directives and runs the command interpreter using dos.library and utility.library calls.
- `parse_command_directive`: Reads characters from the command file, skips whitespace, matches a directive name and dispatches on its index to set command fields or report an invalid directive.
- `read_line_into_buffer_with_echo`: Reads characters from the console file handle into the 80-byte buffer at offset 120 of the context, echoing each, handling backspace and newline, then looks up the entered name in the table at offset 400 and returns.
- `read_line_skip_if_buffer_empty`: Reads a line into the buffer at offset 120 of the context via the console handle, discarding it if the line is empty or terminated by newline or end of file, and returns the first buffered byte.
- `validate_nonempty_buffer_or_report`: Checks whether the buffer at offset 120 of the context holds more than one byte; if not, prints the invalid directive argument message, then returns the first buffered byte.
- `call_execute_vectors_with_frame_args`: Builds an a5 stack frame, loads the library base from the first argument, then calls the library vectors at offsets minus sixty and minus three hundred forty-eight, passing the second argument and a pointer to the frame's extra arguments.
- `run_execute_command_with_io_redirection`: Opens the library, optionally redirects input and output streams, calls the execute helper with the command arguments, then closes the streams and frees the command buffer before returning.
- `close_execute_library_and_free_segment`: Loads the library base from the argument, calls the close-library vector, then calls the free-segment vector with the returned base and a zero length, restoring saved registers.
- `save_registers_to_caller_frame`: Loads the return address from the stack into a0, stores d1-d7 and a1-a7 plus the caller's return address into the structure pointed to by a0, sets d0 to zero and returns.
- `restore_registers_from_frame`: Reads a value from the stack into d0, replaces zero with one, loads the frame pointer from the stack into a0, restores d1-d7 and a1-a7 from it and copies the saved return address back onto the stack.
- `signed_32bit_divide_with_sign_fixup`: Takes d0 and d1, negates negative operands, calls the unsigned division helper, then negates the quotient and remainder according to the original signs before returning.
- `unsigned_32bit_divide_d0_by_d1`: Divides the 32-bit value in d0 by d1 using shift-and-subtract, returning quotient in d0 and remainder in d1, preserving d2 and d3 on the stack.

### copy

- `copy_command_main_entry`: Program entry for the copy command: parses arguments, opens dos.library, allocates buffers and file handles, then copies source files to the destination, reporting errors.
- `set_and_restore_dos_signal_mask`: Saves the current dos signal mask via the library call, applies the mask in d0 if nonzero, then restores the saved mask and returns the result in d0.
- `repeat_library_call_d0_times`: Saves d6, d7, a3 and a6, then calls the library function at offset -948 of the a6 base d0 times, passing the address of a nearby data word in d1 each time, and restores the registers.
- `check_d0_in_ascii_range_bitmask`: Returns 1 in d0 if the input value lies between 201 and 231 and the corresponding bit of the mask is set, otherwise returns 0, preserving d7.
- `case_insensitive_string_compare_2`: Compares two null-terminated strings byte by byte, converting lowercase letters to uppercase, and returns the difference of the first differing characters in d0.

### Makedir

- `make_directory_from_args`: Parses the NAME/M argument, creates a directory via dos.library, reports errors, and returns 20 or 0 in d0.

### cd

- `allocate_and_init_cd_structure`: Finds the highest node value, allocates a block, fills it with incrementing longwords, copies 16 longs, and returns 0 or -1 in d0.
- `copy_d1_to_d0_and_branch_3`: Moves d1 into d0 and branches to another routine, passing the value as the return argument.

### if

- `allocate_and_init_icon_structure`: Calls library routines to find the largest icon number, allocates a block, fills it with incrementing icon identifiers, copies sixteen longs into the icon area and returns zero or minus one.
- `return_d1_in_d0_then_jump_2`: Copies d1 into d0 and branches into the middle of another routine, so the value in d1 becomes the return value.

### iconx

- `iconx_startup_parse_and_run`: Opens dos.library, allocates a task structure, parses the command line into arguments handling quotes and escapes, opens the program file and jumps to the loaded code.
- `close_iconx_task_and_free_resources`: Finds the current task, switches to its stack, closes the window and library, frees the message port and memory, then replies the startup message and returns its value.
- `run_iconx_tool_from_workbench_args`: Parses Workbench arguments, opens the tool window, reads tool types, builds a command line, runs the tool with a delay and closes the window.
- `execute_tool_with_output_redirection`: Builds a command line from the tool name and arguments, opens the tool's output file, runs it with optional wait and delay, and returns success.
- `free_icon_list_and_children`: Walks a linked list of icon nodes, frees each node's name and next pointer via library calls, recursively frees child nodes, then frees the list head and returns the first freed pointer.
- `build_icon_tooltype_string`: Iterates an icon tool-type list, appends each entry's name separated by a slash into the caller's buffer, converts the first slash to a colon, and returns the buffer length.
- `append_three_strings_with_separators`: Takes three string pointers, appends the first to the third, the second to the first, and the third plus the second's length to the second, using string append calls.
- `strlen_count_chars`: Loads the string pointer from the stack into a0, counts bytes until a null terminator, and returns the count in d0.
- `strcpy_copy_string`: Loads destination and source pointers from the stack, copies bytes from source to destination including the null terminator, advancing both pointers.
- `strcat_append_string`: Loads a string pointer and a second pointer from the stack, scans to the first string's null terminator, then calls the copy routine to append the second string.
- `parse_signed_decimal_string`: Reads a string pointer from the stack, skips an optional plus or minus sign, accumulates decimal digits into d0, negates if minus, and returns the value in d0.
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
- `call_function_with_extra_arg`: Saves d2 and d3, loads two stack arguments, pushes a pointer to the third stack argument plus both values, calls a function, then cleans the stack and returns.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `format_string_via_rawdofmt_2`: Calls exec RawDoFmt with a format string and arguments into a 140-byte stack buffer, measures the resulting string length, and passes it to another routine.
- `store_byte_d0_to_a3_postinc`: Writes the low byte of d0 to the address in a3, then increments a3 by one and returns to the caller.
- `format_output_via_rawdofmt`: Saves a2-a4 and a6, loads format arguments from the stack, calls exec RawDoFmt through the library base at absolute address 4, then restores registers and returns.
- `store_byte_d0_to_a3_postinc_alt`: Writes the low byte of d0 to the address in a3, advances a3 by one byte and returns; identical code reached from a different caller.

### LOADWB

- `parse_tool_args_and_start_workbench`: Opens dos.library, allocates a message port and parses the command line into argument pointers, then loads and starts the Workbench tool, returning a status code in d0.
- `reply_and_free_tool_message`: Finds the current task, restores its stack, closes the tool's file and library, frees its message port and replies the message, returning the original argument in d0.
- `launch_workbench_tool_with_args`: Opens dos.library, allocates and fills a message port, builds the tool argument list, starts the Workbench tool and waits for its reply, returning after cleanup.
- `compare_two_strings_lexicographically`: Compares the null-terminated strings pointed to by the two stack arguments byte by byte, returning zero if equal, minus one if the first is lower, or one if higher.
- `build_linked_list_of_allocated_nodes`: Walks a linked list of offsets, allocates eight-byte nodes with a memory pool call, stores each node's data pointer from a second call, links nodes via shifted addresses and returns the first node.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `check_workbench_version_string`: Compares the byte at offset twenty of the structure in a2 against the character two, then compares against a version string, copying a suffix into the structure when matched.
- `write_dot_and_spaces_to_a2_buffer`: Writes a period byte then four space bytes into offsets 17 through 21 of the structure pointed to by a2, then branches to another routine.
- `compare_a2_string_with_pc_table`: Copies a2 into a0, points a1 at a nearby byte table, tests the first byte and compares bytes from a0 and a1, branching on mismatch or match.
- `restore_a0_a1_and_jump_via_pc`: Loads a2 with a nearby address, pops a0 and a1 from the stack, pushes the value at the program counter and returns through it.
- `call_reschedule_2`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_2`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_three_arg_function_via_stack`: Saves d2-d3, loads two arguments from the stack, pushes a pointer to the third stack argument plus both values, calls an absolute address, then cleans the stack and restores d2-d3.
- `format_string_with_rawdofmt_then_dispatch`: Calls exec RawDoFmt into a 140-byte stack buffer, measures the resulting string length, and if non-empty passes the buffer and length to another routine before restoring registers.
- `store_d0_byte_and_return`: Writes the low byte of d0 to the memory pointed to by a3, increments a3, and returns to the caller.

### wait

- `wait_until_time_or_interrupt`: Parses an HH:MM argument, reads the current time from dos.library, then loops calling delay and checking for a key press until the target time is reached or a key is hit.

### DISK-VALIDATOR

- `disk_validator_data_table`: The bytes are data decoded as code, not executable instructions; they form a table used by the disk validator and contain no meaningful 68000 program flow.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 2 of 2).adf:Credits.exe

- `_main`: Saves the current DMACON and INTENA values, clears the BSS area, then calls the setup, screen swap, credits and system reset routines before restoring registers and returning.
- `_copper_list`: Two zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `_reset_system`: Saves all registers, disables all interrupts and DMA, calls the keyboard, VBI and copper reset routines, then re-enables interrupts and DMA using the saved old_ena values.
- `old_ena`: A single zero word that decodes as ori.b #0,d0; it is a data word holding the saved interrupt and DMA enable values, not executable code.
- `_reset_copper`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `_reset_vbi`: Exchanges the current level 3 interrupt vector with the saved one and the current trap vector with its saved copy, then returns.
- `_setup_vbi`: Disables interrupts via INTENA, saves the old interrupt and trap vectors, installs new handlers for both, re-enables interrupts with INTENA, and returns.
- `_do_ints`: The single instruction sets the low bit of d1; the bytes are data, not executable code.
- `pcol`: Decrements a frame delay, and when it expires writes two shifted colour values to the colour registers of two player sprites, then advances or reverses the colour index within its bounds.
- `clamp_colour_index_and_set_interrupt_flag`: Subtracts one from d0, and if the result is not positive sets the interrupt enable flag and stores the new index in memory before returning.
- `_vbi_interupt`: Saves all registers, checks VBI and CIA interrupt flags, reads JOY1DAT to derive dx/dy from lookup tables, updates fire counters from the mouse button bits, then calls the mouse and timer routines.
- `vbi_potgo_reset_and_button_poll`: Clears the fire counter, writes zero to POTGO, reads POTGOR into d0, tests bit 14 and increments the fire counter if the button is not pressed, then branches on.
- `vbi_clear_fire_and_ack_potgo`: Clears the fire2 flag and writes zero to the POTGO register, clamps fire to 255, calls the mouse VBI, sets the VBI done flag, increments the VBI timer, and if interrupts are disabled restores registers and returns through the old level 1 vector.
- `ack_vbi_interrupt_and_restore`: Writes 32 to the INTREQ register to acknowledge the vertical blank interrupt, then pops d0-d7 and a0-a6 from the stack before falling through to the next routine.
- `_div_by_zero`: Empty exception handler that returns immediately with rte, installed as the divide-by-zero vector but performing no work.
- `_setup_screens`: Calls the mouse setup, stores the screen memory pointer as the display screen, builds two copper lists from the default palette into copper memory, sets the work screen pointer 32000 bytes on, and restores registers.
- `_copper_pallet`: Copies sixteen words from the address in a2 into the color entries at offset two of a0 and a1, advancing both by four bytes per iteration.
- `_make_copper`: Writes a copper display list into memory at a0 with bitplane, palette, sprite and pointer entries, stores its start in a3 and a6, then enables copper DMA and records the list address in the showing variable.
- `_setup_keyboard`: Disables keyboard interrupts, saves the old level 2 interrupt vector, points it at the keyboard interrupt routine, then re-enables keyboard interrupts.
- `_reset_keyboard`: Disables keyboard interrupts, swaps the current level 2 interrupt vector back with the saved old one, then re-enables keyboard interrupts.
- `keyboard_interupt`: Reads the CIAA keyboard serial register into the key variables, and on the toggle key flips the screen-system bit, loading either the showing pointer or the graphics base offset into the display pointer and setting the flag.
- `keyboard_interrupt_clear_screen_bit`: Clears the screen-system bit, zeroes the display flag, writes the showing pointer to the copper list high address, and records the key press in the key table before chaining to the old keyboard interrupt.
- `its_not_the_system`: On the toggle key sets the screen-system bit, loads the graphics base offset into the display pointer and sets the flag, then stores the key code into the key table and returns.
- `clear_audio_irq_and_return_from_exception`: Clears a byte at offset one plus d0 in the a0 buffer, acknowledges the level 3 interrupt by writing 8 to INTREQ, restores d0 and a0 from the stack and returns from the exception.
- `_mydosbase`: Two ori.b instructions with immediate zero and one that are placeholder data bytes, not meaningful executable code, and execution falls through to the following routine.
- `_setup_libs`: Opens dos.library and graphics.library via OldOpenLibrary, stores both bases, reads the graphics palette flag into _pal, finds the current task and sets its stack limit field to minus one.
- `_swap_screens`: Swaps the display and work screen pointers, flips the toggle bits, loads the selected copper list pointer into the showing variable and writes it to the copper list high register, then clears the vbi done flag.
- `_toggle`: This single zero word decodes as an ori.b immediate instruction but is really a zero-filled data location referenced by address from the screen swap code.
- `_showing`: This single zero word decodes as an ori.b immediate instruction but is really a zero-filled data location whose address is taken by the copper and screen swap code.
- `_scr_system`: This single zero word decodes as ori.b #0,d0; the address is referenced by interrupt and keyboard handlers, so it is a zero-filled data area rather than executable code.
- `_wait_vbi`: Tests the word at absolute address a fixed address and returns immediately if it is zero, otherwise spins until that word becomes zero.
- `_vbi_mouse`: Reads JOY0DAT mouse counters, sign-extends the deltas, clamps the coarse and fine positions, derives half-scale mouse coordinates, latches left and right button presses from CIA, and moves the pointer.
- `poll_mouse_buttons_and_sync_x`: Clears the left button flag, writes the POTGO register, reads POTGOR to test the right button bit, sets the right button and copies the mouse x position, then calls the pointer mover.
- `clear_right_button_and_move_pointer`: Sets the right button flag to zero, pushes a zero word argument, calls the pointer mover and cleans the stack before returning.
- `_setup_mouse`: Reads JOY0DAT twice, storing the low byte as the old mouse x and the shifted high byte as the old mouse y, preserving d0 and d1.
- `_move_pointer`: Saves d0-d3/a0, reads three word arguments from the stack, computes sprite x/y bytes and flag bits, and writes them into the pointer data block and its shadow copy before restoring registers.
- `_old_mousey`: This single zero word decodes as ori.b #0,d0; it is a zero-filled data area referenced by mouse setup and VBI code, not executable code.
- `_big_mousey`: Loads the DosBase pointer, calls the function at offset minus thirty in its jump table with the two longword arguments from the stack, then restores d1-d2/a0 and returns.
- `_Close`: Saves d1 and a0, loads the file handle from the stack into d1, calls the DosBase close vector at offset minus thirty-six, then restores registers and returns.
- `_Read`: Saves d1-d3 and a0, sets d0 to zero, and if the three stack arguments are all non-null calls the DosBase read vector at offset minus forty-two before restoring registers.
- `_Write`: Saves d1-d3 and a0, sets d0 to zero, and if the three stack arguments are all non-null calls the DosBase write vector at offset minus forty-eight before restoring registers.
- `_Seek`: Saves d1-d3 and a0, sets d0 to zero, and if the first stack argument is nonzero calls the DOS seek vector at offset -66 in DosBase with the three stack arguments.
- `_credits`: Saves all registers, calls the intro setup, intro, credits setup and credits routines in order, then restores registers and returns.
- `_do_intro`: Saves registers, waits for vertical blank, scrolls the small frog, clears the vbi done word, and repeats until a mouse button is pressed.
- `_set_up_intro`: Saves all registers, waits for vertical blank, copies the frog image to the screen, clears plane three at offset 8000, then draws the big and small frogs.
- `_scroll_small_frog`: Shifts 88 rows of 40 bytes starting at screen offset 10880 right by one bit, carrying the top bit between consecutive longwords and wrapping the last bit back into the first.
- `_draw_small_frog`: Reads frog logo bytes, combines pairs through the half-shrink table into nibbles, and writes each byte to five plane rows at screen offset 10880 for 88 rows.
- `_draw_big_frog`: Copies 176 rows of seven words from the frog logo data into the screen buffer at offset 16492 from a1, advancing the destination by 40 bytes per row.
- `_convert_amiga_to_atari`: Reads 4000 words from each of four bitplanes of the work screen and writes them interleaved as four-word groups into the destination screen buffer.
- `tweek_widths`: Stores the byte value 16 into twenty selected entries of the font width table pointed to by a0, then returns.
- `_set_up_credits`: Saves all registers, prepares font and string data, waits for a VBI, copies the frog image and logo into the display screen, clears plane three, zeroes twenty rows of the work screen, then prints the credits string list.
- `_do_credits`: Waits for a VBI, scrolls the display, prints and deletes credit lines while advancing the line counter and string list pointer, looping until the left or right mouse button is pressed.
- `_string_list_position`: This single zero word is a zero-filled data location used as the credits string list pointer, not executable code.
- `_scroll_display`: Advances two bitplane pointers by 40 and 8000 bytes, then copies 100 blocks of ten longwords each upward by 80 bytes and clears the trailing ten longwords in both planes.
- `_display_string`: Uses d0 times 40 as an offset into a0, then ORs ten longwords from that string into a1 plus 8000 and a1 plus 16000, advancing a1 by four each time.
- `_delete_string`: Uses d0 times 40 as an offset into a0 and clears ten consecutive longwords starting there, erasing one stored credits string.
- `convert_string`: Clears a global word, then for each string in the string list maps every character through the sconvert table in place, sums font widths, and stores a computed width word after each string.
- `print_string`: Reads a string descriptor from a0, centers it using the 320-pixel width, and blits each character's font rows into the bitmap at a1, optionally half-shrinking and double-spacing, advancing by font widths.
- `draw_logo`: Computes a bitmap offset from d0 and d1, then copies 76 columns of logo data from a0 into a1 at four vertically spaced row offsets, advancing a1 by 20 bytes per column.
- `find_font_size`: Scans 64 glyphs of 20 words in _font, ORs each glyph's bits, finds the highest set bit, and stores the resulting width byte into _font_width.
- `clear_plane3`: Writes zero to 200 rows of 40 bytes starting at a0, clearing the bitplane 3 bitmap buffer using d1-d7 and a2-a4 as zero sources.
- `copy_frog`: Fills 50 rows of 40 bytes at a1 plus 24000 with ones, then copies 50 rows of 40 bytes from a0 to a1, duplicating the frog sprite.

### ICON.LIBRARY

- `library_open_return_minus_one`: Returns the value minus one in d0 as a 32-bit result, used as the library open entry point that reports failure.
- `increment_open_count_clear_flag`: Adds one to the 16-bit open counter at offset 32 of the library base in a6, clears bit 3 of the byte at offset 14, and returns the base pointer in d0.
- `decrement_open_count_maybe_close`: Sets d0 to zero, decrements the 16-bit open counter at offset 32 of the a6 library base, and if it reaches zero with bit 3 of the byte at offset 14 set, calls the close routine.
- `icon_close_and_free_port`: Checks the word at offset 32 of the library base; if zero, unlinks the port node at offsets 0 and 4, calls the library close vector at offset 34 with d0 set from word 16 and d1 from word 18, and returns the saved pointer.
- `icon_set_busy_flag_return_zero`: Sets bit 3 of the byte at offset 14 of the library base and returns zero in d0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `icon_open_dos_library_or_alert`: Stores a0 and a6 into the library base, opens dos.library via the exec vector at offset 408, saves the result at offset 38, and calls exec Alert with code an address if the open failed.
- `copy_a2_offsets_to_library_globals`: Copies the longwords at offsets 34 and 38 from a2 into the library globals at 402 and 406, returns a2 in d0, then pops a2 and returns.
- `zero_filled_data_word_at_0192`: A single zero word that decodes as ori.b with zero immediate; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_area_2`: Contains only a zero word that decodes as an ori.b instruction, so this range is zero-filled data referenced by address, not executable code.
- `icon_library_push_a0_a6_call_vector`: Pushes a0 then a6 onto the stack, calls the library vector at address zero, discards both saved registers and returns to the caller.
- `icon_library_push_a0_a1_a6_call_vector`: Saves a0 and a1 plus a6 on the stack, calls the library vector at offset a8, then removes the saved registers and returns.
- `icon_library_push_a0_a2_a6_call_vector`: Saves a0, a1 and a2 plus a6 on the stack, calls the library vector at offset 124, cleans the stack and returns to the caller.
- `icon_library_call_3fe_with_a0_a1`: Saves a0, a1 and a6 on the stack, calls the library routine at offset 3fe, then discards the saved registers and returns.
- `icon_library_call_656_with_a0`: Pushes a0 and a6 onto the stack, calls the library routine at offset 656, adds eight to the stack pointer and returns.
- `icon_library_call_692_with_a0`: Pushes a0 and a6 onto the stack, calls the library routine at offset 692, adds eight to the stack pointer and returns.
- `icon_library_call_6a8_with_a6`: Pushes a6 onto the stack, calls the library routine at offset 6a8, discards the pushed longword and returns to the caller.
- `icon_library_call_71c_save_a0_a2`: Saves a0 through a2 and a6 on the stack, calls the library routine at offset 71c, then removes all sixteen bytes and returns.
- `icon_library_call_768_save_a0`: Pushes a0 and a6 onto the stack, calls the library routine at offset 768, discards the eight saved bytes and returns to the caller.
- `icon_library_save_a0_a6_call_helper`: Pushes a0 and a6 onto the stack, calls a library helper, then discards both saved registers and returns to the caller.
- `icon_library_save_a0_a1_call_helper`: Pushes a0 and a1 onto the stack, calls a library helper, then discards both saved registers and returns to the caller.
- `save_regs_call_864_then_restore`: Pushes a0 and a1 onto the stack, calls the routine at address 864, then adds eight to the stack pointer to discard them and returns.
- `icon_call_jsr_8d6_with_a0_a1_saved`: Saves a0 and a1 on the stack, calls the library routine at address 8d6, then discards the saved registers and returns to the caller.
- `icon_set_field_94_via_library_call`: Opens the library base from the stack argument, calls the library vector at offset minus 294 to get a pointer, stores the second argument at offset 148 of that pointer, and returns.
- `icon_zero_filled_data_area`: Contains a single zero word that decodes as ori.b with immediate zero; this is zero-filled data, not executable code.
- `allocate_and_init_icon_object`: Calls the library allocator for the given size, then fills the new icon object from a stack template and links it into its parent list, returning the object or zero.
- `copy_template_into_icon_object`: Copies eleven longs from the stack frame into the icon object at a2, stores the extra fields, and back-links the object into its parent at offset 424.
- `return_zero_restore_regs`: Sets d0 to zero as the return value, restores d2 through d4 and a2 from the stack frame, releases a6 and returns to the caller.
- `build_icon_header_and_dispatch`: Copies eleven longwords from the icon structure at offset 96 into a local header, fills in fields from offsets 61, 72, 76, 84, 88, 92, 156 and 160, then calls the shared dispatch routine with the two arguments and the header.
- `open_icon_and_load_resources`: Builds a path buffer, appends the caller's name and a fixed suffix, opens the file, then reads and validates the icon header and loads its associated resource blocks into the structure.
- `read_icon_header_and_attach_resources`: Calls the read callback for the header, verifies the magic and version, clears two fields, allocates and reads the resource block, then loads the two linked data blocks into the icon structure.
- `load_icon_offsets_and_allocate_buffers`: Calls the library allocator for two icon fields at offsets 18 and 22 of a4, then conditionally allocates buffers at offsets 50, 54 and 70 of a2, returning zero on failure.
- `fill_icon_buffer_array_from_allocator`: Loops while the count at -4(a6) exceeds four, calling the allocator and storing each result into the array at a4, then optionally allocates the buffer at offset 70 of a2.
- `set_d4_zero_and_branch_to_cleanup`: Loads zero into d4 and branches to the shared cleanup path, marking the operation as failed for the caller.
- `return_d4_restore_regs_unlk`: Moves d4 into d0 as the return value, restores d2-d5 and a2-a5 from the a6 frame, releases the frame and returns to the caller.
- `icon_open_parse_and_load`: Opens a resource by tag, parses the icon structure fields, loads sub-images and strings through a callback, then closes the resource and returns a status.
- `icon_draw_from_parsed_struct`: Saves the current image pointer, opens the resource, then draws the icon's main image, optional mask, and bit-flagged sub-images via callbacks, restoring the pointer afterward.
- `icon_draw_optional_text_fields`: Calls library helpers to measure and draw the optional text strings at offsets 22, 26, 50 and 54 of the icon structure, storing results in d4 and returning zero on failure.
- `icon_skip_remaining_string_list`: Advances a3 past the remaining nonzero string pointers in the list, adds four to the local size counter, then calls the draw callback and reloads a3 from offset 54 of the icon structure.
- `icon_draw_string_list_and_finish`: Loops over the string pointer list at a3, measuring and drawing each entry, then draws the optional string at offset 70, restores registers and returns d4 as the result.
- `push_a2_and_enter_list_at_offset_two`: Pushes a2 onto the stack, loads the caller's first argument from the stack into a0, sets a2 to two bytes past it and branches onward into the list walker.
- `walk_list_until_end_or_zero`: Calls library routines on each node, pushing d0 and a2, stops when a call returns zero or a2 reaches the node at offset eight, then frees the last result and restores a2.
- `forward_call_with_offset_eighty_plus`: Loads the first stack argument into d0 and the second into a0, pushes the address eighty plus bytes into a0, pushes d0 and calls the list entry routine.
- `allocate_and_init_icon_node`: Saves d2-d3/a2, allocates a 65536-byte block, initializes fields at offsets 142 and 140 with the caller's argument, and returns the block pointer or zero on failure.
- `retry_icon_node_init_with_extra_arg`: Pushes the a8 word, a2, the node field at offset 140 and d2, calls the init routine, cleans 16 bytes of stack, and returns a2 on success or falls through on failure.
- `return_zero_restore_saved_regs`: Sets d0 to zero as the return value, restores d2-d3/a2 from the stack and returns to the caller.
- `decrement_count_and_store_pair`: Pops d2 and a2, decrements the word count at a2, and if it reaches zero calls the library allocator before storing two long values into the table at offset 16 and 20 indexed by the count.
- `decrement_count_store_two_longs`: Decrements the word count at a2, then stores d2 and the stack argument into the table pointed to by a2 plus 16 and 20, returning 1 in d0.
- `allocate_icon_node_and_register`: Allocates a 94-byte block, initializes its fields, registers it with the icon list, then stores the caller's two arguments into the node and returns the block pointer or zero.
- `branch_to_icon_library_entry`: Branches unconditionally to another location in the ICON.LIBRARY code, transferring execution without touching registers or memory.
- `push_five_byte_arg_call_and_return_zero`: Pushes a small immediate and d4 onto the stack, calls a library routine, clears d4, restores the stack and registers d2-d4/a2, and returns zero in d0.
- `call_with_offset_second_argument`: Saves d2, loads two stack arguments, adds seventy-eight to the second argument, pushes both and calls a library routine, then restores d2 and returns.
- `icon_library_call_with_stack_arg`: Saves d2-d4 and a2-a3, loads a2 and d2 from the caller's stack arguments, pushes d2, calls a library function, stores the result in d3 and a2 in d4, then adjusts the stack and branches based on the result.
- `icon_library_scan_for_equals_byte`: Saves d3, d2 and a3, calls a library function, and if it returns nonzero adds d3 to a3, checks for a byte equal to decimal 61, and returns that pointer in d0 or branches away.
- `icon_library_next_entry_or_restore`: Loads the next pointer from the a2 list into a3 and d0, branches if it is nonzero, otherwise sets d0 to zero, restores d2-d4 and a2-a3, and returns.
- `call_with_second_arg_and_save_result`: Saves registers d2 through d5 on the stack, loads two arguments from the caller's stack into d2 and d3, pushes d3 and calls a library function, then stores the result in d5 and returns.
- `allocate_seventy_six_bytes_and_compare`: Pushes the constant 124 and register d2, calls a library allocation function, stores the result in d4, and if non-null returns the difference between d4 and d2.
- `verify_two_library_calls_match`: Calls a library function with d2, compares the result to d5, then calls another function with d5, d3 and d2, returning 1 if both checks succeed and 0 otherwise.
- `copy_d4_to_d2_plus_one`: Tests d4; if non-zero, copies d4 into d2 and increments d2 by one, then branches onward; if zero, branches to the alternate path.
- `zero_d2_and_return_zero_d0`: Clears d2, tests it (always zero so no branch), sets d0 to zero, restores d2-d5 from the stack and returns to the caller.
- `parse_icon_copy_spec_into_buffer`: Saves registers, clears byte 30 of the first argument, parses the second argument's copy specification via library calls, and fills the destination buffer with the parsed count and text.
- `icon_parse_skip_five_bytes_and_call`: Advances a4 by five, pushes three arguments and calls a library routine, then skips twelve stack bytes; on failure sets d3 to one and advances a4 by three before branching away.
- `icon_set_d3_zero_and_branch`: Sets d3 to zero and branches forward into another routine, used as the success path after the preceding icon parse call.
- `icon_parse_decimal_number_string`: Builds a decimal value in d3 from bytes at a4, calling a digit conversion routine per character, then optionally formats and copies the result via library calls before returning a2.
- `ascii_digit_to_value`: Reads the longword argument from the stack, checks it lies between character codes for digits zero and nine, and returns its numeric value or falls through to the shared failure path.
- `return_minus_one_failure`: Loads minus one into d0 and returns immediately, serving as the shared failure exit for the digit conversion routine.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `icon_lookup_with_fallback`: Saves d2-d5, calls the icon open routine with two arguments, then calls the match routine with the result and arguments, closing the icon and returning zero if matching fails.
- `alloc_icon_node_and_init`: Saves d2/a2, allocates 96 bytes with tag 65536, stores 10 in the node's word at offset 14 and in the caller's word, then links the node into the caller's list and returns 1 on success.
- `return_zero_restore_d2_a2`: Sets d0 to zero as the return value, restores d2 and a2 from the stack and returns to the caller.
- `compare_three_args_via_call`: Loads three arguments from the stack, pushes them back, calls the routine at offset 48, compares the result with the fourth argument and returns zero unless they match.
- `return_d1_in_d0`: Copies the longword in d1 into d0 and returns, so the caller receives d1 as the result.
- `compare_three_args_via_library_call_2`: Pushes the three longword arguments from the stack, calls the library routine at offset 76, compares the result with the saved fourth argument, cleans the stack and returns zero or the result in d0.
- `return_d1_in_d0_alt_entry`: Copies the longword in d1 into d0 and returns, providing the same d1-to-d0 result copy through a second entry point.
- `signed_32bit_divide_by_16bit`: Compares d2 with 65535; if larger, performs a 32-bit division of d1 by d2 via shift-subtract loop, otherwise divides the high and low words of d1 separately by the low word of d2, returning quotient in d0 and remainder in d1.
- `unsigned_32bit_divide_loop`: Shifts d1 left through 16 iterations, subtracting d2 from the high part when it fits and setting the low bit, leaving the quotient in d1 and remainder in d0.
- `multiply_32bit_words_to_64bit`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, combines them into a 64-bit result in d0, and returns with d2 preserved.
- `icon_format_string_via_rawdoffmt`: Saves a2-a4/a6, loads format string and argument pointer from the stack, points a2 at a pc-relative format, calls exec RawDoFmt through a6, then restores registers and returns.
- `store_byte_d0_to_a3_postincrement`: Writes the low byte of d0 to the memory pointed to by a3, then increments a3 by one and returns to the caller.
- `icon_strlen_count_bytes`: Loads the string pointer from the stack into a0, scans up to 255 bytes for a zero terminator using dbeq, then returns the complemented count in d0.
- `find_char_in_string_or_return_zero`: Loads a0 from the first stack argument and d0 from the second, reads a byte from (a0)+ into d1; if zero returns zero, if equal to d0 backs a0 up by one and returns that pointer, otherwise continues scanning.
- `return_zero_and_branch_to_shared_exit`: Sets d0 to zero with moveq and branches to the shared exit path of the string search routine, returning a null result to the caller.
- `copy_string_until_null_terminator`: Loads a0 and a1 from the stack arguments, then copies bytes from (a1)+ to (a0)+ repeatedly until a zero byte is copied, and returns.
- `skip_leading_nul_advance_a0`: Loads a0 and a1 from the caller's stack arguments, reads a byte at a0 and increments a0; if the byte is nonzero it branches ahead, otherwise it backs a0 up by one and branches elsewhere.
- `compare_strings_with_length_limit`: Loads a0 and a1 from the stack, takes a length from the stack into d0, decrements it and returns negative if exhausted; otherwise compares bytes from a1 and a0, returning zero on a full match or a mismatch result.
- `return_one_or_minus_one_by_flags`: Sets d0 to one, branches if the condition flags indicate greater, otherwise sets d0 to minus one and returns to the caller.
- `scan_string_until_nul_or_count`: Loads a0 and a1 from the stack, loads a count into d0, then advances a0 past bytes until a zero byte or the count runs out, returning via the caller's flow.
- `decrement_a0_and_copy_string`: Subtracts one from a0 and branches to the string copy routine, then returns to its caller.
- `copy_bytes_with_zero_fill_2`: Copies bytes from a1 to a0 while decrementing d0, stopping on a zero byte or exhausted count, then pads the destination with a zero byte.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `init_icon_header_self_pointers`: Loads the first stack argument into a0, stores a0 at (a0), adds 4 to that longword, clears the longword at offset 4, stores a0 at offset 8 and returns.
- `draw_icon_with_scaled_offsets`: Saves d2-d3/a2, loads three stack arguments, calls a library routine with a size of 20, then scales two word fields by 8 and calls the routine again with the icon data.
- `load_args_and_branch_to_icon_code`: Saves d2-d3/a2 on the stack, loads the three stack arguments into d2, d3 and a2, then branches unconditionally into another library routine.
- `walk_icon_list_validate_entries`: Walks a linked list of icon nodes in a2, calling the library validation routine on each node's fields and following the next pointer at offset 12, returning 1 when the list ends or 0 on failure.
- `icon_library_return_zero_restore_d2_d3_a2`: Library code sets d0 to zero as its return value, then pops d2, d3 and a2 from the stack and returns to the caller.
- `create_icon_from_definition`: Allocates a 20-byte icon structure, copies the definition data into it, computes scaled dimensions, creates the two bitmaps and stores the second bitmap pointer at offset 10, returning the icon or -1.
- `icon_library_fail_return_minus_one`: Sets d0 to minus one as the failure return value, restores d2 through d5 and a2 from the stack, then returns to the caller.
- `icon_library_save_regs_load_three_args`: Pushes d2 through d6 and a2 onto the stack, loads three longword arguments from the caller's stack into d2, d3 and d4, clears d6, then branches onward.
- `icon_library_push_args_call_and_test`: Pushes a size, a low-memory pointer, d3 and d2 as arguments, calls a library routine, stores the result, discards the arguments, then tests the result and d6 before branching.
- `icon_library_parse_icon_chunks`: Walks a linked list of icon structures, calls library helpers to allocate and read chunk data, stores results at offsets in the icon node, and returns minus one on failure or the saved d6 value on success.
- `icon_library_return_minus_one_2`: Loads minus one into d0 as the failure return value, restores registers d2 through d6 and a2 from the stack, then returns to the caller.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 2 of 2).adf:Challenge

- `challenge_load_and_decompress_rnc_data`: Allocates memory via exec AllocMem, scans the loaded file for an RNC signature, copies and decompresses the packed data into the buffer, then frees the allocation and repeats.
- `decode_bitstream_symbol`: Scans a0 pairs until d6-masked word equals the next word, then consumes bit counts from offsets 60 and 61, refilling d6 via the bit routine and setting bit d2 in d0 before returning.
- `set_error_code_and_bit_count`: Loads d0 with minus one and d1 with sixteen, returning these two register values to the caller.
- `mask_d0_and_refill_bits`: Masks d0 with d6, subtracts d1 from d7, refills d6 through the bit routine if the count went negative, then returns the masked value in d0.
- `extract_three_bit_field_from_stream`: Adds d1 to d7, shifts d7 right by d6 bits, swaps d6, advances a3 by four, reads two bytes backwards from a3 into d6 with a byte swap, subtracts d7 from d1, sets d7 to sixteen minus d1 and returns.
- `build_huffman_decode_table`: Reads code lengths via a helper, then fills a table at a0 with masks, bit-reversed codes, symbol lengths and offsets for each of seventeen code lengths, returning after restoring d5-d7 and the stack.
- `free_chunk_list_and_exit`: Walks a list at a4 while its first longword equals one thousand four, clears entries, adjusts pointers at a5 by scaled offsets, then frees memory through exec FreeMem and restores registers.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr DLT](Disk 2 of 2).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_library_and_memory`: Calls several routines to close a library and free memory, then performs a series of operations involving a structure in a6, including memory allocation and byte manipulation within a buffer.
- `release_library_and_memory_2`: Closes a library and frees memory pointed to by a register, then calls another routine to perform a task involving a structure in a6.
- `release_library_and_memory`: Closes a library, frees memory pointed to by a register, and then calls another routine to perform a task involving a structure in a6.
- `clear_hardware_and_call_subroutines`: Clears 32 words starting at the hardware register address in a1 using a loop, then executes a long sequence of subroutines.
- `find_task_and_copy_data`: Clears a global variable, finds a task using a library call, and if successful, copies a block of data from one memory location to another and updates hardware registers.
- `copy_data_and_enable_interrupts`: Copies a block of data from one location to another and updates two hardware registers by setting their most significant bit.
- `update_registers_and_globals`: Writes specific values to the INTENA hardware register and updates two global variables.
- `check_bit_and_increment_counters`: Checks bit 5 of a hardware register; if set, it calls a subroutine and increments three different global variables.
- `execute_subroutines_with_pointers`: Calls a subroutine, then performs two separate calls to another routine using different pointer values and offsets, storing the results in a table.
- `build_complex_data_table`: Initializes a large data structure with various constants, populates it with values from a table using a loop, and performs several more loops to generate patterned data before updating hardware registers and global pointers.
- `update_interrupt_and_pointer`: Writes values to the INTENA hardware register and updates a global variable with a new pointer.
- `open_libraries_and_check_bit`: Opens two libraries, checks bit 2 of a word from the first library, stores that bit in a global variable, and then writes a value to a location within the second library.
- `update_pointer_and_cop_register`: Swaps two pointers stored in global variables, modifies a global value by XORing it with 4, and then updates a COP1LCH register and a global variable with a value derived from that pointer.
- `update_joystick_and_status`: Reads a value from the joystick port, performs arithmetic to update several global variables, and conditionally modifies hardware registers and other global status flags.
- `read_mouse_position`: Reads a 16-bit value from the mouse port, masks it to 8 bits, stores it in a global variable, and then stores the result of the original value shifted right by 3 into another global variable.
- `calculate_and_store_offsets`: Calculates several values based on a global variable and a constant, then writes these values into specific offsets of a structure pointed to by a table.
- `execute_with_interrupt_control`: Disables interrupts, copies a pointer from a global variable to another location, sets values in a structure pointed to by a register, calls another routine, and then modifies a byte at a specific hardware address before re-enabling interrupts.
- `swap_interrupt_pointers`: Disables interrupts, swaps the values of two global variables, and then re-enables interrupts.
- `calculate_serper_value`: Sets a value in the SERPER register by dividing a constant by a value from a global variable and subtracting one, otherwise it sets a different constant.
- `update_sprite_position_offset`: Adds a value from a global variable to a 16-bit coordinate stored in a structure pointed to by a6, then calls another routine.
- `call_rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `call_delay_routine`: Populates a structure at a specific memory location with several parameters and calls a routine via a pointer in a global variable, then stores the return value in another global variable.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `loop_until_condition_and_call_dos`: Enters a loop that calls a routine until it returns zero, then calls three different routines via pointers in global variables and sets a global variable to 1.
- `forbid_task_and_wait`: Calls a routine via a pointer, waits in a loop until bit 6 of a hardware register is cleared, calls another routine via a pointer, and sets a hardware register to 40.
- `permit_task_and_wait`: Calls a routine via a pointer, waits in a loop until bit 6 of a hardware register is cleared, calls another routine via a pointer, and sets a hardware register to 8040.
- `execute_conditional_tasks`: Calls three different routines and performs a comparison on a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_sequence_and_dispatch`: Calls two separate routines using pointers from global variables, clears a 16-bit value in a global variable, and executes two additional routines.
- `dispatch_sequence_pointers`: Loads two pointers into registers a2 and a3, then calls two routines using those pointers.
- `execute_sequence_and_store_byte`: Calls three different routines in sequence and then stores a byte from a register into a location pointed to by a register.
- `call_and_clear_global`: Calls another routine, clears a 16-bit word in a global variable, and returns.
- `process_table_data`: Checks a 16-bit value in a global variable against several table values, and if conditions are met, iterates through a table to find and swap specific byte values, then calls another routine and returns a 16-bit value.
- `parse_string_template`: Processes a source string by expanding placeholders enclosed in braces using data from a secondary pointer list.
- `process_input_characters`: Compares a byte in d1 against several constants and, if it falls within the range of uppercase letters, performs a loop to shift characters in a buffer or clear a byte.
- `store_byte_to_table_2`: Compares a byte in register d1 to 32 and, if it is not less than that value, writes the byte to the memory location at offset minus 1 from a table.
- `store_byte_to_table`: Compares a byte in register d1 to 32 and, if it is not less than that value, writes the byte to the memory location at offset minus 1 from the pointer in a2 and writes zero to the location pointed to by a2.
- `call_two_subroutines`: Loads a long value from a global variable into a0, calculates an address in a1 by adding a long value from a global variable to a pointer from the same global variable, loads two words from subsequent global variables into d0 and d1, and calls two different routines.
- `copy_hunk_byte_table`: Reads a byte stream from a pointer in a1 to determine indices, then copies bytes from a global table into a destination buffer at a0 using those indices.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_execute_data_2`: Copies 64 sets of word data from a source table to a destination structure using a loop, then decrements a counter and calls two routines and several others based on the results.
- `set_increment_value_one`: Sets a register to 1 and branches to another routine.
- `set_increment_value_two`: Sets a register to 2 and branches to another routine.
- `set_increment_value_three`: Sets a register to 3 and branches to another routine.
- `increment_table_values`: Uses a loop to increment bytes at specific offsets within a table calculated from a global variable, then decrements a counter in that table.
- `copy_and_execute_data`: Copies 64 sets of word data from a source table to a destination structure using a loop, then decrements a counter and calls two routines and a function pointer from a calculated table based on a value from a stack-based pointer.
- `execute_nested_double_call_loop`: Runs a nested loop that executes another routine twice per inner iteration, where the outer loop runs twice and the inner loop runs three times.
- `transfer_word_blocks`: Copies word data from one memory location to another in blocks of four words at specific offsets, choosing between two different source pointers based on a comparison of a value in d4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `double_d7_and_branch`: Multiplies the value in d7 by two and branches to another location.
- `update_blt_parameters`: Calculates and writes new source or destination pointers, or size and mode parameters, to a set of global variables and certain hardware registers, using a table lookup or conditional logic based on input or register values.
- `extract_and_mask_value`: Calls another routine, stores its 32-bit result in a global variable, then returns the concatenation of the high 24 bits of that result with the low 8 bits of the input register, masked to 43 bits.
- `multiply_32_bit_integers`: The routine takes two 32-bit values from registers and returns a 64-bit product formed by combining the low 16-bit multiplication and the high 16-bit cross terms, with the high 32 bits placed in the upper half of the result.
- `copy_16_word_blocks`: The routine copies 500 blocks of 16 words from a source pointer to a destination pointer, word by word, using a fixed loop count.
- `interpolate_tables_iteratively`: The routine performs a nested loop of 17 iterations, where each iteration calls another routine to fetch data and then performs complex arithmetic to interpolate values between two source tables and store them in two destination tables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decode_and_loop`: The routine calls another routine, then XORs two consecutive words pointed to by a pointer with constants, then repeatedly calls another routine until a short value in a register is zero, and finally calls another routine once more.
- `encode_id_to_base36`: The routine encodes a 32-bit value in D0 into four characters written through a pointer in A1.
- `swap_bytes_in_struct`: The routine swaps pairs of bytes within a structure pointed to by a0.
- `rotate_bits_in_place`: Rotates a block of eight bytes in place, shifting each byte right by one bit and feeding the carry into the next byte, then stores a pointer to another routine and a global variable into the buffer.
- `process_and_xor_structure`: Runs a nested loop that executes another routine twice per inner iteration, then performs an XOR operation on two 16-bit values at specific offsets within a structure pointed to by a0.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decode_four_byte_value`: The routine reads four bytes from a pointer in A0 and adds a constant to each byte.
- `decompress_bit_stream`: Processes a bit stream by reading bits and using them to index into a table to copy bytes into a destination buffer.
- `branch_if_not_equal`: Performs a conditional branch if the zero flag is not set, otherwise it returns.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `process_bit_stream`: The routine contains a loop that shifts a 32-bit value and performs bitwise operations based on a counter in D1, and if a condition is met, it proceeds to copy and manipulate data from a source pointer into a destination table using multiple loops.
- `update_resource_table_and_execute`: Iterates through a table to subtract values from a counter based on entries in a secondary table, then executes a sequence of subroutines that includes conditional logic based on the results of those calls and modifies a bit in a global variable before calling several final subroutines.
- `validate_and_process_entity_list`: Calls validate_and_update_memory to synchronize structure fields, then iterates through a list of entities starting at a specific memory address.
- `validate_and_update_memory`: Checks a memory range for specific byte values and calls another routine if conditions are met, then updates several fields in a structure pointed to by a1.
- `check_memory_range`: Compares a pointer derived from a global address against a constant value, and if the condition is met, calls another routine and checks if a value at an offset of 90 from a pointer in a1 matches the incremented value of d1.
- `initialize_system_state`: Clears three global variables, copies several values from two tables into a stack-based sequence, and if a global variable is zero, initializes a series of bytes and a pointer with specific constants.
- `calculate_derived_value`: The routine calls another routine twice and then stores a value derived from the low word of a register and another register into a global variable.
- `swap_two_entries`: The routine swaps a small fixed-size block of bytes and two 16-bit words between two entries indexed by a short in D0 and a global short.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_tile_flags_and_neighbor`: The routine indexes a global variable table with an index from the first argument and calls another routine twice with values derived from the entry and a global table, then if a short field is non-zero it repeats the calls for the referenced entry.
- `update_global_pointer`: If a global variable is non-zero, it increments the variable by four, stores it in another global variable, and calls two other routines.
- `initialize_interrupt_handlers`: Writes a pointer to a global variable and several subroutine addresses into various locations in RAM, registers an interrupt server, calls an indirect routine through a global variable, and writes three bytes to a global variable before returning the concatenation of the two input registers.
- `trigger_interrupt_handler`: If a global variable is non-zero, it sets a byte at offset a fixed address to one, calls a routine via a pointer at offset 4, and sets a hardware register to 15.
- `set_unit_flags_by_mask`: Iterates up to four times through a table of unit structures, using a bitmask derived from the low nibble of the input parameter to select entries.
- `compare_and_increment_pointer`: Loads a 16-bit word from the address in a0 into d3, compares d3 with d1, and if they are not equal, increments the address in a0 by 4 and loops back to the start.
- `tiny_movea_move_rts`: Small routine of 3 instructions that copies (a0) to a0, then copies a0 to -(a7), then returns.
- `set_carry_flag`: Sets the carry flag in the status register and returns immediately.
- `handle_data_pointer`: The routine receives a pointer in a register, loads a global variable and stores the pointer value into a table entry via an indirect address derived from the pointer.
- `process_flagged_entities`: The routine iterates over four entries in a table of structures and, for each entry whose flag bit is set in a mask, updates several bytes in the structure based on bits in a 64-bit value returned by another routine, including clearing or writing fields and setting bits in a status word, and may call another routine and continue processing.
- `update_structure_values`: Clears bit 0 of a byte at offset 11 of a structure in a0, then conditionally updates various fields in the structure and a pointed-to table based on comparisons and arithmetic involving offsets 0, 1, 2, 5, 8, 10, 12, and 13.
- `set_status_bit`: Sets bit 2 of a byte at offset 11 from a pointer in a0 and returns immediately.
- `validate_and_reset_entity_attributes`: Checks if two consecutive bytes at offsets 8 and 12 of a structure are both 255; if they are not, it evaluates bit 5 of a status byte at offset 39, and if that bit is clear, it modifies several fields in a linked entity structure by setting bit 8 of a word at offset 38, clearing a byte at offset 43, and clearing bit 1 of a byte at offset 39 before updating a word at offset 38 of the primary structure and returning.
- `copy_and_clear_data`: Copies a 32-bit word from the address in a3 to the address at offset 12 of a3, clears a 16-bit word at offset 36 of a3, and branches to another routine.
- `update_entity_index_and_fetch_pointer`: Increments a 16-bit counter at offset 36 of a structure pointed to by a3, then uses a value from d0 to index into a table via a0 to fetch a new pointer which is stored at offset 8 of the structure.
- `update_entity_state_and_reset_attributes`: Calculates an index from a register and a byte in a structure to fetch two words from a table, then clears several byte and word fields in two separate structures pointed to by offsets 16 and 20 of a structure in a3 before branching to another routine.
- `branch_on_status_bits`: Checks bit 6 of a register and branches, otherwise checks bit 5 of the register and branches, otherwise masks the register with 31 and stores the result at offset 24 of a structure in a6 before branching.
- `load_sprite_attribute_data`: Masks the lower four bits of a register and uses the result as an index into a table to load two consecutive bytes into offsets 40 and 41 of a structure pointed to by a3.
- `initialize_entity_data_from_table`: Checks a bit in a register and, if not set, calculates an offset into a table using a value in a register, extracts a status byte and a data byte, and populates a structure with these values while clearing several fields and setting a flag.
- `fetch_table_entry_via_index`: Loads a pointer from a table using a value in d0 that has been masked to the lowest three bits and shifted left by two, then pushes that pointer onto the stack before returning.
- `update_pointer_with_min`: Compares a byte from a pointer to a value at another pointer and stores the minimum or an offset-calculated value into a global pointer.
- `clear_dma_control_register`: The routine writes zero to the DMACON hardware register and sets a global variable to a bit mask derived from an input value.
- `update_entity_pointer_and_byte`: Loads a pointer from offset 8 of a structure into a register, reads a byte from that pointer and stores it at offset 42 of the structure, updates the pointer in the structure to point to the next byte, and branches to another routine.
- `initialize_entity_attribute_structure`: Reads a byte from a pointer stored at offset 8 of a structure, multiplies it by 13, and uses the result to calculate an offset into a data table.
- `initialize_entity_from_attribute_table`: Reads a byte from a pointer stored at offset 8 of a structure, scales it by 13, and uses the result as an offset into a data table to initialize a new entity structure.
- `branch_to_subroutine`: Returns immediately by performing a relative branch to another routine.
- `process_data_structure`: Calculates an offset based on a value in d0, accesses a table at that offset, and enters a loop that calls various routines and updates a global variable based on the results of those calls.
- `allocate_and_initialize_memory`: Allocates memory using a library call, and if successful, initializes specific entries in a table at offset 20820 before performing a second allocation and potential initialization.
