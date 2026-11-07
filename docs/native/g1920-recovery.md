# Game 1920 (indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01920). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2753344 top-level bytes in 132 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 20 of 20 units identical (bytes and relocations), 15 raw-word lines |
| Functions named and described | pass, 662 of 662 named (0 author symbols), 662 of 662 described (94 by rule, 555 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3661 of 3661 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 28835 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 15490 of 15498 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2247636 |
| code | 52566 |
| data | 338273 |
| padding | 29365 |
| unused | 85504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 117 | 2 |
| SetMap | 2 | 209 | 8 |
| More | 71 | 1986 | 97 |
| mount | 2 | 117 | 2 |
| relabel | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| loadwb | 2 | 209 | 8 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 1 of 3).adf:Indy | 7 | 233 | 7 |
| d | 1 | 13 | 1 |
| TRISTAR | 2 | 104 | 4 |
| icon.library | 10 | 1375 | 109 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 1 of 3).adf:BTsnap | 3 | 3318 | 135 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |
| Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 2 of 3).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 164 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 138 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7230 | 275 |
| 98.LFL (raw code file, load address unknown) | 1 | 0 | 0 |
| 23.LFL (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address (play-rs-m1500): 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 70 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2014 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar][a2](disk 1 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 1 of 3).adf:Rooms/98.LFL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar][a2](disk 2 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 2 of 3).adf:rooms/23.LFL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar][a2](disk 3 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- indiana jones and the last crusade (1989)(lucasfilm games)(de)[cr tristar][a2](disk 1 of 3)/Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 1 of 3).adf:Indy: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 28835 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `handle_disk_error_loop` (runtime segment an address (play-rs-m1500)): Checks a global variable and enters a loop that displays text strings and performs conditional logic based on a return value. Kind: disk/track check loop. Pass path: the branch where _DAT_00c1c996 != 0 (the break condition).

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies sixteen words from a source pointer to a destination pointer and then calls another routine.

### SetMap

- `parse_amiga_hunk_relocations`: Iterates through a memory structure to identify and process AmigaDOS hunk type codes.
- `stub_empty_return_routine`: This routine is empty and contains only a return instruction.
- `decrement_and_loop`: Decrements a value in a register until it reaches zero.
- `tiny_ori`: Small routine of 1 instruction that merges 44 into d0.
- `copy_memory_blocks`: Performs a complex memory copy operation involving conditional loops and bit-shifting logic to move data between memory locations.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte register once for each count unit, wrapping to a space character when it reaches zero, and returns.
- `modify_memory_and_halt`: Shifts a word right by one bit, stores a byte from one memory location to another, and then attempts to move a value into a register before encountering invalid data.

### More

- `initialize_filesystem_environment`: Determines the current directory and parses file paths or command-line arguments into a structured table.
- `cleanup_and_alert`: Checks several global flags to execute specific functions and then frees a linked list of memory blocks.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `execute_dos_operation`: Performs a series of library calls based on global flags and returns a constant value.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `split_arguments_and_open_streams`: Splits a string into up to 32 words by replacing delimiters with null terminators and storing pointers to each word in a table.
- `allocate_and_init_block`: Allocates a block sized to the input plus a constant, stores that size inside the block, passes the global block and the block address to another routine, and returns the block address plus a constant offset.
- `allocate_and_check_block`: Calls a routine to allocate a memory block and returns negative one if the result is not zero.
- `return_minus_one`: Returns minus one without other work.
- `free_linked_memory_blocks`: Walks a singly linked list whose head is held in a global, freeing each node with the size stored at its offset and following the next pointer until the list ends.
- `append_node_to_list`: Appends a node to a list whose head and tail pointers are held in the first structure by updating the next and head/tail pointers.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `implement_more_utility`: Implements a command-line style pager that reads a file and displays its contents line by line.
- `visit_global_table_indices`: Loops twenty times calling one routine to get an index and passing the global table address offset by that index to a second routine, then loops a variable number of times passing each counter value to the second routine, and finally forwards two parameters to a third routine.
- `count_string_bytes`: Scans forward from a pointer byte by byte until it reaches a zero byte, counting the bytes examined, and returns that count as the length of the string.
- `copy_string_to_destination`: Copies a null-terminated string from the second argument into the first, byte by byte including the terminator, and returns the destination.
- `copy_null_terminated_string`: Copies bytes from a source pointer to a destination pointer until a null byte is encountered.
- `clear_byte_and_return_word`: Clears a single byte at a location specified by an offset from a stack pointer and returns a word located at a different offset within the same stack frame.
- `copy_string_to_offset`: Calls a helper with the first argument to find an offset, then copies a NUL-terminated string from the second argument to that offset in memory, and returns the first argument.
- `copy_string_with_offset`: Copies a null-terminated string to a destination address calculated by adding a word value to a base pointer.
- `clear_byte_and_get_word`: Writes a zero byte to the memory location pointed to by an offset in the stack frame and returns a word from a different offset in the same frame.
- `execute_empty_stub`: Performs a sequence of instructions that results in no observable change to registers, memory, or hardware state before returning.
- `find_table_entry`: Iterates through a linked list of structures until it finds an entry with a non-zero field, then calls another routine with that entry.
- `forward_entry_arguments_conditional`: Checks a global value and, if it does not match a specific constant, passes three arguments to another routine.
- `forward_entry_arguments`: Passes two stored arguments and a table entry pointer to another routine before returning.
- `parse_stream_command`: Parses a command character from a string to execute specific seek or read operations based on the character and a flag.
- `process_stream_command_request`: Parses a command character from a stream record to execute specific seek or read operations.
- `update_stream_record_complex`: Updates a stream record by performing a seek-like operation, setting status flags based on a conditional check, and modifying a pointer structure.
- `update_stream_record`: Updates a stream record structure by setting specific bytes and pointers based on the results of a conditional check and a file seek operation.
- `update_stream_record_two`: Updates a structure's internal buffer and pointer fields based on a condition and a value of two.
- `update_stream_buffer`: Updates a buffer structure by evaluating a conditional check, calling another routine to update a pointer and status, and then resetting specific fields within the buffer.
- `update_stream_buffer_state_3`: Updates a buffer pointer and status flags by evaluating a conditional check and performing bitwise operations on a table entry.
- `update_stream_record_one_2`: Updates a record's buffer pointer and status flags based on a conditional check using a value of one.
- `update_stream_record_one`: Updates a structure's internal buffer and pointer fields based on a condition and a value of one.
- `process_stream_metadata`: Updates a buffer structure by calculating status flags based on a conditional check and synchronizing a pointer with a value returned from another routine.
- `process_stream_buffer_metadata`: Updates a buffer structure by calculating status flags based on a conditional check and synchronizing a pointer with a value returned from another routine.
- `update_stream_buffer_state_2`: Updates a record's buffer pointer and status flags based on a conditional check using a value of two.
- `update_stream_buffer_state`: Updates a record's buffer pointer and status flags based on a conditional check.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `reset_stream_record`: Updates a record's state and buffer fields based on a condition, or clears specific fields within a structure.
- `clear_stream_record_fields`: Clears the record's count word, sets its buffer fields and state byte, then returns the record.
- `clear_object_links`: Checks a flag bit to conditionally close a file entry, then conditionally calls another routine using two pointer fields before clearing those pointers and a flag byte, and finally passes a byte from the object to another routine.
- `lookup_open_table_entry`: Checks that an index lies within the table bounds and that the top bit of the entry's flag byte is set.
- `update_status_flags`: Searches a table for a non-null entry, calculates a new byte value based on bitwise operations on a parameter, updates a byte in the table with specific status bits, and calls another routine to update the entry's value.
- `update_file_buffer`: Updates a structure using bitwise operations and performs a write operation to a memory location derived from a global variable and an offset.
- `update_file_status_flags_2`: Updates bit flags in a structure and performs a file operation using a pointer derived from a global variable and an offset.
- `update_file_status_flags`: Updates bit flags in a structure based on a mode value and conditionally performs a file operation.
- `perform_file_operation_bit_20_2`: Updates bit flags in a structure, conditionally calls another routine, and performs a file operation with a calculated argument.
- `perform_file_operation_bit_20`: Sets a bit flag in a structure, performs a file operation with a calculated argument, and updates a global value.
- `update_file_status_and_recurse_2`: Sets specific bit flags in a structure and conditionally calls another routine using a pointer from that structure.
- `update_file_status_and_recurse`: Sets specific bit flags in a structure and conditionally calls another routine using a pointer from that structure.
- `fail_slot_with_invalid_mode`: Clears the slot's status byte, records an invalid-argument error code and returns minus one.
- `handle_file_operation`: Checks bit flags in a structure, performs a file operation with a specific argument, and updates a global value based on the result.
- `open_file_with_error_handling`: Attempts to open a file and stores the result, returning an error value if the operation fails.
- `return_stored_value`: Returns a longword stored at a fixed offset from a base register.
- `clear_status_flag_and_recurse`: Clears a status bit in the second argument and calls the routine again with the modified arguments, passing the first argument unchanged.
- `read_file_into_buffer`: Attempts to read a specified amount of data into a buffer using a handle retrieved from a table.
- `read_into_buffer_checked`: Calls a read library routine on the stored handle with the caller's buffer and length, and keeps the byte count.
- `write_to_handle_checked`: Obtains a file handle and, when its flag bit is set, seeks to the end before writing the buffer.
- `seek_file_with_offset_table`: Obtains a file handle and, for an index below three, seeks to an offset taken from a global table, records any I/O error, then seeks back to the start and returns the result.
- `close_file_entry`: Closes a file entry's handle unless a flag bit says not to, records an I/O error and minus one if the close fails, and clears the entry's flag byte.
- `delete_file_with_error_handling`: Attempts to delete a file and returns an error value if the operation fails.
- `return_zero_2`: This routine is empty and simply returns a zero value to the caller.
- `open_file_handle`: Opens the file named in the global text buffer with a fixed mode value and stores the resulting handle in a global variable.
- `read_buffered_byte`: Checks a global variable for a byte, and if empty, reads a byte from a file handle into a stack location.
- `read_byte_to_stack`: Reads a single byte from a file handle into a location on the stack and returns that byte.
- `write_byte_to_default_file`: Writes a byte argument to a file handle, opening the default file first if the handle is unset, and returns the byte.
- `forward_two_args_call`: Forwards its two arguments to one subroutine routine and returns nothing.
- `read_stream_byte_5`: Manages a data stream by checking status flags, updating a pointer or a constant value, and calling another routine to retrieve data.
- `read_stream_byte_4`: Manages a data stream by checking status flags, updating a pointer or a constant value, calling another routine to retrieve data, and then updating the stream state.
- `read_stream_byte_3`: Manages a data stream by checking status flags, updating a pointer or a constant value, calling another routine to retrieve data, and then updating a buffer pointer and a counter.
- `read_stream_data`: Updates status flags in a structure and retrieves a byte from a buffer, potentially triggering a refill if the buffer is exhausted.
- `read_stream_byte_2`: Retrieves a byte from a buffer, decrementing a counter in a structure and potentially performing a file read if the buffer is exhausted.
- `read_stream_byte`: Decrements a counter in a structure and returns either a byte from a buffer or calls another routine if the counter reaches zero.
- `forward_stream_to_helper`: Passes the stream pointer from the frame to a helper routine and returns.
- `write_byte_with_retry`: Writes a byte from its first argument into a buffer tracked by a state byte and a count field, updating the buffer position and counter.
- `reset_target_flags`: Returns zero immediately when the structure already has a target and no reset flag is set.
- `shift_global_variable_left`: Returns a global variable shifted left by three bits and performs no other operation.
- `call_with_value_twice`: Passes its single argument to another routine twice, as both parameters, and returns without further work.
- `find_node_by_distance`: Walks a linked list of nodes from a global head, moving by the requested distance and updating a global counter.
- `call_with_second_repeated`: Calls another routine with its first argument followed by its second argument repeated.
- `insert_or_update_linked_list`: Traverses a linked list to insert a new node at a position determined by a global counter, or updates an existing node by incrementing its value if a match is found.
- `check_signal_and_dispatch`: Reads the signal state for two bits, and when neither is set returns zero.
- `compute_signed_magnitude_division`: Divides two register inputs by a restoring division loop over 32 bit iterations on their magnitudes, giving the quotient the sign of the xor of the inputs.

