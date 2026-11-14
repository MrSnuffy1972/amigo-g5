# Game 2165 (black crypt (1992)(electronic arts)(disk 1 of 3)[cr conspiracy]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02165). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 27 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 579 of 579 named (0 author symbols), 579 of 579 described (33 by rule, 131 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 250 of 250 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16312 of 16313 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 654718 |
| code | 51682 |
| data | 184190 |
| padding | 6946 |
| unused | 3584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 458 | 12 |
| Stack | 2 | 117 | 2 |
| avail | 11 | 376 | 12 |
| runback | 3 | 709 | 39 |
| endcli | 2 | 182 | 3 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:InstallCrypt | 3 | 3235 | 57 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:BlackCrypt | 3 | 2384 | 104 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:crypt2 | 1 | 188 | 36 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bcdfp | 3 | 6602 | 223 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bcdfq | 2 | 1120 | 70 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bcdft | 7 | 230 | 17 |
| Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- black crypt (1992)(electronic arts)(disk 1 of 3)[cr conspiracy]/Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:crypt2: Double Action 1.0 (xfd) stream found by ancient scan (9072 -> 13082 bytes) but only 0 of 5 sampled windows are in reference RAM: not used
- black crypt (1992)(electronic arts)(disk 1 of 3)[cr conspiracy]/Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bcdft: Imploder (xfd) stream found by ancient scan (85684 -> 180004 bytes) but only 0 of 86 sampled windows are in reference RAM: not used
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
- `initialize_entity_structure`: Initializes a structure by setting multiple offsets to zero, fetching values from tables based on specific indices, and performing a loop to copy a sequence of bytes from one memory location to another.
- `update_entity_state_from_table`: Fetches a byte from a table using a value at offset 0 of a structure, stores it at offset 32 of the structure, and calculates a new value at offset 36.
- `copy_byte_from_table_to_entity`: Calculates a destination offset in a structure using a value from offset 12 and a constant, then copies a byte from a table in a0 to that structure using a second calculated offset based on offset 24 and a value from offset 20.
- `update_entity_state_and_call_handler`: Updates several fields in a structure pointed to by a1 using values from a0, including setting a byte to 10 and copying longwords between offsets.
- `add_and_branch_to_routine`: Adds a longword from offset 54 of a structure in a1 to d1 and branches to another routine.
- `test_and_branch_on_structure_value`: Tests a longword at offset 48 of a structure in a1 and branches to another routine if the value is non-zero, otherwise it branches to a third routine.
- `modify_status_bits`: Performs a bitwise AND between a long value at offset 48 of a structure in a1 and a constant, and if the result is non-zero, sets bits in that same location; otherwise it branches to another location.
- `update_entity_pointer_and_call_handler`: Copies a long value from offset 64 to offset 108 of a structure in a1, prepares arguments in d1, d2, and d3, and calls a routine via a pointer in a5.
- `reset_entity_state_and_update_pointers`: Clears several fields in a structure pointed to by a1 and updates a corresponding entry in a table pointed to by a0 using an index derived from a1.
- `execute_complex_state_updates`: Calls several routines via a pointer in a6 with various parameters, updates a long value at offset 128 of a structure in a1, performs conditional decrements on values in a table pointed to by a0, and ends with an indirect jump through a6.

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
- `process_system_state`: Modifies a word in a structure, updates a pointer in a1, calls several functions via a pointer in a5, clears a value in a table, and conditionally decrements a value in a table before performing an indirect jump.

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
- `call_routine_and_branch`: Loads the value 1 into register d2, calls another routine, and then branches to another routine.
- `call_routine_with_parameter`: Calls another routine, pushes a word of four onto the stack, pushes a pointer to a location relative to the program counter onto the stack, calls another routine, cleans the stack, and branches to another location.
- `call_routine_with_parameter_2`: Calls another routine, pushes a word of five onto the stack, pushes a pointer to a location relative to the program counter onto the stack, calls another routine, cleans the stack, and branches to another location.
- `request_disk_and_shutdown`: Calls a routine to prompt for a disk insertion, stores the returned value into a structure at offset 24, sets a status field at offset 20 of that structure to 6, and then calls a library function before branching to a loop that handles event processing and system shutdown.
- `dispatch_input_and_update_event_status`: Calls dispatch_input_event_handler and stores the result in a structure at offset 22, then sets a status value of 6 at offset 20 of that same structure before calling an Amiga OS library function and branching to another routine.
- `execute_branching_calls`: Copies the word in d0 to d2, calls two different routines, and then branches to another location.
- `reset_and_call_subroutines`: Calls a subroutine, loads a pointer from an offset of a4 into a0, clears a longword at offset 82 of the structure in a0, pushes two parameters onto the stack and calls a subroutine, updates the longword at offset 82 of the structure in a0 with the value 33800, calls another subroutine, copies the word from d0 to d2, adjusts the stack pointer, and branches to another location.
- `process_event_loop_and_shutdown`: Enters a loop that checks a status value in a structure and executes different branches based on its value, including calling routines to unload segments, poll input, or handle disk status.
- `initialize_graphics_and_config`: Clears global state fields, opens the screen and window libraries and then the main screen and window, and reads the configuration file.
- `close_screen_window_and_libraries`: Closes the screen and window if they are open, frees the allocated block and closes the libraries held in global fields.
- `delay_and_unload_segment`: Sets a message's state field to a finished value and replies to the message.
- `poll_input_event_loop`: Loops reading input events until one case sets the exit flag.
- `process_input_and_state`: Calls a routine to handle an input event, then if the result in d2 meets specific criteria involving a global byte, shifts d2 left by 8, adds 16384, stores it in a global word, and calls a routine to update the global state with the value from d0.
- `clear_d2_and_branch`: Sets d2 to zero and branches to another location.
- `call_with_stack_setup`: Pushes a pointer to a global variable onto the stack, calls a routine, cleans up the stack, and branches to another location.
- `call_and_branch`: Calls a routine, sets d2 to zero, and branches to another location.
- `dispatch_jump_table`: Compares a word in d0 to 5, and if it is not less than 5, adds d0 to itself and uses the result to jump to an offset in a table.
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
- `copy_menu_tiles`: Calls a routine to process a string character and then calls a routine to print six zero-terminated strings using various word constants and pointers.
- `copy_menu_tiles_save`: Calls a routine to calculate length and copy tiles, then calls another routine to iterate and copy tiles multiple times using various word constants, and if a global byte is zero, performs one additional tile copy routine before returning.
- `copy_menu_tiles_save_2`: Calls a routine to print four zero-terminated strings using various word constants and pointers, then branches to another location.
- `copy_menu_tiles_load`: Calls a routine to calculate length and copy tiles, then calls another routine to iterate and copy tiles multiple times using various word constants, and if a global byte is not zero, performs additional tile copy routines using different word constants before returning.
- `populate_menu_text_tiles`: Calls a routine to copy tiles from four different string locations to a menu interface, using specific word constants as parameters for each string.
- `copy_text_tiles`: Calls a routine to process four zero-terminated strings using various word constants and pointers.
- `wait_for_menu_item_click`: Waits for input messages and returns once a message of type 8 with code 104 falls inside the menu area.
- `dispatch_input_event_handler`: Clears a graphics bitmap, then dispatches on the event type code to one of several handlers for codes one through four and six.
- `call_and_move_result`: Calls another routine and moves the resulting word from d0 to d2, then performs a branch to another location.
- `call_and_move_result_variant`: Calls another routine and moves the resulting word from d0 to d2, then performs a branch to another location.
- `transfer_result_and_branch`: Calls another routine, moves the returned word from register d0 to register d2, and performs an unconditional short branch.
- `transfer_result_to_register`: Calls a routine, moves the returned word from the accumulator into a second register, and performs an unconditional branch.
- `dispatch_input_event`: Calls a routine, saves the result in d2, and branches to another location; if the result is not less than 7, it uses the result to index into a table and jumps to a calculated address.
- `print_config_strings`: Sets up a stack frame, pushes 5 onto the stack, pushes a value from d0 to the stack, and calls a routine to process string characters.
- `copy_config_tiles`: Calls a routine to print four zero-terminated strings using various word constants and pointers.
- `build_keyboard_configuration_string`: Constructs a string by copying several text segments into a local buffer and then enters a loop that reads characters from an input stream.
- `display_intro_text_sequence`: Copies a string from a table to a buffer, calls a routine to display it, and then calls a routine to render tiles based on that string.
- `display_input_prompt_and_loop`: Copies a string from a table to a buffer, calls a routine to display it, and then calls a routine to iterate through tiles.
- `display_input_prompt_and_parse`: Copies a string to a buffer, displays it, and then iterates through a linked list of structures to validate and append input characters to a local buffer until a specific character or length limit is reached.
- `display_input_prompt_and_parse_2`: Copies strings to a buffer, displays them via a tile-based routine, and enters a loop that parses input characters to build a string.
- `display_prompt_and_parse_input`: Displays a string from a table and calls a routine to iterate through tiles, then enters a loop that parses input characters from a linked list of structures to validate them against specific character ranges and appends valid characters to a local buffer.
- `jump_via_table`: Compares d0 to 6, and if it is not less than 6, adds d0 to itself and jumps to an offset in a table.
- `parse_input_string_and_validate`: Checks two condition flags and then iterates through a linked list of structures to extract a byte from offset 15 of each structure, using the value to perform a bitwise shift and calling two library functions.
- `clear_d4_and_branch`: Sets d4 to zero and branches to another location.
- `validate_and_store_input_character`: Iterates through a buffer to check if a specific byte already exists, then checks if the byte is a vowel or within a specific range of ASCII values.
- `store_byte_and_call_libraries`: Stores a byte from register d2 into a buffer at an offset determined by d5, increments d5, and then calls two library functions using pointers retrieved from a global structure.
- `branch_to_address`: Performs a short branch to a specific address.
- `branch_to_address_2`: Performs a short branch to a specific address.
- `copy_config_tile_bytes`: If a specific word in d6 is zero, it performs a loop that copies 6 bytes from a source buffer located at an offset from a5 to a destination buffer located at an offset from a4.
- `copy_data_and_check_input`: Copies two longwords from a location offset from a4 to a location offset from a0, calls a routine using a pointer from a6, calls another routine, calls a third routine, and if d3 is zero, calls a fourth routine and branches to another location.
- `process_menu_selection`: Constructs a keyboard configuration string and then enters a loop that waits for a menu item click to perform various actions.
- `test_and_call_subroutine`: Tests a byte at an offset from a4, and if it is non-zero, pushes a pointer to a location relative to the program counter onto the stack, calls a routine, cleans the stack, and branches to another location.
- `print_string_and_branch`: Prints a zero-terminated string and then performs a short branch to a location ahead in the instruction stream.
- `manage_configuration_file_access`: Attempts to open a configuration file and, if successful, writes data to it using a specific error code; if the operation fails, it executes a sequence of error handling routines and updates a status value.
- `execute_error_handling_sequence`: Pushes a value onto the stack, then calls three consecutive routines before adjusting the stack pointer and branching to another routine.
- `handle_disk_io_error`: Attempts to read a file and, if the result is non-zero, calls three subroutines to process the error status and data before returning.
- `handle_disk_read_error`: Pushes a pointer to a string onto the stack, calls three subroutines to process data and disk status, and then returns via a short branch.
- `initialize_menu_state_and_branch`: Calls clear_bitmap_after_setup, sets register d3 to 1, and performs a short branch to a subsequent instruction stream.
- `branch_with_offset_lookup`: Compares a word in d0 to 6, and if it is not equal, adds d0 to itself and performs a jump to an address retrieved from a table using d0 as an index.
- `branch_to_offset_4928`: Performs a short branch to a location 4928 bytes ahead in the instruction stream.
- `copy_two_longwords`: If d3 is equal to 1, copies two longwords from a location offset from a5 to a location offset from a4.
- `check_pointer_validity`: Loads a pointer from offset 8 of a structure in a5, and if the pointer is not zero, moves it to a2 and branches to another routine; otherwise it returns.
- `iterate_string_and_call_routines`: Loads a pointer from a structure in a4, then enters a loop that calls a routine for the current character and another routine to calculate the string length, updating the pointer until a null byte is encountered.
- `process_event_sequence`: Checks a value at an offset of 34 of a structure pointed to by a4, and if it is 1 and a3 is not 0, calls call_routine_per_string_character, call_two_routines_in_sequence, wait_for_menu_item_click, and clear_bitmap_after_setup.
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
- `copy_string_to_null_2`: Locates the null terminator of the first string parameter by incrementing a pointer until a zero byte is found, then copies characters from a second string into that location.
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
- `process_modified_word`: Loads a word from an offset of a4, modifies it by adding 65535 and masking the lower two bits, pushes the result to the stack, calls three different routines, and then loops back to the start of this routine.
- `process_incremented_word`: Loads a word from an offset of a4, modifies it by adding 1 and masking the lower two bits, pushes the result to the stack, calls three different routines, and then loops back to the start of this routine.
- `call_and_branch_2`: Calls another routine and then performs a short branch to a different location.
- `process_sprite_entries_with_dispatch`: Calls a routine, then performs a series of checks on d0 to jump to different locations, and if d2 is non-zero, calls render_text_entry_string, swap_buffer_pointers_after_copy, and render_text_entry_string again before returning.
- `initialize_sprite_grid_entries`: Clears a global counter and runs setup helpers, then loops four times over six positions, calling a helper for each cell.
- `compute_nine_sprite_offsets`: Runs a counted loop nine times, calling the sprite offset helper once per iteration.
- `call_sprite_offset_calculator`: Calls a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `store_sprite_position_value`: Computes a value from a column variable and the entry index times nine, minus one, and stores it in a per-entry table.
- `configure_sprite_render_params`: Sets render parameters for the current sprite entry using the constants 149, 33 and 16 and a position derived from the entry index.
- `select_sprite_draw_routine`: Compares a column value against the current entry index, then switches on a difference computed from stored position values.
- `clear_stack_and_call`: Clears a word from the stack, calls another routine, and then performs a short branch to an earlier location.
- `clear_stack_and_call_2`: Clears a word from the stack, calls another routine, and then performs a short branch to an earlier location.
- `push_word_and_call`: Pushes a word onto the stack, calls another routine, and then performs a short branch to a different location.
- `push_word_and_call_2`: Pushes a word onto the stack, calls another routine, and then performs a short branch to a different location.
- `push_call_and_branch`: Pushes the constant two onto the stack, calls a routine, cleans the stack, and performs an unconditional short branch.
- `call_sprite_logic_two`: Pushes 2 onto the stack, calls a routine, adjusts the stack pointer, and branches to another location.
- `call_sprite_logic_three`: Pushes 3 onto the stack, calls a routine, adjusts the stack pointer, and branches to another location.
- `call_sprite_logic_three_alt`: Pushes 3 onto the stack, calls a routine, adjusts the stack pointer, and branches to another location.
- `call_sprite_logic_four`: Pushes 4 onto the stack, calls a routine, adjusts the stack pointer, and branches to another location.
- `dispatch_sprite_logic`: Calls check_and_update_sprite_condition, then uses a jump table based on d0 to call either store_word_in_global_slot, swap_buffer_pointers_after_copy, or return_without_work_2.
- `find_entry_covering_point`: Scans ten entries of a global table, each giving the origin of a fixed-size box, and checks whether the given x and y coordinates fall inside that box.
- `check_and_update_sprite_condition`: Checks values in a global table indexed by parameters, increments or decrements specific fields if conditions are met, and calls helper routines for further processing.
- `update_sprite_conditions`: Calculates offsets into a table, compares values, and if they differ, decrements one value and increments another in a table before calling branch_helpers_on_tens_quotient twice.
- `branch_helpers_on_tens_quotient`: Reads a value from a table indexed by the parameter and divides it by ten.
- `render_text_entry_string`: Looks up a text entry in a global table and calls helpers to compute its length and position.
- `clear_flagged_sprite_slot`: Looks up the slot for the current index and, when a helper returns a nonzero value, reads two status flags.
- `clear_slot_with_flag_check`: Reads status flags and clears either the entry before the given index or the entry at the current index.
- `clear_previous_entry_and_refresh`: Clears the entry one position before the given index, then calls helpers including one with the value one.
- `store_entry_value_clear_next`: Looks up a value for the entry at the current index.
- `find_index_in_short_table`: Scans a table of twenty-six words for the parameter value.
- `calculate_string_offset`: Calculates a memory address based on a value from a global variable and d2, calls a routine to find a string length, and then returns.
- `update_text_display_buffer`: Calculates an offset using a value from a global variable and d2, then writes a space to that location; it then calculates a second offset using d2 plus one and clears a byte at that location; finally, it calls three routines and branches back to an earlier instruction.
- `allocate_memory_and_copy`: Copies a byte from a source pointed to by a0 to a destination pointed to by a2, then calls a routine using a pointer from a system library.
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
- `process_data_tables`: Clears a memory block using a loop, calculates a length based on a null-terminated string, copies seven bytes from a table to a destination using a loop, and sets a word at offset 974 of a structure in a5 to 2400.
- `copy_data_and_calculate`: Copies eight words from a source table to a destination table using a loop, sets a word at offset 974 of a structure in a5 to 2400, and stores a product of two values into offset 1084 of that structure.
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
- `negate_and_store_values`: Negates two 16-bit values in d0 and d1, subtracts 12 from each, and stores the results into a table at offsets 162 and 164 from a base address.
- `store_registers_to_table`: Stores the values of d0, d1, a0, and the stack pointer into a table at offsets 168, 170, 172, and 174 from a base address.
- `compare_d0_value`: Compares the word in d0 against the values 1, 2, 3, or 4 and branches to different locations, otherwise it sets d0 to 65535 and returns.
- `get_byte_from_offset_c4`: Returns a byte from offset 196 of a structure in a0.
- `get_byte_from_offset_c5`: Returns a byte from offset 197 of a structure in a0.
- `get_word_from_offset_82`: Returns a word from offset 130 of a structure in a0.
- `get_word_from_offset_84`: Returns a word from offset 132 of a structure in a0.
- `update_scaled_value`: Stores a 16-bit word from d0 into offset 178 of a structure, calculates a new value by multiplying d0 by 46 and dividing by 100 into offset 176 of the same structure, and modifies the interrupt enable register based on a bit test of a value from the stack.
- `update_structure_values_2`: Calls another routine, checks a byte at offset 200 of a structure in a5, and if it is not zero, writes a word, a byte, and two words to memory locations pointed to by a0 before decrementing a byte at offset 200 of the structure in a5.
- `transfer_list_node`: Sets a word at offset 154 of a structure in a6 to 16384, checks a word at offset 190 of a structure in a5, and if it is zero, clears two longwords at a0 and returns.
- `transfer_and_clear_list_node`: Decrements a 16-bit value at offset 190 of a structure, traverses a table using a value from offset 188 and offset 182, and if a comparison with 128 is met, copies a 32-bit doubleword from a table to a destination and clears the source entries, otherwise it returns.
- `dequeue_pending_entry_with_wait`: Takes the oldest entry from a ring of 128 pending records, waiting while a count is nonzero and a flag is set.
- `reset_pending_queue`: Clears the queue counters and zeroes the two longwords of each of 128 queue records, toggling interrupt enables around the update.
- `set_state_word_to_one`: Sets a state word in the structure to one.
- `clear_state_word_to_zero`: Sets the same state word to zero.
- `mask_input_and_check_bit`: Masks the input word with a fixed mask and stores it in a state structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `clear_bits_in_structure`: Clears specific bits in a 16-bit word at offset 194 of a structure by performing a bitwise AND with the inverse of a masked version of d0.
- `set_bits_in_structure`: Sets specific bits in a 16-bit word at offset 194 of a structure by performing a bitwise OR with a masked version of d0.
- `enqueue_record_when_masked`: When an input word overlaps the stored mask and the queue is not full, it finds the free end of a record list and writes the two input words and the current position into it.
- `append_record_to_ring`: Stores a pointer to a buffer in a ring of 128 slots, advances the slot index with wraparound, and writes two input words followed by the current position into the buffer.
- `scale_value_by_word_ratio`: Multiplies the input by a stored word, divides the product by sixty, forces the result to at least one, and stores the low byte in a state field.
- `copy_byte_from_offset`: Copies a byte from offset 201 of a structure pointed to by a pointer at offset 5582 of the program to offset 200 of that same structure.
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
- `call_drawing_routines`: Calls return_without_work and draw_sprite_stripe_lines after saving and restoring registers a5 and a6.
- `loop_through_data_and_call`: Pushes a2 to the stack, enters a loop that loads a word from a structure, calculates an address, and calls a routine via bsr.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `return_without_work_2`: This routine performs no operations and immediately returns to the caller.
- `draw_five_rows_then_callback`: Stores a record pointer, computes a width from its fields, and draws five rows into the output by copying from row tables at a computed start offset.
- `draw_sprite_segment_with_clipping`: Stores two input words in the record and draws five rows of blitter lines for a sprite segment, with source and destination offsets chosen by a flag bit.
- `branch_to_offset`: Loads a pointer from offset 992 of a structure in a5 into a0 and branches to another routine.
- `process_and_dispatch_list`: Calculates several values based on a structure, populates a local structure in a6, and enters a loop that updates a6 entries using data from a list before performing an indirect jump to a routine stored at an offset in a structure.
- `populate_and_loop_structure`: Populates a local structure in a6 with values from a structure and a list, then enters a loop that updates a6 entries using data from the list until a counter reaches zero.
- `wait_for_bit_clear`: Loops while bit 0 of a 16-bit word at offset 4 of the structure in a6 is set, then compares a 16-bit word at offset 6 of a6 with d0 after shifting it right by 8 bits.
- `rotate_byte_buffer`: Calculates a sum of three bytes from a table using carry propagation, stores the result in the first byte of the table, and then shifts five bytes from the table to the five preceding bytes in memory.
- `write_nibble_codes_per_bit`: Loops over sixteen bit positions, waiting for two flag bytes on each pass and calling a helper, then writes a computed value for each bit into a buffer.
- `decrement_nibbles_in_words`: Walks a table of four-byte records and decrements each nonzero nibble of the word at offset two, processing the high, middle and low four-bit groups separately.
- `increment_nibbles_when_different`: Compares the word at offset two of each four-byte record with the matching source word and increments each nibble that differs.
- `zero_five_longword_blocks`: Walks an array of five pointers and zeroes two thousand longwords starting at each pointer.
- `service_four_audio_channels`: Walks four audio channel entries from the highest index down.
- `loop_audio_channel_entries`: Loops over audio channel entries, setting type-one entries to a table pointer and length with their DMA bit enabled and giving other entries a default length and pointer.
- `process_audio_channel_entries`: Sets a default length and pointer for each audio channel entry, then enables or disables the DMA bit of type-one and type-two entries until the channel index runs out.
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
- `modify_hardware_registers`: Performs several bitwise operations on memory locations at offsets 28, 462, 102, 102, and 110 of structures in a0, a2, and a6, and branches to another routine.
- `add_to_structure_offset`: Adds a word in d4 to a word at offset 18 of a structure in a5, then branches to another routine.
- `calculate_index_offset`: Tests a byte in d4, and if it is non-zero, divides d3 by 3, swaps the high and low bytes of the result, and if the result is non-zero, performs bitwise operations on d4 and adds a byte from a structure in a5 to d4 before branching to another routine.
- `decrement_and_branch`: Decrements d0 by 1, branches if not zero, otherwise shifts d4 right by 4, adds a byte from a5 to d4, and branches to another routine.
- `fetch_table_value`: Calculates an index using a byte from a5, a byte from a structure in a4, and a byte from offset 10 of a5, then loads a word from a table pointed to by a1 into d5 and branches to another routine.
- `update_structure_values_3`: Moves a byte from d4 to d1, and if the lower nibble of d1 is non-zero, branches to another routine; otherwise it subtracts d4 from a byte at offset 2 of a structure in a5 and updates d0 and d1 based on the result.
- `clamp_and_store_byte`: Calculates a value by adding a shifted byte to another byte, clamps the result to a maximum of 64, and stores it at offset 2 of a structure pointed to by a5.
- `compare_and_subtract`: Moves a word from offset 18 of a structure in a5 to d5, compares d3 to 3, branches if d3 is greater than or equal to 3, otherwise subtracts d4 from d5 and branches to another routine.
- `check_counter_overflow`: Loads a word from offset 30 of a structure in a5, and if it is not zero, compares a word from offset 18 of the structure to that word, adds a byte from offset 70 of the structure to the word from offset 18, and branches to another routine if the new value is less than the word from offset 30.
- `reset_counter_on_overflow`: Subtracts d4 from d1, compares d1 to d0, and if d1 is not higher than d0, loads a word from offset 30 of a structure in a5, clears that offset, and stores the value in d1 and d5.
- `update_pointer_conditionally_2`: Loads a pointer from offset 20 of a5 into a1, then if d5 is not zero, writes d5 to offset 6 of a1 and a byte from offset 2 of a5 to offset 9 of a1.
- `update_structure_values`: If d4 is not zero, it stores d4 at offset 33 of a5, calculates a value using a table lookup and arithmetic based on offsets 32 and 18 of a5, and updates offset 32 of a5 before branching to another routine.
- `compare_status_bytes`: Compares a byte in d4 to 241; if it matches, it compares a byte in d3 to 3, otherwise it branches to another location.
- `compare_multiple_values`: Compares d4 to 242, branches if not equal, then compares d3 to 3 and branches if not equal, otherwise branches to another routine.
- `execute_dma_update`: Compares d4 to 243, and if it matches, checks if bits 1 or 2 are set in d3, then loads several bytes from a structure in a5 and calls another routine via a branch to subroutine using a pointer from offset 12 of a5.
- `clear_ciaa_bit`: Compares d4 to two different values and, if d4 equals 249, clears bit 1 of the CIAA_PRA register before branching to another routine.
- `set_ciaa_bit`: Sets bit 1 of the CIAA_PRA register and branches to another routine.
- `update_pointer_conditionally`: Loads a pointer from offset 20 of a5 into a1, then if d5 is not zero, writes d5 to offset 6 of a1 and a byte from offset 2 of a5 to offset 9 of a1, increments d7, and calls another routine if d7 is less than a constant.
- `lookup_or_divide_table_entry_2`: Checks if an input index is within bounds to read a 16-bit value from a global table; otherwise, it divides a global constant by the input.
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
- `execute_multiple_calls`: Calls a routine via a pointer in a4 multiple times, updates a value in d5, and performs a loop that calls another routine and potentially adds values to a table until d5 reaches 3.
- `accumulate_values_in_loop`: Performs a loop where it calls another routine, potentially calls a second routine, and adds a value from d3 to a table at an offset determined by d2 and a3.
- `loop_call_routine`: If a zero flag is set, it branches to another routine, otherwise it moves a3 to a1, calls another routine, and branches to another routine.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes`: If a comparison between a6 and a2 is not equal, it copies four bytes from the source pointed to by a2 to the destination pointed to by a1 in a loop, then loads a longword from a1 into d0 and returns.
- `decompress_variable_length_bitstream_2`: Reads a control byte and a bit count from a table to determine how many bits to process, then performs a series of left shifts and conditional additions using bytes from a source buffer to reconstruct a value.
- `process_bitstream_loop`: Sets a counter to 2, adds the byte in d1 to itself, and if the result is not zero, enters a loop that adds a byte from a buffer in a0 to d1 and d0 using extended addition, decrementing the counter until it reaches zero, then increments d0 by 6 and branches to another location.
- `load_constants_and_branch`: Loads the value 5 into register d0, the value 3 into register d7, and branches to another routine.
- `load_constants_and_branch_2`: Loads the value 4 into register d0, the value 2 into register d7, and branches to another routine.
- `load_constants_and_branch_3`: Loads the value 3 into register d0, the value 1 into register d7, and branches to another routine.
- `decode_variable_length_bitstream`: Reads a variable-length bitstream from a buffer in a0, using a series of conditional shifts and additions to reconstruct a value.
- `add_and_branch`: Adds a byte in d1 to itself and branches if non-zero, otherwise it loads a byte from a0, adds it to d1 with carry, adds d2 to itself with carry, and branches to the start of this routine.
- `decompress_bitstream_data`: Processes a variable-length bitstream to reconstruct data by reading bits from a source buffer and using them to index into a table for dictionary-based expansion.
- `decompress_variable_length_bitstream`: Reads a bitstream from a source buffer, using a control byte to determine the number of bits to process and the length of subsequent data segments.
- `push_data_and_branch`: Pushes a byte from d0, a byte from d1, and a word from d2 onto the stack at the location pointed to by a4, restores several registers from the stack, and branches to another routine.

### Black Crypt (1992)(Electronic Arts)(Disk 1 of 3)[cr Conspiracy].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
