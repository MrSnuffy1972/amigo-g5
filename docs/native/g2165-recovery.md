# Game 2165 (black crypt (1992)(electronic arts)(disk 1 of 3)[cr conspiracy]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02165). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 27 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 444 of 444 named (0 author symbols), 444 of 444 described (31 by rule, 71 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 250 of 250 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14478 of 14478 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 654718 |
| code | 45874 |
| data | 190007 |
| padding | 6937 |
| unused | 3584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Stack | 2 | 117 | 2 |
| avail | 11 | 376 | 12 |
| runback | 3 | 709 | 39 |
| endcli | 2 | 117 | 2 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:InstallCrypt | 3 | 3235 | 57 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:BlackCrypt | 3 | 1745 | 48 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:crypt2 | 1 | 188 | 36 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bcdfp | 3 | 6150 | 184 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bcdfq | 2 | 983 | 56 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bcdft | 7 | 29 | 2 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- black crypt (1992)(electronic arts)(disk 1 of 3)[cr conspiracy]/Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:crypt2: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- black crypt (1992)(electronic arts)(disk 1 of 3)[cr conspiracy]/Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bcdft: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `disable_interrupts_and_relocate_code` (crypt2): Clears interrupt enables through the custom chip register and sets a CIA port value. Kind: self-relocating loader/decryption stub. Pass path: the code following the loop (the indirect jump to the relocated code).

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_to_buffer`: If the stack argument points to a buffer, copies sixteen longwords from a fixed offset in the data area into it.

### Stack

- `stack_command_entry_point`: Program entry for the Stack command, which scans a linked list of entries for the largest value.
- `copy_data_block`: Copies a block of sixteen words from a source address to a destination address.

### avail

- `avail_entry_parse_arguments`: Entry point of the Avail command.
- `finish_and_reply_message`: Stores its argument as the return value in a per-task record, closes a library if one is open, and if a reply message is pending it forbids multitasking and replies to that message.
- `sum_matching_block_sizes`: Walks a linked list of memory entries and adds up the size of each entry whose flag field shares a bit with the argument.
- `print_memory_availability_table`: Prints the memory availability table for chip, fast and total memory, or a usage line when the argument is not one of those.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_args_with_stack_slot`: Takes two arguments and calls another routine with them plus a pointer to a fixed slot on the stack, then returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `measure_stack_buffer_and_forward`: Calls the operating system formatting routine first, then measures the length of a 140-byte stack buffer up to its zero terminator.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### runback

- `program_entry_forwarder`: Program entry point consisting of a single instruction that transfers control to one other routine.
- `start_background_program_from_cli`: Parses the command line for an optional load delay in seconds and a program name with its parameters, then starts that program as a background task.
- `clear_table_and_store_pointers`: Clears a block of 40 longwords at an offset taken from a stack argument, then writes a pointer to a stack slot and a global value into the two fields after the block.
- `call_optional_handler_then_continue`: If a global function pointer is set, calls it.
- `initialise_background_run_state`: Allocates memory blocks through library calls and stores them in global fields, reporting an error code when the first allocation fails.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `run_loop_and_dispatch_globals`: Calls another routine ten times in a loop, then checks several global handler and state fields.
- `append_string_with_fixed_limit`: Finds the end of the destination string and appends the source string to it, copying at most 32767 bytes, then terminates the result with a zero byte.
- `append_string_with_count_limit`: Finds the end of the destination string and copies bytes from the source onto it until a zero byte is copied or the count in the third argument runs out, then terminates the destination.
- `copy_string_to_buffer`: Copies bytes from the source string into the destination buffer, including the terminating zero, and returns the destination pointer.
- `compute_string_length`: Walks a zero-terminated byte string to find its terminator and returns the number of bytes before it.
- `copy_bytes_with_padding`: Copies bytes from the source into the destination for a count taken from the third argument, stopping after a zero byte is copied, then fills the rest of that count with zero bytes.
- `release_table_slot_entry`: Checks that an index from 0 to 19 refers to an occupied table slot, and if not stores error code 3 and returns minus one.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_function_stub`: Loads a library base from a global variable and calls the library function at offset minus 102 from it, passing no arguments of its own.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_function_stub_2`: Loads a library base from a global variable and calls the library function at offset minus 84 from it, passing no arguments of its own.
- `call_library_function_stub_3`: Loads a library base from a global variable and calls the library function at offset minus 30 from it, passing no arguments of its own.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_function_stub_4`: Loads a library base from a global variable and calls the library function at offset minus 60 from it, passing no arguments of its own.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_function_stub_5`: Loads a library base from a global variable and calls the library function at offset minus 90 from it, passing no arguments of its own.
- `call_library_function_stub_6`: Loads a library base from a global variable and calls the library function at offset minus 48 from it, passing no arguments of its own.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_function_stub_7`: Loads a library base from a global variable and calls the library function at offset minus 198 from it, passing no arguments of its own.
- `execute_indirect_call_2`: Performs an indirect call to a function pointer retrieved from a memory location.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_function_stub_8`: Loads a library base from a global variable and calls the library function at offset minus 132 from it, passing no arguments of its own.
- `call_library_function_stub_9`: Loads a library base from a global variable and calls the library function at offset minus 210 from it, passing no arguments of its own.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### endcli

- `endcli_command_entry_point`: Program entry for the EndCLI command, which scans a linked list of entries for the largest value.
- `copy_multiple_words`: Copies a fixed number of words from a source pointer to a destination pointer if the destination pointer is not null.

### Port-Handler

- `parse_executable_hunk_records`: Program entry for a loader-type handler that initialises a state block and then walks a stream of executable hunk records.

### Disk-Validator

- `validate_disk_bitmap_blocks`: Program entry for the disk validation handler.

### Ram-Handler

- `copy_memory_then_dispatch`: Program entry for the RAM handler that copies memory through the operating system copy routine via a global library base, then makes a further indirect call to a target the decompiler could not recover.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:InstallCrypt