### mount

- `update_linked_list_buffer`: Traverses a linked list to find the maximum value in a specific field and then expands a data buffer by writing a header and copying a block of data.
- `copy_data_block`: Copies sixteen words from a source address to a destination address using a loop.

### relabel

- `update_linked_list_buffer_2`: Traverses a linked list to find the maximum value in a specific field and then expands a memory buffer by writing a header and copying data blocks.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location to a destination address using a loop, then executes another routine.

### endcli

- `update_linked_list_buffer_3`: Traverses a linked list to find the maximum value in a specific field, then expands a data buffer by writing a header and copying a block of data.
- `copy_memory_block_and_call_2`: Copies sixteen bytes from a memory location to a destination address using a loop.

### loadwb

- `parse_amiga_hunk_loader`: Parses a memory structure containing AmigaDOS hunk type codes to perform relocation and data copying.
- `return_from_subroutine_2`: This routine is empty and performs only a return instruction.
- `decrement_register_to_zero`: Decrements the value in a register until it reaches zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 188 into d0.
- `copy_memory_blocks_2`: Performs a complex memory copy operation involving conditional loops and bit-shifting logic to move data between memory locations.
- `decrement_byte_cycle_counter_2`: Decrements a byte register once for each count unit, wrapping to a space character when it reaches zero, and returns.
- `decrement_byte_with_space_wrap`: Decrements a byte in a register repeatedly until it reaches zero, at which point it is replaced with a space character.
- `modify_memory_bits`: Performs bitwise operations on a memory location using values from registers and then stores a value to another memory location.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 1 of 3).adf:Indy

- `free_blocks_and_clear_field`: Runs a short loop that releases memory blocks through the system FreeMem entry, then zeroes a longword field in the caller's structure.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `process_memory_blocks`: Performs multiple memory copy operations and updates integer values at specific offsets within a destination buffer.
- `copy_memory_block_3`: Copies a block of data from a source address to a destination address using a decrementing counter.
- `memmove_block_copy`: Copies a block of memory from a source address to a destination address.
- `copy_memory_block_4`: Copies a block of memory from a source address to a destination address using a loop.
- `null_routine`: This routine is empty and contains no executable instructions.

### d

- `bitwise_or_byte`: Performs a bitwise OR operation between a byte and a value at a memory location pointed to by a register, storing the result back to that location.

### TRISTAR

