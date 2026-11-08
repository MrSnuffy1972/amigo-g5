# Game 1827 (dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01827). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (965472 top-level bytes in 53 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 25 of 25 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 626 of 626 named (0 author symbols), 626 of 626 described (78 by rule, 540 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3568 of 3568 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 23268 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 20129 of 20131 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 526951 |
| code | 63722 |
| data | 320688 |
| padding | 11615 |
| unused | 42496 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:Install | 7 | 45 | 2 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:DuneII | 13 | 251 | 6 |
| Startup | 2 | 913 | 20 |
| Format | 30 | 2009 | 76 |
| More | 19 | 2290 | 76 |
| Else | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Execute | 2 | 117 | 2 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| IconX | 12 | 806 | 37 |
| Disk-Validator | 1 | 386 | 1 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:DUNEint.EXE | 2 | 267 | 8 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:Add16k | 3 | 475 | 32 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:MakeDummy | 2 | 1275 | 25 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 144 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1633 | 53 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1831 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7031 | 218 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 262 | 11 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:DUNE.ENG (raw code file, load address unknown) | 1 | 0 | 0 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:DUNE.FRE (raw code file, load address unknown) | 1 | 0 | 0 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:DUNE.GER (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 72 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 388 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 40 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2355 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 205 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)/Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:DUNE.ENG: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)/Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:DUNE.FRE: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)/Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:DUNE.GER: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)/Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)/Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:DuneII: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 23268 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_unreachable_code_sequence` (More): Executes a sequence of instructions that calls a routine with a global variable, calls itself recursively, and then executes a function pointer stored in an unaffiliated register. Kind: sabotage path/invalid pointer trap. Pass path: The routine executes a call to an uninitialized or otherwise invalid pointer `unaff_A3` immediately after certain subroutines. In a successful protection check, the code would likely branch away or exit before reaching the dereferenced call.

## Functions


### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:Install

- `run_hdinstall_entry`: Runs a sequence of helper calls and a loop that calls a function, clears a word in a shared record, and swaps a table entry with a RAM variable if it matches a fixed constant.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:DuneII

- `run_game_entry`: Executes a sequence of calls including a loop that calls a function, clears a word in a shared record, and swaps a table entry with a RAM variable if it matches a specific constant.
- `decrement_word_until_negative_one`: Decrements a 16-bit register or local variable until it reaches negative one and then returns.
- `execute_complex_initialization`: Executes a series of calls and loops that modify memory by adding an offset to a value at a pointer, followed by a conditional call to an indirect function pointer.
- `copy_memory_block`: copies a block of memory from a source pointer to a destination pointer.
- `copy_string_bitset_reverse`: Performs a bit-reversed or bit-shuffled memory copy or bit-stream processing, where a bit-shift or bit-manipulation loop checks or shifts a global bit-flag while copying or decrementing memory pointers.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Startup

- `initialize_execution_environment`: Initializes global memory structures and pointer tables, calculates required buffer sizes, and allocates a temporary scratchpad or stack area.
- `execute_cleanup_sequence`: Performs a series of conditional calls to routines stored in a global structure and executes a cleanup sequence involving a linked list.
- `execute_cleanup_callbacks`: Iterates through a linked list of structures to invoke a sequence of function pointers.
- `return_without_effect_3`: the routine performs no observable work beyond returning to its caller.
- `return_without_effect_4`: The routine is empty and performs no observable work beyond returning to its caller.
- `free_linked_list_memory`: traverses a linked list and calls a function to free each element until the list is empty.
- `process_callback_status_alt`: Evaluates a status value by checking bits in a structure to determine whether to store a constant, execute a callback, or iterate through a linked list.
- `update_global_state`: updates a global variable or a pointer-based value based on the return value of another routine.
- `manage_startup_task_state`: Checks a status flag in a structure to determine if a specific initialization sequence has already occurred.
- `execute_cleanup_list`: Iterates through a linked list of entries and executes specific routines or cleanup sequences based on flags found within each entry.
- `parse_string_tokens`: Parses a source string to extract or transform quoted or escaped tokens, while populating a list of pointers to these tokens.
- `copy_string_and_set_pointer`: copies a sequence of bytes from a source to a local buffer and updates a global pointer with the buffer address.
- `initialize_startup_environment`: Copies a string from a global location to a local buffer, checks system task status, and updates a global pointer with a function address.
- `traverse_linked_list`: iterates through a linked list for a specified number of steps and returns a pointer to the current node.
- `copy_string_and_set_signal`: Copies a sequence of bytes from a source pointer to a local buffer and uses a library call to signal a task status.
- `find_byte_in_buffer_2`: searches for a specific byte value within a memory buffer of a given length and returns a pointer to the matching byte or null if not found.
- `append_string_to_buffer_2`: finds the null terminator of a destination string and copies a source string to that location, terminating the new string with a null byte.
- `copy_and_pad_string`: copies bytes from a source pointer to a destination pointer until a null byte is encountered, then fills the remaining bytes up to a remaining count with null bytes.
- `process_cleanup_list`: Iterates through a linked list to execute certain function pointers and executes a cleanup sequence or certain routine based on flags or bitmask checks.
- `do_nothing`: This routine is empty and performs no operations before returning.

### Format

- `parse_format_arguments`: Parses command line arguments to extract disk or volume names and formatting options.
- `update_and_cleanup_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `format_disk_from_arguments`: Parses command line arguments to identify the drive, volume name, and formatting options.
- `release_format_buffers_and_handles`: Releases the buffers and handles that the format routine allocated, calling a release routine for each field that is set, then calls another routine with its argument.
- `process_disk_error_status`: Performs a loop checking a return value against a constant, updates pointers or constants, and calls a routine to check if a value is within a certain range or matches a specific constant.
- `report_disk_errors`: Checks a condition and either calls several routines with a global variable or calls a routine with a specific constant and a global variable.
- `store_matched_table_byte`: Searches an eight-entry global byte table for the given value and records the matching entry in a field of a structure obtained from another routine.
- `find_drive_list_entry`: Walks a linked list of drive entries until one matches the given number or pointer, marks that entry, and returns 1.
- `calculate_string_length`: Returns the number of characters in a null-terminated string before its terminator.
- `copy_cstring_to_buffer`: Library code that copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers and writing each byte until a zero byte is encountered.
- `convert_char_to_uppercase`: Returns the character with lowercase ASCII letters converted to uppercase by subtracting 32.
- `compare_strings_ignoring_case`: Compares two null-terminated strings one character at a time after converting each to uppercase.
- `truncate_string_at_colon`: Replaces the first colon in a string with a null terminator, cutting the string short.
- `call_with_single_value_status`: Passes its first argument in a one-slot stack buffer to another routine with code 31 and length one.
- `create_block_from_source_words`: Library code that allocates a 68-byte object, copies up to seven words from a source array into it, and links it to a handle from another routine.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `forward_argument_to_library_stub`: Passes its one incoming argument, a library base pointer read from a global variable, and the address of its own stack argument to the library call stub at offset minus 132.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_tracks_and_verify`: Formats a disk track by track, building track data in a buffer and writing it through the disk device, then verifies the tracks.
- `write_value_to_output_handle`: Calls two output routines with fixed arguments using a handle taken from a structure, then passes a value from another routine through the same handle.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `rotate_and_mod_value`: Performs a bitwise rotation or a complex modulo-based calculation on a 32-bit value using a third or certain register value.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry_2`: Invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_pointer_2`: Stores a byte from a register into a memory location pointed to by a register.
- `allocate_indexed_record_block`: Attempts to allocate a 34-byte block using a library call, then populates certain fields with a provided parameter, a flag, a constant, and a slot index or pointer.
- `release_record_chain`: Recursively releases a linked chain of records, processing the linked record first when its link field is set.
- `forward_argument_with_code`: Passes its first argument to another routine along with the constant 48.
- `forward_single_argument`: Passes its single argument to another routine and returns.
- `initialize_system_structures`: Initializes several global pointers and variables by retrieving values from a memory block, potentially setting them to default values if the block is null, and performs a series of function calls to manage these structures.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a value from a stack slot or a negative error code.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `store_word_and_pointer_in_node`: Calls a library-backed allocation routine with a second argument, and if it returns a non-null block, stores a byte value, a word from the second argument, and the first argument pointer into fixed offsets of that block.
- `reset_node_and_release_it`: If the given pointer is non-null, it sets a byte field to 255 and clears two longword fields of the structure, then passes the pointer and a word field to another routine and returns its result.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### More

- `parse_command_line_arguments`: Parses command line arguments to identify drive, volume, and formatting options, while handling quoted strings and escape sequences.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from thetable to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `handle_disk_format_error_and_buffer`: Checks a status byte against specific error codes and, if certain conditions are met, clears a memory buffer, copies a value from a table, and performs a sequence of updates to a structure and a global pointer.
- `execute_unreachable_code_sequence`: Executes a sequence of instructions that calls a routine with a global variable, calls itself recursively, and then executes a function pointer stored in an unaffiliated register.
- `initialize_entity_state`: Initializes fields in a structure and invokes another routine with values or pointers from that structure, conditionally calling further routines or looping based on certain structure fields.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from an input pointer, stores the second result in a field of the pointed-to structure, and increments a short at an offset from a pointer in that structure if a flag field is zero.
- `parse_buffer_content`: Uses another routine to check a buffer, updates a size field, and scans for a newline or form-feed character while potentially modifying a flag or string content.
- `process_entity_list`: Determines a loop limit based on certain fields in a passed structure and iterates to call another routine using a pointer to a table and a global variable.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Runs a sequence of calls to another routine using arguments from a parameter block and a unable to determine purpose, returning the result of one of those calls.
- `check_flag_and_call`: Copies a null-terminated string and converts it to uppercase, then checks a short value at an offset from a pointer to conditionally call another routine with one argument.
- `draw_scroll_window_bounds`: Calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: Calls a drawing routine with a zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `draw_text_line`: Uses a handler pointer to call drawing primitives while iterating over a text buffer, tracking line count and stopping if a certain character or flag is encountered.
- `dispatch_recursive_pointer`: Calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition or partial way is unable to be met, or if a certain field is non-zero, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `recursively_adjust_structure_bounds`: Recursively calls another routine if a certain field is non-zero and a certain boundary check fails, otherwise copies a field, calls that routine with a new offset and a size-derived parameter, and updates a field with the returned value if positive while resetting a size field and copying a pointer.
- `adjust_window_bounds`: Calculates a target position using a provided offset and checks it against a structure's boundary limits.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string_2`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_uppercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated string, handling an optional leading plus or minus sign.
- `call_library_function_with_mask`: Checks if a pointer and a secondary pointer's target are non-null, then calls a library function using a global base address and a mask.
- `process_message`: Recursively processes a structure by invoking another routine on certain fields and string arguments, then conditionally updates structure fields and invokes further routines.
- `build_message_buffer`: The routine obtains a pointer from another routine and then calls another routine with that pointer, a constant, a stack-allocated buffer containing the input parameter, and a size of one.
- `copy_entity_data`: Obtains a handle from another routine and, if valid, reads a value from that handle using a small stack buffer and copies two fields from the handle into a provided structure before releasing the handle.
- `initialize_object_from_template`: Allocates a new object, initializes its fields with a template and source data, and copies a block of words from a source address into the new object.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_function_via_pointer`: Calls a function using a pointer retrieved from a memory location offset from a global address.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_switch_handler`: Executes a function pointer located at an offset from a global pointer.
- `execute_dispatch_handler`: Executes a function pointer located at a different offset from a global pointer.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `execute_allocmem_handler`: Executes a function pointer located at a third offset from a global pointer.
- `execute_allocabs_handler`: Executes a function pointer located at a fourth offset from a global pointer.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `execute_library_handler_one`: Executes a function pointer located at an offset from a global pointer using a library call.
- `execute_library_handler_two`: Executes a function pointer located at a different offset from a global pointer using a library call.
- `execute_library_handler_three`: Executes a function pointer located at a third offset from a global pointer using a library call.
- `execute_library_handler_four`: Executes a function pointer located at a fourth offset from a global pointer using a library call.
- `execute_library_handler_five`: Executes a function pointer located at a fifth offset from a global pointer using a library call.
- `execute_library_handler_six`: Executes a function pointer located at a sixth offset from a global pointer using a library call.
- `call_indirect_handler`: Calls a function pointer stored at an offset from a global pointer.
- `execute_indirect_function_16`: loads a function pointer from a global table using a constant offset and jumps to it.
- `execute_indirect_function_17`: loads a function pointer from a global table using a fixed offset and executes it.
- `execute_indirect_function_18`: loads a function pointer from a fixed offset within a global table and executes it.
- `execute_indirect_function_19`: loads a function pointer from a memory-offset relative to a global pointer and calls it.
- `execute_indirect_function_20`: loads a function pointer from a global table with a fixed offset and jumps to it.
- `execute_indirect_function_21`: loads a function pointer from a fixed offset within a global table and jumps to that address.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `extract_byte_from_call`: Calls another routine with a pointer to a small stack buffer and a constant, returning a byte from that buffer if the call returns certain a value, otherwise returning negative one.
- `call_global_and_checksum_buffer_3`: Invokes a function pointer from a global variable, then calculates a rolling checksum on a stack buffer and passes the buffer and checksum to another routine if the inverted checksum is non-zero.
- `store_register_to_memory_2`: Stores a byte from a register into a memory location pointed to by an untracked pointer.
- `modular_inverse_32bit`: Performs a 32-bit or 16-bit modular reduction or modulo operation using repeated doubling and subtraction or division.
- `multiply_32_bit_integers_2`: Computes a 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper routine and returns.
- `call_and_negate_on_flag`: Calls a helper routine and returns its first result, or returns the negation of that result if a second register contains a non-zero value.
- `invoke_table_entry_3`: Retrieves a function pointer from a fixed offset within a global data structure and executes it.
- `store_register_to_pointer`: copies a byte from a register into a memory location pointed to by a register.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Attempts to allocate a message buffer and, if successful, populates its fields with parameters and a value from another routine before dispatching it.
- `delete_port_logic`: Removes a port from a global list if a certain field is non-zero, updates status fields, and executes indirect functions or library handlers.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain_2`: Stores a pointer and an incremented version of that pointer into a memory block, followed by a zeroed word and the original pointer.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_block`: Copies a fixed number of words from a source address to a destination address on the stack if the destination pointer is not null.

