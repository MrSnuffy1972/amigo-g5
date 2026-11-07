# Game 1826 (dune ii - battle for arrakis (1993)(virgin)(m3)(disk 1 of 6)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01826). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (964160 top-level bytes in 53 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 24 of 24 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 613 of 613 named (0 author symbols), 613 of 613 described (75 by rule, 529 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3314 of 3314 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 23268 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 19894 of 19894 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 522785 |
| code | 62604 |
| data | 344017 |
| padding | 11714 |
| unused | 23040 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:Install | 7 | 45 | 2 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:DuneII | 2 | 267 | 8 |
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
| Intro | 2 | 267 | 8 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:Add16k | 3 | 475 | 32 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:MakeDummy | 2 | 1275 | 25 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 144 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1645 | 51 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1830 | 40 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7031 | 218 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:DUNE.ENG (raw code file, load address unknown) | 1 | 0 | 0 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:DUNE.FRE (raw code file, load address unknown) | 1 | 0 | 0 |
| Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:DUNE.GER (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address (play-long-joy): 72 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 388 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 40 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2355 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dune ii - battle for arrakis (1993)(virgin)(m3)(disk 1 of 6)[cr skr]/Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:DUNE.ENG: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- dune ii - battle for arrakis (1993)(virgin)(m3)(disk 1 of 6)[cr skr]/Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:DUNE.FRE: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- dune ii - battle for arrakis (1993)(virgin)(m3)(disk 1 of 6)[cr skr]/Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:DUNE.GER: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- dune ii - battle for arrakis (1993)(virgin)(m3)(disk 1 of 6)[cr skr]/Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- dune ii - battle for arrakis (1993)(virgin)(m3)(disk 1 of 6)[cr skr]/Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:DuneII: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 23268 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Intro

## Copy protection found

- `execute_unreached_code` (More): This routine is unreached and contains a gap that decodes as code. Kind: unreachable code/indirect jump trap. Pass path: the routine must complete without crashing or triggering a trap; the 'unreached' code is likely the payload or the next stage of a loader that only executes if a previous integrity check passes..

## Functions


### Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:Install

- `run_hdinstall_entry`: Runs a sequence of helper calls, performs a loop involving memory freeing, and conditionally swaps a table entry with a RAM variable based on a constant match.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:DuneII

- `load_and_relocate_hunks`: Parses a data stream containing hunk type codes to allocate memory and load segments.
- `decrement_and_loop`: Decrements a value in a register and loops until the value reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_2`: Performs bitwise operations and memory copies on data based on a variable count and specific bit flags.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Iterates through a loop to perform bitwise shifts and rotations on data based on a variable count.
- `execute_trap_and_store`: Triggers a software interrupt and then stores a byte from a register into a memory location pointed to by another register.

### Startup

- `initialize_application_environment`: Sets up the initial memory structures and stack pointers for the application.
- `execute_cleanup_sequence`: Performs a series of conditional calls to routines stored in a global structure and executes a cleanup sequence involving a linked list.
- `execute_cleanup_callbacks_2`: Iterates through a linked list of structures to invoke a sequence of function pointers.
- `return_without_effect_3`: The routine has 8 instructions in the binary, but the decompiler reduces them to an immediate return, meaning the body performs no observable work beyond returning to its caller.
- `return_without_effect_4`: This routine executes eight instructions that result in no observable side effects other than returning to the caller.
- `free_linked_list_memory`: The routine traverses a linked list and calls a function to free each element until the list is empty.
- `process_callback_status_alt`: Evaluates a status value by checking bits in a structure to determine whether to store a constant, execute a callback, or iterate through a linked list.
- `process_status_code`: The routine evaluates a status value and updates various global variables and parameters based on the result.
- `manage_startup_task_state_2`: Checks a status flag in a structure to determine if a specific initialization sequence has already occurred.
- `execute_cleanup_list`: Iterates through a linked list of entries and executes specific routines based on flags found within each entry.
- `parse_command_line_arguments_4`: Parses a string into an array of pointers by splitting it into tokens based on whitespace or quoted substrings.
- `copy_and_update_buffer`: The routine copies a sequence of bytes to a local stack buffer and updates a pointer in a table.
- `initialize_startup_environment`: Copies a string from a global location to a local buffer, checks system task status, and updates a global pointer with a function address.
- `traverse_linked_list`: The routine iterates through a linked list for a specified number of steps and returns a pointer to the current node.
- `manage_startup_task_state`: Checks a status flag in a structure to determine if a specific initialization sequence has already occurred.
- `find_byte_in_buffer_2`: Iterates through a memory buffer of a specified length to find a matching byte value.
- `append_string_to_buffer_2`: Finds the null terminator of a destination string and copies a source string to the position immediately following it.
- `copy_and_pad_string`: Copies bytes from a source pointer to a destination pointer until a null terminator is reached.
- `execute_cleanup_callbacks`: Traverses linked lists to execute function pointers and performs conditional routine calls based on bitwise flags within a structure.
- `do_nothing`: This routine is empty and performs no operations before returning.

### Format

- `parse_command_line_arguments_2`: Parses a command line string into individual tokens, handling quoted strings and whitespace delimiters.
- `update_state_and_trigger_callbacks`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
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
- `release_format_buffers_and_handles`: Releases the buffers and handles that the format routine allocated, calling a release routine for each field that is set.
- `handle_disk_error_status`: Evaluates a status code and selects a pointer to a global variable to pass to various routines, potentially performing a specific operation if a certain value is met.
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
- `format_disk_tracks_and_verify`: Formats a disk track by track by building track data in a buffer and writing it through the disk device, then verifies the tracks.
- `write_value_to_output_handle`: Calls two output routines with fixed arguments of 20, 30 and 1 on a handle taken from a structure, then passes a value from another routine through the same handle.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `rotate_and_divide_32bit_value`: Performs a bitwise rotation on a 32-bit value and then calculates a division-based result using the rotated components.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry_2`: Retrieves a function pointer from an offset within a global data structure and executes it.
- `store_byte_at_pointer_2`: Stores a byte from a register into the memory location pointed to by a register.
- `allocate_indexed_record_block`: Obtains a slot index from an unnamed routine and, when valid, allocates a 34-byte block through a library call with flags 65537.
- `release_record_chain`: Recursively releases a linked chain of records, processing the linked record first when its link field is set.
- `forward_argument_with_code`: Passes its one argument to another routine together with the constant 48.
- `forward_single_argument`: Passes its one argument on to another routine and returns nothing.
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

- `parse_command_line_arguments_3`: Parses the command line string into individual tokens, handling quoted strings and converting specific character sequences into newline or escape characters.
- `update_and_cleanup_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `format_disk_media`: Checks the media type and capacity of a disk, then performs a track-by-track formatting and verification process.
- `execute_unreached_code`: This routine is unreached and contains a gap that decodes as code.
- `initialize_entity_state`: Initializes fields in a structure and invokes another routine with values or pointers from that structure, conditionally calling further routines or looping based on certain structure fields.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from the input pointer, stores the second result into a field of the pointed-to structure, and if a flag field is zero it increments a short located at an offset from a pointer stored in the structure.
- `parse_buffer_content`: Uses another routine to check a buffer, updates a size field, and scans for a newline or form-feed character while potentially modifying a flag or string content.
- `process_entity_list`: Determines a loop limit based on fields in a passed structure and iterates to call another routine using a pointer to a table and a global variable.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Runs a sequence of calls to another routine using arguments from a parameter block and a global variable, returning the result of one of those calls.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at an offset from a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: The routine calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: The routine calls a drawing routine with a fixed or zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `draw_text_line`: Uses a handler pointer to call drawing primitives while iterating over a text buffer, tracking line count and stopping if a certain character or flag is encountered.
- `dispatch_recursive_pointer`: The routine calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks if a requested size is within the remaining buffer limits and, if so, updates the current position and remaining size offsets after recursively processing the buffer.
- `realign_and_recursively_offset_buffer`: The routine recursively calls another routine to process segments of a structure based on an offset and size field, updating the offset field with a positive return value.
- `update_scroll_window`: Calculates a target position based on provided offsets and compares it against the bounds of a window structure.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string_2`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_uppercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal_2`: Converts a null-terminated string representing a signed decimal integer into a numeric value.
- `call_library_function_with_mask`: Checks if a pointer in a structure is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `process_message`: Recursively processes a structure by invoking another routine on certain fields and string arguments, then conditionally updates structure fields and invokes further routines.
- `build_message_buffer`: The routine obtains a pointer from another routine and then calls another routine with that pointer, a constant, a stack-allocated buffer containing the input parameter, and a size of one.
- `copy_entity_data`: The routine obtains a handle from another routine and, if valid, reads a value from that handle, passes it to another routine together with a small stack buffer, and copies two fields from the handle into the caller-provided structure.
- `initialize_object_from_template`: Obtains a template identifier, allocates an object, links its fields to a template and source block, and copies words from a source address into the new object.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_scheduled_function_2`: Calls a function pointer retrieved from an offset of a global variable.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_indirect_function_10`: Fetches a function pointer from a location offset from a global variable and executes it.
- `execute_indirect_function`: Fetches a function pointer from a global table using a fixed offset and jumps to it.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_indirect_pointer`: Calls a function pointer stored at an offset from a global variable.
- `call_indirect_pointer_2`: Executes a subroutine located at a specific offset from a global variable.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_indirect_pointer_3`: Performs an indirect call to a function address retrieved from a global variable.
- `call_indirect_pointer_4`: Branches to a subroutine address found by offsetting a global variable.
- `call_indirect_pointer_5`: Invokes a function through a pointer stored within a global variable structure.
- `call_indirect_pointer_6`: Calls a routine using an address calculated from a global variable offset.
- `call_indirect_pointer_7`: Transfers execution to a function pointer located at a global variable offset.
- `call_indirect_pointer_8`: Executes a subroutine via a pointer stored in a global variable.
- `call_indirect_pointer_9`: Calls a function by dereferencing a pointer at a global variable offset.
- `call_indirect_pointer_10`: Jumps to a subroutine address retrieved from a global variable.
- `call_indirect_handler`: Calls a function pointer located at an offset from a global variable.
- `call_indirect_handler_2`: Executes a routine via a pointer stored in a global table.
- `call_indirect_handler_3`: Dereferences a pointer from a global variable to call another routine.
- `call_indirect_handler_4`: Performs an indirect call using a pointer retrieved from a global variable.
- `call_indirect_handler_5`: Invokes a subroutine through a pointer stored in a global memory location.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `extract_byte_from_call`: Calls another routine with a pointer to a small stack buffer and a constant, returning a byte from that buffer if the call returns a specific value, otherwise returning negative one.
- `invoke_and_verify_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_via_pointer_2`: Stores a byte from a data register into a memory location pointed to by an address register.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_2`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry_3`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer_3`: Stores a single byte from a data register into a memory location pointed to by an address register.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: The routine attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: The routine removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `build_linked_list_node`: Stores the address of a pointer argument into the memory location it points to, increments that stored value by four, clears the subsequent word, and stores the original pointer address in the following word.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_and_call`: Copies a block of sixteen words from a source offset to a destination and then executes a function pointer.

### Echo

- `reallocate_and_patch_table`: Iterates through a linked list to find a maximum value, then reallocates a table by expanding its size and filling it with a sequence of incrementing values.
- `copy_table_and_call_function`: Copies sixteen words from a memory location offset from a pointer into a destination address on the stack, then executes a function pointer.

### Execute

- `reallocate_and_patch_table_2`: Traverses a linked list to find the maximum value in a sequence, then expands a table by writing a series of incrementing values.
- `copy_memory_and_call`: Copies sixteen bytes from a memory location to a destination pointed to by the stack, then executes another routine via a function pointer.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `reallocate_and_patch_table_3`: Traverses a linked list to find a maximum value and then reallocates a table by expanding its size and filling it with incrementing values.
- `copy_table_and_call_function_2`: Copies sixteen words from a memory location to a stack-based destination before executing a function pointer.

### IconX

- `parse_command_line_arguments`: Parses a command line string into individual tokens, handling quoted strings and converting specific character sequences into newline or escape characters.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor by making the library structure and setting up its function table.
- `scan_ram_entries_for_names`: Allocates a 260-byte buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `join_strings_into_buffer`: Copies one string into a destination buffer and joins a second string onto it, then passes the buffer and combined length to a helper.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: Library code that copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string and then passes that position and a second argument to another routine.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_supervisor_function`: Calls a function pointer located within a global structure.
- `call_indirect_function`: Fetches a function pointer from a global table and executes it.
- `call_scheduled_function`: Calls a function pointer stored at an offset within a global variable.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatched_function`: Executes a function retrieved from a specific offset in a global table.
- `call_library_vector_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_function_maker`: Invokes a routine located at a fixed offset from a global pointer.
- `call_resident_initializer`: Calls a function pointer found by applying an offset to a global variable.
- `call_enabled_function`: Performs a call to a function pointer stored within a global data structure.
- `call_memory_allocator`: Calls a routine located at a specific offset relative to a global pointer.
- `call_memory_freeer`: Executes a function pointer retrieved from a global variable using an offset.
- `call_entry_allocator`: Calls a function pointer stored at a calculated offset from a global variable.
- `recurse_with_stack_slot_2`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `call_first_table_function`: Calls a function pointer located at an offset from a global base address.
- `call_second_table_function`: Invokes a function pointer retrieved from a global variable via a specific offset.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `execute_indirect_call`: Loads a pointer from a location relative to a global variable and jumps to it.
- `call_offset_pointer`: Performs an indirect function call using an address stored at a specific offset from a global variable.
- `invoke_stored_function`: Retrieves a function address from a table and transfers control to that address.
- `jump_to_pointer`: Executes a subroutine by dereferencing a pointer located at a fixed offset from a global variable.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer from a global variable and computes a rolling checksum over a stack buffer, passing the buffer and checksum to another routine if the inverted checksum is non-zero.
- `store_byte_at_pointer`: Stores a byte from a register into a memory location pointed to by another register.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer`: Stores a byte from a data register into the memory location pointed to by an address register.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a buffer.

### Intro

- `load_hunk_data`: Parses a data stream containing hunk type codes to allocate memory and copy segments.
- `decrement_and_loop_2`: Decrements a value in a register in a loop until it reaches zero.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `manipulate_memory_blocks`: Performs bitwise shifts and conditional memory copying or rotation operations on data based on a variable count.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `extract_bitstream_data`: Reads a sequence of bits from a source buffer and reconstructs them into a destination register.
- `process_bit_stream_3`: Performs bitwise shifts and rotations on data based on a variable count and specific bit flags.
- `execute_trap_instruction`: triggers a software interrupt and performs a bitwise OR operation on a value at a calculated memory location.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:Add16k

- `traverse_and_decrement_linked_list`: Recursively traverses a linked list structure by following pointers stored in a record.
- `open_graphics_and_intuition_libraries`: Opens the graphics and intuition libraries by name through the library-open routine, keeping both bases.
- `clear_slot_record_store_stack`: Clears forty-two longwords of a per-slot record, stores the stack position and a global value into it, then calls another routine.
- `open_dos_and_mathffp_libraries`: Opens the dos and mathffp libraries by name and stores their base addresses in global variables.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `close_stored_libraries_and_resources`: Runs a ten-step wait, closes each stored library base that is nonzero and releases a stored resource through its routine.
- `append_string_with_count_2`: Walks to the end of a destination string and appends bytes from a source string until a terminator is reached or a specified count is exhausted.
- `append_string_with_count`: walks to the end of a destination string and appends bytes from a source string until a terminator is reached or a specified count is exhausted.
- `copy_string_to_buffer`: copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `compute_string_length`: walks forward through a zero-terminated byte string to calculate the number of bytes preceding the terminator.
- `copy_bytes_zero_padded`: copies a specified number of bytes from a source to a destination and fills any remaining space in the count with zero bytes.
- `lookup_and_execute_callback`: validates an index against a table, checks a flag, and either calls another routine or returns an error code.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_function`: calls a function via an offset from a pointer stored in a global variable.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_indirect_via_offset`: Calls a subroutine using a pointer retrieved from a global structure at a specific offset.
- `call_library_function_offset`: calls a function using an indirect pointer retrieved from a global structure at a specific offset.
- `call_indirect_function_2`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `call_indirect_function_3`: Executes a function located at an address calculated from a base pointer and a specific offset.
- `call_indirect_function_4`: Calls a subroutine by dereferencing a pointer found within a data structure.
- `call_indirect_function_5`: Uses a calculated address to jump to another routine via a pointer in a table.
- `call_indirect_function_6`: Triggers an indirect call to a routine based on an offset from a global pointer.
- `execute_indirect_pointer`: Performs an indirect call to a function address retrieved by traversing two levels of pointers starting from an offset of a global variable.
- `call_indirect_function_7`: Performs an indirect branch to a function address stored in a table.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_indirect_function_8`: Calls a routine through a pointer retrieved from a memory location using an offset.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_function_9`: Executes a function call using an address derived from a table and an offset.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:MakeDummy

- `initialize_system_environment`: Sets up global state and memory structures by calculating stack offsets and allocating memory.
- `store_value_and_call_routines`: It stores a passed value at a fixed offset from a computed base pointer, then conditionally decrements a counter and invokes a routine from a table.
- `execute_callback_chains`: Iterates through multiple linked lists to execute a sequence of function pointers.
- `return_without_effect`: The routine performs no observable work beyond returning to its caller.
- `return_without_effect_2`: This routine consists of instructions that result in an immediate return without modifying data.
- `drain_queue_list`: It walks a doubly linked list by following forward links, invoking a routine for each node until it reaches the end.
- `allocate_and_store_base_pointer`: It checks a flag bit in a structure and, when set, calls another routine to obtain a base pointer.
- `process_callback_status`: Reads status bytes from a structure and branches on their bits to either record a fixed value, invoke a callback, or walk a linked list of callbacks in a loop.
- `process_status_result`: It evaluates a status value and updates various global fields or returns a modified parameter based on the result of another routine.
- `dispatch_buffer_result`: Calls a routine to obtain a status value, stores a fixed byte into a field when it is zero, and otherwise reads a pointer from a table to call another routine and store its result.
- `iterate_and_close_library`: Stores a fixed value at a fixed offset from a base pointer, then walks a linked list of entries invoking a routine for each one before closing a library.
- `parse_file_into_tokens`: Parses a text file into an array of whitespace-separated tokens, handling quoted strings with backslash escape sequences, then stores the resulting token pointers in a table and prepares a set of callback offsets for later processing.
- `copy_and_update_table`: It copies a sequence of bytes from a source to a local buffer, performs several conditional checks involving other routines, and stores a pointer to the buffer into a table.
- `copy_and_dispatch_routine`: Copies a bounded number of bytes from a source into a stack buffer, null-terminates it, then calls a sequence of routines that branch on the contents of a structure and store a callback address into a table.
- `resolve_index_from_list`: Walks a singly linked list of pointers, decrementing a counter until it reaches zero, and returns the matching node or a null pointer when the bounds are exceeded.
- `copy_and_dispatch_data`: Copies a bounded number of bytes from a source into a stack buffer, null-terminates it, then calls a sequence of routines that branch on the data.
- `find_byte_in_buffer`: The routine searches for a specific byte value within a memory buffer of a given length.
- `append_string_to_buffer`: The routine appends a source string to the end of a destination string.
- `copy_bytes_with_termination`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `allocate_aligned_memory_block`: Aligns the requested size to a four-byte boundary and searches a linked list of free memory blocks for a suitable match.
- `process_callback_chains`: Iterates through two linked lists to execute callbacks and conditionally invokes another routine by traversing a third linked list based on bitwise status checks.
- `copy_string_with_limits`: Walks a null-terminated string, copying its characters into a destination buffer while enforcing a maximum length stored in a structure.
- `store_byte_and_update_state`: Examines status bytes in a structure and branches on their bits to set flags, reset counters, or return early.
- `update_file_node_state`: Iterates a linked list to aggregate a status flag, then resets or updates a node's counters and status bytes based on helper return values and returns a status code.
- `do_nothing_return`: The routine performs no operations and simply returns to its caller.

### Dune II - Battle for Arrakis (1993)(Virgin)(M3)(Disk 1 of 6)[cr SKR].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `format_string_buffer`: Clears a memory region and constructs a new string by copying bytes from a source and appending specific character delimiters.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `traverse_function_pointer_list`: Follows a linked list of pointers and executes a subroutine at a fixed offset from each pointer until a null pointer is encountered.
- `parse_string_tokens`: Parses a string into tokens by skipping whitespace and handling quoted substrings, storing the resulting pointers in a local array.
- `initialize_global_variables`: Initializes a block of consecutive global variables by writing a word value, a long word value, and a zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_struct_and_toggle_flag`: It initializes several fields of a structure with fixed values, then toggles and clears specific bits of a flag byte.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `initialize_and_dispatch_handlers`: Initializes a structure and stores a passed value into it when a global flag is clear.
- `dispatch_pending_handlers`: Runs two setup routines, then checks two global flags and, for each one that is set, invokes a handler pointer stored within a structure before clearing the flag.
- `init_audio_device`: Initializes and opens the audio device, then sets up a table of sound-related structures and starts the audio subsystem.
- `clear_flags_and_process_structures`: Clears two bits in a global variable, then loops four times to check a bit and reset a byte within each of four consecutive structures.
- `iterate_structures_and_toggle_flags`: Loops four times, calling another routine for each of four structures stored consecutively in a table whenever the first field of that structure is nonzero, advancing by a fixed stride each iteration.
- `store_register_and_call_division`: Stores a word taken from a register into a global variable, then calls a division routine purely for its side effects and stores the discarded result into another global variable before returning.
- `update_flag_from_input`: Reads a hardware register for input state, then sets or clears a specific bit in a global variable based on that value.
- `check_global_flag_bit`: Checks a flag bit in a global variable and returns one when the bit is clear, otherwise returns zero.
- `advance_table_loop_counter`: Decrements a loop counter and repeatedly advances through a global table, skipping entries that match a marker and zero flag, then reloads the traversal pointer from another global entry until the counter is exhausted.
- `invoke_routine_wait_flag`: Stores a constant and a parameter into global variables, invokes a routine via a pointer in RAM, then busy-waits until a global flag clears before returning the stored parameter.
- `initialize_and_reset_state`: Executes a sequence of operations including clearing bits in a global variable, performing several routine calls, and setting a global variable to a specific value.
- `check_value_and_store_offset`: Compares a passed value against a global variable and, when it is smaller, clears two flag bits and stores a shifted, offset value into a table.
- `update_object_data`: Calls two routines through pointers in a global variable, updates a field in a second object with a value from the first, and optionally stores the second object in a pointer.
- `free_linked_list_entries`: Walks a doubly linked list, freeing each entry, then clears the head pointer of the list.
- `clear_table_entry_fields`: Clears several fields of a table entry and returns a value retrieved from one of those fields.
- `clear_list_and_reset_globals`: Loops over two fixed ranges calling a routine when a register value is not one or two, then walks a linked list freeing entries and resetting several global variables.
- `process_object_list_and_update_data`: Iterates through a linked list of structures to update object data and manage memory.
- `search_pairs_for_value`: Scans an array of pairs, returning a null pointer when it hits an empty slot or the matching pair when found.
- `call_global_routine`: Checks if a global variable is non-zero and, if so, calls a routine through a pointer at a fixed offset from that variable.
- `call_dispatched_routines`: Retrieves a pointer from an offset of a register and calls two routines through pointers at fixed offsets from that retrieved pointer.
- `call_two_dispatched_routines`: Reads a pointer from a global variable, then calls two routines through pointers stored at fixed offsets from that pointer.
- `initialize_and_dispatch_routines`: Runs a chain of initialization checks and, when they pass, sets a flag and performs a series of setup operations that load values from a table, advance time-based counters, and dispatch work through a routine pointer.
- `negate_and_trap`: Negates a value at a pointer, executes a trap, and then performs a large switch-case based on a register value.
- `route_record_index`: Compares a two-byte value against a series of constants and branches to a shared block for matching cases while falling through to a final call for the last accepted value.
- `select_constant_by_condition`: Compares two global variables and checks a third for zero and for the value two, then stores one of three constant values into a global variable depending on those conditions.
- `copy_string_to_buffer_3`: Copies a fixed number of bytes from a source pointer into a global buffer, null-terminates it, and then appends additional data to that buffer.
- `find_first_and_last_valid_records`: Iterates through a linked list of records, applying bitmask and flag checks to identify valid entries.
- `reset_and_call_nested`: Calls a routine with a global variable and clears that variable, then optionally calls another routine twice and updates the global variable with the result.
- `null_routine`: This routine is empty and contains no executable instructions.
- `process_recursive_data`: Checks a global variable, calls another routine to clear it, and then performs a recursive operation using two parameters.
- `process_single_parameter`: Checks a global variable, calls another routine to clear it, and then performs a recursive operation using one parameter.
- `null_operation`: This routine is empty and performs no operations.
- `enter_infinite_loop`: Executes an infinite loop that performs no operations.
- `nop_placeholder`: This routine contains no executable instructions.
- `enter_infinite_loop_2`: This routine enters an infinite loop that prevents further execution.
- `execute_invalid_instruction`: This routine consists of a single byte of data that does not form a valid instruction.
- `enter_infinite_loop_3`: The routine enters an infinite loop that prevents further execution.
- `append_string`: Finds the end of the first string and copies the contents of the second string to that location.
- `enter_infinite_loop_4`: This routine enters an infinite loop that prevents further execution of the calling thread.
- `process_recursive_data_2`: Checks a global variable and calls another routine if it is non-zero, then performs a series of recursive calls and updates a global variable with the result.
- `null_routine_2`: The routine is empty and contains no executable instructions.
- `perform_nested_calls`: Sets a global variable to one and executes a long sequence of nested conditional calls to another routine before resetting the global variable to zero.
- `enter_infinite_loop_5`: This routine executes an infinite loop that prevents further code execution.
- `initialize_application_startup`: Sets up the application's startup sequence by running a chain of initialization checks and, when they pass, configuring a display context and memory allocation tracker.
- `execute_single_instruction`: Executes a single instruction and is called via a relative address.
- `execute_sequential_calls`: Calls another routine with a pointer to a global variable and two constants, followed by four subsequent calls to another routine.
- `initialize_unit_objects`: Iterates through a table of entity pointers to allocate memory and initialize properties for each object.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_pointer`: Checks if a global variable is non-zero, calls another routine with it, and then resets the global variable to zero.
- `combine_byte_and_memory`: Combines the high byte of a parameter with a byte fetched from a memory location calculated using the low byte of that parameter.
- `manage_linked_list`: Traverses a list to find an empty slot, inserts a new node containing a pointer and a value, and updates a global pointer.
- `clear_global_pointer_recursive`: Checks a global variable to clear it via another routine, then recursively processes a parameter through two subsequent calls.
- `validate_parameter_state`: Calls another routine and, if the result is two, performs conditional checks and bitwise operations on a parameter before returning it.
- `calculate_scaled_value`: Performs a series of arithmetic operations including multiplication, division, and bit shifting using values from a table and a global variable.
- `execute_empty_call`: This routine performs no operations and immediately returns to its caller.
- `dispatch_command_type`: Reads a byte from a pointer and executes different routines based on whether the value is zero, three, or four.
- `process_object_data`: Fetches data from a structure, copies a string from a table into a local buffer, and calls another routine with that buffer.
- `drain_null_terminated_routine`: Sets a global byte to one, then repeatedly calls a routine while it keeps returning a null byte, stopping as soon as it returns a non-null value.
- `initialize_and_load_intro_assets`: Initializes a structure and stores its pointer in a global variable, then attempts to load data from a file record sequence to populate the structure's fields.
- `clear_globals_and_loop_dispatch`: Clears several global variables and then loops a fixed number of times, writing a counter value to a global byte and calling a routine through a pointer stored in a structure.
- `initialize_game_state_and_resources`: Copies a block of data from a source buffer to a global destination, then performs a series of conditional checks and resource lookups.
- `initialize_intro_resource_sequence`: Initializes and validates a sequence of multimedia resources by checking for the presence of specific data headers and managing a global state pointer.
- `copy_buffer_then_check_result`: Copies a 64-byte block from a global buffer into another global buffer, then repeatedly calls a routine with a fixed argument until it returns a non-null byte.
- `process_intro_data_sequence`: Performs a series of data manipulations including table interpolation, buffer copying, and a loop that checks for a non-null result from another routine.
- `copy_buffers_and_scan_chars`: Copies a fixed number of bytes from one global buffer to another, then branches on a global flag to invoke a setup routine with different arguments, and finally loops over a buffer while reading characters until a non-empty one is found.
- `load_resource_files_sequentially`: Copies a null-terminated string from a global buffer into a fixed output buffer, then runs a series of checks that load resource files by name and process their contents.
- `copy_buffers_and_loop_check`: Copies a fixed number of bytes from one buffer to another, then repeatedly calls a routine with a running counter until it returns a non-zero result, finally reporting whether that result was non-null.
- `copy_buffer_and_scan_routine`: Copies a fixed block of bytes from one global buffer into another, then repeatedly queries a routine for a value and branches on it depending on the loop position and the state of a global flag.
- `copy_and_scan_buffer`: Copies a fixed number of bytes from one buffer to another, then repeatedly calls a routine over a range of indices, inserting special handling for certain indices when a global flag is set, and stops early when that routine reports a value.
- `load_and_validate_text`: Copies a block of bytes from a table into a buffer, clears a portion of that buffer, and then performs multiple sequential attempts to read text data, returning success if a valid sequence is found.
- `copy_and_invoke_routine`: Copies a 64-byte block from one buffer to another, then repeatedly invokes a routine through a pointer stored at a fixed offset within a global structure.
- `process_linked_nodes_loop`: Initializes a byte flag to zero, stores a parameter into a global byte variable, and then loops while that global is nonzero, calling another routine and setting the flag when it returns nonzero before returning the flag.
- `iterate_and_clear_bits`: Clears the lowest bit of a global variable and iterates through a fixed-size array of pointers to call another routine if a pointer is non-zero.
- `update_global_pointer`: Updates a global variable with a provided pointer or sets it to zero and performs arithmetic operations on it.
- `initialize_subsystem`: Initializes a subsystem by calling a function pointer from a structure, setting various offset-based values, and updating global state variables.
- `execute_callback_with_stack`: Stores five parameters into a local stack frame and calls a function pointer retrieved from a global variable.
- `interpolate_data_table`: Iteratively adjusts values in a table by comparing nibbles and bits between two arrays until they match, calling a function after each change.
- `search_bit_fields`: Recursively checks the high and low words of a 32-bit integer by calling another routine on each half.
- `enter_infinite_loop_6`: An empty routine that enters an infinite loop.
- `call_routine_thrice`: Calls another routine three times in succession.
- `set_flag_on_nonzero`: Calls another routine and sets a specific bit in a global variable if the input parameter is not zero.
- `modify_global_value`: If a parameter is less than a global variable, it clears a bit in that variable and adds a shifted version of the parameter to it.
- `parse_resource_header`: Iterates through a data stream to identify and extract specific magic number markers.
- `execute_callback_from_global`: Checks a global variable and, if non-zero, calls a function pointer stored at a specific offset from that global value before clearing the global.
- `clear_global_variable`: Sets a global variable to zero.
- `manipulate_global_bits`: Performs a series of bitwise operations and assignments to a global variable based on the result of another routine.
- `get_pointer_from_table`: Returns a pointer retrieved from a table using an offset derived from a global variable and a parameter.
- `call_indirect_via_offset_2`: Calls a function pointer located at a specific offset from a value retrieved from a memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_deferred_value`: Flushes a pending value held in a global slot by running another routine on it.
- `process_nested_calls`: Executes a sequence of nested routine calls using parameters and results passed between them.
- `copy_and_call_routines`: Copies a block of data from a pointer to a fixed destination and then calls two other routines using specific addresses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `swap_and_clear_buffer`: Swaps the contents of a global word with a parameter value and returns the previous value, then reads a length from a global variable and a pointer from the adjacent location and clears that many consecutive bytes.
- `clear_memory_blocks`: Clears a specified amount of memory by filling large segments with zero in sixty-four thousand byte increments, followed by a final loop to clear any remaining bytes.
- `fill_memory_blocks`: Fills large blocks of memory with zero in increments of sixty-four thousand bytes until a specified count is reached.
- `fill_memory_block`: Fills a block of memory with a specific byte for a specified number of iterations.
- `swap_global_word`: Swaps the contents of a global word with a parameter value and returns the previous value.
- `swap_flag_and_set_offsets`: Swaps a global flag value with a passed-in value and restores the previous one on return, then stores several word-sized fields into a structure indexed by a parameter.
- `copy_and_call_routines_2`: Copies sixty-four bytes one at a time from one pointer to another, then invokes two routines through function pointers stored in a structure.
- `copy_fixed_bytes`: Copies sixty-four bytes one at a time from one pointer to another.
- `initialize_from_pointer`: Initializes several global fields from a pointer, or resets them to defaults when the pointer is null, returning the previous stored pointer.
- `set_flag_bit`: Sets a bit in a global byte and returns the previous value of that bit.
- `clear_bit_and_return_status`: Clears a specific bit in a global byte and returns a value indicating whether that bit was set prior to being cleared.
- `lookup_and_adjust_value`: Looks up an entry in a table using a passed value and, depending on the result, either adjusts a value taken from the stack or clears a field through a pointer.
- `swap_flag_and_process_game_state`: It swaps a global flag with a passed value, copies forty bytes from one pointer to another, and initializes global fields from a pointer or resets them to defaults.
- `swap_flag_and_set_offsets_2`: Swaps a global flag value with a passed-in value and restores the previous one on return, then stores several word-sized fields into a structure and clears two more fields.
- `find_max_fixed_point_index`: Repeatedly evaluates three fixed-point products, tracks the largest product seen with its index, and stops when a table entry holds a sentinel value or the index exceeds a limit.
- `append_string_to_buffer_3`: Finds the null terminator of a destination buffer and copies bytes from a source pointer into that location until a null terminator is reached.
- `process_linked_list_flags_2`: Performs the same linked list traversal and flag comparison logic as another routine.
- `multiply_fixed_point_values`: Calculates the product of two 16-bit values expanded into a 32-bit fixed-point result using bit shifts and multiplications.
- `run_and_store_result`: Sets a local counter to one, runs a routine with a passed value, splits the high half of another result into a local slot, and calls a routine with several of these values.
- `store_indirect_value`: Copies a 32-bit value from a location four bytes before a register into the address held in another register, then returns.
- `reset_global_byte`: Sets a global byte to zero.
- `swap_flag_and_compute_offsets`: Swaps a global flag value with a passed-in value and restores the previous one on return.
- `execute_nested_routines`: Calls two consecutive routines, returning the value produced by the first call.
- `reset_dispatched_routine_state`: Reads an index from a global structure, calls a routine with a computed pointer and adjusted value, then clears several fields in that structure.
- `copy_value_and_decode_text`: Copies a 32-bit value from a memory location pointed to by a register into a destination address, then processes a compressed text stream.
- `process_compressed_text_stream`: Runs a state machine over a compressed text buffer to decode escape sequences such as percent and 's' markers.
- `execute_recursive_graphics_logic`: Checks a global pointer, executes a routine through it, and then performs recursive calls using values derived from a global structure.
- `advance_and_truncate_string`: Decrements a per-object counter and, upon reaching zero, scans a string backward to find a space character to truncate it.
- `fetch_next_compressed_byte`: Fetches the next byte from a chain of pending buffers, advancing through them until a value is found.
- `fetch_pending_compressed_byte`: Loads a byte from a global variable, replaces that variable with a new value from another routine, and returns the original byte.
- `clear_global_word`: Sets a global word to zero.
- `forward_arguments`: It forwards its three arguments to another routine and then returns, performing no work of its own.
- `call_copy_buffer_interleave_2`: Calls a routine that copies five consecutive bytes from a source buffer into five scattered positions of a destination array, repeating this process for each row of a two-dimensional structure.
- `forward_three_arguments`: Passes three arguments to another routine and then returns without performing any additional operations.
- `clear_byte_buffer`: Reads a length from a global variable and a pointer from the adjacent location, then clears that many consecutive bytes starting at the pointer.
- `clip_line_to_bounds`: Clamps a line segment's endpoints to a rectangular region using Cohen-Sutherland-style boundary flags, computing the intersection point where the segment crosses each edge.
- `check_bounds_flags`: Compares two coordinate fields against stored limits and accumulates flag bits indicating whether each coordinate falls below or above those limits.
- `store_bounds_values`: Stores four values into four consecutive global variables, pairing the low and high bytes of two input values into separate storage slots.
- `clamp_and_draw_lines`: It clamps two coordinate pairs so the second lies at or below the first, then issues four line-drawing calls with the resulting corner values.
- `execute_graphics_calls`: Retrieves a pointer from a global variable and executes three function calls at specific offsets from that pointer.
- `register_interrupt_server`: It registers a routine with the operating system by calling a library function with a pointer taken from a global structure, then returns.
- `set_global_constants`: Sets a global variable to several specific constant values.
- `evaluate_conditional_flags`: Runs a sequence of conditional checks that call other routines and combine their results into a byte flag, returning that flag.
- `lookup_routine_pointer`: Multiplies an index by four and returns the pointer stored at that offset in a table of routine pointers.
- `cleanup_display_context_2`: Conditionally invokes a callback via a global pointer, then resets a structure by writing a constant, a zero, and a specific value to fixed offsets.
- `call_self_recursively`: Calls itself with the same argument.
- `clear_global_pointer_2`: Checks a global variable and, if non-zero, calls a routine at an offset from that variable before clearing the global variable.
- `call_rem_int_server_2`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `process_string_handlers`: When a flag is clear, it runs two setup routines and then repeatedly walks a null-terminated string, calling a handler for each segment and advancing the pointer past each chunk.
- `accumulate_offset_differences`: Accumulates differences between pairs of global values depending on which of two bit flags are set in the argument, then returns the total.
- `compare_and_update_memory`: It calls a routine to check available memory and compares the provided parameters against a global value to conditionally update that value.
- `return_zero`: This routine is empty and simply returns a zero value to the caller.
- `update_range_with_flags`: It branches on a flag bit of an argument, clamps another argument against a stored range, advances the range end, calls another routine, and sets specific bits in the returned location.
- `call_and_return_result`: Calls another routine and returns its return value.
- `loop_until_nonzero_result`: It loops, repeatedly calling a helper and remapping its result until a non-zero value is produced, then returns that value.
- `forward_another_routine`: Calls another routine and returns its result.
- `loop_until_non_zero_mapped`: Repeatedly calls another routine and returns the first non-zero result produced by a mapping operation.
- `loop_until_nonzero_mapped`: It loops, repeatedly calling a helper and mapping its result until a non-zero value is produced, then returns that value.
- `loop_until_nonzero_wrapped`: It loops calling a helper and mapping its result, incrementing a wrapped counter when the mapped value is zero, and returns the first non-zero mapped value or zero.
- `map_argument_with_flags`: Maps an argument through a lookup table and conditional adjustments, then sets several value bits according to flag bits in the argument.
- `map_argument_table_flags`: Reads a byte from a lookup table using the low bits of the argument, remaps some values, then sets result bits according to flag bits in the argument.
- `process_circular_list_entry`: Locks interrupts, walks a circular list of entries, updates the head index and copies data from the current entry, then releases interrupts and returns a value.
- `load_table_entry`: Reads a four-byte value from a table indexed by a global variable and returns it, or zero when two global variables are equal.
- `advance_counter_locked`: Locks the system while calling two routines from a global structure and advances a counter modulo sixteen, skipping a value when it matches one of two codes.
- `update_structure_and_lookup_table`: Waits for a flag to clear, then computes a coordinate offset from the passed values and writes position and size fields into a structure.
- `update_structure_fields_2`: Computes a coordinate offset from the passed values and writes a set of position and size fields into a structure, then repeatedly updates those fields with incremented offsets while waiting for a global flag to clear.
- `update_structure_fields`: Waits for a flag to clear, writes several fields of a structure, then waits again and overwrites some of those fields before returning.
- `reset_and_update_structure`: Waits for a flag to clear, zeroes several structure fields and writes initial values, then waits again and adds a value to the stored fields before returning.
- `store_register_pair`: It calls another routine, then copies the two 32-bit result registers (D0 and D1) into a global variable.
- `setup_blitter_operation`: Initializes a blitter operation by waiting for the blitter idle flag, programming the blitter control, size, and address-modify registers, and loading a palette entry through the color registers.
- `increment_structure_field`: Waits for a flag to clear, writes several structure fields including a value, then waits again and increments one stored field before returning.
- `store_stack_value_then_call`: Stores a value taken from the stack into a global variable, then calls another routine and returns that routine's two data-register results unchanged.
- `parse_data_stream_2`: Iterates through a buffer, performing different actions based on byte values, including updating a global variable and calling another routine when specific criteria are met.
- `tiny_ori`: Small routine of 1 instruction that merges 239 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `copy_buffer_interleave_2`: Copies five consecutive bytes from a source buffer into five scattered positions of a destination array, repeating for each row of a two-dimensional block.
- `copy_buffer_interleave`: Copies five bytes from a source buffer into five scattered positions of a destination array, repeating for each row of a two-dimensional block.
- `interleave_buffer_data`: Reads five scattered positions from a destination array and writes them into five consecutive bytes of a source buffer, repeating for each row.
- `gather_buffer_interleave`: Reads five scattered positions from a destination array and writes them into five consecutive bytes of a source buffer, repeating for each row of a two-dimensional block.
- `initialize_display_context`: Initializes a display context by caching screen setup routines, clearing interrupt state, and zeroing a callback, then allocates and configures per-entry memory blocks while linking them into a table and updating their bookkeeping fields.
- `sum_string_byte_values`: Walks through a null-terminated string and accumulates the byte values returned by a lookup routine for each character, then returns the running total.
- `fetch_table_value_or_default`: Returns a fixed value of ten when a global flag is clear, otherwise it indexes a byte table using a scaled parameter and returns the fetched byte.
- `increment_counters_conditionally`: Increments a sequence of words pointed to by a register, conditionally bumping the second and third words based on bits in a global flag, and decrements the fourth word without going below zero.
- `invoke_double_pointer_routine`: Reads a pointer from a global structure, then invokes the routine stored at an offset from that pointer twice in succession.
- `register_table_entry`: Searches a fixed-size table for the first available slot and populates it with a pointer, a code pointer, and various status fields.
- `handle_indexed_cleanup`: Increments a global counter, reads a short from a table indexed by a passed value, and either invokes a stored routine pointer or clears a field in a structure when a flag is set, then marks the entry as handled and decrements the counter.
- `fetch_conditional_value`: Checks if a parameter is within a specific range and evaluates a condition against a global table to return either a value from a table or a constant.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies (a7)+ to d7, then returns.
- `accumulate_minimal_difference`: Tracks the smallest positive difference between a stored value and a passed value, then accumulates that amount into a running total either in one step or by repeatedly invoking a stored routine until the amount is exhausted.
- `accumulate_entity_attribute_values_2`: It iteratively calls a routine to retrieve values, adds them to a running total, and updates a table with the maximum value encountered.
- `accumulate_entity_attribute_values`: Iteratively calls a routine to retrieve values, adds them to a running total, and updates a table with the maximum value encountered.
- `accumulate_table_values_2`: Iteratively retrieves values from a routine, adds them to a running total, and updates entries in a table based on comparison logic.
- `accumulate_table_values`: Iteratively calls a routine to retrieve values, adds them to a running total, and updates entries in a table based on comparison logic.
- `update_accumulated_values_2`: Iteratively updates values in a table and a global variable by adding results from another routine.
- `update_accumulated_values`: Iteratively updates values in a table and a global variable by adding results from another routine.
- `decrement_and_dereference`: Decrements a global variable and returns a value from a memory location offset from a register.
- `invoke_double_pointer_dispatch`: A longer routine that invokes a routine reading a pointer from a global structure twice in succession, validates a table index before dispatching, and also dispatches display callbacks.
- `check_sixteen_bit_flags`: Splits a 32-bit value into its two 16-bit halves and checks each half in turn.
- `register_string_category`: Looks up a category index for a given string and, if the category has no active entry, copies the string into a stack buffer and registers it.
- `capture_and_store_result`: Runs a routine twice with different arguments, capturing the value returned by the second call.
- `resolve_string_category`: Finds the index of a matching string and returns a value built from its category, or the stored pointer of that entry when the category is marked as active.
- `swap_global_with_parameter`: Swaps the contents of a global variable with a value passed in a parameter, returning the previous value.
- `clear_linked_list`: Recursively traverses a linked list and sets the next pointer of each node to zero.
- `traverse_and_clear_linked_list`: Increments a global variable and recursively calls itself to traverse a linked list, clearing a pointer field in each matching entry.
- `reset_and_process_records`: Clears a fixed block of short records and a parallel block of byte flags, then walks a table of five-field records and, for each entry whose second flag has a certain bit set, toggles some flag bits while invoking two other routines on the associated pointer.
- `return_immediately_2`: This routine performs no operations and returns immediately to its caller.
- `validate_index_and_dispatch`: Rejects indices outside the 0 to 29 range, then triggers a stored routine whenever the matching table entry is nonzero, and finally returns the constant 1.
- `find_matching_string_index`: Walks an array of null-terminated pointers, comparing each string against a given argument, and returns the index of the first match or a sentinel value when none is found.
- `adjust_and_process_string_length`: Computes a string length adjusted by one when a flag is set, then passes the result to another routine and invokes a library call.
- `dispatch_display_callbacks`: Invokes a stored routine pointer, then calls two other routines in sequence, and finally returns without producing a value.
- `remove_and_compact_entries`: Iterates through a table to find entries that meet specific bitmask and null-byte criteria, removing them via a linked list routine and marking them as invalid.
- `allocate_tracked_memory`: Allocates a tracked block from the exec library when the flag is clear, linking it into a free list and updating bookkeeping counters, otherwise returns a previously allocated block.
- `execute_sequential_calls_2`: Calls a routine with three arguments, followed by four separate calls to another routine, and returns zero.
- `remove_entry_from_linked_list`: Adjusts a global accounting variable based on a value extracted from the passed parameter, removes a matching entry from a linked list of globals, optionally calls a routine via a pointer stored in a table, decrements a global counter, and returns the located entry or zero.
- `call_avail_mem`: Library stub: loads its arguments and the exec library base, calls AvailMem and returns its result to the caller.
- `compute_available_memory_delta`: Requests available memory from the operating system, accumulates offset differences, and returns the requested amount minus the accumulated total.
- `free_linked_list_entries_2`: Recursively frees each entry in a linked list and then clears the head pointer to empty the list.
- `call_other_routine`: Calls another routine and then returns.
- `initialize_and_scale_pointers`: Loads two pointers from a global structure and initializes their pointed-to words, then conditionally calls a routine and rescales the values when the inputs fall within range.
- `return_immediately_3`: This routine is empty and returns to its caller without performing any operations.
- `check_entity_flag_and_process`: Retrieves a pointer from another routine and, when that pointer is valid, checks a specific bit in the second byte of the pointed-to structure.
- `parse_recursive_data_stream`: Iterates through a byte stream to interpret command codes and nested structures by recursively calling itself.
- `parse_command_stream_2`: Iterates through a byte-stream buffer to interpret command codes and pointers.
- `parse_command_stream_buffer`: Iterates through a byte-stream buffer to interpret command codes and pointers.
- `return_no_operation_stub`: The routine consists of a single instruction that returns straight to its caller, so it performs no work.
- `parse_command_stream`: Iterates through a byte-stream buffer to interpret command codes and pointers.
- `parse_data_stream`: Iterates through a byte-stream buffer, interpreting values as command codes or pointers.
- `parse_nested_data_stream`: Iterates through a data stream by reading bytes and following pointers to nested structures.
- `parse_nested_data_stream_2`: Iterates through a byte stream to process structured data, handling nested sequences and specific control characters.
- `return_immediately_4`: This routine performs no operations and returns to its caller immediately.
- `parse_bracketed_range_expression_6`: Parses a string within square brackets to check if a target value matches a specific byte or falls within a numeric range.
- `parse_bracketed_range_expression_5`: Parses a string within square brackets to determine if a target value matches a specific byte or falls within a numeric range.
- `parse_bracketed_range_expression_4`: Parses a string within square brackets to determine if a target value matches a specific byte or falls within a numeric range.
- `parse_bracketed_range_expression_3`: Parses a string within square brackets to identify if a target value matches a specific byte or falls within a numeric range.
- `parse_bracketed_range_expression_2`: Parses a string within square brackets to identify a target byte value or a numeric range.
- `parse_bracketed_range_expression`: Parses a string within square brackets to identify a target byte value or a numeric range.
- `decrement_pointer_target`: Decrements a value at a memory location pointed to by a pointer, with logic to determine whether to store the result as a byte or a word.
- `decrement_pointer_target_value`: Decrements a word value located at a memory address retrieved through a pointer and stores the result back to a destination address.
- `decrement_and_store_word`: Decrements a word value from a memory location and stores the result into a destination pointed to by another memory location.
- `decode_compressed_bitstream_data`: Decodes a variable-length bitstream by extracting values through a bitstream reader and accumulating them into a local integer.
- `decode_variable_length_bitstream_3`: Extracts and accumulates values from a bitstream using variable-length encoding.
- `decode_variable_length_bitstream_2`: Extracts and accumulates values from a bitstream using variable-length encoding.
- `accumulate_masked_values_2`: Iteratively fetches values from a pointer-based source and accumulates their lower nibbles into a destination buffer.
- `accumulate_signed_masked_values`: Fetches values from a pointer-based source, applies a sign based on specific characters, and accumulates their lower nibbles into a destination buffer.
- `accumulate_masked_values`: Iteratively fetches values from a pointer-based source and accumulates their lower nibbles into a destination buffer.
- `process_fixed_point_loop`: Executes a loop that calls a routine to fetch values, performs arithmetic, and stores the result into a destination pointed to by a structure.
- `copy_word_via_pointer`: Copies the lower word of a long value from a memory location to a destination pointed to by another memory location.
- `copy_long_to_word`: Copies a long value from a memory location and stores the lower word into a destination pointed to by another memory location.
- `decode_variable_length_integer`: Reads a sequence of values to reconstruct a bit-packed integer using bitwise shifts and accumulation.
- `decode_bitstream_values`: Processes a sequence of values by performing bitwise operations and accumulation, then stores the final result into a destination structure.
- `update_pointer_data_2`: Copies a word from one memory location to another using two different pointer structures and increments a value.
- `update_pointer_data`: Copies a word from one memory location to another via two different pointer structures and increments a value.
- `decode_variable_length_bitstream`: Extracts a sequence of variable-length bitstream values by iteratively reading data and applying bitmasks to shift and combine them into a single accumulator.
- `decode_variable_bitstream_sequence`: Extracts and accumulates variable-length bitstream data by iteratively reading values and applying bitmasks to shift and combine them into a single integer.
- `accumulate_bitstream_data_2`: Iteratively extracts values from a stream based on bitmask flags, shifts them, and accumulates them into a temporary integer before storing the result.
- `accumulate_bitstream_data`: Iteratively extracts values from a stream based on bitmask flags, shifts them, and accumulates them into a temporary integer before storing the result into a destination structure.
- `store_result_to_structure`: Stores a value into a destination address pointed to by a structure, choosing between word or long storage based on internal flags.
- `store_indexed_word`: Stores a word from a memory location into a destination address pointed to by an offset from a pointer.
- `update_pointer_indexed_data`: Stores a word from a memory location into a destination address pointed to by an offset from a pointer, then stores a second word into another destination address derived from a different offset.
- `read_byte_sequence`: Repeatedly calls a routine to fetch bytes, stores them into a destination buffer while incrementing a pointer, and handles a termination condition.
- `check_and_store_word`: Checks if a word is equal to negative one and, if so, stores a specific value into a destination pointed to by a pointer.
- `increment_and_return`: Increments a value and returns it.
- `process_input_stream`: Iteratively calls a function to retrieve values, stores bytes into a buffer, and decrements a counter until a specific bit condition or zero count is met.
- `parse_bracketed_value_range`: Parses a string within square brackets to check if a target byte value falls within a specified range or matches a specific character.
- `parse_bracketed_range_string`: Parses a string enclosed in square brackets to identify a specific byte value or a range of values defined by a hyphen.
- `parse_bracketed_string`: Searches through a string for a specific character or a value enclosed within square brackets.
- `search_buffer_for_byte_with_delimiter`: Iterates through a memory buffer to find a specific byte value, while tracking minimum and maximum byte values encountered between occurrences of a hyphen delimiter.
- `search_and_update_buffer`: Iterates through a memory buffer to find a specific byte value.
- `search_byte_in_range`: Iterates through a memory buffer to find a specific byte value.
- `search_and_store_byte`: Iterates through a memory buffer to find a matching byte or a specific delimiter.
- `search_input_buffer`: Iterates through a buffer to find a specific byte, updating a counter and potentially clearing a value at a pointer if a match is found.
- `clear_and_store_pointer`: Checks if a value at a specific offset within a structure is non-zero, and if so, clears the value pointed to by that offset.
- `process_input_stream_2`: Checks a counter and a pointer to update a value, then potentially iterates through a buffer to find a specific byte and update state.
- `nop_padding_sequence`: This routine is empty and consists of two instructions that perform no operation.
- `store_word_and_increment`: Stores a word into a memory location at an offset from a pointer and returns an incremented value.
- `parse_format_string`: Parses a format string containing tokens such as hash, plus, minus, asterisk, decimal points, and type specifiers to format data into an ASCII string.
- `process_format_string`: Walks through a character string one byte at a time and stops when it finds a percent sign.
- `iterate_table_entries`: Steps backward through a global table of structures, skipping entries that are empty or have a particular flag bit set, and calls another routine for each matching entry.
- `tokenize_quoted_string`: Walks through a text buffer, skipping whitespace and splitting the content into up to 32 tokens while treating double-quoted runs as single tokens, writing a pointer to each token into a stack table and null-terminating each token in place.
- `execute_callback_chain`: Checks a global pointer and executes several functions at various offsets from that pointer.
- `execute_looping_callbacks`: Executes a series of functions at specific offsets from a pointer within a loop based on a counter.
- `swap_memory_blocks`: Swaps a specified number of bytes between two memory locations.
- `convert_uppercase_chars`: Iterates through a null-terminated string and converts uppercase ASCII characters to lowercase based on a flag in a global lookup table.
- `copy_and_pad_string_2`: Copies characters from a source to a destination and fills the remaining length of the destination buffer with null bytes.
- `copy_and_zero_buffer`: Copies a string from one location to another and fills the remaining length of the destination buffer with null bytes.
- `zero_memory_block`: Fills a memory block with zeros using a counter.
- `compare_case_insensitive_strings`: The routine compares two byte strings character by character, converting uppercase letters to lowercase for comparison, and returns the difference between the first mismatching characters or zero if equal.
- `copy_string_to_buffer_2`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `find_byte_in_string`: Searches a null-terminated string for a specific byte and returns a pointer to its location, or zero when the byte is not present.
- `call_and_return_value`: It forwards its argument to another routine and returns whatever that routine produces.
- `find_last_byte_in_string`: Walks a null-terminated string looking for a specific byte and returns a pointer to its last occurrence, or zero when the byte never appears.
- `find_first_set_bit`: Increments a pointer through a buffer until a byte with its eighth bit set is encountered.
- `tiny_move_movea_rts`: Small routine of 3 instructions that copies a3 to d0, then copies (a7)+ to a3, then returns.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `format_integer_to_string`: Converts a 32-bit integer into a null-terminated ASCII string by extracting digits in base-8 and reversing them into a destination buffer.
- `convert_number_to_ascii`: Converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `expand_and_parse_data`: Expands a packed nibble value using a lookup table and then parses a numeric string into a signed integer.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_integer_string_2`: Converts a string containing a signed numeric value into a short integer and returns the number of characters processed.
- `parse_integer_string`: Iterates through a string of characters to calculate a signed integer value based on numeric digits and a sign character.
- `initialize_command_parser`: Clears a global counter, stores a pointer to a buffer in a global variable, and calls a routine to parse a command stream using that pointer and a stack slot.
- `increment_and_store`: Increments a global pointer and stores a byte from a parameter into the location pointed to by the incremented global pointer.
- `format_string_to_global_pointer`: Assigns a supplied address to a global variable, invokes a routine to process a character string, and then stores a zero byte at the address held by the global variable.
- `copy_memory_block`: Copies a specified number of bytes from a source address to a destination address.
- `validate_linked_entry`: Calls another routine to retrieve a pointer.
- `process_linked_list_flags`: Walks a global linked list of structures, testing two flag bits in a status byte and computing the difference between two integer fields.
- `invoke_global_callback`: Invokes a callback through a pointer in a global table and, when a global flag is set, recursively calls itself first.
- `dispatch_callback_with_adjustment`: Invokes a callback through a pointer in a global table and, when a global flag is set, recursively calls itself first.
- `initialize_structure_fields`: It writes a full-byte bitmask at a fixed offset of a structure and writes full-word bitmasks at two other offsets.
- `cleanup_display_context`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `invoke_method_if_present`: It checks whether a global pointer is non-null and calls another routine when it is, then invokes a function pointer stored within a structure.
- `compute_fixed_point_product`: Takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `calculate_fixed_point_remainder`: Performs a normalized fixed-point division operation to calculate a remainder based on input values.
- `negate_concatenated_result`: Takes a 64-bit result from another routine and returns a new 64-bit value where the high and low 32-bit words are negated.
- `perform_fixed_point_division`: Performs a complex division operation on input values using bit shifts and normalization to calculate a result.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend by a 32-bit divisor using a normalized fixed-point method and returns a 64-bit result containing the quotient and remainder.
- `allocate_and_set_node`: Allocates memory for a structure through a library call and, if successful, initializes specific bytes and a 32-bit value with a provided parameter.
- `create_message_node`: Obtains a global variable and uses a table to call various routines, eventually filling a newly allocated node with parameters and fields from a table.
