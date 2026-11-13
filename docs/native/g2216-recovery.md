# Game 2216 (tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02216). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1827140 top-level bytes in 60 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 32 of 32 units identical (bytes and relocations), 23 raw-word lines |
| Functions named and described | pass, 965 of 965 named (0 author symbols), 965 of 965 described (229 by rule, 717 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4379 of 4383 (99.91%) |
| Recovered without load address (counted in proof 1, flagged) | 596969 bytes, 29 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16867 of 16886 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 628006 |
| code | 67102 |
| data | 876633 |
| padding | 14759 |
| unused | 240640 |

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
| micro3 | 4 | 127 | 8 |
| paranoimia | 2 | 83 | 3 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:dawn | 1 | 155 | 3 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:bb | 3 | 8490 | 446 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:XC2.1 | 1 | 99 | 3 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:bootblock | 1 | 11 | 1 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 120 | 120 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 492 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2893 | 90 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1076 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 3 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:dawn#unpacked | 1 | 1025 | 29 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:Fluff.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 183 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 520 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 711 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 2 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 92 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:S/paranoimia: ByteKiller 1.3 (xfd) stream found by ancient scan (6920 -> 12322 bytes) but only 0 of 5 sampled windows are in reference RAM: not used
- tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:dawn: crunched executable unpacked with ancient scan (DefJam Cruncher 3.2 (xfd), 48880 -> 196608 bytes), 14 of 35 windows confirmed in reference RAM, loaded at an address: recovered as u28
- tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:XC2.1: TetraPack 2.2 (xfd) stream found by ancient scan (42108 -> 65536 bytes) but only 0 of 22 sampled windows are in reference RAM: not used
- tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:Fluff.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate
- tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:dawn: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 4 never-loaded region(s), 596969 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### More

- `initialize_game_resources`: Initializes the game engine by restoring execution state, loading file metadata, and parsing a configuration string or file header to populate internal structures with pointers to string data and memory-resident assets.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `process_entity_state_and_buffer`: Updates an entity's state or type and conditionally clears or modifies a memory buffer.
- `dispatch_entity_actions`: Checks a longword at offset 60 of a structure in a2 and, if not zero, calls dispatch_by_flag and update_entity_and_increment_counter, or alternatively calls draw_text_window_range and another routine depending on various conditions and branches.
- `execute_recursive_calls_2`: Calls a routine passing a pointer to a global variable, then calls itself, and finally executes a function pointer stored in an unrelated address register.
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
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition or partial way is met, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `recursively_adjust_buffer_offset`: The routine recursively traverses a structure or buffer, checking if a size field or a boundary value matches a specific constant.
- `update_scroll_window`: Updates a scrolling window structure by calculating a target position using provided offsets and checking it against current window bounds.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_uppercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a unable-to-determine source and a provided mask.
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
- `swap_and_call_modular_inverse`: Moves d2 to the stack, swaps d0 and d1, calls another routine, and restores d2 from the stack before returning.
- `calculate_modular_inverse`: Manipulates d0 and d1 using negation and bitwise NOT operations, then calls modular_inverse_32bit with a pointer in a0, finally returning a value in d2.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_pointer`: Stores a byte from a register through a pointer.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a certain value before dispatching it.
- `delete_port`: Removes a port from a global list if a specific field is non-zero, sets status fields to indicate deletion, and releases the associated memory block.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: Copies a fixed number of words from a source table to a stack-based destination before calling another routine.
- `update_structured_data_3`: Performs a large sequence of conditional checks and calls to various routines using pointers and indices derived from a1 and a0, while updating several longwords in a2 and a1.
- `branch_short`: Performs a short relative branch.
- `write_constant_to_offset`: Sets d1 to negative one, stores it in a global variable at offset 676 of the structure in a2, and branches to another routine.
- `write_constant_to_offset_a1`: Sets d1 to negative one, stores it in a structure in a1 at offset d4, and branches to another routine.
- `calculate_and_call_function`: Calculates a value by adding a value from a structure in a1 to a shifted version of another value from a1, fetches a byte from a table using this value, and calls a routine using a pointer from a2 via a5.
- `fetch_and_compare_byte`: Calculates an index by adding a long value from offset 544 of a1 to a shifted version of a long value from offset 548 of a1, fetches a byte from a table at a0, calls a subroutine at a5, and compares the result to 72 or 78.
- `update_structured_data_2`: Increments a longword at offset 224 of a structure in a1, then performs several conditional checks and calls a routine using a pointer from a2 and a constant in d0.
- `execute_sequential_calls`: Stores 1 in a global variable, calls a routine using a pointer from offset 216 of a structure in a2, and performs several subsequent conditional calls to routines using pointers from offsets 220, 148, 248, and 292 of a2, finally storing 5 in a global variable at offset 632.
- `branch_to_offset_696`: Performs a short branch to a location 696 bytes ahead in the code.
- `call_indirect_with_constant`: Loads the constant 232 into d0, loads a pointer from offset 336 of a structure in a2 into a4, calls the routine at a5, and jumps to the address in a6.

### Assign

- `initialize_resource_tables`: Iterates through a linked list or table to find a maximum value, then populates a secondary table with an incrementing sequence of integers.
- `copy_resource_block_3`: copies a fixed-size block of sixteen bytes from a memory offset to the stack.
- `assign_structured_fields`: Performs bitwise operations on a4, updates several longword fields in a1 using an index from d1, and calls multiple routines using pointers from a2 and constants in d0.
- `update_structured_data`: Updates multiple longword fields in a1 using values from a table at a0 and performs several calls to a routine using pointers from a2 at offsets 428, 296, and 352.
- `assign_table_values`: Copies a value from a table at a0 into offset 496 of a1, calls a routine using a pointer from a2 at offset 308, and if d1 is zero, performs further checks and calls a routine using a pointer from a2 at offset 296.
- `update_structure_fields`: Performs multiple calls to routines using pointers from offsets 436, 120, 352, 340, and 292 of a2, while updating various longword fields in a structure in a1 based on results and table lookups.
- `update_structure_and_call_functions`: Manipulates several longwords in a structure pointed to by a1 using offsets and bit shifts, performs multiple calls to a function pointed to by a5 using various parameters and offsets from a2 and a4, and finally jumps to a location in a6.

### EndCLI

- `initialize_engine_metadata_2`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with a sequence of incrementing values.
- `copy_resource_block_4`: copies a fixed-size block of 16 words from a memory offset to the stack and then executes a stored function pointer.
- `execute_command_sequence`: Performs a series of operations involving bitwise manipulation of a value in a2, multiple calls to a function via a5 with various parameters in d0 and a4, and conditional logic that updates a longword in a table and decrements a value at an offset of a0.

### Dir

- `initialize_engine_metadata_3`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with a sequence of incrementing or fixed values.
- `copy_resource_block_5`: copies a fixed-size block of 16 or 17 words from a memory location offset by a pointer to the stack.
- `process_directory_calls`: A routine that performs several indirect calls via a5 and modifies a longword in a1 based on values in structures pointed to by a1 and a2.
- `perform_indirect_calls`: Checks if a longword at offset 364 of a1 is zero; if not, it loads three longwords from offsets 8, 48, and 72 of a1 into d1, d2, and d3, sets d0 to 464, and calls a routine using a5 and a modified a4 before storing d1 at offset 352 of a1.
- `clear_offset_value`: Tests a longword at offset 384 of a1; if it is not zero, it clears it and otherwise it returns immediately.
- `execute_indirect_directory_calls`: Loads a longword from offset 8 of a1, sets d0 to 464, and performs four indirect calls via a5 using modified values in a4.
- `execute_indirect_calls_2`: Performs several indirect calls via a5 using values derived from a structure in a1 and a table in a0, and modifies a byte in a table based on a comparison.
- `execute_indirect_calls`: Performs multiple operations involving shifting values, comparing them against a table, and making several indirect calls via a5 with various parameters and offsets.
- `update_directory_state_2`: Checks several longwords in structures pointed to by a1 and a2, performs indirect calls via a5, and modifies a longword in a1.
- `calculate_offset_and_call`: Adds a longword from a structure to 2, shifts the result right by 2, loads a pointer from a global address, and calls another routine before branching.
- `update_directory_state`: Calls a function via a5 with d0 set to 368 and a4 pointing to a value from a global address, decrements a longword from a1, calls the same function again with a modified a4, and if the decremented value is not zero, it updates several global words and jumps to a location in a6.

### Echo

- `initialize_engine_state_2`: Iterates through a pointer-based table to find a maximum value, then populates a memory-mapped or global table with an incrementing sequence of integers.
- `copy_resource_block_6`: copies a fixed-size block of sixteen bytes from a memory location or table to the stack and then executes a previously stored function pointer.
- `dispatch_table_calls`: Performs an OR operation on a longword in a4, stores a scaled value from d1 into a1, and calls a routine via a5 twice using different offsets from a2 and different values in d1.
- `update_table_entries_2`: Uses a value from a1 to index into a table at a0, calls a routine via a5, stores the result in a1 at offset 328, and then performs several more table lookups and calls to a5, eventually updating offsets 332 and 336 of a1 and potentially updating offset 328 again.
- `copy_longword`: Copies a longword from offset 336 of a1 to offset 332 of a1 and then branches.
- `calculate_index_offset`: Sets a longword at offset 328 of a1, then calculates a value by adding offsets from a1 and compares it to a value at offset 336 of a1, branching based on the result.
- `move_and_branch_longword`: Copies a longword from offset 150 of a1 to offset 14c of a1 and then branches to another location.
- `dispatch_conditional_call_2`: Moves longwords within a structure in a1, performs an indirect call via a5 using a value from a table, and branches to another location.
- `dispatch_conditional_call`: Shifts a longword from a1 left by 2, checks a value at an offset in a table, and if that value is not zero, calls another routine before branching.
- `dispatch_echo_request`: If a longword at a global offset is not zero, it calls a function via a5 twice with different d0 and a4 values, and if the second call returns a non-zero d1, it calls the function a third time with d1 set to zero and a specific a4 value before jumping to a6.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Execute

- `initialize_execution_environment`: Calculates the maximum size of a linked list or table, populates a memory-mapped or global table with an incrementing sequence of values, and copies a fixed block of data from a source table to a destination buffer while setting a function pointer for a subsequent routine.
- `copy_stack_data_2`: Checks if a pointer is non-null and copies a fixed number of words from a source offset to a destination pointer.

### Copy

- `reconstruct_engine_state`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with a sequence of incrementing values.
- `copy_stack_data_3`: Copies a block of data from a source address to a destination pointer if the destination pointer is not null.
- `process_conditional_data`: Performs arithmetic on a value in a1, clears several longwords at offsets 532, 680, 684, and 688 of a2, and calls a routine twice using different parameters based on a byte check from a table.
- `copy_bytes_from_table`: Calculates indices using values from a1 and a2 to fetch longwords and bytes from a table in a0, then stores the fetched bytes into offsets 544 and 548 of a1 before branching.
- `execute_dual_calls`: Calls a routine twice using values derived from offset 540 of a1 and offset 536 of a4, then branches back to an earlier instruction.
- `execute_conditional_calls_2`: Calls a routine twice using values derived from offset 540 of a1 and offset 536 of a4, but only performs the second call if the value at offset 548 of a1 is not zero, then branches back to an earlier instruction.
- `compare_and_branch_constant`: Compares a longword from offset 544 of a1 with several constants, branching to different locations if matches are found for 35, 37, 40, or 124.
- `compare_and_jump`: Increments a longword at an offset of a1, compares it to a value at an offset of a2, and if the comparison fails, performs a series of checks on a byte from a table before jumping to the address in a6.
- `check_and_jump`: Tests a longword at an offset of a2; if it is non-zero, it branches to another routine, otherwise it clears d1 and jumps to the address in a6.
- `update_and_index_table`: Increments a longword at offset 532 of a1, uses a scaled index to read a byte from a0 into d2, stores d2 in a2, and performs several other data movements and table lookups before branching.
- `branch_to_address`: Performs a short branch to a fixed address.
- `copy_and_call`: Calculates an offset using a value from a1 and a table in a0, compares a byte from that table to a value in a2, and performs an indirect call via a5.
- `compare_longword_against_constants`: Compares a longword from offset 552 of a1 against several constant values, branching to different locations if matches are found for 35, 37, 39, 40, 63, or 124.
- `increment_and_branch`: Adds a longword from a structure at offset 544 to 1 and branches to another location.
- `branch_to_offset_3`: Performs a relative branch to a specific offset.
- `compare_and_branch`: Tests d1, and if it is zero, stores a constant in a structure at offset 512 of a2 and branches to another location; otherwise it branches to another location.
- `compare_and_offset_data`: Moves a longword to an offset in a structure, compares it to 1, and if not greater, calculates a new offset using a global value and a shift, then compares a value at that offset to a register before branching.
- `copy_longword_with_offset`: Increments a global word, adds that value to a longword from another global location, shifts the result left by 2, and copies a longword from the address in a1 to the address in a0 plus that calculated offset.
- `compare_and_set_byte`: Compares a long value from offset 704 of a structure in a2 with a long value from offset 704 of a structure in a2, and if the first is greater, sets a byte at offset 700 of the structure in a2 to 255 before branching.
- `store_byte_from_table`: Increments a global word, calculates an index by adding that word to a value from a global location and shifting it left by 2, then moves a byte from the address in a0 plus that index into a global word.
- `call_handler_if_match`: Compares d1 with a global word; if they match, it calls a function via a5 twice with d0 set to 12 and a4 pointing to an offset from a4, then jumps to a6.
- `store_and_call`: Stores longwords from a2 into a1, then performs an indirect call via a5 and branches to another location.
- `update_pointer_and_jump`: Sets a long value at offset 44 of a structure in a2 to negative one, loads a long value from a1 into d1, and jumps to the address in a6.
- `call_subroutine_twice`: Calls a subroutine at a5 with a4 offset by 0, then calls it again with a4 offset by 164, and finally jumps to the address in a6.
- `execute_conditional_calls`: Calls a subroutine at a5 with a4 offset by 132, stores the result back to a1, and if a value at offset 700 of a2 is not 41, sets a value at offset 712 of a2 to negative one and calls a subroutine at a5 with a4 offset by -28 before jumping to a6.
- `branch_on_value`: Compares a long value from offset 4 of a1 against several constants, including negative one, 35, 40, 41, and 124, branching to different locations depending on the match.
- `dispatch_offset_calls`: Clears offset 4 of a1, calls a routine via a5 with d0 set to 20, and stores the result in offset 8 of a1.
- `calculate_offset_and_call_2`: Calculates an offset using a value from a2 and a1, stores a byte from a2 into a structure at a0, stores a longword from a2 into a1, and performs an indirect call via a5 before branching to another location.
- `call_with_offset_address`: Loads a long value from offset 8 of a1 and a long value from offset 704 of a2, sets d0 to 24, loads the address at offset 128 of a4 into a4, and calls a subroutine at a5.
- `branch_immediately`: Performs a no-operation and then branches to another location.
- `loop_byte_copy_with_offset`: Enters a loop where it calculates an offset using a value from offset 696 of a2 and a pointer in a1, reads a byte from a0 at that offset, stores it in offset 8 of a1 and offset 7 of a1, and repeats if the value at offset 8 of a1 is not zero.
- `branch_and_copy_long`: Stores a longword from d1 into offset 8 of a1, then branches to another routine if d1 is not zero, otherwise it moves d2 into d1 and jumps to the address in a6.
- `branch_to_loop_byte_copy_and_store`: Performs a short branch to a location within loop_byte_copy_and_store.
- `loop_byte_copy_and_store`: Enters a loop that reads bytes from a0 at an offset derived from offset 696 of a2 and a1, stores them into a1, and repeats while the byte in d2 is not zero; it then performs one final byte write to a0 at the calculated offset and jumps to a6.
- `clear_and_update_table`: Clears several long values at offsets 704, 708, 712, and 720 of a structure in a2, then enters a loop that reads a byte from a table at a0 using an index derived from d1 and d2, and writes it to offset 8 of a1 and offset 716 of a2.
- `call_multiple_subroutines`: Calls a subroutine at a5 three times with different offsets in a4, then performs a bitwise NOT on a value from offset 712 of a2 and jumps to a6.
- `copy_bytes_with_offset`: Reads two bytes from a table at a0 using indices from d1 and d2, stores them into offsets 8, 12, and 16 of a structure in a1, and then enters a loop that reads and writes bytes to a0 using offsets from a1.
- `add_and_store_byte`: Adds the values at offsets 8 and 12 of a1, then stores the resulting byte into a0 at an offset determined by shifting the value at offset 12 of a1 left by two bits.
- `copy_and_call_routine`: Copies bytes between locations in a0 using offsets from d1 and d2, then enters a loop that calls a subroutine at a5 using a pointer from offset 300 of a2, and performs byte writes to a0 based on values in a1.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `call_multiple_subroutines_2`: Calls a subroutine at a5 using a pointer from offset 284 of a2, and if the returned value in d1 is not zero, calls two more subroutines using pointers from offsets 468 and 536 of a2.
- `write_long_to_offset`: Calculates an offset by shifting a value from offset 4 of a1, then writes a long value from offset 27c of a2 to a0 at that offset, and finally writes a long value from a1 to a0 at a similar offset.

### Makedir

- `initialize_engine_state_3`: Iterates through a pointer-based table to find a maximum value, then populates a memory-mapped or global table with an incrementing sequence of integers.
- `copy_stack_data_4`: Performs a loop that copies a sequence of words from a memory offset to a stack-based pointer.
- `execute_system_calls`: Calculates a value from a1, clears offsets 336 and 338 of a1, and performs a sequence of calls to a subroutine at a5 using pointers from a2 and various values in d0 and d1.
- `perform_multiple_system_calls`: Executes a sequence of calls to a subroutine at a5 using various pointers from a2 and values from a1, performing conditional logic based on the results of these calls and updating values at offsets 208, 212, and 216 of a1.

### CD

- `initialize_engine_state`: Iterates through a table to find a maximum value, then populates a memory-mapped or global table with an incrementing sequence of integers.
- `copy_resource_block`: copies a fixed-size block of sixteen bytes from a memory offset to the stack.
- `process_multiple_tables`: Increments d0 and a3, updates several pointers in a1 using offsets 212 and 444, clears offset 608 of a2, and performs multiple calls to a routine using pointers from a2 at offsets 204, 312, 292, and 336.
- `process_table_entries`: Calls a routine using a pointer from offset 292 of a2, then calls a routine using a pointer from offset 336 of a2, and if a value at an offset in a table indexed by a shifted value from a1 is non-zero, it calls a routine using a pointer from offset 492 of a2 and another using a pointer from offset 296 of a2.
- `execute_multiple_system_calls`: Performs a series of calls to a routine in a5 using various offsets from a2 and a4, including loading values from a1 and a2, and ends with a jump to a6.

### EndIf

- `return_zero`: The routine is empty and simply returns a zero value to the caller.

### If

- `initialize_engine_metadata`: Traverses a linked list or table to find a maximum value and then populates a memory-mapped or global table with an incrementing sequence of or encoded values.
- `copy_resource_block_2`: copies a fixed-size block of sixteen bytes from a memory location offset by a base pointer into a destination stack or memory area, then executes a previously stored function pointer.
- `execute_complex_logic`: Performs a sequence of operations including a call to another routine.
- `compare_scaled_index_byte`: Scales a longword from a1 by four, checks a byte at the resulting offset in a0, and branches if that byte is not greater than a value at offset 224 of a2; otherwise it branches to another location.
- `check_table_value`: Loads a longword from a1, scales it, and checks a byte at an offset in a0; if the byte is not greater than a value at offset 224 of a2, it sets d1 to negative one and branches.
- `process_table_entries_2`: Extracts several longwords from a table in a0 using indices from a1, performs byte comparisons against a constant, and calls routines using pointers from a4 and values from a1.
- `process_multiple_table_checks`: Calls a routine using a pointer from a2 and a4, performs several checks against bytes in a table at a0 using scaled indices, and branches based on the results.
- `branch_to_offset`: Performs a relative branch to an offset.
- `compare_and_branch_offset`: Loads a longword from a1, scales it, and checks a byte at offset 18 of a0; if the byte is not zero and the longword at offset 164 of a1 does not equal 1, it sets d1 to negative one and branches.
- `branch_to_offset_2`: Performs a short branch to an offset.
- `compare_and_branch_4`: Compares a longword at offset 388 of a1 with d1, and if equal or if the longword is zero, it branches; otherwise it sets d1 to negative one and branches to another location.
- `process_table_values`: Checks a byte at offset 36 of a table at a0 using an index from a1, calls a routine with constant 276, and if the result is non-zero, calls another routine with constant 280.
- `check_and_call_handler`: Stores zero in offset 328 of a1, checks a value in a table at a0 using an index from a1, and if certain conditions are met, clears offset 364 of a1 and calls a routine using a pointer from a2 at offset 244.
- `execute_multiple_calls_and_logic`: Performs a series of calls to a routine via a5 using various values in d0 and pointers in a4, updates several fields in a structure in a1, and includes a loop that repeats if d1 equals 32.
- `loop_until_condition`: Calls a routine using a pointer from offset 216 of a2, then enters a loop that calls the same routine until d1 matches a value from offset 600 of a2, and if certain conditions in a structure in a1 are met, it performs additional calls using pointers from offset 8 of a2 and offset 1180 of a4.

### IconX

- `initialize_game_environment`: Initializes the game engine by loading file metadata, allocating memory, and parsing a configuration string or file header.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer, then conditionally invokes several cleanup callbacks.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a certain character.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a 260-byte buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `join_strings_into_buffer`: Copies one string into a destination buffer and joins a certain string onto it, then passes the buffer and the combined length to a further helper.
- `count_string_length`: Receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to another routine.
- `parse_signed_decimal_2`: Converts a null-terminated string representing a signed decimal integer into a signed integer value.
- `execute_supervisor_call`: Executes a routine called via a pointer stored in a global variable.
- `exit_interrupt_handler`: Executes a routine called via a pointer stored in a global variable and references an interrupt exit function.
- `schedule_task_handler`: Executes a routine called via a pointer stored in a global variable and references a task scheduling function.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `dispatch_handler`: Executes a routine called via a pointer stored in a global variable and references a dispatch function.
- `handle_exception`: Executes a routine called via a pointer stored in a global variable and references an exception handler.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `enable_interrupts`: Executes a routine called via a pointer stored in a global variable and references an interrupt enable function.
- `allocate_memory_handler`: Executes a routine called via a pointer stored in a global variable and references a memory allocation function.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `allocate_entry_handler`: Executes a routine called via a pointer stored in a global variable and references an entry allocation function.
- `invoke_recursive_string_compare`: Calls another routine passing two parameters and a pointer to a specific stack location, which is used by a string comparison routine.
- `execute_library_call_a`: Executes a routine called via a pointer stored in a global variable and references a library function.
- `execute_library_call_b`: Executes a routine called via a pointer stored in a global variable and references a different library function.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `execute_indirect_function`: Loads a function pointer from a calculated offset in a global variable and executes it.
- `execute_indirect_function_2`: Retrieves a pointer from a global variable using a different offset and jumps to that address.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found, and passes the buffer and checksum to another routine if the inverted checksum is non-zero.
- `store_byte_to_pointer_2`: Stores a byte from a register into a memory location pointed to by a register.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_register_to_pointer`: stores the contents of a data register into a memory location pointed to by a register.

### Disk-Validator

- `format_string_and_initialize_table`: Converts an integer to a string and stores it in a buffer, then initializes a memory-mapped table or array with bitmask-based or constant values using a loop.

### micro3

- `initialize_program_entry`: Performs initial data unpacking and file metadata loading.
- `copy_and_reconstruct_bitstream`: Copies a block of memory from a source address and reconstructs or unpacks bit-shifted data into a destination buffer.
- `shift_and_mask`: Reads a value from the stack, stores a modified version of it into a memory location, and returns a bit-shifted version of the original value.
- `perform_modular_reduction`: Performs a loop that shifts a value right and calls another routine if the value becomes zero, while decrementing a counter.
- `call_library_entry`: Invokes another routine and checks the zero flag, then calls a second routine and executes a function pointer from a global table before calling a third routine.
- `open_dos_library`: Calls the exec library function OldOpenLibrary to load dos.library and stores the resulting library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### paranoimia

- `decompress_and_verify_stream`: Unpacks a compressed bitstream by reading bits and copying back-referenced runs backward in memory, then verifies a checksum.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:dawn

- `initialize_and_copy_memory_block`: Initializes a block of memory by copying data from a global source and then performs a complex, recursive bit-shifting and conditional copying or overwriting of memory or a table using a counter-based loop.
- `shift_and_store_word`: Shifts a 16-bit value right and stores the result into a memory location.
- `shift_and_recurse`: Recursively shifts a register right and calls itself if the register becomes zero, while decrementing a counter.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:bb

- `initialize_and_clear_memory`: Clears a block of memory using a loop and then executes a function pointer retrieved from a global variable.
- `update_string_metadata`: Compares two values and, if different, copies a sequence of words from a source to a destination while updating a string or buffer with a length and certain characters.
- `swap_linked_list_nodes`: Checks a pointer against a global value, swaps two adjacent pointers in a linked list, and updates a global pointer to point to the new head or a node.
- `process_list_and_set_timer`: Iterates through a list, calls a function for each non-zero entry, clears those entries, and calls a library function with a fixed value if a certain pointer is non-zero.
- `update_buffer_and_linked_list`: Updates a buffer or table entry with a sequence of incrementing values and modifies a linked list by swapping or updating pointers.
- `copy_data_table`: Copies thirty-two words from a table into a destination buffer pointed to by a parameter.
- `update_buffer_recursively`: Recursively updates a buffer or a certain table entry with a value from a parameter, or updates a pointer-based destination if a certain condition is met.
- `recurse_and_increment_counter`: Recursively calls another routine while incrementing a counter and decrementing a value until it reaches zero.
- `update_table_recursively`: Recursively updates a table by incrementing a counter and modifying values through a pointer until a termination condition is met.
- `compare_and_branch_3`: Compares a word at offset 12 of the stack frame in a5 with 98 and branches if it is lower; otherwise, it moves that word to d0 and branches.
- `compare_and_branch_2`: Compares d1 with 98 and d0 with 130, then branches to another routine if d1 is less than d0, otherwise it sets d0 to 130 and branches to another location.
- `compare_word_value`: Compares a word at offset 12 of a structure in a5 to 98, and if it is lower, branches to another routine; otherwise, it moves the value to d0 and branches to another routine.
- `write_and_compare_value`: Writes the constant 98 to a memory location at offset -20486 of a4, compares a value at offset 14 of a5 to 25, and branches to another routine based on the result.
- `write_constant_to_memory`: Moves the constant 25 into d0, writes it to offset -12936 of a4, unlinks a5, and returns.
- `wait_for_value_match`: Enters a loop that continues until the contents of two memory locations are equal.
- `rotate_bits_in_memory`: Calls a library function, stores the result in a global variable, adds a constant, and then performs a bitwise rotation or shift-based update to that global variable.
- `find_first_set_bit`: Calculates the number of leading zeros in a value and uses it to index into a table to perform a bitwise AND operation.
- `load_value_from_offset`: Returns a 32-bit value loaded from a memory address offset by a negative constant.
- `execute_unnamed_routine`: Empty routine that calls another routine.
- `decrement_and_execute_list`: Decrements two global short values, updates a third global short based on a subtraction, and executes a series of functions via a linked list.
- `update_global_state`: Copies a value from one global variable to another and updates several global shorts or bytes with constant values.
- `insert_linked_node`: Inserts a new node into a linked list if the second element of the provided structure is zero.
- `insert_into_linked_list`: Traverses a linked list to find a specific node and inserts a new pointer into the list, then clears a value at an offset.
- `traverse_and_clear_pointer`: Traverses a linked list until a certain pointer is reached, then copies a pointer and clears a value at a certain offset.
- `assign_pointer_and_clear`: Copies a pointer from one location to another and then clears the value at the offset of that pointer.
- `reset_global_pointer`: Calls another routine with specific arguments if a global variable is non-zero, then clears that global variable.
- `update_blt_registers`: Updates several bitplane-related registers and a DMA controller register using values or calculated bitmasks while checking a global flag.
- `empty_routine`: the routine is empty and performs no operations before returning.
- `calculate_blit_parameters_2`: Executes a routine that calculates and updates values used for a blitter operation, including bitmasking and pointer-based calculations.
- `calculate_blit_parameters`: Calculates and updates values used for a blitter operation, including bitmasking and pointer-based calculations.
- `update_blit_parameters_2`: Iterates through a data structure to calculate and update values used for Blitter operations, including bitmask and mask-related calculations.
- `update_blit_parameters`: Iterates through a data structure to calculate and update values used for Blitter operations.
- `update_blitter_parameters`: Calculates and writes new source, destination, and control registers for the blitter, including mask and area or mode-specific settings.
- `empty_routine_2`: The routine is empty and performs no operations before returning.
- `initialize_input_state`: Sets certain memory or hardware registers to a fixed value and fills a buffer with a constant value using a loop.
- `execute_indirect_call_8`: Performs an indirect call to a routine using a pointer or address stored in a register or memory.
- `call_indirect_function`: Calls a function pointer located at an offset from a global pointer.
- `clear_and_combine_bytes`: Clears certain bytes in a table and returns a value constructed from those bytes and a third byte.
- `clear_and_accumulate_table_bits`: Iterates through a table to clear specific bytes and calculate a bit-shifted checksum or hash-like value.
- `iterate_and_clear_table`: Iterates through a table, clearing bytes and calculating a bit-shifted value based on table contents.
- `clear_table_bytes`: Clears certain bytes in a table and returns a value constructed from those bytes.
- `clear_and_shift_bit_table_2`: Iterates through a table to clear bytes to zero while simultaneously updating an adjacent memory location with a bit-shifted value derived from the cleared bytes.
- `clear_and_shift_bit_table`: Iterates through a table, clearing bytes to zero and updating a bit-shifted value in a secondary or adjacent memory location.
- `process_linked_list_nodes`: Iterates through a linked list, calling a function with certain parameters and potentially calling it a second time if a bit or a global flag is set.
- `initialize_buffer_structure`: Calls a function to get a pointer, then populates a buffer with values from a global variable and a parameter.
- `call_routine_with_offset_pointer`: Checks if offset 8 of a5 is zero; otherwise, it calculates an offset from a word at offset 2 of a structure pointed to by offset 8 of a5, pushes two values to the stack, and calls a routine at an offset from a4.
- `validate_and_initialize_resource_buffer`: Calls another routine to retrieve a pointer, then performs a series of checks by copying bytes into a local buffer and comparing them against the magic constants 'FORM' and 'CMAP'. If these checks pass, it calculates a value by dividing a word from the structure by three and calls a routine to initialize a buffer structure.
- `update_table_via_loop`: Performs a loop that reads three bytes from a table using offsets derived from a global variable, performs bitwise operations on them, and writes the result to a destination table, then calls another routine and branches.
- `setup_stack_and_call`: Pushes values onto the stack, calls a routine at a specific offset, and then performs an extension and a branch.
- `call_with_stack_cleanup`: Pushes a value from a5 onto the stack, calls a routine, cleans up the stack, and returns a value from an offset in a5.
- `combine_three_bytes_into_word`: Reads three bytes from offsets 9, 11, and 13 of a structure in a5, shifts them left by 8, 4, and 0 bits respectively, and combines them into d0 using OR operations.
- `split_and_store_word_bits`: Extracts three different bit-shifted and masked segments from a word at offset 8 of a5 and stores each resulting word into three different memory locations pointed to by offsets 10, 14, and 18 of a5.
- `execute_loop_calls`: Iterates through a range defined by a parameter and calls a function using a value constructed from bit-shifted components.
- `transform_nibble_data`: Iterates through a table of words to perform bitwise manipulations on nibbles and stores the results in a local buffer.
- `execute_sequence_with_checks`: Calls a routine repeatedly until a certain value is returned, then updates a global variable with a constant value.
- `parse_input_sequence_table`: Iterates through a table of bytes to update a pointer and a counter, branching or updating a state-based counter based on the high bits of the current byte.
- `process_input_command_sequence`: Parses a sequence of bytes from a table to update a state machine, where certain bit patterns trigger a switch or branch to update a counter, a pointer, and a delay or timing variable.
- `parse_input_sequence_and_update_state_2`: Iterates through a table of bytes to update a counter and a pointer, while updating a global variable with a constant value and calculating a new state or jump destination based on bit-masked input values.
- `parse_input_sequence_and_update_state_3`: Iterates through a table of bytes to update a counter and a pointer, while updating a global variable with a constant value and calculating a new state or jump target based on bit-masked input values.
- `parse_input_sequence_and_update_state_4`: Iterates through a table of bytes to update a counter and a pointer, while updating a global variable with a constant value and calculating a new offset or state.
- `parse_input_sequence_and_update_state_5`: Iterates through a table of bytes to update a pointer and a counter, using a state machine to determine a new limit or next instruction offset based on the contents of the table and a global state variable.
- `parse_input_sequence_and_update_state`: Iterates through a table of bytes to update a counter and a pointer, while updating a global variable with a constant value and calculating a new timing or delay value.
- `update_state_machine_and_counters`: Updates a state-based counter and pointer using a table of bytes, while simultaneously adjusting several global counters and a temporary buffer.
- `update_input_buffer_state`: Updates a buffer or state machine by reading a byte from a table and calculating a new delay or next state.
- `process_input_sequence`: Iterates through a table of bytes to update a counter and a pointer, branching or updating a state-based counter based on the high bits of the read byte.
- `process_entity_state_updates`: Iterates through a list of structures to update internal state values based on specific command bytes.
- `search_and_update_pointer`: Searches for a specific constant value in a memory range and, if found, updates a global pointer with the address of that value.
- `iterate_entity_flags`: Iterates through a list of structures and calls another routine if a specific bit is set in a flag field.
- `iterate_entity_state_flags`: Iterates through a list of entity structures, checking a specific bit in a flag field and calling another routine if the bit is set.
- `increment_and_store_pointer`: Increments a global counter and, if it wraps to zero, calls a function to update a pointer, potentially resetting that pointer to a new value from a table.
- `update_animation_pointer_wrapper`: Calls a routine that updates a pointer in a table by adding a bit-shifted value from a register to an existing value.
- `update_pointer_and_offset`: Updates a pointer in a table using a return value from another routine and, if the new pointer is non-zero, increments a value at a location relative to that pointer.
- `update_pointer_and_target`: Updates a pointer in a table using a return value from another routine and, if the new pointer is non-zero, writes a new address to the memory location pointed to by that pointer.
- `populate_animation_pointer_table`: Iterates through a table to check bitmask flags and calls a library function to retrieve or update pointers, potentially performing a block memory write of a global byte if certain conditions are met.
- `process_recursive_pointer_chain`: Calls a routine with a pointer from a global table and stores the result in a global variable.
- `execute_conditional_pointer_loop`: Calls a routine to check a condition and, if non-zero, resets a global variable and potentially sets it to a specific value.
- `update_table_with_result`: Calls a routine using a pointer from a global table and a constant, then stores the returned value into a table offset from a global pointer.
- `store_routine_return_value`: Calls a routine using a pointer from a global table and stores the returned value into a table offset from a global pointer.
- `process_sprite_animation_sequence`: Updates a pointer-based table or state machine by calculating new animation or sprite offsets and checking bitmask-based conditions.
- `update_sprite_buffer_and_timer`: Copies a block of memory from a source table to a destination buffer and updates a timing or offset variable based on a calculated value.
- `update_animation_pointer`: Updates a pointer in a table by adding a bit-shifted value from a register to an existing value.
- `update_entity_state_recursively`: Iterates through an entity list to update or initialize state-related pointers and values, using a recursive check to ensure valid state or next-entity pointers.
- `update_clamped_input_vectors`: Splits a 16-bit input value into two 8-bit components and calculates the difference from their previous values.
- `update_clamped_values`: Updates two values in a structure by retrieving data from tables based on an index, applying clamping logic to ensure the results stay within specified minimum and maximum bounds.
- `call_empty_routine`: Calls another routine with a constant value of fifteen.
- `execute_conditional_callbacks`: Checks several global pointers and, if they are non-zero, executes the routines or library functions they point to.
- `execute_recursive_calls_3`: The routine calls itself repeatedly before returning.
- `initialize_library_pointers`: Loads library base addresses into global variables and checks if certain library functions return zero, potentially triggering a routine that clears an array or updates a global variable.
- `invoke_copy_data_table_with_offset`: Calculates a source address by adding an offset to a global variable and passes this address and a parameter to a routine that copies a table of words into a destination buffer.
- `iterate_and_call_function`: Iterates thirty-two times, calling a function with a global pointer and a value retrieved from a pointer-indexed table.
- `clear_global_pointer_target`: Clears a byte at a location derived from a global pointer and a pointer-indexed table, then calls another routine.
- `execute_loop_calls_2`: Calls a routine twice using a range or certain values, passing a destination buffer and a range or certain or certain values.
- `call_function_with_split_parameter`: Calls a function with a constant value and calls another function using a value split from a single parameter and a constant.
- `process_string_table_entry`: Uses a parameter to calculate an offset into a table, retrieves a pointer, and calls a library function using a value from a global variable.
- `update_pointer_from_string`: Retrieves a pointer or identifier from a string-based table and stores the result in a global variable, with a conditional check for a zero return.
- `traverse_entity_list`: Recursively calls another routine if a certain bit is set in a field within a list of structures.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `call_function_with_global_pointer`: Calls a function using a value stored at a global pointer location.
- `call_function_with_global_pointer_alt`: Calls a function using a value stored at a global pointer location using a different function pointer.
- `call_function_with_indirect_values_2`: Calls a function using two values retrieved from memory locations pointed to by global variables, or swaps the order of arguments if a certain condition is met.
- `call_function_with_indirect_values`: Calls a function using two values retrieved from memory locations pointed to by global variables.
- `call_function_with_parameter`: Calls a function using a global pointer and a provided parameter.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_buffer_contents`: Checks a condition or value, then uses a function pointer to perform memory-clearing or block-filling operations on a buffer, followed by a copy or write operation and a final cleanup or state-update routine.
- `update_buffer_range`: Performs a sequence of memory copies and updates a pointer or value based on the result of a memory check.
- `update_buffer_state`: Checks a condition or return value and conditionally clears or copies a range of memory or a buffer, followed by a block-fill or copy operation and a final cleanup or size-update call.
- `update_buffer_state_2`: Performs memory copies and conditional block-fills or clears based on a return value or a condition.
- `perform_multiple_function_calls_2`: Executes a sequence of function calls that manipulate values in a structure, including clearing a value and copying data between buffers.
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
- `reset_system_state`: Checks a global flag and, if zero, executes a sequence of routines and updates a global variable.
- `set_half_state_flag`: Checks a parameter, calls a routine with a constant if non-zero, sets a global byte to zero, executes an indirect call, and calls another routine with a constant.
- `initialize_memory_tables`: Sets certain memory locations to specific constant values using a loop.
- `sum_table_bytes`: Iteratively calls a function to fetch values from a table and accumulates certain bytes into a local variable.
- `update_tables_with_calculated_offsets`: Writes certain values to a table, calculates and stores new pointer-based offsets into a table, and calls a routine with a pointer from a table.
- `initialize_game_state`: Resets several memory-mapped or global arrays to zero and configures a table of parameters based on a specific or previously set value.
- `test_and_execute_conditional_routine`: Checks if a routine returns zero twice, then sets a flag in a global variable, executes a function using a pointer-to-function-pointer with two arguments, and resets the flag.
- `execute_sequence_of_function_pointers`: Executes a series of indirect function calls using addresses or offsets derived from a global pointer, and writes a constant value to a memory location.
- `execute_conditional_function_calls`: Checks a global variable and either executes a sequence of four function calls or a single different function call.
- `update_conditional_state`: Calls a routine with a constant, conditionally sets a global byte, and updates a global word or pointer based on the result of an indirect call or a comparison.
- `call_offset_routine`: Calls another routine using a value calculated from a pointer and an offset.
- `recurse_with_parameter`: Checks a parameter and calls itself recursively with that parameter and a global variable.
- `call_library_with_parameters`: Calls a library function using a pointer and certain values retrieved from a table.
- `update_conditional_table_entry`: Initializes a memory range to a constant value and then conditionally updates a table entry based on a comparison with a library function return value.
- `copy_word_to_pointer_2`: Checks if a global value is less than a certain amount and then stores a word from a parameter into that global location.
- `copy_word_to_pointer`: stores a word from a source pointer into a destination pointer.
- `recursively_call_routine`: Checks if a parameter is non-zero and, if so, calls itself with that parameter.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_table_data`: iterates through a loop to copy or update several values in a table based on conditional checks and pointer-based lookups.
- `search_and_update_table`: Iterates through a table to find an entry that satisfies a condition, then updates a value or a pointer in that table.
- `search_and_update_table_entry`: Iterates through a table to find an entry where a certain condition is met, then updates a value in that table or a secondary pointer-based value using a division-based check.
- `update_pointer_table`: Iterates through a table to update a pointer or a value based on a division, while conditionally updating a byte or a flag.
- `update_pointer_list`: Updates a pointer in a table and a counter, while conditionally updating a byte or a value based on a division.
- `decrement_and_store_byte`: Decrements a byte value stored in a table and stores the result into a memory location calculated from a pointer and an index.
- `update_pointer_list_and_values`: Increments a counter in a structure, updates a pointer or index in a table, and conditionally updates a value or a flag based on a division result.
- `initialize_array_bounds`: clears a range of memory and then populates a secondary table with index-based offsets.
- `initialize_memory_and_flags_2`: Clears a range of memory and sets a memory location to zero or one based on a previously cleared byte.
- `initialize_array_and_flags`: Clears a range of memory, populates a secondary table with offsets, and sets a memory location to zero or one based on a condition.
- `initialize_memory_and_flags`: Clears a range of memory and then sets a certain memory location to zero or one based on a previously cleared byte.
- `reset_word_to_zero`: stores a zero into a word at a specific pointer location.
- `update_index_with_function_results`: iterates through a loop, calling a function via a pointer and storing the result or its negation into a table.
- `load_and_call_pointers`: calls two library functions, clears a table, loads several values from a library-referenced structure, and calls another routine for each loaded pointer.
- `execute_callback_list`: iterates through a table of pointers and calls another routine for any non-zero pointer, then calls two library functions.
- `process_recursive_parameters`: Executes a function call using a value from a pointer and then calls itself recursively with modified parameters.
- `dispatch_indirect_calls`: Calls three different routines using a pointer from a table, passing various combinations of provided parameters.
- `execute_parameterized_calls_4`: Executes a loop that calls a routine with parameters derived from a table and a structure, and conditionally updates a value or a pointer.
- `execute_parameterized_calls_2`: Calls a routine via a pointer using a value from a table, then calls another routine using four parameters retrieved from a structure.
- `execute_loop_of_parameterized_calls`: Executes a loop that calls a routine with parameters derived from a table and a structure, then calls a routine via a pointer with a single parameter.
- `execute_parameterized_calls_3`: Executes a loop that calls a routine with parameters derived from a table and a structure.
- `execute_parameterized_calls`: Calls a routine via a pointer using a value from a table, then calls another routine using four values retrieved from a structure.
- `call_wrapper_routine`: calls a function via a pointer with three parameters, calls a second function via a pointer, and then calls another routine using the first parameter, the fourth parameter, and the result of the second function.
- `increment_counters_and_read_memory`: Increments counters in a structure and reads a word from a memory location located before a pointer.
- `increment_counters_until_limit`: Increments two counters in a structure and skips certain bytes in a table until a certain limit or null terminator is reached.
- `read_word_predecessor`: reads a word from a memory location located four bytes before the address stored in a register.
- `call_with_offset_parameters`: Calls a routine via a pointer using a single parameter, then calls another routine using four parameters where two are sums of input values.
- `call_via_offset_table`: Retrieves bytes from a table using an offset and calls a routine twice if a specific byte in the table is non-zero.
- `update_entity_attributes`: Reads attribute offsets and conditional modifiers from a table and dispatches them to a series of subroutines.
- `update_table_data_2`: Extracts multiple values from a table using an index and offsets, then calls three different routines with these values and updated pointers.
- `call_subroutine_via_pointer`: Calls a routine through a pointer stored in a global table using a passed parameter.
- `execute_indirect_call`: Fetches a function pointer from a table using an offset from a base address and calls it with a parameter.
- `execute_indirect_function_call`: Fetches a function pointer from a memory location and calls it using a value passed as an argument.
- `clear_memory_buffer`: Uses nested loops to set a range of memory or a multi-dimensional array or buffer to zero.
- `add_to_table_values`: Updates a value in a table using a calculated address and then adds a value from a register to several memory locations.
- `add_to_table_value`: Calculates an address in a table using an index and an offset, then adds a value from a register to the memory at that address.
- `update_global_state_and_table`: Executes a sequence of indirect function calls that update global state and modify table values based on conditional checks.
- `update_state_and_execute_subroutine`: Checks a flag in a structure to decide whether to reset a value or call a routine, then updates a global flag and executes a sequence of indirect function calls using parameters or return values from a previous routine.
- `execute_nested_calls`: Calls a routine, stores its return value in a global variable, and executes a sequence of other routines with or without certain parameters.
- `store_and_call_concatenated_2`: Calls a routine, stores its return value in a global variable, and calls another routine using a concatenated 32-bit value.
- `store_and_call_concatenated`: Calls a routine, stores a returned value in a global variable, and calls another routine using a concatenated 32-bit value.
- `update_global_state_2`: Sets a global variable to zero and calls routines using a pointer and a concatenated value or a pointer-based argument.
- `update_global_state_3`: Resets or updates certain global variables and executes a sequence of indirect function calls with or without arguments.
- `update_table_values`: Performs complex conditional updates to a table or array, involving index-based lookups and division-based assignments.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a2).
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a7).
- `recalculate_attribute_table`: Iterates through a table of structures to recalculate or copy attribute values, such as a ratio or a secondary property, based on a primary value.
- `update_attribute_table`: Iterates through a table of structures to recalculate or copy attribute values, such as a ratio or a secondary property, based on a primary value and a lookup or calculation involving a secondary offset.
- `initialize_global_state`: Calls a routine with two arguments, stores the return value in a global variable, and then clears a small array of four or more words to zero.
- `call_routine_with_constant`: Calls a routine using a value stored in a global variable and a constant integer as an argument.
- `enable_dma_channel`: An 18-instruction routine that modifies the DMA controller register to enable a DMA channel.
- `update_dma_config`: Writes a value from a memory location to a hardware register and sets another hardware register to a constant value.
- `update_coprocessor_hardware`: Writes a longword and a word into hardware registers using an offset calculated from a provided parameter.
- `enable_dma_channels`: Sets a global variable to a value that enables DMA channels via the DMACON register.
- `write_hardware_registers`: Writes two or-sized values into a hardware register or a memory-mapped area using an offset derived from a parameter.
- `check_table_entry`: Loads a word from offset 8 of a structure in a5, sign-extends it, shifts it left by 2, and tests the longword at that offset from a base address in a4; if the value is non-zero, it branches to another routine, otherwise it returns.
- `update_state_by_case_2`: Checks a value against several cases and, depending on the case, modifies a global memory area or updates a pointer or a value.
- `initialize_dma_settings`: Stores several constant values into a global memory area and enables a DMA channel using a value from a table.
- `update_dma_and_memory_values`: Performs arithmetic on a value at offset -8 of a5, writes constants to offsets -14, -13786, -13778, and -13772 of a4, and branches to a local address.
- `write_status_values`: Writes a word of 1 to an offset of 22 from a5, checks a word at an offset of 53 of a4, and if not equal to zero, writes 1 to an offset of 22 from a5 and 120 to an offset of 10 from a5 before branching to another location.
- `set_dma_value`: Uses a value from a table to store a constant through a pointer into a memory location and enables a DMA channel.
- `do_nothing`: this routine is empty and performs no operations.
- `update_state_by_case`: Checks a register against several possible values and, depending on the case, modifies a global memory area or updates a pointer or a value in a table.
- `return_no_op_2`: The routine consists of a single return instruction and performs no work.
- `clear_and_set_pointers`: Clears a range of memory, calls a library function, and stores return values from other routines into a global variable.
- `update_table_from_lookup_2`: Iterates through a range of indices to update a destination memory area using values fetched from a secondary lookup table.
- `update_table_from_lookup`: Iterates through a table to update a destination memory area using values fetched from a secondary lookup table.
- `call_function_conditionally`: Checks a global variable and calls a function pointer with a specific value if that variable is non-zero, then calls the same function pointer with a different constant.
- `decrement_and_clamp_values`: Decrements a global counter and, if it reaches zero, resets certain values; otherwise, it updates a range-limited value using a clamped or otherwise modified value.
- `update_pointer_and_call`: Calls a function pointer, calls another function pointer with zero, and copies a value from a memory location to a global variable.
- `reset_function_and_copy_value`: Calls a function pointer with zero and copies a longword from a memory location pointed to by a pointer-to-pointer into a global variable.
- `reset_state_and_call_subroutines`: Clears a value in a global memory location and then executes two previously stored function pointers, passing a zero or no argument to the first.
- `execute_function_pointer`: Calls a function through a pointer stored at a specific offset from a base address.
- `configure_dma_and_clear_flag`: Clears a global flag and calls a routine to enable a specific number of DMA channels, followed by a routine that writes a value to a memory location.
- `test_global_pointer`: Tests if the longword at offset -18790 from the address in a4 is non-zero; if it is, it branches to another routine, otherwise it returns.
- `call_with_dereferenced_value`: Calls another routine using a value fetched from a memory location.
- `decompose_and_update_table`: Decomposes a parameter into individual digits and stores them in a local array, then iterates through a table to call another routine based on those values or a conditional check.
- `update_table_with_logic`: Iterates through a list to update a memory area with a constant or zero, then calls another routine using values from a table.
- `iterate_and_call_on_condition_2`: Iterates through a list and calls a function pointer with a value from a memory location if a certain condition in a lookup table is unable to be met.
- `iterate_and_call_on_condition`: Iterates through a list and calls a function pointer with a value from a memory location if a certain condition in a lookup table is met.
- `get_absolute_value`: Returns the absolute value of a passed word.
- `return_dereferenced_word`: Returns a word stored at an offset from a pointer.
- `copy_byte_sequence_2`: copies a specified number of bytes from a source memory location to a destination memory location.
- `copy_byte_sequence`: Copies a sequence of bytes from one memory area to another using a loop and returns the final index.
- `process_function_chain`: Iteratively calls a routine via a pointer, updates a global variable with the return value, and conditionally updates a byte from a structure.
- `update_function_pointer`: Updates a global pointer with the return value of another routine.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `update_position_and_clear_byte`: Calculates a new coordinate using a pointer-based offset and calls a routine with the result, then potentially clears a byte in a buffer if a certain condition is met.
- `decompose_and_render_value`: Decomposes a numeric parameter into individual digits stored in a local array, then iterates through that array to call another routine using a certain value.
- `parse_string_and_get_string_id`: Copies a null-terminated or colon-terminated string into a global buffer and retrieves a corresponding identifier or pointer.
- `traverse_linked_list_and_update_value`: Iterates through a chain of pointers, updating a global variable with a value retrieved from each node's next pointer.
- `return_constant_value`: Returns a constant integer value.
- `traverse_function_pointer_chain`: Iteratively calls functions via a pointer retrieved from a memory location, updating a global variable with the return value and checking if the new return value matches a previously stored value.
- `execute_conditional_logic`: Performs a series of conditional checks and calls other routines, potentially updating a global variable and calling a library function.
- `traverse_pointer_chain`: Iteratively calls functions via a pointer retrieved from a memory location, updating a global variable with the return value.
- `iterate_and_assign_pointer`: Compares a global variable against a value and assigns a new value from a table if certain conditions are met.
- `return_constant_value_2`: The routine is empty and performs no operations before returning a constant integer value.
- `handle_state_transition`: Checks a parameter, calls a routine to update a global variable, and executes a sequence of calls including a library function and a loop.
- `handle_disk_swap_and_state_transition`: Checks a flag to determine if a disk swap or state change is required, then updates a global state variable and executes a sequence of conditional routine calls or an infinite loop.
- `call_indirect_functions_sequentially`: Pushes four different words onto the stack and calls a routine via a5 four times, passing a different address in a4 each time.
- `call_indirect_functions_conditionally`: Pushes a value onto the stack and calls a routine via a5, then compares a value at offset -13858 of a4 to 5 and calls the routine again via a5 if the comparison fails.
- `process_indirect_function`: Pushes a word of 105 to the stack, calls an indirect function using a value from the stack, and then either performs a jump based on a table lookup or clears a word at an offset of 15524 of a4 and performs several memory operations before returning.
- `write_table_values`: Iterates up to five times to write values from a table into a destination via a routine call, then resets a global variable if certain conditions are met.
- `execute_indirect_function_3`: loads a function pointer from a memory location and calls it with a constant argument.
- `get_and_return_value`: Calls a routine and returns a value stored in a global variable.
- `initialize_game_state_constants`: Checks a global flag and, if non-zero, initializes a series of global or static memory locations with constant integer or short values.
- `initialize_global_variables`: Sets several global or static memory locations to specific constant values or copies a value from a pointer-based memory location.
- `update_timer_and_bounds`: Decrements a global counter and, if it reaches zero, resets certain values; otherwise, it updates a range-limited value using a clamped or or-ed calculation.
- `conditional_execute_routines`: Checks several global conditions and, if satisfied, executes two different routines based on a returned value and a counter check.
- `update_attribute_table_entry_2`: Updates or resets a series of global attribute or property values, such as flags, timers, and table-derived offsets, based on a conditional check of an input parameter and a global counter.
- `set_short_values`: Stores a constant value of one and a constant value of twenty-eight into a global variable via an offset.
- `set_short_values_alt`: Stores a constant value of one and a constant value of forty into a global variable via an offset.
- `update_attribute_table_3`: Calculates a new index or offset and updates a series of table entries with constant values and data fetched from a table.
- `update_attribute_table_2`: Checks if a global value exceeds a certain threshold, then updates a series of offsets in a table with new constant values and data fetched from another table using an index-based offset.
- `set_attribute_values`: Stores several constant values into a global variable via an offset and conditionally updates a table entry based on a comparison.
- `set_multiple_values`: Stores several constant values into a global variable via an offset.
- `update_attribute_table_entry`: Updates a series of global attribute or property values based on a conditional check of a register.
- `execute_indirect_from_pointer`: Loads a function pointer from an offset of a global pointer and calls it.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `fill_buffer_with_count`: The routine iterates a specified number of times, writing the current loop counter value as a byte to a destination buffer and incrementing the pointer.
- `copy_bytes_with_limit`: Iterates through a nested loop structure, calling another routine to fetch a byte, and stores that byte into a buffer until a limit or a return value of -1 is encountered.
- `iterate_and_copy_bytes`: Iterates through a loop, calling another routine and storing the returned byte into a buffer until a certain condition or a return value of negative one is met.
- `process_and_store_byte`: Calls another routine to retrieve a value, stores that value into a buffer, and potentially increments a counter or checks a limit.
- `iterate_and_copy_bytes_2`: Iterates through a buffer, calling another routine to fetch a value, and stores that value into a buffer until a certain condition or a return value of -1 is met.
- `clear_bit_and_call_handler`: Clears bit 3 of a longword at offset 12 of a2, checks bit 2 of the same location, and if bit 2 is clear, calls another routine after pushing a pointer and a constant to the stack.
- `branch_to_mid_routine`: Performs a short branch into another routine.
- `clear_and_call_routine`: Checks if a word at offset 16 of a5 is 1 and if a2 is non-null, then clears offsets 0 and 4 of a2, pushes several values to the stack, and calls a routine using a byte from offset 13 of a2.
- `clear_d0_and_branch`: Sets d0 to zero and branches to another routine.
- `calculate_pointer_difference_2`: Pushes a2 to the stack, calls a routine using a byte from offset 13 of a structure in a2, and if bit 2 of offset 12 of a2 is set, calculates the difference between a2 and offset 8 of a2 and adds it to a value on the stack.
- `calculate_pointer_difference`: Checks if a2 is null; if not, it calculates the difference between a pointer at offset 4 of a2 and a2 itself, stores this in d0, subtracts d0 from a value at offset 4 of a5, and returns.
- `get_and_increment_pointer_2`: Checks if a pointer-range or counter is valid, increments the pointer, and returns the byte previously pointed to.
- `get_and_increment_pointer`: Returns a byte from a pointer and increments that pointer.
- `update_pointer_and_set_bits_2`: Checks a bitmask in a structure, potentially scans a memory range or recurses, and updates a pointer or sets a bit in a byte based on a return value.
- `update_pointer_and_set_bits`: Clears a bit in a byte, checks a flag to potentially execute a loop that scans a memory range, and updates a global pointer and its associated value based on a return value.
- `set_bit_in_byte`: Sets a specific bit in a byte located at an offset within a global variable.
- `update_pointer_and_calculate`: Updates a pointer in a table, modifies an adjacent value, and returns a masked, concatenated value.
- `call_and_validate`: Saves a pointer from a2 onto the stack, calls another routine, and if the returned value in d0 is non-zero, it branches to another routine; otherwise it clears d0 and returns.
- `push_and_call`: Pushes three longwords onto the stack, calls another routine, and then branches to another routine.
- `call_and_check_byte`: Saves registers, pushes a value from a5, calls another routine, and then checks a byte at the address in a2; if the byte is non-zero, it branches to another routine, otherwise it writes a constant to a global variable and returns.
- `compare_bytes_and_increment`: Compares bytes from two different sources, and if they match and are non-zero, it performs arithmetic on a byte and either branches or increments a pointer in a2 and branches.
- `call_and_loop`: Pushes two words onto the stack, calls another routine, and then loops if the returned value in d0 is not negative sixty-five-five-hundred-thirty-six.
- `write_bytes_to_structure`: Writes the value from d4 to offset d of a structure pointed to by a0, then writes the constant one to offset c of the same structure before branching to another routine.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `find_char_in_string`: Iterates through a string until a specific character or a null terminator is encountered, returning the pointer to that character.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `perform_no_operation`: This routine is empty and performs no executable instructions or operations.
- `execute_indirect_call_2`: Performs an indirect jump or call to a location stored in a register or memory.
- `execute_conditional_logic_2`: Calls a routine using a pointer in a5, multiplies d0 by 6, calculates an address using a2 and a constant from a4, and if d4 is within a specific range and a value at the calculated address is not zero, writes 2 to an offset in a4.
- `handle_io_error`: Pushes three values onto the stack, calls a routine, and if the result is not negative one, enters a loop that repeats the call, otherwise it calls an error handler and stores a value at an offset in a4.
- `push_seek_argument`: Clears two longwords on the stack, moves a longword from a pointer in a2 to the stack, calls a routine, adjusts the stack pointer, and branches to another routine.
- `call_subroutine_with_stack`: Saves a word from a5 and the constant three-hundred-one onto the stack, pushes a pointer from a5, calls another routine, and then returns.
- `setup_and_branch`: Saves registers to the stack, calls another routine, and then loads a value from a global variable into a3 before branching.
- `loop_and_compare_d4`: Multiplies d4 by six, checks a value in a table at a3 using that result as an index, and loops while incrementing d4 until it reaches a value at an offset in a4.
- `handle_file_operations`: Checks bit 1 of offset 12 of a5 and performs a sequence of calls including a routine at an offset from a4, a file deletion routine, and an error handler, depending on various conditions and register values.
- `execute_indirect_call_9`: Pushes a longword onto the stack, saves a2, calls a routine at an offset from a4, and if d0 is not zero, calls another routine at an offset from the program counter before branching.
- `execute_indirect_calls_3`: Pushes two values onto the stack, calls a routine at an offset from the program counter, and if d0 is not zero, pushes d7 to the stack and calls another routine using a pointer in a4.
- `perform_io_operations`: Pushes two values onto the stack, calls a routine at an offset from the program counter, pushes a third value, calls another routine at an offset from the program counter, and then branches to another location.
- `close_file_conditionally`: Checks if a word at offset 12 of a structure in a5 has bit 9 set; if so, it pushes d6 to the stack, calls a routine to close a file, sets d5 to 4, updates a value at a specific offset in a4, sets d0 to negative one, restores registers, unlinks a5, and returns.
- `close_file_logic`: Calculates an offset into a table using d4, moves a value from that table into a register, and if bit 3 of a word at offset 12 of a structure in a5 is set, it calls a routine using a pointer from a3 and d6.
- `read_buffer_and_check_error`: Reads a buffer and checks for a negative return value, potentially storing an error code or a returned value into a global variable.
- `read_buffer_and_store_error`: Reads a buffer and checks for a negative return value, storing a specific error code or a returned value into a global variable.
- `read_buffer_with_error_check`: Calls a routine to read data and checks if the return value is negative, potentially calling another routine to store an error or result.
- `empty_routine_12`: the routine is empty and performs no operations.
- `rebuild_active_object_list`: Iterates through a table of object or entity pointers, checking for non-zero or non-empty entries, and rebuilds a secondary list or buffer by calculating new pointer offsets and clearing or updating certain fields.
- `update_table_entries`: Iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `manage_library_resources`: Interacts with library structures to check status, execute functions, and update a pointer in a table.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `clear_memory_and_call_function`: Clears a block of memory using a loop and then calls a function pointer retrieved from a global variable.
- `empty_routine_3`: the routine contains no executable instructions and performs no operations.
- `parse_string_tokens`: Parses a source string to extract and store individual tokens, handling quoted strings and skipping whitespace.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_to_buffer_2`: Calculates the length of a source string and copies it to a destination buffer, ensuring a null terminator is present.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `traverse_linked_structure`: Traverses a linked structure, updates a global pointer, and calls a function pointer or an indirect routine with a specific parameter.
- `traverse_and_update_pointers`: Traverses a linked structure, updates a global pointer with a value from a table, and calls a function pointer.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 3deleted value.
- `apply_sign_to_result`: Calls a routine and then negates the returned result if a certain input register is negative.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `transform_and_divide_words`: Performs a complex bitwise transformation involving word swapping, doubling, and conditional subtraction, followed by a division-based remainder calculation.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `increment_pointer_and_store_byte`: Increments a pointer stored in a global variable and stores a byte from a table into the memory location pointed to by the incremented pointer while checking a flag.
- `increment_and_store_byte`: Increments a pointer stored in a global variable and stores a byte from a table into the memory location pointed to by the incremented pointer.
- `compare_pointer_ranges`: Calls a routine, then compares a value derived from a4 against a value in a2 to determine whether to return a value from the stack.
- `check_pointer_and_return`: Loads a pointer from offset 8 of a5 into a2 and checks if it is non-zero; if it is zero, it sets d0 to negative one, otherwise it sets d0 to zero and returns.
- `process_structured_data`: Checks bit 2 of a byte at offset 12 of a2; if set, it saves a2 and a3 to the stack, calls a routine, and then performs several conditional calls to other routines using values from a2 before clearing offsets 0, 4, 8, and 12 of a2 and returning.
- `process_pointer_and_flag`: Updates a global pointer and checks a flag before performing conditional pointer or flag updates.
- `process_buffer_and_update_state`: Recursively checks a buffer or range, updating a pointer and a flag or clearing a pointer and flag if certain conditions or end-of-buffer markers are met.
- `update_pointer_and_flag_2`: Modifies a pointer and a flag, and may copy a byte from a source to a destination pointed to by a table entry.
- `update_pointer_and_flag`: Updates a pointer and a flag, and may copy a byte from a source to a destination pointed to by a table entry.
- `update_pointer_and_set_flag`: Updates a pointer, modifies a value at an offset, sets a bit in a byte, and stores a byte from a table into the memory location pointed to by the incremented pointer.
- `validate_address_range`: Pushes a pointer from a2 onto the stack, checks a byte at offset 12 of a pointer derived from a4, and if it is non-zero, checks if a calculated address is within a specific range before returning.
- `branch_to_sub_routine`: Performs a short branch to another routine.
- `clear_structure_fields`: Clears the longword at a2, the longword at offset 4 of a2, and the longword at offset 8 of a2, then moves a2 to d0 and branches to another routine.
- `modify_memory_and_call_routine_2`: Writes a constant to a memory location, modifies a byte via a bitwise OR, copies a value from a pointer, and calls another routine.
- `modify_memory_and_call_routine`: Writes a constant to a memory location, modifies a byte via a bitwise OR, copies a value from a pointer, and calls another routine while conditionally updating a byte.
- `setup_linked_list_pointer`: Links a5, pushes registers a2 and a3 to the stack, and moves a pointer from an offset of a4 into a2 before branching to another routine.
- `loop_and_clear_global`: Stores a pointer from a2 into a3, calculates a value from offset 4 of a2, pushes it to the stack, calls a routine at an offset of a4, and loops back to the start of the routine while decrementing the value until it reaches zero, finally clearing a longword at an offset of 20114 of a4.
- `insert_into_linked_list_2`: Uses a function call to get a pointer, then inserts a new node into a linked list by updating a global head pointer and a node's next pointer.
- `swap_and_store_pointers`: Loads a pointer from a fixed offset, stores a new pointer or value into that location, and returns an incremented pointer.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `update_a2_pointer`: Saves a2 and a3 to the stack, subtracts a3 from itself, and loads a new value into a2 from an offset of a4.
- `copy_pointer_from_list`: Compares a pointer at offset 8 of a5 with a2, and if they are not equal, sets a3 to a2, updates a2 with the value at a2, and if d0 is not zero, branches back to this routine.
- `copy_pointer_value`: If d0 is not zero, copies a longword from a pointer in a2 to the location pointed to by a3, then branches to another routine.
- `call_handler_with_offset`: Loads a pointer from a2 and a value from an offset of a4, increments the value in d0 by 8, pushes d0 and a2 to the stack, calls a routine, sets d0 to zero, adjusts the stack pointer, and branches to another routine.
- `validate_table_index_and_call`: Checks if a provided index is within a certain range and if a table entry is non-zero, then calls another routine and returns a boolean result.
- `check_table_condition`: Calls a routine using a pointer calculated from a table and an offset, returning a boolean result based on whether the return value is non-zero.
- `return_zero_2`: The routine returns a zero value.
- `execute_error_sequence`: Pushes a value from offset 8 of a5 onto the stack, calls a routine, and if d0 is non-zero, enters a loop that repeats the call, otherwise it calls another routine and stores a value in a4.
- `clear_d0_and_branch_2`: Sets d0 to zero and branches to another routine.
- `evaluate_io_status_recursively`: This routine checks an I/O result and, if an error is detected, calls itself recursively while updating a global variable.
- `check_io_status_recursively`: Evaluates an I/O result and, if an error is detected, recursively calls itself and updates a global variable with the result.
- `evaluate_recursive_io_result`: checks the return value of a routine called with certain parameters and, if it indicates an error, recursively calls itself and stores the result into a global memory location.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `execute_conditional_branch`: Calls a routine with a fixed range and, if a specific bit is set, checks a global variable before potentially calling another routine.
- `execute_and_return_zero`: Calls a routine and then returns a zero value.
- `execute_write_sequence`: Calls a function via a pointer to process a global variable and then executes two subsequent routine calls.
- `execute_indirect_functions`: Checks a global pointer and, if non-zero, executes the routine pointed to by that address, followed by another indirect function call.
- `validate_and_clear_index`: Checks if an index is within a valid range and if a pointer in a table is non-null, then clears a value in a table or calls another routine recursively.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `call_library_vector_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 6 of that base, returning its result to the caller.
- `call_library_vector_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 12 of that base, returning its result to the caller.
- `execute_indirect_from_table`: Executes a routine located at an offset from a pointer stored in a global table.
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
- `execute_indirect_from_address`: Executes a routine located at a fixed offset from a global address.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `execute_indirect_function_4`: loads a function pointer from a nested table lookup and executes it.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_seek_2`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_un_lock_2`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_pointer`: Loads a function pointer from a nested pointer or table and executes it.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `execute_indirect_function_5`: loads a function pointer from a memory location offset by a register value and jumps to it.
- `execute_indirect_function_6`: loads a function pointer from a nested table lookup and executes it.
- `execute_indirect_call_3`: Performs an indirect call to a function pointer located at an offset from a register.
- `execute_indirect_call_with_dereference`: Loads a pointer from a table and performs an indirect call to that address.
- `execute_indirect_function_7`: calculates an offset from a register and executes the code located at the resulting address.
- `execute_indirect_function_pointer_2`: Loads a function pointer from a nested pointer or table and executes it.
- `execute_indirect_function_pointer_3`: Loads a function pointer from a nested pointer or table and executes it.
- `execute_indirect_function_8`: loads a function pointer from a table using an offset and executes it.
- `execute_indirect_function_9`: Loads a function pointer from a nested table lookup and executes it.
- `execute_indirect_function_10`: Loads a function pointer from a memory location calculated by adding an offset to a register and executes it.
- `execute_indirect_function_11`: loads a function pointer from a nested table or structure and executes it.
- `call_indirect_from_table`: Loads a function pointer from a nested table or structure and calls it.
- `execute_indirect_function_12`: loads a function pointer from a nested table or structure and calls it.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `execute_indirect_call_4`: Performs an indirect call to a function pointer located at an offset from a global variable.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `execute_indirect_call_5`: Loads a pointer from a global variable, applies an offset, and calls the resulting address.
- `execute_indirect_function_pointer_4`: loads a function pointer from a nested table or structure and executes it.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_function_13`: Loads a function pointer from a table located via an offset from a global pointer and executes it.
- `execute_indirect_function_14`: Loads a function pointer from a table located at a fixed offset from a base address and executes it.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_function_15`: loads a function pointer from a table located at a fixed offset from a base address and executes it.
- `call_library_vector_486`: Library stub: loads a library base from a global variable and calls the library function at offset minus 486 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_5`: loads a function pointer from a nested pointer or table and executes it.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_indirect_function_16`: loads a function pointer from a table located via an offset from a global pointer and executes it.
- `execute_indirect_function_17`: Loads a function pointer from a table located via an offset from a global pointer and executes it.
- `execute_indirect_function_pointer_6`: Loads a function pointer from a table located via an offset from a global variable and executes it.
- `execute_indirect_call_6`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `execute_indirect_call_7`: Performs an indirect call to a function pointer retrieved from a table using a different offset.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `execute_indirect_function_18`: loads a function pointer from a table using an offset from a global address and executes it.
- `execute_indirect_function_19`: Loads a function pointer from a table using an offset from a global pointer and executes it.
- `execute_indirect_function_20`: loads a function pointer from a table using an offset from a global pointer and executes it.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_7`: loads a function pointer from a table using an offset from a global variable and executes it.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:XC2.1

- `initialize_and_expand_data_stream`: Copies a block of data from a source to a destination and then processes a bitstream or compressed-style data stream to expand or decompress it into a target memory area.
- `shift_and_return_value`: Loads a word from a pointer, stores it into a destination, and returns the value right-shifted by one with the highest bit set.
- `shift_and_loop_register`: Shifts a register right and executes a loop that decrements a register until it reaches a certain value.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:bootblock

- `check_function_return`: Calls a function from a library and returns a certain value based on whether the return value is zero.

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

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subsequent_functions`: Calls a routine that initializes memory blocks and then calls two other function pointers.
- `populate_memory_tables`: Performs a series of writes to various memory locations using a loop and an offset derived from a global variable.
- `update_sprite_attribute_table`: Iterates through a range of memory to update or duplicate sprite or object attribute data, including updating or clearing associated word-sized or byte-sized metadata or secondary attribute buffers.
- `initialize_memory_blocks`: Sets several global variables or memory locations to specific values and clears a series of byte-sized memory locations using a loop.
- `clear_memory_blocks`: Clears or zeros out a series of word-sized memory locations using a loop.
- `update_memory_tables`: Copies word-sized values from a source table to a destination table, using an offset or a value from a global variable to determine the source address.
- `loop_and_write_bytes`: Calculates an address using a5 and a4, reads a byte from that address, and enters a nested loop structure that writes bytes to specific memory locations based on values in a5 before returning.
- `initialize_string_buffer`: Allocates a memory buffer and initializes it with a specific string header, then updates several global state pointers and flags.
- `copy_d0_to_d5`: Copies the longword from register d0 to register d5.
- `process_callback_list`: Iterates through a list of function pointers and executes them, followed by a routine that passes a calculated size or pointer to another function.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_game_state_and_handle_logic`: Resets a series of global state counters and pointers to a default or zeroed state before recursively calling itself to handle state transitions.
- `update_sprite_attributes`: Iterates through a table of object descriptors to modify certain attribute bytes, such as color or certain flags, and calls a routine to update the display or position.
- `validate_input_byte`: checks if a byte extracted from a word is within a specific set of prohibited or allowed values and returns a decremented offset if no match is found.
- `return_immediately_2`: The routine performs no operations and immediately returns.
- `execute_empty_routine`: The routine performs no operations and returns immediately.
- `check_value_range`: Compares a register against a series of constant values and returns a decremented value if no match is found.
- `empty_routine_10`: The routine is empty and performs no operations.
- `clear_global_byte`: stores a zero byte into a global variable.
- `update_global_pointers`: executes a function pointer and then copies values from a table or structure into several global variables.
- `call_subroutines`: calls two subroutines via pointers, passing zero as an argument to the first one.
- `update_state_counters`: Updates or resets several global counters and pointers based on conditional checks of existing state values.
- `set_conditional_values`: sets certain global bytes or words based on the value of a parameter and then executes a function pointer.
- `update_sprite_attributes_2`: Sets certain global words, then enters a loop that calls another routine and updates a global word with a modulo-reduced value.
- `call_with_table_value`: executes a function pointer using a value retrieved from a table.
- `reset_and_configure_game_state`: Initializes a table with a constant value and resets several state-related flags or counters.
- `update_entity_attributes_and_scale_values`: Checks a word in a global structure, then calls another routine, iterates through entity attributes, calls a routine to scale and store values, and finally clears several word-sized fields at specific offsets within a global structure.
- `execute_scaled_calls`: Clears a word on the stack, calls a routine at offset 4344, clears the stack, calls a routine at offset 12108, and branches to offset 4122.
- `execute_scaled_operations`: Clears the stack, calls iterate_entity_attributes with 26 as an argument, calls scale_and_store_values with arguments 1, 7, and 10, calls another routine, writes 1 to a specific offset in a4, and then performs a jump based on a table lookup using d0.
- `fill_array_with_parameter`: executes a function pointer and then loops to assign a parameter value and a constant to a table.
- `iterate_entity_attributes`: Iterates through a range of memory, checks certain attribute values, and updates a pointer or a value in a table if certain conditions are met.
- `update_entity_attribute_loop`: Iterates through a range of an array to update a memory-mapped attribute or pointer, and if a certain condition is met, updates a secondary table and calls two function pointers with a new coordinate or state.
- `scale_and_store_values`: loops through a range and scales a value from a source table before storing it in a destination table.
- `update_game_state_machine`: Updates a state-based or event-driven state machine by checking a provided input against a table and branching to specialized logic or resetting state flags.
- `load_from_table`: Loads a word from a table using an offset derived from another global word and stores it into a destination.
- `clear_memory_block`: Writes three words to specific offsets of a4, then enters a loop that clears a word at offset 336 of a structure pointed to by a0, where a0 is derived from a value in a5, and repeats this eight times.
- `loop_attribute_scaling`: Calculates a value by subtracting 32 from a word at offset -20056 of a4 and dividing by 8, uses the result to access a table via a0, and loops by decrementing the value at offset -20056 of a4 if certain conditions are met.
- `clamp_value_in_table`: Compares a word at a specific offset of a4 with 38; if it is greater than or equal, it branches; otherwise, it writes 42 to that offset, writes 42 to the offset immediately preceding it, and branches to another routine.
- `empty_c_function_5`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `perform_math_and_call_routines`: Calls a routine at an offset of a4, performs multiplication and subtraction to update a word at offset 14 of a5, and then calls another routine using a calculated address and several parameters from the stack.
- `search_table_for_pattern`: iterates through a table to find a specific pattern or value, returning a calculated index or pointer.
- `empty_c_function_6`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_offset_from_table`: Decrements a word at offset -17996 of a4, uses the result as an index into a table at offset -31368 of a4 to fetch a byte, and stores that byte at offset -20074 of a4.
- `compare_and_update_offset`: Compares a word at offset -4e5a of a4 to 160, and if it is not less than or equal, adds 14 to a word at offset -4e6a of a4 and sets a word at offset -2 of a5 before branching.
- `perform_multiple_scaled_calls`: Sets a word at offset -2 of a5, calls three different routines using values from a5 and a4, performs several unsigned multiplications to calculate offsets, and uses those offsets to call a fourth routine multiple times before branching to another routine.
- `execute_until_zero`: executes a function pointer repeatedly while certain global shorts are non-zero, then executes two more function pointers.
- `parse_string_delimiters`: Searches through a buffer for a specific character and replaces a delimiter character with a null terminator.
- `load_and_extend_word`: Loads a word from an offset in a4, extends it to a longword, and branches to another location.
- `update_values_from_a4`: Extends a byte from an offset of a4, pushes it to the stack, calls a routine, and then updates two different offsets of a4 based on a comparison between a byte at an offset of a4 and d0.
- `calculate_and_store_value`: Subtracts 3 from a word at an offset of a4, multiplies the result by 180, and stores the result at another offset of a4 before branching.
- `jump_via_table`: Compares d0 to 4, and if it is not less than 4, shifts d0 left and jumps to an address from a table using d0 as an index.
- `unstack_and_return`: Calls another routine, unlinks a5, and returns.
- `process_scaled_entity_data`: Calls scale_and_store_values and iterate_entity_attributes, then performs a division on a swapped word from offset -17848 of a4 to store a result at offset -4 of a5.
- `fetch_and_call_indexed_data`: Sets a word at offset -6 of a5, calculates an index using a value from a5, fetches a pointer and a word from a table, and calls a routine at offset -32490 of a4.
- `process_indexed_data`: Calculates an index by multiplying a value from a5 by 3, adding 3, and shifting left by 2, then uses this to fetch a pointer and other values from tables to call another routine.
- `copy_player_sprite_attributes`: Copies or copies-from a source buffer or table to a destination buffer using a nested loop structure to duplicate or modify sprite or object attribute data.
- `search_buffer_for_value`: Checks a buffer for a specific byte value and returns a value based on its position or a multiplier.
- `select_conditional_table_entry_2`: Calculates a value based on a global variable and a parameter, then uses a switch statement to return a value or a fallback based on specific bit checks.
- `select_conditional_table_entry`: checks specific bits in a byte and returns a word from a table or a fallback location based on those bits.
- `empty_routine_11`: the routine contains no instructions and performs no operations.
- `get_conditional_word`: Checks specific bits in a byte and returns a word from a table or a fallback location based on those bits.
- `read_offset_value`: Returns a word-sized value from a memory location calculated using a register and an offset.
- `update_offset_with_bounds`: Links a5, compares a word at offset -4ab8 of a4 to 3, and if it is not less than or equal, performs several comparisons and subtractions on values at offset -464c of a4 before moving a result to offset -8 of a5.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `update_offset_value`: Compares a value at offset -8 of a5 to 1 and branches if not equal; otherwise, it increments a value at offset -4ab8 of a4 and wraps it within a range of 18 if it exceeds that limit.
- `xor_table_value`: Loads a word from a table using an index derived from a value at offset 8 of a5, performs an XOR operation with a word at an offset of a4, stores the result back at that offset, and branches back to an earlier instruction in the routine.
- `update_game_state_logic`: Updates a global state machine by checking conditional flags and executing a sequence of specialized logic or pointer-based function calls.
- `calculate_bit_index`: Calculates a bitmask or bit index and a corresponding word or byte index from a provided value.
- `update_buffer_offset`: Updates a global variable or pointer offset based on a calculated value and checks it against certain bounds.
- `update_index_and_clamp_value`: Updates a global index by scanning a table for duplicate adjacent values, then clamps a secondary global value within a fixed range.
- `update_global_values`: Copies several dwords or words from a source table or pointer-based memory to a set of global variables and clears certain global variables to zero or one.
- `call_with_calculated_offset`: Calls a function pointer using a parameter and a value calculated from a table lookup and an offset-based calculation.
- `update_pointer_and_state`: Updates a pointer or a global word with a value from another memory location and conditionally sets certain global words based on a parameter.
- `clear_memory_and_assign`: Clears a range of bytes in a memory block, then calls another routine and stores its return value into a global variable.
- `execute_multiple_calls`: Executes a sequence of function calls, some of which pass a value retrieved from a global table or pointer-based memory.
- `set_constant_recursively`: Writes a constant value to a memory location and calls another routine recursively if a certain condition is met.
- `conditional_function_execution`: Calls a function, then checks if two global shorts are non-zero before calling another function that writes a constant to a memory location and passing a value from a global variable.
- `execute_set_constant`: Executes a function call, then calls another function to write a constant value to a global memory location, followed by a final function call.
- `return_no_op`: The routine has three instructions and its Ghidra pseudo-code is an empty body that only returns.
- `update_table_with_parameter`: Calls another routine with a constant, then performs a series of word-sized copies between a table and a global variable.
- `conditional_stack_processing`: Clears words at offsets 8 and 10 of a5 if they equal 64, pushes several values to the stack including a longword from an offset of a4, calls a routine, and then calls another routine if a comparison with a value from a4 fails.
- `load_and_branch_value`: Pushes a word onto the stack, calls another routine, and then loads a word from offset -4b22 of a4 into d0 before branching to another location.
- `call_text_routine`: Pushes a pointer to a location near the program counter and several values from a5 and a4 onto the stack, then calls another routine before branching to another location.
- `execute_scaled_calculations`: Performs multiple calculations involving multiplying values from a5 by 22 and adding them to table offsets, calls two different routines with these calculated values, and finally branches to another routine.
- `execute_scaled_calculations_branch`: Similar to R4, it calculates offsets using values from a5 multiplied by 22, calls two routines with these values, and ends with a relative branch to another routine.
- `decrement_and_branch_d0`: Tests d0, and if it is not zero, decrements it and branches to different locations depending on whether it equals 1 or 7, otherwise it branches to offset 12838.
- `call_offset_routine_2`: Pushes a word of 1 to the stack, calls a routine at an offset of a4, and returns.
- `update_buffer_from_table`: Clears a local buffer, then iterates through a range, reading values from a table and incrementing a byte in a buffer based on certain comparison logic.
- `compare_text_buffers_2`: Calls a routine, then compares a byte from a table at offset -19167 of a4 using an index from a global variable to a byte from another table at offset -19167 of a1, and returns based on the result.
- `prepare_series_text_call`: Moves two words from a4 to a5, pushes a pointer to the stack, moves a longword from a4 to the stack, and calls a routine at offset -7e24 of a4.
- `compare_text_buffers`: Loads words from offsets -4576 and -4574 of a4 into d0 and d1, fetches a byte from a table at offset -4adf of a4 using d0, and compares it to a byte from the same table using d1.
- `render_championship_series_text`: Calculates an offset into a text table using a value from a structure, calls render_series_text, and then conditionally updates a buffer using update_buffer_with_conditional_copy.
- `render_series_text`: Calls a routine with a pointer, then performs several lookups in tables using values from a5 to fetch data, calls another routine multiple times with these values, and finally calls a routine with a calculated value before branching.
- `update_buffer_with_conditional_copy`: Checks if a memory-to-register copy or a certain length is non-zero, then performs a series of conditional memory-to-memory or memory-to-register copies or clears a memory range.
- `execute_memory_operations_2`: Performs several memory-to-register or memory-to-memory copies using a pointer and a temporary variable, then updates a global variable base.
- `execute_memory_operations`: Performs several memory-to-register or memory-to-memory copies using a pointer and a temporary variable, then updates a global variable based on a comparison.
- `perform_memory_copy_operations`: Calls a function to copy a range of memory, and if successful, performs further copies or a copy that returns a value from a global variable.
- `copy_buffer_with_offset`: Copies dwords from a pointer-based table and calls another routine, then conditionally writes incremented bytes to a calculated memory offset.
- `accumulate_table_values`: Iterates through a nested loop to add values from one table into another table at calculated offsets.
- `update_table_with_max_value`: Iterates through a loop to compare values in a table and updates a global table with a new value if it is larger.
- `copy_and_parse_buffer`: Copies several values from a source table to a destination, then calls a routine to parse a buffer and potentially performs further memory copies.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: removes the stack frame with unlk, then returns to the caller.
- `find_marker_and_call`: Iterates through a table until a specific byte value is found, then calls another routine multiple times using the discovered offset or an unable to incremented offset.
- `populate_buffer_from_table`: If a certain condition is met, copies bytes from a table into a destination buffer using an offset, or otherwise checks a table and updates a local counter.
- `swap_array_elements`: Searches a table for a matching value and swaps two adjacent elements in that table if a certain condition is met.
- `map_buffer_indices`: Iterates through a buffer and, if a certain byte is non-zero, copies a byte from a source table to a destination table at a calculated offset.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_7`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `initialize_and_reorder_pixel_buffer`: Clears a secondary buffer and then reorders or copies pixel-related data into a new table structure based on conditional checks of existing values.
- `compare_and_recurse_table`: Iterates through a table and calls another routine if certain adjacent or nearby values in the table match.
- `compare_table_indices`: Compares values in a table using indices or a secondary table, and calls another routine if certain conditions or certain value comparisons are met.
- `reorder_table_by_value_comparison`: Clears a counter array and then iterates through a range of indices to compare values in a table, incrementing counters if certain conditions are met.
- `swap_table_values`: Reads two word-sized values from a table and writes them back to a different index in the same table, effectively swapping or overwriting values.
- `update_table_with_sum`: Calculates the sum of two short values from a table and passes this sum, along with a pointer and a constant, to a series of called subroutines.
- `process_pixel_value_differences`: Clears a memory buffer and then iterates through a table of byte values, comparing adjacent bytes and updating or decrementing values in a separate memory area based on the comparison results and a call to another routine.
- `find_matching_differences`: Iterates through a table and compares two values, storing an index and a difference into a destination table if a match is found.
- `modify_memory_via_pointer`: The routine performs a sequence of instructions that modifies a value in memory using a pointer held in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transfer_pointer_data`: Calls another routine, then copies a longword from a memory location pointed to by a value at a specific address into two different locations relative to a6, writes a word from a memory location to the address in a1, and performs a long jump.
- `set_bit_and_jump`: Sets bit 0 of a byte at the address in a6 and then jumps to a long address.
- `call_subroutine_at_offset_1f0`: Calls a subroutine located at a specific offset within the code segment.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:dawn#unpacked