### Echo

- `reallocate_and_copy_table_entry`: Calculates the maximum value in a linked list of table entries, updates a table entry with a new size and a sequence of incrementing values, and then copies a block of data from a source to a destination if a pointer is non-null.
- `copy_stack_block_2`: Copies a fixed number of words from a source address to a destination address on thestack if the destination pointer is not null.

### Execute

- `update_table_and_copy_stack`: Calculates the maximum value in a linked list or table, extends that table with an incrementing sequence, and performs a conditional block copy of stack-based data or a secondary table into a destination memory area.
- `copy_install_data_block`: copies a block of sixteen-bit words from a memory offset to the stack and then calls another routine.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `reallocate_and_populate_table`: Iterates through a linked list to find a maximum value, then expands a memory table or buffer by writing an incrementing pattern.
- `copy_install_data_to_stack`: copies a fixed number of words from a memory-resident table into the current stack frame before calling another routine.

### IconX

- `initialize_program_environment`: Initializes the game's internal state by parsing command line arguments and setting up required memory structures.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and a given name, including a wait command and an endcli redirection, after checking for a leading dot and checking if a name exists.
- `install_library_functions`: Creates and installs a library from a descriptor by setting up the library structure and its function table, returning zero if no descriptor is provided.
- `scan_ram_entries_for_names`: Allocates a buffer and walks a linked list of entries, copying certain entry names into the buffer and calling helper routines or performing string replacements.
- `concatenate_strings_to_buffer`: Copies a source string into a destination buffer and appends a second string onto it, then passes the resulting buffer and the new length to another routine.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: Library code that copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the null terminator of a string and then calls another routine passing that position and a second argument.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `execute_callback_from_table`: Executes a function pointer retrieved from an offset within a global structure.
- `execute_indirect_function`: Loads a function pointer from a memory location and jumps to it.
- `execute_indirect_function_2`: loads a function pointer from a specific offset within a global table and executes it.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_indirect_function_3`: loads a function pointer from a table and executes it.
- `call_library_vector_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `execute_function_pointer`: loads a function pointer from a global variable and jumps to it.
- `execute_indirect_function_pointer`: loads a function pointer from a memory location and jumps to it, using an offset relative to a base address.
- `execute_indirect_function_4`: loads a function pointer from a global table and executes it.
- `call_indirect_function`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_5`: Loads a function pointer from a location relative to a global pointer and jumps to it.
- `execute_indirect_function_6`: loads a function pointer from a global table and jumps to it.
- `recurse_with_stack_slot_2`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `execute_indirect_function_7`: loads a function pointer from a table using a constant offset and calls that routine.
- `execute_indirect_function_8`: loads a function pointer from a memory location offset by a constant and executes it.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `execute_indirect_function_9`: fetches a function pointer from a global table using a constant offset and executes it.
- `execute_library_routine_d`: loads a function pointer from a global unable to be determined and jumps to it.
- `execute_indirect_call`: Loads a function pointer from a global variable offset and jumps to it.
- `execute_indirect_call_2`: Retrieves a pointer from a global variable at a different offset and executes that code.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer from a global variable and computes a rolling checksum over a stack buffer, passing the buffer and checksum to another routine if the inverted checksum is non-zero.
- `store_register_to_memory`: Stores the contents of a register into a memory location pointed to by another register.
- `invoke_table_entry`: Invokes a function pointer retrieved from an offset within a global data structure.
- `store_byte_to_pointer`: stores a byte from a data register into a memory location pointed to by an address register.

### Disk-Validator

- `format_string_and_initialize_buffer`: Converts a numeric value to an ASCII string and stores it in a buffer, then initializes a memory range with a specific bitmask or value.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:DUNEint.EXE

- `process_amigados_hunk_relocation`: Parses executable hunk type codes to perform memory relocation and patch jump or pointer addresses.
- `decrement_loop_counter`: Executes a loop that decrements a register or variable until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_shift_memory`: Performs a loop that copies bytes from one memory location to another while checking or shifting bits or pointer values.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `perform_bit_rotation_loop`: Performs a loop that manipulates and shifts bits or values, updating a register or memory based on certain conditions.
- `execute_trap_instruction`: Executes a software interrupt trap instruction.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:Add16k

