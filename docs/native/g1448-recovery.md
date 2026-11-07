# Game 1448 (balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01448). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916908 top-level bytes in 78 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 52 of 52 units identical (bytes and relocations), 60 raw-word lines |
| Functions named and described | pass, 2268 of 2268 named (278 author symbols), 2268 of 2268 described (427 by rule, 1793 by the local model) |
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
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf:stack | 2 | 117 | 2 |
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
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf:CLI | 9 | 362 | 18 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 15 | 1414 | 108 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2476 | 170 |
| mathieeedoubbas.library | 3 | 632 | 60 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 13 | 830 | 53 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf:bop | 3 | 5064 | 305 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1700 | 37 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 150 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf bytes 603136-603648 (code never loaded by a reference run) | 1 | 28 | 1 |
| SavedGame (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m2]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf: file devs/printers/Okidata_92 (header 5476 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1478 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 93 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 20 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m2]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf: bytes 603136-603648 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m2]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf:asave/SavedGame: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m2]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf: 1 logged loads cover 512 bytes of the image
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx][m2]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf: blocks 112-120 (bytes 57344-61952) outside the filesystem hold data, not code (opcode density 0.025, entropy 4.98)
- 1 never-loaded region(s), 192665 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf:stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 into d0 and then branches to another location.

### Format

- `parse_command_string`: Parses a string into space-separated tokens, handling quoted substrings and converting specific character sequences into newline or escape characters, then stores the resulting tokens in a table of pointers.
- `execute_conditional_callbacks`: Checks two global variables for non-zero values and, if non-zero, calls function pointers at specific offsets from a global variable and a structure in a6.
- `invoke_two_function_pointers`: Loads a pointer from an offset of a4 into a0 and calls a function pointer at a negative offset from a6, then repeats this with a different negative offset from a6.
- `execute_conditional_callbacks_3`: Loads an address into a1, clears d0, calls a routine, stores the result in a global variable, and returns if the result was not zero.
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
- `traverse_and_invoke_table`: Iterates through a table of pointers, checking for null or empty entries, and calls a routine to compare strings and another routine to invoke a table entry.
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
- `rotate_32bit_value_3`: Performs a bitwise rotation and division on two 32-bit values using a loop that shifts bits and handles carries.
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

- `parse_command_string_2`: Iterates through a string to extract tokens, handling quoted text and converting specific character sequences into newline or escape characters.
- `execute_conditional_callbacks_2`: Stores a parameter into a stack-based location, then checks two global variables to conditionally call routines at offsets within a structure pointed to by a global variable.
- `invoke_two_function_pointers_2`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `execute_conditional_callbacks_2_2`: Calls a routine at an offset from a6, stores the result in a location at offset 8 of a structure, and returns if the result is zero; otherwise it returns.
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
- `update_structure_fields_3`: Stores a 16-bit value at offset 28, a 32-bit value at offset 36, and a 32-bit value at offset 40 of a structure, then calls another routine.
- `update_structure_fields_2`: Stores a 16-bit value at offset 28, a 32-bit value at offset 36, and a 32-bit value at offset 40 of a structure, calls another routine, and then adds the result of that call to a value at offset 44 of the structure.
- `synchronize_disk_contents`: Performs a disk-to-disk copy operation by iterating through file structures on a source disk and replicating them to a destination disk.
- `synchronize_disk_contents_2`: Performs a disk-to-disk copy operation by iterating through file structures on a source disk and replicating them to a destination disk.
- `execute_cleanup_sequence`: Checks a global variable.
- `cleanup_system_resources`: Checks several global pointers and executes various cleanup routines, including deleting port structures, deleting extended I/O, calling library functions to free memory or close libraries, and managing error message tasks.
- `extract_message_data`: Copies three values from a structure at a pointer in a global variable into three other global variables, calls another routine with that structure, and then calls another routine with a global variable if the first copied value is not 512.
- `execute_library_call_3`: Loads a value from the stack into d0, pushes a value from a global variable and d0 onto the stack, calls a routine at an offset from a global variable, and if the result is not zero, it branches.
- `perform_disk_copy_loop`: Iterates through a table of data blocks to perform disk I/O operations, including reading from a source and writing to a destination.
- `allocate_multiple_libraries`: Checks a value at an offset of 20 from a global variable; if it is less than 33, it calls a routine with a fixed address and then calls a routine with 0.
- `initialize_disk_copy_params`: Populates a stack-based structure with several constants and then calls two library functions using a global base.
- `report_disk_errors_2`: Depending on the value of a byte parameter, it sets a global pointer to a global variable and calls another routine with various parameters, including handling specific cases for parameter values 0, 1, and values between 21 and 30, before calling another routine twice with different parameters and returning 0.
- `report_disk_errors`: Sets a pointer to a global variable and calls another routine with a global value, a parameter, and a stack address, then repeats this with a different constant and calls another routine with a single constant before returning.
- `process_io_and_status`: Calls a routine and checks the result; if the result is zero, it calls another routine with a pointer from a structure and a value from a table.
- `check_signal_status`: Calls another routine with two parameters, checks if the 12th bit of the result is set, and if so, calls another routine with a global variable; otherwise, it returns the bit status.
- `process_message_queue`: If a global variable is non-zero, it enters a loop to process messages via a routine that returns a value, then extracts data from a structure pointed to by a global variable and returns a value based on specific fields within that structure; otherwise, it extracts data from a structure pointed to by a global variable and returns a value based on its contents.
- `set_parameter_zero`: Stores zero in a 32-bit word at offset 36 from a pointer and nine in a 16-bit word at offset 28 from that same pointer before calling another routine.
- `manage_error_message_task`: Allocates memory, populates the resulting structure with various values and pointers, calls a routine to handle the structure, and then frees the memory.
- `set_parameter_and_call`: Writes the constant 4 to offset 28 of a structure, then calls another routine twice with the constant 5 written to offset 28 before the first call.
- `execute_multiple_calls`: Pushes two global longwords onto the stack and calls a routine twice for each value, then cleans up the stack and returns.
- `process_disk_data`: Calls another routine with a constant and a value, then if the result is non-zero, it performs several calls using a pointer from a global variable, updates a global variable with a pointer, modifies a byte at an offset of the pointer, calculates a sum by looping through 128 words in a table, subtracts that sum from a value at offset 5 of the pointer, and calls another routine with a constant and the pointer.
- `perform_io_operation`: Calls another routine with four parameters, then calls another routine and returns whether its result is zero.
- `compare_and_io`: Performs an I/O operation, calls another routine to update a structure, and returns whether a parameter matches a value stored in a global variable.
- `allocate_message_entry_2`: Attempts to find a task, allocates memory, and if successful, populates a message buffer with a value from a parameter, a byte from another parameter, and a constant, then calls another routine to set a pointer and dispatches the buffer.
- `delete_port_2`: If a field at offset 10 of the provided pointer is non-zero, it calls another routine, sets a byte at offset 8 to 255, sets a long at offset 20 to negative one, frees a signal, and removes a port.
- `create_ext_io_2`: If the first argument is non-zero, it allocates memory with a specific flag and, if successful, sets a byte at offset 8 to 5, a short at offset 18 to the second argument, and a long at offset 14 to the first argument.
- `delete_ext_io_2`: If the provided pointer is non-zero, it sets a byte at offset 8 to 255, and sets longs at offsets 20 and 24 to negative one, then frees the memory block using a size stored at offset 18.
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
- `pass_registers_to_helper`: Moves the contents of two 32-bit registers into a stack frame, calls a helper routine, and then restores the first register from the stack before returning.
- `call_modular_inverse_and_return_d1`: Moves the contents of register d0 into d1, calls modular_inverse_32bit, and returns the resulting value from d1 in d0.
- `negate_modular_result`: Calls modular_inverse_32bit and negates the returned value if the helper leaves a nonzero value in an address register.
- `call_and_negate_on_flag_2`: Calls modular_inverse_32bit and returns its first result, or the negation of that result if the second result in an address register is nonzero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment_pointer`: Stores a byte from register d0 into the memory location pointed to by register a3 and then increments the pointer by one before returning.
- `call_formatted_print`: Pushes a pointer and a value onto the stack and calls a routine to print a formatted message, then returns the result in a register.
- `pass_params_to_subroutine_2`: Pushes a global value, a parameter, and a pointer to an eight-byte stack slot onto the stack before calling another routine and returning the result in a register.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_global_library_function_2`: Calls a function through a pointer located at a fixed offset from a global variable and returns.
- `execute_with_status_check_2`: Calls a routine via a global pointer and returns either a byte from a stack slot if the result is 1 or a negative error code otherwise.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.

### Port-Handler

- `parse_amigados_hunks`: Reads a structure containing hunk type codes and sizes, then iterates through a table to initialize memory segments based on whether the type is an executable, relocation, or data hunk.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a table, then enters a loop to clear a range of memory and performs bitwise operations to update a data structure.

### Ram-Handler

- `execute_jump_table`: Sets d0 to twelve, saves registers, calls a routine via a pointer at an offset of a6, restores registers, and performs an indirect jump using a pointer in a6.

###            

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with sequential words.
- `move_and_branch_2`: Moves the contents of d1 into d0 and then performs a short branch to another location.

