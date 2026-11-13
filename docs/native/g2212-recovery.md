# Game 2212 (tv sports basketball (1990)(cinemaware)[cr pna](disk 1 of 2)[b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02212). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (925428 top-level bytes in 46 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 27 of 27 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 946 of 946 named (0 author symbols), 946 of 946 described (222 by rule, 704 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3480 of 3482 (99.94%) |
| Recovered without load address (counted in proof 1, flagged) | 25591 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14487 of 14506 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 393252 |
| code | 61052 |
| data | 259631 |
| padding | 10277 |
| unused | 201216 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| More | 19 | 2321 | 79 |
| Type | 2 | 363 | 12 |
| Assign | 2 | 435 | 7 |
| EndCLI | 2 | 182 | 3 |
| Dir | 2 | 419 | 11 |
| Echo | 2 | 241 | 10 |
| Lab | 1 | 2 | 1 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 601 | 42 |
| Makedir | 2 | 211 | 4 |
| CD | 2 | 282 | 5 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 413 | 15 |
| IconX | 12 | 806 | 37 |
| Disk-Validator | 1 | 10 | 2 |
| paranoimia | 2 | 83 | 3 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2)[b].adf:bb | 3 | 8490 | 465 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2)[b].adf:bootblock | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 115 | 115 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 220 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 492 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1584 | 95 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1076 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| paranoimia#unpacked | 1 | 55 | 1 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2)[b].adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 38 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 192 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 183 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 114 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 456 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 2 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 92 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- tv sports basketball (1990)(cinemaware)[cr pna](disk 1 of 2)[b]/TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2)[b].adf:S/paranoimia: crunched executable unpacked with ancient scan (ByteKiller 1.3 (xfd), 6920 -> 12322 bytes), 5 of 5 windows confirmed in reference RAM, loaded at an address: recovered as u25
- tv sports basketball (1990)(cinemaware)[cr pna](disk 1 of 2)[b]/TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2)[b].adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate
- 2 never-loaded region(s), 25591 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_function_pointers` (More): Calls a routine with a pointer to a global variable, calls another routine, and then invokes a function pointer retrieved from a pointer. Kind: sabotage path. Pass path: the execution of u0__sub_2_0D82() or any instructions following the pointer dereference.

## Functions


### More

- `initialize_game_environment`: Initializes the game's internal state by restoring patch states and loading file metadata.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `process_character_glyph`: Checks a character or byte against a set of specific values and, if certain conditions or range-based checks pass, updates a buffer or calls a routine to handle character rendering or state modification.
- `dispatch_entity_actions`: Checks a longword at offset 60 of a structure in a2; if non-zero, it calls a flag dispatcher and an entity updater, otherwise it calls a text window drawer and another routine via a pointer at offset 52 of a2.
- `execute_function_pointers`: Calls a routine with a pointer to a global variable, calls another routine, and then invokes a function pointer retrieved from a pointer.
- `initialize_entity_state`: Initializes fields in a structure and invokes another routine with values or pointers from that structure, conditionally calling further routines or looping based on certain structure fields.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from an input pointer, stores the second result into a field of the pointed-to structure, and if a flag field is zero it increments a short located at an unable to determine offset.
- `parse_buffer_content`: Uses another routine to check a buffer, updates a size field, and scans for a newline or form-feed character while potentially modifying a flag or string content.
- `process_entity_list`: Determines a loop limit based on fields in a passed structure and iterates to call another routine using a pointer to a table and a global variable.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Runs a sequence of calls to another routine using arguments from a parameter block and a global variable, returning the result of one of those calls.
- `copy_and_check_flag`: Copies a string and then checks a short value at an offset from a pointer to decide whether to call another routine with one argument.
- `draw_scroll_window_bounds`: The routine calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: The routine calls a drawing routine with a fixed or zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `draw_text_line`: Uses a handler pointer to call drawing primitives while iterating over a text buffer, tracking line count and stopping if a certain character or flag is encountered.
- `dispatch_recursive_pointer`: The routine calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a unable to determine if it is a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, if certain conditions are met, updates several offset and remaining size fields and calls another routine with a pointer derived from the structure.
- `recursively_adjust_buffer_bounds`: The routine recursively traverses a structure, checking if a certain field equals a fixed value, and updates a pointer or size field based on the return value of a recursive call.
- `recalculate_window_bounds`: Calculates a target position based on provided offsets and checks it against a window's current boundaries.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_uppercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `process_message`: Recursively processes a structure by invoking another routine on certain fields and string arguments, then conditionally updates structure fields and invokes further routines.
- `build_message_buffer`: Obtains a pointer from another routine and then calls another routine with that pointer, a constant, a stack-allocated buffer containing the input parameter, and a size of one.
- `copy_entity_data`: Obtains a handle from another routine and, if valid, reads a value from that handle, passes it to another routine together with a small stack buffer, and copies two fields from the handle into a provided structure.
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
- `invoke_modular_inverse`: Calls a routine to perform a 32-bit modular reduction or 16-bit modulo operation and returns.
- `swap_and_call_inverse`: Swaps the values of d0 and d1, calls another routine, and restores d2 from the stack before returning.
- `calculate_modular_inverse`: Manipulates d1 and d2 using negation and bitwise inversion, then calls modular_inverse_32bit with a pointer in a0 before returning a value in d2.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_pointer`: Stores a byte from a register through a pointer.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: Removes a port from a global list if a specific field is non-zero, sets status fields to indicate deletion, and calls another routine to release resources.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: Copies a fixed number of words from a source address to the stack if the stack pointer is not null.
- `update_multiple_table_values`: Sets a bit in d0, calculates a new value for a1, clears several offsets in a1 and a2, and performs a long sequence of calls to a routine in a5 using various offsets from a1, a2, and a0 to update multiple values in a2 and a1.
- `branch_to_offset_3`: Performs a short branch to a specific offset.
- `set_global_byte`: Sets a byte at a2 offset 644 to negative one and then branches to another location.
- `set_structure_byte`: Sets a byte at a1 offset determined by a4 to negative one and then branches to another location.
- `call_with_offset_byte`: Reads a byte from a0 at an offset calculated from a1, then calls a routine in a5 using a pointer from a2 and branches to another location.
- `compare_and_branch`: Calculates an offset using values from a1, fetches a byte from a0, calls a routine in a5 using a pointer from a2 offset 300, and branches to different locations depending on whether the result in d1 matches 72 or 78.
- `update_table_and_call`: Increments a longword at an offset of a1, fetches a byte from a table in a0 using an index from a1, and if it is less than the value at an offset of a1, performs several calls to a5 using pointers and constants derived from a2 and a1.
- `execute_conditional_sequence`: Stores a value in a2 offset 616, calls a routine in a5 using a pointer from a2 offset 216, and then performs a series of conditional calls to a5 using pointers from a2 at offsets 220, 148, 248, and 292.
- `branch_to_offset`: Performs a relative branch to an offset.
- `execute_indirect_call_6`: Loads the constant 232 into d0, loads a pointer from offset 336 of a2 into a4, calls the routine pointed to by a5, and jumps to the address in a6.

### Assign

- `initialize_data_structures_2`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with an incrementing sequence of values.
- `restore_stack_values`: Copies a sequence of words from a source address to a destination address if a pointer is non-null, then calls another routine.
- `execute_callback_sequence`: Performs bitwise operations on a4, shifts d1, stores d1 in a structure in a1, and calculates several offsets to store values in a1.
- `execute_multiple_callbacks`: Performs a series of operations involving shifting values from a1 and a0 to access structures, calling a function pointer in a5 with various parameters and offsets, and updating several longwords in a1 based on the results of these calls and comparisons.
- `update_structure_via_callback`: Moves a value from a structure in a1 into a location in a0, calls a function pointer in a5 with a parameter from a2, and if the result is zero, performs further checks on a structure in a1 before calling the function pointer again with different parameters and updating a value in a1.
- `assign_and_execute_callbacks`: Calculates an offset using values from a1 and a0 to access a structure, performs several calls to a function pointer in a5 with various parameters, and updates values in a1 based on results.
- `assign_and_call_functions`: Manipulates several 32-bit values using offsets from a1 and a0, performs multiple calls to a function pointer in a5 using various parameters and offsets from a2 and a4, and finally jumps to a location in a6.

### EndCLI

- `initialize_data_structures_3`: Calculates the maximum value in a linked list or table, then populates a memory-mapped array with a sequence of incrementing values.
- `copy_metadata_block`: copies a fixed-size block of data from a memory offset to the stack and then executes a previously stored function pointer.
- `execute_multiple_function_pointers`: Performs several operations involving shifting values, updating a structure, and calling various routines via function pointers stored in a structure.

### Dir

- `initialize_memory_tables_2`: Calculates the maximum value in a linked list of memory blocks and populates a table with incrementing or decrementing values.
- `copy_block_and_execute_2`: copies a fixed-size block of 16 or 17 words from a memory location or table to the stack before calling another routine.
- `process_game_state_logic`: A routine that calls a handler to fetch and call a handler, then calls a routine to execute conditional logic sequence.
- `fetch_and_call_handler`: Tests a longword at offset 332 of a1, and if it is zero, branches to another routine; otherwise it loads several longwords from a1 into d1, d2, and d3, loads 368 into d0, sets a4 to an offset from itself, calls a routine in a5, stores d1 at offset 352 of a1, and branches to another routine.
- `clear_offset_on_test`: Tests a longword at offset 320 of a1; if it is non-zero, it clears that longword and branches to another routine, otherwise it branches to another routine.
- `execute_multiple_calls_3`: Moves a longword from an offset of a1 into d1, moves 368 into d0, moves a pointer from an offset of a2 into a4, calls a routine in a5, moves d1 to an offset of a1, tests d1, and if d1 is not zero, performs several more calls to a5 with different offsets of a4 and d0 values before branching.
- `update_state_via_calls`: Checks a longword at an offset of a1, calls a routine in a5 with various parameters, and modifies bytes in a table at an offset of a0 based on values from a1.
- `update_game_state`: Checks a value at offset 88 of a1 against 2, then performs a series of calls to a routine in a5 with various parameters and offsets, while updating several values in a1 and a0 based on data from a table.
- `execute_conditional_logic_sequence`: Checks several longwords in a2 and a1, then calls a routine in a5 using parameters derived from offsets of a1, a2, and a4.
- `calculate_index_and_call`: Adds a value from offset 88 of a1 to 2, shifts the result, and calls a routine in a5 using a pointer from offset 3f8 of a4 and a parameter derived from the shifted value.
- `execute_conditional_logic`: Calls a routine with a parameter of 464, then performs a second call with a parameter of a pointer offset by -2944 from a value in a4, and if a value from a1 offset 344 is zero, sets a global variable at offset 620 to 20 and copies a value from a1 offset 348 to offset 612 of a2.

### Echo

- `initialize_global_metadata_table`: Iterates through a linked list or table to find a maximum value, then populates a new table or memory region with an incrementing sequence of values.
- `copy_block_and_execute_3`: copies a fixed-size block of 16 or 17 words from a memory location or table to the stack before calling another routine.
- `update_pointers_via_table`: Performs a bitwise operation on a value at a4, stores a shifted version of d1 into a1, calls a routine using a pointer from a2 and a value from a5, and if d1 is not zero, calls another routine using a different pointer from a2 and a value from a5 before branching.
- `update_table_pointers`: Tests a longword at an offset in a0 determined by a value in a1, and if non-zero, calls a routine in a5, stores the result in a1 at offset 328, then performs a second call to a routine in a5 using a different offset in a0, stores the results in a1 at offsets 332 and 336, and finally performs arithmetic on the stored values and branches to another routine.
- `move_and_branch_3`: Moves a longword from offset 370 of a1 to d1, moves d1 to offset 360 of a1, and then branches to a location specified by d4.
- `calculate_structure_offset`: Stores 1 at offset 328 of a1, fetches a longword from a1, shifts it left by 2, tests a longword at offset 12 of a table in a0 using that value as an index, and if both a longword at offset 332 of a1 and the table entry are non-zero, calculates a value based on offsets 328 and 332 of a1 and branches.
- `shift_longword`: Moves a longword from an offset of a1 into d1, moves d1 to a different offset in a1, and branches.
- `update_structure_and_call_handler`: Shifts longwords within a structure at a1, fetches a byte from a table in a0 using an index derived from a1 and a1's first longword, and calls a routine in a5 with d0 set to 400 and a4 pointing to a location from a2.
- `check_and_call_handler`: Compares a byte at an offset in a0 to zero, and if it is not zero, calls a routine in a5 using a pointer from offset e0 of a2 with a parameter of 160.
- `perform_multiple_calls`: If a value at offset 584 of a2 is not zero, it calls a routine with parameter 364 using a pointer from offset 264 of a2, then calls another routine with parameter 384 using a pointer from offset -104 of a2; if the second call's return value is not zero, it calls a third routine with parameter 384 using a pointer from offset 224 of a2.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Execute