- `process_slot_record_stack`: Clears a block of memory, updates a global variable and a stack pointer, and then recursively traverses a linked-list-like structure or tree if certain non-zero return values are encountered.
- `open_libraries_and_cleanup`: Attempts to open certain libraries and, if successful, updates library bases and calls drawing or cleanup routines for each library.
- `clear_slot_record_store_stack`: Clears forty-two longwords of a per-slot record, stores the stack position and a global value into it, then calls another routine.
- `open_dos_and_mathffp_libraries`: Attempts to open the DOS and mathffp libraries, storing their bases in global variables and proceeding to initialize or clean up resources based on whether the library or its certain offsets are valid.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `close_stored_libraries_and_resources`: Runs a ten-step wait, closes any non-zero stored library bases, and releases stored resources using their respective cleanup routines before returning a given value.
- `append_string_to_buffer_3`: Finds the end of a destination string and copies a source string into that location up to a maximum number of bytes.
- `append_string_with_count`: Walks to the end of a destination string and appends bytes from a source string until a null terminator or a maximum count is reached, then null-terminates the destination.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `compute_string_length`: Walks forward through a zero-terminated string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies up to a certain number of bytes from a source to a destination, stopping if a zero byte is found, and fills any remaining bytes up to the count with zeros.
- `lookup_and_call_function`: Checks if an index is within a valid range and if a pointer in a table is non-null, then calls the function at that pointer if certain flags are absent and clears the pointer.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_function`: Loads a function pointer from a library ort and calls it.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_indirect_library_function`: Retrieves a function pointer from an offset within a library base and executes it.
- `call_library_function_indirect`: Retrieves a function pointer from a library ort and calls it.
- `execute_indirect_function_10`: Loads a function pointer from a memory location calculated using an offset and a base address, then branches to that address.
- `execute_indirect_function_11`: loads a function pointer from a nested table lookup and executes it.
- `call_indirect_function_2`: Performs an indirect call to a function pointer retrieved from a table using a calculated offset.
- `execute_indirect_function_12`: Loads a function pointer from a table using an offset from a global pointer and executes it.
- `execute_indirect_function_13`: fetches a function pointer from a table using an offset and executes it.
- `execute_indirect_table_function`: Loads a function pointer from an offset within a table and executes it.
- `dereference_function_pointer`: loads a function pointer from a table or structure and executes it.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `execute_indirect_function_14`: Loads a function pointer from a table and executes it.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `execute_indirect_function_15`: loads a function pointer from a table located via a global pointer and executes it.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:MakeDummy

