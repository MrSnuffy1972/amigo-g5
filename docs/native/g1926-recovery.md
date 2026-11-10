# Game 1926 (indiana jones and the last crusade (1989)(lucasfilm games)(disk 1 of 3)[cr pna][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01926). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905360 top-level bytes in 49 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 158 of 158 named (0 author symbols), 158 of 158 described (33 by rule, 32 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 337 of 337 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2042 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2485 of 2485 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 682495 |
| code | 7950 |
| data | 193389 |
| padding | 10774 |
| unused | 10752 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:joker | 2 | 106 | 4 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:indy | 2 | 70 | 5 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:EndCLI | 2 | 117 | 2 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:paranoimia | 3 | 11 | 1 |
| icon.library | 10 | 1375 | 109 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:LoadWB | 12 | 538 | 30 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 258 | 6 |
| 98.LFL (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- indiana jones and the last crusade (1989)(lucasfilm games)(disk 1 of 3)[cr pna][a]/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:IndyCodes1.zoo: high-entropy data file (7.91 bits/byte), classified as packed data
- reference run failed (Karl 4 Nov 2026): run play-long-keys slid through memory (an address: 131 64 KB pages each with >= 29491 distinct PCs executed, 2146304 of its 2160997 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-mixed slid through memory (an address: 131 64 KB pages each with >= 29491 distinct PCs executed, 2146304 of its 2160997 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 131 64 KB pages each with >= 29491 distinct PCs executed, 2146304 of its 2160987 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 131 64 KB pages each with >= 29491 distinct PCs executed, 2146304 of its 2160987 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 131 64 KB pages each with >= 29491 distinct PCs executed, 2146304 of its 2160987 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 131 64 KB pages each with >= 29491 distinct PCs executed, 2146304 of its 2160987 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address: 237 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- indiana jones and the last crusade (1989)(lucasfilm games)(disk 1 of 3)[cr pna][a]/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:Rooms/98.LFL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- indiana jones and the last crusade (1989)(lucasfilm games)(disk 1 of 3)[cr pna][a]/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:paranoimia: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 2042 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `compare_and_copy_string` (indy): Compares a string against a global constant and, if they match, copies the source string into the location of the global constant. Kind: string comparison and overwrite check. Pass path: the second do-while loop (the copy loop) is executed, followed by the code immediately following the function call.

## Functions


### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:joker

- `program_entry_shim`: Program entry shim that calls one routine, stores a global pointer to itself, calls the routine again and returns a register value.
- `unpack_compressed_stream_to_memory`: Decodes a bit-packed compressed stream by copying back-references and literal bytes into a window, and expanding escape code 21 into repeated runs.
- `refill_bit_buffer_from_stream`: Fetches the next 32-bit word from the input stream, stores its low byte at the destination and returns the word shifted right by one with the top bit set.
- `skip_bits_from_buffer`: Discards a count of bits from the packed bit buffer by shifting it one bit at a time, refilling it from the input stream whenever it runs empty.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:indy

- `setup_marker_and_launch_program`: Copies a 42-entry block of code to low memory, writes a signature word into a global, then opens system libraries and jumps to a routine found through them.
- `compare_prefix_and_copy_string`: Checks that a stored marker word has the expected value, then compares a 13-byte prefix of the stored text with the input.
- `check_loader_marker`: Checks that a stored marker word has the expected value and returns without any other effect.
- `compare_and_copy_string`: Compares a string against a global constant and, if they match, copies the source string into the location of the global constant.
- `copy_one_byte_to_slot`: Copies one byte from the source pointer into a slot addressed through a register, then returns the unchanged result register.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:EndCLI

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_state_block_and_call`: When the destination pointer is set, copies a 16-longword block from a fixed offset in a state structure to that destination, then calls a supplied routine.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:paranoimia

- `stub_bad_instruction_data`: The decoded body is a stub: it copies the register set into fixed memory cells and then reaches unimplemented instructions, so no game logic is recovered.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit counter at a fixed offset in a structure and clears bit 3 of a flag byte at another offset.
- `decrement_refcount_run_cleanup`: Library code that decrements a 16-bit reference counter in a structure.
- `unlink_node_or_mark_pending`: Library code that, when a counter field is zero, removes a node from a doubly linked list and calls a library function through a stored pointer.
- `set_pending_flag_bit`: Library code that sets bit 3 of a flag byte in a structure and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_probe_result`: Library code that saves two argument values into a structure, calls a library function through a base pointer and stores its result in the structure.
- `copy_two_fields_to_globals`: Library code that copies two 32-bit fields from a structure at fixed offsets into two global cells.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `forward_to_library_routine`: Library code that calls one other routine and then returns.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `dispatch_icon_record_write`: This library routine acts as a wrapper that unconditionally calls a helper routine to write an icon record to a stream.
- `call_handler_pointer`: This library routine calls a routine via a pointer retrieved from data and then returns.
- `call_library_handler_3`: This library routine retrieves a handler pointer from a data structure and calls it.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `trigger_icon_initialization`: This library routine calls a single initialization function to set up icon data structures and then returns.
- `call_library_handler_2`: This library routine calls another routine and then returns.
- `execute_recursive_handler`: This library routine calls another routine and then returns.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_another_routine`: This routine calls another routine and then returns.
- `call_other_routine_2`: The library routine calls another routine and then returns.
- `store_arg_in_looked_up_record`: Library code that calls a function through a pointer taken from the first argument's structure, then stores the second argument into the returned structure at a fixed offset.
- `acquire_handle_with_fallback_release`: Library code that calls a helper to obtain a handle and, if it is non-zero, calls a second helper with it.
- `fill_record_from_buffer_and_link`: Obtains a record through a helper and, on success, copies a buffer of ten longwords and several single fields into it.
- `copy_fields_into_record_link`: Copies ten longwords and several single fields from a source block into a record at fixed offsets, then sets the back-link in the record referenced by one of its fields when that link is present.
- `return_zero_stub`: Empty library stub that immediately returns zero and does no other work.
- `copy_record_to_stack_and_call`: Copies ten longwords and several fields from a structure at an offset into a local buffer on the stack, prefixes the buffer with a fixed two-word header, then calls another routine with the first two arguments and a pointer to that buffer.
- `read_icon_record_from_stream`: Reads an icon record through library calls using a 260-byte scratch buffer and a fixed mode number, then checks a signature word and version field before filling the record's fields and reading its optional lists.
- `read_icon_record_fields`: Checks a signature word and version field, then reads the record's fields and optional lists through library calls.
- `read_icon_optional_lists`: Reads two fields into the record and then, for each optional list present at fixed offsets, reads its longword entries through library calls.
- `read_longword_list_and_field`: Reads longword entries into an array until the remaining byte count runs out, then reads one further value into a record field.
- `return_zero_after_call`: Calls one other routine and returns zero regardless of its result.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `write_icon_record_to_stream`: Writes an icon record to a stream through library calls: it allocates a 260-byte scratch buffer, writes the record's fields and optional lists with each list preceded by its byte size, then releases the buffer and returns the success flag.
- `write_icon_record_fields`: Writes an icon record's fields and optional lists to a stream through library calls, using the .info text the routine references.
- `write_icon_optional_lists`: Writes the optional lists of an icon record through library calls, each list preceded by its byte size, and returns the final success flag.
- `write_icon_pointer_list`: Measures a null-terminated list of pointers, then writes each entry's length-prefixed data through library calls, followed by an optional extra entry.
- `write_pointer_table_entries`: Walks a null-terminated table of pointers, writing each entry's byte size and data through library calls, then writes an optional extra entry.
- `iterate_linked_list_nodes`: Walks a linked list from a head pointer, calling a helper on each node until the list ends or the head sentinel is reached, then calls the helper once more on a remaining node if there is one.
- `iterate_node_chain_callback`: Walks a chain of nodes by following a link field, calling a callback on each node until the chain ends or a helper returns nothing, then calls the callback on a remaining node if present.
- `call_with_offset_argument`: Calls another routine with the same first argument and the second argument advanced by a fixed field offset, then returns.
- `allocate_record_with_setup`: Allocates a 168-byte block, sets a link field inside it, then calls a helper with the new block.
- `call_refill_helper`: Library code that calls a routine to decrement a counter in a structure and invokes a helper to refill it when the counter reaches zero.
- `return_zero_3`: Library code that returns zero.
- `push_pair_into_table`: Decrements a short counter in a structure and, when it reaches zero or below, calls a helper to refill it.
- `decrement_counter_and_store_pair`: Decrements a short counter through a pointer, then stores two register values into the table entry at the new counter index and returns one.
- `allocate_record_with_callback`: Allocates a 94-byte block, sets a sub-field, and calls helpers with it.
- `empty_return_routine`: Empty routine that returns immediately without doing any work.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `incremental_recursive_call`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `search_list_for_equals_value`: Searches a null-terminated list of pointers, passing each entry through a helper, and returns the text after the equals sign of the first matching entry, or zero if none matches.
- `scan_entries_for_equals_value`: Scans a null-terminated list of entries, calling a helper on each, and returns the text after the first equals sign found, or zero when the list ends.
- `find_equals_value_in_list`: Walks a null-terminated pointer list, calling a helper on each entry, and returns the text after the equals sign in the first entry that matches, or zero if the list ends.
- `check_entry_size_chain`: Walks a chain of entries, comparing each entry's size with a value from a helper and validating matches with a second helper.
- `find_matching_size_entry`: Iterates through a chain of entries from a start value, comparing each entry's size with a stored target size and validating matches with a helper.
- `scan_chain_for_size_match`: Scans a chain of entries, comparing each size with a target and validating matches with a helper.
- `validate_entry_chain`: Walks a chain of entries and calls helpers to validate each entry's size against a stored value.
- `match_entry_size_in_chain`: Repeatedly walks entries from a start value, comparing each entry's size with a target and validating matches with a helper.
- `parse_copy_count_from_text`: Parses a text field that may begin with a fixed prefix followed by a decimal number, storing the number and the remaining text into a record.
- `skip_decimal_marker_field`: Checks for a three-character marker, reads decimal digits after it, and checks an optional four-character suffix, then calls two helper routines to finish.
- `parse_decimal_count_text`: Reads a run of decimal digits from a text pointer, accumulating their value, then checks a four-character suffix when digits were read.
- `parse_decimal_string`: Library code that parses a decimal number from a string and performs string comparisons and library calls based on the result.
- `digit_value_or_minus_one`: Returns the numeric value of an ASCII digit character, or minus one when the character is not a decimal digit.
- `return_minus_one_stub`: Empty library stub that immediately returns minus one and does no other work.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `allocate_record_into_buffer`: Allocates a 96-byte record, stores a fixed size value in it and in the caller's buffer, copies the record into the buffer through a helper, and returns whether allocation succeeded.
- `return_zero`: This library routine is empty and simply returns zero to the caller.
- `check_helper_result_matches`: Calls a helper with three arguments and returns the value it produced when that equals the fourth argument, or zero otherwise.
- `tiny_move_rts`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `call_library_and_verify`: Library code that calls another routine and returns zero if the result does not match a provided parameter.
- `return_void`: The routine returns without performing any operations.
- `divide_unsigned_32bit`: Divides a 32-bit unsigned value by another, using a bit-by-bit restoring loop for large divisors and direct 16-bit division otherwise.
- `divide_unsigned_bit_loop`: Runs the sixteen-step restoring division loop on a 32-bit dividend, subtracting the divisor whenever the partial remainder is at least the divisor.
- `multiply_32_bit_integers`: Computes the low 32-bit product of two values from 16-bit partial products, and also returns a half-word swapped copy of the second operand.
- `invoke_table_entry`: Calls the routine whose address is stored at a fixed offset in the library's global data structure, then returns.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `measure_c_string_length`: Counts the characters of a null-terminated string and returns the count.
- `find_char_in_c_string`: Searches a null-terminated string for the first occurrence of a given character and returns a pointer to it, or zero if the terminator comes first.
- `return_zero_4`: This library routine is empty and always returns zero.
- `copy_c_string`: Copies a null-terminated string from the source pointer to the destination, including the terminator.
- `append_c_string`: Appends a null-terminated source string to the end of a destination string, including the terminator.
- `compare_c_strings_limited`: Compares two strings for up to a given count of characters.
- `return_one_stub`: Empty library stub that immediately returns one and does no other work.
- `scan_string_with_limit`: Steps through a string for up to a given count of characters, stopping at its terminator, and calls a helper routine when the terminator is reached.
- `call_and_return`: Library code that calls another routine and then returns.
- `copy_c_string_padded`: Copies a null-terminated string into a destination of a given size, stopping at the terminator and zero-filling the remaining bytes of the destination up to that size.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_list_head`: Initialises a list header in place: it stores the header's own address plus four in its first field, clears the second field, and points the third field back at the header.
- `process_entry_with_helpers`: Returns one for a null entry; otherwise it looks up or allocates related data, calls helpers to process the entry's pointer field, and returns the helper result.
- `walk_entries_checking_helpers`: Walks a linked list of entries through the link field, calling a helper on each entry and on its sized data block.
- `walk_entries_with_two_checks`: Walks a linked list of entries through the link field, requiring two helper calls to succeed on each entry.
- `return_zero_2`: The routine returns a zero value.
- `allocate_linked_entry_node`: Allocates a 20-byte entry, links it through helper calls, stores a derived value in one of its fields, and returns the entry, or minus one on failure.
- `return_negative_one`: The routine returns a value of all ones.
- `build_entry_chain`: Builds a chain of entries through helper calls, linking each new entry to the previous one, and returns the first entry or minus one when a helper fails.
- `traverse_icon_chain`: Library code that iterates through a linked structure, updating memory values and propagating changes until a termination condition or a zero return value is met.
- `process_icon_chain`: Library code that iterates through a structure, performing multiple library calls and updating memory values based on their results.
- `return_all_ones`: This library routine returns a value consisting of all bits set to one.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:LoadWB

- `parse_command_line_into_argv`: Program entry that finds the task's command-line string, opens the DOS and exec libraries, and allocates a block for the split arguments.
- `release_task_resources`: Stores a parameter into the task's control record, then conditionally closes a file handle, frees a memory block, closes a library and replies to a message through library calls.
- `launch_workbench_tool`: Opens the icon and intuition support libraries, decodes a text block by exclusive-ORing it with a fixed byte, allocates a 160-byte record and starts the requested workbench tool through library calls.
- `compare_c_strings`: Compares two null-terminated strings byte by byte, returning minus one when the first is less than the second, one when it is greater, and zero when they are equal.
- `build_list_from_table`: Walks a chain of table records, allocating an 8-byte node for each, copying a value from the record into it through a helper and linking the nodes together.
- `invoke_table_entry_2`: Calls the routine stored at a fixed offset in the library's global data structure, then returns.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `check_text_prefix_and_pad`: Compares a global text with the start of a buffer.
- `write_dot_padding_bytes`: Writes a dot followed by four spaces into five fixed positions of a buffer.
- `compare_global_text_prefix`: Compares a global text with the start of a buffer, advancing through the text until it ends or a mismatch occurs.
- `stub_return_only`: Contains only a return instruction, with no calls, memory access or hardware use, so it does no work.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_with_stack_slot`: Calls another routine with the two incoming arguments and a pointer to a local stack slot, then returns.
- `measure_stack_text_and_send`: Calls a library routine first, then measures a 140-byte local text buffer.
- `store_byte_at_pointer`: Stores one byte value through a pointer argument.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(Disk 1 of 3)[cr PNA][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_copper_list_and_dma`: Builds a copper list and a table of display values in memory, clears a block of memory and writes the custom chip registers to set up the copper, DMA and interrupt enables.
- `wait_and_program_blitter`: Swaps two stored values, waits until the blitter busy flag clears, then writes blitter registers with masks and a fixed control word for a copy operation.
- `program_blitter_row_entries`: Waits until the blitter is idle, writes mask and control values into its registers, then programs a sequence of row address entries computed from the inputs.
- `advance_cursor_copy_record`: Counts down a delay counter and, every seventh call, advances a cursor through a byte table to its next entry.
- `program_blitter_fill_setup`: Waits for the blitter to become idle, then writes zero and constant words to its registers and address values to its pointer fields, setting up a fixed blit.
- `update_wave_offsets_table`: Updates a table of positions with wrap-around steps, reversing their direction on a periodic counter.