- `process_library_initialization`: Retrieves a pointer from a global variable and uses it to perform a loop that increments and decrements a counter while calling a routine via a pointer.
- `iterate_string_and_output`: Iterates through a null-terminated string using a pointer and performs two library calls, one of which is likely for outputting the string.
- `copy_memory_block_2`: Copies a block of memory from one location to another using a loop.
- `store_registers_to_memory`: Stores four registers into a series of global memory locations and performs several writes to memory using offsets from a stack pointer.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_status`: This library code checks a value in a structure and either updates a linked list of pointers or sets a specific bit in a byte.
- `set_library_bit`: Library code that sets a specific bit in a byte at an offset from a base pointer and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_library_status`: This library code copies two 32-bit values from a table into global variables and calls another routine, potentially triggering a callback if the result is zero.
- `copy_two_pointers_from_table`: This library routine copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
- `perform_infinite_recursion`: This routine calls itself indefinitely without performing any other operations.
- `call_handler_pointer`: This library routine calls a routine via a pointer stored in a data structure and then returns.
- `trigger_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `trigger_icon_hierarchy_rebuild`: This library routine invokes a single routine to rebuild an icon's child pointers and internal state and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `recurse_with_incremented_parameter`: This library routine calls another routine with the same first argument and an increased second argument, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `dispatch_icon_handler`: This library routine calls another routine and then returns.
- `execute_handler_pointer`: This library routine fetches a function pointer from a data structure and executes it.
- `call_other_routine`: This library code calls another routine and then returns without performing any other operations.
- `call_search_pattern_routine`: This library routine calls another routine and then returns.
- `call_numeric_parser`: This library routine calls another routine to parse a numeric string and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `traverse_and_modify_linked_list`: Recursively traverses a linked structure, calling another routine to process nodes.
- `initialize_icon_data`: This library code performs a series of calls to retrieve data, copies a sequence of words from a stack to a structure, and updates several fields within that structure.
- `copy_structure_fields`: This library code copies a sequence of words from one memory area to another and then performs multiple individual word and byte writes to a target structure.
- `return_zero_stub_2`: This library routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `rebuild_icon_hierarchy`: Updates an icon's child pointers and internal state by iterating through a linked list of icon structures.
- `update_icon_hierarchy`: Iterates through a linked list of icon structures to update child pointers and state fields.
- `update_icon_state`: Library code that iterates through a linked list of structures to update specific fields and validate icon states by calling a function through a pointer.
- `validate_icon_state`: Library code that repeatedly calls a function through a pointer to update a value in a structure and checks a status flag.
- `call_and_return_zero`: This library code calls another routine and then returns zero.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `traverse_icon_structure_2`: Recursively traverses an icon node structure and its associated child tables, including linked lists of pointers.
- `traverse_icon_structure`: Recursively traverses an icon node structure and its associated child tables.
- `propagate_icon_changes`: Library code that traverses multiple levels of icon nodes and tables, invoking callback routines for each entry found.
- `process_icon_table`: This library code iterates through a null-terminated table of pointers, calling a routine for each entry and performing several conditional checks and updates to a structure.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list_nodes`: Library code that traverses a linked list structure by following pointers stored at an offset from the current node.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `execute_handler_callback`: Library code that retrieves a pointer from a structure and calls a function with specific parameters, potentially handling errors recursively.
- `call_subroutines`: This library routine calls two separate subroutines in sequence before returning.
- `return_zero_value`: This library routine is empty and simply returns zero to the caller.
- `decrement_counter_and_store_pair_2`: The library code decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable to store two 32-bit values.
- `decrement_counter_and_store_pair`: The library code decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `manage_icon_hierarchy`: This library code traverses structures and performs recursive calls to manage a hierarchy, potentially updating pointer chains and status values.
- `return_empty_stub`: This library routine is empty.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `incremental_recursive_call`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `search_list_for_delimiter`: Iterates through a list of pointers to find an entry containing a specific character delimiter.
- `find_string_delimiter`: This library routine iterates through a list of pointers to find a specific character delimiter within a memory area.
- `find_delimiter_in_list`: Library code that iterates through a list of pointers to find a specific character delimiter within a memory area.
- `search_memory_for_pattern_2`: This library code searches through a memory range to locate a specific byte pattern and returns a result based on the search outcome.
- `search_memory_pattern`: Library code that iterates through memory locations to find a specific pattern or length match.
- `validate_memory_range`: This library routine performs a loop involving arithmetic comparisons and memory checks to validate a specific range or condition.
- `process_iterative_logic`: Library code that performs a loop involving arithmetic comparisons and calls to another routine.
- `search_memory_for_pattern`: Iterates through memory locations to locate a specific byte pattern.
- `parse_numeric_string_3`: This library code parses a numeric string from a source, converts the ASCII characters to an integer, and copies the result into a destination buffer.
- `convert_ascii_to_integer`: Parses a numeric string by iterating through characters to calculate a decimal value.
- `parse_numeric_string_2`: Library code that converts an ASCII string into a numeric value and performs subsequent checks using a table of function pointers.
- `parse_numeric_string`: This library routine converts an ASCII string into a numeric value and performs subsequent checks using a table of function pointers.
- `convert_ascii_digit`: Library code that converts an ASCII character to its numeric value or returns negative one if the input is out of range.
- `return_negative_one_2`: The library code returns a value of negative one.
- `call_indirect_function`: The library code calls a function through a pointer retrieved from an offset relative to a global variable.
- `execute_library_handler`: As part of a library, this routine calls a function pointer retrieved from a location in memory.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `process_recursive_data`: This library code calls another routine and, if a non-zero value is returned, writes a constant to memory and calls itself recursively.
- `return_zero`: This library routine returns the constant zero.
- `compare_and_return_zero`: This library routine calls another routine and returns zero if the result does not match a provided parameter.
- `tiny_move_rts`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `validate_library_result`: This library code calls another routine and returns zero if the result does not match a provided parameter.
- `return_void`: This library routine is empty and performs no work.
- `perform_large_division`: Library code that performs division on a 32-bit value using a 16-bit divisor.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_string_pointer`: Library code that iterates through a string to find the position of a specific character and returns a pointer to it.
- `return_zero_3`: Library code that returns zero.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_c_string_2`: Library code that copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each step.
- `compare_memory_blocks`: Library code that compares two memory areas byte by byte up to a specified length and returns one if they match or zero otherwise.
- `return_one`: Library code that returns one.
- `iterate_and_call`: Library code that iterates through a list of items and calls another routine for each non-null element.
- `call_and_return`: Library code that calls another routine and then returns.
- `copy_and_pad_string`: The library code copies characters from a source to a destination up to a specified count or until a null terminator is reached, then pads the remaining destination bytes with null terminators.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `traverse_icon_nodes`: This library code repeatedly calls another routine in a loop, traversing a structure through a pointer until a specific condition is met or a null pointer is encountered.
- `iterate_and_validate_nodes`: This library code repeatedly calls another routine in a loop, traversing a structure through a pointer until a specific condition is met or a zero value is encountered.
- `return_zero_stub`: This library routine is an empty stub that immediately returns the constant zero.
- `update_icon_attribute`: Library code that calculates a new value using multiplication and updates a specific offset within a structure if certain conditions are met.
- `return_negative_one`: This library routine returns the constant negative one.
- `update_icon_list_2`: This library code iterates through a linked list of structures, updating specific fields and checking return values from other library routines.
- `traverse_icon_list`: Library code that iterates through a linked list of structures, updating specific fields and returning a pointer or error code.
- `update_icon_list`: Library code that iterates through a linked list of structures, updating specific fields and checking return values from other library routines.
- `return_all_ones`: This library routine returns a value consisting of all bits set to one.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 1 of 3).adf:BTsnap