- `initialize_engine_subsystem`: Initializes a global structure, calculates a required memory buffer size, and allocates memory via a system call.
- `store_value_and_call_routines`: Stores a passed value at a fixed offset from a computed base pointer, then conditionally decrements a counter and invokes routines from various tables depending on the state of certain fields.
- `process_callback_list`: Iterates through a linked list of structures to execute function pointers and perform conditional logic based on bitmask checks or flag verification.
- `return_without_effect`: The routine immediately returns to its caller without performing any observable work.
- `return_without_effect_2`: The routine executes a series of instructions that result in no observable side effects before returning to the caller.
- `drain_queue_list`: It walks a doubly linked list by following forward links, invoking a routine for each node until it reaches the end.
- `allocate_and_store_base_pointer`: Checks a flag bit in a structure and, when unset, calls a routine to allocate an aligned memory block.
- `process_callback_status`: Reads status bytes from a structure and branches on certain bits to either store a constant, execute a callback, or walk a linked list of callbacks.
- `update_state_and_calculate_parameter`: updates a global state or flag and calculates a new parameter value based on the return value of a called routine and an input parameter.
- `update_status_and_dispatch_callback`: Checks a status value and, if zero, writes a constant to a global field; otherwise, it clears a field and executes a callback function retrieved from a table, potentially updating a secondary field or executing a secondary callback if a certain error or status code is returned.
- `iterate_and_close_library`: Walks a linked list of entries and executes a routine for each entry, potentially calling a cleanup sequence or closing a library.
- `parse_file_into_tokens`: Parses a text buffer into an array of whitespace-separated tokens, handling quoted strings with backslash-style escape sequences for newlines and other characters, and prepares a linked list of callback-related structures or pointers.
- `copy_and_store_buffer`: Copies a certain number of bytes from a source pointer to a local stack buffer and updates a global pointer with the stack address.
- `copy_and_dispatch_routine`: Copies a bounded number of bytes from a source into a stack buffer, null-terminates it, and then calls a sequence of routines that branch on the contents of a structure and store a callback address into a table.
- `resolve_index_from_list`: Walks a singly linked list of pointers, decrementing a counter until it reaches zero, and returns the matching node or a null pointer when the bounds are exceeded.
- `process_string_and_execute_callbacks`: Copies a string from a pointer into a stack buffer and calls a sequence of routines that branch based on certain conditions or flags.
- `find_byte_in_buffer`: The routine searches for a specific byte value within a memory buffer of a given length.
- `append_string_to_buffer`: The routine appends a source string to the end of a destination string.
- `copy_bytes_with_termination`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `allocate_aligned_memory_block`: Aligns the requested size to a four-byte boundary and searches a linked list of free memory blocks for a suitable fit.
- `execute_callback_list`: Iterates through a linked list of structures to execute certain function pointers and potentially calls another routine if certain bitmask conditions or pointer comparisons are met.
- `copy_string_with_limits`: Walks a null-terminated string and copies characters into a destination buffer while monitoring a maximum length counter and checking a status flag.
- `store_byte_and_update_state`: Examines status bytes in a structure and branches on their bits to set flags, reset counters, or return early.
- `update_file_node_state`: Iterates a linked list to aggregate a status flag, then resets or updates a node's counters and status bytes based on helper return values and returns a status code.
- `do_nothing_return`: The routine performs no operations and simply returns to its caller.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `format_string_buffer`: Constructs a formatted string by copying data from a source into a destination buffer, inserting specific control characters and null terminators.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `sort_and_recurse_values`: Determines the minimum and maximum values from a set of inputs and performs multiple recursive calls using various combinations of these values.
- `update_indexed_table_entry`: Calculates an index using a modulo operation and updates a global variable with values retrieved from a table at that index.
- `execute_conditional_callbacks`: Performs a series of conditional calls to functions stored in a table or pointed to by global variables based on various status checks.
- `clear_global_variables`: Sets a global variable to zero and several other specific constant values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_struct_and_toggle_flag`: It initializes several fields of a structure with fixed values, then toggles and clears specific bits of a flag byte.
- `call_subroutine`: This routine calls another subroutine and then returns.
- `perform_fixed_point_division`: Performs division on 32-bit values, handling both integer and fixed-point arithmetic logic through bit shifting and loops.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `initialize_object_structure`: Initializes a data structure by calling a function pointer from an offset within a provided pointer, then populates several specific fields with fixed values.
- `initialize_audio_subsystem`: Initializes a series of data structures and tables by iterating through memory locations to set specific byte and word values.
- `clear_structure_flags`: Clears specific bits in a global variable and iterates through a table to call a subroutine if a bit is set, then clears bytes in that table.
- `initialize_and_clear_structures`: Iterates through a table to call a subroutine if a value is non-zero, then iterates through another table to clear specific bits and set a byte to a fixed value in each entry.
- `store_subroutine_address`: Stores a value from a register into a global variable and then stores the return address of the current routine into another global variable.
- `update_flag_from_input`: Reads the state of a hardware input register and sets or clears a specific bit in a global variable based on the result.
- `check_global_flag_bit`: Checks a flag bit in a global variable and returns one when the bit is clear, otherwise returns zero.
- `traverse_linked_list_of_structures`: Iterates through a list of structures, skipping entries that match a specific pattern until a null terminator is found.
- `execute_and_wait_for_pointer`: Assigns a parameter to a global variable, calls a function via an offset from that variable, and enters a loop until the variable becomes zero.
- `initialize_system_state`: Performs a sequence of operations including clearing bits in a global variable, setting multiple bytes to a specific value, and updating a bit in a global variable.
- `update_offset_value`: If a parameter is less than a global variable, it clears bits in that variable and adds a scaled version of the parameter to it.
- `update_object_data`: Calls two functions via pointers from a global structure and updates a target structure with values from registers or a pointer.
- `clear_linked_list`: Traverses a linked list starting from an offset in a table, calling a function for each element while clearing the list entries.
- `reset_table_entries`: Clears specific entries in a table based on an index and calls a function to reset a value if a certain field is non-zero.
- `process_linked_list`: Performs two loops of a subroutine call, then traverses a linked list to call a function pointer stored in each element before clearing the list entries and resetting a global variable.
- `update_linked_list_objects`: Iterates through a linked list of objects to update their data and status flags.
- `search_pairs_for_value`: Scans an array of pairs, returning a null pointer when it hits an empty slot or the matching pair when found.
- `call_global_function`: Checks if a global variable is non-zero and, if so, calls a function pointer located at a fixed offset from that variable.
- `execute_subroutine_calls`: Retrieves a pointer from a memory offset and calls two functions located at specific offsets within that pointed-to structure.
- `execute_indirect_function_pointers`: Fetches a pointer from a memory offset and executes two functions located at specific offsets within the pointed-to structure.
- `call_indirect_function_3`: Retrieves a function pointer from a memory location offset from a base address and executes it.
- `nop_placeholder`: This routine is empty and performs no operations.
- `load_resource_assets`: Attempts to load multiple game data files including shapes, text, and sound assets into memory.
- `load_and_process_font_data`: Allocates memory to load font data from a file and processes it through a series of state-dependent loops.
- `negate_and_trap`: Negates the value pointed to by a memory location and executes a software interrupt.
- `route_record_index`: Compares a two-byte value against a series of constants and branches to a shared block for matching cases while falling through to a final call for the last accepted value.
- `update_global_value`: Modifies a global variable based on its current value through several conditional checks.
- `copy_data_to_table`: Copies a block of data from a provided pointer to a fixed destination table and then calls another routine twice with calculated arguments.
- `select_priority_entity`: Iterates through a linked list of entities to identify and select specific objects based on bitmask flags and priority criteria.
- `execute_empty_recursion`: This routine performs infinite self-recursion by calling itself repeatedly without performing any operations or returning.
- `execute_recursive_calls`: This routine performs three consecutive, identical calls to itself before returning.
- `execute_recursive_calls_2`: This routine calls itself three times in succession before returning to its caller.
- `execute_recursive_calls_3`: This routine performs three consecutive calls to itself before returning.
- `execute_recursive_calls_4`: The routine performs three consecutive calls to itself before returning.
- `empty_routine`: This routine contains no valid instructions and performs no operations.
- `perform_empty_recursion`: This routine calls itself repeatedly without performing any operations or modifying any state.
- `execute_infinite_recursion`: The routine calls itself repeatedly without a termination condition, resulting in an infinite loop of nested calls.
- `execute_empty_recursion_2`: This routine performs infinite self-recursion by calling itself repeatedly without any instructions or state changes.
- `execute_empty_recursion_3`: This routine performs an infinite recursive loop by calling itself repeatedly without performing any operations or returning.
- `execute_empty_call`: This routine performs no operations and immediately returns to its caller.
- `call_recursive_routines`: Calls itself three times before returning.
- `execute_empty_recursion_4`: The routine performs an infinite recursive loop by calling itself repeatedly without performing any operations or returning.
- `perform_recursive_calls`: This routine calls itself three times in succession before returning.
- `nop_padding_sequence`: This routine consists of a single instruction that serves as a gap or padding within the executable segment.
- `null_routine`: This routine is empty and contains no executable instructions.
- `process_resource_object_lifecycle`: Iterates through a collection of objects to execute specific function pointers based on a state flag.
- `execute_empty_recursion_5`: The routine performs an infinite recursive call to itself.
- `return_from_lifecycle_process`: This routine is empty and serves as a return point for a function managing resource object lifecycles.
- `reset_indexed_values`: Clears two words at a specific offset within a table indexed by a global variable and then resets that global variable to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `skip_instruction`: This routine consists of a single instruction that advances the program counter.
- `call_other_routines`: Executes another routine three consecutive times.
- `nop_padding_sequence_2`: This routine consists of a single instruction that performs no operation, serving as a gap or padding within the executable segment.
- `copy_data_stream`: Copies bytes from a source buffer to a destination buffer using a loop that processes variable-sized chunks.
- `nop_padding_sequence_3`: This segment consists of an empty sequence of instructions used for data alignment or padding within the executable.
- `perform_infinite_recursion`: This routine calls itself indefinitely without performing any operations or returning, resulting in a stack overflow.
- `nop_padding_sequence_4`: This routine is empty and consists of a sequence of no-operation instructions used as padding within the executable segment.
- `update_bitfields`: modifies specific bits within three different byte locations relative to a pointer based on the state of a carry flag and a 16-bit value.
- `null_routine_2`: The routine is empty and contains no executable instructions.
- `null_routine_3`: This routine is empty and contains no valid instructions.
- `execute_recursive_calls_8`: Performs a series of nested recursive calls to another routine, checking for zero return values before proceeding.
- `process_intro_string`: Parses a string containing escape sequences and copies characters to a destination.
- `execute_repeated_calls`: Performs a series of recursive calls and then enters a loop that iterates a fixed number of times.
- `initialize_engine_subsystems`: Copies a block of data from a fixed memory location to a global buffer and executes a series of conditional checks and function calls.
- `initialize_intro_sequence`: Loads and processes introductory text and multimedia data by checking specific flags and conditional memory offsets.
- `process_intro_sequence`: Copies a string from a fixed location to a global buffer and enters a loop that waits for a specific condition to be met via a subroutine call.
- `execute_intro_script`: Manipulates bitfields, copies a string to a global variable, and enters a loop that repeatedly calls a subroutine to check for a non-zero return value.
- `run_intro_subroutine`: Updates bitfields, copies a string to a global buffer, and executes a loop that checks for a condition while occasionally calling a subroutine with different parameters based on a counter.
- `validate_resource_availability`: Performs a series of conditional checks by querying a status function and verifying specific data offsets.
- `copy_and_loop_input`: Copies a block of data from a fixed source to a global variable and then enters a loop that repeatedly calls another routine to check for input.
- `execute_table_calls`: Iterates through a loop to call functions via a table of pointers and subsequently performs data copying and input polling.
- `copy_data_and_poll`: Copies a sequence of bytes to a global variable and enters a loop that monitors input while occasionally calling another routine based on a counter.
- `load_and_parse_intro_data`: Copies a block of data from a table into a global buffer and initializes a memory range with a specific value.
- `parse_and_execute_sequence`: Parses a string and copies characters to a destination, while performing recursive calls and updating bitfields based on a global variable and a loop.
- `loop_until_flag_set`: Assigns a value to a global variable and enters a loop that repeatedly calls another routine until a specific condition is met and a flag is set.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `execute_empty_calls`: This routine performs three consecutive calls to itself before returning.
- `process_string_data`: sets a bit in a byte at an offset from a pointer, then performs a series of memory copies and calls several functions via a global pointer and offsets.
- `empty_routine_2`: This routine contains no valid instructions.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `empty_routine_3`: This routine contains no executable instructions.
- `execute_recursive_calls_5`: This routine performs three consecutive recursive calls to itself before returning.
- `perform_recursive_calls_2`: This routine calls itself three times before returning.
- `null_routine_4`: The routine is empty and contains no valid instructions.
- `empty_routine_5`: The routine is empty and performs no operations.
- `enter_infinite_loop`: The routine enters an infinite loop that performs no actions.
- `call_function_repeatedly`: The routine iterates a number of times based on a local variable and calls a function pointer retrieved from a table twice per iteration.
- `empty_routine_4`: The routine contains no executable instructions.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `process_escape_sequence`: Parses a string and copies characters to a destination, stopping if an escape character is encountered.
- `perform_recursive_calls_3`: The routine calls itself three times before returning to its caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_padding_instructions`: This routine consists of two instructions that perform no operation and do not modify any registers or memory.
- `process_deferred_value`: Flushes a pending value held in a global slot by running another routine on it, then processes a passed value, storing the resulting deferred value back into that global slot.
- `perform_recursive_call`: Executes a series of nested calls to another routine using values derived from return results.
- `copy_and_call_subroutines`: Copies a sequence of bytes from a pointer to a fixed destination and then calls another routine twice with calculated parameters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `swap_and_clear_buffer`: Swaps the contents of a global word with a parameter value and returns the previous value, then reads a length from a global variable and a pointer from the adjacent location and clears that many consecutive bytes.
- `clear_large_memory_blocks`: Fills large blocks of memory with zero bytes by iterating through memory in fixed-size increments and clearing any remaining bytes.
- `fill_memory_with_zeros`: Fills large blocks of memory with zero bytes using a loop that processes memory in fixed-size increments.
- `fill_memory_block`: Fills a block of memory with a specific byte for a specified number of iterations.
- `swap_global_word`: Swaps the contents of a global word with a parameter value and returns the previous value.
- `swap_flag_and_set_offsets`: Swaps a global flag value with a passed-in value and restores the previous one on return, then stores several word-sized fields into a structure indexed by a parameter.
- `copy_and_call_routines`: Copies forty bytes one at a time from one pointer to another, then invokes two routines through function pointers stored in a structure.
- `copy_fixed_bytes`: Copies forty bytes one at a time from one pointer to another.
- `initialize_from_pointer`: Initializes several global fields from a pointer, or resets them to defaults when the pointer is null, returning the previous stored pointer.
- `set_flag_bit`: Sets a bit in a global byte and returns the previous value of that bit.
- `clear_bit_and_return_status`: Clears a specific bit in a global byte and returns a value indicating whether that bit was set prior to the operation.
- `lookup_and_adjust_value`: Looks up an entry in a table using a passed value and, depending on the result, either adjusts a value taken from the stack or clears a field through a pointer.
- `swap_flag_and_process_game_state`: It swaps a global flag with a passed value, copies forty bytes from one pointer to another, and initializes global fields from a pointer or resets them to defaults.
- `swap_flag_and_set_offsets_2`: Swaps a global flag value with a passed-in value and restores the previous one on return, then stores several word-sized fields into a structure and clears two more fields.
- `find_max_fixed_point_index`: Repeatedly evaluates three fixed-point products, tracks the largest product seen with its index, and stops when a table entry holds a sentinel value or the index exceeds a limit.
- `append_string_to_buffer_4`: Finds the null terminator of a destination buffer and copies bytes from a source pointer into that location until a null terminator is reached.
- `traverse_linked_list_and_call`: Walks a global linked list of structures, testing two flag bits in a status byte and computing the difference between two integer fields before calling another routine.
- `multiply_fixed_point_values`: Calculates the product of two 32-bit fixed-point values using high and low word multiplication and bit shifting.
- `run_and_store_result`: Sets a local counter to one, runs a routine with a passed value, splits the high half of another result into a local slot, and calls a routine with several of these values.
- `store_indirect_value`: Copies a 32-bit value from a location four bytes before a register into the address held in another register, then returns.
- `reset_global_variable`: Sets a global variable to one and then immediately sets it to zero.
- `swap_flag_and_compute_offsets`: Swaps a global flag value with a passed-in value and restores the previous one on return.
- `execute_recursive_calls_7`: Calls another routine twice, returning the value obtained from the first call.
- `reset_dispatched_routine_state`: Reads an index from a global structure, calls a routine with a computed pointer and adjusted value, then clears several fields in that structure.
- `call_subroutines_recursively`: Calls itself recursively with its own address as an argument.
- `process_compressed_text_stream`: Runs a small state machine over a compressed text buffer, decoding escape sequences such as percent and 's' markers while advancing a per-object counter.
- `execute_recursive_graphics_logic`: Checks a global pointer, executes a routine through it, and then performs recursive calls using values derived from a global structure.
- `advance_and_truncate_string`: Decrements a per-object counter and, upon reaching zero, scans a string backward to find a space for null-termination.
- `fetch_next_compressed_byte`: Fetches the next byte from a chain of pending buffers, advancing through them until a value is found.
- `fetch_pending_compressed_byte`: Loads a byte from a buffer into a global variable, swaps the current value with the buffered value, and resets the buffer.
- `zero_global_variable`: Sets a global variable to zero twice.
- `forward_arguments_three`: Forwards its three arguments to another routine and then returns.
- `call_copy_buffer_interleave_2`: Calls a routine that copies five consecutive bytes from a source buffer into five scattered positions of a destination array, repeating this process for each row of a two-dimensional structure.
- `forward_arguments_to_interleave`: Passes three arguments to a routine that interleaves data from scattered positions into a consecutive buffer.
- `clear_byte_buffer`: Reads a length from a global variable and a pointer from the adjacent location, then clears that many consecutive bytes starting at the pointer.
- `clip_line_to_bounds`: Clamps a line segment's endpoints to a rectangular region using Cohen-Sutherland-style boundary flags, computing the intersection point where the segment crosses each edge.
- `check_bounds_flags`: Compares two coordinate fields against stored limits and accumulates flag bits indicating whether each coordinate falls below or above those limits.
- `store_bounds_values`: Stores four values into four consecutive global variables, pairing the low and high bytes of two input values into separate storage slots.
- `clamp_and_draw_lines`: Clamps two coordinate pairs so the second lies at or below the first, then issues four line-drawing calls with the resulting corner values after invoking two routines through a pointer in a global structure.
- `execute_graphics_calls`: Retrieves a pointer from a global variable and executes three function calls at specific offsets from that pointer.
- `register_interrupt_server`: It registers a routine with the operating system by calling a library function with a pointer taken from a global structure, then returns.
- `reset_global_variable_2`: Sets a global variable to zero through a series of assignments.
- `evaluate_conditional_flags`: Runs a sequence of conditional checks that call other routines and combine their results into a byte flag, returning that flag.
- `lookup_routine_pointer`: Multiplies an index by four and returns the pointer stored at that offset in a table of routine pointers.
- `cleanup_display_context_2`: Conditionally invokes a callback when a global flag is set, then resets a structure by writing a constant at one offset, a zero at another offset, and a pointer at a third offset.
- `execute_recursive_calls_6`: Calls itself three times in succession.
- `clear_global_pointer`: Calls two routines and then checks a global variable to either call a routine at an offset or return zero before clearing the global variable.
- `call_rem_int_server_2`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `process_string_handlers`: When a flag is clear, it runs two setup routines and then repeatedly walks a null-terminated string, calling a handler for each segment and advancing the pointer past each chunk.
- `accumulate_offset_differences`: Accumulates differences between pairs of global values depending on which of two bit flags are set in the argument, then returns the total.
- `compare_and_update_memory`: It checks available memory via a library call and updates a global variable based on comparisons between two input parameters.
- `return_zero_value`: The routine clears a register and returns to the caller.
- `update_range_with_flags`: It branches on a flag bit of an argument, clamps another argument against a stored range, advances the range end, calls another routine, and sets specific bits in the returned location.
- `poll_until_nonzero`: Calls another routine repeatedly until it returns a non-zero value.
- `loop_until_nonzero_result`: It loops, repeatedly calling a helper and remapping its result until a non-zero value is produced, then returns that value.
- `loop_until_non_zero`: Repeatedly calls another routine and returns the first non-zero result obtained from the mapping process.
- `loop_until_non_zero_mapped`: Iteratively executes another routine and returns the first non-zero value produced after mapping the result.
- `loop_until_nonzero_mapped`: It loops, repeatedly calling a helper and mapping its result until a non-zero value is produced, then returns that value.
- `loop_until_nonzero_wrapped`: It loops calling a helper and mapping its result, incrementing a wrapped counter when the mapped value is zero, and returns the first non-zero mapped value or zero.
- `map_value_via_table_and_flags`: Maps an input argument through a series of range checks and lookup tables, applies conditional subtractions to the result, and then sets specific bits in the return value based on flags present in the original argument.
- `map_argument_table_flags`: Reads a byte from a lookup table using the low bits of the argument, remaps some values, then sets result bits according to flag bits in the argument.
- `process_circular_list_entry`: Locks interrupts, walks a circular list of entries, updates the head index and copies data from the current entry, then releases interrupts and returns a value.
- `load_table_entry`: Reads a four-byte value from a table indexed by a global variable and returns it, or zero when two global variables are equal.
- `advance_counter_locked`: Locks the system while calling two routines from a global structure and advances a counter modulo sixteen, skipping a value when it matches one of two codes.
- `check_and_sample_table_bounds`: Waits for a flag to clear, then computes a coordinate offset from passed values and writes position and size fields into a structure.
- `update_structure_fields_2`: Computes a coordinate offset from the passed values and writes a set of position and size fields into a structure, then repeatedly updates those fields with incremented offsets while waiting for a global flag to clear.
- `update_structure_fields`: Waits for a flag to clear, writes several fields of a structure, then waits again and overwrites some of those fields before returning.
- `reset_and_update_structure`: Waits for a flag to clear, zeroes several structure fields and writes initial values, then waits again and adds a value to the stored fields before returning.
- `store_register_pair`: It calls another routine, then copies the two 32-bit result registers (D0 and D1) into a global variable.
- `setup_blitter_operation`: Initializes a blitter operation by waiting for the blitter idle flag, programming the blitter control, size, and address-modify registers, and loading a palette entry through the color registers.
- `increment_structure_field`: Waits for a flag to clear, writes several structure fields including a value, then waits again and increments one stored field before returning.
- `store_stack_value_then_call`: Stores a value taken from the stack into a global variable, then calls another routine and returns that routine's two data-register results unchanged.
- `parse_data_block`: Iterates through a data block, performing conditional calculations and calling another routine based on specific byte values.
- `tiny_ori_4`: Small routine of 1 instruction that merges 239 into d0.
- `return_from_subroutine`: The routine is empty and performs no operations before returning to its caller.
- `copy_buffer_interleave_2`: Copies five consecutive bytes from a source buffer into five scattered positions of a destination array, repeating for each row of a two-dimensional block.
- `copy_buffer_interleave`: Copies five bytes from a source buffer into five scattered positions of a destination array, repeating for each row of a two-dimensional block.
- `scatter_buffer_interleave`: Reads five scattered positions from a destination array and writes them into five consecutive bytes of a source buffer, repeating for each row of a block.
- `gather_buffer_interleave`: Reads five scattered positions from a destination array and writes them into five consecutive bytes of a source buffer, repeating for each row of a two-dimensional block.
- `initialize_entity_memory_tables`: Initializes a display context and then iterates through a table of entities to allocate and configure per-entry memory blocks.
- `sum_string_byte_values`: Walks through a null-terminated string and accumulates the byte values returned by a lookup routine for each character, then returns the running total.
- `fetch_table_value_or_default`: Returns a fixed value of ten when a global flag is clear, otherwise it indexes a byte table using a scaled parameter and returns the fetched byte.
- `increment_counters_conditionally`: Increments a sequence of words pointed to by a register, conditionally bumping the second and third words based on bits in a global flag, and decrements the fourth word without going below zero.
- `invoke_double_pointer_routine`: Reads a pointer from a global structure, then invokes the routine stored at an offset from that pointer twice in succession.
- `register_table_entry`: Finds the first free slot in a fixed-size table of records and populates it with a pointer, a code pointer, and status fields.
- `handle_indexed_cleanup`: Increments a global counter, reads a short from a table indexed by a passed value, and either invokes a stored routine pointer or clears a field in a structure when a flag is set, then marks the entry as handled and decrements the counter.
- `validate_and_fetch_value`: Checks if a parameter is within a specific range and verifies conditions in a table, returning either a value from a table or a constant.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies (a7)+ to d7, then returns.
- `accumulate_minimal_difference`: Tracks the smallest positive difference between a stored value and a passed value, then accumulates that amount into a running total either in one step or by repeatedly invoking a stored routine until the amount is exhausted.
- `accumulate_callback_results_2`: Iterates through a loop to execute a stored routine pointer, accumulating the returned value into a local variable, a global variable, and a table entry.
- `accumulate_callback_results`: Iterates through a loop to execute a stored routine pointer, accumulating the returned value into a local variable, a global variable, and a table.
- `accumulate_callback_values`: Iterates through a loop to accumulate values from a function call into a global variable and a table, while updating a maximum value in a table.
- `accumulate_values_loop`: Iterates through a loop to accumulate values from a function call into a global variable and a table, while updating a maximum value in a separate table.
- `accumulate_and_track_max_values`: Iterates through a data structure to accumulate values into a global variable and a table, while simultaneously updating a separate table with the maximum value encountered during the loop.
- `process_data_loop`: Iterates through a data structure to accumulate values into a global variable and a table based on results from another routine.
- `decrement_and_dereference`: Decrements a global variable and returns a value from a memory location offset from a register.
- `invoke_double_pointer_dispatch`: A longer routine that invokes a routine reading a pointer from a global structure twice in succession, validates a table index before dispatching, and also dispatches display callbacks.
- `check_sixteen_bit_flags`: Splits a 32-bit value into two 16-bit halves and checks each half in turn.
- `register_string_category`: Looks up a category index for a given string and, if the category has no active entry, copies the string into a stack buffer and registers it.
- `capture_and_store_result`: Runs a routine twice with different arguments, capturing the value returned by the second call.
- `resolve_string_category`: Finds the index of a matching string and returns a value built from its category, or the stored pointer of that entry when the category is marked as active.
- `swap_global_with_parameter`: Swaps the contents of a global variable with a value passed in a parameter, returning the previous value.
- `traverse_linked_list_2`: Recursively traverses a linked list, clearing a pointer field in each node and checking a condition via a function call.
- `traverse_and_clear_linked_list`: Increments a global variable and recursively calls itself to traverse a linked list, clearing a pointer field in each matching entry.
- `reset_and_process_records`: Clears a fixed block of short records and a parallel block of byte flags, then walks a table of five-field records and, for each entry whose second flag has a certain bit set, toggles some flag bits while invoking two other routines on the associated pointer.
- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `validate_index_and_dispatch`: Rejects indices outside the 0 to 29 range, then triggers a stored routine whenever the matching table entry is nonzero, and finally returns the constant 1.
- `find_matching_string_index`: Walks an array of null-terminated pointers, comparing each string against a given argument, and returns the index of the first match or a sentinel value when none is found.
- `adjust_and_process_string_length`: Computes a string length adjusted by one when a flag is set, then passes the result to another routine and invokes a library call.
- `dispatch_display_callbacks`: Invokes a stored routine pointer, then calls two other routines in sequence, and finally returns without producing a value.
- `remove_and_compact_entries`: Scans a table of entries, freeing those that are marked and empty, then compacts the remaining entries and reduces the stored count by the number removed.
- `allocate_tracked_memory`: Allocates a tracked block from the exec library when the flag is clear, linking it into a free list and updating bookkeeping counters, otherwise returns a previously allocated block.
- `attempt_memory_allocation`: Initializes a memory allocation tracker for a block of the requested size, then repeatedly attempts to obtain memory until the attempt succeeds.
- `remove_entry_from_linked_list`: Adjusts accounting fields based on a flag in the passed value, removes a matching entry from a linked list of globals, optionally calls a stored routine, decrements a global counter, and returns the located entry or zero.
- `call_avail_mem`: Library stub: loads its arguments and the exec library base, calls AvailMem and returns its result to the caller.
- `compute_available_memory_delta`: Requests available memory from the operating system, accumulates offset differences, and returns the requested amount minus the accumulated total.
- `free_linked_list_entries`: Recursively frees each entry in a linked list and then clears the head pointer to empty the list.
- `call_recursive_routine`: Calls itself and then returns.
- `initialize_and_scale_pointers`: Loads two pointers from a global structure and initializes their pointed-to words, then conditionally calls a routine and rescales the values when the inputs fall within range.
- `nop_operation`: This routine is empty and performs no actions before returning.
- `check_entity_flag_and_process`: Retrieves a pointer from another routine and, when that pointer is valid, checks a specific bit in the second byte of the pointed-to structure.
- `parse_recursive_data_stream`: Iterates through a data buffer, identifying specific control characters to trigger recursive calls or token processing.
- `parse_nested_token_stream_2`: Iterates through a data buffer to parse a sequence of tokens, supporting nested structures and specific character delimiters.
- `parse_nested_token_stream`: Iterates through a data buffer to parse a sequence of tokens, supporting nested structures and specific character delimiters.
- `return_immediately_3`: This routine performs no operations and returns to its caller immediately.
- `parse_nested_data_stream_2`: Iterates through a data buffer by reading byte values and advancing a pointer, supporting recursive calls for nested structures.
- `parse_data_stream`: Iterates through a data buffer by reading byte values and advancing a pointer.
- `parse_nested_data_stream`: Iterates through a data stream by reading bytes and following pointers to process nested structures.
- `parse_input_stream`: Iterates through a buffer of data, performing recursive calls and conditional checks based on specific byte values.
- `restore_saved_registers_return_d7`: Copies d7 into d0 as the return value, restores saved registers from the stack frame, releases a5 frame storage and returns.
- `parse_bracketed_range_token_8`: Parses a string within square brackets to check if a specific byte value matches a literal character or falls within a numeric range defined by a hyphen.
- `parse_bracketed_range_token_7`: Parses a string within square brackets to determine if a specific byte value falls within a defined numeric range or matches a literal character.
- `parse_bracketed_range_token_6`: Parses a string within square brackets to determine if a specific byte value matches a literal character or falls within a numeric range.
- `parse_bracketed_range_token_5`: Parses a string within square brackets to determine if a specific byte value matches a literal character or falls within a numeric range defined by a hyphen.
- `parse_bracketed_range_token_4`: Parses a string within square brackets to determine if a specific byte value matches a literal character or falls within a numeric range defined by a hyphen.
- `parse_bracketed_range_token_3`: Parses a string within square brackets to identify if a specific byte value falls within a defined range or matches a literal value.
- `decrement_pointer_target`: Decrements a value at a memory location pointed to by a structure member and stores the result back, using different logic based on the contents of two other offsets.
- `decrement_pointer_target_value`: Decrements a word value located at a memory address pointed to by a structure member and stores the result back to that same location.
- `decrement_and_store_word`: Decrements a word value from a memory location and stores the result into another location pointed to by a pointer.
- `decode_compressed_bitstream_data`: Decodes a variable-length bitstream using a custom compression scheme.
- `decode_variable_length_bitstream_3`: Decodes a variable-length bitstream by reading successive values and accumulating bits into a local buffer.
- `decode_variable_length_bitstream_2`: Decodes a variable-length bitstream by iteratively reading values and accumulating bits into a local buffer.
- `parse_integer_value`: Executes a loop that calls another routine to fetch data, performs arithmetic on the results, and stores the final value into a destination pointed to by a global variable.
- `parse_signed_integer_3`: Checks for sign characters to determine a multiplier, then executes a loop to parse numeric data and stores the resulting signed value into a destination.
- `process_loop_with_output_2`: Executes a loop that calls another routine and performs arithmetic on the result, eventually storing a value into a destination pointed to by a structure.
- `process_loop_with_output`: Executes a loop that calls another routine and performs arithmetic on the result, eventually storing a value into a destination pointed to by a structure member.
- `copy_word_conditionally`: Copies a word from a source offset to a destination address pointed to by a pointer based on a condition.
- `store_word_and_increment_pointer`: Stores a word from a memory location into a destination pointed to by a pointer, then increments a value and returns it.
- `unpack_variable_length_integer`: Checks if a value is within a specific range; if not, it stores the value and returns.
- `process_bit_stream`: Iterates through a loop that extracts bits from a value returned by another routine, accumulating them into a local variable before storing the result into a destination pointer.
- `update_pointer_data`: Checks a value at an offset from a pointer and updates a destination address with a word from a source location.
- `transfer_word_data`: Moves a word from a source offset to a destination address stored within a pointer.
- `decode_variable_length_bitstream_buffer`: Decodes a variable-length bitstream by iteratively reading bits and accumulating them into a local buffer.
- `decode_variable_length_bitstream`: Decodes a variable-length bitstream by reading bits through a stream decoder and accumulating values based on specific flag bits.
- `decode_variable_bit_stream_2`: Decodes a bit stream by checking multiple flag bits to select specific offsets, shifting a local accumulator, and storing the final result into a destination.
- `decode_variable_bit_stream`: Decodes a bit stream by checking multiple flag bits to select specific offsets, shifting a local accumulator, and storing the final result into a pointer.
- `store_result_to_pointer`: Checks if a pointer in a structure is non-zero and stores a value from a local offset into the memory location it points to.
- `update_pointer_data_2`: Checks a value at a specific offset from a pointer and stores a word from a preceding offset into a destination address.
- `store_word_from_offset`: Takes a word from a specific offset and stores it into a destination indicated by a pointer.
- `stream_bytes_to_buffer`: Iterates through a loop, calling another routine to fetch bytes and storing them into a buffer pointed to by a structure member while incrementing the buffer pointer.
- `update_word_and_increment`: Stores a specific word value into a memory location via a pointer and returns an incremented value or zero.
- `increment_and_return`: Increments a value and returns it.
- `process_input_stream`: Repeatedly calls a function to retrieve values, stores bytes into a buffer, and decrements a counter until a condition is met or a specific bit is set in a global byte.
- `parse_bracketed_range_token_2`: Parses a string within brackets to identify if a specific byte value falls within a defined range or matches a literal value.
- `parse_bracketed_range_token`: Parses a string within brackets to identify a specific byte value, supporting hyphen-delimited numeric ranges.
- `parse_string_token`: Iterates through a string to find a specific character or a delimiter within brackets.
- `search_byte_with_range_tracking`: Iterates through a memory buffer to find a specific byte value while tracking the minimum and maximum byte values encountered between specific delimiter characters.
- `search_and_update_buffer`: Iterates through a memory buffer to find a specific byte value within a defined range.
- `search_byte_in_range`: Iterates through a memory buffer to find a specific byte value within a defined range.
- `search_byte_in_range_2`: Iterates through a memory range to find a specific byte value.
- `process_data_stream`: Iterates through a data stream to find specific characters and updates a pointer and a counter.
- `clear_and_store_pointer`: Checks if a value at a specific offset within a structure is non-zero, and if so, clears the value pointed to by that offset.
- `process_buffer_stream`: Processes a stream of data by checking for specific characters and updating a pointer and a counter, potentially terminating based on a value or a specific byte.
- `nop_padding_sequence_5`: This routine is empty and consists of two instructions that perform no operation, serving as a gap or padding within the executable segment.
- `store_word_and_increment`: Stores a word into a memory location at an offset from a pointer and returns an incremented value.
- `parse_format_string`: Parses a format string containing tokens like #, +, -, *, leading zeros, ., l, h, p, x, and digits, advancing a pointer through the string while reading values from a parameter list.
- `process_format_string`: Walks through a character string one byte at a time and stops when it finds a percent sign.
- `iterate_table_entries`: Steps backward through a global table of structures, skipping entries that are empty or have a particular flag bit set, and calls another routine for each matching entry.
- `tokenize_quoted_string`: Walks through a text buffer, skipping whitespace and splitting the content into up to 32 tokens while treating double-quoted runs as single tokens, writing a pointer to each token into a stack table and null-terminating each token in place.
- `execute_indirect_function_chain`: Checks if a global variable contains a function pointer and executes it, followed by a sequence of indirect calls using offsets derived from that pointer.
- `process_object_vtable_callbacks`: Iterates through a list of objects to execute specific functions retrieved from their virtual method tables.
- `swap_memory_blocks`: Swaps a specified number of bytes between two memory locations.
- `convert_uppercase_chars`: Iterates through a null-terminated string and converts uppercase ASCII characters to lowercase based on a flag in a global lookup table.
- `copy_and_pad_string_2`: Copies characters from a source to a destination until a null terminator is reached or a length limit is hit, then pads the remainder of the destination with null bytes.
- `copy_and_null_terminate`: Copies a string from one location to another and fills the remaining length of the destination buffer with null bytes.
- `zero_memory_block`: Fills a memory block with zeros using a counter.
- `compare_case_insensitive_strings`: The routine compares two byte strings character by character, converting uppercase letters to lowercase for comparison, and returns the difference between the first mismatching characters or zero if equal.
- `copy_string_to_buffer_2`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `find_byte_in_string`: Searches a null-terminated string for a specific byte and returns a pointer to its location, or zero when the byte is not present.
- `call_and_return_value`: It forwards its argument to another routine and returns whatever that routine produces.
- `find_last_byte_in_string`: Walks a null-terminated string looking for a specific byte and returns a pointer to its last occurrence, or zero when the byte never appears.
- `skip_flagged_bytes`: Iterates through a memory block and returns a pointer to the first byte that does not have a specific bit set in a global lookup table.
- `tiny_move_movea_rts`: Small routine of 3 instructions that copies a3 to d0, then copies (a7)+ to a3, then returns.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `convert_integer_to_ascii_buffer`: Converts an integer into a sequence of ASCII decimal digits and stores them in a destination buffer.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `expand_and_parse_data`: Expands a packed nibble value using a lookup table and then parses the resulting numeric string into a signed integer.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_signed_integer_2`: Converts a string of numeric characters into a signed short integer, handling optional leading plus or minus signs.
- `parse_signed_integer`: Parses a string of numeric characters into a signed short integer, accounting for a potential negative sign.
- `initialize_command_parser`: Clears a global counter, stores a pointer in a global variable, and calls a routine to parse a command stream using that pointer and a stack slot.
- `increment_and_store`: Increments a global pointer and stores a byte from a parameter into the location pointed to by the incremented global pointer.
- `format_string_to_buffer`: Assigns a global pointer to a supplied address, processes a character string byte by byte, and stores a zero byte at the resulting pointer location.
- `copy_memory_block_2`: Copies a specified number of bytes from a source address to a destination address.
- `validate_linked_entry`: Calls another routine to retrieve a pointer.
- `process_linked_list_flags`: Walks a global linked list of structures, testing two flag bits in a status byte and computing the difference between two integer fields.
- `invoke_global_callback`: Invokes a callback through a pointer in a global table and, when a global flag is set, recursively calls itself first.
- `dispatch_callback_with_adjustment`: Invokes a callback through a pointer in a global table and, when a global flag is set, recursively calls itself first.
- `initialize_structure_fields`: It writes a full-byte bitmask at a fixed offset of a structure and writes full-word bitmasks at two other offsets.
- `cleanup_display_context`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `invoke_method_if_present`: It checks whether a global pointer is non-null and calls another routine when it is, then invokes a function pointer stored within a structure.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `divide_fixed_point_values`: Performs division on two 32-bit fixed-point values, including sign handling and normalization of the divisor.
- `negate_fixed_point`: Returns a 64-bit value where the high and low 32-bit words are the negation of the input's components.
- `divide_fixed_point`: Performs a complex fixed-point division by normalizing the divisor through bit shifts and calculating the quotient and remainder.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `allocate_and_set_node`: It allocates memory for a structure through a library call and, if successful, initializes certain bytes and a dword with a provided parameter.
- `create_message_node`: Obtains a global variable and calls a routine through a table; if the result is a sentinel, it returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_and_dma_registers`: Sets a series of memory locations to specific values, including an or even loop that writes certain patterns to a table, and updates hardware registers for audio and DMA.
- `update_state_machine_and_table`: Updates a state-machine-based routine or initializes global pointers and counters.
- `update_engine_state_machine`: Updates a global state machine or state-dependent configuration by resetting or incrementing counters and copying data or pointer-based values from one memory location to another.
- `initialize_and_update_state_machine`: A state-machine routine that initializes or resets several global counters and memory values, and performs a bulk copy of data from a source table to a destination buffer if certain memory conditions are met.
- `update_memory_with_difference_2`: Updates several memory locations with data from a source pointer, while also decrementing a global counter and wrapping it if it reaches a certain limit.
- `update_memory_with_difference`: Compares the high or low nibbles of two or even individual bytes and updates a memory location by a certain amount based on which value is larger.
- `tiny_ori_2`: Small routine of 1 instruction that merges 12 into d0.
- `set_register_value`: stores a value into a register via an absolute or or-addressing move or clear instruction.
- `tiny_ori_3`: Small routine of 1 instruction that merges 168 into d0.
- `decrement_memory_value`: decrements a value stored at a memory location using an absolute subtraction instruction.
- `move_to_memory`: moves a value into a memory location using an absolute addressing mode.