### serial.device

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reinit_serial_buffer_fields`: Calls a routine via a pointer in a6 if a1 is non-zero, then updates several fields in a structure at a6 with a value from d2, including setting half-size and quarter-size values.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `set_byte_and_branch`: Stores the byte value fd at offset 31 of a structure in a1 and then performs a short branch.
- `toggle_interrupt_enable`: Writes 3277 to the interrupt enable register, calls another routine, and then writes 34817 to the interrupt enable register.
- `check_serial_command`: Clears a byte at offset 31 of a structure in a1 and branches to another routine if the word at offset 28 of a1 is 2, 3, or 10.
- `compare_and_clear_pointer`: Compares a longword at offset 74 of a structure in a6 with a pointer in a1; if they are equal, it clears bit 1 of a byte at offset 454 of the structure in a6, clears the longword at offset 74 of the structure in a6, and then branches to another routine.
- `unlink_list_node`: Tests bit 6 of a byte at offset 30 of a structure in a1; if set, it unlinks a node by updating pointers in a list and then decrements a 16-bit counter at offset 70 of a structure in a6.
- `reset_serial_hardware`: Compares a pointer in a1 to a value at offset 78 of a structure in a6; if they match and a 16-bit value at offset 28 of the structure in a1 is 10, it clears bit 5 of a byte at offset 470 of the structure in a6, writes 800 to a hardware register, calls another routine, clears the value at offset 78 of the structure in a6, and clears bit 4 of a byte at offset 472 of the structure in a6.
- `unlink_second_queue_request`: If a bit is set in a byte at offset 30 of a structure in a1, it unlinks that structure from a list, decrements a counter in a6, and calls a routine via a pointer in a6 if a bit is not set.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `set_status_bit_3`: Sets bit 6 of a byte at offset 472 of a structure in a6 if a 16-bit value at offset 32 of the structure in a6 is non-zero; otherwise it returns.
- `invoke_library_vectors_then_unlink`: Calls several routines via pointers in a6 and a structure in a6, copies a longword from a structure in a6, and unlinks a block from a list by updating pointers.
- `manage_linked_list_node`: Sets a byte at offset 8 of a structure to zero, then if a byte at offset 9 is non-zero, it conditionally clears bit 1 of a byte at offset 30 and sets bit 6 of that same byte before inserting the structure into a linked list and incrementing a 16-bit value at offset 70 of another structure.
- `process_serial_data_buffer_queue`: Iterates through a linked list of data buffers, decrementing a counter and updating status bits in each buffer structure.
- `process_serial_data_stream`: Iterates through a data buffer to extract bytes, updating a pointer and a counter within a structure in a6.
- `test_serial_bit`: Tests bit 0 of a byte at offset 445 of a structure in a6 and branches to another routine if the bit is set, otherwise it branches to another routine.
- `branch_and_set_byte`: Calls another routine, and if the result is zero, sets a byte at offset 31 of a structure in a1 to 9 before branching to another routine; otherwise it branches to another routine.
- `process_serial_data_buffer`: Iterates through a linked list of data buffers to process bytes, updating status flags and counters within a structure pointed to by a6.
- `restore_registers_and_return_7`: Returns immediately after restoring registers d2 and a6 from the stack.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `clear_status_bit_2`: Clears bit 2 of a byte at offset 470 of a structure in a6, and if the bit was not set, writes the value 15 to offset 31 of a structure in a1, then branches to another routine.
- `clear_serial_bit`: Clears bit 7 of a byte at offset 470 of a structure in a6 and branches to another routine if the bit was set, otherwise it writes the value 9 to offset 31 of a structure in a1.
- `update_status_and_clear_long`: Sets a byte at offset 31 of a structure in a1 to 12, tests bit 0 of a byte at offset 470 of a structure in a6, and if zero, sets a byte at offset 31 of a structure in a1 to 6 and clears a longword at offset 62 of the structure in a6 before branching to another routine; otherwise it performs a bitwise AND on a byte at offset 470 of the structure in a6 and branches to another routine.
- `process_serial_data_and_status`: Checks status bits in a structure to manage serial communication, updates a data buffer by copying a byte from a pointer and incrementing buffer indices, and calls another routine via a pointer in the structure.
- `toggle_serial_interrupt`: Compares a byte at offset 34 of the structure in a6 with d1 and branches; if not equal, it tests bit 1 of a byte at offset 472 of the structure in a6, and if set, clears that bit and enables interrupts by writing to the INTENA register, otherwise it returns.
- `test_serial_status`: Tests bit 5 of a byte at offset 472 of a structure in a6 and branches if set; otherwise it tests bit 1 of a byte at offset 445 of the structure in a6, calls another routine, and then tests bit 8 of d0 before returning.
- `test_serial_bit_2`: Tests bit 8 of d0 and branches to another routine if the bit is set, otherwise it branches to another routine.
- `test_status_bit`: Tests bit 0 of a byte at offset 445 of the structure in a6 and branches to another routine if the bit is set.
- `set_serial_pointer`: Sets bit 7 of a byte at offset 470 of a structure in a6 and, if a longword at offset 62 of a6 is zero, copies a longword from offset 42 to offset 62.
- `process_serial_buffer`: Compares a longword at offset 54 of a structure in a6 with a longword at offset 58, and if the latter is smaller, updates a pointer and increments two counters.
- `record_pointer_then_signal`: Checks if a longword at offset 62 of a structure in a6 is zero; if it is, it copies a longword from offset 42 to offset 62, calls another routine, and then calls a library vector using a pointer from offset 98 of the structure in a6.
- `clear_interrupt_bit`: Copies a byte from offset 34 of a structure in a6 to offset 473 of the same structure, clears bit 3 of a byte at offset 472 of the structure in a6, tests bit 2 of a byte at offset 69 of a structure in a6, and if zero, clears bit 6 of the hardware register at address bf d000.
- `update_serial_interrupt_state`: Copies a byte from a structure offset to another offset, sets bit 3 of a structure byte, and conditionally modifies hardware interrupt enable and CIA port A settings based on bits in a structure.
- `clear_status_byte_2`: Clears a byte at offset 473 of the structure pointed to by a6 and returns.
- `update_status_and_signal`: Checks if a byte at offset 470 of a structure in a6 has bit 6 set; if not, it sets that bit, clears a longword at offset 418, copies a longword from offset 66 to offset 418, subtracts 10 from that longword, sets a 16-bit value at offset 414 to 9, and calls another routine.
- `modify_interrupt_status`: Saves registers d2, a3, and a6 to the stack, clears bit 6 of a byte at offset 470 of a structure in a6, and if the bit was zero, checks if the low 10 bits of the serial data register are zero before setting bit 2 of the byte at offset 470 of the structure in a6.
- `load_address_and_call`: Loads the address from offset 386 of the structure in a6 into a0, calls another routine, and then branches to another routine.
- `set_status_bit_4`: Sets bit 0 of a byte at offset 470 of a structure in a6 and branches to another routine.
- `manage_serial_request_queue`: Clears a value at offset 8 of a structure and, if a flag is set, either clears bit 1 of a status byte and attempts to queue a new request by updating a linked list and incrementing a counter, or sets bit 4 of the status byte and updates several fields in a structure.
- `update_interrupt_state_and_enable`: Tests a bit in a structure, calls another routine if set, then checks a byte in a structure to either set bit 4 at offset 30 or clear a longword at offset 94 of a global structure, before finally enabling interrupts via the INTENA and INTREQ registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_request_state_on_flags`: Checks if bits 3 and 4 of a global variable are set, and if so, writes a byte with a value of thirteen to an offset of 31 within a structure pointed to by a1.
- `manage_serial_request_queue_2`: Manages a linked list of serial request structures by popping the head of the list, updating status flags, and decrementing a request counter.
- `update_serial_state_2`: Writes 200 to offset 476 of a structure in a6, loads a pointer from offset 86 of a6 into a0, clears d0, and loads a byte from the pointer into d0; it then compares a longword at offset 36 of a1 with d1, tests bit 4 of offset 472 of a6, and decrements a longword at offset 94 of a6 before branching.
- `process_serial_event_2`: Updates several values in a structure in a6, including incrementing two longwords at offsets 86 and 94, and performs a loop that compares bytes in a buffer against a value derived from a structure member.
- `process_serial_event`: Calls another routine, modifies a value based on a bitmask from a byte at offset 67 of a structure in a6, updates a global variable, increments two longwords at offsets 86 and 94 of the structure, and if a bit is set, iterates through a list to compare values before potentially calling another routine.
- `decrement_list_count`: If a shortword at offset 72 of a structure in a6 is not zero, it manipulates a linked list starting at offset 176 of the structure in a6, updates a pointer at offset 78 of the structure in a6, sets bit 4 and clears bit 7 of a byte at offset 30 of a structure in a1, and clears a longword at offset 94 of the structure in a6; otherwise it sets a global variable to 1.
- `enable_interrupts`: Restores registers d2 and a6 from the stack, sets bit 0 of the interrupt enable register, and returns.
- `update_serial_interrupt_mask`: Enables the serial interrupt, reads two bytes from a structure in a1, combines them, writes the result to the interrupt mask register, clears a byte in the structure, and otherwise returns.
- `reset_serial_parameters`: Writes to the interrupt enable register, calls several other routines to validate and sync parameters, updates several longwords in a structure, and branches to a handler routine if a specific bitmask test passes.
- `call_handler_and_branch`: Calls a routine via a branch to stack and then performs an unconditional branch to another location.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `signal_and_call_handler`: Checks if a longword at offset 78 of a structure in a6 is non-zero and, if so, sets bit 2 of a byte at offset 472 of the structure in a6; then, if a longword at offset 74 of the structure in a6 is non-zero, it calls another routine and checks if bit 0 or bit 4 of a byte at offset 30 of the resulting structure in a1 are set before calling a library vector via a pointer in a structure in a6.
- `clear_interrupt_and_call`: Clears bit 3 of a byte at offset 472 of a structure in a6, checks if a longword at offset 74 is non-zero to call another routine, and if a specific bitmask matches a byte from a pointer in a1, calls another routine and writes 32769 to the INTREQ register.
- `set_interrupt_mask_and_call`: Writes a 16-bit value to the interrupt enable register, calls a routine via a branch to stack, writes a different 16-bit value to the interrupt enable register, and performs an unconditional branch.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `update_pointer_and_loop`: Copies the value from a0 to a1, then if the value in a1 is non-zero, it stores it in a structure at offset 0, exchanges d0 and a1, stores a0 at offset 4 of a1, and enters a loop that calls a routine and branches back to the start of the routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_serial_state`: Calculates a byte value based on a hardware register and two bytes from a structure, then writes this byte to offset 81 of a structure, copies a byte from offset 470 of another structure to offset 80, clears bit 1 of that byte, updates a 32-bit value at offset 32, and clears bit 1 of a byte at offset 470 before potentially calling a routine via a pointer.
- `test_request_status`: Clears bit 0 of a byte at offset 30 of a structure in a1, tests bit 3 of a byte at offset 79 of the same structure, and if bit 3 is not set, tests if a word at offset 72 or a longword at offset 78 of a structure in a6 is non-zero.
- `process_request_and_hardware`: Moves a 32-bit value from offset 106 of the structure in a6 to d0, calls another routine, tests a byte at offset 31 of the structure in a1 and, if non-zero, branches to another routine; otherwise, it copies a1 to offset 78 of the structure in a6, sets bit 5 of offset 30 of the structure in a6, moves the address of a custom chip register to a0, writes a value to that register, and branches to another routine.
- `reply_and_clear_active_request`: Pushes a6 to the stack, updates a6 using a pointer at offset 98, calls a routine via a pointer, restores a6 from the stack, clears a 32-bit value at offset 78 of the structure in a6, and returns.
- `clear_uart_break_and_reply`: Clears bit 5 of a byte at offset 1d6 of a structure in a1, and if bit 5 of that byte is set, a non-zero value is in a6, and a short at offset 1c of a pointer in a1 is 10, it sets bit 1 of the byte, clears a pointer in a1, writes 2048 to a global variable, calls a routine via a pointer, and sets a global variable to 32769.
- `start_timed_read_request`: Sets a global variable to one, clears a 32-bit value at offset 142 of a structure in a6, stores a 32-bit value from d0 at offset 146, sets a 16-bit value at offset 13e to nine, calls a routine via a pointer, and if a byte at offset 1f of a1 is non-zero, sets that byte to 11.
- `modify_interrupt_status_2`: Tests bit 7 of a byte at offset 79 of a1 and branches if clear; otherwise it sets bit 7 of a byte at offset 69 of a6, clears bit 2 of a byte at offset 472 of a6, tests bit 3 of offset 472 of a6, and if set, clears bit 7 of offset 79 of a1, calls another routine, and sets bit 7 of offset 79 of a1.
- `test_and_update_status`: Clears bit 7 of a byte at offset 69 of a structure in a6, then checks bit 1 of a byte at offset 474 of a structure in a6 and bit 4 of a byte at offset 478 of a structure in a1.
- `toggle_interrupt_and_sync`: Writes 1985 to the INTENA register, calls another routine, writes 34849 to the INTENA register, and if a bitmask matches a byte at offset 30 of a structure in a1, calls another routine.
- `validate_and_sync_serial_parameters`: Validates a set of character and integer parameters within a structure, copies the validated values to a secondary structure, updates status flags based on specific bit patterns, and calls reinit_serial_buffer_fields and calculate_period_quotient to synchronize the device state.
- `compute_unit_channel_masks`: Calculates a 16-bit mask based on a byte at offset 4d of a1, modifies it based on a byte at offset 4e of a1 and a bit in a byte at offset 45 of a6, stores the result in a structure in a6, and calls two other routines to store results at offsets 1d2 and 1d0.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_byte_and_branch_2`: Sets bit 2 of a byte at offset 31 of the structure in a1 and branches to another routine.
- `calculate_period_quotient`: Loads a 32-bit value from offset 450 of the structure in a6 into d0 and d1, performs arithmetic operations including a subtraction and a division by a 16-bit value, and branches to another routine.
- `store_serial_period_quotient`: Divides a 32-bit value in d1 by a 16-bit value in d0, stores the quotient in a structure in a6 and a global variable, clears bit 5 of a byte at offset 1d8 of a6, and if a byte at offset 42 of a6 is 8 and bit 0 of a byte at offset 45 of a6 is set, it performs a bitwise OR on the quotient and sets bit 5 of the byte at offset 1d8.
- `configure_serial_device`: Loads two different addresses into a0 and a3, calls another routine twice, and then branches to another location.
- `configure_device_io`: Loads two different addresses into a0 and a3, then calls another routine twice before branching to another location.
- `store_pointer_to_offset`: Stores the value in a0 into a 32-bit word at offset 10 of a structure in a1.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `configure_serial_device_state`: Updates status bytes and pointers within a structure in a6, then calls a library function via a pointer and stores the results in the structure.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_d0_and_branch`: Clears d0 and branches to another location.

### parallel.device

- `update_parallel_device_status`: Checks specific bits in two structures, increments a 16-bit value at offset 32 of the first structure if certain conditions are met, copies a byte from offset 117 to offset 53, and conditionally copies 8-bit values between the structures.
- `update_buffer_and_status`: Increments a 16-bit value at offset 32 of the structure in a6, copies a byte from offset 117 of the structure in a6 to offset 53 of the structure in a1, sets bit 4 of offset 118 of the structure in a6, and if bit 1 of the byte at offset 53 of the structure in a1 is set, copies two 32-bit values from the structure in a6 to the structure in a1.
- `update_status_and_copy_data`: Clears a byte at offset 76 of a structure in a6, conditionally sets bit 5 of offset 75, increments a 16-bit counter at offset 20 of a6 if two bytes are zero, copies a byte from offset 35 of a1 to offset 75 of a6, sets bit 4 of offset 76, and conditionally copies two 32-bit words from a6 to a1.
- `process_parallel_interrupt_status`: Checks a status word at offset 28 of a structure and calls a routine if a specific bitmask is met.
- `handle_parallel_interrupt`: If a short at offset 1c of a1 is 3, clears bit 0 of a global variable, and if bit 0 is zero, checks bit 2 of a byte at offset 76 of a structure in a6; if bit 2 is not set, it calls a routine via a pointer, then sets bit 2 of the byte at offset 76, and finally calls another routine via a pointer using a1 and a6.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `update_parallel_device_status_2`: Writes the byte 253 to offset 31 of a structure pointed to by a1 and branches to another routine.
- `check_device_pointer`: Tests a longword at offset 36 of a structure in a1; if it is not zero, it branches to another routine, otherwise it clears a longword at offset 32 of the structure in a1 and branches to another routine.
- `update_device_list`: Clears bit 1 of a byte at offset 30 of a structure in a1, then checks bit 2 of a byte at offset 118 of a structure in a6.
- `update_device_status`: Sets several fields in a structure, modifies a global variable, and performs bitwise operations on memory values.
- `process_parallel_device_queue`: Manages a linked list of pending device operations by decrementing a counter and updating pointers, then performs data transfers or status updates by comparing values in a list and calling handle_parallel_transfer, update_device_status, or clear_device_status.
- `handle_parallel_transfer`: Checks a bitmask in a structure, performs a loop to compare values in a list, updates counters and status bytes in a structure, and calls another routine or a second routine if certain conditions are met.
- `clear_device_status`: Copies a 32-bit value from offset 46 of a structure in a6 to offset 32 of a structure in a1, clears bit 3 of a byte at offset 118 of a6, and clears bit 1 of a byte at offset 30 of a1.
- `test_cia_register`: Moves a byte from a0 to offset 193 of a CIA register, then branches to another routine if the value is not zero; otherwise, it tests bit 3 of a byte at offset 118 of a structure in a6 and branches to another routine if set, or branches to another routine if not set.
- `transfer_data_and_status`: Calls another routine, then calls a routine using a pointer from a structure in a6, sets a byte at offset 118 of the structure in a6 to 0, conditionally sets bit 5 of a byte at offset 117 of a structure in a6, and potentially increments a 16-bit value at offset 32 of the structure in a6 and copies data from the structure in a6 to the structure in a1.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_if_not_zero`: Checks if a 32-bit value at offset 26 of a structure in a6 is non-zero and, if so, calls a routine via a pointer located at an offset from a value at offset 22 of a6.
- `invoke_callback_pointer`: Executes a function through a pointer stored in a structure if the associated index is not zero.
- `transfer_pointer_data`: Loads a pointer from offset 76 of a structure in a6 into a0, then moves the value at the pointer in a1 into d0.
- `clear_word_field`: Clears a 16-bit word at offset 114 of a structure in a6 and returns.
- `modify_status_byte`: Sets bit 5 of a byte at offset 30 of a structure in a1, sets a byte at offset 31 of the structure to 254, and calls a routine via a pointer stored in a structure in a6.
- `update_global_and_field`: Manipulates bits within a global variable and updates a byte in a structure using a library function call.
- `set_status_byte_2`: Tests bit 2 of a byte at offset 118 of a structure in a6; if it is not set, it writes the byte value 1 to offset 31 of a structure in a1 and branches to another routine.
- `copy_structure_fields`: Copies a byte from one structure to another and, if a specific bit is set, copies two long words as well.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `recurse_on_pointer`: Saves registers a0 and a6 to the stack, calls another routine via a pointer, restores the stack pointer, and returns.
- `recursively_calls_itself`: Calls itself and then returns.
- `dispatch_clipboard_handler`: Pushes the current stack pointer to the stack and calls a handler pointer retrieved from a data structure, then adjusts the stack pointer before returning.
- `jump_to_handler`: Performs a long jump to an address at offset 246 of a structure.
- `call_external_handler_2`: Pushes the contents of a1 onto the stack, calls a routine at a relative offset, cleans up the stack, and returns.
- `push_and_call_handler`: Pushes the contents of a1 onto the stack and calls another routine via a pointer.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `execute_library_calls_5`: Saves registers to the stack, loads values from the stack into d3 and a2, stores a2 into a global variable, clears the stack, pushes a pointer to a structure, calls another routine, stores d0 into a global variable, adjusts the stack pointer, and branches to another routine if d0 is not zero.
- `process_clipboard_data`: Stores a value into a global variable, calls three different routines, and returns a boolean value based on whether the third routine call returned zero.
- `call_two_functions`: Calls two different functions via library vectors and returns one.
- `process_clipboard_string`: Calls another routine, then calls another routine with two arguments; if the result is zero, it stores a word of 0 at offset 94 of a structure in a1, otherwise it conditionally sets a 32-bit value at offset 14 of a structure in a1 based on a value at offset 4 of a structure pointed to by a1, and calls another routine with a1 and the result as arguments.
- `call_library_function_3`: Clears a 32-bit value at offset 14 of a3, pushes a3 onto the stack, calls a routine via a pointer at offset 72 of a2, and then cleans up the stack.
- `zero_word_field`: Stores a zero word into a specific offset within a structure and then calls another routine.
- `update_pointer_chain_2`: Saves registers to the stack, loads a pointer from the stack into a2, calls another routine, loads a pointer from offset 72 of the structure in a2 into a0, loads a longword from the address in a0 into d2, and if d2 is zero, it pushes a value onto the stack and calls another routine; otherwise, it pushes a value onto the stack, calls another routine, and branches to another routine.
- `update_pointer_chain`: Pushes the address at offset 72 of a2 onto the stack, calls a routine, clears a 32-bit value at offset 86 of a2, calls another routine, and then restores d2 and a2 from the stack.
- `copy_buffer_segment`: Copies a segment of data from a linked list of 512-byte buffers into a destination buffer, using an offset and length to determine the source range.
- `compare_clipboard_values`: Compares a longword from a structure at offset 90 of a2 with a longword on the stack, and if the stack value is not lower, performs arithmetic on a longword from offset 44 of the stack and compares it to the value from a2.
- `copy_clipboard_data_2`: Traverses a linked list of structures to find a specific node based on a value derived from the stack, then copies bytes from the structure at offset 18 of the found node to a destination buffer using a loop that handles remaining capacity in 512-byte chunks, and finally updates a pointer in a structure in a2.
- `compare_and_branch_value`: Moves d1 to d0, compares d0 with a longword at offset 14 of a structure pointed to by a1, and branches if d0 is lower than or equal to that value; otherwise, it updates a1 and branches to another location.
- `compare_and_branch_limit`: Copies a longword from d3 to d5, checks if d5 is zero, and if not, calculates a limit by subtracting d2 from 512, compares d5 to this limit, and branches based on the result.
- `copy_data_loop`: Copies bytes from a table starting at offset 18 of a1 into a destination pointed to by a3, using d4 as an index, and repeats this in a loop until d4 reaches 512.
- `move_and_branch_4`: Moves a 16-bit word from register d2 into d4 and then branches to another routine.
- `copy_data_with_limit`: Copies bytes from a table starting at offset 18 of a1 into a destination pointed to by a3, using d4 as an index, and repeats this in a loop until a comparison between a 32-bit value in d0 and the sum of d1 and d2 fails.
- `store_long_word`: Stores a long word into a specific offset within a structure and then calls another routine.
- `reallocate_and_append_clipboard_buffer`: Checks if a clipboard buffer requires reallocation based on size constraints, then iterates through a linked list of memory blocks to append data from a source buffer into the appropriate segments, handling block overflows and list traversal.
- `copy_clipboard_data_to_buffer`: Iterates through a linked list of memory blocks to copy data from a source buffer into the blocks.
- `branch_to_offset_48`: Pushes the address of a location at offset 72 of a structure in a2 onto the stack, calls a routine at offset 0, adjusts the stack pointer, and branches to a location at offset 1046.
- `append_clipboard_buffer_to_list`: Iterates through a linked list of structures to find a suitable insertion point based on a value derived from a bitwise operation on a field in a structure.
- `transfer_clipboard_buffer_2`: Calls three different routines and performs a loop that copies bytes from a source to a linked list of structures based on values derived from a structure in a6.
- `transfer_clipboard_buffer`: Traverses a linked list of structures while a condition involving a longword at offset 14 of a structure in a6 is met, then copies bytes from a source pointer to the structures in the list using a loop that handles 512-byte boundaries, updates the remaining count in a structure in a6, and updates a pointer in a structure in a2.
- `check_buffer_limits`: Tests d4 and branches if zero, tests a longword at offset 20 of a structure in a6 and branches if lower or equal, calculates a value by subtracting d3 from 512, compares it to a longword at offset 20 of the structure in a6, and branches based on the carry flag.
- `copy_clipboard_data`: Copies bytes from a source pointer in a3 to a destination pointer in a4 while managing a length counter and potentially calling another routine to handle buffer exhaustion or allocation.
- `move_and_branch_5`: Moves a 16-bit word from register d3 into d1 and then branches to another routine.
- `copy_buffer_data`: Copies bytes from a source pointer to a destination buffer, potentially calling another routine to retrieve a new source pointer when the current one is null or empty.
- `store_register_to_offset`: Stores the contents of a data register into a memory location calculated by adding a fixed offset to a pointer held in an address register.
- `check_status_bits`: Calls another routine, then checks bit 7 and bit 0 of a byte at offset 30 of a structure in a2, and calls another routine if bit 0 is not set.
- `modify_byte_bits`: Clears the least significant bit and sets the most significant bit of a byte at a specific offset in a buffer.
- `modify_byte_bits_2`: Calls another routine, then checks bit 7 of a byte at offset 30 of a structure in a2 and, if not set, clears bit 1 of that byte; it then calls another routine, clears bit 3 of the same byte, and returns.
- `compare_structure_values`: Compares a long value at offset 48 of a structure in a1 with a long value at offset 48 of a structure in a0, and returns 1 in d0 if the value in a0 is less than the value in a1.
- `set_d0_to_zero_and_branch`: Sets register d0 to zero and performs a short branch to another routine.
- `clear_d0_and_branch_2`: Sets d0 to zero and performs a short branch to another location.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_structure_fields_2`: Copies a 32-bit value from offset 40 of a second structure to offset 152 of a first structure, a 16-bit value from offset 16 of the second structure to offset 146 of the first, and a 32-bit value from offset 48 of the second structure to offset 148 of the first, then calls another routine.
- `update_clipboard_bit_status`: Sets bit 5 of a byte at offset 30 of a structure in a3, then checks a byte at offset 66 of a structure in a2; if the byte is non-zero, it clears bit 5 of the byte at offset 30 of the structure in a3, otherwise it returns.
- `update_clipboard_state_machine_3`: Executes a state machine loop by checking a status field at offset 64 of a structure in a2.
- `clear_d3_and_branch_2`: Clears the 16-bit d3 register and branches to another routine.
- `update_clipboard_state_machine_2`: Updates a state machine by checking a status field in a structure and performing different operations based on its value.
- `push_pointer_and_call`: Pushes the pointer in a2 onto the stack, calls another routine, adjusts the stack pointer, writes 1 to offset 94 of the structure pointed to by a2, and branches to another routine.
- `process_clipboard_state_machine`: Updates a state machine by checking a status field in a structure and performing different operations based on its value.
- `clear_d3_and_branch`: Clears the 16-bit register d3 and branches to another routine.
- `update_clipboard_state_machine`: Processes a state machine by reading a structure via a pointer in a structure, updating buffer offsets, counters, and linked list segments based on the current state value.
- `manage_clipboard_state_machine`: Updates the clipboard status based on a state field, managing buffer offsets, counters, and linked list segments.
- `compare_and_update_offset`: Loads a longword from offset 90 of the structure in a2 into d2, calculates a value by adding a longword from offset 36 of the structure in a4 to a longword from offset 44 of the structure in a4, compares this value to d2, and if it is not lower, stores the longword from offset 36 of the structure in a4 into offset 32 of the structure in a4 before branching to another routine.
- `update_clipboard_offset`: Subtracts a longword at offset 44 of a structure in a2 from a longword at offset 44 of a structure in a4, stores the result at offset 32 of the structure in a4, and then adds that result back to the longword at offset 44 of the structure in a4 before branching to offset 2022.
- `decrement_clipboard_counter`: Clears a longword at an offset of 32 in a4, sets a d0 register to 255, writes it to an offset of 48 in a4, decrements a word at an offset of 110 in a2, calls another routine, and enters a loop that clears a word at an offset of 64 in a2 if the previously decremented word is non-zero.
- `update_clipboard_buffer_state`: Checks if a status field at offset 64 of a structure equals 4; if so, it updates a counter at offset 90 based on the return value of another routine.
- `copy_clipboard_pointer`: Copies a longword from an offset of 44 in a4 to offset 90 in a2, pushes a4 and a3 onto the stack, calls another routine, and then branches to another location.
- `check_clipboard_status`: Compares the word at offset 28 of a structure in a4 with 4, and if they are not equal, branches to another location; otherwise, it stores 4 at offset 64 of a structure in a2 and branches to another location.
- `clear_d3_register`: Clears the d3 register and branches to another location.
- `manage_clipboard_buffer_state`: Checks if a status field at offset 64 of a structure equals 4; if so, it updates a counter at offset 90 based on the return value of another routine, clears a 16-bit value at offset 110, and calls another routine with a value from offset 112.
- `clear_memory_location`: Clears a longword at offset 90 of a structure pointed to by a2 and branches to another location.
- `update_clipboard_status`: Checks bit 5 of a byte at offset 30 of a structure in a3 and clears bit 1 of that byte if it is set.
- `traverse_linked_list`: Follows a pointer chain through a list while comparing a value at offset 48 of a structure in a2 against a value at offset 12 of the current list element, then calls another routine with a value from the final element.
- `clear_d1_and_branch`: Sets register d1 to zero and branches to another location.
- `call_library_function_2`: Pushes four values onto the stack, calls a library function using a base from a pointer at offset 4 of a2, and restores the stack and a2 before returning.
- `set_byte_value`: Writes the constant byte value 253 to offset 31 of a structure pointed to by a pointer on the stack and calls another routine.
- `set_byte_constant_offset`: Writes a constant byte value to a specific offset within a structure and then calls another routine.
- `set_byte_constant_offset_and_call`: Writes the constant byte value 253 to offset 31 of a structure pointed to by a pointer on the stack and then calls another routine.
- `set_byte_constant_offset_2`: Writes the constant byte value 253 to offset 31 of a structure pointed to by a value on the stack and then calls another routine.
- `set_byte_constant_offset_and_call_2`: Stores the constant byte value 253 at offset 31 of a structure pointed to by a value retrieved from the stack and then calls another routine.
- `update_structure_byte_sequence`: Writes the constant byte value 253 to offset 31 of three different structures pointed to by values retrieved from the stack, calling a different routine after each write.
- `update_clipboard_data_2`: Calls another routine, checks bit 7 and bit 0 of a byte at offset 48 of the structure in a2, and if bit 0 is not set, copies a 32-bit word from an offset 56 of a pointed-to structure into offset 48 of the structure in a2 and sets bit 4 of a byte at offset 30 of the structure in a2.
- `call_subroutine_with_offset`: Calls another routine using a value from a register plus an offset of 96 and then calls a second routine.
- `increment_clipboard_counter`: Calls another routine, checks bit 7 and bit 0 of a byte at offset 48 of the structure in a2, and if bit 0 is not set, increments a 32-bit word at offset 56 of a pointed-to structure and stores the result into offset 48 of the structure in a2.
- `call_subroutine_with_different_offset`: Calls another routine using a value from a register plus an offset of 112 and then calls a second routine.
- `copy_structure_field`: Copies a longword from offset 56 of a structure pointed to by the address at offset 24 of a structure in a0 into offset 48 of that same structure in a0, then calls another routine.
- `copy_structure_field_offset`: Copies a longword from offset 60 of a structure pointed to by the address at offset 24 of a structure in a0 into offset 48 of that same structure in a0, then calls another routine.
- `manage_clipboard_buffer_allocation_2`: Searches a linked list for a node matching a specific value and, if not found, allocates a new node, initializes its members, and updates a global counter.
- `compare_and_clear_registers`: Compares a long value at offset 14 of a structure in a3 with d3; if they are not equal, it sets d4 and d2 to zero and branches to another location.
- `manage_clipboard_buffer_allocation`: If a flag is zero, it increments a counter in a structure and a global variable before potentially calling several other routines recursively.
- `synchronize_clipboard_buffer`: Initializes a structure by setting several fields to one and calls a function pointer with a pointer to the structure plus an offset.
- `process_buffer_chunk`: Calls another routine, and if the result in d0 is not 512, it clears a word at offset 94 of a structure in a3 and subtracts 512 from a value at offset -4 of a structure in a6.
- `loop_and_clear_word`: Tests a word at offset 94 of a structure in a3 and branches if zero, tests a longword at offset -4 of a structure in a6 and branches if lower or equal, calls a routine with a1 on the stack, and if the return value in d0 is not zero, loops by adding 4 to the stack pointer and repeating the process; otherwise, it clears a word at offset 94 of a3 and branches.
- `process_clipboard_data_2`: Updates a structure based on comparison results, clears several fields at offsets 5a, 5e, and 43, and enters a loop that calls another routine using a pointer from a structure until it returns zero.
- `process_complex_structure`: Increments a value at offset 18 of a structure, increments a global variable, stores a pointer in a structure at offset 24, and performs several conditional library calls and function calls based on values in a structure before returning a value from offset 31 of a structure.
- `decrement_and_cleanup_clipboard_structure`: Decrements a 16-bit counter at offset 18 of a structure and, if it reaches zero, performs a recursive cleanup of a linked list of data blocks, eventually clearing the structure's pointers and decrementing a global byte counter.
- `execute_library_call`: Calls another routine with a pointer to offset 18 of a structure in a4, and if the result in d0 does not match d4, it clears a word at offset 94 of a structure in a3 and sets d4 to zero.
- `clear_status_word`: Clears a word at offset 94 of a structure in a3 and branches to another location.
- `cleanup_and_call_library`: Calls three routines, conditionally calls a fourth routine based on a longword at offset 68 of a structure in a3, calls two library functions using a base from a global variable, clears two 32-bit fields at offsets 20 and 24 of a structure in a2, decrements a global byte, and conditionally calls a library function if that global byte is zero and a bit in another global variable is set.
- `cleanup_structure_data`: Conditionally calls another routine, resets two 32-bit fields at offsets 14 and 18 of a structure to zero, decrements a global byte, and conditionally calls another routine if the byte is zero and bit 3 of a global variable is set.
- `execute_chained_calls`: If a word at offset 252 of a structure is not zero, it calls another routine using a pointer from a global variable, then calls another routine with a pointer from a structure, and finally moves a long value from offset 314 of a structure into d0.
- `set_bit_in_global`: Sets bit 3 of a long value at a specific memory location and returns zero in d0.
- `update_handler_status`: Calls another routine, sets a byte at offset 8 of a structure to 5, performs a bitwise AND on a byte at offset 1e of a structure, and then either calls another routine or branches to another location based on a comparison of a word at offset 1c with 12.
- `invoke_table_entry_4`: The routine invokes a function pointer retrieved from an offset within a global variable using an index, then calls another routine before returning.
- `check_bits_and_set_byte`: Calls another routine, then if bits 7 and 6 of a byte at offset 30 of a structure in a2 are both clear, writes the constant byte 254 to offset 31 of that structure and calls another routine if bit 5 is set, followed by another routine call; it then calls another routine before returning.
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
- `update_linked_list_node_2`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the value at that location by four, clears the following word, and stores the original pointer address into the word after that.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `invoke_table_entry_6`: Retrieves a function pointer from a global variable and calls it using arguments loaded from the stack.
- `store_byte_and_increment`: Stores the byte from d0 at the address in a3 and increments a3 before returning.

### printer.device

- `execute_initialization_pointer`: Calls a routine through a pointer at offset -54 of a structure in a6, and if the result is non-zero, stores that result in a global variable.
- `set_status_byte`: Loads the value negative two into register d0 and calls another routine.
- `manage_linked_list`: Sets a bit in a byte, performs a linked list insertion if a flag is set, and calls functions through pointers retrieved from a table.
- `execute_printer_initialization`: Calls a routine through a pointer at offset -54 of a structure in a6, and if the result is non-zero, stores that result in a global variable, sets a value at offset 34 of the result structure, and calls another routine through a pointer at offset -272 of a6.
- `initialize_device_structure`: Calls four routines using pointers from a table based on a global variable, and if a specific routine returns zero, it fills a memory range with a constant value in a loop, updates several pointers and values in a structure, and calls two more routines before clearing a global variable.
- `call_library_functions`: Calls four different routines using pointers retrieved from a table based on offsets from a global variable.
- `call_three_handlers`: Calls three different routines sequentially and then returns.
- `set_status_byte_alternative`: Loads the value negative three into register d0 and calls another routine.
- `set_status_bit`: Sets bit 0 of a byte at offset 61 of the structure in a6.
- `clear_status_bit`: Clears bit 0 of a byte at offset 61 of the structure in a6.
- `check_handler_pointer`: Loads a pointer from an offset of 48 of a structure in a6, checks if it is null, and if not, calls another routine; otherwise it branches to itself.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_previous`: Performs a relative branch to a previous address.
- `process_device_requests_2`: Calls several library functions, initializes two linked lists within a structure in a6, and enters a loop that checks bits in a value from a library call to either iterate through a linked list and call functions via a table, or clear specific bits in a byte at offset 1554 of the structure in a6.
- `compare_and_clear_bit`: Compares register d0 with a value loaded from an offset of a6, and if they are equal, clears bit 1 of a byte at an offset of a6.
- `process_device_requests`: Enters a loop that checks bits in a value from a library call to either iterate through a linked list and call functions via a table, or clear bits in a byte at an offset of a6 based on returned values.
- `call_serial_buffer_handler`: Loads a pointer from offset 4 of a7 into a0, loads a value from offset 8 of a7 into d0, and calls another routine.
- `update_status_and_pointers`: Calls another routine, and if it returns zero, it modifies a byte at an offset of a pointer from a global variable, then updates a byte at an offset of a structure and sets several fields in a structure before calling two more routines.
- `adjust_stack_and_branch`: Adds 8 to the stack pointer and branches to a previous address.
- `set_status_and_branch`: Sets bit 1 of a byte at offset 2486 of a structure in a6, loads the address at offset 190 of a6 into a1, and branches to a previous address.
- `check_status_byte`: Loads a pointer from a global variable, reads a byte from offset 31 of that pointer, and if it is not zero, checks if bit 0 or bit 1 of a byte at offset 2450 of a structure in a6 is set, potentially returning zero.
- `loop_status_checks`: Performs a loop that calls several routines, checks a longword at an offset of a structure in a6, and conditionally modifies a byte at an offset of a structure in a6 based on a returned value.
- `compare_and_branch_status`: Loads a value into a1 from an offset of a6, compares it to d0, and if they are not equal, branches; otherwise it clears bit 1 of a value at offset 950 of a6, calls a routine via bsr, and branches back to the start.
- `compare_and_branch_on_status`: Compares the value at offset 272 of a structure in a6 to a value in d0, and if they are not equal, calls a routine through a pointer in d0 and returns if d0 is zero.
- `execute_callback_sequence_2`: Calls another routine and then branches to a previous address.
- `update_structure_fields`: Loads a pointer from a global variable into a1, writes 9 to offset 28 of the structure in a1, copies a long value from offset 50 of the structure in a0 to offset 32 of the structure in a1, and clears a long value at offset 36 of the structure in a1.
- `clear_status_byte`: Sets a byte at a specific offset to zero and calls a routine through a pointer found in a structure.
- `verify_callback_offset`: Loads an address from a structure in a6, compares the byte at offset 8 of that address with 7, and returns if they are not equal.
- `execute_callback_sequence`: Calls a routine through a pointer in a structure and then calls another routine using a value from a global base.
- `check_device_mode`: Loads a pointer from a table, checks a byte at offset 31 of that pointer, and if it is non-zero, clears a6 and checks if a byte at offset 2454 of a6, masked with three, is non-zero.
- `process_device_status`: Enters a loop that calls three different routines and continues if a longword at offset 748 of a structure in a6 is zero; within the loop, it checks a value returned by a routine and clears bit 0 of a byte at offset 934 of a structure in a6 if the value matches a specific offset, otherwise it calls another routine and branches.
- `compare_and_clear_bit_2`: Compares a longword at an offset of 46 in a6 with d0 and, if they are not equal, branches; otherwise it clears bit 0 of a byte at offset 2486 in a6, calls another routine, and branches.
- `compare_and_branch_address`: Compares a longword at an offset of a6 with d0, and if they are not equal, branches; otherwise it calls a routine and branches based on the result in d0.
- `call_and_branch_routine`: Calls a routine via bsr and then branches to another location.
- `check_device_status`: Saves registers to the stack, compares a byte at an offset of a structure in a6 to one, and if they are not equal, branches to a routine using a pointer pushed onto the stack.
- `update_interrupt_and_status`: Modifies a global variable, updates a value at an offset of a global variable, calls another routine, and clears bit 0 or bit 1 of a byte at offset 2486 of a structure in a6 based on the results of calls and existing bit states; it also modifies the INTENA register.
- `clear_status_bits_2`: Calls another routine, and if it returns zero, clears bit 0 of a byte at offset 2450 of a structure in a6 and calls another routine if bit 0 was set, then clears bit 1 of that same byte and calls another routine if bit 1 was set.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `update_list_and_copy_string`: Pushes a pointer onto the stack twice, calls a routine through a pointer, restores the pointer to a1, calls another routine, and returns.
- `reset_structure_field`: Sets a 32-bit field in a structure at offset 32 of a1 to zero and conditionally updates it from the field at offset 36 of a1 based on a routine return value, then calls another routine twice.
- `update_list_and_copy_string_2`: Pushes the value in a1 onto the stack twice, calls a routine using the value in a1, restores a1 from the stack, calls another routine, and returns.
- `execute_callback_and_update_list`: Pushes two pointers onto the stack, calls a routine via a pointer, restores the stack, and calls another routine before returning.
- `dispatch_pending_entry`: Checks a condition via a routine call, and if met, populates a structure with two parameters and executes several routines through a global base.
- `execute_printer_command`: Saves registers to the stack, loads a pointer from offset 8 of a structure in a6 into a2, sets a2 to the constant 284 and a4 to the constant 266, calls a routine, and then branches to another routine based on the zero flag.
- `reorder_printer_buffer_data`: Calculates the length of a null-terminated string and copies bytes from a source buffer into a global array using indices stored in a secondary buffer.
- `set_printer_status`: Moves a longword from the address in a3 plus an offset of 456 to a global variable, clears a 16-bit value at a global variable, and then branches to another location.
- `copy_buffer_to_global_array`: Copies bytes from a source buffer to a global array using two incrementing pointers, then enters a loop that performs recursive calls to another routine while checking a character against a specific byte in memory.
- `clear_registers_and_branch`: Sets registers d4, d6, and d2 to zero and then branches to another location.
- `process_printer_buffer_queue`: Iterates through a buffer to match data against a target, then copies bytes from a source pointer into a global array using a counter.
- `copy_memory_and_set_field`: Copies bytes from one memory location to another in a loop while incrementing counters, then sets a 32-bit value at an offset of 32 from a pointer in a2 to either zero or a value from a register based on a condition.
- `clear_memory_offset`: Stores a zero at a specific offset within a memory location pointed to by a register.
- `transform_character_via_lookup_table`: Iterates through a table to find a matching character, applying specific transformations to the byte based on its index and value, then shifts the table elements left by one byte to remove the processed entry.
- `clear_and_branch_2`: Clears a longword at offset negative 4 from a structure in a6 and branches to another routine.
- `transform_and_shift_string_data`: Iterates through a loop to compare a string against a table, performing conditional character transformations based on specific index and value matches before storing the result via a pointer.
- `check_and_write_status_2`: Compares d0 to 1 and then compares d0 to 5; if d0 is 5, it stores the byte 255 at the address in a2 and branches.
- `write_status_byte_3`: Compares d0 to two and a longword at a global address, and if d0 equals twelve and matches the longword, stores the byte 75 at the address in a2.
- `check_and_set_byte`: Compares a long value in d5 to nine, and if the byte at the address in a4 is non-zero, writes the value fifty-nine to the address in a2.
- `write_indexed_byte`: Compares a long value in d5 to six and, if it is less than or equal to six, copies a byte from a table at an offset determined by d5 into the location pointed to by a2.
- `check_value_and_write_byte`: Checks if a long value in d5 is non-zero; if it is zero, it compares a byte at the address in a4 to 29, and if that byte is 29 or greater, it writes the value 12 to the address in a2 before branching.
- `transform_string_character_via_lookup`: Compares a character against a table using a nested loop and performs conditional transformations on a destination byte based on specific matches or index values.
- `process_printer_escape_sequences_2`: Iterates through a byte stream to interpret control characters, handling escape sequences for formatting, character selection, and line feed.
- `process_printer_escape_sequences`: Iterates through a byte stream to interpret control characters, handling escape sequences for formatting, character selection, and line feeds by updating a destination buffer and calling other routines to manage printer state or command execution.
- `parse_printer_command_stream_2`: Iterates through a stream of bytes to interpret control characters and data, writing bytes to a destination buffer and calling another routine to handle specific command sequences or escape characters.
- `compare_printer_byte`: Compares the byte at the address in a3 with 91, and if it is not equal, branches to another location; otherwise, it branches to another location.
- `parse_printer_command_stream`: Iterates through a buffer to parse control characters and escape sequences, translating specific bytes into command codes or looking up string expansions in a table.
- `check_printer_command`: Compares the byte at the address in a3 with 10, and if it is not equal, branches to another location; otherwise, it tests a byte at offset 104 of a structure, and if it is not zero, branches to another location, otherwise it stores 3 at the address in a4 and branches to another location.
- `process_printer_command_stream`: Parses a stream of bytes to interpret control characters and writes data to a destination buffer.
- `write_multiple_bytes`: Writes the value 10 to a memory location calculated by adding d2 to a pointer, increments that pointer, then repeats the process to write the value 13 to a second calculated memory location before branching.
- `write_single_byte`: Writes the value 10 to a memory location calculated by adding d2 to a pointer, increments that pointer, and then branches.
- `check_character_and_call`: Compares a byte at a3 to 155; if it is not equal, it branches to another location, otherwise it pushes d2 and d3 to the stack, calls another routine, cleans the stack, and branches to another location.
- `parse_file_path_string_2`: Processes a string to extract file path components, handling backslashes and drive specifiers by calculating string lengths and copying characters into a destination buffer.
- `parse_file_path_string`: Parses a string to extract directory components and numeric values from path separators, then stores the resulting characters into a destination buffer.
- `update_pointer_and_offset`: Increments a1 and a global longword, checks if the byte at a1 is non-zero, and if it is zero, loads a value from a2, adds a global longword to it, compares it to 256, and conditionally updates a global longword with a value from a table.
- `calculate_pointer_offset`: Loads a byte from a pointer in a3, subtracts 160, shifts it left by two, adds it to d4, and uses the result to load a pointer into a1.
- `skip_non_numeric_characters`: Checks if a byte at a1 is a backslash, and if not, increments a1 and checks if the next byte is a digit between zero and nine, branching back if the character is not a digit.
- `parse_ascii_numeric_string`: Converts a three-digit ASCII numeric string into an integer and stores it in a destination buffer at an offset determined by an incrementing index.
- `copy_byte_to_offset`: Loads a pointer from a2, increments a2, and copies a byte from the address in a1 to the address calculated by adding the value in a2 to the value in a0, then branches to another routine or a different location.
- `parse_printer_escape_sequences`: Iterates through a byte stream to interpret control characters, handling escape sequences for formatting, character selection, and line feed.
- `update_global_values`: Subtracts the difference between two global variables from a third global variable and clears a fourth global variable.
- `copy_buffered_data_with_overflow_check`: Calls another routine to populate a local stack buffer, then attempts to copy that buffer to a destination address using a global offset.
- `update_and_branch_on_pointer`: Adds d3 to a value from a global variable, and if the result is greater than two hundred fifty-six, updates a global variable with d3 and branches to another routine; otherwise, it checks a byte at an offset from a6 and may call another routine.
- `decrement_and_store_global_pointer`: Decrements a 32-bit value from a global variable by one, stores the result in another global variable, and branches to another routine.
- `clear_register_and_branch`: Sets register d4 to zero and branches to another routine.
- `copy_memory_block`: Copies a sequence of bytes from a source address to a destination address, incrementing a global pointer for each byte.
- `clear_byte_and_branch`: Clears a byte at a location pointed to by a register, then branches to another routine.
- `clear_byte_on_match`: Compares a byte in d1 to fifty-nine, and if they are equal, increments a3 and clears a byte at the address calculated by adding a2 to a0.
- `load_pointer_on_match`: Compares a byte in d1 to thirty-two and, if they are equal, loads a pointer from an offset of the stack into a0 and writes one to the location pointed to by a0.
- `compare_and_load_pointer`: Compares a byte in d1 to 34; if not equal, it branches to another routine, otherwise it loads a pointer from a stack offset into a0, sets d0 to two, and branches to another routine.
- `parse_character_stream`: Enters a loop that reads a byte, performs arithmetic on a byte at an offset in a structure, and checks if the byte is a digit, a space, a semicolon, or a quote, updating a pointer and a global variable accordingly.
- `fetch_indexed_global_value`: Increments a value pointed to by a parameter and uses the original value to fetch a word from a table, then calls another routine if a condition involving a second parameter or a global variable is met.
- `return_register_value`: Returns the value currently held in a data register.
- `update_global_pointer_from_variable`: Loads a pointer into a1, decrements a 32-bit value from a global variable by one, stores the result at the address in a1, and branches to another routine.
- `decrement_and_check_table`: Decrements a value pointed to by a register and continues looping as long as the value is positive and a specific bit pattern is found in a table.
- `reset_global_state`: Clears three global variables, sets a byte at offset 2720 of a structure to zero, and sets another global variable to 3.
- `copy_data_to_globals`: Copies six 16-bit values from a structure into a set of global variables.
- `compare_global_data`: Compares six 16-bit values from a structure against a set of global variables, returning 0 if all match or -1 otherwise.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_nested_routines`: Extracts four bytes from a structure pointed to by param_1, then calls another routine with a short value from that structure and performs conditional calls to other routines based on the return value.
- `dispatch_conditional_routines`: Calls another routine with parameters 1 and 0, and if the result is zero, calls another routine with a pointer from a structure in a6; if the first call returned zero and d2 is zero, it also calls another routine with parameters 3 and 0.
- `copy_string_with_logic`: Calls a routine via a pointer in a global variable; if the return value is zero, it checks param_1 against specific constants to update a global variable, then either copies a string from a table to the destination at param_2 or copies a single byte from a pointer to the destination.
- `set_status_byte_3`: Compares a word at offset 22 of the stack to 54 and 53, and if it matches 53, sets a byte at offset 262 of a structure in a6 to the result of a logical AND between 1 and the value at offset 22 of the stack.
- `copy_string_with_translation`: Checks a condition involving a global variable and a value, then enters a loop that copies bytes from a source to a destination, performing specific translations on certain byte values and stopping when a null terminator or a specific value is encountered.
- `set_d1_value`: Compares the byte in d1 with 255, and if it is not equal, sets d1 to 255 and branches to another location.
- `copy_data_loop_2`: Enters a loop that copies bytes from one memory location to another, updating an index and checking for a null terminator or a specific value.
- `jump_via_table`: Compares a value in d1 against 7, and if it is not less than 7, uses a value from a table to jump to a new address.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `validate_printer_timing_parameters_2`: Extracts timing and configuration data from a structure, calculates available time intervals by subtracting start and end values, and compares these against minimum thresholds.
- `update_printer_timing_limits`: Calculates current timing intervals by subtracting start and end values from a structure, compares these against minimum required thresholds, and updates a global status byte and a set of minimum timing values in a structure if the current intervals are sufficient.
- `validate_printer_timing_parameters`: Calculates the duration of two time intervals from a structure, compares them against stored minimum limits, and updates a global status byte and timing values in a structure based on whether the intervals are sufficient.
- `recalculate_printer_timing_parameters`: Calculates minimum timing values by comparing current values against stored limits in a structure and updates a global status byte through bitwise manipulations.
- `update_printer_timing_and_status_2`: Updates minimum timing values in a structure based on current values and performs bitwise manipulations on a global status byte, including updating a 32-bit integer pair and adjusting timing constants.
- `compare_and_branch_6`: Compares the word in register d0 to 2048; if they are not equal, it branches to another routine, otherwise it branches to another routine.
- `load_constant_and_branch_2`: Loads the value 12 into register d1 and branches to another routine.
- `load_constant_and_branch_3`: Loads the value 15 into register d1 and branches to another routine.
- `update_printer_timing_and_status`: Updates minimum timing values in a structure based on current values and performs bitwise manipulations on a global status byte, including a bit-swapping operation.
- `update_printer_status_and_buffer`: Updates a minimum value in a structure and performs complex bitwise manipulations and swaps on several global variables and buffer pointers.
- `modify_stack_offset_value`: Sets register d6 to 6 and d0 to 7, stores d0 at offset 12 of a structure in a6, checks bytes at offsets 360 and 362 of global variables, and if both are non-zero, adds d6 to itself and adds the value at offset 12 of the structure in a6 to itself before storing the result back at that offset and branching to another routine.
- `check_status_and_branch`: Clears a byte at offset 22 of a global variable, then checks bytes at offsets 222 and 230 of global variables to decide whether to clear the stack and push a pointer to a local variable before branching to another routine.
- `update_color_component_bounds`: Calculates the minimum and maximum values for color components by comparing values from a palette and updates several global variables with these bounds and their calculated ranges.
- `calculate_color_lookup_tables_2`: Determines color component bounds from a palette, clears a 64-entry table, and populates it by extracting bit components from a palette.
- `calculate_color_lookup_tables`: Determines color component bounds from a palette, clears a 64-entry table, and populates it by extracting bit components from a palette.
- `generate_color_lookup_tables_4_2`: Calculates color component bounds, clears a 64-entry table, and populates it by extracting bit components from a palette.
- `generate_color_lookup_tables_4`: Calculates color component bounds, clears a 64-entry table, and populates it by extracting bit components from a palette.
- `generate_color_lookup_tables_3`: Clears a table of 64 entries and populates it by calling convert_and_process_color_palette to extract bit components into separate memory locations.
- `generate_color_lookup_tables_2`: Clears a table of 64 entries and populates it by calling convert_and_process_color_palette to extract bit components into separate memory locations.
- `generate_color_lookup_tables`: Clears a table of 64 entries and populates it by calling convert_and_process_color_palette to extract bit components into separate memory locations.
- `convert_and_process_color_palette`: Extracts 4-bit color components from a source value to populate a table, performs bitwise masking and inversion on a series of 32-bit values in a loop, and calls another routine with various arguments based on the results of conditional checks.
- `initialize_printer_parameters_3`: Sets three global 32-bit values, calls a routine through a pointer with five arguments, and then conditionally calls that same routine with different arguments based on the returned value and a modulo check against a value at offset 36 of a pointer in d5.
- `call_function_and_store_result`: Pushes three empty 32-bit values onto the stack, loads a pointer from offset 40 of a global variable into a2, calls the routine at that pointer, stores the result from d0 into offset 366 of a global variable, and then branches to another routine or a different location based on the zero flag.
- `check_printer_status_and_call_2`: Tests two bytes in a global variable; if both are non-zero, it calls another routine and then branches to another location, otherwise it returns.
- `test_and_call_printer`: Tests a byte in a global variable and, if it is zero, branches to another location; otherwise, it calls another routine and then branches to another location.
- `check_printer_status_and_call`: Tests a byte in a global variable and, if non-zero, calls another routine before branching to a different location; otherwise, it branches to a different location.
- `update_status_from_call`: Calls another routine, stores the result in a global variable, tests that global variable, and branches to another routine if it is zero, otherwise it branches to another routine.
- `execute_conditional_callback`: Checks if a global variable is divisible by a value at offset 24 of a pointer in d5, or calls a routine through a pointer in a global variable and, if the result is zero, sets a global variable to zero, then calls another routine with the result.
- `call_empty_routine`: Calls a routine with two zero arguments and returns.
- `calculate_weighted_value`: If a global variable is zero, it checks bit 2 of a byte at offset 21 of a global variable and, if set, finds the smallest byte value among three provided bytes; otherwise, it calculates a weighted sum of three bytes and potentially subtracts the result from 15.
- `find_minimum_byte`: Checks bit 2 of a byte at offset 21 of a global variable and, if set, finds the smallest byte value among three provided bytes and returns it, otherwise it returns the original second parameter.
- `calculate_pointer_offset_2`: Calculates a value based on a global variable and a table, calls another routine, and then updates a global variable and a global longword using the result or a table lookup.
- `update_global_pointers`: Calculates a value using a global variable and a table, then updates four global pointers by adding a global variable to each, while also calling two different routines and performing conditional assignments to a global variable and a pointer.
- `update_pointer_table_elements`: Fetches a 32-bit value from a table using an index and a global base address, then updates four global pointers by adding a value to each.
- `move_and_test_bit`: Moves a longword from an offset of a2 to a memory location, moves a value into a0, tests bit 3 of a byte at an offset of a0, and performs conditional moves and subtractions.
- `shift_and_compare_d0`: Moves d2 to d1 and d3 to d0, performs a logical shift right on d0 by 4 bits, and branches based on whether d0 is less than 1 or greater than 1.
- `compare_and_branch_constant`: Compares d0 to two and branches to a previous address if it is not equal, otherwise it branches to a later address.
- `decrement_and_loop`: Decrements a value in d0 and branches to an earlier instruction if the result is not zero, otherwise it branches to another location.
- `store_byte_and_branch`: Stores a byte from d1 into a global variable at offset 272 and branches to another location.
- `store_byte_and_branch_offset`: Stores a byte from d1 into a global variable at offset 274 and branches to another location.
- `update_pointer_table_2`: Moves a byte from d1 to an offset of 273, pushes a pointer from a2 onto the stack, calls another routine, stores the result in the location pointed to by a2, adjusts the stack pointer, and moves the value from a2 to offset 276.
- `update_pointer_table`: Loads a 32-bit value from a table using an index and updates four global pointers by adding a global base address to them.
- `calculate_and_update_pointers`: Calculates a value based on the product or sum of two globals and updates four global variables by calling another routine multiple times.
- `execute_priority_sequence_2`: Clears a global variable and then performs a series of conditional checks involving calling routines and checking their results to determine a final return value.
- `execute_priority_sequence`: Checks if two global variables are zero, then calls three different routines through a pointer in a global variable using parameters from the stack, and if any return a non-zero value, stores that value in a pointer from the stack and returns it.
- `compare_and_store_result_2`: Compares two values using a table or a global variable, then calls another routine and stores its result in a location pointed to by a parameter, potentially calling a third routine if the result is non-zero.
- `compare_and_store_result`: Calls another routine and, if the result is non-zero, calls a third routine with four parameters and stores the result in the location pointed to by a2.
- `clear_and_restore_registers`: Sets d0 to zero and restores registers a2 through a4 from the stack.
- `execute_and_store_result_2`: Increments a global variable and, if a condition involving a value from a table is met, calls another routine twice and stores the result in a global variable.
- `execute_and_store_result`: Pushes three zeros onto the stack, loads a pointer from a global variable, calls that routine, stores the result in the location pointed to by a2, and branches to another location based on the status of d0.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `setup_limit_check`: Saves a2 through a4 on the stack, loads three constant addresses into a2, a4, and a3, moves a word from offset 380 of a global variable into the address in a4, and branches to another routine.
- `execute_complex_nested_logic_2`: Calls accumulate_scaled_values, then enters a loop that performs nested calls to execute_complex_nested_logic and accumulate_scaled_values based on values retrieved from a global variable and a table.
- `execute_nested_loop_logic`: Executes nested loops that call another routine, updates a global variable by adding a product of two values to it, and performs conditional checks on a pointer's target value.
- `clear_and_branch`: Clears a 16-bit word in a global variable and branches to another location.
- `process_nested_loops`: Executes a series of nested loops that call another routine, update various global variables, and perform conditional checks on the results.
- `execute_complex_nested_logic`: Increments a global variable, updates a global variable by adding a product of another global variable and a value from a table to it, and enters a loop that performs conditional calls to other routines and updates various memory locations via pointers.
- `increment_and_check_limit`: Adds a word from the address in a3 to a word at the address in a4, compares the new value at the address in a4 to 300, and if it is less than 300, branches to offset 3956; otherwise, it clears d0 and returns.
- `initialize_printer_data`: Saves registers a2 through a4 to the stack, loads three constant addresses into a2, a4, and a3, clears a longword at a global variable, moves a word from a global variable to the address in a4, and branches to another routine.
- `execute_scaled_loop_logic_2`: Calls accumulate_scaled_values and enters a loop that performs conditional calls to accumulate_scaled_values and execute_scaled_loop_logic based on a global variable.
- `clear_printer_registers`: Clears four longwords at offsets 128, 120, 124, and 11c of a structure, clears a word at offset 154, and branches to another location.
- `execute_scaled_loops`: Performs a loop that calls another routine, updates a global variable, and contains a nested while loop that modifies several global variables and calls another routine.
- `call_and_store_result`: Calls a routine, stores the result in a memory location pointed to by a2, and branches based on whether the result is zero.
- `execute_scaled_loop_logic`: Increments a global variable, decrements another global variable, and enters a loop that performs conditional calls to other routines and updates memory locations via pointers.
- `increment_and_check_limit_2`: Adds a word from the address in a3 to a word at the address in a4, compares the new value at the address in a4 to 300, and if it is less than 300, branches to offset 2914; otherwise, it clears d0 and returns.
- `initialize_printer_buffer_parameters`: Clears a 16-bit value at a global offset, loads three constant addresses into registers, copies a 16-bit word from a global variable to the address in the second register, and branches to another routine.
- `call_accumulation_routine`: Pushes two 16-bit values from global variables onto the stack, calls another routine, stores the result in the location pointed to by a3, and branches to another location.
- `initialize_state_pointers`: Clears a 16-bit word at a global address, copies a longword from one global address to another, copies a longword from another global address to a third, moves a word from a global address to a register, and branches to another routine.
- `update_printer_state`: Calls another routine, increments a global longword, compares it to a 16-bit value at a global address, and if not greater, calls two other routines.
- `update_clipboard_data`: Calls another routine, increments a global longword, compares it to a 16-bit value at a global address, and if not greater, calls two other routines before storing d0 into a structure at a2 and branching.
- `execute_decrementing_loop_logic`: Increments a global variable, updates a global variable by adding a product of another global variable and a value from a table to it, and enters a loop that performs conditional calls to other routines and decrements a value through a pointer.
- `decrement_and_check_limit`: Decrements a word at the address in a3, and if the result is not below zero, it increments a word at the address in a4, compares the value at the address in a4 to 300, and if it is less than 300, branches to offset 2974; otherwise, it clears d0 and returns.
- `reset_printer_status_counters`: Clears two 16-bit words at specific offsets in a data structure and updates a value at an offset in another structure using a value from a third location, then branches to another routine.
- `push_and_call_routine`: Pushes two 16-bit values from memory onto the stack, calls another routine, stores the result in a location pointed to by a3, and branches to another location.
- `update_internal_state_pointers`: Clears a 16-bit value at a global address, copies a longword from one global address to another, copies a longword from another global address to a third, moves a word from a fourth global address to a fifth, and branches to another routine.
- `manage_priority_counters`: Clears four global variables, enters a loop that calls another routine, and contains nested loops that update global variables and call another routine based on specific conditions.
- `update_pointer_from_call`: Calls another routine, stores the resulting value into a location pointed to by a register, and if the result is not zero, reloads the register with the stored value before branching to another routine; otherwise it branches to another routine.
- `manage_printer_loops`: Increments a global variable and, if a condition involving a value from a table is met, calls another routine twice and stores the result in a location pointed to by a register.
- `update_pointer_and_check_limit`: Decrements a 16-bit word at a location pointed to by a3, branches if the value is higher than 0, otherwise increments a 16-bit word at a location pointed to by a4, compares a word from that location to a global constant, and returns after restoring registers if the comparison fails.
- `accumulate_scaled_values`: Iteratively adds a doubled value to a pointer's target until the target is no longer less than one, then subtracts a doubled second parameter from that target.
- `compare_table_values`: Compares a byte with either a value from a table using indices derived from two parameters or with a value calculated from a global variable, returning a boolean result.
- `compare_table_value`: Performs a comparison between a byte retrieved from a table using bitwise-derived indices and a provided byte.
- `negate_boolean_results`: Calls two different routines and returns the negation of the first return value based on the status of a second return value.
- `rotate_32bit_value_4`: Performs a complex 32-bit bitwise rotation and division operation using a loop that calls another routine.
- `rotate_32bit_value_2`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_3`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `process_string_buffer`: Calls another routine, then enters a loop that compares bytes from a source pointer and a destination pointer, calling another routine for each pair until a null byte is encountered.
- `fetch_table_address`: Shifts a 16-bit value in d0 left by two bits, loads a longword from a table at offset 14 plus the new d0 value into a4, and branches to another routine.
- `branch_to_offset_334`: Performs a short branch to an instruction located 334 bytes ahead in the code segment.
- `set_address_register`: Loads the constant 32 into a2 and branches to another routine.
- `process_string_and_structure`: Compares strings via a loop, and if a condition is met, calls a routine using a value from a structure, modifies offsets within that structure, and copies a string to a global variable.
- `manage_disk_and_memory`: If a value at offset 86 of a structure pointed to by a3 is non-zero, it calls a routine with that value, clears the value, and sets a 16-bit field at offset 90 of the structure to 65535; it then performs several calls and potentially copies a null-terminated string from a source to a destination.
- `unlock_and_get_directory`: Calls a routine to unlock a resource using a pointer from a register, then calls another routine to retrieve the current directory, and returns negative one.
- `execute_callback_and_recurse`: Checks a global value, calls a function pointer from a table, and then calls itself with a value from a pointer.
- `compare_case_insensitive_bytes`: Adjusts two bytes by subtracting 32 if they fall within a specific range and returns whether they are equal.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_3`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### narrator.device

- `initialize_narrator_task_structures`: Retrieves a pointer from a global variable and calls a function via a pointer in that global variable to allocate a structure.
- `test_and_branch_d0`: Tests the value of d0 and, if it is not zero, loads the constant negative one into d0 and branches to another routine; otherwise it branches to another routine.
- `update_device_state`: Updates various fields in structures at a6 and a1, including incrementing counters, setting specific bit patterns, and calling routines via pointers based on the value of a field at offset 32 of a6.
- `update_device_state_fields_2`: Writes values to a structure at a6, calls a routine via a pointer, and then either sets specific fields in a structure at a2 or updates several fields in the structure at a6 and a2 based on the return value.
- `update_device_state_fields`: The routine stores an input value into a structure, increments counters in the structure, copies a counter to another structure, writes several constant words and bytes to the second structure, and clears a flag bit in the first structure.
- `store_byte_and_reset_value`: Stores a byte from d0 into a structure at offset 31 of a2, sets a 32-bit value at offset 20 of a2 to all ones, and branches to another routine.
- `clear_bit_and_branch`: Clears bit zero of a byte at offset 30 of a structure in a1, loads a pointer from offset 20 of a1 into a3, and loads a pointer from a constant into a6; it then tests bit six of the structure at offset 34 of a3 and branches based on the result, or tests a longword at offset 48 of a3 and branches if it is not zero, otherwise it sets a byte at offset 31 of a1 to a constant and calls another routine.
- `manage_linked_list_2`: Updates a linked list by inserting a structure at a specific position, modifies a byte at offset 68 of a6, and writes to the INTENA register to enable or disable interrupts.
- `update_linked_list_and_interrupts`: If certain conditions are met, it copies a byte from one structure to another, splits it into nibbles, and clears a byte; otherwise, it updates a global variable, increments a counter in a structure, and performs a linked list insertion before decrementing the counter and potentially updating a global variable.
- `copy_byte_and_split_nibbles`: The routine copies a byte from one structure to another, splits it into low and high nibbles into two bytes, clears a byte in the destination structure, and calls another routine.
- `process_narrator_event_queue`: Traverses a linked list of event structures, decrementing a counter in a global variable and disabling interrupts via the INTENA register when the counter reaches zero.
- `decrement_narrator_counter`: Decrements a byte in a structure pointed to by a global variable; if the byte reaches zero, it disables interrupts via the hardware register and jumps to another routine, otherwise it returns.
- `traverse_and_decrement_counter`: Traverses a linked list via a pointer in a structure, calling itself if a value matches a field in the list node.
- `decrement_counter_and_close`: Decrements a 16-bit counter in a structure and, when it reaches zero, calls library functions via a base pointer and optionally calls another routine.
- `call_table_entry`: Loads a word from a structure, and if it is between zero and seven inclusive, uses it to index into a table and calls the resulting routine.
- `update_pointer_field`: Sets bit 5 of a byte at offset 34 of a structure in a3, then branches to another routine.
- `swap_pointers_and_return_value`: The routine calls library functions via a base pointer, swaps values between a pointer table and a data word, updates a pointer field, and returns a value from a table entry.
- `dispatch_narrator_event_handler`: Processes a command structure by checking a type field and executing various handlers, including setting status bits, updating linked lists of event structures, and calling other routines via pointers found at specific offsets within the structures.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `invoke_library_handlers`: Clears a bit in a structure, writes a word to another structure, sets a bit in a third structure, and calls two routines via pointers found in a structure.
- `set_flags_and_invoke_handler`: Sets bit 6 of a byte at offset 34 of a structure in a0, writes 6 to a word at offset 196 of a structure in a6, sets bit 0 of a byte at offset 198 of that same structure, calls a handler via a pointer at offset 164 of the structure in a6, writes 0 to a byte at offset 31 of a structure in a1, and calls another routine.
- `set_flag_then_clear`: Sets bit 7 in a byte at offset 34 of a structure, calls two other routines, then clears bit 7 in the same byte.
- `set_flag_clear_byte_then_call`: Sets bit 7 in a byte at offset 34 of a structure, calls another routine, clears bit 7, clears a byte at offset 31 of a second structure, and calls another routine.
- `modify_global_and_list`: Sets a global variable to a constant, increments a byte at offset 326 of that global variable, and traverses a linked list via a pointer at offset 20 of a3, calling another routine for each element; it then decrements the byte at offset 326 of the global variable and, if the previous value was less than one, sets the global variable to a different constant.
- `decrement_counter_and_set_global`: The routine decrements a byte in a global variable area and, if the previous value was less than one, writes a constant to a global variable.
- `update_interrupt_status`: Saves registers, sets the INTENA register to enable interrupts, increments a byte at offset 358 of a6, and sets bit 5 of a byte at offset 30 of a1 if a comparison between a1 and a value in a3 matches.
- `compare_and_update_pointer`: Checks if a value at a pointer is zero or if it matches a specific address, otherwise it updates a pointer in a structure and branches to another routine.
- `modify_interrupt_and_call_library`: Moves a pointer from a1 to a0, updates a pointer at offset 4 of a1 with the value in a0, writes the value in a0 to offset 4 of the structure in a1, sets bit 7 of a byte at offset 31 of a1, decrements a byte at offset 326 of a6, and if the result is not negative, writes 49152 to the INTENA register and calls another routine.
- `decrement_structure_counter`: The routine decrements a byte in a structure pointed to by a register and, if the previous value was less than one, writes a constant to a global variable.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `set_byte_and_call_routine`: Writes 249 to a byte at offset 31 of a structure in a1 and calls another routine.
- `clear_byte_and_call_routine`: Clears a byte at offset 31 of a structure in a1 and calls another routine.
- `clear_flags_and_call_handler`: Clears a flag bit in a structure, clears a byte in another structure, and calls a handler via a base pointer.
- `populate_narrator_text_structure`: Validates several 16-bit and 8-bit values from an input structure against specific ranges and, if valid, copies them into a target structure at various offsets.
- `update_clamped_value_and_check_char`: Compares a 16-bit value in d1 to 400; if it is not greater, it stores d1 at offset 30 of the structure in a5.
- `compare_and_store_value`: Compares a 16-bit value in d1 to 320; if it is not greater, it stores d1 at offset 28 of a structure in a5.
- `store_and_check_value`: Stores a 16-bit value from d1 into an offset of 48 of a structure, then loads a 16-bit value from an offset of 54 of a structure into d1 and branches to another location if d1 is zero or one; otherwise, it loads the constant 233 into d3 and branches to another location.
- `update_and_compare_value`: Stores a 16-bit value from a register into an offset of 38 in a structure, then compares a new 16-bit value from an offset of 64 in a2 against 5000; it branches to another routine if the value is greater than or equal to 5000, otherwise it sets a register and branches to another routine.
- `compare_and_store_word`: Compares a 16-bit value in d1 to 26464 and branches if it is greater; otherwise, it stores the value from d1 into an offset of 21648 in a5, loads a 16-bit word from offset 62 of a2 into d1, and if d1 is not less than 0, sets d3 to 230 before branching to another routine.
- `scan_string_for_delimiters`: Scans a string backwards from a specified offset to find a delimiter such as a comma, hyphen, or space.
- `parse_text_token_2`: Scans a string backwards from a given offset to find a delimiter such as a comma, hyphen, or space.
- `copy_and_branch`: Copies a longword from one location to another and branches to another routine.
- `clear_long_and_set_byte`: Clears a longword at offset 32 of a structure in a1, writes the value from d0 to a byte at offset 31 of that structure, and calls another routine.
- `parse_text_token`: Scans a string backwards from a given offset to find a delimiter, such as a comma, hyphen, or space.
- `process_data_table_2`: Loads a longword, performs arithmetic and bitwise operations, and if a comparison with a byte from a table succeeds, it updates a word in a structure and clears a longword.
- `shift_and_clear_value`: Loads a word from offset 150 of a structure in a5, shifts it right by one, stores the result in offset 156 of the structure, and clears the longword at offset 150.
- `initialize_narrator_text_buffer`: Clears several fields in two structures, sets specific byte values, and calls two other routines.
- `update_status_byte_2`: Loads a longword from offset 24 of a structure in a1, moves a byte from it to offset 67 of a structure in a2, and calls a routine at an offset from a6.
- `populate_speech_buffer`: Initializes a structure with several values, calls two library functions, and if a routine is found in a table, it enters a nested loop to copy data from a source to a destination buffer.
- `calculate_and_copy_data`: Writes a byte from d0 to offset 68 of a structure in a2, calculates a value based on d0, calls a library function, and if the result is not non-zero, writes 254 to d0.
- `copy_word_table_to_buffer`: Stores a pointer in a global variable and copies 34 words from a table in a global variable to a buffer pointed to by an argument, writing each word twice per iteration, repeating for a count derived from a byte in a global variable.
- `perform_logical_bit_tests`: Performs several bitwise operations and bit tests on registers d0, d1, d3, and d4.
- `handle_io_and_free_resources`: Writes a byte and a word to a global variable, calls a routine via a global variable, conditionally calls two other routines via the same global variable, clears a word in the global variable and returns a combined value.
- `test_narrator_value`: Tests the word at offset 206 of a structure in a5, and if it is not greater than zero, clears d3 and returns; otherwise, it returns.
- `parse_narrator_command_string`: Iterates through a string in a0 to parse command tokens and control characters, updating a state structure at a5 with parsed values, index offsets, and control flags.
- `modify_register_and_branch`: Negates a 16-bit word in d1, adds 111 to it, compares d0 to 12288, and branches based on the result; otherwise, it adds the value in d2 to a0 and branches.
- `parse_command_stream_buffer`: Iterates through a byte stream to identify control characters and command sequences, updating a command table and an index while managing a character buffer and associated attribute arrays.
- `parse_command_stream`: Parses a stream of bytes from a buffer, updating a command table and an index while checking for specific control characters and bit flags.
- `test_and_set_bit_five`: Compares a byte in d1 to 5; if it is not equal to 5, it sets bit 5 of a byte at an offset determined by a6 and d3, then branches to another routine.
- `test_and_set_bit`: Compares a byte in d1 to 6; if it is not equal to 6, it sets bit 4 of a byte at an offset relative to a pointer in a6 and d3, then branches to another routine.
- `update_table_index_and_buffer`: Writes a byte from register d1 to a table at the address in a2 using an index in d3, increments the index, and if the index is less than 512 and the byte is not 1 or 2, subtracts a longword from a structure at offset 146 of a5 from a0, stores the result in a structure at offset 150 of a5, and sets the carry flag before returning.
- `clear_buffer_bytes`: Subtracts a longword at offset 146 of a structure in a5 from a0, compares the result to a 16-bit word at offset 156 of a5, and if the result is not greater than or equal to that word, clears a byte at offset d3 from a structure in a3 and a byte at offset d3 from a structure in a6 before branching to another routine.
- `increment_longword_in_structure`: Copies a longword from a register to a longword in a structure at offset sixty-six from a5, then increments that value by one and returns.
- `decrement_index_on_match`: Loads a byte from a table using an index, and if the byte is not zero, decrements the index and branches to another routine.
- `update_structure_and_counters`: Checks a bit in a value retrieved from a table, and if a condition is met, writes 4, 0, and 0 to three consecutive bytes at an offset determined by d3 in three different structures, then increments d3.
- `store_word_and_set_cc`: Stores a word from register d3 into a structure at offset 154 of the structure in a5 and sets the condition code register to 8.
- `modify_string_flags`: Iterates through a null-terminated string and, based on values retrieved from a global table, sets bit 4 in various bytes preceding the current string position.
- `modify_byte_conditionally`: Performs a bitwise AND on d2 with 21504, and if the result is not zero, sets bit 4 of a byte at offset -3 of a structure in a0.
- `filter_and_set_bit`: Checks if a value at a specific offset in a structure matches a constant and, if so, sets bit four of a byte at a different offset in another structure.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `transform_narrator_text_stream`: Iterates through a source buffer and performs bitwise transformations on a destination buffer based on control characters and a lookup table.
- `process_narrator_text_stream`: Iterates through a source buffer to perform bitwise transformations on a destination buffer based on control characters.
- `parse_narrator_command_stream`: Iterates through a command buffer in a0, using values as indices into a lookup table to perform bitwise operations and byte manipulations on a destination buffer in a1.
- `calculate_offset_and_branch`: Subtracts a word from d2 from d0, decrements the result by 1, and branches to another location.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `scan_and_patch_buffer`: Iterates through a buffer, comparing bytes against a target table and checking specific bit flags.
- `compare_and_patch_buffer`: Compares bytes from a source buffer against a target buffer using a series of conditional checks and bitmask tests, then optionally writes bytes from the source buffer into the target buffer.
- `increment_and_branch_on_byte`: Increments d1 by 1, loads a byte from offset 6 of the pointer in a0, and branches to another location if the byte is not 255; otherwise, it calls another routine and returns if the carry flag is set.
- `test_bit_and_branch`: Tests bit 5 of a byte at offset 3 from the pointer in a0 and branches to another routine or a different location based on the result.
- `test_bit_with_table_lookup`: Loops reading a byte from a buffer and a word from a table indexed by a word, modifies the word based on bits in the byte, and exits when a specific bit is set or a termination condition is met.
- `compare_word_and_branch`: Loads a word from offset 154 of a structure in a5 into d7, compares it to 512, and either sets the carry flag and branches or continues to call shift_bytes_and_clear_fields.
- `shift_bytes_and_clear_fields`: Increments a word counter in a structure and shifts bytes backward in three buffers by a computed count, then writes a byte from a register and two zero bytes to the new ends of the buffers.
- `call_narrator_subroutines`: Calls seven different routines in sequence and then clears register d0 before returning.
- `clear_byte_if_not_set`: Loads a pointer from offset 40 of a structure in a5 into a6, compares the byte at the address in a6 to 255, and if they are not equal, reloads the pointer from offset 40 of the structure in a5 and writes 160 to offset 7 of that pointer before returning.
- `clear_byte_and_increment`: Clears a byte at offset 7 of a structure in a6, increments the pointer in a6 by 8, and branches to another routine.
- `check_stream_byte_2`: Loads three pointers from a structure in a5 and a global address, then checks if the byte at the first pointer is equal to 255; otherwise it returns.
- `process_audio_stream`: Loops through a data stream, performing bitwise manipulations and table lookups to update a buffer, then calls a routine to decode the stream and another routine to increment three pointers.
- `decode_audio_stream_2`: Processes an audio data stream by reading bytes, performing bitwise operations, and fetching values from a table to update a buffer.
- `decode_audio_stream`: Processes a data stream by reading bytes from a source, fetching values from a table using those bytes as indices, and performing bitwise operations and arithmetic to update a destination buffer and three pointers.
- `test_bit_and_set_byte`: Tests bit 6 of d1 and branches to another routine if it is clear; otherwise it sets bit 3 of the byte at the address in a2 and branches to another routine.
- `set_bit_and_branch_3`: Tests bit 5 of d1 and branches to another location if it is clear; otherwise, it sets bit 2 of the byte at the address in a2 and branches to another location.
- `set_bit_and_increment`: Sets bit 2 of a byte at the address in a2, increments the pointer in a2 by 1, and branches to another routine.
- `increment_three_pointers`: Increments three pointers in a0, a1, and a2 by 1 and branches to another routine.
- `calculate_masked_byte_sum`: Loads a byte from an offset of 1258 in a5 into d2, compares it to 255, and if they are not equal, masks d2 with 63 and adds the result to d0 before branching to another routine.
- `update_structure_and_call_alloc`: Stores d0 at offset 58 of a structure in a5, shifts d0 left by 3, adds 8 to it, stores the result at offset 54 of the structure in a5, calls a routine using a pointer from offset 4 of a global variable, and if d0 is not zero, sets the carry flag before returning; otherwise, it adds 4 to a7 and returns.
- `update_structure_from_call`: Stores d0 at offset 40 of a structure in a5, checks a byte at offset 219, and if non-zero, loads a value from offset 58 of the structure into d0, calls another routine, and if d0 is non-zero, stores it at offset 44 of the structure before returning.
- `expand_narrator_data_stream_2`: Reads a compressed stream of bytes from a source buffer and expands them into a destination buffer by looking up multi-byte structures in a table.
- `expand_narrator_data_stream`: Reads a compressed stream of bytes from a source buffer and expands them into a destination buffer by looking up multi-byte structures in a table.
- `expand_compressed_data_stream`: Reads a stream of bytes from a source buffer, where each byte acts as an index into a table to retrieve multi-byte data structures.
- `interpolate_buffer_data_2`: Performs linear interpolation on data within a buffer by calculating weighted averages between values based on a table of coefficients.
- `check_audio_status`: Shifts d0 left by 2, loads a longword from a table at a4 plus the shifted d0 into d1, and if bit 21 of d1 is set and bit 1 of the resulting mask is set, increments a0 and branches back to the start of the routine.
- `interpolate_audio_sample_stream`: Iterates through a data stream to perform linear interpolation on a sequence of bytes, using a weight derived from a table to blend values between two source points.
- `interpolate_buffer_data`: Performs a weighted interpolation of bytes between two memory locations using a value in d1 and stores the results back into a buffer pointed to by a6.
- `increment_a6_and_branch`: Increments a6 by 1 and branches to another routine.
- `process_narrator_data_table`: Iterates through a data structure, performing bitwise masking and comparisons on values retrieved from a table and a global variable, then updates a destination table with calculated values and zero-initialized padding.
- `fetch_and_test_data`: Loads a byte from a memory location, masks it, shifts a word in d7, loads a longword from a table using the shifted value, and branches based on testing bits 10 or 11 of that longword; otherwise, it increments a0 and loops back to the start.
- `process_narrator_data_stream_2`: Iterates through a data stream to perform comparisons between values fetched from tables and uses the results to clear or initialize memory blocks in a structure pointed to by a6.
- `compare_table_values_3`: Loads a byte from a0 into d7, uses it to fetch a byte from a table in a4 into d2, compares d2 with d6, and branches based on the result.
- `update_narrator_structure_table`: Iterates through a data stream to perform arithmetic comparisons between values in a table and a stream, then updates a table of structures by writing zeroed values and specific constants at various offsets.
- `process_narrator_data_stream`: Iterates through a data stream to perform arithmetic on bytes and updates a table of structures by writing zeroed values and specific constants to various offsets.
- `calculate_offset_value`: Subtracts the values in d2 and d3 from the byte in d0 and stores the result in d4, then sets d1 to zero and copies the byte from d2 into d1 before branching to another routine.
- `adjust_pointer_and_loop_2`: Clears a6 and the byte at offset 2 of a6, then checks bit 31 of d0 to decide whether to branch; otherwise it copies d6 to offsets 3 and 4 of a6, increments a6 by 8, and loops using d1 as a counter.
- `adjust_pointer_and_loop`: Increments a6 by 8, sets d1 to the value in d3, and then branches to another routine.
- `clear_and_copy_data`: Clears a 16-bit word at the address in a6 and a byte at offset 2 of the address in a6, then enters a loop that copies a byte from d6 to offsets 3 and 4 of the address in a6, increments a6 by 8, and decrements d1 until it reaches zero, otherwise it increments a0 and branches to another routine.
- `reset_registers_and_branch`: Sets d1 to the byte in d0, and sets d3 and d4 to zero, then branches to another routine.
- `filter_and_copy_buffers`: Iterates through three buffers, checking a bit in a global table indexed by the byte value at the first buffer; if the bit is not set, it calls another routine to write the bytes from all three buffers to new locations, and terminates the loop by writing 255 to the current positions in all three buffers.
- `write_byte_to_three_pointers`: Writes the same byte from a register to the memory locations pointed to by three registers.
- `call_subroutines`: Calls two different subroutines in succession and then returns.
- `mask_stream_byte`: Compares a byte at an offset of 616 in a structure to 255; if they are equal, it returns, otherwise it performs a bitwise AND on that byte with 240 and branches to another routine.
- `check_stream_byte`: Loads a byte from an offset of 232 in the structure pointed to by a5 into d0, compares it to 255, and returns if the byte is not 255; otherwise it branches to another routine.
- `process_audio_data_stream`: Iterates through a data stream, using bytes to index into a table to fetch longwords, and updates bytes in a destination buffer by setting various bits based on the fetched values and the contents of the stream.
- `process_linked_list_2`: Sets bit 0 of the byte at offset -1 of a1, then checks bit 7 of d1; if bit 7 is not set, it checks if the byte at a0 is 24 or 23, otherwise it sets bit 1 of the byte at a1 and moves a1 to a3.
- `process_linked_list`: Sets bit 1 of a byte at a memory location and enters a loop that traverses a list of indices to update bits in a destination buffer based on values found in a table.
- `set_bit_and_branch`: Sets bit 2 of a byte at the address in a3 and branches to another routine.
- `set_multiple_bits_and_branch`: Sets bit 0 of the byte at offset -2 of the address in a1, bit 1 of the byte at offset -1 of the address in a1, and bit 2 of the byte at offset -1 of the address in a1, then branches to another routine.
- `populate_division_lookup_table`: Decodes a variable-length bitstream to calculate bit offsets and populates a table with byte values derived from dividing a constant by various values extracted from the stream.
- `calculate_bit_offset`: Masks d0 with 63, tests bit 0 of a byte at the address in a2 and increments a2, branches to an earlier instruction if the bit was set, otherwise it shifts d0 left by 3, adds the result to a6, and branches to another routine.
- `decode_variable_length_stream`: Decodes a stream of variable-length encoded data by reading bytes, masking them, and calculating bit offsets to write values into a destination buffer.
- `parse_variable_length_encoded_data_2`: Loops through a data stream by reading a byte from a1, masking it with 63, checking bit 1 of a byte at a2, and adding the result to d1 after a left shift if the bit is not set.
- `parse_variable_length_encoded_data`: Iterates through a data stream using variable-length bit-packed encoding to calculate and store byte values derived from dividing a 32-bit value by various 16-bit or 8-bit divisors found within the stream.
- `calculate_division_and_branch`: Swaps the high and low words of d5, adds d6 to d5, divides the 32-bit value in d7 by the 16-bit value in d5, stores the byte result at offset -1 of the address in a6, decrements a1 and a2 by 1, and branches to another routine.
- `update_table_with_division_result`: Loads a pointer from a structure, divides a long value in d7 by 110, and writes the resulting byte to a table at an offset derived from the quotient.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `parse_metadata_stream`: Iterates through a data stream using three pointers, performing bitwise checks on metadata flags and byte values to decide whether to call another routine or continue processing.
- `test_metadata_bits`: Shifts register d5 left by two, loads a word from a table at an offset determined by d5 into d3, and branches based on the status of bit 20 or bit 25 of d3, otherwise it moves 24 into d6 and branches to another routine.
- `increment_pointers_and_branch`: Increments a1, a0, and a3 by 1 and branches to another routine.
- `process_text_stream_metadata`: Evaluates a series of bitwise flags and character properties from a data stream to determine control flow, calling other routines based on specific bit patterns and character values found in a table.
- `test_bits_and_call`: Tests bit 12 of d7 and branches if it is not set, then tests bit 9 of d7 and branches if it is not set; otherwise, it loads 38 into d1, calls another routine, and branches.
- `test_bit_and_branch_2`: Tests bit 10 of register d7 and, if not set, calls a subroutine with 38 in register d1 before branching to another location; otherwise, it branches to another location.
- `test_bits_and_branch`: Tests bit 4 of d7 and bit 2 of the byte at offset 1 of a3, then either calls a subroutine or branches to another location.
- `test_bit_and_call_subroutine`: Tests bit 11 of a register and, if not set, loads the value 22 into a register, calls another routine, and then branches to another location; otherwise, it branches to another location.
- `decode_text_stream_with_scaling`: Iterates through a byte stream, performing lookups in tables to determine character properties and applying arithmetic scaling and bitwise transformations to calculate new byte values.
- `check_status_bits_2`: Tests bit 1 of d6 and bit 1 of d7, and if both are set, loads 16 into d1 and calls another routine before branching to a different location.
- `process_encoded_text_stream_2`: Sets d1 to 22 and calls another routine, then branches to another location.
- `process_encoded_text_stream`: Reads a stream of bytes from a source pointer, performs bitwise transformations and arithmetic scaling based on values retrieved from a lookup table, and writes the resulting characters to a destination buffer.
- `compute_scaled_shifted_product`: The routine multiplies the low word of a register by a word input and returns a combined value formed from the high word of the product and the low word shifted right by five bits after adding sixteen.
- `initialize_and_clear_buffers`: The routine stores several offsets relative to a base pointer into a global structure, increments a few of those pointers, clears a set of word fields, then zeroes six blocks of memory by iterating a fixed count.
- `increment_and_call_multiple`: Increments a 16-bit word at offset 136 of a structure in a5, then calls four different subroutines based on the zero and overflow flags before returning.
- `decode_compressed_stream_2`: Decodes a compressed stream by reading bytes from two pointers, checking a value from a global table, and performing arithmetic to write bytes to a destination buffer while updating counters and pointers.
- `process_compressed_stream_byte`: Checks bit 7 of a byte at an offset from a pointer, tests bit 5 of a register, and if certain conditions are met, increments two pointers and loads a new byte into a register before branching to another routine.
- `decode_compressed_stream`: Decodes a stream of data by applying a scaling factor and bitwise transformations to bytes, writing the results to a destination buffer while updating several counters and status flags in a structure.
- `set_flag_and_store_indices`: The routine ORs a constant into a byte addressed by a base pointer and an index, then stores two short values derived from an index and a conditional copy into fields of a global structure.
- `process_narrator_text_stream_2`: Iterates through three parallel streams of data—a control byte stream, a character stream, and a flag stream—to modify a target buffer based on specific control codes.
- `process_flagged_byte`: Shifts a word in d3 left by 2, loads a long word from a table using the result as an offset, and branches based on bit 13 of that word.
- `modify_byte_by_flag`: Checks bit 4 of a byte at an offset from a pointer in a6, and if the resulting nibble is between 1 and 254, it sets bits 3, 2, and 1 of that byte.
- `update_structure_pointers`: Decrements three pointers in a0, a1, and a2, stores these values into offsets 124, 132, and 128 of a structure pointed to by a5, and branches to another location after moving d4 into d0.
- `update_byte_with_conditional_mask`: Performs a bitwise OR of d7 with a byte at an offset in a4, moves d0 to d2, compares a byte at an offset in a6 with 2 and branches if equal, compares a byte at an offset in a6 with 14 and branches if not equal, sets d1 to 255, sets d7 to 16, performs a bitwise OR of d7 with a byte at an offset in a4, and branches to another routine.
- `update_byte_on_condition`: Checks bit 5 of a byte at an offset from a pointer in a4, and if bit 5 is not set, compares a word in d2 to a word in d4, clearing a byte at an offset from a6 if d2 is less than d4.
- `swap_and_clear_byte_flags`: Swaps a byte between two positions in a table addressed by a6 and two indices in d0 and d2, clears the source byte if the indices differ, and loops using d0 as a counter.
- `prepare_set_bits_call`: Loads a pointer from an offset of 112 from a5 into a1 and moves a word from d4 into d0, then branches to another routine.
- `set_bits_in_loop`: The routine iterates through a range of memory using a counter and sets the lowest bit of each byte.
- `or_mask_into_buffer_with_early_exit`: The routine selects a mask based on a control byte, then ORs that mask into a series of bytes in one buffer while checking a flag in another buffer and returning early if the flag is set.
- `set_tile_and_mask`: The routine updates a byte in a screen buffer using a mask derived from a map byte, and optionally masks and sets a flag in another byte based on a scan leftwards in a second buffer.
- `accumulate_table_entries`: Reads a word from offset 140 of a structure in a5, then if a short value at offset 48 is zero and a short value at offset 138 is not zero, it calls another routine four times; otherwise it calls another routine twice.
- `clamp_and_store_value`: Calculates a value based on a word in d6 and a word in d7, clamps the result between 125 and 164, and stores the byte at an address formed by adding a word in d5 to a pointer found at an offset of 96 from a5.
- `clamp_value_and_store_char`: The routine clamps a short input to a lower and upper bound and stores the resulting byte at an address formed from a base pointer and an index.
- `adjust_table_values_loop_2`: Performs a multi-pass transformation on a destination table based on values from two source tables.
- `adjust_table_values_loop`: The routine repeatedly computes an absolute difference between two table entries, scales it and adds the result to a destination byte, then scans backwards through a range using flag bits to locate entries.
- `scale_and_store_byte_2`: Loops through a range of indices, calculating scaled values from multiple tables using bitwise checks and arithmetic, and stores the results into two different destination tables.
- `scale_and_store_byte`: Calculates a value by multiplying a signed word from a table at a0 offset by 65434, then stores the result as a byte into a table at a3 offset after adjusting the value based on its sign.
- `compute_scaled_char_values`: The routine loops over a range of indices, using flag bits to select entries and computing scaled character values from two tables with multiplication and shifts.
- `load_pointers_and_branch`: Loads three values from offsets 104, 112, and 158 of a structure in a5 into registers a0, a1, and d1, moves the value in d1 to d0, and then branches to another location.
- `process_audio_filter_coefficients`: Iterates through audio data buffers to apply gain adjustments and signal processing by performing bitwise shifts, additions, and multiplications on bytes stored at various offsets in memory.
- `transform_and_multiply_data`: Adds 64 to d2, shifts d2 right by 7 bits, and adds the result to a byte at a table offset determined by a3 and d0.
- `transform_data_values`: Adds 64 to d2, shifts d2 right by 7 bits, adds the result to a byte at a table offset determined by a3 and d1, then performs a series of arithmetic operations on a byte from a table at a6 and d0 before branching based on the sign of the result.
- `modify_table_byte_2`: Adds a value derived from d2 to a byte in a table at a6 plus d0, then performs arithmetic on a byte from a6 plus d1 and branches based on the sign of the result.
- `modify_table_byte`: Performs arithmetic operations using bytes from three different tables at offsets a2, a3, and a6, then either subtracts the resulting byte from a table at a3 offset or branches to another routine.
- `add_byte_and_branch`: Adds a byte from d2 to a byte at an offset from a2 and then branches to another routine.
- `apply_audio_filter_and_gain`: Iterates through audio buffers to apply gain adjustments and signal processing based on a mode value, performing bitwise shifts and additions to modify sample values.
- `transform_audio_buffer_data_2`: Adds a value derived from d2 to a byte in a3 plus d0, then multiplies a signed word from a3 plus d1 by d7 and branches based on the sign of the result.
- `transform_audio_buffer_data`: Iterates through a data buffer using a loop to perform arithmetic transformations on bytes based on bitmask checks, applying scaling and offsets.
- `process_audio_buffer_data`: Iterates through a data buffer using a loop to perform arithmetic transformations on bytes based on bitmask checks, applying scaling and offset adjustments to values stored in a table.
- `apply_audio_signal_processing`: Iterates through a data buffer to apply gain adjustments and signal processing by performing bitwise shifts, additions, and multiplications on byte values.
- `scale_byte_value`: Loads a byte from a buffer at offset d0 in a2 into d1, sign-extends it, multiplies it by 102, and then either subtracts 64 or branches to another location based on the sign of the result.
- `apply_scaled_offset`: Adds 64 to d1, shifts d1 right by 7, adds the result to a byte at offset d0 of the buffer in a2, and branches to another location.
- `adjust_tile_heights_with_mask`: Iterates over a byte range using a loop, updates a running maximum in d7 from a byte in a table pointed to by a6, calculates a scaled value by multiplying d7 by 154 and shifting right by 7, and then subtracts this value from a byte in a source buffer before storing the result in a destination buffer.
- `setup_buffer_pointers`: Loads two pointers from a structure at offsets 104 and 104 from a5 into a2 and a0, clears d1, moves d4 into d0, and branches to another location.
- `modify_buffer_with_scaled_offset`: Iterates through a buffer backwards using a loop, and if the high nibble of a byte is non-zero, it calculates a signed offset based on the high bit of that byte and adds a scaled version of a byte from another buffer to the current byte.
- `scale_buffer_by_signed_factor`: Adds 64 to d1, shifts d1 right by 7, adds the result to a byte at offset d0 of the buffer in a2, and repeats the process through a loop until d0 reaches zero.
- `process_audio_buffer_differential`: Iterates through a buffer to perform differential arithmetic on bytes, using a control table to determine bitwise masking and applying corrections to multiple destination buffers based on specific bit flags.
- `copy_and_increment_bytes`: Iterates through a range using a loop, copies a byte from one location to another, subtracts a byte from a register from the destination byte, and then adds 5 to a byte at a location pointed to by a6.
- `process_masked_audio_buffer`: Iterates through a buffer to perform bitwise masking and arithmetic operations on bytes, using a control table to determine bit manipulation patterns and applying incremental adjustments to multiple destination buffers based on the nibble values.
- `update_buffer_with_masked_nibble`: Iterates through a range using a loop, copies a byte from a source buffer to a destination buffer and back, and then adds a value derived from the low nibble of a byte in a fourth buffer to a byte at a location pointed to by a6.
- `load_address_and_call_2`: Loads the address stored at a longword at offset 10708 into register a4, calls a subroutine, and then branches to another location.
- `load_address_and_branch_7`: Loads a long value into a4, calls another routine, and then branches to another location.
- `load_initial_values`: Loads a long word from an offset of 108 from a structure in a5 into a4, sets d6 to 110, and moves d5 into d0 before branching to another location.
- `transform_text_buffer_encoding`: Iterates through a buffer to perform bitwise transformations and arithmetic adjustments on byte values.
- `process_text_buffer`: Decrements a counter in d0 while checking a bit in a buffer, calculates a divisor, and if non-zero, enters a loop that copies bytes between buffers, subtracts a calculated value from them, and updates a third buffer.
- `process_buffer_and_calculate_max`: Copies bytes from one buffer to another while subtracting a byte value, then if a specific byte is a backspace, it finds the maximum byte in a second buffer, calculates a scaled value, and writes it to two locations; otherwise, it writes specific character constants to the first buffer.
- `initialize_registers_and_branch`: Clears d7 and d1, moves a 16-bit value from d4 to d0, and then branches to another location.
- `accumulate_bytes_and_write_results`: The routine accumulates bytes from a source area into a register, keeps a running maximum, derives a scaled character value, and writes that value and a derived value to several destination buffers with a conditional copy.
- `decrement_pointers_and_branch`: Decrements the values at offsets 124, 128, and 132 of a structure in a5, sets register d2 to zero, moves the word in d4 to d0, and then branches to another location.
- `apply_text_scaling_transformations`: Iterates through a buffer in a loop, performing conditional arithmetic on bytes by looking up scaling factors and bit flags in a table and applying the results to multiple destination buffers via pointers stored in a structure.
- `calculate_scaled_offset_2`: Loads a long word from an offset of 96 from a structure in a5 into a6, loads a byte from a table at offset d0 from a6 into d1, subtracts 110 from d1, multiplies d1 by 26, and either subtracts 64 or branches to another location based on the sign.
- `adjust_text_buffer_values_2`: Iterates through a buffer and modifies bytes by adding or subtracting values derived from a table lookup and a scaling factor.
- `adjust_text_buffer_values`: Iterates through a buffer to modify bytes by adding or subtracting a calculated value derived from a table lookup and a scaling factor.
- `compare_and_branch_3`: Decrements d7, compares it to d0, and branches to another location if they are equal; otherwise, it sets d7 to 64 and branches to another location.
- `set_value_and_branch`: Sets d7 to 86 and branches to another location.
- `calculate_scaled_value`: Loads two pointers from a table, reads a byte from the first pointer, extends it to a word, multiplies it by 26, and if the result is negative, subtracts 64 from it.
- `apply_signed_scaling_adjustments`: Adds 64 to register d2, performs an arithmetic shift right by 7 bits, then adds the resulting byte to a buffer at offset d0 of a structure in a6 and subtracts it from a buffer at offset d0 of a structure in a3, looping until d0 reaches zero.
- `branch_to_next_routine`: Performs a short branch to another location.
- `copy_and_adjust_bytes_with_feedback`: The routine copies bytes between buffers, updates a feedback value by multiplying it with a table entry and shifting, subtracts the feedback from a byte, and writes adjusted bytes to two output areas.
- `check_narrator_flag_and_dispatch`: Calls another routine twelve times and then checks if a byte at offset 219 of a structure in a5 is non-zero, otherwise it returns.
- `fetch_three_bytes`: Loads a pointer from an offset of 40 in a structure, adds 3, 4, and 5 to that pointer sequentially, and calls another routine after each addition.
- `interpolate_and_store_bytes`: The routine reads two bytes from a buffer, computes a scaled difference based on an index, accumulates the result, and stores successive byte results into the buffer at regular intervals.
- `fetch_and_check_byte`: Increments d1 by 8, loads a byte from the table at a0 plus the new d1 offset into d6, and returns if d6 is 255; otherwise it branches to another location.
- `check_conditions_and_increment`: Compares d6 to 254 and branches to another location if equal; otherwise, it compares d1 to 8 and, if not equal, branches to another location; if equal, it adds 8 to a0 and branches to another location.
- `call_interpolation_and_branch`: Calls another routine and then branches to another location.
- `test_table_byte`: Increments d1 by 8, tests the byte at a0 plus the new d1 offset, and returns if that byte is 255; otherwise it branches to another location if the byte is 0 or if the byte is not 255.
- `increment_pointer_in_loop`: Compares d1 to 8 and, if not equal, branches to another location; if equal, it adds 8 to a0 and branches to another location.
- `scan_string_blocks`: Calls interpolate_and_store_bytes and then iterates through a buffer in 8-byte increments, searching for a null terminator or a byte with all bits set to one.
- `decode_buffer_data`: Processes a buffer by reading bytes to determine offsets and values, then modifies a destination string by adding characters or copying blocks of data from a table based on bitwise checks of a global variable.
- `decode_and_copy_buffer`: Contains a loop that processes data from a buffer.
- `offset_pointer_and_branch`: Shifts d0 left by 3, adds the result to a0, and then branches to another location.
- `shift_and_compare_d3`: Performs a logical shift right on d3 by 2 bits, compares the result to 42, and if equal, sets d3 to 96 before branching to another location.
- `update_register_value`: Compares d3 to 43 and, if equal, sets d3 to 97 and branches to another location; otherwise it branches to another location.
- `modify_byte_and_branch`: Compares d3 to 44 and, if not equal, branches to another location; if equal, it sets d3 to 98 and branches to another location.
- `copy_bytes_from_offset_table`: Copies six bytes from a table located at an offset determined by a byte in a register to a destination pointed to by a register, then increments the destination pointer by two and repeats this process for a count specified in a register.
- `initialize_structure_pointers`: Loads three addresses into a0, a1, and a2 from offsets of a structure in a5, loads a long value into a4, loads a pointer from an offset of a5 into a6, and sets d6 to 12288, d4 to 0, d4 to 240, d5 to 255, and d7 to 63 before branching to another location.
- `set_bit_and_compare_byte`: Sets bit 31 of d4, clears d0 and d2, loads a byte from the address in a0 and increments a0, then compares that byte to d5 and returns if they are equal.
- `decode_bitstream_structure`: Iterates through an encoded data stream to perform bitwise masking and logical operations on a structure pointed to by a6.
- `process_encoded_stream`: Iterates through a data stream, performing bitwise masking and logical operations on a structure in a6 based on values read from the stream.
- `calculate_scaled_offset`: Shifts the value in register d2 left by 3 bits, adds the result to the address in a6, and branches to another routine.
- `replace_bytes_from_table`: Iterates through a buffer using a pointer, replacing bytes with values from a global table based on the byte value, skipping eight bytes if a specific marker is encountered, or stopping if a termination marker is found.
- `copy_bytes_with_index`: Uses a byte from a table at offset d0 of a1 to write to a destination in a0, then increments a0 and updates d0 from the next byte in a0 before looping back to an earlier instruction.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `compare_byte_at_offset`: Loads a byte from a structure at offset 48 of a6 into d3 and compares it to 255, otherwise it returns.
- `update_scaled_value`: Adds d3 to d0, performs a loop using d1, and inside the loop updates a byte at offset 18 of a6 by adding d3 to d0 and shifting the result right by 3 bits.
- `compare_table_byte_to_constant`: Loads a byte from a table at an offset calculated by multiplying six by eight and adds it to the address in a6, then compares this byte to a constant value of 255 and otherwise returns.
- `process_data_table`: Iterates through a table, updating a byte at offset 24 of a6 using a calculation involving a value from the table and other data, then increments a6 by 8 and resets a counter when a specific byte is reached.
- `loop_until_end_marker`: Loads four pointers from various locations and a pointer from an offset of 40 in a structure, then enters a loop that loads a byte from the pointer in a0, increments a0, and continues if the byte is not 255.
- `decode_text_stream`: Iterates through a data stream to decode characters, updating a structure's internal state and counters by adding, subtracting, or shifting values based on the character encountered.
- `add_constant_in_loop`: Adds 10 to a byte at offset 7 of a6, decrements d2, and loops using d2 until it reaches zero before branching to another routine.
- `load_constant_and_branch`: Loads the value 250 into d3 and branches to another routine.
- `load_negative_constant_and_branch`: Loads the value negative 6 in register d3 and branches into another routine.
- `load_negative_constant_and_branch_2`: Loads the value negative six into register d3 and branches into the middle of another routine.
- `write_fixed_byte_loop`: Writes the byte value 230 to offset 7 of a6, decrements d2, and loops using d2 until it reaches zero before branching to another routine.
- `modify_byte_with_offset`: Decrements d2, reads a byte from a6, subtracts 43, shifts it right by 2, adds the result to offset 7 of a6, and loops using d2 until it reaches zero before branching to another routine.
- `decode_nibble_checksum_bytes`: Reads a pointer and a count from a structure pointed to by A5 and processes a byte array in two passes.
- `update_task_status`: Calls three functions via pointers stored in a global variable to retrieve values, stores these values at offsets 30, 31, and 32 of a structure in a1, writes a bitmask to offset 308 of that structure, and enters a loop that calls another routine and a function from the global variable.
- `manage_device_status`: Enters a loop that checks bit 6 of a byte at offset 34 of a structure in a2.
- `clear_status_bits`: Clears bit 1 and bit 0 of a byte at offset 34 of a structure pointed to by A2.
- `prepare_speech_io_request`: Populates a structure with speech timing and buffer data, performs a series of loops to initialize a data stream and clear specific memory regions, and calls initialize_narrator_text_buffer_2.
- `initialize_narrator_text_buffer_2`: Copies a data stream into a buffer, populates a structure with parsed character data and timing values, and then executes a list of function pointers from a global variable.
- `decode_text_stream_data_2`: Iterates through a data stream to decode characters by performing bitwise operations and lookups in a table, updating a structure's internal state and counters.
- `parse_text_stream`: Iterates through a data stream to decode characters, updating a structure's internal state and counters.
- `decode_text_stream_data_2_2`: Iterates through a data stream to decode characters by performing bitwise operations and lookups in a table, updating a structure's internal state.
- `decode_text_stream_data`: Iterates through a data stream to decode characters by performing bitwise operations and lookups in a table, updating a structure's internal counters and state.
- `process_text_stream_data`: Iterates through a data stream to decode characters, updating a structure's internal state and counters, and calls another routine to handle specific control sequences or character processing.
- `update_status_word`: Decrements a word in register d0, branches if it is not positive, otherwise it writes the word from d5 to offset 16 of the structure in a5 and branches to another routine.
- `execute_callback_list`: Checks a bit at offset 210 of a5, potentially writes to a hardware register, and then executes a series of function pointers from a global variable in a loop based on a value at offset 228 of a5.
- `enable_dma_if_flag_set`: Tests a flag bit at offset 210 of a structure pointed to by A5 and, if set, writes a value to a custom hardware register that enables DMA channels by setting bit 15 of DMACON.
- `manage_interrupt_state`: Updates a global variable, increments a byte at offset 294 of a6, and if certain conditions regarding a structure in a5 and a4 are met, updates several fields in a structure and calls another routine.
- `decrement_counter_and_enable_interrupts`: Decrements a byte at offset 294 of a structure pointed to by A6 and, if the byte was zero or one before the decrement, writes a value to a global variable that sets the INTENA register with bit 15 set, which enables interrupts.
- `calculate_offset_address`: Loads a byte from an offset of 68 of a structure in a3, multiplies it by 68, and if a byte at offset 22 of a structure in a5 is not zero, calculates a new address by adding the result to a pointer retrieved from an offset of 236 of a structure in a4.
- `process_dma_list`: Loops through a list of structures to set bit 0 of a byte at offset 30, calls a function pointer from each structure, and then performs conditional logic involving bitwise operations and function calls.
- `process_dma_loop`: Decrements a counter in a register and loops to call another routine, setting bit 5 of a byte at offset 30 of a structure in a3 and writing to a hardware register if a byte at offset 30 of a5 has bit 6 set.
- `set_bit_and_branch_2`: Sets bit 5 of a byte at offset 30 of a3 and branches to an earlier instruction.

### Qume_LetterPro_20

- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `execute_triple_call_2`: Calls a function pointer from a global variable three times and, if all calls return non-zero, returns 0, otherwise it calls another routine using a pointer from a global variable and returns -1.
- `call_function_triple`: Calls the same function pointer three times via a structure and returns a value of negative one.
- `execute_handler_sequence`: Calls a routine through a pointer in a global table, then calls that same routine twice more with the value from A6 as an argument.
- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_command_type`: Restores registers from the stack, loads three pointers from the stack into a0, a1, and a4, clears d1, d2, and d3, and branches to another routine if the word at the pointer in a0 is not equal to 1.
- `process_printer_command_sequence_3`: Copies a sequence of bytes from a table into a destination buffer and updates the buffer based on command codes read from a structure.
- `copy_byte_from_table`: Copies a byte from a table at address zero using an index in D0 to a destination at the sum of A3 and A4, then branches to another routine.
- `update_word_value_2`: Compares a word at the address in a0 to 27; if it matches and a byte at the address in a1 is not negative, it writes 32 to the word at a0, writes 1 to the byte at a1, and branches to another routine.
- `update_word_value`: Compares a word at a pointer in a0 to 27; if it matches and a byte at a pointer in a1 is less than zero, it writes 4 to the word at a0 and branches to another routine, otherwise it returns.
- `update_threshold_values_2`: Compares a word at a pointer in a0 to 28; if it matches and a byte at a pointer in a1 is non-zero, it writes 33 to the word at a0 and then branches to another routine.
- `update_threshold_values`: Compares a word at a pointer in a0 to 29; if it matches and a byte at a pointer in a1 is not zero, it writes 33 to the word at a0 and 255 to the byte at a1.
- `check_and_update_threshold`: Compares a word at a pointer in a0 to 29; if it is not equal, it returns to another routine, otherwise it tests if a byte at a pointer in a1 is greater than 0, and if so, writes the word 2 to the location in a0 before branching to another routine.
- `update_word_and_byte_threshold`: Compares a word at a pointer in a0 to 30; if it is not equal, it branches to another routine, otherwise it tests if a byte at a pointer in a1 is less than 0, and if so, writes 32 to the word at the pointer in a0 before branching to another routine.
- `update_word_based_on_byte`: Compares a word at a pointer in a0 to 31; if it is not equal, it branches to another routine; otherwise, it tests a byte at a pointer in a1, and depending on whether that byte is greater than 0 or less than or equal to 0, writes either 33 or 32 to the word at the pointer in a0, then branches to another routine or clears the byte at the pointer in a1.
- `increment_byte_on_match`: Compares a word at a pointer in a0 to 32 and, if it is not equal, branches to another routine; otherwise it increments a byte at a pointer in a1 and branches to another routine.
- `check_command_byte`: Compares a word at a pointer in a0 to 33, and if it is not equal, branches to another routine; otherwise it decrements a byte at a pointer in a1 and branches to another routine.
- `format_status_string`: If a word at a pointer in a0 is 39, it writes several bytes to a destination in a4, including a character derived from a stack value; otherwise, if the word is 0, it writes 253 to offset 680 of a structure in d1.
- `add_constant_to_byte`: Adds 48 to a sign-extended word in D0 and stores the resulting byte at the address in A3, then branches to another routine.
- `write_status_byte`: Checks if a word at the address in A0 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in D1.

