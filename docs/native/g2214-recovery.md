# Game 2214 (tv sports basketball (1990)(cinemaware)[cr pna][a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02214). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1828564 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 29 of 29 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 689 of 689 named (0 author symbols), 689 of 689 described (215 by rule, 458 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4598 of 4598 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 400361 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16325 of 16325 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 627183 |
| code | 54040 |
| data | 789868 |
| padding | 12897 |
| unused | 344576 |

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
| Disk-Validator | 1 | 10 | 2 |
| paranoimia | 2 | 83 | 3 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 1 of 2).adf:bb | 3 | 7843 | 388 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 1 of 2).adf:bootblock | 1 | 11 | 1 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 72 | 72 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 220 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 453 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2273 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1073 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 1 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 2 of 2).adf:Fluff.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 2 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 205 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 193 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 522 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 668 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 2 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 92 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- tv sports basketball (1990)(cinemaware)[cr pna][a3](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 1 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][a3](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 2 of 2).adf:Fluff.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][a3](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 2 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][a3](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 2 of 2).adf: blocks 940-940 (bytes 481280-481792) outside the filesystem hold data, not code (opcode density 0.020, entropy 7.17)
- 3 never-loaded region(s), 400361 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 1536 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_function_pointer_2` (More): Calls a routine with a global variable, calls another routine with no arguments, and then invokes a function pointer stored in an address register. Kind: indirect function call/control flow obfuscation. Pass path: the branch taken by the dereferenced pointer unaff_A3.

## Functions


### More

- `initialize_resource_loader`: Initializes a global data structure by calling routines to restore patch states and load file metadata.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `process_entity_state_logic`: Evaluates an entity's state byte against specific constant values to determine branching logic.
- `execute_function_pointer_2`: Calls a routine with a global variable, calls another routine with no arguments, and then invokes a function pointer stored in an address register.
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
- `recursively_process_buffer_segments`: The routine recursively calls another routine using a pointer and a size derived from a structure field, potentially updating that field with a positive return value.
- `update_scroll_window`: Calculates a target position based on provided offsets and compares it against a window's current bounds.
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
- `store_byte_to_pointer`: Stores a byte from a register into the memory location pointed to by a register.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: The routine attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: The routine removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_and_branch`: Moves a long value from one register to another and performs a relative branch.

### Assign

- `reconstruct_resource_metadata_table`: Traverses a linked list to determine a maximum value and then populates a structured table with metadata and incremental values.
- `transfer_register_and_branch_3`: Moves a longword from one data register to another and then performs a short relative branch to another location.

### EndCLI

- `populate_resource_metadata_table`: Traverses a linked list to determine a maximum value and then populates a table with incremental data.
- `transfer_register_and_branch_4`: Copies the contents of one data register to another and then performs a short relative branch to another location.

### Dir

- `populate_resource_metadata_table_2`: Iterates through a linked list to determine a maximum value and then populates a table with incremental data.
- `transfer_register_and_branch_5`: Moves the contents of one data register into another and performs a short unconditional branch to another location.

### Echo

- `populate_resource_metadata_table_3`: Iterates through a linked list to determine a maximum value and then populates a table with incrementing data.
- `transfer_register_and_branch_6`: Moves a longword from one data register to another and performs a short relative branch to another location.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Execute

- `populate_resource_metadata_table_4`: Iterates through a linked list to determine a maximum value and then populates a table with incrementing data.
- `transfer_and_branch_2`: Moves a longword from one data register to another and then performs a short unconditional branch to another location.

### Copy

- `reconstruct_resource_metadata`: Iterates through a linked list to determine a maximum value and then populates a structured table with metadata and incremental values.
- `transfer_register_and_branch_7`: Copies a longword from one data register to another and then performs a short relative branch.

### Makedir

- `populate_resource_metadata_table_5`: Iterates through a linked list to determine a maximum value and then populates a table with incrementing data.
- `transfer_register_and_branch_8`: Moves a longword from a source data register to a destination data register before executing a relative branch.

### CD

- `reconstruct_resource_table`: Iterates through a linked list to find a maximum value and then populates a data structure with a sequence of incrementing values.
- `transfer_register_and_branch`: Moves the contents of one data register into another and then performs a short relative branch to another location.

### EndIf

- `return_zero`: This routine is empty and simply returns a value of zero to the caller.

### If

- `rebuild_resource_table`: Iterates through a linked list to find a maximum value and then populates a table with incrementing data.
- `transfer_register_and_branch_2`: Moves the contents of one data register into another and performs a short relative branch to another location.

### IconX

- `initialize_environment_and_parse_args`: Initializes system libraries and allocates memory for a state structure.
- `update_and_cleanup_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start by referencing text for Workbench-only use and utilizing the icon library.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a 260-byte buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `concatenate_and_dispatch`: Copies one null-terminated string into a destination buffer, appends a second null-terminated string to that same buffer, and then calls another routine using the resulting buffer and the combined string length as arguments.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal_2`: Converts a null-terminated string representing a signed decimal integer into a numeric value.
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
- `invoke_recursive_stack_parameter`: Calls another routine while passing two parameters and a pointer to a specific stack location.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `execute_and_verify_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over the subsequent stack buffer until a null terminator is reached.
- `store_byte_indirect`: Stores a byte from a register through a pointer.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer`: Stores a byte from a data register into a memory location pointed to by a register.

### Disk-Validator

- `perform_decryption_loop`: Performs a loop that XORs bytes from a memory location with a constant and decrements a counter.
- `copy_word_from_offset`: Copies a word from one offset in a data structure to another offset within the same structure.

### paranoimia

