# Game 436 (Tv Sports Basketball): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00436). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1826976 top-level bytes in 52 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 25 of 25 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 682 of 682 named (0 author symbols), 682 of 682 described (94 by rule, 586 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4309 of 4311 (99.95%) |
| Recovered without load address (counted in proof 1, flagged) | 400361 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16465 of 16465 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 627491 |
| code | 54534 |
| data | 776406 |
| padding | 12705 |
| unused | 355840 |

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
| TV Sports Basketball (1990)(Cinemaware)(Disk 1 of 2).adf:bb | 3 | 7832 | 384 |
| TV Sports Basketball (1990)(Cinemaware)(Disk 1 of 2).adf:bootblock | 1 | 105 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 79 | 79 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 449 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2273 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1068 | 14 |
| TV Sports Basketball (1990)(Cinemaware)(Disk 1 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)(Disk 2 of 2).adf:Fluff.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)(Disk 2 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 53 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 183 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 615 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 653 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- tv sports basketball (1990)(cinemaware)(disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)(Disk 1 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)(disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)(Disk 2 of 2).adf:Fluff.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)(disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)(Disk 2 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)(disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)(Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 3 never-loaded region(s), 400361 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_callback_chain_2` (More): Calls a global function pointer and then executes another routine. Kind: callback/function pointer execution trap. Pass path: the execution of the function pointer (*unaff_A3).

## Functions


### More

- `initialize_environment_and_load_data`: Initializes system libraries and global state before parsing a command-line string or configuration file.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `process_entity_state_logic`: Updates an entity's internal state and status flags based on specific byte values.
- `execute_callback_chain_2`: Calls a global function pointer and then executes another routine.
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
- `reset_state_and_move_offset`: The routine resets a set of fields in a structure and then calls another routine with a pointer into the structure and a size derived from a field, adding the returned value to a field when positive.
- `update_scroll_window`: Updates a scrolling window structure by calculating a target position using provided offsets and checking it against current window bounds.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string`: Library code that copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
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
- `recursive_call`: The routine calls itself and then returns.
- `negate_on_flag`: Calls another routine and returns its first return value, or the negation of that value if the second return value is non-zero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_address`: Stores a single byte from a register into a memory location pointed to by another register.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: The routine attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: The routine removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_and_call_function_2`: Copies sixteen bytes from a source offset to a destination and then executes a function pointer.

### Assign

- `allocate_and_initialize_memory_block`: Calculates the required size for a data structure by traversing a linked list, allocates a memory block, and populates it with a sequence of incrementing values.
- `copy_data_and_execute_callback_2`: Copies sixteen bytes from a memory location offset from a register to a destination on the stack.

### EndCLI

- `build_dynamic_command_table`: Calculates the maximum size required for a data structure by traversing a linked list and then populates a table with incrementing values.
- `copy_data_and_execute_callback_3`: Copies sixteen words from a memory location offset from a register into a stack-based destination before calling a function pointer.

### Dir

- `initialize_dynamic_data_structures`: Traverses a linked list to determine a maximum value and then populates a table with incrementing data.
- `copy_data_and_execute_callback_4`: Copies sixteen bytes from a memory location to a destination pointed to by the stack before calling a function pointer.

### Echo

- `populate_dynamic_data_table`: Iterates through a linked list to find a maximum value and then populates a structured table with calculated entries.
- `copy_data_and_execute_callback_5`: Copies sixteen bytes from a memory location offset from a register into a stack-based destination before calling a function pointer.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Execute

- `construct_dynamic_data_structure`: Iterates through a linked list to find a maximum value and then populates a new memory block with a sequence of incrementing values.
- `copy_memory_block`: Copies sixteen bytes from a source offset to a destination pointer if the destination is not null, then calls another routine.

### Copy

- `reconstruct_data_structure`: Traverses a linked list to find a maximum value and then populates a memory buffer with a sequence of incrementing values.
- `transfer_data_chunk`: Checks if a destination pointer is non-zero and performs a sixteen-byte block copy from a source address before executing a function call.

### Makedir

- `populate_dynamic_data_structure_3`: Iterates through a linked list to find a maximum value and then populates a data structure with a sequence of incrementing values.
- `perform_block_copy`: Iterates through sixteen words to copy data from a source location to a destination, followed by a call to another routine.

### CD

- `populate_dynamic_data_structure`: Iterates through a linked list to find a maximum value and then populates a memory buffer with a sequence of incrementing values.
- `copy_data_and_execute_callback`: Copies sixteen bytes from a memory location to a destination pointed to by a stack value, then calls a function via a pointer.

### EndIf

- `return_zero`: This routine is empty and simply returns a value of zero to the caller.

### If

- `populate_dynamic_data_structure_2`: Traverses a linked list to find a maximum value and then initializes a memory block with a sequence of incrementing values.
- `copy_memory_and_call_function`: Copies sixteen bytes from a memory location to a destination pointed to by a stack value, then executes a function via a pointer.

### IconX

- `initialize_environment_and_parse_config`: Initializes system structures and loads configuration data from a file.
- `update_and_cleanup_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `manage_screen_mode`: Configures a temporary screen and icon settings, allocates memory, and performs a block memory copy based on parameters from a structure.
- `handle_iconx_request`: The routine processes an icon request by initializing a buffer, looking up a handler from a table, and invoking another routine to obtain a handler result.
- `initialize_icon_library_functions`: The routine initializes a library by calling another routine to process a pointer, then creates function entries using library stubs.
- `parse_icon_path_string`: Iterates through a linked list of string nodes to locate a specific entry, then processes the associated path string by replacing the first slash character with a colon.
- `calculate_string_lengths`: The routine calculates the length of three null-terminated strings passed as parameters and stores the results in memory locations derived from those pointers.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string_2`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `skip_to_null_terminator_and_recurse`: The routine advances a pointer through a zero-terminated byte string until the terminating zero byte is found, then calls another routine with the pointer positioned at the terminator and the original second argument unchanged.
- `parse_signed_decimal_2`: Converts a null-terminated string representing a signed decimal integer into a numeric value.
- `call_supervisor_function`: Calls a function pointer retrieved from a global table offset.
- `call_exit_interrupt_function`: Executes a function pointer located at a specific offset within a global table.
- `call_schedule_function`: Invokes a function pointer stored in a global table.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatch_function`: Performs an indirect call to a function pointer found in a global table.
- `execute_indirect_function`: Fetches a function pointer from a location offset from a global variable and executes it.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable_function`: Executes a function pointer located within a global table.
- `execute_indirect_function_pointer`: Fetches a function pointer from a global table and executes it.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `execute_indirect_function_2`: Fetches a function pointer from a calculated offset within a global data structure and jumps to it.
- `recurse_with_stack_slot_2`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `call_indirect_handler`: Executes a function pointer retrieved from an offset within a global data structure.
- `call_offset_handler`: Performs an indirect call to a subroutine located at a specific offset from a global variable.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `execute_stored_pointer`: Loads a function address from a global table and jumps to it.
- `invoke_data_pointer`: Calls a routine by dereferencing a pointer stored at a negative offset from a global variable.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `invoke_and_checksum_stack`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a null-terminated buffer on the stack.
- `store_byte_via_pointer`: Stores a byte from a register into a memory location pointed to by another register.
- `invoke_table_entry_2`: Retrieves a function pointer from an offset within a global variable and executes it.
- `store_byte_via_pointer_2`: Writes a single byte to a destination address held in a register.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a table.

### TV Sports Basketball (1990)(Cinemaware)(Disk 1 of 2).adf:bb

- `initialize_system_state`: Clears a block of memory, updates a global variable, and executes a function pointer based on a bit test in a structure.
- `update_display_parameters`: Compares two memory locations, copies a sequence of words if they differ, and updates several fields in a structure using values from other memory locations.
- `swap_linked_list_nodes`: Compares values from a global variable and a pointer, then swaps two adjacent elements in a linked list and updates several pointers.
- `process_indirect_list`: Iterates through a list of pointers, calls a routine for each non-zero entry, and performs a library call to clear specific memory locations.
- `allocate_and_initialize_memory_blocks`: Allocates two separate memory blocks and populates them with sequential integer values.
- `copy_data_table`: Copies thirty-two words from a table into a destination buffer pointed to by a parameter.
- `update_table_entry`: Updates a specific entry in a table with a value and recursively calls itself with modified parameters.
- `update_table_recursively_2`: Recursively traverses a structure by incrementing a counter and modifying values through a pointer until a termination condition is met.
- `update_table_recursively`: Recursively updates a table by incrementing a counter and modifying values through a pointer until a termination condition is met.
- `wait_for_equality`: Enters a loop that waits until the values at two different memory locations become equal.
- `modify_global_value`: Calls a function via a pointer, stores the result, and performs arithmetic and bitwise shifts to update a global variable.
- `calculate_bit_shift_and_mask`: Calculates a bit shift amount based on the leading zeros of an input, then performs a bitwise AND operation between a table value and the result of another routine.
- `return_stack_value`: Returns a value stored at a location relative to the stack pointer.
- `update_global_via_stack`: Retrieves a value from a location relative to the stack pointer and passes it to a routine that updates a global variable using arithmetic and bitwise shifts.
- `decrement_and_execute_list`: Decrements two global short values, updates a third global short based on a subtraction, and executes a series of functions via a linked list.
- `update_state_variables`: Copies a value from one memory location to another and modifies several global variables.
- `insert_linked_node`: Inserts a new node into a linked list if the second element of the provided structure is zero.
- `replace_linked_list_node`: Searches a linked list for a specific node and replaces its pointer with a new value from a provided parameter, then clears a field in that parameter.
- `traverse_and_replace_pointer`: Traverses a linked list to find a matching value and replaces the target pointer with a new value while clearing a field at an offset.
- `assign_pointer_and_clear`: Copies a pointer from one location to another and then clears the value at the offset of that pointer.
- `reset_global_pointer`: Calls another routine with specific arguments if a global variable is non-zero, then clears that global variable.
- `configure_blitter`: Sets up and executes a blitter operation by configuring hardware registers using values from a table and parameters.
- `nop_padding_sequence`: This routine is empty and contains no executable instructions.
- `calculate_blit_parameters_3`: Performs complex arithmetic involving bitwise shifts, offsets, and table lookups to update a structure and a global variable.
- `calculate_blit_parameters_2`: Performs complex arithmetic involving bitwise shifts, offsets, and table lookups to update several values in a structure and a global variable.
- `calculate_blit_parameters`: Calculates and updates values for Blitter operations, including bitmasking and address calculations based on a table of offsets.
- `update_blit_parameters`: Iterates through a data structure to calculate and update values used for Blitter operations.
- `perform_blitter_area_fill`: Configures and executes a hardware blitter operation to fill or copy a rectangular area of memory.
- `nop_padding_sequence_2`: The routine is empty and performs no operations.
- `initialize_buffer_data`: Fills a buffer with specific constant values and updates global state based on input parameters and joystick or mouse input.
- `execute_indirect_callback`: Calls a function through a pointer stored at a specific offset relative to a register.
- `call_indirect_function`: calls a function through a pointer stored at an offset from a global base.
- `process_single_entry`: reads three bytes from a table at an offset calculated from a parameter, clears two of those bytes in the table, and returns a combined value.
- `accumulate_shifted_bytes`: Iterates through a table to clear specific bytes and accumulates their values into a single unsigned short using bitwise shifts.
- `clear_and_combine_table`: iterates through a table, clearing specific bytes at each entry and accumulating a bit-shifted value from the remaining bytes.
- `clear_two_bytes`: reads two bytes from a table, clears them in the table, and returns a value formed by bit-shifting and combining them.
- `combine_table_bytes`: Iterates through a table to clear bytes and combines them into a single unsigned short via bitwise shifts and OR operations.
- `clear_and_shift_table`: iterates through a table to clear bytes and accumulate a bit-shifted value into a single result.
- `traverse_linked_list`: iterates through a linked list, performing conditional function calls using data from each node and an accumulating value.
- `allocate_and_initialize_structure`: calls a function to allocate memory, then populates the memory with values from a global table and sets up a pointer to itself.
- `execute_loop_calls`: iterates through a range defined by a parameter and calls a function using a value constructed from bit-shifted components.
- `transform_nibble_data`: Iterates through a table of words to perform bitwise manipulations on nibbles and stores the results in a local buffer.
- `execute_looping_function`: Repeatedly calls another routine with specific parameters until it returns a value other than negative one.
- `execute_state_machine_sequence`: Iterates through a data buffer using a counter and a pointer, executing a series of operations driven by a switch statement.
- `parse_instruction_stream`: Iterates through a table of data to decode a sequence of instructions using a state machine.
- `process_data_stream_state_machine`: Iterates through a sequence of bytes extracted from a table, using a state machine to decode and store values into a local structure.
- `parse_bitstream_state_machine`: Iterates through a data table to extract bytes and processes them through a state machine that uses bitwise masking and shifting to determine subsequent actions.
- `parse_instruction_stream_2`: Iterates through a table of data to extract bytes and processes them through a state machine.
- `parse_data_stream_state_machine`: Extracts a sequence of bytes from a table and processes them through a state machine that uses conditional branching to determine subsequent data offsets and values.
- `populate_data_buffer_and_iterate`: Extracts a sequence of bytes from a table into a local structure and then enters a loop that processes these values through a state machine.
- `parse_bitstream_data`: Iterates through a data buffer using a state machine to extract variable-length bitfields.
- `process_state_machine_sequence_2`: Iterates through a data buffer using a counter and a pointer, executing a state machine driven by a switch statement.
- `process_state_machine_sequence`: Iterates through a data buffer using a counter and a pointer, then executes a state machine driven by a switch statement.
- `process_entity_state_updates`: Iterates through a list of structures to update internal state values based on specific command bytes.
- `search_nested_table`: Iterates through a structure to find a specific four-byte pattern.
- `iterate_entity_list_2`: Iterates through a list of structures and calls another routine for each entry where a specific byte flag is set.
- `iterate_entity_list`: Loops through a list of structures based on a count stored in a table.
- `increment_and_store_pointer`: Increments a counter in a global variable and, if it reaches zero, calls a function to fetch a value that is then stored in a global variable.
- `update_indirect_calculated_value`: Calculates a new value by adding a constant to a shifted register and stores the result in a memory location using multiple pointers.
- `update_pointer_offset`: Stores a returned value into a structure field and, if non-zero, increments a value at a specific offset within that structure.
- `update_pointer_indirect`: Stores a returned value into a structure field and, if non-zero, adds a constant to the value pointed to by that field.
- `allocate_and_initialize_object_array`: Allocates a block of memory and iterates through a bitmask to initialize a series of objects.
- `execute_recursive_state_sequence`: Calls a state machine routine and, if the returned value is non-zero, stores it in a memory location and performs three subsequent recursive calls using offsets from that value.
- `process_conditional_state_loop`: Executes a state machine routine and, based on bitwise checks of a byte, either stores a constant or performs a loop that repeatedly calls another routine with incrementing values.
- `update_state_with_constant`: Calls a state machine routine with a specific constant argument and stores the resulting value into a memory location through a pointer.
- `execute_state_machine_call`: Calls a state machine routine with zeroed arguments and stores the returned value into a memory location through a pointer.
- `perform_blitter_graphics_operation`: Configures and executes a series of Blitter operations by setting up transfer modes, area dimensions, and color data.
- `update_entity_state_and_buffer`: Determines a state identifier based on a bitmask check and copies a data block from a table to a local buffer.
- `update_calculated_value`: Calculates a new value by adding a constant to a shifted register value and stores the result in a memory location through multiple pointers.
- `process_entity_state_updates_2`: Iterates through a list of entity structures to update internal state values based on specific command bytes.
- `update_clamped_input_vectors`: Processes 16-bit input values by splitting them into two 8-bit components and calculating their differences from previous values.
- `update_clamped_values`: Updates two values in a structure by retrieving data from tables based on an index, applying clamping logic to ensure the results stay within specified minimum and maximum bounds.
- `call_empty_routine`: Calls another routine with a constant value of fifteen.
- `cleanup_global_pointers`: Clears specific memory locations and calls various routines based on the presence of non-zero values in several global pointers.
- `invoke_handler_sequence`: Executes three consecutive calls to another routine with varying arguments.
- `initialize_graphics_environment`: Attempts to open several library-related resources and initializes global pointers and memory structures based on the results.
- `call_data_copy_routine`: Retrieves a pointer from a global variable and passes it along with a parameter to a routine that copies data from a table into a buffer.
- `loop_and_call_function`: Iterates thirty-two times, passing a global value and a word from a provided memory location to a function pointer.
- `clear_and_update_buffer`: Clears a global value and sets a specific byte to zero at a location calculated from a parameter and a table.
- `execute_dual_copy`: Calls another routine twice with different parameter orders to copy data from a table to a destination.
- `call_function_with_data`: Calls a function with a constant value and then calls another function using a value from a table indexed by the upper bits of a parameter.
- `load_resource_data`: Retrieves data from a table using an index and performs a conditional library call based on the result.
- `process_table_entry_3`: Accesses a table using an index to retrieve a value and stores the result of a subsequent function call into a global variable.
- `dispatch_entity_callback`: Checks a value in a table at an offset derived from a parameter and calls another routine if that value is non-zero.
- `call_function_with_global`: Calls a function pointer using a global variable as an argument.
- `call_function_with_global_alt`: Calls a different function pointer using a global variable as an argument.
- `call_function_conditionally`: Calls a function pointer using two values retrieved from memory locations, with the order of arguments determined by a parameter.
- `call_function_with_indirect_values`: Calls a function using two values retrieved from memory locations pointed to by global variables.
- `call_function_with_parameter`: Calls a function pointer using a global variable and a provided word parameter.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `allocate_and_initialize_memory`: Allocates a memory block based on a provided size and initializes it by either clearing the entire range or setting a specific sub-section.
- `update_structure_state_2`: Updates a structure by performing conditional data copying and value clearing based on the result of a function call.
- `update_state_conditional_copy`: Updates a structure by calling a function to retrieve a value and then conditionally performing data copies or clearing operations based on the result.
- `process_conditional_state`: Executes several function calls using values from a structure, performs conditional data copying or clearing, and updates a local variable with a return value.
- `update_structure_state`: Performs a sequence of function calls to update a structure, including conditional data copying and value clearing based on a returned result.
- `perform_multiple_function_calls`: Executes a sequence of four function calls that manipulate values stored in a structure, including clearing a value and copying data between locations.
- `execute_object_callback`: Creates an object and executes a function through a pointer, returning the result of that function.
- `calculate_sum_and_store`: Sums three values from a table and a global variable, then stores the result as a 32-bit value in a table.
- `accumulate_table_values`: Performs a complex arithmetic accumulation involving multiple table values and a global variable, storing the result in a table.
- `return_uninitialized_stack_value`: This routine returns an uninitialized value from the stack.
- `add_scaled_global_to_table`: Adds a value derived from a table and a global variable to an existing table entry, scaled by a repeated addition of the global variable.
- `execute_system_orchestration`: Performs a complex sequence of system initializations, pointer resets, and conditional function calls using values from various tables and global variables.
- `dispatch_indirect_calls`: Calls a routine using a value from a table and then executes two subsequent function calls.
- `call_wrapper_routine_2`: Calls another routine and returns.
- `copy_and_call_function`: Copies two 32-bit values from a memory location to another and then calls a function pointer.
- `trigger_animation_initialization`: Calls two different routines with specific constant parameters to initialize internal states.
- `initialize_and_loop`: Sets several global word values, calls multiple routines, and executes a loop that continues as long as a routine returns a non-zero value.
- `reset_system_state`: Updates several global variables and calls multiple routines to reset specific system flags and pointers.
- `trigger_replay`: Calls another routine with a specific address and a global variable.
- `update_system_parameters`: Calls three different routines with specific parameters to update the current state.
- `conditional_reset_sequence`: Checks a global value and, if zero, executes a sequence of routines including a conditional call using a value from a table.
- `handle_half_state`: Checks a parameter to conditionally call a routine and then updates a global byte and calls another routine.
- `initialize_data_tables`: Fills five different tables with specific 16-bit values using a loop that runs four times.
- `accumulate_weighted_bytes`: Iterates through four pairs of parameters, multiplying bytes from a table by the parameters and summing them into a single byte.
- `calculate_weighted_values`: Iterates through loops to set table values, calculates a sum using a routine, and updates a table with calculated memory addresses.
- `initialize_game_parameters`: Clears several small arrays and sets specific configuration values based on a comparison of a short integer.
- `check_and_call_function`: Checks two consecutive return values from another routine and, if both are zero, calls a function pointer using two memory values.
- `execute_multiple_calls`: Calls several routines via pointers retrieved from memory, including one that uses a value from a table to index into another structure.
- `execute_conditional_callbacks`: Checks a global flag to either execute a sequence of four function calls or call one function and reset the flag.
- `evaluate_conditional_logic`: Performs several checks on global variables and updates a table entry based on the outcome of a function call.
- `call_offset_routine`: Calls another routine using a value calculated from a pointer and an offset.
- `recurse_with_parameter`: Checks a parameter and calls itself recursively with that parameter and a global variable.
- `call_library_function`: Calls a library function using a pointer and arguments retrieved from a table and a global variable.
- `filter_table_indices`: Initializes a table with maximum values and then populates it with indices from another table if a condition is met.
- `update_short_value`: Waits for a value at a specific memory offset to become non-negative and then stores a two-byte value into that same location.
- `recursively_call_routine`: Checks if a parameter is non-zero and, if so, calls itself with that parameter.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `process_data_tables`: Iterates four times to copy data between tables and perform conditional updates to various memory offsets based on specific values.
- `search_indexed_table`: Iterates through a table to find an entry matching specific criteria and returns its index or a modified value.
- `update_indexed_table`: Iterates through a table to perform arithmetic and conditional logic, potentially updating byte values based on comparisons and function return values.
- `update_indexed_table_values_2`: Iterates through a loop up to four times to perform arithmetic and conditional logic, modifying bytes in a table based on values retrieved from another table.
- `update_indexed_table_values`: Iterates through a loop to perform arithmetic and conditional logic, modifying bytes in a table based on values retrieved from another table.
- `modify_byte_via_index`: Calculates an offset using a value from a table and a short integer, then stores a negated byte at that location.
- `process_loop_logic`: Iterates through a loop to perform arithmetic on a value, performs conditional logic based on a function return, and modifies a byte in a table.
- `initialize_index_array`: Clears a range of memory and then populates an array with signed index values based on the contents of another table.
- `process_indexed_search`: Iterates through a table to find specific byte values, updates a secondary table with negated indices, and modifies global status bytes based on comparison results.
- `process_indexed_loop`: Iterates through a table to conditionally update memory locations and sets a two-byte value based on a comparison.
- `initialize_index_array_2`: Clears two bytes of memory and then conditionally sets a two-byte value to zero or one based on the contents of a table and another memory location.
- `zero_if_zero`: Checks if a two-byte value at a specific offset is zero and, if so, sets another two-byte value to zero.
- `poll_and_store_results`: Loops four times, calling a function pointer to retrieve values and storing the results into two different tables.
- `reset_and_load_pointers`: Calls two functions, clears a list of four pointers, loads new values from a table into that list, and then calls another routine for each non-zero pointer.
- `dispatch_and_cleanup`: Iterates through a list of four pointers to call another routine for each non-zero entry, then calls two final functions.
- `process_recursive_parameters`: Executes a function call using a value from a pointer and then calls itself recursively with modified parameters.
- `dispatch_indirect_calls_2`: Calls three different routines using a pointer from a table, passing various combinations of provided parameters.
- `process_recursive_parameter_table`: This routine performs recursive calls based on a value in a parameter structure.
- `call_with_structure_data`: Executes a routine via a pointer using four values extracted from a structure, performing this twice with different parameter combinations.
- `loop_parameterized_calls`: Executes a routine eight times in a loop, using values from two different tables to offset parameters passed to the routine.
- `execute_parameterized_calls_2`: Performs a series of function calls using values from a structure and tables, with different execution paths based on an input integer.
- `execute_parameterized_calls`: Calls a routine via a pointer using a value from a table, then calls another routine using four values retrieved from a structure.
- `call_wrapper_routine`: Calls a function via a pointer with three parameters, calls a second function via a pointer, and then calls another routine using the first parameter, the fourth parameter, and the result of the second function.
- `search_and_return_short`: Increments counters and advances through a null-terminated string in a table before returning a two-byte value.
- `increment_and_search_table`: Increments two counters in a loop and advances a third counter through a null-terminated string in a table until a limit is reached.
- `return_short_value`: Returns a two-byte value stored at a specific memory offset.
- `call_with_offset_parameters`: Calls a routine via a pointer using a single parameter, then calls another routine using four parameters where two are sums of input values.
- `call_via_offset_table`: Retrieves bytes from a table using an offset and calls a routine twice if a specific byte in the table is non-zero.
- `process_table_entry_2`: Extracts values from a table based on an index and calls three different routines with various combinations of those values and calculated offsets.
- `update_table_data`: Extracts multiple values from a table using an index and offsets, then calls three different routines with these values and updated pointers.
- `call_subroutine_via_pointer`: Calls a routine through a pointer stored in a global table using a passed parameter.
- `execute_indirect_call`: Fetches a function pointer from a table using an offset from a base address and calls it with a parameter.
- `execute_indirect_function_call`: Fetches a function pointer from a memory location and calls it using a value passed as an argument.
- `clear_memory_block`: Clears a large multi-dimensional array of words to zero using nested loops.
- `accumulate_table_values_2`: Calculates several memory offsets based on input parameters and a base address, then adds a value to the short integers stored at those locations.
- `add_value_to_table`: Adds a short value from a source to a value located at a calculated offset within a table.
- `update_state_and_dispatch`: Modifies values within a structure and executes a series of indirect function calls based on conditional logic and retrieved values.
- `update_state_callbacks`: Updates a status flag in a structure and executes several indirect function calls using values retrieved from that structure.
- `manage_state_callbacks`: Updates a status flag in a structure and executes several indirect function calls based on values retrieved from that structure.
- `update_state_and_recurse`: Updates a status flag in a structure and performs a series of indirect function calls using values extracted from that structure.
- `execute_concatenated_calls`: Calls a routine, stores the result, and then performs several subsequent function calls using concatenated or extracted data from a structure.
- `update_status_flags`: Modifies specific bytes in a global structure based on the value of a short integer and then calls two other routines with different parameters.
- `update_state_and_dispatch_callbacks`: Modifies status flags within a structure and executes a series of indirect function calls using values retrieved from that structure.
- `process_table_elements`: Iterates through a table to perform comparisons and updates values based on conditional logic and division.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a2).
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a7).
- `update_attribute_table`: Iterates through a nested loop to compare values from a table against a secondary data set and updates a destination structure.
- `update_player_attribute_table`: Iterates through a table of structures to update specific numeric values by comparing them against a secondary data set.
- `initialize_data_structures`: Calls a routine to obtain a value, stores it in a global variable, and clears four word-sized entries in a table.
- `pass_parameter_to_routine`: Calls a routine using a value stored in a global variable and a constant integer.
- `enable_dma_channel`: An 18-instruction routine that modifies the DMA controller register to enable a DMA channel.
- `update_coprocessor_registers`: Writes a longword and a word into hardware registers using an offset derived from a parameter.
- `update_coprocessor_hardware`: Writes a longword and a word into hardware registers using an offset calculated from a provided parameter.
- `enable_dma_channels`: Sets a global variable to a value that enables DMA channels via the DMACON register.
- `write_hardware_registers`: Writes two word-sized values into a hardware register or a memory-mapped area using an offset derived from a parameter.
- `dispatch_switch_logic`: Uses a parameter to index into a table and executes a switch statement that modifies global variables and calls another routine.
- `update_state_via_switch_2`: Updates various memory locations and performs arithmetic based on a value in a register using a switch statement.
- `initialize_table_constants`: Stores several constant two-byte values into memory locations and then calls another routine.
- `write_and_call`: Stores a two-byte value into a table at an offset determined by a short integer, then calls another routine.
- `return_immediately_6`: This routine is empty and returns immediately to its caller.
- `update_state_via_switch`: Updates various memory locations and performs arithmetic based on a value in a register using a switch statement.
- `return_immediately_3`: This routine performs no operations and returns immediately to its caller.
- `initialize_data_and_call_library`: Clears a range of memory using a loop, calls a library function, and updates two global variables with return values from other routines.
- `update_indexed_data_table`: Iterates through a range of indices to perform longword transfers.
- `process_data_table`: Iterates through a table to perform longword transfers between memory locations based on indexed values.
- `execute_conditional_callback`: Checks a global variable and calls a function pointer with specific arguments.
- `decrement_counters_and_check_limits`: Decrements a global short value and, if it reaches zero, resets two other global shorts while updating several other global shorts and memory locations.
- `reset_and_copy_data`: Calls two function pointers and then copies a longword from a pointer-based memory location to a global variable.
- `reset_and_update_pointer`: Calls a function pointer and then copies a longword from a memory location to a global variable.
- `clear_and_call_routines`: Clears a word in memory and executes two function pointers.
- `execute_function_pointer`: Calls a function through a pointer stored at a specific offset from a base address.
- `reset_value_and_call_functions`: Sets a global word to zero and calls two function pointers with specific constant arguments.
- `call_routine_twice_2`: Checks a global variable and, if non-zero, calls another routine twice using the value stored in that variable.
- `call_routine_twice`: Calls another routine twice using a value retrieved from a memory location.
- `format_numeric_display`: Decomposes an integer into individual digits and stores them in a local array.
- `process_list_with_logic`: Iterates through a list to update values, performs conditional assignments, and executes a function pointer for specific entries.
- `process_list_with_callbacks`: Iterates through a list to modify values and calls a function pointer for entries that do not match a specific constant.
- `iterate_and_call_callback`: Loops through a list and calls a function pointer for each entry that does not match a specific constant.
- `return_absolute_value`: Returns the absolute value of a passed integer.
- `fetch_word_from_offset`: Returns a word stored at an offset of eight bytes from a provided pointer.
- `copy_byte_sequence_2`: Copies a sequence of bytes from one memory area to another using a loop and returns the count of bytes copied.
- `copy_byte_sequence`: Copies a sequence of bytes from one memory area to another using a loop and returns the final index.
- `process_indirect_loop`: Executes a loop that calls another routine via a pointer from a table and updates a global variable based on the returned value and specific bit flags.
- `update_buffer_element`: Calculates an index based on two input parameters and a global offset, calls a routine with that index, and then potentially clears a byte in a buffer if a comparison with a third parameter succeeds.
- `format_numeric_display_2`: Decomposes an integer into individual digits and stores them in a local array.
- `parse_and_load_string_resource`: Copies a string from a provided pointer into a local buffer until a colon or null terminator is reached.
- `loop_callback_sequence`: Enters a nested loop structure that compares values in a list and executes indirect function calls until specific constant values are met.
- `return_constant_one`: Calls another routine and returns the value one.
- `execute_conditional_callbacks_4`: Iterates through a data structure to perform conditional function calls and updates a global variable based on comparison results.
- `execute_conditional_callbacks_3`: Iterates through data to perform conditional function calls and updates a global variable based on comparison results.
- `iterate_callback_list`: Iterates through a linked list or data structure to execute function pointers stored within the elements.
- `execute_conditional_callbacks_2`: Iterates through data to call another routine with various parameters and performs an indirect function call using a pointer stored in a global variable.
- `return_constant_one_2`: This routine returns the integer value one to its caller.
- `handle_state_transition`: Evaluates a parameter to determine a return value, otherwise calls another routine and conditionally executes a loop or a library function.
- `process_game_state_transition`: Updates internal state variables and executes a sequence of conditional subroutines based on a state value.
- `process_data_list`: Iterates through a list of five 16-bit values to pass them to another routine, then conditionally calls two other routines if a specific global flag is set, before clearing a global short.
- `call_routine_with_constant`: Calls a routine stored at a global pointer with the constant value zero-A0.
- `call_and_return_value`: Calls a routine and returns a 16-bit value stored in a global variable.
- `initialize_parameter_table`: Checks a global variable and, if non-zero, populates a series of memory locations with constant values and register data.
- `initialize_data_table`: Stores a series of constant values into various memory locations and a table.
- `decrement_and_update_counters`: Decrements a global short value and, if it reaches zero, resets two other global shorts; it also updates several global shorts and memory locations based on clamped arithmetic operations.
- `check_and_trigger_event`: Checks several global conditions and, if met, calls a routine and conditionally executes two more routines based on the result and global short values.
- `update_game_state_parameters`: Updates a set of state variables and configuration values in a data structure based on an input index.
- `set_short_values`: Stores the values one and twenty-four into two consecutive memory locations.
- `set_short_values_alt`: Stores the values one and forty into two consecutive memory locations.
- `update_structured_data_2`: Calculates an index and copies several values from different tables into a structure based on a conditional check.
- `update_structured_data`: If a value in a table exceeds two, it copies multiple values from other tables into a structure using an index.
- `set_structure_fields`: Assigns values to several memory locations, including values copied from a table and constants, based on a comparison.
- `initialize_structure_fields`: Sets four specific memory locations to the values one, one, four-hundred, and four-hundred.
- `populate_player_attribute_structure`: Updates a data structure by selecting specific values from multiple tables based on an input index.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `fill_buffer_with_count`: Iterates a specified number of times, writing the current loop counter value as a byte to a destination buffer and incrementing the pointer.
- `copy_nested_table_data`: Uses nested loops to call another routine and store the returned bytes into a destination buffer based on specified counts.
- `iterate_and_copy_bytes`: Loops through a table to call another routine and stores the returned byte into a destination buffer until a specific condition is met.
- `process_nested_data_loop`: Iterates through a data structure, calling another routine to retrieve values and storing them in a buffer while managing nested loop counters.
- `populate_buffer_from_list`: Fills a destination buffer with bytes retrieved from a recursive data structure until a termination value is encountered.
- `fetch_linked_list_byte`: Recursively traverses a linked list structure to fetch a byte from a pointer and increment a counter.
- `increment_and_fetch_byte`: Fetches a byte from a pointer and increments that pointer.
- `traverse_and_update_linked_list`: Checks bit flags in a structure to determine if it should recursively traverse a linked list or iterate through a memory table.
- `update_buffer_state_2`: Updates status flags and pointers within a structure, potentially iterating through a list of memory locations based on a bitwise check.
- `set_bit_in_byte`: Performs a bitwise OR to set the fifth bit of a byte in a structure.
- `update_pointer_and_calculate`: Updates two memory locations based on a third, increments a pointer, and returns a combined value.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `find_string_pointer`: Searches for a specific byte within a null-terminated string and returns a pointer to its location.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `return_immediately_4`: This routine is empty and performs no operations before returning to the caller.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a jump table.
- `process_input_data`: Calls a routine to read data into a global variable and then performs a conditional call based on a bitwise check and a global variable.
- `read_input_data`: Attempts to read data from a source and stores a result or an error value into a global variable based on the success of the operation.
- `perform_io_read`: The routine attempts to read data from a source and, if the operation fails, calls an error routine and stores a result into a global variable.
- `return_immediately_5`: This routine performs no operations and immediately returns to its caller.
- `update_table_entries_2`: The routine iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `update_table_entries`: Iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `manage_library_resources`: Interacts with library structures to check status, execute functions, and update a pointer in a table.
- `safe_memory_copy`: Copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `clear_memory_and_call_function`: Clears a block of memory using a loop, updates a global variable, and executes a function pointer based on a bit test.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `parse_quoted_string_list`: Parses a source string into a list of individual tokens, handling whitespace and quoted substrings.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit_2`: Calculates the length of a source string and copies it to a destination buffer until a null terminator is reached or a maximum length is hit.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `copy_string_pad_zeros`: The routine copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: The routine takes two 32-bit values from registers and returns a 64-bit product formed from the high and low 16-bit halves of each operand.
- `process_window_data_2`: The routine traverses a nested data structure to retrieve values and updates a global structure with results from indirect function calls.
- `process_window_data`: Traverses a nested data structure to retrieve values and updates a global structure with results from indirect function calls.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `perform_signed_math`: Performs signed division and fixed-point math, then negates the result if the input sign is negative.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call_2`: The routine calls another routine and then returns.
- `perform_fixed_point_math`: Performs division or multiplication operations on 32-bit values split into 16-bit components using carry propagation.
- `swap_and_rotate_words`: The routine swaps the high and low words of a value in a register, then repeatedly doubles a pair of words while propagating carry, subtracting a constant if the intermediate value exceeds a threshold, and decrements a counter fifteen times.
- `update_circular_buffer_state`: Updates a circular buffer by modifying internal pointers and flags while performing conditional writes to a memory location.
- `increment_and_store_byte`: Increments a pointer stored in a register and stores a byte from another memory location into the original pointer location.
- `manage_circular_buffer_2`: Manages a circular buffer by updating pointers and flags within a structure and performing conditional writes to a memory location.
- `manage_circular_buffer`: Updates pointers and flags within a structure to manage data flow and performs conditional writes to a memory location.
- `update_circular_buffer`: Manages a circular buffer by updating a pointer and a counter within a structure.
- `update_buffer_pointer`: Manages a pointer and a counter within a structure, potentially updating a byte and a bitmask.
- `update_buffer_state`: Updates a structure with a new pointer and length, modifies a bitmask, and stores a byte from a source into a destination.
- `update_status_structure`: Calls another routine and updates several fields within a provided structure based on the returned value.
- `update_status_flags_2`: Modifies bytes and words at offsets from a pointer and conditionally sets a bit in a byte based on the return value of another routine.
- `allocate_and_store_pointer`: The routine calls a function to allocate memory, then stores two 32-bit double-words into the new buffer and updates a global pointer.
- `copy_data_block`: Copies two 32-bit double-words from two different memory locations into a new destination buffer.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `check_interactivity_status_2`: Checks if a value retrieved from a table using an offset is non-zero and updates a global variable accordingly.
- `check_interactivity_status`: The routine retrieves a value from a table using an offset and checks if the resulting value is non-zero.
- `return_zero_2`: An empty routine that returns zero.
- `process_output_data`: Calls a routine to write data to a source and then performs a conditional call based on a bitwise check and a global variable.
- `write_output_data`: Attempts to write data to a source and stores a result or an error value into a global variable based on the success of the operation.
- `perform_io_write`: The routine attempts to write data and, if the operation fails, calls an error routine and stores a result into a global variable.
- `execute_conditional_io_write`: This routine is empty.
- `execute_conditional_call`: Calls another routine and, if a specific bit is set in the return value and a global variable is non-zero, calls a third routine.
- `execute_write_sequence_2`: The routine calls another routine and then returns zero.
- `execute_write_sequence`: Calls a function via a pointer to process a global variable and then executes two subsequent routine calls.
- `call_conditional_function`: Checks if a pointer at a specific offset is non-zero before calling it, then calls another function located at a different offset.
- `process_table_entry`: Validates an index against a table size and, if valid, calls another routine using a pointer from the table before clearing that table entry.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `call_library_vector_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 6 of that base, returning its result to the caller.
- `call_library_vector_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 12 of that base, returning its result to the caller.
- `execute_indirect_call_2`: Performs an indirect call to a function pointer retrieved from a table via an offset.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_absolute_call`: Performs an indirect call to a function pointer stored at a specific memory location.
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
- `execute_indirect_function_3`: Calculates an address by adding an offset to a value stored in a register and then calls the function located at that address.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `execute_indirect_function_4`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested structure within a global data table and executes it.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_3`: Calculates an address by adding a negative offset to a value in a register and executes the code located at that resulting pointer.
- `execute_indirect_function_pointer_4`: Retrieves a function pointer from a nested offset within a data structure and executes it.
- `execute_indirect_function_5`: Calculates an address by adding an offset to a value in a register and executes the function located at that memory location.
- `execute_indirect_function_pointer_5`: Fetches a function pointer from a nested offset within a data structure and executes it.
- `execute_indirect_call_3`: Performs an indirect call to a function pointer located at an offset from a value in a register.
- `execute_offset_call`: Calls a function by adding an offset to a pointer retrieved from a table.
- `execute_table_call`: Performs an indirect call using an offset applied to a pointer found within a table.
- `execute_register_offset_call`: Executes a call to a function address calculated from a register and a constant offset.
- `execute_table_offset_call`: Calls a function located at a specific offset from a pointer stored in a table.
- `execute_indirect_jump_2`: Performs an indirect call to a function address derived from a register and a constant.
- `execute_pointer_offset_call`: Calls a function by applying an offset to a pointer retrieved from a data structure.
- `execute_indirect_call_14`: Performs an indirect call to a routine address retrieved from a table using an offset.
- `execute_complex_offset_call`: Performs an indirect call to a function address calculated from a table entry and an offset.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `execute_direct_indirect_call`: Executes an indirect call to a function address stored at a specific offset from a register.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `execute_table_indexed_call`: Calls a function using an offset applied to a pointer retrieved from a table.
- `execute_indirect_call_4`: Performs an indirect call to a function pointer retrieved from a nested offset within a data structure.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_call_5`: Loads a function pointer from a table using an offset and executes it.
- `execute_indirect_call_6`: Calls a subroutine located at a specific offset within a structure pointed to by a register.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_call_7`: Retrieves a function address from a table and jumps to it.
- `call_library_vector_486`: Library stub: loads a library base from a global variable and calls the library function at offset minus 486 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `execute_indirect_call_8`: Executes a function by dereferencing a pointer found at a specific offset in a data block.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_indirect_call_9`: Performs an indirect jump to a routine located via a pointer in a data structure.
- `execute_indirect_call_10`: Calls a routine through a pointer stored at a specific offset in a table.
- `execute_indirect_call_11`: Uses a pointer from a data structure to execute another routine.
- `execute_indirect_call_12`: Dereferences a pointer from a table and transfers control to that address.
- `execute_indirect_call_13`: Executes a routine by following a pointer through multiple levels of indirection.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_indirect_function_2`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `call_indirect_function_3`: Executes a function located at an offset within a structure pointed to by a global variable.
- `call_indirect_function_4`: Calls a subroutine via a pointer found by applying an offset to a base address.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_indirect_function_5`: Uses a pointer from a table to jump to another routine.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.

### TV Sports Basketball (1990)(Cinemaware)(Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `initialize_system_interrupts`: Performs system initialization tasks including finding device names and copying data, while manipulating the interrupt enable bit in the INTENA register.
- `check_system_status`: Checks if a value in a register matches specific constants and performs a software interrupt if it does not.
- `copy_data_loop`: Repeatedly copies data from one memory location to another based on a loop counter and a termination condition.
- `find_device_name`: Searches for a device name using a library call and performs a software interrupt if a specific condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_empty_loop`: The routine is empty and consists only of an infinite loop.
- `execute_empty_loop_2`: This routine contains no functional code and enters an infinite loop.
- `execute_empty_loop_3`: The routine is empty and performs an infinite loop.
- `execute_empty_loop_4`: This routine is empty and results in an infinite loop.
- `execute_empty_loop_5`: The routine is empty and consists of an infinite loop.
- `execute_empty_loop_6`: The routine contains an infinite loop and performs no operations.
- `execute_empty_loop_7`: This routine consists of an infinite loop with no functional logic.
- `execute_empty_loop_8`: The routine is an empty infinite loop.
- `execute_empty_loop_9`: This routine performs no actions and enters an infinite loop.
- `execute_empty_loop_10`: The routine is an infinite loop that does not modify any data.
- `execute_empty_loop_11`: This routine contains only an infinite loop structure.
- `execute_empty_loop_12`: The routine executes an infinite loop without side effects.
- `execute_empty_loop_13`: This routine is an empty infinite loop.
- `execute_empty_loop_14`: The routine performs no work and loops indefinitely.
- `execute_empty_loop_15`: This routine consists of an infinite loop with no instructions.
- `execute_empty_loop_16`: The routine contains an infinite loop.
- `execute_empty_loop_17`: This routine consists of an infinite loop.
- `execute_empty_loop_18`: The routine performs an infinite loop.
- `execute_empty_loop_19`: This routine is an infinite loop.
- `execute_empty_loop_20`: The routine executes an infinite loop.
- `execute_empty_loop_21`: This routine contains an infinite loop.
- `execute_empty_loop_22`: The routine is an infinite loop.
- `nop_padding_gap`: This routine is empty and consists of a single instruction that serves as padding within the executable segment.
- `execute_infinite_loop`: The routine enters an infinite loop that prevents further execution of the calling code.
- `execute_infinite_loop_2`: The routine enters an infinite loop that performs no operations.
- `execute_empty_loop_23`: The routine consists of an infinite loop.
- `execute_infinite_loop_3`: The routine enters an infinite loop that prevents further execution.
- `execute_empty_loop_24`: The routine performs an endless loop.
- `execute_empty_loop_25`: This is an infinite loop routine.
- `execute_empty_loop_26`: The routine enters an infinite loop.
- `nop_padding_gap_2`: This routine is empty and consists of non-functional padding bytes within the executable segment.
- `execute_infinite_loop_4`: The routine enters an infinite loop that prevents further execution of the current thread.
- `execute_empty_loop_27`: This routine consists of a continuous loop.
- `execute_empty_loop_28`: The routine performs a loop that never terminates.
- `nop_padding`: The routine is empty and contains no executable instructions.
- `execute_infinite_loop_5`: The routine performs an infinite loop that prevents further execution of the program flow.
- `execute_infinite_loop_6`: Performs an endless loop of execution.
- `execute_infinite_loop_7`: Contains a loop that never terminates.
- `nop_padding_2`: This routine is empty and consists of a single instruction that performs no operation, serving as padding within the executable segment.
- `execute_infinite_loop_8`: Traps execution in a loop.
- `execute_infinite_loop_9`: Enters a state of perpetual looping.
- `execute_infinite_loop_10`: Executes a loop without an exit condition.
- `execute_empty_loop_29`: The routine consists of an infinite loop that performs no operations.
- `execute_infinite_loop_11`: Stays within a loop forever.
- `execute_infinite_loop_12`: Repeats the same instruction indefinitely.
- `execute_empty_loop_30`: The routine contains a single instruction that results in an infinite loop.
- `execute_empty_loop_31`: This unreached routine consists of a single instruction that creates an infinite loop.
- `execute_empty_loop_32`: The routine performs a single instruction that causes an infinite loop.
- `execute_empty_loop_33`: This routine executes a single instruction that results in an infinite loop.
- `execute_empty_loop_34`: The routine consists of one instruction that enters an infinite loop.
- `execute_empty_loop_35`: This routine contains a single instruction that results in an infinite loop.
- `execute_empty_loop_36`: The unreached routine executes a single instruction that forms an infinite loop.
- `execute_empty_loop_37`: This routine performs a single instruction that leads to an infinite loop.
- `execute_empty_loop_38`: The routine consists of a single instruction that results in an infinite loop.
- `execute_empty_loop_39`: This routine executes a single instruction that causes an infinite loop.
- `execute_single_instruction`: Performs a single instruction that overlaps with another instruction.
- `execute_unreached_instruction`: This routine is unreached and contains a single instruction.
- `execute_single_instruction_2`: Executes one instruction that is entered from outside the static flow.
- `nop_padding_byte`: This routine is empty and consists of a single instruction that performs no operation.
- `execute_single_instruction_3`: Performs a single instruction that is executed during a reference run.
- `execute_infinite_loop_13`: The routine enters an unconditional infinite loop that prevents further execution of the current thread.
- `execute_infinite_loop_14`: The routine enters an infinite loop that never exits.
- `execute_infinite_loop_15`: The routine enters an unconditional infinite loop that prevents further execution of the calling code.
- `execute_infinite_loop_16`: The routine enters an endless loop that prevents further execution of the calling sequence.
- `execute_overlapping_instruction`: Executes one instruction that overlaps with another instruction.
- `execute_infinite_loop_17`: The routine performs an infinite loop that prevents further execution of the calling sequence.
- `execute_infinite_loop_18`: The routine consists of an infinite loop that performs no operations and never returns.
- `execute_infinite_loop_19`: The routine performs an infinite loop that prevents further execution of the calling code.
- `execute_infinite_loop_20`: This routine enters an infinite loop that prevents further code execution.
- `execute_instruction`: Carries out a single instruction.
- `execute_infinite_loop_21`: The routine enters an infinite loop that does not perform any operations.
- `nop_padding_gap_3`: This routine is empty and consists of a single non-operational instruction used for alignment or padding within the executable segment.
- `execute_infinite_loop_22`: This routine enters an infinite loop that prevents further execution of the calling code.
- `nop_padding_sequence_3`: This routine is empty and consists of non-functional data bytes that do not execute any instructions.
- `manage_game_state_logic`: Updates a complex state structure by resetting various flags, counters, and timers based on conditional checks.
- `clear_memory_blocks`: Clears a series of bytes in a loop and then sets two specific words to zero.
- `execute_indirect_calls`: Calls another routine and then executes two functions via pointers stored in a table.
- `enter_infinite_loop`: An infinite loop that performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutines_via_pointers`: Executes two subroutines through pointers stored at offsets from a register and returns zero.
- `initialize_circular_lookup_tables`: Populates multiple synchronized tables with incrementing index values and their offsets to create a circular mapping.
- `update_sprite_attribute_tables`: Iterates through a list of values to update multiple synchronized data tables using an index and an offset.
- `initialize_memory_values`: Sets several global variables to specific constant values and clears a small block of memory by writing zeros.
- `clear_memory_blocks_2`: Clears two separate memory regions by writing zero to twelve consecutive words through a loop.
- `copy_table_data`: Copies four words from a source table into a destination buffer using an index derived from a parameter and a constant offset.
- `initialize_asset_buffer`: Allocates memory for a data structure and initializes its header with a specific magic signature.
- `execute_callback_chain`: Iterates through a list of function pointers and executes them sequentially.
- `process_callback_and_memory_tables`: Iterates through a list of function pointers and executes them, followed by calls to free or manipulate memory blocks based on several conditional checks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_game_state_transitions`: Updates a complex state machine by resetting various control flags, counters, and memory buffers based on conditional checks.
- `update_sprite_attributes`: Iterates through a list of active objects to update their color, transparency, and priority attributes.
- `calculate_index_offset`: Extracts a byte from a word and returns a calculated offset if the byte does not match a specific list of excluded values.
- `return_stack_frame`: Removes the stack frame held in a5 and returns.
- `return_immediately_2`: This routine performs no operations and returns to its caller.
- `validate_and_offset_index`: Checks if a value in a register falls within a specific range of excluded constants.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `clear_word_memory`: Stores a zero word into a memory location calculated from a pointer.
- `update_data_pointers`: Calls two subroutines via pointers and then copies three longwords from a source table into a destination table.
- `call_subroutines`: Calls two subroutines via pointers, passing zero as an argument to the first one.
- `update_state_counters`: Updates several state variables and counters through a pointer based on nested conditional checks.
- `set_status_values`: Sets bytes and words in memory based on the value of an input parameter and calls a subroutine via a pointer.
- `update_status_and_loop`: Sets several memory values, enters a loop that updates a value using a modulo operation via a subroutine call, and then calls another routine.
- `call_with_table_value`: Calls a subroutine using a value retrieved from a table based on an input parameter.
- `update_game_state_logic`: Updates various game state flags and tables based on an input parameter through a large switch statement.
- `fill_memory_loop`: Iterates through a range and for each index stores a value from a global variable and a parameter into two different tables, then calls a subroutine.
- `iterate_table_elements`: Loops through a range of indices to perform data transfers and indirect function calls based on table contents.
- `process_data_table_entry`: Retrieves a word from a table using an index and performs conditional updates and indirect function calls based on table values.
- `process_attribute_table`: Iterates through a range of indices and performs a mathematical transformation on shortwords from one table before storing them in another table.
- `update_game_state_logic_2`: Updates game state flags and tables based on an input parameter through a large switch statement.
- `copy_value_from_table`: Fetches a word from a table using an index and an offset, then stores it in a local variable.
- `search_data_table`: Iterates through a table of sixty-byte structures to find a specific pattern of bytes and returns the resulting index and offset.
- `execute_callback_loop`: Repeatedly calls a function pointer stored in a table as long as two specific short values are non-zero, then calls two other functions, one with a constant value.
- `parse_string_table`: Iterates through a table of data, searching for a semicolon character to terminate a string and replacing it with a null byte.
- `copy_sprite_attribute_data`: Copies blocks of attribute data from a source table to a destination buffer using nested loops.
- `search_attribute_table`: Searches a table for a specific byte value at certain offsets and returns a value based on whether the byte is found at an even or odd position.
- `fetch_conditional_word_2`: Retrieves a word from memory by checking a bitmask against a value derived from a global variable and a parameter.
- `fetch_conditional_word`: Retrieves a word from a memory location based on a bitmask check against a control byte.
- `nop_padding_sequence_4`: This routine is empty and performs no operations.
- `get_conditional_word`: Returns a word from a memory location based on a switch case that checks specific bits in a byte or word.
- `return_word_from_offset`: Returns a word from a memory location calculated by adding a negative offset to a value in a register.
- `process_game_state_transitions`: Updates game state flags and manages memory transfers between data tables based on conditional logic and index values.
- `calculate_bit_index`: Calculates a bit index and a byte offset from a given input value using division and modulo operations.
- `update_offset_value`: Updates a global short value by adding an offset derived from a table, while performing bounds checking and resetting the value if it falls outside a specific range.
- `update_index_and_clamp_value`: Calculates a new value based on a table lookup and increments or decrements an index while skipping identical consecutive entries in a data array.
- `update_global_data`: Copies four values from a table into several global variables and sets three others to zero or one.
- `call_function_with_offset`: Calls a function pointer using a value calculated from a global variable and an offset derived from two input parameters.
- `swap_and_update_memory`: Performs multiple memory transfers between different locations and updates several word values based on an input parameter.
- `clear_and_search_memory`: Clears a block of memory with zeros and then calls two subroutines to perform searches or data transfers.
- `process_table_values`: Calls multiple functions to retrieve values from a table and passes them to other functions via a global variable.
- `execute_nested_function_calls`: Calls a function pointer using a value from a structure and performs a conditional recursive call based on a short value.
- `conditional_data_transfer`: Checks two global shorts and, if both are non-zero, updates a byte in a structure and calls a function with specific data.
- `write_constant_to_structure`: Calls two functions, one of which writes a constant value to a location within a structure.
- `return_from_subroutine_2`: The routine performs a return from a subroutine.
- `rotate_table_values`: Performs a series of word-sized transfers between various memory locations and global variables.
- `increment_table_counters`: Clears a small buffer and then iterates through a table to increment values in a buffer based on specific byte comparisons.
- `process_data_structure`: Retrieves multiple values from a structure through various offsets and performs conditional writes to a destination memory area.
- `fetch_multiple_values`: Retrieves three values through a subroutine using different offsets and stores the result in a local variable.
- `fetch_and_store_data`: Calls a function multiple times to retrieve values from a structure and stores the results in a local variable before conditionally setting a byte in a global variable.
- `perform_block_transfers`: Calls a function multiple times to transfer blocks of data between different memory locations based on the result of the first call.
- `update_data_structure`: Retrieves values from a structure through various offsets, performs block transfers, and writes incremented byte values to a destination memory area based on calculated offsets.
- `accumulate_table_values_3`: Iterates through nested loops to add values from one table into another table at calculated offsets.
- `compare_and_update_table`: Iterates through a loop to compare values in one table with another and updates the second table if the first is larger.
- `transfer_and_copy_data`: Copies several values from a structure into local variables and, if a function returns non-zero, performs a block memory transfer and calls two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_for_byte`: Iterates through a table looking for a specific byte value and then calls another routine three times with different offsets.
- `update_index_table`: Updates a destination table by copying bytes from a source table based on values found in an index table.
- `sort_word_pairs`: Iterates through a table to compare and swap word pairs based on matches found in another table.
- `update_data_from_index_table`: Uses a byte from an index table to fetch and store word pairs into a destination table based on a provided parameter.
- `nop_padding_sequence_5`: This routine is empty and consists of three instructions that perform no operation.
- `reset_and_partition_entity_table`: Clears a two-dimensional data table and then partitions its entries into two separate groups based on a threshold value.
- `sort_table_entries`: Performs a nested loop to compare values within a table and calls other routines based on the comparison results.
- `compare_table_entries`: Compares two values from a table using an index derived from a parameter and calls another routine based on the comparison result.
- `sort_entities_by_distance`: This routine performs a bubble sort on a subset of an entity array based on a calculated distance value.
- `swap_table_entries`: Swaps two pairs of words located at specific offsets within a table indexed by a parameter.
- `process_indexed_data`: Performs a series of indirect function calls using values from a table and passes a sum of two short integers to another routine.
- `process_collision_data`: Clears a structured data table and then iterates through a secondary buffer to detect specific byte value transitions.
- `search_and_store_indices`: Iterates through a table to compare values derived from two arrays, storing an index and a calculated difference into a separate table when a match is found.
- `update_memory_via_pointer`: The routine performs a sequence of fifteen instructions to modify data in memory.
