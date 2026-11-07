# Game 2017 (populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf](disk 1 of 2)[t +4 afl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02017). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904944 top-level bytes in 48 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 230 of 230 named (0 author symbols), 230 of 230 described (34 by rule, 70 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1013 of 1013 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4279 of 4279 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 427612 |
| code | 14044 |
| data | 422992 |
| padding | 12648 |
| unused | 27648 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:Install | 2 | 267 | 8 |
| EndCLI | 2 | 117 | 2 |
| LoadWB | 12 | 536 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:CREDITS.EXE | 2 | 167 | 7 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:POPINTRO | 2 | 79 | 3 |
| icon.library | 10 | 1375 | 109 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:POP2_TRAINER | 1 | 187 | 38 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:POPULOUS.II | 2 | 267 | 8 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 452 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 388 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |

## Cases handled

- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf](disk 1 of 2)[t +4 afl]/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:awards.pak: high-entropy data file (7.70 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf](disk 1 of 2)[t +4 afl]/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:hands.pak: high-entropy data file (7.68 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf](disk 1 of 2)[t +4 afl]/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:end.pak: high-entropy data file (7.66 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf](disk 1 of 2)[t +4 afl]/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:judge.pak: high-entropy data file (7.68 bits/byte), classified as packed data
- runtime segment an address (play-rs-m1500): 432 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 240 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf](disk 1 of 2)[t +4 afl]/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:CREDITS.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf](disk 1 of 2)[t +4 afl]/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:POP2_TRAINER: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf](disk 1 of 2)[t +4 afl]/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:POPULOUS.II: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:POPINTRO
- Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:POP2_TRAINER

## Copy protection found

- none found

## Functions


### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:Install

- `start_installer_program`: Entry point of the Install program.
- `alert_then_close_library`: Calls two library functions in sequence through the library base in register A6.
- `invoke_two_function_pointers`: The routine calls two functions through pointers stored in a global variable.
- `open_titled_console_window`: Opens a console window with a fixed size and title text, then closes the file handle through a second library call.
- `read_input_line_until_marker`: Reads a line of user input, removes the trailing newline and repeats a check routine until it returns a FAIL or OK four-character code.
- `copy_file_contents_to_output`: Builds a destination path from the source name and the chosen location, opens the input and output files, then copies the data in a read and write loop.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `write_null_terminated_string`: Measures the length of the null-terminated string at the address in register A0, then writes that many bytes through the library function Write.

### EndCLI

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_table_block_then_call`: Copies sixteen longwords from a fixed table into a caller-supplied buffer when the buffer pointer is not null, then calls a function pointer held in register A5.

### LoadWB

- `start_loadwb_command_line`: Entry point of the LoadWB command.
- `release_startup_resources_and_reply`: Stores a value into the startup message slot and, depending on which pointers are set, closes the file, frees the library and buffer memory and replies to the startup message.
- `launch_workbench_tool`: Opens the icon and intuition libraries, decodes a stored name by XORing each byte with a fixed key, then finds a resident tag and starts the tool.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte.
- `build_resident_tag_list`: Walks a linked table of entries, allocating a cleared node for each one and filling it with the result of a resident-tag lookup.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `pad_workbench_version_suffix`: Compares the start of a version string with the reference text 'Workbench release 1.2' and, when the version character matches, overwrites the bytes after the release number with a dot and blanks.
- `write_dot_and_blank_padding`: Writes a period followed by four spaces into five consecutive bytes at the address in register A2.
- `skip_matching_string_prefix`: Advances the pointer in register A2 while its bytes match a global string, stopping when the global string ends.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends with a return.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_with_stack_argument`: Forwards its two parameters to another routine together with the address of a fixed stack location, then returns.
- `measure_and_forward_stack_text`: Calls a library function through a global pointer, then measures the null-terminated text in a 140-byte stack buffer.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Disk-Validator

- `verify_disk_bitmap_blocks`: Walks the words of the disk's block bitmap, testing each bit against the expected allocation state and counting mismatches.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:CREDITS.EXE

- `unpack_compressed_executable`: Checks the first longword for the packed-file signature and, when it matches, reads the header fields and copies the packed data backwards into place.
- `read_longword_from_bytes`: Reads four consecutive bytes from the address in register A0 and combines them into a 32-bit value in the data register, most significant byte first.
- `copy_bytes_backward_from_bitstream`: Reads a count and an offset from a bit stream, then copies that many bytes backwards from one buffer to another until the count is exhausted.
- `empty_routine_2`: The routine contains five instructions and performs no observable work; it ends immediately.
- `decode_count_and_repeat`: Reads bits until a count reaches a negative sentinel, then looks up a flag in a global table indexed by that count.
- `decode_symbol_sequence`: Reads symbols from a bit stream in one of two modes.
- `busy_wait_loop`: Decrements a counter register until it reaches zero, which provides a short timed delay.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:POPINTRO