### CBM_MPS1000

- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `run_dual_subroutines`: Calls a function pointer from a global variable and then calls another function pointer from a different global variable twice, returning 0 if all succeed or -1 if they fail.
- `call_two_subroutines_3`: Calls two different subroutines using pointers from a table and then branches to another routine.
- `execute_indirect_function_calls`: Fetches a function pointer from a global variable using an offset and calls that function three times, passing the contents of register a6 as an argument to the second and third calls.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_2`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and another routine through a pointer stored within that structure.
- `manage_printer_buffer`: Processes printer buffer operations via a switch statement, including clearing memory, writing control characters and data, and performing recursive buffer management calls.
- `restore_registers_and_return`: Clears register d0 and restores d2, d3, a2, a3, and a4 from the stack before returning.
- `initialize_printer_parameters`: Saves registers to the stack, loads three pointers from the stack into a2, a3, and a4, clears registers d1, d4, and d5, sets d3 to 3, and branches to another routine if the word at the pointer in a2 is not 1.
- `format_printer_command_buffer_4`: Copies a sequence of bytes from a table into a destination buffer, prepending specific control characters based on values read from a structure pointed to by a register.
- `modify_buffer_content`: Loads a pointer from a structure in d1, compares a word at an offset of 2048 from that pointer to 2048, and if they match, writes the byte 15 to a location calculated from d4 and a4.
- `process_printer_command_sequence`: Copies a null-terminated string to a destination buffer and then processes a command byte by either appending specific data sequences, toggling state flags, or updating status values in a structure based on the command type.
- `copy_data_loop_3`: Copies bytes from a table at a1 to a destination at a0 plus an offset in a4 using a loop controlled by d5 and d4.
- `validate_and_update_printer_status`: Compares a word at a pointer in a2 to 66; if it is not equal, it branches to another routine, otherwise it sets a byte to 1 at a location pointed to by a pointer in d1, checks if a word at offset 214 of that pointer is 2048, and if so, writes 60 to a byte at a location pointed to by d3 before branching to another routine.
- `write_byte_to_pointer`: Loads a pointer from a structure in a0 into a1, compares a word at offset 86 of that pointer to 1024, and if it is not equal, branches to another routine; otherwise it writes the byte 137 to the address in a0 which was loaded from d3.
- `set_initial_byte`: Moves the value in d3 to a0, writes the byte value 80 to the address in a0, and branches to another routine.
- `transfer_bytes_loop`: Performs a loop that copies bytes from a table at a1 to a destination at a0 plus an offset in a4, incrementing d4 and d5 during each iteration.
- `modify_word_and_byte`: Compares a word at a pointer in a2 to 32; if it is equal, it tests a byte at a pointer in a3, and if that byte is not zero, it writes 1 to the byte at the pointer in a3 and 27 to the word at the pointer in a2, then branches to another routine.
- `clear_byte_conditional`: Tests a byte at a3; if it is not positive, it clears the byte at a3 and writes 30 to a word at a2, then branches to another routine.
- `branch_with_constant`: Sets D0 to 255 and branches to another routine.
- `update_status_values_3`: Compares a word at a pointer in a2 to 33 and, if it is not equal, branches to another routine; otherwise it tests a byte at a pointer in a3, and if that byte is not zero, branches to another routine, otherwise it writes the byte 255 to the address in a3 and the word 29 to the address in a2 before branching to another routine.
- `clear_status_byte_4`: Tests a byte at a pointer in a3, and if it is less than or equal to zero, clears that byte, writes the word 28 to a pointer in a2, and branches to another routine; otherwise it branches to another routine.
- `branch_to_offset`: Performs a short branch to another routine.
- `update_status_values`: Checks the value at the address in A2 and writes specific bytes to either a stack location, a destination at A4 plus an offset, or a structure at offset 2720 of the pointer in D1.
- `write_status_byte_alt`: Checks if a word at the address in A2 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in D1.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_630

- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_library_function_pointers`: Attempts to execute three different functions at specific offsets from a pointer stored in a global variable, returning zero if all three calls succeed or a failure value otherwise; if any call fails, it calls another routine using an offset from the global variable before returning.
- `execute_dual_pointers`: Loads a pointer from a table, saves a6 to the stack, calls a function at an offset from a6, restores a6, loads a second pointer, saves a6 again, calls a second function, restores a6, and branches to another routine.
- `call_triple_handler`: Calls a routine through a pointer found at an offset from a global base three times, passing the value from a6 on the second and third calls.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_command_parameters`: Saves registers to the stack, loads three pointers from the stack, clears three registers, and compares the word at the first pointer to one before branching to another routine or continuing to a different instruction.
- `process_input_state_machine`: Copies data from a structure into a local buffer, performs a string copy from a source pointer, and then updates a state variable through a series of conditional checks and increments based on the current value of a character and an integer.
- `process_string_and_update_state`: Copies a null-terminated string from a source to a destination buffer, then performs a series of conditional checks on a word and a byte to update the word value and modify the byte based on specific numeric ranges and constants.
- `copy_byte_from_table_2`: Copies a byte from a table at a zero offset into a destination address calculated from d2, d3, and a4, then branches to another routine.
- `update_threshold_values_3`: Compares a word at a pointer in a0 to 27; if it matches and a byte at a pointer in a1 is not positive, it writes 32 to the word at a0, writes 1 to the byte at a1, and branches to another routine.
- `check_threshold_match`: Compares a word at a pointer in a0 to 27; if it matches and a byte at a pointer in a1 is less than zero, it writes 4 to the word at a0 and returns to another routine via a branch.
- `update_threshold_on_positive_byte`: Compares a word at a pointer in a0 to 28; if it matches and a byte at a pointer in a1 is greater than zero, it writes 33 to the word at a0.
- `update_threshold_on_positive_byte_alt_2`: Compares a word at a pointer in a0 to 29; if it matches and a byte at a pointer in a1 is greater than zero, it writes 33 to the word at a0, writes 255 to the byte at a1, and branches to another routine; otherwise it returns.
- `update_threshold_on_positive_byte_alt`: Compares a word at a pointer in a0 to 29; if it matches and a byte at a pointer in a1 is greater than zero, it writes 2 to the word at a0.
- `update_threshold_on_value_match`: Compares a word at a pointer in a0 to 30; if equal, it tests a byte at a pointer in a1 and, if that byte is less than 0, writes 32 to the word at the pointer in a0, then branches to another routine; otherwise it returns.
- `update_word_and_clear_byte`: Compares a word at a pointer in a0 to 31; if it is equal, it tests a byte at a pointer in a1, and depending on whether that byte is greater than 0 or less than or equal to 0, writes either 33 or 32 to the word at the pointer in a0, then branches to another routine or clears the byte at the pointer in a1.
- `increment_byte_on_mismatch`: Compares a word at a pointer in a0 to thirty-two; if they are not equal, it increments a byte at a pointer in a1 and branches to another routine.
- `decrement_byte_on_mismatch`: Compares a word at a pointer in a0 to thirty-three; if they are not equal, it decrements a byte at a pointer in a1 and branches to another routine, otherwise it branches to another routine.
- `write_conditional_bytes`: If a word at a pointer in a0 is 12, it writes 27 to a destination in a4 plus an offset in d2, and writes either 65 or 66 to the next byte depending on a stack value; otherwise, if the word is 0, it writes 253 to offset 680 of a structure in d1.
- `write_constant_byte`: Writes the byte 66 to a destination address calculated from d2 and a4, then branches to another routine.
- `write_status_byte_to_structure`: Checks if a word at the pointer in a0 is non-zero; if it is zero, it writes the byte 253 to an offset of 2720 in the structure pointed to by the value in d1, then returns.

### ImagewriterII

- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_callbacks`: Stores a parameter and a pointer to a global variable into a structure, then calls three successive functions via pointers found at specific offsets from that global variable and a second global variable, calling a cleanup routine if certain conditions are met.
- `execute_dual_function_pointers`: Loads two different function pointers from global variables and calls each one using a value from a table in a6, then returns zero.
- `execute_indirect_function_calls_2`: Fetches a function pointer from a table using an offset and calls that function three times in succession.
- `return_constant_20`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_21`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_3`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes four routines through a global base and a local pointer.
- `POSDIV`: Performs signed division of a 32-bit value in d1 by a 16-bit value in d2, returning the quotient and remainder in d1 and d2 respectively.
- `LONGDIV`: Performs a 32-bit division by rotating a 32-bit value left bit by bit for sixteen iterations, subtracting a value from a register if the high half meets or exceeds a threshold.
- `ulmult`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components.
- `ulmodt`: Moves the contents of register d1 into d2, moves the contents of register d0 into d1, calls POSDIV, and returns.
- `uldivt`: Moves the contents of register d0 into d1, moves the contents of d2 into d1, calls another routine, and returns the value left in d1 via d0.
- `lmodt`: Calls POSDIV and negates the returned value if the flag left in an address register by the called routine is nonzero.
- `ldivt`: Calls POSDIV and returns its first result, or the negation of that result if the second result in an address register is nonzero.
- `dispatch_via_table`: Saves registers to the stack, reads a byte from the stack, and if the byte is 4 or less, uses it to index into a table to jump to another routine; otherwise it jumps to a different address.
- `clear_d0_and_restore_registers_alt`: Clears register d0 and restores d2 through d5 and a2 through a4 from the stack before returning.
- `initialize_printer_state`: Saves registers d2 through d4 and a2 through a4 to the stack, loads a pointer from offset 28 of the stack into a1, loads d2 from offset 32 of the stack, loads a3 from offset 36 of the stack, clears d3, d4, and a4, and branches to another routine if the 16-bit word at the pointer in a1 is not equal to 1.
- `format_imagewriter_escape_sequences`: Copies a sequence of bytes from a table into a destination buffer and appends various escape sequences based on values read from a structure.
- `translate_imagewriter_escape_sequences_4`: Translates internal control codes into specific escape sequences for an ImageWriter II printer by writing bytes to a destination buffer.
- `emit_imagewriter_escape_sequences_3`: Translates specific control codes into corresponding escape sequences for an ImageWriter II printer.
- `modify_imagewriter_sequence`: Compares a word at a pointer in a1 to 31; if it matches and a byte at a pointer in a3 is greater than or equal to zero, it writes 33 to the word at a1, writes 1 to the byte at a3, and branches to another routine.
- `modify_byte_and_word`: Tests a byte at a pointer in a3; if it is less than zero, it writes 32 to the word at a pointer in a1, sets the byte at a3 to 255, and branches to another routine.
- `branch_to_address_2`: Branches to a specific address.
- `translate_imagewriter_escape_sequences_3`: Translates input characters into Apple ImageWriter escape sequences by writing specific byte combinations to a destination buffer.
- `write_four_bytes`: Writes the bytes 66, 27, 108, and 48 to a destination address calculated from d2 and d3.
- `translate_imagewriter_escape_sequences_2`: Translates input characters into Apple ImageWriter escape sequences by writing specific byte combinations to a destination buffer.
- `write_byte_and_branch`: Calculates an address by adding d2 and d3, writes the byte 66 to that address, increments d3, and branches to another routine.
- `translate_imagewriter_escape_sequences`: Reads a byte from a source pointer and translates it into various Apple ImageWriter escape sequences by writing specific byte combinations to a destination buffer.
- `write_byte_and_increment`: Writes the byte 80 to a destination address calculated from d2 and d3, then increments d3 and branches to another routine.
- `emit_imagewriter_escape_sequences_2`: Translates specific control codes into Apple LaserWriter/ImageWriter escape sequences by writing bytes to a destination buffer.
- `write_byte_and_increment_70`: Writes the byte 112 to a destination address calculated from d2 and d3, then increments d3 and branches to another routine.
- `emit_imagewriter_escape_sequences`: Checks a value via a pointer in a1 and writes specific escape sequences to a destination buffer calculated from d2 and d3.
- `write_encoded_printer_sequence`: Checks a word at a pointer in a1 and, depending on its value, writes a sequence of bytes to a destination calculated from d2 and d3, including an escape character and a character determined by a global variable or a counter stored in a stack location.
- `conditional_write_byte`: Compares a byte at a memory location to 1, and if not equal, branches to another routine; otherwise, it writes the byte 80 to an address calculated from d2 and d3, increments d3, and branches to another routine.
- `update_printer_status_byte`: Increments a value in a register, calculates a new address by adding two registers together, writes the byte seventy to that address, and branches to another routine.
- `write_encoded_sequence_2`: Checks a word at a pointer in a1 and, depending on its value, writes specific byte sequences to a destination calculated from d2 and d3, while also updating a value in a stack location.
- `conditional_write_byte_alt`: Compares a byte at a memory location to 1, and if not equal, branches to another routine; otherwise, it writes the byte 69 to an address calculated from d2 and d3, increments d3, and branches to another routine.
- `write_byte_and_increment_4e`: Writes the byte 78 to a destination address calculated from d2 and d3, then increments d3 and branches to another routine.
- `write_encoded_sequence`: Checks a word at a pointer in a1 and, depending on whether it is 48 or 12, writes specific byte sequences to a destination calculated from d2 and d3 while updating a counter on the stack.
- `check_and_modify_bytes`: Compares a word at a pointer in a1 to 12, and if not equal, branches to another routine; otherwise, it checks a byte at a pointer from the stack, and if it matches 39, writes 30 to that location; it then checks if a byte at a pointer from the stack is at least 37, and if so, branches to another routine.
- `write_mixed_bytes`: Writes the bytes 27 and 75 to a destination address calculated from d2 and d3, then adds a byte from a global table to the next position and returns.
- `clear_d0_and_restore_registers`: Clears register d0 and restores d2 through d4 and a2 through a4 from the stack before returning.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_C-150

