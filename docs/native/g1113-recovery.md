# Game 1113 (tv sports basketball (1990)(cinemaware)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01113). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1827156 top-level bytes in 51 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 27 of 27 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 678 of 678 named (0 author symbols), 678 of 678 described (213 by rule, 425 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4261 of 4261 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 400361 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16393 of 16393 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 626903 |
| code | 54232 |
| data | 773915 |
| padding | 12170 |
| unused | 359936 |

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
| TV Sports Basketball (1990)(Cinemaware)[a](Disk 1 of 2).adf:bb | 3 | 7843 | 388 |
| TV Sports Basketball (1990)(Cinemaware)[a](Disk 1 of 2).adf:bootblock | 1 | 11 | 1 |
| TV Sports Basketball (1990)(Cinemaware)[a](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 72 | 72 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 453 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2273 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1068 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| TV Sports Basketball (1990)(Cinemaware)[a](Disk 1 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[a](Disk 2 of 2).adf:Fluff.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[a](Disk 2 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 183 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 520 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 711 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 2 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 92 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- tv sports basketball (1990)(cinemaware)[a](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[a](Disk 1 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[a](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[a](Disk 2 of 2).adf:Fluff.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[a](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[a](Disk 2 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 400361 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_callback_and_return` (More): Calls another routine, then calls a second routine, and finally calls a function pointer stored in a register before returning. Kind: callback-based integrity or environment check. Pass path: the instruction following the indirect call (*unaff_A3)().

## Functions


### More

- `parse_command_string`: Parses a command string by iterating through characters to extract tokens, handling quoted strings and converting specific character sequences into newline or escape characters, then stores the resulting tokens into a structure.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with specific parameters and then loops through a buffer to check if each byte falls within several allowed ranges, returning 0 if a byte is invalid or the loop exceeds 79 iterations.
- `process_entity_input_state`: Sets a field at offset 60 of the structure to 1 and enters a loop that calls another routine until it returns a non-zero value.
- `execute_callback_and_return`: Calls another routine, then calls a second routine, and finally calls a function pointer stored in a register before returning.
- `initialize_entity_state`: Sets a 32-bit field at offset 60 and a 16-bit field at offset 94 of a structure to 1 and 0 respectively, calls another routine with a pointer and zeroed arguments, stores the result in a field at offset 76, calls itself recursively if a field at offset 100 is non-zero, and enters a loop that calls another routine while certain conditions regarding a field at offset 60 and values within a table are met.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from the input pointer, stores the second result into a field of the pointed-to structure, and if a flag field is zero it increments a short located at an offset from a pointer stored in the structure.
- `parse_buffer_content`: Calls another routine twice to check a buffer, then scans the buffer for a newline or form-feed character, and if a form-feed is found, modifies the buffer content and sets a flag at offset 94 of a structure.
- `process_entity_list`: Determines a loop limit based on fields at offset 164 and offset 102 of a structure, then iterates to call another routine, a loop that calls another routine using a pointer from a table and a global variable, and calls another routine using a global variable and the structure.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Runs a sequence of calls to another routine using arguments from a parameter block and a global variable, returning the result of one of those calls.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at offset 98 from a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: The routine calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: The routine calls a drawing routine with a fixed or zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `draw_text_line`: Uses a handler pointer to call drawing primitives while iterating over a text buffer, tracking line count and stopping if a certain character or flag is encountered.
- `dispatch_recursive_pointer`: The routine calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition or partial way is met, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `recursively_process_memory_segments`: If a field at offset 120 is non-zero and a field at offset 116 is not 4096, it calls itself recursively with a pointer into the structure and a size from offset 120.
- `update_scroll_window`: Calculates a target position for a window based on provided offsets and compares it against the current window bounds.
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
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment`: Stores a byte from register d0 into the memory location pointed to by a3 and increments a3.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: The routine attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: The routine removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch_2`: Copies a longword from d1 to d0 and then branches to another location.

### Assign

- `find_max_and_prepare_buffer_4`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with sequential words.
- `transfer_register_and_branch_3`: Moves a longword from register d1 to register d0 and then performs a short branch to another routine.

### EndCLI

- `find_max_and_prepare_buffer_5`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with sequential words.
- `transfer_register_and_branch_4`: Copies the contents of register d1 into register d0 and then performs a short branch to another location.

### Dir

- `find_max_and_prepare_buffer_6`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with a sequence of incrementing words.
- `transfer_register_and_branch_5`: Moves the contents of register d1 into register d0 and then performs an unconditional short branch to another location.

### Echo

- `find_max_and_prepare_buffer_7`: Walks a linked list to find the maximum value in a field, then writes this value to a structure and fills a buffer with sequential words.
- `transfer_register_and_branch_6`: Moves a 32-bit value from register d1 to register d0 and then performs a short branch to another location.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Execute

- `find_max_and_prepare_buffer_8`: Walks a linked list of entries to find the maximum value stored in a field, then writes this value to a structure and fills a buffer with sequential words starting from a constant.
- `transfer_register_to_register`: Moves a 32-bit value from register d1 to register d0 and branches to another routine.

### Copy

- `find_max_and_prepare_buffer_9`: Walks a linked list of entries to find the maximum value stored in a field and, if a condition is met, writes a small header and fills a buffer with sequential words.
- `move_longword`: Copies a longword from register d1 to register d0 and then branches to another location.

### Makedir

- `find_max_and_prepare_buffer_10`: Walks a linked list of entries to find the maximum value stored in a field and, if a condition is met, writes a header and fills a buffer with sequential words.
- `transfer_longword`: Moves a 32-bit value from register d1 into register d0 and performs a short branch.

### CD

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value stored in a field and, if a condition is met, writes a header and fills a buffer with sequential words.
- `transfer_register_and_branch`: Moves the contents of register d1 into register d0 and then performs a short branch to another location.

### EndIf

- `clear_d0_register`: Returns immediately after clearing register d0.

### If

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value stored in a field, then if a condition is met, writes a header and fills a buffer with sequential words.
- `transfer_register_and_branch_2`: Moves the contents of register d1 into register d0 and performs a relative branch to another routine.

### IconX

- `parse_command_line_arguments`: Parses a string of command-line arguments by iterating through characters to identify space-separated tokens or quoted strings, handling escape sequences for newlines and escape characters, and populates a structure with the resulting pointers and metadata.
- `update_state_and_invoke_callbacks`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start by checking a parameter and a byte in a structure, then proceeds to allocate memory, parse decimal values, and perform various library calls including icon and exec functions.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: This routine allocates a 260-byte buffer and walks a linked list of entries, copying the entries whose name bytes are above 31 into it and passing them to helper routines.
- `concatenate_strings_and_call_helper`: Copies a null-terminated string into a destination buffer, appends a second null-terminated string to that buffer, and then calls another routine with the buffer pointer and the total length of the combined strings.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to another routine.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated character string by checking for an optional leading plus or minus sign and iterating through numeric characters to calculate the value.
- `call_supervisor_2`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr_2`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule_2`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule_2`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_2`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception_2`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable_2`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry_2`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_compare_strings_with_stack_pointer`: Passes two values and a pointer to a stack location to compare_strings_with_wrap and then returns.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `invoke_function_and_checksum_stack`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_and_increment_2`: Stores a byte from register d0 into the memory location pointed to by a3 and increments the pointer by one before returning.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `write_byte_to_pointer`: Writes the byte contained in register d0 to the address held in a3, increments a3, and then returns.

### Disk-Validator

- `format_and_populate_string_table`: Converts a numeric value into a character representation and stores it in a table, then enters a loop to initialize a bitmask array and performs bitwise operations to populate a data structure based on values retrieved from a global variable.

### TV Sports Basketball (1990)(Cinemaware)[a](Disk 1 of 2).adf:bb

- `branch_to_initialization`: Performs a short branch to another routine.
- `update_string_metadata`: Compares two values, copies a sequence of words from a source to a destination, and updates a string or buffer with a length and certain characters.
- `swap_linked_list_nodes`: Checks a pointer against a global value, swaps two adjacent pointers in a linked list, and updates a global pointer to point to the new head or a node.
- `process_queue_and_delay`: Iterates up to two times to call a function via a pointer, checks a value at an offset from the result, calls another routine with a specific value, and clears a memory location before calling a library function with a constant.
- `allocate_and_initialize_audio_buffers`: Allocates two memory blocks using a library function, populates a table with incrementing values, and initializes a sequence of data structures by writing specific constants and a global variable pointer into a buffer.
- `copy_data_table`: Copies thirty-two words from a table into a destination buffer pointed to by a parameter.
- `update_buffer_recursively`: Recursively updates a buffer or a table entry with a value from a parameter, or updates a pointer-based destination if a certain condition is met.
- `setup_stack_frame`: Sets up a stack frame and branches to another location.
- `update_table_recursively`: Recursively updates a table by incrementing a counter and modifying values through a pointer until a termination condition is met.
- `wait_for_value_match`: Enters a loop that continues until the contents of two memory locations are equal.
- `rotate_bits_in_memory`: Calls a library function, stores the result in a global variable, adds a constant, and then performs a bitwise rotation or shift-based update to that global variable.
- `calculate_bit_shift_count`: Calculates the number of leading zeros in a 32-bit value using a loop, then uses that count to index into a table and performs a bitwise AND with the result of another routine.
- `return_offset_value`: Returns a 32-bit value stored at an offset of minus eight from a pointer in a stack frame.
- `branch_to_subroutine`: Calls a subroutine and then branches to another location.
- `decrement_and_execute_list`: Decrements two global short values, updates a third global short based on a subtraction, and executes a series of functions via a linked list.
- `update_global_state`: Copies a 32-bit value from one global variable to another and updates three global shorts or bytes with constant values.
- `insert_linked_node`: Inserts a new node into a linked list if the second element of the provided structure is zero.
- `update_pointer_from_table`: Checks if a value at an offset of a pointer in a5 is zero; if not, it compares a value at an offset of a4 with the pointer in a5, and if they match, it copies a longword from the location pointed to by a5 into a location relative to a4.
- `transfer_pointer_and_clear_source`: Copies a longword from a location relative to a4 into a location relative to a5, then checks if the value at the new pointer matches a value at offset 8 of the structure in a5; if they are not equal, it copies the value at the new pointer back to the source location and branches to another routine, otherwise it branches to another routine.
- `assign_pointer_and_clear`: Copies a pointer from one location to another and then clears the value at the offset of that pointer.
- `reset_global_pointer`: Calls another routine with specific arguments if a global variable is non-zero, then clears that global variable.
- `update_blt_registers`: Updates several bitplane-related registers and a DMA controller register using values or calculated bitmasks while checking a global flag.
- `restore_registers_and_return`: Restores registers d4 through d6 and a2 through a3 from the stack, unlocks a semaphore in a5, and returns.
- `calculate_blit_parameters_2`: Calls a routine that updates several fields in a structure pointed to by a5, including a bit-shifted value, a counter, and a value calculated from offset 4.
- `calculate_blit_parameters`: Updates several fields in a structure pointed to by a5, including a bit-shifted value, a counter, and a calculated value based on offsets 4, 10, 12, 14, 16, and 18, then enters a loop that iterates through a table pointed to by a5 plus 8.
- `update_blit_parameters_2`: Iterates through a data structure to calculate and update values used for Blitter operations.
- `update_blit_parameters`: Iterates through a data structure to calculate and update values used for Blitter operations.
- `configure_blt_area_transfer`: Calculates source and destination dimensions and offsets to configure the Amiga Blitter hardware registers for a block transfer.
- `restore_registers_and_return_2`: Restores registers from the stack, unlinks a5, and returns.
- `initialize_input_state`: Sets certain memory or hardware registers to a fixed value and fills a buffer with a constant value using a loop.
- `execute_indirect_calls`: Pushes four words from a structure in a5 onto the stack, calls a routine using a pointer relative to a5, and then calls another routine using a pointer relative to a4 with a parameter relative to a4.
- `call_indirect_function`: Calls a function pointer located at an offset from a global pointer.
- `clear_and_combine_bytes`: Clears two bytes at specific offsets within a table entry and returns a value constructed from those bytes and a third byte.
- `setup_table_iteration`: Saves registers to the stack, initializes two registers with zero and three, and branches to another routine.
- `iterate_and_clear_table`: Iterates through a table, clearing bytes and calculating a bit-shifted value based on table contents.
- `clear_table_bytes`: Clears certain bytes in a table and returns a value constructed from those bytes.
- `save_registers_and_jump`: Saves registers d4 through d5 and a5 onto the stack, clears d4 and d5 with small constants, and performs a short branch to another routine.
- `clear_and_shift_bit_table`: Iterates through a table, clearing bytes to zero and updating a bit-shifted value in a secondary or adjacent memory location.
- `process_linked_list_nodes`: Iterates through a linked list, calling a function with certain parameters and potentially calling it a second time if a bit or a global flag is set.
- `initialize_buffer_structure`: Calls a function to get a pointer, then populates a buffer with values from a global variable and a parameter.
- `execute_loop_calls`: Iterates through a range defined by a parameter and calls a function using a value constructed from bit-shifted components.
- `transform_nibble_data`: Iterates through a table of words to perform bitwise manipulations on nibbles and stores the results in a local buffer.
- `execute_sequence_with_checks`: Calls a routine repeatedly until a certain value is returned, then updates a global variable with a constant value.
- `parse_data_stream_subsequences`: Iterates through a data buffer using a pointer and a counter, implementing a state machine that reads bytes to determine subsequence types.
- `parse_compressed_data_stream`: Reads a byte from a table via a pointer, checks its high bit to determine a command type, and executes a switch-case logic that extracts multiple subsequent bytes or increments a data length counter.
- `increment_sequence_index`: Loads a longword from an offset of negative 4 from a5, adds 2 to it, stores the result at an offset of negative 18666 from a4, clears a byte at an offset of negative 19528 from a4, and branches to another routine.
- `update_and_process_sequence`: Increments a longword at an offset of negative 19450 from a4, checks if a longword at an offset of negative 19466 from a4 is greater than zero, calls another routine if it is not, decrements the value at offset negative 19466 from a4, moves a pointer from offset negative 19450 from a4 into a0, copies a byte from the address in a0 to an offset of negative 19464 from a4, and branches to another routine.
- `parse_data_stream_sequence`: Iterates through a data stream by reading bytes from a table using an incrementing index and stores them into a structure at specific offsets.
- `parse_compressed_data_stream_2`: Reads a sequence of bytes from a table using an incrementing index and uses a state machine to decode them into a structure.
- `extract_data_stream_sequence`: Reads a sequence of bytes from a table using an incrementing index and stores them into a structure at specific offsets, while managing a recursion depth counter and a loop that processes data based on a state machine.
- `decode_bitstream_sequence`: Iterates through a data stream using a state machine to decode variable-length bit patterns.
- `parse_data_stream_state_machine`: Iterates through a data buffer using a pointer and a counter, processing bytes through a state machine that updates various control values and calls another routine to fetch more data when a counter reaches zero.
- `process_data_stream_sequence`: Iterates through a data buffer using a counter and a pointer, then enters a state machine that reads bytes from a table to determine subsequent actions, including calling another routine to fetch more data.
- `process_entity_state_updates`: Iterates through a list of structures to update internal state values based on specific command bytes.
- `process_data_stream`: Enters a loop that clears a global variable, calls a routine to retrieve data, and if successful, performs several nested calls to manipulate and validate the retrieved data against a specific constant.
- `call_entity_flag_callback`: Loads a pointer from an offset of minus 19504 from a4, checks if it is zero, and if not, pushes a value from that pointer onto the stack and calls another routine using that value as a pointer.
- `iterate_entity_state_flags`: Iterates through a list of entity structures, checking if bit 0 is set in a flag field at offset 8 of each structure, and calls another routine if the bit is set.
- `increment_and_store_pointer`: Increments a global counter and, if it wraps to zero, calls a function to update a pointer, potentially resetting that pointer to a new value from a table.
- `update_animation_pointer_2`: Calls another routine to update a pointer in a table.
- `update_animation_pointer_subsequence`: Calls another routine with a pointer and two zero arguments, stores the result in a field at offset 8 of a structure pointed to by a value at offset 68 of another structure, and if that field is non-zero, sets a field at offset 4 of that same structure to the value of the field at offset 8 plus 8.
- `update_animation_frame_pointer`: Calls parse_data_stream_subsequences to retrieve a value, stores it in a table, and if the value is non-zero, increments the data at the address pointed to by that value by twelve.
- `allocate_and_load_subsequence_assets`: Calls parse_data_stream_subsequences to populate a data buffer, then iterates through a bitmask to allocate memory for individual components.
- `process_data_subsequences`: Calls a routine with a value from a global variable, stores the result in a structure at offset -76 of a pointer in a5, and if the result is non-zero, stores it in a table at offset 8 of a structure and calls another routine three times with values from offsets 4, 8, and 24 of the result.
- `process_subsequence_data_stream`: Calls parse_data_stream_subsequences to evaluate a data buffer, then uses the result to update a value in a structure and optionally executes a loop calling another routine with incrementing values.
- `transfer_data_to_table`: Pushes a value onto the stack, calls a routine with a value from a global variable, and then stores the returned value at offset 8 of a structure pointed to by a value at offset -68 of a5 before branching.
- `clear_and_transfer_data`: Clears two stack slots, calls a routine with a value from a global variable, and then stores the returned value at offset 8 of a structure pointed to by a value at offset -68 of a5 before branching.
- `render_blt_animation_sequence`: Processes a data stream to determine animation types and updates a pointer table, then performs a series of Blitter operations to draw graphics using parameters such as source, destination, and size.
- `update_subsequence_buffer_and_timer`: Determines a subsequence type based on a bit flag, copies a data block to a local buffer, and calculates a new timer value by subtracting a calculated duration from an existing value.
- `update_animation_pointer`: Updates a pointer in a table by adding a bit-shifted value from a register to an existing value.
- `update_entity_state_and_animations`: Iterates through an entity list to update internal state values and animation pointers based on command bytes.
- `update_clamped_input_vectors`: Splits a 16-bit input value from a global variable into two 8-bit components and calculates the difference from their previous values stored in a structure.
- `update_clamped_values`: Updates two values in a structure by retrieving data from tables based on an index, applying clamping logic to ensure the results stay within specified minimum and maximum bounds.
- `call_empty_routine`: Calls another routine with a constant value of fifteen.
- `execute_conditional_callbacks`: Checks several global pointers and, if they are non-zero, executes the routines or library functions they point to.
- `execute_recursive_calls`: Calls three different routines via pointers on the stack, then calls a fourth routine, clears a longword on the stack, and calls a fifth routine before returning.
- `initialize_library_pointers`: Loads library base addresses into global variables and checks if certain library functions return zero, potentially triggering a routine that clears an array or updates a global variable.
- `invoke_copy_data_table_with_offset`: Calculates a source address by adding an offset to a global variable and passes this address and a parameter to a routine that copies a table of words into a destination buffer.
- `iterate_and_call_function`: Iterates thirty-two times, calling a function with a global pointer and a value retrieved from a pointer-indexed table.
- `clear_global_pointer_target`: Clears a byte at a location derived from a global pointer and a pointer-indexed table, then calls another routine.
- `execute_loop_calls_2`: Calls a routine twice using a range or certain values, passing a destination buffer and a range or certain values.
- `call_function_with_split_parameter`: Calls a function with the constant value 10 and calls another function using a value split from a single parameter and a constant.
- `process_string_table_entry`: Uses a parameter to calculate an offset into a table, retrieves a pointer, and calls a library function using a value from a global variable.
- `update_pointer_from_string`: Retrieves a pointer or identifier from a string-based table and stores the result in a global variable, with a conditional check for a zero return.
- `traverse_entity_list`: Recursively calls another routine if a certain bit is set in a field within a list of structures.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `call_function_with_global_pointer`: Calls a function using a value stored at a global pointer location.
- `call_function_with_global_pointer_alt`: Calls a function using a value stored at a global pointer location using a different function pointer.
- `call_function_with_indirect_values_2`: Calls a function using a global pointer and then calls another function using two values retrieved from memory locations pointed to by global variables, with the order of arguments depending on a bit in the first parameter.
- `call_function_with_indirect_values`: Calls a function using two values retrieved from memory locations pointed to by global variables.
- `call_function_with_parameter`: Calls a function using a global pointer and a provided parameter.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `execute_conditional_draw`: Calls a routine with a parameter and a constant, then conditionally calls another routine to perform drawing or data operations based on the return value.
- `execute_single_function_call`: Pushes a constant, a pointer from offset 8 of a5, and a pointer from offset minus 24736 of a4 onto the stack, then calls a routine at offset minus 32384 from a4.
- `update_structure_member_values`: Pushes three values onto the stack to call a function using data from a structure in a5 and a table in a4.
- `branch_conditionally`: Subtracts a constant from d0, and if the result is zero, branches to one location, otherwise it branches to another location.
- `perform_multiple_function_calls_2`: Executes a sequence of four function calls that manipulate values stored in a structure, including clearing a value and copying data between.
- `perform_multiple_function_calls`: Executes a sequence of four function calls that manipulate values stored in a structure, including clearing a value and copying data between locations.
- `set_structure_bounds_and_clear`: Calls a library function to create a structure, clears a memory range, and then calls another routine using a pointer or identifier retrieved from that structure.
- `update_global_with_sum`: Calculates a sum involving a global variable and a value from a library call, then stores a bit-manipulated version of that sum into a global variable.
- `accumulate_global_value`: Performs a complex arithmetic loop that accumulates values from a global variable and other memory locations into a global variable.
- `return_uninitialized_stack_value`: This routine returns an uninitialized value from the stack.
- `increment_global_with_sum`: Calculates a sum involving a global variable and certain memory locations, then adds a repeated global variable value and that sum to a global variable.
- `initialize_system_state_2`: Resets several global variables to zero or a constant and executes a sequence of indirect function calls and conditional branches.
- `execute_callback_list_callback`: Executes a routine or a sequence of routines retrieved from a table or a global pointer-based list.
- `call_wrapper_routine`: Calls another routine and returns.
- `load_and_call_function`: Loads two 32-bit values from a table into global variables and then calls a function pointer.
- `initialize_animation_parameters`: Calls a routine with a specific constant, executes an indirect function call, and then calls another routine with a different constant.
- `initialize_system_state`: Resets several global flags or counters and executes a sequence of library or internal routines before calling another routine with a global variable.
- `reset_global_flags`: Calls a routine with a constant, resets a global word to zero, executes two indirect calls, resets another global byte to one, and calls a final routine.
- `trigger_replay`: Calls another routine with a specific address and a global variable.
- `execute_multiple_initializations`: Calls a routine with a constant, then executes two subsequent indirect routine calls.
- `conditionally_initialize_system`: Checks if a 16-bit value at an offset in a structure is zero, then calls several routines, potentially calling another routine with a calculated value, and finally calls itself recursively.
- `set_half_state_flag`: Checks a parameter, calls a routine with a constant if non-zero, sets a global byte to zero, executes an indirect call, and calls another routine with a constant.
- `initialize_memory_tables`: Sets certain memory locations to specific constant values using a loop that runs four times.
- `sum_table_bytes`: Iteratively calls a function to fetch values from a table and accumulates certain bytes into a local variable based on provided parameters.
- `update_tables_with_calculated_offsets`: Writes certain values to a table, calculates and stores new pointer-based offsets into a table, and calls a routine with a pointer from a table.
- `initialize_game_state`: Resets several memory-mapped or global arrays to zero and configures a table of parameters based on a specific or previously set value.
- `check_and_combine_bytes`: Calls a routine twice to check if returned values are zero, then either calls another routine with two parameters or sets a byte in a global variable and returns.
- `execute_sequence_of_function_pointers`: Executes a series of indirect function calls using addresses or offsets derived from a global pointer, and writes a constant value to a memory location.
- `dispatch_conditional_pointers`: Checks a 16-bit value at an offset from a pointer; if it is zero, it calls four different routines via pointers in a structure, otherwise it calls a single routine via a pointer.
- `update_conditional_state`: Calls a routine with a constant, conditionally sets a global byte, and updates a global word or pointer based on the result of an indirect call or a comparison.
- `call_offset_routine`: Calls another routine using a value calculated from a pointer and an offset.
- `recurse_with_parameter`: Checks a parameter and calls itself recursively with that parameter and a global variable.
- `call_library_with_parameters`: Calls a library function using a pointer and certain values retrieved from a table.
- `update_conditional_table_entry`: Initializes a memory range to a constant value and then conditionally updates a table entry based on a comparison with a library function return value.
- `validate_range_and_branch`: Compares a 16-bit word at a negative offset from a pointer against 32 and 0, then branches to another routine if the value is outside that range, otherwise it branches to another instruction within the routine.
- `store_word_to_offset`: Stores a 16-bit word from an offset of 8 in a structure into a memory location at a negative offset from a pointer.
- `recursively_call_routine`: Checks if a parameter is non-zero and, if so, calls itself with that parameter.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_table_data`: Iterates through a loop to copy or update several values in a table based on conditional checks and pointer-based lookups.
- `test_byte_in_table_and_return`: Tests a word at a negative offset from a pointer in a4; if the value is zero, it branches to another routine, otherwise it clears d0 and returns.
- `check_byte_and_branch`: Loads a byte from an offset of a structure in a5, uses it as an index to test a byte in a table at an offset from a4, and branches based on the result.
- `process_data_loop_2`: Increments a 16-bit value at an offset from a pointer, compares it to decimal 4, and loops to perform data operations involving a table and a function call until the counter reaches 4.
- `process_data_loop`: Performs a loop that increments a 16-bit value at an offset from a pointer, compares it to decimal 4, and either writes a negated byte to memory or calls a routine via a pointer.
- `write_negated_byte_to_memory`: Calculates a byte value by adding 1 to a byte from offset 9 of a5, negating it, and storing it at a memory location determined by adding a byte from offset minus 8 of a5 to an offset of minus 18750 from a4.
- `increment_and_branch_on_counter`: Increments a 16-bit value at an offset from a pointer, compares it to decimal 4, and branches to another routine or loops back to itself.
- `initialize_array_bounds`: Clears a range of memory and then populates a secondary table with index-based offsets.
- `initialize_bounds_logic`: Compares a word from a table in a4 with a byte from a4, and if the comparison condition is met, writes two values to a structure in a5 before branching to another routine.
- `initialize_array_bounds_3`: Iterates through a loop, checking a byte at an offset from a4 against a value at an offset from a5, and performs memory writes and routine calls based on comparison results.
- `initialize_array_bounds_2`: Clears two bytes and a word at specific offsets from a pointer, calls another routine, and then either sets a word in a global variable or branches to another routine based on a byte check.
- `clear_word_at_offset`: Clears a 16-bit word at a specific offset from a pointer.
- `update_index_with_function_results`: Iterates through a loop, calling a function via a pointer and storing the result or its negation into a table.
- `load_and_call_pointers`: Calls two library functions, clears a table, loads several values from a library-referenced structure, and calls another routine for each loaded pointer.
- `execute_callback_list`: Iterates through a table of four pointers and calls another routine for any non-zero pointer, then calls two library functions.
- `process_recursive_parameters`: Executes a function call using a value from a pointer and then calls itself recursively with modified parameters.
- `dispatch_indirect_calls`: Calls three different routines using a pointer from a table, passing various combinations of provided parameters.
- `dispatch_handler_chain`: Calls a routine, then conditionally performs further calls or a loop of eight calls based on values in a structure and a table, finally calling another routine and a third routine.
- `toggle_structure_bounds_via_pointer`: Calls a routine via a pointer using a value from a structure, then calls another routine twice with incremented values from offsets 10 and 12 of a structure, followed by a call via a pointer using a value from offset 22 of that structure, and finally calls the second routine again with the original values from offsets 10 and 12.
- `execute_parameterized_calls_loop`: Calls a routine via a pointer using a value from a table, then executes a loop eight times calling another routine with parameters derived from a structure and two tables, before calling two more routines.
- `decrement_and_branch`: Decrements d0 multiple times, branching to different locations depending on whether d0 reaches zero after subtracting one, one, or six.
- `execute_parameterized_calls`: Calls a routine via a pointer using a value from a table, then calls another routine using four values retrieved from a structure.
- `call_wrapper_routine_2`: Calls a function via a pointer with three parameters, calls a second function via a pointer, and then calls another routine using the first parameter, the fourth parameter, and the result of the second function.
- `call_stack_manipulation_routines`: Calls a routine that increments two 16-bit values at specific offsets relative to a6 and then calls a routine that loads a 16-bit word from an offset of minus four from a pointer in a5.
- `increment_stack_values`: Increments two 16-bit values at specific offsets relative to a6 and then branches to another location.
- `return_value_from_stack`: Loads a 16-bit word from an offset of minus four from a pointer in a5, then returns.
- `call_with_offset_parameters`: Calls a routine via a pointer using a single parameter, then calls another routine using four parameters where two are sums of input values.
- `call_via_offset_table`: Retrieves bytes from a table using an offset and calls a routine twice if a specific byte in the table is non-zero.
- `dispatch_table_attribute_updates`: Reads a 10-byte entry from a table using an index and extracts various byte and word values to pass as arguments to three different function pointers.
- `update_table_data_2`: Extracts multiple values from a table using an index and offsets, then calls three different routines with these values and updated pointers.
- `call_subroutine_via_pointer`: Calls a routine through a pointer stored in a global table using a passed parameter.
- `execute_indirect_call`: Fetches a function pointer from a table using an offset from a base address and calls it with a parameter.
- `execute_indirect_function_call`: Fetches a function pointer from a memory location and calls it using a value passed as an argument.
- `clear_memory_buffer`: Uses nested loops to set a range of memory to zero.
- `update_table_values_via_index`: Calculates a value from a table using a byte index and an offset, then adds this value to three separate 16-bit locations within a structured data block.
- `add_to_table_value`: Calculates an address in a table using an index and an offset, then adds a value from a register to the memory at that address.
- `call_external_function`: Calls a routine through a pointer at an offset of negative 32386 from a6, stores a byte from offset 23 of a5 into a global variable at offset negative 13962 from a4, and then branches to another location after extending a word from offset 24 of a5 into a longword in d0.
- `test_and_clear_longword`: Tests a longword at an offset of a5; if it is non-zero, it branches to another routine, otherwise it clears a longword at an offset of a5 and sets a longword at an offset of a5 to one.
- `negate_and_call_callback`: Tests a longword at an offset in a structure, negates it if it is not zero, calls a routine using a pointer from the structure, and stores the negated result of that call back into the structure.
- `execute_callback_chain`: Calls several routines via pointers stored in a table, stores a returned value in a structure, and performs conditional calls based on values in that structure.
- `store_and_call_concatenated`: Calls a routine, stores a returned value in a global variable, and calls another routine using a concatenated 32-bit value.
- `call_parameterized_routines`: Sets a byte in a table, calls two routines via pointers using values from a structure, and conditionally updates a byte in a table.
- `decrement_and_branch_on_d0`: Decrements a longword in d0 and branches based on whether the result is zero or one, otherwise it branches to another location.
- `pop_and_or_registers`: Pops three 32-bit words from the stack into d0 and performs a bitwise OR operation on d0 with zero.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a2).
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a7).
- `transfer_stack_and_branch`: Clears a 16-bit value at offset minus 2 of the structure in a5, checks the value at offset 12 of a5, and if it is not zero, moves the cleared value to offset minus 4 of a5 before branching to another routine; otherwise it branches to a different location.
- `calculate_interpolated_values`: Iterates through a nested loop structure to perform calculations based on values retrieved from a table.
- `initialize_global_state`: Calls a routine with two arguments, stores the return value in a global variable, and then clears two small arrays of four words each to zero.
- `call_routine_with_constant`: Calls a routine using a value stored in a global variable and a constant integer as an argument.
- `enable_dma_channel`: Modifies the DMA controller register to enable a DMA channel.
- `update_dma_config`: Writes a value from a memory location to a hardware register and sets another hardware register to a constant value.
- `update_coprocessor_hardware`: Calls three separate routines to write a value from a memory location to a hardware register and set another hardware register to a constant value.
- `enable_dma_channels`: Sets a global variable to a value that enables DMA channels via the DMACON register.
- `write_hardware_registers`: Writes two word-sized values into a hardware register or a memory-mapped area using an offset derived from a parameter.
- `lookup_table_entry`: Loads a word from an offset of a5, extends it to a longword, shifts it left by two, and tests a byte in a table at an offset from a4.
- `lookup_and_call_function`: Clears a word, sets a word to decimal 40, fetches a pointer from a table using a value from a structure, and calls that routine.
- `write_configuration_values`: Writes three 16-bit values to a table in a4 and two 16-bit values to a structure in a5, then branches to another routine.
- `write_and_call_routine`: Writes the value 2 to a memory location calculated using a value from a structure in a5 and then calls another routine with a value from offset minus 4 of a5.
- `branch_to_offset_target`: Performs a short branch to another instruction offset.
- `jump_via_table`: Compares d0 to decimal 18, and if it is not less than 18, shifts d0 left by one and jumps to an address from a table using the new d0 value.
- `branch_to_offset`: Performs a short branch to a specific instruction offset.
- `clear_and_set_pointers`: Clears a range of memory, calls a library function, and stores return values from other routines into a global variable.
- `setup_stack_and_branch`: Sets up a stack frame and branches to another routine.
- `update_table_from_lookup`: Iterates through a table to update a destination memory area using values fetched from a secondary lookup table.
- `call_function_conditionally`: Checks a global variable and calls a function pointer with a specific value if that variable is non-zero, then calls the same function pointer with a different constant.
- `decrement_counters_and_bounds`: Decrements a global counter and, if it reaches zero, updates a loop count and decrements values in a table based on conditional checks of other table entries.
- `update_pointer_and_call`: Calls a function pointer, calls another function pointer with zero, and copies a value from a memory location to a global variable.
- `reset_function_and_copy`: Calls a function pointer with zero and copies a value from a memory location to a global variable.
- `reset_state_and_call_subroutines`: Clears a value in a global memory location and then executes two previously stored function pointers, passing a zero or no argument to the first.
- `execute_function_pointer`: Calls two routines and then calls a function through a pointer stored at a specific offset from a base address in a6.
- `configure_dma_and_clear_flag`: Clears a global flag and calls a routine to enable a specific number of DMA channels, followed by a routine that writes a value to a memory location.
- `check_and_return`: Checks a long value at offset minus 18790 of a4 and returns if the value is not zero; otherwise it returns.
- `call_routine_with_pointer`: Calls another routine twice using values fetched from a pointer stored in a structure.
- `format_numeric_digits_and_dispatch`: Converts a signed integer into individual decimal digits stored in a local array, then iterates backward through the array to call a routine using values from a table, unless a specific condition sets the digits to 10 or 32.
- `modify_table_and_dispatch_callback`: Iterates backward through a list to update short values in a table with 10 or 32 based on specific conditions, then iterates through the list again to call a function pointer with two arguments for every entry whose table value is not 32.
- `iterate_and_call_on_condition_2`: Iterates through a list to update values in a table and then calls a function pointer with two arguments if a value in the table does not match a specific constant.
- `iterate_and_call_on_condition`: Iterates through a list and calls a function pointer with a value from a memory location if a certain condition in a lookup table is met.
- `negate_and_move_word`: Checks a word at offset 8 of the structure in a5 and if it is negative, moves the negated value into d0 and returns; otherwise it returns.
- `move_and_branch`: Moves a word from an offset of a5 into d0 and branches to another routine.
- `initialize_and_branch`: Sets up a stack frame, clears a word at a negative offset from a5, and branches to another routine.
- `copy_byte_sequence`: Copies a sequence of bytes from one memory area to another using a loop and returns the final index.
- `process_and_update_byte`: Clears a byte at offset 17706 of the structure in a4, enters a loop that calls another routine and updates a global variable with the result, and conditionally updates a byte in a4 based on values from a structure pointed to by a global variable.
- `clear_and_branch`: Clears a byte at offset 17706 of the structure in a4 and a word at offset 2 of the structure in a5, pushes a long value from offset 13418 of the structure in a4 onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine if the word at offset 2 of the structure in a5 is zero, otherwise it branches to a different location.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `update_position_and_clear_byte`: Calculates a new coordinate using a pointer-based offset and calls a routine with the result, then potentially clears a byte in a buffer if a certain condition is met.
- `decompose_and_render_value`: Calls a routine with a parameter, then enters a loop that iterates six times to call another routine and a third routine using values from a table or a global variable.
- `parse_and_open_file`: Copies a string from a buffer into a global structure until a colon or null terminator is reached, then attempts to open the file.
- `execute_nested_loops_2`: Performs a series of nested loops that compare values in a table, update a global variable, and execute indirect function calls.
- `call_and_unlk`: Calls another routine, sets d0 to 1, removes the stack frame held in a5, and returns.
- `update_status_and_call_routine`: Writes a word to an offset in a structure pointed to by a4, calls another routine, pushes a pointer to a stack, copies a longword from an offset in a5 to the stack, and calls another routine.
- `execute_nested_loops`: Executes a series of nested loops that call other routines and perform conditional updates to a global variable and a library function call based on comparisons of values in a table.
- `decrement_and_branch_2`: Subtracts decimal 80 from d0, branches if d0 is zero, otherwise subtracts decimal 2 from d0, branches if d0 is zero, otherwise branches to an earlier instruction.
- `branch_to_offset_5766`: Performs a short branch to a location within another routine.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `evaluate_state_condition`: Checks a parameter, calls another routine, and updates a global variable with the result.
- `handle_disk_swap_and_state_transition`: Checks a flag to determine if a disk swap or state change is required, then updates a global state variable and executes a sequence of conditional routine calls or an infinite loop.
- `write_table_values`: Iterates up to five times to write values from a table into a destination via a routine call, then resets a global variable if certain conditions are met.
- `execute_indirect_function`: Loads a function pointer from a memory location and calls it using a constant argument.
- `get_and_return_value`: Calls a routine and returns a value stored in a global variable.
- `check_and_return_immediately`: Sets up a stack frame, tests a longword at an offset in a structure, and if it is zero, clears d0 and returns; otherwise it returns.
- `initialize_structure_values`: Writes several words to various offsets in a structure in a4, including copying a word from a structure in a5 to a structure in a4, and sets several specific word values at offsets within that structure.
- `update_counters_and_bounds`: Decrements a global counter and, if it reaches zero, sets two specific values; otherwise, it updates a value by adding it to another, clamping the result between two other values and storing the result in two different locations.
- `conditional_execute_routines`: Checks several global conditions and, if satisfied, executes two different routines based on a returned value and a counter check.
- `update_game_state_parameters`: Reads a value from a global structure, adds seven to it, and uses the result to perform various conditional updates to multiple fields within the same structure.
- `update_status_values`: Stores the word 1 at offset -13348 and the word 20 at offset -13350 of the structure in a4, then branches to another routine.
- `update_status_values_alt`: Stores the word 1 at offset -13348 and the word 40 at offset -13350 of the structure in a4, then branches to another routine.
- `update_structure_fields`: Calculates a value based on a division of a word from a structure, then conditionally writes several words to different offsets of a structure using values from a table.
- `update_structure_from_table`: If a value at offset -15444 of the structure in a4 is greater than 2, it updates several fields in the structure in a4 using values from a table indexed by a value from a structure in a5.
- `modify_structure_values`: Writes a word to an offset in a structure, calculates an offset using a value from a structure, moves a word from a table to a structure, and conditionally writes several other values to the structure.
- `set_structure_constants`: Stores the word 1 at offsets -13356 and -13352, and the word 388 at offsets -13354 and -13350 of the structure in a4, then branches to another routine.
- `decrement_and_test_d0`: Tests d0 and performs a series of conditional subtractions of various amounts, branching to other routines if d0 reaches specific values, otherwise it unlinks a5 and returns.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `fill_buffer_with_count`: The routine iterates a specified number of times, writing the current loop counter value as a byte to a destination buffer and incrementing the pointer.
- `initialize_structure_state`: Loads a pointer from an offset of 8 in a5 into a2, clears a word at offset -2 of the structure in a5, and branches to another routine.
- `branch_with_data_move`: Moves a word from offset 12 of the structure in a5 into d5 and then branches to another routine.
- `execute_pointer_loop`: Pushes a value from an offset of 16 in a5 onto the stack, calls another routine, and then enters a loop that continues as long as the returned value is not equal to 65535.
- `update_data_and_branch`: Stores a byte from d4 into a2, increments a2, and enters a loop that decrements d5 and repeats the process if d5 is not zero, eventually performing comparisons on a value from a stack offset and branching based on the result.
- `process_pointer_stream_2`: Loads a pointer from an offset of 8 in a5 into a2, compares a word at offset 4 of that pointer with a word at offset 0, and if the comparison fails, calls another routine before updating a2 and returning.
- `process_pointer_stream`: Loads a pointer from a2 into a0, increments the pointer in a2, extracts a byte from the address in a0 into d0, and branches to an earlier instruction in this routine.
- `check_bitmask_status`: Loads a pointer from an offset of 8 in a5 into a2, checks if bits 3 or 4 of a byte at offset 12 of that pointer are set, and otherwise returns.
- `process_data_structure_list`: Clears bit 2 of a byte at offset 3 of a structure in a2, then iterates through a list of structures starting from a calculated pointer until a limit is reached, calling another routine for each entry that matches a specific bit pattern.
- `apply_bitmask_to_byte`: Sets the lower 5 bits of a byte at offset 12 of the structure in a2 to 16 and then branches to an earlier instruction in this routine.
- `calculate_and_process_pointer`: Extends d4 to a longword, adds it to a value at offset 8 of the structure in a2, stores the result at offset 4 of the structure in a2, updates a0 from the new pointer at offset 8, increments the pointer in a2, and branches to an earlier instruction in this routine.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `compare_and_decrement_pointer`: Compares a byte from a buffer with a value in a register, and if they match, decrements the pointer in a register and returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `jump_to_address_with_stack_save`: Pops an address into a0, pushes d2, d3, and a4 onto the stack, pushes a0 onto the stack, and jumps to the address in a0.
- `execute_indirect_jump`: Restores registers from the stack and performs an indirect jump using a pointer retrieved from the stack.
- `execute_conditional_library_call`: Calls check_status_bit and call_library_and_branch.
- `check_status_bit`: Masks a word from an offset in a2, compares it to one, and if it does not match, writes a word to an offset in a4 and branches.
- `execute_io_operation`: Calls another routine with three parameters, and if the result is negative one, calls a second routine and stores its result in a structure at a specific offset.
- `branch_with_register_move`: Moves the longword in d5 into d0 and then branches to another routine.
- `check_pointer_value`: Saves registers to the stack, compares a word at a pointer to a constant, and either branches to a specific location or increments the pointer and branches elsewhere.
- `update_table_entries`: Iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `manage_library_resources`: Interacts with library structures to check status, execute functions, and update a pointer in a table.
- `copy_memory_with_overlap_check`: Checks if the destination and source addresses are identical and returns immediately if they are; otherwise, it copies a specified number of bytes from the source to the destination using a forward loop if the destination address is lower than the source, or a backward loop if the destination address is higher than the source.
- `clear_memory_and_call_function`: Clears a block of memory using a loop and then calls a function pointer retrieved from a global variable.
- `load_address_into_a4`: Loads a constant value into register a4 and returns.
- `parse_quoted_string_tokens`: Parses a string into individual tokens by handling whitespace and quoted substrings, then stores the resulting pointers into a newly allocated array.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `set_limit_and_branch`: Sets register d0 to 32767 and branches to another routine.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `call_library_function`: Saves registers, loads a pointer from a structure, calls a routine via a pointer on the stack, stores the result in a structure, and restores registers before returning.
- `process_window_data`: Calls several routines to retrieve data, updates a structure at offset 96 and 100 with values from a library call, and then clears a global variable at offset 52.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_operands`: Saves registers, clears a register, and checks the sign of two longwords, negating them and setting a counter if they are negative before calling another routine.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `swap_rotate_and_divide`: Performs a series of word swaps, rotations, and conditional subtractions in a loop, or performs division and modulo operations on words if a specific bit is set.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `update_structure_elements`: Calls another routine to copy a byte from offset 9 of a structure in a5 to a location in a0 and increments a2, then calls a third routine that loads a pointer from offset 8 of a5 into a2 and performs a bitwise check on a byte at offset 12.
- `update_byte_at_pointer`: Copies a byte from an offset of 9 in a structure pointed to by a5 to a location pointed to by a0, increments the pointer in a2, and branches to an earlier instruction.
- `check_bit_and_branch`: Saves registers, loads a pointer from offset 8 of a5 into a2, writes a constant to an offset in a4, and if bit 4 of a byte at offset 12 of a2 is not set, branches to another routine; otherwise it sets d0 to negative 1 and returns.
- `process_structure_data`: Compares a word at offset 12 of a structure in a5 with negative 1, and if they are not equal, clears bit 2 of a byte at offset 12 of a2, clears a longword, and calls another routine.
- `clear_data_on_condition`: Compares a word at offset 12 of a5 with negative 1, and if they are not equal, clears bit 2 of a byte at offset 12 of a2, clears a longword at a2, clears a longword at offset 4 of a2, sets d0 to zero, and branches to another location.
- `update_structure_and_return_value`: Updates a structure by setting its first two longwords to the value of a third longword, or alternatively updates its first two longwords and a byte at offset 3 using values from a third longword and a short at offset 4, then returns a 16-bit value.
- `update_structure_and_write_byte`: Updates a 32-bit value in a structure by adding a 16-bit value to a 16-bit component, sets bit 2 of a byte at offset 3, increments a pointer, and writes a byte from an offset of 13 in a structure to the new pointer location.
- `update_structure_fields_2`: Calls another routine and, if the return value is zero, sets a 16-bit value at offset 16 and a 32-bit value at offset 8 of a structure in param_1; otherwise, it sets a 16-bit value at offset 16, sets bit 1 of a byte at offset 12, and sets a 32-bit value at offset 8.
- `update_structure_and_call_handler`: Writes a 16-bit constant to offset 16 of a structure, sets bit 1 of a byte at offset 12, and copies a longword from a stack location to offset 8 of the structure.
- `allocate_and_link_node`: Calls a function pointer with two parameters, and if the result is not zero, stores a longword from a structure into the new memory, links it to a global variable, and updates a pointer in a structure.
- `move_and_offset_pointer`: Moves a longword from an offset of negative 20114 in a4 to a location in a2, updates a structure at offset 4 with a value from offset 8 in a5, and branches to an earlier instruction.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `validate_scaled_index`: Checks if a parameter is within certain bounds and if a value at a scaled index in a structure is non-zero; if the check fails, it writes a word to a structure in a4 and returns a specific value, otherwise it calls another routine and returns a boolean result.
- `process_scaled_index`: Multiplies a word from a structure by six, uses the result to push a longword from a table onto the stack, calls another routine, and sets d0 based on the result.
- `clear_d0_and_branch`: Sets d0 to zero and branches to an earlier instruction.
- `mask_word_and_call_library`: Masks a word from a structure at an offset in a2 and, if the result is non-zero, writes a word to an offset in a4 and calls a routine at a relative offset.
- `test_and_set_status`: Masks a word from an offset in a2 and, if the result is non-zero, writes a word to an offset in a4 and branches.
- `handle_io_error_and_write`: Calls a routine with three arguments, and if the result is negative, calls another routine and stores its return value into a global variable at a negative offset from a pointer.
- `move_long_and_branch`: Moves a 32-bit value from register d5 to d0 and then performs a short branch.
- `call_library_and_branch`: Calls a routine at a relative offset, clears register d0, and then performs a short branch.
- `call_routine_and_branch`: Calls a routine at a relative offset, clears register d0, and then performs a short branch.
- `execute_write_sequence`: Calls a function via a pointer to process a global variable and then executes two subsequent routine calls.
- `execute_indirect_functions`: Checks a global pointer and, if non-zero, executes the routine pointed to by that address, followed by another indirect function call.
- `process_table_entry`: Checks if an index is within the bounds of a table, and if the entry is valid and not marked, calls another routine and clears the entry; otherwise, it sets a global variable to 2 and returns -1.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `call_library_vector_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 6 of that base, returning its result to the caller.
- `call_library_vector_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 12 of that base, returning its result to the caller.
- `jump_to_close_handler`: Jumps to a routine located at an offset from a pointer stored in a global variable.
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
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `dispatch_indirect_call`: Performs an indirect jump to a function pointer retrieved from a table located at an offset from a value in a register.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_2`: Loads a pointer from a structure located via an offset from a value in a register and jumps to the address stored at a fixed offset within that pointed-to structure.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `indirect_jump_stub_3`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `indirect_jump_stub_4`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `indirect_jump_stub_5`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `indirect_jump_stub_6`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `indirect_jump_stub_7`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `execute_indirect_function_3`: Performs an indirect jump to a function address retrieved from a pointer located at an offset within a structure pointed to by a value in a register.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `indirect_jump_stub_8`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_306_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_486`: Library stub: loads a library base from a global variable and calls the library function at offset minus 486 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_96_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_384_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.

### TV Sports Basketball (1990)(Cinemaware)[a](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### TV Sports Basketball (1990)(Cinemaware)[a](Disk 2 of 2).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

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

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_initialization_routines`: Calls a routine that initializes memory blocks and clears a series of byte-sized memory locations, then calls two other routines using pointers stored in a structure.
- `shift_and_store_byte`: Loads a byte from an offset of a structure in a4, shifts it left by one bit, stores the result back to a different offset in the same structure, clears a 16-bit value at an offset of a5, and performs a short branch.
- `update_sprite_attribute_table`: Iterates through a range of memory defined by a counter and a limit, updating multiple attribute buffers including byte-sized and word-sized metadata and secondary attribute data based on an index.
- `initialize_memory_blocks`: Sets several global variables or memory locations to specific values and clears a series of byte-sized memory locations using a loop.
- `clear_memory_blocks`: Clears or zeros out a series of word-sized memory locations using a loop that runs 6 times.
- `update_memory_tables`: Copies word-sized values from a source table to a destination table, using an offset or a value from a global variable to determine the source address.
- `initialize_string_buffer`: Allocates a memory buffer and initializes it with a specific header, then updates several global state pointers and flags.
- `move_register_value`: Moves the contents of register d0 into register d5.
- `process_callback_list`: Iterates through a list of function pointers and executes them, followed by a routine that passes a calculated size or pointer to another function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_state_machine_and_counters`: Updates a complex state machine by checking a status flag in a global structure, resetting multiple 16-bit and 32-bit values, and executing various function pointers based on conditional checks of counters and flags.
- `update_sprite_attributes`: Iterates through a table of object descriptors to modify specific attribute bytes, including color and flags, and calls a routine to update the display or position.
- `extract_byte_and_branch`: Loads a byte from an offset of 9 relative to a5 into d0 and branches to another routine.
- `branch_to_offset_five_hundred_and_ten`: Performs a short branch to an instruction at a relative offset of five hundred and ten.
- `branch_to_offset_1502`: Performs a short branch to an instruction located 1502 bytes ahead.
- `calculate_offset_from_value`: Compares a value in d0 against several constants and, if no match is found, subtracts 217 from that value and returns it; otherwise it returns 0.
- `unlock_address_register_5`: Unlocks address register 5 and returns immediately.
- `clear_global_byte`: Clears a zero byte into a global variable.
- `update_global_pointers`: Executes a function pointer and then copies values from a table or structure into several global variables.
- `call_subroutines`: Calls two subroutines via pointers, passing zero as an argument to the first one.
- `update_state_counters`: Updates several 16-bit counters and status values within a structure pointed to by a register through a series of nested conditional checks.
- `set_conditional_values`: Sets certain global bytes or words based on the value of a parameter and then executes a function pointer.
- `update_sprite_attributes_2`: Sets certain global words, then enters a loop that calls another routine and updates a global word with a modulo-reduced value.
- `call_with_table_value`: Executes a function pointer using a value retrieved from a table.
- `reset_and_configure_game_state`: Initializes a table with eight entries of the value nine, resets two 16-bit values to zero, and calls another routine with zero.
- `fill_array_with_parameter`: Executes a function pointer and then loops to assign a parameter value and a constant to a table.
- `modify_offset_and_branch`: Moves a word from an offset of a5 to a negative offset of a4, tests a word at an offset of a5, and if it is zero, adds 30 to a word from an offset of a5, stores it at that offset, moves a word to a different offset of a5, and branches to another routine.
- `update_entity_attribute_loop`: Iterates through a range of an array to update a memory-mapped attribute or pointer, and if a certain condition is met, updates a secondary table and calls two function pointers with a new coordinate or state.
- `transform_table_data`: Loops through a table to multiply values by six and add thirty-one, or sets them to zero, based on a source table.
- `update_game_state_logic`: Processes a state machine based on an input value, performing conditional checks against a table and global variables to update various status flags, counters, and pointers.
- `copy_value_from_table`: Copies a 16-bit value from a table location, calculated using an index from a global word, to a destination at an offset from a pointer in a5, then calls another routine.
- `search_table_for_pattern`: Iterates through a table to find a specific pattern or value, returning a calculated index or pointer.
- `execute_until_zero`: Executes a function pointer repeatedly while certain global shorts are non-zero, then executes two more function pointers.
- `parse_string_delimiters`: Searches through a buffer for a specific character and replaces a delimiter character with a null terminator.
- `copy_player_sprite_attributes`: Uses nested loops to copy sprite attribute data from a source table to a destination buffer, specifically transferring 14 bytes of data in one loop and 18 bytes in another.
- `search_table_entry`: Checks if a value provided in a register exists within a table structure located via a pointer in a global variable, using a loop that increments by 4 and returns a signed value based on the match found.
- `modify_offset_value`: Links a5 with an offset of -4, clears a word at offset -6 of a5, moves 21 to offset -4 of a5, compares a word at offset -11704 of a4 to 3, and if the value is less than or equal to 3 moves the value from offset -11704 of a4 to offset -2 of a5 before branching to another location; otherwise it branches to another location.
- `modify_and_branch_data`: Copies a word from a location offset by a negative value from a pointer in a4 to a location offset by negative 2 from a pointer in a5, subtracts 32 from a word in d0, and then branches to another routine.
- `unstack_a5`: Returns immediately after unlocking register a5.
- `load_word_and_branch_2`: Compares d0 to 9, and if it is less than 9, shifts d0 left by 1 and jumps to a location determined by a table lookup using the modified d0.
- `load_word_and_branch`: Loads a word from an offset of negative four from a pointer in a5 into d0 and then branches to another location.
- `process_state_machine_and_update_pointers`: Checks a flag in a global structure to conditionally execute a sequence of routines and reset specific word values.
- `extract_bitfield`: Calculates a 32-bit value by extracting a bit index and a word index from a 16-bit input value using division and modulo operations.
- `update_buffer_offset`: Updates a global variable or pointer offset based on a calculated value and checks it against certain bounds.
- `update_index_and_clamp_value`: Updates a global index by scanning a table for duplicate adjacent values, then clamps a secondary global value within a fixed range.
- `update_global_values`: Copies several dwords or words from a source table or pointer-based memory to a set of global variables and clears two or certain global variables to zero or one.
- `call_with_indexed_table_value`: Calls a function pointer stored at a global offset using two parameters, where the second parameter is used to calculate an index into a table to retrieve a value that is multiplied by four and added to another value retrieved from a global offset.
- `update_pointer_and_state`: Updates a pointer or a global word with a value from another memory location and conditionally sets certain global words based on a parameter.
- `clear_memory_and_assign`: Clears a range of 28 bytes in a memory block, sets a global word to 0, calls two routines, and stores the return value of the second call into a global word.
- `execute_multiple_calls`: Executes a sequence of function calls, some of which pass a value retrieved from a global table or pointer-based memory.
- `set_constant_recursively`: Calls a routine to write a constant value to a memory location, then calls another routine with two arguments, and finally calls itself recursively if a word at a specific offset in a structure is non-zero.
- `conditional_function_execution`: Calls a function, then checks if two global shorts are non-zero before calling another function that writes a constant to a memory location and passing a value from a global variable.
- `execute_set_constant`: Executes a function call, then calls another function to write a constant value to a global memory location, followed by a final function call.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_table_with_parameter`: Calls another routine with the constant 21, performs several word-sized copies between a table and a global variable, and calls the same routine again with 21.
- `update_buffer_from_table`: Clears a local buffer, then iterates through a range, reading values from a table and incrementing a byte in a buffer based on certain comparison logic.
- `update_structure_via_indirect_calls`: Calls several routines using a pointer from a4 to fetch a value, then conditionally performs a sequence of calls to write data to specific offsets within a structure.
- `branch_to_offset_2`: Performs a short branch to an instruction at a relative offset of 26.
- `fetch_and_store_values`: Calls three different routines to fetch values using a pointer from a5 and a constant, stores the result in a location at offset negative 14 of a5, and sets a byte at offset negative 74 of a4 to 1 if certain conditions are met.
- `perform_conditional_memory_operations`: Calls a function with four parameters and, if the result is non-zero, performs three additional function calls with varying arguments before returning the result.
- `copy_buffer_with_offset`: Copies dwords from a pointer-based table, calls another routine, and conditionally writes incremented bytes to a calculated memory offset.
- `accumulate_table_values`: Calls two routines, then enters a nested loop that iterates 20 times and 14 times respectively to add values from two different source tables into a destination table at calculated offsets.
- `update_table_with_max_value`: Iterates through a loop to compare values in a table and updates a global table with a new value if it is larger.
- `parse_string_delimiters_2`: Copies several values from a structure into local variables, calls a routine to parse a buffer, and if the result is non-zero, calls a routine to perform a memory operation and then calls two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `find_marker_and_call`: Iterates through a table until a specific byte value is found, then calls another routine multiple times using the discovered offset or an incremented offset.
- `populate_buffer_from_table`: If a parameter's upper word is 6, it loops through a table to copy bytes into a destination buffer at an offset determined by a parameter; otherwise, if the upper word is 7 or 0, it calls another routine and then loops through a table to update a local counter based on comparisons.
- `swap_array_elements`: Searches a table for a matching value and swaps two adjacent elements in that table if a certain condition is met, repeating this process in a loop.
- `map_buffer_indices`: Determines an initial offset based on a parameter, then enters a loop that iterates through a buffer and, if a byte is non-zero, copies two bytes from a source table to a destination table at calculated offsets.
- `empty_c_function_5`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `initialize_and_reorder_pixel_buffer`: Clears a secondary buffer by iterating through a 29 by 29 grid of 16-bit values.
- `compare_and_recurse_table`: Iterates through a table in a loop and calls another routine if certain adjacent or nearby values in the table match.
- `compare_and_swap_indices`: Reads two word values from a table using indices derived from a parameter, compares values in a second table at those indices, and calls another routine or returns a modified parameter based on the comparison.
- `calculate_index_offsets_and_reorder_table`: Clears a counter array of 29 elements in a structure, then iterates through a range of indices to compare calculated differences between values in a table and increments counters based on those comparisons.
- `swap_table_values`: Reads two word-sized values from a table at offsets 116 and 174 and writes them back to a different index in the same table, effectively swapping or overwriting values.
- `update_table_with_sum`: Calculates the sum of two short values from a table and passes this sum, along with a pointer and a constant, to a series of called subroutines.
- `process_pixel_value_differences`: Clears a memory buffer and then iterates through a table of byte values, comparing adjacent bytes and updating or decrementing values in a separate memory area based on the comparison results and a call to another routine.
- `find_matching_differences`: Iterates through a table and compares two values, storing an index and a difference into a destination table if a match is found.
- `transfer_data_block`: Copies a block of data from a source address to a destination address using a loop that decrements a counter and increments the pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_and_set_bit`: Calls another routine, then conditionally sets bit 4ef9 of the byte at the address in d0 using a6.
- `call_subroutine_at_offset_1f0`: Calls a subroutine located at offset 1f0.