- `unpack_intro_with_bit_reader`: Program entry for the intro, which reads packed data through the bit-reader routines and writes the background colour register.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_flag`: Library code that increments a 16-bit counter in the structure at the base pointer and clears bit 3 of the flag byte that follows.
- `decrement_refcount_and_cleanup`: Library code that decrements a 16-bit reference count in the structure.
- `free_node_or_mark_pending`: Library code that unlinks a node from its list and frees its memory through a library function when the busy field is zero.
- `set_pending_free_flag`: Sets bit 3 of the flag byte in the structure at the base pointer and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_or_alert`: Stores two values into the structure, opens the dos.library through a library call and stores the result.
- `copy_two_pointers_from_table`: Copies two 32-bit pointers from the structure addressed by a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `jump_to_shared_handler`: Forwards control to another routine with a single jump instruction, adding no logic of its own.
- `call_library_handler`: Calls one other routine and then returns, with no other work.
- `call_helper_and_return`: Calls one other routine and returns, with no other work.
- `call_setup_routine`: Calls one other routine and returns, with no other work.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_library_handler_2`: This library routine simply calls another routine and then returns.
- `invoke_handler_via_structure_offset`: This library routine calls another routine using a pointer to a specific field within a provided structure, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_icon_handler`: This library routine calls a handler routine via a pointer retrieved from a data structure and then returns.
- `call_library_handler_3`: This library routine calls another routine via a pointer located in a structure field and then returns.
- `call_other_routine`: The library routine calls another routine and then returns.
- `call_other_routine_2`: The library routine calls another routine and then returns.
- `call_build_copy_name`: This library routine calls a routine that builds a copy name from a numeric count and then returns.
- `store_parameter_in_returned_struct`: Calls a function through a pointer stored in the library base field, passing the first parameter, then stores the second parameter at a fixed offset in the structure that call returns.
- `call_pair_then_release`: Calls one routine and, if it returns a non-null result, passes that result to a second routine.
- `copy_state_fields_into_structure`: Copies ten longwords from a stack buffer and several single fields into the structure returned by a call.
- `copy_state_fields_from_frame`: Copies ten longwords and several single fields from the caller's frame into the structure at the address in register A2.
- `return_zero_stub`: Returns zero with no other work.
- `pack_state_fields_and_call`: Copies ten longwords and several single fields from the structure passed in as the third parameter into a stack record, then passes that record to another routine.
- `read_icon_info_file`: Reads an icon file ending in .info into a 260-byte stack buffer, checks the header magic word and then reads the icon's fields.
- `parse_icon_structure_fields`: Continues reading an icon structure after the header check, storing the image pointers and list links it reads from the file.
- `read_icon_image_pointers`: Reads the remaining icon fields and allocates the image and list-node structures from the file data, storing their pointers.
- `read_longwords_into_table`: Reads longwords into an array until a byte counter in the frame is exhausted, then reads a final value.
- `call_helper_return_zero`: Calls one other routine and then returns zero.
- `empty_routine_3`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `allocate_and_read_icon_block`: Allocates a 260-byte block and reads icon data from the file into it, using the marker values stored with the data.
- `process_icon_structure_fields`: Reads the icon structure flag bits and calls library functions to handle its image and text fields, with the .info extension referenced in the same routine.
- `process_icon_pointer_lists`: Computes the total length of each null-terminated pointer list in the structure, then passes the entries through library calls.
- `process_icon_string_list`: Counts the entries of a null-terminated list, then passes each entry to a library call with its length.
- `process_null_terminated_pointer_table`: Walks a null-terminated table of pointers, passing each entry to a helper and checking the library results.
- `iterate_list_until_end_marker`: Walks a list starting at an embedded node, calling a helper on each node until it reaches the list's end marker, then calls the helper once more on the last node found.
- `traverse_node_list`: Walks a linked list, passing each node to a helper and advancing through its next pointer.
- `call_with_structure_field_pointer`: Calls another routine with a pointer to a field 140 bytes into the structure it was given, then returns.
- `allocate_icon_structure`: Allocates a cleared 168-byte structure, stores a link to it and calls a helper with it.
- `decrement_structure_counter`: Library code that returns zero and decrements a counter within a structure, refilling it via a helper if it reaches zero.
- `return_zero_3`: Library code that returns zero.
- `store_pair_with_refill`: Decrements a counter in the structure passed in; when it runs out, it refills through a helper.
- `store_pair_without_refill`: Decrements a counter in the structure, then stores two values into the array at the new index and returns one.
- `allocate_icon_node_structure`: Allocates a cleared 94-byte node, fills in its link and calls a helper with it.
- `return_without_action`: Returns immediately with no work done.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `call_with_structure_field_pointer_2`: Calls another routine with a pointer to a field 78 bytes into the structure it was given, then returns.
- `find_entry_value_after_equals`: Searches a null-terminated list of names for one whose text is followed by an equals sign, and returns the text after that sign.
- `find_text_after_equals`: Scans a key and returns the text that follows the matching key with an equals sign.
- `find_entry_value_after_equals_2`: Searches a null-terminated list of entries for a key followed by an equals sign and returns the text after that sign.
- `find_matching_bar_entry`: Walks a string of entries separated by vertical bar characters, comparing each entry's length with a required length and checking it with a helper.
- `test_bar_entry_length`: Scans bar-separated entries and returns one when an entry of the required length passes a helper check.
- `scan_bar_entries_with_index`: Scans bar-separated entries using a running index and returns one when an entry matches the required length and passes a helper check.
- `scan_bar_entries_for_length`: Walks bar-separated entries until the last one, checking each length with a helper.
- `find_bar_entry_by_length`: Walks bar-separated entries and returns one for the first entry whose length matches and whose helper check succeeds.
- `build_copy_name_from_number`: Reads a numeric copy count from the source text and writes a name built from the text 'copy' with that count, then returns the output buffer.
- `parse_decimal_after_prefix`: Parses a decimal number that starts five bytes into the given text, checking the digits with a helper, then calls two follow-up routines before returning.
- `parse_decimal_digits`: Reads decimal digits from the text at the pointer, building a number until a non-digit is reached.
- `parse_copy_count_from_text`: Reads a decimal copy count from the text, and when the count is non-zero and a helper check fails, it calls two cleanup routines before returning.
- `digit_char_to_value`: Converts a character to its decimal digit value, returning minus one when the character is not a digit from zero to nine.
- `return_minus_one_stub`: Returns minus one with no other work.
- `call_library_vector_198`: Calls the library function at offset 198 through the library base held in a global variable and returns.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Calls the library function at offset 42 from the library base stored in a global variable and returns.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `allocate_counted_record`: Allocates a cleared 96-byte record, stores a count of ten in it and in the caller's pointer, then calls a helper with the record.
- `return_zero`: The library routine returns zero.
- `check_result_equals_argument`: Calls another routine and returns its result when it equals the fourth parameter, otherwise returns zero.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `validate_library_function`: Library code that calls a library function and returns zero if the returned value does not match a provided parameter.
- `return_empty`: The library routine returns without performing any operations.
- `divide_longword_by_divisor`: Divides a 32-bit value by a divisor, using a shift and subtract loop when the divisor is larger than 16 bits and word-sized division otherwise.
- `divide_by_shift_subtract_loop`: Divides a 32-bit value by the divisor one bit at a time, shifting quotient bits in and subtracting the divisor whenever it fits.
- `multiply_32bit_by_partial_products`: Multiplies two 32-bit values by splitting each into 16-bit halves and adding the cross products.
- `call_raw_do_fmt`: Calls the exec RawDoFmt function through the library base stored in a global variable and returns.
- `store_byte_at_pointer`: Stores the low byte of the data register at the address held in register A3 and returns.
- `measure_null_terminated_length`: Counts the characters of a null-terminated string and returns the count, not including the terminator.
- `find_character_in_string`: Scans a null-terminated string for a given character and returns a pointer to its first match, or nothing when the string ends first.
- `return_null_pointer`: This library routine always returns a null pointer.
- `copy_null_terminated_string`: Copies bytes from the source string into the destination until the null terminator has been copied.
- `append_null_terminated_string`: Moves to the end of the destination string, then copies the source string onto it including its terminator.
- `compare_strings_up_to_count`: Compares two strings for up to a given number of characters.
- `return_one_stub`: Returns one with no other work.
- `walk_string_with_count_limit`: Walks a string while counting down a limit.
- `library_routine_recursively_calls_itself`: Library code that calls itself and then returns.
- `copy_string_with_zero_padding`: Copies a string into a fixed-length buffer, stopping at the length limit, then fills the rest of the buffer with zero bytes.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `init_empty_list_header`: Initializes a list header so its first and last pointers point to itself and its middle field is cleared, leaving the list empty.
- `check_then_process_block`: Returns one when the input pointer is null.
- `walk_entries_until_end`: Walks a linked chain of entries, calling a helper with each entry's fields and a fixed size.
- `verify_linked_entries`: Walks a linked chain, calling two helpers on each entry, and returns one when every entry succeeds and the chain ends.
- `return_zero_2`: This library routine returns a constant zero value.
- `allocate_and_store_record`: Allocates a cleared 20-byte record, reads a value through two helper calls and stores the result at offset 10 of the record.
- `return_all_ones`: The library routine returns a value where all bits are set.
- `build_linked_entry_nodes`: Allocates nodes in a loop, linking each new node to the previous one and filling it through helper calls.
- `build_linked_nodes_from_chain`: Walks a chain of entries, allocating a node for each step and linking it onto the previous one through helper calls.
- `walk_and_link_node_chain`: Repeatedly takes the next node in a chain, fills it through helper calls and links it to the one before.
- `return_all_ones_2`: The library or routine returns a value where all bits are set.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:POP2_TRAINER