- `return_constant_22`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_release_resource_pointers`: Attempts to call three consecutive functions via offsets from a pointer stored in a global variable, and if any call fails, it calls a cleanup routine using the same pointer and returns a failure code; otherwise, it returns a success code.
- `call_dual_function_pointers`: Loads two function pointers from a global structure into a1 and a6, calls the first via a pointer at offset minus 406 of a6, then calls the second via a pointer at offset minus 406 of a6 after restoring a6, and finally returns 255 in d0.
- `execute_indirect_function_calls_3`: Fetches a function pointer from a global variable using a specific offset and calls that function three times in succession.
- `return_constant_23`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_24`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_4`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes several routines through a global base using a pointer retrieved from that structure.
- `divide_32_bit_integers`: Calls a library function using a base from a global variable to perform a division operation on 32-bit values.
- `rotate_and_threshold_32bit`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_4`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit halves.
- `negate_string_length_on_flag`: Calls measure_string_length and returns its result, or returns the negation of that result if the second return value in an address register is nonzero.
- `execute_jump_table_2`: Loads values from the stack, performs a comparison on a byte from the stack, and jumps to an address from a table using an offset derived from that byte.
- `return_zero`: Sets d0 to zero and returns.
- `process_command_data`: Updates various memory locations and buffers based on the value of a word at a pointer in a1, including performing calculations for a byte at a pointer in a6 and calling another routine.
- `update_indexed_byte`: Increments d4 and d5, then writes a byte from a table at address a0 plus offset d1 to a destination at address a2 plus offset d3.
- `update_structure_based_on_value`: If a 16-bit value at a pointer in a3 is 12, it performs calculations involving a stack value and a byte from a stack location to update a value in a stack location, then writes three bytes to a structure at a pointer in a3; otherwise, if the 16-bit value is 0, it writes the byte 253 to offset 2720 of a structure pointed to by a4.
- `write_byte_to_offset_2`: Tests a 16-bit value at a pointer in a3; if it is zero, it writes the byte 253 to offset 2720 of a structure pointed to by a4, then calls another routine that returns a string length or its negation, and finally writes a byte from a global table to offset 3 of the structure at a4 + 4.
- `write_byte_to_offset`: Tests a 16-bit value at a pointer in a3; if it is zero, it writes the byte 253 to offset 2720 of a structure pointed to by a4.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### generic

