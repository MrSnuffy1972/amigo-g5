# Game 2216 (tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02216). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1827140 top-level bytes in 59 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 31 of 31 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 696 of 696 named (0 author symbols), 696 of 696 described (171 by rule, 506 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4379 of 4383 (99.91%) |
| Recovered without load address (counted in proof 1, flagged) | 400361 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16867 of 16886 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 628006 |
| code | 55508 |
| data | 888245 |
| padding | 14741 |
| unused | 240640 |

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
| micro3 | 4 | 127 | 8 |
| paranoimia | 2 | 83 | 3 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:dawn | 1 | 155 | 3 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:bb | 3 | 7843 | 388 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:XC2.1 | 1 | 99 | 3 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:bootblock | 1 | 11 | 1 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 72 | 72 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 453 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2273 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1073 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 2 |
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
- tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:Fluff.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 2 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][h acu - dawn](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][h ACU - Dawn](Disk 1 of 2).adf:dawn: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 400361 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

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

### Assign

- `initialize_resource_tables`: Iterates through a linked list or table to find a maximum value, then populates a secondary table with an incrementing sequence of integers.
- `copy_resource_block_3`: copies a fixed-size block of sixteen bytes from a memory offset to the stack.

### EndCLI

- `initialize_engine_metadata_2`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with a sequence of incrementing values.
- `copy_resource_block_4`: copies a fixed-size block of 16 words from a memory offset to the stack and then executes a stored function pointer.

### Dir

- `initialize_engine_metadata_3`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with a sequence of incrementing or fixed values.
- `copy_resource_block_5`: copies a fixed-size block of 16 or 17 words from a memory location offset by a pointer to the stack.

### Echo

- `initialize_engine_state_2`: Iterates through a pointer-based table to find a maximum value, then populates a memory-mapped or global table with an incrementing sequence of integers.
- `copy_resource_block_6`: copies a fixed-size block of sixteen bytes from a memory location or table to the stack and then executes a previously stored function pointer.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Execute

- `initialize_execution_environment`: Calculates the maximum size of a linked list or table, populates a memory-mapped or global table with an incrementing sequence of values, and copies a fixed block of data from a source table to a destination buffer while setting a function pointer for a subsequent routine.
- `copy_stack_data_2`: Checks if a pointer is non-null and copies a fixed number of words from a source offset to a destination pointer.

### Copy

- `reconstruct_engine_state`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with a sequence of incrementing values.
- `copy_stack_data_3`: Copies a block of data from a source address to a destination pointer if the destination pointer is not null.

### Makedir

- `initialize_engine_state_3`: Iterates through a pointer-based table to find a maximum value, then populates a memory-mapped or global table with an incrementing sequence of integers.
- `copy_stack_data_4`: Performs a loop that copies a sequence of words from a memory offset to a stack-based pointer.

### CD

- `initialize_engine_state`: Iterates through a table to find a maximum value, then populates a memory-mapped or global table with an incrementing sequence of integers.
- `copy_resource_block`: copies a fixed-size block of sixteen bytes from a memory offset to the stack.

### EndIf

- `return_zero`: The routine is empty and simply returns a zero value to the caller.

### If

- `initialize_engine_metadata`: Traverses a linked list or table to find a maximum value and then populates a memory-mapped or global table with an incrementing sequence of or encoded values.
- `copy_resource_block_2`: copies a fixed-size block of sixteen bytes from a memory location offset by a base pointer into a destination stack or memory area, then executes a previously stored function pointer.

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
- `get_and_increment_pointer_2`: Checks if a pointer-range or counter is valid, increments the pointer, and returns the byte previously pointed to.
- `get_and_increment_pointer`: Returns a byte from a pointer and increments that pointer.
- `update_pointer_and_set_bits_2`: Checks a bitmask in a structure, potentially scans a memory range or recurses, and updates a pointer or sets a bit in a byte based on a return value.
- `update_pointer_and_set_bits`: Clears a bit in a byte, checks a flag to potentially execute a loop that scans a memory range, and updates a global pointer and its associated value based on a return value.
- `set_bit_in_byte`: Sets a specific bit in a byte located at an offset within a global variable.
- `update_pointer_and_calculate`: Updates a pointer in a table, modifies an adjacent value, and returns a masked, concatenated value.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `find_char_in_string`: Iterates through a string until a specific character or a null terminator is encountered, returning the pointer to that character.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `perform_no_operation`: This routine is empty and performs no executable instructions or operations.
- `execute_indirect_call_2`: Performs an indirect jump or call to a location stored in a register or memory.
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
- `process_pointer_and_flag`: Updates a global pointer and checks a flag before performing conditional pointer or flag updates.
- `process_buffer_and_update_state`: Recursively checks a buffer or range, updating a pointer and a flag or clearing a pointer and flag if certain conditions or end-of-buffer markers are met.
- `update_pointer_and_flag_2`: Modifies a pointer and a flag, and may copy a byte from a source to a destination pointed to by a table entry.
- `update_pointer_and_flag`: Updates a pointer and a flag, and may copy a byte from a source to a destination pointed to by a table entry.
- `update_pointer_and_set_flag`: Updates a pointer, modifies a value at an offset, sets a bit in a byte, and stores a byte from a table into the memory location pointed to by the incremented pointer.
- `modify_memory_and_call_routine_2`: Writes a constant to a memory location, modifies a byte via a bitwise OR, copies a value from a pointer, and calls another routine.
- `modify_memory_and_call_routine`: Writes a constant to a memory location, modifies a byte via a bitwise OR, copies a value from a pointer, and calls another routine while conditionally updating a byte.
- `insert_into_linked_list_2`: Uses a function call to get a pointer, then inserts a new node into a linked list by updating a global head pointer and a node's next pointer.
- `swap_and_store_pointers`: Loads a pointer from a fixed offset, stores a new pointer or value into that location, and returns an incremented pointer.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `validate_table_index_and_call`: Checks if a provided index is within a certain range and if a table entry is non-zero, then calls another routine and returns a boolean result.
- `check_table_condition`: Calls a routine using a pointer calculated from a table and an offset, returning a boolean result based on whether the return value is non-zero.
- `return_zero_2`: The routine returns a zero value.
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
- `execute_indirect_from_address`: Executes a routine located at a fixed offset from a global address.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `execute_indirect_function_4`: loads a function pointer from a nested table lookup and executes it.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
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