- `installer_entry_clear_state`: Program entry for the installer that clears a block of 80 longwords of its state and records the process pointer.
- `wait_for_installer_window_event`: Locks the current directory around a wait loop on the window message port.
- `open_install_window_and_buffers`: Clears several state fields, then opens the installer window and calls a routine with the value 10240, storing each result in a state field and calling a failure routine when a required result is zero.
- `draw_install_path_prompt`: Draws the installation dialog through a long fixed sequence of graphics calls for moving the pen, setting colour and drawing text.
- `show_install_failure_and_close`: Draws an install-failure message with the exit code when its code argument is non-zero, then releases the installer window and its resources.
- `copy_counted_name_with_colon`: Copies the name stored at an indexed table entry into the destination, using the length byte stored there and resetting the length when it exceeds 33.
- `check_entries_under_forbid`: Forbids multitasking, walks a linked list of entries and calls two routines for each entry with a non-zero field, setting the return value to 1 when a comparison fails.
- `prompt_insert_disk_and_wait`: Checks a path and, if it is not available, draws a prompt asking the user to insert the named disk and waits for a close or gadget event.
- `copy_file_with_progress_message`: Copies one file from a source name to a destination by opening both, reading and writing blocks in a loop, and closing both.
- `install_game_files_to_destination`: Prepares the destination directory, creating it if it is missing, and walks a table of files to copy.
- `forward_base_offset_pointer`: Calls another routine with a pointer into a fixed offset of the data base, the argument it received, and the address of its own stack argument, then returns that result.
- `copy_byte_array`: Copies a counted block of bytes from the source pointer to the destination, using a 32-bit count, and returns the destination pointer.
- `compare_byte_strings_ordered`: Compares two byte strings from their start and stops at the first differing byte.
- `empty_return_stub`: Does nothing and returns immediately.
- `init_control_block_with_streams`: Allocates a control block and initialises its fields, then writes a four-character tag into the message area.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `tokenise_command_line_arguments`: Splits a command line into words, treating spaces, tabs and line breaks as separators and keeping quoted text together.
- `append_string_with_limit`: Finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `append_string_with_limit_2`: Finds the null terminator of the first string and begins copying characters from the second string into that position.
- `read_window_tooltype_from_icon`: Opens the icon library and loads the icon for a named file.
- `append_string_unbounded`: Finds the end of the destination string and copies the source string onto it, including the terminating zero.
- `copy_string_to_null`: Locates the null terminator of the first string parameter by incrementing a pointer until a zero byte is found, then copies characters from the second string into the first.
- `copy_string_or_pad`: Copies characters from the source into the destination for up to the count given, stopping when the count runs out or the source ends.
- `multiply_32_bit_cross_terms`: Combines the 16-bit halves of two 32-bit register operands into a 64-bit return value.
- `format_string_into_stream`: Formats text into an output stream by copying literal characters and expanding percent conversions with flags, width, precision and size modifiers.
- `compute_string_length_2`: Walks a zero-terminated string to find its terminator and returns the count of bytes before it.
- `run_routine_over_table_entries`: Walks a table of 22-byte entries in the data area and calls another routine once for each entry until the end of the table.
- `write_byte_to_buffered_stream`: Writes one byte to a buffered stream structure after checking that the stream is open and not in an error state.
- `close_buffered_stream`: Closes a buffered stream by flushing it if needed, releasing its buffer and related resources through other routines, and clearing the structure fields.
- `close_buffered_stream_2`: Closes a buffered stream by flushing it if needed, releasing its buffer and related resources through other routines, and clearing the structure.
- `return_zero_stub`: Returns zero with no other work.
- `tiny_addq_addq`: Small routine of 2 instructions that adds 2 to a5, then adds 8 to d0.
- `flush_stream_or_all_entries`: When its argument is zero it walks the table of stream entries and flushes each open one, returning minus one if a flush fails and zero otherwise.
- `flush_open_stream_entries`: Walks the table of 22-byte stream entries and, for each one that is in use and not in an excluded state, calls a routine with the flush argument.
- `forward_call_stub`: Calls one other routine with no arguments and returns.
- `perform_signed_division`: Records the sign of each operand, calls the division routine, and negates its result when the operand signs differ.
- `negate_result_for_negative_input`: Calls another routine, then negates the value it leaves in a data register when the first input is negative.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value whose low word is the original second input and whose high word is the register value left by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: Divides a 32-bit dividend held in one register by a 32-bit divisor in another, returning the quotient and remainder packed into a 64-bit value.
- `init_stream_buffer`: Initialises a stream record's buffer pointers and flags.
- `unlink_and_free_node`: Searches a linked list for the given node, unlinks it when found and frees its memory through a library call.
- `free_all_list_nodes`: Walks the linked list and frees each node through a library call, then clears the list head and returns zero.
- `free_all_list_nodes_2`: Frees every node of a linked list through a library call, then clears the list head and returns zero.
- `allocate_linked_list_node`: Allocates memory for the requested size through a library call, links the new node at the front of the global list and stores the size in it.
- `copy_string_to_buffer_2`: Copies characters from the source string into the destination until the terminating zero is copied, then returns the destination pointer.
- `check_stream_is_interactive`: Checks that a stream index is valid and in use, setting error code 3 and returning minus one when it is not.
- `seek_stream_with_error_code`: Validates a stream index, then performs a seek on that stream through the operating system.
- `delete_file_with_error_code`: Deletes a file through the operating system and returns zero on success.
- `return_zero_constant`: This routine is empty and simply returns a value of zero to the caller.
- `write_stream_with_error_code`: Validates a stream index and that the stream is open, storing an error code and returning minus one if not.
- `poll_status_flag_and_return`: Reads a status word through a library call and returns it when the 4096 bit is set and a stored state flag is zero.
- `write_four_bytes_to_output`: Writes four bytes from a global buffer to the standard output through the operating system, then flushes the output and calls another routine with the value one.
- `drain_list_then_run_cleanup`: Repeatedly removes the head of a linked list and calls the cleanup routine stored in each node until the list is empty.
- `shutdown_installer_resources`: Closes each stream entry, releases a directory lock, and frees the window, screen and library resources held in global fields, restoring a saved value along the way.
- `return_void`: The routine is empty and performs no operations.
- `close_stream_entry_by_index`: Validates a stream index, storing error code 3 and returning minus one when it is invalid.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:BlackCrypt

