# Game 2213 (tv sports basketball (1990)(cinemaware)[cr pna][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02213). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1828036 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 29 of 29 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 926 of 926 named (0 author symbols), 926 of 926 described (226 by rule, 677 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4372 of 4376 (99.91%) |
| Recovered without load address (counted in proof 1, flagged) | 412683 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16696 of 16715 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 626671 |
| code | 66824 |
| data | 774000 |
| padding | 12893 |
| unused | 347648 |

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
| Disk-Validator | 1 | 386 | 1 |
| paranoimia | 2 | 83 | 3 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 1 of 2).adf:bb | 3 | 8490 | 446 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 1 of 2).adf:bootblock | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 120 | 120 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 220 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 492 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2893 | 90 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1076 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| paranoimia#unpacked | 1 | 55 | 1 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 1 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 2 of 2).adf:Fluff.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 2 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 192 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 183 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 517 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 561 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 2 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 92 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- tv sports basketball (1990)(cinemaware)[cr pna][a2](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 1 of 2).adf:S/paranoimia: crunched executable unpacked with ancient scan (ByteKiller 1.3 (xfd), 6920 -> 12322 bytes), 5 of 5 windows confirmed in reference RAM, loaded at an address: recovered as u25
- tv sports basketball (1990)(cinemaware)[cr pna][a2](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 1 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][a2](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 2 of 2).adf:Fluff.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][a2](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 2 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate
- tv sports basketball (1990)(cinemaware)[cr pna][a2](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 4 never-loaded region(s), 412683 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_unreachable_code_sequence` (More): An unreachable code block that executes a routine with a global variable, calls itself recursively, and then executes a function pointer stored in an unaffected address register. Kind: sabotage path. Pass path: u0__sub_2_0D82.

## Functions


### More

- `initialize_game_environment`: Initializes the game state by restoring patch states, allocating memory, and parsing a file header or string-based configuration.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `process_character_glyph_and_buffer`: Checks a character or byte against a set of specific values and, if certain conditions or character ranges are met, clears a memory buffer and updates a pointer-based table or structure.
- `dispatch_entity_data`: Checks a longword at offset 60 of a structure in a2; if non-zero, it calls a flag dispatcher, an entity updater, and potentially a text window drawer or another routine using values from a2 and the stack.
- `execute_unreachable_code_sequence`: An unreachable code block that executes a routine with a global variable, calls itself recursively, and then executes a function pointer stored in an unaffected address register.
- `initialize_entity_state`: Initializes fields in a structure and invokes another routine with values or pointers from that structure, conditionally calling further routines or looping based on certain structure fields.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from an input pointer, stores the second result into a field of the pointed-to structure, and if a flag field is zero, increments a short located at an offset from a pointer stored in that structure.
- `parse_buffer_content`: Uses another routine to check a buffer, updates a size field, and scans for a newline or form-feed character while potentially modifying a flag or string content.
- `process_entity_list`: Determines a loop limit based on fields in a passed structure and iterates to call another routine using a pointer to a table and a global variable.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Runs a sequence of calls to another routine using arguments from a parameter block and a global variable, returning the result of one of those calls.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at an offset from a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: Calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: Calls a drawing routine with a fixed or zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `draw_text_line`: Uses a handler pointer to call drawing primitives while iterating over a text buffer, tracking line count and stopping if a certain character or flag is encountered.
- `dispatch_recursive_pointer`: Calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition or partial way is met, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `reset_state_and_move_offset`: The routine resets a set of fields in a structure and then calls another routine with a pointer into the structure and a size derived from a field, adding the returned value to a field when positive.
- `update_scroll_window`: Calculates a target window position based on provided offsets and checks it against current window bounds.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_uppercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `process_message`: Recursively processes a structure by invoking another routine on certain fields and string arguments, then conditionally updates structure fields and invokes further routines.
- `build_message_buffer`: The routine obtains a pointer from another routine and then calls another routine with that pointer, a constant, a stack-allocated buffer containing the input parameter, and a size of one.
- `copy_entity_data`: The routine obtains a handle from another routine and, if valid, reads a value from that handle, passes it to another routine together with a small stack buffer, and copies two fields from the handle into the caller-provided structure.
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
- `swap_and_call_modular_inverse`: Moves d2 to the stack, swaps d1 and d0, calls another routine, restores d0 from d1, and restores d2 from the stack before returning.
- `calculate_modular_inverse`: Manipulates d2 and d1 by negating or bitwise inverting them based on their sign, then calls modular_inverse_32bit with a pointer in a0 before returning a result in d2.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_pointer`: Stores a byte from a register through a pointer.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port_instance`: Checks a specific field in a structure, and if non-zero, updates status or deletion flags and recursively processes a linked list or associated memory block.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `restore_stack_and_call`: Copies a fixed number of words from a source address to a stack-based location and then calls another routine.
- `process_complex_data_structure`: Sets a bit on the stack, updates a value in a1, clears several offsets in a1 and a2, and executes a long sequence of conditional calls to various routines using pointers from a2 and indices derived from a1 and a0.
- `branch_to_relative_offset`: Performs a short branch to a relative offset.
- `set_global_byte`: Sets a global variable at offset 644 of a structure in a2 to negative one and branches to another location.
- `set_structure_byte`: Sets a byte at an offset in a structure in a1 to negative one and branches to another location.
- `call_with_offset_calculation`: Calculates an offset using values from a structure in a1, reads a byte from a0, and calls another routine with a value from a global variable and a pointer in a4 before branching to another location.
- `compare_and_call`: Calculates an offset from values in a1, reads a byte from a0, and calls a routine via a5 with d0 set to 240 using a pointer from offset 300 of a2, then branches based on comparisons of d1 with 72 and 78.
- `update_structure_and_call_handlers`: Increments a longword at offset 548 of a1, fetches a byte from a table using an index from offset 220 of a1, and performs multiple conditional calls via a5 using pointers from a2 and a1.
- `execute_conditional_calls`: Performs a series of calls via a5 using pointers from offsets 216, 220, 148, 248, and 292 of a2, updating various offsets in a1 and a2 based on the results and conditional branches.
- `branch_to_offset`: Performs a short branch to an offset.
- `call_indirect_with_constant`: Loads the constant 232 into d0, loads a pointer from offset 336 of a structure in a2 into a4, calls the routine at a5, and jumps to the address in a6.

### Assign

- `reconstruct_lookup_table`: Iterates through a linked list or table to find a maximum value, then populates a new memory area with a sequence of incrementing or decrementing values.
- `copy_config_block_3`: copies a fixed-size block of 16 bytes from a memory offset or configuration structure to the stack and then calls another routine.
- `update_indexed_data_structure_3`: Calculates several offsets in a structure in a1 based on a value in d1, clears two longwords at offsets 472 and 476, and performs multiple conditional calls to routines using pointers from a2 and values derived from a1.
- `update_indexed_data_structure_2`: Uses values from a1 to index into a table in a0 and performs several conditional calls to routines using pointers from a2.
- `update_indexed_structure_fields`: Loads a value from offset 492 of a1, uses it to index into a table in a0, and stores the result in offset 496 of a1.
- `update_indexed_data_structure`: Uses values from a1 to index into a table in a0, performs several conditional calls to routines using pointers from a2, and updates various fields in the structure in a1 based on the results.
- `update_structure_and_call_routines`: Moves values between a1, a0, and a2 using various offsets and bit shifts, calls several routines via pointers in a5, a4, and a6, and performs multiple conditional checks on values at offsets 484, 472, 488, and 476 of a1.

### EndCLI

- `initialize_game_environment_4`: Traverses a linked list or table to find a maximum value and then populates a memory buffer with an incrementing sequence of integers.
- `copy_config_block_4`: copies a fixed-size block of sixteen bytes from a memory offset to the stack and then executes another routine.
- `process_data_structure`: Modifies a value at a location in a2, calls several routines via a pointer in a5 with various arguments, and decrements a value at an offset in a0 if a condition is met before jumping to a location in a6.

### Dir

- `initialize_game_environment_5`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with a sequence of incrementing or decrementing values.
- `copy_config_block_5`: copies a fixed-size block of 16 bytes from a memory offset to the stack and then calls another routine.
- `process_directory_entries`: A routine that performs multiple comparisons and conditional calls using pointers from a5 and a4, and updates values in structures pointed to by a1 and a2.
- `update_directory_state_3_2`: Tests a longword at an offset of a1, and if non-zero, calls a routine using a pointer in a5 and a modified a4, then stores a value from a1 into another offset of a1.
- `check_and_clear_offset`: Tests a longword at offset 384 of a1; if it is non-zero, it clears that longword and branches to another location, otherwise it returns.
- `update_directory_state_3`: Loads a value from offset 8 of a1, calls a routine using a4 and a5, and stores the result in offset 320 of a1.
- `update_directory_state_2`: Checks a long at offset 324 of a1; if non-zero, it uses a value from a1 to index into a table at offset 12 of a0, calls a routine via a5, and performs several checks and updates to a1 and a2.
- `update_directory_state`: Performs multiple calls to a routine via a5 using various offsets and values derived from a1 and a0, and updates several longwords in a1 based on the results.
- `manage_directory_entries`: Checks several longwords in a2 and a1; if conditions are met, it calls a routine via a5, decrements a word at offset 4 of a1, and updates offsets 356 and 360 of a1.
- `calculate_directory_offset`: Adds a value from offset 58 of a1 to 2, calculates a shifted value, and calls a routine via a5 before branching to another routine.
- `update_directory_entry`: Calls a routine via a pointer in a5, decrements a value from a1, calls another routine via a5 with a modified pointer, and conditionally sets a value in a2 to 20 and another in a2 to the value from a1 before jumping to a location in a6.

### Echo

- `initialize_program_metadata_and_tables`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with a sequence of incrementing or fixed values.
- `restore_stack_and_call_2`: Copies a fixed number of words from a source or source-relative address to a stack-based location and then calls another routine.
- `update_indexed_data_structure_5`: Shifts a value in d1 right by two, stores it in a global variable, and then performs two conditional calls to a routine via a5 using values derived from a table and offsets from a2.
- `update_indexed_data_structure_4`: Uses a value from a1 to index into a table in a0, calls a routine via a5 with d0 set to 372, stores a value from the table at offset 328 of a1, calls a routine via a5 with d0 set to 376, and performs several conditional branches and data movements involving offsets 332 and 336 of a1.
- `copy_and_branch_data`: Moves a longword from offset 336 of a1 to d1, stores d1 at offset 332 of a1, and then branches to another location.
- `shift_and_compare_longwords`: Stores 1 at offset 328 of a1, then uses a value from a1 to index into a table in a0, and performs several comparisons and branches based on longword values at offsets 332 and 336 of a1.
- `shift_longword_and_branch`: Moves a longword from offset 368 of a1 to d1, stores d1 at offset 364 of a1, and branches to another routine.
- `echo_conditional_byte`: Shifts longwords within a1, compares them, and if a condition is met, uses a value from a1 to index into a table at offset 0 of a0 to fetch a byte.
- `execute_conditional_echo_2`: Checks a byte at an offset in a0 determined by a1, and if non-zero, calls a routine via a5 before branching to another routine.
- `execute_conditional_echo`: Checks a value in a2, and if non-zero, calls a routine via a5 with different arguments and offsets, then jumps to a location in a6.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Execute

- `initialize_game_environment_6`: Calculates the maximum value in a linked list of integers, populates a table with an incrementing sequence of values, and copies a block of data from a source buffer to a destination buffer while updating a function pointer in a table.
- `copy_config_block_6`: copies a fixed-size block of sixteen bytes from a memory offset to a stack-based or global buffer and then calls another routine.

### Copy

- `reconstruct_table_and_copy_data`: Iterates through a linked list or table to find a maximum value, then populates a memory region with an incrementing sequence of integers.
- `copy_stack_from_memory`: Checks if a stack pointer is non-null and copies a fixed number of words from a memory offset into the stack before calling another routine.
- `update_structure_and_call_handler`: Calculates an offset in a1, clears several longwords in a2, calls a routine using a pointer from a2, and then performs a conditional byte comparison from a table in a0 to call another routine.
- `fetch_and_store_bytes`: Adds a value from offset 536 of a1 to a value from offset 680 of a2, uses the result to index into a table in a0 to fetch a longword, then fetches two bytes from the table using values from a1 and stores them at offsets 544 and 548 of a1.
- `call_routines_with_offset`: Adds a value from offset 540 of a1 to d1, calls a routine using a pointer from a4, and then calls another routine using a different pointer from a4 before branching back.
- `call_routines_conditionally`: Adds a value from offset 540 of a1 to d1, calls a routine using a pointer from a4, and if the longword at offset 548 of a1 is non-zero, calls another routine using a pointer from a4 before branching back.
- `compare_value_against_constants`: Loads a longword from offset 544 of a1 into d1 and compares it against the constants 35, 37, 40, and 124, branching to different locations based on the results.
- `compare_and_jump`: Increments a longword at offset 218 of a1, compares it to a longword at offset 2ac of a2, and if the comparison fails, fetches a byte from a table in a0 and either jumps to an address in a6 or branches to another routine.
- `branch_on_longword`: Tests a longword at offset 2ac of a2 and branches if non-zero; otherwise it sets d1 to zero and jumps to the address in a6.
- `update_table_index_2`: Increments a longword at offset 532 of a1, uses a value from offset 8 of a1 to index into a0 to fetch a byte, stores that byte in a1 at offset 700 of a2, and then performs a series of operations involving clearing offsets 684 and 688 of a2 before branching to another routine.
- `jump_to_increment_and_branch`: Jump stub: passes control straight on to increment_and_branch.
- `update_state_from_table`: Fetches a byte from a table using an index derived from a structure and stores it in another structure offset, then calls another routine if this value matches a value in a global variable.
- `compare_longword_against_constants`: Compares a longword from offset 552 of a1 against several constant values, branching to different locations depending on the comparison results.
- `increment_and_branch`: Adds 1 to a value from offset 544 of a1 and branches to another routine.
- `branch_to_offset_52`: Performs a short branch to offset 52.
- `update_table_value`: Tests d1 and branches if non-zero; otherwise it sets offset 688 of a2 to negative one and branches to another routine.
- `verify_and_increment_copy`: Moves a longword from offset 684 of a2 to offset 4 of a1, compares a value in a1 to 1, and if not greater, checks a table at an offset in a2 against a value in a1 before branching.
- `copy_incremented_data`: Increments a value at an offset in a2, calculates an offset using a value from a2 and a1, and moves a longword from a1 to that calculated address before jumping to a location in a6.
- `compare_and_set_byte`: Compares a longword at offset 708 of a2 with a longword at offset 704 of a2, and if the first is not greater, stores the value -1 into offset 700 of a2 before branching.
- `store_byte_with_offset`: Increments a value at an offset in a2, calculates an offset using a value from a2, reads a byte from that offset into d2, and stores d2 into a different offset in a2 before jumping to a location in a6.
- `call_routine_on_match`: Compares a value in a2 with 37, and if they match, calls a routine via a5 twice with a specific offset in a4 before jumping to a location in a6.
- `copy_and_call_structure`: Copies a longword from offset 704 of a2 to a1, copies another longword from offset 700 of a2 to offset 4 of a1, then calls a routine using a pointer in a5 with d0 set to 20 and a pointer in a4 derived from offset negative 28 of a4 before branching to another routine.
- `set_constant_and_load`: Stores 255 into an offset in a2 and loads a longword from a1 into d1 before jumping to a location in a6.
- `execute_dual_calls`: Calls a routine via a5 with a pointer in a4, loads a longword from a1 into d2, calls the routine again with a different offset in a4, and loads the value from a1 into d1 before jumping to a location in a6.
- `copy_and_verify_value`: Calls a routine via a5, stores the result back into a1, compares d2 with a value in a2, and if they do not match, stores -1 in a2 and calls the routine again via a5 before jumping to a location in a6.
- `validate_constant_value`: Compares a longword from offset 4 of a1 against several constant values, branching to different locations if a match is found.
- `execute_conditional_calls_with_offsets`: Clears a longword at offset 4 of a1, calls a routine using a4 offset -132, stores the result in offset 8 of a1, and performs a series of comparisons and conditional calls via a5 based on a value at offset 700 of a2.
- `write_byte_and_call`: Calculates an offset by adding a value from a1 to the value at offset 704 of a2 shifted left by two, writes a byte from offset 707 of a2 to a0 at that offset, copies a longword from offset 704 of a2 to a1, then calls a routine using a pointer in a5 with d0 set to 24 and a pointer in a4 derived from offset negative 160 of a4 before branching to another routine.
- `call_external_routine`: Loads a long value from offset 444 of a structure in a2 and a long value from offset 8 of a1 into d2 and d1, sets d0 to 24, loads a pointer into a4 from offset 128 of a4, calls a routine via a5, and branches to another location.
- `branch_after_nop`: Does nothing and then branches to another location.
- `update_table_and_copy_byte`: Loops through a table, copying a byte from a source at an offset calculated from a value in a global variable and a pointer in a1 to a destination, then stores the byte at offset 0 of the destination and updates a pointer at offset 8 of the structure in a1.
- `store_and_jump`: Stores d1 into offset 8 of a1, tests d1 and branches if non-zero; otherwise it moves d2 to d1 and jumps to the address in a6.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `copy_bytes_in_loop`: Loops through a table, copying a byte from a source at an offset calculated from a value in a global variable and a pointer in a1 to the location pointed to by a1, continuing until the copied byte is zero, then writes a byte from a source to offset 7 of the structure in a1.
- `clear_and_copy_byte`: Clears several long values at offsets 680, 684, 700, and 688 of a structure in a2, stores d1 and d2 at offsets 692 and 696 of a2, clears 704 of a2, reads a byte from a table in a0 at an offset derived from d3, stores it in 708 of a2 and 8 of a1, and performs a loop that updates a byte in a0 using d1 and d2.
- `execute_three_calls`: Calls another routine three times with different arguments in d0 and a4, then performs a bitwise NOT on a value from a global variable and jumps to a location in a6.
- `update_table_bytes`: Reads two bytes from a table in a0 at offsets derived from d3 and d5, stores them in a1 at offsets 8, 12, and 16, and enters a loop that reads a byte from a0 and writes it back to a0 at an offset calculated from a1.
- `add_and_store_byte`: Adds the values at offset 8 and offset 12 of a structure in a1, then stores the resulting byte at an offset in a0 calculated from the value at offset 4 of the structure in a1 shifted left by two.
- `process_and_copy_data`: Copies a byte from a source to a destination using offsets from d1 and d2, updates a value at offset 12 of a structure in a1, and if a condition is met, calls another routine and writes a byte to a destination at an offset calculated from a1 and d1.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `execute_multiple_calls_2`: Sets d1 to 1 and d0 to 16, calls a routine via a5 with a pointer from offset 116 of a2, updates offset 4 of a1, and if d1 is non-zero, performs two more calls via a5 using pointers from offsets 292 and 336 of a2.
- `move_long_values`: Moves a long value from a source at offset 4 of a structure in a1 to a destination in a0 at an offset calculated from that value, then moves a long value from a source at offset 4 of a structure in a1 to a global variable and another long value from a1 to a destination in a0.

### Makedir

- `initialize_game_environment_7`: Calculates the maximum value in a linked list or table, then populates a memory-mapped or global table with a sequence of incrementing values.
- `copy_stack_from_memory_2`: Performs a loop to transfer a block of data from a memory location to the stack and then executes a function pointer.
- `perform_system_calls`: Sets a value in a1 by adding a1 to a shifted version of itself, clears offsets 208 and 212 of a1, and performs a sequence of calls via a5 using pointers from offsets 312, 40, and 292 of a2 to update values in a1.
- `perform_multiple_system_calls`: Performs a sequence of calls to another routine using various offsets from a2 and a4, updating values at offsets 216, 212, 216, 212, 208, and 212 of a structure in a1 based on the results.

### CD

- `initialize_game_environment_2`: Iterates through a linked list or table to find a maximum value, then populates a memory buffer with an incrementing sequence of integers.
- `copy_config_block`: copies a fixed-size block of 16 bytes from a memory offset or table into a stack-based or local buffer, then executes a previously stored function pointer.
- `process_multiple_calls`: Increments a value at a3 and increments a3, calculates and stores three different longword offsets into a1, clears offset 608 of a2 and stores a calculated value into offset 612 of a2, then performs several calls via a5 using various pointers from a2 and a4 and various values in d0, including a conditional branch based on a byte in a0.
- `process_conditional_data`: Performs two calls via a5 using pointers from offsets 292 and 336 of a2, then checks a byte in a0 at an offset from a1; if non-zero, it calls a routine via a5 using a pointer from offset 492 of a2 and potentially performs more calls using pointers from offsets 296 and 336 of a2.
- `update_multiple_parameters`: Performs a series of calls to a routine at a5 using various offsets from a2 and a1, including loading values from a structure in a0 via an index in d2, and ends with a jump to a6.

### EndIf

- `return_zero`: The routine is empty and simply returns a zero value to the caller.

### If

- `initialize_game_environment_3`: Traverses a linked list or table to find a maximum value, then populates a memory-mapped table with an incrementing sequence of integers.
- `copy_config_block_2`: copies a fixed-size block of 16 bytes from a memory offset or table into a stack-based or local buffer and then calls another routine.
- `execute_conditional_logic_3`: Calls a routine to compare a byte in a0 with a value in a2 using a longword from a1, and calls another routine to test a byte in a0 and potentially perform a bitwise NOT on offset 328 of a1.
- `compare_and_branch_byte`: Loads a longword from a1, shifts it left by two, and compares a byte at an offset in a0 with a value from a2; if the comparison passes, it sets d1 to 255 and branches.
- `test_and_branch_condition`: Uses a value from a1 to index into a table in a0, compares the byte at offset 12 of that entry with the value at offset 544 of a2, and branches based on the comparison.
- `update_state_from_table_2`: Fetches values from a table using indices derived from a structure, updates several longwords in a second structure, and calls a routine via a pointer in a5 multiple times based on byte comparisons.
- `execute_conditional_table_updates`: Performs three separate sequences of calling a routine via a5 with values from a1 and a2, then checks a byte in a table in a0 against the constant 36 before proceeding.
- `branch_to_offset_254`: Performs a short branch to a relative offset of 254.
- `check_table_entry_and_branch`: Uses a value from a1 to index into a table in a0, checks if the byte at offset 24 of that entry is non-zero, and compares the value at offset 356 of a1 with 1 before branching.
- `branch_to_offset_596`: Performs a short relative branch to a location 596 bytes ahead.
- `test_and_branch_logic`: Compares a longword at an offset of a1 with one, and if equal, branches; otherwise, it tests the longword and branches based on its value or sets d1 to a constant and branches.
- `test_table_and_call_logic`: Fetches a longword from a1, uses it as an index to test a byte in a0, and if non-zero calls a routine via a5 using a pointer from offset 432 of a2, then potentially calls another routine via a5 using a pointer from offset 436 of a2.
- `check_and_call_logic`: Stores zero in d1 at offset 328 of a1, tests a byte in a0 at an offset derived from a1, and if non-zero performs a bitwise NOT on offset 328 of a1; if that value is zero, it clears offset 364 of a1, sets offset 600 of a2 to 10, and calls a routine using a pointer in a5 with d0 set to 412 and a pointer in a4 from offset 244 of a2.
- `execute_conditional_logic_2`: Sets d0 to 380 and calls a function using a pointer from a structure in a2, then updates a value at offset 600 of that structure.
- `execute_complex_logic_sequence`: Sets d0 to 380, calls a routine using a pointer from a structure in a2, updates a value at offset 600 of that structure, and enters a loop that conditionally calls another routine and performs further calls based on values in a1 and a2.

### IconX

- `initialize_program_environment`: Parses a string or file content to extract command-line arguments or file paths, handling quoted strings and escape sequences, while initializing internal structures with memory-allocated pointers and library-retrieved file or directory information.
- `update_state_and_invoke_callbacks`: Updates a field in a structure pointed to by a retrieved pointer and conditionally executes a sequence of cleanup or notification callbacks stored in a table, passing a specific value to a structure before each call.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and a name, including a wait command and an endcli redirection, after checking if the name starts with a dot, then runs the command through the DOS library.
- `install_library_functions`: Creates and installs a library from a descriptor by making the library structure, setting up its function table, and returning the result or zero if no descriptor is provided.
- `scan_ram_entries_for_names`: Allocates a 260-byte buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `join_strings_into_buffer`: Copies one string into a destination buffer and joins a second string onto it.
- `count_string_length`: Receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal_2`: Converts a null-terminated string of digits into a signed integer, handling an optional leading plus or minus sign.
- `execute_subroutine`: loads a function pointer from a global variable and jumps to it.
- `execute_indirect_function`: loads a function pointer from a global table and executes it.
- `execute_scheduled_handler`: Loads a function pointer from a location relative to a global pointer and executes it.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Loads a function pointer from a memory location offset by a constant and executes it.
- `execute_indirect_function_3`: loads a function pointer from a location relative to a global pointer and executes it.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `execute_indirect_function_4`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_5`: loads a function pointer from a global table and executes it.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `execute_indirect_function_6`: loads a function pointer from a memory location and executes it.
- `invoke_string_comparison_with_stack_ref`: Calls a string comparison routine using two parameters and a pointer to a specific location on the stack.
- `execute_indirect_function_7`: loads a function pointer from a memory offset and executes it.
- `execute_indirect_function_8`: loads a function pointer from a global table and jumps to it.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `execute_indirect_call`: Loads a function pointer from a global memory location and executes it.
- `execute_indirect_call_2`: Retrieves a pointer from a global offset and performs an indirect jump to that address.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `invoke_and_checksum_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer_2`: Stores a byte from a register into a memory location pointed to by a register.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_register_to_pointer`: stores the contents of a data register into a memory location pointed to by a register.

### Disk-Validator

- `format_string_and_initialize_table`: Converts a numeric value into a string or a space and stores it in a buffer, then initializes a memory-mapped table or array with bitmask-related or sentinel values.

### paranoimia

- `decompress_and_initialize_system`: Unpacks a compressed stream by reading bits from its end and copying back-referenced runs backward into memory, then checks a checksum.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 1 of 2).adf:bb

- `initialize_memory_and_dispatch_logic`: Clears a block of memory to zero and then executes a conditional branch or a function call based on the state of a global variable and a secondary function pointer.
- `update_string_metadata`: Compares two values, copies a sequence of words from a source to a destination, and updates a string or buffer with a length and certain characters.
- `swap_linked_list_nodes`: Checks a pointer against a global value, swaps two adjacent pointers in a linked list, and updates a global pointer to point to the new head or a node.
- `process_linked_list_nodes_2`: Iteratively calls a routine to check a table, potentially calls another routine with a status flag and a pointer, and clears certain memory locations or calls a routine with a fixed value.
- `update_buffer_and_linked_list`: Updates a sequence of memory or buffer entries by incrementing pointer values by a fixed offset and manages linked list node pointers.
- `copy_data_table`: Copies thirty-two words from a table into a destination buffer pointed to by a parameter.
- `update_buffer_recursively`: Recursively updates a buffer or a table entry with a value from a parameter, or updates a pointer-based destination if a certain condition is met.
- `iterate_table_and_increment`: Iterates through a table by decrementing a counter, calling another routine with a pointer-based value, and incrementing the value stored at that pointer.
- `update_table_recursively`: Recursively updates a table by incrementing a counter and modifying values through a pointer until a termination condition is met.
- `compare_word_and_branch`: Links a5, compares the word at offset 12 of a5 with 98, and if the value is less than or equal to 98, branches to another routine; otherwise it moves the value to d0 and branches to another routine.
- `compare_and_branch_word_2`: Compares the word 32768 with 130 and branches if the carry flag is set; otherwise it sets d0 to 130 and branches to another routine.
- `compare_and_branch_word`: Compares a word at offset 12 of a5 to 98 and branches if it is less than or equal to that value; otherwise it moves the word to d0 and branches.
- `write_and_compare_word`: Writes the value 98 to a word at offset -20614 from a4, then compares a word at offset 14 of a5 to 25 and branches if it is less than or equal to that value.
- `write_constant_to_memory`: Sets d0 to 25, writes this word to an offset of -12906 from a4, unlinks a5, and returns.
- `wait_for_value_match`: Enters a loop that continues until the contents of two memory locations are equal.
- `rotate_bits_in_memory`: Calls a library function, stores the result in a global variable, adds a constant, and then performs a bitwise rotation or shift-based update to that global variable.
- `calculate_leading_zeros_and_mask`: Calculates the number of leading zeros in a value and performs a bitwise AND with a value loaded from a table.
- `load_from_offset`: Loads a longword from a memory location offset by a negative constant into a register.
- `rotate_bits_in_memory_2`: Stores a result in a global variable, adds a constant, and performs a bitwise rotation or shift-based update.
- `decrement_and_execute_list`: Decrements two global short values, updates a third global short based on a subtraction, and executes a series of functions via a linked list.
- `update_global_state`: Copies a value from one global variable to another and updates several global shorts or bytes with constant values.
- `insert_linked_node`: Inserts a new node into a linked list if the second element of the provided structure is zero.
- `update_linked_list_node`: Traverses a pointer chain to find a specific node and updates its next pointer to a new value before clearing a secondary field.
- `traverse_pointer_chain`: Follows a chain of pointers until a target address or a null-terminating condition is met, then updates a destination pointer to a new value and clears a following word in memory.
- `assign_pointer_and_clear`: Copies a pointer from one location to another and then clears the value at the offset of that pointer.
- `reset_global_pointer`: Calls another routine with specific arguments if a global variable is non-zero, then clears that global variable.
- `update_blt_registers`: Updates several bitplane-related registers and a DMA controller register using values or calculated bitmasks while checking a global flag.
- `empty_routine`: the routine is empty and performs no operations before returning.
- `calculate_blit_edge_mask_2`: Executes a routine that calls a function to calculate a bitmask or offset based on a pointer-referenced data structure and updates a destination or source address or counter.
- `calculate_blit_edge_mask`: Calculates a bitmask or offset based on a pointer-referenced data structure and updates a destination or source address or counter.
- `update_blit_parameters_2`: Iterates through a data structure to calculate and update values used for Blitter operations.
- `update_blit_parameters`: Iterates through a data structure to calculate and update values used for Blitter operations.
- `update_blitter_parameters`: Calculates and writes new source, destination, and control registers for the blitter, including masking and area-size adjustments.
- `return_immediately`: The routine is empty and performs no operations before returning.
- `initialize_input_state`: Sets certain memory or hardware registers to a fixed value and fills a buffer with a constant value using a loop.
- `execute_indirect_call_9`: Performs an indirect call to a routine using a pointer or address stored in a register or memory.
- `call_indirect_function`: Calls a function pointer located at an offset from a global pointer.
- `clear_and_combine_bytes`: Clears certain bytes in a table and returns a value constructed from those bytes and a third byte.
- `clear_and_accumulate_table_bits`: Iterates through a table of bytes, clearing certain bytes to zero while accumulating a bit-shifted value into a register based on the cleared and unmodified table contents.
- `iterate_and_clear_table`: Iterates through a table, clearing bytes and calculating a bit-shifted value based on table contents.
- `clear_table_bytes`: Clears certain bytes in a table and returns a value constructed from those bytes.
- `clear_and_shift_bit_table_2`: Iterates through a table, clearing bytes to zero and updating a bit-shifted value in an adjacent memory location.
- `clear_and_shift_bit_table`: Iterates through a table, clearing bytes to zero and updating a bit-shifted value in a secondary or adjacent memory location.
- `process_linked_list_nodes`: Iterates through a linked list, calling a function with certain parameters and potentially calling it a second time if a bit or a global flag is set.
- `initialize_buffer_structure`: Calls a function to get a pointer, then populates a buffer with values from a global variable and a parameter.
- `call_routine_with_offset_index`: If offset 8 of a5 is non-zero, it calculates an index from a word at offset 2 of a structure in a0, pushes this index and the structure pointer onto the stack, and calls another routine.
- `initialize_buffer_sequence`: Calls another routine, stores the result in a structure in a5, and if not zero, performs a series of conditional checks by calling a routine to populate a buffer from a source, comparing the results against specific constants, and eventually calls another routine to initialize a buffer structure.
- `process_stack_data`: Performs a loop that manipulates values from the stack and a table in a0, eventually calls a routine using a pointer from the stack, and stores an extended longword in a1 at offset -22.
- `call_and_store_extended_register`: Pushes two values onto the stack, calls a routine via a5, cleans the stack, extends d0 to a longword, stores it at an offset in a5, and branches.
- `call_and_return_value`: Pushes a value onto the stack, calls a routine, cleans the stack, loads a value from an offset in a5 into d0, unlinks a5, and returns.
- `combine_three_bytes`: Reads three bytes from offsets 9, 11, and 13 of a structure in a5, shifts them left by 8, 4, and 0 bits respectively, and combines them into d0 using bitwise OR.
- `split_word_into_three_destinations`: Extracts three different bit-shifted or masked segments from a word at offset 8 of a structure in a5 and writes each segment to three different pointers located at offsets 10, 14, and 18 of a5.
- `execute_loop_calls`: Iterates through a range defined by a parameter and calls a function using a value constructed from bit-shifted components.
- `transform_nibble_data`: Iterates through a table of words to perform bitwise manipulations on nibbles and stores the results in a local buffer.
- `execute_sequence_with_checks`: Calls a routine repeatedly until a certain value is returned, then updates a global variable with a constant value.
- `decode_instruction_stream`: Iterates through a data buffer and decodes or dispatches instructions based on a state machine.
- `decode_instruction_stream_3`: Decodes a stream of bytes from a table to populate a data buffer, using certain bits to determine the next instruction type and length.
- `parse_instruction_stream`: Iterates through a table of bytes to decode a custom instruction stream, updating a destination buffer and a loop counter based on certain bit-flags or immediate values found in the stream.
- `decode_instruction_stream_4`: Decodes a variable-length instruction stream by reading bytes from a table and updating a destination buffer.
- `parse_instruction_stream_2`: Iterates through a table of bytes to decode a custom instruction sequence, updating a destination buffer and a loop counter.
- `parse_instruction_stream_3`: Iterates through a table of bytes to decode a custom instruction or command sequence, updating a destination buffer with or calculating a loop limit based on certain bit-flags or or instruction-specific opcodes.
- `populate_data_buffer`: Iterates through a pointer-based table to load a sequence of bytes into a destination buffer, while simultaneously calculating a loop limit and updating a state-tracking global variable.
- `parse_variable_length_stream`: Iterates through a data buffer to decode a custom bitstream or instruction sequence, updating a counter and a pointer based on certain bit-flags or or value-dependent jump offsets.
- `decode_instruction_stream_2`: Decodes a stream of bytes by reading a pointer and using the byte's value to determine a branch or a sequence of operations.
- `process_data_stream_with_branching`: Iterates through a memory buffer while updating a counter and checking a secondary pointer.
- `process_entity_state_updates`: Iterates through a list of structures to update internal state values based on specific command bytes.
- `search_and_update_pointer`: Searches for a specific pattern in a memory range and, if found, updates a global pointer with a new address or a pointer to a structure.
- `iterate_entity_flags`: Iterates through a list of entity structures and calls another routine if a specific bit is set in a flag field.
- `iterate_entity_state_flags`: Iterates through a list of entity structures, checking a specific bit in a flag field and calling another routine if the bit is set.
- `increment_and_store_pointer`: Increments a global counter and, if it wraps to zero, calls a function to update a pointer, potentially resetting that pointer to a new value from a table.
- `update_animation_pointer_wrapper`: Calls a routine that updates a pointer in a table by adding a bit-shifted value from a register to an existing value.
- `update_pointer_and_increment_offset`: Calls a routine to decode an instruction stream and updates a table entry with a returned value.
- `update_pointer_and_modify_target`: Calls a routine to decode an instruction stream and updates a table entry with a returned value.
- `populate_animation_pointer_table`: Iterates through a bitmask-checked loop to resolve or update pointers in a table, potentially performing a block memory write of a global variable or decrementing a counter and re-executing a routine if certain conditions are met.
- `process_instruction_chain`: Calls a routine with a pointer from a global variable and stores the result in a global variable.
- `execute_conditional_loop`: Calls a routine with a specific offset and, if the result is non-zero, resets a global variable and potentially sets it to a different value.
- `update_table_pointer`: Calls a routine with a pointer from a global variable and a constant, then stores the returned value into a table entry.
- `store_decoded_pointer`: Calls a routine using a pointer from a global variable and stores the returned value into a table entry.
- `process_sprite_animation_chain`: Iterates through a linked list or table of animation or sprite-related data, updating pointers and calculating bit-shifted or masked values.
- `update_animation_frame_buffer`: Copies a block of memory from a source pointer to a destination buffer and updates a global timer or counter based on a calculated offset.
- `update_animation_pointer`: Updates a pointer in a table by adding a bit-shifted value from a register to an existing value.
- `update_entity_state_recursively`: Iterates through an entity list to check or update state flags and pointers, using a recursive or iterative loop to re-validate or re-initialize entity properties if certain bit-flags or null-pointers are encountered.
- `update_clamped_input_vectors`: Splits a 16-bit input value into two 8-bit components and calculates the difference from their previous values.
- `update_clamped_values`: Updates two values in a structure by retrieving data from tables based on an index, applying clamping logic to ensure the results stay within specified minimum and maximum bounds.
- `call_empty_routine`: Calls another routine with a constant value of fifteen.
- `execute_conditional_callbacks`: Checks several global pointers and, if they are non-zero, executes the routines or library functions they point to.
- `execute_callback_chain`: Iterates through a series of global function pointers and executes any that are non-zero.
- `initialize_library_and_state`: Loads library bases into global variables and initializes a small array to zero while performing indirect calls and conditional pointer checks.
- `invoke_copy_data_table_with_offset`: Calculates a source address by adding an offset to a global variable and passes this address and a parameter to a routine that copies a table of words into a destination buffer.
- `iterate_and_call_function`: Iterates thirty-two times, calling a function with a global pointer and a value retrieved from a pointer-indexed table.
- `clear_global_pointer_target`: Clears a byte at a location derived from a global pointer and a pointer-indexed table, then calls another routine.
- `execute_loop_calls_2`: Calls a routine twice using a range or certain values, passing a destination buffer and a range or certain values.
- `call_function_with_split_parameter`: Calls a function with a constant value and calls another function using a value split from a single parameter and a constant.
- `execute_callback_with_table_lookup`: Calculates an address using a table-based offset and executes a conditional callback or a routine with a specific address if a library call fails.
- `update_pointer_from_table`: Uses a table-based offset to retrieve a pointer, executes a routine, and stores the result in a global variable.
- `traverse_entity_list`: Recursively calls another routine if a certain bit is set in a field within a list of structures.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `call_function_with_global_pointer`: Calls a function using a value stored at a global pointer location.
- `call_function_with_global_pointer_alt`: Calls a function using a value stored at a global pointer location using a different function pointer.
- `call_function_with_conditional_args`: Calls a routine using values retrieved from memory locations pointed to by global variables, swapping the order of arguments based on a condition.
- `call_function_with_indirect_values`: Calls a function using two values retrieved from memory locations pointed to by global variables.
- `call_function_with_parameter`: Calls a function using a global pointer and a provided parameter.
- `return_empty_routine`: the routine performs no operations and immediately returns.
- `execute_library_function_with_bounds`: Calls a library function with a pointer and a size, then conditionally updates a memory range or a global variable with a new value.
- `update_buffer_contents`: Copies data between buffers and updates a structure or a pointer with the result of a memory-related function.
- `update_buffer_contents_2`: Performs a sequence of memory copies and updates a structure or a pointer with the result of a memory-related function.
- `update_buffer_and_pointer`: Executes a sequence of memory copies and updates a global variable with the result of a routine call.
- `update_structure_and_copy_data`: Executes a sequence of function calls that manipulate values in a structure, including clearing a value and copying data between buffers.
- `perform_multiple_function_calls`: Executes a sequence of four function calls that manipulate values stored in a structure, including clearing a value and copying data between locations.
- `execute_function_with_range_fill`: Calls a library function to create an object, fills a memory range with a constant, and then executes a routine using that object's pointer.
- `update_global_with_sum`: Calculates a sum involving a global variable and a value from a library call, then stores a bit-manipulated version of that sum into a global variable.
- `accumulate_global_value`: Performs a complex arithmetic loop that accumulates values from a global variable and other memory locations into a global variable.
- `return_uninitialized_stack_value`: This routine returns an uninitialized value from the stack.
- `increment_global_with_sum`: Calculates a sum involving a global variable and certain memory locations, then adds a repeated global variable value and that sum to a global variable.
- `initialize_system_state_2`: Resets several global variables to zero and executes a sequence of indirect function calls and table-based initializations.
- `execute_callback_list_callback`: Executes a routine or a sequence of routines retrieved from a table or a global pointer-based list.
- `call_wrapper_routine_2`: Calls another routine and returns.
- `load_and_call_function`: Loads two 32-bit values from a table into global variables and then calls a function pointer.
- `initialize_animation_parameters`: Updates a global pointer with a specific constant and executes an indirect routine call.
- `initialize_system_state`: Resets several global flags or counters and executes a sequence of library or internal routines before calling another routine with a global variable.
- `reset_global_flags_and_execute`: Sets certain global bytes to zero or one and executes a sequence of indirect routine calls.
- `trigger_replay`: Calls another routine with a specific address and a global variable.
- `process_multiple_table_entries`: Executes a sequence of three different routines using addresses derived from a global pointer and a constant offset.
- `reset_system_state`: Checks a global flag and, if zero, executes a sequence of routines and calls another routine with a global variable.
- `reset_state_and_execute_callbacks`: Checks a parameter, updates a global byte to zero, and executes two different indirect routines with specific offsets.
- `initialize_memory_tables`: Sets certain memory locations to specific constant values using a loop.
- `sum_table_bytes`: Iteratively calls a function to fetch values from a table and accumulates certain bytes into a local variable.
- `update_tables_with_calculated_offsets`: Writes certain values to a table, calculates and stores new pointer-based offsets into a table, and calls a routine with a pointer from a table.
- `initialize_game_state`: Resets several memory-mapped or global arrays to zero and configures a table of parameters based on a specific or previously set value.
- `check_and_set_table_flag`: Checks if certain values returned from routines are zero, then sets a flag in a table, calls a routine with two pointers, and clears the flag.
- `execute_sequence_of_function_pointers`: Executes a series of indirect function calls using addresses or offsets derived from a global pointer, and writes a constant value to a memory location.
- `execute_conditional_function_calls`: Checks a global variable and either executes a sequence of four function calls or a single different function call.
- `update_conditional_state_logic`: Updates a global byte based on a condition and executes a routine that stores a result in a global variable or a bit-masked value.
- `call_offset_routine`: Calls another routine using a value calculated from a pointer and an offset.
- `recurse_with_parameter`: Checks a parameter and calls itself recursively with that parameter and a global variable.
- `call_library_with_parameters`: Calls a library function using a pointer and certain values retrieved from a table.
- `update_conditional_table_entry`: Initializes a memory range to a constant value and then conditionally updates a table entry based on a comparison with a library function return value.
- `store_word_to_global_location`: Waits for a global value to reach zero and then stores a word from a parameter into a global location.
- `copy_word_from_pointer`: stores a word from a source pointer into a destination pointer.
- `recursively_call_routine`: Checks if a parameter is non-zero and, if so, calls itself with that parameter.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_table_data`: iterates through a loop to copy or update several values in a table based on conditional checks and pointer-based lookups.
- `update_table_index`: Searches a table for a valid entry and updates a table value with an incremented or decremented index.
- `update_table_entry`: Checks a condition based on a table value and, if certain criteria are met, updates a table entry with a value derived from a function call or a pointer-based calculation.
- `update_table_with_conditional_logic_2`: Iterates through a list and updates a table entry or modifies a byte in a table based on a conditional check.
- `update_table_with_conditional_logic`: Updates a table entry or modifies a byte in a table based on a conditional check while iterating through a list.
- `decrement_table_byte`: decrements a byte stored in a table and stores the result back to a location determined by an offset and a pointer.
- `update_table_index_and_value`: Increments a value in a structure and updates a table entry or modifies a byte in a table based on a conditional check.
- `initialize_array_bounds`: clears a range of memory and then populates a secondary table with index-based offsets.
- `initialize_memory_range_2`: Clears a range of memory and conditionally sets a value in a secondary table based on a previously cleared byte.
- `initialize_array_bounds_2`: Iterates through a range of memory, clearing certain bytes and conditionally updating a secondary table with new values or flags.
- `initialize_memory_range`: Clears a range of memory and then conditionally sets a value in a secondary table based on a previously cleared byte.
- `zero_word_at_pointer`: stores a zero word into a memory location via a pointer.
- `update_index_with_function_results`: iterates through a loop, calling a function via a pointer and storing the result or its negation into a table.
- `load_and_call_pointers`: calls two library functions, clears a table, loads several values from a library-referenced structure, and calls another routine for each loaded pointer.
- `execute_callback_list`: iterates through a table of pointers and calls another routine for any non-zero pointer, then calls two library functions.
- `process_recursive_parameters`: Executes a function call using a value from a pointer and then calls itself recursively with modified parameters.
- `dispatch_indirect_calls`: Calls three different routines using a pointer from a table, passing various combinations of provided parameters.
- `recursively_process_coordinate_offsets`: This routine recursively calls another routine while applying or iterating through a table of coordinate offsets.
- `execute_parameterized_calls_wrapper`: Invokes a routine via a pointer-table lookup, then calls another routine using a pointer to a structure and a set of or-ed or incremented or decremented parameters, before repeating the process with a different parameter or a decremented value.
- `loop_parameterized_calls`: Executes a loop that calls a routine with a pointer and calculated offsets, then calls a routine via a pointer with a single parameter.
- `execute_parameterized_calls_2`: Executes a loop that calls a routine with a pointer and calculated offsets or a single parameter.
- `execute_parameterized_calls`: Calls a routine via a pointer using a value from a table, then calls another routine using four values retrieved from a structure.
- `call_wrapper_routine`: calls a function via a pointer with three parameters, calls a second function via a pointer, and then calls another routine using the first parameter, the fourth parameter, and the result of the second function.
- `load_and_increment_members`: Loads a word from a memory location via a pointer and increments structure members.
- `increment_structure_members`: Increments two values in a structure within a loop that continues until a certain condition or a null terminator is met.
- `load_word_from_pointer`: loads a word from a memory location via a pointer into a register.
- `call_with_offset_parameters`: Calls a routine via a pointer using a single parameter, then calls another routine using four parameters where two are sums of input values.
- `call_via_offset_table`: Retrieves bytes from a table using an offset and calls a routine twice if a specific byte in the table is non-zero.
- `update_entity_attributes`: Reads attribute or offset values from a table and dispatches them to a series of subroutines.
- `update_table_data_2`: Extracts multiple values from a table using an index and offsets, then calls three different routines with these values and updated pointers.
- `call_subroutine_via_pointer`: Calls a routine through a pointer stored in a global table using a passed parameter.
- `execute_indirect_call_3`: Fetches a function pointer from a table using an offset from a base address and calls it with a parameter.
- `execute_indirect_function_call`: Fetches a function pointer from a memory location and calls it using a value passed as an argument.
- `clear_memory_buffer`: Uses nested loops to set a range of memory or a multi-dimensional array or buffer to zero.
- `add_to_table_value_2`: Updates a value in a table by adding a register value to a memory location, while also updating other table entries via pointer arithmetic.
- `add_to_table_value`: Calculates an address in a table using anindex and an offset, then adds a value from a register to the memory at that address.
- `update_global_state_and_table`: Updates a global variable and executes a sequence of indirect function calls or updates a table with conditional logic.
- `execute_conditional_logic`: Updates a global variable and executes a sequence of indirect calls using values or pointers retrieved from a structure.
- `update_state_and_recurse`: Updates a global variable with the return value of a routine, resets a flag, and calls a sequence of routines using values or pointers retrieved from a data structure.
- `execute_and_store_result`: Calls a routine, stores its return value in a global variable, and calls another routine using a concatenated value.
- `store_and_call_concatenated`: Calls a routine, stores a returned value in a global variable, and calls another routine using a concatenated 32-bit value.
- `update_global_state_2`: Updates a global variable and calls a routine using a pointer and a concatenated value or a specific constant.
- `update_global_state_3`: Updates a global variable and executes a sequence of indirect function calls using values or pointers retrieved from a structure.
- `update_table_values`: Performs complex updates to a table or array, including conditional logic and division-based calculations.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a2).
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a7).
- `recalculate_attribute_table`: Iterates through a table of structures to recalculate or copy attribute values based on a lookup or calculation involving a secondary table.
- `update_attribute_table`: Iterates through a table of structures to recalculate or copy attribute values based on a lookup or calculation involving a secondary table.
- `initialize_global_state`: Calls a routine with two arguments, stores the return value in a global variable, and then clears a small array of four or more words to zero.
- `call_routine_with_constant`: Calls a routine using a value stored in a global variable and a constant integer as an argument.
- `enable_dma_channel`: An 18-instruction routine that modifies the DMA controller register to enable a DMA channel.
- `update_dma_config`: Writes a value from a memory location to a hardware register and sets another hardware register to a constant value.
- `update_coprocessor_hardware`: Writes a longword and a word into hardware registers using an offset calculated from a provided parameter.
- `enable_dma_channels`: Sets a global variable to a value that enables DMA channels via the DMACON register.
- `write_hardware_registers`: Writes two or-sized values into a hardware register or a memory-mapped area using an offset derived from a parameter.
- `dispatch_id_based_logic`: Checks a value against a table and, depending on the value, executes different logic or updates global constants and pointers.
- `update_state_by_id_2`: Checks a register against several possible values and, depending on the case, writes certain constants to a global variable or modifies a pointer.
- `write_fixed_values`: Writes several constant values to a global variable and calls another routine.
- `modify_stack_and_hardware_registers`: Performs arithmetic on values stored at offsets relative to a5 and writes several constant words to offsets relative to a4 before branching to another location.
- `write_status_values`: Writes a word to an offset of negative 14 from a5, checks a word at a negative 5298 offset from a4, and if not equal to zero, writes a word to an offset of negative 14 from a5 and a word to an offset of negative 10 from a5 before branching.
- `enable_dma_channel_2`: Modifies a DMA controller register to enable a specific DMA channel and calls another routine.
- `return_empty_subroutine`: this routine is empty and performs no operations before returning.
- `update_state_by_id`: Checks a register against several possible values and, depending on the case, writes certain constants to a global variable or modifies a pointer before calling another routine.
- `return_no_operation_2`: The routine consists of a single instruction that returns immediately, performing no work.
- `clear_and_set_pointers`: Clears a range of memory, calls a library function, and stores return values from other routines into a global variable.
- `update_table_from_lookup_2`: Iterates through a range of indices to update a destination memory area using values fetched from a secondary lookup table.
- `update_table_from_lookup`: Iterates through a table to update a destination memory area using values fetched from a secondary lookup table.
- `call_function_conditionally`: Checks a global variable and calls a function pointer with a specific value if that variable is non-zero, then calls the same function pointer with a different constant.
- `decrement_and_update_counters_2`: Decrements a global variable and potentially updates a range-limited counter or resets a value if a condition is met.
- `update_pointer_and_call`: Calls a function pointer, calls another function pointer with zero, and copies a value from a memory location to a global variable.
- `reset_function_and_copy`: Calls a function pointer with zero and copies a value from a memory location to a global variable.
- `reset_state_and_call_subroutines`: Clears a value in a global memory location and then executes two previously stored function pointers, passing a zero or no argument to the first.
- `execute_function_pointer`: Calls a function through a pointer stored at a specific offset from a base address.
- `configure_dma_and_clear_flag`: Clears a global flag and calls a routine to enable a specific number of DMA channels, followed by a routine that writes a value to a memory location.
- `execute_indirect_calls_3`: Checks a global pointer and, if not null, calls the routine pointed to twice.
- `call_subroutine_with_pointer`: Calls another routine using a pointer loaded from a memory location.
- `process_numeric_callback`: Decomposes a parameter into individual digits, stores them in a local array, and iterates through a list of function pointers to execute certain callbacks based on the calculated digits.
- `process_conditional_callback_list_2`: Iterates through a list, updates a table with a constant value if a condition is met, and executes a function pointer if certain conditions are met.
- `process_conditional_callback_list`: Iterates backwards through a list of indices, updating a lookup table with a constant value if a condition is met, then executes a function pointer for any remaining non-matching entries using a calculated parameter.
- `iterate_and_call_on_condition`: Iterates through a list and calls a function pointer with a value from a memory location if a certain condition in a lookup table is met.
- `get_absolute_value`: Returns the absolute value of the provided parameter.
- `load_word_from_pointer_2`: Loads a word from a memory location pointed to by a register.
- `copy_byte_sequence_2`: Copies a sequence of bytes from one memory area to another using a loop and returns the number of bytes copied.
- `copy_byte_sequence`: Copies a sequence of bytes from one memory area to another using a loop and returns the final index.
- `traverse_linked_list`: Traverses a linked list, updating a global variable with a value from a structure member and calling another routine if a certain bit is set.
- `update_function_pointer`: Updates a global pointer with the return value of another routine.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `update_position_and_clear_byte`: Calculates a new coordinate using a pointer-based offset and calls a routine with the result, then potentially clears a byte in a buffer if a certain condition is met.
- `process_numeric_callback_2`: Decomposes a parameter into individual digits, stores them in a local array, and iterates through a list of function pointers to execute certain routines.
- `parse_string_and_update_state`: Copies a null-terminated or colon-terminated string into a global buffer and then updates a global state variable or executes a routine based on the result of a string-related lookup.
- `traverse_linked_list_2`: Iterates through a linked list of function pointers, updating a global variable with a value from the list and performing indirect calls.
- `return_constant_value`: Returns a constant integer value of one.
- `execute_indirect_call_10`: Retrieves a function pointer from a memory location and executes it using an indirect call.
- `execute_indirect_calls_2_2`: Performs an indirect call to a function pointer using a value retrieved from a memory location.
- `traverse_linked_list_and_execute`: Iterates through a linked list of function pointers, updating a global variable with the next node's value and executing the functions found at those addresses.
- `execute_indirect_calls_2`: Performs an indirect call to a function pointer using a value retrieved from a memory location.
- `return_constant_one`: The routine is empty and returns a constant integer value of one.
- `handle_state_transition`: Checks a parameter, calls a routine, and either updates a global variable or executes a library function and returns a status code.
- `handle_disk_swap_and_state_transition`: Checks a flag to determine if a disk swap or state change is required, then updates a global state variable and executes a sequence of conditional routine calls or an infinite loop.
- `call_indirect_functions`: Pushes four different words onto the stack and calls an indirect function four times with each word and a pointer from a4.
- `call_conditional_functions`: Pushes a word onto the stack and calls an indirect function, then compares a word at offset -13346 from a4 to 5 and, if not equal, pushes another word and calls an indirect function again.
- `execute_indirect_logic`: Pushes a word onto the stack, calls an indirect function, and then performs a series of operations including a jump based on a table lookup, clearing a word at a negative 15524 offset from a4, and comparing a word at a negative 13410 offset from a4 to 5.
- `write_table_values`: Iterates up to five times to write values from a table into a destination via a routine call, then resets a global variable if certain conditions are met.
- `call_indirect_function_2`: loads a function pointer from a memory location and calls it with a constant argument.
- `get_and_return_value`: Calls a routine and returns a value stored in a global variable.
- `initialize_game_constants`: Checks a global flag and, if non-zero, initializes a series of global or table-based parameters with constant or register-provided values.
- `initialize_global_values`: Sets several global variables or table entries to specific constant values.
- `decrement_and_update_counters`: Decrements a global variable if it is non-zero, and if it reaches zero, resets certain values; otherwise, it updates a range-limited counter using a clamped value.
- `conditional_execute_routines`: Checks several global conditions and, if satisfied, executes two different routines based on a returned value and a counter check.
- `configure_game_state_parameters`: Updates a global structure or memory block with specific constant or table-derived values, such as timer or score-related parameters, based on the value of an input parameter.
- `set_short_values`: Stores two specific constant values into a global variable via an offset.
- `set_short_values_alt`: Stores two different constant values into a global variable via an offset.
- `update_struct_members`: Calculates a value based on a global variable and updates several members of a structure with fixed or table-derived values.
- `update_struct_member_values`: Checks if a global value exceeds two, then updates several members of a structure or object using fixed values and data fetched from a table indexed by a value stored in another global variable.
- `set_struct_values`: Sets a global variable or a structure member to a constant value and optionally updates other members based on a comparison.
- `set_four_values`: Stores four constant values into a global variable via an offset.
- `update_struct_member_values_2`: Updates a structure or global memory block with constant or table-derived values based on the value held in a register.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `fill_buffer_with_count`: The routine iterates a specified number of times, writing the current loop counter value as a byte to a destination buffer and incrementing the pointer.
- `populate_buffer_from_source`: Iterates through a nested loop to call another routine and store the returned byte into a buffer.
- `iterate_and_copy_bytes`: Iterates through a loop, calling another routine and storing the returned byte into a buffer until a certain condition or end-of-string is met.
- `traverse_and_store_bytes`: Calls another routine to retrieve a value and stores it into a buffer.
- `copy_bytes_with_limit_check`: Iterates through a loop to copy bytes from a source obtained via another routine into a buffer.
- `clear_bit_and_call_routine`: Clears bit 3 of a longword at offset 12 of a2, tests bit 2 of the same location, and if set, calls a routine via a5 using a pointer from a2.
- `branch_to_subsequent_code`: Performs a short branch to a location within another routine.
- `calculate_difference_and_call`: Compares a word at offset 22 of a5 with 1, and if equal, calculates the difference between a pointer at offset 4 of a2 and a2 itself, clears offsets 4 and 0 of a2, and calls a routine via a5 using a byte from offset 13 of a2.
- `clear_and_branch`: Sets d0 to 0 and performs a short branch.
- `fetch_byte_and_add_pointer_offset`: Links a5, calls a routine using a byte from offset 13 of a structure pointed to by offset 8 of a5, and if bit 2 of offset 18 of that structure is set, calculates the difference between a pointer at offset 4 of a2 and offset 8 of a2 and adds it to the result of the previous call.
- `calculate_pointer_difference`: Checks if a2 is zero; otherwise, it calculates the difference between a pointer at offset 4 of a2 and a2 itself, stores this in d0, subtracts d0 from a value at offset negative 4 of a5, and returns.
- `get_and_increment_pointer_2`: Checks if a pointer or range is valid, increments the pointer, and returns the byte previously at that location.
- `get_and_increment_pointer`: Returns a byte from a pointer and increments that pointer.
- `clear_bit_and_traverse_list`: Clears a bit in a byte and conditionally traverses a memory-based list or structure.
- `modify_structure_and_traverse_list`: Clears a bit in a byte, then conditionally traverses a memory-based list or structure.
- `set_bit_in_byte`: Sets a specific bit in a byte located at an offset from a pointer.
- `update_pointer_and_calculate`: Updates a pointer, modifies an adjacent memory location, and returns a masked value derived from a calculation.
- `validate_and_return`: Pushes a2 onto the stack, calls another routine, and if the return value is not zero, branches to another routine; otherwise, it clears the stack, restores the stack pointer, and returns.
- `push_and_branch`: Pushes the value in a2 and two values from a structure in a5 onto the stack, calls another routine, restores the stack pointer, and branches to another routine.
- `call_and_test_pointer`: Saves registers to the stack, calls a routine using a pointer from offset 16 of a5, tests a byte at the address in a2, and branches to another routine if the byte is non-zero.
- `compare_and_modify_bytes`: Compares bytes from a table in a2 with a table in a1, and if they match and are non-zero, subtracts a byte from a1 and branches to another routine or increments a pointer in a2.
- `loop_and_call_routine`: Pushes two words onto the stack, calls a routine at offset 6320, and enters a loop that continues if the returned value in d0 is not equal to 65535.
- `modify_structure_bytes`: Loads a pointer from offset 16 of a5 into a0, writes the byte from d4 to offset 13 of that pointer, then writes the byte 1 to offset 12 of the same pointer and branches.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `find_char_in_string`: Searches for a specific byte within a string and returns a pointer to that byte or zero if not found.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `pop_return_save_d2_d3_a4_push_jump`: Pops the return address into a0, saves d2-d3 and a4 on the stack, pushes a0 back, and jumps to an absolute target.
- `execute_indirect_jump`: Performs an indirect call to a location stored in a jump table or register.
- `calculate_conditional_offset`: Fetches a word from offset 8 of a5, calls a routine, multiplies that word by 6, and uses the result to calculate an address by adding it to a value from a4.
- `process_io_request`: Calculates a value from a word at offset 14 of a5, pushes several values onto the stack including a longword from a2, calls a routine, and if the result is not negative one, calls an error handler and writes the result to a global variable.
- `call_seek_wrapper`: Clears two stack slots, pushes a longword from a2 onto the stack, calls a function at offset 25462 from the program counter, and then restores the stack pointer before branching.
- `call_subroutine_with_args`: Pushes two words and a pointer from offset 8 of a5 onto the stack, calls a routine via bsr, and then returns.
- `setup_and_call_routine`: Saves registers to the stack, loads a pointer from offset 8 of a5 into a2, calls a routine at offset 6e40, and then loads a value from a global variable into a3 before branching.
- `loop_and_compare_d4`: Multiplies d4 by 6, tests a value in a table using the result as an index, and if non-zero, enters a loop that increments d4 and compares it against a value from a4 until the condition is met.
- `execute_file_operations`: Tests bit 1 of a word at offset 12 of a5, calls several routines using a2 and a4 as arguments, and performs conditional branches based on the results of those calls and bit tests at offset 12 of a5.
- `perform_indirect_calls_2`: Pushes a value to the stack, calls a routine at a fixed address, moves the result to d6, and then branches to another location or calls another routine via the program counter.
- `perform_indirect_calls`: Pushes two values onto the stack, calls a routine at a fixed address, moves the result to d7, and if a condition is met, pushes d7 to the stack and calls another routine via a4.
- `perform_seek_operations`: Pushes two values onto the stack, calls a routine at a fixed address, pushes a third value, calls another routine at a fixed address, and then branches to another location.
- `close_file_if_flag_set`: Checks if bits 9 and 10 are set in a word at offset 12 of a5; if they are, it pushes d6 onto the stack, calls a function, writes 4 to an offset in a4, sets d0 to -1, restores registers, unlinks a5, and returns; otherwise it branches.
- `calculate_and_write_offset`: Multiplies d4 by 6, uses the result to index into a table pointed to by a3 to write a word from a5, and then conditionally calls a routine using a value from d6.
- `traverse_linked_structure`: Recursively traverses a linked structure and may store a constant or a returned value into a global variable if a bitmask condition or error occurs.
- `traverse_recursive_structure`: Recursively traverses a linked structure and stores a constant or a returned value into a global variable if an error or certain bitmask condition is met.
- `evaluate_recursive_read`: Recursively calls another routine using arguments passed via registers or memory, and if the result indicates an error, stores the result of a subsequent recursive call into a global variable.
- `nop_padding_sequence`: This routine contains no executable instructions and acts as a padding or gap in the code sequence.
- `process_table_entries`: Iterates through a table and, if certain conditions are met, updates a secondary list with new pointers and constant values.
- `update_table_entries`: Iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `manage_library_resources`: Interacts with library structures to check status, execute functions, and update a pointer in a table.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `clear_memory_and_call_function`: Clears a block of memory using a loop and then calls a function pointer retrieved from a global variable.
- `return_immediately_stub`: A two-instruction routine whose body is only a return, performing no work.
- `parse_string_tokens`: Parses a source string to extract or copy space-delimited or quoted tokens into a destination table.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_to_buffer_2`: Calculates the length of a source string and copies it to a destination buffer, ensuring a null terminator is placed at the end.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `traverse_and_update_pointers_2`: Traverses a linked structure and, if certain pointers are non-null, performs indirect calls to update structure members with table-derived addresses.
- `traverse_and_update_pointers`: Traverses a linked structure, calls a function pointer with a specific offset, and updates a global pointer or a table entry with a new address.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 3t-bit value.
- `apply_sign_to_result`: Performs a division-based routine and negates the result if the input or a flag indicates a negative sign.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `transform_and_divide_words`: Performs a complex sequence of bit-shifting, word-swapping, and division-based arithmetic on a 32-bit value.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `increment_pointer_and_write_byte`: Increments a pointer and writes a byte from a source address to the new pointer location, followed by a routine that checks a flag and updates a pointer or a flag.
- `increment_and_store_byte`: Increments a pointer stored in a register and writes a byte from a source address to the location pointed to by the incremented pointer.
- `link_and_call_subroutine`: Links a5, pushes a2 to the stack, sets up two pointers in a0 and a2 from a4, calls a subroutine via bsr, and then compares a pointer in a2 against a value from a4 before returning.
- `check_pointer_and_return`: Loads a pointer from offset 8 of a5 into a2, moves the value at that pointer into d0, and branches if d0 is non-zero; otherwise it sets d0 to 255 and returns.
- `execute_error_sequence_2`: Tests bits in a byte at an offset of a2, calls a routine via a brief branch, calls another routine using a pointer from a4, and performs several conditional calls using pointers from a2 before clearing several fields in a2 and returning.
- `update_pointer_with_value`: Checks a flag in a byte, then recursively processes a pointer or updates a pointer and a flag while writing a byte to a memory location.
- `process_recursive_buffer`: Recursively checks a condition or error state while updating a pointer, clearing a pointer and a following word, or writing a byte to a memory location.
- `modify_pointer_and_flag`: Modifies a bit in a byte, clears a pointer and a following word, or updates a pointer and a flag while writing a byte to a memory location.
- `update_pointer_and_flag`: Updates a pointer or a global variable and modifies a bit in a byte or writes a byte from a source address to a destination address.
- `update_pointer_and_set_flag`: Updates a pointer or a global variable, modifies a bit in a byte, and writes a byte from a source address to a destination address.
- `validate_pointer_range`: Sets up a stack frame, checks a byte at offset 12 of a structure in a2, and compares a pointer derived from a2 against a constant address; it returns if the comparison fails or if the byte is zero.
- `branch_to_offset_2`: Performs a short branch to an offset.
- `clear_pointer_and_offsets`: Clears a long value in a2 and the long values at offsets 4 and 8 from a2, moves a2 to d0, and branches to another routine.
- `update_structure_fields`: Updates fields within a structure, such as setting a pointer or modifying a bit in a byte, based on the return value of another routine.
- `modify_memory_attributes`: Writes a constant value to a memory location, modifies a byte via a bitwise or, and copies a value from a memory location.
- `load_and_branch_pointer`: Saves a5 and registers a2 and a3 to the stack, loads a pointer from a negative offset from a4 into a2, and branches to another routine.
- `loop_and_clear_global`: Stores a2 and a value from offset 4 of a2 into the stack, calls a routine at a negative 32112 offset from a4, and loops while the value in a2 is not zero, finally clearing a longword at a negative 20114 offset from a4.
- `insert_into_linked_list`: Sets a pointer to a global variable and, if a lookup routine returns a valid pointer, inserts a new node into a linked list.
- `set_pointer_to_table_entry`: Loads an address from a global offset into a pointer and sets a secondary pointer to a value from a table.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `update_pointer_from_indirect_address`: Links a5, saves a2 and a3 to the stack, subtracts a3 from a3, and loads a new pointer into a2 from an offset of a4.
- `copy_pointer_from_offset`: Loads a pointer from offset 8 of a5, compares it to a2, and if they are not equal, sets a3 to a2 and a2 to the value pointed to by a2.
- `copy_pointer_if_not_zero`: If d0 is not zero, copies the longword from a2 into the location pointed to by a3, then branches.
- `call_routine_with_pointer_offset`: Loads a pointer from a negative offset from a4, adds 8 to a value from offset 4 of that pointer, pushes the result and the pointer to the stack, calls another routine, and branches back to an earlier instruction.
- `validate_table_index`: Checks if a provided index is within a valid range and if a corresponding table entry is non-zero, returning a boolean or a constant error code.
- `check_interactive_status`: Checks a value at an offset within a table and returns a boolean result based on whether a specific routine returns non-zero.
- `return_zero_2`: The routine returns a zero value.
- `execute_error_sequence`: Sets up a stack frame, calls a routine using a pointer from a5, and if the result in d0 is not zero, calls another routine and writes d0 to a global variable.
- `zero_d0_and_branch`: Sets d0 to zero and branches to an earlier instruction.
- `read_recursive_structure`: Performs a recursive read operation that may update a global variable with a constant or a value from another routine if a bit or error occurs.
- `read_recursive_with_fallback`: Performs a recursive read operation and updates a global variable with a constant or a value from another routine if the read fails or a bitmask condition is met.
- `read_and_store_result_alt`: Executes a read operation and, if it fails, stores a value from another routine into a global variable.
- `return_without_action`: This routine consists of two instructions and performs no work: it immediately returns to its caller.
- `execute_conditional_indirect`: Calls a routine with specific arguments and, if a certain bit is set, potentially executes an indirect call via a pointer stored in a global variable.
- `execute_and_return_zero`: Calls another routine and then returns a zero value.
- `execute_write_sequence`: Calls a function via a pointer to process a global variable and then executes two subsequent routine calls.
- `execute_indirect_functions`: Checks a global pointer and, if non-zero, executes the routine pointed to by that address, followed by another indirect function call.
- `validate_and_clear_table_entry`: Checks if an index is within a valid range and if a table entry is non-zero, then clears the entry and calls a routine if a certain bit is not set.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `call_library_vector_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 6 of that base, returning its result to the caller.
- `call_library_vector_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 12 of that base, returning its result to the caller.
- `call_indirect_from_offset_table`: Loads a function pointer from an offset within a global table and executes it.
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
- `execute_indirect_from_global_address`: Executes a routine located at an address stored in a global variable.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `execute_indirect_function_9`: Loads a function pointer from a table or structure and executes it.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_seek_2`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_un_lock_2`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_pointer`: loads a function pointer from a table or structure and executes it.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `execute_indirect_function_10`: Loads a function pointer from a memory location calculated by adding an offset to a register and executes it.
- `execute_indirect_function_pointer_2`: Loads a function pointer from a table located at an offset from a global address and executes it.
- `execute_indirect_call_4`: Performs an indirect call to a location calculated by adding an offset to a pointer.
- `execute_indirect_call_with_dereference`: Loads a pointer from a table and calls the address found at a secondary offset.
- `execute_indirect_call_5`: Executes an indirect jump or call using a pointer-based offset.
- `execute_indirect_call_with_dereference_2`: Dereferences a pointer from a table and calls the resulting address with an offset.
- `execute_indirect_call_with_dereference_3`: Performs an indirect call using a pointer retrieved from a table and an offset.
- `execute_indirect_call_6`: Performs an indirect call to a destination calculated from a pointer and an offset.
- `execute_indirect_call_with_dereference_4`: Retrieves a pointer from a table and executes an indirect call to an offset address.
- `execute_indirect_call_7`: Executes an indirect call using an offset applied to a pointer.
- `execute_indirect_call_with_dereference_5`: Uses a pointer from a table to calculate a target address for an indirect call.
- `execute_indirect_table_call`: Loads a pointer from a table using an offset and calls the address stored at a secondary offset within that target structure.
- `execute_indirect_call_with_dereference_6`: Performs an indirect call to a target address derived from a table entry and an offset.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `execute_indirect_call_8`: Performs an indirect call to a function pointer stored in a global variable.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `execute_indirect_call_offset`: Loads a pointer from a global variable, applies a negative offset, and calls the resulting address.
- `execute_indirect_function_11`: loads a function pointer from a nested table or structure and executes it.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_function_12`: Loads a function pointer from a table located at a fixed offset from a base address and executes it.
- `execute_indirect_function_13`: loads a function pointer from a table located at a fixed offset from a base address and executes it.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_function_14`: loads a function pointer from a table located at a fixed offset from a global pointer and executes it.
- `call_library_vector_486`: Library stub: loads a library base from a global variable and calls the library function at offset minus 486 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_3`: loads a function pointer from a table or structure and executes it.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_indirect_function_15`: loads a function pointer from a table located at an offset from a global pointer and executes it.
- `execute_indirect_call_offset_2`: Loads apointer from a global variable, applies a negative offset, and calls the resulting address.
- `dereference_function_pointer`: loads a function pointer from a table or structure and executes it.
- `call_indirect_function_3`: Loads a function pointer from a table using an offset from a global pointer and calls it.
- `call_indirect_function_4`: Performs an indirect call to a function pointer retrieved from a table using a different offset.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `execute_indirect_function_16`: loads a function pointer from a table located at a fixed offset from a global address and calls it.
- `call_indirect_function_5`: Calls a function pointer calculated by adding an offset to a value retrieved from a table.
- `call_indirect_function_6`: Performs an indirect jump to a routine address found by indexing a table with an offset.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_indirect_function_7`: Executes a call to a routine address derived from a table entry and a large offset.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][a2](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

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
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_38`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_39`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_40`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_41`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_42`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_43`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_46`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_47`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_48`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_49`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_50`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_51`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_52`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_53`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_54`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_55`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_56`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_57`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_58`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_59`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_60`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_61`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_62`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_63`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_64`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_65`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_66`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_67`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_68`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_69`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_70`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_71`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_72`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_73`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_74`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_75`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_76`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_77`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_78`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
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
- `jump_vector_entry_79`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_80`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_34`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_35`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_81`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_36`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_82`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_83`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_37`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_copper_and_clear_memory`: Opens a library, copies a table of words into a memory block, clears a large memory region, waits for a hardware flag to clear, and closes the library.
- `check_and_service_interrupts`: Tests an interrupt flag bit in a global variable and, if set, checks a specific interrupt request bit.
- `write_next_table_entry`: Reads a byte from a pointer, wraps the pointer to a base address if the value is negative, stores a word built from that byte and a constant into a memory location, and increments the pointer by four.
- `build_frame_copper_list`: Computes per-frame display geometry from state fields and fills a copper-style list of word pairs in memory.
- `update_hardware_copy_parameters`: Waits for a hardware busy bit to clear, then initializes hardware registers with control words and address pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_calls_4`: Calls a routine to initialize memory blocks and then executes two subsequent routines via pointers stored in a table or memory location relative to a register.
- `update_memory_tables_2`: Uses a loop to store or modify bytes and words in several tables based on a value read from a global variable.
- `update_sprite_attribute_table`: Iterates through a range of memory to duplicate or update sprite or object attribute data, including updating or clearing associated word-sized or byte-sized metadata or secondary attribute buffers.
- `initialize_memory_blocks`: Sets several global variables or memory locations to specific values and clears a series of byte-sized memory locations using a loop.
- `clear_memory_blocks`: Clears or zeros out a series of word-sized memory locations using a loop.
- `update_memory_tables`: Copies word-sized values from a source table to a destination table, using an offset or a value from a global variable to determine the source address.
- `loop_and_write_bytes`: Calculates an address using a5 and a4, reads a byte, and enters a loop that writes bytes to an address calculated from a5 and a4, decrementing two counters in the stack frame until they reach specific limits.
- `initialize_string_buffer`: Allocates a memory buffer and initializes it with a specific string or header, then updates several global state pointers and flags.
- `release_game_resources_and_exit`: Frees the allocated memory blocks and loaded resources held in the global data area, conditionally closes a handle and a library, and, if a pending handle is set, closes it instead of performing the normal shutdown sequence, before returning.
- `process_callback_list`: Iterates through a list of function pointers and executes them, followed by a routine that passes a calculated size or pointer to another function.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_game_state_and_dispatch_logic`: Resets a series of global counters and flags, clears certain memory buffers, and executes a state-machine-driven sequence of function pointers.
- `update_sprite_attributes`: Iterates through a table of sprite or object descriptors to modify certain attribute bytes, such as color or certain flags, and calls a routine to update the display or position.
- `filter_register_range`: Extracts a byte from a register and checks it against a list of prohibited values, returning a calculated offset if the value is not in the prohibited list.
- `return_immediately_2`: The routine performs no operations and immediately returns.
- `return_immediately_3`: This routine is empty and executes no instructions before returning.
- `check_register_range`: Checks if a register value falls within a specific range of prohibited or otherwise handled values and returns a modified or original value.
- `empty_routine_7`: The routine is empty and performs no operations.
- `clear_global_byte`: Stores a zero byte into a global variable.
- `update_global_pointers`: Executes two function pointers and then copies values from a table or structure into several global variables.
- `call_subroutines`: Calls two subroutines via pointers, passing zero as anargument to the first one.
- `update_state_counters`: Updates or resets several global counters and pointers based on conditional checks of existing global values.
- `set_conditional_values`: Sets certain global bytes or words based on the value of a parameter and then executes a function pointer.
- `update_sprite_attributes_2`: Sets certain global words, then enters a loop that calls another routine and updates a global word with a modulo-reduced value.
- `call_with_table_value`: Executes a function pointer using a value retrieved from a table.
- `reset_and_configure_game_state`: Initializes a table with a constant value and resets several state-related flags or counters.
- `perform_attribute_transformations`: Checks a word at a negative offset from a4, then calls three different routines while pushing parameters onto the stack and clearing several word-sized values at various offsets from a4.
- `execute_dual_routine_calls`: Clears the stack, pushes 41 onto the stack, calls a routine, clears the stack, calls another routine, and then branches.
- `execute_transformation_sequence`: Calls iterate_and_update_attributes, calls transform_table_data, calls another routine, and uses a jump table based on d0 to branch to another routine.
- `fill_array_with_parameter`: Executes a function pointer and then loops to assign a parameter value and a constant to a table.
- `iterate_and_update_attributes`: Iterates through a range of an array to update a memory-mapped attribute or pointer and executes indirect calls if certain conditions are met.
- `update_entity_attribute_loop`: Iterates through a range of an array to update a memory-mapped attribute or pointer, and if a certain condition is met, updates a secondary table and calls two function pointers with a new coordinate or state.
- `transform_table_data`: Iterates through a table and scales or clears certain values based on a source table.
- `update_game_state_machine`: Updates a state-based logic controller by checking a value against a table and branching or updating state-related flags.
- `copy_from_table_offset`: Copies a word from a table or memory location using an offset derived from another memory location.
- `initialize_and_loop_clear`: Writes three words to specific offsets from a4, enters a loop that clears a word at offset 336 from an address calculated using a5 and a4, and repeats this eight times.
- `loop_and_decrement_value`: Calculates a value by subtracting 32 from a word at offset -20056 of a4, divides it by 8, and stores the result at offset -2 of a5.
- `clamp_and_copy_word`: Compares a word at a negative offset from a4 to 38; if it is less than 38, it sets that word to 42, copies the value from offset 42 to offset 40 at a negative offset from a4, and branches to another routine.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `perform_math_and_call_routines`: Calls three different routines using offsets from a4, performs arithmetic involving multiplication and subtraction on values from the stack, and stores a result in a stack offset before returning.
- `search_table_for_pattern`: Iterates through a table to find a specific pattern or value, returning a calculated index or pointer.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `modify_byte_from_table`: Subtracts 1 from a word at offset -17996 of a4, uses that value to index into a table at offset -31368 of a4, and writes the resulting byte to offset -20074 of a4.
- `compare_and_modify_value`: Compares a word at offset -20058 of a4 to 160; if it is less than or equal to 160, it branches to another routine, otherwise it adds 14 to a word at offset -20054 of a4, sets a word at offset -2 of a5 to the value in a3, and branches to another routine.
- `perform_multiple_calls_with_calculated`: Sets a word at offset -2 of a5, calls three different routines at negative offsets from a4 using various calculated values and stack parameters, and branches to an earlier instruction.
- `execute_until_zero`: Executes a function pointer repeatedly while certain global shorts are non-zero, then executes two more function pointers.
- `parse_string_delimiters`: Searches through a buffer for a specific character and replaces a delimiter character with a null terminator.
- `load_and_branch_d0`: Links a5, loads a word from an offset in a4 into d0, extends d0 to a longword, and branches.
- `execute_subroutine_and_unlk_2`: Pushes a byte from a4 to the stack, calls a routine via a pointer from a4, and then updates two bytes at offsets in a4 based on the result in d0 and a comparison.
- `calculate_and_branch`: Subtracts three from a word at an offset of a4, multiplies it by a constant, stores the result back to an offset in a4, and branches.
- `jump_via_table`: Compares d0 to 4, and if not lower, shifts d0 left by 1 and performs a jump to an address retrieved from a table using d0 as an index.
- `execute_subroutine_and_unlk`: Calls a subroutine using a brief branch and then unlinks register a5 before returning.
- `transform_and_iterate_attributes`: Pushes three words onto the stack and calls a routine, then pushes 24 and calls another routine.
- `calculate_and_call_routine`: Sets a word at offset -6 of a5, pushes it to the stack, calculates an index from a word at offset -2 of a5, loads a long from a table at offset -25348 of a4, pushes several values to the stack, and calls another routine.
- `loop_and_call_with_calculated_index`: Calculates an index using a value from offset -2 of a5, loads a long from a table using that index, calls another routine, increments the value at offset -2 of a5, and loops while that value is less than 4.
- `copy_player_sprite_attributes`: Copies or copies-from a source buffer or table to a destination buffer using a nested loop structure to duplicate or modify sprite or object attribute data.
- `find_byte_in_table`: Searches a table or buffer for a specific byte value and returns a value based on its position or presence.
- `check_conditional_bits_2`: Checks specific bits in a byte or word and returns a value from a memory location or a default value.
- `check_conditional_bits`: Checks specific bits in a byte or word and returns a value from a memory location or a default value based on those checks.
- `do_nothing_routine`: the routine is empty and performs no operations.
- `get_conditional_value`: Checks specific bits in a byte or bits in a word and returns a value from a memory location or a default value based on those checks.
- `load_unreferenced_value`: Loads a word from a memory location calculated using a register offset into a register.
- `update_index_with_bounds`: Links the stack frame, compares a word at offset -19128 of a4 to 3; if it is less than or equal to 3, it subtracts 4 from a word at offset -19128 of a4.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `update_global_index_with_bounds_check`: Compares a word at offset -8 of a5 to 1; if not equal, it adds 8 to a global variable at a negative offset from a4 and performs a bounds check to subtract 24 if the value exceeds 24.
- `xor_and_update_global`: Loads a word from a table using an index derived from a stack value, performs an XOR operation with a global word, stores the result back to that global word, and branches back to an earlier instruction.
- `update_game_state_logic`: Updates a global state machine by checking conditional flags, updating pointers, and swapping or clearing values in a global buffer.
- `extract_bit_index`: Calculates a bit index and a word index from a given offset.
- `update_buffer_offset`: Updates a global variable or pointer offset based on a calculated value and checks it against certain bounds.
- `update_index_and_clamp_value`: Updates a global index by scanning a table for duplicate adjacent values, then clamps a secondary global value within a fixed range.
- `update_global_values`: Copies several dwords or words from a source table or pointer-based memory to a set of global variables and clears two orset certain global variables to zero or one.
- `call_with_calculated_offset`: Calls a function pointer using a parameter and a value calculated from a table lookup and an offset-based calculation.
- `update_pointer_and_state`: Updates a pointer or a global word with a value from another memory location and conditionally sets certain global words based on a parameter.
- `clear_memory_and_assign`: Clears a range of bytes in a memory block, then calls another routine and stores its return value into a global variable.
- `execute_multiple_calls`: Executes a sequence of function calls, some of which pass a value retrieved from a global table or pointer-based memory.
- `set_constant_recursively`: Writes a constant value to a memory location and calls another routine recursively if a certain condition is met.
- `conditional_function_execution`: Calls a function, then checks if two global shorts are non-zero before calling another function that writes a constant to a memory location and passing a value from a global variable.
- `execute_set_constant`: Executes a function call, then calls another function to write a constant value to a global memory location, followed by a final function call.
- `return_empty_routine_2`: The routine contains no executable instructions and performs no operations before returning.
- `update_table_with_parameter`: Calls another routine with a constant, then performs a series of word-sized copies between a table and a global variable.
- `compare_and_call_routines`: Clears offsets 8 and 10 of a structure in a5 if they equal 64, then pushes several values onto the stack to call a routine, and potentially calls another routine based on a comparison with a value from a4.
- `branch_on_offset_value`: Pushes a word onto the stack, calls a routine, and then loads a word from offset -4b22 of a4 into d0 before branching to another location.
- `call_routine_with_text_and_parameters`: Pushes a word and a pointer to a string containing the text WEST VS EAST onto the stack, pushes values from a5 and other constants, calls another routine, and branches to an earlier instruction.
- `execute_triple_calculation`: Calculates a value based on a word from offset 8 of a5, calls a routine, performs arithmetic on the result, and then calls another routine using a value from offset 10 of a5.
- `execute_triple_calculation_branch`: Calculates a value based on a word from offset 8 of a5, calls a routine, performs arithmetic on the result, and then calls another routine using a value from offset 10 of a5, before branching to another routine.
- `branch_on_register_values`: Checks if d0 is zero, if it is one, or if it is seven, and branches to different locations based on these checks; otherwise it branches to another routine.
- `call_offset_routine_2`: Pushes a word onto the stack, calls a routine at a negative 32492 offset from a4, and returns.
- `update_buffer_from_table`: Clears a local buffer, then iterates through a range, reading values from a table and incrementing a byte in a buffer based on certain comparison logic.
- `compare_table_bytes`: Calls a routine using a pointer from a4, then compares a byte from a table at offset -19167 of a4 using indices from offsets -17782 and -17780 of a4, and performs a final test on a byte from the table.
- `push_data_and_call`: Moves two words from offsets -18134 and -18132 of a4 to offsets -2 and -4 of a5, pushes a pointer to a string to the stack, pushes a long from offset -25322 of a4 to the stack, calls another routine, and then branches to another routine.
- `compare_and_store_properties`: Compares a byte from a table at offset -4adf of a4 using indices from offsets -4576 and -4574 of a4.
- `process_series_and_properties`: Calculates an address using a word at offset -2 of a5 multiplied by 34, pushes it to the stack, and calls another routine.
- `process_series_data`: Calls a routine three times using various offsets and values from a5, performs a multiplication and subtraction, and then calls a routine with the result before branching to another routine.
- `update_object_property_conditionally`: Checks a condition or property via a routine and, if met, updates an object's attribute or state using a series of parameter-driven calls.
- `execute_with_parameters`: Calls a routine with certain parameters and stores the return value in a global variable, then conditionally sets a global flag.
- `set_conditional_flag`: Calls a routine multiple times with different parameters and sets a global byte to one if a certain condition is met.
- `perform_memory_copy_operations`: Calls a function to copy a range of memory, and if successful, performs further copies or checks using a function pointer and specific memory ranges.
- `sync_object_attribute_and_update_sprite`: Copies a longword or two dwords from a source object or table into a destination memory location, then conditionally updates a byte-pair or word in a sprite or attribute table using an offset-based calculation.
- `accumulate_table_values`: Iterates through a nested loop to add values from one table into another table at calculated offsets.
- `update_table_with_max_value`: Iterates through a loop to compare values in a table and updates a global table with a new value if it is larger, while also storing a secondary value.
- `copy_and_parse_buffer`: Copies several values from a source table to a destination, then parses a buffer and copies a range of memory if a certain condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: removes the stack frame with unlk, then returns to the caller.
- `find_buffer_marker`: Iterates through a buffer or table until a specific byte value is found, then calls another routine with the discovered offset.
- `populate_buffer_from_table`: If a certain condition is met, copies bytes from a table into a destination buffer using an offset, or otherwise checks a table and updates a local counter.
- `swap_array_elements`: Searches a table for a matching value and swaps two adjacent elements in that table if a certain condition is met.
- `map_buffer_indices`: Iterates through a buffer and, if a certain byte is non-zero, copies a byte from a source table to a destination table at a calculated offset.
- `return_no_operation`: The routine consists of a single return instruction with no other effect: it performs no computation, no hardware register access, no library calls, and no memory writes.
- `empty_c_function_5`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `initialize_and_reorder_attribute_table`: Clears a 2D memory buffer and then reorders or copies entries from a source table into a new destination table based on a conditional check of a value.
- `compare_and_recurse_table`: Iterates through a table and calls another routine if certain adjacent or nearby values in the table match.
- `compare_table_indices`: Compares values in a table using indices and calls another routine if the values do not match or if a certain comparison fails.
- `reorder_table_by_value_comparison`: Clears a counter array, then iterates through a range of indices to compare values in a table and increment counters if certain conditions are met.
- `swap_table_values`: Reads two word-sized values from a table and writes them back to a different index in the same table, effectively swapping or overwriting values.
- `update_table_with_sum`: Calculates the sum of two short values from a table and passes this sum, along with a pointer and a constant, to a series of called subroutines.
- `process_pixel_value_differences`: Clears a memory buffer and then iterates through a table of byte values, comparing adjacent bytes and updating or decrementusing values in a separate memory area based on the comparison results and a call to another routine.
- `find_matching_differences`: Iterates through a table and compares two values, storing an index and a difference into a destination table if a match is found.
- `modify_memory_via_pointer`: The routine performs a sequence of instructions that modifies a value in memory using a pointer held in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine`: Calls a subroutine at a fixed offset.
- `call_subroutine_at_offset_1f0`: Calls a subroutine located at offset 1f0 from the start of the executable.

### paranoimia#unpacked

- `initialize_system_state_3`: Calls a routine via a pointer in a global variable, performs a loop to copy 16 words from one memory location to another, clears a memory range starting at a specific address, and performs several library calls using a pointer from a global variable.