- `copy_code_block_and_jump`: Program entry for the trainer.
- `unpack_packed_block_backwards`: Copies a block of longwords into place, then decodes the packed stream from its end, reading one bit at a time and copying output bytes into the destination buffer.
- `unpack_stream_word_bits`: Loads a packed longword and shifts its bits out one at a time, refilling from the packed data when the word runs out.
- `jump_to_unresolved_target`: Transfers control through a jump-table pointer that the decompiler could not resolve, then returns.
- `store_byte_and_dispatch`: Stores one byte just before the address in register A2, then jumps to one of two unresolved targets depending on whether a counter is zero.
- `dispatch_on_flag_byte`: Calls one of two unresolved jump targets depending on whether the flag byte in the data register is non-zero.
- `jump_to_pointer`: Performs an indirect jump through a pointer stored in a jump table.
- `jump_to_unresolved_target_2`: Transfers control through a jump-table pointer.
- `invoke_indirect_function`: loads a function pointer from a memory location and transfers control to that address.
- `invoke_indirect_function_2`: Loads a function pointer from a memory location and transfers control to that address.
- `decode_packed_bit_stream`: Decodes a packed stream by shifting its bits out one at a time, refilling from the packed data as needed, and writing decoded bytes through the output pointer.
- `dispatch_by_small_count`: Chooses between two unresolved jump targets depending on whether the count in the data register is below six, equal to six or above.
- `branch_on_unresolved_target`: Chooses between two unresolved jump targets based on a value in a register.
- `dispatch_on_count_above_one`: Chooses between two unresolved jump targets depending on whether the count in the data register is greater than one.
- `jump_to_unresolved_address`: Transfers control through an unresolved jump-table pointer.
- `execute_indirect_jump_2`: Executes an indirect jump to an unresolved address.
- `perform_indirect_call`: Uses a pointer from an unresolved jump table to transfer control.
- `branch_on_register_value`: Checks a register and performs an indirect jump to an unresolved target regardless of whether the register is zero or non-zero.
- `transfer_control_indirect`: Loads a function pointer from a memory location and executes it.
- `dispatch_on_value_below_three`: Chooses between two unresolved jump targets depending on whether the value in the data register is below three.
- `dispatch_on_value_above_two`: Chooses between two unresolved jump targets depending on whether the value in the data register is above two.
- `branch_on_byte_greater_than_two`: Checks if a byte is greater than two and executes an indirect jump to an unresolved target.
- `jump_to_unresolved_address_3`: Executes an indirect jump to an unresolved address, potentially following a conditional check.
- `branch_on_byte_non_zero`: Checks if a byte is non-zero and executes an indirect jump to an unresolved target.
- `jump_to_unresolved_address_2`: Executes an indirect jump using a pointer from an unresolved jump table.
- `execute_indirect_jump`: Transfers control to an address retrieved from an unresolved jump table.
- `copy_bytes_backwards`: Copies a block of bytes backwards from a source to a destination before jumping to an unresolved target.
- `copy_bytes_backward_then_jump`: Copies a run of bytes backwards from the source area to the destination, counting the length down to minus one, then jumps to an unresolved target.
- `execute_unresolved_jump`: Executes an indirect jump to an unresolved target.
- `execute_indirect_jump_3`: Performs an indirect jump to an address stored in a memory location.
- `jump_to_unresolved_table`: Performs an indirect jump or copies a block of bytes backwards before jumping to an unresolved target.
- `decode_packed_literal_runs`: Reads bits from a packed longword stream and copies literal or back-referenced bytes from the source buffer, looping until the packed data ends.
- `decode_packed_copy_loop`: Shifts bits out of the packed stream and, depending on each bit, copies a literal byte or repeats an earlier output run.
- `decode_bits_into_forward_copy`: Reads a packed longword bit by bit and copies bytes forward from the source into the output buffer, stopping when the packed data is used up.
- `expand_back_references`: Copies bytes forward into the output buffer, then repeatedly copies earlier output bytes to fill the match lengths read from the packed stream.
- `store_byte_then_call_handler`: Stores one byte just before the address in register A2, then calls a handler through a global function pointer.
- `copy_backward_literal_runs`: Copies runs of bytes backwards from the packed source into the output, reading each run length from a stop marker.
- `copy_backward_byte_runs`: Copies runs of repeated byte values backwards into the output, counting each run down, and calls a handler when a stop marker is found.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:POPULOUS.II

