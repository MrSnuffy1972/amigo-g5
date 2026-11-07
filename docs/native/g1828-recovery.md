# Game 1828 (dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01828). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (965472 top-level bytes in 54 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 26 of 26 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 626 of 626 named (0 author symbols), 626 of 626 described (76 by rule, 540 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3568 of 3568 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 31078 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 20129 of 20131 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 526951 |
| code | 63722 |
| data | 329417 |
| padding | 12102 |
| unused | 33280 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:Install | 7 | 45 | 2 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DuneII | 13 | 251 | 6 |
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
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DUNEint.EXE | 2 | 267 | 8 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:Add16k | 3 | 475 | 32 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:MakeDummy | 2 | 1275 | 25 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 144 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1633 | 53 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1831 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7031 | 218 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 262 | 11 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DUNE.ENG (raw code file, load address unknown) | 1 | 0 | 0 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DUNE.FRE (raw code file, load address unknown) | 1 | 0 | 0 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DUNE.GER_old (raw code file, load address unknown) | 1 | 0 | 0 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DUNE.GER (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 72 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 388 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 40 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2355 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 205 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)[a2]/Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DUNE.ENG: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)[a2]/Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DUNE.FRE: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)[a2]/Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DUNE.GER_old: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)[a2]/Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DUNE.GER: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)[a2]/Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- dune ii - battle for arrakis (1993)(virgin)(m3)[cr pdx](disk 1 of 6)[a2]/Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DuneII: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 4 never-loaded region(s), 31078 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:Install

- `run_hdinstall_entry`: It runs a fixed sequence of helper calls, then a loop that calls a routine a fixed number of times while dereferencing a handler from a global record.
- `decrement_until_minus_one`: Loops, decrementing a signed 16-bit value by one each iteration, until that value becomes negative one, then returns.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DuneII

- `initialize_and_verify_table`: It invokes a couple of setup routines, then loops a fixed number of times calling another routine and dereferencing a pointer stored in a global variable.
- `countdown_to_minus_one`: Repeatedly subtracts one from a local signed word until the value equals negative one, then exits.
- `run_cache_clear_loop`: It calls a routine twice, then loops a fixed number of times calling another routine and processing its result through nested branches.
- `copy_byte_block`: Copies a run of bytes from a source location to a destination location, decrementing both pointers as it goes.
- `copy_memory_block`: Copies a block of bytes from one memory location to another, moving backward through both buffers.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Startup

- `initialize_startup_environment`: Resets a block of memory to zero, sets up a stack frame, and then runs a series of conditional initialization routines that parse the command line and register cleanup handlers.
- `execute_cleanup_sequence`: It performs a series of conditional calls to routines stored in a global structure.
- `execute_cleanup_callbacks`: Iterates through a linked list of structures to invoke a sequence of function pointers.
- `return_without_effect_3`: This routine contains no executable body and simply returns to its caller without performing any work.
- `return_without_effect_4`: This routine contains eight instructions that perform no observable work and simply return to its caller.
- `free_linked_list_memory`: The routine frees a linked list of pointers by walking it and invoking a free callback for each element, then processes a second list of structures using a similar callback and returns its last result.
- `process_callback_status_alt`: Evaluates a status value by checking bits in a structure to determine whether to store a constant, execute a callback, or iterate through a linked list, updating a structure field with the result before returning.
- `process_status_counter`: Copies a value into a local variable, then reads a status from a table of routines and branches on it.
- `manage_startup_task_state`: Checks a status flag in a structure to determine whether a specific initialization sequence has already occurred, then either records a fixed value or invokes a handler pointer stored in that structure.
- `run_cleanup_sequence`: Sets a flag in a global structure, then walks a linked list of entries calling cleanup routines based on each entry's flags.
- `parse_command_line_and_initialize`: Parses a command line by splitting it into tokens on spaces and newlines, handling quoted strings and backslash escape sequences, and storing each token pointer in a table.
- `copy_and_setup_buffer`: Copies a bounded number of bytes from a source buffer into a stack buffer and null-terminates it, then runs a series of setup routines that decide whether initialization succeeds.
- `copy_startup_environment`: Copies a bounded number of bytes from a global structure into a local stack buffer and null-terminates the result.
- `traverse_linked_list`: The routine iterates through a linked list for a specified number of steps and returns a pointer to the current node.
- `copy_startup_environment_2`: Copies a bounded number of bytes from a global structure into a local stack buffer and null-terminates the result.
- `find_byte_in_buffer_2`: The routine searches for a specific byte value within a memory buffer of a given length, stepping through the buffer and returning a pointer to the matching byte or null if the length is exhausted without a match.
- `advance_to_string_end`: It advances a pointer through a null-terminated string until the terminator is reached, leaving that pointer on the zero byte.
- `copy_bytes_with_termination_2`: It copies bytes from a source pointer to a destination pointer until a zero byte is encountered, then fills the remaining space up to a given count with zero bytes using a counter.
- `run_callback_cleanup`: Walks a linked list of callback entries, invoking each entry's routine at offset 4, then iterates a second linked list calling cleanup handlers based on per-entry flags, and finally processes a third list of callbacks whose behavior depends on flag bits.
- `do_nothing`: This routine is empty and performs no operations before returning.

### Format

- `parse_command_line_arguments_2`: Initializes a small structure from the incoming arguments, then parses a command line string into individual tokens, handling quoted sections and backslash escape sequences, storing the resulting token pointers for later use.
- `restore_patch_state_2`: It retrieves a pointer from a global structure and stores a parameter into a slot within a structure pointed to by that pointer.
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
- `release_format_buffers_and_handles`: It releases the buffers and handles that the format routine allocated, calling a release routine for each field that is set.
- `report_disk_errors_2`: Retries a device operation until it succeeds or a non-retryable error occurs, then maps the resulting error code to a message and reports it through a handler.
- `report_disk_errors`: Branches on a global short and calls different routines depending on whether it is zero, then returns a byte from a register.
- `store_matched_table_byte`: It searches an eight-entry global byte table for the given value and records the matching entry in a field of a structure obtained from another routine, returning the original value whether or not a match was found.
- `find_drive_list_entry`: Walks a linked list of drive entries until one matches the given number or pointer, marks that entry, and returns 1.
- `calculate_string_length`: It walks through a null-terminated string counting characters until the terminator and returns that count.
- `copy_cstring_to_buffer`: It copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers and writing each byte until a zero byte is encountered.
- `convert_lowercase_to_uppercase`: It compares the input byte against the lowercase letter range and, if the value falls between 'a' and 'z', subtracts 32 from it; any other value is passed through unchanged.
- `compare_strings_ignoring_case`: It compares two null-terminated strings one character at a time after converting each to uppercase.
- `truncate_string_at_colon`: It scans a string until it finds a colon or a null terminator, then overwrites that position with a zero byte to cut the string short.
- `call_single_value_status`: Places its first argument into a one-word stack buffer and forwards it to a routine invoked with a length of one.
- `create_block_from_source_words`: Allocates a fixed-size block through a library call, links it into a record chain, copies up to seven words from a source array into it, and returns a value from the allocated block.
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
- `forward_argument_to_library_stub`: It passes its one incoming argument, a library base pointer read from a global variable, and the address of its own stack argument to the library call stub at offset minus 132.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_tracks_and_verify`: Allocates a buffer and an interrupt server, then formats a disk track by track by filling the buffer with track data and writing it through the disk device, and finally verifies the tracks.
- `write_value_to_output_handle`: It calls two output routines with fixed arguments on a handle taken from a structure.
- `call_global_and_checksum_buffer`: It invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `divide_32bit_values`: It divides a 32-bit value by a 16-bit divisor, computing the quotient and remainder for the high and low halves.
- `rotate_32bit_value`: It splits a 32-bit input into its high and low halves, then loops sixteen times doubling the low half while folding a carry bit into the high half.
- `multiply_32_bit_integers`: It multiplies the high and low 16-bit halves of two 32-bit inputs using four partial products and combines them into a 32-bit result.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer`: It stores a byte received in a register into the memory location pointed to by a pointer held in another register, then returns.
- `allocate_indexed_record_block`: Allocates a fixed-size block through a library call using a flag byte, then fills the block with a parameter, a flag, the constant 4, and a slot index before calling more library functions and returning the block or zero when a step fails.
- `release_record_chain`: It recursively walks a linked chain of records, processing the linked record first when its link field is set.
- `forward_argument_with_code`: Passes its single argument to another routine together with the constant 48 and then returns.
- `forward_single_argument`: Passes its single argument on to another routine and returns nothing.
- `initialize_system_structures`: Initializes several global pointers and variables by retrieving values from a memory block, potentially setting them to default values if the block is null, and performs a series of function calls to manage these structures.
- `execute_with_status_check`: It invokes a routine through a global pointer, passing a value and a stack slot, then checks the returned status.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: It writes its own pointer argument into the location it points to, adds four to that stored value, clears the next word, and stores the original pointer into the following word.
- `store_word_and_pointer_in_node`: It calls a library-backed allocation routine with a second argument, and if it returns a non-null block, stores a byte value, a word from the second argument, and the first argument pointer into fixed offsets of that block.
- `reset_node_and_release_it`: If the given pointer is non-null, it sets a byte field to 255 and clears two longword fields of the structure, then passes the pointer and a word field to another routine and returns its result.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### More

