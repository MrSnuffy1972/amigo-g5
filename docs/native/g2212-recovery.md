# Game 2212 (tv sports basketball (1990)(cinemaware)[cr pna](disk 1 of 2)[b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02212). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (925428 top-level bytes in 45 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 26 of 26 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 650 of 650 named (0 author symbols), 650 of 650 described (151 by rule, 479 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3480 of 3482 (99.94%) |
| Recovered without load address (counted in proof 1, flagged) | 13269 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14487 of 14506 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 393252 |
| code | 47728 |
| data | 272972 |
| padding | 10260 |
| unused | 201216 |

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
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2)[b].adf:bb | 3 | 7843 | 407 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2)[b].adf:bootblock | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 59 | 57 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 220 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 453 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 453 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1073 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2)[b].adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 38 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 192 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 183 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 114 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 456 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 2 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 92 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- tv sports basketball (1990)(cinemaware)[cr pna](disk 1 of 2)[b]/TV Sports Basketball (1990)(Cinemaware)[cr PNA](Disk 1 of 2)[b].adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 13269 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

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

### Assign

- `initialize_data_structures_2`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with an incrementing sequence of values.
- `restore_stack_values`: Copies a sequence of words from a source address to a destination address if a pointer is non-null, then calls another routine.

### EndCLI

- `initialize_data_structures_3`: Calculates the maximum value in a linked list or table, then populates a memory-mapped array with a sequence of incrementing values.
- `copy_metadata_block`: copies a fixed-size block of data from a memory offset to the stack and then executes a previously stored function pointer.

### Dir

- `initialize_memory_tables_2`: Calculates the maximum value in a linked list of memory blocks and populates a table with incrementing or decrementing values.
- `copy_block_and_execute_2`: copies a fixed-size block of 16 or 17 words from a memory location or table to the stack before calling another routine.

### Echo

- `initialize_global_metadata_table`: Iterates through a linked list or table to find a maximum value, then populates a new table or memory region with an incrementing sequence of values.
- `copy_block_and_execute_3`: copies a fixed-size block of 16 or 17 words from a memory location or table to the stack before calling another routine.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Execute

- `initialize_global_metadata_and_stack`: Iterates through a linked list or table to find a maximum value, then populates a memory-mapped or global table with an incrementing sequence of integers.
- `copy_metadata_block_2`: copies a block of sixteen bytes from a memory location offset by a register or pointer to a stack or memory destination, then calls another routine.

### Copy

- `initialize_stack_and_metadata_table_2`: Calculates the maximum value in a linked list or table, then populates a memory region with an incrementing sequence of values and copies a block of data from a source pointer to a destination pointer.
- `copy_memory_block`: copies a fixed-size block of 16 bytes from a memory location offset by a register-held pointer to the stack.

### Makedir

- `initialize_stack_and_metadata_table_3`: Calculates the maximum value in a linked list or table, then populates a memory region with an incrementing sequence of integers.
- `copy_block_and_branch_2`: copies a fixed-size block of 16 or 17 words from a memory location or table to the stack before calling another routine.

### CD

- `initialize_stack_and_metadata_table`: Calculates the maximum size of a linked list or table, then populates a memory region with a sequence of incrementing values and copies a block of data from a source buffer to a destination buffer or stack-relative location.
- `copy_block_and_branch`: copies a fixed-size block of 16 or 17 words from a memory location to the stack before calling another routine.

### EndIf

- `return_zero`: The routine is empty and always returns a zero value.

### If