- `game_entry_clear_state`: Program entry that clears a block of 67 longwords of state and records the process pointer.
- `run_game_disk_event_loop`: Locks the game disk volume and runs an event loop that switches on each input message type, updating state and calling handlers for each case until the loop ends.
- `initialize_graphics_and_config`: Clears global state fields, opens the screen and window libraries and then the main screen and window, and reads the configuration file.
- `close_screen_window_and_libraries`: Closes the screen and window if they are open, frees the allocated block and closes the libraries held in global fields.
- `delay_and_unload_segment`: Sets a message's state field to a finished value and replies to the message.
- `poll_input_event_loop`: Loops reading input events until one case sets the exit flag.
- `copy_counted_name_uppercase`: Copies a counted string of at most 33 characters into the destination, converting lowercase letters to uppercase.
- `check_entries_under_forbid_2`: Forbids multitasking, walks the linked list of entries and for each one with a non-zero field calls a comparison routine on its name, setting the return value to 1 when the result is zero.
- `prompt_insert_disk_and_wait_2`: Checks a path and, if it is not present, draws a prompt asking the user to insert the disk and waits for a gadget or close event before checking again.
- `wait_for_library_call`: Repeatedly calls a routine through a library base and then calls a second library routine, looping until the second returns a non-zero value.
- `start_game_process`: Creates a new process for the game named BC Game, loading its program segment from the game disk and sending it a start message.
- `reply_and_send_game_message`: Looks up a pending message, checks that its state matches the expected value, marks it as finished and replies to it.
- `handle_segment_load_unload`: Loads a program segment through the operating system and, when the load succeeds, calls the loaded code through a computed pointer using the segment value and a size derived from it.
- `print_zero_terminated_string`: Calls two routines to set up output, then calls another routine once for each byte of the zero-terminated string it is given.
- `nop_padding_sequence`: This routine is empty and contains no executable instructions.
- `call_routine_per_string_character`: Calls another routine with its argument, then calls a second routine once for each byte of the zero-terminated string, stopping at the terminator.
- `call_two_routines_in_sequence`: Calls two other routines one after the other with no arguments and returns.
- `clear_bitmap_after_setup`: Calls another routine, then clears a graphics block through a graphics library call that takes no arguments of its own.
- `draw_menu_labels`: Draws four menu labels by calling a text routine with fixed positions and text identifiers, then returns the value five.
- `wait_for_menu_item_click`: Waits for input messages and returns once a message of type 8 with code 104 falls inside the menu area.
- `dispatch_input_event_handler`: Clears a graphics bitmap, then dispatches on the event type code to one of several handlers for codes one through four and six.
- `store_register_values`: Stores the data register values it receives into three global variables, with no other work.
- `copy_tile_data`: Copies eight bytes per row from a source table into a destination area at fixed row strides, repeating for six rows.
- `scan_global_variable_array`: Walks a table in the data area in two passes, stepping through the entries two bytes at a time.
- `repeat_sixteen_library_passes`: Runs sixteen passes, each calling two library routines through the same base, one further routine and a third library routine.
- `decrement_nibble_values`: Walks a list of 16-bit words, with the count taken from the first register, and decrements each of the three 4-bit fields in every word that is non-zero.
- `execute_nibble_increment_loop`: Executes a loop sixteen times, calling indirect function pointers and a routine to increment nibble values in lists of words.
- `increment_nibble_values`: Walks two word lists in step, and for each word increments each of its three 4-bit fields in the first list wherever that field differs from the corresponding field in the second list.
- `copy_byte_array_2`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `compare_string_bytes`: Compares two byte strings character by character.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to its caller.
- `handle_workbench_disk_access`: Interacts with the Workbench interface, locks the game disk volume, parses strings, opens icon libraries, and processes input events and state changes.
- `parse_string_tokens`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `tokenize_string_into_list`: Parses a string by splitting it into individual tokens based on whitespace or quoted substrings.
- `find_string_end`: Finds the end of the first string by searching for a null terminator.
- `append_string_with_limit_3`: Finds the null terminator of the first string and begins copying characters from the second string onto that position.
- `initialize_window_data`: Opens a file and uses icon library functions to retrieve and store window information into a structure.
- `append_string_to_buffer`: The routine advances a pointer to the end of the first string, then copies characters from the second string into that location until the null terminator is copied.
- `copy_string_or_pad_2`: The routine copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `allocate_signal_and_port_block`: Allocates a signal bit and registers a message port through the operating system, storing both in a new control block whose fields are set from the arguments.
- `release_signal_and_port_block`: Removes the message port from its block when one is set, marks the block as unused and releases its signal and memory through library calls.
- `calculate_string_length`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `initialise_self_referencing_header`: Stores its own address plus four in the first longword, clears the second longword and stores its own address in the third longword.
- `copy_string_simple`: The routine copies characters from a source string to a destination buffer until the null terminator is copied.
- `execute_shutdown_callbacks`: Iterates through a list of indices to invoke helper routines and executes conditional function pointers stored in global memory.
- `shutdown_game_resources`: Calls another routine for each entry up to a count and frees a block when a flag is set.
- `null_operation_routine`: This routine is empty and performs no operations before returning.
- `close_resource_from_table`: Checks if an index is valid within a table and then calls a function to close a resource, setting the table entry to zero if successful.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:crypt2