- `return_constant_25`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_26`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_27`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_2`: Returns a constant zero value.

### Brother_HR-15XL

- `return_constant_28`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_function_calls`: Calls three functions via pointers stored at an offset of 552 from a global variable and a pointer in a0, then calls another routine using a pointer at an offset of 414 from a global variable if certain conditions are met.
- `call_triple_function_pointers`: Loads two function pointers from a global structure into a1 and a6, calls the first via a pointer at offset minus 406 of a6, restores a6, loads a different pointer into a1, restores a6, and calls the second via a pointer at offset minus 406 of a6 before returning 255 in d0.
- `execute_library_calls`: Calls a library function via a pointer at an offset from a global base, then calls that same function twice more with a value from a6 as an argument.
- `return_constant_29`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_30`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_31`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_parameters_2`: Sets registers a0, a1, and a4 from stack offsets 32, 28, and 24, clears d1, d2, and d3, and compares a word at the address in a0 to 1, branching to a different location if it is not 1 or to offset 54 if it is 1.
- `process_printer_control_sequence`: Copies a sequence of bytes from a source buffer into a destination buffer, applying specific control characters based on values read from a structure.
- `process_printer_control_sequence_2`: Copies a null-terminated string to a destination buffer and then processes a series of control codes by updating a word at a pointer in a0 and a byte at a pointer in a1 based on specific conditional logic.
- `write_byte_from_table`: Increments d2 and d3, then writes a byte from a table at address a3 plus offset d0 to a destination at address a2 plus offset d2.
- `update_conditional_word_2`: Compares a word at the pointer in a0 to 27; if it matches and the byte at the pointer in a1 is less than 0, it writes 27 to the word at a0 and 1 to the byte at a1, then branches to another routine.
- `update_status_on_match_3`: Compares a word at a pointer in a0 to 27; if it matches and the byte at a pointer in a1 is less than 0, it writes 4 to the word at a0 and branches to another routine.
- `update_status_on_match_2_2`: Compares a word at a pointer in a0 to 28; if it matches and the byte at a pointer in a1 is greater than or equal to 0, it writes 33 to the word at the pointer in a0, then branches to another routine; otherwise it returns.
- `update_status_on_match_2`: Compares a word at a pointer in a0 to 29; if it is equal and the byte at a pointer in a1 is greater than or equal to 0, it writes 33 to the word in a0, writes 255 to the byte in a1, and branches to another routine; otherwise it returns.
- `update_value_if_byte_positive`: Compares a word at a pointer in a0 to 29; if it is equal and the byte at a pointer in a1 is greater than or equal to 0, it writes 2 to the word at the pointer in a0 and branches to another routine, otherwise it returns.
- `update_word_if_byte_negative`: Compares a word at a pointer in a0 to 30; if it is equal and the byte at a pointer in a1 is less than 0, it writes 32 to the word at the pointer in a0, then branches to another routine.
- `update_value_on_condition_3`: Compares a 16-bit value at the address in a0 to 31; if it is not 31, it branches.
- `increment_value_on_match`: Compares a 16-bit value at the address in a0 to 32; if it is not 32, it branches.
- `decrement_byte_on_match_2`: Compares a 16-bit value at a0 to 33; if it is not 33, it returns, otherwise it decrements the byte at the address in a1 and branches to another location.
- `check_and_write_status`: If the word at a0 is 12, it writes 27 to offset 0 of a structure at a4 + d2, and writes either 65 or 66 to offset 1 depending on a stack value; otherwise, if the word at a0 is zero, it writes 253 to offset 2720 of a structure at d1 + 2720.
- `write_constant_byte_2`: Increments d2, calculates a destination address by adding a4 to a2, writes the byte 66 to that address, and branches.
- `write_error_flag_to_structure`: Checks if the word at the pointer in a0 is zero; if it is, writes the byte 253 to an offset of 2720 of the structure pointed to by d1, then returns.

### HP_LaserJet

- `return_constant_32`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_calls`: Calls three consecutive functions using pointers retrieved from a structure at a global variable; if any call returns a non-zero value, it calls another routine with the global variable as an argument before returning 255, otherwise it returns 0.
- `execute_dual_function_pointers_2`: Loads a pointer from a global variable into a1, then calls a routine via a pointer stored in a global variable using an offset of 414.
- `execute_indirect_function_calls_4`: Fetches a function pointer from a global variable using an offset and calls that function three times consecutively.
- `return_constant_33`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_5`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes four routines through a global base and a pointer retrieved from the structure.
- `process_hp_laserjet_state_machine`: Updates a state machine based on input control characters and printer status flags, copying bytes from a table into a buffer and calling format_hp_laserjet_command, process_imagewriter_escape_sequences, or convert_value_to_ascii depending on the current state and input values.
- `format_hp_laserjet_command`: Constructs an HP ImageWriter II command sequence by copying bytes from a table into a buffer, adjusting values based on printer status flags, and calling another routine to emit the resulting data.
- `process_imagewriter_escape_sequences`: Translates control characters into HP ImageWriter II escape sequences by updating a state machine and calling emit_imagewriter_escape_sequences_3.
- `copy_table_to_buffer`: Copies 11 words from a global table to a buffer at address a4 plus an offset, then calls another routine twice with specific calculated arguments.
- `modify_value_on_match`: Compares a 16-bit value at the address in a2 to 27; if it is not 27, it branches.
- `match_escape_sequence`: Compares a word at the address in a2 to 27; if it is not equal, it branches to another location; otherwise, it tests a byte at the address in a3 and, if it is less than zero, writes the word 4 to the address in a2 and branches to another location.
- `update_word_on_match_2`: Compares a word at the address in a2 to 28; if it is equal and the byte at the address in a3 is greater than 0, it writes 33 to the word at the address in a2 and branches to another routine.
- `update_word_and_byte_on_match`: Compares a word at a pointer in a2 to 29; if it is equal and the byte at a pointer in a3 is greater than 0, it writes 33 to the word at a2 and 255 to the byte at a3, otherwise it returns.
- `update_word_if_byte_positive`: Compares a word at a pointer in a2 to 29; if it is equal and the byte at a pointer in a3 is greater than 0, it writes 2 to the word at the pointer in a2, then branches to another routine.
- `update_word_if_positive`: Compares a word at the address in a2 to 30; if it is equal and the byte at the address in a3 is greater than or equal to 0, it writes 32 to the word at the address in a2 and branches to another routine.
- `update_word_based_on_byte_2`: Compares a word at the address in a2 to 31; if it is not 31, it branches to another routine, otherwise it checks a byte at the address in a3.
- `compare_and_increment_byte`: Compares a word at the address in a2 to 32; if it is not 32, it branches to another routine, otherwise it increments a byte at the address in a3 and branches to another routine.
- `decrement_and_branch_2`: Compares a word at the address in a2 to 33; if it is not 33, it branches to offset 550, otherwise it decrements a byte at the address in a3 and branches to offset 802.
- `copy_byte_from_global`: Compares a word at a pointer in a2 to 64; if it matches, it loads a pointer from an offset of 28 in a6, checks if the byte at that pointer is non-zero, and if so, writes the byte from a global variable to that pointer before branching to another routine.
- `decrement_and_copy_data`: Decrements a byte at the address pointed to by an offset of 28 in a6, updates a global word with the new value, conditionally updates a byte at the next address, copies 11 bytes from a global variable to a buffer in a4 using a loop, and calls another routine twice.
- `copy_byte_to_address`: Increments a longword from an offset of 28 in a6, moves the byte at that new address into d1, and then writes d1 to the address pointed to by a4 before branching to another routine.
- `copy_and_call_threshold`: Copies 11 bytes from a global variable to a destination buffer in a4 using a loop, then calls another routine twice with different arguments derived from a structure pointed to by a6.
- `compare_word_and_branch_3`: Compares a word at the address in a2 to 57 and branches to a different location if they are not equal, otherwise it branches to another routine.
- `copy_table_and_call`: Copies 11 bytes from a global variable to a destination buffer in a4 using a loop, then calls another routine twice with arguments 3 and 7.
- `write_byte_on_null_check_alt`: Checks if a word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d2.
- `convert_value_to_ascii`: Writes a character to a destination at a pointer calculated from a parameter and a second parameter, then performs further writes to subsequent memory locations based on the value of the first parameter.
- `adjust_ascii_value`: Compares the byte in d0 to 99; if it is less than or equal to 99, it branches.
- `convert_integer_to_ascii`: Writes the byte 48 to a memory location, then either writes 48 to the next location if a byte is less than 10, or writes the result of dividing that byte by 10 and adding 48 to the next location, and finally calls another routine to write the remainder plus 48 to the following location.
- `convert_to_ascii`: Writes the byte 48 to a memory location, calls another routine, and then writes the returned byte incremented by 48 to the next memory location.
- `call_two_subroutines`: Calls a routine at an offset in a global variable with two parameters, then calls another routine at a different offset in the same global variable.
- `rotate_and_subtract_32bit_value_2`: Performs a 32-bit division and modulo operation using a 32-bit value and a 16-bit value derived from a global variable.
- `rotate_and_subtract_32bit_value`: Rotates a 32-bit value left by one bit for sixteen iterations, using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_5`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components and returns the result.
- `negate_result_on_flag_2`: Calls another routine and negates its return value if the resulting value in an address register is non-zero.
- `dispatch_jump_table_entry`: Loads a byte from the stack, compares it to five, and if it is less than five, uses it to calculate an offset into a table to jump to another routine; otherwise it jumps to a different location.
- `clear_registers_and_return`: Clears d0 and restores d2 through d4 and a2 through a4 from the stack before returning.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `initialize_local_variables`: Saves registers a2 and a3 to the stack, loads a word from the stack into d0, and sets a1, a2, and a3 to specific constants.
- `branch_on_d0_range`: Compares d0 to 256; if it is less than 256, it branches to a specific location, otherwise it branches to another location if d0 is greater than 256, otherwise it branches to a third location.
- `compare_and_branch_4`: Compares d0 to 512 and branches if not equal, otherwise it branches to another location.
- `compare_and_branch`: Compares d0 to 1024 and branches if they are not equal.
- `initialize_structure_fields`: Writes several 16-bit and 32-bit values into a structure pointed to by a1 and a pointer within that structure, and sets a global variable to 48.
- `initialize_structure_values_3`: Writes the constant 800 to offset 26 of a structure, the constant 1060 to offset 30, sets a 16-bit value of 100 at offset 36 and offset 34, and writes 49 to offset 45 of the structure pointed to by a1, then branches to another routine.
- `initialize_structure_values_variant`: Writes the constant 1168 to offset 26 of a structure, the constant 1590 to offset 30, sets a 16-bit value of 150 at offset 36 and offset 34, and writes 53 to offset 45 of the structure pointed to by a1, then writes 53 to the address in a2 and branches to another routine.
- `initialize_structure_values`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 33, and writes 30 to two memory locations.