- `unpack_compressed_stream`: Unpacks a compressed stream by reading bits from its end and copying back-referenced runs backward into memory, then checks a checksum.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 1 of 2).adf:bb

- `initialize_system`: Clears a block of memory and then executes several function pointers retrieved from a global structure.
- `update_string_metadata`: Compares two values, copies a sequence of words from a source to a destination, and updates a string or buffer with a length and certain characters.
- `swap_linked_list_nodes`: Checks a pointer against a global value, swaps two adjacent pointers in a linked list, and updates a global pointer to point to the new head or a node.
- `process_and_delay_nodes`: Iterates through a list of pointers, calls a routine for non-zero entries, clears the entries, and applies a delay to two global pointers.
- `allocate_and_initialize_buffer_chains`: Allocates memory for two separate sequences of data, populates them with incrementing values, and manages linked lists of nodes.
- `copy_data_table`: Copies thirty-two words from a table into a destination buffer pointed to by a parameter.
- `update_buffer_recursively`: Recursively updates a buffer or a table entry with a value from a parameter, or updates a pointer-based destination if a certain condition is met.
- `create_stack_frame_and_branch`: Creates a new stack frame and performs a short relative branch to another routine.
- `update_table_recursively`: Recursively updates a table by incrementing a counter and modifying values through a pointer until a termination condition is met.
- `wait_for_value_match`: Enters a loop that continues until the contents of two memory locations are equal.
- `rotate_bits_in_memory`: Calls a library function, stores the result in a global variable, adds a constant, and then performs a bitwise rotation or shift-based update to that global variable.
- `calculate_leading_zeros`: Determines the number of leading zeros in a value and uses it to index into a table for bitwise operations.
- `get_stack_pointer`: Returns a value from an offset relative to a pointer.
- `branch_to_subroutine`: Calls a subroutine and then performs a short branch to another location.
- `decrement_and_execute_list`: Decrements two global short values, updates a third global short based on a subtraction, and executes a series of functions via a linked list.
- `update_global_state`: Copies a value from one global variable to another and updates several global shorts or bytes with constant values.
- `insert_linked_node`: Inserts a new node into a linked list if the second element of the provided structure is zero.
- `update_pointer_table_2`: Checks a value at a memory offset and conditionally moves a longword from one location to another.
- `update_pointer_table`: Moves a longword from a memory offset to a destination, compares a value at an offset to zero, and conditionally updates a pointer in a table.
- `assign_pointer_and_clear`: Copies a pointer from one location to another and then clears the value at the offset of that pointer.
- `reset_global_pointer`: Calls another routine with specific arguments if a global variable is non-zero, then clears that global variable.
- `update_blt_registers`: Updates several bitplane-related registers and a DMA controller register using values or calculated bitmasks while checking a global flag.
- `nop_padding`: This routine is empty and performs no operations.
- `calculate_blit_parameters`: Calls a routine that calculates blitter area dimensions and source or destination offsets based on a data structure.
- `calculate_blit_area_parameters`: Calculates blitter area dimensions and source/destination offsets based on a data structure.
- `configure_blit_parameters`: Calculates and writes various values to hardware blitter registers and memory locations based on a data structure and a loop.
- `update_blit_parameters`: Iterates through a data structure to calculate and update values used for Blitter operations.
- `setup_blitter_area_transfer`: Calculates source and destination coordinates and dimensions for a Blitter operation.
- `return_immediately`: This routine performs an immediate return from a subroutine.
- `initialize_input_state`: Sets certain memory or hardware registers to a fixed value and fills a buffer with a constant value using a loop.
- `execute_indirect_call_2`: Performs an indirect call to a routine using a pointer or address stored in a register or memory.
- `call_indirect_function`: Calls a function pointer located at an offset from a global pointer.
- `clear_and_combine_bytes`: Clears certain bytes in a table and returns a value constructed from those bytes and a third byte.
- `setup_stack_frame`: Sets up a stack frame and initializes two registers before branching to another routine.
- `iterate_and_clear_table`: Iterates through a table, clearing bytes and calculating a bit-shifted value based on table contents.
- `clear_table_bytes`: Clears certain bytes in a table and returns a value constructed from those bytes.
- `setup_stack_frame_alt`: Initializes a stack frame and registers before performing a short branch to another location.
- `clear_and_shift_bit_table`: Iterates through a table, clearing bytes to zero and updating a bit-shifted value in a secondary or adjacent memory location.
- `process_linked_list_nodes`: Iterates through a linked list, calling a function with certain parameters and potentially calling it a second time if a bit or a global flag is set.
- `initialize_buffer_structure`: Calls a function to get a pointer, then populates a buffer with values from a global variable and a parameter.
- `execute_loop_calls`: Iterates through a range defined by a parameter and calls a function using a value constructed from bit-shifted components.
- `transform_nibble_data`: Iterates through a table of words to perform bitwise manipulations on nibbles and stores the results in a local buffer.
- `execute_sequence_with_checks`: Calls a routine repeatedly until a certain value is returned, then updates a global variable with a constant value.
- `parse_compressed_data_stream`: Iterates through a data buffer using a state machine to decode variable-length instructions.
- `parse_compressed_data_stream_2`: Decodes a sequence of bytes from a pointer-based table by interpreting control bits to determine how many subsequent bytes to extract or skip.
- `increment_counter_value`: Loads a longword from an offset of a pointer, increments it by two, and stores the result back into a different offset of a pointer.
- `execute_sequence_loop`: Increments a value in a table, checks if it is greater than zero, and calls another routine if it is.
- `parse_compressed_data_stream_3`: Decodes a variable-length bitstream by extracting bytes and using specific bit patterns to determine the length and type of subsequent data sequences.
- `decode_compressed_data_stream`: Parses a custom bitstream by extracting bytes from a table and using a state machine to interpret variable-length sequences.
- `pop_data_stream_sequence`: Extracts a sequence of bytes from a pointer-based table into a local buffer and calculates a loop counter based on an input value.
- `decode_variable_length_data_stream`: Processes a data stream using a state machine that reads bytes and performs conditional transitions.
- `parse_data_stream_sequence`: Iterates through a data buffer and performs state transitions based on byte values.
- `process_data_stream_sequence`: Iterates through a data buffer using an index and a counter, performing a series of conditional state transitions based on byte values retrieved from a table.
- `process_entity_state_updates`: Iterates through a list of structures to update internal state values based on specific command bytes.
- `search_data_structure`: Iteratively searches through a data structure by calling nested functions and comparing values.
- `execute_conditional_function`: Checks a value from a global offset, and if it is non-zero, calls a function pointer retrieved from that value.
- `iterate_entity_state_flags`: Iterates through a list of entity structures, checking a specific bit in a flag field and calling another routine if the bit is set.
- `increment_and_store_pointer`: Increments a global counter and, if it wraps to zero, calls a function to update a pointer, potentially resetting that pointer to a new value from a table.
- `update_animation_table`: An eighteen instruction routine that updates a pointer in a table by adding a bit-shifted value from a register to an existing value.
- `update_animation_pointer_offset`: Decodes a data stream and stores the result in a structure field, then increments that field by eight if it is non-zero.
- `update_animation_pointer_indirect`: Decodes a data stream and stores the result in a structure field, then adds twelve to the value pointed to by that field if it is non-zero.
- `allocate_and_load_asset_array`: Allocates a memory block and populates an array of pointers by loading multiple data segments.
- `process_recursive_data`: Decodes a data stream and, if the result is non-zero, stores it in a local variable and recursively calls itself with several offsets from that value.
- `decode_conditional_stream`: Decodes a data stream and, depending on specific bits in a byte, either sets a value to zero or two, then potentially executes a loop that calls another routine multiple times.
- `execute_data_subroutine`: Pushes several values onto the stack, calls a subroutine, and then stores the returned value into a table via a pointer.
- `execute_data_subroutine_cleared`: Clears the stack, pushes a value from a memory location, calls a subroutine, and stores the result into a table via a pointer.
- `render_compressed_sprite_animation`: Decodes a compressed data stream to update animation pointers and performs bit-blt operations using hardware registers to draw graphics.
- `update_animation_frame_data`: Fetches a data block from a table based on a calculated index and copies it to a destination buffer.
- `update_animation_pointer`: Updates a pointer in a table by adding a bit-shifted value from a register to an existing value.
- `process_entity_state_updates_2`: Iterates through a list of structures to update internal state values based on specific command bytes.
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
- `call_function_with_swapped_arguments`: Calls a routine using two values retrieved from memory locations pointed to by global variables, potentially swapping the order of the arguments based on a byte value.
- `call_function_with_indirect_values`: Calls a function using two values retrieved from memory locations pointed to by global variables.
- `call_function_with_parameter`: Calls a function using a global pointer and a provided parameter.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `draw_parameterized_object`: Calls a routine to draw an object using provided parameters and handles conditional logic for setting attributes or bounds.
- `update_structure_values_2`: Performs a series of function calls using values from a structure and updates a local variable with a returned result.
- `update_structure_values_3`: Retrieves data from a structure to pass into several function calls and stores a result in a local variable.
- `branch_on_subtraction`: Subtracts a constant from a register and branches based on the result.
- `update_structure_values`: Retrieves a value from a structure, updates a local variable, and performs a sequence of conditional function calls to modify or copy data within a structure.
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
- `conditionally_initialize_state`: Checks a global value and, if zero, executes a sequence of functions and calls another routine with a global variable.
- `set_half_state_flag`: Checks a parameter, calls a routine with a constant if non-zero, sets a global byte to zero, executes an indirect call, and calls another routine with a constant.
- `initialize_memory_tables`: Sets certain memory locations to specific constant values using a loop.
- `sum_table_bytes`: Iteratively calls a function to fetch values from a table and accumulates certain bytes into a local variable.
- `update_tables_with_calculated_offsets`: Writes certain values to a table, calculates and stores new pointer-based offsets into a table, and calls a routine with a pointer from a table.
- `initialize_game_state`: Resets several memory-mapped or global arrays to zero and configures a table of parameters based on a specific or previously set value.
- `check_and_combine_bytes`: Checks if two consecutive routine calls return zero, then either stores a value from a function call into a global variable or sets a global variable to one.
- `execute_sequence_of_function_pointers`: Executes a series of indirect function calls using addresses or offsets derived from a global pointer, and writes a constant value to a memory location.
- `execute_conditional_callback`: Checks a global value and either executes a sequence of function pointers using data from a table or calls a single alternative function.
- `update_conditional_state`: Calls a routine with a constant, conditionally sets a global byte, and updates a global word or pointer based on the result of an indirect call or a comparison.
- `call_offset_routine`: Calls another routine using a value calculated from a pointer and an offset.
- `recurse_with_parameter`: Checks a parameter and calls itself recursively with that parameter and a global variable.
- `call_library_with_parameters`: Calls a library function using a pointer and certain values retrieved from a table.
- `update_conditional_table_entry`: Initializes a memory range to a constant value and then conditionally updates a table entry based on a comparison with a library function return value.
- `compare_and_branch_on_value`: Compares a word at a specific offset to a constant and branches based on whether the value is greater than or equal to that constant or is less than or equal to zero.
- `copy_word_to_memory`: Stores a word from a pointer into a memory location.
- `recursively_call_routine`: Checks if a parameter is non-zero and, if so, calls itself with that parameter.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_table_data`: Iterates through a loop to copy or update several values in a table based on conditional checks and pointer-based lookups.
- `test_and_branch_zero`: Tests a word at a specific memory offset and branches if it is zero.
- `test_memory_byte`: Loads a byte from a table using an offset, tests the value at a calculated memory location, and branches based on the result.
- `update_conditional_data`: Iterates through a table to modify bytes based on a comparison and a function return value.
- `process_data_loop`: Iterates through a structure to modify bytes based on conditional comparisons and function return values.
- `modify_byte_via_offset_2`: Calculates a value from a byte at an offset, negates it, and stores the result into a memory location using a word-based offset.
- `compare_and_branch`: Increments a word at a location relative to a pointer, compares it to a value, and branches based on the result.
- `initialize_array_bounds`: Clears a range of memory and then populates a secondary table with index-based offsets.
- `initialize_local_data`: Sets a word at a specific offset, compares a byte to a word, and conditionally stores a word into a local stack area.
- `search_and_initialize_array`: Iterates through a range of memory to find a null byte, then populates a secondary table with calculated values and conditionally updates a word based on a byte comparison.
- `clear_and_call_routine`: Clears several bytes and a word at offsets relative to a pointer, calls another routine, and then conditionally stores a word into a memory location.
- `clear_word_at_offset`: Stores a zero word into a specific memory location.
- `update_index_with_function_results`: Iterates through a loop, calling a function via a pointer and storing the result or its negation into a table.
- `load_and_call_pointers`: Calls two library functions, clears a table, loads several values from a library-referenced structure, and calls another routine for each loaded pointer.
- `execute_callback_list`: Iterates through a table of pointers and calls another routine for any non-zero pointer, then calls two library functions.
- `process_recursive_parameters`: Executes a function call using a value from a pointer and then calls itself recursively with modified parameters.
- `dispatch_indirect_calls`: Calls three different routines using a pointer from a table, passing various combinations of provided parameters.
- `dispatch_recursive_handler`: Calls a routine and then conditionally executes further routines or a loop of eight calls based on the value of a parameter.
- `toggle_object_state`: Calls a function via a pointer to modify an object's state using incremented coordinate or dimension values, then calls the same function again with the original values.
- `loop_parameterized_calls`: Calls a routine eight times in a loop using values retrieved from two separate tables and a structure, then performs two additional calls.
- `decrement_and_branch`: Decrements a register multiple times and branches to different locations depending on the value.
- `execute_parameterized_calls`: Calls a routine via a pointer using a value from a table, then calls another routine using four values retrieved from a structure.
- `call_wrapper_routine`: Calls a function via a pointer with three parameters, calls a second function via a pointer, and then calls another routine using the first parameter, the fourth parameter, and the result of the second function.
- `increment_offset_word_pairs`: Increments two word values located at offsets relative to a pointer and then returns.
- `increment_and_loop`: Increments two word values at offsets relative to a pointer and branches back to a previous instruction.
- `return_word_from_offset`: Returns a word from a memory location using an offset.
- `call_with_offset_parameters`: Calls a routine via a pointer using a single parameter, then calls another routine using four parameters where two are sums of input values.
- `call_via_offset_table`: Retrieves bytes from a table using an offset and calls a routine twice if a specific byte in the table is non-zero.
- `dispatch_table_entry_values`: Extracts multiple fields from a structured table entry based on an index and applies conditional offsets to those values.
- `update_table_data_2`: Extracts multiple values from a table using an index and offsets, then calls three different routines with these values and updated pointers.
- `call_subroutine_via_pointer`: Calls a routine through a pointer stored in a global table using a passed parameter.
- `execute_indirect_call`: Fetches a function pointer from a table using an offset from a base address and calls it with a parameter.
- `execute_indirect_function_call`: Fetches a function pointer from a memory location and calls it using a value passed as an argument.
- `clear_memory_buffer`: Uses nested loops to set a range of memory or a multi-dimensional array or buffer to zero.
- `update_table_entry_values`: Updates three consecutive short integer values in a table by adding a provided value to the existing contents.
- `add_to_table_value`: Calculates an address in a table using an index and an offset, then adds a value from a register to the memory at that address.
- `update_state_variables`: Modifies several values in a global structure and calls other routines using pointers.
- `update_status_flag`: Tests a long value at an offset and either clears a memory location or sets it to one based on the result.
- `negate_and_call_routine`: Negates a long value from a structure and calls another routine using values from that structure.
- `execute_multiple_callbacks`: Calls several function pointers using values from a structure and updates a global variable.
- `store_and_call_concatenated`: Calls a routine, stores a returned value in a global variable, and calls another routine using a concatenated 32-bit value.
- `dispatch_function_calls`: Sets a global variable and calls two different function pointers using values from a structure.
- `decrement_and_branch_5`: Decrements a long value twice and branches based on whether the value reaches zero.
- `pop_and_or_register`: Pops three longwords from a stack into a register and performs a bitwise OR operation on the result.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a2).
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a7).
- `clear_and_branch_on_test`: Clears a word at an offset, tests a word at another offset, and branches based on the result.
- `update_player_attribute_table`: Iterates through a structured data table to update specific attribute values based on a comparison with a secondary lookup table.
- `initialize_global_state`: Calls a routine with two arguments, stores the return value in a global variable, and then clears a small array of four or more words to zero.
- `call_routine_with_constant`: Calls a routine using a value stored in a global variable and a constant integer as an argument.
- `enable_dma_channel`: An 18-instruction routine that modifies the DMA controller register to enable a DMA channel.
- `update_dma_config`: Writes a value from a memory location to a hardware register and sets another hardware register to a constant value.
- `update_coprocessor_hardware`: Writes a longword and a word into hardware registers using an offset calculated from a provided parameter.
- `enable_dma_channels`: Sets a global variable to a value that enables DMA channels via the DMACON register.
- `write_hardware_registers`: Writes two word-sized values into a hardware register or a memory-mapped area using an offset derived from a parameter.
- `lookup_table_index`: Calculates an index by shifting a word from a stack offset and tests a value in a table at that index.
- `execute_table_switch`: Retrieves a pointer from a table and performs different operations, including updating global variables, based on a switch index.
- `update_table_entry_and_call_2`: Stores a word into a memory location calculated using a base address, an index from a table, and a multiplier.
- `update_table_entry_and_call`: Stores a word into a memory location calculated using a base address, an index from a table, and a multiplier.
- `execute_indirect_branch`: Performs a short branch to a memory address retrieved from a pointer in a data table.
- `dispatch_indirect_jump`: Compares a value in a register against a constant and performs an indirect jump to an address retrieved from a table using a modified offset.
- `branch_to_address`: Performs a short branch to a specific memory address.
- `clear_and_set_pointers`: Clears a range of memory, calls a library function, and stores return values from other routines into a global variable.
- `setup_stack_and_branch`: Sets up a stack frame, moves a word from a memory location, and performs a short branch.
- `update_table_from_lookup`: Iterates through a table to update a destination memory area using values fetched from a secondary lookup table.
- `call_function_conditionally`: Checks a global variable and calls a function pointer with a specific value if that variable is non-zero, then calls the same function pointer with a different constant.
- `decrement_bounded_counters`: Decrements a global counter and uses a loop to decrement several other values while updating limits based on specific conditions.
- `update_pointer_and_call`: Calls a function pointer, calls another function pointer with zero, and copies a value from a memory location to a global variable.
- `reset_function_and_copy`: Calls a function pointer with zero and copies a value from a memory location to a global variable.
- `reset_state_and_call_subroutines`: Clears a value in a global memory location and then executes two previously stored function pointers, passing a zero or no argument to the first.
- `execute_function_pointer`: Calls a function through a pointer stored at a specific offset from a base address.
- `configure_dma_and_clear_flag`: Clears a global flag and calls a routine to enable a specific number of DMA channels, followed by a routine that writes a value to a memory location.
- `test_and_branch`: Tests a long word at a memory location and branches if it is not zero.
- `call_routine_with_parameter`: Calls another routine twice using values fetched from a memory location.
- `format_numeric_digits`: Decomposes a value into individual decimal digits stored in a local array and executes a series of function calls based on the resulting digits and specific conditional flags.
- `process_conditional_list`: Iterates through a list of elements, marking specific entries with a flag before executing a function pointer for each entry that does not match a condition.
- `process_conditional_list_callback`: Iterates through a list of elements, marking specific entries with a flag before executing a function pointer for each entry that does not match that flag.
- `iterate_and_call_on_condition`: Iterates through a list and calls a function pointer with a value from a memory location if a certain condition in a lookup table is met.
- `negate_and_return`: Checks if a word at a memory offset is negative, and if so, negates it and returns.
- `move_and_branch`: Moves a word from a memory location into a register and performs a short branch.
- `initialize_and_branch`: Sets up a stack frame, clears a word in memory, and performs a short branch.
- `copy_byte_sequence`: Copies a sequence of bytes from one memory area to another using a loop and returns the final index.
- `execute_callback_loop`: Clears specific memory locations and enters a loop that calls a routine and updates a global variable based on the result.
- `clear_and_call_subroutine`: Clears a byte at an offset from a pointer, clears a word at an offset from another pointer, pushes a value from a table onto the stack, calls a subroutine, adjusts the stack, and performs conditional branching based on a word value.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `update_position_and_clear_byte`: Calculates a new coordinate using a pointer-based offset and calls a routine with the result, then potentially clears a byte in a buffer if a certain condition is met.
- `decompose_and_render_value`: Decomposes a numeric parameter into individual digits stored in a local array, then iterates through that array to call another routine using a certain value.
- `parse_and_load_file`: Copies a string up to a colon or null terminator into a global buffer and attempts to open the specified file.
- `process_data_loop_2`: Executes a loop that compares values in a table and conditionally calls routines or performs indirect function calls based on those values.
- `call_write_table`: Calls a routine that writes values from a table into a destination.
- `execute_conditional_subroutine`: Stores a word in a global variable, calls another routine, and then conditionally calls a third routine based on the result in register d0.
- `process_nested_loops`: Executes a series of nested loops that call other routines and update a global variable based on specific comparison results.
- `decrement_and_branch_3`: Subtracts values from a register and branches based on whether the result is zero.
- `branch_to_offset`: Performs a short relative branch to a different location in the code.
- `branch_to_previous_instruction`: Performs a short relative branch to the instruction immediately preceding this one.
- `handle_state_transition`: Checks a parameter to determine a return value, otherwise calls a routine and performs a sequence of operations including writing table values and calling a library function.
- `handle_disk_swap_and_state_transition`: Checks a flag to determine if a disk swap or state change is required, then updates a global state variable and executes a sequence of conditional routine calls or an infinite loop.
- `write_table_values`: Iterates up to five times to write values from a table into a destination via a routine call, then resets a global variable if certain conditions are met.
- `execute_indirect_function`: Loads a function pointer from a memory location and calls it using a constant argument.
- `get_and_return_value`: Calls a routine and returns a value stored in a global variable.
- `test_and_branch_on_offset`: Tests a long word at a specific offset from a pointer and branches if it is non-zero; otherwise, it clears a register and returns.
- `initialize_global_table`: Writes several word values to various offsets in a global structure and copies a value from a table into another location.
- `update_counters_and_limits`: Decrements a global counter and, if it reaches zero, modifies several other global values and performs arithmetic to update bounded limits.
- `conditional_execute_routines`: Checks several global conditions and, if satisfied, executes two different routines based on a returned value and a counter check.
- `update_game_state_parameters`: Updates a set of state variables and timer values in a global structure based on an input parameter.
- `update_status_values`: Stores two specific word values into a table and branches to another routine.
- `update_status_values_alt`: Stores two different word values into a table and branches to another routine.
- `copy_table_data_2`: Calculates an index and copies several values from a table into a global variable structure based on a condition.
- `copy_table_data`: If a value in a table is greater than two, it copies several values from a table into another location using an index.
- `update_table_value`: Fetches a word from a table using a calculated index and stores it in a global variable, then conditionally updates other table entries.
- `set_fixed_parameters`: Stores four specific word values into a table and branches to another routine.
- `decrement_and_branch_2`: Decrements a value in register d0 multiple times, branching to different locations if the value reaches specific constants.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `fill_buffer_with_count`: The routine iterates a specified number of times, writing the current loop counter value as a byte to a destination buffer and incrementing the pointer.
- `initialize_and_branch_2`: Sets up a stack frame, saves registers, and clears a word in a global variable before branching to another routine.
- `branch_with_data`: Moves a word from a table into a register and branches to another routine.
- `execute_looping_call`: Calls another routine and enters a loop that continues as long as the returned word is not equal to all ones.
- `update_data_and_loop`: Stores a byte into a memory location, increments the pointer, and decrements a counter to manage a loop.
- `compare_pointer_and_branch`: Compares a longword at a pointer with a value stored in a structure and calls another routine if the longword is greater than or equal to that value.
- `increment_pointer_and_mask`: Increments a pointer, loads a byte from the original address, and masks it before branching to another routine.
- `check_bitmask_status`: Loads a byte from a structure and checks if specific bits are set.
- `process_structure_data_3`: Clears a bit in a byte within a structure and conditionally iterates through a table to call another routine based on bitwise checks.
- `apply_bitmask`: Performs a bitwise OR operation on a byte in a table using a constant value and branches to another routine.
- `update_accumulated_value`: Adds a value to a longword stored in a table, increments a pointer, and performs a bitwise mask on a byte before branching to another routine.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `compare_and_decrement_pointer`: Compares a byte from a pointer to a value and, if they match, returns a decremented version of that pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a memory location.
- `execute_conditional_logic`: This routine calls a bitmask extraction routine and a library function, conditionally executing another routine based on a bit in the library return value.
- `check_bit_mask`: Extracts a bitmask from a word, compares it to a value, and stores a constant in a global variable if the comparison fails.
- `handle_io_error`: Checks the result of a routine and, if it indicates an error, calls another routine to store a result in a global variable.
- `move_register_and_branch`: Moves a longword from a register to another register and branches to another routine.
- `adjust_pointer_offset`: Compares a value at a pointer to a constant and either branches or increments that pointer by four.
- `update_table_entries`: Iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `manage_library_resources`: Interacts with library structures to check status, execute functions, and update a pointer in a table.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `clear_memory_and_call_function`: Clears a block of memory using a loop and then calls a function pointer retrieved from a global variable.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `parse_quoted_string_tokens`: Parses a source string into individual tokens, handling whitespace and quoted substrings.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `load_short_and_branch`: Loads a word into a register and performs a short branch to another location.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `call_library_function`: Calls a library function using a pointer from a stack-based structure and stores the result in a global variable.
- `process_window_data`: Retrieves data from a nested structure through multiple pointer dereferences and updates a global table with the results.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_operands`: Checks the signs of two registers and negates them if necessary before calling another routine.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_word_math`: Performs complex arithmetic involving word swapping, doubling, and conditional subtraction based on input values.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `execute_short_sequence`: A short routine consisting of fifteen instructions that calls two other routines to increment values and check structure flags.
- `increment_and_copy_byte`: Increments a value at a pointer, copies a byte from an offset of a register to a destination, and performs bitwise masking before branching.
- `check_structure_flag`: Saves registers, updates a global variable with a local address, and branches based on a bit test in a structure.
- `process_structure_data_2`: Performs complex conditional logic involving pointer arithmetic, bitwise flag manipulation, and recursive calls to manage data structures.
- `clear_structure_fields`: Compares a word at an offset from a pointer to a specific value and branches if they are not equal.
- `process_structure_data`: Modifies fields within a structure, updates a pointer, and performs bitwise operations on a byte based on conditional checks.
- `update_pointer_and_byte`: Updates a pointer and a value in a table, sets a bit in a byte, and writes a byte from a register to the memory location pointed to by the updated pointer.
- `update_parameter_structure`: Calls a routine and modifies various fields within a structure based on the returned value.
- `modify_memory_and_call`: This routine writes a word and a bit to a memory location, moves a long value to an offset, calls another routine, and then modifies a byte in memory based on a comparison.
- `store_pointer_to_list`: Calls a routine to obtain a pointer, then stores a value and a new pointer into a linked structure.
- `move_and_offset_pointer`: Moves a longword from an offset of a register to a pointer, updates a table entry, and performs an addition on a register before branching.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `validate_table_index`: Checks if an index is within bounds and if a value at a scaled table offset is non-zero, then recursively calls itself.
- `execute_scaled_call`: Multiplies a word by six, uses the result to push a value from a table onto the stack, and calls another routine.
- `clear_d0_and_branch`: Clears the D0 register and performs a short branch.
- `process_flag_and_library`: This routine performs a bitwise check on a word and calls a library function, which may trigger a conditional call to another routine.
- `test_and_set_flag`: Performs a bitwise AND on a word and, if the result is non-zero, stores a constant in a global variable.
- `process_io_result`: Evaluates a routine result and, if it is negative, calls another routine to update a global variable.
- `move_and_branch_2`: Moves the contents of a long register into a data register and then performs a short branch to another location.
- `call_library_function_2`: Calls a library routine and, if a specific bit is set in the return value, conditionally calls another routine.
- `branch_after_call`: Calls another routine and then performs a short branch.
- `execute_write_sequence`: Calls a function via a pointer to process a global variable and then executes two subsequent routine calls.
- `execute_indirect_functions`: Checks a global pointer and, if non-zero, executes the routine pointed to by that address, followed by another indirect function call.
- `traverse_and_clear_linked_list`: Validates an index against a table and, if valid, recursively calls another routine through a pointer found in the table before clearing the entry by setting it to zero.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `call_library_vector_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 6 of that base, returning its result to the caller.
- `call_library_vector_12`: Library stub: loads a library base from a global variable and calls the library function at offset minus 12 of that base, returning its result to the caller.
- `call_close_indirect`: Calls a routine through an indirect pointer located within a table.
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
- `execute_indirect_function_2`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_pointer`: Fetches a function pointer from a nested structure within a global data block and executes it.
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
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested structure within a table and executes it.
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

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][a3](Disk 2 of 2).adf:bootblock

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

- `setup_copper_and_clear_memory`: Opens a library through the exec base, copies a table of words into a block of memory and clears a 1920-byte region.
- `service_pending_interrupt_flags`: Tests an interrupt flag bit and, when it is set, checks a second interrupt request bit and calls up to three helper routines.
- `write_next_table_entry`: Reads the byte at the current table pointer, wrapping the pointer back to a base address when its value is negative.
- `build_frame_copper_list`: Computes per-frame display geometry from state fields and fills a copper-style list of word pairs in memory.
- `program_hardware_copy_block`: Waits until a hardware busy bit clears, then writes control words and address pointers into hardware registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_calls`: Calls two different routines through pointers stored in memory.
- `modify_byte_via_offset`: Decrements a value from a memory location, shifts it left by one bit, and stores the result back into a different memory location.
- `update_sprite_attribute_table`: Iterates through a range of memory to update or duplicate sprite or object attribute data, including updating or clearing associated word-sized or byte-sized metadata or secondary attribute buffers.
- `initialize_memory_blocks`: Sets several global variables or memory locations to specific values and clears a series of byte-sized memory locations using a loop.
- `clear_memory_blocks`: Clears or zeros out a series of word-sized memory locations using a loop.
- `update_memory_tables`: Copies word-sized values from a source table to a destination table, using an offset or a value from a global variable to determine the source address.
- `initialize_string_buffer`: Allocates a memory buffer and initializes it with a specific header, then updates several global state pointers and flags.
- `move_data_register`: Copies the contents of one data register to another.
- `process_callback_list`: Iterates through a list of function pointers and executes them, followed by a routine that passes a calculated size or pointer to another function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_game_state_transitions`: Resets a large block of global state variables and counters before executing a sequence of state-dependent function pointers.
- `update_sprite_attributes`: Iterates through a table of object descriptors to modify specific attribute bytes, such as color and flags, and calls a routine to update the display or position.
- `calculate_offset_from_byte`: Extracts a byte from a word and compares it against a series of constants to calculate a subtraction-based offset.
- `perform_no_operation`: This routine is empty and performs no actions.
- `perform_no_operation_2`: This routine is empty and contains only instructions that perform no operation.
- `check_value_range`: Compares a value in a register against a series of constants and returns a calculated offset if no match is found.
- `nop_padding_sequence`: This routine is empty and consists of two instructions that perform no operation.
- `clear_global_byte`: stores a zero byte into a global variable.
- `update_global_pointers`: executes a function pointer and then copies values from a table or structure into several global variables.
- `call_subroutines`: calls two subroutines via pointers, passing zero as an argument to the first one.
- `update_state_counters`: Increments or decrements several global short integers based on nested conditional checks.
- `set_conditional_values`: sets certain global bytes or words based on the value of a parameter and then executes a function pointer.
- `update_sprite_attributes_2`: Sets certain global words, then enters a loop that calls another routine and updates a global word with a modulo-reduced value.
- `call_with_table_value`: executes a function pointer using a value retrieved from a table.
- `reset_and_configure_game_state`: Initializes a table with a constant value and resets several state-related flags or counters.
- `fill_array_with_parameter`: executes a function pointer and then loops to assign a parameter value and a constant to a table.
- `update_structure_offset`: Copies a word from a structure to a memory location, then checks a value in that structure to determine whether to increment a field by a constant and store the result back, or simply copy a word to a different offset within the same structure.
- `update_entity_attribute_loop`: Iterates through a range of an array to update a memory-mapped attribute or pointer, and if a certain condition is met, updates a secondary table and calls two function pointers with a new coordinate or state.
- `transform_data_table`: iterates through a nested loop to scale values from one table and store them in another.
- `process_state_transition_index`: Evaluates an input index against a table of values and executes specific logic branches based on the result.
- `copy_table_entry`: Copies a word from a table into a destination location using an index derived from another table.
- `search_table_for_pattern`: iterates through a table to find a specific pattern or value, returning a calculated index or pointer.
- `execute_until_zero`: executes a function pointer repeatedly while certain global shorts are non-zero, then executes two more function pointers.
- `parse_string_delimiters`: Searches through a buffer for a specific character and replaces a delimiter character with a null terminator.
- `copy_player_sprite_attributes`: Copies sprite attribute data from a source table to a destination buffer using nested loops to duplicate specific byte sequences.
- `search_byte_table`: Iterates through a table to find a specific byte and returns a signed value based on its position or presence.
- `decrement_and_branch_4`: This routine compares a word against a constant, and if the value is not lower than three, subtracts four from it and branches to another location.
- `fetch_conditional_word`: Compares a value against a bitmask in a byte and returns a word from a table if the condition is met.
- `nop_padding_sequence_2`: This routine is empty and consists only of two no-operation instructions.
- `fetch_indexed_word`: Compares a register to a constant, shifts the register, and uses it to load a word from a table.
- `fetch_word_from_offset`: Loads a two-byte value from a memory location calculated by subtracting four from a value held in a register.
- `process_state_transition_logic`: Evaluates conditional flags and table values to manage state transitions, updating various global pointers and counters.
- `calculate_bit_offset_and_index`: Subtracts a constant from an input value to determine a byte index and a bit position within that byte.
- `update_buffer_offset`: Updates a global variable or pointer offset based on a calculated value and checks it against certain bounds.
- `update_index_and_clamp_value`: Updates a global index by scanning a table for duplicate adjacent values, then clamps a secondary global value within a fixed range.
- `update_global_values`: Copies several dwords or words from a source table or pointer-based memory to a set of global variables and clears two or certain global variables to zero or one.
- `call_with_indexed_table_lookup`: Calls a function pointer using a parameter and a value derived from a table lookup involving multiple offset calculations.
- `update_pointer_and_state`: Updates a pointer or a global word with a value from another memory location and conditionally sets certain global words based on a parameter.
- `clear_memory_and_assign`: Clears a range of bytes in a memory block, then calls another routine and stores its return value into a global variable.
- `execute_multiple_calls`: Executes a sequence of function calls, some of which pass a value retrieved from a global table or pointer-based memory.
- `set_constant_recursively`: Writes a constant value to a memory location and calls another routine recursively if a certain condition is met.
- `conditional_function_execution`: Calls a function, then checks if two global shorts are non-zero before calling another function that writes a constant to a memory location and passing a value from a global variable.
- `execute_set_constant`: Executes a function call, then calls another function to write a constant value to a global memory location, followed by a final function call.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_table_with_parameter`: Calls another routine with a constant, then performs a series of word-sized copies between a table and a global variable.
- `update_buffer_from_table`: Clears a local buffer, then iterates through a range, reading values from a table and incrementing a byte in a buffer based on certain comparison logic.
- `update_data_structures`: Performs several memory operations and conditional writes to a destination based on input parameters and a global value.
- `branch_to_offset_2`: Performs a relative branch to another location.
- `fetch_and_store_values`: Retrieves three values from a source using specific offsets and stores them into a single destination location.
- `perform_conditional_memory_operations`: Calls a function to check a condition and, if successful, performs several memory operations using different offsets and values via a function pointer.
- `copy_buffer_with_offset`: Copies dwords from a pointer-based table, calls another routine, and conditionally writes incremented bytes to a calculated memory offset.
- `accumulate_table_values`: Iterates through a nested loop to add values from one table into another table at calculated offsets.
- `update_table_with_max_value`: Iterates through a loop to compare values in a table and updates a global table with a new value if it is larger.
- `parse_and_copy_data`: Copies several values from a table to local variables, searches a buffer for delimiters, and performs conditional memory operations if a condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 93 into d0, then returns.
- `find_marker_and_call`: Iterates through a table until a specific byte value is found, then calls another routine multiple times using the discovered offset or an incremented offset.
- `populate_buffer_from_table`: If a certain condition is met, copies bytes from a table into a destination buffer using an offset, or otherwise checks a table and updates a local counter.
- `swap_array_elements`: Searches a table for a matching value and swaps two adjacent elements in that table if a certain condition is met.
- `map_buffer_indices`: Iterates through a buffer and, if a certain byte is non-zero, copies a byte from a source table to a destination table at a calculated offset.
- `empty_c_function_5`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `initialize_and_reorder_pixel_buffer`: Clears a secondary buffer and reorders pixel-related data into two separate destination tables based on whether a value is below or above a specific threshold.
- `compare_and_recurse_table`: Iterates through a table and calls another routine if certain adjacent or nearby values in the table match.
- `compare_table_elements`: Compares two word-sized values from a table and calls another routine based on whether the values are equal or follow a specific order.
- `calculate_index_offsets_and_reorder_table`: This routine clears a counter array, then iterates through a range of indices to compare values in a table and increment counters if certain conditions are met.
- `swap_table_values`: Reads two word-sized values from a table and writes them back to a different index in the same table, effectively swapping or overwriting values.
- `update_table_with_sum`: Calculates the sum of two short values from a table and passes this sum, along with a pointer and a constant, to a series of called subroutines.
- `process_pixel_value_differences`: Clears a memory buffer and iterates through a table of byte values, comparing adjacent bytes and updating or decrementing values in separate memory areas based on the comparison results and a call to another routine.
- `find_matching_differences`: Iterates through a table and compares two values, storing an index and a difference into a destination table if a match is found.
- `unknown_routine`: The routine contains eighteen instructions but no specific logic is provided.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine`: Performs a word-sized subroutine call to another routine.
- `call_subroutine_at_offset`: Performs a word-sized branch with link to a specific address.