- `initialize_global_metadata_and_stack`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with an incrementing sequence of integers.
- `copy_metadata_block_2`: copies a block of sixteen bytes from a memory location offset by a register or pointer to a stack or memory destination, then calls another routine.

### Copy

- `initialize_stack_and_metadata_table_2`: Calculates the maximum value in a linked list or table, then populates a memory region with an incrementing sequence of values and copies a block of data from a source pointer to a destination pointer.
- `copy_memory_block`: copies a fixed-size block of 16 bytes from a memory location offset by a register-held pointer to the stack.
- `update_table_and_call_handler`: Calculates a value based on a1, stores it at offset 12 of a1, clears several offsets in a1 and a2, calls a routine using a pointer in a5, fetches a byte from a table in a0 using an index from a1, and if the byte is non-zero, calls the routine in a5 again and stores 1 at offset 536 of a1.
- `copy_bytes_from_table`: Calculates an index by adding values from a1 and a2, fetches a byte from a table at a0 using that index, stores it in a1, repeats this for a second byte, and stores both bytes into a1 before branching.
- `execute_double_calls`: Adds 1 to a value at offset 540 of a1, calls a routine using a pointer in a5 with a parameter in d0 and a pointer in a4, then repeats this process using a value from offset 516 of a1 and branches back to an earlier instruction.
- `execute_conditional_double_calls`: Adds 1 to a value at offset 540 of a1, calls a routine using a pointer in a5 with a parameter in d0 and a pointer in a4, and if the longword at offset 516 of a1 is non-zero, calls the routine in a5 again before branching back to an earlier instruction.
- `compare_and_branch_3`: Loads a longword from offset 544 of a1 into d1 and branches to another routine if d1 equals 35, 37, 40, or 124; otherwise it branches to another location.
- `compare_and_jump`: Increments a longword at an offset of a1, compares it to a longword at an offset of a2, and branches if the first is greater than or equal to the second.
- `jump_on_test`: Tests a longword at an offset of a pointer in a2; if it is not zero, it jumps to another location, otherwise it clears d1 and jumps to the address in a6.
- `update_table_values_4`: Increments a longword at a positive offset of a1, calculates an index using d1 and a5, fetches a byte from a table in a0, stores it in a2, performs several longword moves and clears between a1 and a2, and finally fetches another byte from a table to store in d2 before branching.
- `branch_to_offset_2`: Performs a short branch to a specific offset.
- `fetch_byte_and_call_handler`: Calculates an index using a value from a1 and a1's first longword to fetch a byte from a0, stores it in a1 at offset 552, and if a value at a2 offset 700 matches that byte, fetches another byte from a0 using a different index and calls a routine in a5 with d0 set to 568.
- `compare_multiple_constants`: Loads a longword from an offset of a1 into d1 and compares it against several constant values, branching to different locations if matches are found.
- `increment_and_branch`: Adds 1 to a value at offset 220 of a1 and branches to another location.
- `branch_to_offset_5`: Performs a relative branch to a location 82 bytes ahead.
- `test_and_set_constant`: Tests d1 and, if non-zero, branches; otherwise it stores 255 in a2 at offset 720 and branches to another location.
- `compare_and_increment_counter`: Moves a value from offset 684 of a2 to offset 4 of a1, compares it to 1, and if not greater than the value at offset 4 of a1, adds a value from offset 680 of a2 to 1 and checks a byte in a table.
- `store_word_at_offset`: Increments a value at offset 716 of a2, calculates an index by adding the value at offset 712 of a2 to the result of shifting that value left by 2, and stores a longword from a1 into a memory location at a0 plus that index.
- `compare_and_set_byte`: Compares a value at offset 2c4 of a2 with a value at offset 2c0 of a2, and if the first is not greater, stores -1 in offset 2bc of a2 before branching.
- `store_byte_at_offset`: Increments a value at offset 704 of a2, calculates an index by adding the value at offset 724 of a2 to the result of shifting that value left by 2, and stores a byte from a0 plus that index into offset 700 of a2.
- `call_twice_if_equal`: If a value at offset 700 of a2 is equal to 39, it calls a routine twice with parameter 12 and a pointer offset by -48 from a4.
- `copy_and_call_routine`: Copies a longword from a2 offset 544 to a1, copies another longword from a2 offset 572 to a1 offset 4, and calls a routine in a5 with d0 set to 20 and a4 pointing to a location relative to a4.
- `set_byte_and_load`: Stores 255 into offset 712 of a2 and loads a longword from a1 into d1.
- `call_with_offsets`: Calls a routine with parameter 32 and a pointer from a4, then calls another routine with parameter 20 and a pointer from a4 plus 260, and finally loads a longword from a1 into d2.
- `call_and_update_status`: Loads a longword from a1 into d1, calls a routine with parameter 20 and a pointer from a4 plus 132, stores d1 back into a1, and if a value at offset 1796 of a2 is not 41, stores -1 into offset 712 of a2 and calls a routine with parameter 20 and a pointer from a4 minus 28.
- `branch_on_constant_match`: Compares a longword at offset 4 of a1 against several constants, including -1, 35, 40, 41, and 124, and branches to different locations depending on the match.
- `call_with_conditional_logic`: Clears a longword at an offset of a1, calls a routine in a5 with 20 in d0 and a pointer from a negative offset of a4, moves the result into an offset of a1, compares a word at an offset of a2 to several constants, and calls a routine in a5 with 24 in d0 and a pointer from an offset of a4 if certain conditions are met.
- `store_byte_and_call_handler`: Calculates an index using a value from a1 and a2 offset 696, stores a byte from a2 offset 707 into a0 at that index, copies a longword from a2 offset 568 to a1, and calls a routine in a5 with d0 set to 24.
- `call_with_offset_parameter`: Loads values from offsets 2c0 of a2 and 8 of a1, then calls a routine in a5 with a parameter of 24 and a pointer from offset 80 of a4.
- `branch_to_next_routine`: Performs a no-operation and then branches to another location.
- `loop_byte_copy_and_store`: Enters a loop where it calculates an index using a value from offset 696 of a2 and a pointer from a1, copies a byte from a0 plus that index to offset 8 of a1, and then copies a byte from a0 plus that index to the location pointed to by a1, repeating until the value at a1 is zero.
- `transfer_and_jump`: Stores d1 into a1 at offset 8, and if d1 is non-zero, branches; otherwise it jumps to the address in a6.
- `branch_to_offset_40e`: Performs a short branch to a location within another routine.
- `loop_byte_copy_until_zero`: Enters a loop that calculates an index using a value from offset 696 of a2 and a pointer from a1, copies a byte from a0 plus that index to a location pointed to by a1, and repeats this until the byte copied into d2 is zero, then performs one final byte copy from a0 plus the index to the location pointed to by a1 offset 7.
- `initialize_and_copy_byte`: Clears several longwords at offsets 680, 684, 700, and 708 in a2, stores values from d1 and d2 into offsets 692 and 696 of a2, fetches a byte from a0 using an index in d3, and enters a loop that copies bytes from a0 to a destination based on values in a1 and a2.
- `execute_three_calls`: Performs three consecutive calls to a routine in a5 with varying arguments in d0 and d1 or d2, then jumps to the address in a6 after bitwise inverting a value from a2.
- `copy_multiple_bytes`: Fetches two bytes from a0 using indices in d3 and d5, stores them into a1 at offsets 8, 12, and 16, and enters a loop that copies bytes from a0 to a destination using offsets derived from a1 and a2.
- `store_calculated_byte`: Adds a value from a1 offset 12 to a value from a1 offset 8, then stores the resulting byte at an offset in a0 determined by a bit-shifted value from a1 offset 4.
- `transfer_byte_with_calls`: Copies bytes between offsets in a0 using values from a1, performs a bitwise operation, and calls a routine in a5 using a pointer from a2.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `execute_multiple_calls`: Calls a routine in a5 using a pointer from a2 offset 116, then if the result in d1 is zero, performs two additional calls to a5 using pointers from a2 offsets 292 and 336 with specific values in d0 and d1.
- `update_global_long`: Moves a longword from a2 offset 604 to a0 offset determined by a value in a1 offset 4, then stores that same value into a2 offset 604.

### Makedir

- `initialize_stack_and_metadata_table_3`: Calculates the maximum value in a linked list or table, then populates a memory region with an incrementing sequence of integers.
- `copy_block_and_branch_2`: copies a fixed-size block of 16 or 17 words from a memory location or table to the stack before calling another routine.
- `perform_system_calls`: Calculates a value based on a1, stores it in a1, and then executes a sequence of calls to a5 using pointers from a2 at offsets 312, 40, and 292 with various parameters in d0 and d1.
- `perform_multiple_system_calls`: Executes a sequence of calls to a routine in a5 using various values from a1 and a2, including conditional branches based on the results.

### CD

- `initialize_stack_and_metadata_table`: Calculates the maximum size of a linked list or table, then populates a memory region with a sequence of incrementing values and copies a block of data from a source buffer to a destination buffer or stack-relative location.
- `copy_block_and_branch`: copies a fixed-size block of 16 or 17 words from a memory location to the stack before calling another routine.
- `initialize_and_dispatch_calls`: Increments a value at a3, calculates and stores several longword offsets in a1, clears a value in a2, and performs a sequence of calls to a5 using various pointers from a2 and offsets from a1 and a4.
- `dispatch_handler_calls`: Calls a routine in a5 using a pointer from a2 offset 292, and if a byte at an offset in a0 is non-zero, performs further calls to a5 using pointers from a2 at offsets 492, 296, and 336.
- `perform_batch_draws`: Performs a series of calls to a routine in a5 using various offsets from a2 and a1 as arguments, including loading values from a table using a scaled index in d2, and ends with a jump to a6.

### EndIf

- `return_zero`: The routine is empty and always returns a zero value.

### If

- `initialize_data_structures`: Calculates the maximum value in a linked list or table and populates a memory-mapped array with an incrementing sequence of values.
- `copy_block_and_execute`: copies a fixed-size block of sixteen bytes from a memory location offset by a register value into the stack, then calls another routine.
- `test_and_modify_structure_bytes`: Loads a longword from a pointer in a1, shifts it left by two, and calls test_and_branch_on_byte to compare a byte at an offset in a0 against a value in d2.
- `test_and_branch_on_byte`: Loads a longword from a1, shifts it left by two, tests a byte at an offset in a0, and if that byte is not zero, compares d2 with a value from a2 and branches or sets d1 to negative one and branches.
- `test_table_entry`: Loads a longword from a1 into d1, shifts it left by two, tests a byte at an offset in a0 determined by the new d1, and branches if that byte is zero; otherwise it compares d2 with a value at offset 548 of a2 and branches accordingly.
- `process_nested_table_indices`: Fetches longwords from a table in a0 using an index from a1, stores them in a1, and if certain bytes in the table match 36, calls a routine in a5 with a parameter in d0 and updates several offsets in a1.
- `execute_nested_table_checks`: Calls a routine using a pointer from a2, stores the result in a1, checks a byte in a table at a0 using an index from a1, calls another routine using a different pointer from a2, checks another byte in the table, calls a third routine, and performs several conditional tests and branches.
- `branch_to_offset_254`: Performs a short relative branch to a location 254 bytes ahead.
- `check_structure_byte`: Loads a longword from a1 into d1, shifts it left by two, tests a byte at an offset in a0 determined by d1, and branches if that byte is zero; otherwise it compares d2 with a value at offset 356 of a1 and branches if they are not equal.
- `branch_to_offset_4`: Performs a short branch to an offset.
- `compare_and_branch_2`: Compares a longword at offset 356 of a structure in a1 to zero, and if it is not zero, branches to another routine; otherwise it sets d1 to negative one and branches to another routine.
- `check_and_call_handler_2`: Fetches a longword from a table in a0 using an index from a1, and if it is non-zero, calls a routine in a5 with 372 in d0 and a pointer from an offset of a2, then moves the result to an offset of a1 and performs a second conditional call to a5.
- `check_condition_and_call_handler`: Sets a longword in a1 at offset 328 to zero, checks a byte in a0 at an index from a1, and if non-zero, clears a value in a1 at offset 356 and calls a routine in a5 with d0 set to 380.
- `process_conditional_logic`: Calls a routine in a5 multiple times with different parameters, performs bitwise NOT and OR operations on values stored in a1, and uses a loop to wait for a value in d1 to equal 32.
- `process_conditional_logic_2`: Calls a routine in a5 using a pointer from a2 offset 216, enters a loop that repeats this call until d1 matches the value in a2 offset 600, and then performs further conditional calls to a5 and jumps to the address in a6.

