# Game 2170 (black crypt (1992)(electronic arts)[cr trsi][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02170). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2713424 top-level bytes in 51 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 19 of 19 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 517 of 517 named (0 author symbols), 517 of 517 described (32 by rule, 477 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1094 of 1094 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 144169 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16648 of 16663 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2025689 |
| code | 53156 |
| data | 455164 |
| padding | 12503 |
| unused | 166912 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Stack | 2 | 117 | 2 |
| avail | 11 | 376 | 12 |
| runback | 3 | 709 | 39 |
| endcli | 2 | 117 | 2 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:TRSI! | 1 | 116 | 3 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:InstallCrypt | 3 | 3235 | 57 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:BlackCrypt | 3 | 1745 | 48 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bcdfp | 3 | 6150 | 184 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bcdfq | 2 | 983 | 56 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bcdft | 5 | 27 | 2 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 2 of 3).adf:bcdfu | 2 | 1490 | 84 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 187 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 582 | 17 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 2 of 3).adf:bcdfx (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 217 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- black crypt (1992)(electronic arts)[cr trsi][a](disk 2 of 3)/Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 2 of 3).adf:bcdfx: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- black crypt (1992)(electronic arts)[cr trsi][a](disk 2 of 3)/Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- black crypt (1992)(electronic arts)[cr trsi][a](disk 3 of 3)/Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- black crypt (1992)(electronic arts)[cr trsi][a](disk 1 of 3)/Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:TRSI!: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- black crypt (1992)(electronic arts)[cr trsi][a](disk 1 of 3)/Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bcdft: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 144169 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: Copies sixteen words from a source address to a destination address pointed to by a stack pointer.

### Stack

- `process_linked_list_and_buffer`: Traverses a linked list to find the maximum value in a specific field and then prepares a data buffer by writing a header and copying data blocks.
- `copy_memory_block`: Copies sixteen bytes from a memory location to a destination pointed to by a stack address.

### avail

- `parse_command_line_arguments`: Parses a string of command-line arguments by splitting them into individual tokens based on whitespace and handling quoted strings.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `sum_memory_region_sizes`: Iterates through a linked list of memory regions and sums the size of any region whose attribute flags match a provided mask.
- `print_memory_availability_table`: Calculates and displays memory statistics including available, in-use, maximum, and largest block sizes for chip, fast, or total memory types based on command-line arguments.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### runback

- `clear_table_and_recurse_2`: Clears a block of forty words starting at an offset from a base pointer and stores a local stack address and a global variable into subsequent locations.
- `run_background_task`: Parses a command line for an optional load delay and a program name with parameters, then starts the named program as a background task, handling errors such as missing command, missing file, directory instead of file, out of memory and usage errors, and prints status messages.
- `clear_table_and_recurse`: Clears a block of forty words starting at an offset from a base pointer, stores a local stack address and a global variable into subsequent fields, and calls another routine.
- `call_optional_handler_and_recurse`: Checks if a global variable is non-zero to call a function pointer, then calls itself recursively.
- `process_dialogue_text`: Parses a string to extract individual words and updates global state fields.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `run_back_loop_handler`: Executes a routine ten times in a loop and then conditionally calls several other routines based on values stored in a table.
- `append_string_with_count_2`: Copies a source string to the end of a destination string until a null terminator is reached or a maximum length is exceeded.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination and returns it.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `lookup_and_execute_table_entry`: Validates an index against a table, checks a bit flag in the entry, and either calls another routine or clears the entry before returning a status.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_offset_102`: Calls a function from a library using an offset derived from a global base pointer.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_indirect_function_via_offset`: Retrieves a function pointer from a nested table structure using two offsets from a base address and executes it.
- `execute_indirect_function_pointer`: Fetches a pointer from a location offset from a global variable and executes the function at that address.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Retrieves a pointer from a location offset from a global variable and executes the function found at that address.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_indirect_function_via_offset_2`: Fetches a function pointer from a nested structure using two offsets and executes it.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `execute_indirect_function_4`: Performs an indirect call to a function pointer located at a specific offset from an address stored in a table.
- `call_indirect_function`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_indirect_function_2`: Performs an indirect call to a function pointer retrieved from a table using a different offset.
- `call_library_function`: Calls a function from an Amiga OS library using a vector retrieved from a global variable.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### endcli

- `process_linked_list_and_update_buffer`: Traverses a linked list to find the maximum value in a specific field and then updates a data structure by writing a header and copying a block of data.
- `copy_memory_block_2`: Copies a sequence of sixteen words from a source pointer to a destination pointer and then calls another routine.

### Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:TRSI!

- `process_input_and_mask_data`: Retrieves an input word and conditionally shifts it right by one bit.
- `select_half_or_input_word`: Returns the input word shifted right by one bit if a specific flag is set, otherwise returns the original word.
- `mask_table_value`: Retrieves a word from a table using an index and performs a bitwise AND operation with another value.

### Port-Handler

- `parse_amiga_hunk_headers`: Iterates through a data structure containing hunk type codes to identify executable or relocation segments.

### Disk-Validator

- `update_display_buffer`: Updates a character buffer with numeric values and performs bitwise operations to populate a data structure.

### Ram-Handler

- `execute_indirect_function_3`: Fetches a function pointer from a location relative to a global variable and executes it.

### Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:InstallCrypt

