# Game 1446 (balance of power - the 1990 edition (1989)(mindscape)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01446). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916908 top-level bytes in 76 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 54 of 54 units identical (bytes and relocations), 59 raw-word lines |
| Functions named and described | pass, 2261 of 2261 named (278 author symbols), 2261 of 2261 described (427 by rule, 1711 by the local model) |
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
| Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf:stack | 2 | 117 | 2 |
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
| Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf:CLI | 9 | 362 | 18 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 15 | 1414 | 108 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2476 | 170 |
| mathieeedoubbas.library | 3 | 632 | 60 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 13 | 830 | 53 |
| Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf:bop | 3 | 5064 | 305 |
| Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1700 | 37 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 150 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf bytes 470528-471040 (code never loaded by a reference run) | 1 | 0 | 0 |
| Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf bytes 603136-603648 (code never loaded by a reference run) | 1 | 28 | 1 |
| Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf bytes 684544-709632 (code never loaded by a reference run) | 1 | 0 | 0 |
| Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf bytes 714240-715264 (code never loaded by a reference run) | 1 | 122 | 1 |
| Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf bytes 867840-868864 (code never loaded by a reference run) | 1 | 0 | 0 |
| SavedGame (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: file devs/printers/Okidata_92 (header 5476 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1478 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 93 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 20 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: bytes 470528-471040 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: bytes 603136-603648 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: bytes 684544-709632 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: bytes 714240-715264 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: bytes 867840-868864 look like code no run loaded: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf:asave/SavedGame: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: 5 logged loads cover 28160 bytes of the image
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: blocks 112-120 (bytes 57344-61952) outside the filesystem hold data, not code (opcode density 0.025, entropy 4.98)
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: blocks 1013-1013 (bytes 518656-519168) hold ASCII text
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: blocks 1298-1298 (bytes 664576-665088) hold ASCII text
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: blocks 1309-1312 (bytes 670208-672256) hold ASCII text
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: blocks 1314-1322 (bytes 672768-677376) hold ASCII text
- balance of power - the 1990 edition (1989)(mindscape)[a2]/Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf: blocks 1325-1332 (bytes 678400-682496) hold ASCII text
- 4 never-loaded region(s), 219289 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf:stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d1_to_d0`: Copies the 32-bit value from register d1 into register d0, then branches into the middle of another routine.

### Format

- `parse_command_line`: Parses a command line into tokens, handling quoted strings and escape sequences, then stores the resulting token pointers in a table.
- `save_and_call_function`: Saves the incoming value and a pointer into a global variable, then calls the function pointer at offset -132 of that structure if a status byte is non-zero.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `call_dos_function`: Loads the dos library base address into a1, clears d0, and calls the library function at offset -132 of that base.
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
- `initialize_and_format_disk`: Initializes a DOS device by opening the library, allocating a memory buffer, and calling the routine that formats and verifies the disk, then returns a status value.
- `execute_callback_chain`: Checks five global variables for non-zero values and calls the corresponding routine stored at each offset when set.
- `traverse_and_invoke_table`: Walks a table of pointers, returns when it reaches a null entry, and calls a comparison routine for each valid entry; when the comparison succeeds it invokes a table entry and returns one, otherwise it returns zero.
- `format_disk_and_verify_2`: Initializes an I/O request structure and a device port, allocates a memory buffer, and writes a pattern of bytes containing the DOS signature to that buffer, then returns.
- `measure_string_length`: Returns the number of characters before the terminating zero byte in the given string.
- `copy_string`: Copies a null-terminated string from a source pointer to a destination pointer.
- `convert_to_uppercase`: Returns the input byte unchanged if it is outside the lowercase letter range, otherwise subtracts 32 from it.
- `compare_strings_case_insensitive`: Compares two strings byte by byte, converting each byte to uppercase via another routine before comparing.
- `truncate_at_colon`: Iterates through a string pointed to by a register until it finds a colon or a null terminator, then replaces that character with a null terminator.
- `manage_task_structure`: Allocates a structure and, on success, fills several fields with pointers and constants, makes a second allocation, and returns an error code if a field is zero.
- `save_and_reset_global_value`: Fetches a value from a structure via a function call, stores it in a global variable, then writes the constant -1 to that structure location.
- `restore_global_value`: Fetches a value from a structure via a function call and restores a previously saved value from a global variable into that structure location.
- `initialize_stack_data`: Populates a stack-based structure with several constant values and a pointer to a global variable, then calls another routine with a pointer to that structure.
- `execute_library_calls`: Reads a pointer from offset 50 of the first argument and calls several library functions using it along with the remaining parameters.
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
- `pass_params_to_subroutine_2`: Saves d2 on the stack, pushes the address of an eight-byte stack slot, and calls the function pointer stored at offset 14 of a global variable with a global value, the incoming argument, and that stack address.
- `read_with_status_check`: It calls a read routine through a global pointer, passing a fixed argument and a four-byte stack slot, then returns the byte from that slot if the result equals one or otherwise returns negative one.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_global_library_function`: It reads a function pointer from a global variable at a fixed offset and calls it with no arguments, then returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found and passes the buffer and checksum to another routine when the inverted checksum is non-zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_64bit_division`: If the unmodified high word of the input exceeds 65535 it divides the value by a divisor using a sixteen-iteration shift-and-subtract loop, otherwise it performs a full 32-bit divide by the low word and returns the quotient and remainder.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_advance_pointer`: It stores the byte in d0 to the memory location pointed to by a3 and advances that pointer by one byte, then returns.
- `format_disk_and_verify`: Initializes an I/O request structure and a device port, allocates a memory buffer, and writes a pattern of bytes containing the DOS signature into it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `execute_complex_logic`: Initializes a structure with a sequence of constant values at various offsets, then calls a routine with three identical arguments and another routine with a single argument.
- `create_extended_io_request_2`: Saves d2 on the stack, pushes the constant 48 as an argument, and calls the routine that creates an extended I/O request for the incoming pointer.
- `delete_extended_io_request`: Saves d2 on the stack, pushes the incoming pointer argument, and calls the routine that deletes an extended I/O request.
- `allocate_and_dispatch_message`: It allocates a signal bit and a memory block, and if either allocation fails it frees what was obtained and returns zero.
- `delete_and_free_port`: It removes the structure from a global list if a field at offset ten is non-zero, marks it deleted by writing constants to its status fields, frees the associated signal bit and releases the memory block, then returns.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `create_extended_io_request`: If the first argument is zero it returns zero, otherwise it allocates memory and, on success, initializes the block's fields with constants and the arguments before returning the pointer.
- `delete_and_free_io_request`: If the pointer argument is non-zero it marks the structure deleted by writing constants to its status fields and frees the associated memory block using the size stored within it, otherwise it does nothing.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### DiskCopy

- `parse_command_line_and_copy_disk`: Parses a command line by splitting it into quoted or space-separated tokens and storing pointers to each token in a table, converting asterisk-n and asterisk-e escapes into a newline and an escape character, then calls the disk copy setup routine.
- `dispatch_conditional_handlers`: Saves a parameter into a global variable, then calls the handler at offset minus 156 of that variable when a flag at offset 8 is nonzero and invokes two other handlers when the flag at offset 32 is nonzero, finally restoring the saved value.
- `invoke_two_function_pointers_2`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `store_result_and_dispatch`: Loads a pointer to a global variable into an address register, clears a 16-bit value in a data register, and calls a routine through that pointer.
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
- `manage_memory_list`: Allocates a block of memory from a global value, then loops through the resulting list storing values from another routine while counting nonzero entries and clearing entries after a nonzero count.
- `loop_until_non_zero`: Enters a loop that calls another routine and checks the return value, then branches on a global variable to call other routines with different arguments before repeating.
- `update_structure_fields_2`: Stores a 16-bit value at offset 28, a 32-bit value at offset 36, and another 32-bit value at offset 40 of a structure, then calls another routine to mutate it and returns.
- `update_structure_fields`: Stores a 16-bit value at offset 28, a 32-bit value at offset 36, and another 32-bit value at offset 40 of a structure, calls a routine to multiply two integers, then adds the result to a value at offset 44 of the structure.
- `synchronize_disk_contents`: Copies the directory structure and files from a source disk to a destination disk by traversing the source directory, creating matching directories on the destination, and copying each file's data while handling directory creation and cleanup.
- `setup_and_report_disk_copy`: Initializes the disk copy by allocating an I/O request and two memory buffers, cleaning up on failure, then computes how many disk swaps are needed and prints a message describing the source and destination drives.
- `execute_cleanup_sequence`: Tests a global variable and calls other routines with different arguments depending on whether it is zero, then calls another routine with 1 and returns.
- `cleanup_system_resources`: Checks several global pointers and, when each is nonzero, calls another routine to delete the corresponding port, I/O request, or memory block, then frees memory, closes libraries, and manages an error message task before returning.
- `extract_message_data`: Copies three values from a structure at the address in a global variable into other global variables, calls another routine with that structure, and then calls another routine with a global variable when the copied value is not 512.
- `execute_library_call`: Loads a value from the stack into d0, pushes a value from a global variable and d0 onto the stack, calls a routine at an offset from a global variable, and if the result is not zero it returns; otherwise it clears the top of the stack, calls another routine, and returns.
- `perform_disk_copy_loop`: Copies data blocks between two disks by issuing read and write I/O requests, transferring buffer contents and updating progress counters, while reporting errors and write-protection status.
- `allocate_multiple_libraries`: Checks a value at offset 20 from a global variable and, if it is less than 33, calls a routine with a fixed address and then calls a routine with 0; it then reads pointers from a global variable, calls routines with them, and uses a loop to retry memory allocation when it fails.
- `initialize_disk_copy_params`: Populates a stack-based structure with several constants and then calls two library functions using a global base.
- `route_disk_error_code`: Sets a pointer to a global variable based on the first parameter and the state of a flag, then calls another routine with a constant and that pointer, repeating the pattern for several parameter values before returning zero.
- `report_disk_errors`: Sets a pointer to a global variable and calls another routine with a global value and a stack address when a flag is zero, otherwise it calls that routine with both arguments, repeating this with a second constant before returning zero.
- `process_io_and_status`: Calls a routine and checks the result; if the result is zero it calls another routine with a pointer from a structure and a value from a table, then checks a global variable to decide whether to call a routine with a byte from a structure or to call a routine and increment the global variable; otherwise it returns zero.
- `check_signal_status`: Calls another routine with two parameters, checks if the twelfth bit of the result is set, and if so, calls another routine with a global variable; otherwise it returns the bit status.
- `process_message_queue`: When a flag is nonzero it enters a loop processing messages, otherwise it reads a message pointer, extracts several fields from the pointed-to structure, and returns a value based on the extracted contents.
- `set_parameter_zero`: Stores zero in a word at an offset from a pointer and nine in a word at a different offset before calling another routine.
- `manage_error_message_task`: Allocates memory, populates the resulting structure with various values and pointers, calls a routine to handle the structure, and then frees the memory.
- `set_parameter_and_call`: Stores the value 4 at offset 28 of a pointer and calls another routine, then stores the value 5 at the same offset and calls it again, and finally calls it once more.
- `execute_multiple_calls`: Pushes two global longwords onto the stack and calls a routine twice for each value, then cleans up the stack and returns.
- `process_disk_data`: Calls another routine with a constant and a value, then if the result is non-zero it performs several calls using a pointer from a global variable, updates a global variable with a pointer, modifies a byte at an offset of the pointer, calculates a sum by looping through 128 words in a table, subtracts that sum from a value at offset 5 of the pointer, and calls another routine with a constant and the pointer.
- `perform_io_operation`: Calls another routine with four parameters, then calls another routine and returns whether its result is zero.
- `compare_and_io`: Performs an I/O operation using a global variable as the request buffer, then calls another routine to update a structure.
- `allocate_message_entry`: Allocates a message buffer and, on success, populates several of its fields with the arguments and a value from another routine before dispatching it, otherwise frees the resources and returns zero.
- `delete_port_cleanup`: Removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `create_ext_io`: Allocates memory for an I/O request structure when the first argument is non-zero, initializes specific fields with constants and arguments, and returns the pointer to the block or zero.
- `delete_ext_io`: Clears specific fields in the I/O request structure and frees the memory block using the size stored within the structure, performing no operation if the pointer is zero.
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
- `execute_indirect_calls`: Fetches a function pointer from a global structure and calls it with a constant, then stores the result of a routine call and invokes the pointer again with different arguments before returning.
- `invoke_indirect_callbacks`: Calls another routine with a local stack array and a global variable, then calls another routine with the four incoming parameters and the same local stack array before returning.
- `convert_char_to_uppercase`: Returns the character with lowercase ASCII letters converted to uppercase by subtracting 32, while any other value is returned unchanged.
- `compare_strings_case_insensitive_2`: Compares two null-terminated strings by converting their characters to uppercase and returns 1 if they match, otherwise it returns 0.
- `terminate_colon_string`: Searches for the end of a null-terminated string and, if the last non-null character is a colon, replaces it with a null terminator.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_and_advance_pointer`: Stores the byte in d0 to the memory location pointed to by a3 and advances that pointer by one, then returns.
- `modular_reduction_32bit`: Performs a 32-bit modular reduction on a 16-bit divisor using repeated doubling and subtraction, then returns a combined 32-bit quotient and remainder.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag_2`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag_2`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_advance_pointer_2`: Stores the byte in d0 through the pointer in a3 and advances that pointer, then returns.
- `call_formatted_print`: Pushes a pointer and a value onto the stack and calls a routine to print a formatted message, then returns the result in a register.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot, then returns.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_global_library_function_2`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `execute_with_status_check`: Invokes another routine via a global pointer with a stack slot and a flag, then returns the value from the stack slot when the result is one or otherwise returns a negative error code.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.

### Port-Handler

- `initialize_port_handler`: Initializes a data structure by copying fields from a source pointed to by a0 and storing word offsets into it, then branches to one of two handlers depending on a value read from that source.

### Disk-Validator

- `writes_digit_and_fills_bitmap`: Writes a space character or a digit into a two-column text table indexed by a counter in a structure, then updates several counters and fills a bitmap-like table with bit patterns derived from a range.

### Ram-Handler

- `invoke_copy_mem`: Sets bit 7 of d0, saves a1 and a6, moves the arguments into a0 and a1, copies d3 into d0, loads the exec library base, and calls the CopyMem function before restoring the registers and jumping back.

### serial.device

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reinit_serial_buffer_fields`: Frees the block named by the first argument when it is set, then allocates a buffer of the size in d2, stores the size and allocated length into fields, and clears the buffer position fields and their half and quarter values.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `set_flag_and_check_handler`: Stores the value 253 into the byte at offset 31 from a1, then calls the handler at offset 378 when bits 0, 4 and 6 of the byte at offset 30 are all clear.
- `enable_interrupts_and_dispatch`: It writes the value 2049 to the interrupt enable register to enable interrupts, then calls the queue-processing routine.
- `dispatch_queue_request`: Clears the byte at offset 1f of a1, then compares the word at offset 1c of a1 against the values two, three and ten and branches to different handlers for each, otherwise it branches onward.
- `clear_queue_request_pointer`: Tests whether the pointer in a1 equals the value at offset 74 of the structure in a6, clears bit 1 of the byte at offset 296 of that structure, zeroes the pointer at offset 74, and returns.
- `unlink_first_queue_request_2`: Tests bit 6 of the byte at offset 30 of the structure in a1 and returns without doing anything when that bit is clear, otherwise it saves a1, relinks the pointers of a queue between two structures, restores a1, and decrements a counter at offset 70 of the structure in a6.
- `handle_palette_request`: Compares the request pointer in a6 with the palette base address and returns if they differ, then checks the request type and returns if it is not palette, clears the palette active flag and returns if already active, otherwise programs the palette data register with the 128-bit value and calls the palette write routine before clearing the request pointer and flag.
- `unlink_first_queue_request`: When bit 6 of the byte at offset 30 is set, unlinks the request from the front of the queue by relinking the pointers, decrements the queue count, marks the next request as pending and ready, then calls the handler when that request is not busy.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `set_serial_status_bit`: Loads zero into a return register, tests a word at offset 32 of the structure in a6 and returns immediately when it is zero, otherwise sets bit 6 of a byte at offset 472 of the structure in a6 and returns.
- `invoke_vectors_then_unlink`: Reads a pointer from offset 438, calls the handlers at offsets 12 and 163 twice each, copies a longword from offset 426, calls the handler at offset 360, unlinks the block from the list by relinking the pointers, and returns the copied value.
- `check_flag_and_call_handler_2`: Tests a flag byte at offset 30 of the structure in a1 against a mask and, if the test passes, calls a handler routine obtained from a table, then returns.
- `manage_serial_struct_list`: Manages a doubly linked list of structures stored in a global variable, inserting a new structure when the list is empty and removing the current one when a counter reaches zero, updating status flags and linked list pointers as it goes.
- `process_serial_buffer_bytes`: Iterates through a circular buffer pointed to by the structure in a6, calling a helper routine to process each byte and clearing a flag when a duplicate value is found, then returns.
- `test_flag_and_branch`: Tests bit 0 of the byte at offset 1bd of the structure in a6 and branches to different handlers depending on whether it is set, otherwise it branches onward.
- `store_byte_when_result_nonzero`: Calls a helper routine and returns without doing anything when its result is zero, otherwise it stores the byte value 9 at offset 31 of the structure in a1 and branches to a shared routine.
- `process_next_structured_item`: Clears a byte at offset 74 of the structure in a6, decrements a counter at offset 73, and if the result is nonzero it decrements a byte at offset 70 and loads a pointer from offset 162 of that structure.
- `restore_registers_and_return_11`: Restores registers d2 and a6 from the stack and returns immediately.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `clear_flag_store_byte`: It clears bit 2 of the byte at offset 470 of the structure in a6 and returns immediately when that bit was already clear.
- `clear_flag_and_store_byte`: Clears bit 7 of the byte at offset 1d6 of the structure in a6 and returns immediately when it was clear, otherwise it stores the value nine at offset 1f of a1 and branches onward.
- `store_byte_and_clear_pointer`: Stores the byte value 12 at offset 31 of the structure in a1, and when bit 0 of the byte at offset 294 of the structure in a6 is set it stores the byte value 6 at the same offset, clears the pointer at offset 62, masks the byte at offset 294 with 248, and returns.
- `handle_serial_interrupt`: Processes a serial device interrupt by checking status flags and, when a byte is available, copies it from the input buffer into the output buffer while advancing the buffer pointers, then signals the serial library.
- `check_and_enable_serial_interrupts`: Compares a byte at offset 34 of the structure in a6 with the value in d1 and returns immediately if they differ, otherwise clears bit 2 of the byte at offset 456 of the same structure and enables interrupts.
- `test_serial_flags_branch`: Tests bit 5 of the byte at offset 45 of the structure in a6 and returns immediately when that bit is clear.
- `test_bit_branch`: It tests bit 8 of the byte in d0 and jumps onward when that bit is set.
- `test_bit_and_branch`: Tests bit 0 of the byte at offset 189 of the structure in a6 and, if it is set, branches into the middle of another routine.
- `set_flag_and_record_pointer`: Sets bit 7 of the byte at offset 1d6 of the structure in a6, tests the long at offset 3e of that structure, and stores the pointer at offset 2a into offset 3e when it is zero, otherwise it branches onward.
- `store_byte_and_signal`: Reads the counter at offset 36 and the write pointer at offset 3a of the structure in a6, stores a byte and advances both pointers when space remains, refreshes the buffer base when it reaches the end, otherwise records the buffer pointer and signals when the buffer is full, then it calls a routine from a table at offset 62.
- `record_pointer_then_signal`: Records the buffer pointer at offset 42 when the field at offset 62 is unset, calls a helper routine, then calls a library vector to signal completion.
- `copy_byte_and_clear_interrupt_flags`: Copies the byte at offset 34 of the structure in a6 to offset 297 of that structure, clears bits 2 and 3 of the byte at offset 296, and when bit 2 of the byte at offset 69 is set it clears bit 6 of the parallel port output register and returns.
- `handle_serial_interrupt_flags`: Copies the byte at offset 35 of the structure in a6 to offset 217 and sets bit 3 of the byte at offset 216.
- `clear_struct_byte_217`: Clears the byte at offset 217 of the structure in a6 and returns.
- `set_flag_and_signal_library`: Sets bit 6 of the byte at offset 214 of the structure in a6, zeroes the longwords at offsets 418 and 422, stores 9 into the word at offset 414, then calls the routine at offset 30 of the pointer at offset 406 and stores 11 into the byte at offset 31 of the value in a1 when it is non-zero.
- `read_serial_data_and_signal`: It saves registers d2, a3 and a6, copies the pointer in a1 into a6, and clears bit 6 of the byte at offset 470 of that structure.
- `load_offset_and_call`: Loads the address of the value at offset 386 of the structure in a6 into a0, calls the routine at 5e8, then branches to the routine at 8e4.
- `set_flag_and_branch_2`: It sets bit 0 of the byte at offset 470 of the structure in a6, then jumps into another routine without returning.
- `process_serial_request`: Clears the word at offset 32 of the request in a1, and when the word at offset 36 is nonzero it clears bit 1 of the byte at offset 30 and links the request into a queue in the structure in a6, incrementing a counter there.
- `handle_request_and_link`: Tests a flag byte at offset 30 of the structure in a1 and, if the test passes, calls a handler routine; otherwise it returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_request_state_on_flags`: Checks if two specific bits in a global variable are set, and if so, writes a byte with a value of thirteen to an offset within a structure pointed to by a register.
- `handle_serial_request_completion`: Processes a serial device request by clearing the pending flag, removing the request node from the linked list, and merging the received byte or timer result into a global status word, then calls the next queued request handler when the list is empty.
- `init_serial_buffer_state`: Stores the decimal 200 into the byte at offset 476 of the structure in a6 and loads a pointer from offset 86 of that structure into a0.
- `merge_bits_process_buffer`: It compares the incoming value in d1 with the counter at offset 94 of the structure in a6 and jumps onward when the counter is larger.
- `merge_bit_and_process_buffer`: Combines a bit derived from the byte at offset 67 of the structure in a6 with the word at offset 468 into a global variable, increments the longwords at offsets 86 and 94, then scans up to seven bytes from offset 110 while they exceed the low byte of that value before storing the longword at offset 94 into the destination at offset 36 of a1.
- `remove_node_from_linked_list`: Removes a node from a doubly linked list when its pointer and the list head are nonzero, clears a byte at offset 30 of the removed node, decrements a counter at offset 72 of the structure in a6, sets and clears bits of a byte at offset 30 of that node, zeroes a field at offset 94, and calls a helper routine when a counter at offset 78 equals 10.
- `set_interrupt_flag_and_return`: Restores registers d2 and a6 from the stack, sets the byte at offset 1 of a global variable to one, and returns.
- `read_serial_data_byte`: Clears the serial data register flag, reads the received byte from offset 473 and combines it with the flags at offset 468.
- `init_serial_request`: Sets a global variable to a value with bit 11 set, then clears bits in two status bytes and a byte mask at offset 45 of the structure in a6.
- `call_handler_and_check_flags`: Calls a routine that tests a flag byte and invokes a handler, then returns if the relevant flag is not set.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `set_flag_and_call_handler`: Sets bit 2 of the byte at offset 296 when the pointer at offset 78 is nonzero, calls a helper routine when the pointer at offset 74 is nonzero, and calls a handler routine obtained from a table when a masked byte at offset 30 of a pointer in a1 is zero.
- `clear_flags_and_call_handler_4`: Clears bits 2 and 3 of the byte at offset 472 of the structure in a6, calls the handler at offset 74 when the longword there is nonzero, writes 32769 to the interrupt enable register, and calls the routine at offset 378 of the pointer at offset 98 when the byte at offset 30 of a1 passes its mask test.
- `update_flags_and_call_handler`: Sets a flag byte in a global variable, calls a routine that tests a flag byte and invokes a handler, updates the flag byte, then returns if the relevant flags are not set.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `swap_pointer_and_call_handler`: Loads a pointer from a global variable and, if it is not null, swaps it with the incoming pointer, stores the new pointer at offset 4 of the structure, and calls a handler routine; otherwise it returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `build_status_and_store`: Builds a status byte from a global variable and flags in the structure in a6, stores it at offset 81 of the structure in a1, copies a byte at offset 470 and masks it, copies a 32-bit value at offset 58, clears a flag in the structure, then returns if the relevant flags are not set.
- `clear_request_flag_and_check_conditions`: Clears bit 0 of the byte at offset 30 of the request in a1, and when bit 3 of the byte at offset 79 is set it returns without doing anything if a counter at offset 72 of the structure in a6 is nonzero or the pointer at offset 78 is nonzero.
- `configure_serial_baud`: Loads the pointer at offset 106 of the structure in a6 and calls the routine at f1a.
- `reply_and_clear_active_request`: Saves the current base register, switches to the exec library, replies to the message through it, restores the base register, clears the active request field at offset 78 of the structure, and returns.
- `clear_uart_break_and_reply`: Clears bit 5 of the status byte at offset 29 of the structure in a1, then, when that bit was set, the flag byte at offset 78 equals ten, and a flag bit at offset 69 of the structure in a6 is set, it sets bit 1 of that status byte, zeroes the word at offset 78, writes a global word to 2048, and calls a routine.
- `start_timed_read_request`: Sets a global word to one, zeroes the word at offset 322 of the structure in a6, stores the input value at offset 326, writes nine to the word at offset 318, calls a library vector, and sets a status byte at offset 31 of the request to eleven when the call reports an error.
- `handle_serial_request_flags`: Returns immediately when bit 7 of the byte at offset 45 of the structure in a1 is clear.
- `validate_serial_request_2`: Clears bit 7 of the byte at offset 45 of the structure in a6, then returns immediately if the byte at offset 44 of a1 has bit 6 set or if the value at offset 78 of a6 is nonzero.
- `enable_interrupts_and_call_handler`: Writes 2049 to the interrupt enable register, calls a helper routine, then writes 34817 to the same register and calls the routine at offset 378 of the pointer at offset 98 when the byte at offset 30 of a1 passes its mask test.
- `validate_serial_request`: Validates the serial request by checking that its port name, unit name and device name fields are non-empty and within expected ranges and that a request pointer is set, then copies those fields and a request ID into a global structure and sets status byte 31 to 5.
- `compute_unit_channel_masks`: Builds a one-bit mask from the byte at offset 77, doubles it when the flag byte at offset 78 is two, shifts it again when a flag bit at offset 69 of the structure in a6 is set, stores the result at offset 468, and calls two helpers to store their results at offsets 466 and 464.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `store_request_status_byte`: Stores the constant 5 in the byte at offset 31 of the request in a1, then branches to the routine at 10de.
- `compute_serial_period`: Multiplies the value at offset 450 by nine, and when that result is greater than 65535 it divides a large constant by it to compute a divisor.
- `store_serial_period_quotient`: Divides the input word by the second input and stores the quotient as the period word at offset 462, clears bit 5 of the byte at offset 472, and when the byte at offset 66 is eight and a flag bit at offset 69 is set, it ORs fifteen into the global word and sets bit 5 of that period byte.
- `init_serial_device_structures_2`: Initializes two serial device structures by calling an initialization routine for each, writing status bytes and request pointers at several offsets, and returns one when the setup succeeds or zero otherwise.
- `init_serial_device_structures`: Initializes two serial device structures by calling an initialization routine for each, writing status bytes and request pointers at several offsets, installing handler vectors, and returning a constant.
- `store_pointer_in_struct`: Copies the pointer in a0 to the word at offset 16 of the structure in a1 and returns.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `init_serial_device`: It initializes serial device structures by writing status bytes and request pointers at several offsets, installs handler vectors, and then calls the interrupt handler before returning a success value.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_and_branch_back`: Clears the return value in d0 and then branches into the middle of the preceding routine.

### parallel.device

- `update_parallel_status_2`: Checks a status bit in the structure in a6 and a flag byte in the request in a1, and when both are set it increments a counter at offset 20 of the structure in a6 if two marker bytes are zero.
- `update_parallel_status`: Increments the word at offset 32 of the structure in a6, copies the byte at offset 117 into the byte at offset 53 of the request in a1, and sets bit 4 of the byte at offset 118.
- `update_status_and_copy_data`: Clears the status byte at offset 118, sets bit 5 of the byte at offset 117 when a flag bit at offset 53 is set, increments the counter at offset 32 when two status bytes are zero, copies a status byte into the structure in a1, sets bit 4 of the byte at offset 118, and copies two 32-bit words when a flag bit at offset 53 is set.
- `handle_parallel_request`: When the byte at offset 30 of the request in a1 has none of bits 0, 6 or 13 set it calls the handler routine at offset 30.
- `handle_parallel_mode_switch`: When the word at offset 28 is three, it clears a global bit, calls a library vector when a flag bit at offset 34 is clear, and sets that flag bit, then calls another library vector with both structure pointers and returns.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `handle_parallel_request_2`: Stores the byte 253 at offset 31 of the request in a1, then when bits 0, 6 and 13 of the byte at offset 30 are all clear it calls the handler pointer stored at offset 15 of the structure in a6.
- `clear_device_status`: Tests the 32-bit value at offset 36 of the structure in a1 and, when it is non-zero, branches away; otherwise it zeroes the 32-bit value at offset 32 and branches to another routine.
- `link_device_status`: Clears bit 1 of the byte at offset 30 of the structure in a1, then, if bit 2 of the byte at offset 118 of the structure in a6 is set, links the a1 structure into a list at offset 84 of a6 and increments the counter at offset 114; otherwise it sets bit 4 of offset 30, stores the a10[10] value at offset 42 of a6, zeroes offset 46, links a1 at offset 38, and adjusts status bytes at offsets 116 and 118.
- `update_device_status`: Sets bit 4 of the byte at offset 30, copies a 32-bit word into the structure in a6, zeroes the following word, stores the request pointer, sets a flag bit when a length is negative, zeroes a global word, clears bit 3 of a status byte, sets a flag bit when that bit was set, and sets a flag and a global word when the word at offset 28 is not two.
- `process_parallel_request_queue`: Pops a pending request from the front of a FIFO queue in the structure in a1 and links it into a free list, then processes the next queued request by calling update_device_status, transfer_byte_and_update_status and update_device_status_2 in sequence.
- `transfer_byte_and_update_status`: Moves the byte at offset 42 of the structure in a1 into a global variable when bit 5 of its byte at offset 30 is clear, adjusts that value through a seven-byte scan of a1, then stores the longword at offset 46 of a6 into the destination at offset 32 of a1 and calls the routine at offset 378 of a6.
- `update_device_status_2`: Copies a 32-bit value from offset 46 of the structure in a6 into offset 32 of the structure in a1, clears bits 3 and 5 of the status bytes at offsets 118 and 30, then calls a routine from a vector table at offset 42 of a6 and zeroes offset 38 of a6.
- `read_cia_register`: Reads a byte from the CIA register at the address in a0 and branches away when it is non-zero, then tests bit 3 of the byte at offset 118 of the structure in a6 and branches away when set, otherwise it branches to another routine.
- `process_parallel_request`: Calls a routine through a pointer at offset 50 of the structure in a6 and zeroes the byte at offset 118 of that structure.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_if_not_zero`: Checks if a value in a structure is non-zero and calls a function through a pointer if it is.
- `invoke_callback_pointer`: Executes a function through a pointer stored in a structure if the associated index is not zero.
- `swap_device_pointer`: Loads a pointer from offset 4 of the structure in a6, and if that pointer is non-zero it replaces the stored pointer with a1, swaps a1 and d0, writes a1 into the pointed-to location, and calls the routine at offset 62 of a1.
- `clear_parallel_status_word`: Clears the word at offset 114 of the structure in a6 and returns.
- `modify_status_byte`: Sets bit 5 of the byte at offset 30 of the structure in a1, writes the value 254 to the byte at offset 31, then calls the routine pointed to by the pointer at offset 50 of the structure in a6 and returns.
- `call_pointer_from_stack`: Saves and restores d2, loads a pointer from the stack into a0, pushes that pointer, calls the routine at offset 100 of a0, then restores d2 and returns.
- `mark_request_handled`: Tests bit 2 of the byte at offset 118 of the structure in a6 and branches away when it is clear, otherwise it stores the byte 1 at offset 31 of the request in a1 and branches to another routine.
- `copy_structure_fields`: Copies a byte from one structure to another and, if a specific bit is set, copies two long words as well.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `call_dos_library_stub`: Saves a0 and a6 on the stack, calls the routine at offset 0 of the structure in a0, removes the two saved registers from the stack, and returns.
- `calls_insert_clipboard_entry`: Saves registers d0 and a1 on the stack, calls insert_clipboard_entry, then restores the stack and returns.
- `release_clipboard_reference_2`: Calls the clipboard routine that releases a clipboard reference and then returns.
- `jump_to_handler`: Branches unconditionally to the routine at the address in the global variable at offset 743.
- `call_and_mask_offset_2`: Pushes a1 onto the stack, calls the routine at offset 250 of a2, restores the stack, and then calls call_and_mask_offset.
- `call_clipboard_handler`: Saves the value in a1 on the stack, calls the routine at the address in the global variable at offset 743, and removes the saved value from the stack.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `call_dos_library`: Saves two registers on the stack, loads a pointer from the stack into a2 and stores it into a global variable at offset 306, then pushes a zero and a literal onto the stack before calling the routine at offset 404 of that global variable.
- `store_global_and_call`: Stores the a3 value into a global variable, calls a routine with the address of that global variable, calls it again with no arguments, and then calls a routine with a computed address.
- `call_library_and_return_one`: Passes the address of a global variable to a library function, calls a second library function, loads d2 and d3 from the stack, and returns the value 1 in d0.
- `call_and_store_result`: Calls a routine with no arguments, then calls a routine with the decimal arguments 530 and 65536 and stores its result.
- `clear_word_and_branch`: Clears the word at offset 14 of the structure in a3, pushes a3 onto the stack, pushes the address at offset 72 of a2, calls the routine at offset 128, adds 8 to the stack pointer, and branches to offset 236.
- `zero_word_field`: Stores a zero word into a specific offset within a structure and then calls another routine.
- `measure_clipboard_string`: Saves a register on the stack, loads a pointer from the stack into a2 and calls the routine at offset 0 of that structure.
- `update_pointer_chain`: Pushes the address at offset 72 of a2 onto the stack, calls the routine at offset 0 of a2, zeroes the 32-bit value at offset 86 of a2, calls the routine at offset 16 of a2, restores d2 and a2 from the stack, and returns.
- `copy_characters_from_clipboard_page`: Copies a variable number of characters from a page within a linked list of character pages to an output buffer, advancing the page pointer and wrapping the character offset within each page.
- `compare_and_store_offset`: Reads a long word from offset 90 of the structure in a2 and compares it with the value at offset 48 of the stack, returning early when it is not greater.
- `copy_clipboard_block_bytes`: Returns immediately when the first argument is zero, otherwise it splits the second argument into a block index and an offset within a block and loads a pointer from offset 86 of the structure in a2.
- `compare_and_loop_pointer`: Copies the longword from d0 into d1, compares d1 with the longword at offset 14 of the structure in a1, and branches forward when d1 is not less than that value.
- `compare_and_branch_forward`: Tests the value in d5, loads 512 into d0 and subtracts d2 from it, then branches forward past the comparison when the result is not less than d5 before copying d2 into d4.
- `copy_table_bytes`: Copies bytes from a table at offset 18 of a1 into the buffer pointed to by a3 for each value of d4, increments d4, and repeats while d4 is less than 512.
- `copy_word_and_branch`: Copies the word in d2 into d4, then jumps forward into the middle of another routine without returning.
- `copy_table_until_total`: Copies a byte from a table at offset 18 of a1 into the buffer pointed to by a3, then compares the running total in d1 with the current d4 value and stops when the total exceeds it.
- `store_long_word`: Stores a long word into a specific offset within a structure and then calls another routine.
- `store_clipboard_buffers`: Checks whether a clipboard request fits in the current buffer and, if so, copies the requested data into a doubly linked list of clipboard buffers stored in a global variable, updating the per-buffer size fields and clearing the status byte when the copy is incomplete.
- `copy_clipboard_buffers`: Walks a doubly linked list of clipboard buffers stored in a global variable, and for each buffer whose size fits the current request copies its bytes into the target buffer at offset 18, advancing a running position counter until all requested data is written.
- `push_clipboard_entry`: Pushes the value at offset 72 of the structure in a2 onto the stack and calls the clipboard device, then adds four to the stack pointer and jumps to the following routine.
- `store_clipboard_data`: Clears the 16-bit value at offset 94 of the structure in a2, then if the value at offset 20 of the structure in a6 is non-zero it walks a linked list of clipboard blocks, copying bytes from the source buffer into each block's data area until the buffer is exhausted.
- `store_clipboard_block_bytes`: Splits the longword at offset 24 of the structure in a6 into a block index and an offset, then loads a pointer from offset 86 of the structure in a2, allocating a block when that pointer is null.
- `copy_clipboard_blocks`: Walks a linked list of structures by following the pointer at the base of each node while its offset 14 longword is greater than d2, reinitializing the pointer through a helper when it reaches a zero entry.
- `compare_and_branch_forward_2`: Tests the value in d4 and the longword at offset 20 of the structure in a6, loads 512 into d0 and subtracts d3 from it, then branches forward past the comparison when the result is not below that longword before copying d3 into d1.
- `copy_buffer_in_chunks`: Copies bytes from a source buffer into a destination chain, advancing a running index and following linked buffers when the current one fills.
- `copy_word_and_branch_2`: Copies the word in d3 into d1, then jumps forward into the middle of another routine without returning.
- `copy_clipboard_data_in_chunks`: Copies a byte from a string through a pointer into a buffer at offset 18, advancing a counter, then walks a chain of linked buffers writing 512-byte chunks until the source is exhausted, storing the total byte count in a global variable and returning it.
- `store_register_to_offset`: Stores the contents of a data register into a memory location calculated by adding a fixed offset to a pointer held in an address register.
- `check_byte_flags_and_call`: Saves a2 on the stack, restores it from the stack, calls the routine at offset 0, checks bit 7 of the byte at offset 30 and branches if set, checks bit 0 of the same byte and branches if set, saves a2 again, calls the routine at offset 344, removes the saved a2, and branches to offset 260.
- `modify_byte_bits`: Clears the least significant bit and sets the most significant bit of a byte at a specific offset in a buffer.
- `clear_multiple_flag_bits`: Clears bit 7 of the byte at offset 30 of a structure when that bit is currently set, then clears bit 1 and bit 5 of the same byte before returning.
- `compare_offsets_return_one`: Loads two pointers from offsets 112 and 96 of a structure, compares the values at offset 48 of each, and returns the constant 1 in d0 when the first value is greater than or equal to the second, otherwise it returns.
- `load_zero_and_branch`: Loads the value 0 into d0 and branches to offset 252.
- `clear_d0_and_branch`: Clears the d0 register and branches into the middle of another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_structure_fields_2`: Copies a long value from offset 40 of one structure into offset 152 of another, then copies a word from offset 16 into offset 146, and finally copies a long from offset 48 into offset 148 before calling another routine.
- `initialize_clipboard_structures`: Copies the two pointers passed in the structure into the registers a2 and a3, then sets bit 5 of the status byte at offset 30 of the second structure.
- `process_clipboard_buffer_entries_2`: Sets the flag byte at offset 66 of the structure in a2 to 1, then repeatedly walks a linked list of clipboard buffer entries, advancing a cursor and adjusting length counters while clearing or setting state flags depending on the current state, and otherwise returns.
- `clear_then_jump`: Clears the word in d3 and then jumps to another routine.
- `advance_clipboard_buffer_entries_2`: Walks a linked list of clipboard buffer entries, advancing a cursor and adjusting length counters while clearing or setting a flag byte, and updates a counter at offset 110 of the structure in a2.
- `call_object_and_flag`: Pushes a2 onto the stack, calls the routine at offset 0 of the structure pointed to by the pushed pointer, pops the stack, sets the word at offset 94 of the structure in a2 to one, and jumps to another routine.
- `advance_clipboard_buffer_entries`: Walks a linked list of clipboard buffer entries, advancing a cursor, adjusting length counters, and clearing or setting state flags depending on each entry's type.
- `clear_register_then_branch`: Clears the 16-bit register d3 and branches to a later routine.
- `advance_clipboard_cursor`: Advances a cursor through a linked list of clipboard buffer entries, updating length counters and state flags, recursing into nested entries and returning when the end of the list is reached.
- `update_clipboard_buffer_entries`: Updates a linked list of clipboard buffer entries by advancing a cursor, adjusting a length counter, and clearing or setting state flags depending on the entry type.
- `accumulate_offsets_and_maybe_copy`: Loads the long at offset 90 of the structure in a2 and the long at offset 44 of the structure in a4, adds the long at offset 44 to that value, and if the accumulated result is not less than the loaded long it copies the long at offset 44 into offset 32 of the structure in a4.
- `subtract_and_accumulate_offsets`: Loads a long from offset 90 of the structure in a2 and subtracts the long at offset 44 of the structure in a4, storing the result at offset 32 of that structure.
- `clear_flags_and_call`: Clears a long at offset 32 of the structure in a4, loads the value 255 into d0, stores it at offset 48, decrements a short at offset 110 of the structure in a2, saves a3 and a4 on the stack, calls a routine, restores the registers, and branches when the decremented short is nonzero.
- `process_clipboard_buffer_entries`: Processes clipboard buffer entries by offset 40 of the structure in a2: when the state is 3 and a linked entry exists, it adds the entry's size to a running total capped by the buffer limit stored at offset 90, updates the total, and recurses on the next entry; when the state is 4 it sets the state to 4.
- `dispatch_clipboard_clear`: Copies a pointer from one structure into another, saves two registers, calls a library routine, and jumps to a shared cleanup routine.
- `compare_store_four_branch`: Loads the value 4 into d0, compares it with the word at offset 28 of the structure in a4, and returns without changes when they differ, otherwise it stores 4 into the word at offset 64 of the structure in a2 and branches into another routine.
- `clear_and_jump_cleanup`: Clears a 16-bit register and jumps to a shared cleanup routine.
- `clear_multiple_flag_bits_2_2`: Clears bit 5 of the byte at offset 30 of the structure in a3 when that bit is set, then clears bits 1 and 5 of the same byte before returning.
- `clear_longword_and_branch`: Clears the longword at offset 90 of the structure in a2, then branches forward into the middle of another routine without returning.
- `clear_multiple_flag_bits_2`: Clears bit 7 of the byte at offset 30 of the structure in a3 when that bit is set, then clears bits 1 and 5 of the same byte before returning.
- `find_last_list_node`: Walks a singly linked list of four-word nodes starting from a pointer stored in the structure at a6, advancing until it finds a node whose offset 12 value is greater than the value at offset 48 of param_2 or reaches the end, then pushes that node's offset 4 value and returns it to the caller.
- `load_zero_and_branch_5`: Loads the constant 0 into register d1 and branches into the middle of another routine.
- `push_pointers_and_call`: Pushes the word at offset 4 of the structure in a2, pushes a0, pushes a1, calls the routine at offset 100, adjusts the stack pointer by 12, restores a2, and returns.
- `set_byte_value`: Writes a constant byte value to an offset within a structure.
- `call_pointer_from_stack_2`: Saves d2 on the stack, loads a pointer from the stack into d2, pushes that pointer, calls the routine at offset 150, adds 4 to the stack pointer, restores d2, and returns.
- `copy_and_call_routine`: Copies the value in d2 to the address stored in d2, then calls a routine with that value and returns.
- `call_pointer_offset`: Pushes a pointer onto the stack, calls the routine at the fixed offset, then restores the stack and returns.
- `forward_argument_and_call`: Copies the value from the second longword argument to the first and then calls another routine with that value.
- `call_three_routines_in_sequence`: Saves register d2 on the stack, passes the argument three times to three different routines in sequence, then restores the stack and returns.
- `copy_node_state_and_traverse`: Loads a pointer from offset 24 of the structure in param_1 and reads the 32-bit value at offset 48 of that pointed-to structure.
- `call_routine_with_offset_96`: Pushes a value from register a3 plus an offset of 96 onto the stack and calls a routine, then pushes the pointers from registers a2 and a3 and calls another routine, restores the registers, and returns.
- `increment_node_count_and_traverse`: Loads a pointer from offset 24 of the structure in param_1 and reads the 32-bit value at offset 48 of that pointed-to structure.
- `call_routine_with_offset_112`: Pushes a value from register a3 plus an offset of 112 onto the stack and calls a routine, then pushes the pointers from registers a2 and a3 and calls another routine, restores the registers, and returns.
- `copy_structure_field`: Copies a longword from an offset relative to a pointer to another offset in a structure.
- `copy_structure_field_offset`: Copies a longword from a different offset relative to a pointer to a field in a structure.
- `insert_clipboard_entry`: Searches a linked list of clipboard entries by comparing a value at offset 14 of each entry, and when no matching entry is found inserts a new entry by incrementing the counter at offset 18 and the global byte at offset 20, storing the new entry pointer at offset 24 of the structure, and if the flag at offset 67 equals one it calls additional setup routines.
- `compare_and_clear_registers`: Compares register d3 with the longword at offset 14 of a structure, and when they are equal it clears registers d4 and d2 before branching into the middle of another routine.
- `initialize_clipboard_buffer_2`: Increments a longword at offset 18 of the structure in a3 and an 8-bit counter, stores a3 at offset 24 of the structure in a2, and, when a flag byte at offset 67 of the structure in a3 equals 1, calls several other routines to set up clipboard state.
- `initialize_clipboard_buffer`: Initializes a clipboard structure by storing a global pointer and several flag bytes, increments a global counter, and then repeatedly opens and reads clipboard data until the buffer is exhausted, copying each clipboard's text into it.
- `call_library_then_clear_flag`: Pushes two arguments onto the stack and calls a library function, then compares its result with the constant 512 and, when they differ, clears a byte at offset 94 of a structure before continuing.
- `check_clipboard_result`: Tests the word at offset 94 of the structure in a3 and the longword at offset 4 of the structure in a6, saving a3 on the stack before calling a helper and loading its result into a0.
- `process_clipboard_string`: Calls a helper routine and compares its result with the longword at offset 4 of the structure in a6, clearing the word at offset 94 when they differ, then repeatedly calls the routine at offset 22 of each returned structure while it is nonzero before returning the low byte of the value in a2.
- `increment_and_call_libraries`: Increments a longword at offset 18 of the structure in a3 and an 8-bit counter, stores a3 at offset 24 of the structure in a2, and, when a flag byte at offset 67 equals one, makes several library calls through pointers and returns a signed byte from offset 31 of the structure in a2.
- `release_clipboard_reference`: Decrements a reference counter in a clipboard structure and, when it reaches zero, releases any clipboard memory it holds by walking its list of buffers and freeing each one, then releases the clipboard itself.
- `call_library_then_clear_byte`: Saves register d4 on the stack and calls a library function, then compares its result with the saved value and, when they are equal, clears a byte at offset 94 of a structure and loads a pointer from offset 0 before continuing.
- `clear_flag_and_branch_2`: Clears the byte at offset 94 of a structure and branches into the middle of another routine.
- `reset_clipboard_state`: Calls a helper three times, then branches on the zero value at offset 68 of the structure in a3 to either call the helper with no argument or with that value.
- `reset_clipboard_structure_fields`: Clears two 32-bit fields at offsets 20 and 24 of the structure in a2, decrements a byte counter in a global variable, and if that counter reaches zero and bit 3 of a global status byte is set it calls measure_clipboard_string, otherwise it returns.
- `check_flag_then_call_libraries`: Checks a global byte and branches when it is set, then pushes a pointer onto the stack and calls two library functions in sequence before loading a result into register d0 and continuing.
- `set_flag_return_zero`: Sets bit 3 of a global byte and returns zero in d0.
- `call_and_mask_offset`: Loads a pointer into a2, calls the routine at the fixed offset, writes five to offset 8 of the pointed-to structure, masks offset 30 with 79, and returns without looping.
- `invoke_table_entry_3`: The routine invokes a function pointer retrieved from an offset within a global variable using an index.
- `check_flags_and_set_byte`: Saves a2 on the stack, calls a routine through a pointer at offset 0, then checks bit 7 of the byte at offset 30 of the structure in param_1 and branches if it is set.
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
- `setup_pointer_chain_4`: Stores the pointer argument at the location it points to, adds four to that stored value, clears the following word, and stores the original pointer into the word after that.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `invoke_table_entry_6`: Loads a base pointer from a global variable, then invokes the function pointer stored at offset 522 below that base and returns.
- `store_byte_advance_pointer_3`: Stores the byte in d0 to the location pointed to by a3 and advances that pointer, then returns.

### printer.device

- `call_and_store_init`: Calls the routine stored at offset 84 of the structure in a6 and stores its result in a global byte, then returns that result.
- `load_constant_and_call`: Loads the constant 254 into d0 and calls the routine at offset 240, then returns.
- `manage_flag_and_list`: Calls a routine through a pointer, sets bit 7 of a flag byte at offset 30, removes the pointed-to node from a linked list when that bit was clear, calls another routine when bit 4 of the flag is set and bit 0 is clear, and finally calls a third routine through a pointer.
- `run_printer_initialization`: Calls the function at offset 84 of the structure in a6 and stores its result in a global byte.
- `dispatch_printer_library_calls`: Stores the base pointer in a6 into a global variable, then calls a sequence of library functions and returns their results, filling a buffer with a repeating word pattern when a value is zero.
- `dispatch_multiple_entries`: Invokes the function pointer stored at offset 414 of the global structure, then calls it twice more with the a6 base, and finally invokes the pointer at offset 210 of the same structure.
- `toggle_status_bits_in_sequence`: Calls three helper routines in sequence, setting or clearing a status bit in the structure in a6, then returns.
- `load_constant_and_call_2`: Loads the constant 253 into d0 and calls the routine at offset 240, then returns.
- `set_status_bit_zero`: Sets bit 0 of the byte at offset 61 of the structure in a6 and returns.
- `clear_status_bit_zero`: Clears bit 0 of the byte at offset 61 of the structure in a6 and returns.
- `test_pointer_and_call`: Loads a base pointer from offset 42 of the structure in a6, tests whether it is zero, and if so branches back to the start of the routine; otherwise it calls the routine at offset 240 and returns.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_branch_return_2`: An empty routine that contains only a branch instruction and returns to the caller without doing anything.
- `init_buffer_and_read_flags`: Reads a hardware value and stores its low byte at offset 67 of the structure in a6, then initializes a growable buffer by storing its own end pointer, zeroing a length field, and recording a new end pointer, repeating the same setup for a second buffer at a higher offset.
- `compare_and_clear_bit`: Computes the address of a base structure into a1, compares it with d0, and branches past the rest if they differ.
- `process_flags_and_queue`: Repeatedly checks a bit in d2 and, if set, walks a linked list from the pointer at offset 72, setting bit 5 of each node's byte at offset 30 and calling a routine indexed by a 16-bit value in the node.
- `load_stack_pointers_and_call`: Loads a pointer from the top of the stack into a0 and a second pointer from eight bytes above into d0, calls the routine at offset 240, and returns.
- `set_flag_and_store_incoming_values`: Reads a byte from a global variable, sets bit 1 of a status byte at offset 9b6 of the structure, adjusts the base pointer by an offset depending on that bit, stores two incoming values at offsets 24 and 28, writes three at offset 1c, then calls two helper routines.
- `empty_adjust_stack_return`: An empty routine that adjusts the stack pointer by adding eight and then returns to the caller without doing anything.
- `set_flag_and_load_offset`: Sets bit 1 of the byte at offset 9b6 of the structure in a6, loads the address of the value at offset 190 into a1, and returns to the caller.
- `check_flag_and_invert`: Loads a base pointer from a global variable, copies the byte at offset 31 into d0, and returns immediately if that byte is zero.
- `loop_printer_operations`: Repeatedly invokes the library stubs at offsets 84, 318, 138 and 170 of the global base until a flag at offset 332 of the structure in a6 is set.
- `compare_printer_pointer`: Loads the address at offset 190 of the structure in a6 into a1 and compares it with d0, branching to a helper when they differ.
- `compare_pointer_and_branch`: Loads an address from offset 110 of the structure in a6 into a1, compares it with d0, and branches to a helper routine when they differ, then tests d0 and returns when it is zero.
- `poll_flags_and_invert`: Calls the routine at offset 908, then repeatedly checks a flag byte at offset 31 of a global structure and returns its value if nonzero, otherwise calls the routine at offset 908 again and loops through a set of hardware polls until a matching entry is found.
- `store_value_from_base`: Reads a pointer from a global base, stores the value nine at offset 28 of the structure it points to, copies a 32-bit value from another global into offset 32 of that structure and clears the long at offset 36.
- `clear_status_byte`: Sets a byte at a specific offset to zero and calls a routine through a pointer found in a structure.
- `compare_byte_at_offset`: Reads a pointer from a global base, compares the byte at offset 8 of the pointed structure to seven and returns if they differ.
- `execute_callback_sequence`: Calls a routine through a pointer in a structure and then calls another routine using a value from a global base.
- `check_printer_flag_byte`: Saves a6 on the stack, reads a byte from the structure pointed to by the value at offset 8 on the stack into d0 and returns when it is non-zero.
- `loop_printer_library_calls`: Calls a sequence of three library functions, then loops calling them while a status word at offset 332 of the structure in a6 stays zero, and otherwise branches back to repeat the loop.
- `compare_and_set_flag`: Compares the address of a structure at offset 94 in a6 with the long in d0 and returns early when they differ.
- `compare_structure_and_call`: Loads the address of a global structure into a1 and compares it with the value in d0, returning early when they differ.
- `call_and_branch_2`: Calls the routine at offset 908 and then branches to offset 94c, transferring control to another routine without returning.
- `save_registers_and_check_flag`: Saves registers d2 through d3 and a2 through a3 on the stack, compares the byte at offset 9b9 of the structure in a6 with one, and if they differ pushes the address at offset 1000 onto the stack before returning.
- `manage_interrupts_and_flags`: Sets a global variable to 16384, increments a counter byte at offset 294, and restores the previous value, decrementing the counter and setting the global to 49152 when it drops to zero or below.
- `clear_bits_conditionally`: Calls a routine through a pointer in a global base and, if the result is zero, clears bit 0 and then bit 1 of the byte at offset 2486 of the structure in a6, invoking the routine again when each bit was set, and returns one, otherwise it returns zero.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `call_and_dispatch`: Pushes a1 twice on the stack, calls the routine at offset 0, pops the stack back, then calls the routine at offset 240 and returns.
- `reset_and_update_field`: Clears a 32-bit field at offset 32 of the structure in a1 to zero, then overwrites it with the value at offset 36 when a helper returns zero.
- `save_and_call_vector`: Saves the value in a1 onto the stack twice, calls a routine through a vector, then restores the stack and a1.
- `call_printer_setup`: Pushes the a1 register twice onto the stack, calls a routine via a pointer, restores a1, then calls setup_printer_rectangle before returning.
- `populate_structure_and_call`: Calls a routine through a pointer in a global base and, if the result is zero, stores nine at offset 300 of that structure, copies two parameters into offsets 304 and 308, clears a byte at offset 302, then calls several routines through pointers in another global base and returns.
- `dispatch_printer_command`: Loads a pointer from offset 8 of the structure in a6 and sets up two table sizes, then calls a dispatch routine.
- `deduplicate_printer_buffer`: Copies bytes from a source table into a global table until the source reaches 256 or the count is exhausted, then calls another routine to search for and remove duplicate runs.
- `load_and_clear_globals`: Loads a long from a3 into a global variable at offset 184, clears the word at offset 182, then jumps to another routine.
- `translate_printer_output_3`: Copies the next byte from the printer output buffer into a translation table, then scans a byte table for escape sequences and converts them into printer output.
- `clear_registers_d2_d4_d6`: Clears the registers d4, d6, and d2, then branches to a routine that clears the stack pointer back to its original value and returns.
- `shift_table_entries_2`: Copies entries from a source table into a destination table, advancing both indices by one each iteration until the destination index reaches a limit, then stores the resulting value.
- `shift_table_entries`: Copies entries from a global table into itself, advancing a source index and a destination index by one each iteration until the destination index reaches the value stored at offset eight of the structure in a6.
- `clear_memory_offset`: Stores a zero at offset twenty of the structure pointed to by a register, then restores the caller's registers and returns a byte from a register.
- `decode_printer_state`: Loads a byte from a stack parameter and compares it against entries in a table, applying table-driven transformations based on the matched index and a flag byte, then loops over a count taken from offset 288 of the structure in d11.
- `clear_long_and_branch`: Clears the long at offset minus four from a6, then branches into another routine.
- `match_table_and_store`: Runs an outer loop that contains an inner loop comparing a byte against entries of a table indexed by a counter.
- `compare_and_store_marker`: Loads one into d0 and compares it with the long at the global variable, branching forward when they differ.
- `check_values_and_store_byte`: Loads two into d0 and compares it with the long at the global variable, returning when they differ.
- `clamp_and_copy_byte_2`: Loads the value nine into a register and compares it with the value in d5, returning when they differ.
- `clamp_and_copy_byte`: Loads the value six into a register, compares it with another value, and when it is greater copies a byte from an indexed table into a location pointed to by a register.
- `validate_and_clamp_printer_state`: Verifies that the byte stored at offset 29 of the structure pointed to by a4 is within the range 0 to 29, and if it is not, it stores the value 12 at offset 12 of the structure pointed to by a2.
- `match_and_decode_printer_state`: Matches a byte against a seven-entry table and stores the matching index plus a table value through a pointer, then repeatedly advances a counter until it exceeds twelve and decodes the byte through another routine, shifting a nine-entry table and incrementing a counter in a structure, returning early if that counter grows too large.
- `translate_printer_output_2`: Scans a byte table looking for escape sequences and translates them into printer output, writing converted characters to a destination buffer while tracking how many bytes were written and whether the buffer is full.
- `decode_printer_output_2`: Reads the next byte from a global table and increments its index, then interprets escape sequences: on the escape marker it fetches a following byte and decodes a font selection, on the bracket marker it invokes the printer handler, on a newline it appends a carriage-return-newline pair to the output buffer when space allows, and on bytes below 160 it copies them directly to the output buffer.
- `decode_printer_output`: Decodes a byte stream from the input buffer at a3 into the output buffer at a2, translating escape sequences and control characters such as backspace, carriage return and line feed, looking up printable characters in a table, and returning the number of bytes written or -1 when nothing was decoded.
- `compare_byte_branch`: Compares a byte at the address in a3 with the decimal value 91, and when it does not match it returns, otherwise it branches into another routine.
- `translate_printer_buffer`: Translates a printer control buffer into a destination buffer, copying printable characters, converting line feeds to CR/LF pairs, and expanding escape sequences from a lookup table, calling a helper routine to write each produced byte.
- `compare_byte_store`: Compares a byte at the address in a3 with the decimal value 10, and when it matches it branches into another routine, otherwise it tests a byte at offset 260 and branches when nonzero, then stores the decimal value 3 into the byte pointed to by a4.
- `process_printer_output_2`: Writes a byte to the printer device buffer, then processes the source text: on a line feed it appends a line feed and carriage return unless the buffer is nearly full, and it decodes escape sequences (ESC 3, ESC (, ESC [) and control codes by translating them through lookup tables before copying ordinary bytes to the printer buffer.
- `write_printer_newline`: Writes a carriage-return byte (value 10) to the printer device at the address formed by the value in a2 plus the offset in d2, then increments the 32-bit counter stored at a2.
- `store_newline_via_pointer`: Reads the 32-bit value stored at the address in a2 into d1, increments that same 32-bit value by one, restores the original value into a0, adds the 32-bit value in d2 to it, writes the byte 10 to the memory location pointed to by the updated a0, then branches to the following code.
- `check_escape_call_routine`: Compares a byte at the address in a3 with the escape character, and when it matches, pushes two registers onto the stack, calls a routine, pops the registers, and branches into another routine.
- `process_printer_buffer`: Processes a printer command buffer byte by byte, copying printable characters and interpreting escape sequences, then returns the number of bytes written.
- `parse_printer_device_string`: Parses a printer device string, copying characters into a buffer in a2 while converting backslash-digit escape sequences into their numeric values, and handling ESC and # markers by looking up and storing values from a table.
- `advance_and_validate_pointer`: Advances a pointer and a global counter, returning early when the pointed byte is zero.
- `index_table_from_byte`: Reads a byte from the location pointed to by a3, subtracts one hundred and sixty from it, multiplies the result by four, and uses it as an offset into a pointer in a global table.
- `check_ascii_digit_range`: Compares a byte at a pointer with the value ninety-two and branches away when it differs, then checks whether the next byte is below forty-eight or above fifty-seven and branches away when it is.
- `parse_digits_and_copy_table`: Converts up to three consecutive ASCII digits from the input string into a single numeric byte and stores it in a byte array whose index is incremented.
- `copy_byte_from_source`: Reads a pointer from the location pointed to by a2, increments that pointer, and adds the value in d2 to a copy of the old pointer.
- `copy_and_translate_printer_output`: Copies the next byte from a source table into a destination buffer and advances the source index, then interprets escape sequences by reading follow bytes and translating them through a lookup table.
- `update_global_values`: Subtracts the difference between two global variables from a third global variable and clears a fourth global variable.
- `copy_buffer_with_length_check`: Clears a global byte at offset 128 and calls a helper to read a length into a 256-byte stack buffer, then returns that length.
- `add_and_store_global_long`: Adds a register value to a global long, stores the sum in another global, and when the result exceeds two hundred and fifty-six it tests a byte and calls a routine before falling through.
- `decrement_global_counter`: Loads a global counter from offset 222, decrements it by one, stores the result back to offset 274, sets d0 to one, and falls through into the following routine.
- `clear_register_fall_through`: Sets d4 to zero and then falls through into the following routine.
- `copy_memory_block_2`: Copies bytes from a source address into a table, advancing both pointers for each byte, and stops when a counter reaches a limit.
- `restore_regs_and_call`: Restores the caller's registers from the stack, loads two long values and a pointer from the stack into registers, clears a byte at the pointer, then jumps into another routine.
- `clear_byte_on_match`: Compares the value in d1 with the value fifty-nine and returns when they differ.
- `store_one_when_match`: Compares a register with the value thirty-two and, when they match, loads a pointer from the stack, sets a register to one, and stores that value through the pointer.
- `compare_and_branch_register`: Compares d1 with the value 34 and, when they differ, falls through into the following routine; otherwise it loads a pointer from offset 32 of the stack frame, sets d0 to two, and jumps into the following routine.
- `parse_number_from_buffer`: Multiplies the byte at the computed address by ten and adds the digit in d1, storing the result back.
- `index_table_and_check_limit`: Reads a value through a pointer, increments the pointed-to value, indexes a global table with the original value, and returns zero when the incremented value is exceeded or a flag byte is nonzero.
- `return_register_value`: Returns the value currently held in a data register.
- `decrement_store_counter`: Loads a global counter from offset 222, decrements it by one, stores the result at the address in a1, and falls through into the following routine.
- `decrement_and_check_table`: Decrements a value pointed to by a register while it stays positive and a masked table entry is nonzero, then sets a global flag and returns the decremented value.
- `reset_global_state`: Clears three global variables and sets a byte at offset 1728 of a structure in a1 to zero, then clears a byte at offset 2812 of a structure in a9.
- `copy_data_to_globals`: Copies six 16-bit values from offsets 2646 through 2654 and offset 2666 of a structure in a global base into six consecutive 16-bit global variables.
- `compare_values_and_return`: Compares six 16-bit values read from offsets 2646 through 2654 and offset 2666 of a structure in a global base with six corresponding global variables and returns zero when all match, otherwise it returns minus one.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `read_params_and_dispatch`: Copies four bytes from offsets 22 through 25 of the input structure onto the stack and reads a word from offset 20.
- `dispatch_conditional_routines`: Initializes a result to zero, then calls another routine with different parameters whenever the result is zero or a register is zero, and finally returns its low byte.
- `copy_buffer_and_handle_markers`: Calls a helper with the first three arguments and, when it returns zero, handles special marker values by storing a bit in a global byte or copying characters from a table buffer into the destination.
- `store_printer_marker_bit`: Compares the word at offset 22 on the stack with 54 and returns when they differ, then compares it with 53 and branches away when it is not equal.
- `copy_buffer_with_markers_2`: Adjusts a stack value when a global flag is set, looks up a pointer in a table using that value, then copies characters from a source buffer into a destination buffer, converting negative markers and stopping at a zero byte.
- `clamp_d1_to_255`: Compares the byte in d1 with 255 and, if they differ, loads 255 into d1.
- `copy_buffer_with_markers`: Copies characters from a source buffer into a destination buffer one byte at a time, clamping a special marker to zero, returning negative one if a stop marker is encountered, and storing an extra byte when the destination length matches one of three specific values.
- `index_jump_table`: Pushes the value of d2 onto the stack, reloads it from the stack, saves a pointer from the stack into a0, then uses d1 and d0 to index a jump table and branches to the selected routine.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `setup_printer_rectangle`: Loads printer configuration fields from a structure, calls compute_printer_rectangle to validate the mode and dimensions, then computes the printer rectangle's width and height from the source coordinates or from the printer device's bounds and stores them in global variables.
- `compute_printer_rectangle`: Validates the printer mode byte and several dimension fields, then computes the printer rectangle's width and height from coordinate values, returning 4 if any dimension is invalid or empty.
- `adjust_printer_rectangle`: Computes the width and height of a rectangle from coordinates in a structure, validates them against stored values, then adjusts and swaps the rectangle's coordinate pairs based on flag bits in a byte at offset 138.
- `select_printer_sub_interval`: Reads a printer structure and its mode byte, then returns 4 if the dimensions are invalid or zero.
- `compute_32bit_product`: Computes the 32-bit product of the values in d0 and d1 using 16-bit partial products and combines them into a 64-bit result, then returns.
- `compare_and_branch_2`: Compares the value in register d0 with the decimal constant 2048 and, if they are not equal, branches to the routine at address 352; otherwise it branches to the routine at address 34e.
- `set_value_and_branch`: Loads the decimal constant 12 into register d1 and then branches to the routine at address 354.
- `set_value_and_branch_2`: Loads the decimal constant 15 into register d1 and then branches to the routine at address 354.
- `update_printer_state`: Compares two 32-bit values from a table in d5 with the values pointed to by d4 and a4, storing the smaller non-zero result at offsets -10 and -14 of the structure in a6.
- `swap_and_rotate_printer_state`: Swaps two 32-bit values pointed to by the two registers, rotates a status byte's bits by one position, and then applies bit-masked shifts and reloads to each value based on the rotated flag bits.
- `store_value_and_check_flags`: Loads the value seven into a register, stores it on the stack, and checks a byte at offset 360 of a structure in a6.
- `check_flags_then_branch`: Clears a byte at offset 268 of a global variable and checks a byte at offset 350.
- `normalize_64_bit_product`: Swaps the two 32-bit halves of a 64-bit product held in memory, applies sign correction and arithmetic right shifts based on a bit pattern, then compares the resulting high and low halves against two values in a structure and stores the smaller of each pair into a pair of 16-bit fields.
- `compute_64_bit_product`: Computes a 64-bit product of the values in d0 and d1 using 16-bit partial products, then returns the result split across d0 and d1.
- `multiply_32_bit_integers_7_2`: Computes a 32-bit product of the values in two registers using 16-bit partial products and combines them into a 64-bit result, then returns.
- `setup_printer_palette_2_2`: Builds a printer palette table by splitting each of 64 entries into four nibbles across three tables, optionally inverting the bit planes and clearing the low nibble of each entry when a flag is unset, then returns.
- `setup_printer_palette_2`: Builds a printer palette table by splitting each of 64 entries into four nibbles across three tables, optionally inverts the bit planes, then processes the palette through a helper routine and returns the resulting handle or falls back to a default handle.
- `setup_printer_palette`: Splits each of the 64 palette entries into four nibbles and stores them in three separate tables, then applies a bit-plane inversion when a flag in the printer structure is set.
- `initialize_printer_palette_2_2`: Builds a 64-entry printer palette table by splitting each 16-bit palette value into four nibbles stored in three separate tables, then adjusts the entries depending on a flag at offset 43 of the printer structure and a bit at offset 21 of the palette base, and finally opens the printer device.
- `initialize_printer_palette_2`: Fills a 64-entry palette table with values from another routine, splitting each 16-bit result into four nibbles stored in three separate tables, then inverts selected entries and applies a second transformation before returning.
- `initialize_printer_palette`: Fills a 64-entry table with palette values derived from a source structure, applies a bit-mask inversion controlled by a status byte, then repeatedly opens a printer device until it succeeds or a counter is divisible by a value read from that structure.
- `setup_printer_device`: Initializes several global variables, calls a routine to open a printer device, and stores the returned handle.
- `open_printer_device`: Pushes a printer device name onto the stack, sets up a printer open structure with zeroed fields, calls the printer open routine, stores the returned device pointer in a global variable, and returns.
- `check_flags_then_call`: Tests the byte at offset 360 of the structure in a2 and jumps away if it is zero, tests the byte at offset 360 and jumps away if it is zero, calls a routine, then jumps to another routine.
- `test_byte_then_call_routine_2`: Tests the byte at offset 360 of the structure in a2 and returns if it is zero, otherwise it calls a routine, then jumps to another routine.
- `test_byte_then_call_routine`: Tests the byte at offset 360 of the structure in a2 and returns if it is zero, otherwise it calls a routine and then jumps to another routine.
- `store_result_and_branch`: Calls a routine, stores the returned value in a global variable at offset 366, and returns if that value is zero otherwise it jumps to another routine.
- `check_divisibility_and_call`: Checks whether the byte at offset 324 of a global variable is divisible by the 16-bit value at offset 24 of a structure in a5, calls a routine through a pointer with two zero arguments, and if that call returns zero it passes zero to another routine and returns.
- `call_empty_routine`: Calls another routine with two zero arguments and returns.
- `reduce_byte_modulo_251`: If a global variable is zero and bit 2 of a byte at offset 21 of a structure in a1 is clear, it returns the parameter unchanged.
- `find_minimum_byte`: Checks bit 2 of the byte at offset 21 of a global variable and, if set, finds the smallest of three bytes packed in the third parameter and returns it, otherwise it returns the original second parameter.
- `select_and_store_print_value`: Computes a value from a global variable and several global constants, calls a print routine to select a table entry, then stores the selected value into a structure while handling different nibble combinations and updating an accumulator.
- `select_table_entry_and_store`: Calls a formatted print routine and stores the returned index into the pointer in a2, then selects a table entry using the low or high nibble of that index depending on a flag byte at offset 21 of the global base.
- `index_table_and_accumulate`: Loads a long from a table indexed by a nibble of d3 into the pointer in a2 when a global variable is zero.
- `store_long_and_shift`: Stores the long in d2 into a global long, loads a device pointer, and tests bit 3 of a status byte, returning early when that bit is set.
- `shift_and_compare_values`: Copies the word in d2 into d1 and the word in d3 into d0, then shifts d0 right by four bits.
- `compare_d0_to_two`: Compares d0 with 2 and branches forward when they are equal, otherwise falling through to the next routine.
- `decrement_d0_and_branch`: Decrements d0 by one and branches when it is nonzero, otherwise falls through to another routine.
- `store_byte_and_branch_2`: Stores the byte in d1 into a global byte at offset 272, then branches to another routine.
- `store_byte_and_branch_offset_112`: Stores the byte in d1 into a global byte at offset 274, then branches to another routine.
- `store_byte_and_call_helper`: Stores the parameter in d1 into a global byte at offset 17, then pushes a long from a2 onto the stack before calling a routine and storing its result into a2.
- `update_pointer_table`: Loads a 32-bit value from a table indexed by a shifted byte into a pointer, then adds a global base address to four other global pointers.
- `compute_and_update_globals`: Multiplies two global bytes and stores the product in a global at offset 24, or stores their sum when the product is zero, then calls a helper routine four times to fill four adjacent globals.
- `process_printer_entries`: Clears a global byte, then runs a sequence of helper calls and library stubs for four indexed entries, continuing only while each step returns zero.
- `process_printer_table_entries`: When two global bytes are both zero it calls a helper with a table pointer and two buffer pointers, and when the result is nonzero calls another routine with an index and stores its value into a destination.
- `store_routine_result`: Calls a helper routine with three arguments and, when the result is nonzero, calls another routine with four arguments and stores its result.
- `call_and_store_result_2`: Calls a helper routine with three arguments and, when the result is nonzero, calls another routine with four arguments and stores its result into a pointer before returning it.
- `clear_and_restore_registers`: Clears the return value register and restores the caller's registers from the stack before returning.
- `increment_and_check_counter`: Increments a global counter and returns zero when it is divisible by a value from a table, otherwise calls a routine twice and returns zero only when both results are zero.
- `call_indexed_routine`: Pushes the address at offset 300 of the global data onto the stack, clears three longs on the stack, loads the routine pointer at offset 396 into a0, and calls it.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `setup_globals_and_jump`: Saves registers a2 through a4 on the stack, loads the addresses of three global variables into a2, a4 and a3 respectively, then jumps to another routine.
- `accumulate_scaled_values_loop`: Calls a routine with two global values and a pointer, stores the returned short at the address in a3, and resets several global fields.
- `scan_table_store_values`: Calls a routine with two global shorts and a pointer to a global variable, clears four global shorts at offsets 200, 208, 204 and 212, then loops over a table via a pointer in a3 while decrementing a counter and storing each returned value into a location.
- `clear_offset_316`: Clears the short at offset 316 of the global data, then branches to a routine that clears the stack pointer back to its original value and returns.
- `scan_table_store_value`: Increments a global short at offset 316, then repeatedly scans a table via a pointer in a3 and stores values from another routine into a location pointed to by a2, returning that stored value when it is nonzero.
- `scan_table_store_values_2`: Increments a global short, updates a global long, and when two globals match stores a result into a pointer, adding to another pointer and returning zero when a limit is reached, otherwise it continues.
- `accumulate_and_compare_globals`: Adds the word at the address in a3 to the word at the address in a4, then compares the resulting word at the address in a4 with the global short at offset 300 and, when that value is greater, it clears registers d0 and the pointers a2 through a4 before returning.
- `clear_total_and_jump`: Saves registers a2 through a4 on the stack, loads the addresses of three global variables into a2, a4 and a3 respectively, clears a word at offset 316 of a global variable, stores a word from a global variable at offset 380 into another at offset 320, then jumps to another routine.
- `accumulate_scaled_values_loop_2`: Calls a routine with two global values and a pointer, stores the returned short at the address in a3, and resets several global fields.
- `clear_globals_branch`: Clears the four global shorts at offsets 200, 208, 204 and 212 with zero, clears the short at offset 340, then branches into the middle of another routine.
- `loop_store_table_values`: Increments a global short, then loops over a table via a pointer in a3, decrementing a counter and storing each returned value into a location pointed to by a2 until the counter reaches zero.
- `call_and_store_result_3`: Calls a helper routine and stores its long result into a global long, returning early when that result is zero.
- `scan_table_store_result`: Increments a global short, decrements another global, and when it falls below one adds to a global long, then stores a result into a pointer and returns zero when a limit is reached, otherwise it continues.
- `clamp_running_total_to_300`: Adds a 16-bit value read from the structure in a4 to a running total stored at the address in a4, then clamps that total to a maximum of 300 by resetting it to zero if it exceeds that limit, and finally returns.
- `clear_accumulator_and_jump`: Saves registers a2 through a4 on the stack, loads the addresses of three global variables into a2, a4 and a3 respectively, clears a word at offset 340 of a global variable, stores a word from a global variable at offset 380 into another at offset 320, then jumps to another routine.
- `call_scaled_accumulator`: Pushes the address at offset 326 of the global data onto the stack, pushes the short at offset 304 followed by the short at offset 302, and calls that routine.
- `clear_and_copy_globals`: Clears the short at offset 226 of the globals, copies the long at offset 334 into the long at offset 330, copies the long at offset 16 into the long at offset 276, and copies the short at offset 378 into the short at offset 314.
- `store_routine_result_2`: Writes the value returned by another routine into a 32-bit global at offset 308, clears four adjacent 32-bit globals, then repeatedly calls that routine until it returns a non-zero value.
- `compare_globals_loop`: Calls a routine, increments a global short at offset 316, loads a global short into d1, and compares it against another global short, branching back to the start when the first is greater.
- `scan_table_store_result_2`: Increments a global short, updates a global long, and when two globals match decrements a pointer, then repeatedly increments another pointer and returns zero when a limit is reached, otherwise it continues.
- `decrement_loop_accumulate_compare`: Decrements the word at the address in a3 and, while that value stays positive it increments the word at the address in a4, compares the word at the address in a4 with the global short at offset 300, and when that value is greater it clears registers d0 and the pointers a2 through a4 before returning.
- `init_globals_from_constants`: Saves the pointers a2 through a4 on the stack, loads the decimal constants 366, 320 and 338 into a2, a4 and a3 respectively, clears d0 and stores it at the global short at offset 316 and at the global short at offset 340, copies the global short at offset 380 into the word at the address in a4, and then jumps to another routine.
- `store_printer_status_return`: Pushes the 16-bit values from the globals at offsets 208 and 190 onto the stack as two 32-bit arguments, calls the routine at offset 5226, stores the returned 16-bit value into the word pointed to by a3, then pops those arguments off the stack and returns.
- `copy_globals_and_branch`: Clears a global short, copies a global long into another global, copies a global long into a third global, copies a global short into a fourth global, and branches into another routine.
- `loop_store_table_values_advance`: Clears four global shorts, then loops over a table via a pointer in a3 while decrementing a counter and storing each returned value into a location pointed to by a2, advancing a pointer in a4 when a condition matches.
- `store_and_branch_result`: Calls another routine, stores the returned value into the longword pointed to by a2, and if that value is nonzero it loads the same longword back into d0 and jumps to another routine; otherwise it returns.
- `decrement_counter_loop`: Increments a global short at offset 322 and decrements a global short at offset 318, then when the latter is less than one it adds a global short to a value, loops while two values match and stores a returned value into the longword pointed to by a2.
- `decrement_and_compare_globals`: Decrementing the short at the address in a3, it returns if that value stays nonnegative.
- `accumulate_scaled_values`: Repeatedly adds a doubled parameter to a value pointed to by a pointer while that value stays below one, then subtracts twice a second parameter from it and returns the iteration count.
- `compare_byte_or_borrow`: Indexes a byte from a table using the low two bits of the second and third parameters, compares the first parameter with that byte, and returns whether they differ and the borrow matches the unsigned comparison.
- `compare_table_value`: Indexes a byte from a table using two masked values and returns whether that byte is less than a provided byte.
- `call_two_helpers_in_order`: Calls two helper routines in sequence and returns without storing any value.
- `divide_32bit_values`: Performs 32-bit division by a 16-bit divisor, using a sixteen-iteration loop that shifts a remainder and quotient when the partial remainder exceeds the divisor, and otherwise falls back to the hardware divide instruction.
- `rotate_32bit_value_2`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_7`: Computes the 32-bit product of the two 32-bit values in d0 and d1 using 16-bit partial products, combining them into a 64-bit result whose low half is returned in d0 and high half in d1.
- `forward_call_to_helper_2`: Calls one helper and returns, with no other work.
- `call_helper_return_value`: Pushes d2 onto the stack, shifts d1 into d2 and d0 into d1, calls a helper routine, restores d2 from the stack, and returns with the helper's result in d0.
- `negate_result_on_flag_4`: Calls a helper routine, then negates the returned value when the flag left in a0 is nonzero, and returns the resulting value.
- `call_and_negate_on_flag_3`: Calls a helper routine, takes its result from d1, and returns that value or its negation depending on whether the flag left in a0 is nonzero.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `find_string_match`: Loads a 16-bit value from offset 2614 of the structure in d1, then uses it as an index to select a pointer from a table at offset 2614 or from the fixed address at offset 2616 when the value is zero.
- `index_table_and_branch`: Multiplies a byte index by four and uses it to load a pointer from a table, then branches into another routine to continue processing.
- `store_byte_advance_pointer_4`: Branches to a routine that stores the byte in d0 through the pointer in a3 and advances that pointer, then returns.
- `load_value_into_a2`: Loads the decimal value 32 into a2, then branches into another routine.
- `compare_strings_and_store`: Increments a pointer and compares characters through two pointers in a loop, exiting when the comparison fails or the source string ends.
- `load_segment_and_copy_string`: Clears a pending flag and its associated word when set, then loads a segment and stores the returned handle along with a shifted value.
- `unlock_and_get_directory`: Calls a routine to unlock a resource using a pointer from a register, then calls another routine to retrieve the current directory, and returns negative one.
- `execute_callback_and_recurse`: Checks a global value and, if it is nonzero, calls a function pointer from a table, then calls itself with that value and returns.
- `normalize_and_compare_chars`: Lowers each of two parameters by thirty-two when they fall between the backtick and the open brace, then returns whether they are equal.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_3`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### narrator.device

- `init_device_request_queue`: Reads a pointer from a global structure and calls a routine at offset negative 84 to obtain a request block, then fills fields of that block including a pointer to the global structure, a message pointer, and a zeroed next pointer.
- `check_d0_then_call_handler`: It saves registers a2, a3 and a6 on the stack, tests the long in d0, and when that value is zero it loads the decimal constant -5 into d0 and branches to the routine at offset 416.
- `update_device_state_fields_3`: Checks a short counter at offset 32 of the structure in a6 and, when it is zero, stores a status byte at offset 31 of the structure in a1, writes a long at offset 20, and otherwise increments two counters in the structure in a6 before filling the structure in a1 with fixed values.
- `write_state_and_call_vector`: Writes the constant 5 to byte 176 of the structure in a6, stores a pointer to a global variable at byte 178, copies a long from offset 244 to offset 182, and calls a routine through a vector in the structure at offset 164, then branches on its result.
- `update_device_state_fields`: It stores an input value into a structure, increments two counters within it, copies one counter to another structure, writes several constant words and bytes to the second structure, and clears a flag bit in the first structure.
- `store_byte_and_branch`: Stores the byte in d0 into a structure at offset 31, writes the constant -1 into a location at offset 20, and then branches into another routine.
- `clear_flags_and_call_handler_3`: Saves registers a3 and a6 on the stack, loads pointers from the structure in a1 and a global variable, clears a flag byte at offset 30 of the structure in a1, inspects a byte at offset 34 of the structure in a3, and then calls a routine through a vector before jumping to another location.
- `link_structures_and_enable_interrupts`: Compares a long at offset 24 of the structure in d0 with a long in the structure in a1, and when they match and a flag byte at offset 44 of the structure in a3 is clear, copies a byte between the structures and splits it into two nibbles.
- `update_device_state_fields_2`: If a flag byte is clear and two compared bytes differ, it copies a byte between structures, splits it into low and high nibbles at two offsets, clears another byte, and calls another routine.
- `copy_byte_and_split_nibbles`: It copies a byte from one structure to another, splits it into low and high nibbles into two bytes, clears a byte in the destination structure, and calls another routine before returning.
- `update_and_remove_narrator_entry`: Increments a byte at offset 294 of a global structure, then walks a linked list until it finds a matching entry to remove it, otherwise it decrements that byte and stores the incoming pointer into a structure.
- `decrement_and_call_narrator`: Decrements a byte counter in a global variable and, while it is non-zero, disables interrupts, returns the saved address from the stack, and calls a routine to store a narrator request.
- `decrement_counter_and_close_2`: Walks a linked list of pointers stored in a global variable, comparing a value from offset 24 of the structure in a1 with each entry and calling a routine when they match, then decrements a counter at offset 32 of the structure in a6 and calls two routines through vectors when it reaches zero.
- `decrement_counter_and_close`: Decrements a 16-bit counter at offset 32 of the structure in a6 and, when it reaches zero, calls two routines through vectors in a global variable.
- `index_and_call_handler`: Reads a word from a structure at offset 28, and if it is negative or greater than 8 it returns immediately.
- `save_registers_check_flag`: Saves registers on the stack and copies a structure pointer into another register, then checks a byte at offset 32 of that structure and, when it is zero, sets bit 5 of the byte at offset 34 before jumping to a shared routine.
- `restore_registers_return`: Restores the caller's registers and returns.
- `dispatch_narrator_command`: Stores 5 at offset 2 of the structure in a1, sets and clears bit 6 of its byte at offset 30, then dispatches by the 16-bit value at offset 7 to call a handler, set a flag and return.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `clear_status_and_call_handlers`: Clears bit 6 of the status byte at offset 34 of the structure in a0, sets a byte at offset 196 of the structure in a6 and sets bit 0 of the byte at offset 198, then calls two handlers from that structure and clears a flag byte at offset 31 of the structure in a1.
- `clear_flags_and_call_handler_2`: Clears a flag bit in a structure, clears a byte in another structure, and calls a handler via a base pointer.
- `set_flag_then_clear`: The routine sets bit 7 in a byte at offset 34 of a structure, calls two other routines, then clears bit 7 in the same byte.
- `set_flag_clear_byte_then_call`: It sets bit 7 in a byte at offset 34 of the structure in a0, calls another routine, clears bit 7 in that same byte, writes zero to a byte at offset 31 of the structure in a1, and calls another routine.
- `decrement_counter_and_set_global_3`: Calls a routine when a long at offset 772 of the structure in a6 is nonzero, sets a global variable to a constant, increments a byte at offset 294 of that global variable, walks a linked list of pointers, then decrements that byte and writes a constant to the global variable when the previous value was small.
- `decrement_counter_and_set_global`: Decrements a byte in a global variable and, if the previous value was less than one, writes a constant to a global variable.
- `save_registers_and_set_flag`: Saves registers a3 and a6 on the stack, copies a6 into a3 and loads a global pointer into a6, then writes an interrupt-enable value to the hardware register and increments a counter byte at offset 294 of the structure in a6.
- `load_pointer_compare`: Loads a pointer from the structure in a0 and the long it references, returns immediately when that value is zero, and otherwise compares it with the pointer in a1, returning when they match.
- `swap_pointers_and_disable_interrupts`: Swaps two pointers held in a structure and a register, sets a byte in that structure, decrements a byte in another structure, and disables interrupts when the previous value was less than one.
- `decrement_counter_and_set_global_2`: Decrements a byte in a structure pointed to by a register and, if the previous value was less than one, writes a constant to a global variable.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `write_byte_and_call_handler`: Writes a constant byte to a structure, then calls a handler routine.
- `clear_byte_and_call_handler`: Clears a byte in a structure, then calls a handler routine.
- `clear_flags_and_call_handler`: The routine clears a flag bit in a structure, clears a byte in another structure, and calls a handler via a base pointer.
- `store_narrator_request`: Stores a pointer value at offset 228 of the structure in d1, then validates a series of fields from the structure in a1 and copies them into the structure at offset 228, otherwise it returns early.
- `clamp_and_index_text`: Clamps a 16-bit value from d1 to a maximum of 320 and stores it at offset 30 of the structure in a5, then computes a byte index from a 32-bit counter and searches backward through a text buffer for a comma, hyphen or space.
- `clamp_and_select_index`: Compares the word in d1 with 320 and jumps away if greater, otherwise stores d1 at offset 28 of the structure in a5, loads the word at offset 52 of the structure in a2 into d1, and returns if that value is zero or one, otherwise it loads 232 into d3 and jumps to another routine.
- `store_and_compare_threshold`: Stores the word in d1 at offset 216 of the structure in a5, then compares the word at offset 64 of the structure in a2 with 27968 and branches back to the start when it is less.
- `store_value_if_large`: Stores the word in d1 at offset 216 of the structure in a5, then compares the word at offset 64 of the structure in a2 with 27968 and branches to a later routine when it is greater or equal, otherwise it loads 231 into d3 and branches to a shared routine.
- `store_narrator_value_if_large`: Compares a 16-bit value in d1 with 27,968 and, if greater, stores it at offset 216 of the structure in a5.
- `process_narrator_byte_stream_3`: If the byte in d1 is 64 or larger it stores zero at offset 964 of the structure in a1, then stores the signed value from another routine at offset 52 and its low byte at offset 51 before returning.
- `locate_text_segment_boundary`: Computes a byte index from a 32-bit counter in a structure, searches backward through a text buffer near offset 96 for a comma, hyphen or space, and stores the resulting position and a cleared counter into other fields of that structure.
- `copy_long_and_jump`: Copies the long at offset 36 of the structure in a1 to offset 32 of that same structure, then jumps into another routine.
- `clear_long_store_byte_and_call_handler`: Clears a long at offset 32 of a structure, stores a byte from a register at offset 31, then calls a handler routine.
- `find_text_segment_marker`: Walks backward through a text buffer starting near offset 96 of the structure in a5, stopping at the first comma or hyphen and recording that position, otherwise it stops at the first space and records that position.
- `scan_table_with_counter`: Reads a long at offset 150 of the structure in a5, computes a scaled value from it, and uses it as a loop counter to scan a byte table.
- `shift_bit_and_clear`: Reads a long at offset 150 of the structure in a5, shifts it right by one bit, stores the result back at offset 156, clears the original long, and jumps into another routine.
- `init_narrator_request`: Clears several fields in two structures, writes the bytes 75 and 65 into two of them, copies a long and a word from one structure into the other, then calls two routines.
- `copy_long_and_call_vector`: Copies a long value from the structure in a1 to a byte in the structure in a2 at offset 67, then calls a routine through a vector in the structure in a6 and checks the returned long in d0.
- `init_struct_and_copy_tables`: Initializes a structure pointed to by d0 with a constant and a global pointer, then calls two routines through vectors in the structure in a6 and stores their results back into it.
- `scale_byte_and_call_library`: Stores the byte in d0 at offset 68 of the structure in a2, scales it by four, and calls the library function at offset minus 198 of the base in a6, then returns when the result is nonzero.
- `copy_word_table_to_buffer`: Stores a pointer in a global variable and copies 34 words from a table in a global variable to a buffer pointed to by an argument, writing each word twice per iteration, repeating for a count derived from a byte in a global variable.
- `set_and_test_bits`: Sets bit 1 of a register, tests a bit in another register, and masks two other registers, then falls through to the next routine.
- `handle_io_and_free_resources`: Writes a byte from a structure field through a pointer and stores the word 9 at the following field, then calls a routine via a global variable and conditionally calls two other routines via the same global variable depending on two flags, finally clears the word at offset 200 of the structure and returns a combined value.
- `test_offset_check_zero`: Tests a word at offset 206 of the structure in a5, and when it is greater than zero, loads zero into d3 and returns.
- `process_narrator_character`: Initializes a structure by clearing several 16-bit and 32-bit fields and setting a byte at offset 21, then processes the next character from the buffer in a0, decoding it via a lookup table and storing the result into the structure's fields, or returns early when the character is a space, a hash, or out of range.
- `check_stream_offset`: Adjusts a counter, compares it against twelve thousand, and returns early when the counter is out of range, otherwise branches back to continue decoding the stream.
- `decode_narrator_text_stream_2`: Decodes the next character from the narrator text stream in a0, translating control markers into attribute bytes and writing the result to the output buffer, then calls decode_narrator_text_stream to continue until the end marker is reached.
- `decode_narrator_text_stream`: Decodes a byte stream from a buffer into a text buffer, tracking character and byte counts, and returns when the end marker is reached.
- `compare_byte_and_set_bit`: Compares the byte in a register with the value 5 and returns immediately when they differ, otherwise sets bit 5 of a byte addressed by a register and a word offset, then jumps into another routine.
- `compare_and_set_bit`: Compares the byte value 6 with the byte in d1 and returns immediately when they differ, otherwise sets bit 4 of the byte at the address one less than a6 indexed by d3 and jumps to the code at the destination.
- `copy_byte_range_loop`: Copies a byte from a source table to a destination table while incrementing a counter, then branches back into the copy loop until the counter reaches 512, after which it stores a pointer from the structure and sets a flag.
- `compare_offsets_and_clear_bytes`: Computes the difference between two addresses from the structure in a5, compares it with a value from the structure in a5, and clears two bytes indexed by d3 unless the comparison fails.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends immediately.
- `read_byte_and_branch`: Loads the constant 0 into d0, reads a byte from the structure in a2 indexed by a3, and branches when that byte is zero.
- `index_table_and_store_bytes`: Indexes a table by a register value and, when a bit is clear and the counter is greater than three, writes bytes to three tables at that offset and increments the counter, then unconditionally writes bytes to all three tables and stores the incremented counter.
- `store_word_and_set_ccr`: Stores the word in a register into a field at offset 154 of a structure, sets the condition code register to 8, and returns.
- `process_string_and_set_flags`: Walks through a null-terminated string in the structure in a5, skipping null bytes, and returns when it reaches a byte with the value -1.
- `mask_register_set_flag`: Masks a register and returns immediately when the result is zero, otherwise sets bit 4 of a byte pointed to by a0 and jumps into another routine.
- `check_table_and_set_flag`: Reads a word from a table indexed by a register, returns immediately when the masked value is zero, returns when a byte does not match a constant, otherwise sets bit 4 of a byte pointed to by a0 and jumps into another routine.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `process_narrator_byte_stream_2`: Walks through a byte stream in the buffer pointed to by a0, copying bytes to the buffer pointed to by a1 and setting bit 7 of each copied byte.
- `process_narrator_stream`: Walks through a stream of bytes in the buffer pointed to by a0, copying them to a second buffer pointed to by a1 while setting bit 7 of each copied byte and bit 6 of bytes whose source value has bit 4 set.
- `process_narrator_byte_stream`: Walks through a byte stream in the buffer pointed to by a0, copying bytes to the buffer pointed to by a1 and setting bit 7 of each copied byte.
- `subtract_and_decrement`: Subtracts the word in d2 from the word in d0 into d5, decrements d5 by one, then branches back to the earlier routine at 4646.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `validate_narrator_script_sequence_2`: Walks through a byte buffer using a 16-bit index, matching each byte against a sequence stored in a structure and advancing until it finds a byte equal to -1.
- `validate_narrator_script_sequence`: Compares a sequence of bytes from the buffer in a0 against expected values stored in the structure in a1, advancing through the buffer and calling another routine for each byte.
- `increment_and_check_byte`: Increments the counter in d1 by one, reads the byte at offset 6 of the structure in a0 into d3, and returns when that byte equals 255.
- `test_bit_in_struct`: Tests bit 5 of the byte at offset 3 in the structure pointed to by a0 and branches away if that bit is set, otherwise it jumps to a shared routine.
- `test_bit_with_table_lookup`: Loops reading a byte from a buffer and a word from a table indexed by a shifted word, modifies the word based on bits in the byte, and exits when a specific bit is set or a termination condition is met.
- `check_counter_disable_interrupts`: Loads a word counter from offset 154 of the structure in a5, compares it with the decimal value 512, and disables interrupts by setting bit 2 of the condition code register when the counter is below that value.
- `shift_bytes_and_clear_fields`: Increments a word counter in a structure and shifts bytes backward in three buffers by a computed count, then writes a byte from a register and two zero bytes to the new ends of the buffers.
- `call_helpers_clear_d0`: Calls seven helper routines in sequence, clears register d0, and returns.
- `compare_and_set_byte`: Loads a pointer from offset 28 of the structure in a5 into a6, sets d1 to 255, and returns when the byte at that pointer differs from d1.
- `clear_struct_byte_advance`: Clears the byte at offset 7 of the structure in a6, advances that pointer by eight bytes, then branches back to a shared routine.
- `read_byte_check_marker`: Loads pointers into registers, clears two working registers, reads a byte from a structure at offset 200, and compares it with 255, returning immediately when the value is not that marker.
- `decode_byte_stream_2`: Loops over a byte stream, decoding a special marker by combining two adjacent bytes into a 14-bit value and writing the split result back, while decoding other entries by reading a table indexed by the marker and writing the resulting byte to a destination pointer, advancing three pointers each iteration.
- `decode_byte_stream`: Reads a 32-bit entry from a table indexed by d0 and branches to another routine when the high bit and a specific flag are both clear.
- `process_byte_stream`: Copies a byte from a table indexed by d0 into the byte pointed to by a1, advances a0 and a1 by one, and sets bits 0, 1 and 2 of the byte pointed to by a2 depending on the values read from a table indexed by a0.
- `test_bit_set_pointer_2`: Tests bit 5 of the byte in d1 and returns when that bit is clear.
- `test_bit_set_pointer`: Tests bit 5 of the byte in d1 and returns when that bit is clear.
- `set_struct_bit_advance`: Sets bit 2 of the byte pointed to by a2, advances that pointer by one byte, then branches back to a shared routine.
- `advance_three_pointers`: Advances the three byte pointers in a0, a1 and a2 by one each, then branches back to a shared routine.
- `scan_table_accumulate`: Scans a byte table in a1 for the first byte that is not 255, masks it with 63 and accumulates the result in d0.
- `store_and_allocate_memory`: Stores the long in d0 at offset 58 of the structure in a5, scales d0 by eight and adds eight, then stores that result at offset 54.
- `store_pointer_call_library`: Stores the value in d0 at offset 40 of the structure in a5, checks a flag at offset 219 and returns early when it is clear, otherwise it loads a pointer from offset 58, calls a library routine, and stores the returned pointer at offset 44 when it is non-zero.
- `copy_narrator_record`: Copies one record from the source buffer into the destination buffer, deriving its fields from a global table and applying a small offset to some of them when a flag at offset 46 of the structure in a5 is set, then repeats until it hits the 255 terminator and pads the tail with seven zero bytes.
- `copy_narrator_record_loop_2`: Reads a byte from a source buffer and a length from a second buffer, then copies a fixed-size record (containing values derived from a global variable table plus a status byte) into a destination buffer, repeating until a 255 byte marker is reached and finally writing seven 255 bytes.
- `copy_narrator_record_loop`: Reads a byte from a source buffer and a length from a second buffer, then copies a fixed-size record (containing values derived from a global table) into a destination buffer, advancing both pointers.
- `blend_and_advance_table_2`: Advances a destination pointer by a scaled amount and, when a condition on a global table is met, blends two source bytes using a third value in a five-iteration loop, then advances all pointers and repeats until a byte equals 255.
- `check_and_scale_bytes`: Shifts d0 left by two and uses it to index a table into d1, then returns without branching when bit 15 of d1 is clear or when the masked value at offset 44 is zero, otherwise clears bits 10, 9 and 8 of d1 and branches away.
- `blend_and_scale_bytes`: Reads a byte from a table indexed by a masked value in d0, computes a smaller masked index, selects a scaling byte from another table, then adds that scaled value to five consecutive bytes pointed to by a6 before returning.
- `blend_and_advance_table`: Computes a weighted blend of two bytes stored in a table and writes the result into a byte pointed to by a6, then follows a pointer table to load the next pair of bytes.
- `advance_pointer_and_branch`: Increments the pointer in a6 by one and then branches into the middle of another routine without returning.
- `process_flagged_entries_loop`: Walks a table of 32-bit entries using a shifting index, reads a byte from a source buffer and a byte from a table indexed by a shifted value, and skips entries whose status flags at bits 10 and 11 are set.
- `skip_set_flag_entries`: Reads a byte from a source buffer and a byte from a table indexed by a shifted value, then skips entries whose status flags at bits ten and eleven are set, otherwise advancing a destination pointer and looping.
- `process_narrator_entries`: Walks through a byte buffer, skipping entries whose status flags are set, and for each valid entry writes a zeroed four-byte block to a destination table, setting byte three to 254 and words to 65566 unless a flag is set, then returns when the end marker is reached.
- `index_table_compare_values`: Reads a byte from a table at a pointer, uses it to index a table and load a value, compares two registers, and when the first is greater loads another indexed value before branching away.
- `compute_and_write_record_boundaries`: Reads a byte from the table in a0 and uses it to index tables in a2 and a3 to compute boundary values, then writes zeroed records into the structure in a6 and a7, returning when the marker byte is reached.
- `compute_index_offsets_2`: Reads a byte from the table in a0 and uses it to index tables in a2 and a3 to compute boundary values, then writes zeroed records into the structure in a6 and advances pointers in a0 and a1 until the end marker is reached.
- `compute_index_offsets`: Subtracts d2 and d3 from d0 into d4, loads d2 into d1, and branches away, leaving d4 and d1 as computed offsets for the shared routine.
- `clear_and_store_indices`: Clears a word and an adjacent byte in the structure pointed to by a6, tests bit 7 of d0 and branches away when set, stores d6 into two fields, advances a6 by eight, then loops on d1 and otherwise returns.
- `advance_and_loop`: Advances a pointer in a6 by four and loops on d1 until it underflows, then clears d1 and branches into the middle of another routine without returning.
- `clear_and_store_struct_loop`: Clears a word and the byte at offset 2 of the structure in a6, sets bit 7 of d5 when it is set, stores d6 into the byte at offset 3 and the word at offset 4, advances a6 by eight, and repeats the block until d1 reaches zero before advancing a0 and branching into another routine.
- `zero_index_fields`: Loads d0 into d1 and clears d4 and d3 to zero, then branches away, supplying zeroed index fields to the shared routine.
- `filter_and_copy_buffers`: Loops through a source buffer until it finds the byte 255, and for each byte copies it along with the corresponding bytes from two other buffers into three destination buffers only when a bit in a table entry is clear, then writes the byte 255 to the ends of the destination buffers.
- `write_byte_to_three_pointers`: Writes the same byte from a register to the memory locations pointed to by three registers.
- `check_stream_and_decode_entries`: Calls a routine that checks a stream-end marker at offset 232 of a structure, then calls a routine that divides a 32-bit value by a 16-bit divisor and decodes a run-length encoded table, and returns.
- `check_narrator_stream_end`: Loads the byte at offset 232 of the structure in a5 into d0 and compares it with 255; if equal it returns, otherwise it clears bit 0 of that byte and returns.
- `check_flag_stream_end`: Loads the byte at offset 232 of the structure in a5 into the low byte of d0, then compares it with the value 255.
- `decode_flag_stream`: Walks a byte stream setting bits 0, 1 and 2 of output bytes based on table flags, then reads the next byte and returns when it is the end marker.
- `set_flag_and_check_byte`: Sets bit 1 of the byte at offset negative one from a1, branches away when bit 7 of d1 is set, loads a byte from a0 and branches away when it equals 24 or 23, sets bit 2 of the byte at a1, copies a1 into a3, and returns.
- `scan_and_set_flags`: Sets bit 1 of a byte pointed to by a1, then scans a table indexed by byte values to find a matching entry and set its flag bits.
- `set_bit_and_branch`: Sets bit 2 of the byte pointed to by a3 and then branches into the middle of another routine without returning.
- `set_flags_and_branch`: Sets bit 0 of the byte two bytes before a1, sets bit 1 of the byte one byte before a1, sets bit 2 of the byte pointed to by a1, then branches into another routine without returning.
- `decode_narrator_entries_2`: Divides a 32-bit value by a 16-bit divisor and stores the result into a global variable, then decodes a run-length encoded table by writing computed bytes to a buffer.
- `advance_rle_output_pointer`: Clears the high bits of d0 and tests bit 0 of the byte at the address in a2, advancing that pointer.
- `decode_rle_stream`: Decodes a length-prefixed, run-length-encoded byte stream pointed to by a1 and writes the resulting bytes to the buffer in a6, using the byte at a2 as a control mask to decide how far to advance.
- `index_and_accumulate_byte`: Loads a byte from a1 into d0, masks it with 63, tests bit 2 of the byte at a2, branches away when set, shifts the index left by three, adds it to d1, and loops back to reload the next byte.
- `decode_narrator_entries`: Walks a table of encoded entries, each with a byte at offset 0, a byte at offset 128, three bytes at offsets 256, 384 and 512, and a flag byte at offset 1, and writes a byte from a structure field at offset 32 divided by a value into a buffer.
- `compute_divide_and_store`: Swaps the halves of d5, adds d6 to it, loads the long at offset 32 of the structure in a5, divides it by the result, stores the quotient into the byte before a6, decrements both a1 and a2, then branches into another routine.
- `store_divided_word`: Loads a pointer and a word, divides the word by 110, advances the pointer, and stores the word at that location unless the byte there is 255, then returns.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `process_narration_node_chain`: Walks a linked list of narration nodes stored in a global structure, following child pointers based on flag bits and calling the text-output routine when a node should be displayed, and returns when the end marker is reached.
- `index_table_branch_bits`: Multiplies d5 by four, loads a pointer from a table indexed by that value into d3, and branches into another routine when bit 19 of the loaded word is set.
- `advance_pointers_and_branch`: Increments the pointers in a1, a0 and a3 each by one and then branches into the middle of another routine without returning.
- `process_narrator_byte`: Reads a byte from a table using a pointer in a0, combines it with a scaled product of a value from a table in a6, clamps the result to a valid range, and stores the resulting byte through a pointer.
- `test_flags_then_encode`: Tests bit twelve of a register and returns when it is clear, otherwise tests bit nine and branches away when set, otherwise loads the value thirty-eight into a register and calls a routine.
- `check_flag_encode_byte`: Tests bit 11 of a register and returns without doing anything when it is clear, otherwise loads the constant 38 into a register and calls a routine that encodes a narrator byte.
- `test_bits_call_scaled_product`: Tests bit 11 of a register and branches away when it is clear, tests bit 4 of a byte at offset 1 of a structure and branches away when it is set, then loads a constant and calls a routine before branching away.
- `encode_narrator_byte_2`: Checks bit 11 of the byte in d7 and, if it is set, computes a scaled and shifted product using the low word of d7 and a value from a table in a6, then stores the clamped result.
- `encode_narrator_byte`: Reads a byte from a lookup table using a pointer in a0, combines it with a scaled product of a value from a table in a6, clamps the result to 63, then stores the encoded byte into the buffer pointed to by a1 and advances both pointers.
- `test_bits_then_scaled_product`: Tests bit 1 of the byte in d6 and bit 1 of the byte in d7, returning early from whichever test is clear, then loads the constant sixteen into d1 and calls the scaled-product routine before jumping to a later instruction.
- `prepare_scaled_shifted_product`: Loads the constant sixteen into d1, calls the scaled product routine, then branches back into the middle of the caller.
- `decode_compressed_narrator_text`: Decodes a compressed text stream in a0 byte by byte, combining a lookup-table value with a delta from a table in a6, then writes the resulting printable characters into the output buffer in a1 while following multi-byte sequences and calling another routine for special markers.
- `compute_scaled_shifted_product`: Multiplies the low word of a register by a word input, adds 16 to the product, shifts it right by five bits, and returns a combined value formed from the high word of the product and the shifted low word.
- `initialize_and_clear_buffers`: Stores several offsets relative to a base pointer into a global structure, increments a few of those pointers, clears a set of word fields, then zeroes six blocks of memory by iterating a fixed count.
- `increment_counter_call_handlers`: Increments a 16-bit counter at offset 136 of a structure, then when the increment did not overflow and did not wrap to zero calls four routines in sequence, clears the condition code register, and returns.
- `decode_run_length_stream_2`: Reads pointers from a structure, then repeatedly reads a byte from one table and a byte from another, and when the first byte is not the end marker branches into the run length stream decoder.
- `decode_run_length_stream_loop`: Tests bit 7 of the byte at offset -1 from a4 and, if it is set, returns.
- `decode_run_length_stream`: Decodes a run-length encoded byte stream from the buffer in a0 and writes the expanded bytes into a destination buffer at a4, using a 256-entry lookup table at a3 to translate each byte.
- `set_flag_and_store_indices`: The routine ORs a constant into a byte addressed by a base pointer and an index, then stores two short values derived from an index and a conditional copy into fields of a global structure.
- `process_narrator_buffer_2`: Walks a byte buffer until it finds a zero byte, setting bits in a status byte at offset 104 of the structure in a5 for each entry and handling the marker at offset 103 by OR-ing it with 144.
- `load_indexed_pointer_d3`: Loads a pointer from a table indexed by d0 (multiplied by four) and stores the pointed-to value into d3, then returns if bit 13 of that value is set.
- `clamp_and_set_byte`: Tests bit 4 of the byte in d2 and returns immediately when it is clear.
- `decrement_and_store_pointers`: Decrements the three pointers in a0, a1 and a2 by one, stores each decremented pointer into three fields of the structure in a5, clears d7, copies d4 into d0, then jumps to a later instruction.
- `check_flag_and_clear_byte_2`: Sets bit 5 of the byte at offset d2 from a4, copies d0 into d2, and compares the byte at offset d0 from a6 against two values.
- `check_flag_and_clear_byte`: Sets d1 to one and d7 to zero, then tests bit 5 of the byte at offset d2 relative to a4.
- `swap_and_clear_byte_flags`: Swaps a byte between two positions in a table addressed by a6 using the indices in d0 and d2, returns immediately when the indices are equal, otherwise decrements d0 and loops back to swap the next pair.
- `load_pointer_and_branch`: Loads a long pointer from the global variable at offset 112 into a1, copies the word in d4 into d0, and then branches into the middle of another routine.
- `clear_bit_in_range`: Loops clearing bit 0 of each byte in a range indexed by a decrementing register until the register reaches -1, then returns.
- `or_mask_into_buffer_with_early_exit`: Selects a mask based on a control byte, then ORs that mask into a series of bytes in one buffer while checking a flag in another buffer and returning early if the flag is set.
- `set_tile_and_mask`: Reads a byte from a structure and, unless it holds a specific marker, sets a mask bit in a byte addressed by another pointer, then scans a second buffer backwards while a bit is clear and clears and sets bits in the byte it stops at.
- `accumulate_table_entries_2`: Reads a short from offset 140 of the structure in a5, conditionally calls another routine several times based on two short fields, then adds the short value to eight consecutive words in a table pointed to by the structure.
- `clamp_value_and_store_char_2`: Computes a scaled value from two arguments, clamps it between two bounds, and stores the resulting byte at an address formed from a base pointer and an index.
- `clamp_value_and_store_char`: The routine clamps a short input to a lower and upper bound and stores the resulting byte at an address formed from a base pointer and an index.
- `adjust_narrator_table_values`: Reads a signed byte from a table and a character code, computes a scaled difference, then walks a second table adjusting its byte values by that difference while skipping bytes with bit 5 set.
- `adjust_table_values_loop`: Repeatedly computes the absolute difference between two signed bytes in a table, scales it by dividing by 128, adds the scaled value to a destination byte, and scans backwards through a range using flag bits to locate entries.
- `compute_scaled_char_values_2`: Loops over a byte-indexed range decrementing d6, and for each entry with bit 5 set computes a scaled product and a clamped value into three structures before decrementing the index and returning when it underflows.
- `adjust_and_scale_byte`: Adjusts a value in d2 by adding 64 and arithmetic-shifting it right by 7, then adds the result to the byte at offset d0 in the structure pointed to by a3.
- `compute_scaled_char_values`: Adds 64 to d2, arithmetic-shifts the result right by 7, adds it to the byte at offset d0 relative to a2, and repeats this in a loop until d0 reaches its starting value.
- `load_pointers_copy_word`: Loads two pointers from a global variable and copies a word from another field into a third register, then branches into another routine.
- `adjust_narrator_byte_tables_2`: Loops over a range of byte offsets, adjusting signed byte values in two structures by scaling them with a mode-dependent factor and rounding, then corrects a third byte based on the difference between the adjusted values, and returns when the range is exhausted.
- `adjust_and_add_byte_d3`: Adds 64 to d2 and arithmetic-shifts the result right by seven, then adds it to the byte at offset d0 in the structure in a3 and returns.
- `adjust_and_compare_bytes_3`: Adds 64 to d2 and arithmetic-shifts the result right by seven, adds it to the byte at offset d1 in the structure at a3, then adjusts d2 by subtracting 110 and multiplying by 4086, returning the non-negative branch or otherwise decrementing d2 by 64.
- `adjust_and_compare_bytes_2`: Adds 64 to d2 and arithmetic-shifts the result right by seven, adds it to the byte at offset d0 in the structure at a6, then loads the byte at offset d1, subtracts 110, multiplies by 26, and branches away when the product is negative.
- `adjust_and_compare_bytes`: Adjusts a value in d2 by adding 64 and arithmetic-shifting it right by 7, then adds the result to the byte at offset d1 in the structure pointed to by a6.
- `add_byte_and_branch`: Adds the byte in d2 to the byte at offset d0 relative to a2, and then branches into the middle of another routine.
- `adjust_narrator_byte_tables`: Adjusts four signed byte values in two structures by scaling them with a mode-dependent factor and rounding, then, when a condition holds and the mode is greater than one, subtracts 105 from a byte.
- `adjust_narrator_byte_table_2_2`: Adds 64 to d2 and arithmetic-shifts the result right by seven, adds it to the byte at offset d0 in the structure at a3, then loads the byte at offset d1, extends it, multiplies by d7, and branches away when the product is negative.
- `adjust_narrator_byte_table_2`: Repeatedly adjusts bytes in the structure at a6 by adding a value derived from a byte in the structure at a4, then scans a table of flags and, when a matching entry is found, modifies a value in the structure at a2.
- `adjust_narrator_byte_table`: Repeatedly adjusts bytes in the structure at a6 by adding a value derived from a byte in the structure at a4, then scans a table of flags and modifies associated bytes, returning when the outer loop counter reaches zero.
- `scale_narrator_byte_tables`: Scales signed bytes in a table by multiplying by 102 and subtracting 64, then stores the result back.
- `scale_byte_by_running_max`: Loads a byte from the structure in a2 at offset d0 into d1, scales it by 102 using the value in d1, and then subtracts 64 from the result when the scaled value is negative.
- `adjust_byte_by_running_max`: Adds 64 to the 16-bit value in d1, arithmetic-shifts it right by 7, adds the resulting byte to the byte at offset d0 of the structure in a3, then clears d7 and stores the byte at offset d0 of the structure in a6 into d7 before jumping to the marker routine.
- `update_tile_heights_with_mask`: Reads a byte from a source buffer indexed by a register and compares it with a running maximum held in another register, storing the byte when it is larger, then decrements the index and repeats until the counter reaches zero.
- `prepare_buffers_for_processing`: Loads two pointers from global variables into a2 and a0, clears d1, copies the value from d4 into d0, and then jumps to the start of another routine.
- `scale_and_adjust_byte_range`: Loops over a byte range decrementing the index, and for each byte whose high nibble is nonzero it multiplies a source byte by a sign-adjusted constant, then adds the shifted result to the destination byte.
- `shift_and_add_byte_range`: Adds 64 to the 16-bit value in d1, arithmetic-shifts it right by 7, adds the resulting byte to the byte at offset d0 of the buffer in a2, then decrements d0 and repeats until it underflows before returning.
- `split_and_adjust_byte_stream`: Reads a length from offset 138 of the structure in a5, then repeatedly finds the next byte with bit 5 set in a table at offset 108 while copying and adjusting values from buffers at offsets 96, 152, 116 and 96.
- `copy_and_adjust_byte_pair`: Copies a byte from one buffer to another at the same index, subtracts a value from the destination, adds five to the source copy, then decrements the counter and branches to a shared routine when the loop ends.
- `split_and_balance_byte_stream`: Splits a packed byte stream into three separate buffers, doubling the low nibble of each stored byte, then repeatedly locates the next set marker in the source, balances the differing values between two buffers, and copies the resulting bytes into the remaining buffer while adding five to each.
- `store_and_nibble_adjust_byte`: Reads a byte from a source buffer and stores it into three buffers at the current index, then doubles the low nibble of the byte and adds it to one destination before decrementing the counter and branching to a shared routine.
- `setup_and_dispatch_routines`: Loads a fixed address into a register and calls a routine that checks a flag and dispatches to other routines, then branches into another routine.
- `load_table_then_call`: Loads the address of a global table into a4, calls a helper routine, then branches into the body of another routine.
- `setup_buffer_pointers_d6`: Loads a pointer from offset 6c of the structure in a5 into a4, stores the constant 110 into d6, copies the value from d5 into d0, and branches back.
- `adjust_and_normalize_buffer_2`: Walkes backward through a byte buffer, copying characters into a second buffer while clearing two other buffers and adjusting values whose bit 4 is set.
- `adjust_and_normalize_buffer`: Finds the last byte with bit 5 set in a buffer, subtracts the byte at the found position from the bytes in another buffer, stores the adjusted values into a third buffer, and then either writes a marker byte or finds the maximum value in the third buffer.
- `swap_and_scale_byte_buffers`: Increments a counter, swaps two bytes between two buffers, subtracts a sign byte from a byte, copies the result into a third buffer, adds five to it, and decrements a loop counter, repeating until it reaches negative one.
- `prepare_loop_registers`: Clears two registers, copies a third register into the loop counter, and branches into the body of another routine.
- `accumulate_bytes_and_write_results`: Loops over a byte range accumulating values while tracking a running maximum, derives a scaled character from that maximum, and writes it plus a derived value to two buffers with a conditional copy.
- `decrement_pointers_copy_word`: Loads three pointers from a global variable, decrements each by one, clears a register, copies a word from another field into a fourth register, then branches into another routine.
- `scale_and_store_byte_stream_2`: Walks a decoded byte stream, decoding each entry and scaling its value by a factor, then adds the adjusted result into four byte buffers pointed to by offsets 92, 120, 116 and 96 of the structure in a5, and returns when the counter reaches negative one.
- `decode_and_scale_byte_d1`: Loads a byte from offset d0 of the structure in a6 into d1, subtracts 110 from it, multiplies the result by 26, and returns.
- `decode_and_scale_byte_stream`: Decodes a byte stream from a buffer, scaling each entry's value by a factor and writing the adjusted result into two bytes, then applies further scaling and accumulation to several byte arrays in a structure.
- `scale_and_store_byte_stream`: Processes a byte stream from a buffer, decoding each entry and scaling its value by a factor, then writes the adjusted result into two byte buffers indexed by a running counter.
- `compare_index_then_load`: Copies the 16-bit value in d4 into d7, subtracts one, compares it with d0, and returns when they are equal; otherwise it loads 56 into d7 and jumps to the scaling routine.
- `load_index_for_scaling`: Loads 86 into d7 and jumps to the scaling routine.
- `scale_buffer_byte_by_index`: Loads a byte from a buffer indexed by a register, sign-extends it, multiplies it by the constant twenty-six, subtracts sixty-four when the product is negative, and branches to a shared routine.
- `scale_and_adjust_byte_pair`: Loads a byte from a table indexed by a counter in d0 and adds it to the byte at the same offset in the structure in a6, then subtracts the same value from the byte at that offset in the structure in a3.
- `branch_into_routine`: Branches into the body of another routine.
- `copy_and_adjust_bytes_with_feedback`: Copies bytes between two buffers, updates a feedback value by multiplying it with a table entry and shifting, subtracts the feedback from a byte, and writes adjusted bytes to two output areas.
- `check_narrator_flag_and_dispatch`: Calls a series of other routines in sequence, then tests the byte at offset 219 from the pointer in a5 and calls another routine if that byte is non-zero.
- `load_sequential_narrator_bytes`: Loads three consecutive bytes from a buffer pointed to by a5 at offsets 3, 4 and 5, advancing the pointer each time, and returns.
- `interpolate_and_store_bytes`: Reads two bytes from a buffer, computes a scaled difference based on an index, accumulates the result, and stores successive byte results into the buffer at regular intervals.
- `load_byte_check_marker`: Loads a byte from the buffer pointed to by a0 into d6 after advancing the offset by 8, then returns if that byte is not the 255 marker.
- `check_markers_then_advance`: Compares d6 with 254 and jumps when they are equal, compares d1 with 8 and jumps when they differ, then adds 8 to a0 and jumps to the marker routine.
- `call_then_branch_routine`: Calls a helper routine and then branches into another routine.
- `test_byte_at_offset`: Tests the byte at offset 8 from the pointer in a0 and returns if it is zero, otherwise returns if it is not the 255 marker.
- `advance_pointer_by_eight`: Compares d1 with 8 and jumps when they differ, adds 8 to a0, then jumps to the marker routine.
- `scan_buffer_for_marker`: Reads a byte from a buffer at an offset that increases by 8 each iteration until a non-zero byte is found, and if that byte equals 255 it returns, otherwise it advances the pointer by 8 and repeats.
- `process_byte_stream_loop_2`: Loops through a byte stream, advancing a pointer by scaled offsets and incrementing or copying bytes depending on flag bits, then processes a special case that writes a run of incremented values.
- `process_byte_stream_loop`: Loops through a byte stream, advancing a pointer by scaled offsets and incrementing or copying bytes depending on flag bits, then processes a second stream by copying fixed-length chunks into a destination buffer.
- `compute_indexed_pointer`: Multiplies the index register by eight, adds the result to a base address to compute a pointer, and branches into another routine.
- `check_and_set_marker_byte`: Shifts the value in d3 right by two bits, then returns without acting if it is not 42, otherwise it stores 96 into d3 and branches to the next routine.
- `check_set_marker`: Compares d3 with 43 and returns without acting if they differ, otherwise stores 97 into d3 and branches to the next routine.
- `set_marker_then_copy`: Compares d3 with 44 and jumps when they differ, stores 98 into d3, then jumps to the copy routine.
- `copy_narrator_block`: Copies 6 bytes from the source address in a0 to a destination buffer pointed to by a6, which is offset 99 from a1, repeating the copy 128 times and then advancing a0 by 2 bytes.
- `initialize_struct_pointers`: Loads five base addresses from fixed offsets of the structure in a5 into registers, sets d6 to 12288, and initializes d4, d5, and d7 with the constants 240, 255, and 63.
- `compare_loaded_byte_to_d5`: Sets bit 7 of the byte in d4, clears d0 and d2, loads the next byte from the buffer in a0 into d0, and returns if it does not match the byte in d5.
- `copy_and_expand_byte_stream_2`: Walks two byte buffers, copying each byte from the source to the destination while applying a per-byte transformation table, and returns when the source byte matches the end marker.
- `copy_and_expand_byte_stream`: Walks two byte buffers, copying each byte from the source to the destination while applying a per-byte transformation table.
- `copy_and_mask_structures`: Copies a block of 32 bytes from a source structure to a destination structure, then indexes a table using a shifted value and applies a mask to two byte fields in the destination structure.
- `translate_byte_records_loop`: Reads a byte from a table at offset 3 of the structure in a5 and stores it through a pointer in a0, then advances that pointer.
- `copy_byte_via_index`: Copies a byte from a table indexed by a value in d0 through a1 into the byte pointed to by a0, advances that pointer, and repeats the read-modify cycle before adding six to the pointer and jumping back.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `compare_byte_against_255`: Loads a byte from offset 24 of the structure in a6 into d3, compares it with the value 255 in d6, and returns immediately if they are equal.
- `scale_and_store_byte`: Adds d3 to d0 and decrements d1 in a loop, and when the loop ends stores a scaled result of a byte at offset 24 of the structure in a6 back into offset 24 before advancing the pointer by eight and jumping back.
- `compare_byte_branch_if_differ`: Loads a byte from offset 24 of the structure in a6 into d3, compares it with the value 255 in d6, and branches to offset 2da4 if they differ.
- `rescale_byte_loop`: Maintains a running sum in d0 and a countdown in d1, and when the countdown reaches its end rescales a value from a structure in a6, advances the pointer, and resets the counters before continuing until a tracked byte stops matching.
- `load_buffer_byte_check_255`: Sets up pointers to two source tables and the buffer in a5, clears d2 and d0, loads the next byte from the first buffer into d0, and returns if that byte is 255.
- `process_narrator_buffer`: Reads bytes from a buffer in a0 and, based on each byte's value, adds a character to the byte at offset 7 of the structure in a6 while advancing that pointer by eight, repeating the process for a run of bytes.
- `increment_byte_table`: Adds ten to a byte at offset 7 of the structure in a6, advances the pointer by eight, and repeats this eight times in a loop before jumping to another routine.
- `set_value_branch_routine`: Loads the value 250 into d3 and then branches to the routine at offset 2e3a.
- `load_negative_byte`: Loads the signed byte value -6 into register d3 and then branches to another routine.
- `load_byte_value_d3`: Loads the byte value 250 into register d3 and then branches to another routine.
- `fill_byte_offset`: Subtracts one from d2, writes the constant 230 into a byte at offset 7 of the structure in a6, advances the pointer by eight, and repeats this for each remaining count before jumping to another routine.
- `shift_and_add_byte`: Decrement d2, shift the byte at the base of the structure in a6 right by two after subtracting 43, add the result to a byte at offset 7, advance the pointer by eight, and repeat for each remaining count before jumping to another routine.
- `decode_nibble_checksum_bytes`: Reads a pointer and a count from a structure and processes a byte array in two passes, computing a checksum from low and then high nibbles of groups of bytes and writing the result back into the third byte of each group, masking the appropriate nibble.
- `call_flagged_request_loop`: Reads two values from function pointers stored in a global structure and stores them at offsets 50 and 49 of the buffer, then sets a 32-bit field at offset 776 to two shifted bits, and finally loops forever calling two more function pointers.
- `process_flagged_request_loop`: Loops while a flag byte at offset 34 of the structure in a2 is free, sets that flag, calls a handler routine, stores its result at offset 772 of a structure in a3 when the result is not two, and returns when the flag was already set.
- `clear_two_flag_bits`: Clears two adjacent bits in a flag byte at a fixed offset of a structure and returns.
- `setup_narrator_struct_2`: Initializes a structure by filling a table of pointers with a global variable, sets up several fields and a flag, then repeatedly copies blocks of bytes between buffers while building a bitmask and zeroing a character buffer.
- `setup_narrator_struct`: Fills a buffer by copying bytes from a table through a pointer, then sets up a structure with character data and calls a routine repeatedly based on a counter.
- `decode_narrator_byte_stream`: Decodes a byte stream and writes pairs of reconstructed bytes to a buffer, calling another routine when a counter at offset 10 of the structure in a5 underflows, then returns.
- `decode_narrator_records`: Decodes a stream of eight-byte records, extracting a byte from each record and using its high bit and low nibble to build a lookup index, then stores the decoded characters into a structure.
- `fill_buffer_from_table_2`: Fills a buffer by reading bytes from a table through a pointer, updating a rolling 1024-value checksum and decrementing a counter at offset 16 of the structure in a5, and calls another routine when that counter reaches zero.
- `decode_run_length_stream_3`: Decodes a run-length encoded byte stream from a buffer, writing the expanded bytes into a structure while tracking a position counter that wraps around, and calls another routine when a record boundary is reached.
- `copy_narrator_record_fields`: Copies the next eight-byte record from the input stream into the fields of a structure, setting a flag when the first byte has its high bit set and computing a lookup index from its low nibbles, then advances the stream pointer.
- `decrement_store_word_branch`: Decrements d0 and returns if the result is negative, otherwise branches to the routine at offset 5618.
- `enable_dma_if_flag_set_2`: Tests a flag byte in a structure and, if set, writes a value to a custom hardware register to enable a DMA channel, then calls a routine from a table repeatedly, restoring the register afterwards.
- `enable_dma_if_flag_set`: The routine tests a flag byte in a structure pointed to by A5 and, if set, writes a value to a custom hardware register that enables DMA channels by setting bit 15 of DMACON.
- `update_interrupt_counter`: Stores the decimal 16384 in a global variable, increments the byte at offset 198 of the structure in a6, and reads a byte from the address in a2.
- `decrement_counter_and_enable_interrupts`: Reads a byte from an offset in a structure, decrements it and writes it back.
- `index_table_from_structure_byte`: Loads the byte at offset 68 of the structure in a3 into d2, copies it to d7, multiplies it by 68, checks a flag in a5, then sets up two pointers for the next routine and returns.
- `set_flag_call_loop_clear`: Sets bit 0 of the byte at offset 30 of the structure in a3, then calls a routine whose address is stored at offset 20 of that same structure, repeating for a counter decremented from a register until it reaches one, and finally clears bit 7 of the byte at offset 30.
- `wait_loop_set_flag`: Decrements a counter and, if bit 6 of a status word in a5 is set, writes 32832 to a custom register at offset 346.
- `set_flag_and_branch`: Sets bit 5 of the byte at offset 30 of the structure in a3 and then branches to the routine at offset 5878.

### Qume_LetterPro_20

- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `call_routine_and_check`: Stores the incoming parameter into a global variable, then calls a routine at offset 228 of that variable.
- `call_routine_twice`: Calls the routine at offset 42 twice and then returns the decimal 255.
- `call_table_routine_three_times`: Calls a routine from a table three times, passing a value on the second and third calls, and returns zero.
- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `setup_printer_pointer_check`: Restores its saved registers from the stack, loads three pointers from the stack frame, clears two data registers and a third register, compares the word at the first pointer with one, and if they differ it jumps to another routine before falling through to it.
- `translate_printer_output`: Copies ten bytes from a global table into a buffer starting at offset zero of the pointer in a4, then reads two bytes from the structure at offsets 221 and 223 and stores them at offsets 12 and 14.
- `advance_pointer_and_store`: Advances the pointer in d2 by one, adds the pointer in a4 to it, copies the byte at offset 1 of d3 into that address, advances d3 by one, and branches to the routine at offset 508.
- `check_escape_store_space`: Compares a word at the address in a0 with the escape character, and when it matches and the byte at a1 is zero, stores the space character at a0 and sets the byte at a1 to one, then branches into another routine.
- `check_escape_store_four`: Compares the byte at the address in a0 with the escape character and, when they match, stores the value 4 at that address, then returns.
- `clamp_value_and_store_byte`: Clamps a 16-bit value stored at the address in a0 to a maximum of 31, then stores 33 at the byte address in a1 when that byte is negative.
- `store_byte_when_not_zero`: Compares the 16-bit value at the address in a0 with 31 and returns without changes when it differs, then tests the byte at the address in a1 and stores the byte 255 through that address when it is not zero.
- `store_byte_when_below_zero_2_2`: Compares the 16-bit value at the address in a0 with 31 and returns without changes when it differs, then tests the byte at the address in a1 and stores 33 at that address when it is at or below 31.
- `store_byte_when_below_zero_2`: Compares the word at the address in a0 with 31 and returns without changes when it differs, then tests the byte at a1 and stores 33 at that address when it is not negative.
- `store_byte_when_below_zero`: Compares the word at the address in a0 with 31 and returns without changes when it differs, then tests the byte at a1 and stores 33 at that address when it is below zero.
- `compare_word_increment_byte`: Compares the word at the address in a0 with 32, and if it differs it returns without changing anything, otherwise it increments the byte at the address in a1 and branches to the next routine.
- `compare_byte_branch_decrement`: Compares the byte at the address in a0 with 33, and if it differs it returns without changing anything, otherwise it decrements the byte at the address in a1 and branches to a later routine.
- `encode_byte_sequence`: Checks the byte at the address in a0 and, if it equals 57, writes the escape byte and the letter F to consecutive bytes, then stores two ASCII digits derived from the nibbles of a stack byte.
- `compute_offset_and_store`: Advances the pointer in d2 by one, adds the pointer in a4 to it, sign-extends the word in d0, adds the decimal 48 to it, stores the result as a byte at that address, and then branches to the routine at offset 508.
- `write_status_byte_3`: Checks if the word at the address in a0 is zero, and if so, writes the byte 253 to offset 4256 of the structure pointed to by the value in d1, then returns zero.

### CBM_MPS1000

- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `invoke_handler_with_param`: Loads a global variable into a register, stores the incoming parameter and related values into a global structure, then calls a routine at a negative offset from that structure and branches to another routine when the result is nonzero.
- `save_registers_and_call`: Loads the long at offset 16 into a1, saves a6 on the stack, loads the long at offset 8 into a6, calls the routine at offset 418, restores a6, loads the long at offset 12 into a1, saves a6 again, loads the long at offset 8 into a6, calls the routine at offset 418, restores a6, loads the decimal 255 into d0, and branches to the routine at offset 174.
- `call_handler_via_base`: Invokes the routine stored at offset -414 of the global base in d1 three times, passing the a6 register on the second and third calls, then returns zero.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry`: Calls the routine at offset 104 of the global base in d1, and if it returns zero, stores the value 9 at offset 300, saves the two parameters at offsets 304 and 308, clears offset 302, loads a pointer from offset 292, and runs four routines through that base, then returns zero.
- `handle_printer_commands`: Selects its behavior by the fourth argument: for 0 it computes and stores pointer offsets and calls another routine, for 1 it ORs a table byte into a buffer, for 2 calls a routine at offset 100 and returns its result, for 3 clears a region of a buffer and writes a control byte, a letter, a 16-bit value, and a newline, and for 4 calls routines at offsets 100 and 104 before returning.
- `restore_registers_and_return`: Restores the caller's d2, d3, a2, a3 and a4 registers from the stack and returns zero in d0.
- `restore_and_check_printer_status`: Restores the caller's registers from the stack and loads three pointers from the stack frame, then compares the word at the first pointer with 1 and branches away when they differ.
- `init_printer_output_buffer`: Initializes a printer output buffer by clearing its header fields, then writes a printer-specific command sequence derived from status values at offsets 1654, 1656, 1658, and 1661 of the structure in d1, and finally copies a two-byte value from offsets 1661 and 1663 into the buffer.
- `store_byte_when_offset_matches`: Loads a pointer from the address in a0, compares the word at offset 1654 of the pointed structure with 2048 and returns without changes when it differs, then stores the byte 15 at the address formed by adding the value in a4 to the value in d4.
- `process_printer_commands`: Copies a NUL-terminated string from a source buffer into a destination buffer, then dispatches on a command byte: for the printer commands it writes the corresponding escape sequences and copies four bytes from a global variable, otherwise it toggles a state flag and writes the matching printer control byte.
- `copy_four_bytes`: Advances the long in d4 by one, uses it as an index into the pointer in a4, copies the byte at offset 1 of d5 into that address, advances d5 by one, compares it with the decimal 4, and branches back to the start when it is greater.
- `store_byte_when_word_matches`: Compares the word at the address in a2 with 66 and returns when it differs, then loads a pointer from a global variable, compares a word at offset 1654 of that structure with 1024, and stores 96 at the word pointed to by a third register when it matches.
- `check_word_store_byte_branch`: Loads a pointer from a global variable into a0 and the pointed-to value into a1, compares a word at offset 1654 of that structure with 1024, and if they differ it stores the byte 137 at the address loaded into a0 from a register before branching away.
- `store_byte_then_branch`: Copies the address from register d3 into register a0, writes the byte 80 to the location pointed to by a0, then branches to the routine at offset 1ac.
- `copy_four_bytes_again`: Advances the long in d4 by one, uses it as an index into the pointer in a4, copies the byte at offset 1 of d5 into that address, advances d5 by one, compares it with the decimal 4, and branches to the routine at offset 634.
- `store_values_when_byte_not_zero`: Compares the word at the address in a2 with 32 and returns when it differs, then stores 1 at the address in a3 and stores 27 at the word address in a2 when the byte at a3 is not zero.
- `conditionally_write_byte`: Tests the byte at the address in a3, returns when that byte is greater than or equal to zero, otherwise clears that byte, writes the byte 30 to the word at the address in a2, and branches to the routine at offset 28c.
- `load_and_branch`: Loads the immediate value 255 into d0 and then branches to the routine located at offset 654 without returning.
- `compare_word_test_byte_store`: Compares the word at the address in a2 with 33, and if it differs it returns, otherwise it tests the byte at the address in a3 and returns if that byte is not zero, otherwise it stores 255 into that byte and 29 into the word at a2.
- `test_byte_clear_store_branch`: Tests the byte at the address in a3 and returns if it is not positive, otherwise it clears that byte, stores the word 28 at the address in a2, and branches to a later routine.
- `unconditional_branch`: Branches unconditionally to the routine at offset 1da.
- `decode_status_byte`: Sets the byte pointed to by a3 based on the value read from a2, stores a code into a stack byte for two specific values, writes a three-byte sequence and returns an offset when a2 equals 2, otherwise writes the byte 253 to offset 4256 of the structure in d1, then returns.
- `write_status_byte`: Checks if a word at the address in a2 is zero, and if so, writes the byte 253 to offset 680 of a structure pointed to by the value in d1.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_630

- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `check_and_call_structure_routines`: Stores the incoming parameter into a global variable, then calls a routine at offset 228 of that variable and compares its result to zero.
- `save_regs_call_twice_branch`: Loads a global pointer into a1, saves a6 on the stack, sets a6 from another global, calls a routine through a6, restores a6, reloads a1 and a6, calls the same routine again, restores a6, and finally loads 255 into a register before branching away.
- `invoke_d1_pointer_routine`: Invokes the routine referenced by the pointer stored at offset -622 in the D1 data hunk, passing the value in a6 as an argument on the second and third calls, then returns zero.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `check_printer_mode`: Loads the printer structure pointer from the stack, initializes several registers to zero, compares the byte at offset 0 of that structure with the value one, and branches to the printer output translation routine when they match, otherwise it returns.
- `normalize_printer_control_byte`: Copies up to eleven bytes from a global table into a buffer, then reads status bytes from a structure and a character string from another table to normalize a control byte and its argument, writing the result back through the pointer.
- `copy_buffer_and_append_globals`: Copies bytes from a source buffer into a destination buffer until a zero byte is reached, then copies two bytes from a table at offsets 2653 and 2655 of a structure and appends eleven bytes from a global variable to the destination when the first byte read is the letter A, otherwise it returns.
- `copy_byte_from_table`: Copies one byte from a table indexed by a running counter into a destination computed from a2 and a4, increments both the counter and the pointer, and repeats while the counter stays below eleven.
- `store_escape_when_match_2`: Compares the byte at the address in a0 with the escape character and, if they are not equal, returns; then checks the byte in a1 and, if it is zero, stores the space character at the address in a0 and sets the byte in a1 to one before returning.
- `store_escape_when_match`: Checks whether the byte at the address in a0 equals the escape character, and if so stores the value 4 there, otherwise it returns.
- `store_word_when_not_greater_zero`: Compares the 16-bit value at the address in a0 with 28 and returns without changes when it differs, then tests the byte at the address in a1 and stores the word 33 through that address when it is not greater than zero.
- `store_printer_status_when_29`: Compares the 16-bit value at the address in a0 with 29 and returns without changes when it differs, then tests the byte at the address in a1 and returns when it is non-zero, otherwise stores 33 at that address and 255 in the byte at the address in a1.
- `store_word_when_not_greater_zero_2`: Compares the 16-bit value at the address in a0 with 29 and returns without changes when it differs, then tests the byte at the address in a1 and stores the word 2 through that address when it is not greater than zero.
- `store_byte_when_not_greater_zero_2`: Compares the word at the address in a0 with 31 and returns without changes when it differs, then tests the byte at a1 and stores 32 at that address when it is not negative.
- `store_byte_when_not_greater_zero`: Compares the word at the address in a0 with 31 and returns without changes when it differs, then stores 32 at that address when the byte at a1 is not greater than zero.
- `skip_printer_byte_if_not_33_2`: Reads a byte from the printer data buffer pointed to by a0 and, if that byte is not 33, increments the counter at a1, then returns.
- `skip_printer_byte_if_not_33`: Reads a byte from the printer data buffer pointed to by a0 and, if that byte is not 33, decrements the counter at a1 and jumps back to the start of the loop.
- `write_escape_sequence`: Checks whether the byte at the address in a0 equals twelve, and if so writes the byte 27 to a computed address and selects the byte 65 or 66 based on another byte, otherwise writes the byte 253 to offset 2720 of a structure when the byte at a0 is zero.
- `write_66_at_offset`: Computes a destination address from a2 and a4, then stores the byte 66 at that location before returning.
- `write_status_byte_to_structure`: Checks if the word at the address in a0 is zero, and if so, writes the byte 253 to offset 2720 of the structure pointed to by the value in d1, then returns zero.

### ImagewriterII

- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `invoke_and_check_pointer`: Reads a pointer from a global variable, invokes the routine it points to, and if that returns nonzero calls the same routine again and then the routine at offset 254 of the structure, otherwise it returns.
- `invoke_library_twice`: Loads the base address of a library from two different global variables into a6 and invokes the same routine at a fixed offset in each, then loads a byte value of 255 into d0 and returns.
- `invoke_printer_library`: Invokes the printer library routine at offset -414 three times, passing the value in a6 on the second and third calls, then returns zero.
- `return_constant_20`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_21`: Returns the constant an address in d0 to the caller without any other effect.
- `handle_pending_printer_entry`: Reads a flag from a global variable through a library base, and if it is zero stores the two parameters at offsets 300 and 336 of a structure, clears the byte at offset 302, then calls three routines through a second global base.
- `POSDIV`: Performs 32-bit division by splitting the work: it runs a sixteen-step binary subtraction loop when the divisor is large, and otherwise falls back to a helper routine for the smaller cases.
- `LONGDIV`: It splits the input into high and low halves, then over sixteen iterations rotates the low half left by one bit while folding the carry into the high half and subtracts a threshold from the high half whenever it is large enough, incrementing the result each time.
- `ulmult`: It multiplies two 32-bit values by combining four 16-bit partial products, scaling the cross terms by 65536 and combining them with the low-half product into a 32-bit result.
- `ulmodt`: Swaps the values of registers d0 and d1 into d1 and d2, calls the routine at the current program counter, restores d2 from the stack, and returns.
- `uldivt`: Moves d1 into d2, copies d0 into d1, calls a helper routine, then restores d0 from d1 and d2 from the stack before returning.
- `lmodt`: Calls a helper routine, then negates the returned value when the flag left in an address register is nonzero, and returns the result.
- `ldivt`: Calls a helper routine, then returns its first result or the negation of that result when the second result left in an address register is nonzero.
- `dispatch_by_index`: It saves registers on the stack, reads three values from the stack frame, clears d0 and d3, compares a stack value against four and branches past the dispatch if it is greater, otherwise doubles that value as an index into a jump table and branches to the selected entry.
- `clear_d0_and_restore_registers_alt`: Clears register d0 and restores d2 through d5 and a2 through a4 from the stack before returning.
- `prepare_printer_output`: Loads a printer structure pointer and a buffer pointer from the stack, clears two working registers and the destination buffer, then compares a 16-bit field at offset 28 of the printer structure and branches away if it is not equal to one.
- `translate_printer_escapes_2`: Fills a buffer with a table of eleven bytes, then writes ESC 'a' and a two-digit code when a flag at offset 2648 of the structure in a4 is nonzero, and ESC 'B' when the value at offset 2650 equals 512.
- `translate_printer_escapes`: Translates printer control bytes into ESCape sequences for an imaginary writer, encoding a three-digit number for the '<' or 'A' case and a two-digit number for the space case, toggling a state flag for the 31 case, and emitting specific escape sequences for other control codes before returning the number of bytes written.
- `printer_write_escape_sequence`: Writes a printer escape sequence to the output buffer in a2, choosing the bytes based on the input byte in a1: it emits a two-character code for the byte 66, a multi-byte sequence with a decimal digit conversion for the byte 32, a single byte for the bytes 15 and 16, and other escape sequences for the bytes 47, 56, and 45.
- `write_escape_and_flag`: Compares the byte at the address in a1 with the escape character and, when they match, writes the escape byte there, sets the byte at the address in a3 to 1, then returns.
- `store_space_clear_byte`: Tests the byte at the address in a3 and, when it is non-negative, stores the value 32 at the address in a1 and clears that byte, then branches away.
- `branch_to_routine`: It is a single unconditional branch to another routine, so it performs no work of its own and simply transfers control.
- `translate_printer_control_bytes_2_3`: Translates printer control bytes into ESC/P escape sequences for an imaginary writer, writing the resulting bytes into a buffer and advancing a pointer.
- `write_four_byte_sequence`: Stores the bytes 66, 27, 108 and 48 at four consecutive locations starting at the address formed from d2 and d3, then returns the pointer advanced by four.
- `translate_printer_control_bytes_2_2`: Translates printer control bytes into ESC/P escape sequences for an imaginary writer, writing the resulting bytes into a buffer and advancing a pointer.
- `store_byte_then_branch_2`: Copies the address from register d3 into register a2, advances d3 by one, adds the value in d2 to a2, writes the byte 66 to the location pointed to by a2, then branches to the routine at offset 2c2.
- `translate_printer_control_bytes_2`: Translates printer control bytes into ESC/P escape sequences for an imaginary writer, writing the resulting bytes into a buffer and advancing a pointer, and updates a global variable based on the input byte.
- `store_byte_at_offset`: It loads the pointer in d3 into a2, increments that pointer by one, adds the value in d2 to it, then stores the byte 80 at the resulting address before branching away.
- `translate_printer_control_bytes`: Translates printer control bytes into escape sequences for an imaginary writer, writing the resulting bytes into a buffer and advancing a pointer, while tracking a digit counter and a table index.
- `store_byte_at_offset_112`: It loads the pointer in d3 into a2, increments that pointer by one, adds the value in d2 to it, then stores the byte 112 at the resulting address before branching away.
- `handle_byte_escape_sequences_3`: Tests the byte at the address in a1 and stores escape sequences at the address formed from d2 and d3, updating a global variable and returning an incremented counter for each recognized marker.
- `handle_byte_escape_sequences_2`: Checks the byte at the address in a1 and, when it is a dash or dot, stores 27 at the address formed from d2 and d3, then stores one of three bytes depending on the value of a global byte.
- `write_80_if_flag_set`: Compares a global byte with one, and when it matches stores the byte 80 at the address formed from d3 and d2 before returning.
- `copy_byte_via_pointers`: Copies the byte at the address in a2 to the byte at the address in a4, advances the source pointer by one, and returns.
- `handle_byte_escape_sequences`: Checks the byte at the address in a1 and, when it equals 46, stores the byte 27 at a computed address and then stores one of three bytes depending on a global counter, otherwise when it equals 48 it clamps a stack counter and writes the bytes 27 and 115, and when it equals 12 it adjusts the counter and writes escape sequences.
- `write_69_if_flag_set`: Compares a global byte with one, and when it matches stores the byte 69 at the address formed from d3 and d2 before returning.
- `store_byte_at_offset_78`: It loads the pointer in d3 into a2, increments that pointer by one, adds the value in d2 to it, then stores the byte 78 at the resulting address before branching away.
- `write_digit_escape`: Checks whether the word at the address in a1 equals 48, and if so adjusts a byte counter, writes the bytes 27 and 115 to two consecutive computed addresses, decrements the counter, and writes the counter plus 48 to the next address, otherwise checks for the value twelve and branches back to a shared section when a counter is within range.
- `check_byte_in_range`: Checks whether the word at a1 equals twelve, then compares a byte at a0 with 27 and with 25, and branches away when the value is out of range.
- `write_escapes_and_digit`: It writes the byte 27 followed by the byte 75 to two consecutive bytes at the address formed from d2 and d3, then adds the low byte of a table entry minus 30 to the character 48 and stores the result one byte later before returning the pointer plus three.
- `clear_d0_and_restore_registers`: Clears register d0 and restores d2 through d4 and a2 through a4 from the stack before returning.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_C-150

- `return_constant_22`: Returns the constant an address in d0 to the caller without any other effect.
- `call_offset_routine_check_result`: Reads a global variable, stores the parameter and a pointer to that variable into it, then calls a routine at a negative offset of the global variable and returns zero when that call is nonzero.
- `invoke_printer_handler`: Saves the a6 register, loads the library base pointer, invokes the handler at offset -414 with the pointer from offset 16, restores a6, then repeats the same save, handler invocation with the pointer from offset 12, and restore before placing -1 into the result register and returning.
- `call_library_routine`: It invokes a routine from a library with no arguments, then invokes it twice more passing a register value, and finally returns zero.
- `return_constant_23`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_24`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_2`: Reads a vector from a global library base and, when that vector is zero, stores the value 9 at offset 300, saves the two parameters at offsets 304 and 308, clears a byte at offset 302, then invokes four routines from another base using a value read at offset 196.
- `compute_quotient_remainder`: Splits a 32-bit input into halves and computes its quotient and remainder against a divisor, using a shift-and-subtract loop when the divisor is large and a direct modulo otherwise.
- `rotate_32bit_value_3`: Rotates a 32-bit value left by one bit across sixteen iterations, carrying bits between the high and low halves and subtracting a threshold from the high half whenever it meets that threshold.
- `multiply_32_bit_integers_3`: Multiplies two 32-bit inputs by combining four 16-bit partial products into a 32-bit result and returns that product.
- `call_and_negate_on_flag`: Calls a helper routine, then returns its first result or the negation of that result when a second value left in an address register is nonzero.
- `dispatch_by_selector`: Saves several registers, reads a selector from the stack, doubles it as an index into a jump table, and branches to the selected handler before returning.
- `return_zero_immediately`: Restores the caller's registers and returns zero immediately, performing no other work.
- `dispatch_short_param_write`: Examines the short parameter and, when it equals 1, writes several bytes and copies two bytes from a data offset, when it equals 65 it clamps two values, converts them to decimal characters, and copies ten of them into a destination, and when it equals 12 it adjusts a counter and writes escape sequences.
- `copy_ten_bytes_from_table`: Copies ten bytes from a table indexed by a running counter into a destination computed from a2 and d3, incrementing both counters each pass before returning.
- `adjust_stack_byte_and_lookup`: Tests the 16-bit value at the pointer in a3, and when it equals twelve it adjusts a stack byte, stores the bytes 27 and 64, and writes two table lookups, otherwise it writes the byte 253 when the value is zero.
- `write_escapes_and_lookup`: Computes a table index from a stack value, stores the bytes 27 and 64 at two consecutive locations, then looks up two further bytes via a helper routine and stores them before returning.
- `write_byte_to_offset`: Tests the 16-bit value at the pointer in a3 and, when it is zero, writes the byte 253 to offset 2720 of the structure pointed to by a4, then returns zero.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### generic

- `return_constant_25`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_26`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_27`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero`: The routine returns a constant zero value.

### Brother_HR-15XL

- `return_constant_28`: Returns the constant an address in d0 to the caller without any other effect.
- `invoke_handler_twice_2`: Saves the parameter in a global variable and stores a pointer to that global variable, then calls a routine at offset 552 of a structure.
- `invoke_handler_twice`: Saves the current context, calls a routine at offset 414 of a global structure twice using two different base pointers, then loads 255 into the return register and branches back into the calling routine.
- `invoke_routine_three_times`: Invokes a routine from a global library base once with no argument and twice with a shared argument, then returns zero.
- `return_constant_29`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_30`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_31`: Returns the constant an address in d0 to the caller without any other effect.
- `select_printer_command`: Loads the printer structure pointer from the stack and compares its command byte at offset 0 against the value 1, then branches to a printer routine if it matches.
- `build_printer_buffer`: Copies up to twelve bytes from a table into a buffer, then appends characters from a string until a zero byte, and returns the new buffer length.
- `copy_and_normalize_bytes`: Copies characters from a source buffer into a destination buffer until a zero byte is reached, then stores two bytes from a table at offsets 2621 and 2623 of a structure and writes the value 65 to a byte.
- `copy_table_bytes_loop`: Copies bytes from a table into a buffer, incrementing the source and destination offsets each time and stopping after eleven entries.
- `check_escape_store_values`: Compares the byte at the address in a0 with the escape character and, when they match, stores 32 at that address and 1 at the address in a1, then branches away.
- `copy_table_bytes_eleven`: Reads a byte from a global table and writes it to a location pointed to by a register, then increments the index and repeats until the index reaches eleven.
- `store_printer_status_byte_2`: Compares the 16-bit value at the address in a0 with 29 and returns without changes if they differ.
- `store_printer_status_byte`: Compares the 16-bit value at the address in a0 with 29 and, if they differ, returns without changes.
- `store_byte_when_not_negative_2`: Compares the word at the address in a0 with 29 and returns without changes when they differ, then stores the byte 32 at that address when the byte at a1 is not greater than 30.
- `store_byte_when_not_negative`: Compares the word at the address in a0 with 30 and returns without changes when it differs, then stores 32 at that address when the byte at a1 is not negative.
- `adjust_status_byte`: Compares the 16-bit value at the pointer in a0 with 31 and, when they differ, branches away.
- `compare_word_branch_add_byte`: Compares the 16-bit value at the pointer in a0 with 33 and, when they differ, it branches away; otherwise it adds one to the byte at the pointer in a1 and jumps to a routine.
- `decrement_status_byte`: Compares the 16-bit value at the pointer in a0 with 33 and, when they differ, branches away.
- `write_byte_and_status`: Tests the 16-bit value at the pointer in a0, and when it equals twelve it writes the byte 27 and either the byte 65 or 66 depending on a stack byte, otherwise it writes the byte 253 when the value is zero.
- `write_single_byte_buffer`: Writes the byte 66 to a single location in a buffer pointed to by a computed address, then returns.
- `write_status_byte_to_structure_2`: Tests the 16-bit value at the pointer in a0 and, when it is zero, writes the byte 253 to offset 2720 of the structure pointed to by a value in d1, then returns zero.

### HP_LaserJet

- `return_constant_32`: Returns the constant an address in d0 to the caller without any other effect.
- `invoke_printer_routines`: Stores the incoming parameter into a global variable, then reads a pointer from offset -552 of that variable and calls the routine it points to.
- `store_printer_offsets`: Loads the value at offset 16 of the structure in a1 into a global variable, then loads the value at offset 12 of that structure into the same global variable.
- `call_routine_three_times`: Calls the routine referenced by the pointer at offset -414 of the global variable in d1 three times in a row, passing the value in a6 on the second and third calls, then returns.
- `return_constant_33`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_3`: Calls a routine at offset 104 of a global structure and, if it returns zero, stores the value 9 at offset 300, saves the two passed parameters at offsets 304 and 308, clears the byte at offset 302, then invokes four routines through a base address read at offset 292.
- `assemble_hp_laserjet_output`: Copies eleven bytes from a global table into the printer output buffer, then writes escape sequences and status bytes at fixed offsets depending on values read from a global status structure, and finally calls the printer output routine.
- `assemble_printer_output`: Copies eleven bytes from a global table into the printer buffer, then writes escape sequences and status bytes at fixed offsets depending on the printer model detected, and finally calls another routine to send the assembled bytes.
- `write_printer_escape_sequence`: Copies eleven bytes from a global table into a buffer pointed to by a4, then stores the escape byte from a1 into the printer output buffer in a2 via printer_write_escape_sequence, returning the new buffer position.
- `copy_eleven_bytes_then_call`: Copies eleven bytes from a global table into a buffer, then calls another routine twice with values derived from a structure.
- `compare_word_store_on_null`: Compares the 16-bit value at the pointer in a2 with 27 and branches away when they differ, then tests the byte at the pointer in a3 and branches away when it is nonzero; otherwise it stores 32 at that pointer, stores 1 in the byte at a3, and jumps to a routine.
- `store_printer_code`: Compares the word at the address in a2 with 27 and returns when they differ, then tests the byte in a3 and returns when it is negative, otherwise stores 4 at that address.
- `store_when_not_greater`: Compares the word at the address in a2 with 28 and returns when they differ, then tests the byte in a3 and stores 33 at that address and 253 at another address when it is not greater than zero, otherwise it branches to a later routine.
- `store_two_when_byte_not_greater_2`: Compares the word at the address in a2 with 29 and returns when they differ, then stores the word 33 at that address and the byte 253 at the address in a3 before jumping to a shared routine.
- `store_two_when_byte_not_greater`: Compares the word at the address in a2 with 29 and returns when it differs, then stores 2 at that address when the byte at a3 is not greater than zero.
- `store_when_not_less`: Compares the word at the address in a2 with 30 and returns when they differ, then tests the byte in a3 and stores 32 at that address and 253 at another address when it is greater than or equal to zero, otherwise it branches to a later routine.
- `compare_word_loop_increment_byte_3`: Compares the word at the address in a2 with the decimal constant 33 and, when they differ, branches back to repeat the comparison.
- `compare_word_loop_increment_byte`: Compares the word at the address in a2 with the decimal constant 33 and, when they differ, branches back to repeat the comparison.
- `compare_word_decrement_byte`: Compares the word at the address in a2 with the decimal constant 33 and, when they differ, branches back to an earlier instruction to repeat the comparison, otherwise it decrements the byte at the address in a3 and returns.
- `compare_word_loop`: Compares the word at the address in a2 with the decimal constant 64 and, when they differ, branches back to an earlier instruction, otherwise it returns.
- `decrement_and_copy_byte_2`: Decrements the byte at the pointer stored at offset 28 of the structure in a6, saves the result into a global variable, then copies a byte from the register pair into the adjacent position depending on whether that position is already zero.
- `increment_and_copy_byte`: Increments the value loaded from offset 28 of the structure in a6, copies the resulting byte from the source buffer into the destination buffer, then branches away.
- `store_printer_four_on_escape`: Compares the byte at the address in a0 with the escape character and returns without changes when they differ.
- `compare_word_branch_back`: Compares the word at the address in a2 with the decimal constant 57 and, when they differ, branches back to an earlier instruction, otherwise falling through to a nearby routine.
- `copy_table_store_sequences`: Copies eleven bytes from a global table into a destination buffer, advancing the index until it reaches 11, then stores two printer command sequences and returns the final index.
- `write_byte_on_null_check_alt`: Checks if the word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of the structure pointed to by d2.
- `convert_value_to_ascii_digits`: Converts a value into three ASCII digits by subtracting 100 when needed, then stores the digit characters at consecutive bytes of the structure in a2 and calls a routine to fetch the next value.
- `subtract_store_byte_loop`: Compares the value in d0 with 99 and branches away when it is less than or equal to that, then advances a pointer by a register and stores the byte 49 at the new location, subtracts 100 from d0, and jumps back to the comparison.
- `convert_to_ascii_2`: Writes 48 to the location pointed to by a register pair, writes 48 to the next location when the value in d0 is below 10 otherwise it stores the tens digit plus 48, then calls another routine and writes its result plus 48 to the following location.
- `convert_to_ascii`: Writes the byte 48 to a memory location, calls another routine, and then writes the returned byte incremented by 48 to the next memory location.
- `call_two_subroutines`: Pushes two global addresses on the stack, then calls the routine at offset 100 of a global variable with those two parameters, and afterwards calls the routine at offset 104 of the same global variable.
- `divide_32bit_value`: Divides a 32-bit value by a 16-bit divisor using a sixteen-iteration loop, returning the quotient and remainder.
- `rotate_32bit_value_4`: Splits a 32-bit value into its high and low halves, then loops sixteen times rotating the value left by one bit using the carry flag to move bits between the halves and subtracting a threshold from the high half when the rotated value meets or exceeds it.
- `multiply_32_bit_integers_4`: Computes the 32-bit product of two 32-bit input values using 16-bit partial products and combines them to form the final 32-bit multiplication result.
- `negate_result_on_flag`: Calls a helper routine and negates the returned value when the flag left in an address register is nonzero.
- `restore_registers_dispatch`: Restores three data registers and three address registers from the stack, then dispatches to a routine selected by a jump table.
- `restore_registers_and_return_2`: Restores the three data registers d2 and d3 and the address registers a2, a3 and a4 from the stack, then returns.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `push_registers_compare_word`: Pushes registers a2 and a3 onto the stack, loads the word at offset 14 of the stack into d0, sets a1 to 4, a3 to 47, and a2 to 46, then compares d0 with 768 and branches away on carry or above; otherwise it jumps to a routine.
- `compare_value_branch_4`: Compares the 16-bit value in d0 with 256, then branches based on the carry and higher conditions to route control into the middle of a nearby routine.
- `compare_d0_branch`: Compares the 16-bit value in d0 with 512, and when they are not equal it branches forward, otherwise it branches to a nearby routine.
- `compare_value_branch`: Compares a value against 1024 and, when they differ, branches to another routine, otherwise it returns.
- `init_structure_values`: Writes 32-bit values at offsets 26 and 30 of a structure, 16-bit values at offsets 34 and 34, a byte at offset 48, and two bytes into separate buffers.
- `store_structure_constants`: Stores the 32-bit value 800 at offset 26 and the 32-bit value 1060 at offset 30 of a structure, then writes a byte at offset 45.
- `store_structure_constants_alt`: Stores the 32-bit value 1200 at offset 26 and the 32-bit value 1590 at offset 30 of a structure, then writes a byte into a buffer.
- `initialize_structure_values`: Writes the 32-bit values 2400 and 3180 at offsets 26 and 30 of the structure pointed to by a1, writes the 16-bit value 300 at offsets 36 and 34, sets a global variable to 51, and stores the byte 48 at two separate memory locations.

### Okidata_292

- `return_constant_34`: Returns the constant an address in d0 to the caller without any other effect.
- `invoke_and_check_result`: Saves the passed parameter into a global variable, then calls a routine located at offset 228 of that global variable and stores its result.
- `copy_printer_pointer`: Copies the value from the global variable at offset 16 into the pointer at offset 16 of the structure in a1, then loads the value from offset 12 of that global variable into a1 and returns.
- `invoke_routine_three_times_2`: Invokes the routine at offset -414 of a global base three times, the first with no arguments and the next two with a single argument, then returns zero.
- `return_constant_35`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_36`: Returns the constant an address in d0 to the caller without any other effect.
- `handle_pending_dispatch_entry`: Calls a routine via a pointer in a global variable and, if it returns zero, stores the two parameters at offsets 304 and 336 of that structure, writes the value 9 at offset 300, clears the byte at offset 302, then calls four routines through a second global base using the value at offset 292.
- `compute_offsets_and_write_printer_data`: Computes several byte offsets from a value read from a table, writes a 32-bit result into a structure at offset 96, then branches on a parameter to set a byte in a table, call a routine at offset 100, or write a 16-bit value and escape sequences into a buffer.
- `restore_registers_and_return_zero`: Restores registers from the stack and returns zero without performing any other operation.
- `init_printer_output_buffer_2`: Loads the printer structure pointer and a status word from the stack, sets a counter and a mode value to zero and three respectively, then compares the status word with one and branches away if they differ.
- `format_printer_buffer`: Copies up to 20 bytes from a table into a buffer, writes control bytes at fixed offsets, then appends characters from a text string whose -2 markers are converted to nulls.
- `store_byte_from_register_2`: Copies a long from a global variable into a data register, increments a counter, stores the byte value 15 at a computed address, and loads a base address from a global variable.
- `process_printer_output`: Copies a NUL-terminated string from the buffer at a0 into the output buffer at d2 byte by byte, then branches on the byte read from the printer control structure at a2 to emit escape sequences, select the printer mode, or copy four bytes from a table, otherwise it returns zero.
- `copy_four_bytes_table`: Copies four bytes from a table into consecutive memory starting at the address in d2, incrementing d3 and d6 each time, then returns.
- `copy_byte_from_table_3`: Compares the word at the address in a2 with 66 and returns when they differ, then stores the byte 1 at a global variable, loads a pointer from the address in d1, and stores the byte 96 at offset 2646 of that structure when the word there equals 2048.
- `store_byte_when_word_not_equal`: Copies the value in d1 through a pointer into a0, compares the word at a0 plus offset 1621 with 1024 and returns when they differ, then stores 137 at that address.
- `store_byte_from_register_3`: Copies the value in d5 into a0, stores the byte 80 at the location pointed to by a0, and jumps to a routine.
- `advance_printer_index`: Saves the current value of d3, increments it by one, then stores the byte at the address in d6 through the pointer in a1, and finally increments d6 by one.
- `check_status_word_value`: Compares the word at the address in a2 with the decimal value 32 and returns if they differ, then tests the byte at a3 and returns if it is nonzero.
- `clear_byte_if_nonnegative`: Tests the byte pointed to by a3 and, if it is negative, returns without changes; otherwise it clears that byte and stores the value 30 at the address in a2.
- `load_value_and_branch`: Loads the value 255 into d0 and branches to the routine at offset 748; it contains no loop and makes no call.
- `check_byte_if_positive`: Compares a word at the address in a2 with the decimal value 33 and, if they differ, jumps away; otherwise it tests the byte at a3 and, when that byte is nonzero, returns without changes.
- `clear_byte_if_positive`: Tests the byte pointed to by a3 and, if it is positive, returns without changes; otherwise it clears that byte and stores the value 28 at the address in a2.
- `return_immediately`: Returns immediately by branching to the following routine.
- `map_status_byte_to_output`: Reads the byte at the address in a2 and sets the byte pointed to by a3 to a value depending on that byte, handling the value 2 by writing the bytes 27 and 74 followed by a stack value at the address in d2 plus d3.
- `check_byte_if_word_equals_twelve`: Compares the word at the address in a2 with 12 and, if equal, checks a byte from a stack address, setting it to 30 if it is below 37, then returns.
- `write_printer_prefix_bytes`: Writes the bytes 27 and 114 to consecutive bytes starting at the address in d2, then stores a byte from a table indexed by a stack value minus 30 at the following byte and returns the address advanced by three.
- `write_status_byte_2`: If the word at the address in a2 is zero, it writes the byte 253 to offset 2720 of the structure pointed to by the address in d1.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Okimate_20

- `return_constant_37`: Returns the constant an address in d0 to the caller without any other effect.
- `call_and_check_result`: Reads a global variable, stores several values into a structure, and calls a routine at a computed offset, returning early with zero when a nested result is nonzero.
- `call_library_twice`: Loads two global addresses into a register, sets up the library base, calls a routine at a fixed offset, restores the base, and repeats this sequence for a second address before loading a value into a register and jumping away.
- `init_printer_status`: Invokes a printer library function three times with the same argument, then stores a routine address into a global variable depending on a status field, and returns zero.
- `store_pointer_on_match`: Loads a pointer from a global variable into a0, compares the word at offset 166 of that pointer with 2 and jumps to a routine when they are equal; otherwise it loads a fixed address into a0, stores that address into a global variable, and jumps to a routine.
- `set_function_pointer`: Computes its own address, stores it into a global variable at offset 54, and returns zero.
- `return_constant_38`: Returns the constant an address in d0 to the caller without any other effect.
- `okimate_20_printer_control`: Implements a printer output routine for the Okimate 20, dispatching on a command byte in d1.
- `restore_registers_and_return_3`: Clears register d0 and restores registers d2 through d6 and a2 through a5 from the stack, then returns.
- `handle_printer_commands_2`: Dispatches printer operations by a command code: for command zero it computes a value from a global flag and stores several results, for command one it writes an OR'd byte into a buffer, for command two it reads a status byte and returns it, for command three it clears a region and writes escape and control bytes, and for command four it reads a status byte and returns.
- `restore_registers_and_return_4`: Restores the registers d2 through d6 and a2 through a5 that were saved on the stack, then returns.
- `divide_32bit_by_16bit`: Computes the quotient and remainder of a 32-bit value divided by a 16-bit divisor, using a shift loop when the divisor exceeds fifteen and otherwise using the hardware divide, then returns both results.
- `multiply_32bit_value`: Multiplies a 32-bit value by two sixteen times using a carry loop, subtracting a threshold from the high half whenever it reaches that value.
- `multiply_32bit_integers`: Multiplies two 32-bit values using sixteen-bit partial products and combines them into a 32-bit result, returning the product.
- `forward_call_to_helper_3`: Calls a helper routine and returns, with no other work.
- `restore_registers_setup_loop`: Restores five data registers and three address registers from the stack, loads three pointers from the stack into registers, initializes several counters to zero, loads two decimal constants, then compares the word at the first pointer with one before branching.
- `copy_table_write_sequences`: Copies fifteen bytes from a global table into a destination buffer, then writes escape sequences into that buffer depending on which of several status words hold specific values, and returns the final index.
- `write_status_byte_on_carriage_return`: Writes the byte 253 to the address in a1 when the byte at the address in a0 equals the carriage return character and the byte at the address in a1 is zero, then branches back to the shared routine.
- `write_carriage_returns_loop_2`: Writes carriage return bytes into a buffer while advancing a pointer, looping until a counter reaches a value read through another pointer, then returns the updated pointer.
- `translate_printer_codes`: Translates printer control codes into escape sequences and carriage returns for the Okimate 20, writing the resulting bytes into a buffer and calling the carriage-return loops when the printer is in the selected mode, otherwise toggling or resetting internal state flags.
- `store_carriage_returns_loop`: Writes carriage return bytes into a buffer, advancing a pointer by one each time and incrementing a counter, looping until the counter reaches a value read through another pointer, then returns the incremented pointer value.
- `write_carriage_returns_loop_3`: Writes carriage return bytes into a buffer while advancing a pointer, looping until a counter reaches a value read through another pointer.
- `write_carriage_returns_loop`: Writes carriage return bytes into a buffer while advancing a pointer, looping until a counter reaches a value read through another pointer, then returns the updated counter.
- `compare_then_store_printer_byte`: Compares the byte at the pointer in a0 with 32 and returns without changes when they differ.
- `check_byte_then_store_30`: Tests the byte at the pointer in a2 and, if it is negative, returns without changes.
- `load_and_branch_2`: Loads the value 255 into a data register and branches to another routine, then returns.
- `check_printer_status`: Checks the byte at a0 and returns unless it equals 33, then tests the byte at a2, clears it to 255, writes 29 to the word at a0, and returns.
- `check_and_clear_status_byte`: Tests the byte at a2 and returns if it is not greater than zero, then clears that byte, writes the decimal value 28 to the word at a0, and jumps away.
- `jump_to_other_routine`: Contains a single unconditional jump to the routine at offset 604 and does nothing else.
- `map_byte_to_values`: Compares a byte read through a pointer against six constants and stores a matching byte value at a destination pointer, writing a different byte for each match.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Diablo_Advantage_D25

- `return_constant_39`: Returns the constant an address in d0 to the caller without any other effect.
- `init_and_call_twice`: Copies the incoming parameter into a global variable, stores the address of that variable in another global, then calls the routine at offset 552 of the structure.
- `call_printer_twice`: Restores the context pointer from the stack, then calls the printer library routine twice using the pointers in the global variables, loads 255 into the return register, and jumps to the shared return code.
- `call_library_routine_2`: Calls a library routine three times with no argument, then with a register value, and finally with the same register value again before returning.
- `return_constant_40`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_41`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_42`: Returns the constant an address in d0 to the caller without any other effect.
- `restore_registers_check_parameter`: Restores the saved registers from the stack and loads a0, a4 and a1 from the incoming parameters, then initializes d2, d3 and d1 to zero and compares the word at the address in a0 with the decimal constant 1, branching away when they differ.
- `fill_buffer_from_table`: Copies twelve bytes from a global table into a buffer, then appends characters from a string until a zero byte is reached.
- `copy_buffer_transform_bytes`: Copies bytes from a source buffer into a destination buffer until a zero byte is reached, then stores two bytes from a structure, writes the decimal constant 65 to a location, and copies a table of eleven bytes based on the stored value.
- `copy_byte_until_limit`: Increments two counters, computes a pointer from a base register and the first counter, copies one byte from a table indexed by that counter into the destination, then loops while the counter stays below twelve.
- `store_escape_when_match_3`: Checks whether the byte at the address in a0 equals the escape character and, when it does, stores the value 4 there, otherwise it returns.
- `check_escape_store_four_2`: Compares the byte at the address in a0 with the decimal constant 27, and when they differ it returns, otherwise it tests the byte at the address in a1 and stores the decimal constant 4 at the first address when that value is not negative.
- `clamp_byte_when_below_zero`: Compares the 16-bit value at the address in a0 with 31 and returns without changes when it differs, then tests the byte at the address in a1 and stores 32 at the address in a0 when that byte is zero or negative.
- `build_printer_control_sequences`: Checks whether a status word equals eight thousand and, if so, increments a counter and stores the byte fifteen at a computed address.
- `clamp_byte_when_not_greater_zero`: Compares the 16-bit value at the address in a0 with 31 and returns without changes when it differs, then tests the byte at the address in a1 and stores 2 at the address in a0 when that byte is zero or negative.
- `compare_then_store_printer_byte_2`: Compares the byte at the pointer in a0 with 32 and returns without changes when they differ.
- `compare_word_then_store_33_32`: Compares the word at the pointer in a0 with 31 and returns if they differ.
- `increment_byte_when_not_space`: Compares the byte at the pointer in a0 with 32 and returns when they differ.
- `check_byte_then_decrement`: Compares the byte at the pointer in a0 with 33, and if they differ it returns, otherwise it decrements the byte at the pointer in a1 and jumps to the shared code.
- `write_status_byte_if_zero`: Checks if a sixteen-bit word at a pointer is zero and, if so, writes the byte 253 to offset 2720 of the structure pointed to by a register.

### Alphacom_Alphapro_101

- `return_constant_43`: Returns the constant an address in d0 to the caller without any other effect.
- `init_printer_context`: Reads a pointer from a global variable, stores a parameter and a pointer to that variable into a structure at offsets 34, 38 and 42, then calls a routine at offset 200 of the structure it just filled.
- `call_twice_with_saved_a6`: Saves the current value of a6, loads a pointer from a global variable into a6, calls the routine at a fixed offset from a6, restores a6, then repeats the same load, call, and restore a second time before returning a value of -1 in d0.
- `invoke_printer_routine`: Dereferences a routine pointer stored in a global variable and invokes it three times, passing the value in a6 to the second and third calls, then returns.
- `return_constant_44`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_45`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_46`: Returns the constant an address in d0 to the caller without any other effect.
- `save_registers_and_check_word`: Saves registers d2 to d3 and a2 to a5 on the stack, loads three pointers from the stack, and clears d1, d2 and d3 to zero.
- `build_printer_buffer_2`: Writes twelve table entries into a buffer indexed by a counter, then inspects a status byte and, when it equals 65, writes an escape sequence and fills the remaining buffer with spaces and control codes before returning the new length.
- `fill_spaces_then_escape`: Writes the space character into a buffer while an index stays below a stored limit, then writes the bytes 27 and 57.
- `fill_buffer_then_escape`: Loops writing the space character into a buffer while an index stays below a stored limit, then writes an escape sequence of five bytes and returns the final index plus seven.
- `check_escape_byte`: Compares the byte at the address in a1 with the escape value 27 and returns immediately when they differ, then compares the byte at a2 with the same value and returns when it differs.
- `check_escape_and_store_four`: Reads the byte at the address in a1 and returns immediately when it is not the escape value 27, then reads the byte at a2 and returns when it is negative, otherwise stores the word 4 through the pointer in a1 and jumps to offset 184.
- `check_word_store_value`: Compares the word at the address in a1 with 28 and, when they match and the byte at the address in a2 is non-negative, stores the value 33 at that address, then branches away.
- `compare_word_then_store_33_253`: Compares the word at the pointer in a1 with 29 and returns if they differ.
- `check_word_then_store_two`: Compares the word at the pointer in a1 with 29, and if they differ it returns, otherwise it tests the byte at the pointer in a2 and returns when that value is not below zero, then stores 2 into the word and jumps to the shared code.
- `check_word_store_space`: Compares the word at the address in a1 with the escape character and, when they match and the byte at the address in a2 is non-negative, stores the value 32 at that address, then branches away.
- `store_status_byte_pair`: Compares the word at the address in a1 with 31 and returns when they differ, then stores 33 at that address and tests the byte in a2, storing 32 there when it is greater than or equal to zero, clearing that byte, and finally branching to a later routine.
- `compare_word_increment_byte_2`: Compares the word at the address in a1 with the decimal constant 32 and, when they differ, returns.
- `compare_word_loop_decrement_byte`: Compares the word at the address in a1 with the decimal constant 33 and, when they differ, branches back to repeat the comparison.
- `write_escape_or_fill`: Stores the escape byte 27 at the destination, then writes the letter A there when a byte equals 31 and the letter B otherwise.
- `write_letter_b_here`: Increments a register, adds it to a base pointer, writes the letter B into the resulting location, and jumps to the shared return code.
- `handle_byte_66_or_zero`: If the byte at the pointer in a1 is 66 it writes a carriage return, fills 80 bytes with the value 8 followed by an escape sequence, then fills 80 more bytes with spaces followed by another escape sequence.
- `write_status_byte_if_zero_2`: Tests whether the 16-bit word at the address in a1 is zero; if it is nonzero it returns immediately, otherwise it stores the byte 253 at offset 2720 of the structure pointed to by d2 and returns zero.

### Epson_JX-80

- `return_constant_47`: Returns the constant an address in d0 to the caller without any other effect.
- `init_printer_struct`: Stores the argument in a global variable, calls a routine at offset 200 of the structure in that variable, and if it returns nonzero calls another routine at offset 418 of the same structure, then calls a routine at offset 414 twice, returning zero if the second call returns nonzero.
- `call_library_twice_2`: Loads a global variable into a1, sets up the stack frame, and calls a library routine, then repeats the same setup and call with a1 pointing to a different global address.
- `call_table_routine_thrice`: Invokes a routine from a table at a negative offset of a global base with no arguments, then calls the same routine twice more passing the value in a6, and finally returns.
- `return_constant_48`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_49`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_4`: Calls a routine from a table at offset 104 of a global base and, when it returns zero, stores the value 9 at offset 300, saves the two parameters at offsets 304 and 308, clears the byte at offset 302, loads a pointer from offset 292, and then invokes four routines through that pointer and the global base.
- `format_printer_command_block`: Writes a 24-byte ESC/P command block into a printer buffer, using the escape byte 27, the letter R, a status byte, and a 16-bit value stored little-endian, then repeats for additional rows.
- `restore_registers_and_return_5`: Restores registers d2, d3, d4, a2, a3 and a4 from the stack, sets d0 to zero, and returns.
- `restore_registers_setup_branch`: Restores six data registers and four address registers from the stack, loads pointers and initializes counters, loads two decimal constants, then compares the word at the first pointer with one before branching to a shared routine.
- `process_printer_output_3`: Fills a buffer with 18 bytes copied from a global table starting at offset 0, then zeroes the bytes at offsets 12 and 18.
- `handle_printer_commands_3`: Copies two bytes from a printer structure into a buffer and, based on the command byte at offset 160, emits escape sequences or copies eight entries from a table, otherwise it returns zero.
- `copy_byte_pair`: Loads a byte from a source address into d0, decrements it, stores the result at a destination, then loads the next source byte and stores it at the next destination address.
- `copy_byte_then_loop`: Copies the value in d3 into d0 and increments d3, then computes a destination address from d1.
- `translate_printer_control_codes`: Translates printer control codes from the source buffer into ESC sequences written to the printer output buffer, using a table of values indexed by a counter and a status byte to decide which escape code to emit.
- `store_byte_if_buffer_valid`: Loads a pointer from the location pointed to by a1 into a0, returns immediately when the word at offset a56 of that buffer is not 1024, otherwise stores the byte 137 at the start of the buffer and jumps to offset 1a8.
- `store_byte_and_jump_shared`: Stores the byte 80 at the location pointed to by a0 and jumps to offset 1a8.
- `copy_table_byte_loop`: Copies a byte from a table indexed by a register incremented each pass into a buffer address derived from another register, then loops back while the counter stays below eight.
- `check_status_and_mark`: Tests the 16-bit word at the address in a2 and returns when it is not the value 32, then checks a byte at a3 and returns when that byte is nonzero.
- `clear_flag_if_positive`: Tests the byte at a3 and returns when it is negative, otherwise clears that byte and writes the value 30 into the word at a2 before falling through to the shared code.
- `load_return_and_jump`: Loads 255 into the return register and jumps to the shared return code.
- `store_escape_marker`: Compares the word at the address in a2 with 33 and returns when they differ, then tests the byte in a3 and returns when it is nonzero, otherwise stores 255 in that byte and 29 at the address.
- `store_when_byte_zero`: Tests the byte in a3 and returns when it is greater than zero, otherwise clears that byte, stores 28 at the address in a2, and branches to a later routine.
- `empty_branch_return`: Empty routine that only branches to the shared return code.
- `handle_escape_sequences_and_lookup`: Compares the byte at a2 against a series of values and stores a matching result byte at a3 or a stack location, writes the escape byte and the letter J when the byte is 2, stores the byte 253 at offset aa0 of a buffer when the byte is 0, otherwise writes the escape byte and the letter R plus a table byte indexed by a stack value and jumps to a shared routine.
- `handle_escape_sequences`: When the word at a2 equals 4 it writes the escape byte 27 and the letter J followed by a byte from the stack.
- `check_buffer_byte_range`: Tests the 16-bit word at a2 and returns when it is not twelve, then inspects a byte loaded from a stack address and returns when it is not thirty-nine.
- `write_escape_and_lookup`: Writes an escape byte and the digit 2 into a buffer, stores a byte looked up from a table indexed by a stack value into the next location, and returns the base pointer plus three.
- `write_status_byte_if_zero_3`: Tests whether the 16-bit word at the address in a2 is zero; if it is nonzero it returns immediately, otherwise it stores the byte 253 at offset 2720 of the structure pointed to by d2 and returns zero.
- `call_library_vector_198_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Epson

- `return_constant_50`: Returns the constant an address in d0 to the caller without any other effect.
- `call_twice_and_check`: Stores the incoming argument into a global variable, reads a routine pointer from that variable, and calls it twice, returning zero only when both calls succeed.
- `call_twice_store_byte`: Saves the current value of a6, loads a routine pointer from a global variable and calls it, then repeats the same save, load and call a second time, and finally stores a byte with all bits set into the structure pointed to by a1.
- `call_routine_via_table`: Reads a routine entry from a table of function pointers and calls it once without arguments, then calls the same entry two more times passing the value in a6, and finally returns zero.
- `return_constant_51`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_52`: Returns the constant an address in d0 to the caller without any other effect.
- `handle_pending_dispatch`: Calls a routine through a pointer in a global table and, if it returns zero, stores the value nine at offset 300 of the structure, writes the two parameters at offsets 304 and 308, clears the byte at offset 302, then calls four routines through pointers derived from the base at offsets 300 and 308.
- `handle_printer_output`: Dispatches on a 0-5 selector to manage a printer output buffer: computes and stores offsets, writes escape sequences and text into the buffer, clears and terminates it, and ORs a byte into it, calling a printer handler routine as needed.
- `restore_registers_from_stack`: Restores the caller's d2, d3, a2, a3 and a4 registers from the stack and returns.
- `restore_registers_setup_compare`: Restores five data registers and four address registers from the stack, loads three pointers into registers, initializes two counters to zero, loads two decimal constants, then compares the word at the first pointer with one before branching.
- `build_printer_output_buffer`: Fills a buffer with eight bytes copied from a global table and terminates it, then writes a status byte and a couple of control bytes into a structure.
- `handle_printer_output_2`: Reads two bytes from a printer structure at offsets 165 and 167 and stores them into a destination buffer, then writes the value 65 into a status byte at offset 176.
- `copy_two_consecutive_bytes`: Copies the byte pointed to by d2 into the location pointed to by d4, then copies the byte at the address one past d2 into the location one past d4, and returns.
- `copy_byte_indexed_then_compare`: Copies the value in d3 to d0 and increments d3, then computes a pointer by adding d0 to a4.
- `handle_printer_control_byte`: Writes the escape byte into a buffer when the byte at the address in a1 is 66, then selects a value from a table based on a word at a fixed offset and copies eight bytes from a global table into a buffer.
- `check_printer_status_and_write`: Loads the printer structure pointer from a global variable, checks whether the printer status byte at offset 1024 equals 64, and if so stores the byte value 137 at the address in register d2.
- `store_printer_byte`: Copies the byte value 80 through the pointer in a0 and then jumps into another routine.
- `copy_byte_from_table_2`: Advances the source pointer in a2 by one byte, then copies the byte at the offset of d5 from the table pointed to by a0 into that source location, and returns.
- `check_byte_and_store_seven`: Returns immediately when the byte at a1 is not 32, then returns when the byte at a3 is nonzero, otherwise stores the byte 1 at a3, stores the word 27 at a1, and jumps to offset 288.
- `clear_byte_and_store_thirty`: Returns when the byte at a3 is non-negative, otherwise clears that byte, stores the word 30 at a1, and jumps to offset 288.
- `load_return_and_jump_2`: Loads the value 255 into the return register and jumps to offset 28a.
- `store_escape_marker_a1`: Compares the word at the address in a1 with 33 and returns when they differ, then tests the byte in a3 and returns when it is nonzero, otherwise stores 255 in that byte and 29 at the address.
- `store_when_byte_zero_alt`: Tests the byte in a3 and returns when it is greater than zero, otherwise clears that byte, stores 28 at the address in a1, and branches to a later routine.
- `set_return_255`: Loads the value 255 into the return register and jumps to the shared return routine, then returns.
- `handle_escape_sequences_and_store`: Compares the byte at a1 against a series of values and stores a matching result byte at a3 or a stack location, writes the escape byte and the letter J plus a stack byte when the byte is 2, stores the byte 253 at offset aa0 of a buffer when the byte is 0, and otherwise returns zero.
- `write_byte_if_zero`: Tests whether the 16-bit word at the address in a1 is zero; if it is nonzero it returns immediately, otherwise it stores the byte 253 at offset 2720 of the structure pointed to by d1 and returns zero.
- `call_library_vector_198_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### HP_LaserJet_PLUS

- `return_constant_53`: Returns the constant an address in d0 to the caller without any other effect.
- `invoke_handler_twice_3`: Loads a global variable into a register, stores the argument and the address of that global variable into consecutive fields of a structure, then calls a routine at offset 552 of the structure twice using two different base pointers and branches to a routine at offset 414 when the first call returns zero.
- `call_library_twice_3`: Loads the base address of a library from offset 16 and calls a routine at offset 228 of that library, then loads the base address from offset 20 and calls the same routine at offset 228, and finally returns a value of 255 in the result register.
- `call_routine_via_global_pointer`: Calls another routine three times through a pointer stored in a global variable, passing the value in a6 on the second and third calls, then returns.
- `return_constant_54`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_pending_entry_5`: Calls a routine referenced from a global base and, if that routine returns zero, stores the value 9 at offset 300, saves the two parameters at offsets 304 and 308, clears a byte at offset 302, then invokes several routines through a second global base using a value read at offset 292.
- `assemble_printer_output_2`: Copies eleven bytes from a global table into the printer output buffer and writes escape sequences and status bytes at fixed offsets depending on values read from a global status table, then calls the printer escape-sequence handler.
- `build_printer_escape_sequence`: Copies eleven bytes from a global table into the printer output buffer, then writes printer-specific escape sequences depending on the selected printer model, and finally calls the printer escape-sequence routine with the constructed command.
- `handle_printer_escape_sequence`: Copies ten bytes from a global table into a buffer pointed to by a4, then stores the escape byte from the printer structure at offset 269 of the data block in a2 and writes the character 65 to the printer output byte in a2.
- `copy_table_bytes_loop_2`: Copies bytes from a global table into a destination buffer, advancing two counters each iteration, until the second counter reaches eleven, then returns the incremented first counter.
- `check_byte_then_store`: Compares the word at the pointer in a2 with 27 and, when they differ, branches away; then tests the byte at a3 and, when it differs, branches away; otherwise stores 32 into the word at a2, stores 1 into the byte at a3, and branches to a routine at offset 818.
- `store_printer_escape_bytes`: Compares the 16-bit value at the address in a2 with 27 and skips the store when they differ, then stores 51 at that address and 253 at the address in a3 when the byte in a3 is not negative.
- `store_printer_status_when_ready`: Compares the 16-bit value stored at the address in a2 with the decimal constant 29 and returns without changes when they differ, then checks the byte at the address in a3 and returns when it is not greater than zero, otherwise stores the word 33 at the address in a2 and the byte 253 at the address in a3.
- `clear_d1_and_branch`: Clears d1 and branches to the routine at the address in the following instruction.
- `store_two_when_byte_not_greater_2_2`: Compares a word at the address in a2 with 29 and returns when they differ, then tests a byte at the address in a3 and returns when it is not positive, otherwise stores the word 2 at that address.
- `store_space_when_byte_not_negative`: Compares the 16-bit value stored at the address in a2 with the decimal constant 33 and, while they differ, compares the byte at the address in a3 with zero, storing the decimal constant 32 at the address in a2 when that byte is not negative.
- `store_printer_status_byte_3`: Compares the 16-bit value at the address in a2 with the decimal constant 33 and, when they differ, returns without changing anything.
- `compare_word_loop_increment_byte_2`: Compares the word at the address in a2 with the decimal constant 33 and, when they differ, branches back to repeat the check, otherwise it increments the byte at the address in a3 and then jumps to another routine.
- `compare_word_then_decrement_byte`: Compares the word at the address in a2 with the decimal constant 33 and, when they differ, branches to another routine.
- `check_word_then_copy_byte`: Compares the 16-bit value at the address in a2 with the decimal constant 57 and, if they differ, branches back to an earlier instruction; otherwise it loads the pointer stored at offset 28 of the structure in a6, checks whether the byte it points to is zero, and if so stores the byte at decimal address 67 into that location.
- `decrement_and_shift_printer_byte`: Decrements the byte at offset 28 of the structure in a6, then copies the following byte from the source buffer into that location while storing the previous value elsewhere, fills a buffer at a4 with eleven entries from a table, and finally calls a routine twice with computed arguments.
- `index_store_byte_table`: Loads a long from offset 28 of the structure in a6, increments it by one, uses it as an index into a table, reads the byte at that index, stores the source byte from a4 through the pointer in a0, and then branches into another routine.
- `copy_eleven_bytes_from_table`: Copies eleven bytes from a global table into consecutive memory starting at the address in a4, incrementing the source index until it reaches eleven, then returns that index.
- `compare_byte_offset_57`: Compares the 16-bit value at the address in a2 with the decimal constant 57, and if the two differ it jumps to the routine at offset 214, otherwise it jumps to the routine at offset 156.
- `copy_eleven_bytes_table`: Copies eleven bytes from a global table into consecutive memory starting at a4, incrementing the destination pointer each iteration, then calls a routine twice with different arguments and returns the final counter value.
- `write_byte_on_null_check_alt_2`: Checks if the word at the address in a2 is zero; if so, it writes the byte 253 to offset 2720 of a structure pointed to by d2.
- `convert_value_to_ascii_digits_2`: Stores 48 at the location pointed to by the sum of two registers when the first parameter is below 100, stores 49 there otherwise and subtracts 100 from it.
- `convert_digit_to_ascii`: Compares the value in d0 with 99 and, when it is greater, advances a pointer in d3, adds an offset from d4 to it, stores 49 into the pointed-to byte, subtracts 100 from d0, and branches back to a routine at offset 92.
- `convert_to_decimal`: Writes the character 0 at a destination, then writes the character 0 at the next location when a value is below 10 and its tens digit otherwise.
- `write_digit_pair`: Stores the decimal digit 48 at the address formed by adding the values in the two address registers, calls another routine, and writes the returned byte plus 48 to the following memory location.
- `call_two_subroutines_2`: Calls a routine at offset 100 in a global variable with two parameters, then calls another routine at offset 104 in the same global variable.
- `divide_32bit_values_2`: Performs a 32-bit division of a value in d1 by a divisor in d2, using a sixteen-iteration shift-and-subtract loop for large divisors and a remainder-and-quotient sequence for small ones, then returns the result in d0 and d1.
- `rotate_32bit_value_5`: Rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves, and subtracts a threshold from the high half when the rotated value meets or exceeds it.
- `multiply_32_bit_integers_5`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products and combines the results to form the final 32-bit multiplication result.
- `negate_result_on_flag_3`: Calls a helper routine to obtain a value, negates it when the flag left in an address register is nonzero, and returns the resulting value.
- `restore_registers_dispatch_2`: Restores the caller's d2, d3, a2, a3 and a4 registers from the stack, loads a byte from the stack into d0, and branches into a dispatch table indexed by that value.
- `restore_registers_and_return_6`: Clears the 16-bit result in d0 and restores the caller's d2, d3, a2, a3 and a4 registers from the stack, then returns.
- `call_library_vector_198_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `compare_value_branch_5`: Saves registers a2 and a3 on the stack, loads a 16-bit value from the stack into d0, sets a1 to 4, a3 to 47, and a2 to 46, then compares d0 with 768 and branches to a routine at offset 38 on carry, to one at offset 60 on above, otherwise to a routine at offset 170.
- `compare_value_range`: Compares the 16-bit value in d0 with 256 and branches forward when it is below, then branches forward when it is above, otherwise it falls through to the next routine.
- `compare_value_branch_3`: Tests the value in d0 against 512 and returns when it differs, otherwise branches to the shared code that stores the structure constants.
- `compare_value_branch_2`: Compares the value in d0 with the decimal 1024, returns if they differ, otherwise branches into another routine.
- `write_structure_values`: Writes the 32-bit values 1536 and 795 at offsets 26 and 30 of the structure pointed to by a1, stores the 16-bit value 75 at offsets 36 and 34, sets a global variable to 48, and writes the bytes 55 and 53 to two address registers.
- `write_structure_values_alt`: Writes the 32-bit values 800 and 1060 at offsets 26 and 30 of the structure pointed to by a0, stores the 16-bit value 100 at offsets 36 and 34, writes the byte 49 to a global variable, and branches into another routine.
- `write_structure_values_alt2`: Writes the 32-bit values 1200 and 1590 at offsets 26 and 30 of the structure pointed to by a0, stores the 16-bit value 150 at offsets 36 and 34, writes the bytes 49 and 53 to a global variable and an address register, then branches into another routine.
- `initialize_structure_values_2`: Writes several 16-bit and 32-bit values into a structure pointed to by a1, sets a global variable to 33, and writes 30 to two memory locations.

### 12

- `return_constant_55`: Returns the constant an address in d0 to the caller without any other effect.
- `index_and_store_byte`: Loads a 16-bit value from a table indexed by a byte in a1, adds the byte to the loaded value, and stores the 16-bit result through a pointer in a0.

### 15

- `return_constant_56`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_57`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_table_bytes_through_pointers`: Loads a byte from the table at a1+12 into a register, stores it through the pointer in a0, then loads a byte from a1+13 and stores it through the pointer in a0, and repeats this pattern for offsets 14 through 21 before returning.

### 12

- `return_constant_58`: Returns the constant an address in d0 to the caller without any other effect.
- `truncate_high_word`: Loads a 32-bit value from a global variable into a register, masks off its low 16 bits, shifts the value right by 16 bits, and stores the resulting 16-bit value back into the same global variable.

### 20

- `return_constant_59`: Returns the constant an address in d0 to the caller without any other effect.
- `mask_table_value`: Loads a 32-bit value from a table indexed by a byte in d7, masks it with a 32-bit constant, and stores the result into a global variable through a pointer in a0.

### 11

- `return_constant_60`: Returns the constant an address in d0 to the caller without any other effect.

### Balance of Power - The 1990 Edition (1989)(Mindscape)[a2].adf:CLI

- `parse_command_line_arguments`: Splits the command-line arguments into individual null-terminated tokens, converting quoted strings and escape sequences, then stores the resulting token pointers in a table and invokes the dispatch routine.
- `invoke_two_function_pointers_3`: It calls the function pointer at offset -288 from the library base register, then calls the function pointer at offset -372 from the same base register, and returns.
- `call_dos_vector_and_check`: It loads the address of a dos.library string, clears d0, and calls the library vector at offset -552 from the base register.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_input_3`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output_3`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_lock_2`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_4`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_dup_lock`: Library stub: loads its arguments and the dos library base, calls DupLock and returns its result to the caller.
- `call_current_dir_2`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_execute`: Library stub: loads its arguments and the dos library base, calls Execute and returns its result to the caller.
- `allocate_cli_control_block`: Allocates and clears a 64-byte control block, stores a shifted value at offset 172 of that block, writes constant words at offsets 24 and 4, stores the value 1000 at offset 52, then calls a routine with a table entry and returns.
- `allocate_bptr_entry_chain`: It walks a linked list of entries, allocating a cleared memory block for each one and recording a value taken from the entry into that block while linking the blocks together.
- `free_bptr_entry_chain`: It walks the chain built by the allocation routine, releasing each entry value through another routine and then freeing the entry block through the memory library, and returns when the chain is exhausted.
- `call_alloc_mem_3`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem_3`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_find_task_3`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.

### mathtrans.library

- `return_empty_routine`: This routine is empty, consisting of a single illegal instruction that performs no useful work and does not return normally.
- `store_a0_into_global`: It stores the 32-bit value passed in a0 into a global variable at offset 408 and returns.
- `handle_null_offset_branch`: When the eight-bit value at offset 8 of the structure in a6 is zero, transfers the base pointer into a linked list node and invokes a routine via a global function pointer, otherwise sets bit 3 of the byte at offset 14 and returns zero.
- `set_flag_and_return_zero`: It sets bit 3 of the byte at offset 14 in the structure pointed to by the base register, clears d0, and returns zero.
- `check_flag_and_increment`: It checks whether bit 3 of the byte at offset 14 in the base structure is clear; if so it increments a short counter at offset 32 and, when that counter becomes one, initializes a global variable and calls another routine, then returns the base register.
- `return_negative_one`: It loads the value negative one into d0 and returns it to the caller.
- `decrement_counter_and_call_2`: Library code that decrements the 16-bit counter at offset 32 of the structure at a6.
- `process_input_byte`: Clears the low bit of an input byte in d0, then returns immediately when that byte is zero, recurses when it is negative, and otherwise processes a positive input.
- `process_byte_input_loop`: Returns immediately when the input byte is zero, recurses when it is negative, then processes a positive input by clearing the low bit and running a doubling-accumulator loop that shifts a running difference until a sign condition is met.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `test_low_byte_then_call`: Copies the low byte of d0 into d7, tests that value, and calls the routine at offset 20 of the library base loaded in a5, then returns.
- `call_jump_table_entries`: Library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `load_address_and_jump`: Loads an address from a fixed location into a0 and jumps to the routine at that address.
- `dispatch_library_routine`: Loads a routine address from a global variable and calls it, then calls a routine at a fixed offset from the same global, and when the resulting flag is clear and a byte at offset 42 matches a byte at offset 42 of a structure in a6 it calls a routine at offset 20 of the library base before returning.
- `library_dispatch_routine`: Initializes three stack variables, calls a routine at offset 42 of the base in a5, and when a flag is clear and two bytes match calls a routine at offset 78 and recurses; then branches on the value in d6 to call routines at offsets 48, 60, 66 or 72 depending on the sign of d7.
- `mask_twice_library_result`: Branches on the value in d6, calling library functions at offsets 66, 48, 60 and 72 of the base in a5 depending on the flags in d4 and d7, then clears bits 2 and 3 of the returned value and returns it.
- `call_library_twice_store`: Stores d7 into d1, calls a library function through a5, and branches away when the carry flag is set.
- `call_library_and_store_d2`: Moves d2 into d0, calls the library function at offset 60 of the base in a5, stores the returned value back into d2, and returns.
- `load_constant_if_d4_nonzero`: Moves d2 into d0, tests d4 and branches away when it is nonzero, otherwise loads the constant an address into d2 and returns.
- `mask_return_value`: Library code that calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `library_division_loop`: Clears bit 7 of the value in d7, initializes a stack variable, and when the byte exceeds 64 or the value exceeds an address stores -1 and recurses; otherwise it runs a loop that repeatedly shifts a an address constant right and subtracts it while decrementing a counter until the value is exhausted.
- `setup_shift_constants`: Arithmetic-shifts d7 right, clears d6, loads the constant an address into d5, loads the address offset by 4 into a0, loads the value 24 into d1 and the value 1 into d2, then branches forward into another routine.
- `execute_loop_with_branching`: Executes a loop that performs arithmetic and bit-shifting on registers while decrementing a counter.
- `push_word_fffe_branch`: Pushes the word value 65534 onto the stack and then branches into another routine, so it only prepares a stack value before handing off.
- `push_word_ffff_branch`: Pushes the word value 65535 onto the stack and then branches into another routine, so it only prepares a stack value before handing off.
- `push_word_one_branch`: Pushes the word value 1 onto the stack and then branches into another routine, so it only prepares a stack value before handing off.
- `check_byte_and_adjust_stack`: Compares the byte in d7 against the constant 184 and branches past the cleanup when the value is greater, then removes two bytes from the stack and returns if d7 is nonzero, otherwise it returns.
- `clamp_and_accumulate_table_2`: Clamps a signed byte into a bounded range using a shift-based method, then walks a table of 8-bit entries accumulating two signed accumulators and stores the final result into a global variable.
- `clamp_and_accumulate_table`: Clamps a value into a range using a shift-based method, then walks a table of 8-bit entries accumulating two signed accumulators and combining them into a result, and finally branches to a callback registered in the library's global data.
- `accumulate_array_and_return_2_2`: Accumulates values from an array pointed to by a0 into registers while decrementing a counter, and when the counter reaches zero it calls a routine with the accumulated result.
- `accumulate_array_and_return_2`: Accumulates values from an array pointed to by a0 into registers while decrementing a counter, and when the counter reaches zero it calls a routine.
- `accumulate_array_and_return`: Accumulates values from an array pointed to by a0 into registers while decrementing a counter, and when the counter reaches zero it calls a routine and returns if the accumulator is zero, otherwise it calls a routine with the accumulator and returns.
- `load_constant_and_branch`: Loads the constant 255 into d7 and branches into another routine without executing any further instructions of its own.
- `conditional_recursive_wrapper`: Returns immediately when the byte in d7 is zero, otherwise copies d5 into d6, calls a routine, copies d6 into d7, tests d7, restores the saved registers, advances the stack pointer, and returns.
- `save_and_forward_registers`: Saves d5 on the stack, calls a routine at offset 60, stores d6 into a stack slot, restores d5 into d6, and branches to a routine at offset 50.
- `clamp_register_value`: Copies d6 into d4, then returns immediately when d6 is negative.
- `shift_d4_and_indirect_jump`: Arithmetically shifts d4 right by eight bits, increments it by one, and branches to a jump table when the result is zero, otherwise it jumps through the indirect pointer in the code.
- `call_routine_conditionally`: Sets a boolean flag to false, calls a routine, and when the flag remains false it calls two additional routines in sequence before returning.
- `call_routines_conditionally`: Sets a boolean flag to false, calls a routine, and when the flag remains false it calls three additional routines in sequence before returning.
- `rotate_d7_and_call`: Returns immediately when the byte in d7 is zero, otherwise it rotates that byte right by one bit into the carry flag, calls a routine, and when the carry flag is clear it calls a routine with the incremented byte twice and a third routine before returning.
- `load_and_rotate_d7`: Loads the constant an address into d7, rotates that byte right by one bit into the carry flag, and branches into another routine without executing any further instructions of its own.
- `load_constant_and_branch_2`: Loads the constant an address into d7 and branches into another routine without executing any further instructions of its own.
- `pop_word_and_load_zero`: Pops a word off the stack into d6, tests that byte and branches past the setup when it is positive, then loads zero into d7 and branches into another routine.
- `shift_clear_and_restore`: It clears register d7, shifts that byte right by one bit, sets the carry flag, restores registers d1 through d6 and a0 from the stack, and returns.
- `setup_and_check_register`: Loads the immediate value an address into register d7, sets up a stack frame, checks the byte value in d7, and then returns to the caller.
- `library_digit_conversion`: Returns immediately when the byte in d7 is zero; otherwise it computes a signed nibble value, and depending on its sign shifts and combines d7 into a stack word, recursing several times, then checks the high byte and returns or continues.
- `triple_recursive_call`: Calls another routine three times in sequence and then returns.
- `accumulate_table_entries`: Increments d2, moves d5 into d3 and d6 into d4, then arithmetic-shifts both by the value in d2.
- `subtract_and_accumulate_table`: Subtracts d4 from d5 and d3 from d6, then repeatedly adds the next entry from the table in a0 to d7 while decrementing d1, returning when the counter reaches zero.
- `compute_absolute_value_d7_2`: Tests the byte in d7 and returns immediately when it is zero, branches away when it is positive, otherwise clears bit 6 of d7, calls a routine at offset 22, sets bits 1 and 2 of the condition code register, and returns.
- `load_ff_into_d7`: Loads the byte value 255 into register d7 and then branches to the instruction following the first instruction of another routine.
- `compute_absolute_value_d7`: Computes the absolute value of a signed number held in d7 using a shift-and-count loop, calling a callback routine at several points, and returns the result.
- `complex_loop_with_branching`: Executes a loop involving bit-shifting and conditional jumps, performing arithmetic on a value and potentially calling other routines.
- `call_other_routine_thrice`: Calls another routine three times in succession and then returns.
- `normalize_d7_and_return`: Checks the sign of the byte in d7, clears its high bit when positive, sets bit 1 of the condition code register, and returns.
- `jump_callback_and_transform`: Loads an address from offset 78 of the library global data into register a0 and jumps to the routine at that address, then calls a routine, and when the byte in d7 is zero it returns otherwise it transforms the byte through a series of arithmetic operations and calls several routines before returning.
- `transform_register_d7`: Computes a value from d7 using a signed multiply, arithmetic shift, subtraction, swap and rotate, then returns the result in d7.
- `rotate_and_toggle_d7`: Swaps the halves of the value in d7, rotates it right, toggles its high bit, and doubles it, returning via a branch when the overflow flag is set.
- `encode_byte_value`: Branches away when a carry flag is clear, then compares d7 against 124 and 126.
- `clear_d7_low_bit`: Clears bit 1 of register d7, then returns zero if the result is zero and clears d7 otherwise.
- `shift_d7_and_check_zero`: Compares d7 against 254 and branches past the shifts when they differ, then logically shifts d7 right by eight bits and again by one bit, returning when the result is zero and otherwise branching to clear the flag.
- `shift_and_rotate_d7`: Shifts the value in d7 left by eight bits, loads 255 into d7, rotates it right by one bit into the carry flag, and sets the overflow condition flag.
- `load_zero_into_d7`: Loads zero into register d7 and then returns.
- `invoke_callback_and_return`: Library code that receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `jump_to_dos_trampoline`: Loads the address of a global variable into a0 and branches to it, acting as a trampoline that jumps to the routine stored in that global variable.
- `dispatch_from_vector_table`: Loads an address from a fixed location into a pointer and jumps to the routine at that address, acting as a dispatch entry for the library.
- `dispatch_routine_via_global`: Loads the address of a routine from a global variable and jumps to it, returning whatever value is in register D0.
- `load_global_and_branch_3`: Loads the address of a global variable into register a0 and branches into another routine without executing any further instructions of its own.
- `load_global_branch`: Loads the address of a global variable into a0 and then branches into another routine, so it only sets up a pointer before handing off.
- `library_dispatch_stub`: Loads a pointer to a library entry point into an address register and branches to it, serving as a library dispatch stub that immediately hands control to the target routine.
- `call_dos_library_2`: Loads the address of a DOS library base into a0, then calls the library entry that saves two registers on the stack, loads a pointer from the stack into a2 and stores it into a global variable at offset 306, and finally pushes a zero.
- `store_register_via_pointer`: Saves several registers on the stack, moves d0 into d7 and d1 into a stack slot, clears d1, and invokes the function pointer loaded from a global variable.
- `process_icon_sublists_recursively_2_2`: Initializes a counter to zero, calls a routine to fetch a pointer, then walks a null-terminated list of pointers at offset 54 of the structure pointed to by the second argument.
- `load_zero_and_return`: Loads the constant value zero into register d0 and returns immediately.
- `copy_icon_data_to_stack`: Copies a fixed number of bytes from a structure pointed to by the third argument into a local stack buffer, then copies several fields from the same structure into local stack slots and calls another routine with the first two arguments and a pointer to a two-word stack header.
- `load_offset26_and_branch`: Loads the address of a global variable at offset 26 into register a0 and branches into another routine without executing any further instructions of its own.
- `empty_routine_returns`: It is an empty routine that returns immediately without modifying any data.
- `forward_d0_value`: Loads the address of a global variable into a0 and branches to a routine at offset 0, which forwards the incoming d0 value as its return value.
- `call_indirect_return_d0_2`: Library code that receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0.
- `return_immediately_2`: It is an empty routine that returns immediately.
- `install_string_handler`: Loads the address of a string literal into a0 and jumps to the library entry point, which stores that string pointer into a global variable at offset 306 and then calls copy_string to duplicate it.
- `jump_to_indirect_caller`: Loads the address of a global variable into a0 and branches to a shared routine that calls the indirect routine at that address.
- `call_pointer_from_global`: Reads a 32-bit pointer from a global variable, subtracts 36 from it to form an address, calls the routine stored there, and returns the value that was in register d0.
- `jump_to_indirect_invoker`: Loads the address of a global variable into a0 and branches to a shared routine that invokes the indirect routine at that address.
- `load_global_and_branch`: Loads the address of a global variable located 48 bytes before the pointer in a0 into a0, then branches to another routine.
- `call_indirect_via_a0`: Adds a negative offset to the pointer in a0, calls the routine found there, then returns the value left in d0.
- `call_routine_via_d0_offset`: Reads a 32-bit value from a global variable, subtracts 60 from it to compute an address, calls the routine stored at that address, then returns the value that was in register D0 before the call.
- `load_pointer_and_jump`: Loads the address stored at the global variable at offset -50 into register a0 and jumps to the routine located at offset 372.
- `load_global_and_branch_2`: Loads the address of a global variable into a0 and branches into another routine without executing any further instructions of its own.
- `load_and_jump_callback`: Loads the address stored at offset -78 of the library's global data into a0 and jumps to the routine at that address.
- `invoke_callback_and_return_concat`: Library code that invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: Library code that calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: Treats the 32-bit value in d6 as a signed fixed-point number, recording its sign, then swaps its high and low words when the magnitude fits, and finally left-shifts the value while decrementing a counter, returning early if the value overflows.

### icon.library

- `return_constant_61`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `link_structure_if_offset_zero`: If the word at offset 8 of the structure in a6 is zero, it reads a pointer from offset 42 and calls the routine at the address stored at offset 34, then links the value from offset 0 into a list by storing it at that pointer and writing the pointer back at offset 4 of the pointed-to location.
- `set_flag_in_structure`: Sets bit 3 of the byte at offset 14 in the structure pointed to by a6 and returns zero.
- `return_constant_62`: Returns the constant an address in d0 to the caller without any other effect.
- `call_function_and_store_result`: Stores a pointer in a global variable and calls the function pointer at offset 34 of the structure in a6, saving its result at offset 38.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `empty_routine_placeholder`: This routine is empty, consisting of a single zero word that other code refers to by address.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `manage_icon_chain_3`: Pushes registers a0 and a6 onto the stack, calls the library dispatch vector at offset 698, restores the stack, and returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `return_allocated_buffer`: Saves registers a0 and a6 on the stack, calls a routine at address 1922, restores the saved registers, and returns the buffer pointer in a0.
- `manage_icon_chain_offset_832`: Pushes registers a0 and a6 onto the stack, calls the library dispatch vector at offset 832, restores the stack, and returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_two_regs_then_return`: Saves registers a0 and a1 on the stack, calls another routine, restores the two registers, and returns.
- `call_and_return_registers`: Saves registers a0 and a1 on the stack, calls another routine, restores the registers, and returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `store_zero_byte`: Contains a single instruction that writes a zero byte to the memory location pointed to by d0, then returns.
- `init_and_link_structure`: Copies 44 bytes from a stack buffer into a structure, stores several stack values into fixed fields of that structure, and returns the structure pointer unless a field is zero, in which case it links the structure into a list and returns it.
- `copy_structures_and_link`: Copies thirty-one bytes from one structure to another using a loop, then copies eight four-byte values between the structures and, if the copied value at offset 76 is non-zero, stores the destination pointer at offset 424 of that value.
- `restore_frame_return_zero`: Restores the frame pointer, loads the registers left on the stack, and returns zero in the result register.
- `jump_callback_and_transform_2`: Loads an address from offset 78 of the library global data into a0 and jumps to the routine at that address, then calls a routine.
- `update_icon_structure_fields`: Reads a 32-bit value from offset 1010 of the icon structure in a3, then updates the icon's name, type, and associated data pointers by calling the icon.library UpdateIcon function and processing the returned icon structure, storing results at offsets 11, 13, 27, 29, 35, and 43.
- `walk_and_store_results_2`: Checks a structure in a2 for specific marker values, then repeatedly calls a routine and stores its results at offsets 18 and 22 of a structure in a3, returning -1 when a call fails or the fourth argument is zero.
- `walk_and_store_results`: Calls a routine, stores its result into a field at offset 18 of a structure in a3, calls the routine again storing the result at offset 22, and when several zero-checked fields are set it calls two routine pointers in an alternating pattern, storing results and looping while a counter decrements.
- `loop_call_library_vectors`: Repeatedly calls two library vectors, storing each first result into a pointed-to location and decrementing a counter, until the counter reaches four or a vector returns zero, then it checks a status byte at offset 34 and calls a second routine before returning.
- `load_zero_and_branch_2`: Loads the value zero into register d6 and branches to an earlier routine, containing no loop and making no call.
- `restore_registers_and_return_8`: Restores the caller's registers by reloading d2 through d6 and a2 through a5 from the stack, then returns.
- `process_icon_sublists_recursively_2`: Initializes a counter to zero, calls a routine to fetch a pointer, then walks a null-terminated list of pointers at offset 54 of the structure in a6, copying each entry's 32-bit value from offset 26 into a stack slot and processing it.
- `process_icon_sublists_recursively`: Copies a 32-bit value from offset 26 of the structure in a6 into a stack slot, then walks a null-terminated list of pointers at offset 54, calling a routine for each entry and accumulating a 32-bit counter.
- `iterate_pointer_lists_accumulate`: Iterates over several null-terminated pointer lists stored at fixed offsets, calling a routine for each entry and accumulating a counter before returning.
- `process_list_recursively`: Advances a pointer and increments a counter until a zero entry is found, then calls a function and, if it returns non-zero, iterates a list calling the function and recursing on entries, recursing on a pointer at offset 70, and finally storing a counter value before returning.
- `process_icon_list_recursively`: Walks a null-terminated list of pointers, calling a function for each entry and incrementing a local counter, then processes a secondary pointer field and stores a stack value before returning.
- `traverse_linked_list`: Walks a linked list by repeatedly advancing a pointer, calling a routine with the incremented value and then with each returned node until a null pointer stops the loop.
- `restore_regs_return_void`: Loads the registers left on the stack and returns immediately without setting a result.
- `call_with_offset_pushed`: Pushes a pointer plus a fixed offset onto the stack and calls the library dispatch vector, then restores the caller's registers and returns.
- `call_pointer_then_verify`: Calls a function pointer stored in a global variable with two arguments and, when that call returns nonzero, calls the same pointer again with the result and a second pointer, returning the original value only if the second call also succeeds.
- `call_vector_then_return`: Pushes two constants onto the stack, saves a2, and calls a routine through a vector, then clears d0 and returns when the result is zero, otherwise restoring registers and returning.
- `restore_registers_and_return_9`: Clears register d0 and restores registers d2, d3, d4, and a2 from the stack, then returns.
- `decrement_counter_store_pair`: Decrements a 16-bit counter through a pointer and returns zero when it underflows, otherwise stores two 32-bit values into adjacent fields of a table indexed by that counter.
- `decrement_counter_and_store_pair`: Decrements a 16-bit counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from registers and the stack into consecutive slots of that table, then returns one.
- `allocate_and_dispatch_buffer`: Allocates a buffer, then runs two branches that either clear the result or call a callback with the buffer, and finally returns the buffer pointer or zero depending on which branch executed.
- `move_d6_to_d0`: Moves the contents of data register d6 into data register d0, then branches into another routine without returning.
- `call_lib_func_and_return_zero`: Pushes a constant from a global variable, calls the function at library offset -210, then restores the stack registers and returns zero.
- `call_with_increased_argument`: Adds a fixed constant to a second argument, pushes both arguments onto the stack, and calls the library dispatch vector before restoring the caller's registers and returning.
- `scan_string_until_equals`: Walks a null-terminated string pointed to by a register, calling a helper for each non-zero element and continuing while the helper returns non-zero and the current character is not an equals sign, then returns a pointer one past the matched character.
- `restore_registers_return_zero`: It clears the return value register and restores the caller's registers from the stack before returning.
- `walk_list_until_match`: Loads a value from a global variable, then repeatedly calls a routine with a pointer and a counter, adjusting the pointer until a matching condition is met, and finally returns one.
- `compare_values_with_counter`: Tests a value from a global variable against a second value and returns one when they match and the second is zero, otherwise adjusts a counter register and repeats until it reaches zero, then returns the other value.
- `increment_d4_and_branch`: Tests register d4, branches away when it is zero, otherwise increments d4 by one, moves the result into d0, and branches into another routine without returning.
- `load_zero_into_d2`: Loads the constant zero into data register d2, then branches into another routine without returning.
- `restore_and_return`: Loads registers d2 through d6 from the stack and returns without using them.
- `parse_signed_decimal_string`: Writes a zero byte at offset 30 of the structure in param_1, then reads a signed decimal number from a string and stores it into a global variable, returning the source pointer.
- `push_constants_and_call_pointer`: Pushes two constants onto the stack, saves a register, and calls a function pointer from a global variable, then loads a value into a register and branches away when the result is nonzero.
- `load_zero_and_branch_6`: Loads zero into d5 and falls through into another routine without returning.
- `accumulate_digits_from_string`: Accumulates successive decimal digits from a string into a running total by multiplying by ten and adding each digit, then converts the accumulated value into its decimal digit bytes and stores them through a pointer.
- `clamp_byte_to_digits`: Reads a long from the stack, clamps it into the decimal range 48 to 57 by subtracting 48, and places the resulting value into data register d0 before branching away.
- `set_return_value_negative_one`: It loads the value negative one into the return value register and returns.
- `call_library_vector_198_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_10`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `library_conditional_call_chain`: Library code that calls another routine with two arguments and branches on the result.
- `handle_library_result`: Calls a library routine with two parameters and, when the result is nonzero, stores the value 10 into a two-byte field and a destination pointer.
- `restore_registers_and_return_7`: Loads zero into register d0, restores registers d2 through d3 and a2 from the stack, and returns.
- `compare_result_and_return_zero`: Calls another routine with three arguments and returns its result, but replaces it with zero if the result differs from the fourth argument.
- `copy_d1_to_d0_and_return`: Copies the value from register d1 into d0, restores registers d2 and d3 from the stack, and returns.
- `invoke_icon_callback`: Loads the icon.library base from a global variable and invokes its function at offset 48, then copies the result from register d1 into register d0 before restoring registers d2 and d3 from the stack and returning.
- `move_d1_to_d0`: Moves the value in register d1 into register d0, then restores registers d2 and d3 from the stack pointer, and returns.

The function list continues in: g1446-recovery-functions-2.md.