### Okidata_292

- `return_constant_34`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_functions`: Calls three different library functions using pointers retrieved from a global variable and an offset, returning zero if all calls succeed or a negative value otherwise.
- `execute_dual_subroutines`: Loads a pointer from a global variable into a6, calls another routine using an offset from a6, restores a6, loads a second pointer from a global variable into a1, and calls another routine using an offset from a6 before branching.
- `execute_library_calls_2`: Calls a function through a pointer located at a specific offset from a global variable three times and returns zero.
- `return_constant_35`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_36`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_6`: Calls a function through a pointer in a global variable and, if the result is zero, populates a structure at a global address with two parameters and a constant, then executes three routines through a global base.
- `manage_printer_buffer_data`: Implements a multi-state state machine to manage a printer buffer by calculating offsets, clearing memory, and writing formatted data sequences including control characters and values from a global array.
- `clear_d0_and_restore_registers_2`: Clears register d0 and restores several data and address registers from the stack before returning.
- `initialize_printer_buffer`: Saves registers to the stack, loads a pointer from the stack into a2, and if the word at a2 is 1, it branches to another routine; otherwise it branches to a different location.
- `format_printer_command_buffer_2`: Copies a sequence of bytes from a table into a destination buffer, prepending specific control bytes based on values read from a structure pointed to by a register.
- `update_memory_byte`: Compares a word at an offset of a pointer in a0 to 32000, and if equal, writes the byte 15 to a memory location calculated from d2, d3, and a1 before branching.
- `process_printer_command_stream_2`: Copies a null-terminated string from a source buffer to a destination buffer, then processes a command byte by updating a state machine, writing specific byte sequences to the destination, or modifying values within a structure pointed to by a register.
- `loop_copy_table_bytes`: Loops while d0 is greater than d6, writing a byte from a table at offset 0 into a memory location calculated from d2 and d3, and incrementing d3 and d6 each iteration.
- `update_conditional_word`: Compares a word at the address in a2 to 66; if it matches, it writes 1 to a byte at a specific location, loads a pointer from d1 into a1, loads a pointer from that address into a0, compares a word at offset 86 of the address in a0 to 2048, and if it matches, writes 96 to a byte at the address in a0.
- `check_and_write_byte`: Loads a pointer from a location pointed to by a register, compares a word at offset 536 of that pointer to 1024, and if they match, writes the byte 137 to the address in d5 before branching.
- `write_byte_to_pointer_2`: Moves the address in d5 to a0, writes the byte 80 to the address in a0, and branches to another routine.
- `write_bytes_from_table_to_memory`: Writes the bytes 27 and 114 to a memory location calculated from two registers, then writes a byte from a global table to the subsequent memory address.
- `update_conditional_values_2`: Compares a word at the address in a2 with 32; if it is not equal, it checks if the byte at the address in a3 is non-zero, and if it is zero, it writes 1 to the address in a3 and 29 to the address in a2, then branches to another routine.
- `clear_byte_conditionally`: If the byte at the address in a3 is not positive, it clears that byte and writes the word 30 to the address in a2, then branches.
- `branch_with_constant_2`: Loads the constant negative one into a data register and branches to another location.
- `update_conditional_values`: Compares a word at the address in a2 with 33; if it is not equal, it checks if the byte at the address in a3 is not positive, and if so, writes 255 to the address in a3 and 29 to the address in a2, otherwise it branches to another routine.
- `clear_byte_if_not_positive`: If the byte at the address in a3 is not less than or equal to zero, it clears that byte and writes the word 28 to the address in a2, then branches.
- `branch_to_previous_routine`: Performs a short branch to a previous location.
- `update_status_and_memory`: Writes various byte values to stack locations based on the value at the address in a2, and if a2 contains 2, writes three bytes to a memory location calculated from d2 and d3.
- `check_and_update_memory`: Compares a word at the address in a2 to twelve; if not equal, it branches to another routine.
- `write_three_bytes_to_memory`: Writes the bytes 27 and 114 to the memory locations at the sum of two registers, then writes a byte from a global table to the following memory location.
- `write_status_byte_2`: If the word at the address in a2 is zero, it writes the byte 253 to offset 2720 of the structure pointed to by d1.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Okimate_20

- `return_constant_37`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_calls_2`: Stores a parameter and the address of a global variable into a table, then performs up to three nested calls using addresses retrieved from a global variable and its offset.
- `execute_dual_callbacks`: Loads a pointer from a global variable, calls a routine at an offset from that pointer, restores a6 from the stack, loads another pointer from a global variable, calls a routine at an offset from that pointer, restores a6 from the stack, sets d0 to 255, and branches to another routine.
- `update_handler_pointer`: Calls a function through a pointer at a specific offset from a global variable three times, then sets a global variable to the address of this routine regardless of a condition checked at a global offset.
- `store_routine_address_2`: Loads a longword from a memory location into a0, compares the word at offset 100 of the structure in a0 with 2, and if it is not equal, loads a longword into a0 and stores it at offset 54 of a global variable before branching to another routine.
- `store_routine_address`: Stores the address of this routine into a global variable at offset 54 and returns zero.
- `return_constant_38`: Returns the constant an address in d0 to the caller without any other effect.
- `format_printer_command_buffer`: Constructs a command sequence in a buffer by calculating offsets and writing specific byte values, including a loop that repeats three times to insert a series of control characters and data bytes.
- `clear_registers_and_return_2`: Clears d0 and restores registers from the stack before returning.
- `manage_printer_buffer_state`: Implements a state machine that handles buffer initialization, data insertion, and character output.
- `clear_d0_and_restore_registers_3`: Sets register d0 to zero and restores several data and address registers from the stack before returning.
- `divide_32_bit_integers_2`: Performs 32-bit division of a value in d1 by a value in d2, using a loop to process bits and returning the quotient and remainder.
- `rotate_and_threshold_32bit_value`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_6`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products and returns the result.
- `forward_call_to_string_length`: Moves a pointer from register d1 to d2 and a value from register d0 to d1, calls measure_string_length, and returns the result in register d2.
- `initialize_stack_and_compare`: Saves registers to the stack, loads several pointers from the stack into a0, a3, and a2, clears d2, d5, and a4, sets d3 to 26 and d4 to 16, and branches to another routine or a different location based on a comparison of a word at the address in a0 with 1.
- `build_complex_command`: Copies 15 bytes from a global variable to a destination buffer in a3 using a loop, then writes a zero byte to the buffer at offset 11 and offset 15, and performs several conditional writes to the buffer based on values in a structure pointed to by a4.
- `build_command_buffer`: Writes a sequence of bytes to a buffer in a3 based on three conditional checks involving values at offsets 2646, 2650, and 2660 of a structure pointed to by a4, including a loop that writes the byte 13 multiple times.
- `fill_memory_with_thirteen`: Fills a memory range starting at an offset from a pointer with the value thirteen in a loop, using a counter that compares against a byte value stored at an offset from another pointer.
- `update_printer_command_state`: Processes command codes to update a state byte and a command word, or generates specific byte sequences for printer control.
- `fill_memory_with_thirteen_2`: Fills a memory range with the value thirteen, starting at an offset from a pointer in a register, until a counter reaches a limit determined by a byte value stored at an offset from another pointer.
- `process_command_state`: Checks a word at a0 against several constants to update a byte at a2 and a word at a0, or modifies a value in a stack-based variable and writes a sequence of bytes to a memory location based on a loop that runs a number of times determined by a value in a global variable.
- `fill_memory_with_constant`: Writes the value thirteen to a memory location at an offset determined by a register and increments a counter until it reaches a value loaded from a pointer, then returns.
- `update_command_values`: Compares a word at the address in a0 to 32, and if they match, checks if the byte at the address in a2 is zero; if it is zero, it writes the byte 1 to the address in a2 and the word 27 to the address in a0 before branching.
- `clear_and_set_word`: Tests a byte at the address in a2; if it is not greater than or equal to zero, it clears that byte.
- `set_register_and_branch`: Sets register d0 to negative one and performs a short branch to a different location.
- `map_command_values_2`: Compares a word at the address in a0 with 33; if it is not equal, it checks if the byte at the address in a2 is non-zero, and if it is zero, it writes 255 to the address in a2 and 29 to the address in a0, then branches to another routine.
- `clear_byte_on_positive`: Checks if the byte at the address in a2 is greater than zero; if it is, it clears the byte at the address in a2, writes 28 to the address in a0, and branches to another routine.
- `return_negative_one_3`: Sets register d0 to negative one and branches to another routine.
- `map_command_values`: Compares a byte at a pointer against several constants and writes specific byte or word values to a destination based on the match.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_Advantage_D25

- `return_constant_39`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_callback_sequence`: Loads a pointer from a global variable and calls a function at a specific offset within that pointer's structure, repeating this process up to three times using different base pointers if the previous calls return non-zero.
- `execute_double_call`: Loads two addresses from a table and calls each one via a subroutine call, then branches.
- `call_library_function_thrice`: Calls a library function three times using a pointer from a global variable, passing the value in a6 to the second and third calls.
- `return_constant_40`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_41`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_42`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_command_parameters_2`: Saves registers to the stack, loads three pointers from the stack, clears registers d2, d3, and d1, and checks if the word at the first pointer is equal to one; otherwise it returns.
- `parse_control_sequence`: Copies a sequence of bytes from a source buffer into a destination buffer, applying specific byte transformations based on values read from a structure.
- `process_buffer_and_logic`: Copies bytes from one location to another in a loop, writes two longwords from a pointer in d1 to the stack, sets a word at a pointer in a0 to 65, and if that word is 65, copies 11 bytes from a global variable to a buffer in a4 using a loop and returns.
- `copy_byte_from_table_3`: Copies a byte from a table at a specific index into a destination address and loops until a counter reaches a limit.
- `update_threshold_status`: Compares a 16-bit word at a pointer in a0 to 27; if it matches and a byte at a pointer in a1 is less than zero, it writes 32 to the word at a0 and 1 to the byte at a1, otherwise it returns.
- `update_word_on_negative_byte_2`: Compares a word at the address in a0 to 27 and branches to another routine if not equal, otherwise it checks if the byte at the address in a1 is less than zero and, if so, writes 4 to the word at the address in a0 and branches to another routine.
- `compare_and_update_word`: Compares a word at a pointer in a0 to 28; if it is not equal, it branches to another routine, otherwise it tests a byte at a pointer in a1 and, if that byte is greater than zero, writes 32 to the word at the pointer in a0 before branching to another routine.
- `modify_threshold_values`: Compares a word at a pointer in a0 to 29; if it matches and the byte at a pointer in a1 is not zero, it writes 33 to the word at a0 and writes 255 to the byte at a1, then branches to another routine.
- `compare_and_set_word`: Compares a word at a pointer in a0 to 29; if it is not equal, it branches to another routine, otherwise it tests a byte at a pointer in a1 and, if that byte is greater than zero, writes 2 to the word at the pointer in a0 before branching to another routine.
- `update_word_on_negative_byte`: Compares a word at a0 to 30; if it matches, it checks the byte at a1 and, if that byte is negative, writes 32 to the word at a0.
- `modify_word_by_byte_comparison`: Compares a word at the address in a0 with 31; if it is not equal, it checks if the byte at the address in a1 is greater than zero, and if so, writes 33 to the address in a0, otherwise it checks if the byte at the address in a1 is less than zero and writes 32 to the address in a0, then clears the byte at the address in a1 and branches to another routine.
- `increment_on_match`: Compares a word at the address in a0 to 32, and if they match, increments the byte at the address in a1 and branches.
- `decrement_byte_on_match`: Decrements a byte at the address in a1 if the 16-bit word at the address in a0 is equal to 33.
- `write_status_byte_if_zero`: Checks if a 16-bit word at the address in a0 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d1.

### Alphacom_Alphapro_101

- `return_constant_43`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_callback_chain`: Loads a pointer from a global variable and calls a function at a specific offset within the structure pointed to by that value.
- `execute_dual_callback_sequence`: Loads a pointer from a global variable and calls a function at a specific offset within that structure, then repeats this process once more with a second pointer from a different global variable before returning.
- `execute_triple_function_pointer`: Fetches a function pointer from an offset within a structure pointed to by a global variable and calls that function three consecutive times using the value from register a6 as an argument.
- `return_constant_44`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_45`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_46`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_stack_and_registers`: Saves several registers to the stack, loads three pointers from the stack, clears registers d1, d2, and d3, and compares a word at the first pointer to 1 before branching.
- `format_printer_command_buffer_3`: Copies twelve bytes from a table into a buffer, then appends a sequence of escape codes and spaces based on values read from a structure.
- `fill_buffer_with_sequences`: Fills a buffer with the value thirty-two in a loop until a counter reaches a value from a table, then writes a sequence of two bytes, continues filling with thirty-two, and finally writes a sequence of six bytes.
- `fill_buffer_with_sequence`: Fills a buffer with the byte 32 in a loop and then writes a fixed sequence of 6 bytes at the end of the filled area.
- `update_status_on_match`: Compares a word at the address in a1 with 27; if it is not equal, it checks if the byte at the address in a2 is not zero, and if so, writes 32 to the address in a1 and 1 to the address in a2, otherwise it branches to another routine.
- `update_status_word_2`: Compares a word at the address in a1 with 27; if it is not equal, it checks if the byte at the address in a2 is not zero, and if so, writes 4 to the address in a1 and branches.
- `update_word_on_positive_byte`: Compares a word at the address in a1 to 28 and branches to another routine if not equal, otherwise it checks if the byte at the address in a2 is greater than zero and, if so, writes 33 to the word at the address in a1 and branches to another routine.
- `update_value_on_condition_2`: Compares a word at the address in a1 to twenty-nine; if not equal, it branches to another routine.
- `update_value_on_condition`: Writes the word 2 to the address in a1 if the word at a1 is 29 and the byte at a2 is greater than zero.
- `update_word_on_negative_byte_alt`: Compares a word at the address in a1 to 30 and branches to another routine if not equal, otherwise it checks if the byte at the address in a2 is less than zero and, if so, writes 32 to the word at the address in a1 and branches to another routine.
- `modify_word_and_byte_3`: Compares a word at the address in a1 to 31; if it is equal and the byte at the address in a2 is greater than 0, it writes 33 to the word at the address in a1.
- `increment_byte_on_match_2`: Compares a word at the address in a1 to 32; if it is not 32, it branches to another routine, otherwise it increments a byte at the address in a2 and branches to another routine.
- `compare_and_decrement_byte`: Compares a word at the address in a1 to 33; if it is not 33, it branches to another routine, otherwise it decrements a byte at the address in a2 and branches to another routine.
- `format_alphacom_printer_buffer`: Checks a word at a1 to determine a printer command sequence; it either writes a two-byte sequence based on a stack value or fills a buffer with specific byte patterns and escape sequences using two loops, otherwise it writes a byte to a global variable offset.
- `write_constant_byte_3`: Writes the byte 66 to a destination address calculated from a4 and d1, then branches.
- `fill_buffer_with_sequences_2`: If the word at a1 is 66, it writes 13 to a buffer, fills 80 bytes with 8 in a loop, and writes a 5-byte sequence; otherwise, if the word at a1 is 0, it writes 253 to offset 2720 of a structure pointed to by a global variable.
- `write_status_byte_if_zero_2`: Checks if a 16-bit word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by the pointer in d2.

### Epson_JX-80

- `return_constant_47`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_function_call`: Calls a function pointer from a structure at a global variable three times, and if all calls return non-zero, it calls another function from the same structure twice before returning -1.
- `execute_dual_function_pointers_3`: Loads a pointer from a global variable into a1, then calls two different routines by retrieving their addresses from a table and using an offset.
- `execute_triple_call`: Calls a routine through a pointer at an offset from a global base three times, passing the value from a register to the second and third calls.
- `return_constant_48`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_49`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_7`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte at specific offsets, then executes three routines through a global base and one routine through a pointer retrieved from the structure.
- `format_epson_jx80_print_buffer`: Implements a multi-state printer driver routine that manages a print buffer by calculating line offsets, clearing memory, and encoding data into ESC/P command sequences including character bytes and control codes for an Epson JX-80 printer.
- `clear_d0_and_restore_registers_4`: Clears register d0 and restores registers d2 through d4 and a2 through a4 from the stack before returning.
- `setup_and_compare_word`: Saves registers to the stack, loads pointers from the stack into a2, d1, and a3, clears d3, d6, and d2, sets d4 to 6 and d5 to 3, and branches to another routine or a different location based on a comparison of a word at the address in a2 with 1.
- `format_printer_command_buffer_5`: Copies a sequence of bytes from a table into a buffer, then populates the buffer with printer control codes and data based on values read from a structure.
- `process_epson_jx_80_commands`: Parses command bytes from a structure to generate a sequence of control codes, including character data and escape sequences, by writing values to a destination buffer and updating internal state flags.
- `copy_bytes_from_table`: Reads a byte from a location on the stack, decrements it, and writes it to the address in a1; then reads a byte from the next location on the stack and writes it to the address in a0, before branching to another routine.
- `copy_byte_from_table_4`: Copies a byte from a global table into a destination address calculated from a1 and d1, using a loop that repeats while d6 is less than 8.
- `encode_epson_jx_80_escape_sequences`: Translates a state byte and a control byte into specific ESC sequences for an Epson JX-80 printer.
- `load_pointer_and_check`: Loads a pointer from the address in a1 into a0, compares the word at offset 86 of the structure in a0 with 1024, and if equal, writes 137 to the address in a0 pointed to by d4 and branches.
- `write_byte_to_a0`: Writes 80 to the address in a0 and branches.
- `write_printer_escape_sequence`: Writes the byte 27 and 114 to a destination address, then copies up to 8 bytes from a table to that same destination using an incrementing pointer.
- `update_status_values_2`: If the word at a2 is 32, it returns; otherwise, if the byte at a3 is non-zero, it returns; if the byte at a3 is zero, it writes 1 to a3 and 27 to a2 before branching.
- `clear_status_byte_3`: If the byte at a3 is negative, it returns; otherwise, it clears the byte at a3, writes 30 to a2, and branches.
- `set_register_and_branch_2`: Sets d0 to 255 and branches.
- `handle_epson_command`: Compares a word at the address in a2 to 33; if it is not equal, it branches to another location; otherwise, it tests a byte at the address in a3 and, if it is non-zero, writes the byte 255 to the address in a3, writes the word 29 to the address in a2, and branches to another location.
- `clear_byte_and_set_word`: Tests the byte at the address in a3; if it is greater than 0, it clears the byte at the address in a3, writes 28 to the word at the address in a2, and branches to another routine.
- `branch_to_offset_470`: Performs a short relative branch to a location 470 bytes ahead in the code.
- `emit_epson_jx80_escape_sequence`: Checks a byte value and writes specific escape sequences to a destination buffer, including an escape character followed by a command byte and either a constant or a byte retrieved from a global table via an offset.
- `modify_stack_data`: If a value in a6 is 4, it writes two bytes and a byte from the stack to a destination; if a6 is 12, it checks a byte on the stack and either modifies it or writes two bytes and a value from a table to a destination; if a6 is 0, it writes a byte to an offset of a global variable; otherwise it returns.
- `check_and_modify_stack_values`: If the word at a2 is 12, it checks if a byte at an address from the stack is 39; if so, it writes 30 to that address; then it checks if the byte at an address from the stack is 37; if that byte is not less than or equal to 37, it branches.
- `write_three_byte_sequence`: Writes the byte 27 and 114 to a destination address, followed by a byte loaded from a global table using an offset from the stack.
- `write_status_byte_if_zero_3`: Checks if a 16-bit word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d2.
- `call_library_vector_198_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Epson

- `return_constant_50`: Returns the constant an address in d0 to the caller without any other effect.
- `verify_triple_function_call`: Calls a function pointer stored at an offset within a global variable three times via a structure, returning zero if all three calls succeed and negative one otherwise; if any call fails, it calls a cleanup routine at a specific offset within the global variable before returning.
- `execute_double_call_2`: Loads a pointer from a table, calls a routine at an offset from a second pointer, restores a6, loads another pointer from a table, calls a routine at an offset from that second pointer, restores a6, and branches to a location with a value of negative 1 in d0.
- `execute_indirect_function_calls_5`: Fetches a function pointer from a table using an offset and calls that function three times consecutively.
- `return_constant_51`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_52`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_8`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine through a pointer stored within that structure.
- `manage_printer_buffer_2`: Acts as a multi-purpose state machine for managing a printer buffer, handling data insertion, buffer clearing, and formatting escape sequences.
- `restore_registers_and_return_2`: Clears register d0 and restores several registers from the stack before returning.
- `setup_and_compare_word_alt`: Saves registers to the stack, loads pointers from the stack into a1, a4, and a3, clears d3, d5, and d1, sets d2 to 6 and d4 to 3, and branches to another routine or a different location based on a comparison of a word at the address in a1 with 1.
- `format_epson_printer_command`: Constructs a command buffer by copying data from a table and appending bytes based on values read from a structure.
- `process_printer_command_sequence_2`: Reads command bytes from a structure and performs conditional data writes to a buffer, including copying bytes from a table in a loop.
- `copy_bytes_from_stack`: Decrements a byte loaded from a pointer on the stack, stores it at the address in a1, and then copies a byte from a pointer at an offset of the stack pointer to the address in a0 before branching to another routine.
- `copy_table_byte`: Increments d3 and d5, calculates a destination address by adding a4 to the address in d3, and copies a byte from a table at offset d0 to that destination; it then loops if d0 is greater than d5.
- `process_command_state_2`: Compares a word at a pointer in a1 against several constants to update a byte at a pointer in a3, modifies a value in a stack-based variable, or writes a sequence of three bytes to a buffer in a4.
- `load_pointer_and_check_alt`: Loads a pointer from the address in a1 into a0, compares the word at offset 86 of the structure in a0 with 1024, and if equal, writes 137 to the address in a0 pointed to by d2 and branches.
- `write_byte_to_a0_alt`: Writes 80 to the address in a0 pointed to by d2 and branches.
- `write_epson_escape_sequence_byte`: Reads a byte from a table using an incremented index and writes it to a destination buffer at an offset determined by a pointer, then performs a conditional branch based on a comparison with eight.
- `check_byte_and_set`: Compares a word at the address in a1 with 32; if it is not equal, it branches, otherwise it checks if the byte at the address in a3 is non-zero, and if it is zero, writes 1 to the address in a3, writes 27 to the address in a1, and branches.
- `clear_byte_on_negative`: Checks if the byte at the address in a3 is negative; if so, it clears the byte at that address, writes 30 to the address in a1, and branches.
- `load_byte_and_branch`: Loads the value negative one into register d0 and performs a short branch to another routine.
- `handle_epson_command_alt`: Compares a word at the address in a1 to 33; if it is not equal, it branches to another location; otherwise, it tests a byte at the address in a3 and, if it is non-zero, writes the byte 255 to the address in a3, writes the word 29 to the address in a1, and branches to another location.
- `clear_byte_and_set_word_alt`: Tests the byte at the address in a3; if it is greater than 0, it clears the byte at the address in a3, writes 28 to the word at the address in a1, and branches to another routine.
- `branch_to_another_routine`: Performs a relative branch to another routine.
- `handle_data_logic`: Performs conditional writes to memory or the stack based on the value pointed to by a1, including writing specific byte sequences or values from a global table.
- `write_byte_if_zero`: Checks if a word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d1.
- `call_library_vector_198_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### HP_LaserJet_PLUS

