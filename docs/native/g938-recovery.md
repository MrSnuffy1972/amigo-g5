# Game 938 (black crypt (1992)(electronic arts)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00938). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2713424 top-level bytes in 50 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 514 of 514 named (0 author symbols), 514 of 514 described (32 by rule, 480 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1094 of 1094 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 144169 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16534 of 16549 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2025689 |
| code | 52896 |
| data | 443774 |
| padding | 12377 |
| unused | 178688 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Stack | 2 | 117 | 2 |
| avail | 11 | 376 | 12 |
| runback | 3 | 709 | 39 |
| endcli | 2 | 117 | 2 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:InstallCrypt | 3 | 3235 | 57 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:BlackCrypt | 3 | 1745 | 48 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:bcdfp | 3 | 6150 | 184 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:bcdfq | 2 | 983 | 56 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:bcdft | 7 | 29 | 2 |
| Black Crypt (1992)(Electronic Arts)(Disk 2 of 3).adf:bcdfu | 2 | 1490 | 84 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 187 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 582 | 17 |
| Black Crypt (1992)(Electronic Arts)(Disk 2 of 3).adf:bcdfx (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 217 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- black crypt (1992)(electronic arts)(disk 2 of 3)/Black Crypt (1992)(Electronic Arts)(Disk 2 of 3).adf:bcdfx: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- black crypt (1992)(electronic arts)(disk 2 of 3)/Black Crypt (1992)(Electronic Arts)(Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- black crypt (1992)(electronic arts)(disk 3 of 3)/Black Crypt (1992)(Electronic Arts)(Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- black crypt (1992)(electronic arts)(disk 1 of 3)/Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:bcdft: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 144169 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: Copies a fixed number of words from a source pointer to a destination pointer on the stack.

### Stack

- `analyze_and_reconstruct_stack_frame`: Iterates through a linked list to find the maximum value in a field, then writes a header and a sequence of incrementing values into a buffer.
- `copy_stack_data_2`: Copies a fixed number of words from a source pointer to a destination pointer on thestack.

### avail

- `parse_command_arguments`: Parses a command-line string or argument list by scanning for delimiters and handling quoted strings or escaped characters.
- `process_message_reply`: Updates a message structure, calls various library functions based on certain flags, and returns a pointer from the message structure.
- `sum_memory_region_sizes`: Iterates through a linked list of memory regions and sums the size of any region whose attribute flags match a provided mask.
- `print_memory_availability_table`: Checks a command string or argument to determine if the output should be filtered by chip, fast, or total memory types, then calculates and prints the available, in-use, and maximum block sizes for the selected memory type or a combination of types.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### runback

- `initialize_program_entry`: A single instruction routine that calls a routine to clear a block of memory and store a stack address and a global variable.
- `run_background_task`: Parses a command line for an optional load delay and a program name with parameters, then starts the named program as a background task, handling errors such as missing command, missing file, directory instead of file, out of memory and usage errors, and prints status messages.
- `clear_table_and_recurse`: Clears a block of thirty-nine words starting at an offset from a pointer-derived address, stores a stack address and a global variable into subsequent fields, and calls another routine.
- `call_optional_handler_and_recurse`: Checks if a global variable is non-zero to call a function pointer, then calls itself recursively.
- `parse_command_line_string`: Parses a string to extract individual words and updates global state fields.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `run_back_loop_handler`: Performs a short loop calling another routine ten times and then inspects several global variables.
- `append_string_to_buffer_2`: Finds the end of a destination string and copies bytes from a source string until a null terminator or a certain count is reached.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runsout, then null-terminates the destination and returns it.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `get_table_entry_and_execute`: Checks if an index is within a valid range and if a table entry is non-zero, then calls the function pointer at that entry and clears the entry.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_indirect_function`: Loads a function pointer from an offset relative to a global library base and executes it.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `execute_indirect_function_2`: loads a function pointer from a nested table or structure and executes it.
- `invoke_library_function`: loads a function pointer from a table or structure and executes it via a library stub.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `invoke_library_function_2`: loads a function pointer from a library-relative offset and executes it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_indirect_function_2`: Performs an indirect call to a function pointer located at an offset from a pointer retrieved from a global variable.
- `call_indirect_function_3`: Loads a function pointer from a table using an offset and executes it.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: Loads a function pointer from a table located via an offset from a global variable and executes it.
- `call_indirect_function_4`: Executes a function pointer retrieved via a pointer-to-pointer lookup from a global variable.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_indirect_function_5`: Performs an indirect branch to a function address calculated from a global pointer and a constant offset.
- `call_library_function`: Loads a library base from a global variable and calls a function at a specific offset within that library.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### endcli

- `find_max_and_prepare_buffer_2`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or table with a header and a sequence of incrementing values.
- `copy_stack_data_3`: Copies a fixed number of words from a memory location to the stack before calling another routine.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:InstallCrypt

- `initialize_program_state`: Clears a range of memory, sets a pointer to a value, and calls a routine or checks a flag before calling another routine or a library function.
- `wait_for_installer_window_event`: Locks the current directory and waits for a window message port event, where certain gadget or close-window messages terminate the loop and release the lock.
- `open_install_window_and_buffers`: Clears several state fields, then opens an installer window and calls a routine with a specific value, storing the result in a state field and calling a failure routine if a required result is zero.
- `draw_install_path_prompt`: Draws the installation dialog through a long fixed sequence of graphics calls for moving the pen, setting colour and drawing text.
- `show_install_failure_and_close`: If an error code exceeds a certain threshold, it clears a portion of the screen before drawing an error message, then closes a window and cleans up a linked list.
- `copy_string_with_colon`: Copies a string from a table using a length byte, resetting the length to zero if it exceeds a certain limit, and appends a colon and a null terminator to the destination.
- `check_entries_under_forbid`: Forbids multitasking, walks a linked list of entries and calls two routines for each entry with a non or non-zero field, setting the return value to 1 when a comparison fails.
- `prompt_insert_disk_and_wait`: Checks a path and, if it is not available, draws a prompt asking the user to insert the named disk and waits for a close or gadget event.
- `copy_file_with_progress_message`: Copies a file from a source to a destination by opening both, reading and writing blocks in a loop, and closing both.
- `install_game_files_to_destination`: Prepares the destination directory, creating it if missing, and iterates through a table of files to copy from a source disk.
- `invoke_base_offset_pointer`: Calls another routine with a pointer into a fixed offset of the data base, the argument provided, and the address of its own stack argument, then returns the result.
- `copy_byte_array`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `compare_byte_strings_ordered`: Compares two byte strings and returns a status code indicating if one is greater, smaller, or if they are identical.
- `return_empty_subroutine`: the routine performs no operations and immediately returns.
- `init_control_block_with_streams`: Allocates a control block, initializes certain fields, and writes a tag into a message area using current input and output streams.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `parse_string_tokens_4`: Parses a source string into individual tokens, handling quoted strings and skipping whitespace, then stores the resulting pointers to these tokens into a destination array.
- `copy_string_to_destination`: Finds the end of a destination string and copies a source string into it, stopping if a counter reaches zero or a null terminator is encountered.
- `copy_string_with_limit_2`: The routine finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `parse_string_and_store_pointer`: Uses library calls to find a string, copies a string to a destination, and stores a pointer or value into a global field.
- `copy_string_to_end_2`: Finds the end of a destination string and copies a source string until a null terminator is reached.
- `copy_string_to_null_2`: Locates the null terminator of the first string parameter by incrementing a pointer until a zero byte is found, then copies the second string to that location.
- `copy_string_or_pad_2`: The routine copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `multiply_32_bit_values`: The routine takes two 32-bit values from registers and returns a 64-bit product formed from the high and low 16-bit halves of each operand.
- `parse_string_for_control_sequences`: Parses a source string to detect specific control characters, such as percent signs, asterisks, and hyphens, while updating a destination buffer and a global integer or pointer.
- `calculate_string_length_2`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `iterate_table_entries`: Iterates through a table of 22-byte entries in the data area and calls another routine once for each entry until the end of the table is reached.
- `update_stream_buffer_state`: Updates a stream buffer's current position, length, and status flags while optionally writing a byte or seeking to a new offset.
- `update_string_buffer`: Converts a numeric value into a string of digit characters and clears a linked list entry.
- `update_string_buffer_with_digits`: Converts a value into a string of digits and stores it in a buffer, then clears a linked list entry.
- `return_zero_3`: The routine returns a zero value to another routine.
- `tiny_addq_addq`: Small routine of 2 instructions that adds 2 to a5, then adds 8 to d0.
- `search_table_for_flag_match`: Iterates through a table of structures, checking if a specific bitmask is absent and if a certain field is non-zero, while recursively calling another routine to validate a condition.
- `iterate_and_test_table`: Iterates through a table, checking certain bit flags and calling another routine until a condition or end-of-table is met.
- `return_to_caller`: This routine is an empty function that immediately returns to its caller.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `apply_signed_division_sign`: Performs a signed 32-bit division and negates the result if the dividend or or the quotient's sign or certain registers indicate a negative value.
- `concatenate_division_remainder_with_input`: Calls a 32-bit division routine and returns a 64-bit value formed by concatenating the original contents of a register with the remainder or quotient left in that register by the called routine.
- `recursive_call_2`: The routine calls another routine, which is the same as the caller, so the routine recurses without performing any other operation.
- `perform_32bit_division_2`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `init_stream_buffer`: Initialises a stream record's buffer pointers and flags.
- `insert_into_linked_list`: Traverses a linked list to find a specific node and inserts a new node or updates a pointer to include a new element.
- `traverse_and_free_list`: Iterates through a linked list by following pointers, calling a routine to deallocate or clear each node, and finally nullifies the list head pointer.
- `free_all_list_nodes`: Frees every node of a linked list through a library call, then clears the list head and returns zero.
- `prepend_to_linked_list`: Retrieves a pointer from a library call and inserts a new node containing a pointer and a value at the head of a linked list.
- `copy_string_simple_2`: The routine copies characters from a source string to a destination buffer until the null terminator is copied.
- `check_input_interactivity`: Checks if a provided index is within a valid range and if a corresponding entry exists, then returns a boolean result based on whether the system is interactive.
- `validate_and_read_file_offset`: Checks if a provided offset is within the bounds of a table and if a certain flag is cleared, then attempts to read or seek within a file, updating a global status or error flag accordingly.
- `set_global_on_file_error`: Checks a return value from a library call and, if zero, calls another routine to store a value in a global variable before returning a failure flag.
- `return_zero_4`: The routine is empty and always returns a zero value to another routine.
- `validate_and_write_buffer_element`: Checks if an index is within the bounds of a table and verifies a bitmask before performing a conditional write or seek operation.
- `check_output_flag`: Checks a bit in a return value from a library function and conditionally calls another routine if a global variable is non-zero.
- `write_four_bytes_to_output`: Writes four bytes from a global buffer to the standard output through the operating system, then flshes the output and calls another routine with the value one.
- `drain_list_then_run_cleanup`: Repeatedly removes the head of a linked list and calls the cleanup routine stored in each node until the list is empty.
- `shutdown_installer_resources`: Closes each stream entry, releases a directory lock, and frees the window, screen and library resources held in global fields, restoring a saved value along a way.
- `return_immediately_3`: The routine is empty and performs no operations before returning.
- `close_and_clear_entry`: Checks if an index is valid, calls a library function to close a resource if a certain flag is not set, and then clears a value in a table.

### Port-Handler

- `process_amigadot_hunks`: Parses a list of executable hunk headers to perform relocation or code loading.

### Disk-Validator

- `format_string_and_initialize_buffer`: Converts a numeric value to a string and stores it in a buffer, then initializes a memory range with a specific bitmask pattern.

### Ram-Handler

- `execute_indirect_call`: Performs an indirect call to a function pointer located at an offset from a global address.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:BlackCrypt

- `initialize_global_memory`: Clears a block of memory by storing zero in a range of addresses and updates a global variable with a new value.
- `handle_workbench_disk_access`: Closes the Workbench interface, locks or unlocks a disk volume, and executes a loop that processes input events and updates a global state or dispatches certain event types.
- `initialize_graphics_and_config`: Opens the graphics and intuition libraries, loads the configuration file, and initializes bitmaps and screen structures for the main game window.
- `execute_shutdown_callbacks_3`: The routine checks multiple global flags and executes corresponding function pointers from library bases.
- `delay_and_unload_segment`: Sets a value in a global variable and calls several indirect function pointers.
- `poll_input_event_loop`: Loops calling an input dispatch routine until a specific condition is met, handling cases for event types 1 through 4 by updating global state or invoking other handlers.
- `convert_string_case`: Copies a byte array and converts lowercase characters to uppercase by subtracting a constant.
- `traverse_linked_list_and_compare`: Iterates through a linked list of pointers, calling a routine to convert string case and another to compare strings, returning a status code.
- `handle_string_and_disk_status`: Checks a flag, calculates a string length, and modifies a buffer with a null or colon character while iterating through a list of pointers and calling library functions.
- `wait_for_library_call_2`: Loops calling a function pointer from a library base until a second function pointer returns a non-zero value.
- `update_global_state`: Updates several global memory locations with return values from function calls and modifies a global pointer or value before calling another routine.
- `execute_library_shutdown_callbacks`: Invokes function pointers from library bases and global structures to perform shutdown sequences, checking flags and iterating through callback lists.
- `handle_segment_load_unload`: Calculates a size based on a returned value, stores it in a global variable, and calls a routine with the calculated size and a pointer.
- `iterate_string_and_copy_tiles`: Iterates through a null-terminated string and calls a routine to copy tile data for each character.
- `return_immediately_4`: the routine performs no operations and immediately returns to its caller.
- `calculate_length_and_copy_tiles`: Calculates the length of a null-terminated string and iterates through it, calling a routine to copy tile data for each character.
- `process_global_array_elements`: The routine calls itself twice before returning.
- `execute_indirect_function_4`: Calls a function pointer retrieved from a table or a global variable via an offset.
- `process_string_with_tiles`: Iterates through null-terminated strings like 'CONFIGURATION MENU' and 'MAIN MENU', copying tile data for each character.
- `check_input_event`: Polls an input event queue, checking message types and data fields to determine a return value based on specific conditions.
- `dispatch_input_event_handler`: Polls an input event queue and dispatches execution to different sub-routines based on the event type code.
- `store_register_values`: Stores the contents of data registers into specific global memory locations.
- `copy_tile_data`: Copies a block of 7 or 8 bytes from a source pointer to multiple destination addresses with a fixed offset, iterating five times.
- `scan_global_variable_array`: Iterates through a global variable array in two passes, stepping by two bytes, and calls a routine to copy tile data.
- `decrement_nibble_loop`: Executes a loop sixteen times, calling indirect function pointers and a routine to decrement nibble values in a list of words.
- `decrement_nibble_values_4`: Iterates through a list of words, decrementing any non-zero nibble (the high, middle, or low nibble) by one.
- `increment_nibble_loop`: Executes a loop sixteen times, calling indirect function pointers and a routine to increment nibble values in lists of words.
- `update_nibble_mismatches`: Iterates through two lists of words, incrementing the high-order nibble, middle nibble, or low-order nibble of the first list's words if they do not match the corresponding nibble in the second list's words.
- `copy_bytes_decrementing_count`: Copies a certain number of bytes from a source pointer to a destination pointer while decrementing a counter.
- `compare_string_bytes`: Compares two byte strings character by character, returning a status code based on the first difference or equality.
- `return_empty_routine`: this routine performs no operations and immediately returns.
- `initialize_disk_access_and_parse_path`: Retrieves the current directory and, if successful, initializes a string buffer with a specific header before checking for existing directory or file information.
- `parse_string_tokens_2`: Parses a string or quoted string, skipping whitespace and extracting tokens into a destination buffer.
- `parse_string_into_tokens`: Parses a source string into individual tokens, handling quoted strings and skipping whitespace, and stores the resulting pointers to these tokens in a destination array.
- `copy_string_with_limit_3`: Finds the end of a destination string and copies characters from a source string until a counter reaches zero or a null terminator is encountered.
- `copy_string_with_limit_4`: Finds the null terminator of the first string, then copies characters from the second string into that location.
- `get_window_info_2`: Retrieves a pointer from a library and stores it in a global variable, then copies a value from a table or pointer offset into a structure member.
- `append_string_to_buffer`: Finds the end of a destination string and copies characters from a source string until a null terminator is copied.
- `copy_string_or_pad_3`: Copies a source string to a destination buffer up to a certain count, or if the count is zero, fills the destination with null bytes.
- `set_pointer_chain`: Initializes a structure or memory block with a pointer, a byte, and a constant, then recursively calls itself if a certain pointer is null.
- `initialize_library_context`: Checks a field in a structure, calls a library function if non-zero, and then sets certain structure fields to fixed values before calling two other library functions.
- `calculate_string_length_3`: Calculates the number of bytes in a null-terminated string by finding the difference between the end of the string and its start.
- `setup_pointer_chain_2`: Stores an incremented pointer into a memory location, clears a subsequent word, and stores the original pointer value in a following word.
- `copy_string_simple_3`: Copies characters from a source string to a destination buffer until a null terminator is copied.
- `execute_shutdown_callbacks_2`: Iterates through a list of indices to invoke a helper routine, then executes a series of conditional function pointers stored in global memory.
- `execute_shutdown_callbacks`: Iterates through a list of indices to invoke a helper routine, then executes a series of conditional function pointers stored in global memory.
- `return_empty_routine_2`: The routine performs no operations and immediately returns or exits.
- `validate_and_clear_handle`: Checks if a handle or index is valid within a range and if its associated pointer is non-zero, then clears the pointer and calls a library function if valid.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:bcdfp

- `initialize_buffer_and_dispatch_logic`: Clears a block of memory by zeroing out a range of bytes and then checks a bitmask in a global structure to conditionally call a function pointer.
- `open_graphics_intuition_libraries`: Opens graphics.library and intuition.library, checks for failure using referenced text strings, and calls routines to wait for library calls and execute cleanup callbacks.
- `initialize_and_process_data_block`: Calls a routine to initialize a data structure, then retrieves a pointer from a global variable to pass to a second routine, followed by two calls to a third routine using that same pointer.
- `process_linked_list`: Traverses a linked list of structures using an index, calling a routine to process data and updating a pointer, while checking for specific byte values.
- `initialize_entity_structures`: Iterates four times to initialize distinct data structures, setting specific byte flags, clearing arrays, and copying configuration values from a global table.
- `initialize_entity_arrays`: Iterates four times to populate multiple global arrays with calculated values and increments specific byte fields.
- `serialize_character_data`: Serializes character configuration blocks, including linked list entries and attribute tables, by writing header data, sprite pointers, and conditional status flags from memory buffers into a file stream.
- `wait_for_library_call`: Calls a function pointer from a global structure repeatedly until a second function pointer from the same structure returns a non-zero value.
- `initialize_library_call_state_zero`: Sets a global state variable to zero and invokes a function pointer from a global structure, then waits for a library call and executes cleanup callbacks if the state variable does not equal six.
- `initialize_library_call_state_two`: Sets a global state variable to two and invokes a function unable to be named, then waits for a library call and executes cleanup callbacks if the state variable does not equal six.
- `initialize_library_call_state_eight`: Sets a global state variable to eight and invokes a function pointer from a global structure, then waits for a library call and executes cleanup callbacks if the state variable does not equal six.
- `initialize_library_call_state_three`: Sets a global state variable to three and stores a parameter in a global variable, then invokes a function pointer, waits for a library call, and executes cleanup callbacks if the state variable does not equal six, followed by additional structure-based function calls.
- `initialize_library_call_state_seven`: Sets a global state variable to seven and stores a parameter in a global variable, then invokes a function pointer, waits for a library call, and executes cleanup callbacks if the state variable does not equal six, finally returning the stored parameter value.
- `initialize_library_call_state_ten`: Sets a global state variable to ten and stores two parameters in global variables, then invokes a function pointer, waits for a library call, and executes cleanup callbacks if the state variable does not equal six.
- `initialize_file_system_state`: Checks a return value, updates a global pointer, and performs a series of library calls and structure initializations involving file or disk-related strings.
- `manage_disk_access_and_saves`: Uses DOS library calls to lock and unlock the game disk, checks disk status, and manages file or disk-related structures.
- `evaluate_render_state_and_dispatch`: Checks a global status or type flag and, depending on its value, evaluates a secondary length or type value against certain thresholds to decide whether to execute further logic or branch to specific rendering or queue management routines.
- `setup_sprite_rendering_loop`: Iterates through loops to calculate sprite offsets, configure rendering parameters, and invoke sprite setup and rendering routines for multiple entities.
- `loop_calculate_sprite_offsets`: Iterates nine times, calling a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `call_sprite_offset_calculator`: Calls a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `calculate_sprite_offset_value_2`: Computes a memory offset based on a register value and a constant, stores the result in a global structure, and calls helper routines for sprite processing.
- `configure_sprite_rendering_params`: Sets specific word-sized values in registers, calls routines to calculate sprite offsets and setup rendering, and invokes text rendering and sprite configuration helpers.
- `dispatch_sprite_rendering`: Calculates sprite offsets and checks range overlaps in a global table, then dispatches to specific rendering routines based on a switch value derived from register calculations.
- `check_range_overlap`: Iterates through ten entries in a global table, checking if a passed parameter falls within specific ranges defined by the table entries.
- `check_and_update_sprite_condition`: Checks values in a global table indexed by parameters, increments or decrements specific fields if conditions are met, and calls helper routines for further processing.
- `check_sprite_condition`: Reads a value from a global table indexed by a parameter, divides it by ten, and branches to different helper routines based on whether the result is zero.
- `render_text_string_2`: Calculates string length and renders text by copying character data from global tables into a buffer, clearing the buffer beforehand.
- `clear_buffer_with_calculated_offset_3`: Calculates a memory offset and writes a zero to a certain location if certain bit-flags in a returned value are not set.
- `clear_buffer_with_calculated_offset_2`: Reads a byte from a global pointer and, if certain bit-flags are not set, writes a zero to a calculated memory location.
- `clear_buffer_with_calculated_offset`: Calculates a memory address using a value from a global variable, a register, and a constant, then writes a zero to that location.
- `update_buffer_with_string_result`: Calculates a string length and, if a certain condition is met, stores a character and a zero terminator into a buffer at a calculated offset.
- `search_short_table`: Iterates through a global array of 26 short integers, comparing each against a passed parameter.
- `execute_indirect_function`: loads a function pointer from a global variable and executes it.
- `allocate_memory_block`: Allocates a block of memory using the Exec library and returns the pointer.
- `initialize_graphics_library`: Allocates memory, opens the graphics library, and populates a data structure with pointers and values from global variables and library calls, returning a status code.
- `process_integer_list`: Iterates through a list of integers until a terminator value is found, invoking a library function for each entry.
- `process_list_with_lib`: Calls a library function, then iterates through a list of integers until a value of -1 is found, calling another library function for each entry.
- `empty_stub_2`: This routine is empty and performs no operations.
- `cleanup_allocated_memory`: Iterates through tables of function pointers and global variables to invoke cleanup routines, then frees allocated memory blocks and closes libraries.
- `initialize_game_state_structures`: Writes values to structure fields, touches the CIAA_ICR hardware register, and calls routines to build copper lists, copy data blocks, and initialize input and player states.
- `configure_cia_registers`: Writes specific values to CIAA control and interrupt registers and updates global status flags and memory offsets.
- `wait_and_set_flag`: Polls a bit in a global status word until it is set, then sets a corresponding bit in another global word and returns the status value.
- `repeat_wait_flag`: Repeats a polling and flag-setting operation a number of times determined by an input parameter, waiting for a specific bit in a global status word before setting a bit in another global word.
- `clear_memory_loop`: Calculates a count based on a global variable and an unable-to-determine input, then repeatedly clears a byte at a specific offset in a global structure, waiting for it to become non-zero before decrementing the count.
- `initialize_copper_list_2`: Constructs a sequence of graphics hardware commands in memory by writing control words and color data from global variables.
- `initialize_copper_list`: Constructs a sequence of graphics hardware commands in memory by writing control words and color data from global variables.
- `copy_block_data`: Copies 17 words from a source address derived from a global pointer to a destination buffer, then copies another 17 words from a different source offset to a separate destination offset.
- `add_offset_to_pointer`: Returns the value of a global pointer variable plus a constant offset of 96.
- `store_word_to_global_offset`: Stores a word into a global variable at a fixed offset.
- `update_buffer_state`: Checks a condition to either clear a single word or fill a memory range with a constant, or alternatively, fills a different memory range with a constant and updates a status flag.
- `copy_global_to_buffer`: Stores a value to a local offset and copies a sequence of words from a global variable area into a buffer structure.
- `toggle_and_update_state_buffer`: Toggles a byte in a global structure, increments a nearby short, and conditionally copies a sequence of words from a pointer-based table.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a register.
- `update_status_flags`: Masks and stores a value into a structure field at one offset, and sets a byte flag to 1 at another offset within a different structure.
- `set_offset_value`: Writes the value 8 to a specific offset in a data structure and returns a combined register value.
- `dispatch_indirect_jump`: Performs an indirect jump to an address stored in a register, effectively acting as a single-entry jump table or a way to redirect execution flow.
- `update_buffer_and_jump`: Decrements a counter, potentially flips a bit in a global word, and copies a sequence of words or fills a buffer before performing an indirect jump.
- `copy_data_and_jump`: Copies a sequence of words from a global variable area into a buffer and performs an indirect jump.
- `copy_and_populate_table`: Copies a sequence of words from a source pointer and then populates a destination buffer with a repeating pattern of bytes and a fixed word.
- `initialize_input_state`: Resets input counters and queue entries, reads hardware status bytes, and initializes local structure fields with specific constant values.
- `update_pointer_and_value`: Checks a condition and a divisibility rule, then increments a pointer and writes a value to a destination offset.
- `update_input_state_and_jump`: Updates a byte in a structure or global memory using a value from a keyboard or button state check, then performs an indirect jump to a target address.
- `validate_and_update_state_buffer`: Checks a value against a cached state and, if different, iterates through a table of pointer-based bounds or flags to validate a range or condition.
- `validate_state_bounds`: Iterates through a table of pointers to check if certain values fall within a specified range and updates a byte in a structure.
- `validate_table_bounds`: Iterates through a table of pointers to check if certain values fall within a specified range.
- `update_state_and_decrement_counter_4`: Checks a global variable and, if non-zero, decrements it, while also checking a table of pointers and updating a byte in a structure.
- `update_state_and_decrement_counter_3`: Iterates through a table of pointers, updates bytes in a structure, and decrements a byte in a structure if it is non-zero.
- `update_state_and_decrement_counter_2`: Stores a byte into a structure, updates a byte in another structure, and if a certain byte is non-zero, decrements it before performing an indirect jump.
- `update_state_and_decrement_counter`: Updates a byte in a structure using a return value, and if a certain byte in a structure is non-zero, decrements it before performing an indirect jump to a jump table or routine.
- `update_keyboard_state`: Checks a flag, clears a bit in a global variable, and sets or clears a bit in a structure based on a value derived from the keyboard serial register.
- `set_bit_in_byte`: Sets a specific bit in a byte at a certain offset within a structure, where the bit index is determined by a register.
- `read_byte_from_global`: Returns the byte value located at a fixed offset from a global pointer variable.
- `initialize_structure_fields`: Sets several byte and word fields within a structure to zero or specific constants, and then clears a block of 335 words pointed to by a pointer within that structure.
- `update_sprite_attribute_buffer_2`: Decrements a counter and, if non-zero, recursively calls itself while copying a sequence of bytes from a source table to a destination buffer.
- `update_sprite_attribute_buffer`: Decrements a counter and, if non-zero, recursively calls itself while copying a sequence of bytes from a source table to a destination buffer, and updates a pointer or index used for sprite or object attribute rendering.
- `increment_and_copy_data`: Increments a byte in a structure and copies a sequence of words from a table or global array to a destination buffer.
- `read_circular_buffer_byte`: Reads a byte from a circular buffer using a head index, decrements the index with wrapping, and returns the shifted byte value.
- `decrement_index_and_return_offset`: Decrements a counter or index and returns a calculated value based on the difference between a byte and a constant.
- `clear_byte_in_structure`: Sets a byte at a fixed offset within a structure to zero.
- `write_circular_buffer_byte`: Writes a byte into a circular buffer at the current tail index, decrements the index with wrapping, and processes a string until a null terminator is found.
- `update_index_on_match`: Stores a byte into a buffer at an offset and decrements an index, returning a status if the new index matches a certain value.
- `store_word_in_structure`: Stores a word into a structure at a fixed offset.
- `decrement_table_values`: Iterates through a sequence of ten memory locations and decrements any non-zero value found there.
- `update_input_state`: Initializes a structure with specific constant values for coordinates and flags, rotates a byte counter, and stores word values into indexed fields.
- `update_and_clamp_coordinates`: Resets certain structure fields to zero or calculates new coordinate positions using a scaling factor and a lookup table, then clamps the resulting values within a specified range.
- `update_input_values`: Compares values from a table and stores a word from a register into a global variable.
- `update_input_state_branch`: Checks a value in a table, copies a word to a global variable, and performs an indirect jump to a routine.
- `calculate_clamped_bounds`: Calculates the minimum or maximum of several values and stores the result back into a pair of global variables.
- `clear_structure_fields`: Resets several specific offsets within two different memory structures to zero.
- `calculate_coordinate_difference`: Calculates the absolute difference between two sets of byte-based coordinates and returns the result as a 64-bit value.
- `calculate_absolute_difference`: Calculates the absolute difference between two integers and returns the result as a 64-bit value with a sign-dependent negation.
- `return_immediately`: The routine contains instructions but performs no data manipulation or hardware access before returning control to the caller.
- `update_pointer_table`: Calculates an offset based on a byte, copies a word from a source address to a new location, and updates a byte value.
- `process_secondary_input`: Decrements a byte counter with a lower bound check, then calls another routine to compute values.
- `compute_input_flags`: Calculates a composite value by adding input registers to global offsets, checking specific bit conditions, and combining theresults into a structured return value.
- `check_button_state`: Reads a global variable representing hardware status and returns a boolean indicating whether a specific bit is cleared.
- `store_mouse_data`: Writes byte values from registers and memory locations into specific offsets of a global structure, while managing interrupt enable flags based on a condition.
- `store_word_data`: Stores two word-sized values from registers into specific offsets of a global structure and updates a global flag based on a memory condition.
- `store_unreached_words`: Stores two word-sized values from registers into specific offsets of a global structure, though this code path is not reached during normal execution.
- `manage_queue_operations`: Checks a state byte and a counter; if active, it waits for a flag to clear while decrementing a value.
- `clear_queue_entries`: Resets three short counters to zero and iterates through a table of word pairs, setting each pair to zero.
- `set_global_value`: Writes a constant value of one to a specific offset within a global structure.
- `clear_global_value`: Writes a constant value of zero to a specific offset within a global structure.
- `mask_and_check_bit`: Masks an input word with a constant, stores it in a global structure, conditionally sets a byte to zero if the lowest bit is clear, and returns the lowest bit.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `append_to_buffer_list_2`: Stores a pointer into a table indexed by a scaled global counter, increments the counter with a wrap-around check, and writes four words of data.
- `append_to_buffer_list`: Stores a pointer into a table indexed by a scaled global counter, increments the counter with a wrap-around check, and writes four words of data to the pointed-to buffer.
- `calculate_scaled_value`: Multiplies a register value by a global word, divides the result by a constant, ensures the outcome is at least one, and stores the byte result to a global variable.
- `transform_linked_list`: Iterates through a linked list of short values, copying and transforming them into a contiguous buffer.
- `render_text_string`: Clears a buffer and copies character glyph data from two global tables based on input string characters, then configures a hardware register structure with pointers and offsets and waits for a status bit to clear before repeating the process.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `initialize_struct_fields`: Sets multiple fields within two global structures to specific constant values, including pointers to offsets within the structure and a reference to a global variable.
- `process_hunk_header`: Reads six words from a pointer stored in a global structure, copies them to other fields in the structure, toggles a status byte, and swaps two pointer fields based on that status.
- `call_sprite_setup_routine`: The routine calls a helper to configure sprite rendering parameters and then invokes an empty stub.
- `calculate_sprite_offset_value`: The routine calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable, and calls a sprite rendering setup routine and an empty stub.
- `scan_sprite_stripes`: Iterates through a table of words using a pointer, calling a rendering routine for each entry until a negative value is encountered.
- `render_sprite_stripes`: Configures a graphics structure with source and destination coordinates, then iterates five times to draw horizontal lines while polling a hardware status register for readiness.
- `setup_sprite_rendering`: Stores three input parameters into global memory locations and then invokes two rendering routines.
- `return_immediately_5`: The routine performs no functional operations and immediately returns to the caller.
- `update_render_buffer_bounds`: Calculates a new buffer size or clipping boundary based on a pointer-referenced value and updates a global or structure-based limit.
- `update_clipping_bounds`: Calculates and stores the horizontal and vertical clipping boundaries and memory offsets for a graphical object.
- `calculate_sprite_dimensions`: Computes clipping bounds and memory offsets for a graphical object based on its position and size parameters.
- `wait_and_update_nibbles`: Waits for specific byte flags at certain offsets from a base pointer to become non-zero, then calls helper routines to decrement or increment nibble values in tables.
- `decrement_nibble_values`: Iterates through a table of 16-bit words, decrementing each non-zero nibble by one.
- `increment_nibble_if_different`: Compares nibbles of words in a destination table against corresponding words in a source table.
- `clear_pointer_arrays`: Retrieves a pointer to an array of pointers from a global variable.
- `decrement_instruction_table`: Iterates through a table, decrementing values and updating a jump table or a data structure with new pointers or bitmasks.
- `update_jump_table_logic_2`: Iterates through a table of instructions, decrementing their values and updating a corresponding jump table or data structure with new pointers and bitmasks.
- `update_jump_table_logic`: Modifies a jump table or a related memory structure by decrementing instruction or code values and updating pointers or bitmasks.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_table_entry`: Writes a byte, a word, and a pointer into a table at an offset determined by a register.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `write_jump_table_entry`: Writes a constant value, a 16-bit parameter, and a calculated code pointer into a global table at an index determined by a register.
- `parse_formatted_string_2`: Calls a routine to parse a format string and convert arguments into a formatted output buffer.
- `copy_string_buffer`: Copies bytes from a source pointer to a destination pointer in a loop based on a count parameter.
- `return_immediately_2`: This routine is empty and performs no operations before returning.
- `parse_icon_library_data`: Opens the icon library, parses string tokens, and processes data structures using DOS library calls for input and output.
- `parse_string_tokens`: Parses a string or a quoted string literal, skipping whitespace and extracting individual tokens into a destination table.
- `parse_string_tokens_3`: Parses a string into individual tokens, handling quoted substrings and skipping whitespace or control characters.
- `find_string_end_and_copy`: Finds the end of the first string and copies characters from a second string into it, stopping if a counter reaches zero or a null terminator is found.
- `copy_string_with_limit`: Finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied, stopping if the counter reaches zero or the source string ends.
- `get_window_info`: Calls a library function to get a pointer, then if successful, retrieves a value from a table and stores it in a structure member, while also updating a pointer or value in that structure.
- `copy_string_to_end`: Finds the end of the first string and copies all characters from a second string until a null terminator is encountered.
- `copy_string_to_null`: Locates the null terminator of the first string parameter by incrementing a pointer until a zero byte is found, then copies characters from a second string into that location.
- `copy_string_or_pad`: Copies characters from a source string to a destination buffer up to a specified count or until the source ends, or pads the destination with null bytes if the count is zero.
- `initialize_node_structure`: Calls a library function to check a value, and if a certain condition is met, initializes a structure or node with a passed parameter, a constant, and a pointer or value from a library call.
- `initialize_data_structure`: Sets specific bytes and words within a data structure, and conditionally calls functions from a global table or other functions using a register value.
- `parse_formatted_string`: Parses a string containing hex-encoded or decimal-encoded characters and special control characters, updating a destination buffer and a secondary integer or flag-based state.
- `calculate_string_length`: Receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminator.
- `setup_pointer_chain`: Stores an incremented version of a pointer into its own memory location, clears a subsequent word, and stores the original pointer value in a following word.
- `iterate_memory_range`: Increments a pointer or index by a fixed amount and calls another routine until a certain memory boundary is reached.
- `update_buffer_state_2`: Updates a buffer or linked list node by checking or modifying flags and writing a byte to a pointer.
- `process_linked_string_node`: Recursively traverses a linked list of string or table structures, performing conditional string copying and digit-to-character conversion for a temporary buffer.
- `handle_string_and_table_data`: Traverses a linked list and a table of structures, copies a string, and writes a sequence of characters to a buffer before clearing certain pointers.
- `return_zero`: The routine is empty and returns a zero value to another routine.
- `search_linked_list_nodes`: traverses a list of structures by incrementing a pointer, checking if a certain bitmask is absent and a certain field is non-zero, and recursively calling itself to find a specific termination condition.
- `search_table_for_condition_2`: Iterates through a table of structures, checking if a specific bitmask is absent and a certain field is non-zero, while recursively calling itself to validate a condition.
- `search_table_for_condition`: Iterates through a memory range, checking a bitmask or value in a table, and calls another routine until a certain condition or end-of-range is met.
- `execute_empty_routine`: this routine performs no operations and immediately returns to its caller.
- `execute_empty_stub`: The routine executes a sequence of instructions that perform no observable work and ends with a return.
- `get_absolute_value`: Calls another routine and returns a 64-bit value where the high-order word is the absolute value of a register and the low-order word is the original register content.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `allocate_and_validate_node`: Allocates memory for a linked list node and validates an index against bounds, setting certain fields in the structure.
- `traverse_linked_list`: Traverses a linked list of pointers until a specific address is found.
- `traverse_and_call_pointer`: Traverses a linked list of pointers and either stores a value from a pointer into a global variable or updates a global variable with that value before calling a function.
- `update_and_call_function`: Checks if a pointer is null, then either stores a value from a pointer into a global variable or updates a global variable with that value, before calling a function via a pointer-to-pointer lookup.
- `store_and_call_library`: Stores a value from a pointer into a global variable and then calls a library function.
- `iterate_linked_list_and_execute`: Traverses a linked list of pointers, executing a routine stored in a table for each node, and resets the list head to null.
- `iterate_and_call_function`: Traverses a linked list of pointers and calls a function pointer or routine for each element, then clears a global status variable.
- `prepend_node_to_list`: If the provided pointer is not null, it creates a new list node by storing a pointer to a routine and the provided parameter, then links this new node to the head of a global linked list.
- `copy_string_simple`: Copies characters from a source string to a destination buffer until a null terminator is encountered, returning the pointer to the start of the destination buffer.
- `check_input_validity`: Checks if an index is within a valid range and if a certain flag is set, then calls a library function and returns a boolean result.
- `execute_conditional_seek`: Checks if a parameter is within a valid range and if a certain value in a table is non-zero, then calls a library function to seek and updates a global status.
- `handle_file_error_status`: Calls a library function to check a status, and if zero, calls another library function to store a result before returning a failure code.
- `return_zero_2`: The routine returns a zero value.
- `execute_conditional_write`: Checks if a parameter or a table entry is valid, then calls a library function to write data and handles errors by updating a global status.
- `execute_conditional_recursion`: Calls a library function and checks a specific bit in the return value.
- `output_and_cleanup_callbacks`: Writes data to the standard output using library calls and then executes cleanup callbacks in a global structure.
- `execute_cleanup_callbacks_2`: Iterates through a count stored in a global structure, calling a validation routine for each index and then calls a final cleanup function if present.
- `execute_cleanup_callbacks`: The routine iterates through a count stored in a global structure, calling a validation routine for each index.
- `do_nothing_2`: the routine is empty and performs no operations.
- `update_and_clear_entry`: Checks if an index is valid and if a certain bit is clear, then calls a library function and clears a value in a table.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:bcdfq

- `main_program_entry`: Opens system libraries, initializes global variables, and enters a main loop that processes graphics, input, and hardware DMA channels, waiting for conditions before executing further routines.
- `check_cia_port_bit`: Checks if a specific bit in a global variable, which represents a hardware port, is clear.
- `check_memory_flag`: Checks if a specific bit in a global memory variable is clear.
- `wait_for_condition`: Polls two adjacent short values in a structure, returning immediately if the first is zero, otherwise waiting until the second becomes non-zero.
- `process_graphics_loop`: Executes a loop that disables interrupts, waits for the top of frame, processes a table of graphics objects by updating their properties, and re-enables interrupts.
- `open_file_resource`: Calls a function pointer and returns true if the return value is zero.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `copy_block_words`: Copies a block of 3000 words from a source pointer to a destination pointer, where the source is offset by 12000 words.
- `clear_and_copy_block`: Calls a function pointer, then copies 8000 words from a source buffer to a destination buffer, where the source is offset by 12000 words.
- `loop_call_until_flag`: Iterates through a table of short values, calling a function pointer for each non-zero entry until a flag in a global structure becomes non-zero.
- `iterate_function_table`: Iterates through a table of short values, calling a function pointer from a global structure for each non-zero entry.
- `loop_call_until_flag_2`: Iterates through a table of short values, calling a functionpinter for each non-zero entry until a flag in a global structure becomes non-zero.
- `call_function_pointer`: Calls a function pointer located at a negative offset from a base address stored in a global structure.
- `loop_decrement_nibbles`: The routine executes a countdown loop from fifteen to zero.
- `decrement_nibble_values_2`: Iterates through a list of words, decrementing any non-zero nibble components within those words.
- `loop_increment_nibbles`: The routine executes a countdown loop from fifteen to zero.
- `increment_nibble_values`: Iterates through two lists of words, incrementing the nibble components of the first list if they differ from the corresponding components in the second list.
- `blit_multi_source_blocks`: Configures and executes multiple blitter operations using hardware registers, waiting for the blitter to finish before proceeding with subsequent blocks.
- `set_dma_channel_mask`: Masks the lower six bits of an input byte.
- `loop_clear_global_flag`: The routine executes a countdown loop from fifteen to zero, calling another routine in each iteration.
- `update_dma_and_structure`: Updates a structure and a global variable with values from registers, including a bitmask for a DMA controller, and modifies a pointer or a short value.
- `update_structure_with_register_values`: Stores a value from a function return into a structure and a global variable, then conditionally updates a pointer and a short value based on a register or a pointer offset.
- `store_register_values_2`: The routine stores register or register-derived values into a structure and a global variable.
- `update_cia_timer_values`: Stores register values into specific offsets of a structure and a global variable.
- `enter_infinite_loop`: The routine contains no executable instructions and effectively enters an infinite loop.
- `conditional_pointer_copy`: Checks a bit in an input register and a field in a structure; if certain conditions are met, it copies a pointer and a 16-bit value to a destination pointed to by another field in the structure.
- `copy_pointer_conditionally`: The routine checks a bit in a register and a field in a structure; if certain conditions are met, it copies a pointer and a 1new 16-bit value to a destination.
- `update_dma_config_list`: Iterates through a list of pointers to update a global DMA configuration structure and a counter.
- `process_dma_config_list`: Iterates through a list of pointers to update a global DMA configuration structure and a counter.
- `update_pointer_table_entry`: Checks a control byte and, if certain conditions or counter-based flags are met, updates a global pointer or table entry using a value derived from a register or a pointer-based lookup.
- `update_conditional_pointer_table`: Checks a control byte and, if certain conditions or counter-based flags are met, updates a global pointer or table entry using a value derived from a register or a pointer-based lookup.
- `update_dma_config_from_list_2_2_2`: Updates a global DMA configuration structure by iterating through a list of pointers and updating a counter.
- `update_dma_config_from_list_2_2`: Iterates through a list of pointers to update a global DMA configuration structure and a counter.
- `increment_global_counter`: Increments a global variable by one.
- `update_text_formatting_parameters`: Updates a global text buffer and associated formatting parameters, such as character spacing, tab stops, and backspace-related offsets, based on a stream of control characters.
- `process_control_escape_sequences`: Parses a stream of control characters to update a global buffer, adjusting text spacing, or setting certain character or length parameters.
- `update_dma_config_from_list_2_3_2_2_2`: Updates a global DMA configuration structure and a counter by iterating through a list of pointers and performing conditional table-based or logic-based updates to a configuration or state variable.
- `update_dma_config_from_list_2_3_2_2`: Updates a global DMA configuration structure and a counter by iterating through a list of pointers and performing conditional table-based or or logic-based updates or checks.
- `update_dma_config_from_list_2_3_2`: Updates a global DMA configuration structure and a counter by iterating through a list of pointers and performing conditional table-based or arithmetic-based memory writes.
- `update_dma_config_from_list_2_3`: Iterates through a list of pointers to update a global DMA configuration structure and a counter.
- `update_dma_config_from_list_2`: Iterates through a list of pointers to update a global DMA configuration structure and a counter.
- `update_dma_config_from_list`: Iterates through a list of pointers to update a global variable and modify DMA controller settings.
- `process_pointer_list_with_side_effects`: Iterates through a list of pointers, updating a register with a byte from a structure, and conditionally updates a global variable or a value at a pointer offset before calling another routine.
- `load_scaled_value`: Loads a 16-bit value from a table or a calculated division result into two separate global byte registers.
- `update_table_or_divide`: Checks a register value to either perform a table lookup or a division, then stores the resulting low and high bytes into a global memory location via a pointer.
- `lookup_or_divide_table_entry`: Checks if an input index is within bounds to read a 16-bit value from a global table; otherwise, it divides a global constant by the input and stores the low and high bytes into separate global byte variables.
- `tiny_ori_2`: Small routine of 1 instruction that merges 240 into d7.
- `store_global_and_return_offset`: Stores the address of a global variable into a slot indexed by a register and returns a value derived from an input pointer.
- `add_to_pointer_if_nonzero`: Reads the value pointed to by a pointer and, if that value is non-zero, adds an input value to it and stores the result back through the pointer.
- `adjust_pointer_offsets`: Runs a helper routine in a loop until it reports completion, then walks an array of pointers adding a base address to each entry when a specific field is negative.
- `scale_palette_from_source`: Reads a scaling byte from a structure and multiplies a source table by that byte, storing the results in a global table; otherwise, it either clears a region of a global variable or copies a variable-length block from a secondary structure.
- `update_global_status_bits`: Checks a global variable and, if non-zero, modifies certain bits of a global variable and updates a global pointer or value using data from a memory offset.
- `check_ciab_and_call_handler`: Checks a global variable and calls a handler if non-zero.
- `clear_flag_and_call_handler`: Clears the least-significant bit of a byte stored through a pointer and then clears a global variable that holds a handler pointer, calling that handler before returning its result.
- `open_resource_and_validate`: Attempts to open a resource and checks if certain library functions return non-zero, updating several global pointers and a status variable accordingly.
- `close_ciab_resource`: Checks if a global resource base pointer is non-zero, clears the global pointer, and returns the result of the close operation.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:bcdft

- `initialize_and_free_memory`: The routine serves as the program entry point.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Black Crypt (1992)(Electronic Arts)(Disk 2 of 3).adf:bcdfu

- `initialize_game_environment`: Opens required system libraries, brings the screen to the front, and disables DMA channels.
- `iterate_null_terminated_string`: Iterates through a null-terminated string by calling another routine for each non-null character.
- `loop_copy_tile_data`: The routine iterates through a null-terminated string.
- `copy_tile_data_block`: Copies eight bytes from a source buffer to five destination locations spaced by 8000 bytes, using offsets derived from input registers.
- `clear_global_offset_value`: Sets a byte at a specific offset within a memory location to zero.
- `multiply_and_load_rgb`: Multiplies a register value by a global memory value, stores the result in a global memory location, and calls a graphics library routine to load RGB4 data.
- `check_cia_port_bit_2`: Checks if a specific bit in a global variable is clear.
- `check_potget_bit_two`: Reads the POTGOR register and returns true if bit two is clear.
- `clear_main_buffer`: Sets four consecutive longwords to zero in a loop, repeating the operation 6000 times to clear a large memory block.
- `clear_offset_buffer`: Sets four consecutive longwords to zero in a loop, repeating the operation 500 times on a memory block offset by 32000 bytes.
- `multiply_and_store`: Multiplies a register value by a value from a global memory location and stores the result in another global memory location.
- `wait_for_zero`: Enters a loop that continues as long as a certain value in a global memory location is non-zero.
- `wait_for_flag_or_zero`: Loops until a primary global variable becomes zero or a secondary global variable becomes non-zero.
- `process_graphics_loop_2`: Executes a loop that disables interrupts, waits for the top of frame, processes a table of graphics objects by updating their properties, and re-ensables interrupts.
- `wait_for_ram_flag`: Polls a byte at a fixed offset in RAM until it equals 15.
- `open_file_resource_2`: Returns true if the open operation failed.
- `close_file_resource`: Calls the library function to close a file handle.
- `decode_rle_chunk_one`: Invokes a function pointer from a global structure and decodes run-length encoded data from a source to a destination buffer.
- `execute_indirect_function_3`: Retrieves a function pointer from an offset within a structure and calls it.
- `read_file_data`: Calls the DOS library to read data from the currently open file handle into a buffer.
- `read_and_decode_rle_data`: Invokes the DOS Read library function to fetch data, then passes the buffer to a routine that decodes run-length encoded information.
- `call_dos_read`: Calls the DOS library function to read data from the currently open file handle into a buffer.
- `shift_buffer_data`: Copies a block of 32-bit words from a higher memory address to a lower address, overlapping the source and destination.
- `decode_and_call_global`: Calls a function pointer stored in a global variable, then invokes a routine to decode run-length encoded data.
- `invoke_decode_rle`: Calls a function pointer stored in a global structure to trigger data decompression, relying on another routine to process the payload.
- `clear_buffer_region`: Sets a large block of 32-bit words to zero in a specific memory region.
- `decode_rle_data`: Decodes run-length encoded data from a source pointer to a destination pointer, handling literal copies and repeated byte expansions.
- `loop_decrement_nibbles_2`: Executes a countdown loop from fifteen to zero, calling two function pointers and a routine that decrements nibble values in an array, then clears a word-sized field in a structure.
- `decrement_nibble_values_3`: Iterates through a list of words, decrementing each of the three nibble components if they are non-zero.
- `call_load_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls LoadRGB4 and returns its result to the caller.
- `loop_increment_nibbles_2`: Executes a countdown loop from fifteen to zero, calling two function pointers and a routine that increments nibble values on mismatch, then sets a word-sized field in a structure to one.
- `decrement_nibble_mismatch_counter`: Executes a countdown loop from fifteen to zero, calling two function pointers and a routine that processes nibble differences in each iteration, before setting a field in a structure to one.
- `increment_mismatched_nibbles`: Iterates through two lists of words, incrementing the high-order nibble, middle nibble, or low-order nibble of the first list if that nibble does not match the corresponding nibble in the second list.
- `decode_nibble_differences`: Iterates through two arrays of words, comparing corresponding 4-bit nibbles and decrementing the destination nibble if the source nibble is greater.
- `initialize_graphics_buffers`: Initializes a memory block by setting words to a specific value, then calls routines to clear buffers, decode data, load graphics, and process differences.
- `check_string_and_potget`: Computes the length of a zero-terminated string and checks if a specific bit in a hardware register is clear.
- `configure_blitter_segments`: Iterates through a table of offsets to configure Blitter registers for multiple copy operations while waiting for a certain condition.
- `blit_copper_list_segments`: Owns the Blitter and iterates through a table of offsets to configure Blitter registers for multiple copy operations while waiting for the Blitter to finish.
- `iterate_and_blit_strips`: Disowns the blitter, then iterates through a buffer of short integers, calling a routine to perform a blit operation for each non-zero entry, and finally reclaims the blitter.
- `blit_vertical_stripes`: Configures the Blitter to copy data with specific stride and size parameters, performing five sequential Blitter operations.
- `copy_tile_data_blocks`: Copies blocks of 32-bit words from a source pointer to a destination pointer using a nested loop structure.
- `blit_conditional_pattern`: Calculates Blitter parameters and sets source and destination pointers based on whether an input value is negative or within a certain range, then executes a Blitter operation.
- `blit_conditional_pattern_2`: Calculates Blitter parameters based on an input value, setting different source and destination pointers and sizes depending on whether a global flag is set, repeating the operation in a countdown loop.
- `set_dma_channel_mask_2`: Masks the lower six bits of an input byte and, if the result is less than four, stores a bitmask in a global variable.
- `loop_clear_global_flag_2`: Executes a countdown loop from fifteen to zero, calling another routine in each iteration, and then sets a global variable to zero.
- `update_control_structure`: Checks if a global resource pointer is non-zero, then updates a structure and a global variable with register-derived values or certain bitmasks.
- `update_control_pointer_and_metadata`: Updates a pointer-based structure or a global variable with a value from a return or a register, and conditionally stores a pointer to a global variable and a constant or register-derived value into a control structure.
- `update_control_fields`: Updates a structure or a global variable with register-derived values, specifically setting certain offsets or pointers.
- `update_control_structure_fields`: Stores register-derived values into a structure and a global variable, specifically updating certain offsets with register contents or a calculated value involving a register and a constant offset.
- `enter_infinite_loop_2`: The routine is empty and enters an infinite loop.
- `conditional_pointer_copy_2`: Checks a bit in a register and a field in a structure; if both are non-zero, it copies a 16-bit value and a pointer from the structure to a new destination and returns the input register shifted right by one bit.
- `copy_conditional_pointer`: Checks a bit in a register and a field in a structure; if both are non-zero, it copies a 16-bit value and a pointer from the structure to a destination.
- `update_control_structure_6`: Updates a control structure and a global variable using values derived from a table or a division operation, and modifies a DMA control register.
- `update_control_structure_value`: This routine updates a control structure and a global variable using values derived from a table or a division operation.
- `execute_countdown_loop_wrapper`: A short routine that calls a countdown loop and is called by a routine that updates a structure and a global variable.
- `execute_countdown_loop`: Executes a loop that counts down from a fixed value while calling another routine and resets a global variable to zero.
- `update_control_structure_3`: Calls routines that mask a byte and update a structure or its parameters.
- `update_control_structure_2`: A short routine that calls a routine to mask a byte and store a bitmask in a global variable, then calls another routine to update a structure.
- `update_control_structure_short`: A two-instruction routine that calls a routine to update a structure and a global variable.
- `update_control_structure_5`: Updates a control structure and a global variable using a table via a wrapper routine and another routine.
- `update_control_structure_4`: Updates a structure using a table via a wrapper routine and modifies a control structure and a global variable.
- `update_structure_from_table_wrapper`: A short routine that calls a routine to update a structure from a table and is called by a routine that updates a structure and a global variable.
- `update_structure_from_table_2`: Updates a structure and loads a value from a table into a global variable, called by a routine that modifies a structure and a global variable.
- `update_structure_from_table`: Calls routines that update a structure and load a value from a table into a global variable.
- `update_control_structure_minimal`: A short routine that calls a routine to update a structure and a global variable.
- `update_control_structure_parameters`: A routine consisting of 78 instructions that updates a structure and a global variable using register-derived values or specific bits, and modifies a DMA control register.
- `modify_dma_control_register`: Modifies the DMA control register.
- `iterate_and_update_struct_fields`: Iterates through a list of pointers, updating a counter and checking a termination condition.
- `update_value_from_table`: If a register value is less than a certain threshold, it loads a value from a table using an offset, otherwise it performs a division and stores the result as a 16-bit value in a global location.
- `update_value_from_table_2`: Loads a 16-bit value from a table or via division and stores the lower and upper bytes into a global variable.
- `lookup_or_divide_table_entry_2`: Checks if an input index is within bounds to read a 16-bit value from a global table; otherwise, it divides a global constant by the input and stores the low and high bytes in global byte variables.
- `store_address_and_return_offset`: Stores the address of a global variable into a structure field and returns a value containing a pointer and an offset.
- `store_global_and_return_offset_2`: Stores the address of a global variable into a slot indexed by a register and returns the sum of a pointer and a constant offset.
- `add_to_pointer_if_nonzero_2`: Checks if the value at a pointer is non-zero, and if so, adds a value to that memory location.
- `adjust_pointer_offsets_2`: Runs a helper routine in a loop until it reports completion, then walks an array of pointers adding a base address to each entry when a specific field is negative.
- `scale_palette_from_source_2`: Reads a scaling byte from a structure, multiplies a source table by that byte, and stores the results in a global table, or copies a variable-length block from a secondary structure if certain conditions are met.
- `modify_global_variable_bits`: Checks a global variable and, if non-zero, clears a bit in a global variable and updates a global variable or pointer using data from a memory offset.
- `update_global_state_and_flags`: Checks a global variable and, if non-zero, clears a bit in a global variable and updates a global variable or pointer using data from a memo.
- `update_global_state_and_bits`: Checks a global variable and, if non-zero, clears a bit in a global variable and updates a global variable or pointer using data from a memory location.
- `check_handler_and_clear_button`: Checks if a global variable is non-zero and calls a handler if so, then clears the least-significant bit of the CIAA_PRA register.
- `clear_flag_and_call_handler_variant`: Clears the least-significant bit of a byte stored through a pointer and then clears a global variable that holds a handler pointer, calling that handler before returning zero if the global variable is zero.
- `open_resource_and_set_status`: Opens a resource and, if successful, updates several global pointers and a status variable based on the returned pointer and further library calls.
- `close_ciab_resource_2`: Checks if a global pointer is non-zero, clears it, and calls a function pointer retrieved from a structure.
- `halt_on_bad_data`: The routine contains invalid or bad instruction data.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block`: Fills a block of memory with zeros using a loop.
- `initialize_system_libraries`: Opens graphics.library and intuition.library, verifying success via return values and error strings.
- `poll_function_pointers`: Repeatedly executes a function pointer from a structure and checks the return value of a second function pointer from the same structure until the latter returns a non-zero value.
- `initialize_library_call_state_three_2`: Sets a global state variable to three and stores a parameter in a global variable.
- `initialize_system_state`: Initializes system libraries, sets specific values in a structure, and performs a bitwise loop to modify a value based on a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_structure`: Zeroes out a range of memory in a global structure and conditionally calls a function pointer based on a flag in a global variable.
- `return_immediately_6`: This routine performs an immediate return to the calling function without executing any instructions.
- `initialize_resource_header`: Initializes a data structure with a specific magic identifier and sets various status flags.
- `parse_string_tokens_5`: Parses a string into individual tokens by skipping whitespace and handling quoted substrings.
- `tokenize_string_into_array`: Parses a source string into individual tokens by handling whitespace and quoted substrings.
- `copy_string_to_buffer_2`: Calculates the length of a source string and copies its contents into a destination buffer.
- `append_string_with_limit`: Finds the null terminator of the destination string and begins copying characters from the source string.
- `load_window_data`: Opens a library and retrieves data from a structure to populate several fields in a parameter block.
- `append_string_to_buffer_3`: The routine advances a pointer to the end of the first string, then copies characters from the second string into that location until the null terminator is copied.
- `copy_string_or_pad_4`: Copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `get_string_length`: Calculates the number of bytes in a null-terminated string by iterating through memory until a zero byte is encountered.
- `setup_pointer_chain_3`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `copy_string_to_buffer_3`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `execute_cleanup_callbacks_3`: The routine iterates through a list of function pointers stored in a global structure, calling each one.
- `dispose_resource_structure`: Iterates through a list of indices to call a validation routine for each, then executes a series of conditional function pointers stored within a global structure.
- `nop_padding_sequence`: An empty routine consisting of two instructions that perform no operation.
- `close_file_handle`: The routine validates an index against a table, checks if a pointer is non-zero, and calls a function to close a resource before clearing a value in the table.