- `disable_interrupts_and_relocate_code`: Clears interrupt enables through the custom chip register and sets a CIA port value.
- `decode_packed_stream_backwards`: Decodes a bit-packed compressed stream backwards.
- `extract_bitstream_bits`: Extracts individual bits from a source stream and shifts them into a byte.
- `call_unresolved_pointer_stub`: Calls through a pointer that the decompiler could not resolve, then returns.
- `dispatch_on_flag_register`: Stores a byte to a memory location, then calls one of two indirect targets depending on whether a data register is zero.
- `branch_to_unresolved_target`: Tests a flag byte and calls one of two indirect targets that the decompiler could not recover, then returns.
- `execute_indirect_call_3`: Performs an indirect call to an address retrieved from a memory location.
- `execute_indirect_call`: Performs an indirect call to an unrecovered address.
- `execute_indirect_pointer`: Loads a function address from a memory location and jumps to it.
- `execute_indirect_pointer_2`: Loads a function address from a memory location and jumps to it.
- `decompress_bitstream_and_expand_runs`: Extracts bits from a source stream to reconstruct bytes and expands run-length encoded data by repeating bytes based on count values.
- `dispatch_by_byte_value`: Compares a byte register with small constants and calls one of two unrecovered indirect targets depending on the result.
- `execute_indirect_call_6`: Performs an indirect call to a function address retrieved from a jump table.
- `execute_indirect_call_5`: Performs an indirect call to a function address retrieved from a jump table based on the value of a register.
- `execute_indirect_jump`: Loads an address from a memory location and jumps to it.
- `execute_indirect_jump_table_entry`: Performs an indirect call to a function address retrieved from a jump table.
- `dispatch_indirect_call`: Performs an indirect call to an address retrieved from a jump table.
- `execute_indirect_call_4`: Performs an indirect call to a function address based on the value of a byte.
- `dispatch_indirect_function`: Loads a function address from a memory location and jumps to it.
- `dispatch_conditional_jump_3`: Checks if a byte is less than three to determine which address to call from a jump table.
- `dispatch_conditional_jump_2`: Evaluates a byte and executes one of two different indirect calls depending on the comparison.
- `dispatch_conditional_jump`: Compares a byte value against a constant and performs an indirect call based on the result.
- `dispatch_jump_table_address`: Retrieves an address from a jump table and performs an indirect call or jump.
- `call_indirect_jump_table`: Performs an indirect call to an address retrieved from a jump table.
- `execute_indirect_jump_2`: Performs an indirect call to an address retrieved from a jump table.
- `copy_block_backwards_then_call`: Copies a block backwards from its end toward its start, one byte at a time, for a count held in a register, then calls an unrecovered indirect target.
- `execute_jump_table_entry`: Retrieves an address from a jump table and performs an indirect call.
- `execute_jump_table_entry_2`: Performs an indirect jump to an address retrieved from a table.
- `dispatch_indirect_call_2`: Retrieves an address from a jump table and performs an indirect call to that location.
- `decompress_data_stream_3`: Processes a compressed data stream by interpreting literal runs and repeated byte markers to reconstruct the original data.
- `expand_compressed_data_stream`: Processes a compressed data stream by interpreting literal runs and repeated byte markers to reconstruct the original data.
- `decompress_data_stream_2`: Expands a compressed data stream by processing literal runs and repeated byte markers.
- `decompress_data_stream`: Expands a compressed data stream by processing literal runs and repeated byte markers.
- `store_byte_then_call_unresolved`: Stores a byte register value just before an address register, then calls an unrecovered indirect target.
- `copy_literal_run_backwards`: Copies bytes backwards from the source toward the destination, expanding repeat markers into runs of the repeated byte.
- `decompress_literal_run`: Expands a compressed data stream by processing literal runs and repeated byte markers while writing to a destination.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bcdfp

