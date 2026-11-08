# Game 1329 (balance of power - the 1990 edition (1990)(mindscape)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01329). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916908 top-level bytes in 75 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 50 of 50 units identical (bytes and relocations), 55 raw-word lines |
| Functions named and described | pass, 2260 of 2260 named (278 author symbols), 2260 of 2260 described (427 by rule, 1728 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3299 of 3299 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 193177 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 34941 of 35164 instructions identical in start and length, 4 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 48803 |
| code | 117540 |
| data | 691469 |
| padding | 17624 |
| unused | 41472 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf:stack | 2 | 117 | 2 |
| Format | 30 | 1919 | 78 |
| DiskCopy | 30 | 2723 | 99 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
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
| 12 | 1 | 115 | 2 |
| 15 | 1 | 2 | 1 |
| 8 | 1 | 115 | 2 |
| 12 | 1 | 115 | 2 |
| 20 | 1 | 115 | 2 |
| 11 | 1 | 2 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf:CLI | 9 | 362 | 18 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 15 | 1414 | 108 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2476 | 170 |
| mathieeedoubbas.library | 3 | 632 | 60 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 13 | 830 | 53 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf:bop | 3 | 5064 | 305 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1700 | 37 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 150 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf bytes 603136-603648 (code never loaded by a reference run) | 1 | 28 | 1 |
| SavedGame (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- balance of power - the 1990 edition (1990)(mindscape)[a]/Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf: file devs/printers/Okidata_92 (header 5476 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1478 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 93 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 20 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- balance of power - the 1990 edition (1990)(mindscape)[a]/Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf: bytes 603136-603648 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[a]/Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf:asave/SavedGame: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[a]/Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf: 1 logged loads cover 512 bytes of the image
- balance of power - the 1990 edition (1990)(mindscape)[a]/Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf: blocks 112-120 (bytes 57344-61952) outside the filesystem hold data, not code (opcode density 0.025, entropy 4.98)
- 1 never-loaded region(s), 192665 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf:stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of register d1 into register d0 and then branches to another routine.

### Format

- `parse_command_string`: Parses a command string by splitting it into tokens based on whitespace and handling quoted substrings, while performing character substitution for specific escape sequences.
- `execute_callback_chain`: Checks several global variables for non-zero values and executes corresponding function pointers stored at specific offsets.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `call_callback_chain`: Loads a value from a memory offset into a1, sets d0 to zero, calls a routine at an offset from a6, stores the result in a global variable, and returns if d0 is zero.
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
- `execute_callback_chain_2`: Checks five global variables for non-zero values and, if non-zero, calls the corresponding function pointer with specific constant arguments or no arguments.
- `traverse_and_invoke_table`: Iterates through a table of pointers, checks for null or empty entries, and calls a routine to compare strings and another routine to invoke a table entry.
- `report_disk_error`: Formats a disk by calculating cylinder and sector counts, allocating a memory buffer, and writing a pattern of bytes, then uses a formatted string to report device failures or disk errors.
- `measure_string_length`: Returns the number of characters before the terminating zero byte in the given string.
- `copy_string`: Copies a null-terminated string from a source pointer to a destination pointer.
- `convert_to_uppercase`: Returns the input byte as is if it is outside the lowercase alphabet range, otherwise it subtracts 32 from it.
- `compare_strings_case_insensitive`: Compares two strings byte by byte by converting each byte to uppercase via another routine, returning 1 if they match and 0 otherwise.
- `truncate_at_colon`: Iterates through a string pointed to by a register until it finds a colon or a null terminator, then replaces that character with a null terminator.
- `manage_task_structure`: Allocates a structure, populates it with various values including a pointer from a parameter, and performs several library calls including a task-related operation.
- `save_and_reset_global_value`: Fetches a value from a structure via a function call, stores it in a global variable, and then writes a constant to the original structure location.
- `restore_global_value`: Fetches a value from a structure via a function call and restores a previously saved value from a global variable into that structure location.
- `initialize_stack_data`: Populates a stack-based structure with several constant values and then calls another routine with a pointer to a global variable.
- `execute_library_calls`: Calls several functions from a library base and performs a write operation using a DOS library base.
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
- `pass_params_to_subroutine_2`: Calls another routine with a global value, its parameter, and the address of an eight-byte stack slot.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a value from a stack slot or a negative error code.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_global_library_function`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `rotate_32bit_value_3`: Performs a bitwise rotation on a 32-bit value using a loop that runs sixteen times, or performs division and modulo operations if a specific condition is met.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment`: Stores a byte from register d0 into the memory location pointed to by a3 and then increments a3.
- `format_disk_and_verify`: Formats a disk by calculating cylinder and sector counts, allocating a memory buffer, and writing a pattern of bytes including a specific signature to the media.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `execute_complex_logic`: Performs a sequence of assignments to various memory locations and global variables, then calls a routine with three identical arguments and another routine with a single argument.
- `create_std_io`: The routine calls another routine to create an extended I/O structure for the supplied pointer, passing a constant size argument of 48.
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

- `parse_command_string_2`: Parses a string into individual tokens by handling spaces, tabs, and double quotes, while performing specific character replacements for certain escape sequences.
- `execute_library_callbacks`: Stores a parameter into a location at an offset from a global variable, then calls a function from a global variable if it is non-zero, and calls two other functions from a global base if a second global variable is non-zero.
- `invoke_two_function_pointers_2`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `call_global_callback`: Stores a value from a register into a global variable at an offset of 8, then calls a function from a global variable if the result of the call is zero.
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
- `update_structure_fields_3`: Stores a 16-bit value at offset 28, a 32-bit value at offset 40, and a 32-bit value at offset 36 of a structure, then calls another routine.
- `update_structure_fields`: Stores a 16-bit value at offset 28, a 32-bit value at offset 40, and a 32-bit value at offset 36 of a structure, calls another routine, and then adds the result of that call to a value at offset 44 of the structure.
- `synchronize_disk_contents`: Performs a disk-to-disk copy operation by iterating through file structures on a source disk and replicating them to a destination disk.
- `synchronize_disk_contents_2`: Performs a disk-to-disk copy operation by iterating through file structures on a source disk and replicating them to a destination disk.
- `execute_cleanup_sequence`: Checks a global variable.
- `cleanup_system_resources`: Checks several global pointers and executes various cleanup routines, including deleting port structures, deleting extended I/O, calling library functions to free memory or close libraries, and managing error message tasks.
- `extract_message_data`: Copies three values from a structure at the address in a global variable into other global variables, calls another routine with that structure, and then calls another routine with a global variable if a specific value is not 512.
- `execute_library_call`: Loads a value from the stack into d0, pushes a value from a global variable and d0 onto the stack, calls a routine at an offset from a global variable, and if the result is not zero, it branches.
- `perform_disk_copy_loop`: Iterates through a table of data blocks to perform disk I/O operations, including reading from a source and writing to a destination.
- `allocate_multiple_libraries`: Checks a value at an offset of 20 from a global variable; if it is less than 33, it calls a routine with a fixed address and then calls a routine with 0.
- `initialize_disk_copy_params`: Populates a stack-based structure with several constants and then calls two library functions using a global base.
- `report_disk_errors_2`: Sets a global pointer to a global variable and calls another routine with various parameters based on the value of a byte parameter, then calls another routine with 40 and 50, and finally calls another routine with 50.
- `report_disk_errors`: Sets a pointer to a global variable and calls another routine with a global value, a parameter, and a stack address, then repeats this with a different constant and calls another routine with a single constant before returning.
- `process_io_and_status`: Calls a routine and checks the result; if the result is zero, it calls another routine with a pointer from a structure and a value from a table.
- `check_signal_status`: Calls another routine with two parameters, checks if the 12th bit of the result is set, and if so, calls another routine with a global variable; otherwise, it returns the bit status.
- `process_message_queue`: Checks a global variable and either enters a loop to process messages or extracts data from a structure pointed to by a global variable, then returns a value based on the contents of that structure.
- `set_parameter_zero`: Stores zero in a word at an offset from a pointer and nine in a word at a different offset before calling another routine.
- `manage_error_message_task`: Allocates memory, populates the resulting structure with various values and pointers, calls a routine to handle the structure, and then frees the memory.
- `set_parameter_and_call`: Stores two different word values into an offset of a provided pointer and calls another routine multiple times.
- `execute_multiple_calls`: Pushes two global longwords onto the stack and calls a routine twice for each value, then cleans up the stack and returns.
- `process_disk_data`: Calls another routine with a constant and a value, then if the result is non-zero, it performs several calls using a pointer from a global variable, updates a global variable with a pointer, modifies a byte at an offset of the pointer, calculates a sum by looping through 128 words in a table, subtracts that sum from a value at offset 5 of the pointer, and calls another routine with a constant and the pointer.
- `perform_io_operation`: Calls another routine with four parameters, then calls another routine and returns whether its result is zero.
- `compare_and_io`: Performs an I/O operation, calls another routine to update a structure, and returns whether a parameter matches a value stored in a global variable.
- `allocate_message_entry_2`: Attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port_structure`: Checks if a value at offset 10 of the passed structure is non-zero; if so, it recurses.
- `allocate_io_request_structure`: If the first argument is non-zero, it calls a routine to allocate memory with a specific constant, then populates the resulting structure by storing the value 5 at offset 8, the second argument as a 16-bit word at offset 18, and the first argument as a 32-bit word at offset 14; otherwise, it returns zero.
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
- `calculate_string_length_2`: Increments a counter while scanning through a null-terminated string starting at the address provided in a stack slot and returns the count.
- `copy_memory_block`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `execute_indirect_calls_2`: Fetches a function pointer from a global structure and executes it multiple times with varying parameters.
- `invoke_indirect_callbacks`: Calls another routine with a pointer to a global variable and a parameter, then calls another routine with four parameters including a local stack array.
- `convert_char_to_uppercase`: Returns the character with lowercase ASCII letters converted to uppercase by subtracting 32.
- `compare_strings_case_insensitive_2`: Compares two null-terminated strings by converting characters to uppercase and returns 1 if they match, otherwise it returns 0.
- `terminate_colon_string`: Searches for the end of a null-terminated string and, if the last non-null character is a colon, replaces it with a null terminator.
- `invoke_pointer_and_checksum_stack`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer`: Stores a byte from register d0 at the address pointed to by a3 and increments a3.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag_2`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag_2`: Calls modular_inverse_32bit and returns its first result, or the negation of that result if the second result in an address register is nonzero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer`: Stores a byte from register d0 into the memory location pointed to by register a3 and increments the pointer by one before returning.
- `call_formatted_print`: Pushes a pointer and a value onto the stack and calls a routine to print a formatted message, then returns the result in a register.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_global_library_function_via_offset`: Calls a function through a pointer located at an offset of 348 from a library base stored in a global variable, then returns.
- `execute_with_status_check_2`: Calls a routine via a global pointer and returns either a byte from a stack slot or a negative error code.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.

### Port-Handler

- `parse_amiga_hunk_data`: Parses a data structure containing AmigaDOS hunk type codes to determine relocation or execution requirements, clears a memory range via a loop, and performs conditional data copying and register updates based on the hunk type.

### Disk-Validator

- `update_display_buffer`: Converts a numeric value into a character string and stores it in a table, then performs a series of bitwise operations and loops to populate a data structure with bit-shifted values and flags based on a counter.

### Ram-Handler

- `execute_jump_table`: Sets d0, saves registers, calls a routine using a pointer from a6 offset -656, restores registers, and performs an indirect jump using a pointer from a6.

### serial.device

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reinit_serial_buffer_fields`: If a byte in a1 is non-zero, it calls a routine via a pointer in a6; it then checks if d2 is non-zero to call another routine, updates several fields in a structure at a6 with the value of d2, and sets two fields at offsets 198 and 202 to half and quarter of d2 respectively.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `set_byte_and_branch`: Stores the byte 253 at offset 31 of a structure in a1 and then branches to another location.
- `toggle_interrupt_enable`: Writes 801 to the INTENA register, calls another routine, writes 8801 to the INTENA register, and returns.
- `clear_and_compare_status`: Clears bit 7 of a byte at offset 31 of a1 and branches to different locations based on whether the word at offset 28 of a1 equals 2, 3, or 10.
- `compare_and_clear_pointer`: Compares a longword at an offset of a6 with a1; if they are equal, it clears bit 1 of a byte at an offset of a6, clears the longword at that same offset in a6, and then branches.
- `unlink_list_node`: Checks bit 6 of a byte at offset 30 of a structure in a1; if set, it unlinks the structure from a list by updating pointers and decrements a 16-bit value at offset 70 of a structure in a6, then branches to another routine.
- `reset_serial_hardware`: Compares a1 with a value at offset 78 of a6; if they match and a 16-bit value at offset 28 of a1 is 10, it clears bit 5 of a byte at offset 470 of a6, writes values to hardware registers at offset a fixed address, calls another routine, clears a value at offset 78 of a6, clears bit 4 of a byte at offset 472 of a6, and branches to another routine.
- `unlink_second_queue_request`: If bit 6 is set in a byte at offset 30 of a structure in a1, it unlinks the structure from a list, decrements a 16-bit value at offset 72 of a structure in a6, updates several status bits and a byte at offset 31 in the structure in a1, and calls a routine via a pointer in a6 if bit 0 is not set.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `set_status_flag_2`: Sets register d0 to zero, checks a 16-bit value at offset 32 of a structure in a6, and if non-zero, sets bit 6 of a byte at offset 472 of that structure before returning.
- `invoke_library_vectors_then_unlink`: Calls several library vectors using pointers derived from a structure in a6, copies a long value from offset 426 of a6, and unlinks a block from a list by updating pointers in a6.
- `insert_node_into_linked_list`: If a value in a structure is non-zero, it clears a bit in a status byte, sets another bit, inserts a new node into a linked list, increments a counter, and calls a handler.
- `update_serial_buffer_status`: Iterates through a linked list of data buffers to update status flags and counters within a structure pointed to by a6.
- `process_serial_data_buffer_stream`: Iterates through a linked list of data buffers to process bytes, updating status flags and counters within a structure pointed to by a6.
- `test_status_bit`: Tests bit 0 of a byte at offset 443 of a6 and branches forward if the bit is clear, otherwise it branches to a different location.
- `call_and_set_byte`: Calls another routine, and if the result is zero, it sets a byte at an offset of a1 to nine and branches.
- `process_serial_data_buffer`: Iterates through a linked list of data buffers to process bytes, updating status flags and counters within a structure pointed to by a6.
- `restore_registers_and_return_10`: Restores d2 and a6 from the stack and returns.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `clear_status_bit_2`: Clears bit 2 of a byte at offset 470 of a6; if the bit was clear, it moves a byte with value 15 to offset 31 of a1 and branches, otherwise it branches to another routine.
- `clear_bit_and_set_byte`: Clears bit 7 of a byte at offset 470 of a6 and branches if the bit was already clear, otherwise it sets a byte at offset 31 of a1 to 9 and branches.
- `update_status_and_clear_field`: Sets a byte at an offset of a1 to twelve, tests bit 0 of a byte at an offset of a6, and if it is zero, sets a byte at an offset of a1 to six and clears a longword at an offset of a6 before branching.
- `manage_serial_buffer_and_interrupts`: Checks status bits in a structure to manage a circular buffer, updates a pointer and a counter when data is available, and calls another routine or a handler via a function pointer.
- `toggle_serial_interrupt_state`: Compares a byte at offset 34 of a structure in a6 with a register, and if they match, checks bit 1 of a byte at offset 472 of the structure; if that bit is set, it clears the bit and writes a value to the interrupt enable register to disable interrupts, otherwise it returns.
- `test_serial_status_bits`: Tests bit 5 of a byte at offset 472 of a6, then tests bit 1 of a byte at offset 445 of a6, calls another routine, and finally tests bit 8 of d0 before returning.
- `test_bit_and_branch_3`: Tests bit 8 of d0; if the bit is set, it branches to another routine, otherwise it branches to another routine.
- `test_bit_and_branch_2`: Tests bit 0 of a byte at offset 443 of the structure in a6 and branches to another routine.
- `set_bit_and_copy_pointer`: Sets bit 7 of a byte at offset 470 of a6 and checks if a longword at offset 62 of a6 is zero; if it is, it copies a longword from offset 42 to offset 62 and branches.
- `process_buffer_data_2`: Compares a longword at offset 54 of a6 with a longword at offset 58 of a6; if the latter is smaller, it writes a byte to a location pointed to by a pointer at offset 42 of a6, increments two values at offsets 58 and 42, and potentially updates a pointer at offset 42 from offset 38, otherwise it updates a longword at offset 62 if it is zero, and finally calls a routine via a pointer at offset 98.
- `record_pointer_then_signal`: Checks if a longword at an offset of a6 is zero; if so, it copies a longword from another offset in a6 to that location, calls a helper routine, then calls a library vector using a pointer from a structure in a6.
- `modify_status_and_register`: Moves a byte from an offset of a6 to an offset of a6, clears bit 3 of a byte at an offset of a6, tests bit 2 of a byte at an offset of a6, and if it is zero, clears a longword at a specific address and branches.
- `update_hardware_status`: Moves a byte from offset 35 of the structure in a6 to offset 473 of the structure in a6, sets bit 2 of offset 472 of the structure in a6, tests bit 2 of a byte at offset 69 of a6, and if bit 2 is clear, sets bit 6 of a hardware register; otherwise, if bit 7 of the byte at offset 69 of a6 is set, it writes a word to a hardware register and returns.
- `clear_byte_at_offset`: Clears a byte at offset 301 of the structure pointed to by a6 and returns.
- `signal_pointer_update`: Checks if a byte at offset 294 of a structure in a6 has bit 6 set; if not, it sets that bit, clears a longword at offset 418, copies a longword from offset 106 to offset 418, subtracts 10 from that longword, sets a 16-bit value at offset 414 to 9, calls another routine, and if a byte at offset 31 of a register is non-zero, sets a byte at offset 31 of another register to 11.
- `modify_status_flags`: Saves d2, a3, and a6 to the stack, clears bit 6 of a byte at offset 470 of a6, and if the bit was already clear, it performs bitwise operations on a word from the hardware register a fixed address and branches, otherwise it sets bit 2 of a byte at offset 470 of a6 and branches.
- `load_address_and_call`: Loads the address of a value at offset 418 of the structure in a6 into a0, calls another routine, and then branches to another routine.
- `set_status_bit_5`: Sets bit 0 of a byte at offset 470 of a6 and branches to another routine.
- `manage_serial_buffer_state`: Clears a value at offset 32 of a structure, then either updates a linked list of buffers and increments a counter or updates a status byte and stores a pointer and a value into a structure.
- `update_interrupt_state`: Tests bit 2 of a byte at offset 69 of a structure in a6; if set, it calls another routine, otherwise it tests bit 0 of a byte at offset 31 of a structure in a1, sets bit 4 of a byte at offset 30 of that structure, clears a 32-bit value at offset 94 of the structure in a6, copies a 32-bit pointer from offset 40 of the structure in a1 to offset 86 of the structure in a6, and writes 32769 to two hardware registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_request_state_on_flags`: Checks if two specific bits in a global variable are set, and if so, writes a byte with a value of thirteen to an offset within a structure pointed to by a register.
- `manage_serial_request_queue`: Processes a linked list of serial request nodes by updating pointers and bit flags, decrements a counter, and calls another routine if a specific condition is met.
- `update_serial_counter`: Writes 200 to offset 476 of a6, loads a pointer from offset 86 of a6 into a0, clears d0, loads a byte from the pointer into d0, loads a longword from offset 36 of a1 into d1, tests bit 4 of offset 472 of a6, and decrements a longword at offset 94 of a6 before branching.
- `update_serial_status_and_call_handler_2`: Updates a bitmask in a structure, increments two counters, performs a comparison loop against a byte sequence, and calls a function via a pointer.
- `update_serial_status_and_call_handler`: Updates a bitmask in a structure, increments two counters, performs a comparison loop against a byte sequence, and calls a function via a pointer in the structure.
- `process_linked_list_node_2`: If a 16-bit value at an offset of a6 is non-zero, it manipulates a linked list by updating pointers and bit flags within the list nodes, updates a longword at an offset of a6, and calls another routine if a specific byte in a node equals ten.
- `set_interrupt_mask`: Restores d2 and a6 from the stack, writes 1 to a hardware register, and returns.
- `update_serial_interrupt_status`: Enables the serial interrupt by writing to the interrupt request register, reads two bytes from a structure in a1, combines them into a 16-bit value, clears the first byte in the structure, writes the combined value to the custom chip's DMA control register, and otherwise returns.
- `synchronize_serial_device`: Writes to the INTENA register, calls other routines to clear fields, writes several longwords to a structure, and calls a handler routine via a pointer.
- `call_handler_and_branch`: Calls another routine and then branches to another location.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `set_flag_and_call_handler`: Sets bit 2 of a byte at an offset of a6 if a longword at an offset of a6 is non-zero, and if a longword at another offset of a6 is non-zero, it calls a routine and then calls a library vector if a bitwise check on a byte in a1 passes.
- `clear_interrupt_and_call_handler`: Clears bit 3 of a byte at offset 472 of a structure in a6, then if a longword at offset 74 of a6 is non-zero, calls another routine, sets a global variable to 32769, and if a byte at offset 30 of a register has bits 0 and 8 set to zero, calls another routine.
- `update_mask_and_call_handler`: Writes 2049 to a hardware register, calls another routine, writes 34817 to a hardware register, and then branches to another location.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `copy_and_update_structure`: Copies a 32-bit value from a structure in a1 to a structure in a0; if the value is not zero, it exchanges d0 and a1, stores a0 at offset 4 of the structure in a1, and if d0 is not zero, it sets a1 to d0 and calls another routine; it also contains a loop that branches back to the start of the routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_device_status_and_call_handler`: Updates a status byte and a 16-bit value in a structure via a pointer, modifies a byte in another structure, and calls a handler routine if a specific bitmask check passes.
- `test_buffer_and_pointer_status`: Clears bit 0 of a byte at an offset of a1, tests bit 3 of a byte at an offset of a1, and if it is set, checks if a 16-bit value or a longword at specific offsets of a6 are non-zero.
- `handle_request_and_interrupt`: Moves a longword from offset 106 of the structure in a6 to d0, calls another routine, tests a byte at offset 31 of a1, and if it is non-zero, moves a pointer from a1 to offset 78 of the structure in a6, sets bit 5 of offset 470 of the structure in a6, and writes a word to a hardware register before branching to another routine.
- `reply_and_clear_active_request`: Pushes a6 to the stack, loads a pointer from offset 98 of the structure in a6, calls another routine using that pointer, restores a6 from the stack, clears a 32-bit value at offset 78 of the structure in a6, and returns.
- `clear_uart_break_and_reply`: Clears bit 5 of a byte at offset 294 of a structure in a1; if bit 5 is set, a6 is not zero, and a 16-bit value at offset 28 of the structure pointed to by offset 78 of the structure in a1 is 10, it sets bit 1 of the byte at offset 294, clears a 32-bit value at offset 78, writes 2048 to a hardware register, and calls another routine; finally, it writes 32769 to a hardware register and returns.
- `start_timed_read_request`: Sets a global word to one, clears two 16-bit fields at offset 322 and 326 of a structure in a6, sets a 16-bit value at offset 318 to nine, calls a library function, and sets a status byte to eleven if the call returns a non-zero result.
- `modify_serial_flags`: Tests bit 7 of a byte at offset 79 of a1, sets bit 7 of a byte at offset 69 of a6, clears bit 2 of a byte at offset 472 of a6, tests bit 3 of offset 472 of a6, and calls another routine.
- `validate_serial_status_and_set_flag`: Clears bit 7 of a byte at offset 69 of a structure, checks bit 1 of a byte at offset 474 and a longword at offset 78, and checks bit 4 of a byte at offset 477; if a longword at offset 70 is non-zero, it sets bit 0 of a byte at offset 31 of a structure and otherwise returns.
- `toggle_interrupt_and_validate`: Writes 2049 to the INTENA register, calls another routine, writes 34817 to the INTENA register, and if a byte at offset 30 of a register has bits 0 and 8 set to zero, calls another routine.
- `validate_and_sync_serial_parameters`: Validates a sequence of three characters and a longword within a structure, then copies these values and several status flags into a second structure.
- `compute_unit_channel_masks`: Calculates a bitmask based on a byte at offset 77 of a structure in a1, modifies it based on a byte at offset 78 and a bit in a structure in a6, stores the result in a structure in a6, and calls another routine twice to store two results at offsets 464 and 466.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_byte_and_branch_2`: Sets bit 2 of a byte at offset 31 of the structure in a1 and branches to another routine.
- `calculate_period_quotient`: Loads a longword from offset 450 of the structure in a6 into d0 and d1, performs arithmetic operations including a subtraction and a division by a constant, and stores the resulting 16-bit quotient in d1 after applying bitwise masks and shifts.
- `store_serial_period_quotient`: Divides a 16-bit value in d1 by a 16-bit value in d0, stores the quotient in a structure in a6 and a global variable, clears bit 5 of a byte at offset 472 of the structure, and if a byte at offset 66 is eight and bit 0 of a byte at offset 69 is set, performs a bitwise OR on the quotient and sets bit 5 of the byte at offset 472 before writing the result to the serial period register.
- `configure_serial_device`: Calls a routine using a pointer from a structure in a6, and if the result is non-zero, stores a0 at offset 266 of that structure, stores a6 at offset 98, and stores 200 at offset 476.
- `update_serial_pointers`: This routine loads two different addresses into a0 and a3 based on a6, calls another routine twice with those addresses, and then branches to another routine.
- `copy_pointer_to_offset`: Stores the pointer in a0 to offset 16 of a structure in a1.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `configure_serial_device_state`: Updates several status bytes and pointers within a structure pointed to by a6, calls a library function via a pointer, and sets a global variable to a specific value before returning.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_and_branch`: Clears the d0 register and branches to another location.

### parallel.device

- `update_parallel_device_status`: Checks specific bits in two structures, increments a 16-bit counter if certain conditions are met, synchronizes a status byte, sets a bit in a status byte, and conditionally copies two 32-bit values between structures.
- `increment_counter_and_copy`: Increments a 16-bit value at offset 32 of a structure in a6, copies a byte from offset 117 of a structure in a6 to offset 53 of a structure in a1, sets bit 4 of a byte at offset 118 of the structure in a6, and if bit 1 of the byte at offset 53 of the structure in a1 is set, copies two 32-bit values from offsets 90 and 94 of the structure in a6 to offsets 54 and 58 of the structure in a1.
- `update_status_and_copy_data`: Updates a status byte based on a bitmask in a structure, increments a 16-bit counter if two specific bytes are zero, and conditionally copies two 32-bit words from one structure to another.
- `process_callback_and_status`: Calls another routine if a bitmask check on a byte in a1 passes, then checks a 16-bit value at offset 28 of a structure in a1 to either modify a global variable and a byte at offset 118 of a structure in a6, or call another routine if a 16-bit value at offset 28 is three and a global variable bit is clear; finally, it calls another routine using a pointer from a structure in a6.
- `execute_conditional_callback`: If a 16-bit value at offset 28 of a structure in a1 is three, it clears bit 0 of a global variable and, if bit 0 of that global variable is zero and bit 2 of a byte at offset 118 of a structure in a6 is not set, calls a routine via a pointer; it then sets bit 2 of the byte at offset 118 and calls another routine via a pointer.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `process_callback_status`: Writes the byte 253 to offset 31 of a structure in a1 and branches to another routine.
- `check_long_and_clear`: Tests a longword at offset 36 of a structure in a1; if it is non-zero, it branches to another routine, otherwise it clears a longword at offset 32 of the structure in a1 and branches to another routine.
- `update_device_status_2`: Clears bit 1 of a byte at offset 30 of a structure in a1, then checks bit 2 of a byte at offset 118 of a structure in a6.
- `update_device_status`: Sets several fields in a structure, modifies a global variable, and performs bitwise operations on memory values.
- `process_parallel_device_queue`: Manages a linked list of device requests by decrementing a counter and updating pointers, then performs a comparison loop against a buffer of bytes.
- `update_parallel_device_counters`: Increments two counters in a structure, updates a value from a pointer, and performs a loop to compare bytes in a buffer.
- `modify_device_state`: Copies a 32-bit value from offset 46 of a structure in a6 to offset 32 of a structure in a1, clears bit 3 of a byte at offset 118 of a6, and clears bit 5 of a byte at offset 30 of a1.
- `test_cia_and_bit`: Reads a byte from the CIA register at offset 101 of the hardware address space into a register; if it is non-zero, it branches to another routine, otherwise it tests bit 3 of a byte at offset 118 of a structure in a6 and branches accordingly.
- `execute_linked_list_update`: Calls another routine, then calls a routine via a pointer at an offset from a structure in a6, sets a byte at offset 118 of a structure in a6 to zero, and branches to another routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_if_not_zero`: Checks if a value in a structure is non-zero and calls a function through a pointer if it is.
- `invoke_callback_pointer`: Executes a function through a pointer stored in a structure if the associated index is not zero.
- `process_linked_list_node`: Loads a pointer from offset 76 of a structure in a6 into a1, moves the longword at a1 into d0, and if d0 is not zero, stores d0 back at offset 76 of a6 and calls another routine via a bsr.
- `clear_word_at_offset`: Clears a 16-bit value at offset 114 of a structure in a6 and returns.
- `modify_status_byte`: Sets bit 5 of a byte at offset 30 of a structure in a1, sets a byte at offset 31 of the structure to 254, and calls a routine via a pointer stored in a structure in a6.
- `update_global_and_field`: Manipulates bits within a global variable and updates a byte in a structure using a library function call.
- `check_bit_and_set`: Tests bit 1 of a byte at offset 118 of a structure in a6; if it is clear, it writes the byte 1 to offset 31 of a structure in a1 and branches to another routine.
- `copy_structure_fields`: Copies a byte from one structure to another and, if a specific bit is set, copies two long words as well.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `recurse_on_pointer`: Saves registers a0 and a6 to the stack, calls itself via a pointer, and then restores the registers and returns.
- `recursively_calls_itself`: This routine performs an infinite recursive call to itself, or otherwise acts as an empty wrapper that calls itself before returning.
- `call_clipboard_cleanup`: Pushes the contents of a1 onto the stack, calls a routine to decrement and clean up a clipboard structure, restores the stack pointer, and returns.
- `jump_to_handler`: Performs a long jump to an address.
- `decrement_and_check_counter`: Pushes a1 onto the stack, calls a routine, and then increments the stack pointer by four.
- `call_and_restore_stack`: Pushes the contents of a1 onto the stack, calls another routine, and then adds 4 to the stack pointer.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `execute_library_call_loop`: Saves registers d2 through d3 and a2 to the stack, loads a longword from the stack into d3 and another longword into a2, stores a2 in a global variable, clears the stack, pushes a pointer to offset zero, calls another routine, stores d0 in a global variable, adjusts the stack pointer, and branches to another routine if d0 is non-zero.
- `execute_conditional_logic`: Stores the value of d3 into a global variable, then calls three different routines using the values in a2 and other registers, returning the result of the third call.
- `call_two_functions`: Calls two different functions via library vectors and returns one.
- `update_clipboard_data`: Calls a routine, and if the result is zero, sets a 16-bit value at offset 94 of a structure in a1 to zero.
- `clear_and_call_routine`: Clears a long word at an offset from a pointer in a3, pushes a3 to the stack, and calls another routine using a pointer at offset 72 of a2.
- `zero_word_field`: Stores a zero word into a specific offset within a structure and then calls another routine.
- `process_pointer_and_string`: Saves registers d2 and a2 to the stack, loads a pointer from offset 12 of the stack into a2, calls another routine, loads a pointer from offset 72 of the structure in a2 into a0, loads a longword from the address in a0 into d2, and if d2 is zero, pushes a pointer to offset 530 of the stack, pushes a pointer from a0 to the stack, calls another routine, and branches to another routine.
- `update_pointer_chain`: Pushes a2 onto the stack, calls a routine, clears a 32-bit value at offset 86 of a2, calls another routine, and then restores d2 and a2 from the stack.
- `copy_clipboard_buffer_segment`: Copies a specific range of bytes from a linked list of 512-byte buffers into a destination buffer, traversing the list to find the starting block based on a calculated offset.
- `compare_buffer_size`: Loads a long word from an offset of 90 in a2, compares it to a long word at offset 48 of the stack, and performs conditional branches based on the comparison results and a calculation involving an offset of 44 from the stack.
- `copy_clipboard_data`: Traverses a linked list to find a buffer matching a specific size, then copies bytes from a source to the buffer using loops that handle remaining data and buffer boundaries, and finally stores the resulting pointer in a structure.
- `compare_and_branch_pointer`: Copies a longword from d1 to d0, compares d0 with a longword at offset 14 of a structure in a1, and if d0 is lower, updates a1 with a pointer from that structure before branching.
- `compare_and_branch_index`: Copies a longword from d3 to d5, tests d5, and if it is non-zero, calculates a value by subtracting d2 from 512, compares d5 to that value, and if d5 is less than or equal to that value, moves d2 to d4 and branches to another routine; otherwise it moves d2 to d4 and branches to a different location.
- `copy_bytes_with_loop`: Copies bytes from a table starting at offset 18 of a structure in a1 to a destination in a3 using d4 as an index, looping until d4 reaches 512, then performs arithmetic on d0, d2, and d5 before branching.
- `transfer_index_and_branch`: Moves a 16-bit word from register d2 to d4 and branches to another routine.
- `copy_bytes_until_limit`: Copies bytes from a table starting at offset 18 of a structure in a1 to a destination in a3 using d4 as an index, looping until the value in d4 reaches the sum of d5 and d2, then clears d5 and branches.
- `store_long_word`: Stores a long word into a specific offset within a structure and then calls another routine.
- `validate_and_append_clipboard_data`: Checks if a clipboard buffer is valid and within size limits, then iterates through a linked list of memory blocks to verify data integrity or appends new data from a source pointer into the linked list structure, managing block offsets and remaining byte counts.
- `copy_clipboard_data_to_buffer`: Iterates through a linked list of memory blocks to copy bytes from a source buffer into the blocks, managing offsets and remaining byte counts within a structure.
- `call_buffer_append_routine`: Pushes a pointer from an offset of 72 in a structure into the stack, calls another routine, adjusts the stack pointer by 4, and branches to another location.
- `append_data_to_linked_buffer`: Traverses a linked list of buffers to find a suitable location based on a size value, then copies bytes from a source pointer into the buffers using a loop.
- `transfer_clipboard_buffers`: Locates a buffer in a linked list based on a size value, then copies bytes from a source to the buffer in a loop, updating the remaining count and the current buffer pointer as it traverses the list, and calls another routine.
- `copy_buffer_data`: Traverses a linked list of buffers while checking a size limit at offset 20 of a structure in a6, copies bytes from a source to the current buffer, and updates the structure and list pointers.
- `validate_buffer_size`: Tests d4, and if it is non-zero, tests a longword at offset 20 of a structure in a6; if that longword is greater than zero, it calculates a value by subtracting d3 from 512, compares a longword at offset 20 of a6 to that value, and if the longword is less than or equal to that value, moves d3 to d1 and branches to another routine; otherwise it branches to a different location.
- `copy_data_and_update_clipboard`: Copies bytes from a source to a destination at an offset of 18, updates a value in a structure, and calls update_clipboard_data.
- `transfer_d3_to_d1_and_branch`: Moves a 16-bit word from register d3 to d1 and branches to another routine.
- `copy_memory_buffer`: Copies bytes from a source pointer to a destination buffer at an offset of 18, using a loop that manages a counter and potentially calls another routine to fetch a new pointer.
- `store_register_to_offset`: Stores the contents of a data register into a memory location calculated by adding a fixed offset to a pointer held in an address register.
- `check_bits_and_measure_string`: Checks bit 7 and bit 0 of a byte at offset 30 of a structure in a2, and if neither bit is set, calls another routine to measure a string length.
- `modify_byte_bits`: Clears the least significant bit and sets the most significant bit of a byte at a specific offset in a buffer.
- `modify_byte_bits_2`: Checks bit 7 of a byte at offset 30 of a structure in a2, and if neither bit is set, calls another routine to measure a string len, then clears bit 1 or bit 3 of that same byte depending on whether a comparison between two parameters succeeds.
- `compare_structure_values`: Loads two pointers from a structure, compares a longword at offset 48 of the first pointer with a longword at offset 48 of the second pointer, and returns 1 in d0 if the first is smaller, otherwise it returns.
- `set_zero_and_branch`: Sets d0 to zero and branches to another location.
- `set_d0_to_one_and_branch`: Sets register d0 to one and performs a short branch to another location.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_structure_fields_2`: Copies several long and word values from one structure to another before calling another routine.
- `modify_clipboard_status_flags`: Sets bit 5 of a byte at offset 30 of a structure pointed to by a value at offset 12 of a stack frame, then checks a byte at offset 66 of a second structure pointed to by a value at offset 12 of the stack frame; if that byte is non-zero, it clears bit 0 of the byte at offset 30 of the first structure before jumping to another routine, otherwise it jumps to another routine.
- `synchronize_clipboard_buffer_state`: Updates a clipboard buffer structure by checking status flags and adjusting buffer lengths, offsets, and segment pointers based on the type.
- `clear_clipboard_register`: Clears a 16-bit register and branches to another routine.
- `process_clipboard_buffer_state`: Updates a clipboard buffer structure by checking status flags and adjusting buffer lengths, offsets, and segment pointers based on the type.
- `set_clipboard_flag`: Pushes the value of a2 onto the stack, calls another routine, pops the value from the stack back into a2, sets a 16-bit value at offset 94 of the structure in a2 to 1, and branches to another routine.
- `update_clipboard_buffer_state_4`: Updates a clipboard buffer structure by checking status flags and adjusting buffer lengths, offsets, and segment pointers based on the type.
- `clear_d3_and_branch`: Clears the word in d3 and branches to another routine.
- `update_clipboard_buffer_state_3`: Updates a clipboard buffer structure by checking status flags and adjusting buffer lengths, offsets, and segment pointers based on the type of data being processed.
- `manage_clipboard_buffer_state_2`: Updates the state of a clipboard buffer structure by checking status flags and adjusting buffer lengths, offsets, and segment pointers based on the current operation mode.
- `update_buffer_offset_2`: Loads a longword from offset 90 of the structure in a2 into d2, calculates a value by adding longwords from offsets 36 and 44 of the structure in a4 into d0, compares d0 to d2, and if d0 is less than or equal to d2, it moves the longword from offset 36 of the structure in a4 to offset 32 of the same structure before branching.
- `update_buffer_offset`: Loads a longword from an offset of 90 in a2 into d2, subtracts a longword from an offset of 44 in a4 from d2, stores the result in an offset of 32 in a4, and then adds the new value at that offset back to the longword at offset 44 in a4.
- `clear_and_process_buffer`: Clears a longword at an offset of 32 in a4, sets d0 to 255, and writes it to an offset of 48 in a4; it then decrements a word at an offset of 110 in a2, pushes a4 and a3 onto the stack, calls another routine, and loops if the word at the offset in a2 is not zero, otherwise it clears a word at offset 64 in a2 and branches to another routine.
- `update_clipboard_buffer_state_2`: Checks a status field and a pointer in a structure to determine if a buffer segment should be processed, updates a length counter by eight if a specific condition is met, and clears a status byte and a 16-bit value in the structure before returning.
- `modify_clipboard_buffer`: This routine copies a long value from an offset in a4 to an offset in a2, pushes a3 and a4 to the stack, calls another routine, and then branches to an earlier instruction.
- `check_clipboard_length`: Library code that compares the word at offset 28 of a structure in a4 to the value four in d0, and if they are not equal, branches to another location; otherwise it writes four to offset 64 of a structure in a2 and branches to another location.
- `clear_clipboard_word`: Clears word d3 and branches to an earlier instruction.
- `manage_clipboard_buffer_state`: Checks a status field in a structure to determine if a buffer segment should be processed, increments a length counter by eight if a specific condition is met, clears a 16-bit value at offset 110, and clears a bit in a status byte at offset 30 of another structure.
- `clear_buffer_pointer`: Clears a longword at offset 90 of a structure in a2 and then branches to another location.
- `update_clipboard_buffer_state`: Iterates through a structure to manage buffer lengths and status flags, performing bitwise operations on bytes at specific offsets and calling other routines to handle string length measurements and buffer clearing.
- `traverse_linked_list`: Traverses a linked list by following pointers until a null pointer is reached, then calls another routine with a pointer to the last valid node.
- `clear_and_branch_4`: Sets register d1 to zero and branches to another location.
- `call_library_function_2`: Loads a library base from a global variable and calls a library function at a specific offset.
- `set_byte_value`: Writes a constant byte value to an offset within a structure.
- `set_byte_constant_offset`: Writes a constant byte value to a specific offset within a structure and then calls another routine.
- `call_set_byte_value`: Pushes a value from a data structure onto the stack and calls set_byte_value.
- `call_set_byte_value_2`: Pushes a pointer from the stack onto the stack and calls set_byte_value, then returns the value retrieved from the stack.
- `call_set_byte_value_3`: Pushes a value from a data structure onto the stack, calls set_byte_value, and then restores the value from the stack before returning.
- `execute_sequential_pointer_updates`: Loads a value from a pointer, then calls three separate routines in sequence, each using that same value as an argument.
- `update_structure_from_pointer`: If a 32-bit value at offset 48 of a structure in a2 is zero, it copies a 32-bit value from offset 56 of a structure pointed to by the value at offset 24 of a2 into offset 48 of a2 and sets bit 4 of a byte at offset 30 of a2.
- `call_subroutine_with_offset`: Calls another routine using a value from a register plus an offset of 96 and then calls a second routine.
- `increment_structure_value`: If a 32-bit value at offset 48 of a structure in a2 is zero, it increments a 32-bit value at offset 56 of a structure pointed to by the value at offset 24 of a2 and stores the result in offset 48 of a2.
- `call_subroutine_with_different_offset`: Calls another routine using a value from a register plus an offset of 112 and then calls a second routine.
- `copy_structure_field`: Copies a longword from an offset relative to a pointer to another offset in a structure.
- `copy_structure_field_offset`: Copies a longword from a different offset relative to a pointer to a field in a structure.
- `search_and_allocate_clipboard_node`: Traverses a linked list to find a node matching a specific value; if no match is found, it allocates a new node, initializes its members, and performs memory management and fragmentation checks via a loop.
- `compare_and_zero_registers`: Compares a longword at offset 14 of a structure in a3 with d3, and if they are not equal, sets d4 and d2 to zero before branching to another location.
- `manage_clipboard_buffer_allocation`: Increments a counter in a global variable and updates a pointer in a structure, then enters a conditional branch to either recursively process data or allocate a memory block, initialize its fields, and perform a loop to transfer data chunks between buffers using a series of function calls.
- `process_clipboard_buffer_data`: Initializes a structure with a global variable and several flags, then enters a complex sequence of calls to manage data chunks.
- `execute_library_function`: Pushes two values onto the stack, calls a library function, and if the result in d0 is not 256, clears a word at offset 94 of a structure in a3 and subtracts 256 from a value at offset -4 of a structure in a6.
- `check_and_clear_field`: Tests a word at offset 94 of a structure in a3 and a longword at offset -4 of a structure in a6, then calls another routine and potentially clears the word at offset 94 of a3 in a loop.
- `synchronize_and_iterate_structure`: Compares a value in a structure against a constant, clears specific fields if they do not match, and enters a loop that calls another routine repeatedly using a pointer and a bit-shifted value derived from a byte at offset 9 of the returned pointer.
- `process_complex_structure`: Increments values in a structure, performs conditional library calls based on a flag, and returns a value from a structure offset.
- `decrement_and_cleanup_clipboard_structure`: Decrements a 16-bit counter at offset 18 of a structure and, if it reaches zero, performs a recursive cleanup of a linked list of data blocks.
- `compare_library_result_2`: Pushes a value onto the stack, calls a library function with a pointer at offset 18 of a4, and if the result in d0 does not match d4, clears a word at offset 94 of a structure in a3, sets d4 to zero, and updates a4 with a value from memory.
- `clear_status_word`: Clears a word at offset 94 of a structure in a3 and branches to another location.
- `cleanup_and_call_library`: Calls several routines, clears 32-bit values at offsets 20 and 24 of a structure in a2, decrements a global byte, and conditionally calls a library function if the global byte is zero and a bit in a global variable is set.
- `cleanup_structure_data`: Resets several structure fields to zero, decrements a global counter, and conditionally calls another routine if the counter reaches zero.
- `execute_chained_calls`: Tests a word at offset 252 of a global variable, and if it is non-zero, pushes a pointer from a global variable to the stack, calls a library function, pushes a pointer from offset 220 of a global variable, calls another routine, and then branches to another location.
- `set_global_bit`: Sets bit 3 of a global longword and returns zero.
- `update_structure_and_call`: Writes a constant byte to an offset in a structure, performs a bitwise AND on another offset, and calls another routine if a comparison with a constant succeeds.
- `invoke_table_entry_3`: The routine invokes a function pointer retrieved from an offset within a global variable using an index.
- `check_bits_and_set_byte`: Checks if bit 7 and bit 6 of a byte at offset 30 of a structure in a2 are both clear; if so, it writes the constant byte 254 to offset 31 of the structure, then calls another routine if bit 5 is set, and finally calls another routine regardless.
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
- `invoke_table_entry_6`: Loads a library base from a global variable and calls the function located at an offset of minus 522 from that base.
- `store_byte_at_pointer_2`: Stores a byte from register d0 at the memory location pointed to by a3 and increments a3.

### printer.device

- `execute_printer_initialization_2`: Branches to a new location.
- `execute_short_handler`: Loads the constant negative two into d0 and calls another routine before returning.
- `manage_linked_list`: Sets a bit in a byte, performs a linked list insertion if a flag is set, and calls functions through pointers retrieved from a table.
- `execute_printer_initialization`: Calls a routine through a pointer at offset -84 of a structure in a6, and if the result is non-zero, stores that result in a global variable, sets a value at offset 34 of the result structure, and calls another routine through a pointer at offset -432 of a6.
- `execute_library_chain`: Calls a library function at an offset of minus 228 from a base in a6 three times, and if the results are non-zero, performs further library calls at offsets minus 188, minus 198, and minus 218, or calls a function at offset minus 514 if the results are zero.
- `call_library_functions_repeatedly`: Calls a function at an offset of minus 414 from a global variable three times, and then calls a function at an offset of minus 210 from the same global variable.
- `execute_status_updates`: Calls three different routines in sequence and then returns.
- `execute_short_handler_alt`: Loads the constant negative three into d0 and calls another routine before returning.
- `set_status_bit_2`: Sets bit 0 of a byte at offset 61 of a structure in a6.
- `clear_status_bit`: Clears bit 0 of a byte at offset 61 of a structure in a6.
- `check_pointer_and_branch`: Loads a pointer from an offset of 72 in a6, checks if it is null, and if not, calls another routine before branching back to itself.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_previous`: Branches to a previous location in the code.
- `process_event_data`: Calls several library functions to retrieve values, updates various offsets in a structure in a6, and enters a loop that checks bits in a returned value to modify bytes at specific offsets in a6 or call library functions based on a linked list.
- `compare_and_clear_bit`: Loads an address from an offset of 94 in a6 into a1, compares d0 to a1, and if they are equal, clears bit 1 of a byte at offset 2486 in a6.
- `process_event_loop`: Enters a loop that checks bits in a value returned by a library function to decide whether to iterate through a linked list of structures, setting bit 5 of a byte at offset 30 in each structure and calling a function through a table, or to call another library function and clear bit 0 or bit 1 of a byte at offset 2486 in a6 based on the returned value.
- `call_handler_with_args`: Loads two values from the stack into a0 and d0, then calls another routine.
- `update_status_and_data`: Calls another routine, and if it returns zero, sets bit 0 of a byte at offset 1502 of a structure in a global variable, then updates a structure at an offset determined by the value of that byte with two 32-bit values and a 16-bit value before calling another routine twice.
- `adjust_stack_and_branch`: Adds 8 to the stack pointer and branches to a previous location.
- `set_status_bit_4`: Sets bit 1 of a byte at offset 2486 in a6, loads an address from offset 190 in a6 into a1, and branches to a previous location.
- `check_status_and_return`: Loads a pointer from a global variable into a0, reads a byte from offset 31 of that pointer into d0, and if it is not zero, checks if bit 0 or bit 1 of the bitwise inverse of that byte is set to determine whether to return immediately or return after clearing d0.
- `process_printer_status_loop`: Enters a loop that repeatedly calls three different routines, checks a value at offset 332 of a structure in a6, and conditionally clears bit 0 or bit 1 of a byte at offset 9b6 of a6 based on a return value, otherwise it returns a value from a byte at offset 1f of a structure in a6.
- `set_bit_and_branch_2`: Calculates an address based on a6, compares d0 to it, and if not equal, clears bit 1 of a byte at offset 2498 of a6 before calling another routine.
- `compare_and_call_routine`: Compares a 32-bit value at offset 272 of a structure in a6 with a value at offset 272 of another location, and if they are not equal, calls another routine; otherwise, it returns.
- `call_status_and_branch`: Calls another routine and then branches to a previous location.
- `modify_structure_fields`: Loads an address from a structure offset into a1, writes a constant word to offset 28 of a1, copies a longword from a pointer in a0 to offset 32 of a1, and clears a longword at offset 36 of a1.
- `clear_status_byte`: Sets a byte at a specific offset to zero and calls a routine through a pointer found in a structure.
- `check_status_byte`: Loads an address from a structure in a6, compares a byte at offset 8 of that address to 7, and returns if they are not equal.
- `execute_callback_sequence`: Calls a routine through a pointer in a structure and then calls another routine using a value from a global base.
- `load_and_test_byte`: Pushes a6 to the stack, loads a pointer from a table, tests a byte at offset 31 of that pointer, and if non-zero, restores a6 from the stack and returns; otherwise it clears a6 and tests a byte at offset 2498 of a6.
- `process_structure_fields`: Loads an address from a structure offset into a1, writes a constant word to offset 28 of a1, copies a longword from a pointer in a0 to offset 28 of a1, calls a routine to check a status byte, and calls another routine to test a byte from a table.
- `compare_and_clear_bit_2`: Compares a longword at offset 46 of a6 with d0, and if they are equal, clears bit 1 of a byte at offset 2470 of a6 before calling another routine and branching to a third location.
- `compare_and_branch_status`: Compares a longword at an offset of 272 from a6 with the value in d0, then branches to another location if they are not equal, otherwise it calls another routine, tests d0, and branches based on whether d0 is zero.
- `branch_and_call_status`: Calls a routine and then branches to another location.
- `compare_status_and_branch`: Saves registers to the stack, compares a byte at offset 2489 in a6 to 1, and if not equal, branches to another location; otherwise, it pushes the program counter and branches to another location.
- `update_interrupt_and_status`: Modifies a byte at offset 294 of a global variable, writes a value to a memory location, calls a library function, and if the result is not zero, clears bit 0 or bit 1 of a byte at offset 2486 in a6 and calls another routine if the corresponding bit was set.
- `clear_status_bits_2`: Calls a routine through a pointer at an offset from a global base; if the result is 0, it clears bit 1 of a byte at offset 9b6 of a structure in a6 and calls another routine if bit 0 was set, then clears bit 2 of that same byte and calls another routine if bit 1 was set.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `update_list_and_copy_string`: Pushes the contents of a1 onto the stack twice, calls another routine, restores a1 from the stack, calls another routine, and returns.
- `reset_structure_field`: Sets a 32-bit field in a structure at offset 20 of a1 to zero, conditionally updates it from the field at offset 24 of a1 based on a routine return value, and calls two other routines.
- `update_list_and_copy_string_2`: Pushes two values from a1 onto the stack, calls a routine through a pointer at offset 288 of a1, restores a1 from the stack, and calls another routine before returning.
- `update_list_and_copy_string_3`: Performs a linked list insertion and sets a status bit, then calls another routine via a pointer retrieved from a table.
- `dispatch_pending_entry_8`: Checks a condition via a routine call at an offset from a global base; if the result is 0, it populates a structure at a global base with two parameters and the value 9 at offset 300, then executes several routines through a global base using a pointer from the structure.
- `call_handler_with_parameters`: Saves registers to the stack, loads a pointer from offset 8 of a structure into a2, sets a2 to 284 and a4 to 266, calls a routine at a fixed address, and then branches to another location based on whether the result in d0 is zero.
- `reorder_buffer_elements_2`: Calculates the length of a null-terminated string from a pointer in a structure, then iterates through the buffer to reorder bytes into a table based on their values.
- `update_printer_pointers`: Moves a long value from a pointer in a3 to a global variable, clears a 16-bit value in a global variable, and then branches to another routine.
- `process_ansi_escape_sequences`: Iterates through a character stream to identify and process ANSI-style escape sequences, handling specific control characters such as line feed.
- `clear_registers_and_branch`: Clears registers d4, d6, and d2 to zero and branches to another routine.
- `reorder_buffer_elements`: Iterates through a buffer to check for specific byte values and calls other routines to manage data consistency.
- `shuffle_and_update_values`: Increments two counters and copies a byte from a global variable to a location indexed by the first counter in a loop, then writes a 32-bit value to offset 32 of a structure in a2 based on whether a register is zero.
- `clear_memory_offset`: Stores a zero at a specific offset within a memory location pointed to by a register.
- `transform_character_data`: Iterates through a loop to compare a byte against a table, applies conditional transformations to that byte based on its value and a counter, and calls another routine.
- `clear_memory_and_branch`: Clears a longword at a negative offset from a6 and branches to another location.
- `transform_and_shift_character_table`: Iterates through a loop to compare a byte from a buffer against a table, then writes a transformed byte to a destination pointer based on specific index and value conditions.
- `set_byte_on_condition`: Compares a longword from a global variable to 2 and compares d5 to 12, then writes a byte of all ones to the address in a2 if both conditions are met, otherwise it branches to another location.
- `compare_and_set_byte_2`: Compares a long word from a global variable to 2, and if equal, compares the value in d5 to 12; if both conditions are met, it writes the byte 75 to the address in a2 and branches to another routine, otherwise it branches to another routine.
- `update_status_byte_3`: Compares a long word in d5 to nine, and if it is not equal, checks if the byte at the pointer in a4 is non-zero; if the byte is zero, it stores the value fifty-nine at the pointer in a2 and branches to another routine.
- `copy_byte_from_table_2`: Compares a long word in d5 to six, and if it is less than six, copies a byte from a table at offset 362 using d5 as an index into the memory location pointed to by a2.
- `compare_and_set_byte`: Tests a long word in d5 and branches if non-zero, otherwise compares a byte at a pointer in a4 to 29 and branches if less than or equal, finally writing the byte 12 to a location in a2 and branching to another routine.
- `transform_character_via_lookup_table`: Iterates through a table to match a byte from a source pointer, then writes a transformed byte to a destination pointer.
- `parse_ansi_escape_sequences_2`: Iterates through a character stream to identify and process ANSI-style escape sequences, handling specific control characters such as line feeds.
- `process_escape_sequences`: Iterates through a character stream to handle ANSI-style escape sequences, including line feed and carriage return conversions, character mapping from tables, and character expansion.
- `parse_escape_sequence_and_format_output`: Parses a stream of characters, handling escape sequences and special control codes such as line feeds and carriage returns.
- `check_escape_character`: Compares the byte at the address in a3 to 91 and branches to another routine if they are not equal, otherwise it performs a long branch.
- `parse_escape_sequences_2`: Parses a stream of characters, handling escape sequences and control codes such as line feeds and carriage returns.
- `handle_control_character`: Compares the byte at the address in a3 to 10 and branches if not equal, otherwise it checks a byte at a global address and writes 3 to the address in a4 if that byte is non-zero.
- `parse_printer_escape_sequences`: Iterates through a buffer to interpret escape characters and control codes, performing specific actions such as inserting line feeds and carriage returns, or looking up character mappings in a table.
- `write_multiple_bytes`: Increments a pointer in a2, writes the byte 10 to the address at the original pointer plus an offset in d2, increments the pointer in a2 again, writes the byte 13 to the address at the second original pointer plus the offset in d2, and branches to another routine.
- `write_single_byte`: Increments a pointer in a2, writes the byte 10 to the address at the original pointer plus an offset in d2, and branches to another routine.
- `handle_backslash_character`: Compares the byte at the address in a3 to 155, and if it is not equal, branches to another location; otherwise it pushes d2 and d3 onto the stack, calls another routine, cleans up the stack, and branches to another location.
- `parse_file_path_string`: Parses a string from a table, handling backslash and drive letter escape sequences to build a character buffer.
- `parse_escape_sequences`: Parses a string for escape characters, converting numeric sequences following a backslash into single characters and handling specific control codes by calling another routine.
- `update_pointer_and_branch`: Increments a pointer in a1 and a global variable, tests a byte at the address in a1, and if it is non-zero, performs several arithmetic operations on d0 and updates a global variable before branching to another location.
- `calculate_pointer_address`: Subtracts 160 from a byte loaded from a3 into d1, shifts the result left by 2, adds the value in d4 to it, and moves the resulting address into a0 and then into a1 from the address in a0 before branching to another routine.
- `skip_invalid_characters`: Checks if a byte at a pointer in a1 is 92, and if not, increments the pointer and checks if the new byte is between 48 and 57 inclusive, branching back if the conditions are not met.
- `parse_printer_command_string`: Parses a command string by converting numeric sequences into integers, handling escape sequences, and processing special control characters such as escape, bracket, and newline.
- `copy_byte_to_destination`: Loads a pointer from a memory location into d1, increments that pointer, and then moves a byte from the pointer in a1 to a destination calculated by adding d1 and d2, then branches to another routine if the byte at a1 is zero, otherwise it branches to a third routine.
- `parse_ansi_escape_sequences`: Iterates through a character stream to identify and process ANSI-style escape sequences, handling specific control characters such as line feeds, carriage returns, and various escape codes by calling other routines to update state or format output.
- `update_global_values`: Subtracts the difference between two global variables from a third global variable and clears a fourth global variable.
- `process_buffer_overflow`: Resets a global byte to zero and calls another routine; if the return value is positive, it either copies a sequence of bytes from the stack to a destination at an offset of a global variable or updates a global byte and calls another routine or modifies a global variable depending on a parameter.
- `update_counter_and_call`: Adds a value from d3 to a value from a global variable, and if the result is greater than 256, moves d3 into a global variable, checks a bit in a global variable, and either calls another routine or branches to another routine.
- `decrement_and_store_pointer`: Decrements a 32-bit value from a global variable by one, stores the result in another global variable, and branches to another routine.
- `clear_register_and_branch`: Sets register d4 to zero and branches to another routine.
- `copy_memory_block_2`: Copies a sequence of bytes from a source address to a destination address, incrementing a global pointer for each byte.
- `initialize_byte_and_branch`: Saves registers to the stack, loads three values from the stack into d3, d2, and a2, clears a byte at the address in a2, and branches to another routine.
- `clear_byte_at_offset_2`: Compares a byte in d1 to fifty-nine, and if it is equal, increments a pointer in a3, adds the value in a2 to it, and clears the byte at the resulting address before branching to another routine.
- `store_constant_at_pointer`: Compares a byte in d1 to 32, and if they are equal, loads a pointer from offset 32 of the stack into a0 and stores the value one into the location pointed to by a0.
- `compare_byte_and_load_pointer`: Compares a byte in d1 to thirty-four, and if they are equal, loads a pointer from offset 32 of the stack into a0 and sets d0 to two before branching to another routine.
- `parse_character_sequence`: Enters a loop that fetches a byte from another routine and checks if it is a digit, a space, a semicolon, or a double quote; it updates a global variable with the value in a3, clears a byte at an offset in a structure if a semicolon is encountered, and returns the fetched byte.
- `increment_and_fetch_table_value`: Increments a value at a pointer, fetches a byte from a table using the original value as an index, and calls another routine if a condition involving a second parameter and a global variable is met.
- `return_register_value`: Returns the value currently held in a data register.
- `update_global_counter`: Loads a 32-bit value from a global variable, decrements it by one, and stores the result in a global variable at offset 448.
- `decrement_and_check_table`: Decrements a value pointed to by a register and continues looping as long as the value is positive and a specific bit pattern is found in a table.
- `reset_global_state`: Clears three global variables, sets a byte at offset 270 of a structure to zero, and sets another global variable to 3.
- `copy_data_to_globals`: Copies six 16-bit values from a structure at a global base into six global variables.
- `compare_global_data`: Compares six 16-bit values from a structure at a global base with six global variables and returns 0 if they are all equal, otherwise it returns -1.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_conditional_calls`: Extracts four bytes from a structure at offset 32, calls another routine with a short value from offset 32 and a pointer to the first extracted byte, then performs a series of conditional calls to other routines based on the return value and the value at offset 32.
- `dispatch_conditional_routines`: Calls various versions of another routine with different parameters based on the state of registers and return values.
- `copy_formatted_string`: Calls a routine through a pointer, then either updates a global variable or enters a loop to copy bytes from a table to a destination at offset 1 of a1, handling specific byte values and potentially appending a byte from a pointer at param_3.
- `store_conditional_word`: Compares a word at offset 22 of the stack to 54 and 53, and if it matches 53, performs bitwise operations and stores a byte at offset 262 of a global variable.
- `copy_processed_string`: Enters a loop that copies bytes from a source address to a destination address, replacing the value negative two with zero and continuing until a null terminator or a negative one is encountered.
- `compare_and_branch_d1`: Compares the byte in d1 with 255 and branches to another routine if they are not equal; otherwise, it sets d1 to 255 and branches to another routine.
- `copy_string_data`: Enters a loop that copies bytes from a source address in d4 to a destination address in d2, updating an index in d3, and returns a value based on the termination condition or specific byte values.
- `execute_jump_table_2`: Saves d2 to the stack, loads a value from the stack into d2, moves a pointer from the stack into a0, and then performs a jump to an address calculated from d2 and a table.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `validate_printer_timing_parameters`: Copies several fields from a structure into global variables, calls calculate_printer_timing_parameters, and performs bounds checking on the resulting timing values against the input parameters.
- `calculate_printer_timing_parameters`: Calculates printer timing values by comparing data from a structure against local variables and updates a buffer.
- `update_printer_timing_parameters_3`: Calculates the minimum timing values by comparing values from a structure with local variables and updates a buffer.
- `recalculate_printer_timing_parameters`: Calculates minimum timing values by comparing values from a structure with local variables and updates a buffer.
- `update_printer_timing_parameters_2`: Compares values from a structure with local variables to determine minimum timing values, updates a buffer, and performs bitwise manipulations on a global variable to adjust timing and buffer parameters.
- `compare_and_branch_5`: Compares the word in register d0 to 2048; if they are not equal, it branches to another routine, otherwise it branches to another routine.
- `load_constant_and_branch_4`: Loads the value 12 into register d1 and branches to another routine.
- `load_constant_and_branch_5`: Loads the value 15 into register d1 and branches to another routine.
- `update_printer_timing_parameters`: Updates timing and buffer values by comparing elements from a structure with local variables and performing bitwise manipulations on a global status byte.
- `update_printer_status_and_buffer`: Compares a value from a structure with a local variable, updates a global status byte, and performs complex bitwise manipulations and conditional updates on two integer values stored in a structure via a6.
- `modify_stack_offset`: Loads small constants into registers, stores a value at an offset of negative 4 from a pointer in a6, tests two different global bytes to determine branching, and performs arithmetic on the value stored at the negative 4 offset from a6 before branching to another routine.
- `check_status_flags`: Clears a 32-bit value at a global address, tests a byte at another global address and branches if zero, then tests a byte at a third global address and branches if zero before pushing a stack address and branching to another routine.
- `calculate_color_bounds_and_clear_table`: Calculates color bounds by comparing two values against a range and stores the minimum and maximum results in global variables.
- `generate_color_lookup_table_2_2`: Calculates color bounds and populates a 64-entry table by iterating through a range of values, decomposing each into nibbles and storing them in a structure.
- `generate_color_lookup_table_2`: Calculates a range of color values based on bounds stored in a structure, populates a 64-entry table by decomposing color values into nibbles, and performs bitwise transformations on the table entries based on specific flags.
- `generate_color_lookup_table`: Calculates a range of color values based on bounds stored in a structure, populates a 64-entry table by decomposing color values into nibbles, and optionally applies bitwise inversion and bit-shifting to the table entries.
- `generate_inverted_color_palette`: Calculates a range of color values, populates a 64-entry table by decomposing color values into nibbles, and optionally applies bitwise inversion and bit-shifting to the table entries.
- `expand_and_invert_color_palette_2`: Clears a table of 64 entries and populates it by calling expand_and_invert_color_palette to decompose color values into nibbles.
- `generate_expanded_palette`: Clears a table of 64 entries and populates it by calling expand_and_invert_color_palette to decompose color values into nibbles.
- `expand_and_invert_color_palette`: Decomposes color values into nibbles stored in a table, optionally shifts bits to create a secondary palette, and performs bitwise inversion on the table entries before calling convert_and_process_color_data.
- `convert_and_process_color_data`: Decomposes color values into nibbles and stores them in a table, then performs bitwise operations on a series of 32-bit integers in a loop.
- `execute_conditional_functions`: Sets three global variables, calls a function via a pointer in a global variable, and if the result is zero, calls that same function again with different arguments.
- `execute_function_and_store_result`: Pushes three empty stack frames, calls a function via a pointer in a global variable, stores the returned value in another global variable, and then branches to either another routine or a location on the stack.
- `check_multiple_printer_flags`: Tests two bytes at global addresses and, if both are non-zero, calls another routine before branching to another routine; otherwise it branches to another routine.
- `test_and_call_printer`: Tests a byte at a global address and, if non-zero, calls another routine before branching to another routine; otherwise it branches to another routine.
- `check_printer_status_and_call_handler`: Tests a byte in a global variable and, if non-zero, calls another routine and then branches to another routine; otherwise, it branches to another routine.
- `call_and_store_result_4`: Calls another routine, stores the returned value from d0 into a global variable, and branches to another routine if the value is not zero.
- `call_empty_routine_conditionally`: Checks if a value in a register is divisible by a 16-bit value at an offset from a pointer in d5, or if a routine call returns 0, then calls another routine with no arguments.
- `call_empty_routine`: Calls another routine with two zero arguments and returns.
- `calculate_weighted_value`: If a global variable is zero, it returns the input parameter unless bit 2 of a byte at offset 21 of a structure is set, in which case it finds the minimum of three values using another routine.
- `find_minimum_byte`: Checks bit 2 of a byte at offset 15 of a global variable and, if set, finds the smallest byte value among three provided bytes and returns it, otherwise it returns the original second parameter.
- `calculate_printer_offset`: This routine performs several arithmetic operations involving global variables and constants, calls another routine, and then updates several global variables based on the result.
- `update_global_pointers`: Calls a routine to print a formatted message, updates four global pointers by adding a global variable to each, and calls another routine to update a pointer table.
- `update_pointer_table_2`: Updates four global pointers by adding a global variable to each, then calls another routine with a value from a memory location and returns.
- `store_and_branch_data`: Stores a longword from a memory location into the address in a2, loads a pointer from a memory location into a0, tests bit 3 of the byte at offset 21 of a0, and if it is set, branches to another location; otherwise it subtracts d2 from 15 and branches to another location.
- `manipulate_and_branch_data`: Moves a word from d2 to d1, moves a word from d3 to d0, shifts d0 right by 4 bits, and branches based on the result of a comparison with 1.
- `compare_and_branch_short`: Compares a 16-bit value in d0 to 2, and if it is not equal, branches to a different location; otherwise it branches forward.
- `decrement_and_loop`: Decrements a 16-bit value in d0 and loops back to an earlier instruction if the result is non-zero, otherwise it branches forward.
- `store_byte_and_branch`: Stores a byte from d1 into a memory location at offset 272 and branches forward.
- `store_byte_and_branch_offset`: Stores a byte from d1 into a memory location at offset 274 and branches forward.
- `transfer_pointer_and_call`: Moves a byte from register d1 to a specific memory location, pushes a pointer from a2 onto the stack, calls another routine, moves the result from d0 into the memory location pointed to by a2, increments the stack pointer by four, moves the value from a2 into another memory location, and branches to another routine.
- `update_pointer_table`: Loads a 32-bit value from a table using an index and updates four global pointers by adding a global base address to them.
- `calculate_and_update_pointers`: Calculates a value based on the product or sum of two globals and updates four global variables by calling another routine multiple times.
- `evaluate_conditional_results_3`: Sets a global variable to zero and performs a series of conditional checks involving multiple routine calls and comparisons to determine a return value.
- `evaluate_conditional_results_2`: Checks if a global variable is zero, then calls three different routines with varying parameters, and if any of those calls return a non-zero value, stores that value in a pointer in a2 and returns it.
- `evaluate_conditional_results`: Compares an input to a table using two parameters, then calls another routine with three parameters; if that result is zero, it calls a second routine with four parameters and stores the result in a pointer, otherwise it calls a different routine with three parameters and stores that result in the pointer.
- `compare_and_store_result`: Calls another routine with three parameters, and if the result is non-zero, calls a third routine with four parameters and stores its result in a location pointed to by a2.
- `clear_and_restore_registers`: Sets d0 to 0 and restores a2, a3, and a4 from the stack before returning.
- `increment_and_check_modulus`: Increments a global variable and, if that variable modulo a value from a structure at offset 24 is not zero, calls another routine twice and stores the result in a global variable.
- `call_and_store_d0`: Pushes three values onto the stack, loads a pointer from an address into a0, calls the routine at that address, stores the result in d0 into the location pointed to by a2, adjusts the stack pointer, and branches to another routine if d0 is zero, otherwise it branches to another routine.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `load_and_branch_indices`: Saves registers a2 through a4 to the stack, loads three constant addresses into a2, a3, and a4, copies a 16-bit value from a global variable to the address in a4, and branches to another routine.
- `process_printer_loop_logic_2`: Performs a nested loop structure that calls other routines, updates several global variables, and uses a loop that increments a value by a product of a global variable and another value.
- `process_nested_loops`: Calls accumulate_scaled_values and process_printer_loop within nested loops, updates several global variables, and performs arithmetic on a global variable before returning a value through a pointer.
- `clear_and_branch_5`: Clears a 16-bit value in a global variable and branches to another routine.
- `process_printer_loop`: Performs a nested loop structure that calls several routines, updates various global variables, and performs arithmetic on global variables and pointers to determine a return value.
- `process_printer_loop_logic`: This routine performs a loop that increments a global variable, updates a global value using a multiplication and addition, and enters a nested loop structure that calls other routines based on various conditions and global variable states.
- `increment_and_check_limit`: Adds a 16-bit value from the address in a3 to the 16-bit value at the address in a4, then compares the new value at the address in a4 to a constant; if it is below the constant, it branches, otherwise it clears d0 and returns.
- `clear_and_load_index`: Saves registers a2 through a4 to the stack, loads three constant addresses into a2, a3, and a4, clears a 32-bit value in a global variable, copies a 16-bit value from a global variable to the address in a4, and branches to another routine.
- `execute_printer_logic_loop_2`: Performs a nested loop structure that calls other routines, updates several global variables, and uses a loop that increments a value by a global variable.
- `clear_global_pointers`: Clears four long values at specific global offsets using zero and then performs a short branch.
- `execute_printer_logic`: Executes a complex loop involving multiple routine calls that updates global variables and performs arithmetic on a pointer's target and other global values.
- `call_and_store_result_2`: Calls another routine, stores the returned value from d0 into the address in a2, and if d0 is not zero, loads the value from the address in a2 into d0 before branching to another location.
- `execute_printer_logic_loop`: This routine implements a loop that increments a global variable and decrements another, containing nested loops and conditional calls to other routines that update global variables and manage data based on returned values.
- `increment_and_check_printer_buffer`: Adds a 16-bit value from a register to a 16-bit value at a pointer in a4, then compares the result against a constant in a global variable.
- `initialize_printer_buffer_counters`: Sets three constant values into registers, clears a 32-bit word at a global offset, copies a 16-bit word from a global variable to a destination specified by a register, and branches to another routine.
- `call_with_stack_parameters`: Pushes two 16-bit values from a global variable onto the stack, calls another routine, stores the result in d0 into the location pointed to by a3, adjusts the stack pointer, and branches to another routine.
- `transfer_printer_data`: Clears a word at an offset of 20 in a global variable, copies a longword from an offset of 30 to an offset of 26, copies a longword from an offset of 16 to an offset of 276, copies a word from an offset of 390 to an offset of 314, and then branches.
- `update_state_with_decrement`: Calls update_printer_state and accumulate_scaled_values, performs arithmetic on a global variable, and decrements a value at a pointer's target within a loop.
- `update_printer_state`: Library code that calls another routine, increments a long value at a global variable, compares a word from that global variable to d1, and if d1 is greater, branches to another location; otherwise it calls two other routines, stores d0 in the address in a2, and if d0 is zero, branches to another location, otherwise it moves the value from a2 to d0 and branches to another location.
- `update_printer_state_2`: Increments a long value in a global variable, compares it to a word in d1, and returns.
- `decrement_and_check_limit`: Decrements a 16-bit value at the address in a3, and if the result is above zero, it increments a 16-bit value at the address in a4, compares that value to a constant, and if it is below the constant, it branches; otherwise it clears d0 and returns.
- `reset_printer_buffer_indices`: Clears two 16-bit values in a global structure and updates a pointer in a table using a value from another memory location, then branches to another routine.
- `push_parameters_and_call_accumulate`: Pushes two 16-bit values from global variables onto the stack and calls accumulate_scaled_values, then stores the result in a memory location pointed to by a register.
- `transfer_printer_pointers`: Clears a word at offset 20 of a structure, copies a long from offset 238 to offset 234, moves a long from offset 16 to offset 276, moves a word from offset 378 to offset 314, and branches to another routine.
- `manage_printer_buffer`: Executes a loop that clears several global variables, calls another routine, and uses a for loop and while loop to update global variables and pointers through multiple routine calls.
- `store_result_and_branch`: Calls another routine, stores the returned value into a memory location pointed to by a register, and branches to another routine if the result is zero; otherwise, it loads the stored value back into a register and branches to a different location.
- `manage_buffer_loop`: Executes a loop that increments a global variable and decrements another, calls increment_and_check_modulus and manage_printer_buffer, and updates a pointer's target with a returned value.
- `decrement_and_validate_buffer_index`: Decrements a 16-bit value at the address in a3 and checks if the result is unsigned greater than zero; if so, it increments a 16-bit value at the address in a4, compares the new value against a constant, and if it exceeds that constant, clears d0 and restores registers from the stack before returning, otherwise it returns.
- `accumulate_scaled_values`: Iteratively adds a doubled value to a pointer's target until the target is no longer less than one, then subtracts a doubled second parameter from that target.
- `compare_input_to_table`: If a global variable is zero, it compares the input parameter to a byte retrieved from a table using indices derived from two other parameters; otherwise, it compares the input parameter to a value derived by subtracting a global variable from 15.
- `compare_table_value`: Performs a comparison between a byte retrieved from a table using bitwise-derived indices and a provided byte.
- `negate_boolean_results`: Calls two different routines and returns the negation of the first return value based on the status of a second return value.
- `divide_32_bit_value`: Performs a division of a 32-bit value by a divisor, using a loop to rotate bits if the divisor is greater than sixty-five thousand five hundred thirty-five.
- `rotate_32bit_value_2`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_8`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper_2`: Calls one helper and returns, with no other work.
- `call_helper_and_return_d1`: Moves the value from register d0 into d1, calls a helper routine, and returns the resulting value from d1 in register d0.
- `negate_result_on_flag_4`: Calls a routine to calculate a value and negates that value if the resulting value in an address register is nonzero, then returns the result.
- `call_and_negate_on_flag_3`: Calls a routine and returns its first result, or the negation of that result if the second result returned in an address register is nonzero.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `traverse_and_copy_string_table`: Iterates through a table of pointers based on a count in a structure, calling another routine for each entry until a null terminator is reached.
- `fetch_table_address`: Shifts the value in d0 left by two bits, loads the constant 32 into a2, loads a pointer from a table using a2 and d0, and branches to another location.
- `branch_to_offset`: Performs a short branch to a location within another routine.
- `set_address_and_branch`: Loads the constant 32 into a2 and branches to another location.
- `process_string_and_disk_data`: Iterates through a string by calling another routine to compare characters, and if a null terminator is reached, it either calls another routine or checks a pointer at offset 86 of a structure to call another routine, performs several other calls, and potentially copies a string to a global variable.
- `initialize_disk_and_copy_data`: Checks a pointer at offset 56 of a structure, calls a routine through that pointer, performs a series of calls to manage memory and disk operations, and enters a loop to copy bytes from a source pointer to a destination pointer until a null byte is encountered.
- `unlock_and_get_directory`: Calls a routine to unlock a resource using a pointer from a register, then calls another routine to retrieve the current directory, and returns negative one.
- `execute_callback_and_recurse`: Checks a global value, calls a function pointer from a table, and then calls itself with a value from a pointer.
- `compare_case_insensitive_bytes`: Adjusts two input bytes by subtracting thirty-two if they fall within the range of backtick to curly brace, then returns whether they are equal.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_3`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### narrator.device

- `initialize_device_structure`: Calls update_task_status to retrieve a pointer, then populates several fields in that structure including offsets 164, 252, and 772, and subsequently calls three more routines via pointers from a global variable to populate additional fields in a second structure before returning -1.
- `test_and_negate_d0`: Tests if a long value in d0 is zero; if it is not zero, it sets d0 to negative one and branches to another routine, otherwise it branches to another routine.
- `update_narrator_device_status`: Checks a status flag in a structure; if zero, it attempts to initialize the device by calling two functions from a library, setting error codes and a global variable if they fail.
- `update_device_state_fields_2`: Stores a byte into a structure at offset 178 of a6, copies a longword from offset 244 of a6 to offset 178 of a6, and calls another routine.
- `update_device_state_fields`: The routine stores an input value into a structure, increments counters in the structure, copies a counter to another structure, writes several constant words and bytes to the second structure, and clears a flag bit in the first structure.
- `update_device_state`: Stores a byte from d0 into a structure at offset 31 of a2 and sets a long value at offset 20 of a2 to negative one.
- `clear_interrupt_flag`: Clears bit 0 of a byte at offset 30 of a structure in a1, loads a pointer from offset 20 of a structure in a1 into a3, and clears bit 6 of a byte at offset 34 of the structure in a3.
- `manage_interrupt_queue_2`: If a longword at offset 24 of a1 matches a value in d0, it checks a bit in a structure and compares a byte in a1 with a byte from another structure; if they differ, it copies a byte, splits it into nibbles, and calls another routine.
- `manage_interrupt_queue`: If certain conditions are met, it copies a byte from one structure to another and splits it into nibbles; otherwise, it updates a linked list and modifies the INTENA register.
- `copy_byte_and_split_nibbles`: The routine copies a byte from one structure to another, splits it into low and high nibbles into two bytes, clears a byte in the destination structure, and calls another routine.
- `manage_voice_reference_counts`: Decrements a reference count in a global structure and, if the count reaches zero, disables interrupts via the INTENA register and calls clear_flags_and_call_handler.
- `decrement_voice_ref_count`: Decrements a byte in a global structure and, if the value reaches zero, disables interrupts via the INTENA register and calls another routine.
- `process_pointer_list`: Traverses a linked list of pointers starting from a pointer at offset 54 of a structure in a6, calling another routine if a 32-bit value in a1 matches a value in the list.
- `decrement_counter_and_close`: Decrements a 16-bit counter at offset 32 of a6 and, if it reaches zero, calls a routine via a pointer in a global variable and then calls another routine via a pointer at an offset from that global variable.
- `dispatch_handler_by_index`: Loads a word from an offset of a1, checks if it is between zero and eight, and if so, calls a routine from a table using the scaled value as an index.
- `set_structure_flag`: Saves registers to the stack, sets a6 to a value from a constant, and if a word at an offset of a3 is non-zero, sets bit 5 of a byte at an offset of a0, which is derived from a3.
- `swap_pointers_and_return_value`: The routine calls library functions via a base pointer, swaps values between a pointer table and a data word, updates a pointer field, and returns a value from a table entry.
- `process_event_status_and_queue_data`: Checks a status field in a structure to determine an action, sets or clears specific bits in a control byte, and either calls a handler via a pointer or inserts the structure into a linked list while updating a global counter.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `update_status_and_invoke_handler`: Clears bit 5 of a byte at offset 34 of a structure in a0, writes 7 to offset 196 of a structure in a6, sets bit 0 of a byte at offset 198 of that same structure, calls a handler via a pointer at offset 164 of a structure in a6, calls another routine via a pointer at offset 132 of that structure, sets a byte at offset 31 of a structure in a1 to 0, and then calls itself recursively.
- `set_flags_and_invoke_handler`: The routine sets a flag bit in a structure, writes a word and sets a bit in a global structure, calls a handler via a base pointer, clears a byte in another structure, and calls another routine.
- `set_flag_then_clear`: The routine sets bit 7 in a byte at offset 34 of a structure, calls two other routines, then clears bit 7 in the same byte.
- `set_flag_clear_byte_then_call`: Sets bit 7 in a byte at offset 34 of a structure in a0, calls another routine, clears bit 7 in that same byte, clears a byte at offset 31 of a structure in a1, and calls another routine.
- `modify_global_counter`: Sets a global variable to 4096, increments a byte at offset 294 of that global variable, and traverses a linked list of pointers starting from a pointer at offset 20 of a structure in a3, calling another routine for each node.
- `decrement_counter_and_set_global`: The routine decrements a byte in a global variable area and, if the previous value was less than one, writes a constant to a global variable.
- `compare_and_update_pointers_2`: Saves registers to the stack, updates a pointer in a6, enables interrupts by writing 16384 to the INTENA register, increments a byte at offset 182 of a6, and compares a longword at offset 772 of a3 with a1.
- `compare_and_update_pointers`: Loads a longword from a structure in a0 into d0, and if d0 is not zero, compares a0 with a1 and branches to another routine if they are equal; otherwise, it sets a0 to the value in d0 and loops back to the start of this routine.
- `update_pointers_and_decrement_counter`: The routine updates two pointers using values from a structure, writes a byte to a destination structure, decrements a byte in a global variable, and if the value is less than one, writes a constant to a hardware register and calls another routine.
- `decrement_counter_and_set_global_2`: The routine decrements a byte in a structure pointed to by a register and, if the previous value was less than one, writes a constant to a global variable.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `write_byte_and_call_handler`: The routine writes a byte to a structure at offset 31 and calls another routine.
- `clear_byte_and_call_handler`: The routine clears a byte in a structure at offset 31 and calls another routine.
- `clear_flags_and_call_handler`: The routine clears a flag bit in a structure, clears a byte in another structure, and calls a handler via a base pointer.
- `populate_voice_parameter_structure`: Validates a series of numeric values from an input structure and stores them into a target structure at specific offsets if they fall within defined ranges.
- `validate_and_store_numeric_value`: Compares a 16-bit value in d1 against 384; if it is not greater, it stores d1 into a 16-bit offset 28 of a structure in a5.
- `store_conditional_word_2`: Compares a 16-bit value in d1 against 384; if it is greater, it branches to another routine, otherwise it stores d1 into a 16-bit offset 28 of a structure in a5, loads a 16-bit word from an address in a2 into d1, and if d1 is 0 or 1, branches to another routine, otherwise it loads the constant 232 into d3 and branches to another routine.
- `store_word_and_branch`: Stores a 16-bit word from register d1 into a structure at offset 48 of a5, then compares a 16-bit word at the address in a2 against 0 or 1; if neither matches, it loads 233 into d3 and branches to another routine.
- `update_threshold_and_branch`: Stores a 16-bit word from register d1 into a structure at offset 38 of a5, then compares a 16-bit word from offset 64 of a2 against 5000; it branches to another routine if the value is greater than or equal to 5000, otherwise it loads a constant into d3 and branches to another routine.
- `compare_and_store_word`: Compares a 16-bit value in d1 to 26464 and branches if it is greater; otherwise, it stores d1 into a 16-bit word at offset 22488 of a structure in a5, loads a 16-bit word from offset 62 of a structure in a2 into d1, and if d1 is not less than or equal to 0, it sets d3 to 230 and branches to another routine.
- `scan_numeric_string_for_delimiters`: Scans a string backwards from a specified offset to find a comma, a minus sign, or a space.
- `parse_numeric_string_3`: Scans a string backwards from a specified offset to find a delimiter such as a comma, a minus sign, or a space.
- `move_and_branch_2`: Copies a longword from offset 36 of a structure in a1 to offset 32 of the same structure and branches to another routine.
- `clear_word_and_write_byte`: The routine clears a longword at offset 32 of a structure, writes a byte to offset 31 of that structure, and calls another routine.
- `parse_numeric_string`: Scans a string backwards from a specified offset to find a delimiter such as a comma, a minus sign, or a space.
- `shift_and_clear_value_2`: Loads a longword from offset 150 of a5, performs arithmetic to calculate a value, and compares a byte at an offset determined by that value with 32.
- `shift_and_clear_value`: Loads a longword from offset 150 of a structure in a5 into d0, shifts it right by one bit, stores the resulting word at offset 156 of the structure in a5, clears the longword at offset 150, and branches to another routine.
- `initialize_voice_buffer_and_copy_table`: Clears several fields in two structures, calls two external routines, and if a condition is met, allocates a new structure and populates it with data.
- `transfer_byte_and_call_library`: Loads a longword from an offset of 24 in a structure pointed to by a1 into d1, stores the byte from d1 into an offset of 67 in a structure pointed to by a2, loads constants into d0 and d1, calls a routine via a pointer in a6, and if d0 is non-zero it branches to another routine; otherwise it sets d3 to 254 and branches to another routine.
- `calculate_and_call_library_2`: Initializes a structure in d0 with a pointer to a global variable and calls two routines via pointers in a6.
- `calculate_and_call_library`: Stores a byte from d0 at offset 68 of a structure in a2, calculates a value by doubling d0 and multiplying it by 68, calls a routine at an offset from a library base in a6, and if the result in d0 is not zero, branches to another routine; otherwise, it sets d0 to 254 and branches to another routine.
- `copy_word_table_to_buffer`: Stores a pointer in a global variable and copies 34 words from a table in a global variable to a buffer pointed to by an argument, writing each word twice per iteration, repeating for a count derived from a byte in a global variable.
- `modify_register_bits`: Sets bit 1 of d1, tests bit 0 of d0, and performs bitwise AND operations on d3 before testing bit 1 of d1.
- `handle_io_and_free_resources`: Writes a byte and a word to a global variable, calls a routine via a global variable, conditionally calls two other routines via the same global variable, clears a word in the global variable and returns a combined value.
- `check_signed_byte`: Tests a byte at offset 206 of a structure in a5 and, if it is greater than zero, sets d3 to zero; otherwise it returns.
- `parse_narrator_command_stream_6`: Parses a stream of bytes from a pointer in a0, performing lookups in a table to decode command identifiers and updating three separate buffers and a counter in a structure pointed to by a5.
- `modify_and_branch_d1`: Negates d1, adds 111 to d1, compares d0 to 12288, and branches based on the comparison results or otherwise adds d2 to a0 and branches to another location.
- `parse_narrator_command_stream_5`: Iterates through a byte stream in a0, performing lookups in a table to identify command characters and updating a command buffer, status flags, and length values in a structure pointed to by a5.
- `parse_narrator_command_stream_4`: Parses a stream of bytes from a pointer in a0, updating a command buffer and status flags based on lookups in a table.
- `set_status_bit_on_match`: Compares a byte in d1 to 5; if equal, it sets bit 5 of a byte at offset d3 relative to the address in a6, then branches to another routine; otherwise it branches to another routine.
- `set_status_bit_6`: Compares a byte in d1 to 6; if equal, it sets bit 2 of a byte at offset -1 relative to the address in a6 plus the offset in d3, then branches to another routine.
- `write_byte_and_update_index`: Writes a byte from d1 to a memory location at the offset of d3 within a structure pointed to by a2, increments the index in d3, and if the byte is neither one nor two and the index is at least five hundred twelve, updates a global pointer by subtracting a value from a structure in a5 and sets the carry flag before returning.
- `clear_buffer_bytes`: Copies a pointer from a0 to a1, subtracts a value from an offset of 146 in a5 from a1, compares a1 to a 156-byte constant at an offset of 156 in a5, and if a1 is greater than or equal to that constant it branches to another routine; otherwise it clears a byte at an offset of d3 in a3 and a byte at an offset of d3 in a6, then branches to another routine.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends immediately.
- `update_buffer_and_counter_2`: Clears d0, loads a byte from a structure at an offset determined by a2 and d3, and if the byte is non-zero, decrements the 16-bit counter in d3.
- `update_buffer_and_counter`: Checks a bit in a longword retrieved from a table using a value in a4 and d0; if the bit is not set and a byte in d3 is greater than 3, it writes 4, 0, and 0 to three separate structures at an offset determined by d3, then increments d3.
- `store_word_to_offset`: Stores a word from d3 into a structure at offset 154 of a6 and sets the carry flag.
- `mask_and_set_status`: Iterates through a string starting at an offset from a5 until a null terminator or a byte of negative one is found.
- `mask_and_set_bit`: Performs a bitwise AND on d2 with a constant, and if the result is not zero, sets bit 4 of a byte at offset -3 of a structure in a0 before branching to another routine; otherwise, it branches to another routine.
- `check_and_set_status`: Loads a longword from a structure in a2 using an offset from d1, masks it with a constant, and if the result is not zero, compares a byte at offset -2 of a structure in a1 with 48; if the byte is 48, it sets bit 4 of a byte at offset -3 of a structure in a0 before branching to another routine, otherwise it branches to another routine.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `apply_text_formatting_attributes`: Iterates through a source byte stream, performing bitwise transformations on destination buffer bytes and setting specific bits in a secondary buffer based on values retrieved from a lookup table.
- `process_text_stream_to_buffer`: Iterates through a source byte stream, performing bitwise transformations and setting specific bits in a destination buffer based on the values of the source bytes.
- `process_narrator_text_stream`: Parses a byte stream from a source buffer, using a lookup table to determine formatting properties for each character.
- `calculate_and_branch`: Subtracts a word from d2 from d0, decrements the result by 1, and branches to another routine.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `patch_buffer_with_conditional_logic`: Iterates through a source buffer, comparing bytes against a target buffer and checking for end-of-stream markers.
- `compare_and_patch_buffer`: Compares bytes from a source buffer against a target buffer using a series of conditional checks and bitmask tests, then optionally writes bytes from the source buffer to the target buffer.
- `increment_and_test_byte`: Increments d1 by 1, checks if a byte at offset 6 of the structure in a0 is equal to 255, and if not, calls another routine and returns if the carry flag is set.
- `test_bit_and_branch`: Tests bit 5 of a byte at offset 3 of the structure in a0 and branches to another routine based on the result.
- `test_bit_with_table_lookup`: Loops reading a byte from a buffer and a word from a table indexed by a word, modifies the word based on bits in the byte, and exits when a specific bit is set or a termination condition is met.
- `check_word_limit`: Loads a word from offset 154 of the structure in a5 into d7, compares it to 512, and if it is less than 512, sets the carry flag and branches to another routine; otherwise it branches to shift_bytes_and_clear_fields.
- `shift_bytes_and_clear_fields`: Increments a word counter in a structure and shifts bytes backward in three buffers by a computed count, then writes a byte from a register and two zero bytes to the new ends of the buffers.
- `execute_multiple_subroutines`: Calls seven different subroutines in sequence and then clears d0 before returning.
- `clear_byte_if_not_full`: Loads a pointer from offset 40 of the structure in a5 into a6, compares the byte at that pointer to 255, and if they are not equal, writes 160 to offset 7 of the structure at the address in a6 before returning.
- `clear_byte_and_increment`: Clears a byte at offset 7 of the structure in a6, increments a6 by 8, and branches to another routine.
- `check_data_byte`: Loads three pointers from offsets 232, 1256, and 744 of a structure in a5, then checks if the byte at the first pointer is equal to 255; otherwise it returns.
- `process_data_stream_3`: Performs complex bitwise transformations and data updates on a buffer using values from a table, increments three pointers, and calls two different routines.
- `process_data_stream_2`: Iterates through a buffer in a0, performing bitwise transformations and data updates based on values read from a table, and calls another routine when a specific byte value is encountered.
- `process_data_stream`: Iterates through a buffer in a0, performing bitwise operations and data transformations based on values read from a table, and updates pointers in a1 and a2 until a byte containing 255 is encountered.
- `test_bit_and_set_byte`: Tests bit 6 of a value in register d1 and, if set, sets bit 2 of a byte at the address in register a2 before branching to another routine; otherwise, it branches to another routine.
- `set_bit_and_branch`: Tests bit 5 of d1 and branches to another routine if the bit is clear, otherwise it sets bit 2 of the byte at the address in a2 and branches to another routine.
- `set_bit_and_increment`: Sets bit 2 of a byte at the address in a2, increments a2 by 1, and branches to another routine.
- `increment_three_pointers`: Increments the values in a0, a1, and a2 by 1 and branches to another routine.
- `calculate_masked_value`: Loads a byte from an offset of 496 in the structure in a5 into d2, compares it to 255, and if not equal, masks the byte with 63 and adds it to d0 before branching to another routine.
- `update_pointer_and_allocate`: Stores d0 into offset 58 of the structure in a5, calculates a new address by shifting d0 left by 3 and adding 8, stores this new address into offset 54 of the structure in a5, and calls a routine at an offset from the address in a6.
- `update_structure_from_call`: Stores d0 into offset 40 of the structure in a5, checks bit 217 of the structure in a5, and if set, loads a value from offset 58 of the structure into d0, calls another routine, and if d0 is not zero, stores d0 into offset 44 of the structure in a5 before returning; otherwise it stores the initial d0 into offset 40 and returns.
- `expand_narrator_command_stream_2`: Iterates through a byte stream, using each byte to index into a global table to retrieve structured data and writing a sequence of expanded data blocks to a destination buffer.
- `expand_narrator_command_stream`: Iterates through a byte stream, using each byte to index into a global table to retrieve structured data and then writes a sequence of expanded data blocks to a destination buffer.
- `parse_narrator_command_stream`: Reads a stream of bytes from a source buffer, using each byte as an index into a global table to retrieve structured data including three byte values and a control byte.
- `interpolate_buffer_data_3`: Loops through a buffer, performing linear interpolation on bytes by calculating differences between values from a global variable and a table, and calls another routine to check and increment a pointer.
- `check_and_increment_pointer`: Shifts d0 left by 2, reads a longword from a table using the result as an offset, and if bit 15 and bit 11 of that longword are both set, increments a0 by 1 and branches to another routine; otherwise it branches to another routine.
- `interpolate_buffer_data_2`: Performs a linear interpolation between two values at a memory location and stores the result back to that location, repeating this process.
- `interpolate_buffer_data`: Performs a linear interpolation between two values at a memory location and stores the result back to that location, repeating this process for a specified count in d1.
- `increment_a6_pointer`: Increments the value in a6 by 1 and branches to another routine.
- `process_data_stream_elements`: Iterates through a data stream, using bytes as indices into a global table to perform comparisons and calculations.
- `loop_through_data_stream`: Loads a byte from a2 using d7 as an index, loads a byte from a1 and increments a1, masks the byte in d0, shifts d7 left by two, loads a longword from a table using a4 and d7, tests bits 10 or 11 of that longword, and branches or increments a0 in a loop.
- `process_narrator_data_table`: Iterates through a data structure, performing comparisons between values fetched from tables and using the results to clear or initialize memory blocks in a list.
- `load_table_values`: Loads a byte from a0 into d7, uses d7 to fetch a byte from a table in a4 into d2, compares d5 and d6, and if d6 is not greater than or equal to d5, loads a byte from a0 into d7 and a byte from the table in a4 into d3 before branching.
- `process_narrator_data_stream_2_2`: Iterates through a data stream to perform arithmetic comparisons and updates a table of values.
- `process_narrator_data_stream_2`: Iterates through a data stream to perform arithmetic comparisons and updates a table of values.
- `calculate_difference_and_branch`: Subtracts the values in d2 and d3 from the value in d0, stores the result in d4, sets d1 to 0, and then sets d1 to the value in d2 before branching to another routine.
- `adjust_pointer_and_loop_2`: Clears a word in a6 and a byte at offset 2 of a6, then checks bit 31 of d0 to decide whether to branch; otherwise it writes d6 to offsets 3 and 4 of a6, increments a6 by 8, and loops using d1 as a counter before branching to another routine.
- `adjust_pointer_and_loop`: Increments a pointer in a6 by 8, initializes d1 with the value from d3, and enters a loop that branches to another routine.
- `update_structure_fields_2`: Clears a 16-bit word at offset 0 and a byte at offset 2 of a structure in a6, then conditionally copies a byte from d6 to offset 3 and a word from d6 to offset 4 of the structure in a6 before incrementing a6 by 8 and looping based on d1.
- `clear_registers_and_branch_2`: Sets d1 to the value in d0, and sets d4 and d3 to 0 before branching to another routine.
- `copy_buffer_with_conditional_check`: Loops through a buffer, checking a bit in a table for each byte, and copies bytes and words to three separate destinations until a termination byte is encountered.
- `write_byte_to_three_pointers`: Writes the same byte from a register to the memory locations pointed to by three registers.
- `call_subroutines`: Calls two different subroutines and then returns.
- `mask_byte_in_structure`: Checks if a byte at a specific offset in a structure is equal to two hundred fifty-five; if not, it performs a bitwise AND on that byte with a mask of two hundred forty and increments the pointer, then branches to another routine.
- `check_byte_value`: Loads two offsets from a structure in a5 into a0 and a1, reads a byte from the address in a0, and returns if the byte is equal to 255; otherwise it branches to another routine.
- `update_linked_list_bits`: Traverses a structure using a0 and a1, reading values from a table to conditionally set bits 0, 1, 2, or 4 in the memory pointed to by a1 and its successors until a byte containing 255 is encountered.
- `process_linked_list_2`: Sets bit 0 of a byte at offset -1 of a1, checks bit 7 of d1 to decide whether to branch; otherwise it reads a byte from a0, compares it to 24 and 23 decimal, and if it matches neither, sets bit 1 of a1 and moves a1 to a3 before branching to another routine.
- `process_linked_list`: Iterates through a list of bytes, using each byte as an index into a table to check specific bits in a longword, and sets bits in the list elements based on those results and calls another routine.
- `set_bit_in_memory`: Sets bit 2 of a byte at the address pointed to by a3 and branches to another routine.
- `set_multiple_bits`: Sets bit 0 of a byte at offset -2 of a structure in a1, and sets bits 1 and 2 of a byte at offset -1 of the same structure before branching to another routine.
- `decode_variable_length_bitstream_3`: Decodes a bitstream using variable-length encoding to populate a buffer with values derived from dividing a calculated quotient by bytes read from a data structure.
- `update_bitstream_pointer`: Masks d0 with 63, tests bit 0 of the byte at the address in a2 and increments a2, and if the bit was set it shifts d0 left by 3 and adds the result to a6 before branching to another routine; otherwise it branches to another routine.
- `decode_variable_bitstream_to_buffer`: Decodes a variable-length bitstream from a source buffer and writes calculated byte values into a destination buffer using a division-based scaling method.
- `decode_variable_length_bitstream_2`: Reads a byte from a1 and masks it with 63, then checks bit 1 of a byte at a2 and increments a2; if bit 1 is set, it shifts the value left by 3, adds it to d1, and loops back to the start of the routine.
- `decode_variable_length_bitstream`: Decodes a bitstream from a table using variable-length encoding, performing divisions of a value from a structure by values extracted from the stream to write bytes to a destination buffer.
- `perform_division_and_decrement`: Swaps the high and low words of d5, adds d6 to d5, divides d7 by the result of that addition where d7 is loaded from offset 32 of a structure in a5, stores the byte result at offset -1 of a structure in a6, decrements a1 and a2, and branches to another routine.
- `calculate_and_store_quotient`: Calculates a quotient from a word in d7 divided by 110, then stores the result in a structure at an offset derived from a6 and branches based on a comparison.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `process_narrator_control_stream`: Iterates through three data streams using pointers in a5, checking control flags and byte values to conditionally call another routine and update the pointers.
- `fetch_control_data`: Shifts d5 left by two, loads a long word from a table using d5 as an offset from a4 into d3, and branches to another routine if bit 20 of d3 is set; otherwise it checks bit 25 of d3 and branches to another routine if it is not set, or sets d6 to 24 and branches to another routine.
- `increment_three_pointers_2`: Increments the values in a0, a1, and a3 by 1 and branches to another routine.
- `process_narrator_text_control_flags`: Tests various bits in registers to determine control flow, performs table lookups based on byte values, and calls decode_narrator_text_stream_3, compute_scaled_shifted_product, or test_bits_and_call depending on the state of the flags.
- `test_bits_and_call`: Tests bit 12 of d7 and branches if clear, tests bit 9 of d7 and branches if clear, otherwise loads 38 into d1, calls another routine, and branches to another location.
- `test_bit_and_branch_4`: Tests bit 4 of d7 and branches if set; otherwise it loads the value 38 into d1, calls another routine, and then branches to another location.
- `test_bits_and_branch`: Tests bit 4 of d7 and branches if set; otherwise it tests bit 2 of the byte at offset 1 of a3 and branches if set; if not set, it loads 27 into d1, calls a subroutine, and branches.
- `test_bit_and_dispatch_narrator`: Tests bit 11 of a register and, if not set, loads the value 22 into a register, calls another routine, and then branches to another location; otherwise, it branches to another location.
- `decode_narrator_text_stream_3`: Decodes a stream of bytes by performing lookups in tables to determine character values, applying bitwise shifts, and performing arithmetic scaling based on flags.
- `test_bit_flags`: Tests bit 1 of d6 and bit 1 of d7, branching to different locations if either bit is zero, otherwise it loads 16 into d1, calls another routine, and branches to another routine.
- `decode_narrator_text_stream_2`: Sets d1 to 22 and calls another routine, then branches to another routine.
- `decode_narrator_text_stream`: Processes a stream of encoded bytes by performing bitwise shifts and arithmetic operations based on values retrieved from a lookup table, then writes the resulting bytes to a destination buffer.
- `compute_scaled_shifted_product`: The routine multiplies the low word of an input register by a word input and returns a combined value formed from the high word of the product and the low word shifted right by five bits after adding a constant.
- `initialize_and_clear_buffers`: The routine stores several offsets relative to a base pointer into a global structure, increments a few of those pointers, clears a set of word fields, then zeroes six blocks of memory by iterating a fixed count.
- `increment_and_call_routines`: Increments a 16-bit word at offset 136 of a structure in a5, then calls four subroutines based on the zero and overflow flags, clears the condition code register, and returns.
- `process_compressed_data_stream_2`: Initializes several values from a structure in a5, then enters a loop that reads bytes from two pointers, performs arithmetic scaling and bitwise operations to write values to a destination buffer, and updates counters in the structure until a termination byte or a specific counter value is reached.
- `process_compressed_data_stream`: Tests a byte in d1 and branches based on the result; it checks bit 7 of a byte at an offset determined by d4 from a pointer in a4, tests bit 5 of d1, and if bit 5 is clear, it performs an AND operation on d1 with 15 and branches if the result is non-zero; otherwise, it increments a pointer in a2 and a pointer in a0, loads a byte from a pointer in a1 into d1, and branches back to an earlier instruction.
- `decode_compressed_stream`: Decodes a stream of data by reading bytes from a source and applying a transformation involving bitwise operations and a lookup table.
- `set_flag_and_store_indices`: The routine ORs a constant into a byte addressed by a base pointer and an index, then stores two short values derived from an index and a conditional copy into fields of a global structure.
- `process_narrator_text_stream_2`: Iterates through three synchronized buffers to parse control bytes, updating a text buffer and a secondary data buffer based on specific bit flags and character values.
- `process_flagged_data`: Shifts a word in d3 left by two, loads a long word from a table using the result as an offset, and branches based on the status of bit 13 of that word.
- `modify_byte_via_flag`: Checks bit 4 of a byte at an offset from a pointer in a6, and if that byte's lower nibble, after being shifted, is not between 0 and 254, it sets bit 3 and 2 of the byte at the same offset.
- `decrement_and_store_pointers`: Decrements three pointers in a0, a1, and a2 by 1, stores these pointers into a structure at offsets 124, 132, and 128 of a5, sets d7 to 0, moves the word in d4 to d0, and branches to another routine.
- `update_status_byte_2`: Performs a bitwise OR of d7 into a byte at an offset in a4, moves d0 to d2, compares a byte at an offset in a6 with 2 and branches if equal, compares a byte at an offset in a6 with 14 and branches if not equal, sets d1 to 255, sets d7 to 16, performs a bitwise OR of d7 into a byte at an offset in a4, and branches to another routine.
- `clear_byte_on_condition`: Checks bit 5 of a byte at an offset from a pointer in a4, and if bit 5 is not set, it compares a 16-bit value in d2 with d4; if d2 is less than d4, it clears a byte at an offset from a6.
- `swap_and_clear_byte_flags`: Moves a byte from a table at offset d0 of a6 to a table at offset d2 of a6, compares d0 and d2, and if they are not equal, clears the byte at offset d0 of a6 before looping using d0 as a counter.
- `prepare_set_bits_range`: Loads a pointer from an offset of 112 from a5 into a1, moves a 16-bit count from d4 into d0, and branches to another routine.
- `set_bits_in_loop`: Loops through a range of memory defined by a pointer in a1 and a count in d0, setting bit 0 of each byte encountered.
- `or_mask_into_buffer_with_early_exit`: The routine selects a mask based on a control byte, then ORs that mask into a series of bytes in one buffer while checking a flag in another buffer and returning early if the flag is set.
- `set_tile_and_mask`: The routine updates a byte in a screen buffer using a mask derived from a map byte, and optionally masks and sets a flag in another byte based on a scan leftwards in a second buffer.
- `accumulate_table_entries`: Reads a short value from offset 140 of a structure in a5, and if the short value at offset 48 is zero, it conditionally calls another routine four times and then calls another routine twice; finally, it adds the short value to eight consecutive long words in a table pointed to by offset 92 of the structure in a5.
- `calculate_and_store_clamped_byte`: Calculates a value using a formula involving a constant, a division by three, and arithmetic on two input values, then clamps the result between 125 and 165 before storing it as a byte at an offset from a pointer located in a structure.
- `clamp_value_and_store_char`: The routine clamps a short input to a lower and upper bound and stores the resulting byte at an address formed from a base pointer and an index.
- `adjust_table_values_loop_2`: Calculates a scaled difference between values in two tables and applies it to a third table through three sequential loops.
- `adjust_table_values_loop`: The routine repeatedly computes an absolute difference between two table entries, scales it and adds the result to a destination byte, then scans backwards through a range using flag bits to locate entries.
- `scale_and_store_byte_2`: Loops through a range of indices, using flag bits to select entries and computing scaled character values from two tables with multiplication and bit shifting, then stores the results into two destination tables.
- `scale_and_store_byte`: Calculates a value by multiplying a byte from a table in a2 by a constant, then stores the result as a byte into a table in a3 at an offset determined by d0 and a modified d2.
- `compute_scaled_char_values`: The routine loops over a range of indices, using flag bits to select entries and computing scaled character values from two tables with multiplication and shifts.
- `load_pointers_and_branch`: Loads a pointer from offset 104 of the structure in a5 into a0, a pointer from offset 112 into a1, and a word from offset 158 into d1, then copies d1 to d0 and branches to another location.
- `apply_audio_filter_coefficients`: Iterates through audio buffers to apply variable gain multipliers and DC offsets based on a control value in a register.
- `calculate_scaled_value_2`: Adds 64 to d2, shifts d2 right by seven, and adds the result to a byte at an offset in a3.
- `process_arithmetic_stream`: Adds 64 to d2, shifts d2 right by 7 bits, adds the result to a byte at offset d1 of a structure in a3, moves a byte from offset d0 of a6 into d2, subtracts 110 from d2, multiplies d2 by a constant, and branches to another routine if the result is positive, otherwise it subtracts 64 from d2 and branches to another routine.
- `update_table_byte_2`: Adds 64 to d2, shifts it right by 7, adds the result to a byte at offset d0 of a6, then reads a byte from offset d1 of a6, subtracts 110, multiplies it by 26, and either branches or subtracts 64 before branching to another routine.
- `update_table_byte`: Performs arithmetic on bytes from three different tables located at a2, a3, and a6 using offsets in d0 and d1, then either subtracts the result from a byte in the table at a3 or branches to another routine.
- `add_byte_to_table`: Adds a byte from d2 to a byte at an offset from a pointer in a2 and branches to another routine.
- `apply_audio_gain_and_offset_filters`: Iterates through audio buffers to apply variable gain multipliers and DC offsets based on a control value in a register.
- `transform_audio_buffer_data_2_2`: Adds 64 to d2, shifts it right by 7, adds the result to a byte at offset d0 of a3, then reads a byte from offset d1 of a3, extends it to a word, multiplies it by d7, and either branches or subtracts 64 before branching to another routine.
- `transform_audio_buffer_data_2`: Iterates through a data buffer using nested loops to perform arithmetic transformations on bytes, applying scaling and bit-shifting operations based on values read from a control table.
- `transform_audio_buffer_data`: Iterates through a data buffer using nested loops to perform arithmetic transformations on bytes, applying scaling and bit-shifting operations based on flags found in a secondary table.
- `process_audio_filter_coefficients`: Iterates through a buffer to apply gain multipliers and DC offsets by reading values from a table, performing fixed-point arithmetic, and updating destination bytes based on bitmask checks.
- `calculate_scaled_value`: Loads a byte from a table using an offset in d0 into d1, extends it to a word, multiplies it by 102, and then either subtracts 64 or branches to another routine based on the sign of the result.
- `update_destination_byte`: Adds a value derived from a 16-bit register shifted right by seven bits to a byte at a destination pointed to by a6 and offset by d0, then branches to another location.
- `adjust_tile_heights_with_mask`: The routine iterates over a byte range using a mask in a global variable and a table pointed to by a register, updates a byte in a destination buffer from a source buffer with a scaled difference based on a running maximum, and writes an accumulated scaled value to another buffer.
- `initialize_buffer_pointers`: Loads two pointers from a structure at offsets 78 and 68 into a2 and a0, clears d1, moves the value from d4 into d0, and branches to another routine.
- `modify_buffer_with_scaled_offset`: The routine iterates backwards through a buffer, checking if the high nibble of a byte is non-zero, and if so, adds a scaled value to the byte at a destination address based on the byte's value and its high bit.
- `scale_buffer_by_loop`: Decrements a counter in d0 and adds a value derived from a 16-bit register shifted right by seven bits to a byte at a2 offset by d0, looping until d0 reaches zero.
- `process_audio_buffer_transformations`: Iterates through a buffer in reverse to perform arithmetic subtractions between multiple source buffers based on a bitmask, then enters a second loop that performs bitwise transformations, error correction, and recursive calls to another routine to adjust audio sample data.
- `copy_and_offset_buffer_elements`: The routine iterates through a loop, copying a byte from one buffer to another, subtracting a value from it, and then adding 5 to the result stored in a third location before branching to another routine.
- `process_audio_buffer_adjustments`: Iterates through a buffer to copy bytes and apply bitwise transformations based on a lookup table, then performs arithmetic adjustments on multiple buffers using values derived from bit-testing and sign-comparison logic.
- `update_buffer_with_shifted_nibble`: The routine iterates through a loop, copying a byte from one buffer to another, and then adds a value to a byte in a third buffer where the value is derived from a masked and shifted nibble from a fourth buffer.
- `load_address_and_call_2`: Loads the address stored at a longword at offset 10708 into a4, calls another routine, and then branches to another location.
- `load_address_and_branch_6`: Loads a 32-bit constant into a4, calls another routine, and then branches to another location.
- `load_buffer_pointers`: Loads a pointer from an offset of a5 into a4, stores the value 110 in d6, and moves the value in d5 into d0 before branching.
- `transform_and_normalize_buffer`: Iterates through a buffer to perform arithmetic on bytes, then scans for a specific bit pattern to calculate a mean value for normalization.
- `process_buffer_data`: Decrements a counter in d0 while checking a bit in a buffer, then performs arithmetic on bytes in a buffer and copies them to other locations based on the value of a byte in d2.
- `process_buffer_and_maximum`: Copies bytes between two buffers while adjusting them by a constant, then if a specific byte is present, finds a maximum value in a third buffer to calculate and write a scaled result to two different locations.
- `initialize_accumulation_registers`: The routine clears registers d7 and d1 and moves a 16-bit value from d4 to d0, then branches to another location.
- `accumulate_bytes_and_write_results`: The routine accumulates bytes from a source area into a register, keeps a running maximum, derives a scaled character value, and writes that value and a derived value to several destination buffers with a conditional copy.
- `decrement_pointers_and_branch`: Loads three pointers from offsets 124, 128, and 132 of the structure in a5 into a0, a1, and a2, decrements each pointer by 1, sets d2 to 0, moves d4 into d0, and branches to another location.
- `apply_dynamic_audio_scaling`: Iterates through a buffer in a loop, applying variable scaling factors to multiple audio data streams based on bitwise flags in a control structure.
- `scale_offset_value`: Loads a pointer from an offset of a5 into a6, loads a byte from a6 at an offset in d0 into d1, subtracts 110 from d1, multiplies d1 by 26, and branches based on the sign of the result.
- `apply_audio_scaling_adjustments`: Iterates through a buffer, performing bitwise checks on data to determine scaling factors, then applies these factors by multiplying bytes and updating multiple destination buffers via pointers stored in a structure.
- `apply_scaled_audio_adjustments`: Iterates through a buffer, performing bitwise checks on data to determine scaling factors, then applies these factors by multiplying bytes from various source pointers and adding or subtracting the result from target memory locations using a sign-extension technique.
- `compare_and_branch_value`: Compares a 16-bit register to a value derived from d4, and if they are not equal, sets d7 to 64 and branches to another location.
- `set_constant_and_branch`: Sets d7 to 86 and branches to another location.
- `calculate_scaled_signed_value`: The routine loads two pointers from a table, reads a byte from the first pointer, multiplies it by 26, and if the result is negative, subtracts 64 from it.
- `apply_signed_scaling_to_buffers`: Adds 64 to a value in d2, performs an arithmetic shift right by 7 bits, then adds this result to a byte at offset d0 of a structure in a6 and subtracts it from a byte at offset d0 of a structure in a3, looping until d0 reaches zero.
- `branch_to_adjustment_logic`: The routine branches to another location.
- `copy_and_adjust_bytes_with_feedback`: The routine copies bytes between buffers, updates a feedback value by multiplying it with a table entry and shifting, subtracts the feedback from a byte, and writes adjusted bytes to two output areas.
- `check_narrator_flag_and_dispatch`: Calls a series of other routines and then checks a byte at offset 219 of a structure in a5; if the byte is non-zero, it calls another routine.
- `call_check_sentinel_routines`: Loads a pointer from an offset of 40 in a5, adds 3, 4, and 5 to that pointer respectively, and calls another routine after each addition.
- `interpolate_and_store_bytes`: The routine reads two bytes from a buffer, computes a scaled difference based on an index, accumulates the result, and stores successive byte results into the buffer at regular intervals.
- `check_sentinel_byte`: Sets register d1 to 8, reads a byte from a table at offset d1 in a0 into d6, and if the byte is not 255, branches to another location; otherwise it returns.
- `check_byte_and_pointer`: Compares d6 to 254 and d1 to 8, and if d1 is not 8, branches to another location; otherwise it adds 8 to a0 and branches to another location.
- `interpolate_and_store_bytes_2`: The routine calls another routine and then branches to another location.
- `scan_table_for_sentinel`: Increments d1 by 8, checks if the byte at offset d1 in a0 is zero, and if not zero, checks if it is 255; if it is 255, it returns, otherwise it branches to an earlier instruction in this routine.
- `increment_pointer_in_loop`: Compares d1 to 8 and, if not equal, adds 8 to a0 and branches to an earlier instruction in this routine.
- `scan_buffer_for_sentinel`: Iterates through a buffer in increments of 8 bytes, searching for a null terminator or a byte with the value negative one, and calls interpolate_and_store_bytes before continuing the search.
- `process_data_stream_4`: Iterates through a data stream, using values from a table to either increment a pointer by multiples of 8 or copy blocks of 128 bytes from a global table into a destination buffer.
- `execute_data_loop`: Performs a complex loop that iterates through data structures, performing arithmetic on bytes, updating pointers, and copying sequences of bytes from a table based on character matches in d3.
- `offset_pointer_by_index`: The routine shifts a 16-bit value in d0 left by 3 bits, adds the result to a pointer in a0, and branches to another location.
- `modify_register_on_match`: Shifts the value in d3 right by 2 bits, compares it to 42, and if equal, sets d3 to 96; otherwise it returns.
- `replace_byte_on_match`: Compares d3 to 43 and, if it is not equal, branches to another location; otherwise it sets d3 to 97 and branches to another location.
- `modify_byte_on_match`: Compares d3 to 44 and, if not equal, branches to another location; otherwise it sets d3 to 98 and branches to another location.
- `copy_bytes_from_offset_table`: Copies six bytes from a table located at an offset determined by a byte in a register to a destination pointed to by a register, then returns.
- `initialize_address_pointers`: Loads four different addresses into a0, a1, a2, and a4 from offsets of a5 and a constant, moves a constant into d6, and sets d4, d5, and d7 to specific byte values before branching.
- `set_bit_and_compare_byte`: Sets bit 31 of d4, clears d0 and d2, loads a byte from the address in a0 into d0, and if d0 is not equal to d5, branches to another location; otherwise it returns.
- `decode_bitstream_structure`: Iterates through a buffer to update a structure in a6 by performing bitwise masking and logical operations based on values read from the buffer.
- `process_bitstream_data`: Iterates through a buffer of bytes, performing bitwise masking and logical operations on a structure in a6 based on values read from the buffer.
- `calculate_table_offset`: Shifts the value in register d2 left by three bits and adds the result to the address in a6, then branches to another routine.
- `copy_bytes_via_lookup`: Copies bytes from a source to a destination by using values from a table as indices, repeating this process in a loop until a byte containing 255 is encountered.
- `copy_bytes_with_lookup`: Uses a byte from a table indexed by a value in a register to copy bytes from a source to a destination, repeating this process in a loop.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `compare_scaled_byte`: Loads a byte from a structure at offset 48 of a6, compares it to 255, and branches if they are not equal.
- `update_scaled_value`: Performs a loop that adds a value from a structure at offset 24 to a register, scales the result by dividing by 8, and stores it back at offset 24 of that structure.
- `compare_zeroed_byte`: Loads a byte from a structure at offset 48 of a6, clears d3, compares the byte to 255, and branches if they are not equal.
- `process_structured_data`: Runs a loop that calculates a value based on a table lookup and a running sum, then updates a byte at offset 24 of a structure every 7 iterations before incrementing the structure pointer by 8.
- `load_pointers_and_check_byte`: Loads three pointers into a0, a2, and a4 from offsets of a5 and a global table, loads a byte from the address in a0 into d0, and if d0 is not 255, branches to another location; otherwise it returns.
- `process_narrator_data_stream`: Iterates through a data stream, using characters as control codes to perform arithmetic operations on a series of 8-byte structures.
- `add_constant_to_offset`: Runs a loop that adds the constant 10 to a byte at offset 7 of a structure and increments the structure pointer by 8 each iteration.
- `load_constant_and_branch_3`: Loads the constant 250 into d3 and branches to another routine.
- `load_negative_constant_and_branch`: Loads the constant negative 6 from a register and branches to a location within another routine.
- `load_negative_constant_and_branch_2`: Loads the constant negative 6 from a small immediate value into a register and branches to a location within another routine.
- `store_fixed_byte`: Runs a loop that stores the byte value 230 at offset 7 of a structure and increments the structure pointer by 8 each iteration.
- `modify_byte_via_offset`: Runs a loop that reads a byte from a structure, subtracts 43, performs an arithmetic shift right by 2, adds the result to a byte at offset 7 of the same structure, and increments the structure pointer by 8.
- `decode_nibble_checksum_bytes`: The routine reads a pointer and a count from a structure pointed to by A5 and processes a byte array in two passes.
- `update_task_status`: Calls three functions via pointers stored in a global variable to retrieve values, stores these values at offsets 31 and 32 of a structure, calculates a bitmask based on two retrieved values into offset 308 of the structure, and enters an infinite loop that calls another routine and then a fourth function.
- `manage_device_status`: Enters a loop that checks bit 6 of a byte at an offset of a2, sets bit 0 of that byte, and calls another routine.
- `clear_status_bits`: Clears bit 0 and bit 1 of a byte at an offset of 34 from a pointer in a2.
- `prepare_speech_display_buffer`: Initializes a series of structures in a table, populates a bitmask table by performing bitwise operations on a byte from a structure, and calls update_narrator_display_buffer to refresh the display data.
- `update_narrator_display_buffer`: Copies a block of data from a source to a destination, performs bitwise operations to populate a bitmask table, and updates a structure containing text character data and timing parameters.
- `process_narrator_command_stream_2`: Iterates through a data stream to extract command bytes, updates a structure's internal counter, and executes a series of callback functions.
- `parse_narrator_command_stream_2`: Iterates through a data stream to extract command bytes, updates a structure's internal counter, and executes a series of callback functions from a table when a specific terminator byte is encountered.
- `process_narrator_text_stream_3`: Iterates through a data stream to perform bitwise operations and lookups in a table, summing three retrieved bytes and storing the result into a destination buffer.
- `parse_narrator_command_stream_3`: Iterates through a data stream to extract command bytes, updates a structure's internal counter, and executes a series of callback functions.
- `process_narrator_command_stream`: Iterates through a data stream to extract command bytes, updates a structure's internal counter, and executes a series of callback functions.
- `update_and_branch`: Subtracts 1 from d0, branches if the result is positive, otherwise writes d5 to offset 16 of a5 and branches to another routine.
- `execute_callback_loop`: The routine decrements a counter from a structure, executes a series of function calls from a table, and conditionally writes a value to a hardware register and a global variable.
- `enable_dma_if_flag_set`: The routine tests a flag byte in a structure pointed to by A5 and, if set, writes a value to a custom hardware register that enables DMA channels by setting bit 15 of DMACON.
- `update_interrupt_state_2`: Compares a byte from a structure to another byte, and if they differ, performs several updates to a linked list and a structure before decrementing a counter in a global variable and calling another routine.
- `decrement_counter_and_enable_interrupts`: The routine reads a byte from an offset in a structure pointed to by A6, decrements it and writes it back.
- `calculate_offset_and_branch_2`: Loads a byte from offset 68 of a3 into d2, multiplies d2 by 68, checks a word at offset 22 of a5, and if not zero, calculates a new address in a2 by adding the product to a pointer from offset 236 of a4, then branches to another location.
- `execute_callback_loop_4`: Sets bit 1 of a byte at offset 30 of a structure in a3, then enters a loop that sets bit 1 of a byte at offset 30 of a structure in a3 and calls a function pointer at offset 14 of a structure in a3.
- `execute_callback_loop_2`: Executes a loop that calls a function pointer from a structure, checks a byte at offset 30 of a3, and either sets bit 5 of that byte and returns or continues the loop, eventually updating a value at offset 210 of a5 and a hardware register.
- `set_status_bit_3`: Sets bit 5 of a byte at offset 30 of a structure in a3 and branches to another routine.

### Qume_LetterPro_20

- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `execute_triple_call_3`: Calls a function pointer from a global variable and two subsequent function pointers from a structure, returning 0 if all three calls succeed or a specific error code if they fail.
- `execute_triple_call_2`: Calls another routine three times and returns -1.
- `execute_triple_call`: Calls a routine through a pointer found at an offset from a global base three times, passing the value from a6 to the second and third calls.
- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_status_word`: Saves registers to the stack, loads three pointers from the stack into a0, a1, and a4, clears registers d1, d2, and d3, compares the word at the pointer in a0 to 1, and branches to a different location if they are not equal or otherwise branches forward.
- `process_printer_status_and_format_command`: Copies a sequence of bytes from a table into a buffer, then modifies the buffer based on status flags read from a structure pointed to by a register.
- `copy_byte_with_offset`: Calculates an address by adding a value in a4 to a value in d2, then copies a byte from a zero-based table to that address, and branches to another routine.
- `update_word_and_byte_4`: Compares a word at the address in a0 to 27 and a byte at the address in a1 to zero; if both conditions are met, it writes the word 32 to the address in a0, writes the byte 1 to the address in a1, and branches to another routine.
- `update_word_value`: Compares a word at the address in a0 to 27; if it is equal to 27 and the byte at the address in a1 is negative, it writes the word 4 to the address in a0 and branches to another routine; otherwise it returns.
- `update_threshold_value_twenty_nine`: Compares a word at the address in a0 to 28; if it equals 28 and the byte at the address in a1 is positive, it writes the word 33 to the address in a0, otherwise it returns.
- `update_threshold_values`: Compares a word at the address in a0 to 29; if it equals 29 and the byte at the address in a1 is positive, it writes the word 33 to the address in a0 and the byte 255 to the address in a1, then branches to another routine.
- `update_value_if_threshold_met`: Compares a word at the address in a0 to 29; if it equals 29 and the byte at the address in a1 is positive, it writes the word 2 into the address in a0 and branches to another routine, otherwise it branches to another routine.
- `update_word_if_negative_2`: Compares a word at the address in a0 to 30; if it is not equal, it branches, otherwise it checks if the byte at the address in a1 is negative, and if so, stores the word 32 at the address in a0.
- `update_word_and_byte_3`: Compares a word at the address in a0 to 31; if it is not equal, it branches, otherwise it checks a byte at the address in a1.
- `increment_byte_on_mismatch`: Compares a word at the address in a0 to 32, and if not equal, increments a byte at the address in a1 and branches to another location.
- `decrement_and_branch_2`: Compares a word at a0 with 33 and, if not equal, decrements a byte at a1 and branches to another location.
- `format_character_data`: If a word at a0 is 57, it writes two bytes and two calculated bytes to a destination, otherwise if the word at a0 is 0, it writes 253 to offset 2720 of a structure in d1.
- `add_constant_to_byte`: Calculates a value by adding 48 to a sign-extended word in d0 and writes the result as a byte to an address derived from a3, then branches to another routine.
- `write_status_byte`: Checks if a word at the address in a0 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in d1.

### CBM_MPS1000

- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_callbacks`: Calls a routine via a pointer at an offset from a global variable, and if successful, calls another routine via a pointer at an offset from a second global variable, and if that is also successful, calls a third routine via a pointer at an offset from the second global variable, finally calling a cleanup routine if any of the calls return zero.
- `perform_dual_calls`: Saves a6 to the stack, calls another routine using an offset from a6, restores a6, and repeats this process with a different value in a1 before branching to another routine.
- `execute_triple_function_pointer`: Fetches a function pointer from a global variable using an offset and calls that function three consecutive times.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry`: Checks a condition via a routine call, and if met, populates a structure with two parameters and executes several routines through a global base.
- `manage_printer_buffer_and_status`: Implements a multi-case state machine that manages a printer buffer by performing memory clears, bitwise OR operations, and formatting escape sequences.
- `restore_registers_and_return`: Clears register d0 and restores several registers from the stack before returning.
- `initialize_printer_parameters`: Saves registers to the stack, loads three pointers from the stack into a2, a3, and a4, clears registers d1, d4, and d5, sets d3 to 3, and branches to another routine if the word at the address in a2 is not equal to 1.
- `format_printer_command_buffer_3`: Copies a sequence of bytes from a table into a destination buffer, prepending specific control characters based on values read from a structure pointed to by a register.
- `modify_buffer_byte`: Loads a pointer from the address in d1 into a0, then loads a pointer from the address in a0 into a1, compares the word at offset 134 of the address in a1 to 2048, and if equal, writes the byte 15 to the address calculated by adding a4 to the value in d4, which is incremented by 1, then loads a long value from a global variable into d6 and branches to another routine.
- `process_printer_command_buffer`: Copies a null-terminated string from a source table into a destination buffer using a loop, then processes a command byte by updating a global variable, modifying a status byte at a specific offset in a structure, or appending specific byte sequences to the buffer based on various command codes.
- `map_status_loop`: Increments registers d4 and d5, then enters a loop that copies a byte from a table at offset d5 into a destination address calculated from a0 and a4, repeating until d5 reaches 4.
- `modify_memory_on_condition`: Compares a word at the address in a2 to 66; if it is not equal, it branches, otherwise it sets a byte at offset 2 of a d1 pointer to 1, loads a pointer from a0 into a1, compares a word at offset 2646 of the address in a1 to 2048, and if equal, stores the byte 96 at the address in a0 pointed to by d3.
- `write_byte_on_condition`: Loads a pointer from a0 into a1, compares a word at offset 1130 of the address in a1 to 1024, and if equal, writes the byte 137 to the address in d3 and branches to another location.
- `write_byte_and_branch`: Moves the address in d3 to a0, writes the byte 80 to the address in a0, and branches to another location.
- `copy_bytes_from_table`: Copies a sequence of bytes from a table into a destination memory location pointed to by the sum of a register and a pointer, incrementing both the source and destination indices until a counter reaches five.
- `modify_word_and_byte`: Compares a word at the address in a2 to 32; if it is not equal, it branches, otherwise it checks a byte at the address in a3.
- `clear_byte_if_negative`: Checks a byte at the address in a3; if it is negative, it clears that byte, writes the word 30 to the address in a2, and branches.
- `set_d0_and_branch`: Sets d0 to 255 and branches to another routine.
- `update_values_on_match`: Compares a word at the address in a2 to 33, and if equal, checks a byte at the address in a3, and if that byte is not zero, sets it to 255, writes the word 29 to the address in a2, and branches to another location.
- `clear_and_write_word`: Tests a byte at a3 and, if it is less than or equal to zero, clears that byte, writes the word 28 to a2, and branches to another location; otherwise it branches to another location.
- `branch_to_offset_2`: Performs a short branch to a specific offset.
- `map_status_values`: Compares a byte from a structure against several constants to write specific values to a stack location or a structure, and if the byte is zero, writes 253 to offset 2720 of a structure in d1.
- `write_status_byte_2`: Checks if a word at the address in a2 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in d1.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_630

- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_library_function_pointers`: Attempts to call three consecutive functions at a specific offset from a base address stored in a global variable, returning failure if any call returns zero; if the calls succeed, it calls a cleanup routine at a different offset from the same base address.
- `execute_dual_callback_sequence`: Loads two different pointers from a table and calls two separate routines using offsets from those pointers, then returns with a negative value in d0.
- `execute_triple_function_pointer_2`: Fetches a function pointer from a global variable using an offset and calls that function three consecutive times using the value from a register as an argument.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_command_parameters`: Saves registers to the stack, loads three pointers from the stack, clears three registers, and compares the word at the first pointer to one; if it is not one, it branches to a different location, otherwise it continues to another routine.
- `parse_control_sequence`: Copies a sequence of bytes from a source structure into a destination buffer, applying specific byte replacements based on values found at offsets 1658 and 1654 of the source structure.
- `process_string_state_and_copy_data`: Copies a null-terminated string from a table into a destination buffer using a loop, then performs a series of conditional checks on a word and a byte to update the word value and modify the byte based on specific constant comparisons.
- `copy_byte_from_table`: Copies a byte from a table at a pointer in d3 to a destination address calculated from d2 and a4, then performs a loop that continues as long as d0 is greater than d3.
- `update_word_and_byte_5`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is negative, it writes the word 32 to the address in a0 and the byte 1 to the address in a1, then branches to another routine.
- `check_word_and_byte`: Compares a word at the address in a0 to 27; if it is equal and the byte at the address in a1 is negative, it writes the word 4 to the address in a0 and branches to another routine, otherwise it branches to another routine.
- `set_specific_word`: Compares a word at the address in a0 to 28; if it equals 28 and the byte at the address in a1 is positive, it writes the word 33 to the address in a0 and branches to another routine.
- `update_conditional_word_and_byte`: Compares a word at the address in a0 to 29; if it equals 29 and the byte at the address in a1 is positive, it writes the word 33 to the address in a0 and the byte 255 to the address in a1, otherwise it returns.
- `write_small_word`: Compares a word at the address in a0 to 29; if it equals 29 and the byte at the address in a1 is positive, it writes the word 2 to the address in a0 and branches to another routine.
- `clamp_word_value_by_byte_sign`: Compares a word at the address in a0 to 30; if the value is not 30 and the byte at the address in a1 is negative, it writes 32 to the word at the address in a0 and branches to another routine, otherwise it branches to another routine.
- `adjust_word_value_by_byte_sign`: Compares a word at the address in a0 to 31; if it is not 31, it branches to another routine.
- `compare_and_increment_byte`: Compares a word at the address in a0 to 32; if it is not 32, it branches to another routine, otherwise it increments a byte at the address in a1 and branches to another routine.
- `decrement_and_branch_on_value`: Compares a word at the address in a0 to 33; if the value is not 33, it branches to another routine, otherwise it decrements a byte at the address in a1 and branches to another routine.
- `write_conditional_bytes`: Checks if a word at the address in a0 is 12; if so, it writes 27 to an address calculated from a4 and d2, and then writes either 65 or 66 to the next byte depending on a stack value; otherwise, if the word is 0, it writes 253 to an offset of 2688 from the address in d1.
- `write_constant_byte`: Writes the byte 66 to a destination address calculated from d2 and a4, then branches to another routine.
- `write_status_byte_to_structure`: Checks if a word at a pointer in a0 is non-zero; if it is zero, it writes the byte 253 to an offset of 2720 in a structure pointed to by a value in d1, then returns.

### ImagewriterII

- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_release_library_handles`: Attempts to call three consecutive functions via offsets from a pointer stored in a global variable, and if any call returns zero, it calls a release routine using an offset from that same global variable pointer to clean up the previous handles.
- `execute_dual_callback_sequence_2`: Loads two different pointers from a global variable and calls two separate routines via offsets from those pointers, then returns.
- `execute_printer_driver_function`: Fetches a function pointer from a table using an offset and calls it three times, passing the value from register a6 as an argument to each call.
- `return_constant_20`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_21`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_2`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine through a pointer stored within that structure.
- `POSDIV`: Performs a division operation using a loop if the divisor in a register is greater than 65535, otherwise it calculates the quotient and remainder using modulo and division operations.
- `LONGDIV`: Rotates a 32-bit value left by one bit for sixteen iterations, using the carry flag to transfer bits between the high and low halves.
- `ulmult`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products and returns the result.
- `ulmodt`: Moves the contents of register d1 into d2, moves the contents of register d0 into d1, calls POSDIV, and returns.
- `uldivt`: Moves the contents of register d0 into d1 and d1 into d2, calls another routine, and returns the resulting value from d1 in d0.
- `lmodt`: Calls POSDIV and negates the returned value if the value in an address register is nonzero, then returns the result.
- `ldivt`: Calls POSDIV and returns its first result, or the negation of that result if the second result in an address register is nonzero.
- `dispatch_via_table_2`: Saves registers to the stack, reads a byte from the stack into d0, and if the byte is not greater than 4, uses it to index into a table to jump to another routine; otherwise it jumps to a different address.
- `clear_d0_and_restore_registers_alt`: Clears register d0 and restores d2 through d5 and a2 through a4 from the stack before returning.
- `initialize_printer_handler`: Saves registers d2 through d4 and a2 through a4 on the stack, loads a pointer from offset 28 of the stack into a1, loads a 32-bit long from offset 32 of the stack into d2, loads a pointer from offset 36 of the stack into a3, clears d3, d4, and a4, and branches to another routine if the 16-bit word at the pointer in a1 is not equal to 1.
- `generate_imagewriter_escape_sequences`: Copies eleven bytes from a table into a destination buffer using a loop, then evaluates various flags and values from a structure to append specific escape sequences, including font selection, character spacing, and decimal string representations of numeric values, into the buffer.
- `format_printer_escape_sequences`: Evaluates a value from a table to select and write specific escape sequence bytes into a buffer, including generating decimal string representations of numeric values.
- `generate_epson_esc_commands`: Evaluates a value from a table to select and write specific escape sequence bytes into a buffer, including generating decimal string representations of a quotient and remainder and handling various control characters for printer command formatting.
- `update_byte_on_match`: Compares a word at the address in a1 to 31; if it matches and the byte at the address in a3 is not less than or equal to zero, it writes the word 33 to the address in a1 and the byte 1 to the address in a3, then branches to another routine.
- `update_byte_on_condition`: Tests a byte at the address in a3; if it is negative, it writes the word 32 to the address in a1, writes the byte 255 to the address in a3, and branches to another routine, otherwise it branches to another routine.
- `branch_to_address`: Branches to a specific address.
- `translate_imagewriter_escape_sequences_3`: Translates specific input characters into Apple ImageWriter II escape sequences by writing control characters and parameters to a destination buffer.
- `write_four_bytes`: Writes the bytes 66, 27, 108, and 48 to a destination address calculated from d2 and d3.
- `translate_imagewriter_escape_sequences_2`: Translates input bytes into specific escape sequences for an ImageWriter II printer by writing control characters and parameters to a destination buffer.
- `write_single_byte_2`: Sets a0 to the address in d3 plus d2, increments d3, and writes the byte 66 to the address in a0 before branching.
- `translate_imagewriter_escape_sequences`: Translates input bytes into specific escape sequences for an ImageWriter II printer by writing control characters and parameters to a destination buffer.
- `write_byte_at_offset`: Writes the byte 80 to a destination address calculated from d2 and d3, then branches to another routine.
- `encode_imagewriter_escape_sequences_2`: Translates specific command bytes into escape sequences for an ImageWriter II printer by writing control characters and parameters to a destination buffer.
- `write_byte_at_offset_70`: Writes the byte 112 to a destination address calculated from d2 and d3, then branches to another routine.
- `encode_imagewriter_escape_sequences`: Reads a command byte from a pointer in a1 and writes specific escape sequences starting with the escape character to a destination buffer.
- `process_printer_commands`: Checks a word at the address in a1 against several constants.
- `write_byte_in_loop`: Compares a byte at address 0 to 1; if it is not 1, it branches, otherwise it increments d3, calculates an address from d2 and d3, writes the byte 80 to that address, and branches.
- `update_printer_buffer_byte`: Increments a value in a register, calculates a new address by adding two registers together, writes the byte seventy to that address, and branches to another routine.
- `process_printer_data_2`: Checks a word at a1 for several specific values; it writes sequences of bytes to a destination address calculated from d2 and d3, modifies a value on the stack, and branches to another routine.
- `write_conditional_byte`: If a byte at a memory location is not equal to 1, it otherwise returns; otherwise, it writes the byte 69 to a destination address calculated by adding d2 and d3, then branches to another routine.
- `write_byte_at_offset_4e`: Writes the byte 78 to a destination address calculated from d2 and d3, then branches to another routine.
- `process_printer_data`: If a word at a1 is 48, it clamps a byte from the stack between 5 and 10, writes the bytes 27 and 115 to a destination, decrements the stack byte, and writes a byte from a table to a destination; otherwise, if a word at a1 is 12, it clamps a byte from the stack between 30 and 38, writes the bytes 27 and 75 to a destination, and writes a byte from a table to a destination.
- `update_byte_from_table`: If a word at the address in a1 is not equal to 12, it otherwise returns; otherwise, it checks a byte at a pointer from the stack, and if that byte is 39, writes 30 to that location, or if it is 37, writes 30 to that location, and if that byte is greater than or equal to 37, it branches to another routine.
- `write_mixed_data`: Writes the bytes 27 and 75 to a destination address calculated from d2 and d3, and writes a byte derived from a global variable and a value from the stack to the next address.
- `clear_d0_and_restore_registers`: Clears register d0 and restores d2 through d4 and a2 through a4 from the stack before returning.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_C-150

- `return_constant_22`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_library_calls`: Loads a value from a global variable, then performs up to three calls to functions at offsets from that value and its copies, and if certain calls return non-zero, it calls a function at an offset from the global variable before returning.
- `execute_dual_callback_sequence_3`: Loads two different pointers from a global variable and calls two separate routines using offsets from those pointers, then returns.
- `execute_indirect_calls`: Calls a routine through a pointer at an offset from a global base, then calls that same routine twice more using the value from register a6 as an argument.
- `return_constant_23`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_24`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_3`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte at specific offsets, then executes three routines through a global base and one routine through a pointer stored within that same structure.
- `divide_32_bit_integers`: If a value in a register is greater than 65535, it performs a sixteen-iteration bit rotation and subtraction loop; otherwise, it performs division and modulo operations using a 16-bit divisor.
- `rotate_32bit_value_with_threshold`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_3`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components and returns the result.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `dispatch_via_stack_offset`: Saves registers to the stack, retrieves several values from the stack, and performs a jump to an address found in a table using a value derived from the stack.
- `restore_registers_and_return_2`: Clears register d0 and restores several data and address registers from the stack before returning.
- `handle_printer_command`: Processes commands based on a word at param_1: if 1, it writes specific bytes to param_2 and param_4 and updates a counter; if 65, it updates a counter in param_6 and copies ten bytes from a global table into param_2; if 12, it calculates a value for param_4 based on param_6 and writes three bytes to param_2 using a table and a call to another routine; otherwise, if 0, it writes 253 to a global variable.
- `copy_byte_with_loop`: Copies bytes from a source table to a destination address calculated from a3 and a2 in a loop, where the loop continues as long as a register is greater than 11.
- `update_complex_structure`: If a word at a pointer in a3 is 12, it modifies a byte from the stack based on its value, writes three bytes to a location determined by d3 and d4 using a global table, and calls another routine to write a fourth byte; otherwise, if the word is 0, it writes 253 to offset 2720 of a structure in a4.
- `update_structure_from_stack`: Calculates a value using two values from the stack, writes three bytes to a location determined by d3 and d4, and then writes a byte from a global table into that location based on a value derived from a function call.
- `write_byte_to_offset`: Tests a 16-bit value at a pointer in a3; if it is zero, it writes the byte 253 to offset 2720 of a structure pointed to by a4.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### generic

- `return_constant_25`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_26`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_27`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero`: The routine returns a constant zero value.

### Brother_HR-15XL

- `return_constant_28`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_calls_2`: Loads a pointer from a global variable into a1 and calls a routine via an offset from a pointer in a6, then performs two additional calls using offsets from the same global variable, and finally calls another routine using an offset from the global variable if the previous calls return zero.
- `execute_triple_calls`: Loads a pointer from a global variable into a1, calls a routine via an offset from a pointer in a6, restores a6 from the stack, loads a new pointer into a1 from a different global variable, calls another routine via an offset from a6, restores a6 from the stack, sets d0 to 255, and branches to another routine.
- `execute_triple_function_call`: Calls a routine through a pointer at a specific offset in a global variable twice, then returns zero.
- `return_constant_29`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_30`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_31`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_parameters_2`: Saves registers to the stack, loads three pointers from the stack into a0, a1, and a4, clears registers d1, d2, and d3, compares the word at the pointer in a0 to 1, and branches based on the result.
- `parse_printer_command_sequence`: Copies a sequence of bytes from a source buffer into a destination buffer, performing conditional checks on specific values at offsets within a structure.
- `process_printer_command_sequence_2`: Copies a string from a source table to a destination buffer and then processes a command byte and a parameter byte through a series of conditional checks to update the command byte and parameter value.
- `copy_bytes_with_counter_2`: Copies bytes from a source table to a destination address calculated from a4 and d2 in a loop, where the loop continues as long as a register is greater than 11.
- `update_value_on_match`: Compares a word at the address in a0 to twenty-nine; if it matches, it writes thirty to that address, writes one to the address in a1, and branches to another routine; otherwise it returns.
- `update_value_if_twenty_nine_2`: Compares a word at the address in a0 to twenty-nine; if it equals twenty-nine and the byte at the address in a1 is non-positive, it writes the word four to the address in a0 and branches.
- `update_value_if_twenty_nine`: Compares a word at the address in a0 to twenty-eight; if it is not equal or if the byte at the address in a1 is non-positive, it branches to another routine, otherwise it writes thirty-three to the word at the address in a0 and branches to another routine.
- `reset_value_if_twenty_nine`: Compares a word at the address in a0 to twenty-nine and checks if the byte at the address in a1 is non-positive; if both conditions are met, it writes thirty-three to the word at a0 and two hundred fifty-five to the byte at a1, then branches to another routine.
- `update_word_if_negative_alt2`: Compares a word at the address in a0 to 29; if it is not equal, it branches, otherwise it checks if the byte at the address in a1 is positive, and if so, stores the word 2 at the address in a0.
- `update_word_if_negative`: Compares a word at the address in a0 to 30; if it is not equal, it branches, otherwise it checks a byte at the address in a1.
- `update_word_and_byte_2`: Compares a word at the address in a0 to 31; if not equal, it branches.
- `increment_on_match`: Compares a word at the address in a0 to 32; if not equal, it branches.
- `decrement_and_branch_3`: Compares a word at the address in a0 to 33 and branches if not equal, otherwise it decrements a byte at the address in a1 and branches to another routine.
- `write_conditional_bytes_2`: If a word at a pointer in a0 is 12, it writes the byte 27 to a location determined by a4 and d2, and writes either 65 or 66 to the next byte depending on a value from the stack; otherwise, if the word is 0, it writes 253 to offset 2720 of a structure in d1.
- `write_constant_byte_2`: Writes the byte 66 to a destination address calculated from a4 and d2, then branches to another location.
- `write_byte_on_null_check`: Checks if a word at the address in a0 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d1.

### HP_LaserJet

- `return_constant_32`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_cleanup_library_pointers`: Attempts to call three consecutive functions via offsets from a pointer stored in a global variable, and if any call returns zero, it executes a cleanup routine via an offset from that same global variable before returning.
- `execute_dual_callback_sequence_4`: Loads a pointer from a global variable into a1, pushes a6 to the stack, and calls a routine via a pointer at an offset of 406 from a value loaded from a global variable.
- `execute_indirect_function_calls`: Fetches a function pointer from a global variable using a specific offset and calls that function three times in succession.
- `return_constant_33`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_4`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base using a pointer retrieved from that structure.
- `process_printer_state_machine`: Manages a state machine for printer control sequences by updating a state byte and a character buffer based on input values.
- `update_printer_state_and_buffer_2`: Copies bytes from a table into a buffer, checks specific values at offsets in a structure to set character bytes at specific offsets, and manages a state machine by incrementing or decrementing a counter based on control characters.
- `process_printer_escape_sequences`: Updates a character buffer and manages state transitions for printer control characters, including handling specific escape sequences and incrementing or decrementing byte values based on character matches.
- `copy_table_to_memory`: Copies values from a global table to a memory location in a loop that runs until a counter reaches 11, then calls another routine twice using values from a structure.
- `reset_values_on_match`: Compares a word at the address in a2 to 27; if not equal, it branches.
- `update_word_on_match_2`: Compares a 16-bit word at the address in a2 to 27; if it matches and the byte at the address in a3 is less than 0, it writes 4 to the address in a2 and branches to another routine.
- `update_word_if_match`: Compares a word at the address in a2 to 28; if it is equal and the byte at the address in a3 is greater than 0, it writes 33 to the address in a2 and branches to another routine.
- `update_word_if_positive_2`: Compares a word at the address in a2 to 29; if it is not equal, it branches, otherwise it checks if the byte at the address in a3 is non-zero, and if so, stores the word 33 at the address in a2 and the byte 255 at the address in a3.
- `update_word_if_positive`: Compares a word at the address in a2 to 29; if it is not equal, it branches, otherwise it checks a byte at the address in a3.
- `update_word_if_match_alternative`: Compares a word at the address in a2 to 30; if it is equal and the byte at the address in a3 is at least 0, it writes 32 to the address in a2 and branches to another routine.
- `update_word_based_on_byte`: Compares the word at the address in a2 to 31; if not equal, it branches to another routine, otherwise it checks the byte at the address in a3.
- `compare_word_and_increment_byte`: Compares the word at the address in a2 to 32; if it is not equal, it branches to another routine, otherwise it increments the byte at the address in a3 and branches to another routine.
- `decrement_status_byte`: Compares the word at the address in a2 to 33; if it does not match, it decrements the byte at the address in a3 and branches.
- `write_byte_to_pointer`: Compares a word at the address in a2 to 64; if it matches, it loads a pointer from an offset of 28 in a6, checks if the byte at that pointer is non-zero, and if so, writes the longword from a global variable to that pointer before branching to another routine.
- `decrement_and_copy_data`: Decrements a byte at the address stored in an offset of a6, updates a global word with the new value, conditionally copies a byte from d4 to an offset of that same address, copies eleven bytes from a global table to a buffer in a4, and calls another routine twice.
- `copy_byte_to_address`: Increments a longword from an offset of a6, loads a byte from that new address into d1, and writes that byte to the address held in d4 before branching to another routine.
- `copy_and_call_functions`: Copies eleven bytes from a global table to a destination buffer using a loop, then calls another routine twice with different arguments derived from a structure in a6.
- `compare_and_branch_4`: Compares a word at the address in a2 to 57 and branches if not equal, otherwise it branches to another location.
- `copy_table_and_call`: Copies eleven bytes from a global table to a destination buffer using a loop, then calls another routine twice with different arguments.
- `write_byte_on_null_check_alt`: Checks if a word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d2.
- `convert_value_to_ascii`: Checks a byte against 200.
- `convert_byte_to_ascii`: Compares the byte in d0 to 99; if it is not less than or equal to 99, it branches.
- `convert_to_ascii_2`: Writes the byte 48 to a memory location, checks if a byte in d0 is less than 10, writes 48 to the next memory location if it is, otherwise writes the result of dividing d0 by 10 plus 48 to the next memory location, calls another routine, and writes the returned byte plus 48 to the following memory location.
- `convert_to_ascii`: Writes the byte 48 to a memory location, calls another routine, and then writes the returned byte incremented by 48 to the next memory location.
- `call_two_subroutines`: Calls a routine at an offset in a global variable with two parameters, then calls another routine at a different offset in the same global variable.
- `rotate_and_threshold_32bit_3`: Performs a 32-bit division where the dividend is split into two 16-bit halves and the divisor is used to calculate a quotient and remainder.
- `rotate_and_threshold_32bit`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_4`: Computes the 32-bit product of two 32-bit values by summing the products of their 16-bit components.
- `negate_result_on_flag`: Calls another routine and negates the returned integer value if the returned word in a0 is nonzero.
- `dispatch_jump_table_entry`: Loads a byte from the stack, compares it to five, and if it is less than five, uses it to calculate an offset into a jump table to call another routine; otherwise, it performs a calculation and jumps to a different address from the table.
- `restore_registers_and_return_3`: Clears register d0 and restores registers d2, d3, a2, a3, and a4 from the stack before returning.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `check_word_range`: Pushes registers a2 and a3 to the stack, reads a word from the stack, and compares it to 768.
- `compare_word_range`: Compares a word in d0 to 256 and branches if it is less than 256, otherwise it branches if it is greater than 256, otherwise it branches to another routine.
- `compare_and_branch_2`: Compares the word in d0 to 512 and branches if not equal, otherwise it branches to another location.
- `compare_word_and_branch`: Compares a word in d0 to 1024 and branches to different locations depending on the result.
- `initialize_structure_fields`: Writes several 32-bit and 16-bit values into a structure pointed to by a1 and a location pointed to by the first word of that structure, and sets a global variable to 48.
- `initialize_structure_values_2`: Writes 800 to offset 38 of a structure, 1060 to offset 30 of the same structure, 100 to offset 36 of the structure, 100 to offset 34 of the structure, and 49 to offset 45 of the structure, then branches to another routine.
- `initialize_structure_values_with_byte`: Writes 1168 to offset 38 of a structure, 1590 to offset 30 of the same structure, 150 to offset 36 of the structure, 150 to offset 34 of the structure, 49 to offset 45 of the structure, 53 to the address in a2, and then branches to another routine.
- `initialize_structure_values`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 33, and writes 30 to two memory locations.

### Okidata_292

- `return_constant_34`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_chain_4`: Loads a global variable and uses it to call a function via a pointer at an offset, then conditionally calls another function via a pointer at an offset, and finally calls a third function via a pointer at an offset before returning.
- `execute_dual_callback_sequence_5`: Loads a pointer from a global variable into a1, pushes a6 onto the stack, and calls a routine at an offset from a pointer in a6.
- `call_function_triple`: Calls a function pointer located at an offset from a global variable three times, passing the value from register a6 to the second and third calls.
- `return_constant_35`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_36`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_5`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine through a pointer retrieved from the structure.
- `manage_dynamic_buffer_structure`: Implements a multi-mode buffer management system that calculates offsets, performs bitwise updates to data, clears memory ranges, and populates a structured table with specific byte sequences and values from a global variable.
- `clear_d0_and_restore_registers_2`: Sets register d0 to zero and restores d2 through d4 and a2 through a4 from the stack before returning.
- `initialize_command_buffer`: Saves registers to the stack, loads several values from the stack into registers, and if a word at the address in a2 is one, it branches to another routine; otherwise it returns.
- `build_printer_command_buffer`: Copies a sequence of bytes from a table into a destination buffer, prepending specific control bytes based on values read from a structure.
- `check_and_write_byte`: Loads a pointer from a list, checks if a word at an offset of that pointer is 2048, and if so, increments a pointer in d3, adds d2 to it, and writes the byte 15 to that location.
- `process_printer_command_sequence`: Copies a null-terminated string from a source buffer to a destination buffer, then processes a command byte by updating a global variable, modifying a structure at an offset in a pointer, or appending specific byte sequences to the destination buffer based on various conditional checks.
- `loop_copy_from_table`: Increments d3 and d6, then enters a loop that copies a byte from a table at offset d1 to a destination at d0 plus d2, continuing until d6 reaches 4.
- `modify_memory_on_condition_alt`: Compares a word at the address in a2 to 66; if it is not equal, it branches, otherwise it sets a byte at offset 2 of a d1 pointer to 1, loads a pointer from the address in a1 into a0, compares a word at offset 2646 of the address in a0 to 2048, and if equal, stores the byte 96 at the address in a0 pointed to by d5.
- `write_byte_at_pointer_2`: Loads a pointer from the address in a1 into a0, compares a word at offset 1048 of the address in a0 to 1024; if not equal, it branches, otherwise it moves the address in d5 to a0 and writes 137 to the address in a0, then branches.
- `write_constant_byte_3`: Moves the address in d5 to a0, writes the byte 80 to the address in a0, and branches.
- `write_bytes_from_table_loop`: Writes two specific bytes and a word from a table to a memory location calculated from a pointer and an offset, then increments the counters and repeats the process if a limit has not been reached; otherwise it returns.
- `reset_status_values`: Compares a word at the address in a2 to 32; if not equal, it branches.
- `clear_and_set_word`: Tests a byte at the address in a3; if it is not greater than or equal to zero, it clears that byte and writes the word 30 to the address in a2, then branches to another routine.
- `load_byte_and_branch`: Loads the value 255 into a register and branches to another routine.
- `update_status_word`: Compares a word at the address in a2 to 33; if it is not equal, it tests a byte at the address in a3, writes 255 to that byte if it is zero, writes 29 to the address in a2, and branches to another routine.
- `clear_and_set_word_2`: Tests a byte at the address in a3; if it is not less than or equal to zero, it clears that byte and writes the word 28 to the address in a2, then branches to another routine.
- `branch_to_another_routine`: Performs a short relative branch to another routine.
- `map_status_values_2`: Compares a byte at the address in a2 against several constants to set a value on the stack, then either writes a sequence of three bytes to a destination or writes a byte to an offset of a structure in d1.
- `check_and_update_status`: Compares a word at the address in a2 with 12; if it is not 12, it branches; otherwise, it checks if a byte at the address in a0 is 39, and if so, writes 30 to that address; finally, if a byte at the address in a0 is not less than 37, it branches to another routine.
- `write_bytes_from_table`: Writes the bytes 27 and 114 to the address in a2 plus an offset, then writes a word from a table to the address in a2 plus 2.
- `write_status_byte_3`: If the word at the address in a2 is zero, it writes the byte 253 to offset 2720 of the structure pointed to by the address in d1.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Okimate_20

- `return_constant_37`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_calls_3`: Loads a value from a global variable and attempts to call three different functions located at specific offsets from that value, returning 0 if all calls succeed or a specific constant if they fail.
- `execute_nested_callbacks_2`: Calls a routine at an offset from a library base using a pointer from a global variable, restores a6, calls another routine at an offset from a library base using a different global pointer, and branches to another routine.
- `update_handler_pointer`: Calls a function at an offset from a global pointer three times and then updates a global variable with its own address.
- `update_function_pointer`: Loads a long value into a0, compares the word at offset 100 of the address in a0 to 2, and if they are equal, it loads the address of a label into a0 and stores it at offset 54 of a global variable before branching to another location; otherwise it branches to another location.
- `set_function_pointer`: Stores the address of this routine into a global variable at offset 54 and returns zero.
- `return_constant_38`: Returns the constant an address in d0 to the caller without any other effect.
- `manage_dynamic_buffer_operations`: Dispatches between several sub-tasks via a switch statement: it calculates buffer offsets and sizes, performs bitwise OR operations on data at specific offsets, clears a memory range with a loop, writes a sequence of bytes to a buffer including three repeated sets of values, and updates a global variable based on a bitmask.
- `clear_d0_and_restore_registers_3`: Sets register d0 to zero and restores several data and address registers from the stack before returning.
- `manage_buffer_state_machine`: Implements a state machine that handles buffer initialization, byte-wise bitwise OR operations, memory clearing via a loop, and writing a specific sequence of bytes including a 16-bit value into a memory buffer.
- `return_zero_and_restore_registers`: Clears the d0 register and restores d2 through d6 and a2 through a5 from the stack before returning.
- `perform_64bit_division`: Performs a division of a 32-bit value by a 16-bit value using a loop for larger divisors, otherwise calculates the quotient and remainder using modulo and division operations.
- `rotate_and_threshold_32bit_2`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_5`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components.
- `forward_call_to_helper_3`: Moves the contents of register d1 into d2 and the contents of register d0 into d1, calls a routine to measure a string length, and returns.
- `initialize_buffer_parameters`: Saves registers to the stack, loads several pointers from the stack into a0, a2, and a3, clears d2 and d5, sets a4 to zero, and branches based on whether a word at the address in a0 is equal to 1.
- `populate_complex_buffer`: Copies fifteen bytes from a global table to a destination buffer using a loop, writes several bytes to that buffer based on values in a structure, and includes a loop that writes the byte thirteen to the buffer a specific number of times.
- `populate_data_buffer`: Writes specific byte sequences to a buffer based on values found at offsets of a structure, and includes a loop that writes the byte twelve a variable number of times.
- `fill_buffer_with_carriage_return`: Fills a memory buffer with the value 13 using a loop that continues until a counter reaches a value loaded from a memory location pointed to by a pointer in a register.
- `update_okimate_printer_state`: Processes printer control codes by updating internal state variables and generating command sequences, including carriage returns and specific escape sequences, based on the input command value.
- `fill_buffer_with_carriage_return_2`: Fills a memory buffer with the value 13 using a loop that continues until a counter reaches a value loaded from a memory location pointed to by a pointer in a table.
- `process_okimate_printer_commands`: Processes printer control codes by updating command values or generating a command sequence that includes two specific bytes followed by a loop that fills a buffer with the value thirteen.
- `fill_buffer_with_constant`: Fills a buffer with the value 13 using a loop that continues until a counter reaches a value loaded from a memory location.
- `update_command_values`: Compares a word at the address in a0 to 32; if it is not equal, it tests a byte at the address in a2, and if that byte is zero, it writes 1 to the address in a2, writes 1 to the address in a0, and branches to another routine; otherwise it branches to another routine.
- `clear_and_write_value`: Tests a byte at the address in a2; if it is negative, it branches; otherwise, it clears that byte, writes 30 to the address in a0, and branches to another routine.
- `set_register_and_branch`: Sets register d0 to negative one and branches to another location.
- `check_command_status`: Compares the word at the address in a0 to 33, and if it is not equal, it tests the byte at the address in a2; if that byte is not zero, it sets the byte at the address in a2 to 255, sets the word at the address in a0 to 29, and branches to another location; otherwise it branches to another location.
- `clear_status_byte_2`: Tests a byte at the address in a2; if it is not less than or equal to zero, it clears that byte and writes 28 to the address in a0, then branches to another routine.
- `branch_to_another_routine_2`: Sets register d0 to negative one and branches to another routine.
- `map_command_values`: Compares a byte at a pointer against several constants and writes specific byte or word values to a destination based on the match.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_Advantage_D25

- `return_constant_39`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_calls`: Loads a global variable and uses it to call a routine at an offset, then conditionally calls another routine at an offset from a second global variable, and finally calls a routine at an offset from the first global variable if certain conditions are met.
- `execute_dual_calls`: Loads two addresses from a table using a pointer in a1 and a6, calls a routine at an offset from the first address, then calls a routine at an offset from the second address before returning.
- `execute_triple_callback`: Calls a routine through a pointer in a global table, then calls that same routine twice more with the value from register a6 as an argument.
- `return_constant_40`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_41`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_42`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_printer_status_type`: Loads three pointers from the stack, clears three registers, and compares the word at the first pointer to one; if it is not one, it branches to a different location, otherwise it continues to another routine.
- `parse_and_format_string`: Copies a sequence of bytes from a source structure into a destination buffer, applying specific byte replacements based on values found at offsets 2650 and 2646 of a structure pointed to by a register.
- `process_input_stream`: Copies bytes from a source in d4 and d3 to a destination in a4 using a loop, updates a global word with two values from a pointer in d1, and performs a series of conditional checks and assignments on a word at a0 and a byte at a1.
- `copy_byte_in_loop`: Copies bytes from a source table to a destination buffer in a loop, incrementing two counters, and branches back to the start of the loop until a counter reaches 11.
- `update_status_flag_on_match`: Compares a word at the address in a0 to 27 and a byte at the address in a1 to zero; if both conditions are met, it writes 32 to the address in a0 and 1 to the address in a1, otherwise it returns.
- `update_word_on_negative_byte`: Compares a word at the address in a0 to 27 and branches if not equal, then checks if the byte at the address in a1 is negative; if it is, it writes the word 4 to the address in a0.
- `update_threshold_value`: Compares a word at the address in a0 to 28; if it is equal, it checks if the byte at the address in a1 is positive, and if so, writes the word 33 to the address in a0 before branching to another routine.
- `set_value_and_byte`: Compares a word at the address in a0 to twenty-nine; if it equals twenty-nine and the byte at the address in a1 is non-zero, it writes the word thirty-three to the address in a0 and the byte two hundred fifty-five to the address in a1 and branches.
- `set_threshold_value`: Compares a word at the address in a0 to 29; if it is equal, it checks if the byte at the address in a1 is positive, and if so, writes the word 2 to the address in a0 before branching to another routine.
- `update_word_if_negative_3`: Compares a word at a0 to 30; if it is not 30 and the byte at a1 is not negative, it writes 32 to the word at a0 and branches to another routine.
- `update_word_on_condition_2`: Compares a 16-bit word at the address in a0 to 31; if it is not 31, it branches; otherwise, it checks a byte at a1, and depending on whether that byte is greater than 0 or less than or equal to 0, writes either 33 or 32 to the address in a0, then clears a byte at a1 and branches.
- `increment_byte_on_match`: Compares a word at the address in a0 to 32; if it is not equal, it branches to another routine; otherwise it increments the byte at the address in a1 and branches to another routine.
- `decrement_byte_on_match`: Compares a 16-bit word at the address in a0 to 33; if they are not equal, it otherwise decrements a byte at the address in a1 and branches.
- `write_status_byte_if_zero`: Checks if a 16-bit word at the address in a0 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d1.

### Alphacom_Alphapro_101

- `return_constant_43`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_library_call_sequence`: Attempts to call three consecutive functions located at specific offsets from two different base addresses stored in a global variable and a passed parameter.
- `execute_dual_indirect_calls`: Loads a pointer from a global variable into a1, then performs two sequential calls to routines located at negative offsets from addresses stored in two different global variables.
- `execute_indirect_function_calls_2`: Fetches a function pointer from a table using an offset and calls that function three times, passing the value from a register as an argument to the first call and the subsequent calls.
- `return_constant_44`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_45`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_46`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_local_registers`: Saves six registers to the stack, loads three values from the stack into a1, a4, and a2, and clears d1, d2, and d3 before branching to another routine if the word at the address in a1 is not 1.
- `format_alphacom_printer_command`: Copies twelve bytes from a table into a buffer and appends an escape sequence containing specific control characters and spaces based on values read from a structure.
- `fill_buffer_with_sequence_2`: Fills a buffer starting at an offset determined by a4 and d1 with the byte 32 in a loop, then writes a sequence of three bytes, followed by another loop that fills more memory with the byte 32, and finally writes a sequence of six bytes.
- `fill_buffer_with_sequence`: Fills a buffer starting at a specific offset with the byte 32 in a loop, then writes a fixed sequence of seven bytes to the subsequent memory locations.
- `update_status_and_value`: Compares a 16-bit word at the address in a1 to 27; if it is not 27, it checks if the byte at a2 is non-zero, otherwise it writes 32 to the address in a1 and 1 to the address in a2 before branching.
- `compare_and_update_word`: Compares a 16-bit word at the address in a1 to 27; if it is not 27, it checks if the byte at a2 is non-zero, otherwise it writes 4 to the address in a1 and branches.
- `update_word_on_positive_byte`: Compares a word at the address in a1 to 28 and branches if not equal, then checks if the byte at the address in a2 is greater than 0; if it is, it writes the word 21 to the address in a1.
- `update_word_on_condition_3`: Compares a 16-bit word at the address in a1 to 29; if it is not 29 or if the byte at a2 is not greater than 0, it branches; otherwise, it writes 33 to the address in a1, writes 255 to the address in a2, and branches.
- `update_word_on_condition`: Compares a 16-bit word at the address in a1 to 29; if they are equal and a byte at the address in a2 is greater than zero, it writes the word 2 to the address in a1.
- `update_word_on_non_negative_byte`: Compares a word at the address in a1 to 30 and branches if not equal, then checks if the byte at the address in a2 is greater than or equal to 0; if it is, it writes the word 32 to the address in a1.
- `modify_word_and_byte_2`: Compares a word at the address in a1 to 31; if it is equal, it checks the byte at the address in a2.
- `increment_byte_on_word_match`: Compares the word at the address in a1 to 32; if not equal, it branches to another routine, otherwise it increments the byte at the address in a2 and branches to another routine.
- `compare_word_and_decrement_byte`: Compares the word at the address in a1 to 33; if it is not equal, it branches to another routine, otherwise it decrements the byte at the address in a2 and branches to another routine.
- `format_printer_command_buffer_2`: Depending on the value at the pointer in a1, it writes specific escape sequences and byte patterns to a buffer at a4 plus an offset in d1.
- `write_byte_to_offset_2`: Adds a pointer in a4 to a pointer in d1, then writes the byte 66 to that resulting address before branching.
- `write_buffer_data`: If the word at a1 is 66, it writes a byte, then enters a loop to write the byte 8 to 80 consecutive memory locations, followed by a sequence of four bytes, then enters a second loop to write the byte 32 to 80 locations, and finally writes a sequence of five bytes; otherwise, if the word at a1 is 0, it writes the byte 253 to an offset of 2688 from a pointer in d2.
- `write_status_byte_if_zero_2`: Checks if a 16-bit word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d2.

### Epson_JX-80

- `return_constant_47`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_library_function_pointers_2`: Attempts to call three consecutive functions at specific offsets from a base address stored in a global variable, returning zero if all calls succeed or returning a failure value if any call returns zero.
- `execute_dual_callback_sequence_6`: Loads two different pointers from a global variable and calls two separate routines via offsets from those pointers, then returns with negative one in d0.
- `execute_printer_driver_function_2`: Calls a function located at an offset from a pointer stored in a global variable three times in succession.
- `return_constant_48`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_49`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_6`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a constant byte, then executes four routines through a global base using pointers retrieved from the structure and other global variables.
- `format_printer_command_buffer`: Constructs a sequence of escape sequences and data bytes in a memory buffer to control an Epson JX-80 printer.
- `clear_d0_and_restore_registers_4`: Clears d0 and restores d2 through d4 and a2 through a4 from the stack before returning.
- `setup_buffer_state`: Saves registers to the stack, loads pointers from the stack into a2, d1, and a3, clears d3, d6, and d2, sets d4 to 6 and d5 to 3, and branches based on whether a word at the address in a2 is equal to 1.
- `format_epson_jx80_command_buffer`: Copies a sequence of bytes from a table into a destination buffer and appends control characters based on values read from a structure.
- `process_epson_printer_command`: Parses a command byte and associated data to generate a sequence of control bytes for an Epson printer.
- `copy_modified_byte`: Loads a pointer from the stack into a0, reads a byte from it, subtracts 1, writes this byte to the address in a1, increments the pointer from the stack, and writes the byte at the new pointer address to the address in a0.
- `write_sequence_to_memory_2`: Calculates a destination address using d0, d1, and d3, then enters a loop that writes a byte from a table at a0 using an index in d2 to the destination, incrementing d3 and d2 in each iteration until d2 reaches 8.
- `process_okimate_printer_commands_2`: Processes printer control codes by updating internal state variables or generating command sequences, including an escape sequence that copies eight bytes from a table into a buffer, or sequences involving escape characters followed by specific command bytes.
- `load_and_check_pointer`: Loads a pointer from the address in d2 into a1, then loads a word from that pointer into a0 and compares it to 1024; if they are not equal, it branches, otherwise it loads d4 into a0, writes 137 to the address in a0, and branches.
- `write_byte_and_branch_2`: Writes the byte 80 to the address in a0 and then branches.
- `copy_bytes_from_table_2`: Copies up to eight bytes from a table to a destination address calculated from two pointers, incrementing the source and destination pointers after each byte is moved, and otherwise returns.
- `update_word_and_byte`: Compares a 16-bit word at the address in a2 with 32, otherwise it branches; if the byte at a3 is not signed, it sets the byte at a3 to 1, writes 27 to the address in a2, and branches to another location.
- `clear_byte_and_word`: Checks the signedness of the byte at a3; if it is negative, it clears the byte at a3, writes 30 to the address in a2, and branches to another location.
- `set_d0_and_branch_2`: Sets d0 to 255 and branches to another location.
- `set_byte_and_word`: Compares a 16-bit word at the address in a2 to 33; if it matches and the byte at the address in a3 is not zero, it writes 255 to the address in a3, writes 29 to the address in a2, and branches to another routine.
- `clear_byte_and_set_word`: Checks if the byte at the address in a3 is greater than 0; if so, it clears the byte at the address in a3, writes 28 to the address in a2, and branches to another routine.
- `branch_to_another_routine_3`: Branches to another routine after a preceding routine sets a register to two hundred fifty-five.
- `process_printer_command`: Checks a byte at a2 against several values to write specific bytes to a stack location or a destination address calculated from d1 and d3, or writes 253 to an offset of 2704 of a pointer in d2 if the value is 0.
- `handle_printer_command_2`: If the word at a2 is 4, it writes 27, 106, and a byte from the stack to the address in d1 plus d3; otherwise, if the word at a2 is 12, it checks if a byte at the stack is 39, and if so, changes it to 30; if that byte is less than 38, it writes 27, 114, and a word from a table to the address in d1 plus d3; if the word at a2 is 0, it writes 253 to offset 2688 of the structure pointed to by d2.
- `modify_pointer_and_byte`: Compares a 16-bit word at the address in a2 with 12; if not equal, it branches; otherwise, it loads a pointer from a stack offset, checks if the byte at that pointer is 39, and if so, writes 30 to that location; it then checks if the byte at that pointer is 37 and, if not less than or equal to it, branches to another location.
- `write_sequence_to_memory`: Writes the byte 27 and the byte 114 to a memory location, then writes a word from a global table to the following address.
- `write_status_byte_if_zero_3`: Checks if a 16-bit word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d2.
- `call_library_vector_198_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Epson

- `return_constant_50`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_function_call_2`: Calls a function pointer at an offset from a global variable three times, and if all three calls return non-zero, it returns 0; otherwise, it calls another routine with the global variable as an argument and returns -1.
- `perform_dual_calls_2`: Loads a pointer from a global variable into a1, saves a6 to the stack, loads a pointer from a global variable into a6, calls the routine at the address in a6, restores a6 from the stack, loads a pointer from a global variable into a1, saves a6 to the stack, loads a pointer from a global variable into a6, calls the routine at the address in a6, restores a6 from the stack, and sets d0 to 255 before branching.
- `execute_triple_pointer_call`: Calls a routine through a pointer found at an offset from a global base, passing the same value in a6 to it three times.
- `return_constant_51`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_52`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_7`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine via a pointer stored in the structure.
- `manage_printer_buffer_state`: Dispatches different operations based on a parameter, including clearing a buffer with a loop, writing escape sequences and data to a memory buffer, performing bitwise OR operations on bytes, and calling another routine to handle data streaming or status checks.
- `clear_d0_and_restore_registers_subset`: Clears d0 and restores d2 through d3 and a2 through a4 from the stack before returning.
- `setup_buffer_state_alt`: Saves registers to the stack, loads pointers from the stack into a1, a4, and a3, clears d3, d5, and d1, sets d2 to 6 and d4 to 3, and branches based on whether a word at the address in a1 is equal to 1.
- `format_printer_command_buffer_4`: Copies a sequence of bytes from a table into a destination buffer and appends control characters based on values read from a structure.
- `process_printer_command_byte`: Reads a byte from a pointer and performs conditional logic based on its value.
- `transfer_byte_sequence`: Loads a pointer from the stack into a0, reads a byte from it, subtracts 1, writes this byte to the address in a1, increments the pointer from the stack, loads a pointer from a2, and writes the byte at the incremented pointer address to the address in a0.
- `copy_byte_from_table_3`: Calculates an address by adding a4 to the value in d3, increments d3, and copies a byte from a table at offset d0 to that address; it then compares d5 to 8 and branches to another routine if d5 is greater than 8, otherwise it branches to another routine.
- `process_okimate_printer_commands_3`: Updates printer control codes and command sequences based on the value in a1, including writing specific bytes to a buffer, updating status flags, and copying eight bytes from a table to a destination.
- `load_and_check_pointer_alt`: Loads a pointer from the address in d1 into a1, then loads a word from that pointer into a0 and compares it to 1024; if they are not equal, it branches, otherwise it loads d2 into a0, writes 137 to the address in a0, and branches.
- `update_printer_status_byte`: Loads a pointer from a register into a0, writes the value 80 to the memory location pointed to by a0, and branches to another routine.
- `write_table_byte_to_buffer`: Increments a pointer and a counter, then writes a byte from a table into a destination address calculated by adding a base address to the pointer.
- `check_and_set_byte`: Compares a 16-bit word at the address in a1 to 32; if it is not 32, it branches, otherwise it checks if the byte at a3 is non-zero, and if it is zero, it writes 1 to the address in a3, writes 27 to the address in a1, and branches.
- `clear_and_write_word_2`: Checks if the byte at a3 is less than 0; if so, it clears the byte at a3, writes 30 to the address in a1, and branches.
- `set_register_and_branch_2`: Sets d0 to 255 and branches.
- `modify_pointer_values`: Compares a 16-bit word at the address in a1 to 33; if it matches and the byte at the address in a3 is not zero, it writes 255 to the address in a3, writes 29 to the address in a1, and branches to another routine.
- `clear_byte_and_set_word_alternative`: Checks if the byte at the address in a3 is greater than 0; if so, it clears the byte at the address in a3, writes 28 to the address in a1, and branches to another routine.
- `jump_to_offset_470`: Performs a short branch to a location 470 bytes ahead.
- `handle_command_logic`: Checks a word at a1 against several values to write specific bytes to a stack location or a destination address calculated from a4 and d3, or writes 253 to an offset of 2704 of a pointer in d1 if the value is 0.
- `write_byte_if_zero`: Checks if a word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d1.
- `call_library_vector_198_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### HP_LaserJet_PLUS

- `return_constant_53`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_function_calls`: Loads a pointer from a global variable into a1, then attempts to call three functions via offsets from that pointer, returning 0 if all three calls succeed or -1 otherwise.
- `execute_triple_call_sequence`: Loads a pointer from a global variable into a1, then performs two consecutive calls to functions located at offsets 8 and 12 of a structure pointed to by a6, using a stack-based preservation of a6.
- `execute_printer_driver_function_3`: Fetches a function pointer from a table using an offset and calls it three times, passing the value from a register as an argument to each call.
- `return_constant_54`: Returns the constant an address in d0 to the caller without any other effect.
- `check_then_populate_request`: Calls a routine through a pointer in a global base structure and, if that check returns zero, stores a 16-bit value and two 32-bit parameters into fields of that structure, clears a byte flag, then invokes three further routines through the global base and returns zero; otherwise it returns the check result unchanged.
- `update_printer_escape_state`: Manages a state machine for printer control characters by updating a state byte and a character buffer based on input values.
- `configure_hp_laserjet_printer_parameters`: Copies a sequence of bytes from a table into a buffer, checks specific hardware status flags to set character values at offsets 7, 14, and 18, and then updates a character buffer and manages state transitions for printer control characters by calling process_printer_escape_sequences.
- `update_printer_state_and_buffer`: Updates a buffer by copying values from a table and manages a state machine that modifies bytes based on specific control characters.
- `copy_table_elements`: Copies elements from a table into a destination buffer using a loop that runs until a counter reaches eleven, then calls another routine twice with specific values derived from a structure.
- `update_status_on_match`: Compares a word at the address in a2 to 27, and if it matches, checks if the byte at the address in a3 is non-zero; if both conditions are met, it writes 32 to the address in a2 and 1 to the address in a3.
- `update_escape_sequence_byte`: Compares a word at the address in a2 to 27; if it matches and the byte at the address in a3 is less than 0, it writes the word 4 to the address in a2 and branches to another routine, otherwise it returns.
- `update_word_if_twenty_eight`: Compares a word at the address in a2 to 28; if it is equal and the byte at the address in a3 is greater than 0, it stores 33 at the address in a2 and branches to another routine, otherwise it branches to another routine.
- `update_word_on_condition_4`: Compares a word at a2 to 29; if it is 29 and the byte at a3 is zero, it writes 33 to the word at a2 and 255 to the byte at a3, then branches to another routine.
- `update_word_on_match`: Compares a 16-bit word at the address in a2 to 29; if it is not 29, it returns, otherwise it checks if the byte at the address in a3 is greater than 0, and if so, writes 2 to the word at the address in a2.
- `update_word_if_thirty`: Compares a word at the address in a2 to 30; if it is equal and the byte at the address in a3 is less than 0, it stores 32 at the address in a2 and branches to another routine, otherwise it branches to another routine.
- `clamp_word_and_reset_byte`: Compares a word at a pointer to 31 and clamps it to 32 or 33 based on a signed byte comparison, then optionally clears that byte and branches to another routine.
- `compare_and_increment_byte_2`: Compares a word at the address in a2 to 32; if it is not equal, it increments the byte at the address in a3 and branches to another location.
- `decrement_and_branch_on_match`: Compares a word at the address in a2 to 33; if it matches, it decrements a byte at the address in a3 and branches to another routine, otherwise it returns.
- `check_and_update_printer_status`: Compares a word at the address in a2 to 64 and branches if not equal; otherwise, it checks a byte at the address stored in a6 plus offset 28, and if that byte is non-zero, it writes the byte from a global variable to the address stored in a6 plus offset 28 before branching to another routine.
- `decrement_and_update_printer_buffer`: Decrements a byte at a pointer stored in a structure offset 28 from a6, updates a global variable with the new value, and conditionally updates a second byte in that structure using a value from d4.
- `increment_and_write_byte`: Increments a value stored in a global variable by one, reads the byte at that new address, and writes it to a destination address held in register d4, then branches to another routine.
- `copy_data_and_call_routine`: Copies 11 bytes from a global table to a destination buffer at an offset in a6, then calls another routine twice using values derived from a pointer at an offset in a6.
- `compare_and_branch_word`: Compares a word at the address in a2 to 57; if the values are not equal, it branches to a different location, otherwise it branches to another location.
- `copy_table_to_buffer_and_call`: Copies eleven bytes from a global table to a destination buffer using a loop, then calls another routine twice with arguments 3 and 7 respectively.
- `write_printer_status_byte`: Checks a word at a location pointed to by a register and, if it is zero, writes the byte value 253 to an offset of 2720 in a structure pointed to by another register, then returns.
- `convert_value_to_ascii_2`: Writes a byte to the address calculated from param_2 and param_3 based on the value of param_1, then calls another routine and writes the returned value plus 48 to the address at offset 2 from the original destination.
- `convert_byte_to_ascii_2`: Compares the byte in d0 to 99, and if d0 is greater than 99, it writes 49 to the address in a2, increments d3, and adds the value of d4 to a2 before branching.
- `convert_digit_to_ascii`: Writes 48 to the address in d4 plus d3, then if the byte in d0 is less than 10, it writes 48 to the next address; otherwise, it writes the result of dividing d0 by 10 plus 48 to the next address, calls another routine, and writes the returned byte plus 48 to the following address.
- `store_formatted_character`: Stores the byte 48 at an address calculated from two registers, calls another routine, and then stores the result of that routine incremented by 48 at the next address.
- `call_two_global_routines`: Calls a routine at offset 100 of a global base with two parameters, then calls another routine at offset 104 of the same base.
- `rotate_32bit_value_5`: If a value in d2 is greater than 65535, it performs a loop that rotates a 32-bit value left by one bit for sixteen iterations, otherwise it performs a division-based calculation.
- `rotate_32bit_value_4`: Rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_6`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `negate_result_on_flag_3`: Calls another routine and negates the returned value if the low word of the returned value is nonzero.
- `dispatch_jump_table`: Restores registers from the stack, reads a byte from the stack, and performs a jump to an address calculated from a table based on that byte.
- `restore_registers_and_return_4`: Clears register d0 and restores multiple registers from the stack before returning.
- `call_library_vector_198_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `initialize_conditional_structure`: Saves registers a2 and a3 to the stack, loads a word from the stack into d0, and branches based on whether d0 is less than 768, greater than 768, or equal to 768.
- `check_value_range`: Compares the word in d0 to 256; if it is less than 256, it branches to a specific location; if it is greater than 256, it branches to another location; otherwise, it branches to a third location.
- `compare_and_branch_3`: Compares d0 with 512; if not equal, it branches, otherwise it branches to another location.
- `compare_word_and_branch_2`: Compares a word in d0 to 1024 and branches to different locations based on the result.
- `initialize_structure_fields_2`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 48, and writes 55 to two memory locations.
- `setup_structure_values`: Writes 800 to offset 26 of a structure, 1060 to offset 30, 100 to offset 34, 34 to offset 34, and 49 to offset 45 of a structure pointed to by a1, then branches to another location.
- `setup_structure_values_alt`: Writes 1168 to offset 26 of a structure, 1592 to offset 30, 150 to offset 34, 34 to offset 34, 53 to a memory location, and 53 to offset 45 of a structure pointed to by a1, then branches to another location.
- `initialize_printer_structure`: Writes the values 2432 and 3214 to offsets 26 and 30 of a structure pointed to by a register, writes 300 to offsets 36 and 34 of that structure, sets a global variable to 51, and writes 48 to two unspecified memory locations.

### 12

- `return_constant_55`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_2`: The routine loads the value zero into a register and returns.

### 15

- `return_constant_56`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_57`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_3`: Sets the return register to zero and returns.

### 12

- `return_constant_58`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_4`: The routine returns zero immediately.

### 20

- `return_constant_59`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_5`: Clears the accumulator and returns zero.

### 11

- `return_constant_60`: Returns the constant an address in d0 to the caller without any other effect.

### Balance of Power - The 1990 Edition (1990)(Mindscape)[a].adf:CLI

- `parse_command_line_arguments`: Parses a command line string by iterating through characters to identify tokens, handling quoted strings and expanding specific escape sequences like newline or escape characters, and stores the resulting pointers in a table.
- `invoke_two_library_function_pointers`: Loads a pointer from an offset of 92 in a6 into a0, calls a function at a negative offset from a6, repeats this for a second function at a different negative offset from a6, and returns.
- `call_and_store_result`: Calls a routine using an offset from a6, stores the returned value in a global variable, and returns if the result is not zero.
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

- `execute_illegal_instruction`: library code executes an illegal instruction.
- `store_pointer_to_global`: library code stores the value in a0 into a global variable.
- `manage_library_list`: As library code, it checks a value at an offset in a structure; if zero, it moves a value into a pointer and calls a function pointer, otherwise it sets bit 3 of a byte at offset 14 of the structure.
- `set_status_bit`: library code sets bit 3 of a byte at offset 14 of a structure in a6 and returns zero.
- `increment_and_initialize_on_condition`: library code checks if bit 3 of a byte at offset 14 of a structure in a6 is clear; if so, it increments a 16-bit value at offset 32 of that structure and, if the value becomes one, calls another routine with a global variable as an argument.
- `return_negative_one`: library code returns negative one in d0.
- `decrement_counter_and_call_2`: Library code that decrements the 16-bit counter at offset 32 of the structure at a6.
- `mask_and_call_library_routine`: Library code that masks bit 7 of d0, calls another routine, and sets bit 1 of the condition code register.
- `process_bitfield_loop`: As library code, it checks if d0 is zero or negative, and if positive, enters a loop that performs bitwise operations and arithmetic to update a value in d2.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `process_and_mask_value`: Copies the value from d0 to d7, calls a routine through a pointer in a register, sets bit 0 of d0, clears bit 3 of d0, and branches to another routine.
- `call_jump_table_entries`: Library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_d1_register`: As library code, it clears the d1 register and branches to another instruction.
- `execute_conditional_logic_2`: Library code retrieves a pointer from a global variable, calls a routine at an offset of negative 54 from that pointer, and then performs a series of conditional calls to other routines based on the state of d7 and other flags before returning a value with bits 0 and 2 cleared.
- `execute_conditional_library_logic`: Library code that calls a function via an offset from a pointer in a5, then performs a series of conditional calls to other functions at various offsets from a5 based on the values of d6 and d7.
- `execute_conditional_library_calls_2`: Library code that performs a series of calls to functions at various offsets from a pointer in a5 based on the values of d6 and d7, then returns a value with the bottom three bits cleared.
- `execute_conditional_library_calls`: This library code moves d7 to d1, calls a routine at an offset of minus 60 from a5, and branches based on the result.
- `call_and_restore_d2`: Copies d2 to d0, calls a routine at an offset of minus 60 from a5, copies the result from d0 back to d2, and branches to another routine.
- `select_constant_or_move_register`: Moves a longword from register d2 to d0, then checks register d4; if d4 is non-zero, it branches to another routine, otherwise it loads a constant into d2 and branches to another routine.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `perform_bit_shift_loop`: Library code that performs a loop involving bit-shifting and arithmetic on a value derived from d7, potentially calling another routine if a counter reaches zero.
- `perform_bit_shift_and_setup`: Library code that performs a logical shift right on d7, sets d6 to zero, sets d5 to a constant, loads a pointer into a0, and sets d1 and d2 to small constants before branching.
- `execute_loop_with_branching`: The library code executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_stack_value_library`: As library code, pushes the word negative two onto the stack and branches to another routine.
- `push_stack_value_library_alt`: As library code, pushes the word negative one onto the stack and branches to another routine.
- `push_constant_one`: Library code that pushes the constant one onto the stack and branches to another location.
- `check_byte_and_adjust_stack`: Compares a byte in d7 to 184, branches if it is higher, increments the stack pointer by 2, and otherwise returns after testing d7.
- `perform_library_math_loop`: This library code performs a loop that uses bit shifts and additions or subtractions to manipulate values in d, and calls another routine.
- `perform_math_loop_2`: As library code, it calls another routine, then performs a loop that uses bit shifts and additions or subtractions to manipulate values in d6 and d7 while updating a pointer to a global variable.
- `perform_math_loop`: This library code performs a loop that uses bit shifts and additions or subtractions to manipulate values in d6 and d7 while updating a pointer to a global variable.
- `perform_arithmetic_loop_3`: As library code, it performs a loop that uses bit shifts and additions/subtractions to manipulate values in d6 and d7, while also updating a value in a global variable and a counter.
- `perform_arithmetic_loop_2`: Library code executes a loop that performs arithmetic operations on d4, d5, d6, and d7, updates a pointer in a0, and decrements a 16-bit counter in d1, eventually calling other routines based on the results.
- `load_constant_and_branch`: Loads the constant an address into d7 and branches to another routine.
- `recursive_call_wrapper`: Library code that branches to itself and then returns.
- `call_routine_with_stack_storage`: Library code that pushes d5 onto the stack, calls another routine, stores d6 at an offset from the stack pointer, restores d6 from the stack, and branches to another location.
- `compare_and_branch_d6`: This library code moves d6 to d4, branches if d4 is negative, otherwise compares d6 to 255 and branches if it is higher, and if not, clears d6 and returns.
- `shift_and_branch_on_zero`: Arithmetically shifts d4 right by 8 bits, increments d4, and branches if d4 is zero; otherwise it jumps to another routine.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_calls_2`: Library code checks a byte in d7 and, if it is non-zero, performs a series of conditional calls to other routines based on the result of a carry flag check.
- `rotate_constant_and_branch`: Loads a constant into d7, rotates d7 right through the carry bit, and branches to another routine.
- `load_constant_and_branch_2`: Loads a constant into d7 and branches to another routine.
- `test_stack_value_and_branch`: Loads a word from the stack into d6, tests if d6 is positive, and otherwise sets d7 to zero and branches to another routine.
- `set_status_and_restore_registers`: Sets bit 1 of the condition code register and restores registers d1 through d6 and a0 from the stack before returning.
- `set_status_and_return`: Loads a constant value into a register, adjusts the stack pointer by an offset of 30, and returns.
- `process_complex_math_logic`: Library code that performs a series of conditional checks on d7 and calls various other routines based on the resulting values and bitwise operations.
- `execute_multiple_calls_2`: Library code that calls another routine four times and then returns.
- `perform_arithmetic_loop`: This library code performs arithmetic on d3 and d4, adds d4 to d5 and d3 to d6, and decrements a value from a table pointed to by a0 in a loop controlled by d1.
- `subtract_table_from_accumulator`: Subtracts d4 from d5, subtracts d3 from d6, adds a value from a table pointed to by a0 to d7, and repeats this in a loop while decrementing d1 until it reaches zero.
- `branch_on_sign_bit`: Tests the sign bit of d7, and if d7 is negative, it masks d7 with 127, calls another routine, and sets bit 1 of the condition code register before returning.
- `set_d7_and_branch`: Sets register d7 to negative one and performs a short branch.
- `perform_complex_math`: Executes a loop involving bit-shifting and conditional jumps, calls three different routines, and performs arithmetic on values derived from d7.
- `complex_loop_with_branching`: The library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `mask_and_call_routine`: As library code, it tests the sign of d7; if it is negative, it branches to another routine, otherwise it masks d7 with 127, calls another routine, and sets the overflow flag.
- `process_conditional_data`: Library code calls two routines, then performs a series of conditional checks on d7 to determine whether to call other routines or perform bitwise manipulations on a value derived from d7.
- `transform_d7_register_value`: Adds the value in d7 to itself, then conditionally performs a series of bitwise operations including an XOR, an arithmetic shift, a subtraction, a word swap, and a bitwise rotation before returning.
- `modify_d7_with_overflow_checks`: Swaps d7, rotates it, performs bitwise XOR and addition, and uses conditional branches based on the overflow flag.
- `test_and_modify_d7`: This library code performs several conditional branches based on the value of d7, including adding 133 and rotating d7 right by 1 bit if certain conditions are met.
- `test_word_bits`: The library code performs a bitwise AND on a 16-bit value in d7, then branches to another routine if the result is zero, otherwise it clears d7 and branches to another routine.
- `compare_and_shift_d7`: Compares d7 to 254, branches if not equal, otherwise shifts d7 right by 8 bits and then shifts it right by 1 bit, branching if the result is not zero.
- `shift_and_set_flag`: The library code shifts d7 left by 8 bits, sets d7 to 255, performs a rotate right through the carry on a byte in d7, sets the 2nd bit of the condition code register, and branches to another routine.
- `clear_d7_and_branch`: The library code clears d7 and branches to another routine.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `load_address_to_a0`: Library code that loads a longword from an offset of 12e into a0 and then branches to the next instruction.
- `load_address_pointer`: This library code loads a longword address into a0 and then branches to itself.
- `load_address_to_a0_3`: As library code, it loads the address at offset 28c into a0 and branches to itself.
- `load_zero_address`: Library code loads zero into a0 and then returns immediately.
- `load_library_address`: As library code, loads the address of a constant into a0 and branches to another routine.
- `load_library_address_a0`: As library code, loads a longword from a specific memory location into a0 and then branches to the start of the routine.
- `load_library_address_a0_alt`: As library code, loads a longword from a different memory location into a0 and then branches to the start of the routine.
- `store_register_via_pointer`: The library code invokes another routine via a pointer in a0, then stores a 32-bit value from register d6 into memory addressed by a pointer passed in a register, then returns.
- `load_address_to_a0_alt`: As library code, it loads the address at offset 34 into a0 and branches to itself.
- `load_zero_address_to_a0`: Library code that loads a zero value into a0 and then branches to the next instruction.
- `load_address_to_a0_final`: As library code, it loads the address at offset 6e into a0 and branches to itself.
- `load_constant_address`: Library code loads the constant 38 into a0 and then returns immediately.
- `load_address_to_a0_2`: Loads a longword address into register a0 and then branches to itself, creating an infinite loop.
- `load_address_constant`: Library code that loads a longword constant into a0 and then branches to itself.
- `call_indirect_return_d0_2`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0.
- `load_address_and_branch_7`: Loads the address of a constant into a0 and then performs a short branch to the next instruction.
- `load_offset_address_to_a0`: Library code that loads a longword from an offset of 14 into a0 and then branches to the next instruction.
- `load_offset_address_a0`: Loads a long value into a0 and branches to another routine.
- `load_indirect_call_address`: Loads the address of a global variable into a register and branches to another routine that performs an indirect call using an offset from that address.
- `load_address_and_branch`: Loads the address stored in a global variable into register a0 and then performs a short branch to another routine.
- `load_address_and_branch_2`: Loads a longword from a specific memory location into a register and performs a short branch to another routine.
- `dispatch_indirect_routine`: Loads the address of a global variable into a register and branches to another routine that executes a call at an offset from that address.
- `load_indirect_handler_address`: Loads the address of a global variable into a register and branches to another routine that calls a function at an offset from that address.
- `load_address_and_branch_3`: Loads a constant address into a register and performs a short branch to another routine.
- `load_address_and_branch_4`: Loads a longword into a register from a global variable and branches to another routine.
- `load_address_and_branch_5`: Loads a different long value into a0 and branches to another routine.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in d6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_constant_61`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `set_icon_bit_2`: Library code checks a byte at offset 8 of a structure in a6; if it is zero, it calls a function via a pointer at offset 22 and updates a linked list, otherwise it sets bit 3 of a byte at offset 14 and returns zero.
- `set_icon_bit`: The library code sets bit 3 of a byte at offset 14 of a structure pointed to by a6, then returns with d0 set to zero.
- `return_constant_62`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_chain_3`: Library code stores values from a0 and a6 into a structure, calls a function via a pointer at offset -552 of a6, and either calls another routine if the result is zero or updates two global variables if the result is non-zero.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_bitwise_or`: The library code performs a bitwise OR of zero into d0.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `traverse_icon_chain_2`: Saves two registers to the stack and calls another routine to manage an icon chain, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `manage_icon_chain_4`: Saves two registers onto the stack, calls execute_icon_logic, and then returns.
- `manage_icon_chain_3`: Saves the current stack pointer and a pointer from a6 to the stack, calls another routine to manage an icon chain, and then restores the stack pointer before returning.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_search_indexed_values`: This library code calls search_indexed_values_2 and then returns.
- `call_parse_numeric_string`: Saves two registers to the stack, calls parse_numeric_string_2, then restores the registers and returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `populate_icon_structure`: As library code, it calls another routine, and if the result is non-zero, it calls a second routine, then copies 43 bytes from a stack buffer to a structure at the address in a1, and finally updates several fields within that structure.
- `update_linked_list_node_2_2`: Library code copies 43 bytes from a source starting at offset -74 of a6 to a destination starting at offset 96 of a2, copies several 32-bit values from a6 to a2 at various offsets, and updates a pointer at offset 76 of a structure if a value at offset 76 of a2 is non-zero.
- `return_zero_and_unlk_a6`: This library routine clears d0, restores registers from the stack, unlocks a6, and returns zero.
- `copy_icon_data_to_stack`: As library code, it copies 43 bytes from a structure pointed to by a third argument into a local stack buffer, copies several fields from that structure into local stack slots, and calls another routine with the first two arguments and a pointer to a two-word stack header.
- `populate_icon_structure_fields`: Populates various integer and pointer fields within an icon structure by performing multiple library calls to retrieve values based on specific offsets and constants.
- `populate_nested_structure_fields`: Checks a structure for specific constant values and then recursively calls other routines to populate various integer fields at offsets 18, 22, 25, 27, and 35.
- `update_nested_structures`: Library code updates two 32-bit values at offsets 18 and 22 of a structure in a3 using results from two routine calls, then enters a complex conditional block that traverses and updates various pointers and values within a structure in a2.
- `execute_callback_loop_3`: This library code executes a loop that calls two different routines repeatedly, storing a result in a memory location and decrementing a counter, then performs further conditional calls based on a value in a structure.
- `clear_d6_and_branch`: Library code sets register d6 to zero and branches to another location.
- `unlk_a6_and_return`: This library routine restores registers from the stack, unlocks a6, and returns.
- `traverse_icon_hierarchy`: Recursively processes an icon structure by checking flags at offsets 16 and 17, then iterates through child pointers, sub-lists, and nested structures stored at various offsets within the data structure.
- `traverse_icon_structure_hierarchically`: Iterates through a hierarchical structure of icon data by checking flags at offsets 16 and 17 of a structure, then recursively processes child pointers located at offsets 50, 54, and 70.
- `traverse_icon_lists`: Iterates through multiple lists and calls a routine via a pointer, updating a value at offset 26 of a structure.
- `process_icon_list_recursively_2`: Library code iterates through a list pointed to by a4, calls a function via a pointer in a3, and if successful, iterates through a sub-list in a2 while calling the function in a3 and updating a value at offset -8 of a6.
- `process_icon_list_recursively`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a pointer or table.
- `traverse_icon_chain`: Iterates through a linked list by retrieving a pointer from a structure, calling another routine with that pointer, and repeating the process if the returned value is non-zero.
- `restore_registers_and_return_6`: This library routine restores d2 and d3 from the stack and returns.
- `incremental_offset_recurse`: Calls another routine with the same first argument and the second argument increased by a fixed offset of 140, then returns.
- `execute_conditional_library_calls_4`: Library code that calls a function pointer from a global variable, then calls another routine with a specific offset, and performs a series of conditional calls to other routines based on the results, eventually returning a value or zero.
- `call_library_routine_and_return_result`: As library code, it pushes several registers and a pointer onto the stack, calls another routine, and then either returns zero or moves the value from a2 into d0 before returning.
- `return_zero_and_unlk_a6_minimal`: This library routine clears d0, restores d2 through d4 and a2 from the stack, and returns zero.
- `update_indexed_table`: Decrements a 16-bit counter through a pointer and, if not zero, stores two 32-bit values into a table at an offset calculated from that counter.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from registers and the stack into consecutive slots of that table.
- `execute_icon_logic`: As library code, it calls another routine, and if the result is non-zero, it performs a series of conditional calls to other routines using various parameters before returning the result.
- `move_and_branch_3`: Moves the contents of register d6 into d0 and then branches to another location.
- `call_lib_func_and_return_zero`: Invokes a function pointer retrieved from a global variable associated with library index -210.
- `incremental_offset_recurse_2`: Calls another routine with the same first argument and the second argument increased by a constant of 78, then returns.
- `traverse_pointer_list`: As library code, it iterates through a list of pointers starting from a pointer in a0, calling another routine for each element, and returns a pointer to a specific character in a list if a match is found or if a zero pointer is encountered.
- `restore_registers_and_return_7`: Clears register d0 and restores registers d2, d3, a2, and a3 from the stack before returning.
- `search_indexed_values_2`: Library code that performs a loop to compare values retrieved from another routine against a target and calls another routine to update a counter.
- `search_indexed_values`: As library code, it performs a loop that calls another routine to retrieve values, compares these values against a target, and uses a counter in d4 to determine whether to continue the loop or return a status.
- `branch_on_d4_value`: Tests the value in d4; if it is zero, it branches to another routine, otherwise it increments d4, moves it to d2, and branches to another routine.
- `clear_and_branch_2`: Sets register d2 to zero and then branches to another location.
- `restore_registers_and_return_8`: Clears register d0 and restores registers d2, d3, d4, d5, and d6 from the stack before returning.
- `parse_numeric_string_2`: Library code that clears a byte at offset 30 of a structure, calls another routine to process a string, and calculates a numeric value by iterating through bytes and multiplying by 10.
- `increment_and_branch_on_result`: Library code that increments a value in a2, pushes registers onto the stack, calls another routine, and then branches based on the value in d0, otherwise it sets d5 to one and branches to another location.
- `clear_d5_and_branch`: As library code, it clears register d5 and branches to another location.
- `process_numeric_string`: As library code, it processes a sequence of bytes to calculate a value, calls another routine to check a byte against a global variable, and then calls another routine twice before returning.
- `compare_and_set_d0`: Compares a value from the stack with the constant 48; if the value is greater than 48, it branches, otherwise it sets d0 to 57, and if the value is less than 48, it sets d0 to 48, otherwise it sets d0 to the value from the stack minus 48 before branching.
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
- `execute_conditional_library_calls_3`: Library code that calls another routine and uses the result to conditionally call a second routine or perform a cleanup call.
- `call_library_and_update_memory`: This library code calls a library function, and if the result is non-zero, it stores the value 10 at an offset of 14 from the result and at the memory location pointed to by a parameter, then calls another routine.
- `restore_registers_and_return_5`: The library code clears d0, restores d2, d3, a2, and the stack pointer from the stack, and returns.
- `compare_library_result`: Library code calls a library function at offset -42 of a library base and returns zero if the result does not match the fourth parameter, otherwise it returns the result.
- `move_d1_to_d0_and_return`: The library code moves the contents of d1 into d0, restores d2 and d3 from the stack, and returns.
- `verify_library_return`: Library code calls a library function at offset -48 of a library base and returns zero if the result does not match the fourth parameter, otherwise it returns the result.
- `transfer_d1_to_d0_and_restore_registers`: Copies the contents of register d1 into register d0 and restores registers d2 and d3 from the stack before returning.
- `call_library_function`: Calls a library function at an offset of minus 522 from a base address stored in a global variable.
- `rotate_and_threshold_32bit_value`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_7`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit partial components and returns the result.
- `invoke_table_entry_4`: The library code retrieves a function pointer from a global variable and calls the address stored at that specific location.
- `store_byte_and_increment_pointer`: Stores a byte from register d0 into the address pointed to by a3 and then increments a3.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_table_byte`: Library code that loads a pointer and a byte from the stack, compares the byte at the pointer to a value in d0, and if they are equal, decrements the pointer by 1 and returns it in d0; otherwise it returns.
- `clear_d0_and_branch_3`: Library code that clears register d0 and branches to another routine.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `check_byte_and_decrement_pointer`: Restores registers a0 and a1 from the stack, checks a byte at the address in a0, and if it is not zero, decrements a0 and branches back to the start of the routine.
- `compare_byte_and_set_d0`: Compares a byte from a pointer in a1 with a byte from a pointer in a0 after decrementing a value in d0; if the bytes match and the byte from a1 is zero, it sets d0 to zero, otherwise it returns.
- `set_conditional_d0`: Sets register d0 to 1 if it is greater than 0, otherwise it sets d0 to 255 and returns.
- `decrement_d0_and_branch`: Tests a byte at the address in a0; if it is non-zero, it decrements d0 and branches to another routine, otherwise it branches to another routine.
- `decrement_and_call`: Decrements the address in a0 by 1 and calls another routine.
- `copy_bytes_with_counter`: Decrements d0 and if it is not less than zero, copies a byte from the address in a1 to the address in a0 and repeats the loop; otherwise, it clears a byte at the address in a0 and returns.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `update_linked_list_node`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the value at that location by four, clears the following word, and stores the original pointer address into the word after that.
- `conditional_library_branching`: Library code checks if a parameter is non-zero; if so, it calls another routine and, if that returns non-zero, calls two more routines and finally calls a routine using a value from a pointer at offset 10 of the third parameter.
- `traverse_linked_list_and_call`: This library code executes a loop that calls a routine using a pointer from a parameter, and if the result is non-zero, it continues to the next element in a list by reading a pointer from an offset of 12 in the current parameter.
- `set_d0_and_branch_3`: Library code sets register d0 to 1 and branches to another location.
- `clear_d0_and_restore_registers_5`: Sets register d0 to zero and restores registers d2 through d5 and a2 from the stack before returning.
- `allocate_and_initialize_structure`: Calls several routines to manage memory and structure initialization, potentially returning negative one if a condition is not met.
- `return_negative_one_stub`: This library routine returns negative one after restoring registers from the stack.
- `process_nested_data_structures`: Library code that executes a loop to traverse data structures, updating a value at an offset of 12 from a pointer in a2 and performing multiple calls to other routines based on returned values and data read from the structures.
- `process_nested_data_loops`: Executes nested loops that call other routines and update values at offsets of a structure pointed to by a register.
- `move_a3_to_d0_and_branch`: Library code moves the contents of register a3 into register d0 and branches to another location.
- `return_negative_one_extended_stub`: This library routine restores a larger set of registers from the stack and returns negative one.

### translator.library

- `initialize_library_structure`: Calls a function through a pointer at an offset from a global variable, and if the result is not zero, stores the global variable and a register value into a structure before calling another function and returning a specific value.
- `clear_flag_and_increment_counter`: Increments a 16-bit counter in the structure it is given and clears bit 5 of that structure's flag byte.
- `decrement_counter_call_on_zero`: Decrements a 16-bit counter in the structure it is given.
- `check_node_bit`: Saves registers to the stack, loads a pointer from a global variable into a3, loads a new library base into a6, checks bit 4 of a byte at offset 32 of the structure in a3, and if that bit is set, sets bit 5 of a byte at offset 14 of the structure in a3 before branching.
- `unlink_node_and_call_library`: Removes the node from its doubly linked list, then calls a function through the Amiga library base and returns a value stored in the node.
- `return_constant_63`: Returns the constant an address in d0 to the caller without any other effect.
- `transform_text_buffer`: Library code that fills a stack area with spaces and calls another routine to perform character transformations based on specific byte patterns.
- `copy_word_and_branch`: As library code, it moves a 16-bit value from register d0 to d1 and then branches to another location.
- `clear_byte_and_return_word`: Calls one unnamed library routine, clears the byte just before the pointer held in the address register, and returns the low 16-bit value of the data register.
- `call_routines_from_pointer`: Library code that loads a pointer from a structure in a5, compares it to zero, and if not zero, loads a second pointer and calls two different routines before branching based on their results.
- `load_pointers_and_call_routines`: As library code, it loads two pointers from a structure, calls two different routines, and then branches based on the result of a comparison of d7.
- `execute_conditional_calls`: As library code, it pushes two pointers onto the stack, calls three different routines conditionally based on previous results, and then restores the pointers from the stack before returning.
- `compare_byte_to_constants`: Loads two pointers from the stack into a3 and a2, loads a byte from the address in a6 into d7, and compares d7 to the decimal values 92 and 96, branching if either match.
- `stub_immediate_return`: The decompiled body contains only a return, so the routine does no visible work in the library.
- `match_bracketed_tag_text`: Skips forward to an opening bracket character, then compares the characters that follow against a reference string until a closing bracket.
- `check_table_and_compare_byte`: Swaps d6, decrements it, and if it is not negative, loads a byte from a table using d6 as an offset, shifts it, and compares it against a byte from a pointer in a2.
- `set_carry_flag`: Sets the carry flag and returns.
- `call_and_check_zero`: Sets register d7 to 1, calls another routine, and if the result is non-zero, branches; otherwise it clears the carry flag and returns.
- `match_string_and_call_routine`: Compares bytes from a source pointer and a destination pointer in a loop, calling another routine if a specific bit is set in a table lookup, and returns when an equality marker is found.
- `call_and_check_nonzero`: Sets register d7 to 255, calls another routine, and if the result is non-zero, branches; otherwise it clears the carry flag and returns.
- `compare_table_byte_to_constants`: Decrements and increments d7, loads a byte from a table at an offset in d7, and compares it against the constants 92 or 96, otherwise it subtracts d6 from d7 and branches to another routine.

The function list continues in: g1329-recovery-functions-2.md.