- `restore_and_parse_command_line`: Initializes a global structure with a couple of values, then parses the command line, splitting it into tokens while trimming spaces and handling quoted strings with escape sequences, storing the resulting arguments into a slot of the structure.
- `restore_patch_state_3`: It retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Allocates a file structure, reads its header to populate metadata fields, and loads the associated data into memory while handling errors and updating internal state flags.
- `iterate_table_with_function`: Loops four times, passing a fixed field from the passed structure alongside each successive value from a four-entry table to another routine.
- `validate_input_buffer`: Invokes another routine twice with arguments derived from the input pointer, then scans up to eighty bytes verifying each value falls within a set of allowed ranges.
- `process_disk_status`: Initializes a structure field, then repeatedly polls a status source until it is non-zero or a timeout occurs.
- `invoke_routines_in_sequence`: Invokes a routine using a global variable, invokes it again with no arguments, then calls a function obtained from an address register, and finally returns.
- `initialize_entity_state`: Initializes fields in a structure and invokes another routine with values from that structure, then loops calling further routines while certain size and flag fields remain satisfied.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from the input pointer, stores the second result into a field of the pointed-to structure, and if a flag field is zero it increments a short located at an offset from a pointer stored in the structure.
- `parse_buffer_content`: Consumes from a buffer, records the resulting size, and walks the data stopping at a newline or form-feed character.
- `process_structured_loop`: Reads a field from a passed structure to compute a loop limit, then repeatedly invokes another routine over a table using a pointer from the structure and entries from a global variable, performing a final check that may trigger another routine.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `forward_handler_sequence`: Runs a fixed sequence of calls to another routine using arguments from a parameter block and a global variable, capturing the result of one of those calls and returning it after issuing a final cleanup call.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at an offset from a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: Calls a drawing routine using a pointer and a value from a structure, then calls another routine using an offset from a pointer stored in that structure.
- `draw_text_window_range`: Calls a drawing routine with a zeroed argument, then calls another routine using a decremented value derived from a pointer in the structure.
- `draw_text_line`: Iterates over a text buffer while issuing drawing primitives through a handler pointer, tracking how many lines have been emitted.
- `dispatch_recursive_pointer`: Calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a flag field in the passed structure and selects a global pointer or a remote structure accordingly.
- `dispatch_by_flag`: Selects two pointers based on a flag in the parameter block, using a table entry for one of them when the flag is set.
- `dispatch_handler_by_flag`: Reads a flag from the parameter block and forwards the request to another routine, passing three references to a global variable when clear or a table value when set.
- `clamp_negative_result`: It calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, when a condition is met, advances offset fields and reduces the remaining size, calling another routine with a pointer derived from the structure.
- `advance_offset_and_update_state`: Checks a size field and, when it is nonzero and its limit field equals a fixed value, calls a routine with a pointer derived from an offset and a remaining size, returning early otherwise.
- `verify_and_adjust_offset`: Computes a target value from a base value and offsets, then verifies it lies within a range defined by a start and size field.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string_2`: Copies bytes from a source pointer to a destination pointer one at a time until a null terminator is reached.
- `convert_string_to_uppercase`: Walks through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant from their values.
- `parse_signed_decimal_2`: Reads a run of digit characters from a null-terminated string into an accumulating value and applies a negative sign when a leading minus is present.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `process_message`: Recursively processes a structure by invoking another routine on certain fields and string arguments, then conditionally updates structure fields and invokes further routines.
- `build_message_buffer`: Obtains a pointer from another routine, then calls a second routine with that pointer, a constant, and a stack buffer holding the input parameter.
- `copy_entity_data`: It requests a handle from another routine and, if the handle is valid, reads a value from it, passes it to another routine together with a small stack buffer, and copies two fields from the handle into the caller-provided structure.
- `build_object_from_template`: Returns zero when the first argument is zero or when a routine called with zero arguments returns zero, otherwise it allocates a block, links its header fields to the allocated address and the source and template arguments, copies words from the source address into the object, reads a value from that object, and performs cleanup calls before returning the copied value.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `invoke_schedule_callback`: Invokes a routine through a pointer obtained from the scheduling library.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_indirect_pointer_2`: Dereferences a function pointer stored in a global variable and invokes it, then returns.
- `invoke_indirect_handler`: Loads a function pointer from a global variable and jumps to it, acting as an indirect call wrapper.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_indirect_pointer_3`: Dereferences a function pointer held in a global variable and calls it before returning.
- `invoke_indirect_handler_2`: Fetches a function pointer from a global variable and executes it via an indirect jump.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_indirect_pointer_4`: Dereferences a function pointer stored in a global variable and calls it, then returns.
- `invoke_indirect_handler_3`: Loads a function pointer from a global variable and jumps to it, serving as an indirect call wrapper.
- `call_indirect_pointer_5`: Dereferences a function pointer held in a global variable and invokes it before returning.
- `call_indirect_global_ptr`: It invokes the routine whose address is stored at a location derived from a global variable, passing no arguments.
- `forward_global_value`: Loads a value from a global variable and passes it to another routine through a stored call pointer.
- `call_indirect_d1_offset_3`: It invokes the routine whose address is stored at a global variable located 336 bytes before the start of the code section, passing no arguments.
- `call_indirect_from_global`: It loads an address from a global variable and jumps to the routine stored there, passing no arguments.
- `invoke_handler_pointer`: It reads a handler pointer from a global variable and transfers control to the routine located at that address.
- `call_stored_pointer`: It fetches a code pointer from a global variable and calls the routine stored at that location.
- `dispatch_indirect_call`: It loads a routine address from a global variable and jumps to the handler kept there.
- `call_indirect_global_ptr_2`: It retrieves a pointer from a global variable and invokes the routine stored at that address.
- `invoke_global_handler`: It reads a handler pointer from a global variable and transfers control to the routine stored there.
- `call_indirect_from_global_ptr`: It loads a code pointer from a global variable and jumps to the routine located at that address.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `checksum_stack_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `extract_byte_from_call`: Invokes another routine through a pointer obtained from the scheduling library, passing a small stack buffer and a constant.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer`: Stores a byte passed in a register into the memory location pointed to by a pointer.
- `modular_inverse_32bit`: Performs a 32-bit modular reduction using repeated doubling and subtraction when the divisor is large, and falls back to 16-bit modulo and divide operations for smaller divisors.
- `multiply_32_bit_integers_2`: Computes the 32-bit product of two 32-bit values by combining their 16-bit halves into a single result.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable and calls the address stored at that specific location in the global data structure.
- `store_byte_via_a3_2`: Stores the low byte of register D0 through the pointer held in A3.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Allocates a message buffer and, if successful, populates several fields with parameters and a value from another routine before dispatching it.
- `process_and_clear_port`: Checks whether a specific field of a structure is non-zero and, if so, invokes a handler through a pointer.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, returning success when they match and skipping the comparison when the first string begins with a newline character.
- `setup_pointer_chain_2`: Writes its pointer argument into the location it points to, advances that stored value by four, zeroes the following word, and records the original pointer in the next word.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_words_then_call_handler`: Copies a fixed number of words from a global table into a stack buffer, then jumps to a handler pointer.

### Echo

- `scan_records_and_copy_table`: Walks a linked list of records to find the largest stored value, then writes that value and a run of incrementing marker words into a table.
- `copy_block_and_call_handler`: It copies a 60-byte block from a source address into a destination address when the destination pointer is not null, then invokes a handler routine pointed to by a global.

### Execute

- `find_max_and_install_handler`: Walks a linked list of records to find the largest value in a specific field, then stores that maximum along with a sequence of marker values into a table.
- `copy_block_and_call_handler_2`: Copies a 15-word block of data from a source table pointed to by a global into a destination buffer passed as an argument, then invokes the handler routine stored in another global.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_copy_block`: Walks a linked list of records to find the largest value stored in one field, then stores that maximum along with a run of incrementing marker words into a fixed table.
- `copy_table_words`: Copies a fixed run of sixteen words from a global table into a caller-provided buffer when that buffer is not null, then invokes a handler routine pulled from a global slot.

