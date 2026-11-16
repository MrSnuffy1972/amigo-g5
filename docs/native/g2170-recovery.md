# Game 2170 (black crypt (1992)(electronic arts)[cr trsi][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02170). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2717344 top-level bytes in 52 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 20 of 20 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 761 of 761 named (0 author symbols), 761 of 761 described (119 by rule, 470 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1094 of 1094 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 144169 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 19647 of 19649 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2025689 |
| code | 62352 |
| data | 449827 |
| padding | 12564 |
| unused | 166912 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 458 | 12 |
| Stack | 2 | 117 | 2 |
| avail | 11 | 376 | 12 |
| runback | 3 | 709 | 39 |
| endcli | 2 | 182 | 3 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:TRSI! | 1 | 116 | 3 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:InstallCrypt | 3 | 3235 | 57 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:BlackCrypt | 3 | 2384 | 104 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bcdfp | 3 | 6602 | 223 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bcdfq | 2 | 1120 | 70 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bcdft | 5 | 160 | 15 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 2 of 3).adf:bcdfu | 2 | 1595 | 97 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 185 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1696 | 114 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 2 of 3).adf:bcdfx (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 6 first-execution byte records contradict the logged opcode word (first an address): bad captures, the opcode word is used
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 31 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 217 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- black crypt (1992)(electronic arts)[cr trsi][a](disk 1 of 3)/Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:TRSI!: CrunchMania Addr (xfd) stream found by ancient scan (13088 -> 32446 bytes) but only 0 of 7 sampled windows are in reference RAM: not used
- black crypt (1992)(electronic arts)[cr trsi][a](disk 1 of 3)/Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bcdft: CrunchMania Normal (xfd) stream found by ancient scan (83200 -> 180004 bytes) but only 0 of 86 sampled windows are in reference RAM: not used
- black crypt (1992)(electronic arts)[cr trsi][a](disk 2 of 3)/Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 2 of 3).adf:bcdfx: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
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
- `initialize_entity_structure`: Initializes a structure by setting multiple offsets to zero, fetching values from tables based on specific indices, and performing a loop to copy a sequence of bytes from one memory location to another.
- `update_entity_state_from_table`: Fetches a byte from a table using a value at offset 0 of a structure, stores it at offset 32 of the structure, and calculates a new value at offset 36.
- `copy_byte_from_table_to_entity`: Calculates a destination offset in a structure using a value from offset 12 and a constant, then copies a byte from a table in a0 to that structure using a second calculated offset based on offset 24 and a value from offset 20.
- `update_entity_state_and_call_handler`: Updates several fields in a structure pointed to by a1 using values from a0, including setting a byte to 10 and copying longwords between offsets.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `frag_tst_bne_bra`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not zero, then jumps to another point in the code.
- `modify_status_bits`: Performs a bitwise AND between a long value at offset 48 of a structure in a1 and a constant, and if the result is non-zero, sets bits in that same location; otherwise it branches to another location.
- `update_entity_pointer_and_call_handler`: Copies a long value from offset 64 to offset 108 of a structure in a1, prepares arguments in d1, d2, and d3, and calls a routine via a pointer in a5.
- `reset_entity_state_and_update_pointers`: Clears several fields in a structure pointed to by a1 and updates a corresponding entry in a table pointed to by a0 using an index derived from a1.
- `execute_complex_state_updates`: Calls several routines via a pointer in a6 with various parameters, updates a long value at offset 128 of a structure in a1, performs conditional decrements on values in a table pointed to by a0, and ends with an indirect jump through a6.

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
- `process_system_state`: Modifies a word in a structure, updates a pointer in a1, calls several functions via a pointer in a5, clears a value in a table, and conditionally decrements a value in a table before performing an indirect jump.

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
- `frag_moveq_jsr_bra`: Short code fragment that loads 1 into d2, then calls delay_and_unload_segment, then jumps to another point in the code.
- `call_routine_with_parameter`: Calls another routine, pushes a word of four onto the stack, pushes a pointer to a location relative to the program counter onto the stack, calls another routine, cleans the stack, and branches to another location.
- `call_routine_with_parameter_2`: Calls another routine, pushes a word of five onto the stack, pushes a pointer to a location relative to the program counter onto the stack, calls another routine, cleans the stack, and branches to another location.
- `process_directory_and_dispatch_event`: Calls a routine to process a DOS directory, stores the result in a structure at offset 24, sets a status value of 6 at offset 20 of that structure, and calls a library function before branching back to a loop that dispatches events based on a status value.
- `dispatch_input_and_update_event_status`: Calls dispatch_input_event_handler and stores the result in a structure at offset 22, then sets a status value of 6 at offset 20 of that same structure before calling an Amiga OS library function and branching to another routine.
- `frag_jsr_move_bra`: Short code fragment that calls delay_and_unload_segment, then calls poll_input_event_loop, then copies d0 to d2, then jumps to another point in the code.
- `reset_and_call_subroutines`: Calls a subroutine, loads a pointer from an offset of a4 into a0, clears a longword at offset 82 of the structure in a0, pushes two parameters onto the stack and calls a subroutine, updates the longword at offset 82 of the structure in a0 with the value 33800, calls another subroutine, copies the word from d0 to d2, adjusts the stack pointer, and branches to another location.
- `process_event_loop_and_shutdown`: Enters a loop that checks a status value in a structure and executes different branches based on its value, including calling routines to unload segments, poll input, or handle disk status.
- `initialize_graphics_and_config`: Opens the graphics and intuition libraries, loads the configuration file, and initializes bitmaps and screen structures for the main game window.
- `execute_shutdown_callbacks_3`: The routine checks multiple global flags and executes corresponding function pointers from library bases.
- `delay_and_unload_segment`: Sets a value in a global variable and calls several indirect function pointers.
- `poll_input_event_loop`: Loops calling an input dispatch routine until a specific condition is met, handling cases for event types 1 through 4 by updating global state variables or invoking other handlers.
- `process_input_and_state`: Calls a routine to handle an input event, then if the result in d2 meets specific criteria involving a global byte, shifts d2 left by 8, adds 16384, stores it in a global word, and calls a routine to update the global state with the value from d0.
- `frag_moveq_bra`: Short code fragment that loads 0 into d2, then jumps to another point in the code.
- `frag_move_pea_jsr_addq_bra`: Short code fragment that copies 4 to the stack, then pushes the address of a location in the code onto the stack, then calls another routine, then adds 6 to a7, then jumps to another point in the code.
- `frag_jsr_moveq_bra`: Short code fragment that calls another routine, then loads 0 into d2, then jumps to another point in the code.
- `dispatch_jump_table`: Compares a word in d0 to 5, and if it is not less than 5, adds d0 to itself and uses the result to jump to an offset in a table.
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
- `copy_menu_tiles`: Calls calculate_length_and_copy_tiles and then calls copy_tiles_until_null six times using various word constants and pointers.
- `copy_menu_tiles_save`: Calls a routine to calculate length and copy tiles, then calls another routine to iterate and copy tiles multiple times using various word constants, and if a global byte is zero, performs one additional tile copy routine before returning.
- `copy_save_menu_tiles`: Calls copy_tiles_until_null four times using various word constants and pointers.
- `copy_menu_tiles_load`: Calls a routine to calculate length and copy tiles, then calls another routine to iterate and copy tiles multiple times using various word constants, and if a global byte is not zero, performs additional tile copy routines using different word constants before returning.
- `populate_menu_text_tiles`: Calls a routine to copy tiles from four different string locations to a menu interface, using specific word constants as parameters for each string.
- `copy_text_tiles`: Calls calculate_length_and_copy_tiles four times with different word constants and pointers to process a set of strings.
- `check_input_event`: Polls an input event queue, checking message types and data fields to determine a return value based on specific conditions.
- `dispatch_input_event_handler`: Polls an input event queue and dispatches execution to different sub-routines based on the event type code.
- `frag_jsr_move_bra_2`: Short code fragment that calls initialize_menu_string_tiles, then copies d0 to d2, then jumps to another point in the code.
- `frag_jsr_move_bra_3`: Short code fragment that calls another routine, then copies d0 to d2, then jumps to another point in the code.
- `frag_jsr_move_bra_4`: Short code fragment that calls another routine, then copies d0 to d2, then jumps to another point in the code.
- `frag_jsr_move_bra_5`: Short code fragment that calls another routine, then copies d0 to d2, then jumps to another point in the code.
- `dispatch_input_event_2`: Calls a routine, saves the result in a register, and branches to another location; if the result is not less than 7, it uses the result to index into a table and jumps to a calculated address.
- `copy_config_tile_bytes_2`: Calls calculate_length_and_copy_tiles and copy_tiles_until_null, then conditionally calls copy_tiles_until_null again based on a byte in a global structure.
- `process_configuration_strings`: Calls copy_tiles_until_null four times, passing a word constant and a pointer to a string for each call to process the configuration menu text.
- `build_keyboard_configuration_string`: Constructs a string by copying several text segments into a local buffer and then enters a loop that reads characters from an input stream.
- `display_intro_text_sequence`: Copies a string from a table to a buffer, calls a routine to display it, and then calls a routine to render tiles based on that string.
- `display_input_prompt_and_loop`: Copies a string from a table to a buffer, calls a routine to display it, and then calls a routine to iterate through tiles.
- `display_input_prompt_and_parse`: Copies a string to a buffer, displays it, and then iterates through a linked list of structures to validate and append input characters to a local buffer until a specific character or length limit is reached.
- `display_input_prompt_and_parse_2`: Copies strings to a buffer, displays them via a tile-based routine, and enters a loop that parses input characters to build a string.
- `display_prompt_and_parse_input`: Displays a string from a table and calls a routine to iterate through tiles, then enters a loop that parses input characters from a linked list of structures to validate them against specific character ranges and appends valid characters to a local buffer.
- `frag_cmpi_bcc_add_move_jmp`: Short code fragment that compares d0 with 6, then branches if the carry flag is clear, then adds d0 to d0, then copies a location in the code to d0, then jumps to another point in the code.
- `parse_input_string_and_validate`: Checks two condition flags and then iterates through a linked list of structures to extract a byte from offset 15 of each structure, using the value to perform a bitwise shift and calling two library functions.
- `frag_moveq_bra_2`: Short code fragment that loads 0 into d4, then jumps to another point in the code.
- `validate_and_store_input_character`: Iterates through a buffer to check if a specific byte already exists, then checks if the byte is a vowel or within a specific range of ASCII values.
- `store_byte_and_call_libraries`: Stores a byte from register d2 into a buffer at an offset determined by d5, increments d5, and then calls two library functions using pointers retrieved from a global structure.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `copy_config_tile_bytes`: If a specific word in d6 is zero, it performs a loop that copies 6 bytes from a source buffer located at an offset from a5 to a destination buffer located at an offset from a4.
- `copy_data_and_check_input`: Copies two longwords from a location offset from a4 to a location offset from a0, calls a routine using a pointer from a6, calls another routine, calls a third routine, and if d3 is zero, calls a fourth routine and branches to another location.
- `frag_jsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `test_and_call_subroutine`: Tests a byte at an offset from a4, and if it is non-zero, pushes a pointer to a location relative to the program counter onto the stack, calls a routine, cleans the stack, and branches to another location.
- `frag_move_pea_jsr_addq_bra_2`: Short code fragment that copies 10 to the stack, then pushes the address of a location in the code onto the stack, then calls copy_tiles_until_null, then adds 6 to a7, then jumps to another point in the code.
- `manage_configuration_file_access`: Attempts to open a configuration file and, if successful, writes data to it using a specific error code; if the operation fails, it executes a sequence of error handling routines and updates a status value.
- `execute_error_handling_sequence`: Pushes a value onto the stack, then calls three consecutive routines before adjusting the stack pointer and branching to another routine.
- `handle_disk_io_error`: Attempts to read a file and, if the result is non-zero, calls three subroutines to process the error status and data before returning.
- `handle_disk_read_error`: Pushes a pointer to a string onto the stack, calls three subroutines to process data and disk status, and then returns via a short branch.
- `frag_jsr_moveq_bra_2`: Short code fragment that calls call_indirect_function_3, then loads 1 into d3, then jumps to another point in the code.
- `frag_cmpi_bcc_add_move_jmp_2`: Short code fragment that compares d0 with 6, then branches if the carry flag is clear, then adds d0 to d0, then copies a location in the code to d0, then jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `copy_two_longwords`: If d3 is equal to 1, copies two longwords from a location offset from a5 to a location offset from a4.
- `check_pointer_validity`: Loads a pointer from offset 8 of a structure in a5, and if the pointer is not zero, moves it to a2 and branches to another routine; otherwise it returns.
- `process_string_tiles_loop`: Iterates through a list of pointers, calling calculate_length_and_copy_tiles and calculate_string_length for each entry.
- `show_click_to_continue_prompt`: Checks a flag in a structure reached through a base register, and if the flag is 1 and a register is non-zero, calls calculate_length_and_copy_tiles, calls several other routines, and then returns.
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
- `copy_string_to_null_2`: Locates the null terminator of the first string parameter by incrementing a pointer until a zero byte is found, then copies the second string to that location.
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
- `process_modified_word`: Loads a word from an offset of a4, modifies it by adding 65535 and masking the lower two bits, pushes the result to the stack, calls three different routines, and then loops back to the start of this routine.
- `process_incremented_word`: Loads a word from an offset of a4, modifies it by adding 1 and masking the lower two bits, pushes the result to the stack, calls three different routines, and then loops back to the start of this routine.
- `frag_jsr_bra_2`: Short code fragment that calls clear_string_buffer_at_offset, then jumps to another point in the code.
- `dispatch_render_commands`: Calls a routine, then performs a series of checks on d0 and d2 to either call render_text_string_2 and process_hunk_header or perform an indirect jump based on values from a structure in a4.
- `setup_sprite_rendering_loop`: Iterates through loops to calculate sprite offsets, configure rendering parameters, and invoke sprite setup and rendering routines for multiple entities.
- `loop_calculate_sprite_offsets`: Iterates nine times, calling a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `call_sprite_offset_calculator`: Calls a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `calculate_sprite_offset_value`: Computes a memory offset based on a register value and a constant, stores the result in a global structure, and calls helper routines for sprite processing.
- `configure_sprite_rendering_params`: Sets specific word-sized values in registers, calls routines to calculate sprite offsets and setup rendering, and invokes text rendering and sprite configuration helpers.
- `dispatch_sprite_rendering`: Calculates sprite offsets and checks range overlaps in a global table, then dispatches to specific rendering routines based on a switch value derived from register calculations.
- `frag_clr_jsr_addq_bra`: Short code fragment that clears the stack, then calls another routine, then adds 2 to a7, then jumps to another point in the code.
- `frag_clr_jsr_addq_bra_2`: Short code fragment that clears the stack, then calls check_and_update_sprite_condition, then adds 2 to a7, then jumps to another point in the code.
- `frag_move_jsr_addq_bra`: Short code fragment that copies 1 to the stack, then calls another routine, then adds 2 to a7, then jumps to another point in the code.
- `frag_move_jsr_addq_bra_2`: Short code fragment that copies 1 to the stack, then calls check_and_update_sprite_condition, then adds 2 to a7, then jumps to another point in the code.
- `frag_move_jsr_addq_bra_3`: Short code fragment that copies 2 to the stack, then calls another routine, then adds 2 to a7, then jumps to another point in the code.
- `frag_move_jsr_addq_bra_4`: Short code fragment that copies 2 to the stack, then calls check_and_update_sprite_condition, then adds 2 to a7, then jumps to another point in the code.
- `frag_move_jsr_addq_bra_5`: Short code fragment that copies 3 to the stack, then calls another routine, then adds 2 to a7, then jumps to another point in the code.
- `frag_move_jsr_addq_bra_6`: Short code fragment that copies 3 to the stack, then calls check_and_update_sprite_condition, then adds 2 to a7, then jumps to another point in the code.
- `frag_move_jsr_addq_bra_7`: Short code fragment that copies 4 to the stack, then calls another routine, then adds 2 to a7, then jumps to another point in the code.
- `dispatch_sprite_logic`: Calls check_and_update_sprite_condition, then branches to a table lookup based on d0, and may call store_word_to_global, process_hunk_header, or return_immediately_3.
- `check_range_overlap`: Iterates through ten entries in a global table, checking if a passed parameter falls within specific ranges defined by the table entries.
- `check_and_update_sprite_condition`: Checks values in a global table indexed by parameters, increments or decrements specific fields if conditions are met, and calls helper routines for further processing.
- `update_sprite_conditions`: Calculates offsets into a table, compares values, and if they differ, decrements one value and increments another in a table before calling check_sprite_condition twice.
- `check_sprite_condition`: Reads a value from a global table indexed by a parameter, divides it by ten, and branches to different helper routines based on whether the result is zero.
- `render_text_string_2`: Calculates string length and renders text by copying character data from global tables into a buffer, clearing the buffer beforehand.
- `clear_string_buffer_at_offset`: Determines the length of a string and clears specific bytes in a memory area based on conditional checks.
- `clear_sprite_data_buffer`: Reads bytes from a global location and clears specific offsets in a memory area based on conditional checks before invoking other routines.
- `render_text_string_sequence`: Clears a memory area, then calls a routine to copy data to a buffer, another to copy a sequence of words from a global area, and finally a routine to process a header structure.
- `write_character_to_buffer`: Calculates a destination offset using a base address, a multiplier, and a length, then stores a byte and a null terminator at that location.
- `search_short_table`: Iterates through a global array of 26 short integers, comparing each against a passed parameter.
- `calculate_string_offset`: Calculates a memory address based on a value from a global variable and d2, calls a routine to find a string length, and then returns.
- `update_text_display_buffer`: Calculates an offset using a value from a global variable and d2, then writes a space to that location; it then calculates a second offset using d2 plus one and clears a byte at that location; finally, it calls three routines and branches back to an earlier instruction.
- `allocate_memory_and_copy`: Copies a byte from a source pointed to by a0 to a destination pointed to by a2, then calls a routine using a pointer from a system library.
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
- `process_data_tables`: Clears a memory block using a loop, calculates a length based on a null-terminated string, copies seven bytes from a table to a destination using a loop, and sets a word at offset 974 of a structure in a5 to 2400.
- `copy_data_and_calculate`: Copies eight words from a source table to a destination table using a loop, sets a word at offset 974 of a structure in a5 to 2400, and stores a product of two values into offset 1084 of that structure.
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
- `negate_and_store_values`: Negates two 16-bit values in d0 and d1, subtracts 12 from each, and stores the results into a table at offsets 162 and 164 from a base address.
- `store_registers_to_table`: Stores the values of d0, d1, a0, and the stack pointer into a table at offsets 168, 170, 172, and 174 from a base address.
- `compare_d0_value`: Compares the word in d0 against the values 1, 2, 3, or 4 and branches to different locations, otherwise it sets d0 to 65535 and returns.
- `frag_moveq_move_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a0 to d0, then returns.
- `frag_moveq_move_rts_2`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a0 to d0, then returns.
- `frag_move_rts`: Short code fragment that copies the memory at an offset from a0 to d0, then returns.
- `frag_move_rts_2`: Short code fragment that copies the memory at an offset from a0 to d0, then returns.
- `update_scaled_value`: Stores a 16-bit word from d0 into offset 178 of a structure, calculates a new value by multiplying d0 by 46 and dividing by 100 into offset 176 of the same structure, and modifies the interrupt enable register based on a bit test of a value from the stack.
- `update_structure_values_2`: Calls another routine, checks a byte at offset 200 of a structure in a5, and if it is not zero, writes a word, a byte, and two words to memory locations pointed to by a0 before decrementing a byte at offset 200 of the structure in a5.
- `transfer_list_node`: Sets a word at offset 154 of a structure in a6 to 16384, checks a word at offset 190 of a structure in a5, and if it is zero, clears two longwords at a0 and returns.
- `transfer_and_clear_list_node`: Decrements a 16-bit value at offset 190 of a structure, traverses a table using a value from offset 188 and offset 182, and if a comparison with 128 is met, copies a 32-bit doubleword from a table to a destination and clears the source entries, otherwise it returns.
- `manage_queue_operations`: Checks a state byte and a counter; if active, it waits for a flag to clear while decrementing a value.
- `clear_queue_entries`: Resets three short counters to zero and iterates through a table of word pairs, setting each pair to zero.
- `set_global_word`: Stores the value one into a word-sized offset of a global structure.
- `clear_global_word`: Stores the value zero into a word-sized offset of a global structure.
- `mask_and_check_bit`: Masks an input word with a constant, stores it in a global structure, conditionally sets a byte to zero if the lowest bit is clear, and returns the lowest bit.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `clear_bits_in_structure`: Clears specific bits in a 16-bit word at offset 194 of a structure by performing a bitwise AND with the inverse of a masked version of d0.
- `set_bits_in_structure`: Sets specific bits in a 16-bit word at offset 194 of a structure by performing a bitwise OR with a masked version of d0.
- `append_to_buffer_list_2`: Stores a pointer into a table indexed by a scaled global counter, increments the counter with a wrap-around check, and writes four words of data.
- `append_to_buffer_list`: Stores a pointer into a table indexed by a scaled global counter, increments the counter with a wrap-around check, and writes four words of data to the pointed-to buffer.
- `calculate_scaled_value`: Multiplies a register value by a global word, divides the result by a constant, ensures the outcome is at least one, and stores the byte result to a global variable.
- `frag_movea_move_rts`: Short code fragment that copies a location in the code to a0, then copies the memory at an offset from a0 to the memory at an offset from a0, then returns.
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
- `frag_movem_bsr_rts`: Short code fragment that saves registers on the stack, then calls render_sprite_stripes, then calls return_from_subroutine_2, then restores saved registers from the stack, then returns.
- `loop_through_data_and_call`: Pushes a2 to the stack, enters a loop that loads a word from a structure, calculates an address, and calls a routine via bsr.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `return_immediately_3`: performs no operations before returning to its caller.
- `update_sprite_buffer_coordinates`: Calculates and stores a set of coordinate offsets and dimensions into a data structure.
- `update_sprite_clipping_bounds`: Calculates and stores the memory offsets and bounding box coordinates for a graphical object.
- `frag_movea_bra`: Short code fragment that copies the memory at an offset from a5 to a0, then jumps to another point in the code.
- `process_and_dispatch_list`: Calculates several values based on a structure, populates a local structure in a6, and enters a loop that updates a6 entries using data from a list before performing an indirect jump to a routine stored at an offset in a structure.
- `populate_and_loop_structure`: Populates a local structure in a6 with values from a structure and a list, then enters a loop that updates a6 entries using data from the list until a counter reaches zero.
- `wait_for_bit_clear`: Loops while bit 0 of a 16-bit word at offset 4 of the structure in a6 is set, then compares a 16-bit word at offset 6 of a6 with d0 after shifting it right by 8 bits.
- `rotate_byte_buffer`: Calculates a sum of three bytes from a table using carry propagation, stores the result in the first byte of the table, and then shifts five bytes from the table to the five preceding bytes in memory.
- `wait_and_update_nibbles`: Waits for specific byte flags at offsets 1112 and 1113 from a base pointer to become non-zero, then calls helper routines to decrement or increment nibble values in tables.
- `decrement_nibble_values_2`: Iterates through a table of 16-bit words, decrementing each non-zero nibble by one.
- `increment_nibble_if_different`: Compares nibbles of words in a destination table against corresponding words in a source table.
- `clear_pointer_arrays`: Retrieves a pointer to an array of pointers from a global variable.
- `execute_bytecode_instructions`: Iterates through a table of instruction opcodes using a decrementing index, performing specific memory writes and pointer arithmetic based on the instruction type.
- `decompress_data_stream`: Iterates through a data structure using a decrementing index to perform conditional pointer arithmetic and memory writes.
- `process_compressed_data_stream`: Iterates through a data structure using a decrementing index to perform conditional pointer arithmetic and memory writes.
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
- `modify_hardware_registers`: Performs several bitwise operations on memory locations at offsets 28, 462, 102, 102, and 110 of structures in a0, a2, and a6, and branches to another routine.
- `frag_add_move_bra`: Short code fragment that adds d4 to the memory at an offset from a5, then copies the memory at an offset from a5 to d5, then jumps to another point in the code.
- `calculate_index_offset`: Tests a byte in d4, and if it is non-zero, divides d3 by 3, swaps the high and low bytes of the result, and if the result is non-zero, performs bitwise operations on d4 and adds a byte from a structure in a5 to d4 before branching to another routine.
- `frag_subq_bne_lsr_add_bra`: Short code fragment that subtracts 1 from d0, then branches if the last result was not zero, then shifts d4 right by 4, then adds the memory at a5 to d4, then jumps to another point in the code.
- `fetch_table_value`: Calculates an index using a byte from a5, a byte from a structure in a4, and a byte from offset 10 of a5, then loads a word from a table pointed to by a1 into d5 and branches to another routine.
- `update_structure_values_3`: Moves a byte from d4 to d1, and if the lower nibble of d1 is non-zero, branches to another routine; otherwise it subtracts d4 from a byte at offset 2 of a structure in a5 and updates d0 and d1 based on the result.
- `clamp_and_store_byte`: Calculates a value by adding a shifted byte to another byte, clamps the result to a maximum of 64, and stores it at offset 2 of a structure pointed to by a5.
- `frag_move_cmp_bge_sub_bra`: Short code fragment that copies the memory at an offset from a5 to d5, then compares d3 with 3, then branches if the comparison was greater or equal, then subtracts d4 from d5, then jumps to another point in the code.
- `check_counter_overflow`: Loads a word from offset 30 of a structure in a5, and if it is not zero, compares a word from offset 18 of the structure to that word, adds a byte from offset 70 of the structure to the word from offset 18, and branches to another routine if the new value is less than the word from offset 30.
- `reset_counter_on_overflow`: Subtracts d4 from d1, compares d1 to d0, and if d1 is not higher than d0, loads a word from offset 30 of a structure in a5, clears that offset, and stores the value in d1 and d5.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `update_structure_values`: If d4 is not zero, it stores d4 at offset 33 of a5, calculates a value using a table lookup and arithmetic based on offsets 32 and 18 of a5, and updates offset 32 of a5 before branching to another routine.
- `frag_cmp_bne_bra`: Short code fragment that compares d4 with 241, then branches if the last result was not zero, then compares d3 with 3, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_cmp_bne_bra_2`: Short code fragment that compares d4 with 242, then branches if the last result was not zero, then compares d3 with 3, then branches if the last result was not zero, then jumps to another point in the code.
- `execute_dma_update`: Compares d4 to 243, and if it matches, checks if bits 1 or 2 are set in d3, then loads several bytes from a structure in a5 and calls another routine via a branch to subroutine using a pointer from offset 12 of a5.
- `clear_ciaa_bit`: Compares d4 to two different values and, if d4 equals 249, clears bit 1 of the CIAA_PRA register before branching to another routine.
- `frag_bset_bra`: Short code fragment that sets bit 1 of a fixed memory address, then jumps to another point in the code.
- `update_pointer_conditionally`: Loads a pointer from offset 20 of a5 into a1, then if d5 is not zero, writes d5 to offset 6 of a1 and a byte from offset 2 of a5 to offset 9 of a1, increments d7, and calls another routine if d7 is less than a constant.
- `lookup_or_divide_table_entry_2`: Checks if the input index is within bounds to read a 16-bit value from a global table; otherwise, it divides a global constant by the input.
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
- `unpack_and_patch_data_block`: Loads three longwords from a table to use as a destination pointer, a source offset, and a length, then performs a bitwise transformation on a word from the source and writes the result to the destination.
- `process_conditional_word_transformation`: Loads the value 14 into register d4 and 5 into d1, then calls another routine; if the carry flag is set, it loads 14 into d1, calls another routine, adds the result in d0 to d4, and branches to a different location; otherwise, it loads 14 into d1 and calls another routine before branching to a different location.
- `frag_bsr_bcs_moveq_bra`: Short code fragment that calls another routine, then branches if the carry flag is set, then loads 1 into d1, then loads 1 into d4, then jumps to another point in the code.
- `frag_bsr_bcs_moveq_bra_2`: Short code fragment that calls another routine, then branches if the carry flag is set, then loads 2 into d1, then loads 3 into d4, then jumps to another point in the code.
- `frag_bsr_bcs_moveq_bra_3`: Short code fragment that calls another routine, then branches if the carry flag is set, then loads 4 into d1, then loads 7 into d4, then jumps to another point in the code.
- `calculate_buffer_offset_and_parameters`: Calls another routine to retrieve a value, adds it to a constant, and compares the result against twenty-two.
- `frag_bsr_bcs_moveq_bra_4`: Short code fragment that calls another routine, then branches if the carry flag is set, then loads 5 into d1, then loads 0 into d2, then jumps to another point in the code.
- `shift_and_copy_buffer_block`: Calls a routine to retrieve a masked value from a table, adds 544 to the result, and uses the sum as an offset to copy a block of bytes from a source buffer in a1 to a destination address pointed to by a1, decrementing a counter in d4 until the loop completes.
- `extract_and_shift_word`: Decrements a register and returns if it is not zero; otherwise, it loads a 16-bit word from a pointer in a register, performs a bitwise swap and a logical shift right on the word, and returns the result in a register.
- `tiny_lsr_rts`: Small routine of 2 instructions that shifts d6 right by 1, then returns.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `copy_and_patch_data_blocks`: Iterates through a list of counts to copy blocks of data from a source table to a destination, then processes a secondary list of structured data to perform additive updates to a global array using values and offsets read from the source.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d3 to d6, then returns.

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
- `modify_hardware_registers_2`: Performs several bitwise operations on memory locations at offsets 28, 462, 102, 102, and 110 of structures in a0, a2, and a6, and branches to another routine.
- `frag_add_move_bra_2`: Short code fragment that adds d4 to the memory at an offset from a5, then copies the memory at an offset from a5 to d5, then jumps to another point in the code.
- `calculate_index_offset_2`: Tests a byte in d4, and if it is non-zero, divides d3 by 3, swaps the high and low bytes of the result, and if the result is non-zero, performs bitwise operations on d4 and adds a byte from a structure in a5 to d4 before branching to another routine.
- `frag_subq_bne_lsr_add_bra_2`: Short code fragment that subtracts 1 from d0, then branches if the last result was not zero, then shifts d4 right by 4, then adds the memory at a5 to d4, then jumps to another point in the code.
- `fetch_table_value_2`: Calculates an index using a byte from a5, a byte from a structure in a4, and a byte from offset 10 of a5, then loads a word from a table pointed to by a1 into d5 and branches to another routine.
- `update_structure_values_3_2`: Moves a byte from d4 to d1, and if the lower nibble of d1 is non-zero, branches to another routine; otherwise it subtracts d4 from a byte at offset 2 of a structure in a5 and updates d0 and d1 based on the result.
- `clamp_and_store_byte_2`: Calculates a value by adding a shifted byte to another byte, clamps the result to a maximum of 64, and stores it at offset 2 of a structure pointed to by a5.
- `frag_move_cmp_bge_sub_bra_2`: Short code fragment that copies the memory at an offset from a5 to d5, then compares d3 with 3, then branches if the comparison was greater or equal, then subtracts d4 from d5, then jumps to another point in the code.
- `check_counter_overflow_2`: Loads a word from offset 30 of a structure in a5, and if it is not zero, compares a word from offset 18 of the structure to that word, adds a byte from offset 70 of the structure to the word from offset 18, and branches to another routine if the new value is less than the word from offset 30.
- `reset_counter_on_overflow_2`: Subtracts d4 from d1, compares d1 to d0, and if d1 is not higher than d0, loads a word from offset 30 of a structure in a5, clears that offset, and stores the value in d1 and d5.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `calculate_and_store_offset`: If d4 is not zero, it stores d4 at offset 33 of a5, performs a table lookup and arithmetic using offsets 32 and 18 of a5 to update a value, and updates offset 32 of a5 before branching to another routine.
- `frag_cmp_bne_bra_3`: Short code fragment that compares d4 with 241, then branches if the last result was not zero, then compares d3 with 3, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_cmp_bne_bra_4`: Short code fragment that compares d4 with 242, then branches if the last result was not zero, then compares d3 with 3, then branches if the last result was not zero, then jumps to another point in the code.
- `update_resource_structure_conditionally`: Checks if a byte in d4 equals 249; if so, it verifies if bit 1 or 2 of d3 is set, then loads bytes from offsets 0 and 1 of a structure in a5, pushes d3 to the stack, calls update_resource_structure using a pointer from offset 12 of a5, and restores d3 before returning.
- `clear_ciaa_bit_2`: Compares d4 to two different values and, if d4 equals 249, clears bit 1 of the CIAA_PRA register before branching to another routine.
- `frag_bset_bra_2`: Short code fragment that sets bit 1 of a fixed memory address, then jumps to another point in the code.
- `update_pointer_conditionally_alt`: Loads a pointer from offset 20 of a5 into a1, then if d5 is not zero, writes d5 to offset 6 of a1 and a byte from offset 2 of a5 to offset 9 of a1, increments d7, and calls another routine if d7 is less than a different constant.
- `lookup_or_divide_table_entry_3`: Checks if an input index is within a specific range to fetch a 16-bit value from a global table; otherwise, it divides a global constant by the input.
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