- `initialize_program_state`: Clears a block of memory, retrieves a global pointer, and executes functions via pointers stored in a global structure.
- `filter_linked_list_nodes_2`: Iterates through a linked list and rearranges nodes into a new sequence based on specific byte, word, and bitmask comparisons.
- `filter_linked_list_nodes`: Iterates through a linked list and selects specific nodes based on byte and word comparisons.
- `traverse_linked_list`: Iterates through a linked list of structures, checking specific byte values and offsets to find a matching node and returning its pointer.
- `filter_linked_list`: Traverses a linked list to find and extract nodes that meet specific byte and word criteria, then updates a pointer in a global structure.
- `find_list_node`: Traverses a linked list to locate a specific node based on multiple conditional checks of its internal data fields.
- `filter_linked_list_2`: Iterates through a linked list and selects specific nodes based on byte and word comparisons.
- `manage_display_object_lifecycle`: This routine handles the allocation, creation, and destruction of display structures by interacting with system libraries and managing nested callback chains.
- `call_other_routine_2`: This routine calls another routine and then returns.
- `process_nested_callback_chain`: Sets a flag in a global structure, then recursively executes a series of functions based on values retrieved from memory structures.
- `compare_and_sync_coordinates`: Compares two short integer values stored in a structure against two target values provided via pointers.
- `copy_and_store_data`: Copies a block of memory from one location to another and stores several bytes from a parameter into a global structure.
- `execute_stored_callbacks`: Retrieves data from a global structure to call a routine with specific parameters, then calls another routine and clears a pointer.
- `decompress_and_blit_image_data`: Decompresses a bitstream into a destination buffer using a run-length or bit-masking scheme and then performs a bitplane-based blit operation to draw the resulting image data into a specific memory area.
- `copy_bytes_with_increment`: Copies a sequence of bytes from a source table to a destination, incrementing a global counter for each byte processed.
- `store_two_bytes`: Stores two bytes of data into a destination, where the first byte is the negation of a value and the second is a provided byte, while incrementing a global counter twice.
- `compress_byte_run`: Processes a buffer of bytes to identify and group consecutive identical characters into a compressed format.
- `find_max_list_element`: Iterates through a list of values retrieved from a buffer, comparing each element to find and store the maximum value.
- `process_nested_structures`: Performs a series of conditional checks and nested calls involving string copying and function execution through pointers retrieved from a structure.
- `parse_signed_integer`: Skips whitespace and an optional sign character to convert a sequence of ASCII digits into a signed integer.
- `manage_dos_chain_state`: Manages a state machine for executing a sequence of DOS operations, handling string comparisons, buffer copies, and conditional callbacks while updating a status flag in a global structure.
- `return_immediately`: An empty routine that simply returns.
- `execute_dos_chain_2`: Executes a sequence of pointers and handles results by interacting with the DOS library and managing memory buffers.
- `execute_dos_chain`: Iterates through a sequence of pointers, executing each one and handling the results until a null pointer or error is encountered.
- `match_string_pattern`: Compares two strings, supporting wildcard characters to determine if the first string matches the pattern defined by the second.
- `find_last_occurrence`: Finds the position of the last occurrence of a specific byte within a null-terminated string.
- `convert_to_uppercase`: Checks if an input byte represents a lowercase ASCII letter; if so, it subtracts a constant to convert it to uppercase, otherwise it returns the byte unchanged.
- `convert_to_lowercase`: Checks if an input byte represents an uppercase ASCII letter; if so, it adds a constant to convert it to lowercase, otherwise it returns the byte unchanged.
- `modify_structure_flags`: Clears specific bits in a structure and, depending on the result of another routine, either resets the structure to zero or returns a specific error value.
- `calculate_offset_difference`: Calculates a signed difference between values in a structure based on a bit flag in the third byte of the input pointer.
- `iterate_nested_structures`: Performs a nested loop through a structure's fields, calling another routine repeatedly until a specific condition is met or a return value of minus one is encountered.
- `search_and_process_list`: Searches a list for a valid entry and passes the result to another routine for further processing.
- `search_and_process_list_2`: Iterates through a list of six-byte entries, comparing them against a parameter, and updates a structure with a result and a flag.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `initialize_window_structure_2`: Initializes a structure by performing various library calls, setting specific bit flags, and populating fields with returned values.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_string_into_array`: Parses a null-terminated string by skipping whitespace and extracting individual words or quoted substrings.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `perform_io_operations`: A routine consisting of 40 instructions that calls conditional callbacks and performs seek and I/O error operations.
- `call_routine_with_same_argument`: Passes its argument to another routine and returns.
- `update_entry_table`: Iterates through a table of entries to find an empty slot, then attempts to populate it by calling various routines with specific parameters.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer until a null terminator is reached or a specified maximum length is exceeded.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `find_byte_in_string`: Searches for a specific byte within a null-terminated string and returns a pointer to its first occurrence.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `initialize_window_structure`: Opens a file, retrieves library-related data, and populates a structure with pointers and values derived from library function calls.
- `store_pointer_to_global`: Stores a value into a global memory location and then clears that same location.
- `format_and_store_data`: Stores a parameter into a global location and then calls routines to increment a counter and parse a format string.
- `increment_and_store_byte`: Increments a global counter and stores a byte into a location pointed to by a value retrieved from a global variable.
- `execute_recursive_calls`: A routine that performs recursive calls and returns a 64-bit value formed by concatenating register contents.
- `parse_format_string`: Parses a format string containing conversion specifiers to extract and format data from a list of arguments.
- `perform_signed_division`: Calculates the signed division of two integers and returns a 64-bit value combining the quotient and the original divisor.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original contents of a register with the value left in that register by the call.
- `call_recursive_routine`: Calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `update_structure_status_wrapper`: This routine acts as a wrapper that calls a routine to update a structure's status flags and pointers.
- `iterate_structure_list`: Iterates through a sequence of values using a loop and calls another routine for each element.
- `clear_and_traverse_structure`: Checks flags within a structure to recursively call other routines and then clears the contents of the structure.
- `update_structure_status`: Updates a structure's status flags and pointers, potentially resetting them to zero or performing a write operation based on specific bitwise conditions.
- `search_linked_list`: Iterates through a table of entries to find a record where a specific byte offset contains a null value, then clears the entire entry.
- `update_status_structure`: Updates various fields within a structure, including setting specific bit flags and storing pointers based on the result of a table check.
- `traverse_and_clear_list`: Iterates through a linked list, calling another routine for each node, and finally sets a global pointer to zero.
- `traverse_linked_list_2`: Iterates through a linked list of pointers and executes a function retrieved from a nested offset within each element.
- `insert_into_linked_list_2`: Inserts a new node into a linked list by updating pointers and executing a function pointer from a structure.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `insert_into_linked_list`: Searches for a specific pointer within a linked list and inserts it into the list, updating the preceding element's pointer.
- `check_table_entry`: Validates an index against a global range and checks if a value at a calculated offset in a table is non-zero, returning a boolean result.
- `delete_file_with_error_check`: Attempts to delete a file and, if unsuccessful, retrieves an error code and stores it in a global variable.
- `execute_write_callback`: A routine consisting of 41 instructions that calls conditional callbacks and performs write and I/O error operations.
- `execute_conditional_callback`: Calls a routine with specific parameters and, if a bit is set in the result, checks a global pointer before calling another routine.
- `write_text_and_execute_callback`: Writes a four-byte text fragment from a fixed address to an output handle and then executes a function via a global pointer.
- `call_indirect_and_routine`: Checks a global pointer and, if non-zero, executes the function at that address before calling another routine.
- `process_complex_structure`: Executes multiple conditional branches that involve iterating through a table, calling functions via pointers, and performing various data-driven operations based on a structure's contents.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `validate_and_clear_pointer`: Checks if an index is within the bounds of a table and if the resulting pointer is non-zero.
- `call_close_2`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir_2`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file_2`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_ex_next`: Wrapper that calls the dos library function ExNext and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err_2`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_dos_lock`: Calls a function via a pointer retrieved from a global table.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open_2`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_seek_2`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `execute_indirect_function`: Retrieves a function pointer from a nested table structure and executes it.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_2`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_function`: Calls a function at a specific offset within a library retrieved from a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_display_structure`: Allocates a new structure by calling library functions and populating its fields with provided parameters and a pointer to another structure.
- `free_display_object`: Validates a pointer and then clears specific fields within the referenced structure, setting bytes to 255 and words to a fixed address.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `create_display_entry`: It calls another routine passing the input parameter and a fixed constant value of 48.
- `call_input_pointer`: It calls another routine passing the input pointer as an argument.
- `assign_structure_data`: Checks if a pointer is null and, if not, populates a structure with a byte, a word, and a pointer.
- `clear_structure_fields_2`: Sets a byte at a specific offset to 255 and two longwords to a fixed value within a structure.
- `clear_structure_fields`: Sets a byte at a specific offset to 255 and two longwords to a fixed value.
- `execute_indirect_function_offset_c6`: Executes a function pointer retrieved from a specific offset within a structure pointed to by a global variable.
- `execute_indirect_function_2`: Fetches a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_function_offset_126`: Executes a function pointer retrieved from a different specific offset within a structure pointed to by a global variable.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_function_5`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `execute_indirect_function_3`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of the argument into the memory location pointed to by the argument, increments that value by four, clears the following word, and stores the original pointer into the subsequent word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested table structure and executes it.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `execute_indirect_function_4`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_indirect_function_2`: Calls a function pointer located at a fixed offset from a pointer stored in a table.
- `call_indirect_function_3`: Calls a function pointer located at a different fixed offset from a pointer stored in a table.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_indirect_function_4`: Calls a function pointer located at a third fixed offset from a pointer stored in a table.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Indiana Jones and the Last Crusade (1989)(LucasFilm Games)(De)[cr Tristar][a2](Disk 2 of 3).adf:bootblock

- `initialize_boot_environment`: Checks a library function return value, modifies a byte in a structure if non-zero, and calls another library function before returning a status.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_callback_and_check_status`: Executes two function pointers, updates several global variables, and performs conditional execution based on a bitmask check and the return value of a subsequent function call.
- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `process_runtime_state_loop`: Performs a complex loop involving bitwise shifts and arithmetic operations on a global variable to determine execution flow.
- `nop_sequence_padding`: This routine consists entirely of no-operation instructions and performs no functional tasks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `handle_disk_error_loop`: Checks a global variable and enters a loop that displays text strings and performs conditional logic based on a return value.
- `check_disk_error_status`: Evaluates a global variable and enters a loop that displays text strings based on the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_with_stored_argument`: Stores its argument in a shared word, then calls a library function with a fixed set of values and calls a helper when that call returns a nonzero result.
- `initialize_shared_tables_and_masks`: Resets shared flag and sentinel bytes, calls helpers, and clears several shared tables, setting the first hundred entries of one to one.
- `initialize_shared_state_defaults`: Sets default values for many shared tables and scalar globals, clears a block of per-slot records, and sets fixed limits such as 319 for a right edge.
- `write_slot_default_record`: Writes a fixed default record into the table slot given by the argument's low word, filling word and byte fields with default values.
- `initialize_resource_handle`: Validates an index, potentially closes an existing resource, and then initializes a new resource structure with a linked list of bytes.
- `setup_four_slot_layout_tables`: Stores a vertical range from its arguments and fixed rectangle bounds for four slots.
- `set_constant_byte_pair`: Writes two fixed constant byte values, 77 and 151, into shared globals.
- `perform_bit_manipulation_2`: Iteratively updates a global byte through eight cycles of bitwise shifts and arithmetic operations.
- `generate_bit_sequence_2`: Iteratively updates a series of bytes in memory using bitwise shifts and arithmetic operations.
- `generate_bit_sequence`: Iteratively updates a series of bytes in memory using bitwise shifts and arithmetic operations.
- `perform_bit_manipulation`: Performs a series of arithmetic and bitwise operations on a global variable within a loop.
- `multiply_and_concatenate_result`: Multiplies the low byte of its argument plus one by a byte from the high word, then returns the product as a 32-bit value with its low word shifted down by eight bits.
- `forward_argument_to_helper`: Passes its argument straight to a helper and returns.
- `retry_resource_allocation`: Attempts to perform a task through two successive loops that check return values, eventually calling a function with specific constants if the loops complete.
- `search_and_process_table_entry`: Iterates through a table to find an entry meeting specific criteria, then clears that entry and passes its value to another routine.
- `find_valid_table_index`: Searches a table for an index that satisfies several conditional checks, then extracts the value and passes it to another routine.
- `extract_zero_byte_entry`: Scans a table for a non-zero pointer where a corresponding byte, potentially masked, is zero, then clears the entry and passes the pointer to another routine.
- `select_table_element`: Loops through a table to find an element that satisfies specific bitwise and null checks, then clears the pointer and passes it to another routine.
- `increment_global_counter`: Increments a global variable in a loop and returns zero or one based on specific conditional checks.
- `process_pointer_list`: Iterates through a list of pointers and updates specific global tables based on a type value found in the data.
- `update_pointer_tables`: Traverses a structure to update various global tables based on a type byte found within the data.
- `trace_unresolved_external_entry`: This routine contains 46 instructions and is called by two different routines.
- `copy_longwords_until_count_ends`: Copies 4-byte words from a source pointer to a destination pointer, stepping both by one word each time, until the remaining length counter is no longer positive.
- `merge_free_memory_blocks`: Iterates through a linked list of memory blocks and merges adjacent free blocks into single larger entries.
- `traverse_and_modify_linked_list_2`: Iterates through a linked list starting from a global pointer, updating each node's pointer with the return value of another routine.
- `find_suitable_memory_block`: Searches through a linked list of memory blocks to find one that satisfies a minimum size requirement.
- `insert_and_merge_free_block`: Inserts a block into the address-sorted free list, then merges it with the following block when they touch and with the preceding block when they touch.
- `allocate_and_sort_memory_blocks`: Attempts to allocate memory blocks of varying sizes through repeated calls to another routine, storing the resulting pointers and sizes in a table.
- `release_pool_block_list`: Calls a helper with each entry of the free-block list, and calls it again with the pending block when that block is set.
- `return_status_after_refresh`: Calls a helper to refresh state, then returns the shared status word.
- `poll_until_status_nonzero`: Calls helpers in a loop, reading a status value on each pass, and stops when the status becomes nonzero.
- `increment_busy_counter`: Increments a shared busy counter, and when it changes from zero it sets a shared flag and calls a helper.
- `decrement_busy_counter`: Decrements a shared busy counter.
- `call_helper_with_table_entry`: Looks up a table entry from the high word of its argument and calls a helper with the shared base and that entry's offset plus four.
- `update_sprite_bounding_boxes_2`: Calculates a vertical extent based on a list of values, clears a memory range, and updates a table of values using a loop.
- `compare_recorded_event_with_text`: When the argument is nonzero it reads a stored character from a fixed table, clears it, calls helpers, and compares it with the last recorded event.
- `fetch_global_word_and_return`: Fetches a word from a global variable via another routine and returns the result.
- `loop_while_test_nonzero`: Calls a test helper, and while it returns nonzero calls a second helper and keeps its result.
- `wait_for_nonzero_event`: Calls a helper until it returns a nonzero value, then calls a second helper when that value equals 301 and returns the value.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_from_function`: The routine performs a single return instruction to exit the current execution flow.
- `call_helper_then_clear_flag`: Calls a helper, then clears a shared byte to zero.
- `take_and_clear_pending_word`: Calls a helper, then returns the shared pending word and resets it to zero.
- `get_global_word`: Returns the value of a global variable, calling another routine first if the variable is zero.
- `store_nonzero_field_word`: Stores the argument word into a field eighteen bytes into the record pointed to by the first argument, but only when that word is nonzero.
- `search_and_update_table`: Clears a series of words in a global table, then iterates through a memory range to find a matching entry and update a global variable accordingly.
- `loop_and_check_memory`: Iterates through a memory range checking specific byte values and updates a global variable based on the results.
- `check_global_status_and_update`: Checks a global variable against specific bit and arithmetic conditions, then either updates it with a byte from a memory location or sets it to a constant value before calling another routine.
- `search_bounding_box_table_3`: Searches a table for a matching entry, updates a global variable with a byte from that entry, and calls another routine.
- `reset_global_variable`: Sets a global variable to zero and then to one, and calls another routine if the variable is non-zero.
- `nop_padding_instruction`: This routine is empty and consists of a single instruction that performs no operation.
- `load_record_header_and_words`: Loads one record at the index given by its argument, with each record thirty-eight bytes long.
- `write_record_bytes_and_columns`: Writes up to two byte fields of the record selected by its first argument, skipping values equal to minus 32768.
- `nop_placeholder_routine`: The routine is empty and performs no operations before returning.
- `draw_palette_entry_via_library`: Rotates a four-entry index through a table, splits the selected 16-bit entry into three nibble fields, and calls a library function twice with those fields and a fixed value of 17, once for each of two bases.
- `read_two_words_from_block`: Copies two words from a fixed block of the shared state into the two output words given by its arguments.
- `call_another_routine_2`: The routine calls another routine and then returns.
- `return_and_clear_shared_word`: Returns the current value of a shared word and sets that word to zero.
- `fetch_and_reset_global_word`: Copies the value of a global word into a register and resets that global word to zero before returning.
- `update_status_fields`: Updates two consecutive word fields in a record based on whether the second argument matches specific byte values.
- `create_library_surface_pair`: Requests a library resource with fixed arguments, stores its handle, and calls a library function twice with fixed sizes and offsets.
- `refresh_library_surface_pair`: Calls a library function once for each of two bases, using the stored handle plus 72 and a size of 144.
- `reset_library_surface_offsets`: Calls a library function twice, once for each base, with sizes of 16 by 16 and zero offsets, using the stored handle plus 72.
- `position_library_surface_pair`: Calls a library function twice, once for each base, with sizes of 16 by 16 and offsets derived from a shared byte: its negated value plus one and its negated value.
- `call_other_routine_3`: Calls another routine based on the value of a global variable.
- `reset_surface_offset_tables`: Resets the offset tables for a specific library surface by calling a function twice with a fixed size and zeroed offsets, using a stored handle incremented by a constant value.
- `find_region_index_for_value`: Returns the index of the first of three ranges that contains the argument word, using each range's base and width.
- `set_shared_byte_to_constant`: Sets a shared byte to the constant 82.
- `publish_byte_and_set_flag`: Copies a shared byte into a shared word, calls a helper, and then sets a shared byte to one.
- `return_from_routine`: The routine performs a single return instruction to exit the current function.
- `nop_padding`: This routine is empty and consists of instructions that perform no operation.
- `search_bounding_box_table_2`: Iterates backward through a table to find an item where a specific coordinate range overlaps with a target range.
- `search_bounding_box_table`: Iterates backward through a table of entries to find an item where a specific coordinate range overlaps with a target range.
- `search_table_entry`: Decrements a counter and checks several conditions against values in a table and a shared word until a match is found or the counter reaches zero.
- `reset_active_entry_flags`: Sets a shared flag and calls a helper, sets both shared references to minus one, and clears a counter.
- `read_sound_size_value`: Reads and returns the contents of a global variable.
- `pass_shared_word_to_helper`: Passes a shared word to a helper and returns.
- `call_audio_cleanup_routine`: Calls a routine to release audio device resources and then returns.
- `return_void_2`: This routine performs no operations and immediately returns to its caller.
- `return_word_from_bytes`: The routine builds a 16-bit value from two consecutive bytes pointed to by its parameter and returns it.
- `initialize_resource_and_populate_table`: Allocates multiple memory buffers and initializes a sixteen-entry word table by iterating through a loop.
- `update_global_pointer_2`: The routine updates a global variable with its own address and performs several indirect calls using that pointer.
- `update_global_pointer`: Updates a global variable with its own address and performs several indirect calls using that pointer.
- `execute_conditional_logic`: Performs a series of nested checks using indirect function calls and updates a global variable based on bitwise results.
- `release_three_stored_handles`: Calls one routine on each of three stored handles in the reverse of the order in which the setup routine opened them.
- `poll_stored_handles_loop`: Loops at most 32 times, asking a routine to check two stored handles and stopping early when both report nothing.
- `resolve_object_properties`: Fetches data from an object structure and populates a destination buffer with its attributes.
- `check_buffer_allocation`: Attempts to allocate memory by performing multiple additions on a returned value and checks if the resulting address is non-zero.
- `release_double_bitplane_buffers`: Calls another routine twice with a size of 32000 to release two buffers.
- `install_callback_and_configure`: Sets two global variables to fixed values, stores the addresses of a shared record and of a routine into a global control block, then calls a library function with a constant argument.
- `call_library_with_global_pointer`: Passes a fixed argument of 5 and the address of a global block to a library function and returns.
- `build_room_file_name_list`: Walks a linked list of records, copying each name with a trailing colon into a table of up to nine string slots, then looks up a room file by name and sets a failure flag when neither lookup succeeds.
- `format_room_paths`: Iterates through a table to format strings using a path pattern, writes them into a buffer, and manages the table entries.
- `update_mode_transition_state`: Clears three counters and a helper value, then compares the new mode with the mode stored last time.
- `call_another_routine`: Calls another routine and then returns.
- `call_and_return_3`: Calls another routine based on the value of a global variable and then returns.
- `copy_plane_with_blitter`: Waits for the blitter, selects one of two plane buffers as source and the other as destination depending on a flag, and programs the blitter pointers, modulo, control and size registers for a copy 200 lines high and 20 words wide.
- `advance_byte_sequence_step`: Flips a buffer selector and runs a routine.
- `remap_sixteen_word_table`: Rebuilds a 16-entry word table in place by replacing each entry with the table value at the index it currently holds, then passes two stored buffers with table offsets to a library routine.
- `expand_changed_bitplane_rows`: For each row in a run, compares a 32-bit plane word with the value last stored and, when it has changed, writes its bits packed into bytes across four destination planes spaced 8000 bytes apart.
- `update_timer_logic`: Modifies a global variable and conditionally calls another routine based on the resulting value.
- `call_four_routines_in_sequence`: Calls four routines one after another with no arguments and returns.
- `call_and_return_2`: Calls another routine and then returns to its caller.
- `process_table_entries`: Iterates through a table to calculate addresses based on various offsets and calls another routine for each valid entry found.
- `compute_row_span_address`: Computes an address from a base table entry plus a row offset and a column offset, then passes that address with a width value to a routine.
- `load_dma_channel_word`: Looks up a record by index and copies its 16-bit field at a fixed offset into a shared word that goes to the DMA control register.
- `load_interrupt_control_word`: Looks up a record by index and copies two 16-bit fields from it into a shared word, the second copy overwriting the first, with the result feeding the interrupt enable and request registers.
- `load_interrupt_request_word`: Looks up a record by index and copies one 16-bit field into a shared word twice, with the result feeding the interrupt enable and request registers.
- `load_word_from_record`: Loads a 16-bit word from a record at a fixed offset into a global variable using an index.
- `copy_word_for_register_index`: Uses the index in the data register to look up a record, then copies its 16-bit field at a fixed offset into the shared word that feeds the interrupt request register.
- `or_record_request_words`: Looks up two records by index, ORs their 16-bit fields at the same offset together, stores the result in the shared word and returns it.
- `store_library_result_in_record`: Looks up a record by the index in the data register, calls a library routine through a stored vector, and writes the 32-bit result into a fixed field of that record.
- `call_library_without_result`: Calls a library routine through a stored vector and passes its result back without storing it.
- `nop_routine`: This routine is empty and performs no operations.
- `call_unreached_library_slot`: Calls a library routine through a stored vector, but this block decodes from a gap in the file and is not reached from the program flow.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `call_unreached_library_entry`: Calls a different library routine through a stored vector, in a gap of the file that is not reached from the program flow.
- `run_routine_four_times`: Calls another routine four times in a row and then returns.
- `execute_routine_four_times`: Calls another routine four times in succession before returning.
- `execute_routine_four_times_2`: Executes a specific routine four times in succession before returning.
- `run_routine_four_times_wrapped`: Calls one routine once, then calls it four more times in a loop, and then once more, with no arguments.
- `call_then_run_four_times`: Calls one routine once, then runs the same routine four times in a loop.
- `manage_audio_channel`: Updates an audio channel record by copying data from another record, adjusting length and volume fields, or resetting fields if the source is null.
- `reset_audio_channel_fields`: Resets an audio channel record by setting its length field to 124 and clearing a volume-like field, copies a control word from the record, runs a short busy loop, and decrements a shared timer entry when it is set.
- `advance_four_channel_timers`: Walks four channel slots and, for each active one, adds its per-tick increments to two position fields and counts down two timers, clearing the start values when a timer runs out.
- `clamp_channel_length_field`: Computes two channel fields from a record: the first is raised to at least 124, and the second is limited to a stored maximum and becomes zero when a shared flag is set.
- `run_channel_command_script`: Decrements a delay counter and, once it expires, executes a sequence of command pairs from a data table.
- `traverse_null_terminated_string`: Iterates through a pointer-based sequence until a null byte is encountered, then increments the pointer and stores it in a global variable.
- `store_register_to_global`: Stores the contents of a register into a global variable.
- `update_sprite_bounding_boxes`: Calculates and updates minimum and maximum width and height bounds for a sprite based on its current dimensions.
- `initialize_audio_system`: Populates a table with results from a routine, then performs a sequence of conditional initializations for a global structure.
- `release_audio_device_resources`: Undoes the audio setup by closing the audio device and freeing the request structure and the eight 64-byte blocks, each only when it is present, and clears the setup flag as it goes.
- `refresh_two_indexed_slots`: Assigns values from a table to two entries in a global table using indices from the argument, then calls another routine with those indices.
- `clear_two_indexed_slots`: Sets two entries in a global table to zero using indices from the argument and calls another routine for each index.
- `clear_indexed_slot`: Sets a single entry in a global table to zero using an index from the argument and calls another routine with that index.
- `update_entity_slot_data`: Updates two indexed entries in a global table by copying structured data from a source object.
- `run_pending_flag_handlers`: Calls one routine for each of two pending slot indices when that index is not the sentinel value of minus one.
- `release_matching_slot_entries`: Compares two shared slot entries with its argument.
- `save_slot_state_snapshot`: Clears a shared flag, copies three shared slot values into saved copies, calls a routine, and then stores a further shared counter into a saved copy.
- `restore_slot_state_snapshot`: Calls one routine with zero when the first saved slot index is valid, calls it again for the second saved index when that index is valid and different, and then restores a saved counter.
- `update_object_lifetimes_and_parse_stream`: Decrements life counters in a table of four entries and updates object properties based on a data stream.
- `invoke_handler_with_parameter`: Passes its parameter, a stack address, and a function reference to a handler routine and returns.
- `execute_indirect_call`: Fetches a function pointer from a global table and executes it, then returns the high word of the input parameter.
- `call_raw_put_char`: Library stub: loads its arguments and the exec library base, calls RawPutChar and returns its result to the caller.
- `print_null_terminated_string`: Iterates through a null-terminated string and calls a routine to output each character.
- `nop_padding_byte`: This routine is an empty instruction consisting of a single byte that performs no operation.
- `copy_stack_to_memory`: Copies a sequence of values from the stack into a memory block and then calls an indirect routine.
- `call_indirect_via_parameter`: Calls a routine through a pointer located at a fixed offset within a provided parameter.
- `copy_string_to_buffer_2`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length_2`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `get_next_byte_from_buffer`: Compares a value in a structure against another value in the same structure, increments the first value if it is smaller, and returns a byte from a pointer.
- `read_data_with_interactivity_check`: Performs a data read with error handling and checks a global table to determine if a device is interactive.
- `find_table_entries`: Searches a table for a matching string and searches another table for an empty entry to clear and return.
- `find_matching_string_in_table`: Iterates through a table of strings to find a match for a provided string and updates a parameter structure with the result.
- `find_byte_in_string_2`: Searches for a specific byte within a null-terminated string and returns a pointer to its location.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `manage_file_descriptor_table`: Iterates through a table of structures to find an empty entry or a specific file handle.
- `read_data_with_error_handling`: A routine consisting of 42 instructions that calls a read function and a function to handle input/output errors.
- `safe_memory_copy`: Copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `process_manx_object`: Allocates a memory block, performs several operations using a pointer to a structure, and conditionally calls functions via offsets within that structure.
- `return_from_subroutine_3`: This routine performs a return from a subroutine.
- `initialize_manx_object`: Allocates a block sized by a count times six bytes and, on success, writes the four-character marker MANX into its header and sets its initial state fields.
- `parse_string_tokens`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `parse_command_string`: Parses a source string into individual tokens, handling quoted substrings and skipping whitespace.
- `copy_string_with_limit_4`: Copies a string from a source to a destination, stopping if a null terminator is reached or a maximum length is exceeded.
- `copy_string_with_limit_3`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `initialize_window_structure_3`: Performs a series of indirect calls to initialize a structure, populating specific offsets with values retrieved from library-related pointers.
- `reset_global_pointer`: Assigns a value to a global pointer and then sets the first byte of the pointed-to location to zero.
- `format_into_buffer_with_result`: Formats its arguments through a formatter routine into the buffer given as its first argument, writes a terminating zero after the output, and returns the formatter's result.
- `store_byte_increment_pointer`: Stores a byte from its argument into the address held in a shared pointer, advances that pointer by one, and returns the stored byte.
- `invoke_handler_with_stack_pointer`: Passes a function reference, a stack address, and a parameter to a handler routine and returns.
- `convert_number_to_digit`: Performs a division and returns a 64-bit value formed by concatenating the original first input with the quotient left in a register, then recursively calls itself while referring to a sixteen-symbol digit table.
- `parse_formatted_string`: Parses a format string to extract and convert values into numeric types.
- `perform_signed_division_2`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `perform_signed_division_3`: Calculates the quotient and remainder of a 32-bit integer division and returns a 64-bit value by concatenating the result with the original remainder, adjusting the sign based on the inputs.
- `call_and_return_concatenated_2`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division_2`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `forward_call_to_handler`: Invokes a single handler routine and returns without passing any arguments.
- `manage_stream_and_data`: Updates a state structure by managing a pointer, a length, and status flags during data transfer, while also incrementing a pointer in a global variable and storing a byte.
- `return_immediately_3`: This routine performs no operations and returns immediately to its caller.
- `increment_and_store_byte_3`: Increments a pointer stored in a global variable and stores a byte from an offset within another structure.
- `increment_and_store_byte_2`: The routine increments a pointer stored in a global variable, reads a byte from an offset within another structure, and stores that byte at the original pointer location.
- `iterate_and_call_loop`: Walks a shared table in steps of 22 bytes, calling one routine with each entry's address until the end of the table is passed.
- `close_resource_handle`: Validates an index into a table, unlinks a node from a linked list, and calls a routine via a pointer from the table to close a handle.
- `manage_buffered_stream_state`: Updates a state structure by managing a pointer, a length, and status flags during data transfer.
- `find_empty_table_entry`: Iterates through a table of structures to find an entry where a specific byte is zero, then clears the entire entry and returns its pointer.
- `check_device_interactivity`: Checks a global table to determine if a device is interactive and updates various fields within a parameter structure based on the result.
- `iterate_linked_list_nodes_2`: Traverses a linked list by following pointers stored in each node and executes another routine using the node address and a data offset.
- `traverse_and_call_list`: Walks a linked list of nodes, calling another routine with each node and its data offset, and clears the list head when the walk ends.
- `insert_linked_list_node`: Inserts a new node into a linked list by updating pointers and storing a global variable and a parameter within the new node.
- `forward_parameter_field_2`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_from_linked_list`: Searches a linked list for a specific node, unlinks it from the list, and returns a status value.
- `check_interactivity_2`: Checks a global table to see if a specific entry is valid and calls a function pointer to determine if the associated device is interactive.
- `handle_indirect_call_error`: Executes a routine and, if it returns zero, updates a global variable and returns a failure code.
- `dispatch_io_operations`: Invokes a stored global function pointer, handles potential input/output errors, and writes data using a function pointer stored in a global variable.
- `invoke_pointer_and_forward_2`: Invokes a stored global function pointer when it is set, then forwards a shared four-byte block and its size to another routine.
- `invoke_pointer_and_forward`: Invokes a stored global function pointer when it is set, then forwards a shared four-byte block and its size to another routine, passes the returned value onward, and finally calls a third routine before returning.
- `call_function_pointer`: Invokes a stored global function pointer when it is set, then calls another routine before returning.
- `process_global_state_updates`: Runs one update pass by calling a helper for each entry of a table, invoking a stored callback when set, clearing or writing several shared pointers, and when a status bit is set calling a supervisor-level library routine before finishing.
- `nop_padding_sequence`: This routine is empty and consists of three instructions that perform no operation.
- `close_file_handle`: Validates an index into a table, checks a status bit, calls a routine via a pointer from the table, and clears the table entry.
- `call_dos_close`: Calls a function pointer stored at a specific offset from a global variable.
- `execute_indirect_function_6`: Loads a function pointer from a global variable with a specific offset and jumps to it.
- `execute_indirect_function_7`: Loads a function pointer from a global variable and jumps to that address.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `execute_indirect_function_8`: Loads a function pointer from a global variable and jumps to it.
- `call_io_error`: Calls a function pointer stored at an offset within a global variable to handle input/output errors.
- `check_interactivity`: Calls a function pointer stored at an offset within a global variable to check if the device is interactive.
- `call_lock_function`: Calls a subroutine using a function pointer located at a specific offset from a global variable.
- `call_lock_2`: Calls a function pointer stored at an offset within a global variable to lock a file.
- `call_open_3`: Calls a function pointer stored at an offset within a global variable to open a file.
- `call_output_3`: Calls a function pointer stored at an offset within a global variable to output data.
- `call_read_2`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek_3`: Calls a function pointer stored at an offset within a global variable to seek within a file.
- `execute_indirect_unlock`: Fetches a function pointer from a global variable using a specific offset and executes it.
- `call_unlock`: Calls a function pointer stored at an offset within a global variable to unlock a file.
- `call_write_3`: Calls a function pointer stored at an offset within a global variable to write data.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function_2`: Calls a subroutine using a function pointer located at a large negative offset from a global variable.
- `call_library_function_a`: Calls a function pointer located at a large negative offset from a global variable.
- `initialize_data_structure`: Populates a structure with provided parameters and pointers, including setting specific byte and word values at various offsets, and then calls another routine based on the input value.
- `free_request_block`: Frees a request block by recursively freeing a chained block if present, writing fill bytes and words to specific offsets, and invoking two other routines on the block.
- `execute_indirect_function_9`: Fetches a function pointer from a global variable using a calculated offset and executes it.
- `call_indirect_function_5`: Executes a function pointer stored at an offset within a global variable.
- `execute_indirect_subroutine`: Fetches a function pointer from a location relative to a global variable and performs an indirect call to that address.
- `call_indirect_function_6`: Performs an indirect call to a subroutine located at a specific offset from a global variable.
- `execute_indirect_function_10`: Fetches a function address from a global table using a calculated offset and jumps to it.
- `call_indirect_function_7`: Loads a function address from a global table and jumps to it.
- `execute_indirect_pointer`: Fetches a function pointer from a global variable using a specific offset and executes it.
- `call_indirect_function_8`: Calls a routine by dereferencing a pointer found at an offset in a global variable.
- `execute_indirect_function_11`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_indirect_function_9`: Uses a pointer from a global variable to execute another routine.
- `call_indirect_function_10`: Jumps to a subroutine address retrieved from a global variable offset.
- `execute_indirect_function_pointer_3`: Fetches a function address from a location relative to a global variable and executes it.
- `call_indirect_function_11`: Executes a function through a pointer stored in a global variable.
- `setup_pointer_chain_3`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_indirect_function_12`: Performs an indirect jump to a routine address calculated from a global variable.
- `execute_indirect_pointer_2`: Fetches a function pointer from an offset within a global variable and executes it.
- `call_indirect_function_13`: Calls a subroutine by accessing a pointer at a fixed offset from a global variable.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_indirect_function_14`: Dereferences a pointer within a global variable to call another routine.
- `execute_indirect_pointer_3`: Fetches a function pointer from a global variable using a constant offset and performs an indirect call to that address.
- `call_indirect_function_15`: Performs an indirect call to a function pointer stored at an offset within a global variable.
- `call_indirect_function_16`: Executes a subroutine by calling a pointer located at a specific offset in a global variable.
- `call_indirect_function_17`: Loads a function address from a global variable offset and jumps to it.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_indirect_function_18`: Calls a routine via a pointer retrieved from a global variable.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_indirect_function_19`: Uses a pointer stored in a global variable to perform an indirect function call.
- `call_indirect_function_20`: Triggers an indirect call using an address found within a global variable.
- `call_indirect_function_21`: Jumps to a subroutine address stored at a specific offset in a global variable.
- `call_indirect_function_22`: Performs an indirect jump to a routine address located in a global variable.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_indirect_function_23`: Calls a function through a pointer retrieved from a global variable offset.
- `call_indirect_function_24`: Executes a function by dereferencing a pointer stored in a global variable.
- `call_indirect_via_offset`: Calls a function pointer stored at a specific offset within a global variable.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_indirect_via_offset_2`: Performs an indirect call using a pointer located at a negative offset from a global variable.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_indirect_via_offset_3`: Executes a function by dereferencing a pointer found at a negative offset from a global variable.
- `call_indirect_via_offset_4`: Loads a function address from a global variable at a specific offset and jumps to it.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
