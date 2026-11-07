# Game 1114 (tv sports basketball (1990)(cinemaware)[cr pna]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01114). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1827156 top-level bytes in 54 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 28 of 28 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 681 of 681 named (0 author symbols), 681 of 681 described (213 by rule, 426 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4254 of 4254 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 400361 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16476 of 16476 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 626671 |
| code | 54464 |
| data | 785482 |
| padding | 12891 |
| unused | 347648 |

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
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2).adf:bb | 3 | 7843 | 388 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2).adf:bootblock | 1 | 11 | 1 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 72 | 72 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 453 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2273 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1068 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 2 of 2).adf:Fluff.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 2 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 183 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 522 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 702 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 2 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 92 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- tv sports basketball (1990)(cinemaware)[cr pna](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 2 of 2).adf:Fluff.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 2 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 400361 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### More

- `initialize_structure_from_globals`: Fetches a pointer from a global variable and stores the D0 register value and an address stack parameter into consecutive fields of the pointed structure, then branches to 20 if that pointer is null.
- `restore_patch_state`: Reads a pointer from a global variable, stores the parameter at offset negative eight of the pointed structure, and conditionally invokes cleanup callbacks from a table while writing different marker values to offset negative twelve before returning the previously stored parameter.
- `load_and_initialize_file_data`: Allocates a structure from a global pool, initializes its fields, and reads a file header to populate metadata.
- `iterate_table_with_function`: Loops four times, each time passing a pointer from offset 16 of the structure and the next four-byte value from a table to another routine, then returns.
- `validate_input_buffer`: Calls a routine twice with arguments derived from the input pointer, then scans up to 79 bytes checking each value against several allowed ranges and returns 1 if all pass or 0 if the counter exceeds 79.
- `process_input_byte_and_update_state`: Sets a flag at offset 60 of the structure to one and clears offset 94, then repeatedly calls a routine with the pointer at offset 12 and the value an address until it returns non-zero.
- `call_sequence_and_return`: Calls a routine with a pointer to a global variable, calls a routine with no arguments, then invokes a function pointer loaded from a register before returning.
- `initialize_structure_fields_2`: Stores one at offset 60 of the structure in param_1 and stores zero at offset 94, then calls another routine with the value at offset 52 and stores the returned value at offset 76.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from the parameter block, stores the second result into the field at offset 68, and when the flag at offset 4 is zero it increments the short at offset 40 of the pointed-to structure.
- `parse_buffer_content`: Calls a routine twice to compute a length, then loops through the input stopping at a newline or form-feed character, and if a form-feed is found writes a caret and L, replaces the character with a caret, and sets a flag before returning the count.
- `process_entity_list`: Computes a loop limit from the byte at offset 100 and offset 96 of the structure in param_1, then loops calling another routine with the pointer at offset 16 and a global variable slot, followed by another routine with the global variable and the structure.
- `handle_search_loop`: Selects a global variable from a flag in the parameter block, performs several setup calls, then loops calling another routine until it returns nonzero, returning one when the search succeeds and zero otherwise.
- `initialize_handler`: Performs a sequence of calls using arguments from the parameter block and a global variable, makes an extra call when the flag at offset 4 is nonzero, and returns the result of one of those calls.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at an offset from a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: Calls another routine with pointers from the parameter block, then calls another routine using the short at offset 44 of the structure pointed to by offset 8.
- `draw_text_window_range`: Calls another routine with a zeroed argument and a flag, then calls another routine using the short at offset 44 of the structure pointed to by offset 8 decremented by one.
- `draw_clamped_text_lines`: Clamps a value read at offset 42 of the structure in a2 to a maximum of 128, initializes a size field at offset 94 to zero, then iterates over a text buffer calling a drawing routine for each line, counting lines and stopping if a form-feed character or flag at offset 60 is encountered.
- `dispatch_recursive_pointer`: Calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Checks a flag at offset four of the parameter block and selects a second pointer from a table entry when the flag is nonzero, then calls another routine with a structure pointer and the two selected pointers, otherwise it returns.
- `dispatch_handler_by_flag`: Reads a flag at offset four of the parameter block and calls another routine with a structure pointer plus three references to a global variable when the flag is zero, otherwise it passes a value from a table entry, then returns.
- `clamp_negative_result`: Calls another routine twice with the same first argument and third arguments of one and negative one, clamps the second result to zero when negative, then returns the clamped value.
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition or partial way is met, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `reset_state_and_move_offset`: Returns zero when a size field is nonzero but a limit field is not the expected value, otherwise calls another routine with a computed pointer and remaining size, then copies fields between offsets and adds the returned amount to the size field when positive before returning it.
- `update_scroll_window_bounds`: Computes a target position from a base value plus an offset at 112 or 80 of the structure in a4, then checks whether that target falls outside the range defined by the fields at 108 and 112.
- `copy_line_from_buffer`: Reads successive bytes from the source buffer pointed to by the second argument into the destination buffer pointed to by the first argument, advancing both, until a newline byte is read or the count reaches one hundred and twenty-six, then writes a null terminator and returns the number of bytes copied.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_uppercase`: Loops through a null-terminated string, subtracting a constant from any lowercase letter before storing it back and advancing the pointer until the end marker is reached, then returns.
- `parse_signed_decimal`: Skips an optional leading sign character, then loops accumulating each digit into a running total multiplied by ten until a non-digit is found, negates the total when a minus sign was present, and returns the result.
- `call_library_function_with_mask`: Checks that both the first argument and a flag at offset four of the second argument are nonzero, then calls a library function using a base address from a global variable and the provided mask, otherwise it returns.
- `process_message`: Recursively invokes another routine on fields of the parameter structure and a string argument, then conditionally updates several fields and loops calling another routine until it returns ten.
- `build_message_buffer`: Retrieves a pointer from a global variable, stores the input parameter on the stack, calls another routine with that pointer, a constant of 994, the stack buffer, and a size of one, then returns the produced value.
- `copy_entity_data`: Requests a handle from another routine and returns zero when it is invalid, otherwise reads a value from the handle, passes it to another routine with a small stack buffer, copies two fields from the handle into the caller-provided structure, releases the handle, and returns the intermediate result.
- `allocate_and_link_object`: Returns zero when the first parameter is zero or the lookup routine returns zero, otherwise allocates a buffer, links its header fields, copies four words from the source block, releases the lookup result, reads a value from the buffer, and calls the deletion routine.
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
- `extract_byte_from_call`: Calls another routine with a pointer to a small stack buffer and the constant one, then returns the first byte of that buffer when the result equals one, otherwise returns negative one.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored at a fixed offset in a global variable, then scans a stack buffer byte by byte until it finds a null terminator while computing a running checksum, and passes the buffer and inverted checksum to another routine when that value is non-zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `modular_inverse_32bit`: Performs a 32-bit modular reduction by repeated doubling and subtraction when the divisor exceeds 15, otherwise splits the dividend into its high and low halves and returns the combined 32-bit remainder and quotient.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `return_maybe_negated_result`: Calls a helper routine that performs a 32-bit modular reduction and returns its result in a register, then negates that result when a second register holds a nonzero value before returning.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_advance_pointer`: Stores the byte in d0 through the pointer in a3, advancing that pointer by one, then returns.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Allocates a message buffer and, on success, fills several fields with the parameters and a value from another routine, records a value from yet another routine, then dispatches the buffer to a caller-selected slot before returning it.
- `delete_port_routine`: Removes the port from a global list when a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block before returning.
- `compare_strings_with_wrap`: Compares two strings byte by byte while the second is non-empty and the first does not start with a newline, returning one when the second string ends first and zero otherwise, advancing the first pointer when characters match.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_d1_to_d0`: Copies the 32-bit value from the d1 register into the d0 register, then branches into the middle of another routine without returning.

### Assign

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value stored in the field at offset 4 of each node, then, if a condition is met, writes that maximum at offset 50 of a structure, fills sixteen consecutive words with sequential odd values, copies sixteen words between a temporary area and a global buffer, installs a function pointer, and returns success or failure depending on the conditions checked during the loop.
- `move_d1_to_d0_2`: Copies the value in register d1 into register d0 and then branches into another routine.

### EndCLI

- `find_max_and_prepare_buffer_5`: Walks a linked list of entries to find the maximum value stored in the field at offset 4, then writes that maximum at offset 50 of the structure pointed to by a global variable.
- `transfer_d1_to_d0_2`: Copies the value in d1 into d0 and then jumps into the middle of another routine without returning.

### Dir

- `find_max_and_prepare_buffer_6`: Walks a linked list of entries to find the maximum value stored in the field at offset 4, then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_d1_to_d0_3`: Copies the value from register d1 into register d0 and then branches into the middle of another routine.

### Echo

- `find_max_and_prepare_buffer_7`: Walks a linked list of structures to find the maximum value in the field at offset 4, then, if a condition is met, writes that value and a sequence of sixteen words into a buffer, installs a function pointer, and copies sixteen words between a temporary area and a global buffer.
- `transfer_d1_to_d0_4`: Copies the value from register d1 into register d0 and then branches into another routine.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Execute

- `find_max_and_prepare_buffer_8`: Walks a linked list of entries to find the maximum value stored in the field at offset 4, then writes that maximum into the structure field at offset 50 and fills sixteen consecutive words at offset 12 with incrementing values starting at an address.
- `transfer_d1_to_d0_5`: Copies the value from register d1 into register d0 and then branches into another routine to continue execution there.

### Copy