- `start_populous_ii_executable`: Program entry of the Populous II executable.
- `busy_wait_word_countdown`: Decrements a 16-bit count in the data register until it wraps around, which gives a short timed delay.
- `return_empty_subroutine`: This routine is empty and performs no operations before returning.
- `unpack_shifted_flag_runs`: Reads run counts and flags from the packed stream and copies bytes backwards into the output.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bits_across_longwords`: Shifts bits out of a pair of longwords across word boundaries, reloading from memory as the bits run out, and returns the carried bit.
- `copy_icon_data`: copies a block of data from a source or a table to a destination.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF](Disk 1 of 2)[t +4 AFL].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_routine_address_and_trap`: Stores the address of the current routine into a global variable and executes a software interrupt trap.
- `initialize_system_state`: Performs a sequence of bitwise operations and assignments to a global variable, calls several other routines, and eventually calls a function pointer retrieved from a library structure.
- `update_blt_parameters`: Updates a global variable and certain memory-mapped or hardware-related registers with a new offset and a bit-shifted value.
- `fill_memory_with_patterns`: Writes alternating patterns of certain bit patterns to a range of memory addresses.
- `set_blt_registers`: Sets several hardware registers or memory-mapped registers to specific constant values and returns a value from a table.
- `set_blt_window_size`: Updates a global pointer and sets certain hardware registers or memory-mapped registers to a value derived from a table.
- `apply_offset_to_table`: Iterates through a table, adding a value from a global variable to each element and storing the result in another memory location.
- `initialize_bitmask_buffer`: Fills a large buffer with a constant, then applies a complex bit-manipulation pattern to clear or set bits in a way that creates a certain shape or pattern.
- `populate_data_table`: Writes a sequence of specific constant values into a table in memory.
- `copy_string_with_stride_2`: Copies a certain number of bytes from a source address to a destination address, using a fixed stride or offset, while updating several global memory locations with constant values.
- `scale_table_values`: Multiplies values in several different tables by a constant factor and initializes a series of memory-mapped registers with specific bit patterns.
- `copy_string_with_stride`: Copies a certain number of bytes from a source address to a destination address, using a fixed stride or offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_string_data`: Finds the maximum byte value in a sequence, uses it to calculate an offset into a data table, and copies strings into a global buffer.
- `reset_globals_to_constant`: Writes zero to four global variables and writes a constant value to a fifth global variable.
- `update_audio_registers_2`: Updates the low-byte or certain bits of the four audio channel low-address registers and the video or certain hardware-related register.
- `update_audio_registers`: Updates the low-byte or certain bits of the four audio channel low-address registers and the video or or certain hardware-related register.
- `update_entity_attribute_and_dma`: Updates an entity's properties or state by reading from a lookup table and writing to a structure, while potentially modifying a global variable and the DMA controller's enable bit.
- `update_position_with_limit`: Checks a value in a structure and either adds or subtracts a byte-sized value to a short, while optionally clamping the result to a limit and updating a global field.
- `update_coordinate_data`: Calculates a new value using bitwise operations and multiplication from a structure, updates a field in that structure, and stores the result in another field.
- `update_entity_position_and_bounds`: Updates a coordinate or attribute value by adding or subtracting a certain amount, while applying clamping or boundary checks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_external_calls`: Executes two library calls with negative or non-standard identifiers and contains eighteen instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_external_calls_2`: Executes two library calls with negative or non-standard identifiers and contains fourteen instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_external_calls_3`: Executes two library calls with negative or non-standard identifiers and contains sixteen instructions.
