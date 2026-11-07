# Game 1449 (balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01449). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916908 top-level bytes in 78 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 52 of 52 units identical (bytes and relocations), 60 raw-word lines |
| Functions named and described | pass, 2268 of 2268 named (278 author symbols), 2268 of 2268 described (427 by rule, 1792 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3299 of 3299 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 193177 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 35204 of 35427 instructions identical in start and length, 4 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 48803 |
| code | 118338 |
| data | 698041 |
| padding | 18446 |
| unused | 33280 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf:stack | 2 | 117 | 2 |
| Format | 30 | 1919 | 78 |
| DiskCopy | 30 | 2723 | 99 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
|             | 2 | 117 | 2 |
| serial.device | 4 | 976 | 83 |
| parallel.device | 3 | 268 | 25 |
| clipboard.device | 10 | 1433 | 128 |
| printer.device | 19 | 3249 | 216 |
| narrator.device | 3 | 3782 | 268 |
| Qume_LetterPro_20 | 10 | 251 | 23 |
| CBM_MPS1000 | 11 | 352 | 28 |
| Diablo_630 | 10 | 229 | 23 |
| ImagewriterII | 12 | 733 | 44 |
| Diablo_C-150 | 12 | 358 | 20 |
| generic | 7 | 11 | 4 |
| Brother_HR-15XL | 10 | 232 | 23 |
| HP_LaserJet | 14 | 609 | 47 |
| Okidata_292 | 11 | 385 | 30 |
| Okimate_20 | 12 | 429 | 32 |
| Diablo_Advantage_D25 | 10 | 210 | 21 |
| Alphacom_Alphapro_101 | 10 | 330 | 24 |
| Epson_JX-80 | 11 | 396 | 31 |
| Epson | 11 | 350 | 28 |
| HP_LaserJet_PLUS | 14 | 609 | 47 |
| type | 1 | 146 | 6 |
| 12 | 1 | 115 | 2 |
| 15 | 1 | 2 | 1 |
| 8 | 1 | 115 | 2 |
| 12 | 1 | 115 | 2 |
| 20 | 1 | 115 | 2 |
| 11 | 1 | 2 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf:CLI | 9 | 362 | 18 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 15 | 1414 | 108 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2476 | 170 |
| mathieeedoubbas.library | 3 | 632 | 60 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 13 | 830 | 53 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf:bop | 3 | 5064 | 305 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1700 | 37 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 150 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf bytes 603136-603648 (code never loaded by a reference run) | 1 | 28 | 1 |
| SavedGame (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf: file devs/printers/Okidata_92 (header 5476 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1478 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 93 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 20 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf: bytes 603136-603648 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf:asave/SavedGame: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf: 1 logged loads cover 512 bytes of the image
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf: blocks 112-120 (bytes 57344-61952) outside the filesystem hold data, not code (opcode density 0.025, entropy 4.98)
- 1 never-loaded region(s), 192665 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf:stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 to d0 and then branches to another location.

### Format

- `parse_command_line_arguments`: Parses a string into individual arguments by scanning for spaces, tabs, or quoted text, handling escape sequences for newlines and escape characters, and stores the resulting pointers in a table.
- `handle_callback_pointers`: Checks two global variables for non-zero values and, if non-zero, calls function pointers at specific offsets from a global variable and a structure in a6.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `call_and_store_result`: Loads an address into a1, clears d0, calls a library function, stores the result in a global variable, and returns if the result is not zero.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_info`: Library stub: loads its arguments and the dos library base, calls Info and returns its result to the caller.
- `call_create_dir`: Library stub: loads its arguments and the dos library base, calls CreateDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_device_proc`: Library stub: loads its arguments and the dos library base, calls DeviceProc and returns its result to the caller.
- `call_date_stamp`: Library stub: loads its arguments and the dos library base, calls DateStamp and returns its result to the caller.
- `call_delay`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `call_wait_for_char`: Library stub: loads its arguments and the dos library base, calls WaitForChar and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `format_and_initialize_disk`: Calculates disk geometry, allocates a memory buffer, and writes a byte pattern to the disk to perform a format operation.
- `execute_callback_chain`: Checks five global variables for non-zero values and, if non-zero, calls the corresponding function pointer with specific offsets or no arguments.
- `traverse_and_invoke_table`: Iterates through a table of pointers, checks for null or empty entries, and calls a routine to compare strings and another routine to invoke a table entry.
- `report_disk_error`: Formats a disk by calculating cylinder and sector counts, allocating a memory buffer to write a pattern containing a signature and sector identifiers to the media, and uses a formatted string to report device failures or disk errors.
- `measure_string_length`: Returns the number of characters before the terminating zero byte in the given string.
- `copy_string`: Copies a null-terminated string from a source pointer to a destination pointer.
- `convert_to_uppercase`: Returns the input byte as is if it is outside the lowercase alphabet range, otherwise it subtracts 32 from it.
- `compare_strings_case_insensitive`: Compares two strings byte by byte by converting each byte to uppercase via another routine, returning 1 if they match and 0 otherwise.
- `truncate_at_colon`: Iterates through a string pointed to by a register until it finds a colon or a null terminator, then replaces that character with a null terminator.
- `manage_task_structure`: Allocates a structure, populates it with various values including a pointer from a parameter, and performs several library calls including a task-related operation.
- `save_and_reset_global_value`: Fetches a value from a structure via a function call, stores it in a global variable, and then writes a constant to the original structure location.
- `restore_global_value`: Fetches a value from a structure via a function call and restores a previously saved value from a global variable into that structure location.
- `initialize_stack_data`: Populates a stack-based structure with several constant values and then calls another routine with a pointer to a global variable.
- `execute_library_calls_3`: Calls several functions from a library base and performs a write operation using a DOS library base.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_remove`: Library stub: loads its arguments and the exec library base, calls Remove and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `call_alloc_signal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_add_port`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `call_rem_port`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_device`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `call_close_device`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_send_i_o`: Library stub: loads its arguments and the exec library base, calls SendIO and returns its result to the caller.
- `call_wait_i_o`: Library stub: loads its arguments and the exec library base, calls WaitIO and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a value from a stack slot or a negative error code.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_global_library_function`: Calls one library function through a library base read from a global variable, passing no arguments, and returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `rotate_32bit_value_3`: Performs a bitwise rotation and division-like operation on two 32-bit values using a loop that shifts bits and handles carries.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_at_pointer`: Stores a byte from register d0 into the memory location pointed to by a3 and increments a3.
- `format_disk_and_verify`: Calculates disk geometry and allocates a memory buffer to write a pattern containing a signature and sector identifiers to the media.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `execute_complex_logic`: Performs a sequence of assignments to various memory locations and global variables, then calls a routine with three identical arguments and another routine with a single argument.
- `create_std_io`: The routine calls another routine to create an extended I/O structure for the supplied pointer, passing a constant size argument.
- `delete_std_io`: The routine receives a pointer argument and forwards it unchanged to another routine that deletes an extended I/O. It performs no other work and then returns.
- `allocate_message_entry`: The routine attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: The routine removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `create_ext_io`: The routine allocates memory for an I/O request structure when the first argument is non-zero, initializes specific fields with constants and arguments, and returns the pointer to the block or zero.
- `delete_ext_io`: The routine clears specific fields in the I/O request structure and frees the memory block using the size stored within the structure, performing no operation if the pointer is zero.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### DiskCopy

- `parse_command_string_2`: Iterates through a string to extract tokens, handling quoted text and converting specific character sequences into newline or escape characters, then stores these tokens into a buffer.
- `execute_conditional_callbacks`: Stores a parameter on the stack, then if a global variable is non-zero, calls a routine at an offset from a structure, and if another global variable is non-zero, calls two routines at offsets from a different structure.
- `invoke_two_library_function_pointers`: Loads a pointer from an offset of 92 in a6 into a0 and calls the function at offset -416 in a6, then repeats this process to call the function at offset -404 in a6.
- `execute_conditional_callbacks_2`: Loads an address from a relative offset, clears d0, calls a routine at an offset from a6, stores the result in a global variable, and returns if the result is zero.
- `call_read_2`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write_2`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input_2`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output_2`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_un_lock_2`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine_2`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_info_2`: Library stub: loads its arguments and the dos library base, calls Info and returns its result to the caller.
- `call_io_err_2`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_device_proc_2`: Library stub: loads its arguments and the dos library base, calls DeviceProc and returns its result to the caller.
- `call_date_stamp_2`: Library stub: loads its arguments and the dos library base, calls DateStamp and returns its result to the caller.
- `call_delay_2`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `call_wait_for_char_2`: Library stub: loads its arguments and the dos library base, calls WaitForChar and returns its result to the caller.
- `call_parent_dir_2`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `manage_memory_list`: Calculates a value using a modular inverse routine, then enters a loop that allocates memory and populates a list with values from another routine, while managing a count of non-zero entries and freeing memory if necessary.
- `loop_until_non_zero`: Enters a loop that calls another routine and checks the return value.
- `update_structure_fields_2`: Calls another routine with four parameters.
- `update_structure_fields`: Stores a 16-bit value at offset 28, a 32-bit value at offset 36, and a 32-bit value at offset 40 of a structure, calls another routine, and then adds the result of that call to a value at offset 44 of the structure.
- `synchronize_disk_contents`: Performs a disk-to-disk copy operation by iterating through file structures on a source disk and replicating them to a destination disk.
- `synchronize_disk_contents_2`: Performs a disk-to-disk copy operation by iterating through file structures on a source disk and replicating them to a destination disk.
- `execute_cleanup_sequence`: Checks a global variable.
- `cleanup_system_resources`: Checks several global pointers and executes various cleanup routines, including deleting port structures, deleting extended I/O, calling library functions to free memory or close libraries, and managing error message tasks.
- `extract_message_data`: Copies three values from a structure at the address in a global variable into other global variables, calls another routine with that structure, and then calls another routine with a global variable if a specific value is not 512.
- `execute_library_call_2`: Loads a value from the stack into d0, pushes a value from a global variable and d0 onto the stack, calls a routine at an offset from a global variable, and if the result is not zero, it branches.
- `perform_disk_copy_loop`: Iterates through a table of data blocks to perform disk I/O operations, including reading from a source and writing to a destination.
- `allocate_multiple_libraries`: Checks a value at an offset of 20 from a global variable; if it is less than 33, it calls a routine with a fixed address and then calls a routine with 0.
- `initialize_disk_copy_params`: Populates a stack-based structure with several constants and then calls two library functions using a global base.
- `handle_disk_errors`: Depending on the input parameters, it sets a pointer to a global variable and calls another routine with various arguments, including specific values for different cases, and finally calls another routine twice with specific values before returning.
- `report_disk_errors`: Sets a pointer to a global variable and calls another routine with a global value, a parameter, and a stack address, then repeats this with a different constant and calls another routine with a single constant before returning.
- `process_io_and_status`: Calls a routine and checks the result; if the result is zero, it calls another routine with a pointer from a structure and a value from a table.
- `check_signal_status`: Calls another routine with two parameters, checks if the 12th bit of the result is set, and if so, calls another routine with a global variable; otherwise, it returns the bit status.
- `process_message_queue`: If a global variable is non-zero, it calls another routine in a loop to process messages and then calls another routine to extract data from a structure pointed to by a global variable.
- `set_parameter_zero`: Stores zero in a 32-bit word at an offset of 36 from a pointer and nine in a 16-bit word at an offset of 28 from that same pointer before calling another routine.
- `manage_error_message_task`: Allocates memory, populates the resulting structure with various values and pointers, calls a routine to handle the structure, and then frees the memory.
- `set_parameter_and_call`: Stores the value 4 at offset 28 of a structure, calls another routine, stores 5 at offset 28, and calls another routine twice more.
- `execute_multiple_calls`: Pushes two global longwords onto the stack and calls a routine twice for each value, then cleans up the stack and returns.
- `process_disk_data`: Calls another routine with a constant and a value, then if the result is non-zero, it performs several calls using a pointer from a global variable, updates a global variable with a pointer, modifies a byte at an offset of the pointer, calculates a sum by looping through 128 words in a table, subtracts that sum from a value at offset 5 of the pointer, and calls another routine with a constant and the pointer.
- `perform_io_operation`: Calls another routine with four parameters, then calls another routine and returns whether its result is zero.
- `compare_and_io`: Performs an I/O operation, calls another routine to update a structure, and returns whether a parameter matches a value stored in a global variable.
- `allocate_message_entry_2`: Attempts to find a task and allocate a message buffer, then populates the buffer with a value, a byte, and a pointer before calling another routine and potentially dispatching the buffer.
- `delete_port_2`: Removes a port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal, and releases the memory block.
- `create_ext_io_2`: Allocates memory for an I/O request structure when the first argument is non-zero, initializes specific fields with constants and arguments, and returns the pointer to the block or zero.
- `delete_ext_io_2`: Clears specific fields in the I/O request structure and frees the memory block using the size stored within the structure, performing no operation if the pointer is zero.
- `call_alert`: Library stub: loads its arguments and the exec library base, calls Alert and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem_2`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_remove_2`: Library stub: loads its arguments and the exec library base, calls Remove and returns its result to the caller.
- `call_find_task_2`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_set_signal_2`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `call_wait`: Library stub: loads its arguments and the exec library base, calls Wait and returns its result to the caller.
- `call_alloc_signal_2`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal_2`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_add_port_2`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `call_rem_port_2`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `call_put_msg_2`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_get_msg`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_reply_msg`: Library stub: loads its arguments and the exec library base, calls ReplyMsg and returns its result to the caller.
- `call_wait_port_2`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_close_library_2`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_device_2`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `call_close_device_2`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `call_do_i_o_2`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_send_i_o_2`: Library stub: loads its arguments and the exec library base, calls SendIO and returns its result to the caller.
- `call_open_library_2`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `calculate_string_length`: Increments a counter while scanning through a null-terminated string starting at the address provided in a stack slot and returns the count.
- `copy_memory_block_2`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `execute_indirect_calls`: Fetches a function pointer from a global structure and executes it multiple times with varying parameters.
- `invoke_indirect_callbacks`: Calls another routine with a pointer to a global variable and a parameter, then calls another routine with four parameters including a local stack array.
- `convert_char_to_uppercase`: Returns the character with lowercase ASCII letters converted to uppercase by subtracting 32.
- `compare_strings_case_insensitive_2`: Compares two null-terminated strings by converting characters to uppercase and returns 1 if they match, otherwise it returns 0.
- `terminate_colon_string`: Searches for the end of a null-terminated string and, if the last non-null character is a colon, replaces it with a null terminator.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer_2`: Stores a byte from register d0 at the address pointed to by a3 and increments a3.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper_2`: Saves d2 on the stack, moves d1 to d2 and d0 to d1, calls a helper routine, restores d2, and returns.
- `call_helper_return_register_value_2`: Saves d2 on the stack, moves d1 to d2 and d0 to d1, calls a helper routine, moves the resulting value from d1 to d0, restores d2, and returns.
- `negate_result_on_flag_3`: Calls a helper routine and negates the value in d1 if the address register a0 is nonzero.
- `call_and_negate_on_flag_3`: Calls a helper routine, then returns the value in d1 or its negation if the address register a0 is nonzero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment_pointer`: Stores a byte from register d0 into the memory location pointed to by register a3 and then increments the pointer in a3 by one before returning.
- `call_formatted_print`: Pushes a pointer and a value onto the stack and calls a routine to print a formatted message, then returns the result in a register.
- `pass_params_to_subroutine_2`: Saves d2 on the stack, loads a parameter from the stack, pushes a global value, pushes a pointer to an eight-byte stack slot, calls another routine, and restores d2.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_global_library_function_2`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `execute_with_status_check_2`: Calls a routine via a global pointer and returns either a byte value from a stack slot or a negative error code.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Saves d2 and d3, loads two parameters from the stack, pushes a pointer to a stack location, calls another routine, and restores the registers.

### Port-Handler

- `parse_amigados_hunks`: Reads a structure containing hunk type codes and offsets to identify executable or relocation segments.

### Disk-Validator

- `format_and_populate_display_buffer`: Converts a numeric value into a character string and stores it in a buffer, then enters a loop to clear a range of memory and populates a structure with bit-shifted values derived from a global variable.

### Ram-Handler

- `execute_jump_table_entry`: Sets d0 to 12, saves registers, calls a routine using a pointer from a6 at an offset of -656, restores registers, and performs an indirect jump using a pointer from a6.

###            

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a specific field, then writes a header and fills a buffer with sequential words.
- `move_and_branch_2`: Moves the contents of d1 into d0 and then performs a short branch to an offset of 260.

### serial.device

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reinit_serial_buffer_fields`: Calls a routine via a pointer in a6 if a1 is non-zero, then updates several fields in a structure at a6 with a value from d2, including setting half-size and quarter-size values.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `set_byte_and_branch`: Stores the byte 253 at offset 31 of a structure in a1 and then performs a short branch to an offset of 1206.
- `toggle_interrupt_enable`: Writes 801 to the INTENA register, calls another routine, writes 8801 to the INTENA register, and returns.
- `test_status_byte`: Clears bit 0 of a byte at offset 31 of a1 and branches to different locations based on whether the word at offset 28 of a1 equals 2, 3, or 10.
- `compare_and_clear_pointer`: Compares a longword at offset 74 of a structure in a6 with a1; if they are not equal, it returns, otherwise it clears bit 1 of offset 472 of a6 and clears the longword at offset 74 of a6.
- `unlink_list_node`: Tests bit 6 of a byte at offset 1e of a structure in a1; if set, it unlinks a node by updating pointers in a list and then decrements a 16-bit counter at offset 46 of a structure in a6.
- `check_and_trigger_hardware_interrupt`: Compares a pointer in a structure to a global variable and checks if a 16-bit value at offset 28 of that structure equals 10.
- `unlink_second_queue_request`: If a bit is set in a byte at offset 30 of a structure in a1, it unlinks that structure from a list, decrements a counter in a6, and calls a routine via a pointer in a6 if a bit is not set.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `set_status_bit_4`: Sets bit 6 of a byte at offset 1d8 of a structure in a6 if a 16-bit value at offset 20 of a structure in a6 is non-zero, otherwise it returns.
- `invoke_library_vectors_then_unlink`: Calls several routines via pointers in a6 and a structure in a6, copies a longword from a structure in a6, and unlinks a block from a list by updating pointers.
- `insert_request_into_list`: Sets a byte at offset 30 of a structure to 0 and bit 6 to 1, then inserts the structure into a linked list by updating pointers and increments a 16-bit counter at offset 70 of a structure in a6 before calling another routine.
- `process_serial_data_buffer_2`: Iterates through a linked list of data buffers, updating a pointer to the next buffer and decrementing a counter in a structure.
- `process_serial_buffer_queue`: Iterates through a linked list of data buffers, decrementing a counter and updating a pointer to the next buffer in the list.
- `test_bit_and_branch_2`: Tests bit 0 of a byte at offset 445 of a6 and branches to different locations depending on the result.
- `update_status_byte_3`: Calls another routine, and if the result is zero, writes the value 9 to offset 31 of a structure in a1 before branching to another location.
- `process_serial_data_buffer`: Reads a byte from a data structure, updates a counter and a status byte, and enters a loop that manages a linked list of buffers.
- `restore_registers_and_return_7`: Returns immediately after restoring d2 and a6 from the stack.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `clear_bit_and_branch_2`: Clears bit 2 of a byte at offset 470 of a6, and if the bit was already clear, branches to another routine; otherwise it writes 15 to offset 31 of a1 and branches to another routine.
- `clear_bit_and_branch`: Clears bit 7 of a byte at offset 470 of a6 and branches to a different location if the bit was already clear, otherwise it writes 9 to offset 31 of a1 and branches to another location.
- `modify_status_and_counter`: Writes the value 12 to offset 31 of a structure in a1, tests bit 0 of offset 470 of a6, and if it is zero, writes the value 6 to offset 31 of a1, clears the longword at offset 62 of a6, and performs a bitwise AND on offset 470 of a6 with 248.
- `process_serial_data_and_signal_handler`: Checks status bits in a structure to manage serial data flow, updates a buffer by storing a byte and incrementing pointers, and calls a handler via a pointer in the structure.
- `toggle_interrupt_enable_2`: Compares a byte at offset 34 of the structure in a6 with a register, then checks bit 1 of a byte at offset 472 of the structure in a6; if bit 1 is set, it clears that bit and writes 32769 to the interrupt enable register, otherwise it returns.
- `test_status_bits_2`: Tests bit 5 of a byte at offset 472 of a structure in a6, then tests bit 1 of a byte at offset 445 of a structure in a6, and if bit 1 is not set, calls another routine, then tests bit 8 of d0 and branches accordingly.
- `test_bit_and_branch_3`: Tests bit 8 of d0, and if it is set, branches to another routine; otherwise it branches to another routine.
- `test_status_bit`: Tests bit 0 of a byte at offset 445 of a structure in a6 and branches to another routine.
- `set_bit_and_copy_pointer`: Sets bit 7 of a byte at offset 470 of a6 and, if a longword at offset 62 of a6 is zero, copies a longword from offset 42 of a6 to offset 62 of a6.
- `process_buffer_data_2`: Compares a word at offset 54 of a6 with a word at offset 58 of a6, and if the latter is smaller, performs several increments and pointer updates before calling a routine via a pointer at an offset from a longword in a6.
- `record_pointer_then_signal`: Checks if a longword at offset 62 of a structure in a6 is zero; if it is, it copies a longword from offset 42 to offset 62, calls another routine, and then calls a library vector using a pointer from offset 98 of the structure in a6.
- `clear_hardware_register`: Moves a byte from offset 34 of a6 to offset 473 of a6, clears bit 3 of offset 472 of a6, tests bit 2 of offset 69 of a6, and if it is zero, clears bit 6 of the hardware register at address bf d000.
- `update_hardware_registers`: Moves a byte from offset 35 of a structure in a6 to offset 473 of the same structure, sets bit 3 of offset 472 of a6, tests bit 2 of offset 69 of a6 and branches, sets bit 6 of a long value at a hardware register, tests bit 7 of offset 69 of a6 and branches, otherwise writes 32769 to a hardware register and returns.
- `clear_byte_at_offset`: Clears a byte at offset 473 of a structure pointed to by a6 and returns.
- `update_status_and_call_handler`: Clears bit 6 of a byte at offset 470 of a structure in a6, sets it, clears a longword at offset 418, copies a longword from offset 106 to offset 422, subtracts 10 from that longword, sets a word at offset 414 to 9, calls another routine, and if a byte at offset 31 of a different structure is non-zero, sets a byte at offset 31 of that structure to 11.
- `modify_status_and_interrupts`: Saves d2, a3, and a6 to the stack, loads a1 into a6, clears bit 6 of a byte at offset 470 of a6, and if the bit was already clear, branches to another routine; otherwise it loads a word from the custom status register into d0, masks it, and if the result is non-zero, branches to another routine; otherwise it sets bit 2 of the byte at offset 470 of a6 and branches to another routine.
- `load_address_and_branch_6`: Loads the address from offset 386 of a structure in a6 into a0, calls another routine, and then branches to another routine.
- `set_status_bit_6`: Sets bit 0 of a byte at offset 470 of a6 and branches to another routine.
- `manage_linked_list_nodes`: Sets offset 40 of a structure in a1 to zero, and if offset 41 of a1 is non-zero, it clears bit 1 of offset 30 of a1 and either inserts a structure into a linked list at offset 184 of a6 or updates a pointer and increments a counter at offset 72 of a6, finally calling a routine via a pointer if a specific bitmask is not met.
- `handle_interrupt_request`: Tests bit 2 of offset 69 of a6 and branches, otherwise calls another routine, tests a byte at offset 31 of a structure in a1 and branches, sets bit 4 of offset 30 of a1, clears a 32-bit value at offset 94 of a6, copies a pointer from offset 40 of a1 to offset 86 of a6, copies a pointer from a1 to offset 78 of a6, and writes 32769 to two hardware registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_request_state_on_flags`: Checks if two specific bits in a global variable are set, and if so, writes a byte with a value of thirteen to an offset within a structure pointed to by a register.
- `manage_serial_request_queue`: Manages a linked list of serial request structures by popping the head node, updating its status flags, and decrementing a pending request counter.
- `update_status_and_decrement`: Writes 200 to offset 476 of a structure in a6, loads a pointer from offset 86 of a6 into a0, clears d0, loads a byte from the pointer into d0, loads a longword from offset 36 of a1 into d1, and if d1 is positive, tests bit 4 of offset 472 of a6 and, if not set, decrements a longword at offset 94 of a6.
- `update_event_counters_and_dispatch`: Updates a 16-bit value in a structure using a bitwise OR with a bit shifted by a value from offset 67, increments two 32-bit counters, and calls another routine via a function pointer at an offset from a pointer in the structure.
- `process_event_and_update_counters`: Calls another routine, updates a global variable using a bitwise OR with a value derived from a byte at offset 67 of a structure in a6, increments two longwords at offsets 86 and 94 of the structure in a6, and if a byte at offset 79 of another structure has bit 6 set, performs a loop to compare bytes in a buffer before potentially calling another routine.
- `process_list_removal`: If the shortword at offset 72 of a6 is non-zero, it manipulates a linked list starting at offset 176 of a6 to remove a node, updates several fields in the structure, and calls another routine if a value at offset 7 of the node is 10; otherwise, it sets a global variable to 1.
- `enable_interrupts_and_return`: Restores d2 and a6 from the stack, sets bit 0 of the interrupt enable register to enable interrupts, and returns.
- `configure_interrupt_state`: Writes 1 to a hardware register, clears d1, moves a byte from offset 425 of a structure in a1 to d1, performs a bitwise OR with a word from offset 468 of a1 into d1, clears offset 425 of a1, moves d1 to a hardware register, tests a pointer at offset 78 of a1 and branches, otherwise tests a word at offset 72 of a1 and branches, otherwise returns.
- `reset_serial_device`: Writes 801 to the INTENA register, calls another routine twice if a 32-bit field at offset 74 of a6 is non-zero, clears a byte at offset 69 of a6, writes several 32-bit values to a structure in a1, calls another routine twice, writes 8801 to the INTENA register, and if a byte at offset 30 of a1 matches a mask, calls a routine via a pointer located at an offset from a pointer in a6.
- `call_handler_and_branch`: Calls another routine and then branches to another routine.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `signal_and_call_handler`: Sets bit 2 of offset 472 of a6 if offset 78 of a6 is non-zero, and if offset 74 of a6 is non-zero, calls another routine and then calls a routine via a pointer if a specific bitmask is not met.
- `clear_interrupt_and_call_handler`: Clears bit 3 of a byte at offset 472 of a structure in a6, sets a global variable to 32769, and if a longword at offset 74 of the structure in a6 is non-zero, calls another routine and then calls a third routine if a byte at an offset from a pointer in a1 satisfies a specific mask.
- `set_interrupt_mask_and_branch`: Sets the interrupt enable register to a specific 16-bit value, calls another routine, sets the interrupt enable register to a different 16-bit value, and then branches to another routine.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `transfer_and_link_data`: Copies a 32-bit value from a pointer in a0 to a pointer in a1, updates a pointer at offset 4 of a structure, and enters a loop that calls another routine and repeats if a 32-bit value is non-zero.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_status_and_request_data`: Updates a byte at offset 81 of a structure using bits from a global variable and other structure offsets, copies a 32-bit value to offset 32, and calls another routine if a specific bitmask check on offset 30 fails.
- `test_structure_flags`: Clears bit 0 of a byte at offset 30 of a structure in a1, tests bit 3 of a byte at offset 79 of the structure in a1, and branches based on the results of testing words at offsets 72 and 78 of a structure in a6.
- `process_serial_event`: Moves a 32-bit value from offset 106 of a structure in a6 to d0, calls another routine, tests a byte at offset 31 of a1 and branches, otherwise moves a pointer from a1 to offset 78 of a6, sets bit 5 of offset 46 of a6, moves the address of a hardware register to a0, writes 34304 to that register, and branches to another routine.
- `reply_and_clear_active_request`: Pushes a6 to the stack, updates a6 using a pointer at offset 98, calls a routine via a pointer, restores a6 from the stack, clears a 32-bit value at offset 78 of the structure in a6, and returns.
- `clear_uart_break_and_reply`: Clears bit 5 of a byte at offset 1d6 of a structure in a1, and if bit 5 of that byte is set, a non-zero value is in a6, and a short at offset 1c of a pointer in a1 is 10, it sets bit 1 of the byte, clears a pointer in a1, writes 2048 to a global variable, calls a routine via a pointer, and sets a global variable to 32769.
- `start_timed_read_request`: Sets a global variable to one, clears a 32-bit value at offset 142 of a structure in a6, stores a 32-bit value from d0 at offset 146, sets a 16-bit value at offset 13e to nine, calls a routine via a pointer, and if a byte at offset 1f of a1 is non-zero, sets that byte to 11.
- `modify_status_byte_2`: Tests bit 7 of a byte at offset 79 of a1, and if not set, sets bit 7 of a byte at offset 69 of a6, clears bit 2 of a byte at offset 472 of a6, tests bit 3 of offset 472 of a6, and if set, clears bit 7 of offset 79 of a1, calls another routine, sets bit 7 of offset 79 of a1, and returns.
- `check_and_set_status_byte`: Clears bit 7 of a byte at offset 69 of a structure in a6, then checks bit 1 of a byte at offset 474 of the same structure and a longword at offset 78.
- `toggle_interrupt_and_call_handler`: Writes 2049 to the INTENA register, calls another routine, writes 34817 to the INTENA register, and if a byte at offset 30 of a structure in a1 satisfies a mask, calls another routine.
- `validate_and_sync_serial_parameters`: Validates a series of character and integer constraints within a structure in a1, then copies several fields from that structure into a secondary structure in a6.
- `compute_unit_channel_masks`: Calculates a 16-bit mask based on a byte at offset 4d of a1, modifies it based on a byte at offset 4e of a1 and a bit in a byte at offset 45 of a6, stores the result in a structure in a6, and calls two other routines to store results at offsets 1d2 and 1d0.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_byte_and_branch_2`: Writes 5 to offset 31 of a structure in a1 and branches to another routine.
- `calculate_period_quotient`: Moves a 32-bit value from offset 450 of a6 to d0, copies d0 to d1, shifts d0 left by 3, subtracts d1 from d0, loads a constant into d1, compares d0 to 65535 and branches, otherwise shifts d0 right by 5, performs a 16-bit unsigned division of d0 by d1, masks d1 to 16 bits, shifts d1 right by 5, and branches to another routine.
- `store_serial_period_quotient`: Divides a 32-bit value in d1 by a 16-bit value in d0, stores the quotient in a structure in a6 and a global variable, clears bit 5 of a byte at offset 1d8 of a6, and if a byte at offset 42 of a6 is 8 and bit 0 of a byte at offset 45 of a6 is set, it performs a bitwise OR on the quotient and sets bit 5 of the byte at offset 1d8.
- `setup_serial_device`: Loads two addresses into A0 and A3 using offsets from A6, calls another routine twice, and then branches to another location.
- `execute_serial_setup`: Loads two different addresses into a0 and a3 using offsets from a6, then calls another routine twice before branching to another location.
- `store_pointer_to_table`: Stores the value from a0 into a location at offset 10 of a structure in a1.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `initialize_serial_device_state`: Updates several fields in a structure, including status bytes and pointers, stores a function pointer in a table, and calls a library function via a pointer.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_and_branch`: Clears d0 and branches to another location.

### parallel.device

- `synchronize_buffer_status`: Checks status bits in two structures, increments a 16-bit value if certain conditions are met, copies a byte between structures, sets bit 4 of a status byte, and conditionally copies 8-byte data blocks.
- `update_buffer_status`: Increments a 16-bit value at offset 32 of a structure in a6, copies a byte from offset 117 of a6 to offset 53 of a1, sets bit 4 of offset 118 of a6, and if bit 1 of the byte at offset 53 of a1 is set, copies two 32-bit values from offsets 90 and 94 of a6 to offsets 54 and 58 of a1.
- `update_status_and_copy_data`: Clears a byte at offset 76 of a structure in a6, conditionally sets bit 5 of offset 75, increments a 16-bit counter at offset 20 of a6 if two bytes are zero, copies a byte from offset 35 of a1 to offset 75 of a6, sets bit 4 of offset 76, and conditionally copies two 32-bit words from a6 to a1.
- `process_parallel_event_2`: Checks a status byte at offset 118 of a structure in a6 and a 16-bit value at offset 28 of a structure in a1 to update a global variable and call another routine if specific bit conditions are met, finally setting bit 2 of the status byte at offset 118.
- `process_parallel_event`: If a short at offset 1c of a1 is 3, it clears bit 0 of a global variable, and if bit 0 is zero, it checks if bit 2 of a byte at offset 76 of a6 is clear before calling a routine via a pointer, then sets bit 2 of that byte; finally, it calls another routine via a pointer using a1 and a6.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `write_status_byte_3`: Stores the byte value 253 into offset 31 of a structure in a1 and then branches to another routine.
- `test_and_clear_long`: Tests a long value at offset 36 of a structure in a1; if it is not zero, it branches to another routine, otherwise it clears a long value at offset 32 of the structure in a1 and branches to another routine.
- `update_device_status_2`: Clears bit 1 of a byte at offset 30 of a structure in a1, then checks bit 2 of a byte at offset 118 of a structure in a6.
- `update_device_status`: Sets several fields in a structure, modifies a global variable, and performs bitwise operations on memory values.
- `process_parallel_device_queue`: Manages a linked list of device requests by decrementing a counter, updating status flags, and performing a loop to find a value in a byte array.
- `manage_parallel_device_state`: Checks a byte at offset 30 of a structure in d0, performs logic involving a global variable and a structure in a6, increments two integers at offsets 42 and 46 of the structure in a6, and potentially executes a loop and calls two different routines.
- `reset_device_state`: Copies a 32-bit value from offset 46 of a structure in a6 to offset 32 of a structure in a1, clears bit 3 of a byte at offset 118 in a6, and clears bit 5 of a byte at offset 30 in a1.
- `check_cia_bit`: Moves a byte from a register to a CIA hardware register, then branches to another routine if the byte is not zero; otherwise it tests bit 3 of a byte at offset 118 of a structure in a6 and branches to another routine if set, otherwise it branches to another routine.
- `process_buffer_update`: Calls another routine, loads a pointer from a structure in a6 into a2, loads a value from a global variable into d0, saves a6 to the stack, loads a pointer from offset 50 of a6 into a6, calls another routine, restores a6 from the stack, and branches to another routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_if_not_zero`: Checks if a 32-bit value at offset 26 of a structure in a6 is non-zero and, if so, calls a routine via a pointer located at an offset from a value at offset 22 of a6.
- `invoke_callback_pointer`: Executes a function through a pointer stored in a structure if the associated index is not zero.
- `process_data_node`: Loads a pointer from offset 76 of a6 into a0, moves the value at a0 into a1, and moves the value at a1 into d0.
- `clear_word_field`: Clears a 16-bit word at offset 114 of the structure in a6 and returns.
- `modify_status_byte`: Sets bit 5 of a byte at offset 30 of a structure in a1, sets a byte at offset 31 of the structure to 254, and calls a routine via a pointer stored in a structure in a6.
- `update_global_and_field`: Manipulates bits within a global variable and updates a byte in a structure using a library function call.
- `set_status_byte`: Tests bit 2 of a byte at offset 118 of a structure in a6; if it is set, it moves the byte value 1 into offset 31 of a structure in a1 and branches to another routine, otherwise it branches to another routine.
- `copy_structure_fields`: Copies a byte from one structure to another and, if a specific bit is set, copies two long words as well.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `recurse_on_pointer`: Saves a0 and a6 to the stack, calls a routine at an address provided in a pointer, restores the stack, and returns.
- `recursively_calls_itself`: Calls itself and then returns.
- `call_cleanup_routine`: Pushes a long value from a1 onto the stack, calls another routine, and then returns.
- `jump_to_handler`: Performs a long jump to an address stored at offset 247 of a structure in a1.
- `call_external_handler_2`: Pushes a long value from a1 onto the stack, calls a routine at a relative offset, cleans up the stack, and returns.
- `push_pointer_and_call`: Pushes the value of a1 onto the stack and calls a routine at an address provided in a pointer.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `execute_library_call`: Saves registers d2, d3, and a2 to the stack, loads a long value from offset 16 of the stack into d3, loads a pointer from offset 20 of the stack into a2, stores a2 into a global variable, clears the stack, pushes a pointer to a structure, calls another routine, stores d0 into a global variable, increments the stack pointer, and branches to another routine if d0 is not zero.
- `check_clipboard_status`: Stores a value from d3 into a global variable, calls three different routines, and returns the result of a comparison between the return value of the third routine and zero.
- `call_two_functions`: Calls two different functions via library vectors and returns one.
- `update_clipboard_data_structure`: Calls a function with two constants and, if the result is non-zero, increments a value at an offset within the returned structure or clears it, then calls another routine recursively.
- `call_library_function_2`: Clears a 16-bit word at offset 14 of a3, pushes a3 onto the stack, pushes the address at offset 72 of a2 onto the stack, calls a routine, cleans up the stack, and branches to another routine.
- `zero_word_field`: Stores a zero word into a specific offset within a structure and then calls another routine.
- `process_clipboard_data_structure`: Calls a routine using a pointer from the stack, then if the value at offset 72 of the returned structure is not zero, calls another routine with a pointer to that value and returns the result.
- `update_pointer_chain`: Pushes the address at offset 72 of a2 onto the stack, calls a routine, clears a 32-bit value at offset 86 of a2, calls another routine, cleans up the stack, restores d2 and a2 from the stack, and returns.
- `copy_clipboard_buffer_segment`: Traverses a linked list of 512-byte buffers to locate a specific segment and copies a range of bytes from the buffer data at offset 18 to a destination pointer.
- `compare_and_calculate_offset`: Compares a long word from a global variable with a value on the stack, and if the comparison passes, calculates a new offset by adding a stack value to a value at offset 44 of a2 and comparing it against the global variable, finally storing a result in d3 derived from the global variable and the stack value before branching to another routine.
- `copy_linked_list_data`: Traverses a linked list to find a specific node based on a value derived from stack parameters, then copies bytes from the node's data at offset 18 to a destination buffer in a3, handling potential 512-byte boundaries and updating the list pointer in a structure in a2.
- `compare_and_update_pointer`: Copies d1 to d0, compares d0 with a value at offset 14 of a structure in a1, and if d0 is not lower, updates a1 with a pointer from offset 0 of that structure before branching.
- `calculate_copy_limit`: Copies d3 to d5, checks if d5 is zero, and if not, calculates a value in d0 by subtracting d2 from 512, then compares d5 to d0 and branches accordingly.
- `copy_data_loop`: Copies bytes from a table starting at offset 18 of a1 into a destination pointed to by a3 using d4 as an index, incrementing d4 each time.
- `transfer_d2_to_d4`: Moves a 16-bit word from d2 to d4 and then branches to another routine.
- `copy_data_with_limit`: Copies bytes from a table starting at offset 18 of a1 into a destination pointed to by a3 using d4 as an index, incrementing d4 each time.
- `store_long_word`: Stores a long word into a specific offset within a structure and then calls another routine.
- `update_clipboard_buffer_chain`: Iterates through a linked list of memory blocks to validate or update clipboard data, copying bytes from a source pointer into destination buffers while handling block offsets and list traversal.
- `copy_clipboard_data_to_buffer`: Iterates through a linked list of memory blocks to copy bytes from a source buffer into the destination blocks.
- `branch_to_offset_7`: Pushes the address of an offset of a structure in a2 onto the stack, calls another routine, adjusts the stack pointer, and branches to another location.
- `distribute_clipboard_buffer_data`: Iterates through a linked list of buffers to copy bytes from a source pointer to destination memory locations, adjusting for buffer offsets and remaining byte counts.
- `transfer_clipboard_data`: Calls another routine, then iterates through a linked list of buffers to copy bytes from a source in a3 to destination buffers, adjusting for 512-byte limits and updating a value at offset 20 of a structure in a6.
- `transfer_clipboard_buffer`: Iterates through a linked list of buffers to copy bytes from a source buffer in a3 to the destination buffers, adjusting for 512-byte limits and updating the remaining byte counts in a structure in a6 and the list pointer in a2.
- `check_and_branch_on_limit`: Tests d4 and returns if zero, otherwise tests a value at offset 20 of a structure in a6 and returns if it is lower; if not, it calculates the difference between 512 and d3, compares it to the value at offset 20 of the structure in a6, and branches based on the result.
- `copy_data_with_overflow_handling`: Copies bytes from a source structure to a destination structure in a loop, and if a counter exceeds 511, it follows a pointer to a new structure, updates a longword at offset 20 of a structure in a6, and calls another routine.
- `transfer_d3_to_d1`: Moves a 16-bit word from d3 to d1 and then branches to another routine.
- `copy_buffer_data`: Copies bytes from a source pointer to a destination buffer at an offset of 18, potentially calling another routine to update a pointer in a structure.
- `store_register_to_offset`: Stores the contents of a data register into a memory location calculated by adding a fixed offset to a pointer held in an address register.
- `check_buffer_bits`: Calls a routine, then checks bit 7 and bit 0 of a byte at offset 30 of a structure, and calls another routine if bit 0 is not set.
- `modify_byte_bits`: Clears the least significant bit and sets the most significant bit of a byte at a specific offset in a buffer.
- `modify_byte_bits_2`: Checks bit 7 of a byte at offset 30 of a structure and calls another routine if it is not set, then clears bit 1 of that byte and calls another routine, and finally clears bit 2 of that byte before returning.
- `compare_structure_values`: Compares a longword at offset 48 of a structure with a longword at offset 112 of another structure, and sets d0 to 1 if the first is not greater than or equal to the second.
- `clear_and_branch_2`: Clears register d0 and branches to another routine.
- `clear_and_branch_3`: Sets register d0 to zero and performs a short branch to another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_structure_fields_2`: Copies a 32-bit value from offset 40 of the second structure to offset 152 of the first structure, a 16-bit value from offset 16 of the second structure to offset 146 of the first, and a 32-bit value from offset 48 of the second structure to offset 148 of the first, then calls another routine.
- `modify_clipboard_segment_status`: Sets bit 5 of a byte at offset 30 of a structure pointed to by a value at offset 12 of the stack frame, then checks a byte at offset 66 of a second structure; if that byte is zero, it clears bit 5 of the byte at offset 30 and otherwise returns.
- `update_buffer_segment_status`: Iterates through a linked list of buffer segments, updating status and size fields based on segment type.
- `clear_d3_register_2`: Clears the d3 register and branches to another routine.
- `process_clipboard_buffer_segments`: Iterates through a linked list of buffer segments to update their size and status fields based on their type.
- `call_process_clipboard`: Pushes a pointer from a2 onto the stack, calls another routine, cleans the stack, stores 1 at offset 94 of a2, and branches to another routine.
- `recursively_update_buffer_segments`: Updates a structure's status and size fields by reading data from a linked buffer, performing recursive updates to buffer segments, and managing segment counts.
- `clear_register_and_branch_3`: Clears the word in d3 and branches to another routine.
- `update_clipboard_buffer_state`: Updates a structure's status and size fields by reading data from a linked buffer, performing recursive updates to buffer segments, and managing a counter based on specific status flags and buffer lengths.
- `manage_clipboard_buffer_state`: Checks a status field in a structure to determine if a buffer segment should be processed, updates a size field based on a recursive call, and modifies buffer offsets and counters.
- `update_buffer_offset`: Loads a longword from offset 90 of a structure in a2 and a longword from offset 36 of a structure in a4, adds the longword at offset 44 of a structure in a4 to the second value, compares the results, and if the first value is not lower than the second, moves the longword from offset 36 of a structure in a4 to offset 32 of the same structure before branching.
- `adjust_buffer_offset_value`: Calculates a new value by subtracting a longword at offset 44 of a structure in a4 from a longword at offset 90 of a structure in a2, stores the result at offset 32 of the structure in a4, and then adds that result to the longword at offset 44 of the structure in a4.
- `process_buffer_segment`: Clears a long value at offset 32 of the structure in a4, sets a long value at offset 48 of the structure in a4 to 255, decrements a 16-bit word at offset 110 of the structure in a2, pushes a3 and a4 onto the stack, calls another routine, and if the word at offset 110 of the structure in a2 is non-zero, clears a 16-bit word at offset 64 of the structure in a2 and loops back to the start of the routine.
- `process_clipboard_buffer_state`: Checks a status field in a structure to determine if a buffer segment should be processed, updates a size field based on a recursive call, and clears a status byte and a flag in another structure before returning.
- `copy_clipboard_pointer`: Copies a longword from an offset of 44 in a structure pointed to by a4 into a longword at offset 90 of a structure in a2, pushes a3 and a4 onto the stack, calls another routine, increments the stack pointer by 8, and branches to another location.
- `check_clipboard_status_2`: Compares a word at offset 28 of a structure in A4 to 4, and if they are not equal, writes 4 to offset 64 of a structure in A2 before branching to another location.
- `clear_d3_register`: Clears the d3 register and branches to another location.
- `update_clipboard_buffer_status`: Checks a status field in a structure to determine if a buffer segment should be processed, increments a counter by eight if a sub-routine returns a specific value, clears a 16-bit field, and then calls another routine.
- `clear_memory_location`: Clears a long value at offset 90 of a structure in a2 and then branches.
- `process_clipboard_data_structure_2`: Iterates through a data structure to update various fields based on pointer values and status bits, performing conditional arithmetic on integer values at specific offsets and calling other routines to handle state changes or bit manipulations.
- `traverse_linked_list`: Traverses a linked list by comparing a value at offset 48 of a structure against a longword in a list element, then calls another routine with a value from the list.
- `clear_and_branch_4`: Sets d1 to zero and branches to another routine.
- `call_library_function`: Saves three registers to the stack, calls a library function using a base address from a structure offset, and returns.
- `set_byte_value`: Writes the constant byte value 253 to offset 31 of a structure and calls another routine.
- `set_byte_constant_offset`: Writes a constant byte value to a specific offset within a structure and then calls another routine.
- `set_byte_constant_offset_2`: Stores the constant byte value 253 at offset 31 of a structure pointed to by a value on the stack and then calls another routine.
- `call_set_byte_value`: Pushes a pointer from the stack onto the stack and calls set_byte_value.
- `call_set_byte_value_2`: Pushes a pointer from the stack onto the stack, calls set_byte_value, and then returns.
- `update_structure_byte_sequence`: Calls three separate routines in sequence, passing the same value retrieved from a stack offset to each one.
- `update_buffer_status_3`: Copies a 32-bit value from offset 56 of a structure pointed to by a value at offset 24 of the input structure to offset 48 of the input structure, sets bit 4 of a byte at offset 30 of the input structure, and then calls another routine recursively based on a comparison between the value at offset 48 and a value at offset 60 of the structure pointed to by offset 24.
- `call_subroutine_with_offset`: Calls another routine using a value from a register plus an offset of 96 and then calls a second routine.
- `increment_buffer_counter`: Increments a 32-bit value at offset 56 of a structure pointed to by a value at offset 24 of the input structure, stores the result in offset 48 of the input structure, and then calls another routine recursively based on a comparison between the value at offset 48 and a value at offset 60 of the structure pointed to by offset 24.
- `call_subroutine_with_different_offset`: Calls another routine using a value from a register plus an offset of 112 and then calls a second routine.
- `copy_structure_field`: Copies a longword from offset 56 of a structure pointed to by an offset 24 pointer within a structure to offset 48 of the original structure, then calls another routine.
- `copy_structure_field_offset`: Copies a longword from offset 60 of a structure pointed to by an offset 24 pointer within a structure to offset 48 of the original structure, then calls another routine.
- `search_and_allocate_clipboard_node`: Traverses a linked list to find a node matching a specific value; if no match is found, it calls another routine to allocate a new node, initializes its fields, and manages a buffer by checking for capacity and potentially reallocating or shifting data within the structure.
- `compare_and_branch_3`: Compares d3 with a longword at offset 14 of a structure, and if they are not equal, sets d2 and d4 to zero before branching to another routine.
- `reallocate_clipboard_buffer`: Attempts to allocate a new memory block for a clipboard structure, initializes several fields including a pointer to a global variable and status flags, and then enters a loop to manage buffer resizing and data integrity by calling other routines to copy or validate memory segments.
- `manage_clipboard_buffer_overflow`: Updates a clipboard data structure by setting several status flags and calling a function via a pointer, then attempts to expand the buffer by checking its capacity and incrementing a length value.
- `process_buffer_limit`: Calls a library function, and if the result is 512, clears a word at offset 94 of a structure and subtracts 512 from a value at offset -4 of a structure.
- `loop_and_clear_status`: Tests a word at offset 94 of a structure in a3 and returns if zero, otherwise tests a long value at offset -4 of a structure in a6 and returns if it is less than or equal to zero; if not, it pushes a3 onto the stack, calls a routine, and if the return value in d0 is not zero, clears a word at offset 94 of the structure in a3 and loops.
- `process_clipboard_data`: Calls a routine using a pointer from the stack, performs conditional updates to a structure, and enters a loop that calls another routine repeatedly based on values retrieved from a structure.
- `process_complex_structure`: Increments values in a structure, performs conditional library calls based on a flag, and returns a value from a structure offset.
- `decrement_and_cleanup_clipboard_structure`: Decrements a 16-bit counter at offset 18 of a structure and, if it reaches zero, performs a recursive cleanup of linked list data, clears several 32-bit fields at offsets 20 and 24, and decrements a global byte before conditionally calling another routine.
- `verify_library_result`: Calls a library function and compares the result with d4, clearing a word at offset 94 of a structure if they are not equal.
- `clear_status_word`: Clears a word at offset 94 of a structure and branches to another routine.
- `cleanup_and_call_library`: Calls three different routines, clears two 32-bit values at offsets 20 and 24 of a structure in a2, decrements a global byte, and if that byte is zero and bit 3 of a global variable is set, calls another routine before calling a final routine and returning.
- `cleanup_structure_data`: Resets two 32-bit fields at offsets 20 and 24 of a structure to zero, decrements a global byte, and conditionally calls another routine if the byte is zero and bit 3 of a global variable is set.
- `execute_library_calls_4`: If a word at offset 252 is not zero, it calls a routine using a pointer from a global variable and then calls another routine with a pointer from offset 220.
- `set_status_bit_3`: Sets bit 3 of a byte in a global variable and returns zero in d0.
- `update_structure_status`: Calls another routine, writes the byte value 5 to offset 8 of a structure in a2, modifies a byte at offset 30 of that structure, and calls another routine if a word at offset 28 is less than or equal to 12.
- `invoke_table_entry_4`: The routine invokes a function pointer retrieved from an offset within a global variable using an index.
- `check_bits_and_set_byte`: Calls another routine, then if bits 7 and 6 of a byte at offset 30 are both not set, writes the constant byte 254 to offset 31 and calls another routine if bit 5 is set, followed by another routine call, before calling a final routine and returning.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_276_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_432`: Library stub: loads a library base from a global variable and calls the library function at offset minus 432 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `update_linked_list_node`: Stores a pointer into the memory location pointed to by the argument, increments that location by four, clears the following word, and stores the original pointer into the word after that.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `invoke_table_entry_6`: Retrieves a function pointer from a global variable and calls it using arguments passed via the stack.
- `store_byte_and_increment_2`: Stores the byte from register d0 at the address in a3 and increments a3.

### printer.device

- `call_initialization_routine`: Calls a routine through a pointer at offset -54 of a structure in a6, and if the result is non-zero, stores that result in a global variable.
- `set_status_and_call`: Sets the value of d0 to negative two and calls another routine.
- `manage_linked_list`: Sets a bit in a byte, performs a linked list insertion if a flag is set, and calls functions through pointers retrieved from a table.
- `execute_printer_initialization`: Calls a routine through a pointer at offset -54 of a structure in a6, and if the result is non-zero, stores that result in a global variable, sets a value at offset 34 of the result structure, and calls another routine through a pointer at offset -272 of a6.
- `initialize_device_and_buffer`: Calls four functions from a table using a base address in a6, and if the first three return non-zero, it fills a buffer starting at offset 434 of a structure in d0 with the value 45019 in a loop, sets several fields in that structure, and calls two more functions from the table before setting a global variable to zero.
- `call_library_functions`: Calls four different functions retrieved from a table using a base address and returns 0.
- `call_three_handlers`: Calls three different routines via branch instructions and then returns.
- `set_status_and_call_short`: Sets the value of d0 to negative three and calls another routine.
- `set_status_bit`: Sets bit 0 of a byte at offset 61 of the structure in a6.
- `clear_status_bit`: Clears bit 0 of a byte at offset 61 of the structure in a6.
- `check_and_call_handler`: Loads a pointer from offset 48 of a structure in a6, checks if it is null, and if not, calls another routine before branching back to itself.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_short_handler`: Performs a short branch to another routine.
- `initialize_device_structures`: Initializes several fields in a structure at a6, including setting up linked lists and copying values from a function call, then enters a loop that processes bits from a value and iterates through a linked list to call functions via a table.
- `compare_and_clear_bit`: Compares d0 with a value at an offset of a6, and if they are equal, clears bit 1 of a byte at an offset of a6.
- `process_device_events`: Enters a loop that checks bits in a value to either iterate through a linked list and call functions via a table, or check for specific values to clear bits in a byte at an offset of a6.
- `dispatch_serial_buffer`: Loads a pointer from offset 4 of a7 and a value from offset 8 of a7, then calls another routine.
- `update_status_and_pointers`: Checks a global variable, and if a returned byte is zero, it modifies a byte at offset 2470 of that global variable by setting bit 0 or bit 1, then writes two 32-bit values and one 16-bit value to specific offsets of a structure based on the previous value, and calls two other routines.
- `adjust_stack_and_branch`: Adds 8 to the stack pointer and branches to another routine.
- `set_status_bit_5`: Sets bit 1 of a byte at offset 158 of a structure in a6, loads the address of a byte at offset 190 of a6 into a1, and branches to another routine.
- `check_status_byte`: Loads a pointer from a global variable, reads a byte from offset 31 of that pointer, and if it is not zero, checks if bit 0 or bit 1 of a byte at offset 2486 of a structure in a6 is set, potentially calling another routine before returning.
- `loop_status_checks`: Performs a loop that calls three different routines and checks a value at offset 14c of a structure in a6 to determine whether to continue.
- `compare_and_branch_structure`: Loads the address of a structure into a1 from a6 plus 94, compares a1 to d0, and if they are not equal, branches to another routine; otherwise, it clears bit 1 of a value at offset 950 of the structure in a6, calls another routine, and branches to another routine.
- `compare_and_branch_register`: Compares the value at offset 272 of a structure in a6 to the value in a1, and if they are not equal, branches to another routine; otherwise, it calls another routine and branches if the result in d0 is zero.
- `call_and_branch`: Calls another routine and then performs a short branch to another routine.
- `initialize_structure_fields`: Loads a pointer into a1 from a6 plus 272, writes 9 to offset 28 of that pointer, copies a 32-bit value from a pointer in a0 to offset 32 of the pointer in a1, and clears a 32-bit value at offset 36 of the pointer in a1.
- `clear_status_byte`: Sets a byte at a specific offset to zero and calls a routine through a pointer found in a structure.
- `verify_offset_byte`: Loads the address of a structure at offset 272 from a6, compares the byte at offset 8 of that structure to 7, and returns if they are not equal.
- `execute_callback_sequence`: Calls a routine through a pointer in a structure and then calls another routine using a value from a global base.
- `check_and_clear_byte`: Loads a pointer from a location, reads a byte from offset 31 of that pointer, and if it is not zero, performs a bitwise AND on it with 3 and branches if the result is non-zero; otherwise, it restores a6 from the stack and returns.
- `process_status_flags`: Enters a loop that repeatedly calls three different routines and checks a value at an offset of 276 from a6.
- `compare_and_clear_bit_2`: Compares a longword in d0 with a value at an offset of 94 from a6, and if they are equal, clears bit 1 of a byte at offset 1446 of a6 before calling another routine and branching.
- `compare_and_branch_5`: Compares a longword at offset 110 of a6 with d0, branches if they are not equal, calls another routine, and then branches based on the result in d0.
- `call_verify_offset_byte_and_branch`: Calls verify_offset_byte and then performs a relative branch to another routine.
- `branch_if_byte_matches`: Saves registers to the stack, compares a byte at offset 2457 of a structure in a6 to one, and if they are not equal, branches to a routine using a pointer pushed onto the stack.
- `update_interrupt_and_status`: Modifies a byte in a global variable, updates a value in a global variable, calls another routine, and clears bits 0 or 1 of a byte at offset 2486 of a structure in a6 based on the results of the call and the current state of the byte.
- `clear_status_bits_2`: Calls a routine through a pointer at a specific offset, and if it returns zero, clears bit 0 of a byte at offset 2486 of a structure in a6 and calls another routine if bit 0 was set, then clears bit 1 of that same byte and calls another routine if bit 1 was set.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `update_list_and_copy_string`: Pushes two values from a1 onto the stack, calls a routine through a pointer, restores a1 from the stack, and calls another routine.
- `reset_structure_field`: Sets a 32-bit field in a structure at offset 32 of a1 to zero and conditionally updates it from the field at offset 36 based on a routine return value, then calls another routine twice.
- `update_list_and_copy_string_2`: Pushes two longwords from a1 onto the stack, calls a routine through a pointer, restores a1 from the stack, and calls another routine before returning.
- `execute_printer_handler_and_callback`: Pushes two pointers onto the stack, calls a routine via a pointer, restores the stack, and then calls another routine via a branch and link instruction before returning.
- `dispatch_pending_entry`: Checks a condition via a routine call, and if met, populates a structure with two parameters and executes several routines through a global base.
- `initialize_printer_buffer_processing`: Saves registers to the stack, loads a pointer from an offset of 8 in a6, and calls a routine at address 2358 with constants 284 and 266 in a3 and a4, then branches based on the result in d0.
- `process_printer_buffer_queue_2`: Calculates the length of a null-terminated string from a source pointer, then iterates through the string to copy bytes into a destination table based on a lookup index.
- `update_printer_buffer_2`: Moves a longword from the address in a3 to a global variable, clears a 16-bit word at a global variable, and then branches to another location.
- `copy_and_process_printer_buffer`: Copies bytes from a source buffer to a destination table using two incrementing pointers, then enters a loop to process the data by calling process_printer_escape_sequences and other routines to handle control sequences or character selection.
- `clear_registers_and_branch`: Sets registers d4, d6, and d2 to zero and then branches to another routine.
- `process_printer_buffer_queue`: Iterates through a data structure to manage a buffer, performing conditional byte copies from a source pointer to a destination table while updating counters and status values.
- `copy_and_update_data`: Enters a loop that copies bytes from one memory location to another while incrementing two counters, then writes either a 32-bit value or zero to an offset of 32 in a structure pointed to by a2 based on the value of d2.
- `clear_memory_offset`: Stores a zero at a specific offset within a memory location pointed to by a register.
- `transform_byte_via_lookup_table_2`: Calls clear_and_branch_5, then enters a loop that calls transform_byte_via_lookup_table, clear_byte_and_branch, and call_and_negate_on_flag.
- `clear_and_branch_5`: Clears a longword at an offset of negative 4 from a6 and then branches to another routine.
- `translate_and_shift_table`: Loops through a table to compare bytes, performs a series of conditional assignments to a destination pointed to by a2 based on a global variable and index values, and then shifts the contents of a global table one byte forward.
- `write_byte_on_match_2`: Compares d0 to a longword at offset 470 and then compares d0 to d5, and if both conditions are met, writes the byte 75 to the location pointed to by a2.
- `write_byte_on_match`: Compares d0 to a longword at offset 470 and then compares d0 to d5, and if both conditions are met, writes the byte 75 to the location pointed to by a2 and branches to another routine.
- `check_and_set_byte_2`: Compares the value in d0 with d5, and if they are not equal, checks if the byte at the address in a4 is non-zero; if it is non-zero, it writes the byte 59 to the address in a2.
- `copy_byte_from_table_3`: If d5 is greater than or equal to 6, it branches to another routine; otherwise, it copies a byte from a table at an offset determined by d5 into the location pointed to by a2.
- `check_and_set_byte`: If d5 is not zero, it branches to another routine; otherwise, it checks if the byte at the location pointed to by a4 is at least 29, and if so, writes the value 12 to the location pointed to by a2 before branching to another routine.
- `transform_byte_via_lookup_table`: Compares a byte against a table of seven entries to perform a conditional assignment to a destination, then enters a nested loop structure that performs further conditional assignments to that destination based on the value of a second byte and a global table.
- `process_printer_escape_sequences`: Iterates through a byte stream to identify and process escape characters, handling specific control sequences for character selection and line formatting.
- `parse_printer_escape_sequences`: Iterates through a byte stream to identify and process escape characters, handling specific control sequences for character selection, line feeds, and command lookups in a table.
- `process_printer_command_stream_2`: Iterates through a byte stream to parse control characters and data, performing table lookups for specific command sequences and writing bytes to a destination buffer.
- `compare_printer_byte`: Compares the byte at the address in A3 to 91, and if it is not equal, branches to another location; otherwise it branches to another location.
- `parse_printer_command_stream`: Parses a stream of characters to identify control codes and command sequences, performing string lookups in a table for commands starting with values 160 or higher.
- `test_printer_command`: Compares the byte at the address in A3 to 10, and if it is not equal, branches to another location; otherwise it tests a byte at offset 260 of a structure and, if non-zero, writes 3 to the address in A4 before branching to another location.
- `process_printer_command_stream`: Parses a stream of bytes to handle printer control sequences, including escape characters and specific command codes.
- `write_multiple_bytes`: Writes the value 10 to a location calculated by adding d2 to a pointer, increments that pointer, then writes the value 13 to a second location calculated by adding d2 to the new pointer, and finally branches to another routine.
- `write_single_byte`: Writes the value 10 to a location calculated by adding d2 to a pointer, increments that pointer, and then branches to another routine.
- `check_path_character`: Compares a byte at the address in a3 to 155, and if it is not equal, pushes d2 and d3 onto the stack, calls another routine, increments the stack pointer by 8, and branches to another location.
- `parse_file_path_string_2`: Parses a string from a table, handling backslash-delimited path components and converting numeric characters following a slash into integer values.
- `parse_file_path_string`: Parses a string to extract directory and file components, handling backslash separators and numeric directory segments.
- `update_pointer_table_3`: Increments a1 and a global variable, checks a byte at the address in a1, and if it is non-zero, performs a series of pointer manipulations and updates a global variable before branching to another routine.
- `calculate_pointer_address`: Reads a byte from a3, subtracts 160, shifts it left by 2, adds it to d4, and uses the result to load a pointer into a1 before branching to another routine.
- `skip_non_numeric_characters`: Checks if the byte at a1 is a backslash or a digit between 0 and 9, and branches back to the start of the routine if the character is a backslash or a digit.
- `parse_ascii_numeric_string`: Parses up to three consecutive ASCII digits from a string to form a numeric value, stores this value into a buffer at an offset determined by an incrementing index, and then processes subsequent characters through a series of conditional checks and recursive calls to another routine.
- `copy_byte_to_pointer`: Increments a pointer in a2, adds the value in d2 to a new pointer derived from the original, and copies a byte from the address in a1 to that new location before branching.
- `parse_printer_escape_sequences_2`: Iterates through a byte stream to identify and process escape characters, handling specific control sequences for character selection, line feeds, and other printer commands by calling another routine to update global values.
- `update_global_values`: Subtracts the difference between two global variables from a third global variable and clears a fourth global variable.
- `transfer_buffered_data`: Calls another routine to retrieve a length, then copies that many bytes from a local stack buffer to a destination pointed to by a global variable and an offset.
- `update_pointer_and_call`: Adds d3 to a value from a global variable and, if the result is greater than 256, updates a global variable and calls another routine if a bit in a structure is set.
- `decrement_and_store_global_pointer`: Decrements a 32-bit value from a global variable by one, stores the result in another global variable, and branches to another routine.
- `clear_register_and_branch`: Sets register d4 to zero and branches to another routine.
- `copy_memory_block`: Copies a sequence of bytes from a source address to a destination address, incrementing a global pointer for each byte.
- `clear_byte_and_branch`: Clears a byte at a location pointed to by a2, then branches to another routine.
- `clear_byte_at_offset_3`: Compares d1 to 59, and if they are equal, increments a pointer in a3, adds the value in a2 to it, and clears the byte at the resulting address.
- `set_value_at_stack_pointer`: If d1 is 32, it loads a pointer from the stack into a0 and writes the value 1 to the location pointed to by a0.
- `compare_and_load_pointer`: Compares register d1 to 34; if they are not equal, it branches to another routine, otherwise it loads a pointer from the stack into a0, sets d0 to two, and branches to another routine.
- `parse_character_input`: Enters a loop that reads a byte, checks if it is a digit, space, or quote, and if it is a semicolon, increments a pointer in a3 and clears the byte at the new location; otherwise, it updates a value at the location pointed to by a2 and a3 and returns the byte.
- `fetch_indexed_global_value`: Increments a global variable and uses its previous value to index into a table to retrieve a byte, then calls another routine and sets the result to zero if a condition is met.
- `return_register_value`: Returns the value currently held in a data register.
- `update_global_pointer_from_memory`: Loads a pointer into a1, decrements a 32-bit value from a global variable by one, stores the result at the address in a1, and branches to another routine.
- `decrement_and_check_table`: Decrements a value pointed to by a register and continues looping as long as the value is positive and a specific bit pattern is found in a table.
- `reset_global_state`: Clears three global variables, sets a byte at offset 2720 of a structure to zero, and sets another global variable to 0.
- `copy_data_to_globals`: Copies six 16-bit values from a structure into a set of global variables.
- `compare_global_data`: Compares six 16-bit values from a structure against a set of global variables, returning zero if all match or negative one otherwise.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `process_nested_calls`: Extracts four bytes from a structure pointed to by a parameter, calls another routine with a pointer and a value, and then performs a series of conditional calls based on the result and the value of a short at offset 20 of the structure.
- `dispatch_conditional_routines`: Calls a routine with parameters 1 and 0, and if the result is zero, calls another routine with a pointer from a structure in a6; if the first call returned zero and d2 is zero, it also calls another routine with parameters 3 and 0.
- `copy_string_with_logic`: Calls a routine through a pointer at offset 42 of a global variable, and if the result is zero, checks if a parameter is 54 or 53 to update a global variable, otherwise it retrieves a pointer from a table and enters a loop to copy bytes from that pointer to a destination buffer, handling specific byte values, and potentially appending a byte from a third parameter.
- `set_byte_from_stack_value`: Compares the word at offset 22 of the stack to 54 and 53, and if it matches 53, performs a bitwise AND between d1 and 1, stores the result in a global variable, and branches to another routine.
- `copy_string_with_replacements`: Enters a loop that copies bytes from a source location to a destination, handling specific character replacements and stopping when a null terminator or a specific byte is encountered.
- `set_d1_value`: Compares the byte in d1 to 255, and if it is not equal, sets d1 to 255 and branches to another location.
- `copy_string_data`: Enters a loop that copies bytes from one memory location to another, updating an index and checking for a null terminator or a specific value.
- `dispatch_via_table`: Compares a value in d1 against seven, and if it is not less than seven, uses it to index into a table to jump to a new address.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `validate_printer_buffer_dimensions_2`: Extracts printer buffer parameters from a structure, calculates the available print area width and height by subtracting offsets from a buffer size, and performs bounds checking against the requested print area.
- `validate_printer_buffer_dimensions`: Calculates the width and height of a print area by subtracting offset values from a structure, then performs bounds checking and updates the buffer dimensions.
- `calculate_printer_buffer_dimensions`: Calculates the width and height of a print area by subtracting offset values from a structure, then performs bounds checking and updates several status variables and buffer size parameters based on the results.
- `update_printer_status_and_buffer_2`: Compares two values from a structure with current values to select the minimum non-zero value and stores it in a buffer.
- `update_printer_buffer_and_status_2`: Compares two values from a structure with current values to select the minimum non-zero value and stores it in a buffer.
- `compare_and_branch_8`: Compares the word in register d0 to 2048; if they are not equal, it branches to another routine, otherwise it branches to another routine.
- `load_constant_and_branch_2`: Loads the value 12 into register d1 and branches to another routine.
- `load_constant_and_branch_3`: Loads the value 15 into register d1 and branches to another routine.
- `update_printer_buffer_and_status`: Compares two values from a structure pointed to by a register with current values, selecting the minimum non-zero value and storing it in a local buffer.
- `update_printer_status_and_buffer`: Updates a local integer value by comparing it with a value from a pointer-based structure, then modifies several global variables and bit-swaps a byte.
- `modify_stack_offset_value`: Loads two small constants into registers, stores one at an offset of negative 4 from a6, checks two different global bytes to perform conditional branches, and performs arithmetic on the value stored at the negative 4 offset from a6 before branching to another routine.
- `check_status_and_branch`: Clears a byte in a global variable, then checks a byte in another global variable and a third byte in a third global variable to perform conditional branches; if both checks pass, it pushes a stack pointer and branches to another routine.
- `update_color_range_bounds`: Calculates new color range bounds by comparing input values with existing limits, updates several global variables representing minimum and maximum color components, and clears a 64-entry table in a global variable using a loop.
- `update_color_lookup_table`: Calculates new bounds for a color range based on input values, clears a 64-entry table in a global variable, and populates it by iterating through a loop to extract 4-bit color values using a mathematical transformation.
- `generate_color_lookup_table_2_2`: Calculates a range of values based on two input bounds, clears a 64-entry table in a global variable, and populates it by extracting 4-bit color components.
- `generate_color_lookup_table_2`: Calculates a range of values based on two input bounds, clears a 64-entry table in a global variable, and populates it by extracting 4-bit color components.
- `generate_color_lookup_table`: Calculates a range of values based on two input bounds, clears a 64-entry table in a global variable, and populates it by extracting 4-bit color components from a sequence of values.
- `build_color_lookup_tables`: Clears a 64-entry table in a global variable and populates it by extracting 4-bit color components from a range of values.
- `generate_color_palette_table_2`: Clears a 64-entry table in a global variable, then iterates through a range of color values to extract 4-bit components into the table and separate memory locations.
- `generate_color_palette_table`: Clears a table of 64 entries, then iterates through a range of color values to extract 4-bit components into a global table and separate memory locations.
- `convert_and_process_color_palette`: Iterates through a range of values to extract 4-bit color components and store them into a global table, then performs bitwise transformations on a series of 32 entries in a table.
- `execute_printer_command_wrapper`: Sets three global variables to specific values, calls a routine using a pointer from a global variable, and then calls another routine with the returned value.
- `execute_printer_command`: Pushes a constant onto the stack, clears three stack slots, loads a pointer from a global variable into a2, calls the routine at that pointer, stores the result from d0 into a global variable at offset 366, and branches to another routine if the result is zero, otherwise it branches to another routine.
- `check_printer_status_and_call_2`: Tests two consecutive bytes in a global variable and, if both are non-zero, calls another routine and then branches to another routine; otherwise it returns.
- `test_printer_status`: Tests a byte in a global variable and branches to another routine if it is zero, otherwise it calls another routine and then branches to another location.
- `check_printer_status_and_call`: Tests a byte in a global variable and, if it is zero, branches to another routine; otherwise, it calls another routine and then branches to another routine.
- `store_and_test_result`: Calls a routine, stores the returned longword into a global variable, and if that value is zero, branches to another routine, otherwise it branches to another location.
- `call_conditional_routine_2`: Checks if a global variable is divisible by a value at offset 24 of a pointer in d5, or if a routine called with four zero arguments returns zero, and if so, calls another routine with zero arguments.
- `call_empty_routine`: Calls a routine with two zero arguments and returns.
- `calculate_weighted_value`: If a global variable is zero, it checks bit 2 of a byte at offset 21 of a global variable; if set, it finds the smallest byte among three provided bytes and returns it, otherwise it calculates a weighted sum of three bytes and optionally subtracts the result from 15 based on a value at offset 2688 of a global structure.
- `find_minimum_byte`: Checks bit 2 of a byte at offset 21 of a global variable and, if set, finds the smallest byte value among three provided bytes and returns it, otherwise it returns the original second parameter.
- `resolve_table_index_and_update_checksums`: Calculates a target address from a table and a global variable, then calls another routine to retrieve a value.
- `update_global_pointers`: Calculates a value based on a global variable and a table, then updates four global pointers by adding a global variable to each of them.
- `update_pointer_table_4`: If a global variable is zero, it loads a 32-bit value from a table using an index into a location pointed to by a2; otherwise, it performs a series of bitwise operations and updates a global variable before calling another routine and updating four global pointers by adding a global variable to them.
- `move_and_test_bit`: Moves a longword from an offset of a global variable to a location in a2, moves a value to a0, tests bit 3 of a byte at offset 21 of a0, and branches or performs arithmetic based on the result.
- `process_word_values`: Moves d2 to d1 and d3 to d0, shifts d0 right by 4 bits, and branches based on whether d0 is less than 1 or greater than 1.
- `branch_on_comparison`: Compares d0 to 2 and branches to an earlier instruction if they are not equal, otherwise it branches to another routine.
- `decrement_and_loop`: Decrements d0 by three and branches back to an earlier instruction if the result is not zero, otherwise it branches forward.
- `store_byte_and_branch`: Stores a byte from d1 into a memory location and branches forward.
- `update_pointer_table_2_2`: Stores a byte from register d1 into a memory location pointed to by a longword and branches to another routine.
- `update_pointer_table_2`: Stores a byte from d1 into a memory location, pushes a pointer from a2 onto the stack, calls another routine, stores the result in d0 into the location pointed to by a2, adjusts the stack pointer, and updates a pointer at offset 276 using the value from a2 before branching to another routine.
- `update_pointer_table`: Loads a 32-bit value from a table using an index and updates four global pointers by adding a global base address to them.
- `calculate_and_update_pointers`: Calculates a value based on the product or sum of two globals and updates four global variables by calling another routine multiple times.
- `execute_conditional_checks`: Sets a global variable to zero and then performs a series of conditional checks involving calling other routines and checking if their results are zero to determine the final return value.
- `execute_multiple_comparisons`: If a global variable is zero, it calls three different routines using parameters from a3 and a4, and if any of these calls return a non-zero value, it stores that value in the location pointed to by a2 and returns it.
- `compare_and_store_results`: Calls another routine to compare values, and if the result is zero, calls a second routine to compare different values; it then stores the return value of the called routine into a location pointed to by a pointer in a2.
- `compare_and_store_result`: Calls another routine and, if the result is non-zero, calls a third routine and stores its return value at the location pointed to by a pointer in a2.
- `clear_and_restore_registers`: Sets d0 to zero and restores registers a2 through a4 from the stack.
- `increment_and_check_modulo`: Increments a global variable and, if it is not a multiple of a value from a table, calls another routine twice to update a global variable.
- `execute_and_store_result`: Pushes a pointer to a location on the stack, clears three words on the stack, loads a pointer from a global variable into a0, calls the routine at that pointer, stores the result in d0 into the location pointed to by a2, and branches to another location.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `load_values_and_branch`: Saves registers a2 through a4 to the stack, loads three constant addresses into a2, a4, and a3, moves a word from offset 380 of a structure into the address in a4, and then branches to another routine.
- `accumulate_scaled_values_2`: Calls accumulate_scaled_values and updates a word at a pointer in a3, then enters a loop that calls accumulate_scaled_values and update_global_counters.
- `process_printer_data_2`: Executes a loop that calls another routine, updates several global variables, and performs arithmetic on a global variable.
- `clear_word_and_branch`: Clears a word in a global variable and branches to another routine.
- `process_printer_data`: Performs a series of nested loops involving multiple calls to other routines, updates several global variables, and performs arithmetic on global variables based on their products and sums.
- `update_global_counters`: Increments a global variable, performs arithmetic on another global variable using a value from a table, and calls increment_and_check_modulo.
- `accumulate_and_check_limit`: Adds a word from the address in a3 to a word at the address in a4, compares the result to 300, and if the result is greater than or equal to 300, clears d0 and returns.
- `initialize_printer_parameters`: Saves registers a2 through a4 to the stack, loads three constant addresses into a2, a3, and a4, clears a 32-bit word at offset 316 of a global variable, moves a word from offset 380 of a global variable to the address in a4, and branches to another routine.
- `decrement_and_check_modulo_2`: Calls decrement_and_check_modulo and accumulate_scaled_values, then enters a loop that calls accumulate_scaled_values and decrement_and_check_modulo.
- `clear_printer_buffers`: Sets four longwords at offsets 296, 288, 292, and 284 of a structure to 0, clears a word at offset 340, and branches to another location.
- `accumulate_printer_values`: Executes a loop that calls another routine, updates a global variable by adding another global variable to it, and contains a nested loop that calls other routines and updates several global variables.
- `call_and_store_result_2`: Calls another routine, stores the result in a location pointed to by a2, and branches based on whether the result is zero.
- `decrement_and_check_modulo`: Increments a global variable, decrements another global variable, and calls increment_and_check_modulo.
- `increment_and_check_threshold`: Adds a 16-bit value from a register to a 16-bit value at a pointer in a4, then compares the result against a constant threshold.
- `update_printer_status`: Saves registers a2 through a4 to the stack, loads three constant addresses into a2, a3, and a4, clears a 32-bit word at offset 340 of a global variable, moves a word from offset 380 of a global variable to the address in a4, and branches to another routine.
- `call_accumulate_scaled_values`: Pushes two words from global variables onto the stack and calls a routine, then stores the result in d0 into the location pointed to by a3 and branches to another location.
- `initialize_local_variables_3`: Clears a word at offset 20 of a structure, copies a longword from offset 238 to offset 234, copies a longword from offset 16 to offset 276, moves a word from offset 378 to offset 378, and branches to another routine.
- `update_printer_buffer`: Calls another routine, increments a longword at a global variable by 1, compares a word at a global variable to that longword, and if the word is not equal, enters a loop that calls another routine and decrements a word at a pointer's target.
- `update_buffer_status_2`: Calls another routine, increments a longword at a global variable by 1, compares a word at a global variable to that longword, and if the word is greater than the longword, calls two other routines, stores d0 into a structure in a2, and branches to another location.
- `calculate_buffer_offset_2`: Increments a global variable and calculates a new value for another global variable using a multiplier and an offset, then calls increment_and_check_modulo.
- `update_counter_and_check_limit`: Decrements a word at the address in a3, checks if it is higher than 255, increments a word at the address in a4, compares that word to 300, and if the result is greater than or equal to 300, clears d0 and returns.
- `reset_printer_status_counters`: Clears two 16-bit values at offsets 316 and 340 of a data structure, updates a word at offset 320 of a different structure with a value from offset 380 of the first structure, and branches to another routine.
- `push_parameters_and_call`: Pushes two 16-bit values from memory onto the stack, calls another routine, stores the result in a memory location pointed to by a3, and then branches to another routine.
- `initialize_status_registers`: Clears a word at offset 322 of a structure, copies a longword from offset 302 to offset 306, copies a longword from offset 16 to offset 276, copies a word from offset 378 to offset 314, and branches to another routine.
- `decrement_printer_counter`: Clears four global variables, calls another routine, and enters a loop that calls other routines, updates a global variable, and decrements a value at a pointer until it reaches zero.
- `store_and_verify_pointer_result`: Calls another routine, stores the returned longword into a location pointed to by a2, and if the result is non-zero, reloads the longword from that same location into d0 before branching to another routine; otherwise it branches to a different routine.
- `manage_printer_loop`: Enters a loop that increments a global variable and decrements another, then performs nested checks and calls to other routines to update various pointers and values.
- `verify_and_increment_counter`: Decrements a word at a pointer in a3 and branches if the result is above 0, otherwise it increments a word at a pointer in a4, compares the value at a4 to a constant, and if it is not below that constant, clears d0 and restores registers before returning.
- `accumulate_scaled_values`: Iteratively adds a doubled value to a pointer's target until the target is no longer less than one, then subtracts a doubled second parameter from that target.
- `compare_table_values`: If a global variable is zero, it compares a byte from a table using indices derived from two parameters; otherwise, it compares a parameter against a value calculated by subtracting a global variable from 15.
- `compare_table_value`: Performs a comparison between a byte retrieved from a table using bitwise-derived indices and a provided byte.
- `negate_boolean_results`: Calls two different routines and returns the negation of the first return value based on the status of a second return value.
- `rotate_32bit_value_4`: Performs a 32-bit rotation and division-based arithmetic using a loop that calls another routine.
- `rotate_32bit_value_2`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_3`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `process_pointer_chain`: Loads a 16-bit value from a table, calls setup_pointer_chain, calls load_address_from_table, and calls branch_to_offset_2.
- `load_address_from_table`: Shifts a 16-bit value in d0 left by two bits, uses the result to index into a table at offset 14, loads the resulting address into a4, and branches to another routine.
- `branch_to_offset_2`: Performs a relative branch to another routine.
- `set_address_register`: Loads the constant 32 into a2 and branches to another routine.
- `compare_strings_and_update`: Compares two strings using a loop, and if the comparison fails, performs a series of complex operations involving a structure in a3, including calling a routine through a pointer and copying a string to a global variable.
- `manage_disk_operations`: Checks a value at offset 86 of a structure, calls another routine, and if a certain condition is met, performs a series of operations including loading a segment, calling a routine through a pointer, and copying a null-terminated string from one location to another before calling two other routines.
- `unlock_and_get_directory`: Calls a routine to unlock a resource using a pointer from a register, then calls another routine to retrieve the current directory, and returns negative one.
- `execute_callback_and_recurse`: Checks a global value, calls a function pointer from a table, and then calls itself with a value from a pointer.
- `compare_case_insensitive_bytes`: Adjusts two input bytes by subtracting 32 if they fall within the range of ASCII characters between backtick and curly brace, then returns whether they are equal.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_3`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### narrator.device

- `initialize_narrator_task_structures`: Allocates two memory blocks and populates them with pointers and values retrieved from a global variable and other routines, then links them into a linked list structure within the first allocated block before calling two final routines.
- `test_and_modify_register`: Tests if register d0 is zero, otherwise it sets d0 to negative one and branches to another routine.
- `update_device_state_2`: If a 16-bit value at offset 32 of a structure in a6 is zero, it calls two routines to update device state and sets specific bytes and a 32-bit value in a structure in a1; otherwise, it increments two counters in the structure in a6 and updates several fields in a structure in a1.
- `update_device_state`: Updates several fields in a structure, calls a function via a pointer, and if the result is zero, sets a byte and a 32-bit value in another structure; otherwise, it updates multiple fields in both structures.
- `update_device_state_fields`: The routine stores an input value into a structure, increments counters in the structure, copies a counter to another structure, writes several constant words and bytes to the second structure, and clears a flag bit in the first structure.
- `store_byte_and_reset_long_value`: Stores a byte from register d0 into a structure at offset 31 of a2 and sets a 32-bit value at offset 20 of a2 to all ones, then branches to another routine.
- `update_structure_and_branch`: Clears bit 0 of a byte at offset 30 of a structure in a1, loads a pointer from offset 20 of a structure in a1 into a3, and if bit 6 of a byte at offset 34 of a3 is set, it branches to another routine; otherwise, it checks a long value at offset 48 of a3, and if it is not zero, sets a byte at offset 31 of a1 to 248 and calls another routine.
- `manage_text_buffer`: Compares a value in a structure in a1 with another, and depending on the result, either updates a byte and calls another routine to split it into nibbles, or performs a linked-list insertion into a structure in a3.
- `update_narrator_text_buffer`: If a condition is met, it updates a byte in a structure, splits it into nibbles at specific offsets, and calls copy_byte_and_split_nibbles; otherwise, it performs a linked list insertion and modifies a global variable and a counter.
- `copy_byte_and_split_nibbles`: The routine copies a byte from one structure to another, splits it into low and high nibbles into two bytes, clears a byte in the destination structure, and calls another routine.
- `process_narrator_speech_queue`: Iterates through a linked list of speech data structures, decrementing a speech counter in a global variable and updating various parameters in a structure via a pointer.
- `decrement_narrator_speech_counter`: Decrements a byte in a global structure and, if the value reaches zero, disables interrupts by writing to the hardware register and calls another routine via a jump table.
- `decrement_and_call_if_zero`: Decrements a 16-bit value at offset 32 of a structure in a6 and, if the result is zero, calls a routine via a pointer stored at an offset from a global variable; otherwise, it calls another routine via a pointer stored at an offset from a global variable.
- `decrement_counter_and_close`: Decrements a 16-bit counter in a structure in a6 and, if the counter reaches zero, calls a function via a pointer at an offset from a global variable, calls another function via a pointer at an offset from that same global variable, and if bit 5 of a byte at offset 34 of a structure in a3 is set, calls another routine.
- `call_table_entry`: Loads a word from offset 28 of a structure in a1, and if it is between 0 and 7 inclusive, it uses the value to index into a table at offset 2666 of a0 and calls the resulting routine.
- `update_pointer_field`: Sets bit 5 of a byte at offset 34 of a structure in a3 if a word at offset 32 of that structure is not zero, then returns.
- `swap_pointers_and_return_value`: The routine calls library functions via a base pointer, swaps values between a pointer table and a data word, updates a pointer field, and returns a value from a table entry.
- `dispatch_event_handler`: Processes an event structure by checking a type field and performing various actions including setting status flags, updating a linked list of structures, and calling specific handler routines via pointers.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `modify_status_and_invoke`: Clears bit 5 of a byte at offset 34 of a structure in a0, writes 7 to a word at offset 196 of a structure in a6, sets bit 0 of a byte at offset 198 of a6, calls a routine via a pointer in a6, calls another routine via a pointer in a6, and clears a byte at offset 31 of a structure in a1.
- `set_flags_and_invoke_handler`: Sets bit 6 of a byte at offset 34 of a structure in a0, writes 6 to a word at offset 196 of a structure in a6, sets bit 0 of a byte at offset 198 of that same structure, calls a handler via a pointer at offset 164 of the structure in a6, writes 0 to a byte at offset 31 of a structure in a1, and returns.
- `set_flag_then_clear`: Sets bit 7 in a byte at offset 34 of a structure, calls two other routines, then clears bit 7 in the same byte.
- `set_flag_clear_byte_then_call`: Sets bit 7 in a byte at offset 34 of a structure, calls another routine, clears bit 7, clears a byte at offset 31 of a second structure, and calls another routine.
- `modify_global_and_interrupts`: Sets a global variable to 16384, increments a byte at offset 294 of a global variable area, and if the byte was previously less than one, sets the global variable to 49152; it also calls another routine if a pointer in a structure in a3 is not zero.
- `decrement_counter_and_set_global`: The routine decrements a byte in a global variable area and, if the previous value was less than one, writes a constant to a global variable.
- `set_interrupt_and_flag`: Saves registers, sets the INTENA register to enable interrupts, increments a byte at offset 294 of a structure in a6, and if a pointer at offset 54 of a structure in a3 does not match a1, it sets bit 5 of a byte at offset 30 of a1.
- `swap_and_compare_pointers`: Loads a pointer from a structure in a0, compares it to a pointer in a1, and if they are not equal, it sets a0 to the value previously in d0 and branches back to the start of the routine.
- `configure_interrupt_and_call_handler`: Swaps two pointers stored in a structure, updates a pointer in another structure, writes the byte value 254 to offset 31 of a structure in a1, decrements a byte at offset 302 of a structure in a6, and if the result is not negative, writes 49152 to the INTENA register and calls another routine before returning.
- `decrement_counter_and_set_global_2`: The routine decrements a byte in a structure pointed to by a register and, if the previous value was less than one, writes a constant to a global variable.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `set_byte_and_call_routine`: Writes the byte 249 to offset 31 of a structure in a1 and calls another routine.
- `clear_byte_and_call_routine`: Clears the byte at offset 31 of a structure in a1 and calls another routine.
- `clear_flags_and_call_handler`: Clears a flag bit in a structure, clears a byte in another structure, and calls a handler via a base pointer.
- `sync_narrator_speech_parameters`: Copies several 16-bit and 8-bit values from a source structure to a destination structure based on range checks, then performs arithmetic on length and offset fields.
- `update_narrator_value_data`: Compares a 16-bit value in d1 to 400; if it is not greater, stores it at offset 30 of the structure in a5.
- `update_narrator_value`: Compares a 16-bit value in d1 to 320; if it is not greater, stores it at offset 28 of a5.
- `compare_narrator_value`: Stores a 16-bit value from d1 into a global variable at offset 48 of a5, then compares a 16-bit value from offset 54 of a2 to zero or one and branches to another routine if either condition is met, otherwise it loads the constant 233 into d3 and branches to another location.
- `update_and_compare_word`: Stores a 16-bit value from register d1 into a global variable at offset 38 of a5, then compares a 16-bit value from offset 64 of a2 to 5000; if the value is 5000 or greater, it branches to another routine, otherwise it loads a constant into d3 and branches to another routine.
- `compare_and_store_word`: Compares a 16-bit value in d1 to 26464; if it is greater, it branches to another routine, otherwise it stores the value in d1 into a 16-bit word at offset 22488 of a6.
- `scan_string_for_delimiters`: Scans a string backwards from a specified offset to find a delimiter such as a comma, hyphen, or space, updates a length field and a pointer.
- `parse_narrator_control_stream_3`: Scans a string backwards from a specified offset to find a delimiter such as a comma, hyphen, or space, updates a length field and a pointer in a structure, and then calls another routine.
- `move_and_branch_4`: Moves a longword from offset 36 of a structure in a1 to offset 32 of the same structure and branches to another location.
- `reset_field_and_call_routine`: Clears a longword at offset 32 of a structure in a1, writes the byte from d0 to offset 31 of that structure, and calls another routine.
- `parse_text_token`: Scans a string backwards from a specified offset to find a delimiter such as a comma, hyphen, or space.
- `process_list_and_clear`: Loads a longword from offset 150 of a structure in a5, performs arithmetic on a word derived from it, and if a byte at an offset determined by the result matches 32, it stores the result in offset 156 of the structure in a5, clears the longword at offset 150 of a5, and branches to another location; otherwise, it branches to another location.
- `shift_and_clear_value`: Loads a word from offset 150 of a structure in a5, shifts it right by one, stores the result in offset 156 of that structure, and clears the longword at offset 150.
- `initialize_speech_buffer`: Clears several fields in two structures, calls two routines via pointers, and if a condition is met, allocates a new structure and populates it with data.
- `update_speech_status`: Loads a longword from offset 24 of a structure in a1, stores the lower byte in offset 67 of a structure in a2, calls a routine via a pointer in a6, and if d0 is zero, sets d3 to negative 2 and branches to another routine; otherwise it branches to another routine.
- `copy_speech_buffer`: Sets several fields in a structure in a3, calls two functions via pointers in a6, and if a byte at offset 67 of a structure in a2 points to a valid routine, it copies data from a table in a3 to a buffer pointed to by a structure in a3 using nested loops.
- `calculate_and_call_library`: Writes a byte from d0 to offset 68 of a structure in a2, calculates a value based on d0, calls a library function using a pointer from a6, and if the result in d0 is not non-zero, writes 254 to d0 before returning.
- `copy_word_table_to_buffer`: Stores a pointer in a global variable and copies 34 words from a table in a global variable to a buffer pointed to by an argument, writing each word twice per iteration, repeating for a count derived from a byte in a global variable.
- `perform_bit_manipulations`: Sets bit 1 of register d1, tests bit 0 of register d2, and performs several bitwise AND operations on register d3 before testing bit 1 of register d4.
- `handle_io_and_free_resources`: Writes a byte and a word to a global variable, calls a routine via a global variable, conditionally calls two other routines via the same global variable, clears a word in the global variable and returns a combined value.
- `check_narrator_value`: Tests a word at offset 206 of a structure in A5; if it is not greater than 0, it sets D3 to 0 and returns, otherwise it returns.
- `parse_narrator_control_stream`: Parses a stream of bytes from a pointer in a0, performing lookups in a table to update three separate buffers at offsets 232, 744, and 1256 of a structure in a5.
- `modify_d1_register`: Negates a 16-bit value in d1, adds 111 to it, compares d0 to 12288, and if d0 is not less than 12288 and not equal to 12288, adds the value in d2 to a0 before branching.
- `parse_encoded_token_stream`: Iterates through a buffer to decode a stream of tokens by comparing bytes against a lookup table, updating a multi-part structure containing an index, data, and attribute byte.
- `parse_narrator_token_stream`: Parses a stream of encoded tokens by reading bytes from a buffer and updating a multi-part structure containing index, data, and attribute arrays.
- `set_bit_if_not_five`: Compares a byte in d1 to 5; if they are not equal, it sets bit 5 in a byte at a location determined by adding a6 and d3, then branches to another routine.
- `set_bit_conditionally`: If the byte in d1 is not 6, it sets bit 4 in a byte at a location determined by a6 and d3, then branches to another routine.
- `write_byte_and_update_index`: Writes a byte from register d1 to a memory location at the offset specified by d3 in a2, increments d3, and if d1 is not one or two, it checks if d3 is less than 512; if so, it branches to another routine, otherwise it subtracts a longword from a structure in a5 from a0, stores the result in a structure in a5, and returns.
- `clear_buffer_elements`: Subtracts a longword from offset 146 of a structure in a5 from a0, compares the result to a 16-bit constant at offset 156 of the structure in a5, and if the result is greater than or equal to that constant, branches to another routine; otherwise it clears a byte at offset d3 of a structure in a3 and a byte at offset d3 of a structure in a6, then branches to another routine.
- `increment_longword_in_structure`: Copies a longword from a register into a longword located at an offset of 66 from a pointer in a5, increments the value by 1, and returns.
- `decrement_index_on_condition`: Loads a byte from an offset in a2 determined by d3, and if the byte is not zero, it decrements the word in d3 and branches to another location.
- `update_structure_counters`: Checks a bit in a value retrieved from a table, and if a condition is met, writes 4, 0, and 0 to three consecutive bytes at an offset determined by a value in d3, then increments that value.
- `store_word_and_set_ccr`: Stores a word from register d3 into a structure at offset 154 of the structure in a5 and sets the carry and overflow flags.
- `update_status_from_table`: Iterates through a null-terminated string in a structure in a5, and for each character, uses its value to index into a global table to check specific bits, subsequently setting bit 4 of bytes at various offsets in the string based on the table values and character matches.
- `modify_byte_conditionally`: Performs a bitwise AND on d2 and, if the result is not zero, sets bit 4 of a byte at offset -3 of a structure in a0 before returning.
- `filter_and_modify_byte`: Checks if a value at a specific offset in a structure matches thirty and, if so, sets bit four of a byte at an offset of three before the structure pointer.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `apply_text_attribute_transformations`: Parses a source buffer to identify control sequences and applies bitwise transformations to a destination buffer and a secondary buffer.
- `process_text_stream_attributes`: Iterates through a source buffer in a0, reading bytes to determine control sequences and applying bitwise transformations to a destination buffer in a1.
- `process_narrator_text_stream`: Parses a stream of bytes from a source buffer, using a lookup table to determine formatting actions.
- `calculate_and_branch_offset`: Subtracts a word in d2 from a word in d0, decrements the result by one, and branches to another location.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `patch_buffer_with_conditional_logic`: Iterates through a source buffer, comparing bytes against a target table; if matches are found and specific bit flags are set in the source data, it calls another routine to determine an offset and writes bytes from the source buffer into the target memory at that offset.
- `compare_and_patch_buffer`: Compares bytes from a source buffer against a target buffer using a series of conditional checks and calls another routine.
- `check_byte_and_call`: Increments d1 by one, loads a byte from offset 6 of the structure in a0, and if it is not 255, calls another routine; otherwise it returns.
- `test_bit_and_branch`: Tests bit 5 of a byte at offset 3 from the pointer in a0 and branches to another routine or a different location based on the result.
- `test_bit_with_table_lookup`: Loops reading a byte from a buffer and a word from a table indexed by a word, modifies the word based on bits in the byte, and exits when a specific bit is set or a termination condition is met.
- `check_word_limit`: Loads a word from offset 154 of a structure in a5, compares it to 512, and if it is less than 512, sets the carry flag and branches to another routine; otherwise, it branches to shift_bytes_and_clear_fields.
- `shift_bytes_and_clear_fields`: Increments a word counter in a structure and shifts bytes backward in three buffers by a computed count, then writes a byte from a register and two zero bytes to the new ends of the buffers.
- `call_multiple_subroutines`: Calls seven different subroutines in sequence, clears register d0, and then returns.
- `clear_byte_at_offset_2`: Loads a pointer from offset 40 of a structure in a5 into a6, compares the byte at that pointer to 255, and if they are not equal, loads the pointer from offset 40 of the structure in a5 again and writes 160 to offset 7 of that pointer before returning.
- `clear_byte_and_increment`: Clears a byte at offset 7 of a structure in a6, increments the pointer in a6 by 8, and branches to another routine.
- `check_data_byte`: Loads three pointers from offsets 232, 744, and 744 of a structure in a5 into a0, a1, and a2, loads a pointer from a global variable into a4, and returns if a byte at a0 is not 255.
- `decode_compressed_data_3`: Processes a data stream by reading values from a table and updating three pointers, performing bitwise operations on a byte at the address in a1, and calls two other routines.
- `decode_compressed_data_2`: Processes a stream of bytes by reading values from a table and updating three pointers, performing bitwise operations on a byte at the address.
- `decode_compressed_data`: Processes a stream of bytes by reading values from a table and updating three pointers, performing bitwise operations on a byte at the address in a2, and handling specific byte markers to modify data in a1 and a2.
- `test_and_set_bit`: Tests bit 6 of d1 and, if not set, branches to another location; otherwise it sets bit 6 of the byte at the address in a2 and branches to another location.
- `set_bit_and_branch_2`: Tests bit 5 of d1, and if it is set, sets bit 5 of the byte at the address in a2 before branching to another location; otherwise it branches to another location.
- `set_bit_and_increment`: Sets bit 2 of a byte at the address in a2, increments the pointer in a2 by 1, and branches to another routine.
- `increment_three_pointers`: Increments the pointers in a0, a1, and a2 by 1 and branches to another routine.
- `calculate_offset_from_byte`: Loads a byte from an offset of 1256 in the structure in a5 into d2, compares it to 255, and if not equal, masks d2 with 63 and adds the result to d0 before branching to another location.
- `allocate_and_store_pointer`: Stores a long value from d0 into offset 58 of a structure in a5, calculates a new address by shifting d0 left by three and adding eight, stores this new address into offset 54 of the structure in a5, calls a routine using a pointer from a global variable, and if d0 is not zero, sets the two most significant bits of the status register before returning.
- `update_structure_from_call`: Stores a long value from d0 into offset 40 of a structure in a5, checks a byte at offset 219 of that structure, and if it is not zero, loads a long value from offset 58 into d0, calls another routine, and if d0 is not zero, stores d0 into offset 44 of the structure before returning.
- `expand_narrator_metadata_stream`: Iterates through a data stream, using bytes to index into a global table to retrieve multi-byte sequences and metadata, then writes these expanded records into a destination buffer.
- `expand_narrator_data_stream`: Iterates through a data stream, using bytes to index into a global table to retrieve multi-byte sequences and metadata, then writes these expanded values into a destination buffer.
- `parse_narrator_data_stream`: Reads a stream of data from a buffer, using bytes to index into a global table to retrieve multi-byte sequences and metadata.
- `interpolate_buffer_data_3`: Performs a loop that calculates interpolated values for a buffer based on data from a table and a global variable area.
- `check_and_increment_pointer`: Shifts d0 left by 2, loads a longword from a table using the result as an offset, and if bit 15 and bit 11 of that longword are set, increments a0 and branches to another routine; otherwise it branches to another location.
- `interpolate_buffer_data_2`: Performs a linear interpolation between two values at a memory location and stores the result back to that location, repeating this process.
- `interpolate_buffer_data`: Performs a linear interpolation between two values at a memory location and stores the result back to that location, repeating this process for a set number of iterations.
- `increment_a6_and_branch`: Increments a6 by 1 and branches to another location.
- `process_narrator_stream_data`: Iterates through a data stream to perform calculations based on values in a table, then writes results into a destination buffer.
- `loop_stream_data`: Copies a byte from a memory location at offset d7 to d5, loads a byte from a pointer in a1 and masks it with 63, shifts d7 left by two, loads a longword from a table at offset d7 into d1, and if bit 10 of d1 is set or bit 11 is clear, adds 1 to a0 and loops back to the start.
- `process_stream_data_table`: Iterates through a data stream to compare values against a lookup table, performing arithmetic on byte values and clearing or initializing segments of a memory structure based on the results.
- `compare_stream_values`: Loads a byte from a0 into d7, loads a byte from a table at a4 plus the offset in d7 into d2, compares d5 and d6, and if d6 is not greater than or equal to d5, loads a byte from a0 into d7 and a byte from the table into d3 before branching.
- `process_narrator_data_stream_3`: Iterates through a data stream to perform arithmetic comparisons between values in a table and a secondary data source, updating a destination table with calculated results and clearing specific memory ranges based on the comparison outcomes.
- `process_narrator_data_stream_2`: Iterates through a data stream to perform arithmetic comparisons and updates a table of values.
- `calculate_byte_difference_3`: Subtracts d3 and d2 from d4, which was loaded from d0, sets d1 to zero, then moves the value of d2 into d1 and branches to another routine.
- `increment_pointer_and_loop_2`: Clears a word in a6 and a byte at offset 2 of a6, then checks bit 15 of d0 to decide whether to branch; otherwise it copies d6 to offsets 3 and 4 of a6, increments a6 by 8, and loops using d1 as a counter before setting d1 from d4 and branching.
- `increment_pointer_and_loop`: Increments a pointer in a6 by 8, then enters a loop that decrements d1 and branches to another location until d1 reaches zero.
- `clear_and_copy_data`: Clears a 16-bit word at the address in a6 and a byte at offset 2 of the address in a6, then enters a loop that checks bit 31 of d5 to conditionally copies a byte and a word from d6 to offsets 3 and 4 of the address in a6 before incrementing a6 by 8 and decrementing d1.
- `clear_registers_and_branch_2`: Sets d1, d4, and d3 to zero using d0 and then branches to another routine.
- `filter_and_copy_triplets`: Iterates through three parallel buffers, checking a global table using a byte from the first buffer as an index; if a specific bit is not set in the table entry, it copies the byte from each of the three buffers to a destination and continues until it encounters a byte of 255, at which point it writes 255 to the current position in all three destination buffers.
- `write_byte_to_three_pointers`: Writes the same byte from a register to the memory locations pointed to by three registers.
- `call_subroutines`: Calls two different subroutines and then returns.
- `mask_stream_byte`: Compares a byte from a pointer at offset 616 of the structure in a5 to 255 and, if not equal, performs a bitwise AND on that byte with 240 and increments the pointer, then branches to another routine.
- `check_stream_terminator`: Loads two pointers from offsets 232 and 744 of a structure in a5, reads a byte from the first pointer, and returns if the byte is 255; otherwise it branches to another routine.
- `process_data_stream`: Iterates through a data stream using a0 and a1, performing lookups in a table and setting various bits in the memory pointed to by a1 and a2 based on the values read and specific bit flags.
- `process_linked_list_2`: Sets bit 0 of a byte at offset -1 of a1, then checks bit 7 of d1 to decide whether to branch; otherwise it compares a byte from a0 to decimal 24 and 23, and if neither matches, sets bit 1 of the byte at a1 and moves a1 to a3.
- `process_linked_list`: Iterates through a list of bytes, checking values against a table to set specific bits in a byte at the address in a1, and calls set_bit_and_branch when certain conditions are met.
- `set_bit_and_branch`: Sets bit 2 of the byte at the address in a3 and branches to another location.
- `set_multiple_bits`: Sets bit 0 of the byte at offset -2 of the address in a1, bit 1 of the byte at offset -1 of the address in a1, and bit 2 of the byte at offset -1 of the address in a1, then branches to another location.
- `decode_compressed_speech_stream`: Decodes a bitstream of compressed data into a table of values by performing divisions of a pre-calculated constant by various byte values extracted from the stream.
- `calculate_bitstream_offset`: Masks a byte in d0, tests bit 0 of a byte at the address in a2 and increments a2, and if the bit was set, shifts d0 left by 3 and adds it to a6 before branching to another routine; otherwise it continues to the next instruction.
- `decode_variable_bitrate_audio_stream`: Decodes a bitstream using a variable-length encoding scheme and writes calculated byte values to a destination buffer.
- `parse_encoded_text_stream_2`: Loads a byte from a1 and increments it, masks the byte with decimal 63, checks bit 1 of a byte at a2 and increments a2, and if bit 1 is set, shifts the byte left by 3, adds it to d1, and loops.
- `parse_encoded_text_stream`: Iterates through a data stream to decode variable-length bit-packed values, performing divisions of a longword from a structure by these decoded values and storing the results into a destination buffer.
- `calculate_division_result`: Swaps the high and low words of d5, adds a word from d6 to d5, divides the word in d5 into the longword at offset 32 of the address in a5, stores the resulting byte at offset -1 of the address in a6, decrements a1 and a2 by 1, and branches to another location.
- `update_table_with_division`: Loads a pointer from a structure, divides a long value in d7 by 110, and either writes the result to a table or branches to another routine based on a comparison.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `parse_narrator_control_stream_2`: Iterates through a data stream using three pointers, performing bitwise tests on values fetched from a table based on the current byte to decide whether to call another routine with updated pointers.
- `test_and_branch_on_bits`: Shifts register d5 left by two, loads a word from a table using the result as an offset, and then performs bit tests on that word to decide whether to branch to specific locations or to another routine.
- `increment_pointers_and_branch`: Increments a1, a0, and a3 by 1 and branches to another location.
- `process_narrator_text_stream_3`: Decodes a stream of bytes by performing look-up table transformations and arithmetic adjustments to calculate new character values, then branches based on bitwise flags in d7 and d3.
- `test_bits_and_branch`: Tests bit 12 of d7 and branches if it is clear, then tests bit 9 of d7 and branches if it is clear, otherwise it loads 38 into d1, calls another routine, and branches.
- `test_bit_and_call`: Tests bit 10 of d7 and, if set, loads 38 into d1, calls another routine, and branches to another location; otherwise it returns.
- `test_bits_and_call`: Tests bit 16 of d7 and, if set, tests bit 2 of the byte at offset 1 of a3; if that bit is set, it loads 27 into d1, calls a subroutine, and branches to another location; otherwise it returns.
- `check_bit_and_call_decoder`: Tests bit 11 of a register and, if set, loads the value 22 into a register, calls decode_narrator_text_stream_4, and then branches to another routine; otherwise, it returns.
- `decode_narrator_text_stream_4`: Decodes a stream of bytes by performing look-up table transformations and arithmetic adjustments to calculate new character values, which are then stored into a destination buffer.
- `test_status_bits`: Tests bit 1 of d6 and bit 1 of d7, and if both are set, loads 16 into d1 and calls another routine before branching to a different location.
- `decode_narrator_text_stream_2`: Sets d1 to decimal 22, calls another routine, and then branches.
- `decode_narrator_text_stream`: Reads a stream of encoded bytes from a source pointer, performs bitwise transformations and arithmetic adjustments based on look-up table values, and writes the resulting characters to a destination buffer until a termination byte is encountered.
- `compute_scaled_shifted_product`: The routine multiplies the low word of an input register by a word input and returns a combined value formed from the high word of the product and the low word shifted right by five bits after adding a constant.
- `initialize_and_clear_buffers`: The routine stores several offsets relative to a base pointer into a global structure, increments a few of those pointers, clears a set of word fields, then zeroes six blocks of memory by iterating a fixed count.
- `increment_and_call_handlers`: Increments a 16-bit word at offset 136 of a structure in a5, and if the result is not zero and the overflow flag is not set, calls four subroutines before returning.
- `process_compressed_data_2`: Processes a compressed data stream by reading bytes from two pointers, performing calculations involving a table lookup and bitwise operations, and writing results to a destination buffer while updating various counters and indices in a structure.
- `process_compressed_data`: Checks bit 7 of a byte in d1 and bit 5 or 7 of a byte at an offset from a4, then performs bitwise masking on d1 and conditionally increments a0 and a2 before loading a new byte into d1 from a memory location pointed to by a1.
- `decode_compressed_stream`: Decodes a stream of data by performing bitwise shifts and masking to expand compressed values into a destination buffer.
- `set_flag_and_store_indices`: The routine ORs a constant into a byte addressed by a base pointer and an index, then stores two short values derived from an index and a conditional copy into fields of a global structure.
- `process_narrator_control_stream`: Iterates through three parallel streams of data—a control byte stream, a character stream, and a flag stream—to modify a target buffer based on specific control codes.
- `process_bit_test_and_modify`: Copies a word from d0 to d3, shifts d3 left by 2, loads a long from a table at offset d3 using a3, tests bit 13 of d3, and if bit 13 is not set, checks if d1 is positive; otherwise it increments d4, tests bit 5 of d2, and if bit 5 is clear, stores the value 2 at offset -1 of a structure in a6 before branching to another location.
- `modify_byte_via_mask`: Checks bit 4 of a byte at an offset from a pointer in a6, and if the resulting nibble is between 1 and 254, it performs a bitwise OR with 14 on that byte; otherwise it returns.
- `decrement_and_store_pointers`: Decrements three pointers in a0, a1, and a2 by 1, stores them at offsets 124, 132, and 128 of a structure in a5, and branches to another location.
- `update_memory_byte`: Performs a bitwise OR on a byte at an offset from a4 using d7, moves d0 to d2, and compares a byte at an offset from a6 with decimal 2 or 14 to determine whether to set d1 to 255, set d7 to 16, and perform another bitwise OR on the byte at the offset from a4.
- `update_byte_conditionally`: Checks bit 5 of a byte at an offset from a pointer in a4, and if that bit is not set, it increments d2 and, if d2 is non-negative and less than d4, clears a byte at an offset from a6.
- `swap_and_clear_byte_flags`: Swaps a byte between two positions in a table addressed by a base pointer and an index, clears the source byte when the positions differ, and then decrements an index.
- `prepare_set_bits_loop`: Loads a pointer from an offset of 112 from a5 into a1, moves d4 into d0, and branches to another routine.
- `set_bits_in_loop`: The routine iterates through a range of memory defined by a base pointer and a count, setting the lowest bit of each byte.
- `or_mask_into_buffer_with_early_exit`: The routine selects a mask based on a control byte, then ORs that mask into a series of bytes in one buffer while checking a flag in another buffer and returning early if the flag is set.
- `set_tile_and_mask`: The routine updates a byte in a screen buffer using a mask derived from a map byte, and optionally masks and sets a flag in another byte based on a scan leftwards in a second buffer.
- `accumulate_table_entries`: Loads a word from offset 140 of a structure in a5, checks two short values at offsets 48 and 138, and then conditionally calls another routine several times before adding the loaded word to eight consecutive longwords in a table pointed to by offset 92 of the structure.
- `clamp_and_store_char`: Calculates a value based on d6 and d7, clamps the result between 125 and 164, and stores the resulting byte at an address formed by adding d5 to a pointer found at an offset of 96 from a5.
- `clamp_value_and_store_char`: The routine clamps a short input to a lower and upper bound and stores the resulting byte at an address formed from a base pointer and an index.
- `adjust_table_values_loop_2`: Calculates a scaled difference between values in two tables and updates a destination table through a forward loop, then performs a backward loop to apply a weighted absolute difference between current and previous entries, and finally applies a final scaling adjustment to the destination table entries.
- `adjust_table_values_loop`: Computes an absolute difference between two table entries, scales it, and adds the result to a destination byte, then scans backwards through a range using flag bits to locate entries.
- `compute_scaled_char_values_2`: Iterates through a range using a counter, performing scaled arithmetic on bytes from three different memory locations and storing the results into two destination buffers based on bit flags.
- `scale_and_add_byte`: Adds 64 to a word in d2, shifts it right by 7 bits, and adds the result to a byte in a table at a0 plus an offset in d0.
- `compute_scaled_char_values`: Loops over a range in d0, adding a scaled value derived from d2 to a destination table, and then copies a byte from a source table to a destination table while subtracting 110 from it.
- `load_structure_pointers_2`: Loads three values from offsets 104, 112, and 158 of a structure in a5 into registers a0, a1, and d1, then moves d1 into d0 and branches to another location.
- `process_audio_sample_buffer`: Iterates through a buffer to perform digital signal processing on audio samples by applying gain coefficients and filtering logic to bytes in two separate tables.
- `modify_table_value`: Adds 64 to d2, shifts d2 right by 7, adds the result to a byte in a table at a3 with an offset in d0, then loads a byte from a3 with an offset in d1 into d2, extends d2 to a word, multiplies d2 by 65485, and if d2 is positive, subtracts 64 before branching to another location.
- `modify_and_branch_on_value`: Adds 64 to d2, shifts it right by 7 bits, adds the result to a byte in a table at a3 plus an offset in d1, then performs arithmetic on a byte loaded from a6 plus an offset in d0 and branches based on the sign of the result.
- `calculate_byte_difference_2`: Adds 64 to d2, shifts it right by 7 bits, and adds the result to a byte in a table at a6 plus an offset in d0, then loads a byte from a6 plus an offset in d1, subtracts 110, multiplies it by decimal 26, and conditionally subtracts 64 before branching.
- `calculate_byte_difference`: Adds 64 to a word in d2, shifts it right by 7 bits, and adds the result to a byte in a table at a6 plus an offset in d1.
- `add_byte_and_branch`: Adds a byte from d2 to a byte at an offset from a2 and then branches to another routine.
- `apply_audio_gain_and_filter`: Applies variable gain coefficients to bytes in two different tables using arithmetic shifts and sign-correction, then performs a loop that modifies values in a third table based on bitmask checks and comparisons with a running maximum value found in a fourth table.
- `process_audio_buffer_samples_2`: Adds 64 to d2, shifts it right by 7 bits, and adds the result to a byte in a table at a3 plus an offset in d0, then loads a byte from a3 plus an offset in d1, extends it to a word, multiplies it by d7, and conditionally subtracts 64 before branching.
- `process_audio_buffer_samples`: Iterates through a data buffer to perform arithmetic scaling and bitwise shifts on bytes, applying conditional offsets and comparisons based on values in a control table.
- `transform_audio_buffer_data`: Iterates through a data buffer using a loop to perform arithmetic operations on bytes, applying scaling factors and bitwise shifts based on flags found in a secondary table.
- `apply_audio_filter_coefficients`: Iterates through a buffer to perform digital signal processing by applying gain coefficients, bitwise masks, and arithmetic adjustments to audio samples.
- `transform_byte_value`: Loads a byte from a table using a pointer in a2 and an offset in d0, extends it to a word, multiplies it by 102, and then either subtracts 64 or branches to another routine based on the sign of the result.
- `update_tile_heights`: Adds 64 to d1, shifts it right by 7, adds the result to a byte at the address in a3 plus the offset in d0, loads a byte from a table at offset d2 into d7, and branches to another location.
- `adjust_tile_heights_with_mask`: The routine iterates over a byte range using a mask in a global variable and a table pointed to by a register, updates a byte in a destination buffer from a source buffer with a scaled difference based on a running maximum, and writes an accumulated scaled value to another buffer.
- `initialize_scaling_parameters`: Loads two pointers from offsets 68 and 78 of a structure in a5 into a0 and a2, clears d1, moves the value of d4 into d0, and branches to another routine.
- `modify_buffer_with_scaled_values`: The routine iterates backwards through a buffer, checking if the high nibble of a byte is non-zero, then calculates a scaled value based on the byte's content and a sign bit, and adds the result to another buffer.
- `scale_buffer_by_signed_factor`: Adds 64 to d1, shifts it right by 7, adds the result to a byte at the address in a2 plus the offset in d0, and loops using d0 as a counter until it returns.
- `process_audio_buffer_transformations`: Iterates through a buffer backwards to subtract values from bytes at specific offsets, then performs a second pass that identifies segments based on bitmask checks and applies differential adjustments and bitwise shifts to multiple destination buffers.
- `update_buffer_with_offset`: The routine iterates through a buffer, copying a byte from one location to another, subtracting a value from it, and then adding five to a byte at a location pointed to by a register, before branching to another routine.
- `process_audio_buffer_stream`: Iterates through a buffer to copy bytes to multiple destinations while applying bitwise transformations based on a mask.
- `copy_and_shift_nibble`: The routine iterates through a buffer, copying a byte to two different locations, and then adds a bit-shifted version of a masked nibble from a fourth buffer to a byte in a third buffer.
- `load_address_and_call_2`: Loads the address stored at a longword at offset 10708 into a4, calls another routine, and branches to another location.
- `load_address_and_call`: Loads a long value into a4, calls another routine, and then branches to another location.
- `load_pointers_and_constants`: Loads a long from offset 108 of a structure in a5 into a4, loads the byte 110 into d6, and moves the word in d5 to d0 before branching to another location.
- `transform_and_scale_audio_buffer`: Iterates backward through a buffer to perform bitwise operations and arithmetic on bytes, then checks a flag to either scale a sequence of bytes by a calculated average or find a maximum value in a buffer to derive a single character value.
- `process_buffer_data`: Decrements a counter in d0 while checking a bit in a buffer, then performs arithmetic and copies data between buffers using a loop that modifies values based on a byte in d2.
- `process_buffer_and_calculate_max`: Copies bytes from one buffer to another while subtracting a byte from d7, then if a byte in d2 is a backspace, it finds the maximum byte in a second buffer, calculates a scaled value, and writes it to two destinations and a third location.
- `initialize_registers_and_branch`: The routine clears registers d7 and d1, moves a word from d4 to d0, and then branches to another location.
- `accumulate_bytes_and_write_results`: The routine accumulates bytes from a source area into a register, keeps a running maximum, derives a scaled character value, and writes that value and a derived value to several destination buffers with a conditional copy.
- `decrement_structure_pointers`: Decrements three pointers stored at offsets 124, 128, and 132 of a structure in a5 by one, sets d2 to zero, moves d4 into d0, and branches to another location.
- `apply_audio_scaling_modifiers`: Iterates through a buffer using a loop, applying fixed-point multiplication and additive scaling to byte values based on bitwise flags and lookups in a table.
- `calculate_scaled_value_2`: Loads a long from offset 96 of a structure in a5 into a6, loads a byte from a6 with an offset in d0 into d1, subtracts 110 from d1, multiplies d1 by 26, and if d1 is positive, subtracts 64 before branching to another location.
- `scale_audio_buffer_samples`: Iterates through a buffer using a loop to perform fixed-point multiplication and scaling on byte values, updating multiple destination buffers by adding and subtracting calculated fractional components.
- `update_audio_buffer_with_scaled_values`: Iterates through a buffer to perform fixed-point arithmetic, scaling bytes by constants such as twenty-six or twenty-six minus one hundred and ten, and updates multiple destination buffers by adding or subtracting the resulting high-order bits.
- `compare_and_set_d7`: Decrements d7 by 1, compares it to d0, and if they are equal it branches to another location; otherwise it sets d7 to 64 and branches to another location.
- `set_d7_and_branch`: Sets d7 to 86 and branches to another location.
- `calculate_scaled_value`: The routine loads two pointers from a table, reads a byte from the first pointer, multiplies it by twenty-six, and then either subtracts sixty-four or branches if the result is positive.
- `apply_scaled_byte_adjustments`: Adds 64 to register d2, performs an arithmetic shift right by 7, then adds the resulting byte to a location at offset d0 in a structure in a6 and subtracts it from a location at offset d0 in a3, looping until d0 reaches zero.
- `branch_to_next_routine`: The routine branches to another location.
- `copy_and_adjust_bytes_with_feedback`: The routine copies bytes between buffers, updates a feedback value by multiplying it with a table entry and shifting, subtracts the feedback from a byte, and writes adjusted bytes to two output areas.
- `check_narrator_flag_and_dispatch`: Calls a series of other routines and then checks a byte at offset 219 from a pointer in a5; if the byte is non-zero it calls another routine.
- `call_fetch_and_check_byte_three_times`: Loads a pointer from an offset of 40 in a5, adds 3, 4, and 5 to that pointer respectively, and calls another routine after each addition.
- `interpolate_and_store_bytes`: The routine reads two bytes from a buffer, computes a scaled difference based on an index, accumulates the result, and stores successive byte results into the buffer at regular intervals.
- `fetch_and_check_byte`: Sets register d1 to 8, loads a byte from a table at offset d1 in a0 into d6, and returns if d6 is equal to 255; otherwise it branches to another routine.
- `check_conditions_and_increment`: Compares d6 to 254 and branches if equal; otherwise it compares d1 to 8 and if not equal branches to another location; if d1 is 8, it adds 8 to a0 and branches to another location.
- `call_interpolation_and_branch`: The routine calls interpolate_and_store_bytes and then branches to another location.
- `test_table_byte`: Sets register d1 to 8, tests a byte from a table at offset d1 in a0, and returns if that byte is both zero and 255; otherwise it branches to another location.
- `increment_pointer_if_eight`: Compares d1 to 8 and if not equal branches to another location; if d1 is 8, it adds 8 to a0 and branches to another location.
- `call_interpolation_and_branch_alternate`: The routine calls interpolate_and_store_bytes and then branches to a different location.
- `process_narrator_buffer`: Iterates through a buffer, using values from a table to conditionally update a pointer and modify data at that pointer, including a loop that writes bytes from a table to a destination.
- `update_buffer_and_data`: Performs a loop that iterates through a buffer, conditionally increments bytes in the buffer, and uses values from a table to copy bytes from one location to another.
- `offset_pointer_and_branch`: The routine shifts d0 left by three bits, adds the result to a0, and then branches to another location.
- `modify_d3_value_based_on_shift`: Shifts d3 right by 2 bits, compares the result to 42, and if equal, sets d3 to 96 before branching to another location; otherwise it branches to another location.
- `update_d3_value`: Compares d3 to 43 and, if equal, sets d3 to 97 before branching to another location; otherwise it branches to another location.
- `modify_d3_value`: Compares d3 to 44 and if not equal branches to another location; if d3 is 44, it sets d3 to 98 and branches to another location.
- `copy_bytes_from_table`: Copies six bytes from a table located at an offset determined by a byte in a register to a destination pointed to by a0, then increments the destination pointer by two for each byte copied.
- `initialize_address_pointers`: Loads four long pointers into a0, a1, a2, and a4 from offsets 232, 1256, 744, and a constant address, moves a long into a6 from offset 40 of a structure in a5, and sets d6 to 12288, d4 to 240, and d5 to 255 before branching to another location.
- `set_bit_and_compare_byte`: Sets bit 31 of d4, clears d0 and d2, loads a byte from the address in a0 into d0 and increments a0, then compares d0 to d5; if they are not equal, it branches to another location, otherwise it returns.
- `update_narrator_stream_structure`: Iterates through a data stream to perform conditional memory writes to a structure pointed to by a6.
- `process_narrator_data_stream`: Iterates through a data stream, performing bitwise operations and conditional memory writes to a structure pointed to by a6 based on values read from a0 and a1.
- `calculate_table_offset`: Shifts the value in register d2 left by 3 bits, adds the result to the address in a6, and branches to another routine.
- `replace_bytes_from_lookup`: Loops through a buffer starting at an offset of 3 from a pointer in a5, replacing bytes with values from a global table based on the byte value, or skipping 8 bytes if the byte is 254, and terminates when a byte is 255.
- `copy_bytes_with_lookup`: Copies bytes from a source pointed to by a0 to a destination using a lookup table in a1, then loops back to the start of the routine.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `compare_scaled_byte`: Loads a byte from a structure at offset 6 times 8 of a6, compares it to 255, and branches to another routine if they are not equal; otherwise it returns.
- `update_scaled_value`: Adds a value from d3 to d0, performs a bitwise shift, stores the result in a structure at offset 18 of a6, and loops back to the start of the routine.
- `check_scaled_byte`: Loads a byte from a structure at offset 48 of a6, compares it to 255, and branches to another routine if they are not equal; otherwise it returns.
- `process_data_table`: Iterates through a table, performing arithmetic on a value in d0 and updating a byte at offset 24 of a structure in a6 every 7 iterations, then calls another routine.
- `load_and_check_stream_byte`: Loads four pointers into a0, a2, a4, and a6 from various offsets and a global address, clears d2 and d0, loads a byte from the address in a0 into d0 and increments a0, then compares d0 to 255; if not equal, it branches to another location, otherwise it returns.
- `decode_narrator_stream`: Iterates through a data stream, decoding characters by updating a byte at offset 7 of a structure in a6 based on specific bit patterns and character values, and returns when a byte of 255 is encountered.
- `add_constant_to_structure`: Decrements d2, adds 10 to a byte at offset 7 of a structure in a6, and loops using d2 as a counter before branching to another location.
- `branch_with_constant_2`: Loads the constant 250 into d3 and branches to another routine.
- `load_negative_constant_and_branch`: Loads the value negative 6 in register d3 and branches to a location within another routine.
- `load_negative_constant_and_branch_2`: Loads the value negative 6 from a constant into register d3 and performs a short branch to another routine.
- `store_fixed_byte_loop`: Decrements d2, stores the byte value 230 at offset 7 of a structure in a6, and loops using d2 as a counter before branching to another location.
- `modify_structure_byte`: Decrements d2, reads a byte from a6, subtracts 43, shifts it right by 2, adds the result to a byte at offset 7 of a structure in a6, and loops using d2 as a counter before branching to another location.
- `decode_nibble_checksum_bytes`: Reads a pointer and a count from a structure pointed to by A5 and processes a byte array in two passes.
- `wait_for_task_signals`: Calls two functions via pointers in a global variable to store their results in a structure at offset 308 and offset 31, then enters a loop that calls another routine and a third function via pointers in the global variable.
- `manage_device_status`: Enters a loop that checks bit 6 of a byte at offset 34 of a structure in a2; if bit 6 is set, it returns, otherwise it sets bit 0 of that byte, calls another routine, and if that routine returns 0, clears bits 0 and 1 of the byte and returns; otherwise it updates a value in a structure in a3 and calls itself.
- `clear_status_bits`: Clears bit 1 and bit 0 of a byte at offset 34 of a structure pointed to by A2.
- `prepare_speech_iorb_and_display`: Initializes a speech I/O request block by setting its command, buffer, and timing parameters, then populates a bitmask table based on input bytes and calls update_narrator_display_state.
- `update_narrator_display_state`: Copies a data stream into a buffer, populates a bitmask table based on input bytes, and updates a structure with character data and timing values.
- `decode_narrator_text_stream_5`: Iterates through a data stream to decode characters by performing bitwise operations and lookups in a table, writing the resulting bytes to a buffer and updating a counter in a structure.
- `process_narrator_text_stream_2`: Iterates through a data stream to decode characters, updating a buffer and managing a counter in a structure.
- `decode_narrator_text_stream_6`: Iterates through a data stream to decode characters by performing bitwise operations and lookups in a table, writing the resulting bytes to a destination buffer and calling decode_narrator_text_stream_5.
- `decode_narrator_text_stream_3_2`: Iterates through a data stream to decode characters, updating a buffer and managing a counter in a structure, and calls update_status_word.
- `decode_narrator_text_stream_3`: Iterates through a data stream to decode characters, updating a buffer and managing a counter in a structure, and calls update_status_word and process_narrator_text_stream_2.
- `update_status_word`: Decrements d0, and if it is not negative, writes the word in d5 to offset 16 of a structure in a5, then branches to another routine.
- `execute_callback_loop`: Checks a bit in a structure at offset 210 of a5, writes 32832 to a hardware register if the bit is set, and calls a routine in a loop based on a value from a structure.
- `enable_dma_if_flag_set`: Tests a flag byte in a structure pointed to by A5 and, if set, writes a value to a custom hardware register that enables DMA channels by setting bit 15 of DMACON.
- `manage_interrupt_and_list`: Updates a global variable, increments a byte at offset 326 of a6, and if a byte from a2 matches a byte in a structure at offset 70 of a5, it manipulates a linked list and calls another routine; otherwise, it decrements a byte in a6 and updates a global variable.
- `decrement_counter_and_enable_interrupts`: Reads a byte from an offset in a structure pointed to by A6, decrements it and writes it back.
- `calculate_offset_address`: Multiplies a byte from offset 68 of a structure in a3 by 68, then if a word at offset 22 of a5 is not zero, adds this result to a pointer from offset 236 of a4 and moves the new address to a2.
- `process_callback_list`: Loops through a list of pointers, setting a bit at offset 30 of each structure and calling the function pointer found at offset 20, then clears bit 7 of a byte at offset 30 of a structure in a3 and calls another routine.
- `loop_and_set_bit`: If bit 6 of a word at offset 226 of a structure in a5 is set, it writes 32832 to a hardware register, otherwise it returns.
- `set_bit_and_loop`: Sets bit 5 of a byte at offset 30 of a structure in a3 and loops back to the start of the routine.

### Qume_LetterPro_20

- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `execute_triple_call_2`: Attempts to call a routine via a pointer in a global variable three times.
- `call_routine_thrice`: Calls another routine three times and returns -1.
- `execute_handler_sequence`: Calls a routine through a pointer, then calls that same routine twice more using the value from A6 as an argument.
- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_printer_status`: Saves registers to the stack, loads three pointers from the stack, clears registers d1, d2, and d3, compares the word at the pointer in a0 with 1, and branches to a different location if they are not equal or otherwise jumps to a fixed offset.
- `process_printer_command_sequence_4`: Copies a sequence of bytes from a table into a destination buffer, then parses a command byte and an associated value to update the buffer with control characters or formatted ASCII digits.
- `copy_byte_from_table`: Copies a byte from a table at a zero-based offset in D0 to a destination address calculated from A3 and A4, then branches to another routine.
- `update_values_on_condition`: Compares a word at the address in a0 to 27 and a byte at the address in a1 to zero; if both conditions are met, it writes 32 to the address in a0, writes 1 to the address in a1, and branches to another location.
- `update_value_on_condition_3`: Compares a word at the address in a0 to 27; if equal and the byte at the address in a1 is less than 0, it writes 4 to the address in a0 and branches to another routine, otherwise it returns.
- `update_value_on_match_2`: Compares a word at the address in a0 to 28; if equal and the byte at the address in a1 is greater than 0, it writes 33 to the address in a0, otherwise it returns.
- `update_word_and_byte_2`: Compares a word at the address in a0 to 29; if equal and the byte at the address in a1 is zero or less, it writes 33 to the address in a0, writes 255 to the address in a1, and branches to another routine.
- `update_word_on_match_2`: Compares a word at the address in a0 to 29; if equal, it checks if the byte at the address in a1 is less than or equal to 0, and if so, writes 2 to the word at the address in a0 before branching to another routine; otherwise it returns.
- `update_word_on_match`: Compares a word at the address in a0 to 30; if it is equal, it checks if the byte at the address in a1 is less than 0, and if so, writes 32 to the address in a0.
- `update_value_on_match`: Compares a word at the address in a0 to 31; if it is equal, it checks if the byte at the address in a1 is greater than 0, and if so, writes 33 to the address in a0; otherwise, if the byte at a1 is less than 0, it writes 32 to the address in a0 and clears the byte at a1, then branches to another routine.
- `increment_byte_on_match`: Compares a word at the address in a0 to 32; if it is equal, increments the byte at the address in a1 and branches to another routine.
- `decrement_and_branch_2`: Compares a word at the address in a0 to 33, and if it is equal, decrements the byte at the address in a1 and branches to another routine.
- `format_status_string`: If a word at the address in a0 is 57, it writes bytes 27 and 70 to a destination address calculated from a3 and a4, then writes a byte representing a digit derived from a stack value to the next two positions.
- `store_incremented_value`: Extends the value in D0 to a long integer, adds 48 to it, and stores the resulting byte at a destination address calculated from A3 and A4 before branching to another routine.
- `write_status_byte`: Checks if a word at the address in A0 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in D1.

### CBM_MPS1000

- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_function_pointers`: Attempts to call three successive functions using offsets from two different global variables, where each subsequent call depends on the success of the previous one.
- `execute_two_subroutines`: Calls a routine using a pointer from a global variable, restores a register from the stack, calls another routine using a different global pointer, restores a register from the stack, and branches to another routine.
- `execute_indirect_function_calls`: Fetches a function pointer from a global variable using an offset and calls that function three times consecutively.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_2`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte constant, then executes three routines through a global base and one routine through a pointer retrieved from the structure.
- `manage_printer_buffer_state`: Acts as a multi-purpose state machine that manages a printer buffer by performing operations such as clearing memory with a loop, writing escape sequences and data bytes, bitwise ORing data into a buffer, or updating status flags and global variables based on a switch case.
- `restore_registers_and_return`: Clears register d0 and restores several registers from the stack before returning.
- `initialize_local_variables_2`: Saves registers to the stack, loads three pointers from offsets 40, 44, and 48 of the stack frame into a2, a4, and a3, clears d4, d5, and d1, sets d3 to 3, and branches to another routine if the word at the address in a2 is not 1.
- `format_printer_command_buffer_3`: Constructs a command string in a buffer by copying data from a table, appending a string from a source pointer, and appending specific byte sequences based on values read from a structure.
- `modify_buffer_byte`: Loads a pointer from a structure in d1, compares a word at offset 2048 of that pointer to 2048, and if equal, increments d4, adds the result to a4, and writes 15 to the address in a0.
- `process_command_and_update_buffer`: Copies a null-terminated string from one memory location to another using a loop, then performs a series of conditional checks on a value at a specific memory address to update a global variable, modify a status byte, or append specific byte sequences to a buffer.
- `copy_bytes_loop_2`: Increments d4 and d5, then enters a loop that copies a byte from a table at offset d0 of a1 to a destination address calculated from a0 and a4.
- `modify_status_and_pointer`: Compares a word at the address in a2 to 66; if it is equal, it sets a byte at offset 2 to 1, loads a pointer from a0 into a1, compares a word at offset 208 of the address in a1 to 2048, and if it is equal, writes 96 to the address in d3.
- `write_byte_at_pointer`: Loads a pointer from the address in a0 into a1, compares a word at offset 138 of the address in a1 to 1024, and if it is equal, writes the byte 137 to the address in d3.
- `set_byte_at_pointer`: Moves the address in d3 to a0, writes the byte 80 to the address in a0, and branches to another routine.
- `copy_bytes_from_table_2`: Copies up to five bytes from a table starting at a memory location calculated from a pointer and an offset into a destination address, then returns.
- `modify_status_values`: Compares a word at the address in a2 to 32; if it is equal, it checks if the byte at the address in a3 is non-zero; if so, it writes 1 to the byte at a3 and writes 27 to the word at a2, then branches to another routine.
- `clear_byte_and_set_word`: Tests a byte at the address in a3; if it is negative, it clears that byte and writes the word 30 to the address in a2, then branches to another routine; otherwise it branches to another routine.
- `branch_with_constant`: Loads the value 255 into D0 and branches to another routine.
- `reset_status_values`: Compares a word at the address in a2 to 33; if it is equal, checks if the byte at the address in a3 is zero, and if so, writes the byte 255 to the address in a3 and the word 29 to the address in a2.
- `clear_and_set_status`: Tests a byte at the address in a3, and if it is less than or equal to zero, clears the byte at the address in a3, writes the word 28 to the address in a2, and branches to another routine.
- `branch_to_offset_3`: Performs a short branch to a different location.
- `update_data_based_on_value`: Compares a word at the address in A2 against several constants to write specific bytes to a destination address in A3 or to the stack, and if the word is zero, writes the byte 253 to offset 558 of a structure pointed to by D1.
- `write_status_byte_alt`: Checks if a word at the address in A2 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in D1.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_630

- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_function_pointers_2`: Calls three consecutive functions using offsets from a pointer stored in a global variable, and if any call returns non-zero, it calls a cleanup routine using an offset from that same global variable before returning.
- `execute_nested_pointers`: Loads a pointer from a global variable into a1, calls a routine using a pointer from a6 at an offset of negative 638, restores a6 from the stack, loads another pointer into a1, calls a routine using a pointer from a6 at an offset of negative 638, restores a6 from the stack, and branches to another routine.
- `execute_triple_handler`: Calls a routine through a pointer found at an offset in a global variable, then calls that same routine twice more using a value from a6 as an argument.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_command_parameters`: Saves registers to the stack, loads three pointers from the stack, clears three registers, and compares the word at the first pointer to one; if it is not one, it branches to a different location, otherwise it continues to another routine.
- `parse_input_buffer`: Copies a sequence of bytes from a source buffer into a destination buffer, applying specific control character transformations based on values read from a structure.
- `process_state_transition_logic`: Copies a null-terminated string into a destination buffer and then performs a series of conditional checks on a word and a byte to update their values.
- `copy_byte_from_table_2`: Copies a byte from a table at a zero offset into a destination address calculated from a pointer in a2, then enters a loop that continues as long as a counter in d3 is greater than 11.
- `update_value_on_match_4`: Compares a word at the address in a0 to 27; if equal and the byte at the address in a1 is not zero, it writes 32 to the address in a0 and 1 to the address in a1, then branches to another routine.
- `update_value_on_match_3`: Compares a word at the address in a0 to 27; if equal and the byte at the address in a1 is less than 0, it writes 4 to the address in a0 and calls another routine; otherwise it returns.
- `update_word_on_match_4`: Compares a word at the address in a0 to 28; if equal and the byte at the address in a1 is greater than zero, it writes 33 to the address in a0 and branches to another routine.
- `update_conditional_word_and_byte`: Compares a word at the address in a0 to 29; if equal and the byte at the address in a1 is zero or less, it writes 33 to the address in a0, writes 255 to the address in a1, and branches to another routine; otherwise it returns.
- `update_conditional_word_3`: Compares a word at the address in a0 to 29; if equal and the byte at the address in a1 is zero or less, it writes 2 to the address in a0 and branches to another routine.
- `update_conditional_word`: Compares a word at the address in a0 to 30; if equal, it checks if the byte at the address in a1 is less than 0, and if so, writes 32 to the word at the address in a0, otherwise it returns.
- `update_word_and_byte`: Compares a word at the address in a0 to 31; if it is equal, it checks if the byte at the address in a1 is greater than 0, and if so, writes 33 to the address in a0; otherwise, if the byte at a1 is less than or equal to 0, it writes 32 to the address in a0 and clears the byte at a1, then branches to another routine.
- `increment_byte_on_match_2`: Compares a word at the pointer in a0 to 32; if they are equal, it increments a byte at the pointer in a1, otherwise it branches to another routine.
- `decrement_byte_on_condition`: Compares a word at the pointer in a0 to 33; if they are not equal, it branches to another routine, otherwise it decrements a byte at the pointer in a1 and branches to another routine.
- `format_conditional_string`: If a word at the address in a0 is 12, it writes byte 27 to a destination address calculated from a4 and a2, then writes either byte 65 or 66 to the following position based on a stack value.
- `write_constant_byte`: Writes the byte 66 to a destination address calculated from a pointer in a2 and then branches to another routine.
- `write_status_byte_to_structure`: Checks if a word at the pointer in a0 is non-zero; if it is zero, it writes the byte 253 to an offset of 2720 in the structure pointed to by the value in d1, then returns.

### ImagewriterII

- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_two_subroutines_2`: Calls a routine using a pointer from a global variable, restores a register from the stack, and calls another routine using a different global pointer.
- `execute_sequential_function_pointers`: Loads a pointer from a global variable into a1, pushes a6 to the stack, and calls a routine at an offset from a new address loaded from another global variable.
- `execute_indirect_function_calls_2`: Fetches a function pointer from a global variable using a negative offset and calls that function three times, passing the contents of register a6 as an argument to the first call and the subsequent calls.
- `return_constant_20`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_21`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_3`: Calls a routine via a pointer in a global variable and, if the result is zero, populates a structure with two parameters and a status byte, then executes three routines through a global base using a pointer retrieved from the structure.
- `POSDIV`: If a value in d2 is greater than 65535, it performs a long division loop using bit shifts and subtractions, otherwise it calculates the quotient and remainder using modulo and division operations on the components of d1 and d2.
- `LONGDIV`: Performs a 16-iteration bitwise division by rotating a 32-bit value and subtracting a threshold from the high half whenever the high half meets or exceeds that threshold.
- `ulmult`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components.
- `ulmodt`: Moves the contents of register d1 into d2 and register d0 into d1 before calling POSDIV and returning.
- `uldivt`: Moves the contents of register d0 into d1, calls another routine, and returns the value left in d1 via d0.
- `lmodt`: Calls POSDIV and negates the returned value if the flag left in an address register by the called routine is nonzero.
- `ldivt`: Calls POSDIV and returns the quotient in the first result register, or the negation of that quotient if the second result register is nonzero.
- `dispatch_via_stack_value`: Saves registers to the stack, reads a byte from the stack into d0, and if the value is not greater than 4, uses it to index a table to jump to a new address; otherwise it jumps to a different address.
- `clear_d0_and_restore_registers_alt`: Clears register d0 and restores d2 through d5 and a2 through a4 from the stack before returning.
- `initialize_printer_handler`: Saves six registers to the stack, loads a pointer from offset 28 of the stack into a1, loads a 32-bit value from offset 32 of the stack into d2, loads a pointer from offset 36 of the stack into a3, clears d3, d4, and a4, compares the 16-bit word at the pointer in a1 to 1, and branches to another routine if they are not equal, otherwise it branches to another routine.
- `format_imagewriter_escape_sequences`: Copies eleven bytes from a table into a destination buffer and then appends various escape sequences based on values read from a structure.
- `generate_imagewriter_escape_sequences`: Translates input values into specific escape sequences for an Imagewriter II printer by writing bytes to a buffer.
- `emit_escape_sequences`: Translates specific input values into corresponding escape sequences for a printer by writing bytes to a buffer pointed to by a register.
- `check_escape_sequence`: Compares a word at the address in a1 to 31; if equal and the byte at the address in a3 is greater than 0, it writes 33 to the address in a1 and 1 to the address in a3, then branches to another routine.
- `reset_status_byte`: Tests the byte at the address in a3; if it is less than 0, it writes 32 to the address in a1, writes 255 to the address in a3, and calls another routine; otherwise it returns.
- `branch_to_address`: Branches to a specific address.
- `translate_imagewriter_escape_sequences_3`: Translates specific input bytes into Apple ImageWriter II escape sequences by writing byte combinations to a destination buffer.
- `write_four_bytes`: Writes the bytes 66, 27, 108, and 48 sequentially to a destination address calculated from a pointer in d2 and an offset in d3.
- `translate_imagewriter_escape_sequences_2`: Translates input bytes into specific escape sequences for an ImageWriter II printer by writing byte combinations to a destination buffer.
- `write_single_byte_2`: Stores the byte 66 at a destination address calculated from a pointer in d2 and an offset in d3, then branches to another routine.
- `translate_imagewriter_escape_sequences`: Translates command buffer bytes into specific escape sequences for an ImageWriter II printer by writing byte combinations to a destination address.
- `write_byte_at_offset`: Writes the byte 80 to a destination address calculated by adding a pointer in d2 to a pointer in a2, then increments d3 and branches to another routine.
- `emit_imagewriter_escape_sequences`: Translates command buffer bytes into specific escape sequences for an ImageWriter II printer by writing byte combinations to a destination address calculated from two pointers.
- `write_byte_at_offset_70`: Writes the byte 112 to a destination address calculated by adding a pointer in d2 to a pointer in a2, then increments d3 and branches to another routine.
- `process_command_buffer`: Checks a byte at the address in a1 against several values to write specific byte sequences to a destination address determined by d2 and d3, or updates a global variable and a stack value.
- `process_buffer_commands`: Checks a byte at the address in a1 against several values to write specific byte sequences to a destination buffer at an offset determined by d2 and d3, while also updating a global variable and a counter in a stack location.
- `conditionally_write_byte`: Compares a byte at a memory location to 1, and if not equal, branches to another routine; otherwise it stores the byte 80 at a destination address calculated from d2 and d3 and branches to another routine.
- `update_printer_status_byte`: Increments a value in a register, calculates a destination address by adding two registers together, stores the byte value one hundred twelve at that address, and branches to another routine.
- `process_buffer_data_3`: Checks a byte at a memory location against several values to write specific bytes to a destination buffer at an offset determined by d3, updates a counter in a stack location within a range of 5 to 10, or copies bytes from a table into the buffer.
- `conditionally_write_byte_offset`: Compares a byte at a memory location to 1, and if not equal, branches to another routine; otherwise it stores the byte 69 at a destination address calculated from d2 and d3 and branches to another routine.
- `write_byte_at_offset_4e`: Writes the byte 78 to a destination address calculated by adding a pointer in d2 to a pointer in a2, then increments d3 and branches to another routine.
- `modify_counter_and_buffer`: Checks a word at the address in a1; if it is 48, it updates a counter in a stack location to a minimum of 5 and a maximum of 10, writes bytes 27 and 115 to a destination, and writes a byte derived from a table to offset 2 of that destination; otherwise, if the word is 12, it updates the counter and writes bytes 27 and 75 to the destination.
- `check_and_modify_memory`: Compares a word at a memory location to 12, and if not equal, branches to another routine; otherwise it checks if a byte at a pointer from the stack is 39 or 37, and if so, writes 30 to that location before branching to another routine.
- `write_three_bytes_with_offset`: Writes the bytes 27, 75, and a value derived from a global variable and a stack value to a destination address calculated from a pointer in d2 and an offset in d3.
- `clear_d0_and_restore_registers`: Clears register d0 and restores d2 through d4 and a2 through a4 from the stack before returning.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_C-150

- `return_constant_22`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_sequential_callback_chain`: Retrieves a pointer from a global variable and attempts to call a function at a specific offset within that pointer's structure; if successful, it repeats this process using the same pointer and a second global variable, calling a cleanup routine at a different offset if any of the calls fail.
- `execute_nested_calls_4`: Loads a pointer from a global variable into a1, saves a6 to the stack, calls a routine at an offset from a6, restores a6 from the stack, loads another pointer into a1, saves a6 to the stack, calls a routine at an offset from a6, restores a6 from the stack, sets d0 to 255, and branches to another location.
- `execute_indirect_function_calls_3`: Fetches a function pointer from a global variable using a negative offset and calls that function three times, passing the contents of a register as an argument to the first call and the subsequent calls.
- `return_constant_23`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_24`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_4`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine via a pointer stored in the structure.
- `divide_32_bit_integers`: Performs 32-bit division using a loop that rotates bits and subtracts a threshold, or performs modulo and division operations if a condition is not met, then calls a library function.
- `rotate_and_threshold_32bit`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_by_16_bit_parts`: Computes a 32-bit product of two 32-bit integers by summing the products of their 16-bit halves and returns the result.
- `call_and_negate_on_flag_2`: Calls another routine and returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `dispatch_via_table_2`: Restores registers from the stack, retrieves three values from the stack, and uses a value from the stack to calculate an offset into a table to perform a jump.
- `return_zero`: Sets d0 to zero and returns.
- `handle_input_command`: Processes a command based on a value in a pointer, which may involve writing bytes to a structure, updating counters, converting values to ASCII characters in a table, or calling another routine.
- `copy_byte_with_loop`: Increments d4 and d5, then enters a loop that copies a byte from a table at address 10 plus d1 to a destination at address d0 plus d3, continuing until d2 is no longer greater than d5.
- `process_and_negate_result_2`: If a 16-bit value at a pointer in a3 is twelve, it modifies a value on the stack based on its content, writes three bytes to a structure, and calls another routine; otherwise, if the value is zero, it writes the byte 253 to offset 2720 of a structure pointed to by a4.
- `process_and_negate_result`: Calculates a value by adding a byte from a global table to a modified stack value, writes two bytes to a structure, calls another routine, and returns either the result or its negation.
- `write_byte_to_offset`: Tests a 16-bit value at a pointer in a3; if it is zero, it writes the byte 253 to offset 2720 of a structure pointed to by a4.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### generic

- `return_constant_25`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_26`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_27`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_2`: The routine returns a constant zero value.

### Brother_HR-15XL

- `return_constant_28`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_indirect_calls_2`: Performs up to three consecutive calls to routines located at offsets from pointers stored in a structure, using a global variable to determine the base address, and returns a status code based on the results.
- `execute_double_indirect_calls`: Loads a pointer from a global variable into a1, then performs two consecutive calls to routines located at offsets from pointers stored in a global variable, using the stack to preserve a6.
- `execute_library_calls`: Calls a library function at an offset from a global base three times, passing a value from a6 to the second and third calls.
- `return_constant_29`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_30`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_31`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_command_parameters_2`: Sets registers a0, a1, a4, d1, d2, and d3 to zero or specific values from the stack, compares a word at the address in a0 to 1, and branches based on the result.
- `process_printer_command_sequence`: Copies a sequence of bytes from a source buffer into a destination buffer, then performs a state machine update on a character and an integer based on specific byte values and offsets within a structure.
- `process_printer_control_sequence`: Copies a null-terminated string to a destination buffer and then processes a series of control characters by updating a value at a pointer in a1 and writing specific byte sequences to the buffer based on the character and its associated byte value.
- `copy_byte_with_loop_alt`: Increments d2, d3, and a2, then enters a loop that copies a byte from address 0 plus d0 to address a2 plus a4, continuing until d2 is no longer greater than d3.
- `compare_and_update_word`: Compares a word at the address in a0 to 27; if it is equal and the byte at the address in a1 is not negative, it writes 32 to the word at a0 and 1 to the byte at a1, then branches to another routine.
- `update_word_on_match_6`: Compares a word at the address in a0 to 27; if it is equal and the byte at the address in a1 is not negative, it writes 4 to the word at the address in a0 and branches to another routine.
- `update_value_if_threshold_met_2`: Compares a word at the address in a0 to 28; if it is equal and the byte at the address in a1 is non-zero, it writes 33 to the word at the address in a0 and branches to another routine, otherwise it returns.
- `update_value_if_threshold_met`: Compares a word at the address in a0 to 29; if it is equal and the byte at the address in a1 is non-zero, it writes 33 to the word at the address in a0 and 255 to the byte at the address in a1, then branches to another routine; otherwise it branches to another routine.
- `update_word_if_positive`: Compares a word at the address in a0 to 29; if it is equal, it checks if the byte at the address in a1 is greater than 0, and if so, writes 2 to the address in a0.
- `update_word_if_negative`: Compares a word at the address in a0 to 30; if it is equal and the byte at the address in a1 is less than 0, it writes 32 to the address in a0, then branches to another routine.
- `update_value_based_on_conditions`: Compares a word at the pointer in a0 to 31; if it is not equal, it branches.
- `increment_byte_on_match_3`: Compares a word at the pointer in a0 to 32; if it is not equal, it branches.
- `decrement_and_branch_3`: Compares a word at a pointer in a0 to 33; if it is not equal, it branches to another location, otherwise it decrements a byte at a pointer in a1 and branches to another location.
- `update_status_byte_2`: Checks if a word at a pointer in a0 is twelve; if so, it writes two specific bytes to a structure, otherwise it checks if the word is zero and writes a byte to offset 2720 of a structure pointed to by d1.
- `write_constant_byte_2`: Increments d2, adds a4 to a2, writes the byte 66 to the address in a2, and then branches.
- `update_status_byte_on_zero`: Checks if a word at the pointer in a0 is zero; if so, it writes the byte 253 to an offset of 2720 of the structure pointed to by d1, then returns.

### HP_LaserJet

- `return_constant_32`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_calls`: Calls a routine using a pointer from a global variable, and if successful, performs a sequence of three additional calls using pointers from a structure, returning zero if all succeed or a specific value otherwise.
- `execute_sequential_function_pointers_2`: Loads two different pointers from global variables and calls two successive functions using offsets from those pointers, then returns.
- `execute_indirect_function_calls_4`: Fetches a function pointer from a global variable using a negative offset and calls that function three times consecutively.
- `return_constant_33`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_5`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine through a pointer stored within that structure.
- `process_hp_laserjet_escape_sequences_3`: Parses a sequence of control characters to manage a printer buffer, including handling escape characters, incrementing or decrementing values, and calling format_hp_laserjet_command_buffer, convert_to_ascii_recursive_2, and process_hp_laserjet_escape_sequences recursively based on the character values.
- `format_hp_laserjet_command_buffer`: Copies a sequence of bytes from a table into a buffer, modifies specific bytes based on hardware identification values, and calls another routine to transmit the resulting data.
- `process_hp_laserjet_escape_sequences`: Processes a sequence of control characters to manage a printer buffer, including handling escape characters, incrementing or decrementing byte values, and copying data from a table into a buffer.
- `copy_data_and_call`: Enters a loop that copies 11 bytes from a global table to a destination at a4 plus d5, then calls another routine twice using values derived from a structure at a6 plus 28.
- `write_word_and_increment_byte`: Compares a word at the pointer in a2 to 27; if it is not equal, it branches.
- `match_escape_sequence`: Compares a word at the address in a2 to 27, and if it matches, checks if the byte at the address in a3 is less than zero; if the byte is not less than zero, it writes the word 4 to the address in a2 and branches to another location.
- `update_word_on_match_7`: Compares a word at the address in a2 to 28; if it is equal, it checks if the byte at the address in a3 is greater than zero, and if so, writes 33 to the word at the address in a2, then branches to another routine.
- `set_values_on_match`: Compares a word at the address in a2 to 29; if it is equal, it checks if the byte at the address in a3 is non-zero, and if so, writes 33 to the address in a2 and 255 to the address in a3.
- `set_word_if_positive`: Compares a word at the address in a2 to 29; if it is equal and the byte at the address in a3 is greater than 0, it writes 2 to the address in a2, then branches to another routine.
- `update_word_if_positive_2`: Compares a word at the address in a2 to 30; if it is equal, it checks if the byte at the address in a3 is non-negative, and if so, writes 32 to the word at the address in a2, then branches to another routine.
- `update_word_and_byte_3`: Compares a word at the address in a2 to 31; if not equal, it branches to another routine, otherwise it checks a byte at a3.
- `compare_and_increment_byte`: Compares a word at the address in a2 to 32; if it is not equal, it branches to another routine, otherwise it increments a byte at the address in a3 and branches to another routine.
- `decrement_and_branch_on_value`: Compares a word at the address in a2 to 33; if equal, it decrements a byte at the address in a3 and branches to another routine, otherwise it branches to another routine.
- `load_global_byte`: Compares a word at the address in a2 to 64; if not equal, it branches to another routine; otherwise, it loads a pointer from offset 28 of the structure in a6 into a0, checks if the byte at that pointer is non-zero, and if so, writes the byte from a global variable at offset 67 to that pointer before branching to another routine.
- `decrement_and_copy_data`: Decrements a byte at the address stored in a pointer from a structure in a6, updates a global variable, conditionally copies a byte from d4 to a buffer, copies 11 bytes from a global table to a destination in a4, and calls two different routines.
- `copy_byte_from_offset`: Increments a value from an offset of 28 in a6, loads a byte from that new address into d1, and then writes d1 to the address held in d4.
- `copy_data_and_call_function`: Copies 11 bytes from a global table to a destination buffer starting at an offset in a4, then calls a routine using a pointer from a6 and calls another routine using a value from a6.
- `compare_and_branch_7`: Compares a word at the address in a2 to 57 and branches to a different routine if it is not equal, otherwise it branches to another location.
- `copy_table_and_call`: Copies 11 bytes from a global table to a destination buffer starting at an offset in a4 using a loop, then calls two different routines using pointers from a6 and a4.
- `write_byte_on_null_check_alt`: Checks if a word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d2.
- `convert_to_ascii_recursive_2`: Writes a character to a destination buffer based on the value of a parameter, modifies that parameter, and then calls itself recursively to process the remaining digits.
- `convert_digit_to_ascii_2`: Compares the byte in d0 to 99; if it is less than or equal, it branches.
- `convert_integer_to_ascii_recursive`: Writes the character zero to a memory location and then recursively calls itself to process a value.
- `convert_to_ascii`: Writes the byte 48 to a memory location, calls another routine, and then writes the returned byte incremented by 48 to the next memory location.
- `call_two_subroutines`: Calls a routine at an offset in a global variable with two parameters, then calls another routine at a different offset in the same global variable.
- `rotate_and_threshold_32bit_2_2`: Performs a 32-bit left rotation sixteen times using the carry flag, subtracting a threshold from the high 16 bits if the value exceeds it, and returns the result.
- `rotate_and_threshold_32bit_2`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag, and subtracts a threshold value from the high 16 bits if the high 16 bits meet or exceed that threshold.
- `multiply_32_bit_integers_4`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components and returns the result.
- `negate_result_on_flag_2`: Calls another routine and negates its return value if the value in the address register is non-zero.
- `dispatch_via_table_alt`: Restores registers from the stack, retrieves four values from the stack, and uses a value from the stack to calculate an offset into a table to perform a jump.
- `clear_registers_and_return`: Sets d0 to zero, restores d2 through d4 and a2 through a4 from the stack, and returns.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `initialize_local_variables`: Saves registers a2 and a3 to the stack, loads a word from the stack into d0, and sets a1, a3, and a2 to specific constant values.
- `compare_word_and_branch_2`: Compares a word in d0 to 256 and branches if it is lower, otherwise branches if it is higher, otherwise branches to another location.
- `compare_and_branch_4`: Compares a word in d0 to 512 and branches if not equal, otherwise it performs a short branch.
- `compare_and_branch`: Compares d0 to 1024 and branches if they are not equal, otherwise it performs a branch.
- `write_structure_fields`: Writes 600 to offset 26 of the structure pointed to by the value in a1, 800 to offset 30, 75 to offset 36, and 75 to offset 34 of the structure pointed to by the value in a1, then writes 48 to offset 34 of the structure pointed to by the value in a1 and sets a global variable to 48.
- `initialize_structure_values_3`: Writes 1280 to offset 26 of a structure, 1060 to offset 30, 154 to offset 36, 154 to offset 34, and 49 to offset 45 of a structure pointed to by a1, then branches to another routine.
- `initialize_structure_values_variant`: Writes 1168 to offset 26 of a structure, 1590 to offset 30, 150 to offset 36, 150 to offset 34, 49 to offset 45, and 53 to the address in a2, then branches to another routine.
- `initialize_structure_values`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 33, and writes 30 to two memory locations.

### Okidata_292

- `return_constant_34`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_chain`: Calls three different library functions using pointers retrieved from a global variable and returns a value based on their results.
- `execute_nested_function_pointers_3`: Loads a pointer from a global variable into a6, calls another routine using an offset from a6, restores a6 from the stack, loads a different pointer from a global variable into a6, calls another routine using an offset from a6, restores a6 from the stack, and sets d0 to negative one.
- `execute_library_calls_2`: Calls a library function three times using a pointer from a global variable and returns zero.
- `return_constant_35`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_36`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_6`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a constant value, then executes three routines through a global base using a pointer retrieved from that structure.
- `manage_buffer_layout_and_fill_data`: Calculates memory offsets for a structured data buffer and uses a switch-based state machine to either clear the buffer, perform bitwise OR operations on specific bytes, or populate the buffer with a repeating sequence of constant bytes and values from a global array using a loop.
- `restore_registers_and_return_2`: Clears register d0 and restores several data and address registers from the stack before returning.
- `initialize_command_buffer`: Saves registers to the stack, loads several values from the stack into a2, d2, and a3, clears d3, d6, and d1, sets d5 to 3, and compares the word at the address in a2 to 1.
- `format_printer_command_buffer`: Copies a sequence of bytes from a table into a destination buffer, inserting specific control characters based on values read from a structure.
- `update_pointer_and_byte_2`: Compares a word at an offset of a pointer in a0 to 2048, and if equal, increments a register and writes the byte 15 to a memory location calculated from that register and d2.
- `process_command_and_update_buffer_2`: Copies a null-terminated string from a source table into a destination buffer, then performs a series of conditional checks on a value at a specific memory location to update a global variable, modify a buffer with specific byte sequences, or toggle state flags.
- `loop_table_copy`: Increments d3 and d6 in a loop, copying bytes from a table at offset 0 to a destination address calculated from d2 and d3.
- `update_conditional_word_2`: Compares a word at the address in a2 to 66; if it is not equal, it sets a longword at the address in d1 to 1, loads a pointer from the address in a1 into a0, compares a word at offset 86 of the structure in a0 to 2048; if not equal, it loads a pointer from d5 into a0 and writes the byte 96 to the address in a0, then branches to another location.
- `check_and_write_byte`: Compares a word at offset 86 of the structure pointed to by the address in a1 to 1024; if not equal, it writes the byte 137 to the address in a0, which is loaded from the address in a1.
- `write_byte_to_pointer`: Moves the address in d5 to a0, writes the byte 80 to the address in a0, and branches.
- `write_byte_sequence_to_memory`: Writes a byte from a table to a memory location calculated from two pointers and increments two counters, repeating this process up to four times before branching to another routine.
- `update_conditional_values`: Compares a word at the address in a2 to 32; if it is not equal, it checks if the byte at the address in a3 is non-zero, and if so, writes the byte 1 to the address in a3 and the word 27 to the address in a2 before branching to another routine.
- `clear_byte_conditionally`: Clears a byte at the address in a3 if it is not negative, then writes the word 30 to the address in a2 and branches to another routine.
- `branch_with_immediate_value`: Loads the value 255 into d0 and branches to another routine.
- `update_status_values_2`: Compares the word at the address in a2 to 33, and if it is not equal, checks if the byte at the address in a3 is not less than or equal to zero; if that condition is met, it writes the byte 255 to the address in a3, writes the word 29 to the address in a2, and branches to another routine.
- `clear_byte_conditionally_alt`: Clears a byte at the address in a3 if it is not less than or equal to zero, then writes the word 28 to the address in a2 and branches to another routine.
- `branch_to_another_routine`: Branches to another routine after the caller loads the value 255 into a data register.
- `process_status_values`: Writes specific bytes to a stack location or a destination address based on the value at the address in a2, and may write a byte to an offset of a structure in d1.
- `check_and_modify_byte`: Compares a word at the address in a2 to 12; if it is not 12, it branches.
- `write_sequence_to_memory`: Writes the bytes 27 and 114 to the memory locations at the address pointed to by d2 and d2 plus one, then writes a longword from a table to the address at d2 plus two.
- `write_status_byte_2`: If the word at the address in a2 is zero, it writes the byte 253 to offset 2720 of the structure pointed to by the address in d1.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Okimate_20

- `return_constant_37`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_calls_2`: Loads a global variable into a register and uses it to call a routine via an offset of -554.
- `execute_nested_calls_5`: Loads a pointer from a global variable into a1, pushes a6 to the stack, loads another pointer from a global variable into a6, calls a routine at an offset from a6, restores a6 from the stack, loads a second pointer into a1, pushes a6 to the stack, loads a pointer from a global variable into a6, calls a routine at an offset from a6, restores a6 from the stack, and branches to another routine.
- `update_handler_pointer`: Calls a library function three times and then updates a global variable with its own address regardless of a condition checked in a global structure.
- `set_global_pointer`: Loads a longword from a memory location into a0, compares a word at offset 100 of the structure in a0 to 2, and if it is not equal, loads the address in a0 with a new value and stores that address into a global variable at offset 54 before branching to another routine.
- `set_function_pointer`: Stores the address of this routine into a global variable at offset 54 and returns zero.
- `return_constant_38`: Returns the constant an address in d0 to the caller without any other effect.
- `manage_printer_buffer_and_output`: Implements a state machine to manage a printer buffer, handling buffer initialization, data insertion via bitwise OR, and clearing the buffer with specific control characters.
- `clear_d0_and_restore_registers_2`: Sets register d0 to zero and restores several data and address registers from the stack before returning.
- `process_printer_command_buffer`: Dispatches printer-related commands via a switch statement that manages a data buffer, including clearing memory with a loop, writing specific byte sequences, and performing bitwise operations on a target memory location.
- `restore_registers_and_return_3`: Clears the d0 register and restores several data and address registers from the stack before returning.
- `divide_32_bit_integers_2`: Performs 32-bit division of a value in d1 by a value in d2, using a loop to handle the high and low halves, and calls a library function.
- `rotate_and_threshold_32bit_3`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_5`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products and returns the result.
- `forward_call_to_helper_3`: Moves the contents of register d1 into d2 and the contents of d0 into d1, calls measure_string_length, and returns.
- `initialize_and_check_word`: Saves registers to the stack, loads several values from the stack into registers, and compares a word at the address in a0 to 1, branching to another routine if it is not equal.
- `build_complex_command`: Copies 15 bytes from a global table to a destination buffer in a loop, then performs multiple conditional checks based on values in a structure pointed to by a4 to write specific byte sequences into the buffer.
- `build_command_buffer`: Writes bytes to a buffer based on values found at specific offsets within a structure pointed to by a4, including a loop that writes the byte 13 a variable number of times.
- `fill_memory_with_thirteen`: Fills a memory area starting at the address in a3 with the value 13, incrementing the offset and a counter in a loop until the counter reaches a value specified by a byte at an address pointed to by a pointer in d3 and d4.
- `update_command_state_and_buffer`: Processes a command byte by updating a state variable and a buffer with specific byte sequences, including a loop that fills a memory range with the value thirteen based on a length byte.
- `fill_memory_with_thirteen_2`: Fills a memory range with the value thirteen, using a byte located at an offset from a pointer in a register as the loop counter.
- `process_command_logic`: Checks if a word at the address in a0 is 12; if so, it checks if a word at offset 2660 of a structure in a4 is 2, and if so, it clamps a byte at a stack location between 30 and 31, writes the difference to a byte at a pointer in d3, and fills a memory range starting at a pointer in a3 with the byte 13 for a count determined by a byte at a pointer in d4.
- `fill_memory_with_constant`: Writes the value 13 to a memory location at an offset determined by a register and increments the offset and a counter until a byte at a pointer-referenced location is reached.
- `update_command_values`: Compares a word at the address in a0 to 32; if not equal, it checks if the byte at the address in a2 is non-zero, and if so, writes the byte 1 to the address in a2 and the word 27 to the address in a0.
- `clear_and_write_word`: Tests a byte at the address in a2; if it is not positive, it branches.
- `load_and_branch`: Loads the value negative 1 into register d0 and performs a short branch to an offset.
- `modify_command_data`: Compares a word at the address in a0 to 33; if it is not equal, it checks if the byte at the address in a2 is non-zero, and if so, writes the byte 255 to the address in a2 and the word 29 to the address in a0 before branching to another routine.
- `clear_status_byte_4`: Checks if the byte at the address in a2 is less than or equal to zero; if it is not, it clears the byte at the address in a2, writes the word 28 to the address in a0, and branches to another routine.
- `branch_to_offset_4`: Performs a short branch to an offset.
- `map_command_values`: Compares a byte at a pointer against several constants and writes specific byte or word values to a destination based on the match.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_Advantage_D25

- `return_constant_39`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_call_sequence`: Attempts to call three functions sequentially using offsets from a global variable and a passed parameter; if any call returns zero, it executes a cleanup routine using an offset from the global variable and returns negative one, otherwise it returns zero.
- `execute_double_call`: Performs two calls to a routine using addresses from a table and sets d0 to negative one before branching.
- `call_library_function_thrice`: Calls a library function three times, passing the value from a6 to the second and third calls.
- `return_constant_40`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_41`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_42`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_command_state`: Clears three 32-bit registers and sets a 32-bit register to zero, then compares the word at offset 32 of the stack to one and branches to another routine or performs a short jump.
- `parse_string_and_update_state`: Copies twelve bytes from a global table into a destination buffer, then performs a series of conditional checks on values at specific offsets within a structure to modify bytes in that buffer and update a state machine based on character comparisons.
- `process_buffer_and_logic`: Copies bytes from a source to a destination in a loop until a null byte is encountered, then performs several conditional checks and writes values to a global variable and a buffer.
- `copy_table_bytes`: Copies bytes from a table to a destination address in a loop, using d0 as an index and d3 as a counter.
- `update_value_on_match_3_condition`: Compares a word at the address in a0 to 27; if equal and the byte at the address in a1 is less than 0, it writes 32 to the address in a0 and 1 to the address in a1, otherwise it returns.
- `update_word_on_condition`: Compares a word at the address in a0 to 27 and checks if the byte at the address in a1 is negative, then writes 4 to the address in a0 and branches to another routine if the conditions are met.
- `update_word_on_match_5`: Compares a word at the address in a0 to 28; if equal, it checks if the byte at the address in a1 is greater than 0, and if so, writes 33 to the address in a0 and branches to another routine.
- `set_word_and_byte`: Compares a word at the address in a0 to 29; if it is equal and the byte at the address in a1 is non-zero, it writes 33 to the word at the address in a0, writes 255 to the byte at the address in a1, and branches to another routine.
- `update_word_on_match_conditional`: Compares a word at the address in a0 to 29; if equal, it checks if the byte at the address in a1 is greater than 0, and if so, writes 2 to the address in a0 and branches to another routine.
- `set_word_on_negative_byte`: Compares a word at the address in a0 to 30; if it is not equal, it returns, otherwise it checks if the byte at the address in a1 is negative; if it is, it writes the word 32 to the address in a0 and branches to another location.
- `modify_status_word`: Compares the word at the address in a0 to 31, and if it is not equal, checks if the byte at the address in a1 is greater than zero; if that condition is met, it writes the word 33 to the address in a0, checks if the byte at the address in a1 is greater than or equal to zero, and if it is not, writes the word 32 to the address in a0 and clears the byte at the address in a1 before branching to another routine.
- `increment_byte_on_match_4`: Compares a word at the address in a0 to 32; if not equal, it otherwise increments a byte at the address in a1 and branches.
- `decrement_byte_on_match`: Decrements a byte at the address in a1 if the word at the address in a0 is equal to 33.
- `write_status_byte_if_zero`: Checks if a 16-bit word at the address in a0 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d1.

### Alphacom_Alphapro_101

- `return_constant_43`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_call_sequence_2`: Performs up to three consecutive calls to routines located at specific offsets within structures pointed to by a global variable and a passed parameter, returning negative one if any call fails, otherwise returning zero.
- `execute_triple_call_sequence_3`: Loads a pointer from a global variable into a1, then performs two sequential calls to functions located at offsets from a table, using values from a global variable and a1 as arguments.
- `execute_indirect_function_calls_5`: Fetches a function pointer from a global variable using a negative offset and calls that function three times, passing the contents of register a6 as an argument to each call.
- `return_constant_44`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_45`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_46`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_local_registers`: Saves registers to the stack, loads three pointers from the stack into a1, a4, and a2, clears registers d1, d2, and d3, and then compares a word at the address in a1 to 1; if not equal, it branches.
- `construct_printer_command_buffer`: Copies a sequence of bytes from a global table into a buffer and appends escape sequences based on values read from a structure.
- `fill_buffer_with_sequence_2`: Fills a buffer starting at a4 with the byte 32 in a loop until a counter reaches a value from a stack pointer.
- `fill_buffer_with_sequence`: Fills a buffer starting at a4 with the byte 32 in a loop, then appends a fixed sequence of six bytes.
- `update_control_bytes`: Compares the word at the address in a1 to 27, and if it is not equal, checks if the byte at the address in a2 is not zero; if that condition is met, it writes the word 32 to the address in a1, writes the byte 1 to the address in a2, and branches to another routine.
- `check_and_update_control_word`: Compares the word at the address in a1 to 27, and if it is not equal, checks if the byte at the address in a2 is not zero; if it is zero, it writes the word 4 to the address in a1 and branches.
- `modify_word_on_condition`: Compares a word at the address in a1 to 28 and checks if the byte at the address in a2 is positive, then writes 33 to the address in a1 and branches to another routine if the conditions are met.
- `update_value_on_condition_2`: Compares a word at the address in a1 to 29; if it is not 29, it branches.
- `update_value_on_condition`: Writes the word 2 to the address in a1 if the word at a1 is 29 and the byte at a2 is positive.
- `modify_word_on_condition_alt`: Compares a word at the address in a1 to 30 and checks if the byte at the address in a2 is non-negative, then writes 32 to the address in a1 and branches to another routine if the conditions are met.
- `modify_word_and_byte_2`: Compares a word at the address in a1 to 31; if it is equal, it checks if the byte at the address in a2 is greater than zero, and if so, writes 33 to the word at the address in a1, otherwise it checks the byte at a2 again and if it is non-negative, writes 32 to the word at a1 and clears the byte at a2, then branches to another routine.
- `increment_byte_on_match_5`: Compares a word at the address in a1 to 32; if not equal, it branches to another routine, otherwise it increments a byte at the address in a2 and branches to another routine.
- `compare_and_decrement_byte`: Compares a word at the address in a1 to 33; if it is not equal, it branches to another routine, otherwise it decrements a byte at the address in a2 and branches to another routine.
- `format_printer_command_buffer_2`: Checks a value in a buffer to write specific control sequences, including a pattern of bytes and escape sequences, into a destination buffer.
- `write_constant_byte_3`: Writes the byte 66 to an address calculated by adding a4 to the value in d1, then branches.
- `fill_buffer_with_patterns`: If a 16-bit word at the address in a1 is 66, it fills a buffer starting at a4 with a byte 13, followed by 80 bytes of 8, then a five-byte sequence, then 80 bytes of 32, and finally another five-byte sequence; otherwise, if the word is zero, it writes the byte 253 to offset 2720 of the structure pointed to by a global variable.
- `write_status_byte_if_zero_2`: Checks if a 16-bit word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by the pointer in d2.

### Epson_JX-80

- `return_constant_47`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_calls_3`: Calls a routine using a pointer from a global variable, and if successful, calls another routine using a pointer from the first global variable, and if that is successful, calls a third routine using a pointer from the first global variable; if the third call fails, it calls a routine using a pointer from the address of the first global variable, and if the second call fails, it also calls that routine.
- `execute_sequential_function_pointers_3`: Loads two different pointers from a table and calls two successive functions using offsets from those pointers, then returns.
- `execute_triple_call`: Calls a routine through a pointer found at an offset from a global base three times, passing the value from a register to the second and third calls.
- `return_constant_48`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_49`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_7`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte at specific offsets, then executes three routines through a global base and a fourth routine using a pointer retrieved from the structure.
- `format_epson_jx80_print_buffer`: Constructs a structured print buffer for an Epson JX-80 printer by calculating offsets, clearing a memory region, and populating it with escape sequences and data bytes in a loop.
- `clear_d0_and_restore_registers_3`: Clears register d0 and restores several data and address registers from the stack before returning immediately.
- `setup_and_compare_word`: Saves registers to the stack, loads values from the stack into registers, and compares a word at the address in a2 to 1, branching to another routine if it is not equal.
- `construct_epson_jx_80_command`: Copies a sequence of bytes from a table into a destination buffer, then performs conditional logic to append command bytes based on values read from a printer structure.
- `process_epson_jx_80_commands_2`: Parses command bytes and state values to generate a sequence of control bytes.
- `copy_bytes_from_stack`: Loads a pointer from the stack into a0, reads a byte from that address into d0, decrements d0 by 1, writes the byte in d0 to the address in a1, loads a pointer from the stack into a1, adds 1 to it, reads a byte from the new address in a1 and writes it to the address in a0, then branches to another routine.
- `copy_byte_from_table_4`: Copies a byte from a table at address zero using an index in d2 to a destination address formed by adding d0 and d1, then loops up to 8 times, incrementing d3 and d6 each iteration.
- `process_epson_jx_80_commands`: Interprets control bytes to generate command sequences for an Epson JX-80 printer.
- `update_pointer_and_set_byte`: Loads a pointer from the address in a1 into a0, compares the word at offset 886 of the address in a0 to 1024, and if equal, loads a pointer from d4 into a0 and writes the byte 137 to the address in a0.
- `write_constant_byte_to_pointer`: Loads the address in d4 into a0, writes the byte 80 to the address in a0, and branches.
- `copy_bytes_from_table_3`: Copies up to eight bytes from a table into a destination buffer by incrementing pointers and reading from a zero-based offset.
- `update_status_values`: If a 16-bit word at the address in a2 is 32, it otherwise returns; if the byte at the address in a3 is not zero, it returns; otherwise it writes the byte 1 to the address in a3, writes the word 27 to the address in a2, and branches.
- `clear_status_byte_2`: If the byte at the address in a3 is non-negative, it otherwise returns; if the byte is negative, it clears the byte at the address in a3, writes the word 30 to the address in a2, and branches.
- `set_d0_and_branch`: Sets d0 to negative one and branches.
- `process_epson_command`: Compares a word at the address in a2 to 33, and if it matches, checks if the byte at the address in a3 is non-zero; if the byte is non-zero, it writes 255 to the address in a3, writes 29 to the address in a2, and branches to another location.
- `clear_byte_and_set_word_3`: Checks if the byte at the address in a3 is greater than zero; if so, it clears the byte at the address in a3, writes 28 to the word at the address in a2, and branches to another routine.
- `return_negative_one_5`: Sets the d0 register to negative one and branches to another routine.
- `process_printer_command_sequence_2`: Checks a value at the address in a2 against several constants to determine which bytes to write to a destination buffer, including writing a two-byte sequence and a byte from the stack, or writing a two-byte sequence followed by a word from a global table.
- `modify_printer_data`: If a word at the address in a2 is 4, it writes two bytes and a byte from the stack to a destination; if the word is 12, it returns; if the word is 18, it checks a byte from the stack and either modifies it or writes two bytes and a value from a table to a destination; if the word is 0, it writes a byte to an offset of 2688 of a pointer in a2.
- `check_and_modify_values`: If a 16-bit word at the address in a2 is 12, it otherwise returns; if not, it checks a byte at an address from the stack, and if that byte is 39, it writes the byte 30 to that address; if the byte is 37, it checks if it is greater than or equal to 37 and then branches.
- `write_three_byte_sequence`: Writes the byte 27 and the byte 114 to a buffer, followed by a word from a global table at an offset determined by a value on the stack.
- `write_status_byte_if_zero_3`: Checks if a 16-bit word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d2.
- `call_library_vector_198_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Epson

- `return_constant_50`: Returns the constant an address in d0 to the caller without any other effect.
- `retry_printer_initialization`: Attempts to call a routine via a pointer found at an offset within a global variable up to three times, returning zero if all attempts succeed or negative one if they fail.
- `execute_double_call_2`: Loads a pointer from a global variable into a1, calls a routine at an offset from a pointer in a6, then loads a second pointer into a1 and calls a routine at the same offset from a pointer in a6, finally setting d0 to 255 and branching to a location in a6.
- `execute_indirect_function_calls_6`: Fetches a function pointer from a global variable using a negative offset and calls that function three times, passing the value from a register as an argument to the first call and the subsequent calls.
- `return_constant_51`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_52`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_8`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte, then executes three routines through a global base and one routine through a pointer stored within that structure.
- `process_printer_command_switch`: Dispatches a multi-case command handler that manages a buffer for printer output, including clearing memory, writing escape sequences, and performing bitwise operations on data.
- `clear_d0_and_restore_registers_4`: Clears register d0 and restores d2 through d3 and a2 through a4 from the stack before returning.
- `setup_and_compare_word_alt`: Saves registers to the stack, loads values from the stack into registers, and compares a word at the address in a1 to 1, branching to another routine if it is not equal.
- `format_printer_command_buffer_4`: Copies a sequence of bytes from a table into a destination buffer and appends control characters based on values read from a structure.
- `process_printer_command_sequence_3`: Reads bytes from a structure and processes them based on a command byte, performing conditional writes to a destination buffer, updating status flags, and looping to append eight bytes from a global table to the buffer.
- `copy_byte_loop`: Decrements a byte from a pointer at an offset of 56 from the stack, stores it in a location pointed to by d4, then reads a byte from a location at an offset of 58 from the stack and stores it in a location pointed to by d2, before branching to an earlier instruction.
- `copy_bytes_from_table_4`: Enters a loop that copies a byte from a table at offset d0 to the address in a2, which is the sum of d0 and a4, while incrementing d3 and d5, and continues as long as d0 is greater than d5.
- `process_input_logic`: Compares a word at the address in a1 to 66; if it matches, it writes 253 to offset 266 of a structure in a1, sets a byte at a location in a4 to 0, and then performs a series of checks on a word at offset 2646 of a structure in a1 to write 96, 137, or 80 to a location in a2 before copying 8 bytes from a global table to a destination pointed to by a4 and returning the new pointer position.
- `update_pointer_and_set_byte_alt`: Loads a pointer from the address in a1 into a0, compares the word at offset 886 of the address in a0 to 1024, and if equal, loads a pointer from d2 into a0 and writes the byte 137 to the address in a0.
- `write_constant_byte_to_pointer_alt`: Loads the address in d2 into a0, writes the byte 80 to the address in a0, and branches.
- `copy_printer_command_bytes`: Copies up to eight bytes from a table into a destination buffer by incrementing two pointers and writing bytes one by one.
- `modify_control_byte_on_condition`: Compares the word at the address in a1 to 32, and if equal, checks if the byte at the address in a3 is non-zero; if it is zero, it writes the byte 1 to the address in a3, writes the word 27 to the address in a1, and branches.
- `clear_byte_and_set_word_2`: Checks if the byte at the address in a3 is negative; if it is, it clears the byte at the address in a3, writes the word 30 to the address in a1, and branches.
- `set_d0_and_branch_4`: Sets register d0 to negative one and performs a short branch to another routine.
- `process_epson_command_alt`: Compares a word at the address in a1 to 33, and if it matches, checks if the byte at the address in a3 is non-zero; if the byte is non-zero, it writes 255 to the address in a3, writes 29 to the address in a1, and branches to another location.
- `clear_byte_and_set_word_alt`: Checks if the byte at the address in a3 is greater than zero; if so, it clears the byte at the address in a3, writes 28 to the word at the address in a1, and branches to another routine.
- `return_negative_one_6`: Sets the first data register to negative one and branches to another routine.
- `execute_printer_data_logic`: Evaluates the value at the address in a1 to perform various writes, such as writing specific bytes to a destination buffer or writing a byte to an offset of 2720 in a structure pointed to by a pointer in d1.
- `write_byte_if_zero`: Checks if a word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d1.
- `call_library_vector_198_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### HP_LaserJet_PLUS

- `return_constant_53`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_indirect_calls_3`: Loads a pointer from a global variable into a1, then performs up to three consecutive calls to routines located at offsets from pointers stored in a1.
- `execute_dual_indirect_calls`: Loads a pointer from a global variable into a1, then performs two consecutive calls to routines located at offsets from pointers stored in a global variable, using the stack to preserve and restore a register between calls, before setting d0 to negative one and branching.
- `execute_indirect_function_calls_7`: Fetches a function pointer from a global table using a negative offset and calls that function three times consecutively.
- `return_constant_54`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_9`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a constant value, then executes several routines through a global base.
- `process_hp_laserjet_escape_sequences_4`: Parses a sequence of control characters to manage a printer buffer, handling escape sequences by modifying a state variable and calling other routines to format command buffers, write characters, or adjust buffer offsets based on hardware identification values.
- `configure_hp_laserjet_parameters`: Copies a sequence of bytes from a table into a buffer and checks specific values at offsets in a structure to set character bytes at various offsets in that buffer.
- `process_hp_laserjet_escape_sequences_2`: Manages a state machine for printer escape sequences by updating a state byte and a counter based on input values.
- `copy_and_call_routines`: Copies 11 bytes from a global table into a destination buffer starting at an offset in a6, then calls two different routines using values derived from a structure pointed to by a6.
- `update_conditional_values_2`: Compares a word at the address in a2 to 27; if it is not equal, it branches, otherwise it checks if the byte at the address in a3 is zero, and if so, writes 32 to the address in a2 and 1 to the address in a3.
- `handle_printer_escape_sequence`: Compares a word at the address in a2 to the decimal value 27; if it matches and the byte at the address in a3 is less than zero, it writes the decimal value 4 to the address in a2 and branches to another routine, otherwise it returns.
- `update_word_if_match`: Compares a word at the address in a2 to 28; if it is equal, it checks if the byte at the address in a3 is non-zero, and if so, writes 33 to the address in a2, then branches to another routine.
- `modify_word_and_byte`: Compares a word at the address in a2 to 29; if it is not equal, it returns, otherwise it checks if the byte at the address in a3 is non-zero; if it is, it writes the word 33 to the address in a2, writes the byte 255 to the address in a3, and branches to another location.
- `update_word_on_match_3`: Compares a word at the address in a2 to 29; if it matches and the byte at the address in a3 is greater than zero, it writes the word 2 to the address in a2 and returns.
- `update_word_if_negative_2`: Compares a word at the address in a2 to 30; if it is equal, it checks if the byte at the address in a3 is negative, and if so, writes 32 to the address in a2, then branches to another routine.
- `clamp_word_and_clear_byte`: Compares a word at the address in a2 to 31; if it is not 31, it branches to another routine, otherwise it checks a byte at the address in a3.
- `compare_and_increment_byte_2`: Compares a word at the address in a2 to 32; if equal, it increments a byte at the address in a3 and branches to another routine, otherwise it branches to another location.
- `decrement_byte_on_match_2`: Compares a word at the address in a2 to 33; if it does not match, it branches to another routine, otherwise it decrements a byte at the address in a3 and branches to another routine.
- `check_and_update_printer_status`: Compares a word at the address in a2 to 64 and, if equal, checks if a byte at the address stored in a6 plus 28 is non-zero; if it is zero, it writes the byte 67 to the address stored in a6 plus 28 and branches to another routine, otherwise it branches to another routine.
- `decrement_and_update_buffer`: Decrements a byte at a pointer stored in a structure offset 28 from a6, updates a global variable with the new value, and conditionally swaps a byte at that pointer plus 1 with a value from d4.
- `increment_and_write_byte`: Increments a value from a table offset in a6 by one, reads a byte from the resulting address, and writes that byte to a destination pointed to by d4.
- `copy_data_and_call_2`: Copies 11 bytes from a global table to a destination pointed to by a structure in a6, then calls another routine twice using values derived from a pointer in a6.
- `compare_and_branch_word`: Compares a word at the address in a2 to 57; if not equal, it branches to another location, otherwise it branches to a different location.
- `copy_table_and_execute_callbacks`: Copies 11 bytes from a global table to a destination buffer at an offset in a4 using a loop, then calls another routine twice using a pointer from a6 and two different arguments.
- `write_byte_on_null_check`: Checks if a word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by the address in d2, then returns.
- `convert_digit_to_ascii_4`: Writes a character to the address in the sum of param_2 and param_3 based on the value of param_1, then calls another routine and writes the returned value plus 48 to the address in the sum of param_2 and param_3 plus 2.
- `convert_digit_to_ascii_3`: Compares d0 to 99; if it is greater than 99, it moves the address in d3 to a2, increments d3, adds d4 to a2, writes 49 to the address in a2, subtracts 100 from d0, and branches.
- `convert_to_ascii_recursive`: Writes the byte 48 to a destination, then either writes 48 to the next byte if a byte is less than 10, or writes the quotient of that byte divided by 10 plus 48 to the next byte, calls another routine, and writes the returned byte plus 48 to the following byte.
- `convert_digit_to_ascii`: Writes the character zero to a memory location pointed to by the sum of two registers, calls another routine, and then writes the returned byte incremented by 48 to the next memory location.
- `call_two_subroutines_2`: Pushes two parameters onto the stack, calls a subroutine located at an offset in a global variable, then calls a second subroutine located at a different offset in the same global variable before returning zero.
- `rotate_and_subtract_32bit`: If a value in d2 is greater than 65535, it performs a loop sixteen times that rotates a 32-bit value left and subtracts a value from the high 16 bits if the high 16 bits exceed the value in d2; otherwise, it performs division and modulo operations on a 32-bit value.
- `rotate_and_threshold_32bit_value`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_6`: Computes the 32-bit product of two 32-bit integers by summing the partial products of their 16-bit halves and returns the result.
- `negate_result_on_flag_4`: Calls another routine and negates the returned value if the high word of the result is nonzero.
- `dispatch_via_table_3`: Restores registers from the stack, reads a byte from the stack, compares it to 5, and then performs a jump to an address calculated from a table using a value derived from the stack.
- `restore_registers_and_return_4`: Clears d0 and restores d2 through d3 and a2 through a4 from the stack before returning.
- `call_library_vector_198_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `branch_on_word_value`: Saves registers a2 and a3 to the stack, loads a word from the stack into d0, sets a1 to 4, a3 to 47, and a2 to 46, then branches based on whether d0 is less than, greater than, or equal to 768.
- `compare_word_and_branch_3`: Compares a word in d0 to 256 and branches to an earlier instruction if it is less than or equal, otherwise it branches to a location if it is greater than 256, otherwise it branches to another location.
- `compare_word_and_branch`: Compares a word in d0 to 512 and branches to different locations based on the result.
- `compare_and_branch_2`: Compares a word in d0 to 1024 and branches if not equal, otherwise it performs an unconditional branch.
- `initialize_structure_fields_2`: Writes 600 and 827 to offsets 26 and 30 of a structure pointed to by a1, writes 75 to offset 36 of that structure and offset 34 of a structure pointed to by the first structure, and sets a global variable to 48.
- `set_structure_values_and_branch`: Sets offsets 26 and 30 of a structure pointed to by a1 to 800 and 1060, writes 100 to offset 36 and offset 34 of the structure, writes 49 to offset 45, and branches to another location.
- `set_structure_values_and_store`: Sets offsets 26 and 30 of a structure pointed to by a1 to 1168 and 1590, writes 150 to offset 36 and offset 34 of the structure, writes 53 to a location pointed to by a2, and branches to another location.
- `initialize_structure_values_2`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 51, and writes 48 to two memory locations.

### type

- `initialize_amiga_exec_structures`: Allocates a memory block based on a value from a function call, populates it with a series of header fields and pointers, and updates a linked list within a global structure.
- `parse_amigados_hunk_data`: Iterates through a structure of hunk type codes and offsets to identify executable or relocation segments.
- `store_and_call_routines`: Stores a value into a global variable, calls a routine using a pointer from a global variable with a constant argument, calls another routine via a pointer from a global variable, and calls a third routine.
- `conditional_xor_decrypt_block`: The routine checks a flag stored in a global variable, clears the flag, and if set XORs a block of words starting at a global variable with a constant, repeating a fixed number of times.
- `check_and_set_flag`: Saves registers to the stack, checks a word at a location relative to the program counter, and if that word is zero, writes 1 to that location before branching.
- `perform_logical_or_sequence`: Performs a sequence of nine logical OR operations on the byte in d0 using various immediate values.

### 12

- `return_constant_55`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_3`: The routine returns zero immediately.

### 15

- `return_constant_56`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_57`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_font_data`: Copies a block of data from a source address to a destination address using a loop, incrementing both pointers and decrementing a counter until the transfer is complete.

### 12

- `return_constant_58`: Returns the constant an address in d0 to the caller without any other effect.
- `process_font_glyph_data`: Reads a sequence of bytes from a source pointer and writes them to a destination buffer, performing bitwise transformations on each byte before updating a counter and returning.

### 20

- `return_constant_59`: Returns the constant an address in d0 to the caller without any other effect.
- `process_font_glyph_data_2`: Iterates through a data structure to extract bitmask patterns for a specific character, performing bitwise operations to prepare the glyph for display.

### 11

- `return_constant_60`: Returns the constant an address in d0 to the caller without any other effect.

### Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m].adf:CLI

- `parse_command_line_string`: Parses a command line string by iterating through characters to extract tokens, handling quoted strings and converting specific character sequences into newline or escape characters, and stores the resulting tokens into a global buffer.
- `invoke_two_function_pointers_2`: Calls two function pointers located at fixed negative offsets from a base register, then returns.
- `call_dos_function`: Calls a function at a negative offset from a base register, stores the result in a global variable, and returns if the result is not zero.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_input_3`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output_3`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_lock_2`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_4`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_dup_lock`: Library stub: loads its arguments and the dos library base, calls DupLock and returns its result to the caller.
- `call_current_dir_2`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_execute`: Library stub: loads its arguments and the dos library base, calls Execute and returns its result to the caller.
- `open_console_window_for_cli`: Opens a console window for a new command interpreter using the window specification and system path text.
- `allocate_bptr_entry_chain`: Walks a linked list, allocating a cleared memory block for each entry, recording a value taken from the entry in the block, and linking the blocks together.
- `free_bptr_entry_chain`: Walks the chain built by the allocation routine, releasing each entry value through another routine and then freeing the 8-byte entry block.
- `call_alloc_mem_3`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem_3`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_find_task_3`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.

### mathtrans.library

- `trigger_illegal_instruction_exception`: Executes an illegal instruction to trigger a processor exception.
- `store_pointer_to_global`: As library code, it stores the value from a register into a global variable and returns.
- `manage_structure_state`: As library code, if a byte at offset 8 of a structure in a6 is zero, it moves a longword from a6 to a pointer in a6 plus 4, stores that pointer at the location pointed to by the longword, and calls a routine through a pointer in a global variable; otherwise, it sets bit 3 of a byte at offset 14 of the structure in a6 and returns.
- `set_bit_in_structure`: As library code, it sets bit 3 of a byte at offset 14 of a structure in a6 and returns.
- `increment_and_call_handler`: Library code checks bit 3 of a byte at offset 14 of a structure in a6; if it is not set, it increments a 16-bit value at offset 32 of that structure, and if that value becomes 1, it stores a global variable and calls a library function via a pointer in a global variable before returning; otherwise it returns -1.
- `return_negative_one`: Library code returns -1 in d0.
- `decrement_counter_and_call`: Library code that decrements the 16-bit counter at offset 32 of the structure at A6.
- `mask_and_call_library`: As library code, it masks the lower 7 bits of d0 and calls another routine.
- `calculate_bitfield_difference`: As library code, if the byte in d0 is zero or negative, it returns or calls itself; otherwise, it performs a loop that updates a running difference using a doubling accumulator and bitwise operations, potentially setting bits in a longword based on a value in d0, until a specific condition is met or a value reaches zero.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `process_masked_value`: As library code, it moves a longword from d2 to d0, calls a routine at an offset of minus 60 from a5, moves the result back to d2, and branches to another routine.
- `call_jump_table_entries`: Library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_d1_register`: As library code, it clears the longword in d1 and branches to another location.
- `execute_indirect_calls_4`: Library code performs a series of indirect calls through offsets from a pointer in a5 based on the value of d7, then performs further conditional indirect calls through offsets from a5 based on the values of d6, d4, and d7 before returning.
- `execute_library_logic`: As library code, it performs a series of indirect calls through offsets from a pointer in a5 based on the values of d6 and d7.
- `execute_conditional_calls_2`: Library code that performs a series of indirect calls through offsets from a pointer in A5 based on the values of D6, D4, and D7, then returns a value masked with a constant.
- `execute_conditional_calls`: As library code, it moves d7 to d1, calls a routine at a negative offset from a5, and if the result is not negative, performs several conditional calls using constants and d2.
- `execute_conditional_assignment`: As library code, it moves a longword from d2 to d0, tests d4, and if d4 is not zero, branches to another routine; otherwise, it loads a constant into d2 and branches to another routine.
- `load_address_and_branch`: As library code, it loads a longword into a0 and branches to another routine.
- `mask_return_value`: Library code that calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `calculate_bit_shift_value`: As library code, it performs a loop that shifts a value and decrements a counter, calling another routine if the counter reaches negative 1.
- `shift_and_initialize_registers`: Library code that performs a logical shift right on D7, sets D6, D5, D1, and D2 to specific constants, loads a constant into A0, and branches to a forward offset.
- `execute_loop_with_branching`: As library code, it executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_and_branch_library`: As library code, it pushes the word negative 65534 onto the stack and then branches to another routine.
- `push_and_branch_library_alt`: As library code, it pushes the word negative 65535 onto the stack and then branches to another routine.
- `push_and_branch_library_short`: As library code, it pushes the word 1 onto the stack and then branches to another routine.
- `check_byte_value`: Library code compares a byte in d7 to 184, otherwise it adds 2 to a7 and returns.
- `library_math_transform`: As library code, it performs arithmetic operations on a value in d7, including conditional branches and a loop that iterates through a global table to modify a value based on the input.
- `perform_math_transformation`: This library code performs complex arithmetic operations involving bit shifts and additions on registers using values from a pointer in a0 and a global variable.
- `perform_math_loop_2`: Library code that performs a loop to update registers using values from a pointer in a0 and a global variable while decrementing a counter.
- `perform_math_loop`: This library code performs a loop that updates registers using values from a pointer in a0 and a global variable, while also decrementing a counter.
- `execute_math_loop`: As library code, it performs a loop that updates d5, d6, and d7 using values from a pointer in a0 and other registers, while also incrementing d2 and a0, and performs several conditional branches and calls to other routines.
- `set_d7_constant`: Library code loads the constant an address into d7 and branches to another routine.
- `call_wrapper_and_restore`: As library code, it branches if d5 is equal to d6, otherwise it moves d5 to d6, calls another routine, moves d6 to d7, tests d7, restores registers from the stack, and returns.
- `push_and_branch`: Library code pushes the contents of d5 onto the stack, calls another routine, restores d6 from the stack, and then branches to another routine.
- `compare_and_branch_d6`: Library code that moves D6 to D4, branches if D4 is negative, compares D6 to a constant, and branches based on the carry flag before returning.
- `shift_and_jump`: As library code, it arithmetic shifts d4 right by 8 bits, increments d4, and then branches to a location or jumps to zero.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_sequence`: Library code checks a byte, and if it is non-zero, performs a series of conditional calls and indirect calls based on the value of d7 and other calculated values.
- `rotate_d7_constant`: Library code loads an address into d7, rotates the value right by 1 bit, and branches to another routine.
- `set_d7_constant_41`: Library code loads an address into d7 and branches to another routine.
- `test_stack_byte`: Library code pops a word from the stack into d6, tests the byte, and if it is positive, moves 0 into d7 and branches to another routine; otherwise it returns.
- `set_status_and_restore_registers`: Sets bit 1 of the condition code register and restores six data registers and the address register A0 from the stack before returning.
- `set_status_and_return`: Loads a constant value into a register, adjusts the stack pointer by an offset of 30, and returns.
- `process_complex_math_logic`: As library code, it performs a series of conditional calls and indirect calls based on the value of d7 and other calculated values.
- `execute_recursive_calls`: Library code that calls another routine four times in succession and then returns.
- `process_table_with_shifts`: As library code, it performs arithmetic shifts on d3 and d4 based on d2, adds them to d5 and d6, and then enters a loop that subtracts values from a table pointed to by a0.
- `subtract_table_from_accumulator`: The routine walks through a table pointed to by a pointer and repeatedly adds the current table entry to an accumulator, then subtracts subsequent entries while a counter is decremented.
- `test_and_branch_d7`: As library code, it tests the byte in d7 and branches if it is zero or negative, otherwise it masks d7 with 127, calls another routine, and sets bit 1 of the status register before returning.
- `load_constant_and_branch`: Library code that loads the value negative 255 into register d7 and then performs a short branch.
- `perform_complex_arithmetic`: Library code that performs a loop of bit-shifting and conditional jumps, calls another routine, and then executes a second loop involving multiplication and conditional branches.
- `complex_loop_with_branching`: As library code, it executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `branch_on_d7_sign`: As library code, it tests the byte in d7; if it is positive, it branches to another location, otherwise it masks d7 with 127, calls another routine, and sets bit 1 of the condition code register before returning.
- `perform_complex_dispatch`: Library code calls a routine, saves d3 through d5 to the stack, calls another routine, restores d3 through d5 from the stack, and then jumps to another routine.
- `transform_register_value`: Adds the value of a register to itself, then conditionally performs a series of bitwise operations including an exclusive OR, an arithmetic shift right, a subtraction, a word swap, and a left rotation before returning.
- `transform_byte_value_2`: Library code that performs a series of swaps, rotations, and bitwise XOR operations on register d7, including conditional additions and branches based on the overflow flag.
- `test_byte_value`: Library code that compares register d7 to 124 and 126, and if it matches 126, adds 133 to d7, performs a rotate through the carry flag, and branches based on the result.
- `test_and_branch_on_word`: Clears bit 7 of a word in d7, then branches to another routine if the result is zero, otherwise it clears d7 and branches to another routine.
- `shift_and_test_byte`: Compares register d7 to 254, and if equal, performs two logical shifts on d7 and branches based on the result of the second shift.
- `set_carry_flag_via_shift`: Shifts d7 left by 8 bits, sets d7 to 255, rotates the carry bit into d7, sets the carry flag, and branches to another routine.
- `clear_d7_and_branch`: Sets d7 to zero and branches to another routine.
- `invoke_callback_and_return`: Library code that receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: Library code that receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `load_address_to_a0`: Library code that loads a longword from an offset into register A0 and then branches to a zero offset.
- `load_address_pointer`: This library code loads the address of a global variable into a0 and then branches to another location.
- `load_address_pointer_short`: As library code, it loads the address of a longword into a0 and branches to another location.
- `load_zero_to_a0_2`: Library code loads zero into a0 and then returns immediately.
- `load_address_and_branch_10`: As library code, it loads a long value into a0 and then branches to another routine.
- `load_address_and_branch_11`: Loads the address of a constant into a6 and branches immediately to another routine.
- `load_address_and_branch_12`: Loads the address of a global variable into a0 and branches immediately to another routine.
- `store_register_via_pointer`: As library code, it invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `load_address_and_branch_9`: Loads the address of a constant into a register and branches to a new location.
- `load_zero_to_a0`: Library code that loads zero into register A0 and then branches to a word offset.
- `load_library_base_address`: Loads the address of a global variable into a register and returns immediately.
- `load_constant_to_a0_2`: Library code loads a constant value into a0 and then returns immediately.
- `load_null_pointer_and_branch`: Loads a null pointer into address register zero and performs a short branch to the next instruction.
- `load_address_and_branch_8`: As library code, it loads the address in a constant into a0 and then performs a short branch.
- `load_null_pointer`: As library code, it loads a zero value into a0 and branches to another routine.
- `load_address_and_branch_7`: Loads a zero value into a register and immediately branches to the next instruction.
- `load_constant_to_a0`: Library code that loads a constant into register A0 and then branches to a word offset.
- `call_indirect_via_offset`: Loads a pointer from a global variable into a register, adds a negative offset to that pointer, and calls the resulting address.
- `load_offset_pointer_and_branch`: Loads the address of a global variable into a register and performs a short branch to another routine.
- `loads_address_and_branches`: Loads a longword from a specific memory location into a6 and performs a short branch to another routine.
- `load_address_and_branch_2`: Loads the address of a specific memory location into a6 and branches to a relative offset within another routine.
- `load_pointer_ca`: Library code loads a longword from a global variable into a0 and branches to another routine.
- `load_indirect_handler_address`: Loads the address of a global variable into a register and branches to another routine that calls a function using an offset from that address.
- `load_address_and_branch_3`: Loads a constant value into a register and performs a short branch to another routine.
- `load_address_and_branch_4`: Loads a longword from a specific memory location into a6 and branches to a relative offset.
- `load_address_and_branch_5`: Loads a constant address into a register and performs a short branch to another routine.
- `invoke_callback_and_return_concat`: Library code that invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: Library code that calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_constant_61`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_status`: If a word at offset 8 of a structure in a6 is zero, it calls a routine using a pointer from offset 34 of the structure, then updates a pointer in a global variable and a linked list; otherwise, it sets bit 3 of a byte at offset 14 of the structure in a6.
- `set_icon_bit`: Sets bit 3 of a byte at offset 14 of a structure pointed to by a6 and returns 0.
- `return_constant_62`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_and_store_result`: Stores d0 and a6 into a structure, calls a routine using a pointer from a6, and if the result is zero, calls another routine; otherwise, it stores a6 and the result into two global variables.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_logical_or`: Performs a bitwise logical OR of register d0 with zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `traverse_icon_chain`: Saves two registers to the stack and calls incremental_offset_recurse to process a linked list of icons.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `invoke_icon_chain_handler`: Pushes two pointers from registers onto the stack and calls another routine, then returns.
- `manage_icon_chain_3`: Saves two registers onto the stack and calls another routine to manage an icon chain, then returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_other_routine_2`: Saves two registers to the stack, calls another routine, and then returns.
- `call_and_return`: Calls another routine and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_register_d0`: Performs a logical OR operation on register d0 with zero.
- `process_icon_update`: Library code that calls another routine, and if the result is non-zero, calls another routine with an offset of 140 from the result, then copies 43 bytes from a stack buffer to a structure at an offset from the result, and finally writes several values to the structure at various offsets.
- `copy_structure_data`: Copies 43 bytes from a source structure pointed to by a6 plus an offset to a destination structure pointed to by a2, then performs several 32-bit and 8-bit transfers from the source to the destination, and finally updates a pointer in the destination structure if a specific 32-bit value is non-zero.
- `return_zero_stub`: Clears register d0 and restores registers d2 through d4 and a2 from the stack before returning.
- `copy_icon_data_to_stack`: Copies 43 bytes from a structure pointed to by the third argument into a local stack buffer, extracts several fields from the same structure into local stack slots, and calls another routine with the first two arguments and a pointer to a two-word stack header.
- `populate_icon_attribute_structure_2`: Validates a structure by checking for specific magic values and flags, then populates various offsets within the structure, including updating linked list nodes and allocating memory for icon attributes.
- `populate_icon_attribute_structure`: Checks a structure for specific magic values and flags, then uses a series of function calls to populate various offsets within that structure, including a loop that writes values into a memory block pointed to by a retrieved pointer.
- `update_structure_fields_3`: Library code retrieves values through a series of calls, stores them at offsets 18 and 22 of a structure, and then executes a loop that calls another routine multiple times to retrieve values, storing a result at offset 8 of a structure.
- `execute_callback_chain_2`: Executes a sequence of function calls through pointers, updating a value in a structure and adjusting a counter, then performs a final conditional check and potential assignment from a library function.
- `clear_d6_and_branch`: Clears register d6 and branches to another location.
- `unlk_and_return`: This library routine unlocks a6 and restores registers from the stack before returning.
- `traverse_icon_hierarchy_recursively`: Iterates through a hierarchical icon structure by checking flags and following pointers at various offsets, including a loop that processes a list of pointers.
- `traverse_icon_hierarchy`: Iterates through a hierarchical icon structure by checking flags and following pointers at various offsets within a structure.
- `traverse_icon_structures`: Library code that iterates through various pointers in a structure, calling another routine and a function pointer, and finally stores a value at offset 26 of the structure.
- `process_icon_list`: Increments a pointer in a loop until it reaches zero, calls a function pointer, and then iterates through a list of pointers, calling itself recursively for each non-zero entry and performing several additional function calls.
- `process_icon_list_recursively`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a pointer or table.
- `iterate_icon_chain`: Enters a loop that calls another routine with an incremented parameter and, if the result is non-zero, calls that same routine again with the returned value.
- `restore_registers_and_return_6`: This library routine restores d2, d3, and a2 from the stack before returning.
- `incremental_offset_recurse`: Calls another routine with the same first argument and the second argument increased by 140, then returns.
- `process_icon_update_logic`: Library code that calls a function pointer, then if the result is non-zero, calls another routine with an offset of 144 from the result, then calls another routine with the result and an offset of 140 from it, and if that result is zero, calls another routine with the result and the constant 168; otherwise it calls another routine with the result, an offset of 140 from it, the result, and the constant 168.
- `call_icon_update_routine`: Pushes several registers onto the stack, calls a routine using a pointer from a2, and then branches to another routine based on the result in d0.
- `return_zero_4`: This library routine sets d0 to zero and restores registers from the stack before returning.
- `update_icon_data`: Decrements a 16-bit value through a pointer and, if the result is zero or the original value was less than one, calls another routine; if that routine returns a non-zero value, it decrements the value again, then uses a pointer from the structure to store two 32-bit values into a table at an offset determined by the decremented value multiplied by eight.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from registers and the stack into consecutive slots of that table.
- `execute_icon_chain`: Library code that calls another routine, and if the result is non-zero, performs a series of conditional calls to other routines using the result and various parameters, eventually returning the result or zero.
- `move_d6_to_d0_and_branch`: Moves the contents of d6 to d0 and branches to another location.
- `call_lib_func_and_return_zero`: Invokes a function pointer retrieved from a global variable associated with library index -210.
- `incremental_offset_recurse_alt`: Calls another routine with the same first argument and the second argument increased by 78, then returns.
- `search_string_sequence`: Calls another routine, then enters a loop that checks if a value at a pointer is zero; if not, it calls another routine and, if the result is non-zero or the byte at the destination pointer plus the value is not an equals sign, it increments the pointer and repeats the loop.
- `return_zero_minimal`: This library routine sets d0 to zero and restores d2, d3, a2, and a3 from the stack before returning.
- `execute_conditional_loop`: Library code that enters a loop that calls another routine and, if the result matches a value in a register and is zero, returns one; otherwise, it updates a value and continues the loop.
- `process_counter_loop`: Enters a loop that calls another routine and, if the result matches a value in a register and is zero, returns one; otherwise, it updates a register with a value from another routine and, if that value is zero, breaks the loop, or if it is non-zero, calculates a difference and continues the loop.
- `increment_and_store_d4`: Tests register d4, and if non-zero, increments its value and stores the result in register d2 before branching.
- `clear_d2_and_branch`: Clears register d2 and branches to another location.
- `save_registers_and_return`: The library code clears register d0 and saves registers d2 through d6 from the stack before returning.
- `process_string_table`: Library code that clears a byte at offset 30 of a structure, calls another routine, and then enters a loop that converts a string of digits into an integer by multiplying a running total by ten for each digit.
- `setup_table_processing`: Library code that increments a pointer in a2 by 5, pushes two pointers onto the stack, calls a routine, and if the result in d0 is non-zero, sets d5 to 1, increments a2 by 3, and branches to another routine; otherwise it branches to another routine.
- `clear_register_and_branch_2`: Clears register d5 and branches to another instruction.
- `process_table_entries`: Enters a loop that updates a value by adding a product of another value and ten, then increments a pointer and updates that value with a byte from the pointer; if the loop terminates and a specific condition is met, it calls another routine twice, otherwise it calls another routine twice and returns.
- `compare_and_set_d0`: Loads a 32-bit value from the stack, compares it against a constant, and sets d0 to either a specific value or the result of a subtraction based on the comparison.
- `return_negative_one_3`: The library code sets register d0 to negative one and returns.
- `call_library_vector_198_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_recursive_traversal`: Library code that calls another routine and uses the result to conditionally call a second routine or perform a cleanup call.
- `conditional_icon_update`: Calls a library function and, if the result is non-zero, writes 10 to an offset of 14 in a structure pointed to by the result and then calls another routine using a pointer incremented from the original parameter.
- `restore_registers_and_return_5`: Clears d0 and restores d2, d3, and a2 from the stack before returning.
- `call_library_and_verify_parameter`: Calls a library routine and, if the returned value does not match the fourth parameter, sets the return value to zero.
- `move_d1_to_d0_and_restore`: Moves the contents of d1 into d0 and restores d2 and d3 from the stack before returning.
- `validate_callback_return_value`: Calls another routine and compares its return value to a fourth parameter; if the values do not match, it returns zero, otherwise it returns the result of the called routine.
- `transfer_d1_to_d0_and_restore_registers`: Moves a longword from register d1 into register d0 and restores registers d2 and d3 from the stack before returning.
- `perform_division`: The library code calls a library function at an offset from a base address retrieved from a global variable.
- `rotate_and_threshold_32bit_4`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_7`: This library routine multiplies the 32-bit values in the two data registers using 16-bit partial products: it forms the high-by-low and low-by-high cross terms, combines them shifted into the upper half, and adds the low-by-low product.
- `invoke_table_entry_3`: The library code retrieves a function pointer from a global variable and calls it, passing a pointer to a structure located on the stack as an argument.
- `store_byte_and_increment`: The library code stores a byte from register d0 at the address in a3 and then increments a3.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_and_decrement_pointer`: Loads a pointer from the stack, compares a byte from that pointer with a value from the stack, and if they are equal, decrements the pointer by 1 and returns the new pointer in d0; otherwise it returns.
- `branch_to_offset_6`: Library code that clears register d0 and branches to a specific offset.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `skip_zero_byte`: The library code restores registers a0 and a1 from the stack, checks a byte at the address in a0, and if that byte is not zero, it branches to another routine; otherwise, it decrements a0 and branches to a different location.
- `compare_memory_bytes`: Decrements a value from the stack, and if it is not less than zero, compares a byte from a source pointer to a byte from a destination pointer, potentially setting d0 to zero if they match and the source byte is zero.
- `set_d0_based_on_comparison`: Sets d0 to one if a comparison is not greater than zero, otherwise it sets d0 to 255 and returns.
- `test_and_decrement_count`: Tests a byte at the address in a0, and if non-zero, decrements a value from the stack and branches based on whether the result is greater than zero.
- `decrement_a0_and_copy`: Decrements the address in a0 and calls a routine to copy bytes.
- `copy_bytes_with_counter`: The library code decrements d0 and, if d0 is not less than zero, copies a byte from the address in a1 to the address in a0 and repeats this loop while d0 is not zero; it then decrements d0 again and, if d0 is not less than or equal to zero, clears a byte at the address in a0 and branches to another location.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain_3`: The library code stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `conditional_library_branching`: The library code checks a parameter and, if non-zero, calls another routine and potentially calls a third routine using a value from a pointer offset.
- `traverse_icon_list`: Iterates through a list by calling a library routine with a pointer from a structure and a calculated value, updating the pointer to the next element in the list until a zero result or a null pointer is encountered.
- `set_d0_and_branch_2`: Sets d0 to 1 and branches to another routine.
- `restore_registers_and_return_alt`: Clears d0 and restores d2, d3, d4, d5, and a2 from the stack before returning.
- `validate_and_store_result`: Library code that calls another routine multiple times, performs arithmetic, and stores a result at an offset of 10 in a structure pointed to by a register.
- `return_negative_one_2`: This library routine sets d0 to negative one and restores registers from the stack before returning.
- `process_nested_structure`: Library code that executes a loop, calling another routine multiple times to retrieve values, storing a result at offset 8 of a structure, and returning negative one if a call fails or a specific condition is met.
- `process_nested_loops`: Library code that executes a nested loop structure, calling another routine multiple times and storing values at offsets of 8 and 12 in a structure.
- `move_a3_to_d0_and_branch`: Moves the contents of a3 into d0 and branches to another routine.
- `return_negative_one_extended`: This library routine sets d0 to negative one and restores a larger set of registers from the stack before returning.

### translator.library

- `execute_library_callback`: Checks a global variable and, if the value at an offset of negative 84 from it is non-zero, stores the global variable and a value from a register into a structure, then calls a function through a pointer at an offset of negative 368 from that structure before returning either zero or negative one.
- `clear_flag_and_increment_counter_2`: Increments a 16-bit counter in the structure it is given and clears bit 5 of that structure's flag byte.
- `decrement_counter_call_on_zero`: Decrements a 16-bit counter in a structure at offset 32 of a6.
- `check_bit_and_branch`: Saves registers to the stack, moves a pointer from a6 to a3, loads a new address into a6 from the stack, tests a 16-bit word at an offset of 32 from a3, and if it is zero branches to another location; otherwise it sets bit 5 of a byte at offset 14 of the structure in a3 and branches to another location.
- `unlink_node_and_call_library`: Removes the node from its doubly linked list, then calls a function through the Amiga library base and returns a value stored in the node.

The function list continues in: g1449-recovery-functions-2.md.