### Black Crypt (1992)(Electronic Arts)[cr TRSi][a](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_library_sequence`: Performs a sequence of library calls and cleanup callbacks, updating various values in a global structure based on the return values.
- `wait_for_library_call_3`: Calls a function pointer from a global structure repeatedly until a second function pointer from the same structure returns a non-zero value.
- `initialize_library_call_state_three_2`: Sets a global state variable to three and stores a parameter in a global variable.
- `process_data_and_call_subroutine`: Calls a function pointer, updates a value at an offset within the returned pointer, calls another function, updates global variables, calls a routine, and then calls the routine from R3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.

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
- `call_library_function_2`: Saves registers to the stack, calls a library function using a pointer from a4, and if the return value is not negative one, it branches to another routine, otherwise it sets the return value to zero and returns.
- `query_and_update_structure`: Calls a library function with a specific constant, then if the result is zero, calls another library function using a value from a register; otherwise, it populates a structure with a 32-bit word, a byte, and a constant value before calling a third library function.
- `update_structure_and_call_library`: Stores a 32-bit word from a3 into offset 10 of a2, stores a byte from the stack into offset 9 of a2, sets offset 8 of a2 to four, clears offset 14 of a2, and stores d2 into offset 15 of a2, then calls a library routine and subsequently calls another library routine based on the value of a3.
- `push_pointer_and_call`: Loads the address at offset 20 of a2 into a0, pushes a0 onto the stack, calls another routine, adjusts the stack pointer, moves a2 into d0, and branches to another routine.
- `initialize_library_context_2`: Checks a field in a structure, calls a library function if non-zero, and then sets certain structure fields to fixed values before calling two other library functions.
- `test_byte_and_branch`: Loads a byte from a pointer stored in a structure and branches to another routine if the byte is not zero; otherwise, it clears a register and returns.
- `write_byte_to_buffer_and_increment_table`: Increments a pointer in a2, compares a byte in d3 to thirty-seven, and if they are not equal, moves a value from d7 to d2.
- `scan_byte_stream_for_terminator`: Increments a counter in d2 and checks a byte at the pointer in a2; if the byte is not zero, it calls another routine, otherwise it returns the counter in d0.
- `increment_pointer_and_fetch_byte`: Increments a pointer in a2 by one, compares a byte in d3 to thirty-seven, and if they are not equal, moves a value from d2 to d7, clears d4, writes thirty-two to a location relative to a5, fetches a byte from the incremented pointer into d3, moves that byte into d0, and branches to another routine.
- `frag_bset_bra_3`: Short code fragment that sets bit 0 of d4, then jumps to another point in the code.
- `frag_bset_bra_4`: Short code fragment that sets bit 1 of d4, then jumps to another point in the code.
- `frag_bset_bra_5`: Short code fragment that sets bit 2 of d4, then jumps to another point in the code.
- `frag_bset_bra_6`: Short code fragment that sets bit 3 of d4, then jumps to another point in the code.
- `process_signed_bytecode_operand`: Reads a longword from a structure pointed to by a6, checks if it is negative, sets bit 0 of d4 if so, and then stores the absolute value at an offset of negative 8 from the structure pointer before fetching a byte from a stream in a2.
- `frag_move_moveq_bra`: Short code fragment that copies 48 to the memory at an offset from a5, then loads 0 into d2, then jumps to another point in the code.
- `parse_ascii_integer_loop`: Converts a sequence of ASCII digits from a stream into a decimal integer by multiplying the current value by ten and adding the next digit, then stores the result at an offset of negative eight from a structure.
- `dispatch_bytecode_opcode`: Decodes an opcode from register d0 by subtracting specific constants and branches to different routines based on the result.
- `parse_bytecode_operand_and_update_state`: Updates a value at offset negative 8 of a structure using a register, then checks a byte in a stream to determine if it should parse a numeric operand or a pointer, updating a status flag and a pointer based on the result.
- `frag_moveq_bra_3`: Short code fragment that loads 0 into d2, then jumps to another point in the code.
- `parse_bytecode_stream`: Reads a byte from a pointer in a2 and performs a series of comparisons to update a status flag in d4 and a pointer in a5.
- `frag_cmpi_bne_bset_bra`: Short code fragment that compares d3 with 108, then branches if the last result was not zero, then sets bit 6 of d4, then jumps to another point in the code.
- `update_pointer_and_opcode`: Checks if a byte in register d3 equals 76; if so, it sets bit 3 of register d4, increments the pointer in a2, stores the new pointer at offset 12 of the structure in a5, and moves the byte from a2 into d0 before branching to another routine.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `update_word_from_d7`: Tests bit 7 of d4 and, if set, loads a pointer from an offset of 4 from a6, then stores the 16-bit value of d7 into the memory location pointed to by the value at offset 0 of the new a6.
- `update_state_from_d7`: Tests bit 6 of d4 and, if set, loads a pointer from an offset of 4 from a6, then stores the value of d7 into the memory location pointed to by the value at offset 0 of the new a6.
- `frag_lea_movea_move_moveq_bra`: Short code fragment that loads the address of the memory at an offset from a6 into a6, then copies the memory at an offset from a6 to a0, then copies d7 to the memory at a0, then loads 0 into d2, then jumps to another point in the code.
- `limit_and_format_string`: Calculates the length of a string and compares it against a limit, then modifies the string by appending specific characters based on bits set in a control register.
- `store_byte_to_buffer`: Reads a byte from the memory location pointed to by the address in a6 plus four, stores it into a buffer at an offset of 520 bytes from a5, and branches to another routine.
- `frag_moveq_bra_4`: Short code fragment that loads 8 into d2, then jumps to another point in the code.
- `frag_ori_moveq_bra`: Short code fragment that merges 72 into d4, then loads 120 into d3, then loads 16 into d2, then jumps to another point in the code.
- `set_bit_and_load_table`: Sets bit 4 of a register, compares a byte in d3 to 88, and if it is not equal, loads the address of a table into d0 and branches.
- `store_table_address`: Loads the address of a table into d0, stores this address at an offset of 716 bytes from a5, tests bit 6 of d4, and if not set, updates a6 by adding 4 and loads a long value from the memory location pointed to by the new a6 into d6 before branching.
- `frag_btst_beq_lea_move_bra`: Short code fragment that tests bit 4 of d4, then branches if the last result was zero, then loads the address of the memory at an offset from a6 into a6, then copies the memory at an offset from a6 to d6, then jumps to another point in the code.
- `format_numeric_string_buffer`: Converts a value into a string by copying bytes from a table into a buffer, handles sign flags by prepending a minus or plus character, and pads the string with zeros or a specific character if the length is insufficient.
- `pad_numeric_string_buffer`: Checks if a byte in d3 is the character '8' or 'X' and, if a counter in d2 is non-zero and the buffer at a2 does not contain the character '0', pads the buffer backwards with '0' characters until a limit is reached.
- `compare_and_adjust_d5`: Tests bit 0 of d4 and compares a value at offset 4 minus 4 from a5 to 48; if these conditions are met and d2 plus 2 is less than a value at offset 8 minus 8 from a5, it sets d0 to d2 plus 2, sets d5 to the value at offset 8 minus 8 from a5 minus 2, and branches.
- `push_bytes_to_buffer`: Pushes a byte from d3 and a constant byte of 48 onto a stack pointed to by a2, increments a counter in d2, and loops if d2 is less than d5 or if a2 is greater than or equal to a calculated address.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `dispatch_opcode_commands`: Decodes an opcode in register d0 to perform various data manipulations, including updating a pointer with a 16-bit value from d7, calculating string lengths, and modifying a buffer by copying bytes from a memory location pointed to by a value in a6.
- `push_byte_forty_five`: Tests bit 4 of register d4; if set, it tests bit 5 of register d4, and if that bit is also set, it pushes the byte value 45 onto the stack pointed to by a2 and branches to another routine, otherwise it branches to another routine.
- `frag_btst_beq_move_bra`: Short code fragment that tests bit 1 of d4, then branches if the last result was zero, then copies 43 to the memory just below a2 (moving a2 down), then jumps to another point in the code.
- `frag_btst_beq_move_bra_2`: Short code fragment that tests bit 2 of d4, then branches if the last result was zero, then copies 32 to the memory just below a2 (moving a2 down), then jumps to another point in the code.
- `populate_node_data_buffer`: Checks specific bit flags in a register and a value at offset negative four of a structure, then copies a byte from a source pointer in a2 to a destination pointer stored in a3, incrementing the counter at a3, and finally decrements a counter at offset negative eight of the structure in a5.
- `process_node_data_and_decrement_counters`: Reads a byte from a pointer in a2, pushes it and a value from a3 onto the stack, and calls another routine.
- `copy_byte_from_a5_to_a0`: Compares the longword at offset 4 of a3 with the longword at a3, and if the comparison does not carry, it increments the longword at a3, copies a byte from offset -1 of a5 to the address in a0, sets d0 to zero, and branches to another routine.
- `process_structure_node_data`: Calls another routine to retrieve a value, then decrements a counter in a structure at offset negative eight from a5.
- `push_and_call_routine_2`: Loads a byte from the address in a2 into d0, pushes d0 and a3 onto the stack, calls a routine at a relative offset, adjusts the stack pointer, and then branches to another routine or performs a conditional branch based on whether d0 is equal to negative 1.
- `frag_btst_beq_move_bra_3`: Short code fragment that tests bit 0 of d4, then branches if the last result was zero, then copies d5 to d2, then jumps to another point in the code.
- `write_constant_to_a0`: Compares the longword at offset 4 of a3 with the longword at a3, and if the comparison does not carry, it increments the longword at a3, writes the byte 32 to the address in a0, sets d0 to 32, and branches to another routine.
- `call_and_decrement_value`: Pushes the contents of a5 onto the stack, calls a routine at a relative offset, adjusts the stack pointer, and if d0 is not negative 1, increments d7, loads a longword from an offset of 8 bytes below a5 into d0, decrements the longword at that same offset by 1, and branches based on a comparison between d0 and d2.
- `frag_moveq_bra_5`: Short code fragment that loads 255 into d0, then jumps to another point in the code.
- `get_string_length_2`: Iterates through a null-terminated byte string to determine its length.
- `initialize_pointer_structure`: The routine stores the address of the provided pointer into a memory location, increments that address by four, sets a subsequent word to zero, and stores the original address into another location.
- `iterate_node_list_2`: Iterates through a list of memory addresses by incrementing a pointer by 22 bytes at each step and calls another routine for each address until a limit is reached.
- `update_structure_node_state`: Validates a structure pointer and its flags, clears specific bits in the status word at offset 12, and updates node values or pointers based on the provided parameter and structure contents.
- `validate_and_execute_entry`: Pushes a2 to the stack, calls another routine, and if bit 14 of a word at offset 12 of a2 is not set, it performs a series of stack operations, extends a byte from offset 14 of a2 to a longword, calls another routine, and updates structure pointers.
- `handle_node_list_iteration`: Checks if a bitwise AND of d2 and a specific mask is non-zero, and if so, updates a structure in a2 by setting bit 6 of a byte at offset 12, incrementing a counter, and copying a byte from d3 to a pointer retrieved from the structure.
- `frag_move_moveq_bra_2`: Short code fragment that copies the memory at a2 to the memory at an offset from a2, then loads 0 into d0, then copies d3 to d0, then jumps to another point in the code.
- `process_structure_update`: If bit 14 of d2 is set and d5 is not zero, it pushes d5 and a value from offset 8 of a2 onto the stack, extends a byte from offset 14 of a2 to a longword, calls another routine, and then either clears bit 6 of a byte at offset 12 of a2 or updates structure pointers based on the value of d4.
- `update_node_structure`: Stores its own address into a global variable, sets bit 6 of a byte at offset 12 of the structure in a2, updates the structure's pointer and offset values, and finally copies the byte from d3 to a location pointed to by a structure member before branching.
- `update_structure_and_branch`: Sets bit 1 of a byte at offset 1 of a structure pointed to by a pointer at offset 12 of a2, copies a 32-bit value from offset 8 of a2 to offset 4 of a2 and then to offset 0 of a2, and branches to another routine.
- `close_buffered_stream`: Closes a buffered stream by flushing it if needed, releasing its buffer and related resources through other routines, and clearing the structure fields.
- `update_string_buffer_with_digits`: Converts a value into a string of digits and stores it in a buffer, then clears a linked list entry.
- `frag_moveq_bra_6`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `search_table_for_flag_match`: Iterates through a table of structures, checking if a specific bitmask is absent and if a certain field is non-zero, while recursively calling another routine to validate a condition.
- `compare_and_branch`: Adds 22 to a2, compares a2 with a 32-bit value at a negative offset from a4, and branches to another routine if the carry flag is set, otherwise it sets d0 to zero and branches to another routine.
- `frag_pea_move_jsr_addq_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then copies a2 to the stack, then calls another routine, then adds 8 to a7, then jumps to another point in the code.
- `negate_and_restore_registers`: Tests a 16-bit value in a register and branches if zero; otherwise, it negates a 32-bit value in a register, restores four 32-bit registers from the stack, tests the negated value, and returns.
- `negate_result_if_negative`: Calls a helper, then negates the value it returns when the first input is negative, and returns the adjusted value together with the second input.
- `call_and_return_concatenated_2`: Pushes d1 to the stack, calls another routine, moves the original d1 to d0, restores d1 from the stack, and returns.
- `frag_move_bsr_tst_rts`: Short code fragment that copies d1 to the stack, then calls another routine, then copies the top of the stack to d1, then tests d0, then returns.
- `swap_and_rotate_words_2`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtract.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `execute_table_entry`: Saves d2 and a2 to the stack, loads a pointer from offset 12 of the stack into a2, extracts a signed byte from offset 14 of the structure in a2 and extends it to a long, pushes that value onto the stack, calls another routine, and if the result in d0 is zero, branches to another location; otherwise it sets d2 to 33 and branches to another location.
- `update_structure_pointer`: Compares a2 to a calculated address from a4 and branches if equal; otherwise, it writes 1024 to offset 16 of a2, calls another routine with 1024, and stores the result in offset 8 of a2.
- `frag_movem_movea_suba_bra`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to a6, then subtracts a3 from a3, then copies the memory at an offset from a4 to a2, then jumps to another point in the code.
- `traverse_and_call_pointer`: Traverses a linked list of pointers and either stores a value from a pointer into a global variable or updates a global variable with that value before calling a function.
- `frag_move_beq_bra`: Short code fragment that copies a3 to d0, then branches if the last result was zero, then copies the memory at a2 to the memory at a3, then jumps to another point in the code.
- `store_and_call_library_2`: Stores a value from a pointer into a global variable and then calls a library function.
- `frag_movem_movea_bra`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a4 to a2, then jumps to another point in the code.
- `iterate_and_call_function_2`: It then clears a global status variable.
- `check_pointer_null`: Saves registers to the stack, loads a long value from an offset of the stack into d2, and if that value is non-zero, it restores the registers and returns, otherwise it sets d0 to zero and returns.
- `call_and_process_d0`: Saves a6 to the stack, loads a pointer from a negative offset of a4 into a6, sets d1 to zero, sets d0 to d2 plus eight, calls a routine at a negative offset of a6, restores a6, moves d0 to a2, and if d0 is not zero it branches to another routine; otherwise it moves zero to d0 and branches to another routine.
- `store_and_update_pointer`: Loads the current program counter into a0, stores a0 at a negative offset from a4, moves a value from a negative offset of a4 into the location pointed to by a2, stores d2 into offset 4 of a2, stores a2 at a negative offset from a4, and branches to another routine.
- `copy_string_to_buffer_3`: Copies bytes from a source pointer to a destination buffer until a null terminator is encountered.
- `validate_and_call_table_entry`: Checks if an index is within a specific range and points to a non-zero entry in a table, then calls the function pointer at that entry and returns its result; otherwise, it updates a global variable and returns negative one.
- `calculate_index_and_call_library`: Saves a6 on the stack, loads a pointer from a negative offset of a4 into a6, copies d2 to d0 and d1, calculates an index by shifting and adding, loads a value from a table using that index, calls a library routine, and branches based on the result in d0.
- `frag_moveq_bra_7`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `validate_and_execute_table_entry`: Validates an index against a range and checks a table entry; if valid, it calls a function pointer from a table using the index and returns the result, otherwise it updates a global status and returns negative one.
- `execute_file_operation`: Loads a function pointer from an offset of 28992 from a location pointed to by a4 into a6, calls that function with d1 containing the value from a2, and if the result is negative one, calls another function from the same pointer location before storing the result in a global variable.
- `call_offset_function`: Loads a pointer from an offset of 28992 from a location pointed to by a4 into a6, clears d2 and d3, loads a value from a2 into d1, and calls the routine at an offset of 66 from a6.
- `handle_file_error_2`: Calls a function using a pointer from a4, and if the result in d0 is zero, calls another function from the same pointer location and stores negative one in a global variable.
- `frag_moveq_bra_8`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `calculate_table_index`: Calls another routine, calculates an index by multiplying a value in d2 by six and adding an offset from a4, then checks if that index is within a specific range and if the value at the resulting address is zero; if all conditions are met, it sets a global variable to three and returns negative one, otherwise it returns.
- `check_bit_and_set_constant`: Loads a word from offset 4 of a2 into d0, performs a bitwise AND on d0 with three, and branches if non-zero; otherwise it stores six at a negative offset of a4, sets d0 to 255, and branches to another routine.
- `perform_conditional_write`: Checks bit 3 of offset 4 of a structure in a2, calls a function if the bit is set, then calls another function with d2 as a3 and d1 as the value from a2, and if the result is negative one, calls a third function from a pointer in a4 and stores the result in a global variable.
- `frag_move_bra`: Short code fragment that copies d3 to d0, then jumps to another point in the code.
- `check_status_bit`: Calls a function using a pointer from a4 with d1 set to 4096, and if bit 12 of the result is set and a global variable is zero, returns the result in d0.
- `frag_jsr_moveq_bra_3`: Short code fragment that calls another routine, then loads 0 into d0, then jumps to another point in the code.
- `write_four_bytes_to_output_2`: Writes four bytes from a global buffer to the standard output through the operating system, then flshes the output and calls another routine with the value one.
- `execute_cleanup_callbacks_3`: The routine iterates through a list of function pointers stored in a global structure, calling each one.
- `release_indexed_resources_and_flags`: Recursively walks a per-index loop over a count stored in the global block, invoking itself for each index, then performs a series of conditional teardown steps: it calls library function pointers when their fields are non-zero, stores a saved value into a pointer target, and finishes by invoking the close-out library call before returning the input parameter.
- `nop_padding_sequence_3`: The routine consists of two instructions that perform no operation and do not modify any state.
- `close_file_handle_2`: The routine validates an index against a table, checks if the corresponding entry is non-zero, and if valid, calls a function to close the resource before clearing the entry in the table.
