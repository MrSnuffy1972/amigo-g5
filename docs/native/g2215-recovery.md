# Game 2215 (tv sports basketball (1990)(cinemaware)[cr pna][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02215). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1828036 top-level bytes in 53 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 29 of 29 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 690 of 690 named (0 author symbols), 690 of 690 described (172 by rule, 499 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4464 of 4468 (99.91%) |
| Recovered without load address (counted in proof 1, flagged) | 400361 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16870 of 16889 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 626665 |
| code | 55774 |
| data | 782500 |
| padding | 12377 |
| unused | 350720 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| More | 19 | 2290 | 76 |
| Type | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Dir | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Lab | 1 | 2 | 1 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| Makedir | 2 | 117 | 2 |
| CD | 2 | 117 | 2 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| IconX | 12 | 806 | 37 |
| Disk-Validator | 1 | 386 | 1 |
| paranoimia | 2 | 83 | 3 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 1 of 2).adf:bb | 3 | 7843 | 388 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 1 of 2).adf:bootblock | 1 | 11 | 1 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 2 of 2).adf:bootblock | 1 | 174 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 72 | 72 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 220 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 453 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2273 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1073 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 1 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 2 of 2).adf:Fluff.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 2 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 192 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 183 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 517 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 653 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 2 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 92 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- tv sports basketball (1990)(cinemaware)[cr pna][a](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 1 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][a](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 2 of 2).adf:Fluff.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][a](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 2 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 400361 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_unreachable_code_sequence` (More): An unreachable code sequence that calls a routine with a global variable, calls itself, and then executes a function pointer stored in an unaffected address register. Kind: sabotage path / invalid pointer trap. Pass path: the return or a valid pointer assignment to unaff_A3.

## Functions


### More

- `initialize_game_environment`: Initializes the game's internal state by restoring patch states and loading file metadata.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `process_character_glyph`: Checks a character or byte for specific printable or control ranges and, if certain conditions are met, updates a buffer or table with a new character or pointer.
- `execute_unreachable_code_sequence`: An unreachable code sequence that calls a routine with a global variable, calls itself, and then executes a function pointer stored in an unaffected address register.
- `initialize_entity_state`: Initializes fields in a structure and invokes another routine with values or pointers from that structure, conditionally calling further routines or looping based on certain structure fields.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from the input pointer, stores the second result into a field of the pointed-to structure, and if a flag field is zero it increments a short located at an offset from a pointer stored in the structure.
- `parse_buffer_content`: Uses another routine to check a buffer, updates a size field, and scans for a newline or form-feed character while potentially modifying a flag or string content.
- `process_entity_list`: Determines a loop limit based on fields in a passed structure and iterates to call another routine using a pointer to a table and a global variable.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Runs a sequence of calls to another routine using arguments from a parameter block and a global variable, returning the result of one of those calls.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at an offset from a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: The routine calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: The routine calls a drawing routine with a fixed or zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `draw_text_line`: Uses a handler pointer to call drawing primitives while iterating over a text buffer, tracking line count and stopping if a certain character or flag is unable to be processed.
- `dispatch_recursive_pointer`: The routine calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition is met, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `realign_and_update_buffer_offset`: This routine recursively checks a buffer or structure, potentially calling another routine with a pointer and size, and updates a length field by adding a returned value if it is positive.
- `update_scroll_window`: Calculates a new target window position based on provided offsets and checks it against existing window boundaries.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_uppercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `process_message`: Recursively processes a structure by invoking another routine on certain fields and string arguments, then conditionally updates structure fields and invokes further routines.
- `build_message_buffer`: Obtains a pointer from another routine and then calls another routine with that pointer, a constant, a stack-allocated buffer containing the input parameter, and a size of one.
- `copy_entity_data`: Obtains a handle from another routine and, if valid, reads a value from that handle, passes it to another routine together with a small stack buffer, and copies two fields from the handle into the caller-provided structure.
- `initialize_object_from_template`: Obtains a template identifier, allocates an object, links its fields to a template and source block, and copies words from a source address into the new object.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `extract_byte_from_call`: Calls another routine with a pointer to a small stack buffer and a constant, returning a byte from that buffer if the call returns certain a value, otherwise returning negative one.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_pointer`: Stores a byte from a register through a pointer.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port_instance`: Checks if a specific field in a structure is non-zero, then sets a status byte to a certain value and a status word to all ones, before recursively calling another routine on a linked pointer or a different parameter.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: Copies a fixed number of words from a source offset to a destination pointer if the destination pointer is not null, then calls another routine.

### Assign

- `initialize_data_structures`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped table with an incrementing sequence of integers.
- `copy_state_block_2`: copies a block of sixteen bytes from a memory location offset from a pointer to the stack, then calls another routine.

### EndCLI

- `initialize_metadata_and_stack_tables_2`: Iterates through a linked list or table to find a maximum value, then populates a secondary table with incrementing or calculated values.
- `copy_block_and_execute`: copies a fixed-size block of 16 words from a memory location offset by a register-held pointer into the stack, then executes a function pointer.

### Dir

- `initialize_global_tables`: Iterates through a linked list or table to find a maximum value, then populates a secondary table with an incrementing sequence of integers.
- `copy_and_execute_block_2`: copies a block of sixteen orp-sized words from a memory location offset from a register to the stack before calling another routine.

### Echo

- `initialize_patch_tables`: Calculates the maximum value in a linked list of patch metadata and populates a jump table or patch-related array with a sequence of incrementing values.
- `copy_stack_block`: copies a block of sixteen-bit or thirty-two-bit words from a memory location or table to the current stack frame before calling another routine.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Execute

- `initialize_program_metadata_and_stack`: Calculates the maximum value in a linked list of metadata or patch entries, then populates a table with incrementing or fixed values.
- `copy_stack_data_2`: Copies a fixed number of words from a source offset to a destination pointer if thedestination pointer is not null, then calls another routine.

### Copy

- `initialize_data_structures_2`: Calculates the maximum value in a linked list or table, then populates a memory-mapped or global array with a sequence of incrementing values.
- `copy_memory_block`: Checks if a pointer is non-null and copies a fixed number of words from a source offset to a destination pointer.

### Makedir

- `initialize_metadata_table`: Calculates the maximum value in a linked list or table, then populates a memory region with an incrementing sequence of values.
- `restore_stack_context`: Copies a block of words from a source address to a stack pointer and jumps to a stored function pointer.

### CD

- `initialize_metadata_and_stack_tables`: Iterates through a linked list or table to find a maximum value, then populates a secondary table with incrementing values.
- `copy_and_execute_block`: copies a block of sixteen or otherwise fixed-size words from a memory offset to the stack before calling another routine.

### EndIf

- `return_zero`: The routine returns a constant zero.

### If

- `initialize_stack_and_metadata_tables`: Calculates the maximum size of a linked list or table, then populates a memory region with a sequence of incrementing values and copies a block of data from a source buffer to a destination buffer.
- `copy_state_block`: copies a block of sixteen bytes from a memory location or table to the stack and then executes a previously stored function pointer.

### IconX

- `initialize_game_environment_2`: Initializes global state and loads external file metadata into memory structures.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a unable to determine if it starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `join_strings_into_buffer`: Copies one string into a destination buffer and joins a second string onto it, then passes the buffer and the combined length to a further helper.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated string, handling an optional leading plus or minus sign.
- `execute_supervisor_call`: Executes a routine called via a pointer stored in a global variable.
- `execute_indirect_function`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_2`: Loads a function pointer from a global table and executes it.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_indirect_function_3`: Loads a function pointer from a table and executes it via an indirect call.
- `execute_indirect_function_4`: Loads a function pointer from a location relative to a global pointer and executes it.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `execute_indirect_function_5`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_6`: loads a function pointer from a global table and jumps to it.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `execute_indirect_function_7`: Loads a function pointer from a global table and executes it.
- `recurse_with_stack_slot_2`: calls another routine passing two parameters and a pointer to a specific location on the stack.
- `execute_indirect_function_8`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_9`: loads a function pointer from a global table and executes it.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `execute_indirect_function_10`: Loads a function pointer from a global offset and executes it.
- `execute_indirect_function_11`: Performs an indirect call to a routine located at a global offset.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found, and passes the buffer and checksum to another routine if the inverted checksum is non-zero.
- `store_unaff_byte`: Stores a byte from a register into a memory location pointed to by a register.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_register_to_pointer`: copies a byte from a data register into a memory location pointed to by a register.

### Disk-Validator

- `format_string_and_initialize_table`: Converts a numeric value into a string or a space and stores it in a buffer, then initializes a memory-mapped table or array with specific bitmask patterns and loop-controlled values.

### paranoimia

- `decompress_bitstream_with_checksum`: Unpacks a compressed bitstream by reading bits and copying back-referenced runs backward into memory, then checks a checksum.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 1 of 2).adf:bb