### IconX

- `parse_command_line_arguments`: Initializes a temporary buffer from a global structure and trims trailing spaces from a command-line string, then splits that string into individual tokens, unescaping quoted sections and converting asterisk escape sequences into newline and escape characters before storing the resulting words.
- `restore_patch_state`: It reads a pointer from a global structure and stores the received parameter into a slot of the pointed-to structure.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and a given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot, then runs the command through the DOS library.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result, returning zero when no descriptor is given.
- `scan_ram_entries_for_names`: Allocates a buffer and walks a linked list of entries, copying entries whose name bytes fall outside the control range into it.
- `reorder_and_chain_scalar_calls`: Calls a helper routine with its arguments rearranged, then calls it again with a different pairing, stores the result of a third call, and finally passes a summed value to a fourth call before returning.
- `count_string_length`: It walks a null-terminated byte string one byte at a time, incrementing a counter until it reaches the terminating zero, and returns that count.
- `copy_null_terminated_string`: It copies bytes from a source pointer to a destination pointer, advancing both, until the source null terminator is copied.
- `advance_to_string_end_and_call`: Walks a null-terminated byte string one byte at a time until it reaches the terminating zero, then passes that position along with a second argument to another routine.
- `parse_signed_decimal`: It skips an optional leading plus or minus sign, then accumulates each following digit into a running total, negating the result when a minus sign was present.
- `call_supervisor_entry`: It jumps through a function pointer located in a library structure and returns.
- `exit_interrupt_handler`: It jumps through a function pointer located in a library structure and returns.
- `schedule_dispatch_work`: It jumps through a function pointer located in a library structure and returns.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatch_entry`: It jumps through a function pointer located in a library structure and returns.
- `call_library_vector_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_exec_make_functions`: It loads the function-pointer slot from an Exec library dispatch table at a negative offset and jumps to it, invoking a library function.
- `call_indirect_d1_offset`: It calls the routine whose pointer is stored at a global variable located 102 bytes before another global variable held in register d1, then returns.
- `call_indirect_global`: It invokes the routine whose address is stored in a global variable and then returns.
- `invoke_table_vector`: It invokes the routine whose address is stored at the beginning of a global table, passing the table's own start address as an argument.
- `call_table_function`: It invokes the function whose address is stored at a fixed location in a global table and then returns without using any arguments.
- `call_indirect_d1_offset_2`: It calls the routine whose address is stored at the location pointed to by the value in register d1 minus 222, then returns.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_function`: It loads the function-pointer slot from a library dispatch table at a negative offset and jumps to it, invoking a library function.
- `call_indirect_pointer`: It dereferences a pointer stored in a global variable and jumps to the routine it points to.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_indirect_table`: It invokes the routine whose entry pointer is stored at a fixed offset within a global table and then returns.
- `call_indirect_d1`: It invokes the function whose pointer is stored at a fixed offset from register d1, then returns without modifying anything else.
- `call_pointer_routine`: Fetches a function pointer from a global variable and jumps to it.
- `call_pointer_routine_2`: Fetches a different function pointer from a global variable and jumps to it.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable, then walks a stack buffer computing a rolling checksum until a null terminator is found, passing the buffer and its inverted checksum to another routine when that value is non-zero.
- `store_byte_from_register`: It stores a byte received in a register into the memory location pointed to by a preserved pointer.
- `invoke_global_function_pointer`: Loads a function pointer from a fixed offset within a global variable and calls it before returning.
- `store_byte_via_a3`: Copies a single byte from register D0 into the memory location pointed to by A3.

### Disk-Validator

- `init_and_process_table`: Initializes a global structure by writing a header value, storing two ASCII characters into a buffer, and zeroing several fields.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:DUNEint.EXE

- `process_relocation_table`: Processes a relocation table by scanning for hunk type markers and copying the referenced data blocks into a new buffer, adjusting their addresses to fit the allocated region.
- `decrement_to_zero`: Executes a countdown loop that decrements a value until it reaches zero, then returns without performing any other work.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `rotate_and_shift_byte`: Tests whether a shifted flag in a global variable is set and returns early when it is.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `rotate_bits_across_word_array`: Processes a fixed number of 32-bit words, rotating bits between an accumulator and the current word depending on a shift amount, updating the accumulator and pulling in new words when it runs out, then decrements a counter before looping.
- `invoke_trap_and_halt`: Triggers a 68000 trap vector and then falls through into some non-executable bytes, so the routine does not perform any recoverable work.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:Add16k

- `resolve_and_decrement_slot_counter`: Resolves a pointer into a per-slot record, decrements a counter byte stored there, and then invokes a routine using the updated pointer and a shifted value.
- `open_graphics_and_intuition_libraries`: Opens the graphics and intuition libraries by name through the library-open routine and keeps both base pointers.
- `clear_slot_record_store_stack`: Clears forty-two longwords of a per-slot record, stores the stack position and a global value into it, then calls another routine.
- `open_dos_and_mathffp_libraries`: Opens the dos and mathffp libraries by name through the library-open routine and keeps their bases.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `wait_and_close_stored_libraries`: Runs a ten-iteration wait loop, then closes each stored library base that is nonzero and releases stored resources through their routines, returning the value it was given.
- `append_limited_string_to_buffer`: Walks to the end of the first string, then copies bytes from the second string until it hits a zero byte or a counter runs out, null-terminates the result, and returns the first string.
- `append_string_limited_by_count`: Walks to the end of the first string, then copies bytes from the second string until it hits a zero byte or the count argument runs out, null-terminates the result, and returns the first pointer.
- `copy_string_including_terminator`: Copies bytes from the source string to the destination pointer until a zero byte is copied, including that terminator, then returns the destination pointer.
- `measure_string_length`: Walks byte by byte through the string until it reaches the terminating zero and returns the number of bytes before that terminator without modifying any memory.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the remaining space up to that count with zeros and returns the destination pointer.
- `invoke_slot_if_flag_unset`: Indexes into a table of two-byte entries using the argument, rejects out-of-range or empty slots, clears a flag in the referenced slot if it is not already set, invokes the routine stored there, and returns zero on success or a failure value otherwise.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_amiga_library_entry`: Fetches a library base from a global variable and jumps through a computed offset within that library, returning immediately after the call.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `jump_indirect_table_offset`: Reads a pointer from a table indexed by an offset derived from a global variable and jumps to the routine stored at that address.
- `dispatch_indirect_table`: Looks up a value in a table indexed by an offset derived from a global variable and then jumps to the routine stored at that computed address.
- `invoke_indirect_table`: Looks up an entry in a table indexed by a value derived from a global variable and then calls the routine stored at that location.
- `dispatch_via_jumptable`: Looks up an entry in a table indexed by a value stored at a fixed global variable and then jumps to the routine found there.
- `jump_indirect_table`: It reads a pointer from a table indexed by an offset and jumps to the routine stored at that address.
- `jump_indirect_table_2`: It reads a pointer from a table indexed by a different offset and jumps to the routine stored at that address.
- `call_indirect_table_2`: Looks up an entry in a table indexed by a value and then calls the routine stored at that entry.
- `jump_indirect_table_other_offset`: Reads a pointer from a table indexed by a different offset within a global variable and jumps to the routine stored at that address.
- `dispatch_table_call`: Looks up a value in a table indexed by a number stored in a global variable and jumps to the corresponding routine.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `invoke_table_slot`: Looks up a slot in a table using a value derived from a register, then invokes the routine stored at that slot.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `invoke_indirect_table_2`: Looks up an entry in a table using a value stored in a global variable and then invokes the routine referenced by that entry.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:MakeDummy