- `initialize_data_structures`: Calculates the maximum value in a linked list or table and populates a memory-mapped array with an incrementing sequence of values.
- `copy_block_and_execute`: copies a fixed-size block of sixteen bytes from a memory location offset by a register value into the stack, then calls another routine.

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
- `fetch_and_increment_pointer`: Checks if a pointer or a range limit has been reached, increments a pointer in a structure, and returns the byte previously stored at that pointer.
- `get_and_increment_pointer`: loads a value from a pointer stored in a global variable, increments that global variable, and returns the value previously stored at the pointer's location.
- `check_bitmask_in_structure`: Loads a pointer from an offset of 8 in a structure in a5, reads a byte from offset 12 of that pointer, and branches to another routine if bits 3 and 4 are not set; otherwise it returns.
- `process_bitmask_and_table`: Clears a bit in a byte, then conditionally iterates through a table to call another routine.
- `set_bit_in_byte`: sets the fifth bit of a byte located at an offset from a pointer-based address.
- `update_pointer_and_increment`: Updates a memory location with a new address calculated by adding a value to an existing entry, then increments that memory location.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `find_char_in_string`: Iterates through a string looking for a specific character and returns a pointer to that character if found, or returns null if the string or a null terminator is encountered.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `save_registers_and_tail_call`: Preserves registers d2, d3, and a4 on the stack beneath the return address before unconditionally jumping to another subroutine.
- `execute_indirect_call_2`: Performs an indirect jump or call to a location stored in a register or memory.
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
- `update_pointer_and_flags_4`: Checks a bit in a structure, and if certain conditions or pointer values meet a certain criteria, it updates a pointer, modifies a status byte, and writes a value to a memory location.
- `update_pointer_and_flags_3`: Updates a pointer and certain flags within a structure while potentially clearing or setting bits in a status byte.
- `update_pointer_and_flags_2`: Modifies a pointer and certain flags within a structure.
- `update_pointer_and_flags`: Modifies a pointer and a flag in a structure, and may update a pointer or a byte in a structure or call another routine based on a condition.
- `update_pointer_and_set_flag`: Updates a pointer, modifies a value at an offset, sets a bit in a byte, and increments a pointer while storing a byte.
- `update_table_and_flag_2`: Updates a structure field with a constant or a pointer, and conditionally modifies a byte in a structure using a bitwise or operation.
- `update_table_and_flag`: Writes a constant to a memory location, modifies a byte via a bitwise or, and copies a dword from a pointer-based offset before conditionally setting a bit in a byte.
- `insert_into_linked_list`: Sets a global pointer to a specific address and, if a routine returns a valid pointer, inserts a value and a pointer into a linked list structure.
- `set_pointer_to_table_entry`: Sets a pointer to a location in a table and stores a value from another memory location into that table entry.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `validate_table_index`: Checks if a provided index is within a certain range and if a value in a table is non-zero, then calls another routine and returns a boolean result.
- `check_table_condition`: Calculates an offset into a table and calls a routine using a pointer from that table, returning whether the result is non-zero.
- `return_zero_2`: An empty routine that returns zero.
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
- `execute_indirect_address`: Executes a routine located at an offset from a global pointer.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_indirect_from_table_offset`: Loads a function pointer from a table and executes it.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
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

- `modify_and_jump`: Sets bits 4, 5, 6, and 7 of d0 using bitwise OR operations and then jumps to a long address.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
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
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
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
- `initialize_string_buffer`: Allocates a memory buffer and initializes it with a specific string or header, then updates several global state pointers and flags.
- `copy_register_to_register`: Copies the longword from d0 to d5.
- `process_callback_list`: Iterates through a list of function pointers and executes them, followed by a routine that passes a calculated size or pointer to another function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_empty_routine_2`: the routine performs no operations and returns immediately.
- `update_global_pointers`: Executes a function pointer and then copies values from a table or structure into several global variables.
- `search_table_for_pattern`: Iterates through a table to find a specific pattern or value, returning a calculated index or pointer.
- `parse_string_delimiters`: Searches through a buffer for a specific character and replaces a delimiter character with a null terminator.
- `clear_memory_and_assign`: Clears a range of bytes in a memory block, then calls another routine and stores its return value into a global variable.
- `update_buffer_and_set_flag_2`: Performs a series of indirect calls to update or clear memory ranges and, if a certain condition involving a global variable is met, sets a flag in a global variable.
- `update_buffer_and_set_flag`: Calls a routine with several arguments, stores the result in a variable, and sets a flag if certain conditions involving a global variable and a pointer-based value are met.
- `update_buffer_pointers`: Updates several pointers or values in a structure using a function that returns a value, and sets a flag if a certain condition is met.
- `execute_conditional_memory_copy`: Calls a function to check a condition and, if successful, performs multiple memory copies or transfers using a specific size or range.
- `copy_buffer_with_offset`: Copies dwords from a pointer-based table, calls another routine, and conditionally writes incremented bytes to a calculated memory offset.
- `accumulate_table_values`: Iterates through a nested loop to add values from one table into another table at calculated offsets.
- `update_table_with_max_value`: Iterates through a loop to compare values in a table and updates a global table with a new value if it is larger.
- `copy_table_parameters`: Copies several values from a source table to a destination table and then calls a function that performs a conditional memory copy.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 93 into d0, then returns.
- `find_marker_and_call`: Iterates through a table until a specific byte value is found, then calls another routine multiple times using the discovered offset or an incremented offset.
- `populate_buffer_from_table`: If a certain condition is met, copies bytes from a table into a destination buffer using an offset, or otherwise checks a table and updates a local counter.
- `swap_array_elements`: Searches a table for a matching value and swaps two adjacent elements in that table if a certain condition is met.
- `map_buffer_indices`: Iterates through a buffer and, if a certain byte is non-zero, copies a byte from a source table to a destination table at a calculated offset.
- `return_empty_routine_3`: the routine performs no operations and immediately returns.
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