- `return_constant_53`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_function_pointers`: Stores a parameter in a global variable and then attempts to call three function pointers sequentially from a structure in a6, using the value from a global variable as a base, returning 0 if all calls succeed or negative one otherwise.
- `execute_triple_call_2_subroutine`: Loads a pointer from a global variable into a1, then calls a routine at an offset from a pointer in a6 twice using values from a stack-based save, finally setting d0 to negative one and branching.
- `execute_triple_function_pointer_2`: Fetches a function pointer from a global variable using an offset and calls that function three times in succession.
- `return_constant_54`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_9`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a constant value, then executes four routines through a global base and another pointer.
- `update_hp_laserjet_state`: Manages a state machine for HP ImageWriter II command sequences by updating status flags and control characters.
- `format_hp_laserjet_command_2`: Copies a sequence of bytes from a table into a buffer, modifies specific bytes based on values read from a structure, and calls another routine to emit the resulting data.
- `process_printer_control_codes`: Parses and translates control characters into escape sequences for an ImageWriter II printer by modifying a buffer and calling emit_imagewriter_escape_sequences_3.
- `copy_and_call_routines`: Copies 11 bytes from a global table into a destination buffer starting at an offset in a structure in a6, then calls two different routines using values derived from a structure in a6.
- `update_values_on_match`: Compares a 16-bit word at a2 to 27, checks if the byte at a3 is non-zero, and if both conditions are met, writes 32 to a2 and 1 to a3.
- `replace_escape_character`: Compares a word at the address in a2 to 27; if it matches and the byte at the address in a3 is less than 0, it writes the word 4 to the address in a2 and branches to another routine, otherwise it returns.
- `update_word_on_match_3`: Compares a word at a pointer in a2 to 28; if it is equal and a byte at a pointer in a3 is greater than 0, it writes 33 to the word at a2 and branches to another routine, otherwise it branches to another routine.
- `modify_word_and_byte_2`: Compares a word at a2 to 29; if it matches and the byte at a3 is non-zero, it writes 33 to the word at a2 and 255 to the byte at a3.
- `update_word_on_match`: Compares a word at a pointer in a2 to 29; if it is equal and a byte at a pointer in a3 is greater than zero, it writes the word 2 to the address in a2.
- `update_word_if_positive_2`: Compares a word at a pointer in a2 to 30; if it is equal and a byte at a pointer in a3 is greater than or equal to 0, it writes 32 to the word at a2 and branches to another routine, otherwise it branches to another routine.
- `clamp_word_and_clear_byte`: Compares a word at the address in a2 to 31; if it is 31, it checks a byte at a3 and either sets the word in a2 to 33 or sets it to 32 depending on whether the byte is greater than or equal to zero, then clears the byte at a3 if it is negative before branching to another routine.
- `increment_counter_on_match`: Compares a word at the address in a2 to 32; if it matches, it increments a byte at the address in a3, otherwise it branches to another routine.
- `decrement_counter_on_match`: Compares a word at the address in a2 to 33; if it matches, it decrements a byte at the address in a3 and branches to another routine, otherwise it returns.
- `check_and_update_printer_status`: Compares a word at the address in a2 to 64 and branches if not equal; otherwise, it checks a byte at the address pointed to by an offset of 28 in a6 and, if that byte is non-zero, stores the byte 67 into the address pointed to by that same offset in a6 before branching to another routine.
- `decrement_and_sync_buffer`: Decrements a byte at a pointer stored in a structure in a6, updates a global variable with the new value, and conditionally synchronizes a second byte at that pointer with a value in d4.
- `increment_and_update_byte`: Increments a value stored in a global variable by one, reads the byte at that new address, and writes it to a destination address stored in d4.
- `copy_data_and_call_routines`: Copies 11 bytes from a global table to a destination pointed to by a pointer in a4 plus an offset in d5, then calls two routines using a pointer from a structure in a6.
- `compare_and_branch_word`: Compares a word at the address in a2 with 57; if the values are not equal, it branches to a different location, otherwise it branches to another location.
- `copy_buffer_and_execute_callbacks`: Copies 11 bytes from a global variable to a destination buffer pointed to by a register using a loop, then calls another routine twice with different arguments.
- `write_byte_on_null_check`: Checks if a word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by the address in d2, then returns.
- `convert_to_ascii_recursive`: Performs a series of conditional checks on a byte to write ASCII digits to a destination buffer, then calls another routine and writes the result plus 48 to the next byte in the buffer.
- `convert_byte_to_ascii`: Compares d0 to 99; if d0 is greater than 99, it writes 49 to the address in a2, increments d3, adds d4 to a2, and then subtracts 100 from d0.
- `convert_to_ascii_sequence`: Writes the byte 48 to a memory location, then either writes 48 to the next location or writes the quotient of a byte divided by 10 plus 48, calls another routine, and writes the returned byte plus 48 to the following memory location.
- `store_ascii_digit_pair`: Writes the character zero to a memory location pointed to by two registers, calls another routine, and then writes the returned byte incremented by 48 to the subsequent memory location.
- `call_two_subroutines_2`: Pushes two parameters onto the stack, calls a subroutine located at an offset in a global variable, then calls a second subroutine located at a different offset in the same global variable before returning zero.
- `rotate_and_subtract_32bit`: Performs a 32-bit rotation and subtraction loop based on a value in d2.
- `rotate_and_threshold_32bit_value_2`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag, and subtracts a value from the high 16 bits if that portion meets or exceeds a threshold stored in a global variable.
- `multiply_32_by_32_unsigned`: Computes the 64-bit product of two 32-bit integers using 16-bit partial products and returns the result across two 32-bit registers.
- `negate_result_on_flag_3`: Calls another routine and negates the returned value if the value in a0 is nonzero.
- `dispatch_via_table_2`: Saves registers to the stack, loads a byte and a word from the stack, performs a comparison, and jumps to an address from a table based on the result.
- `restore_registers_and_return_3`: Clears d0 and restores d2 through d3 and a2 through a4 from the stack before returning.
- `call_library_vector_198_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `branch_on_word_value`: Restores registers from the stack, loads a word from the stack into d0, and branches to different locations based on whether d0 is less than 768 or greater than 768.
- `compare_d0_limit`: Compares d0 to 256 and branches to one location if it is less than or equal, branches to another if it is greater than, otherwise it branches to a third location.
- `compare_word_and_branch_2`: Compares a word in d0 to 512 and branches if not equal, otherwise it performs a branch to another routine.
- `compare_and_branch_2`: Compares a word in d0 to 1024 and branches to different locations based on the result.
- `initialize_structure_fields_2`: Writes 600 and 827 to specific offsets of a structure pointed to by a1, writes 75 to two different locations, and sets a global variable to 48.
- `set_structure_constants`: Writes 800 and 1060 to offsets of a structure pointed to by a1, writes 100 to offsets of that structure, and writes 49 to an offset of the structure.
- `set_structure_and_byte`: Writes 1168 and 1590 to offsets of a structure pointed to by a1, writes 150 to offsets of that structure, writes 49 to an offset of the structure, and writes 53 to a byte at a location in a2.
- `initialize_structure_values_2`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 51, and writes 48 to two memory locations.

### type

- `initialize_memory_segment_structures`: Iterates through a linked list of memory segments, allocating a new block of memory for each segment and populating a structure with metadata including size, type, and pointers to subsequent segments.
- `process_amigados_hunk_data`: Iterates through a structure of hunk type codes and sizes to initialize memory segments by comparing types against a table.
- `store_and_call_routines`: Stores a value in a global variable, calls a routine using a pointer from a global variable with a constant argument, and then calls another routine via a pointer from a global variable.
- `conditional_xor_decrypt_block`: The routine checks a flag stored in a global variable, clears the flag, and if set XORs a block of words starting at a global variable with a constant, repeating a fixed number of times.
- `toggle_memory_byte`: Saves registers to the stack, checks a byte at a memory location, and if that byte is not zero, branches; otherwise it writes 1 to that location and branches.
- `perform_logical_or_sequence`: Performs a sequence of nine bitwise OR operations on register d0 using various immediate byte values.

### 12

- `return_constant_55`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_data_to_memory`: Copies a block of data from a source address pointed to by a register to a destination address pointed to by another register, using a byte-by-byte loop that decrements a counter until it reaches zero.

### 15

- `return_constant_56`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_57`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_font_data_to_memory`: Copies a block of data from a source address pointed to by a register to a destination address in memory, incrementing both pointers and decrementing a counter until the count reaches zero.

### 12

- `return_constant_58`: Returns the constant an address in d0 to the caller without any other effect.
- `process_font_glyph_data`: Reads a sequence of bytes from a source buffer pointed to by a register and writes them to a destination memory location, incrementing both pointers and a counter until a specific termination condition is met.

### 20

