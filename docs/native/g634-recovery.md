# Game 634 (balance of power - the 1990 edition (1990)(mindscape)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00634). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916908 top-level bytes in 76 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 54 of 54 units identical (bytes and relocations), 59 raw-word lines |
| Functions named and described | pass, 2261 of 2261 named (278 author symbols), 2261 of 2261 described (427 by rule, 1707 by the local model) |
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
| Balance of Power - The 1990 Edition (1990)(Mindscape).adf:stack | 2 | 117 | 2 |
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
| Balance of Power - The 1990 Edition (1990)(Mindscape).adf:CLI | 9 | 362 | 18 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 15 | 1414 | 108 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2476 | 170 |
| mathieeedoubbas.library | 3 | 632 | 60 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 13 | 830 | 53 |
| Balance of Power - The 1990 Edition (1990)(Mindscape).adf:bop | 3 | 5064 | 305 |
| Balance of Power - The 1990 Edition (1990)(Mindscape).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1700 | 37 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 150 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape).adf bytes 470528-471040 (code never loaded by a reference run) | 1 | 0 | 0 |
| Balance of Power - The 1990 Edition (1990)(Mindscape).adf bytes 603136-603648 (code never loaded by a reference run) | 1 | 28 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape).adf bytes 684544-709632 (code never loaded by a reference run) | 1 | 0 | 0 |
| Balance of Power - The 1990 Edition (1990)(Mindscape).adf bytes 714240-715264 (code never loaded by a reference run) | 1 | 122 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape).adf bytes 867840-868864 (code never loaded by a reference run) | 1 | 0 | 0 |
| SavedGame (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: file devs/printers/Okidata_92 (header 5476 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1478 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 93 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 20 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: bytes 470528-471040 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: bytes 603136-603648 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: bytes 684544-709632 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: bytes 714240-715264 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: bytes 867840-868864 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf:asave/SavedGame: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: 5 logged loads cover 28160 bytes of the image
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: blocks 112-120 (bytes 57344-61952) outside the filesystem hold data, not code (opcode density 0.025, entropy 4.98)
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: blocks 1013-1013 (bytes 518656-519168) hold ASCII text
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: blocks 1298-1298 (bytes 664576-665088) hold ASCII text
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: blocks 1309-1312 (bytes 670208-672256) hold ASCII text
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: blocks 1314-1322 (bytes 672768-677376) hold ASCII text
- balance of power - the 1990 edition (1990)(mindscape)/Balance of Power - The 1990 Edition (1990)(Mindscape).adf: blocks 1325-1332 (bytes 678400-682496) hold ASCII text
- 4 never-loaded region(s), 219289 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Balance of Power - The 1990 Edition (1990)(Mindscape).adf:stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of register d1 to register d0 and then branches to another location.

### Format

- `parse_command_line_arguments`: Parses a string into a list of tokens by handling spaces, tabs, and quoted substrings, while performing character substitution for specific escape sequences.
- `execute_callback_chain`: Checks several global variables for non-zero values and executes corresponding function pointers stored at specific offsets.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `execute_callback_chain_3`: Loads a pointer into a1, clears d0, calls a routine at an offset from a6, stores the result in a global variable, and returns if d0 is not zero.
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
- `execute_callback_chain_2`: Checks five global variables for non-zero values and, if non-zero, calls the corresponding function pointer with specific offsets or no arguments.
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
- `pass_params_to_subroutine_2`: Calls another routine using a global variable, a parameter passed via the stack, and the address of an eight-byte stack slot, then returns.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a value from a stack slot or a negative error code.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_global_library_function`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `rotate_32bit_value_3`: If a value in a register is greater than 65535, it performs sixteen iterations of a loop that rotates a 32-bit value left by one bit and subtracts a value from the high word if it exceeds a threshold; otherwise, it performs modulo and division operations on the high and low words using a value from a register.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment`: Stores a byte from register d0 into the memory location pointed to by a3 and increments a3.
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

- `parse_command_string`: Parses a string into individual tokens by handling spaces, tabs, and quoted substrings, while performing character substitution for specific escape sequences.
- `execute_conditional_callbacks`: Stores a parameter into a stack-based location, then checks two global variables to conditionally call functions via pointers stored in a global structure.
- `invoke_two_function_pointers_2`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `call_conditional_callback`: Stores the result of a function call into a location at offset 8 of a structure, then branches to another routine if the result is zero.
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
- `execute_library_call_2`: Loads a value from the stack into d0, pushes a value from a global variable and d0 onto the stack, calls a routine at an offset from a global variable, and if the result is not zero, it branches.
- `perform_disk_copy_loop`: Iterates through a table of data blocks to perform disk I/O operations, including reading from a source and writing to a destination.
- `allocate_multiple_libraries`: Checks a value at an offset of 20 from a global variable; if it is less than 33, it calls a routine with a fixed address and then calls a routine with 0.
- `initialize_disk_copy_params`: Populates a stack-based structure with several constants and then calls two library functions using a global base.
- `report_disk_errors_2`: Sets a pointer to a global variable and calls another routine with a global value, a parameter, and a stack address, then repeats this with different parameters and calls another routine to handle error reporting.
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
- `allocate_message_entry_2`: Attempts to find a specific task and allocate a signal message buffer; if successful, it populates the buffer with a provided integer, a byte, a constant value of 4, a zero, and a task identifier, then stores a value from another routine at offset 16 of the buffer before dispatching the message.
- `delete_port_structure`: If a value at offset 10 of the passed structure is non-zero, it calls itself recursively; otherwise, it sets a byte at offset 8 to 255, sets a 32-bit integer at offset 20 to negative one, calls a routine to free a signal, and calls a routine to free the memory block.
- `create_ext_io_request`: If the first argument is non-zero, it calls a routine to allocate memory with a specific constant, then populates the resulting structure by storing the first argument at offset 14, the second argument at offset 18, the second argument as a 16-bit value at offset 18, and the constant 5 at offset 8.
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
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_and_increment_2`: Stores a byte from register d0 to the memory location pointed to by a3 and increments a3.
- `modular_inverse_32bit`: Performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag_2`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag_2`: Calls modular_inverse_32bit, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer`: Stores a byte from register d0 into the memory location pointed to by register a3 and increments the pointer by one before returning.
- `call_formatted_print`: Pushes a pointer and a value onto the stack and calls a routine to print a formatted message, then returns the result in a register.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_global_library_function_via_offset`: Calls a function through a pointer located at an offset of 348 from a library base address stored in a global variable and returns.
- `read_byte_or_fail`: Calls a routine through a global pointer requesting one byte into a 4-byte stack buffer; if the call returns 1, it returns the buffer's first byte sign-extended to an int, otherwise it returns -1.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.

### Port-Handler

- `parse_amigados_hunks`: Reads a structure containing hunk type codes and sizes, then iterates through a table to initialize memory segments based on whether the code indicates an executable, relocation, or data hunk.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a buffer, then enters a loop to initialize a bitmask and perform bitwise operations on data structures using several subroutine calls.

### Ram-Handler

- `execute_jump_table`: Sets d0 to 12, saves registers, calls a routine using a pointer from a6 at a negative offset, restores registers, and performs an indirect jump using a6.

### serial.device

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reinit_serial_buffer_fields`: Calls a routine via a pointer in a6 if a1 is non-zero, then updates several fields in a structure at a6 with a value from d2, including setting half-size and quarter-size values.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `set_flag_and_branch`: Stores the byte 253 at offset 31 of a structure in a1 and branches to another location.
- `toggle_interrupt_enable_2`: Writes 2049 to the INTENA register, calls another routine, and then writes 22401 to the INTENA register.
- `test_status_word`: Clears a byte at offset 31 of a structure in a1 and branches to different locations based on whether a word at offset 28 of that same structure equals 2, 3, or 10.
- `compare_and_clear_pointer`: Compares a longword at offset 74 of a structure in a6 with a pointer in a1; if they are equal, it clears bit 1 of a byte at offset 456 of the structure in a6 and clears the longword at offset 74 of the structure in a6 before branching to another routine.
- `unlink_list_node`: Checks bit 6 of a byte at offset 30 of a structure in a1; if set, it unlinks the structure from a list by updating pointers, decrements a 16-bit counter at offset 70 of a structure in a6, and branches to another routine.
- `reset_serial_hardware`: Compares a structure in a1 with a value at offset 78 of a structure in a6; if they match and a word at offset 28 of the structure in a1 is 10, it clears bit 5 of a byte at offset 470 of the structure in a6, writes values to hardware registers, calls another routine, clears a longword at offset 78 of the structure in a6, clears bit 4 of a byte at offset 472 of the structure in a6, and branches to another routine.
- `unlink_second_queue_request`: If a bit is set in a byte at offset 30 of a structure in a1, it unlinks that structure from a list, decrements a counter in a6, and calls a routine via a pointer in a6 if a bit is not set.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `set_status_bit_3`: Clears register d0, checks a word at offset 32 of a structure in a6, and if non-zero, sets bit 6 of a byte at offset 472 of the structure in a6 before returning; otherwise it returns.
- `invoke_library_vectors_then_unlink`: Calls several routines via pointers in a6 and a structure in a6, copies a longword from a structure in a6, and unlinks a block from a list by updating pointers.
- `insert_node_into_linked_list`: Inserts a new structure into a doubly linked list by updating pointers and increments a counter in a structure, then calls a handler routine.
- `update_serial_buffer_status`: Iterates through a linked list of data structures to process bytes, updating counters and status flags within a structure pointed to by a6, and calls process_serial_data_buffer_loop.
- `process_serial_data_buffer_loop`: Iterates through a linked list of data structures to process bytes, updating counters and status flags within a structure pointed to by a6.
- `test_bit_and_branch_3`: Tests bit 0 of a byte at offset 445 of a structure in a6 and branches to a later address if the bit is set, otherwise it branches to a different later address.
- `branch_on_call_result`: Calls another routine, and if the result is zero, writes the value 9 to a byte at offset 31 of a structure in a1 before branching to another routine.
- `process_serial_data_buffer`: Iterates through a linked list of data structures to process bytes, updating counters and status flags within a structure pointed to by a register.
- `restore_registers_and_return_6`: Returns immediately after restoring registers d2 and a6 from the stack.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `clear_bit_and_set_byte`: Clears bit 2 of a byte at offset 470 of a structure in a6; if that bit was set, it writes 15 to offset 31 of a structure in a1 and branches to another routine, otherwise it branches to another routine.
- `clear_bit_and_branch`: Clears bit 7 of a byte at offset 470 of a structure in a6 and branches to a later address if the bit was set, otherwise it writes the value 9 to a byte at offset 31 of a structure in a1 and branches to a later address.
- `update_status_and_mask`: Writes the value 12 to a byte at offset 31 of a structure in a1, tests bit 0 of a byte at offset 470 of a structure in a6, and if that bit is zero, writes the value 6 to offset 31 of the structure in a1, clears a longword at offset 62 of the structure in a6, and masks bit 4 of the byte at offset 470 of the structure in a6 before branching to another routine.
- `process_serial_data_buffer_2`: Checks status bits in a structure to manage serial data flow, updates a buffer pointer and a count when data is received, and calls another routine via a pointer in the structure.
- `toggle_serial_interrupt_state`: Compares a byte at offset 34 of the structure in a6 with a register, then tests bit 1 of a byte at offset 472 of the structure in a6; if bit 1 is set, it clears that bit and writes a value to the interrupt enable register to enable interrupts, otherwise it returns.
- `test_serial_bits`: Tests bit 5 of a byte at offset 472 of a structure in a6, then tests bit 1 of a byte at offset 475 of the same structure.
- `test_bit_and_branch_4`: Tests bit 8 of d0; if the bit is set, it branches to another routine, otherwise it branches to another routine.
- `test_bit_and_branch_2`: Tests bit 0 of a byte at offset 349 of the structure in a6 and branches to another routine.
- `set_bit_and_copy_pointer`: Sets bit 7 of a byte at offset 470 of a structure in a6 and, if a longword at offset 62 of that structure is zero, copies a longword from offset 42 to offset 62.
- `process_buffer_logic`: Compares two longwords in a structure in a6 and, if the second is less than the first, performs several increments and conditional pointer updates before calling a routine through a pointer stored in a structure.
- `record_pointer_then_signal`: Checks if a longword at offset 62 of a structure in a6 is zero; if it is, it copies a longword from offset 42 to offset 62, calls another routine, and then calls a library vector using a pointer from offset 98 of the structure in a6.
- `clear_hardware_bit`: Moves a byte from offset 34 of a structure in a6 to offset 473 of the structure in a6, clears bit 3 of a byte at offset 472 of the structure in a6, tests bit 2 of a word at offset 69 of a structure in a6, and if that bit is zero, clears bit 6 of the hardware register at address an address before branching to another routine.
- `update_interrupt_settings`: Moves a byte from offset 35 of the structure in a6 to offset 473 of the structure in a6, sets bits 3 and 6 of a hardware register, and conditionally writes 32769 to a hardware register before returning.
- `clear_byte_at_offset`: Clears a single byte at offset 473 of the structure pointed to by a6 and then returns.
- `update_status_byte_3`: Checks if bit 6 of a byte at offset 294 of a structure in a6 is clear; if so, it sets that bit, clears a longword at offset 418, copies a longword from offset 106 to offset 422, subtracts 10 from the value at offset 422, sets a word at offset 414 to 9, calls another routine, and if a byte at offset 47 of a structure in a1 is non-zero, sets a byte at offset 47 of that structure to 11.
- `check_interrupt_status`: Saves registers d2, a3, and a6 to the stack, loads a pointer from a1 into a6, clears bit 6 of a byte at offset 470 of the structure in a6, and if that bit was set, checks if the low 10 bits of the value in the SERDATR register are non-zero; if they are zero, it sets bit 2 of the byte at offset 470 of the structure in a6 and branches to another routine, otherwise it branches to another routine.
- `load_address_and_call`: Loads the address from offset 386 of the structure in a6 into a0, calls another routine, and then branches to another routine.
- `set_bit_and_branch_4`: Sets bit 0 of a byte at offset 470 of a structure in a6 and branches to another routine.
- `manage_request_queue`: Clears a byte at offset 30 of the structure in a1, conditionally updates a structure in a6 with values from a1 and a global variable, or inserts a1 into a linked list at offset 184 of the structure in a6, and finally calls another routine if a specific bit pattern is found in a byte at offset 30 of a1.
- `update_interrupt_state_and_registers`: Tests a bit in a structure, calls another routine, then checks a byte in a structure to conditionally set bit 4 of a byte at offset 30, clears a longword at offset 94, copies a longword and a pointer into offsets 86 and 78 of a structure, and enables interrupts by writing to two hardware registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_request_state_on_flags`: Checks if two specific bits in a global variable are set, and if so, writes a byte with a value of thirteen to an offset within a structure pointed to by a register.
- `manage_serial_request_queue`: Manages a linked list of serial request nodes by updating status flags, decrementing counters, and manipulating pointers within a structure.
- `update_status_and_pointer`: Writes 200 to offset 476 of a structure in a6, moves a pointer from offset 86 of a6 to a0, and clears d0 with the byte at a0.
- `update_counters_and_status`: Performs several operations involving bitwise logic on a structure in a6, increments two longwords at offsets 86 and 94 of that structure, and then enters a loop that compares bytes in a buffer starting at offset 110 of a6 against a value derived from a word in the structure, terminating the loop if a byte is less than or equal to the target value.
- `update_serial_status_and_call_handler`: Updates a status bitmask and increments two counters within a structure, then performs a loop to compare a byte against a value before calling another routine via a pointer.
- `process_linked_list_node`: Checks if a 16-bit value at offset 72 of a structure in a6 is non-zero; if so, it performs several operations involving a pointer from offset 176 of the structure in a6, including updating a linked list and modifying a byte at an offset of the pointed-to structure, and then calls another routine if a specific value is found.
- `set_interrupt_register_and_return`: Restores registers d2 and a6 from the stack, writes the value 1 to the hardware register at offset 154, and then returns.
- `transfer_serial_data`: Writes 1 to a hardware register, clears d1, moves a byte from offset 473 of the structure in a1 to d1, performs a logical OR with a word from offset 468 of the structure in a1 into d1, clears offset 473 of the structure in a1, writes d1 to a hardware register, and conditionally returns based on values at offsets 78 and 72 of the structure in a1.
- `update_serial_configuration`: Sets a global variable to 2049, performs conditional calls based on values at offsets 74 and 78 of a structure in a6, clears a byte at offset 69 of that structure using a mask, clears a byte at offset 472 of that structure, writes several longwords to a structure in a1, performs two more calls, sets a global variable to 22017, and calls a routine via a pointer if a bitmask test on a byte at offset 30 of a1 passes.
- `call_handler_and_branch`: Calls another routine and then performs an unconditional branch to another location.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `set_flags_and_call_helper`: Loads a longword from offset 78 of a structure in a6 into d0; if d0 is not zero, it sets bit 2 of a byte at offset 472 of the structure in a6; then it loads a longword from offset 74 of the structure in a6 into d0, and if d0 is not zero, it calls another routine before branching to another routine.
- `clear_interrupt_flag`: Clears bit 2 of a byte at offset 472 of a structure in a6, and if a longword at offset 74 of the structure in a6 is non-zero, calls another routine and writes 32769 to the interrupt request register; otherwise, it calls another routine and writes 32769 to the interrupt request register.
- `set_interrupt_and_call_routine`: Writes the value 2049 to the hardware register at offset 154, calls another routine, writes the value 34849 to the hardware register at offset 154, and then performs an unconditional branch.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `update_value_and_loop`: Copies a pointer from a0 to a1, reads a 32-bit value from the structure in a1, and if it is non-zero, writes that value to the structure in a0, exchanges d0 and a1, writes a0 to offset 4 of the new a1, and if d0 is non-zero, calls another routine before branching back to the start of this routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_status_and_dispatch_handler`: Updates a status byte and a 16-bit value in a structure via a pointer in a1 using values from a structure in a6, then calls a handler routine if a specific bitmask check passes.
- `test_request_status`: Clears bit 0 of a byte at offset 30 of a structure in a1, tests bit 3 of a byte at offset 79 of a structure in a1, and if that bit is set, checks if a word at offset 72 or a longword at offset 78 of a structure in a6 is non-zero.
- `handle_interrupt_request`: Moves a longword from offset 106 of the structure in a6 to d0, calls another routine, and if a byte at offset 31 of the structure in a1 is not zero, stores a1 into offset 78 of the structure in a6, sets bit 5 of offset 46 of the structure in a6, and writes 34816 to a hardware register before branching.
- `reply_and_clear_active_request`: Pushes a6 to the stack, updates a6 using a pointer at offset 98, calls a routine via a pointer, restores a6 from the stack, clears a 32-bit value at offset 78 of the structure in a6, and returns.
- `clear_uart_break_and_reply`: Clears bit 5 of a byte at offset 1d6 of a structure in a1, and if a specific bit is set in that byte, a non-zero value in a6, and a short at offset 1c of a pointer in a1 is 10, it sets bit 1 of the byte, clears a pointer in a1, writes 2048 to a global variable, calls a routine via a pointer, and sets a global variable to 32769.
- `start_timed_read_request`: Sets a global variable to one, clears a 32-bit value at offset 142 of a structure in a6, stores a 32-bit value from d0 at offset 146, sets a 16-bit value at offset 13e to nine, calls a routine via a pointer, and if a byte at offset 1f of a1 is non-zero, sets that byte to 11.
- `toggle_serial_flag`: Tests bit 7 of a byte at offset 79 of a structure in a1.
- `test_serial_status`: Clears bit 7 of a byte at offset 69 of a structure in a6, then branches to another location if bit 1 of a byte at offset 472 of the same structure is set, if a longword at offset 78 is non-zero, or if bit 2 of a byte at offset 30 of a structure in a1 is set; otherwise, it checks if a longword at offset 70 of the structure in a6 is zero and, if not, sets a byte at offset 31 of the structure in a1 to 1.
- `toggle_interrupt_enable`: Writes 2049 to the interrupt enable register, calls another routine, writes 34817 to the interrupt enable register, and if a byte at offset 30 of a structure in a1 has bits 0 and 6 set, it calls another routine.
- `validate_and_update_serial_parameters`: Validates a range of input bytes and a longword value within a structure, then copies several fields to a secondary structure and calls reinit_serial_buffer_fields.
- `compute_unit_channel_masks`: Calculates a 16-bit mask based on a byte at offset 4d of a1, modifies it based on a byte at offset 4e of a1 and a bit in a byte at offset 45 of a6, stores the result in a structure in a6, and calls two other routines to store results at offsets 1d2 and 1d0.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_byte_and_branch`: Stores the value 5 in a byte at offset 31 of the structure in a1 and branches to another routine.
- `calculate_period_quotient`: Loads a longword from offset 450 of the structure in a6 into d0 and d1, performs arithmetic operations including a division by a 16-bit value, and branches to another routine.
- `store_serial_period_quotient`: Divides a 32-bit value in d1 by a 16-bit value in d0, stores the quotient in a structure in a6 and a global variable, clears bit 5 of a byte at offset 1d8 of a6, and if a byte at offset 42 of a6 is 8 and bit 0 of a byte at offset 45 of a6 is set, it performs a bitwise OR on the quotient and sets bit 5 of the byte at offset 1d8.
- `configure_serial_device_parameters`: Retrieves a pointer from a table, populates a structure with values including a constant of 200, and performs multiple calls to initialize hardware settings.
- `initialize_serial_parameters`: Loads two addresses into a0 and a3 based on offsets of a6, calls a routine twice using those addresses, and then branches to another routine.
- `copy_pointer_to_structure`: Stores the value in a0 into a structure at offset 10 of a1.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `initialize_serial_structure`: Sets various byte and longword fields within a structure at offsets 300, 394, 426, and 640, calls a function through a pointer, and sets a global variable to 2049.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_d0_and_branch`: Clears d0 and branches to an earlier instruction.

### parallel.device

- `synchronize_device_status`: Checks specific bits in two structures, increments a 16-bit counter in one if a condition is met, copies a status byte and a 32-bit value between the structures, and sets a bit in a status byte.
- `increment_counter_and_copy`: Increments a 16-bit counter at offset 32 of the structure in a6, copies a byte from offset 117 of the structure in a6 to offset 53 of the structure in a1, sets bit 4 of offset 118 of the structure in a6, and conditionally copies two 32-bit values from the structure in a6 to the structure in a1.
- `update_status_and_copy_data`: Clears a byte at offset 76 of a structure in a6, conditionally sets bit 5 of offset 75, increments a 16-bit counter at offset 20 of a6 if two bytes are zero, copies a byte from offset 35 of a1 to offset 75 of a6, sets bit 4 of offset 76, and conditionally copies two 32-bit words from a6 to a1.
- `process_parallel_status`: Calls another routine if a bit pattern is met, conditionally modifies a global variable and a byte at offset 118 of the structure in a6 based on a 16-bit value at offset 28 of the structure in a1, and calls another routine via a pointer at offset 34 of the structure in a6.
- `handle_parallel_request`: If a short at offset 1c of a1 is 3, it clears bit 0 of a global variable and, if bit 0 is zero and bit 2 of offset 76 of a6 is not set, calls a routine via a pointer, then sets bit 2 of offset 76; finally, it calls another routine via a pointer passing a6 and a1.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `process_status_byte`: Writes the byte 253 to offset 31 of a structure in a1 and then calls another routine if a specific bit pattern is not met in a byte at offset 30 of the structure in a1; it also modifies a global variable and a byte at offset 76 of a structure in a6 based on the values in a1 and a6.
- `check_long_word`: Tests a long word at offset 36 of a structure in a1; if it is non-zero, it branches to another routine, otherwise it clears a long word at offset 32 of the structure in a1 and branches to another routine.
- `update_device_status_2`: Clears bit 1 of a byte at offset 30 of a structure in a1, then checks a bit in a global variable to either insert a structure into a linked list and increment a counter at offset 70 of a6, or modify bits in a byte at offset 30 of a1 and update several fields in a6 based on values in a1.
- `update_device_status`: Sets several fields in a structure, modifies a global variable, and performs bitwise operations on memory values.
- `process_device_event_queue`: Manages a linked list of pending device events by decrementing a counter, updating status flags, and performing a comparison loop through a data buffer.
- `update_device_counters`: Increments two counters in a structure, performs a comparison loop through a data buffer, and calls two external routines while clearing specific bits in status bytes.
- `modify_device_state`: Copies a longword from offset 46 of a6 to offset 32 of a1, clears bit 3 of a byte at offset 118 of a6, clears bit 4 of a byte at offset 30 of a1, calls a routine via a pointer, and if a word at offset 114 of a6 is zero, clears bit 2 of a byte at offset 118 of a6 and calls another routine.
- `test_cia_register`: Moves a byte from offset 16 of a structure in a0 to the CIA register at offset 101 of the hardware, then branches if the value is zero; it also tests bit 3 of a byte at offset 118 of a structure in a6 and branches accordingly.
- `dispatch_library_function`: Calls a function from a library via a pointer stored in a structure and then updates status flags and a counter within that same structure.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_if_not_zero`: Checks if a value in a structure is non-zero and calls a function through a pointer if it is.
- `invoke_callback_pointer`: Executes a function through a pointer stored in a structure if the associated index is not zero.
- `transfer_pointer_list`: Loads a pointer from offset 76 of a6 into a1, moves the value at a1 into d0, and if d0 is not zero, stores d0 at offset 4 of a1 and calls another routine; otherwise it stores d0 at offset 76 of a6.
- `clear_status_word`: Clears a 16-bit word at offset 72 of a structure in a6 and returns.
- `modify_status_byte`: Sets bit 5 of a byte at offset 30 of a structure in a1, sets a byte at offset 31 of the structure to 254, and calls a routine via a pointer stored in a structure in a6.
- `update_global_and_field`: Manipulates bits within a global variable and updates a byte in a structure using a library function call.
- `set_status_byte_2`: Tests bit 2 of a byte at offset 76 of a structure in a6; if it is clear, it sets a byte at offset 31 of a structure in a1 to 1 and branches to another routine.
- `copy_structure_fields`: Copies a byte from one structure to another and, if a specific bit is set, copies two long words as well.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `recurse_on_pointer`: Saves registers a0 and a6 to the stack, calls itself via a pointer, and then restores the registers and returns.
- `recursively_calls_itself`: Performs an infinite recursive call to itself.
- `dispatch_clipboard_handler`: Pushes the current stack pointer to the stack and calls a handler function via a pointer, then adjusts the stack pointer before returning.
- `jump_to_handler`: Performs a long jump to an address at offset 246 of a structure in a1.
- `decrement_and_check_counter`: Pushes a1 onto the stack, calls a routine, and then restores the stack pointer.
- `call_external_routine`: Pushes the contents of a1 onto the stack, calls another routine, and then increments the stack pointer by 4.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `update_clipboard_handler_pointer`: Copies a pointer from the stack to a global variable and calls another routine using a pointer from the stack, then stores the result in a different global variable.
- `process_clipboard_data`: Stores a value from d3 into a global variable, then calls three different routines via library vectors and a global variable, returning the result of the last call.
- `call_two_functions`: Calls two different functions via library vectors and returns one.
- `update_clipboard_buffer`: Calls a routine, and if the result is zero, clears a word at offset 94 of a1; otherwise, it checks a value at offset 4 of a pointer within a1, updates a longword at offset 14 of the result, and calls another routine.
- `clear_and_call_library`: Clears a longword at offset 14 of a structure in a3, pushes a pointer to a structure in a2 onto the stack, and calls a library function.
- `zero_word_field`: Stores a zero word into a specific offset within a structure and then calls another routine.
- `process_clipboard_string_data`: Loads a pointer from the stack, calls another routine, and if the value at offset 72 of the resulting structure is not zero, calls a routine to measure the string length and then calls a library function.
- `update_pointer_chain`: Pushes a2 onto the stack, calls a routine, clears a 32-bit value at offset 86 of a2, calls another routine, and then restores d2 and a2 from the stack.
- `copy_buffer_segment`: Copies a specific range of bytes from a linked list of 512-byte buffers into a destination buffer, updating the current buffer pointer in a structure.
- `compare_and_calculate_offset`: Compares a long word from a global variable with a value on the stack, then performs arithmetic involving a second stack value and a value from a structure to determine a new long word value.
- `copy_clipboard_data`: Traverses a linked list of structures to find a specific node based on a value derived from stack data.
- `compare_and_update_pointer`: Copies a longword from d1 to d0, compares d0 with a longword at offset 14 of a structure in a1, and if d0 is not lower than or equal to that value, updates a pointer in a1 and branches.
- `compare_buffer_lengths`: Copies a longword from d3 to d5, and if d5 is non-zero, it calculates the difference between 512 and a word in d2, compares this difference to d5, and if d5 is less than or equal to the difference, it moves the word in d2 to d4 and branches to another routine.
- `copy_data_loop`: Copies bytes from a table at offset 18 of a1 to a destination at a3 using a counter in d4, looping until the counter reaches 512, then performs arithmetic on d0, d2, and d5 before branching.
- `transfer_control_flow`: Moves a word from d2 to d4 and branches to another routine.
- `copy_data_with_limit`: Copies bytes from a table at offset 18 of a1 to a destination at a3 using a counter in d4, looping until the counter reaches a limit determined by the sum of d2 and d5, then clears d5 and branches.
- `store_long_word`: Stores a long word into a specific offset within a structure and then calls another routine.
- `update_clipboard_buffer_segments`: Iterates through a linked list of memory segments to synchronize or copy a source buffer into them, handling block boundaries by calculating offsets and performing byte-by-byte copies.
- `copy_buffer_to_linked_list`: Iterates through a linked list of memory blocks to copy a source buffer into them, handling block boundaries by calculating offsets and remaining byte counts.
- `call_subroutine_with_stack_adjustment`: Pushes a pointer from an offset of a structure in a2 onto the stack, calls another routine, adjusts the stack pointer by four, and then performs an unconditional branch.
- `copy_buffer_to_linked_list_2`: Navigates a linked list to find a memory segment matching a specific offset and then copies bytes from a source pointer into the segment's data area.
- `transfer_buffer_segments`: Navigates a linked list to locate a structure matching a specific value.
- `process_linked_list_buffer`: Iterates through a linked list of data structures, copying bytes from a source to a destination at offset 18 of the current structure, and updates a longword at offset 20 of a structure in a6 based on the remaining size.
- `validate_buffer_range`: Tests d4, and if it is non-zero, tests a longword at offset 20 of a structure in a6; if that longword is not less than or equal to zero, it calculates the difference between 512 and d3, compares it to the longword at offset 20, and if the longword is less than or equal to the difference, it moves the word in d3 to d1 and branches to another routine.
- `update_clipboard_buffer_2`: Calls a routine, and if the result is zero, clears a word at offset 94 of a1; otherwise, it checks a value at offset 4 of a pointer within a1.
- `transfer_control_flow_2`: Moves a word from d3 to d1 and branches to another routine.
- `copy_memory_buffer`: Copies bytes from a source pointer to a destination buffer in a loop, updating a counter and potentially calling another routine to fetch more data when a pointer reaches zero or a limit is reached.
- `store_register_to_offset`: Stores the contents of a data register into a memory location calculated by adding a fixed offset to a pointer held in an address register.
- `check_bits_and_call`: Checks bit 7 and bit 0 of a byte at offset 30 of a structure in a2, and calls another routine if bit 0 is not set.
- `modify_byte_bits`: Clears the least significant bit and sets the most significant bit of a byte at a specific offset in a buffer.
- `modify_byte_bits_2`: Checks bit 7 of a byte at offset 38 of a structure, clears bit 1 if the structure pointer does not match a second parameter, calls another routine, and then clears bit 2 of the same byte.
- `compare_structure_values`: Loads two pointers from a structure, checks if they are null, and if both are valid, compares a longword at offset 48 of the first structure with a longword at offset 48 of the second structure, returning 1 if the first is smaller.
- `set_zero_and_branch`: Sets d0 to zero and branches to another location.
- `zero_d0_and_branch`: Sets register d0 to zero and performs a short branch to another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_structure_fields_2`: Copies a long value from offset 40 of a second structure to offset 152 of a first structure, a word from offset 16 of the second structure to offset 146 of the first, and a long value from offset 48 of the second structure to offset 148 of the first, then calls another routine.
- `update_clipboard_status_flags`: Sets bit 5 of a byte at offset 30 of a structure and checks a byte at offset 66 of another structure; if the byte is not zero, it clears bit 5 of the byte at offset 30 and otherwise returns.
- `update_clipboard_buffer_state_machine`: Iterates through a loop to manage a clipboard buffer state machine by checking a status field in a structure.
- `clear_register_and_branch_3`: Clears a 16-bit register in d3 and branches to another routine.
- `update_clipboard_buffer_state_4`: Updates a clipboard buffer state machine by checking a status field in a structure and performing conditional updates to the buffer size, reallocating or adjusting memory offsets based on specific status codes.
- `push_pointer_and_call`: Pushes the address in a2 onto the stack, calls another routine, cleans up the stack, sets a 16-bit word at offset 94 of the structure in a2 to 1, and branches to another routine.
- `manage_clipboard_buffer_states`: Updates a clipboard buffer state machine by checking a status field in a structure and performing conditional updates to the buffer size, remaining bytes, and internal pointers.
- `clear_word_and_branch`: Clears the 16-bit register d3 and branches to another location.
- `update_clipboard_buffer_state_3`: Updates the clipboard buffer state by checking a status field in a structure and performing conditional updates to the buffer size, remaining byte count, and status flags.
- `update_clipboard_buffer_state_2`: Checks a status field in a structure to determine the current buffer state and performs conditional updates to buffer size, remaining byte counts, and status flags.
- `update_buffer_offset_2`: Loads a longword from offset 90 of a structure in a2 into d2, adds a longword from offset 36 of a structure in a4 and a longword from offset 44 of a structure in a4 to d0, compares d0 to d2, and if d0 is lower than or equal to d2, it moves a longword from offset 36 of a structure in a4 to offset 32 of a structure in a4 before branching to another location.
- `update_buffer_offset`: Loads a longword from an offset of a2 into d2, subtracts a longword at an offset of a4 from d2, stores the result at an offset of a4, adds the result back to the value at an offset of a4, and branches to another routine.
- `process_clipboard_buffer`: Clears a longword at offset 32 of a4 and sets a longword at offset 48 of a4 to negative 255; it then decrements a word at offset 110 of a2, calls another routine, and enters a loop that clears a word at offset 64 of a2.
- `process_clipboard_buffer_state`: Updates a buffer size counter in a structure based on the result of another routine and manages state transitions by writing specific status words to offsets 64 and 66 of the structure.
- `transfer_clipboard_data`: Copies a longword from an offset of a4 to an offset of a2, pushes a3 and a4 onto the stack, calls another routine, and then branches to an earlier instruction.
- `check_status_and_write_word`: Compares a word at offset 28 of a structure in a4 to 4, and if they are equal, writes the word 4 to offset 64 of a structure in a2.
- `clear_clipboard_word`: Clears the word in d3 and branches to an earlier instruction.
- `update_clipboard_buffer_state`: Checks a status word at offset 64 of a structure; if it equals 4, it retrieves a pointer from offset 112, updates a counter at offset 90 by increments of 8 or resets it to 0 based on the result of another routine, clears a 16-bit value at offset 110, and calls another routine with the retrieved pointer before resetting the status word to 0.
- `clear_clipboard_pointer`: Clears a longword at offset 90 of a structure in a2 and then branches to another routine.
- `process_clipboard_data_structure`: Iterates through a data structure to manage state flags and buffer pointers, performing conditional bitwise operations on bytes at offset 30 and 46 of various structures.
- `traverse_linked_list`: Traverses a linked list by following pointers until a null pointer is reached or a value at an offset of 12 in the current element is less than a value in a parameter, then calls another routine with the last valid pointer and a value from that pointer's offset 4.
- `set_d1_and_branch`: Sets register d1 to zero and branches to another location.
- `call_library_function_2`: Loads a library base from a global variable and calls a library function at a specific offset.
- `set_byte_value`: Writes a constant byte value to an offset within a structure.
- `set_byte_constant_offset`: Writes a constant byte value to a specific offset within a structure and then calls another routine.
- `set_byte_constant_offset_2`: Writes a constant byte value to an offset within a structure and then calls another routine.
- `write_byte_to_structure_and_call`: Stores a byte value from a register into a specific offset of a structure and then calls another routine.
- `set_byte_constant_offset_3`: Stores a constant byte value at a specific offset within a structure and then calls another routine.
- `update_structure_byte_sequence`: Calls three different routines to update a byte at a specific offset within a structure using either a constant value or a value from a register.
- `update_clipboard_buffer_3`: Checks bit 7 and bit 0 of a byte at offset 48 of a structure in a2 and calls another routine if bit 0 is not set, then calls a routine using a value from a register plus an offset of 96 and then calls a second routine.
- `call_subroutine_with_offset`: Calls another routine using a value from a register plus an offset of 96 and then calls a second routine.
- `increment_clipboard_counter`: Checks bit 7 and bit 0 of a byte at offset 48 of a structure in a2 and calls another routine if bit 0 is not set, then calls a routine using a value from a register plus an offset of 112 and then calls a second routine.
- `call_subroutine_with_different_offset`: Calls another routine using a value from a register plus an offset of 112 and then calls a second routine.
- `copy_structure_field`: Copies a longword from an offset relative to a pointer to another offset in a structure.
- `copy_structure_field_offset`: Copies a longword from a different offset relative to a pointer to a field in a structure.
- `search_and_allocate_clipboard_node`: Searches a linked list for a node matching a specific key; if found, it increments a counter and updates a pointer, otherwise it attempts to allocate a new node, initializes its members, and performs a series of checks and potential memory reallocations or buffer adjustments based on the node's size and status.
- `compare_and_clear_registers`: Compares a longword at offset 14 of a structure in a3 with d3; if they are not equal, it clears d4 and d2 and branches to another location.
- `manage_clipboard_buffer_allocation`: If a flag is zero, it increments a counter in a structure and performs recursive calls based on a byte value.
- `synchronize_clipboard_buffer`: Initializes a structure by setting several fields and calling a function pointer, then attempts to allocate and manage a memory buffer by iterating through chunks of data.
- `allocate_memory_and_update`: Pushes two pointers onto the stack, calls a library function, and if the result is not 512, clears a word at offset 94 of a structure in a3 and subtracts 512 from a value at offset -4 of a structure in a6.
- `clear_buffer_field_conditionally`: Checks a word at offset 94 of a structure in a3 and a longword at offset -4 of a structure in a6, then calls another routine and, if the result is not zero, clears the word at offset 94 of a3 and loops.
- `process_clipboard_buffer_contents`: Iterates through a linked list of data structures, clearing specific fields and calling another routine for each element based on a bitmask derived from a byte at offset 9.
- `process_complex_structure`: Increments values in a structure, performs conditional library calls based on a flag, and returns a value from a structure offset.
- `decrement_and_cleanup_clipboard_structure`: Decrements a 32-bit counter at offset 18 of a structure and, if it reaches zero, performs a recursive cleanup of linked list data, resets structure fields at offsets 20 and 24, and decrements a global byte, conditionally calling another routine if that global byte is zero and a specific bit is set.
- `compare_result_and_clear`: Pushes d4 onto the stack, pushes a pointer to an offset of a4, calls a library function, and if the result does not match d4, clears a word at offset 94 of a structure in a3, sets d4 to zero, and updates a4 from a pointer.
- `clear_byte_and_branch`: Clears a word at offset 94 of a structure in a3 and branches to another location.
- `cleanup_and_decrement_counter`: Clears two 32-bit fields at offsets 20 and 24 of a structure in a2, decrements a global byte, and if that byte is zero and a bit in a global variable is set, calls another routine.
- `cleanup_structure_data`: Resets two 32-bit fields at offsets 20 and 24 of a structure in a2 to zero, decrements a global byte, and conditionally calls another routine if the byte is zero and bit 3 of a global variable is set.
- `execute_conditional_sequence`: Tests a word at offset 252, and if it is non-zero, pushes a pointer from a global variable, calls another routine, pushes a pointer from a global variable, calls another routine, and moves a value from a global variable into d0.
- `set_global_bit`: Sets bit 3 of a global variable and returns zero.
- `update_structure_and_call_library`: Writes the byte value 5 to offset 8 of a structure, performs a bitwise AND on offset 30, and calls a library function if a comparison with 12 fails.
- `invoke_table_entry_3`: The routine invokes a function pointer retrieved from an offset within a global variable using an index.
- `check_bits_and_set_byte`: Checks if bits 7 and 6 of a byte at offset 30 of a structure in a2 are both clear; if so, it writes 254 to offset 31 of that structure, calls another routine if bit 5 is set, and calls another routine regardless.
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
- `update_linked_list_node`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the value at that location by four, clears the following word, and stores the original pointer value into the word after that.
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

- `execute_initialization_pointer`: Calls a routine through a pointer at offset -54 of a structure in a6, and if the result is non-zero, stores that result in a global variable.
- `execute_short_branch`: Loads the value 254 into d0 and calls another routine before returning.
- `manage_linked_list`: Sets a bit in a byte, performs a linked list insertion if a flag is set, and calls functions through pointers retrieved from a table.
- `execute_printer_initialization`: Calls a routine through a pointer at offset -54 of a structure in a6, and if the result is non-zero, stores that result in a global variable, sets a value at offset 34 of the result structure, and calls another routine through a pointer at offset -272 of a6.
- `initialize_printer_buffers`: Calls a library function three times, and if all return non-zero, calls another library function, fills a buffer starting at offset 406 of a structure in d0 with the value 45019 in a loop, and then calls two more library functions and sets a global variable to zero.
- `call_library_functions_repeatedly`: Calls a function pointer retrieved from a global variable three times, then calls another function pointer from a global variable once.
- `execute_status_updates`: Calls three different routines in sequence and then returns.
- `execute_short_branch_alternative`: Loads the value 253 into d0 and calls another routine before returning.
- `set_status_bit_2`: Sets bit 0 of a byte at offset 61 of a structure in a6.
- `clear_status_bit`: Clears bit 0 of a byte at offset 61 of a structure in a6.
- `loop_on_pointer_check`: Loads a pointer from an offset of 72 in a6 into a1, checks if the pointer is null, and if not null, calls another routine before branching back to the start of this routine.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_previous`: Branches to a previous location in the code.
- `initialize_and_process_buffers`: Initializes two buffer structures by setting pointers, counters, and offsets, then enters a loop that calls several library functions to check bits and potentially clears bits at offsets 1550 and 1551 of a structure in a6 based on values returned from library calls.
- `compare_and_clear_bit`: Compares d0 to a value at an offset from a6; if they are not equal, it branches to another location, otherwise it clears bit 1 of a byte at an offset from a6 and branches to another location.
- `process_linked_list_and_status`: Iterates through a linked list by reading pointers from a structure in a6, setting bit 5 of a byte at an offset from each element and calling a function via a table, then enters a loop that calls functions via a table and clears bit 0 or bit 1 of a byte at an offset from a6 based on the returned value.
- `call_subroutine_with_arguments`: Loads a pointer from offset 4 of the stack into a0, loads a value from offset 8 of the stack into d0, calls another routine, and then returns.
- `update_status_and_data_2`: Calls another routine, and if the result is zero, sets bit 0 or bit 1 of a byte at offset 1526 of a structure in a global variable, writes two 32-bit values to offsets 36 and 40 of a structure, writes the value 3 to offset 28, and calls another routine twice.
- `adjust_stack_and_branch`: Increments the stack pointer by 8 and branches to a previous location in the code.
- `set_status_bit_and_branch`: Sets bit 1 of a byte at offset 1558 in a6 and loads the address of a value at offset 190 in a6 into a1, then branches to a previous location in the code.
- `check_status_bits`: Loads a byte from an offset of a pointer in a0 into d0; if it is not zero, it checks if bit 0 or bit 1 of a byte at an offset from a6 is set, and if bit 0 or 1 is set, it performs a bitwise NOT and AND on d0 before returning.
- `process_status_loop`: Enters a loop that calls three different routines, then checks a global variable and a byte at offset 9b6 of a6 to conditionally call other routines or return a value.
- `modify_status_and_call`: Loads an address into a1 based on an offset from a6, compares d0 to that address, and if they are not equal, clears bit 1 of a word at offset 248 of a6 and calls another routine.
- `compare_and_call_routine`: Compares a long value at offset 272 of a structure in a6 with the value in d0, and if they are not equal, calls another routine; otherwise it returns.
- `call_and_branch_status`: Calls another routine and then branches to a previous location in the code.
- `update_structure_fields_2`: Writes 9 to offset 28 of a structure at a1, copies a 32-bit value from a pointer in a0 to offset 32 of a1, and clears a 32-bit value at offset 36 of a1.
- `clear_status_byte`: Sets a byte at a specific offset to zero and calls a routine through a pointer found in a structure.
- `check_status_byte`: Compares a byte at offset 8 of a structure in a1 to 7 and returns if they are not equal.
- `execute_callback_sequence`: Calls a routine through a pointer in a structure and then calls another routine using a value from a global base.
- `check_and_restore_a6`: Pushes a6 onto the stack, loads a pointer from an offset of a0, checks a byte at offset 31 of that pointer, and if it is non-zero, restores a6 from the stack and returns; otherwise it performs bitwise operations on a byte at offset 248 of a6.
- `process_event_queue_and_update_status`: Iterates through a list of events by calling other routines, clearing bit 1 or bit 0 of a status byte at offset 2466 of a structure in a6 based on the event type, and continues until a specific condition in the structure is met or a non-zero return value is received.
- `clear_bit_and_call_subroutine`: Compares a longword at offset 46 of a structure in a6 to the value in a1, clears bit 0 of a byte at offset 246 of the structure in a6, calls another routine, and then branches to another location.
- `compare_and_branch_subroutine`: Compares a longword at an offset of 272 from a6 with the value in a1, and if they are not equal, branches to another location; otherwise it calls a subroutine, tests d0, and branches based on whether d0 is zero.
- `branch_to_subroutines`: Calls a routine and then branches to another location.
- `check_byte_and_branch`: Saves registers d2, d3, a2, and a3 to the stack, compares a byte at offset 1529 in a6 to 1, and if not equal, branches to a previous location; otherwise, it pushes the current stack pointer and branches to another location.
- `update_interrupt_and_status`: Modifies a byte at an offset of a global variable, writes a value to a hardware register, and if a function call returns zero, clears bit 0 of a byte at an offset from a6 and calls another routine if bit 0 was set, then clears bit 1 of that same byte and calls another routine if bit 1 was set.
- `clear_status_bits_2`: Calls another routine, and if it returns 0, clears bit 0 of a byte at offset 1558 of a structure, calls another routine if bit 0 was set, then clears bit 1 of that same byte and calls another routine if bit 1 was set.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `update_list_and_copy_string`: Pushes the contents of a1 onto the stack twice, calls another routine, restores a1 from the stack, and calls another routine before returning.
- `reset_structure_field`: Sets a 32-bit field in a structure to zero and conditionally updates it from another field based on a routine return value.
- `update_list_and_copy_string_2`: Pushes the value in a1 onto the stack twice, calls a routine using a pointer from a table, restores a1 from the stack, calls another routine, and then returns.
- `execute_callback_and_notify`: Pushes two pointers from the stack to call a subroutine, restores the stack, retrieves a pointer, and calls another routine before returning.
- `dispatch_pending_entry_7`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a constant byte at specific offsets, then executes three routines through a global base using a pointer retrieved from that same structure.
- `call_mapping_routine`: Saves registers to the stack, loads a pointer from an offset of 8 in a6, and calls a routine at a fixed address before branching to another location based on the result in d0.
- `map_character_stream_to_table`: Calculates the length of a null-terminated string and then iterates through the stream to map characters into a destination table using a lookup index.
- `clear_and_move_word`: Moves a longword from a pointer in a3 to a global variable, clears a 16-bit word at a global variable, and then branches to another routine.
- `search_and_copy_character_stream`: Iterates through a character stream to find a specific character, then copies data from a source buffer to a destination table based on the found position.
- `clear_registers_and_branch`: Sets registers d4, d6, and d2 to zero and branches to another routine.
- `search_and_copy_character_stream_2`: Performs a recursive search through a character stream to find a specific byte, then copies bytes from a source table into a destination table using a loop.
- `copy_and_update_data`: Increments two counters and copies a byte from a source table to a destination table in a loop, then writes a 32-bit value to offset 32 of a structure based on whether a register is zero.
- `clear_memory_offset`: Stores a zero at a specific offset within a memory location pointed to by a register.
- `map_character_values`: Calls clear_and_branch_2, then enters a loop that compares a byte against a table of values using map_character_to_value_table and calls call_and_negate_on_flag_3.
- `clear_and_branch_2`: Clears a longword at a negative offset from a6 and branches to another location.
- `map_character_to_value`: Iterates through a loop to compare a byte against a table of values, performs complex conditional logic to determine a character to store in the address in a2, and then shifts elements in a global table.
- `conditionally_store_byte`: Compares a long value from a global variable to two and a value in d5 to twelve, then stores the byte ff at the address in a2 if both conditions are met.
- `compare_and_store_byte`: Compares a long value from a global variable to 2 and a value in d5 to 12, then stores the byte 75 at the address in a2 if both conditions are met.
- `update_status_byte_4`: Compares a long value in d5 to 9, and if d5 is not equal to 9 and the byte at the address in a4 is zero, stores the byte 59 at the address in a2.
- `copy_byte_from_table_3`: Compares d5 to 6, branches if d5 is greater than or equal to 6, otherwise moves d5 to d0, loads a table address into a0, and copies a byte from the table at offset d0 to the location in a2 before branching to another routine.
- `check_and_set_byte`: Tests register d5 and branches if non-zero, otherwise compares a byte at the address in a4 to 29 and branches if it is less than or equal to that value; if the comparison fails, it writes the byte 12 to the address in a2 and branches to another routine.
- `map_character_to_value_table`: Iterates through a table of seven values to compare against a byte, and if a match is found, stores a calculated value into a destination pointer.
- `parse_ansi_escape_sequences_2_2`: Iterates through a character stream to identify and process ANSI escape sequences, including control codes for line feeds and character selection.
- `parse_ansi_escape_sequences`: Iterates through a character stream to identify and process ANSI escape sequences, including control codes for line feeds, character selection, and parameter parsing.
- `parse_escape_sequence`: Processes a stream of characters by checking for escape characters and special control codes, such as line feeds or specific escape sequences.
- `check_escape_character`: Compares a byte at the address in a3 to 91 and branches to another routine if they are not equal, otherwise it branches to a different location.
- `parse_escape_sequence_stream`: Iterates through a data stream, handling escape characters and control codes to build a processed buffer.
- `handle_control_byte`: Compares a byte at the address in a3 to 10 and branches to another routine if they are not equal, otherwise it checks a byte at a global offset and, if non-zero, writes 3 to the address in a4 before branching.
- `process_printer_command_stream_2`: Parses a stream of bytes to interpret control characters and writes data to a destination buffer.
- `write_multiple_bytes`: Loads a pointer from the address in a2, increments that pointer, and writes the byte 10 to the address calculated by adding d2 to the original pointer; it then repeats this process by loading the incremented pointer, incrementing it again, and writing the byte 13 to the address calculated by adding d2 to the second pointer before branching to another routine.
- `write_single_byte`: Loads a pointer from the address in a2, increments that pointer, and writes the byte 10 to the address calculated by adding d2 to the original pointer before branching to another routine.
- `compare_and_call`: Compares a byte at the address in a3 to 155, and if it matches, pushes two values onto the stack, calls another routine, cleans the stack, and branches.
- `parse_file_path_string`: Parses a string to extract file path components, handling backslashes and drive identifiers, and stores the resulting characters into a destination buffer.
- `parse_path_string`: Parses a string containing directory separators and numeric components, converting character sequences into integer values and storing them in a destination buffer.
- `increment_and_branch_on_pointer`: Increments a pointer in a1 and a global long value, checks a byte at the pointer, and if it is zero, performs arithmetic on a value from a pointer in a2 to conditionally update a global long value and branch.
- `calculate_pointer_address`: Subtracts 160 from a byte at the address in a3, shifts the result left by 2, adds the value in d4 to it, and moves the resulting address into a1 before branching to another routine.
- `skip_non_numeric_characters`: Checks if a byte at a1 is 92, otherwise increments a1 and checks if the byte is less than 48 or between 48 and 57; it branches to another routine if these conditions are met, otherwise it loops back to the first check.
- `parse_numeric_string_to_buffer`: Reads up to three consecutive numeric characters from a source string, converts them into a decimal value, and stores the resulting byte into a destination buffer at an offset determined by an incrementing counter in a pointer.
- `copy_byte_to_offset_address`: Increments a pointer in a2, calculates an address by adding d2 to the original value in a2, moves a byte from the address in a1 to that calculated address, and branches to another routine based on whether the next byte in a1 is zero.
- `parse_ansi_escape_sequences_2`: Iterates through a character stream to identify and process ANSI escape sequences, including control codes for line feeds, character selection, and parameter parsing.
- `update_global_values`: Subtracts the difference between two global variables from a third global variable and clears a fourth global variable.
- `process_buffer_data_5`: Sets a global variable to zero, calls another routine, and if the result is positive, either copies a block of bytes from a stack buffer to a destination pointed to by a global pointer and increments that pointer in a loop, or updates a global variable and calls another routine or modifies a global variable based on a parameter.
- `update_and_branch_on_flag`: Adds d3 to a 32-bit value from a global variable, compares the result to 256, and if it is greater than 256, stores d3 in a global variable and checks a bit at offset 27 of a6; if that bit is non-zero, it calls another routine, otherwise it branches to another routine.
- `decrement_and_store_pointer`: Decrements a 32-bit value from a global variable by one, stores the result in another global variable, and branches to another routine.
- `clear_register_and_branch`: Sets register d4 to zero and branches to another routine.
- `copy_memory_block_2`: Copies a sequence of bytes from a source address to a destination address, incrementing a global pointer for each byte.
- `clear_byte_at_pointer`: Saves registers d2, d3, a2, and a3 to the stack, loads values from the stack into d3, d2, and a2, clears the byte at the address in a2, and branches to another routine.
- `clear_byte_at_offset_2`: Compares d1 to 59, and if they are equal, increments a3, adds the result to a2, and clears the byte at the resulting address.
- `set_pointer_and_value`: Compares d1 to 32, and if they are equal, loads a pointer from offset 32 of the stack into a0, sets d0 to 1, stores d0 at the location in a0, and branches to another routine.
- `compare_and_load_pointer`: Compares register d1 to 34, and if they are equal, loads a pointer from offset 32 of the stack into a0 and sets d0 to 2 before branching to another routine.
- `process_character_data`: Updates a byte at the address calculated by adding a3 to a2 by multiplying its current value by 10 and adding the difference between d1 and 48, then enters a loop that fetches bytes from another routine and performs conditional checks and memory writes based on the value.
- `increment_and_fetch_table_value`: Increments a value at a pointer, fetches a value from a table using the original value as an index, and if a second parameter is less than the incremented value or a global variable exceeds 255, calls another routine and sets the result to zero; otherwise it returns the table value.
- `return_register_value`: Returns the value currently held in a data register.
- `update_global_pointer_2`: Loads a 32-bit value from a global variable, decrements it by one, and stores the result in a global variable at offset 4 of a pointer held in a1.
- `decrement_and_check_table`: Decrements a value pointed to by a register and continues looping as long as the value is positive and a specific bit pattern is found in a table.
- `reset_global_state`: Clears three global variables, sets a byte at offset 430 of a structure to zero, and sets another global variable to 3.
- `copy_data_to_globals`: Copies six 16-bit values from a structure into a set of global variables.
- `compare_global_data`: Compares six 16-bit values from a structure against six global variables and returns 0 if all match, otherwise returns all ones.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `process_nested_parameters`: Extracts four bytes from a structure at an offset of 32, then performs a series of conditional calls to another routine using various combinations of the extracted bytes and integer constants.
- `dispatch_conditional_routines`: Calls another routine with parameters 1 and 0, and if the result is zero, calls another routine with a pointer from a structure in a6; if the first call returned zero and d2 is zero, it also calls another routine with parameters 3 and 0.
- `copy_formatted_string`: Calls a routine, then if the result is zero, checks a parameter to determine whether to set a global variable or copy a string from a table to a destination, handling specific character replacements along the way.
- `set_byte_from_offset_check`: Compares a word at offset 22 of the stack to 54 and 53, and if it matches 53, performs bitwise operations on a word from the stack and stores the result in a global variable.
- `copy_string_with_conversion`: Copies bytes from a source pointer to a destination pointer in a loop, converting the value negative two to zero and stopping if a null terminator or a negative one is encountered.
- `check_and_set_d1`: Compares the byte in register d1 to 255; if it is not 255, it branches to another routine, otherwise it sets register d1 to 255 and branches to another routine.
- `copy_string_data`: Copies bytes from a source pointer to a destination pointer in a loop, checking for a null terminator or a specific byte value, and handles special character cases by writing a byte from a pointer to the destination.
- `execute_jump_table_2`: Saves d2 to the stack, loads a 32-bit value into d2, compares d2 to 7, and if d2 is 7 or greater, adds d2 to d1 and jumps to an address from a table using d1 as an index.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `validate_and_prepare_print_buffer`: Extracts dimensions and pointer data from a structure, calculates buffer width and height by subtracting bounds, and validates that the requested print area fits within the available buffer.
- `calculate_clamped_buffer_dimensions`: Calculates the width and height of a buffer by subtracting minimum bounds from maximum bounds stored in a structure, then validates these dimensions against a set of required minimums and clamps the resulting values.
- `calculate_and_clamp_buffer_dimensions`: Calculates the width and height of a buffer from a structure, validates these dimensions against minimum required bounds, and clamps the resulting values using minimums retrieved from another structure.
- `validate_and_clamp_buffer_dimensions`: Validates buffer dimensions against a structure, calculates minimum values from two offsets in a structure, and updates a buffer with these limits while performing bitwise manipulations on a status mask.
- `update_buffer_limits_and_bitmask`: Calculates minimum values from two offsets in a structure pointed to by a data register and stores them in a buffer, then performs bitwise manipulations and conditional updates on two 32-bit integers based on a bitmask and a global state variable.
- `compare_and_branch_4`: Compares the word in register d0 to 2048; if they are not equal, it branches to another location, otherwise it performs a short branch to another location.
- `load_constant_and_branch_3`: Loads the value 12 into register d1 and performs a short branch to another location.
- `load_constant_and_jump`: Loads the value 15 into register d1 and performs a short branch to another location.
- `update_buffer_and_status_flags_2`: Calculates the minimum values from two different offsets within a structure pointed to by a pointer in a data register and stores them at offsets 10 and 14 relative to a6.
- `update_buffer_and_status_flags`: Calculates a minimum value from a pointer-based structure, updates a value at an offset of 14 before the address in a6, and performs complex bitwise manipulations and conditional updates on several global variables and values pointed to by a4 and d4.
- `update_offset_values`: Loads the values 6 and 7 into registers, stores 7 at an offset of 4 before the address in a6, checks a global byte and branches if it is non-zero, otherwise it adds 6 to d6, checks another global byte and branches if it is non-zero, or otherwise adds the value at the offset in a6 to itself and branches to another routine.
- `check_status_flags`: Clears a longword at a global address, then checks two different global bytes; if the first is zero, it branches to a different location, otherwise it checks a third global byte and branches if it is zero, or otherwise clears the stack and branches to another routine.
- `normalize_and_populate_color_tables`: Calculates the minimum and maximum values for two data sets, clears a 64-entry table, and populates it by extracting nibbles from values ret.
- `update_color_nibble_tables_2`: Calculates the minimum and maximum values for two data sets, clears a 64-entry table, and populates it by extracting nibbles from values retrieved through a loop.
- `update_color_nibble_tables`: Calculates the minimum and maximum values of two data sets, clears a 64-entry table, and populates three separate tables by deconstructing 16-bit values into four 4-bit nibbles.
- `populate_color_nibble_tables_3`: Calculates the minimum and maximum values of two data sets, clears a 64-entry table, and populates three separate tables by deconstructing 16-bit values into four 4-bit nibbles.
- `populate_color_nibble_tables_2`: Calculates the minimum and maximum values of two sets of data, clears a 64-entry table, and populates three separate tables by deconstructing 16-bit color values into four 4-bit nibbles.
- `populate_color_nibble_tables`: Clears a table of 64 entries and populates three separate tables by deconstructing 16-bit color values into four 4-bit nibbles.
- `generate_color_palette_table_2`: Clears a table of 64 entries, then populates it by deconstructing 16-bit color values into four 4-bit nibbles stored across three separate memory locations.
- `generate_color_palette_table`: Clears a table of 64 entries, then populates it by deconstructing 16-bit color values into four 4-bit nibbles stored across three separate memory locations.
- `convert_and_process_color_table`: Deconstructs a 16-bit color value into four 4-bit nibbles and stores them into a table, then performs bitwise operations and masking on a sequence of 64 longwords in a global array.
- `execute_callback_and_store_result_2`: Sets three global longwords to specific values, calls a routine via a pointer in a global variable, and stores the returned value into a global variable.
- `execute_callback_and_store_result`: Pushes three null longwords onto the stack, calls a routine via a pointer in a global variable, stores the returned value into a global variable, and branches to another routine based on the zero flag.
- `check_bytes_and_call`: Tests two bytes in global variables at offsets 342 and 344; if both are non-zero, it calls another routine and then branches to another routine; otherwise it branches to another routine.
- `test_and_call_printer`: Tests a byte in a global variable and, if it is non-zero, calls another routine and then branches to another routine; otherwise it branches to another routine.
- `check_and_call_printer_routine`: Tests a byte in a global variable and, if non-zero, calls another routine and then branches to another routine; otherwise, it branches to another routine.
- `call_and_store_result_3`: Calls another routine, stores the returned value from d0 into a global variable, and branches to another routine if the value is not zero, otherwise it branches to a different location.
- `check_modulo_or_call`: Checks if a global value modulo a 16-bit value from a structure is zero, or if a routine call returns 0, then calls another routine with the result.
- `call_empty_routine`: Calls another routine with two zero arguments and returns.
- `calculate_scaled_value_2`: If a global variable is zero, it returns the input parameter unless bit 2 of a byte at offset 21 of a global variable is set, in which case it finds the minimum of three bytes; otherwise, it calculates a value using a weighted sum of three bytes and optionally subtracts it from 15 based on a 16-bit value at offset 2656 of a structure.
- `find_minimum_byte`: Checks bit 2 of a byte at offset 21 of a global variable and, if set, finds the smallest byte value among three provided bytes and returns it, otherwise it returns the original second parameter.
- `calculate_printer_offset`: Calculates a value based on a global variable and several global constants, calls another routine with the result, and then updates several global variables based on the return value.
- `update_pointer_and_print`: Calls a routine to print a formatted message, then updates four global pointers by adding a global variable to them, and conditionally updates a pointer in a2 using values from a table or a global variable.
- `update_pointer_table_2`: Loads a longword from a global table using an index, stores it in the address held in a2, and then adds a global variable to four different global variables.
- `store_and_branch_logic`: Stores a longword from a memory location into the address in a2, loads a pointer from a global table into a0, tests bit 3 of a byte at offset 21 of that pointer, and then either branches or performs arithmetic on d1 before branching.
- `shift_and_compare_word`: Moves two words into d1 and d0, performs a logical shift right on d0, and branches based on the result of a comparison with one.
- `branch_on_value`: Compares a word in d0 to 2 and branches to another location if it is not equal, otherwise it branches to a different location.
- `decrement_and_loop`: Decrements a word in d0 and branches to an earlier instruction if the result is not zero, otherwise it branches to another location.
- `store_byte_and_branch`: Stores a byte from d1 into a memory location at offset 272 and branches to another location.
- `store_byte_and_branch_offset`: Stores a byte from d1 into a memory location at offset 274 and branches to another location.
- `transfer_pointer_and_call`: Moves a byte from register d1 to a specific memory location, pushes a 32-bit value from a pointer in a2 onto the stack, calls another routine, moves d0 to the location pointed to by a2, increments the stack pointer by 4, moves the value from a2 to another memory location, and branches to another routine.
- `update_pointer_table`: Loads a 32-bit value from a table using an index and updates four global pointers by adding a global base address to them.
- `calculate_and_update_pointers`: Calculates a value based on the product or sum of two globals and updates four global variables by calling another routine multiple times.
- `evaluate_multiple_conditions`: Sets a global variable to zero and then performs a series of conditional checks by calling multiple routines and checking their results.
- `evaluate_multiple_comparisons`: Calls three different routines using parameters from a table, and if any of those calls return a non-zero value, calls another routine with four parameters and stores the result in a location pointed to by a2.
- `evaluate_comparison_results`: Calls compare_table_or_value twice with different parameters, then calls compare_and_store_result based on the results, and stores the returned value into a location pointed to by a pointer in a2.
- `compare_and_store_result`: Calls another routine and, if the result is non-zero, calls a third routine with four parameters, stores that result in a location pointed to by a2, and returns that value if it is non-zero; otherwise it returns 0.
- `restore_registers_and_return_7`: Clears d0 and restores registers a2 through a4 from the stack before returning.
- `increment_and_check_modulo`: Increments a global variable and, if that variable modulo a value from a structure at offset 24 of a pointer in d1 is not zero, or if two consecutive calls to another routine both return zero, sets a local variable to zero; otherwise it returns the result of the calls.
- `execute_and_store_result`: Pushes three zeros onto the stack, loads a pointer from offset 24 of a structure into register a0, calls the routine at that pointer, stores the result from register d0 into the location pointed to by register a2, and branches to another routine if the zero flag is set, otherwise it branches to another routine.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `update_word_at_address`: Saves registers a2 through a4 to the stack and moves a word from a global variable into the address pointed to by a4, then branches to another routine.
- `execute_complex_logic_3_2`: Performs a series of nested loops and calls other routines to update a pointer's target, a global variable, and several other values, eventually returning a value from a pointer.
- `execute_complex_logic_2_2`: Calls accumulate_scaled_values, clears four global variables, and enters a loop that performs nested iterations and conditional calls, updating a pointer's target and a global variable based on the results of other routines.
- `clear_and_branch`: Clears a 16-bit word in a global variable and branches to another routine.
- `execute_complex_logic_2`: Performs a series of nested loops and conditional calls that update multiple global variables and pointers based on the results of other routines.
- `execute_complex_logic_3`: Increments a global variable and performs a series of calculations involving a global variable and other values; it then enters a loop that calls another routine and updates a value at a pointer.
- `increment_and_check_limit`: Adds a word from the address in a3 to a word at the address in a4, then compares the new word at the address in a4 to a constant at offset 300; if the result is lower than the constant, it branches to another location, otherwise it clears d0, restores registers a2 through a4 from the stack, and returns.
- `clear_and_update_word`: Saves registers a2 through a4 to the stack, clears a 32-bit word at a global variable, moves a word from a global variable into the address pointed to by a4, and branches to another routine.
- `execute_iterative_logic_2_2`: Executes a series of nested loops and calls other routines to update a global variable, several other values, and a pointer's target, eventually returning a value from a pointer.
- `clear_global_tables`: Clears four longwords at specific global memory locations, clears a word at a global offset, and branches to another routine.
- `execute_iterative_logic`: Executes a loop that calls another routine to determine a count, then performs a series of conditional calls and updates several global variables and pointers.
- `call_and_store_result`: Calls a subroutine, stores the returned value from d0 into the address in a2, and then either branches or loads the value from a2 into d0 before branching.
- `execute_iterative_logic_2`: Decrements a word in a global variable and enters a nested loop structure that calls another routine, clears four global variables, and performs iterative calls based on a value in a global variable.
- `increment_and_compare_word`: Adds a word from a pointer in a3 to a word at a pointer in a4, then compares the result against a constant value.
- `initialize_printer_buffer_parameters`: Clears a 16-bit value at a global offset, copies a 16-bit word from a global variable to a memory location, and branches to another routine after loading three constant values into registers.
- `accumulate_and_branch_back`: Pushes two 16-bit values from a structure into the stack, calls another routine, stores the result in the location pointed to by register a3, and branches back to an earlier instruction in this routine.
- `initialize_state_values`: Clears a word at a global variable, copies a longword from one global variable to another, copies a longword from another global variable to a third, moves a word from a global variable to d1, and branches to another routine.
- `process_conditional_calls_2`: Calls accumulate_scaled_values, clears four global variables, and enters a loop that performs nested iterations and conditional calls, decrementing a short value at a pointer's target and incrementing a word at another pointer's target.
- `process_conditional_calls`: Increments a 32-bit global variable, compares its lower 16 bits to a value, and if the comparison passes, calls two other routines and stores the result in a pointer in a2.
- `process_conditional_calls_3`: Increments a global variable and enters a loop that decrements a value at a pointer, calls another routine, and performs nested iterations and conditional calls based on the results.
- `decrement_and_check_limit`: Decrements a word at the address in a3, and if the result is not higher than unsigned, it branches to another location; otherwise it increments a word at the address in a4, compares the word at the address in a4 to a constant at offset 300, and if the result is lower than the constant, it branches to another location, otherwise it clears d0, restores registers a2 through a4 from the stack, and returns.
- `initialize_status_values`: Pushes registers a2 through a4 onto the stack, loads a2, a4, and a3 with specific constants, clears d0, writes d0 to offsets 316 and 340 of a structure in a2, writes a word from offset 380 of a structure in a2 to the address in a4, and branches to another location.
- `accumulate_and_branch_forward`: Pushes two 16-bit values from a structure into the stack, calls another routine, stores the result in the location pointed to by register a3, and branches to another routine.
- `copy_pointer_values`: Clears a word at a global variable, copies a longword from offset 238 to offset 234, copies a longword from offset 16 to offset 276, and moves a word from offset 378 to offset 314 before branching.
- `execute_scaled_logic`: Clears four global variables, then enters a loop that calls another routine to determine a count and performs conditional updates to a pointer's target and other globals.
- `store_result_and_branch_2`: Calls another routine, stores the returned longword into a memory location pointed to by a register, and branches to another routine if the result is zero; otherwise, it loads the stored value back into a register and branches to a third routine.
- `execute_scaled_logic_2`: Increments a global variable and decrements another, then enters a loop that performs conditional calls, updates a pointer's target, and clears four global variables.
- `decrement_and_check_bounds`: Decrements a word at the address in a3; if the result is not higher than zero, it branches to another routine, otherwise it increments a word at the address in a4, compares the value at a4 to a global variable, and either returns immediately or restores registers and returns.
- `accumulate_scaled_values`: Iteratively adds a doubled value to a pointer's target until the target is no longer less than one, then subtracts a doubled second parameter from that target.
- `compare_table_or_value`: If a global variable is zero, it compares a byte from a table using indices derived from two parameters; otherwise, it compares a signed byte with a value derived from subtracting a global variable from 15.
- `compare_table_value`: Performs a comparison between a byte retrieved from a table using bitwise-derived indices and a provided byte.
- `negate_boolean_results`: Calls two different routines and returns the negation of the first return value based on the status of a second return value.
- `process_large_value_with_rotation`: If a parameter is greater than 65535, it performs a loop that rotates bits and subtracts the parameter from a 32-bit value, otherwise it performs a division-based calculation.
- `rotate_32bit_value_2`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_7`: Computes the 32-bit product of two 32-bit values using 16-bit partial products and returns the result.
- `forward_call_to_helper_2`: Calls one helper and returns, with no other work.
- `call_helper_and_return_d1`: Moves the value from register d0 to d1, calls a helper routine, and returns the resulting value from d1.
- `negate_result_on_flag_4`: Calls process_large_value_with_rotation and negates the returned value if the helper leaves a non-zero result in an address register.
- `call_and_negate_on_flag_3`: Calls a routine and returns its first result, or the negation of that result if the second result in an address register is nonzero.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `process_pointer_chain`: Calls setup_pointer_chain with a pointer argument, then calls fetch_table_entry_and_branch and branch_to_offset_3.
- `fetch_table_entry_and_branch`: Shifts the value in d0 left by two bits, loads a pointer from a table at offset 14 using the result in a2, and then moves that pointer to d4 before branching.
- `branch_to_offset_3`: Performs a short branch to a location within another routine.
- `load_constant_and_branch_2`: Loads the value 32 into register a2 and branches to another location.
- `compare_and_update_printer_buffer`: Compares two null-terminated strings using a case-insensitive comparison routine, then if the strings match, updates a structure by checking a pointer at offset 86, potentially calling another routine, and subsequently copying a string into a global buffer after performing memory operations involving offsets 86, 90, 92, and 94.
- `manage_disk_operations`: Checks a pointer in a structure, calls a routine, and if a subsequent routine returns a non-zero value, performs a series of disk-related operations including a loop that copies a string and calls two other routines.
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

- `initialize_device_structure`: Calls three different functions via pointers from a global variable to populate fields in a structure, then calls another routine to populate further fields in a second structure, and finally calls two more routines via pointers from the global variable before returning.
- `test_and_modify_d0`: Tests the long value in d0; if it is zero, it branches to another routine, otherwise it sets d0 to negative five and branches to another routine.
- `manage_audio_device_state`: Checks a status flag in a structure; if zero, it attempts to initialize the audio device via function pointers, setting error codes and a global variable on failure, otherwise it increments counters and updates a secondary structure with various values and a pointer.
- `update_device_state`: Stores a value and a pointer into a structure, copies a long value from one offset to another, and calls a routine via a pointer.
- `update_device_state_fields`: The routine stores an input value into a structure, increments counters in the structure, copies a counter to another structure, writes several constant words and bytes to the second structure, and clears a flag bit in the first structure.
- `update_device_state_fields_2`: Stores a byte from d0 into a structure at offset 31 of a2, sets a long value at offset 20 of a2 to negative one, and branches to an earlier instruction.
- `process_linked_list_node_2`: Loads a pointer from offset 20 of a structure in a1 into a3, loads a value into a6, clears bit 0 of a byte at offset 30 of the structure in a1, and if bit 6 of a byte at offset 34 of a3 is set, it branches to an earlier instruction; otherwise, it checks a 32-bit value at offset 772 of a3, and if it is non-zero, it sets a byte at offset 31 of the structure in a1 to 248 and calls another routine.
- `manage_linked_list_2`: Compares a value in a structure to a value in another structure and, if they match, checks a bit and a byte before either calling a routine or performing a linked list insertion and updating a counter and a global variable.
- `copy_byte_and_split_nibbles_2`: Copies a byte from one structure to another, splits it into low and high nibbles into two bytes, clears a byte in the destination, and manages a linked list while modifying the INTENA register.
- `copy_byte_and_split_nibbles`: The routine copies a byte from one structure to another, splits it into low and high nibbles into two bytes, clears a byte in the destination structure, and calls another routine.
- `process_linked_list_and_update_state`: Traverses a linked list to find a specific node, decrements a counter in a global variable, and if a condition is met, updates several fields in a structure including bounds-checked values for offsets 30, 32, 34, 36, and 40, before calling clear_flags_and_call_handler.
- `decrement_and_disable_interrupts`: Decrements a byte in a global variable and, if the result is negative, writes a value to the interrupt enable register to disable interrupts before returning.
- `traverse_and_decrement_counter`: Traverses a linked list via a pointer in a structure, calling itself recursively if a value matches a field in the list node.
- `decrement_counter_and_close`: Decrements a 16-bit counter in a structure and, when it reaches zero, calls two routines via pointers from a global variable and optionally calls another routine if a bit is set in a structure.
- `set_byte_and_call_routine_2`: Loads a word from a structure, checks if it is between zero and eight, uses it as an index into a table to call a routine, and otherwise returns.
- `set_structure_bit`: Checks a word at offset 32 of a structure in a6, and if it is not zero, sets bit 5 of a byte at offset 34 of a structure pointed to by a3, then branches to another routine.
- `swap_pointers_and_return_value`: The routine calls library functions via a base pointer, swaps values between a pointer table and a data word, updates a pointer field, and returns a value from a table entry.
- `process_narrator_event_request`: Checks a status value in a structure to determine an action, then modifies flags in a global structure and either calls a handler via a pointer or recursively calls itself.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `update_status_and_call_handlers`: Clears bit 5 of a byte at offset 34 of a structure in a0, writes 7 to a word at offset 196 of a structure in a6, sets bit 0 of a byte at offset 198 of that same structure, calls two handlers via pointers stored in a structure in a6, and sets a byte at offset 31 of a structure in a1 to zero before returning.
- `set_flags_and_invoke_handler`: The routine sets a flag bit in a structure, writes a word and sets a bit in a global structure, calls a handler via a base pointer, clears a byte in another structure, and calls another routine.
- `set_flag_then_clear`: The routine sets bit 7 in a byte at offset 34 of a structure, calls two other routines, then clears bit 7 in the same byte.
- `set_flag_clear_byte_then_call`: Sets bit 7 in a byte at offset 34 of a structure, calls another routine, clears bit 7, clears a byte at offset 31 of a second structure, and calls another routine.
- `manage_interrupt_state`: Sets a 32-bit value in a global variable to 16384, increments a byte at offset 294 of that global variable, traverses a linked list via a pointer at offset 20 of a structure in a3 calling another routine for each node, then decrements the byte at offset 294 of the global variable and, if it was previously less than one, sets the global variable to 49152.
- `decrement_counter_and_set_global`: The routine decrements a byte in a global variable area and, if the previous value was less than one, writes a constant to a global variable.
- `enable_interrupts_and_set_bit`: Saves registers, updates a pointer in a structure, sets the interrupt enable bit in the INTENA register, increments a byte at offset 18 of a structure, and sets bit 5 of a byte at offset 30 of a structure if a comparison with a value at offset 772 of another structure fails.
- `update_pointer_from_table`: Loads a pointer from a structure in a0, compares it to a1, and if they are equal or if the loaded pointer is zero, it updates a0 with the value from offset 0 of the structure pointed to by a0 and branches to another routine; otherwise it branches to another routine.
- `update_pointers_and_decrement_counter`: The routine updates two pointers in a structure, writes a byte to a structure, decrements a byte in a global variable, and if the value is less than one, writes a constant to a hardware register and calls another routine.
- `decrement_counter_and_enable_interrupts`: Decrements a byte at offset 294 of a structure pointed to by a6 and, if the value was zero or less, writes 49152 to a global variable and enables interrupts via the INTENA register; otherwise it returns.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `set_byte_and_call_routine`: The routine writes a byte to a structure and calls another routine.
- `clear_byte_and_call_routine`: The routine clears a byte in a structure and calls another routine.
- `clear_flags_and_call_handler`: The routine clears a flag bit in a structure, clears a byte in another structure, and calls a handler via a base pointer.
- `sync_speech_metadata_and_scan_delimiters`: Copies several 16-bit and 32-bit values from a source structure to a destination structure after validating that they fall within specific numeric ranges.
- `update_threshold_and_load_value`: Compares a 16-bit word in d1 to 400; if it is greater than 400, it stores d1 at offset 30 of the structure in a5, otherwise it loads a 16-bit word from offset 50 of the structure in a2 into d1.
- `update_and_check_value`: Compares a 16-bit word in d1 to 320; if it is greater than 320, it stores d1 at offset 28 of a structure in a5; otherwise, it loads a 16-bit word from offset 52 of a structure in a2 into d1, checks if d1 is 0 or 1, and if not, moves 232 into d3 and branches to another routine.
- `update_narrator_status`: Stores a 16-bit word from register d1 at offset 48 of the structure in a5, loads a 16-bit word from the address in a2 into d1, and branches to another routine if d1 is 0 or 1, otherwise it loads 233 into d3 and branches to another routine.
- `store_and_compare_word`: Stores a 16-bit word from register d1 at offset 38 of the structure in a5, loads a 16-bit word from the address in a2 into d1, compares it to 4984, and branches to another location or sets d3 to 231 before branching to another location.
- `validate_and_store_text_offset`: Compares a 16-bit value in register d1 against 26464; if it is not greater, it stores d1 at an offset of 56 from a5, then loads a 16-bit value from an offset of 62 from a2 into d1, checks if it is non-zero, and if so, sets d3 to 230 and branches to another routine; otherwise, it returns.
- `scan_text_for_delimiters_2`: Scans a text buffer backwards from a specified offset to find a comma or hyphen, or searches for a space character to identify a word boundary.
- `parse_narrator_text_delimiters`: Scans a text buffer backwards from a specified offset to find a comma, hyphen, or space.
- `copy_long_word`: Copies a long word from offset 36 of a structure in a1 to offset 32 of the same structure and branches to another routine.
- `write_long_and_byte_then_call`: The routine writes a long word to a structure, writes a byte to another structure, and calls another routine.
- `scan_text_for_delimiters`: Scans a string backwards from a specified offset to find a comma, a hyphen, or a space.
- `process_offset_table`: Subtracts a shifted version of a long value from itself, compares a byte from a table at that offset against a constant, and if the result is non-negative, stores the offset in a structure and clears a long value.
- `shift_and_clear_word`: Loads a word from offset 150 of a structure in a5, shifts it right by one, stores the result in offset 156 of the same structure, clears the long word at offset 150, and branches to another routine.
- `initialize_speech_buffer`: Clears several fields in two structures, calls two routines, and if a condition is met, allocates a new structure, populates it with a global variable and several values from other routines, and then enters a nested loop to copy data from a table into a buffer.
- `transfer_speech_status`: Loads a longword from offset 24 of a structure in a1, stores the lower byte of it at offset 67 of a structure in a2, and calls a routine using an offset from a6.
- `copy_speech_data`: Sets several fields in a structure, calls two routines via pointers, and if a table lookup returns a non-zero value, enters a nested loop to copy words from a table into a buffer.
- `call_library_with_calculated_value`: Calculates a value by multiplying a doubled byte from offset 68 of a structure in a2 by 68, then calls a library function using a pointer at offset minus 198 of a6 with a constant in d1, and if the result is zero, sets d0 to negative 2 before branching to another routine.
- `copy_word_table_to_buffer`: The routine stores a pointer in a global variable and copies 34 words from a table in a global variable to a buffer pointed to by an argument, writing each word twice per iteration, repeating for a count derived from a byte in a global variable.
- `perform_bit_tests`: The routine performs several bitwise operations and bit tests on registers.
- `handle_io_and_free_resources`: Writes a byte and a word to a global variable, calls a routine via a global variable, conditionally calls two other routines via the same global variable, clears a word in the global variable and returns a combined value.
- `test_and_return`: Tests a 16-bit word at an offset of 78 from a5, and if it is not greater than zero, sets d3 to zero and returns.
- `parse_narrator_command_string`: Iterates through a string in a0 to decode command characters by looking up values in a table, updating several buffers at offsets 232, 744, and 1256 of a structure pointed to by a5, and managing a local index and counter.
- `modify_and_branch_on_d0`: Negates d1, adds 111 to it, and branches based on whether d0 is less than 12288 or not equal to 0; if it is not less than 12288, it adds d2 to a0 and branches.
- `decode_encoded_stream`: Parses a stream of bytes by performing lookups in a table to translate values, updates an indexed buffer and a counter, and writes results to three parallel arrays based on specific control characters.
- `parse_encoded_stream`: Parses a stream of encoded bytes by reading from a pointer, updating an indexed buffer and a counter, and performing lookups in a table to handle control characters and data values.
- `set_bit_if_not_five`: Compares the byte in d1 to 5; if it is not equal, it sets bit 5 of the byte at the address calculated by adding d3 to a6, then branches to another routine.
- `set_status_bit_4`: Compares the byte in d1 to 6, and if it is not equal, sets bit 2 of a byte at offset -1 of the structure pointed to by a6 and d3.
- `update_indexed_byte_and_counter`: Writes a byte from a register to a memory location at an offset determined by an index, increments that index, and checks if the byte is one or two; if the index is less than five hundred twelve, it subtracts a longword from a global variable at an offset from a register, updates a pointer in a structure, and sets the carry flag, otherwise it returns.
- `clear_buffer_bytes`: Subtracts a longword from offset 146 of a structure in a5 from a1, compares a1 to a 16-bit value at offset 156 of the structure in a5, and if a1 is greater than or equal to that value, it branches to another routine.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends immediately.
- `decrement_index_byte`: Loads a byte from a structure at an offset determined by a register, and if the byte is non-zero, decrements a byte at offset 3 of a structure and branches.
- `update_offset_and_counters`: Reads a long word from a table using an index derived from a4 and d0, and if bit 25 of that word is zero and d3 is greater than 3, it writes 4, 0, and 0 to three separate byte offsets in structures pointed to by a2, a3, and a6 using d3 as an index.
- `write_word_to_offset`: Writes a word from register d3 to an offset of 154 in the structure pointed to by a5 and returns.
- `update_status_bits_from_table`: Iterates through a string in memory and, for each character, checks a bit in a table; if the bit is set, it performs conditional bitwise OR operations on bytes at specific offsets within a structure based on further table lookups and character values.
- `set_status_bit_conditionally`: Performs a bitwise AND on d2 with 21504, and if the result is not zero, sets bit 4 of a byte at offset minus 3 of a structure in a0 before branching to another routine.
- `set_status_bit_on_match`: Loads a long word from a table using a2 and d1, masks it with 68608, and if the result is not zero and a byte at offset minus 2 of a structure in a1 is 48, sets bit 4 of a byte at offset minus 3 of a structure in a0 before branching to another routine.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `apply_text_attribute_formatting`: Iterates through a source byte stream to apply bitwise attribute transformations to a destination buffer, using a lookup table to determine formatting changes and control character handling.
- `process_narrator_text_stream`: Iterates through a source buffer in a0, performing bitwise transformations on a destination buffer in a1 based on control characters and lookup table values.
- `process_narrator_text_stream_2`: Parses a byte stream from a source buffer to apply formatting and attribute changes to a destination buffer.
- `calculate_offset_and_branch`: Subtracts a word from d2 from d0, subtracts 1 from the result, and branches to another routine.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `sync_buffer_with_table`: Iterates through a source buffer, comparing its bytes against a target table; if matches are found and specific bit conditions are met, it updates bytes in the target table using values from the source buffer and calls compare_and_patch_buffer or test_bit_with_table_lookup.
- `compare_and_patch_buffer`: Compares bytes from a source buffer against a target buffer using a series of conditional checks and bitmask tests, then optionally writes bytes from the source buffer to the target buffer.
- `increment_and_branch_on_byte`: Increments d1 by 1, loads a byte from offset 6 of the structure in a0, and branches to another routine if the byte is not 255; otherwise it calls another routine and returns if the carry flag is set.
- `test_bit_and_branch`: Tests bit 5 of a byte at offset 3 from the pointer in a0 and branches to another routine based on the result.
- `test_bit_with_table_lookup`: Loops reading a byte from a buffer and a word from a table indexed by a word, modifies the word based on bits in the byte, and exits when a specific bit is set or a termination condition is met.
- `check_word_limit`: Loads a word from offset 154 of a structure in a5 into d7, compares it to 512, and if it is less than 512, sets the carry flag and branches to another routine; otherwise it calls shift_bytes_and_clear_fields.
- `shift_bytes_and_clear_fields`: Increments a word counter in a structure and shifts bytes backward in three buffers by a computed count, then writes a byte from a register and two zero bytes to the new ends of the buffers.
- `execute_multiple_subroutines`: Calls seven different subroutines in sequence and then clears the d0 register before returning.
- `clear_byte_if_not_set`: Loads a pointer from offset 40 of a structure in a5 into a6, compares the byte at the pointer in a6 to 255, and if they are not equal, reloads the pointer from offset 40 of the structure in a5 and writes 160 to offset 7 of the structure in a6 before returning.
- `clear_byte_and_increment`: Clears a byte at offset 7 of a structure in a6, increments the pointer in a6 by 8, and branches to another routine.
- `check_data_end_marker`: Loads three pointers from a structure in a5 and a constant into a4, then checks if the byte at the first pointer is equal to 255; otherwise it returns.
- `process_encoded_stream`: Performs bitwise manipulations and data transfers based on values read from a table and a source, then increments the pointers in a0, a1, and a2 and branches to another routine.
- `process_encoded_data_2`: Processes a stream of data by reading bytes from a source, performing bitwise manipulations based on values retrieved from a table, and writing the results to a destination.
- `process_encoded_data`: Processes a stream of data by reading bytes from a source, performing bitwise manipulations based on values retrieved from a table, and writing results to a destination buffer until a terminator byte is encountered.
- `test_bit_and_modify_byte`: Tests bit 6 of d1 and branches to another routine if the bit is clear; otherwise it sets bits 1 and 2 of the byte at the pointer in a2 and branches to another routine.
- `set_bits_and_branch`: Tests bit 5 of d1 and branches to another routine if the bit is clear; otherwise it sets bits 2 and 3 of the byte at the pointer in a2 and branches to another routine.
- `set_bit_and_increment`: Sets bit 2 of a byte at the pointer in a2, increments the pointer in a2 by 1, and branches to another routine.
- `increment_three_pointers`: Increments the pointers in a0, a1, and a2 by 1 and branches to another routine.
- `calculate_masked_byte_sum`: Loads a byte from an offset of 1256 in a structure pointed to by a5 into d2, compares it to 255, and if not equal, masks the byte with 63 and adds it to d0 before branching to another routine.
- `allocate_memory_and_update_pointer`: Stores a long value from d0 into offset 58 of a structure in a5, shifts d0 left by 3 and adds 8, stores the result in offset 54 of the structure in a5, calls a routine via a pointer in a6, and if d0 is not zero, sets the 2nd word of the status register and increments a7 before returning.
- `update_structure_from_call`: Stores d0 into offset 40 of a structure in a5, checks bit 219 of a structure in a5, and if set, loads a long value from offset 58 of the structure into d0, calls another routine, and if d0 is not zero, stores d0 into offset 44 of the structure in a5 before returning.
- `expand_narrator_command_stream_2`: Iterates through a command stream, using each byte as an index into a table to retrieve structured data blocks and writing them to a destination buffer.
- `expand_narrator_command_stream`: Iterates through a command stream, using each byte as an index into a table to retrieve a structured data block and then writes a sequence of expanded data entries into a destination buffer.
- `parse_narrator_command_stream`: Reads a sequence of bytes from a source buffer, using each byte as an index into a table to retrieve structured data including three byte values and a control byte.
- `interpolate_buffer_data_3`: Iterates through a buffer, performing linear interpolation on bytes using values from a global table and a secondary table based on offsets derived from a pointer.
- `check_buffer_condition`: Shifts a value in d0 left by two, loads a word from a table using the result as an offset, and if bit 20 of that word is set, increments a pointer in a0 and branches to another routine; otherwise it branches to another routine.
- `interpolate_buffer_data_2`: Performs a linear interpolation between two values stored in a buffer and writes the result back to the buffer using a loop, then enters a sub-loop that processes data based on a byte value from a table.
- `interpolate_buffer_data`: Performs a linear interpolation between two values stored in a buffer and writes the result back to the buffer using a loop, then enters a second loop that processes a table based on values found in a byte stream until a terminator is reached.
- `increment_a6_and_branch`: Increments a6 by 1 and branches to another routine.
- `process_speech_stream_data`: Reads a stream of bytes from a buffer, using each byte as an index into a table to retrieve control parameters.
- `process_stream_byte_loop`: Loads a byte from a pointer in a1, masks it, and enters a loop that reads a byte from a table using an index in d7, tests bits 10 and 11 of the retrieved value, and branches accordingly.
- `process_narrator_stream_data`: Iterates through a data stream to perform lookups in tables and compares values, subsequently clearing or initializing a sequence of 4-byte structures in memory based on the results of the comparison loop.
- `compare_stream_bytes`: Loads a byte from a0 into d7, uses it to fetch a byte from a table in a4 into d2, compares d5 and d6, and if d6 is not greater than or equal to d5, it updates d7 and d3 from the source and a table before branching.
- `process_narrator_data_stream_4`: Iterates through a data stream, performing arithmetic comparisons between bytes and values from a table to update a structure in memory.
- `process_narrator_data_stream_3`: Iterates through a data stream, performing arithmetic comparisons between bytes and values from a table, and updates a structure in memory by writing zeroed bytes and specific constant values based on the results of these comparisons.
- `calculate_byte_difference`: Subtracts the values in d2 and d3 from the byte in d0, stores the result in d4, and then sets d1 to the value of d2 before branching to another routine.
- `update_pointer_and_loop`: Clears a 16-bit word in a6 and a byte at offset 2 of a6, checks bit 31 of d0, and if bit 31 is not set, copies d6 to offsets 3 and 4 of a6, increments a6 by 8, and loops using d1 as a counter; otherwise it sets d1 from d4 and branches.
- `increment_pointer_and_branch_2`: Increments a pointer in a6 by 8, sets a loop counter in d1 using a byte from d3, and then branches to another routine.
- `clear_and_copy_data`: Clears a 16-bit word at offset 0 and a byte at offset 2 of a structure in a6, then if bit 31 of d5 is not set, copies d6 to offsets 3 and 4 of the structure in a6, increments a6 by 8, and loops using d1 until it branches to another routine.
- `clear_registers_and_branch_2`: Sets d1 to the byte in d0 and clears d4 and d3 to zero before branching to another routine.
- `copy_filtered_buffer_data`: Loops through a buffer, checking a bit in a global table for each byte, and if the bit is not set, copies the byte and the corresponding words from two other buffers into destination buffers until a termination byte is reached.
- `write_byte_to_three_pointers`: Writes the same byte from a register to the memory locations pointed to by three registers.
- `call_subroutines`: Calls two separate subroutines and then returns.
- `mask_stream_byte`: Compares a byte at an offset of 648 from a structure in a5 with a constant; if they are not equal, it performs a bitwise AND on that byte using a constant mask and then branches to another routine.
- `check_stream_byte`: Loads two pointers from offsets 232 and 744 of a structure in a5 and a constant address into a0 and a4, then reads a byte from the address in a0 and returns if the byte is not 255.
- `decode_bitstream_data`: Iterates through a data stream, using bytes to index into a table and performing complex bitwise updates to a destination buffer based on the retrieved table values and specific byte markers.
- `set_bit_and_branch_3`: Sets bit 0 of a byte at offset -1 of a1, checks bit 7 of d1, and if bit 7 is not set, compares a byte from a0 to 24 or 23, and if neither matches, sets bit 1 of a byte at a1 and moves a1 to a3.
- `process_bitmask_stream`: Sets bit 1 of a byte at the address in a1, then enters a loop that iterates through a byte stream, performing bitwise operations on bytes and values retrieved from a table in a4 based on the stream content, and calls set_bit_and_branch when certain conditions are met.
- `set_bit_and_branch`: Sets bit 2 of a byte at the address in a3 and branches to another routine.
- `set_multiple_bits`: Sets bit 0 of a byte at offset -2 of the structure in a1, then sets bits 1 and 2 of the byte at offset -1 of the structure in a1, and branches to another routine.
- `decode_variable_length_bitstream`: Decodes a variable-length bitstream from a source buffer to populate a destination buffer with values derived from dividing a calculated constant by bytes read from a lookup table.
- `process_bitstream_value`: Masks the lower 6 bits of d0, tests bit 0 of a byte at the address in a2 and increments a2, and if the bit was set, shifts d0 left by 3 and adds it to a6 before branching to another routine.
- `decode_variable_length_stream_3`: Decodes a variable-length bitstream by reading bytes, masking bits, and performing arithmetic scaling on values to write bytes into a destination buffer.
- `decode_variable_length_stream_2`: Clears d0, loads a byte from a1 and increments a1, masks the byte with 63, checks bit 1 of a byte at a2 and increments a2, and if bit 1 is set, shifts d0 left by 3, adds it to d1, and loops.
- `decode_variable_length_stream`: Iterates through a bitstream using two synchronized pointers to decode variable-length data, performing division operations on a value from a structure and storing the results into a destination buffer.
- `divide_and_update_pointers`: Swaps the high and low words of d5, adds d6 to d5, divides d5 by a 16-bit word from offset 32 of a structure in a5, stores the result in a byte at offset -1 of a structure in a6, decrements a1 and a2, and branches to another routine.
- `update_byte_from_division`: Calculates a value by dividing a word in d7 by 110, then writes this byte to a location in a structure and branches based on whether the byte is 255.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `traverse_narrator_data_structure`: Iterates through three parallel data streams using pointers in a5, performing bitwise tests on values fetched from a table using an index derived from the first stream.
- `test_bit_flags_2`: Shifts d5 left by two, loads a longword from a pointer in a4 offset by d5, and performs bit tests on that value to decide whether to branch or load a constant into d6.
- `increment_three_pointers_2`: Increments a1, a0, and a3 by 1 and branches to another routine.
- `parse_text_stream_control_codes`: Iterates through a byte stream, checking bit flags in registers and specific bit patterns in a table to determine how to process the next character.
- `test_bits_and_call_subroutine`: Tests bit 12 of d7 and branches if it is set, then tests bit 9 of d7 and branches if it is set; otherwise, it loads 38 into d1, calls another routine, and branches.
- `test_bit_and_branch_5`: Tests bit 10 of d7; if it is not set, it loads 38 into d1, calls another routine, and then branches to another location, otherwise it branches to another location.
- `test_bits_and_branch`: Tests bit 16 of d7 and bit 2 of the byte at offset 1 of a3; if bit 16 is set and bit 2 is not set, it loads 27 into d1, calls a subroutine, and branches to another location.
- `test_bit_and_call_stream_processor`: Tests bit 11 of a register and, if set, loads the value 22 into a register, calls another routine, and then branches to another location; otherwise, it branches to another location.
- `process_text_stream_with_scaling`: Reads a stream of bytes and performs bitwise transformations and arithmetic scaling based on values in a lookup table.
- `test_bit_flags`: Tests bit 1 of d6 and d7, and if both are set, loads 16 into d1, calls a subroutine, and branches to another location; otherwise it branches to one of two other locations.
- `call_subroutine_and_branch`: Sets d1 to 22, calls a subroutine, and then branches.
- `decode_text_stream`: Reads a stream of bytes from a source buffer, performs bitwise transformations and arithmetic scaling based on values in a lookup table, and writes the resulting characters to a destination buffer.
- `compute_scaled_shifted_product`: The routine multiplies the low word of an input register by a word input and returns a combined value formed from the high word of the product and the low word shifted right by five bits after adding a constant.
- `initialize_and_clear_buffers`: The routine stores several offsets relative to a base pointer into a global structure, increments a few of those pointers, clears a set of word fields, then zeroes six blocks of memory by iterating a fixed count.
- `increment_and_call_handlers`: Increments a 16-bit word at offset 136 of a structure in a5, then calls up to four subroutines based on the zero and overflow flags, and finally clears the condition code register before returning.
- `process_compressed_stream`: Processes a stream of data by checking for an end marker, reading values from a table, and performing arithmetic scaling and bitwise operations to write values to a destination buffer while updating various counters and indices.
- `process_compressed_stream_byte`: Tests a byte in d1 and branches based on its sign; if negative, it checks bit 7 of a byte at an offset from a pointer in a4 and d4, then checks bit 5 of d1.
- `decode_compressed_stream`: Decodes a stream of data by reading bytes from a source buffer and applying a transformation involving bitwise operations and a lookup table.
- `set_flag_and_store_indices`: The routine ORs a constant into a byte addressed by a base pointer and an index, then stores two short values derived from an index and a conditional copy into fields of a global structure.
- `process_narrator_text_stream_3`: Iterates through three synchronized buffers to parse control codes and update a text display buffer.
- `check_bit_and_branch`: Shifts a word in d3 left by two, loads a long word from a table using d3 as an offset, and branches based on bit 13 of that word or the sign of d1.
- `modify_byte_via_flag`: Checks bit 4 of a byte at an offset from a pointer in a6, performs bitwise operations and shifts on a byte from a memory location, and if the resulting value is between 254 and 255, sets bits 3 and 4 of that memory location before branching to another routine.
- `decrement_and_store_pointers`: Decrements three pointers in a0, a1, and a2 by 1, stores these pointers into offsets 124, 132, and 128 of a structure in a5, sets d7 to 0, moves the word in d4 to d0, and branches to another location.
- `update_byte_with_conditional_mask`: Performs a bitwise OR of d7 into a byte at an offset from a4, moves d0 to d2, and compares a byte at an offset from a6 with 2 or 14.
- `clear_byte_on_condition`: Checks bit 5 of a byte at an offset from a pointer in a4, and if the bit is not set, increments d2 and, if d2 is non-negative and less than d4, clears a byte at an offset from a6 before branching to another routine.
- `swap_and_clear_byte_flags`: Swaps a byte between two positions in a table addressed by a base pointer and an index, clears the source byte when the positions differ, and then decrements an index using a loop.
- `load_pointers_for_loop`: Loads a pointer from an offset of a5 into a1, moves a 16-bit value from d4 to d0, and branches to another routine.
- `set_bits_in_loop`: Loops through a range of memory defined by a pointer in a1 and a count in d0, setting the lowest bit of each byte.
- `or_mask_into_buffer_with_early_exit`: The routine selects a mask based on a control byte, then ORs that mask into a series of bytes in one buffer while checking a flag in another buffer and returning early if the flag is set.
- `set_tile_and_mask`: Updates a byte in a screen buffer using a mask derived from a map byte, and optionally masks and sets a flag in another byte based on a scan leftwards in a second buffer.
- `accumulate_table_entries`: Reads a 16-bit value from offset 140 of a structure in a5, conditionally calls other routines based on values at offsets 48 and 138, and then adds the 16-bit value to eight consecutive longwords in a table starting at offset 92.
- `clamp_value_and_store_char_2`: Clamps a short input to a lower and upper bound and stores the resulting byte at an address formed from a base pointer and an offset.
- `clamp_value_and_store_char`: The routine clamps a short input to a lower and upper bound and stores the resulting byte at an address formed from a base pointer and an index.
- `adjust_table_values_loop_2`: Iterates through a table to modify byte values based on scaled differences between entries, applying a smoothing algorithm and bitwise adjustments to a destination buffer using values from two source tables.
- `adjust_table_values_loop`: The routine repeatedly computes an absolute difference between two table entries, scales it and adds the result to a destination byte, then scans backwards through a range using flag bits to locate entries.
- `compute_scaled_char_values_2`: Loops through a range of indices, using flag bits to select entries and computing scaled character values from two tables with multiplication and bit shifting, then writes the results to two different destination buffers.
- `scale_and_store_byte`: Adds 64 to a value in d2, shifts it right by 7, and adds the result to a byte at an offset from a pointer in a3 using d0 as an index.
- `compute_scaled_char_values`: The routine loops over a range of indices, using flag bits to select entries and computing scaled character values from two tables with multiplication and shifts.
- `load_pointers_and_branch`: Loads two pointers from offsets 104 and 112 of a structure in a5 into a0 and a1, moves a word from offset 158 of the structure into d1 and then d0, and branches to another location.
- `apply_audio_filter_coefficients`: Iterates through a buffer to apply gain and offset adjustments to audio samples based on values in a control table.
- `modify_arithmetic_value`: Adds 64 to d2, shifts it right by seven, and adds the result to a byte at an offset from a3.
- `transform_arithmetic_value`: Adds 64 to d2, shifts d2 right by 7, adds the result to a byte at offset d1 from a3, clears d2, loads a byte from a6 at offset d0 into d2, subtracts 110 from d2, multiplies d2 by -214, and then either branches to a location or subtracts 64 from d2 and branches to another location based on the sign of the result.
- `modify_byte_via_arithmetic_2`: Adds 64 to d2, shifts it right by 7, and adds the result to a byte at an offset from a6 using d0 as an index, then loads a byte from a6 at offset d1, subtracts 110, multiplies by 26, and branches based on the sign.
- `modify_byte_via_arithmetic`: Adds 64 to d2, shifts it right by 7, and adds the result to a byte at an offset from a pointer in a6 using d1 as an index.
- `add_byte_and_branch`: Adds a byte from d2 to a byte at an offset from a pointer in a2 and then branches to another routine.
- `process_audio_sample_buffer_2`: Iterates through a buffer to apply gain and offset adjustments to audio samples based on values in a control table.
- `transform_audio_sample_buffer_2`: Adds 64 to d2, shifts it right by 7, and adds the result to a byte at an offset from a3 using d0 as an index, then loads a byte from a3 at offset d1, extends it to a word, multiplies it by d7, and branches based on the sign.
- `transform_audio_sample_buffer`: Iterates through an audio data buffer, applying scaling, offset, and bitmask-based transformations to each byte.
- `process_audio_sample_buffer`: Iterates through a buffer of audio data, applying scaling and offset transformations to bytes based on bitmask checks in a control table.
- `process_audio_sample_buffer_3`: Iterates through a buffer to apply gain and offset adjustments to audio samples based on values in a control table.
- `calculate_scaled_value_3`: Loads a byte from a table using an offset in d0 and a pointer in a2, multiplies the sign-extended result by 102, and then either subtracts 64 or branches to another location based on the sign of the result.
- `update_destination_byte`: Adds 64 to d1, shifts d1 right by 7, adds the result to a byte at the address in a3 plus the offset in d0, and branches to another location.
- `adjust_tile_heights_with_mask`: Iterates over a byte range using a mask in a global variable and a table pointed to by a register, updates a byte in a destination buffer from a source buffer with a scaled difference based on a running maximum, and writes an accumulated scaled value to another buffer.
- `load_buffer_pointers`: Loads two pointers from a structure using offsets 104 and 104 from a5 into a2 and a0, clears d1, moves the value in d4 to d0, and branches to another location.
- `apply_scaled_offset_to_buffer`: Iterates backwards through a buffer, checking if the high nibble of a byte is non-zero, and if so, modifies a byte in a destination buffer by adding a value derived from a source byte multiplied by either negative 102 or positive 38, adjusted by a constant offset.
- `scale_buffer_by_signed_factor`: Adds 64 to d1, shifts d1 right by 7, adds the result to a byte at the address in a2 plus the offset in d0, and loops using d0 as a counter until it returns.
- `process_audio_buffer_mask_2`: Iterates through a data stream to copy bytes between buffers while applying bitwise masks and arithmetic adjustments to values at specific offsets.
- `copy_and_modify_bytes`: Iterates through a loop, copying a byte from one location to another, performing a subtraction with a value in a register, and then adding 5 to the resulting byte in a destination buffer.
- `process_audio_buffer_mask`: Iterates through a data stream to copy bytes between buffers while applying bitwise masks and arithmetic adjustments to values at specific offsets.
- `update_buffer_with_masked_value`: Iterates through a loop, copying a byte from a source buffer to a destination buffer, and then adds a value to a byte in a third buffer that is derived from a masked and shifted byte from a fourth buffer.
- `load_address_and_call_2`: Loads the address stored at a pointer in a global variable into a4, calls another routine, and then branches to another location.
- `load_address_and_branch_5`: Loads a long value into a4, calls another routine, and then branches to another location.
- `load_pointers_and_constants`: Loads a pointer from an offset of a5 into a4, sets d6 to 110, and moves d5 into d0 before branching.
- `transform_text_buffer_and_scale_values`: Iterates through a buffer to perform arithmetic on bytes and copies them to multiple destinations.
- `process_text_buffer_2`: Decrements a counter in d0 while checking a bit in a buffer, then performs arithmetic on values in a buffer and copies bytes between a0, a1, and a6 while adjusting for a backspace character in d2.
- `process_buffer_and_calculate_max`: Copies bytes from one buffer to another while subtracting a byte from d7, then if a byte in d2 is a backspace, it finds the maximum byte in a destination buffer, calculates a scaled value, and writes it to several locations; otherwise, it writes specific character values to a buffer based on the value in d2.
- `initialize_registers_and_branch`: Clears registers d7 and d1, moves a 16-bit value from d4 to d0, and then branches to another location.
- `accumulate_bytes_and_write_results`: Accumulates bytes from a source area into a register, keeps a running maximum, derives a scaled character value, and writes that value and a derived value to several destination buffers with a conditional copy.
- `decrement_pointers_and_branch`: Loads three pointers from offsets 124, 128, and 132 of a structure in a5 into a0, a1, and a2, decrements each pointer by 1, sets d2 to 0, moves d4 into d0, and branches to another location.
- `apply_audio_scaling_adjustments`: Iterates through a buffer using a loop to perform signed arithmetic scaling on audio samples based on bitwise flags in a control structure.
- `scale_byte_value`: Loads a pointer from an offset of a5 into a6, loads a byte from a6 at offset d0 into d1, subtracts 110 from d1, multiplies d1 by 26, and branches based on the sign of the result.
- `scale_audio_buffer_samples`: Iterates through a buffer using a loop to perform signed arithmetic scaling on audio samples.
- `modify_audio_buffer_via_scaling`: Iterates through a buffer using a loop, performing bitwise checks on data to determine scaling factors, then applies signed arithmetic to adjust bytes in multiple memory locations by adding or subtracting values derived from multiplying a byte by a constant.
- `compare_and_branch_on_d7`: Copies d4 to d7, decrements d7, compares d7 to d0, and either branches to an earlier location or sets d7 to 64 and branches to another location.
- `set_d7_and_branch`: Sets d7 to 86 and branches to another location.
- `calculate_scaled_value`: Loads two pointers from a table, reads a byte from the first pointer, extends it to a word, multiplies it by 26, and if the result is negative, subtracts 64 from it.
- `scale_audio_samples`: Adds 64 to a 16-bit value in d2, performs an arithmetic shift right by 7, and then adds and subtracts this result from bytes at offsets d0 in structures pointed to by a6 and a3 respectively, looping until d0 reaches zero.
- `branch_to_next_routine`: Performs a short branch to another location.
- `copy_and_adjust_bytes_with_feedback`: Copies bytes between buffers, updates a feedback value by multiplying it with a table entry and shifting, subtracts the feedback from a byte, and writes adjusted bytes to two output areas.
- `check_narrator_flag_and_dispatch`: Calls another routine eleven times and then checks if a byte at an offset of 219 from a pointer in a5 is non-zero; if it is, it calls another routine before returning.
- `call_subroutines_with_offsets`: Loads a pointer from an offset of 40 in a structure, then calls another routine three times using offsets of 3, 4, and 5 added to that pointer.
- `interpolate_and_store_bytes`: Reads two bytes from a buffer, computes a scaled difference based on an index, accumulates the result, and stores successive byte results into the buffer at regular intervals.
- `check_byte_sentinel`: Increments d1 by 8, loads a byte from the table at a0 plus the new d1 offset into d6, and returns if d6 is 255; otherwise it branches to another location.
- `check_conditions_and_increment`: Compares d6 to 254, branches if equal, compares d1 to 8, and if d1 is not 8, branches to another location; otherwise, it adds 8 to a0 and branches to another location.
- `call_interpolation_routine`: Calls another routine and then branches to another location.
- `scan_table_for_sentinel`: Increments d1 by 8 and checks if the byte at the offset of a0 plus d1 is zero or 255; if either condition is met, it branches to another location, otherwise it returns.
- `loop_increment_pointer`: Compares d1 to 8, and if it is not 8, branches to an earlier location; otherwise, it adds 8 to a0 and branches to an earlier location.
- `scan_buffer_for_sentinel`: Iterates through a buffer in increments of 8 bytes, searching for a null terminator or a byte with the value negative one.
- `process_buffer_data_4`: Iterates through a buffer, using values from a second buffer to index a table and modify a destination string through pointer arithmetic and byte additions.
- `process_buffer_data_3`: Performs complex data manipulation involving loops that increment pointers, modify bytes in a buffer based on bitwise checks of values in a table, and copy bytes from one location to another using specific character constants.
- `offset_pointer_and_branch`: Shifts a 16-bit value in d0 left by 3 bits, adds the result to a pointer in a0, and then branches to another location.
- `modify_d3_value`: Shifts d3 right by 2 bits, compares it to 42, and if equal, sets d3 to 96 before branching to another location; otherwise it branches to another location.
- `update_d3_value`: Compares d3 to 43 and, if equal, sets d3 to 97 before branching to another location; otherwise it branches to another location.
- `check_d3_and_branch`: Compares d3 to 44, and if it is not 44, branches to an earlier location; otherwise, it sets d3 to 98 and branches to another location.
- `copy_bytes_from_table`: Copies six bytes from a table located at an offset determined by a byte in a register to a destination pointed to by a register, then returns.
- `initialize_pointers_and_registers`: Loads three pointers from offsets of a5 into a0, a1, and a2, loads a constant into a4, loads a pointer from an offset of a5 into a6, and sets d6, d4, d5, and d7 to specific values before branching.
- `set_bit_and_compare_byte`: Sets bit 31 of d4, then loads a byte from a pointer in a0 into d0 and compares it to d5, returning if they are not equal.
- `update_narrator_stream_state`: Iterates through a data stream to perform conditional block copies or bitwise modifications on a target structure.
- `process_narrator_data_stream`: Iterates through a data stream, performing bitwise masking and logical operations on structure members at calculated offsets, and copies blocks of bytes from one memory location to another based on values read from the stream.
- `calculate_scaled_index_offset`: Shifts the value in register d2 left by three bits, adds the resulting value to the address in a6, and branches to another routine.
- `copy_data_from_table`: Reads bytes from a pointer derived from a structure in a5, compares them to specific markers, and either copies three bytes from a global table using the values as indices or skips eight bytes if a marker is found, looping until a termination byte is encountered.
- `copy_bytes_with_offset`: Copies bytes from a source buffer to a destination buffer using an index from a1 to select bytes from a table, then loops back to an earlier instruction.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `load_byte_from_offset`: Sets d6 to 255, calculates an offset by shifting 6 left by 3, loads a byte from a structure in a6 at that offset into d3, and branches to another routine if d3 does not equal 255.
- `update_scaled_value`: Calculates a value by adding d3 to d0, performs a bitwise shift, and stores the result at offset 24 of a structure in a6, then loops back to an earlier instruction.
- `compare_table_byte_to_constant`: Loads a byte from a table using an index derived from the value six, compares it to a constant value of two hundred fifty-five, and branches to another routine if they are not equal; otherwise it returns.
- `process_data_table`: Iterates through a table to perform arithmetic on a running sum and updates a byte at offset 24 of a structure in a6 every 7 iterations, continuing until a comparison with a byte in the table is met.
- `load_pointers_and_check_byte`: Loads several pointers into a0, a2, a4, and a6 from various offsets and a global address, then loads a byte from the address in a0 into d0 and returns if that byte is 255.
- `process_narrator_data_stream_2`: Iterates through a data stream until a terminator byte is encountered, performing arithmetic operations on a byte at offset 7 of a structure pointed to by a6 based on the values read from the stream.
- `add_constant_to_structure`: Adds the constant 10 to a byte at offset 7 of a structure in a6 and loops based on a counter in d2, then branches to another routine.
- `load_constant_d3`: Loads the constant 250 into d3 and branches to another routine.
- `load_negative_constant_and_branch`: Loads the constant negative 6 in register d3 and branches to a location within another routine.
- `load_negative_constant_and_branch_2`: Loads the constant negative 6 from a register and branches to another routine.
- `store_fixed_byte`: Stores the byte value 230 at offset 7 of a structure in a6 and loops based on a counter in d2, then branches to another routine.
- `modify_structure_byte`: Subtracts 43 from a byte at a6, performs an arithmetic shift right by 2, adds the result to a byte at offset 7 of a structure in a6, and loops based on a counter in d2, then branches to another routine.
- `decode_nibble_checksum_bytes`: Reads a pointer and a count from a structure pointed to by A5 and processes a byte array in two passes.
- `wait_for_events`: Calls three different functions via pointers from a global structure to populate fields in a structure at a0, then enters an infinite loop that calls another routine and a fourth function.
- `handle_device_event`: Enters a loop that checks a bit in a byte at an offset of a2, sets bit 0 of that byte, and calls another routine.
- `clear_status_bits`: Clears bit 1 and bit 0 of a byte at offset 34 of a structure in A2.
- `configure_speech_playback_parameters`: Initializes a playback structure by copying data from a source, populating fields such as sample rate and buffer offsets, and then executes a loop to fill a memory area with a specific pattern.
- `initialize_narrator_playback_structure`: Copies a block of data from one memory location to another, clears a twelve-byte area in a structure, and populates several fields in that structure using values from a string pointer.
- `transform_narrator_command_stream`: Iterates through a data stream to decode and transform command bytes by performing arithmetic operations on values retrieved from a table and writing the results to a destination buffer.
- `parse_narrator_command_stream_2`: Iterates through a data stream to parse command bytes, updating a structure's internal state and executing a series of function pointers from a table based on a count retrieved from a structure offset.
- `decode_narrator_command_stream_2`: Iterates through a data stream to decode command bytes by performing arithmetic operations on values retrieved from a table and writes the resulting character to a destination buffer.
- `decode_narrator_command_stream`: Iterates through a data stream to decode command bytes, updating a structure's internal state and writing processed bytes to a destination buffer.
- `parse_narrator_command_stream_3`: Iterates through a data stream to parse command bytes, updating a structure's internal state and executing a series of function pointers from a table.
- `update_offset_value`: Decrements d0 by 1, branches if d0 is positive, otherwise writes d5 to an offset of 16 in a5 and branches to another routine.
- `execute_callback_loop`: Decrements a counter from a value stored at an offset in a structure, calls a function pointer from a global table that many times, and calls enable_dma_if_flag_set if a specific bit is set in a structure.
- `enable_dma_if_flag_set`: The routine tests a flag byte in a structure pointed to by A5 and, if set, writes a value to a custom hardware register that enables DMA channels by setting bit 15 of DMACON.
- `process_interrupt_logic`: Compares a byte from a pointer to a value in a structure, and if they differ, updates several fields in a linked list structure, modifies a counter in a global variable, and calls another routine.
- `decrement_counter_and_enable_interrupts_2`: Reads a byte from an offset in a structure, decrements it and writes it back.
- `calculate_table_offset`: Loads a byte from offset 68 of a structure into d2, multiplies it by 68, checks a word at offset 22 of a structure, and if it is not zero, calculates a new address by adding the result to a pointer from offset 236 of a structure.
- `execute_callback_loop_3`: Sets bit 0 of a byte at offset 30 of a structure, then enters a loop that calls a function pointer from a table, eventually clearing bit 7 of a byte at offset 30 of a structure and potentially updating a hardware register or a value at offset 196 of a structure.
- `execute_callback_loop_2`: Checks a bit in a structure at offset 210 of a5, potentially writes to a hardware register, then enters a loop that calls a function pointer from a structure in a6, checks a byte at offset 30 of a3, and either returns or continues the loop based on that byte and a counter.
- `set_bit_and_branch_2`: Sets bit 5 of a byte at offset 30 of a structure in a3 and branches to an earlier instruction.

### Qume_LetterPro_20

- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `call_function_triplet`: Loads a parameter into a global variable and attempts to call three consecutive function pointers from a table starting at that global variable, returning 0 if all calls succeed or a failure code otherwise.
- `call_routine_thrice`: Calls another routine three times and returns.
- `call_library_function_thrice`: Calls a function from a library three times using a pointer and an argument.
- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_command_parameters`: Clears registers d1, d2, and d3, loads three pointers from the stack into a0, a1, and a4, and branches to another routine if the word at the pointer in a0 is not equal to 1.
- `parse_printer_command_sequence`: Copies a sequence of bytes from a table into a destination buffer and performs a series of conditional checks on a structure pointed to by a register to update the buffer with specific control characters or numeric ASCII representations.
- `copy_byte_loop`: Copies bytes from a table at offset 0 to a destination at offset a4 plus d2, incrementing the source and destination pointers in a loop until a counter in d3 reaches 11.
- `update_status_values_5`: Compares a word at the address in a0 to 27, checks if the byte at the address in a1 is not negative, and if both conditions are met, writes 32 to the word at a0 and 1 to the byte at a1.
- `update_word_on_match`: Compares a word at the address in a0 to 27; if it matches and a byte at the address in a1 is less than zero, it writes 4 to the word at a0, then 1 to the byte at a1 and branches to another location.
- `update_threshold_if_match`: Compares a word at the address in a0 to 28 and a byte at the address in a1 to zero; if both conditions are met, it writes 33 to the word at the address in a0, then branches to another routine.
- `update_threshold_values`: Compares a word at the address in a0 to 29 and a byte at the address in a1 to zero; if both conditions are met, it writes 33 to the word at the address in a0 and 255 to the byte at the address in a1, then branches to another routine.
- `update_threshold_value_check`: Compares a word at the address in a0 to 29; if it is not equal, it branches to another routine, otherwise it checks if a byte at the address in a1 is greater than 0, and if so, writes the word 2 to the address in a0 before branching to another routine.
- `update_threshold_value`: Compares a word at the address in a0 to 30; if it is not equal, it branches to another location, otherwise it checks a byte at the address in a1 and, if it is less than 0, writes 32 to the word at the address in a0 before branching to another location.
- `update_word_based_on_byte`: Compares a word at the address in a0 to 31; if it is not equal, it branches to another location, otherwise it checks a byte at a1 and, depending on whether it is greater than or equal to zero or less than or equal to zero, writes either 33 or 32 to the address in a0 and clears the byte at a1.
- `increment_byte_on_match`: Compares a word at the address in a0 to 32, and if it is not equal, branches to another location; otherwise it increments a byte at the address in a1 and branches to another location.
- `decrement_and_branch_2`: Compares a word at the address in a0 with 33, and if equal, decrements the byte at the address in a1 before branching.
- `format_status_string`: Checks if a word at a pointer in a0 is 57; if so, it writes two specific bytes and a four-character numeric representation of a value from the stack into a buffer at an offset in a4, otherwise it checks if the word is zero and writes 253 to offset 680 of a structure in d1.
- `add_constant_to_byte`: Adds 48 to a sign-extended word in d0 and stores the result as a byte at offset a4 plus d2, then branches to an earlier instruction.
- `write_status_byte_3`: Checks if a word at the address in a0 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in d1.

### CBM_MPS1000

- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_dual_calls_2`: Calls a routine using a pointer from a global variable, and if that returns non-zero, calls another routine using a pointer from a different global variable, then potentially calls a third routine using a pointer from the first global variable, and finally calls a routine at an offset of -414 from the first global variable if certain conditions are met.
- `execute_dual_calls`: Saves a6 to the stack, calls another routine using a pointer from a table, restores a6, and repeats this process with a different pointer from a table before branching to an earlier instruction.
- `execute_handler_triple`: Calls a routine at an offset from a pointer in a global variable three times, passing the value from a6 to the second and third calls.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry`: Checks a condition via a routine call, and if met, populates a structure with two parameters and executes several routines through a global base.
- `manage_buffer_state_machine`: Dispatches logic based on a parameter to perform various buffer operations including clearing memory, writing specific byte sequences, bitwise ORing data into a buffer, or updating global state variables.
- `clear_d0_and_return`: Sets d0 to zero and returns.
- `initialize_printer_state`: Saves registers to the stack, loads pointers from the stack into a2, a3, and a4, clears d4, d5, and d1, sets d3 to 3, and branches to another routine if the word at the address in a2 is 1.
- `format_printer_command_buffer`: Copies a sequence of bytes from a table into a destination buffer, inserting specific control characters based on values read from a structure, and appends a four-byte sequence from a global variable or a constant value depending on a comparison check.
- `modify_buffer_byte`: Loads a pointer from the address in a0, compares a word at offset 134 of that pointer to 2048, and if equal, writes 15 to a byte at the address calculated by adding a4 to the sum of d4 and a0, then branches to another routine.
- `process_state_and_update_buffer`: Copies a string from a source table into a destination buffer using a loop, then performs conditional updates to the buffer and a structure based on the value at a pointer in a2.
- `copy_small_table_loop`: Copies bytes from a table at offset 0 to a destination at offset a4 plus d4, incrementing the source and destination pointers in a loop until a counter in d5 reaches 4.
- `validate_and_update_status`: Compares a word at a pointer in a2 to 66; if equal, it loads a pointer from d1, checks a word at offset 1366 of the pointed structure for 2048, and if it matches, writes the byte 96 to the address in d3 before branching to another routine.
- `branch_on_match`: Loads a pointer from a0 into a1, compares a word at offset 1022 of a1 with 1024, and if equal, writes the byte 137 to the address in a0 before branching.
- `set_byte_and_branch_2`: Moves the value in d3 to a0, writes the byte 80 to the address in a0, and branches to another routine.
- `copy_bytes_from_table_2`: Copies up to five bytes from a table starting at a pointer in a4 to a destination address calculated from a pointer in a2 and an offset in d4.
- `modify_word_and_byte`: Compares a word at the address in a2 to 32; if it is not equal, it branches to another location, otherwise it checks a byte at a3 and, if it is non-zero, writes 1 to the byte at a3 and 27 to the word at a2 before branching to another location.
- `clear_byte_and_branch_2`: Tests the byte at the address in a3; if it is non-negative, it clears the byte at a3, writes the word 30 to the address in a2, and branches to another routine; otherwise it branches to another routine.
- `set_d0_and_branch`: Sets d0 to 255 and branches to another routine.
- `update_status_values_3`: Compares a word at the address in a2 to 33, and if it is not equal, branches to another location; otherwise it checks a byte at the address in a3, and if that byte is not zero, branches to another location, otherwise it sets that byte to 255 and the word at the address in a2 to 29 before branching to another location.
- `clear_and_set_status`: Checks a byte at the address in a3; if it is less than or equal to zero, it clears that byte and writes the word 28 to the address in a2, otherwise it branches.
- `branch_to_previous_instruction`: This routine performs a short relative branch to an earlier instruction in the calling code.
- `update_status_values`: Compares a word at a pointer in a2 against several constants to write specific bytes to a stack location or a structure, or calls write_status_byte if the word is zero.
- `write_status_byte`: Checks if a word at the address in a2 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in d1.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_630

- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_release_library_handles`: Attempts to call three consecutive functions via offsets from a pointer stored in a global variable, and if any call returns a non-zero value, it executes a cleanup routine using an offset from that same global variable to release the handles.
- `execute_dual_calls_4`: Calls a routine using a pointer from a global variable, restores a6 from the stack, calls a second routine using a different pointer from a global variable, restores a6, and returns with d0 set to 255.
- `execute_indirect_function_calls`: Fetches a function pointer from a global variable using an offset and calls that function three times, passing the value from register a6 as an argument to the second and third calls.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_command_parameters_2`: Loads three pointers from the stack into registers, clears three registers to zero, and compares a word at the first pointer to one; if it is not one, it branches to a different location, otherwise it continues to another routine.
- `process_input_state_machine`: Updates a state machine by comparing values at specific offsets within a structure to determine new state values and increments or decrements a secondary value.
- `process_control_sequence_and_copy_data`: Copies a null-terminated string from a source table to a destination buffer, then performs a series of conditional checks on a word and a byte to update the word with specific values or modify the byte based on a range of control codes.
- `copy_table_byte_in_loop`: Copies a byte from a table at offset d0 to a destination address formed by adding a2 to a4, then enters a loop that repeats this process for a specified count in d3.
- `update_word_and_byte_on_match`: Compares a word at the address in a0 to 27; if it matches and a byte at the address in a1 is less than zero, it writes 32 to the word at a0, writes 1 to the byte at a1, and otherwise returns.
- `update_word_on_match_2`: Compares a word at the address in a0 to 27; if it matches and a byte at the address in a1 is less than zero, it writes 4 to the word at a0 and branches to another routine, otherwise it branches to another routine.
- `check_threshold_limit`: Compares a word at the address in a0 to 28 and a byte at the address in a1 to zero; if both conditions are met, it writes 33 to the word at the address in a0 and branches to another routine.
- `reset_threshold_values`: Compares a word at the address in a0 to 29 and a byte at the address in a1 to zero; if both conditions are met, it writes 33 to the word at the address in a0, writes 255 to the byte at the address in a1, and otherwise returns.
- `adjust_threshold_word`: Compares a word at the address in a0 to 29 and a byte at the address in a1 to zero; if both conditions are met, it writes 2 to the word at the address in a0 and branches to another routine.
- `update_threshold_value_2`: Compares a word at a pointer in a0 to 30; if it is not 30, it checks if a byte at a pointer in a1 is less than 0, and if so, writes 32 to the word in a0, then branches to another routine.
- `adjust_word_and_byte_threshold`: Compares a word at a pointer in a0 to 31; if it is not 31, it checks a byte at a pointer in a1 and either writes 33 to the word in a0 if the byte is greater than 0, or writes 32 to the word in a0 and clears the byte in a1 if the byte is less than 0, then branches to another routine.
- `compare_and_increment_byte`: Compares a word at the pointer in a0 to 32; if it is not 32, it increments a byte at the pointer in a1 and branches to another routine.
- `decrement_and_branch_on_value`: Compares a word at the pointer in a0 to 33; if the value is not 33, it decrements a byte at the pointer in a1 and branches to another routine, otherwise it branches to another routine.
- `update_data_structure`: If the word at the address in a0 is 12, it writes the byte 27 to an offset of d2 from a4, and then writes either the byte 65 or 66 to an offset of d2 plus 1 from a4; otherwise, if the word at a0 is 0, it writes the byte 253 to offset 2720 of the structure pointed to by d1.
- `write_constant_byte`: Writes the byte 66 to a destination address formed by adding a2 to a4, then branches to another routine.
- `write_status_byte_to_structure`: Checks if a word at the pointer in a0 is non-zero; if it is zero, it writes the byte 253 to an offset of 2720 in the structure pointed to by the value in d1, then returns.

### ImagewriterII

- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_sequential_function_pointers`: Attempts to call three consecutive function pointers located at specific offsets within a structure pointed to by a global variable.
- `execute_dual_function_pointers_2`: Loads two function pointers from a table and calls each sequentially, using the return value of the first to determine if the second is executed, then returns with negative one in the data register.
- `execute_indirect_function_calls_2`: Fetches a function pointer from a global variable using an offset and calls that function three times in succession.
- `return_constant_20`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_21`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_2`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base using a pointer retrieved from that structure.
- `POSDIV`: Performs a division operation on a 32-bit value in d1 by a divisor in d2, using a loop for large divisors or a modulo-based approach for small divisors, and returns the quotient and remainder.
- `LONGDIV`: Performs long division on a 32-bit value split between two registers by rotating bits through a carry flag for sixteen iterations.
- `ulmult`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products and returns the result.
- `ulmodt`: Pushes a longword from register d2 onto the stack, moves d1 into d2 and d0 into d1, calls POSDIV, restores d2 from the stack, and returns.
- `uldivt`: Moves the contents of register d0 into d1 and d1 into d2, calls POSDIV, and returns the value left in register d1 in register d0.
- `lmodt`: Calls a routine to perform a division and returns the result, negating the value if the helper leaves a non-zero value in an address register.
- `ldivt`: Calls POSDIV and returns the quotient in d1, or the negation of the quotient if the remainder in a0 is nonzero.
- `dispatch_via_table`: Saves registers to the stack, retrieves a byte from the stack, compares it to 4, and if it is greater than 4, performs a calculation and jumps to an address from a table using the result.
- `clear_d0_and_restore_registers_alt`: Clears register d0 and restores d2 through d5 and a2 through a4 from the stack before returning.
- `initialize_printer_command_buffer`: Loads a pointer from the stack into a1, retrieves a destination address from the stack into d2, and sets a pointer in a4 to zero.
- `emit_imagewriter_escape_sequences`: Copies eleven bytes from a table into a buffer and then appends various escape sequences based on flags and command bytes found in a structure.
- `translate_printer_command_byte`: Reads a command byte from a pointer in a1 and writes corresponding escape sequences into a buffer at d2 plus an offset in d3.
- `emit_escape_sequence_commands`: Reads a command byte from a pointer in a1 and writes corresponding escape sequences to a buffer pointed to by a combination of d2 and d3.
- `modify_status_and_word`: Compares a word at the address in a1 to 31; if it matches and a byte at the address in a3 is greater than or equal to zero, it writes 33 to the word at a1 and writes 1 to the byte at a3, then branches to another routine, otherwise it branches to another routine.
- `reset_status_byte`: Checks if the byte at the address in a3 is less than zero; if so, it writes 32 to the word at the address in a1, writes 255 to the byte at the address in a3, and branches to another location.
- `branch_to_offset`: Performs a short branch to a specific offset.
- `translate_imagewriter_escape_sequences_4`: Translates input bytes into specific escape sequences for an ImageWriter II printer by writing control characters and ASCII strings to a destination buffer.
- `write_four_bytes`: Writes the bytes 66, 27, 108, and 48 sequentially to a memory location starting at the address formed by the sum of d2 and d3.
- `translate_imagewriter_escape_sequences_3`: Reads a byte from a buffer and writes specific escape sequences to a destination memory location based on the input value, such as converting input bytes into printer control codes for line spacing, font selection, or character positioning.
- `write_byte_and_branch_2`: Moves the value in d3 to a2, increments d3, adds d2 to a2, writes the byte 66 to the address in a2, and branches to another routine.
- `translate_imagewriter_escape_sequences_2`: Reads a byte from a buffer and writes corresponding escape sequences to a destination memory location based on the input value.
- `write_byte_and_branch`: Adds a value from a register to a pointer, stores the byte 80 at that location, increments a register, and branches to another routine.
- `translate_imagewriter_escape_sequences`: Reads a byte from a buffer and writes corresponding escape sequences to a destination memory location based on the input value.
- `write_byte_and_branch_alt`: Adds a value from a register to a pointer, stores the byte 112 at that location, increments a register, and branches to another routine.
- `encode_imagewriter_escape_sequences`: Reads a byte from a buffer and writes specific escape sequences starting with the byte thirty-one to a destination memory location.
- `process_buffer_data_2`: Checks a word at the address in a1 and calls another routine to handle the data based on its value.
- `write_byte_conditionally`: If a byte at a specific location is not 1, it writes the byte 80 to a destination address formed by adding a2 to d2, then branches to another routine.
- `write_byte_to_offset_address`: Calculates an address by adding a value in a register to a pointer in another register, stores the byte value seventy at that address, and branches to another routine.
- `process_buffer_data`: Checks a word at the address in a1; if it is 46, it writes bytes to a destination based on a global variable; if it is 48, it updates a value at a stack location and writes an escape sequence and a character to a destination; otherwise, it writes an escape sequence and a character from a table to a destination.
- `write_byte_conditionally_alt`: If a byte at a specific location is not 1, it writes the byte 69 to a destination address formed by adding a2 to d2, then branches to another routine.
- `write_byte_and_branch_low`: Adds a value from a register to a pointer, stores the byte 78 at that location, increments a register, and branches to another routine.
- `process_buffer_values`: If the word at the address in a1 is 48, it updates a value on the stack, writes the bytes 27, 115, and a byte derived from the stack value plus 48 to an address formed by adding d2 and d3; otherwise, if the word at a1 is 12, it updates a value on the stack and either writes the bytes 27, 75, and a byte from a table to an address formed by adding d2 and d3, or it returns.
- `check_and_update_byte`: Checks if a word at a pointer in a1 is 12; if so, it examines a byte at a pointer from the stack to determine whether to write 30, 37, or 21 to that location, otherwise it branches to another routine.
- `store_three_bytes`: Stores three bytes into a memory location starting at an address formed by the sum of two registers.
- `clear_d0_and_restore_registers`: Clears register d0 and restores d2 through d4 and a2 through a4 from the stack before returning.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_C-150

- `return_constant_22`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_pointer_calls`: Saves a6 to the stack, calls a routine using a pointer from a global variable at an offset, and if the result is nonzero, repeats this with two more calls using the same global variable and offset, finally calling another routine at an offset from the global variable if certain conditions are met.
- `execute_dual_function_pointers_3`: Loads two function pointers from a table and calls them sequentially, using the return value of the first call to determine whether to execute the second, then returns with negative one in d0.
- `call_indirect_triple`: Calls a function through a pointer located at a specific offset from a global variable three times consecutively.
- `return_constant_23`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_24`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_3`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte, then executes three routines through a global base and one routine through a pointer stored within that structure.
- `divide_32_bit_integers`: Performs a 32-bit division of a value in d1 by a value in d2, returning the quotient and remainder.
- `rotate_32bit_value_with_threshold`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves, subtracting a threshold from the high half if the value meets or exceeds it.
- `multiply_32_bit_integers_3`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components and returns the result.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `dispatch_via_table_2`: Saves registers to the stack, loads three values from the stack, and uses a byte from the stack as an index into a jump table to perform a jump to another routine.
- `clear_d0_and_restore_registers_2`: Sets register d0 to zero and restores several data and address registers from the stack before returning.
- `handle_printer_command`: Processes commands based on a value in a pointer; it can initialize a structure, convert a numeric value to ASCII characters in a buffer, or write escape sequences and characters to a destination buffer based on a state value.
- `copy_table_byte_loop_small`: Copies a byte from a table at offset d1 to a destination address formed by adding a2 to d0, then enters a loop that repeats this process for a specified count in d5.
- `update_memory_from_stack`: If a word at a pointer in a3 is twelve, it performs calculations using a byte from the stack and a global variable to write three bytes to a memory location, then calls another routine to write a fourth byte from a table; otherwise, if the word is zero, it writes the byte 253 to offset 688 of a structure in a4.
- `update_stack_and_memory`: Calculates a value using a byte from the stack and a global variable, writes three bytes to a memory location starting at the sum of d3 and d4, calls another routine, and writes a byte from a global table to the next memory location.
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
- `execute_dual_calls_5_2`: Calls another routine and, if the result is non-zero, calls it again twice more, potentially calling a third routine if those calls return non-zero.
- `execute_dual_calls_5`: Loads a pointer from a global variable into a1, calls a routine using a pointer from a table, loads another pointer into a1, and calls a second routine using a pointer from the same table, then sets d0 to negative 1 and branches.
- `execute_triple_library_call`: Calls a library function at a specific offset from a global base three times, passing the value from a6 to the second and third calls.
- `return_constant_29`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_30`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_31`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_printer_command_structure`: Loads three pointers from the stack into registers, clears three registers to zero, and compares a word at the first pointer to one; if the value is not one, it branches to a different location, otherwise it continues to another part of the code.
- `process_printer_command_stream`: Copies a sequence of bytes from a source buffer into a destination buffer and performs conditional checks on specific values at offsets within a structure.
- `process_printer_control_sequence`: Copies a null-terminated string from a table to a destination buffer, then processes a series of control characters by updating a word at a pointer in a0 and a byte at a pointer in a1 based on specific conditional logic.
- `copy_byte_from_table_in_loop`: Copies bytes from a table at address zero to a destination calculated from a pointer in a2 and a4 using a loop that runs until d0 is no longer greater than d3.
- `update_threshold_values_2`: Compares a word at the address in a0 to 27; if it matches and a byte at the address in a1 is not zero, it writes 32 to the word at a0, writes 1 to the byte at a1, and branches to another location.
- `update_status_on_condition_3`: Compares a word at the address in a0 to 27; if it matches and a byte at the address in a1 is less than zero, it writes the word 4 to the address in a0 and branches to another routine.
- `update_status_on_condition_2`: Compares a word at the address in a0 to 28; if it matches and a byte at the address in a1 is positive, it writes the word 33 to the address in a0, otherwise it returns.
- `update_status_on_condition`: Compares a word at the address in a0 to 29; if it is equal and a byte at the address in a1 is not positive, it writes the word 33 to the address in a0 and the byte 255 to the address in a1, then branches to another routine; otherwise it returns.
- `update_word_on_condition`: Compares a word at the address in a0 to 29; if it is equal and a byte at the address in a1 is not positive, it writes the word 2 to the address in a0 and branches to another routine.
- `set_word_if_byte_negative`: Compares a word at the address in a0 to 30; if it is not equal, it branches to another location, otherwise it checks a byte at a1 and, if it is negative, writes 32 to the address in a0.
- `update_word_and_byte`: Compares a word at a pointer in a0 to 31; if it is not 31, it branches, otherwise it checks a byte at a1 and either writes 33 to a0 and branches if a1 is greater than 0, or writes 32 to a0 and clears a1 if a1 is less than or equal to 0, then branches.
- `compare_and_increment_byte_2`: Compares a word at a pointer in a0 to 32; if it is not 32, it branches, otherwise it increments a byte at a1 and branches.
- `decrement_byte_on_match`: Compares a word at a pointer in a0 to 33; if it is not 33, it branches, otherwise it decrements a byte at a1 and branches.
- `write_conditional_bytes`: If a word at a pointer in a0 is twelve, it writes the byte 27 to a memory location and then writes either the byte 65 or 66 to the next location; otherwise, if the word is zero, it writes the byte 253 to offset 688 of a structure in a1.
- `write_constant_byte_to_structure`: Writes the byte 66 to a destination address calculated by adding a pointer in a4 to a value in d2, then branches to another routine.
- `write_status_byte_to_structure_2`: Checks if a word at a pointer in a0 is non-zero; if it is zero, it writes the byte 253 to an offset of 2720 in a structure pointed to by a value in d1, then returns.

### HP_LaserJet

- `return_constant_32`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_execute_callback_chain`: Attempts to call three consecutive function pointers located at specific offsets within a structure pointed to by a global variable, returning a failure code if any call returns zero, otherwise it calls a cleanup routine at a different offset within the same structure before returning.
- `execute_dual_function_pointers_4`: Loads a pointer from a global variable into a1, then performs two consecutive calls to routines located at offsets from pointers retrieved from a table.
- `execute_indirect_function_pointer`: Fetches a function pointer from a global variable using an offset and calls that function three times consecutively.
- `return_constant_33`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_4`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine through a pointer stored within that structure.
- `process_hp_laserjet_control_stream`: Manages a state machine for HP LaserJet printer escape sequences by updating a character buffer and a state byte based on incoming control characters.
- `update_hp_laserjet_state`: Updates a character buffer and a state byte based on incoming control characters, managing a state machine that handles specific escape sequences and printer status flags.
- `process_hp_laserjet_escape_sequences`: Manages a state machine for HP LaserJet printer escape sequences by updating a character buffer and a state byte based on incoming control characters.
- `copy_table_elements_and_call_function`: Copies elements from a global table to a destination pointed to by a4 using a loop that runs 10 times, then calls a function twice using values derived from a structure at offset 28 of a6 and a value at offset -4 of a6.
- `update_word_and_byte_2`: Compares a word at a2 to 27; if it is 27 and a byte at a3 is zero, it writes 32 to the word at a2 and writes 1 to the byte at a3, then branches.
- `handle_laserjet_escape`: Compares the 16-bit word at the address in a2 to 27, and if it matches and the byte at the address in a3 is less than 0, it writes the word 4 to the address in a2 and branches to another routine; otherwise it branches to another routine.
- `update_word_on_match_4`: Compares a 16-bit word at the address in a2 to 28; if it is equal and the byte at the address in a3 is greater than 0, it writes the word 33 to the address in a2 and branches to another location.
- `modify_values_on_match`: Compares a word at the address in a2 to 29; if it is equal and a byte at the address in a3 is not negative, it writes the word 33 to the address in a2, writes the byte 255 to the address in a3, and branches to another routine.
- `update_word_if_byte_positive`: Compares a word at the address in a2 to 29; if it is not equal, it branches to another location, otherwise it checks a byte at a3 and, if it is greater than zero, writes 2 to the address in a2.
- `update_word_if_negative_2`: Compares a 16-bit word at the address in a2 to 30; if it is equal and the byte at the address in a3 is less than 0, it writes the word 32 to the address in a2 and branches to another location.
- `update_word_and_byte_3`: Compares a word at the address in a2 to 31; if not equal, it branches to another location, otherwise it checks a byte at the address in a3.
- `compare_and_increment_byte_3`: Compares a word at the address in a2 to 32; if it is not equal, it branches to another location, otherwise it increments a byte at the address in a3 and branches to another location.
- `compare_word_and_decrement_byte`: Compares a word at the address in a2 to 33 and, if equal, decrements a byte at the address in a3 before branching to another location; otherwise it branches to another location.
- `compare_and_branch_data_2`: Compares a word at the address in a2 to 64 and branches if not equal; otherwise, it checks a byte at the address in a0 pointed to by offset 28 of a6, and if that byte is non-zero, it writes the byte at offset 67 of a global variable to the address in a0.
- `decrement_and_copy_data`: Decrements a byte at the address pointed to by an offset of 28 in a6, updates a global variable with the new value, conditionally copies a byte from d4 to the address following the decremented byte, copies 11 bytes from a global table to a destination buffer in a loop, and calls another routine twice with specific arguments.
- `copy_byte_to_offset`: Increments a value from a global variable at offset 28 in a6, moves the byte from that new address into d1, and writes d1 to the address in a0.
- `copy_and_call_threshold_check`: Copies 11 bytes from a global table to a destination buffer using a loop, then calls another routine twice with specific arguments derived from a structure pointed to by a6.
- `compare_and_branch_data`: Compares a word at the address in a2 to 57 and branches to another routine if it is not equal, otherwise it branches to another location.
- `copy_table_and_call`: Copies 11 bytes from a global table into a destination buffer using a loop, then calls another routine twice using values from a structure.
- `write_byte_on_null_check_alt`: Checks if a word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d2.
- `convert_value_to_ascii`: Converts a numeric value into a sequence of ASCII characters and writes them to a destination pointed to by the sum of a2 and a3.
- `modify_byte_and_pointer`: Compares a byte in d0 to 99; if it is greater than 99, it writes 49 to a byte at a location calculated from a2 and d4, decrements d3, and branches.
- `convert_to_ascii_sequence`: Writes the byte 48 to a memory location, checks if a byte in d0 is less than 10; if so, it writes 48 to the next memory location, otherwise it writes the result of dividing d0 by 10 plus 48 to the next memory location, calls another routine, and writes the returned byte plus 48 to the following memory location.
- `convert_to_ascii`: Writes the byte 48 to a memory location, calls another routine, and then writes the returned byte incremented by 48 to the next memory location.
- `call_two_subroutines`: Calls a routine at an offset in a global variable with two parameters, then calls another routine at a different offset in the same global variable.
- `perform_32bit_division_and_modulo`: Performs division and modulo operations on a 32-bit value using a 16-bit divisor, returning the quotient and remainder as a 32-bit result.
- `rotate_and_threshold_32bit`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_two_32bit_integers`: Computes the 32-bit product of two 32-bit integers by summing the partial products of their 16-bit components and returns the result.
- `negate_result_on_flag`: Calls another routine and negates the returned integer if the low word of the returned value is nonzero.
- `dispatch_via_table_from_stack`: Restores registers from the stack, reads three values from the stack, and performs a jump to an address retrieved from a table using a value from the stack as an index.
- `clear_registers_and_return`: Clears d0 and restores registers d2 through d4 from the stack before returning.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `save_registers_and_compare_word`: Saves registers a2 and a3 to the stack, loads a word from the stack into d0, and compares it to 768, branching based on whether it is less than or greater than 768.
- `branch_on_word_range`: Compares a 16-bit value in d0 to 256 and branches if it is less than 256, branches if it is greater than 256, otherwise it branches.
- `compare_and_branch_2`: Compares a 16-bit value in d0 to 512 and branches if they are not equal, otherwise it branches to another location.
- `compare_word_and_branch`: Compares the word in d0 to 1024 and branches to a different location if they are not equal, otherwise it branches to another location.
- `initialize_structure_fields`: Writes the values 600, 800, 75, and 53 to various offsets within a structure pointed to by a1 and a global variable, then returns.
- `set_structure_values_and_constant`: Writes the values 800, 1060, 100, and 49 to various offsets within a structure pointed to by a1 and a0, then branches to another routine.
- `set_structure_values_and_byte`: Writes the values 1168, 1590, 150, and 49 to various offsets within a structure pointed to by a1 and a0, writes the byte 53 to the address in a2, and branches to another routine.
- `initialize_structure_values`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 33, and writes 30 to two memory locations.

### Okidata_292

- `return_constant_34`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_execute_library_calls`: Attempts to call three consecutive functions via offsets from a pointer stored in a global variable, returning a failure code if any call returns zero, otherwise it calls a cleanup routine using an offset from the global variable and returns.
- `execute_dual_function_pointers_5`: Loads a pointer from a global variable into a1, then calls a routine located at an offset from a second global variable.
- `call_routine_three_times`: Calls a routine at an offset in a global variable three times, passing the value from a6 as a parameter to the second and third calls.
- `return_constant_35`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_36`: Returns the constant an address in d0 to the caller without any other effect.
- `check_then_queue_request`: Calls a function through a global base pointer to test a condition; if the test returns zero, it stores a code of 9 and two 32-bit parameters into the global state block, clears a flag byte, and then invokes several library-style routines on a loaded object pointer.
- `manage_printer_buffer_data`: Implements a state machine to manage a printer buffer by calculating offsets, clearing memory, and writing formatted escape sequences and data bytes to a buffer pointed to by a global structure.
- `restore_registers_and_return`: Clears register d0 and restores several data and address registers from the stack before returning.
- `setup_command_buffer`: Saves registers to the stack, loads pointers from the stack into a2 and a3, clears d3 and d6, sets d1 to 0 and d5 to 3, and branches to another routine if the word at the address in a2 is 1.
- `format_printer_command_string`: Constructs a command string by copying a sequence of bytes from a table, appending specific control characters based on values read from a structure, and then appending a two-byte sequence derived from a global variable or specific constants.
- `check_and_fill_byte`: Compares a word at offset 2048 of a structure in a0 to 2048, and if equal, sets a byte to 15 at a destination address formed by adding d2 to a pointer in d0, then branches to another routine.
- `process_printer_command_sequence`: Copies a null-terminated string from a source buffer to a destination, then processes a command byte by updating a global variable, writing specific byte sequences to a destination, or modifying status bytes in a structure based on various command codes.
- `copy_byte_from_table`: Copies a byte from a global table to a destination address calculated from d0 and d2, then loops until d6 reaches 4.
- `write_byte_to_pointer_2`: Compares a word at the address in a2 to 66; if it is equal, it writes the byte 1 to an address at offset 2 of a global variable, loads a pointer from a location pointed to by a1 into a0, compares a word at offset 138 of the address in a0 to 2048, and if equal, writes the byte 96 to the address in a0 before branching to another routine.
- `write_byte_to_pointer_on_match`: Loads a pointer from a location pointed to by a1 into a0, compares a word at an offset of a0 to 1024; if it is not equal, it branches to another location, otherwise it writes 137 to the address in a0.
- `write_byte_to_pointer`: Moves a pointer from d5 to a0 and writes the byte 80 to the location pointed to by a0, then branches.
- `write_bytes_from_table_loop`: Writes a byte from a table to a destination address calculated from two registers, then increments the destination pointer and the table index.
- `update_conditional_values_2`: Compares a word at the address in a2 to 32; if it is not equal, it checks if a byte at a3 is non-zero, and if it is zero, it sets that byte to 1 and writes 27 to the address in a2 before branching to another routine.
- `clear_byte_and_set_word`: Clears a byte at the address in a3 if it is not positive, writes the word 30 to the address in a2, and then branches.
- `branch_to_offset_2`: Sets d0 to 255 and branches to an offset in another routine.
- `update_conditional_values`: Compares a word at a2 to 33, and if not equal, checks if a byte at a3 is non-zero; if it is zero, it sets that byte to 255 and writes 29 to a2 before branching to another routine.
- `clear_byte_if_negative`: Clears a byte at the address in a3 if it is not positive, writes the word 28 to the address in a2, and then branches.
- `branch_to_other_routine`: Performs a relative branch to another routine.
- `update_status_and_data`: Updates various memory locations based on the value at a2, including writing a byte to a structure at offset 680 and writing three bytes to a destination calculated from d2 and d3.
- `check_and_modify_stack_values`: Checks if the word at a2 is 12, then compares values at an address retrieved from the stack to modify them or branch.
- `write_three_bytes`: Writes the bytes 27 and 114 to two consecutive memory locations and then writes a byte from a global table to a third location.
- `write_status_byte_2`: If the word at the address in a2 is zero, it writes the byte 253 to offset 2720 of the structure pointed to by the address in d1.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Okimate_20

- `return_constant_37`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_dual_calls_3`: Saves a6 to the stack, calls another routine using a pointer from a table, restores a6, and repeats this process with a different pointer from the table.
- `execute_dual_calls_6`: Calls a routine using a pointer from a global variable, and if that returns non-zero, calls another routine using a pointer from a different global variable.
- `update_handler_pointer`: Calls a routine via a pointer in a global structure three times and then updates a global variable with its own address.
- `set_global_function_pointer`: Compares a word at an offset of 100 from a pointer in a0 to 2; if it is 2, it branches, otherwise it loads the address of a label into a0 and writes it to a global variable at offset 54, then branches.
- `set_function_pointer`: Stores the address of this routine into a global variable at offset 54 and returns zero.
- `return_constant_38`: Returns the constant an address in d0 to the caller without any other effect.
- `manage_printer_buffer_state`: Acts as a multi-purpose state machine for managing a printer buffer, handling buffer allocation, data insertion via bitwise OR, and clearing or writing specific byte sequences to the buffer.
- `return_zero_and_restore`: Clears d0 and restores registers from the stack before returning.
- `manage_printer_buffer_state_2`: Acts as a state machine that handles buffer allocation, byte-wise bitwise OR operations into a memory location, and zeroing out a memory range.
- `clear_d0_and_restore_registers_3`: Sets register d0 to zero and restores several data and address registers from the stack before returning immediately.
- `divide_32_bit_integers_2`: Performs 32-bit integer division using a loop of sixteen iterations that rotates bits and subtracts a threshold, or performs a direct division if a condition is met, and calls a library function.
- `rotate_32bit_value_4`: Rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves, and subtracts a threshold from the high half if the rotated value meets or exceeds it.
- `multiply_32_bit_integers_4`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products and combines the results.
- `forward_call_to_helper_3`: Calls another routine and returns, with no other work.
- `initialize_stack_and_compare`: Saves registers to the stack, loads values from the stack into a0, a3, and a2, clears d2, d5, and a4, sets d3 to 26 and d4 to 16, and compares a word at the address in a0 to 1, branching to another routine if not equal or otherwise branching to another location.
- `populate_data_buffer_2`: Copies 15 bytes from a global table into a destination buffer using a loop, writes zero to two specific offsets in that buffer, and performs several conditional writes to the buffer based on values read from a structure.
- `populate_data_buffer`: Writes a sequence of bytes to a buffer based on three conditional checks involving values at specific offsets of a structure, and includes a loop that writes the byte 13 multiple times.
- `fill_buffer_with_constant`: Enters a loop that fills a destination buffer with the value 13 until a counter reaches a value loaded from a memory location.
- `format_okimate_printer_command`: Processes printer command bytes by updating a state byte and a buffer, or by appending carriage return sequences and escape codes to a destination buffer based on specific command values and a hardware status check.
- `fill_buffer_with_carriage_return`: Fills a destination buffer with the value 13 in a loop until a counter reaches a value loaded from a memory location.
- `process_command_byte_state`: Checks a command byte and updates a state byte and a buffer based on specific values.
- `map_command_values_2`: Fills a destination buffer with the value 13 in a loop until a counter reaches a value loaded from a memory location.
- `update_command_byte`: Compares a word at the address in a0 to 32; if it does not match, it checks the byte at the address in a2, and if that byte is non-zero, sets it to 1, otherwise it writes 1 to the word at the address in a0 and branches to another routine.
- `branch_on_byte_value`: Checks the byte at the address in a2; if it is non-negative, it branches to another routine, otherwise it clears the byte at that address, writes the word 30 to the address in a0, and branches to another routine.
- `branch_with_constant`: Sets register d0 to negative one and branches to another routine.
- `validate_and_update_command_byte`: Compares a word at a pointer in a0 against 33; if it matches, it checks a byte at a pointer in a2, sets that byte to 255 if it is non-zero, writes 29 to the word at a0, and branches to another routine; otherwise, it branches to another routine.
- `map_command_values_3`: Tests a byte at the address in a2; if it is not less than or equal to zero, it clears that byte and writes 28 to the address in a0 before branching to another routine.
- `branch_to_routine`: Performs an unconditional branch to another routine.
- `map_command_values`: Compares a byte at a pointer against several constants and writes specific byte or word values to a destination based on the match.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_Advantage_D25

- `return_constant_39`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_triple_library_call_2`: Sets several values using a parameter and a global variable, then calls three routines at specific offsets from two different pointers, potentially calling a fourth routine if the previous calls return non-zero.
- `execute_double_library_call`: Loads two pointers from a table, calls a routine at an offset from the first pointer, then calls a routine at an offset from the second pointer, and finally sets d0 to 255 before branching.
- `call_library_function_thrice_2`: Calls a library function at a specific offset from a global base three times, passing a value from a6 to the second and third calls.
- `return_constant_40`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_41`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_42`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_registers_and_check_word`: Saves six registers to the stack, loads three values from the stack into a0, a4, and a1, clears d2, d3, and d1, then compares a word at the address in a0 to 1 and branches to another location if not equal; otherwise it branches to another location.
- `parse_control_sequence`: Copies a sequence of bytes from a source structure into a destination buffer, performing conditional checks on specific offsets to insert control characters.
- `copy_string_and_update_state`: Copies a null-terminated string from one memory location to another using a loop, then performs a series of conditional checks on a word and a byte to update the word with specific values and modify the byte based on its current state.
- `copy_byte_from_table_2`: Copies a byte from a table at offset d0 to a destination address calculated from a3, then loops until d0 is no longer greater than d3.
- `update_word_on_match_3`: Compares a word at the address in a0 to 27 and checks if a byte at the address in a1 is non-zero; if both conditions are met, it writes 32 to the word at a0 and 1 to the byte at a1.
- `check_and_write_threshold`: Compares a word at the address in a0 to 27, and if it matches, checks if the byte at the address in a1 is negative; if both conditions are met, it writes 4 to the word at a0 and branches to another location, otherwise it branches to another location.
- `update_threshold_word`: Compares a word at the address in a0 to 28; if it is not equal, it branches to another routine, otherwise it checks a byte at the address in a1, and if that byte is greater than 0, it writes 33 to the word at the address in a0 and branches to another routine.
- `modify_threshold_values_2`: Compares a word at the address in a0 to 29; if it matches and a byte at the address in a1 is non-zero, it writes the word 33 to the address in a0 and the byte 255 to the address in a1, then branches to another routine.
- `update_threshold_byte`: Compares a word at the address in a0 to 29; if it is not equal, it branches to another routine, otherwise it checks a byte at the address in a1, and if that byte is greater than 0, it writes 2 to the word at the address in a0 and branches to another routine.
- `update_threshold_value_3`: Compares a word at the address in a0 to 30; if it is not 30 and the byte at the address in a1 is negative, it writes the word 32 to the address in a0 and branches.
- `update_conditional_word`: Compares the word at the address in a0 to 31; if it is not 31, it branches to another routine, otherwise it checks the byte at a1 and writes either the word 33 or 32 to the address in a0 depending on whether the byte is greater than zero or less than or equal to zero, and finally clears the byte at a1 before branching to another routine.
- `increment_threshold_byte`: Compares a word at the address in a0 to 32; if it does not match, it increments the byte at the address in a1 and branches to another routine.
- `decrement_value_if_match`: Compares a 16-bit word at a0 to 33; if it matches, it decrements the byte at a1 and branches.
- `write_status_byte_if_zero`: Checks if a 16-bit word at the address in a0 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d1.

### Alphacom_Alphapro_101

- `return_constant_43`: Returns the constant an address in d0 to the caller without any other effect.
- `verify_library_function_availability`: Attempts to call three specific functions at an offset of 552 from two different library base pointers stored in a global variable and a passed parameter.
- `execute_triple_library_call_3`: Sets several values using a parameter and a global variable, then calls three routines at specific offsets from two different pointers.
- `execute_indirect_function_calls_3`: Retrieves a function pointer from a global variable using a negative offset and calls it three times, passing the value from register a6 as an argument to the second and third calls.
- `return_constant_44`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_45`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_46`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_local_registers`: Saves registers to the stack, loads three pointers from the stack into a1, a4, and a2, clears registers d1, d2, and d3, sets d2 to 0, and compares a word at the address in a1 to 1.
- `construct_printer_command_buffer`: Copies twelve bytes from a table into a buffer and appends an escape sequence containing specific control characters and padding based on values read from a structure.
- `fill_buffer_with_pattern_2`: Fills a buffer starting at a4 with the byte 32 in a loop until a counter reaches a value from the stack, then writes a sequence of bytes to the end of the buffer and continues filling with the byte 32 in a second loop before writing a final sequence of bytes.
- `fill_buffer_with_pattern`: Fills a buffer starting at a4 with the byte 32 in a loop, then writes a sequence of six specific bytes to the end of the buffer.
- `update_status_and_values`: Compares a 16-bit word at the address in a1 to 27; if it is not 27, it checks if the byte at the address in a2 is non-zero, otherwise it writes 32 to the address in a1 and 1 to the address in a2.
- `compare_and_update_word`: Compares a 16-bit word at the address in a1 to 27; if it is not 27, it checks if the byte at the address in a2 is non-zero, otherwise it writes the word 4 to the address in a1 and branches.
- `update_word_if_positive`: Compares a word at the address in a1 to 28; if it matches and the byte at the address in a2 is greater than zero, it writes 33 to the word at a1 and branches to another location.
- `update_status_byte_2`: Compares the word at the address in a1 to 29; if it is not 29, it branches to another routine, otherwise it checks if the byte at a2 is non-zero and, if so, writes the word 33 to the address in a1 and the byte 255 to the address in a2 before branching to another routine.
- `update_word_if_condition_met`: Compares a 16-bit word at a1 to 29; if it matches and the byte at a2 is greater than zero, it writes the word 2 to a1.
- `update_word_if_negative`: Compares a word at the address in a1 to 30; if it matches and the byte at the address in a2 is less than zero, it writes 32 to the word at a1 and branches to another location.
- `modify_word_and_byte_2`: Compares a 16-bit word at the address in a1 to 31; if it is equal and the byte at the address in a2 is greater than 0, it writes the word 33 to the address in a1; otherwise, if the word is 31 and the byte at the address in a2 is not greater than or equal to 0, it writes the word 32 to the address in a1 and clears the byte at the address in a2.
- `increment_byte_on_match_2`: Compares a word at the address in a1 to 32; if it is not equal, it branches to another location, otherwise it increments a byte at the address in a2 and branches to another location.
- `compare_and_decrement_byte_2`: Compares a word at the address in a1 to 33; if it is not equal, it branches to another location, otherwise it decrements a byte at the address in a2 and branches to another location.
- `process_buffer_command`: If the word at the address in a1 is 66, it writes the byte 13 to a destination address, otherwise if the word is 0, it writes the byte 253 to an offset of 2720 from a pointer in a global variable.
- `write_constant_byte_2`: Writes the byte 66 to a destination address calculated from a0 and then branches.
- `fill_structured_buffer`: If the word at the address in a1 is 66, it writes a byte 13 and then fills a buffer with the byte 8 in a loop, followed by a sequence of bytes, then fills more of the buffer with the byte 32 in a loop and writes a final sequence of bytes; otherwise, if the word at a1 is zero, it writes the byte 253 to an offset of 2688 from a pointer in d2.
- `write_status_byte_if_zero_2`: Checks if a 16-bit word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by the pointer in d2.

### Epson_JX-80

- `return_constant_47`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_sequential_function_pointers_2`: Attempts to call three consecutive function pointers located at specific offsets within a structure pointed to by a global variable.
- `execute_dual_pointers`: Saves a6 to the stack, loads a pointer from a global variable, calls a routine at an offset from that pointer, restores a6, loads a second pointer from a different global variable, calls a routine at an offset from that pointer, restores a6, and sets d0 to 255.
- `execute_triple_callback`: Calls a routine at an offset from a pointer in a global variable, then calls that same routine twice more using the value in a6 as an argument.
- `return_constant_48`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_49`: Returns the constant an address in d0 to the caller without any other effect.
- `check_and_queue_print_job`: Calls a condition-check routine through a global base pointer and, if the check returns zero, stores a state code and two caller-supplied parameters into the global structure, clears a flag byte, and invokes several routines on a stored sub-object before returning the zero status.
- `format_epson_jx_80_print_buffer`: Constructs a print buffer by calculating offsets and writing escape sequences and data into a memory region.
- `return_zero_and_restore_registers`: Clears the d0 register and restores d2 through d4 and a2 through a4 from the stack before returning.
- `setup_and_compare_buffer`: Saves registers to the stack, loads values from the stack into a2, d1, and a3, clears d3, d6, and d2, sets d4 to 6 and d5 to 3, and compares a word at the address in a2 to 1, branching to another routine if not equal or otherwise branching to another location.
- `format_epson_jx_80_command`: Constructs a command buffer by copying bytes from a table and appending specific control sequences based on values read from a structure.
- `process_epson_jx_80_commands_2`: Parses command bytes and state values to generate a sequence of control bytes.
- `decrement_and_copy_bytes`: Decrements a byte loaded from an offset of 60 from the stack, writes it to the address in d5, then loads a byte from an offset of 61 from the stack and writes it to the address in d4.
- `write_bytes_from_table`: Calculates an address by adding d1 and d3, then writes a byte from a table at an offset determined by d2 to that address, and finally branches to another routine if a comparison between d0 and d6 fails.
- `process_epson_jx_80_commands`: Evaluates a command byte to update a state byte and a buffer, performing various conditional writes to a structure and a table.
- `load_and_check_offset`: Loads a pointer from d2 into a1, then loads a pointer from a1 into a0, and compares the word at offset 854 of a0 to 1024; if it is not 1024, it branches, otherwise it loads d4 into a0, writes the byte 137 to a0, and branches.
- `write_byte_and_branch_3`: Loads d4 into a0, writes the byte 80 to the address in a0, and branches.
- `copy_table_bytes`: Copies a byte from a table at offset zero of a register to a destination address calculated from a pointer in a1 and a value in d1, then enters a loop that repeats up to eight times to copy subsequent bytes from the table to the destination.
- `update_status_values_2`: Compares a 16-bit word at a2 to 32, and if equal, checks a byte at a3; if that byte is not zero, it sets the byte at a3 to 1 and the word at a2 to 27, otherwise it branches to another routine.
- `clear_status_byte_2`: Checks if a byte at a3 is negative; if so, it clears the byte at a3, writes 30 to the word at a2, and branches to another routine.
- `set_d0_and_branch_2`: Sets d0 to 255 and branches.
- `update_epson_word`: Compares the 16-bit word at the address in a2 to 33, and if it matches and the byte at the address in a3 is not zero, it writes the byte 255 to the address in a3, writes the word 29 to the address in a2, and branches to another routine; otherwise it branches to another routine.
- `clear_byte_and_update_word`: Checks if the byte at the address in a3 is greater than 0; if so, it clears that byte and writes the word 28 to the address in a2 before branching to another location.
- `branch_to_offset_4`: Branches to a specific offset.
- `modify_structured_data_complex`: Checks the word at a2 against several constants to write specific bytes to a stack location or a destination address calculated from d1 and d3, or writes a byte to a global variable at offset 2720 of a pointer in d2.
- `modify_structured_data`: If the word at a2 is 4, it writes 27, 106, and a byte from the stack to a destination address; if the word is 12 and a byte from the stack is 39, it changes that byte to 30; otherwise if the word is 0, it writes 253 to an offset of 2720 from a pointer in a global variable.
- `compare_and_modify_byte`: Compares a 16-bit word at a2 to 12, and if equal, checks a byte at a pointer stored at offset 60 of the stack; if that byte is 39, it writes 30 to that location, and if it is 37 and not less than or equal to 0, it branches to another routine.
- `write_three_bytes_to_memory`: Writes the byte 27 to the address at in_D1 plus unaff_D3, the byte 114 to the next address, and a word from a global table to the following address.
- `write_status_byte_if_zero_3`: Checks if a 16-bit word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d2.
- `call_library_vector_198_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Epson

- `return_constant_50`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_calls_3`: Calls a routine through a pointer at an offset from a global variable up to three times, and if any call returns a non-zero value, it calls another routine with the global variable as an argument before returning.
- `execute_sequential_library_calls`: Loads a pointer from a global variable into a1, then performs two consecutive calls to functions retrieved from a table using offsets from a6.
- `execute_indirect_function_calls_4`: Fetches a function pointer from an offset within a structure pointed to by a global variable and calls that function three times in succession.
- `return_constant_51`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_52`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_5`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte at specific offsets, then executes three routines through a global base and one routine through a pointer stored within that structure.
- `manage_printer_buffer_state_3`: Implements a state machine to manage a print buffer by handling memory allocation, clearing data, appending bitwise-masked bytes, and inserting escape sequences.
- `restore_registers_and_return_2`: Clears register d0 and restores several registers from the stack before returning.
- `setup_and_compare_pointer`: Saves registers to the stack, loads values from the stack into a1, a4, and a3, clears d3 and d5, sets d1 to 0, d2 to 6, and d4 to 3, and compares a word at the address in a1 to 1, branching to another routine if not equal or otherwise branching to another location.
- `construct_epson_escape_sequence`: Builds a command buffer by copying data from a table and appending bytes based on values read from a structure.
- `process_epson_printer_command`: Parses a command byte and associated data from a structure to build a command buffer, handling specific character sequences and control codes.
- `copy_bytes_from_stack`: Decrements a byte loaded from an offset of 56 from the stack and writes it to the address in d4, then loads a byte from an offset of 57 from the stack and writes it to the address in d2.
- `copy_byte_loop_2`: Increments d3 and d5, calculates a destination address by adding a4 to the value in d0, copies a byte from an offset in a0 to that destination, and loops if d5 is less than 8.
- `process_command_byte_state_2`: Checks a word at the address in a1 against several constant values and, depending on the match, updates a byte at an offset of a structure pointed to by d1, modifies a byte at a pointer in a3, or writes a sequence of three bytes to a buffer pointed to by a4.
- `load_and_check_offset_alt`: Loads a pointer from d1 into a1, then loads a pointer from a1 into a0, and compares the word at offset 854 of a0 to 1024; if it is not 1024, it branches, otherwise it loads d2 into a0, writes the byte 137 to a0, and branches.
- `write_byte_and_branch_alt_2`: Loads d2 into a0, writes the byte 80 to the address in a0, and branches.
- `copy_bytes_from_table_3`: Copies up to eight bytes from a table starting at a zero address to a destination address calculated by adding a pointer in a4 to an offset in d3.
- `check_byte_and_set_word`: Compares the word at the address in a1 to 32; if it is not 32, it branches, otherwise it checks if the byte at the address in a3 is non-zero, and if it is zero, it writes the byte 1 to the address in a3, writes the word 27 to the address in a1, and branches.
- `clear_byte_and_set_word_2`: Checks if the byte at the address in a3 is signed-negative; if it is not, it clears the byte at the address in a3, writes the word 30 to the address in a1, and branches.
- `set_d0_and_branch_4`: Sets register d0 to 255 and performs a short branch to another routine.
- `modify_epson_command`: Compares the 16-bit word at the address in a1 to 33, and if it matches and the byte at the address in a3 is not zero, it writes the byte 255 to the address in a3, writes the word 29 to the address in a1, and branches to another routine; otherwise it branches to another routine.
- `clear_byte_and_set_word_3`: Checks if the byte at the address in a3 is greater than 0; if so, it clears that byte and writes the word 28 to the address in a1 before branching to another location.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `modify_structured_data_simple`: Checks the word at a1 against several constants to write specific bytes to a stack location or a destination address, or writes a byte to a global variable at offset 2720 of a pointer in d1.
- `write_byte_if_zero`: Checks if a word at the address in a1 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d1.
- `call_library_vector_198_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### HP_LaserJet_PLUS

- `return_constant_53`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_pointer_calls`: Loads a pointer from a global variable, performs three sequential indirect calls using pointers derived from that global variable and its offset, and returns zero or negative one based on the results.
- `execute_triplet_function_sequence`: Loads a pointer from a global variable into a1, then executes two consecutive functions from a table by jumping to an offset within the routine pointed to by a6.
- `execute_indirect_function_calls_5`: Retrieves a function pointer from an offset within a structure pointed to by a global variable and calls that function three times using a value from a register as an argument.
- `return_constant_54`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_6`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte at specific offsets, then executes three routines through a global base and one routine via a pointer stored in the structure.
- `update_hp_laserjet_state_machine`: Manages a state machine for HP LaserJet printer escape sequences by updating a character buffer and a state byte based on incoming control characters.
- `update_hp_laserjet_escape_state`: Updates a state machine and character buffer based on control characters, modifying a state byte and a character pointer in a structure.
- `process_hp_laserjet_escape_sequences_2`: Manages a state machine for parsing printer control characters by updating a status byte and a secondary value based on specific input bytes.
- `copy_table_elements`: Copies elements from a global table into a destination buffer using two incrementing counters in a loop, then calls another routine twice using values derived from a pointer at offset 28 of a structure in a6.
- `compare_and_update_values`: Compares a word at the address in a2 to 27, and if it matches, checks if the byte at a3 is non-zero; if both conditions are met, it writes 32 to the address in a2, writes 1 to the address in a3, and branches to another location.
- `update_escape_sequence_word`: Compares a 16-bit word at the address in a2 to 27; if it matches and the byte at the address in a3 is not greater than 0, it writes the word 4 to the address in a2 and branches to another routine, otherwise it returns.
- `update_value_on_match`: Compares a word at the address in a2 to 28; if it matches and the byte at the address in a3 is positive, it writes the word 33 to the address in a2, otherwise it returns.
- `modify_threshold_values`: Compares a word at the address in a2 to 29; if it matches and the byte at the address in a3 is non-zero, it writes the word 33 to the address in a2 and the byte 255 to the address in a3.
- `update_status_values_4`: Compares a word at the address in a2 to 29; if it is not 29, it checks if the byte at the address in a3 is less than or equal to zero, otherwise it writes the word 2 to the address in a2 and branches to another routine.
- `update_word_on_negative_byte`: Compares a word at the address in a2 to 30; if the word equals 30 and the byte at the address in a3 is negative, it writes 32 to the word at the address in a2, otherwise it returns.
- `clamp_word_and_clear_byte`: Compares a word at the address in a2 to 31; if it is 31, it sets the word to 33, otherwise if it is 32 or greater, it sets the word to 33, and if it is less than 32 but the byte at a3 is greater than or equal to 0, it sets the word to 32, otherwise it sets the word to 32 and clears the byte at a3, then returns.
- `compare_and_increment_byte_4`: Compares a word at the address in a2 to 32; if not equal, it branches to another location, otherwise it increments a byte at the address in a3 and branches to another location.
- `compare_and_decrement_byte`: Compares a word at the address in a2 to 33; if not equal, it branches to another routine, otherwise it decrements a byte at the address in a3 and branches to another routine.
- `check_and_update_printer_status`: Compares a word at the address in a2 to 64 and, if equal, checks if a byte at the address pointed to by offset 28 of the structure in a6 is non-zero; if that byte is zero, it writes the byte from a global variable to the address pointed to by offset 28 of the structure in a6 and branches to another routine.
- `decrement_and_update_printer_buffer`: Decrements a byte at a pointer stored in a structure, updates a global variable with the new value, and conditionally updates a second byte in that structure.
- `update_pointer_and_store_byte`: Increments a long value from an offset of 28 in a structure in a6, loads a byte from that new address into d1, and stores d1 as a word into the address held in d4.
- `copy_data_and_call_threshold`: Copies 11 bytes from a global table to a destination buffer, then calls a routine with a word from a pointer in a6 and a constant 3, and calls another routine with the difference between a byte and a word at a pointer in a6 and a constant 7.
- `compare_word_and_branch_3`: Compares a word at the address in a2 to fifty-seven; if the value is not equal, it branches to another routine, otherwise it branches to a different routine.
- `copy_table_and_call_functions`: Copies 11 bytes from a global table to a destination buffer using a loop, then calls another routine twice with specific arguments.
- `write_byte_on_null_check`: Checks if a word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by the address in d2, then returns.
- `convert_to_ascii_sequence_2`: Writes a byte to a location calculated from a2 and param_2 based on the value of param_1, then writes a second byte to the subsequent location based on the value of param_1, and finally calls another routine to write a third byte.
- `convert_byte_to_ascii`: Compares the byte in d0 to 99, and if d0 is greater than 99, it writes the byte 49 to the address in a2, increments d3, and adds the value in d4 to a2 before branching to another location.
- `convert_digit_to_ascii`: Writes the byte 48 to a destination address, writes either 48 or the result of dividing a byte in d0 by 10 plus 48 to the next address, calls another routine, and writes the returned byte plus 48 to the following address.
- `store_digit_ascii`: Writes the character zero to a memory location pointed to by the sum of two registers, calls another routine, and then writes the returned byte incremented by 48 to the subsequent memory location.
- `execute_dual_function_pointers`: Pushes two parameters onto the stack, calls a subroutine via a pointer at an offset of 100 in a global variable, then calls a second subroutine via a pointer at an offset of 104 in the same global variable before returning zero.
- `rotate_32bit_value_5`: Performs a 16-iteration loop that rotates a 32-bit value left by one bit using the carry flag, while subtracting a value from the result if it exceeds a threshold.
- `rotate_and_subtract_32bit_value`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_5`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components.
- `negate_result_on_flag_3`: Calls another routine and negates the returned value if the low word of the returned value is nonzero.
- `dispatch_jump_table`: Pops registers from the stack, loads a value from the stack into d0, and uses it to index into a jump table to perform a jump.
- `clear_registers_and_return_2`: Sets d0 to zero and restores several registers from the stack before returning.
- `call_library_vector_198_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `branch_on_word_value`: Saves registers a2 and a3 to the stack, loads a word from the stack into d0, and branches to different locations depending on whether d0 is less than 768 or greater than 768.
- `branch_on_value_range`: Compares a 16-bit value in d0 to 256 and branches to different locations depending on whether the value is less than or equal to 256 or greater than 256.
- `compare_d0_value`: Compares the word in d0 to 512 and branches to a specific offset if they are not equal, otherwise it branches to another offset.
- `compare_word_and_branch_2`: Compares a word in d0 to 1024 and branches to different locations based on the result.
- `initialize_structure_fields_2`: Writes 600 to offset 26 of a structure, 827 to offset 30, 75 to offset 36, 75 to offset 34 of a second structure, and 48 to a global variable, then writes 55 and 53 to two other memory locations.
- `set_structure_constants`: Sets offset 26 to 800 and offset 30 to 1060 in a structure, then sets offset 36 to 100, offset 34 to 100, and offset 45 to 49 in the same structure before branching.
- `set_structure_constants_with_byte`: Sets offset 26 to 1168 and offset 30 to 1590 in a structure, then sets offset 36 to 150, offset 34 to 150, offset 45 to 49, and offset 34 of a second structure to 53 before branching.
- `initialize_printer_structure_values`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 51, and writes 48 to two memory locations.

### 12

- `return_constant_55`: Returns the constant an address in d0 to the caller without any other effect.
- `process_font_data_large`: A routine that processes data from a font file.

### 15

- `return_constant_56`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_57`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_stub`: The routine ignores its arguments and returns a constant zero as a 32-bit value.

### 12

- `return_constant_58`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_2`: The routine returns zero immediately.

### 20

- `return_constant_59`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_font_data`: Copies a sequence of bytes from a source address to a destination address using a loop, where the addresses are provided via pointers in a data structure.

### 11

- `return_constant_60`: Returns the constant an address in d0 to the caller without any other effect.

### Balance of Power - The 1990 Edition (1990)(Mindscape).adf:CLI

- `parse_command_line_arguments_2`: Parses a string from a buffer into individual arguments, handling quoted strings and converting specific escape sequences like asterisk-N to newline and asterisk-E to escape.
- `invoke_two_library_functions`: Loads a pointer from an offset of 92 in a4 into a0, calls a function at a negative offset from a6, repeats this process with a different negative offset from a6, and then returns.
- `call_dos_function`: Loads a value from a constant into a register, calls a library function at a negative offset from a base register, stores the result in a global variable, and returns if the result is not zero.
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
- `store_pointer_to_global`: As library code, it copies the value in a0 to a global variable and returns.
- `update_library_status`: As library code, it checks a value at offset 8 of a structure in a6; if zero, it moves the value from a6 to a pointer at offset 4 of that pointer, updates a linked list, and calls a function pointer; otherwise, it sets bit 3 of a byte at offset 14 of the structure.
- `set_library_status_bit`: As library code, it sets bit 3 of a byte at offset 14 of a structure in a6 and returns zero.
- `increment_library_counter`: As library code, it checks if bit 3 of a byte at offset 14 of a structure in a6 is clear; if so, it increments a 16-bit word at offset 32 of that structure, and if that word becomes one, it calls another routine with a global variable and returns the structure address, otherwise it returns minus one.
- `return_negative_one`: As library code, it returns minus one.
- `decrement_counter_and_call_2`: Library code that decrements the 16-bit counter at offset 32 of the structure at a6.
- `mask_and_call_routine`: As library code, it masks the lower seven bits of a byte in d0, calls another routine, and sets the second bit of the condition code register.
- `process_bitfield_loop`: As library code, it checks if the byte in d0 is non-zero and non-negative, then enters a loop that performs arithmetic on a running difference and an accumulator to update a bitmask.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `modify_and_call_pointer`: Copies a long value from d7 to d0, calls a routine through a pointer in a register, sets bit 0 of d0, clears bit 3 of d0, and branches to another routine.
- `call_jump_table_entries`: Library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_d1_register`: As library code, it clears the long register d1 and branches to another location.
- `dispatch_conditional_calls`: Library code performs an indirect call through a pointer at an offset from a structure in a5, then executes a sequence of conditional indirect calls based on the value of a byte in d0 and the sign of d7.
- `dispatch_library_functions`: As library code, it performs an indirect call using a pointer from a structure in a5, then performs a series of conditional indirect calls through that same structure based on the values in d6 and d7.
- `execute_conditional_dispatch`: Evaluates the state of registers D4, D6, and D7 to perform a sequence of indirect function calls through a structure pointed to by A5.
- `execute_conditional_calls`: As library code, it moves d7 to d1, calls a routine at an offset from a5, and branches if the result is not negative; otherwise, it checks d4 to either branch or call two different routines using a constant and a value from d2.
- `call_and_restore_d2`: Copies d2 to d0, calls a routine using an offset from a5, copies the result in d0 back to d2, and branches to another routine.
- `branch_or_load_constant`: Copies d2 to d0, checks if d4 is non-zero to branch to another routine, otherwise loads a constant into d2 and branches to another routine.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `calculate_bit_shift_loop`: As library code, it performs a series of checks on a value derived from d7, and if certain conditions are met, enters a loop that performs bit-shifting and arithmetic to calculate a value.
- `initialize_register_values`: As library code, it performs a logical shift right on d7, sets d6 to zero, sets d5 to a constant, loads a pointer into a0, and sets d1 and d2 to specific constants before branching.
- `execute_loop_with_branching`: As library code, it executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_and_branch_library_code`: As library code, it pushes the word negative 40982 onto the stack and then branches to another location.
- `push_and_branch_library_code_2`: As library code, it pushes the word negative 40991 onto the stack and then branches to another location.
- `push_and_branch_library_code_3`: As library code, it pushes the word 1 onto the stack and then branches to another location.
- `compare_and_adjust_stack`: As library code, it compares a byte in d7 to 184, branches if it is higher, otherwise it increments the stack pointer by 2, tests d7, and returns.
- `perform_math_transformation`: Library code that performs a series of arithmetic operations using values from a pointer in a0 and a global variable within a loop, and calls another routine.
- `perform_math_loop`: As library code, it performs a series of arithmetic operations using values from a pointer in a0 and a global variable within a loop, and calls several other routines.
- `perform_arithmetic_loop_3`: As library code, it performs a series of arithmetic operations using values from a pointer in a0 and a global variable within a loop.
- `perform_arithmetic_loop_2`: As library code, it executes a loop that performs arithmetic operations on d6 and d7 using values from a pointer in a0 and a global variable, while also updating a counter and a pointer to a table.
- `perform_arithmetic_loop`: Library code executes a loop that performs arithmetic operations on d4, d5, d6, and d7 using values from a pointer in a0, while also decrementing a 16-bit counter in d1.
- `load_constant_and_branch`: As library code, it loads the constant an address into d7 and branches to another routine.
- `recursive_call_wrapper`: As library code, it branches if d0 is zero, otherwise it moves d5 to d6, calls another routine, moves the result to d7, tests d7, restores registers from the stack, and returns.
- `save_and_call_routine`: As library code, it pushes d5 onto the stack, calls another routine, restores d6 from the stack, and then branches to another location.
- `compare_and_branch_d6`: As library code, it moves d6 to d4, branches if d4 is negative, otherwise it compares d6 to 255 and branches based on whether it is higher or lower, or sets d6 to zero and returns.
- `shift_and_branch_on_zero`: As library code, it arithmetic shifts d4 right by 8 bits, increments d4, and branches if d4 is zero; otherwise it jumps to a location via a jump instruction.
- `return_without_effect`: Library code calls another routine and then calls it again if the returned value is zero.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_sequence_2`: Library code checks if a byte in d7 is non-zero, then performs a sequence of indirect calls based on the carry flag and the value of that byte.
- `rotate_constant_and_branch`: As library code, it loads an address into d7, rotates d7 right through the carry bit by 1, and branches to another routine.
- `load_and_branch_constant`: As library code, it loads an address into d7 and branches to another routine.
- `test_stack_byte_and_branch`: As library code, it pops a word from the stack into d6, tests the byte in d6, branches if it is positive, otherwise it sets d7 to 0 and branches to another routine.
- `set_negative_flag_and_restore_registers`: Sets the negative flag in the condition code register and restores several registers from the stack before returning.
- `load_status_and_return`: Loads a constant value into a register, adjusts the stack pointer by an offset of 30, and returns.
- `process_complex_data_structure`: As library code, it performs a series of conditional checks on d7 and calls various other routines, including multiple calls to a routine that processes data based on the bits of d7.
- `perform_stack_based_calls`: As library code, it calls another routine three times and then returns.
- `transform_and_accumulate_table_values`: Increments a register, performs arithmetic shifts on two values, and if a third value is non-negative, adds the shifted values to two other registers and subtracts a value from a table at a0 from the third register before looping based on a counter in d1.
- `subtract_table_from_accumulator`: As library code, it subtracts d4 from d5, subtracts d3 from d6, adds a value from a table at a0 to d7, and repeats this in a loop while d1 is decremented before returning.
- `test_and_branch_on_d7`: As library code, it tests the byte in d7, and if it is not zero and not positive, it masks the lower seven bits, calls another routine, and sets the second bit of the condition code register before returning.
- `set_d7_to_negative_one`: Sets register d7 to negative one and branches to another routine.
- `perform_complex_arithmetic_loop`: Executes a loop involving bit-shifting and conditional jumps, and calls another routine if a specific condition is met.
- `complex_loop_with_branching`: As library code, it executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `mask_and_call_routine_2`: As library code, it tests the byte in d7; if it is positive, it branches, otherwise it masks d7 with 127, calls another routine, and sets the bit 1 of the status register.
- `process_conditional_data`: Library code calls two routines, then performs a series of conditional checks on d7 to determine whether to call another routine multiple times or perform bitwise manipulations on a value derived from d7.
- `transform_and_rotate_word`: Modifies a 32-bit value by performing bitwise operations, arithmetic shifts, and a subtraction on its lower byte, then swaps the high and low words and performs a 7-bit left rotation before returning.
- `manipulate_d7_bits`: Swaps the high and low words of d7, performs a rotate right through the carry, and executes several conditional branches based on the overflow flag.
- `branch_on_d7_comparison`: As library code, it branches if a condition is met, otherwise compares d7 to 124 and 126, and if d7 equals 126, it adds 133 to d7, performs a rotate left on d7, and branches based on the result.
- `test_masked_register`: The library code performs a bitwise AND on d7 with a mask, then branches to another routine if the result is zero, otherwise it tests the high byte of d7 and branches to another routine.
- `compare_and_shift_d7`: As library code, it compares d7 to 254, branches if not equal, otherwise it performs two logical shifts right on d7 and branches based on the result.
- `shift_and_set_flag`: The library code performs a logical shift left on d7 by 8 bits, loads 255 into d7, performs a rotate right through the carry bit on d7, sets the extension bit in the status register, and branches to another routine.
- `branch_with_zero_register`: The library code clears d7 and branches to another routine.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `load_address_into_a0`: As library code, it loads a long value into a0 and then branches to the next instruction.
- `load_address_pointer`: As library code, it loads the address of a global variable into a0 and then branches to another location.
- `load_address_to_a0_2`: As library code, it loads the address of a constant into a0 and branches to the start of the routine.
- `load_zero_address`: Library code loads zero into a0 and then branches to another routine.
- `load_address_and_branch_7`: As library code, it loads the address of a value at offset 30 into a0 and then branches to another location.
- `load_address_and_branch_library`: As library code, it loads the address of a constant into a0 and branches to another location.
- `load_address_to_a0_3`: As library code, it loads a long value from a location relative to the program counter into a0 and then branches to another location.
- `store_register_via_pointer`: As library code, it invokes another routine using a pointer from a0, then stores a 32-bit value from d7 into memory addressed by a pointer passed in d1, before restoring registers and returning.
- `load_address_to_a0_alt`: As library code, it loads a different constant address into a0 and branches to the start of the routine.
- `load_zero_address_into_a0`: As library code, it loads zero into a0 and then branches to the next instruction.
- `load_address_to_a0_final`: As library code, it loads a third constant address into a0 and branches to the start of the routine.
- `load_constant_address`: Library code loads the constant value 38 into a0 and then branches to another routine.
- `load_address_to_a0_and_branch`: Loads a longword from a memory location into address register zero and then performs a short branch to the next instruction.
- `load_address_and_branch_6`: As library code, it loads a long constant into a0 and then performs an unconditional branch to a relative offset.
- `call_indirect_return_d0_2`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0.
- `load_address_to_a0`: Loads a longword value from a memory location into address register a0 and then branches to the next instruction.
- `load_address_to_register`: Loads a longword from a location relative to a pointer into a register and then branches to the next instruction.
- `load_offset_address_a`: Loads a long value into a0 and branches to another routine.
- `dispatch_indirect_routine`: Loads the address of a global variable into a register and branches to another routine that executes a call at an offset from that address.
- `load_address_and_branch`: Loads the address of a global variable into a0 and performs a short branch to another routine.
- `load_address_and_branch_2`: Loads a longword from a specific memory location into a6 and performs a short branch to another routine.
- `load_offset_pointer_and_branch`: Loads the address of a global variable into a register and performs a short branch to another routine.
- `prepare_indirect_call`: Loads the address of a global variable into a register and branches to another routine to perform an indirect call using an offset.
- `load_address_and_branch_3`: Loads a longword into a register from a specific memory location and performs a short branch to another routine.
- `load_address_and_branch_4`: As library code, it loads a long constant into a0 and branches to another routine.
- `load_different_address_and_branch`: As library code, it loads a different long constant into a0 and branches to another routine.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in d6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_constant_61`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `set_icon_bit_conditional`: The library code checks a byte at offset 8 of a structure in a6; if it is zero, it calls another routine, updates a pointer in a structure, and modifies a linked list; otherwise, it sets bit 3 of a byte at offset 14 of the structure in a6 and returns zero.
- `set_icon_bit`: The library code sets bit 3 of a byte at offset 14 of the structure pointed to by a6, then returns with d0 set to zero.
- `return_constant_62`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_chain_4`: The library code stores a longword from a0 into a structure at offset 42 and a6 into offset 34, calls another routine, and if the result is zero, calls another routine; otherwise, it stores a6 and the result into two global variables.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_zero_or_instruction`: The library code performs a bitwise OR on d0 with zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `traverse_icon_chain_2`: Saves two registers to the stack and calls another routine with the same first argument and the second argument increased by 140, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `execute_icon_chain_logic`: Pushes two pointers from registers onto the stack and calls another routine, then returns.
- `traverse_icon_chain_3`: Saves two registers to the stack, calls another routine to manage an icon chain, and then returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_other_routine_2`: As library code, this routine calls another routine and then returns.
- `call_parse_numeric_string_2`: Saves registers to the stack, calls parse_numeric_string_2, and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `process_icon_data`: As library code, it calls a routine, and if the result is non-zero, it calls another routine, then copies 43 bytes from a stack buffer to a destination pointed to by a pointer in a1, and finally writes several values to a structure at offsets 61, 72, 92, 84, 88, and 76.
- `copy_and_update_node`: The library code copies 43 bytes from a source to a destination in a loop, copies several longwords from a structure in a6 to a structure in a2, and if a longword at offset 76 of the structure in a2 is non-zero, stores a2 into a location at offset 424 of the structure pointed to by that longword.
- `return_zero_and_unlk`: This library routine clears the register d0 to zero, restores registers from the stack, unlocks the register a6, and returns.
- `copy_icon_data_to_stack`: As library code, it copies 43 bytes from a structure pointed to by a third argument into a local stack buffer, copies several fields from that same structure into local stack slots, and calls another routine with the first two arguments and a pointer to a two-word stack header.
- `populate_icon_attribute_fields_2`: Checks a structure for specific magic constants and flags, then iteratively calls external routines to populate various integer fields at offsets 11, 17, 19, 21, 23, and 33 of the structure.
- `populate_icon_attribute_fields`: Checks a structure for specific magic constants and flags, then iteratively calls external routines to populate various integer fields at offsets 18, 22, 33, 35, and 51 of the structure.
- `update_icon_structure`: Library code updates offsets 18 and 22 of a structure in a3 with values from two consecutive routine calls, then performs a series of conditional indirect calls and a loop to update a list of pointers in a structure in a2.
- `execute_icon_callback_loop`: As library code, it executes a loop that calls two function pointers and stores the result of the second call into a table, decrementing a counter in a structure in a6 until it reaches zero or a condition is met, then optionally updates a value in a structure at offset 70.
- `branch_with_zero`: The library code sets register d6 to zero and then branches to another routine.
- `unlk_and_return`: This library routine unlocks the register a6 and restores registers from the stack before returning.
- `traverse_icon_hierarchy`: Recursively processes an icon structure by checking flags and iterating through child pointers, including a list of sub-elements and a pointer to a list of additional icons.
- `traverse_icon_structure_recursively`: Iterates through a hierarchical icon structure by checking flags in a structure at offset 16 and 17, then recursively processes child pointers found at offsets 50, 54, and 70.
- `process_icon_list_recursively_3`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a.
- `process_icon_list_recursively_2`: The library code iterates through a list in a4, calls another routine, and if it returns non-zero, iterates through a second list of pointers in a2, calling another routine recursively for each; it also checks a pointer at offset 70 of the structure in a2 to perform a recursive call and finally stores a value into offset 26 of the structure in a2.
- `process_icon_list_recursively`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a pointer or table.
- `traverse_icon_chain`: Iterates through a linked list of pointers, calling a library function for each non-zero element found and then calling another routine on the result.
- `restore_registers_only`: This library routine restores registers d2 and d3 from the stack and returns.
- `incremental_offset_recurse`: As library code, it calls another routine with the same first argument and the second argument increased by 140, then returns.
- `execute_conditional_library_logic`: Library code that calls a function pointer, then calls another routine with an offset from the result, then calls a third routine using a parameter and a stack slot, and finally calls a fourth routine with four arguments if the third returns a non-zero value.
- `call_restore_and_return_result`: Pushes several registers onto the stack, calls another routine, and then either returns zero or moves the value from a pointer in a2 into d0 before returning.
- `return_zero_and_restore_2`: This library routine sets d0 to zero, restores registers from the stack, and returns.
- `update_icon_table_entry`: Decrements a 16-bit counter through a pointer and, if the counter is not zero, stores two 32-bit values into a table at an offset calculated from the counter and a base address.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from registers and the stack into consecutive slots of that table.
- `execute_icon_logic`: As library code, it calls a routine with specific constants, and if the result is non-zero, it calls another routine, then calls a third routine with four arguments, and finally calls a fourth routine if certain conditions are met.
- `move_d6_to_d0`: Copies the contents of register d6 into register d0 and then branches to another location.
- `call_lib_func_and_return_zero`: Invokes a function pointer retrieved from a global variable associated with library index -210.
- `incremental_offset_recurse_2`: As library code, it calls another routine with the same first argument and the second argument increased by 78, then returns.
- `traverse_pointer_list`: As library code, it loops through a list of pointers, calling another routine for each element, and returns a pointer to a specific character if a match is found or zero if the list is empty or the match condition is not met.
- `restore_registers_and_return_4`: Clears register d0 and restores registers d2, d3, a2, and a3 from the stack before returning.
- `iterate_nested_values_2`: Library code that executes a loop calling another routine, updates a value in d4, and uses that value to calculate a new pointer.
- `iterate_nested_values`: As library code, it executes a loop that calls another routine, updates a value in d4, and uses that value to calculate a new pointer, returning zero or one based on specific comparison results.
- `increment_and_branch`: As library code, it tests register d4, and if it is not zero, it increments d4, moves the result to d2, and branches to another routine; otherwise it branches to another routine.
- `clear_d2_and_branch`: Sets register d2 to zero and then branches to another location.
- `restore_registers_and_return_5`: Clears register d0 and restores registers d2, d3, d4, d5, d6, and a2 from the stack before returning.
- `parse_numeric_string_2`: Library code that clears a byte at offset 30 of a structure, calls another routine to check a condition, and then loops through a string to calculate a numeric value by multiplying by 10.
- `increment_pointer_and_branch_3`: Library code that adds five to a pointer in a2, pushes two pointers onto the stack, calls another routine, and then either branches if d0 is non-zero or sets d5 to one and increments a2 before branching.
- `set_d5_and_branch`: Sets d5 to zero and performs a short branch.
- `parse_numeric_string`: As library code, it loops through a string to calculate a numeric value, then calls another routine to check a condition, and finally calls a library function at a fixed offset from a global base.
- `calculate_offset_value`: Compares a value from the stack with decimal 48 and decimal 57 to determine whether to set register d0 to the stack value, decimal 48, or decimal 57 minus 48.
- `set_d0_to_all_ones`: Sets register d0 to all ones and returns.
- `call_library_vector_198_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_conditional_library_calls`: Library code that calls another routine and uses the result to conditionally call a second routine or perform a cleanup call.
- `call_library_and_update_memory`: As library code, it calls a library function, and if the result is non-zero, it stores 10 at an offset of 14 from the returned address, stores 10 at the address in a parameter, and calls another routine with an incremented pointer.
- `restore_registers_and_return_3`: The library code clears d0, restores d2, d3, a2, and a7 from the stack, and returns.
- `compare_library_result`: The library code calls a library function and, if the returned value does not match the fourth parameter, returns zero; otherwise, it returns the result of the library call.
- `move_d1_to_d0_and_return`: The library code moves the contents of d1 to d0, restores d2 and d3 from the stack, and returns.
- `verify_library_return`: The library code calls a library function and, if the result is not equal to the fourth parameter, returns zero; otherwise, it returns the result of the library call.
- `transfer_d1_to_d0_and_restore_registers`: Moves a longword from register d1 to register d0 and restores registers d2 and d3 from the stack before returning.
- `divide_32_bit_integers_3`: Performs a 32-bit division using a loop that rotates a 32-bit value and subtracts a threshold, then calls another routine to return the result.
- `rotate_and_threshold_32bit_2`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_6`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit partial components and returns the result.
- `invoke_table_entry_4`: Loads a library base from a global variable and calls a function pointer located at an offset of 522 bytes from that base, passing three arguments from the stack.
- `store_byte_and_increment_pointer`: Stores a byte from register d0 into the memory location pointed to by a3, then increments the pointer in a3 by one before returning.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_and_decrement_pointer`: Library code that loads a pointer and a byte value from the stack, compares the byte to the value in d0, and if they are equal, decrements the pointer by 1 and moves it into d0 before returning; otherwise it returns.
- `set_zero_and_branch_2`: Library code that sets d0 to zero and performs a relative branch.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `skip_zero_byte`: Restores a0 and a1 from the stack, checks if the byte pointed to by a0 is zero, and if not, decrements a0 and branches back to the start of the routine.
- `compare_bytes_and_set_d0`: As library code, it decrements a value from the stack, and if it is not less than zero, it compares a byte from a pointer in a1 with a byte from a pointer in a0, and if they match and the byte in a1 is zero, it sets d0 to zero; otherwise it returns.
- `set_d0_conditional`: Sets register d0 to 1 if it is greater than zero, otherwise it sets register d0 to 255 and returns.
- `test_byte_and_decrement_d0`: As library code, it tests a byte at the address in a0, and if it is not zero, it decrements d0 if d0 is greater than zero, then branches to another routine; otherwise it branches to another routine.
- `decrement_pointer_and_call`: Decrements the address in register a0 by 1 and calls another routine.
- `copy_bytes_with_counter`: Decrements d0 and, if d0 is not less than zero, copies a byte from the address in a1 to the address in a0 and repeats the process; otherwise, it clears a single byte at the address in a0 and returns.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain_3`: Stores the address of a pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `conditional_library_branching`: The library code checks if a parameter is non-zero; if so, it calls another routine and, if that returns non-zero, calls three more routines in sequence, including one that uses a value from a pointer at offset 10 of a structure; otherwise, it returns one.
- `traverse_and_call_structure`: As library code, it enters a loop that calls a library function with a parameter, and if the result is non-zero, it calls the same library function again using a pointer and a value from an offset of the parameter, then updates the parameter using an offset of 12.
- `branch_with_constant_2`: The library code sets register d0 to one and then branches to another routine.
- `clear_d0_and_restore_registers_4`: Sets register d0 to zero and restores registers d2 through d5 and a2 from the stack before returning.
- `allocate_icon_structure`: Calls several routines to manage memory and data, and if successful, stores a 32-bit value at an offset of 10 from a pointer returned by another routine.
- `return_negative_one_stub`: This library routine returns negative one after restoring registers from the stack.
- `traverse_icon_linked_list`: Library code that iterates through a linked list, updating a pointer at offset 12 of a structure and performing several calls to other routines based on retrieved values and a loop condition.
- `traverse_and_update_linked_list`: Iterates through a linked list by repeatedly calling another routine to fetch the next element, updating a pointer at offset 12 of a structure and storing a value at offset 8 of a stack-based structure.
- `branch_with_longword`: The library code moves a longword from a3 to d0 and then branches to another routine.
- `return_negative_one_extended_stub`: This library routine returns negative one after restoring a larger set of registers from the stack.

### translator.library

- `initialize_library_structure`: Calls a function through a pointer at an offset from a global variable, and if the result is not zero, stores a global variable and a register value into a structure and calls another routine before returning a value.
- `clear_flag_and_increment_counter`: Increments a 16-bit counter in the structure it is given and clears bit 5 of that structure's flag byte.
- `decrement_counter_call_on_zero`: As library code, it decrements a 16-bit counter in the structure it is given.
- `modify_node_status`: Saves registers, updates a library base from a pointer at a fixed offset, checks a bit at offset 32 of a structure, and if that bit is set, sets bit 5 at offset 14 of that structure before branching.
- `unlink_node_and_call_library`: Removes the node from its doubly linked list, then calls a function through the Amiga library base and returns a value stored in the node.
- `return_constant_63`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_text_buffer`: Library code that fills a stack-based buffer with space characters using a loop and then performs conditional calls to other routines based on character checks.
- `move_word_and_branch`: As library code, it moves a 16-bit word from register d0 to d1 and then performs a short branch.
- `clear_byte_and_return_word`: As library code, it calls one unnamed library routine, clears the byte just before the pointer held in the address register, and returns the low 16-bit value of the data register.
- `execute_conditional_library_calls_3`: Library code that loads a pointer from a global structure, compares it to zero, and if non-zero, loads a second pointer, calls two routines, and then branches based on the sign of d7.
- `execute_conditional_library_calls_2`: Loads two pointers from a global structure, calls two routines sequentially, and then branches based on the sign of d7.
- `execute_conditional_calls_2`: As library code, it calls three different routines sequentially, using the results of the first two to determine whether to call a third, and restores two pointers from the stack before returning.

The function list continues in: g634-recovery-functions-2.md.