- `initialize_memory_and_dispatch_logic`: Clears a block of memory to zero and then executes a conditional branch or a function call based on the state of a global variable and a secondary function pointer.
- `update_string_metadata`: Compares two values, copies a sequence of words from a source to a destination, and updates a string or buffer with a length and certain characters.
- `swap_linked_list_nodes`: Checks a pointer against a global value, swaps two adjacent pointers in a linked list, and updates a global pointer to point to the new head or a node.
- `iterate_and_clear_list`: Iterates a limited number of times, calling a routine to check a pointer, and if a certain condition is met, calls another routine with a status and a pointer before clearing a memory location.
- `update_buffer_and_linked_list`: Updates a buffer or table entry with a sequence of incrementing values and modifies a linked list by swapping or updating pointers.
- `copy_data_table`: Copies thirty-two words from a table into a destination buffer pointed to by a parameter.
- `update_buffer_recursively`: Recursively updates a buffer or a table entry with a value from a parameter, or updates a pointer-based destination if a certain condition is met.
- `update_table_recursively_2`: Recursively updates a table by increment or decrement and modifies values through a pointer until a termination condition is met.
- `update_table_recursively`: Recursively updates a table by incrementing a counter and modifying values through a pointer until a termination condition is met.
- `wait_for_value_match`: Enters a loop that continues until the contents of two memory locations are equal.
- `rotate_bits_in_memory`: Calls a library function, stores the result in a global variable, adds a constant, and then performs a bitwise rotation or shift-based update to that global variable.
- `calculate_leading_zeros_and_mask`: Calculates the number of leading zeros in a value and performs a bitwise AND with a value loaded from a table.
- `load_from_offset`: Returns a longword loaded from a memory location offset by a negative constant.
- `rotate_bits_in_memory_2`: Performs a bitwise rotation or shift-based update on a global variable and loads a longword from a memory location offset by a negative constant.
- `decrement_and_execute_list`: Decrements two global short values, updates a third global short based on a subtraction, and executes a series of functions via a linked list.
- `update_global_state`: Copies a value from one global variable to another and updates several global shorts or bytes with constant values.
- `insert_linked_node`: Inserts a new node into a linked list if the second element of the provided structure is zero.
- `insert_into_linked_list`: Traverses a linked list to find a target or end, then inserts a new pointer and clears a value at an offset of a pointer.
- `assign_pointer_and_clear_2`: Copies a pointer from one location to another, traverses a linked list, and clears a value at an offset of a pointer.
- `assign_pointer_and_clear`: Copies a pointer from one location to another and then clears the value at the offset of that pointer.
- `reset_global_pointer`: Calls another routine with specific arguments if a global variable is non-zero, then clears that global variable.
- `update_blt_registers`: Updates several bitplane-related registers and a DMA controller register using values or calculated bitmasks while checking a global flag.
- `empty_routine`: the routine is empty and performs no operations.
- `calculate_blit_source_coordinates_2`: Calculates and stores a new source or destination address or offset into a data structure, using a bitmask-based loop to check a bitset.
- `calculate_blit_source_coordinates`: Calculates and stores a new source or destination address or offset into a data structure, using a bitmask-based loop to check a bitset and update a pointer or counter.
- `update_blit_parameters_2`: Iterates through a data structure to calculate and update values used for Blitter operations and modifies a global variable.
- `update_blit_parameters`: Iterates through a data structure to calculate and update values used for Blitter operations.
- `update_blitter_parameters`: Calculates and writes new source, destination, and control registers for the blitter, including masking and masking-width or area-size adjustments.
- `empty_routine_2`: the routine is empty and performs no operations before returning.
- `initialize_input_state`: Sets certain memory or hardware registers to a fixed value and fills a buffer with a constant value using a loop.
- `execute_indirect_call_3`: Performs an indirect call to a function pointer stored in a memory location offset by a constant.
- `call_indirect_function`: Calls a function pointer located at an offset from a global pointer.
- `clear_and_combine_bytes`: Clears certain bytes in a table and returns a value constructed from those bytes and a third byte.
- `clear_and_accumulate_table_bits`: Iterates through a table to clear specific bytes and calculate a bit-shifted accumulation of values read from the table.
- `iterate_and_clear_table`: Iterates through a table, clearing bytes and calculating a bit-shifted value based on table contents.
- `clear_table_bytes`: Clears certain bytes in a table and returns a value constructed from those bytes.
- `clear_and_shift_bit_table_2`: Iterates through a table to clear bytes to zero while simultaneously updating an adjacent memory location by or-ing a bit-shifted version of the cleared value into a secondary bit-table.
- `clear_and_shift_bit_table`: Iterates through a table, clearing bytes to zero and updating a bit-shifted value in a secondary or adjacent memory location.
- `process_linked_list_nodes`: Iterates through a linked list, calling a function with certain parameters and potentially calling it a second time if a bit or a global flag is set.
- `initialize_buffer_structure`: Calls a function to get a pointer, then populates a buffer with values from a global variable and a parameter.
- `execute_loop_calls`: Iterates through a range defined by a parameter and calls a function using a value constructed from bit-shifted components.
- `transform_nibble_data`: Iterates through a table of words to perform bitwise manipulations on nibbles and stores the results in a local buffer, then calls another routine using the modified values and an index.
- `execute_sequence_with_checks`: Calls a routine repeatedly until a certain value is returned, then updates a global variable with a constant value.
- `decode_bitstream_sequence`: Iterates through a bitstream or data buffer, using a state machine to decode or unpack compressed or encoded data into a target buffer.
- `parse_bitstream_sequence_2`: Decodes a variable-length bitstream or instruction sequence by reading bytes from a table and updating a global pointer.
- `parse_variable_bitstream`: Decodes a bitstream-encoded sequence by reading bytes from a table and updating a global counter or state.
- `parse_bitstream_sequence_3`: Decodes a variable-length bitstream or compressed instruction sequence by reading bytes from a table and updating a bit-counter or state-machine.
- `parse_bitstream_sequence_4`: Decodes a variable-length bitstream or compressed-style sequence by reading bytes from a table and using bit-masking or bit-shifting to determine the number of subsequent bytes to process or skip.
- `parse_bitstream_sequence_5`: Reads a sequence of bytes from a table and updates a global counter or state.
- `parse_bitstream_sequence`: Reads a sequence of bytes from a pointer-based table and updates a global counter or state.
- `decode_bitstream_sequence_3`: Iterates through a bitstream or data buffer, using a counter to determine the next state or branch.
- `decode_bitstream_sequence_2`: Decodes a bitstream-encoded buffer by reading a control byte and using its bit-fields to determine the number of bytes to write and the next state or jump.
- `process_bitstream_sequence`: Iterates through a data buffer or bitstream, using a counter to determine the next state or jump or branch.
- `process_entity_state_updates`: Iterates through a list of structures to update internal state values based on specific command bytes.
- `find_matching_pointer`: Searches a range of memory for a specific 32-bit pattern and returns its location if found.
- `iterate_entity_flags`: Iterates through a list of entity structures and calls another routine if a specific bit is set in a flag field.
- `iterate_entity_state_flags`: Iterates through a list of entity structures, checking a specific bit in a flag field and calling another routine if the bit is set.
- `increment_and_store_pointer`: Increments a global counter and, if it wraps to zero, calls a function to update a pointer, potentially resetting that pointer to a new value from a table.
- `update_table_pointer`: Updates a pointer in a table by adding a bit-shifted value from a register to an existing value.
- `decode_and_update_pointer`: Decodes a bitstream sequence and stores the result in a table, then potentially increments a value at a pointer-based offset.
- `decode_and_modify_target`: Decodes a bitstream sequence with a different parameter and updates a value at a location pointed to by a table entry.
- `populate_animation_table`: Iterates through a bitstream to decode or unpack compressed data into a table of pointers, while updating animation pointers or filling a buffer with a global byte value if certain bit-flags are met.
- `decode_and_recurse_data`: Decodes a bitstream sequence and, if the result is non-zero, stores it in a global variable and recursively calls itself using offsets from that value.
- `decode_and_iterate_sequence`: Decodes a bitstream sequence and, depending on certain flag bits, either sets a value in a global variable or enters a loop that calls another routine with incrementing pointers.
- `decode_and_store_result`: Decodes a bitstream sequence using a fixed mode and stores the resulting value into a pointer-based table entry.
- `decode_and_update_table`: Decodes a bitstream sequence and stores the returned value into a table entry located via a pointer in a global variable.
- `process_bitstream_animation_data`: Decodes a bitstream or compressed data buffer and updates animation or pointer tables.
- `update_animation_frame_buffer`: Copies a block of memory from a source pointer to a destination pointer and updates a global timer or counter based on a calculated offset.
- `update_animation_pointer`: Updates a pointer in a table by adding a bit-shifted value from a register to an existing value.
- `resolve_entity_reference`: Resolves a pointer or index to a specific entity within a table and updates its associated state or linked reference.
- `update_clamped_input_vectors`: Splits a 16-bit input value into two 8-bit components and calculates the difference from their previous values.
- `update_clamped_values`: Updates two values in a structure by retrieving data from tables based on an index, applying clamping logic to ensure theresults stay within specified minimum and maximum bounds.
- `call_empty_routine`: Calls another routine with a constant value of fifteen.
- `execute_conditional_callbacks`: Checks several global pointers and, if they are non-zero, executes the routines or library functions they point to.
- `execute_recursive_calls_2`: The routine calls itself repeatedly before returning.
- `initialize_library_pointers`: Loads library base addresses into global variables and checks if certain library functions return zero, potentially triggering a routine that clears an array or updates a global variable.
- `invoke_copy_data_table_with_offset`: Calculates a source address by adding an offset to a global variable and passes this address and a parameter to a routine that copies a table of words into a destination buffer.
- `iterate_and_call_function`: Iterates thirty-two times, calling a function with a global pointer and a value retrieved from a pointer-indexed table.
- `clear_global_pointer_target`: Clears a byte at a location derived from a global pointer and a pointer-indexed table, then calls another routine.
- `execute_loop_calls_2`: Calls a routine twice using a range or certain values, passing a destination buffer and a range or certain values.
- `call_function_with_split_parameter`: Calls a function with a constant value and calls another function using a value split from a single parameter and a constant.
- `process_string_table_entry`: Uses a parameter to calculate an offset into a table, retrieves a pointer, and calls a library function using a value from a global variable.
- `update_pointer_from_string`: Retrieves a pointer or identifier from a string-based table and stores the result in a global variable, with a conditional check for a zero return.
- `traverse_entity_list`: Recursively calls another routine if a certain bit is set in a field within a list of structures.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `call_function_with_global_pointer`: Calls a function using a value stored at a global pointer location.
- `call_function_with_global_pointer_alt`: Calls a function using a value stored at a global pointer location using a different function pointer.
- `call_function_with_conditional_arguments`: Calls a routine using a value stored at a global pointer location, then calls another routine using either a specific or swapped pair of values retrieved from memory locations pointed to by global variables.
- `call_function_with_indirect_values`: Calls a function using two values retrieved from memory locations pointed to by global variables.
- `call_function_with_parameter`: Calls a function using a global pointer and a provided parameter.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_structure_member`: Calls a library function and, if the return value is non-zero, updates a structure member or a global variable with a new value or a range.
- `update_structure_data`: Performs a sequence of memory copies and updates a value within a structure based on a conditional check.
- `update_structure_member_values`: Updates a structure or object by conditionally clearing or copying data between memory locations based on a conditional check.
- `execute_conditional_callbacks_2`: Calls a routine with certain parameters and updates a structure member or a global variable based on a conditional check.
- `perform_multiple_function_calls_2`: Executes a sequence of four function calls that manipulate values stored in a structure, including clearing a value and copying data between.
- `perform_multiple_function_calls`: Executes a sequence of four function calls that manipulate values stored in a structure, including clearing a value and copying data between locations.
- `set_structure_bounds_and_clear`: Calls a library function to create a structure, clears a memory range, and then calls another routine using a pointer or identifier retrieved from that structure.
- `update_global_with_sum`: Calculates a sum involving a global variable and a value from a library call, then stores a bit-manipulated version of that sum into a global variable.
- `accumulate_global_value`: Performs a complex arithmetic loop that accumulates values from a global variable and other memory locations into a global variable.
- `return_uninitialized_stack_value`: This routine returns an uninitialized value from the stack.
- `increment_global_with_sum`: Calculates a sum involving a global variable and certain memory locations, then adds a repeated global variable value and that sum to a global variable.
- `initialize_system_state_2`: Resets several global variables to zero or a constant and executes a sequence of indirect function calls and conditional branches.
- `execute_callback_list_callback`: Executes a routine or a sequence of routines retrieved from a table or a global pointer-based list.
- `call_wrapper_routine_2`: Calls another routine and returns.
- `load_and_call_function`: Loads two 32-bit values from a table into global variables and then calls a function pointer.
- `initialize_animation_parameters`: Calls a routine with a specific constant, executes an indirect function call, and then calls another routine with a different constant.
- `initialize_system_state`: Resets several global flags or counters and executes a sequence of library or internal routines before calling another routine with a global variable.
- `reset_global_flags`: Calls a routine with a constant, resets a global word to zero, executes two indirect calls, resets another global byte to one, and calls a final routine.
- `trigger_replay`: Calls another routine with a specific address and a global variable.
- `execute_multiple_initializations`: Calls a routine with a constant, then executes two subsequent indirect routine calls.
- `reset_system_state`: Checks a global flag and, if certain conditions are met, executes a sequence of routines or library calls before calling another routine with a global variable.
- `set_half_state_flag`: Checks a parameter, calls a routine with a constant if non-zero, sets a global byte to zero, executes an indirect call, and calls another routine with a certain constant.
- `initialize_memory_tables`: Sets certain memory locations to specific constant values using a loop.
- `sum_table_bytes`: Iteratively calls a function to fetch values from a table and accumulates certain bytes into a local variable.
- `update_tables_with_calculated_offsets`: Writes certain values to a table, calculates and stores new pointer-based offsets into a table, and calls a routine with a pointer from a table.
- `initialize_game_state`: Resets several memory-mapped or global arrays to zero and configures a table of parameters based on a specific or previously set value.
- `test_and_execute_conditional_routine`: Checks if a routine returns zero twice, then sets a flag in a global variable, executes a function using a pointer-to-function-pointer with two arguments, and resets the flag.
- `execute_sequence_of_function_pointers`: Executes a series of indirect function calls using addresses or offsets derived from a global pointer, and writes a constant value to a memory location.
- `execute_conditional_function_calls`: Checks a global variable and either executes a sequence of four function pointers or a single alternative function pointer.
- `update_conditional_state`: Calls a routine with a constant, conditionally sets a global byte, and updates a global word or pointer based on the result of an indirect call or a comparison.
- `call_offset_routine`: Calls another routine using a value calculated from a pointer and an offset.
- `recurse_with_parameter`: Checks a parameter and calls itself recursively with that parameter and a global variable.
- `call_library_with_parameters`: Calls a library function using a pointer and certain values retrieved from a table.
- `update_conditional_table_entry`: Initializes a memory range to a constant value and then conditionally updates a table entry based on a comparison with a library function return value.
- `store_word_to_global_location`: Stores a word from a pointer-based source into a destination address after checking a condition on a global value.
- `copy_word_from_pointer`: stores a word from a pointer-based source into a destination address.
- `recursively_call_routine`: Checks if a parameter is non-zero and, if so, calls itself with that parameter.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_table_data`: iterates through a loop to copy or update several values in a table based on conditional checks and pointer-based lookups.
- `search_table_for_index`: Iterates through a table to find an entry that satisfies a condition, potentially updating a value or returning an index.
- `update_table_with_conditional_increment_2`: Iterates through a table, checking a condition and potentially updating an entry with a value from a function call or an incremented pointer.
- `update_table_with_conditional_increment`: Iterates through a table or list, checking a condition or a value, and updates table entries with an incremented or modified value based on a calculation involving a division or a comparison.
- `update_loop_with_pointer_redirection_2`: Updates a pointer or value based on a table lookup while potentially looping or performing conditional assignments.
- `update_pointer_with_offset_value`: Calculates a destination address by adding a value from a pointer-based lookup to a base address and stores a decremented byte value there.
- `update_loop_with_pointer_redirection`: Increments a value in a structure and updates a pointer or value based on a table lookup, potentially looping or performing conditional assignments.
- `initialize_array_bounds`: clears a range of memory and then populates a secondary table with index-based offsets.
- `initialize_memory_and_set_flag_2`: Clears a range of memory and a specific word, then sets a memory location based on a comparison.
- `process_array_with_conditional_branching`: Iterates through a range of memory, updating a counter and conditionally updating a secondary value or a global word based on a comparison.
- `initialize_memory_and_set_flag`: Clears a range of memory and a specific word, then sets a certain memory location to zero or one based on a check.
- `zero_word_at_address`: stores a zero word into a specific memory location.
- `update_index_with_function_results`: iterates through a loop, calling a function via a pointer and storing the result or its negation into a table.
- `load_and_call_pointers`: calls two library functions, clears a table, loads several values from a library-referenced structure, and calls another routine for each loaded pointer.
- `execute_callback_list`: iterates through a table of pointers and calls another routine for any non-zero pointer, then calls two library functions.
- `process_recursive_parameters`: Executes a function call using a value from a pointer and then calls itself recursively with modified parameters.
- `dispatch_indirect_calls`: Calls three different routines using a pointer from a table, passing various combinations of provided parameters.
- `recursively_process_coordinate_offsets`: This routine recursively calls itself while applying or adding offsets from a table to certain parameters.
- `execute_nested_parameterized_calls`: Executes a sequence of parameterized calls that pass or modify certain values from a structure, specifically incrementing or decrementing certain or updating a value via a pointer-based routine call.
- `loop_parameterized_calls_with_offsets`: Executes a loop that calls a routine with a pointer and calculated offsets, then calls a routine via a pointer using a value from a table.
- `loop_parameterized_calls_with_offsets_2`: Executes a loop that calls a routine with a pointer and calculated offsets, or calls a routine via a pointer using a value from a table.
- `execute_parameterized_calls`: Calls a routine via a pointer using a value from a table, then calls another routine using four values retrieved from a structure.
- `call_wrapper_routine`: calls a function via a pointer with three parameters, calls a second function via a pointer, and then calls another routine using the first parameter, the fourth parameter, and the result of the second function.
- `search_table_and_load_word`: Performs a search through a table using an incrementing pointer and loads a word from an offset relative to a pointer.
- `increment_and_search_table`: Increments two values in a structure and searches through a table until a null terminator is found, returning a pointer or value.
- `load_word_from_offset`: returns a word loaded from an offset relative to a pointer.
- `call_with_offset_parameters`: Calls a routine via a pointer using a single parameter, then calls another routine using four parameters where two are sums of input values.
- `call_via_offset_table`: Retrieves bytes from a table using an offset and calls a routine twice if a specific byte in the table is non-zero.
- `update_entity_attributes`: Reads attribute or offset values from a table and dispatches them to a series of subroutines.
- `update_table_data_2`: Extracts multiple values from a table using an index and offsets, then calls three different routines with these values and updated pointers.
- `call_subroutine_via_pointer`: Calls a routine through a pointer stored in a global table using a passed parameter.
- `execute_indirect_call`: Fetches a function pointer from a table using an offset from a base address and calls it with a parameter.
- `execute_indirect_function_call`: Fetches a function pointer from a memory location and calls it using a value passed as an argument.
- `clear_memory_buffer`: Uses nested loops to set a range of memory or a multi-dimensional array or buffer to zero.
- `modify_table_values_with_offset`: Updates several memory locations in a table by adding a value from a register to the existing content, using complex address calculations.
- `add_to_table_value`: Calculates an address in a table using an index and an offset, then adds a value from a register to the memory at that address.
- `execute_state_callbacks`: Executes a sequence of indirect function calls and updates a global variable with a byte value.
- `update_state_and_recurse_4`: Updates a global variable with a return value from a routine and executes a sequence of indirect calls with or without conditional logic.
- `update_state_and_recurse_2`: Updates a global variable with a return value from a routine, resets a flag, and calls a routine with a pointer-derived or constant value.
- `update_state_and_recurse`: Updates a global variable with a return value from a routine, resets a flag, and calls a routine with a pointer-derived or constant value.
- `store_and_call_concatenated`: Calls a routine, stores a returned value in a global variable, and calls another routine using a concatenated 32-bit value.
- `update_state_and_dispatch_callbacks`: Clears a word in a global memory area and conditionally updates a byte if a certain value is detected.
- `dispatch_state_callbacks`: Updates a global variable and executes a sequence of indirect function calls or conditional branches.
- `tiny_move_move_move_ori`: Small routine of 4 instructions that copies -(a0) to d0, then copies -(a0) to d0, then copies -(a0) to d0, then merges 0 into d0.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a2).
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a7).
- `update_attribute_table_2`: Iterates through a table of structures to recalculate or copy attribute values based on a lookup or calculation involving a secondary table.
- `update_attribute_table`: Iterates through a table of structures to recalculate or copy attribute values based on a lookup or calculation involving a secondary table.
- `initialize_global_state`: Calls a routine with two arguments, stores the return value in a global variable, and then clears a small array of four or more words to zero.
- `call_routine_with_constant`: Calls a routine using a value stored in a global variable and a constant integer as an argument.
- `enable_dma_channel`: An 18-instruction routine that modifies the DMA controller register to enable a DMA channel.
- `update_dma_config`: Writes a value from a memory location to a hardware register and sets another hardware register to a constant value.
- `update_coprocessor_hardware`: Writes a longword and a word into hardware registers using an offset calculated from a provided parameter.
- `enable_dma_channels`: Sets a global variable to a value that enables DMA channels via the DMACON register.
- `write_hardware_registers`: Writes two or-sized values into a hardware register or a memory-mapped area using an offset derived from a parameter.
- `process_event_and_recurse`: Checks a table for a non-zero value, then uses a switch-case to update a flag and certain memory offsets or values before recursively calling itself with a modified or or original parameter.
- `update_state_and_recurse_3`: Updates a flag and a value in a memory structure based on a lookup or switch-case, then recursively calls another routine using a pointer-derived address.
- `update_dma_register_2`: Stores a constant or a value from a table into a memory location and calls another routine.
- `update_dma_register`: Stores a value into a memory location calculated from a pointer and a value fetched from a table, then calls another routine.
- `return_empty`: The routine is empty and returns immediately.
- `branch_on_input_value`: Checks an input register against several cases to perform conditional writes to memory or update a pointer before calling another routine.
- `return_empty_routine`: The routine performs no operations and returns immediately.
- `clear_and_set_pointers`: Clears a range of memory, calls a library function, and stores return values from other routines into a global variable.
- `update_table_from_lookup_2`: Iterates through a range of indices to update a destination memory area using values fetched from a secondary lookup table.
- `update_table_from_lookup`: Iterates through a table to update a destination memory area using values fetched from a secondary lookup table.
- `call_function_conditionally`: Checks a global variable and calls a function pointer with a specific value if that variable is non-zero, then calls the same function pointer with a different constant.
- `decrement_and_update_counters_2`: Decrements a global variable and, if it reaches zero, resets other global values; otherwise, it updates a range of counters based on certain conditions.
- `update_pointer_and_call`: Calls a function pointer, calls another function pointer with zero, and copies a value from a memory location to a global variable.
- `reset_function_and_copy`: Calls a function pointer with zero and copies a value from a memory location to a global variable.
- `reset_state_and_call_subroutines`: Clears a value in a global memory location and then executes two previously stored function pointers, passing a zero or no argument to the first.
- `execute_function_pointer`: Calls a function through a pointer stored at a specific offset from a base address.
- `configure_dma_and_clear_flag`: Clears a global flag and calls a routine to enable a specific number of DMA channels, followed by a routine that writes a value to a memory location.
- `branch_if_non_zero`: Tests a longword at a negative offset from a pointer in a4 and branches to another routine if the value is not zero; otherwise it returns.
- `call_with_memory_value`: Calls another routine using a value loaded from a memory location.
- `format_numeric_digits`: Calculates individual digits of a value, stores them in a local array, and iterates through a list to call another routine based on those digits.
- `process_conditional_list`: Iterates through a list, updating memory or calling another routine based on values in a lookup table and a counter.
- `process_conditional_function_list`: Iterates backward through a list of pointers, checking a condition in a lookup table and updating a value to a constant if a certain condition is met.
- `iterate_and_call_on_condition`: Iterates through a list and calls a function pointer with a value from a memory location if a certain condition in a lookup table is met.
- `get_absolute_value`: Returns the absolute value of the provided parameter.
- `read_offset_word`: Reads a word from a memory location offset by a fixed amount from a pointer held in a register.
- `copy_byte_sequence_2`: copies a specified number of bytes from a source memory location to a destination memory location.
- `copy_byte_sequence`: Copies a sequence of bytes from one memory area to another using a loop and returns the final index.
- `traverse_and_update_state`: Iterates through a linked list, updating a global variable with a value from a pointer or a byte, and calls another routine.
- `traverse_linked_list_and_update_pointer`: Iterates through a linked list by following pointers, updating a global pointer with the next element's address, and clearing a value in a memory location.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `update_position_and_clear_byte`: Calculates a new coordinate using a pointer-based offset and calls a routine with the result, then potentially clears a byte in a buffer if a certain condition is met.
- `decompose_and_render_value`: Decomposes a numeric parameter into individual digits stored in a local array, then iterates through that array to call another routine using a certain value.
- `parse_string_and_update_state`: Copies a null-terminated or colon-terminated string into a global buffer and updates a state-related global variable with the result of a routine call.
- `iterate_function_chain_with_state_check`: Iterates through a chain of function pointers, updating a global variable with the return value of each call and checking if the return value matches a previously stored value.
- `return_constant_value`: Calls another routine and returns a constant integer.
- `execute_function_chain`: Iterates through a chain of function pointers, updating a global variable with a return value and performing indirect calls.
- `process_function_chain_2`: Iterates through a chain of function pointers, updating a global variable and performing indirect calls.
- `traverse_function_chain`: Iterates through a chain of function pointers, updating a global variable with the return value of each call and executing the next function in the chain if the return value is non-zero.
- `process_function_chain`: Iterates through a chain of function pointers, updating a global variable with a value from a table and performing indirect calls.
- `return_without_action_2`: The routine consists of a single instruction that returns immediately, performing no work and touching no registers or memory.
- `handle_state_transition`: Checks a parameter, calls another routine, and either calls a library function or executes a loop that calls another routine and checks a return value.
- `handle_disk_swap_and_state_transition`: Checks a flag to determine if a disk swap or state change is required, then updates a global state variable and executes a sequence of conditional routine calls or an infinite loop.
- `write_table_values`: Iterates up to five times to write values from a table into a destination via a routine call, then resets a global variable if certain conditions are met.
- `execute_indirect_function_12`: Loads a function pointer from a memory location and calls it with a constant argument.
- `get_and_return_value`: Calls a routine and returns a value stored in a global variable.
- `branch_if_non_zero_and_clear_d0`: Tests a longword at a negative offset from a pointer in a4 and branches to another routine if the value is not zero; otherwise it clears d0 and returns.
- `initialize_global_parameters`: Sets several global variables or table entries to specific constant values or copies a value from a pointer-based source.
- `decrement_and_update_counters`: Decrements a global variable and, if it reaches zero, resets other global values; otherwise, it updates a range of counters based on certain bounds.
- `conditional_execute_routines`: Checks several global conditions and, if satisfied, executes two different routines based on a returned value and a counter check.
- `update_game_state_parameters`: Updates several global state variables, including timers and attribute flags, based on the value of an input parameter.
- `set_first_value`: Stores a constant value of one into a global variable and a different constant into another global variable.
- `update_global_flags`: sets a global flag to one and a secondary flag or value to forty.
- `update_entity_attributes_3`: Calculates a value based on a global variable and updates several properties in a list or table with new values or pointers from a table.
- `update_entity_attributes_2`: If a certain global value exceeds two, this routine updates a series or a list of properties by copying values from a table or a list, and sets several fixed integer values or constants into a structure or a global array.
- `set_attribute_constants`: Sets a global variable to a constant and updates other global variables with either constant values or a value from a table based on a comparison.
- `set_multiple_values`: Stores several constant values into a series of global variables.
- `update_entity_attributes_conditional`: Updates a set of entity properties or pointers from a table based on a conditional check of a register value.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `fill_buffer_with_count`: The routine iterates a specified number of times, writing the current loop counter value as a byte to a destination buffer and incrementing the pointer.
- `fill_buffer_with_nested_loop`: Iterates through a nested loop structure, calling another routine to retrieve a value and storing that value into a buffer.
- `iterate_and_store_bytes`: Iterates through a loop, calling another routine and storing its return value into a buffer until a certain condition or a certain value is met.
- `populate_buffer_from_source`: Calls another routine to retrieve a value, stores it in a buffer, and conditionally increments a counter or checks a limit.
- `store_bytes_with_limit_check`: Stores a value into a buffer and increments a counter, checking a limit or a loop counter to decide whether to continue or return a value.
- `get_next_byte_recursive`: Checks if a pointer-range or counter has not exceeded a limit, then increments the pointer and returns the byte at the previous location.
- `get_next_byte`: Returns a byte from a pointer and increments that pointer.
- `update_status_and_pointer`: Checks a bitmask in a structure and conditionally clears a bit, potentially iterating through a table or updating a pointer and a value based on a return value.
- `process_status_and_iterate_table`: Clears a bit in a byte within a structure and conditionally iterates through a table or calls another routine based on bitmask checks.
- `set_flag_bit`: Sets a specific bit in a byte within a structure and returns a constant value.
- `update_pointer_and_value`: Updates a pointer and an adjacent value in a structure using an offset, then increments the pointer.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `find_char_in_string`: Searches for a specific byte within a null-terminated string and returns a pointer to that byte or zero if not found.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_routine_11`: the routine performs no operations and immediately returns.
- `execute_indirect_jump`: Performs an indirect call to a function pointer stored in an unrecovered jump table.
- `check_input_mask_conditional`: Checks a bitmask in a pointer-referenced memory location and conditionally executes another routine based on a global variable.
- `check_input_mask`: Checks a bitmask in a pointer-referenced memory location and updates a global variable with a constant or a return value from another routine.
- `read_input_status`: Calls a routine with parameters from a pointer and checks if the result is negative, potentially storing a result from another routine into a global variable.
- `return_empty_routine_2`: This routine is empty and performs no operations before returning.
- `process_table_updates`: Iterates through a table and, if certain conditions are met, updates a secondary list with new pointers and values.
- `update_table_entries`: Iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `manage_library_resources`: Interacts with library structures to check status, execute functions, and update a pointer in a table.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `clear_memory_and_call_function`: Clears a block of memory using a loop and then calls a function pointer retrieved from a global variable.
- `return_empty_subroutine`: The routine is empty and performs no operations before returning.
- `parse_string_tokens`: Parses a source string to extract or copy space-delimited or quoted tokens into a destination array of pointers.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit_2`: Calculates the length of a source string and copies it to a destination buffer, stopping if a certain limit or a null terminator is reached.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `traverse_nested_pointers`: Traverses a series of nested pointers and executes indirect function calls, updating a global variable with a value from a table if certain conditions are met.
- `traverse_nested_structures`: Traverses a series of nested pointers and executes indirect function calls while updating a global pointer with a value from a table.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `apply_signed_sign_correction`: Checks the sign of an input and negates the result of a division routine if the input is negative.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `transform_words_and_divide`: Performs a complex sequence of bitwise shifts, swaps, and conditional subtractions on a 32-bit value, followed by a division-based calculation.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `store_byte_and_update_pointer`: Stores a byte from a source address into a destination pointed to by a pointer and then updates that pointer or a memory location via a bitmask check.
- `increment_pointer_and_store_byte`: Increments a pointer and stores a byte from a source address into the location pointed to by that pointer.
- `update_pointer_with_value`: Checks a bitmask and recursively updates a pointer or writes a byte value to a memory location pointed to by a pointer, or resets a pointer to zero.
- `update_pointer_logic`: Recursively checks a pointer and bitmask, potentially updating a pointer or clearing a bitmask and resetting a pointer to zero.
- `update_pointer_and_flags_2`: Modifies a pointer and a bitmask in a global variable.
- `update_pointer_and_flags`: Modifies a pointer and a bitmask in a global variable, and potentially copies a byte to a new memory location.
- `update_pointer_and_set_bit`: Updates a pointer, modifies a value at an offset, sets a bit in a byte, and increments a pointer while storing a byte.
- `update_memory_and_validate_index_2`: Updates a structure field with a constant or a pointer, and conditionally modifies a byte via a bitwise or or sets a pointer to a new address.
- `update_memory_and_validate_index`: Writes a constant to a memory location, modifies a byte via a bitwise or, copies a value from a pointer-referenced location, and validates a table index.
- `insert_into_linked_list_2`: Uses a library function to find a pointer and then inserts a new node into a linked list, updating a global pointer to the new head.
- `swap_pointer_values`: Copies a value from a source address to a pointer and then updates that pointer to point to a new location.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `validate_table_index`: Checks if a provided index is within a certain range and if a value in a table is non-zero, then updates a global variable and returns a boolean result.
- `check_interactive_status`: Calls a routine using a pointer calculated from a table and returns a boolean result based on whether the return value is non-zero.
- `return_zero_2`: The routine returns a zero value.
- `check_io_status_conditional`: Performs an I/O read check and conditionally executes another routine based on a bitmask check or a failed read.
- `check_io_status`: Performs an I/O read check and updates a global variable with a value from another routine if the read fails or if a bitmask check fails.
- `perform_io_read_check`: Attempts a read operation and, if it fails, stores a value from another routine into a global variable before returning an error code.
- `return_without_action`: This two-instruction routine does nothing but return immediately, as the pseudo-code shows an empty body consisting only of a return.
- `execute_conditional_function`: Calls a routine with specific arguments and, if a certain bit is set, conditionally calls another routine based on a global variable value.
- `execute_and_return_zero`: Calls another routine and then returns a zero value.
- `execute_write_sequence`: Calls a function via a pointer to process a global variable and then executes two subsequent routine calls.
- `execute_indirect_functions`: Checks a global pointer and, if non-zero, executes the routine pointed to by that address, followed by another indirect function call.
- `validate_and_clear_pointer`: Checks if an index is within a valid range and if a pointer in a table is non-zero, then potentially calls another routine and clears the pointer or sets an error flag.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `call_library_vector_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 6 of that base, returning its result to the caller.
- `call_library_vector_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 12 of that base, returning its result to the caller.
- `call_indirect_from_table`: Loads a function pointer from a table using an offset and executes it.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_date_stamp`: Wrapper that calls the dos library function DateStamp and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_address`: Executes a routine located at an address stored in a global variable.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_indirect_from_offset`: Loads a function pointer from a table using a fixed offset and executes it.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_pointer`: Loads a function pointer from a memory location calculated using a base address and an offset, then executes that function.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `execute_indirect_function_13`: calculates an address by adding an offset to a value in a register and executes the function pointer stored at that location.
- `call_indirect_from_library_table`: Loads a function pointer from a table offset relative to a library base and executes it.
- `execute_indirect_call_2`: Performs an indirect call to an address calculated by adding an offset to a register.
- `execute_indirect_call_from_table`: Loads a pointer from a table and performs an indirect call using an offset.
- `execute_indirect_function_14`: calculates an offset from a register and executes the code located at the resulting address.
- `execute_indirect_function_pointer_2`: Loads a function pointer from a nested table lookup and executes it.
- `execute_indirect_function_pointer_3`: Loads a function pointer from a nested pointer or table and executes it.
- `execute_indirect_function_15`: calculates an offset from a register and executes the function located at the resulting pointer in a table or memory block.
- `execute_indirect_function_pointer_4`: Loads a function pointer from a table located at an offset from a global pointer and executes it.
- `execute_indirect_function_16`: loads a function pointer from a memory location offset by a constant and jumps to it.
- `execute_indirect_function_pointer_5`: loads a function pointer from a nested offset within a table and executes it.
- `execute_indirect_pointer`: Loads a function pointer from a nested offset within a table and executes it.
- `execute_indirect_function_17`: Loads a function pointer from a nested offset within a table and executes it.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `execute_indirect_function_18`: Performs an indirect call to a function pointer located at an offset from a global pointer.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `execute_indirect_function_19`: Loads a pointer from a global table and calls a function at a fixed offset from that pointer.
- `execute_indirect_function_pointer_6`: loads a function pointer from a nested table or structure and executes it.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_function_20`: loads a function pointer from a table located at a fixed offset from a base address and executes it.
- `execute_indirect_function_21`: loads a function pointer from a table located at a fixed offset from a base address and executes it.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_function_22`: loads a function pointer from a table located at a fixed offset from a base address and executes it.
- `call_library_vector_486`: Library stub: loads a library base from a global variable and calls the library function at offset minus 486 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `execute_indirect_function_23`: Loads a pointer from a global table and calls a function at a fixed or calculated offset.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_indirect_function_24`: loads a function pointer from a table located at an offset from a global pointer and executes it.
- `execute_indirect_function_25`: Loads a function pointer from a table located at a fixed offset from a base address and executes it.
- `execute_indirect_function_26`: loads a function pointer from a nested table or structure and executes it.
- `call_indirect_from_table_2`: Loads a pointer from a table using an offset from a global variable and calls it.
- `call_indirect_from_table_3`: Loads a pointer from a table using a different offset from a global variable and calls it.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `execute_indirect_function_27`: loads a function pointer from a table located at a fixed offset from a base address and calls it.
- `execute_indirect_function_28`: loads a function pointer from a table using an offset from a register and executes it.
- `execute_indirect_function_29`: loads a function pointer from a table using an offset from a global pointer and executes it.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `execute_indirect_function_30`: loads a function pointer from a table located at a fixed offset from a global pointer and executes it.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][a](Disk 2 of 2).adf:bootblock

- `validate_and_load_disk_image`: Iterates through a table of offsets to check if any pointer or memory range points to valid custom chip or chip RAM, then proceeds to format strings and clear or reset memory or execution state based on the presence of valid or suspicious pointers.
- `reset_and_call_stored_vector`: Executes a processor reset and then calls the address held in a global variable.
- `format_string_to_buffer`: Formats a string using the RawDoFmt routine into a local stack buffer, then copies the null-terminated string to a destination buffer pointed to by a register.
- `append_byte_to_counted_buffer`: Stores a byte into a buffer and increments a counter if it is less than 255.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_30`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_31`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_32`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_33`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_34`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_35`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_36`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_37`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_copper_and_clear_memory`: Opens a library, copies a table of words into a memory block, clears a large memory region, configures DMA and copper registers, waits for a hardware flag, and closes the library.
- `service_pending_interrupt_flags`: Tests an interrupt flag bit and, when it isset, checks a second interrupt request bit and calls up to three helper routines.
- `write_next_table_entry`: Reads a byte from a pointer, wraps the pointer to a base address if the value is negative, and stores a word built from that byte and a constant into a global variable before advancing the pointer.
- `build_frame_copper_list`: Computes per-frame display geometry from state fields and fills a copper-style list of word pairs in memory.
- `update_hardware_copy_block`: Waits for a hardware busy bit to clear, then initializes hardware registers with control words and address pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_calls_2`: Calls a routine to initialize memory and then executes two subsequent routines via pointers stored in a table or memory offset relative to a register.
- `populate_memory_tables`: Uses a loop to write a sequence of incrementing or decremented values into several different memory tables.
- `update_sprite_attribute_table`: Iterates through a range of memory to duplicate or update sprite or object attribute data, including updating or clearing associated word-sized or byte-sized metadata or secondary attribute buffers.
- `initialize_memory_blocks`: Sets several global variables or memory locations to specific values and clears a series of byte-sized memory locations using a loop.
- `clear_memory_blocks`: Clears or zeros out a series of word-sized memory locations using a loop.
- `update_memory_tables`: Copies word-sized values from a source table to a destination table, using an offset or a value from a global variable to determine the source address.
- `initialize_string_buffer`: Allocates a memory buffer and initializes it with a specific header, then updates several global state pointers and flags.
- `copy_d0_to_d5`: Copies the contents of d0 into d5.
- `process_callback_list`: Iterates through a list of function pointers and executes them, followed by a routine that passes a calculated size or pointer to another function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_game_state_and_dispatch_logic`: Resets a large block of global state variables to zero or default values and recursively calls a state-management routine.
- `update_sprite_attributes`: Iterates through a table of object descriptors to modify certain attribute bytes, such as color or certain flags, and calls a routine to update the display or position.
- `filter_register_value`: Extracts a byte from a register and checks it against a list of prohibited values, returning a calculated offset if the value is not in the list.
- `return_immediately`: The routine performs no operations and immediately returns.
- `execute_empty_routine_2`: The routine calls an empty routine and then returns.
- `check_register_range`: Compares a register against a list of specific values and returns a modified or unchanged value based on those comparisons.
- `empty_routine_9`: The routine is empty and performs no operations.
- `clear_global_byte`: Stores a zero byte into a global variable.
- `update_global_pointers`: Executes a function pointer and then copies values from a table or structure into several global variables.
- `call_subroutines`: Calls two subroutines via pointers, passing zero as an argument to the first one.
- `update_state_counters`: Updates or resets several global counters and conditional flags based on their current values.
- `set_conditional_values`: Sets certain global bytes or words based on the value of a parameter and then executes a function pointer.
- `update_sprite_attributes_2`: Sets certain global words, then enters a loop that calls another routine and updates a global word with a modulo-reduced value.
- `execute_table_function_pointer`: Retrieves a function pointer from a table using an offset derived from the upper bits of a parameter and executes it with an argument loaded from a memory location.
- `reset_and_configure_game_state`: Initializes a table with a constant value and resets several state-related flags or counters.
- `fill_array_with_parameter`: Executes a function pointer and then loops to assign a parameter value and a constant to a table.
- `iterate_entity_attributes`: Iterates through a range of memory, checking certain flags and updating a pointer or a value in a table or structure.
- `update_entity_attribute_loop`: Iterates through a range of an array to update a memory-mapped attribute or pointer, and if a certain condition is met, updates a secondary table and calls two function pointers with a new coordinate or state.
- `scale_and_store_values`: Iterates through a nested loop to scale a value from a source table and store it in a destination table.
- `update_game_state_machine`: Updates a global state variable or dispatches new state-related logic based on a provided input or current table-driven state.
- `copy_from_table_offset`: Loads a word from a table using an offset and stores it into a global variable.
- `search_table_for_pattern`: Iterates through a table to find a specific pattern or value, returning a calculated index or pointer.
- `execute_until_zero`: Executes a function pointer repeatedly while certain global shorts are non-zero, then executes two more function pointers.
- `parse_string_delimiters`: Searches through a buffer for a specific character and replaces a delimiter character with a null terminator.
- `copy_player_sprite_attributes`: Uses nested loops to copy attribute data from a source buffer or table to a destination buffer, while simultaneously updating a set of global or local state variables with values retrieved from other memory locations or routine return values.
- `search_buffer_for_value`: Searches a buffer for a specific byte value and returns a value based on its position or presence.
- `resolve_conditional_pointer_2`: Calculates a value based on a global variable and a parameter, then returns a pointer or a fallback address using a switch-based bitmask check.
- `resolve_conditional_pointer`: Checks specific bits in a byte or a word to decide whether to return a value from a table or a fallback address.
- `empty_routine_10`: this routine performs no operations and returns immediately.
- `get_conditional_pointer`: Checks specific bits in a byte and returns a word-sized value from a memory location or a fallback address based on those bits.
- `read_offset_pointer`: Returns a word-sized value from a memory location calculated by adding a negative offset to a register.
- `process_state_transition_and_buffer_swap`: Checks a global flag to execute a sequence of recursive calls and function pointer dispatches, while conditionally swapping or updating values between a global variable and a memory location pointed to by a table.
- `get_bit_and_index`: Calculates a bitmask and a bit index or bit position from a provided value.
- `update_buffer_offset`: Updates a global variable or pointer offset based on a calculated value and checks it against certain bounds.
- `update_index_and_clamp_value`: Updates a global index by scanning a table for duplicate adjacent values, then clamps a secondary global value within a fixed range.
- `update_global_values`: Copies several dwords or words from a source table or pointer-based memory to a set of global variables and clears certain global variables to zero or one.
- `call_with_calculated_offset`: Calls a function pointer using a parameter and a value calculated from a table lookup and an offset-based calculation.
- `update_pointer_and_state`: Updates a pointer or a global word with a value from another memory location and conditionally sets certain global words based on a parameter.
- `clear_memory_and_assign`: Clears a range of bytes in a memory block, then calls another routine and stores its return value into a global variable.
- `execute_multiple_calls`: Executes a sequence of function calls, some of which pass a value retrieved from a global table or pointer-based memory.
- `set_constant_recursively`: Writes a constant value to a memory location, calls another routine with a value from a pointer, and calls itself recursively if a certain memory value is non-zero.
- `conditional_function_execution`: Calls a function, then checks if two global shorts are non-zero before calling another function that writes a constant to a memory location and passing a value from a global variable.
- `execute_set_constant`: Executes a function call, then calls another function to write a constant value to a global memory location, followed by a final function call.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_table_with_parameter`: Calls another routine with a constant, then performs a series of word-sized copies between a table and a global variable, including a recursive call.
- `update_buffer_from_table`: Clears a local buffer, then iterates through a range, reading values from a table and incrementing a byte in a buffer based on certain comparison logic.
- `sync_buffer_and_update_flag`: Checks if a memory range contains non-zero data, then performs a series of conditional memory-to-memory copies or clears a memory range, and updates a global byte if a certain counter or value is below a threshold.
- `copy_and_update_state`: Performs a series of memory-to-memory copies of varying lengths using a pointer, then updates a global variable if a certain condition involving a counter and a flag is met.
- `update_memory_and_set_flag`: Performs several memory-to-memory copies or writes using a pointer and a constant, then sets a byte in a global variable if a certain condition is met.
- `execute_memory_copy_sequence`: Calls a function to copy a range of memory, and if the return value is non-zero, performs further copies or checks a value via a function call.
- `copy_buffer_with_offset`: Copies dwords from a pointer-based table, calls a routine, and conditionally writes incremented bytes to a calculated memory offset.
- `accumulate_table_values`: Iterates through a nested loop to add values from one table into another table at calculated offsets.
- `update_table_with_max_value`: Iterates through a loop to compare values in a table and updates a global table with a new value if it is larger.
- `copy_and_parse_string`: Copies several values from a source table to a destination, then calls a routine to parse a string, and if successful, performs a memory copy and calls another routine twice.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 93 into d0, then returns.
- `find_marker_and_call`: Iterates through a table until a specific byte value is found, then calls another routine multiple times using the discovered offset or an incremented offset.
- `populate_buffer_from_table`: If a certain condition is met, copies bytes from a table into a destination buffer using an offset, or otherwise checks a table and updates a local counter.
- `swap_array_elements`: Searches a table for a matching value and swaps two adjacent elements in that table if a certain condition is met.
- `map_buffer_indices`: Iterates through a buffer and, if a certain byte is non-zero, copies a byte from a source table to a destination table at a calculated offset.
- `empty_c_function_5`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `initialize_and_reorder_pixel_buffer`: Clears a secondary buffer and then reorders or copies pixel-related data into a new table structure based on conditional checks of existing values.
- `compare_and_recurse_table`: Iterates through a table and calls another routine if certain adjacent or nearby values in the table match.
- `compare_table_attributes`: Compares two values from a table and calls another routine recursively or otherwise, depending on whether the values or their corresponding table attributes match or satisfy a comparison.
- `reorder_table_by_attribute_comparison`: Clears a counter array, then iterates through a range of indices to compare certain attribute values and increment counters if a condition is met.
- `swap_table_values`: Reads two word-sized values from a table and writes them back to a different index in the same table, effectively swapping or overwriting values.
- `update_table_with_sum`: Calculates the sum of two short values from a table and passes this sum, along with a pointer and a constant, to a series of called subroutines.
- `process_pixel_value_differences`: Clears a memory buffer and then iterates through a table of byte values, comparing adjacent bytes and updating or decrementand values in a separate memory area based on the comparison results and a call to another routine.
- `find_matching_differences`: Iterates through a table and compares two values, storing an index and a difference into a destination table if a match is found.
- `execute_empty_routine`: The routine is called by another routine that performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine`: Calls a subroutine at a fixed offset and continues execution into the next routine.
- `call_subroutine_2`: Calls a subroutine at a fixed offset and continues execution into the next routine.
