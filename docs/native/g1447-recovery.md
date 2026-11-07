# Game 1447 (balance of power - the 1990 edition (1990)(mindscape)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01447). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf:stack | 2 | 117 | 2 |
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
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf:CLI | 9 | 362 | 18 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 15 | 1414 | 108 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2476 | 170 |
| mathieeedoubbas.library | 3 | 632 | 60 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 13 | 830 | 53 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf:bop | 3 | 5064 | 305 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1700 | 37 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 150 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf bytes 603136-603648 (code never loaded by a reference run) | 1 | 28 | 1 |
| SavedGame (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- balance of power - the 1990 edition (1990)(mindscape)[cr qtx]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf: file devs/printers/Okidata_92 (header 5476 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1478 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 93 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 20 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf: bytes 603136-603648 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf:asave/SavedGame: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf: 1 logged loads cover 512 bytes of the image
- balance of power - the 1990 edition (1990)(mindscape)[cr qtx]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf: blocks 112-120 (bytes 57344-61952) outside the filesystem hold data, not code (opcode density 0.025, entropy 4.98)
- 1 never-loaded region(s), 192665 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf:stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 to d0 and then branches to another location.

### Format

- `parse_command_string`: Parses a string into tokens by scanning for spaces, tabs, or quoted text, while performing character substitution for newline and escape sequences, and stores the resulting tokens in a table.
- `execute_conditional_callbacks`: Checks two global variables for non-zero values and, if non-zero, calls a function pointer at a specific offset from a global variable or calls two function pointers at specific offsets from another global variable.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `execute_conditional_callbacks_2`: Loads an address into a1, clears d0, calls a function via a library offset, stores the result in a global variable, and returns if the result is non-zero.
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
- `rotate_32bit_value_3`: Performs a 32-bit left rotation on a value for sixteen iterations and handles overflow logic based on a divisor.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_increment`: Stores a byte from register d0 into the memory location pointed to by a3 and increments a3.
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

- `parse_command_string_2`: Iterates through a string to extract tokens, handling quoted substrings and converting specific character sequences into newline or escape characters.
- `dispatch_library_calls`: Stores a parameter in a stack-based structure and calls one or two routines located at offsets within a global variable based on whether certain global values are non-zero.
- `invoke_two_library_function_pointers`: Loads a pointer from an offset of a4 into a0 and calls a function at a negative offset from a6, then repeats this process with a different negative offset from a6 before returning.
- `dispatch_library_calls_2`: Loads a value from an offset of the program counter into a1, clears d0, calls a routine at an offset from a6, stores the result in a global variable, and returns if the result is zero.
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
- `update_structure_fields_3`: Calls another routine with four parameters.
- `update_structure_fields_2`: Stores a 16-bit value at offset 28, a 32-bit value at offset 36, and a 32-bit value at offset 40 of a structure, calls another routine, and then adds the result of that call to a value at offset 44 of the structure.
- `synchronize_disk_contents`: Performs a disk-to-disk copy operation by iterating through file structures on a source disk and replicating them to a destination disk.
- `synchronize_disk_contents_2`: Performs a disk-to-disk copy operation by iterating through file structures on a source disk and replicating them to a destination disk.
- `execute_cleanup_sequence`: Checks a global variable.
- `cleanup_system_resources`: Checks several global pointers and executes various cleanup routines, including deleting port structures, deleting extended I/O, calling library functions to free memory or close libraries, and managing error message tasks.
- `extract_message_data`: Copies three values from a structure at the address in a global variable into other global variables, calls another routine with that structure, and then calls another routine with a global variable if a specific value is not 512.
- `execute_library_call`: Loads a value from the stack into d0, pushes a value from a global variable and d0 onto the stack, calls a routine at an offset from a global variable, and if the result is not zero, it branches.
- `perform_disk_copy_loop`: Iterates through a table of data blocks to perform disk I/O operations, including reading from a source and writing to a destination.
- `allocate_multiple_libraries`: Checks a value at an offset of 20 from a global variable; if it is less than 33, it calls a routine with a fixed address and then calls a routine with 0.
- `initialize_disk_copy_params`: Populates a stack-based structure with several constants and then calls two library functions using a global base.
- `report_disk_errors_2`: Checks a parameter to determine which error handling routine to call, sets a global pointer to a global variable, and calls various routines with specific constants or the global variable itself before returning zero.
- `report_disk_errors`: Sets a pointer to a global variable and calls another routine with a global value, a parameter, and a stack address, then repeats this with a different constant and calls another routine with a single constant before returning.
- `process_io_and_status`: Calls a routine and checks the result; if the result is zero, it calls another routine with a pointer from a structure and a value from a table.
- `check_signal_status`: Calls another routine with two parameters, checks if the 12th bit of the result is set, and if so, calls another routine with a global variable; otherwise, it returns the bit status.
- `process_message_queue`: If a global variable is non-zero, it enters a loop to process messages via another routine and then performs a series of calls and data extractions from a structure pointed to by a global variable, returning a value based on the contents of that structure.
- `set_parameter_zero`: Stores zero in a word at an offset of 36 from a pointer and nine in a word at an offset of 28 from that same pointer before calling another routine.
- `manage_error_message_task`: Allocates memory, populates the resulting structure with various values and pointers, calls a routine to handle the structure, and then frees the memory.
- `set_parameter_and_call`: Writes 4 to offset 28 of a structure, calls another routine, writes 5 to offset 28 of the same structure, and calls another routine twice more.
- `execute_multiple_calls`: Pushes two global longwords onto the stack and calls a routine twice for each value, then cleans up the stack and returns.
- `process_disk_data`: Calls another routine with a constant and a value, then if the result is non-zero, it performs several calls using a pointer from a global variable, updates a global variable with a pointer, modifies a byte at an offset of the pointer, calculates a sum by looping through 128 words in a table, subtracts that sum from a value at offset 5 of the pointer, and calls another routine with a constant and the pointer.
- `perform_io_operation`: Calls another routine with four parameters, then calls another routine and returns whether its result is zero.
- `compare_and_io`: Performs an I/O operation, calls another routine to update a structure, and returns whether a parameter matches a value stored in a global variable.
- `allocate_message_entry_2`: Attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port_structure`: Checks if a field at offset 10 of a structure is non-zero to trigger a recursive call, then sets a byte at offset 8 to 255, sets a 32-bit value at offset 20 to negative one, calls a routine to free a signal, and calls a routine to free the memory block.
- `initialize_io_request_structure`: If the first argument is non-zero, it calls a routine to allocate memory for an I/O request structure with a constant value of 65537, then populates the structure by storing the value 5 at offset 8, the second argument as a 16-bit integer at offset 18, and the first argument as a 32-bit integer at offset 14.
- `delete_ext_io_2`: If the provided pointer is not zero, it sets the byte at offset 8 to 255, sets the 32-bit words at offsets 20 and 24 to an address, and calls a routine to free the memory block using the 16-bit size stored at offset 18.
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
- `execute_indirect_calls_2`: Fetches a function pointer from a global structure and executes it multiple times with varying parameters.
- `invoke_indirect_callbacks`: Calls another routine with a pointer to a global variable and a parameter, then calls another routine with four parameters including a local stack array.
- `convert_char_to_uppercase`: Returns the character with lowercase ASCII letters converted to uppercase by subtracting 32.
- `compare_strings_case_insensitive_2`: Compares two null-terminated strings by converting characters to uppercase and returns 1 if they match, otherwise it returns 0.
- `terminate_colon_string`: Searches for the end of a null-terminated string and, if the last non-null character is a colon, replaces it with a null terminator.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer`: Stores a byte from register d0 at the memory location pointed to by a3 and increments a3.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper_2`: Moves the contents of two registers into a stack frame, calls a helper routine, and then restores the first register from the stack before returning.
- `call_modular_inverse_and_return_d1`: Moves the contents of register d0 into d1, calls modular_inverse_32bit, and returns the resulting value from register d1 back into register d0.
- `negate_modular_inverse`: Calls modular_inverse_32bit and negates the returned value if the helper leaves a non-zero value in an address register.
- `call_and_negate_on_flag_2`: Calls modular_inverse_32bit and returns its first result, or the negation of that result if the second result in an address register is nonzero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment_pointer`: Stores a byte from register d0 into the memory location pointed to by register a3 and then increments the pointer by one before returning.
- `call_formatted_print`: Pushes a pointer and a value onto the stack and calls a routine to print a formatted message, then returns the result in a register.
- `pass_parameters_to_string_length`: Pushes a global variable, a parameter, and a stack address onto the stack and calls a routine to measure the length of a string.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_global_library_function_2`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `execute_callback_with_status_check`: Calls a routine via a global pointer and returns either a byte from a stack slot or a negative error code if the called routine returns one.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.

### Port-Handler

- `parse_amiga_hunk_data`: Parses a structure containing AmigaDOS hunk type codes to determine memory allocation requirements, clears a range of memory using a loop, and performs conditional data copying and register updates based on the hunk type.

### Disk-Validator

- `update_display_buffer`: Converts a value from a global variable into a character string and writes it to a buffer, then enters a loop to clear a range of memory and performs bitwise operations to update a bitmask in a structure.

### Ram-Handler

- `execute_copy_mem_wrapper`: Sets d0 to 12, saves registers, calls a routine at an offset of -656 from a6 using a0 and a1 as arguments, restores registers, and jumps to the address in a6.

###            

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with sequential words.
- `move_and_branch_2`: Moves the contents of d1 into d0 and then performs a short branch to an offset of 104.

### serial.device

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reinit_serial_buffer_fields`: Calls a routine via a pointer in a6 if a1 is non-zero, then updates several fields in a structure at a6 with a value from d2, including setting half-size and quarter-size values.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `set_byte_and_branch`: Stores the byte 253 at offset 31 of a structure in a1 and then performs a short branch to offset 1174.
- `toggle_interrupt_enable`: Writes 801 to the INTENA register, calls another routine, writes 8801 to the INTENA register, and returns.
- `test_request_type`: Clears bit 7 of a byte at an offset in a1 and then compares a word at an offset in a1 against the values 2, 3, and 10, branching to different locations based on the result.
- `compare_and_clear_pointer`: Compares a longword at offset 74 of a structure in a6 with a1; if they are equal, it clears bit 1 of a byte at offset 454 of a6, clears the longword at offset 74 of a6, and branches.
- `unlink_list_node`: Tests bit 6 of a byte at offset 30 of a structure in a1; if set, it unlinks a structure by updating pointers and decrements a 16-bit counter at offset 70 of a structure in a6.
- `reset_serial_hardware`: Compares a1 to a longword at offset 78 of a structure in a6; if they match and a 16-bit word at offset 28 of a1 is 10, it clears bit 5 of a byte at offset 470 of a structure in a6, writes 2048 to a hardware register, calls another routine, clears the longword at offset 78 of a structure in a6, and clears bit 4 of a byte at offset 472 of a structure in a6.
- `unlink_second_queue_request`: If a bit is set in a byte at offset 30 of a structure in a1, it unlinks that structure from a list, decrements a counter in a6, and calls a routine via a pointer in a6 if a bit is not set.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `set_status_bit_4`: Clears d0 and tests a 16-bit word at offset 32 of a structure in a6; if non-zero, it sets bit 6 of a byte at offset 472 of a structure in a6, otherwise it returns.
- `invoke_library_vectors_then_unlink`: Calls several routines via pointers in a6 and a structure in a6, copies a longword from a structure in a6, and unlinks a block from a list by updating pointers.
- `manage_linked_list_node`: Sets a 32-bit value at offset 0 of a structure in a1 to the value of a6 plus 166, updates a linked list by inserting the structure in a1 between a pointer at offset 170 of a6 and the previous head, clears bit 1 of a byte at offset 30 of the structure in a1 if certain conditions are met, sets bit 6 of that same byte, and calls a routine via a pointer.
- `process_serial_data_buffer_2`: Iterates through a linked list of data structures, updating status bits and managing a circular buffer of bytes.
- `process_serial_buffer_stream`: Iterates through a circular buffer of data, decrementing a counter and updating a pointer to the next byte.
- `test_status_bit_3`: Tests bit 0 of a byte at an offset in a6 and branches to different locations depending on whether the bit is set.
- `update_status_byte_3`: Calls another routine, and if the result is zero, writes the byte 9 to offset 31 of a structure in a1 before branching.
- `process_serial_data_buffer`: Iterates through a linked list of data buffers to process bytes, updating status flags and counters within a structure pointed to by a6.
- `restore_registers_and_return_6`: Returns immediately after restoring d2 and a6 from the stack.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `clear_status_byte_5`: Clears bit 2 of a byte at offset 470 in a6, and if the bit is clear, writes the value 15 to offset 31 of a structure in a1 before branching to another routine; otherwise it branches to another routine.
- `clear_status_bit_2`: Clears bit 7 of a byte at an offset in a6 and, if the bit was already clear, writes the value 9 to an offset in a1; otherwise, it branches forward.
- `modify_status_and_clear_counter`: Writes the byte 12 to offset 31 of a structure in a1, tests bit 0 of a byte at offset 470 of a6, and if it is zero, writes the byte 6 to offset 31 of a1, clears a longword at offset 62 of a6, and performs a bitwise AND on a byte at offset 470 of a6.
- `process_serial_data_and_signal`: Checks status bits in a structure to manage serial data flow, updates a buffer with received bytes, and calls a handler routine via a pointer in the structure.
- `toggle_interrupt_state`: Compares a byte at offset 34 of the structure in a6 with a register, then checks bit 1 of a byte at offset 472 of the structure in a6; if bit 1 is set, it clears that bit and writes 32769 to the interrupt enable register, otherwise it returns.
- `test_status_bits_2`: Tests bit 5 of a byte at offset 472 of a structure in a6, then tests bit 1 of a byte at offset 445 of a structure in a6.
- `test_bit_and_branch_2`: Tests bit 8 of d0 and branches to another routine if the bit is set, otherwise it branches to another routine.
- `test_status_bit`: Tests bit 0 of a byte at offset 445 of a structure in a6 and branches to another routine.
- `set_pointer_if_zero`: Sets bit 7 of a byte at an offset in a6 and, if a longword at an offset in a6 is zero, copies a longword from one offset in a6 to another offset in a6 before branching.
- `update_circular_buffer_and_call_callback`: Checks if a counter at offset 58 of the structure in a6 is less than a limit at offset 54; if so, it writes a byte to a pointer at offset 42, increments counters at offsets 58 and 42, and updates the pointer if a threshold at offset 50 is reached.
- `record_pointer_then_signal`: Checks if a longword at offset 62 of a structure in a6 is zero; if it is, it copies a longword from offset 42 to offset 62, calls another routine, and then calls a library vector using a pointer from offset 98 of the structure in a6.
- `clear_hardware_register_bit`: Moves a byte from offset 34 of a structure in a6 to offset 297 of a6, clears bit 3 of a byte at offset 296 of a6, tests bit 2 of a byte at offset 69 of a6, and if it is zero, clears bit 6 of the hardware register at address bf d000.
- `update_hardware_registers`: Moves a byte from offset 35 of a structure in a6 to offset 473 of that same structure, sets bit 3 of offset 472 in a6, tests bit 2 of a byte at offset 69 in a6, and if zero branches to another routine; otherwise it sets bit 6 of a long value at a hardware register, tests bit 7 of the byte at offset 69 in a6, and if set branches to another routine; otherwise it writes the word 32769 to a hardware register and returns.
- `clear_status_byte_2`: Clears a byte at offset 473 of the structure in a6 and returns.
- `update_status_and_signal`: Checks if a byte at offset 470 of a structure in a6 has bit 6 set; if not, it sets that bit, clears a longword at offset 418, copies a longword from offset 106 to offset 418, decrements that value by 10, sets a word at offset 414 to 9, calls another routine, and if a byte at offset 47 of a structure in a1 is non-zero, sets a byte at offset 47 of that structure to 11.
- `check_interrupt_status`: Saves registers to the stack, loads a pointer from a1 into a6, clears bit 6 of a byte at offset 470 in a6, and if the bit is clear, reads a word from the custom chip register, masks it, and branches to another routine; otherwise it tests if the word is non-zero and, if so, branches to another routine, or clears bit 2 of a byte at offset 470 in a6 and branches to another routine.
- `load_address_and_call`: Loads the address at offset 450 of a structure in a6 into a0, calls another routine, and then branches to another routine.
- `set_status_bit_6`: Sets bit 0 of a byte at offset 470 in a6 and branches to another routine.
- `manage_serial_request_queue`: Clears a value at offset 8 of a structure, then if offset 9 is non-zero, modifies a status byte at offset 30 and either inserts the structure into a linked list at offset 78 of a second structure or updates a pointer and increments a counter at offset 72.
- `test_flag_and_update_interrupt_state`: Tests bit 2 of a byte at offset 69 in a6, calls another routine if set, then checks bit 31 of a byte at offset 31 in a1 to set bit 4 of a byte at offset 30 in a1 and update a structure in a6 before writing to the interrupt enable and request registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_request_state_on_flags`: Checks if two specific bits in a global variable are set, and if so, writes a byte with a value of thirteen to an offset within a structure pointed to by a1.
- `process_serial_request_queue`: Manages a linked list of pending serial requests by popping the head of the list, updating status flags, and decrementing a request counter.
- `update_serial_counter`: Writes the word 504 to offset 476 of a structure in a6, loads a pointer from offset 86 of a structure in a6 into a0, and clears d0 before loading a byte from the address in a0.
- `update_serial_status_and_call_handler_2`: Updates a status bitmask and increments two counters within a structure, then executes a function pointer via a table lookup if specific conditions are met.
- `update_serial_status_and_call_handler`: Updates a bitmask in a global variable, increments two counters in a structure, and executes a function pointer via a table lookup if certain conditions are met.
- `decrement_list_and_update_status`: If a shortword at offset 72 of a structure in a6 is non-zero, it manipulates a linked list starting at offset 176 of a6, decrements the shortword at offset 72, and updates several fields in a structure pointed to by a pointer from the list; otherwise, it sets a global variable to 1.
- `enable_interrupts`: Restores d2 and a6 from the stack, sets bit 0 of the interrupt enable register, and returns.
- `update_serial_interrupt_mask`: Enables the serial port interrupt, reads a byte and a word from a structure in a1, clears a byte at offset 473 of that structure, writes the combined value to the interrupt mask register, and otherwise returns.
- `reset_serial_device`: Writes 801 to the INTENA register, calls another routine twice if certain conditions in a6 are met, writes several longwords to a structure in a1, calls another routine twice, writes 8801 to the INTENA register, and calls another routine via a pointer if a bitmask test passes.
- `call_handler_and_branch`: Calls a routine via a pointer and then branches to another routine.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `signal_and_call_handler`: Checks if a longword at offset 78 of a structure in a6 is non-zero and sets bit 2 of a byte at offset 296 of a6 if it is, then checks if a longword at offset 74 of a6 is non-zero to call another routine, and finally calls a library vector if a specific bitmask is not met in a structure in a1.
- `clear_bit_and_call_handler`: Clears bit 2 of a byte at offset 472 of a structure in a6, loads a longword from offset 74 of a6 into d0, and if d0 is not zero, calls another routine and then, if a byte at offset 30 of the result is not set to 0 or 1, calls another routine after writing 32769 to the hardware register at offset 252.
- `toggle_interrupt_and_call`: Sets the interrupt enable register to a specific 16-bit value, calls a routine, sets the register to a different 16-bit value, and then branches to another routine.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `update_pointer_and_loop`: Copies the value from a0 to a1, then enters a loop where it writes the value from d0 to the address in a0, updates a pointer at offset 4 of a1, and either calls a routine or branches back to the start of the routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_serial_state_data`: Updates several fields in a structure in a1 using values from a structure in a6, including a byte at offset 81, a byte at offset 80, and a 32-bit value at offset 32, then calls a routine via a pointer if a specific bitmask check passes.
- `test_status_and_registers`: Clears bit 0 of a byte at offset 30 of a structure in a1, tests bit 3 of a byte at offset 79 of a structure in a1, and if bit 3 is not set, branches to another location; otherwise, it tests a word at offset 72 of a structure in a6 and branches if non-zero, or tests a longword at offset 78 of a6 and branches if non-zero.
- `handle_serial_event`: Moves a long from offset 106 of a structure in a6 to d0, calls another routine, tests a byte at offset 31 of a structure in a1 and branches to another routine if non-zero; otherwise it moves a1 to offset 78 of a6, sets bit 5 of offset 46 in a6, moves the address a fixed address into a0, writes the word 34816 to the address in a0, and branches to another routine.
- `reply_and_clear_active_request`: Pushes a6 to the stack, updates a6 using a pointer at offset 98, calls a routine via a pointer, restores a6 from the stack, clears a 32-bit value at offset 78 of the structure in a6, and returns.
- `clear_uart_break_and_reply`: Clears bit 5 of a byte at offset 1d6 of a structure in a1, and if bit 5 of that byte is set, a non-zero value in a6, and a short at offset 1c of a pointer in a1 is 10, it sets bit 1 of the byte, clears a pointer in a1, writes 2048 to a global variable, calls a routine via a pointer, and sets a global variable to 32769.
- `start_timed_read_request`: Sets a global variable to one, clears a 32-bit value at offset 142 of a structure in a6, stores a 32-bit value from d0 at offset 146, sets a 16-bit value at offset 13e to nine, calls a routine via a pointer, and if a byte at offset 1f of a1 is non-zero, sets that byte to 11.
- `toggle_status_bit`: Tests bit 7 of a byte at offset 79 of a structure in a1.
- `check_serial_status_and_set_bit`: Clears bit 7 of a byte at offset 69 of a structure, checks bit 1 of a byte at offset 472 of a structure, and checks if a longword at offset 78 of the structure is non-zero.
- `toggle_interrupt_and_call_2`: Writes 2049 to the hardware register at offset 250, calls another routine, writes 35329 to the hardware register at offset 250, and then branches to another location.
- `validate_and_update_serial_parameters`: Validates a range of input bytes and a long value within a structure, then copies several fields to a target structure in a6.
- `compute_unit_channel_masks`: Calculates a 16-bit mask based on a byte at offset 4d of a1, modifies it based on a byte at offset 4e of a1 and a bit in a byte at offset 45 of a6, stores the result in a structure in a6, and calls two other routines to store results at offsets 1d2 and 1d0.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_byte_and_branch_2`: Sets bit 2 of a byte at offset 31 of a structure in a1 and branches to another routine.
- `calculate_serial_period`: Moves a long from offset 450 of a structure in a6 into d0 and d1, shifts d0 left by 3, subtracts d1 from d0, loads a long into d1, compares d0 to 65535 and branches to another routine if less than or equal, otherwise it shifts d0 right by 5, performs an unsigned division of d1 by d0, masks d1 to 16 bits, shifts d1 right by 5, and branches to another routine.
- `store_serial_period_quotient`: Divides a 32-bit value in d1 by a 16-bit value in d0, stores the quotient in a structure in a6 and a global variable, clears bit 5 of a byte at offset 1d8 of a6, and if a byte at offset 42 of a6 is 8 and bit 0 of a byte at offset 45 of a6 is set, it performs a bitwise OR on the quotient and sets bit 5 of the byte at offset 1d8.
- `process_io_requests`: Loads two addresses into a0 and a3, calls another routine twice, and then branches to another routine.
- `execute_io_requests`: Loads two different addresses into a0 and a3, then calls a routine twice before branching to another routine.
- `store_pointer_to_offset`: Stores the value in a0 into a 32-bit word at offset 10 of a structure in a1.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `configure_serial_device_state`: Updates several status bytes and pointers within a structure, calls a function via a pointer in the structure, and sets a global variable to a specific value before returning.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_and_branch`: Clears d0 and branches to another location.

### parallel.device

- `update_buffer_status_2`: Checks if bits 5 and 6 are set in two different memory locations, and if so, increments a 16-bit counter at offset 20 of a structure in a6, copies a byte from offset 117 of a structure in a6 to offset 53 of a structure in a1, and sets bit 4 of a byte at offset 76 of a structure in a6; otherwise it sets a byte at offset 31 of a structure in a1 to 1.
- `update_buffer_status`: Increments a 16-bit counter at offset 32 of a structure in a6, copies a byte from offset 117 of a structure in a6 to offset 53 of a structure in a1, sets bit 4 of offset 118 in a6, and if bit 1 of the byte at offset 53 in a1 is set, copies two 32-bit values from offsets 90 and 94 of a structure in a6 to offsets 54 and 58 of a structure in a1.
- `update_status_and_copy_data`: Clears a byte at offset 76 of a structure in a6, conditionally sets bit 5 of offset 75, increments a 16-bit counter at offset 20 of a6 if two bytes are zero, copies a byte from offset 35 of a1 to offset 75 of a6, sets bit 4 of offset 76, and conditionally copies two 32-bit words from a6 to a1.
- `process_parallel_event_status`: Checks a status field in a structure to update a global variable and a status byte, then calls handle_parallel_event if specific bit conditions are met.
- `handle_parallel_event`: If a short at offset 1c of a1 is 3, clears bit 0 of a global variable and, if bit 0 is already zero and bit 2 of a byte at offset 76 of a6 is not set, calls a routine via a pointer; it then sets bit 2 of the byte at offset 76 of a6 and calls another routine via a pointer passing a6 and a1.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `process_event_status`: Writes the byte 253 to offset 31 of a structure in a1 and branches to another routine.
- `check_device_long`: Tests a longword at offset 36 of a structure in a1 and branches if non-zero, otherwise it clears a longword at offset 32 of the same structure and branches to another routine.
- `manage_device_list`: Clears bit 1 of a byte at offset 30 of a structure in a1, then checks bit 2 of a byte at offset 118 of a structure in a6.
- `update_device_status`: Sets several fields in a structure, modifies a global variable, and performs bitwise operations on memory values.
- `process_device_event_queue`: Manages a linked list of device status updates by decrementing a counter, updating status fields, and performing a loop to find a value in a byte array.
- `update_device_counters_and_status`: Increments two counters in a structure, updates a status field with a counter value, clears specific bits in status bytes, and calls two external routines via pointers in a structure.
- `update_device_status_2`: Copies a 32-bit value from offset 46 of a structure in a6 to offset 32 of a structure in a1, clears bit 3 of a byte at offset 118 of a6, and clears bit 2 of a byte at offset 30 of a1.
- `test_cia_register`: Reads a byte from a CIA register at offset 32 of the hardware address, branches if non-zero, then tests bit 3 of a byte at offset 118 of a structure in a6 and branches accordingly.
- `execute_status_update`: Calls another routine, then calls a routine via a pointer at offset 46 of a structure in a6, sets a byte at offset 118 of a structure in a6 to 0, and if bit 5 of a byte at offset 53 of a structure in a1 is set, sets bit 5 of a byte at offset 75 of a structure in a6; it also increments a 16-bit counter at offset 32 of a structure in a6 if bit 0 of a byte at offset 53 of a structure in a1 is set.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_if_not_zero`: Checks if a 32-bit value at offset 26 of a structure in a6 is non-zero and, if so, calls a routine via a pointer located at an offset from a value at offset 22 of a6.
- `invoke_callback_pointer`: Executes a function through a pointer stored in a structure if the associated index is not zero.
- `process_linked_list`: Loads a pointer from offset 76 of a6 into a0, moves the value at a0 into a1, and moves the value at a1 into d0.
- `clear_word_field`: Clears a 16-bit word at offset 114 of a structure in a6.
- `modify_status_byte`: Sets bit 5 of a byte at offset 30 of a structure in a1, sets a byte at offset 31 of the structure to 254, and calls a routine via a pointer stored in a structure in a6.
- `update_global_and_field`: Manipulates bits within a global variable and updates a byte in a structure using a library function call.
- `test_status_bit_2`: Tests bit 2 of a byte at offset 118 of a structure in a6 and branches if it is set, otherwise it writes the byte 1 to offset 31 of a structure in a1 and branches to another routine.
- `copy_structure_fields`: Copies a byte from one structure to another and, if a specific bit is set, copies two long words as well.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `recurse_on_pointer`: Saves a0 and a6 to the stack, calls a routine using a pointer from the stack, increments the stack pointer by 8, and returns.
- `recursively_calls_itself`: Calls itself and then returns.
- `call_cleanup_routine`: Pushes the contents of a1 onto the stack, calls another routine, and then returns.
- `jump_to_handler`: Performs a long jump to an address stored at offset 2470 of a structure in a1.
- `push_and_call_handler`: Pushes a long value from a1 onto the stack, calls a routine, increments the stack pointer by 4, and returns.
- `call_and_adjust_stack`: Pushes a1 onto the stack, calls a routine using a pointer from the stack, and increments the stack pointer by 4.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `execute_library_loop`: Saves registers d2-d3 and a2 to the stack, loads a longword from offset 16 of the stack into d3, loads a pointer from offset 20 of the stack into a2, stores a2 into a global variable, and calls a routine at an offset from the pointer in a2; it then enters a loop that repeats if d0 is non-zero.
- `execute_clipboard_logic`: Stores a value from d3 into a global variable, calls three different routines, and returns the result of the third routine's comparison with zero.
- `call_two_functions`: Calls two different functions via library vectors and returns one.
- `process_clipboard_data`: Calls a routine, then calls another routine with two arguments, and if the result is non-zero, it either sets a word at offset 94 of a structure in a1 to 0 or increments a longword at offset 14 of a structure in a1 based on a value at offset 14 of another structure; it then calls another routine and finally calls a third routine.
- `call_clipboard_handler`: Clears a 32-bit value at offset 14 of a3, pushes a3 onto the stack, pushes the address at offset 72 of a2 onto the stack, calls a routine, cleans up the stack, and branches to another location.
- `zero_word_field`: Stores a zero word into a specific offset within a structure and then calls another routine.
- `handle_pointer_chain`: Saves registers d2 and a2 to the stack, loads a pointer from offset 12 of the stack into a2, calls a routine, loads a pointer from offset 72 of a structure in a2 into a0, and if the longword at that pointer is zero, it branches; otherwise it pushes a value onto the stack, calls another routine, and branches to another routine.
- `update_pointer_chain`: Pushes the address at offset 72 of a2 onto the stack, calls a routine, clears a 32-bit value at offset 86 of a2, calls another routine, and then restores d2 and a2 from the stack.
- `copy_buffer_segment`: Copies a specified range of bytes from a linked list of 512-byte buffers into a destination buffer, updating a pointer to the current buffer position within a structure.
- `compare_and_calculate_offset_2`: Compares a longword from a structure in a2 with a value on the stack, and if the value is not lower, calculates the difference between a longword in a2 and the sum of a longword on the stack and a value at offset 44 of the stack, then branches to another routine.
- `traverse_and_copy_list`: Traverses a linked list starting from a pointer found at offset 86 or 72 of a structure in a1 until a specific longword value is reached.
- `compare_and_update_pointer_2`: Copies d1 to d0, compares d0 with a value at offset 14 of a structure in a1, and if d0 is lower than or equal to that value, updates a1 with a pointer from offset 0 of that structure and branches back to the start of this routine.
- `compare_and_branch_on_value`: Copies d3 to d5, tests d5, and if d5 is not zero, calculates a value in d0 by subtracting d2 from 512, compares d5 to d0, and if d5 is less than or equal to d0, moves d2 to d4 and branches to another location; otherwise, it moves d2 to d4 and branches to another location.
- `copy_data_loop`: Copies bytes from a table starting at offset 18 of a1 to a destination pointed to by a3, using d4 as an index, and loops until d4 reaches 512.
- `move_index_and_branch`: Moves a 16-bit word from d2 to d4 and branches to another location.
- `copy_data_conditional`: Copies bytes from a table starting at offset 18 of a1 to a destination pointed to by a3, using d4 as an index, and loops while d4 is less than the value of d1 plus d2.
- `store_long_word`: Stores a long word into a specific offset within a structure and then calls another routine.
- `reallocate_and_append_clipboard_buffer`: Checks if a clipboard buffer requires resizing based on a length parameter, reallocates memory if necessary, and then iterates through a linked list of memory blocks to copy a source byte stream into them, managing block boundaries and remaining byte counts.
- `copy_buffer_to_linked_list`: Iterates through a linked list of memory blocks to copy a source byte stream into them, managing block boundaries and remaining byte counts.
- `branch_to_offset_routine`: Pushes the address of an offset of a structure in a2 onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `append_buffer_to_linked_list`: Traverses a linked list of buffers to find the correct insertion point based on an offset and length, then copies data from a source pointer into the buffers, potentially spanning multiple list nodes, and updates the list pointer in a structure.
- `manage_clipboard_buffer`: Calls three different routines to process data, copy a buffer to a linked list, and check buffer limits, then updates a pointer in a structure at offset 86 of a2.
- `copy_buffer_to_list`: Iterates through a linked list to find a suitable position and then copies bytes from a source buffer into the list nodes.
- `check_buffer_limits`: Tests d4 and a value at offset 20 of a structure in a6; if d4 is non-zero and the value in a6 is not lower than d0 calculated by subtracting d3 from 512, it moves d3 to d1 and branches to another location; otherwise, it branches to the start of this routine.
- `copy_data_with_buffer_management`: Enters a loop that copies bytes from a structure in a3 to a destination at an offset from a pointer in a4, incrementing a counter, and if the counter exceeds 511, it updates a pointer in a4, calculates a value based on a longword in a6, and manages a buffer by copying remaining bytes and updating a longword at offset 20 of a structure in a6.
- `move_index_and_branch_2`: Moves a 16-bit word from d3 to d1 and branches to another location.
- `copy_buffer_data`: Copies bytes from a source pointer to a destination buffer, potentially calling another routine to fetch more data when a pointer is null or a limit is reached.
- `store_register_to_offset`: Stores the contents of a data register into a memory location calculated by adding a fixed offset to a pointer held in an address register.
- `check_status_bits`: Calls a routine, then checks bit 7 and bit 0 of a byte at offset 30 of a structure in a2, and calls another routine if bit 0 is not set.
- `modify_byte_bits`: Clears the least significant bit and sets the most significant bit of a byte at a specific offset in a buffer.
- `modify_byte_bits_2`: Calls another routine, then checks bit 7 of a byte at offset 30 of a structure in a2 and calls another routine if it is not set, clears bit 1 of that byte if a comparison between a2 and a parameter fails, and finally clears bit 3 of that byte before returning.
- `compare_structure_values`: Compares a longword at offset 48 of a structure in a1 with a longword at offset 48 of a structure in a0, and sets d0 to one if the value at offset 48 of a0 is less than the value at offset 48 of a1.
- `set_d0_to_one_and_branch`: Sets register d0 to one and performs a short branch to another location.
- `clear_d0_and_branch`: Sets d0 to zero and performs a short branch to another location.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_structure_fields_2`: Copies a 32-bit value from offset 40 of a second structure to offset 152 of a first structure, a 16-bit value from offset 16 of the second structure to offset 146 of the first, and a 32-bit value from offset 48 of the second structure to offset 148 of the first, then calls another routine.
- `modify_clipboard_status_flags`: Sets bit 5 of a byte at offset 30 of a structure pointed to by a value at offset 12 of the stack frame, then checks a byte at offset 66 of a second structure pointed to by a value at offset 12 of the stack frame; if that byte is not zero, it clears bit 0 of the byte at offset 30 of the first structure, otherwise it returns.
- `update_clipboard_buffer_state_transitions`: Iterates through a structure to manage clipboard buffer states by checking status words and updating data offsets, buffer sizes, and internal counters.
- `clear_d3_and_branch`: Clears the 16-bit register d3 and branches to another routine.
- `process_clipboard_buffer_states`: Updates clipboard buffer state transitions by checking a status word at offset 64 of a structure and manages buffer size and data offsets based on specific status codes.
- `call_subroutine_and_set_flag`: Pushes the value in a2 onto the stack, calls a routine, adjusts the stack pointer, writes 1 to a 16-bit field at offset 94 of the structure in a2, and branches to another routine.
- `update_clipboard_buffer_states_2`: Processes clipboard buffer state transitions by checking a status word at offset 64 of a structure.
- `clear_d3_register_2`: Clears the d3 register and branches to another routine.
- `update_clipboard_buffer_states`: Updates a clipboard buffer structure by checking a status word to determine whether to copy a 32-bit value, increment or decrement a counter, or modify a linked list based on various state flags and length constraints.
- `manage_clipboard_buffer_states`: Checks a status word in a structure to determine if it should update a 32-bit value, increment or decrement a counter, or modify a linked list of buffers based on specific state values.
- `update_buffer_pointer`: Loads a longword from offset 90 of a2 and a longword from offset 36 of a4, adds a longword from offset 44 of a4 to the second value, compares the sum to the first value, and if the sum is lower than or equal to the first value, stores the longword from offset 36 of a4 into offset 32 of a4.
- `update_buffer_offset`: Subtracts a longword at offset 44 of a4 from a longword at offset 90 of a2, stores the result at offset 32 of a4, and then adds that result to the longword at offset 44 of a4.
- `update_clipboard_buffer_state_2_2`: Clears a longword at offset 32 of a4, writes a longword containing all bits set to offset 48 of a4, decrements a word at offset 110 of a2, calls another routine, and if the word at offset 110 of a2 is non-zero, it clears a word at offset 64 of a2 and branches, otherwise it clears the word and branches.
- `update_clipboard_buffer_state_2`: Checks a status word at offset 64 of a structure; if it equals 4, it updates a 32-bit value at offset 90 by either resetting it to zero or incrementing it by 8 based on the result of another routine.
- `copy_clipboard_pointer`: Copies a longword from an offset of 44 in a6 to an offset of 90 in a2, pushes a3 and a4 onto the stack, calls another routine, and then restores the stack and branches to another location.
- `check_clipboard_status`: Compares a word at offset 64 of a structure in a4 with 4, and if they are equal, writes 4 to offset 64 of a structure in a2 before branching.
- `clear_d3_register`: Clears the d3 register and branches to another location.
- `manage_clipboard_buffer_state`: Checks a status word at offset 64 of a structure; if it equals 4, it updates a 32-bit value at offset 90 by either resetting it to zero or incrementing it by 8 based on the result of another routine, then clears a 16-bit value at offset 110 and calls another routine.
- `clear_status_word_2`: Clears a long value at offset 90 of a structure in a2 and branches to another location.
- `update_clipboard_buffer_state`: Iterates through a structure to manage buffer offsets and status flags, updating a 16-bit value at offset 94 and clearing specific bits in a status byte at offset 30 of a pointed-to structure.
- `traverse_pointer_list`: Traverses a linked list of pointers while comparing a value at offset 48 of a structure in a2 against the offset 12 value of the current pointer, then calls another routine with a parameter from the final pointer's offset 4.
- `set_zero_and_branch_2`: Sets d1 to zero and branches to another location.
- `call_library_function`: Saves three registers to the stack, calls a library function using a base address from a structure offset, restores the stack pointer, and returns.
- `set_byte_value`: Writes the constant byte value 253 to offset 31 of a structure in a0 and calls another routine.
- `set_byte_constant_offset`: Writes a constant byte value to a specific offset within a structure and then calls another routine.
- `set_byte_constant_offset_and_call`: Writes the constant byte value 253 to offset 31 of a structure pointed to by a value loaded from the stack and then calls another routine.
- `set_byte_constant_offset_and_call_2`: Writes the constant byte value 253 to offset 31 of a structure pointed to by a value retrieved from the stack and then calls another routine.
- `set_byte_constant_offset_and_call_3`: Stores the constant byte value 253 at offset 31 of a structure pointed to by a value loaded from the stack and then calls another routine.
- `update_structure_byte_sequence`: Pushes d2 onto the stack, retrieves a value from offset 8 of the stack into d2, pushes d2 onto the stack, calls three different routines using d2 as an argument, adjusts the stack pointer, and returns.
- `update_clipboard_data`: Copies a 32-bit word from offset 56 of a structure pointed to by a value at offset 24 of the input structure to offset 48 of the input structure, sets bit 4 of a byte at offset 30 of the input structure, and then calls another routine using a value from a register plus an offset of 96 and then calls a second routine.
- `call_subroutine_with_offset`: Calls another routine using a value from a register plus an offset of 96 and then calls a second routine.
- `increment_clipboard_counter`: Increments a 32-bit word at offset 56 of a structure pointed to by a value at offset 24 of the input structure, stores the result at offset 48 of the input structure, and then calls another routine using a value from a register plus an offset of 112 and then calls a second routine.
- `call_subroutine_with_different_offset`: Calls another routine using a value from a register plus an offset of 112 and then calls a second routine.
- `copy_structure_field`: Copies a longword from offset 56 of a structure pointed to by an offset 24 field of a structure in a0 to offset 48 of that same structure in a0, then calls another routine.
- `copy_structure_field_offset`: Copies a longword from offset 60 of a structure pointed to by an offset 24 field of a structure in a0 to offset 48 of that same structure in a0, then calls another routine.
- `manage_clipboard_linked_list`: Searches a linked list for a node matching a specific identifier; if found, it increments a counter and updates a pointer in a structure, otherwise it allocates a new node, initializes its members, and performs a series of memory transfers and buffer management operations involving a loop that processes data in chunks.
- `compare_and_branch_2`: Compares d3 with a longword at offset 14 of a structure in a3; if they are not equal, it sets d4 and d2 to zero and branches to another location.
- `manage_clipboard_buffer_transfer_2`: Increments a counter in a structure and a global variable, then performs a series of conditional checks and calls to other routines to manage buffer transfers, including a loop that processes data in chunks of 512 bytes.
- `manage_clipboard_buffer_transfer`: Initializes a structure with several fields and calls a routine via a function pointer, then performs a series of conditional checks and calls to process data in chunks of 512 bytes.
- `process_buffer_limit`: Calls a routine, then if the result in d0 is not 512, it clears a word at offset 94 of a structure in a3 and subtracts 512 from a value at offset -4 of a structure in a6.
- `loop_and_clear_word`: Tests a word at offset 94 of a structure in a3 and a long value at offset -4 of a structure in a6; if both tests pass, it pushes a3 onto the stack, calls a routine, and if the return value in d0 is not zero, clears the word at offset 94 of a3 and branches back to the start of this routine.
- `process_complex_structure_2`: Updates several fields in a structure based on conditional checks, processes a pointer chain in a loop, and returns a byte from a structure offset.
- `process_complex_structure`: Increments values in a structure, performs conditional library calls based on a flag, and returns a value from a structure offset.
- `decrement_and_cleanup_clipboard_structure`: Decrements a 16-bit counter at offset 18 of a structure, and if it reaches zero, performs a recursive cleanup of linked list data, resets several 32-bit fields at offsets 20 and 24, and decrements a global byte before conditionally calling another routine.
- `verify_library_result`: Calls a routine using a pointer at offset 18 of a structure in a4, compares the result in d0 with d4, and if they are not equal, clears a word at offset 94 of a structure in a3 and sets d4 to zero.
- `clear_status_word`: Clears a word at offset 94 of a structure in a3 and branches to another location.
- `cleanup_and_call_library`: Calls three routines, then clears two 32-bit fields at offsets 20 and 24 of a structure in a2, decrements a global byte, and if that byte is zero and bit 3 of a global variable is set, calls another routine before returning.
- `cleanup_structure_data`: Resets two 32-bit fields at offsets 20 and 24 of a structure in a2 to zero, decrements a global byte, and conditionally calls another routine if the byte is zero and bit 3 of a global variable is set.
- `execute_chained_calls`: If a word at offset 252 of a structure is not zero, it calls a routine using a pointer from a global variable, then calls another routine using a pointer from a stack-based address, and finally moves a longword from a global variable into d0.
- `set_global_bit`: Sets bit 3 of a global variable and returns zero in d0.
- `update_status_and_call`: Writes the constant byte value 5 to offset 8 of a structure in a2, performs a bitwise AND on offset 30 of that structure, and calls another routine if a word at offset 28 is less than or equal to 12.
- `invoke_table_entry_4`: The routine invokes a function pointer retrieved from an offset within a global variable using an index.
- `check_bits_and_set_byte`: Calls another routine, checks if bits 7 and 6 of a byte at offset 30 of a structure in a2 are both clear, and if so, writes the constant byte 254 to offset 31 of that structure and calls another routine if bit 5 is set, followed by another routine call before calling a final routine and returning.
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
- `update_linked_list_node_2`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the value at that location by four, clears the following word, and stores the original pointer value into the word after that.
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
- `store_byte_at_pointer_3`: Stores the byte from d0 into the memory location pointed to by a3 and increments a3.

### printer.device

- `call_initialization_pointer`: Branches to a routine called through a pointer at offset -54 of a structure in a6, and if the result is non-zero, stores that result in a global variable.
- `set_status_byte`: Sets register d0 to negative 2 and calls another routine.
- `manage_linked_list`: Sets a bit in a byte, performs a linked list insertion if a flag is set, and calls functions through pointers retrieved from a table.
- `execute_printer_initialization`: Calls a routine through a pointer at offset -54 of a structure in a6, and if the result is non-zero, stores that result in a global variable, sets a value at offset 34 of the result structure, and calls another routine through a pointer at offset -272 of a6.
- `process_device_initialization`: Calls three routines through pointers retrieved from a structure in a6, and if the third call returns zero, it fills a buffer starting at offset 406 of d0 with the value 45019 in a loop, then sets several fields in the structure at d0 and calls two more routines through pointers.
- `execute_indirect_calls`: Calls four different routines using pointers retrieved from a table based on offsets from a global variable.
- `execute_status_updates`: Calls three different routines in sequence and then returns.
- `set_status_byte_alt`: Sets register d0 to negative 3 and calls another routine.
- `set_status_bit`: Sets bit 0 of a byte at offset 61 of the structure in a6 and returns.
- `clear_status_bit`: Clears bit 0 of a byte at offset 61 of the structure in a6 and returns.
- `check_pointer_and_branch`: Loads a pointer from an offset of 72 in a6 into a1, checks if the pointer is null, and if not null it calls another routine, otherwise it branches to itself.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_previous`: Performs a short branch to a previous address.
- `initialize_and_process_lists`: Initializes several fields in a structure pointed to by a6, calls several routines through a table, and enters a loop that iterates through linked lists while clearing specific bits in a byte at offset 2470 of a6 based on values returned from other routines.
- `compare_and_clear_bit`: Compares register d0 with a value at an offset from a6, and if they are equal, clears bit 1 of a byte at an offset from a6.
- `process_device_lists`: Enters a loop that checks bits in a register to iterate through linked lists, calling routines through a table and modifying bytes at specific offsets in a structure.
- `call_buffer_processor`: Loads a pointer from offset 4 of a7 into a0, loads a value from offset 8 of a7 into d0, and calls another routine.
- `update_status_and_pointers`: Calls a routine, and if it returns zero, it modifies a byte at offset 2486 of a pointer from a global variable, then either adds 108 or 190 to that pointer before writing two values and a constant to the structure at the new pointer location and calling two more routines.
- `adjust_stack_and_branch`: Adds 8 to the stack pointer a7 and branches to a previous address.
- `set_status_bit_5`: Sets bit 1 of a byte at offset 2454 of a6, loads the address of a structure at offset 190 of a6 into a1, and branches to a previous address.
- `check_status_byte`: Loads a pointer from a global variable, reads a byte from offset 31 of that pointer, and if it is not zero, checks bit 0 and bit 1 of a byte at offset 1538 of a structure in a6, calling another routine if either bit is set.
- `process_status_loop`: Enters a loop that calls three different routines via pointers from a global variable, checks a value at offset 14c of a structure in a6, and performs conditional bitwise operations on a byte at offset 9b6 of that structure based on the result of a fourth routine.
- `compare_and_branch_status_2`: Loads the address of a structure at offset 272 from a6, compares the value at offset 8 of that structure to 7, and if it is not equal, calls another routine and then branches to a previous instruction.
- `compare_and_branch_status`: Compares the value at offset 272 of a6 with a value in d0, and if they are not equal, calls another routine and then branches to a previous address.
- `call_status_and_branch`: Calls another routine and then branches to a previous address.
- `update_structure_fields`: Loads a pointer from a global variable, writes the value 9 to offset 28 of a structure in a1, copies a 32-bit value from offset 50 of the structure in a0 to offset 32 of the structure in a1, and clears a 32-bit value at offset 36 of the structure in a1.
- `clear_status_byte`: Sets a byte at a specific offset to zero and calls a routine through a pointer found in a structure.
- `verify_offset_byte`: Loads the address of a structure at offset 272 from a6, compares the byte at offset 8 of that structure to 7, and returns if they are not equal.
- `execute_callback_sequence`: Calls a routine through a pointer in a structure and then calls another routine using a value from a global base.
- `check_and_clear_status`: Loads a pointer from a table, checks a byte at offset 31 of that pointer, and if it is not zero, clears a byte at offset 1558 of a6 and performs bitwise operations on a byte from a6 before returning.
- `execute_loop_logic`: Performs a loop that calls three different routines and checks a longword at offset 76 of a structure in a6 to determine whether to continue.
- `compare_and_branch_pointer_2`: Loads a longword from offset 46 of a6 into a1, compares it with d0, branches if they are not equal, clears bit 1 of a byte at offset 470 of a6, calls another routine, and then branches to another location.
- `compare_and_branch_pointer`: Compares a longword at offset 110 of a6 with a longword in d0, branches if they are not equal, calls another routine, and branches based on the zero flag.
- `branch_to_subroutine`: Calls another routine and then branches to a previous instruction.
- `check_status_byte_3`: Saves registers to the stack, compares a byte at offset 2491 of a6 to one, and if they are not equal, pushes the address of a location to the stack and branches to another routine.
- `update_status_and_interrupts`: Updates a byte at offset 294 of a global variable, modifies a global variable based on its value, calls another routine through a pointer in a global variable, and clears bit 0 and bit 1 of a byte at offset 2470 of a6 if certain conditions are met, while interacting with the INTENA register.
- `clear_status_bits_2`: Calls another routine through a pointer retrieved from a global variable; if that routine returns zero, it clears bit 0 of a byte at offset 2486 of a structure in a6 and calls another routine if bit 0 was set, then clears bit 1 of the same byte and calls another routine if bit 1 was set.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `update_list_and_copy_string`: Pushes two values from a1 onto the stack, calls a routine through a pointer, restores a1 from the stack, and calls another routine.
- `reset_structure_field`: Sets a 32-bit field in a structure to zero and conditionally updates it from another field based on a routine return value.
- `update_list_and_copy_string_2`: Pushes the value of a1 onto the stack twice, calls a routine through a pointer at offset 288 of a1, restores a1 from the stack, calls another routine, and returns.
- `execute_callback_and_update_list`: Pushes two pointers onto the stack, calls a routine via a pointer, restores the stack, retrieves the first pushed pointer into a1, and calls another routine before returning.
- `dispatch_pending_entry`: Checks a condition via a routine call, and if met, populates a structure with two parameters and executes several routines through a global base.
- `execute_buffer_processing`: Saves registers to the stack, loads a pointer from offset 8 of a structure in a6 into a2, sets a2 to 284 and a4 to 266, calls another routine, and then branches to another routine based on whether the result in d0 is zero.
- `reorder_buffer_by_character`: Calculates the length of a null-terminated string and iteratively reorders its elements into a table based on character values.
- `update_buffer_pointers`: Moves a longword from an address in a3 plus an offset of 468 to a global variable, clears a word at offset 470 of that same global variable, and then branches to another routine.
- `parse_escape_sequences`: Iterates through a source buffer to identify and process control codes and escape sequences, writing the resulting processed bytes into a destination buffer.
- `clear_registers_and_branch`: Sets registers d4, d6, and d2 to zero and then branches to another location.
- `search_and_reorder_buffer_elements`: Iteratively searches through a buffer to find matching characters, then performs complex reordering and copying of elements between a source structure and a global array based on conditional checks and status bytes.
- `copy_memory_and_set_field`: Iteratively copies bytes from one memory location to another while incrementing counters, then sets a 32-bit value at offset 32 of a structure in a2 to either a value from d2 or zero based on whether d2 is zero.
- `clear_memory_offset`: Stores a zero at a specific offset within a memory location pointed to by a register.
- `map_character_table`: Calls another routine, then enters a loop that iterates through a table to match a byte against specific values, performing various transformations and updates to that byte based on the match and a global variable.
- `clear_and_branch_3`: Clears a longword at offset negative 4 from a6 and branches to another location.
- `map_input_to_character`: Iterates through a table to match a byte against a sequence of values, then writes a derived character to a destination pointer based on specific index and value conditions.
- `write_conditional_byte`: Compares d0 to a value from a global table and d5 to 5, and if both conditions are met, stores the byte -1 at the location pointed to by a2.
- `write_byte_on_condition`: Compares d0 to a value from a global table and d5 to 12, and if both conditions are met, stores the byte 75 at the location pointed to by a2 and branches to another routine.
- `check_and_write_byte`: Compares d5 to 9, and if d5 is not equal to 9 and the byte at the location pointed to by a4 is zero, writes the byte 59 to the location pointed to by a2.
- `write_byte_from_table`: Compares d5 to 6, and if d5 is less than 6, copies a byte from a table at offset 362 using d5 as an index into the location pointed to by a2.
- `check_byte_and_write`: If d5 is not zero, it branches to another location; otherwise, it checks if the byte at the address in a4 is at least 29, and if so, writes the value twelve to the address in a2 before branching to another location.
- `map_input_to_character_2`: Iterates through a table to match a byte against a sequence of seven values, then writes a derived character to a destination pointer.
- `process_escape_sequences_2`: Parses a stream of characters to handle control codes, escape sequences, and line endings, writing the resulting processed bytes into a destination buffer.
- `process_printer_escape_sequences`: Iterates through a byte stream to identify control characters, handling escape sequences by matching bytes against a table, processing line feeds by appending carriage returns and line feeds to a buffer, and mapping high-range bytes to entries in a global table.
- `parse_printer_command_stream_3`: Iterates through a stream of bytes to identify and process control characters, including line feeds and escape sequences.
- `compare_printer_byte`: Compares a byte at the address in a3 with 91, and if it is not equal, branches to another location; otherwise it branches to another location.
- `parse_printer_command_stream`: Iterates through a source buffer to parse control characters and escape sequences, writing processed bytes to a destination buffer.
- `check_command_byte`: Compares a byte at the address in a3 with 10, and if it is not equal, branches to another location.
- `parse_printer_command_stream_2`: Parses a stream of bytes to execute commands, handling escape sequences and control characters by calling another routine.
- `write_multiple_bytes`: Writes the value ten to a memory location calculated by adding d2 to a pointer retrieved from a2, increments the pointer in a2, and then repeats this process to write the value thirteen to the next calculated location before branching to another location.
- `write_single_byte`: Writes the value ten to a memory location calculated by adding d2 to a pointer retrieved from a2, increments the pointer in a2, and then branches to another location.
- `check_character_and_call`: Compares a byte at the address in a3 to 155; if it is not equal, it branches to another location, otherwise it pushes d2 and d3 to the stack, calls another routine, restores the stack, and branches to another location.
- `parse_file_path_string`: Processes a string by checking for backslash and colon characters to extract numeric components and path segments, storing the results into a destination buffer while incrementing a counter.
- `parse_path_string`: Parses a string to extract directory components and numeric values, storing the results into a destination buffer.
- `update_pointer_and_table`: Increments a pointer and a global table entry, checks a byte at the pointer, and if it is non-zero, performs arithmetic on a value from a2 and updates a global table entry.
- `calculate_pointer_offset`: Loads a byte from a3, subtracts 160, shifts it left by two, adds it to d4, and then loads a pointer from the resulting address into a1 before branching to another routine.
- `skip_non_numeric_characters`: Checks if the byte at a1 is 92, and if so, increments a1 and checks if the next byte is between 48 and 57, branching back to the start if the character is not a digit.
- `parse_ascii_numeric_string`: Parses up to three consecutive ASCII digits from a string into a single integer value, stores the result in a buffer at an offset determined by an incrementing index, and then processes the remaining string characters through a series of conditional checks and recursive calls.
- `copy_byte_from_source`: Copies a byte from the location pointed to by a1 plus d2 to the location pointed to by a0, then branches based on the value of the byte at a1 plus 1.
- `parse_escape_sequence_byte`: Reads a byte from a source stream and compares it against a table of control characters.
- `update_global_values`: Subtracts the difference between two global variables from a third global variable and clears a fourth global variable.
- `copy_buffered_data_with_overflow_check`: Calls another routine to fill a local stack buffer, then copies bytes from that buffer to a destination pointed to by a global variable and a parameter if the total count does not exceed a threshold.
- `update_and_call_routine`: Adds d3 to a value from a global variable, and if the result is greater than 256, stores d3 in a global variable and either calls another routine or branches based on a bit in a structure.
- `decrement_and_store_global_pointer`: Decrements a 32-bit value from a global variable by one, stores the result in another global variable, and branches to another routine.
- `clear_register_and_branch`: Sets register d4 to zero and branches to another routine.
- `copy_memory_block`: Copies a sequence of bytes from a source address to a destination address, incrementing a global pointer for each byte.
- `clear_byte_and_branch`: Clears a byte at a location pointed to by a register, then branches to another routine.
- `clear_byte_at_offset`: If d1 is 59, increments a3, then clears a byte at the location calculated by adding a2 to a3.
- `set_value_at_pointer`: Compares d1 to 32, and if they are equal, loads a pointer from offset 32 of the stack into a0 and writes 1 to the location pointed to by a0.
- `compare_and_load_pointer`: Compares register d1 to 34; if they are equal, it loads a pointer from offset 32 of the stack into a0 and branches to another routine, otherwise it branches to another routine.
- `parse_character_stream`: Enters a loop that reads bytes, checks if they are within the range of ASCII digits or specific characters, and if a character is a semicolon, increments a pointer and clears a byte at the location pointed to by a2 and a3.
- `fetch_indexed_global_value`: Increments a value at a pointer, then uses the original value to fetch a word from a global table, and calls another routine if a condition is met.
- `return_register_value`: Returns the value currently held in a data register.
- `update_global_pointer_from_memory`: Loads a pointer into a1, decrements a 32-bit value from a global variable by one, stores the result at the address in a1, and branches to another routine.
- `decrement_and_check_table`: Decrements a value pointed to by a register and continues looping as long as the value is positive and a specific bit pattern is found in a table.
- `reset_global_state`: Clears three global variables, sets a byte at offset 2720 of a structure to zero, and sets another global variable to 3.
- `copy_data_to_globals`: Copies six 16-bit values from a structure into a set of global variables.
- `compare_global_data`: Compares six 16-bit values from a structure against a set of global variables, returning zero if all match or all ones otherwise.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_nested_calls`: Extracts four bytes and a short from a structure pointed to by a parameter, then performs a series of conditional calls to other routines based on the return value of a primary call and the value of the short.
- `dispatch_conditional_routines`: Calls a routine with parameters 1 and 0, and if the result is zero, calls another routine with a pointer from a structure in a6; if the first call returned zero and d2 is zero, it also calls another routine with parameters 3 and 0.
- `copy_string_with_logic`: Calls a routine through a pointer at offset 42 of a table, then conditionally updates a global variable based on a parameter, and enters a loop to copy bytes from a source buffer to a destination buffer while handling specific byte values.
- `handle_specific_values`: Compares a word at offset 22 of the stack to 54 and 53, and if the value is 53, stores a bitwise result in a global variable and branches to another routine.
- `copy_string_with_escape_handling`: Copies bytes from a source buffer to a destination buffer, treating specific byte values as escape characters or terminators.
- `set_d1_value`: Compares the byte in d1 to 255; if it is not 255, it sets d1 to 255 and branches to another location.
- `copy_string_data`: Enters a loop that copies bytes from a source buffer to a destination buffer, updating an index and checking for a null terminator or a specific byte value.
- `execute_jump_table_entry`: Pushes a 32-bit value from d2 onto the stack, loads a new address into a0 from the stack, and performs a jump to an address calculated using a table and an offset derived from d1.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `validate_printer_buffer_dimensions_2`: Extracts a pointer and several coordinate and size values from a structure, calculates the width and height of a buffer by subtracting coordinate pairs, performs bounds checking against minimum required dimensions, and calls another routine to determine the minimum buffer size based on available memory.
- `validate_printer_buffer_dimensions`: Calculates the width and height of a buffer by subtracting coordinate pairs from a structure, performs bounds checking against minimum required dimensions, and updates the buffer's width and height values in a structure.
- `recalculate_printer_buffer_dimensions`: Calculates the width and height of a buffer based on structure members, performs bounds checking, and updates several values in a structure including minimum dimensions and a bit-swapped status flag.
- `update_printer_buffer_parameters`: Calculates minimum values from two structure members at offsets 26 and 30, stores them on the stack, and performs complex bitwise manipulation on a global status byte.
- `update_dynamic_state_parameters_2_2`: Calculates minimum values from two structure members at offsets 26 and 30 and stores them on the stack, then performs complex bitwise manipulations on a global variable and updates several values in a structure based on conditional checks and bitwise flags.
- `compare_and_branch_7`: Compares the word in register d0 to 2048; if the values are not equal, it branches to another routine, otherwise it branches to another routine.
- `load_constant_and_branch_2`: Loads the value 12 into register d1 and branches to another routine.
- `load_constant_and_branch_3`: Loads the value 15 into register d1 and branches to another routine.
- `update_dynamic_state_parameters_2`: Calculates minimum values from two structure members at offsets 26 and 30 and stores them on the stack, then performs complex bitwise manipulations and conditional updates on several global variables and values pointed to by registers, including a bit-swapping operation on a global byte.
- `update_dynamic_state_parameters`: Calculates a minimum value from a structure member and stores it on the stack, then performs complex bitwise manipulations and conditional updates on several global variables and values pointed to by registers.
- `modify_stack_offset_value`: Sets d0 to 7 and stores it at an offset of 4 before a negative offset from a6, checks a byte at a global variable and branches if it is non-zero, otherwise it adds 12 to d6, checks another byte at a global variable and branches if it is non-zero, otherwise it adds the value at the offset in a6 to d0 and stores the result back at that offset before branching to another routine.
- `check_and_branch_status`: Checks a byte at a global variable and branches if it is zero, then checks another byte at a global variable and branches if it is zero, otherwise it clears the stack and branches to another routine.
- `update_color_range_bounds`: Calculates the minimum and maximum values for two color components by comparing current values against stored bounds and calls calculate_color_palette_table to populate a table with the resulting color data.
- `calculate_color_palette_table`: Calculates a 64-entry color table by iterating through a range of values, extracting bit-plane components from a source, and storing them in a table.
- `generate_color_palette_table_2`: Calculates a 64-entry color table by iterating through a range of values, extracting bit-plane components from a source, and storing them into a table.
- `generate_color_palette_table`: Calculates a range of values based on two input parameters and populates a 64-entry table by extracting bit-plane components from a source value.
- `calculate_and_populate_color_palette`: Calculates a range of values based on two input parameters and populates a 64-entry table by extracting bit-plane components from a source value.
- `populate_planar_color_palette_2`: Clears a table of 64 entries and populates three separate planar bit-planes by extracting 4-bit components from a source value via a call to populate_planar_color_palette.
- `populate_planar_color_palette`: Clears a table of 64 entries and populates three separate planar bit-planes by extracting 4-bit components from a source value via a call to generate_planar_color_palette.
- `generate_planar_color_palette`: Clears a table of 64 entries and populates three separate planar bit-planes by extracting 4-bit components from a source value via a call to convert_rgb_to_planar_palette.
- `convert_rgb_to_planar_palette`: Extracts 4-bit color components from a source value to populate a planar color table and performs bitwise transformations on a series of 32-bit values in a loop.
- `initialize_global_values`: Sets three global variables to specific constant values, calls another routine with five arguments, and then calls another routine multiple times based on the results and a condition involving a value at offset 24 of a pointer in d5.
- `execute_function_and_store_result`: Pushes a constant onto the stack, clears three 32-bit words on the stack, loads a pointer from a global variable into a2, calls the routine at that pointer, stores the result from d0 into a global variable, adjusts the stack pointer, and branches to another routine based on the zero flag.
- `check_printer_status_and_call`: Tests two consecutive bytes in a global variable; if either byte is zero, it branches to another routine, otherwise it calls another routine and then branches to another routine.
- `test_and_call_callback`: Tests a byte in a global variable; if the byte is zero, it branches to another routine, otherwise it calls another routine and then branches to another routine.
- `test_and_execute_callback`: Tests a byte in a global variable and, if it is zero, branches to another routine; otherwise, it calls another routine and then branches to another routine.
- `execute_conditional_callback_2`: Calls a routine, stores the result in a global variable, and if that global variable is zero, branches to another routine; otherwise, it branches to another routine.
- `execute_conditional_callback`: Checks if a global variable is divisible by a value at offset 24 of a pointer in d5, or calls a routine through a pointer and if the result is zero, calls another routine with two zero arguments.
- `call_empty_routine`: Calls a routine with two zero arguments and returns.
- `calculate_weighted_value`: If a global variable is zero, it checks bit 2 of a byte at offset 21 of a global variable; if set, it finds the minimum of three bytes and returns it, otherwise it calculates a weighted sum of three bytes and optionally subtracts the result from 15 based on a value at offset 2658 of a global variable.
- `find_minimum_byte`: Checks bit 2 of a byte at offset 21 of a global variable and, if set, finds the smallest byte value among three provided bytes and returns it, otherwise it returns the original second parameter.
- `update_pointer_structure`: Calculates a value based on a global variable and two constants, calls another routine with the result, and then updates a global variable and a structure member based on the returned value and bit tests.
- `update_global_pointers`: Updates four global pointers by adding a global variable to them and calls update_pointer_table, update_pointer_table_2, and call_formatted_print.
- `update_pointer_table_2`: Stores a longword from offset 114 into a location pointed to by a2, loads a longword from offset 4 into a0, tests bit 3 of a byte at offset 21, calls another routine, and adds a global variable to four global pointers at offsets 28, 36, 32, and 40.
- `test_bit_and_subtract`: Stores a longword from offset 114 into a location pointed to by a2, moves a longword from offset 4 into a0, tests bit 3 of a byte at offset 15 of a0, and either branches or subtracts d2 from 15 in d1.
- `process_word_and_branch`: Moves words into d1 and d0, performs a logical shift right on d0, and branches based on the result of a comparison with 1.
- `compare_and_branch_constant`: Compares d0 to 2 and branches to an earlier instruction if it is not equal, otherwise it branches to another routine.
- `decrement_and_loop`: Decrements d0 by three and branches to an earlier instruction if the result is not zero, otherwise it branches to another routine.
- `store_byte_and_branch`: Stores a byte from d1 into a location at offset 272 and branches to another routine.
- `store_byte_and_branch_offset`: Stores a byte from d1 into a location at offset 274 and branches to another routine.
- `execute_table_call`: Stores a byte from d1 into a location at offset 273, pushes a pointer from a2 onto the stack, calls another routine, stores the result in d0 into the location pointed to by a2, increments the stack pointer by 4, stores the pointer from a2 into a location at offset 276, and branches to a previous address.
- `update_pointer_table`: Loads a 32-bit value from a table using an index and updates four global pointers by adding a global base address to them.
- `calculate_and_update_pointers`: Calculates a value based on the product or sum of two globals and updates four global variables by calling another routine multiple times.
- `execute_callback_chain_2`: Sets a global variable to zero and sequentially calls several routines through pointers, checking their return values to determine whether to continue or call another routine.
- `process_multiple_callbacks`: Checks if a global variable is zero, then sequentially calls three different routines through a pointer, and if any of them return a non-zero value, stores that value in a location pointed to by a2 and returns.
- `compare_and_store_result_2`: Compares two values using a table and a global variable, then calls another routine to store a result into a location pointed to by a2.
- `compare_and_store_result`: Calls another routine and, if the result is non-zero, calls a third routine using parameters from a2, a3, and a4, then stores the result in the location pointed to by a2.
- `clear_d0_and_restore_registers`: Sets d0 to zero and restores registers a2 through a4 from the stack before returning.
- `increment_and_execute_conditional`: Increments a global variable and, if it meets a condition involving a value from a pointer at offset 24 of another structure, calls another routine twice and stores the result in a global variable.
- `execute_and_store_result`: Pushes three zeros onto the stack, loads a pointer from a global variable, calls that routine, stores the result in the location pointed to by a2, and branches based on the status of d0.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `set_word_to_structure`: Saves registers a2 through a4 onto the stack, sets a2, a4, and a3 to specific constant addresses, moves a word from offset 380 of a structure to the address in a4, and then branches to another routine.
- `accumulate_scaled_values_3`: Calls accumulate_scaled_values and update_global_accumulator, then stores the result in a location pointed to by a3.
- `accumulate_scaled_values_2`: Contains a loop that performs several calculations involving global variables and calls another routine, eventually updating a pointer's target and returning a value.
- `clear_word_and_branch`: Clears a 16-bit word at a global variable and branches to another routine.
- `update_pointer_structure_2`: Calculates a value by multiplying a global variable by a constant and adding it to another global variable, calls another routine with this result, and then updates a global variable with the returned value.
- `update_global_accumulator`: Increments a global variable, updates another global variable by adding the product of a global variable and a constant to it, and then calls increment_and_execute_conditional.
- `accumulate_word_with_limit`: Adds a word from the address in a3 to a word at the address in a4, then compares the result to 300; if the result is less than 300, it returns, otherwise it clears d0 and returns.
- `update_memory_value`: Saves registers to the stack, loads three constant addresses into registers, clears a longword at offset 316 of a global variable, moves a word from offset 380 of a global variable to the address in a4, and branches to another routine.
- `decrement_and_accumulate_values_2`: Calls decrement_and_accumulate_values and accumulate_scaled_values, then stores a value in a location pointed to by a2.
- `clear_printer_registers`: Clears four longwords at offsets 120, 124, 128, and 11c of a structure, clears a word at offset 154, and then branches.
- `accumulate_printer_values`: Executes a loop that calls another routine, increments a global 16-bit word, and performs a series of conditional calls and updates to global variables and pointers.
- `execute_callback_and_store_2`: Calls a routine, stores the result in a location pointed to by a2, and branches based on the zero flag.
- `decrement_and_accumulate_values`: Increments a global variable, decrements another global variable, and calls increment_and_execute_conditional.
- `accumulate_and_check_threshold`: Adds a 16-bit value from a pointer in a3 to a 16-bit value at a pointer in a4, then compares the result against a constant; if the result is greater than or equal to that constant, it clears d0 and restores registers from the stack before returning, otherwise it returns.
- `clear_and_copy_word`: Saves registers to the stack, loads three constant addresses into registers, clears a longword at offset 340 of a global variable, moves a word from offset 380 of a global variable to the address in a4, and branches to another routine.
- `call_accumulate_scaled_values`: Pushes two 16-bit values from global variables onto the stack and calls another routine, then stores the result in the location pointed to by a3 and branches to an earlier instruction.
- `initialize_state_registers`: Clears a 16-bit word at a global offset, copies a longword from one global offset to another, copies a longword from another global offset to a third, moves a 16-bit word from a global offset to a register, and branches to another routine.
- `update_printer_state_4`: Contains a loop that calls another routine, increments a global 32-bit word, and performs conditional updates to a pointer's target and other values.
- `update_clipboard_state`: Calls another routine, increments a global 32-bit word, compares it with a 16-bit word at a global offset, and if the comparison is met, calls two more routines and updates a pointer in a2.
- `calculate_scaled_offset`: Increments a global variable and updates another global variable by adding the product of a global variable and a constant to it, then calls increment_and_execute_conditional.
- `decrement_and_check_limit`: Decrements a word at the address in a3, and if the result is not greater than 255, it increments a word at the address in a4, compares that word to 300, and either returns or clears d0 and returns.
- `reset_printer_status_and_counters`: Clears two 16-bit words in a global structure at offsets 380 and 340, updates a value at offset 320 in a global structure using a value from another location, and branches to another routine.
- `push_and_call_routine`: Pushes two 16-bit values from memory onto the stack, calls another routine, stores the result in a memory location pointed to by a3, and then branches to another location.
- `transfer_and_branch_registers`: Clears a 16-bit word at a global offset, copies a longword from one global offset to another, copies a longword from one global offset to another, copies a 16-bit word from one global offset to another, and branches to another routine.
- `reset_printer_counters`: Clears four global 16-bit words, calls another routine, and then enters a loop that performs conditional calls and updates to global variables and pointers.
- `store_callback_result`: Calls another routine, stores the returned value into a location pointed to by a register, and if the result is zero, reloads the value from that same location into a register before branching to another routine; otherwise it returns.
- `reset_printer_counters_2`: Contains a loop that increments a global variable and decrements another, performing multiple conditional calls and updates to various values and pointers.
- `decrement_and_check_counter`: Decrements a 16-bit word at a location pointed to by a3 and branches if the value is higher than 0; otherwise, it increments a 16-bit word at a location pointed to by a4, compares its value to a constant, and if the value is not lower than the constant, it clears a register and restores registers from the stack before returning.
- `accumulate_scaled_values`: Iteratively adds a doubled value to a pointer's target until the target is no longer less than one, then subtracts a doubled second parameter from that target.
- `compare_table_values`: If a global variable is zero, it compares a byte from a table using indices derived from two parameters; otherwise, it compares a parameter against a value calculated by subtracting the global variable from fifteen.
- `compare_table_value`: Performs a comparison between a byte retrieved from a table using bitwise-derived indices and a provided byte.
- `negate_boolean_results`: Calls two different routines and returns the negation of the first return value based on the status of a second return value.
- `rotate_32bit_value_4`: Performs a 32-bit bitwise rotation or division-based transformation on a value in a register using a loop.
- `rotate_32bit_value_2`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_3`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `process_pointer_chain`: Calls a routine with a pointer and a constant, then enters a loop that compares bytes from a source and destination pointer until a null terminator is reached, and subsequently performs several operations involving a global variable and a structure.
- `fetch_table_entry`: Shifts a 16-bit value in d2 left by two bits, uses the result to index into a table starting at a constant address, and loads the resulting address into a4 and d4 before branching.
- `branch_to_offset`: Performs a relative branch to a specific offset.
- `set_constant_pointer`: Loads the constant 32 into a2 and branches to another location.
- `compare_strings_and_update_structure`: Compares two strings in a loop, and if a match is not found, performs several calls to other routines to update values at offsets 56 and 5c of a structure, potentially calling a routine through a pointer at offset 1c of a calculated address.
- `manage_disk_operations`: Checks a pointer in a structure, calls a routine through a pointer, and if a result is non-zero, performs a series of operations including calling a routine through a pointer, copying a null-terminated string, and calling two other routines.
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

- `initialize_narrator_device_structures`: Allocates two memory blocks and populates them with pointers, values, and linked list links retrieved from a global variable and other routines.
- `test_and_branch_d0`: Tests the value in register d0 and, if it is not zero, sets d0 to negative 5 and branches to another routine; otherwise it branches to another routine.
- `update_device_structure_2`: Updates a structure at a6 by either modifying specific fields and calling another routine, or by incrementing counters and writing several values to a structure at a1.
- `update_device_structure`: Updates several fields in a structure at a6, calls another routine, and then either writes specific values to a structure at a2 or updates counters and fields in the structure at a6.
- `update_device_state_fields`: The routine stores an input value into a structure, increments counters in the structure, copies a counter to another structure, writes several constant words and bytes to the second structure, and clears a flag bit in the first structure.
- `write_byte_and_reset_long_value`: Stores a byte from register d0 into offset 31 of a structure in a2, stores a 32-bit value of all ones into offset 20 of the same structure, and branches to another routine.
- `clear_flag_and_branch`: Clears bit 0 of a byte at offset 30 of a structure in a1, loads a pointer from offset 20 of a structure in a1 into a3, loads a pointer from a constant address into a6, clears bit 0 of a byte at offset 30 of a structure in a1, tests bit 6 of a byte at offset 34 of the structure in a3, and branches based on the result.
- `manage_linked_list_2`: Updates a linked list by either splitting a byte into nibbles and calling itself, or by inserting a node into a list and updating a global variable.
- `update_linked_list_node_3`: If a byte comparison fails, it copies a byte into a structure, splits it into two nibbles, and calls itself recursively; otherwise, it inserts a structure into a linked list and modifies a global variable and a counter.
- `copy_byte_and_split_nibbles`: The routine copies a byte from one structure to another, splits it into low and high nibbles into two bytes, clears a byte in the destination structure, and calls another routine.
- `process_narrator_token_list`: Iterates through a linked list of tokens, decrementing a global counter and updating a structure with values from the current token including a range-checked short at offset 48, a short at offset 50, and a short at offset 64.
- `decrement_narrator_token_counter`: Decrements a byte in a global structure and, if the value reaches zero, disables interrupts by writing to the hardware register and calls sync_narrator_token_data.
- `traverse_list_and_decrement`: Traverses a linked list by following pointers, calls another routine if a value in a structure in a6 matches a value in the current list node, decrements a 16-bit counter at offset 32 of a structure in a6, and if the counter reaches zero, calls a function via a pointer in a global variable and calls another routine via a pointer in a structure in a global variable.
- `decrement_counter_and_close`: Decrements a 16-bit counter in a structure and, when it reaches zero, calls a function via a pointer in a global variable, calls another function via a pointer at an offset from that global variable, and optionally calls another routine if a bit is set in a byte at offset 34 of a structure.
- `call_table_entry`: Loads a word from offset 28 of a structure in a1, and if it is between zero and seven inclusive, uses it to index into a table to call a routine.
- `update_pointer_and_flag`: Sets bit 5 of a byte at offset 34 of a structure in a3, then branches to another routine.
- `swap_pointers_and_return_value`: The routine calls library functions via a base pointer, swaps values between a pointer table and a data word, updates a pointer field, and returns a value from a table entry.
- `dispatch_event_handler`: Processes an event structure by checking a type field and executing specific logic for different values, including setting status flags, updating a linked list of structures, and calling various handlers via pointers in a structure.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `modify_status_and_invoke_handlers`: Clears bit 5 of a byte at offset 34 of a structure in a0, sets a 16-bit value at offset 196 of a structure in a6 to seven, sets bit 0 of a byte at offset 198 of a structure in a6, and calls two routines via pointers stored in a structure in a6.
- `set_flags_and_invoke_handler`: Sets bit 6 of a byte at offset 34 of a structure in a0, writes 6 to offset 196 of a structure in a6, sets bit 0 of a byte at offset 198 of a structure in a6, calls a handler via a pointer, sets a byte at offset 31 of a structure in a1 to 0, and calls another routine.
- `set_flag_then_clear`: Sets bit 7 in a byte at offset 34 of a structure, calls two other routines, then clears bit 7 in the same byte.
- `set_flag_clear_byte_then_call`: Sets bit 7 in a byte at offset 34 of a structure, calls another routine, clears bit 7, clears a byte at offset 31 of a second structure, and calls another routine.
- `update_global_and_list`: Checks if a pointer at offset 516 of a structure in a6 is non-zero and calls another routine if it is, sets a global variable to 16384, increments a byte at offset 294 of a global variable, traverses a linked list via a pointer at offset 20 of a structure in a3 while calling another routine, decrements the byte at offset 294 of the global variable, and if the byte was less than 1, sets the global variable to 49152.
- `decrement_counter_and_set_global`: The routine decrements a byte in a global variable area and, if the previous value was less than one, writes a constant to a global variable.
- `update_pointer_and_interrupts`: Saves registers, enables interrupts by writing 16384 to the INTENA register, increments a byte at offset 294 of a structure in a6, and sets bit 5 of a byte at offset 30 of a structure in a1 if a pointer comparison fails.
- `compare_and_update_pointer`: Loads a pointer from a structure in a0, compares it with a pointer in a1, and if they are not equal, updates a pointer at offset 4 of a1 with the value from a0 and updates a pointer at offset 4 of a0 with the value from the original a0.
- `modify_interrupt_and_call_handler`: Moves a pointer from a1 to a0, updates a pointer at offset 4 of a1 with the value in a0, updates a pointer at offset 4 of a0 with the value in a1, sets bit 7 of a byte at offset 31 of a structure in a1, decrements a byte at offset 358 of a structure in a6, and if the result is not negative, sets the INTENA register to 32768 and calls another routine.
- `decrement_counter_and_set_global_2`: The routine decrements a byte in a structure pointed to by a register and, if the previous value was less than one, writes a constant to a global variable.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `set_byte_and_call_routine`: Sets a byte at offset 31 of a structure in a1 to 249 and calls another routine.
- `clear_byte_and_call_routine`: Clears a byte at offset 31 of a structure in a1 and calls another routine.
- `clear_flags_and_call_handler`: Clears a flag bit in a structure, clears a byte in another structure, and calls a handler via a base pointer.
- `sync_narrator_token_data`: Copies several 16-bit and 8-bit values from a source structure into a destination structure based on specific range checks, then performs arithmetic on a length field and enters a loop to scan a string backwards for delimiters like commas, hyphens, or spaces.
- `update_token_value_threshold`: Compares a 16-bit value in d1 to 400; if it is greater, it stores d1 into a 16-bit field at offset 30 of the structure in a5, otherwise it loads a 16-bit value from offset 50 of the structure in a2 into d1, compares it to 65, and if it is less than 65, sets d3 to 234 and branches to another routine; otherwise it returns.
- `update_token_value`: Compares a 16-bit value in d1 to 320; if it is greater, it stores d1 into a 16-bit field at offset 28 of a structure in a5, otherwise it loads a 16-bit value from offset 52 of a2 into d1.
- `compare_and_branch_token`: Stores a 16-bit value from register d1 into a structure at offset 48 of a5, loads a 16-bit value from offset 54 of a2 into d1, and branches to another routine if d1 is zero or one; otherwise, it loads the constant 233 into d3 and branches to another routine.
- `update_token_value_and_compare`: Stores a 16-bit value from register d1 into a structure at offset 38 of a5, then loads a new 16-bit value from offset 64 of a2 and compares it to 5000; if the value is greater than or equal to 5000, it branches to another routine, otherwise it sets d3 to a constant and branches to another routine.
- `compare_and_store_word`: Compares a 16-bit value in d1 to 26464; if greater, it branches to another routine, otherwise it stores d1 at offset 21688 of a5.
- `scan_token_delimiters`: Scans a string backwards from a specified position to find a delimiter such as a comma, hyphen, or space.
- `parse_narrator_token_stream_2`: Scans a string backwards from a current position to find a delimiter such as a comma, hyphen, or space, updating a structure with the new position and resetting a counter.
- `copy_longword_and_branch`: Copies a longword from offset 36 of a structure in a1 to offset 32 of the same structure and branches to another routine.
- `clear_long_and_set_byte`: Clears a longword at offset 32 of a structure in a1, sets a byte at offset 31 of that structure to the value in d0, and calls another routine.
- `parse_text_token`: Scans a string backwards from a current position to find a delimiter such as a comma, hyphen, or space.
- `calculate_and_store_offset`: Loads a longword from offset 150 of a structure in a5, performs arithmetic involving a word derived from that longword, and if a comparison with a byte in a table is successful, stores a word at offset 156 of the structure in a5 and clears the longword at offset 150.
- `shift_and_clear_value`: Loads a word from offset 150 of a structure in a5, shifts it right by one, stores the result at offset 156 of the same structure, and clears the longword at offset 150.
- `initialize_narrator_buffer_and_copy_data`: Clears several fields in two structures, performs two calls to other routines, and if a condition is met, allocates a new structure and populates it with data.
- `copy_byte_and_call_library`: Copies a longword from offset 24 of a structure in a1 to d1, stores the byte from d1 into offset 67 of a structure in a2, and calls a library function using d0 and d1 as arguments; if d0 is not 34, it branches to another routine, otherwise it sets d3 to 254 and branches to another routine.
- `initialize_speech_iorb_and_copy_data`: Sets several fields in a structure, calls two library functions, and if a routine pointer from a table is not zero, copies data from a table in a structure into a buffer using nested loops.
- `calculate_and_call_library`: Sets a byte at offset 68 of a structure in a2, calculates a value based on d0, calls a library function, and if the result is not non-zero, sets d0 to 254.
- `copy_word_table_to_buffer`: Stores a pointer in a global variable and copies 34 words from a table in a global variable to a buffer pointed to by an argument, writing each word twice per iteration, repeating for a count derived from a byte in a global variable.
- `modify_registers_and_test_bits`: Sets bit 1 of register d1, tests bit 0 of register d2, and performs bitwise AND operations on register d3 before testing bit 1 of register d4.
- `handle_io_and_free_resources`: Writes a byte and a word to a global variable, calls a routine via a global variable, conditionally calls two other routines via the same global variable, clears a word in the global variable and returns a combined value.
- `test_narrator_offset`: Tests a word at offset 206 of a structure in a5; if it is greater than zero, it branches to another location, otherwise it clears d3 and returns.
- `parse_narrator_command_string`: Parses a string in a0 to extract command codes and parameters, updating a state structure at offset 231 in a5 with various byte and word values based on the character matches and table lookups.
- `modify_register_and_branch`: Negates a word in d1, adds a constant to it, compares d0 to a constant, and branches based on the result of that comparison or performs an addition to a0 and a branch.
- `parse_narrator_command_stream_3`: Iterates through a stream of bytes, performing lookups in a control table to identify command codes and their associated parameters.
- `parse_narrator_command_stream_2`: Parses a stream of command bytes by looking up control codes in a table and updating a stack of command parameters.
- `set_bit_on_condition_3`: Compares a byte in register d1 to five and, if not equal, sets bit 5 of a byte at a location calculated from a structure in a6 and a word in d3, then branches to another routine.
- `set_bit_on_condition_2`: Compares a byte in a register to six and, if not equal, sets bit 4 of a byte at a location calculated from a structure and a register, then branches to another routine.
- `update_table_index_and_pointer`: Writes a byte from a register to a table at an index, increments the index, and if the index is at least five hundred twelve and the byte is neither one nor two, updates a pointer in a structure using a value from a different offset in that same structure before returning.
- `clear_bytes_at_indexed_offsets`: Subtracts the value at offset 146 of a structure in a5 from a1, compares a1 to the word at offset 156 of the structure in a5, and if a1 is greater than or equal to that value, it branches to another routine; otherwise it clears a byte at offset d3 of a structure in a3 and a byte at offset d3 of a structure in a6, then branches to another routine.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends immediately.
- `decrement_index_on_non_zero`: Loads a byte from a table using an index in d3, and if the byte is not zero, decrements the index in d3 and branches to another routine.
- `update_structure_elements`: Checks a bit in a value from a table, and if a condition is met, writes 4, 0, and 0 to three consecutive bytes at an offset determined by a value in d3 of three different structures, then increments d3.
- `store_word_and_set_cc`: Stores a word from register d3 into a structure at an offset from a6 and sets the carry and overflow flags.
- `process_string_and_set_bits`: Iterates through a string, and for each non-zero character, checks bits in a global table to conditionally set bit 4 of bytes at various offsets preceding the current character in the string.
- `modify_byte_conditionally`: Performs a bitwise AND on d2 with 21504, and if the result is not zero, sets bit 4 of a byte at offset -3 of a structure in a0.
- `filter_and_set_bit`: Masks a longword from a pointer in a2 and d1, then if the result is non-zero, checks if a byte at offset 2 minus 2 of a structure in a1 is equal to 48; if so, it sets bit 4 of a byte at offset 3 minus 3 of a structure in a0 before returning.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `apply_text_formatting_attributes`: Iterates through a source byte stream and applies bitwise transformations to a destination buffer based on values retrieved from a lookup table.
- `process_narrator_text_stream`: Iterates through a source buffer, performing bitwise transformations on a destination buffer based on specific control characters and a lookup table.
- `process_narrator_text_stream_2`: Parses a byte stream from a source buffer, using a lookup table to determine formatting properties for each byte.
- `calculate_offset_value`: Subtracts a word in d2 from a word in d0, subtracts 1 from the result, and branches to another location.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `match_and_patch_buffer_sequence`: Iterates through a source buffer, comparing its bytes against a target buffer; if a match is found and specific bitmask conditions are met, it writes a byte from the source buffer to the target buffer and calls another routine.
- `compare_and_patch_buffer`: Compares bytes from a source buffer against a target buffer using a series of conditional checks and bitmask tests, then optionally writes bytes from the source buffer into the target buffer.
- `increment_and_test_byte`: Increments d1 by 1, loads a byte from offset 6 of a pointer in a0, and branches to another routine if the byte is not 255, otherwise it branches to another location or returns.
- `test_bit_and_branch`: Tests bit 5 of a byte at offset 3 from a pointer in a0 and branches to another routine or a different location based on the result.
- `test_bit_with_table_lookup`: Loops reading a byte from a buffer and a word from a table indexed by a word, modifies the word based on bits in the byte, and exits when a specific bit is set or a termination condition is met.
- `compare_word_and_branch`: Loads a word from offset 154 of a structure in a5 into d7, compares it to 512, and branches to another routine or a different location based on the comparison.
- `shift_bytes_and_clear_fields`: Increments a word counter in a structure and shifts bytes backward in three buffers by a computed count, then writes a byte from a register and two zero bytes to the new ends of the buffers.
- `execute_sequential_subroutines`: Calls seven different subroutines in sequence and then clears the d0 register before returning.
- `clear_byte_if_not_set`: Loads a pointer from offset 40 of a structure in a5 into a6, compares the byte at the address in a6 to 255, and if they are not equal, loads a new pointer from offset 40 of the structure in a5 and writes 160 to offset 7 of that pointer before returning.
- `clear_byte_and_increment`: Clears a byte at offset 7 of a structure in a6, increments the pointer in a6 by 8, and branches to another routine.
- `check_data_stream_end`: Loads three pointers from a structure in a5 and a global pointer into a4, then checks if a byte at the first pointer is 255; otherwise it returns.
- `process_data_stream_4`: Iterates through a data stream using three pointers in a0, a1, and a2, performing bitwise manipulations and table lookups based on values in d0, and calls another routine to increment the pointers.
- `process_data_stream_2`: Iterates through a data stream, performing bitwise transformations and table lookups, and continues until a byte containing 255 is encountered.
- `process_data_stream`: Iterates through a data stream in a0, performing bitwise operations and data transformations based on values read from a table, and calls other routines to update pointers and set bits until a byte containing 255 is encountered.
- `test_and_set_bit`: Tests bit 5 of d1 and branches to another routine if it is clear, otherwise it sets bit 2 of a byte at the pointer in a2 and branches to another routine.
- `set_bit_on_condition`: Tests bit 5 of d1 and branches to another location if it is clear, otherwise it sets bit 2 of a byte at a pointer in a2 and branches to another location.
- `set_bit_and_increment`: Sets bit 2 of a byte at a pointer in a2, increments the pointer in a2 by 1, and branches to another routine.
- `increment_three_pointers`: Increments three pointers in a0, a1, and a2 by 1 and branches to another routine.
- `calculate_masked_value`: Loads a byte from an offset of 496 in a structure into d2, compares it to 255, and if it is not equal, masks d2 with 63 and adds the result to d0 before branching to another routine.
- `update_and_allocate_memory`: Stores a long from d0 into offset 58 of a structure in a5, shifts d0 left by 3, adds 8 to d0, stores the result in offset 54 of the structure in a5, calls a routine using a pointer from offset 4 of a system stack, and if d0 is not zero, sets the overflow flag and increments a value in a7 before returning.
- `update_structure_from_call`: Stores d0 into offset 40 of a structure in a5, checks a byte at offset 219 of that structure, and if not zero, loads a long from offset 58 into d0, calls another routine, and if d0 is not zero, stores d0 into offset 44 of the structure before returning.
- `expand_narrator_data_stream`: Reads a stream of bytes from a source buffer and uses them as indices to fetch multi-byte data structures from a table, which are then expanded into a destination buffer.
- `expand_compressed_narrator_data`: Reads a stream of bytes from a source buffer and uses them as indices to fetch multi-byte data structures from a table, which are then expanded into a destination buffer.
- `expand_compressed_data_stream`: Reads a stream of bytes from a source buffer, using each byte as an index into a table to retrieve multi-byte data structures.
- `interpolate_buffer_data_3`: Performs linear interpolation on data values within a loop, using values from a table and a global variable, until a byte containing 255 is encountered.
- `check_and_increment_pointer`: Shifts d0 left by 2, loads a longword from a table at that offset into d1, and if bit 15 and bit 11 of d1 are both set, increments a0 and branches to another routine; otherwise it branches to another routine.
- `interpolate_buffer_data_2`: Performs a weighted interpolation of bytes between two memory locations using a value from a third location, then enters a loop that processes the data until a byte containing the value 255 is encountered.
- `interpolate_buffer_data`: Performs a weighted interpolation of bytes between two memory locations using a value from a third location, then enters a loop that processes a table of data based on values read from a buffer and a structure.
- `increment_a6_and_branch`: Increments a6 by 1 and branches to another location.
- `process_voice_stream_data`: Iterates through a data stream to calculate timing and pitch values based on a lookup table, then writes these values into a structured buffer.
- `process_data_stream_6`: Loads a byte from a memory location, masks it, and enters a loop that uses a value from a table to test specific bits and either increments a0 or branches back to the start of the loop.
- `process_narrator_data_stream`: Iterates through a data stream to perform comparisons between values retrieved from tables and updates a structure in memory by zeroing out fields and setting specific bytes at offsets 5 and 11.
- `compare_table_values_3`: Loads a byte from a0 into d7, loads a byte from a table at a4 plus the offset in d7 into d2, compares d5 and d6, and if d5 is not greater than or equal to d6, it loads a byte from a0 into d7, loads a byte from the table at a4 plus the offset in d7 into d3, and branches to another routine.
- `recalculate_narrator_data_offsets`: Iterates through a data stream to compare values from multiple tables, updates a structure in a6 by zeroing out elements or setting specific bytes at offsets 5 and 11, and adjusts pointer offsets based on the results of the comparisons.
- `parse_narrator_data_stream`: Iterates through a data stream to parse structured information, performing comparisons between bytes from multiple tables and updating a series of 4-byte entries in a destination buffer.
- `calculate_offset_value_2`: Subtracts the values in d2 and d3 from the value in d0 and stores the result in d4, then sets d1 to the value in d2 and branches to another routine.
- `reset_and_copy_data`: Clears a word at the address in a6 and a byte at offset 2 of that address, then enters a loop that copies a byte from d6 to offset 3 and a word from d6 to offset 4 of the address in a6, incrementing a6 by 8 each iteration until d1 reaches zero.
- `increment_pointer_and_loop`: Increments a6 by 8, sets d1 to the value in d3, and then enters a loop that decrements d1 until it reaches zero.
- `loop_and_copy_data`: Clears a 16-bit word at the address in a6 and a byte at offset 2 of the same address, then checks bit 31 of d5; if bit 31 is not set, it copies d6 to offsets 3 and 4 of the address in a6, increments a6 by 8, and repeats the process using a loop counter in d1 before incrementing a0 and branching.
- `initialize_registers_and_branch_2`: Sets d1 to the value in d0, and sets d4 and d3 to 0, then branches to another routine.
- `filter_and_copy_triplets`: Iterates through three parallel buffers until a byte value of 255 is encountered, and for each byte that satisfies a bitwise check against a global table, it calls another routine to write the byte to the three current pointer locations, finally terminating the buffers with 255.
- `write_byte_to_three_pointers`: Writes the same byte from a register to the memory locations pointed to by three registers.
- `call_subroutines`: Calls two subroutines in succession and then returns.
- `mask_byte_in_structure`: Compares a byte at an offset of 616 from the structure in a5 to 255; if they are not equal, it performs a bitwise AND on that byte with 240 and increments the pointer, otherwise it branches to another routine.
- `check_byte_value`: Loads two pointers from offsets 232 and 744 of a structure in a5, reads a byte from the first pointer, and returns if the byte is 255; otherwise it branches to another routine.
- `update_linked_list_nodes`: Processes a sequence of bytes in a0 and a1, reading values from a table to determine bitwise updates to the bytes in a1 and calling other routines to set bits or manage node transitions until a byte containing 255 is encountered.
- `process_linked_list_nodes_2`: Sets bit 1 of a byte at offset -1 of the address in a1, then checks bit 7 of d1; if bit 7 is not set, it checks if a byte from a0 is equal to 24 or 23, and if not, it sets bit 2 of the byte at a1 and moves a1 to a3.
- `process_linked_list_nodes`: Sets bit 1 of a byte at offset 1 of the address in a1, then enters a loop that traverses a list of bytes, checking a table for specific bit flags and updating bytes in the list by setting bits 0, 1, or 2.
- `set_bit_and_branch`: Sets bit 2 of a byte at the address in a3 and branches to another location.
- `set_multiple_bits_and_branch`: Sets bit 0 of the byte at offset -2 of the address in a1, bit 1 of the byte at offset -1 of the address in a1, and bit 2 of the byte at offset -1 of the address in a1, then branches to another location.
- `expand_encoded_text_stream`: Decodes a bit-packed stream of values into a table by reading control bytes and calculating offsets and interpolated values based on a set of divisor parameters.
- `calculate_stream_offset`: Masks the lower 6 bits of d0, tests bit 0 of a byte at the address in a2 and increments a2, and if the bit is not set, shifts d0 left by 3 and adds it to a6 before branching to another routine.
- `decode_variable_length_text_stream`: Decodes a bit-packed stream of values by reading bytes, masking bits, and applying bit-shifts based on control bits, then writes the resulting values to a destination buffer.
- `parse_encoded_text_stream_2`: Enters a loop that reads a byte from a1, masks it with 63, checks bit 1 of a byte at a2, and if bit 1 is not set, shifts the value left by 3, adds it to d1, and repeats.
- `parse_encoded_text_stream`: Parses a stream of encoded data to calculate byte values via division and writes them to a destination buffer.
- `calculate_division_and_branch`: Swaps the high and low words of d5, adds d6 to d5, divides the longword in d7 by the resulting word in d5, stores the byte from d7 at offset -1 of the address in a6, decrements a1 and a2, and branches to another location.
- `update_table_with_division`: Loads a pointer and a value from a structure, divides the value by 110, and either writes the result to a table or branches to another location.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `parse_narrator_token_stream`: Iterates through a data stream using three pointers, performing bitwise flag checks on values loaded from a table and calling another routine recursively based on specific bit patterns and character values.
- `test_bit_flags`: Shifts d5 left by two, loads a word from an address in a4 plus the new d5 into d3, and branches based on the status of bit 20 or bit 25 of d3, or branches to another location after setting d6 to 24.
- `increment_pointers_and_branch`: Increments a1, a0, and a3 by 1 and branches to another location.
- `process_narrator_text_stream_4`: Decodes a compressed text stream by performing bitwise tests on control flags, calling decode_narrator_text_stream_3 to extract characters, and using compute_scaled_shifted_product to calculate offsets into a data table.
- `test_bits_and_call`: Tests bit 12 of d7 and branches if it is not set, then tests bit 9 of d7 and branches if it is not set, otherwise it loads a constant into d1, calls another routine, and branches.
- `test_bit_and_branch_3`: Tests bit 10 of d7 and branches to an earlier instruction if it is not set; otherwise it loads 38 into d1, calls another routine, and branches to another location.
- `test_bits_and_branch`: Tests bit 16 of d7 and branches to an earlier instruction if it is not set; otherwise it tests bit 2 of the byte at offset 1 of a3 and, if set, loads 27 into d1, calls another routine, and branches to another routine.
- `test_bit_and_branch_to_subroutine`: Tests bit 11 of a register and, if set, loads the value 22 into a register, calls another routine, and then branches to another location; otherwise, it branches to a different location.
- `decode_narrator_text_stream_3`: Decodes a compressed text stream by reading bytes from a source pointer, performing bitwise shifts and arithmetic adjustments based on values from a lookup table, and writing the resulting characters to a destination buffer.
- `test_status_bits`: Tests bit 1 of register d6 and bit 1 of register d7, then either calls a subroutine at offset 1e12 or branches to another location.
- `decode_narrator_text_stream_2`: Sets d1 to 22 and calls another routine, then branches to another location.
- `decode_narrator_text_stream`: Decodes a stream of compressed data by reading bytes from a source pointer, performing bitwise shifts and arithmetic adjustments based on values in a lookup table, and writing the resulting characters to a destination buffer.
- `compute_scaled_shifted_product`: Multiplies the low word of a register by a word input and returns a combined value formed from the high word of the product and the low word shifted right by five bits after adding sixteen.
- `initialize_and_clear_buffers`: Stores several offsets relative to a base pointer into a global structure, increments those pointers, clears several word fields, and then zeroes six blocks of memory by iterating a fixed count.
- `increment_and_call_routines`: Increments a 16-bit word at offset 136 of a structure in a5, then calls four subroutines if the result is not zero and the overflow flag is not set; otherwise it clears the condition code register and returns.
- `set_flag_and_store_indices_2`: Processes a stream of data by checking a table for specific bit patterns, performing arithmetic scaling on values, and writing results to a destination buffer while updating various counters in a structure pointed to by a5.
- `process_compressed_data`: Tests a byte in d1 and branches based on the result; it checks bit 7 of a byte at an offset from a4 and bit 5 of d1, then performs bitwise operations on d1 and increments a0 and a2 before loading a byte from a1 into d1 and branching to an earlier instruction.
- `decode_compressed_stream`: Decodes a stream of data by reading bytes from a source buffer and applying bitwise transformations and table lookups to write the results into a destination buffer.
- `set_flag_and_store_indices`: ORs a constant into a byte addressed by a base pointer and an index, then stores two short values derived from an index and a conditional copy into fields of a global structure.
- `process_narrator_text_stream_3`: Iterates through three synchronized streams—a command byte stream, a character stream, and a control byte stream—to modify a target buffer based on specific bit flags and character values.
- `test_and_branch_byte`: Loads a word from a table using a shifted value in d3, tests bit 19 of that word, and if set, branches to another routine; otherwise it tests the sign of d1, increments d4 if d1 is non-negative, tests bit 5 of d2, and if clear, stores the value 2 at an offset from a6 determined by d4 before branching to another routine.
- `modify_byte_via_flags`: Checks bit 4 of a byte at an offset from a pointer in a6, and if bit 5 of a byte at an offset from a6 is not set, it performs bitwise operations on that byte and writes it back to the same location.
- `update_structure_pointers`: Decrements three pointers in a0, a1, and a2, stores them into a structure at offsets 124, 132, and 128 of a5, and then branches to another location.
- `update_status_byte_2`: Ors a byte from d7 into a memory location at an offset from a4, moves d0 to d2, compares a byte at an offset from a6 with 2 and 14, and if the byte is 14, sets d1 to 255 and d7 to 16 before ors d7 into the memory location at an offset from a4 and branches to another routine.
- `update_byte_conditionally`: Checks bit 5 of a byte at an offset from a4, and if it is not set, it increments d2 and clears a byte at an offset from a6 if d2 is less than d4.
- `swap_and_clear_byte_flags`: Swaps a byte between two positions in a table addressed by a base pointer in a6 and indices in d0 and d2, clears the source byte if the positions differ, and then decrements d0 in a loop.
- `prepare_range_pointer`: Loads a pointer from an offset of a5 into a1, moves a word from d4 to d0, and then branches to another routine.
- `set_bits_in_range`: Sets the lowest bit of bytes in a range starting from an address in a register, iterating using a loop counter.
- `or_mask_into_buffer_with_early_exit`: Selects a mask based on a control byte, then ORs that mask into a series of bytes in one buffer while checking a flag in another buffer and returning early if the flag is set.
- `set_tile_and_mask`: Updates a byte in a screen buffer using a mask derived from a map byte, and optionally masks and sets a flag in another byte based on a scan leftwards in a second buffer.
- `accumulate_table_entries`: Reads a word from offset 140 of a structure in a5, then conditionally calls another routine several times based on two short fields at offsets 48 and 138, and finally adds the word to eight consecutive long words in a table pointed to by the structure at offset 92.
- `clamp_and_store_byte`: Calculates a value based on d6 and d7, clamps the result between 123 and 165, and stores the resulting byte at an address determined by a pointer in a5 and an index in d5.
- `clamp_value_and_store_char`: Clamps a short input between 125 and 165 and stores the resulting byte at an address formed from a base pointer and an index.
- `smooth_table_values`: Performs a multi-pass smoothing operation on a destination table by calculating scaled differences between values in a source table and applying them to the destination.
- `adjust_table_values_loop`: Computes the absolute difference between two bytes in a table, scales it, and adds it to a destination byte, then scans backwards through a range using flag bits to locate entries and updates a table entry based on a flag and a scaled difference.
- `compute_scaled_char_values_2`: Loops through a range of indices, performing complex arithmetic scaling and sign-adjusted additions on bytes from three different buffers based on flags in a fourth buffer.
- `scale_and_add_byte`: Adds 64 to a 16-bit value in d2, shifts it right by 7 bits, and adds the result to a byte at an offset from a3 using d0 as an index.
- `compute_scaled_char_values`: Loops over a range using d0, adds a scaled value to a byte at an offset from a2, and then copies a byte from a6 to a3 while subtracting 110 from it.
- `load_pointers_and_branch`: Loads two pointers from offsets 104 and 112 of a structure in a5 into a0 and a1, moves a word from offset 158 of the structure into d1 and then d0, and branches to another location.
- `process_audio_sample_transformations`: Iterates through a buffer to perform non-linear scaling and additive mixing on audio sample data.
- `calculate_arithmetic_offset`: Adds 64 to d2, shifts d2 right by 7, adds the result to a byte at an offset from a3, loads a byte from a3 at an offset from d1 into d2, extends d2 to a word, multiplies d2 by 65485, and branches based on the sign bit; if negative, it subtracts 64 before branching to another routine.
- `modify_byte_with_arithmetic`: Adds a value to a byte at an offset from a3, performs several arithmetic operations on d2 including a multiplication, and then branches to another location based on the sign of the result.
- `calculate_and_update_byte_2`: Enters a loop that adds a scaled version of d2 to a byte at an offset from a6, then updates d2 by reading a byte from a6 at an offset from d1, performing a multiplication, and applying a conditional subtraction before branching back.
- `calculate_and_update_byte`: Adds 64 to d2, shifts it right by 7 bits, and adds the result to a byte at an offset from a6 using d1 as an index.
- `add_byte_to_table`: Adds a byte from d2 to a byte at an offset from a2 and then branches to another routine.
- `apply_audio_sample_transformations_2_2`: Performs additive sample mixing and amplitude scaling on audio data buffers using variable coefficients determined by a control value.
- `apply_audio_sample_transformations_2`: Enters a loop that adds a scaled version of d2 to a byte at an offset from a3, then updates d2 by reading a byte from a3 at an offset from d1, extending it to a word, multiplying it by d7, and applying a conditional subtraction before branching back.
- `apply_audio_sample_transformations`: Iterates through a data buffer to perform arithmetic scaling and bitmask-based adjustments on bytes.
- `transform_audio_buffer_data`: Iterates through a data buffer using a loop to perform arithmetic transformations on bytes based on bitmask checks, applying scaling and offset adjustments to values stored in a table.
- `transform_audio_sample_buffer`: Performs non-linear scaling and additive mixing on audio sample data by iterating through a buffer and applying bitwise masks and arithmetic transformations to the byte values.
- `transform_byte_value`: Loads a byte from a buffer at an offset in d0 into d1, multiplies the sign-extended 16-bit value by 102, and then either subtracts 64 or branches to another location based on the sign.
- `update_destination_byte`: Adds a value derived from a 16-bit register to a byte in a destination buffer at an offset in d0, then branches to another location.
- `adjust_tile_heights_with_mask`: Iterates over a byte range using a mask in a global variable and a table pointed to by a register, updates a byte in a destination buffer from a source buffer with a scaled difference based on a running maximum, and writes an accumulated scaled value to another buffer.
- `load_buffer_pointers`: Loads two long pointers from offsets 120 and 104 of a structure in a5 into a2 and a0, clears d1, sets d0 to the value in d4, and branches to another location.
- `modify_buffer_with_scaling`: Iterates through a buffer backwards, checking if the high nibble of a byte is non-zero, and if so, adds a scaled value to a byte in another buffer based on the high bit of the source byte and the value of the source byte.
- `scale_buffer_by_signed_factor`: Adds a value derived from a 16-bit register to a byte in a destination buffer at an offset in d0, then loops backwards using d0 as a counter until it returns.
- `process_audio_stream_buffers`: Iterates through a data stream to perform arithmetic adjustments on multiple buffers, including subtracting values between buffers and applying bitwise manipulations based on flag bits.
- `copy_and_increment_bytes`: Iterates through a loop, copying a byte from one location to another, performing a subtraction, and then adding five to a byte at a location pointed to by a value loaded from a global variable.
- `process_audio_buffer_stream`: Iterates through a data stream to copy bytes between multiple buffers while performing bitwise manipulations and arithmetic adjustments based on nibble values and bit flags.
- `update_buffer_with_shifted_nibble`: Iterates through a loop, copying a byte from one buffer to two others, and then adds the lower nibble of a byte from a fourth buffer, shifted left by one, to a byte in the third buffer.
- `load_address_and_call_2`: Loads a 32-bit pointer from a global variable into a4, calls another routine, and branches to another location.
- `load_address_and_branch_5`: Loads a 32-bit constant into a4, calls another routine, and then branches to another location.
- `load_structure_values`: Loads a pointer from an offset of 110 in a5 into a4, stores 110 in d6, and moves the value of d5 into d0 before branching to another routine.
- `transform_and_normalize_audio_buffer`: Performs a loop to scale bytes in a buffer by their lower nibble and then processes a segment of data by calculating an average offset based on specific bit flags, applying this offset to the buffer, and optionally calculating a peak-scaled character value for a result byte.
- `process_buffer_data`: Decrements a counter in d0 while checking a bit in a buffer, then performs arithmetic on bytes in a buffer and copies them to another buffer and a third buffer while applying offsets and scaling based on a value in d2.
- `process_buffer_and_calculate_max`: Copies bytes between two buffers while subtracting a value from them, then if a byte in d2 is a backspace, it finds the maximum byte in a buffer, calculates a scaled value, and writes it to multiple locations; otherwise, it writes specific character values to a buffer.
- `initialize_registers_and_branch`: Clears registers d7 and d1, moves a 16-bit value from d4 to d0, and then branches to another location.
- `accumulate_bytes_and_write_results`: Accumulates bytes from a source area into a register, keeps a running maximum, derives a scaled character value, and writes that value and a derived value to several destination buffers with a conditional copy.
- `decrement_pointers_and_branch`: Loads three pointers from offsets 124, 128, and 132 of a structure in a5 into a0, a1, and a2, decrements each pointer by 1, sets d2 to 0, moves d4 into d0, and branches to another location.
- `apply_audio_scaling_and_mixing`: Iterates through a data stream in a loop, performing fixed-point arithmetic to scale and mix audio samples into multiple destination buffers based on flags and values found in a control structure and a global table.
- `scale_byte_value`: Loads a pointer from an offset of 96 in a5 into a6, loads a byte from a6 at an offset from d0 into d1, subtracts 110 from d1, multiplies d1 by 26, and branches based on the sign bit; if negative, it subtracts 64 before branching to another routine.
- `mix_audio_samples_with_scaling`: Iterates through a data stream to perform fixed-point arithmetic on sample values, applying scaling factors and bit-shifting to adjust samples and mixing the results into multiple destination buffers.
- `update_audio_buffer_samples`: Iterates through a data stream to perform fixed-point arithmetic on sample values, applying scaling factors and bit-shifting to adjust sample amplitudes before writing the results back to two separate memory buffers.
- `compare_and_branch_3`: Decrements d7, compares it to d0, and branches to another location if they are equal; otherwise, it sets d7 to 64 and branches to another location.
- `set_register_and_branch`: Sets d7 to 86 and branches to another location.
- `calculate_scaled_value`: Loads two pointers from a table, reads a byte from the first pointer, multiplies it by twenty-six, and if the result is negative, subtracts sixty-four from it.
- `scale_and_mix_audio_samples`: Adds 64 to a 16-bit value in d2, performs an arithmetic shift right by 7 bits, then adds the resulting byte to a destination buffer at offset d0 of a structure in a6 and subtracts it from another buffer at offset d0 of a3, looping until d0 reaches zero.
- `branch_to_next_routine`: Performs a short branch to another location.
- `copy_and_adjust_bytes_with_feedback`: Copies bytes between buffers, updates a feedback value by multiplying it with a table entry and shifting, subtracts the feedback from a byte, and writes adjusted bytes to two output areas.
- `check_narrator_flag_and_dispatch`: Calls another routine twelve times and then checks if a byte at offset 219 of a structure in a5 is non-zero; if it is, the routine calls another routine before returning.
- `call_subroutine_three_times`: Loads a pointer from an offset of 40 in a5, adds 3, 4, and 5 to that pointer respectively, and calls another routine after each addition before returning.
- `interpolate_and_store_bytes`: Reads two bytes from a buffer, computes a scaled difference based on an index, accumulates the result, and stores successive byte results into the buffer at regular intervals.
- `increment_and_check_byte`: Adds 8 to d1, loads a byte from a table at offset d1 in a0 into d6, and returns if d6 is equal to 255; otherwise it branches to another location.
- `check_conditions_and_increment`: Compares d6 to 254 and branches if equal; otherwise, it compares d1 to 8 and, if not equal, branches to another location; if d1 is 8, it adds 8 to a0 and branches to another location.
- `call_interpolation_and_branch`: Calls another routine and then branches to another location.
- `scan_table_for_sentinel`: Adds 8 to d1 and checks if the byte at offset d1 in a0 is zero or 255; if either condition is met, it branches to another location, otherwise it returns.
- `scan_buffer_incrementally`: Compares d1 to 8 and, if not equal, adds 8 to a0 and loops back to the start of the routine.
- `scan_buffer_for_sentinel`: Increments an index by 8 in a loop to scan a buffer for a null byte, then checks if the byte at that index is -1 to determine whether to return or continue scanning.
- `process_data_stream_5`: Iterates through a data buffer until a byte of 255 is encountered, using values from a second buffer to index a table and update a pointer or modify data at that pointer.
- `process_data_stream_3`: Performs a loop that processes data from a buffer in a0 by checking bits in a table, conditionally increments values in the buffer, and handles character-based expansion by copying bytes from a source table into the buffer using nested loops.
- `offset_pointer_and_branch`: Shifts a 16-bit value in d0 left by three, adds it to a pointer in a0, and then branches to another location.
- `shift_and_compare_value`: Performs a logical shift right by 2 bits on d3, compares d3 to 42, and if equal, sets d3 to 96 before branching to another location; otherwise it branches to another location.
- `update_register_value`: Compares d3 to 43 and, if equal, sets d3 to 97 and branches to another location; otherwise it branches to another location.
- `replace_byte_and_branch`: Compares d3 to 44 and, if not equal, branches to another location; if equal, it sets d3 to 98 and branches to another location.
- `copy_bytes_from_table`: Copies six bytes from a table located at an offset determined by a byte in a register to a destination pointed to by a register, then returns.
- `initialize_status_pointers`: Loads three addresses into a0, a1, and a2 from offsets 232, 776, and 488 of a5, loads a pointer from an offset of 40 in a5 into a6, sets d6 to 12288, and sets d4 to 240, d5 to 255, and d7 to 63 before branching to another routine.
- `set_bit_and_compare_byte`: Sets bit 31 of d4, clears d0 and d2, loads a byte from the address in a0 into d0, and compares d0 to d5; if they are not equal, it branches to another location, otherwise it returns.
- `update_bitmask_from_buffer_2`: Iterates through a buffer and performs bitwise operations on a structure pointed to by a6, either copying blocks of 8 bytes or modifying specific bits within elements of the structure based on values read from the buffer and a lookup table.
- `update_bitmask_from_buffer`: Iterates through a buffer, performing bitwise AND and OR operations on specific bytes within a structure pointed to by a6 based on values read from the buffer.
- `calculate_array_index_offset`: Shifts the value in register d2 left by 3 bits and adds the result to the address in register a6, then branches to another routine.
- `replace_bytes_with_lookup`: Iterates through a buffer pointed to by a pointer at offset 40 of a structure in a5, replacing bytes with values from a global table based on their content, skipping 8 bytes when a marker of 254 is encountered, and stopping when a marker of 255 is found.
- `copy_bytes_with_lookup`: Copies bytes from a source pointed to by a0 to a destination using a lookup table at a1, incrementing a0 and updating d0 in a loop.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `compare_scaled_byte`: Loads a byte from a structure in a6 at an offset determined by shifting the value 6 left by 3 bits, compares it to 255, and branches to another routine if they are not equal.
- `update_scaled_value`: Performs a loop that adds a value from a structure at offset 24 in a6 to d0, shifts the result, and writes it back to that same offset.
- `compare_table_byte`: Loads a byte from a table using an index derived from the value six, then compares this byte to a constant value of two hundred fifty-five; if they are not equal, it branches to another routine, otherwise it returns.
- `process_data_table`: Iterates through a table, performing arithmetic on a value in d0 and updating a byte at offset 24 of a structure in a6 based on a calculation involving other table entries.
- `process_byte_stream`: Loads several pointers into a0, a2, a4, and a6 from offsets of a5 and a global address, clears d2 and d0, loads a byte from the address in a0 into d0, and if d0 is not 255, branches to another location; otherwise it returns.
- `process_command_stream`: Iterates through a stream of bytes, performing arithmetic on a byte at offset 7 of a structure in a6 based on the character value, and continues until a byte with value 255 is encountered.
- `add_constant_to_structure`: Runs a loop that adds the constant 10 to a byte at offset 7 of a structure in a6, decrementing d2 each iteration.
- `load_constant_and_branch`: Loads the constant 250 into d3 and branches to another routine.
- `load_negative_constant_and_branch`: Loads the value negative 6 in register d3 and branches to a location within another routine.
- `load_negative_constant_and_branch_2`: Loads the constant negative 6 from a small immediate value into register d3 and branches to a location within another routine.
- `write_fixed_byte_loop`: Runs a loop that writes the byte value 230 to offset 7 of a structure in a6, decrementing d2 each iteration.
- `modify_structure_byte`: Runs a loop that reads a byte from a6, subtracts 43, performs an arithmetic shift right by 2, and adds the result to offset 7 of the structure in a6.
- `decode_nibble_checksum_bytes`: Reads a pointer and a count from a structure pointed to by A5 and processes a byte array in two passes.
- `wait_for_signals`: Calls three different functions via pointers stored in a global variable to retrieve values, stores these values at offsets 48 and 49 of a structure in a0, sets a bitmask at offset 776 of the structure, and enters a loop that calls another routine and a fourth function indefinitely.
- `manage_event_status`: Enters a loop that checks bit 6 of a byte at offset 34 of a structure in a2; if set, it returns, otherwise it sets bit 0 of that byte, calls another routine, and if that routine returns zero, it clears bits 0 and 1 of the byte at offset 34 and returns; otherwise it updates a value at an offset of 772 in a3 and sets bit 1 of the byte at offset 34 before calling itself recursively.
- `clear_status_bits`: Clears bit 1 and bit 0 of a byte at offset 34 of a structure pointed to by A2.
- `prepare_speech_playback_parameters`: Populates a playback structure with timing and bitmask values derived from a byte stream, copies a block of data from a source buffer, and executes a callback function.
- `prepare_audio_playback_parameters`: Copies a block of data from a source buffer, populates a structure with timing and bitmask values derived from a byte stream, and executes a callback loop through a function pointer in a global variable.
- `decode_narrator_audio_stream`: Iterates through a byte stream to perform bitwise operations and lookups in a table, writing the resulting values to a destination buffer.
- `parse_narrator_command_stream`: Iterates through a stream of bytes to decode command parameters and executes a series of function pointers stored in a table when a null terminator is encountered.
- `decode_audio_stream_data`: Iterates through a byte stream using nested loops to perform bitwise manipulations and table lookups, writing the resulting values to a destination buffer.
- `decode_narrator_command_stream`: Iterates through a byte stream to decode command parameters, updates a structure with character data and bit-shifted values, and calls process_narrator_command_stream, enable_dma_if_flag_set, manage_interrupt_state, or calculate_offset_and_branch_2.
- `process_narrator_command_stream`: Iterates through a byte stream to decode command parameters, updates a structure with character data and bit-shifted values, and executes a series of function pointers from a table until a null terminator is reached.
- `decrement_and_branch_2`: Decrements register d0, branches if the result is positive, otherwise writes the value from d5 to offset 16 of a structure in a5 and branches to another routine.
- `execute_callback_loop`: Checks a bit in a structure at offset 210 in a5, potentially enables DMA by writing to a hardware register, and then calls a function pointer from a global variable in a loop.
- `enable_dma_if_flag_set`: Tests a flag byte in a structure pointed to by A5 and, if set, writes a value to a custom hardware register that enables DMA channels by setting bit 15 of DMACON.
- `manage_interrupt_state`: Updates a byte at offset 358 in a6, compares values in several structures, and calls a function pointer from a structure in a6 while managing a global variable and interrupt enable bits.
- `decrement_counter_and_enable_interrupts`: Reads a byte from an offset in a structure pointed to by A6, decrements it and writes it back.
- `calculate_offset_and_branch_2`: Loads a byte from offset 68 of a structure in a3, multiplies it by 68, and if a word at offset 22 of a structure in a5 is not zero, calculates a new address by adding the result to a pointer from offset 60 of a structure in a4 before branching.
- `execute_callback_loop_2`: Loops to call a routine via a pointer at offset 22 of a structure, setting bit 1 of a byte at offset 30 of a structure in a3, then performs conditional logic involving a second routine call and updating a global value.
- `loop_and_set_bit`: Decrements a counter in a register and loops to call a routine repeatedly, setting bit 5 of a byte at offset 30 of a structure in a3 if a byte at offset 31 of a structure in a3 is non-zero, otherwise it writes a value to a hardware register and returns.
- `set_bit_and_branch_2`: Sets bit 5 of a byte at offset 30 of a structure in a3 and branches back to an earlier instruction.

### Qume_LetterPro_20

- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `execute_triple_call_2`: Calls a routine at an offset from a global variable three times, and if all three calls return non-zero, it calls another routine with a global variable as an argument and returns 0, otherwise it calls that routine and returns -1.
- `call_function_triple`: Calls the same function pointer three times and returns a value of negative one.
- `execute_handler_sequence`: Calls a routine through a pointer located at a specific offset in a global variable, then calls it twice more using the value from A6 as an argument.
- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_parameters_2`: Clears registers d1, d2, and d3, loads three pointers from the stack into a0, a1, and a4, and branches to another routine if the word at the pointer in a0 is not equal to 1.
- `parse_printer_command_sequence`: Copies a sequence of bytes from a table into a destination buffer and then processes a command byte and its parameter to update the buffer with control codes or formatted ASCII characters.
- `copy_table_byte`: Copies a byte from a table at offset zero using a value in D0 to a destination address calculated from A3 and A4, then branches to another routine.
- `update_threshold_values`: Compares a word at the address in a0 to 27; if it matches, it checks if the byte at the address in a1 is not zero, and if so, writes 32 to the address in a0 and 1 to the address in a1, then branches to another location.
- `check_threshold_and_update`: Compares a word at the pointer in a0 to 27; if it matches, it checks if the byte at the pointer in a1 is less than zero, and if so, writes 4 to the pointer in a0 before branching to another routine.
- `update_status_if_threshold_met`: Compares a word at a pointer in a0 to 28; if it matches and a byte at a pointer in a1 is greater than zero, it writes 33 to the pointer in a0, then returns.
- `update_status_values_3`: Compares a word at a pointer in a0 to 29; if it matches and a byte at a pointer in a1 is non-zero, it writes 33 to the pointer in a0 and 255 to the pointer in a1, then branches.
- `update_printer_status_word_2`: Compares a word at a pointer in a0 to 29; if it matches, it checks if a byte at a pointer in a1 is greater than 0, and if so, writes 2 to the word at the pointer in a0 before branching to another routine.
- `update_printer_status_word`: Compares a word at a pointer in a0 to 30; if it matches, it checks if a byte at a pointer in a1 is less than 0, and if so, writes 32 to the word at the pointer in a0 before branching to another routine.
- `update_word_and_byte`: Compares a word at the address in a0 to 31; if it matches, it checks a byte at a1, and depending on whether that byte is less than or equal to or greater than or equal to zero, it writes 33 or 20 to the address in a0, otherwise it writes 32 to the address in a0 and clears the byte at a1.
- `increment_byte_on_match`: Compares a word at the address in a0 to 32; if it is not equal, it branches to another routine, otherwise it increments the byte at the address in a1 and branches to another routine.
- `compare_and_decrement`: Compares a word at the address in a0 to 33; if not equal, it branches to another routine, otherwise it decrements the byte at the address in a1 and branches to another routine.
- `format_status_string`: If a word at a pointer in a0 is 57, it writes two bytes and two calculated character bytes to a destination address, otherwise if the word is zero, it writes the byte 253 to offset 680 of a structure in a1.
- `add_constant_to_byte`: Adds 48 to a sign-extended word in D0 and stores the resulting byte at a destination address calculated from A3, then branches to another routine.
- `write_status_byte`: Checks if a word at the address in A0 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in D1.

### CBM_MPS1000

- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `verify_library_function_availability`: Attempts to call three consecutive functions at specific offsets from a library base pointer stored in a global variable, returning zero if all three calls succeed or a failure code otherwise.
- `call_library_twice`: Saves A6 to the stack, calls a routine at an offset from a pointer in a global variable, restores A6, calls another routine at an offset from a different global pointer, and then branches to another routine.
- `execute_indirect_function_calls`: Fetches a function pointer from a global variable using an offset and calls that function three times in succession.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_2`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters, sets a byte to zero at offset 302, and executes three routines through a global base using a pointer retrieved from offset 300 of that same structure.
- `manage_printer_buffer_and_control`: Acts as a multi-purpose state machine that manages a printer buffer by clearing memory, writing control sequences, and performing bitwise operations on data.
- `restore_registers_and_return`: Clears register d0 and restores several registers from the stack before returning.
- `initialize_printer_parameters`: Saves registers to the stack, loads three pointers from the stack into a2, a4, and a3, clears registers d1, d4, and d5, sets d3 to 3, and branches to another routine if the word at the pointer in a2 is not equal to 1.
- `format_printer_command_buffer_2`: Copies a sequence of bytes from a table into a destination buffer, inserting specific control characters based on values read from a structure pointed to by d1.
- `modify_buffer_byte`: Loads a pointer from a location pointed to by d1, compares a word at offset 134 of that pointer to 2048, and if they match, writes 15 to a byte at a calculated address using d4 and a4, then branches.
- `process_printer_command_stream`: Copies a null-terminated string from a source table to a destination buffer using a loop, then evaluates a command byte to either write specific data sequences, update status bytes, or modify values in a structure based on various conditional checks.
- `copy_data_loop_2`: Increments registers d4 and d5, then copies bytes from a table at offset 0 to a destination address calculated from a0 and a4 in a loop that runs until d5 reaches 4.
- `modify_status_and_byte`: Compares a word at the address in a2 to 66; if it matches, it writes 1 to a byte at offset 2, loads a pointer from a0 into a1, compares a word at offset 1286 of the address in a1 to 2048, and if it is not equal, writes 96 to the address in a0 before branching.
- `write_byte_and_branch`: Loads a pointer from the address in a0 into a1, compares a word at offset 138 of the address in a1 to 1024, and if it is not equal, branches to another routine; otherwise it writes the byte 137 to the address in d3 and branches to another routine.
- `set_byte_and_branch_3`: Moves the address in d3 to a0, writes the byte 80 to the address in a0, and branches to another routine.
- `copy_bytes_from_table_2`: Copies a sequence of bytes from a table into a destination memory location using an incrementing offset and a counter, looping until the counter reaches four.
- `modify_word_and_byte`: Compares a word at the address in a2 to 32; if it matches, it checks a byte at a3, and if that byte is not zero, it writes 1 to the byte at a3 and writes 27 to the word at a2, otherwise it returns.
- `clear_byte_and_set_word`: Tests a byte at a pointer in a3; if it is negative, it clears that byte and writes the word 30 to a location in a2, otherwise it branches to another routine.
- `branch_with_constant`: Sets D0 to 255 and branches to another routine.
- `update_values_on_match`: Compares a word at the address in a2 to 33; if it is not equal, it branches to another routine; otherwise it checks a byte at the address in a3, and if that byte is not zero, it writes the byte 255 to the address in a3, writes the word 29 to the address in a2, and branches to another routine.
- `test_and_clear_byte`: Tests a byte at the address in a3; if it is less than or equal to zero, it clears the byte at that address, writes the word 28 to the address in a2, and branches to another routine.
- `branch_to_offset_2`: Performs a short branch to a specific offset.
- `update_status_from_value`: Compares a byte at an address in A2 against several constants to write specific bytes to a destination address in A3 or the stack, and if the value is zero, writes 253 to offset 2720 of a structure in D1.
- `write_status_byte_alt`: Checks if a word at the address in A2 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in D1.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_630

- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_library_function_pointers`: Attempts to call three different functions at a specific offset from a pointer stored in a global variable, returning zero if all three calls succeed; otherwise, it calls a cleanup routine at a different offset from the same pointer and returns a value of negative one.
- `verify_library_functions`: Calls two different routines using pointers from a structure, passing a value from a global variable in a1, and then sets register d0 to negative 1 before branching to another routine.
- `execute_triple_handler`: Calls a routine through a pointer found at an offset in a global variable, then calls that same routine twice more using the value from register a6 as an argument.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_command_parameters`: Loads three pointers from the stack into registers, clears three registers to zero, and compares a word at the first pointer to one; otherwise it branches to another routine.
- `parse_input_command_string`: Copies a string from a structure into a local buffer, applying specific control characters based on values read from offsets 1658, 1654, and 1653.
- `update_state_and_copy_string`: Copies a null-terminated string from a table into a destination buffer using a loop, then performs a series of conditional checks on a word and a byte to update the word to specific values and modify the byte.
- `copy_byte_from_table`: Copies a byte from a table at offset zero of a pointer in d3 to a destination address calculated from d2 and a4, then loops while d3 is greater than 11.
- `update_value_on_match_2`: Compares a word at the pointer in a0 to 27; if it matches, it checks if the byte at the pointer in a1 is negative, and if so, writes 32 to the pointer in a0 and 1 to the pointer in a1, then branches.
- `update_value_on_match`: Compares a word at the pointer in a0 to 27; if it matches, it checks if the byte at the pointer in a1 is negative, and if so, writes 4 to the pointer in a0.
- `write_status_word`: Compares a word at a pointer in a0 to 28; if it matches and a byte at a pointer in a1 is greater than 0, it writes 33 to the pointer in a0 and branches.
- `update_status_word_and_byte`: Compares a word at a pointer in a0 to 29; if it matches and a byte at a pointer in a1 is greater than 0, it writes 33 to the pointer in a0, writes 255 to the pointer in a1, and branches to another routine; otherwise it returns.
- `set_status_word`: Compares a word at a pointer in a0 to 29; if it matches and a byte at a pointer in a1 is greater than 0, it writes 2 to the pointer in a0 and branches.
- `update_value_based_on_sign`: Compares a word at a pointer in a0 to 30; if it matches, it checks the sign of a byte at a pointer in a1 and, if the byte is negative, writes 32 to the pointer in a0, otherwise it returns.
- `update_counter_and_status`: Compares a word at the pointer in a0 to 31; if it matches, it checks a byte in a1 to either write 33 to the pointer in a0 or write 32 to it, then either clears the byte in a1 or branches to another routine.
- `compare_and_increment`: Compares a word at the pointer in a0 to 32; if it does not match, it branches to another routine, otherwise it increments a byte at the pointer in a1 and branches to another routine.
- `decrement_byte_and_branch`: Compares a word at the pointer in a0 to 33; if it does not match, it branches to another routine, otherwise it decrements a byte at the pointer in a1 and branches to another routine.
- `write_conditional_bytes`: If a word at a pointer in a0 is 12, it writes 27 to a destination address and then writes either 65 or 66 to the next byte depending on a stack value, otherwise if the word is zero, it writes 253 to offset 680 of a structure in a1.
- `write_constant_byte`: Writes the byte 66 to a destination address calculated from d2 and a4, then branches to another routine.
- `write_status_byte_to_structure`: Checks if a word at the pointer in a0 is zero; if it is, it writes the byte 253 to an offset of 2720 in the structure pointed to by the value in d1.

### ImagewriterII

- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_library_function_pointers_2`: Attempts to call three consecutive functions at specific offsets from a pointer stored in a global variable, calling a cleanup routine if any of the calls return zero.
- `execute_library_calls_6`: Loads a value from a global variable into a1, calls a routine using a pointer from a structure, then loads a different value into a1 and calls another routine using a pointer from a structure before branching to another routine.
- `execute_indirect_function_calls_2`: Retrieves a function pointer from a global variable using a negative offset and calls that function three times in succession.
- `return_constant_20`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_21`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_3`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine through a pointer retrieved from the structure.
- `POSDIV`: Performs long division on a 32-bit value using a divisor in d2, returning the quotient and remainder in d1 and d0 respectively.
- `LONGDIV`: Performs a 32-bit division by rotating a 32-bit value left bit by bit for sixteen iterations, subtracting a value from a register if the high half meets or exceeds a threshold and incrementing the low half.
- `ulmult`: Computes the 32-bit product of two 32-bit values stored in two registers using 16-bit partial products and returns the result.
- `ulmodt`: Moves the contents of register d1 into d2, moves the contents of register d0 into d1, and calls POSDIV.
- `uldivt`: Moves the contents of register d0 into d1, calls another routine, and returns the resulting value from d1 in d0.
- `lmodt`: Calls POSDIV and negates the returned value if the value in an address register is nonzero.
- `ldivt`: Calls POSDIV and returns the quotient in d1, or the negation of the quotient if the remainder in a0 is nonzero.
- `dispatch_via_table`: Saves registers to the stack, reads a byte from an offset of the stack pointer, and if that byte is not greater than 4, uses it as an index into a table to jump to another routine.
- `clear_d0_and_restore_registers_alt`: Clears register d0 and restores d2 through d5 and a2 through a4 from the stack before returning.
- `initialize_printer_state`: Saves registers d2 through d4 and a2 through a4 to the stack, loads a pointer from offset 28 of the stack into a1, loads a 32-bit value from offset 32 of the stack into d2, loads a pointer from offset 36 of the stack into a3, clears d3, d4, and a4, and branches to another routine if the 16-bit word at the pointer in a1 is not equal to 1.
- `generate_imagewriter_escape_sequences`: Copies a sequence of bytes from a table into a destination buffer and appends various escape sequences based on the values found at specific offsets within a structure pointed to by a register.
- `translate_printer_control_byte`: Reads a control byte from a pointer in a1 and writes corresponding escape sequences to a buffer in a2 and a3.
- `generate_epson_esc_commands`: Evaluates a control byte from a pointer in a1 and writes corresponding escape sequence bytes to a buffer pointed to by a2 and a3.
- `modify_byte_and_word_2`: Compares a word at the pointer in a1 to 31; if it matches, it checks if the byte at the pointer in a3 is greater than or equal to zero, and if so, writes 33 to the pointer in a1 and 1 to the pointer in a3, then branches.
- `modify_byte_and_word`: Checks if the byte at the pointer in a3 is negative; if so, it writes 32 to the pointer in a1, writes 255 to the pointer in a3, and branches to another routine.
- `branch_to_address_2`: Branches to a specific address.
- `translate_imagewriter_escape_sequences_2`: Translates input bytes into ImageWriter escape sequences by writing specific control characters and command strings to a destination buffer.
- `write_four_bytes`: Writes the bytes 66, 27, 108, and 48 sequentially to a destination address calculated from d2 and d3.
- `translate_imagewriter_escape_sequences`: Reads a byte from a source pointer and, based on its value, writes specific escape sequences to a destination buffer to control an ImageWriter II printer.
- `write_byte_to_calculated_address`: Sets an address register using a value from a register, increments a second register, adds the two to form a destination address, writes the byte 66 to that address, and branches to another routine.
- `translate_printer_escape_sequences`: Reads a byte from a source pointer and translates it into specific escape sequences for an ImageWriter II printer, writing the resulting bytes to a destination buffer.
- `write_byte_at_offset`: Writes the byte 80 to a destination address calculated from d2 and d3, then branches to another routine.
- `emit_printer_escape_sequences`: Processes input bytes to write specific escape sequences into a destination buffer, including handling character set changes, printer modes, and decrementing a counter to emit numeric escape codes.
- `write_byte_at_offset_70`: Writes the byte 112 to a destination address calculated from d2 and d3, then branches to another routine.
- `process_escape_sequences`: Checks a byte at the address in a1 and writes various escape sequences to a destination buffer in d2 based on the value, including handling specific control characters and updating a counter in a stack variable.
- `emit_escape_sequences`: Processes a byte from a pointer in a1 to write escape sequences to a destination buffer in d2, handling specific control characters by writing a sequence of bytes and updating a global variable and a counter.
- `write_byte_conditionally`: Compares a byte at a memory location to 1, and if it is not equal, branches to another routine; otherwise, it calculates a destination address from two registers, increments one of them, and writes the byte 80 to that address before branching to another routine.
- `update_printer_status_byte`: Increments a value in a register, calculates a target address by adding two registers together, and writes the byte value one hundred twelve to that address before branching to another routine.
- `emit_imagewriter_escape_sequences`: Writes escape sequences to a destination buffer based on the value at a pointer in a1; it handles specific control characters by writing an escape byte followed by varying command bytes or ASCII digits derived from a decrementing counter in a stack location.
- `write_byte_conditionally_offset`: Compares a byte at a memory location to 1, and if it is not equal, branches to another routine; otherwise, it calculates a destination address from two registers, increments one of them, and writes the byte 69 to that address before branching to another routine.
- `write_byte_at_offset_4e`: Writes the byte 78 to a destination address calculated from d2 and d3, then branches to another routine.
- `update_printer_state`: Checks a word at the address in a1; if it is 48, it updates a value in a stack location and writes bytes to a destination calculated from d2 and d3, otherwise if it is 12, it performs conditional updates to the stack and writes bytes to the destination.
- `check_and_modify_memory_values`: Compares a word at a memory location to 12, and if not equal, branches to another routine; otherwise, it performs several checks on values at an address stored on the stack, writing the byte 30 to a location if a byte is 39, and writing the byte 30 to a location if a byte is 37 and is not less than or equal to 37.
- `write_string_data`: Writes the bytes 27, 75, and a value from a global table plus 48 to a destination address calculated from d2 and d3.
- `clear_d0_and_restore_registers_2`: Clears register d0 and restores d2 through d4 and a2 through a4 from the stack before returning.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_C-150

- `return_constant_22`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_library_function_pointers_3`: Attempts to call three consecutive functions at a specific offset from a pointer stored in a global variable, calling a cleanup routine if any of the calls return zero.
- `verify_library_functions_2`: Loads a library base from a global variable, calls a function at an offset of minus 614 from that base, restores the previous base, loads another library base, calls a function at an offset of minus 614 from that base, restores the previous base, and sets d0 to 255 before branching.
- `execute_indirect_function_pointer`: Fetches a function pointer from a global variable using a negative offset and calls that function three times in succession.
- `return_constant_23`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_24`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_4`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes four routines through pointers found in a global base and another structure.
- `divide_32_bit_integers`: Performs a 32-bit division by a 16-bit value using a loop that rotates bits and subtracts a threshold, or performs a modulo operation if the divisor is 16-bit.
- `rotate_and_threshold_32bit_value`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves, and subtracts a value from the high half if it meets or exceeds a threshold stored in a register.
- `multiply_32_bit_integers_4`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components and returns the result.
- `negate_string_length_on_flag`: Calls measure_string_length and returns its result, or returns the negation of that result if the address register returned by the called routine is non-zero.
- `dispatch_via_table_2`: Loads several values from the stack, performs a comparison on a byte from the stack, and then jumps to an address retrieved from a table using an offset derived from the stack value.
- `return_zero`: Sets d0 to zero and returns.
- `handle_data_stream`: Processes data based on the value at the pointer in param_1, which includes writing bytes to param_2 and param_4, updating values in param_6, and calling another routine.
- `copy_byte_with_loop`: Copies a byte from a source address calculated from d1 and a constant to a destination address calculated from d3 and d0, then branches based on a comparison between d2 and d5.
- `update_calculated_value`: If a 16-bit value at a pointer in a3 is 12, it modifies a value on the stack based on its current content and then writes a 4-byte sequence to a destination address using values from a global table; otherwise, if the value is 0, it writes 253 to offset 2720 of a structure in a4.
- `write_calculated_byte`: Calculates a value using a combination of a global variable and a stack value, writes the byte 27 to a specific offset and the byte 64 to the next offset, then writes a byte from a table at an index derived from the calculated value to a fourth offset, and finally calls another routine to write a byte from a table at an index derived from that routine's return value.
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
- `execute_triple_call_3`: Performs up to three consecutive calls to routines located at offsets from a pointer in a global variable, and if any call returns a non-zero value, it calls another routine using the global variable as an argument.
- `execute_triple_call_2_sequence`: Loads a pointer from a global variable into a1, then performs two consecutive calls to a routine at an offset from a pointer stored in a global variable, using the stack to preserve a6.
- `execute_library_calls`: Calls a routine at an offset from a global base three times, passing a value from a6 to the second and third calls.
- `return_constant_29`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_30`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_31`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_parameters_3`: Sets registers a0, a1, a4, d1, d2, and d3 to zero, loads a word from a pointer at offset 32 of the stack into a0, and branches to another routine if the word is not one.
- `process_printer_control_sequence`: Copies a sequence of bytes from a source buffer into a destination buffer, applying specific byte transformations based on values read from a structure.
- `process_printer_control_sequence_2`: Copies a null-terminated string to a destination buffer and then processes a series of control characters by updating a status byte and a secondary value based on specific character matches.
- `copy_byte_with_loop_alt`: Copies a byte from a constant address to a destination address calculated from a pointer in d2 and a pointer in a4, then branches based on a comparison between d0 and d3.
- `update_status_value`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is negative, it writes 32 to the address in a0 and 1 to the address in a1, then branches to another routine.
- `update_status_on_threshold_2_2`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is negative, it writes 4 to the address in a0 and branches to another routine.
- `update_status_on_threshold_2`: Compares a word at the address in a0 to 28; if it matches and the byte at the address in a1 is not negative, it writes 33 to the address in a0 and branches to another routine, otherwise it returns.
- `update_status_on_threshold`: Compares a word at the address in a0 to 29; if it matches and the byte at a1 is not negative, it writes 33 to the address in a0, writes 255 to the address in a1, and branches to another routine; otherwise it returns.
- `update_word_if_greater`: Compares a word at the address in a0 to 29; if it matches and the byte at a1 is greater than or equal to zero, it writes 2 to the address in a0, otherwise it returns.
- `update_word_conditionally`: Compares a word at the address in a0 to 30; if it matches and the byte at a1 is less than zero, it writes 32 to the address in a0, otherwise it returns.
- `update_value_based_on_range`: Compares a 16-bit value at a pointer in a0 to 31; if it is not 31, it branches.
- `increment_on_match`: Compares a 16-bit value at a pointer in a0 to 32; if it is not 32, it branches.
- `decrement_and_branch_3`: Compares a 16-bit value at a pointer in a0 to 33; if it is not equal, it branches, otherwise it decrements the byte at the pointer in a1 and branches.
- `write_conditional_bytes_2`: If a word at a pointer in a0 is 12, it writes the byte 27 to a destination address and then writes either 65 or 66 to the next byte depending on a stack value; otherwise, if the word is 0, it writes the byte 253 to an offset of 2720 of a structure pointed to by a pointer in d1.
- `write_constant_byte_2`: Writes the byte 66 to a destination address calculated from a pointer in d2 and a pointer in a4, then branches.
- `write_status_byte_to_structure_2`: Checks if a word at the pointer in a0 is non-zero; if it is zero, it writes the byte 253 to an offset of 2720 in the structure pointed to by the value in d1, then returns.

### HP_LaserJet

- `return_constant_32`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_calls_5`: Calls a routine at an offset from a pointer in a global variable three times, and if all calls return non-zero, it calls another routine at an offset from the same global variable; otherwise, it calls that routine twice and returns -1.
- `test_library_function_offsets`: Loads a library base pointer from a global variable and calls two functions at specific offsets from that pointer, returning negative one if both calls succeed.
- `execute_indirect_function_calls_3`: Fetches a function pointer from a global variable using a negative offset and calls that function three times consecutively.
- `return_constant_33`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_5`: Checks a condition via a routine call, and if met, populates a structure with two parameters and a constant, then executes three routines through a global base and one routine through a pointer stored in the structure.
- `process_hp_laserjet_control_codes`: Manages printer state and command buffering by copying bytes from a global table into a destination buffer and calling format_hp_laserjet_command_buffer, update_printer_state_and_buffer, or convert_to_ascii_2 based on specific control code values.
- `format_hp_laserjet_command_buffer`: Copies bytes from a global table into a buffer, modifies specific bytes based on values read from a printer state structure, and calls another routine to output the resulting data.
- `update_printer_state_and_buffer`: Updates a printer state structure by copying bytes from a global table, modifies a character buffer based on control codes, and calls write_status_word and generate_epson_esc_commands to manage printer command sequences.
- `copy_data_and_call`: Copies bytes from a global table to a destination buffer in a loop, then calls two different routines using addresses and values derived from a structure in a6.
- `write_value_on_condition`: Compares a 16-bit value at a pointer in a2 to 27; if it is not 27, it branches.
- `check_escape_sequence`: Compares a word at the address in a2 to 27; if it does not match, it branches to another routine; if it matches, it tests the byte at the address in a3, and if that byte is less than zero, it writes the word 4 to the address in a2 and branches to another routine.
- `update_word_on_match_2`: Compares a word at the address in a2 to 28; if it matches and the byte at the address in a3 is greater than zero, it writes 33 to the address in a2 and branches to another routine.
- `set_word_and_byte`: Compares a word at the address in a2 to 29; if it matches and the byte at a3 is not zero, it writes 33 to the address in a2 and writes 255 to the address in a3, then branches.
- `update_word_on_match`: Compares a word at the address in a2 to 29; if it matches and the byte at a3 is greater than zero, it writes 2 to the address in a2, otherwise it returns.
- `update_word_on_match_alternate`: Compares a word at the address in a2 to 30; if it matches and the byte at the address in a3 is less than zero, it writes 32 to the address in a2 and branches to another routine.
- `update_word_and_byte_2`: Compares a word at the address in a2 to 31; if it does not match, it branches to another routine.
- `increment_byte_on_match_2`: Compares a word at the address in a2 to 32; if it matches, it increments the byte at the address in a3 and branches to another routine; otherwise it branches to another routine.
- `decrement_byte_on_match_2`: Compares a word at the address in a2 to 33; if it matches, it decrements the byte at the address in a3 and branches to another routine.
- `decrement_and_copy_data_2`: Compares a word at the pointer in a2 to 64; if it matches, it loads a pointer from offset 28 of a6 into a0, checks if the byte at that pointer is non-zero, and if so, writes the contents of a global variable at offset 67 to that pointer, then branches.
- `decrement_and_copy_data`: Decrements a byte at the address stored in an offset of 28 of a6, updates a global variable with the new value, conditionally writes a byte from d4 to the address following the decremented byte, copies 11 bytes from a global table to a buffer in a loop, and calls another routine twice.
- `copy_byte_to_address`: Increments a longword from an offset of 28 in a6, moves the byte at that new address into d1, and then writes d1 to the address stored in d4.
- `copy_and_call_status_functions`: Copies 11 bytes from a global table to a destination buffer in a loop, then calls another routine twice using values derived from a structure pointed to by a6.
- `compare_word_and_branch_4`: Compares a word at the address in a2 to 57 and branches to a different location if they are not equal, otherwise it branches to another location.
- `copy_table_and_call`: Copies 11 bytes from a global table to a destination buffer starting at the address in a4 using a loop, then calls another routine twice using pointers from a structure.
- `write_byte_on_null_check_alt`: Checks if a word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d2.
- `convert_to_ascii_2`: Converts a numeric value into ASCII characters by writing bytes to a destination buffer at a pointer in a3, using division and addition to determine the character values.
- `convert_byte_to_ascii`: Compares a byte in d0 to 99; if it is not less than or equal to 99, it branches.
- `convert_to_ascii_buffer`: Writes the byte 48 to a memory location, checks if a byte is less than 10 to write 48 to the next location or writes the quotient of that byte divided by 10 plus 48 to the next location, calls another routine, and writes the returned byte plus 48 to the following location.
- `convert_to_ascii`: Writes the byte 48 to a memory location, calls another routine, and then writes the returned byte incremented by 48 to the next memory location.
- `call_two_subroutines`: Calls a routine at an offset in a global variable with two parameters, then calls another routine at a different offset in the same global variable.
- `rotate_and_threshold_32bit_4`: Performs a 16-iteration loop that rotates a 32-bit value left by one bit using the carry flag, subtracting a threshold value from the high word if it exceeds it.
- `rotate_and_threshold_32bit`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_5`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products and returns the result.
- `negate_result_on_flag_2`: Calls another routine and negates its return value if the resulting value in an address register is non-zero.
- `dispatch_jump_table_entry`: Loads a byte from the stack, compares it against five, and if it is less than or equal to five, uses it as an index into a jump table to call another routine; otherwise, it jumps to a different location.
- `clear_registers_and_return`: Clears d0 and restores d2 through d4 and a2 from the stack before returning.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `check_word_and_initialize`: Saves registers a2 and a3 to the stack, loads a word from an offset of 14 from the stack into d0, and sets a1, a3, and a2 to specific constant values.
- `check_range_and_branch`: Compares a 16-bit value in d0 to 256 and branches if it is less than 256, branches if it is greater than 256, otherwise it branches.
- `compare_and_branch_4`: Compares a word in d0 to 512 and branches if not equal, otherwise it branches to another location.
- `compare_word_and_branch_2`: Compares a word in d0 to 1024 and branches if they are not equal.
- `initialize_structure_fields`: Writes several 32-bit and 16-bit values into a structure pointed to by a pointer in a1, and updates a global variable and two other memory locations.
- `initialize_structure_values_3`: Writes 800 to offset 26 of a structure, 1060 to offset 30, 100 to offset 36, and 49 to offset 45 of a structure pointed to by a1, then branches to another routine.
- `initialize_structure_values_variant`: Writes 1168 to offset 26 of a structure, 1590 to offset 30, 150 to offset 36, and 49 to offset 45 of a structure pointed to by a1, then writes 53 to a location pointed to by a2 and branches to another routine.
- `initialize_structure_values`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 33, and writes 30 to two memory locations.

### Okidata_292

- `return_constant_34`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_chain`: Calls three different routines using offsets from a global variable and returns zero if all calls succeed, otherwise it returns a negative value.
- `verify_library_function_availability_2`: Loads a pointer from a global variable into a6, calls another routine at an offset from that pointer, restores a6, loads a different pointer into a1, calls another routine at an offset from that pointer, and then sets d0 to negative one.
- `execute_library_calls_2`: Calls a routine through a pointer in a global structure three times, passing an undefined value from a register to the second and third calls, then returns zero.
- `return_constant_35`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_36`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_6`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte at specific offsets, then executes three routines through a global base and one routine through a pointer retrieved from the structure.
- `manage_printer_buffer_data`: Implements a multi-mode state machine that manages a memory buffer by clearing it, performing bitwise OR operations on specific bytes, or populating it with a sequence of control characters and data values in a loop.
- `clear_d0_and_restore_registers_3`: Sets register d0 to zero and restores several data and address registers from the stack before returning.
- `initialize_command_buffer`: Saves registers to the stack, loads several values from the stack, and branches to another routine if the word at the address in a2 is not 1.
- `format_printer_command_buffer`: Copies a sequence of bytes from a table into a destination buffer, inserts specific control characters based on values read from a structure, and appends a string or command sequence depending on the state of a status byte.
- `update_table_byte`: Compares a word at an offset of a pointer in a1 to a constant, and if equal, increments a value in d3 and writes the byte 15 to a memory location calculated from d1 and d2.
- `process_command_byte_and_update_state`: Copies a null-terminated string into a destination buffer and then performs a series of conditional checks on a command byte.
- `copy_table_to_memory`: Loops while d6 is less than or equal to 4, copying bytes from a global table into a memory location determined by the sum of d1 and d2, while incrementing d3 and d6.
- `compare_and_set_byte`: Compares a word at the address in a2 to 66; if it matches, it writes 1 to a longword at a memory location pointed to by d1, then loads a pointer from that location into a0, compares a word at offset 86 of the structure in a0 to 2048, and if it matches, writes 60 to the address in a0 before branching.
- `compare_and_write_byte`: Compares a 16-bit word at offset 86 of the structure pointed to by the address in a1 to 1024; if they are not equal, it branches, otherwise it writes the byte 137 to the address in d5 and branches.
- `write_byte_at_pointer`: Moves the address in d5 to a0, writes the byte 80 to the address in a0, and branches.
- `write_bytes_from_table_to_memory`: Copies a sequence of bytes from a table to a memory location calculated by adding two registers, incrementing two counters during the process.
- `update_conditional_values`: Compares a word at the address in a2 to 32; if it is not 32, it tests the byte at the address in a3, and if that byte is not positive, it writes 1 to the address in a3 and 29 to the address in a2, then branches to another routine.
- `clear_byte_conditionally`: Clears a byte at the address in a3 if it is not positive, then writes the word 30 to the address in a2 and branches to another routine.
- `branch_with_mask`: Loads the value 255 into register d0 and branches to another routine.
- `update_status_values_2`: Compares a word at the address in a2 to 33, and if it matches, checks the byte at the address in a3; if that byte is not positive, it writes 255 to the address in a3 and 29 to the address in a2, then branches to another routine.
- `clear_byte_if_not_positive`: Clears a byte at the address in a3 if it is not positive, then writes the word 28 to the address in a2 and branches to another routine.
- `return_all_ones`: Returns immediately after branching to another routine.
- `process_status_sequence`: Writes specific bytes to a stack location or a memory location based on the value at the address in a2, and if a2 contains 2, writes a three-byte sequence to a memory location determined by d2 and d3.
- `check_status_byte_2`: Compares a word at the address in a2 with 12, and if it is not equal, branches; otherwise, it checks if a byte at the address in a0 is 39, and if it is not, branches; if it is, it writes 30 to the address in a0, then checks if a byte at the address in a0 is 37, and if it is not, branches; otherwise, it branches to another routine.
- `write_sequence_to_memory`: Writes the bytes 27 and 114 followed by a byte from a global table to a memory location determined by the sum of two registers.
- `write_status_byte_2`: If the word at the address in a2 is zero, it writes the byte 253 to offset 2720 of the structure pointed to by the address in d1.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Okimate_20

- `return_constant_37`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_library_function_pointers_4`: Attempts to call three consecutive functions at specific offsets within a structure pointed to by a global variable, returning zero if all calls succeed or a failure code otherwise.
- `execute_library_calls_7`: Loads a pointer from a global variable into a1, calls a function at an offset from a second pointer, restores a6 from the stack, loads a new pointer into a1, calls another function at an offset from a6, restores a6 from the stack, and branches to another routine.
- `update_handler_pointer`: Calls a routine through a pointer in a global structure three times, then sets a global variable to the address of this routine regardless of the value at a specific offset in another global structure.
- `update_function_pointer`: Loads a pointer from a global variable into a0, compares the word at offset 100 of that pointer to 2, and if it is 2, loads a new address into a0 and stores it at offset 54 of a global variable before branching to another routine.
- `set_function_pointer`: Stores the address of this routine into a global variable at offset 54 and returns zero.
- `return_constant_38`: Returns the constant an address in d0 to the caller without any other effect.
- `manage_buffer_state_machine`: Implements a state machine via a switch statement to manage a data buffer, including calculating offsets, performing bitwise OR operations on buffer elements, clearing memory with a loop, and writing specific byte sequences to a memory location.
- `restore_registers_and_return_2`: Clears register d0 and restores several data and address registers from the stack before returning.
- `manage_buffer_state_machine_2`: Implements a state machine that performs buffer operations including zeroing memory, writing a sequence of bytes starting with specific control characters, calculating offsets based on a global variable, and calling other routines to read or write data to a buffer located at an offset within a structure.
- `clear_d0_and_restore_registers_8`: Clears the d0 register and restores d2 through d6 and a2 through a5 from the stack before returning.
- `divide_32_bit_integers_2`: Performs 32-bit division of a value in d1 by a value in d2, returning the quotient and remainder.
- `rotate_and_threshold_32bit_2`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_6`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components.
- `forward_call_to_helper_3`: Moves the contents of d1 into d2, moves the contents of d0 into d1, calls a helper routine, and restores d2 from the stack before returning.
- `initialize_stack_and_compare`: Saves registers to the stack, loads several values from the stack into a0, a2, a3, and a4, clears d2, d5, and a4, and compares a word at the address in a0 to 1, branching if they are not equal or otherwise branching to another location.
- `build_command_sequence_2`: Copies 15 bytes from a global table to a buffer in a3 using a loop, then performs several conditional writes to the buffer based on values read from a structure pointed to by a4.
- `build_command_sequence`: Writes a sequence of bytes to a buffer in a3 based on values read from a structure in a4, including a loop that writes the byte 13 a variable number of times.
- `fill_memory_with_thirteen`: Fills a memory range starting at the address in a3 with the value 13, using a counter in d2 and a limit in d5, until the counter reaches a value determined by a byte at an offset from a pointer in d3.
- `process_okimate_printer_commands`: Processes a command byte by either appending escape sequences and carriage returns to a buffer, toggling a state byte, or filling a memory range with a specific byte value using a loop based on a length byte found in a table.
- `fill_memory_with_thirteen_2`: Fills a memory range with the value thirteen using a loop, where the number of bytes to write is determined by reading a byte from a location pointed to by a table entry.
- `update_printer_state_2`: Checks a word at a0 for several specific values and updates a byte at a2 or a word at a0 accordingly, while also handling a case where it fills a memory range with the value 13 based on a count and limit.
- `fill_memory_with_constant`: Writes the value thirteen to a memory location at an offset determined by a counter, incrementing both the counter and a limit value until the limit is reached.
- `update_command_byte`: Compares a 16-bit word at the address in a0 to 32; if it is not equal, it branches, otherwise it tests the byte at the address in a2, and if that byte is zero, it writes the byte 1 to the address in a2 and the word 27 to the address in a0 before branching.
- `update_byte_and_word`: Tests a byte at the address in a2, and if it is non-negative, branches; otherwise, it clears the byte at the address in a2, writes the word 30 to the address in a0, and branches to another routine.
- `load_and_branch`: Loads the value negative one into register d0 and performs a short branch to an offset.
- `modify_command_data`: Compares a word at the address in a0 to 33; if it is 33, it tests the byte at the address in a2, and if that byte is not positive, it writes 255 to the address in a2 and 29 to the address in a0, then branches to another routine.
- `modify_byte_at_pointer`: Tests the byte at the address in a2; if it is less than or equal to zero, it clears the byte at the address in a2, writes 28 to the address in a0, and branches to another routine; otherwise it branches to another routine.
- `branch_to_offset_3`: Branches to a short offset.
- `map_command_values`: Compares a byte at a pointer against several constants and writes specific byte or word values to a destination based on the match.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_Advantage_D25

- `return_constant_39`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_indirect_calls`: Calls three functions via offsets from a global variable and its contents, performing cleanup calls if any of the three return non-zero.
- `execute_dual_calls`: Calls a routine twice using different pointers from a table and then sets d0 to negative 255 before branching.
- `call_library_function_thrice`: Calls a library function three times using a pointer from a global variable, passing the value from a6 to the second and third calls.
- `return_constant_40`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_41`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_42`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_command_parameters_2`: Saves registers to the stack, loads three pointers from the stack, clears three registers, and compares the word at the first pointer to one before branching to another routine or continuing to a different location.
- `parse_control_sequence`: Copies a sequence of bytes from a source structure into a destination buffer, applying specific byte transformations based on values found at offsets 1658 and 1654 of the structure.
- `process_buffer_and_update_values`: Copies bytes from a source address in d4 to a destination address in a4 in a loop until a null byte is encountered, then performs several conditional checks and writes to a pointer in a0 and a byte at a pointer in a1, and finally copies 11 bytes from a global table to a buffer in a loop.
- `copy_byte_from_table_2`: Copies a byte from a table at offset d0 to a destination at a3, which is a pointer derived from d2 and a4, then branches based on a comparison.
- `reset_value_if_match`: Compares a word at the pointer in a0 to 27; if it matches and the byte at the pointer in a1 is not negative, it writes 32 to the word at the pointer in a0 and 1 to the byte at the pointer in a1, then returns.
- `update_status_on_condition`: Compares a word at the address in a0 to 27 and branches if they are not equal, otherwise it tests a byte at the address in a1 and, if it is less than zero, writes the word 4 to the address in a0.
- `update_status_word`: Compares a word at the pointer in a0 to 28; if it matches, it checks if a byte at the pointer in a1 is greater than 0, and if so, writes 33 to the pointer in a0, then branches to another routine.
- `set_status_values`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is non-zero, it writes 33 to the address in a0, writes 255 to the address in a1, and branches to another routine.
- `write_status_value`: Compares a word at the pointer in a0 to 29; if it matches, it checks if a byte at the pointer in a1 is greater than 0, and if so, writes 2 to the pointer in a0, then branches to another routine.
- `check_and_update_word`: Compares a word at the address in a0 to 30; if it matches and the byte at the address in a1 is negative, it writes 32 to the address in a0.
- `update_word_based_on_comparison`: Compares a word at the address in a0 to 31; if it matches, it tests the byte at the address in a1; if that byte is greater than zero, it writes 33 to the address in a0, otherwise it writes 32 to the address in a0 and clears the byte at the address in a1; if the initial word comparison does not match, it branches to another routine.
- `increment_status_byte`: Compares a 16-bit word at the address in a0 to 32; if it is not equal, it branches, otherwise it increments the byte at the address in a1 and branches.
- `decrement_byte_on_match`: Decrements a byte at the address in a1 if the word at the address in a0 is equal to 33, otherwise it branches.
- `write_status_byte_if_zero`: Checks if a 16-bit word at the address in a0 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d1.

### Alphacom_Alphapro_101

- `return_constant_43`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_callback_chain`: Calls a routine via a pointer at an offset from a global variable up to three times, using different base addresses from a table, and calls a cleanup routine if any call returns non-zero.
- `execute_dual_indirect_calls`: Loads two pointers from a global variable and calls two separate functions using offsets from those pointers, then returns with negative one in the data register.
- `execute_indirect_function_calls_4`: Fetches a function pointer from a global variable using a specific offset and calls that function three times in succession.
- `return_constant_44`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_45`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_46`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_stack_registers`: Saves six registers to the stack, loads three pointers from the stack, and clears registers d1, d2, and d3 before branching based on a 16-bit comparison at the first pointer.
- `construct_printer_command_buffer`: Copies twelve bytes from a table into a buffer, then appends an escape sequence containing specific control characters and spaces based on values read from a structure.
- `fill_buffer_with_spaces_2`: Fills a buffer starting at the address in a4 with the value 32 in a loop until a counter reaches a limit, then writes a sequence of bytes including 27, 57, 32, 27, 48, 13, 27, 39, and 13 to the buffer.
- `fill_buffer_with_spaces`: Fills a buffer starting at a4 with the value 32 in a loop until a counter reaches a limit, then writes a sequence of seven specific bytes to the end of the buffer.
- `update_memory_values`: Compares a word at the address in a1 to 27; if it matches, it tests the byte at the address in a2; if that byte is not zero, it writes 32 to the address in a1 and writes 1 to the address in a2, then branches to another routine; otherwise it branches to another routine.
- `compare_and_update_word`: Compares a word at the address in a1 to 27; if it matches, it tests the byte at the address in a2, and if that byte is less than zero, it writes 4 to the word at the address in a1 before branching.
- `modify_word_on_condition`: Compares a word at the address in a1 to 28 and branches if they are not equal, otherwise it tests a byte at the address in a2 and, if it is greater than zero, writes the word 33 to the address in a1.
- `update_value_on_condition_2`: Compares a word at the address in a1 with 29, and if it is not equal, branches; otherwise, it tests a byte at the address in a2, and if it is zero, branches; otherwise, it writes the word 33 to the address in a1, writes the byte 255 to the address in a2, and branches to another routine.
- `update_value_on_condition`: Writes the word 2 to the address in a1 if the word at a1 is 29 and the byte at a2 is greater than zero, otherwise it branches.
- `update_word_on_condition`: Compares a word at the address in a1 to 30 and branches if they are not equal, otherwise it tests a byte at the address in a2 and, if it is less than zero, writes the word 32 to the address in a1.
- `modify_word_and_byte_3`: Compares a word at the address in a1 to 31; if it matches and the byte at the address in a2 is greater than zero, it writes 33 to the address in a1.
- `increment_byte_on_match_4`: Compares a word at the address in a1 to 32; if it matches, it increments the byte at the address in a2 and branches to another routine.
- `decrement_byte_on_match_4`: Compares a word at the address in a1 to 33; if it matches, it decrements the byte at the address in a2 and branches to another routine; otherwise it branches to another routine.
- `format_alphacom_printer_buffer`: Checks a value in a1 and either writes a two-byte sequence based on a stack value or writes a series of escape sequences and padding bytes into a buffer.
- `write_constant_byte_3`: Writes the byte 66 to a destination at a0, which is calculated by adding a4 to a pointer derived from d1, then branches.
- `fill_buffer_with_patterns`: If a 16-bit word at the address in a1 is 66, it writes a byte, then loops to write 80 instances of the value 8, followed by five specific bytes, then loops to write 80 instances of the value 32, followed by five more specific bytes and a final byte; otherwise, if the word is zero, it writes the byte 253 to offset 1056 of the structure pointed to by a global variable.
- `write_status_byte_if_zero_2`: Checks if a 16-bit word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by the pointer in d2.

### Epson_JX-80

- `return_constant_47`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_library_call`: Calls a routine at an offset from a global variable three times, and if all three calls return non-zero, it returns 0; if any call fails, it calls another routine at an offset from the global variable and returns -1.
- `test_library_function_offsets_2`: Loads a library base pointer from a global variable and calls two functions at specific offsets from that base, returning negative one if both calls succeed.
- `execute_triple_call`: Calls a routine through a pointer found at an offset from a global base three times, passing the value from a6 to the second and third calls.
- `return_constant_48`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_49`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_7`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a constant value, then executes several routines through a global base using a pointer retrieved from that structure.
- `format_epson_jx80_print_buffer`: Constructs a structured print buffer for an Epson JX-80 printer by calculating offsets and writing control sequences, including ESC/P commands, character data, and byte values into a memory area.
- `clear_d0_and_restore_registers_4`: Clears the d0 register and restores several data and address registers from the stack before returning.
- `setup_parameters_and_compare`: Saves registers to the stack, loads values from the stack into a2, d1, and a3, clears d3, d6, and d2, and compares a word at the address in a2 to 1, branching if they are not equal or otherwise branching to another location.
- `format_printer_command_buffer_3`: Copies a sequence of bytes from a table into a destination buffer and populates it with command bytes based on values read from a structure.
- `process_epson_jx_80_commands`: Parses command bytes and status values from a structure to generate a sequence of control bytes in a destination buffer.
- `copy_bytes_from_stack_pointers`: Loads a pointer from the stack into a0, reads a byte from that address and subtracts 1, writes that byte to the address in a1, then loads a second pointer from the stack into a1, increments it, reads a byte from that address, and writes it to the address in a0 before branching to another routine.
- `copy_byte_from_table_3`: Calculates a destination address using d0 and d1, then enters a loop that copies a byte from a table at address 0 using an index in d2 to the destination, repeating until d6 reaches 8.
- `process_printer_command_sequence`: Evaluates a command byte to update a printer state machine and generates an escape sequence by writing specific bytes to a buffer.
- `load_and_check_pointer`: Loads a pointer from the address in a1 into a0, compares the word at offset 870 of the structure in a0 to 1024, and if they match, writes the byte 137 to the address in a0 before branching.
- `write_byte_at_pointer_2`: Loads the address in d4 into a0, writes the byte 80 to the address in a0, and then branches.
- `copy_bytes_from_table_3`: Copies up to eight bytes from a table into a destination buffer by reading from a zero-based table using an index in a register and writing to a pointer formed by the sum of two registers.
- `update_status_values`: If a 16-bit word at the address in a2 is 32, it otherwise returns; if the byte at the address in a3 is not zero, it otherwise returns; if the byte at a3 is zero, it writes 1 to a3, writes 27 to a2, and branches.
- `clear_status_byte_3`: If the byte at the address in a3 is non-negative, it otherwise returns; if the byte at a3 is negative, it clears the byte at a3, writes 30 to a2, and branches.
- `set_d0_and_branch`: Sets d0 to negative 255 and branches.
- `process_command_match`: Compares a word at the address in a2 to 33; if it does not match, it branches to another routine; if it matches, it tests the byte at the address in a3, and if that byte is not zero, it writes the byte 255 to the address in a3, writes the word 29 to the address in a2, and branches to another routine.
- `clear_byte_and_set_word_2`: Tests the byte at the address in a3; if it is greater than zero, it clears the byte at the address in a3, writes 28 to the address in a2, and branches to another routine.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `process_command_data`: Checks a byte at a pointer in a2 against several constants to write specific bytes to a destination, including writing 27 and 74 to a buffer, or writing 253 to an offset of 2722 in a structure, or writing 27 and 114 followed by a value from a global table to a buffer.
- `process_printer_command`: Checks a 16-bit word at the address in a2; if it is 4, it writes three bytes starting at a specific offset in a structure; if it is 12, it checks if a byte from the stack is 39 and writes it; if it is 12 and the stack byte is 47, it modifies the stack byte to 30 and writes three bytes to a structure; if it is 0, it writes a byte to an offset of 2688 in a structure.
- `check_and_modify_buffer`: If a 16-bit word at the address in a2 is 12, it otherwise returns; if the byte at the address in a0 (loaded from the stack) is 39, it writes 30 to that address; if the byte at that address is 37 and is not less than or equal to 37, it otherwise returns; if the byte is 37 and is greater than 37, it branches.
- `write_three_bytes`: Writes the byte 27 and the byte 114 to a buffer, followed by a word from a global table at an offset determined by a value on the stack, then returns.
- `write_status_byte_if_zero_3`: Checks if a 16-bit word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d2.
- `call_library_vector_198_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Epson

- `return_constant_50`: Returns the constant an address in d0 to the caller without any other effect.
- `verify_triple_function_execution`: Calls a function pointer located at an offset from a global variable three times, checking for non-zero return values.
- `execute_repeated_calls`: Loads a pointer from a table, calls a routine at an offset from a pointer in a6, restores a6, loads a different pointer into a1, calls a routine at an offset from a6 again, restores a6, and branches to a location with a value of 255 in d0.
- `execute_indirect_function_calls_5`: Fetches a function pointer from a table using an offset and calls that function three times in succession.
- `return_constant_51`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_52`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_8`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value at specific offsets, then executes three routines through a global base and one routine through a pointer stored within the structure.
- `manage_printer_buffer_state`: Acts as a multi-purpose state machine that manages a data buffer by performing bitwise OR operations, clearing memory ranges, and inserting escape sequences into a buffer.
- `clear_d0_and_restore_registers_5`: Sets register d0 to zero and restores several registers from the stack before returning.
- `initialize_registers_and_compare`: Saves registers to the stack, loads values from the stack into a1, a4, and a3, clears d3, d5, and d1, and compares a word at the address in a1 to 1, branching if they are not equal or otherwise branching to another location.
- `format_printer_command_buffer_4`: Copies a sequence of bytes from a global table into a destination buffer and populates it with command bytes based on values read from a structure pointed to by a register.
- `process_epson_printer_command`: Parses a command byte and updates a printer state structure, handling specific character sequences and control codes.
- `copy_bytes_from_stack`: Loads a byte from a location pointed to by a stack offset, decrements it, and writes it to the address in a1; then it loads a byte from an address incremented from a stack offset and writes it to the address in a0, before branching to another routine.
- `copy_data_loop_3`: Increments d3 and d5, calculates a destination address by adding a4 to the address in d3, and copies a byte from a table at offset d0 to the destination; it loops if d0 is greater than d5.
- `update_printer_state_3`: Compares a word at the address in a1 to 66; if it matches, it writes 253 to offset 2688 of the structure in a1, sets a byte at a4 to 0, and then checks a word at offset 2646 of the structure in a1 to write 96, 137, or 80 to a2.
- `load_and_check_pointer_alt`: Loads a pointer from the address in a1 into a0, compares the word at offset 870 of the structure in a0 to 1024, and if they match, writes the byte 137 to the address in d2 before branching.
- `write_byte_at_pointer_alt`: Loads the address in d2 into a0, writes the byte 80 to the address in a0, and then branches.
- `write_bytes_from_table`: Copies up to eight bytes from a table into a destination buffer by iterating through a pointer and an offset, then returns.
- `test_and_set_byte`: Compares a word at the address in a1 to 32; if it matches, it tests the byte at the address in a3, and if that byte is not zero, it writes 1 to the address in a3 and writes 27 to the word at the address in a1 before branching.
- `clear_byte_on_negative`: Tests the byte at the address in a3; if it is less than zero, it clears the byte at the address in a3, writes 30 to the word at the address in a1, and then branches.
- `set_d0_and_branch_alt`: Sets d0 to 255 and then branches.
- `process_command_match_alt`: Compares a word at the address in a1 to 33; if it does not match, it branches to another routine; if it matches, it tests the byte at the address in a3, and if that byte is not zero, it writes the byte 255 to the address in a3, writes the word 29 to the address in a1, and branches to another routine.
- `clear_byte_and_set_word_alternate`: Tests the byte at the address in a3; if it is greater than zero, it clears the byte at the address in a3, writes 28 to the address in a1, and branches to another routine.
- `branch_to_another_routine_2`: Performs a short relative branch to another routine.
- `execute_command_logic`: Checks a byte at a pointer in a1 against several constants to write specific bytes to a destination, including writing 27 and 74 followed by a value from a stack to a buffer, or writing 253 to an offset of 2722 in a structure.
- `write_byte_if_zero`: Checks if a word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d1.
- `call_library_vector_198_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### HP_LaserJet_PLUS

- `return_constant_53`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_call_sequence_2`: Loads a pointer from a global variable into a1, then performs three consecutive calls to routines at offsets from the pointer stored in that global variable, returning 0 if all calls succeed or -1 otherwise.
- `execute_triple_call_sequence`: Loads a pointer from a global variable and calls a routine at a negative offset from a second pointer twice, then branches to another location after setting a register to negative one.
- `execute_triple_function_pointer`: Retrieves a function pointer from an offset within a global structure and calls that function three times in succession.
- `return_constant_54`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_9`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte at specific offsets, then executes several routines through a global base using a pointer retrieved from that structure.
- `process_hp_laserjet_control_codes_2`: Processes a sequence of control codes to build a command buffer by copying bytes from a global table and modifying specific offsets based on printer state values.
- `format_hp_laserjet_command_buffer_2`: Copies bytes from a global table into a buffer, modifies specific bytes based on values read from a printer state structure, and calls another routine to write the resulting data.
- `process_printer_control_sequence_3`: Evaluates a control byte and an associated value to update a state machine, modifies a buffer by copying bytes from a table, and calls write_status_word or generate_epson_esc_commands based on the control byte value.
- `copy_table_elements`: Copies ten elements from a table into a destination buffer using a loop, then calls another routine twice with different values derived from a structure pointed to by a6.
- `compare_and_update_values`: Compares a 16-bit word at the address in a2 to 27; if it is not equal, it branches, otherwise it checks if the byte at the address in a3 is zero, and if so, writes 32 to the address in a2, writes 1 to the address in a3, and branches to another routine.
- `update_hp_laserjet_escape_sequence`: Compares a word at the address in a2 to twenty-seven; if it matches and the byte at the address in a3 is zero, it writes the word four to the address in a2, otherwise it returns.
- `update_word_on_match_3`: Compares a word at the address in a2 to 28; if it matches and the byte at a3 is greater than zero, it writes 33 to the address in a2 and branches to another routine.
- `modify_word_and_byte_2`: Compares a word at the address in a2 to 29; if it matches and the byte at the address in a3 is non-zero, it writes 33 to the address in a2 and writes 255 to the address in a3.
- `set_word_on_match`: Compares a word at the address in a2 to 29; if it matches, it tests a byte at the address in a3, and if that byte is greater than zero, it writes the word 2 to the address in a2 and branches to another routine.
- `update_word_on_match_alt`: Compares a word at the address in a2 to 30; if it matches and the byte at a3 is less than zero, it writes 32 to the address in a2 and branches to another routine.
- `update_threshold_and_byte`: Compares a word at a pointer in a2 to 31; if it matches, it checks a byte at a3, and depending on whether that byte is positive or negative, updates the word in a2 to 33 or 32 and either clears the byte at a3 or branches to another routine; otherwise, if the word is 31 and the byte is non-positive, it sets the word in a2 to 33 and clears the byte at a3.
- `increment_byte_on_match_3`: Compares a word at the address in a2 to 32; if it does not match, it increments the byte at the address in a3 and branches to another routine.
- `decrement_byte_on_match_3`: Compares a word at the address in a2 to 33; if it matches, it decrements a byte at the address in a3 and branches to another routine, otherwise it returns.
- `check_and_update_status_byte`: Compares a word at the address in a2 to 64 and, if equal, checks if a byte at the address stored in a6 plus 28 is non-zero; if so, it writes the byte 67 to the address stored in a6 plus 28 and branches to another routine, otherwise it branches to another routine.
- `decrement_and_update_buffer`: Decrements a byte at a pointer stored in a structure, updates a global variable with the new value, and conditionally updates a byte at a destination address.
- `increment_and_write_byte`: Increments a longword value stored in a global variable by one, reads the byte at that new address, and writes it as a word to the address held in register d4, then branches to another routine.
- `copy_data_and_call_2`: Copies 11 words from a global table to a destination buffer starting at an offset in a6, then calls two routines using pointers from a structure in a6.
- `compare_word_and_branch_5`: Compares a word at the address in a2 to 57; if not equal, it branches to a different location, otherwise it branches to another location.
- `copy_status_parameters_and_call`: Copies 11 bytes from a global table to a destination buffer using a loop, then calls another routine twice using values from two different pointers.
- `write_byte_on_word_test`: Checks if a word at the address in a register is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by a register, then returns.
- `convert_to_ascii_sequence`: Writes a byte to the address at the sum of param_2 and param_3 based on the value of param_1, then calls another routine and writes the returned value plus 48 to the address at offset 2 from the original destination.
- `convert_byte_to_ascii_2`: Compares the byte in d0 to 99; if it is greater than 99, it moves the address in d3 to a2, increments d3, adds the value in d4 to a2, writes 49 to the address in a2, subtracts 100 from d0, and branches to another routine.
- `convert_integer_to_ascii`: Writes the byte 48 to a memory location, then writes either 48 or the quotient of a byte divided by 10 plus 48 to the next location, calls another routine, and writes the returned byte plus 48 to the following location.
- `convert_digit_to_ascii`: Writes the character zero to a memory location pointed to by two registers, calls another routine, and then writes the returned byte incremented by 48 to the subsequent memory location.
- `execute_dual_function_pointers`: Pushes two parameters onto the stack, calls a subroutine via a pointer at offset 100 of a global variable, then calls a second subroutine via a pointer at offset 104 of the same global variable before returning zero.
- `rotate_32bit_value_5`: Performs a 32-bit rotation using a loop that runs sixteen times, updating two 16-bit halves based on the carry flag.
- `rotate_and_threshold_32bit_3`: Rotates a 32-bit value left by one bit for sixteen iterations, using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_7`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit partial components.
- `negate_result_on_flag_3`: Calls another routine and negates its return value if the returned value in a0 is nonzero.
- `dispatch_via_table_3`: Restores registers from the stack, loads a value from the stack, and uses it to select and jump to a destination from a jump table.
- `restore_registers_and_return_3`: Restores d2-d3 and a2-a4 from the stack and returns.
- `call_library_vector_198_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `initialize_local_parameters`: Restores registers from the stack, loads a word from the stack into d0, sets a1 to 4, a3 to 47, and a2 to 46, then compares d0 to 768; it branches to another routine if d0 is less than 768, branches to another routine if d0 is greater than 768, otherwise it branches to another routine.
- `compare_word_and_branch_3`: Compares a 16-bit word in d0 to 256 and branches to one location if it is less than or equal, branches to another if it is greater than, otherwise it branches to a third location.
- `compare_and_branch_5`: Compares a 16-bit value in d0 to 512 and branches if not equal, otherwise it branches to another routine.
- `compare_and_branch`: Compares a word in d0 to 1024 and branches to different locations depending on the result.
- `initialize_structure_fields_2`: Writes 600 and 827 to specific offsets of a structure pointed to by a1, writes 75 to offsets 36 and 34 of that structure and a second structure, and sets a global variable to 48.
- `setup_structure_values`: Sets values at offsets 26 and 30 of a structure pointed to by a1, then writes 100 to offsets 36 and 34 of that structure and 49 to offset 45.
- `setup_structure_values_alt`: Sets values at offsets 26 and 30 of a structure pointed to by a1, then writes 150 to offsets 36 and 34 of that structure and 53 to a location pointed to by a2.
- `initialize_structure_values_2`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 51, and writes 48 to two memory locations.

### type

- `initialize_memory_hunk_structure`: Allocates a memory block based on a value from a function call, populates a structure with various header fields and pointers, and updates a linked list within a global structure.
- `process_dos_hunk_data`: Parses a structure containing AmigaDOS hunk type codes to determine memory allocation requirements, clears a range of memory using a loop, and performs a sequence of bitwise OR operations on a register.
- `store_and_call_routines`: Stores a value in a global variable, calls a routine using a pointer from a global variable with a constant argument, calls another routine via a pointer from a global variable, and finally calls a third routine.
- `conditional_xor_decrypt_block`: The routine checks a flag stored in a global variable, clears the flag, and if set XORs a block of words starting at a global variable with a constant, repeating a fixed number of times.
- `update_flag_byte`: Checks a byte at a relative offset; if it is zero, it writes 1 to that location and branches, otherwise it continues.
- `perform_logical_or_sequence`: Performs a sequence of nine bitwise OR operations on register d0 using various immediate byte values.

### 12

- `return_constant_55`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_data_to_memory`: Copies a block of data from a source address pointed to by a register to a destination address pointed to by another register, using a counter in a third register to track the number of bytes to transfer, and returns when the counter reaches zero.

### 15

- `return_constant_56`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_57`: Returns the constant an address in d0 to the caller without any other effect.
- `process_font_glyph_data`: Reads a sequence of bytes from a font data table and writes them to a destination buffer in memory, using a loop to copy each byte until a termination condition is met.

### 12

- `return_constant_58`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_value`: The routine clears the accumulator and returns zero.

### 20

- `return_constant_59`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_3`: Returns the value zero to the caller.

### 11

- `return_constant_60`: Returns the constant an address in d0 to the caller without any other effect.

### Balance of Power - The 1990 Edition (1990)(Mindscape)[cr QTX].adf:CLI

- `parse_command_line_arguments`: Parses a command line string by iterating through characters to identify tokens, handling quoted strings and converting specific character sequences into newline or escape characters, and stores the resulting pointers in a table.
- `invoke_two_function_pointers_2`: loads a pointer from an offset of 92 from a base register into a0, calls a function at a negative offset of 416 from that base register, repeats this with a different offset of 404 from the base register, and then returns.
- `call_dos_function`: loads a pointer into a1, calls a function at a negative offset of 552 from a base register, stores the result in a global variable at offset 8, and returns if the result is zero; otherwise it continues.
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

- `execute_illegal_instruction`: as library code, executes an illegal instruction.
- `store_a0_to_global`: as library code, stores the contents of a0 into a global variable at offset 408.
- `manage_linked_list_node_2`: As library code, if a byte at offset 8 of a structure in a6 is zero, it moves a long value from the structure to a pointer at offset 4 of another long value, sets a pointer at offset 4 of the structure to that long value, calls a routine via a pointer in a global variable, and returns; otherwise it sets bit 3 of a byte at offset 14 of the structure in a6 and returns.
- `set_status_bit_2`: as library code, sets bit 3 of a byte at offset 14 of a structure in a6 and returns.
- `increment_and_call_handler`: Checks if bit 3 of a byte at offset 14 in a structure in a6 is clear; if so, it increments a 16-bit value at offset 32 of that structure, and if that value becomes 1, it stores a global variable and calls a library function via a pointer in a global variable, otherwise it returns -1.
- `return_negative_one`: Returns -1 in D0 and returns immediately.
- `decrement_counter_and_call`: Library code that decrements the 16-bit counter at offset 32 of the structure at A6.
- `mask_and_call_library`: As library code, it masks the lowest 7 bits of d0, calls another routine, and sets the 2nd bit of the status register.
- `calculate_bitfield_difference`: As library code, if the byte in d0 is zero it returns, or if it is negative it calls another routine and returns; otherwise it performs a loop that updates a running difference using a doubling accumulator and bitwise operations until a condition is met.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `process_and_mask_value`: As library code, it moves a long from d0 to d7, tests the byte in d7, calls a routine via a pointer at offset 20 of a6, sets bit 1 of d0, clears bit 3 of d0, and branches to another routine.
- `call_jump_table_entries`: Calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_register_and_branch_2`: Clears the longword in d1 and branches to another location.
- `execute_conditional_logic_2`: Library code that calls a routine via a pointer derived from a value in a register, then performs a series of conditional calls to other routines at various offsets from that pointer, and finally calls another routine and returns a value with bits 0 and 1 modified.
- `execute_conditional_library_calls_3`: As library code, it calls a routine at an offset from a pointer in a5, then performs a series of conditional calls to other routines at various offsets from a5 based on the values in d6 and d7.
- `execute_conditional_library_calls_2`: Library code that performs a series of calls through pointers at various offsets from a base address in A5, depending on the values of D6, D4, and D7, and returns a value with the lower three bits masked out.
- `execute_conditional_library_calls`: As library code, it moves d7 to d1, calls a routine at an offset of minus 30 from a5, and if the result is not negative, performs a series of conditional checks and calls to routines at offsets minus 48 and minus 42 from a5.
- `call_and_store_result`: As library code, it moves a long from d2 to d0, calls a routine at offset minus 60 of a5, moves the result in d0 back to d2, and branches to another routine.
- `conditionally_load_constant`: As library code, it moves a long from d2 to d0, tests a long in d4, and if d4 is not zero branches to another routine; otherwise it loads a constant into d2 and branches to another routine.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `perform_bit_shifting_loop`: As library code, it performs a series of bit-shifting and arithmetic operations in a loop, potentially calling another routine if a counter reaches -1.
- `prepare_shift_parameters`: Library code that performs a logical shift right on D7, sets D6 to zero, sets D5 to a constant, loads a constant into A0, and sets D1 and D2 to specific values before branching.
- `execute_loop_with_branching`: As library code, it executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_word_and_branch`: As library code, it pushes the word 65534 onto the stack and then branches to a different location.
- `push_word_and_branch_2`: As library code, it pushes the word 65535 onto the stack and then branches to a different location.
- `push_word_and_branch_3`: As library code, it pushes the word 1 onto the stack and then branches to a different location.
- `check_byte_and_adjust_stack`: As library code, it compares a byte in d7 to 184, branches if it is higher, otherwise it adds 2 to the stack pointer, tests d7, and returns.
- `process_math_data`: As library code, it performs complex arithmetic involving bit shifts and a loop that modifies d5, d6, and a value at the address in a0.
- `perform_math_operations_2`: This library code performs complex arithmetic involving bit shifts and a loop that modifies d5, d6, and a value at the address in a0 using values from a table.
- `perform_math_loop_2`: Library code that performs a loop that modifies d5, d6, and a value at the address in a0 using values from a table.
- `perform_math_loop`: This library code performs a loop that modifies d5, d6, and a value at the address in a0 using values from a table.
- `execute_math_loop`: As library code, it performs a loop that modifies d5, d6, and a value at the address in a0 using values from a table, and contains conditional branches based on the results of these operations.
- `load_long_and_branch`: As library code, it loads the constant an address into d7 and branches to another location.
- `recursive_call_wrapper`: As library code, it moves d5 to d6, calls another routine, moves d6 to d7, tests the byte in d7, restores registers from the stack, adjusts the stack pointer, and returns.
- `push_and_call_routine_2`: Library code that pushes the contents of d5 onto the stack, calls another routine, restores a value from the stack into d6, and then branches to another routine.
- `compare_and_branch_d6`: Library code that moves D6 to D4, branches if D4 is negative, otherwise compares D6 to a constant and branches based on whether it is higher, and finally sets D6 to zero before returning.
- `shift_and_jump`: As library code, it performs an arithmetic shift right on d4, increments d4, and branches if d4 is not zero, otherwise it jumps to a fixed address.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_calls`: Library code checks a byte in d7 and, if it is non-zero, performs a series of conditional calls to other routines based on the results of those calls.
- `rotate_long_and_branch`: As library code, it loads the constant an address into d7, rotates the bits in d7 right by 1, and branches to another location.
- `load_long_and_branch_2`: As library code, it loads the constant 21474836get into d7 and branches to another location.
- `test_stack_word_and_branch`: As library code, it pops a word from the stack into d6, tests the byte in d6, and branches if it is positive; otherwise it moves 0 into d7 and branches to another location.
- `set_negative_flag_and_restore_registers`: Sets the negative flag in the condition code register and restores registers d1 through d6 and a0 from the stack before returning.
- `load_constant_and_return`: Loads a constant value into a register, adjusts the stack pointer by eighteen bytes, and returns.
- `execute_recursive_math_logic`: As library code, it performs a series of conditional checks on d7 and calls another routine multiple times based on those checks and the resulting values.
- `execute_recursive_math`: Library code that calls another routine three times and then returns.
- `loop_with_arithmetic_and_subtraction`: As library code, it performs arithmetic shifts on d3 and d4, adds them to d5 and d6, and enters a loop that subtracts values from d7 using a table in a0 until a counter in d1 reaches zero.
- `subtract_table_from_accumulator`: The routine walks through a table pointed to by a pointer and repeatedly adds the current table entry to an accumulator, then subtracts subsequent entries while a counter is decremented.
- `test_and_branch_on_sign`: As library code, it tests the byte in d7 and branches if it is zero or negative, otherwise it performs a bitwise AND on d7 with 127, calls another routine, sets bit 1 of the status register, and returns.
- `set_register_and_branch_2`: Library code that sets register d7 to negative one and performs a short branch.
- `perform_complex_arithmetic`: Library code that executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic.
- `complex_loop_with_branching`: As library code, it executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `process_signed_byte`: Tests the byte in d7; if it is negative, it branches, otherwise it masks the byte with 127, calls another routine, and sets bit 1 of the status register before returning.
- `execute_nested_calls`: Library code calls two routines, then if a value in d7 is non-zero, performs a series of conditional calls to other routines based on various bitwise manipulations of that value.
- `transform_and_rotate_register`: Modifies a value in a register by performing a series of arithmetic operations, including a bitwise XOR, a subtraction, a swap, and a left rotation, then returns.
- `transform_d7_register`: Library code that swaps the contents of d7 with d6, performs a rotate right through the carry bit on d7 by seven, and then conditionally modifies d7 through a series of arithmetic and logical operations before returning.
- `test_and_modify_register`: Library code that compares register d7 to specific values and, if they match, modifies d7 and performs a branch.
- `test_word_and_branch`: Tests if the word in d7 after masking with a constant is zero; if it is not zero, it clears d7 and branches to another location, otherwise it continues.
- `shift_and_test_d7`: Compares register d7 to 254, and if equal, performs a 32-bit logical shift left by eight bits followed by a logical shift right through the carry bit by one bit, otherwise it returns.
- `shift_and_set_flag`: Shifts the word in d7 left by eight bits, moves the value 255 into d7, performs a rotate right through the carry bit on d7, sets the bit 1 of the condition code register, and branches to another location.
- `clear_d7_and_branch`: Sets d7 to zero and branches to another location.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `load_offset_address_a_2`: Library code that loads a longword from an offset into register A0 and then branches to the next instruction.
- `load_address_pointer`: This library code loads the longword at a constant address into a0 and then branches to another routine.
- `load_library_pointer_zero`: As library code, it loads a longword from the address in a0 with the value at offset 444 into a0 and branches to itself.
- `load_address_and_branch_9`: Library code that loads a constant address into a0 and then branches to another routine.
- `load_address_and_branch_10`: As library code, it loads the address at offset 30 into a0 and then branches to a different location.
- `load_library_address`: As library code, it loads the address of a constant into a0 and then branches.
- `load_address_and_branch_11`: Loads the address of a constant into a0 and branches to a different location.
- `store_register_via_pointer`: As library code, it invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `load_library_pointer_one`: As library code, it loads a longword from the address in a0 with the value at offset 52 into a0 and branches to itself.
- `load_zero_address_a`: Library code that loads zero into register A0 and then branches to the next instruction.
- `load_library_pointer_two`: As library code, it loads a longword from the address in a0 with the value at offset 110 into a0 and branches to itself.
- `load_jump_target_address`: Loads the address of a constant into a register and then performs an unconditional branch to the next instruction.
- `load_address_and_branch_6`: Loads a longword into address register a0 and performs a short branch to the next instruction.
- `load_address_and_branch_8`: As library code, it loads the constant address in 204 into a0 and then branches to another routine.
- `load_zero_address`: As library code, it loads zero into a0 and branches to another routine.
- `load_address_and_branch_7`: Loads a longword into address register a0 and then performs a short branch to the next instruction.
- `load_constant_address_a`: Library code that loads a constant into register A0 and then branches to the next instruction.
- `load_offset_address_a`: As library code, it loads a constant into a0 and branches to another routine.
- `load_indirect_call_address`: Loads the address of a global variable into a register and branches to another routine that performs an indirect call using an offset from that address.
- `load_pointer_and_branch`: Loads a longword from a global variable into a0 and performs a short branch to another routine.
- `load_address_and_branch`: Loads a longword from a specific memory location into register a0 and performs a short branch to another routine.
- `load_pointer_and_branch_2`: As library code, it loads a pointer from a global variable into a0 and branches to another location.
- `load_pointer_and_branch_3`: As library code, it loads a pointer from a different global variable into a0 and branches to another location.
- `load_address_and_branch_2`: Loads a constant address into a register and performs a short branch to another routine.
- `load_address_and_branch_3`: Loads a longword from a global variable into a6 and branches to an offset within another routine.
- `load_address_and_branch_4`: Loads a longword address into a6 and branches to a relative offset within another routine.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_constant_61`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_status`: If a word at offset 8 of a structure in a6 is zero, it calls a function pointer using a pointer at offset 22 of the structure, then updates a pointer in a structure with the value from a6 and sets a pointer at offset 4 of the structure pointed to by the value in a6.
- `set_icon_bit`: Sets bit 3 of a byte at offset 14 of a structure pointed to by a6 and returns zero.
- `return_constant_62`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_and_store`: Stores values from d0 and a6 into a structure, calls a function pointer using a pointer at offset 552 from a6, and if the return value is zero, calls another routine; otherwise, it stores the values from d0 and the return value into two global variables.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_logical_or`: Performs a bitwise logical OR of register d0 with zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `traverse_icon_chain_2`: Saves two registers onto the stack and calls another routine to manage an icon chain, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `invoke_icon_chain_traversal`: Saves two registers onto the stack and calls traverse_icon_chain_3 before returning.
- `manage_icon_chain_3`: Saves the current stack pointer and a pointer in a6 to the stack, calls another routine to manage an icon chain, and then restores the stack pointer before returning.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_process_table_indices_2`: Saves two address registers to the stack, calls process_table_indices_2, and then returns.
- `call_parse_numeric_string_2`: Saves two registers to the stack, calls parse_numeric_string_2, and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_byte`: Performs a logical OR operation on a byte in register d0 with zero, effectively leaving the value unchanged.
- `parse_icon_resource`: Library code that calls another routine, and if the result is non-zero, copies 43 bytes from a stack buffer to a destination structure, updates several 32-bit fields in that structure, and potentially writes a value to an offset of a pointer within that structure.
- `copy_icon_data`: Copies 43 bytes from a source buffer to a destination structure, updates several 32-bit fields in the destination structure from the source, and if a specific 32-bit field in the destination is non-zero, stores the destination address into a field at offset 424 of the structure pointed to by that field.
- `clear_d0_and_restore_registers_6`: Clears register d0 and restores registers d2 through d4 and a2 from the stack before unlocking a6 and returning.
- `copy_icon_data_to_stack`: Copies 43 bytes from a structure pointed to by the third argument into a local stack buffer, copies several fields from the same structure into local stack slots, and calls another routine with the first two arguments and a pointer to a two-word stack header.
- `parse_icon_resource_structure`: Validates an icon resource structure by checking magic values and flags, then populates various offsets within the structure by calling other routines to load data.
- `populate_icon_resource_data`: Checks a structure for specific magic values and flags, then iteratively calls another routine to populate various offsets within that structure or a linked list of structures.
- `process_icon_resource`: Library code iterates through a linked list of structures, updating values at specific offsets within those structures and calling other routines to process resource chains.
- `process_resource_chain`: Enters a loop that calls a routine and, if the result is non-zero, calls another routine to update a value in a structure, then decrements a counter in a structure, and finally performs conditional library calls based on a value at offset 70 of a structure.
- `clear_d6_and_branch`: Clears register d6 and branches to another location.
- `unlock_and_return`: This library routine unlocks the resource in a6 and returns.
- `traverse_icon_structure_recursively`: Recursively processes an icon structure by checking flags and following pointers to child elements, sub-lists, or additional data structures.
- `process_icon_structure_recursively`: Iterates through a hierarchical icon structure by checking flags and following pointers at various offsets within a structure.
- `process_icon_list_recursively_2`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a.
- `process_icon_list`: Increments a pointer in a loop until it reaches zero, calls a function pointer, and if that function returns non-zero, iterates through a list of pointers to call the same function pointer recursively and perform additional calls.
- `process_icon_list_recursively`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a pointer or table.
- `traverse_icon_chain`: Enters a loop that calls another routine with an incremented parameter and, if the result is non-zero, calls that same routine again with the returned value.
- `return_immediately`: This library routine returns immediately.
- `incremental_offset_recurse`: Calls another routine with the same first argument and the second argument increased by a fixed offset of 140, then returns.
- `execute_conditional_library_calls_4`: Library code that calls a function pointer, then conditionally calls another routine with an offset of the result, then calls a third routine with the result and a pointer, and finally calls a fourth routine with the result and a pointer if the third call returns zero.
- `call_icon_update_routine`: This library code pushes registers onto the stack, calls another routine, and if the return value in d0 is zero, it returns d0; otherwise, it returns the value from a2 in d0.
- `return_zero_4`: This library routine sets register d0 to zero and returns.
- `update_icon_data`: Decrements a 16-bit counter through a pointer, and if the counter reaches zero or is less than one, calls another routine and potentially decrements the counter again.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from registers and the stack into consecutive slots of that table.
- `traverse_icon_chain_3`: Library code that calls another routine, and if the result is non-zero, performs a series of conditional calls to other routines using various parameters and offsets before returning a value.
- `move_d6_to_d0_and_branch`: Moves the contents of register d6 to d0 and branches to another location.
- `call_lib_func_and_return_zero`: Invokes a function pointer retrieved from a global variable associated with library index -210.
- `incremental_offset_recurse_alt`: Calls another routine with the same first argument and the second argument increased by a constant of 78, then returns.
- `search_string_list`: Calls another routine, then enters a loop that iterates through a list of pointers.
- `clear_d0_and_restore_registers_7`: Clears register d0 and restores d2, d3, a2, and a3 from the stack before returning.
- `process_table_indices_2`: Library code that enters a loop that calls another routine and compares the result to a value in register d5.
- `process_table_indices`: Enters a loop that calls another routine and compares the result to a value in register d5.
- `increment_and_store_d2`: Tests register d4, and if non-zero, increments the value in d4 and stores the result in register d2, otherwise it returns.
- `clear_d2_and_branch`: Clears register d2 and branches to another location.
- `save_registers_and_return`: Clears register d0 and saves registers d2 through d6 from the stack before returning.
- `parse_numeric_string_2`: Library code that updates a value by adding a product of a register and ten while incrementing a pointer in a loop.
- `prepare_stack_and_call`: Library code that increments a value in a2, pushes two pointers onto the stack, calls a routine, and then either sets d5 to one and branches or returns.
- `clear_d5_register`: This library code clears register d5 and branches to another location.
- `parse_numeric_string`: Enters a loop that updates a value by adding a product of a register and ten, while incrementing a pointer.
- `compare_and_calculate_offset`: Compares a value from the stack with a constant, and based on the comparison, either sets d0 to a specific constant, sets d0 to the stack value minus 48, or sets d0 to the stack value.
- `set_d0_to_negative_one`: Sets register d0 to negative one and returns.
- `call_library_vector_198_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `complex_library_traversal`: Library code that calls another routine and uses the result to conditionally call a second routine or perform a cleanup call.
- `execute_conditional_library_call`: Calls a library function using a base address from a global variable, and if the result is non-zero, stores the result at offset 14 of a structure in a6, updates a pointer in a parameter, and calls another routine.
- `restore_registers_and_return_4`: Clears d0 and restores d2, d3, and a2 from the stack before returning.
- `call_and_verify_parameter`: Calls another routine and, if the return value does not match the fourth parameter, sets the return value to zero.
- `move_d1_to_d0_and_restore`: Moves the contents of d1 to d0 and restores d2 and d3 from the stack before returning.
- `compare_and_validate_result`: Calls another routine and compares its return value to a fourth parameter; if the values do not match, it returns zero, otherwise it returns the result of the called routine.
- `restore_registers_and_return_5`: Moves a longword from register d1 to d0 and restores registers d2 and d3 from the stack before returning.
- `divide_32_bit_integers_3`: Performs 32-bit division of two 32-bit values, using a loop to rotate bits and subtract a threshold if the high half meets or exceeds it, or uses a different method if the divisor is within a specific range.
- `rotate_and_threshold_32bit_value_2`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_8`: Computes the 32-bit product of two 32-bit integers by summing the results of four 16-bit partial products.
- `invoke_table_entry_3`: Loads a library base from a global variable and calls a function pointer located at an offset of 522 bytes from that base, passing three arguments from the stack.
- `store_byte_and_increment_pointer_2`: Stores a byte from register d0 into the memory location pointed to by register a3, then increments the pointer in a3 by one before returning.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `search_table_byte`: Library code that loads a pointer from offset 4 of the stack into a0 and a value from offset 8 of the stack into d0, then compares a byte from the pointer in a0 with d0, decrementing the pointer and updating d0 if they match before returning.
- `branch_to_offset_5`: Library code that clears register d0 and branches to a specific offset.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `skip_zero_byte`: Loads registers a0 and a1 from the stack, checks if the byte at the pointer in a0 is zero, and if not, decrements a0 and branches to another routine.
- `compare_byte_at_offset`: Decrements a value from a parameter, and if it is not negative, compares a byte from a pointer in a1 with a byte from a pointer in a0, and if they are equal and the byte in a1 is zero, sets d0 to zero.
- `set_d0_clamped`: Sets register d0 to 1 if it is greater than zero, otherwise sets d0 to 255 and returns.
- `test_and_decrement_d0`: Tests a byte at the address in a0, and if it is non-zero, decrements a value in d0 and branches based on whether the result is greater than zero.
- `decrement_pointer_and_call`: Decrements the address in register a0 and calls another routine.
- `copy_bytes_with_counter`: Decrements a counter in d0 and copies a byte from the address in a1 to the address in a0 if the counter is not below zero, repeating this in a loop until the counter reaches zero or the byte is zero, then clears a byte at the address in a0 and branches to another routine.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `update_linked_list_node`: Stores a pointer into the memory location pointed to by the argument, increments the value at that location by four, clears the word at offset four from that location, and stores the original pointer into the word at offset eight from that location.
- `conditional_library_branching`: The library code checks a parameter and, if non-zero, calls another routine and potentially calls a third routine using a value from a pointer offset.
- `iterate_linked_list_nodes`: Enters a loop that checks a parameter, calls another routine with offset 16, and if the result is non-zero, calls another routine using a value from a pointer offset and a calculated multiplier, then updates the parameter from a pointer at offset 12.
- `set_d0_and_branch_2`: Sets d0 to one and branches to another location.
- `restore_registers_and_return_alt`: Clears d0 and restores d2, d3, d4, d5, and a2 from the stack before returning.
- `process_icon_structure`: Library code that calls several other routines to perform calculations and stores a result in a structure at offset 10.
- `return_negative_one_2`: This library routine sets register d0 to negative one and returns.
- `iterate_icon_linked_list`: Library code that executes a nested loop, calling other routines to update a value at offset 12 of a structure, calculate a value based on a byte at offset 7, and store a result at offset 8 of a structure.
- `iterate_linked_data`: Library code that executes a nested loop, calling other routines and updating values at various offsets in a structure.
- `move_a3_to_d0_and_branch`: Moves the longword from a3 into d0 and branches to another location.
- `return_negative_one_3`: Sets the d0 register to negative one and restores the previous register states from the stack before returning.

### translator.library

- `execute_library_callback`: Checks a global variable and calls a function through a pointer at an offset from that variable; if the call returns non-zero, it stores the global variable and a value from a register into a structure and calls another routine before returning a specific constant.
- `clear_flag_and_increment_counter_2`: Increments a 16-bit counter in the structure it is given and clears bit 5 of that structure's flag byte.
- `decrement_counter_call_on_zero`: Decrements a 16-bit counter in the structure it is given.
- `update_status_bit`: Saves registers to the stack, moves a pointer from a6 to a3, loads a new address into a6 from the stack, checks a 16-bit word at offset 32 of the structure in a3, and if it is zero, branches; otherwise it sets bit 5 of a byte at offset 14 of the structure in a3 and branches.
- `unlink_node_and_call_library`: Removes the node from its doubly linked list, then calls a function through the Amiga library base and returns a value stored in the node.
- `return_constant_63`: Returns the constant an address in d0 to the caller without any other effect.

The function list continues in: g1447-recovery-functions-2.md.