- `game_entry_clear_state_2`: Program entry that clears a block of 494 longwords of state and records the process pointer.
- `open_graphics_intuition_libraries`: Opens the graphics and intuition libraries, reporting a failure message for each one that cannot be opened.
- `close_library_and_free_block`: Frees a memory block through a library call, passes a stored pointer to another routine, then makes two library close calls on that pointer.
- `copy_linked_entries_to_buffer`: Walks a linked list of 20-byte entries from a starting index, copying each entry into a buffer and advancing the buffer pointer.
- `initialize_entity_structures`: Initialises four entity records in turn, setting specific flag bytes, clearing their arrays and copying configuration values into them from the global table.
- `initialize_entity_arrays`: Fills several per-entity fields with index values derived from the loop counter and adds totals from a table to each entity.
- `serialize_character_data`: Writes the character save data to a file by copying header bytes and per-character blocks, including linked entries and attribute tables, into a buffer and writing it out.
- `wait_for_library_call_2`: Calls a function pointer from a global structure repeatedly until a second function pointer from the same structure returns a non-zero value.
- `send_request_with_state_zero`: Sets a request state field to zero, sends the request through the message port and runs a follow-up routine.
- `send_request_with_state_two`: Sets a request state field to two, sends the request through the message port and runs a follow-up routine.
- `send_request_with_state_eight`: Sets a request state field to eight, sends the request through the message port and runs a follow-up routine.
- `send_request_state_three_lock`: Sets a request state field to three, stores its argument and sends the request through the message port.
- `set_state_seven_call_library`: Stores seven in a global state word and saves the parameter in a second global word.
- `set_state_ten_call_library`: Stores ten in a global state word and saves two parameters in global words.
- `write_and_verify_dungeon_save`: Builds a save record in memory and writes it to a dungeon save file, referencing the original and temporary dungeon file names, then reads the file back and compares its length.
- `lock_game_disk_and_prompt`: Locks the game disk and loops while a disk status check reports a problem, showing the matching message text (disk is full, or disk is in use) for each status.
- `process_sprite_entries_until_done`: Loops over sprite entries, dispatching on a type value of two or eight and on a code value.
- `initialize_sprite_grid_entries`: Clears a global counter and runs setup helpers, then loops four times over six positions, calling a helper for each cell.
- `compute_nine_sprite_offsets`: Runs a counted loop nine times, calling the sprite offset helper once per iteration.
- `call_sprite_offset_calculator`: Calls a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `store_sprite_position_value`: Computes a value from a column variable and the entry index times nine, minus one, and stores it in a per-entry table.
- `configure_sprite_render_params`: Sets render parameters for the current sprite entry using the constants 149, 33 and 16 and a position derived from the entry index.
- `select_sprite_draw_routine`: Compares a column value against the current entry index, then switches on a difference computed from stored position values.
- `find_entry_covering_point`: Scans ten entries of a global table, each giving the origin of a fixed-size box, and checks whether the given x and y coordinates fall inside that box.
- `check_and_update_sprite_condition`: Checks values in a global table indexed by parameters, increments or decrements specific fields if conditions are met, and calls helper routines for further processing.
- `branch_helpers_on_tens_quotient`: Reads a value from a table indexed by the parameter and divides it by ten.
- `render_text_entry_string`: Looks up a text entry in a global table and calls helpers to compute its length and position.
- `clear_flagged_sprite_slot`: Looks up the slot for the current index and, when a helper returns a nonzero value, reads two status flags.
- `clear_slot_with_flag_check`: Reads status flags and clears either the entry before the given index or the entry at the current index.
- `clear_previous_entry_and_refresh`: Clears the entry one position before the given index, then calls helpers including one with the value one.
- `store_entry_value_clear_next`: Looks up a value for the entry at the current index.
- `find_index_in_short_table`: Scans a table of twenty-six words for the parameter value.
- `run_helper_then_library_call`: Calls a helper routine, then makes an indirect call through a library pointer in the global base and returns.
- `allocate_memory_block`: Calls a helper and then invokes the Exec memory allocation through a library pointer, without keeping the returned block.
- `initialize_graphics_library`: Allocates a control block, stores the custom chip base address in it, and opens graphics.library.
- `call_two_helpers_sequentially`: Calls two helper routines one after the other and returns.
- `run_library_list_sequence`: Calls one library function, then walks a list of two-word records calling another library function for each entry until a marker value is reached.
- `empty_stub_2`: This routine is empty and performs no operations.
- `free_blocks_and_close_library`: Walks two pointer tables and frees each non-null memory block with the Exec free routine.
- `initialize_custom_chip_state`: Copies two words from one state structure into another, clears all interrupt enables, and writes saved copper list pointers into the custom chip.
- `restore_custom_chip_registers`: Disables interrupts, writes saved copper list pointers and a saved interrupt word back to the custom chip, and enables DMA channels from a masked saved value.
- `wait_for_vblank_then_clear`: Polls the interrupt request word until the vertical blank bit is set, then clears that bit by writing it back to the interrupt request register.
- `wait_for_vblank_repeatedly`: Repeats the vertical blank wait and clear the number of times given in the input value.
- `repeat_clear_and_wait_flag`: Computes a repeat count from the input value and a structure field, then repeatedly clears a flag byte and waits for it to become nonzero, decrementing the count each time.
- `call_single_helper_routine`: Calls one helper routine and returns without doing anything else.
- `build_copper_list_commands`: Writes a sequence of copper list words into memory, including bitplane pointer pairs from global data, a color table copied from a global array, and a wait-and-end sequence.
- `copy_long_blocks_to_buffer`: Copies seventeen longwords from a fixed source block into a buffer, then copies seventeen more from a later point in the source into a second region of the buffer.
- `return_pointer_plus_offset`: Returns the address held in a global pointer, advanced by ninety-six bytes.
- `store_word_in_global_slot`: Writes the word passed in the data register into a fixed field of a global structure and returns.
- `fill_copper_entries_by_condition`: If the helper returns zero, it clears a flag and fills fourteen longwords with one fixed pattern.
- `copy_global_words_into_buffer`: Stores a helper result and a minus-one marker in a state structure, then copies six words from a global area into a buffer with eight-byte spacing.
- `update_frame_and_clamp_positions`: Toggles a frame parity byte and increments a frame counter.
- `indirect_call_unrecovered_target`: Makes an indirect call through a jump table that the decompiler could not recover, so the target is not known from this code alone.
- `acknowledge_masked_interrupt_flags`: Writes the masked interrupt request bits back to the interrupt clear register, then sets a flag byte to one in the state structure.
- `write_eight_after_helper_call`: Calls a helper, then writes the value eight to a fixed custom chip word and returns the data registers unchanged.
- `indirect_call_unrecovered_handler`: Makes an indirect call through a jump table that could not be recovered, so the handler it reaches is not known from this code alone.
- `advance_countdown_and_refill`: When an enable flag is set, decrements a countdown and, on reaching zero, toggles a state word.
- `reset_countdown_and_copy_words`: Sets a countdown to twenty-six and copies seven words from a global area into a buffer, then makes an indirect call through a jump table.
- `flush_pending_entries_to_buffer`: When the pending flag is set, copies twelve words into a buffer and, if the index is nonnegative, fills seven entries from a global table at that index.
- `initialize_input_state_counters`: Sets a buffer pointer one thousand twenty-four bytes past a base and clears several counters.
- `advance_queue_every_third_frame`: When an enable flag is set and the frame counter is a multiple of three, reads the next value from a pointer-based sequence and stores it in a structure field, using a default when the entry is zero.
- `store_helper_result_and_dispatch`: Stores a helper's two result bytes in the state structure, calls another helper, then makes an indirect call through a jump table.
- `process_input_region_changes`: Records two input bytes and, when a byte changes, scans a table of rectangular regions for the current position using a flag bit for each byte.
- `check_regions_for_input_change`: Scans a table of rectangular regions for the current position where the first flag bit is set and calls a helper on a match.
- `check_second_byte_regions`: When the second input byte has changed, scans the region table for its flag bit and calls a helper on a match.
- `compare_second_byte_and_dispatch`: Calls two helpers first, then if the second input byte has changed, scans the region table for its flag bit and calls a helper on a match.
- `scan_regions_then_store_input`: Walks the region table until a terminator, calling a helper for the first region whose flag bit is set and contains the current position.
- `store_input_state_and_dispatch`: Calls helpers, stores the returned input bytes into the state structure, calls another helper, and decrements a counter.
- `refresh_input_state_and_dispatch`: Calls a helper, stores its returned input bytes, calls another helper, and decrements a counter.
- `decode_keyboard_serial_byte`: Decodes a byte from the keyboard serial register when its data-ready flag is set, inverting and rotating it into a key code.
- `set_key_state_bit`: Sets one bit in a key state byte, selected by the low three bits of the input, and returns minus one.
- `read_key_state_byte`: Returns the byte stored at a fixed offset from a global pointer.
- `reset_fields_and_clear_block`: Sets several byte and word fields of a state structure to constants, then clears a block of 336 words that a pointer in the structure refers to.
- `scroll_buffer_column_and_update`: Advances a countdown and, when it passes below zero, copies eight byte rows into a buffer at an offset that wraps every forty-two bytes.
- `copy_byte_rows_on_countdown`: When the state byte is nonzero and its countdown falls below zero, copies eight byte rows into a buffer at an offset that wraps every forty-two bytes.
- `step_row_and_copy_table`: Increments a row index unless it is already eight, then copies seven words from a global table at that row into a target.
- `fetch_value_then_run_helper`: Calls a helper to fetch a value, runs a second helper, and returns the fetched value.
- `read_next_ring_buffer_entry`: Reads the next byte from an eighty-entry ring buffer, moving the read position backwards with wraparound.
- `clear_active_flag_and_return`: Clears a state flag byte and returns zero.
- `write_string_until_terminator`: Walks a null-terminated string, calling a helper for each character and setting a flag after each call.
- `write_byte_to_ring_buffer`: Stores the low byte of the input at the tail position of an eighty-entry ring buffer and moves the tail backwards with wraparound.
- `set_ring_tail_index`: Stores the input word as the ring buffer tail index and returns zero.
- `decrement_countdown_fields`: Decrements each nonzero countdown in a table of ten words, then makes an indirect call through a jump table.
- `initialize_object_state_defaults`: Sets coordinate, velocity and bound fields of a state structure to fixed constants and clears its position fields.
- `step_position_and_clamp_bounds`: If the state byte is zero, clears a set of counters.
- `clamp_position_then_dispatch`: Clamps the x and y values against stored minimum and maximum bounds, stores them as the current position, calls two helpers, and makes an indirect call through a jump table.
- `clamp_y_and_store_position`: Clamps the y value against its upper bound, stores the x and y values as the current position, calls two helpers, and makes an indirect call through a jump table.
- `add_velocity_and_clamp_position`: Adds stored velocity values to the current position, clamps the result between stored minimum and maximum bounds, stores it, calls two helpers, and makes an indirect call through a jump table.
- `clear_counter_blocks_and_dispatch`: Clears several longword fields in two blocks referenced by pointers in the structure, then makes an indirect call through a jump table.
- `compute_wrapped_byte_deltas`: Stores the new low and high bytes of a position and returns the two differences from the previous values as signed deltas that wrap around 256.
- `compute_mirrored_motion_deltas`: Computes two signed motion deltas, one from an input value reflected around 255 and one from the difference of two inputs, each folded into a signed byte range and sign-adjusted by a direction flag.
- `return_immediately`: The routine contains instructions but performs no data manipulation or hardware access before returning control to the caller.
- `record_history_and_store_values`: Stores the current position in a four-entry history table at the current index, steps that index backwards with wraparound, and writes either a helper's result or zeros into two pairs of words in a state structure.
- `update_secondary_counter_and_write`: Decrements a counter with a lower bound, calls a helper, and zeroes its results when a field is zero.
- `compute_input_status_word`: Adds two offset values to input coordinates, derives status bits from the sums and the low bit of the first value, and returns a combined value with bit seven set.
- `test_fire_button_status`: Returns minus one when bit six of a global status byte, which mirrors the fire button register, is clear, and zero otherwise.
- `capture_mouse_counters_safely`: Disables interrupts while it copies the input flags and the mouse movement counter into a state structure, then re-enables them if they were enabled before.
- `store_position_with_interrupts_off`: Disables interrupts, stores two input words as the current position, and re-enables interrupts if they were enabled before.
- `store_velocity_words`: Stores the two input words as velocity values in a state structure.
- `dequeue_pending_entry_with_wait`: Takes the oldest entry from a ring of 128 pending records, waiting while a count is nonzero and a flag is set.
- `reset_pending_queue`: Clears the queue counters and zeroes the two longwords of each of 128 queue records, toggling interrupt enables around the update.
- `set_state_word_to_one`: Sets a state word in the structure to one.
- `clear_state_word_to_zero`: Sets the same state word to zero.
- `mask_input_and_check_bit`: Masks the input word with a fixed mask and stores it in a state structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `enqueue_record_when_masked`: When an input word overlaps the stored mask and the queue is not full, it finds the free end of a record list and writes the two input words and the current position into it.
- `append_record_to_ring`: Stores a pointer to a buffer in a ring of 128 slots, advances the slot index with wraparound, and writes two input words followed by the current position into the buffer.
- `scale_value_by_word_ratio`: Multiplies the input by a stored word, divides the product by sixty, forces the result to at least one, and stores the low byte in a state field.
- `copy_linked_records_into_buffer`: Walks a linked list of records, copying each into a buffer with two computed values, one from the first field and an offset and one from the sum of two fields.
- `render_text_glyphs_with_blitter`: Clears a buffer, copies glyph rows for each character of the string from two global character tables into two buffers, then sets up blitter registers and starts a blit for each of six rows, waiting for the blitter between rows.
- `return_without_work`: Contains only a return and performs no work when called.
- `initialize_buffer_pointers_and_fields`: Points two buffer pointers at offsets inside the structure, clears several flag and counter fields, writes a fixed value to a custom chip register, and sets a counter to thirty.
- `swap_buffer_pointers_after_copy`: Copies six longwords from the current buffer into fixed fields of the state structure, toggles a buffer selector, and swaps the two buffer pointers to match.
- `chain_two_helper_calls`: Calls two helper routines in sequence and returns.
- `compute_row_stride_offset`: Multiplies the input word by 576, adds 96, and stores the result in a global offset, calling helpers before and after the store.
- `draw_entries_until_negative`: Walks a table of five-word entries, calling a helper for each one, until it reaches an entry whose first word is negative.
- `draw_sprite_stripe_lines`: Sets blitter masks and dimensions from the input record, then draws five horizontal stripe lines from a pointer table, waiting for the blitter between lines.
- `store_params_then_call_helpers`: Stores three input parameters in global variables and then calls two helper routines.
- `return_without_work_2`: This routine performs no operations and immediately returns to the caller.
- `draw_five_rows_then_callback`: Stores a record pointer, computes a width from its fields, and draws five rows into the output by copying from row tables at a computed start offset.
- `draw_sprite_segment_with_clipping`: Stores two input words in the record and draws five rows of blitter lines for a sprite segment, with source and destination offsets chosen by a flag bit.
- `draw_clipped_sprite_five_rows`: Clips a sprite's position and size against screen limits, computes a row stride and bit masks, and draws five rows through the blitter from the computed start offset.
- `write_nibble_codes_per_bit`: Loops over sixteen bit positions, waiting for two flag bytes on each pass and calling a helper, then writes a computed value for each bit into a buffer.
- `decrement_nibbles_in_words`: Walks a table of four-byte records and decrements each nonzero nibble of the word at offset two, processing the high, middle and low four-bit groups separately.
- `increment_nibbles_when_different`: Compares the word at offset two of each four-byte record with the matching source word and increments each nibble that differs.
- `zero_five_longword_blocks`: Walks an array of five pointers and zeroes two thousand longwords starting at each pointer.
- `service_four_audio_channels`: Walks four audio channel entries from the highest index down.
- `loop_audio_channel_entries`: Loops over audio channel entries, setting type-one entries to a table pointer and length with their DMA bit enabled and giving other entries a default length and pointer.
- `process_audio_channel_entries`: Sets a default length and pointer for each audio channel entry, then enables or disables the DMA bit of type-one and type-two entries until the channel index runs out.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `store_audio_entry`: Stores a constant value, a short value, and a pointer into a table based on an index.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `store_channel_sample_entry`: Sets a channel's type code to two, stores a word parameter in a per-channel table, and stores a pointer computed from the low bits of the first argument, scaled by eight, relative to a base table.
- `forward_call_with_stack_arguments`: Passes a fixed global buffer, the first argument and the caller's stack arguments to another routine and returns its result.
- `copy_bytes_into_buffer`: Copies a count of bytes one at a time from a source pointer to a destination pointer, then returns the destination.
- `return_without_effect`: The body is a bare return, so calling it changes nothing.
- `setup_io_control_block`: Allocates a control block, sets its fields and writes a four-character tag into it.
- `split_command_line_tokens`: Copies an argument string into a buffer, splits it into words separated by blanks with quoted text kept together, and builds an array of pointers to the words.
- `split_argument_string_words`: Copies an argument string into a buffer, splits it into blank-separated words with quoted text kept together, and builds a null-terminated array of pointers to the words.
- `append_limited_string`: Finds the end of the destination string and appends characters from the source until the source terminator or the given count is reached, then null-terminates the result.
- `copy_string_with_limit`: The routine finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `read_icon_window_tool_type`: Opens icon.library, retrieves an icon, and when its window tool type is present stores the value and a count in the given structure.
- `concatenate_strings`: Advances to the terminator of the destination string and copies the source string onto its end, returning the destination.
- `append_string_at_terminator`: Walks the destination to its null terminator and copies the source string there, including its terminator.
- `copy_string_with_padding`: Copies characters from a source string into a destination until the count is used up or the source terminator is copied, then pads the rest of the count with zero bytes.
- `allocate_signal_and_record`: Allocates a signal bit, then allocates a record holding the bit, a found task pointer and two parameter fields.
- `free_signal_record`: When the record's stored pointer is set, calls a library routine on it.
- `format_values_into_buffer`: Parses a format string with percent conversions for flags, width, precision and length modifiers, writing text, decimal, hexadecimal and string conversions into an output buffer.
- `measure_string_length`: Advances to the terminating zero byte of a string and returns the number of bytes before it.
- `initialize_self_referencing_block`: Sets the first word of a block to point just past itself, clears the next word, and stores the block's own address in the following word.
- `walk_record_table_with_helper`: Walks a fixed table of twenty-two-byte records from its start to its end, calling a helper with each record's address.
- `buffered_record_write_byte`: Writes one byte into a record's output buffer, flushing buffered data through a helper when the buffer is full or the byte is a newline.
- `close_buffered_record`: Closes a buffered record: flushes pending data when needed, closes its handle through a helper, and clears its counters and flags.
- `close_buffered_record_inline`: Performs the same close sequence as the main close routine, working directly on the record pointer held in a register.
- `return_zero_status`: Returns zero without doing anything else.
- `flush_one_or_all_records`: When given no record, flushes every open record in the table and returns minus one on the first failure, otherwise it flushes the given record and returns zero.
- `flush_buffered_records`: Iterates through a table of records to flush those lacking specific status flags, or flushes a single specified record.
- `flush_records_until_failure`: Walks the record table forward from its current position, flushing each open record that lacks the excluded status flags.
- `invoke_helper_and_return`: Calls one helper routine and returns without other work.
- `return_no_operations`: Contains only a return and performs no operations.
- `negate_result_if_negative`: Calls a helper, then negates the value it returns when the first input is negative, and returns the adjusted value together with the second input.
- `call_helper_return_register_pair`: Calls a helper and returns a pair of values: the second input register unchanged and the register the helper left behind.
- `call_one_routine_and_return`: Makes a single call to one routine and returns without other work.
- `divide_longs_with_remainder`: Divides a 32-bit unsigned value by another 32-bit value, using shift-and-subtract when the divisor exceeds 16 bits and word division otherwise.
- `allocate_node_buffer`: Looks up a value for the record, allocates a block of one thousand twenty-four bytes for it when needed, and sets the record's buffer pointers and flag bits.
- `unlink_and_free_node_2`: Searches a singly linked list for the given node, unlinks it when found, and frees its memory with the Exec free routine.
- `remove_list_node_and_free`: Walks a linked list to find the given node, unlinks it, and frees it through the Exec free routine.
- `unlink_known_node_and_free`: Unlinks a node from a list using its predecessor pointer, or the list head when there is no predecessor, and frees it through the Exec free routine.
- `pop_list_head_and_free`: Moves the list head to the next node, then frees the old head node with the Exec free routine.
- `free_all_list_nodes_3`: Walks the global node list, freeing each node through the Exec free routine, then clears the list head and returns zero.
- `free_node_chain_from_pointer`: Walks a chain of nodes starting from the given pointer, freeing each node through the Exec free routine, then clears the global list head and returns zero.
- `allocate_and_link_node`: Allocates a block of the requested size with the Exec allocator and links it at the front of a global list, returning the address past the link fields.
- `copy_string_to_destination`: Copies characters from a source string into a destination buffer through the null terminator, then returns the destination pointer.
- `check_file_handle_interactive`: Checks that a file slot index is in range and open.
- `seek_file_slot_position`: Validates a file slot index and seeks within it through the DOS routine.
- `delete_file_and_record_error`: Attempts to delete a file through DOS. On failure it records the DOS error code and returns minus one, otherwise it returns zero.
- `return_zero_only`: Returns zero and does nothing else.
- `write_to_file_slot`: Validates a file slot and checks that it is open for writing, storing error codes three or six when it is not.
- `check_status_bit_twelve`: Calls a library function and tests bit twelve of its result.
- `write_four_bytes_then_helper`: Writes four bytes from a global buffer through the DOS output routines, then calls a helper with the value one.
- `run_callback_list_then_cleanup`: Calls each function pointer in a global list in turn, unlinking entries as it goes, then calls a final cleanup routine when one is set.
- `release_all_resources`: Runs a validation helper for each index of a table and frees the table, then calls a stored callback.
- `return_no_effect_stub`: A bare return with no operations.
- `close_file_slot`: Validates a file slot and closes its DOS handle unless a flag bit is set, then clears the slot.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bcdfq

