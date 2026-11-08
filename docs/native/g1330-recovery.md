# Game 1330 (balance of power - the 1990 edition (1990)(mindscape)[cr pete]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01330). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916908 top-level bytes in 75 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 50 of 50 units identical (bytes and relocations), 55 raw-word lines |
| Functions named and described | pass, 2260 of 2260 named (278 author symbols), 2260 of 2260 described (426 by rule, 1730 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3299 of 3299 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 193177 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 34938 of 35161 instructions identical in start and length, 4 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 48775 |
| code | 117540 |
| data | 691469 |
| padding | 17652 |
| unused | 41472 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf:stack | 2 | 117 | 2 |
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
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf:CLI | 9 | 362 | 18 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 15 | 1414 | 108 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2476 | 170 |
| mathieeedoubbas.library | 3 | 632 | 60 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 13 | 830 | 53 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf:bop | 3 | 5064 | 305 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf:bootblock | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1700 | 37 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 150 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf bytes 603136-603648 (code never loaded by a reference run) | 1 | 28 | 1 |
| SavedGame (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- balance of power - the 1990 edition (1990)(mindscape)[cr pete]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf: file devs/printers/Okidata_92 (header 5476 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1478 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 93 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 20 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- balance of power - the 1990 edition (1990)(mindscape)[cr pete]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf: bytes 603136-603648 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[cr pete]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf:asave/SavedGame: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1990)(mindscape)[cr pete]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf: 1 logged loads cover 512 bytes of the image
- balance of power - the 1990 edition (1990)(mindscape)[cr pete]/Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf: blocks 112-120 (bytes 57344-61952) outside the filesystem hold data, not code (opcode density 0.025, entropy 4.98)
- 1 never-loaded region(s), 192665 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf:stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of register d1 to d0 and then branches to another routine.

### Format

- `parse_command_string_arguments`: Parses a command string by iterating through characters to extract space-separated or quoted arguments, handling escape sequences for newlines and escape characters, and stores the resulting tokens into a table of pointers.
- `execute_callback_chain`: Checks several global variables for non-zero values and executes corresponding function pointers stored at specific offsets.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `execute_callback_chain_3`: Sets a register to zero, calls a routine at an offset from a6, stores the result in a global variable, and returns if the result is not zero.
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
- `parse_disk_command_arguments`: Parses a command string to extract drive and name parameters, then iterates through a list of structures to perform file system operations or error reporting.
- `execute_callback_chain_2`: Checks five global variables for non-zero values and, if non-zero, calls another routine using the variable as a pointer and passing specific constant offsets as arguments.
- `traverse_and_invoke_table`: Traverses a multi-level pointer structure in a loop, checking for null pointers and specific values, then calls a routine with three arguments and returns a status.
- `report_disk_error`: Formats a error message string using a device name and a failure reason, then displays it to the user.
- `measure_string_length`: Returns the number of characters before the terminating zero byte in the given string.
- `copy_string`: Copies a null-terminated string from a source pointer to a destination pointer.
- `convert_to_uppercase`: If a byte is between 97 and 122 inclusive, subtracts 32 from it; otherwise, it returns the original value.
- `compare_case_insensitive_strings`: Compares two strings by converting each byte to uppercase via another routine and checking for equality until a null terminator is reached.
- `truncate_at_colon`: Iterates through a string pointed to by a register until it finds a colon or a null terminator, then writes a null terminator at that position.
- `manage_dynamic_task_structure`: Allocates a memory block and populates a structure with linked list pointers, a constant value, and a passed parameter.
- `save_and_reset_global_value`: Fetches a value from a structure via a function call, stores it in a global variable, and then writes a constant to the original structure location.
- `restore_global_value`: Fetches a value from a structure via a function call and restores a previously saved value from a global variable into that structure location.
- `initialize_stack_structure`: Populates a structure on the stack with several constant values and then calls another routine with a pointer to that structure.
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
- `rotate_32bit_value_3`: Performs a bitwise rotation on a 32-bit value using a loop that runs sixteen times, or performs modulo arithmetic if a specific condition is met.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment`: Stores a byte from register d0 into the memory location pointed to by a3 and increments a3.
- `format_disk_filesystem`: Allocates memory for a new filesystem structure and populates it with a signature and data blocks.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `manipulate_filesystem_structure`: Populates a data structure with various constants and pointers, then calls a routine to manage directory or file operations using the provided data.
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

- `parse_command_line_arguments_2`: Parses a string from a memory location into individual tokens, handling quoted strings and converting specific character sequences into newline or escape characters, then stores the resulting pointers in a table.
- `execute_library_calls_2`: Stores a parameter into a location near a global variable and calls two different functions through pointers located in a global variable and another global variable.
- `invoke_two_function_pointers_2`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `call_function_and_store_result`: Calls a function through a pointer located at an offset from the program counter, stores the returned value in a global variable, and returns if the value is zero.
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
- `manage_memory_list`: Allocates a memory block based on a value from a global variable and a constant, then enters a loop to fill the block with values from another routine.
- `loop_until_non_zero`: Enters a loop that calls another routine and checks the return value; if the return value is not zero, it breaks the loop, otherwise it checks a global variable and calls various other routines based on its value.
- `store_and_add_results`: Stores three values into a structure at offsets 28, 36, and 40, calls another routine, and then adds the returned value to a fourth parameter.
- `store_and_add_result`: Stores three values into a structure at offsets 28, 36, and 40, calls another routine, and then adds the returned value to a fourth parameter before storing it at offset 44 of the structure.
- `copy_disk_contents`: Performs a disk copy operation by iterating through directory structures of a source and destination disk.
- `initialize_disk_copy_operation`: Allocates memory and initializes I/O structures for a disk copy operation, then calculates the required number of disk swaps.
- `execute_conditional_sequence`: Checks a global variable and calls another routine with either one or two parameters depending on that value, then calls another routine with the constant 1 before returning.
- `execute_cleanup_sequence`: Checks several global pointers and, if non-zero, calls various routines to manage memory, delete ports, handle I/O requests, or execute indirect calls, including a loop that iterates through a table of pointers and clears them.
- `extract_and_dispatch_data`: Copies three values from a structure pointed to by a global variable into other global variables, calls another routine with that structure, and then calls another routine with a global variable if a specific global value is not 512.
- `execute_library_call`: Loads a word from the stack into d0, pushes a value from a global variable onto the stack, pushes d0 onto the stack, calls a routine at a fixed address, checks if d0 is non-zero to branch, clears the top of the stack, calls another routine, and returns.
- `perform_disk_copy_loop`: Iterates through a table of data to perform disk I/O operations, managing source and destination drives while handling errors and updating progress messages.
- `allocate_multiple_resources`: Checks a value at an offset of a global variable; if it is less than 33, it calls a routine and then another.
- `initialize_stack_and_call_library`: Populates a stack frame with several constant values and calls two different library functions using a global base.
- `handle_error_status`: Sets a global pointer to a global variable and calls another routine with various parameters based on the value of the first parameter, eventually calling another routine twice with the address of a global variable.
- `log_error_status`: Sets a pointer to a global variable and calls another routine with a parameter and the address of the global variable if a value is zero, otherwise it calls another routine.
- `process_io_and_errors`: Calls a routine and checks the result; if the result is zero, it calls another routine, then calls a third routine with a parameter and a value from a structure.
- `check_signal_bit`: Calls another routine with two parameters, checks if the 12th bit of the result is set, and if so, calls another routine with a global variable.
- `process_message_buffer`: If a global value is non-zero, it enters a loop calling another routine until it returns 10, otherwise it retrieves a message from a structure, extracts several fields into global variables, and calls another routine with that structure.
- `set_parameter_zero`: Stores zero in a word at an offset from a pointer and nine in a word at a different offset before calling another routine.
- `manage_memory_and_task`: Allocates memory and calls several library functions to manipulate a structure, including setting values at offsets 10, 20, 24, 28, and 40, before freeing memory and returning a value.
- `set_parameter_and_call`: Stores two different word values into an offset of a provided pointer and calls another routine multiple times.
- `call_routine_with_stack_parameters`: Pushes two different 16-bit values from a global variable onto the stack and calls another routine twice for each value.
- `process_filesystem_data`: Calls a routine with two parameters, and if the result is non-zero, it performs several operations including calling a routine to manage a structure, updating a global pointer, and executing a loop that sums 128 longwords from a table to subtract the total from a value at offset 5 of that structure.
- `execute_io_operation`: Calls another routine with four parameters, then calls another routine and returns whether the result is zero.
- `perform_io_and_compare`: Calls a routine to perform input/output and then calls another routine to store three values into a structure at offsets 28, 36, and 40, finally comparing a parameter to a value from a global variable.
- `allocate_message_entry_2`: Attempts to find a task, allocates memory, and if successful, populates the buffer with a value from a parameter, a byte from another parameter, and a constant, then calls a routine to set a pointer and calls another routine to dispatch the buffer.
- `delete_port_2`: Removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
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
- `calculate_string_length_2`: Increments a counter while checking bytes in a string starting at a pointer from the stack until a zero byte is found, then returns the count.
- `copy_memory_block`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `execute_indirect_calls`: Fetches a function pointer from a global structure and executes it multiple times with varying parameters.
- `invoke_dual_callbacks`: Calls another routine with a pointer to a global variable and a parameter, then calls another routine using four parameters including a local stack array.
- `convert_char_to_uppercase`: Returns the character with lowercase ASCII letters converted to uppercase by subtracting 32.
- `compare_strings_case_insensitive`: Compares two null-terminated strings by converting each character to uppercase and returns 1 if they match, otherwise it returns 0.
- `strip_colon_suffix`: Searches for a colon in a string and, if found, replaces it with a zero byte.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer`: Stores a byte from register d0 at the address pointed to by a3 and increments a3.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag_2`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag_2`: Calls modular_inverse_32bit and returns its first result, or the negation of that result if the second result in an address register is nonzero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment_pointer`: Stores a byte from register d0 into the memory location pointed to by register a3, then increments the pointer in a3 by one and returns.
- `call_formatted_print`: Passes a pointer from a stack location and a global variable to a routine that performs formatted printing, then returns.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_global_library_function_2`: Calls one library function through a library base read from a global variable, passing no arguments, and returns.
- `execute_with_status_check_2`: Calls a routine via a global pointer and returns either a byte from a stack slot if the return value is 1, or a negative error code otherwise.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.

### Port-Handler

- `parse_amigados_hunks`: Parses a structure containing AmigaDOS hunk type codes to identify executable or relocation segments.

### Disk-Validator

- `update_display_buffer`: Converts a numeric value into a character and stores it in a buffer, then enters a loop to clear a range of memory and performs bitwise operations to update a data structure based on a calculated index.

### Ram-Handler

- `execute_jump_table_entry`: Sets d0, saves registers, moves d1 and d2 to a0 and a1, calls a routine using a pointer from a6, restores registers, subtracts a0 from itself, and jumps to an address in a6.

### serial.device

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reinit_serial_buffer_fields`: If a byte at offset 30 of a structure in a1 is non-zero, it calls a routine via a library vector; it then checks if d2 is non-zero to call another routine, updates several fields in the structure at a6 with the value of d2 or a returned value, and stores half and quarter of d2 at offsets 298 and 296.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `set_status_byte`: Stores the byte value fd at offset 31 of a structure in a1 and branches to another routine.
- `toggle_interrupt_enable`: Writes 801 to the INTENA register, calls another routine, writes 8801 to the INTENA register, and returns.
- `test_command_type`: Clears bit 7 of a byte at offset 31 of a1 and branches to different locations based on whether the 16-bit value at offset 28 of a1 is equal to 2, 3, or 10.
- `clear_pointer_on_match`: Compares a longword at an offset of a6 with a1; if they are equal, it clears bit 1 of a byte at an offset of a6, clears the longword at that same offset, and then branches to another routine.
- `unlink_second_queue_request_2`: Checks bit 6 of a byte at offset 30 of a structure in a1; if set, it unlinks a node from a linked list by updating pointers and decrements a 16-bit value at offset 70 of a structure in a6.
- `call_library_vector_480_2`: Compares a pointer in a1 with a value at offset 78 of a structure in a6; if they match and a 16-bit value at offset 28 of a structure in a1 is 10, it clears bit 5 of a byte at offset 470 of a structure in a6, writes a 16-bit value to a hardware register, calls another routine, clears a 32-bit value at offset 78 of a structure in a6, and clears bit 4 of a byte at offset 472 of a structure in a6.
- `unlink_second_queue_request`: If bit 6 of a byte at offset 30 of a structure in a1 is set, it unlinks a request from a list by updating pointers, decrements a counter at offset 72 of a structure in a6, modifies flags and a byte at offset 31 of the request, and calls a routine if bit 0 of the flag byte is clear.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `invoke_library_vectors_then_unlink_2`: Sets a 32-bit register to zero, checks a 16-bit value at offset 32 of a structure in a6, and if non-zero, sets bit 6 of a byte at offset 472 of a structure in a6 before returning.
- `invoke_library_vectors_then_unlink`: Calls four different library routines using vectors from a structure in a6, then unlinks a block from a list by updating pointers and returns a value from offset 426 of the structure in a6.
- `insert_node_into_linked_list`: Inserts a new structure into a linked list by updating pointers and increments a counter in a structure, then calls a handler routine.
- `process_serial_data_buffer_queue_2`: Iterates through a linked list of data buffers, decrementing a counter and updating a pointer to the next buffer in the list.
- `process_serial_data_buffer_queue`: Iterates through a linked list of data buffers, decrementing a counter and updating a pointer to the next buffer in the list.
- `test_status_bit_2`: Tests bit 0 of a byte at offset 475 of a6 and branches to one location if the bit is clear, otherwise it branches to another location.
- `set_status_byte_and_branch`: Calls another routine, and if the result is zero, sets a byte at an offset of a1 to nine and branches to another routine; otherwise it branches to another routine.
- `process_serial_data_buffer`: Iterates through a linked list of data buffers, performing bitwise operations on status bytes and updating counters.
- `restore_registers_and_return_9`: Returns immediately after restoring D2 and A6 from the stack.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `clear_bit_and_branch`: Clears bit 2 of a byte at offset 470 of a6, and if the bit was clear it branches to another routine, otherwise it writes 15 to offset 31 of a1 and branches to another routine.
- `clear_bit_and_set_byte`: Clears bit 7 of a byte at offset 470 of a6 and branches if the bit was already clear, otherwise it writes 9 to offset 31 of a1 and branches to another location.
- `modify_status_and_clear_counter`: Sets a byte at an offset of a1 to twelve, tests bit 0 of a byte at an offset of a6, and if bit 0 is clear, branches to another routine; otherwise it sets the byte at the same offset of a1 to six, clears a longword at an offset of a6, masks bit 4 through 7 of a byte at an offset of a6, and branches to another routine.
- `process_serial_data_buffer_2`: Checks status bits in a structure to manage serial communication state, updates a data buffer by copying a byte from a register to a pointer, increments buffer pointers and counters, and calls another routine via a function pointer in the structure.
- `toggle_serial_interrupt_state`: Compares a byte at offset 34 of the structure in a6 with d1 and, if equal, tests bit 1 of a byte at offset 472 of the structure in a6; if that bit is set, it clears bit 1 at that offset and enables interrupts by writing to the INTENA register, otherwise it returns.
- `test_serial_status_bits`: Tests bit 5 of a byte at offset 376 of a structure in a6, and if it is clear, tests bit 1 of a byte at offset 445 of a6; if that bit is also clear, it calls another routine, otherwise it tests bit 8 of d0 and branches accordingly.
- `test_bit_and_branch_2`: Tests bit 8 of d0, and if the bit is set it branches to another routine, otherwise it branches to another routine.
- `test_status_bit`: Tests bit 0 of a byte at offset 445 of a structure in a6 and branches to another routine.
- `set_bit_and_copy_pointer`: Sets bit 7 of a byte at offset 470 of a6, tests a longword at offset 62 of a6, and if that longword is non-zero copies a longword from offset 42 of a6 to offset 62 of a6.
- `process_buffer_data`: Compares two longwords at offsets 54 and 58 of a6, and if the value at offset 58 is less than the value at offset 54, it writes a byte to a pointer at offset 42 of a6, increments two values at offsets 58 and 42, and potentially updates a pointer at offset 42 from offset 38 if a threshold is met.
- `record_pointer_then_signal`: Tests a longword at an offset of a6, and if it is non-zero, copies a longword from an offset of a6 to that same offset, calls another routine, and then calls a library vector using a pointer found at an offset of a6.
- `clear_interrupt_bit`: Copies a byte from an offset of a6 to an offset of a6, clears bit 3 of a byte at an offset of a6, tests bit 2 of a byte at an offset of a6, and if bit 2 is clear, clears bit 6 of a longword at a specific address and branches to another routine; otherwise it branches to another routine.
- `update_interrupt_settings`: Moves a byte from offset 35 of a structure in a6 to offset 473 of the same structure, sets bit 3 of offset 472, tests bit 2 of offset 69 of a structure in a6, and if bit 2 is clear, branches to another routine; otherwise, it sets bit 6 of a hardware register, tests bit 7 of offset 69, and if bit 7 is set, writes the word 32769 to a hardware register.
- `clear_status_byte_2`: Clears a byte at offset 473 of the structure in A6 and returns.
- `update_status_and_signal`: Sets bit 6 of a byte at offset 214 of a structure in a6, clears a longword at offset 258, copies a longword from offset 106 to offset 262, subtracts 10 from the value at offset 262, sets a word at offset 254 to 9, and calls another routine; if a byte at offset 31 of a register is non-zero, it sets that byte to 11.
- `clear_bit_and_check_interrupt`: Saves d2, a3, and a6 to the stack, copies a1 to a6, clears bit 6 of a byte at offset 470 of a6, and if the bit was clear it branches to another routine, otherwise it checks if the value in the interrupt mask register is non-zero, and if it is zero it sets bit 2 of a byte at offset 470 of a6 and branches to another routine.
- `load_address_and_branch_6`: Loads the address at offset 418 of a structure in a6 into a0, calls another routine, and then branches to another routine.
- `set_bit_and_branch_3`: Sets bit 0 of a byte at offset 470 of a6 and branches to another routine.
- `manage_serial_request_queue`: Clears a value at offset 8 of a structure, modifies a status byte at offset 30, and either updates a pointer and a global variable or inserts the structure into a linked list.
- `process_interrupt_request_state`: Tests a bit in a structure, calls another routine if set, then checks a byte in a structure to conditionally set bit 4 and update a pointer and a long value in a structure before enabling interrupts via the INTENA and INTREQ registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_request_state_on_flags`: Checks if two specific bits in a global variable are set, and if so, writes a byte with a value of thirteen to an offset within a structure pointed to by a register.
- `process_serial_request_queue`: Manages a linked list of serial request nodes by popping the head of the list, updating its status flags, and decrementing a remaining request counter.
- `update_serial_buffer_pointer`: Writes 200 to offset 476 of a structure in a6, moves a pointer from offset 86 of a6 to a0, clears d0, and loads a byte from the address in a0 into d0; it then tests bit 4 of a byte at offset 472 of a6 and, if set, decrements a longword at offset 94 of a6.
- `update_serial_status_and_call_handler_2`: Updates a bitmask and increments two counters within a structure, then performs a comparison loop against a byte sequence before calling another routine.
- `update_serial_status_and_call_handler`: Updates a bitmask and increments two counters within a structure, then performs a comparison loop against a byte sequence before calling another routine via a pointer.
- `process_linked_list_node_2`: If a shortword at an offset of a6 is non-zero, it performs several operations on a linked list starting at an offset of a6, including updating pointers, modifying a byte at an offset of a structure in a1, and calling another routine; otherwise it sets a global variable to one and returns.
- `enable_interrupt_bit`: Restores D2 and A6 from the stack, writes the value 1 to the INTENA hardware register, and returns.
- `set_interrupt_and_clear_data`: Writes the word 1 to a hardware register, clears d1, moves a byte from offset 473 of a structure in a1 to d1, performs a bitwise OR with a word from offset 468 of the same structure into d1, clears offset 473 of the structure in a1, moves d1 to a hardware register, and then conditionally returns based on values at offsets 78 and 72 of the structure in a1.
- `update_serial_device_state`: Updates a structure with several longword values, modifies a byte at offset 69 of a structure in a6, and calls a handler via a pointer in a structure if a bitmask test on a byte at offset 30 passes.
- `call_handler_and_branch`: Calls another routine and then branches to another routine.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `set_flag_and_call_handler`: If a longword at an offset of a6 is non-zero, it sets bit 2 of a byte at an offset of a6; if a longword at another offset of a6 is non-zero, it calls another routine; finally, if a specific bit pattern is not found in a byte at an offset of a1, it calls another routine using a pointer from a structure in a6.
- `clear_interrupt_and_call`: Clears bit 3 of a byte at offset 472 of a6, loads a longword from offset 74 of a6 into d0, and if d0 is non-zero, calls another routine; it then sets a global variable to 8001 and, if a byte at offset 30 of a register has bits 0 and 8 unset, calls another routine.
- `set_interrupt_and_call`: Writes the value 2049 to the INTENA hardware register, calls another routine, writes the value 34817 to the INTENA hardware register, and then branches to another routine.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `update_pointer_and_loop`: Copies a pointer from A0 to A1, then if the value at the A1 pointer is non-zero, it writes that value to the location in A0, exchanges D0 and A1, writes the A0 pointer to offset 4 of the A1 structure, and if D0 is non-zero, sets A1 to D0, calls another routine, and loops back to the start of the routine; otherwise, it continues.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_status_and_dispatch_handler`: Updates a byte at offset 81 of a structure using a masked value and bitwise logic, copies a 16-bit value from a structure in a6 to offset 80 of a structure in a1, and calls a handler routine if a bitmask test on a byte at offset 30 of the a1 structure passes.
- `test_request_and_buffer_status`: Clears bit 0 of a byte at an offset of a1, tests bit 3 of a byte at an offset of a1, and if bit 3 is set, tests a word at an offset of a6 and a longword at another offset of a6, branching to another routine if either is non-zero; otherwise it branches to another routine.
- `handle_request_and_interrupt`: Moves a longword from offset 106 of a structure in a6 to d0, calls another routine, tests bit 31 of offset 31 of a structure in a1, and if set, branches to another routine; otherwise, it stores a pointer from a1 into offset 78 of a structure in a6, sets bit 5 of offset 46 of the structure in a6, and writes the word 34816 to a hardware register before branching to another routine.
- `reply_and_clear_active_request`: Pushes A6 to the stack, sets A6 to a pointer from offset 98 of the current A6 structure, calls a routine via a pointer, restores A6 from the stack, clears a 32-bit value at offset 78 of the A6 structure, and returns.
- `clear_uart_break_and_reply`: Clears bit 5 of a byte at offset 1d6 of a structure in a1, and if bit 5 of that byte and a non-zero value in a6 are set and a short at offset 1c of a pointer in a1 equals 10, it sets bit 1 of the byte at offset 1d6, clears a pointer at offset 4e of a1, sets a global variable to 800, and calls a function via a pointer in a6.
- `start_timed_read_request`: Sets a global variable to one, clears a 32-bit value at offset 142 of a structure in a6, stores a value from d0 at offset 146 of a6, sets a 16-bit value at offset 13e of a6 to nine, and calls a function via a pointer in a6.
- `modify_interrupt_status`: Tests bit 7 of a byte at offset 79 of a1, and if set, sets bit 7 of a byte at offset 69 of a6, clears bit 2 of a byte at offset 472 of a6, and tests bit 3 of a byte at offset 472 of a6; if that bit is set, it clears bit 7 of a byte at offset 79 of a1, calls another routine, sets bit 7 of a byte at offset 79 of a1, and returns.
- `update_interrupt_status`: Clears bit 7 of a byte at offset 69 of a structure in a6, checks bit 1 of a byte at offset 472 of a structure in a6, and checks if a longword at offset 78 of a structure in a6 is non-zero.
- `toggle_interrupt_mask`: Writes 801 to the INTENA register, calls another routine, writes 8801 to the INTENA register, and if a byte at offset 30 of a register has bits 0 and 8 unset, calls another routine.
- `validate_and_sync_serial_parameters`: Validates a range of configuration bytes and a longword value within a structure, then copies several fields to a secondary structure and performs bitwise updates on a status byte.
- `compute_unit_channel_masks`: Calculates a 16-bit mask by shifting 1 based on a byte at offset 4d of a1, modifies the mask if a byte at offset 4e of a1 is two or if bit 0 of a byte at offset 45 of a6 is set, stores the mask at offset 1d4 of a6, and calls two other routines to store results at offsets 1d2 and 1d0 of a6.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_byte_and_branch`: Sets bit 2 of a byte at offset 31 of a structure in a1 and branches to another routine.
- `calculate_serial_period`: Loads a longword from offset 450 of a structure in a6 into d0, copies it to d1, shifts d0 left by 3, subtracts d1 from d0, compares d0 to 65535, and if d0 is less than or equal to that value, branches to another routine; otherwise, it shifts d0 right by 5, performs a 16-bit unsigned division of d0 by d1, masks the result to 16 bits, shifts it right by 5, and branches to another routine.
- `store_serial_period_quotient`: Divides a 32-bit value by a 16-bit value and stores the quotient at offset 1ce of a structure in a6, clears bit 5 of a byte at offset 1d8 of a6, and if a byte at offset 42 of a6 is eight and bit 0 of a byte at offset 45 of a6 is set, it performs a bitwise OR on the quotient with 15 and sets bit 5 of the byte at offset 1d8 of a6.
- `configure_serial_device_2`: Calls a routine using a pointer from a structure, stores a value in a structure at offset 330, and performs multiple calls to another routine while updating several fields in a structure and a global variable.
- `configure_serial_parameters`: Loads two different addresses into a0 and a3, calls another routine twice, and then branches to another routine.
- `copy_pointer_to_offset`: Stores the value in a0 into a 32-bit location at offset 10 of a structure in a1.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `configure_serial_device`: Updates several fields in a structure, including setting bytes and longwords at various offsets, calls a library function via a pointer, and sets a global variable to 2049.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_d0_and_branch`: Clears the d0 register and branches to an earlier instruction.

### parallel.device

- `synchronize_parallel_buffer_state`: Checks status bits in two structures, increments a 16-bit counter if certain conditions are met, updates a status byte, and conditionally copies 8 bytes of data from one structure to another.
- `update_parallel_buffer`: Increments a 16-bit value at offset 32 of a structure in a6, copies a byte from offset 117 of a structure in a6 to offset 53 of a structure in a1, sets bit 4 of offset 118 of the structure in a6, and if bit 1 of the byte at offset 53 is set, copies two 32-bit values from offsets 90 and 94 of the structure in a6 to offsets 54 and 58 of the structure in a1.
- `update_device_status_and_data`: Updates a status byte based on a bitmask from a structure, increments a counter if two specific bytes are zero, and copies a 32-bit value from one structure to another if a bit is set.
- `process_parallel_interrupt_status`: Checks a status value in a structure and updates a global variable and a status byte at offset 118 in a6.
- `handle_parallel_interrupt`: If a 16-bit value at offset 1c of a1 is three, it clears bit 0 of a global variable, and if bit 0 of that global variable is zero and bit 2 of a byte at offset 76 of a6 is not set, it calls a function via a pointer in a6; it then sets bit 2 of the byte at offset 76 of a6 and calls another function via a pointer in a6 using a1 and a6 as arguments.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `handle_parallel_interrupt_2`: Writes the byte 253 to offset 31 of a structure in a1 and branches to another routine.
- `test_and_clear_long`: Tests a longword at offset 36 of a structure in a1; if it is not zero, it branches to another routine, otherwise it clears a longword at offset 32 of the structure in a1 and branches to another routine.
- `update_device_status_2`: Clears bit 1 of a byte at offset 30 of a structure in a1, then checks bit 2 of a byte at offset 118 of a structure in a6.
- `update_device_status`: Sets a bit in a byte at offset 30 of a structure, copies a 32-bit value from offset 40 to offset 42 of another structure, and modifies status bits in a structure based on values at offsets 36 and 28.
- `process_device_event_queue`: Manages a linked list of pending device events by popping elements from the list, updating device counters, and performing a comparison loop through a byte sequence.
- `update_device_counters_and_call_callback`: Increments two 32-bit counters in a structure, performs a comparison loop through a byte sequence, and calls two function pointers after clearing specific bits in status bytes.
- `modify_device_state`: Copies a 32-bit value from offset 46 of a6 to offset 32 of a1, clears bit 3 of a byte at offset 118 of a6, and clears bit 4 of a byte at offset 30 of a1 before calling another routine.
- `poll_cia_bit_and_branch`: Reads a longword from a pointer in a0 into a memory location, then checks bit 3 of a byte at offset 118 of a6; if that bit is set, it branches to another routine, otherwise it loops back to the start of this routine.
- `process_device_update`: Calls another routine, loads a pointer from offset 77 into d0, sets a6 to a pointer from offset 32 of the structure in a6, calls a routine at an offset in a6, restores a6 from the stack, and branches to another routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_if_not_zero`: Checks if a value in a structure is non-zero and calls a function through a pointer if it is.
- `invoke_callback_pointer`: Executes a function through a pointer stored in a structure if the associated index is not zero.
- `process_linked_list_node`: Loads a pointer from offset 76 of a6 into a1, moves the longword at that pointer into d0, and if d0 is not zero, stores d0 back at offset 76 of a6, exchanges d0 with a1, and stores a1 at offset 4 of the new a1.
- `clear_offset_72`: Clears a 16-bit value at offset 72 of a structure in a6 and returns.
- `modify_status_byte`: Sets bit 5 of a byte at offset 30 of a structure in a1 and stores the value 254 into the byte at offset 31 of the same structure, then calls a function via a pointer.
- `update_global_and_field`: Manipulates bits within a global variable and updates a byte in a structure using a library function call.
- `set_request_byte`: Tests bit 1 of a byte at offset 76 of a structure in a6; if it is set, it writes the byte 1 to offset 1f of a structure in a1 and branches to another routine.
- `copy_structure_fields`: Copies a byte from one structure to another and, if a specific bit is set, copies two long words as well.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `recurse_on_pointer`: Saves a0 and a6 to the stack, calls itself, and then restores the registers and returns.
- `recursively_calls_itself`: This routine performs an infinite recursive call to itself, or otherwise acts as an empty wrapper that calls itself before returning.
- `call_cleanup_routine`: Pushes the contents of a1 onto the stack, calls another routine, adds four to the stack pointer, and returns.
- `jump_to_handler`: Performs a long jump to an address in a table.
- `decrement_and_check_counter`: Pushes a1 onto the stack, calls another routine, and then increments the stack pointer by 4 before returning.
- `call_external_routine`: Pushes the contents of a1 onto the stack, calls a routine, and then increments the stack pointer by 4.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `execute_clipboard_logic_2`: Saves d2-d3 and a2 to the stack, loads a longword from offset 10 of the stack into d3, loads a pointer from offset 14 of the stack into a2, stores a2 into a global variable, clears the top of the stack, pushes the address 0 onto the stack, calls a routine, stores the result in a global variable, increments the stack pointer by 8, and branches if the result is not zero.
- `execute_clipboard_logic`: Stores a value from d3 into a global variable and calls two different routines.
- `call_two_functions`: Calls two different functions via library vectors and returns one.
- `update_clipboard_data`: Calls another routine and uses the result to either set a 16-bit value at offset 94 of a structure to zero or update a 32-bit value at offset 14 of a structure using a value from offset 14 of another structure plus one, then calls another routine.
- `clear_and_call_routine`: Clears a longword at offset 14 of a structure in a3, pushes a pointer to offset 72 of a structure in a2 onto the stack, and calls a routine.
- `zero_word_field`: Stores a zero word into a specific offset within a structure and then calls another routine.
- `process_string_data`: Saves d2 and a2 to the stack, loads a pointer from offset 12 of the stack into a2, calls a routine, loads a pointer from offset 72 of the structure in a2 into a0, loads a longword from the address in a0 into d2, and if d2 is not zero, pushes the address 530 onto the stack, pushes a0 onto the stack, calls a routine, moves d2 into a0, increments the stack pointer by 8, and branches to another routine.
- `update_pointer_chain`: Pushes the address at offset 72 of a2 onto the stack, calls another routine, clears a longword at offset 86 of a2, and calls another routine.
- `copy_buffer_segment`: Copies a specified number of bytes from a linked list of 512-byte buffers into a destination buffer, traversing the list as needed.
- `compare_data_size`: Loads a long word from an offset of 90 in a2 into d0, compares it with a long word at offset 48 of the stack, and branches if the value is lower; otherwise, it performs several arithmetic operations involving d0, d1, and d3 using values from offsets 48 and 44 of the stack and branches based on the result.
- `copy_clipboard_data_blocks`: Uses values from the stack to traverse a linked list of structures to find a specific node, then enters a loop to copy bytes from the data at offset 18 of the found structure to a destination pointed to by a1, handling block boundaries and updating the pointer in a structure at offset 86 of a2.
- `compare_buffer_size`: Copies a longword from d1 to d0, compares d0 against a longword at offset 14 of a1, and if d0 is not less than or equal, updates a1 with the value at offset 0 of a1 before branching.
- `calculate_buffer_limit`: Copies a longword from d3 to d5, and if d5 is non-zero, compares the value 512 minus a word in d2 against d5; if d5 is less than or equal to that result, it moves the word in d2 to d4 and branches to another routine.
- `copy_bytes_with_limit`: Copies bytes from a table starting at offset 18 of a1 to a destination in a3 using a counter in d4, and loops until the counter reaches 512.
- `transfer_control_flow`: Moves a 16-bit word from d2 to d4 and branches to another routine.
- `copy_bytes_until_limit`: Copies bytes from a table starting at offset 18 of a1 to a destination in a3 using a counter in d4, and loops as long as the counter is less than the sum of d1 and d2.
- `store_long_word`: Stores a long word into a specific offset within a structure and then calls another routine.
- `update_fragmented_clipboard_buffer`: Iterates through a linked list of memory blocks to copy a source byte stream into them, managing block offsets and remaining byte counts.
- `copy_buffer_to_linked_list`: Iterates through a linked list of memory blocks to copy a source byte stream into them, managing block offsets and remaining byte counts.
- `call_fragmented_clipboard_copy`: Pushes the address stored at an offset of 72 from a pointer in a2 onto the stack, calls another routine, adjusts the stack pointer by 4, and performs an unconditional short branch.
- `copy_fragmented_clipboard_data`: Traverses a linked list of memory blocks to find a target node based on bit-shifted values, then copies bytes from a source pointer into the identified nodes until a remaining byte count reaches zero.
- `transfer_clipboard_data_linked`: Retrieves bit-shifted values from offset 24 of a6, traverses a linked list of structures to locate a target node, and then enters a loop to copy bytes from the data at offset 18 of the current node to a destination pointed to by a3, updating the remaining count at offset 20 of a6 and the list pointer at offset 86 of a2.
- `transfer_clipboard_data`: While a pointer in d4 is not null and a longword at offset 14 of d4 is less than d2, it follows the pointer; if the pointer is null, it calls another routine.
- `check_buffer_bounds`: Tests d4, and if non-zero, tests a longword at offset 20 of a6; if that longword is not less than or equal to zero, it compares 512 minus d3 against the longword at offset 20 of a6; if the result is not greater than or equal to the longword, it moves the word in d3 to d1 and branches to another routine.
- `update_clipboard_data_2`: Copies bytes from a source to a destination at an offset of 18, updates a 32-bit value at offset 14 of a structure, and calls another routine.
- `transfer_control_flow_2`: Moves a 16-bit word from d3 to d1 and branches to another routine.
- `copy_buffer_data`: Copies bytes from a source pointer to a destination offset in a loop, potentially calling another routine to fetch more data if a null pointer is encountered.
- `store_register_to_offset`: Stores the contents of a data register into a memory location calculated by adding a fixed offset to a pointer held in an address register.
- `check_bits_and_call`: Checks bit 7 and bit 0 of a byte at offset 30 of a structure in a2, and if neither bit is set, calls another routine.
- `modify_byte_bits`: Clears the least significant bit and sets the most significant bit of a byte at a specific offset in a buffer.
- `modify_byte_bits_2`: Checks bit 7 of a byte at offset 30 of a structure in a2, calls another routine if it is not set, and clears bit 7 and bit 4 of that byte under specific conditions.
- `compare_structure_values`: Compares a longword at offset 30 of a structure in a0 with a longword at offset 30 of a structure in a1, returning 1 in d0 if the first is less than the second.
- `set_zero_and_branch`: Sets d0 to zero and branches to another location.
- `set_d0_to_zero_and_branch`: Sets register d0 to zero and performs a short branch to another location.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_structure_fields_2`: Copies a 32-bit word from offset 28 of a structure to offset 98 of another, a 16-bit word from offset 10 to offset 92, and a 32-bit word from offset 30 to offset 94 before calling another routine.
- `modify_buffer_status_flags`: Sets bit 5 of a byte at offset 30 of a structure pointed to by a value at offset 12 of the stack frame, then checks a byte at offset 66 of a second structure; if that byte is not zero, it clears bit 5 of the byte at offset 30 and otherwise returns.
- `reconcile_buffer_fragmentation_states`: Iterates through a structure to manage buffer metadata by checking status flags and lengths, adjusting fragmentation flags, and updating buffer size values.
- `clear_d3_and_branch_2`: Clears a 16-bit word in d3 and branches.
- `process_clipboard_buffer_state`: Updates buffer metadata and manages fragmentation by checking status flags and lengths within structures.
- `call_and_set_flag`: Pushes the value of a2 onto the stack, calls another routine, pops the value from the stack, stores 1 into offset 94 of the structure in a2, and branches.
- `update_buffer_state_metadata`: Updates buffer metadata by checking status flags and lengths within structures, adjusting fragmentation flags, modifying buffer size values, and updating counters based on specific state constants.
- `clear_d3_and_branch`: Clears the 16-bit register d3 and branches to another routine.
- `update_clipboard_buffer_states`: Manages clipboard buffer metadata by updating lengths, adjusting fragmentation flags, and modifying status bytes within structures.
- `manage_clipboard_buffer_states`: Updates buffer lengths, manages fragmentation, and modifies status flags within a structure based on current buffer state and pointer values, calling synchronize_clipboard_buffer_state and synchronize_clipboard_buffer as needed.
- `compare_and_update_pointer`: Loads a longword from offset 90 of a structure in a2 into d2, adds a longword from offset 44 and a longword from offset 44 of a structure in a4 to d0, compares d0 to d2, and if d0 is not lower than d2, copies a longword from offset 36 of a structure in a4 to offset 32 of the same structure before branching to another location.
- `calculate_offset_difference`: Loads a longword from an offset of 90 of a structure in a2 into d2, subtracts a longword at an offset of 44 of a structure in a4 from d2, stores the result at an offset of 32 of the structure in a4, and then adds the value at offset 32 of a4 back to the value at offset 44 of a4.
- `synchronize_clipboard_buffer`: Clears a longword at an offset of a4, sets a longword at an offset of a4 to negative one, decrements a word at an offset of a2, pushes a4 and a3 onto the stack, calls another routine, and if the word at the offset of a2 is non-zero, clears a word at an offset of a2 before branching to another routine.
- `synchronize_clipboard_buffer_state`: Checks a status field and a pointer in a structure to update a length counter and manage buffer fragmentation.
- `update_clipboard_buffer`: Copies a longword from an offset in a4 to an offset in a2, pushes two pointers onto the stack, calls another routine, cleans the stack, and branches to another routine.
- `compare_and_set_word`: Library code that compares a word at offset 28 of a structure in a4 with the value four in d0, and if they are equal, writes the word four to offset 64 of a structure in a2 before branching to another routine.
- `clear_clipboard_word`: Clears a 16-bit register and branches to another routine.
- `process_buffer_state_and_counters`: Checks a status field at offset 64 of a structure, updates a counter at offset 90 based on the result of another routine, and clears a bit in a byte at offset 30 of a structure.
- `clear_and_branch_5`: Clears a longword at offset 90 of a2 and branches to another instruction.
- `update_structure_state_and_counters`: Checks status bits in a structure and updates various integer fields and counters based on the values of pointers stored at offsets 112 and 96 of the structure in a2.
- `traverse_linked_list_2`: Traverses a linked list by following pointers, comparing a value at offset 12 of the current element with a value at offset 48 of a structure in a2, and calls another routine with a value from the last element visited.
- `set_zero_and_branch_3`: Sets register d1 to zero and branches to another location.
- `call_library_function`: Loads a library base from a global variable and calls a library function at a specific offset.
- `set_byte_value`: Writes a constant byte value to an offset within a structure.
- `set_byte_constant_offset`: Writes a constant byte value to a specific offset within a structure and then calls another routine.
- `call_set_byte_value`: Pushes a pointer and a value onto the stack, calls a routine to write a byte to an offset within a structure, and then returns.
- `call_set_byte_value_2`: Pushes a pointer and a constant byte value onto the stack, calls set_byte_value, and returns.
- `write_byte_and_call_handler`: Stores a byte value from a register into a specific offset of a structure and then calls another routine.
- `update_structure_byte_sequence`: Updates a byte at a specific offset within a structure by calling three separate routines in sequence, each performing a write operation and a handler call.
- `update_structure_from_pointer`: If a 32-bit value at offset 48 of a structure in a2 is zero, it copies a 32-bit value from offset 56 of a second structure pointed to by offset 24 of the first structure into offset 48, and sets bit 4 of a byte at offset 30.
- `call_subroutine_with_offset`: Calls another routine using a value from a register plus an offset of 96 and then calls a second routine.
- `increment_structure_counter`: If a 32-bit value at offset 48 of a structure in a2 is zero, it increments a 32-bit value at offset 56 of a second structure pointed to by offset 24 of the first structure and stores the result in offset 48.
- `call_subroutine_with_different_offset`: Calls another routine using a value from a register plus an offset of 112 and then calls a second routine.
- `copy_structure_field`: Copies a longword from an offset relative to a pointer to another offset in a structure.
- `copy_structure_field_offset`: Copies a longword from a different offset relative to a pointer to a field in a structure.
- `search_and_manage_linked_list`: Traverses a linked list to find a node matching a specific value; if found, it increments counters and updates a pointer in a structure, otherwise it attempts to allocate a new node, initializes its members, and performs a series of data transfers and memory checks within a loop.
- `compare_and_clear_registers`: Compares a longword at offset 14 of a structure in a3 with d3, and if they are not equal, clears d4 and d2 before branching.
- `manage_clipboard_data_transfer`: Increments a counter in a global variable and updates a structure with a pointer, then performs a complex sequence of data transfers and memory management involving conditional loops to copy data between buffers and update status flags within a structure.
- `synchronize_clipboard_data`: Initializes a structure with a global variable and several flags, then performs a series of library calls to manage data transfers.
- `call_library_and_decrement`: Calls a library function with a pointer to offset 18 of a structure in a0, and if the result is not 512, clears a word at offset 94 of a structure in a3 and decrements a value at offset -4 of a structure in a6.
- `process_clipboard_status`: Tests a word at offset 94 of a3 and branches if it is zero; tests a longword at offset -4 of a6 and branches if it is less than or equal to zero.
- `process_clipboard_data`: Performs multiple conditional checks and library calls involving a structure in a3, updates a word at offset 94 of a3, clears a longword at offset 90 of a3, clears a byte at offset 67 of a3, and enters a loop that calls another routine and potentially calls another routine using a value from the returned pointer; finally, it returns a byte from offset 31 of a register.
- `process_complex_structure`: Increments values in a structure, performs conditional library calls based on a flag, and returns a value from a structure offset.
- `decrement_and_cleanup_clipboard_structure`: Decrements a reference counter in a structure and, if it reaches zero, performs a multi-stage cleanup involving the traversal of a linked list, clearing structure fields, and decrementing a global counter.
- `verify_library_result`: Calls a library function with a pointer to offset 18 of a structure in a4, compares the result to d4, and if they are not equal, clears a word at offset 94 of a structure in a3 and sets d4 to zero.
- `clear_status_and_branch`: Clears a word at offset 94 of a structure in a3 and branches to another location.
- `cleanup_clipboard_structure`: Calls another routine three times, then conditionally calls another routine using a value at offset 68 of a3.
- `cleanup_structure_data`: Resets several structure fields to zero, decrements a global counter, and conditionally calls another routine if the counter reaches zero.
- `execute_library_sequence`: Tests a word at offset 252 of a structure, calls a library function using a pointer from a global table, calls another library function with a pointer from offset 220 of a structure, and moves a value from offset 314 of a global table into d0.
- `set_global_bit`: Sets bit 3 of a global variable and returns zero.
- `update_structure_and_call`: Writes a constant byte to offset 8 of a structure, performs a bitwise AND on offset 30, and calls another routine if a comparison with 12 fails.
- `invoke_table_entry_3`: The routine invokes a function pointer retrieved from an offset within a global variable using an index.
- `check_bits_and_set_byte`: Checks bit 7 and bit 0 of a byte at offset 30 of a structure in a2, and if neither bit is set, writes 254 to offset 31 of the structure, then calls another routine, and if bit 5 is set, calls another routine before calling a third routine.
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
- `update_linked_list_node_2`: Writes the address of a pointer argument into the memory location pointed to by that argument, increments the value at that location by four, clears the following word, and stores the original pointer address into the word after that.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `invoke_table_entry_6`: Retrieves a function pointer from a global variable and calls it using parameters passed via the stack.
- `store_byte_at_pointer_3`: Stores a byte from register d0 into the memory location pointed to by register a3 and increments the pointer.

### printer.device

- `initialize_printer_hardware`: Calls a routine through a pointer at offset -84 of a6, and if the result is non-zero, stores it in a global variable and stores a value from a0 at offset 34 of the structure in a6.
- `return_with_constant`: Loads the constant 254 into d0 and calls another routine before returning.
- `manage_linked_list`: Sets a bit in a byte, performs a linked list insertion if a flag is set, and calls functions through pointers retrieved from a table.
- `execute_printer_initialization`: Calls a routine through a pointer at offset -84 of a6, and if the result is non-zero, stores it in a global variable, stores a value from a0 at offset 34 of the result, and calls another routine through a pointer at offset -432 of a6.
- `initialize_printer_buffers`: Calls a library function three times to check for non-zero values, and if successful, fills a memory range with a specific pattern, updates several structure offsets, and calls two more library functions.
- `call_library_function_multiple_times`: Calls a function pointer retrieved from a global variable three times, with the second, third, and fourth calls passing the value from register a6 as an argument.
- `execute_status_updates`: Calls three different routines in sequence and then returns.
- `return_with_constant_2`: Loads the constant 253 into d0 and calls another routine before returning.
- `set_status_bit_3`: Sets bit 0 of a byte at offset 61 of a structure in a6.
- `clear_status_bit`: Clears bit 0 of a byte at offset 61 of a structure in a6.
- `check_pointer_and_loop`: Loads a pointer from offset 48 of a6 into a1, checks if the pointer is null, and either branches to a routine via bsr or branches back to itself in a loop.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_previous`: Performs a short branch to a previous location in the code.
- `initialize_and_process_buffers`: Calls a library function to get a value, stores it in a structure, initializes two linked lists with specific offsets, and enters a loop that calls a library function to check bits and iterates through a linked list to modify bytes and call a library function.
- `clear_bit_on_comparison`: Compares d0 with the value at offset 190 of a structure in a6; if they are equal, it clears bit 1 of the byte at offset 2486 of that structure.
- `process_linked_list_and_status`: Enters a loop that iterates through a linked list of pointers, setting bit 5 of a byte at offset 30 of each element and calling a function through a table, then calls another routine to check a bit and potentially clears bit 0 or bit 1 of a byte at offset 2486 of a structure in a6 based on the returned value.
- `call_handler_with_arguments`: Loads a pointer from offset 4 of a7 into a0, loads a longword from offset 8 of a7 into d0, and calls another routine before returning.
- `update_status_and_store_data`: Calls a routine, and if it returns zero, modifies a byte at an offset of a pointer, then updates a structure with two 32-bit values and a 16-bit value before calling another routine twice.
- `adjust_stack_and_branch`: Adds 8 to the stack pointer a7 and branches to a previous location in the code.
- `set_status_bit_5`: Sets bit 1 of a byte at offset 2486 of a structure in a6, loads the address at offset 94 of a6 into a1, and branches to a previous location in the code.
- `check_status_and_return`: Loads a pointer from a global variable, reads a byte from offset 31 of that pointer, and if it is not zero, checks if bit 0 or bit 1 of the bitwise NOT of that byte is set, otherwise it returns.
- `poll_printer_status`: Enters a loop that calls three different routines through pointers in a global variable, checks a value at offset 14c of a6, and conditionally clears bit 0 or bit 1 of a byte at offset 9b6 of a6 based on the return value of another routine.
- `check_printer_status`: Loads a pointer from offset 94 of a6 into a1, compares a1 to d0, and if they are not equal, clears bit 1 of a byte at offset 950 of a6, calls another routine, and branches.
- `compare_and_call_routine`: Compares a value at an offset of a6 with a calculated address, and if they are not equal, calls another routine; otherwise it returns.
- `call_status_check_and_branch`: Calls another routine and then branches to a previous location in the code.
- `initialize_structure_fields_3`: Loads a pointer from a structure in a6 into a1, writes 9 to offset 28 of that structure, copies a 32-bit value from a pointer in a0 to offset 32 of the structure in a1, and clears offset 36 of the structure in a1.
- `clear_status_byte`: Sets a byte at a specific offset to zero and calls a routine through a pointer found in a structure.
- `check_status_byte`: Loads a pointer from a6 into a1, compares the byte at offset 8 of that structure to 7, and returns if they are not equal.
- `execute_callback_sequence`: Calls a routine through a pointer in a structure and then calls another routine using a value from a global base.
- `get_printer_mode`: Pushes a6 to the stack, loads a pointer from a table, and if a byte at offset 31 of that pointer is non-zero, it sets a6 to a value from a table and checks if bit 1 or 2 of a byte at offset 950 of a6 is set.
- `process_loop_logic`: Executes a loop that calls three different routines, and if a specific longword in a structure matches a register, it clears bit 0 of a byte and calls another routine; otherwise, it continues the loop or returns a value.
- `compare_and_clear_bit`: Compares a longword at offset 94 of a6 with d0, and if they are not equal, branches to another routine; otherwise, it clears bit 0 of a byte at offset 982 of a6, calls another routine, and branches to another routine.
- `compare_address_and_branch`: Compares a longword at an offset of 272 from a6 with d0, then branches to another routine if they are not equal, otherwise it calls another routine and branches based on the result in d0.
- `branch_to_status_check`: Calls a routine and then branches to another location.
- `save_registers_and_branch`: Saves d2, d3, a2, and a3 to the stack, compares a byte at offset 249 of a6 with 1, and if not equal, branches to a previous location; otherwise it pushes the current PC to the stack and branches to another location.
- `clear_status_bits_recursively_2`: Sets bit 15 of the INTENA register, modifies a byte at offset 358 of a global variable, calls another routine, and then recursively calls itself to clear bit 0 or bit 1 of a byte at offset 2486 of a structure in a6 if those bits are set.
- `clear_status_bits_recursively`: Calls a routine to check a condition; if the result is 0, it clears bit 0 of a byte at offset 2486 of a structure in a6 and calls itself if bit 0 was set, then clears bit 1 of that same byte and calls itself if bit 1 was set, finally returning 1 if the condition was 0 or 0 otherwise.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `update_list_and_copy_string`: Pushes two values from a1 onto the stack, calls another routine, pops a value into a1, calls another routine, and returns.
- `reset_structure_field`: Sets a 32-bit field in a structure at offset 20 of a1 to zero and conditionally updates it from the field at offset 24 of a1 based on the return value of another routine, then calls two other routines.
- `update_list_and_copy_string_2`: Pushes two values from a1 onto the stack, calls a routine through a pointer, restores a1 from the stack, and calls another routine before returning.
- `update_list_and_copy_string_3`: Sets a status bit in a byte and performs a linked list insertion if a flag is set, then calls another routine.
- `dispatch_pending_entry_8`: Checks a condition via a routine call; if met, it populates a structure at offset 768 of a global variable with 9, two parameters, and 0 at offset 302, then calls three routines through pointers in a structure and another routine using a global base.
- `call_buffer_search_routine`: Saves registers to the stack, loads a pointer from an offset of 8 in a structure in a6, and calls a routine at address 2358.l with a3 and a4 initialized to constants, then branches based on the result in d0.
- `search_and_reorder_buffer_data`: Calculates the length of a null-terminated string in a structure, then iterates through the buffer to perform nested searches for specific byte patterns.
- `set_printer_values`: Moves a long value from a pointer in a3 to a global variable, clears a 16-bit word in another global variable, and branches.
- `copy_and_process_printer_stream`: Copies bytes from a source buffer to a destination buffer using two incrementing pointers, then enters a loop that calls process_printer_control_stream to handle control characters and escape sequences based on a comparison with a specific byte value.
- `clear_registers_and_branch`: Sets registers d4, d6, and d2 to zero and branches to another location.
- `process_printer_data_buffer`: Iterates through a data buffer to find specific byte patterns, performing nested searches and copying bytes from a source structure into a global array.
- `copy_and_update_data`: Copies bytes from one memory location to another using two incrementing pointers in a loop, then writes a 32-bit value to offset 32 of a structure in a6 based on the value of d2.
- `clear_memory_offset`: Stores a zero at a specific offset within a memory location pointed to by a register.
- `decode_obfuscated_character`: Calls clear_and_branch_7, then enters a loop that calls decode_obfuscated_string_character, and finally calls clear_byte_at_pointer and call_and_negate_on_flag_3.
- `clear_and_branch_7`: Clears a longword at a negative offset from a6 and branches to another location.
- `decode_obfuscated_string`: Iterates through a loop to compare a character against a table, then performs a series of conditional transformations to derive a new character based on index values and a global variable.
- `set_status_byte_3`: Compares d0 to a global long value and compares d5 to the constant 5, and if both conditions are met, stores the byte ff at the address in a2 before branching.
- `compare_and_set_byte_2`: Compares the value 2 with a long value in a global variable and compares the value 12 with the value in d5; if both conditions are met, it writes the byte 75 to the address in a2 and branches to another routine, otherwise it branches to another routine.
- `check_and_set_byte`: Compares the long value in d5 to 9 and branches if they are not equal; if they are equal, it checks if the byte at the pointer in a4 is non-zero, and if so, it branches to another routine; otherwise, it stores the byte 59 at the pointer in a2 and branches to another routine.
- `copy_clamped_byte`: Compares d5 to 6, branches if d5 is greater than or equal to 6, otherwise copies a byte from a table at offset 362 using d5 as an index into the location pointed to by a2.
- `compare_and_set_byte`: Tests d5 and branches if non-zero, otherwise compares the byte at the pointer in a4 to 29; if the byte is 29 or greater, it writes the value 12 to the address in a2 and branches.
- `decode_obfuscated_string_character`: Iterates through a table to match a byte, then performs a series of conditional transformations and recursive calls to derive a new character value.
- `process_printer_control_stream`: Iterates through an input data stream to handle printer control characters, including escape sequences and line feeds.
- `parse_escape_sequences_and_control_codes`: Iterates through a source buffer to identify escape characters and control codes, translating them into specific byte sequences or lookup values.
- `parse_escape_sequence_and_buffer_data`: Iterates through a source buffer to identify escape characters and control codes, translating them into specific byte sequences or lookup values.
- `check_escape_character`: Compares a byte at the address in a3 to 91 and branches to another routine if they are not equal, otherwise it branches to a third location.
- `parse_printer_escape_sequences`: Parses a stream of bytes to handle escape sequences, including line feed processing and character expansion from a lookup table.
- `test_control_character`: Compares a byte at the address in a3 to 10 and, if equal, tests a byte at a global variable location, then conditionally writes 3 to a pointer's target and branches.
- `process_printer_stream`: Parses a stream of bytes by comparing them against a lookup table and calls other routines based on specific control characters.
- `write_two_bytes`: Writes the value 10 to the address calculated by adding d2 to a pointer stored at the address in a2, increments the pointer in a2, and then repeats this process to write the value 13 to the new address before branching.
- `write_single_byte`: Writes the value 10 to the address calculated by adding d2 to a pointer stored at the address in a2, increments the pointer in a2, and then branches.
- `check_character_byte`: Compares a byte at the address in a3 to 155, and if it is not equal, pushes d2 and d3 onto the stack, calls another routine, cleans the stack, and branches to another routine.
- `parse_file_path_string`: Processes a source string by checking for escape characters and directory separators to extract path components.
- `parse_escape_sequences`: Parses a string to identify and process escape sequences, such as converting numeric characters following a backslash into a single byte value.
- `update_buffer_pointer`: Increments a1 and a global long value, checks a byte at a1, and if it is zero, performs arithmetic on a value from a2 and compares it to 256, potentially updating a global long value before branching.
- `calculate_table_address`: Loads a byte from the address in a3 into d1, subtracts 160 from d1, shifts the result left by 2, adds the value in d4 to it, loads the address at the resulting location into a1, and branches to another routine.
- `skip_non_digit_characters`: Checks if the byte at the pointer in a1 is a backslash, or if it is a digit between 0 and 9, and branches to skip non-digit characters.
- `parse_numeric_string_and_process_tokens`: Parses up to three consecutive ASCII digits into an integer and stores it in a buffer, then enters a loop to process characters from a string by comparing them against a table of values and calling another routine recursively for specific control characters.
- `copy_byte_to_destination`: Loads a pointer from a2 into d1, increments the pointer in a2, sets a0 to the sum of d1 and d2, copies a byte from the pointer in a1 to the location in a0, increments a1, and then branches to another routine or a different part of the current routine based on the byte at the new pointer in a1.
- `parse_printer_escape_sequences_2`: Iterates through a data stream to identify and translate escape characters and control codes by comparing bytes against a lookup table.
- `update_global_values`: Subtracts the difference between two global variables from a third global variable and clears a fourth global variable.
- `transfer_buffer_data`: Copies a sequence of bytes from a local stack buffer to a destination pointed to by a parameter, incrementing a global index for each byte, provided the total count does not exceed 257; otherwise, it updates a global variable with the count and optionally calls another routine or modifies a global variable.
- `add_and_conditional_call`: Adds a 32-bit value from d3 to a 32-bit value from a global variable, compares the result to 256, and if the result is greater than 256, stores d3 in a global variable and conditionally calls another routine.
- `decrement_and_store_pointer`: Decrements a 32-bit value from a global variable by one, stores the result in another global variable, and branches to a different location.
- `clear_register_and_branch`: Sets register d4 to zero and branches to a different location.
- `copy_memory_block_2`: Copies a sequence of bytes from a source address to a destination address, incrementing a global pointer for each byte.
- `clear_byte_at_pointer`: Restores registers from the stack, loads three values from the stack into d3, d2, and a2, clears the byte at the address in a2, and branches to another routine.
- `clear_byte_at_offset`: Compares the byte in d1 to 59 and branches if they are not equal; if they are equal, it increments a3, adds the result to a2 to form a new pointer in a0, clears the byte at that pointer, and branches to another routine.
- `branch_on_space_and_set`: Compares a byte in d1 to 32, and if it is equal, loads a pointer from the stack into a0, sets d0 to 1, stores d0 at the location in a0, and branches to another routine.
- `branch_on_byte_comparison`: Compares a byte in d1 to 34, and if they are not equal, branches to another location; otherwise, it loads a pointer from offset 32 of the stack and sets d0 to 2 before branching to another location.
- `parse_input_string`: Iterates through a string to identify numeric characters, spaces, or specific delimiters, updating a pointer and a global variable while performing arithmetic on character values.
- `increment_and_lookup_table`: Increments a value at a pointer, uses the original value to index into a table, and calls another routine if a condition involving the pointer and a global variable is met.
- `return_register_value`: Returns the value currently held in a data register.
- `update_global_counter`: Loads a 32-bit value from a global variable, decrements it by one, stores the result in a global variable at offset 448, and branches to another location.
- `decrement_and_check_table`: Decrements a value pointed to by a register and continues looping as long as the value is positive and a specific bit pattern is found in a table.
- `initialize_and_check_globals`: Clears three global variables, sets a byte at offset 2720 of a global variable to zero, and sets another global variable to three.
- `copy_data_to_globals`: Copies six 16-bit values from a structure at offset 2662 of a global variable into six global variables.
- `compare_global_data`: Compares six 16-bit values from a structure at offset 2662 of a global variable against six global variables, returning 0 if all match and all bits set if any differ.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_conditional_logic`: Extracts four bytes from a structure in a parameter, calls another routine with a short value and a pointer, and then performs a series of conditional calls to other routines based on the return value and a short value at offset 32 of the parameter.
- `dispatch_conditional_routines`: Calls various versions of another routine with different parameters based on the state of registers and return values.
- `copy_formatted_string`: Calls a routine through a pointer, and if it returns zero, checks a parameter and either sets a global variable or copies a null-terminated string from a table to a destination buffer while handling specific character replacements.
- `update_printer_index`: Compares a word at offset 22 of the stack to 54 and 53; if the value is 53, it clears d1, moves the word from offset 22 of the stack to d1, performs a bitwise AND with 1, stores the result in a byte at offset 262, clears d1, and branches.
- `copy_processed_string`: Iterates through a source buffer, copying bytes to a destination buffer and updating an index, while checking for a termination value and handling specific character replacements.
- `compare_and_branch_d1`: Compares the byte in d1 to 255 and branches if they are not equal; otherwise it sets d1 to 255 and branches to another location.
- `copy_string_data`: Iterates through a source buffer, copying bytes to a destination buffer and updating an index, while checking for a termination value and handling specific character cases.
- `execute_jump_table`: Pushes d2 to the stack, loads a value from the stack into d2, moves a pointer from the stack into a0, copies d2 to d1, and if d1 is not less than 0, performs a jump to an address calculated using a table and d1.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `validate_and_clip_rectangle`: Extracts rectangle dimensions and coordinates from a structure, calculates the width and height by subtracting coordinate offsets, and calls calculate_clipping_rectangle to determine the valid drawing area.
- `calculate_clipping_rectangle`: Calculates the dimensions and boundaries of a rectangle by subtracting coordinate offsets and compares them against existing bounds.
- `update_bounding_box_and_state_2_2`: Calculates the width and height of a rectangle from two offsets in a structure pointed to by d5, compares these dimensions against existing bounds, and updates a bounding box structure in a6 with minimum values and scaling factors.
- `update_bounding_box_and_state_2`: Calculates the minimum values from two offsets in a structure pointed to by a pointer in d5 and stores them in a structure in a6.
- `update_bounding_box_and_state`: Finds the minimum values from two offsets in a structure pointed to by a pointer in d5 and stores them in a structure in a6.
- `compare_and_branch_5`: Compares the word in register d0 with 2048; if they are not equal, it branches to another routine, otherwise it branches to another routine.
- `load_constant_and_branch`: Loads the value 12 into register d1 and branches to another routine.
- `load_constant_and_branch_2`: Loads the value 15 into register d1 and branches to another routine.
- `update_bounds_and_recalculate_state`: Compares values from a pointer with local variables to find minimums, updates several global variables, and performs bitwise manipulations on a global status word.
- `update_status_and_calculate_bounds`: Compares a value from a pointer with a local variable to find a minimum, then updates a global variable and several other global values based on conditional checks.
- `modify_stack_offset_value`: Loads small constants into registers, stores a value at an offset of negative 4 from a pointer in a6, checks a byte in a global variable to branch, and if a second byte in a global variable is non-zero, performs arithmetic on registers and the value at the negative 4 offset from a6 before branching.
- `check_status_and_branch`: Clears a byte in a global variable, checks a byte in another global variable to branch, then checks a third global variable to branch again before clearing the stack and branching to another routine.
- `generate_color_lookup_table_3_2`: Calculates a color range based on two input values, determines the minimum and maximum bounds for two color components, and populates a 64-entry table with zeroed values.
- `generate_color_lookup_table_3`: Calculates a color range based on two input values, determines the minimum and maximum bounds for two color components, and populates a 64-entry table by decomposing color values into nibbles through a loop.
- `generate_color_lookup_table_2`: Calculates a color range from two values, clears a 64-entry table, and populates it by decomposing color values into nibbles.
- `generate_color_lookup_table`: Calculates a color range from two values, clears a 64-entry table, and populates it by decomposing color values into nibbles.
- `process_color_palette_data`: Calculates a range based on two values, clears a table of 64 entries, and populates it by decomposing color values into three nibble-based tables.
- `expand_and_invert_palette_2`: Decomposes 16-bit color values into three separate nibble tables, optionally performs a bitwise inversion and bit-shift on the data, and calls decompose_and_invert_palette for each color entry.
- `decompose_and_invert_palette`: Decomposes 16-bit color values into individual 4-bit nibbles stored in a table, then optionally performs a bitwise inversion and bit-shiftin and calls expand_and_invert_palette.
- `expand_and_invert_palette`: Decomposes 16-bit color values into individual 4-bit nibbles stored in a table, then optionally performs a bitwise inversion and bit-shifting on the values.
- `decompose_rgb_and_process_palette`: Decomposes a series of 16-bit color values into individual 4-bit nibbles stored in a table, optionally performs a bitwise transformation on a set of 32 values, and then iterates through a 64-entry table to update color data while conditionally applying a bitwise inversion based on a flag in a structure.
- `execute_function_and_store_result_2`: Sets three global variables, calls a routine via a pointer from a global variable, and then performs a series of conditional checks involving a global variable, a modulo operation against a 16-bit value from a structure, and further calls to the same pointer-based routine before returning.
- `execute_function_and_store_result`: Pushes three empty stack frames, calls a routine via a pointer from a global variable, stores the return value in a global variable, and branches to another routine or a specific offset based on the zero flag.
- `test_two_bytes_and_call`: Tests a byte in a global variable and, if zero, branches; if non-zero, it tests another byte in a global variable and, if zero, branches; otherwise it calls another routine and then branches.
- `test_and_call_printer`: Tests a byte in a global variable and, if non-zero, calls another routine and then branches to a different location; otherwise it branches.
- `test_and_call_printer_routine`: Tests a byte in a global variable and, if non-zero, calls another routine and then branches to a different location; otherwise, it branches to a different location.
- `call_and_test_global`: Calls another routine, stores the returned value in a global variable, and branches to one of two locations depending on whether that global variable is zero.
- `check_modulo_or_call`: Checks if a global value modulo a 16-bit value from a structure is 0 or if a routine call returns 0, then calls another routine with the result.
- `call_empty_routine`: Calls another routine with two zero arguments and returns.
- `calculate_weighted_value`: If a global variable is zero, it returns the input parameter unless bit 2 of a byte at offset 21 of a global variable is set, in which case it finds the minimum of three values.
- `find_minimum_byte`: Checks bit 2 of a byte at offset 21 of a global variable and, if set, finds the smallest byte value among three provided bytes and returns it, otherwise it returns the original second parameter.
- `process_printer_data`: Library code that performs complex calculations involving global variables and calls another routine to process data.
- `update_global_pointers`: Performs a series of conditional checks and calls a routine to update a pointer, then adds a global variable to four different global pointers.
- `update_pointer_table_with_offset`: Retrieves a value from a table using an index and stores it at a destination pointer, then updates four global pointers by adding a byte value to each.
- `store_and_branch_on_bit`: Stores a longword from a memory location into the address in a2, loads a pointer from a memory location into a0, tests bit 3 of a byte at offset 21 of a0, and then either performs a subtraction or branches to another location.
- `process_status_bits`: Moves two words into d1 and d0, shifts d0 right by 4 bits, and performs conditional branches based on the resulting value.
- `compare_printer_value`: Compares a word in d0 to 2 and branches if it is not equal, otherwise it branches to another location.
- `decrement_and_loop`: Decrements a 16-bit value in d0 and branches back to an earlier instruction if the result is not zero, otherwise it branches to another location.
- `store_byte_and_branch`: Stores a byte from d1 into a memory location and branches to another location.
- `store_byte_and_branch_offset`: Stores a byte from d1 into a different memory location and branches to another location.
- `transfer_pointer_and_call`: Stores a byte from d1 into a memory location, pushes a 32-bit value from a pointer in a2 onto the stack, calls another routine, stores d0 into the location pointed to by a2, increments the stack pointer by 4, stores the value from a2 into another memory location, and branches to another routine.
- `update_pointer_table`: Loads a 32-bit value from a table using an index, then updates four global pointers by adding a base address to each.
- `calculate_and_update_pointers`: Calculates a value based on the product or sum of two globals and updates four global variables by calling another routine multiple times.
- `query_printer_tables_2`: Sets a global variable to zero and then executes a series of conditional checks that call various routines and update a global variable based on their results.
- `query_printer_tables`: If a global variable is zero, it calls another routine three times with different parameters.
- `query_table_state`: Calls compare_table_or_variable with two different table indices, then calls compare_and_fetch_result to store a retrieved value into a memory location pointed to by a2.
- `compare_and_fetch_result`: Calls another routine, and if the result is non-zero, calls a third routine using parameters from a3 and a4, then stores the result in the location pointed to by a2.
- `restore_registers_and_return_10`: Clears d0 and restores registers a2 through a4 from the stack before returning.
- `increment_and_conditional_call`: Increments a global variable and, if a condition involving a modulo operation is met, calls a routine at an address stored in a memory location and stores the result in a global variable.
- `call_and_store_result_2`: Pushes a constant onto the stack, clears three words on the stack, calls a routine at an address stored in a memory location at offset 40, stores the returned value from d0 into the location pointed to by a2, and branches to another location based on the zero flag.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `load_addresses_and_branch`: Saves registers a2 through a4 to the stack, loads three specific addresses into a2, a4, and a3, copies a 16-bit word from a global variable to the address in a4, and branches to another routine.
- `process_printer_loop_logic_2`: Performs a nested loop structure that updates several global variables and calls another routine multiple times, eventually returning a value through a pointer.
- `process_nested_loops`: Performs a nested loop structure that calls another routine multiple times, updates several global variables, and conditionally returns a value.
- `clear_word_and_branch`: Clears a 16-bit word at offset 316 of a global variable and branches to another location.
- `process_printer_loop`: Performs a nested loop structure that calls another routine multiple times, updates several global variables, and conditionally returns a value from a pointer in a2.
- `process_printer_loop_logic`: A loop that increments a global variable, updates a value using a global variable and another value, and contains nested loops that call another routine multiple times based on values in a structure.
- `increment_and_check_limit`: Adds a 16-bit value from a structure in a3 to a 16-bit value in a structure in a4, then compares the result in a4 to 300; if the result is less than 300, it branches to another location, otherwise it clears d0 and returns.
- `clear_and_copy_word`: Saves registers a2 through a4 to the stack, loads three specific addresses into a2, a4, and a3, clears a 32-bit global variable, copies a 16-bit word from a global variable to the address in a4, and branches to another routine.
- `execute_printer_logic_loop_2`: Executes a loop structure that decrements a value in a loop and calls another routine, updating global variables and returning a value through a pointer.
- `clear_global_pointers`: Clears four long values at specific memory locations using zero and then branches to another routine.
- `execute_printer_logic`: Executes a loop that calls another routine to update a global variable, contains a for loop that calls another routine, and performs conditional checks on global variables and pointers.
- `call_and_store_result_3`: Calls another routine, stores the resulting longword from d0 into the address in a2, and then either branches to another location or loads the value from the address in a2 into d0 and branches to another location.
- `execute_printer_logic_loop`: A loop that decrements a global variable and contains nested loops and conditional branches that call other routines and update values in a structure or global variables.
- `increment_and_check_counter_2`: Adds a 16-bit value from a pointer in a3 to a 16-bit value at a pointer in a4, then compares the result against a constant in a global variable.
- `initialize_decrement_and_check_parameters`: Sets three pointers to specific memory locations, clears a 32-bit value at an offset, copies a 16-bit word from a global variable to a destination, and branches to another routine.
- `call_and_store_scaled_value`: Pushes two 16-bit values from a global variable onto the stack and calls a routine; it then stores the returned value from d0 into the location pointed to by a3 and branches to another location.
- `update_global_pointers_2`: Clears a word at a global address, copies a longword from one global address to another, copies a longword from a third global address to a fourth, copies a word from a fifth global address to a sixth, and then branches to another location.
- `update_state_with_decrement`: Executes a loop structure that calls another routine, updates a global variable, and decrements a value at a pointer's target before conditionally calling another routine.
- `update_printer_state_2`: Library code that increments a long global variable, compares a word from a global variable to d1, and if d1 is greater than the global variable, calls two other routines and moves d0 to a location in a2.
- `increment_global_and_call_routine`: Increments a global variable and calls another routine, then returns the value stored in a register.
- `decrement_and_check_limit`: Decrements a 16-bit value at the address in a3, and if the result is not higher than zero, it increments a 16-bit value at the address in a4, compares that value to 300, and if it is less than 300, branches to another location, otherwise it clears d0 and returns.
- `initialize_printer_status_and_counters`: Clears two 16-bit values at offsets 380 and 340 of a data structure, copies a 16-bit value from offset 380 of a structure to offset 320 of another, and branches to another routine.
- `push_parameters_and_call_accumulate`: Pushes two 16-bit values from global variables onto the stack and calls accumulate_scaled_values, then stores the result in the memory location pointed to by a register and restores the stack pointer.
- `clear_and_copy_memory_pointers`: Clears a word at a global address, copies a long value from one memory location to another, copies a long value from another location to a third, copies a word to a fourth, and branches to another routine.
- `process_printer_buffer`: Clears four global variables, enters a loop that calls another routine, and performs nested checks involving a global variable and a pointer in a3.
- `store_result_and_branch_2`: Calls another routine, stores the returned value into a memory location pointed to by a register, and then either branches to another routine or performs a conditional check to move the stored value into a register before branching elsewhere.
- `execute_do_while_loop`: Executes a do-while loop that increments a global variable, decrements another, and contains nested loops that call another routine and update a pointer's target.
- `decrement_and_check_counter_2`: Decrements a 16-bit value at the address pointed to by a3 and checks if it is greater than zero; if so, it increments a 16-bit value at the address pointed to by a4, compares the new value against a constant, and either returns or clears d0 before restoring registers and returning.
- `accumulate_scaled_values`: Iteratively adds a doubled value to a pointer's target until the target is no longer less than one, then subtracts a doubled second parameter from that target.
- `compare_table_or_variable`: If a global variable is zero, it compares a parameter against a byte retrieved from a table using indices derived from two other parameters.
- `compare_table_value`: Performs a comparison between a byte retrieved from a table using bitwise-derived indices and a provided byte.
- `negate_boolean_results`: Calls two different routines and returns the negation of the first return value based on the status of a second return value.
- `process_large_value_rotation`: If a parameter is greater than 65535, it performs a bitwise rotation and subtraction loop sixteen times; otherwise, it performs a division-based calculation using the parameter as a modulus.
- `rotate_32bit_value_2`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_7`: Computes the 32-bit product of two 32-bit integers by calculating partial products of their 16-bit components and returning the result.
- `forward_call_to_helper_2`: Calls one helper and returns, with no other work.
- `call_helper_return_register_value_2`: Pushes d2 to the stack, moves d1 to d2 and d0 to d1, calls another routine, moves the resulting d1 back to d0, restores d2 from the stack, and returns.
- `negate_result_on_flag_4`: Calls process_large_value_rotation and negates the returned value if the flag in an address register is nonzero.
- `call_and_negate_on_flag_3`: Calls a routine and returns its result from a data register, or returns the negation of that result if a second result is nonzero in an address register.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `process_pointer_chain`: Calls setup_pointer_chain with two arguments, then calls fetch_table_address to retrieve an address into d4, and finally calls branch_to_offset_334.
- `fetch_table_address`: Shifts a word in d0 left by two bits, uses the result to index into a table via a2, and moves the resulting address into d4 before branching.
- `branch_to_offset_334`: Performs a short branch to an instruction located 334 bytes ahead in the code.
- `set_address_and_branch`: Loads the value 32 into a2 and branches to another routine.
- `process_disk_commands`: Iterates through a string by calling another routine to compare characters, and if a null terminator is not reached, checks a pointer at offset 86 of a structure, calls another routine, and performs several operations including a loop that copies characters to a global variable and calls other routines.
- `execute_disk_commands`: Checks a pointer at offset 56 of a structure, calls a routine, and if a subsequent routine returns a non-zero value, performs file system operations including locking, loading a segment, and copying a string from a buffer.
- `unlock_and_get_directory`: Calls a routine to unlock a resource and then calls another routine to retrieve the current directory, returning negative one in register d0.
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

- `initialize_device_structure`: Initializes several fields within structures returned by two different function calls, including setting pointers and values at various offsets, and calls two other routines before returning a value of negative one or zero.
- `test_and_negate_d0`: Tests the long value in d0; if it is zero, it branches to another routine, otherwise it negates d0 and branches to another routine.
- `manage_audio_device_state`: Checks a status flag in a structure; if zero, it attempts to initialize the audio device by calling two functions and updating state fields, otherwise it increments counters and populates a data structure with specific values and a pointer.
- `update_device_state`: Sets a byte at offset 176 of a structure to 5, stores a pointer to a global variable at offset 178, and copies a longword from offset 244 to offset 240 of a structure.
- `update_device_state_fields`: The routine stores an input value into a structure, increments counters in the structure, copies a counter to another structure, writes several constant words and bytes to the second structure, and clears a flag bit in the first structure.
- `update_device_state_fields_2`: Stores a byte from d0 into a structure at offset 31 of a2, writes a long value of negative one to offset 20 of a2, and branches to another routine.
- `update_status_and_call_handler`: Clears bit 0 of a byte at offset 30 of a structure, checks bit 6 of a byte at offset 34 of a secondary structure, and if certain conditions are met, writes a byte to offset 31 of the primary structure and calls another routine.
- `manage_interrupt_queue_2`: Checks if a value at offset 24 of a structure matches a value in a register, then either copies a byte from a structure to another and splits it into two nibbles while calling another routine, or performs a linked list insertion into a structure and updates a counter and a global variable.
- `manage_interrupt_queue`: If certain conditions are met, it copies a byte from one structure to another and splits it into low and high nibbles into two bytes; otherwise, it updates a linked list and modifies the INTENA register.
- `copy_byte_and_split_nibbles`: The routine copies a byte from one structure to another, splits it into low and high nibbles into two bytes, clears a byte in the destination structure, and calls another routine.
- `process_linked_list_and_update_state`: Traverses a linked list to find a specific node, decrements a counter in a global structure, and if the counter reaches zero, disables interrupts via the INTENA register.
- `decrement_and_disable_interrupts`: Decrements a byte in a global structure and, if the result is negative, disables interrupts by writing to the hardware register; otherwise, it returns.
- `traverse_linked_list_3`: Traverses a linked list starting from a pointer at offset 54 of a structure, calling another routine if a 32-bit value in the list matches a value from a structure, then decrements a 16-bit counter at offset 32 of a structure and, if it reaches zero, calls two routines via pointers stored in a global variable.
- `decrement_counter_and_close`: Decrements a 16-bit counter in a structure and, when it reaches zero, calls two routines via pointers stored in a global variable and optionally calls another routine if a bit is set in a structure.
- `dispatch_handler_by_index`: Loads a word from a structure, checks if it is between zero and eight, and if so, calls a routine from a table using the value as an index.
- `set_status_bit_4`: Sets bit 5 of a byte at offset 34 of a structure pointed to by a0, which is derived from a pointer in a3, then returns.
- `swap_pointers_and_return_value`: The routine calls library functions via a base pointer, swaps values between a pointer table and a data word, updates a pointer field, and returns a value from a table entry.
- `process_narrator_event_request`: Evaluates a request structure based on a type field and executes various actions including setting status flags, updating a linked list of request structures, and calling specific handler functions via pointers in a global structure.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `modify_structure_flags`: Clears bit 5 of a byte at offset 34 of a structure in a0, writes 7 to a word at offset 196 of a structure in a6, sets bit 0 of a byte at offset 198 of that same structure, calls two routines via pointers in a structure in a6, and sets a byte at offset 31 of a structure in a1 to 0.
- `set_flags_and_invoke_handler`: The routine sets a flag bit in a structure, writes a word and sets a bit in a global structure, calls a handler via a base pointer, clears a byte in another structure, and calls another routine.
- `set_flag_then_clear`: The routine sets bit 7 in a byte at offset 34 of a structure, calls two other routines, then clears bit 7 in the same byte.
- `set_flag_clear_byte_and_call`: Sets bit 7 in a byte at offset 34 of a structure, calls another routine, clears bit 7, clears a byte at offset 31 of a second structure, and calls another routine.
- `modify_interrupt_state`: Sets a global variable to 65536, increments a byte at offset 294 of that global variable, calls a routine via a pointer in a linked list, and then decrements the byte at offset 294 of the global variable, setting the global variable to 49152 if the byte was less than one.
- `decrement_counter_and_set_global`: The routine decrements a byte in a global variable area and, if the previous value was less than one, writes a constant to a global variable.
- `enable_interrupt_and_set_bit`: Saves registers, updates a global pointer, enables interrupts by writing to the INTENA register, increments a byte at offset 182 of a structure, and sets bit 5 of a byte at offset 30 of a structure if a comparison succeeds.
- `traverse_linked_list`: Loads a pointer from a structure in a0 into a0, loads a longword from that new address into d0, and if d0 is not zero, compares a0 with a1 and updates a0 to the value in d0 if they are not equal.
- `update_pointers_and_decrement_counter`: The routine updates two linked pointers, sets a byte at an offset in a structure, decrements a byte in a global variable, and if the value is less than one, writes a constant to a hardware register and calls another routine.
- `decrement_counter_and_set_global_2`: The routine decrements a byte in a structure pointed to by a register and, if the previous value was less than one, writes a constant to a hardware register.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `set_byte_and_call_handler`: The routine stores the byte value 249 at an offset in a structure and calls another routine.
- `clear_byte_and_call_handler`: The routine clears a byte at an offset in a structure and calls another routine.
- `clear_flags_and_call_handler`: The routine clears a flag bit in a structure, clears a byte in another structure, and calls a handler via a base pointer.
- `update_speech_buffer_parameters`: Validates several numeric fields from an input structure and, if they fall within specific ranges, copies them into a target structure.
- `validate_and_store_word`: Compares a 16-bit value in d1 to 320; if it is not greater than 320, it stores the value into offset 28 of the structure in a5.
- `compare_and_store_word_2`: Compares a 16-bit value in d1 to 320; if it is greater than 320, it branches, otherwise it stores d1 into offset 28 of a structure in a5, then compares a 16-bit value at offset 52 of a structure in a2 to 0 or 1 and branches accordingly, otherwise it loads 232 into d3 and branches.
- `store_and_branch_on_word`: Stores a 16-bit value from register d1 into offset 48 of a structure pointed to by a5, then compares a 16-bit value at the address in a2 against 0 and 1, branching to another routine if either condition is met, otherwise it loads 233 into d3 and branches to another routine.
- `update_and_compare_word`: Stores a 16-bit value from register d1 into an offset of 38 in a structure pointed to by a5, then compares a 16-bit value from an offset of 64 in a2 to 5000; if the value is greater than or equal to 5000, it branches to another routine, otherwise it loads a constant into d3 and branches to another routine.
- `compare_and_store_word`: Compares a 16-bit value in d1 to 26464; if it is greater, it branches to another routine, otherwise it stores d1 at offset 2168 of the structure in a5.
- `scan_string_for_delimiters`: Scans a string backwards from a specified offset to find a comma, a hyphen, or a space.
- `scan_text_for_delimiters_2`: Scans a string backwards from a specified offset to find a comma, a hyphen, or a space.
- `copy_longword`: Copies a longword from offset 36 of a structure in a1 to offset 32 of the same structure, then branches to another routine.
- `clear_word_and_set_byte`: The routine clears a longword at an offset in a structure, stores a byte from a register into another offset in that structure, and calls another routine.
- `scan_text_for_delimiters`: Scans a string backwards from a specified offset to find a comma, a hyphen, or a space.
- `process_word_offset`: Loads a longword from offset 150 of a structure, performs arithmetic involving a bit-shifted version of a word from that longword, and either branches to another routine or updates a word at offset 156 and clears a longword at offset 150.
- `shift_and_clear_word`: Loads a word from offset 150 of a structure in a5 into d0, shifts it right by one, stores the result in offset 156 of the structure, clears the longword at offset 150, and branches to another routine.
- `initialize_speech_buffer`: Clears several fields in two structures, performs two function calls, and if a condition is met, allocates a new structure and populates it with data from a global variable and a table.
- `transfer_speech_parameters`: Loads a longword from offset 24 of a structure in a1 into d1, stores the byte from d1 into offset 67 of a structure in a2, sets d0 to 34 and d1 to 65537, then calls a routine at an offset from a6; if d0 is not zero, it branches to another location, otherwise it sets d3 to 254 and branches to another routine.
- `copy_speech_data`: Initializes a structure with a value, a pointer, and a constant, then calls two routines to retrieve values.
- `call_library_function_2`: Stores a byte from d0 into offset 68 of a structure in a2, calculates a value in d0 by doubling it and multiplying by 68, calls a library function using a pointer in a6, and if the result in d0 is zero, sets d0 to 254 before branching to another routine.
- `copy_word_table_to_buffer`: Stores a pointer in a global variable and copies 34 words from a table in a global variable to a buffer pointed to by an argument, writing each word twice per iteration, repeating for a count derived from a byte in a global variable.
- `modify_register_bits`: Sets bit 1 of d1, then checks bit 0 of d0, performs bitwise AND operations on d3, and checks bit 1 of d1.
- `handle_io_and_free_resources`: Writes a byte and a word to a global variable, calls a routine via a global variable, conditionally calls two other routines via the same global variable, clears a word in the global variable and returns a combined value.
- `check_signed_byte`: Tests a word at offset 206 of a structure in a5 and, if it is greater than zero, clears register d3 before returning.
- `parse_narrator_control_string`: Parses a string in a0 to extract control codes and metadata, updating a structure at offset 232 in a5 with character values and status flags while managing a 5-element buffer.
- `modify_and_branch`: Negates a word in d1, adds 111 to it, and performs conditional branches based on a comparison of d0 with 12288; if the comparison is not less than that value and not equal, it branches, otherwise it adds d2 to a0 and branches.
- `parse_narrator_input_stream`: Parses a stream of input bytes by looking up control codes in a table, updating a buffer with the input values, and managing segment status.
- `process_narrator_input_stream`: Parses a stream of input bytes by looking up control codes in a table, updating a buffer with the input values, and managing segment status bits.
- `set_status_bit_five`: Compares a byte in d1 to 5; if it is not equal, it branches to another routine, otherwise it sets bit 5 of a byte at offset d3 of the structure in a6 and branches to another routine.
- `set_status_bit_6`: Compares a byte in d1 to 6; if it is not equal, it sets bit 2 of a byte at offset -1 of the structure pointed to by a6 and d3, then jumps to another routine.
- `update_buffer_index_and_validate`: Stores a byte from d1 into a table at the offset specified by d3 and increments d3, then checks if d1 is one or two; if d1 is neither and d3 is less than five hundred twelve, it subtracts a longword from a global variable at offset one hundred forty-six of the structure in a5, stores the result at offset one hundred fifty in the same structure, and sets the carry flag before returning.
- `clear_buffer_segments`: Subtracts a longword from offset 146 of a structure in a5 from a1, compares a1 to a word at offset 156 of the structure in a5, and if a1 is greater than or equal to that word, it branches to another routine; otherwise it clears a byte at offset d3 from a structure in a3 and a byte at offset d3 from a structure in a6, then branches to another routine.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends immediately.
- `decrement_index_on_match`: Loads a byte from a structure using an index in a register, and if the byte is not zero, decrements that index and branches back to the start of the routine.
- `update_data_entries`: Checks a bit in a longword retrieved from a table using an index in a4, and if the bit is not set and a counter in a3 is less than 4, writes 4, 0, and 0 to three consecutive bytes at an offset determined by the counter in three different structures.
- `store_word_to_offset`: Stores a word from d3 into a structure at offset 154 of a6 and sets the condition codes.
- `update_status_from_string`: Iterates through a null-terminated string, using bytes from the string as indices into a table of longwords to check specific bits, and sets bit 4 of various bytes in the string based on the results of these checks.
- `set_status_byte_2`: Performs a bitwise AND on d2 with a constant, and if the result is non-zero, sets bit 4 of a byte at offset -3 of a structure in a0.
- `conditional_set_status_byte`: Loads a longword from a table using a pointer in a2 and an index in d1, masks it, and if the result is non-zero and a byte at offset -2 of a structure in a1 is 48, sets bit 4 of a byte at offset -3 of a structure in a0.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `apply_text_formatting_attributes`: Parses a source buffer byte by byte, using a lookup table to determine formatting flags and applying bitwise transformations to a destination buffer.
- `process_narrator_text_stream`: Iterates through a source buffer, performing bitwise transformations on a destination buffer based on control characters and lookup table values.
- `process_text_stream_formatting`: Parses a source buffer byte by byte, using a lookup table to determine formatting flags for a destination buffer.
- `calculate_offset_and_branch`: Subtracts a word in d2 from a word in d0, decrements the result by 1, and branches to another location.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `scan_pattern_and_update_buffer`: Iterates through a buffer to find a sequence of bytes matching a pattern, where certain bytes are optional or required.
- `match_and_skip_pattern_bytes`: Compares bytes from a source buffer against a target buffer using a pattern of optional and required values, skipping ahead in the source buffer based on a calculated offset if a mismatch occurs.
- `increment_and_test_byte`: Increments a word in d1, loads a byte from offset 6 of a structure in a0, and branches if the byte is 255; otherwise it calls another routine and returns if the carry flag is not set.
- `test_bit_and_branch`: Tests bit 5 of a byte at offset 3 of the structure in a0 and branches to another routine or a different location based on the result.
- `test_bit_with_table_lookup`: Loops reading a byte from a buffer and a word from a table indexed by a word, modifies the word based on bits in the byte, and exits when a specific bit is set or a termination condition is met.
- `compare_word_and_branch`: Loads a word from offset 154 of a structure in a5 into d7, compares it to 512, and branches to another routine or sets the carry flag and branches if the value is less than 512.
- `shift_bytes_and_clear_fields`: Increments a word counter in a structure and shifts bytes backward in three buffers by a computed count, then writes a byte from a register and two zero bytes to the new ends of the buffers.
- `execute_multiple_subroutines`: Calls seven different subroutines in sequence and then clears register d0 before returning.
- `clear_byte_if_not_set`: Loads a pointer from offset 40 of a structure in a5 into a6, compares the byte at that pointer to 255, and if they are not equal, loads the pointer from offset 40 of a5 into a6 again and writes 160 to offset 7 of that structure before returning.
- `clear_byte_and_advance`: Clears a byte at offset 7 of a structure in a6, increments a6 by 8, and branches to another routine.
- `initialize_data_pointers`: Loads three pointers from offsets 232, 1256, and 744 of a structure in a5 into registers a0, a1, and a2, loads a value from a global variable into a4, and returns if the byte at a0 is not 255.
- `process_encoded_data_3`: Performs complex bitwise manipulations and table lookups to decode data from three source pointers into a destination, calling another routine to increment the pointers and another routine to process encoded data.
- `process_encoded_data_2`: Processes a stream of bytes by reading from a source buffer, performing bitwise operations and table lookups to update a destination buffer.
- `process_encoded_data`: Processes a stream of bytes by reading from a source buffer, performing bitwise operations and table lookups to update a destination buffer and a structure in a2, and continues in a loop until a byte containing 255 is encountered.
- `test_bit_and_set_byte`: Tests bit 5 of d1 and branches if it is clear; otherwise it sets bit 2 of a byte at offset 0 of a structure in a2 and branches to another routine.
- `set_bit_and_branch_2`: Tests bit 5 of d1 and branches if it is clear; otherwise it sets bit 2 of a byte at offset 0 of a structure in a2 and branches to another location.
- `set_bit_and_advance`: Sets bit 2 of a byte at offset 0 of the structure in a2, increments a2 by 1, and branches to another routine.
- `increment_three_pointers`: Increments three pointers in a0, a1, and a2 by 1 and branches to another routine.
- `calculate_masked_value`: Loads a byte from offset 488 of a structure in a5 into d2, compares it to 255, and if not equal, masks d2 with 63 and adds the result to d0 before branching to another routine.
- `allocate_memory_and_update_pointer`: Stores a longword from d0 into offset 58 of a structure in a5, shifts d0 left by 3, adds 8 to d0, stores the result in offset 54 of the structure in a5, calls a routine using a pointer from offset 4 of a global variable, and if d0 is not zero, sets the two most significant bits of the status register and increments a7 by 4 before returning.
- `update_structure_from_call`: Stores d0 into offset 40 of a structure in a5, checks bit 217 of a structure in a5, and if set, loads d0 from offset 58 of the structure, calls another routine, and if d0 is non-zero, stores d0 into offset 44 of the structure before returning.
- `expand_narrator_stream_records`: Iterates through a byte stream, using each byte as an index into a global table to extract structured data blocks.
- `expand_narrator_stream_data`: Iterates through a stream of bytes to extract structured data from a global table and writes the resulting expanded records into a destination buffer.
- `parse_narrator_stream`: Reads a stream of data from a source buffer, using bytes to index into a global table to retrieve structured data including three values and a control byte.
- `interpolate_buffer_data_3`: Iterates through a buffer and performs linear interpolation on bytes using values from a global table, calling another routine to check buffer status during the loop.
- `check_buffer_status`: Shifts d0 left by 2, loads a longword from a table using the result as an offset, and if bit 21 and bit 11 of that longword are both set, increments a0 and branches to another routine; otherwise it branches to another routine.
- `interpolate_buffer_data_2`: Performs a weighted interpolation of bytes between two values in a buffer and uses a loop to process data based on values read from a table.
- `interpolate_buffer_data`: Performs a weighted interpolation of bytes between two values in a buffer and uses a loop to process data based on values read from a table and a counter.
- `increment_a6_and_branch`: Increments the value in a6 by 1 and branches to another routine.
- `process_narrator_data_stream_3`: Iterates through a data stream to perform comparisons between values retrieved from a table and updates a destination buffer with calculated values.
- `iterate_data_stream`: Loads a byte from a table using a2 and d7, increments a pointer in a1, masks d0, shifts d7, loads a longword from a table, and performs conditional branches based on bits 10 and 11 of that longword before looping back.
- `process_narrator_data_table`: Iterates through a data structure, performing comparisons between values retrieved from tables and registers, and updates a series of memory structures by zeroing bytes and setting specific word values based on the comparison results.
- `compare_table_values`: Loads a byte from a0 into d7, loads a byte from a table at a4 plus the offset in d7 into d2, and compares d5 to d6, branching if d5 is greater than or equal to d6; otherwise, it loads a byte from a0 into d7, loads a byte from the table into d3, and branches to another location.
- `update_narrator_table_entries`: Iterates through a data stream to perform arithmetic comparisons between values in a table and a secondary data source, then updates a structured table in memory by clearing specific fields and writing new values based on the comparison results.
- `process_narrator_data_stream_2`: Iterates through a data stream to perform arithmetic comparisons and updates a structured table in memory.
- `calculate_difference`: Subtracts the values in d3 and d2 from the byte in d0, stores the result in d4, and then moves the byte from d2 into d1 before branching to another routine.
- `increment_pointer_and_loop_2`: Clears a word at the address in a6 and a byte at offset 2 of that address, then checks bit 31 of d0; if bit 31 is not set, it stores d6 at offsets 3 and 4 of the address in a6, increments a6 by 8, and iterates a loop using d1 as a counter before branching to another location.
- `increment_pointer_and_loop`: Adds 8 to a pointer in a6, then initializes a loop counter with the value in d3 and iterates using a dbra instruction.
- `clear_and_copy_data`: Clears a 16-bit word at the address in a6 and a byte at offset 2 of that address, then checks bit 31 of d5 to conditionally move the byte in d6 to offset 3 and the word in d6 to offset 4 of the structure in a6, increments a6 by 8, and enters a loop using d1 that increments a0 and branches to another routine.
- `clear_registers_and_branch_2`: Clears d1, d3, and d4 by setting them to zero, moves the byte from d0 into d1, and then branches to another routine.
- `copy_filtered_buffer_data`: Loops through a buffer, checking a bitmask in a global table for each byte, and copies bytes and words from three source pointers to three destination pointers if the bitmask condition is met.
- `write_byte_to_three_pointers`: Writes the same byte from a register to the memory locations pointed to by three registers.
- `call_subroutines`: Calls two consecutive subroutines and then returns.
- `mask_buffer_byte`: Compares a byte at offset 616 of a structure to 255; if they are equal, it branches to another routine, otherwise it performs a bitwise AND on that byte with 240 and branches to another routine.
- `check_buffer_byte`: Loads a byte from an offset of 232 in a structure pointed to by a5 into d0, compares it to 255, and returns if the value is not 255; otherwise it branches to another routine.
- `update_buffer_flags`: Iterates through a buffer, performing complex bitwise updates to bytes based on values retrieved from a table, and continues in a loop until a byte containing 255 is encountered.
- `update_status_flags_2`: Sets bit 0 of a byte at offset -1 from the pointer in a1, checks bit 7 of d1, and if bit 7 is set, it compares a byte from a0 to 24 or 27; if the byte is neither 24 nor 27, it sets bit 1 of a byte at the address in a1 and moves a1 to a3.
- `update_status_flags`: Sets bit 1 of a byte at an offset from a pointer in a1, then enters a loop that checks values in a table and updates bits in a buffer based on specific bitwise conditions and table lookups, eventually calling another routine.
- `set_bit_and_branch`: Sets bit 2 of a byte at the address in a3 and branches to another routine.
- `set_multiple_bits`: Sets bit 0 of a byte at offset -2 of the address in a1, and sets bits 1 and 2 of a byte at offset -1 of the address in a1, then branches to another routine.
- `interpolate_scaled_buffer_values_2`: Calculates a scaling factor from a global value and a word in a structure, then iterates through a control table to write scaled bytes into a destination buffer.
- `calculate_scaled_offset`: Masks d0 with 63, tests bit 0 of a byte at the address in a2 and increments a2, then if the bit was set, shifts d0 left by 3 and adds it to a6 before branching to another routine; otherwise it continues to the next instruction.
- `interpolate_scaled_buffer_values`: Iterates through a data stream to perform linear interpolation between values, calculating scaled results based on bit-masked offsets and writing them to a destination buffer.
- `interpolate_and_write_scaled_values_2`: Iterates through a loop where it masks a byte from a1, checks bit 1 of a byte at the address in a2, and if bit 1 is set, it shifts the masked byte left by 3 and adds it to d1 before repeating.
- `interpolate_and_write_scaled_values`: Iterates through a data stream to calculate scaled values by dividing a longword from a structure by various divisors derived from the stream, then writes these bytes to a destination buffer using bit-manipulated offsets.
- `perform_division_and_branch`: Swaps the high and low words of d5, adds the word in d6 to d5, divides the word in d5 into the longword at offset 32 of the structure in a5, stores the resulting byte in d7 at offset -1 of the structure in a6, decrements a1 and a2, and branches to another routine.
- `divide_and_store_result`: Loads a pointer and a word from a structure, divides the word by 110, increments the pointer, and either stores the result at the pointer or branches.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `parse_narrator_control_stream`: Iterates through three parallel data streams, using values from one stream as indices into a table to fetch control words.
- `fetch_indexed_data`: Shifts register d5 left by two, loads a longword from a table using d5 as an offset from a4 into d3, and then performs conditional branches based on bits 20 and 25 of d3.
- `increment_three_pointers_2`: Increments the values in a0, a1, and a3 by 1 and branches to another routine.
- `decode_narrator_text_stream_character`: Performs a multi-stage conditional lookup based on bits in registers and values in a data stream to select a specific transformation or scaling constant from a table.
- `test_bits_and_call`: Tests bit 12 of d7 and branches if it is clear; if bit 12 is set, it tests bit 9 of d7 and branches if it is clear; otherwise, it loads 38 into d1, calls another routine, and branches.
- `test_bit_and_branch_3`: Tests bit 10 of d7 and branches if it is set; otherwise, it loads 38 into d1, calls another routine, and branches to another location.
- `test_bits_and_branch`: Tests bit 4 of d7 and branches if it is set; otherwise, it tests bit 2 of the byte at offset 1 of a3 and branches if it is set; if not, it loads 27 into d1, calls a subroutine, and branches to another location.
- `test_bit_and_call_routine`: Tests bit 11 of a register and, if not set, loads the value 22 into a register, calls another routine, and then branches to a different location; otherwise, it branches to a different location.
- `decode_narrator_text_stream_3`: Decodes a stream of bytes by performing lookups in tables to calculate new character values, applying scaling and bitwise transformations, and writing the results to a destination buffer in a1.
- `test_and_branch_flags`: Tests bit 1 of d6 and bit 1 of d7, branching to different locations if either bit is clear, otherwise it sets d1 to 16, calls another routine, and branches to another location.
- `decode_narrator_text_stream_2`: Sets d1 to 22 and calls another routine before branching to another location.
- `decode_narrator_text_stream`: Reads a stream of encoded bytes from a source pointed to by a0, performs bitwise transformations and arithmetic based on lookups in a table, and writes the resulting characters to a destination pointed to by a1.
- `compute_scaled_shifted_product`: The routine multiplies the low word of an input register by a word input and returns a combined value formed from the high word of the product and the low word shifted right by five bits after adding a constant.
- `initialize_and_clear_buffers`: The routine stores several offsets relative to a base pointer into a global structure, increments a few of those pointers, clears a set of word fields, then zeroes six blocks of memory by iterating a fixed count.
- `increment_and_call_handlers`: Increments a 16-bit word at offset 136 of a structure in a5, then calls five subroutines based on the zero and overflow flags, finally clearing the condition code register and returning.
- `process_compressed_data_2`: Processes a data stream by checking a byte at a pointer in a8, performing arithmetic on values from a table, and writing transformed bytes to a destination buffer while updating counters and checking for an end marker.
- `process_compressed_data`: Tests a byte in d1 and branches; otherwise it checks bit 7 of a byte at a negative offset from a pointer in a4 plus d4, then checks bit 5 of d1.
- `decode_compressed_stream`: Decodes a stream of data by performing bitwise shifts and masking to expand values, writing the resulting bytes to a destination buffer while updating several counters and status flags in a structure.
- `set_flag_and_store_indices`: The routine ORs a constant into a byte addressed by a base pointer and an index, then stores two short values derived from an index and a conditional copy into fields of a global structure.
- `process_text_stream_and_apply_formatting`: Iterates through three synchronized buffers to scan for an end marker, applying bitwise flags to a target buffer based on control bytes and character values.
- `process_indexed_byte`: Loads a word from a table using an index derived from d3, checks bit 19 of that word, and if set, branches; otherwise it checks if d1 is negative, increments d4 if it is positive, checks bit 5 of d2, and either writes the value 2 to a byte at an offset from a6 or branches to another location.
- `modify_byte_via_flag`: Checks bit 4 of a byte at an offset from a pointer in a6, and if bit 4 is not set, it performs bitwise operations and updates the byte at that same offset before branching to another routine.
- `decrement_and_store_pointers`: Decrements three pointers in a0, a1, and a2 by 1, stores these updated pointers into a structure at offsets 124, 132, and 128 of a5, sets d7 to 0, moves the word in d4 to d0, and branches to another location.
- `update_status_byte_2`: Performs a bitwise OR of d7 into a byte at an offset from a4, moves d0 to d2, and compares a byte at an offset from a6 with 2 or 14.
- `clear_byte_on_condition`: Checks bit 5 of a byte at an offset from a pointer in a4, and if that bit is not set, it compares a word in d2 with d4 and clears a byte at an offset from a6 if d2 is less than d4.
- `swap_and_clear_byte_flags`: Swaps a byte between two positions in a table addressed by a base pointer in a6 and two indices in d0 and d2; if the indices are not equal, it clears the byte at the position indicated by d0 and iterates a loop using d0 as a counter.
- `prepare_set_bits_range`: Loads a pointer from an offset of a5 into a1, moves a word from d4 into d0, and branches to another routine.
- `set_bits_in_loop`: Loops through a range of memory defined by a pointer in a1 and a count in d0, setting bit 0 of each byte.
- `or_mask_into_buffer_with_early_exit`: The routine selects a mask based on a control byte, then ORs that mask into a series of bytes in one buffer while checking a flag in another buffer and returning early if the flag is set.
- `set_tile_and_mask`: The routine updates a byte in a screen buffer using a mask derived from a map byte, and optionally masks and sets a flag in another byte based on a scan leftwards in a second buffer.
- `accumulate_table_entries`: Reads a word from offset 140 of a structure in a5, checks if the short value at offset 48 is zero, and if so, conditionally calls another routine four times based on a short value at offset 138; it then adds the word to eight consecutive longwords in a table pointed to by offset 92 of the structure in a5.
- `clamp_and_store_value`: The routine calculates a value based on several inputs, clamps it between 125 and 165, and stores the resulting byte at an offset from a pointer in a structure.
- `clamp_value_and_store_char`: The routine clamps a short input to a lower and upper bound and stores the resulting byte at an address formed from a base pointer and an index.
- `adjust_table_values_loop_2`: Calculates a scaled difference between values in two tables and applies it to a third table through three sequential loops.
- `adjust_table_values_loop`: The routine repeatedly computes an absolute difference between two table entries, scales it and adds the result to a destination byte, then scans backwards through a range using flag bits to locate entries.
- `compute_scaled_char_values_2`: Loops through a range of indices, calculating scaled values using arithmetic on bytes from three different tables and storing the results in two destination buffers based on bit flags.
- `scale_and_store_byte`: Adds 64 to a 16-bit value in d2, shifts it right by 7, and stores the resulting byte at an offset from a pointer in a3 using d0 as the index.
- `compute_scaled_char_values`: The routine loops over a range of indices, using flag bits to select entries and computing scaled character values from two tables with multiplication and shifts.
- `load_pointers_and_branch_2`: Loads two pointers from offsets 104 and 112 of the structure in a5 into a0 and a1, moves a word from offset 158 of the structure into d1 and then d0, and branches to another location.
- `process_audio_stream_samples`: Iterates through two data streams, applying scaling factors and bit shifts to bytes based on a control byte's bit 5 status and the distance between two indices.
- `calculate_scaled_value_2`: Adds 64 to d2, shifts it right by 7, adds the result to a byte at an offset from a3 using d0 as an index, clears d2, loads a byte from a3 at offset d1 into d2, sign-extends d2 to a word, multiplies it by 65485, and either subtracts 64 or branches to another location based on the sign.
- `modify_and_multiply_value`: Adds 64 to d2, shifts d2 right by 7, adds the result to a byte at offset d1 of a3, sets d2 to 0, loads a byte from a6 at offset d0 into d2, subtracts 110 from d2, multiplies d2 by -230, and branches to another location if the result is positive, otherwise it subtracts 64 from d2 and branches to another location.
- `modify_table_byte_2`: Adds 64 to d2, shifts it right by 7, and adds the result to a byte at an offset from a6 determined by d0; it then loads a byte from a6 at an offset from a1, performs arithmetic involving a multiplication by 26, and branches based on the result.
- `modify_table_byte`: Adds 64 to a 16-bit value in d2, shifts it right by 7, and stores the resulting byte at an offset from a pointer in a6 using d1 as the index.
- `add_byte_and_branch`: Adds a byte from d2 to a byte at an offset from a pointer in a2 and then branches to another routine.
- `transform_narrator_audio_stream`: Performs a series of arithmetic transformations on bytes within two data streams, applying scaling factors and bit shifts based on a control value in d2.
- `transform_narrator_data_stream_2`: Adds 64 to d2, shifts it right by 7, and adds the result to a byte at an offset from a3 determined by d0; it then loads a word from a3 at an offset from a1, sign-extends it, multiplies it by d7, and branches based on the result.
- `transform_narrator_data_stream`: Iterates through a data structure using a pointer, performing arithmetic transformations on bytes based on bitmask checks and constant multipliers.
- `process_narrator_data_stream`: Iterates through a data structure using a pointer, performing arithmetic transformations on bytes based on bitmask checks and constant multipliers.
- `process_audio_stream_samples_2`: Iterates through two data streams, applying scaling factors and bit shifts to bytes based on a control byte's bit 5 status and the distance between samples.
- `transform_byte_value`: Loads a byte from a buffer at an offset in d0 into d1, multiplies the sign-extended word by 102, and then either subtracts 64 or branches to another routine based on the sign of the result.
- `update_destination_byte`: Adds a value derived from a 16-bit register shifted right by 7 to a byte in a destination buffer at an offset in d0, then branches to another routine.
- `adjust_tile_heights_with_mask`: Iterates over a byte range using a mask in a global variable and a table pointed to by a register, updates a byte in a destination buffer from a source buffer with a scaled difference based on a running maximum, and writes an accumulated scaled value to another buffer.
- `load_buffer_pointers`: Loads two longwords from offsets 104 and 104 of a structure in a5 into a2 and a0, clears d1, moves the word in d4 to d0, and branches to another routine.
- `apply_scaled_offset_to_buffer`: Iterates through a buffer backwards, checking if the high nibble of a byte is non-zero, and if so, adds a value to that byte derived from multiplying a byte at a destination offset by a constant and shifting the result.
- `scale_buffer_by_loop`: Decrements a counter in d0 in a loop, adding a value derived from a 16-bit register shifted right by 7 to a byte in a destination buffer at an offset in d0, then returns.
- `process_audio_waveform_buffers`: Iterates through a buffer to perform arithmetic transformations on bytes, applying bitwise masks and subtracting values between multiple destination and source buffers.
- `copy_and_increment_buffer_values`: Iterates through a loop, copying a byte from one location to another, subtracting a value from the destination byte, and then adding 5 to a byte at a location pointed to by a value stored in a destination byte.
- `process_masked_buffer_data`: Iterates through a buffer to copy bytes between three locations while applying a bitwise mask and arithmetic transformations based on values from a control table.
- `update_buffer_with_masked_value`: Iterates through a loop, copying a byte from a source buffer to both a destination buffer and a third buffer, then adds a value to the third buffer derived from the lower nibble of a byte at a fourth location.
- `load_address_and_call`: Loads the address stored at a longword at offset 10708 into a4, calls another routine, and branches to another location.
- `load_address_and_branch_7`: Loads a 32-bit constant into a4, calls another routine, and then branches to another location.
- `initialize_waveform_parameters`: Loads a pointer from an offset of a5 into a4, sets d6 to 110, and moves the value of d5 into d0 before branching to another location.
- `adjust_waveform_amplitude`: Iterates backward through a buffer to perform a weighted accumulation of bytes, then identifies a specific bit pattern to calculate a scaling factor based on a segment length.
- `normalize_and_scale_buffer`: Iterates backward through a buffer to find a non-zero bit, calculates a scaling factor based on the difference between a value and a constant, and then loops through the buffer to swap bytes between two locations while applying the scaling factor and an offset to a third buffer.
- `process_buffer_and_write_results`: Swaps bytes between two buffers in a loop, then if a byte in d2 is a backspace, finds a maximum value in a buffer, calculates a scaled value, and writes it to several locations; otherwise, it writes specific byte values to a buffer based on the value in d2.
- `initialize_registers_and_branch`: Clears registers d7 and d1, moves a word from d4 to d0, and then branches to another location.
- `accumulate_bytes_and_write_results`: The routine accumulates bytes from a source area into a register, keeps a running maximum, derives a scaled character value, and writes that value and a derived value to several destination buffers with a conditional copy.
- `decrement_pointers_and_branch`: Loads three pointers from offsets 124, 128, and 132 of the structure in a5 into a0, a1, and a2, decrements each pointer by 1, sets d2 to 0, moves d4 into d0, and branches to another location.
- `apply_audio_sample_scaling`: Iterates through a buffer using a loop, performing arithmetic scaling and bitwise shifts on bytes based on values from a lookup table and applying the results to multiple destination buffers.
- `scale_data_value`: Loads a pointer from an offset of a5 into a6, loads a byte from a6 at offset d0 into d1, subtracts 110 from d1, multiplies d1 by 26, and either subtracts 64 or branches to another location based on the sign.
- `scale_audio_sample_buffer`: Iterates through a data buffer using a pointer, performing arithmetic scaling and bitwise shifts on bytes based on values from a lookup table and adding the results to multiple destination buffers.
- `adjust_audio_sample_buffer`: Iterates through a data buffer, performing bitwise shifts and arithmetic scaling on bytes based on values found in a lookup table.
- `compare_and_branch_value`: Decrements d7, compares it to d0, and if they are equal, branches to another location; otherwise, it sets d7 to 64 and branches to another location.
- `set_register_and_branch_2`: Sets d7 to 86 and branches to another location.
- `calculate_scaled_value`: Loads a word from a table, multiplies it by 26, and if the result is negative, subtracts 64 from it.
- `scale_and_update_buffers`: Adds 64 to a 16-bit value in d2, performs an arithmetic shift right by 7, and then adds and subtracts this value from bytes at offsets d0 in two different buffers pointed to by a6 and a3 before looping via a decrement and branch instruction.
- `branch_to_offset`: Performs a short branch to an instruction located 56 bytes ahead in the code segment.
- `copy_and_adjust_bytes_with_feedback`: The routine copies bytes between buffers, updates a feedback value by multiplying it with a table entry and shifting, subtracts the feedback from a byte, and writes adjusted bytes to two output areas.
- `check_narrator_flag_and_dispatch`: Calls another routine twelve times and then checks if a byte at offset 219 of a structure pointed to by a5 is non-zero; if it is, it calls another routine before returning.
- `call_subroutine_three_times`: Loads a pointer from a global variable, adds 3, 4, and 5 to it respectively, and calls another routine after each addition.
- `interpolate_and_store_bytes`: The routine reads two bytes from a buffer, computes a scaled difference based on an index, accumulates the result, and stores successive byte results into the buffer at regular intervals.
- `increment_and_check_byte`: Sets register d1 to zero, adds 8 to it, loads a byte from a table at offset d1 in a0 into d6, and branches to another routine if d6 is not 255; otherwise it returns.
- `check_conditions_and_increment`: Compares d6 to 254 and branches if equal; otherwise, it compares d1 to 8 and, if not equal, branches to another location; if d1 is 8, it adds 8 to a0 and branches to another location.
- `call_interpolation_and_branch`: Calls another routine and then branches to another location.
- `scan_table_for_sentinel`: Sets register d1 to zero, adds 8 to it, tests a byte at offset d1 in a0, and branches to another routine if the byte is zero or if the byte is not 255; otherwise it returns.
- `scan_buffer_loop`: Compares d1 to 8 and, if not equal, branches to another location; if equal, it adds 8 to a0 and branches to another location.
- `scan_buffer_for_sentinel`: Increments an index by 8 in a loop to scan a buffer for a null terminator, then checks if the byte at that position is negative two five five to return, otherwise it continues scanning the next 8-byte block.
- `process_buffer_data_2`: Iterates through a data stream using two pointers, performing bitwise checks against a global table to either increment a pointer by multiples of 8, modify bytes at that pointer, or copy 128-byte blocks from a table into the destination.
- `update_buffer_data`: Contains a loop that iterates through a data structure, performing various pointer arithmetic operations, bitwise checks, and conditional byte writes and additions to a buffer based on values loaded from a table and register contents.
- `offset_pointer_by_index`: Shifts a word in d0 left by 3 bits, adds the result to a pointer in a0, and then branches to another location.
- `modify_register_d3`: Shifts the value in d3 right by 2 bits, compares it to 41, and if equal, sets d3 to 96; otherwise it branches to another location.
- `replace_byte_if_match`: Compares a byte in d3 to 43 and, if it is not equal, branches to another location; otherwise it sets d3 to 97 and branches to another location.
- `check_byte_and_replace`: Compares d3 to 44 and, if not equal, branches to another location; if equal, it sets d3 to 98 and branches to another location.
- `copy_bytes_from_offset_table`: Copies six bytes from a table located at an offset determined by a byte in a register to a destination pointed to by a0, then increments the destination pointer by two for each byte copied.
- `setup_address_pointers`: Loads four different addresses into a0, a1, a2, and a4 from offsets of a5 and a constant, moves a constant into a6, and sets d6, d4, d5, and d7 to specific values before branching to another location.
- `set_bit_and_compare_byte`: Sets bit 31 of d4, clears d0 and d2, loads a byte from the address in a0 into d0, and compares d0 to d5; otherwise it returns.
- `update_bitmask_table_elements`: Iterates through a data stream to perform bitwise operations on a table structure, using a calculated offset to modify specific bytes and 16.
- `update_bitmask_table`: Iterates through a data stream to perform bitwise operations on a table structure, using a calculated offset to modify specific bytes and 16-bit words.
- `calculate_table_offset`: Shifts the value in register d2 left by 3 bits and adds the result to register a6, then branches to another routine.
- `copy_indexed_bytes`: Loops through a buffer, copying three consecutive bytes from a table at a location determined by a byte in d0 to the destination in a0, and skips eight bytes if the byte in d0 is 254.
- `copy_bytes_with_index`: Copies bytes from a source table indexed by a byte in d0 to a destination buffer in a0, updating d0 with the next byte from the destination buffer, and branches to another routine.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `compare_scaled_byte`: Sets d6 to 255, clears d2, d3, and d0, sets d1 to 6, shifts d2 left by 3, loads a byte from a structure in a6 at offset d2 into d3, and branches if d3 does not equal d6.
- `update_scaled_value`: Performs a loop that adds a value from a structure offset 24 to d0, shifts the result right by 3, stores it back to the same offset, and branches to another routine.
- `compare_scaled_byte_with_zero`: Sets d6 to 255, clears d2, d3, and d0, sets d1 to 6, shifts d2 left by 3, clears d3, loads a byte from a structure in a6 at offset d2 into d3, and branches if d3 does not equal d6.
- `process_structured_data`: Runs a loop that calculates a value based on a table lookup, updates a byte at offset 24 of a structure in a6, increments the pointer in a6 by 8, and repeats until a comparison fails.
- `load_pointers_and_check_byte`: Loads several pointers into a0, a2, a4, and a6 from various offsets, clears d2 and d0, loads a byte from the address in a0 into d0, and compares d0 to 255; otherwise it branches to another location.
- `process_narrator_text_stream_3`: Iterates through a stream of bytes, performing arithmetic operations on a byte at offset 7 of an 8-byte structure pointed to by a6 based on the value of the current byte and a lookup table.
- `add_constant_to_buffer`: Runs a loop that adds the constant 10 to a byte at offset 7 of a structure in a6, decrements d2, and branches to another routine.
- `load_constant_d3`: Loads the constant 250 into d3 and branches to another routine.
- `load_negative_constant_and_branch`: Loads the constant negative 6 in register d3 and branches into another routine.
- `load_negative_constant_and_branch_2`: Loads the value negative 6 from a constant into a register and branches to another routine.
- `write_constant_to_buffer`: Runs a loop that writes the byte 230 to offset 7 of a structure in a6, decrements d2, and branches to another routine.
- `modify_buffer_by_offset`: Runs a loop that takes a byte from a structure in a6, subtracts 43, shifts it right by 2, adds the result to offset 7 of the same structure, and branches to another routine.
- `decode_nibble_checksum_bytes`: The routine reads a pointer and a count from a structure pointed to by A5 and processes a byte array in two passes.
- `wait_for_events`: Calls two functions via pointers in a global structure to retrieve values, stores these values at offsets 48 and 49 of a structure in a1, calculates a bitmask based on these values into offset 776, and enters a loop that calls another routine and a third function repeatedly.
- `process_event_loop`: Contains a loop that checks bit 6 of a byte at an offset from a2; if set, it returns, otherwise it sets bit 0 of that byte, calls another routine, and if that routine returns zero, clears bits 0 and 1 of the byte and returns; otherwise it updates a value at an offset from a3 and calls itself recursively.
- `clear_status_bits`: Clears bit 0 and bit 1 of a byte at an offset from a pointer in a register.
- `prepare_speech_io_request`: Initializes an I/O request structure by copying data from a table, sets a bit in a status byte, and populates a buffer with encoded bitstream data.
- `update_narrator_display_state`: Copies a block of data from one memory location to another, performs bitwise operations to set bits in a buffer based on an encoded stream, and updates a structure with character data and timing values before calling a series of function pointers through a global variable.
- `decode_narrator_text_stream_5`: Iterates through a data stream to parse encoded characters, updating a structure's internal state and performing bitwise manipulations on the output buffer.
- `process_narrator_text_stream_2`: Iterates through a data stream to parse encoded characters, updating a structure's internal state and performing bitwise manipulations on byte values.
- `parse_encoded_narrator_text_stream`: Iterates through an encoded data stream to extract characters by performing bitwise manipulations and table lookups, then writes the resulting characters to a destination buffer.
- `decode_narrator_text_stream_4`: Iterates through a data stream to parse encoded characters, updating a structure's internal state and performing bitwise manipulations on the output buffer.
- `parse_narrator_text_stream`: Iterates through a data stream to parse encoded characters, updating a structure's internal state and performing bitwise manipulations on the stream data.
- `branch_on_d0_sign`: Decrements d0, branches if d0 is positive, otherwise writes the word in d5 to offset 16 of a structure in a5 and branches to another routine.
- `execute_callback_loop`: The routine checks a flag in a structure, potentially updates a global variable, and then calls a series of function pointers stored in a table a specific number of times.
- `enable_dma_if_flag_set`: The routine tests a flag byte in a structure pointed to by A5 and, if set, writes a value to a custom hardware register that enables DMA channels by setting bit 15 of DMACON.
- `process_interrupt_logic`: Compares a byte from a structure to another value, and if they differ, updates several fields in a linked list structure and decrements a byte in a structure pointed to by a6, then calls another routine and returns.
- `decrement_counter_and_enable_interrupts`: The routine reads a byte from an offset in a structure pointed to by A6, decrements it and writes it back.
- `calculate_offset_and_branch_2`: Loads a byte from an offset of a3 into d2, multiplies d2 by 68, checks a word at an offset of a5, and if zero, calculates a new address by adding the result of the multiplication to a pointer from a4; otherwise it branches to another location.
- `process_callback_list_2`: Iterates through a list of structures, setting bit 0 of a byte at offset 30 of each structure and calling a function pointer stored at offset 20 of that structure.
- `update_dma_and_status`: Checks a bit in a structure at offset 210 of a5, writes to a hardware register, and enters a loop that calls another routine and sets bit 5 of a byte at offset 30 of a structure in a3 if a byte at offset 30 of a3 is non-zero.
- `set_interrupt_bit`: Sets bit 5 of a byte at offset 30 of the structure in a3 and branches to another routine.

### Qume_LetterPro_20

- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `call_subroutine_thrice_2`: Attempts to call a routine via a pointer in a global variable up to three times, returning 0 if all calls succeed or a specific value if they fail.
- `call_subroutine_thrice`: Calls another routine three times and returns -1.
- `execute_triple_handler`: Calls a routine through a pointer found at an offset from a global base three times, passing the value from a6 to the second and third calls.
- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_status_word`: Clears registers d1, d2, and d3, then compares the word at the address stored in a pointer from the stack to 1; if the values do not match, it branches to a different location, otherwise it continues to a branch instruction.
- `update_printer_status_buffer`: Initializes a buffer with a series of bytes from a table, checks specific word values at offsets 2650 and 2646 of a structure pointed to by a register, and then updates the buffer based on a sequence of conditional checks and arithmetic on a byte in a register.
- `copy_byte_from_table`: Copies a byte from a table at offset d0 to a destination at offset a3, where a3 is derived from d2 and a4, and branches to another routine if d0 is not greater than 11.
- `update_status_values_3`: Compares a word at the address in a0 to 27, and if it matches, checks if the byte at the address in a1 is less than zero; if both conditions are met, it writes 32 to the word at the address in a0, writes 1 to the byte at the address in a1, and branches to another location.
- `update_status_value`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is less than 0, it writes 4 to the word at the address in a0 and branches to another routine.
- `update_status_value_by_id`: Compares a word at the address in a0 to 28; if it matches and the byte at the address in a1 is greater than 0, it writes 33 to the word at the address in a0, otherwise it returns.
- `update_status_values_2`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is not less than or equal to 0, it writes 33 to the address in a0, writes 255 to the address in a1, and branches to another routine.
- `update_status_value_on_condition`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is not less than or equal to 0, it writes the word 2 to the address in a0, otherwise it returns.
- `update_word_on_condition`: Compares a word at the address in a0 to 30; if it matches and the byte at the address in a1 is less than 0, it writes the word 32 to the address in a0 and branches.
- `update_word_and_byte_2`: Compares a word at the address in a0 to 31.
- `increment_byte_on_mismatch`: Compares a word at the address in a0 to 32, and if not equal, increments a byte at the address in a1 and branches to another location.
- `decrement_and_branch_2`: Compares a word at a pointer in a0 to 33, and if it is not equal, decrements a byte at a1 and branches.
- `write_formatted_data`: If a word at a0 is 57, writes two bytes and two ASCII-encoded nibbles to a destination in a4; otherwise, if the word at a0 is 0, writes 253 to offset 2656 of a structure in d1.
- `write_offset_byte`: Calculates a value by adding 48 to the sign-extended version of d0 and writes the result as a byte to a destination at offset a3, where a3 is derived from d2 and a4, then branches to another routine.
- `write_status_byte`: Checks if a word at the address in a0 is zero, and if so, writes the byte 253 to offset 272 of a structure pointed to by the value in d1.

### CBM_MPS1000

- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_callbacks`: Stores a parameter and several pointers into a global structure, then calls a function at an offset from a global variable, and if that returns non-zero, calls another function at an offset from a different global variable, potentially calling a third function and a cleanup routine.
- `execute_dual_calls`: Saves a6 to the stack, calls another routine using a pointer from a6, restores a6, and repeats this process with a different pointer and value in a1 before branching to another routine.
- `execute_indirect_function_pointer`: Fetches a function pointer from a global variable using a specific offset and calls that function three times consecutively.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry`: Checks a condition via a routine call, and if met, populates a structure with two parameters and executes several routines through a global base.
- `manage_printer_buffer_state`: Acts as a multi-purpose state machine that manages a printer buffer by performing operations such as clearing memory with a loop, appending specific byte sequences, bitwise ORing data into a buffer, and updating global status flags based on conditional checks and calls to another routine.
- `restore_registers_and_return`: Clears register d0 and restores several registers from the stack before returning.
- `initialize_printer_command_state`: Saves registers to the stack, loads three pointers from the stack into a2, a4, and a3, clears d4, d5, and d1, sets d3 to 3, and branches to another routine if the word at the address in a2 is not 1.
- `format_printer_command_buffer`: Copies a sequence of bytes from a table into a destination buffer, prepending a header based on values read from a structure pointed to by a register.
- `modify_pointer_data_2`: Loads a pointer from the address in a0, compares a word at offset 2144 of that pointer to 2048, and if they match, increments d4, adds the result to a pointer in a4, writes 15 to that location, and branches to another routine.
- `process_printer_command_sequence`: Copies a byte sequence from a source to a destination using a loop until a null terminator is reached, then processes a command byte from a structure to either write specific multi-byte sequences to a buffer, toggle state flags, or update values in a structure based on specific command codes.
- `copy_table_bytes`: Runs a loop that copies bytes from a table at offset zero using a register as an index and an offset in a4, incrementing the source and destination pointers until a counter in d5 reaches four.
- `modify_memory_on_match`: Compares a word at the address in a2 to 66; if it matches, it writes the byte 1 to a memory location, loads a pointer from d1 into a1, loads a pointer from a1 into a0, compares a word at offset 2166 of the structure in a0 to 2048, and if equal, writes the byte 96 to the address in d3.
- `write_byte_on_comparison`: Loads a pointer from a0 into a1, compares a word at an offset of a1 to 1024, and if equal, writes the byte 137 to the address in d3; otherwise it branches to another location.
- `write_constant_byte_2`: Moves the address in d3 to a0, writes the byte 80 to the location in a0, and branches.
- `copy_bytes_from_lookup_table`: Iterates through a loop to copy bytes from a table into a destination calculated from a pointer and an offset.
- `modify_word_and_byte`: Compares a word at the address in a2 to 32.
- `clear_and_set_word`: Checks a byte at a pointer in a3; if it is negative, it clears that byte and writes the value 30 to a word at a pointer in a2, otherwise it branches away.
- `branch_with_constant`: Sets d0 to 255 and branches to another routine.
- `update_values_on_condition`: Compares a word at the address in a2 to 33, and if equal, checks a byte at the address in a3; if that byte is not zero, it branches to another location, otherwise it sets the byte at a3 to 255 and the word at a2 to 29; otherwise it branches to another location.
- `clear_and_set_word_2`: Tests a byte at a3, and if it is less than or equal to zero, clears that byte and writes the word 28 to a location at a2 before branching; otherwise, it branches.
- `branch_to_offset_2`: Performs a short branch to a specific offset.
- `update_data_based_on_value`: Checks the value at a pointer in a2 against several constants to write specific bytes to either a stack location or a destination at offset 2704 of a structure in d1, then returns.
- `write_status_byte_2`: Checks if a word at the address in a2 is zero, and if so, writes the byte 253 to offset 272 of a structure pointed to by the value in a1, which was loaded from a pointer in d1.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_630

- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_library_calls`: Attempts to call three consecutive functions by following pointers at specific offsets within a structure pointed to by a global variable and a parameter; if any call returns zero, it executes a cleanup routine at a different offset within the structure before returning.
- `execute_nested_calls_4`: Loads a pointer from a global address into a1, pushes a6 to the stack, calls a function at an offset from a6, restores a6 from the stack, loads a new pointer into a1, pushes a6 to the stack, calls another function at an offset from a6, restores a6, and branches.
- `execute_triple_function_call`: Calls a function located at an offset from a pointer in a global variable three times in succession.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `check_printer_status_type`: Loads three pointers from the stack and clears three registers, then compares a word at the first pointer to one and branches to a different location if it is not equal to one; otherwise it continues to another routine.
- `parse_filesystem_metadata_string`: Copies a sequence of bytes from a structure into a destination buffer, applying specific byte transformations based on values found at offsets 1658 and 1654 of that structure.
- `update_buffer_and_state_logic`: Copies a string from a source table into a destination buffer using a loop, then performs a series of conditional checks on a word and a byte to update the buffer content and modify a status value based on specific numeric ranges.
- `write_byte_from_table_in_loop`: Calculates a destination address by adding a value in a4 to a modified d2, then enters a loop that copies a byte from a zero-indexed table into that destination, incrementing d3 and d2 each iteration until d0 is no longer greater than d3.
- `update_threshold_values`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is less than 0, it writes 32 to the word at a0 and 1 to the byte at a1, then branches to another routine.
- `apply_threshold_correction_27`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is less than 0, it writes 4 to the word at the address in a0, otherwise it returns.
- `update_threshold_value_2`: Compares a word at the address in a0 to 28; if it matches and the byte at the address in a1 is greater than 0, it writes 33 to the address in a0 and branches to another routine.
- `reset_threshold_and_flag`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is greater than 0, it writes 33 to the address in a0, writes 255 to the address in a1, and branches to another routine; otherwise it returns.
- `update_counter_value`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is greater than 0, it writes 2 to the address in a0 and branches to another routine.
- `reset_threshold_on_match`: Compares a word at a pointer in a0 to thirty; if it matches and a byte at a pointer in a1 is less than zero, it writes thirty-two to the word at the pointer in a0 and branches to another routine, otherwise it returns.
- `update_threshold_and_counter`: Compares a word at a pointer in a0 to thirty-one; if it matches, it checks a byte at a pointer in a1, and if that byte is greater than zero, it writes thirty-three to the word in a0 and branches to another routine, otherwise it writes thirty-two to the word in a0 and clears the byte in a1.
- `increment_byte_on_match`: Compares a word at a pointer in a0 to thirty-two; if they match, it increments a byte at a pointer in a1 and branches to another routine, otherwise it branches to another routine.
- `decrement_byte_on_mismatch`: Compares a word at a pointer in a0 to thirty-three; if they do not match, it decrements a byte at a pointer in a1 and branches to another routine, otherwise it returns.
- `write_conditional_bytes`: If a word at a pointer in a0 is 12, it writes the byte 27 to a destination calculated from a4 and d2, and then writes either 65 or 66 to the next byte based on a stack value; otherwise, if the word is 0, it writes the byte 253 to an offset of a structure pointed to by a pointer in d1.
- `write_constant_byte_and_branch`: Calculates a destination address by adding a value in a4 to a modified d2, writes the byte 66 to that location, and then branches to another routine.
- `write_status_byte_3`: Checks if a word at the address in a0 is zero, and if so, writes the byte 253 to offset 272 of a structure pointed to by the value in a3, which was loaded from a pointer in d1.

### ImagewriterII

- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_release_library_handles`: Attempts to call functions at specific offsets within two different structures pointed to by a global variable and a passed parameter; if these calls succeed, it attempts to release the structures by calling a function at a specific offset within them.
- `execute_dual_callback_sequence`: Loads a pointer from a global variable into a1, pushes a6 onto the stack, and calls a routine at an offset from a pointer retrieved from a global variable.
- `execute_printer_driver_function`: Retrieves a function pointer from a global variable using an offset and calls that function three times, passing the value from register a6 as an argument to each call.
- `return_constant_20`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_21`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_2`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value at specific offsets, then executes three routines through a global base and one routine through a pointer stored within that structure.
- `POSDIV`: Performs a division operation using a loop or direct calculation depending on the value in a register, then returns the result in d1 and d0.
- `LONGDIV`: Performs sixteen iterations of a 32-bit left rotation by one bit, using the carry flag to transfer bits between the high and low halves.
- `ulmult`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components.
- `ulmodt`: Moves the contents of register d1 into d2 and register d0 into d1 before calling POSDIV and returning.
- `uldivt`: Saves d2 to the stack, moves d1 into d2 and d0 into d1, calls another routine, moves the resulting value from d1 into d0, restores d2, and returns.
- `lmodt`: Calls POSDIV and negates the returned value if the flag left in an address register by the called routine is nonzero.
- `ldivt`: Calls POSDIV and returns its first result, or the negation of that result if the second result in an address register is nonzero.
- `dispatch_via_stack_values`: Restores registers from the stack, reads three values from the stack, and uses a value from the stack to index into a table to perform a jump to a calculated address.
- `return_zero_alt`: Clears the d2 register, moves it to d0, restores registers from the stack, and returns.
- `initialize_printer_handler`: Saves registers d2 through d4 and a2 through a4 to the stack, then loads a pointer from offset 28 of the stack into a1, a 32-bit value from offset 32 into d2, and a pointer from offset 36 into a3.
- `format_imagewriter_escape_sequences`: Copies a header from a table into a destination buffer and appends specific escape sequences based on the contents of a structure, including font selection, character scaling, and line spacing commands.
- `encode_imagewriter_escape_sequences_2`: Translates internal control codes into specific escape sequences for an ImageWriter II printer by writing bytes to a memory buffer.
- `translate_printer_control_codes`: Translates internal control codes into specific escape sequences for an ImageWriter II printer by writing bytes to a memory buffer.
- `modify_escape_sequence`: Compares a word at the address in a1 to 31; if it matches and the byte at the address in a3 is greater than 0, it writes 33 to the word at a1 and 1 to the byte at a3, then branches to another routine.
- `modify_byte_and_word`: Checks if the byte at the address in a3 is less than 0; if so, it writes 32 to the word at the address in a1, sets the byte at the address in a3 to 255, and branches to another routine.
- `branch_to_address`: Branches to a specific address.
- `translate_imagerwriter_escape_sequences_3`: Translates input bytes into specific escape sequences for an ImageWriter II printer by writing bytes to a destination buffer.
- `write_four_bytes_to_memory`: Writes the bytes 66, 27, 108, and 48 sequentially to a memory location calculated by adding d2 and d3.
- `translate_imagerwriter_escape_sequences_2`: Translates input bytes into specific escape sequences for an ImageWriter II printer by writing bytes to a destination buffer.
- `write_single_byte_2`: Calculates a destination address by adding d2 and d3, writes the byte 66 to that location, and then branches to another routine.
- `translate_imagerwriter_escape_sequences`: Translates specific input bytes into corresponding escape sequences for an ImageWriter II printer by writing bytes to a destination buffer.
- `write_byte_at_offset_and_branch`: Writes the byte 80 to a destination address calculated by adding d2 to a modified d3, then branches to another routine.
- `encode_imagerwriter_escape_sequences`: Translates specific input bytes into corresponding escape sequences for an ImageWriter II printer by writing bytes to a destination buffer.
- `write_byte_at_offset_and_branch_alt`: Writes the byte 112 to a destination address calculated by adding d2 to a modified d3, then branches to another routine.
- `encode_escape_sequences_2`: Checks the byte at the address in a1 to determine whether to update a global variable or write specific escape sequences to a destination buffer, and calls another routine before returning.
- `encode_escape_sequences`: Writes escape sequences to a destination buffer based on the value at a pointer in a1, handling specific control characters and updating a counter in a global variable.
- `write_conditional_byte`: Checks if a longword at address zero is 1; if not, it branches away; otherwise, it increments a pointer in d3, calculates a destination by adding d2, and writes the byte 80 to that location.
- `update_printer_status_byte`: Increments a value in a register, calculates a new address by adding two registers, stores the byte value one hundred and twelve at that address, and branches to another routine.
- `encode_imagewriter_escape_sequences`: Writes escape sequences to a destination buffer based on the value at a pointer in a1; it handles specific control characters by writing an escape byte followed by a command byte and a character byte derived from a counter or a table.
- `update_pointer_and_byte`: If a byte at a memory location is not one, it increments a value in d3, calculates a destination address by adding d2 to the new d3, writes the byte 69 to that address, and branches to another routine; otherwise it branches to another routine.
- `write_byte_at_offset_and_branch_alt_3`: Writes the byte 78 to a destination address calculated by adding d2 to a modified d3, then branches to another routine.
- `update_printer_state`: If a word at a1 is 48, it checks a value at a stack pointer, potentially setting it to 5 or 10, then writes bytes 27 and 115 to a destination calculated from d2 and d3, decrements a value at the stack pointer, and writes a byte to a location at offset 2 from the destination.
- `modify_byte_based_on_value`: If a word at a pointer in a1 is twelve, it checks a pointer from the stack; if that byte is 39, it writes 30 to that location, and if that byte is 37 or greater, it branches to another routine; otherwise, if the byte is 39, it writes 30, and if the byte is 37, it branches to another routine; if the word in a1 is not twelve, it branches to another routine.
- `write_three_bytes_to_memory`: Writes the byte 27 and the byte 75 to a memory location calculated by adding d2 and d3, then writes a byte to the next location by adding a value from a global table to the character '0'.
- `return_zero_2`: Sets the return register to zero and restores registers from the stack before returning.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_C-150

- `return_constant_22`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_dual_calls_sequence`: Saves a6 to the stack, calls another routine using a pointer from a6, restores a6, and repeats this process with a different pointer and value.
- `execute_dual_callback_sequence_2`: Loads two different pointers from a global structure into a1 and a6, then performs two consecutive calls to routines located at an offset from the pointers in a6.
- `call_function_pointer_triple`: Calls a function through a pointer located at an offset from a global base three times.
- `return_constant_23`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_24`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_3`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte at specific offsets, then executes several routines through a global base using a pointer retrieved from that structure.
- `divide_32_bit_integers`: If a value in a register is greater than 65535, it performs a bitwise rotation and subtraction loop; otherwise, it performs division and modulo operations using a 16-bit divisor.
- `rotate_and_threshold_32bit`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_3`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit components and returns the result.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `dispatch_via_table`: Loads several values from the stack, compares a byte to 5, and then uses the result to jump to an address from a table.
- `return_zero_short`: Sets the d0 register to zero and restores registers from the stack before returning.
- `update_numeric_display_buffer`: Updates a string buffer with ASCII digits representing a value, increments and clamps a counter, or performs a lookup and calculation to write specific bytes into a destination buffer and calls another routine.
- `copy_byte_in_loop`: Copies a byte from a table at offset 10 to a destination calculated from a2 and d3, then branches based on a comparison between d2 and d5.
- `process_data_structure`: If a word at a pointer in a3 is twelve, it modifies a byte from the stack based on its value, writes three bytes to a destination calculated from d3 and d4, and calls another routine to write a byte from a global table; otherwise, if the word in a3 is zero, it writes 253 to an offset of 1696 from a pointer in a4.
- `write_data_structure`: Calculates a value using two values from the stack, writes three bytes to a memory location, and then writes a byte from a global table into that same location after calling another routine.
- `write_byte_if_zero`: Tests a word at a pointer in a register and, if it is zero, writes the byte 253 to an offset of 2704 from a second pointer in a register.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### generic

- `return_constant_25`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_26`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_27`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero`: The routine returns a constant zero value.

### Brother_HR-15XL

- `return_constant_28`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_callbacks_2`: Loads a value from a global variable, performs three sequential calls to routines at specific offsets from that value, and returns either negative one or zero depending on the success of those calls.
- `execute_dual_callbacks`: Loads a pointer from a global variable into a1, saves a6 to the stack, calls a routine at an offset from a pointer in a6, restores a6 from the stack, loads a different pointer into a1, calls a routine at the same offset from a pointer in a6, restores a6, sets d0 to negative 1, and branches to another location.
- `execute_triple_pointer_call`: Calls a routine through a pointer stored in a global variable, then calls that same routine twice more using a value from an address register as an argument.
- `return_constant_29`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_30`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_31`: Returns the constant an address in d0 to the caller without any other effect.
- `check_printer_status_word`: Initializes registers d2, d3, and d1 to zero, loads three pointers from the stack into a0, a1, and a4, compares a word at the address in a0 to 1, and branches to a different location if it is not 1 or branches to a third location if it is 1.
- `process_printer_command_stream`: Copies a sequence of bytes from a source buffer into a destination buffer, applying specific byte transformations based on values read from a structure.
- `process_printer_control_sequence`: Copies a null-terminated string to a destination buffer and then processes a series of control characters by updating a state byte and a counter based on specific character matches.
- `copy_byte_with_loop`: Copies a byte from a table at offset 0 to a destination calculated from a2 and a4, then branches based on a comparison between d0 and d3.
- `update_values_on_match`: Compares a word at the address in a0 to 27; if equal, it checks if the byte at the address in a1 is less than zero, and if so, writes 32 to the address in a0 and 1 to the address in a1, then branches to another routine.
- `update_status_on_match_3`: Compares a word at the address in a0 to 27; if equal, it checks if the byte at the address in a1 is less than zero, and if so, writes the word 4 to the address in a0 and branches to another routine.
- `update_status_on_match_2`: Compares a word at the address in a0 to 28; if equal, it checks if the byte at the address in a1 is non-zero, and if so, writes the word 33 to the address in a0 before branching to another routine; otherwise it returns.
- `update_status_on_match`: Compares a word at the address in a0 to 29 and checks if the byte at the address in a1 is non-zero; if both conditions are met, it writes the word 33 to the address in a0 and the byte 255 to the address in a1, otherwise it returns.
- `update_word_on_condition_2`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is greater than 0, it writes the word 2 to the address in a0 and branches.
- `set_word_on_match`: Compares a word at the address in a0 to 30.
- `update_word_and_byte`: Compares a word at the address in a0 with 31; if not equal, it branches.
- `increment_byte_on_match_2`: Compares a word at the address in a0 with 32; if not equal, it branches.
- `decrement_byte_on_match_2`: Compares a word at the address in a0 with 33, and if they are not equal, it decrements the byte at the address in a1 before branching to another location.
- `write_conditional_bytes_2`: If a word at a pointer in a0 is twelve, it writes the byte 27 to a destination calculated from a4 and d2, then writes either 65 or 66 to the next byte depending on a byte from the stack; otherwise, if the word in a0 is zero, it writes 253 to an offset of 4256 from a pointer in d1.
- `write_constant_byte`: Calculates a destination address from a2 and a4, writes the byte 66 to that address, and then branches.
- `write_byte_if_zero_2`: Checks if a word at the address in a0 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d1.

### HP_LaserJet

- `return_constant_32`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_library_calls_2`: Attempts to call three consecutive functions via offsets from pointers stored in a global variable and a passed parameter; if any call returns non-zero, it proceeds to the next, otherwise it calls a cleanup routine via an offset from the global variable and returns.
- `execute_dual_callback_sequence_3`: Loads a pointer from a global variable into a1, then performs two consecutive calls to functions located at an offset from pointers stored in a table.
- `execute_indirect_function_calls`: Fetches a function pointer from a table using an offset and calls that function three times, passing the value from a register as an argument to the first call and the subsequent calls.
- `return_constant_33`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_4`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine through a pointer stored within that structure.
- `format_hp_laserjet_escape_sequences_2`: Copies a sequence of bytes from a table into a buffer and modifies specific bytes based on hardware status flags, then calls another routine to append additional bytes from a table and calls another routine to finalize the sequence.
- `format_hp_laserjet_escape_sequences`: Copies a sequence of bytes from a table into a buffer, modifies specific bytes based on hardware-specific status flags, and calls another routine to output the resulting control characters.
- `process_printer_control_sequences`: Updates a buffer with control characters and calls translate_printer_control_codes to convert internal codes into escape sequences.
- `copy_table_elements`: Enters a loop that copies 11 elements from a global table to a destination, then calls another routine twice using values derived from a structure.
- `set_word_and_byte`: Compares a word at the address in a2 with 27; if not equal, it branches.
- `update_control_code_word`: Compares a 16-bit word at the address in a2 to 27, and if it matches and the byte at the address in a3 is less than 0, it writes the word 4 to the address in a2 and branches to another routine.
- `update_word_on_match`: Compares a word at the address in a2 to 28; if it matches and the byte at the address in a3 is greater than 0, it writes the word 33 to the address in a2 and branches to another location.
- `update_word_and_byte_3`: Compares a word at the address in a2 to 29; if it matches and the byte at the address in a3 is not 0, it writes the word 33 to the address in a2, writes the byte 255 to the address in a3, and branches.
- `set_word_on_condition`: Compares a word at the address in a2 to 29.
- `update_word_on_condition_3`: Compares a word at the address in a2 to 30; if it matches and the byte at the address in a3 is non-negative, it writes the word 32 to the address in a2 and branches to another location.
- `update_word_based_on_byte`: Compares a word at the address in a2 to 31; if it is not 31, it branches to another location, otherwise it checks a byte at a3.
- `compare_and_increment_byte`: Compares a word at the address in a2 to 32; if it does not match, it branches to another location, otherwise it increments a byte at the address in a3 and branches to another location.
- `decrement_status_byte`: Compares a word at the address in a2 to 33; if it does not match, it branches to another location, otherwise it decrements a byte at the address in a3 and branches to a third location.
- `write_byte_to_pointer_3`: Compares a word at the address in a2 to 64; if it matches, it loads a pointer from an offset of a structure in a6 into a0, checks if the byte at that pointer is non-zero, and if so, writes the byte from a global variable to that pointer before branching to another routine.
- `decrement_and_copy_data`: Decrements a byte at the address pointed to by an offset of a structure in a6, updates a global word with the new value, conditionally updates a byte at the next address, copies 11 bytes from a global variable to a destination buffer in a loop, and calls two routines using pointers derived from the structure.
- `copy_byte_from_offset`: Increments a longword from offset 28 of a structure in a6, loads a byte from that new address into d1, and then writes that byte to the address pointed to by a4.
- `copy_and_call_functions`: Copies 11 bytes from a global variable to a destination buffer pointed to by a combination of a4 and d5 in a loop, then calls another routine twice using values derived from a structure pointed to by a6.
- `compare_word_and_branch_4`: Compares a word at the address in a2 to 57; if it is not equal, it branches to a different location, otherwise it branches to another location.
- `copy_table_and_call`: Copies 11 bytes from a global variable to a destination buffer using a loop, then calls two separate routines using pointers stored in a structure.
- `write_byte_if_zero_alt`: Checks if a word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d2.
- `format_numeric_string_4`: Writes a character to a destination buffer based on the value of a parameter, then calls another routine to write a second character to the next byte in the buffer.
- `convert_byte_to_ascii`: Compares a byte in d0 with 99; if less than or equal, it branches.
- `format_numeric_string_2`: Stores the byte 48 at a memory location, checks if a byte is less than 10, and if so stores 48 at the next byte, otherwise it stores the result of dividing that byte by 10 plus 48 at the next byte, then calls another routine and stores the result of that routine plus 48 at the following byte.
- `format_numeric_string`: Stores the byte 48 at a memory location, calls another routine, and then stores the result of that routine incremented by 48 at the next byte location.
- `call_two_global_pointers`: Calls a routine using a pointer from a global variable with two arguments, then calls another routine using a pointer from the same global variable.
- `perform_large_division`: Performs a division of a 32-bit value by a 16-bit value and returns the quotient and remainder as a 64-bit result.
- `rotate_32bit_value_4`: The routine rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to move bits between the high and low halves, and subtracts a threshold from the high half if the rotated value meets or exceeds it.
- `multiply_32_bit_integers_4`: The routine computes the 32-bit product of two 32-bit registers using 16-bit partial products and combines them to form the final result.
- `negate_result_on_flag`: Calls another routine and negates the returned integer if the value in an address register is nonzero.
- `dispatch_via_jump_table`: Loads a byte from the stack, compares it to five, and if it is less than five, uses it to index into a table to jump to another routine; otherwise, it performs an arithmetic operation on the value and jumps to a different location in a table.
- `clear_d0_and_restore_registers`: Clears register d0 and restores d2, d3, a2, a3, and a4 from the stack before returning.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `validate_word_range`: Saves registers a2 and a3 to the stack, loads a word from the stack into d0, and sets a1, a3, and a2 to specific constants.
- `compare_word_range`: Compares a word in d0 with 256, and branches to another location if the value is less than 256 or greater than 256, otherwise it branches to another location.
- `compare_and_branch_2`: Compares the word in d0 with 512 and branches if not equal, otherwise it performs a short branch.
- `compare_word_and_branch_2`: Compares a word in d0 to 1024 and branches if they are not equal.
- `initialize_structure_values`: Writes several 16-bit and 32-bit values to offsets of a structure pointed to by a1 and a second pointer, and updates a global variable.
- `initialize_structure_fields_4`: Writes the constant 800 to offset 38 of a structure, the constant 1060 to offset 30 of the same structure, sets a 16-bit value of 100 at offset 36 of a global variable and offset 34 of the structure, and writes the byte 49 to offset 45 of the structure before branching to another routine.
- `initialize_printer_structure_fields`: Writes four 16-bit values to specific offsets of a structure pointed to by a1, updates a global variable, and stores two bytes at offsets 45 and 53 of the structure.
- `initialize_structure_fields`: Writes four 16-bit values to specific offsets of a structure pointed to by a1, updates a global variable, and writes values to two memory locations pointed to by a2 and a3.

### Okidata_292

- `return_constant_34`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_calls`: Loads a value from a global variable, sets three temporary storage locations, and then performs a sequence of three calls to functions retrieved from offsets of the provided pointer and the global variable, returning 0 if all calls succeed or negative one otherwise.
- `execute_dual_callback_sequence_4`: Loads two different pointers from a table and calls two separate routines by jumping to an offset from each pointer, then returns with a signed byte value of negative one in d0.
- `call_function_via_pointer`: The routine calls a function through a pointer located at an offset from a global variable twice, passing the value from a register to the second and third calls.
- `return_constant_35`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_36`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_5`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte value, then executes three routines through a global base using a pointer retrieved from that structure.
- `manage_buffer_data_structure`: Implements a recursive state machine to manage a data buffer through four modes: mode 0 calculates offsets and calls itself to initialize the structure, mode 1 performs bitwise OR operations on buffer elements, mode 2 checks buffer bounds and updates an offset, and mode 3 clears the buffer and populates it with a sequence of bytes including constants and values from a global array.
- `clear_d0_and_restore_registers_2`: Sets register d0 to zero and restores several data and address registers from the stack before returning.
- `initialize_command_buffer`: Saves registers to the stack, loads several values from the stack, and if a word at the address in a2 is not 1, it branches to another routine; otherwise it branches to a different location.
- `construct_printer_command_buffer`: Copies a sequence of bytes from a table into a destination buffer, appending a string and specific control bytes based on values read from a structure pointed to by a register.
- `check_and_write_byte`: Loads a pointer from a location pointed to by d1, compares a word at an offset of that pointer with 2048, and if equal, increments d3, adds d2 to d3, and writes the byte 15 to that address before loading a new pointer from a table.
- `update_filesystem_command_buffer`: Copies a null-terminated string into a destination buffer and then performs a series of conditional writes to that buffer or other memory locations based on the value of a word at a specific address.
- `loop_table_copy`: Increments d3 and d6, calculates a destination address by adding d2 to d3, copies a byte from a table at offset d1 to that destination, and loops if d6 is less than 4.
- `modify_memory_on_match_2`: Compares a word at the address in a2 to 66; if it matches, it writes the byte 1 to a memory location, loads a pointer from d1 into a1, loads a pointer from a1 into a0, compares a word at offset 2166 of the structure in a0 to 2048, and if equal, writes the byte 96 to the address in d5.
- `write_byte_to_pointer_2`: Loads a pointer from a global variable into a1, loads a pointer from the address in a1 into a0, compares a word at an offset of a0 to 1024, and if it matches, writes 137 to the address in a0 before branching to another location.
- `write_byte_to_pointer`: Moves the address in d5 to a0, writes the byte 80 to the address in a0, and branches.
- `copy_bytes_from_table_to_memory`: Copies a sequence of bytes from a table to a memory location calculated from two registers, incrementing a counter and a pointer for each byte until a limit is reached, then branches to another routine.
- `update_status_values`: Compares a word at the address in a2 with 32; if not equal, it branches.
- `clear_and_store_word`: Tests a byte at the address in a3; if it is not greater than or equal to zero, it clears that byte, stores the word 30 at the address in a2, and then performs a short branch.
- `load_negative_one_and_branch`: Loads the value negative one into a data register and performs a short branch to another routine.
- `compare_and_update_values_2`: Compares a word at the address in a2 with 33; if it is not equal, it tests a byte at the address in a3, which is then set to 255 if it is zero or less, and finally stores the word 29 at the address in a2 before branching.
- `clear_and_store_word_alt`: Tests a byte at the address in a3; if it is not less than or equal to zero, it clears that byte, stores the word 28 at the address in a2, and then performs a short branch.
- `branch_to_routine`: Performs a short branch to another routine.
- `process_status_values`: Checks a byte at the address in a2 against several constants to set a byte or a stack value, and if a2 is 2, writes three bytes to a destination calculated from d2 and d3, or if a2 is 12 and a stack value is 39, writes three bytes to a destination calculated from d2 and d3 using a value from a table.
- `check_status_byte_2`: Compares a word at a location pointed to by a register against 12; if not equal, it branches.
- `write_three_bytes`: Writes the bytes 27 and 114 to a memory location, then writes a 32-bit word from a global variable table to the following offset.
- `write_status_byte_4`: If the word at a location pointed to by a register is zero, it stores the byte 253 at an offset of 2688 from a pointer in a register.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Okimate_20

- `return_constant_37`: Returns the constant an address in d0 to the caller without any other effect.
- `open_multiple_libraries`: Sets up a stack frame with a global variable and its address, then calls a function from a library at an offset.
- `execute_nested_calls_5`: Calls a routine using an offset from a base address in a6 and a pointer from a global variable, then calls another routine using a different pointer from a global variable and a different offset from a6.
- `update_handler_pointer`: Calls a library function three times and then updates a global variable with its own address.
- `update_printer_handler`: Loads a pointer from a memory location, compares a word at offset 100 of that pointer to 2, and if it is equal, stores a constant address into a global variable at offset 54.
- `store_routine_pointer`: Stores the address of this routine into a global variable at offset 54 and returns 0.
- `return_constant_38`: Returns the constant an address in d0 to the caller without any other effect.
- `manage_printer_buffer_state_2`: Acts as a state machine for a printer buffer, handling buffer allocation, bitwise data insertion, and byte-by-byte command formatting.
- `restore_registers_and_return_2`: Clears register d0 and restores several data and address registers from the stack before returning.
- `manage_printer_buffer_state_3`: Uses a switch-based state machine to manipulate a memory buffer, including zeroing out a range of bytes, writing a specific byte sequence starting with 27, 37, and 79, and performing bitwise OR operations on buffer contents using values from a global variable.
- `restore_registers_and_return_3`: Clears register d0 and restores a set of registers from the stack before returning.
- `perform_division`: Performs a division operation on a 32-bit value using a 16-bit divisor and returns the quotient and remainder.
- `rotate_and_threshold_32bit_2`: Rotates a 32-bit value left by one bit for sixteen iterations, using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_5`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products and returns the result.
- `forward_call_to_helper_3`: Moves the contents of register d1 into d2, moves the contents of register d0 into d1, calls a routine to measure a string length, and returns.
- `initialize_stack_and_compare`: Saves registers to the stack, loads three pointers from the stack into a0, a3, and a2, clears d2, d5, and a4, sets d3 to 26 and d4 to 16, and compares a word at the address in a0 to 1; if it is not 1, it branches to a different location, otherwise it branches to another location.
- `build_data_sequence_2`: Copies 15 bytes from a global variable to a destination buffer in a loop, then performs several conditional checks on values within a structure pointed to by a4 to write specific byte sequences to the buffer.
- `build_data_sequence`: Writes a sequence of bytes to a destination buffer based on comparisons of values at specific offsets within a structure pointed to by a4, including a loop that writes the byte 13 a variable number of times.
- `fill_buffer_loop`: Enters a loop that writes the byte 13 to a buffer at an offset determined by a counter, incrementing the counter and the buffer pointer in a loop until a limit.
- `update_printer_escape_sequences`: Processes a control byte to append specific escape sequences, carriage returns, or line feeds to a buffer, while toggling state flags and updating a counter based on the input value.
- `fill_buffer_with_carriage_return`: Fills a memory buffer with the carriage return byte using a loop that continues until a counter reaches a limit value read from a pointer-based structure.
- `process_printer_control_codes`: Checks a control word and updates a buffer with specific escape sequences and padding bytes based on the value.
- `fill_buffer_with_constant`: Writes the byte 13 to a buffer at an offset determined by a counter, incrementing the counter and the buffer pointer in a loop until a limit is reached.
- `compare_and_set_values`: Compares a word at the address in a0 to 32; if it is not equal, it tests a byte at the address in a2, and if that byte is not zero, it sets that byte to 1, stores 1 in the word at the address in a0, and branches to another routine; otherwise it branches to another routine.
- `update_command_value`: Tests a byte at a location pointed to by a register; if it is negative, it clears that byte.
- `set_register_and_branch`: Sets register d0 to negative one and branches to another location.
- `check_command_byte`: Compares a word at the address in a0 to 33; if it is not equal, it tests a byte at the address in a2, and if that byte is not zero, it sets it to 255 and writes the word 29 to the address in a0 before branching.
- `clear_and_branch_4`: Tests a byte at the address in a2; if it is not less than or equal to zero, it clears that byte, stores the word 28 at the address in a0, and branches.
- `branch_to_another_routine`: Sets register d0 to negative one and branches to another routine.
- `map_command_byte`: Compares a byte pointed to by a register against several constants and writes a corresponding byte to a stack location or a pointer.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_Advantage_D25

- `return_constant_39`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_calls_2`: Checks if three consecutive function calls through pointers at specific offsets from two different base addresses return non-zero, and if the third call fails, it calls a routine at an offset from a third base address before returning zero or negative one.
- `execute_dual_calls_2`: Loads two different addresses from a table into a1 and a6, then performs two calls to offsets of those addresses before returning.
- `execute_triple_callback`: Calls a routine through a pointer in a table, then calls that same routine twice more using a value from a register as an argument.
- `return_constant_40`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_41`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_42`: Returns the constant an address in d0 to the caller without any other effect.
- `check_printer_type_and_clear_registers`: Clears registers d2, d3, and d1, then compares a word at an offset from a pointer on the stack to one; if the value is not one, it branches to a different location, otherwise it continues to another routine.
- `parse_filesystem_metadata_string_2`: Copies a sequence of bytes from a structure into a destination buffer, applying specific byte substitutions based on values found at offsets 2650 and 2646 of that structure.
- `process_buffer_and_update_values`: Copies bytes from one location to another in a loop until a null byte is encountered, updates a longword at an offset of a pointer in d1, sets a word at a0 to 65, and if that word is 65, copies 11 bytes from a global variable to a destination buffer in a loop and returns a value; otherwise, it performs several conditional checks and updates values at a0 and a1.
- `copy_byte_from_table_2`: Copies a byte from a table at offset d0 to a destination at a3 + d2, then loops based on a comparison between d0 and d3.
- `update_threshold_value_3`: Compares a word at the address in a0 to 27; if it matches and the byte at the address in a1 is less than 0, it writes 32 to the word at the address in a0 and 1 to the byte at the address in a1, then returns.
- `update_word_on_negative_byte`: Compares a word at the address in a0 to 27; if it is not 27, it branches to a different location, otherwise it checks a byte at the address in a1; if that byte is not negative, it branches to a different location, otherwise it writes 4 to the word at a0 and branches to another location.
- `update_value_on_match`: Compares a word at the address in a0 to 28; if it matches and the byte at the address in a1 is greater than 0, it writes 33 to the word at the address in a0 and branches to another routine; otherwise it branches to another routine.
- `modify_values_on_match`: Compares a word at the address in a0 to 29; if equal, it checks if the byte at the address in a1 is non-zero, and if so, writes the word 33 to the address in a0, writes the byte 255 to the address in a1, and branches to another routine.
- `update_value_on_match_2`: Compares a word at the address in a0 to 29; if it matches and the byte at the address in a1 is greater than 0, it writes 2 to the word at the address in a0 and branches to another routine; otherwise it branches to another routine.
- `update_threshold_value`: Compares a word at a pointer in a0 to 30; if it matches and a byte at a pointer in a1 is not greater than or equal to zero, it writes the word 32 to the address in a0.
- `update_conditional_values`: Compares a 16-bit word at a0 to 31; if it is 31, it checks a byte at a1 and, depending on whether it is greater than zero or less than or equal to zero, writes either 33 or 32 to a0 and clears a byte at a1.
- `increment_and_branch`: Compares a word at the address in a0 to 32; if it is not equal, it branches to another routine; otherwise it increments a byte at the address in a1 and branches to another routine.
- `decrement_byte_on_match`: Compares a 16-bit word at a0 to 33; if it does not match, it branches, otherwise it decrements a byte at a1 and branches.
- `write_status_byte_if_zero`: Checks if a 16-bit word at the address in a0 is zero; if so, it stores the byte 253 at offset 2720 of the address stored in d1.

### Alphacom_Alphapro_101

- `return_constant_43`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_execute_nested_calls`: Attempts to call three consecutive functions via offsets from pointers stored in a global variable and a passed parameter; if any call returns zero, it executes a cleanup routine via an offset from the global variable before returning a failure code, otherwise it returns zero.
- `verify_dual_function_pointers`: Loads two different pointers from a table, then performs two consecutive calls to routines located at an offset of eight bytes from those pointers.
- `execute_indirect_function_calls_2`: Fetches a function pointer from a global variable using an offset and calls that function three times consecutively.
- `return_constant_44`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_45`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_46`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_registers_and_test`: Saves registers to the stack, loads three pointers from the stack into a1, a4, and a2, clears registers d1, d2, and d3, sets d2 to 0, compares a word at the address in a1 to 1, and branches to another routine if not equal or branches to another routine if equal.
- `construct_printer_command_buffer_2`: Copies twelve bytes from a global table into a destination buffer and populates it with specific control sequences and data based on values read from a structure.
- `fill_pattern_memory`: Fills a memory range starting at a4 plus d1 with the byte 32 in a loop, then writes a sequence of six specific bytes at the end of the range, potentially continuing to fill with the byte 32 based on a value from the stack.
- `fill_memory_with_pattern`: Fills a memory range starting at a4 + d1 with the byte 32 in a loop, then writes a sequence of six specific bytes at the end of the range.
- `update_status_and_values`: Compares a 16-bit word at the address in a1 with 27; if not equal, it checks if the byte at a2 is non-zero, otherwise it writes 32 to the address in a1, writes 1 to the address in a2, and branches to another routine.
- `compare_and_update_word`: Compares a 16-bit word at the address in a1 with 27; if not equal, it checks if the byte at a2 is non-zero, otherwise it writes 4 to the address in a1 and branches.
- `modify_word_on_positive_byte`: Compares a word at the address in a1 to 28; if it is not 28, it branches to a different location, otherwise it checks a byte at the address in a2; if that byte is not positive, it branches to a different location, otherwise it writes 33 to the word at a1 and branches to another location.
- `modify_conditional_data`: Compares a 16-bit word at a1 to 29; if it matches and the byte at a2 is non-zero, it writes the word 33 to a1 and the byte 255 to a2.
- `update_value_on_condition`: Compares a 16-bit word at a1 to 29; if it matches and the byte at a2 is greater than zero, it writes the word 2 to a1.
- `modify_word_on_negative_byte`: Compares a word at the address in a1 to 30; if it is not 30, it branches to a different location, otherwise it checks a byte at the address in a2; if that byte is not negative, it branches to a different location, otherwise it writes 32 to the word at a1 and branches to another location.
- `modify_word_and_byte_2`: Compares a word at the address in a1 to 31; if it matches and the byte at the address in a2 is greater than 0, it writes the word 33 to the address in a1.
- `increment_byte_on_match_3`: Compares a word at the address in a1 to 32; if it matches, it increments the byte at the address in a2 and branches to another location.
- `compare_and_decrement_byte`: Compares a word at the address in a1 to 33; if it does not match, it branches to another location, otherwise it decrements a byte at the address in a2 and branches to another location.
- `process_structured_data_2`: If the 16-bit word at a1 is 12, it writes 27 to a location in a1 and a value from the stack to a location in a1 plus 2; if the word at a1 is 66, it writes 13 to a location in a4 plus d1, fills a range with 8, and writes four specific bytes; otherwise, if the word at a1 is 0, it writes 253 to a global variable at offset 2688.
- `store_constant_byte`: Adds a1 to a4 and stores the byte 66 at that location, then branches.
- `write_structured_data`: If a 16-bit word at a1 is 66, it writes a byte 13 and then fills a range with the byte 8 in a loop, followed by four specific bytes; otherwise, if the word at a1 is zero, it writes the byte 253 to an offset of 2688 from a pointer in d2.
- `write_status_byte_if_zero_2`: Checks if a 16-bit word at the address in a1 is zero; if so, it stores the byte 253 at offset 2720 of the address stored in d2.

### Epson_JX-80

- `return_constant_47`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_library_function_calls`: Attempts to call three consecutive functions via offsets from two different pointers retrieved from a global variable and a parameter; if any call returns a non-zero value, it proceeds to the next, otherwise it calls a cleanup routine using an offset from the first pointer and returns.
- `execute_double_callbacks`: Loads a pointer from a global variable into a1, calls a routine at an offset from a pointer in a6, then loads a different pointer into a1 and calls another routine at an offset from a pointer in a6, finally setting d0 to 255 and branching to another routine.
- `execute_indirect_function_calls_3`: Fetches a function pointer from a global variable with an offset of negative 414 and calls it three times using the value from register a6 as an argument.
- `return_constant_48`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_49`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_6`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte at specific offsets, then executes four routines through a global base and a secondary pointer.
- `format_epson_jx80_print_buffer`: Constructs a structured print buffer for an Epson JX-80 printer by calculating offsets and writing escape sequences, character data, and coordinate values into a memory area.
- `restore_registers_and_return_4`: Clears d0 and restores d2 through d4 and a2 through a4 from the stack before returning.
- `setup_and_compare_a2`: Saves registers to the stack, loads a pointer into a2, loads d1, a3, d3, d6, d2, d4, and d5 from the stack or immediate values, and compares a word at the address in a2 to 1; if it is not 1, it branches to a different location, otherwise it branches to another location.
- `format_printer_command_buffer_2`: Copies a sequence of bytes from a table into a destination buffer and populates it with command bytes based on values read from a structure pointed to by a register.
- `generate_epson_jx_80_command_sequence`: Constructs a command buffer by interpreting a control byte and associated data to write specific escape sequences, including printer status checks and character formatting, into a destination buffer.
- `transfer_bytes_from_stack`: Loads a byte from an offset of a7 into d0, decrements it by 1, writes this value to the address in d5, increments the address in a7, and writes the byte from the new address in a7 to the address in d4, then branches to another routine.
- `loop_write_from_table`: Enters a loop that writes a byte from a global table at offset d2 to a memory location calculated from d0 and d1, incrementing the source and destination pointers until a counter in d6 reaches 8.
- `emit_epson_escape_sequences`: Processes a control byte to generate specific escape sequences for an Epson printer, including handling character sets, line spacing, and font selection by writing bytes to a buffer.
- `load_and_check_pointer`: Loads a pointer from the address in a1, compares the 16-bit word at offset 886 of that pointer with 1024, and if they are not equal, it writes 137 to the address in a0 and branches.
- `write_byte_and_branch`: Writes the byte 80 to the address in a0 and then branches.
- `write_bytes_from_table`: Writes up to eight bytes from a table into a destination memory location starting at an offset determined by adding two registers, incrementing the destination pointer and the table index for each byte written.
- `compare_and_update_values`: Compares a 16-bit word at the address in a2 with 32; if not equal, it branches, otherwise it checks a byte at a3, and if that byte is zero, it stores 1 at a3 and 27 at a2 before branching to another routine.
- `check_byte_and_branch`: Checks a byte at a3; if it is negative, it clears the byte at a3, stores 30 at the address in a2, and branches to another routine; otherwise it branches to another routine.
- `set_d0_and_branch`: Sets d0 to 255 and branches.
- `modify_byte_and_word_2`: Compares a 16-bit word at the address in a2 to 33, and if it matches and the byte at the address in a3 is not zero, it writes the byte 255 to the address in a3, writes the word 29 to the address in a2, and branches to another routine.
- `clear_byte_and_set_word`: Checks if the byte at the address in a3 is greater than 0; if so, it clears the byte at the address in a3, writes the word 28 to the address in a2, and branches to another location.
- `branch_to_another_routine_2`: Branches to another routine after setting the first operand of a subsequent operation to two hundred fifty-five.
- `process_conditional_data`: Checks a 16-bit word at a2 against several constants to write specific bytes to a location determined by d1 and d3, or writes a byte to a location determined by d2 and 2720 if the word is 0.
- `modify_pointer_data`: If the 16-bit word at a2 is 4, it writes 27, 106, and a byte from the stack to a location in d1 plus d3; if the word at a2 is 12, it checks a byte from the stack and either modifies it or writes 27, 114, and a value from a table to a location in d1 plus d3; otherwise, if the word at a2 is 0, it writes 253 to a global variable at offset 2688.
- `verify_and_modify_pointer_data`: Compares a 16-bit word at a2 with 12; if not equal, it branches, otherwise it loads a pointer from the stack, compares the byte at that pointer with 39, and if it is 39, stores 30 at that pointer; it then loads the same pointer, compares the byte with 37, and if it is not greater than 37, it branches to another routine before branching to another routine.
- `write_three_bytes_2`: Writes the byte 27 at an offset, the byte 114 at the next offset, and a byte from a global table at the following offset.
- `write_status_byte_if_zero_3`: Checks if a 16-bit word at the address in a2 is zero; if so, it stores the byte 253 at offset 2720 of the address stored in d2.
- `call_library_vector_198_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Epson

- `return_constant_50`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_nested_calls_3`: Loads a value from a global variable, calls a routine using a pointer from that global variable, and if the result is non-zero, calls the same routine again, and if that result is also non-zero, calls it a third time; if any call returns zero, it calls another routine with the global variable as an argument and returns -1, otherwise it returns 0.
- `execute_dual_indirect_calls`: Loads a pointer from a global variable into a1, pushes a6 to the stack, and calls a routine at an offset from a pointer in a global variable.
- `execute_triple_pointer_call_2`: Calls a routine through a pointer found at an offset from a global base, passing a value from a6 to that routine three times.
- `return_constant_51`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_52`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_7`: Checks a condition via a routine call; if the result is zero, it populates a structure with two parameters and a byte at specific offsets, then executes three routines through a global base and one routine through a pointer stored within that structure.
- `manage_printer_buffer_state_4`: Acts as a state machine handler that manages a memory buffer, performing operations such as clearing memory with a loop, writing escape sequences, performing bitwise OR operations on bytes, and calling another routine to handle data transmission or status checks.
- `restore_registers_and_return_5`: Clears d0 and restores d2 through d3 and a2 through a4 from the stack before returning.
- `setup_and_compare_a1`: Saves registers to the stack, loads a pointer into a1, a4, a3, d3, d5, d1, d2, and d4 from the stack or immediate values, and compares a word at the address in a1 to 1; if it is not 1, it branches to a different location, otherwise it branches to another location.
- `construct_printer_command_packet`: Copies a sequence of bytes from a table into a destination buffer and appends command bytes based on values read from a structure pointed to by a register.
- `process_epson_printer_command`: Parses a command byte and updates a structure in a pointer, handling specific character sequences for printer control.
- `write_bytes_from_table_2`: Loads a byte from an offset of a7 into d0, decrements it by 1, writes this value to the address in a1, increments the address in a7, and writes a byte from the new address in a7 to the address in a0, then branches to another routine.
- `copy_byte_from_table_3`: Copies a byte from a table at the address in a0 using an offset derived from d0 to a location calculated from a2 and a4, increments d3 and d5, and branches to another routine if d5 is less than 8; otherwise it branches to another routine.
- `process_printer_control_codes_2`: Checks a word at the address in a1 and updates a buffer with specific escape sequences, padding bytes, or values from a global table based on various conditions.
- `check_and_set_printer_status`: Loads a pointer from a register, compares a word at an offset of one thousand twenty-four from that pointer to one thousand twenty-eight, and if they match, writes the byte eighty-nine to the address stored in another register before branching to another routine; otherwise, it branches to another routine.
- `write_status_byte_and_branch`: Loads a pointer from a register into a0, writes the value eighty to the address pointed to by a0, and branches to another routine.
- `copy_byte_from_table_loop`: Copies a byte from a table at the address in a0 using an offset derived from d0 to a location calculated from a2 and a4, increments d3 and d5, and branches to the start of this routine if d5 is less than 8; otherwise it branches to another routine.
- `check_and_set_byte_2`: Compares a 16-bit word at a1 with 32; if not equal, it checks if the byte at a3 is non-zero, otherwise it writes 1 to a3, writes 27 to a1, and branches.
- `clear_and_write_word`: Checks if the byte at a3 is less than 0; if so, it clears the byte at a3, writes 30 to the address in a1, and branches.
- `set_d0_and_branch_alt`: Sets d0 to 255 and then branches.
- `modify_pointer_data_via_a1`: Compares a 16-bit word at the address in a1 to 33, and if it matches and the byte at the address in a3 is not zero, it writes the byte 255 to the address in a3, writes the word 29 to the address in a1, and branches to another routine.
- `clear_byte_and_set_word_alt`: Checks if the byte at the address in a3 is greater than 0; if so, it clears the byte at the address in a3, writes the word 28 to the address in a1, and branches to another location.
- `return_constant_value`: Sets the d0 register to two hundred fifty-five and branches to another routine.
- `process_conditional_data_alt`: Checks a 16-bit word at a1 against several constants to write specific bytes to a location determined by a4 and d3, or writes 253 to a location determined by d1 and offset 2720.
- `write_status_byte_if_zero_4`: Tests a 16-bit word at the address in a1; if it is zero, it writes the byte 253 to offset 2720 of a structure pointed to by d1.
- `call_library_vector_198_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### HP_LaserJet_PLUS

- `return_constant_53`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_chain_5`: Loads a pointer from a global variable into a1, saves a6 to the stack, and calls a routine at an offset from a pointer in a6.
- `execute_printer_function_twice`: Loads a pointer from a global variable into a1, then performs two consecutive calls to a routine located at an offset from a pointer stored in a global variable, using the stack to preserve a6, before returning with a value of negative one in d0.
- `execute_indirect_function_calls_4`: Fetches a function pointer from a table using an offset and calls that function three times, passing the value from a register as an argument to each call.
- `return_constant_54`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_9`: Calls a routine via a pointer in a global variable and, if it returns zero, populates a structure with two parameters and a byte value, then executes three routines through a global base and one routine via a pointer retrieved from the structure.
- `construct_hp_laserjet_escape_sequence`: Copies bytes from a table into a buffer and modifies specific offsets based on hardware status flags, then calls format_hp_laserjet_escape_sequences, process_printer_control_sequences, or format_numeric_string_4.
- `configure_hp_laserjet_parameters`: Copies a sequence of bytes from a table into a buffer and modifies specific bytes in that buffer based on hardware identification values.
- `process_printer_control_sequences_2`: Updates a buffer with values from a table using a loop, then performs a series of conditional checks on a byte and a secondary value to modify them or update a global variable.
- `copy_table_elements_2`: Copies elements from a table into a destination buffer using a loop, then calls another routine twice with specific calculated arguments.
- `compare_and_update_values_3`: Compares a word at the address in a2 with 27; if not equal, it branches to another location.
- `update_escape_sequence_value`: Compares a word at the address in a2 to 27; if it matches and the byte at the address in a3 is not positive, it writes the word 4 to the address in a2, otherwise it returns.
- `update_word_on_match_2`: Compares a word at the address in a2 to 28; if it matches and the byte at the address in a3 is greater than 0, it writes the word 33 to the address in a2 and branches to another routine.
- `set_conditional_values`: Compares a word at a pointer in a2 to 29; if it matches and a byte at a pointer in a3 is zero, it writes the word 33 to the address in a2 and the byte 255 to the address in a3.
- `update_printer_status`: Compares a word at the address in a2 to 29; if it matches, it checks if the byte at the address in a3 is greater than zero, and if so, writes the word 2 to the address in a2.
- `update_word_on_condition_4`: Compares a word at the address in a2 to 30; if it matches and the byte at the address in a3 is less than 0, it writes the word 32 to the address in a2 and branches to another routine.
- `clamp_value_within_range`: Compares a word at the address in a2 to 31; if it is not 31, it branches to another routine.
- `compare_and_increment`: Compares a word at the address in a2 to 32; if it is not equal, it branches to another location, otherwise it increments a byte at the address in a3 and branches to another location.
- `decrement_byte_on_match_3`: Compares a word at the address in a2 to 33; if it matches, it decrements a byte at the address in a3 and branches to another routine, otherwise it returns.
- `check_and_update_status_byte`: Compares a word at the address in a2 to 64 and, if equal, checks if a byte at the address pointed to by offset 28 of the structure in a6 is non-zero; if that byte is zero, it writes the byte from a global variable to the address pointed to by offset 28 of the structure in a6 and then branches to another routine.
- `decrement_and_update_buffer`: Decrements a byte at a pointer stored in a structure in a6, updates a global variable with the new value, and conditionally updates a byte at the next address in that structure using a value from d4.
- `increment_and_update_byte`: Increments a longword stored at offset 28 of a structure in a6, reads the byte at the resulting address, and writes that byte as a word to the address held in d4, then branches to another routine.
- `copy_data_and_call_threshold`: Copies 11 bytes from a global table to a destination pointed to by a pointer plus an offset in a6, then calls another routine twice using values derived from a pointer in a6.
- `compare_word_and_branch_5`: Compares a word at the address in a2 to 57; if the values are not equal, it branches to a specific offset, otherwise it branches to another offset.
- `copy_buffer_and_execute_callbacks`: Copies 11 bytes from a global table to a destination buffer and then calls two separate routines using values stored in registers.
- `write_status_byte_if_zero_5`: Tests a 16-bit word at the address in a2; if it is zero, it writes the byte 253 to offset 2720 of a structure pointed to by d2.
- `format_numeric_string_5`: Writes a byte to a location calculated from a pointer in a3 and a parameter in a2 based on the value of a parameter in a1, then calls another routine and writes the result plus 48 to the next byte location.
- `convert_byte_to_string`: Compares the byte in d0 with 99; if it is not less than or equal to 99, it moves the address in d3 to a2, increments d3, adds the value in d4 to a2, writes the byte 49 to the address in a2, subtracts 100 from d0, and branches to another location.
- `format_numeric_string_3`: Writes 48 to a location in d4 plus d3, writes either 48 or the result of dividing a byte in d0 by 10 plus 48 to the next location, calls another routine, and writes the result of that routine plus 48 to the following location.
- `write_formatted_characters`: Stores the byte 48 at an address calculated from a3 and a4, calls another routine, and then stores the result of that routine incremented by 48 at the next address.
- `call_sequential_routines`: Pushes two values onto the stack, calls a routine at an offset from a global base, then calls another routine at a different offset from that same global base.
- `perform_32bit_division_2`: Performs a 32-bit division using a loop that rotates bits and handles remainders and quotients.
- `rotate_and_threshold_32bit_3`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_two_32bit_integers`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit partial components and returns the result.
- `negate_result_on_flag_3`: Calls another routine and negates the returned value if the result in a0 is nonzero.
- `dispatch_jump_table`: Loads several values from the stack, compares a byte to five, and performs a jump to an address calculated from a table based on that byte.
- `return_zero_and_restore_registers`: Clears the d0 register and restores d2, d3, a2, a3, and a4 from the stack before returning.
- `call_library_vector_198_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `initialize_structure_with_word`: Saves a2 and a3 to the stack, loads a word from an offset of the stack into d0, sets a1 to 4, sets a3 to 47, sets a2 to 46, and compares d0 with 768.
- `compare_word_range_2`: Compares the 16-bit word in d0 with 256 and branches to another routine if it is less than or equal, or branches to another routine if it is higher, otherwise it branches to another routine.
- `compare_and_branch_3`: Compares the register d0 with 512; if not equal, it branches, otherwise it branches to another routine.
- `compare_word_and_branch_3`: Compares a word in d0 to 1024 and branches to different locations depending on the result.
- `initialize_structure_values_2`: Writes several 16-bit and 32-bit values to a structure pointed to by a1 and updates a global variable.
- `setup_structure_parameters`: Writes specific 32-bit and 16-bit values to a structure pointed to by a1 and performs a relative branch.
- `setup_structure_parameters_alt`: Writes specific 32-bit and 16-bit values to a structure pointed to by a1, writes a byte to a2, and performs a relative branch.
- `initialize_structure_fields_2`: Writes 2424 and 3214 to offsets 26 and 30 of a structure pointed to by a1, writes 300 to offset 36 of that structure and offset 34 of the structure pointed to by the first structure, sets a global variable to 51, and writes 48 to two other memory locations.

### 12

- `return_constant_55`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_data_via_pointer`: Copies a block of data from a source address to a destination address using a loop, incrementing the pointers and a counter until the transfer is complete.

### 15

- `return_constant_56`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_57`: Returns the constant an address in d0 to the caller without any other effect.
- `process_font_glyph_data`: Reads a sequence of bytes from a font data table and writes them to a destination buffer in memory, using a loop to copy each byte until a termination condition is met.

### 12

- `return_constant_58`: Returns the constant an address in d0 to the caller without any other effect.
- `process_font_glyph_data_2`: Iterates through a data block to extract bitmask patterns for a font character, performing bitwise operations to prepare pixel data for display.

### 20

- `return_constant_59`: Returns the constant an address in d0 to the caller without any other effect.
- `process_font_glyph_data_3`: Reads a sequence of bytes from a source pointer and writes them to a destination memory location, incrementing both pointers and a counter until a specific termination value is encountered.

### 11

- `return_constant_60`: Returns the constant an address in d0 to the caller without any other effect.

### Balance of Power - The 1990 Edition (1990)(Mindscape)[cr Pete].adf:CLI

- `parse_command_line_arguments`: Parses a command string by iterating through characters to extract space-delimited or quoted tokens, handling escape sequences for newlines and escape characters, and stores the resulting argument list into a global structure.
- `invoke_two_library_functions`: Loads a pointer from an offset of 92 in a6 into a0, calls a function at a negative offset from a6, repeats this process with a different negative offset from a6, and then returns.
- `call_and_store_result`: Calls a routine using a pointer relative to the program counter, stores the returned value in a global variable, and returns if the result is not zero.
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
- `manage_linked_list_node`: As library code, it checks a value at an offset in a structure; if zero, it inserts a node into a linked list and calls a function pointer, otherwise it sets bit 3 of a byte at offset 14 of the structure.
- `set_status_bit`: library code sets bit 3 of a byte at offset 14 of a structure in a6 and returns.
- `increment_and_check_counter`: library code checks if bit 3 of a byte at offset 14 of a structure in a6 is clear; if so, it increments a 16-bit word at offset 32 of that structure, and if the new value is 1, it calls another routine using a global variable as an argument.
- `return_negative_one`: library code returns a 32-bit value of negative one.
- `decrement_counter_and_call_2`: Library code that decrements the 16-bit counter at offset 32 of the structure at a6.
- `mask_and_call_library_routine`: Library code that masks bit 7 of d0, calls another routine, and sets bit 1 of the condition code register before returning.
- `calculate_bitfield_difference`: As library code, it checks if a byte in d0 is non-zero and non-negative, then enters a loop to update a bitfield and running difference based on bitwise operations.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `process_and_mask_d0`: Copies the value from d0 to d7, tests the byte in d7, calls a routine through a pointer, sets bit 0 of d0, clears bit 3 of d0, and branches to another routine.
- `call_jump_table_entries`: Calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_d1_register`: As library code, it clears the longword in d1 and branches to another location.
- `execute_conditional_math_calls_2`: Library code that calls a routine through a pointer derived from a value in a register, then performs a series of conditional calls to other routines at specific offsets from that same pointer based on the results of previous calls and the value of a register.
- `execute_conditional_math_calls`: As library code, it calls a routine at an offset from a pointer in a5.
- `execute_conditional_library_calls_2`: Library code that performs a series of calls to functions at offsets relative to a base address in a5, with the specific sequence of calls depending on the values of d6 and d7, and returns a value with bits cleared.
- `call_library_subroutines`: The library code moves a longword from d7 to d1, calls a routine at an offset of minus 48 from a5, and if the result is not negative, tests d4 and branches based on its value.
- `call_and_restore_d2`: Copies d2 to d0, calls a routine at an offset of minus 60 from a5, copies the result in d0 back to d2, and branches to another routine.
- `load_constant_on_zero_d4`: Copies d2 to d0, tests d4, and if d4 is zero, loads the constant a fixed address into d2 before branching to another routine; otherwise it branches to another routine.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `perform_bit_shifting_loop`: As library code, it calculates a value based on d7 and performs a series of checks.
- `initialize_register_values`: Library code that performs a logical shift right on d7, sets d6 to zero, sets d5 to a constant, loads a constant into a0, and sets d1 and d2 to specific small values before branching.
- `execute_loop_with_branching`: The library code executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_word_and_branch`: As library code, it pushes the word 65534 onto the stack and branches to another routine.
- `push_word_and_branch_alt`: As library code, it pushes the word 65535 onto the stack and branches to another routine.
- `push_constant_one`: Library code that pushes the constant one onto the stack and branches to another routine.
- `compare_and_adjust_stack`: As library code, it compares register d7 to 184, branches if it is higher, otherwise it increments the stack pointer by 2, tests register d7, and returns.
- `perform_math_table_lookup`: Library code that performs complex arithmetic and uses a loop to iterate through a table of values to calculate an index.
- `perform_complex_math`: As library code, it performs arithmetic operations using values from a pointer in a0 and a global variable within a loop, while also calling other routines to load addresses and branch.
- `perform_math_loop_2`: As library code, it executes a loop that performs arithmetic operations using values from a pointer in a0 and a global variable, adjusting r.
- `perform_math_loop`: As library code, it executes a loop that performs arithmetic operations using values from a pointer in a0 and a global variable, adjusting registers based on a counter.
- `perform_iterative_math`: Library code that executes a loop performing arithmetic operations on registers and values from a pointer in a0, and contains conditional branches to call other routines based on the state of a stack value and register d7.
- `set_d7_and_branch`: As library code, it loads the constant an address into register d7 and branches to another routine.
- `recursive_call_wrapper`: Library code that branches to a subroutine, moves the result to d7, tests d7, restores registers from the stack, and returns.
- `call_subroutine_and_restore`: As library code, it pushes d5 onto the stack, calls a subroutine, stores the value from d6 at an offset of the stack, restores d6 from the stack, and branches to another location.
- `compare_and_branch_d6`: The library code moves d6 to d4, branches if d4 is negative, otherwise it compares d6 to a constant and branches if it is greater than that constant, or if it is equal to a different constant, it moves zero into d6 and returns.
- `shift_and_jump`: As library code, it arithmetic shifts register d4 right by 8 bits, increments d4, and if d4 is zero, it performs an absolute jump.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_callbacks`: Library code checks a byte, and if it is non-zero, performs a series of conditional calls to other routines based on the results of those calls.
- `rotate_and_branch`: As library code, it loads an address into register d7, rotates d7 right by 1 bit, and branches to another routine.
- `load_and_branch`: As library code, it loads an address into register d7 and branches to another routine.
- `test_stack_byte_and_branch`: As library code, it pops a word from the stack into register d6, tests if d6 is positive, otherwise it moves 0 into register d7 and branches to another routine.
- `set_status_flags_and_restore_registers`: Sets bit 1 of the condition code register and restores six data registers and the address register A0 from the stack before returning.
- `load_constant_and_return`: Loads a constant value into a data register, adjusts the stack pointer by an offset of 30, and returns to the caller.
- `execute_variable_math_sequence`: As library code, it checks if d7 is zero and returns if so.
- `perform_multiple_calls`: Library code that calls another routine four times and then returns.
- `loop_and_add_table_values`: The library code increments d2, moves d5 to d3 and d6 to d4, performs arithmetic shifts on d3 and d4, tests d7 and branches if it is negative, otherwise it adds d4 to d5, adds d3 to d6, subtracts a value from a table pointed to by a0 and increments a0, and repeats this via a loop controlled by d1.
- `subtract_table_from_accumulator`: As library code, it subtracts d4 from d5, subtracts d3 from d6, adds a value from a table pointed to by a0 to d7, and repeats this in a loop while decrementing d1 until it reaches zero, then returns.
- `test_and_branch_d7`: As library code, it tests the byte in d7; if it is zero, it branches to another location.
- `set_d7_and_branch_2`: Sets register d7 to negative one and branches to an offset.
- `perform_complex_arithmetic`: Executes a loop involving bit-shifting and conditional jumps, and calls other routines based on the results of arithmetic operations on registers.
- `complex_loop_with_branching`: The library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `process_d7_bitmask`: As library code, it tests a byte in d7; if it is positive, it branches; otherwise, it masks d7 with 127, calls another routine, and sets the overflow flag.
- `process_conditional_data_2`: Library code calls two routines, then performs a series of conditional checks on a register and uses the results to either return or call other routines with modified values.
- `transform_register_value`: Performs a series of arithmetic and bitwise operations on a value in a register, including a conditional exit, a bitwise XOR, a subtraction, a word swap, and a 7-bit left rotation, before returning.
- `manipulate_d7_bits`: Swaps the high and low bytes of d7, performs several bitwise operations and shifts, and branches based on the overflow flag.
- `test_and_rotate_d7`: The library code branches if a condition is met, compares d7 to two different constants and branches if they match, otherwise it adds a constant to d7, performs a rotate left on d7, tests d7 and branches based on the result, or branches to a final location.
- `test_and_branch_on_word`: Performs a bitwise AND on a word in d7 with a mask, then branches to another routine if the result is zero, otherwise it clears d7 and branches to another routine.
- `compare_and_shift`: As library code, it compares register d7 to 254, branches if not equal, otherwise it performs two logical shifts right on d7 and branches to another routine if the result is not zero, otherwise it branches to another routine.
- `shift_and_set_cc`: Shifts d7 left by 8 bits, clears the upper byte of d7, rotates the carry bit into the lowest bit of d7, sets the 2nd bit of the condition code register, and branches to another routine.
- `clear_d7_and_branch`: Clears d7 and branches to another routine.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `load_address_and_branch_8`: Library code that loads a constant address into a0 and then branches to address zero.
- `load_address_to_a0`: Loads the address of a global variable into register a0 and then branches to another routine.
- `load_address_pointer`: As library code, it loads the address of a constant into a0 and branches immediately.
- `load_null_address`: Library code that loads a zero value into register a0 and then branches to another routine.
- `load_address_and_branch_12`: As library code, it loads the address of a constant into a0 and branches to another routine.
- `load_address_and_branch_13`: As library code, it loads a longword into a0 and branches to the next instruction.
- `load_address_and_branch_14`: Loads the address of a global variable into a0 and branches to another routine.
- `store_register_via_pointer`: The library code loads a pointer from a memory location into a0, saves registers d3 through d7 and d1 to the stack, calls the routine at the address in a0 with d0 as a parameter, restores d7 from the stack and stores it into the memory location pointed to by the restored a0, then restores d6 to d0 and the remaining registers before returning.
- `load_address_pointer_alt`: As library code, it loads a different address into a0 and branches immediately.
- `load_null_address_and_branch`: Library code that loads a zero value into a0 and then branches to address zero.
- `load_address_pointer_final`: As library code, it loads a third address into a0 and branches immediately.
- `load_address_and_branch_11`: Library code that loads a value from a memory location into register a0 and then branches to another routine.
- `nullify_address_register`: Sets the address register a0 to zero and then branches to the next instruction.
- `load_address_and_branch_10`: As library code, it loads the address at a constant location into a0 and then branches to another location.
- `call_indirect_return_d0_2`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0.
- `load_address_and_branch_9`: Loads a longword address into a0 and immediately branches to the next instruction.
- `load_pointer_to_address_register`: Loads a pointer from an offset of 20 in the stack into an address register and then branches to the next instruction.
- `load_offset_a0_and_branch`: Loads a constant into a0 and branches to another routine.
- `dispatch_indirect_routine`: Loads a pointer from a global variable into a register, adds a negative offset to it, and calls the resulting address.
- `load_address_and_branch`: Loads the address of a global variable into a0 and performs a short branch to another routine.
- `load_address_and_branch_2`: Loads a longword from a specific memory location into a register and performs a short branch to another routine.
- `dispatch_indirect_routine_2`: Loads the address of a global variable into a register and branches to another routine that executes a call at an offset from that address.
- `access_indirect_function_pointer`: Loads the address of a global variable into a register and branches to another routine that calls a function at an offset from that address.
- `load_address_and_branch_3`: Loads a longword value from a specific memory location into a register and performs a short branch to another routine.
- `load_address_and_branch_4`: As library code, it loads a long value into register a0 and branches to another routine.
- `load_address_and_branch_5`: As library code, it loads a different long value into register a0 and branches to another routine.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in d6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_constant_61`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_status`: If a value at offset 8 of a structure in a6 is zero, it calls a routine using a pointer from offset 22, updates a linked list by setting a pointer at offset 22 of a target structure to the value in a6, and sets the next pointer of that target structure to the value in a6; otherwise, it sets bit 3 of a byte at offset 14 of the structure in a6 and returns 0.
- `set_icon_bit`: Sets bit 3 of a byte at offset 14 of a structure pointed to by a6 and returns 0.
- `return_constant_62`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_chain_4`: Copies a value from a0 to offset 2a of a structure in d0 and a6 to offset 22 of the same structure, calls a routine using a pointer from offset -228 of a6, and stores the result in offset 26 of the structure; if the result is zero, it calls another routine, otherwise it stores a6 and the result in two global variables.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_bitwise_or`: Performs a bitwise OR of d0 with zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `traverse_icon_chain_2`: Saves two registers to the stack and calls another routine to manage an icon chain, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `invoke_icon_chain_handler`: Saves two registers onto the stack and calls a routine to process an icon chain, then restores the stack and returns.
- `traverse_icon_chain_3`: Saves two registers onto the stack and calls another routine to manage an icon chain, then returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_other_routine_2`: As library code, this routine calls another routine and then returns.
- `call_accumulate_and_copy_data`: Saves two registers to the stack, calls accumulate_and_copy_data, and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_register_d0`: Performs a logical OR operation on register d0 with zero.
- `update_icon_structure`: As library code, it calls another routine and, if the result is non-zero, calls a second routine and then copies 43 bytes from a source to a destination before updating several fields in a structure and potentially setting a pointer in another structure.
- `copy_and_update_structure`: Copies 43 bytes from a source starting at offset -74 of a6 to a destination starting at offset 96 of a2 using a loop, then copies eight specific values from offsets -30 to -4 of a6 into various offsets of a2, and if a value at offset 76 of a2 is non-zero, it writes a2 into a location at offset 424 of the structure pointed to by that value.
- `return_zero_and_unlock`: This library routine returns zero.
- `copy_icon_data_to_stack`: As library code, it copies 43 bytes from a structure at an offset of 96 from the third argument into a local stack buffer, copies several fields from that structure into local stack slots, and calls another routine with the first two arguments and a pointer to the stack header.
- `update_icon_attribute_structure`: Validates a structure by checking magic values and flags, then recursively populates various attribute fields, including linked list nodes and potential sub-structures, by calling other routines.
- `populate_icon_attribute_structure`: Checks a structure for specific magic values and flags, then recursively calls another routine to populate various attribute fields at offsets 18, 22, 25, 27, and 35.
- `traverse_and_update_structure`: Library code traverses a linked data structure, updating values at specific offsets within the structure and executing a series of callback routines based on the contents of the structure and the results of those calls.
- `process_callback_list`: The library code executes a loop that calls two different routines, updates a value at an offset of minus 4 from a pointer in a6, and continues as long as the value at that offset is greater than 4.
- `clear_d6_and_branch`: Sets d6 to 0 and then branches to another location.
- `unlock_and_return`: This library routine restores registers from the stack, unlocks the register in a6, and returns.
- `traverse_icon_hierarchy_recursively`: Iterates through a hierarchical icon structure by recursively processing child elements, including a list of pointers, and updates a value at offset 26 of the structure.
- `traverse_icon_hierarchy`: Iterates through a hierarchical structure of icon pointers, performing recursive calls to process child elements and updating a value at offset 26 of a structure.
- `traverse_icon_lists`: Iterates through multiple lists of pointers, calling other routines for each element and updating a value at offset 26 of a structure.
- `process_icon_list_recursively_2`: Increments a pointer in a4 until it reaches a zero value, calls a routine using a pointer from a3, and if that returns non-zero, it iterates through a list of pointers starting at offset 54 of a2, calling itself recursively for each, and then calls itself once more if a pointer at offset 70 of a2 is non-zero; finally, it calls two more routines and stores a value from a6 into offset 26 of a2.
- `process_icon_list_recursively`: Library code that iterates through a list of pointers, calling functions and updating a local stack variable, then executes a callback via a pointer or table.
- `traverse_icon_chain`: Iterates through a linked list by retrieving a pointer from the stack, then calls two library functions and a register restoration routine.
- `restore_registers_and_return_6`: This library routine restores registers d2 and d3 from the stack and returns.
- `incremental_offset_recurse`: Calls another routine with the same first argument and the second argument increased by a fixed offset, then returns.
- `execute_conditional_logic`: Library code that calls a function pointer, then performs a series of conditional calls using values from a structure, and finally updates a field in a structure with a value from another routine.
- `call_subroutine_with_stack_args`: As library code, it pushes several registers and a pointer from a2 onto the stack, calls another routine, and then either moves a2 into d0 or returns based on the result in d0.
- `return_zero_minimal_stub`: This library routine returns zero.
- `update_table_entry`: Decrements a 16-bit counter through a pointer and, if the counter is not zero, stores two 32-bit values into a table at an offset determined by the counter.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from registers and the stack into consecutive slots of that table.
- `process_icon_chain`: As library code, it calls a routine with specific constants, and if the result is non-zero, it performs a series of conditional calls to other routines, including a function pointer and a library function, before returning a value.
- `move_and_branch_2`: Moves the contents of register d6 into d0 and then performs a short branch.
- `call_lib_func_and_return_zero`: Invokes a function pointer retrieved from a global variable associated with library index -210.
- `incremental_offset_recurse_2`: Calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `traverse_pointer_list`: This library code iterates through a list of pointers, calling another routine for each element, and returns a pointer to a specific character in a buffer if a termination condition is met.
- `restore_registers_and_return_7`: The library code clears register d0 and restores registers d2, d3, a2, and a3 from the stack before returning.
- `process_indexed_values_2`: Library code that executes a loop to fetch values via another routine, updates a register based on a conditional check, and returns.
- `process_indexed_values`: This library code executes a loop that calls another routine to fetch values, updates a register based on a conditional check, and returns either zero or one.
- `branch_on_d4_value`: Tests the value of a 32-bit register and either branches to another routine or sets a 32-bit register to the value of that register incremented by one and branches elsewhere.
- `clear_and_branch`: Sets register d2 to zero and then performs a short branch.
- `restore_registers_and_return_8`: The library code clears register d0 and restores registers d2, d3, d4, d5, d6 from the stack before returning.
- `accumulate_and_copy_data`: Library code that clears a byte at offset 30 of a structure, performs a loop to accumulate a value by multiplying a register by ten and adding bytes from a pointer, and calls other routines to copy data and update the structure.
- `process_stack_and_branch`: Library code that increments a value in a2, pushes two pointers onto the stack, calls another routine, and then branches based on the result in d0.
- `clear_d5_and_branch`: Clears register d5 and branches to another location.
- `calculate_accumulated_value`: This library code performs a loop that accumulates a value by multiplying a register by ten and adding a byte fetched from a pointer, then calls several other routines based on the result.
- `compare_and_calculate_offset`: Compares a value from the stack with a constant, then either sets d0 to a specific value or calculates the difference between the value and a constant before branching.
- `return_negative_one_2`: The library code sets register d0 to negative one and returns.
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
- `call_library_and_update_pointer`: The library code calls a library function using a base address from a global variable, and if the result is non-zero, it stores the value 10 at an offset of 14 from the result, stores 10 at the memory location pointed to by a parameter, and calls another routine using a pointer to the next location.
- `return_zero_with_restore`: Clears d0 and restores d2, d3, and a2 from the stack before returning.
- `compare_library_result`: Calls a library function at offset -42 of a library base, and if the returned value does not match the fourth parameter, it returns 0; otherwise it returns the result of the library call.
- `move_d1_to_d0_and_return`: Moves the contents of d1 into d0, restores d2 and d3 from the stack, and returns.
- `validate_library_return`: Calls a library function at offset -48 of a library base, and if the returned value does not match the fourth parameter, it returns 0; otherwise it returns the result of the library call.
- `copy_d1_to_d0_and_restore`: Copies the value from d1 to d0, restores d2 and d3 from the stack, and returns.
- `divide_32bit_integers`: The library code performs 32-bit division by rotating a 32-bit value left by one bit for sixteen iterations, subtracting a threshold from the high half if the value meets or exceeds it, and calls another routine to return the result.
- `rotate_and_threshold_32bit_value`: Rotates a 32-bit value left by one bit for sixteen iterations using the carry flag to transfer bits between the high and low halves.
- `multiply_32_bit_integers_6`: Computes the 32-bit product of two 32-bit integers by summing the products of their 16-bit partial components.
- `invoke_table_entry_4`: The library code loads a library base from a global variable and calls the library function at offset minus 522 of that base.
- `store_byte_and_increment_2`: The library code stores a byte from register d0 at the address in a3 and then increments a3.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_and_decrement_pointer`: This library code loads a pointer from an offset of 4 on the stack and a value from an offset of 8 on the stack, then compares the value at the pointer with the value from the stack.
- `clear_d0_and_branch_4`: Library code that clears register d0 and performs a relative branch.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `skip_zero_byte`: The library code checks a byte at the address in a0; if it is not zero, it decrements a0 and branches to a different location, otherwise it continues.
- `compare_byte_with_table`: Compares a byte from a table at an offset determined by a value from the stack against a byte from a pointer in a1, then sets d0 to zero if they match and the byte is zero, otherwise it returns.
- `set_conditional_d0`: Sets d0 to one if a condition is met, otherwise it sets d0 to 255 and returns.
- `test_byte_and_decrement`: Tests a byte at the address in a0, and if it is not zero, decrements a 16-bit value in d0 and branches elsewhere; otherwise it branches to a different location.
- `decrement_and_copy`: Decrements the address in a0 by one, calls another routine, and then returns.
- `copy_bytes_with_counters`: The library code copies bytes from the address in a1 to the address in a0 using a loop that decrements a counter in d0, and if the counter reaches zero, it clears a byte at the address in a0 and calls another routine.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `update_linked_list_node`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the value at that location by four, clears the following word, and stores the original pointer value into the word after that.
- `conditional_library_branching`: If the third parameter is non-zero, it calls another routine with a constant, and if that returns non-zero, it calls a third routine, then a fourth, then a fifth, and finally a sixth routine using a value from offset 10 of the third parameter; otherwise it returns 1.
- `traverse_and_call_linked_list`: The library code executes a loop that calls a routine with three parameters, and if the result is non-zero, it calls another routine using a pointer from an offset of 8 in the third parameter and a value from an offset of 7 in the third parameter, then updates the third parameter using a pointer at an offset of 12.
- `return_zero_and_branch`: Sets d0 to 1 and then branches to another location.
- `return_zero_stub`: This library routine returns zero.
- `allocate_and_initialize_structure`: Calls several routines to manage memory and data, and stores a longword at an offset of 10 from a pointer retrieved from the stack.
- `return_negative_one_stub`: This library routine returns negative one.
- `traverse_linked_data_structure`: Library code that loops through a linked list, calling another routine to retrieve a value, updating a longword at offset 12 of a structure, and using a value from offset 7 of a structure to calculate a size for a subsequent call.
- `process_linked_data_loop`: Executes a nested loop that calls other routines and stores values at offsets of 8 and 12 from a pointer retrieved from the stack.
- `return_longword_and_branch`: Moves the longword from a3 into d0 and then branches to another location.
- `return_negative_one_extended_stub`: This library routine returns negative one.

### translator.library

- `initialize_library_structure`: Calls a function through a pointer at an offset from a global variable, and if the result is not zero, stores a global variable and a register value into a structure and calls another routine before returning a specific value.
- `clear_flag_and_increment_counter`: Increments a 16-bit counter in the structure it is given and clears bit 5 of that structure's flag byte.
- `decrement_counter_call_on_zero`: Decrements a 16-bit counter in the structure it is given.
- `update_node_status`: Saves registers to the stack, loads a library base, checks a bit at offset 32 of a structure, and if set, sets bit 5 at offset 14 of that structure before branching.
- `unlink_node_and_call_library`: Removes the node from its doubly linked list, then calls a function through the Amiga library base and returns a value stored in the node.
- `return_constant_63`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_text_buffer`: Library code that fills a stack area with space characters using a loop and performs conditional calls based on character matches in a buffer.
- `copy_word_register`: This library code copies the contents of register d0 to register d1 and then branches to another location.
- `clear_byte_and_return_word`: Calls one unnamed library routine, clears the byte just before the pointer held in the address register, and returns the low 16-bit value of the data register.
- `execute_conditional_library_calls_5`: Library code that loads a pointer from a structure, calls two routines, and branches based on the result of the second call.
- `execute_conditional_library_calls_4`: As library code, it loads a pointer from a structure, calls two different routines, and then branches based on the value in d7.
- `execute_conditional_calls`: This library code calls three different routines sequentially, branching or returning based on the results of those calls, and restores two registers from the stack before returning.
- `check_byte_value`: Loads two pointers from the stack, reads a byte from the address in a6, and compares it against two different constants before branching.
- `stub_immediate_return`: The decompiled body contains only a return, so the routine does no visible work in the library.
- `match_bracketed_tag_text`: Skips forward to an opening bracket character, then compares the characters that follow against a reference string until a closing bracket.
- `check_table_and_compare`: Decrements a 16-bit register, uses the result to fetch a byte from a table in a6, uses that byte to fetch a word from a table in a4, and compares the byte from a6 with a byte at the address in a2.
- `set_condition_code`: Sets the carry flag and returns.
- `dispatch_special_character_one`: Sets d7 to one, calls another routine, and if the result is not zero, clears the carry flag and returns; otherwise it returns.
- `match_string_and_call_routine`: Compares a string in a1 with a string in a6 in a loop, calling another routine if a specific bit is set in a word fetched from a table, and returns if an equals sign is encountered.
- `dispatch_special_character_two`: Sets d7 to 255, calls another routine, and if the result is not zero, clears the carry flag and returns; otherwise it returns.
- `compare_byte_against_constants`: Increments and decrements a 16-bit register, fetches a byte from a table in a6 using that register, and compares that byte against two constant values before branching.
- `calculate_text_offset`: Subtracts a value from a longword in a1 from a value at offset 8 of a structure in a5, adds d7, compares the result to d2, and if the result is greater than or equal to d2, it performs a bitwise AND on d6, adds the result to a6, and branches to another routine; otherwise it pushes d7 to the stack and branches to another routine.
- `copy_counted_bytes_with_flag`: Copies a counted run of bytes from the source to the destination buffer, then sets the continuation flag bit and clears it again if the next source byte is a backtick.
- `truncate_text_at_break`: Walks backwards from the current position through a text buffer until it reaches a break character, then writes a zero byte there to end the string.
- `dispatch_on_special_character`: Searches a short fixed table of special characters for the byte in the data register and calls the handler at the matching position.
- `check_and_load_byte`: Tests d7 and, if it is not negative, loads a byte from the address in a2 into d1 and branches to another routine; otherwise it branches to another routine.

The function list continues in: g1330-recovery-functions-2.md.