- `call_subsequent_functions`: Calls a routine that initializes memory blocks and then calls two other function pointers.
- `populate_memory_tables`: Performs a series of writes to various memory locations using a loop and an offset derived from a global variable.
- `update_sprite_attribute_table`: Iterates through a range of memory to update or duplicate sprite or object attribute data, including updating or clearing associated word-sized or byte-sized metadata or secondary attribute buffers.
- `initialize_memory_blocks`: Sets several global variables or memory locations to specific values and clears a series of byte-sized memory locations using a loop.
- `clear_memory_blocks`: Clears or zeros out a series of word-sized memory locations using a loop.
- `update_memory_tables`: Copies word-sized values from a source table to a destination table, using an offset or a value from a global variable to determine the source address.
- `initialize_string_buffer`: Allocates a memory buffer and initializes it with a specific string header, then updates several global state pointers and flags.
- `copy_d0_to_d5`: Copies the longword from register d0 to register d5.
- `process_callback_list`: Iterates through a list of function pointers and executes them, followed by a routine that passes a calculated size or pointer to another function.

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
- `fill_array_with_parameter`: executes a function pointer and then loops to assign a parameter value and a constant to a table.
- `iterate_entity_attributes`: Iterates through a range of memory, checks certain attribute values, and updates a pointer or a value in a table if certain conditions are met.
- `update_entity_attribute_loop`: Iterates through a range of an array to update a memory-mapped attribute or pointer, and if a certain condition is met, updates a secondary table and calls two function pointers with a new coordinate or state.
- `scale_and_store_values`: loops through a range and scales a value from a source table before storing it in a destination table.
- `update_game_state_machine`: Updates a state-based or event-driven state machine by checking a provided input against a table and branching to specialized logic or resetting state flags.
- `load_from_table`: Loads a word from a table using an offset derived from another global word and stores it into a destination.
- `search_table_for_pattern`: iterates through a table to find a specific pattern or value, returning a calculated index or pointer.
- `execute_until_zero`: executes a function pointer repeatedly while certain global shorts are non-zero, then executes two more function pointers.
- `parse_string_delimiters`: Searches through a buffer for a specific character and replaces a delimiter character with a null terminator.
- `copy_player_sprite_attributes`: Copies or copies-from a source buffer or table to a destination buffer using a nested loop structure to duplicate or modify sprite or object attribute data.
- `search_buffer_for_value`: Checks a buffer for a specific byte value and returns a value based on its position or a multiplier.
- `select_conditional_table_entry_2`: Calculates a value based on a global variable and a parameter, then uses a switch statement to return a value or a fallback based on specific bit checks.
- `select_conditional_table_entry`: checks specific bits in a byte and returns a word from a table or a fallback location based on those bits.
- `empty_routine_11`: the routine contains no instructions and performs no operations.
- `get_conditional_word`: Checks specific bits in a byte and returns a word from a table or a fallback location based on those bits.
- `read_offset_value`: Returns a word-sized value from a memory location calculated using a register and an offset.
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
- `update_buffer_from_table`: Clears a local buffer, then iterates through a range, reading values from a table and incrementing a byte in a buffer based on certain comparison logic.
- `update_buffer_with_conditional_copy`: Checks if a memory-to-register copy or a certain length is non-zero, then performs a series of conditional memory-to-memory or memory-to-register copies or clears a memory range.
- `execute_memory_operations_2`: Performs several memory-to-register or memory-to-memory copies using a pointer and a temporary variable, then updates a global variable base.
- `execute_memory_operations`: Performs several memory-to-register or memory-to-memory copies using a pointer and a temporary variable, then updates a global variable based on a comparison.
- `perform_memory_copy_operations`: Calls a function to copy a range of memory, and if successful, performs further copies or a copy that returns a value from a global variable.
- `copy_buffer_with_offset`: Copies dwords from a pointer-based table and calls another routine, then conditionally writes incremented bytes to a calculated memory offset.
- `accumulate_table_values`: Iterates through a nested loop to add values from one table into another table at calculated offsets.
- `update_table_with_max_value`: Iterates through a loop to compare values in a table and updates a global table with a new value if it is larger.
- `copy_and_parse_buffer`: Copies several values from a source table to a destination, then calls a routine to parse a buffer and potentially performs further memory copies.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 93 into d0, then returns.
- `find_marker_and_call`: Iterates through a table until a specific byte value is found, then calls another routine multiple times using the discovered offset or an unable to incremented offset.
- `populate_buffer_from_table`: If a certain condition is met, copies bytes from a table into a destination buffer using an offset, or otherwise checks a table and updates a local counter.
- `swap_array_elements`: Searches a table for a matching value and swaps two adjacent elements in that table if a certain condition is met.
- `map_buffer_indices`: Iterates through a buffer and, if a certain byte is non-zero, copies a byte from a source table to a destination table at a calculated offset.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
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
- `call_subroutine_at_offset_1f0`: Calls a subroutine located at a specific offset within the code segment.