- `initialize_program_environment`: Initializes a global structure, sets up a stack frame, and then runs a series of conditional setup routines that allocate memory, open libraries, and prepare data structures for the program.
- `store_value_and_call_routines`: Stores a passed value at a fixed offset from a computed base pointer, then conditionally decrements a counter and invokes a routine from a table, calling several routines from tables depending on the state of various fields before returning the stored value.
- `write_dummy_marker_file`: Walks two linked lists invoking callbacks for active nodes, then iterates a third list invoking per-entry callbacks based on status flags, and finally opens a file named DUMMY, writes the text DUMMY into it, and reports failure if the file cannot be created.
- `return_without_effect`: The routine has 8 instructions in the binary, but the decompiler reduces them to an immediate return, meaning the body performs no observable work beyond returning to its caller.
- `return_without_effect_2`: The routine executes eight instructions and then returns to its caller without performing any observable work.
- `drain_queue_list`: It walks a doubly linked list by following forward links, invoking a routine for each node until it reaches the end.
- `allocate_and_store_base_pointer`: It inspects a flag bit in a structure and, when set, calls another routine to obtain a base pointer.
- `process_callback_status`: Reads a status value from a structure and branches on its bits to either store a fixed constant, invoke a callback, or walk a linked list of callbacks in a loop, storing the resulting value into a structure field.
- `record_process_index`: Copies one argument into another slot, evaluates a value stored at a global offset, and then updates two fields of a record pointed to by a base register based on whether that value is zero or negative.
- `dispatch_buffer_result`: Calls a routine to obtain a status value, stores a fixed byte into a field when it is zero, and otherwise reads a pointer from a table to call another routine and store its result.
- `iterate_and_close_library`: Stores a fixed value at a fixed offset from a base pointer, then walks a linked list of entries invoking a routine for each one based on its flags.
- `parse_file_into_tokens`: Reads a text file into memory, splits it into whitespace-separated tokens while handling quoted strings and backslash escape sequences, and stores the resulting token pointers in a table.
- `copy_and_dispatch_buffer`: Copies a bounded number of bytes from one buffer into a stack buffer and null-terminates it, then inspects several fields of a structure and stores either a pointer to the stack buffer or a routine address into a table depending on the results.
- `copy_and_dispatch_routine`: It copies a bounded number of bytes from a source into a stack buffer and null-terminates the result.
- `resolve_index_from_list`: Resets a status field, then walks a singly linked list of pointers while decrementing a counter, returning the matching node or a null pointer when the bounds are exceeded.
- `copy_and_dispatch_routine_2`: Reads a flag word from a global structure and, when a certain bit is set, copies a bounded number of bytes from a source into a stack buffer and null-terminates the result.
- `find_byte_in_buffer`: Steps through a memory buffer comparing each byte to a target value, returning a pointer to the matching byte or null when the length is exhausted without a match.
- `append_string_to_buffer`: Locates the end of a destination string, then appends the bytes of a source string to it and writes a terminating zero byte.
- `copy_bytes_with_termination`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, then fills the remaining space up to a specified count with zero bytes using a counter.
- `allocate_aligned_memory_block`: Aligns the requested size up to a next multiple of four bytes, then walks a linked list of memory blocks looking for a free region large enough to satisfy the request.
- `invoke_node_callbacks`: Walks a linked list of nodes, invoking each node's callback when its status bits indicate it is active and its identifier differs from a stored value, then sets a flag and runs callbacks for a second linked list based on per-entry flags.
- `copy_string_with_limits`: Walks a null-terminated string, copying each character into a destination buffer while enforcing a maximum length stored in a structure.
- `store_byte_and_update_state`: Examines status bytes in a structure and branches on their bits to set flags, reset counters, or return early.
- `update_file_node_state`: Returns a failure flag by walking a linked list and aggregating helper return values when no node is supplied, otherwise resets a node's counters and status bytes, updating status bits based on helper return values and returning a status code.
- `do_nothing_return`: It performs no operations and simply returns to its caller.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)[cr PDX](Disk 1 of 6)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `format_string_buffer`: Constructs a formatted string by copying a source character sequence into a destination buffer, wrapping the content with double quotes and a trailing space.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `find_min_max_and_recurse`: Calculates the minimum and maximum values from a set of registers and recursively calls another routine with those values.
- `update_global_with_table_value`: Checks a bitmask against a register, and if a certain condition is met, updates a global variable using a value loaded from a table.
- `execute_conditional_function_calls`: Performs a series of conditional checks on a global variable and a pointer, calling various functions via offsets if certain conditions or non-zero values are met.
- `clear_global_variables`: Sets a global variable to zero and several other specific constant values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_struct_and_toggle_flag`: It initializes several fields of a structure with fixed values, then toggles and clears specific bits of a flag byte.
- `invoke_subroutine`: This routine calls another routine and then returns.
- `perform_fixed_point_division`: Performs division on 32-bit values, handling both integer and fixed-point arithmetic logic through bit shifting and loops.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `initialize_object_structure`: Initializes a data structure by calling a function pointer from an offset within a provided pointer, then populates several specific fields with fixed values.
- `initialize_audio_subsystem`: Initializes a series of data structures and tables by iterating through memory locations to set specific byte and word values.
- `clear_structure_flags`: Clears specific bits in a global variable and iterates through a table to call a subroutine if a bit is set, then clears bytes in that table.
- `initialize_and_clear_structures`: Iterates through a table to call a subroutine if a value is non-zero, then iterates through another table to clear specific bits and set a byte to a fixed value in each entry.
- `store_subroutine_address`: Stores a value from a register into a global variable and then stores the return address of the current routine into another global variable.
- `update_flag_from_input`: Reads a hardware register for input state, then sets or clears a specific bit in a global variable based on that value.
- `check_global_flag_bit`: Checks a flag bit in a global variable and returns one when the bit is clear, otherwise returns zero.
- `traverse_linked_list_of_structures`: Iterates through a list of structures, skipping entries that match a specific pattern until a null terminator is found.
- `execute_and_wait_for_pointer`: Assigns a parameter to a global variable, calls a function via an offset from that variable, and enters a loop until the variable becomes zero.
- `initialize_system_state`: Performs a sequence of operations including clearing bits in a global variable, setting multiple bytes to a specific value, and updating a bit in a global variable.
- `update_offset_value`: If a parameter is less than a global variable, it clears certain bits in that variable and adds a scaled version of the parameter to it.
- `update_structure_fields`: Calls a function via a pointer in a global variable and, if the return value is non-zero, calls another function and updates a structure with the returned values.
- `clear_linked_list`: Traverses a linked list starting from an offset in a table, calling a function for each element while clearing the list entries.
- `reset_table_entries`: Clears specific entries in a table based on an index and calls a function to reset a value if a certain field is non-zero.
- `process_linked_list`: Performs two loops of a subroutine call, then traverses a linked list to call a function pointer stored in each element before clearing the list entries and resetting a global variable.
- `update_structure_fields_3`: Updates a structure or linked list of objects by calling a function pointer stored in each element and clearing or updating specific flags or fields.
- `search_pairs_for_value`: Scans an array of pairs, returning a null pointer when it hits an empty slot or the matching pair when found.
- `call_global_function`: Checks if a global variable is non-zero and, if so, calls a function pointer located at a fixed offset from that variable.
- `execute_subroutine_calls`: Retrieves a pointer from a memory offset and calls two functions located at specific offsets within that pointed-to structure.
- `execute_indirect_function_pointers`: Fetches a pointer from a memory offset and executes two functions located at specific offsets within the pointed-to structure.
- `invoke_empty_routine`: Loads a function pointer from a table offset and calls it, where the called routine is empty.
- `do_nothing_2`: this routine is empty and performs no operations before returning.
- `update_graphics_and_state`: Updates a global variable or pointer using a value retrieved from a table and performs conditional checks to branch into different subroutines.
- `load_and_process_font_data`: Allocates memory to load font data from a file and processes it through a series of state-dependent loops.
- `negate_pointer_and_trap`: Negates the value pointed to by a register and executes a trap instruction.
- `route_record_index`: Compares a value against a series of constants and branches to a shared block or calls another routine if a match is found.
- `update_global_value`: Modifies a global variable based on its current value through several conditional checks.
- `copy_data_to_table`: Copies a block of data from a provided pointer to a fixed destination table and then calls another routine twice with calculated arguments.
- `select_priority_entity`: Iterates through a linked list of entities to identify and select specific objects based on bitmask flags and priority criteria.
- `execute_three_calls`: Calls another routine three times in succession.
- `execute_three_calls_alt`: Performs a sequence of three calls to another routine.
- `execute_three_calls_seq`: Executes a series of three calls to a routine.
- `execute_three_calls_repeat`: Repeats a call to another routine three times.
- `execute_three_calls_triple`: Triggers a sequence of three calls to another routine.
- `return_empty_routine`: the routine is empty and performs no operations.
- `execute_three_calls_iter`: Invokes another routine three times.
- `execute_three_calls_final`: Executes a chain of three calls to another routine.
- `return_immediately`: the routine contains no instructions and immediately returns to its caller.
- `execute_empty_routine`: The routine contains no instructions.
- `execute_empty_routine_2`: This routine performs no operations.
- `do_nothing_3`: the routine is empty and performs no operations before returning.
- `execute_empty_routine_3`: This routine contains no executable code.
- `execute_empty_routine_4`: the routine performs no operations and simply returns or exits.
- `execute_empty_routine_5`: The routine is empty and performs no operations.
- `empty_routine`: the routine contains no executable instructions.
- `process_resource_object_lifecycle`: Iterates through a collection of objects to execute specific function pointers based on a state flag.
- `chain_three_calls`: The routine makes three consecutive calls to the same routine and then returns, with no other work visible in the decompiled body.
- `unresolved_data_stub`: The decompiled body consists only of a bad-instruction trap, indicating that the entry point was reached as an address reference but the bytes there do not decode to valid 68000 code.
- `reset_global_data`: Clears two words in a table or memory area and resets a global variable to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_2`: This routine contains no executable instructions and performs no operations.
- `execute_empty_routine_6`: The routine performs no operations and contains no instructions.
- `execute_empty_routine_7`: the routine contains no executable instructions and calls itself recursively.
- `parse_instruction_stream`: Iterates through a buffer of bytes, decoding or skipping certain byte sequences and writing or skipping bytes to a destination buffer.
- `execute_empty_sequence`: this routine contains no executable instructions and acts as a sequence of empty calls to itself.
- `execute_recursive_calls`: The routine calls itself recursively without performing any other operations or modifying any registers or memory.
- `execute_empty_routine_8`: the routine contains no executable instructions and performs no operations.
- `set_bit_flags_at_offsets`: Updates a bit in a byte at a pointer-referenced address and two other bytes at fixed offsets from that same pointer, using a bitmask determined by a register or a bit-test of a word.
- `empty_routine_3`: the routine is empty and contains no executable instructions.
- `null_routine`: This routine is empty and contains no valid instructions.
- `execute_recursive_calls_4`: Performs a series of nested recursive calls to another routine, checking for zero return values before proceeding.
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
- `halt_execution`: the routine contains invalid or undefined instruction data, causing execution to stop or fail.
- `repeat_call_three_times`: The routine consists of a single instruction whose pseudo-code shows it calling a routine three times in succession before returning.
- `copy_string_and_call_function`: Sets a bit in a byte, then copies a sequence of bytes from a source to a destination, wraps them with certain characters, and executes a function pointer stored in a global variable.
- `empty_routine_4`: The routine contains no executable instructions and performs no operations.
- `execute_empty_routine_9`: The routine contains no executable instructions.
- `return_empty_routine_2`: The routine contains no valid executable instructions and performs no operations.
- `execute_empty_routine_alt`: This routine is empty and performs no operations.
- `execute_empty_routine_alt2`: This routine contains no instructions and does nothing.
- `return_from_subroutine`: the routine executes a single instruction that terminates the current execution flow and returns control to the caller.
- `return_empty_routine_5`: This routine is empty and contains no executable instructions.
- `execute_infinite_loop`: The routine enters an infinite loop that does nothing.
- `execute_function_loop`: The routine iterates a certain number of times, calling another routine twice per iteration using a pointer or address retrieved from a global variable.
- `undefined_instruction_trap`: The routine executes an invalid or undefined instruction, causing a break in the instruction stream or a processor exception.
- `empty_routine_5`: This routine contains no executable instructions.
- `process_string_buffer`: Iterates through a source string and copies or modifies characters into a destination buffer until an escape or null terminator is encountered.
- `execute_recursive_calls_2`: Empty routine that performs a sequence of recursive calls to itself.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_empty_sequence_2`: This routine executes a sequence of empty instructions before returning, effectively acting as a no-op or a gap in the instruction stream.
- `process_deferred_value`: Flushes a pending value held in a global slot by running another routine on it, then processes a passed value and stores the resulting deferred value back into that global slot.
- `execute_nested_logic`: Performs a sequence of recursive calls and conditional logic based on a return value.
- `copy_and_initialize_buffer`: Copies a fixed number of bytes from a source pointer to a destination buffer and then calls another routine with a global variable and a constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `swap_and_clear_buffer`: Swaps the contents of a global word with a parameter value and then clears a buffer whose length and pointer are read from adjacent global variables.
- `clear_large_memory_blocks`: Iteratively clears blocks of memory of a fixed size (64000 bytes) or a remaining partial block by writing zeroed bytes.
- `clear_memory_blocks`: Iteratively fills a large block of memory with zeroed bytes using a pointer and a counter.
- `fill_memory_block`: Fills a block of memory with a specific byte for a specified number of iterations.
- `swap_global_word`: Swaps the contents of a global word with a parameter value and returns the previous value.
- `swap_flag_and_set_offsets`: Swaps a global flag value with a passed-in value, restores the previous one on return, and stores several word-sized fields into a structure indexed by a parameter.
- `copy_and_call_routines`: Copies forty bytes one at a time from one pointer to another, then invokes two routines through function pointers stored in a structure.
- `copy_fixed_bytes`: Copies forty bytes one at a time from one pointer to another.
- `initialize_from_pointer`: Initializes several global fields from a pointer, or resets them to defaults when the pointer is null, returning the previous stored pointer.
- `set_bit_and_return_previous_state`: Sets a bit in a global byte based on a provided index and returns a value where the least significant bit indicates if that bit was previously set.
- `clear_and_test_bit`: Clears a bit in a global byte and returns a value indicating if that bit was previously set.
- `lookup_and_adjust_value`: Looks up an entry in a table using a passed value and, depending on the result, either adjusts a value taken from the stack or clears a field through a pointer.
- `swap_flag_and_process_game_state`: It swaps a global flag with a passed value, copies forty bytes from one pointer to another, and initializes global fields from a pointer or resets them to defaults.
- `update_flag_and_clear_fields`: Swaps a global flag value with a new value and restores the previous one on return, then clears two additional word-sized fields in a structure.
- `find_max_fixed_point_index`: Repeatedly evaluates three fixed-point products, tracks the largest product seen with its index, and stops when a table entry holds a sentinel value or the index exceeds a limit.
- `append_string_to_buffer_2`: finds the null terminator of a destination buffer and copies a source string to the position immediately following it.
- `traverse_entity_list`: Walks a global linked list of structures and calls another routine if a specific bitmask condition is met and a certain value difference is non-zero.
- `compute_fixed_point_product_2`: Calculates a 32-bit fixed-point product using the lower and upper halves of two 32-bit registers.
- `execute_and_propagate_result`: Executes a routine with a passed value and then executes it without arguments, storing the high half of the second result in a local slot.
- `store_indirect_value`: Copies a 32-bit value from a location four bytes before a register into the address held in another register, then returns.
- `reset_global_variable`: Sets a global variable to one and then immediately sets it to zero.
- `swap_flag_and_compute_offsets`: Swaps a global flag value with a passed-in value and restores the previous one on return.
- `recurse_and_return_value`: The routine calls itself recursively and returns the value returned by the recursive call.
- `reset_dispatched_routine_state`: Reads an index from a global structure, calls a routine with a computed pointer and adjusted value, then clears several fields in that structure.
- `copy_value_from_pointer`: copies a 32-bit value from a location four bytes before a register into the address held in another register, then returns.
- `process_compressed_text_stream`: Runs a small state machine over a compressed text buffer, decoding escape sequences such as percent and 's' markers while advancing a per-object counter.
- `execute_recursive_graphics_logic`: Checks a global pointer, executes a routine through it, and then performs recursive calls using values derived from a global structure.
- `advance_and_truncate_string`: Decrements a per-object counter and, if it reaches zero, scans a string backward to find a space, null-terminating it and copying a trailing portion into a fixed buffer.
- `fetch_next_compressed_byte`: Fetches the next byte from a chain of pending buffers, advancing through them until a value isfound.
- `reset_global_variable_3`: Checks if a global variable is zero and, if so, calls another routine to update it, then resets the global variable to zero before returning its previous value.
- `clear_global_variable`: Sets a global variable to zero twice.
- `forward_arguments`: It forwards its three arguments to another routine and then returns, performing no work of itsown.
- `call_copy_buffer_interleave_2`: Calls a routine that copies five consecutive bytes from a source buffer into five scattered positions of a destination array, repeating for each row of a two-dimensional structure.
- `forward_arguments_2`: It forwards its three arguments to another routine and then returns, performing no work of its own.
- `clear_byte_buffer`: Reads a length from a global variable and a pointer from the adjacent location, then clears that many consecutive bytes starting at the pointer.
- `clip_line_to_bounds`: Clamps a line segment's endpoints to a rectangular region using Cohen-Sutherland-style boundary flags and calculates new intersection coordinates.
- `check_bounds_flags`: Compares two coordinate fields against stored limits and accumulates flag bits indicating whether each coordinate falls below or above those limits.
- `store_bounds_values`: Stores four values into four consecutive global variables, pairing the low and high bytes of two input values into separate storage slots.
- `clamp_and_draw_lines`: It clamps two coordinate pairs so the second lies at or below the first, then issues four line-drawing calls with the resulting corner values.
- `execute_graphics_calls`: Retrieves a pointer from a global variable and executes three function calls at specific offsets from that pointer.
- `register_interrupt_server`: Registers a routine with the operating system by calling a library function with a pointer taken from a global structure, then returns.
- `reset_global_variable_2`: Sets a global variable to zero and certain maximum values.
- `evaluate_conditional_flags`: Runs a sequence of conditional checks that call other routines and combine their results into a byte flag, returning that flag.
- `lookup_routine_pointer`: Multiplies an index by four and returns the pointer stored at that offset in a table of routine pointers.
- `reset_display_context`: Conditionally invokes a callback if a global flag is set, then resets a structure by writing a constant, a zero, and a pointer to certain offsets.
- `execute_recursive_calls_3`: Calls itself recursively three times.
- `clear_global_pointer`: Calls two functions and then sets a global variable to zero if it was non-zero.
- `call_rem_int_server_2`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `process_string_handlers`: When a flag is clear, it runs two setup routines and then repeatedly walks a null-terminated string, calling a handler for each segment and advancing the pointer past each chunk.
- `accumulate_offset_differences`: Accumulates differences between pairs of global values depending on which of two bit flags are set in the argument, then returns the total.
- `check_available_memory_bounds`: Checks if a certain amount of memory is available and updates a global variable based on the remaining space.
- `return_zero_constant`: Returns a constant zero value.
- `extend_range_with_flags`: Checks a flag bit in an argument to determine which bits to set in a returned memory location, while clamping an input value against a global range, advancing that range, and calling a memory allocation routine.
- `recurse_and_return`: This routine calls itself and returns the resulting value.
- `loop_until_nonzero_result`: Loops, repeatedly calling a helper and remapping its result until a non-zero value is produced, then returns that value.
- `find_first_nonzero`: Calls a helper routine and returns the first non-zero result encountered while incrementing a counter.
- `recursively_call_routine`: Calls itself and returns the result of the recursive call.
- `loop_until_nonzero_mapped`: Loops, repeatedly calling a helper and mapping its result until a non-zero value is produced, then returns that value.
- `loop_until_nonzero_wrapped`: Loops calling a helper and mapping its result, incrementing a wrapped counter when the mapped value is zero, and returns the first non-zero mapped value or zero.
- `map_argument_with_lookup_and_flags`: Maps an input argument through a lookup table or conditional offsets, then applies bitwise flag-based offsets to the resulting value if the input is non-zero.
- `map_argument_table_flags`: Reads a byte from a lookup table using the low bits of the argument, remaps some values, then sets result bits according to flag bits in theargument.
- `process_circular_list_entry`: Locks interrupts, walks a circular list of entries, updates the head index and copies data from the current entry, then releases interrupts and returns a value.
- `load_table_entry`: Reads a four-byte value from a table indexed by a global variable and returns it, or zero when two global variables are equal.
- `advance_counter_locked`: Locks the system while calling two routines from a global structure and advances a counter modulo sixteen, skipping a value when it matches one of two codes.
- `update_bounds_and_check_collision`: Calculates a coordinate offset and updates a structure's position and size fields, then checks if a value falls within those bounds.
- `update_structure_fields_2_2`: Computes a coordinate offset from passed values and writes a set of position and size fields into a structure, then repeatedly updates those fields with incremented offsets while waiting for a global flag to clear.
- `update_structure_fields_2`: Waits for a flag to clear, writes several fields of a structure, then waits again and overwrites some of those fields before returning.
- `reset_and_update_structure`: Waits for a flag in a global variable to clear, initializes several fields in a structure to zero or fixed values, waits for the flag to clear again, and then updates those fields by adding a value or a previously stored value.
- `store_register_pair`: It calls another routine, then copies the two 32-bit result registers (D0 and D1) into a global variable.
- `setup_blitter_operation`: Initializes a blitter operation by waiting for the blitter idle flag, programming the blitter control, size, and address-modify registers, and loading a palette entry through the color registers.
- `increment_structure_field`: Waits for a flag to clear, writes several structure fields including a value, then waits again and increments one stored field before returning.
- `store_stack_value_then_call`: Stores a value taken from the stack into a global variable, then calls another routine and returns its two data-register results unchanged.
- `parse_string_and_update_bounds`: Iterates through a buffer, updating a global variable and certain memory locations based on the value of the current byte.
- `tiny_ori_4`: Small routine of 1 instruction that merges 239 into d0.
- `return_from_subroutine_2`: the routine executes a single instruction that returns or exits the current execution flow.
- `copy_buffer_interleave_2`: Copies five consecutive bytes from a source buffer into five scattered positions of a destination array, repeating for each row of a two-dimensional block.
- `copy_buffer_interleave`: Copies five bytes from a source buffer into five scattered positions of a destination array, repeating for each row of a two-dimensional block.
- `scatter_buffer_bytes`: Writes a single byte from a source buffer into a destination array at a position determined by a secondary offset, repeating for a set number of bytes or rows.
- `gather_buffer_interleave`: Reads five scattered positions from a destination array and writes them into five consecutive bytes of a source buffer, repeating for each row of a two-dimensional block.
- `initialize_display_context`: Initializes a display context by caching screen setup routines, clearing interrupt state, and zeroing a callback, then allocates and configures per-entry memory blocks while linking them into a table and updating their bookkeeping fields.
- `sum_string_byte_values`: Walks through a null-terminated string and accumulates the byte values returned by a lookup routine for each character, then returns the running total.
- `fetch_table_value_or_default`: Returns a fixed value of ten when a global flag is clear, otherwise it indexes a byte table using a scaled parameter and returns the fetched byte.
- `increment_counters_conditionally`: Increments a sequence of words pointed to by a register, conditionally bumping the second and third words based on bits in a global flag, and decrements the fourth word without going below zero.
- `invoke_double_pointer_routine`: Reads a pointer from a global structure, then invokes the routine stored at an offset from that pointer twice in succession.
- `register_table_entry`: Finds the first empty slot in a fixed-size table of records and populates it with a pointer, a code pointer, and several status fields.
- `handle_indexed_cleanup`: Increments a global counter, reads a short from a table indexed by a passed value, and either invokes a stored routine pointer or clears a field in a structure when a flag is set, then marks the entry as handled and decrements the counter.
- `get_conditional_value`: Checks if a parameter or a value in a table falls within a certain range, then returns a value from a table or a default constant.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies (a7)+ to d7, then returns.
- `accumulate_minimal_difference`: Tracks the smallest positive difference between a stored value and a passed value, then accumulates that amount into a running total either in one step or by repeatedly invoking a stored routine until the amount is exhausted.
- `accumulate_and_validate_table_index`: It validates an index, checks a bitmask in a table, and if certain conditions are met, enters a loop that accumulates values and updates a global variable or a table entry.
- `update_table_and_accumulate_values_2`: Checks a bitmask in a table and, if a condition or a certain value is met, increments a global variable and enters a loop that accumulates values and updates a table or a pointer.
- `update_table_and_accumulate_values`: Checks a bitmask in a table and, if a condition or a certain value is met, increments a global variable and enters a loop that accumulates return values from a routine into a table and a local variable while updating a secondary table with a minimum value.
- `update_table_with_return_value`: Increments a global variable and updates a table entry with a value returned from another routine while checking a secondary condition.
- `update_table_and_decrement_global_2`: Decrements a global variable and updates a table entry with a value from a library call or a pointer-based lookup.
- `update_table_and_decrement_global`: Decrements a global variable and updates a table entry with a value from a library call or a pointer-based lookup.
- `decrement_and_dereference`: Decrements a global variable and returns a value from a memory location offset from a register.
- `invoke_double_pointer_dispatch`: A longer routine that invokes a routine reading a pointer from a global structure twice in succession, validates a table index before dispatching, and also dispatches display callbacks.
- `check_sixteen_bit_flags`: Splits a 32-bit value into its two 16-bit halves and checks each half in turn.
- `register_string_category`: Looks up a category index for a given string and, if the category has no active entry, copies the string into a stack buffer and registers it.
- `capture_and_store_result`: Runs a routine twice with different arguments, capturing the value returned by the second call, stores that captured value into a global variable, and then returns it to the caller.
- `resolve_string_category`: Finds the index of a matching string and returns a value built from its category, or the stored pointer of that entry when the category is marked as active.
- `swap_global_with_parameter`: Swaps the contents of a global variable with a value passed in a parameter, returning the previous value.
- `traverse_and_clear_list`: Recursively traverses a linked list, clearing a pointer field in certain entries if a condition is met.
- `traverse_and_clear_linked_list`: Increments a global variable and recursively calls itself to traverse a linked list, clearing a pointer field in each matching entry.
- `reset_and_process_records`: Clears a fixed block of short records and a parallel block of byte flags, then walks a table of five-field records to modify flags and invoke routines on associated pointers.
- `return_empty_routine_3`: The routine is empty and performs no operations before returning.
- `validate_index_and_dispatch`: Checks if a provided index is within a certain range, and if a corresponding entry in a table is non-zero, invokes a stored routine pointer.
- `find_matching_string_index`: Walks an array of null-terminated pointers, comparing each string against a given argument, and returns the index of the first match or a sentinel value when none is found.
- `adjust_and_process_string_length`: Computes a string length adjusted by one when a flag is set, then passes the result to another routine and invokes a library call.
- `dispatch_display_callbacks`: Invokes a stored routine pointer, then calls two other routines in sequence, and finally returns without producing a value.
- `remove_and_compact_entries`: Scans a table of entries, freeing those that are marked and empty, then compacts the remaining entries and reduces the stored count by the number removed.
- `allocate_tracked_memory`: Allocates a tracked block from the exec library when a flag is clear, linking it into a free list and updating bookkeeping counters, otherwise returns a previously allocated block.
- `handle_allocation_failure`: Executes a sequence of setup and reset routines, including clearing a global variable, and returns a zero result.
- `remove_entry_from_linked_list`: Adjusts accounting fields based on a flag in the passed value, removes a matching entry from a linked list of globals, optionally calls a stored routine, decrements a global counter, and returns the located entry or zero.
- `call_avail_mem`: Library stub: loads its arguments and the exec library base, calls AvailMem and returns its result to the caller.
- `compute_available_memory_delta`: Requests available memory from the operating system, accumulates offset differences, and returns the requested amount minus the accumulated total.
- `free_linked_list_entries`: Recursively frees each entry in a linked list and then clears the head pointer to empty the list.
- `initialize_pointers_and_return`: Loads two pointers from a global structure and initializes their pointed-to words, then returns without any further action.
- `initialize_and_scale_pointers`: Loads two pointers from a global structure and initializes their pointed-to words, then conditionally calls a routine and rescales the values when the inputs fall within range.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `check_entity_flag_and_process`: Retrieves a pointer from another routine and, when that pointer is valid, checks a specific bit in the second byte of the pointed-to structure.
- `decode_nested_instruction_stream`: Iterates through a byte stream to decode and execute a sequence of instructions, supporting recursive calls for nested or conditional instruction blocks.
- `interpret_bytecode_stream`: Iterates through a stream of opcodes, updating a pointer and a counter, and executes conditional jumps or calls to other routines based on the decoded instruction type or a specific terminator byte.
- `parse_bytecode_stream_3`: Iterates through a byte stream to interpret opcodes and jump targets, recursively processing or skipping instructions.
- `perform_no_operation`: The routine is empty and performs no executable instructions before returning.
- `parse_bytecode_stream_2`: Iterates through a byte stream to interpret opcodes and jump targets, recursively processing or skipping instructions.
- `parse_bytecode_stream`: Iterates through a stream of bytes, interpreting certain values as opcodes or jump targets, and recursively calls itself to process or skip data.
- `parse_string_stream`: Iterates through a buffer of bytes to decode or skip certain characters, updating a pointer and a running length or checksum.
- `parse_instruction_stream_2`: Iterates through a buffer of bytes, decoding or executing instructions based on certain byte values and updating a pointer or a running sum.
- `return_empty_routine_4`: the routine contains no executable instructions and performs no operations before returning.
- `validate_command_argument_range`: Parses a string or range-delimited list of characters to check if a specific value falls within any defined ranges or matches a specific character.
- `parse_command_range_string_2`: Parses a string or a range-delimited list of characters to check if a specific value falls within any defined ranges or matches a specific character.
- `parse_command_range_string`: Parses a string or a range-delimited list of characters to check if a specific value falls within any defined ranges or matches a specific character.
- `parse_command_token_range`: Parses a string or a range-delimited list of characters, checking if a specific value falls within any defined ranges or matches a specific character.
- `parse_string_range_tokens_2`: Parses a string or a range-delimited list of characters, checking if a specific value falls within any defined ranges or matches a specific character.
- `parse_string_range_tokens`: Parses a string-based command or range list, handling special bracketed syntax and hyphen-delimited character ranges.
- `decrement_target_and_return_incremented`: Decrements a value at a memory location pointed to by a pointer and returns an incremented value.
- `decrement_pointer_target_value`: Decrements a word at a memory location pointed to by a pointer-based offset and stores the result.
- `decrement_and_store_word`: Decrements a word from a pointer-based location and stores the result into a memory location offset from a pointer.
- `parse_bitstream_opcodes`: Decodes a variable-length bit-encoded stream to extract or update integer values.
- `decode_variable_length_bitstream`: Decodes a bit-encoded or bit-masked stream of values into a local integer, updating a pointer-based destination with the result.
- `decode_bitstream_to_value`: Decodes a variable-length bit-encoded or bit-masked stream of values into a local integer, updating a pointer-based destination with the resulting value.
- `accumulate_list_index`: Iterates through a linked list of structures, decrementing a counter-based loop limit and accumulating a masked value from a pointer-based offset into a destination.
- `accumulate_signed_list_index`: Iterates through a linked list while checking for specific character values to determine a sign, then accumulates masked values from a pointer-based offset into a destination.
- `traverse_linked_list_and_accumulate_index`: Iterates through a linked list of structures, decrementing a counter-based loop limit and accumulating a masked value from a pointer-based field into a local variable.
- `process_loop_and_store_result`: Executes a loop that calls another routine, updating a local variable and a table entry, then stores a result into a pointer-based location.
- `store_word_and_increment_pointer_2`: Stores a word from a memory location through a pointer and returns an incremented value.
- `store_word_and_increment_2`: Stores a word from a memory location through a pointer and returns an incremented value.
- `update_bit_stream_buffer`: Updates a bit-buffer or bit-stream by shifting existing bits and appending new bits from a sequence.
- `accumulate_value_in_loop`: Performs a loop that accumulates a value into a local variable using a bitwise operation and updates a pointer-based location if a condition is met.
- `update_pointer_and_store_value`: Checks a value at an offset from a pointer and, regardless of the condition, stores a value from a previous memory location into a destination address pointed to by a secondary pointer.
- `store_word_and_increment_pointer`: Stores a word from a memory location into a destination address calculated from a pointer, then returns an incremented value.
- `parse_bitmasked_attribute_stream`: Iteratively reads a stream of bitmasked values from a pointer-based source, applying or shifting offsets based on specific bit flags, and accumulates the result into a destination buffer.
- `extract_and_accumulate_bitmasked_values`: Iteratively reads values from a pointer-based source, checks bitmasks to decide whether to apply certain offsets, and accumulates or shifts these values into a destination buffer.
- `parse_and_shift_values_2`: Checks bitmasks on a pointer, shifts a value left, and or-masks values from a pointer-based offset before storing a final result to a destination.
- `parse_and_shift_values`: Checks bitmasks on a pointer, shifts a value left, and or-masks values from a pointer-based offset before storing a final result to a pointer target.
- `store_value_to_pointer_target`: Checks if a pointer is non-null and copies a word from a local variable to a destination address, then increments a pointer.
- `copy_word_and_increment_pointer_2`: Copies a word from a memory location to a destination address calculated from a pointer and increments a pointer.
- `copy_word_and_increment_pointer`: Copies a word from a memory location to a destination address calculated from a pointer, then increments a register used as a pointer or counter.
- `update_buffer_from_routine_calls`: Calls another routine in a loop, updating a buffer and a counter, and handles a termination condition or a specific return value.
- `update_word_on_condition`: Stores a word into a memory location offset from a pointer if a certain value is not certain, otherwise stores a constant value.
- `increment_and_return`: Increments a register value and returns it.
- `process_input_loop`: Iteratively updates a counter and a memory byte based on a function return value, continuing until a condition involving a counter or a bitmask check fails, then clears a memory location and stores a value.
- `parse_char_range_string_2`: Parses a string or range of characters, checking if a target byte falls within any identified range or matches a specific character.
- `parse_char_range_string`: Parses a string or a range of characters delimited by hyphens, checking if a target byte falls within any identified range or matches a specific character.
- `parse_string_with_hyphen_ranges`: Parses a string or a range of characters delimited by hyphens, updating a pointer or a value if a character or range matches a target byte.
- `scan_input_buffer_for_key_range`: Iterates through an input buffer to find a matching byte or a byte within a specific range, such as a hyphen-delimited range.
- `search_input_buffer_for_key`: Iterates through an input buffer to find a matching byte or a byte within a specific range.
- `scan_input_buffer_for_match`: Iterates through a buffer of input bytes to check if a specific byte or range of bytes matches a value retrieved from another routine.
- `search_and_update_string_buffer`: Searches a memory range for a specific byte value and updates a pointer-based buffer with that value if a match or a specific delimiter is found.
- `update_string_pointer`: Iterates through a memory buffer to find a specific character, updating a pointer and a counter or clearing a value at a pointer offset if a certain condition is met.
- `clear_and_update_pointer_target`: Checks if a value at a pointer offset is non-zero and clears it, then writes a register value to a new pointer location and returns an incremented pointer or counter.
- `update_string_pointer_and_counter`: Updates a pointer-based memory location with a value, increments a counter, and checks if a certain character or range or a pointer or a counter or a value matches a certain condition.
- `store_word_and_increment_3`: This routine stores a word into a memory location at an offset from a pointer and returns an incremented value.
- `store_word_and_increment`: Stores a word into a memory location at an offset from a pointer and returns an incremented value.
- `parse_format_string`: Parses a format string containing tokens like #, +, -, *, leading zeros, ., l, h, p, x, and digits, advancing a pointer through the string while reading values from a parameter list.
- `process_format_string`: Walks through a character string one byte at a time and stops when it finds a percent sign.
- `iterate_table_entries`: Steps backward through a global table of structures, skipping entries that are empty or have a particular flag bit set, and calls another routine for each matching entry.
- `tokenize_quoted_string`: Walks through a text buffer, skipping whitespace and splitting the content into up or fewer tokens while treating double-quoted runs as single tokens, writing a pointer to each token into a stack table and null-terminating each token in place.
- `execute_conditional_function_pointers`: Checks if a global pointer is non-null before calling it, then proceeds to execute a sequence of indirect function calls or calls to functions located at fixed offsets from that pointer, depending on whether the pointer is null or non-null.
- `execute_vtable_loop`: Checks a condition or a flag in a parameter, then iterates through a loop calling functions via pointers or offsets from a table.
- `swap_memory_blocks`: Swaps a specified number of bytes between two memory locations.
- `convert_uppercase_chars`: Iterates through a null-terminated string and converts certain uppercase characters to lowercase based on a flag in a global lookup table.
- `copy_and_zero_memory_2`: Copies a string from a source to a destination and then fills any remaining space in the destination buffer with null bytes until a certain length is reached.
- `copy_and_zero_memory`: Copies a block of memory from a source to a destination and fills any remaining space in the destination block with null bytes.
- `zero_memory_block`: Fills a memory block with zeros using a counter.
- `compare_case_insensitive_strings`: Compares two strings case-insensitively and returns the difference between the first mismatching characters or zero if equal.
- `copy_string_to_buffer`: Finds the end of a destination buffer and copies a null-terminated string into it.
- `find_byte_in_string`: Searches a null-terminated string for a specific byte and returns a pointer to its location, or zero if not found.
- `call_and_return_value`: It forwards its argument to another routine and returns whatever that routine produces.
- `find_last_byte_in_string`: Walks a null-terminated string looking for a specific byte and returns a pointer to its last occurrence.
- `skip_bytes_with_flag`: Increments a pointer while a specific bit in a global lookup table or a byte at the pointer's location remains set.
- `tiny_move_movea_rts`: Small routine of 3 instructions that copies a3 to d0, then copies (a7)+ to a3, then returns.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `encode_base3_string`: Encodes an integer into a base-3 string represented by ASCII digits and writes it to a buffer in correct order.
- `convert_number_to_ascii`: Converts an integer into ASCII digits and writes them in reverse order to a buffer.
- `expand_and_parse_string`: Expands a value using a lookup table, reverses the resulting bytes into a buffer, and parses a signed integer from a string while updating a pointer.
- `expand_nibbles_to_buffer`: Expands a value nibble-by-nibble using a lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_signed_integer`: Parses a string containing an optional sign and digits, calculates a signed integer value, and stores it in a pointer location.
- `parse_signed_integer_string`: Parses a string of digits, calculating a numeric value, and stores the result in a pointer, while also handling a potential negative sign.
- `initialize_command_parser`: Clears a global counter, stores a pointer to that counter in another global, and calls a routine that parses a command string using that pointer and a stack slot.
- `increment_and_store`: Increments a global pointer and stores a byte from a parameter into the location pointed to by the incremented global pointer.
- `format_string_to_pointer`: Sets a global pointer to a supplied address, processes a format string, and stores a zero byte at the location pointed to by the updated global pointer.
- `copy_memory_block_2`: Copies a specified number of bytes from a source address to a destination address, using a reverse-order copy if the source address is less than or equal to the destination address to prevent overwriting, and a forward-order copy otherwise.
- `validate_linked_entry`: Calls another routine to retrieve a pointer and recursively checks the next entry in a linked structure if the pointer is not null and a certain global variable is zero.
- `process_entity_list_flags`: Walks a global linked list of structures, testing if a specific bit is cleared and another bit is set in a status byte, then calculates the difference between two integer fields.
- `invoke_global_callback`: Invokes a callback through a pointer in a global table and, when a global flag isset, recursively calls itself first.
- `dispatch_callback_with_adjustment`: Invokes a callback through a pointer in a global table and, when a unable global flag is set, recursively calls itself first.
- `set_structure_bitmasks_and_call`: It writes a full-byte bitmask and two full-word bitmasks to a structure and then calls a function pointer located at a fixed offset within that structure.
- `cleanup_display_context`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `invoke_method_if_present`: It checks whether a global pointer is non-null and calls another routine when it is, then invokes a function pointer stored within a structure.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `divide_fixed_point_value`: Performs a complex fixed-point division or modulo-based calculation using bit-shifting and normalization or fallback logic based on the sign and value of registers.
- `extract_high_word`: Returns the high word of a 64-bit value returned by another routine.
- `perform_fixed_point_division_2`: Performs a complex fixed-point division using bit-shifting and normalization or a fallback division method.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `allocate_and_set_node`: It allocates memory for a structure through a library call and, if successful, initializes certain bytes and a dword with a provided parameter.
- `create_message_node`: Checks a global table via a function call, and if the result is not a sentinel, calls a second routine and a third routine to initialize a new node with parameters and a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_sound_and_blitter`: Initializes the four sound channels by writing their registers and sets up the blitter, then waits for a vertical retrace before returning.
- `load_animation_frames`: Drives a small state machine that loads animation frames for a set of objects.
- `load_animation_frames_2`: Runs a state machine that loads animation frames for a series of objects.
- `blitter_state_machine`: Runs a state machine that sets up blitter parameters and then copies sprite data from a graphics table into a screen buffer.
- `load_blitter_table`: Loads word values from a table into a set of custom blitter registers and advances an index that wraps around after reaching its limit.
- `accumulate_nibble_differences`: It compares the high nibble, middle nibble, and low nibble of two values one group at a time.
- `tiny_ori_2`: Small routine of 1 instruction that merges 12 into d0.
- `single_instruction_target`: The routine has a single instruction and is entered only by taking its address for move and subtract-quick operations, so it functions as a data target rather than performing work.
- `tiny_ori_3`: Small routine of 1 instruction that merges 168 into d0.
- `store_table_byte`: Loads a byte from a table indexed by a value in a register and stores that byte through a pointer held in another register.
- `nine_instruction_target`: The routine contains nine instructions and is entered only by taking its address for a move operation, marking it as a data target used by a single caller.