- `initialize_global_state`: Clears a block of memory with zeros and initializes several global variables and function pointers.
- `wait_for_installer_window_event`: Locks the current directory and waits for window messages, using gadget codes to determine the outcome before releasing the lock and returning.
- `open_install_window_and_buffers`: Clears several state fields, then opens the installer window and calls a routine with the value 10240, storing each result in a state field and calling a failure routine when a required result is zero.
- `draw_install_path_prompt`: Draws the installation dialog through a long fixed sequence of graphics calls for moving the pen, setting colour and drawing text.
- `show_install_failure_and_close`: Draws an install-failure message with the exit code when its code argument is non-zero, then releases the installer window and its resources.
- `copy_counted_name_with_colon`: Copies a name from an indexed table into a destination using a length byte, capping the length at a specific value.
- `check_entries_under_forbid`: Forbids multitasking, walks a linked list of entries and calls two routines for each entry with a non-zero field, setting the return value to 1 when a comparison fails.
- `prompt_insert_disk_and_wait`: Checks a path and, if it is not available, draws a prompt asking the user to insert the named disk and waits for a close or gadget event.
- `copy_file_with_progress_message`: Copies a file from a source to a destination by opening both, reading and writing blocks in a loop, and closing them.
- `install_game_files_to_destination`: Prepares the destination directory, creating it if it is missing, and walks a table of files to copy.
- `forward_base_offset_pointer`: Calls another routine with a pointer into a fixed offset of the data base, the argument it received, and the address of its own stack argument, then returns that result.
- `copy_memory_block_3`: Copies a specified number of bytes from a source address to a destination address.
- `compare_byte_strings_ordered`: Compares two byte strings from their start and returns 1 if the first is greater, -1 if smaller, or 0 if they are identical.
- `nop_operation`: The routine is empty and performs no operations.
- `init_control_block_with_streams`: Allocates a control block, initializes its fields with a four-character tag, and uses current input and output streams to process a message.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_and_store_string`: Parses a source string into individual tokens by splitting on whitespace and handling quoted substrings.
- `copy_string_with_limit_2`: Calculates the length of a source string and then copies characters from a second string into the destination buffer.
- `append_string_with_limit_4`: Finds the null terminator of the destination string and begins copying characters from the source string.
- `open_window_resource_2`: Opens a library and window resource, then stores the resulting pointers and values into a structure.
- `append_string`: Finds the end of the first string and copies the second string to that location.
- `copy_string_to_destination`: Finds the end of a null-terminated string and then copies a second null-terminated string to that location.
- `copy_string_or_pad_3`: Copies characters from a source string to a destination buffer until a specified count is reached or the source string ends.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `parse_expression_string`: Parses a string to evaluate mathematical operations by identifying numeric values and operators.
- `calculate_string_length_2`: Calculates the length of a null-terminated string by iterating through its bytes until a zero byte is encountered.
- `run_routine_over_table_entries`: Walks a table of 22-byte entries in the data area and calls another routine once for each entry until the end of the table.
- `manage_stream_buffer_state`: Validates a stream structure's flags and pointers before performing recursive updates to buffer offsets and data pointers.
- `clear_linked_list_node`: Recursively traverses and clears a linked list structure by zeroing its fields and converting a numeric value into a five-character null-terminated string.
- `clear_list_entry`: Clears a structure by zeroing its fields and converts a numeric value into a five-character null-terminated string stored in a local buffer.
- `return_zero_3`: This routine performs no operations and returns zero to the caller.
- `tiny_addq_addq`: Small routine of 2 instructions that adds 2 to a5, then adds 8 to d0.
- `traverse_linked_data_structures`: Iterates through a table of entries by incrementing a pointer by a fixed offset.
- `iterate_table_entries`: Iterates through a table, checking specific bit flags in each entry and calling another routine until a condition is met.
- `execute_nested_calls`: This routine performs two consecutive calls to other subroutines before returning.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `perform_signed_division_2`: Calculates a signed division by performing an unsigned division and then adjusting the sign of the result based on the input values.
- `call_and_concatenate_result`: Calls another routine and returns a 64-bit value by concatenating the original contents of a register with the value left in that register by the called routine.
- `invoke_division_routine`: Calls a routine that performs a 32-bit integer division using the values in two registers and then returns.
- `perform_32bit_division_2`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `init_stream_buffer`: Initialises a stream record's buffer pointers and flags.
- `insert_into_linked_list`: Traverses a linked list to find a specific pointer, then inserts a new node into the list and calls another routine.
- `traverse_and_free_linked_list`: Iterates through a linked list by following pointers, calls a routine to free each individual node, and then sets the list head to null.
- `free_all_list_nodes`: Frees every node of a linked list through a library call, then clears the list head and returns zero.
- `insert_into_linked_list_2`: Inserts a new element into a linked list by allocating a node and updating the pointers of the previous head.
- `copy_string_simple_3`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `check_interactive_status_2`: Checks if an index is within valid bounds and if a corresponding entry is non-zero, then calls a library function to determine interactivity.
- `perform_file_seek_2`: Validates an index against a table and calls a routine to perform a seek operation, updating a global variable with the result.
- `handle_file_error`: Checks the result of a file operation and, if it fails, stores an error value in a global variable and returns a failure code.
- `return_zero_4`: This is an empty routine that returns a zero value to the caller.
- `write_table_entry`: Validates an index and bitmask within a table before calling a routine to perform a write operation.
- `check_library_return_status`: Calls a function via a pointer in a global structure and checks if a specific bit is set in the return value.
- `write_four_bytes_to_output`: Writes four bytes from a global buffer to the standard output through the operating system, then flushes the output and calls another routine with the value one.
- `drain_list_then_run_cleanup`: Repeatedly removes the head of a linked list and calls the cleanup routine stored in each node until the list is empty.
- `shutdown_installer_resources`: Closes each stream entry, releases a directory lock, and frees the window, screen and library resources held in global fields, restoring a saved value along the way.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `close_list_entry`: Validates an index against a table, calls a library function to close a resource if a specific bit is not set, and clears the entry in the table.

### Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:BlackCrypt

- `initialize_program_state`: Clears a series of memory locations, sets a global pointer, and checks a bit in a register to decide whether to call another routine or initialize a library.
- `manage_workbench_disk_access`: Opens and closes the Workbench interface, locks the game disk volume, and executes a loop processing input events and state changes before unlocking the disk.
- `initialize_graphics_and_config`: Opens the graphics and intuition libraries, loads the configuration file, and initializes bitmaps and screen structures for the main game window.
- `execute_shutdown_callbacks_3`: The routine checks multiple global flags and executes corresponding function pointers from library bases.
- `delay_and_unload_segment`: Sets a value in a global variable and calls several indirect function pointers.
- `poll_input_event_loop`: Loops calling an input dispatch routine until a specific condition is met, handling cases for event types 1 through 4 by updating global state variables or invoking other handlers.
- `convert_string_case`: Copies a byte array and converts lowercase characters to uppercase by subtracting a constant.
- `search_linked_list_for_string`: Iterates through a linked list of structures, calling a routine to convert a string to uppercase and another to compare it against a target parameter.
- `process_dos_directory`: Iterates through a directory structure, performing string manipulations and calling various function pointers based on the contents of the directory entries.
- `wait_for_library_call`: Loops calling a function pointer from a library base until a second function pointer returns a non-zero value.
- `verify_segment_data`: Checks the return values of several function calls, updates global pointers, and compares a value from a structure against a parameter.
- `execute_library_shutdown_callbacks`: Invokes function pointers from library bases and global structures to perform shutdown sequences, checking flags and iterating through callback lists.
- `handle_segment_load_unload`: Calls routines to load or unload memory segments, checks the return value, and if successful, invokes a function pointer with calculated parameters before cleaning up.
- `copy_tiles_until_null`: Iterates through a null-terminated string and calls a routine to copy tile data for each character.
- `nop_padding_sequence`: This routine is empty and consists of instructions that perform no operation.
- `calculate_length_and_copy_tiles`: Calculates the length of a null-terminated string and iterates through it, calling a routine to copy tile data for each character.
- `call_subroutines`: Calls two instances of another routine sequentially before returning.
- `call_indirect_function_3`: Fetches a function pointer from a memory location calculated from a global variable and executes it.
- `initialize_menu_string_tiles`: Calls routines to process multiple null-terminated strings, such as menu options, by copying tile data for each character using specific length and buffer parameters.
- `check_input_event`: Polls an input event queue, checking message types and data fields to determine a return value based on specific conditions.
- `dispatch_input_event_handler`: Polls an input event queue and dispatches execution to different sub-routines based on the event type code.
- `store_register_values`: Stores the contents of data registers into specific global memory locations.
- `copy_tile_data`: Copies a block of data from a source pointer to multiple destination addresses with fixed offsets, iterating five times.
- `scan_global_variable_array`: Iterates through a global variable array in two passes, stepping by two bytes.
- `decrement_nibble_loop`: Executes a loop sixteen times, calling indirect function pointers and a routine to decrement nibble values in a list of words.
- `decrement_nibble_values`: Iterates through a list of words, decrementing each of the three nibble components if they are non-zero.
- `increment_nibble_loop`: Executes a loop sixteen times, calling indirect function pointers and a routine to increment nibble values in lists of words.
- `increment_nibble_values`: Iterates through two lists of words, incrementing the nibble components of the first list if they differ from the corresponding components in the second list.
- `copy_byte_array`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `compare_string_bytes`: Compares two byte strings character by character and returns a status code based on the first difference or equality.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `handle_workbench_disk_access`: Interacts with the Workbench interface, locks the game disk volume, parses strings, and opens icon libraries to process input events.
- `parse_string_tokens`: Parses a string into individual tokens by skipping whitespace and handling quoted substrings.
- `tokenize_string_into_array`: Parses a string into individual tokens by splitting on whitespace and handling quoted substrings.
- `copy_string_with_limit`: The routine finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `append_string_with_limit`: Locates the null terminator of the destination string and begins copying characters from the source string.
- `open_window_resource`: Opens a library and uses it to retrieve and store information related to a window structure into a provided data block.
- `append_string_to_buffer`: The routine advances a pointer to the end of the first string, then copies characters from the second string into that location until the null terminator is copied.
- `copy_string_or_pad`: The routine copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `allocate_structure_member`: The routine attempts to allocate a structure, populates several fields with provided parameters and library-returned values, and handles a recursive call if a specific parameter is zero.
- `initialize_library_context`: The routine checks a field in a structure; if non-zero, it calls a function from a library base.
- `calculate_string_length`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `copy_string_simple`: The routine copies characters from a source string to a destination buffer until the null terminator is copied.
- `execute_shutdown_callbacks_2`: Iterates through a list of indices to invoke a helper routine and executes conditional function pointers stored in global memory.
- `execute_shutdown_callbacks`: Iterates through a list of indices to invoke a helper routine, then executes a series of conditional function pointers stored in global memory.
- `nop_padding_sequence_2`: This routine is empty and performs no operations.
- `close_file_handle`: The routine validates an index against a table, checks a flag, and if valid, calls a library function to close a resource and clears the entry in the table.

### Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bcdfp

- `clear_and_init_memory`: Fills a memory area with zeros and sets a global variable to a specific value before calling another routine.
- `open_graphics_intuition_libraries`: Opens graphics.library and intuition.library, checks for failure using referenced text strings, and calls routines to wait for library calls and execute cleanup callbacks.
- `initialize_data_block`: Calls a routine to initialize a data structure, then retrieves a pointer from a global variable.
- `process_linked_list`: Traverses a linked list of structures using an index, calling a routine to process each element and checking specific byte values to trigger additional calls.
- `initialize_entity_structures`: Iterates four times to initialize distinct data structures, setting specific byte flags, clearing arrays, and copying configuration values from a global table.
- `initialize_entity_arrays`: Iterates four times to populate multiple global arrays with calculated values and increments specific byte fields.
- `serialize_character_data`: Copies character configuration blocks, including linked list entries and attribute tables, from memory buffers into a file handle.
- `wait_for_library_call_2`: Calls a function pointer from a global structure repeatedly until a second function pointer from the same structure returns a non-zero value.
- `initialize_library_call_state_zero`: Sets a global state variable to zero and invokes a function pointer from a global structure.
- `initialize_library_call_state_two`: Sets a global state variable to two and invokes a function pointer from a global structure.
- `initialize_library_call_state_eight`: Sets a global state variable to eight and invokes a function pointer from a global structure.
- `initialize_library_call_state_three`: Sets a global state variable to three and stores a parameter in a global variable.
- `initialize_library_call_state_seven`: Sets a global state variable to seven and stores a parameter in a global variable.
- `initialize_library_call_state_ten`: Sets a global state variable to ten and stores two parameters in global variables.
- `initialize_file_system_state`: Initializes multiple data structures and arrays, performs string operations, and invokes library calls to set up file system or disk-related states.
- `manage_disk_access_and_saves`: Uses DOS library calls to lock and unlock the game disk, checks disk status, and manages dungeon save files.
- `dispatch_sprite_rendering_2`: The routine calculates sprite offsets, checks range overlaps in a global table, and dispatches to specific rendering routines based on switch values, including text rendering and queue management.
- `setup_sprite_rendering_loop`: Iterates through loops to calculate sprite offsets, configure rendering parameters, and invoke sprite setup and rendering routines for multiple entities.
- `loop_calculate_sprite_offsets`: Iterates nine times, calling a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `call_sprite_offset_calculator`: Calls a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `calculate_sprite_offset_value`: Computes a memory offset based on a register value and a constant, stores the result in a global structure, and calls helper routines for sprite processing.
- `configure_sprite_rendering_params`: Sets specific word-sized values in registers, calls routines to calculate sprite offsets and setup rendering, and invokes text rendering and sprite configuration helpers.
- `dispatch_sprite_rendering`: Calculates sprite offsets and checks range overlaps in a global table, then dispatches to specific rendering routines based on a switch value derived from register calculations.
- `check_range_overlap`: Iterates through ten entries in a global table, checking if a passed parameter falls within specific ranges defined by the table entries.
- `check_and_update_sprite_condition`: Checks values in a global table indexed by parameters, increments or decrements specific fields if conditions are met, and calls helper routines for further processing.
- `check_sprite_condition`: Reads a value from a global table indexed by a parameter, divides it by ten, and branches to different helper routines based on whether the result is zero.
- `render_text_string_2`: Calculates string length and renders text by copying character data from global tables into a buffer, clearing the buffer beforehand.
- `clear_string_buffer_at_offset`: Determines the length of a string and clears specific bytes in a memory area based on conditional checks.
- `clear_sprite_data_buffer`: Reads bytes from a global location and clears specific offsets in a memory area based on conditional checks before invoking other routines.
- `render_text_string_sequence`: Clears a memory area, then calls a routine to copy data to a buffer, another to copy a sequence of words from a global area, and finally a routine to process a header structure.
- `write_character_to_buffer`: Calculates a destination offset using a base address, a multiplier, and a length, then stores a byte and a null terminator at that location.
- `search_short_table`: Iterates through a global array of 26 short integers, comparing each against a passed parameter.
- `execute_indirect_function`: loads a pointer from a global variable and executes the function located at that address.
- `allocate_memory_block`: Allocates a block of memory using the Exec library and returns the pointer.
- `initialize_graphics_library`: Allocates memory, opens the graphics library, and populates a data structure with pointers and values from global variables and library calls, returning a status code.
- `process_integer_list`: Iterates through a list of integers until a terminator value is found, invoking a library function for each entry.
- `process_list_with_lib`: Calls a library function, then iterates through a list of integers until a value of -1 is found, calling another library function for each entry.
- `nop_padding`: This routine consists of four instructions that perform no operations.
- `cleanup_allocated_memory`: Iterates through tables of function pointers and global variables to invoke cleanup routines, then frees allocated memory blocks and closes libraries.
- `initialize_game_state_structures`: Writes values to structure fields, touches the CIAA_ICR hardware register, and calls routines to build copper lists, copy data blocks, and initialize input and player states.
- `configure_cia_registers`: Writes specific values to CIAA control and interrupt registers and updates global status flags and memory offsets.
- `wait_and_set_flag`: Polls a bit in a global status word until it is set, then sets a corresponding bit in another global word and returns the status value.
- `repeat_wait_flag`: Repeats a polling and flag-setting operation a number of times determined by an input parameter, waiting for a specific bit in a global status word before setting a bit in another global word.
- `clear_memory_loop`: Calculates a count based on a global variable and an input parameter, then repeatedly clears a byte at a specific offset in a global structure, waiting for it to become non-zero before decrementing the count.
- `call_copper_initialization`: Calls a routine that constructs a sequence of graphics hardware commands in memory using data from global variables.
- `initialize_copper_list`: Constructs a sequence of graphics hardware commands in memory by writing control words and color data from global variables.
- `copy_block_data`: Copies 17 words from a source address derived from a global pointer to a destination buffer, then copies another 17 words from the end of the source block to a separate destination offset.
- `add_offset_to_pointer`: Returns the value of a global pointer variable plus a constant offset of 96.
- `store_word_to_global`: Stores a word from a register into a fixed offset of a global variable.
- `initialize_buffer_values`: Checks a condition and either clears a buffer with a specific constant or fills a series of memory locations with another constant value.
- `copy_global_to_buffer`: Stores a value to a local offset and copies a sequence of words from a global variable area into a buffer structure.
- `toggle_and_update_state_structure`: Toggles a byte flag and increments a short value within a structure.
- `execute_indirect_jump`: Performs an indirect jump to an address contained within a jump table.
- `update_status_flags`: Masks and stores a value into a structure field at one offset, and sets a byte flag to 1 at another offset within a different structure.
- `set_offset_value`: Writes the value 8 to a specific offset in a data structure and returns a combined register value.
- `execute_indirect_jump_2`: Performs an indirect jump to an address retrieved from a jump table.
- `update_state_and_jump`: Decrements a counter, potentially flips a bitmask or copies data from a global area, and performs an indirect jump.
- `copy_data_and_jump`: Copies a sequence of words from a global variable area into a buffer and performs an indirect jump.
- `copy_data_and_initialize_table`: Copies a block of data from one location to another and then populates a table with specific byte patterns.
- `initialize_input_state`: Resets input counters and queue entries, reads hardware status bytes, and initializes local structure fields with specific constant values.
- `update_pointer_and_value`: Increments a pointer stored in a structure and updates a value at a specific offset within another structure.
- `process_input_and_jump`: Processes keyboard input to update a structure and performs an indirect jump using a jump table.
- `update_input_state_and_dispatch`: Compares current input values against stored state and iterates through a table of range-based entries to detect matches.
- `dispatch_input_event`: Iterates through a table of entries to find a match based on range comparisons and then executes an indirect jump.
- `search_range_table`: Iterates through a table of entries to find a match based on range comparisons.
- `process_input_and_dispatch`: Iterates through a table of entries to find a match based on range comparisons, then updates a global state and executes an indirect jump.
- `search_input_table_and_dispatch`: Iterates through a table of entries to find a match based on multiple range comparisons.
- `process_input_and_jump_3`: Stores two bytes into a global structure, decrements a byte value if it is positive and non-zero, and performs an indirect jump via a jump table.
- `process_input_and_append`: Processes keyboard input, updates a counter with a wrap-around check, and performs an indirect jump using a jump table.
- `process_keyboard_input`: Manipulates a global byte based on bitwise operations and updates a bitmask in a structure using values derived from the keyboard serial register.
- `set_bit_in_byte`: Sets a specific bit within a byte at a fixed offset of a structure using a bit index from a register.
- `read_byte_from_global`: Returns the byte value located at a fixed offset from a global pointer variable.
- `initialize_structure_fields`: Sets several byte and word fields within a structure to zero or specific constants, and then clears a block of 335 words pointed to by a pointer within that structure.
- `update_circular_buffer_and_copy_data_2`: Decrements a counter value and, if it reaches zero, performs a series of non-contiguous memory writes to a destination buffer based on a source pointer.
- `update_circular_buffer_and_copy_data`: Decrements a byte value and, if it reaches a specific threshold, performs a series of non-contiguous memory writes to a destination buffer using an offset.
- `increment_counter_and_copy_block`: Increments a byte in a structure and copies a block of data from a global table to a destination pointed to by a structure.
- `read_circular_buffer_byte`: Reads a byte from a circular buffer using a head index, decrements the index with wrapping, and returns the shifted byte value.
- `decrement_index_and_calculate_offset`: Decrements a value in a structure if it differs from another value and returns a calculated integer based on a byte from a table.
- `clear_byte_in_structure`: Sets a byte at a fixed offset within a structure to zero.
- `write_circular_buffer_byte`: Writes a byte into a circular buffer at the current tail index, decrements the index with wrapping, and processes a string until a null terminator is found.
- `store_byte_and_decrement_index`: Stores a byte into a table at an index specified by a structure and decrements that index.
- `store_word_in_structure`: Stores a word from a register into a fixed offset within a structure.
- `decrement_table_elements`: Iterates through a table of ten words and decrements each non-zero value by one.
- `update_input_state`: Initializes a structure with specific constant values for coordinates and flags, rotates a byte counter, and stores word values into indexed fields.
- `update_and_clamp_coordinates`: Updates a pair of short values by either adding them to existing values or performing a scaled calculation based on a sign-dependent multiplier, then clamps the resulting values within specified minimum and maximum bounds.
- `clamp_and_store_values`: Clamps two values between specified minimum and maximum bounds stored in memory and writes the results to a different location in a data structure.
- `process_input_and_jump_2`: Stores two words from registers into a table and performs an indirect jump using a jump table.
- `clamp_coordinate_values`: Calculates new values by adding pairs of shorts and clamps them between specified minimum and maximum bounds stored in a structure.
- `clear_structure_fields`: Sets several specific offsets within two different structures to zero.
- `calculate_signed_differences_2`: Computes the signed difference between two sets of byte values stored in a structure and a source.
- `calculate_signed_differences`: Computes two signed values based on the subtraction of input registers and a sign flag.
- `return_immediately`: The routine contains instructions but performs no data manipulation or hardware access before returning control to the caller.
- `update_input_offset`: Decrements a byte counter with a lower bound check, then calls another routine to compute values.
- `process_secondary_input`: Decrements a byte counter with a lower bound check, then calls another routine to compute values.
- `compute_input_flags`: Calculates a composite value by adding input registers to global offsets, checking specific bit conditions, and combining the results into a structured return value.
- `check_button_state`: Reads a global variable representing hardware status and returns a boolean indicating whether a specific bit is cleared.
- `store_mouse_data`: Writes byte values from registers and memory locations into specific offsets of a global structure, while managing interrupt enable flags based on a condition.
- `store_word_data`: Stores two word-sized values from registers into specific offsets of a global structure and updates a global flag based on a memory condition.
- `store_unreached_words`: Stores two word-sized values from registers into specific offsets of a global structure, though this code path is not reached during normal execution.
- `manage_queue_operations`: Checks a state byte and a counter; if active, it waits for a flag to clear while decrementing a value.
- `clear_queue_entries`: Resets three short counters to zero and iterates through a table of word pairs, setting each pair to zero.
- `set_global_word`: Stores the value one into a word-sized offset of a global structure.
- `clear_global_word`: Stores the value zero into a word-sized offset of a global structure.
- `mask_and_check_bit`: Masks an input word with a constant, stores it in a global structure, conditionally sets a byte to zero if the lowest bit is clear, and returns the lowest bit.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `append_to_buffer_list_2`: Stores a pointer into a table indexed by a scaled global counter, increments the counter with a wrap-around check, and writes four words of data.
- `append_to_buffer_list`: Stores a pointer into a table indexed by a scaled global counter, increments the counter with a wrap-around check, and writes four words of data to the pointed-to buffer.
- `calculate_scaled_value`: Multiplies a register value by a global word, divides the result by a constant, ensures the outcome is at least one, and stores the byte result to a global variable.
- `transform_linked_list`: Iterates through a linked list of short values, copying and transforming them into a contiguous buffer.
- `render_text_string`: Clears a buffer and copies character data from two global tables based on input string characters.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `initialize_struct_fields`: Sets multiple fields within two global structures to specific constant values, including pointers to offsets within the structure and a reference to a global variable.
- `process_hunk_header`: Reads six words from a pointer stored in a global structure, copies them to other fields in the structure, toggles a status byte, and swaps two pointer fields based on that status.
- `call_sprite_setup_routine`: The routine calls a helper to configure sprite rendering parameters and then invokes an empty stub.
- `calculate_sprite_offset_and_call_setup`: Calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `scan_sprite_stripes`: Iterates through a table of words using a pointer, calling a rendering routine for each entry until a negative value is encountered.
- `render_sprite_stripes`: Configures a graphics structure with source and destination coordinates derived from input parameters.
- `setup_sprite_rendering`: Stores three input parameters into global memory locations and then invokes two rendering routines.
- `return_immediately_3`: performs no operations before returning to its caller.
- `update_sprite_buffer_coordinates`: Calculates and stores a set of coordinate offsets and dimensions into a data structure.
- `update_sprite_clipping_bounds`: Calculates and stores the memory offsets and bounding box coordinates for a graphical object.
- `calculate_sprite_dimensions`: Computes clipping bounds and memory offsets for a graphical object based on its position and size parameters.
- `wait_and_update_nibbles`: Waits for specific byte flags at offsets 1112 and 1113 from a base pointer to become non-zero, then calls helper routines to decrement or increment nibble values in tables.
- `decrement_nibble_values_2`: Iterates through a table of 16-bit words, decrementing each non-zero nibble by one.
- `increment_nibble_if_different`: Compares nibbles of words in a destination table against corresponding words in a source table.
- `clear_pointer_arrays`: Retrieves a pointer to an array of pointers from a global variable.
- `execute_bytecode_instructions`: Iterates through a table of instruction opcodes using a decrementing index, performing specific memory writes and pointer arithmetic based on the instruction type.
- `decompress_data_stream`: Iterates through a data structure using a decrementing index to perform conditional pointer arithmetic and memory writes.
- `process_compressed_data_stream`: Iterates through a data structure using a decrementing index to perform conditional pointer arithmetic and memory writes.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_table_entry`: Stores a value, a short integer, and a pointer into a table at an offset determined by the provided parameters.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `write_jump_table_entry`: Writes a constant value of 2, a 16-bit parameter, and a calculated code pointer into a global table at an index determined by the second argument.
- `format_string_output`: Calls a routine to parse a format string and convert arguments into a formatted output buffer.
- `copy_string_buffer`: Copies bytes from a source pointer to a destination pointer in a loop based on a count parameter.
- `return_from_subroutine_3`: This routine performs an immediate return from a subroutine.
- `parse_icon_library_data`: Opens the icon library, parses string tokens, and processes data structures using DOS library calls for input and output.
- `parse_string_tokens_2`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `tokenize_string_into_array_2`: Parses a source string into individual tokens by splitting on whitespace or handling quoted substrings.
- `append_string_with_limit_2`: Finds the null terminator of a destination string and copies characters from a source string into that location.
- `append_string_with_limit_3`: Finds the null terminator of the first string and begins copying characters from the second string into that position.
- `initialize_window_data`: Opens a file and uses icon library functions to retrieve data from a structure, storing the results into a passed parameter.
- `concatenate_strings`: The routine finds the end of the first string and then copies the contents of the second string onto the end of the first.
- `copy_string_to_null`: Locates the null terminator of the first string parameter by incrementing a pointer until a zero byte is found.
- `copy_string_or_pad_2`: Copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `allocate_and_initialize_object`: The routine uses a library to allocate memory for a structure and populates its fields with provided parameters and metadata.
- `initialize_data_structure`: Sets specific bytes and words within a data structure pointed to by an argument, conditionally calls a function from a global table, and invokes two other functions from that table with a register value.
- `parse_formatted_string`: Parses a string containing control characters and numeric values into a destination buffer.
- `get_string_length`: Iterates through a null-terminated byte string to determine its length.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `iterate_node_list`: Loops through a range of memory addresses, calling another routine at each step with an incrementing offset.
- `process_recursive_buffer_node`: Manages a recursive data structure by performing bounds checking, updating internal pointers, and writing bytes to memory locations.
- `clear_and_format_linked_list`: Recursively traverses a linked structure, converts a numeric value within a node into a five-character ASCII string, and zeros out all fields of the structure.
- `clear_linked_list_entry`: Recursively traverses a structure, converts a value to a five-character string, and zeros out the structure fields.
- `return_zero`: This routine is empty and simply returns zero to the caller.
- `traverse_linked_list`: Recursively searches through a linked list of structures by checking specific bitmask flags and pointer validity.
- `traverse_entry_table`: Iterates through a table of entries by incrementing a pointer, checking specific bit flags within each entry, and performing a recursive call if certain conditions are met.
- `search_table_entries`: Iterates through a table of entries, checking specific bit flags and calling another routine until a condition is met or the end of the table is reached.
- `execute_recursive_call`: This routine performs a single call to another routine and then returns.
- `empty_stub_routine`: The routine consists of six instructions that perform no observable work and end with a return.
- `return_absolute_division`: Calls another routine and returns a 64-bit value where the high word is the absolute value of the result in D1.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `allocate_and_validate_node`: Allocates memory for a linked list node and validates an index against bounds.
- `remove_linked_list_node`: Searches a linked list for a specific node and removes it by updating the preceding pointer.
- `traverse_linked_list_and_call`: Iterates through a linked list until a specific condition is met or a null pointer is encountered.
- `store_and_call_function`: Stores a value from a pointer into a global variable or another location and then calls a function via a pointer.
- `store_and_call_library`: Stores a value from a pointer into a global variable and then calls a function from a library.
- `iterate_linked_list`: Traverses a linked list of pointers and executes a function call for each element, finally clearing a global pointer.
- `iterate_and_call_function`: Traverses a linked list of pointers and executes a function call for each element.
- `insert_node_into_list`: Checks if a provided pointer is non-zero, then inserts a new node into a linked list by updating pointers and storing the input parameter within the new element.
- `copy_string_simple_2`: Copies characters from a source string to a destination buffer until the null terminator is copied.
- `check_interactive_status`: Validates an index against a table and calls a function to determine if the system is in an interactive state.
- `perform_file_seek`: Validates an index against a global table and performs a seek operation using a library function.
- `delete_file_and_check_error`: Attempts to delete a file and stores the result in a global variable, returning a specific value based on whether the operation succeeded.
- `return_zero_2`: This routine is empty and simply returns a value of zero to the caller.
- `perform_file_write`: Validates an index and checks specific bit flags in a table before performing a write operation via a library function.
- `check_library_status`: Calls a library function and checks a specific bit in the return value.
- `output_and_cleanup_callbacks`: Writes data to the standard output using library calls.
- `execute_cleanup_callbacks_2`: Iterates through a list of function pointers stored in a global structure, calling each one, and then executes a final cleanup function if one is present.
- `execute_cleanup_callbacks`: The routine iterates through a count stored in a global structure, calling a validation routine for each index.
- `do_nothing`: the routine is empty and performs no operations.
- `close_resource_entry`: Checks if a provided index is valid within a table and then calls a function to close a resource.

### Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bcdfq

- `main_program_entry`: Opens system libraries, initializes global variables, and enters a main loop that processes graphics, input, and hardware DMA channels, waiting for conditions before executing further routines.
- `check_cia_port_bit`: Reads the CIAA parallel port register and checks if a specific bit associated with joystick or mouse fire buttons is clear.
- `check_memory_flag`: Checks if a specific bit in a global memory variable is clear.
- `wait_for_condition`: Polls two adjacent short values in a structure, returning immediately if the first is zero, otherwise waiting until the second becomes non-zero.
- `process_graphics_loop`: Executes a loop that disables interrupts, waits for the top of frame, processes a table of graphics objects by updating their properties, and re-enables interrupts.
- `open_file_resource`: Calls a function pointer from a global structure and returns true if the result is zero.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `copy_block_words`: Copies 3000 words from a source pointer to a destination pointer, where the source is offset by 12000 words from the destination.
- `clear_and_copy_block`: Calls a function pointer from a global structure and then copies 8000 words from a source buffer to a destination buffer.
- `loop_call_until_flag`: Iterates through a table of short values, calling a function pointer from a global structure for each non-zero entry until a flag in a global structure becomes non-zero.
- `execute_table_callbacks`: Iterates through a table of short values, calling a function pointer from a global structure for each non-zero entry.
- `execute_table_callbacks_2`: Iterates through a table of short values, calling a function pointer from a global structure for each non-zero entry.
- `call_function_pointer`: Calls a single function pointer located at a negative offset from a base address stored in a global structure.
- `loop_decrement_nibbles`: The routine executes a countdown loop from fifteen to zero.
- `decrement_bcd_nibbles`: Iterates through a list of words, decrementing each of the three non-zero nibble components by one.
- `loop_increment_nibbles`: The routine executes a countdown loop from fifteen to zero.
- `increment_mismatched_nibbles`: Iterates through two lists of words, incrementing the individual nibble components of the first list if they differ from the corresponding components in the second list.
- `blit_multi_source_blocks`: Configures and executes multiple blitter operations using hardware registers, setting up source and destination pointers, sizes, and masks.
- `set_dma_channel_mask`: Masks the lower six bits of an input byte.
- `loop_clear_global_flag`: The routine executes a countdown loop from fifteen to zero, calling another routine in each iteration.
- `update_timer_and_dma`: Updates a timer structure with calculated values and modifies a DMA control register.
- `update_timer_structure_2`: Updates a structure with values derived from a calculation, including storing a global variable address and various register-based values into specific offsets.
- `update_timer_structure`: The routine stores several values into a structure, including a calculation involving a register and a value from a register multiplied by two.
- `update_cia_timer_values`: Stores register values into specific offsets of a structure and a global variable, likely updating CIA timer registers or related control structures for hardware timing or interrupt handling.
- `enter_infinite_loop`: The routine contains no executable instructions and is effectively empty, immediately entering an infinite loop.
- `conditional_pointer_copy`: The routine checks a bit in the input register and a field in a structure; if both conditions are met, it copies a pointer and a 16-bit value from the structure to a destination pointed to by another field in the structure.
- `copy_conditional_pointer`: The routine checks a global variable and then performs several calls.
- `update_conditional_data_list_4`: Iterates through a linked list of data structures to perform conditional byte updates and bitmask modifications.
- `process_conditional_data_updates_4`: Iterates through a linked list of data structures to perform conditional byte updates and bitmask modifications.
- `process_conditional_data_updates_3`: Iterates through a linked list of data structures to perform conditional byte updates and bitmask modifications.
- `process_conditional_data_updates`: Iterates through a linked list of data structures to perform conditional byte updates and bitmask modifications.
- `update_conditional_data_list_elements`: Iterates through a linked list of data structures to perform conditional byte updates and bitmask modifications.
- `apply_bitmask_to_data_list`: Iterates through a linked list of data structures to perform conditional byte updates and bitmask modifications.
- `increment_global_counter`: Increments a global byte variable by one.
- `process_control_character_updates_2`: Evaluates a byte as a control character to perform specific data manipulations, such as scaling values, updating bitmasks, or modifying buffers.
- `process_control_character_updates`: Evaluates a byte as a control character to perform specific data manipulations, such as scaling values, updating bitmasks, or modifying buffer indices.
- `update_conditional_state_data`: Processes a series of conditional updates by iterating through a linked list of data structures.
- `process_conditional_data_updates_2`: Iterates through a linked list of data structures to perform conditional byte updates and bitmask modifications.
- `update_conditional_data_list_3`: Iterates through a linked list of data structures to perform conditional byte updates and bitmask modifications.
- `update_conditional_data_list_2_2`: Iterates through a linked list of data structures to perform conditional byte updates and bitmask modifications.
- `update_conditional_data_list_2`: Iterates through a linked list of data structures to perform conditional byte updates and bitmask modifications.
- `update_conditional_data_list`: Iterates through a linked list of data structures to perform conditional byte updates and bitmask modifications.
- `process_conditional_data_list`: Iterates through a list of data, performing conditional updates to bytes and potentially setting a global bitmask before calling a function from a table.
- `lookup_or_divide_table_entry`: The routine is a single instruction that performs a lookup or division based on an input value and stores the result in two global byte variables.
- `fetch_table_or_divide_value`: Retrieves a word from a table using an index if the index is small, or performs a division if the index is large.
- `lookup_or_divide_table_entry_2`: Checks if the input index is within bounds to read a 16-bit value from a global table; otherwise, it divides a global constant by the input.
- `tiny_ori_2`: Small routine of 1 instruction that merges 240 into d7.
- `store_global_and_return_offset`: It stores the address of a global variable into a slot indexed by an uninitialized register and returns the sum of the value pointed to by the input pointer and the input pointer plus six.
- `add_to_pointer_if_nonzero`: It reads the value pointed to by the pointer and, if that value is non-zero, adds the input value to it and stores the result back through the pointer.
- `adjust_pointer_offsets`: Runs a helper routine in a loop until it reports completion, then walks an array of pointers adding a base address to each entry when a specific field is negative.
- `scale_palette_from_source`: The routine reads a scaling byte from a structure pointed to by the argument and multiplies a 16-byte source table by that byte, storing the scaled 16-bit results into a global variable table.
- `process_resource_status`: Checks a global variable and performs bitwise operations on a memory location based on values read from a structure.
- `check_ciab_and_call_handler`: Checks a global variable and calls a handler if non-zero.
- `clear_flag_and_call_handler`: Clears the least-significant bit of a byte stored through a pointer and then calls a handler routine stored in a global variable.
- `open_ciab_resource`: Attempts to open a resource and, if successful, initializes several global pointers and a status variable based on the resource handle.
- `close_ciab_resource`: Checks if a global resource base pointer is non-zero, and if so, clears the pointer and calls a function to close the resource.

### Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bcdft

- `initialize_program_environment`: Performs a sequence of subroutine calls and frees memory using a system library.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.

### Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 2 of 3).adf:bcdfu

- `initialize_game_environment`: Opens the DOS, graphics, and intuition libraries, brings the screen to the front, and disables DMA channels.
- `iterate_null_terminated_string`: Iterates through a null-terminated string by calling another routine for each character.
- `loop_copy_tile_data`: The routine iterates through a null-terminated string.
- `copy_tile_data_block`: Copies eight bytes from a source buffer to five destination locations spaced by 8000 bytes, using offsets derived from input registers.
- `execute_multiple_subroutines`: Performs several calls to another routine and then stores a zero byte at an offset from a value held in a register.
- `execute_six_calls`: The routine calls another routine six times in succession.
- `check_cia_port_bit_2`: Checks if a specific bit in a global variable is clear.
- `check_potgor_bit_two`: Reads the POTGOR register and returns true if bit two is clear.
- `clear_main_buffer`: Sets four consecutive longwords to zero in a loop, repeating the operation 6000 times to clear a large memory block.
- `clear_offset_buffer`: Sets four consecutive longwords to zero in a loop, repeating the operation 500 times on a memory block offset by 32000 bytes.
- `multiply_and_store_result`: Multiplies a short value from a register by a short value from a table and stores the result in another location in that table.
- `wait_for_non_zero_value`: Enters a loop that continues as long as a specific short value in a table is not zero.
- `wait_for_flag_or_zero`: Loops until a primary global variable becomes zero or a secondary global variable becomes non-zero.
- `update_graphics_object_properties`: Disables interrupts, waits for the top of frame, and iterates through a table to decrement a counter and update a value in a buffer based on a byte and the decremented counter.
- `wait_for_ram_flag`: Polls a byte at a fixed offset in Chip RAM until it equals the value 15.
- `open_file_resource_2`: It returns true if the open operation failed.
- `close_file_resource`: Calls the DOS library to close a file handle.
- `decode_rle_chunk_one`: Invokes a function pointer from a global structure and decodes run-length encoded data from a source to a destination buffer.
- `call_dos_read_and_recurse`: Calls a function via a pointer located at an offset from a value in a register and then calls itself recursively.
- `read_file_data`: Calls the DOS library to read data from the currently open file handle into a buffer.
- `read_and_decode_rle_data`: Invokes the DOS Read library function to fetch data, then passes the buffer to a routine that decodes run-length encoded information.
- `call_dos_read`: Calls a function pointer located at a specific offset within a structure pointed to by a register.
- `shift_buffer_data`: Copies a block of 32-bit words from a higher memory address to a lower address, overlapping the source and destination.
- `decode_and_call_global`: Calls a function pointer stored in a global variable, then invokes a routine to decode run-length encoded data.
- `invoke_indirect_function`: Fetches a function pointer from an offset within a structure pointed to by a register and executes it.
- `clear_buffer_region`: Sets a large block of 32-bit words to zero in a specific memory region.
- `decode_rle_data`: Decodes run-length encoded data from a source pointer to a destination pointer by handling literal copies and repeated byte expansions.
- `loop_decrement_nibbles_2`: Executes a countdown loop from fifteen to zero, calling two function pointers and a routine that decrements nibble values in an array before clearing a word-sized field.
- `decrement_bignum_nibbles`: Iterates through a list of words, decrementing each of the three non-zero nibble components by one.
- `call_load_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls LoadRGB4 and returns its result to the caller.
- `loop_increment_nibbles_2`: Executes a countdown loop from fifteen to zero, calling two function pointers and a routine that increments nibble values on mismatch before setting a word-sized field to one.
- `execute_nibble_mismatch_countdown`: Executes a loop from fifteen down to zero, calling two function pointers and a routine that increments nibble values on mismatch during each iteration.
- `increment_mismatched_nibbles_2`: Iterates through two lists of words, incrementing the nibble components of the first list if they differ from the corresponding nibbles in the second list.
- `decode_nibble_differences`: Iterates through two arrays of words, comparing corresponding 4-bit nibbles and decrementing the destination nibble if the source nibble is greater.
- `initialize_graphics_buffers`: Initializes a memory block by setting words to a specific value, then calls routines to clear buffers, decode data, load graphics, and process differences.
- `calculate_length_and_check_bit`: Calculates the length of a zero-terminated byte string and checks the state of bit two in a hardware register.
- `blit_copper_list_segments_2`: Owns the Blitter and iterates through a table of offsets to configure Blitter registers for multiple copy operations, waiting for the hardware.
- `blit_copper_list_segments`: Owns the Blitter and iterates through a table of offsets to configure Blitter registers for multiple copy operations, waiting for the hardware to finish each step.
- `blit_vertical_stripes_2`: Configures the Blitter to copy data from a source buffer to a destination buffer with specific stride and size parameters, looping until a termination condition in a global variable is met.
- `blit_vertical_stripes`: Configures the Blitter to perform five sequential operations to copy data with specific stride and size parameters.
- `copy_tile_data_blocks`: Copies blocks of 32-bit words from a source pointer to a destination pointer using nested loops.
- `blit_conditional_pattern`: Calculates Blitter parameters based on an input value to set source and destination pointers and sizes, then executes a Blitter operation.
- `blit_conditional_pattern_2`: Calculates Blitter parameters based on an input value, setting different source and destination pointers and sizes depending on whether a global flag is set, repeating the operation in a countdown loop.
- `set_dma_channel_mask_2`: Masks the lower six bits of an input byte and, if the result is less than four, stores a bit mask in a global variable.
- `loop_and_clear_flag`: Executes a countdown loop from fifteen to zero, calling another routine in each iteration.
- `update_resource_structure`: Updates a structure with values derived from a pointer and a bitmask, potentially modifying a global variable and setting specific fields based on the input values.
- `update_resource_structure_2`: Updates a structure by storing several 16-bit values and a global variable address into specific offsets based on register inputs.
- `store_structure_fields`: Stores several 16-bit values into specific offsets of a structure based on input registers.
- `update_cia_timer_values_2`: Stores register values into specific offsets of a structure and a global variable.
- `enter_infinite_loop_2`: Enters an infinite loop without executing any functional instructions.
- `conditional_pointer_copy_2`: Checks a bit in the input register and a field in a structure; if both conditions are met, it copies a pointer and a 16-bit value from the structure to a destination pointed to by another field in the structure.
- `copy_conditional_pointer_2`: Checks a global flag and a bit in a returned value, then conditionally copies a pointer and a 16-bit value from a structure to another location.
- `dispatch_subroutine_calls`: Invokes three different routines by retrieving their addresses from a data structure.
- `fetch_table_entry_or_divide`: Compares a word to a constant to determine whether to retrieve a value from a table using an index or to perform a division and store the result.
- `execute_indirect_call`: Loads a subroutine address from a data location and transfers control to it.
- `invoke_subroutine_via_pointer`: Loads a pointer from a data location and calls the routine at that address.
- `update_hardware_parameters_3`: Calls routines to mask bits of an input byte and update hardware control parameters.
- `update_hardware_parameters`: Calls a routine to mask bits of an input byte and another routine to update hardware control parameters.
- `update_hardware_parameters_2`: Calls a routine to update a structure using a pointer and bitmask and interacts with a DMA control register.
- `configure_dma_parameters`: Updates data structures and interacts with a DMA control register by calling two separate subroutines.
- `update_dma_settings`: Updates data structures and interacts with a DMA control register through two subroutines.
- `update_dma_configuration`: This routine calls two subroutines to update data structures and interact with a DMA control register.
- `update_dma_parameters`: Updates a data structure and interacts with a DMA control register while retrieving values from a table.
- `configure_dma_control_2`: Calls routines to update a structure, interact with a DMA control register, and retrieve a value from a table or perform a division.
- `configure_dma_control`: Calls a routine to update a structure and interacts with a DMA control register.
- `update_hardware_control_parameters`: This routine calls a routine to update a structure using a pointer and bitmask, a routine to interact with a DMA control register, and a routine that performs table lookups or division to store a word.
- `modify_dma_control`: A routine of 42 instructions that is accessed via a pointer in data and interacts with the DMACON register.
- `process_conditional_data_list_alt`: Iterates through a list of data to perform conditional byte subtractions and updates a global bitmask before invoking a routine via a table.
- `lookup_and_store_word`: If the input word is less than eleven, it fetches a word from a table using an index; otherwise, it performs a division and stores the result in two bytes.
- `fetch_table_or_divide`: Retrieves a word from a table using an index if the index is small, otherwise performs a division.
- `lookup_or_divide_table_entry_3`: Checks if an input index is within a specific range to fetch a 16-bit value from a global table; otherwise, it divides a global constant by the input.
- `store_global_and_return_offset_2`: Stores the address of a global variable into a slot indexed by an uninitialized register and returns the sum of the value pointed to by the input pointer and the input pointer plus six.
- `store_pointer_and_return_sum`: Stores the address of a global variable into a table at an offset determined by a register.
- `add_to_indirect_value`: Checks if the value at a pointer is non-zero, and if so, adds a value to it and stores the result back at that pointer location.
- `update_pointer_arrays`: Runs a helper routine in a loop until it returns zero, then iterates through a list of pointers to check if a specific field is negative.
- `scale_and_copy_palette`: Scales a sixteen-entry source table by a byte value retrieved from a structure and stores the results in a global table.
- `modify_global_state`: Performs bitwise manipulations on a memory location and updates a global variable using values from a structure.
- `update_resource_flags`: Calls another routine and, if the return value is non-zero, performs a series of bitwise operations on a global variable based on values read from a memory structure.
- `update_resource_state_flags`: Checks if a global variable is non-zero, then clears a bit in a memory location and updates the global variable by modifying specific bits based on a byte read from memory.
- `check_ciab_and_call_handler_2`: Checks if a global variable is non-zero and, if so, calls a handler routine.
- `clear_bit_and_call_handler`: Clears the least-significant bit of a byte at a location pointed to by a pointer and then calls a routine via a handler pointer stored in a global variable.
- `open_ciab_resource_2`: Attempts to open a specific resource and performs a series of checks on the returned pointer and its associated functions to set a global status value.
- `close_ciab_resource_2`: The routine checks if a global resource base pointer is non-zero.
- `empty_routine`: This routine is empty and contains no executable instructions.

### Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_and_call_functions`: Clears a block of memory with zeros using a loop and then executes several function pointers stored in a global structure based on specific conditional checks.
- `initialize_system_libraries`: Attempts to open required Amiga system libraries and verifies their successful loading by checking return values against null.
- `wait_for_library_call_3`: Repeatedly calls a function pointer from a global structure until a second function pointer from the same structure returns a non-zero value.
- `execute_library_callback_sequence`: Sets a global state variable to three and stores a parameter in a global variable before invoking a function pointer.
- `initialize_system_state`: Sets specific values in a structure and performs a loop that modifies a value using bitwise operations based on a memory range.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_structure`: Zeroes out a range of memory in a global structure and conditionally calls a function pointer based on a flag in a global variable.
- `return_from_subroutine_4`: This routine performs a return from a subroutine.
- `initialize_window_structure`: Opens a library and initializes a data structure by setting specific flags and a magic number.
- `parse_string_tokens_3`: Parses a string into individual tokens by skipping whitespace and handling quoted substrings.
- `tokenize_string_into_array_3`: Parses a source string into individual tokens by splitting on whitespace and handling quoted substrings.
- `copy_string_to_buffer_2`: Finds the end of a null-terminated string and copies characters from a source string to that location.
- `append_string_with_limit_5`: Locates the null terminator of the first string and copies characters from the second string onto that position.
- `fetch_window_data`: Calls several library functions to retrieve data and stores the resulting pointer and associated values into a structure.
- `concatenate_strings_2`: Finds the null terminator of the first string and then copies the second string into the buffer starting at that position.
- `copy_string_or_pad_4`: Copies characters from a source pointer to a destination pointer up to a specified count.
- `get_string_length_2`: Iterates through a null-terminated byte string to determine its length.
- `initialize_pointer_structure`: The routine stores the address of the provided pointer into a memory location, increments that address by four, sets a subsequent word to zero, and stores the original address into another location.
- `copy_string_to_buffer_3`: Copies bytes from a source pointer to a destination buffer until a null terminator is encountered.
- `execute_cleanup_callbacks_3`: The routine iterates through a list of function pointers stored in a global structure, calling each one.
- `release_indexed_resources_and_flags`: Recursively walks a per-index loop over a count stored in the global block, invoking itself for each index, then performs a series of conditional teardown steps: it calls library function pointers when their fields are non-zero, stores a saved value into a pointer target, and finishes by invoking the close-out library call before returning the input parameter.
- `nop_padding_sequence_3`: The routine consists of two instructions that perform no operation and do not modify any state.
- `close_file_handle_2`: The routine validates an index against a table, checks if the corresponding entry is non-zero, and if valid, calls a function to close the resource before clearing the entry in the table.