- `run_program_main_loop`: Opens the DOS, graphics and intuition libraries through Exec and copies startup values into globals.
- `check_fire_button_clear`: Returns true when bit six of a global hardware status byte is clear, which the register note links to the fire button input.
- `check_hardware_flag_clear`: Returns true when bit two of a byte in a fixed hardware register is clear.
- `wait_for_condition`: Polls two adjacent short values in a structure, returning immediately if the first is zero, otherwise waiting until the second becomes non-zero.
- `update_counters_at_frame_top`: Repeats 105 times: disables interrupts, waits for the top of the frame, decrements the counters in three table entries, writes a scaled value of each into a target, then re-enables interrupts.
- `open_file_resource`: Opens the file named by a text reference through DOS, stores the returned handle in a structure field, and returns true when the open failed.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `copy_block_words_from_offset`: Copies twelve thousand longwords from a block offset by twelve thousand longwords into the start of the buffer.
- `clear_and_copy_block`: Calls the graphics block-clear routine through a pointer, then copies eight thousand longwords from a block offset by twelve thousand longwords into the buffer's start.
- `read_table_entries_until_zero`: Walks a table of nonzero short entries, calling a library routine through a pointer for each one.
- `read_table_entries_until_flag`: Walks a table of nonzero short entries, calling DOS Read through a pointer for each one, and stops at the first zero entry or when a flag in the structure becomes nonzero.
- `loop_call_until_flag`: Iterates through a table of short values, calling a function pointer from a global structure for each non-zero entry.
- `call_dos_read_routine`: Calls the DOS Read routine through a pointer at a negative offset from a structure base and returns.
- `fade_palette_down_steps`: Runs sixteen steps, each waiting for two top-of-frame events, decrementing the color nibbles of a palette list and loading the palette.
- `decrement_palette_nibbles`: Walks a list of words for the given count, decrementing each nonzero four-bit component and writing each word back in place.
- `fade_palette_up_steps`: Runs sixteen steps, each waiting for two top-of-frame events, incrementing the color nibbles of a palette toward its target and loading the palette.
- `increment_palette_nibbles_toward_target`: Compares each word of a source list with the matching target word and increments each four-bit component that differs, writing the results back in place.
- `blit_five_blocks_per_row`: Takes ownership of the blitter, then for each of sixteen rows sets masks, moduli and pointers from global tables and starts five blits, waiting for each blit to finish before the next.
- `set_dma_channel_bit`: When the low six bits of the input are below four, stores the bit for that channel number in a global.
- `call_helper_sixteen_then_clear`: Calls a helper sixteen times, then sets a global variable to zero.
- `start_audio_sample_on_channel`: When a sample table entry for the index is present, marks the channel's DMA bit and clears its status bytes, then stores the sample pointer and length in the channel record.
- `store_audio_channel_fields`: Stores a pointer and length in an audio channel record, selecting either a fixed buffer with length one or the given length.
- `set_audio_block_pointer_fields`: Stores a length or requested value in the channel record and sets its pointer and two value fields from the inputs.
- `store_audio_block_pointers_only`: Stores the pointer, value and length fields into the channel record and the channel structure without any branching.
- `loop_forever_idle`: Spins in an endless loop with no work, so control never returns.
- `copy_pending_pointer_on_flag`: When bit zero of the input is set and a pending pointer field in the structure is nonzero, copies that pointer and a word into the destination named in the structure.
- `service_audio_channel_state`: When a channel-active word is nonzero, stores a helper's result in a global, calls three helpers, and copies a pending pointer and word when bit zero of the helper result is set.
- `call_four_unnamed_routines`: The decompiler produced no pseudo-code for this routine, so only its facts are known: 22 instructions, entered through a pointer stored in the hunk data, and it calls four unnamed routines.
- `call_six_unnamed_routines`: The decompiler produced no pseudo-code for this routine, so only its facts are known: 71 instructions, entered through a pointer stored in data and called by two unnamed routines, and it calls six unnamed routines.
- `call_two_unnamed_routines`: The decompiler produced no pseudo-code for this routine, so only its facts are known: 5 instructions, entered through a pointer stored in data and called by one unnamed routine, and it calls two unnamed routines.
- `call_three_unnamed_routines`: The decompiler produced no pseudo-code for this routine, so only its facts are known: 8 instructions, entered through a pointer stored in data and called by one unnamed routine, and it calls three unnamed routines.
- `call_two_routines_long_body`: The decompiler produced no pseudo-code for this routine, so only its facts are known: 13 instructions, entered through a pointer stored in data and called by one unnamed routine, and it calls two unnamed routines.
- `call_shared_routine`: A five-instruction routine that calls another routine.
- `call_single_routine_stub`: A two-instruction routine with one unnamed caller that makes one call to another unnamed routine.
- `call_two_routines_sequence`: A four-instruction routine reached from one caller that makes calls to two other routines.
- `coordinate_two_routine_calls`: A seventeen-instruction routine with one caller that makes calls to two other routines.
- `invoke_two_helper_routines`: A seven-instruction routine with one caller that calls two other routines.
- `invoke_two_helper_routines_2`: An eight-instruction routine with one caller that calls two other routines.
- `execute_helper_routines`: A nine-instruction routine that calls a shared routine which subsequently performs four additional calls.
- `call_shared_routine_short`: A seven-instruction routine that calls another routine.
- `shared_routine_with_four_calls`: A seventy-eight-instruction routine reached from twelve unnamed callers that makes four calls to other routines.
- `write_dmacon_and_call_helpers`: A forty-two-instruction routine with three callers that writes the DMA control register and makes two calls.
- `scan_records_then_dispatch`: Walks an array of pointers to records and reads one byte from each record while counting.
- `lookup_or_divide_table_entry`: Looks up a 16-bit value from a global table when the input index is below eleven, otherwise divides a global constant by the input.
- `store_indexed_byte`: Uses a value from a register to either fetch a word from a table or perform a division, then stores the resulting bytes into two separate memory locations.
- `lookup_or_divide_table_entry_2`: Checks if an input index is within bounds to read a 16-bit value from a global table; otherwise, it divides a global constant by the input.
- `tiny_ori_2`: Small routine of 1 instruction that merges 240 into d7.
- `store_global_and_return_offset`: Stores the address of a global variable into a slot at a fixed offset from a base register, then returns the value at the input pointer together with that pointer plus six.
- `add_to_pointer_if_nonzero`: If the value at the input pointer is non-zero, adds the value in data register D1 to it and stores the sum back.
- `adjust_pointer_offsets`: Makes several helper calls, then walks a chain of structures and, for each one whose marked field is negative, adds the structure's base address to each pointer in its following table.
- `scale_source_table_into_globals`: Reads a scale byte from a structure and multiplies sixteen source bytes by it, storing each shifted result into the global pointer table.
- `prepare_state_if_mmd0_found`: Calls a check routine and returns a fixed all-ones byte when it reports a non-zero result.
- `call_optional_handler_then_next`: Calls a helper routine only when a global flag is non-zero, then always calls a second helper routine.
- `clear_flag_and_call_handler`: If a global handler variable is set, clears the lowest bit of a byte through a pointer, clears the global, and calls the handler, returning its result.
- `open_ciab_resource_and_setup`: Opens the ciab.resource system resource through an exec library call and, if it is obtained, stores several register addresses into globals.
- `close_ciab_resource`: If a stored resource base is set, clears that global and calls a library function through the base, returning its result.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bcdft

- `initialize_and_free_memory`: Program entry point that runs two setup calls, calls a library function pointer in a counted loop, and then frees memory through the exec library.
- `decrement_until_minus_one`: Decrements a 16-bit counter held in a register until it reaches negative one, then returns.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
