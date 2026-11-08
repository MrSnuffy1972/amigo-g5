# Game 1450 (balance of power - the 1990 edition (1990)(mindscape)[m]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01450). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916908 top-level bytes in 76 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 54 of 54 units identical (bytes and relocations), 59 raw-word lines |
| Functions named and described | pass, 2261 of 2261 named (278 author symbols), 2261 of 2261 described (427 by rule, 1709 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3299 of 3299 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 220825 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 35057 of 35286 instructions identical in start and length, 4 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 48775 |
| code | 118042 |
| data | 731401 |
| padding | 17666 |
| unused | 1024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf:stack | 2 | 117 | 2 |
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
| Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf:CLI | 9 | 362 | 18 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 15 | 1414 | 108 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2476 | 170 |
| mathieeedoubbas.library | 3 | 632 | 60 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 13 | 830 | 53 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf:bop | 3 | 5064 | 305 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1700 | 37 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 150 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf bytes 470528-471040 (code never loaded by a reference run) | 1 | 0 | 0 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf bytes 603136-603648 (code never loaded by a reference run) | 1 | 28 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf bytes 684544-709632 (code never loaded by a reference run) | 1 | 0 | 0 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf bytes 714240-715264 (code never loaded by a reference run) | 1 | 122 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf bytes 867840-868864 (code never loaded by a reference run) | 1 | 0 | 0 |
| SavedGame (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: file devs/printers/Okidata_92 (header 5476 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1478 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 93 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 20 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: bytes 470528-471040 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: bytes 603136-603648 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: bytes 684544-709632 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: bytes 714240-715264 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: bytes 867840-868864 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf:asave/SavedGame: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: 5 logged loads cover 28160 bytes of the image
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: blocks 112-120 (bytes 57344-61952) outside the filesystem hold data, not code (opcode density 0.025, entropy 4.98)
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: blocks 1013-1013 (bytes 518656-519168) hold ASCII text
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: blocks 1298-1298 (bytes 664576-665088) hold ASCII text
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: blocks 1309-1312 (bytes 670208-672256) hold ASCII text
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: blocks 1314-1322 (bytes 672768-677376) hold ASCII text
- balance of power - the 1990 edition (1990)(mindscape)[m]/Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf: blocks 1325-1332 (bytes 678400-682496) hold ASCII text
- 4 never-loaded region(s), 219289 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf:stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of register d1 to register d0 and then performs a short branch to a different location.

### Format

- `parse_command_string`: Parses a string by splitting it into tokens, handling quoted substrings and converting specific character sequences into newline or escape characters, then stores the resulting tokens into a buffer and calls format_and_initialize_disk.
- `execute_callback_chain`: Checks several global variables for non-zero values and executes corresponding function pointers stored at specific offsets.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `execute_callback_chain_3`: Sets a register to zero, calls a routine via a pointer at an offset from a6, stores the result in a global variable, and returns if the result is zero.
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
- `report_disk_error`: Formats a disk by calculating cylinder and sector counts, allocating a memory buffer, and writing a pattern of bytes including a specific signature to the media, then uses a formatted string to report device failures or disk errors.
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
- `pass_params_to_subroutine_2`: Calls another routine using a global variable, a parameter, and the address of an eight-byte stack slot, then returns.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a value from a stack slot or a negative error code.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_global_library_function`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `rotate_32bit_value_3`: If a value in a register is greater than 65535, it performs sixteen iterations of a bitwise rotation on two 16-bit halves of a 32-bit value, adjusting for overflow by subtracting a value and incrementing a part of the result; otherwise, it performs division and modulo operations on the 32-bit value using a second value.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment`: Stores a byte from register d0 into the memory location pointed to by a3 and increments a3.
- `format_disk_and_verify`: Calculates cylinder and sector counts, allocates a memory buffer, and writes a pattern of bytes including a specific signature to the media.
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

- `parse_command_string_2`: Parses a command string by iterating through characters to extract tokens, handling quoted strings and converting specific escape sequences like newline or escape characters.
- `execute_conditional_library_calls`: Stores a parameter into a stack-based location, then calls a function from a global structure if a specific global variable is non-zero, and calls two functions from another global structure if a different global variable is non-zero.
- `invoke_two_function_pointers_2`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `execute_conditional_library_calls_2`: Stores a value from a register into a global variable, calls a function from a global structure, and then branches to another routine or returns.
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
- `update_structure_fields`: Stores a 16-bit value at offset 28, a 32-bit value at offset 36, and a 32-bit value at offset 40 of a structure, calls another routine, and then adds the result of that call to a value at offset 44 of the structure.
- `synchronize_disk_contents`: Performs a disk-to-disk copy operation by iterating through file structures on a source disk and replicating them to a destination disk.
- `synchronize_disk_contents_2`: Performs a disk-to-disk copy operation by iterating through file structures on a source disk and replicating them to a destination disk.
- `execute_cleanup_sequence`: Checks a global variable.
- `cleanup_system_resources`: Checks several global pointers and executes various cleanup routines, including deleting port structures, deleting extended I/O, calling library functions to free memory or close libraries, and managing error message tasks.
- `extract_message_data`: Copies three values from a structure at the address in a global variable into other global variables, calls another routine with that structure, and then calls another routine with a global variable if a specific value is not 512.
- `execute_library_call`: Loads a value from the stack into d0, pushes a value from a global variable and d0 onto the stack, calls a routine at an offset from a global variable, and if the result is not zero, it branches.
- `perform_disk_copy_loop`: Iterates through a table of data blocks to perform disk I/O operations, including reading from a source and writing to a destination.
- `allocate_multiple_libraries`: Checks a value at an offset of 20 from a global variable; if it is less than 33, it calls a routine with a fixed address and then calls a routine with 0.
- `initialize_disk_copy_params`: Populates a stack-based structure with several constants and then calls two library functions using a global base.
- `report_disk_errors_2`: Sets a pointer to a global variable and calls another routine with a global value, a parameter, and a stack address, and then repeats this with different parameters based on the value of the first parameter.
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
- `allocate_message_entry_2`: Attempts to find a specific task and allocate a message buffer; if successful, it populates the buffer with a value, a byte, a constant, and a task pointer before dispatching the message via a signal.
- `delete_port_structure`: Checks if a field at offset 10 of a structure is non-zero; if so, it recursively calls itself.
- `create_ext_io_request`: If the first argument is non-zero, it calls a routine to allocate memory for an I/O request structure with a constant value of 65537, then populates the structure at offset 8 with 5, offset 14 with the first argument, and offset 18 with the second argument as a 16-bit word; otherwise, it returns zero.
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
- `copy_memory_block`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `execute_indirect_calls`: Fetches a function pointer from a global structure and executes it multiple times with varying parameters.
- `invoke_indirect_callbacks`: Calls another routine with a pointer to a global variable and a parameter, then calls another routine with four parameters including a local stack array.
- `convert_char_to_uppercase`: Returns the character with lowercase ASCII letters converted to uppercase by subtracting 32.
- `compare_strings_case_insensitive_2`: Compares two null-terminated strings by converting characters to uppercase and returns 1 if they match, otherwise it returns 0.
- `terminate_colon_string`: Searches for the end of a null-terminated string and, if the last non-null character is a colon, replaces it with a null terminator.
- `invoke_global_and_checksum_stack`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_and_return`: Stores a byte from register d0 through a pointer in a3 and then returns.
- `modular_inverse_32bit`: Performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag_2`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag_2`: Calls modular_inverse_32bit and returns its first result, or the negation of that result if the second result in an address register is nonzero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment_pointer`: Stores a byte from register d0 into the memory location pointed to by register a3, then increments the pointer in a3 by one before returning.
- `call_formatted_print`: Pushes a pointer and a value onto the stack and calls a routine to print a formatted message, then returns the result in a register.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_global_library_function_via_offset`: Calls a library function through a library base read from a global variable at an offset of 348, passing no arguments and returning.
- `execute_with_status_check_2`: Calls a routine via a global pointer and returns either a byte from a stack slot or a negative error code.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.

### Port-Handler

- `parse_amiga_hunk_data`: Parses a structure containing AmigaDOS hunk type codes to determine relocation or executable loading requirements.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a table, then enters a loop to clear a range of memory and performs bitwise operations to manipulate data within a structure.

### Ram-Handler

- `execute_jump_table`: Sets bit 2 and 3 of d0, calls a routine using a pointer from a6 at an offset of negative 656, and then performs an indirect jump using a6.

### serial.device

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reinit_serial_buffer_fields`: Frees a block named by the second argument when it is set, then allocates a buffer of a specified size, clears unit fields, and stores half-size and quarter-size values for buffer positions.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `check_flag_and_call_handler_2`: Stores the byte 253 at offset 31 of a structure in a1 and then branches to another location, potentially calling a handler if a bitmask test on offset 30 passes.
- `toggle_interrupt_enable`: Writes 2049 to the INTENA register, calls another routine, and then writes 22401 to the INTENA register.
- `clear_request_byte`: Clears bit 7 of a byte at offset 30 of a structure in a1 and branches to different locations based on whether the word at offset 28 of the same structure is 2, 3, or 10.
- `compare_and_clear_pointer`: Compares a long value at offset 74 of a structure in a6 with a1; if they are equal, it clears bit 1 of a byte at offset 474 of a structure in a6 and then clears the long value at offset 74 of a structure in a6.
- `unlink_request_node`: Checks bit 6 of an offset 30 value in a1; if set, it unlinks a node by updating pointers between a0 and a1, decrements a 16-bit counter at offset 70 of a6, and branches to another routine.
- `reset_serial_hardware`: Compares a1 with an offset 78 value in a6 and checks if an offset 28 value in a1 is 10; if so, it clears bit 5 of an offset 470 value in a6, writes values to hardware registers at offset 78 of the hardware base, calls another routine, clears the offset 78 value in a6, clears bit 4 of an offset 472 value in a6, and branches to another routine.
- `unlink_second_queue_request`: When the request's flag bit is set, unlinks it from the second queue and decrements that queue's count, then marks the request done and replies to its message when it is not busy.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `set_status_flag`: Sets register d0 to zero, checks a 16-bit value at offset 32 of a6, and if non-zero, sets bit 6 of an offset 472 value in a6 before returning.
- `invoke_library_vectors_then_unlink`: Calls several library vectors, copies a value from a structure, and unlinks a block from a list by updating a pointer and its contents.
- `manage_linked_list_and_handler`: Clears a 32-bit value at offset 8 of a structure in A1, then checks several conditions in a structure in A6 to either clear bit 1 of offset 30 or set bit 6 of offset 30 in A1.
- `process_serial_data_buffer_2`: Iterates through a linked list of data buffers, decrementing a counter and updating a pointer to the next element in the list, then calls process_serial_data_stream.
- `process_serial_data_stream`: Iterates through a linked list of data buffers, decrementing a counter and updating a pointer to the next element in the list.
- `test_status_bit`: Tests bit 0 of a byte at offset 443 of a structure in a6 and branches to one location if the bit is clear, otherwise it branches to another location.
- `set_status_byte_and_branch`: Calls another routine, and if the result is zero, it writes the value 9 to offset 31 of a structure in a1 before branching to another routine.
- `process_serial_data_buffer`: Iterates through a linked list of data buffers, performing bitwise manipulations on status bytes and updating counters.
- `restore_registers_and_return_8`: Returns immediately after restoring D2 and A6 from the stack.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `clear_status_bit_and_branch`: Clears bit 2 of a byte at offset 470 of a structure in a6, and if the result is zero, writes 15 to offset 31 of a structure in a1 and branches to another routine; otherwise it branches to another routine.
- `clear_status_bit_3`: Clears bit 7 of a byte at offset 470 of a structure in a6 and, if the bit was already clear, writes 9 to offset 31 of a structure in a1 before branching to another location.
- `update_status_and_clear_field`: Writes the value 12 to offset 31 of a structure in a1, tests bit 0 of a byte at offset 470 of a structure in a6, and if that bit is zero, writes the value 6 to offset 31 of a structure in a1, clears a long value at offset 62 of a structure in a6, and masks bit 4 through 7 of a byte at offset 470 of a structure in a6 before branching to another routine.
- `process_serial_data_buffer_3`: Manages a circular buffer by checking status bits in a structure, updating data pointers, and copying bytes into a buffer if space is available.
- `toggle_serial_interrupt`: Compares a byte at offset 34 of the structure in a6 with d1 and, if equal, tests bit 1 of a byte at offset 472 of the structure in a6; if that bit is set, it clears it and otherwise enables interrupts by writing to the intena register, then returns.
- `check_serial_status`: Tests bit 5 of a byte at offset 296 of a structure in a6, and if it is not set, branches to another routine; otherwise it tests bit 0 of a byte at offset 445 of a structure in a6, and if it is set, branches to another routine; if not, it calls another routine, and if the result is zero, tests bit 7 of d0, and if it is not set, branches to another routine, otherwise it branches to another routine.
- `test_bit_and_branch_3`: Tests bit 7 of a byte in d0, and if it is set, branches to another routine; otherwise it branches to another routine.
- `test_bit_and_branch_2`: Tests bit 0 of a byte at offset 413 of a structure in A6 and branches to another routine.
- `set_flag_and_copy_pointer`: Sets bit 7 of a byte at offset 470 of a structure in a6 and, if the longword at offset 62 of the same structure is zero, copies the longword from offset 42 to offset 62 before branching.
- `process_buffer_data_2`: Compares a longword at offset 54 of a structure in a6 with a value at offset 58, and if the latter is smaller, performs several increments and pointer updates, otherwise it updates a longword at offset 62 with the value from offset 42 if the value at offset 62 is zero, then calls a routine through a pointer located at an offset from a value in a6.
- `record_pointer_then_signal`: If a long value at offset 62 of a structure in a6 is zero, it copies a long value from offset 42 of a structure in a6 to offset 62 of a structure in a6; it then calls another routine and calls a library vector using a pointer from offset 98 of a structure in a6.
- `clear_hardware_bit`: Copies a byte from offset 34 of a structure in a6 to offset 473 of a structure in a6, clears bit 3 of a byte at offset 474 of a structure in a6, tests bit 2 of a byte at offset 69 of a structure in a6, and if that bit is zero, clears bit 6 of the hardware register at address an address before branching to another routine.
- `update_interrupt_settings`: Moves a byte from offset 35 of a structure in A6 to offset 473 of a structure in A6, sets bit 2 of a byte at offset 472 of a structure in A6, tests bit 2 of a byte at offset 69 of a structure in A6, and if bit 2 is clear, sets bit 6 of a long value at offset 496 of a structure in A6; otherwise, if bit 7 of the byte at offset 69 is set, it writes the value 32769 to a long value at offset 410 of a structure in A6.
- `clear_byte_at_offset`: Clears a single byte at offset 473 of a structure in A6 and returns.
- `update_structure_and_signal`: If bit 6 of a byte at offset 470 of a structure in a6 is not set, it sets that bit, clears a long value at offset 418, copies a long value from offset 66 to offset 418 while subtracting 10, sets a 16-bit value at offset 414 to 9, calls another routine, and if a byte at offset 31 of a structure in a1 is non-zero, sets a byte at offset 31 of a structure in a1 to 11.
- `update_status_from_interrupt_register`: Saves registers d2, a3, and a6 to the stack, loads a pointer from a1 into a6, clears bit 6 of a byte at offset 470 of the structure in a6, and if the bit was set, checks the interrupt status register in d0; if the masked value of d0 is not zero, it branches to another routine, otherwise it sets bit 2 of a byte at offset 470 of the structure in a6 and branches to another routine.
- `load_address_and_call`: Loads the address of a structure at offset 418 of a structure in A6 into A0, calls another routine, and then branches to another routine.
- `set_bit_and_branch_4`: Sets bit 0 of a byte at offset 470 of a structure in a6 and branches to another routine.
- `manage_serial_buffer_queue`: Clears a value at offset 32 of a structure in a1 and, if offset 36 is non-zero, clears bit 1 of offset 30.
- `update_interrupt_and_request_state`: Tests bit 2 of offset 69 of a structure in A6; if set, it calls another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_request_state_on_flags`: Checks if two specific bits in a global variable are set, and if so, writes a byte with a value of thirteen to an offset within a structure pointed to by a register.
- `process_serial_request_queue`: Manages a linked list of serial request nodes by updating pointers and status bits, decrements a counter, and calls another routine to handle pending requests.
- `update_serial_status_and_counters_2_2`: Writes a constant to a global variable, loads a pointer from a global variable, and moves a byte from that pointer into a register.
- `update_serial_status_and_counters_2`: Updates a bitmask in a global variable based on a status byte, increments two counters within a structure, and performs a loop to compare values in a buffer against the bitmask before calling another routine via a function pointer.
- `update_serial_status_and_counters`: Updates a bitmask in a global variable based on a status byte, increments two counters within a structure, and performs a loop to compare values in a buffer before calling another routine.
- `manage_linked_list_node`: If a 16-bit value at offset 72 of a structure in a6 is not zero, it manipulates a linked list by updating pointers and setting bit 4 of a byte at offset 30 of a structure pointed to by a long value from offset 78 of a structure in a6, then decrements the 16-bit value at offset 72 of a structure in a6 and calls another routine if a byte at offset 7 of that same structure is 10.
- `set_interrupt_enable_bit`: Restores D2 and A6 from the stack, writes 1 to the INTENA hardware register, and returns.
- `update_serial_status`: Enables serial interrupts, reads two bytes from a structure in a1 to combine them into a 16-bit value, writes that value to the custom chip data port, clears a byte in the structure, and returns based on two conditional checks.
- `reset_and_configure_serial_device`: Updates several fields in a structure via a pointer, enables interrupts by writing to the INTENA register, and calls a handler routine if a specific bitmask test passes.
- `call_handler_and_branch`: Calls another routine and then branches to another location.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `set_flag_and_call_handler`: If a long value at offset 78 of a structure in a6 is not zero, it sets bit 2 of a byte at offset 474 of a structure in a6; if a long value at offset 74 of a structure in a6 is not zero, it calls another routine and then calls a library vector if bit 0 or bit 6 of a byte at offset 30 of a structure in a1 is not set.
- `clear_interrupt_and_call_handler`: Clears bit 2 of a byte at offset 472 of a structure in a6, and if a long value at offset 74 of a structure in a6 is not zero, calls another routine; it then sets a 16-bit global variable to 32769 and, if a byte at offset 30 of the returned pointer has bits 0 and 6 cleared, calls another routine.
- `toggle_interrupt_and_call`: Writes 2049 to the INTENA hardware register, calls another routine, writes 34817 to the INTENA hardware register, and then branches to another location.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `update_pointer_and_call_handler`: Copies a 32-bit value from a structure in A1 to A0, and if it is non-zero, writes it to the structure in A0, swaps A1 and D0, writes A0 to offset 4 of A1, and if D0 is non-zero, calls another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_status_and_transfer_data`: Calculates a byte based on a bitmask and two flags in a structure in A6, then writes it to offset 81 of a structure in A1.
- `test_buffer_and_pointer_status`: Clears bit 0 of a byte at offset 30 of a structure in a1, tests bit 3 of a byte at offset 79 of a structure in a1, and if that bit is set, checks if a 16-bit value at offset 72 of a structure in a6 is non-zero or if a long value at offset 78 of a structure in a6 is non-zero before branching to another routine.
- `process_request_and_interrupt`: Moves a long value from offset 106 of a structure in A6 to D0, calls another routine, tests a byte at offset 31 of a structure in A1, and if it is non-zero, it stores the pointer in A1 into offset 78 of a structure in A6, sets bit 5 of a byte at offset 470 of a structure in A6, and writes 35328 to a long value at offset 414 of a structure in A6.
- `reply_and_clear_active_request`: Pushes A6 to the stack, loads a new address into A6 from offset 98 of the old A6, calls a routine via a pointer, restores A6 from the stack, clears a 32-bit value at offset 78 of A6, and returns.
- `clear_uart_break_and_reply`: Clears bit 5 of a byte at offset 1d6 of a structure in a1, and if bit 5 of that byte is set, a6 is non-zero, and a value at an offset of a pointer in a1 is 10, it sets bit 1 of that byte, clears a pointer in a1, sets a global variable to 2048, and calls a routine via a pointer in a1.
- `start_timed_read_request`: Sets a global variable to one, clears a 32-bit value at offset 142 of a structure in a6, stores a 16-bit value from d0 at offset 146, and stores the value 9 at offset 13e of the structure in a6.
- `toggle_serial_status_bit`: Checks bit 7 of a byte at offset 79 of a structure in a1; if set, it sets bit 7 of a byte at offset 69 of a structure in a6, clears bit 2 of a byte at offset 472 of a structure in a6, and if bit 3 of that same byte is set, it clears bit 7 of the byte in a1 and calls another routine before setting bit 7 of the byte in a1 again; otherwise, it clears bit 7 of the byte in a1 and returns.
- `test_serial_conditions`: Clears bit 7 of a byte at offset 69 of a structure in a6, then checks bits 1 and 4 of bytes at offsets 78 and 30 of structures in a6 and a1 respectively, and if those conditions are met, sets bit 0 of a byte at offset 31 of a structure in a1.
- `toggle_interrupt_and_sync`: Sets the interrupt enable bit and bit 0 of the INTENA register to 3297, calls another routine, sets the INTENA register to 34817, and if a byte at offset 30 of the returned pointer has bits 0 and 6 cleared, calls another routine.
- `validate_and_sync_serial_parameters`: Validates a sequence of three bytes and a long value within a structure, then copies several fields to a secondary structure and performs bitwise operations on a status byte.
- `compute_unit_channel_masks`: Calculates a 16-bit mask by shifting 1 based on a byte at offset 4d of a structure in a1, potentially shifting it further based on a byte at offset 4e of a1 and a bit in a byte at offset 45 of a structure in a6.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_byte_and_branch`: Sets a byte at offset 31 of a structure in A1 to 5 and branches to another routine.
- `calculate_serial_period`: Loads a long value from offset 450 of a structure in A6 into D0 and D1, shifts D0 left by 3, subtracts D1 from D0, compares D0 to 65535, and if D0 is less than or equal to 65535, it shifts D0 right by 5, performs an unsigned division of D0 by D1, masks the result to 16 bits, and shifts the result right by 5.
- `store_serial_period_quotient`: Divides a 32-bit value in d1 by a 16-bit value in d0, stores the quotient in a structure in a6 at offset 1ce, and clears bit 5 of a byte at offset 1d8 of the structure in a6.
- `initialize_serial_device_2`: Calls a subroutine using a pointer from a structure in a6, and if the result is non-zero, performs multiple writes to the returned structure, calls several subroutines, and updates several fields in the structure before setting a global variable to 2049.
- `initialize_serial_parameters`: This routine performs two branch-to-subroutine calls using addresses derived from a6 and then branches to another location.
- `copy_pointer_to_offset`: Stores the value in a0 into a structure at offset 10 of a1.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `initialize_serial_device`: Writes values to various offsets of a structure in a6, sets a global variable to 3265, and calls a function through a pointer located at offset 98 of the structure in a6.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_and_branch_2`: Clears the d0 register and branches to a previous instruction.

### parallel.device

- `update_parallel_device_status_2`: Checks status bits in two structures, increments a 16-bit value at offset 32 of one structure if certain conditions are met, copies a byte to offset 35 of the other, and conditionally copies 8 bytes of data from offsets 90 and 94 of the first structure to offsets 54 and 58 of the second.
- `increment_counter_and_update_status`: Increments a 16-bit value at offset 32 of a structure in a6, sets bit 4 of a byte at offset 117 of that structure, and copies data from a6 to a1 if bit 1 of a byte at offset 53 of the structure in a1 is set.
- `update_status_and_copy_data`: Clears a byte at offset 76 of a structure in a6, sets bit 5 of a byte at offset 75 if bit 5 of a byte at offset 35 of a structure in a1 is set, and increments a 16-bit counter at offset 20 of a6 if two bytes in a1 and a6 are zero.
- `process_parallel_device_status`: Checks a status field in a structure and updates a global variable and a status byte in a6.
- `check_and_call_handler`: If a 16-bit value at offset 1c of a structure in a1 is 3, it clears bit 0 of a global variable and, if bit 0 is still zero and bit 2 of a byte at offset 76 in a6 is not set, calls a routine via a pointer in a6.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `write_status_byte_4`: Writes the byte value 253 to offset 31 of a structure in a1 and branches to another routine.
- `check_and_clear_long`: Tests a longword at offset 36 of a structure in a1; if it is non-zero, it branches to another routine, otherwise it clears a longword at offset 32 of the structure in a1 and branches to another routine.
- `update_device_status_2`: Clears bit 1 of a byte at offset 30 in a1, then either inserts a1 into a linked list at offset 54 of a structure in a6 and increments a 16-bit counter at offset 72 of that structure, or updates a global variable and modifies bytes at offsets 74 and 76 of the structure in a6 based on a comparison and a value at offset 7 of a1.
- `update_device_status`: Sets bit 4 of a byte at offset 1e in a1, copies a 32-bit value from a1 to a6, clears a 32-bit value at offset 2e in a6, and stores a pointer from a1 into a6 at offset 26.
- `process_parallel_device_interrupt`: Manages a linked list of device structures to clear status bits and decrement a counter, then performs a comparison loop against a buffer of bytes to check for values exceeding a threshold.
- `update_parallel_device_counters`: Increments two counters in a structure, performs a comparison loop against a byte in a buffer, and updates status bits in a structure and the CIA hardware register before calling two functions via pointers.
- `update_parallel_device_status`: Copies a 32-bit value from one structure to another, modifies specific bits in two status bytes, calls a function via a pointer, and conditionally clears a bit and calls another function if a 16-bit value is zero.
- `test_cia_and_status`: Moves a byte from offset 0 of a structure in a0 to offset 1,048,577 of the CIA hardware register; if the value is not zero, it branches to another routine, otherwise it tests bit 3 of a byte at offset 118 of a structure in a6; if bit 3 is set, it branches to another routine, otherwise it branches to another routine.
- `call_status_update_routine`: Calls another routine, sets a pointer in a6 to the address at offset 50 of a structure in a6, moves the address at offset 119 of a global variable into d0, pushes a6 onto the stack, sets a6 to the address at offset 50 of the structure in a6, calls a routine via a pointer, restores a6 from the stack, and branches to another routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_if_not_zero`: Checks if a value in a structure is non-zero and calls a function through a pointer if it is.
- `invoke_callback_pointer`: Executes a function through a pointer stored in a structure if the associated index is not zero.
- `process_pointer_list`: Loads a pointer from offset 76 of a structure in a6 into a1, moves the value at a1 into d0, and if d0 is not zero, stores d0 back at offset 76 of the structure in a6, exchanges d0 with a1, stores a1 at offset 4 of a1, and if d0 is not zero, calls another routine with a1 as an argument.
- `clear_offset_72`: Clears a 16-bit value at offset 72 of a structure in a6 and returns.
- `modify_status_byte`: Sets bit 5 of a byte at offset 30 of a structure in a1, sets a byte at offset 31 of the structure to 254, and calls a routine via a pointer stored in a structure in a6.
- `update_global_and_field`: Manipulates bits within a global variable and updates a byte in a structure using a library function call.
- `set_request_byte`: Tests bit 1 of a byte at offset 118 of a structure in a6; if it is clear, it writes the byte 1 to offset 31 of a structure in a1 and branches to another routine.
- `copy_structure_fields`: Copies a byte from one structure to another and, if a specific bit is set, copies two long words as well.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `recurse_on_pointer`: Calls another routine via a pointer stored on the stack and then returns.
- `recursively_calls_itself`: Performs an infinite recursive call to itself.
- `call_cleanup_routine`: Pushes the contents of a1 onto the stack, calls another routine, pops the value from the stack, and returns.
- `jump_to_handler`: Performs a long jump to the address at offset 4038 of a global variable.
- `decrement_and_check_counter`: Pushes a1 onto the stack, calls a routine, and then increments the stack pointer by 4.
- `call_and_restore_stack`: Pushes the contents of a1 onto the stack, calls another routine, and adds 4 to the stack pointer.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `update_clipboard_handler_pointer`: Loads a pointer from the stack into a global variable, calls another routine, and stores the result in a second global variable.
- `execute_conditional_logic`: Stores a value from d3 into a global variable, then calls three different routines, one of which returns a value used to decide whether to call two other routines or a single routine.
- `call_two_functions`: Calls two different functions via library vectors and returns 1.
- `update_clipboard_data`: Calls a routine, then calls another routine with two arguments; if the result is non-zero, it either clears a 16-bit value at offset 94 of a structure in a1 or increments a 32-bit value at offset 14 of a structure pointed to by a pointer at offset 80 of a1, then calls a third routine with a structure in a1 and the result.
- `clear_and_call_library`: Clears a long word at offset 14 of a structure in a3, then calls a library function using a pointer at offset 72 of a structure in a2.
- `zero_word_field`: Stores a zero word into a specific offset within a structure and then calls another routine.
- `process_clipboard_data_pointer`: Calls a routine using a pointer from the stack, then checks a value at offset 72 of the returned pointer; if non-zero, it calls another routine with a pointer to that value and returns the original value.
- `update_pointer_chain`: Pushes a2 onto the stack, calls a routine with a pointer at offset 72 of a2, clears a 32-bit value at offset 86 of a2, calls another routine, and then restores d2 and a2 from the stack.
- `copy_linked_list_buffer_segment`: Traverses a linked list of 512-byte buffers to copy a specific range of bytes from the list into a destination buffer.
- `compare_buffer_values`: Loads a long word from an offset of 90 in a2, compares it to a long word at offset 48 of the stack, and performs conditional branches based on the comparison results.
- `copy_buffer_segment`: Traverses a linked list of structures to find a specific node based on an index, then copies a sequence of bytes from that node's data offset into a destination buffer using a loop.
- `compare_and_update_pointer`: Copies a long value from d1 to d0, compares d0 with a long value at offset -6 of a1, and if d0 is not lower than or equal to that value, updates a1 with the value at offset 0 of a1 and branches to another routine.
- `compare_and_branch_limit`: Copies a long value from d3 to d5, and if d5 is not zero, it calculates a value in d0 by subtracting d2 from 512, compares d5 to d0, and if d5 is less than or equal to d0, it moves the 16-bit value in d2 to d4 and branches to another routine.
- `copy_data_loop`: Copies bytes from a table starting at offset 18 of a1 to a destination in a3 using an index in d4, increments d4, and loops if d4 is less than 512; otherwise, it performs arithmetic on d0, d2, and d5 and branches to another routine.
- `transfer_control_flow`: Moves a 16-bit value from d2 to d4 and branches to another routine.
- `copy_data_with_limit`: Copies bytes from a table starting at offset 18 of a1 to a destination in a3 using an index in d4, increments d4, and loops if the sign-extended 32-bit version of d4 is less than the sum of d5 and d2; otherwise, it clears d5 and branches to another routine.
- `store_long_word`: Stores a long word into a specific offset within a structure and then calls another routine.
- `validate_and_update_clipboard_buffer`: Validates the size and integrity of a clipboard buffer by checking a linked list of memory blocks and then copies bytes from a source pointer into the appropriate blocks, managing block boundaries and offsets.
- `copy_clipboard_buffer_data`: Iterates through a linked list of memory blocks to copy bytes from a source buffer into the blocks, managing remaining byte counts and block offsets until the source data is exhausted or the destination is full.
- `call_and_adjust_stack`: Pushes a pointer from an offset of 72 in a6 onto the stack, calls another routine, increments the stack pointer by 4, and performs an unconditional short branch.
- `update_clipboard_buffer_list`: Traverses a linked list of nodes to find a specific index, then copies bytes from a source buffer into the nodes of the list until the remaining byte count reaches zero.
- `traverse_and_fill_linked_list`: Traverses a linked list to find a node matching a specific index, then copies bytes from a source buffer into the nodes of the list until the buffer is exhausted.
- `copy_buffer_data_3`: While a pointer in d4 is not zero and the long value at offset 14 of the structure in a6 is not zero, it copies bytes from a source pointed to by a3 into a destination buffer starting at offset 18 of the pointer in d4, updating the destination pointer and the remaining count in the structure in a6; finally, it stores the destination pointer into offset 86 of a2 and calls another routine.
- `check_buffer_bounds`: Tests d4, and if not zero, tests a long value at offset 20 of a structure in a6; if that value is not less than or equal to zero, it calculates a value in d0 by subtracting d3 from 512, compares that value to the long value at offset 20 of a structure in a6, and if the comparison result is not carry, it moves the 16-bit value in d3 to d1 and branches to another routine.
- `copy_buffer_data_2`: Copies bytes from a source pointer to a destination at offset 18 of a structure in a1, looping until a counter reaches 512, then calls another routine and calls a second routine that may clear a 16-bit value at offset 94 of a structure.
- `transfer_control_flow_2`: Moves a 16-bit value from d3 to d1 and branches to another routine.
- `copy_buffer_data`: Copies bytes from a source pointer to a destination buffer at an offset of 18, potentially looping to fill up to 512 bytes while updating a counter in a structure.
- `store_register_to_offset`: Stores the contents of a data register into a memory location calculated by adding a fixed offset to a pointer held in an address register.
- `check_bits_and_measure`: Checks bit 7 and bit 0 of a byte at offset 30 of a structure in a2, and if neither is set, calls a routine to measure a string length.
- `modify_byte_bits`: Clears the least significant bit and sets the most significant bit of a byte at a specific offset in a buffer.
- `modify_byte_bits_2`: Checks bit 7 of a byte at offset 30 of a structure, clears bit 1 if two pointers are not equal, and clears bit 2 of the same byte.
- `compare_structure_values`: Loads two pointers from a structure, compares a longword at offset 48 of the first pointer with a longword at offset 48 of the second pointer, and sets d0 to 1 if the first is smaller.
- `set_zero_and_branch`: Sets d0 to 0 and branches to another location.
- `set_d0_to_one_and_branch`: Sets register d0 to one and performs a short branch to another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_structure_fields_2`: Copies several long and word values from one structure to another before calling another routine.
- `update_clipboard_status_flags`: Sets bit 5 of a byte at offset 30 of a structure pointed to by a value at offset 12 of a structure in a6, then checks a byte at offset 66 of the first structure; if that byte is not zero, it clears bit 5 of the byte at offset 30 and otherwise returns.
- `update_buffer_management_state_machine`: Executes a loop that processes a buffer management state machine by checking a status field in a structure and performing data transfers, counter increments, or pointer updates based on the current state.
- `clear_and_branch_9`: Clears a 16-bit word in register d3 and branches to another location.
- `process_clipboard_buffer_state_2`: Updates a buffer management state machine by reading a pointer from a structure, checking its type, and performing data transfers or counter increments based on the type.
- `call_and_set_status`: Pushes register a2 onto the stack, calls another routine, pops the stack, sets a 16-bit word at offset 94 of a structure in a2 to 1, and branches to another location.
- `update_buffer_state_machine`: Processes a buffer management state machine by checking a status field in a structure and performing operations based on its value.
- `clear_and_branch_8`: Clears the 16-bit register d3 and branches to another location.
- `update_clipboard_buffer_states`: Iterates through a linked list of buffer structures to manage their status and size.
- `manage_clipboard_buffer_states`: Checks a status field in a structure and performs different operations based on its value: if the status is 2, it updates a linked list of buffers by adjusting size and pointer values; if the status is 3, it updates a size counter and pointer through a linked list; if the status is 4, it resets specific fields and calls another routine.
- `compare_buffer_offsets`: Loads a longword from offset 90 of a structure in a2 into d2, adds a longword from offset 28 and a longword from offset 36 of a structure in a4 to d0, compares d0 to d2, and if d0 is not lower than d2, stores the longword from offset 36 of a structure in a4 into offset 32 of the same structure before branching to another routine.
- `update_buffer_offset`: Loads a longword from offset 90 of a structure in a2 into d2, subtracts a longword from offset 44 of a structure in a4 from d2, stores the result in offset 32 of the structure in a4, and adds the new value in d2 to offset 44 of the structure in a4 before branching to another routine.
- `clear_and_call_buffer_handler`: Clears a longword at offset 32 of a structure in a4, sets a byte at offset 48 of the same structure to 255, decrements a word at offset 110 of a structure in a2, and calls a routine using a0 and a4 as arguments before looping if the word at offset 110 is non-zero.
- `process_clipboard_buffer_state`: Checks a status field in a structure and, if it equals three, traverses a linked list to update a size counter and a pointer.
- `update_clipboard_pointer`: This routine copies a longword from an offset in a4 to an offset in a2, pushes two pointers to the stack, calls another routine, and then branches back to an earlier instruction.
- `check_status_and_write_2`: Compares the word at offset 28 of the structure in a4 to 4, and if they are equal, writes the word 4 to offset 64 of the structure in a2 and branches to another routine.
- `clear_and_branch_7`: Clears the word in d3 and branches to an earlier instruction.
- `update_clipboard_buffer_status`: Checks a status field in a structure; if it equals four, it retrieves a pointer, increments a counter by eight if a subsequent routine returns four, or resets the counter to zero otherwise, then clears a 16-bit value and calls another routine with the retrieved pointer.
- `clear_clipboard_pointer`: Clears a long value at offset 90 of a structure in a2 and then branches to another routine.
- `process_clipboard_data_structure`: Iterates through a data structure to update status flags and pointer values based on comparisons between multiple memory locations.
- `traverse_linked_list_2`: Traverses a linked list by following pointers until a null pointer is reached or a value at an offset of 12 of the current structure is less than a value in a parameter, then calls another routine with the address of the last non-null structure and a value from its offset 1.
- `clear_d1_and_branch`: Sets register d1 to zero and branches to another location.
- `call_library_function_3`: Loads a library base from a global variable and calls a library function at a specific offset.
- `set_byte_value`: Writes a constant byte value to an offset within a structure.
- `set_byte_constant_offset`: Writes a constant byte value to a specific offset within a structure and then calls another routine.
- `set_byte_constant_offset_4`: Pushes a value from a stack-based parameter onto the stack, calls another routine, and then restores the value from the stack before returning.
- `set_byte_constant_offset_2`: Stores a constant byte value at a specific offset within a structure and then calls another routine.
- `set_byte_constant_offset_3`: Stores a constant byte value into a structure at a specified offset and then calls another routine.
- `update_structure_byte_sequence`: Updates three specific byte offsets within a memory structure by calling three separate routines that each store a constant byte value at a specified offset.
- `synchronize_buffer_data`: Copies a 32-bit value from a source structure to a destination structure and sets bit 4 of a byte at offset 30.
- `call_subroutine_with_offset`: Calls another routine using a value from a register plus an offset of 96 and then calls a second routine.
- `increment_structure_value`: If a 32-bit value at offset 48 of a structure in a2 is zero, it increments a 32-bit value at offset 56 of a second structure pointed to by offset 24 of the first structure and stores the result in offset 48, then either calls another routine recursively or calls two other routines based on a comparison between the value at offset 48 and a value at offset 60 of the second structure.
- `call_subroutine_with_different_offset`: Calls another routine using a value from a register plus an offset of 112 and then calls a second routine.
- `copy_structure_field`: Copies a longword from an offset relative to a pointer to another offset in a structure.
- `copy_structure_field_offset`: Copies a longword from a different offset relative to a pointer to a field in a structure.
- `search_and_allocate_clipboard_node`: Searches a linked list for a node matching a specific identifier; if found, it increments a counter and updates a pointer, otherwise it calls another routine to allocate a new node, initializes its fields, and performs a series of memory management and validation checks including a loop that processes data in blocks of 512 bytes.
- `compare_and_zero_registers`: Compares a longword at offset 14 of a structure in a3 with d3; if they are not equal, it clears d2 and d4 and branches to another location.
- `allocate_and_transfer_clipboard_buffer`: Allocates a memory block and initializes its fields, including a pointer to a global variable and several status flags.
- `synchronize_clipboard_data_structure`: Initializes a data structure by setting several fields to one and calling a function pointer, then performs a series of library calls to validate, update, and loop through data chunks.
- `execute_library_call_and_adjust`: Calls a library function with a pointer to offset 18 of a0, and if the return value in d0 is 256, clears a word at offset 94 of a3 and subtracts 256 from a value at offset -4 of a6.
- `validate_and_clear_clipboard`: Tests a word at offset 94 of a3, and if it is not zero, tests a long value at offset -4 of a6; if that value is not less than or equal to zero, it pushes a3 to the stack, calls another routine, compares the result in d0 to zero, pops the stack, and if d0 is not zero, clears the word at offset 94 of a3 and loops.
- `process_clipboard_data_structure_2`: Validates a structure by comparing a value against a constant, clears specific fields at offsets 43, 90, and 94, and enters a loop that calls another routine using a pointer and a bitmask derived from a byte at offset 9.
- `process_complex_structure`: Increments values in a structure, performs conditional library calls based on a flag, and returns a value from a structure offset.
- `decrement_and_cleanup_clipboard_structure`: Decrements a 16-bit counter at offset 18 of a structure and, if it reaches zero, performs a recursive cleanup of linked list elements by traversing a chain of pointers, clearing specific fields, and decrementing a global byte.
- `execute_library_call_and_clear`: Calls a library function with a pointer to offset 18 of a4, and if d0 does not match d4, clears a word at offset 94 of a3 and sets d4 to zero.
- `clear_word_and_branch`: Clears a word at offset 94 of a3 and branches to another location.
- `cleanup_and_call_library`: Calls three routines, then if a value at offset 68 of a3 is non-zero, calls another routine with a value from offset 68 of a3; if a word at offset 94 of a3 or an integer at offset 90 of a3 is zero, it calls another routine with a value from a6 minus 32; finally, it clears two 32-bit fields at offsets 20 and 24 of a2, decrements a global byte, and conditionally calls another routine and another routine.
- `cleanup_structure_data`: Resets two 32-bit fields at offsets 20 and 24 of a structure in a2 to zero, decrements a global byte, and conditionally calls another routine if the global byte is zero and bit 3 of a global variable is set.
- `process_conditional_library_calls`: Tests a word at offset 252 of a structure, and if it is non-zero, calls two different library routines using pointers from a global table.
- `set_global_bit`: Sets bit 3 of a global variable and returns zero.
- `update_structure_and_call`: Writes a constant byte to an offset in a structure, modifies a byte at another offset, and calls another routine if a comparison with a constant fails.
- `invoke_table_entry_3`: The routine invokes a function pointer retrieved from an offset within a global variable using an index.
- `check_bits_and_set_byte`: Checks if bit 7 and bit 6 of a byte at offset 30 of a structure in a2 are both clear; if so, it writes the constant byte 254 to offset 31 of the structure, then calls another routine, and if bit 5 is set, calls another routine before calling a third routine.
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
- `update_linked_list_node_2`: Stores the address of the provided pointer into the memory location pointed to by that pointer, increments the value at that location by four, clears the following word, and stores the original pointer address into the word after that.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `invoke_table_entry_6`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_at_pointer_2`: Stores a byte from register d0 at the memory location pointed to by register a3 and increments the pointer.

### printer.device

- `initialize_printer_device`: Calls a routine through a pointer at offset -84 of a structure in a6, and if the result is non-zero, stores that result in a global variable.
- `set_status_and_branch`: Loads the constant -2 into d0 and calls another routine before returning.
- `manage_linked_list`: Sets a bit in a byte, performs a linked list insertion if a flag is set, and calls functions through pointers retrieved from a table.
- `execute_printer_initialization`: Calls a routine through a pointer at offset -84 of a structure in a6, and if the result is non-zero, stores that result in a global variable, sets a value at offset 34 of the result structure, and calls another routine through a pointer at offset -432 of a6.
- `initialize_printer_state`: Calls three library functions using pointers from a structure in a6, and if all return non-zero, initializes a structure in d0 by filling a memory range with the value 45019 via a loop, setting several pointers and a value from a library call, and finally calls two more library functions.
- `call_library_functions`: Calls three functions through pointers retrieved from a global variable, passing the value from a6 to the second and third calls.
- `execute_status_updates`: Calls three different routines in sequence and then returns.
- `set_status_and_branch_2`: Loads the constant -3 into d0 and calls another routine before returning.
- `set_status_bit_3`: Sets bit 0 of a byte at offset 61 of a structure in a6.
- `clear_status_bit`: Clears bit 0 of a byte at offset 61 of a structure in a6.
- `check_pointer_and_branch`: Loads a pointer from an offset of 48 of a6 into a1, checks if the pointer is null, and if not null, calls another routine before branching to a location earlier in the routine.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_previous`: Branches to a location earlier in the routine.
- `initialize_event_structures`: Calls a library function to get a value, stores it in a structure, initializes two linked lists with new nodes, sets several offsets in a structure to point to a common location, and enters a loop that calls a library function to check bits and iterates through a linked list to modify status bytes and call another routine.
- `clear_status_bit_2`: Compares d0 to a value loaded from an offset of a structure in a6; if they are equal, it clears bit 1 of a byte at an offset of the structure in a6.
- `process_event_loop`: Enters a loop that checks bits in a value returned by a library function to either iterate through a linked list of structures and call functions via a table, or call library functions and clear bit 0 or bit 1 of a byte at an offset of a structure in a6 based on the returned value.
- `call_handler_with_parameters`: Loads a pointer from an offset of 4 of a7 into a0, loads a value from an offset of 8 of a7 into d0, and calls another routine.
- `update_status_and_data`: Checks a byte from a global variable, and if it is zero, sets bit 0 or bit 1 of a byte at offset 1558 of a structure in a global variable, writes two 32-bit values to offsets 36 and 40 of a structure, writes the value 3 to offset 28 of a structure, and calls another routine twice.
- `adjust_stack_and_branch`: Adds 8 to a7 and branches to a location earlier in the routine.
- `set_status_bit_and_branch`: Sets bit 1 of a byte at offset 1530 of a6, loads the address of an offset of 94 of a6 into a1, and branches to a location earlier in the routine.
- `check_status_bits`: Loads a byte from an offset of a structure in a0, and if it is not zero, checks if the bitwise NOT of that byte masked with 3 is zero; if so, it clears bit 0 or bit 1 of a byte at an offset of a structure in a6 based on the result of a bitwise NOT and AND operation.
- `process_printer_status_loop`: Enters a loop that calls several routines, conditionally clears bits in a byte at offset 9b6 of a structure in a6, and returns a value from a byte at offset 31 of a global variable if it is non-zero.
- `check_and_set_status_2`: Loads a pointer into a1 from offset 46 of a6, compares it with d0, and if they are not equal, clears bit 1 of a byte at offset 246 of a6 and calls another routine.
- `compare_and_call`: Compares a 32-bit value at offset 272 of a structure in a6 with the value in a1, calls another routine if they are not equal, and otherwise returns if d0 is zero.
- `check_status_and_branch`: Calls another routine and then branches to a location earlier in the routine.
- `update_structure_fields_2`: Loads a pointer from a global variable, copies a 32-bit value from that pointer to offset 32 of a structure in a1, sets a 16-bit value at offset 28 of the structure to 9, and clears a 32-bit value at offset 36 of the structure.
- `clear_status_byte`: Sets a byte at a specific offset to zero and calls a routine through a pointer found in a structure.
- `check_status_byte`: Loads a pointer into a1 and compares the byte at offset 8 of that pointer to 7, otherwise it returns.
- `execute_callback_sequence`: Calls a routine through a pointer in a structure and then calls another routine using a value from a global base.
- `retrieve_printer_status`: Pushes a6 to the stack, loads a pointer from a specific location, reads a byte from offset 31 of that pointer, and if it is not zero, sets a6 to a value from the stack and checks a byte at offset 246 of a6; if the lower two bits of that byte are not zero, it returns, otherwise it clears d0 and returns.
- `update_structure_fields_2_2`: Loads a pointer from a global variable, copies a 32-bit value from that pointer to offset 32 of a structure in a1, sets a 16-bit value at offset 48 of the structure, and calls another routine.
- `compare_and_clear_bit`: Compares a longword at offset 94 of a structure in a6 to d0, clears bit 0 of a byte at offset 2462 of the same structure, and calls another routine before branching to another location.
- `compare_address_and_branch`: Compares a longword at an offset of 272 in a6 to d0, and if they are not equal, branches to another location; otherwise it calls another routine, tests d0, and branches based on the result.
- `branch_to_status_check`: Calls a routine and then branches to another location.
- `check_byte_and_branch`: Saves registers d2, d3, a2, and a3 to the stack, compares a byte at offset 1533 of a6 to 1, and if not equal, branches to a location earlier in the routine; otherwise, it pushes the current stack pointer and branches to another location.
- `update_interrupt_and_status`: Sets bit 15 of the INTENA register, modifies a byte at an offset of a global variable, and calls a library function; if the result is non-zero, it clears bit 0 or bit 1 of a byte at an offset of a structure in a6 and calls another routine accordingly.
- `clear_status_bits_2`: Calls another routine; if it returns 0, it clears bit 1 of a byte at offset 2486 of a structure in a6 and calls another routine if bit 0 was set, then clears bit 2 of that same byte and calls another routine if bit 1 was set.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `update_list_and_copy_string`: Pushes the value in a1 onto the stack twice, calls another routine, pops the value back into a1, calls another routine, and returns.
- `reset_structure_field`: Sets a 32-bit field in a structure at offset 20 to zero and conditionally updates it from another field based on a routine return value, then calls two other routines.
- `execute_indirect_calls_2`: Pushes the contents of a1 onto the stack twice, calls a routine using a pointer from a table, restores a1 from the stack, and calls another routine before returning.
- `update_list_and_copy_string_2`: Sets a status bit in a byte and performs a linked list insertion if a flag is set, then calls another routine to find a marker in an array of words and copy a null-terminated string into a lookup table.
- `dispatch_pending_entry_8`: Checks a condition via a routine call; if met, it populates a structure with two parameters and several constants, then calls three routines through a global base using a pointer retrieved from the structure.
- `call_printer_status_handler`: Saves registers to the stack, loads a pointer from an offset of 8 in a6, and calls a routine at a fixed address with a3 and a4 pointing to specific constants.
- `process_printer_data_buffer_2`: Calculates the length of a null-terminated string, copies bytes from a source buffer into a destination table, and then iterates through the data to handle specific control characters by calling another routine.
- `update_printer_pointers`: Moves a longword from a memory location pointed to by a register into a global variable, clears a 16-bit word in a global variable, and branches to another location.
- `parse_ansi_escape_sequences_3`: Iterates through a buffer to identify and process escape characters, handling specific control codes such as line feeds and carriage returns by calling other routines to manage the sequence processing and buffer state.
- `clear_registers_and_branch`: Sets registers d4, d6, and d2 to zero and branches to another routine.
- `process_printer_data_buffer`: Iterates through a data buffer to check for specific characters and manages a queue by copying bytes from a source pointer into a destination structure.
- `copy_and_update_data`: Increments two counters and copies a byte from a global variable to a destination indexed by the first counter in a loop, then writes a 32-bit value to offset 32 of a structure in a2 based on the value of d2.
- `clear_memory_offset`: Stores a zero at a specific offset within a memory location pointed to by a register.
- `decode_obfuscated_data`: Iterates through a loop to perform character substitutions on a value based on its index and comparisons against a table, then calls another routine.
- `clear_and_branch_6`: Clears a longword at a negative offset from a6 and branches to another location.
- `decode_obfuscated_string_table`: Iterates through a loop to compare bytes from a source pointer against a table, performing conditional arithmetic or character substitutions to store a decoded byte into a destination pointer.
- `compare_and_set_status_byte`: Compares a longword in a global variable with 1, and if they are equal, compares d0 with d5; if d0 and d5 are equal, it stores a byte of all ones at the address in a2.
- `compare_and_set_byte`: Compares a longword in a global variable with the value 2 in d0, and if they are equal, compares d0 with d5; if d0 and d5 are not equal, it stores the byte value 75 at the address in a2 and branches to another routine.
- `update_status_byte_4`: Compares d0 with d5 and, if they are not equal and the byte at the address in a4 is non-zero, stores the byte value 59 at the address in a2.
- `copy_from_table_with_offset`: Compares d0 with d5, and if d0 is less than d5, moves d5 into d0, then copies a byte from a table at a fixed base address plus d0 into the location pointed to by a2 before branching to another routine.
- `check_and_set_byte`: Compares a byte at the address in a4 to 29 and, if it is not less than that value and d5 is zero, stores the byte 12 at the address in a2 before branching to another routine.
- `decode_obfuscated_string_table_2`: Iterates through a source buffer and performs character substitutions based on comparisons against a lookup table.
- `process_ansi_escape_sequences`: Iterates through a buffer to identify and process escape characters, handling specific control codes such as line feeds, carriage returns, and extended character mappings via a lookup table.
- `parse_ansi_escape_sequences`: Iterates through a buffer to identify and process escape characters, handling specific control codes such as line feeds, carriage returns, and various escape sequences.
- `parse_escape_sequences_2`: Iterates through a buffer to identify and process escape characters, handling specific control codes such as line feeds, carriage returns, and character mapping from a table.
- `check_escape_character`: Compares a byte at a pointer to a constant and branches to another routine if they are not equal; otherwise it branches to a third routine.
- `parse_printer_command_stream`: Parses a stream of bytes to interpret printer control codes, handling escape sequences, line feeds, and character expansions from a lookup table.
- `process_control_byte`: Compares a byte at a pointer to a constant, checks a byte at a global variable, and if both conditions are met, stores a byte in a location pointed to by a register before branching.
- `process_printer_command_stream`: Parses a stream of bytes to identify control characters and command sequences, writing line feed and carriage return bytes to a destination buffer when certain conditions are met.
- `write_multiple_bytes`: Loads a pointer from a2, increments it, and writes the byte 10 to the address at the original pointer plus the offset in d2, then repeats this process to write the byte 13 to the next pointer location and branches to another routine.
- `write_single_byte_2`: Loads a pointer from a2, increments it, and writes the byte 10 to the address at the original pointer plus the offset in d2 before branching to another routine.
- `push_and_call_routine`: Compares a byte at a pointer to a constant, and if it matches, pushes two longwords onto the stack, calls another routine, and then branches.
- `parse_file_path_string`: Processes a character stream to handle file path syntax, specifically looking for backslashes and drive designators to calculate string lengths and extract path components into a destination buffer.
- `parse_escape_sequences`: Parses a string for backslash escape sequences and converts numeric characters into their integer values.
- `update_pointer_and_branch`: Increments a1 and a global variable, checks a byte at the address in a1, and if it is zero, performs arithmetic on d0 using a global variable and branches based on the result.
- `calculate_table_address`: Loads a byte from the address in a3, subtracts 160, shifts the result left by 2, adds the value in d4 to it, and then loads a pointer from the resulting address into a1 before branching to another routine.
- `skip_non_numeric_characters`: Checks if the byte at the location in a1 is a backslash, and if not, checks if the following byte is a numeric digit between 0 and 9, branching to another routine if these conditions are not met.
- `parse_numeric_string_and_process_escapes`: Parses a three-digit numeric string into an integer and stores it in a buffer, then iterates through a character stream to handle escape sequences and special control characters by calling another routine.
- `copy_byte_to_offset_address`: Increments a pointer in a2, calculates an address by adding d2 to the original pointer value, moves a byte from the address in a1 to that calculated address, and branches based on the value of the next byte in a1.
- `parse_ansi_escape_sequences_2`: Iterates through a buffer to identify and process escape characters, handling specific control codes such as line feeds, carriage returns, and bracketed sequences.
- `update_global_values`: Subtracts the difference between two global variables from a third global variable and clears a fourth global variable.
- `process_buffer_data_3`: Sets a global variable to zero and calls another routine; if the return value is positive, it either copies that many bytes from a stack buffer to a destination pointed to by a structure in a6 using a loop that increments a global offset, or it updates two global variables and calls another routine if the value exceeds 100.
- `check_and_branch_on_value`: Adds d3 to a 32-bit value from a memory location, compares it to 100, and if it is greater than 100, stores d3 in a memory location, checks a byte at an offset from a6, and either calls another routine or branches to another routine.
- `decrement_and_store_pointer`: Decrements a 32-bit value from a global variable by one, stores the result in another global variable, and branches to another routine.
- `clear_register_and_branch`: Sets register d4 to zero and branches to another routine.
- `copy_memory_block_2`: Copies a sequence of bytes from a source address to a destination address, incrementing a global pointer for each byte.
- `clear_byte_at_pointer`: Restores registers from the stack, loads three values from the stack into d3, d2, and a2, clears the byte at the address in a2, and branches to another routine.
- `clear_byte_at_offset_3`: Compares d1 to 59 and, if they are equal, increments a pointer in a3, calculates an address by adding a2 to that pointer, and clears the byte at that address.
- `set_pointer_and_increment`: Compares d1 to 32, and if they are equal, loads a pointer from the stack, sets the value at that pointer to 1, and branches to another routine.
- `compare_and_load_pointer`: Compares a byte in d1 to 34; if they are equal, it loads a pointer from offset 32 of the stack into a0 and branches to another routine, otherwise it branches to another location.
- `parse_input_character`: Reads a byte and enters a loop that checks if the character is a digit, a space, or a specific delimiter.
- `increment_and_fetch_table_value`: Increments a value at a pointer, fetches a byte from a table using the original value as an index, and if a condition involving a parameter and a global variable is met, calls another routine and sets the result to zero.
- `return_register_value`: Returns the value currently held in a data register.
- `update_global_counter`: Loads a 32-bit value from a global variable, decrements it by one, and stores the result in a global variable at offset 440 of a structure pointed to by a1.
- `decrement_and_check_table`: Decrements a value pointed to by a register and continues looping as long as the value is positive and a specific bit pattern is found in a table.
- `reset_global_state`: Clears three global variables, sets a byte at offset 270 of a structure to zero, and sets another global variable to 3.
- `copy_data_to_globals`: Copies six 16-bit values from a structure starting at offset 1654 into six global variables.
- `compare_data_tables`: Compares six 16-bit values from a structure starting at offset 1654 against six global variables, returning 0 if all match or all ones otherwise.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_conditional_calls`: Extracts four bytes from a structure in a1, calls another routine with a pointer and a value, and then performs a series of conditional calls based on the return value and a short value at offset 32 of the structure.
- `dispatch_conditional_routines`: Calls another routine with different arguments based on the value of d2 and the return value of a previous call, returning the lower byte of the result.
- `copy_string_from_table`: Calls a routine through a pointer, then either sets a global variable or copies a string from a table into a destination buffer at the address in a1, handling specific character replacements along the way.
- `process_status_word`: Compares a word at offset 22 of the stack to 54 and 53, and if either matches, it performs bitwise operations on a word from the stack and stores the result in a global variable at offset 262.
- `copy_string_data_2`: Copies bytes from a source buffer to a destination buffer, handling specific character replacements and terminating the loop when a null byte or a specific byte value is encountered.
- `compare_and_branch_d1`: Compares the byte in d1 to 255 and branches if it is not equal; otherwise it sets d1 to 255 and branches to another routine.
- `copy_string_data`: Iterates through a source buffer, copying bytes to a destination buffer and updating an index, until a null byte is encountered or a specific byte value is found, then performs a final copy and returns a calculated length.
- `execute_jump_table_2`: Pushes d2 to the stack, loads a 32-bit value from the stack into d2, moves a pointer into a0, and performs a jump to an address determined by a table lookup using d2 as an index.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `validate_printer_buffer_dimensions`: Extracts buffer and offset parameters from a structure, calculates the effective width and height by subtracting offsets from buffer dimensions, and validates that these dimensions meet minimum threshold requirements.
- `update_printer_timing_parameters_2`: Calculates buffer dimensions by subtracting offset values within a structure and compares these against minimum threshold values stored in a global table.
- `recalculate_printer_timing_parameters_2`: Calculates the minimum values from two structure offsets and stores them in a structure at a6, then performs bitwise manipulations on a global variable.
- `recalculate_printer_timing_parameters`: Calculates minimum values from two structure offsets and stores them in a structure at a6, then performs bitwise manipulations on a global variable.
- `update_printer_timing_parameters`: Calculates minimum values from two structure offsets and stores them in a structure at a6, then performs bitwise manipulations on a global variable and updates several values in a structure based on conditional checks and bitwise flags.
- `compare_and_branch_6`: Compares the word in register d0 to 2048; if it is not equal, it branches to another routine, otherwise it branches to another routine.
- `load_constant_and_branch_2`: Loads the value 12 into register d1 and branches to another routine.
- `load_constant_and_branch_3`: Loads the value 15 into register d1 and branches to another routine.
- `update_printer_buffer_and_status`: Calculates minimum values from a structure and stores them at offsets 6 and 14 of a structure in a6, then performs bitwise manipulations on a global variable and updates two values in a4 and d4 based on various conditional bitmask checks and arithmetic operations.
- `update_printer_status_and_buffer`: Calculates a minimum value from a structure, updates a status byte, and performs complex bitwise manipulations and conditional updates on two global integer values based on a bitmask.
- `modify_offset_and_branch`: Loads two small constants into registers, stores one at an offset of negative four from a pointer in a6, checks a byte at a global address and branches if it is non-zero, otherwise it adds the value in d6 to itself, checks another global byte and branches if it is non-zero, otherwise it adds the value stored at the offset in a6 to itself and branches to another routine.
- `check_status_and_branch_2`: Clears a longword at a global address, checks a byte at another global address and branches if it is zero, then checks a third global byte and branches if it is zero, otherwise it pushes a stack pointer and branches to another routine.
- `process_color_clamping_and_table_clear`: Clamps two 32-bit values within specified bounds, calculates their absolute differences, and clears a 64-entry table in a global structure.
- `clamp_and_populate_color_table`: Clamps two 32-bit values within specified bounds, calculates their absolute differences, clears a 64-entry table in a global structure, and populates it by extracting bitplane nibbles from color values through a loop.
- `update_color_lookup_tables_3`: Calculates color bounds, clears a 64-entry table, and populates it by deconstructing color values into bitplane nibbles via a loop.
- `update_color_lookup_tables_2`: Calculates color bounds, clears a 64-entry table, and populates it by deconstructing color values into bitplane nibbles via a loop.
- `update_color_lookup_tables`: Calculates color bounds, clears a 64-entry table, and populates it by deconstructing color values into bitplane nibbles via a loop.
- `populate_color_lookup_tables`: Clears a 64-entry table and populates it by deconstructing color values into separate bitplane nibbles using a loop.
- `convert_rgb_to_planar_format_2`: Clears a 64-entry table and populates it by deconstructing color values into separate bitplane nibbles.
- `prepare_planar_color_table`: Clears a table of 64 entries and populates it by calling convert_rgb_to_planar_format to deconstruct color values into separate bitplane nibbles.
- `convert_rgb_to_planar_format`: Deconstructs 16-bit color values into 4-bit nibbles and stores them into separate bitplanes in a global table using a loop.
- `initialize_global_values_2`: Sets three global variables to specific values, calls a routine via a pointer, and then conditionally calls another routine multiple times based on the returned value and a modulo check against a 16-bit value from a structure.
- `execute_callback_and_store_result`: Pushes three words onto the stack, calls a routine via a pointer in a global variable, stores the returned value into a global variable, and then branches to another routine or returns based on the zero flag.
- `test_status_and_call`: Tests two bytes in global variables and, if both are non-zero, calls another routine, then branches to another location; otherwise it branches to another location.
- `check_and_call_printer`: Tests a byte in a global variable and, if non-zero, calls another routine, then branches to another location; otherwise it branches to another location.
- `check_printer_status_and_call`: Tests a byte in a global variable and, if non-zero, calls another routine before branching to another location; otherwise, it branches to a different location.
- `call_and_store_result_4`: Calls another routine, stores the returned longword from d0 into a global variable, and branches to one of two different locations depending on whether that longword is zero.
- `check_modulo_or_callback`: Checks if a global value modulo a 16-bit value from a structure is zero or if a routine call returns 0, then calls another routine with the result.
- `call_empty_routine`: Calls another routine with two zero arguments and returns.
- `calculate_scaled_value_2`: If a global variable is zero, it checks bit 2 of a byte at offset 21 of another structure and returns the input if the bit is not set, otherwise it returns the smallest of three input bytes.
- `find_minimum_byte`: Checks bit 2 of a byte at offset 21 of a global variable; if set, it finds the smallest byte among three provided bytes and returns it, otherwise it returns the original second parameter.
- `resolve_printer_table_index`: This routine calculates an index using a global variable and constants, calls another routine, and then performs conditional logic to update a global variable with a value from a table or a modified value.
- `resolve_table_index_and_update_pointers`: Calculates an index using a base address and a multiplier, then uses the resulting value to fetch a 32-bit pointer from a table.
- `update_pointers_from_table`: Loads a longword from a table using an index, updates four global pointers by adding a global base address to them, and calls another routine with the loaded value.
- `update_pointer_and_branch_2`: Moves a longword from a memory location to the address in a2, tests bit 3 of a byte at offset 21 of a0, and if that bit is not set, branches to another location; otherwise it subtracts d2 from 15 and branches.
- `process_word_and_branch`: Moves d2 to d1, moves d3 to d0, shifts d0 right by 4 bits, and then branches based on whether d0 is less than or equal to 1 or greater than 1.
- `branch_on_constant`: Compares a word in d0 to 2, and if it is not equal, branches to another location; otherwise it branches to another location.
- `decrement_and_loop`: Decrements d0 and loops back to an earlier instruction if the result is not zero, otherwise branches to another location.
- `store_byte_and_branch`: Stores a byte from d1 into a memory location at offset 272 and branches to another location.
- `store_byte_and_branch_2`: Stores a byte from d1 into a memory location at offset 274 and branches to another location.
- `update_pointer_table_and_store`: Moves a byte from d1 to a memory location, pushes a 32-bit value from a pointer in a2 onto the stack, calls another routine, moves d0 into the location pointed to by a2, increments the stack pointer by 4, moves the value from a2 into a memory location, and branches to another routine.
- `update_pointer_table`: Loads a 32-bit value from a table using an index and updates four global pointers by adding a global base address to them.
- `calculate_and_update_pointers`: Calculates a value based on the product or sum of two globals and updates four global variables by calling another routine multiple times.
- `evaluate_multiple_conditions`: Sets a global variable to zero and then performs a series of conditional checks by calling several routines and checking their results.
- `evaluate_table_conditions_2`: If a global variable is zero, it calls three different routines using values from a table and parameters in a3 and a4, and if any of these calls return a non-zero value, it stores that result in the location pointed to by a2 and returns it; otherwise it returns zero.
- `evaluate_table_conditions`: Compares a value at a global variable to a table using two parameters; if the first comparison is zero, it performs a second comparison with a different table index and stores the result in a pointer-referenced location, otherwise it performs a comparison with a different index and stores the result.
- `compare_table_value_and_store`: Calls another routine, and if the result is non-zero, calls a routine to compare a byte from a table indexed by two parameters and stores the result in a location pointed to by a2.
- `save_registers_and_return`: Clears d0 and saves registers a2 through a4 and a7 to the stack before returning.
- `increment_and_call_handler`: Increments a global variable, and if a condition involving a modulo of that variable and a value from a structure is met, calls a routine twice and stores the result in a global variable; otherwise, it sets a local variable to zero.
- `call_and_store_d0`: Pushes three zeroed values onto the stack, calls a routine at an address stored in a memory location, stores the result from d0 into the location pointed to by a2, and branches to another routine or a different location based on the zero flag.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `update_word_via_pointer`: Saves registers a2 through a4 to the stack, loads three specific addresses into a2, a4, and a3, moves a word from a global variable into the address pointed to by a4, and branches to another routine.
- `process_nested_loops_2`: Performs a series of nested loops involving multiple calls to other routines, updates several global variables, and performs arithmetic on values stored at various memory locations.
- `execute_nested_loops_2`: Performs nested loops that call another routine, updates a global variable, and modifies a table or structure based on a value from a routine call.
- `clear_and_branch_5`: Clears a 16-bit word at offset 316 of a structure and branches to another routine.
- `execute_nested_loops`: Performs a series of nested loops that increment global variables, call other routines, and update values in a table or structure based on conditional results.
- `process_nested_loops`: Performs a loop that increments a global variable and updates a value based on a multiplication, then enters a while loop that calls another routine, updates a pointer, and performs further calculations and nested loops.
- `increment_and_check_limit`: Adds a word from the address in a3 to a word at the address in a4, then compares the resulting word at the address in a4 to 300; if it is lower, it branches to another routine, otherwise it clears d0, restores registers a2 through a4 from the stack, and returns.
- `clear_and_update_word`: Saves registers a2 through a4 to the stack, loads three specific addresses into a2, a4, and a3, clears a longword at a global variable, moves a word from a global variable into the address pointed to by a4, and branches to another routine.
- `decrement_and_process_loops_2`: Executes a complex sequence of nested loops and calls to other routines that manipulate global variables and perform arithmetic on values stored in memory.
- `clear_global_longwords`: Clears four longwords at specific memory locations with zero and then branches to another routine.
- `process_iterative_updates`: Executes a loop that decrements a value from a routine call, performs conditional updates to a pointer's target, and clears several global variables.
- `call_and_store_result_2`: Calls another routine, stores the result in the address pointed to by a2, and if the result is not zero, loads the value from that address into d0 and branches.
- `decrement_and_process_loops`: Decrements a word at offset 110 of a structure in a2 and enters a loop that increments a global variable, performs pointer arithmetic, and executes nested loops and routine calls.
- `increment_and_check_limit_2`: Adds a 16-bit value from a pointer in a3 to a 16-bit value at a pointer in a4, then compares the result against a constant.
- `initialize_printer_buffer_parameters`: Clears a 32-bit word at a global offset, copies a 16-bit word from a global variable to a memory location, and branches to another routine.
- `call_with_stack_parameters_2`: Pushes two 16-bit values from memory locations into the stack, calls another routine, stores the result in the location pointed to by a3, and branches to an earlier instruction.
- `initialize_structure_fields_3`: Clears a word at a global address, copies a longword from one global address to another, copies a longword from another global address to a third, and moves a word from a fourth global address to a fifth before branching to another routine.
- `increment_and_call_routines_2`: Increments a global variable, performs nested loops that call other routines, and updates a pointer's target or a global variable based on results.
- `increment_and_call_routines`: Increments a 16-bit word in a global variable, compares it to a value in a register, and calls two different routines before storing a result in a pointer's target.
- `execute_conditional_decrement_loop`: Increments a global variable, enters a loop that performs multiplication and pointer updates, and includes a nested while loop that decrements a value at a pointer and increments another value until a threshold is reached.
- `decrement_and_verify_limit`: Decrements a word at the address in a3 and branches to another routine if the result is not higher than 0, otherwise it increments a word at the address in a4, compares the word at the address in a4 to 300, and if it is lower, branches to another routine, otherwise it clears d0, restores registers a2 through a4 from the stack, and returns.
- `initialize_global_values`: Saves registers a2 through a4 to the stack, loads specific addresses into a2, a4, and a3, clears d0, stores d0 into three different longword locations, moves a word from the address in a3 to the address in a4, and branches to another routine.
- `call_with_stack_parameters_alt`: Pushes two 16-bit values from memory locations into the stack, calls another routine, stores the result in the location pointed to by a3, and branches to a different earlier instruction.
- `copy_multiple_pointers`: Clears a word at offset 322, copies a longword from offset 302 to offset 330, copies a longword from offset 16 to offset 276, and copies a word from offset 378 to offset 314.
- `update_structure_fields_5`: Clears four global variables and enters a loop that calls another routine, updates a pointer's target, and performs conditional increments on a structure field.
- `store_result_and_branch_2`: Calls another routine, stores the returned longword into a memory location pointed to by a register, and then either branches to another routine or loads that same value back into a register before branching to a different location.
- `update_structure_fields_6`: Enters a loop that increments a global variable, calls another routine, and updates a pointer's target or a global variable based on the result.
- `validate_and_increment_counter`: Decrements a word at the address in a3, and if it is not below zero, increments a word at the address in a4; it then compares the value at the address in a4 to a constant and either branches to another routine or restores registers and returns.
- `accumulate_scaled_values`: Iteratively adds a doubled value to a pointer's target until the target is no longer less than one, then subtracts a doubled second parameter from that target.
- `compare_table_value_2`: If a global variable is zero, it compares a byte from a table indexed by two parameters to the input byte.
- `compare_table_value`: Performs a comparison between a byte retrieved from a table using bitwise-derived indices and a provided byte.
- `negate_boolean_results`: Calls two different routines and returns the negation of the first return value based on the status of a second return value.
- `rotate_and_modulo_32bit`: Performs bitwise rotations and arithmetic operations on a 32-bit value based on a count and a divisor.
- `rotate_32bit_value_2`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_7`: Computes the 32-bit product of two 32-bit integers by summing the partial products of their 16-bit components and returns the result.
- `forward_call_to_helper_2`: Calls one helper and returns, with no other work.
- `transfer_register_to_register_via_call`: Moves the value from register d0 to d1, calls a helper routine, and then moves the resulting value from d1 back into d0 before returning.
- `negate_result_on_flag_4`: Calls rotate_and_modulo_32bit and negates the returned value if the helper leaves a non-zero value in an address register.
- `call_and_negate_on_flag_3`: Calls a routine and returns its result, or returns the negation of that result if the second return value in an address register is nonzero.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `process_string_table`: Compares a string from a table to a global buffer in a loop, calls another routine to handle specific conditions, and copies bytes from a source pointer to a destination pointer.
- `fetch_table_entry_via_index`: Multiplies a 16-bit value in d2 by 4 and uses the result as an offset into a table located at a fixed address to load a pointer into a4, then copies that pointer to d4 and branches.
- `branch_to_offset_334`: Performs a short branch to an instruction located 334 bytes ahead of the current location.
- `load_constant_and_branch`: Loads the constant 32 into a2 and branches to another location.
- `process_string_and_callback`: Iterates through a string by comparing characters, and if a null terminator is reached, calls another routine; otherwise it checks a pointer at offset 86 of a structure, calls it if non-zero, and performs several other calls and data movements including copying a string to a global variable.
- `process_printer_device_callback`: Checks if a pointer at offset 86 of a structure is non-zero, executes that pointer, and clears the pointer and a 16-bit value at offset 90.
- `unlock_and_get_directory`: Calls a routine to unlock a resource using a pointer from a register, then calls another routine to retrieve the current directory, and returns negative one.
- `execute_callback_and_recurse`: Checks a global value, calls a function pointer from a table, and then calls itself with a value from a pointer.
- `compare_case_insensitive_chars`: Adjusts two input bytes by subtracting 32 if they fall within a specific range and returns whether they are equal.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_3`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### narrator.device

- `initialize_device_structure`: Calls a routine via a pointer in a global variable to obtain a structure, populates several fields in that structure including a pointer to the global variable and a stack-based pointer, then calls another routine via a pointer in the global variable to obtain a second structure and populates its fields before calling two more routines via pointers in the global variable and returning -1.
- `test_and_negate_d0`: Tests the long value in d0; if it is zero, it branches to another routine, otherwise it sets d0 to negative one and branches to another routine.
- `update_narrator_device_state`: Checks a status flag in a structure; if zero, it attempts to initialize the device by calling two functions from a library, otherwise it increments two counters in the structure and updates a secondary structure with various values and a pointer.
- `update_device_state`: Stores a value into a structure, increments two counters in that structure, copies one counter to another structure, calls another routine, and then either writes specific values to a second structure or updates several fields in the first structure and a third structure.
- `update_device_state_fields`: The routine stores an input value into a structure, increments counters in the structure, copies a counter to another structure, writes several constant words and bytes to the second structure, and clears a flag bit in the first structure.
- `update_device_state_fields_2`: Stores a byte from d0 into a structure at offset 31 of a2, writes a long value of negative one to offset 20 of a2, and branches to another routine.
- `manage_linked_list_flags`: Loads a pointer from offset 20 of a1 into a3, loads a pointer from a fixed address into a6, clears bit 0 of a byte at offset 30 of a1, and if bit 6 of a byte at offset 34 of a3 is set, it branches to another routine; otherwise, it checks a 32-bit value at offset 772 of a3 and, if non-zero, branches to another routine, otherwise it sets a byte at offset 31 of a1 to 248 and calls a routine at the address in a6.
- `manage_linked_list_2`: Compares a value in a structure with a value in another structure and, depending on the result, either updates a byte and calls another routine or performs a linked list insertion by updating pointers and a counter in a global structure.
- `copy_byte_and_split_nibbles_2`: Copies a byte from one structure to another, splits it into low and high nibbles into two bytes, clears a byte in the destination, and manages a linked list while modifying the INTENA register.
- `copy_byte_and_split_nibbles`: The routine copies a byte from one structure to another, splits it into low and high nibbles into two bytes, clears a byte in the destination structure, and calls another routine.
- `process_speech_queue_and_update_metadata`: Iterates through a linked list of speech data, decrementing a reference counter in a global structure and updating a destination structure with validated timing, volume, and playback parameters.
- `decrement_speech_ref_count`: Decrements a byte in a global structure and, if the value reaches zero, disables interrupts via the INTENA register and jumps to another routine.
- `traverse_and_decrement_counter`: Iterates through a linked list of pointers starting from a pointer at offset 54 of a structure in a6, calling another routine if a 32-bit value in the current list element matches a 32-bit value at offset 24 of a1.
- `decrement_counter_and_close`: Decrements a 16-bit counter in a structure and, when it reaches zero, calls two routines via pointers in a global variable and optionally calls another routine.
- `set_byte_and_call_routine_2`: Loads a word from offset 28 of a1, and if it is between zero and eight inclusive, calls a routine from a table using the word as an index multiplied by four.
- `set_status_bit_4`: Loads a pointer from a global variable into a6, then checks a byte at offset 32 of the structure pointed to by a3; if that byte is not zero, it sets bit 5 of the byte at offset 34 of the structure pointed to by a0.
- `swap_pointers_and_return_value`: The routine calls a function via a base pointer, swaps a value between a pointer table and a data word, updates a pointer field, calls another function, and returns a value from a table entry.
- `process_event_request_queue`: Inspects a request structure to determine its type and modifies status flags in a secondary structure accordingly.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `modify_structure_flags`: Clears bit 5 of a byte at offset 34 of a structure in a0, writes 7 to a word at offset 196 of a structure in a6, sets bit 0 of a byte at offset 198 of that same structure, calls two routines via pointers found in a structure in a6, and sets a byte at offset 31 of a structure in a1 to zero.
- `set_flags_and_invoke_handler`: The routine sets a flag bit in a structure, writes a word and sets a bit in a global structure, calls a handler via a base pointer, clears a byte in another structure, and calls another routine.
- `set_flag_then_clear`: The routine sets bit 7 in a byte at offset 34 of a structure, calls two other routines, then clears bit 7 in the same byte.
- `set_flag_clear_byte_then_call`: Sets bit 7 in a byte at offset 34 of a structure, calls another routine, clears bit 7, clears a byte at offset 31 of a second structure, and calls another routine.
- `toggle_interrupt_state`: Calls another routine via a pointer at offset 20 of a3, sets a global variable to 65536, increments a byte at offset 294 of that global variable, and iterates through a linked list of pointers, calling another routine for each element.
- `decrement_counter_and_set_global`: The routine decrements a byte in a global variable and, if the previous value was less than one, writes a constant to the INTENA register.
- `enable_interrupts_and_set_bit`: Saves registers, sets the interrupt enable bit in the INTENA register, increments a byte in a global structure, and sets bit 5 of a byte at an offset in a structure if a comparison succeeds.
- `update_pointer_table_2`: Loads a pointer from the location pointed to by a0 into a0, then loads a longword from that new address into d0; if d0 is not zero, it compares a0 with a1 and, if they are equal, updates a0 with the value from d0.
- `update_pointers_and_decrement_counter`: The routine updates two pointers, writes a byte to a structure, decrements a byte in a structure, and if the value is less than one, writes a constant to the INTENA register, calls another routine, and branches.
- `decrement_counter_and_enable_interrupts`: Decrements a byte at offset 294 of a structure pointed to by a6 and, if the value was zero or less, writes 49152 to a global variable and enables interrupts via the INTENA register; otherwise it returns.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `set_byte_and_call_routine`: The routine writes a byte to a structure and calls another routine.
- `clear_byte_and_call_routine`: The routine clears a byte in a structure and calls another routine.
- `clear_flags_and_call_handler`: The routine clears a flag bit in a structure, clears a byte in another structure, and calls a handler via a base pointer.
- `populate_speech_metadata_structure`: Populates a speech structure with validated values from a source buffer, including bounds-checked coordinates, flags, and a timestamp, then performs a backward scan of a text buffer to find delimiters such as commas, hyphens, or spaces.
- `compare_and_conditionally_store_word`: Compares a 16-bit word in register d1 to 320; if it is greater than 320, it branches to another routine, otherwise it stores the word at offset 30 of a structure in a5.
- `compare_and_store_word`: Compares a 16-bit word in register d1 to 320; if it is greater than 320, it branches to another location, otherwise it stores the word at offset 28 of a structure in a5, loads a 16-bit word from offset 52 of a structure in a2 into d1, and branches to another location if d1 is 0 or 1, otherwise it loads 232 into d3 and branches to another location.
- `store_and_compare_word_2`: Stores a 16-bit word from register d1 at offset 48 of a structure in a5, loads a 16-bit word from offset 54 of a structure in a2 into d1, and branches to another routine if d1 is 0 or 1, otherwise it branches to another routine after loading 233 into d3.
- `store_and_compare_word`: Stores a 16-bit word from register d1 at offset 38 of a structure in a5, loads a 16-bit word from offset 64 of a structure in a2 into d1, and compares it to 5000; otherwise it branches to another location.
- `validate_and_store_text_offset`: Compares a 16-bit value in register d1 against 26464; if it is not greater, it stores the value in a 16-bit word at offset 2168 of a structure pointed to by a5.
- `scan_text_buffer_for_delimiters`: Scans a text buffer backwards from a specified offset to find a comma, a hyphen, or a space.
- `parse_text_buffer_delimiters_2`: Scans a text buffer backwards from a specified offset to find a comma, a hyphen, or a space.
- `copy_longword`: Copies a longword from offset 36 of a structure in a1 to offset 32 of the same structure, then branches to another location.
- `write_long_and_byte_then_call`: The routine writes a longword zero to a structure, writes a byte from a register to another structure, and calls another routine.
- `parse_text_buffer_delimiters`: Scans a text buffer backwards from a specified offset to find a comma, a hyphen, or a space.
- `process_buffer_offset`: Calculates a value by subtracting a bit-shifted version of a word from itself, compares the result against a constant in a table, and either clears a longword and writes the result to a structure or performs a branch.
- `shift_and_clear_value`: Loads a word from offset 150 of a structure in a5 into d0, shifts it right by one bit, stores the result in offset 156 of the structure, clears the longword at offset 150, and branches to another location.
- `initialize_speech_buffer`: Initializes a structure with specific byte and word values, calls two routines, and if a condition is met, allocates a new structure and populates it with data from a table using a nested loop that writes each word twice into a buffer.
- `transfer_speech_status`: Loads a longword from offset 24 of a structure in a1, stores the byte from it into offset 67 of a structure in a2, sets d0 to 34 and d1 to 65537, then calls another routine; if d0 is non-zero, it branches to another location, otherwise it sets d3 to 254 and branches to another location.
- `copy_speech_data`: Initializes a structure, calls two routines, and if a value from a table is non-zero, enters a nested loop to copy words from a source table into a destination buffer, writing each word twice per iteration.
- `calculate_and_call_library`: Stores a byte from d0 into offset 68 of a structure in a2, calculates a value by doubling d0 and multiplying it by 68, calls a routine using a pointer at an offset from a6, and if the calculated value is not zero, branches to another location; otherwise it sets d0 to 254 and branches to another location.
- `copy_word_table_to_buffer`: The routine stores a pointer in a structure and copies words from a table to a buffer, writing each word twice per iteration, repeating for a count derived from a byte in a structure.
- `perform_bit_tests`: The routine performs several bitwise operations and bit tests on registers.
- `handle_io_and_free_resources`: Writes a byte and a word to a global variable, calls a routine via a global variable, conditionally calls two other routines via the same global variable, clears a word in the global variable and returns a combined value.
- `check_word_and_return`: Tests a 16-bit word at an offset from a register, and if it is greater than zero, clears a register and returns; otherwise it returns immediately.
- `parse_narrator_script_tokens`: Parses a sequence of bytes from a buffer, performing lookups in a table to identify command tokens and updating several state buffers at offsets 232, 744, and 776 of a structure pointed to by a5.
- `modify_and_branch_3`: Negates a 16-bit value in d1, adds 111 to it, and branches to another location if d0 is less than 12288; otherwise it adds the value in d2 to a0 and branches to another location.
- `parse_narrator_token_stream_2`: Iterates through a byte stream to identify tokens by comparing characters against a lookup table, updating a metadata structure with indices and bit-flags based on the character values.
- `parse_narrator_token_stream`: Parses a stream of bytes to update a table of indices and metadata, performing bitwise operations on specific bytes based on their values and checking against a lookup table.
- `set_bit_on_condition`: Compares the byte in d1 to 5; if it is not equal, it branches to another routine, otherwise it sets bit 5 of the byte at the offset in d3 of the structure pointed to by a6 and branches to another routine.
- `set_status_bit_5`: Compares the value in register d1 to 6; if it is not equal, it sets bit 2 of a byte located at offset -1 of the structure pointed to by a6 plus the offset in d3, then branches to another routine.
- `update_table_index_and_decrement_counter`: Writes a byte from a register to a table at an offset determined by an index, increments that index, and if the byte is neither one nor two, checks if the index is less than five hundred twelve; if so, it subtracts a longword from a structure at offset one hundred forty-six from a register and stores the result at offset one hundred fifty in the same structure, otherwise it sets the carry flag and returns.
- `clear_conditional_bytes`: Subtracts a longword from offset 146 of a structure in a5 from a0, compares the result to a word at offset 156 of the structure in a5, and if the result is greater than or equal to that word, it clears a byte at offset d3 of a structure in a3 and a byte at offset d3 of a structure in a6 before branching to another location.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends immediately.
- `decrement_index_conditionally`: Loads a byte from a table using an index, and if the byte is non-zero, decrements the index and branches.
- `update_narrator_buffer_state`: Checks a bit in a table entry and, if not set and an index is within bounds, writes specific byte values to three parallel arrays.
- `write_word_to_offset`: Writes a word from register d3 to offset 154 of the structure in a5 and sets the carry flag.
- `update_status_flags`: Iterates through a string until a null terminator or a specific byte is found, then performs bitwise checks on values retrieved from a table to set bits in a structure.
- `set_status_bit_conditional`: Performs a bitwise AND on d2 with 21408; if the result is not zero, it sets bit 4 of a byte at offset -3 of the structure in a0 and branches to another location.
- `check_and_set_status`: Loads a longword from a table using a0 and d1 as index, masks it with 6864, and if the result is not zero, checks if a byte at offset -2 of a structure in a1 is 48; if it is, it sets bit 4 of a byte at offset -3 of a structure in a0 and branches to another location.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `transform_encoded_text_stream`: Parses a byte stream from a source buffer and applies bitwise transformations to a destination buffer using a lookup table.
- `process_narrator_text_stream`: Iterates through a source buffer in a0, performing bitwise transformations on a destination buffer in a1 based on control characters and lookup table values.
- `process_encoded_text_stream`: Parses a stream of bytes from a source buffer, using a lookup table to determine bitwise transformations and applying bitwise OR operations to a destination buffer.
- `calculate_offset_and_branch_2`: Subtracts a 16-bit value from d2 from d0, stores the result in d5, decrements d5 by 1, and branches to another routine.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `patch_string_data_via_buffer`: Iterates through a buffer, comparing bytes against a target string and checking specific bit flags.
- `compare_and_patch_string_data`: Compares bytes from a source buffer against a target string, applying specific byte updates or skips based on bit flags in the buffer.
- `increment_and_branch_on_byte`: Increments d1 by 1, loads a byte from offset 6 of a structure in a0 into d3, and branches to another routine if d3 is not 255; otherwise it calls another routine and returns if the overflow flag is not set.
- `test_bit_and_branch`: Tests bit 5 of a byte at offset 3 from a pointer in a0 and branches to another routine based on the result.
- `test_bit_with_table_lookup`: Loops reading a byte from a buffer and a word from a table indexed by a word, modifies the word based on bits in the byte, and exits when a specific bit is set or a termination condition is met.
- `check_word_limit`: Saves registers a0 through a3, loads a 16-bit value from offset 154 of a structure in a5, and branches to another routine if the value is less than 512; otherwise it sets the carry flag and branches to a different location.
- `shift_bytes_and_clear_fields`: Increments a word counter in a structure and shifts bytes backward in three buffers by a computed count, then writes a byte from a register and two zero bytes to the new ends of the buffers.
- `call_multiple_subroutines`: Calls seven different subroutines in sequence and then clears a 16-bit word in d0 before returning.
- `clear_byte_at_offset_2`: Loads a pointer from offset 40 of a structure in a5 into a6, compares the byte at the address in a6 to 255, and if they are not equal, loads the pointer from offset 40 of a5 into a6 again and writes 160 to offset 7 of the structure in a6 before returning.
- `clear_byte_and_advance`: Clears a byte at offset 7 of a structure in a6, increments a6 by 8, and branches to another routine.
- `check_stream_byte`: Loads three pointers from a structure in a5 and a global pointer into a0, a1, a2, and a4, then checks if the byte at the first pointer is equal to 255; otherwise it returns.
- `process_data_stream_3`: Performs complex bitwise manipulations and data transfers between three pointers in a0, a1, and a2 based on values read from a table, and calls another routine to increment those pointers.
- `process_data_stream_2`: Processes a stream of data by reading bytes from a source, performing bitwise operations based on lookups in a table, and writing results to a destination.
- `process_data_stream`: Processes a stream of data by reading bytes from a source, performing bitwise operations based on lookups in a table, and writing results to multiple destination pointers in a loop until a byte containing 255 is encountered.
- `test_and_set_bit_6`: Tests bit 6 of a byte in register d1; if the bit is set, it sets bit 6 of a byte at the address in a2 and branches to another routine, otherwise it branches to another routine.
- `set_bit_and_branch_3`: Tests bit 5 of d1 and branches to another routine if the bit is clear; otherwise it sets bit 2 of a byte at the address in a2 and branches to another routine.
- `set_bit_and_increment`: Sets bit 2 of a byte at a pointer in a2, increments a2 by 1, and branches to another routine.
- `increment_three_pointers`: Increments three pointers in a0, a1, and a2 by 1 and branches to another routine.
- `calculate_masked_value`: Loads a byte from an offset of 1258 in a structure into d2, compares it to 255 and branches to another routine if they are equal; otherwise it masks d2 with 63, adds it to d0, and branches to another routine.
- `allocate_memory_and_update_structure`: Stores a 32-bit value from d0 into offset 58 of a structure in a5, calculates a new value by shifting d0 left by 3 and adding 8, stores this new value into offset 54 of the structure in a5, calls a routine via a pointer in a6, and if d0 is not zero, sets the negative flag and increments a7 before returning.
- `update_structure_from_call`: Stores a 32-bit value from d0 into offset 40 of a structure in a5, checks a byte at offset 219 of that structure, and if it is non-zero, loads a 32-bit value from offset 58 of the structure into d0, calls another routine, and if d0 is non-zero, stores d0 into offset 44 of the structure before returning.
- `expand_narrator_command_stream_data`: Iterates through a source buffer of command bytes, fetching structured data from a table based on each byte, applying character offsets or bitwise transformations, and writing the resulting 8-byte data blocks into a destination buffer.
- `expand_narrator_command_stream`: Reads a sequence of bytes from a source buffer and expands each byte into an 8-byte structured data stream written to a destination buffer.
- `parse_narrator_command_stream`: Reads a sequence of bytes from a source buffer to expand them into an 8-byte structured data stream, where each entry is composed of several bytes and three high-order bytes derived from a lookup table.
- `interpolate_buffer_data_3`: Loops through a buffer, performing linear interpolation on bytes using values from a global table and a local index, and calls another routine.
- `check_and_increment_pointer`: Shifts a word in d0 left by 2, loads a longword from a table using the result as an offset, and if bit 21 of that longword is set, increments the pointer in a0 and branches back to the start of the routine.
- `interpolate_buffer_data_2`: Performs a weighted interpolation of bytes between two source locations and writes the result to a destination buffer using a loop controlled by a value in a register.
- `interpolate_buffer_data`: Performs a weighted interpolation of bytes between two source locations and writes the result to a destination buffer using a loop controlled by a counter in a register.
- `increment_a6_and_branch`: Increments the value in a6 by 1 and branches to another location.
- `process_narrator_data_table_entries`: Iterates through a data table, performing comparisons between values retrieved from a global table using indices found in the data, and updates a structure pointed to by a pointer in a structure in a5.
- `process_narrator_data_table_2`: Loads a byte from a table using a6 and d7, loads a byte from a1 and increments a1, masks the byte in d0, shifts d7 left by 2, loads a longword from a table using a4 and d7, tests bit 10 of that longword, tests bit 11, and if bit 11 is set it adds 1 to a0 before looping back to an earlier instruction.
- `process_narrator_data_table`: Iterates through a data structure to perform table-based lookups and comparisons, then clears or initializes segments of a memory buffer based on the results.
- `fetch_table_values`: Loads a byte from a0 into d7, uses it to fetch a byte from a table in a4 into d2, compares d5 and d6, and if d6 is not greater than or equal to d5, it loads a byte from a0 into d7 and a byte from the table in a4 into d3 before branching.
- `process_narrator_data_stream_2_2`: Iterates through a data stream to perform arithmetic comparisons between bytes and updates a table of values.
- `process_narrator_data_stream_2`: Iterates through a data stream to perform arithmetic comparisons between bytes and updates a table of values.
- `calculate_byte_difference`: Subtracts the values of d2 and d3 from the byte in d0, stores the result in d4, and then moves the byte from d2 into d1 before branching to another routine.
- `adjust_pointer_and_loop_2`: Clears a word in a6, clears a byte at offset 2 of a6, tests bit 31 of d0, and if bit 31 is not set, copies d6 to offsets 3 and 4 of a6, increments a6 by 8, and enters a loop using d1 as a counter; otherwise it moves d4 to d1 and branches.
- `adjust_pointer_and_loop`: Increments a pointer in a6 by 8, sets a loop counter in d1 using a byte from d3, and branches to another routine.
- `clear_and_copy_data`: Clears a 16-bit word at the address in a6 and a byte at offset 2 of the structure in a6, then if bit 31 of d5 is not set, copies the byte in d6 to offset 3 and the word in d6 to offset 4 of the structure in a6.
- `clear_registers_and_branch_2`: Clears d1, d3, and d4 by moving zero into them, moves the byte from d0 into d1, and then branches to another routine.
- `filter_and_copy_bytes`: Loops through a buffer, checks a bitmask in a global table for each byte, and if the bit is not set, copies the byte and the corresponding words from two other buffers into destination buffers until a termination byte is found.
- `write_byte_to_three_pointers`: Writes the same byte from a register to the memory locations pointed to by three registers.
- `call_subroutines`: Calls two consecutive subroutines and then returns.
- `mask_byte_in_structure`: Compares a byte at offset 616 of a structure to 255; if they are equal, it branches to another routine, otherwise it performs a bitwise AND on that byte with 240 and branches to another routine.
- `check_byte_value_2`: Loads a byte from an offset of 232 in the structure pointed to by a5 into d0, compares it to 255, and branches to another routine if the value is not 255; otherwise it returns.
- `update_linked_list_nodes`: Iterates through a data stream, performing lookups in a table and updating bits in destination bytes based on the values found, continuing until a byte containing 255 is encountered.
- `process_linked_list_2`: Sets bit 0 of a byte at offset minus 1 of a1, tests bit 7 of d1, and if bit 7 is set, returns; otherwise it compares a byte from a0 to constants 24 and 23, and if neither matches, sets bit 1 of a byte at a1 and moves a1 to a3.
- `process_linked_list`: Iterates through a list of bytes, performing bitwise operations on values at addresses derived from a table, and sets bit 2 of a byte at a specific location before returning.
- `set_bit_and_branch`: Sets bit 2 of a byte at the address in a3 and branches to another location.
- `set_multiple_bits`: Sets bit 0 of the byte at offset -2 of the address in a1, and sets bits 1 and 2 of the byte at offset -1 of the address in a1, then branches to another location.
- `decode_variable_length_audio_stream_2`: Decodes a bitstream from a source buffer into a destination buffer by performing divisions of a calculated constant by values read from a control structure.
- `process_bit_flag`: Masks d0 with 63, tests bit 0 of a byte at the address in a2 and increments a2, and branches to another routine if the bit was set; otherwise it shifts d0 left by 3, adds the result to a6, and branches to an earlier instruction in this routine.
- `decode_variable_length_audio_stream`: Decodes a variable-length bitstream by reading bit-packed values and calculating sample amplitudes through division and interpolation.
- `decode_variable_length_stream_2`: Loads a byte from a1 and increments a1, masks the byte with 63, tests bit 1 of a byte at a2 and increments a2, and if bit 1 is set, branches; otherwise it shifts the value left by 3, adds it to d1, and loops back.
- `decode_variable_length_stream`: Decodes a bitstream using variable-length encoding by reading bytes and applying bitwise masks to calculate offsets and values.
- `divide_and_update_pointer`: Swaps the high and low words of d5, adds d6 to d5, divides d5 by the 16-bit word at offset 32 of the structure in a5, stores the resulting byte in d7 at offset -1 of the structure in a6, decrements a1 and a2 by 1, and branches to another location.
- `divide_and_store_byte`: Loads a pointer and a word from a structure, divides the word by 110, and if the result is not 255, stores the result in a byte at an offset from the pointer before branching back.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `parse_narrator_token_stream_3`: Iterates through three parallel data streams, using a byte from the first stream as an index into a table to fetch a longword, then performs multiple conditional checks on the longword bits and the values in the other two streams to decide whether to call another routine and advance the pointers.
- `test_bits_and_branch`: Shifts a word left by two, loads a longword from a table using the result as an offset, and performs bit tests to decide whether to branch or move a constant into a register.
- `increment_three_pointers_2`: Increments a1, a0, and a3 by 1 and branches to another location.
- `parse_text_stream_control_bits`: Iterates through a byte stream and performs multiple conditional checks on bits within registers and table lookups to determine the next execution path, calling process_text_stream_with_scaling, compute_scaled_shifted_product, or test_bits_and_call based on the results.
- `test_bits_and_call`: Tests bit 12 of d7 and branches to another location if it is not set; otherwise it tests bit 9 of d7 and branches to another location if it is not set; otherwise it loads 38 into d1, calls another routine, and branches to another location.
- `test_bit_and_call_routine`: Tests bit 4 of d7; if it is not set, it branches to another routine, otherwise it loads 38 into d1 and calls another routine before branching to a third location.
- `test_bit_and_branch_4`: Tests bit 4 of d7 and bit 2 of the byte at offset 1 of a3; if bit 4 of d7 is set and bit 2 of the byte is set, it loads 27 into d1, calls another routine, and branches; otherwise it branches to a different location.
- `test_bit_and_dispatch_text`: Tests bit 11 of a register and, if not set, loads the value 22 into a register, calls process_text_stream_with_scaling, and then branches to another routine; otherwise it returns.
- `process_text_stream_with_scaling`: Iterates through a byte stream, performing lookups in tables to determine character values and applying a scaled, shifted product calculation to modify those values.
- `test_status_bits`: Tests bit 1 of d6 and d7, and if both are set, loads 16 into d1 and calls another routine before branching to a different location; otherwise it branches to one of two other locations.
- `decode_text_stream_2`: Sets d1 to 22 and calls another routine, then branches to another location.
- `decode_text_stream`: Reads a stream of bytes from a source pointer, performs bitwise transformations and arithmetic based on values retrieved from a lookup table, and writes the resulting characters to a destination buffer.
- `compute_scaled_shifted_product`: The routine multiplies the low word of an input register by a word input and returns a combined value formed from the high word of the product and the low word shifted right by five bits after adding a constant.
- `initialize_and_clear_buffers`: The routine stores several offsets relative to a base pointer into a global structure, increments a few of those pointers, clears a set of word fields, then zeroes six blocks of memory by iterating a fixed count.
- `increment_and_call_handlers`: Increments a 16-bit word at offset 136 of a structure in a5, then calls five different subroutines based on the zero and overflow flags before returning.
- `process_compressed_stream_data_2`: Processes a compressed stream by reading bytes from two source pointers, performing arithmetic transformations involving scaling and bitwise operations, and writing results to a destination buffer until a termination byte or a counter limit is reached.
- `process_compressed_stream_data`: Checks bit 7 of a byte in d1, tests bit 7 of a byte at offset minus 1 from a4 plus d4, and tests bit 5 of d1.
- `decode_compressed_stream`: Decodes a stream of data by reading bytes from a source buffer and using them as indices into a lookup table to produce output bytes.
- `set_flag_and_store_indices`: The routine ORs a constant into a byte addressed by a base pointer and an index, then stores two short values derived from an index and a conditional copy into fields of a global structure.
- `process_narrator_text_stream_3`: Iterates through three synchronized buffers to parse control codes and update a text buffer and a secondary data buffer.
- `process_data_flags`: Loads a word from a table using a shifted value in d3, checks bit 13 of that word, and if set, branches to another routine; otherwise, it checks if d1 is negative, increments d4 if it is positive, checks bit 5 of d2, and if clear, writes the value 2 to an offset in a structure in a6 before branching to another routine.
- `modify_byte_via_flags`: Checks bit 4 of a byte at an offset in a structure in a6, and if it is not zero, performs bitwise operations to modify that byte before branching to another routine.
- `decrement_and_store_pointers`: Decrements three pointers in a0, a1, and a2 by 1, stores these pointers at offsets 124, 132, and 128 of a structure in a5, and branches to another location after moving the word in d4 to d0.
- `update_status_byte`: Ors a byte from d7 into a byte at offset d0 of a4, moves d0 to d2, and compares a byte at offset d0 of a6 with 2 or 14.
- `clear_byte_on_condition`: Checks bit 5 of a byte at an offset in a4, and if it is zero, compares a value in d2 to d4 and clears a byte at an offset in a6 if d2 is less than d4.
- `swap_and_clear_byte_flags`: Swaps a byte between two positions in a table addressed by a base pointer and an index, clears the source byte when the positions differ, and then decrements an index using a loop.
- `prepare_buffer_pointer`: Loads a pointer from an offset in a5 into a1, moves a value from d4 to d0, and branches to another routine.
- `set_bits_in_loop`: Loops through a buffer and sets the lowest bit of each byte using a counter in d0.
- `or_mask_into_buffer_with_early_exit`: The routine selects a mask based on a control byte, then ORs that mask into a series of bytes in one buffer while checking a flag in another buffer and returning early if the flag is set.
- `set_tile_and_mask`: Updates a byte in a screen buffer using a mask derived from a map byte, and optionally masks and sets a flag in another byte based on a scan leftwards in a second buffer.
- `accumulate_table_entries`: Reads a 16-bit word from a structure, conditionally calls other routines, and then adds that word to eight consecutive longwords in a table pointed to by the structure.
- `clamp_value_and_store_char_2`: Calculates a value based on several arithmetic operations and clamps it between 125 and 165, then stores the resulting byte at an address formed from a base pointer and an offset.
- `clamp_value_and_store_char`: The routine clamps a short input to a lower and upper bound and stores the resulting byte at an address formed from a base pointer and an index.
- `adjust_table_values_loop_2`: Performs a multi-pass mathematical transformation on a destination table based on values from two source tables.
- `adjust_table_values_loop`: The routine repeatedly computes an absolute difference between two table entries, scales it and adds the result to a destination byte, then scans backwards through a range using flag bits to locate entries.
- `compute_scaled_char_values_2`: Iterates through a table using a counter, performing complex scaling and arithmetic on bytes retrieved from multiple source tables and writing the results to destination tables based on bit flags.
- `scale_character_value`: Adds 64 to a value in d2, shifts it right by 7, and adds the result to a byte at an offset in a structure in a3.
- `compute_scaled_char_values`: The routine loops over a range of indices, using flag bits to select entries and computing scaled character values from two tables with multiplication and shifts.
- `load_pointers_and_branch`: Loads two pointers from offsets 104 and 112 of a structure in a5 into a0 and a1, moves a word from offset 158 of the structure into d1 and d0, and then branches to another routine.
- `process_audio_filter_coefficients`: Iterates through a data structure to apply scaling and offset adjustments to bytes in three separate buffers, performing sign-clamping and bit-shifting to update values based on distance and specific bit flags.
- `calculate_scaled_value_4`: Adds 64 to d2, shifts it right by 7, and adds the result to a byte at an offset in a3; then it loads a byte from a3 at offset d1 into d2, sign-extends it, multiplies it by 65485, and branches based on the sign, potentially subtracting 64 before branching to another routine.
- `transform_and_add_value`: Adds 64 to d2, shifts it right by 7, and adds the result to a byte at offset d1 of a table in a3; then it loads a byte from offset d0 of a structure in a6 into d2, subtracts 110, multiplies d2 by -30, and branches to another location based on the sign of the result.
- `update_table_byte_2`: Adds 64 to d2, shifts it right by 7, and adds the result to a byte at offset d0 of a6, then loads a byte from offset d1 of a6 into d2, subtracts 110, multiplies d2 by 26, and branches based on the sign.
- `update_table_byte`: Adds 64 to d2, shifts it right by 7, and adds the result to a byte at an offset in a6.
- `add_byte_to_table`: Adds a byte from d2 to a byte at an offset in a2 and branches to another routine.
- `apply_audio_gain_and_filter`: Applies variable gain coefficients to bytes in two buffers based on a control value in a register, performs sign-clamping, and executes a loop that processes audio samples by applying bitmask-based conditional transformations and scaling values from a source buffer into a destination buffer.
- `transform_audio_buffer_data_2_2`: Adds 64 to d2, shifts it right by 7, and adds the result to a byte at offset d0 of a3, then loads a byte from offset d1 of a3 into d2, extends it to a word, multiplies it by d7, and branches based on the sign.
- `transform_audio_buffer_data_2`: Iterates through a data buffer using nested loops to perform arithmetic transformations on bytes based on bitmask checks, applying scaling and offset adjustments to the values.
- `transform_audio_buffer_data`: Iterates through a data buffer using nested loops to perform arithmetic transformations on bytes based on bitmask checks, applying scaling and offsets to values stored in a table.
- `apply_audio_filter_scaling`: Iterates through a data structure to apply scaling and offset adjustments to bytes in three separate buffers, performing sign-clamping and bitwise checks on control bytes.
- `calculate_scaled_value_3`: Loads a byte from a table at offset d0 of a2, sign-extends it to a word, multiplies it by 102, and then either subtracts 64 or branches to another routine based on the sign of the result.
- `update_destination_byte`: Adds a value derived from a 16-bit register shifted right by 7 to a byte at a destination pointed to by a3 and indexed by d0, then branches to another routine.
- `adjust_tile_heights_with_mask`: Iterates over a byte range using a mask in a global variable and a table pointed to by a register, updates a byte in a destination buffer from a source buffer with a scaled difference based on a running maximum, and writes an accumulated scaled value to another buffer.
- `load_buffer_pointers`: Loads two pointers from offsets 104 and 100 of a structure in a5 into a2 and a0, clears d1, and moves the value in d0 to d0 before branching to another routine.
- `modify_buffer_with_mask`: Iterates through a buffer backwards, and for each byte where the high nibble is non-zero, adds a value to the byte at the destination pointer based on whether the high bit is set and the value of a byte from a source pointer.
- `scale_buffer_by_signed_factor`: The routine walks a byte range backwards, skipping bytes whose high nibble is zero, and adds a shifted accumulator to a destination byte.
- `process_audio_buffer_stream_2`: Iterates through a data stream to perform bitwise manipulations and byte-wise arithmetic on multiple buffers, including copying bytes and adding values to a buffer at offset 96 of a structure in a5.
- `copy_and_increment_bytes`: Iterates through a loop, copying a byte from one location to another, performing arithmetic on the destination byte, and updating a byte in a third location using a value from a global variable.
- `process_audio_buffer_stream`: Iterates through a data stream to perform bitwise manipulations and byte-wise arithmetic on multiple buffers, including copying bytes and adjusting values based on nibble-derived calculations and bitwise checks.
- `update_buffer_with_shifted_nibble`: Iterates through a loop, copying a byte from a source buffer to a destination buffer, and adding a value to a byte in a third buffer derived from a masked and shifted nibble from a fourth buffer.
- `load_address_and_call_2`: Loads the address stored at a longword at offset 10708 into a4, calls another routine, and then branches to another location.
- `load_address_and_branch_7`: Loads a 32-bit constant into a4, calls another routine, and then branches to another location.
- `load_pointers_and_constants`: Loads a pointer from an offset in a5 into a4, sets d6 to 110, and moves the value in d5 into d0 before branching to another routine.
- `normalize_and_scale_audio_buffer`: Iterates through a buffer to perform bitwise transformations and data copies, then checks a bit in a control byte to either calculate a mean offset for a sequence of bytes or find a maximum value to scale a single byte.
- `process_buffer_data`: Decrements a counter in d0 while checking a bit in a buffer, then performs a series of calculations and data movements between buffers in a0, a1, and a6 based on the value of d2 and d4.
- `process_buffer_and_calculate_max`: Copies bytes between two buffers while subtracting a value from each, then if a specific byte is a backspace, it finds the maximum byte in a third buffer, calculates a scaled value, and writes it to multiple destinations.
- `initialize_registers_and_branch`: Clears registers d7 and d1, moves a 16-bit value from d4 to d0, and branches to another location.
- `accumulate_bytes_and_write_results`: The routine accumulates bytes from a source area into a register, keeps a running maximum, derives a scaled character value, and writes that value and a derived value to several destination buffers with a conditional copy.
- `decrement_pointers_and_branch`: Loads three pointers from offsets 124, 128, and 132 of a structure in a5 into a0, a1, and a2, decrements each pointer by 1, sets d2 to 0, moves a word from d4 into d0, and branches to another routine.
- `apply_text_scaling_adjustments_2`: Iterates through a text buffer in a loop, performing arithmetic scaling and bitwise adjustments on bytes using values from a table and multiple pointers.
- `scale_offset_value`: Loads a pointer from an offset in a5 into a6, loads a byte from a6 at offset d0 into d1, subtracts 110 from d1, multiplies d1 by 26, and branches based on the sign, potentially subtracting 64 before branching to another routine.
- `apply_text_scaling_adjustments`: Iterates through a text buffer, performing arithmetic scaling and bitwise adjustments on bytes using values from a table and multiple pointers stored in a structure.
- `process_narrator_text_buffer`: Iterates through a buffer to perform arithmetic operations on bytes, applying scaling factors and bitwise adjustments based on values found in a lookup table.
- `compare_and_set_register`: Compares d0 with d7 decremented by 1, and if they are not equal, sets d7 to 64 and branches to another location.
- `set_register_and_branch`: Sets d7 to 86 and branches to another location.
- `calculate_scaled_value`: Loads a 16-bit constant, retrieves pointers from a table, loads a byte from a source buffer, extends it to a word, and multiplies it by the constant, then conditionally subtracts a value if the result is negative.
- `adjust_attribute_buffers_via_scaling`: Adds 64 to a register, performs an arithmetic shift right by 7, then adds and subtracts the resulting byte to two different buffers at offsets determined by a register and a structure pointer in a6 and a3, looping until a counter reaches zero.
- `branch_to_offset_52`: Performs a short branch to an instruction located 52 bytes ahead in the code segment.
- `copy_and_adjust_bytes_with_feedback`: The routine copies bytes between buffers, updates a feedback value by multiplying it with a table entry and shifting, subtracts the feedback from a byte, and writes adjusted bytes to two output areas.
- `check_narrator_flag_and_dispatch`: Calls several other routines and then checks if a byte at offset 219 of a structure in a5 is non-zero; if it is, it calls another routine before returning.
- `call_check_buffer_byte`: Loads a pointer from a global variable, adds 3, and calls another routine; repeats this by adding 4 and 5 to the same base pointer before returning.
- `interpolate_and_store_bytes`: The routine reads two bytes from a buffer, computes a scaled difference based on an index, accumulates the result, and stores successive byte results into the buffer at regular intervals.
- `check_buffer_byte`: Increments a word in d1 by 8, loads a byte from the address in a0 plus the offset in d1 into d6, and returns if d6 is not 255.
- `scan_buffer_for_sentinels_2`: Checks if d6 is 254, and if not, checks if d1 is 8; if d1 is 8, it increments a0 by 8 and branches to another location.
- `scan_buffer_for_sentinels`: Iterates through a buffer in increments of 8 bytes, checking for specific byte values.
- `test_buffer_byte`: Increments a word in d1 by 8, tests the byte at the address in a0 plus the offset in d1, and returns if that byte is 0 or 255.
- `increment_buffer_pointer`: Checks if d1 is 8, and if so, increments a0 by 8 and branches to another location.
- `scan_buffer_for_sentinel`: Iterates through a buffer in increments of 8 bytes, searching for a null terminator or a byte with the value negative one.
- `process_string_buffer`: Performs a loop that reads values from a table, modifies a pointer in a0 based on a bitwise mask, and either increments bytes in a buffer or calls process_buffer_and_strings.
- `process_buffer_and_strings`: Performs a loop that reads values from a table, modifies a pointer in a0 based on a bitwise mask, and either increments bytes in a buffer or copies characters from a source table into a destination buffer based on specific bit patterns and character values.
- `offset_destination_pointer`: Shifts a 16-bit value in d0 left by three bits, adds the result to the pointer in a0, and branches to another location.
- `modify_register_value`: Shifts the value in d3 right by 2 bits, compares it to 42, and if equal, sets d3 to 96 before branching.
- `update_register_value`: Compares d3 to 43 and, if equal, sets d3 to 97 before branching to another location; otherwise it branches to another location.
- `modify_byte_on_match`: Compares d3 to 44, and if it is not equal, branches to another location; otherwise it sets d3 to 98 and branches to another location.
- `copy_bytes_from_offset_table`: Copies six bytes from a table located at an offset determined by a byte in a register to a destination pointed to by a register, then increments the destination pointer by two and repeats this process for a variable number of iterations based on a value in a register.
- `initialize_address_registers`: Loads four different addresses into a0, a1, a2, and a4 from offsets in a5 and a constant, moves a pointer into a6, and sets d6, d4, d5, and d7 to specific constant values before branching to another routine.
- `set_bit_and_compare`: Sets bit 31 of d4, clears d0 and d2, loads a byte from the address in a0 into d0, and returns if d0 is not equal to d5.
- `process_narrator_stream_data`: Iterates through a data stream, performing block memory copies of 8-byte chunks if a control byte is 1 or 2, otherwise it performs bitwise masking and logical OR operations on specific bytes within a structure pointed to by a6 based on values derived from the stream and a lookup table.
- `process_narrator_data_stream`: Iterates through a data stream, performing bitwise masking and logical operations on bytes within a structure pointed to by a6 based on values read from a0 and a1.
- `calculate_array_index_offset`: Shifts the value in register d2 left by three bits and adds the result to the address in register a6, then branches to another routine.
- `replace_bytes_via_lookup`: Loops through a buffer, replacing bytes with values from a global table based on the byte's value, or skipping eight bytes if the byte is 126.
- `copy_bytes_with_lookup`: Copies bytes from a source pointed to by a0 to a destination using a lookup table in a1 and d0, then loops back to the start of the routine.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `compare_scaled_byte`: Loads a byte from a structure at offset 48 of a6 using a shifted value in d2, compares it to 255, and branches if they are not equal.
- `update_scaled_value`: Adds a value from d3 to d0, performs a bitwise shift, and stores the result in a byte at offset 24 of the structure in a6 within a loop.
- `compare_table_byte_with_constant`: Loads a byte from a table at an offset calculated from a register into a register and compares it against a constant value of 255, otherwise it returns.
- `process_data_table`: Iterates through a table using a counter in d1, accumulating values into d0 and updating a byte at offset 24 of a structure in a6 every 7 iterations based on a calculation involving other table values.
- `load_stream_offsets`: Loads four pointers into a0, a2, a4, and a6 from offsets of a structure in a5, clears d2 and d0, loads a byte from the address in a0 into d0, and returns if d0 is 255.
- `process_narrator_text_stream_2`: Iterates through a byte stream, performing arithmetic operations on a byte at offset 7 of a structure pointed to by a6 based on character values and bitwise flags found in the stream.
- `add_constant_to_offset`: Adds the constant 10 to a byte at offset 7 of the structure in a6 and loops based on a counter in d2.
- `load_constant_d3`: Loads the constant 250 into d3 and branches to another routine.
- `load_negative_constant_and_branch`: Loads the constant negative 6 in register d3 and branches to an instruction within another routine.
- `load_negative_constant_and_branch_2`: Loads the constant negative 6 for the value in register d3 and branches to a location within another routine.
- `store_fixed_byte`: Stores the byte 230 at offset 7 of the structure in a6 and loops based on a counter in d2.
- `modify_byte_by_offset`: Subtracts 43 from a byte at the current a6 pointer, shifts it right by 2 bits, adds the result to a byte at offset 7 of the structure in a6, and loops based on a counter in d2.
- `decode_nibble_checksum_bytes`: The routine reads a pointer and a count from a structure pointed to by A5 and processes a byte array in two passes.
- `update_device_status_3`: Calls three different routines via pointers in a global variable and a structure, stores the results in the structure at offsets 30, 31, and 308, and then enters an infinite loop that calls manage_device_status and another routine.
- `manage_device_status`: Enters a loop that checks bit 6 of a byte at an offset in a structure, sets bit 0 of that byte, and if bit 0 was not already set, calls another routine; if that routine returns zero, it clears bits 0 and 1 of the byte and returns, otherwise it sets bit 1 and calls itself recursively.
- `clear_status_bits`: Clears bit 0 and bit 1 of a byte at an offset of 34 from a pointer in a2.
- `prepare_speech_io_request`: Initializes a series of structures by copying data from a table, calculates a timing value based on a division, and sets flags in a speech IORB. It then performs a loop to copy a block of data, another loop to populate a buffer with a specific value, and a nested loop to set bits in a buffer based on a bitmask before calling update_narrator_display_state.
- `update_narrator_display_state`: Copies a block of data, performs bitwise operations to set flags in a buffer, and updates several fields in a structure including a calculated value at offset 1e.
- `decode_narrator_command_stream_data`: Iterates through a command stream to perform bitwise manipulations and arithmetic on values, updating a destination buffer and decrementing counters within a structure.
- `parse_narrator_command_stream_2`: Iterates through a data stream to decode command bytes, updating a structure's internal state and performing bitwise manipulations on values derived from the stream.
- `process_narrator_command_stream_data`: Iterates through a command stream to perform bitwise manipulations and arithmetic on values, updating a destination buffer and decrementing counters.
- `decode_narrator_command_stream`: Iterates through a data stream to decode command bytes, updating a structure's internal state and performing bitwise manipulations on values.
- `process_narrator_command_stream`: Iterates through a data stream to decode command bytes, updating a structure's internal state and performing bitwise manipulations on values, then calls parse_narrator_command_stream_2 and branch_on_d0_sign.
- `branch_on_d0_sign`: Subtracts 1 from d0, branches if the result is positive, otherwise it writes d5 to offset 16 of a5 and branches to another routine.
- `execute_callback_loop`: Decrements a value from a structure, calls a function through a pointer in a global variable within a loop, and conditionally writes a value to a hardware register.
- `enable_dma_if_flag_set`: The routine tests a flag byte in a structure pointed to by A5 and, if set, writes a value to a custom hardware register that enables DMA channels by setting bit 15 of DMACON.
- `process_interrupt_logic`: Compares a byte from a structure to another byte, and if they differ, performs several checks and updates a linked list and a counter before calling another routine.
- `decrement_counter_and_enable_interrupts_2`: The routine reads a byte from an offset in a structure pointed to by A6, decrements it and writes it back.
- `calculate_address_offset`: Loads a byte from an offset of a3 into d2, copies d2 to d7, multiplies d7 by 68, checks a byte at an offset of a5, and if zero, calculates a new address in a2 by adding d7 to a pointer from a structure in a4.
- `execute_callback_loop_3`: Sets bit 0 of a byte at offset 30 of a structure, then enters a loop that calls a function pointer from a table and increments a pointer by 68 bytes until a counter reaches negative one.
- `execute_callback_loop_2`: Loops through a series of function pointers, checking a byte at offset 30 of a3 to decide whether to return or continue, and modifies a hardware register and a value at offset 22 of a3.
- `set_bit_and_branch_2`: Sets bit 5 of a byte at offset 30 of a structure in a3 and then branches to another routine.

### Qume_LetterPro_20

- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `execute_library_calls_3`: Calls a function pointer from a global variable three times, returning 0 if all calls succeed or a specific error value if they fail.
- `call_routine_thrice`: Calls another routine three times and returns -1.
- `call_handler_triple`: Calls the same routine three times using pointers retrieved from a table.
- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_status_word`: Clears registers d1, d2, and d3, loads three pointers from the stack into a0, a1, and a4, compares the word at the pointer in a0 to 1, and branches to a different location if the comparison fails or otherwise performs a short branch.
- `parse_printer_status_and_format_output`: Copies a sequence of bytes from a table into a destination structure, then checks specific word values at offsets 2650 and 2646 of a pointer to set control bytes.
- `copy_byte_loop`: Copies bytes from a table at offset 0 to a destination at offset 1 of a structure in a3, using a loop that runs up to 11 times.
- `update_conditional_word_3`: Compares a word at a pointer to a constant and checks if a byte at another pointer is negative; if both conditions are met, it writes a word and a byte to those locations and branches.
- `update_conditional_word_2`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is negative, it writes 32 to the word at the address in a0 and branches to another routine.
- `update_status_value_29`: Compares a word at the address in a0 to 28; if it matches and the byte at the address in a1 is positive, it writes the word 33 to the address in a0, otherwise it returns.
- `update_status_values_2`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is positive, it writes the word 33 to the address in a0, writes the byte 255 to the address in a1, and branches to another routine.
- `update_status_flag_on_match`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is positive, it writes the word 2 to the address in a0, otherwise it returns.
- `compare_word_and_byte`: Compares a word at the address in a0 to 30; if it is not equal or if the byte at the address in a1 is not negative, it branches to another routine, otherwise it writes 32 to the address in a0 and branches to another routine.
- `update_word_and_byte_3`: Compares a word at the address in a0 to 31.
- `increment_byte_on_mismatch`: Compares a word at the address in a0 to 32, and if not equal, increments a byte at the address in a1 and branches.
- `decrement_and_branch_2`: Compares a word at the address in a0 to 33, and if it is not equal, decrements the byte at the address in a1 and branches.
- `format_numeric_string`: If a word at a0 is 57, it writes two bytes and two calculated character bytes to a destination; otherwise, if the word is 0, it writes 253 to offset 2720 of a structure in d1.
- `write_offset_byte`: Writes a byte to a destination at offset 1 of a structure in a3, where the byte value is the result of adding 48 to a sign-extended word in d0.
- `write_status_byte_3`: Checks if a word at the address in a0 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in d1, then returns.

### CBM_MPS1000

- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_conditional_call`: Calls three different routines via offsets from two different global pointers, only executing subsequent calls if the previous one returns a non-zero value, and calls a cleanup routine if any call fails.
- `execute_dual_calls`: Calls another routine twice using different addresses stored in memory and then branches.
- `execute_handler_triple`: Calls a routine at an offset from a pointer in a global variable, then calls that same routine twice more using a value from a6 as an argument.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry`: Checks a condition via a routine call, and if met, populates a structure with two parameters and executes several routines through a global base.
- `manage_printer_buffer_state`: Acts as a multi-purpose state machine that manages a memory buffer by performing operations such as clearing memory with a loop, appending escape sequences and data, bitwise ORing values into the buffer, or calculating offsets.
- `clear_d0_and_return`: Sets d0 to zero and returns.
- `initialize_printer_state_2`: Saves registers to the stack, loads several pointers from the stack into a2, a3, and a4, clears d1, d4, and d5, sets d3 to 3, and branches to another routine if the word at the address in a2 is 1.
- `build_printer_command_string`: Constructs a command string by copying data from a table and appending values based on checks of a structure pointed to by a register.
- `modify_memory_byte`: Loads a pointer from the address in d1, accesses a value at offset 134 of that pointer, and if it equals 2048, increments d4, adds the result to a0 and a4, and writes the byte 15 to the resulting address before branching to another routine; otherwise it loads a value from offset 296 into d6 and branches.
- `process_command_and_update_state`: Copies a string from a source to a destination using a loop, then performs conditional logic based on a value at a specific memory location to update a global variable, modify a structure, or toggle states in a buffer.
- `copy_bytes_from_zero_offset`: Copies up to 4 bytes from a table at offset 0 to a destination at offset 1 of a structure in a0, using a loop.
- `check_word_and_modify_byte`: Compares a word at the address in a2 to 66; if equal, it sets a longword at offset 2 to 1, loads a pointer from d1 into a0, loads a pointer from the address in a0 into a1, compares a word at offset 134 of the address in a1 to 2048, and if equal, writes 96 to the address in d3 before branching to another routine.
- `set_byte_and_branch_3`: Loads a pointer from a0, compares a word at offset 1024 of that pointer to 1024, and if they are not equal, writes the byte 137 to the address in a0 and branches.
- `set_byte_and_branch_2`: Moves the value in d3 to a0, writes the byte 80 to the address in a0, and branches to another routine.
- `copy_bytes_from_table`: Iterates through a table of bytes, copying each byte to a destination address calculated by adding an offset to a pointer in a4, and continues until four bytes have been processed.
- `modify_word_and_byte_2`: Compares a word at the address in a2 to 32.
- `clear_byte_and_move_word`: Checks the byte at the address in a3; if it is negative, it clears that byte, otherwise it branches to another routine; if it was cleared, it writes the word 30 to the address in a2 and branches to another routine.
- `set_d0_and_branch`: Sets d0 to 255 and branches to another routine.
- `update_status_bytes`: Compares a word at the address in a2 to 33, and if equal, checks a byte at the address in a3; if that byte is non-zero, it branches, otherwise it sets that byte to 255, sets the word at the address in a2 to 29, and branches.
- `clear_byte_and_branch`: Tests a byte at the address in a3, and if it is less than or equal to zero, clears that byte and writes the word 28 to the address in a2 before branching; otherwise, it branches.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `update_state_from_a2`: Compares a word in a2 against several constants to write specific bytes to a stack location or a structure, and if a2 is zero, calls write_status_byte.
- `write_status_byte`: Checks if a word at the address in a2 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in d1.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_630

- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_indirect_calls`: Calls a routine via a pointer at an offset from a global variable three times using different addresses, and if any call returns zero, it calls another routine via an offset from the global variable to clean up.
- `execute_triple_conditional_calls`: Loads a pointer from a global variable, saves a6 to the stack, calls a function at an offset from a6, restores a6 from the stack, loads a second pointer, saves a6 again, calls a second function at an offset from a6, restores a6, and finally branches.
- `execute_indirect_function_calls`: Fetches a function pointer from a global variable using a specific offset and calls that function three times in succession.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_status_check`: Loads three pointers from the stack into registers, clears three registers, and compares the word at the first pointer to one; if the value is not one, it branches to a different location, otherwise it continues to another routine.
- `parse_input_buffer_and_update_state`: Copies data from a structure into a local buffer, performs a series of conditional checks on values at specific offsets within that structure, and updates a state variable and a secondary value based on those comparisons.
- `process_control_sequence_data`: Copies a null-terminated string from a source address to a destination address, then performs a series of conditional checks and updates on a word and a byte based on specific constant values.
- `copy_table_byte_loop`: Copies a byte from a table at offset zero of a pointer in a3 to a destination address calculated from d2, d3, and a4, then loops if d0 is greater than d3.
- `update_conditional_word_and_byte`: Compares a word at the address in a0 to 27 and a byte at the address in a1 to zero; if both conditions are met, it writes 32 to the word at the address in a0 and 1 to the byte at the address in a1, then branches to another routine.
- `update_conditional_word_4`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is negative, it writes 4 to the word at the address in a0 and branches to another routine, otherwise it branches to another routine.
- `set_word_value`: Compares a word at the address in a0 to 28; if it matches and the byte at the address in a1 is positive, it writes the word 33 to the address in a0 and branches to another routine.
- `update_status_word_and_byte`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is positive, it writes the word 33 to the address in a0 and the byte 255 to the address in a1, then branches to another routine; otherwise it returns.
- `write_status_word`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is positive, it writes the word 2 to the address in a0 and branches to another routine.
- `clamp_word_value`: Compares a word at the address in a0 to 30; if the value is less than 30 and a byte at the address in a1 is negative, it writes 32 to the address in a0 and branches to another routine, otherwise it branches to another routine.
- `adjust_word_and_byte`: Compares a word at the address in a0 to 31.
- `increment_and_branch_on_mismatch`: Compares a word at the pointer in a0 to 32; if they are not equal, it increments a byte at the pointer in a1 and branches to another routine, otherwise it branches to another location.
- `decrement_byte_on_mismatch`: Compares a word at a pointer in a0 to 33; if they are not equal, it decrements a byte at a pointer in a1 and branches to another routine, otherwise it branches to another routine.
- `update_conditional_bytes`: If the word at the address in a0 is 12, it writes the byte 27 to an address calculated from a4 and d2, and then writes either the byte 65 or 66 to the next address based on a stack value; otherwise, if the word in a0 is 0, it writes the byte 253 to an offset of 2688 from a pointer in d1.
- `write_single_byte`: Writes the byte 66 to a destination address calculated by adding a value in a4 to an incremented d2, then branches to another routine.
- `update_status_byte_if_zero`: Checks if a word at the pointer in a0 is zero; if so, it writes the byte 253 to an offset of 2720 in the structure pointed to by the value in d1, then returns.

### ImagewriterII

- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_indirect_calls_2`: Calls three different routines by dereferencing offsets from a global variable and its contents, then calls a cleanup routine if any of the previous calls return non-zero.
- `execute_sequential_conditional_calls`: Loads a pointer from a global variable into a1, then calls a routine at an offset from a second global pointer, repeating this process with a different pointer in a1 and a third global pointer before returning with a negative value in d0.
- `execute_indirect_function_calls_2`: Fetches a function pointer from a location offset from a global variable and calls that function three times consecutively.
- `return_constant_20`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_21`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_2`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine through a pointer stored within that structure.
- `POSDIV`: Performs unsigned division using a loop of shifts and subtractions if a value in d2 is greater than 65535, otherwise it performs the division using a different method and returns the quotient and remainder.
- `LONGDIV`: Performs a division-like operation by rotating a 32-bit value left for sixteen iterations and subtracting a value from the high half if a condition is met.
- `ulmult`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products and combines the results.
- `ulmodt`: Moves the contents of register d1 into d2, moves the contents of register d0 into d1, and calls POSDIV before returning.
- `uldivt`: Saves d2 to the stack, moves d1 into d2 and d0 into d1, calls another routine, moves the resulting value from d1 into d0, restores d2 from the stack, and returns.
- `lmodt`: Calls POSDIV and negates the returned value if the flag left in an address register by the called routine is nonzero.
- `ldivt`: Calls POSDIV and returns its first result, or the negation of that result if the second result in an address register is nonzero.
- `dispatch_via_table`: Saves registers to the stack, retrieves a byte from the stack, and if that byte is not greater than 4, uses it to index into a table and jump to the resulting address.
- `clear_d0_and_restore_registers_alt`: Clears register d0 and restores d2 through d5 and a2 through a4 from the stack before returning.
- `initialize_printer_status_check_2`: Initializes several registers to zero, loads a pointer from the stack, and compares the word at that pointer to one; if the value is not one, it branches to another location, otherwise it continues to another routine.
- `generate_printer_escape_sequences`: Copies a sequence of bytes from a table into a destination buffer and appends various escape sequences based on values read from a structure.
- `format_epson_escape_sequences`: Translates command bytes from a pointer in a1 into specific escape sequences for a printer buffer, including converting numeric values into ASCII characters via division and modulo operations.
- `generate_epson_esc_commands`: Processes a command byte from a pointer in a1 to write specific escape sequences into a buffer.
- `modify_buffer_value`: Compares a word at the address in a1 to 31; if it matches and the byte at the address in a3 is positive, it writes 33 to the word at the address in a1 and writes 1 to the byte at the address in a3, then branches to another routine, otherwise it branches to another routine.
- `set_negative_byte`: Checks if the byte at the address in a3 is negative; if so, it writes 32 to the address in a1, writes 255 to the address in a3, and branches to another routine.
- `branch_to_offset`: Branches to a specific offset.
- `translate_imagewriter_escape_sequences_2`: Translates input characters into Apple ImageWriter II escape sequences by writing specific byte combinations to a destination buffer.
- `write_four_bytes`: Writes the bytes 66, 27, 108, and 48 to a memory location at the address formed by adding d2 and d3.
- `translate_imagewriter_escape_sequences`: Processes input bytes to generate corresponding escape sequences for an ImageWriter II printer by writing specific byte combinations to a destination buffer.
- `write_byte_and_branch_2`: Moves the value in d3 to a2, increments d3 by 1, adds d2 to a2, writes the byte 66 to the address in a2, and branches to another routine.
- `translate_imagerwriter_escape_sequences_2`: Processes input bytes to generate corresponding escape sequences for an ImageWriter II printer by writing specific byte combinations to a destination buffer.
- `write_byte_and_branch`: Sets a byte at the address calculated by adding a value in a2 to a value in d2, increments d3, and branches to another routine.
- `translate_imagerwriter_escape_sequences`: Translates specific input bytes into corresponding escape sequences for an ImageWriter II printer by writing bytes to a destination buffer.
- `update_printer_buffer_byte`: Increments a value in a register, calculates a new address by adding two registers, writes the byte seventy to that address, and branches to another routine.
- `translate_printer_escape_sequences`: Reads a byte from a pointer in a1 and writes specific escape sequences to a destination buffer pointed to by the sum of d2 and d3.
- `process_printer_escape_sequences`: Checks a byte at the pointer in a1 and, depending on its value, writes an escape character followed by a specific byte to a destination buffer at the address calculated from d2 and d3, while also updating a global variable and a value on the stack.
- `conditional_write_byte`: If a byte at a specific address is not one, it increments d3, calculates a destination address using d2 and the new d3, writes the byte 80 to that address, and branches to another routine.
- `increment_pointer_and_write_byte`: Increments a value in register d3, calculates a destination address by adding the contents of register d2 and the new value in d3 to register a2, writes the byte 112 to that address, and branches to another routine.
- `process_printer_command`: Checks a byte at the address in a1 to determine how to write escape sequences and data to a destination buffer, potentially updating a counter in a stack location and writing ASCII digits to the buffer.
- `conditional_write_byte_alternative`: If a byte at a specific address is not one, it increments d3, calculates a destination address using d2 and the new d3, writes the byte 69 to that address, and branches to another routine.
- `update_printer_status_byte`: Increments a value in a register, calculates a target address by adding two registers, and writes the byte seventy-eight to that address before branching to another routine.
- `update_printer_status_byte_2`: Checks a word at a pointer in a1 and, if it matches 12 or 48, modifies a byte at a stack pointer and writes three bytes to a memory location, where the third byte is either a decremented value or a value from a table.
- `check_and_update_status`: Compares a word at a pointer in a1 to 12; if equal, it checks a byte at a pointer from the stack, and if that byte is 39, writes 30 to that stack pointer; if the byte is 37 or greater, it loops back to the start.
- `store_three_bytes`: Stores three bytes into a memory location, where the third byte is retrieved from a global table using an offset derived from a value on the stack.
- `clear_d0_and_restore_registers`: Clears register d0 and restores d2 through d4 and a2 through a4 from the stack before returning.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_C-150

- `return_constant_22`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_callback_sequence`: Attempts to call a function via an offset of 554 from a pointer stored in a global variable up to three times, using different base addresses for each attempt, and calls a cleanup routine via an offset of 414 from the global variable if any of the calls return non-zero.
- `execute_sequential_conditional_calls_2`: Loads a pointer from a global variable and calls a routine at a negative offset from a second global pointer, then repeats this process using a different global pointer and a different argument in a1, before returning with a value of negative one in d0.
- `call_indirect_three_times`: Calls a routine through a pointer located at an offset from a global variable three times.
- `return_constant_23`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_24`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_3`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine through a pointer stored within the structure.
- `divide_32_bit_integers`: Performs 32-bit division of two values, using a loop to handle bit shifts and subtractions based on a threshold.
- `rotate_and_threshold_32bit`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to move bits between the high and low halves.
- `multiply_two_32bit_integers`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components and returns the result.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `dispatch_via_table_2`: Saves registers to the stack, loads three values from the stack, and uses one of those values as an index into a jump table to jump to a new address, provided the index is not greater than 5.
- `restore_registers_and_return`: Clears register d0 and restores several data and address registers from the stack before returning.
- `handle_printer_data`: Processes a command based on a value in a pointer, performing various writes to a destination buffer, updating counters, and converting numeric values into ASCII characters via a loop.
- `copy_table_byte_loop_short`: Copies a byte from a table at offset 10 of a pointer in a0 to a destination address calculated from d4, d3, and d0, then loops if d2 is greater than d5.
- `update_data_and_call_2`: Calculates a value using bits from two stack locations, writes three bytes to a destination address formed by d3 and d4, calls another routine, and then writes a fourth byte from a table at an offset determined by the result of that routine.
- `update_data_and_call`: Calculates a value using bits from two stack locations, writes three bytes to a destination address formed by d3 and d4, calls another routine, and writes a byte from a global table at an index returned by that routine to the fourth byte of the destination.
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
- `execute_sequential_function_pointers`: Loads a value from a global variable and calls three successive functions located at offsets from that value, and if the second and third calls return non-zero, it calls another routine using the global variable as an argument.
- `execute_two_function_pointers`: Loads a pointer from a global variable into a1, then performs two successive calls to functions located at offsets from a pointer in a6, using values from a global variable and a1, before returning with -1 in d0.
- `execute_triple_library_call`: Calls a library function at a specific offset from a global base three times, passing the value from a6 to the second and third calls.
- `return_constant_29`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_30`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_31`: Returns the constant an address in d0 to the caller without any other effect.
- `check_printer_status_type`: Loads three pointers from the stack, clears three registers, and compares the word at the first pointer to one; if the value is not one, it branches to a different location, otherwise it continues to another routine.
- `parse_printer_command_sequence`: Copies a sequence of bytes from a source buffer into a destination buffer, applying specific control characters based on values read from a structure.
- `process_printer_control_sequence`: Copies a null-terminated string into a destination buffer, then reads two bytes from a table at an offset of a pointer in a register to initialize a sequence.
- `copy_table_values_in_loop`: Copies bytes from a table at a constant address to a destination buffer starting at an offset of a pointer in a2, using a loop that runs until a counter in d3 reaches 11.
- `update_status_word`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is not zero, it writes 32 to the address in a0, writes 1 to the address in a1, and branches to another routine.
- `update_word_on_condition_3`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is less than zero, it writes 4 to the word at the address in a0 and branches to another routine.
- `update_status_on_condition_2`: Compares a word at the address in a0 to 28; if it matches and the byte at the address in a1 is non-zero, it writes 33 to the word at the address in a0, then branches to another routine; otherwise it returns.
- `update_status_on_condition`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is non-zero, it writes 33 to the word at the address in a0 and 255 to the byte at the address in a1, then branches to another routine.
- `update_word_on_condition_2`: Compares a word at the address in a0 to 29; if it matches, it checks if the byte at the address in a1 is greater than zero, and if so, writes the word 2 to the address in a0.
- `set_word_if_negative`: Compares a word at the address in a0 to 30.
- `update_word_and_byte`: Compares a word at a pointer in a0 to 31; if it is not 31, it branches elsewhere, otherwise it checks a byte in a1 to decide whether to write 33 or 20 to the word at a0 and whether to clear the byte in a1.
- `increment_on_match`: Compares a word at a pointer in a0 to 32; if it is not 32, it increments a byte in a1 and branches to another routine.
- `decrement_byte_on_match_2`: Compares a word at a pointer in a0 to 33; if it is 33, it decrements a byte in a1 and branches to another routine, otherwise it branches elsewhere.
- `write_status_byte_to_structure_2`: Checks if a word at a pointer in a0 is non-zero; if it is zero, it writes the byte 253 to an offset of 2720 in a structure pointed to by a4.
- `write_constant_byte_to_buffer`: Writes the byte 66 to a destination buffer at an offset calculated from a pointer in a2 and then branches to another routine.
- `write_status_byte_to_structure`: Checks if a word at a pointer in a0 is non-zero; if it is zero, it writes the byte 253 to an offset of 2720 in a structure pointed to by a value in d1, then returns.

### HP_LaserJet

- `return_constant_32`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_callback_sequence_2`: Attempts to call three different functions stored at specific offsets within two different structures; if all three calls succeed, it returns a failure code, otherwise it calls a cleanup routine on the first structure and returns.
- `execute_sequential_conditional_calls_3`: Loads a pointer from a global variable and calls a routine at a negative offset from a second global pointer, then repeats this process using a third global pointer and a different value in a1 before returning.
- `execute_printer_driver_function`: Fetches a function pointer from a global variable using an offset and calls that function three times, passing the value from a register as an argument to the first call and the subsequent calls.
- `return_constant_33`: Returns the constant an address in d0 to the caller without any other effect.
- `check_then_queue_request`: Calls a function through a pointer in the global base and, if it returns zero, stores a request code and two parameters into a structure, clears a flag byte, then invokes several Amiga library routines through the base to process that structure, returning the zero status.
- `format_hp_laserjet_escape_sequence_3`: Copies a sequence of bytes from a table into a buffer and modifies specific bytes based on values read from a structure.
- `format_hp_laserjet_escape_sequence`: Copies a sequence of bytes from a table into a buffer and modifies specific bytes based on values read from a structure.
- `process_printer_escape_sequences_2`: Updates a buffer with data from a table and performs conditional logic based on specific byte values to modify escape characters and control printer command sequences.
- `copy_array_and_call_subroutines`: Copies bytes from a global table to a destination buffer in a loop for 11 iterations, then calls another routine twice using values derived from a structure and a global variable.
- `update_word_and_byte_2`: Compares a word at a pointer in a2 to 27; if it is 27 and the byte at a3 is non-zero, it writes 32 to the word at a2 and 1 to the byte at a3, then branches to another routine.
- `update_word_on_match_3`: Compares a 16-bit word at the address in a2 to 27; if it matches and the byte at the address in a3 is less than zero, it writes the word 4 to the address in a2 and branches to another routine.
- `update_word_on_match`: Compares a 16-bit word at the address in a2 to 28; if it matches and the byte at the address in a3 is greater than zero, it writes the word 33 to the address in a2 and branches to another location; otherwise it returns.
- `modify_values_on_match`: Compares a word at the address in a2 to 29; if it matches and the byte at the address in a3 is non-zero, it writes the word 33 to the address in a2 and the byte 255 to the address in a3.
- `set_word_if_positive`: Compares a word at the address in a2 to 29.
- `set_word_if_negative_2`: Compares a 16-bit word at the address in a2 to 30; if it matches and the byte at the address in a3 is less than zero, it writes the word 32 to the address in a2 and branches to another location; otherwise it returns.
- `update_status_word_2`: Compares a word at the address in a2 to 31; if it is not 31, it branches to another routine, otherwise it checks a byte at a3.
- `compare_and_increment_byte`: Compares a word at the address in a2 to 32; if it does not match, it branches to another routine, otherwise it increments the byte at the address in a3 and branches to another routine.
- `decrement_and_branch_4`: Compares a word at the address in a2 to 33; if it does not match, it branches to another routine, otherwise it decrements the byte at the address in a3 and branches to another routine.
- `check_word_and_copy_byte`: Compares a word at the address in a2 to 64; if it matches, it loads a pointer from offset 28 of a structure in a6, checks if the byte at that pointer is non-zero, and if so, copies a byte from a global variable to that pointer location.
- `decrement_and_copy_data`: Decrements a byte at the address pointed to by an offset in a6, updates a global variable with the new value, conditionally updates a byte at the next address, copies 11 bytes from a global table to a destination in a4, and calls two subroutines.
- `copy_byte_from_offset`: Increments a value from a global variable at an offset in a6, loads a byte from that new address into d1, and writes d1 to the address in a4.
- `copy_and_call_subroutines`: Copies 11 bytes from a global variable to a destination buffer starting at the address in a4 plus the offset in a5 using a loop, then calls two different subroutines using a pointer in a3.
- `compare_word_and_branch_3`: Compares a word at the address in a2 to 57 and branches to different locations based on the result.
- `copy_data_and_call`: Copies 11 bytes from a global variable to a destination buffer using a loop, then calls another routine twice with different arguments.
- `write_byte_on_null_check_alt`: Checks if a word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d2.
- `convert_to_ascii_digits`: Writes a character to a destination buffer based on the value of a parameter, then writes a second character to the next byte in the buffer by dividing a value by 10 and adding 48, and finally writes a third character to the following byte by calling another routine.
- `convert_byte_to_ascii`: Compares a byte in d0 to 99; if it is greater than 99, it writes 49 to a location at a2, decrements d0 by 64, and branches to another routine.
- `convert_integer_to_ascii`: Writes the byte 48 to a memory location, then writes either 48 or the result of dividing a byte in d0 by 10 plus 48 to the next memory location, calls another routine, and writes the returned byte plus 48 to the following memory location.
- `convert_to_ascii`: Writes the byte 48 to a memory location, calls another routine, and then writes the returned byte incremented by 48 to the next memory location.
- `call_two_subroutines`: Calls a routine at an offset in a global variable with two parameters, then calls another routine at a different offset in the same global variable.
- `rotate_32bit_value_4`: Performs a 32-bit division and modulo operation using a value in d1 and a threshold in d2.
- `rotate_and_threshold_32bit_2`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_3`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components.
- `negate_result_on_flag`: Calls another routine and negates the returned value if the low word of the result is nonzero.
- `dispatch_via_stack_values`: Restores registers from the stack, loads a value from the stack, and uses it to perform an addition and a jump to an address from a table.
- `clear_registers_and_return`: Clears d0 and restores registers d2 through d4 from the stack before returning.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `initialize_structure_data`: Saves registers a2 and a3 to the stack, loads a word from the stack into d0, and branches to another routine if d0 is not 768, otherwise it branches to another routine.
- `compare_word_and_branch_2`: Compares a 16-bit value in d0 to 256 and branches based on whether the value is less than or equal to or higher than that constant, otherwise it branches elsewhere.
- `compare_and_branch_2`: Compares a 16-bit value in d0 to 512 and branches if it is not equal to that value, otherwise it branches to a different location.
- `compare_word_and_branch`: Compares a word in d0 to 1024 and branches to a specific address if they are not equal, otherwise it branches to another address.
- `initialize_structure_fields`: Writes the values 600, 800, 75, and 53 to various offsets within a structure pointed to by a pointer in a1, and sets a global variable to 48.
- `set_structure_constants`: Writes the values 800, 1060, 100, 86, and 49 to specific offsets in a structure pointed to by a pointer in a1.
- `set_structure_and_byte`: Writes the values 1168, 1590, 150, 134, and 49 to specific offsets in a structure pointed to by a pointer in a1, and writes the byte 53 to the address in a2.
- `initialize_structure_values`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 33, and writes 30 to two memory locations.

### Okidata_292

- `return_constant_34`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_callback_chain`: Calls a function pointer located at an offset from a global variable three times sequentially using different addresses stored in memory, and if any call returns non-zero, it executes a cleanup routine at a specific offset from that same global variable before returning.
- `execute_sequential_conditional_calls_4`: Loads a pointer from a global variable into a1, then performs two sequential calls to routines located at offsets from two different global pointers, where each subsequent call is only executed if the previous one returns.
- `call_routine_three_times`: Calls a routine at an offset in a global variable three times, passing the value from a6 to the second and third calls.
- `return_constant_35`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_36`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_4`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base using a pointer retrieved from that structure.
- `manage_dynamic_buffer_structure`: Implements a multi-mode state machine that manages a dynamic data buffer by calculating offsets, performing bitwise OR operations on specific bytes, clearing memory ranges, and populating a structured table with repeating sequences of constant bytes and values from a global array.
- `clear_d0_and_restore_registers_2`: Sets register d0 to zero and restores several data and address registers from the stack before returning.
- `initialize_command_buffer`: Saves registers to the stack, loads pointers from the stack into a2 and a3, clears d3 and d6, sets d1 to 0 and d5 to 3, and branches to another routine if the word at the address in a2 is 1.
- `format_printer_command_buffer`: Constructs a command string in a destination buffer by copying data from a table and appending bytes based on values read from a structure.
- `write_byte_to_calculated_address`: Compares a word at offset 128 of a structure pointed to by a register against 2048, and if equal, writes the byte 15 to an address calculated from d2, d3, and a4, then sets a0 to a new pointer.
- `process_printer_command_sequence`: Copies a null-terminated string from a source buffer to a destination buffer using a loop, then processes a command byte by updating a structure, writing specific byte sequences to the destination, or modifying values in a data structure based on various command codes.
- `copy_byte_from_table`: Copies a byte from a table at address zero, indexed by a value in d1, to a destination address calculated from d0 and d2, and loops until d6 reaches 4.
- `write_byte_on_match_2`: Compares a word at the address in a2 to 66; if it matches, it writes the byte 1 to offset 2 of a structure, loads a pointer from a structure in d1 into a1 and then into a0, checks if a word at offset 2166 of the address in a0 is 2048, and if it is, writes the byte 96 to the address in a0.
- `write_byte_on_match`: Loads a pointer from a global variable into a1, then loads a pointer from the address in a1 into a0.
- `write_byte_to_pointer`: Moves the address in d5 to a0, writes the byte 80 to the location in a0, and branches to another routine.
- `copy_bytes_from_table_2`: Copies a sequence of bytes from a table to a destination address calculated from two pointers, incrementing the destination and the table index for each byte until a counter reaches four.
- `update_status_values`: Compares a word at the pointer in a2 to 32; if it is not equal, it checks if a byte at a3 is non-zero, and if it is zero, writes 1 to a3 and 27 to a2, otherwise it branches to another routine.
- `update_byte_and_word`: Clears a byte at address a3 if it is not positive, writes the word 30 to address a2, and then branches.
- `branch_with_setup`: Sets d0 to 255 and branches to an offset in another routine.
- `update_status_byte_2`: Compares a word at a2 to 33, and if not equal, checks if a byte at a3 is non-zero; if it is zero, writes the byte 255 to a3 and the word 29 to a2, otherwise it returns.
- `clear_byte_and_word`: Clears a byte at address a3 if it is not less than or equal to zero, writes the word 28 to address a2, and then branches.
- `branch_to_offset_2`: Branches to a relative offset.
- `update_state_values`: Updates various bytes and words based on the value at address a2, and if the value is 2, writes three bytes to a destination calculated from d2 and d3 before branching.
- `modify_stack_bytes`: If the word at address a2 is 12, it checks a byte at an offset from the stack pointer; if that byte is 39, it writes 30 to that stack location, and if that byte is 37 or less, it branches.
- `write_three_bytes`: Writes the bytes 27 and 114, followed by a byte from a table indexed by a value minus 30, to a memory location.
- `write_status_byte_2`: If the word at the address in a2 is zero, it writes the byte 253 to offset 2720 of the structure pointed to by the address in d1.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Okimate_20

- `return_constant_37`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_indirect_calls_3`: Attempts to call three different functions stored at specific offsets within two different structures pointed to by a global variable and a parameter, then executes cleanup calls at a specific offset if any of the initial calls return non-zero.
- `execute_triple_conditional_call_2`: Calls three different routines via offsets from two different global pointers, only executing subsequent calls if the previous one returns a non-zero value.
- `update_handler_pointer`: Calls a routine three times using a pointer from a global structure and then updates a global variable with its own address.
- `set_function_pointer_2`: Loads a pointer from a global variable into a0, compares a word at offset 164 of that pointer to 2; if it is 2, it loads a new address into a0 and writes it to offset 54 of that pointer, then branches to another routine.
- `set_function_pointer`: Stores the address of this routine into a global variable at offset 54 and returns zero.
- `return_constant_38`: Returns the constant an address in d0 to the caller without any other effect.
- `construct_printer_command_buffer`: Constructs a command sequence in a memory buffer by calculating offsets and writing specific byte values, including a loop that repeats three times to insert a series of control characters and data bytes.
- `return_zero_and_restore`: Clears register d0 and restores registers d2 through d6 and a2 through a5 from the stack before returning.
- `manage_buffer_state_machine`: Implements a state machine that handles buffer initialization, byte-wise bitwise OR operations, memory clearing, and string construction using a series of cases.
- `return_empty_register_state`: Clears the d0 register and restores several data and address registers from the stack before returning immediately.
- `divide_32_bit_integers_2`: Performs 32-bit division of a value in d1 by a value in d2, returning the quotient and remainder in a 64-bit result.
- `rotate_and_subtract_32bit_value`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_4`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products and returns the result.
- `forward_call_to_helper_3`: Moves the contents of two registers into the arguments of another routine, calls it, and returns the result in a register.
- `initialize_and_check_status`: Saves registers to the stack, loads several pointers from the stack, clears d2 and d5, sets a0 to zero, sets d3 to 26 and d4 to 16, and branches based on a comparison of a word at the address in a0 with 1.
- `build_command_sequence_2`: Copies 15 bytes from a global variable to a destination buffer in a loop, writes zero to two different offsets in that buffer, and performs several conditional writes to the buffer based on values found at specific offsets within a structure pointed to by a register.
- `build_command_sequence`: Writes a sequence of bytes to a destination buffer based on values read from a structure, including a loop that writes the byte 13 a variable number of times.
- `fill_buffer_with_carriage_return`: Loops through a destination buffer, writing the value 13 to each byte at an incrementing offset until a counter reaches a limit value read from a pointer-based structure.
- `process_printer_control_codes_2`: Processes printer control sequences by writing carriage returns and line feeds, or inserting escape sequences based on the value of a byte at a pointer.
- `fill_buffer_with_carriage_return_2`: Loops through a destination buffer, writing the value 13 to each byte at an incrementing offset until a counter reaches a limit value read from a pointer-based structure.
- `process_printer_control_codes`: Checks a byte at a pointer to determine a control sequence; if it matches a specific value, it writes an escape sequence and a loop of carriage return bytes to a buffer based on a length value.
- `map_command_values_2`: Loops through a sequence, writing the value 13 to a destination buffer at an offset determined by a counter, until the counter reaches a value loaded from a pointer.
- `compare_and_set_bytes`: Compares a word at the address in a0 to 32; if it does not match, it branches to another routine, otherwise it tests a byte at the address in a2, and if that byte is zero, it sets the byte at a2 to 1 and the word at a0 to 1.
- `update_conditional_byte`: Checks if a byte at the address in a2 is non-negative; if it is, it clears the byte at that address, writes the word 30 to the address in a0, and branches to another routine; otherwise, it clears the byte at the address in a2 and branches to another routine.
- `set_d0_and_branch_2`: Sets register d0 to negative one and branches to another routine.
- `validate_and_reset_command_byte`: Compares a word at a pointer in a0 to 33; if it does not match, it branches to another routine, otherwise it tests a byte at a pointer in a2, and if that byte is not zero, it clears it and writes 29 to the word at a0 before branching to another routine; otherwise it writes 255 to the byte at a2 and writes 29 to the word at a0 before branching to another routine.
- `map_command_values_3`: Tests a byte at the pointer in a2; if it is not less than or equal to zero, it clears that byte and writes the word 28 to the address in a0, otherwise it branches to another routine.
- `branch_to_another_routine_2`: Branches to another routine after the caller sets register d0 to negative one.
- `map_command_values`: Compares a byte at a pointer against several constants and writes specific byte or word values to a destination based on the match.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_Advantage_D25

- `return_constant_39`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_call`: Sets several stack values using a parameter and a global variable, then calls three different routines via pointers found at specific offsets from a global variable and another variable, returning 0 if all calls succeed or a fixed address otherwise.
- `execute_dual_calls_2`: Loads two different addresses from a table into a1 and a6, then calls a routine at an offset from the first address and another routine at an offset from the second address before branching.
- `call_library_function_thrice`: Calls a library function at a specific offset from a base address three times, passing the value from a6 to the second and third calls.
- `return_constant_40`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_41`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_42`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_and_check_status_2`: Saves registers to the stack, loads three pointers from the stack into a0, a4, and a1, clears registers d2, d3, and d1, and branches to another routine if the word at the address in a0 is not equal to 1.
- `parse_printer_control_sequence`: Copies a sequence of bytes from a source buffer into a destination buffer, applying specific byte transformations based on values read from a structure.
- `process_buffer_and_update_values`: Copies bytes from a source in d4 and d3 to a destination in a4 using a loop, updates a global variable with two words from a pointer in d1, and performs several conditional checks and updates on a word in a0 and a byte in a1.
- `copy_byte_in_loop`: Copies bytes from a table at address zero to a destination calculated from a3 and a4 in a loop that runs until a counter in d0 reaches the value in d3.
- `update_word_and_byte_4`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is not negative, it writes 32 to the word at the address in a0 and writes 1 to the byte at the address in a1.
- `update_word_if_negative`: Compares a word at the address in a0 to 27, checks if the byte at the address in a1 is negative, and if so, writes the word 4 to the address in a0.
- `update_word_value`: Compares a word at the address in a0 to 28; if it matches and the byte at the address in a1 is positive, it writes the word 33 to the address at a0 and branches to another routine, otherwise it branches to another routine.
- `modify_word_and_byte_3`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is non-zero, it writes 33 to the word at the address in a0, writes 255 to the byte at the address in a1, and branches to another routine.
- `set_word_if_positive_2`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is positive, it writes the word 2 to the address at a0 and branches to another routine, otherwise it branches to another routine.
- `update_conditional_byte_2`: Compares a word at the address in a0 to 30; if it is not equal, it branches to another routine, otherwise it checks if the byte at the address in a1 is negative and, if so, writes 32 to the address in a0.
- `modify_word_and_byte`: Compares a word at the address in a0 to 31; if it is not 31, it branches to another routine; if it is 31, it checks a byte at the address in a1; if that byte is greater than zero, it writes the word 33 to the address in a0; if that byte is less than or equal to zero, it checks the byte at a1 again and writes the word 32 to the address in a0 and clears the byte at the address in a1 before branching to another routine.
- `increment_byte_on_match`: Compares a word at the address in a0 to 32; if it does not match, it branches to another routine, otherwise it increments the byte at the address in a1 and branches to another routine.
- `decrement_byte_on_match`: Compares a 16-bit word at the address in a0 to 33; if they are equal, it decrements the byte at the address in a1.
- `write_status_byte_if_zero`: Checks if a 16-bit word at the address in a0 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d1.

### Alphacom_Alphapro_101

- `return_constant_43`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_cleanup_library_pointers`: Attempts to call three consecutive functions via offsets from a pointer stored in a global variable, returning 0 if all succeed or a fixed address if any fail.
- `execute_triple_call_3`: Sets several stack values using a parameter and a global variable, then calls three different routines via pointers found at specific offsets.
- `execute_indirect_function_calls_3`: Fetches a function pointer from a global variable using a negative offset and calls that function three times, passing the value from register a6 as an argument to the second and third calls.
- `return_constant_44`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_45`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_46`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_local_registers`: Saves six registers to the stack, loads three pointers from the stack into a1, a4, and a2, and clears registers d1, d2, and d3 before checking if a word at the address in a1 is 1.
- `construct_printer_escape_sequence`: Copies twelve bytes from a table into a buffer, then appends an escape sequence based on values read from a structure.
- `fill_buffer_with_sequence_2`: Fills a buffer starting at the address in a4 with the byte 32 in a loop until a counter reaches a value from the stack, then appends a sequence of bytes, performs a second loop to fill more bytes with 32, and appends another sequence of bytes.
- `fill_buffer_with_sequence`: Fills a buffer starting at the address in a4 with the byte 32 in a loop, then appends a specific sequence of seven bytes to the end of the filled area.
- `update_status_byte_3`: Compares a 16-bit word at the address in a1 to 27; if it is not 27, it checks if the byte at a2 is non-zero, otherwise it writes 32 to the address in a1, writes 1 to the address in a2, and branches to another location.
- `compare_and_update_word`: Compares a 16-bit word at the address in a1 to 27; if it is not 27, it checks if the byte at a2 is non-zero, otherwise it writes 4 to the address in a1 and branches to another routine.
- `update_conditional_word_alt`: Compares a word at the address in a1 to 28; if it matches and the byte at the address in a2 is positive, it writes 33 to the address in a1 and branches to another routine.
- `update_conditional_word`: Compares a word at the address in a1 to 29; if it is not 29, it branches to another routine; if it is 29, it checks if the byte at the address in a2 is non-zero; if it is, it writes the word 33 to the address in a1, writes the byte 255 to the address in a2, and branches to another routine.
- `update_word_on_condition`: Compares a 16-bit word at the address in a1 to 29; if they are equal and the byte at the address in a2 is greater than zero, it writes the word 2 to the address in a1.
- `update_conditional_word_final`: Compares a word at the address in a1 to 30; if it matches and the byte at the address in a2 is negative, it writes 32 to the address in a1 and branches to another routine.
- `modify_conditional_bytes`: Compares a 16-bit word at the address in a1 to 31; if it matches and the byte at the address in a2 is greater than zero, it writes the word 33 to the address in a1; if it matches and the byte at the address in a2 is less than or equal to zero, it writes the word 32 to the address in a1 and clears the byte at the address in a2; otherwise it returns.
- `increment_byte_on_match_2`: Compares a word at the address in a1 to 32; if it is not 32, it branches to another routine, otherwise it increments the byte at the address in a2 and branches to another routine.
- `compare_and_decrement_byte`: Compares a word at the address in a1 to 33; if it does not match, it branches to another routine, otherwise it decrements the byte at the address in a2 and branches to another routine.
- `process_buffer_command`: If the word at a1 is 12, it writes 27 to the address in a1 plus the offset in d1, and writes 65 or 66 to the address in a1 plus the offset in d1 plus 1 based on a byte at a stack location.
- `write_byte_to_offset_2`: Writes the byte 66 to a destination calculated by adding a4 to the address in a0, which is derived from d1.
- `write_structured_buffer`: If the word at the address in a1 is 66, it writes a sequence of bytes including 13 and 80 copies of the byte 8 to a buffer, followed by a sequence of escape bytes; if the word at the address in a1 is 0, it writes the byte 253 to an offset of 2688 from a pointer in d2.
- `write_status_byte_if_zero_2`: Checks if a 16-bit word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by the pointer in d2.

### Epson_JX-80

- `return_constant_47`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_indirect_calls_4`: Retrieves a pointer from a global variable and calls a function at a specific offset within that pointer's target structure.
- `execute_triple_conditional_calls_2`: Calls two different routines using offsets from two global pointers, only executing the second call if the first returns a non-zero value, then sets a register to negative one and branches.
- `execute_triple_handler`: Calls a routine through a pointer found at an offset from a global variable, then calls that same routine twice more using the value in a6 as an argument.
- `return_constant_48`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_49`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_5`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte at specific offsets, then executes several routines through a global base and another pointer.
- `format_epson_jx80_print_buffer`: Constructs a structured print buffer for an Epson JX-80 printer by calculating offsets and writing escape sequences, character data, and control bytes into a memory area.
- `return_zero_and_restore_registers`: Clears the d0 register and restores d2 through d4 and a2 through a4 from the stack before returning.
- `setup_and_branch_on_word`: Saves registers to the stack, loads a2 from the stack, loads d1 from the stack, loads a3 from the stack, clears d3 and d6, clears d2, sets d4 to 6 and d5 to 3, and branches based on a comparison of a word at the address in a2 with 1.
- `format_epson_jx80_command`: Constructs a command buffer by copying data from a table and appending specific control bytes based on values read from a structure.
- `process_epson_jx_80_commands`: Parses command bytes and status values to construct a sequence of control bytes in a destination buffer.
- `copy_modified_bytes`: Loads a byte from a stack location, decrements it, and writes it to the address in a1.
- `copy_byte_from_table_2`: Calculates a destination address using d1 and d3, increments d3, and copies a byte from a table at an offset in d6 to the destination; it then compares d0 to d6 and branches to another routine if d0 is greater than d6.
- `translate_printer_control_codes`: Processes a stream of control characters by translating specific input values into corresponding escape sequences for an Epson JX-80 printer.
- `load_and_check_pointer`: Loads a pointer from the address in a1 into a0, compares the word at offset 1048 of the address in a0 to 1024, and if they are not equal, it writes the byte 137 to the address in a0 and branches to another routine.
- `write_byte_to_pointer_2`: Writes the byte 80 to the address in a0 and branches to another routine.
- `copy_bytes_from_table_3`: Copies a byte from a table at offset zero to a destination at the address formed by adding a pointer in a1 and d1, then repeats this in a loop for a count specified by d6, incrementing the source and destination pointers each time.
- `update_values_conditionally`: Compares a 16-bit word at the address in a2 to 32, and if it is not equal, branches; otherwise, it checks a byte at a3, and if that byte is not zero, it sets the byte at a3 to 1, sets the word at a2 to 27, and branches.
- `clear_byte_and_set_word`: Checks if a byte at a3 is negative; if so, it clears the byte at a3, sets the word at a2 to 30, and branches.
- `set_d0_and_branch_3`: Sets d0 to 255 and branches to another location.
- `set_printer_status`: Compares a 16-bit word at the address in a2 to 33; if it matches and the byte at the address in a3 is not zero, it writes the byte 255 to the address in a3, writes the word 29 to the address in a2, and branches to another routine.
- `clear_and_set_word`: Checks if the byte at the address in a3 is greater than zero; if so, it clears the byte at the address in a3, writes the word 28 to the address in a2, and branches to another location; otherwise it returns.
- `branch_to_another_routine_3`: Performs a short branch to another routine after setting a register to two hundred fifty-five via set_d0_and_branch_3.
- `process_printer_command_2`: Checks a byte at a2 against several values to write specific bytes or sequences to a destination buffer, including writing 27 and 74 followed by a value from the stack, or writing 27 and 114 followed by a value from a global table.
- `handle_printer_command`: If the word at a2 is 4, it writes 27 and 106 to a destination, then writes a byte from the stack to that destination.
- `check_buffer_contents`: Compares a 16-bit word at a2 to 12, and if it is not equal, branches; otherwise, it checks a byte at the address in a0 (loaded from a stack offset) against 39 and 37, and if the byte is not 37 or is less than 37, it branches or performs a final branch.
- `write_sequence_to_buffer`: Writes the byte 27 and the byte 114 to a buffer at an offset, then writes a 16-bit word from a global table to the following position in the buffer.
- `write_status_byte_if_zero_3`: Checks if a 16-bit word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d2.
- `call_library_vector_198_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Epson

- `return_constant_50`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_call_2`: Loads a value from a global variable, calls a function pointer at an offset from that value three times, and if all calls return non-zero, returns zero; otherwise, it calls another routine with the global variable as an argument and returns -1.
- `execute_triple_call_sequence`: Loads a pointer from a global variable and calls a function at an offset from that pointer twice, then branches to a third address loaded from a different global variable.
- `execute_printer_driver_function_2`: Calls a function pointer located at an offset from a global variable three times in succession.
- `return_constant_51`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_52`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_6`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a status byte, then executes three routines through a global base using a pointer from the structure.
- `manage_printer_buffer_and_output`: Implements a state machine for printer buffer management, including zeroing out a memory range, appending bitmask data, and formatting a control sequence with specific byte values for output.
- `restore_registers_and_return_2`: Clears register d0 and restores several registers from the stack before returning.
- `setup_and_branch_on_word_alt`: Saves registers to the stack, loads a1 from the stack, loads a4 from the stack, loads a3 from the stack, clears d3 and d5, clears d1, sets d2 to 6 and d4 to 3, and branches based on a comparison of a word at the address in a1 with 1.
- `format_printer_command_buffer_2`: Copies a sequence of bytes from a table into a destination buffer and populates it with command parameters derived from a structure pointed to by a register.
- `process_printer_command_sequence_2`: Parses a command byte to update a printer status structure and constructs a command buffer by copying bytes from a table or decrementing values from a stack.
- `copy_table_bytes`: Loads a byte from a stack location, decrements it, and writes it to the address in a1.
- `copy_byte_in_loop_2`: Copies a byte from a table at offset d0 to a destination at the address in a2, which is calculated by adding a4 to d0, and repeats this in a loop while d5 is less than 8.
- `process_printer_control_codes_3`: Updates printer status and control sequences based on a byte value, including writing escape sequences to a buffer and copying eight bytes from a table.
- `check_and_set_status_byte`: Loads a pointer from a register, compares a word at an offset of 1024 from that pointer to 1024, and if they match, writes the byte 137 to the address stored in a second register; otherwise, it branches to another routine.
- `update_status_byte_via_pointer`: Loads a pointer from a data register into a0, writes the value 80 to the address pointed to by a0, and branches to another routine.
- `copy_table_bytes_to_buffer`: Copies a sequence of bytes from a table into a destination buffer by incrementing pointers and indices, continuing until a counter reaches eight.
- `check_status_and_write`: Compares a 16-bit word at the address in a1 to 32; if it is not 32, it checks if the byte at a3 is non-zero, otherwise it writes 1 to the address in a3, writes 27 to the address in a1, and branches to another routine.
- `clear_and_write_word`: Checks if the byte at a3 is negative; if so, it clears the byte at a3, writes 30 to the address in a1, and branches to another routine.
- `set_d0_and_branch_6`: Sets register d0 to 255 and performs a short branch to another routine.
- `modify_printer_command`: Compares a 16-bit word at the address in a1 to 33; if it matches and the byte at the address in a3 is not zero, it writes the byte 255 to the address in a3, writes the word 29 to the address in a1, and branches to another routine.
- `clear_and_set_word_alt`: Checks if the byte at the address in a3 is greater than zero; if so, it clears the byte at the address in a3, writes the word 28 to the address in a1, and branches to another location; otherwise it returns.
- `branch_to_another_routine_4`: Performs a short relative branch to another routine.
- `handle_command_logic`: Checks a byte at a1 against several values to write specific bytes to a destination, such as writing 253 to an offset of 2720 in a buffer, or writing 27, 74, and a stack value to a destination.
- `write_byte_if_zero`: Checks if a word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d1.
- `call_library_vector_198_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### HP_LaserJet_PLUS

- `return_constant_53`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_function_chain`: Loads a pointer from a global variable and performs three successive calls to functions located at specific offsets from that pointer, returning zero if all calls succeed or negative one otherwise.
- `execute_dual_library_calls`: Loads two function pointers from a table and calls each one sequentially, returning negative one if successful.
- `execute_indirect_function_calls_4`: Fetches a function pointer from a table using an offset and calls that function three times in succession.
- `return_constant_54`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_7`: Checks a condition via a routine call, and if met, populates a structure with two parameters and a byte at offset 300, then executes several routines through a global base.
- `generate_hp_laserjet_escape_sequences`: Copies bytes from a table into a buffer and modifies specific offsets based on values read from a structure, then calls format_hp_laserjet_escape_sequence, process_printer_escape_sequences_2, or convert_to_ascii_digits depending on the state of the input parameters.
- `format_hp_laserjet_escape_sequence_2`: Copies bytes from a table into a buffer, modifies specific characters based on values read from a structure, and calls process_printer_escape_sequences_2.
- `process_hp_laserjet_escape_sequences`: Updates a buffer with specific byte sequences and manages printer state by checking and modifying control characters.
- `copy_table_elements`: Copies elements from a table into a destination buffer using a loop that runs eleven times, then calls another routine twice with specific arguments derived from a structure and a global variable.
- `compare_and_update_values`: Compares a word at the address in a2 to 27; if equal, it checks a byte at a3, and if that byte is non-zero, it writes 32 to the address in a2 and 1 to the address in a3 before branching.
- `update_printer_escape_sequence`: Compares a 16-bit word at the address in a2 to 27; if it matches and the byte at the address in a3 is not greater than zero, it writes the word 4 to the address in a2 and branches to another routine, otherwise it returns.
- `update_word_on_match_2`: Compares a word at the address in a2 to 28; if it matches and the byte at the address in a3 is greater than zero, it writes the word 33 to the address in a2, otherwise it returns.
- `set_word_and_byte`: Compares a word at the address in a2 to 29; if it is not equal, it branches to another routine, otherwise it checks if the byte at the address in a3 is non-zero, and if so, writes 33 to the address in a2 and 255 to the address in a3.
- `modify_value_on_match`: Compares a word at the address in a2 to 29; if it matches and the byte at the address in a3 is greater than zero, it writes the word 2 to the address in a2 and branches to another routine; otherwise it returns.
- `update_status_word_2_2`: Compares a word at the address in a2 to 30; if it is not 30, it branches to another routine, otherwise it checks if a byte at the address in a3 is less than 0, and if so, writes 32 to the word at the address in a2 before branching to another routine.
- `adjust_word_and_byte_range`: Compares a word at the address in a2 to 31; if it matches, it writes 33 to that address, otherwise if it is 32 it writes 32 to that address and clears the byte at the address in a3.
- `compare_and_increment`: Compares a word at the address in a2 to 32; if it is not equal, it branches to another routine, otherwise it increments the byte at the address in a3 and branches to another routine.
- `decrement_and_branch_5`: Compares a word at the address in a2 to 33; if it is not equal, it decrements a byte at the address in a3 and branches to another routine.
- `check_and_update_printer_status`: Compares a word at the address in a2 to 64 and, if equal, checks if a byte at the address pointed to by an offset in a6 is non-zero; if non-zero, it stores the byte 67 into the location pointed to by that same offset in a6 and branches to another routine, otherwise it branches to another routine.
- `decrement_and_update_buffer`: Decrements a byte at a pointer stored in a structure in a6, updates a global variable with the new value, and conditionally updates a second byte at that pointer's offset 1.
- `increment_and_copy_byte`: Increments a 32-bit value from a global variable at offset 28 of the structure in a6, loads a byte from the new address into d1, stores d1 as a 16-bit word at the address in a0, and branches to another routine.
- `copy_data_and_call_routines`: Copies 11 bytes from a global table to a destination pointed to by a pointer in a6 plus an offset, then calls another routine twice using values derived from a pointer at an offset in a6.
- `compare_and_branch_word`: Compares a word at the address in a2 to fifty-seven; if the value is not equal, it branches to another routine, otherwise it branches to a different routine.
- `copy_and_call_subroutines_2`: Copies 11 bytes from a global variable to a destination buffer starting at the address in a4 plus the offset in a5 using a loop, then calls two subroutines using pointers from a structure.
- `write_byte_on_null_check`: Checks if a word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by the address in d2, then returns.
- `convert_value_to_ascii`: Performs a recursive conversion of a value into ASCII characters by writing bytes to a destination at the address in a3 plus an offset in a2, based on whether the value is less than 200, less than 100, or less than a newline character.
- `convert_byte_to_ascii_2`: Compares a byte in d0 to 99; if greater than 99, it writes 49 to the address in a2, decrements d0 by 64, and branches; otherwise, it increments d3, adds the result to a2, and writes 49 to a2.
- `convert_digit_to_ascii_2`: Writes 48 to a destination, then writes either 48 or the result of dividing a byte by 10 plus 48 to the next byte.
- `convert_digit_to_ascii`: Writes the character zero to a memory location pointed to by two registers, calls another routine, and then writes the returned byte incremented by 48 to the subsequent memory location.
- `execute_dual_structure_callbacks`: Pushes two parameters onto the stack, calls a subroutine at an offset within a global variable, then calls a second subroutine at a different offset within the same global variable before returning.
- `rotate_32bit_value_5`: Performs a 32-bit rotation and conditional subtraction within a loop that executes sixteen times.
- `rotate_and_threshold_32bit_3`: Rotates a 32-bit value left by one bit for sixteen iterations, using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_5`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit halves and returns the result.
- `negate_result_on_flag_3`: Calls another routine and negates the returned value if the result in a0 is nonzero.
- `dispatch_via_table_3`: Restores registers from the stack, performs a comparison on a value from the stack, and then jumps to an address selected from a table based on that value.
- `clear_registers_and_return_2`: Sets d0 to zero and restores several registers from the stack before returning.
- `call_library_vector_198_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `branch_on_word_value`: Saves registers a2 and a3 to the stack, loads a word from the stack into d0, and branches based on whether d0 is less than or equal to 768, higher than 768, or neither.
- `compare_word_range`: Compares a 16-bit word in d0 to 256; if it is less than or equal to 256, it branches to another location, otherwise if it is greater than 256, it branches to another location, otherwise it branches to another location.
- `compare_d0_and_branch`: Compares d0 to 512 and branches if it is not equal, otherwise it branches to another location.
- `compare_and_branch`: Compares a word in d0 to 1024 and branches to different locations depending on the result.
- `initialize_structure_fields_2`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 48, and writes 55 and 53 to two memory locations.
- `set_structure_constants_2`: Writes several values into a structure pointed to by a1, including the values 800, 1060, 100, and 49, then branches to another location.
- `update_structure_and_register`: Writes several values into a structure pointed to by a1, including 1168, 1592, 150, and 53, writes 53 to the address in a2, and branches to another location.
- `initialize_structure_values_2`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 51, and writes 48 to two memory locations.

### 12

- `return_constant_55`: Returns the constant an address in d0 to the caller without any other effect.
- `process_font_data`: Performs complex data processing on a font structure.

### 15

- `return_constant_56`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_57`: Returns the constant an address in d0 to the caller without any other effect.
- `process_font_glyph_data`: Iterates through a data block to extract bitmask patterns and writes them to a destination buffer using a loop.

### 12

- `return_constant_58`: Returns the constant an address in d0 to the caller without any other effect.
- `process_font_glyph_data_2`: Iterates through a data structure to extract bitmask patterns and writes them to a destination buffer using a pointer in a register.

### 20

- `return_constant_59`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_2`: Clears the accumulator and returns zero.

### 11

- `return_constant_60`: Returns the constant an address in d0 to the caller without any other effect.

### Balance of Power - The 1990 Edition (1990)(Mindscape)[m].adf:CLI

- `parse_command_line_arguments`: Parses a command line string by iterating through characters to extract tokens, handling quoted strings and converting specific character sequences into newline or escape characters.
- `invoke_two_library_functions`: Loads a pointer from an offset of 92 in a4 into a0 and calls a function at a negative offset from a6, then repeats this process with a different negative offset from a6 before returning.
- `call_and_store_result`: Calls a routine at a negative offset from a base register, stores the result in a global variable, and otherwise returns.
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
- `store_pointer_to_global`: library code stores the value in a0 into a global variable and returns.
- `manage_library_list`: As library code, it checks a value in a structure; if zero, it moves a pointer and calls a function pointer, otherwise it sets bit 3 of a byte at offset 14 of the structure.
- `set_status_bit`: library code sets bit 3 of a byte at offset 14 of a structure in a6 and returns 0.
- `increment_and_initialize_on_condition`: library code checks if bit 3 of a byte at offset 14 of a structure in a6 is clear; if so, it increments a 16-bit value at offset 32 of that structure and, if that value is 1, calls another routine with a global variable as an argument, then returns the structure address; otherwise it returns -1.
- `return_negative_one`: Sets the d0 register to negative one and returns immediately.
- `decrement_counter_and_call_2`: Library code that decrements the 16-bit counter at offset 32 of the structure at A6.
- `mask_and_call_routine`: This library code performs a bitwise AND on register d0 with 127, calls another routine, and sets the carry flag.
- `process_bitfield_loop`: As library code, it checks a value in d0 and enters a loop that performs bitwise operations and arithmetic to update a value in d2 based on a running difference.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `call_and_mask_result`: Copies the value from d0 to d7, calls a routine through a pointer in a global variable, sets bit 0 of d0, clears bit 3 of d0, and branches to another location.
- `call_jump_table_entries`: Library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_d1_register`: As library code, it clears the long register d1 and branches to another instruction.
- `execute_conditional_logic_chain`: Library code that calls a routine via a pointer at an offset from a value in d0, performs conditional checks on a byte, and then calls several other routines at various offsets from that same pointer or calls another routine multiple times based on the results.
- `execute_conditional_library_calls_5`: As library code, it calls a function at an offset from a5, then performs a series of conditional calls to other functions at various offsets from a5 based on the values of d6 and d7.
- `execute_conditional_library_calls_3`: Library code that performs a series of conditional calls to routines at various offsets from a pointer in a5 based on the values of d6, d4, and d7, and returns a value with bits cleared.
- `execute_conditional_calls`: This library code moves d7 to d1, calls a routine at a negative offset from a5, and branches based on the result.
- `call_and_restore_d2`: Copies d2 to d0, calls a routine at an offset of minus 60 from a6, copies the result back to d2, and branches to another location.
- `select_constant_or_move_register`: Moves a longword from register d2 to d0, then checks register d4; if d4 is non-zero, it branches to another routine, otherwise it loads a constant into d2 and branches to another routine.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `calculate_bit_shift_value`: As library code, it performs a loop that bit-shifts values and decrements a counter to calculate a result based on the input in d7.
- `shift_and_branch_library_code`: Library code that performs a logical shift right on d7, initializes d6, d5, d1, and d2 with constant values, loads an address into a0, and branches to another location.
- `execute_loop_with_branching`: The library code executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_and_branch_library`: As library code, it pushes the word negative 65534 onto the stack and branches to another routine.
- `push_and_branch_library_alt`: As library code, it pushes the word negative 65535 onto the stack and branches to another routine.
- `push_and_branch_library_short`: As library code, it pushes the word 1 onto the stack and branches to another routine.
- `check_byte_and_adjust_stack`: As library code, it compares a byte in d7 to 184, branches if it is higher, otherwise it increments the stack pointer by 2, tests d7, and returns.
- `perform_math_transformations`: This library code performs complex arithmetic operations involving bitwise shifts and additions on registers d6 and d7, and uses a loop to iterate through a global table.
- `perform_arithmetic_loop_4`: As library code, it performs a loop that executes arithmetic operations using bitwise shifts and additions involving registers d6 and d7, and calls other routines.
- `perform_arithmetic_loop_3`: This library code performs a loop that executes arithmetic operations using bitwise shifts and additions involving registers d6 and d7, and accesses a global table via a pointer.
- `perform_arithmetic_loop_2`: As library code, it executes a loop that performs arithmetic operations using bitwise shifts and additions involving registers d6 and d7, while incrementing a pointer to a global variable and decrementing a counter.
- `perform_arithmetic_loop`: Library code that executes a loop performing arithmetic on d4, d5, d6, and d7, including bitwise shifts and pointer increments, and calls other routines based on the results and stack values.
- `set_d7_and_branch`: As library code, it loads the constant an address into d7 and branches to another routine.
- `recursive_call_wrapper`: Library code that branches if a byte in d7 is zero, otherwise moves d5 to d6, calls another routine, moves the result to d7, tests d7, restores registers from the stack, and returns.
- `call_routine_with_stack_storage`: As library code, it pushes d5 onto the stack, calls another routine, stores d6 at an offset from the stack pointer, restores d6 from the stack, and branches.
- `compare_and_branch_3`: This library code moves d6 to d4, branches if d4 is negative, otherwise compares d6 to 255 and branches if it is higher, or clears d6 and returns.
- `shift_and_jump`: As library code, it arithmetic shifts d4 right by 8 bits, increments d4, and if d4 is zero, it jumps to a location containing a jump instruction; otherwise it returns.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_callbacks`: Library code checks a byte and, if non-zero, performs a series of conditional calls to other routines based on the result of a comparison.
- `rotate_and_branch`: As library code, it loads an address into d7, rotates d7 right by 1 bit, and branches to another routine.
- `load_and_branch`: As library code, it loads an address into d7 and branches to another routine.
- `test_stack_byte_and_branch`: As library code, it pops a word from the stack into d6, tests the byte in d6, and if it is positive, branches to another routine; otherwise it sets d7 to 0 and branches to another routine.
- `set_negative_flag_and_restore_registers`: Sets the negative flag in the condition code register and restores several registers from the stack before returning.
- `load_constant_and_return`: Loads a constant into a register, adjusts the stack pointer by an offset of 30, and returns.
- `process_complex_math_logic`: As library code, it performs a series of conditional checks on d7 and calls various other routines based on the resulting values and bitwise manipulations.
- `execute_multiple_calls_2`: Library code that calls another routine four times and then returns.
- `loop_with_arithmetic_and_table_access`: This library code increments d2, moves d5 and d6 to d3 and d4, performs arithmetic shifts on d3 and d4, and enters a loop that adds d4 to d5, adds d3 to d6, and subtracts a value from a table pointed to by a0 until a counter in d1 reaches zero.
- `subtract_table_from_accumulator`: As library code, it subtracts d4 from d5, subtracts d3 from d6, adds a value from a table pointed to by a0 to d7, and repeats this in a loop while decrementing d1 until it reaches zero, then returns.
- `test_sign_and_branch`: As library code, it tests the sign bit of d7, branches if zero or positive, otherwise it masks d7 with 127, calls another routine, sets bit 1 of the status register, and returns.
- `set_d7_and_branch_2`: Sets register d7 to negative one and performs a short branch to another routine.
- `perform_complex_arithmetic`: Executes a loop involving bit-shifting and conditional jumps, and calls another routine.
- `complex_loop_with_branching`: The library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `test_and_call_routine`: As library code, it tests a byte in d7; if it is positive, it branches to another instruction, otherwise it masks d7 with 127, calls another routine, and sets the carry flag.
- `process_data_with_jumps`: Library code calls two routines, checks if a value in d7 is zero, and if not, performs a series of conditional calls and bitwise manipulations involving a 16-bit value.
- `transform_register_value`: Adds a value to a register, then performs a series of bitwise operations including an XOR, a bit shift, a subtraction, a word swap, and a bit rotation before returning.
- `modify_d7_with_overflow_checks`: Swaps d7, performs a rotate, and executes several conditional branches based on overflow and bitwise operations before returning.
- `compare_and_modify_byte`: This library code branches based on a condition, compares d7 to 124 and 126, and if d7 equals 126, adds 133 to it and performs a rotate left on d7 before testing d7 and branching.
- `test_word_mask`: Performs a bitwise AND on d7 with a mask, then branches to another routine if the result is zero, otherwise it clears d7 and branches to that same routine.
- `compare_and_shift`: As library code, it compares d7 to 254, and if it is not equal, branches to another routine; otherwise it performs two logical shifts right on d7 and branches based on the result of the second shift or branches to another routine.
- `shift_and_set_flag`: Shifts d7 left by 8 bits, sets d7 to 255, rotates the carry bit into the least significant bit of d7, sets the 2nd bit of the condition code register, and branches to another routine.
- `clear_and_branch`: Clears d7 and branches to another routine.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `load_address_and_branch_8`: Library code that loads a long value from an offset into a0 and then branches to the address in a0.
- `load_address_to_a0_3`: This library code loads a long address into a0 and then branches to the instruction at the address in a0.
- `load_address_to_a0`: As library code, it loads a long constant into register a0 and branches to another instruction.
- `load_zero_address`: Library code that loads the value zero into a0 and then branches.
- `load_address_and_branch_12`: As library code, it loads the address of a global variable into a0 and branches to the next instruction.
- `library_branch_with_address`: As library code, it loads the address of a stack pointer into a0 and branches to a different location.
- `load_address_to_a0_4`: As library code, it loads a long word from an offset of 6 in a global variable into a0 and then branches to another routine.
- `store_register_via_pointer`: This library code invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `load_address_and_branch_11`: Loads the address stored at offset 52 from a structure into a register and then branches to the next instruction.
- `load_zero_address_and_branch`: Library code that loads zero into a0 and then branches to the address in a0.
- `load_address_to_a0_2`: Loads the address of a global variable into register a0 and returns immediately.
- `load_constant_address`: Library code that loads the constant 38 into a0 and then branches.
- `load_null_pointer_and_branch`: Loads a null pointer into a register and performs a short branch to the next instruction.
- `load_address_and_branch_10`: As library code, it loads a constant address into a0 and then performs a short branch.
- `call_indirect_return_d0_2`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0.
- `load_address_and_branch_9`: Loads a longword from a memory location into a register and then branches to a new address.
- `load_address_to_register`: Loads a longword from a memory location into a register and then returns immediately.
- `load_address_and_branch`: Loads a long value into a0 and branches to another location.
- `indirectly_call_routine_via_offset`: Loads the address of a global variable into a register and branches to another routine that performs an indirect call using an offset added to the value in that register.
- `load_address_and_branch_2`: Loads the address of a global variable into a0 and performs a short branch to another routine.
- `load_address_and_branch_3`: Loads the address of a global variable into a6 and performs a short branch to another routine.
- `load_offset_pointer_and_branch`: Loads the address of a global variable into a register and branches to another routine.
- `load_indirect_call_address`: Loads the address of a global variable into a register and branches to another routine that performs an indirect call using an offset from that address.
- `load_address_and_branch_4`: Loads a longword into address register zero from a constant value and performs a short branch to another routine.
- `load_address_and_branch_5`: As library code, it loads a specific address into a0 and branches to another routine.
- `load_address_and_branch_6`: As library code, it loads a different specific address into a0 and branches to another routine.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in d6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_constant_61`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `set_icon_status_bit`: If a word at offset 8 of a structure in a6 is zero, it calls a function via a pointer at offset 22, updates a pointer in a linked list using values from a6, and returns a value from a6; otherwise, it sets bit 3 of a byte at offset 14 of the structure in a6 and returns zero.
- `set_icon_bit`: Sets bit 3 of a byte at offset 14 of a structure pointed to by a6, then returns with d0 set to zero.
- `return_constant_62`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_and_store`: Stores a value from a0 at offset 2a of a structure in d0 and a6 at offset 22, calls a function via a pointer at offset -228 of a6, and if the result is zero, calls another routine; otherwise, it stores a6 and the result into two global variables.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_bitwise_or`: Performs a bitwise OR of d0 with zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `recurse_icon_chain`: Saves two registers to the stack and calls another routine with the same first argument and a second argument increased by 140, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `manage_icon_chain_3`: Saves two registers to the stack, calls process_icon_chain, restores the registers, and returns.
- `traverse_icon_chain_2`: Saves two registers to the stack and calls another routine with the same first argument and a second argument increased by 78, then returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_and_return_subroutine`: Saves two registers to the stack, calls another routine, and then returns.
- `call_parse_numeric_string`: Saves registers a0 and a1 to the stack, calls parse_numeric_string, and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `populate_icon_structure`: As library code, it calls another routine, and if the result is non-zero, it copies 43 bytes from a stack buffer to a structure at the address in a1, writes several values to offsets of that structure, and potentially writes a pointer to a location at offset 76 of the structure.
- `copy_and_update_structure`: Copies 43 bytes from a buffer starting at offset -74 of a6 to a destination starting at offset 96 of a2 using a loop, then copies several 16-bit and 32-bit values from offsets -30, -28, -24, -20, -16, -12, -8, and -4 of a6 to various offsets in a2, and if a value at offset 76 of a2 is non-zero, stores a2 at offset 424 of the structure pointed to by that value.
- `return_zero_and_unlk_a6`: This library routine clears d0, restores registers from the stack, unlocks a6, and returns zero.
- `copy_icon_data_to_stack`: As library code, it copies 43 bytes from a structure pointed to by the third argument into a local stack buffer, copies several fields from that structure into local stack slots, and calls another routine with the first two arguments and a pointer to a two-word stack header.
- `populate_icon_structure_members`: Populates various fields of an icon structure by recursively calling other routines to fetch data for specific offsets, including handling nested structures and linked lists of data.
- `populate_nested_structure_members`: Checks a structure for specific constant values and then recursively calls other routines to populate various offsets with returned values.
- `update_nested_structures`: Library code iterates through structures, calling routines via pointers to update values at offsets 18, 22, 50, and 54, and performs a loop to copy data into a buffer.
- `iterate_and_update_icon_list`: As library code, this routine iterates through a list of pointers, calling two separate routines and updating a pointer and a counter in a structure until a counter reaches four or a routine returns zero.
- `clear_d6_and_branch`: Sets register d6 to zero and branches to another location.
- `unlk_a6_and_return`: This library routine unlocks a6 and returns without modifying d0.
- `traverse_icon_data_hierarchy`: Recursively iterates through a hierarchical icon structure by following pointers at various offsets within a structure, performing callbacks for child elements, lists, and associated data.
- `traverse_icon_hierarchy`: Recursively iterates through icon lists by checking flags and following pointers at offsets 50, 54, and 70 of a structure.
- `traverse_icon_lists`: Library code that iterates through several lists of pointers, calling functions via a pointer and updating a value at an offset of 26 from a structure.
- `process_icon_list_recursively_2`: Iterates through a list of pointers in a4 until a null pointer is found, calls a function via a pointer in a3, and if the result is non-zero, enters a loop that calls the same function for each pointer in a list at offset 54 of a2 and performs additional calls to the function in a3; finally, it calls the function in a3 twice and stores a value from a6 at offset 26 of a2.
- `process_icon_list_recursively`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a pointer or table.
- `traverse_icon_chain`: Iterates through a linked list of pointers, calling another routine on each non-zero element found, and calls two library functions using bases from global variables before returning.
- `restore_registers_and_return_4`: This library routine restores d2 and d3 from the stack and returns.
- `incremental_offset_recurse`: As library code, this routine performs a recursive call with the same first argument and the second argument increased by 140.
- `execute_conditional_library_calls_6`: Library code that calls a function pointer, then conditionally calls another routine, then calls a third routine with a parameter and a stack address, then conditionally calls a fourth routine with four parameters, and finally calls a fifth routine with three parameters before returning a value.
- `call_subroutine_with_stack_setup`: As library code, it pushes several registers and a pointer onto the stack, calls another routine, and then either moves a value from a pointer in a2 to d0 or adjusts the stack pointer before returning.
- `return_zero_short_stub`: This library routine restores d2, d3, and d4 from the stack and returns zero.
- `update_structure_elements`: Decrements a 16-bit counter through a pointer and, if not zero, stores two 32-bit values at offsets 16 and 20 of a structure calculated from a table base and the counter.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from registers and the stack into consecutive slots of that table.
- `process_icon_chain`: As library code, it calls another routine, and if the result is non-zero, it performs a series of conditional calls to other routines using various parameters, including the result of the first call and an offset of the result, and returns the result.
- `move_and_branch_2`: Moves the contents of register d6 into d0 and then branches to another location.
- `call_lib_func_and_return_zero`: Invokes a function pointer retrieved from a global variable associated with library index -210.
- `incremental_offset_recurse_offset`: As library code, this routine performs a recursive call with the same first argument and the second argument increased by 78.
- `search_string_for_equals`: As library code, it calls another routine to process a pointer from a list, then loops through the list, checking if the byte at the address pointed to by the current list element plus an offset is an equals sign, and returns a pointer to the character following the equals sign or zero if a null pointer is encountered.
- `restore_registers_and_return_5`: Clears d0 and restores d2, d3, a2, and a3 from the stack before returning.
- `iterate_and_compare_values_2`: Library code that calls another routine, then enters a loop to check if a 32-bit value in d4 is zero, increments it if non-zero, and performs conditional calls and bitwise manipulations based on the result.
- `iterate_and_compare_values`: As library code, it calls another routine, then enters a loop where it checks if a 32-bit value in d4 is zero, increments it if non-zero, and calls another routine to update d4 with a new value, continuing until d4 is zero or a specific condition is met.
- `increment_and_branch_on_d4`: As library code, this routine checks if a 32-bit value in d4 is zero; if not, it increments it, moves it to d2, and branches to another location, otherwise it branches elsewhere.
- `clear_and_branch_3`: Sets register d2 to zero and then branches to another location.
- `restore_registers_and_return_6`: Clears d0 and restores d2, d3, d4, d5, d6, and a2 from the stack before returning.
- `parse_numeric_string`: Library code that sets a byte at offset 30 of a structure to zero, calls another routine, and then either processes a string to calculate a numeric value or performs a series of calls and copies based on the string content.
- `prepare_stack_and_branch`: Library code that adds five to a register, pushes two values onto the stack, calls another routine, and then either branches to a new location or sets register d5 to one and branches elsewhere.
- `clear_d5_and_branch`: As library code, it clears register d5 and branches to another location.
- `process_numeric_string`: As library code, it performs a calculation involving a value from a register and a value from a pointer, then loops through a string, updating a value based on characters, and calls another routine to compare a byte at a pointer against a global variable before calling another routine twice.
- `compare_and_calculate_range`: Compares a value from the stack with the constant 48; if the value is greater than 48, it sets d0 to 48; if the value is less than 48, it sets d0 to 57; otherwise, it sets d0 to the value minus 48 and sets d1 to 48, then branches.
- `set_d0_to_negative_one`: Sets d0 to negative one and returns.
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
- `call_and_update_pointer`: This library code calls a library function, and if the result is non-zero, it writes 10 to an offset of 14 from the result, writes 10 to the memory at the pointer in param_2, and calls another routine using the incremented pointer.
- `restore_registers_and_return_3`: Clears d0 and restores d2, d3, and a2 from the stack before returning.
- `compare_library_result`: Calls a library function using a base from a global variable and an offset of -42, then returns that result if it does not equal the fourth parameter, otherwise it returns zero.
- `copy_d1_to_d0_and_return`: Copies the contents of d1 to d0, restores d2 and d3 from the stack, and returns.
- `compare_library_result_offset_48`: Calls a library function using a base from a global variable and an offset of -48, then returns that result if it does not equal the fourth parameter, otherwise it returns zero.
- `transfer_d1_to_d0_and_restore_registers`: Copies the contents of register d1 into register d0 and restores registers d2 and d3 from the stack before returning.
- `call_library_function_2`: Calls a library function at an offset of minus 198 from a base address stored in a global variable.
- `rotate_and_threshold_32bit_4`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_6`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit partial components and returns the result.
- `invoke_table_entry_4`: Loads a library base from a global variable and calls a function pointer located at an offset of 522 bytes from that base, passing three arguments from the stack.
- `store_byte_and_increment_2`: Stores a byte from d0 into the memory location pointed to by a3 and then increments a3.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_and_decrement_pointer`: Library code that loads a pointer from offset 4 and a byte value from offset 8 of the stack, compares the byte at the pointer to the value in d0, and if they are equal, decrements the pointer by 1 and returns that new pointer in d0; otherwise it returns the original value in d0.
- `branch_to_offset_5`: Library code that clears d0 and branches to a specific offset.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `skip_zero_byte`: Restores a0 and a1 from the stack, checks if the byte pointed to by a0 is zero, and if not, decrements a0 and branches back to the start of the routine.
- `compare_bytes_with_decrement`: As library code, this routine decrements a value from a structure, and if it is not negative, compares two bytes from two different memory locations and returns zero if they match and the first byte is zero.
- `set_conditional_d0`: Sets d0 to 1, and if d0 is not greater than zero, sets d0 to 255 before returning; otherwise it returns.
- `decrement_and_test_byte`: As library code, this routine checks a byte at the address in a0; if it is not zero, it decrements d0 and branches based on whether d0 is greater than zero.
- `decrement_and_call`: Decrements the address in a0 by 1 and calls another routine.
- `copy_bytes_with_limit`: Copies bytes from a1 to a0 while decrementing d0 in a loop, and if d0 reaches zero, it clears a single byte at a0 before returning.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `update_linked_list_node`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the value at that location by four, clears the following word, and stores the original pointer value into the word after that.
- `conditional_library_branching`: If the third parameter is non-zero, it calls another routine with offset 20, and if that result is non-zero, it calls two more routines and then calls a third routine using a value from a pointer at offset 10 of the third parameter; otherwise it returns one.
- `traverse_linked_list`: This library code enters a loop that checks if a parameter is zero, otherwise calls a routine with a specific constant, and if successful, calls another routine using a pointer and a value derived from an offset of the parameter before updating the parameter with a value from an offset of 12.
- `set_d0_and_branch_4`: Sets register d0 to 1 and branches to another location.
- `clear_d0_and_restore_registers_3`: Clears register d0 and restores registers d2 through d5 and a2 from the stack before returning.
- `process_nested_structures`: Library code that performs multiple recursive calls and stores a 32-bit value at an offset of 10 from a pointer returned by another routine.
- `return_negative_one_stub`: This library routine returns negative one after restoring registers from the stack.
- `iterate_nested_structures`: Library code that executes a loop to repeatedly call another routine, updating a value at offset 12 of a structure and storing a result at offset 8 of a structure before returning -1 if a call fails or the loop terminates.
- `execute_iterative_logic`: Executes a loop that calls another routine repeatedly and stores values at offsets of 8 and 12 from a pointer.
- `move_a3_to_d0_and_branch`: Moves the contents of a3 to d0 and branches to another location.
- `return_negative_one_extended_stub`: This library routine restores more registers from the stack and returns negative one.

### translator.library

- `initialize_library_structure`: Checks a global variable and calls a function through a pointer at an offset from that variable; if the result is not zero, it stores the global variable and a register value into a structure and calls another routine before returning.
- `clear_flag_and_increment_counter`: Increments a 16-bit counter in the structure it is given and clears bit 5 of that structure's flag byte.
- `decrement_counter_call_on_zero`: As library code, this routine decrements a 16-bit counter in the structure it is given.
- `test_and_set_bit`: Saves registers to the stack, loads a pointer from a global variable into a3, checks a bit at offset 32 of the structure pointed to by a3, and if that bit is set, sets bit 5 at offset 14 of that structure before branching.
- `unlink_node_and_call_library`: Removes the node from its doubly linked list, then calls a function through the Amiga library base and returns a value stored in the node.
- `return_constant_63`: Returns the constant an address in d0 to the caller without any other effect.
- `process_text_buffer_2`: Library code that initializes a stack buffer with spaces, then enters a loop to check for a hash character or spaces, performing various calls and data manipulations based on the characters found.
- `copy_word_and_branch`: As library code, it copies a 16-bit word from d0 to d1 and then performs a short branch.
- `clear_byte_and_return_word`: As library code, this routine calls another library routine, clears the byte at the address pointed to by a1 minus one, and returns the low 16-bit value of d1.
- `call_subroutines_conditionally`: Library code that loads a pointer from a structure, compares it to zero, and if non-zero, loads another pointer, calls two subroutines, and then branches based on the result of a comparison.
- `load_pointers_and_call_subroutines`: As library code, it loads two pointers from a structure, calls two different routines, and then branches based on the value in d7.
- `execute_conditional_library_calls_4`: As library code, it calls another routine, then conditionally calls three different routines based on the results of previous calls, while pushing and popping two pointers onto the stack.
- `check_byte_value`: Loads two pointers from the stack into a3 and a2, increments a6 and reads a byte into d7, then compares d7 to 92 and 96, branching if either match is found.
- `stub_immediate_return`: The decompiled body contains only a return, so the routine does no visible work in the library.
- `match_bracketed_tag_text`: Skips forward to an opening bracket character, then compares the characters that follow against a reference string until a closing bracket.
- `check_table_and_compare_byte`: As library code, this routine decrements a 16-bit value in d6, uses it to fetch a byte from a table, performs bitwise operations on that byte, and compares it to a byte in a second table.
- `set_condition_code`: Sets the carry flag and returns immediately.
- `call_and_branch_on_result`: Sets d7 to 1, calls another routine, and if the result is not zero, branches to another location; otherwise it clears the carry flag and returns.
- `match_string_with_escape_check`: As library code, this routine iterates through a string, checking for an equals sign and calling another routine if a specific bit is set in a 16-bit value at an offset determined by the current character.
- `call_and_branch_on_negative_result`: Sets d7 to 255, calls another routine, and if the result is not zero, branches to another location; otherwise it clears the carry flag and returns.
- `search_table_for_byte`: This library code routine loads a byte from a table at an offset determined by d6 and d7, then compares it against two constants, and loops through the table by decrementing d7 until it reaches zero or a match is found.
- `calculate_offset_and_branch`: Subtracts a value from a global variable at offset 8 from a6, adds a word from d7, compares the result to d2, and if the result is less than d2, performs a bitwise AND on d6, adds d6 to a6, stores d7 on the stack, and branches back to an earlier instruction; otherwise it performs the AND, addition, and stack operation before branching.
- `copy_counted_bytes_with_flag`: Copies a counted run of bytes from the source to the destination buffer, then sets the continuation flag bit and clears it again if the next source byte is a backtick.
- `truncate_text_at_break`: Walks backwards from the current position through a text buffer until it reaches a break character, then writes a zero byte there to end the string.
- `dispatch_on_special_character`: Searches a short fixed table of special characters for the byte in the data register and calls the handler at the matching position.
- `check_sign_and_load_byte`: Tests d7 and, if it is negative, branches to an earlier instruction; otherwise it loads a byte from the address in a2 into d1 and branches to an earlier instruction.

The function list continues in: g1450-recovery-functions-2.md.
