# Game 1090 (red baron (1990)(dynamix)(disk 1 of 3)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01090). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1401792 top-level bytes in 38 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 276 of 276 named (0 author symbols), 276 of 276 described (42 by rule, 226 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 164327 of 164330 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 386879 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 171423 of 171423 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 25865 |
| code | 515078 |
| data | 814878 |
| padding | 11155 |
| unused | 34816 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:install | 10 | 45 | 2 |
| Run | 2 | 117 | 2 |
| More | 19 | 2290 | 76 |
| Avail | 11 | 376 | 12 |
| AddBuffers | 2 | 117 | 2 |
| Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:sim | 8 | 45 | 2 |
| Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:ps | 9 | 45 | 2 |
| Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:baron | 9 | 251 | 6 |
| Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:amigasnd.drv | 3 | 1820 | 112 |
| Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 163855 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2444 | 58 |
| Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:credits.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:volume.001 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 163856 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 226 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- red baron (1990)(dynamix)(disk 1 of 3)/Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:credits.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- red baron (1990)(dynamix)(disk 1 of 3)/Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:volume.001: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- red baron (1990)(dynamix)(disk 1 of 3)/Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red baron (1990)(dynamix)(disk 1 of 3)/Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:sim: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red baron (1990)(dynamix)(disk 1 of 3)/Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:ps: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red baron (1990)(dynamix)(disk 1 of 3)/Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:baron: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 386879 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_complex_data_structure` (More): Performs a series of nested operations on a structure in a2, including calling multiple subroutines to parse decimal strings, manage memory, and manipulate data fields based on conditional checks and loops. Kind: indirect function call/control flow obfuscation. Pass path: the address stored in unaff_A3.

## Functions


### Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:install

- `initialize_game_state`: Executes a sequence of function calls and checks a global pointer to swap a memory value if a specific constant is found.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 to d0 and then branches to another location.

### More

- `parse_and_load_config_file`: Parses a string into tokens, handling quoted strings and converting specific character sequences into control characters, then uses the resulting data to populate a structure and calls another routine to load associated file data.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function_2`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to recurse_with_stack_slot, then returns.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `parse_and_dispatch_entity_command`: Parses a command byte from a structure and uses a jump table to dispatch specific logic based on the value.
- `process_complex_data_structure`: Performs a series of nested operations on a structure in a2, including calling multiple subroutines to parse decimal strings, manage memory, and manipulate data fields based on conditional checks and loops.
- `initialize_entity_state`: Sets a 32-bit field at offset 60 and a 16-bit field at offset 94 of a structure to specific values, calls another routine with a pointer from offset 52, calls itself recursively if offset 100 is non-zero, and enters a loop that calls another routine while certain conditions regarding offset 60, offset 8 of a pointed-to structure, and offset 94 are met.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from the input pointer, stores the second result into a field of the pointed-to structure, and if a flag field at offset 4 is zero it increments a short located at an offset from a pointer stored in the structure.
- `parse_buffer_content`: Uses another routine to check a buffer, updates a size field, and scans for a newline or form-feed character while potentially modifying a flag or string content.
- `process_entity_list`: Determines a loop limit based on fields in a passed structure and iterates to call another routine using a pointer to a table and a global variable; it also recursively calls itself, calls another routine with a pointer to a stack buffer, and calls a library routine to reschedule.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Calls another routine with zero, then with a parameter, then with a pointer from offset 16 of a structure, a global variable, and a parameter, conditionally calls another routine with the same arguments but without the parameter, calls another routine with two arguments, calls another routine with a constant, calls another routine with a parameter, and returns the result of the call involving two arguments.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at offset 152 of a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: The routine calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: The routine calls a drawing routine with a fixed or zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `draw_text_line`: Uses a handler pointer to call drawing primitives while iterating over a text buffer, tracking line count and stopping if a certain character or flag is encountered.
- `dispatch_recursive_pointer`: The routine calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition or partial way is met, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `recursively_process_buffer_segments`: If a field at offset 120 is non-zero and a field at offset 116 is not 4096, it calls itself recursively with a pointer into the structure and a size from offset 120.
- `update_scroll_window`: Calculates a target position based on provided offsets and compares it against the current window bounds.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_uppercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `process_message`: Recursively processes a structure by invoking itself on various fields and string arguments, conditionally updates fields at offsets 8, 6, and 7, and performs a loop based on a return value.
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
- `compute_modular_result`: Computes a 32-bit result by repeatedly doubling and subtracting the divisor when the running remainder exceeds it, otherwise it performs a 16-bit modulo on the low half of the dividend and returns the combined quotient and remainder.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_at_pointer`: Stores a byte from register d0 into the memory location pointed to by a3 and returns.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: The routine attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: The routine removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Avail

- `parse_command_line_arguments`: Parses a command string by splitting it into tokens, handling quoted strings and converting specific character sequences into control characters, then stores the resulting pointers in a structure.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `sum_memory_region_sizes`: Iterates through a linked list of memory regions and sums the size of any region whose attribute flags match a provided mask.
- `print_memory_availability_table`: Checks a command string or argument to determine if the output should be filtered by chip, fast, or total memory types, then calculates and prints the available, in-use, and maximum block sizes for the selected memory type or a combination of types.
- `call_reschedule_2`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_2`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot_2`: Pushes two values and a pointer to a stack location onto the stack, calls another routine, and returns.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `verify_stack_buffer_checksum`: Invokes a function pointer from a global variable and computes a rolling checksum over a stack-allocated buffer until a null terminator is found.
- `store_byte_at_pointer_2`: Stores a byte from register d0 at the memory location pointed to by a6 and increments the pointer.

### AddBuffers

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of incrementing words into a buffer.
- `move_and_branch_2`: Moves the contents of register d1 to d0 and then branches to another location.

### Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:sim

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.

### Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:ps

- `free_blocks_and_restore_patch`: Calls two unnamed routines, loops to free five memory blocks, and restores a saved word in a table if a specific masked pattern is matched.
- `decrement_d7_by_d6`: Decrements a 16-bit value in d7 by the value in d6 using a loop, then returns.

### Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:baron

- `free_memory_and_restore_patch`: Calls two unnamed routines, loops to free five memory blocks via a library call, and restores a saved word in a table if a specific masked pattern is matched at a calculated offset.
- `decrement_until_minus_one_3`: Copies a 16-bit value from register d6 to d7 and decrements d7 in a loop until it reaches negative one, then returns.
- `process_decompressed_data`: Calls a routine multiple times to decompress data, updates a table using results from another routine, and conditionally calls a routine via a pointer in a global variable.
- `load_address_and_constant`: Loads an address into register a1 using a PC-relative offset and moves the constant two into register d0.
- `decompress_variable_length_data`: Decompresses a data stream using a variable-length encoding scheme where bytes are expanded based on bit-shifting and carry-propagation logic, then stores the result into a destination buffer.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:amigasnd.drv

- `execute_indirect_call`: Performs an indirect function call using a pointer derived from a global variable and a register value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `check_status_bytes`: Compares bytes from two different offsets in a structure with d0 and d1, and if they match, checks a third offset and potentially writes a byte to a structure at offset 528 of a4.
- `write_status_bytes`: Writes the byte one to an offset in a1 determined by d2, then writes the byte zero to another offset in a1, and branches to another location.
- `increment_and_pop_stack`: Increments a 16-bit value in register d2, and if it is not equal to four, it branches to another location; otherwise it pops five longwords from the stack into registers a1, a0, d3, and d2.
- `push_registers_and_swap_d1`: Pushes three longwords onto the stack, swaps the high and low bytes of d1, and if the resulting byte is not zero, calls another routine and then branches to another location.
- `compare_and_reset_table_entry`: Compares a byte in d0 and the high byte of d1 against two consecutive bytes in a table at offsets 520 and 524 of the structure in a4; if they match, it clears a byte at offset 528 of the structure in a4 and calls two other routines.
- `increment_and_check_index`: Increments a byte in d2, compares it to 4, and if not equal it branches; otherwise it calls a routine, compares d2 to 255, and if not equal it calls another routine before popping values from the stack and returning.
- `update_audio_parameter_byte`: Checks if a value in a register matches a specific constant, then performs bitwise manipulation on that value and stores the result into a byte at an offset of 392 from a base address in a4, using a byte from d0 as an index.
- `swap_and_call_routine_2`: Compares d1 to 64, and if it is not equal, branches to another location; otherwise it swaps the high and low bytes of d1, calls another routine, swaps the bytes again, and branches to another location.
- `store_swapped_byte`: If d1 is not equal to 1, it swaps the high and low bytes of d1, stores the resulting byte into a table at offset 424 of the structure in a4 using d0 as an index, and then swaps d1 again before branching.
- `swap_and_call_routine`: Compares d1 to 75, and if not equal it branches; otherwise it swaps d1, calls a routine, swaps d1 again, and branches to another location.
- `search_audio_data`: Compares a byte in d0 against a sequence of bytes in a global structure and updates a local variable if a match is found and the corresponding byte at offset 524 is not 255.
- `store_byte_at_offset`: Stores a byte into a memory location calculated from a base address, an offset, and a word value.
- `update_audio_register`: Calculates a 16-bit value based on d1 and d0, stores it at an offset in a4 determined by d0 and d1, and then iterates through a four-byte sequence to check if a byte at offset 520 of a4 matches d0, calling another routine if a match is found.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `get_and_set_byte`: Returns a byte from a memory location and, if the input byte is not negative one, updates that location with the input byte.
- `get_and_set_word`: Returns a word from a memory location and, if the input word is not negative thirty-two thousand seven hundred sixty-eight, updates that location with the input word.
- `update_byte_via_pointer`: Reads a byte from a memory location and conditionally overwrites it with a value from a register if that register does not contain a specific negative value.
- `update_audio_index_2`: Performs a series of comparisons and conditional branches to calculate a value based on bytes from a table and updates a field in a structure.
- `update_audio_index`: Compares a word from a table at offset 532 of a4 with d5, and if d5 is not smaller, performs a series of swaps and shifts on d5 and d2 before potentially calling another routine and writing a byte to a location in a0.
- `update_audio_channel_volume`: Iterates through a 4-byte sequence in a structure to count occurrences of a specific byte value, adds this count to a value retrieved from a table using a byte from a register as an index, and then updates a target byte in a register by calling another routine once or twice depending on whether the new value is smaller or larger than the current value.
- `call_routine_twice`: Subtracts d1 from d4, moves the result to d1, calls a routine, calls another routine, and returns.
- `modify_audio_channel_3`: Checks for a specific byte value in a structure at offset 520 of a4, and if found, increments two counters and performs a subtraction and a call to another routine if a comparison with a byte at offset 456 of a4 succeeds.
- `modify_audio_channel_2`: Subtracts d4 from d3, stores the result in a structure at offset d0 of a0, and calls another routine before branching to another location.
- `increment_and_pop_stack_2`: Increments a 16-bit value in d0, compares it to 16, and if not equal, branches to another location; otherwise it pops five values from the stack into a0, d4, d3, d1, and d0 before returning.
- `modify_audio_channel`: Searches for a specific byte value in a structure, updates it, and increments a value at a calculated memory offset.
- `compare_and_clear_byte`: Compares a byte at an offset from a pointer in a4 with a value in d1, and if they are equal, sets that byte to zero.
- `subtract_audio_byte`: Subtracts a byte from a byte at offset d0 of a structure pointed to by a0 using d1 as the subtrahend, then branches.
- `process_audio_threshold_2`: Subtracts a byte from a structure, conditionally updates a field in a structure at offset 520 of a4, and performs a series of checks and branches involving a table at offset 540 of a1.
- `process_audio_threshold`: Compares a word from a table at offset 532 of a4 with d6, and if d6 is not smaller, performs swaps and shifts on d5 and d3 before potentially calling another routine and writing bytes to a0 and a2.
- `execute_empty_routine`: This routine consists of ten instructions but performs no observable operation, as it does not modify any registers, memory, or hardware state before returning.
- `update_status_table`: Stores a byte at a calculated offset and, if that byte is zero, iterates through a table to update specific flags based on a matching index.
- `reset_and_search_audio_parameters`: Clears several bytes and words in a table pointed to by a4, then searches a linked list of structures for a range match against a value in d1.
- `increment_pointer_and_branch`: Adds 20 to the address in a1 and branches to another routine.
- `update_audio_parameters`: Stores several pointers and values into a table at offsets in a4 based on d2, then updates specific bytes in a4 using values derived from d1 and a table, and calls another routine.
- `call_audio_reset`: Calls a routine and then restores six registers from the stack before returning.
- `clear_audio_channel_data`: Calls another routine and then resets several memory locations and offsets within a global structure to zero or 255 based on an index in d2.
- `fetch_audio_parameters`: Retrieves several pointers from a table using an index in d2, performs arithmetic, and calls subroutines to process values based on a check of a table entry at offset 640 of a4.
- `return_stack_value`: Returns a 24-bit value formed by combining a constant with a 4-byte field taken from a 32-bit value located on the stack at a fixed offset.
- `update_audio_voice_counters`: Iterates through a sequence of four entries and increments specific values in a table based on the contents of a structure.
- `update_audio_envelope_state_2`: Updates an audio structure by modifying a state byte and an amplitude byte based on values from lookup tables, and calls modify_audio_data if a specific condition is met.
- `update_audio_envelope_state`: Updates an audio structure by modifying a state byte and an amplitude byte based on values from lookup tables.
- `increment_audio_data`: Copies a byte from a table to a structure, adds another byte from a different table to that same structure, and conditionally updates a byte in a structure at offset d2 of a6.
- `decrement_conditional_byte`: Checks if a byte at offset d2 of a structure in a6 is 1, and if so, checks if a byte at offset d2 of a structure in a0 is 0; if both conditions are met, it decrements the byte at offset d2 of the structure in a0.
- `decrement_audio_data`: Copies a byte from a table to a structure, subtracts a byte from a different table from that structure, and conditionally updates a byte in a structure at offset d2 of a6.
- `decrement_audio_byte`: Compares a byte at offset d2 of a structure in a6 with 2, and if they are equal, moves a byte from offset 13 of a structure in a1 to a location at offset d2 of a structure in a2.
- `decrement_if_match`: Compares a byte at offset d2 of a6 with 3, and if it matches, checks if a byte at offset d2 of a0 is 0; if so, it decrements that byte and branches to another location.
- `modify_audio_data`: Subtracts a value from a byte in a structure using an offset from another table and calls itself recursively if a condition is met.
- `calculate_scaled_volume`: Loads three bytes from different offsets within a structure pointed to by a4 using d2 as an index, and if the third byte is non-zero, proceeds to a branch; otherwise, it clears d4.
- `check_volume_scaling`: Compares a byte in d4 with 0 and branches accordingly.
- `calculate_volume_scaling_3`: Multiplies the lower bytes of d3 and d4, divides the result by 63, and then scales a value from a global structure at offset 662 of a4 by the result.
- `calculate_volume_scaling_2`: Multiplies the lower bytes of d3 and d4, divides the result by 63, and then scales a value from a global structure at offset 662 of a4 by four to determine a final value.
- `calculate_volume_scaling`: Calculates a value based on the product of two inputs and stores the result in a global variable.
- `compare_audio_parameter_sequence`: Compares a byte at an offset in a structure to d1, and if they are equal it branches; otherwise it increments d2 and compares it to 4 before branching again.
- `update_audio_parameters_2`: Updates several values in a structure at a4 using an index in d2, including setting specific bytes at offsets 524, 564, and 568, and stores three calculated pointers into a table at offsets 136, 264, and 600.
- `configure_audio_parameters`: Sets a value in a structure at offset 18 of a3, updates a pointer and a 16-bit value in a structure at a4 using an index in d2, and then populates several fields of a structure pointed to by a pointer retrieved from a table using bytes from a2 before calling another routine and potentially setting a byte to 255 in a structure at a4.
- `reset_audio_hardware_state`: Calls a routine to reset the audio hardware and returns a value from a memory location via a pointer.
- `clear_audio_channel_data_2_2`: Compares a longword at offset d2 of a1 to a0, and if they are not equal, increments d2 and continues a loop until d2 reaches 16.
- `clear_audio_channel_data_2`: Shifts the value in d2 right by 2, calls another routine, and then pops two words from the stack into a1 and d2 before returning.
- `search_table_for_match`: Compares a longword at an offset in a1 with a0, and if they are not equal, loops through a table at offset 288 of a4 by incrementing d2 until it reaches 16 or finds a match.
- `clear_and_return_byte_2`: Sets a word at a fixed offset to one, retrieves a byte from a table using an index, clears that byte in the table, and returns the byte shifted.
- `clear_and_return_byte`: Sets a word at a fixed offset to one, retrieves a byte from a table using an index, clears that byte in the table, and returns the byte shifted left by sixteen bits.
- `fetch_indexed_value`: Swaps the high and low bytes of d2, compares it to 192, and if not equal, retrieves a word from a structure at offset 488 of a4 using the modified d2 as an index, then jumps to another routine.
- `fetch_indexed_byte`: Compares a 16-bit value in d2 to 192, and if not equal, swaps d2 and retrieves a byte from a structure at offset 408 of a4 using the modified d2 as an index into d0.
- `fetch_table_byte_4`: Compares a 16-bit value in d2 to 176, and if equal, performs byte swaps on d1, checks if the new d2 is 7, and if so, retrieves a byte from a table at offset 392 of a4 using a masked d0 as an index.
- `fetch_table_byte_3`: Compares a 16-bit value in d2 against 64, and if not equal, branches; otherwise it masks d0 and retrieves a byte from a table at offset 440 of a4 using d0 as an index.
- `fetch_table_byte_2`: Compares d2 against 1, and if equal, retrieves a byte from a table at offset 456 of a4 using a value derived from d0 and d2, then branches to another location.
- `fetch_table_byte`: Compares d2 against 75, and if equal, compares d1 against 255, then retrieves a byte from a table at offset 456 of a4 using the lower byte of d0.
- `find_matching_byte`: Searches through a table at an offset in a4 to find a byte matching d0, incrementing a counter in d3 by two each iteration until the counter reaches 4, then returns the counter in d0.
- `return_constant_value`: Returns a constant 16-bit value.
- `update_buffer_pointers`: Stores a longword from a0 into offset 132 of a4, and adds the value of a4 to the longwords at offsets 140, 172, 204, and 236 of a4 before calling another routine.
- `get_concatenated_data`: Concatenates two 16-bit values from a table into a single 32-bit result.
- `allocate_and_initialize_audio_buffers`: Calls another routine, then calls a routine via a pointer from a global variable to allocate memory, and subsequently stores the returned address and several calculated offsets into a structure in a4 before checking if the address is zero.
- `open_audio_device`: Sets up a structure with specific values and a pointer, then calls a routine via a library vector; if the result is zero it continues to set up further structures and calls more routines, otherwise it returns a specific value.
- `initialize_audio_interrupt_handlers`: Initializes four structures with specific byte values and function pointers, calls a routine to register these structures, writes a value to the INTENA register, and calls another routine.
- `release_audio_resources`: Writes a value to the INTENA register, calls four routines via pointers, and then conditionally calls two more routines via pointers based on values in a4.
- `reset_audio_hardware`: Clears two bytes in a table at offsets 5358 and 5354 of a4, resets a hardware register at offset 8 of a calculated address, calls another routine, and sets a byte at offset 5301 of a4 to 1.
- `update_audio_registers`: Loads a word and a byte from a table into specific hardware registers and clears a third register at an offset in the structure in a6.
- `calculate_audio_offset`: Saves several registers to the stack, checks a value in a table at offset 640 of a4, and if non-zero, performs arithmetic involving division and multiplication to retrieve a value from a table at offset 4852 of a4.
- `calculate_scaled_offset_2`: Performs a series of arithmetic operations using values from a2 and a4 to calculate a value in d1, and retrieves a byte from a table at offset 4208 of a4 into d4.
- `adjust_pointer_by_scaled_offset`: Calculates an offset by subtracting a value at offset 16 from a value at offset 0 within a structure pointed to by a3, scales the result by dividing by 16 and multiplying by a value at offset 8 of a1, and subtracts this product from the original value at offset 0.
- `calculate_scaled_offset`: Subtracts d5 from a value at an offset in a3, divides the result by 16, multiplies it by the negative of a value from a1, adds the result to d3, and performs comparisons on the new d3 value.
- `pop_arguments_from_stack`: Loads six values from the stack into registers and moves a seventh value from the stack into a register before returning.
- `write_audio_register`: Writes a 16-bit value to a hardware register at an offset calculated from a value in d2.
- `write_audio_register_alt`: Writes a 16-bit value to the audio hardware register at an offset calculated from a value in d2.
- `decrement_and_update_audio`: Decrements a byte in a table and, if it reaches a specific value, updates hardware registers with values from a table.
- `update_audio_sample_buffer`: Copies a large block of sample data from a source buffer to a destination structure and updates audio channel parameters, including volume and active status, based on calculated values.
- `update_audio_channel_active`: Sets a byte at an offset in a4 to 1, calculates a value based on a subtraction and a loop, stores that value in a structure, and then stores a 32-bit value from a table into an audio hardware register.
- `store_table_word`: Stores a 16-bit word from a table into a memory location.
- `update_audio_sample_buffer_2`: Copies a large block of data from a source buffer to a destination buffer within a structure, then updates a sample offset and volume parameters in a global audio control table.
- `update_audio_channel_inactive`: Sets a byte at an offset in a4 to 1, calculates a value based on a subtraction and a loop, stores that value in a structure, and then stores a 16-bit word from a table into an audio hardware register while setting a byte at an offset in a4 to 0.
- `store_word_from_table`: Reads a 16-bit word from a table using an offset in a register and stores it into a memory location at an offset of 4 from a pointer in a register, then returns.
- `reset_audio_channel`: Saves registers to the stack, loads two addresses from a table, checks a byte at the first address, and if it is not 255, sets d2 to 0 and calls another routine.
- `set_buffer_byte_high`: Compares a byte at the address in a1 to zero, and if it is non-zero, sets the byte at the address in a0 to 255.
- `update_audio_buffer_zero`: Sets d2 to 0, compares the byte at the address in a0 to 0, and if not equal, calls another routine.
- `trigger_interrupt_request`: Calls another routine, sets bit 7 of the interrupt request register, and returns with d2 restored from the stack.
- `reset_audio_channel_one`: Saves registers to the stack, loads two addresses from a table, checks a byte at the first address, and if it is not 255, sets d2 to 1 and calls another routine.
- `check_and_set_audio_byte`: Compares a byte at the address in a1 to zero; if it is not zero, it writes the value 255 to the address in a0 and branches to another routine, otherwise it returns.
- `update_audio_buffer_one`: Sets d2 to 1, compares the byte at the address in a0 to 0, and if not equal, calls another routine.
- `set_audio_interrupt_flag_high`: Calls another routine, writes 256 to the interrupt request register, and returns with d2, a4, a1, and a0 restored from the stack.
- `reset_audio_channel_two`: Saves registers to the stack, loads two addresses from a table, checks a byte at the first address, and if it is not 255, sets d2 to 2 and calls another routine.
- `check_and_reset_audio_byte`: Compares a byte at a memory location to zero; if it is not zero, it writes the value two hundred fifty-five to the address in a0 and then branches to another routine, otherwise it returns.
- `update_audio_buffer_two`: Sets d2 to 2, compares the byte at the address in a0 to 0, and if not equal, calls another routine.
- `set_audio_interrupt_request`: Calls another routine, writes 512 to the interrupt request register, and returns using values popped from the stack into registers d2, a4, a1, and a0.
- `reset_audio_channel_three`: Saves registers to the stack, loads two addresses from a table, checks a byte at the first address, and if it is not 255, sets d2 to 3 and calls another routine.
- `check_and_reset_audio_buffer_byte`: Compares a byte at the address in a1 to zero; if it is not zero, it writes the value 255 to the address in a0 and branches to another routine, otherwise it branches to another routine.
- `update_audio_buffer_three`: Sets d2 to 3, compares the byte at the address in a0 to 0, and if not equal, calls another routine.
- `reset_audio_interrupt_flag`: Calls another routine and then writes 1024 to the interrupt request register.
- `write_hardware_constants`: Writes specific constant values to two hardware registers.
- `process_data_table`: Iterates through a table of pointers in a0, following each pointer to an offset of 10 to check for a terminator, and if a specific byte at offset 8 of a target structure is zero, it sets it to 65535 and modifies a byte at an offset calculated from a value at offset 12.
- `decrement_and_compare_word`: Increments a word in d0, compares it to 512, and branches if not equal; otherwise it pops a word from the stack into d0, pops several registers, and returns.

### Red Baron (1990)(Dynamix)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_bit_manipulation`: Repeatedly performs bitwise OR operations with a constant on the contents of register d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_file_buffer_and_parse`: Initializes a structure by clearing a range of memory, then calls parse_command_string to process a string into a table of pointers.
- `execute_callback_chain`: Calls several functions via pointers stored in a structure and calls free_linked_list if a specific field is non-zero.
- `initialize_graphics_and_intuition`: Attempts to open the graphics and intuition libraries, and if successful, copies a string from a table into a local buffer and calls a routine to manage display contexts.
- `process_input_stream`: Calls a routine, then compares the result to 10 or 27; if neither matches it decrements a 16-bit counter in a global variable and, if not below zero, reads a byte from a table at a global pointer, increments that pointer, and calls another routine.
- `process_linked_list_nodes`: Calls a routine via a pointer from a global variable, stores the result in a global variable, and then iterates through a linked list to copy pointers into a global array.
- `update_linked_list_data`: Iterates through a linked list, copying a longword from a table into a field at offset 4 of the current list element, then calls a routine via a pointer from a global table and updates a global pointer.
- `jump_to_address`: Performs an absolute jump to the address stored at a fixed address.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_indirect_function`: Loads a function pointer from an offset relative to a value stored on the stack and executes it.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `validate_pointer_and_execute`: Calls another routine to retrieve a pointer, checks if bit 3 of the byte at offset 1 of that pointer is set, and if so calls a second routine with a value of 2.
- `cleanup_display_context_2`: Calls two routines via pointers in a structure, then conditionally calls a third routine if specific fields are zero, and finally performs a series of memory operations and calls involving a structure and a global pointer.
- `process_record_data`: Iterates through a null-terminated string, decrements a 16-bit counter in a global variable, and either writes bytes to a table at offset 52 of a4 or calls another routine to handle data when the counter is too low.
- `parse_command_string`: Parses a string into a table of pointers, handling quoted substrings and expanding escape sequences like newline or carriage return.
- `manage_list_node`: Checks a flag in a structure and, if certain conditions are met, calls another routine to allocate memory, updates two pointers in the structure with the result, and if the result is zero, sets a value in a global variable and returns -1; otherwise it clears two fields and returns 0.
- `manage_record_data`: Performs multiple operations involving seeking, reading, validating, and allocating handles for records within a table by manipulating structure flags, updating pointers, and calling various subroutines to manage memory and data integrity.
- `move_word_from_pointer`: Moves a 16-bit word from the address pointed to by a2 into d6.
- `decode_variable_length_byte`: Parses a variable-length encoded stream by reading bytes from a buffer pointer, handling escape sequences and length prefixes.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `copy_string_with_limit`: Copies a null-terminated string from a source pointer to a destination pointer, truncating the copy if it exceeds a specified maximum length and ensuring null-termination.
- `copy_string_to_buffer`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `insert_into_ordered_list`: Inserts a block of memory into a linked list at a position determined by a size value, potentially splitting an existing node.
- `execute_io_with_pointer`: Saves registers, calls a routine, and if the result is non-zero, calls another routine using a pointer from the first result, then checks a 16-bit value at offset 436 of a4 to determine the final return value.
- `free_linked_list`: Iterates through a linked list and calls a function for each node before clearing the list pointers in a structure.
- `insert_memory_block`: Calls another routine to free or manipulate a block at a pointer from a global variable, then calls another routine to allocate a new block of a specified size, and if successful, writes the size into the new block and returns a pointer to it.
- `validate_nested_pointer_and_check_flag`: Calls another routine to retrieve a pointer, then calls a second routine using a value from that pointer; if the first call fails or if a specific 16-bit value at offset 436 of a structure is zero, it returns negative four, otherwise it returns the result of the second routine.
- `allocate_and_link_node`: Allocates a block of memory of a specified size and inserts it into a linked list structure.
- `manage_ordered_list`: Manages an ordered list by searching for a position based on a size, potentially splitting nodes, allocating new memory, and calling routines for fixed-point math and list insertion.
- `update_memory_pointer`: If a pointer in d0 is not zero, it calls another routine and stores the resulting pointer, decremented by 2, into a global variable at offset 940 of a4.
- `traverse_linked_list`: Iterates through a linked list starting at offset 12 of a structure, pushing the address of the next element and the contents of offsets 4 and 8 of each element onto the stack, then calls another routine.
- `format_path_string`: Calculates the total length of a list of strings, populates a structure with metadata including a count and flags, and constructs a single space-separated string by copying the input strings into a new memory block.
- `resolve_directory_path`: Calls a routine to get the current directory, then iterates through a provided string to find a directory separator or null terminator.
- `process_linked_records`: Saves registers, checks flags in a structure at a3, and if specific conditions are met, calculates a size, calls another routine, and then calls a third routine using a value from the stack before iterating through a linked list.
- `execute_io_callback`: Checks a global variable, calls another routine, and if the result is -1, calls a second routine from a table and updates a global variable at offset 436 of a4.
- `process_io_with_logic`: Calls another routine and uses a value from a table to determine a result, which is then modified based on the value of a third parameter before returning.
- `execute_io_operation`: Checks if a value at offset 460 of a structure in a4 is non-zero, calls another routine, clears a 16-bit field at offset 436 of the structure, and then calls a routine via a pointer at offset 1012 of the structure, potentially updating offset 436 and offset 1012 with a new value.
- `cleanup_display_context`: If a field at offset 10 of a structure is nonzero, it calls a callback through a global table, writes 255 to offset 8 and -1 to offset 20 of the structure, then calls two more callbacks from the structure and a third from a global table.
- `call_conditional_handler`: Checks if a longword at offset 108 of a structure pointed to by a4 is non-zero, then calls a routine via a pointer at offset 1016 of that same structure.
- `compute_fixed_point_product`: Takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value by multiplying the high 16-bit parts, the low 16-bit parts, and combining the cross terms.
- `normalize_signed_division`: Handles the signs of two 32-bit integers by negating them as needed and calls a normalization routine to perform division.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method and returns a 64-bit result with the quotient in the high 32 bits and the remainder in the low 32 bits.
- `compare_and_branch`: Compares a byte at offset -1 of a structure pointed to by a0 with 79, and if the byte is greater than or equal to 79, it sets d7 to 78 and d0 to 78 before branching.
- `copy_and_write_data`: Copies a block of bytes using a loop, clears a byte in a structure, calls a library routine, and if a specific field in the returned structure is non-zero, calls another library routine twice with different arguments and returns 0 or -1.
- `call_library_function`: Loads a library address into a6, calls a routine via a pointer, stores the result in a structure, and branches based on the result.
- `execute_callback_with_args`: Stores a value into a structure field, then pushes several arguments onto the stack to call a routine, returning zero if the result is one and negative one otherwise.
- `return_zero_constant`: The routine returns a constant zero value without performing any other operations or memory accesses.
- `create_message_node`: It obtains a global variable and calls another routine through a table.
- `get_table_entry`: Sets a value at offset 436 of a structure to zero, then checks if a value from the stack is within a specific range and points to a non-zero entry in a table to return a pointer, otherwise it sets an offset in the structure to 9 and returns zero.
- `check_and_set_signal`: Calls a library routine to check a status, and if a specific bit is set and a pointer in a global variable is non-zero, calls the routine pointed to by that global variable and clears it.
- `set_bits_and_byte`: Performs several bitwise OR operations on d0, writes 48 to a byte at the address in a0, and sets bit 3 of a byte at the address in a1 before incrementing a1.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_status_flags`: Performs several bitwise OR operations on d0 and writes specific bytes to offsets of a structure pointed to by a0, while also moving bytes from a0 to the stack.
- `modify_register_and_branch`: Performs bitwise OR operations on d0 and branches to another location.
- `update_global_parameters`: Calls three routines via pointers, updates a global pointer with a value from a structure, and finally calls a routine using a pointer derived from a value at offset 4 of a structure.
- `update_global_params`: Stores a value into a global variable and calls two other routines using pointers from a second parameter.
- `set_status_bit`: Sets bit 0 of a byte at an offset in a structure and returns a value with bits 3, 4, and 5 set.