- `return_constant_59`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_5`: The routine returns zero immediately.

### 11

- `return_constant_60`: Returns the constant an address in d0 to the caller without any other effect.

### Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX][m2].adf:CLI

- `parse_command_line_string`: Parses a command line string by iterating through characters and handling quoted substrings or space-delimited tokens, while also performing conditional cleanup of structures and calling other routines based on the results of a function pointer call.
- `invoke_two_function_pointers_3`: loads an address from an offset of 92 from a base register into a0, calls the function at a negative offset of 384 from that base register, loads the same address into a0 again, calls the function at a negative offset of 372 from the base register, and then returns.
- `call_dos_function`: loads an address into a1 from an offset of 510 from the program counter, clears d0, calls a function at a negative offset of 552 from a base register, stores the result in a global variable, and returns if d0 is not zero.
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
- `allocate_bptr_entry_chain`: walks a linked list, allocating a cleared memory block for each entry, recording a value taken from the entry in the block, and linking the blocks together.
- `free_bptr_entry_chain`: walks the chain built by the allocation routine, releasing each entry value through another routine and then freeing the 8-byte entry block.
- `call_alloc_mem_3`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem_3`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_find_task_3`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.

### mathtrans.library

- `execute_illegal_instruction`: Triggers an illegal instruction exception by executing an undefined opcode.
- `store_address_to_global`: as library code, stores the address in a0 into a global variable at offset 408.
- `update_library_status`: If a byte at offset 8 of a structure in a6 is zero, it moves the value from a6 to a pointer in a6 plus 4, stores that pointer at the location pointed to by the value in a6, and calls a routine through a pointer in a global variable; otherwise, it sets bit 3 of a byte at offset 14 of the structure in a6 and returns zero.
- `set_library_status_bit`: as library code, sets bit 3 of a byte at offset 14 from a base register and returns zero.
- `increment_and_call_if_not_set`: Checks if bit 3 of a byte at offset 14 of a structure in a6 is clear; if so, it increments a 16-bit value at offset 32 of that structure, and if that value becomes 1, it stores a global variable and calls a library function via a pointer in a global variable, otherwise it returns -1.
- `return_negative_one`: Sets the data register 0 to a value of negative one and returns immediately.
- `decrement_counter_and_call`: Library code that decrements the 16-bit counter at offset 32 of the structure at A6.
- `mask_and_call_library`: As library code, it masks the lowest 7 bits of d0, calls another routine, sets the carry flag, and returns.
- `calculate_bitfield_difference`: If the byte in d0 is zero, it returns; if it is negative, it calls itself; otherwise, it performs a loop that modifies a running difference using a doubling accumulator and bitwise operations, updating a bitmask in uVar1 based on a value in uVar2 until a condition is met.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `process_and_mask_d0`: Moves d0 to d7, tests d7, calls a routine through a pointer in d7, sets bit 1 of d0, clears bit 3 of d0, and branches to another routine.
- `call_jump_table_entries`: Calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_d1_register`: As library code, it clears the long register d1 and then branches to another routine.
- `execute_conditional_library_calls`: Library code that calls a routine through a pointer at an offset from a register, performs several conditional calls to other function pointers, and returns a value with bits 0 and 1 modified.
- `execute_library_conditional_calls`: As library code, it calls a function pointer at an offset from a5, then performs a series of conditional calls to other function pointers at various offsets from a5 based on the values in d6 and d7.
- `execute_masked_calls_2`: Library code that performs a series of calls through pointers at various offsets from a base address in A5, depending on the values in D6, D4, and D7, and returns a value with the lower three bits cleared.
- `execute_masked_calls`: As library code, it moves d7 to d1, calls a routine at an offset of minus 48 from a5, and if the result is not negative, performs a series of checks on d4 before calling two more routines at offsets of minus 72 and minus 66 from a5.
- `call_masked_routine`: Moves d2 to d0, calls a routine at an offset of minus 60 from a5, moves the result in d0 back to d2, and branches to another routine.
- `conditional_load_d2`: Moves d2 to d0, tests d4, and if d4 is not zero branches to another routine; otherwise, it loads a constant into d2 and branches to another routine.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `calculate_bit_shifted_value`: As library code, it performs a loop that calculates a value using bit-shifting and comparisons, potentially calling another routine if a counter reaches zero.
- `perform_bit_shifts`: Library code that performs a logical shift right on D7, sets D6 to zero, sets D5 to a constant, loads a constant into A0, and sets D1 and D2 to specific values before branching to a later instruction.
- `execute_loop_with_branching`: As library code, it executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_word_and_branch`: As library code, it pushes the word 65534 onto the stack and then branches to another location.
- `push_word_and_branch_2`: As library code, it pushes the word 65535 onto the stack and then branches to another location.
- `push_word_and_branch_3`: As library code, it pushes the word 1 onto the stack and then branches to another location.
- `check_byte_and_adjust_stack`: Library code compares a byte in d7 to 184, branches if higher, otherwise increments a7 by 2, tests d7, and returns.
- `transform_math_data`: As library code, it performs a series of comparisons and arithmetic operations on the byte in d7, then enters a loop that modifies a value using a global variable and a counter.
- `perform_math_transformations`: This library code performs complex arithmetic by iterating through a memory block and modifying two 32-bit values in d6 and d7 using bitwise shifts and additions based on values from a pointer in a0.
- `transform_data_via_bit_shifts`: Iterates through a memory block pointed to by a register, modifying two 32-bit values in d6 and d7 by adding or subtracting values derived from bitwise right shifts of those registers based on a count in d5.
- `perform_math_loop`: As library code, it performs a loop that modifies d6 and d7 using values from a pointer in a0 and bitwise shifts based on a value in d5.
- `execute_math_loop`: As library code, it performs a loop that modifies d5, d6, and d7 using values from a pointer in a0 and bitwise shifts based on a value in d2.
- `set_d7_and_branch`: Library code loads the constant an address into d7 and branches to another location.
- `call_wrapper_and_restore`: As library code, it moves d5 to d6, calls another routine, moves d6 to d7, tests d7, restores registers from the stack, and returns.
- `call_wrapper_and_branch`: Library code that pushes d5 to the stack, calls another routine, restores d6 from the stack, and branches to another routine.
- `compare_and_branch_d6`: Library code that moves D6 to D4, branches if D4 is negative, otherwise compares D6 to 255 and branches if it is higher, and if not it sets D6 to zero before returning.
- `shift_and_jump_library`: As library code, it performs an arithmetic shift right on d4, increments d4, and branches if d4 is zero, otherwise it jumps to a fixed address.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_callbacks_5`: Library code checks a byte, and if it is non-zero, performs a series of conditional calls to other routines based on carry and other flags.
- `rotate_and_branch`: Library code loads the constant an address into d7, performs a right rotate through carry on d7, and branches to another location.
- `set_d7_and_branch_alt`: Library code loads the constant an address into d7 and branches to another location.
- `test_stack_byte`: Library code pops a word from the stack into d6, tests the byte in d6, and if it is positive, sets d7 to 0 and branches to another location; otherwise it returns.
- `set_negative_flag_and_restore_registers`: Sets the negative flag in the condition code register and restores several registers from the stack before returning.
- `load_and_test_constant`: Loads a constant into a register, adjusts the stack pointer by an offset of 30, tests the value in the register, and returns.
- `process_complex_data_structure`: As library code, it checks if d7 is zero, otherwise it performs a series of conditional calls to other routines and manipulates values based on the result of a calculation involving d7.
- `recurse_with_stack_slot_2`: Library code that calls another routine passing two parameters and a pointer to a specific location on the stack.
- `loop_with_arithmetic_and_shifts`: As library code, it increments d2, shifts d3 and d4 right by the value in d2, and enters a loop that adds d4 to d5, adds d3 to d6, subtracts a value from d7 using a pointer in a0, and decrements d1 until it reaches zero.
- `subtract_table_from_accumulator`: The routine walks through a table pointed to by a pointer and repeatedly adds the current table entry to an accumulator, then subtracts subsequent entries while a counter is decremented.
- `test_and_branch_on_d7`: As library code, it tests the byte in d7, branching if it is zero or negative, otherwise it masks d7 with 127, calls another routine, sets bit 1 of the status register, and returns.
- `branch_with_fill`: Library code that loads the value negative 255 into register d7 and then performs a short branch.
- `perform_complex_arithmetic`: Library code that executes a loop involving bit-shifting and conditional jumps, calling other routines and performing arithmetic on values derived from a register and a global variable.
- `complex_loop_with_branching`: As library code, it executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `process_signed_byte`: As library code, it tests the sign of a byte in d7; if it is negative, it branches to another routine, otherwise it masks d7 with 127, calls another routine, and sets bit 1 of the status register before returning.
- `execute_complex_library_logic`: Library code calls two routines, then performs a series of conditional calls and data manipulations involving bitwise operations and sign extensions based on the value in d7.
- `transform_d7_register`: Adds the value in d7 to itself, then conditionally performs a series of bitwise operations including an XOR, a bit shift, a subtraction, a word swap, and a left rotation before returning.
- `transform_d7_value`: Library code that swaps d7 with a memory location, performs a series of bitwise rotations, XOR operations, and additions on d7, and branches based on the overflow flag.
- `test_and_modify_d7`: Library code that compares d7 to 124 and 126, performs an addition and a rotate left on d7 if it matches 126, and branches based on the result.
- `test_and_branch_on_word`: Performs a bitwise AND on a word in d7 with a mask, then branches to another routine if the result is zero, otherwise it clears d7 and branches to another routine.
- `test_and_shift_d7`: Compares d7 to 254 and, if equal, performs two logical shifts on d7, then branches based on the result of the second shift.
- `set_status_bits`: Shifts a word in d7 left by 8 bits, moves a constant into d7, and performs a rotate through carry on d7 before setting the 2nd bit of the condition code register and branching to another routine.
- `clear_d7_and_branch`: Sets d7 to zero and branches to another routine.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `load_address_into_a0`: Library code that loads a constant address into A0 and then branches to address zero.
- `load_address_pointer`: As library code, it loads a long address into a0 and then branches to another routine.
- `load_library_address_zero`: As library code, it loads the address of a long constant into a0 and then branches to the start of the routine.
- `load_zero_into_a0_2`: Library code that loads a constant value into register a0 and then performs a short branch.
- `load_address_and_branch_14`: As library code, it loads the address at offset 30 into a0 and then branches to the next instruction.
- `load_library_address`: As library code, it loads the address of a constant into a0 and branches to another routine.
- `load_address_and_branch_15`: Loads the address of a constant into a0 and branches to the next instruction.
- `store_register_via_pointer`: As library code, it invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `load_address_and_branch_12`: Loads the address of a data structure at offset 52 into a0 and performs an unconditional short branch.
- `load_zero_into_a0`: Library code that loads zero into A0 and then branches to address zero.
- `load_address_and_branch_13`: Loads a 32-bit constant into a register and performs an unconditional branch to the next instruction.
- `load_address_and_branch_11`: Loads the address of a constant into a register and then performs an unconditional branch to the next instruction.
- `load_null_pointer`: Loads a null pointer into address register 0 and then branches to the next instruction.
- `load_address_and_branch_10`: As library code, it loads a longword into a0 and then performs an unconditional branch.
- `load_address_and_branch`: Loads a constant into a0 and branches to another routine.
- `load_address_and_branch_8`: Loads a constant address into a0 and then performs a short branch to the next instruction.
- `load_address_and_branch_9`: Loads an address from a constant offset into a0 and then performs an unconditional branch to the next instruction.
- `call_indirect_routine_via_offset`: Loads a pointer from a global variable, adds a negative offset of 30 to it, and calls the resulting address.
- `load_indirect_handler_address`: Loads the address of a global variable into a register and branches to another routine that calls a function at an offset from that address.
- `load_address_and_branch_2`: Loads the address stored in a global variable into a0 and performs a short branch to another routine.
- `load_address_and_branch_3`: Loads a longword from a specific memory location into a register and performs a short branch to another routine.
- `load_address_ca`: Library code loads a long value from a global variable into a0 and branches to another location.
- `load_indirect_call_address`: Loads the address of a global variable into a register and branches to another routine that performs an indirect call using an offset from that address.
- `load_address_and_branch_4`: Loads a constant value into a register and performs a short branch to another routine.
- `load_address_and_branch_5`: Loads a longword from a global variable into a0 and branches to another routine.
- `load_address_and_branch_6`: Loads a longword into a register from a fixed memory location and performs a short branch to another routine.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_constant_61`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_status`: If a 16-bit value at offset 8 of the structure in a6 is zero, it calls another routine using a pointer from offset 22, then updates a pointer in a structure using values from a6; otherwise, it sets bit 3 of a byte at offset 14 of the structure in a6 and returns 0.
- `set_icon_bit`: Sets bit 3 of a byte at offset 14 of the structure pointed to by a6 and returns 0.
- `return_constant_62`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_conditional_callbacks_4`: Stores values from d0 and a0 into a structure, calls another routine using a pointer from a6, and if the result is zero, calls another routine; otherwise, it stores the values from a6 and the result into two global variables.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_logical_or`: Performs a bitwise logical OR of register d0 with zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `traverse_icon_chain`: Saves two registers onto the stack and calls another routine to manage an icon chain, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `manage_icon_chain_4`: Pushes two pointers onto the stack and calls another routine to manage an icon chain.
- `manage_icon_chain_3`: Saves the current stack pointer and a pointer from a register onto the stack, calls another routine to manage an icon chain, and then returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_other_routine_2`: As library code, this routine calls another routine and then returns.
- `call_parse_numeric_string`: Saves two registers to the stack, calls parse_numeric_string, restores the registers, and returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_register_d0`: Performs a logical OR operation on register d0 with zero.
- `populate_icon_data`: As library code, it calls validate_and_update_icon_data, then if the result is non-zero, it calls populate_icon_attribute_chain_2, copies 43 bytes from a stack buffer to a structure at a pointer in a1, and then performs several multi-byte transfers to various offsets of that structure before calling return_zero_and_unlock.
- `copy_icon_data`: Copies 43 bytes from a source in a6 to a destination in a2 using a loop, then performs several multi-byte transfers from a6 to a2 at specific offsets, and finally updates a pointer at offset 76 of the structure in a2 if the value at offset 76 of the structure in a2 is not zero.
- `return_zero_and_unlock`: This library routine sets register d0 to zero, unlocks the register a6, and returns.
- `copy_icon_data_to_stack`: Copies 43 bytes from a structure pointed to by the third argument into a local stack buffer, extracts several fields from the same structure into local stack slots, and calls another routine with the first two arguments and a pointer to a two-word stack header.
- `populate_icon_attribute_chain_2`: Populates integer fields at offsets 18, 22, 33, and 35 of a structure by calling other routines and validates the results.
- `populate_icon_attribute_chain`: Checks specific values in a structure and uses a series of function calls to populate integer fields at offsets 18, 22, 33, and 35.
- `process_icon_data_chain`: Library code updates integer values at offsets 18 and 22 of a structure, then enters a loop that calls routines to traverse and update a linked list of data.
- `process_data_chain`: Repeatedly calls two different routines in a loop, updating a pointer and a counter at an offset of -4 from a6, then performs conditional checks on a value at offset 68 of a structure to return a status.
- `clear_register_and_branch_2`: Clears register d6 and branches to another location.
- `unlock_and_return`: This library routine unlocks the register a6 and returns without setting a return value.
- `traverse_icon_hierarchy`: Recursively traverses a hierarchical icon structure by checking flags and offsets within the structure, updating various data fields and processing child elements, including lists of pointers and nested structures.
- `traverse_icon_structure_tree`: Recursively traverses a hierarchical icon structure by checking flags and offsets within a structure pointed to by a2.
- `process_icon_list_2`: Library code that iterates through various lists pointed to by offsets 32, 36, and 46 of a structure, calling a function pointer in a3 and updating a value at offset 26 of the structure.
- `process_icon_list`: Increments a pointer in a loop until a null terminator is reached, calls a function pointer in a3, and if that returns non-zero, iterates through a list of pointers at offset 54 of the structure in a2 to call another routine recursively, finally storing a value from a6 into offset 26 of the structure in a2.
- `process_icon_list_recursively`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a pointer or table.
- `iterate_icon_chain`: Calls two library functions at specific offsets and then calls a routine to restore registers d2 and d3 before returning.
- `restore_registers_and_return_6`: This library routine restores registers d2 and d3 from the stack and returns.
- `incremental_offset_recurse`: Calls another routine with the same first argument and the second argument increased by a fixed offset of 140, then returns.
- `validate_and_update_icon_data`: Calls a routine to obtain a pointer, then conditionally calls another routine to update a value at an offset of 14 from that pointer.
- `call_update_icon`: As library code, it pushes several registers onto the stack, calls another routine, and if the result in d0 is zero, it returns a value from a2; otherwise it returns zero.
- `return_zero_3`: This library routine sets register d0 to zero and returns.
- `update_icon_structure`: As library code, it decrements a 16-bit value through a pointer, and if the result is zero or the original value was less than one, it calls another routine and potentially decrements the value again.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from registers and the stack into consecutive slots of that table.
- `process_icon_chain`: As library code, it calls execute_conditional_logic_2, then if the result is non-zero, it calls iterate_icon_chain, then calls populate_icon_attribute_chain_2, and finally calls update_icon_structure or call_lib_func_and_return_zero depending on the results.
- `move_register_and_branch`: Moves the contents of register d6 into d0 and branches to another location.
- `call_lib_func_and_return_zero`: Invokes a function pointer retrieved from a global variable associated with library index -210.
- `incremental_offset_recurse_constant`: Calls another routine with the same first argument and the second argument increased by a constant of 78, then returns.
- `search_icon_list`: As library code, it calls another routine and then enters a loop that traverses a list of pointers.
- `return_zero_extended`: This library routine sets register d0 to zero and restores registers d2, d3, a2, and a3 from the stack before returning.
- `process_library_sequence_2`: Library code that calls another routine in a loop, performs conditional checks on the result, and updates a parameter based on whether the returned value is zero or non-zero.
- `process_library_sequence`: As library code, it enters a loop that calls another routine and performs conditional checks.
- `increment_and_store_d2`: Tests d4 and, if non-zero, increments d4 by 1 and stores the result in d2 before branching.
- `clear_register_and_branch_3`: Clears register d2 and branches to another location.
- `save_registers_and_return`: Clears register d0 and restores registers d2 through d6 from the stack before returning.
- `parse_numeric_string`: Library code that clears a byte at offset 30 of a structure, calls another routine, and then enters a loop to convert a string of characters into a decimal integer by multiplying a running total by ten and adding each digit.
- `prepare_stack_and_call`: Library code that increments a value in a2 by 5, pushes two pointers onto the stack, pushes a2 onto the stack, calls another routine, and if the result in d0 is non-zero, sets d5 to 1, increments a2 by 3, and branches to another location; otherwise it returns.
- `clear_register_d5`: As library code, it clears register d5 and branches to another location.
- `calculate_icon_offset`: As library code, it enters a loop that updates a value by multiplying a register by ten and adding a byte from a pointer.
- `compare_and_calculate_difference`: Loads a longword from the stack into d1, compares it against the constant 48, and depending on the comparison result, sets d0 to 48, 57, or the result of subtracting 48 from d1 before branching.
- `set_d0_to_ff`: Sets register d0 to a value of 255 and returns.
- `call_library_vector_198_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `traverse_nested_structures`: Library code that calls another routine and uses the result to conditionally call a second routine or perform a cleanup call.
- `execute_conditional_logic_2`: Calls a library function and, if the result is non-zero, writes 10 to a 16-bit location at an offset of 14 from the returned pointer, updates a parameter, and calls another routine.
- `restore_registers_and_return_4`: Clears d0 and restores d2, d3, and a2 from the stack before returning.
- `call_and_validate_result`: Calls another routine and, if the returned value does not match the fourth parameter, sets the return value to zero.
- `move_d1_to_d0`: Copies the longword from d1 to d0 and restores d2 and d3 from the stack before returning.
- `call_and_verify_parameter`: Calls another routine and, if the returned value is not equal to the fourth parameter, sets the return value to zero.
- `transfer_register_and_restore_stack`: Moves a longword from register d1 to d0 and restores registers d2 and d3 from the stack before returning.
- `call_library_function`: Calls a library function at an offset of minus 198 from a library base retrieved from a global variable.
- `rotate_and_threshold_32bit_value_3`: Rotates a 32-bit value left by one bit per iteration for sixteen iterations using the carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_7`: Computes the 32-bit product of two 32-bit values by summing the products of their 16-bit components.
- `invoke_table_entry_3`: Loads a library base from a global variable and calls a function pointer located at an offset of 522 bytes from that base.
- `store_byte_via_pointer`: Stores a byte from register d0 into the memory location pointed to by register a3 and increments the pointer by one before returning.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_and_decrement_pointer`: As library code, it compares a byte from a pointer in a0 with a value in d0, and if they are equal, it decrements the pointer by 1 and returns d0 with the new pointer value; otherwise it returns.
- `branch_to_offset_2`: Library code that clears register d0 and branches to a specific offset.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `skip_zero_byte`: Restores registers a0 and a1 from the stack, checks a byte at the address in a0, and if that byte is not zero, branches to another routine; otherwise it decrements a0 and branches to a different location.
- `compare_byte_at_offset_2`: Decrements a value from the stack, and if it is not negative, compares a byte from a pointer in a1 with a byte from a pointer in a0, returning 0 if they match and the byte in a1 is zero.
- `set_signed_byte`: Sets d0 to 1 if it is greater than zero, otherwise sets d0 to 255 and returns.
- `test_byte_and_decrement`: Tests a byte at the address in a0, and if it is non-zero, decrements d0 and branches based on whether the new d0 is greater than zero.
- `decrement_pointer_and_call`: Decrements the address in a0 by 1 and calls another routine.
- `copy_bytes_with_counter`: Decrements d0 and, if it is not less than zero, copies a byte from the address in a1 to the address in a0 and repeats if the byte was not zero; it then decrements d0 again and, if not less than or equal to zero, clears a byte at the address in a0 and branches to another routine.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `update_linked_list_node`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the value at that location by four, clears the following word, and stores the original pointer value into the word after that.
- `conditional_library_branching`: The library code checks a parameter and, if non-zero, calls another routine and potentially calls a third routine using a value from a pointer offset.
- `iterate_linked_list_nodes`: Enters a loop that checks a parameter against zero, calls a routine with a pointer from the parameter and a calculated value, and updates the parameter using a pointer at offset 12.
- `set_d0_and_branch`: Sets d0 to 1 and branches to another routine.
- `restore_registers_and_return_5`: Clears d0 and restores d2, d3, d4, d5, and a2 from the stack before returning.
- `execute_nested_logic`: Library code that performs a series of conditional calls to other routines, updates a value at an offset of 10 from a pointer, and returns either a pointer or negative one.
- `return_negative_one_2`: This library routine sets register d0 to negative one and returns.
- `process_linked_icon_data`: Library code that enters a loop to update a value at an offset of 12 from a pointer, calls other routines, and checks for zero results to return negative one or a calculated value.
- `process_linked_data`: Library code that enters a loop to update a value at an offset of 12 from a pointer, calls other routines, and checks for zero results to return negative one.
- `move_a3_to_d0_and_branch`: Moves the contents of a3 into d0 and branches to another routine.
- `return_negative_one_extended`: This library routine sets register d0 to negative one and restores several registers from the stack before returning.

### translator.library

- `execute_callback_with_data`: Checks if a function pointer at an offset from a global variable is non-zero, then if it is, stores a global variable and a value from a register into a structure and calls another routine through a pointer at an offset from that global variable.
- `clear_flag_and_increment_counter_2`: Increments a 16-bit counter in the structure it is given and clears bit 5 of that structure's flag byte.
- `decrement_counter_call_on_zero`: Decrements a 16-bit counter in the structure it is given.
- `check_and_set_bit`: Saves registers to the stack, moves a pointer from a6 to a3, loads a new address into a6 from the stack, tests a word at offset 32 of the structure in a3, and if it is not zero, sets bit 5 of the byte at offset 14 of the structure in a3 before branching.
- `unlink_node_and_call_library`: Removes the node from its doubly linked list, then calls a function through the Amiga library base and returns a value stored in the node.
- `return_constant_63`: Returns the constant an address in d0 to the caller without any other effect.
- `format_buffer_content`: Library code that initializes a stack buffer with space characters, then enters a loop to process bytes from a source, performing character substitutions and calling other routines based on specific byte patterns.
- `copy_word_and_branch`: As library code, it copies a 16-bit value from register d0 to d1 and then branches to another location.
- `clear_byte_and_return_word`: Calls one unnamed library routine, clears the byte just before the pointer held in the address register, and returns the low 16-bit value of the data register.
- `execute_translator_logic_2`: Library code that loads a pointer from a global variable, compares it to zero, and if it is not zero, loads the value at that pointer into a6, calls two routines, and branches based on the result of the second call; if the result is negative, it branches to another location, otherwise it branches to another location.
- `execute_translator_logic`: As library code, it loads two pointers from a global variable, calls two routines, and then branches based on the value in d7.
- `execute_special_character_checks`: As library code, it calls three different routines sequentially, using the results of these calls to determine whether to continue.
- `check_special_character`: As library code, it loads two pointers from the stack into a2 and a3, reads a byte from the address in a6, and branches to another routine if that byte is 92 or 96; otherwise it returns.
- `stub_immediate_return`: The decompiled body contains only a return, so the routine does no visible work in the library.
- `match_bracketed_tag_text`: Skips forward to an opening bracket character, then compares the characters that follow against a reference string until a closing bracket.
- `compare_table_values_2`: Decrements a 16-bit value in d6, uses the result as an offset to load a byte from a table in a6, and then uses a modified version of that byte as an offset to load a word from a table in a4 to check bit 3 of the result and compare the original byte with a byte at the pointer in a2.
- `set_processor_status`: Sets the condition code register to a specific value and returns.
- `dispatch_special_character_one`: As library code, it loads 1 into d7, calls another routine, and if the result is not zero, it branches to another routine; otherwise it clears the carry flag and returns.
- `match_string_and_process_special`: Compares a string at a pointer in a6 with a string at a parameter in a1, and if a character matches the '=' character, it returns; otherwise, it checks bit 6 of a word at an offset in a4 and calls another routine if the bit is set.
- `dispatch_special_character_two`: As library code, it loads 255 into d7, calls another routine, and if the result is not zero, it branches to another routine; otherwise it clears the carry flag and returns.
- `check_character_range`: Loads a byte from a table in a6 using an offset derived from d6, and if that byte is equal to 92 or 96, it returns; otherwise, it subtracts d6 from the offset and returns if the result is not zero.

The function list continues in: g1448-recovery-functions-2.md.