### IconX

- `initialize_game_environment_2`: Initializes the game's internal state by opening required libraries, allocating memory, and parsing a configuration string or file header to populate metadata structures.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer, then conditionally invokes several cleanup callbacks.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a buffer and walks a linked list of entries, copying the entries whose name bytes are above a certain range into it and passing them to helper routines.
- `concatenate_strings_and_pass_result`: Copies a source string into a destination buffer, appends a second string, and passes the new buffer and its total length to another routine.
- `count_string_length`: Receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `execute_supervisor_call`: Executes a routine called via a pointer stored in a global variable.
- `execute_indirect_function`: loads a function pointer from a location relative to a global pointer and executes it.
- `execute_indirect_handler`: loads a function pointer from a global table and executes it.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Loads a function pointer from a global table using an offset and jumps to that address.
- `execute_indirect_function_3`: Loads a function pointer from a location relative to a global pointer and executes it.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `execute_indirect_function_4`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer`: loads a function pointer from a global table and executes it.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `execute_indirect_function_5`: loads a function pointer from a global table and jumps to it.
- `recurse_with_stack_slot_2`: Calls another routine passing two parameters and a pointer to a specific location on a stack buffer.
- `execute_indirect_function_6`: loads a function pointer from a memory location offset by a constant and executes it.
- `execute_indirect_function_7`: loads a function pointer from a global table using a fixed offset and executes it.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `execute_indirect_function_8`: loads a function pointer from a global table using a constant offset and executes it.
- `execute_indirect_function_9`: performs an indirect call to a function pointer located at a different offset in a global table.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found, and passes the buffer and checksum to another routine if the inverted checksum is non-zero.
- `store_byte_to_pointer_2`: Stores a byte from a register into a memory location pointed to by a register.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_register_to_pointer`: stores the contents of a data register into a memory location pointed to by a register.

### Disk-Validator

- `apply_xor_decryption_loop`: Iteratively XORs bytes at a code location with a decreasing value and copies a word from one memory location to another.
- `copy_word_to_memory`: copies a word from one memory location to another using an offset from a register.

### paranoimia

- `decompress_bitstream_with_checksum`: Unpacks a compressed bitstream by reading bits and copying back-referenced runs backward into memory, then verifies a checksum.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2)[b].adf:bb