- `initialize_system`: Calls several routines, modifies hardware registers including the custom chip and interrupt enable, and performs a series of conditional calls and memory operations before returning.
- `toggle_table_values`: Decrements a value at a fixed location and, if not zero, loads four 16-bit values from a table and negates them if they are negative at a specific destination.
- `initialize_registers_with_constants`: Performs a long sequence of bitwise OR operations to set specific bit patterns in registers d0 through d4.
- `loop_copy_data`: Checks a byte at a table offset, calculates a destination address, and enters a loop that calls another routine twice per iteration to process data, repeating 20 times.
- `loop_copy_data_pc`: Checks a byte at a table offset, calculates a destination address using a PC-relative offset, and enters a loop that calls another routine twice per iteration, repeating until a counter reaches 20.
- `batch_process_segments`: Calls another routine six times in succession, using different source and destination pointers and incrementing the destination pointer by 2 after each call.
- `store_values_to_stack_structure`: Writes several 16-bit constants and register values to specific offsets within a structure pointed to by a6, including storing a pointer from a0 and a1.
- `clear_and_store_values`: Clears several 16-bit and 32-bit values at offsets of a6, stores d1 at offset 66, and writes a pointer from a1 and register values from d2 to a structure.
- `transform_and_store_data`: Rotates d3 left by 4, performs an XOR with a constant, and stores the result and several other values and pointers into a structure at offsets 64, 66, 68, 70, 80, 84, 88, 100, 102, and 150.
- `update_table_index_and_copy_data`: Decrements a 16-bit value in a global variable and returns, otherwise it calls another routine, updates a global index, and calculates an offset into a table to call another routine twice with specific parameters for data copying.
- `prepare_pointer_and_call`: Clears d0 and d1, loads a word into d2, sets d3 to 14, loads a pointer from a global address into a0 with an offset of 2, and calls another routine.
- `copy_memory_block`: Loads three addresses using the program counter, moves a longword from the address in a0 to the address in a2, and then moves a longword from offset 4 of a0 to offset 4 of a2.
- `process_geometry_data_structures`: Loads two pointers from a table, calls another routine to resolve them, and then iterates through a loop of 18 elements to update structure geometry data by reading four 16-bit values and writing them to specific offsets within a structure.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.b) and then falls through into the code that follows, with no return of its own.
- `clear_data_registers`: Sets registers d0, d1, d2, and d3 to zero and then returns.
- `apply_clamped_offset`: Adds a 16-bit value from a0 to a location in a1, clamps the result to a maximum of 316, then performs similar addition and clamping operations for a second 16-bit value at an offset of 2 from the original pointers.
- `update_structure_geometry_data`: Calls another routine to compare values, then calculates a memory address based on a multiplication and offset, and writes several 16-bit and 32-bit values into a structure pointed to by a6.
- `clear_structure_fields_2`: Calls another routine, then clears several longwords and one word at specific offsets within a structure pointed to by a6.
- `wait_for_bit`: Enters a loop that tests bit 14 of a byte at offset 2 of a6 and returns when that bit is not set.
- `process_indexed_table_data`: Loads a word from a global address, masks it, and performs several operations including calling two different routines, a loop using a dbra instruction, and updating a global word and a counter.
- `copy_and_modify_bytes`: Loads six longwords from a list, calls another routine in a loop, and then stores several bytes into specific memory locations, some of which are incremented by 16.
- `process_data_block`: The routine performs a series of operations on data using a pointer in a register and calls another routine once before returning.
- `initialize_and_process_data_tables`: Calls another routine to clear values, then executes two loops that iterate through a table to load multiple longwords into registers and call another routine for each entry.
- `update_table_index`: If a word at a global address is not zero, it decrements that word and returns; otherwise, it sets that word to 87, increments a second global word, uses the incremented value shifted left by 5 to index into a table at a global address, and if the retrieved longword is not all ones, it copies four longwords from that table entry to four specific global locations.
- `calculate_scaled_value`: Sets a global word from d0, uses d2 to index into a table at a relative address, compares a word from that table with d1, and if d1 is not less than or equal to the table word, it updates the table word with d5, then calculates a value by multiplying a longword from a table by d4, shifting it right by 15 bits, adding d3, and storing the result in the address pointed to by a2.
- `process_list_elements`: Performs two loops using a dbra instruction; the first loop iterates 483 times to increment a pointer, and the second loop iterates 483 times to read a longword from a table, shift it right by 2 bits, and write it back to the table, finally storing 19957 in a global word.
- `update_state_and_table`: Sets several registers, calls another routine, performs bit tests and shifts on a value, and if certain conditions are met, calls another routine and copies bytes from a table into a structure.
- `process_input_and_loop`: Checks a bit in a hardware register, calls another routine, and then enters a nested loop structure that performs byte transfers from a table to a destination pointed to by a1 and a2.
- `initialize_data_structures`: Populates several memory locations with specific longwords and word values, using a combination of direct moves, swaps, and two loops that iterate based on d0 to write values into a table structure.