- `find_max_and_prepare_buffer_9`: Walks a linked list of entries to find the maximum value stored in a field, then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d1_to_d0`: Copies the 32-bit value from register d1 into register d0, then branches to another routine.

### Makedir

- `find_max_and_prepare_buffer_10`: Walks a linked list of entries to find the maximum value stored in a field, then writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_d1_to_d0_6`: Copies the value in register d1 into register d0 and then jumps into the middle of another routine to continue its execution.

### CD

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value stored in the field at offset 4, then writes that maximum at offset 50 of a structure and fills sixteen consecutive words starting at offset 3 with the sequential values 3, 5, 7, up to the maximum plus 71.
- `transfer_byte_to_d0`: Copies the signed byte in d1 into d0 and then branches into the middle of another routine.

### EndIf

- `return_zero_value`: Clears register d0 and returns immediately.

### If

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value stored in the field at offset 4, then writes that value into the structure at offset 50 and fills sixteen consecutive words starting at offset 3 with the repeating sequence 3, 5, 7, 9.
- `transfer_d1_to_d0`: Copies the value in register d1 into register d0 and then branches into the middle of another routine.

### IconX

- `parse_command_line_arguments`: Parses a command-line string stored in a global variable, splitting it into space-separated tokens and converting asterisk escape sequences such as *N and *E into a newline and an escape character, then stores the resulting token pointers and text in a structure.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves the parameter into a slot within that structure, then conditionally invokes several cleanup callbacks stored in a table while setting specific values before each call, and finally returns the saved parameter.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start by checking the first parameter and a byte at offset 28 of the structure in a4, then allocates 1536 bytes of memory, parses a decimal value from the environment, and performs icon.library and exec.library calls to build and run a console command.
- `build_wait_and_endcli_command`: Clears a byte in the destination buffer, builds a command string from a template and a name, and if that name starts with a dot it appends a wait command with a long timeout and an endcli redirected to NIL before running the command through the DOS library, otherwise it discards the buffer and returns.
- `install_library_functions`: Returns zero when no descriptor is given, otherwise creates a library structure, sets up its function table, enables the library, and returns the resulting library base.
- `scan_ram_entries_for_names`: Allocates a two-hundred-and-four-byte buffer, walks a linked list of entries copying those whose name byte exceeds thirty-one into the buffer, converts the first slash to a colon, and returns the resulting string or zero when the allocation fails.
- `concatenate_strings_and_call_helper`: Copies a null-terminated string into a destination buffer, appends a second null-terminated string to that buffer, and then calls another routine with the buffer pointer and the total length of the combined strings.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to another routine.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign, and returns the accumulated value.
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
- `recurse_with_stack_slot_2`: Pushes a pointer to a location on the stack, calls another routine passing the two parameters and that stack slot, then restores the stack pointer and returns.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found, and passes the buffer and inverted checksum to another routine when the checksum is nonzero.
- `store_byte_via_pointer`: Stores the byte in register d0 through the pointer in a3 and advances that pointer by one byte, then returns.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer_2`: Stores the low byte of register d0 into the byte pointed to by a3, then returns.

### Disk-Validator

- `write_digit_and_process_table`: Writes a space character or a digit into a screen buffer at a position derived from a counter in the structure in a2, then decrements two counters, copies a 32-bit value, and clears several fields.

### paranoimia

- `intro_bytekiller_depack`: Unpacks a compressed stream by reading bits from its end and copying back-referenced runs backward into memory, then checks a checksum.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2).adf:bb

- `initialize_memory_and_dispatch`: Fills a block of 1918 words starting at the address in a4 minus 20978 with zero using a decrementing loop, writes the value from register 0 at offset 17804 and 19650 in the same a4-based structure, then reads a function pointer from a global variable and calls it passing the value returned by another routine.
- `update_string_metadata`: Compares two pointers and, when they differ, copies five words from a source to a destination through a structure, then updates several byte fields at offsets 208, 216, 135, 147, 151, and 159 of that structure with transformed values, and returns.
- `swap_linked_list_pointers`: Compares a pointer read at offset 342 of the structure in a6 with a global value, and if they differ and the pointer at the matching offset equals the stored pointer, it calls another routine with a base address.
- `process_table_entries_in_loop`: When the parameter is nonzero it loops up to twice, repeatedly calling a routine from a table and, if the resulting index points to a nonzero entry, calling another routine with a computed value.
- `initialize_library_and_store_values`: Initializes a library, then repeatedly reads a value from it and stores a running total plus a sequence of queued items into a structure, updating several byte and word fields along the way.
- `copy_table_words_to_buffer`: Initializes a stack counter to zero, then loops thirty-two times copying a word from a table into a destination buffer while advancing the pointer, and returns.
- `update_buffer_recursively`: If the second parameter's low byte is zero it writes the low word into a structure field, then stores the low word into a table entry indexed by that byte.
- `empty_routine`: This routine is empty, containing only a link instruction and a branch to another address.
- `update_table_recursively`: Loops while a counter at offset twelve is nonzero, incrementing a value at offset twelve, reading a word from a table through a pointer at offset twelve, and recursing with the pointed-to value before decrementing the counter.
- `wait_for_value_match`: Enters a loop that spins until the value at a global offset equals the value at another global offset, then returns.
- `rotate_bits_in_memory`: Calls a routine from a table, stores the result into a global variable, adds a constant to it, and saves the original value.
- `count_and_loop_until_satisfied`: Returns zero when the input is zero, otherwise counts the leading zero bits of the input by shifting it left in a loop, then repeatedly calls another routine and ANDs the result with a saved value until the result is not less than the input, and returns that value.
- `return_saved_value`: Loads a value from the location just below the return address into a register and returns it.
- `call_and_jump_forward`: Calls the routine at offset six hundred and ninety and then jumps to the code at offset seven hundred and seventy-eight.
- `decrement_and_execute_list`: Decrements two 16-bit counters at offsets -30574 and -30572 of the structure in a4, sets the first counter to six when it reaches zero, subtracts a value at offset -30568 from it, then walks a linked list of function pointers calling each one in turn, and returns zero.
- `update_global_state`: Copies a 32-bit value from a structure field to another field in the same structure, then stores the constants zero, one, and sixty into three consecutive byte and word fields.
- `insert_linked_node`: If the second element of the passed structure is zero it links the structure into a global list by copying the current head pointer into its first field, storing the parameter as its second field, and updating the global head pointer to point at it, then returns.
- `check_and_store_pointer`: Loads a pointer from a stack slot and checks whether the longword it references is zero, returning immediately when it is.
- `load_pointer_or_update`: Loads a pointer from a global variable, compares the value it points to with another stored pointer, and returns immediately when they match, otherwise overwrites the pointer with the pointed-to value and jumps back to the start.
- `assign_pointer_and_clear`: Copies a pointer from one location to another, then clears the value at offset four of the copied pointer's target before returning.
- `reset_global_pointer`: Calls another routine with specific arguments if a global variable is non-zero, then clears that global variable.
- `update_blt_registers`: Calls a routine, computes a bitmask from a parameter, and waits for a global flag to clear.
- `restore_registers_and_return`: Restores the caller's registers from the stack and returns.
- `setup_and_wait_for_flag_2`: Calls a routine that computes a 16-bit value from a register offset, stores a shifted version of it, decrements a counter at another offset, and fills the remaining data.
- `setup_and_wait_for_flag`: Computes a 16-bit value from a register offset and stores a shifted version of it, then decrements a counter at another offset and fills three adjacent words with initial values.
- `configure_blit_parameters`: Decrements a counter, sets up blit mask values, waits for a blitter flag, then loops over a table computing blit parameters and storing them through a pointer while a flag byte is set.
- `update_blit_parameters`: Loops over a table of byte flags, skipping entries whose bit is clear.
- `clip_rectangle_bounds`: Clips a rectangle defined by coordinates and dimensions in the structure at a6, adjusting its bounds when they fall outside the visible area and computing a 16-bit mask word and shift from the horizontal offset, then returns.
- `restore_registers_and_return_2`: Restores the caller's registers and stack frame, then returns.
- `initialize_input_state`: Writes the value one at offset -30532 of the structure in a4, calls a routine with a pointer at offset -19626, then loops four times writing the words 319 and 199 at offsets -19602 and -19598 plus a per-index stride of 30 bytes.
- `call_indirect_routines`: Saves three register values on the stack, performs an indirect call through a pointer, pushes two operands, and calls another routine through a pointer in a global variable before restoring the saved registers and returning.
- `call_indirect_function`: Calls a function pointer located at an offset from a global pointer.
- `clear_and_combine_table_bytes`: Reads three bytes from a table entry selected by a parameter, clears two of them, and returns a value combining those bytes with a bit-shifted offset.
- `prepare_and_branch_table`: Sets up an empty stack frame, saves registers d4 and d5 on the stack, loads the value three into d5 and the value zero into d4, then branches into another routine without starting a loop of its own.
- `iterate_and_clear_table`: Loops through a table from a positive index down to zero, clearing two bytes per entry and accumulating a bit-shifted combination of the read values, then returns the accumulated value.
- `clear_table_bytes`: Reads two bytes from a table entry selected by a parameter, clears one of them, and returns a value combining the two bytes with a bit-shifted offset.
- `reset_frame_counter`: Initializes the frame counter in d4 to zero and the shift amount in d5 to three, then jumps into another routine to begin processing.
- `clear_and_shift_bit_table`: Loops through a table from a positive index down to zero, clearing one byte per entry and accumulating a bit-shifted combination of the read values, then returns the accumulated value.
- `process_linked_list_nodes`: Calls a routine if a global flag is set, then iterates through a linked list calling a routine for each node and a second time when a bit and a global flag are both set, and finally calls another routine if the flag is set and returns its result.
- `initialize_buffer_structure`: Calls a routine to obtain a buffer pointer, clears a global variable, and if the pointer is non-zero stores a byte from a global variable, another byte, a parameter, and a self-referential pointer into the buffer, then stores that pointer back into the global variable and returns it.
- `call_loop_shifted_index`: Loops a counter from a starting value to an ending value stored in a structure, and on each iteration calls a routine from a table using an address built from a base offset plus a value formed by combining two other parameters with bit shifts, passing that combined value and the first parameter as arguments.
- `transform_nibble_data`: Calls a routine with a table pointer and a stack buffer, then loops over table entries applying nibble-wise comparisons and shifts to build a combined word that it stores back into the buffer and passes to another routine.
- `execute_sequence_with_checks`: Calls a routine with two structure fields and the value 512, and repeats calling a no-op routine while that call returns negative one, then stores the value 512 and the second structure field into adjacent fields before returning.
- `decode_length_prefixes`: Resolves a source pointer from a global table when none is provided, then repeatedly reads a control byte and copies the corresponding number of bytes from the source to the destination, advancing both pointers and decrementing the remaining count until the loop is exhausted.
- `decode_stream_byte`: Increments a counter in a global variable and calls another routine when a second counter is below one, then decodes a byte from a stream buffer by reading its high bits to select a case that advances a source pointer and adjusts a destination length accordingly.
- `increment_and_store_value`: Loads a 16-bit value from a global variable, adds two to it, stores the result back into another global variable, clears a byte at offset 4912 of the structure in a4, and then jumps to a shared routine.
- `advance_stream_position`: Advances a 16-bit position in a global variable by two and stores it back, then increments a byte at offset 1217 and branches away if the byte at offset 1206 is greater than zero.
- `decode_compressed_stream_2`: Decodes a compressed byte stream by reading a control byte, then copying a variable number of literal bytes and calling another routine for each byte, branching back to repeat while more data remains.
- `decode_compressed_stream_3`: Decodes a compressed stream by reading bytes from a source buffer through a circular index and writing them to a destination buffer, interpreting a leading bit pattern to determine whether to copy one, two, or more bytes.
- `copy_and_decode_stream_2`: Copies the next bytes from a source buffer into a destination buffer using a circular index, decrementing the remaining count each iteration and calling another routine when the index underflows, then decodes the stream.
- `clamp_and_advance_decode_stream`: Clamps a decoded value between the source and destination remaining counts, decrements those counts and the source pointer by the clamped amount, then reads the next byte and dispatches based on its high bits.
- `decode_compressed_stream`: Clamps a stored value against a limit and subtracts it from two counters, then calls copy_and_decode_stream and fill_buffer_with_counter before decoding a compressed byte stream whose header bits select which of several copy patterns to execute.
- `copy_and_decode_stream`: Copies bytes from a source buffer into a destination buffer using a circular index, decrementing the remaining count each iteration, then decodes a stream of length-prefixed commands that copy one, two, or three bytes and invoke another routine when a command counter reaches zero.
- `process_entity_state_updates`: Iterates through a list of 12-byte structures, clearing a flag bit and dispatching each non-skipped entry to a handler pointer stored at offset 16, interpreting command bytes at offset 12 to compute the argument passed to that handler.
- `dispatch_via_indirect_table`: Clears a 32-bit value in a global variable at offset -3414 and uses it as an index into a table at offset -42, then jumps to the routine found there.
- `save_d4_and_branch`: Saves the value in d4 to a global variable, checks that same global variable, and otherwise branches to a routine whose address is stored there.
- `iterate_entity_state_flags`: Iterates through a list of 12-byte structures, checking bit 0 of the flag field at offset 8 and calling a handler pointer at offset 8 of another structure when the bit is set.
- `increment_and_store_pointer`: If a global word equals negative three hundred and eighty-eight, it increments that word and, when it wraps to zero, calls a routine with a shifted global pointer and stores the result, resetting the counter if the result is zero.
- `accumulate_shifted_value_2`: Reads a 16-bit value from offset 26 of a structure in a5, adds to it the value at offset 22 of the same structure shifted right by one, and stores the result.
- `store_decoded_pointer`: Calls another routine with a pointer derived from a4 and stores the returned value at offset 8 of a structure reached through a5, then stores a computed pointer at offset 4 when the stored value is nonzero.
- `store_decoded_pointer_variant`: Calls another routine with a pointer derived from a4 and the constant two, stores the returned value at offset 8 of a structure reached through a5, then stores a computed pointer at offset 0 when the stored value is nonzero.
- `allocate_and_transfer_memory_block`: Allocates a block of memory and, if a flag byte at offset 33 of the structure in a5 is set, copies the pointer and size of that block into a global variable before releasing the block and clearing the stored pointers.
- `store_and_recursively_call`: Reads a value from a global table using a pointer at offset 7680 of the structure in a4, stores the result at offset 76 of the structure in a5, and if that stored value is nonzero it writes the value to offset 8 of a structure pointed to by the field at offset 68 and then calls another routine three times with the stored value plus 4, 8, and 24.
- `init_linked_list_node`: Initializes a linked list node by setting its type field based on a status byte, then recursively links child nodes and copies their data.
- `push_constants_and_call`: Pushes the constant 2 and a cleared stack slot, pushes the value at offset 7680 of the structure in a4, calls the routine at offset 356, adjusts the stack pointer, loads a structure pointer from offset 68 of the structure in a5, stores the result in offset 8 of that structure, and branches to the routine at offset 6022.
- `push_clears_and_call`: Pushes two cleared stack slots and the value at offset 7680 of the structure in a4, calls the routine at offset 356, adjusts the stack pointer, loads a structure pointer from offset 68 of the structure in a5, stores the result in offset 8 of that structure, and branches to the routine at offset 6022.
- `decode_and_store_sprite_pointer`: Checks bit 3 of a status byte in the structure in a5 and, if clear, calls the length-prefix decoder to read a source pointer, then stores two bytes from the source into offsets 32 and 33.
- `process_and_store_computed_value`: Calls another routine to build a value from a table, then stores it in a structure field and copies a block of bytes from one location to another.
- `accumulate_shifted_value`: Reads a 16-bit value from offset 26 of a structure in a5, adds to it the value at offset 22 of the same structure shifted right by one, and stores the result back at offset 30.
- `process_entity_state_updates_2`: Writes the value 32767 to a 16-bit field at offset 20686 of the global variable and, if the structure pointer at offset 19492 is non-zero and its length field at offset 4 exceeds the second parameter, iterates over a list of twelve-byte entries, calling a handler routine for each entry whose flag at offset 8 is set.
- `process_clamped_input_vectors`: Iterates over a table of structures and, for each one with a nonzero index-10 field, splits its index-10 value into two bytes and computes their signed differences from the stored values at offsets 8 and 9, clamping each difference to the range -127 to 127.
- `update_clamped_values`: Reads a byte from a table indexed by a passed value and, if negative, clears a word at offset eight of the structure in a0.
- `call_empty_routine`: Sets up a stack frame, pushes the constant fifteen and two cleared words, calls a routine with the value fifteen, cleans up the stack, removes the frame, and returns without doing any work.
- `conditional_clear_and_call`: Clears a longword at offset 184 of the structure pointed to by a global variable when that global is nonzero, then calls a routine with its value.
- `call_four_routines_sequentially`: Sets up a stack frame, then calls four routines in sequence: two with arguments pushed first, one with an empty stack slot, and one with no arguments, cleaning up the stack after each call before returning.
- `store_library_result_and_clear`: Calls a library function through a pointer using two constants, stores the returned value in a global variable, then clears four 16-bit words at offsets 18, 22, 26, and 30 of the structure pointed to by a global variable when that global is nonzero.
- `invoke_copy_data_table_with_offset`: Reads a pointer from a global variable at offset minus 6366, adds it to the passed parameter, and calls a routine that copies thirty-two words from a table into a destination buffer.
- `iterate_and_call_function`: Initializes a local counter to zero, then loops thirty-two times calling a routine from a global table with a value from a pointer-indexed table and incrementing the counter, otherwise it returns.
- `clear_global_pointer_target`: Clears a longword at offset minus 773c of a structure, clears a byte at a location derived from a pointer-indexed table, then calls another routine.
- `execute_loop_calls`: Calls a routine from a table using a global pointer with a range from zero to thirty-one, then calls it again with that range reversed.
- `call_function_with_split_parameter`: Calls a routine with the constant ten, then calls another routine using a value split from a single parameter and a constant, otherwise it returns.
- `dispatch_table_entry_and_check`: Reads a pointer from a table indexed by a parameter and calls a library function with a value from a global variable, then branches to an error message if that function returns zero.
- `store_table_result_and_check`: Reads a pointer from a table indexed by a parameter and calls a library function with a value from a global variable, stores the result in a table slot, then branches to an error message if that result is zero.
- `recurse_if_table_nonzero`: Reads a 16-bit value from a table in a4 indexed by a parameter, and if that value is nonzero it calls the routine whose address is stored there.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `call_function_with_global_pointer`: Calls a function using a value stored at a global pointer location.
- `call_function_with_global_pointer_alt`: Calls a function using a value stored at a global pointer location using a different function pointer.
- `call_with_swapped_arguments`: Calls a routine with a global pointer, then calls another routine with two values retrieved from memory locations pointed to by global variables, swapping their order depending on a passed flag, otherwise it returns.
- `call_function_with_indirect_values`: Calls a function using two values retrieved from memory locations pointed to by global variables.
- `call_function_with_parameter`: Calls a function using a global pointer and a provided parameter.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `subtract_and_branch_on_zero_2`: Subtracts 1005 from a value in a register and branches to a location depending on whether the result is zero.
- `push_params_and_call`: Pushes a constant, a structure value and a global pointer onto the stack, then calls a routine through a pointer at offset -32352 of a4.
- `store_routine_return_value`: Calls a routine with two 16-bit values read from a global variable and stores the returned 16-bit result four bytes before a structure, then returns.
- `subtract_and_branch_on_zero`: Subtracts 1005 from the value in d0, then branches to one of two locations depending on whether the result is zero.
- `conditional_store_and_call`: Reads a value from a routine's return through a pointer, stores it at offset minus four of a structure, and if nonzero branches on a flag to call a routine with different arguments before making two further calls.
- `perform_multiple_function_calls`: Performs a sequence of four function calls that manipulate values stored in a structure, including clearing a value and copying data between locations, then returns.
- `setup_structure_and_dispatch`: Calls a library function with a constant and a global value, and if the result is nonzero, sets up a structure, calls another routine through it, and closes that structure before returning its result.
- `update_global_with_sum`: Calls a routine with a global pointer, then runs a loop that decrements a local counter from five thousand to zero, sums values from global variables and a library call, and stores a bit-manipulated version of that sum into a global variable.
- `accumulate_global_value`: Calls a routine with a global pointer, combines values from global variables, then runs a loop that accumulates a computed value into a running total fifty times before storing the result into a global variable.
- `return_uninitialized_stack_value`: This routine returns an uninitialized value from the stack.
- `increment_global_with_sum`: Sums values from global variables and a passed value, runs a loop that decrements a counter derived from that sum, then adds five copies of a global variable and the sum into a global variable, otherwise it returns.
- `call_and_clear_structure`: Calls two routines using pointers stored at fixed offsets from a4, then clears the byte at offset 4bda and the word at offset 348a of that structure and the longwords at offsets 3482 and 347e.
- `execute_callback_list_callback`: Executes a routine or a sequence of routines retrieved from a table or a global pointer-based list.
- `call_wrapper_routine`: Calls another routine and returns.
- `load_and_call_function`: Loads two 32-bit values from a table into global variables and then calls a function pointer.
- `initialize_animation_parameters`: Calls a library function with a constant, executes an indirect call through a global pointer, then calls another library function with a different constant.
- `initialize_system_state`: Resets two 16-bit counters in a global variable, sets the counter at offset 44da to one, then calls three routines in sequence.
- `reset_global_flags`: Calls a library function with a constant, clears a global word at offset 4586, executes two indirect calls through global pointers, sets a global byte at offset 44da to one, and calls a final library function.
- `trigger_replay`: Calls another routine with a specific address and a global variable.
- `execute_multiple_initializations`: Calls a library function with a constant, then executes two subsequent indirect calls through global pointers.
- `conditionally_update_state`: Checks a 16-bit value in a global variable and, if it is zero, calls two routines in sequence.
- `set_half_state_flag`: Checks if a parameter is non-zero and, if so, calls a library function with a constant, sets a global byte at offset 452a to zero, executes an indirect call through a global pointer, and calls another library function with a different constant.
- `initialize_memory_tables`: Initializes a local variable to zero, then loops four times writing one to a location and zero to four other locations in a structure using an incrementing index.
- `accumulate_table_bytes`: Reads a byte from a table via a global variable and a routine's return value, multiplies it by a shifted parameter, and accumulates the product into a local byte.
- `update_tables_with_calculated_offsets`: Writes ones into a byte range of a table via a global pointer, computes a value from a routine's return, stores its complement in a second table, then builds two pointer tables from a global array and calls a routine with that array.
- `reset_game_state_globals`: Clears a 16-bit counter in a global variable and a four-entry table of 16-bit values, then branches on the value read from that counter to configure a second table before calling another routine recursively.
- `conditional_flag_and_return`: Reads a value from a routine and, if it is zero, reads another value and returns it after setting and clearing a flag, otherwise sets a different flag and returns one.
- `execute_indirect_call_sequence`: Performs a series of indirect subroutine calls through pointers derived from a global variable, passing a value read from a 16-bit location at offset -15510 to the fifth call and a per-element index multiplied by two into the same call.
- `store_indexed_value`: Reads a byte from a table indexed by a global variable, multiplies it by two, and uses the result as an offset into a second table to store a value returned by a routine.
- `update_conditional_state`: Calls a library function with a constant, sets a global byte at offset 4b3d if a global word at offset 4b94 is nonzero, executes an indirect call, then reads a value from a library function and stores its low bit into a global word at offset 3c6e.
- `call_offset_routine`: Calls another routine using a value calculated from a pointer and an offset.
- `recurse_with_parameter`: Reads a global variable and passes it as an argument to itself, recursing until the value becomes zero.
- `call_library_with_parameters`: Loads a library base from a global variable and calls its function at offset 270, passing a table entry, an offset plus eight, a value from that entry, and a value six bytes later.
- `update_conditional_table_entry`: Fills a two-entry table with the value 65535, then loops over four entries and, when a flag is set, calls a routine and stores the entry at an index chosen by comparing another value.
- `compare_global_bounds`: Compares a global 16-bit value against thirty-two and, if not greater or equal, compares it against zero and, if not less than or equal, jumps to another routine.
- `copy_structure_value`: Copies a 16-bit value from offset eight of a structure into a global variable and returns.
- `recursively_call_routine`: Checks if a parameter is non-zero and, if so, calls itself with that parameter.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_table_data`: Loops over four entries, copying values between tables, zeroing fields when a flag is clear, incrementing and indexing into tables when set, and storing an XOR of two values.
- `test_global_word_and_maybe_return`: Sets up a stack frame, tests a global word at offset 47720, and returns immediately when that value is zero, otherwise falling through to the next instruction.
- `index_and_check_byte`: Loads a byte from offset nine of the structure in a5 into d0, indexes a byte from a table at offset 4b92 using it, and returns zero when that byte is zero, otherwise it branches to a routine at offset 4024.
- `load_and_index_routine`: Clears a word at offset minus six of the structure in a5, then loads a byte from offset nine of that structure and a byte from offset minus 14686 of the structure in a4 to index a routine pointer, storing the result back at offset minus six.
- `increment_until_table_done`: Clears a 16-bit field at offset minus four of a5, then repeatedly increments a counter at offset minus two until it exceeds three or a table entry at offset 9 of a5 indicates completion.
- `compute_index_and_store`: Loads a byte from offset 9 of a5 into d0, increments it, negates the result and adds a value from offset -8 of a5.
- `increment_and_compare`: Increments a 16-bit value at offset minus two of a structure, compares it against four and jumps accordingly, otherwise jumping to a different routine.
- `initialize_array_bounds`: Clears a byte range sized by a global variable, then loops over four flags and writes index-based offsets into the range depending on whether each flag is negative or positive.
- `compare_global_byte_values`: Initializes a 16-bit value at offset minus four of a5 to seven, loads a byte from a global variable into d0, compares it with a word from another global variable, and branches if they differ.
- `loop_and_update_flags`: Loops while a byte read from a table at offset -14494 of a4 indexed by a structure value is zero, writing a negated incremented value into a table at offset -19262 and calling a routine.
- `clear_flags_and_call`: Clears two bytes and a global 16-bit value, calls a routine, and if the cleared byte is nonzero writes one into a global 16-bit variable.
- `clear_global_value`: Clears a 16-bit value in a global variable and returns.
- `update_index_with_function_results`: Loops over four entries, calling a routine and, when the result is non-zero, storing its negation and the negation of a second result into indexed table positions.
- `load_and_call_pointers`: Calls two routines, zeroes a four-entry table of 32-bit values, loads four pointers from a library-referenced structure, and calls a routine for each non-zero pointer.
- `run_callback_list`: Walks a table of four pointers stored in a global variable and calls the referenced routine for each entry that is not zero, then invokes two library functions via pointers in that same global variable.
- `recurse_with_parameters`: Calls a routine using a short value from offset 787a and the second field of the fourth argument, then calls itself recursively with modified parameters before returning.
- `dispatch_indirect_calls`: Reads a pointer from a global variable at offset -30842 and passes it to a routine looked up in a table, then calls a routine through a pointer at offset -32052 passing that pointer and a fifth value, and finally calls another routine through the same table pointer passing the base pointer and three additional values.
- `dispatch_and_adjust_stack`: Reads a pointer from offset minus 30842 of the structure in a4 and calls it, then branches on a word at offset minus 15524 to invoke a routine using parameters and adjust a value at offset minus two of the stack frame.
- `call_parameterized_pair`: Calls a routine through a pointer at offset -32052 of a4 with a global pointer and zero, then calls another routine with that pointer plus incremented structure values.
- `loop_and_call_parameterized`: Calls a routine through a pointer at offset -32052 of a4 with a global pointer and zero, then initializes a counter to zero.
- `decrement_and_select_branch`: Decrements d0 three times, branching to one of three different locations each time the value reaches zero.
- `invoke_parameterized_routines`: Calls a routine through a pointer taken from a global variable at offset -32052, passing that pointer and a 16-bit value read at offset 22 of a structure in a5, then calls another routine through a pointer at offset -30842 passing a pointer from that structure, two 16-bit values at offsets 10 and 12, and a value at offset 18.
- `invoke_wrapper_routine`: Calls a routine through a pointer using three parameters, then calls a second routine through another pointer using one parameter, and finally calls another routine using the first parameter, the fourth parameter, and the value returned by the second call.
- `increment_and_return_sp`: Increments the 16-bit values at offsets -2 and -4 of a5, then branches to another routine.
- `increment_and_branch`: Increments the 16-bit value at offset -2 of a5 and the 16-bit value at offset -4 of a5, then branches to another routine.
- `return_stack_pointer_word`: Loads the 16-bit word located at offset -4 from the value in a5 into d0, restores the stack pointer from a5, and returns.
- `call_with_offset_parameters`: Calls a routine through a pointer using a value from a global variable and a parameter, then calls another routine through a pointer using that same value plus four parameters derived from the inputs, two of which are sums of input pairs.
- `call_via_offset_table`: Reads two bytes from offsets 4 and 8 of a table entry selected by multiplying a value by 10 and adding offsets to a base, calls a routine through a pointer passing those two bytes, and if the byte at offset 9 of the entry is non-zero calls the same routine again with the same two bytes.
- `update_table_entry_fields`: Reads a 10-byte table entry selected by an index, copies its two 16-bit fields into two local variables depending on a flag byte, then calls three routines to update a structure pointed to by a global offset.
- `update_table_data_2`: Reads several values from a table using an index and offsets, stores some of them in local variables, calls three different routines through pointers with those values, and calls a further routine when a byte read at offset nine is not zero.
- `call_subroutine_via_pointer`: Calls a routine through a pointer stored in a global table using a passed parameter.
- `execute_indirect_call`: Fetches a function pointer from a table using an offset from a base address and calls it with a parameter.
- `execute_indirect_function_call`: Fetches a function pointer from a memory location and calls it using a value passed as an argument.
- `clear_memory_buffer`: Uses three nested loops to write zero into each 16-bit slot of a multi-dimensional table, iterating four times, then twenty times, then twice over the base address.
- `add_to_table_value_2`: Adjusts a parameter using a global byte, looks up a value in a table, and adds that parameter to 16-bit words at three nearby offsets in a table addressed by an index.
- `add_to_table_value`: Calculates an address in a table using an index and an offset, then adds a value from a register to the memory at that address.
- `dispatch_on_counter_value`: Saves a stack pointer, pushes a fixed longword, and calls a dispatch routine using a value from a table.
- `test_and_negate_long`: Tests the long at offset eight of the structure in a5 and jumps away when it is nonzero, otherwise clears that long, stores the value one at offset twelve, and branches to a routine at offset 4abe.
- `test_negate_and_call`: Tests a long at offset eight of the structure in a5 and jumps away when it is zero, otherwise negates it, loads a long from offset twelve into d1 and a long from offset eight into d0, calls a routine through a pointer in a4, negates d0, stores it at offset minus four, and jumps to a fixed address.
- `call_and_store_result_chain`: Calls a routine through a pointer in a4, stores the result at offset minus four of a5, sets a global byte to zero, then calls two more routines using a value at offset 16 of a5.
- `store_and_call_concatenated`: Calls a routine, then loads a value through a pointer at offset -32494 of a4 and stores it at offset -4 of a5.
- `conditionally_set_global_byte`: Sets a global byte to zero, checks a word at offset minus 17778 of a4, sets another global byte to four if it equals four, then calls two routines using values from a5.
- `decrement_long_and_dispatch`: Tests the long in d0 and subtracts one from it twice, branching away to different locations whenever the value becomes zero after each test, and finally branches unconditionally to another routine.
- `discard_stack_longwords`: Pops three longwords off the stack pointed to by a0, discarding them, then reads a longword from the top of the stack into d0.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a2).
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a7).
- `clear_frame_and_test`: Clears a 16-bit value at offset -2 of the frame in a5, tests a word at offset 12 of that frame, and if that word is zero it returns without using the value it just stored.
- `store_table_lookup_values`: Initializes a two-element loop counter and, for each iteration, scans a table of 12 entries to find the first value greater than a stored value, then stores the matching index, value, and a computed ratio into the structure in a4, adjusting the loop counter and repeating until the second iteration completes.
- `initialize_global_state`: Calls a library function through a pointer using two constants, stores the returned value in a global variable, then clears four 16-bit words in two nearby arrays of that global variable to zero.
- `call_routine_with_constant`: Pushes a constant onto the stack, pushes a value read from a global variable, calls a routine through a pointer using those two arguments, then restores the stack and returns.
- `enable_dma_channels`: Sets bit 15 of the DMA control register to enable DMA channels and returns.
- `write_dma_registers`: Multiplies a parameter by sixteen, stores the result at a hardware register base, copies a 32-bit value from a global variable into that register, and sets the following register to one.
- `push_dma_parameters`: Pushes the words 1, 2, and 4 onto the stack in turn and calls the DMA register writer after each push, so the writer receives 1, then 2, then 4 as its parameter.
- `enable_dma_channels_2`: Writes the value fifteen into the DMA control register to enable DMA channels and returns.
- `write_hardware_registers`: Multiplies a parameter by sixteen, then stores two 16-bit parameters into two consecutive hardware registers using the computed offset and returns.
- `check_indexed_byte`: Loads a word from offset eight of the structure in a5, scales it as a table index, and returns zero when the indexed byte is zero, otherwise it clears d0, unwinds the stack frame, and returns.
- `index_jump_table_routine`: Clears a word at offset minus eleven, stores the value 40 at offset minus ten, loads a long from a table indexed by a word at offset eight into offset minus four, then loads a word from offset eight, extends it to a long, shifts it left by two, and indexes a jump table to load a routine address.
- `store_constants_and_call`: Stores the values one and eighty into global words and into fields of a5, then stores two into a table indexed by that field and calls a routine with a value from a5.
- `store_two_and_call`: Stores the value two into a word computed from a5, then calls a routine passing a word read from offset -4 of a5 and returns.
- `branch_to_routine`: A one-instruction stub that unconditionally branches to another routine and never executes any instruction of its own.
- `dispatch_via_jump_table`: Compares d0 with one, branches if it is at least one, doubles d0, loads a word from a jump table into d0, and jumps to the address computed from it.
- `jump_to_4efe`: Unconditionally jumps to the routine starting at offset 4efe and never executes any instruction of its own, so it does nothing before transferring control.
- `clear_and_set_pointers`: Clears a 17-word range of a structure in a4, calls a library function, then stores return values from two other routines into a global variable and branches into a library call when a stored value is nonzero.
- `copy_frame_and_branch`: Copies the word at offset 8 from the frame pointer into the local variable at offset -2, then unconditionally branches to the routine at offset 50ec.
- `update_table_from_lookup`: Loops while a counter is below a limit, fetching a value from a lookup table to index a second table and storing each fetched value into a destination memory area, then increments the counter.
- `call_function_conditionally`: Checks a global variable and calls a stored function pointer with a value if that variable is non-zero, then calls the same function pointer with a different constant.
- `decrement_and_update_counters`: Decrements a 16-bit counter at offset -13354 of the structure in a4, then if the counters at offsets -13444 and -13462 are both zero it runs a two-iteration loop that decrements paired counters and clears them when they reach zero, otherwise it runs a single-iteration loop.
- `update_pointer_and_call`: Calls a stored function pointer, calls another stored function pointer with zero, then copies a value from a memory location to a global variable.
- `reset_function_and_copy`: Calls a function pointer with zero and copies a value from a memory location to a global variable.
- `reset_state_and_call_subroutines`: Clears a value in a global memory location, then executes two stored function pointers, passing zero to the first and no argument to the second.
- `call_reset_and_function`: Calls two stored routines with no arguments, then invokes the function pointer at offset -32378 of the base in a4 with no argument and returns.
- `clear_flag_configure_dma`: Clears the flag at offset -13614 of the structure in a4, calls the routine that enables DMA channels, then invokes the function pointer at offset -32230 with the value 5 and the function pointer at offset -32652 with the value an address.
- `test_dma_base_branch`: Tests a long at offset -19334 of the structure in a4 and, unless that value is zero, branches to another routine before restoring the stack frame and returning.
- `pass_dma_base_twice`: Loads the 32-bit value stored at offset -19334 of the structure in a4 and passes it twice to the DMA channel enabler, then returns.
- `format_decimal_digits`: Splits a positive integer into its decimal digits and calls a routine for each digit using a table of pointers, skipping leading zero digits.
- `process_and_dispatch`: Loops while a counter at offset -14 exceeds one, clearing entries and decrementing the counter, then runs two similar loops and dispatches through a table when a counter is nonzero.
- `process_and_call_dispatch`: Loops while a counter at offset -14 is greater than one, clearing entries at offset -10 and decrementing the counter, then loads a byte from offset 15 into that counter and loops again calling the function pointer at offset -32292 with a computed table index until the counter reaches zero.
- `iterate_and_call_on_condition`: Loops while a counter is not zero, comparing a value from a lookup table to a constant and calling a stored function pointer with a value from a memory location when the condition is met, then decrements the counter.
- `negate_frame_value`: Tests a word at offset 8 of the frame in a5, and when that value is non-negative it negates the value in d0 before restoring the stack frame and returning.
- `load_frame_to_d0`: Copies the word at offset 8 from the frame pointer into register d0, then unconditionally branches to the routine at offset 5460.
- `init_local_and_branch`: Initializes a local variable at offset -2 to zero, then unconditionally branches to the routine at offset 5492 without executing any further instruction of its own.
- `copy_byte_sequence`: Loops while an index is below a limit, copying a byte from one memory area to another using the index, then increments the index and returns it.
- `loop_clear_flags`: Clears a byte at offset -17782 of the structure in a4, then loops calling a routine, storing its result, and updating that byte based on flag bits until the routine returns zero.
- `clear_flags_call_return`: Clears a byte at offset -17782 of the structure in a4 and a 16-bit value at offset -2 of the structure in a5, then calls the routine at 28912 and adds four bytes back to the stack.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `compute_offset_and_clear_byte`: Reads a 16-bit value from a table at offset -32314 and adds to it the second argument scaled by 18 plus the first argument scaled by 448, then calls the function pointer at offset -32326 with the first argument and that result plus 1.
- `call_and_loop_routines`: Calls a routine using a parameter and a value from a table indexed by the high word of a second parameter, then loops six times calling a routine with a pointer from offset minus 25296 and another routine with that same pointer.
- `copy_string_and_dispatch`: Copies the null-terminated string from the first argument into a buffer in a global structure, stopping at a colon or end marker, then null-terminates it.
- `loop_until_value_130`: Stores the value an address at offset 342c, then loops while that value is not 130, repeatedly calling routines, updating the stored value, setting a byte at offset 3462 to three, and draining a queue of values.
- `call_setup_return_one`: Calls the routine at 23346, loads the value one into register d0, removes the stack frame held in a5, and returns.
- `store_and_call_with_offsets`: Stores the value 3 at offset -3462 of the structure in a4, then calls another routine.
- `process_structure_entries`: Reads a 16-bit value from offset 8 of the structure in a5 and a 16-bit value from offset 12, calls another routine with them, then reads a 16-bit value from offset 1318 of the structure in a4 and a 16-bit value from offset 1338 and calls another routine with them.
- `compare_and_branch_d0`: Compares the value in d0 against the constants 80 and 2, branching to different code depending on which comparison matches, and otherwise falling through to a third branch.
- `jump_to_routine`: Unconditionally transfers control to the routine at 22374, so this routine does nothing on its own.
- `branch_into_routine`: A one-instruction stub that unconditionally branches into the middle of another routine and never executes any instruction of its own.
- `handle_disk_swap_branch`: If the byte argument equals 2 it stores 1 in the return register and returns, otherwise it calls a routine with a negative argument, stores the result in a global variable at offset -13404, and either runs a loop that repeats until the routine returns zero or invokes a function pointer stored at offset -3244 of the structure in a4.
- `handle_disk_swap_state_transition`: Checks a flag at offset -21578 and, if clear, updates a global state variable and runs a sequence of conditional routine calls, an infinite loop when a flag at offset -1346 equals 5, and a five-iteration loop copying words from a table into a destination buffer.
- `write_table_values_and_reset`: If the 16-bit global at offset -21578 is nonzero it loops five times calling the function pointer at offset -32760 with the 16-bit value at offset -25446 and each successive 16-bit entry from a table starting at offset -15588.
- `call_indirect_function_160`: It loads the function pointer at offset -32490 and calls it with the constant 160 as its argument, then returns.
- `get_and_return_value`: Calls a routine and returns a value stored in a global variable.
- `test_and_return_zero`: Tests a long at offset -19334 of the structure in a4 and jumps away when it is nonzero, otherwise loads zero into d0, removes the stack frame, and returns.
- `initialize_structure_fields`: Writes the constants 6, 1, 1, 5, 5, and 0 into several word fields of the structure in a4, then copies a 16-bit value from offset 8 of the structure in a5 into offset -3426.
- `update_clamped_counters`: If the 16-bit global at offset -13334 is zero it checks the 16-bit global at offset -13352, decrementing it and, when it reaches zero, setting the value at offset -13348 to 65535 and the one at offset -13340 to one.
- `check_conditions_and_call`: If the 16-bit global at offset -17176 is nonzero while the ones at offsets -17210 and -17208 are zero it calls the function pointer at offset -32574.
- `increment_and_dispatch_counter`: Loads a 16-bit counter from a global variable, adds 7 to it, and stores both the incremented counter and another global 16-bit value onto the stack, then branches to a dispatch routine passing a value read from the stack.
- `store_values_branch`: Stores the value 1 at offset -3424 of the structure in a4 and the value 20 at offset -3422 of the same structure, then branches to the routine at 24254.
- `store_values_branch_2`: Stores the value 1 at offset -3424 of the structure in a4 and the value 40 at offset -3422 of the same structure, then branches to the routine at 24254.
- `fill_structure_from_index`: Computes a short from a field at offset -15444 of the structure in a4, stores it at offset -2 of the structure in a5, clears the word at offset -4, and then writes a set of constants and copied table values into fields of the structure in a4 depending on whether that cleared word is zero or the source field exceeds 2.
- `conditionally_fill_structure`: If the short at offset -3c54 of the structure in a4 is greater than 2, it writes a series of constants and copied values into fields at offsets -3416 through -3428 of that structure, using a value from offset -2 of the structure in a5 to index into tables at offsets -5428, -540a and -5446, then returns.
- `index_table_and_store`: Clears a 16-bit value at offset 13348 of the structure in a4, loads a short from offset 2 of the structure in a5, extends it to a long, and doubles it to index a table at offset 21514 of the structure in a4, storing the fetched short at offset 13344.
- `store_values_branch_3`: Stores the value 1 at offsets -341c and -3428 of the structure in a4 and the value 404 at offsets -341a and -3418 of the same structure, then branches to the routine at 24254.
- `dispatch_by_d0_value`: Tests the value in d0 against the constants 1, 7, 2, 9, and 9 using successive decrements, branching to a different label after each match.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `fill_buffer_with_counter`: It copies the 16-bit loop counter from the stack into d0 and the destination pointer into a0, then loops writing the current counter value as a byte to the location pointed to by a0 and incrementing that pointer until the counter reaches zero.
- `clear_word_and_branch`: Clears the word at offset -2 of the structure in a5 and then branches into another routine.
- `load_word_branch`: Loads the word at offset 12 of the structure in a5 into d5 and then branches to the routine at 24380.
- `call_loader_and_check_result`: Calls the pointer-checking routine, saves its result in d4, and returns immediately if that result is not the value -1.
- `store_bytes_in_loop`: Writes the byte in d4 to the location pointed to by a2, advances that pointer, and decrements the counter in d5, repeating the write until the counter reaches zero.
- `compare_pointers_and_call_loader`: Copies the long pointer stored at the address in a2 into a0, compares it with the pointer at offset 4, and calls the byte-loading routine if the copied pointer is less than or equal to the stored one.
- `increment_pointer_read_byte`: Copies the long pointer stored at the address in a2 into a0, increments that pointer, reads the byte it points to into d0, sign-extends it to a word and masks it to the low byte, then branches to the routine at 24628.
- `check_masked_byte_flags`: Reads the byte at offset 12 of the structure in a2, masks off all but bits 4, 5 and 6, and returns immediately if that masked value is zero, otherwise it discards its work and returns.
- `clear_flag_and_scan_table`: Clears bit 2 of the byte at offset 3 of the structure in a2, then returns immediately if the 16-bit value at offset 8 is zero or if bit 7 of that same byte is set.
- `set_bit_branch`: Sets d0 to 16, sets bit 4 of the byte at offset 12 of the structure in a2, sets d0 to 255, then branches to the routine at 24672.
- `accumulate_and_read_byte`: Copies d4 into d0, sign-extends it to a long, adds the long at offset 8 of the structure in a2 to it, stores the result at offset 4 of that structure, then reads a byte through the pointer in a2 (which it increments), masks it to a byte, and branches to the routine at 24672.
- `traverse_and_free_structures`: If the 16-bit value at the start of the structure pointed to by the first argument equals 20233 it walks a pointer list, finds a matching entry, and for each item in a nested loop it zeroes a slot in a table, follows a linked chain, calls the function pointer at offset -210, and writes a fixed value plus several computed fields into a growing structure.
- `compare_byte_and_return_pointer`: It loads the destination pointer into a0 and the comparison value into d0, then reads successive bytes from that pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `restore_registers_jump`: Pops a return address from the stack into a0, pushes d2, d3 and a4 onto the stack, pushes a0 back onto the stack, then jumps to the routine at 26232.
- `jump_to_popped_pointer`: It pops a pointer off the stack into a0, restores registers d2, d3 and a4 from the stack, and jumps to the function pointer in a0.
- `check_and_dispatch_error`: Checks a 16-bit value at offset 4 of the structure in a2, and if its low two bits equal 1 it calls check_and_store_error.
- `check_and_store_error`: Loads a 16-bit value from offset 4 of the structure in a2 and masks its low bits with 3, then compares the result with 1.
- `read_file_and_report`: Calls a routine with a pointer from a global variable and two values from the structure in a5, and if the result is -1 it stores the error code from another routine at offset -13328 of the structure in a4 before returning that -1.
- `copy_register_branch`: Copies d5 into d0 and then branches to the routine at 25752.
- `check_library_signature`: Pushes several registers on the stack, loads a pointer from the stack, checks whether the word it points at equals the library signature, and if so adds 4 to that pointer and branches away to a dispatcher routine.
- `copy_table_entries`: Computes an index into a global structure from a byte argument and walks a table of word pairs, copying jump-table entries into a destination buffer while skipping empty slots.
- `manage_library_resources`: It calls the function pointer at offset -552 and, when the result is nonzero, repeatedly calls the function pointer at offset -66 until it returns a negative value, then calls the one at offset -150.
- `copy_memory_backwards`: If the destination pointer equals the source pointer it returns immediately.
- `clear_memory_and_call_function`: Clears a block of memory using a loop and then calls a function pointer retrieved from a global variable.
- `load_pointer_register`: It loads the address of a fixed data location into a pointer register and then returns.
- `tokenize_string_buffer`: Parses a text buffer into individual null-terminated tokens, skipping whitespace and handling quoted strings, then stores pointers to each token in a table of 32-bit entries.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `load_seventeen_bit_branch`: Stores the decimal value 32767 into register d0 and then branches to continue execution inside another routine.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `open_icon_library`: Links the a5 register, pushes a stack frame, loads a library base from a5, pushes a library name, calls the library open routine, stores the returned base at offset -13326 of the structure in a4, and returns if the open failed.
- `store_library_results`: Chains a series of library calls and, when they succeed, stores returned pointers and values into a structure at offsets 156, 152, 160 and 154, then clears a flag and returns.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: Reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_division`: Pushes two registers, clears a high word, negates the d0 and d1 inputs when they are negative while counting the negations, calls a division routine, and copies the result from d1 into d0 before branching away.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: Calls another routine and then returns.
- `compute_64bit_division`: Splits a 64-bit value into 32-bit halves and, when the upper half is nonzero, repeatedly doubles a pair of words while propagating carry and conditionally subtracting a divisor.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `copy_byte_and_branch_2`: Copies a byte from a global variable to a pointed-to location, increments the source counter, masks the value to a byte, and branches away.
- `copy_byte_and_branch`: Copies a value from a global byte into a pointed-to location, increments the source counter, masks the value to a byte and then branches away.
- `load_structure_and_check`: Sets up its stack frame, saves d4 and a2, loads the pointer from offset 8 of the stack and the address of a table at offset 6a50, stores that table address at offset 34 of a4, and returns immediately if bit 2 of the byte at offset 12 of the loaded structure is clear.
- `compare_structure_fields`: Checks bit 2 of the byte at offset 3 of the structure in a2 and, if it is clear, compares the word at offset 0 with the word at offset 4 of that structure.
- `reset_structure_and_branch`: Compares a word at offset 12 of the structure in a5 to -1 and returns without effect if it is not -1.
- `dispatch_on_structure_fields`: Checks a word at offset eight of the structure in a2 and calls a routine when it is zero, then compares a word at offset four and branches based on whether it equals one, storing values and setting status bits along the way before returning a computed value.
- `add_words_and_append_byte`: Adds two words stored in a structure, stores the sum back, sets bit 2 of a status byte, appends a byte from a global location to a buffer and increments the buffer pointer.
- `set_flag_and_store`: Calls a routine and, if the result is zero, stores one at offset 16 and the address at offset 8 of the parameter structure, otherwise stores 1024 at offset 16, sets bit 1 of the byte at offset 12, and possibly sets bit 7.
- `set_flag_and_call_routine`: Sets bit 1 of the byte at offset 12 of the structure in a2, then loads a pointer from the stack into offset 8 of that structure.
- `allocate_and_store_pointer`: Stores the address of a global variable at offset -17272 of the structure in a4, calls a constructor routine with an offset and zero, and if the returned pointer is non-null stores a value and the parameter into its fields before returning an incremented pointer.
- `store_pointer_and_branch`: Stores a pointer from a global variable, copies a global pointer into a structure, saves the structure pointer to a global location and then branches away.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `validate_and_call_indexed`: Checks whether the parameter is negative, greater than a field at offset -5182 of the structure in a4, or indexes a zero entry in a table at offset -5192.
- `check_interactive_entry`: Loads a value from a5, multiplies it by 6, indexes a table at offset -17296 of the structure in a4, pushes the fetched pointer, calls a routine, and returns 1 when that routine reports the value is non-zero.
- `set_zero_then_branch`: Loads the value zero into register d0 and then branches to another routine.
- `check_and_dispatch_zero`: Checks a 16-bit value at offset 4 of the structure in a2, and if its low two bits equal 0 it calls check_and_store_zero.
- `check_and_store_zero`: Loads a 16-bit value from offset 4 of the structure in a2 and masks its low bits with 3, then compares the result with 0.
- `write_file_and_report`: Calls a routine with a pointer from a global variable and two values from the structure in a5, and if the result is -1 stores the error code from another routine at offset -13328 of the structure in a4 before returning that -1.
- `transfer_and_branch`: Copies the 32-bit value in d0 into d0 and then branches to another routine, passing control onward without returning.
- `check_flag_and_call_2`: Calls a routine via a global pointer with two arguments, then if bit 12 of the result is set it checks a signed word at offset -13542 of the structure in a4 and, when that word is nonzero, calls another routine before returning the result.
- `call_then_branch_zero`: Calls a routine via a global pointer, then loads zero into d0 and branches to another routine.
- `execute_write_sequence`: Links the frame pointer, pushes a size argument and a routine address onto the stack, and calls a function via a global pointer to process a global variable.
- `execute_indirect_functions`: Checks a global pointer and, if non-zero, executes the routine pointed to by that address, followed by another indirect function call.
- `validate_and_close_entry`: Indexes into a table using a parameter and checks whether the value is out of range or the entry is empty, storing a failure code and returning negative in that case.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `call_library_vector_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 6 of that base, returning its result to the caller.
- `call_library_vector_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 12 of that base, returning its result to the caller.
- `dispatch_via_jumptable`: Looks up a routine address in a table indexed by a global variable and jumps to it to dispatch the call.
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
- `jump_via_indirect_table`: Reads a 32-bit value from a global variable at offset -3414, indexes a table using that value at offset -42, and jumps to the routine found there.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_indexed_routine`: Reads a pointer from a table indexed by a value stored in a global variable, then calls the routine at that address and returns.
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
- `invoke_library_function_552`: Resolves a library base from a global variable and invokes the library function located at offset 552 of that base, then returns.
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

### TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 2 of 2).adf:bootblock

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

- `init_and_call_routines`: Initializes a structure via a5, then calls two routines referenced from function pointers stored at offsets -32536 and -32548 of the structure in a4, and returns zero.
- `double_and_store_byte`: Reads a byte from a structure, doubles its value, stores the result back into the same structure, clears a word at the top of its stack frame, and branches to another routine.
- `update_sprite_attribute_table`: Iterates a counter from zero up to a byte-sized limit stored at offset 74 of the structure in a4, copying a 16-bit value from offset 74 of the structure in a5 into several attribute tables and clearing their secondary metadata.
- `initialize_and_call_routine`: Stores several byte- and word-sized constants into fixed offsets of a structure, zeroes four consecutive byte locations in a loop, clears three more word- and byte-sized fields, calls another routine, and invokes a function through a pointer before returning.
- `clear_memory_blocks`: Zeros out six word-sized memory locations in a global variable using a loop that increments a counter and repeats while the counter is less than six.
- `update_memory_tables`: Copies a byte from a source table indexed by a parameter into a global variable, forces a comparison byte to seven, and then copies four word-sized entries from two source tables into two destination tables using a loop.
- `allocate_and_initialize_string_buffer`: Allocates a memory buffer via dos.library and stores its address in a global variable, then writes the four-character header value a fixed address at offset 4 of that buffer.
- `copy_d0_to_d5`: Copies the 32-bit value in register d0 into register d5 and returns.
- `process_callback_list`: Iterates through a list of function pointers and executes them, then passes a calculated size to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_state_and_update_flags`: Initializes a block of state fields in the structure in a4 to fixed values, then calls a routine and invokes the function pointer at offset 32622.
- `update_sprite_attributes`: Calls three other routines to initialize state, then loops through a table of sprite descriptors, setting color and flag bytes and invoking a routine to update each sprite's display position and attributes.
- `map_byte_to_offset`: Sets up the stack frame, loads the constant 0 into d0, then reads a byte from offset 9 of the structure in a5 into d0.
- `jump_to_other_routine`: Jumps to another routine and returns without doing anything else.
- `empty_return_routine`: It is an empty routine that returns immediately.
- `compute_offset_from_value`: Compares the byte in d0 against a fixed set of values and, when it does not match any of them, stores the signed difference from 217 into the return register, otherwise it returns zero.
- `restore_stack_and_return`: Restores the stack pointer from a5 and returns without doing anything else.
- `clear_global_word`: Clears the 16-bit value at offset -19348 of the structure in a4 by writing a zero word there, then returns.
- `call_pointers_and_copy_globals`: Calls the function pointer at offset -32640, then calls the function pointer at offset -32646 with a zero argument, and finally copies four 32-bit values from offsets 292, 340 and 532 of the structure pointed to by the integer at offset -19368 into the globals at offsets -19346, -19342 and -19338, then returns.
- `call_subroutines`: Calls two subroutines via pointers, passing zero as an argument to the first one.
- `update_match_timer_state`: Maintains a small state machine that counts down or up a timer field by three per call until it reaches its target, then sets a flag byte and decrements a counter, and when a separate five-step counter reaches zero it invokes another routine.
- `set_conditional_values_and_call`: Writes a zero byte at offset -19262 and 255 at offset -19261, then adjusts those bytes based on the parameter, and finally calls the function pointer at offset -32520.
- `loop_and_update_word`: Sets two global words, then loops while a word at offset -19256 equals 21, repeatedly calling a routine from a function pointer at offset -32574 and storing the modulo-28 of its result back into that word.
- `call_with_table_value`: Reads a 32-bit argument from a table at an index derived from the upper word of the parameter, then calls the function pointer at offset -32418 with that value and returns.
- `reset_and_configure_game_state`: Clears eight 16-bit slots in a table by storing the value nine, zeroes two adjacent 16-bit fields, and resets several state variables and counters.
- `fill_array_with_parameter`: Calls the function pointer at offset -32460, then loops from the start value to the end value, writing a 32-bit value from offset -31494 and the parameter word into a table, and calling the function pointer at offset -32490 with 31 each iteration, then returns.
- `adjust_word_and_branch`: Sets up a stack frame, stores a word from the parameter structure into a global location, and returns immediately when a second word is zero, otherwise adding thirty to it, storing the result back, decrementing the original, and branching to another routine.
- `update_entity_attribute_loop`: Loops over a table using a 16-bit index, copying a 16-bit value from a computed offset into a slot four bytes before the index, and increments the index until it reaches the value ten bytes past the index.
- `scale_table_entries`: If the second parameter word is zero it stores zero, otherwise it loops over the range and for each of eight slots writes either zero or the source value scaled by six plus 31 into a destination table, then returns the loop counter.
- `resolve_state_index_by_table`: Looks up a value in a table indexed by a counter in the structure in a4 and, if it matches the argument and is less than 8, returns it unchanged.
- `store_indexed_word`: Reads a word from a table indexed by a value at offset -17940 of the structure in a4 and stores it into the word at offset -2 of the structure in a5, then returns.
- `search_table_for_pattern`: Walks a table of 60-byte records, inspecting the first byte and scanning inner bytes until it finds a matching slot, then returns a combined value of the found index and a pointer offset.
- `execute_until_zero`: Calls the function pointer at offset -32532 while two global shorts are non-zero, then calls the function pointer at offset -32712 and the one at offset -32214 with 100, then returns.
- `parse_string_delimiters`: Scans a buffer of 64-byte records, returning early if it finds byte 2, and otherwise replacing each semicolon delimiter with a null terminator while skipping past any byte 2, then loops forever over the records.
- `copy_player_sprite_attributes`: Copies sprite attribute bytes from two source tables into a destination table using two nested loops, indexing the sources by two player indices, then clears a block of 16-bit values and stores player-derived values into a structure.
- `find_table_entry_match`: Checks a condition on a global structure and returns 1 if it fails, otherwise searches a table of 12-byte records for a matching entry and returns the product of a stored byte and a sign flag, or 0 when no record matches.
- `dispatch_indexed_routine`: Allocates stack space, clears a word on the stack, and pushes the constant 21.
- `adjust_and_dispatch_from_table`: Copies a 16-bit word from a table indexed by offset -17620 in a4 into the word at offset -2 of the structure in a5, then subtracts 32 from a value read at offset 8 of that structure.
- `restore_frame_and_return`: Restores the previous frame pointer and returns immediately, performing no other work.
- `dispatch_from_table`: Compares d0 with nine and, when it is less, doubles d0 and uses it to index a jump table, then branches to the selected entry without returning.
- `load_word_and_jump`: Loads a 16-bit word from offset -4 of the pointer in a5 into d0, then jumps to the routine at 10424 without returning.
- `manage_state_buffer_update`: Clears a flag at offset 19420 of the structure in a4 and, if it was already set, runs a sequence of state and buffer updates that swap two 32-bit values through a global pointer, loops a handler while a per-player flag is set, and stores a computed state index.
- `pack_division_result`: Takes a signed value, subtracts 32 from it, and packs the remainder and quotient of division by 8 into a single 32-bit result, returning 0 when the value is below 32.
- `update_buffer_offset`: Copies a stored value into a working slot, adds a scaled table lookup to it, and loops adjusting the value by a step until it leaves the valid range or a table entry is found, then stores the final value and its shifted index.
- `update_index_and_clamp_value`: Multiplies a global index by six, adds the product to a value at offset 19034 of the structure in a4, and stores the result back.
- `copy_global_values`: Copies three 32-bit values from a table addressed through a pointer in a global variable into three 32-bit fields of the structure in a6, then returns.
- `call_with_calculated_offset`: Adds a parameter to a value from a global variable, uses the low word of that sum as an index into a table to read a byte, multiplies it by four, adds the high word of the parameter, and calls the function pointer obtained from another global variable with the parameter and that computed value, then returns.
- `swap_and_set_state`: Swaps two four-byte values held through a pointer at offset -30842 and another at offset -19368, calls a routine from a function pointer at offset -32448 with zero, then sets two global words based on whether the parameter equals fourteen.
- `clear_and_call_routines`: Clears 28 consecutive bytes starting at offset -19730 of the structure in a4, zeroes a word at offset -17802, then calls three routines and stores one of their results into a word at offset -19052.
- `execute_multiple_calls`: Calls a sequence of function pointers from a global table, storing some return values and passing values retrieved from a structure field to later calls, then returns.
- `call_stored_functions`: Invokes a stored function pointer with an argument from offset -25446, then calls another stored function pointer with two computed values, and finally calls a third stored function pointer before returning.
- `conditional_function_execution`: Calls a function, then if two global shorts are non-zero writes a constant to a memory location and calls another function with a value from a global variable, and always makes a final call, then returns.
- `execute_set_constant`: Calls a function, then calls another function to write a constant value to a memory location, followed by a final function call, then returns.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `chain_table_moves`: Calls a routine with the constant 21, then performs a chain of four 32-bit moves between a global variable and offsets 4 of two tables, calls the routine with 21 again, and finally copies the value back before returning.
- `update_buffer_from_table`: Clears a local buffer of 29 bytes, then iterates through a range reading values from a table and incrementing a byte in the buffer based on comparison logic, then returns.
- `invoke_and_branch_into_routines`: Invokes a routine stored at offset -17620 in a4 with a value from offset -24608, then calls a routine at offset -17660 with the result.
- `branch_into_routine_2`: Unconditionally branches forward into the middle of another routine and does not return.
- `store_routine_results`: Calls three stored routines with different arguments, storing each return value at offset -14 of the structure in a5, then calls a fourth stored routine with a single argument and sets a byte at offset -19426 to 1 if a table value is below 25 and a field at offset 24 is zero.
- `conditional_call_sequence`: Calls a function with two parameters and two constants, and if the return value is non-zero makes two more calls with different arguments before returning the final result, otherwise returns zero.
- `copy_and_conditional_write`: Copies two 32-bit values from a table pointed to by a global variable into two fields of the structure in a4, then calls a routine with those values and two parameters.
- `accumulate_table_values`: Calls two stored routines with no arguments, then runs a nested loop that adds word values from two regions of a table into a destination table at computed offsets, repeating 14 times for the outer loop and 14 times for the inner loop.
- `update_table_with_max_value`: Iterates through a loop comparing values in a table and updates a global table with a new value if it is larger, storing the source index, then returns.
- `copy_values_and_call`: Copies six 32-bit values from various offsets of a table into a global variable, then calls a routine with four arguments and, if the return value is non-zero, calls two more stored routines before returning the same non-zero flag.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `find_marker_and_call`: Loops through a table, incrementing an index while the byte at each position is not six, then calls a routine three times with the found index, the index, and the index plus one before returning.
- `populate_buffer_from_table`: When the high word of the first parameter is six it copies two bytes from a table into a destination buffer using an offset, otherwise it checks a table and updates a local counter.
- `sort_and_call_routine`: Runs a nested loop over pairs in a table indexed from offset -17620, comparing values and swapping adjacent pairs when a match is found, then calls a routine with one argument once the outer loop completes.
- `map_buffer_indices`: Sets a stack value based on a conditional call when a parameter byte equals 2, initializes two loop counters, then loops 30 times copying bytes from a source table to a destination table at computed offsets whenever a read byte is non-zero.
- `empty_c_function_5`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `reorder_and_index_pixel_buffer`: Clears a 28-by-4 array of 16-bit counters in a global variable, then reorders the 29 entries of a source table into two destination tables based on whether each value is below 15.
- `compare_and_recurse_table`: Loops through a table of 16-bit values, and whenever two adjacent entries are equal it calls another routine with the following index, then advances by 14 and repeats until the counter reaches 16.
- `compare_and_swap_values`: Reads two word values from offsets 116 and 174 of a table indexed by the parameter, then compares entries in another table and either calls itself with the high byte, calls itself without returning, or masks the parameter to its low word before returning.
- `count_and_reorder_table_values`: Clears a 29-entry array of 16-bit counters, then scans a table of 16-bit values to count how many are equal to the first.
- `swap_table_values`: Reads two word-sized values from offsets 116 and 174 of a table indexed by the parameter, then swaps them back into the same two offsets of that table.
- `update_table_with_sum`: Reads two adjacent short values from a table indexed by the parameter, adds them together, and passes the sum along with a pointer and the decimal constant 65539 to a routine, then calls a second routine twice.
- `process_pixel_value_differences`: Clears an array of 28 rows by 4 short counters, then, while any row's first byte is greater than 1, compares pairs of adjacent bytes in that row and increments counters and calls another routine based on whether each byte is larger or smaller than its neighbor.
- `store_matching_differences`: Loops a byte counter from a parameter value down to its negative, subtracts two 16-bit values from a table for each of 29 entries, and stores the entry index and the difference into a destination table when they match.
- `short_processing_routine`: A short routine of fifteen instructions that performs its work and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_and_set_system_bit`: Branches to a subroutine, returns when the carry flag is clear, and then sets a bit in a system byte.
- `call_subroutine_only`: A single instruction that branches to a subroutine and does not return.