- `initialize_and_execute_entry`: Clears a block of memory, sets a global pointer, and calls a function pointer retrieved from a global variable or another routine.
- `update_string_metadata`: Compares two values, copies a sequence of words from a source to a destination, and updates a string or buffer with a length and certain characters.
- `swap_linked_list_nodes`: Checks a pointer against a global value, swaps two adjacent pointers in a linked list, and updates a global pointer to point to the new head or a node.
- `process_conditional_recursion`: Checks a value in a table and recursively calls itself or another routine while clearing a value in a table or setting a value to a constant.
- `update_buffer_and_linked_list`: Updates a buffer or table entry with a sequence of incremented values and modifies a linked list by swapping or updating pointers.
- `copy_data_table`: Copies thirty-two words from a table into a destination buffer pointed to by a parameter.
- `update_buffer_recursively`: Recursively updates a buffer or a table entry with a value from a parameter, or updates a pointer-based destination if a certain condition is met.
- `iterate_and_increment_table`: Iterates through a table using a pointer, decrementing a counter-based loop limit while incrementing the value stored at the current pointer location and calling another routine recursively.
- `update_table_recursively`: Recursively updates a table by incrementing a counter and modifying values through a pointer until a termination condition is met.
- `compare_structure_word`: Compares a word at offset 12 of a structure in a5 to 98, and if it is not less than or equal to 98, moves 130 into d0 and branches.
- `compare_and_branch_constant`: Loads 98 into d0 and 130 into d1, compares them, and branches if d1 is less than or equal to d0; otherwise it moves 130 into d0 and branches to another location.
- `compare_byte_and_branch`: Compares a byte at offset 12 of a5 to 98; if it is less than or equal to 98, it branches to another routine, otherwise it moves the byte into d0 and branches to another location.
- `update_hardware_value`: Sets a 16-bit value at offset -20616 from a4 to 98, compares a byte at offset 14 of a5 to 25, and branches to another routine if the byte is not less than or equal to 25; otherwise it moves the byte from offset 14 of a5 into d0 and branches to another routine.
- `write_constant_to_hardware`: Writes the value 25 to a memory location at offset -20616 from a4 and unlinks a5 before returning.
- `wait_for_equality`: Enters a loop that continues until the contents of two memory locations are equal.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `rotate_bits_in_memory`: Calls a library function, stores the result in a global variable, adds a constant, and then performs a bitwise rotation or shift-based update to that global variable.
- `calculate_bit_shift_limit`: Calculates how many bits a value can be left-shifted before a certain bit is set, then retrieves a value from a table and performs a bitwise AND check.
- `get_value_from_offset`: Returns a value stored at a fixed offset from a pointer.
- `update_bit_rotated_value`: Retrieves a value from a fixed offset relative to a pointer, adds a constant, and performs a bitwise rotation or shift-based update on a global variable or memory location.
- `decrement_and_execute_list`: Decrements two global short values, updates a third global short based on a subtraction, and executes a series of functions via a linked list.
- `update_global_state_3`: Copies a value from one global variable to another and updates several global shorts or bytes with constant values.
- `insert_linked_node`: Inserts a new node into a linked list if the second element of the provided structure is zero.
- `update_linked_list_node`: Traverses a linked list to find a specific node and replaces its next pointer with a new pointer, then clears a value at an offset of that pointer.
- `traverse_and_clear_pointer`: Traverses a linked list or pointer chain until a certain value is found, then clears a value at an offset of a pointer.
- `assign_pointer_and_clear`: Copies a pointer from one location to another and then clears the value at the offset of that pointer.
- `reset_global_pointer`: Calls another routine with specific arguments if a global variable is non-zero, then clears that global variable.
- `update_blt_registers`: Updates certain hardware registers and a global variable using values or bit-shifted data derived from a passed pointer and a table.
- `calculate_bitmask_logic`: Iterates through a loop checking bitmasks and updates a global variable based on bitwise conditions or pointer-based lookups.
- `calculate_bit_mask_loop`: Iterates through a loop checking bitmasks and updates a global variable or calls another routine based on bitwise conditions.
- `search_bitmask_pattern`: Iterates through a loop checking bitmasks against a value and updates a global variable.
- `empty_routine`: the routine is empty and performs no operations.
- `update_blit_parameters_3_precursor`: Prepares a data structure for a blitter-related routine by calculating and writing new values, including bit-shifted offsets and pointer-based calculations.
- `update_blit_parameters_3`: Calculates and writes new values to a data structure, including bit-shifted offsets and pointer-based calculations.
- `update_blit_parameters_2`: Iterates through a data structure to calculate and update values used for Blitter operations.
- `update_blit_parameters`: Iterates through a data structure to calculate and update values used for Blitter operations.
- `update_display_attribute_masks`: Updates a table of bitmask-based flags and pointer offsets, recalculating bitplane-related masks and pointer values based on a change in certain parameters or bitplane configuration.
- `recalculate_blt_attribute_masks_2`: Updates a table of bitmask-based flags and pointer offsets, specifically recalculating bitplane-related masks and pointer values based on a certain offset or size-related parameter.
- `recalculate_blt_attribute_masks_3`: Updates a table of bitmask-based flags and pointer offsets, recalculating bitplane-related masks and pointer values based on a change in certain parameters.
- `update_blt_attribute_masks`: Updates a table of bitmask-based flags and pointer offsets, specifically recalculating bitplane-related masks and pointer values based on a certain offset or bitmask-based flag.
- `update_blt_attributes_and_pointers_2_2_2`: Updates a table of bitmask-based flags and pointer values, specifically modifying bitplane-related masks and pointer offsets or bitmask-base.
- `recalculate_blt_attribute_masks`: Updates a table of bitmask-based flags and pointer offsets, specifically recalculating bitplane-related masks and pointer values based on a source pointer and a bitmask-derived offset.
- `update_blt_attributes_and_pointers_2_2`: Updates a table of bitmask-based flags and pointer values, specifically modifying bitplane-related masks and pointer offsets or bitmask-based flags while iterating through a list of sprite or or bitmask-related attributes.
- `update_blt_attributes_and_pointers_2`: Iterates through a table to update bitmask-based flags and pointer values while modifying certain bitplane and mask-related hardware registers.
- `update_blt_attributes_and_pointers`: Iterates through a table to update pointer values and bitmask-based flags, while modifying certain bitplane and mask-related hardware registers.
- `update_pointer_table_values`: Iterates through a table, updating pointers and clearing or setting certain memory locations based on bitwise checks.
- `update_pointer_table`: Iterates through a table to update pointers and certain memory locations based on bitwise checks and pointer-based lookups.
- `return_empty`: The routine is empty and performs no actions.
- `initialize_input_state`: Sets certain memory or hardware registers to a fixed value and fills a buffer with a constant value using a loop.
- `execute_indirect_call_5`: Performs an indirect call to a routine using a pointer or address stored in a register or memory.
- `call_indirect_function`: Calls a function pointer located at an offset from a global pointer.
- `clear_and_combine_bytes`: Clears certain bytes in a table and returns a value constructed from those bytes and a third byte.
- `clear_and_accumulate_bits`: Iterates through a table, clearing bytes to zero and calculating a bit-shifted value based on table contents.
- `iterate_and_clear_table`: Iterates through a table, clearing bytes and calculating a bit-shifted value based on table contents.
- `clear_table_bytes`: Clears certain bytes in a table and returns a value constructed from those bytes.
- `clear_and_shift_bit_table_2`: Iterates through a table, clearing bytes to zero and updating a bit-shifted value in a secondary or adjacent memory location.
- `clear_and_shift_bit_table`: Iterates through a table, clearing bytes to zero and updating a bit-shifted value in a secondary or adjacent memory location.
- `iterate_linked_list`: Iterates through a linked list of structures, updating a running sum and calling a function pointer with data extracted from each node.
- `conditionally_call_function`: Checks a byte in a global variable and calls a function pointer if the byte is non-zero.
- `initialize_buffer_structure`: Calls a function to get a pointer, then populates a buffer with values from a global variable and a parameter.
- `call_routine_with_offset`: If offset 8 of a5 is non-zero, it calculates a value by shifting and adding to a word from offset 2 of a structure in a0, then calls another routine using a pointer from a5 and the calculated value.
- `process_buffer_sequence`: Calls another routine, and if the result is non-zero, performs a series of calls to another routine with different parameters based on comparisons of values at a negative offset of a5, eventually calls another routine and branches to another location.
- `loop_and_call_routine_2`: Calculates an index from a word at a negative offset of a5, fetches and manipulates bits from a table at a negative offset of a1, stores a result into a table at a negative offset of a4, enters a loop that decrements a counter at a negative offset of a5, calls another routine, and extends the sign of d0 before branching.
- `call_and_extend_result`: Pushes two longwords onto the stack, calls a routine, cleans up the stack, extends the sign of d0, stores the result in a5, and branches to another routine.
- `load_pointer_from_stack`: Pushes a longword from a negative offset of a5 onto the stack, calls another routine, cleans up the stack, moves a longword from a negative offset of a5 into d0, unlinks a5, and returns.
- `combine_bitfields`: Reads three bytes from offsets 9, 11, and 13 of the stack frame, shifts them by 8, 4, and 0 bits respectively, and combines them into a single word in d0.
- `split_and_store_bits`: Extracts three different bit segments from a word at offset 8 of the stack frame using logical shifts and masks, then stores each result into three separate locations pointed to by offsets 10, 14, and 18 of the stack frame.
- `execute_loop_calls`: Iterates through a range defined by a parameter and calls a function using a value constructed from bit-shifted components.
- `update_palette_nibbles_clamped`: Iterates through a source table to extract 4-bit nibbles and compares them against a target buffer, incrementing or decrementing the target nibbles toward the source value.
- `transform_nibble_data_2`: Extracts nibbles from a source, applies clamping logic, and writes the modified nibbles into a destination table.
- `transform_nibble_data`: Iterates through a source table to extract and manipulate 4-bit nibbles, then writes the modified nibbles into a destination table.
- `update_color_palette_nibbles`: Iterates through a source table to extract and manipulate 4-bit nibbles, applying clamping or incrementing logic before writing the modified values into a destination palette or attribute buffer.
- `update_sprite_color_palette`: Iterates through a range of indices to calculate and write new color or attribute values into a buffer.
- `execute_sequence_with_checks`: Calls a routine repeatedly until a certain value is returned, then updates a global variable with a constant value.
- `update_sprite_animation_frame`: Iterates through an animation-related table to update a pointer and a frame counter, while checking a secondary counter or a limit.
- `update_animation_frame_sequence_2`: Updates a sequence of bytes from a table and calculates a new delay or counter value.
- `advance_animation_sequence`: Iterates through a table of bytes to advance an animation or sprite sequence, updating a delay counter and a state-based index or frame pointer.
- `process_animation_sequence_with_delays`: Iterates through a table of bytes to update an animation frame sequence while calculating or updating a delay counter.
- `process_animation_sequence`: Iterates through a table of bytes to update an animation or delay counter, using a switch-based state machine to determine the next delay or frame or to skip certain values.
- `process_animation_sequence_2`: Iterates through a table of bytes to update an animation or delay counter, branching or adjusting timing delays based on the value of the current byte.
- `update_animation_frame_sequence`: Updates a sequence of bytes from a table and calculates a new delay or counter value.
- `update_sprite_animation_state_2`: Updates a sprite animation or sequence controller by checking a byte-encoded instruction or state, adjusting a counter, and updating a pointer or a global state variable.
- `update_sprite_animation_state`: Updates a pointer and a counter while checking a secondary limit, then iterates through a table to update a sprite or animation frame pointer and a frame-duration counter.
- `process_sprite_animation_sequence`: Iterates through a table of bytes to update a pointer and a counter, while checking a secondary counter or a limit.
- `process_entity_state_updates`: Iterates through a list of structures to update internal state values based on specific command bytes.
- `search_and_set_pointer`: Searches for a specific pattern in a memory range and, if found, updates a global pointer with the address of a matching structure.
- `iterate_entity_flags`: Iterates through a list of entity structures and calls another routine if a specific bit is set in a flag field.
- `iterate_entity_state_flags`: Iterates through a list of entity structures, checking a specific bit in a flag field and calling another routine if the bit is set.
- `increment_and_store_pointer`: Increments a global counter and, if it wraps to zero, calls a function to update a pointer, potentially resetting that pointer to a new value from a table.
- `update_table_pointer`: Updates a pointer in a table by adding a bit-shifted register value to an existing value.
- `update_animation_pointer_and_frame`: Updates a pointer in a table via a routine call and, if the new pointer is non-zero, increments a value at an offset from that pointer.
- `update_animation_pointer_and_offset`: Updates a pointer in a table via a routine call and, if the new pointer is non-zero, adds a constant value to the memory location pointed to by that pointer.
- `update_sprite_animation_frames`: Iterates through a table of sprite pointers, updating animation frames and checking bitmasks to determine if a sprite or a block of memory should be updated or cleared.
- `update_animation_state`: Retrieves a value from a global pointer and stores it in a local variable.
- `process_animation_loop`: Checks a value from a global pointer and conditionally sets a local variable.
- `set_animation_pointer_fixed`: Calls a routine with a fixed third argument and stores the returned value into a table offset via a pointer.
- `set_animation_pointer_zero`: Calls a routine with a zeroed third argument and stores the returned value into a table offset via a pointer.
- `update_blt_graphics_parameters`: Updates a pointer-based table with new coordinate or size data and configures the bitblt hardware registers.
- `update_sprite_buffer_and_timer`: Copies a block of memory from a source table to a destination buffer and updates a timing or counter-related value based on a calculated offset.
- `update_animation_pointer`: Updates a pointer in a table by adding a bit-shifted value from a register to an existing value.
- `update_entity_state_recursively`: Iterates through an entity list to check flags and update internal state or pointers.
- `update_clamped_input_vectors`: Splits a 16-bit input value into two 8-bit components and calculates the difference from their previous values.
- `update_clamped_values`: Updates two values in a structure by retrieving data from tables based on an index, applying clamping logic to ensure the results stay within specified minimum and maximum bounds.
- `call_empty_routine`: Calls another routine with a constant value of fifteen.
- `execute_conditional_callbacks`: Checks several global pointers and, if they are non-zero, executes the routines or library functions they point to.
- `execute_recursive_calls`: The routine calls itself repeatedly before returning.
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
- `call_function_with_indirect_values_2`: Calls a routine using values retrieved from memory locations pointed to by global variables, depending on a parameter check.
- `call_function_with_indirect_values`: Calls a function using two values retrieved from memory locations pointed to by global variables.
- `call_function_with_parameter`: Calls a function using a global pointer and a provided parameter.
- `do_nothing_routine`: the routine is empty and performs no operations before returning.
- `update_structure_bounds_3`: Calls a library function to create or modify a structure, then conditionally updates or clears memory ranges or structure members based on provided parameters.
- `update_structure_bounds`: Calculates a new range or size, updates a structure member, and potentially clears or copies data within a memory range.
- `update_structure_bounds_duplicate`: Performs a sequence of memory clearing and copying or updating structure members based on a calculated size.
- `update_structure_bounds_2`: Updates a structure member with a return value from another routine and conditionally clears or copies a memory range.
- `manipulate_structure_members`: Executes a sequence of function calls that clear values, copy data, and update structure members.
- `perform_multiple_function_calls`: Executes a sequence of four function calls that manipulate values stored in a structure, including clearing a value and copying data between locations.
- `set_structure_bounds_and_clear`: Calls a library function to create a structure, clears a memory range, and then calls another routine using a pointer or identifier retrieved from that structure.
- `update_global_with_sum`: Calculates a sum involving a global variable and a value from a library call, then stores a bit-manipulated version of that sum into a global variable.
- `accumulate_global_value`: Performs a complex arithmetic loop that accumulates values from a global variable and other memory locations into a global variable.
- `return_uninitialized_stack_value`: Returns an uninitialized value from the stack.
- `increment_global_with_sum`: Calculates a sum involving a global variable and certain memory locations, then adds a repeated global variable value and that sum to a global variable.
- `initialize_system_state_2`: Resets several global variables to zero or a constant and executes a sequence of indirect function calls and conditional branches.
- `execute_callback_list_callback`: Executes a routine or a sequence of routines retrieved from a table or a global pointer-based list.
- `call_wrapper_routine_2`: Calls another routine and returns.
- `load_and_call_function`: Loads two 32-bit values from a table into global variables and then calls a function pointer.
- `initialize_animation_parameters`: Calls a routine with a specific constant, executes an indirect function call, and then calls another routine with a different constant.
- `initialize_system_state`: Resets several global flags or counters and executes a sequence of library or internal routines before calling another routine with a global variable.
- `reset_global_flags`: Calls a routine with a constant, resets a global word to zero, executes two indirect calls, resets another global byte to one, and calls a final routine.
- `trigger_replay_routine`: Calls another routine using a specific address and a global variable.
- `execute_multiple_initializations`: Calls a routine with a constant, then executes two subsequent indirect routine calls.
- `reset_and_call_routine`: Checks a global flag, executes a sequence of routines, and calls another routine with a specific address and a global variable if certain conditions are met.
- `set_half_state_flag`: Checks a parameter, calls a routine with a constant if non-zero, sets a global byte to zero, executes an indirect call, and calls another routine with a constant.
- `initialize_memory_tables`: Sets certain memory locations to specific constant values using a loop.
- `sum_table_bytes`: Iteratively calls a function to fetch values from a table and accumulates certain bytes into a local variable.
- `update_tables_with_calculated_offsets`: Writes certain values to a table, calculates and stores new pointer-based offsets into a table, and calls a routine with a pointer from a table.
- `initialize_game_state`: Resets several memory-mapped or global arrays to zero and configures a table of parameters based on a specific or previously set value.
- `check_and_set_global_flag`: Checks if certain routine calls return zero, then sets a global flag, calls a routine with two pointers, and resets the flag.
- `execute_sequence_of_function_pointers`: Executes a series of indirect function calls using addresses or offsets derived from a global pointer, and writes a constant value to a memory location.
- `execute_conditional_branch`: Checks a global variable and either executes a sequence of function calls or a single different function call.
- `update_conditional_state`: Calls a routine with a constant, conditionally sets a global byte, and updates a global word or pointer based on the result of an indirect call or a comparison.
- `call_offset_routine`: Calls another routine using an address calculated from a pointer and an offset.
- `recurse_with_offset`: Calls itself recursively using a value calculated by adding a negative offset to a register and a global variable.
- `call_library_with_parameters`: Calls a library function using a pointer and certain values retrieved from a table.
- `update_conditional_table_entry`: Initializes a memory range to a constant value and then conditionally updates a table entry based on a comparison with a library function return value.
- `set_global_short_value`: Waits for a global short value to reach zero and then stores a word from a parameter into that global location.
- `copy_word_to_memory_2`: copies a word from a source address to a destination address.
- `recursively_call_routine`: Checks if a parameter is non-zero and, if so, calls itself with that parameter.
- `return_empty_routine`: The routine performs no operations and immediately returns.
- `update_table_data`: iterates through a loop to copy or update several values in a table based on conditional checks and pointer-based lookups.
- `search_and_update_table`: Iterates through a table to find an entry that satisfies a condition, then updates a table entry with an incremented or decremented value based on a comparison.
- `update_table_with_conditional_increment_2`: Iterates through a table using a pointer, checking a condition and potentially updating a table entry with a calculated value or a pointer-based value.
- `update_table_with_conditional_increment`: Iterates through a table using a pointer, checking a condition against a value returned by another routine.
- `update_table_values_2`: Iteratively updates a table with a negated or incremented value while checking a condition against a return value from another routine.
- `update_table_value_via_pointer`: calculates a new byte value by negating an incremented byte from a source and stores it into a table at an offset determined by a pointer-based index.
- `update_pointer_list_and_values`: Increments a value in a structure, updates a pointer from a table, and conditionally modifies a byte in a table based on a division result and a function return.
- `initialize_array_bounds`: clears a range of memory and then populates a secondary table with index-based offsets.
- `update_array_bounds_2`: Iteratively updates a pointer or a value in a table and conditionally sets a word in a table to zero or one.
- `update_array_bounds`: Iterates through a range, updating a pointer or a value in a table, and conditionally sets a word in a table to zero or one.
- `reset_array_bounds`: Clears several memory locations and then conditionally sets a word in a table to zero or one based on a cleared byte.
- `reset_global_word`: sets a word at a specific offset relative to a pointer to zero.
- `update_index_with_function_results`: iterates through a loop, calling a function via a pointer and storing the result or its negation into a table.
- `load_and_call_pointers`: calls two library functions, clears a table, loads several values from a library-referenced structure, and calls another routine for each loaded pointer.
- `execute_callback_list`: iterates through a table of pointers and calls another routine for any non-zero pointer, then calls two library functions.
- `process_recursive_parameters`: Executes a function call using a value from a pointer and then calls itself recursively with modified parameters.
- `dispatch_indirect_calls`: Calls three different routines using a pointer from a table, passing various combinations of provided parameters.
- `process_recursive_coordinate_table`: This routine recursively traverses a table of coordinate offsets or or instructions, updating or decrementing a value based on a library function return, and potentially executing a fixed number of or a conditional number of recursive calls with incremented or offset-based parameters.
- `update_bounds_and_call_routine`: Updates a routine call with modified or incremented values retrieved from a structure, then calls a routine again with the original or a different value from that structure.
- `loop_parameterized_calls`: Iterates a loop to call a routine with a pointer and calculated offsets, then calls a routine via a pointer using a value from a table.
- `execute_parameterized_calls_2`: Calls a routine with modified or incremented parameters, potentially iterating through a table to perform multiple calls.
- `execute_parameterized_calls`: Calls a routine via a pointer using a value from a table, then calls another routine using four values retrieved from a structure.
- `call_wrapper_routine`: calls a function via a pointer with three parameters, calls a second function via a pointer, and then calls another routine using the first parameter, the fourth parameter, and the result of the second function.
- `search_and_increment_table`: Increments two values in a structure and searches through a table until a null byte is encountered, returning a word from a memory location relative to a pointer.
- `increment_and_search_table`: Increments two values in a structure and searches through a table until a null byte is encountered, returning the final incremented value.
- `read_word_from_offset`: returns a word from a memory location relative to a pointer.
- `call_with_offset_parameters`: Calls a routine via a pointer using a single parameter, then calls another routine using four parameters where two are sums of input values.
- `call_via_offset_table`: Retrieves bytes from a table using an offset and calls a routine twice if a specific byte in the table is non-zero.
- `update_entity_attributes`: Reads attribute offsets and conditional modifiers from a table and calls three different subroutines to update or add values to a target memory location.
- `update_table_data_2`: Extracts multiple values from a table using an index and offsets, then calls three different routines with these values and updated pointers.
- `call_subroutine_via_pointer`: Calls a routine through a pointer stored in a global table using a passed parameter.
- `execute_indirect_call`: Fetches a function pointer from a table using an offset from a base address and calls it with a parameter.
- `execute_indirect_function_call`: Fetches a function pointer from a memory location and calls it using a value passed as an argument.
- `clear_memory_buffer`: Uses nested loops to set a range of memory or a multi-dimensional array or buffer to zero.
- `update_table_values`: Calculates a base address using a table index and offset, then adds a value to three separate memory locations within a table or structure.
- `add_to_table_value`: Calculates an address in a table using an index and an offset, then adds a value from a register to the memory at that address.
- `execute_state_callbacks`: Executes a sequence of indirect function calls and updates a global variable with a byte value.
- `dispatch_state_callbacks`: Updates a global variable, executes a sequence of indirect calls with certain parameters, and conditionally calls another routine.
- `update_state_and_dispatch_callback`: Updates a global variable with a return value from a routine, modifies a flag in a data structure, and executes a sequence of indirect function calls using parameters or return values retrieved from a pointer-based data structure.
- `execute_concatenated_calls`: Calls a routine, stores its return value in a global variable, and calls another routine using a concatenated 32-bit value.
- `store_and_call_concatenated`: Calls a routine, stores a returned value in a global variable, and calls another routine using a concatenated 32-bit value.
- `update_state_and_dispatch_calls`: Clears a word in a global structure and conditionally updates a byte if a certain value is detected.
- `dispatch_state_callbacks_2`: Updates a global word and executes a sequence of indirect function calls with various parameters, potentially updating a global variable with a returned value.
- `update_table_indices`: Performs complex updates to a table or array, involving conditional logic, index-based lookups, and writing new values to a table.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a2).
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a7).
- `recalculate_attribute_table`: Iterates through a table of structures to recalculate or copy attribute values, such as a scaling or ratio-based calculation, based on a value.
- `update_attribute_table`: Iterates through a table of structures to recalculate or copy attribute values, such as a scaling or ratio-based calculation, based on values stored in a secondary data table.
- `initialize_global_state`: Calls a routine with two arguments, stores the return value in a global variable, and then clears a small array of four or more words to zero.
- `call_routine_with_constant`: Calls a routine using a value stored in a global variable and a constant integer as an argument.
- `enable_dma_channel`: An 18-instruction routine that modifies the DMA controller register to enable a DMA channel.
- `update_dma_config`: Writes a value from a memory location to a hardware register and sets another hardware register to a constant value.
- `update_coprocessor_hardware`: Writes a longword and a word into hardware registers using an offset calculated from a provided parameter.
- `enable_dma_channels`: Sets a global variable to a value that enables DMA channels via the DMACON register.
- `write_hardware_registers`: Writes two or-sized values into a hardware register or a memory-mapped area using an offset derived from a parameter.
- `process_event_table_and_recurse`: Checks a table for a non-zero value, then uses a switch-case to update a global state or flag and a secondary pointer or value before recursively calling itself with a new parameter.
- `update_state_table_and_recurse`: Updates a global state or flag and a secondary pointer or value based on a lookup or switch-case, then calls another routine using a pointer retrieved from a table.
- `update_table_entry_2`: Writes several constant values to a global memory location and calls another routine with a value loaded from a pointer.
- `update_local_values`: Adds a longword from offset -8 of a5 to a longword at offset -4 of a5, writes several constants to the stack and to memory relative to a4, and branches to another location.
- `update_table_values_3`: Writes a 16-bit value of 1 to an offset of -14 from a5, checks a 16-bit value at offset -13594 from a4, and if not equal to zero, writes 1 to offset -10 from a5 and 120 to offset -16 from a5 before branching to another routine.
- `update_table_entry`: Calculates an offset using a value from a table and writes a constant to that memory location, then calls another routine with a value loaded from memory.
- `return_no_op`: The routine is a single return instruction with no body, so it performs no work.
- `branch_on_input_value`: Checks an input register against several cases to perform conditional writes to a global memory location or modify a pointer before calling another routine.
- `return_no_op_stub`: The routine consists of a single return instruction and performs no work.
- `clear_and_set_pointers`: Clears a range of memory, calls a library function, and stores return values from other routines into a global variable.
- `update_lookup_table_values`: Iterates through a range of indices to update a destination memory area using a value calculated from a secondary lookup table and a primary pointer table.
- `update_table_from_lookup`: Iterates through a table to update a destination memory area using values fetched from a secondary lookup table.
- `call_function_conditionally`: Checks a global variable and calls a function pointer with a specific value if that variable is non-zero, then calls the same function pointer with a different constant.
- `decrement_active_counters`: Decrements a global counter and, if it reaches zero, conditionally decrements or resets a secondary set of counters or pointers based on a secondary check.
- `update_pointer_and_call`: Calls a function pointer, calls another function pointer with zero, and copies a value from a memory location to a global variable.
- `reset_function_and_copy`: Calls a function pointer with zero and copies a value from a memory location to a global variable.
- `reset_state_and_call_subroutines`: Clears a value in a global memory location and then executes two previously stored function pointers, passing a zero or no argument to the first.
- `execute_function_pointer`: Calls a function through a pointer stored at a specific offset from a base address.
- `configure_dma_and_clear_flag`: Clears a global flag and calls a routine to enable a specific number of DMA channels, followed by a routine that writes a value to a memory location.
- `test_global_pointer`: Tests a longword at a negative offset from a pointer in a4 and branches to another routine if the value is non-zero; otherwise it returns.
- `call_subroutine_with_parameter`: Calls another routine twice using a value loaded from a memory location.
- `decompose_and_render_value`: Decomposes a numeric parameter into individual digits stored in a local array, then iterates through that array to call another routine using values retrieved from a table.
- `process_list_and_update_table`: Iterates through a list, updating a table with a value or a constant, and calls another routine with arguments from a table.
- `process_linked_list_elements`: Iterates backward through a list of pointers, updating a status flag in a lookup table and resetting a counter, then calls another routine for any remaining active elements in the list.
- `iterate_and_call_on_condition`: Iterates through a list and calls a function pointer with a value from a memory location if a certain condition in a lookup table is met.
- `get_absolute_value`: Returns the absolute value of a passed parameter.
- `return_value_from_pointer`: Returns a word stored at an offset from a pointer.
- `copy_byte_sequence_2`: Copies a sequence of bytes from one memory area to another using a loop and returns the number of bytes copied.
- `copy_byte_sequence`: Copies a sequence of bytes from one memory area to another using a loop and returns the final index.
- `traverse_and_set_byte`: Traverses a linked list, calls another routine, and conditionally writes a byte or a constant to a global variable.
- `traverse_linked_list`: Iterates through a linked list by following pointers, updating a global pointer with the next node and checking a bitmask in a byte or a zero-check for termination.
- `return_empty_2`: The routine is empty and performs no operations.
- `update_position_and_clear_byte`: Calculates a new coordinate using a pointer-based offset and calls a routine with the result, then potentially clears a byte in a buffer if a certain condition is met.
- `decompose_and_render_value_2`: Decomposes a numeric parameter into individual digits stored in a local array, then iterates through that array to call another routine using a certain value.
- `copy_string_and_get_string_id`: Copies a null-terminated or colon-terminated string into a global buffer and then retrieves a corresponding identifier or pointer.
- `traverse_pointer_list_and_execute_2`: Iterates through a list of pointers, calling a routine with arguments and updating a global variable with a value from a table.
- `return_constant_value`: Calls another routine and returns a constant integer.
- `execute_function_loop`: Iterates through a list of pointers, calling a routine with arguments from a table and updating a global variable with the return value.
- `process_pointer_list_2`: Iterates through a list of pointers, calling another routine with arguments from a table and updating a global variable.
- `traverse_pointer_list_and_execute`: Iterates through a list of pointers, calling a routine with arguments and updating a global variable with the return value.
- `process_pointer_list`: Iterates through a list of pointers, calling another routine with arguments from a table and updating a global variable with a value from that table.
- `return_void`: The routine is empty and returns without performing any operations or returning a value.
- `handle_state_and_library_call`: Checks a parameter, calls a routine, and either calls a library function or executes a loop that calls other routines and updates a global variable.
- `handle_disk_swap_and_state_transition`: Checks a flag to determine if a disk swap or state change is required, then updates a global state variable and executes a sequence of conditional routine calls or an infinite loop.
- `execute_indirect_calls`: Performs four consecutive calls to an indirect function, passing a pointer from d0 or a memory location relative to a4 on the stack, while pushing specific 16-bit constants onto the stack before each call, and finally branches to another routine.
- `handle_file_error`: Pushes a 16-bit value onto the stack, calls an indirect function with a pointer relative to the program counter, compares a 16-bit value at offset -13458 from a4 to 5, and if not equal branches to another routine; otherwise it pushes a 16-bit value onto the stack, calls an indirect function with a pointer relative to the program counter, and branches to another routine.
- `execute_indirect_function_22`: Pushes a 16-bit value onto the stack, calls an indirect function using a pointer in d0, and then performs a jump to a location calculated from a table using d0.
- `write_table_values`: Iterates up to five times to write values from a table into a destination via a routine call, then resets a global variable if certain conditions are met.
- `execute_indirect_function_10`: Loads a function pointer from a memory location and calls it using a constant argument.
- `get_and_return_value`: Calls a routine and returns a value stored in a global variable.
- `initialize_game_state_constants`: Checks a global flag and, if non-zero, populates a series of global or table-based word-sized or byte-sized constants, including specific numeric values and a value passed via a parameter, before calling another routine.
- `initialize_global_variables`: Sets several global variables or table entries to specific constant values or copies a value from a pointer-based source.
- `decrement_and_clamp_counters`: Decrements a global counter and, if it reaches zero, resets other values and clamps a range of counters or pointers to certain bounds.
- `conditional_execute_routines`: Checks several global conditions and, if satisfied, executes two different routines based on a returned value and a counter check.
- `reset_game_state_parameters`: resets or modifies a set of global state variables and table-based parameters based on a provided input value.
- `update_global_state`: Sets a global word-sized or byte-sized value to one and another value to a constant via a pointer-based offset.
- `update_global_state_2`: Sets a global word-sized value to one and another or the same memory location to a constant value.
- `update_struct_members`: Calculates a new value based on a global short divided by two and then updates several members of a structure with fixed or table-derived values.
- `update_struct_member_values`: Checks if a global short value exceeds two, then updates several members of a structure or object using fixed values and values fetched from a table using an index-based offset.
- `set_global_parameters`: Sets a global word to a maximum value and updates other global word-sized values to either a constant or a value fetched from a table.
- `update_global_parameters`: Sets two global word-sized values to one and updates two other global word-sized values to a constant value using a pointer-based offset.
- `update_game_state_parameters`: Updates or resets a set of global state variables and structure members based on the value of a register, which may include updating values from a table or setting certain members to constant or calculated values.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `fill_buffer_with_count`: Iterates a specified number of times, writing the current loop counter value as a byte to a destination buffer and decrementing the counter.
- `populate_buffer_from_nested_loop`: Uses a nested loop to call another routine and store the returned byte into a buffer, continuing until a limit or a specific return value is encountered.
- `iterate_and_store_results`: Iterates through a list of pointers, calling another routine for each and storing the returned byte into a destination buffer.
- `iterate_and_store_results_2`: Calls another routine and stores the returned byte into a buffer, while potentially incrementing a counter and checking a limit.
- `iterate_and_store_results_with_exit`: Iterates through a list of pointers, calling another routine for each and storing the returned byte into a destination buffer, or exits if a certain value is returned.
- `update_pointer_flags`: Saves registers, loads a pointer from offset 8 of a5 into a2, clears bit 3 of offset 12 of a2, tests bit 2 of offset 12 of a2, and if bit 2 is set, calls another routine with a stack frame and then returns; otherwise it branches to another location.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `process_pointer_and_call`: Compares a word at offset 16 of a structure in a5 to 1, and if it is not 1, returns.
- `clear_d0_and_branch`: Clears d0 and branches to another routine.
- `update_pointer_from_call`: Pushes a2 onto the stack, loads a pointer from offset 8 of a5 into a2, pushes a word and a longword onto the stack, extracts a signed byte from offset 13 of a2 into d0, calls another routine, stores d0 at offset -4 of a5, and if bit 2 of offset 12 of a2 is set, calculates the difference between a pointer at offset 0 of a2 and a pointer at offset 8 of a2 and adds it to offset -4 of a5.
- `calculate_pointer_difference`: Checks if a2 is null, otherwise it moves a pointer from offset 4 of a2 to a0, subtracts a2 from a0, stores the result in d0, subtracts d0 from a 16-bit value at offset -4 from a5, and stores the result back into offset -4 from a5 before returning.
- `fetch_and_increment_pointer`: Checks if a pointer or a range limit has been reached, increments a pointer in a structure, and returns the byte previously stored at that pointer.
- `get_and_increment_pointer`: loads a value from a pointer stored in a global variable, increments that global variable, and returns the value previously stored at the pointer's location.
- `check_bitmask_in_structure`: Loads a pointer from an offset of 8 in a structure in a5, reads a byte from offset 12 of that pointer, and branches to another routine if bits 3 and 4 are not set; otherwise it returns.
- `process_bitmask_and_table`: Clears a bit in a byte, then conditionally iterates through a table to call another routine.
- `set_bit_in_byte`: sets the fifth bit of a byte located at an offset from a pointer-based address.
- `update_pointer_and_increment`: Updates a memory location with a new address calculated by adding a value to an existing entry, then increments that memory location.
- `validate_and_return_pointer`: Calls another routine, moves the result to a2, and if d0 is non-zero, branches to another location; otherwise it clears d0 and returns.
- `push_and_branch`: Pushes two values from a5 onto the stack, calls another routine, adjusts the stack pointer, and branches to another location.
- `check_byte_and_set_value`: Calls another routine, loads a pointer from a4 into a2, and if the byte at a2 is zero, stores the value 5 at a specific offset of a4 and returns 0; otherwise it branches to another location.
- `loop_and_compare_bytes`: Sets a0 to a2 and a1 to an offset of a5, then compares a byte from a0 to a byte from a1.
- `execute_conditional_loop`: Pushes a word from an offset of a2 and a pointer from an offset of a5 onto the stack, calls another routine, and then compares the result to a constant.
- `write_bytes_to_structure`: Loads a pointer from offset 10 of a5 into a0, stores the byte from d4 into offset of a0, reloads the pointer from offset 10 of a5 into a0, stores the byte 1 into offset 12 of a0, loads the pointer from offset 10 of a5 into d0, and branches to another routine.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `find_char_in_string`: Iterates through a string looking for a specific character and returns a pointer to that character if found, or returns null if the string or a null terminator is encountered.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `save_registers_and_tail_call`: Preserves registers d2, d3, and a4 on the stack beneath the return address before unconditionally jumping to another subroutine.
- `execute_indirect_call_2`: Performs an indirect jump or call to a location stored in a register or memory.
- `execute_and_calculate_offset`: Calls a routine using a value from a structure in a5, multiplies that value by 6, adds it to an offset from a4, and if certain conditions regarding the value and a global variable are met, writes 2 to a location in a4 and returns -1 in d0; otherwise it returns.
- `handle_io_result`: Pushes three values onto the stack, calls a routine, and if the result in d0 is not negative one, calls another routine and stores the result in a4 at a negative offset.
- `call_seek_wrapper`: Clears two 32-bit values on the stack, pushes the 32-bit value from a2 onto the stack, calls another routine, restores the stack pointer, and branches to another routine.
- `call_with_stack_setup`: Links a5, pushes a word from an offset of a5, pushes the constant 771, pushes a pointer from an offset of a5, calls another routine, cleans the stack, unlinks a5, and returns.
- `execute_and_load_a3`: Links a5, saves d4-d7 and a2-a3 onto the stack, loads a pointer from a positive offset of a5 into a2, calls another routine, loads a longword from a negative offset of a4 into a3, clears d4, and branches.
- `increment_and_compare_loop`: Multiplies d0 by 6, tests a longword at an offset in a3, and if the test fails, enters a loop that increments d4 and compares it against a value at a negative offset of a4 until the condition is met, then moves 6 into d5 and branches.
- `execute_io_sequence`: Tests bit 1 of offset 12 of a5 and branches if it is zero; otherwise it performs a series of calls involving a pointer in a2, a value in d6, and a call to a routine at an offset from a4, eventually branching to another location.
- `execute_indirect_calls_3`: Pushes a longword onto the stack, calls a routine at an offset from the program counter, moves the result to d6, and if d0 is not zero, branches to another routine; otherwise it calls another routine and branches to another routine.
- `execute_indirect_calls_2`: Pushes two pointers onto the stack, calls a routine at an offset from the program counter, moves the result to d7, and if d7 is not zero, pushes it onto the stack and calls another routine using a pointer from a4.
- `push_and_call_routines`: Pushes two pointers onto the stack, calls a routine using a pointer from the stack, pushes a pointer and a zeroed longword onto the stack, calls another routine using a pointer from the stack, and then branches to another location.
- `close_file_if_flag_set`: Checks if bit 10 and 11 are set in a 16-bit word at offset 12 of a5; if they are, it pushes d6 onto the stack, calls another routine, sets d5 to 4, stores d5 at offset -13328 from a4, sets d0 to -1, restores registers from the stack, unlinks a5, and returns.
- `calculate_and_call_subroutine`: Multiplies a word in d4 by 6, uses the result to calculate an offset into a table pointed to by a3 to store a longword from d6, then checks bit 3 of a 16-bit word at offset 12 of a5 and calls another routine if the bit is set.
- `recurse_on_bitmask`: Checks a bitmask in a pointer-based value and either stores a constant or recursively calls another routine.
- `check_pointer_and_recurse`: Checks if a certain bitmask or flag in a pointer-based value is set, then either stores a constant or recursively calls another routine and stores its result in a global variable.
- `execute_io_with_fallback`: Calls a routine with certain parameters and, if the result is negative, calls another routine and stores its result in a global variable.
- `nop_padding_sequence`: this routine is empty and contains only no-operation instructions used as padding or a gap in the executable.
- `rebuild_active_object_list`: Iterates through a table of objects, and for every non-zero entry, updates a secondary list by clearing certain fields and recalculating a pointer-based offset or address.
- `update_table_entries`: Iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `manage_library_resources`: Interacts with library structures to check status, execute functions, and update a pointer in a table.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than thesource.
- `clear_memory_and_call_function`: Clears a block of memory using a loop and then calls a function pointer retrieved from a global variable.
- `return_empty_routine_2`: The routine is empty and performs no operations before returning.
- `parse_string_tokens`: Parses a source string or buffer, handling quoted or whitespace-separated tokens, and stores pointers to these tokens into a destination array.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_to_buffer_2`: Calculates the length of a source string and copies its contents to a destination buffer, ensuring a null terminator is present.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `traverse_and_set_pointers`: Recursively traverses a pointer-based structure and, if certain conditions are met, updates a structure member with a value or a function pointer fetched from a nested lookup.
- `traverse_and_update_pointers`: Recursively traverses a pointer-based structure, updating a global pointer or a table entry with a value or a function pointer, and finally clears a global pointer.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 3deleted value.
- `apply_sign_to_result`: Calls a routine and negates the returned result if a certain input register is negative.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `transform_words_with_math`: Performs complex bitwise shifts, swaps, and arithmetic operations involving a carry-propagating loop or division/modulo operations.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `update_pointer_and_flags_5`: Increments a pointer while storing a byte from a source address and checks a bit in a structure to conditionally update a pointer and a status flag.
- `increment_and_store_byte`: Increments a pointer and stores a byte from a source address into the memory location pointed to by that pointer.
- `compare_and_call_pointer`: Links a5, pushes a2 onto the stack, loads a longword into a2 from a negative offset of a4, sets a0 to that value, increments a2 by 16, pushes a0 onto the stack, calls another routine, cleans up the stack, compares a2 with a longword from a negative offset of a4, and if a2 is not lower, pops a2 from the stack before unlinking a5 and returning.
- `load_and_return_pointer`: Saves registers, loads a pointer from offset 8 of a5 into a2, and if a2 is not zero, moves its value to d0; otherwise it sets d0 to 255 and returns.
- `process_object_flags_and_callbacks`: Checks status bits in a structure pointed to by a2, executes conditional subroutines based on those bits, and clears the structure's memory and status byte.
- `update_pointer_and_flags_4`: Checks a bit in a structure, and if certain conditions or pointer values meet a certain criteria, it updates a pointer, modifies a status byte, and writes a value to a memory location.
- `update_pointer_and_flags_3`: Updates a pointer and certain flags within a structure while potentially clearing or setting bits in a status byte.
- `update_pointer_and_flags_2`: Modifies a pointer and certain flags within a structure.
- `update_pointer_and_flags`: Modifies a pointer and a flag in a structure, and may update a pointer or a byte in a structure or call another routine based on a condition.
- `update_pointer_and_set_flag`: Updates a pointer, modifies a value at an offset, sets a bit in a byte, and increments a pointer while storing a byte.
- `validate_pointer_range`: Checks if a byte at offset 12 of a structure pointed to by a2 is non-zero, and if so, verifies if a pointer incremented by 22 is within a specific range before returning zero.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `clear_structure_fields`: Clears the 32-bit values at offset 0, offset 4, and offset 8 of the structure pointed to by a2, moves the pointer from a2 into d0, and branches to another routine.
- `update_table_and_flag_2`: Updates a structure field with a constant or a pointer, and conditionally modifies a byte in a structure using a bitwise or operation.
- `update_table_and_flag`: Writes a constant to a memory location, modifies a byte via a bitwise or, and copies a dword from a pointer-based offset before conditionally setting a bit in a byte.
- `link_and_restore_registers`: Links a5, pushes registers a2 and a3 onto the stack, loads a pointer from offset -20114 from a4 into a2, and branches to another routine.
- `loop_and_call_routine`: Stores a pointer from a2 in a3, increments a 32-bit value from offset 4 of a2 by 8, pushes the new value and the original pointer onto the stack, calls a routine using a pointer from a4, and loops if the value in d0 is not zero until it clears a 32-bit value at offset -20114 from a4.
- `insert_into_linked_list`: Sets a global pointer to a specific address and, if a routine returns a valid pointer, inserts a value and a pointer into a linked list structure.
- `set_pointer_to_table_entry`: Sets a pointer to a location in a table and stores a value from another memory location into that table entry.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `modify_a2_pointer`: Saves a2 and a3 to the stack, subtracts a3 from itself, loads a pointer from an offset relative to a4 into a2, and branches.
- `compare_and_move_pointer`: Loads a pointer from a structure in a5, compares it to a2, and if they are equal, branches; otherwise it moves a pointer from a2 into a3, moves the value pointed to by a2 into a2, and if the new a2 is not zero, branches back to this routine; otherwise it moves 255 into d0 and returns.
- `move_and_branch`: Moves a longword from the location pointed to by a2 into the location pointed to by a3, then branches to another location if a3 is not equal to d0.
- `push_and_call_pointer`: Copies a longword from an offset of a structure in a2 to a location relative to a4, pushes the value from offset 4 of a2 and the value of a2 onto the stack, calls another routine, and then branches to another location.
- `validate_table_index`: Checks if a provided index is within a certain range and if a value in a table is non-zero, then calls another routine and returns a boolean result.
- `check_table_condition`: Calculates an offset into a table and calls a routine using a pointer from that table, returning whether the result is non-zero.
- `return_zero_2`: An empty routine that returns zero.
- `handle_io_error`: Pushes a value from the stack, calls another routine, and if d0 is not zero, calls a second routine and stores d0 at an offset relative to a4 before returning.
- `clear_and_branch`: Clears register d0 and branches to another location.
- `test_and_store_result_3`: Checks a bitmask in a pointer-based value and either stores a constant or calls another routine, storing the return value in a global variable.
- `test_and_store_result_2`: Checks a bitmask in a pointer-based value and either stores a constant or calls another routine, storing the return value in a global variable if a certain condition is met.
- `test_and_store_result`: checks the return value of another routine and, if it indicates an error, stores the result of a subsequent routine call into a global variable.
- `empty_routine_11`: This routine is empty and performs no operations before returning.
- `execute_conditional_call`: Calls another routine and, if a specific bit is set in the return value, checks a global variable and potentially calls another routine via an indirect pointer.
- `execute_and_return_zero`: Calls another routine and then returns zero.
- `execute_write_sequence`: Calls a function via a pointer to process a global variable and then executes two subsequent routine calls.
- `execute_indirect_functions`: Checks a global pointer and, if non-zero, executes the routine pointed to by that address, followed by another indirect function call.
- `validate_and_clear_index`: Checks if an index is within a valid range and if a corresponding pointer in a table is non-null, then either calls another routine or updates a status variable.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `call_library_vector_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 6 of that base, returning its result to the caller.
- `call_library_vector_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 12 of that base, returning its result to the caller.
- `call_indirect_from_table`: Loads a function pointer from an offset within a table and executes it.
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
- `call_lock_2`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_address`: Executes a routine located at an offset from a global pointer.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_indirect_from_table_offset`: Loads a function pointer from a table and executes it.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_seek_2`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_un_lock_2`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_pointer_2`: Loads a function pointer from a nested pointer or table and executes it.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `execute_indirect_function_11`: loads a function pointer from a memory location calculated using an offset and executes it.
- `call_indirect_via_library`: Loads a function pointer from an offset within a library-related table and executes it.
- `execute_indirect_call_3`: Performs an indirect call to a location calculated by adding an offset to a global variable.
- `execute_indirect_call_with_offset`: Performs an indirect call to a location calculated by adding an offset to a value loaded from a global variable.
- `execute_indirect_subroutine`: calculates an offset from a register and executes the subroutine located at the resulting address.
- `execute_indirect_function_pointer_3`: Loads a function pointer from a nested table lookup and executes it.
- `execute_indirect_function_pointer_4`: Loads a function pointer from a nested offset within a table and executes it.
- `execute_indirect_function_12`: calculates an address by adding an offset to a value in a register and executes the code located at that pointer.
- `execute_indirect_function_pointer_5`: Loads a function pointer from a table located via an offset from a base address and executes it.
- `execute_indirect_function_13`: calculates an offset from a register and executes the function located at the resulting pointer.
- `execute_indirect_function_pointer_6`: loads a function pointer from a nested table or structure and executes it.
- `execute_indirect_lookup`: Loads a function pointer from a nested table lookup and executes it.
- `execute_indirect_function_14`: loads a function pointer from a nested table lookup and executes it.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `execute_indirect_call_4`: Performs an indirect call to a function pointer located at an offset from a global pointer.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `execute_indirect_call_offset`: Loads a pointer from a global variable and calls a function at a specific offset from that pointer.
- `execute_indirect_call_offset_2`: Loads a pointer from a global variable and calls a function at a different offset from that pointer.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_function_15`: Loads a function pointer from a table located via an offset from a register and executes it.
- `execute_indirect_function_16`: loads a function pointer from a table located at a fixed offset from a base address and executes it.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_function_17`: loads a function pointer from a table using an offset from a global pointer and executes it.
- `call_library_vector_486`: Library stub: loads a library base from a global variable and calls the library function at offset minus 486 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_7`: loads a function pointer from a nested table or structure and executes it.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_8`: Loads a function pointer from a table located at a fixed offset from a base address and executes it.
- `execute_indirect_function_pointer_9`: loads a function pointer from a nested table offset and executes it.
- `execute_indirect_function_pointer_10`: loads a function pointer from a nested table offset and executes it.
- `call_indirect_from_table_2`: Loads a function pointer from a table using an offset from a global variable and calls it.
- `call_indirect_from_table_3`: Performs an indirect call to an address retrieved from a table using a different offset.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `execute_indirect_function_18`: loads a function pointer from a table located at a fixed offset from a global pointer and executes it.
- `execute_indirect_function_19`: loads a function pointer from a table using an offset from a global variable and executes it.
- `execute_indirect_function_20`: Loads a function pointer from a table located at a fixed offset from a global address and executes it.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `execute_indirect_function_21`: loads a function pointer from a table using an offset from a global variable and calls it.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2)[b].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_38`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_39`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_40`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_41`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_42`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_43`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_46`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_47`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_48`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_49`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_50`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_51`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_52`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_53`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_54`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_55`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_56`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_57`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_58`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_59`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_60`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_61`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_62`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_63`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_64`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_65`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_66`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_67`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_68`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_69`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_70`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_71`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_72`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_73`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_74`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_75`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_76`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_77`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_78`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_79`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_80`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_81`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_82`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_83`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_84`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_85`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_86`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_copper_and_clear_memory`: Opens a library, copies a table of words into memory, clears a large memory region, waits for a hardware flag to clear, and closes the library.
- `service_pending_interrupt_flags`: Tests an interrupt flag bit and, when it is set, checks a second interrupt request bit and calls up to three helper routines.
- `write_next_table_entry`: Reads a byte from a pointer, wraps the pointer to a base address if the value is negative, stores a word built from that byte and a constant into a table, and advances the pointer by four.
- `build_frame_copper_list`: Computes per-frame display geometry from state fields and fills a copper-style list of word pairs in memory.
- `update_hardware_copy_block`: Waits for a hardware busy bit to clear, then initializes hardware registers with control words and address pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_blocks_2`: Calls a routine that initializes memory blocks and clears a series of byte-sized memory locations.
- `update_sprite_attribute_table_2`: Iterates through a range of memory to update or duplicate sprite or object attribute data, including updating or clearing associated word-sized or byte-sized memory locations.
- `update_sprite_attribute_table`: iterates through a range of memory to update or duplicate sprite or object attribute data, including updating or clearing associated word-sized or byte-sized metadata or or secondary attribute buffers.
- `initialize_memory_blocks`: Sets several global variables or memory locations to specific values and clears a series of byte-sized memory locations using a loop.
- `clear_memory_blocks`: Clears or zeros out a series of word-sized memory locations using a loop.
- `update_memory_tables`: Copies word-sized values from a source table to a destination table, using an offset or a value from a global variable to determine the source address.
- `fill_memory_loop`: Calculates an address based on values at offset -2 and -4 from a5 and an offset from a4, then enters a loop that writes a byte from offset 254 of that address to a new location, updates a counter at offset -4 from a5, and clears a 16-bit value at offset -27668 from a4 until a counter reaches 12.
- `initialize_string_buffer`: Allocates a memory buffer and initializes it with a specific string or header, then updates several global state pointers and flags.
- `copy_register_to_register`: Copies the longword from d0 to d5.
- `process_callback_list`: Iterates through a list of function pointers and executes them, followed by a routine that passes a calculated size or pointer to another function.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_empty_routine_2`: the routine performs no operations and returns immediately.
- `update_global_pointers`: Executes a function pointer and then copies values from a table or structure into several global variables.
- `call_two_routines`: Pushes the value 5 onto the stack, calls a routine using an offset from a4, pushes 5 onto the stack again, and calls another routine using a different offset from a4 before returning.
- `decrement_offset_word`: Tests a word at a negative offset from a4 and, if it is not zero, adds 10 to it; otherwise it returns immediately.
- `decrement_and_call`: Tests a word at a negative offset from a4, and if it is not zero, decrements it; if it is zero, it pushes one onto the stack, calls a routine, and branches to another routine.
- `calculate_and_update_offset`: Checks three words at negative offsets from a4; if they meet specific conditions, it calculates a value by adding a word at offset -19452 and a word at offset -19436 from a4, scales it, and uses it to fetch a word from a table.
- `update_status_values`: Compares two 16-bit words at offsets relative to a4 and, based on their values and a third 16-bit word, writes specific 16-bit constants to an offset relative to a4.
- `decrement_and_check_bounds`: Decrements a 16-bit word at an offset relative to a4, increments another 16-bit word at a different offset, and writes a byte to a third offset if the first word reaches a specific value.
- `update_status_byte`: Decrements a 16-bit value at an offset relative to a4, compares it to 1, and if it is not 1, writes 2 to a byte at an offset relative to a4, increments a 16-bit value at another offset relative to a4, clears d0, and branches to another location.
- `clear_and_branch_2`: Sets up a stack frame, clears a byte at a negative offset from a4, sets a byte at another negative offset from a4 to 255, and branches to another location after loading a word from a5 into d0 and extending it to a longword.
- `set_control_values`: Writes 254 to a byte at an offset relative to a4, and writes 1 to two 16-bit words at offsets relative to a4, then branches to another location.
- `negate_and_store_byte`: Clears d0, loads a byte from an offset relative to a4 into d0, increments d0, negates it, and writes the result to a byte at an offset relative to a4 before branching to another location.
- `decrement_and_call_routine`: Decrements d0 three times, branching to different locations if d0 reaches zero, and calls another routine if d0 is not zero before writing a word to a location relative to a4.
- `load_and_branch`: Sets up a stack frame, loads a word from a structure in a5 into d0, and branches to another location.
- `store_and_branch`: Stores a longword and a word into negative offsets of a structure in a5 and branches to another location.
- `store_constants_to_a5`: Stores a longword and a word to offsets relative to a5 and branches to another location.
- `store_different_constants_to_a5`: Stores a longword and a word to offsets relative to a5 and branches to another location.
- `dispatch_via_table`: Decrements d0 three times, and if d0 is not zero, pushes several values from a5 onto the stack, calculates an offset using a value from a5, and calls another routine using a pointer from a table.
- `execute_conditional_logic_2`: Initializes local variables on the stack, performs a series of operations involving a0 and d0 to access a table at a negative offset of a4, and if a counter reaches 8, clears two words at a negative offset of a4, calls three different routines with various stack setups, and performs conditional branches based on values at negative offsets of a4.
- `call_with_parameters_2`: Compares a word at offset 8 of a5 to 20 and branches if it is less; otherwise it pushes three values onto the stack, calls another routine, pushes three more values, calls another routine, clears a word at a negative offset from a4, and branches to another location.
- `call_with_stack_parameters_2`: Pushes a word from offset 8 of a5, the value 14, and zero onto the stack, calls a routine, then pushes the values 9 and 8 and zero onto the stack, calls another routine, and finally branches to another routine using a value from offset 8 of a5 in d0.
- `branch_to_address`: Writes the value 10 to offset -4b34 from a4 and branches to a location in a6.
- `clear_and_call_routines`: Tests a word at a negative offset from a4 and, if not zero, performs several calls with stack parameters, clears several words at negative offsets from a4, and branches to another routine.
- `execute_stack_setup_and_call`: Pushes two words onto the stack, calls a routine, cleans up the stack, pushes a word, calls another routine, cleans up the stack, and branches to another routine.
- `setup_parameters_and_clear_memory`: Pushes two words onto the stack, calls a routine, pushes three words onto the stack, calls another routine, writes a word to an offset relative to a4, and branches to another routine.
- `jump_via_table`: Compares d0 to 26, and if d0 is less than 26, shifts d0 left by 1, loads a word from a table using d0 as an offset, and jumps to the address in d0.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `call_and_move_word`: Sets up a stack frame, pushes 1 onto the stack, calls another routine, increments the stack pointer by 2, moves a word from an offset of a5 to an offset of 2 less than a5, and branches to another location.
- `process_table_entries`: Loads values from a table using a 16-bit value from offset -2 of a5, pushes several values onto the stack, calls a routine using an offset from a4, and loops if the value at offset 10 from a5 is less than the value at offset -2 from a5.
- `update_offset_value`: Sets up a stack frame, moves a word from offset 8 of a5 to an offset from a4, and if a byte at offset 10 of a5 is non-zero, branches to another routine; otherwise it adds 30 to a value from offset 8 of a5, stores it at offset 10 of a5, and branches to another routine.
- `loop_through_table_entries`: Calculates an offset by scaling a word from offset -2 of a5 and adding it to a base address in a4, then performs several operations including calling multiple routines via the stack, updating values in a table, and decrementing a word at offset -2 of a5 in a loop that continues until the value is no longer less than 10.
- `check_and_return`: Pushes a5 to the stack, checks a word at offset 12 of a5, and if it is not zero, branches; otherwise it clears d0 and returns.
- `move_and_branch_2`: Moves a word from an offset of a5 to a different offset of a5 and branches to another location.
- `calculate_and_store_offset_value`: Clears a 16-bit value at an offset relative to a5, calculates an offset by adding a shifted 16-bit value from offset 8 of a5 and a shifted 16-bit value from offset 4 of a5, checks a byte at that offset in a table pointed to by a4, and if non-zero, calculates a new value using a second offset and a second table, then stores the result at the first offset.
- `clear_offset_memory`: Calculates an offset by adding shifted 16-bit words from offsets 2 and 4 of a5, clears a 16-bit word at that offset in a structure pointed to by a4, and updates values at offsets 2 and 4 of a5.
- `fetch_and_validate_word`: Links a5, stores a local address on the stack, shifts a word from a global variable left by one, and uses it to fetch a word from a table.
- `check_multiple_conditions`: Compares a word at a global offset to 19, compares a word at an offset of a5 to 4, and compares a word at a global offset to a word in d0; if these conditions are met, it pushes one to the stack, calls another routine, and branches to another location; otherwise, it clears d0 and branches to another location.
- `dispatch_by_constant`: Compares a word at a positive offset of a5 against several constants, and if it matches none and is not less than 20, moves a word from a negative offset of a4 to a negative offset of a5, calls another routine, and moves a word from a negative offset of a5 into d0 before branching.
- `branch_with_word`: Clears d0, moves a word from offset 8 of a5 into d0, and branches to another location.
- `fetch_table_value`: Loads a word from a negative offset of a4, shifts it left by one, uses it as an index to fetch a word from a table at a negative offset of a4, stores the result on the stack, calls another routine, and branches to another location.
- `branch_to_address_2`: Performs a short branch to a fixed address.
- `branch_by_index`: Compares a longword in d0 to 26, branches if it is less than 26, otherwise shifts d0 left by 1, fetches a word from a table using d0 as an index, and jumps to the address at that table location.
- `fetch_indexed_word`: Loads a word from a table using an index derived from a value at an offset of a4, then branches to another location.
- `clear_memory_range`: Writes three 16-bit values to offsets -20056, -20054, and -20058 from a4, then enters a loop that clears a 16-bit value at offset 336 from a pointer calculated from a 16-bit value at offset -2 from a5 and an offset from a4, repeating until a counter reaches 8.
- `decrement_and_loop_value`: Calculates a value by subtracting 32 from a word at an offset of a4, divides it by 8, and uses the result to access a table; if certain conditions are met, it decrements the original word and loops.
- `clamp_value_in_structure`: Compares a word at an offset of a4 with 38; if it is greater than or equal, it sets the value to 42 and moves it to another offset in a4, otherwise it branches to another location.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `calculate_and_store_value`: Calls two routines, calculates a value based on a word from a structure and a constant, stores the result in a structure offset, and then calls a third routine with several parameters on the stack.
- `search_table_for_pattern`: Iterates through a table to find a specific pattern or value, returning a calculated index or pointer.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `modify_word_from_table`: Decrements a word at a negative offset from a4, uses the result as an index to fetch a byte from a table at a negative offset from a4, stores that byte back to a negative offset from a4, and otherwise sets a word at a negative offset from a4 to 10 before returning.
- `modify_global_values`: Compares a word at a negative offset from a4 to 160, branches if it is less than or equal to 160, otherwise adds 14 to a word at a negative offset from a4, moves 163 to a negative offset from a5, and branches.
- `execute_multiple_calls_2`: Performs several calls to routines using parameters pushed onto the stack, including values derived from multiplying a word at an offset of a4 by 34 and adding offsets to values from a5.
- `pop_longwords_and_call`: Pops ten longwords from a pointer in a0 into d0, then checks two words at offsets of a4 to decide whether to call another routine or branch.
- `execute_two_calls`: Calls two routines and returns.
- `parse_string_delimiters`: Searches through a buffer for a specific character and replaces a delimiter character with a null terminator.
- `load_and_extend_word`: Loads a word from an offset of a4, extends it to a longword in d0, and branches to another location.
- `update_global_values`: Loads a byte from a global variable, extends it to a word, and calls another routine with it on the stack.
- `calculate_scaled_value`: Subtracts three from a word at a global offset, multiplies the result by 180, and stores the product in another global variable before branching to another location.
- `branch_via_table`: Compares d0 to 4; if it is not greater than or equal to 4, it branches to another location, otherwise it shifts d0 left by one, uses the result to fetch a word from a table, and jumps to an address calculated from that word.
- `call_and_unlink`: Calls another routine using a brief subroutine call, unlinks a5, and returns.
- `calculate_and_call_with_division`: Sets up a stack frame, calls a routine with three parameters, cleans up the stack, calls another routine, cleans up the stack, performs a division using a value from a4, stores the result in a5, calls a third routine, cleans up the stack, and performs a comparison and conditional branch.
- `call_with_calculated_offset`: Calculates an offset using a value from a5, pushes several parameters onto the stack including a longword from a table, and calls a routine.
- `execute_parameterized_calls_3`: Calculates an offset using a value from a5, pushes several parameters onto the stack, calls a routine, increments a value at an offset of a5, and then calls another routine if a comparison with 4 is not met.
- `calculate_offset_value`: Sets up a stack frame, clears a word at a negative offset from a5, moves 21 to a negative offset from a5, compares a word at a negative offset from a4 to 3, branches if it is less than or equal to 3, otherwise moves the word from that offset into d0, subtracts 4 from d0, stores d0 at a negative offset from a5, and branches.
- `transfer_word_to_register`: Moves a word from an offset of a4 to a location at an offset of a5, then moves a word from an offset of a5 to d0 and d1 before branching.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: removes the stack frame with unlk, then returns to the caller.
- `branch_by_table_lookup`: Compares d0 with 9; if d0 is less than 9, it shifts d0 left by 1 and branches to a location determined by a word at an offset of a4 plus d0, then jumps to another location.
- `branch_with_value_move`: Moves a word from a structure offset into d0 and branches to another location.
- `modify_global_word`: Compares a word at an offset of a4 against several constants and, if it falls within a specific range, subtracts 16 from it and stores the result in a location at an offset of a5.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `decrement_global_counter`: Compares a word at an offset of a5 with 1; if not equal, it checks a word at an offset of a4, and if that value is less than 24, subtracts 24 from it before branching.
- `xor_and_update_global`: Shifts a word from a structure into d0, performs an XOR between that value and a global variable, stores the result back in the global variable, and branches to another routine.
- `clear_memory_and_assign`: Clears a range of bytes in a memory block, then calls another routine and stores its return value into a global variable.
- `fetch_and_process_structure_fields`: Repeatedly calls a routine to get a byte from a structure, uses it as an index to fetch words or longwords from a table, and passes those values to other routines.
- `call_multiple_routines`: Pushes a longword from an offset of a4 onto the stack and calls a routine, then pushes two words and two longwords from offsets of a0 and a4 to call another routine, and finally calls a third routine if a word at an offset of a4 is not zero.
- `conditional_structure_update`: Calls a routine, checks two word values, and if both are non-zero, updates a byte in a structure and calls two other routines with parameters from a table and a structure.
- `call_with_parameters`: Calls a routine, pushes a longword and two words onto the stack, and then calls two other routines.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `perform_pointer_swaps`: Calls a routine with three parameters on the stack, then performs several pointer assignments between locations at offsets of a4 and a0, and calls the routine again before returning.
- `process_conditional_calls`: Clears words at offsets 8 and 10 from a5 if they equal 64, then pushes several values onto the stack including a longword from a negative offset from a4 and a word from offset 18 of a5, calls a routine at a negative offset from a4, and if a word at offset 12 of a5 does not equal a word at a negative offset from a4, calls another routine at a negative offset from a4.
- `call_with_stack_parameter`: Pushes the word 8 onto the stack, calls a routine using an offset from a4, increments the stack pointer by 2, clears d0, moves a word from a negative offset from a4 into d0, and branches.
- `call_with_stack_parameters`: Pushes several words onto the stack, including a longword from an offset of a4 and a word from an offset of a5, calls a routine, and then branches to another location.
- `perform_scaled_calculations`: Calculates two different offsets by multiplying values from a5 by 34 and adding a base address, pushes these and other parameters onto the stack, calls a routine, performs arithmetic on d1, and calls another routine twice before branching.
- `execute_scaled_logic_sequence`: Calculates offsets using values from a5 multiplied by 34, calls a routine, performs arithmetic on d1, and calls another routine twice using parameters from a5 and a4 before branching to another location.
- `decrement_and_branch_d0`: Decrements the value in d0 and branches to different locations if d0 reaches zero after subtracting 1 or 7.
- `call_with_constant`: Pushes a word containing one onto the stack, calls a routine, and returns.
- `update_buffer_and_set_flag_2`: Performs a series of indirect calls to update or clear memory ranges and, if a certain condition involving a global variable is met, sets a flag in a global variable.
- `update_buffer_and_set_flag`: Calls a routine with several arguments, stores the result in a variable, and sets a flag if certain conditions involving a global variable and a pointer-based value are met.
- `update_buffer_pointers`: Updates several pointers or values in a structure using a function that returns a value, and sets a flag if a certain condition is met.
- `execute_conditional_memory_copy`: Calls a function to check a condition and, if successful, performs multiple memory copies or transfers using a specific size or range.
- `copy_buffer_with_offset`: Copies dwords from a pointer-based table, calls another routine, and conditionally writes incremented bytes to a calculated memory offset.
- `accumulate_table_values`: Iterates through a nested loop to add values from one table into another table at calculated offsets.
- `update_table_with_max_value`: Iterates through a loop to compare values in a table and updates a global table with a new value if it is larger.
- `copy_table_parameters`: Copies several values from a source table to a destination table and then calls a function that performs a conditional memory copy.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_5`: Function exit sequence shared or jumped to by a routine: removes the stack frame with unlk, then returns to the caller.
- `find_marker_and_call`: Iterates through a table until a specific byte value is found, then calls another routine multiple times using the discovered offset or an incremented offset.
- `populate_buffer_from_table`: If a certain condition is met, copies bytes from a table into a destination buffer using an offset, or otherwise checks a table and updates a local counter.
- `swap_array_elements`: Searches a table for a matching value and swaps two adjacent elements in that table if a certain condition is met.
- `map_buffer_indices`: Iterates through a buffer and, if a certain byte is non-zero, copies a byte from a source table to a destination table at a calculated offset.
- `return_empty_routine_3`: the routine performs no operations and immediately returns.
- `empty_c_function_5`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `initialize_and_reorder_pixel_buffer`: Clears a secondary buffer and then reorders or copies pixel-related data into a new table structure based on conditional checks of existing values.
- `compare_and_recurse_table`: Iterates through a table and calls another routine if certain adjacent or nearby values in the table match.
- `compare_table_indices`: Compares two values retrieved from a table and calls another routine with a modified parameter if a certain condition or comparison result is met.
- `calculate_index_offsets_and_reorder_table`: This routine clears a counter array, then iterates through a range of indices to compare values in a table and increment counters if certain conditions are met.
- `swap_table_values`: Reads two word-sized values from a table and writes them back to a different index in the same table, effectively swapping or overwriting values.
- `update_table_with_sum`: Calculates the sum of two short values from a table and passes this sum, along with a pointer and a constant, to a series of called subroutines.
- `process_pixel_value_differences`: Clears a memory buffer and then iterates through a table of byte values, comparing adjacent bytes and updating or decrementing values in a separate memory area based on the comparison results and a call to another routine.
- `find_matching_differences`: Iterates through a table and compares two values, storing an index and a difference into a destination table if a match is found.
- `modify_memory_via_pointer`: The routine performs a sequence of instructions that modifies a value in memory using a pointer held in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine`: calls a subroutine at a fixed offset.
- `call_subroutine_at_offset_1f0`: Calls a subroutine located at an offset of 496 from the start of the executable.

### paranoimia#unpacked

- `initialize_and_copy_data`: Calls a routine, performs a loop to copy words from one memory area to another, sets several values in a structure, checks a bit in a hardware register, and calls three routines via a pointer from a library.
